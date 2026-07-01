#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_173(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49750_e82891, assign49750_e82891_d_n3, assign49750_e82891_d_n4, assign49750_e82891_d_n5, assign49750_e82891_d_n6, assign49750_e82891_d_n7, assign49750_e82891_d_n8, assign49750_e82891_d_n9, assign49750_e82891_d_n10, assign49750_e82891_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49750_e82883: f64 = (-locals.var_vbd_jct);
        let assign49750_e82885: f64 = (assign49750_e82883 / locals.var_nvtm2);
        let assign49750_e82887: f64 = (assign49750_e82885 * locals.var_vtun0d_i);
        let assign49750_e82889: f64 = (assign49750_e82887 * locals.var_t1);
        (assign49750_e82889, (assign49750_e82887 * locals.var_t1_dn3), ((((-((assign49750_e82883 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn4)), ((((-((assign49750_e82883 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn6)), (assign49750_e82887 * locals.var_t1_dn7), (assign49750_e82887 * locals.var_t1_dn8), (assign49750_e82887 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn10)), (assign49750_e82887 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49750_e82891;
        locals.var_t0_dn3 = assign49750_e82891_d_n3;
        locals.var_t0_dn4 = assign49750_e82891_d_n4;
        locals.var_t0_dn5 = assign49750_e82891_d_n5;
        locals.var_t0_dn6 = assign49750_e82891_d_n6;
        locals.var_t0_dn7 = assign49750_e82891_d_n7;
        locals.var_t0_dn8 = assign49750_e82891_d_n8;
        locals.var_t0_dn9 = assign49750_e82891_d_n9;
        locals.var_t0_dn10 = assign49750_e82891_d_n10;
        locals.var_t0_dn11 = assign49750_e82891_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49760_e82903, assign49760_e82903_d_n3, assign49760_e82903_d_n4, assign49760_e82903_d_n5, assign49760_e82903_d_n6, assign49760_e82903_d_n7, assign49760_e82903_d_n8, assign49760_e82903_d_n9, assign49760_e82903_d_n10, assign49760_e82903_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49760_e82901: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49760_e82901, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49760_e82903;
        locals.var_t1_dn3 = assign49760_e82903_d_n3;
        locals.var_t1_dn4 = assign49760_e82903_d_n4;
        locals.var_t1_dn5 = assign49760_e82903_d_n5;
        locals.var_t1_dn6 = assign49760_e82903_d_n6;
        locals.var_t1_dn7 = assign49760_e82903_d_n7;
        locals.var_t1_dn8 = assign49760_e82903_d_n8;
        locals.var_t1_dn9 = assign49760_e82903_d_n9;
        locals.var_t1_dn10 = assign49760_e82903_d_n10;
        locals.var_t1_dn11 = assign49760_e82903_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49770_e82916, assign49770_e82916_d_n3, assign49770_e82916_d_n4, assign49770_e82916_d_n5, assign49770_e82916_d_n6, assign49770_e82916_d_n7, assign49770_e82916_d_n8, assign49770_e82916_d_n9, assign49770_e82916_d_n10, assign49770_e82916_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49770_e82914: f64 = (locals.var_wstsi * locals.var_jtund);
        (assign49770_e82914, (locals.var_wstsi * locals.var_jtund_dn3), (locals.var_wstsi * locals.var_jtund_dn4), (locals.var_wstsi * locals.var_jtund_dn5), (locals.var_wstsi * locals.var_jtund_dn6), (locals.var_wstsi * locals.var_jtund_dn7), (locals.var_wstsi * locals.var_jtund_dn8), (locals.var_wstsi * locals.var_jtund_dn9), (locals.var_wstsi * locals.var_jtund_dn10), (locals.var_wstsi * locals.var_jtund_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49770_e82916;
        locals.var_t3_dn3 = assign49770_e82916_d_n3;
        locals.var_t3_dn4 = assign49770_e82916_d_n4;
        locals.var_t3_dn5 = assign49770_e82916_d_n5;
        locals.var_t3_dn6 = assign49770_e82916_d_n6;
        locals.var_t3_dn7 = assign49770_e82916_d_n7;
        locals.var_t3_dn8 = assign49770_e82916_d_n8;
        locals.var_t3_dn9 = assign49770_e82916_d_n9;
        locals.var_t3_dn10 = assign49770_e82916_d_n10;
        locals.var_t3_dn11 = assign49770_e82916_d_n11;
        locals.var_t3_rv = 0.0;

        let assign49830_e82970: f64 = if p.p36 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard759 = assign49830_e82970;
        locals.var_guard759_rv = 0.0;

        let (assign49840_e82979, assign49840_e82979_d_n3, assign49840_e82979_d_n4, assign49840_e82979_d_n5, assign49840_e82979_d_n6, assign49840_e82979_d_n7, assign49840_e82979_d_n8, assign49840_e82979_d_n9, assign49840_e82979_d_n10, assign49840_e82979_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) {
        let assign49840_e82977: f64 = (locals.var_epsratio * p.p76);
        (assign49840_e82977, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49840_e82979;
        locals.var_t0_dn3 = assign49840_e82979_d_n3;
        locals.var_t0_dn4 = assign49840_e82979_d_n4;
        locals.var_t0_dn5 = assign49840_e82979_d_n5;
        locals.var_t0_dn6 = assign49840_e82979_d_n6;
        locals.var_t0_dn7 = assign49840_e82979_d_n7;
        locals.var_t0_dn8 = assign49840_e82979_d_n8;
        locals.var_t0_dn9 = assign49840_e82979_d_n9;
        locals.var_t0_dn10 = assign49840_e82979_d_n10;
        locals.var_t0_dn11 = assign49840_e82979_d_n11;
        locals.var_t0_rv = 0.0;

        let assign49850_e82990: f64 = if (((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) || (locals.var_cgidl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard760 = assign49850_e82990;
        locals.var_guard760_rv = 0.0;

        let (assign49860_e82999, assign49860_e82999_d_n3, assign49860_e82999_d_n4, assign49860_e82999_d_n5, assign49860_e82999_d_n6, assign49860_e82999_d_n7, assign49860_e82999_d_n8, assign49860_e82999_d_n9, assign49860_e82999_d_n10, assign49860_e82999_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign49860_e82999;
        locals.var_t6_dn3 = assign49860_e82999_d_n3;
        locals.var_t6_dn4 = assign49860_e82999_d_n4;
        locals.var_t6_dn5 = assign49860_e82999_d_n5;
        locals.var_t6_dn6 = assign49860_e82999_d_n6;
        locals.var_t6_dn7 = assign49860_e82999_d_n7;
        locals.var_t6_dn8 = assign49860_e82999_d_n8;
        locals.var_t6_dn9 = assign49860_e82999_d_n9;
        locals.var_t6_dn10 = assign49860_e82999_d_n10;
        locals.var_t6_dn11 = assign49860_e82999_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign49870_e83016, assign49870_e83016_d_n3, assign49870_e83016_d_n4, assign49870_e83016_d_n5, assign49870_e83016_d_n6, assign49870_e83016_d_n7, assign49870_e83016_d_n8, assign49870_e83016_d_n9, assign49870_e83016_d_n10, assign49870_e83016_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49870_e83008: f64 = (-locals.var_vgd_noswap);
        let assign49870_e83010: f64 = (assign49870_e83008 - locals.var_egidl_i);
        let assign49870_e83012: f64 = (assign49870_e83010 + locals.var_vfbsdr);
        let assign49870_e83014: f64 = (assign49870_e83012 / locals.var_t0);
        (assign49870_e83014, (-((assign49870_e83012 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn6) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn7) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn8) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign49870_e83012 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_dn10) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign49870_e83012 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49870_e83016;
        locals.var_t1_dn3 = assign49870_e83016_d_n3;
        locals.var_t1_dn4 = assign49870_e83016_d_n4;
        locals.var_t1_dn5 = assign49870_e83016_d_n5;
        locals.var_t1_dn6 = assign49870_e83016_d_n6;
        locals.var_t1_dn7 = assign49870_e83016_d_n7;
        locals.var_t1_dn8 = assign49870_e83016_d_n8;
        locals.var_t1_dn9 = assign49870_e83016_d_n9;
        locals.var_t1_dn10 = assign49870_e83016_d_n10;
        locals.var_t1_dn11 = assign49870_e83016_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49880_e83039, assign49880_e83039_d_n3, assign49880_e83039_d_n4, assign49880_e83039_d_n5, assign49880_e83039_d_n6, assign49880_e83039_d_n7, assign49880_e83039_d_n8, assign49880_e83039_d_n9, assign49880_e83039_d_n10, assign49880_e83039_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49880_e83028: f64 = (locals.var_t1 * locals.var_t1);
        let assign49880_e83031: f64 = (4.0 * 0.01);
        let assign49880_e83033: f64 = (assign49880_e83031 * 0.01);
        let assign49880_e83034: f64 = (assign49880_e83028 + assign49880_e83033);
        let assign49880_e83035: f64 = (assign49880_e83034).sqrt();
        let assign49880_e83036: f64 = (locals.var_t1 + assign49880_e83035);
        let assign49880_e83037: f64 = (0.5 * assign49880_e83036);
        (assign49880_e83037, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign49880_e83035)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49880_e83039;
        locals.var_t1_dn3 = assign49880_e83039_d_n3;
        locals.var_t1_dn4 = assign49880_e83039_d_n4;
        locals.var_t1_dn5 = assign49880_e83039_d_n5;
        locals.var_t1_dn6 = assign49880_e83039_d_n6;
        locals.var_t1_dn7 = assign49880_e83039_d_n7;
        locals.var_t1_dn8 = assign49880_e83039_d_n8;
        locals.var_t1_dn9 = assign49880_e83039_d_n9;
        locals.var_t1_dn10 = assign49880_e83039_d_n10;
        locals.var_t1_dn11 = assign49880_e83039_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49890_e83053, assign49890_e83053_d_n3, assign49890_e83053_d_n4, assign49890_e83053_d_n5, assign49890_e83053_d_n6, assign49890_e83053_d_n7, assign49890_e83053_d_n8, assign49890_e83053_d_n9, assign49890_e83053_d_n10, assign49890_e83053_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49890_e83050: f64 = (locals.var_t1 + 0.001);
        let assign49890_e83051: f64 = (locals.var_bgidl_t / assign49890_e83050);
        (assign49890_e83051, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign49890_e83050 * assign49890_e83050))), (((locals.var_bgidl_t_dn4 * assign49890_e83050) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign49890_e83050 * assign49890_e83050)), (((locals.var_bgidl_t_dn5 * assign49890_e83050) - (locals.var_bgidl_t * locals.var_t1_dn5)) / (assign49890_e83050 * assign49890_e83050)), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign49890_e83050 * assign49890_e83050))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign49890_e83053;
        locals.var_t2_dn3 = assign49890_e83053_d_n3;
        locals.var_t2_dn4 = assign49890_e83053_d_n4;
        locals.var_t2_dn5 = assign49890_e83053_d_n5;
        locals.var_t2_dn6 = assign49890_e83053_d_n6;
        locals.var_t2_dn7 = assign49890_e83053_d_n7;
        locals.var_t2_dn8 = assign49890_e83053_d_n8;
        locals.var_t2_dn9 = assign49890_e83053_d_n9;
        locals.var_t2_dn10 = assign49890_e83053_d_n10;
        locals.var_t2_dn11 = assign49890_e83053_d_n11;
        locals.var_t2_rv = 0.0;

        let assign49900_e83056: f64 = if locals.var_cgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard761 = assign49900_e83056;
        locals.var_guard761_rv = 0.0;

        let (assign49910_e83072, assign49910_e83072_d_n3, assign49910_e83072_d_n4, assign49910_e83072_d_n5, assign49910_e83072_d_n6, assign49910_e83072_d_n7, assign49910_e83072_d_n8, assign49910_e83072_d_n9, assign49910_e83072_d_n10, assign49910_e83072_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let assign49910_e83068: f64 = (locals.var_vdb_noswap * locals.var_vdb_noswap);
        let assign49910_e83070: f64 = (assign49910_e83068 * locals.var_vdb_noswap);
        (assign49910_e83070, 0.0, 0.0, 0.0, ((((locals.var_vdb_noswap_dn6 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn6)) * locals.var_vdb_noswap) + (assign49910_e83068 * locals.var_vdb_noswap_dn6)), ((((locals.var_vdb_noswap_dn7 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn7)) * locals.var_vdb_noswap) + (assign49910_e83068 * locals.var_vdb_noswap_dn7)), 0.0, 0.0, ((((locals.var_vdb_noswap_dn10 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn10)) * locals.var_vdb_noswap) + (assign49910_e83068 * locals.var_vdb_noswap_dn10)), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49910_e83072;
        locals.var_t3_dn3 = assign49910_e83072_d_n3;
        locals.var_t3_dn4 = assign49910_e83072_d_n4;
        locals.var_t3_dn5 = assign49910_e83072_d_n5;
        locals.var_t3_dn6 = assign49910_e83072_d_n6;
        locals.var_t3_dn7 = assign49910_e83072_d_n7;
        locals.var_t3_dn8 = assign49910_e83072_d_n8;
        locals.var_t3_dn9 = assign49910_e83072_d_n9;
        locals.var_t3_dn10 = assign49910_e83072_d_n10;
        locals.var_t3_dn11 = assign49910_e83072_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign49920_e83089, assign49920_e83089_d_n3, assign49920_e83089_d_n4, assign49920_e83089_d_n5, assign49920_e83089_d_n6, assign49920_e83089_d_n7, assign49920_e83089_d_n8, assign49920_e83089_d_n9, assign49920_e83089_d_n10, assign49920_e83089_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let assign49920_e83084: f64 = (locals.var_t3).abs();
        let assign49920_e83085: f64 = (locals.var_cgidl_i + assign49920_e83084);
        let assign49920_e83087: f64 = (assign49920_e83085 + 0.0001);
        (assign49920_e83087, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) },)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign49920_e83089;
        locals.var_t4_dn3 = assign49920_e83089_d_n3;
        locals.var_t4_dn4 = assign49920_e83089_d_n4;
        locals.var_t4_dn5 = assign49920_e83089_d_n5;
        locals.var_t4_dn6 = assign49920_e83089_d_n6;
        locals.var_t4_dn7 = assign49920_e83089_d_n7;
        locals.var_t4_dn8 = assign49920_e83089_d_n8;
        locals.var_t4_dn9 = assign49920_e83089_d_n9;
        locals.var_t4_dn10 = assign49920_e83089_d_n10;
        locals.var_t4_dn11 = assign49920_e83089_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign49930_e83122, assign49930_e83122_d_n3, assign49930_e83122_d_n4, assign49930_e83122_d_n5, assign49930_e83122_d_n6, assign49930_e83122_d_n7, assign49930_e83122_d_n8, assign49930_e83122_d_n9, assign49930_e83122_d_n10, assign49930_e83122_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign49930_e83102: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign49930_e83105: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign49930_e83108: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign49930_e83109: f64 = (assign49930_e83105 * assign49930_e83108);
        let assign49930_e83112: f64 = (4.0 * 1e-6);
        let assign49930_e83114: f64 = (assign49930_e83112 * 1e-6);
        let assign49930_e83115: f64 = (assign49930_e83109 + assign49930_e83114);
        let assign49930_e83116: f64 = (assign49930_e83115).sqrt();
        let assign49930_e83117: f64 = (assign49930_e83102 + assign49930_e83116);
        let assign49930_e83118: f64 = (0.5 * assign49930_e83117);
        let assign49930_e83120: f64 = (assign49930_e83118 - 1e-6);
        (assign49930_e83120, (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign49930_e83122;
        locals.var_t5_dn3 = assign49930_e83122_d_n3;
        locals.var_t5_dn4 = assign49930_e83122_d_n4;
        locals.var_t5_dn5 = assign49930_e83122_d_n5;
        locals.var_t5_dn6 = assign49930_e83122_d_n6;
        locals.var_t5_dn7 = assign49930_e83122_d_n7;
        locals.var_t5_dn8 = assign49930_e83122_d_n8;
        locals.var_t5_dn9 = assign49930_e83122_d_n9;
        locals.var_t5_dn10 = assign49930_e83122_d_n10;
        locals.var_t5_dn11 = assign49930_e83122_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign49940_e83135, assign49940_e83135_d_n3, assign49940_e83135_d_n4, assign49940_e83135_d_n5, assign49940_e83135_d_n6, assign49940_e83135_d_n7, assign49940_e83135_d_n8, assign49940_e83135_d_n9, assign49940_e83135_d_n10, assign49940_e83135_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign49940_e83135;
        locals.var_t5_dn3 = assign49940_e83135_d_n3;
        locals.var_t5_dn4 = assign49940_e83135_d_n4;
        locals.var_t5_dn5 = assign49940_e83135_d_n5;
        locals.var_t5_dn6 = assign49940_e83135_d_n6;
        locals.var_t5_dn7 = assign49940_e83135_d_n7;
        locals.var_t5_dn8 = assign49940_e83135_d_n8;
        locals.var_t5_dn9 = assign49940_e83135_d_n9;
        locals.var_t5_dn10 = assign49940_e83135_d_n10;
        locals.var_t5_dn11 = assign49940_e83135_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign49950_e83155, assign49950_e83155_d_n3, assign49950_e83155_d_n4, assign49950_e83155_d_n5, assign49950_e83155_d_n6, assign49950_e83155_d_n7, assign49950_e83155_d_n8, assign49950_e83155_d_n9, assign49950_e83155_d_n10, assign49950_e83155_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49950_e83145: f64 = (locals.var_agidl_i * locals.var_wdiod);
        let assign49950_e83147: f64 = (assign49950_e83145 * locals.var_t1);
        let assign49950_e83149: f64 = (-locals.var_t2);
        let assign49950_e83150: f64 = { let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign49950_e83151: f64 = (assign49950_e83147 * assign49950_e83150);
        let assign49950_e83153: f64 = (assign49950_e83151 * locals.var_t5);
        (assign49950_e83153, (((((assign49950_e83145 * locals.var_t1_dn3) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn3)), (((((assign49950_e83145 * locals.var_t1_dn4) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn4)), (((((assign49950_e83145 * locals.var_t1_dn5) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn5)), (((((assign49950_e83145 * locals.var_t1_dn6) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn6)), (((((assign49950_e83145 * locals.var_t1_dn7) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn7)), (((((assign49950_e83145 * locals.var_t1_dn8) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn8)), (((((assign49950_e83145 * locals.var_t1_dn9) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn9)), (((((assign49950_e83145 * locals.var_t1_dn10) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn10)), (((((assign49950_e83145 * locals.var_t1_dn11) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign49950_e83155;
        locals.var_t6_dn3 = assign49950_e83155_d_n3;
        locals.var_t6_dn4 = assign49950_e83155_d_n4;
        locals.var_t6_dn5 = assign49950_e83155_d_n5;
        locals.var_t6_dn6 = assign49950_e83155_d_n6;
        locals.var_t6_dn7 = assign49950_e83155_d_n7;
        locals.var_t6_dn8 = assign49950_e83155_d_n8;
        locals.var_t6_dn9 = assign49950_e83155_d_n9;
        locals.var_t6_dn10 = assign49950_e83155_d_n10;
        locals.var_t6_dn11 = assign49950_e83155_d_n11;
        locals.var_t6_rv = 0.0;

        let assign49970_e83173: f64 = if (((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) || (locals.var_cgisl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard762 = assign49970_e83173;
        locals.var_guard762_rv = 0.0;

        let (assign49980_e83182, assign49980_e83182_d_n3, assign49980_e83182_d_n4, assign49980_e83182_d_n5, assign49980_e83182_d_n6, assign49980_e83182_d_n7, assign49980_e83182_d_n8, assign49980_e83182_d_n9, assign49980_e83182_d_n10, assign49980_e83182_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign49980_e83182;
        locals.var_t6_dn3 = assign49980_e83182_d_n3;
        locals.var_t6_dn4 = assign49980_e83182_d_n4;
        locals.var_t6_dn5 = assign49980_e83182_d_n5;
        locals.var_t6_dn6 = assign49980_e83182_d_n6;
        locals.var_t6_dn7 = assign49980_e83182_d_n7;
        locals.var_t6_dn8 = assign49980_e83182_d_n8;
        locals.var_t6_dn9 = assign49980_e83182_d_n9;
        locals.var_t6_dn10 = assign49980_e83182_d_n10;
        locals.var_t6_dn11 = assign49980_e83182_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign49990_e83199, assign49990_e83199_d_n3, assign49990_e83199_d_n4, assign49990_e83199_d_n5, assign49990_e83199_d_n6, assign49990_e83199_d_n7, assign49990_e83199_d_n8, assign49990_e83199_d_n9, assign49990_e83199_d_n10, assign49990_e83199_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign49990_e83191: f64 = (-locals.var_vgs_noswap);
        let assign49990_e83193: f64 = (assign49990_e83191 - locals.var_egisl_i);
        let assign49990_e83195: f64 = (assign49990_e83193 + locals.var_vfbsdr);
        let assign49990_e83197: f64 = (assign49990_e83195 / locals.var_t0);
        (assign49990_e83197, (-((assign49990_e83195 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn6) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn7) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn8) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign49990_e83195 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_dn10) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign49990_e83195 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49990_e83199;
        locals.var_t1_dn3 = assign49990_e83199_d_n3;
        locals.var_t1_dn4 = assign49990_e83199_d_n4;
        locals.var_t1_dn5 = assign49990_e83199_d_n5;
        locals.var_t1_dn6 = assign49990_e83199_d_n6;
        locals.var_t1_dn7 = assign49990_e83199_d_n7;
        locals.var_t1_dn8 = assign49990_e83199_d_n8;
        locals.var_t1_dn9 = assign49990_e83199_d_n9;
        locals.var_t1_dn10 = assign49990_e83199_d_n10;
        locals.var_t1_dn11 = assign49990_e83199_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50000_e83222, assign50000_e83222_d_n3, assign50000_e83222_d_n4, assign50000_e83222_d_n5, assign50000_e83222_d_n6, assign50000_e83222_d_n7, assign50000_e83222_d_n8, assign50000_e83222_d_n9, assign50000_e83222_d_n10, assign50000_e83222_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign50000_e83211: f64 = (locals.var_t1 * locals.var_t1);
        let assign50000_e83214: f64 = (4.0 * 0.01);
        let assign50000_e83216: f64 = (assign50000_e83214 * 0.01);
        let assign50000_e83217: f64 = (assign50000_e83211 + assign50000_e83216);
        let assign50000_e83218: f64 = (assign50000_e83217).sqrt();
        let assign50000_e83219: f64 = (locals.var_t1 + assign50000_e83218);
        let assign50000_e83220: f64 = (0.5 * assign50000_e83219);
        (assign50000_e83220, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign50000_e83218)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50000_e83222;
        locals.var_t1_dn3 = assign50000_e83222_d_n3;
        locals.var_t1_dn4 = assign50000_e83222_d_n4;
        locals.var_t1_dn5 = assign50000_e83222_d_n5;
        locals.var_t1_dn6 = assign50000_e83222_d_n6;
        locals.var_t1_dn7 = assign50000_e83222_d_n7;
        locals.var_t1_dn8 = assign50000_e83222_d_n8;
        locals.var_t1_dn9 = assign50000_e83222_d_n9;
        locals.var_t1_dn10 = assign50000_e83222_d_n10;
        locals.var_t1_dn11 = assign50000_e83222_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50010_e83236, assign50010_e83236_d_n3, assign50010_e83236_d_n4, assign50010_e83236_d_n5, assign50010_e83236_d_n6, assign50010_e83236_d_n7, assign50010_e83236_d_n8, assign50010_e83236_d_n9, assign50010_e83236_d_n10, assign50010_e83236_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign50010_e83233: f64 = (locals.var_t1 + 0.001);
        let assign50010_e83234: f64 = (locals.var_bgisl_t / assign50010_e83233);
        (assign50010_e83234, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign50010_e83233 * assign50010_e83233))), (((locals.var_bgisl_t_dn4 * assign50010_e83233) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign50010_e83233 * assign50010_e83233)), (((locals.var_bgisl_t_dn5 * assign50010_e83233) - (locals.var_bgisl_t * locals.var_t1_dn5)) / (assign50010_e83233 * assign50010_e83233)), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign50010_e83233 * assign50010_e83233))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50010_e83236;
        locals.var_t2_dn3 = assign50010_e83236_d_n3;
        locals.var_t2_dn4 = assign50010_e83236_d_n4;
        locals.var_t2_dn5 = assign50010_e83236_d_n5;
        locals.var_t2_dn6 = assign50010_e83236_d_n6;
        locals.var_t2_dn7 = assign50010_e83236_d_n7;
        locals.var_t2_dn8 = assign50010_e83236_d_n8;
        locals.var_t2_dn9 = assign50010_e83236_d_n9;
        locals.var_t2_dn10 = assign50010_e83236_d_n10;
        locals.var_t2_dn11 = assign50010_e83236_d_n11;
        locals.var_t2_rv = 0.0;

        let assign50020_e83239: f64 = if locals.var_cgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard763 = assign50020_e83239;
        locals.var_guard763_rv = 0.0;

        let (assign50030_e83255, assign50030_e83255_d_n3, assign50030_e83255_d_n4, assign50030_e83255_d_n5, assign50030_e83255_d_n6, assign50030_e83255_d_n7, assign50030_e83255_d_n8, assign50030_e83255_d_n9, assign50030_e83255_d_n10, assign50030_e83255_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let assign50030_e83251: f64 = (locals.var_vsb_noswap * locals.var_vsb_noswap);
        let assign50030_e83253: f64 = (assign50030_e83251 * locals.var_vsb_noswap);
        (assign50030_e83253, 0.0, 0.0, 0.0, ((((locals.var_vsb_noswap_dn6 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn6)) * locals.var_vsb_noswap) + (assign50030_e83251 * locals.var_vsb_noswap_dn6)), ((((locals.var_vsb_noswap_dn7 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn7)) * locals.var_vsb_noswap) + (assign50030_e83251 * locals.var_vsb_noswap_dn7)), 0.0, 0.0, ((((locals.var_vsb_noswap_dn10 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn10)) * locals.var_vsb_noswap) + (assign50030_e83251 * locals.var_vsb_noswap_dn10)), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50030_e83255;
        locals.var_t3_dn3 = assign50030_e83255_d_n3;
        locals.var_t3_dn4 = assign50030_e83255_d_n4;
        locals.var_t3_dn5 = assign50030_e83255_d_n5;
        locals.var_t3_dn6 = assign50030_e83255_d_n6;
        locals.var_t3_dn7 = assign50030_e83255_d_n7;
        locals.var_t3_dn8 = assign50030_e83255_d_n8;
        locals.var_t3_dn9 = assign50030_e83255_d_n9;
        locals.var_t3_dn10 = assign50030_e83255_d_n10;
        locals.var_t3_dn11 = assign50030_e83255_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50040_e83272, assign50040_e83272_d_n3, assign50040_e83272_d_n4, assign50040_e83272_d_n5, assign50040_e83272_d_n6, assign50040_e83272_d_n7, assign50040_e83272_d_n8, assign50040_e83272_d_n9, assign50040_e83272_d_n10, assign50040_e83272_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let assign50040_e83267: f64 = (locals.var_t3).abs();
        let assign50040_e83268: f64 = (locals.var_cgisl_i + assign50040_e83267);
        let assign50040_e83270: f64 = (assign50040_e83268 + 0.0001);
        (assign50040_e83270, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) },)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50040_e83272;
        locals.var_t4_dn3 = assign50040_e83272_d_n3;
        locals.var_t4_dn4 = assign50040_e83272_d_n4;
        locals.var_t4_dn5 = assign50040_e83272_d_n5;
        locals.var_t4_dn6 = assign50040_e83272_d_n6;
        locals.var_t4_dn7 = assign50040_e83272_d_n7;
        locals.var_t4_dn8 = assign50040_e83272_d_n8;
        locals.var_t4_dn9 = assign50040_e83272_d_n9;
        locals.var_t4_dn10 = assign50040_e83272_d_n10;
        locals.var_t4_dn11 = assign50040_e83272_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign50050_e83305, assign50050_e83305_d_n3, assign50050_e83305_d_n4, assign50050_e83305_d_n5, assign50050_e83305_d_n6, assign50050_e83305_d_n7, assign50050_e83305_d_n8, assign50050_e83305_d_n9, assign50050_e83305_d_n10, assign50050_e83305_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4;
        let assign50050_e83285: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign50050_e83288: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign50050_e83291: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign50050_e83292: f64 = (assign50050_e83288 * assign50050_e83291);
        let assign50050_e83295: f64 = (4.0 * 1e-6);
        let assign50050_e83297: f64 = (assign50050_e83295 * 1e-6);
        let assign50050_e83298: f64 = (assign50050_e83292 + assign50050_e83297);
        let assign50050_e83299: f64 = (assign50050_e83298).sqrt();
        let assign50050_e83300: f64 = (assign50050_e83285 + assign50050_e83299);
        let assign50050_e83301: f64 = (0.5 * assign50050_e83300);
        let assign50050_e83303: f64 = (assign50050_e83301 - 1e-6);
        (assign50050_e83303, (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50050_e83305;
        locals.var_t5_dn3 = assign50050_e83305_d_n3;
        locals.var_t5_dn4 = assign50050_e83305_d_n4;
        locals.var_t5_dn5 = assign50050_e83305_d_n5;
        locals.var_t5_dn6 = assign50050_e83305_d_n6;
        locals.var_t5_dn7 = assign50050_e83305_d_n7;
        locals.var_t5_dn8 = assign50050_e83305_d_n8;
        locals.var_t5_dn9 = assign50050_e83305_d_n9;
        locals.var_t5_dn10 = assign50050_e83305_d_n10;
        locals.var_t5_dn11 = assign50050_e83305_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign50060_e83318, assign50060_e83318_d_n3, assign50060_e83318_d_n4, assign50060_e83318_d_n5, assign50060_e83318_d_n6, assign50060_e83318_d_n7, assign50060_e83318_d_n8, assign50060_e83318_d_n9, assign50060_e83318_d_n10, assign50060_e83318_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50060_e83318;
        locals.var_t5_dn3 = assign50060_e83318_d_n3;
        locals.var_t5_dn4 = assign50060_e83318_d_n4;
        locals.var_t5_dn5 = assign50060_e83318_d_n5;
        locals.var_t5_dn6 = assign50060_e83318_d_n6;
        locals.var_t5_dn7 = assign50060_e83318_d_n7;
        locals.var_t5_dn8 = assign50060_e83318_d_n8;
        locals.var_t5_dn9 = assign50060_e83318_d_n9;
        locals.var_t5_dn10 = assign50060_e83318_d_n10;
        locals.var_t5_dn11 = assign50060_e83318_d_n11;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_174(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50070_e83338, assign50070_e83338_d_n3, assign50070_e83338_d_n4, assign50070_e83338_d_n5, assign50070_e83338_d_n6, assign50070_e83338_d_n7, assign50070_e83338_d_n8, assign50070_e83338_d_n9, assign50070_e83338_d_n10, assign50070_e83338_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign50070_e83328: f64 = (locals.var_agisl_i * locals.var_wdios);
        let assign50070_e83330: f64 = (assign50070_e83328 * locals.var_t1);
        let assign50070_e83332: f64 = (-locals.var_t2);
        let assign50070_e83333: f64 = { let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50070_e83334: f64 = (assign50070_e83330 * assign50070_e83333);
        let assign50070_e83336: f64 = (assign50070_e83334 * locals.var_t5);
        (assign50070_e83336, (((((assign50070_e83328 * locals.var_t1_dn3) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn3)), (((((assign50070_e83328 * locals.var_t1_dn4) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn4)), (((((assign50070_e83328 * locals.var_t1_dn5) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn5)), (((((assign50070_e83328 * locals.var_t1_dn6) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn6)), (((((assign50070_e83328 * locals.var_t1_dn7) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn7)), (((((assign50070_e83328 * locals.var_t1_dn8) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn8)), (((((assign50070_e83328 * locals.var_t1_dn9) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn9)), (((((assign50070_e83328 * locals.var_t1_dn10) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn10)), (((((assign50070_e83328 * locals.var_t1_dn11) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50070_e83338;
        locals.var_t6_dn3 = assign50070_e83338_d_n3;
        locals.var_t6_dn4 = assign50070_e83338_d_n4;
        locals.var_t6_dn5 = assign50070_e83338_d_n5;
        locals.var_t6_dn6 = assign50070_e83338_d_n6;
        locals.var_t6_dn7 = assign50070_e83338_d_n7;
        locals.var_t6_dn8 = assign50070_e83338_d_n8;
        locals.var_t6_dn9 = assign50070_e83338_d_n9;
        locals.var_t6_dn10 = assign50070_e83338_d_n10;
        locals.var_t6_dn11 = assign50070_e83338_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign50090_e83355, assign50090_e83355_d_n3, assign50090_e83355_d_n4, assign50090_e83355_d_n5, assign50090_e83355_d_n6, assign50090_e83355_d_n7, assign50090_e83355_d_n8, assign50090_e83355_d_n9, assign50090_e83355_d_n10, assign50090_e83355_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50090_e83353: f64 = (locals.var_epsratio * p.p76);
        (assign50090_e83353, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50090_e83355;
        locals.var_t0_dn3 = assign50090_e83355_d_n3;
        locals.var_t0_dn4 = assign50090_e83355_d_n4;
        locals.var_t0_dn5 = assign50090_e83355_d_n5;
        locals.var_t0_dn6 = assign50090_e83355_d_n6;
        locals.var_t0_dn7 = assign50090_e83355_d_n7;
        locals.var_t0_dn8 = assign50090_e83355_d_n8;
        locals.var_t0_dn9 = assign50090_e83355_d_n9;
        locals.var_t0_dn10 = assign50090_e83355_d_n10;
        locals.var_t0_dn11 = assign50090_e83355_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50100_e83367, assign50100_e83367_d_n6, assign50100_e83367_d_n7, assign50100_e83367_d_n8, assign50100_e83367_d_n10,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50100_e83363: f64 = (locals.var_rgisl_i * locals.var_vg);
        let assign50100_e83365: f64 = (assign50100_e83363 - locals.var_vd);
        (assign50100_e83365, (-locals.var_vd_dn6), (-locals.var_vd_dn7), (locals.var_rgisl_i * locals.var_vg_dn8), ((locals.var_rgisl_i * locals.var_vg_dn10) - locals.var_vd_dn10),)
    } else {
        (locals.var_vgd_noswap_1, locals.var_vgd_noswap_1_dn6, locals.var_vgd_noswap_1_dn7, locals.var_vgd_noswap_1_dn8, locals.var_vgd_noswap_1_dn10,)
    }
};
        locals.var_vgd_noswap_1 = assign50100_e83367;
        locals.var_vgd_noswap_1_dn6 = assign50100_e83367_d_n6;
        locals.var_vgd_noswap_1_dn7 = assign50100_e83367_d_n7;
        locals.var_vgd_noswap_1_dn8 = assign50100_e83367_d_n8;
        locals.var_vgd_noswap_1_dn10 = assign50100_e83367_d_n10;
        locals.var_vgd_noswap_1_rv = 0.0;

        let (assign50110_e83379, assign50110_e83379_d_n6, assign50110_e83379_d_n7, assign50110_e83379_d_n8, assign50110_e83379_d_n10,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50110_e83375: f64 = (locals.var_rgidl_i * locals.var_vg);
        let assign50110_e83377: f64 = (assign50110_e83375 - locals.var_vs);
        (assign50110_e83377, (-locals.var_vs_dn6), (-locals.var_vs_dn7), (locals.var_rgidl_i * locals.var_vg_dn8), ((locals.var_rgidl_i * locals.var_vg_dn10) - locals.var_vs_dn10),)
    } else {
        (locals.var_vgs_noswap_1, locals.var_vgs_noswap_1_dn6, locals.var_vgs_noswap_1_dn7, locals.var_vgs_noswap_1_dn8, locals.var_vgs_noswap_1_dn10,)
    }
};
        locals.var_vgs_noswap_1 = assign50110_e83379;
        locals.var_vgs_noswap_1_dn6 = assign50110_e83379_d_n6;
        locals.var_vgs_noswap_1_dn7 = assign50110_e83379_d_n7;
        locals.var_vgs_noswap_1_dn8 = assign50110_e83379_d_n8;
        locals.var_vgs_noswap_1_dn10 = assign50110_e83379_d_n10;
        locals.var_vgs_noswap_1_rv = 0.0;

        let (assign50120_e83389, assign50120_e83389_d_n3, assign50120_e83389_d_n4, assign50120_e83389_d_n5, assign50120_e83389_d_n6, assign50120_e83389_d_n7, assign50120_e83389_d_n8, assign50120_e83389_d_n9, assign50120_e83389_d_n10, assign50120_e83389_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50120_e83387: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign50120_e83387, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50120_e83389;
        locals.var_t2_dn3 = assign50120_e83389_d_n3;
        locals.var_t2_dn4 = assign50120_e83389_d_n4;
        locals.var_t2_dn5 = assign50120_e83389_d_n5;
        locals.var_t2_dn6 = assign50120_e83389_d_n6;
        locals.var_t2_dn7 = assign50120_e83389_d_n7;
        locals.var_t2_dn8 = assign50120_e83389_d_n8;
        locals.var_t2_dn9 = assign50120_e83389_d_n9;
        locals.var_t2_dn10 = assign50120_e83389_d_n10;
        locals.var_t2_dn11 = assign50120_e83389_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign50130_e83402, assign50130_e83402_d_n3, assign50130_e83402_d_n4, assign50130_e83402_d_n5, assign50130_e83402_d_n6, assign50130_e83402_d_n7, assign50130_e83402_d_n8, assign50130_e83402_d_n9, assign50130_e83402_d_n10, assign50130_e83402_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50130_e83397: f64 = (locals.var_t2 * locals.var_t2);
        let assign50130_e83399: f64 = (assign50130_e83397 + 0.0001);
        let assign50130_e83400: f64 = (assign50130_e83399).sqrt();
        (assign50130_e83400, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign50130_e83400)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign50130_e83402;
        locals.var_vgs_eff_dn3 = assign50130_e83402_d_n3;
        locals.var_vgs_eff_dn4 = assign50130_e83402_d_n4;
        locals.var_vgs_eff_dn5 = assign50130_e83402_d_n5;
        locals.var_vgs_eff_dn6 = assign50130_e83402_d_n6;
        locals.var_vgs_eff_dn7 = assign50130_e83402_d_n7;
        locals.var_vgs_eff_dn8 = assign50130_e83402_d_n8;
        locals.var_vgs_eff_dn9 = assign50130_e83402_d_n9;
        locals.var_vgs_eff_dn10 = assign50130_e83402_d_n10;
        locals.var_vgs_eff_dn11 = assign50130_e83402_d_n11;
        locals.var_vgs_eff_rv = 0.0;

        let assign50140_e83409: f64 = if ((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard764 = assign50140_e83409;
        locals.var_guard764_rv = 0.0;

        let (assign50150_e83419, assign50150_e83419_d_n3, assign50150_e83419_d_n4, assign50150_e83419_d_n5, assign50150_e83419_d_n6, assign50150_e83419_d_n7, assign50150_e83419_d_n8, assign50150_e83419_d_n9, assign50150_e83419_d_n10, assign50150_e83419_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50150_e83419;
        locals.var_t6_dn3 = assign50150_e83419_d_n3;
        locals.var_t6_dn4 = assign50150_e83419_d_n4;
        locals.var_t6_dn5 = assign50150_e83419_d_n5;
        locals.var_t6_dn6 = assign50150_e83419_d_n6;
        locals.var_t6_dn7 = assign50150_e83419_d_n7;
        locals.var_t6_dn8 = assign50150_e83419_d_n8;
        locals.var_t6_dn9 = assign50150_e83419_d_n9;
        locals.var_t6_dn10 = assign50150_e83419_d_n10;
        locals.var_t6_dn11 = assign50150_e83419_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign50160_e83437, assign50160_e83437_d_n3, assign50160_e83437_d_n4, assign50160_e83437_d_n5, assign50160_e83437_d_n6, assign50160_e83437_d_n7, assign50160_e83437_d_n8, assign50160_e83437_d_n9, assign50160_e83437_d_n10, assign50160_e83437_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50160_e83429: f64 = (-locals.var_vgd_noswap_1);
        let assign50160_e83431: f64 = (assign50160_e83429 - locals.var_egidl_i);
        let assign50160_e83433: f64 = (assign50160_e83431 + locals.var_vfbsdr);
        let assign50160_e83435: f64 = (assign50160_e83433 / locals.var_t0);
        (assign50160_e83435, (-((assign50160_e83433 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn6) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn7) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn8) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign50160_e83433 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_1_dn10) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign50160_e83433 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50160_e83437;
        locals.var_t1_dn3 = assign50160_e83437_d_n3;
        locals.var_t1_dn4 = assign50160_e83437_d_n4;
        locals.var_t1_dn5 = assign50160_e83437_d_n5;
        locals.var_t1_dn6 = assign50160_e83437_d_n6;
        locals.var_t1_dn7 = assign50160_e83437_d_n7;
        locals.var_t1_dn8 = assign50160_e83437_d_n8;
        locals.var_t1_dn9 = assign50160_e83437_d_n9;
        locals.var_t1_dn10 = assign50160_e83437_d_n10;
        locals.var_t1_dn11 = assign50160_e83437_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50170_e83461, assign50170_e83461_d_n3, assign50170_e83461_d_n4, assign50170_e83461_d_n5, assign50170_e83461_d_n6, assign50170_e83461_d_n7, assign50170_e83461_d_n8, assign50170_e83461_d_n9, assign50170_e83461_d_n10, assign50170_e83461_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50170_e83450: f64 = (locals.var_t1 * locals.var_t1);
        let assign50170_e83453: f64 = (4.0 * 0.01);
        let assign50170_e83455: f64 = (assign50170_e83453 * 0.01);
        let assign50170_e83456: f64 = (assign50170_e83450 + assign50170_e83455);
        let assign50170_e83457: f64 = (assign50170_e83456).sqrt();
        let assign50170_e83458: f64 = (locals.var_t1 + assign50170_e83457);
        let assign50170_e83459: f64 = (0.5 * assign50170_e83458);
        (assign50170_e83459, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign50170_e83457)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50170_e83461;
        locals.var_t1_dn3 = assign50170_e83461_d_n3;
        locals.var_t1_dn4 = assign50170_e83461_d_n4;
        locals.var_t1_dn5 = assign50170_e83461_d_n5;
        locals.var_t1_dn6 = assign50170_e83461_d_n6;
        locals.var_t1_dn7 = assign50170_e83461_d_n7;
        locals.var_t1_dn8 = assign50170_e83461_d_n8;
        locals.var_t1_dn9 = assign50170_e83461_d_n9;
        locals.var_t1_dn10 = assign50170_e83461_d_n10;
        locals.var_t1_dn11 = assign50170_e83461_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50180_e83476, assign50180_e83476_d_n3, assign50180_e83476_d_n4, assign50180_e83476_d_n5, assign50180_e83476_d_n6, assign50180_e83476_d_n7, assign50180_e83476_d_n8, assign50180_e83476_d_n9, assign50180_e83476_d_n10, assign50180_e83476_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50180_e83473: f64 = (locals.var_t1 + 0.001);
        let assign50180_e83474: f64 = (locals.var_bgidl_t / assign50180_e83473);
        (assign50180_e83474, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign50180_e83473 * assign50180_e83473))), (((locals.var_bgidl_t_dn4 * assign50180_e83473) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign50180_e83473 * assign50180_e83473)), (((locals.var_bgidl_t_dn5 * assign50180_e83473) - (locals.var_bgidl_t * locals.var_t1_dn5)) / (assign50180_e83473 * assign50180_e83473)), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign50180_e83473 * assign50180_e83473))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50180_e83476;
        locals.var_t2_dn3 = assign50180_e83476_d_n3;
        locals.var_t2_dn4 = assign50180_e83476_d_n4;
        locals.var_t2_dn5 = assign50180_e83476_d_n5;
        locals.var_t2_dn6 = assign50180_e83476_d_n6;
        locals.var_t2_dn7 = assign50180_e83476_d_n7;
        locals.var_t2_dn8 = assign50180_e83476_d_n8;
        locals.var_t2_dn9 = assign50180_e83476_d_n9;
        locals.var_t2_dn10 = assign50180_e83476_d_n10;
        locals.var_t2_dn11 = assign50180_e83476_d_n11;
        locals.var_t2_rv = 0.0;

        let assign50190_e83479: f64 = if locals.var_kgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard765 = assign50190_e83479;
        locals.var_guard765_rv = 0.0;

        let (assign50200_e83495, assign50200_e83495_d_n3, assign50200_e83495_d_n4, assign50200_e83495_d_n5, assign50200_e83495_d_n6, assign50200_e83495_d_n7, assign50200_e83495_d_n8, assign50200_e83495_d_n9, assign50200_e83495_d_n10, assign50200_e83495_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign50200_e83491: f64 = (-locals.var_vdb_noswap);
        let assign50200_e83493: f64 = (assign50200_e83491 - locals.var_fgidl_i);
        (assign50200_e83493, 0.0, 0.0, 0.0, (-locals.var_vdb_noswap_dn6), (-locals.var_vdb_noswap_dn7), 0.0, 0.0, (-locals.var_vdb_noswap_dn10), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50200_e83495;
        locals.var_t3_dn3 = assign50200_e83495_d_n3;
        locals.var_t3_dn4 = assign50200_e83495_d_n4;
        locals.var_t3_dn5 = assign50200_e83495_d_n5;
        locals.var_t3_dn6 = assign50200_e83495_d_n6;
        locals.var_t3_dn7 = assign50200_e83495_d_n7;
        locals.var_t3_dn8 = assign50200_e83495_d_n8;
        locals.var_t3_dn9 = assign50200_e83495_d_n9;
        locals.var_t3_dn10 = assign50200_e83495_d_n10;
        locals.var_t3_dn11 = assign50200_e83495_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50210_e83510, assign50210_e83510_d_n3, assign50210_e83510_d_n4, assign50210_e83510_d_n5, assign50210_e83510_d_n6, assign50210_e83510_d_n7, assign50210_e83510_d_n8, assign50210_e83510_d_n9, assign50210_e83510_d_n10, assign50210_e83510_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign50210_e83508: f64 = (locals.var_t3 + 0.0001);
        (assign50210_e83508, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50210_e83510;
        locals.var_t4_dn3 = assign50210_e83510_d_n3;
        locals.var_t4_dn4 = assign50210_e83510_d_n4;
        locals.var_t4_dn5 = assign50210_e83510_d_n5;
        locals.var_t4_dn6 = assign50210_e83510_d_n6;
        locals.var_t4_dn7 = assign50210_e83510_d_n7;
        locals.var_t4_dn8 = assign50210_e83510_d_n8;
        locals.var_t4_dn9 = assign50210_e83510_d_n9;
        locals.var_t4_dn10 = assign50210_e83510_d_n10;
        locals.var_t4_dn11 = assign50210_e83510_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign50220_e83544, assign50220_e83544_d_n3, assign50220_e83544_d_n4, assign50220_e83544_d_n5, assign50220_e83544_d_n6, assign50220_e83544_d_n7, assign50220_e83544_d_n8, assign50220_e83544_d_n9, assign50220_e83544_d_n10, assign50220_e83544_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign50220_e83524: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign50220_e83527: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign50220_e83530: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign50220_e83531: f64 = (assign50220_e83527 * assign50220_e83530);
        let assign50220_e83534: f64 = (4.0 * 1e-6);
        let assign50220_e83536: f64 = (assign50220_e83534 * 1e-6);
        let assign50220_e83537: f64 = (assign50220_e83531 + assign50220_e83536);
        let assign50220_e83538: f64 = (assign50220_e83537).sqrt();
        let assign50220_e83539: f64 = (assign50220_e83524 + assign50220_e83538);
        let assign50220_e83540: f64 = (0.5 * assign50220_e83539);
        let assign50220_e83542: f64 = (assign50220_e83540 - 1e-6);
        (assign50220_e83542, (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50220_e83544;
        locals.var_t5_dn3 = assign50220_e83544_d_n3;
        locals.var_t5_dn4 = assign50220_e83544_d_n4;
        locals.var_t5_dn5 = assign50220_e83544_d_n5;
        locals.var_t5_dn6 = assign50220_e83544_d_n6;
        locals.var_t5_dn7 = assign50220_e83544_d_n7;
        locals.var_t5_dn8 = assign50220_e83544_d_n8;
        locals.var_t5_dn9 = assign50220_e83544_d_n9;
        locals.var_t5_dn10 = assign50220_e83544_d_n10;
        locals.var_t5_dn11 = assign50220_e83544_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign50230_e83558, assign50230_e83558_d_n3, assign50230_e83558_d_n4, assign50230_e83558_d_n5, assign50230_e83558_d_n6, assign50230_e83558_d_n7, assign50230_e83558_d_n8, assign50230_e83558_d_n9, assign50230_e83558_d_n10, assign50230_e83558_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50230_e83558;
        locals.var_t5_dn3 = assign50230_e83558_d_n3;
        locals.var_t5_dn4 = assign50230_e83558_d_n4;
        locals.var_t5_dn5 = assign50230_e83558_d_n5;
        locals.var_t5_dn6 = assign50230_e83558_d_n6;
        locals.var_t5_dn7 = assign50230_e83558_d_n7;
        locals.var_t5_dn8 = assign50230_e83558_d_n8;
        locals.var_t5_dn9 = assign50230_e83558_d_n9;
        locals.var_t5_dn10 = assign50230_e83558_d_n10;
        locals.var_t5_dn11 = assign50230_e83558_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign50240_e83580, assign50240_e83580_d_n3, assign50240_e83580_d_n4, assign50240_e83580_d_n5, assign50240_e83580_d_n6, assign50240_e83580_d_n7, assign50240_e83580_d_n8, assign50240_e83580_d_n9, assign50240_e83580_d_n10, assign50240_e83580_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50240_e83569: f64 = (locals.var_agidl_i * locals.var_wdiod);
        let assign50240_e83571: f64 = (assign50240_e83569 * locals.var_t1);
        let assign50240_e83573: f64 = (-locals.var_t2);
        let assign50240_e83574: f64 = { let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50240_e83575: f64 = (assign50240_e83571 * assign50240_e83574);
        let assign50240_e83577: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50240_e83578: f64 = (assign50240_e83575 * assign50240_e83577);
        (assign50240_e83578, (((((assign50240_e83569 * locals.var_t1_dn3) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3))), (((((assign50240_e83569 * locals.var_t1_dn4) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4))), (((((assign50240_e83569 * locals.var_t1_dn5) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5))), (((((assign50240_e83569 * locals.var_t1_dn6) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6))), (((((assign50240_e83569 * locals.var_t1_dn7) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7))), (((((assign50240_e83569 * locals.var_t1_dn8) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8))), (((((assign50240_e83569 * locals.var_t1_dn9) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9))), (((((assign50240_e83569 * locals.var_t1_dn10) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10))), (((((assign50240_e83569 * locals.var_t1_dn11) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50240_e83580;
        locals.var_t6_dn3 = assign50240_e83580_d_n3;
        locals.var_t6_dn4 = assign50240_e83580_d_n4;
        locals.var_t6_dn5 = assign50240_e83580_d_n5;
        locals.var_t6_dn6 = assign50240_e83580_d_n6;
        locals.var_t6_dn7 = assign50240_e83580_d_n7;
        locals.var_t6_dn8 = assign50240_e83580_d_n8;
        locals.var_t6_dn9 = assign50240_e83580_d_n9;
        locals.var_t6_dn10 = assign50240_e83580_d_n10;
        locals.var_t6_dn11 = assign50240_e83580_d_n11;
        locals.var_t6_rv = 0.0;

        let assign50260_e83595: f64 = if ((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard766 = assign50260_e83595;
        locals.var_guard766_rv = 0.0;

        let (assign50270_e83605, assign50270_e83605_d_n3, assign50270_e83605_d_n4, assign50270_e83605_d_n5, assign50270_e83605_d_n6, assign50270_e83605_d_n7, assign50270_e83605_d_n8, assign50270_e83605_d_n9, assign50270_e83605_d_n10, assign50270_e83605_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50270_e83605;
        locals.var_t6_dn3 = assign50270_e83605_d_n3;
        locals.var_t6_dn4 = assign50270_e83605_d_n4;
        locals.var_t6_dn5 = assign50270_e83605_d_n5;
        locals.var_t6_dn6 = assign50270_e83605_d_n6;
        locals.var_t6_dn7 = assign50270_e83605_d_n7;
        locals.var_t6_dn8 = assign50270_e83605_d_n8;
        locals.var_t6_dn9 = assign50270_e83605_d_n9;
        locals.var_t6_dn10 = assign50270_e83605_d_n10;
        locals.var_t6_dn11 = assign50270_e83605_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign50280_e83623, assign50280_e83623_d_n3, assign50280_e83623_d_n4, assign50280_e83623_d_n5, assign50280_e83623_d_n6, assign50280_e83623_d_n7, assign50280_e83623_d_n8, assign50280_e83623_d_n9, assign50280_e83623_d_n10, assign50280_e83623_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50280_e83615: f64 = (-locals.var_vgs_noswap_1);
        let assign50280_e83617: f64 = (assign50280_e83615 - locals.var_egisl_i);
        let assign50280_e83619: f64 = (assign50280_e83617 + locals.var_vfbsdr);
        let assign50280_e83621: f64 = (assign50280_e83619 / locals.var_t0);
        (assign50280_e83621, (-((assign50280_e83619 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn6) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn7) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn8) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign50280_e83619 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_1_dn10) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign50280_e83619 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50280_e83623;
        locals.var_t1_dn3 = assign50280_e83623_d_n3;
        locals.var_t1_dn4 = assign50280_e83623_d_n4;
        locals.var_t1_dn5 = assign50280_e83623_d_n5;
        locals.var_t1_dn6 = assign50280_e83623_d_n6;
        locals.var_t1_dn7 = assign50280_e83623_d_n7;
        locals.var_t1_dn8 = assign50280_e83623_d_n8;
        locals.var_t1_dn9 = assign50280_e83623_d_n9;
        locals.var_t1_dn10 = assign50280_e83623_d_n10;
        locals.var_t1_dn11 = assign50280_e83623_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50290_e83647, assign50290_e83647_d_n3, assign50290_e83647_d_n4, assign50290_e83647_d_n5, assign50290_e83647_d_n6, assign50290_e83647_d_n7, assign50290_e83647_d_n8, assign50290_e83647_d_n9, assign50290_e83647_d_n10, assign50290_e83647_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50290_e83636: f64 = (locals.var_t1 * locals.var_t1);
        let assign50290_e83639: f64 = (4.0 * 0.01);
        let assign50290_e83641: f64 = (assign50290_e83639 * 0.01);
        let assign50290_e83642: f64 = (assign50290_e83636 + assign50290_e83641);
        let assign50290_e83643: f64 = (assign50290_e83642).sqrt();
        let assign50290_e83644: f64 = (locals.var_t1 + assign50290_e83643);
        let assign50290_e83645: f64 = (0.5 * assign50290_e83644);
        (assign50290_e83645, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign50290_e83643)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50290_e83647;
        locals.var_t1_dn3 = assign50290_e83647_d_n3;
        locals.var_t1_dn4 = assign50290_e83647_d_n4;
        locals.var_t1_dn5 = assign50290_e83647_d_n5;
        locals.var_t1_dn6 = assign50290_e83647_d_n6;
        locals.var_t1_dn7 = assign50290_e83647_d_n7;
        locals.var_t1_dn8 = assign50290_e83647_d_n8;
        locals.var_t1_dn9 = assign50290_e83647_d_n9;
        locals.var_t1_dn10 = assign50290_e83647_d_n10;
        locals.var_t1_dn11 = assign50290_e83647_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50300_e83662, assign50300_e83662_d_n3, assign50300_e83662_d_n4, assign50300_e83662_d_n5, assign50300_e83662_d_n6, assign50300_e83662_d_n7, assign50300_e83662_d_n8, assign50300_e83662_d_n9, assign50300_e83662_d_n10, assign50300_e83662_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50300_e83659: f64 = (locals.var_t1 + 0.001);
        let assign50300_e83660: f64 = (locals.var_bgisl_t / assign50300_e83659);
        (assign50300_e83660, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign50300_e83659 * assign50300_e83659))), (((locals.var_bgisl_t_dn4 * assign50300_e83659) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign50300_e83659 * assign50300_e83659)), (((locals.var_bgisl_t_dn5 * assign50300_e83659) - (locals.var_bgisl_t * locals.var_t1_dn5)) / (assign50300_e83659 * assign50300_e83659)), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign50300_e83659 * assign50300_e83659))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50300_e83662;
        locals.var_t2_dn3 = assign50300_e83662_d_n3;
        locals.var_t2_dn4 = assign50300_e83662_d_n4;
        locals.var_t2_dn5 = assign50300_e83662_d_n5;
        locals.var_t2_dn6 = assign50300_e83662_d_n6;
        locals.var_t2_dn7 = assign50300_e83662_d_n7;
        locals.var_t2_dn8 = assign50300_e83662_d_n8;
        locals.var_t2_dn9 = assign50300_e83662_d_n9;
        locals.var_t2_dn10 = assign50300_e83662_d_n10;
        locals.var_t2_dn11 = assign50300_e83662_d_n11;
        locals.var_t2_rv = 0.0;

        let assign50310_e83665: f64 = if locals.var_kgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard767 = assign50310_e83665;
        locals.var_guard767_rv = 0.0;

        let (assign50320_e83681, assign50320_e83681_d_n3, assign50320_e83681_d_n4, assign50320_e83681_d_n5, assign50320_e83681_d_n6, assign50320_e83681_d_n7, assign50320_e83681_d_n8, assign50320_e83681_d_n9, assign50320_e83681_d_n10, assign50320_e83681_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 != 0.0)) {
        let assign50320_e83677: f64 = (-locals.var_vsb_noswap);
        let assign50320_e83679: f64 = (assign50320_e83677 - locals.var_fgisl_i);
        (assign50320_e83679, 0.0, 0.0, 0.0, (-locals.var_vsb_noswap_dn6), (-locals.var_vsb_noswap_dn7), 0.0, 0.0, (-locals.var_vsb_noswap_dn10), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50320_e83681;
        locals.var_t3_dn3 = assign50320_e83681_d_n3;
        locals.var_t3_dn4 = assign50320_e83681_d_n4;
        locals.var_t3_dn5 = assign50320_e83681_d_n5;
        locals.var_t3_dn6 = assign50320_e83681_d_n6;
        locals.var_t3_dn7 = assign50320_e83681_d_n7;
        locals.var_t3_dn8 = assign50320_e83681_d_n8;
        locals.var_t3_dn9 = assign50320_e83681_d_n9;
        locals.var_t3_dn10 = assign50320_e83681_d_n10;
        locals.var_t3_dn11 = assign50320_e83681_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50330_e83696, assign50330_e83696_d_n3, assign50330_e83696_d_n4, assign50330_e83696_d_n5, assign50330_e83696_d_n6, assign50330_e83696_d_n7, assign50330_e83696_d_n8, assign50330_e83696_d_n9, assign50330_e83696_d_n10, assign50330_e83696_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 != 0.0)) {
        let assign50330_e83694: f64 = (locals.var_t3 + 0.0001);
        (assign50330_e83694, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50330_e83696;
        locals.var_t4_dn3 = assign50330_e83696_d_n3;
        locals.var_t4_dn4 = assign50330_e83696_d_n4;
        locals.var_t4_dn5 = assign50330_e83696_d_n5;
        locals.var_t4_dn6 = assign50330_e83696_d_n6;
        locals.var_t4_dn7 = assign50330_e83696_d_n7;
        locals.var_t4_dn8 = assign50330_e83696_d_n8;
        locals.var_t4_dn9 = assign50330_e83696_d_n9;
        locals.var_t4_dn10 = assign50330_e83696_d_n10;
        locals.var_t4_dn11 = assign50330_e83696_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign50340_e83730, assign50340_e83730_d_n3, assign50340_e83730_d_n4, assign50340_e83730_d_n5, assign50340_e83730_d_n6, assign50340_e83730_d_n7, assign50340_e83730_d_n8, assign50340_e83730_d_n9, assign50340_e83730_d_n10, assign50340_e83730_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4;
        let assign50340_e83710: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign50340_e83713: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign50340_e83716: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign50340_e83717: f64 = (assign50340_e83713 * assign50340_e83716);
        let assign50340_e83720: f64 = (4.0 * 1e-6);
        let assign50340_e83722: f64 = (assign50340_e83720 * 1e-6);
        let assign50340_e83723: f64 = (assign50340_e83717 + assign50340_e83722);
        let assign50340_e83724: f64 = (assign50340_e83723).sqrt();
        let assign50340_e83725: f64 = (assign50340_e83710 + assign50340_e83724);
        let assign50340_e83726: f64 = (0.5 * assign50340_e83725);
        let assign50340_e83728: f64 = (assign50340_e83726 - 1e-6);
        (assign50340_e83728, (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50340_e83730;
        locals.var_t5_dn3 = assign50340_e83730_d_n3;
        locals.var_t5_dn4 = assign50340_e83730_d_n4;
        locals.var_t5_dn5 = assign50340_e83730_d_n5;
        locals.var_t5_dn6 = assign50340_e83730_d_n6;
        locals.var_t5_dn7 = assign50340_e83730_d_n7;
        locals.var_t5_dn8 = assign50340_e83730_d_n8;
        locals.var_t5_dn9 = assign50340_e83730_d_n9;
        locals.var_t5_dn10 = assign50340_e83730_d_n10;
        locals.var_t5_dn11 = assign50340_e83730_d_n11;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_175(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50350_e83744, assign50350_e83744_d_n3, assign50350_e83744_d_n4, assign50350_e83744_d_n5, assign50350_e83744_d_n6, assign50350_e83744_d_n7, assign50350_e83744_d_n8, assign50350_e83744_d_n9, assign50350_e83744_d_n10, assign50350_e83744_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50350_e83744;
        locals.var_t5_dn3 = assign50350_e83744_d_n3;
        locals.var_t5_dn4 = assign50350_e83744_d_n4;
        locals.var_t5_dn5 = assign50350_e83744_d_n5;
        locals.var_t5_dn6 = assign50350_e83744_d_n6;
        locals.var_t5_dn7 = assign50350_e83744_d_n7;
        locals.var_t5_dn8 = assign50350_e83744_d_n8;
        locals.var_t5_dn9 = assign50350_e83744_d_n9;
        locals.var_t5_dn10 = assign50350_e83744_d_n10;
        locals.var_t5_dn11 = assign50350_e83744_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign50360_e83766, assign50360_e83766_d_n3, assign50360_e83766_d_n4, assign50360_e83766_d_n5, assign50360_e83766_d_n6, assign50360_e83766_d_n7, assign50360_e83766_d_n8, assign50360_e83766_d_n9, assign50360_e83766_d_n10, assign50360_e83766_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50360_e83755: f64 = (locals.var_agisl_i * locals.var_wdios);
        let assign50360_e83757: f64 = (assign50360_e83755 * locals.var_t1);
        let assign50360_e83759: f64 = (-locals.var_t2);
        let assign50360_e83760: f64 = { let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50360_e83761: f64 = (assign50360_e83757 * assign50360_e83760);
        let assign50360_e83763: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50360_e83764: f64 = (assign50360_e83761 * assign50360_e83763);
        (assign50360_e83764, (((((assign50360_e83755 * locals.var_t1_dn3) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3))), (((((assign50360_e83755 * locals.var_t1_dn4) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4))), (((((assign50360_e83755 * locals.var_t1_dn5) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5))), (((((assign50360_e83755 * locals.var_t1_dn6) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6))), (((((assign50360_e83755 * locals.var_t1_dn7) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7))), (((((assign50360_e83755 * locals.var_t1_dn8) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8))), (((((assign50360_e83755 * locals.var_t1_dn9) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9))), (((((assign50360_e83755 * locals.var_t1_dn10) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10))), (((((assign50360_e83755 * locals.var_t1_dn11) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50360_e83766;
        locals.var_t6_dn3 = assign50360_e83766_d_n3;
        locals.var_t6_dn4 = assign50360_e83766_d_n4;
        locals.var_t6_dn5 = assign50360_e83766_d_n5;
        locals.var_t6_dn6 = assign50360_e83766_d_n6;
        locals.var_t6_dn7 = assign50360_e83766_d_n7;
        locals.var_t6_dn8 = assign50360_e83766_d_n8;
        locals.var_t6_dn9 = assign50360_e83766_d_n9;
        locals.var_t6_dn10 = assign50360_e83766_d_n10;
        locals.var_t6_dn11 = assign50360_e83766_d_n11;
        locals.var_t6_rv = 0.0;

        let assign50400_e83795: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard768 = assign50400_e83795;
        locals.var_guard768_rv = 0.0;

        let assign50410_e83802: f64 = if ((locals.var_alpha0_i <= 0.0) || (locals.var_beta0_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard769 = assign50410_e83802;
        locals.var_guard769_rv = 0.0;

        let assign50430_e83815: f64 = (locals.var_beta0_t / 80.0);
        let assign50430_e83816: f64 = if locals.var_diffvds > assign50430_e83815 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign50430_e83816;
        locals.var_guard770_rv = 0.0;

        let (assign50440_e83831, assign50440_e83831_d_n3, assign50440_e83831_d_n4, assign50440_e83831_d_n5, assign50440_e83831_d_n6, assign50440_e83831_d_n7, assign50440_e83831_d_n8, assign50440_e83831_d_n9, assign50440_e83831_d_n10, assign50440_e83831_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 != 0.0)) && (locals.var_guard769 == 0.0)) && (locals.var_guard770 != 0.0)) {
        let assign50440_e83827: f64 = (-locals.var_beta0_t);
        let assign50440_e83829: f64 = (assign50440_e83827 / locals.var_diffvds);
        (assign50440_e83829, (-((assign50440_e83827 * locals.var_diffvds_dn3) / (locals.var_diffvds * locals.var_diffvds))), ((((-locals.var_beta0_t_dn4) * locals.var_diffvds) - (assign50440_e83827 * locals.var_diffvds_dn4)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_t_dn5) * locals.var_diffvds) - (assign50440_e83827 * locals.var_diffvds_dn5)) / (locals.var_diffvds * locals.var_diffvds)), (-((assign50440_e83827 * locals.var_diffvds_dn6) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn7) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn8) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn9) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn10) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn11) / (locals.var_diffvds * locals.var_diffvds))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50440_e83831;
        locals.var_t1_dn3 = assign50440_e83831_d_n3;
        locals.var_t1_dn4 = assign50440_e83831_d_n4;
        locals.var_t1_dn5 = assign50440_e83831_d_n5;
        locals.var_t1_dn6 = assign50440_e83831_d_n6;
        locals.var_t1_dn7 = assign50440_e83831_d_n7;
        locals.var_t1_dn8 = assign50440_e83831_d_n8;
        locals.var_t1_dn9 = assign50440_e83831_d_n9;
        locals.var_t1_dn10 = assign50440_e83831_d_n10;
        locals.var_t1_dn11 = assign50440_e83831_d_n11;
        locals.var_t1_rv = 0.0;

        let assign50470_e83876: f64 = if p.p44 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard771 = assign50470_e83876;
        locals.var_guard771_rv = 0.0;

        let assign50480_e83891: f64 = if ((locals.var_alpha0_i <= 0.0) || (((locals.var_beta2_i == 0.0) && (locals.var_beta1_i == 0.0)) && (locals.var_beta0_t == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard772 = assign50480_e83891;
        locals.var_guard772_rv = 0.0;

        let (assign50500_e83928, assign50500_e83928_d_n4, assign50500_e83928_d_n5,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50500_e83919: f64 = (locals.var_tratio - 1.0);
        let assign50500_e83920: f64 = (p.p600 * assign50500_e83919);
        let assign50500_e83921: f64 = (1.0 + assign50500_e83920);
        let assign50500_e83922: f64 = (locals.var_vdsatii0_i * assign50500_e83921);
        let assign50500_e83925: f64 = (locals.var_lii_i / locals.var_leff);
        let assign50500_e83926: f64 = (assign50500_e83922 - assign50500_e83925);
        (assign50500_e83926, (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn4)), (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5,)
    }
};
        locals.var_vdsatii0 = assign50500_e83928;
        locals.var_vdsatii0_dn4 = assign50500_e83928_d_n4;
        locals.var_vdsatii0_dn5 = assign50500_e83928_d_n5;
        locals.var_vdsatii0_rv = 0.0;

        let (assign50510_e83943, assign50510_e83943_d_n3, assign50510_e83943_d_n4, assign50510_e83943_d_n5, assign50510_e83943_d_n6, assign50510_e83943_d_n7, assign50510_e83943_d_n8, assign50510_e83943_d_n9, assign50510_e83943_d_n10, assign50510_e83943_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50510_e83941: f64 = (locals.var_esatii_i * locals.var_leff);
        (assign50510_e83941, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50510_e83943;
        locals.var_t0_dn3 = assign50510_e83943_d_n3;
        locals.var_t0_dn4 = assign50510_e83943_d_n4;
        locals.var_t0_dn5 = assign50510_e83943_d_n5;
        locals.var_t0_dn6 = assign50510_e83943_d_n6;
        locals.var_t0_dn7 = assign50510_e83943_d_n7;
        locals.var_t0_dn8 = assign50510_e83943_d_n8;
        locals.var_t0_dn9 = assign50510_e83943_d_n9;
        locals.var_t0_dn10 = assign50510_e83943_d_n10;
        locals.var_t0_dn11 = assign50510_e83943_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50520_e83962, assign50520_e83962_d_n3, assign50520_e83962_d_n4, assign50520_e83962_d_n5, assign50520_e83962_d_n6, assign50520_e83962_d_n7, assign50520_e83962_d_n8, assign50520_e83962_d_n9, assign50520_e83962_d_n10, assign50520_e83962_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50520_e83956: f64 = (locals.var_sii0_i * locals.var_t0);
        let assign50520_e83959: f64 = (1.0 + locals.var_t0);
        let assign50520_e83960: f64 = (assign50520_e83956 / assign50520_e83959);
        (assign50520_e83960, ((((locals.var_sii0_i * locals.var_t0_dn3) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn3)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn4) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn4)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn5) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn5)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn6) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn6)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn7) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn7)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn8) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn8)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn9) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn9)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn10) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn10)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn11) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn11)) / (assign50520_e83959 * assign50520_e83959)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50520_e83962;
        locals.var_t1_dn3 = assign50520_e83962_d_n3;
        locals.var_t1_dn4 = assign50520_e83962_d_n4;
        locals.var_t1_dn5 = assign50520_e83962_d_n5;
        locals.var_t1_dn6 = assign50520_e83962_d_n6;
        locals.var_t1_dn7 = assign50520_e83962_d_n7;
        locals.var_t1_dn8 = assign50520_e83962_d_n8;
        locals.var_t1_dn9 = assign50520_e83962_d_n9;
        locals.var_t1_dn10 = assign50520_e83962_d_n10;
        locals.var_t1_dn11 = assign50520_e83962_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50530_e84004, assign50530_e84004_d_n3, assign50530_e84004_d_n4, assign50530_e84004_d_n5, assign50530_e84004_d_n6, assign50530_e84004_d_n7, assign50530_e84004_d_n8, assign50530_e84004_d_n9, assign50530_e84004_d_n10, assign50530_e84004_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50530_e83978: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50530_e83980: f64 = (assign50530_e83978 * locals.var_nvt);
        let assign50530_e83983: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50530_e83985: f64 = (assign50530_e83983 * locals.var_nvt);
        let assign50530_e83988: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50530_e83990: f64 = (assign50530_e83988 * locals.var_nvt);
        let assign50530_e83991: f64 = (assign50530_e83985 * assign50530_e83990);
        let assign50530_e83994: f64 = (4.0 * p.p643);
        let assign50530_e83996: f64 = (assign50530_e83994 * p.p643);
        let assign50530_e83997: f64 = (assign50530_e83991 + assign50530_e83996);
        let assign50530_e83998: f64 = (assign50530_e83997).sqrt();
        let assign50530_e83999: f64 = (assign50530_e83980 + assign50530_e83998);
        let assign50530_e84000: f64 = (0.5 * assign50530_e83999);
        let assign50530_e84001: f64 = (1.0 + assign50530_e84000);
        let assign50530_e84002: f64 = (1.0 / assign50530_e84001);
        (assign50530_e84002, (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn3)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn3)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn3)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn4)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn4)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn4)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn5)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn5)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn5)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn6)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn6)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn6)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn7)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn7)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn7)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn8)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn8)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn8)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn9)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn9)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn9)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn10)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn10)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn10)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn11)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn11)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn11)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50530_e84004;
        locals.var_t0_dn3 = assign50530_e84004_d_n3;
        locals.var_t0_dn4 = assign50530_e84004_d_n4;
        locals.var_t0_dn5 = assign50530_e84004_d_n5;
        locals.var_t0_dn6 = assign50530_e84004_d_n6;
        locals.var_t0_dn7 = assign50530_e84004_d_n7;
        locals.var_t0_dn8 = assign50530_e84004_d_n8;
        locals.var_t0_dn9 = assign50530_e84004_d_n9;
        locals.var_t0_dn10 = assign50530_e84004_d_n10;
        locals.var_t0_dn11 = assign50530_e84004_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50540_e84019, assign50540_e84019_d_n3, assign50540_e84019_d_n4, assign50540_e84019_d_n5, assign50540_e84019_d_n6, assign50540_e84019_d_n7, assign50540_e84019_d_n8, assign50540_e84019_d_n9, assign50540_e84019_d_n10, assign50540_e84019_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50540_e84017: f64 = (locals.var_t0 + locals.var_sii2_i);
        (assign50540_e84017, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50540_e84019;
        locals.var_t3_dn3 = assign50540_e84019_d_n3;
        locals.var_t3_dn4 = assign50540_e84019_d_n4;
        locals.var_t3_dn5 = assign50540_e84019_d_n5;
        locals.var_t3_dn6 = assign50540_e84019_d_n6;
        locals.var_t3_dn7 = assign50540_e84019_d_n7;
        locals.var_t3_dn8 = assign50540_e84019_d_n8;
        locals.var_t3_dn9 = assign50540_e84019_d_n9;
        locals.var_t3_dn10 = assign50540_e84019_d_n10;
        locals.var_t3_dn11 = assign50540_e84019_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50550_e84057, assign50550_e84057_d_n3, assign50550_e84057_d_n4, assign50550_e84057_d_n5, assign50550_e84057_d_n6, assign50550_e84057_d_n7, assign50550_e84057_d_n8, assign50550_e84057_d_n9, assign50550_e84057_d_n10, assign50550_e84057_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50550_e84033: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50550_e84035: f64 = (assign50550_e84033 * locals.var_t3);
        let assign50550_e84038: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50550_e84040: f64 = (assign50550_e84038 * locals.var_t3);
        let assign50550_e84043: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50550_e84045: f64 = (assign50550_e84043 * locals.var_t3);
        let assign50550_e84046: f64 = (assign50550_e84040 * assign50550_e84045);
        let assign50550_e84049: f64 = (4.0 * p.p644);
        let assign50550_e84051: f64 = (assign50550_e84049 * p.p644);
        let assign50550_e84052: f64 = (assign50550_e84046 + assign50550_e84051);
        let assign50550_e84053: f64 = (assign50550_e84052).sqrt();
        let assign50550_e84054: f64 = (assign50550_e84035 + assign50550_e84053);
        let assign50550_e84055: f64 = (0.5 * assign50550_e84054);
        (assign50550_e84055, (0.5 * (((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn3)) + (((((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn3)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn3)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn4)) + (((((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn4)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn4)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn5)) + (((((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn5)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn5)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn6)) + (((((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn6)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn6)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn7)) + (((((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn7)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn7)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn8)) + (((((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn8)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn8)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn9)) + (((((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn9)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn9)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn10)) + (((((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn10)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn10)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn11)) + (((((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn11)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn11)))) / (2.0 * assign50550_e84053)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50550_e84057;
        locals.var_t2_dn3 = assign50550_e84057_d_n3;
        locals.var_t2_dn4 = assign50550_e84057_d_n4;
        locals.var_t2_dn5 = assign50550_e84057_d_n5;
        locals.var_t2_dn6 = assign50550_e84057_d_n6;
        locals.var_t2_dn7 = assign50550_e84057_d_n7;
        locals.var_t2_dn8 = assign50550_e84057_d_n8;
        locals.var_t2_dn9 = assign50550_e84057_d_n9;
        locals.var_t2_dn10 = assign50550_e84057_d_n10;
        locals.var_t2_dn11 = assign50550_e84057_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign50560_e84076, assign50560_e84076_d_n3, assign50560_e84076_d_n4, assign50560_e84076_d_n5, assign50560_e84076_d_n6, assign50560_e84076_d_n7, assign50560_e84076_d_n8, assign50560_e84076_d_n9, assign50560_e84076_d_n10, assign50560_e84076_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50560_e84072: f64 = (locals.var_siid_i * locals.var_vdsx);
        let assign50560_e84073: f64 = (1.0 + assign50560_e84072);
        let assign50560_e84074: f64 = (1.0 / assign50560_e84073);
        (assign50560_e84074, (-((locals.var_siid_i * locals.var_vdsx_dn3) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn4) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn5) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn6) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn7) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn8) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn9) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn10) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn11) / (assign50560_e84073 * assign50560_e84073))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50560_e84076;
        locals.var_t3_dn3 = assign50560_e84076_d_n3;
        locals.var_t3_dn4 = assign50560_e84076_d_n4;
        locals.var_t3_dn5 = assign50560_e84076_d_n5;
        locals.var_t3_dn6 = assign50560_e84076_d_n6;
        locals.var_t3_dn7 = assign50560_e84076_d_n7;
        locals.var_t3_dn8 = assign50560_e84076_d_n8;
        locals.var_t3_dn9 = assign50560_e84076_d_n9;
        locals.var_t3_dn10 = assign50560_e84076_d_n10;
        locals.var_t3_dn11 = assign50560_e84076_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50570_e84093, assign50570_e84093_d_n3, assign50570_e84093_d_n4, assign50570_e84093_d_n5, assign50570_e84093_d_n6, assign50570_e84093_d_n7, assign50570_e84093_d_n8, assign50570_e84093_d_n9, assign50570_e84093_d_n10, assign50570_e84093_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50570_e84089: f64 = (locals.var_t1 * locals.var_t2);
        let assign50570_e84091: f64 = (assign50570_e84089 * locals.var_t3);
        (assign50570_e84091, ((((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn3)), ((((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn4)), ((((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn5)), ((((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn6)), ((((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn7)), ((((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn8)), ((((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn9)), ((((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn10)), ((((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn11)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    }
};
        locals.var_vgsstep = assign50570_e84093;
        locals.var_vgsstep_dn3 = assign50570_e84093_d_n3;
        locals.var_vgsstep_dn4 = assign50570_e84093_d_n4;
        locals.var_vgsstep_dn5 = assign50570_e84093_d_n5;
        locals.var_vgsstep_dn6 = assign50570_e84093_d_n6;
        locals.var_vgsstep_dn7 = assign50570_e84093_d_n7;
        locals.var_vgsstep_dn8 = assign50570_e84093_d_n8;
        locals.var_vgsstep_dn9 = assign50570_e84093_d_n9;
        locals.var_vgsstep_dn10 = assign50570_e84093_d_n10;
        locals.var_vgsstep_dn11 = assign50570_e84093_d_n11;
        locals.var_vgsstep_rv = 0.0;

        let (assign50580_e84108, assign50580_e84108_d_n3, assign50580_e84108_d_n4, assign50580_e84108_d_n5, assign50580_e84108_d_n6, assign50580_e84108_d_n7, assign50580_e84108_d_n8, assign50580_e84108_d_n9, assign50580_e84108_d_n10, assign50580_e84108_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50580_e84106: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign50580_e84106, locals.var_vgsstep_dn3, (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11,)
    }
};
        locals.var_vdsatii = assign50580_e84108;
        locals.var_vdsatii_dn3 = assign50580_e84108_d_n3;
        locals.var_vdsatii_dn4 = assign50580_e84108_d_n4;
        locals.var_vdsatii_dn5 = assign50580_e84108_d_n5;
        locals.var_vdsatii_dn6 = assign50580_e84108_d_n6;
        locals.var_vdsatii_dn7 = assign50580_e84108_d_n7;
        locals.var_vdsatii_dn8 = assign50580_e84108_d_n8;
        locals.var_vdsatii_dn9 = assign50580_e84108_d_n9;
        locals.var_vdsatii_dn10 = assign50580_e84108_d_n10;
        locals.var_vdsatii_dn11 = assign50580_e84108_d_n11;
        locals.var_vdsatii_rv = 0.0;

        let (assign50590_e84123, assign50590_e84123_d_n3, assign50590_e84123_d_n4, assign50590_e84123_d_n5, assign50590_e84123_d_n6, assign50590_e84123_d_n7, assign50590_e84123_d_n8, assign50590_e84123_d_n9, assign50590_e84123_d_n10, assign50590_e84123_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50590_e84121: f64 = (locals.var_vdsx - locals.var_vdsatii);
        (assign50590_e84121, (locals.var_vdsx_dn3 - locals.var_vdsatii_dn3), (locals.var_vdsx_dn4 - locals.var_vdsatii_dn4), (locals.var_vdsx_dn5 - locals.var_vdsatii_dn5), (locals.var_vdsx_dn6 - locals.var_vdsatii_dn6), (locals.var_vdsx_dn7 - locals.var_vdsatii_dn7), (locals.var_vdsx_dn8 - locals.var_vdsatii_dn8), (locals.var_vdsx_dn9 - locals.var_vdsatii_dn9), (locals.var_vdsx_dn10 - locals.var_vdsatii_dn10), (locals.var_vdsx_dn11 - locals.var_vdsatii_dn11),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11,)
    }
};
        locals.var_vdiff = assign50590_e84123;
        locals.var_vdiff_dn3 = assign50590_e84123_d_n3;
        locals.var_vdiff_dn4 = assign50590_e84123_d_n4;
        locals.var_vdiff_dn5 = assign50590_e84123_d_n5;
        locals.var_vdiff_dn6 = assign50590_e84123_d_n6;
        locals.var_vdiff_dn7 = assign50590_e84123_d_n7;
        locals.var_vdiff_dn8 = assign50590_e84123_d_n8;
        locals.var_vdiff_dn9 = assign50590_e84123_d_n9;
        locals.var_vdiff_dn10 = assign50590_e84123_d_n10;
        locals.var_vdiff_dn11 = assign50590_e84123_d_n11;
        locals.var_vdiff_rv = 0.0;

        let (assign50600_e84146, assign50600_e84146_d_n3, assign50600_e84146_d_n4, assign50600_e84146_d_n5, assign50600_e84146_d_n6, assign50600_e84146_d_n7, assign50600_e84146_d_n8, assign50600_e84146_d_n9, assign50600_e84146_d_n10, assign50600_e84146_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50600_e84137: f64 = (locals.var_beta1_i * locals.var_vdiff);
        let assign50600_e84138: f64 = (locals.var_beta2_i + assign50600_e84137);
        let assign50600_e84141: f64 = (locals.var_beta0_t * locals.var_vdiff);
        let assign50600_e84143: f64 = (assign50600_e84141 * locals.var_vdiff);
        let assign50600_e84144: f64 = (assign50600_e84138 + assign50600_e84143);
        (assign50600_e84144, ((locals.var_beta1_i * locals.var_vdiff_dn3) + (((locals.var_beta0_t * locals.var_vdiff_dn3) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn3))), ((locals.var_beta1_i * locals.var_vdiff_dn4) + ((((locals.var_beta0_t_dn4 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn4))), ((locals.var_beta1_i * locals.var_vdiff_dn5) + ((((locals.var_beta0_t_dn5 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn5))), ((locals.var_beta1_i * locals.var_vdiff_dn6) + (((locals.var_beta0_t * locals.var_vdiff_dn6) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn6))), ((locals.var_beta1_i * locals.var_vdiff_dn7) + (((locals.var_beta0_t * locals.var_vdiff_dn7) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn7))), ((locals.var_beta1_i * locals.var_vdiff_dn8) + (((locals.var_beta0_t * locals.var_vdiff_dn8) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn8))), ((locals.var_beta1_i * locals.var_vdiff_dn9) + (((locals.var_beta0_t * locals.var_vdiff_dn9) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn9))), ((locals.var_beta1_i * locals.var_vdiff_dn10) + (((locals.var_beta0_t * locals.var_vdiff_dn10) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn10))), ((locals.var_beta1_i * locals.var_vdiff_dn11) + (((locals.var_beta0_t * locals.var_vdiff_dn11) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn11))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50600_e84146;
        locals.var_t0_dn3 = assign50600_e84146_d_n3;
        locals.var_t0_dn4 = assign50600_e84146_d_n4;
        locals.var_t0_dn5 = assign50600_e84146_d_n5;
        locals.var_t0_dn6 = assign50600_e84146_d_n6;
        locals.var_t0_dn7 = assign50600_e84146_d_n7;
        locals.var_t0_dn8 = assign50600_e84146_d_n8;
        locals.var_t0_dn9 = assign50600_e84146_d_n9;
        locals.var_t0_dn10 = assign50600_e84146_d_n10;
        locals.var_t0_dn11 = assign50600_e84146_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50610_e84164, assign50610_e84164_d_n3, assign50610_e84164_d_n4, assign50610_e84164_d_n5, assign50610_e84164_d_n6, assign50610_e84164_d_n7, assign50610_e84164_d_n8, assign50610_e84164_d_n9, assign50610_e84164_d_n10, assign50610_e84164_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50610_e84159: f64 = (locals.var_t0 * locals.var_t0);
        let assign50610_e84161: f64 = (assign50610_e84159 + 1e-10);
        let assign50610_e84162: f64 = (assign50610_e84161).sqrt();
        (assign50610_e84162, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign50610_e84162)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50610_e84164;
        locals.var_t1_dn3 = assign50610_e84164_d_n3;
        locals.var_t1_dn4 = assign50610_e84164_d_n4;
        locals.var_t1_dn5 = assign50610_e84164_d_n5;
        locals.var_t1_dn6 = assign50610_e84164_d_n6;
        locals.var_t1_dn7 = assign50610_e84164_d_n7;
        locals.var_t1_dn8 = assign50610_e84164_d_n8;
        locals.var_t1_dn9 = assign50610_e84164_d_n9;
        locals.var_t1_dn10 = assign50610_e84164_d_n10;
        locals.var_t1_dn11 = assign50610_e84164_d_n11;
        locals.var_t1_rv = 0.0;

        let assign50640_e84264: f64 = if ((locals.var_alpha0_i <= 0.0) || (((locals.var_beta2_i == 0.0) && (locals.var_beta1_i == 0.0)) && (locals.var_beta0_t == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard773 = assign50640_e84264;
        locals.var_guard773_rv = 0.0;

        let (assign50660_e84303, assign50660_e84303_d_n4, assign50660_e84303_d_n5,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50660_e84294: f64 = (locals.var_tratio - 1.0);
        let assign50660_e84295: f64 = (p.p600 * assign50660_e84294);
        let assign50660_e84296: f64 = (1.0 + assign50660_e84295);
        let assign50660_e84297: f64 = (locals.var_vdsatii0_i * assign50660_e84296);
        let assign50660_e84300: f64 = (locals.var_lii_i / locals.var_leff);
        let assign50660_e84301: f64 = (assign50660_e84297 - assign50660_e84300);
        (assign50660_e84301, (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn4)), (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5,)
    }
};
        locals.var_vdsatii0 = assign50660_e84303;
        locals.var_vdsatii0_dn4 = assign50660_e84303_d_n4;
        locals.var_vdsatii0_dn5 = assign50660_e84303_d_n5;
        locals.var_vdsatii0_rv = 0.0;

        let (assign50670_e84319, assign50670_e84319_d_n3, assign50670_e84319_d_n4, assign50670_e84319_d_n5, assign50670_e84319_d_n6, assign50670_e84319_d_n7, assign50670_e84319_d_n8, assign50670_e84319_d_n9, assign50670_e84319_d_n10, assign50670_e84319_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50670_e84317: f64 = (locals.var_esatii_i * locals.var_leff);
        (assign50670_e84317, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50670_e84319;
        locals.var_t0_dn3 = assign50670_e84319_d_n3;
        locals.var_t0_dn4 = assign50670_e84319_d_n4;
        locals.var_t0_dn5 = assign50670_e84319_d_n5;
        locals.var_t0_dn6 = assign50670_e84319_d_n6;
        locals.var_t0_dn7 = assign50670_e84319_d_n7;
        locals.var_t0_dn8 = assign50670_e84319_d_n8;
        locals.var_t0_dn9 = assign50670_e84319_d_n9;
        locals.var_t0_dn10 = assign50670_e84319_d_n10;
        locals.var_t0_dn11 = assign50670_e84319_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50680_e84339, assign50680_e84339_d_n3, assign50680_e84339_d_n4, assign50680_e84339_d_n5, assign50680_e84339_d_n6, assign50680_e84339_d_n7, assign50680_e84339_d_n8, assign50680_e84339_d_n9, assign50680_e84339_d_n10, assign50680_e84339_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50680_e84333: f64 = (locals.var_sii0_i * locals.var_t0);
        let assign50680_e84336: f64 = (1.0 + locals.var_t0);
        let assign50680_e84337: f64 = (assign50680_e84333 / assign50680_e84336);
        (assign50680_e84337, ((((locals.var_sii0_i * locals.var_t0_dn3) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn3)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn4) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn4)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn5) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn5)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn6) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn6)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn7) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn7)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn8) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn8)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn9) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn9)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn10) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn10)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn11) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn11)) / (assign50680_e84336 * assign50680_e84336)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50680_e84339;
        locals.var_t1_dn3 = assign50680_e84339_d_n3;
        locals.var_t1_dn4 = assign50680_e84339_d_n4;
        locals.var_t1_dn5 = assign50680_e84339_d_n5;
        locals.var_t1_dn6 = assign50680_e84339_d_n6;
        locals.var_t1_dn7 = assign50680_e84339_d_n7;
        locals.var_t1_dn8 = assign50680_e84339_d_n8;
        locals.var_t1_dn9 = assign50680_e84339_d_n9;
        locals.var_t1_dn10 = assign50680_e84339_d_n10;
        locals.var_t1_dn11 = assign50680_e84339_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50690_e84382, assign50690_e84382_d_n3, assign50690_e84382_d_n4, assign50690_e84382_d_n5, assign50690_e84382_d_n6, assign50690_e84382_d_n7, assign50690_e84382_d_n8, assign50690_e84382_d_n9, assign50690_e84382_d_n10, assign50690_e84382_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50690_e84356: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50690_e84358: f64 = (assign50690_e84356 * locals.var_nvt);
        let assign50690_e84361: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50690_e84363: f64 = (assign50690_e84361 * locals.var_nvt);
        let assign50690_e84366: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50690_e84368: f64 = (assign50690_e84366 * locals.var_nvt);
        let assign50690_e84369: f64 = (assign50690_e84363 * assign50690_e84368);
        let assign50690_e84372: f64 = (4.0 * p.p643);
        let assign50690_e84374: f64 = (assign50690_e84372 * p.p643);
        let assign50690_e84375: f64 = (assign50690_e84369 + assign50690_e84374);
        let assign50690_e84376: f64 = (assign50690_e84375).sqrt();
        let assign50690_e84377: f64 = (assign50690_e84358 + assign50690_e84376);
        let assign50690_e84378: f64 = (0.5 * assign50690_e84377);
        let assign50690_e84379: f64 = (1.0 + assign50690_e84378);
        let assign50690_e84380: f64 = (1.0 / assign50690_e84379);
        (assign50690_e84380, (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn3)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn3)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn3)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn4)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn4)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn4)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn5)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn5)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn5)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn6)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn6)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn6)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn7)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn7)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn7)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn8)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn8)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn8)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn9)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn9)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn9)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn10)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn10)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn10)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn11)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn11)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn11)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50690_e84382;
        locals.var_t0_dn3 = assign50690_e84382_d_n3;
        locals.var_t0_dn4 = assign50690_e84382_d_n4;
        locals.var_t0_dn5 = assign50690_e84382_d_n5;
        locals.var_t0_dn6 = assign50690_e84382_d_n6;
        locals.var_t0_dn7 = assign50690_e84382_d_n7;
        locals.var_t0_dn8 = assign50690_e84382_d_n8;
        locals.var_t0_dn9 = assign50690_e84382_d_n9;
        locals.var_t0_dn10 = assign50690_e84382_d_n10;
        locals.var_t0_dn11 = assign50690_e84382_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50700_e84398, assign50700_e84398_d_n3, assign50700_e84398_d_n4, assign50700_e84398_d_n5, assign50700_e84398_d_n6, assign50700_e84398_d_n7, assign50700_e84398_d_n8, assign50700_e84398_d_n9, assign50700_e84398_d_n10, assign50700_e84398_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50700_e84396: f64 = (locals.var_t0 + locals.var_sii2_i);
        (assign50700_e84396, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50700_e84398;
        locals.var_t3_dn3 = assign50700_e84398_d_n3;
        locals.var_t3_dn4 = assign50700_e84398_d_n4;
        locals.var_t3_dn5 = assign50700_e84398_d_n5;
        locals.var_t3_dn6 = assign50700_e84398_d_n6;
        locals.var_t3_dn7 = assign50700_e84398_d_n7;
        locals.var_t3_dn8 = assign50700_e84398_d_n8;
        locals.var_t3_dn9 = assign50700_e84398_d_n9;
        locals.var_t3_dn10 = assign50700_e84398_d_n10;
        locals.var_t3_dn11 = assign50700_e84398_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50710_e84437, assign50710_e84437_d_n3, assign50710_e84437_d_n4, assign50710_e84437_d_n5, assign50710_e84437_d_n6, assign50710_e84437_d_n7, assign50710_e84437_d_n8, assign50710_e84437_d_n9, assign50710_e84437_d_n10, assign50710_e84437_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50710_e84413: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50710_e84415: f64 = (assign50710_e84413 * locals.var_t3);
        let assign50710_e84418: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50710_e84420: f64 = (assign50710_e84418 * locals.var_t3);
        let assign50710_e84423: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50710_e84425: f64 = (assign50710_e84423 * locals.var_t3);
        let assign50710_e84426: f64 = (assign50710_e84420 * assign50710_e84425);
        let assign50710_e84429: f64 = (4.0 * p.p644);
        let assign50710_e84431: f64 = (assign50710_e84429 * p.p644);
        let assign50710_e84432: f64 = (assign50710_e84426 + assign50710_e84431);
        let assign50710_e84433: f64 = (assign50710_e84432).sqrt();
        let assign50710_e84434: f64 = (assign50710_e84415 + assign50710_e84433);
        let assign50710_e84435: f64 = (0.5 * assign50710_e84434);
        (assign50710_e84435, (0.5 * (((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn3)) + (((((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn3)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn3)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn4)) + (((((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn4)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn4)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn5)) + (((((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn5)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn5)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn6)) + (((((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn6)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn6)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn7)) + (((((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn7)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn7)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn8)) + (((((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn8)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn8)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn9)) + (((((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn9)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn9)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn10)) + (((((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn10)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn10)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn11)) + (((((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn11)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn11)))) / (2.0 * assign50710_e84433)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50710_e84437;
        locals.var_t2_dn3 = assign50710_e84437_d_n3;
        locals.var_t2_dn4 = assign50710_e84437_d_n4;
        locals.var_t2_dn5 = assign50710_e84437_d_n5;
        locals.var_t2_dn6 = assign50710_e84437_d_n6;
        locals.var_t2_dn7 = assign50710_e84437_d_n7;
        locals.var_t2_dn8 = assign50710_e84437_d_n8;
        locals.var_t2_dn9 = assign50710_e84437_d_n9;
        locals.var_t2_dn10 = assign50710_e84437_d_n10;
        locals.var_t2_dn11 = assign50710_e84437_d_n11;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_176(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50720_e84457, assign50720_e84457_d_n3, assign50720_e84457_d_n4, assign50720_e84457_d_n5, assign50720_e84457_d_n6, assign50720_e84457_d_n7, assign50720_e84457_d_n8, assign50720_e84457_d_n9, assign50720_e84457_d_n10, assign50720_e84457_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50720_e84453: f64 = (locals.var_siid_i * locals.var_vdsx);
        let assign50720_e84454: f64 = (1.0 + assign50720_e84453);
        let assign50720_e84455: f64 = (1.0 / assign50720_e84454);
        (assign50720_e84455, (-((locals.var_siid_i * locals.var_vdsx_dn3) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn4) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn5) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn6) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn7) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn8) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn9) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn10) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn11) / (assign50720_e84454 * assign50720_e84454))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50720_e84457;
        locals.var_t3_dn3 = assign50720_e84457_d_n3;
        locals.var_t3_dn4 = assign50720_e84457_d_n4;
        locals.var_t3_dn5 = assign50720_e84457_d_n5;
        locals.var_t3_dn6 = assign50720_e84457_d_n6;
        locals.var_t3_dn7 = assign50720_e84457_d_n7;
        locals.var_t3_dn8 = assign50720_e84457_d_n8;
        locals.var_t3_dn9 = assign50720_e84457_d_n9;
        locals.var_t3_dn10 = assign50720_e84457_d_n10;
        locals.var_t3_dn11 = assign50720_e84457_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50730_e84475, assign50730_e84475_d_n3, assign50730_e84475_d_n4, assign50730_e84475_d_n5, assign50730_e84475_d_n6, assign50730_e84475_d_n7, assign50730_e84475_d_n8, assign50730_e84475_d_n9, assign50730_e84475_d_n10, assign50730_e84475_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50730_e84471: f64 = (locals.var_t1 * locals.var_t2);
        let assign50730_e84473: f64 = (assign50730_e84471 * locals.var_t3);
        (assign50730_e84473, ((((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn3)), ((((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn4)), ((((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn5)), ((((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn6)), ((((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn7)), ((((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn8)), ((((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn9)), ((((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn10)), ((((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn11)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    }
};
        locals.var_vgsstep = assign50730_e84475;
        locals.var_vgsstep_dn3 = assign50730_e84475_d_n3;
        locals.var_vgsstep_dn4 = assign50730_e84475_d_n4;
        locals.var_vgsstep_dn5 = assign50730_e84475_d_n5;
        locals.var_vgsstep_dn6 = assign50730_e84475_d_n6;
        locals.var_vgsstep_dn7 = assign50730_e84475_d_n7;
        locals.var_vgsstep_dn8 = assign50730_e84475_d_n8;
        locals.var_vgsstep_dn9 = assign50730_e84475_d_n9;
        locals.var_vgsstep_dn10 = assign50730_e84475_d_n10;
        locals.var_vgsstep_dn11 = assign50730_e84475_d_n11;
        locals.var_vgsstep_rv = 0.0;

        let (assign50740_e84491, assign50740_e84491_d_n3, assign50740_e84491_d_n4, assign50740_e84491_d_n5, assign50740_e84491_d_n6, assign50740_e84491_d_n7, assign50740_e84491_d_n8, assign50740_e84491_d_n9, assign50740_e84491_d_n10, assign50740_e84491_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50740_e84489: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign50740_e84489, locals.var_vgsstep_dn3, (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11,)
    }
};
        locals.var_vdsatii = assign50740_e84491;
        locals.var_vdsatii_dn3 = assign50740_e84491_d_n3;
        locals.var_vdsatii_dn4 = assign50740_e84491_d_n4;
        locals.var_vdsatii_dn5 = assign50740_e84491_d_n5;
        locals.var_vdsatii_dn6 = assign50740_e84491_d_n6;
        locals.var_vdsatii_dn7 = assign50740_e84491_d_n7;
        locals.var_vdsatii_dn8 = assign50740_e84491_d_n8;
        locals.var_vdsatii_dn9 = assign50740_e84491_d_n9;
        locals.var_vdsatii_dn10 = assign50740_e84491_d_n10;
        locals.var_vdsatii_dn11 = assign50740_e84491_d_n11;
        locals.var_vdsatii_rv = 0.0;

        let (assign50750_e84507, assign50750_e84507_d_n3, assign50750_e84507_d_n4, assign50750_e84507_d_n5, assign50750_e84507_d_n6, assign50750_e84507_d_n7, assign50750_e84507_d_n8, assign50750_e84507_d_n9, assign50750_e84507_d_n10, assign50750_e84507_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50750_e84505: f64 = (locals.var_vdsx - locals.var_vdsatii);
        (assign50750_e84505, (locals.var_vdsx_dn3 - locals.var_vdsatii_dn3), (locals.var_vdsx_dn4 - locals.var_vdsatii_dn4), (locals.var_vdsx_dn5 - locals.var_vdsatii_dn5), (locals.var_vdsx_dn6 - locals.var_vdsatii_dn6), (locals.var_vdsx_dn7 - locals.var_vdsatii_dn7), (locals.var_vdsx_dn8 - locals.var_vdsatii_dn8), (locals.var_vdsx_dn9 - locals.var_vdsatii_dn9), (locals.var_vdsx_dn10 - locals.var_vdsatii_dn10), (locals.var_vdsx_dn11 - locals.var_vdsatii_dn11),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11,)
    }
};
        locals.var_vdiff = assign50750_e84507;
        locals.var_vdiff_dn3 = assign50750_e84507_d_n3;
        locals.var_vdiff_dn4 = assign50750_e84507_d_n4;
        locals.var_vdiff_dn5 = assign50750_e84507_d_n5;
        locals.var_vdiff_dn6 = assign50750_e84507_d_n6;
        locals.var_vdiff_dn7 = assign50750_e84507_d_n7;
        locals.var_vdiff_dn8 = assign50750_e84507_d_n8;
        locals.var_vdiff_dn9 = assign50750_e84507_d_n9;
        locals.var_vdiff_dn10 = assign50750_e84507_d_n10;
        locals.var_vdiff_dn11 = assign50750_e84507_d_n11;
        locals.var_vdiff_rv = 0.0;

        let (assign50760_e84531, assign50760_e84531_d_n3, assign50760_e84531_d_n4, assign50760_e84531_d_n5, assign50760_e84531_d_n6, assign50760_e84531_d_n7, assign50760_e84531_d_n8, assign50760_e84531_d_n9, assign50760_e84531_d_n10, assign50760_e84531_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50760_e84522: f64 = (locals.var_beta1_i * locals.var_vdiff);
        let assign50760_e84523: f64 = (locals.var_beta2_i + assign50760_e84522);
        let assign50760_e84526: f64 = (locals.var_beta0_t * locals.var_vdiff);
        let assign50760_e84528: f64 = (assign50760_e84526 * locals.var_vdiff);
        let assign50760_e84529: f64 = (assign50760_e84523 + assign50760_e84528);
        (assign50760_e84529, ((locals.var_beta1_i * locals.var_vdiff_dn3) + (((locals.var_beta0_t * locals.var_vdiff_dn3) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn3))), ((locals.var_beta1_i * locals.var_vdiff_dn4) + ((((locals.var_beta0_t_dn4 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn4))), ((locals.var_beta1_i * locals.var_vdiff_dn5) + ((((locals.var_beta0_t_dn5 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn5))), ((locals.var_beta1_i * locals.var_vdiff_dn6) + (((locals.var_beta0_t * locals.var_vdiff_dn6) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn6))), ((locals.var_beta1_i * locals.var_vdiff_dn7) + (((locals.var_beta0_t * locals.var_vdiff_dn7) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn7))), ((locals.var_beta1_i * locals.var_vdiff_dn8) + (((locals.var_beta0_t * locals.var_vdiff_dn8) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn8))), ((locals.var_beta1_i * locals.var_vdiff_dn9) + (((locals.var_beta0_t * locals.var_vdiff_dn9) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn9))), ((locals.var_beta1_i * locals.var_vdiff_dn10) + (((locals.var_beta0_t * locals.var_vdiff_dn10) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn10))), ((locals.var_beta1_i * locals.var_vdiff_dn11) + (((locals.var_beta0_t * locals.var_vdiff_dn11) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn11))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50760_e84531;
        locals.var_t0_dn3 = assign50760_e84531_d_n3;
        locals.var_t0_dn4 = assign50760_e84531_d_n4;
        locals.var_t0_dn5 = assign50760_e84531_d_n5;
        locals.var_t0_dn6 = assign50760_e84531_d_n6;
        locals.var_t0_dn7 = assign50760_e84531_d_n7;
        locals.var_t0_dn8 = assign50760_e84531_d_n8;
        locals.var_t0_dn9 = assign50760_e84531_d_n9;
        locals.var_t0_dn10 = assign50760_e84531_d_n10;
        locals.var_t0_dn11 = assign50760_e84531_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50770_e84550, assign50770_e84550_d_n3, assign50770_e84550_d_n4, assign50770_e84550_d_n5, assign50770_e84550_d_n6, assign50770_e84550_d_n7, assign50770_e84550_d_n8, assign50770_e84550_d_n9, assign50770_e84550_d_n10, assign50770_e84550_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50770_e84545: f64 = (locals.var_t0 * locals.var_t0);
        let assign50770_e84547: f64 = (assign50770_e84545 + 1e-10);
        let assign50770_e84548: f64 = (assign50770_e84547).sqrt();
        (assign50770_e84548, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign50770_e84548)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50770_e84550;
        locals.var_t1_dn3 = assign50770_e84550_d_n3;
        locals.var_t1_dn4 = assign50770_e84550_d_n4;
        locals.var_t1_dn5 = assign50770_e84550_d_n5;
        locals.var_t1_dn6 = assign50770_e84550_d_n6;
        locals.var_t1_dn7 = assign50770_e84550_d_n7;
        locals.var_t1_dn8 = assign50770_e84550_d_n8;
        locals.var_t1_dn9 = assign50770_e84550_d_n9;
        locals.var_t1_dn10 = assign50770_e84550_d_n10;
        locals.var_t1_dn11 = assign50770_e84550_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50800_e84648, assign50800_e84648_d_n3, assign50800_e84648_d_n4, assign50800_e84648_d_n5, assign50800_e84648_d_n6, assign50800_e84648_d_n7, assign50800_e84648_d_n8, assign50800_e84648_d_n9, assign50800_e84648_d_n10, assign50800_e84648_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50800_e84643: f64 = (locals.var_ebjtii_i * locals.var_leff);
        let assign50800_e84644: f64 = (locals.var_cbjtii_i + assign50800_e84643);
        let assign50800_e84646: f64 = (assign50800_e84644 / locals.var_leff);
        (assign50800_e84646, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50800_e84648;
        locals.var_t0_dn3 = assign50800_e84648_d_n3;
        locals.var_t0_dn4 = assign50800_e84648_d_n4;
        locals.var_t0_dn5 = assign50800_e84648_d_n5;
        locals.var_t0_dn6 = assign50800_e84648_d_n6;
        locals.var_t0_dn7 = assign50800_e84648_d_n7;
        locals.var_t0_dn8 = assign50800_e84648_d_n8;
        locals.var_t0_dn9 = assign50800_e84648_d_n9;
        locals.var_t0_dn10 = assign50800_e84648_d_n10;
        locals.var_t0_dn11 = assign50800_e84648_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50810_e84667, assign50810_e84667_d_n4, assign50810_e84667_d_n5,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50810_e84662: f64 = (locals.var_tratio - 1.0);
        let assign50810_e84663: f64 = (p.p666 * assign50810_e84662);
        let assign50810_e84664: f64 = (1.0 + assign50810_e84663);
        let assign50810_e84665: f64 = (locals.var_vbci_i * assign50810_e84664);
        (assign50810_e84665, (locals.var_vbci_i * (p.p666 * locals.var_tratio_dn4)), (locals.var_vbci_i * (p.p666 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vbc, locals.var_vbc_dn4, locals.var_vbc_dn5,)
    }
};
        locals.var_vbc = assign50810_e84667;
        locals.var_vbc_dn4 = assign50810_e84667_d_n4;
        locals.var_vbc_dn5 = assign50810_e84667_d_n5;
        locals.var_vbc_rv = 0.0;

        let assign50820_e84670: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard774 = assign50820_e84670;
        locals.var_guard774_rv = 0.0;

        let (assign50830_e84685, assign50830_e84685_d_n3, assign50830_e84685_d_n4, assign50830_e84685_d_n5, assign50830_e84685_d_n6, assign50830_e84685_d_n7, assign50830_e84685_d_n8, assign50830_e84685_d_n9, assign50830_e84685_d_n10, assign50830_e84685_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard774 != 0.0)) {
        let assign50830_e84683: f64 = (locals.var_vbc - locals.var_vbd_jct);
        (assign50830_e84683, 0.0, locals.var_vbc_dn4, locals.var_vbc_dn5, (-locals.var_vbd_jct_dn6), 0.0, 0.0, 0.0, (-locals.var_vbd_jct_dn10), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50830_e84685;
        locals.var_t1_dn3 = assign50830_e84685_d_n3;
        locals.var_t1_dn4 = assign50830_e84685_d_n4;
        locals.var_t1_dn5 = assign50830_e84685_d_n5;
        locals.var_t1_dn6 = assign50830_e84685_d_n6;
        locals.var_t1_dn7 = assign50830_e84685_d_n7;
        locals.var_t1_dn8 = assign50830_e84685_d_n8;
        locals.var_t1_dn9 = assign50830_e84685_d_n9;
        locals.var_t1_dn10 = assign50830_e84685_d_n10;
        locals.var_t1_dn11 = assign50830_e84685_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50840_e84701, assign50840_e84701_d_n3, assign50840_e84701_d_n4, assign50840_e84701_d_n5, assign50840_e84701_d_n6, assign50840_e84701_d_n7, assign50840_e84701_d_n8, assign50840_e84701_d_n9, assign50840_e84701_d_n10, assign50840_e84701_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign50840_e84699: f64 = (locals.var_vbc - locals.var_vbs_jct);
        (assign50840_e84699, 0.0, locals.var_vbc_dn4, locals.var_vbc_dn5, 0.0, (-locals.var_vbs_jct_dn7), 0.0, 0.0, (-locals.var_vbs_jct_dn10), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50840_e84701;
        locals.var_t1_dn3 = assign50840_e84701_d_n3;
        locals.var_t1_dn4 = assign50840_e84701_d_n4;
        locals.var_t1_dn5 = assign50840_e84701_d_n5;
        locals.var_t1_dn6 = assign50840_e84701_d_n6;
        locals.var_t1_dn7 = assign50840_e84701_d_n7;
        locals.var_t1_dn8 = assign50840_e84701_d_n8;
        locals.var_t1_dn9 = assign50840_e84701_d_n9;
        locals.var_t1_dn10 = assign50840_e84701_d_n10;
        locals.var_t1_dn11 = assign50840_e84701_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50850_e84714, assign50850_e84714_d_n3, assign50850_e84714_d_n4, assign50850_e84714_d_n5, assign50850_e84714_d_n6, assign50850_e84714_d_n7, assign50850_e84714_d_n8, assign50850_e84714_d_n9, assign50850_e84714_d_n10, assign50850_e84714_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50850_e84712: f64 = (locals.var_mbjtii_i - 1.0);
        (assign50850_e84712, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50850_e84714;
        locals.var_t2_dn3 = assign50850_e84714_d_n3;
        locals.var_t2_dn4 = assign50850_e84714_d_n4;
        locals.var_t2_dn5 = assign50850_e84714_d_n5;
        locals.var_t2_dn6 = assign50850_e84714_d_n6;
        locals.var_t2_dn7 = assign50850_e84714_d_n7;
        locals.var_t2_dn8 = assign50850_e84714_d_n8;
        locals.var_t2_dn9 = assign50850_e84714_d_n9;
        locals.var_t2_dn10 = assign50850_e84714_d_n10;
        locals.var_t2_dn11 = assign50850_e84714_d_n11;
        locals.var_t2_rv = 0.0;

        let assign50860_e84717: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard775 = assign50860_e84717;
        locals.var_guard775_rv = 0.0;

        let (assign50870_e84735, assign50870_e84735_d_n3, assign50870_e84735_d_n4, assign50870_e84735_d_n5, assign50870_e84735_d_n6, assign50870_e84735_d_n7, assign50870_e84735_d_n8, assign50870_e84735_d_n9, assign50870_e84735_d_n10, assign50870_e84735_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign50870_e84729: f64 = (-locals.var_abjtii_i);
        let assign50870_e84732: f64 = (locals.var_t1).powf(locals.var_t2);
        let assign50870_e84733: f64 = (assign50870_e84729 * assign50870_e84732);
        (assign50870_e84733, (assign50870_e84729 * if locals.var_t2_dn3 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn3)) } } else { (assign50870_e84732 * ((locals.var_t2_dn3 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn3 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn4 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn4)) } } else { (assign50870_e84732 * ((locals.var_t2_dn4 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn4 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn5 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn5)) } } else { (assign50870_e84732 * ((locals.var_t2_dn5 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn5 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn6 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn6)) } } else { (assign50870_e84732 * ((locals.var_t2_dn6 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn6 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn7 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn7)) } } else { (assign50870_e84732 * ((locals.var_t2_dn7 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn7 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn8 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn8)) } } else { (assign50870_e84732 * ((locals.var_t2_dn8 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn8 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn9 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn9)) } } else { (assign50870_e84732 * ((locals.var_t2_dn9 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn9 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn10 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn10)) } } else { (assign50870_e84732 * ((locals.var_t2_dn10 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn10 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn11 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn11)) } } else { (assign50870_e84732 * ((locals.var_t2_dn11 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn11 / locals.var_t1)))) }),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50870_e84735;
        locals.var_t3_dn3 = assign50870_e84735_d_n3;
        locals.var_t3_dn4 = assign50870_e84735_d_n4;
        locals.var_t3_dn5 = assign50870_e84735_d_n5;
        locals.var_t3_dn6 = assign50870_e84735_d_n6;
        locals.var_t3_dn7 = assign50870_e84735_d_n7;
        locals.var_t3_dn8 = assign50870_e84735_d_n8;
        locals.var_t3_dn9 = assign50870_e84735_d_n9;
        locals.var_t3_dn10 = assign50870_e84735_d_n10;
        locals.var_t3_dn11 = assign50870_e84735_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50880_e84749, assign50880_e84749_d_n3, assign50880_e84749_d_n4, assign50880_e84749_d_n5, assign50880_e84749_d_n6, assign50880_e84749_d_n7, assign50880_e84749_d_n8, assign50880_e84749_d_n9, assign50880_e84749_d_n10, assign50880_e84749_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard775 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50880_e84749;
        locals.var_t3_dn3 = assign50880_e84749_d_n3;
        locals.var_t3_dn4 = assign50880_e84749_d_n4;
        locals.var_t3_dn5 = assign50880_e84749_d_n5;
        locals.var_t3_dn6 = assign50880_e84749_d_n6;
        locals.var_t3_dn7 = assign50880_e84749_d_n7;
        locals.var_t3_dn8 = assign50880_e84749_d_n8;
        locals.var_t3_dn9 = assign50880_e84749_d_n9;
        locals.var_t3_dn10 = assign50880_e84749_d_n10;
        locals.var_t3_dn11 = assign50880_e84749_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50890_e84761, assign50890_e84761_d_n3, assign50890_e84761_d_n4, assign50890_e84761_d_n5, assign50890_e84761_d_n6, assign50890_e84761_d_n7, assign50890_e84761_d_n8, assign50890_e84761_d_n9, assign50890_e84761_d_n10, assign50890_e84761_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50890_e84759: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign50890_e84759, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50890_e84761;
        locals.var_t4_dn3 = assign50890_e84761_d_n3;
        locals.var_t4_dn4 = assign50890_e84761_d_n4;
        locals.var_t4_dn5 = assign50890_e84761_d_n5;
        locals.var_t4_dn6 = assign50890_e84761_d_n6;
        locals.var_t4_dn7 = assign50890_e84761_d_n7;
        locals.var_t4_dn8 = assign50890_e84761_d_n8;
        locals.var_t4_dn9 = assign50890_e84761_d_n9;
        locals.var_t4_dn10 = assign50890_e84761_d_n10;
        locals.var_t4_dn11 = assign50890_e84761_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign50930_e84811, assign50930_e84811_d_n4, assign50930_e84811_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50930_e84807: f64 = (locals.var_tratio - 1.0);
        let assign50930_e84808: f64 = (locals.var_aigc1_i * assign50930_e84807);
        let assign50930_e84809: f64 = (locals.var_aigc_i + assign50930_e84808);
        (assign50930_e84809, (locals.var_aigc_i_dn4 + (locals.var_aigc1_i * locals.var_tratio_dn4)), (locals.var_aigc_i_dn5 + (locals.var_aigc1_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigc_i, locals.var_aigc_i_dn4, locals.var_aigc_i_dn5,)
    }
};
        locals.var_aigc_i = assign50930_e84811;
        locals.var_aigc_i_dn4 = assign50930_e84811_d_n4;
        locals.var_aigc_i_dn5 = assign50930_e84811_d_n5;
        locals.var_aigc_i_rv = 0.0;

        let (assign50940_e84822, assign50940_e84822_d_n4, assign50940_e84822_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50940_e84818: f64 = (locals.var_tratio - 1.0);
        let assign50940_e84819: f64 = (locals.var_aigs1_i * assign50940_e84818);
        let assign50940_e84820: f64 = (locals.var_aigs_i + assign50940_e84819);
        (assign50940_e84820, (locals.var_aigs_i_dn4 + (locals.var_aigs1_i * locals.var_tratio_dn4)), (locals.var_aigs_i_dn5 + (locals.var_aigs1_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigs_i, locals.var_aigs_i_dn4, locals.var_aigs_i_dn5,)
    }
};
        locals.var_aigs_i = assign50940_e84822;
        locals.var_aigs_i_dn4 = assign50940_e84822_d_n4;
        locals.var_aigs_i_dn5 = assign50940_e84822_d_n5;
        locals.var_aigs_i_rv = 0.0;

        let (assign50950_e84833, assign50950_e84833_d_n4, assign50950_e84833_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50950_e84829: f64 = (locals.var_tratio - 1.0);
        let assign50950_e84830: f64 = (locals.var_aigd1_i * assign50950_e84829);
        let assign50950_e84831: f64 = (locals.var_aigd_i + assign50950_e84830);
        (assign50950_e84831, (locals.var_aigd_i_dn4 + (locals.var_aigd1_i * locals.var_tratio_dn4)), (locals.var_aigd_i_dn5 + (locals.var_aigd1_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigd_i, locals.var_aigd_i_dn4, locals.var_aigd_i_dn5,)
    }
};
        locals.var_aigd_i = assign50950_e84833;
        locals.var_aigd_i_dn4 = assign50950_e84833_d_n4;
        locals.var_aigd_i_dn5 = assign50950_e84833_d_n5;
        locals.var_aigd_i_rv = 0.0;

        let (assign50960_e84844, assign50960_e84844_d_n4, assign50960_e84844_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50960_e84840: f64 = (locals.var_tratio - 1.0);
        let assign50960_e84841: f64 = (locals.var_alphagb1_t_i * assign50960_e84840);
        let assign50960_e84842: f64 = (locals.var_alphagb1_i + assign50960_e84841);
        (assign50960_e84842, (locals.var_alphagb1_i_dn4 + (locals.var_alphagb1_t_i * locals.var_tratio_dn4)), (locals.var_alphagb1_i_dn5 + (locals.var_alphagb1_t_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_alphagb1_i, locals.var_alphagb1_i_dn4, locals.var_alphagb1_i_dn5,)
    }
};
        locals.var_alphagb1_i = assign50960_e84844;
        locals.var_alphagb1_i_dn4 = assign50960_e84844_d_n4;
        locals.var_alphagb1_i_dn5 = assign50960_e84844_d_n5;
        locals.var_alphagb1_i_rv = 0.0;

        let (assign50970_e84855, assign50970_e84855_d_n4, assign50970_e84855_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50970_e84851: f64 = (locals.var_tratio - 1.0);
        let assign50970_e84852: f64 = (locals.var_alphagb2_t_i * assign50970_e84851);
        let assign50970_e84853: f64 = (locals.var_alphagb2_i + assign50970_e84852);
        (assign50970_e84853, (locals.var_alphagb2_i_dn4 + (locals.var_alphagb2_t_i * locals.var_tratio_dn4)), (locals.var_alphagb2_i_dn5 + (locals.var_alphagb2_t_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_alphagb2_i, locals.var_alphagb2_i_dn4, locals.var_alphagb2_i_dn5,)
    }
};
        locals.var_alphagb2_i = assign50970_e84855;
        locals.var_alphagb2_i_dn4 = assign50970_e84855_d_n4;
        locals.var_alphagb2_i_dn5 = assign50970_e84855_d_n5;
        locals.var_alphagb2_i_rv = 0.0;

        let (assign50980_e84866, assign50980_e84866_d_n4, assign50980_e84866_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50980_e84862: f64 = (locals.var_tratio - 1.0);
        let assign50980_e84863: f64 = (locals.var_aigbcp2_t_i * assign50980_e84862);
        let assign50980_e84864: f64 = (locals.var_aigbcp2_i + assign50980_e84863);
        (assign50980_e84864, (locals.var_aigbcp2_i_dn4 + (locals.var_aigbcp2_t_i * locals.var_tratio_dn4)), (locals.var_aigbcp2_i_dn5 + (locals.var_aigbcp2_t_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigbcp2_i, locals.var_aigbcp2_i_dn4, locals.var_aigbcp2_i_dn5,)
    }
};
        locals.var_aigbcp2_i = assign50980_e84866;
        locals.var_aigbcp2_i_dn4 = assign50980_e84866_d_n4;
        locals.var_aigbcp2_i_dn5 = assign50980_e84866_d_n5;
        locals.var_aigbcp2_i_rv = 0.0;

        let assign51040_e84898: f64 = if ((p.p37 != 0.0) || (p.p38 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard776 = assign51040_e84898;
        locals.var_guard776_rv = 0.0;

        let (assign51050_e84913, assign51050_e84913_d_n3, assign51050_e84913_d_n4, assign51050_e84913_d_n5, assign51050_e84913_d_n6, assign51050_e84913_d_n7, assign51050_e84913_d_n8, assign51050_e84913_d_n9, assign51050_e84913_d_n10, assign51050_e84913_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51050_e84906: f64 = (locals.var_vgfb - locals.var_psip);
        let assign51050_e84908: f64 = (assign51050_e84906 + locals.var_qs_1);
        let assign51050_e84910: f64 = (assign51050_e84908 + locals.var_qdeff);
        let assign51050_e84911: f64 = (locals.var_nvt * assign51050_e84910);
        (assign51050_e84911, ((locals.var_nvt_dn3 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn3 - locals.var_psip_dn3) + locals.var_qs_1_dn3) + locals.var_qdeff_dn3))), ((locals.var_nvt_dn4 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn4 - locals.var_psip_dn4) + locals.var_qs_1_dn4) + locals.var_qdeff_dn4))), ((locals.var_nvt_dn5 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn5 - locals.var_psip_dn5) + locals.var_qs_1_dn5) + locals.var_qdeff_dn5))), ((locals.var_nvt_dn6 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn6 - locals.var_psip_dn6) + locals.var_qs_1_dn6) + locals.var_qdeff_dn6))), ((locals.var_nvt_dn7 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn7 - locals.var_psip_dn7) + locals.var_qs_1_dn7) + locals.var_qdeff_dn7))), ((locals.var_nvt_dn8 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn8 - locals.var_psip_dn8) + locals.var_qs_1_dn8) + locals.var_qdeff_dn8))), ((locals.var_nvt_dn9 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn9 - locals.var_psip_dn9) + locals.var_qs_1_dn9) + locals.var_qdeff_dn9))), ((locals.var_nvt_dn10 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn10 - locals.var_psip_dn10) + locals.var_qs_1_dn10) + locals.var_qdeff_dn10))), ((locals.var_nvt_dn11 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn11 - locals.var_psip_dn11) + locals.var_qs_1_dn11) + locals.var_qdeff_dn11))),)
    } else {
        (locals.var_voxm1, locals.var_voxm1_dn3, locals.var_voxm1_dn4, locals.var_voxm1_dn5, locals.var_voxm1_dn6, locals.var_voxm1_dn7, locals.var_voxm1_dn8, locals.var_voxm1_dn9, locals.var_voxm1_dn10, locals.var_voxm1_dn11,)
    }
};
        locals.var_voxm1 = assign51050_e84913;
        locals.var_voxm1_dn3 = assign51050_e84913_d_n3;
        locals.var_voxm1_dn4 = assign51050_e84913_d_n4;
        locals.var_voxm1_dn5 = assign51050_e84913_d_n5;
        locals.var_voxm1_dn6 = assign51050_e84913_d_n6;
        locals.var_voxm1_dn7 = assign51050_e84913_d_n7;
        locals.var_voxm1_dn8 = assign51050_e84913_d_n8;
        locals.var_voxm1_dn9 = assign51050_e84913_d_n9;
        locals.var_voxm1_dn10 = assign51050_e84913_d_n10;
        locals.var_voxm1_dn11 = assign51050_e84913_d_n11;
        locals.var_voxm1_rv = 0.0;

        let (assign51060_e84925, assign51060_e84925_d_n3, assign51060_e84925_d_n4, assign51060_e84925_d_n5, assign51060_e84925_d_n6, assign51060_e84925_d_n7, assign51060_e84925_d_n8, assign51060_e84925_d_n9, assign51060_e84925_d_n10, assign51060_e84925_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51060_e84920: f64 = (locals.var_voxm1 * locals.var_voxm1);
        let assign51060_e84922: f64 = (assign51060_e84920 + 0.0001);
        let assign51060_e84923: f64 = (assign51060_e84922).sqrt();
        (assign51060_e84923, (((locals.var_voxm1_dn3 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn3)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn4 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn4)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn5 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn5)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn6 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn6)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn7 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn7)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn8 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn8)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn9 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn9)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn10 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn10)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn11 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn11)) / (2.0 * assign51060_e84923)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51060_e84925;
        locals.var_t1_dn3 = assign51060_e84925_d_n3;
        locals.var_t1_dn4 = assign51060_e84925_d_n4;
        locals.var_t1_dn5 = assign51060_e84925_d_n5;
        locals.var_t1_dn6 = assign51060_e84925_d_n6;
        locals.var_t1_dn7 = assign51060_e84925_d_n7;
        locals.var_t1_dn8 = assign51060_e84925_d_n8;
        locals.var_t1_dn9 = assign51060_e84925_d_n9;
        locals.var_t1_dn10 = assign51060_e84925_d_n10;
        locals.var_t1_dn11 = assign51060_e84925_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51070_e84937, assign51070_e84937_d_n3, assign51070_e84937_d_n4, assign51070_e84937_d_n5, assign51070_e84937_d_n6, assign51070_e84937_d_n7, assign51070_e84937_d_n8, assign51070_e84937_d_n9, assign51070_e84937_d_n10, assign51070_e84937_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51070_e84932: f64 = (-locals.var_voxm1);
        let assign51070_e84934: f64 = (assign51070_e84932 + locals.var_t1);
        let assign51070_e84935: f64 = (0.5 * assign51070_e84934);
        (assign51070_e84935, (0.5 * ((-locals.var_voxm1_dn3) + locals.var_t1_dn3)), (0.5 * ((-locals.var_voxm1_dn4) + locals.var_t1_dn4)), (0.5 * ((-locals.var_voxm1_dn5) + locals.var_t1_dn5)), (0.5 * ((-locals.var_voxm1_dn6) + locals.var_t1_dn6)), (0.5 * ((-locals.var_voxm1_dn7) + locals.var_t1_dn7)), (0.5 * ((-locals.var_voxm1_dn8) + locals.var_t1_dn8)), (0.5 * ((-locals.var_voxm1_dn9) + locals.var_t1_dn9)), (0.5 * ((-locals.var_voxm1_dn10) + locals.var_t1_dn10)), (0.5 * ((-locals.var_voxm1_dn11) + locals.var_t1_dn11)),)
    } else {
        (locals.var_voxmacc, locals.var_voxmacc_dn3, locals.var_voxmacc_dn4, locals.var_voxmacc_dn5, locals.var_voxmacc_dn6, locals.var_voxmacc_dn7, locals.var_voxmacc_dn8, locals.var_voxmacc_dn9, locals.var_voxmacc_dn10, locals.var_voxmacc_dn11,)
    }
};
        locals.var_voxmacc = assign51070_e84937;
        locals.var_voxmacc_dn3 = assign51070_e84937_d_n3;
        locals.var_voxmacc_dn4 = assign51070_e84937_d_n4;
        locals.var_voxmacc_dn5 = assign51070_e84937_d_n5;
        locals.var_voxmacc_dn6 = assign51070_e84937_d_n6;
        locals.var_voxmacc_dn7 = assign51070_e84937_d_n7;
        locals.var_voxmacc_dn8 = assign51070_e84937_d_n8;
        locals.var_voxmacc_dn9 = assign51070_e84937_d_n9;
        locals.var_voxmacc_dn10 = assign51070_e84937_d_n10;
        locals.var_voxmacc_dn11 = assign51070_e84937_d_n11;
        locals.var_voxmacc_rv = 0.0;

        let (assign51080_e84948, assign51080_e84948_d_n3, assign51080_e84948_d_n4, assign51080_e84948_d_n5, assign51080_e84948_d_n6, assign51080_e84948_d_n7, assign51080_e84948_d_n8, assign51080_e84948_d_n9, assign51080_e84948_d_n10, assign51080_e84948_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51080_e84945: f64 = (locals.var_voxm1 + locals.var_t1);
        let assign51080_e84946: f64 = (0.5 * assign51080_e84945);
        (assign51080_e84946, (0.5 * (locals.var_voxm1_dn3 + locals.var_t1_dn3)), (0.5 * (locals.var_voxm1_dn4 + locals.var_t1_dn4)), (0.5 * (locals.var_voxm1_dn5 + locals.var_t1_dn5)), (0.5 * (locals.var_voxm1_dn6 + locals.var_t1_dn6)), (0.5 * (locals.var_voxm1_dn7 + locals.var_t1_dn7)), (0.5 * (locals.var_voxm1_dn8 + locals.var_t1_dn8)), (0.5 * (locals.var_voxm1_dn9 + locals.var_t1_dn9)), (0.5 * (locals.var_voxm1_dn10 + locals.var_t1_dn10)), (0.5 * (locals.var_voxm1_dn11 + locals.var_t1_dn11)),)
    } else {
        (locals.var_voxminv, locals.var_voxminv_dn3, locals.var_voxminv_dn4, locals.var_voxminv_dn5, locals.var_voxminv_dn6, locals.var_voxminv_dn7, locals.var_voxminv_dn8, locals.var_voxminv_dn9, locals.var_voxminv_dn10, locals.var_voxminv_dn11,)
    }
};
        locals.var_voxminv = assign51080_e84948;
        locals.var_voxminv_dn3 = assign51080_e84948_d_n3;
        locals.var_voxminv_dn4 = assign51080_e84948_d_n4;
        locals.var_voxminv_dn5 = assign51080_e84948_d_n5;
        locals.var_voxminv_dn6 = assign51080_e84948_d_n6;
        locals.var_voxminv_dn7 = assign51080_e84948_d_n7;
        locals.var_voxminv_dn8 = assign51080_e84948_d_n8;
        locals.var_voxminv_dn9 = assign51080_e84948_d_n9;
        locals.var_voxminv_dn10 = assign51080_e84948_d_n10;
        locals.var_voxminv_dn11 = assign51080_e84948_d_n11;
        locals.var_voxminv_rv = 0.0;

        let assign51090_e84951: f64 = if p.p38 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard777 = assign51090_e84951;
        locals.var_guard777_rv = 0.0;

        let (assign51100_e84962, assign51100_e84962_d_n3, assign51100_e84962_d_n4, assign51100_e84962_d_n5, assign51100_e84962_d_n6, assign51100_e84962_d_n7, assign51100_e84962_d_n8, assign51100_e84962_d_n9, assign51100_e84962_d_n10, assign51100_e84962_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51100_e84960: f64 = (locals.var_voxm1 / p.p671);
        (assign51100_e84960, (locals.var_voxm1_dn3 / p.p671), (locals.var_voxm1_dn4 / p.p671), (locals.var_voxm1_dn5 / p.p671), (locals.var_voxm1_dn6 / p.p671), (locals.var_voxm1_dn7 / p.p671), (locals.var_voxm1_dn8 / p.p671), (locals.var_voxm1_dn9 / p.p671), (locals.var_voxm1_dn10 / p.p671), (locals.var_voxm1_dn11 / p.p671),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51100_e84962;
        locals.var_t1_dn3 = assign51100_e84962_d_n3;
        locals.var_t1_dn4 = assign51100_e84962_d_n4;
        locals.var_t1_dn5 = assign51100_e84962_d_n5;
        locals.var_t1_dn6 = assign51100_e84962_d_n6;
        locals.var_t1_dn7 = assign51100_e84962_d_n7;
        locals.var_t1_dn8 = assign51100_e84962_d_n8;
        locals.var_t1_dn9 = assign51100_e84962_d_n9;
        locals.var_t1_dn10 = assign51100_e84962_d_n10;
        locals.var_t1_dn11 = assign51100_e84962_d_n11;
        locals.var_t1_rv = 0.0;

        let assign51120_e85017: f64 = if p.p696 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard778 = assign51120_e85017;
        locals.var_guard778_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_177(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (assign51130_e85032, assign51130_e85032_d_n3, assign51130_e85032_d_n4, assign51130_e85032_d_n5, assign51130_e85032_d_n6, assign51130_e85032_d_n7, assign51130_e85032_d_n8, assign51130_e85032_d_n9, assign51130_e85032_d_n10, assign51130_e85032_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) {
        let assign51130_e85029: f64 = (locals.var_voxmacc / p.p696);
        let assign51130_e85030: f64 = (1.0 - assign51130_e85029);
        (assign51130_e85030, (-(locals.var_voxmacc_dn3 / p.p696)), (-(locals.var_voxmacc_dn4 / p.p696)), (-(locals.var_voxmacc_dn5 / p.p696)), (-(locals.var_voxmacc_dn6 / p.p696)), (-(locals.var_voxmacc_dn7 / p.p696)), (-(locals.var_voxmacc_dn8 / p.p696)), (-(locals.var_voxmacc_dn9 / p.p696)), (-(locals.var_voxmacc_dn10 / p.p696)), (-(locals.var_voxmacc_dn11 / p.p696)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51130_e85032;
        locals.var_t0_dn3 = assign51130_e85032_d_n3;
        locals.var_t0_dn4 = assign51130_e85032_d_n4;
        locals.var_t0_dn5 = assign51130_e85032_d_n5;
        locals.var_t0_dn6 = assign51130_e85032_d_n6;
        locals.var_t0_dn7 = assign51130_e85032_d_n7;
        locals.var_t0_dn8 = assign51130_e85032_d_n8;
        locals.var_t0_dn9 = assign51130_e85032_d_n9;
        locals.var_t0_dn10 = assign51130_e85032_d_n10;
        locals.var_t0_dn11 = assign51130_e85032_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign51140_e85044, assign51140_e85044_d_n3, assign51140_e85044_d_n4, assign51140_e85044_d_n5, assign51140_e85044_d_n6, assign51140_e85044_d_n7, assign51140_e85044_d_n8, assign51140_e85044_d_n9, assign51140_e85044_d_n10, assign51140_e85044_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51140_e85044;
        locals.var_t0_dn3 = assign51140_e85044_d_n3;
        locals.var_t0_dn4 = assign51140_e85044_d_n4;
        locals.var_t0_dn5 = assign51140_e85044_d_n5;
        locals.var_t0_dn6 = assign51140_e85044_d_n6;
        locals.var_t0_dn7 = assign51140_e85044_d_n7;
        locals.var_t0_dn8 = assign51140_e85044_d_n8;
        locals.var_t0_dn9 = assign51140_e85044_d_n9;
        locals.var_t0_dn10 = assign51140_e85044_d_n10;
        locals.var_t0_dn11 = assign51140_e85044_d_n11;
        locals.var_t0_rv = 0.0;

        let assign51150_e85047: f64 = if locals.var_t0 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard779 = assign51150_e85047;
        locals.var_guard779_rv = 0.0;

        let (assign51160_e85058, assign51160_e85058_d_n3, assign51160_e85058_d_n4, assign51160_e85058_d_n5, assign51160_e85058_d_n6, assign51160_e85058_d_n7, assign51160_e85058_d_n8, assign51160_e85058_d_n9, assign51160_e85058_d_n10, assign51160_e85058_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard779 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51160_e85058;
        locals.var_t0_dn3 = assign51160_e85058_d_n3;
        locals.var_t0_dn4 = assign51160_e85058_d_n4;
        locals.var_t0_dn5 = assign51160_e85058_d_n5;
        locals.var_t0_dn6 = assign51160_e85058_d_n6;
        locals.var_t0_dn7 = assign51160_e85058_d_n7;
        locals.var_t0_dn8 = assign51160_e85058_d_n8;
        locals.var_t0_dn9 = assign51160_e85058_d_n9;
        locals.var_t0_dn10 = assign51160_e85058_d_n10;
        locals.var_t0_dn11 = assign51160_e85058_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign51170_e85079, assign51170_e85079_d_n3, assign51170_e85079_d_n4, assign51170_e85079_d_n5, assign51170_e85079_d_n6, assign51170_e85079_d_n7, assign51170_e85079_d_n8, assign51170_e85079_d_n9, assign51170_e85079_d_n10, assign51170_e85079_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51170_e85067: f64 = (locals.var_leff * locals.var_weff);
        let assign51170_e85069: f64 = (assign51170_e85067 / p.p1373);
        let assign51170_e85072: f64 = (p.p1381 / p.p2);
        let assign51170_e85073: f64 = (assign51170_e85069 + assign51170_e85072);
        let assign51170_e85075: f64 = (assign51170_e85073 * p.p700);
        let assign51170_e85077: f64 = (assign51170_e85075 * locals.var_toxratio);
        (assign51170_e85077, (assign51170_e85075 * locals.var_toxratio_dn3), (assign51170_e85075 * locals.var_toxratio_dn4), (assign51170_e85075 * locals.var_toxratio_dn5), (assign51170_e85075 * locals.var_toxratio_dn6), (assign51170_e85075 * locals.var_toxratio_dn7), (assign51170_e85075 * locals.var_toxratio_dn8), (assign51170_e85075 * locals.var_toxratio_dn9), (assign51170_e85075 * locals.var_toxratio_dn10), (assign51170_e85075 * locals.var_toxratio_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51170_e85079;
        locals.var_t1_dn3 = assign51170_e85079_d_n3;
        locals.var_t1_dn4 = assign51170_e85079_d_n4;
        locals.var_t1_dn5 = assign51170_e85079_d_n5;
        locals.var_t1_dn6 = assign51170_e85079_d_n6;
        locals.var_t1_dn7 = assign51170_e85079_d_n7;
        locals.var_t1_dn8 = assign51170_e85079_d_n8;
        locals.var_t1_dn9 = assign51170_e85079_d_n9;
        locals.var_t1_dn10 = assign51170_e85079_d_n10;
        locals.var_t1_dn11 = assign51170_e85079_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51180_e85090, assign51180_e85090_d_n3, assign51180_e85090_d_n4, assign51180_e85090_d_n5, assign51180_e85090_d_n6, assign51180_e85090_d_n7, assign51180_e85090_d_n8, assign51180_e85090_d_n9, assign51180_e85090_d_n10, assign51180_e85090_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51180_e85088: f64 = (p.p701 * p.p76);
        (assign51180_e85088, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51180_e85090;
        locals.var_t2_dn3 = assign51180_e85090_d_n3;
        locals.var_t2_dn4 = assign51180_e85090_d_n4;
        locals.var_t2_dn5 = assign51180_e85090_d_n5;
        locals.var_t2_dn6 = assign51180_e85090_d_n6;
        locals.var_t2_dn7 = assign51180_e85090_d_n7;
        locals.var_t2_dn8 = assign51180_e85090_d_n8;
        locals.var_t2_dn9 = assign51180_e85090_d_n9;
        locals.var_t2_dn10 = assign51180_e85090_d_n10;
        locals.var_t2_dn11 = assign51180_e85090_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51190_e85107, assign51190_e85107_d_n3, assign51190_e85107_d_n4, assign51190_e85107_d_n5, assign51190_e85107_d_n6, assign51190_e85107_d_n7, assign51190_e85107_d_n8, assign51190_e85107_d_n9, assign51190_e85107_d_n10, assign51190_e85107_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51190_e85101: f64 = (locals.var_betagb2_i * locals.var_voxmacc);
        let assign51190_e85102: f64 = (locals.var_alphagb2_i - assign51190_e85101);
        let assign51190_e85103: f64 = (locals.var_t2 * assign51190_e85102);
        let assign51190_e85105: f64 = (assign51190_e85103 / locals.var_t0);
        (assign51190_e85105, (((((locals.var_t2_dn3 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn3)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn4 * assign51190_e85102) + (locals.var_t2 * (locals.var_alphagb2_i_dn4 - (locals.var_betagb2_i * locals.var_voxmacc_dn4)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn5 * assign51190_e85102) + (locals.var_t2 * (locals.var_alphagb2_i_dn5 - (locals.var_betagb2_i * locals.var_voxmacc_dn5)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn6 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn6)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn7 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn7)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn8 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn8)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn9 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn9)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn10 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn10)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn11 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn11)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51190_e85107;
        locals.var_t3_dn3 = assign51190_e85107_d_n3;
        locals.var_t3_dn4 = assign51190_e85107_d_n4;
        locals.var_t3_dn5 = assign51190_e85107_d_n5;
        locals.var_t3_dn6 = assign51190_e85107_d_n6;
        locals.var_t3_dn7 = assign51190_e85107_d_n7;
        locals.var_t3_dn8 = assign51190_e85107_d_n8;
        locals.var_t3_dn9 = assign51190_e85107_d_n9;
        locals.var_t3_dn10 = assign51190_e85107_d_n10;
        locals.var_t3_dn11 = assign51190_e85107_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51200_e85117, assign51200_e85117_d_n3, assign51200_e85117_d_n4, assign51200_e85117_d_n5, assign51200_e85117_d_n6, assign51200_e85117_d_n7, assign51200_e85117_d_n8, assign51200_e85117_d_n9, assign51200_e85117_d_n10, assign51200_e85117_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51200_e85115: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51200_e85115, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51200_e85117;
        locals.var_t4_dn3 = assign51200_e85117_d_n3;
        locals.var_t4_dn4 = assign51200_e85117_d_n4;
        locals.var_t4_dn5 = assign51200_e85117_d_n5;
        locals.var_t4_dn6 = assign51200_e85117_d_n6;
        locals.var_t4_dn7 = assign51200_e85117_d_n7;
        locals.var_t4_dn8 = assign51200_e85117_d_n8;
        locals.var_t4_dn9 = assign51200_e85117_d_n9;
        locals.var_t4_dn10 = assign51200_e85117_d_n10;
        locals.var_t4_dn11 = assign51200_e85117_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51230_e85156, assign51230_e85156_d_n3, assign51230_e85156_d_n4, assign51230_e85156_d_n5, assign51230_e85156_d_n6, assign51230_e85156_d_n7, assign51230_e85156_d_n8, assign51230_e85156_d_n9, assign51230_e85156_d_n10, assign51230_e85156_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51230_e85152: f64 = (locals.var_voxm1 - locals.var_eigbinv_i);
        let assign51230_e85154: f64 = (assign51230_e85152 / p.p671);
        (assign51230_e85154, (locals.var_voxm1_dn3 / p.p671), (locals.var_voxm1_dn4 / p.p671), (locals.var_voxm1_dn5 / p.p671), (locals.var_voxm1_dn6 / p.p671), (locals.var_voxm1_dn7 / p.p671), (locals.var_voxm1_dn8 / p.p671), (locals.var_voxm1_dn9 / p.p671), (locals.var_voxm1_dn10 / p.p671), (locals.var_voxm1_dn11 / p.p671),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51230_e85156;
        locals.var_t1_dn3 = assign51230_e85156_d_n3;
        locals.var_t1_dn4 = assign51230_e85156_d_n4;
        locals.var_t1_dn5 = assign51230_e85156_d_n5;
        locals.var_t1_dn6 = assign51230_e85156_d_n6;
        locals.var_t1_dn7 = assign51230_e85156_d_n7;
        locals.var_t1_dn8 = assign51230_e85156_d_n8;
        locals.var_t1_dn9 = assign51230_e85156_d_n9;
        locals.var_t1_dn10 = assign51230_e85156_d_n10;
        locals.var_t1_dn11 = assign51230_e85156_d_n11;
        locals.var_t1_rv = 0.0;

        let assign51250_e85203: f64 = if p.p697 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard780 = assign51250_e85203;
        locals.var_guard780_rv = 0.0;

        let (assign51260_e85218, assign51260_e85218_d_n3, assign51260_e85218_d_n4, assign51260_e85218_d_n5, assign51260_e85218_d_n6, assign51260_e85218_d_n7, assign51260_e85218_d_n8, assign51260_e85218_d_n9, assign51260_e85218_d_n10, assign51260_e85218_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign51260_e85215: f64 = (locals.var_voxminv / p.p697);
        let assign51260_e85216: f64 = (1.0 - assign51260_e85215);
        (assign51260_e85216, (-(locals.var_voxminv_dn3 / p.p697)), (-(locals.var_voxminv_dn4 / p.p697)), (-(locals.var_voxminv_dn5 / p.p697)), (-(locals.var_voxminv_dn6 / p.p697)), (-(locals.var_voxminv_dn7 / p.p697)), (-(locals.var_voxminv_dn8 / p.p697)), (-(locals.var_voxminv_dn9 / p.p697)), (-(locals.var_voxminv_dn10 / p.p697)), (-(locals.var_voxminv_dn11 / p.p697)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51260_e85218;
        locals.var_t0_dn3 = assign51260_e85218_d_n3;
        locals.var_t0_dn4 = assign51260_e85218_d_n4;
        locals.var_t0_dn5 = assign51260_e85218_d_n5;
        locals.var_t0_dn6 = assign51260_e85218_d_n6;
        locals.var_t0_dn7 = assign51260_e85218_d_n7;
        locals.var_t0_dn8 = assign51260_e85218_d_n8;
        locals.var_t0_dn9 = assign51260_e85218_d_n9;
        locals.var_t0_dn10 = assign51260_e85218_d_n10;
        locals.var_t0_dn11 = assign51260_e85218_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign51270_e85230, assign51270_e85230_d_n3, assign51270_e85230_d_n4, assign51270_e85230_d_n5, assign51270_e85230_d_n6, assign51270_e85230_d_n7, assign51270_e85230_d_n8, assign51270_e85230_d_n9, assign51270_e85230_d_n10, assign51270_e85230_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard780 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51270_e85230;
        locals.var_t0_dn3 = assign51270_e85230_d_n3;
        locals.var_t0_dn4 = assign51270_e85230_d_n4;
        locals.var_t0_dn5 = assign51270_e85230_d_n5;
        locals.var_t0_dn6 = assign51270_e85230_d_n6;
        locals.var_t0_dn7 = assign51270_e85230_d_n7;
        locals.var_t0_dn8 = assign51270_e85230_d_n8;
        locals.var_t0_dn9 = assign51270_e85230_d_n9;
        locals.var_t0_dn10 = assign51270_e85230_d_n10;
        locals.var_t0_dn11 = assign51270_e85230_d_n11;
        locals.var_t0_rv = 0.0;

        let assign51280_e85233: f64 = if locals.var_t0 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard781 = assign51280_e85233;
        locals.var_guard781_rv = 0.0;

        let (assign51290_e85244, assign51290_e85244_d_n3, assign51290_e85244_d_n4, assign51290_e85244_d_n5, assign51290_e85244_d_n6, assign51290_e85244_d_n7, assign51290_e85244_d_n8, assign51290_e85244_d_n9, assign51290_e85244_d_n10, assign51290_e85244_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard781 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51290_e85244;
        locals.var_t0_dn3 = assign51290_e85244_d_n3;
        locals.var_t0_dn4 = assign51290_e85244_d_n4;
        locals.var_t0_dn5 = assign51290_e85244_d_n5;
        locals.var_t0_dn6 = assign51290_e85244_d_n6;
        locals.var_t0_dn7 = assign51290_e85244_d_n7;
        locals.var_t0_dn8 = assign51290_e85244_d_n8;
        locals.var_t0_dn9 = assign51290_e85244_d_n9;
        locals.var_t0_dn10 = assign51290_e85244_d_n10;
        locals.var_t0_dn11 = assign51290_e85244_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign51300_e85265, assign51300_e85265_d_n3, assign51300_e85265_d_n4, assign51300_e85265_d_n5, assign51300_e85265_d_n6, assign51300_e85265_d_n7, assign51300_e85265_d_n8, assign51300_e85265_d_n9, assign51300_e85265_d_n10, assign51300_e85265_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51300_e85253: f64 = (locals.var_leff * locals.var_weff);
        let assign51300_e85255: f64 = (assign51300_e85253 / p.p1373);
        let assign51300_e85258: f64 = (p.p1381 / p.p2);
        let assign51300_e85259: f64 = (assign51300_e85255 + assign51300_e85258);
        let assign51300_e85261: f64 = (assign51300_e85259 * p.p698);
        let assign51300_e85263: f64 = (assign51300_e85261 * locals.var_toxratio);
        (assign51300_e85263, (assign51300_e85261 * locals.var_toxratio_dn3), (assign51300_e85261 * locals.var_toxratio_dn4), (assign51300_e85261 * locals.var_toxratio_dn5), (assign51300_e85261 * locals.var_toxratio_dn6), (assign51300_e85261 * locals.var_toxratio_dn7), (assign51300_e85261 * locals.var_toxratio_dn8), (assign51300_e85261 * locals.var_toxratio_dn9), (assign51300_e85261 * locals.var_toxratio_dn10), (assign51300_e85261 * locals.var_toxratio_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51300_e85265;
        locals.var_t1_dn3 = assign51300_e85265_d_n3;
        locals.var_t1_dn4 = assign51300_e85265_d_n4;
        locals.var_t1_dn5 = assign51300_e85265_d_n5;
        locals.var_t1_dn6 = assign51300_e85265_d_n6;
        locals.var_t1_dn7 = assign51300_e85265_d_n7;
        locals.var_t1_dn8 = assign51300_e85265_d_n8;
        locals.var_t1_dn9 = assign51300_e85265_d_n9;
        locals.var_t1_dn10 = assign51300_e85265_d_n10;
        locals.var_t1_dn11 = assign51300_e85265_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51310_e85276, assign51310_e85276_d_n3, assign51310_e85276_d_n4, assign51310_e85276_d_n5, assign51310_e85276_d_n6, assign51310_e85276_d_n7, assign51310_e85276_d_n8, assign51310_e85276_d_n9, assign51310_e85276_d_n10, assign51310_e85276_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51310_e85274: f64 = (p.p699 * p.p76);
        (assign51310_e85274, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51310_e85276;
        locals.var_t2_dn3 = assign51310_e85276_d_n3;
        locals.var_t2_dn4 = assign51310_e85276_d_n4;
        locals.var_t2_dn5 = assign51310_e85276_d_n5;
        locals.var_t2_dn6 = assign51310_e85276_d_n6;
        locals.var_t2_dn7 = assign51310_e85276_d_n7;
        locals.var_t2_dn8 = assign51310_e85276_d_n8;
        locals.var_t2_dn9 = assign51310_e85276_d_n9;
        locals.var_t2_dn10 = assign51310_e85276_d_n10;
        locals.var_t2_dn11 = assign51310_e85276_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51320_e85293, assign51320_e85293_d_n3, assign51320_e85293_d_n4, assign51320_e85293_d_n5, assign51320_e85293_d_n6, assign51320_e85293_d_n7, assign51320_e85293_d_n8, assign51320_e85293_d_n9, assign51320_e85293_d_n10, assign51320_e85293_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51320_e85287: f64 = (locals.var_betagb1_i * locals.var_voxminv);
        let assign51320_e85288: f64 = (locals.var_alphagb1_i - assign51320_e85287);
        let assign51320_e85289: f64 = (locals.var_t2 * assign51320_e85288);
        let assign51320_e85291: f64 = (assign51320_e85289 / locals.var_t0);
        (assign51320_e85291, (((((locals.var_t2_dn3 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn3)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn4 * assign51320_e85288) + (locals.var_t2 * (locals.var_alphagb1_i_dn4 - (locals.var_betagb1_i * locals.var_voxminv_dn4)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn5 * assign51320_e85288) + (locals.var_t2 * (locals.var_alphagb1_i_dn5 - (locals.var_betagb1_i * locals.var_voxminv_dn5)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn6 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn6)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn7 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn7)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn8 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn8)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn9 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn9)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn10 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn10)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn11 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn11)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51320_e85293;
        locals.var_t3_dn3 = assign51320_e85293_d_n3;
        locals.var_t3_dn4 = assign51320_e85293_d_n4;
        locals.var_t3_dn5 = assign51320_e85293_d_n5;
        locals.var_t3_dn6 = assign51320_e85293_d_n6;
        locals.var_t3_dn7 = assign51320_e85293_d_n7;
        locals.var_t3_dn8 = assign51320_e85293_d_n8;
        locals.var_t3_dn9 = assign51320_e85293_d_n9;
        locals.var_t3_dn10 = assign51320_e85293_d_n10;
        locals.var_t3_dn11 = assign51320_e85293_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51330_e85303, assign51330_e85303_d_n3, assign51330_e85303_d_n4, assign51330_e85303_d_n5, assign51330_e85303_d_n6, assign51330_e85303_d_n7, assign51330_e85303_d_n8, assign51330_e85303_d_n9, assign51330_e85303_d_n10, assign51330_e85303_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51330_e85301: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51330_e85301, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51330_e85303;
        locals.var_t4_dn3 = assign51330_e85303_d_n3;
        locals.var_t4_dn4 = assign51330_e85303_d_n4;
        locals.var_t4_dn5 = assign51330_e85303_d_n5;
        locals.var_t4_dn6 = assign51330_e85303_d_n6;
        locals.var_t4_dn7 = assign51330_e85303_d_n7;
        locals.var_t4_dn8 = assign51330_e85303_d_n8;
        locals.var_t4_dn9 = assign51330_e85303_d_n9;
        locals.var_t4_dn10 = assign51330_e85303_d_n10;
        locals.var_t4_dn11 = assign51330_e85303_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51370_e85355, assign51370_e85355_d_n3, assign51370_e85355_d_n4, assign51370_e85355_d_n5, assign51370_e85355_d_n6, assign51370_e85355_d_n7, assign51370_e85355_d_n8, assign51370_e85355_d_n9, assign51370_e85355_d_n10, assign51370_e85355_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51370_e85351: f64 = (locals.var_vfb * locals.var_nvt);
        let assign51370_e85353: f64 = (assign51370_e85351 + p.p1383);
        (assign51370_e85353, ((locals.var_vfb_dn3 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn3)), ((locals.var_vfb_dn4 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn4)), ((locals.var_vfb_dn5 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn5)), ((locals.var_vfb_dn6 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn6)), ((locals.var_vfb_dn7 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn7)), ((locals.var_vfb_dn8 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn8)), ((locals.var_vfb_dn9 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn9)), ((locals.var_vfb_dn10 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn10)), ((locals.var_vfb_dn11 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn11)),)
    } else {
        (locals.var_vfb2, locals.var_vfb2_dn3, locals.var_vfb2_dn4, locals.var_vfb2_dn5, locals.var_vfb2_dn6, locals.var_vfb2_dn7, locals.var_vfb2_dn8, locals.var_vfb2_dn9, locals.var_vfb2_dn10, locals.var_vfb2_dn11,)
    }
};
        locals.var_vfb2 = assign51370_e85355;
        locals.var_vfb2_dn3 = assign51370_e85355_d_n3;
        locals.var_vfb2_dn4 = assign51370_e85355_d_n4;
        locals.var_vfb2_dn5 = assign51370_e85355_d_n5;
        locals.var_vfb2_dn6 = assign51370_e85355_d_n6;
        locals.var_vfb2_dn7 = assign51370_e85355_d_n7;
        locals.var_vfb2_dn8 = assign51370_e85355_d_n8;
        locals.var_vfb2_dn9 = assign51370_e85355_d_n9;
        locals.var_vfb2_dn10 = assign51370_e85355_d_n10;
        locals.var_vfb2_dn11 = assign51370_e85355_d_n11;
        locals.var_vfb2_rv = 0.0;

        let assign51380_e85378: f64 = if (((((p.p43 != 0.0) && true) && (!((p.p40 != 0.0) && (!true)))) && (p.p45 == 1.0)) && (p.p1380 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard782 = assign51380_e85378;
        locals.var_guard782_rv = 0.0;

        let (assign51390_e85389, assign51390_e85389_d_n8, assign51390_e85389_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51390_e85387: f64 = (locals.var_devsign * (nv8 - nv11));
        (assign51390_e85387, locals.var_devsign, (-locals.var_devsign),)
    } else {
        (locals.var_vgb, locals.var_vgb_dn8, locals.var_vgb_dn11,)
    }
};
        locals.var_vgb = assign51390_e85389;
        locals.var_vgb_dn8 = assign51390_e85389_d_n8;
        locals.var_vgb_dn11 = assign51390_e85389_d_n11;
        locals.var_vgb_rv = 0.0;

        let (assign51400_e85400, assign51400_e85400_d_n3, assign51400_e85400_d_n4, assign51400_e85400_d_n5, assign51400_e85400_d_n6, assign51400_e85400_d_n7, assign51400_e85400_d_n8, assign51400_e85400_d_n9, assign51400_e85400_d_n10, assign51400_e85400_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51400_e85398: f64 = (locals.var_vgb - locals.var_vfb2);
        (assign51400_e85398, (-locals.var_vfb2_dn3), (-locals.var_vfb2_dn4), (-locals.var_vfb2_dn5), (-locals.var_vfb2_dn6), (-locals.var_vfb2_dn7), (locals.var_vgb_dn8 - locals.var_vfb2_dn8), (-locals.var_vfb2_dn9), (-locals.var_vfb2_dn10), (locals.var_vgb_dn11 - locals.var_vfb2_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51400_e85400;
        locals.var_t0_dn3 = assign51400_e85400_d_n3;
        locals.var_t0_dn4 = assign51400_e85400_d_n4;
        locals.var_t0_dn5 = assign51400_e85400_d_n5;
        locals.var_t0_dn6 = assign51400_e85400_d_n6;
        locals.var_t0_dn7 = assign51400_e85400_d_n7;
        locals.var_t0_dn8 = assign51400_e85400_d_n8;
        locals.var_t0_dn9 = assign51400_e85400_d_n9;
        locals.var_t0_dn10 = assign51400_e85400_d_n10;
        locals.var_t0_dn11 = assign51400_e85400_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign51410_e85414, assign51410_e85414_d_n3, assign51410_e85414_d_n4, assign51410_e85414_d_n5, assign51410_e85414_d_n6, assign51410_e85414_d_n7, assign51410_e85414_d_n8, assign51410_e85414_d_n9, assign51410_e85414_d_n10, assign51410_e85414_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51410_e85409: f64 = (locals.var_t0 * locals.var_t0);
        let assign51410_e85411: f64 = (assign51410_e85409 + 0.0001);
        let assign51410_e85412: f64 = (assign51410_e85411).sqrt();
        (assign51410_e85412, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign51410_e85412)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51410_e85414;
        locals.var_t1_dn3 = assign51410_e85414_d_n3;
        locals.var_t1_dn4 = assign51410_e85414_d_n4;
        locals.var_t1_dn5 = assign51410_e85414_d_n5;
        locals.var_t1_dn6 = assign51410_e85414_d_n6;
        locals.var_t1_dn7 = assign51410_e85414_d_n7;
        locals.var_t1_dn8 = assign51410_e85414_d_n8;
        locals.var_t1_dn9 = assign51410_e85414_d_n9;
        locals.var_t1_dn10 = assign51410_e85414_d_n10;
        locals.var_t1_dn11 = assign51410_e85414_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51420_e85430, assign51420_e85430_d_n3, assign51420_e85430_d_n4, assign51420_e85430_d_n5, assign51420_e85430_d_n6, assign51420_e85430_d_n7, assign51420_e85430_d_n8, assign51420_e85430_d_n9, assign51420_e85430_d_n10, assign51420_e85430_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51420_e85423: f64 = (-locals.var_t0);
        let assign51420_e85425: f64 = (assign51420_e85423 + locals.var_t1);
        let assign51420_e85427: f64 = (assign51420_e85425 - 0.01);
        let assign51420_e85428: f64 = (0.5 * assign51420_e85427);
        (assign51420_e85428, (0.5 * ((-locals.var_t0_dn3) + locals.var_t1_dn3)), (0.5 * ((-locals.var_t0_dn4) + locals.var_t1_dn4)), (0.5 * ((-locals.var_t0_dn5) + locals.var_t1_dn5)), (0.5 * ((-locals.var_t0_dn6) + locals.var_t1_dn6)), (0.5 * ((-locals.var_t0_dn7) + locals.var_t1_dn7)), (0.5 * ((-locals.var_t0_dn8) + locals.var_t1_dn8)), (0.5 * ((-locals.var_t0_dn9) + locals.var_t1_dn9)), (0.5 * ((-locals.var_t0_dn10) + locals.var_t1_dn10)), (0.5 * ((-locals.var_t0_dn11) + locals.var_t1_dn11)),)
    } else {
        (locals.var_vgp_eff, locals.var_vgp_eff_dn3, locals.var_vgp_eff_dn4, locals.var_vgp_eff_dn5, locals.var_vgp_eff_dn6, locals.var_vgp_eff_dn7, locals.var_vgp_eff_dn8, locals.var_vgp_eff_dn9, locals.var_vgp_eff_dn10, locals.var_vgp_eff_dn11,)
    }
};
        locals.var_vgp_eff = assign51420_e85430;
        locals.var_vgp_eff_dn3 = assign51420_e85430_d_n3;
        locals.var_vgp_eff_dn4 = assign51420_e85430_d_n4;
        locals.var_vgp_eff_dn5 = assign51420_e85430_d_n5;
        locals.var_vgp_eff_dn6 = assign51420_e85430_d_n6;
        locals.var_vgp_eff_dn7 = assign51420_e85430_d_n7;
        locals.var_vgp_eff_dn8 = assign51420_e85430_d_n8;
        locals.var_vgp_eff_dn9 = assign51420_e85430_d_n9;
        locals.var_vgp_eff_dn10 = assign51420_e85430_d_n10;
        locals.var_vgp_eff_dn11 = assign51420_e85430_d_n11;
        locals.var_vgp_eff_rv = 0.0;

        let (assign51430_e85444, assign51430_e85444_d_n3, assign51430_e85444_d_n4, assign51430_e85444_d_n5, assign51430_e85444_d_n6, assign51430_e85444_d_n7, assign51430_e85444_d_n8, assign51430_e85444_d_n9, assign51430_e85444_d_n10, assign51430_e85444_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let (assign51430_e85442,) = {
            if (p.p30 == 1.0) {
                (p.p702,)
            } else {
                (p.p703,)
            }
        };
        (assign51430_e85442, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign51430_e85444;
        locals.var_t11_dn3 = assign51430_e85444_d_n3;
        locals.var_t11_dn4 = assign51430_e85444_d_n4;
        locals.var_t11_dn5 = assign51430_e85444_d_n5;
        locals.var_t11_dn6 = assign51430_e85444_d_n6;
        locals.var_t11_dn7 = assign51430_e85444_d_n7;
        locals.var_t11_dn8 = assign51430_e85444_d_n8;
        locals.var_t11_dn9 = assign51430_e85444_d_n9;
        locals.var_t11_dn10 = assign51430_e85444_d_n10;
        locals.var_t11_dn11 = assign51430_e85444_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign51440_e85458, assign51440_e85458_d_n3, assign51440_e85458_d_n4, assign51440_e85458_d_n5, assign51440_e85458_d_n6, assign51440_e85458_d_n7, assign51440_e85458_d_n8, assign51440_e85458_d_n9, assign51440_e85458_d_n10, assign51440_e85458_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let (assign51440_e85456,) = {
            if (p.p30 == 1.0) {
                (p.p704,)
            } else {
                (p.p705,)
            }
        };
        (assign51440_e85456, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign51440_e85458;
        locals.var_t12_dn3 = assign51440_e85458_d_n3;
        locals.var_t12_dn4 = assign51440_e85458_d_n4;
        locals.var_t12_dn5 = assign51440_e85458_d_n5;
        locals.var_t12_dn6 = assign51440_e85458_d_n6;
        locals.var_t12_dn7 = assign51440_e85458_d_n7;
        locals.var_t12_dn8 = assign51440_e85458_d_n8;
        locals.var_t12_dn9 = assign51440_e85458_d_n9;
        locals.var_t12_dn10 = assign51440_e85458_d_n10;
        locals.var_t12_dn11 = assign51440_e85458_d_n11;
        locals.var_t12_rv = 0.0;

        let (assign51450_e85469, assign51450_e85469_d_n3, assign51450_e85469_d_n4, assign51450_e85469_d_n5, assign51450_e85469_d_n6, assign51450_e85469_d_n7, assign51450_e85469_d_n8, assign51450_e85469_d_n9, assign51450_e85469_d_n10, assign51450_e85469_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51450_e85467: f64 = (locals.var_vgb * locals.var_vgp_eff);
        (assign51450_e85467, (locals.var_vgb * locals.var_vgp_eff_dn3), (locals.var_vgb * locals.var_vgp_eff_dn4), (locals.var_vgb * locals.var_vgp_eff_dn5), (locals.var_vgb * locals.var_vgp_eff_dn6), (locals.var_vgb * locals.var_vgp_eff_dn7), ((locals.var_vgb_dn8 * locals.var_vgp_eff) + (locals.var_vgb * locals.var_vgp_eff_dn8)), (locals.var_vgb * locals.var_vgp_eff_dn9), (locals.var_vgb * locals.var_vgp_eff_dn10), ((locals.var_vgb_dn11 * locals.var_vgp_eff) + (locals.var_vgb * locals.var_vgp_eff_dn11)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51450_e85469;
        locals.var_t2_dn3 = assign51450_e85469_d_n3;
        locals.var_t2_dn4 = assign51450_e85469_d_n4;
        locals.var_t2_dn5 = assign51450_e85469_d_n5;
        locals.var_t2_dn6 = assign51450_e85469_d_n6;
        locals.var_t2_dn7 = assign51450_e85469_d_n7;
        locals.var_t2_dn8 = assign51450_e85469_d_n8;
        locals.var_t2_dn9 = assign51450_e85469_d_n9;
        locals.var_t2_dn10 = assign51450_e85469_d_n10;
        locals.var_t2_dn11 = assign51450_e85469_d_n11;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_178(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51460_e85482, assign51460_e85482_d_n3, assign51460_e85482_d_n4, assign51460_e85482_d_n5, assign51460_e85482_d_n6, assign51460_e85482_d_n7, assign51460_e85482_d_n8, assign51460_e85482_d_n9, assign51460_e85482_d_n10, assign51460_e85482_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51460_e85478: f64 = (locals.var_aigbcp2_i * locals.var_cigbcp2_i);
        let assign51460_e85480: f64 = (assign51460_e85478 - locals.var_bigbcp2_i);
        (assign51460_e85480, 0.0, (locals.var_aigbcp2_i_dn4 * locals.var_cigbcp2_i), (locals.var_aigbcp2_i_dn5 * locals.var_cigbcp2_i), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51460_e85482;
        locals.var_t3_dn3 = assign51460_e85482_d_n3;
        locals.var_t3_dn4 = assign51460_e85482_d_n4;
        locals.var_t3_dn5 = assign51460_e85482_d_n5;
        locals.var_t3_dn6 = assign51460_e85482_d_n6;
        locals.var_t3_dn7 = assign51460_e85482_d_n7;
        locals.var_t3_dn8 = assign51460_e85482_d_n8;
        locals.var_t3_dn9 = assign51460_e85482_d_n9;
        locals.var_t3_dn10 = assign51460_e85482_d_n10;
        locals.var_t3_dn11 = assign51460_e85482_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51470_e85493, assign51470_e85493_d_n3, assign51470_e85493_d_n4, assign51470_e85493_d_n5, assign51470_e85493_d_n6, assign51470_e85493_d_n7, assign51470_e85493_d_n8, assign51470_e85493_d_n9, assign51470_e85493_d_n10, assign51470_e85493_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51470_e85491: f64 = (locals.var_bigbcp2_i * locals.var_cigbcp2_i);
        (assign51470_e85491, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51470_e85493;
        locals.var_t4_dn3 = assign51470_e85493_d_n3;
        locals.var_t4_dn4 = assign51470_e85493_d_n4;
        locals.var_t4_dn5 = assign51470_e85493_d_n5;
        locals.var_t4_dn6 = assign51470_e85493_d_n6;
        locals.var_t4_dn7 = assign51470_e85493_d_n7;
        locals.var_t4_dn8 = assign51470_e85493_d_n8;
        locals.var_t4_dn9 = assign51470_e85493_d_n9;
        locals.var_t4_dn10 = assign51470_e85493_d_n10;
        locals.var_t4_dn11 = assign51470_e85493_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51480_e85517, assign51480_e85517_d_n3, assign51480_e85517_d_n4, assign51480_e85517_d_n5, assign51480_e85517_d_n6, assign51480_e85517_d_n7, assign51480_e85517_d_n8, assign51480_e85517_d_n9, assign51480_e85517_d_n10, assign51480_e85517_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51480_e85501: f64 = (-locals.var_t12);
        let assign51480_e85503: f64 = (assign51480_e85501 * p.p76);
        let assign51480_e85507: f64 = (locals.var_t3 * locals.var_vgp_eff);
        let assign51480_e85508: f64 = (locals.var_aigbcp2_i + assign51480_e85507);
        let assign51480_e85511: f64 = (locals.var_t4 * locals.var_vgp_eff);
        let assign51480_e85513: f64 = (assign51480_e85511 * locals.var_vgp_eff);
        let assign51480_e85514: f64 = (assign51480_e85508 - assign51480_e85513);
        let assign51480_e85515: f64 = (assign51480_e85503 * assign51480_e85514);
        (assign51480_e85515, ((((-locals.var_t12_dn3) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn3 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn3)) - ((((locals.var_t4_dn3 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn3)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn3))))), ((((-locals.var_t12_dn4) * p.p76) * assign51480_e85514) + (assign51480_e85503 * ((locals.var_aigbcp2_i_dn4 + ((locals.var_t3_dn4 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn4))) - ((((locals.var_t4_dn4 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn4)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn4))))), ((((-locals.var_t12_dn5) * p.p76) * assign51480_e85514) + (assign51480_e85503 * ((locals.var_aigbcp2_i_dn5 + ((locals.var_t3_dn5 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn5))) - ((((locals.var_t4_dn5 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn5)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn5))))), ((((-locals.var_t12_dn6) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn6 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn6)) - ((((locals.var_t4_dn6 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn6)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn6))))), ((((-locals.var_t12_dn7) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn7 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn7)) - ((((locals.var_t4_dn7 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn7)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn7))))), ((((-locals.var_t12_dn8) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn8 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn8)) - ((((locals.var_t4_dn8 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn8)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn8))))), ((((-locals.var_t12_dn9) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn9 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn9)) - ((((locals.var_t4_dn9 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn9)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn9))))), ((((-locals.var_t12_dn10) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn10 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn10)) - ((((locals.var_t4_dn10 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn10)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn10))))), ((((-locals.var_t12_dn11) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn11 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn11)) - ((((locals.var_t4_dn11 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn11)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn11))))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign51480_e85517;
        locals.var_t5_dn3 = assign51480_e85517_d_n3;
        locals.var_t5_dn4 = assign51480_e85517_d_n4;
        locals.var_t5_dn5 = assign51480_e85517_d_n5;
        locals.var_t5_dn6 = assign51480_e85517_d_n6;
        locals.var_t5_dn7 = assign51480_e85517_d_n7;
        locals.var_t5_dn8 = assign51480_e85517_d_n8;
        locals.var_t5_dn9 = assign51480_e85517_d_n9;
        locals.var_t5_dn10 = assign51480_e85517_d_n10;
        locals.var_t5_dn11 = assign51480_e85517_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign51490_e85527, assign51490_e85527_d_n3, assign51490_e85527_d_n4, assign51490_e85527_d_n5, assign51490_e85527_d_n6, assign51490_e85527_d_n7, assign51490_e85527_d_n8, assign51490_e85527_d_n9, assign51490_e85527_d_n10, assign51490_e85527_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51490_e85525: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51490_e85525, ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign51490_e85527;
        locals.var_t6_dn3 = assign51490_e85527_d_n3;
        locals.var_t6_dn4 = assign51490_e85527_d_n4;
        locals.var_t6_dn5 = assign51490_e85527_d_n5;
        locals.var_t6_dn6 = assign51490_e85527_d_n6;
        locals.var_t6_dn7 = assign51490_e85527_d_n7;
        locals.var_t6_dn8 = assign51490_e85527_d_n8;
        locals.var_t6_dn9 = assign51490_e85527_d_n9;
        locals.var_t6_dn10 = assign51490_e85527_d_n10;
        locals.var_t6_dn11 = assign51490_e85527_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign51500_e85540, assign51500_e85540_d_n3, assign51500_e85540_d_n4, assign51500_e85540_d_n5, assign51500_e85540_d_n6, assign51500_e85540_d_n7, assign51500_e85540_d_n8, assign51500_e85540_d_n9, assign51500_e85540_d_n10, assign51500_e85540_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51500_e85536: f64 = (locals.var_t11 * p.p1380);
        let assign51500_e85538: f64 = (assign51500_e85536 * locals.var_toxratio);
        (assign51500_e85538, (((locals.var_t11_dn3 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn3)), (((locals.var_t11_dn4 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn4)), (((locals.var_t11_dn5 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn5)), (((locals.var_t11_dn6 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn6)), (((locals.var_t11_dn7 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn7)), (((locals.var_t11_dn8 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn8)), (((locals.var_t11_dn9 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn9)), (((locals.var_t11_dn10 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn10)), (((locals.var_t11_dn11 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn11)),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign51500_e85540;
        locals.var_t11_dn3 = assign51500_e85540_d_n3;
        locals.var_t11_dn4 = assign51500_e85540_d_n4;
        locals.var_t11_dn5 = assign51500_e85540_d_n5;
        locals.var_t11_dn6 = assign51500_e85540_d_n6;
        locals.var_t11_dn7 = assign51500_e85540_d_n7;
        locals.var_t11_dn8 = assign51500_e85540_d_n8;
        locals.var_t11_dn9 = assign51500_e85540_d_n9;
        locals.var_t11_dn10 = assign51500_e85540_d_n10;
        locals.var_t11_dn11 = assign51500_e85540_d_n11;
        locals.var_t11_rv = 0.0;

        let assign51530_e85568: f64 = if p.p37 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard783 = assign51530_e85568;
        locals.var_guard783_rv = 0.0;

        let (assign51540_e85581, assign51540_e85581_d_n3, assign51540_e85581_d_n4, assign51540_e85581_d_n5, assign51540_e85581_d_n6, assign51540_e85581_d_n7, assign51540_e85581_d_n8, assign51540_e85581_d_n9, assign51540_e85581_d_n10, assign51540_e85581_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51540_e85578: f64 = (locals.var_bigc_i * locals.var_voxminv);
        let assign51540_e85579: f64 = (locals.var_aigc_i - assign51540_e85578);
        (assign51540_e85579, (-(locals.var_bigc_i * locals.var_voxminv_dn3)), (locals.var_aigc_i_dn4 - (locals.var_bigc_i * locals.var_voxminv_dn4)), (locals.var_aigc_i_dn5 - (locals.var_bigc_i * locals.var_voxminv_dn5)), (-(locals.var_bigc_i * locals.var_voxminv_dn6)), (-(locals.var_bigc_i * locals.var_voxminv_dn7)), (-(locals.var_bigc_i * locals.var_voxminv_dn8)), (-(locals.var_bigc_i * locals.var_voxminv_dn9)), (-(locals.var_bigc_i * locals.var_voxminv_dn10)), (-(locals.var_bigc_i * locals.var_voxminv_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51540_e85581;
        locals.var_t1_dn3 = assign51540_e85581_d_n3;
        locals.var_t1_dn4 = assign51540_e85581_d_n4;
        locals.var_t1_dn5 = assign51540_e85581_d_n5;
        locals.var_t1_dn6 = assign51540_e85581_d_n6;
        locals.var_t1_dn7 = assign51540_e85581_d_n7;
        locals.var_t1_dn8 = assign51540_e85581_d_n8;
        locals.var_t1_dn9 = assign51540_e85581_d_n9;
        locals.var_t1_dn10 = assign51540_e85581_d_n10;
        locals.var_t1_dn11 = assign51540_e85581_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51550_e85594, assign51550_e85594_d_n3, assign51550_e85594_d_n4, assign51550_e85594_d_n5, assign51550_e85594_d_n6, assign51550_e85594_d_n7, assign51550_e85594_d_n8, assign51550_e85594_d_n9, assign51550_e85594_d_n10, assign51550_e85594_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51550_e85591: f64 = (locals.var_cigc_i * locals.var_voxminv);
        let assign51550_e85592: f64 = (1.0 + assign51550_e85591);
        (assign51550_e85592, (locals.var_cigc_i * locals.var_voxminv_dn3), (locals.var_cigc_i * locals.var_voxminv_dn4), (locals.var_cigc_i * locals.var_voxminv_dn5), (locals.var_cigc_i * locals.var_voxminv_dn6), (locals.var_cigc_i * locals.var_voxminv_dn7), (locals.var_cigc_i * locals.var_voxminv_dn8), (locals.var_cigc_i * locals.var_voxminv_dn9), (locals.var_cigc_i * locals.var_voxminv_dn10), (locals.var_cigc_i * locals.var_voxminv_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51550_e85594;
        locals.var_t2_dn3 = assign51550_e85594_d_n3;
        locals.var_t2_dn4 = assign51550_e85594_d_n4;
        locals.var_t2_dn5 = assign51550_e85594_d_n5;
        locals.var_t2_dn6 = assign51550_e85594_d_n6;
        locals.var_t2_dn7 = assign51550_e85594_d_n7;
        locals.var_t2_dn8 = assign51550_e85594_d_n8;
        locals.var_t2_dn9 = assign51550_e85594_d_n9;
        locals.var_t2_dn10 = assign51550_e85594_d_n10;
        locals.var_t2_dn11 = assign51550_e85594_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51560_e85607, assign51560_e85607_d_n3, assign51560_e85607_d_n4, assign51560_e85607_d_n5, assign51560_e85607_d_n6, assign51560_e85607_d_n7, assign51560_e85607_d_n8, assign51560_e85607_d_n9, assign51560_e85607_d_n10, assign51560_e85607_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51560_e85603: f64 = (locals.var_bechvb * locals.var_t1);
        let assign51560_e85605: f64 = (assign51560_e85603 * locals.var_t2);
        (assign51560_e85605, (((locals.var_bechvb * locals.var_t1_dn3) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn3)), (((locals.var_bechvb * locals.var_t1_dn4) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn4)), (((locals.var_bechvb * locals.var_t1_dn5) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn5)), (((locals.var_bechvb * locals.var_t1_dn6) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn6)), (((locals.var_bechvb * locals.var_t1_dn7) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn7)), (((locals.var_bechvb * locals.var_t1_dn8) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn8)), (((locals.var_bechvb * locals.var_t1_dn9) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn9)), (((locals.var_bechvb * locals.var_t1_dn10) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn10)), (((locals.var_bechvb * locals.var_t1_dn11) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51560_e85607;
        locals.var_t3_dn3 = assign51560_e85607_d_n3;
        locals.var_t3_dn4 = assign51560_e85607_d_n4;
        locals.var_t3_dn5 = assign51560_e85607_d_n5;
        locals.var_t3_dn6 = assign51560_e85607_d_n6;
        locals.var_t3_dn7 = assign51560_e85607_d_n7;
        locals.var_t3_dn8 = assign51560_e85607_d_n8;
        locals.var_t3_dn9 = assign51560_e85607_d_n9;
        locals.var_t3_dn10 = assign51560_e85607_d_n10;
        locals.var_t3_dn11 = assign51560_e85607_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51570_e85625, assign51570_e85625_d_n3, assign51570_e85625_d_n4, assign51570_e85625_d_n5, assign51570_e85625_d_n6, assign51570_e85625_d_n7, assign51570_e85625_d_n8, assign51570_e85625_d_n9, assign51570_e85625_d_n10, assign51570_e85625_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51570_e85616: f64 = (locals.var_nq * locals.var_nvt);
        let assign51570_e85619: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign51570_e85620: f64 = (assign51570_e85616 * assign51570_e85619);
        let assign51570_e85622: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign51570_e85623: f64 = (assign51570_e85620 * assign51570_e85622);
        (assign51570_e85623, ((((((locals.var_nq_dn3 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn3)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3))), ((((((locals.var_nq_dn4 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn4)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4))), ((((((locals.var_nq_dn5 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn5)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5))), ((((((locals.var_nq_dn6 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn6)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6))), ((((((locals.var_nq_dn7 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn7)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7))), ((((((locals.var_nq_dn8 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn8)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8))), ((((((locals.var_nq_dn9 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn9)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9))), ((((((locals.var_nq_dn10 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn10)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10))), ((((((locals.var_nq_dn11 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn11)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51570_e85625;
        locals.var_t4_dn3 = assign51570_e85625_d_n3;
        locals.var_t4_dn4 = assign51570_e85625_d_n4;
        locals.var_t4_dn5 = assign51570_e85625_d_n5;
        locals.var_t4_dn6 = assign51570_e85625_d_n6;
        locals.var_t4_dn7 = assign51570_e85625_d_n7;
        locals.var_t4_dn8 = assign51570_e85625_d_n8;
        locals.var_t4_dn9 = assign51570_e85625_d_n9;
        locals.var_t4_dn10 = assign51570_e85625_d_n10;
        locals.var_t4_dn11 = assign51570_e85625_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51590_e85668, assign51590_e85668_d_n3, assign51590_e85668_d_n4, assign51590_e85668_d_n5, assign51590_e85668_d_n6, assign51590_e85668_d_n7, assign51590_e85668_d_n8, assign51590_e85668_d_n9, assign51590_e85668_d_n10, assign51590_e85668_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51590_e85661: f64 = (locals.var_vdseff * locals.var_vdseff);
        let assign51590_e85663: f64 = (assign51590_e85661 + 0.01);
        let assign51590_e85664: f64 = (assign51590_e85663).sqrt();
        let assign51590_e85666: f64 = (assign51590_e85664 - 0.1);
        (assign51590_e85666, (((locals.var_vdseff_dn3 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn3)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn4 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn4)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn5 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn5)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn6 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn6)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn7 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn7)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn8 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn8)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn9 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn9)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn10 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn10)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn11 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn11)) / (2.0 * assign51590_e85664)),)
    } else {
        (locals.var_vdseffx, locals.var_vdseffx_dn3, locals.var_vdseffx_dn4, locals.var_vdseffx_dn5, locals.var_vdseffx_dn6, locals.var_vdseffx_dn7, locals.var_vdseffx_dn8, locals.var_vdseffx_dn9, locals.var_vdseffx_dn10, locals.var_vdseffx_dn11,)
    }
};
        locals.var_vdseffx = assign51590_e85668;
        locals.var_vdseffx_dn3 = assign51590_e85668_d_n3;
        locals.var_vdseffx_dn4 = assign51590_e85668_d_n4;
        locals.var_vdseffx_dn5 = assign51590_e85668_d_n5;
        locals.var_vdseffx_dn6 = assign51590_e85668_d_n6;
        locals.var_vdseffx_dn7 = assign51590_e85668_d_n7;
        locals.var_vdseffx_dn8 = assign51590_e85668_d_n8;
        locals.var_vdseffx_dn9 = assign51590_e85668_d_n9;
        locals.var_vdseffx_dn10 = assign51590_e85668_d_n10;
        locals.var_vdseffx_dn11 = assign51590_e85668_d_n11;
        locals.var_vdseffx_rv = 0.0;

        let (assign51600_e85679, assign51600_e85679_d_n3, assign51600_e85679_d_n4, assign51600_e85679_d_n5, assign51600_e85679_d_n6, assign51600_e85679_d_n7, assign51600_e85679_d_n8, assign51600_e85679_d_n9, assign51600_e85679_d_n10, assign51600_e85679_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51600_e85677: f64 = (locals.var_pigcd_i * locals.var_vdseffx);
        (assign51600_e85677, (locals.var_pigcd_i * locals.var_vdseffx_dn3), (locals.var_pigcd_i * locals.var_vdseffx_dn4), (locals.var_pigcd_i * locals.var_vdseffx_dn5), (locals.var_pigcd_i * locals.var_vdseffx_dn6), (locals.var_pigcd_i * locals.var_vdseffx_dn7), (locals.var_pigcd_i * locals.var_vdseffx_dn8), (locals.var_pigcd_i * locals.var_vdseffx_dn9), (locals.var_pigcd_i * locals.var_vdseffx_dn10), (locals.var_pigcd_i * locals.var_vdseffx_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51600_e85679;
        locals.var_t1_dn3 = assign51600_e85679_d_n3;
        locals.var_t1_dn4 = assign51600_e85679_d_n4;
        locals.var_t1_dn5 = assign51600_e85679_d_n5;
        locals.var_t1_dn6 = assign51600_e85679_d_n6;
        locals.var_t1_dn7 = assign51600_e85679_d_n7;
        locals.var_t1_dn8 = assign51600_e85679_d_n8;
        locals.var_t1_dn9 = assign51600_e85679_d_n9;
        locals.var_t1_dn10 = assign51600_e85679_d_n10;
        locals.var_t1_dn11 = assign51600_e85679_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51610_e85690, assign51610_e85690_d_n3, assign51610_e85690_d_n4, assign51610_e85690_d_n5, assign51610_e85690_d_n6, assign51610_e85690_d_n7, assign51610_e85690_d_n8, assign51610_e85690_d_n9, assign51610_e85690_d_n10, assign51610_e85690_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51610_e85687: f64 = (-locals.var_t1);
        let assign51610_e85688: f64 = { let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51610_e85688, ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11)),)
    } else {
        (locals.var_t1_exp, locals.var_t1_exp_dn3, locals.var_t1_exp_dn4, locals.var_t1_exp_dn5, locals.var_t1_exp_dn6, locals.var_t1_exp_dn7, locals.var_t1_exp_dn8, locals.var_t1_exp_dn9, locals.var_t1_exp_dn10, locals.var_t1_exp_dn11,)
    }
};
        locals.var_t1_exp = assign51610_e85690;
        locals.var_t1_exp_dn3 = assign51610_e85690_d_n3;
        locals.var_t1_exp_dn4 = assign51610_e85690_d_n4;
        locals.var_t1_exp_dn5 = assign51610_e85690_d_n5;
        locals.var_t1_exp_dn6 = assign51610_e85690_d_n6;
        locals.var_t1_exp_dn7 = assign51610_e85690_d_n7;
        locals.var_t1_exp_dn8 = assign51610_e85690_d_n8;
        locals.var_t1_exp_dn9 = assign51610_e85690_d_n9;
        locals.var_t1_exp_dn10 = assign51610_e85690_d_n10;
        locals.var_t1_exp_dn11 = assign51610_e85690_d_n11;
        locals.var_t1_exp_rv = 0.0;

        let (assign51620_e85705, assign51620_e85705_d_n3, assign51620_e85705_d_n4, assign51620_e85705_d_n5, assign51620_e85705_d_n6, assign51620_e85705_d_n7, assign51620_e85705_d_n8, assign51620_e85705_d_n9, assign51620_e85705_d_n10, assign51620_e85705_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51620_e85699: f64 = (locals.var_t1 + locals.var_t1_exp);
        let assign51620_e85701: f64 = (assign51620_e85699 - 1.0);
        let assign51620_e85703: f64 = (assign51620_e85701 + 0.0001);
        (assign51620_e85703, (locals.var_t1_dn3 + locals.var_t1_exp_dn3), (locals.var_t1_dn4 + locals.var_t1_exp_dn4), (locals.var_t1_dn5 + locals.var_t1_exp_dn5), (locals.var_t1_dn6 + locals.var_t1_exp_dn6), (locals.var_t1_dn7 + locals.var_t1_exp_dn7), (locals.var_t1_dn8 + locals.var_t1_exp_dn8), (locals.var_t1_dn9 + locals.var_t1_exp_dn9), (locals.var_t1_dn10 + locals.var_t1_exp_dn10), (locals.var_t1_dn11 + locals.var_t1_exp_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51620_e85705;
        locals.var_t3_dn3 = assign51620_e85705_d_n3;
        locals.var_t3_dn4 = assign51620_e85705_d_n4;
        locals.var_t3_dn5 = assign51620_e85705_d_n5;
        locals.var_t3_dn6 = assign51620_e85705_d_n6;
        locals.var_t3_dn7 = assign51620_e85705_d_n7;
        locals.var_t3_dn8 = assign51620_e85705_d_n8;
        locals.var_t3_dn9 = assign51620_e85705_d_n9;
        locals.var_t3_dn10 = assign51620_e85705_d_n10;
        locals.var_t3_dn11 = assign51620_e85705_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51630_e85722, assign51630_e85722_d_n3, assign51630_e85722_d_n4, assign51630_e85722_d_n5, assign51630_e85722_d_n6, assign51630_e85722_d_n7, assign51630_e85722_d_n8, assign51630_e85722_d_n9, assign51630_e85722_d_n10, assign51630_e85722_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51630_e85715: f64 = (locals.var_t1 + 1.0);
        let assign51630_e85717: f64 = (assign51630_e85715 * locals.var_t1_exp);
        let assign51630_e85718: f64 = (1.0 - assign51630_e85717);
        let assign51630_e85720: f64 = (assign51630_e85718 + 0.0001);
        (assign51630_e85720, (-((locals.var_t1_dn3 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn3))), (-((locals.var_t1_dn4 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn4))), (-((locals.var_t1_dn5 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn5))), (-((locals.var_t1_dn6 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn6))), (-((locals.var_t1_dn7 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn7))), (-((locals.var_t1_dn8 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn8))), (-((locals.var_t1_dn9 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn9))), (-((locals.var_t1_dn10 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn10))), (-((locals.var_t1_dn11 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn11))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51630_e85722;
        locals.var_t4_dn3 = assign51630_e85722_d_n3;
        locals.var_t4_dn4 = assign51630_e85722_d_n4;
        locals.var_t4_dn5 = assign51630_e85722_d_n5;
        locals.var_t4_dn6 = assign51630_e85722_d_n6;
        locals.var_t4_dn7 = assign51630_e85722_d_n7;
        locals.var_t4_dn8 = assign51630_e85722_d_n8;
        locals.var_t4_dn9 = assign51630_e85722_d_n9;
        locals.var_t4_dn10 = assign51630_e85722_d_n10;
        locals.var_t4_dn11 = assign51630_e85722_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51640_e85735, assign51640_e85735_d_n3, assign51640_e85735_d_n4, assign51640_e85735_d_n5, assign51640_e85735_d_n6, assign51640_e85735_d_n7, assign51640_e85735_d_n8, assign51640_e85735_d_n9, assign51640_e85735_d_n10, assign51640_e85735_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51640_e85731: f64 = (locals.var_t1 * locals.var_t1);
        let assign51640_e85733: f64 = (assign51640_e85731 + 0.0002);
        (assign51640_e85733, ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign51640_e85735;
        locals.var_t5_dn3 = assign51640_e85735_d_n3;
        locals.var_t5_dn4 = assign51640_e85735_d_n4;
        locals.var_t5_dn5 = assign51640_e85735_d_n5;
        locals.var_t5_dn6 = assign51640_e85735_d_n6;
        locals.var_t5_dn7 = assign51640_e85735_d_n7;
        locals.var_t5_dn8 = assign51640_e85735_d_n8;
        locals.var_t5_dn9 = assign51640_e85735_d_n9;
        locals.var_t5_dn10 = assign51640_e85735_d_n10;
        locals.var_t5_dn11 = assign51640_e85735_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign51700_e85811, assign51700_e85811_d_n3, assign51700_e85811_d_n4, assign51700_e85811_d_n5, assign51700_e85811_d_n6, assign51700_e85811_d_n7, assign51700_e85811_d_n8, assign51700_e85811_d_n9, assign51700_e85811_d_n10, assign51700_e85811_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51700_e85809: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign51700_e85809, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51700_e85811;
        locals.var_t2_dn3 = assign51700_e85811_d_n3;
        locals.var_t2_dn4 = assign51700_e85811_d_n4;
        locals.var_t2_dn5 = assign51700_e85811_d_n5;
        locals.var_t2_dn6 = assign51700_e85811_d_n6;
        locals.var_t2_dn7 = assign51700_e85811_d_n7;
        locals.var_t2_dn8 = assign51700_e85811_d_n8;
        locals.var_t2_dn9 = assign51700_e85811_d_n9;
        locals.var_t2_dn10 = assign51700_e85811_d_n10;
        locals.var_t2_dn11 = assign51700_e85811_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51710_e85825, assign51710_e85825_d_n3, assign51710_e85825_d_n4, assign51710_e85825_d_n5, assign51710_e85825_d_n6, assign51710_e85825_d_n7, assign51710_e85825_d_n8, assign51710_e85825_d_n9, assign51710_e85825_d_n10, assign51710_e85825_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51710_e85820: f64 = (locals.var_t2 * locals.var_t2);
        let assign51710_e85822: f64 = (assign51710_e85820 + 0.0001);
        let assign51710_e85823: f64 = (assign51710_e85822).sqrt();
        (assign51710_e85823, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign51710_e85823)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign51710_e85825;
        locals.var_vgs_eff_dn3 = assign51710_e85825_d_n3;
        locals.var_vgs_eff_dn4 = assign51710_e85825_d_n4;
        locals.var_vgs_eff_dn5 = assign51710_e85825_d_n5;
        locals.var_vgs_eff_dn6 = assign51710_e85825_d_n6;
        locals.var_vgs_eff_dn7 = assign51710_e85825_d_n7;
        locals.var_vgs_eff_dn8 = assign51710_e85825_d_n8;
        locals.var_vgs_eff_dn9 = assign51710_e85825_d_n9;
        locals.var_vgs_eff_dn10 = assign51710_e85825_d_n10;
        locals.var_vgs_eff_dn11 = assign51710_e85825_d_n11;
        locals.var_vgs_eff_rv = 0.0;

        let assign51720_e85828: f64 = if p.p1295 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard785 = assign51720_e85828;
        locals.var_guard785_rv = 0.0;

        let (assign51730_e85864, assign51730_e85864_d_n3, assign51730_e85864_d_n4, assign51730_e85864_d_n5, assign51730_e85864_d_n6, assign51730_e85864_d_n7, assign51730_e85864_d_n8, assign51730_e85864_d_n9, assign51730_e85864_d_n10, assign51730_e85864_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard785 != 0.0)) {
        let assign51730_e85841: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51730_e85842: f64 = (locals.var_aigs_i - assign51730_e85841);
        let assign51730_e85846: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51730_e85847: f64 = (locals.var_aigs_i - assign51730_e85846);
        let assign51730_e85851: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51730_e85852: f64 = (locals.var_aigs_i - assign51730_e85851);
        let assign51730_e85853: f64 = (assign51730_e85847 * assign51730_e85852);
        let assign51730_e85856: f64 = (4.0 * 1e-6);
        let assign51730_e85858: f64 = (assign51730_e85856 * 1e-6);
        let assign51730_e85859: f64 = (assign51730_e85853 + assign51730_e85858);
        let assign51730_e85860: f64 = (assign51730_e85859).sqrt();
        let assign51730_e85861: f64 = (assign51730_e85842 + assign51730_e85860);
        let assign51730_e85862: f64 = (0.5 * assign51730_e85861);
        (assign51730_e85862, (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn3)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn3)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn3)))) / (2.0 * assign51730_e85860)))), (0.5 * ((locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)) + ((((locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)) * assign51730_e85852) + (assign51730_e85847 * (locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)))) / (2.0 * assign51730_e85860)))), (0.5 * ((locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)) + ((((locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)) * assign51730_e85852) + (assign51730_e85847 * (locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn6)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn6)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn6)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn7)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn7)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn7)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn8)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn8)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn8)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn9)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn9)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn9)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn10)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn10)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn10)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn11)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn11)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn11)))) / (2.0 * assign51730_e85860)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51730_e85864;
        locals.var_t1_dn3 = assign51730_e85864_d_n3;
        locals.var_t1_dn4 = assign51730_e85864_d_n4;
        locals.var_t1_dn5 = assign51730_e85864_d_n5;
        locals.var_t1_dn6 = assign51730_e85864_d_n6;
        locals.var_t1_dn7 = assign51730_e85864_d_n7;
        locals.var_t1_dn8 = assign51730_e85864_d_n8;
        locals.var_t1_dn9 = assign51730_e85864_d_n9;
        locals.var_t1_dn10 = assign51730_e85864_d_n10;
        locals.var_t1_dn11 = assign51730_e85864_d_n11;
        locals.var_t1_rv = 0.0;

        let assign51740_e85867: f64 = if locals.var_cigs_i < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard786 = assign51740_e85867;
        locals.var_guard786_rv = 0.0;

        let (assign51750_e85880,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 != 0.0)) {
        (0.01,)
    } else {
        (locals.var_cigs_i,)
    }
};
        locals.var_cigs_i = assign51750_e85880;
        locals.var_cigs_i_rv = 0.0;

        let (assign51760_e85896, assign51760_e85896_d_n3, assign51760_e85896_d_n4, assign51760_e85896_d_n5, assign51760_e85896_d_n6, assign51760_e85896_d_n7, assign51760_e85896_d_n8, assign51760_e85896_d_n9, assign51760_e85896_d_n10, assign51760_e85896_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard785 == 0.0)) {
        let assign51760_e85893: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51760_e85894: f64 = (locals.var_aigs_i - assign51760_e85893);
        (assign51760_e85894, (-(locals.var_bigs_i * locals.var_vgs_eff_dn3)), (locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)), (locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn6)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn7)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn8)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn9)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn10)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51760_e85896;
        locals.var_t1_dn3 = assign51760_e85896_d_n3;
        locals.var_t1_dn4 = assign51760_e85896_d_n4;
        locals.var_t1_dn5 = assign51760_e85896_d_n5;
        locals.var_t1_dn6 = assign51760_e85896_d_n6;
        locals.var_t1_dn7 = assign51760_e85896_d_n7;
        locals.var_t1_dn8 = assign51760_e85896_d_n8;
        locals.var_t1_dn9 = assign51760_e85896_d_n9;
        locals.var_t1_dn10 = assign51760_e85896_d_n10;
        locals.var_t1_dn11 = assign51760_e85896_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51770_e85909, assign51770_e85909_d_n3, assign51770_e85909_d_n4, assign51770_e85909_d_n5, assign51770_e85909_d_n6, assign51770_e85909_d_n7, assign51770_e85909_d_n8, assign51770_e85909_d_n9, assign51770_e85909_d_n10, assign51770_e85909_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51770_e85906: f64 = (locals.var_cigs_i * locals.var_vgs_eff);
        let assign51770_e85907: f64 = (1.0 + assign51770_e85906);
        (assign51770_e85907, (locals.var_cigs_i * locals.var_vgs_eff_dn3), (locals.var_cigs_i * locals.var_vgs_eff_dn4), (locals.var_cigs_i * locals.var_vgs_eff_dn5), (locals.var_cigs_i * locals.var_vgs_eff_dn6), (locals.var_cigs_i * locals.var_vgs_eff_dn7), (locals.var_cigs_i * locals.var_vgs_eff_dn8), (locals.var_cigs_i * locals.var_vgs_eff_dn9), (locals.var_cigs_i * locals.var_vgs_eff_dn10), (locals.var_cigs_i * locals.var_vgs_eff_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51770_e85909;
        locals.var_t2_dn3 = assign51770_e85909_d_n3;
        locals.var_t2_dn4 = assign51770_e85909_d_n4;
        locals.var_t2_dn5 = assign51770_e85909_d_n5;
        locals.var_t2_dn6 = assign51770_e85909_d_n6;
        locals.var_t2_dn7 = assign51770_e85909_d_n7;
        locals.var_t2_dn8 = assign51770_e85909_d_n8;
        locals.var_t2_dn9 = assign51770_e85909_d_n9;
        locals.var_t2_dn10 = assign51770_e85909_d_n10;
        locals.var_t2_dn11 = assign51770_e85909_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51780_e85922, assign51780_e85922_d_n3, assign51780_e85922_d_n4, assign51780_e85922_d_n5, assign51780_e85922_d_n6, assign51780_e85922_d_n7, assign51780_e85922_d_n8, assign51780_e85922_d_n9, assign51780_e85922_d_n10, assign51780_e85922_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51780_e85918: f64 = (locals.var_bechvbedge * locals.var_t1);
        let assign51780_e85920: f64 = (assign51780_e85918 * locals.var_t2);
        (assign51780_e85920, (((locals.var_bechvbedge * locals.var_t1_dn3) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn3)), (((locals.var_bechvbedge * locals.var_t1_dn4) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn4)), (((locals.var_bechvbedge * locals.var_t1_dn5) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn5)), (((locals.var_bechvbedge * locals.var_t1_dn6) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn6)), (((locals.var_bechvbedge * locals.var_t1_dn7) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn7)), (((locals.var_bechvbedge * locals.var_t1_dn8) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn8)), (((locals.var_bechvbedge * locals.var_t1_dn9) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn9)), (((locals.var_bechvbedge * locals.var_t1_dn10) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn10)), (((locals.var_bechvbedge * locals.var_t1_dn11) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51780_e85922;
        locals.var_t3_dn3 = assign51780_e85922_d_n3;
        locals.var_t3_dn4 = assign51780_e85922_d_n4;
        locals.var_t3_dn5 = assign51780_e85922_d_n5;
        locals.var_t3_dn6 = assign51780_e85922_d_n6;
        locals.var_t3_dn7 = assign51780_e85922_d_n7;
        locals.var_t3_dn8 = assign51780_e85922_d_n8;
        locals.var_t3_dn9 = assign51780_e85922_d_n9;
        locals.var_t3_dn10 = assign51780_e85922_d_n10;
        locals.var_t3_dn11 = assign51780_e85922_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51790_e85932, assign51790_e85932_d_n3, assign51790_e85932_d_n4, assign51790_e85932_d_n5, assign51790_e85932_d_n6, assign51790_e85932_d_n7, assign51790_e85932_d_n8, assign51790_e85932_d_n9, assign51790_e85932_d_n10, assign51790_e85932_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51790_e85930: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51790_e85930, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51790_e85932;
        locals.var_t4_dn3 = assign51790_e85932_d_n3;
        locals.var_t4_dn4 = assign51790_e85932_d_n4;
        locals.var_t4_dn5 = assign51790_e85932_d_n5;
        locals.var_t4_dn6 = assign51790_e85932_d_n6;
        locals.var_t4_dn7 = assign51790_e85932_d_n7;
        locals.var_t4_dn8 = assign51790_e85932_d_n8;
        locals.var_t4_dn9 = assign51790_e85932_d_n9;
        locals.var_t4_dn10 = assign51790_e85932_d_n10;
        locals.var_t4_dn11 = assign51790_e85932_d_n11;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_179(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51820_e85973, assign51820_e85973_d_n3, assign51820_e85973_d_n4, assign51820_e85973_d_n5, assign51820_e85973_d_n6, assign51820_e85973_d_n7, assign51820_e85973_d_n8, assign51820_e85973_d_n9, assign51820_e85973_d_n10, assign51820_e85973_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51820_e85971: f64 = (locals.var_vgd_noswap - locals.var_vfbsdr);
        (assign51820_e85971, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgd_noswap_dn6, locals.var_vgd_noswap_dn7, locals.var_vgd_noswap_dn8, 0.0, locals.var_vgd_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51820_e85973;
        locals.var_t2_dn3 = assign51820_e85973_d_n3;
        locals.var_t2_dn4 = assign51820_e85973_d_n4;
        locals.var_t2_dn5 = assign51820_e85973_d_n5;
        locals.var_t2_dn6 = assign51820_e85973_d_n6;
        locals.var_t2_dn7 = assign51820_e85973_d_n7;
        locals.var_t2_dn8 = assign51820_e85973_d_n8;
        locals.var_t2_dn9 = assign51820_e85973_d_n9;
        locals.var_t2_dn10 = assign51820_e85973_d_n10;
        locals.var_t2_dn11 = assign51820_e85973_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51830_e85987, assign51830_e85987_d_n3, assign51830_e85987_d_n4, assign51830_e85987_d_n5, assign51830_e85987_d_n6, assign51830_e85987_d_n7, assign51830_e85987_d_n8, assign51830_e85987_d_n9, assign51830_e85987_d_n10, assign51830_e85987_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51830_e85982: f64 = (locals.var_t2 * locals.var_t2);
        let assign51830_e85984: f64 = (assign51830_e85982 + 0.0001);
        let assign51830_e85985: f64 = (assign51830_e85984).sqrt();
        (assign51830_e85985, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign51830_e85985)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11,)
    }
};
        locals.var_vgd_eff = assign51830_e85987;
        locals.var_vgd_eff_dn3 = assign51830_e85987_d_n3;
        locals.var_vgd_eff_dn4 = assign51830_e85987_d_n4;
        locals.var_vgd_eff_dn5 = assign51830_e85987_d_n5;
        locals.var_vgd_eff_dn6 = assign51830_e85987_d_n6;
        locals.var_vgd_eff_dn7 = assign51830_e85987_d_n7;
        locals.var_vgd_eff_dn8 = assign51830_e85987_d_n8;
        locals.var_vgd_eff_dn9 = assign51830_e85987_d_n9;
        locals.var_vgd_eff_dn10 = assign51830_e85987_d_n10;
        locals.var_vgd_eff_dn11 = assign51830_e85987_d_n11;
        locals.var_vgd_eff_rv = 0.0;

        let assign51840_e85990: f64 = if p.p1295 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard787 = assign51840_e85990;
        locals.var_guard787_rv = 0.0;

        let (assign51850_e86026, assign51850_e86026_d_n3, assign51850_e86026_d_n4, assign51850_e86026_d_n5, assign51850_e86026_d_n6, assign51850_e86026_d_n7, assign51850_e86026_d_n8, assign51850_e86026_d_n9, assign51850_e86026_d_n10, assign51850_e86026_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard787 != 0.0)) {
        let assign51850_e86003: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51850_e86004: f64 = (locals.var_aigd_i - assign51850_e86003);
        let assign51850_e86008: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51850_e86009: f64 = (locals.var_aigd_i - assign51850_e86008);
        let assign51850_e86013: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51850_e86014: f64 = (locals.var_aigd_i - assign51850_e86013);
        let assign51850_e86015: f64 = (assign51850_e86009 * assign51850_e86014);
        let assign51850_e86018: f64 = (4.0 * 1e-6);
        let assign51850_e86020: f64 = (assign51850_e86018 * 1e-6);
        let assign51850_e86021: f64 = (assign51850_e86015 + assign51850_e86020);
        let assign51850_e86022: f64 = (assign51850_e86021).sqrt();
        let assign51850_e86023: f64 = (assign51850_e86004 + assign51850_e86022);
        let assign51850_e86024: f64 = (0.5 * assign51850_e86023);
        (assign51850_e86024, (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn3)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn3)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn3)))) / (2.0 * assign51850_e86022)))), (0.5 * ((locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)) + ((((locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)) * assign51850_e86014) + (assign51850_e86009 * (locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)))) / (2.0 * assign51850_e86022)))), (0.5 * ((locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)) + ((((locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)) * assign51850_e86014) + (assign51850_e86009 * (locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn6)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn6)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn6)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn7)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn7)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn7)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn8)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn8)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn8)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn9)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn9)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn9)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn10)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn10)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn10)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn11)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn11)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn11)))) / (2.0 * assign51850_e86022)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51850_e86026;
        locals.var_t1_dn3 = assign51850_e86026_d_n3;
        locals.var_t1_dn4 = assign51850_e86026_d_n4;
        locals.var_t1_dn5 = assign51850_e86026_d_n5;
        locals.var_t1_dn6 = assign51850_e86026_d_n6;
        locals.var_t1_dn7 = assign51850_e86026_d_n7;
        locals.var_t1_dn8 = assign51850_e86026_d_n8;
        locals.var_t1_dn9 = assign51850_e86026_d_n9;
        locals.var_t1_dn10 = assign51850_e86026_d_n10;
        locals.var_t1_dn11 = assign51850_e86026_d_n11;
        locals.var_t1_rv = 0.0;

        let assign51860_e86029: f64 = if locals.var_cigd_i < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard788 = assign51860_e86029;
        locals.var_guard788_rv = 0.0;

        let (assign51870_e86042,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard787 != 0.0)) && (locals.var_guard788 != 0.0)) {
        (0.01,)
    } else {
        (locals.var_cigd_i,)
    }
};
        locals.var_cigd_i = assign51870_e86042;
        locals.var_cigd_i_rv = 0.0;

        let (assign51880_e86058, assign51880_e86058_d_n3, assign51880_e86058_d_n4, assign51880_e86058_d_n5, assign51880_e86058_d_n6, assign51880_e86058_d_n7, assign51880_e86058_d_n8, assign51880_e86058_d_n9, assign51880_e86058_d_n10, assign51880_e86058_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard787 == 0.0)) {
        let assign51880_e86055: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51880_e86056: f64 = (locals.var_aigd_i - assign51880_e86055);
        (assign51880_e86056, (-(locals.var_bigd_i * locals.var_vgd_eff_dn3)), (locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)), (locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn6)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn7)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn8)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn9)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn10)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51880_e86058;
        locals.var_t1_dn3 = assign51880_e86058_d_n3;
        locals.var_t1_dn4 = assign51880_e86058_d_n4;
        locals.var_t1_dn5 = assign51880_e86058_d_n5;
        locals.var_t1_dn6 = assign51880_e86058_d_n6;
        locals.var_t1_dn7 = assign51880_e86058_d_n7;
        locals.var_t1_dn8 = assign51880_e86058_d_n8;
        locals.var_t1_dn9 = assign51880_e86058_d_n9;
        locals.var_t1_dn10 = assign51880_e86058_d_n10;
        locals.var_t1_dn11 = assign51880_e86058_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51890_e86071, assign51890_e86071_d_n3, assign51890_e86071_d_n4, assign51890_e86071_d_n5, assign51890_e86071_d_n6, assign51890_e86071_d_n7, assign51890_e86071_d_n8, assign51890_e86071_d_n9, assign51890_e86071_d_n10, assign51890_e86071_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51890_e86068: f64 = (locals.var_cigd_i * locals.var_vgd_eff);
        let assign51890_e86069: f64 = (1.0 + assign51890_e86068);
        (assign51890_e86069, (locals.var_cigd_i * locals.var_vgd_eff_dn3), (locals.var_cigd_i * locals.var_vgd_eff_dn4), (locals.var_cigd_i * locals.var_vgd_eff_dn5), (locals.var_cigd_i * locals.var_vgd_eff_dn6), (locals.var_cigd_i * locals.var_vgd_eff_dn7), (locals.var_cigd_i * locals.var_vgd_eff_dn8), (locals.var_cigd_i * locals.var_vgd_eff_dn9), (locals.var_cigd_i * locals.var_vgd_eff_dn10), (locals.var_cigd_i * locals.var_vgd_eff_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51890_e86071;
        locals.var_t2_dn3 = assign51890_e86071_d_n3;
        locals.var_t2_dn4 = assign51890_e86071_d_n4;
        locals.var_t2_dn5 = assign51890_e86071_d_n5;
        locals.var_t2_dn6 = assign51890_e86071_d_n6;
        locals.var_t2_dn7 = assign51890_e86071_d_n7;
        locals.var_t2_dn8 = assign51890_e86071_d_n8;
        locals.var_t2_dn9 = assign51890_e86071_d_n9;
        locals.var_t2_dn10 = assign51890_e86071_d_n10;
        locals.var_t2_dn11 = assign51890_e86071_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51900_e86084, assign51900_e86084_d_n3, assign51900_e86084_d_n4, assign51900_e86084_d_n5, assign51900_e86084_d_n6, assign51900_e86084_d_n7, assign51900_e86084_d_n8, assign51900_e86084_d_n9, assign51900_e86084_d_n10, assign51900_e86084_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51900_e86080: f64 = (locals.var_bechvbedge * locals.var_t1);
        let assign51900_e86082: f64 = (assign51900_e86080 * locals.var_t2);
        (assign51900_e86082, (((locals.var_bechvbedge * locals.var_t1_dn3) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn3)), (((locals.var_bechvbedge * locals.var_t1_dn4) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn4)), (((locals.var_bechvbedge * locals.var_t1_dn5) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn5)), (((locals.var_bechvbedge * locals.var_t1_dn6) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn6)), (((locals.var_bechvbedge * locals.var_t1_dn7) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn7)), (((locals.var_bechvbedge * locals.var_t1_dn8) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn8)), (((locals.var_bechvbedge * locals.var_t1_dn9) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn9)), (((locals.var_bechvbedge * locals.var_t1_dn10) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn10)), (((locals.var_bechvbedge * locals.var_t1_dn11) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51900_e86084;
        locals.var_t3_dn3 = assign51900_e86084_d_n3;
        locals.var_t3_dn4 = assign51900_e86084_d_n4;
        locals.var_t3_dn5 = assign51900_e86084_d_n5;
        locals.var_t3_dn6 = assign51900_e86084_d_n6;
        locals.var_t3_dn7 = assign51900_e86084_d_n7;
        locals.var_t3_dn8 = assign51900_e86084_d_n8;
        locals.var_t3_dn9 = assign51900_e86084_d_n9;
        locals.var_t3_dn10 = assign51900_e86084_d_n10;
        locals.var_t3_dn11 = assign51900_e86084_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51910_e86094, assign51910_e86094_d_n3, assign51910_e86094_d_n4, assign51910_e86094_d_n5, assign51910_e86094_d_n6, assign51910_e86094_d_n7, assign51910_e86094_d_n8, assign51910_e86094_d_n9, assign51910_e86094_d_n10, assign51910_e86094_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51910_e86092: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51910_e86092, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51910_e86094;
        locals.var_t4_dn3 = assign51910_e86094_d_n3;
        locals.var_t4_dn4 = assign51910_e86094_d_n4;
        locals.var_t4_dn5 = assign51910_e86094_d_n5;
        locals.var_t4_dn6 = assign51910_e86094_d_n6;
        locals.var_t4_dn7 = assign51910_e86094_d_n7;
        locals.var_t4_dn8 = assign51910_e86094_d_n8;
        locals.var_t4_dn9 = assign51910_e86094_d_n9;
        locals.var_t4_dn10 = assign51910_e86094_d_n10;
        locals.var_t4_dn11 = assign51910_e86094_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51990_e86166, assign51990_e86166_d_n3, assign51990_e86166_d_n4, assign51990_e86166_d_n5, assign51990_e86166_d_n6, assign51990_e86166_d_n7, assign51990_e86166_d_n8, assign51990_e86166_d_n9, assign51990_e86166_d_n10, assign51990_e86166_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign51990_e86164: f64 = (locals.var_cjs_t * locals.var_aseff);
        (assign51990_e86164, (locals.var_cjs_t * locals.var_aseff_dn3), ((locals.var_cjs_t_dn4 * locals.var_aseff) + (locals.var_cjs_t * locals.var_aseff_dn4)), ((locals.var_cjs_t_dn5 * locals.var_aseff) + (locals.var_cjs_t * locals.var_aseff_dn5)), (locals.var_cjs_t * locals.var_aseff_dn6), (locals.var_cjs_t * locals.var_aseff_dn7), (locals.var_cjs_t * locals.var_aseff_dn8), (locals.var_cjs_t * locals.var_aseff_dn9), (locals.var_cjs_t * locals.var_aseff_dn10), (locals.var_cjs_t * locals.var_aseff_dn11),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn3, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11,)
    }
};
        locals.var_czbs = assign51990_e86166;
        locals.var_czbs_dn3 = assign51990_e86166_d_n3;
        locals.var_czbs_dn4 = assign51990_e86166_d_n4;
        locals.var_czbs_dn5 = assign51990_e86166_d_n5;
        locals.var_czbs_dn6 = assign51990_e86166_d_n6;
        locals.var_czbs_dn7 = assign51990_e86166_d_n7;
        locals.var_czbs_dn8 = assign51990_e86166_d_n8;
        locals.var_czbs_dn9 = assign51990_e86166_d_n9;
        locals.var_czbs_dn10 = assign51990_e86166_d_n10;
        locals.var_czbs_dn11 = assign51990_e86166_d_n11;
        locals.var_czbs_rv = 0.0;

        let (assign52000_e86173, assign52000_e86173_d_n3, assign52000_e86173_d_n4, assign52000_e86173_d_n5, assign52000_e86173_d_n6, assign52000_e86173_d_n7, assign52000_e86173_d_n8, assign52000_e86173_d_n9, assign52000_e86173_d_n10, assign52000_e86173_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52000_e86171: f64 = (locals.var_cjsws_t * locals.var_pseff);
        (assign52000_e86171, (locals.var_cjsws_t * locals.var_pseff_dn3), ((locals.var_cjsws_t_dn4 * locals.var_pseff) + (locals.var_cjsws_t * locals.var_pseff_dn4)), ((locals.var_cjsws_t_dn5 * locals.var_pseff) + (locals.var_cjsws_t * locals.var_pseff_dn5)), (locals.var_cjsws_t * locals.var_pseff_dn6), (locals.var_cjsws_t * locals.var_pseff_dn7), (locals.var_cjsws_t * locals.var_pseff_dn8), (locals.var_cjsws_t * locals.var_pseff_dn9), (locals.var_cjsws_t * locals.var_pseff_dn10), (locals.var_cjsws_t * locals.var_pseff_dn11),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn3, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11,)
    }
};
        locals.var_czbssw = assign52000_e86173;
        locals.var_czbssw_dn3 = assign52000_e86173_d_n3;
        locals.var_czbssw_dn4 = assign52000_e86173_d_n4;
        locals.var_czbssw_dn5 = assign52000_e86173_d_n5;
        locals.var_czbssw_dn6 = assign52000_e86173_d_n6;
        locals.var_czbssw_dn7 = assign52000_e86173_d_n7;
        locals.var_czbssw_dn8 = assign52000_e86173_d_n8;
        locals.var_czbssw_dn9 = assign52000_e86173_d_n9;
        locals.var_czbssw_dn10 = assign52000_e86173_d_n10;
        locals.var_czbssw_dn11 = assign52000_e86173_d_n11;
        locals.var_czbssw_rv = 0.0;

        let (assign52010_e86182, assign52010_e86182_d_n4, assign52010_e86182_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52010_e86178: f64 = (locals.var_cjswgs_t * locals.var_weffcj);
        let assign52010_e86180: f64 = (assign52010_e86178 * p.p2);
        (assign52010_e86180, ((locals.var_cjswgs_t_dn4 * locals.var_weffcj) * p.p2), ((locals.var_cjswgs_t_dn5 * locals.var_weffcj) * p.p2),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5,)
    }
};
        locals.var_czbsswg = assign52010_e86182;
        locals.var_czbsswg_dn4 = assign52010_e86182_d_n4;
        locals.var_czbsswg_dn5 = assign52010_e86182_d_n5;
        locals.var_czbsswg_rv = 0.0;

        let (assign52020_e86190,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52020_e86187: f64 = (-p.p913);
        let assign52020_e86188: f64 = (0.1_f64).powf(assign52020_e86187);
        (assign52020_e86188,)
    } else {
        (locals.var_czbs_p1,)
    }
};
        locals.var_czbs_p1 = assign52020_e86190;
        locals.var_czbs_p1_rv = 0.0;

        let assign52030_e86193: f64 = if p.p913 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard789 = assign52030_e86193;
        locals.var_guard789_rv = 0.0;

        let (assign52040_e86203,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard789 != 0.0)) {
        let assign52040_e86200: f64 = (0.1_f64).ln();
        let assign52040_e86201: f64 = (1.5 - assign52040_e86200);
        (assign52040_e86201,)
    } else {
        (locals.var_czbs_p2,)
    }
};
        locals.var_czbs_p2 = assign52040_e86203;
        locals.var_czbs_p2_rv = 0.0;

        let (assign52050_e86227,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard789 == 0.0)) {
        let assign52050_e86212: f64 = (1.0 - p.p913);
        let assign52050_e86213: f64 = (1.0 / assign52050_e86212);
        let assign52050_e86217: f64 = (0.05 * p.p913);
        let assign52050_e86220: f64 = (1.0 + p.p913);
        let assign52050_e86221: f64 = (assign52050_e86217 * assign52050_e86220);
        let assign52050_e86223: f64 = (assign52050_e86221 * locals.var_czbs_p1);
        let assign52050_e86224: f64 = (1.0 - assign52050_e86223);
        let assign52050_e86225: f64 = (assign52050_e86213 * assign52050_e86224);
        (assign52050_e86225,)
    } else {
        (locals.var_czbs_p2,)
    }
};
        locals.var_czbs_p2 = assign52050_e86227;
        locals.var_czbs_p2_rv = 0.0;

        let (assign52060_e86235,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52060_e86232: f64 = (-p.p915);
        let assign52060_e86233: f64 = (0.1_f64).powf(assign52060_e86232);
        (assign52060_e86233,)
    } else {
        (locals.var_czbssw_p1,)
    }
};
        locals.var_czbssw_p1 = assign52060_e86235;
        locals.var_czbssw_p1_rv = 0.0;

        let assign52070_e86238: f64 = if p.p915 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard790 = assign52070_e86238;
        locals.var_guard790_rv = 0.0;

        let (assign52080_e86248,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard790 != 0.0)) {
        let assign52080_e86245: f64 = (0.1_f64).ln();
        let assign52080_e86246: f64 = (1.5 - assign52080_e86245);
        (assign52080_e86246,)
    } else {
        (locals.var_czbssw_p2,)
    }
};
        locals.var_czbssw_p2 = assign52080_e86248;
        locals.var_czbssw_p2_rv = 0.0;

        let (assign52090_e86272,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard790 == 0.0)) {
        let assign52090_e86257: f64 = (1.0 - p.p915);
        let assign52090_e86258: f64 = (1.0 / assign52090_e86257);
        let assign52090_e86262: f64 = (0.05 * p.p915);
        let assign52090_e86265: f64 = (1.0 + p.p915);
        let assign52090_e86266: f64 = (assign52090_e86262 * assign52090_e86265);
        let assign52090_e86268: f64 = (assign52090_e86266 * locals.var_czbssw_p1);
        let assign52090_e86269: f64 = (1.0 - assign52090_e86268);
        let assign52090_e86270: f64 = (assign52090_e86258 * assign52090_e86269);
        (assign52090_e86270,)
    } else {
        (locals.var_czbssw_p2,)
    }
};
        locals.var_czbssw_p2 = assign52090_e86272;
        locals.var_czbssw_p2_rv = 0.0;

        let (assign52100_e86280,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52100_e86277: f64 = (-p.p917);
        let assign52100_e86278: f64 = (0.1_f64).powf(assign52100_e86277);
        (assign52100_e86278,)
    } else {
        (locals.var_czbsswg_p1,)
    }
};
        locals.var_czbsswg_p1 = assign52100_e86280;
        locals.var_czbsswg_p1_rv = 0.0;

        let assign52110_e86283: f64 = if p.p917 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard791 = assign52110_e86283;
        locals.var_guard791_rv = 0.0;

        let (assign52120_e86293,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard791 != 0.0)) {
        let assign52120_e86290: f64 = (0.1_f64).ln();
        let assign52120_e86291: f64 = (1.5 - assign52120_e86290);
        (assign52120_e86291,)
    } else {
        (locals.var_czbsswg_p2,)
    }
};
        locals.var_czbsswg_p2 = assign52120_e86293;
        locals.var_czbsswg_p2_rv = 0.0;

        let (assign52130_e86317,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard791 == 0.0)) {
        let assign52130_e86302: f64 = (1.0 - p.p917);
        let assign52130_e86303: f64 = (1.0 / assign52130_e86302);
        let assign52130_e86307: f64 = (0.05 * p.p917);
        let assign52130_e86310: f64 = (1.0 + p.p917);
        let assign52130_e86311: f64 = (assign52130_e86307 * assign52130_e86310);
        let assign52130_e86313: f64 = (assign52130_e86311 * locals.var_czbsswg_p1);
        let assign52130_e86314: f64 = (1.0 - assign52130_e86313);
        let assign52130_e86315: f64 = (assign52130_e86303 * assign52130_e86314);
        (assign52130_e86315,)
    } else {
        (locals.var_czbsswg_p2,)
    }
};
        locals.var_czbsswg_p2 = assign52130_e86317;
        locals.var_czbsswg_p2_rv = 0.0;

        let assign52140_e86320: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard792 = assign52140_e86320;
        locals.var_guard792_rv = 0.0;

        let (assign52150_e86329, assign52150_e86329_d_n3, assign52150_e86329_d_n4, assign52150_e86329_d_n5, assign52150_e86329_d_n6, assign52150_e86329_d_n7, assign52150_e86329_d_n8, assign52150_e86329_d_n9, assign52150_e86329_d_n10, assign52150_e86329_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) {
        let assign52150_e86327: f64 = (locals.var_vbs_jct / locals.var_pbs_t);
        (assign52150_e86327, 0.0, (-((locals.var_vbs_jct * locals.var_pbs_t_dn4) / (locals.var_pbs_t * locals.var_pbs_t))), (-((locals.var_vbs_jct * locals.var_pbs_t_dn5) / (locals.var_pbs_t * locals.var_pbs_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbs_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbs_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52150_e86329;
        locals.var_t1_dn3 = assign52150_e86329_d_n3;
        locals.var_t1_dn4 = assign52150_e86329_d_n4;
        locals.var_t1_dn5 = assign52150_e86329_d_n5;
        locals.var_t1_dn6 = assign52150_e86329_d_n6;
        locals.var_t1_dn7 = assign52150_e86329_d_n7;
        locals.var_t1_dn8 = assign52150_e86329_d_n8;
        locals.var_t1_dn9 = assign52150_e86329_d_n9;
        locals.var_t1_dn10 = assign52150_e86329_d_n10;
        locals.var_t1_dn11 = assign52150_e86329_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52160_e86332: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard793 = assign52160_e86332;
        locals.var_guard793_rv = 0.0;

        let (assign52170_e86343, assign52170_e86343_d_n3, assign52170_e86343_d_n4, assign52170_e86343_d_n5, assign52170_e86343_d_n6, assign52170_e86343_d_n7, assign52170_e86343_d_n8, assign52170_e86343_d_n9, assign52170_e86343_d_n10, assign52170_e86343_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) {
        let assign52170_e86341: f64 = (1.0 - locals.var_t1);
        (assign52170_e86341, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52170_e86343;
        locals.var_arg_dn3 = assign52170_e86343_d_n3;
        locals.var_arg_dn4 = assign52170_e86343_d_n4;
        locals.var_arg_dn5 = assign52170_e86343_d_n5;
        locals.var_arg_dn6 = assign52170_e86343_d_n6;
        locals.var_arg_dn7 = assign52170_e86343_d_n7;
        locals.var_arg_dn8 = assign52170_e86343_d_n8;
        locals.var_arg_dn9 = assign52170_e86343_d_n9;
        locals.var_arg_dn10 = assign52170_e86343_d_n10;
        locals.var_arg_dn11 = assign52170_e86343_d_n11;
        locals.var_arg_rv = 0.0;

        let assign52180_e86346: f64 = if p.p913 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard794 = assign52180_e86346;
        locals.var_guard794_rv = 0.0;

        let assign52190_e86349: f64 = if p.p913 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard795 = assign52190_e86349;
        locals.var_guard795_rv = 0.0;

        let (assign52200_e86365, assign52200_e86365_d_n3, assign52200_e86365_d_n4, assign52200_e86365_d_n5, assign52200_e86365_d_n6, assign52200_e86365_d_n7, assign52200_e86365_d_n8, assign52200_e86365_d_n9, assign52200_e86365_d_n10, assign52200_e86365_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) {
        let assign52200_e86362: f64 = (locals.var_arg).sqrt();
        let assign52200_e86363: f64 = (1.0 / assign52200_e86362);
        (assign52200_e86363, (-((locals.var_arg_dn3 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn4 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn5 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn6 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn7 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn8 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn9 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn10 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn11 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52200_e86365;
        locals.var_sarg_dn3 = assign52200_e86365_d_n3;
        locals.var_sarg_dn4 = assign52200_e86365_d_n4;
        locals.var_sarg_dn5 = assign52200_e86365_d_n5;
        locals.var_sarg_dn6 = assign52200_e86365_d_n6;
        locals.var_sarg_dn7 = assign52200_e86365_d_n7;
        locals.var_sarg_dn8 = assign52200_e86365_d_n8;
        locals.var_sarg_dn9 = assign52200_e86365_d_n9;
        locals.var_sarg_dn10 = assign52200_e86365_d_n10;
        locals.var_sarg_dn11 = assign52200_e86365_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52210_e86384, assign52210_e86384_d_n3, assign52210_e86384_d_n4, assign52210_e86384_d_n5, assign52210_e86384_d_n6, assign52210_e86384_d_n7, assign52210_e86384_d_n8, assign52210_e86384_d_n9, assign52210_e86384_d_n10, assign52210_e86384_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 == 0.0)) {
        let assign52210_e86378: f64 = (-p.p913);
        let assign52210_e86380: f64 = (locals.var_arg).ln();
        let assign52210_e86381: f64 = (assign52210_e86378 * assign52210_e86380);
        let assign52210_e86382: f64 = { let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52210_e86382, ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52210_e86384;
        locals.var_sarg_dn3 = assign52210_e86384_d_n3;
        locals.var_sarg_dn4 = assign52210_e86384_d_n4;
        locals.var_sarg_dn5 = assign52210_e86384_d_n5;
        locals.var_sarg_dn6 = assign52210_e86384_d_n6;
        locals.var_sarg_dn7 = assign52210_e86384_d_n7;
        locals.var_sarg_dn8 = assign52210_e86384_d_n8;
        locals.var_sarg_dn9 = assign52210_e86384_d_n9;
        locals.var_sarg_dn10 = assign52210_e86384_d_n10;
        locals.var_sarg_dn11 = assign52210_e86384_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52220_e86407, assign52220_e86407_d_n3, assign52220_e86407_d_n4, assign52220_e86407_d_n5, assign52220_e86407_d_n6, assign52220_e86407_d_n7, assign52220_e86407_d_n8, assign52220_e86407_d_n9, assign52220_e86407_d_n10, assign52220_e86407_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign52220_e86395: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign52220_e86399: f64 = (locals.var_arg * locals.var_sarg);
        let assign52220_e86400: f64 = (1.0 - assign52220_e86399);
        let assign52220_e86401: f64 = (assign52220_e86395 * assign52220_e86400);
        let assign52220_e86404: f64 = (1.0 - p.p913);
        let assign52220_e86405: f64 = (assign52220_e86401 / assign52220_e86404);
        (assign52220_e86405, ((((locals.var_pbs_t * locals.var_czbs_dn3) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52220_e86404), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52220_e86404), (((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn6) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn7) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn8) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn9) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn10) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn11) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52220_e86404),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52220_e86407;
        locals.var_qbsj1_dn3 = assign52220_e86407_d_n3;
        locals.var_qbsj1_dn4 = assign52220_e86407_d_n4;
        locals.var_qbsj1_dn5 = assign52220_e86407_d_n5;
        locals.var_qbsj1_dn6 = assign52220_e86407_d_n6;
        locals.var_qbsj1_dn7 = assign52220_e86407_d_n7;
        locals.var_qbsj1_dn8 = assign52220_e86407_d_n8;
        locals.var_qbsj1_dn9 = assign52220_e86407_d_n9;
        locals.var_qbsj1_dn10 = assign52220_e86407_d_n10;
        locals.var_qbsj1_dn11 = assign52220_e86407_d_n11;
        locals.var_qbsj1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_180(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52230_e86425, assign52230_e86425_d_n3, assign52230_e86425_d_n4, assign52230_e86425_d_n5, assign52230_e86425_d_n6, assign52230_e86425_d_n7, assign52230_e86425_d_n8, assign52230_e86425_d_n9, assign52230_e86425_d_n10, assign52230_e86425_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 == 0.0)) {
        let assign52230_e86419: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign52230_e86421: f64 = (locals.var_arg).ln();
        let assign52230_e86422: f64 = (-assign52230_e86421);
        let assign52230_e86423: f64 = (assign52230_e86419 * assign52230_e86422);
        (assign52230_e86423, (((locals.var_pbs_t * locals.var_czbs_dn3) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn6) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn7) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn8) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn9) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn10) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn11) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52230_e86425;
        locals.var_qbsj1_dn3 = assign52230_e86425_d_n3;
        locals.var_qbsj1_dn4 = assign52230_e86425_d_n4;
        locals.var_qbsj1_dn5 = assign52230_e86425_d_n5;
        locals.var_qbsj1_dn6 = assign52230_e86425_d_n6;
        locals.var_qbsj1_dn7 = assign52230_e86425_d_n7;
        locals.var_qbsj1_dn8 = assign52230_e86425_d_n8;
        locals.var_qbsj1_dn9 = assign52230_e86425_d_n9;
        locals.var_qbsj1_dn10 = assign52230_e86425_d_n10;
        locals.var_qbsj1_dn11 = assign52230_e86425_d_n11;
        locals.var_qbsj1_rv = 0.0;

        let (assign52240_e86451, assign52240_e86451_d_n3, assign52240_e86451_d_n4, assign52240_e86451_d_n5, assign52240_e86451_d_n6, assign52240_e86451_d_n7, assign52240_e86451_d_n8, assign52240_e86451_d_n9, assign52240_e86451_d_n10, assign52240_e86451_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 == 0.0)) {
        let assign52240_e86436: f64 = (locals.var_t1 - 1.0);
        let assign52240_e86437: f64 = (locals.var_czbs_p1 * assign52240_e86436);
        let assign52240_e86440: f64 = (5.0 * p.p913);
        let assign52240_e86443: f64 = (locals.var_t1 - 1.0);
        let assign52240_e86444: f64 = (assign52240_e86440 * assign52240_e86443);
        let assign52240_e86447: f64 = (1.0 + p.p913);
        let assign52240_e86448: f64 = (assign52240_e86444 + assign52240_e86447);
        let assign52240_e86449: f64 = (assign52240_e86437 * assign52240_e86448);
        (assign52240_e86449, (((locals.var_czbs_p1 * locals.var_t1_dn3) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn3))), (((locals.var_czbs_p1 * locals.var_t1_dn4) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn4))), (((locals.var_czbs_p1 * locals.var_t1_dn5) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn5))), (((locals.var_czbs_p1 * locals.var_t1_dn6) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn6))), (((locals.var_czbs_p1 * locals.var_t1_dn7) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn7))), (((locals.var_czbs_p1 * locals.var_t1_dn8) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn8))), (((locals.var_czbs_p1 * locals.var_t1_dn9) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn9))), (((locals.var_czbs_p1 * locals.var_t1_dn10) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn10))), (((locals.var_czbs_p1 * locals.var_t1_dn11) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52240_e86451;
        locals.var_t2_dn3 = assign52240_e86451_d_n3;
        locals.var_t2_dn4 = assign52240_e86451_d_n4;
        locals.var_t2_dn5 = assign52240_e86451_d_n5;
        locals.var_t2_dn6 = assign52240_e86451_d_n6;
        locals.var_t2_dn7 = assign52240_e86451_d_n7;
        locals.var_t2_dn8 = assign52240_e86451_d_n8;
        locals.var_t2_dn9 = assign52240_e86451_d_n9;
        locals.var_t2_dn10 = assign52240_e86451_d_n10;
        locals.var_t2_dn11 = assign52240_e86451_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign52250_e86467, assign52250_e86467_d_n3, assign52250_e86467_d_n4, assign52250_e86467_d_n5, assign52250_e86467_d_n6, assign52250_e86467_d_n7, assign52250_e86467_d_n8, assign52250_e86467_d_n9, assign52250_e86467_d_n10, assign52250_e86467_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 == 0.0)) {
        let assign52250_e86461: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign52250_e86464: f64 = (locals.var_t2 + locals.var_czbs_p2);
        let assign52250_e86465: f64 = (assign52250_e86461 * assign52250_e86464);
        (assign52250_e86465, (((locals.var_pbs_t * locals.var_czbs_dn3) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn3)), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn4)), ((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn5)), (((locals.var_pbs_t * locals.var_czbs_dn6) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn6)), (((locals.var_pbs_t * locals.var_czbs_dn7) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn7)), (((locals.var_pbs_t * locals.var_czbs_dn8) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn8)), (((locals.var_pbs_t * locals.var_czbs_dn9) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn9)), (((locals.var_pbs_t * locals.var_czbs_dn10) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn10)), (((locals.var_pbs_t * locals.var_czbs_dn11) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52250_e86467;
        locals.var_qbsj1_dn3 = assign52250_e86467_d_n3;
        locals.var_qbsj1_dn4 = assign52250_e86467_d_n4;
        locals.var_qbsj1_dn5 = assign52250_e86467_d_n5;
        locals.var_qbsj1_dn6 = assign52250_e86467_d_n6;
        locals.var_qbsj1_dn7 = assign52250_e86467_d_n7;
        locals.var_qbsj1_dn8 = assign52250_e86467_d_n8;
        locals.var_qbsj1_dn9 = assign52250_e86467_d_n9;
        locals.var_qbsj1_dn10 = assign52250_e86467_d_n10;
        locals.var_qbsj1_dn11 = assign52250_e86467_d_n11;
        locals.var_qbsj1_rv = 0.0;

        let (assign52260_e86475, assign52260_e86475_d_n3, assign52260_e86475_d_n4, assign52260_e86475_d_n5, assign52260_e86475_d_n6, assign52260_e86475_d_n7, assign52260_e86475_d_n8, assign52260_e86475_d_n9, assign52260_e86475_d_n10, assign52260_e86475_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard792 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52260_e86475;
        locals.var_qbsj1_dn3 = assign52260_e86475_d_n3;
        locals.var_qbsj1_dn4 = assign52260_e86475_d_n4;
        locals.var_qbsj1_dn5 = assign52260_e86475_d_n5;
        locals.var_qbsj1_dn6 = assign52260_e86475_d_n6;
        locals.var_qbsj1_dn7 = assign52260_e86475_d_n7;
        locals.var_qbsj1_dn8 = assign52260_e86475_d_n8;
        locals.var_qbsj1_dn9 = assign52260_e86475_d_n9;
        locals.var_qbsj1_dn10 = assign52260_e86475_d_n10;
        locals.var_qbsj1_dn11 = assign52260_e86475_d_n11;
        locals.var_qbsj1_rv = 0.0;

        let assign52270_e86478: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard796 = assign52270_e86478;
        locals.var_guard796_rv = 0.0;

        let (assign52280_e86487, assign52280_e86487_d_n3, assign52280_e86487_d_n4, assign52280_e86487_d_n5, assign52280_e86487_d_n6, assign52280_e86487_d_n7, assign52280_e86487_d_n8, assign52280_e86487_d_n9, assign52280_e86487_d_n10, assign52280_e86487_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) {
        let assign52280_e86485: f64 = (locals.var_vbs_jct / locals.var_pbsws_t);
        (assign52280_e86485, 0.0, (-((locals.var_vbs_jct * locals.var_pbsws_t_dn4) / (locals.var_pbsws_t * locals.var_pbsws_t))), (-((locals.var_vbs_jct * locals.var_pbsws_t_dn5) / (locals.var_pbsws_t * locals.var_pbsws_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbsws_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbsws_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52280_e86487;
        locals.var_t1_dn3 = assign52280_e86487_d_n3;
        locals.var_t1_dn4 = assign52280_e86487_d_n4;
        locals.var_t1_dn5 = assign52280_e86487_d_n5;
        locals.var_t1_dn6 = assign52280_e86487_d_n6;
        locals.var_t1_dn7 = assign52280_e86487_d_n7;
        locals.var_t1_dn8 = assign52280_e86487_d_n8;
        locals.var_t1_dn9 = assign52280_e86487_d_n9;
        locals.var_t1_dn10 = assign52280_e86487_d_n10;
        locals.var_t1_dn11 = assign52280_e86487_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52290_e86490: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard797 = assign52290_e86490;
        locals.var_guard797_rv = 0.0;

        let (assign52300_e86501, assign52300_e86501_d_n3, assign52300_e86501_d_n4, assign52300_e86501_d_n5, assign52300_e86501_d_n6, assign52300_e86501_d_n7, assign52300_e86501_d_n8, assign52300_e86501_d_n9, assign52300_e86501_d_n10, assign52300_e86501_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) {
        let assign52300_e86499: f64 = (1.0 - locals.var_t1);
        (assign52300_e86499, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52300_e86501;
        locals.var_arg_dn3 = assign52300_e86501_d_n3;
        locals.var_arg_dn4 = assign52300_e86501_d_n4;
        locals.var_arg_dn5 = assign52300_e86501_d_n5;
        locals.var_arg_dn6 = assign52300_e86501_d_n6;
        locals.var_arg_dn7 = assign52300_e86501_d_n7;
        locals.var_arg_dn8 = assign52300_e86501_d_n8;
        locals.var_arg_dn9 = assign52300_e86501_d_n9;
        locals.var_arg_dn10 = assign52300_e86501_d_n10;
        locals.var_arg_dn11 = assign52300_e86501_d_n11;
        locals.var_arg_rv = 0.0;

        let assign52310_e86504: f64 = if p.p915 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard798 = assign52310_e86504;
        locals.var_guard798_rv = 0.0;

        let assign52320_e86507: f64 = if p.p915 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard799 = assign52320_e86507;
        locals.var_guard799_rv = 0.0;

        let (assign52330_e86523, assign52330_e86523_d_n3, assign52330_e86523_d_n4, assign52330_e86523_d_n5, assign52330_e86523_d_n6, assign52330_e86523_d_n7, assign52330_e86523_d_n8, assign52330_e86523_d_n9, assign52330_e86523_d_n10, assign52330_e86523_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) {
        let assign52330_e86520: f64 = (locals.var_arg).sqrt();
        let assign52330_e86521: f64 = (1.0 / assign52330_e86520);
        (assign52330_e86521, (-((locals.var_arg_dn3 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn4 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn5 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn6 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn7 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn8 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn9 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn10 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn11 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52330_e86523;
        locals.var_sarg_dn3 = assign52330_e86523_d_n3;
        locals.var_sarg_dn4 = assign52330_e86523_d_n4;
        locals.var_sarg_dn5 = assign52330_e86523_d_n5;
        locals.var_sarg_dn6 = assign52330_e86523_d_n6;
        locals.var_sarg_dn7 = assign52330_e86523_d_n7;
        locals.var_sarg_dn8 = assign52330_e86523_d_n8;
        locals.var_sarg_dn9 = assign52330_e86523_d_n9;
        locals.var_sarg_dn10 = assign52330_e86523_d_n10;
        locals.var_sarg_dn11 = assign52330_e86523_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52340_e86542, assign52340_e86542_d_n3, assign52340_e86542_d_n4, assign52340_e86542_d_n5, assign52340_e86542_d_n6, assign52340_e86542_d_n7, assign52340_e86542_d_n8, assign52340_e86542_d_n9, assign52340_e86542_d_n10, assign52340_e86542_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 == 0.0)) {
        let assign52340_e86536: f64 = (-p.p915);
        let assign52340_e86538: f64 = (locals.var_arg).ln();
        let assign52340_e86539: f64 = (assign52340_e86536 * assign52340_e86538);
        let assign52340_e86540: f64 = { let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52340_e86540, ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52340_e86542;
        locals.var_sarg_dn3 = assign52340_e86542_d_n3;
        locals.var_sarg_dn4 = assign52340_e86542_d_n4;
        locals.var_sarg_dn5 = assign52340_e86542_d_n5;
        locals.var_sarg_dn6 = assign52340_e86542_d_n6;
        locals.var_sarg_dn7 = assign52340_e86542_d_n7;
        locals.var_sarg_dn8 = assign52340_e86542_d_n8;
        locals.var_sarg_dn9 = assign52340_e86542_d_n9;
        locals.var_sarg_dn10 = assign52340_e86542_d_n10;
        locals.var_sarg_dn11 = assign52340_e86542_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52350_e86565, assign52350_e86565_d_n3, assign52350_e86565_d_n4, assign52350_e86565_d_n5, assign52350_e86565_d_n6, assign52350_e86565_d_n7, assign52350_e86565_d_n8, assign52350_e86565_d_n9, assign52350_e86565_d_n10, assign52350_e86565_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign52350_e86553: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign52350_e86557: f64 = (locals.var_arg * locals.var_sarg);
        let assign52350_e86558: f64 = (1.0 - assign52350_e86557);
        let assign52350_e86559: f64 = (assign52350_e86553 * assign52350_e86558);
        let assign52350_e86562: f64 = (1.0 - p.p915);
        let assign52350_e86563: f64 = (assign52350_e86559 / assign52350_e86562);
        (assign52350_e86563, ((((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52350_e86562), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52350_e86562), (((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52350_e86562),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52350_e86565;
        locals.var_qbsj2_dn3 = assign52350_e86565_d_n3;
        locals.var_qbsj2_dn4 = assign52350_e86565_d_n4;
        locals.var_qbsj2_dn5 = assign52350_e86565_d_n5;
        locals.var_qbsj2_dn6 = assign52350_e86565_d_n6;
        locals.var_qbsj2_dn7 = assign52350_e86565_d_n7;
        locals.var_qbsj2_dn8 = assign52350_e86565_d_n8;
        locals.var_qbsj2_dn9 = assign52350_e86565_d_n9;
        locals.var_qbsj2_dn10 = assign52350_e86565_d_n10;
        locals.var_qbsj2_dn11 = assign52350_e86565_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let (assign52360_e86583, assign52360_e86583_d_n3, assign52360_e86583_d_n4, assign52360_e86583_d_n5, assign52360_e86583_d_n6, assign52360_e86583_d_n7, assign52360_e86583_d_n8, assign52360_e86583_d_n9, assign52360_e86583_d_n10, assign52360_e86583_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 == 0.0)) {
        let assign52360_e86577: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign52360_e86579: f64 = (locals.var_arg).ln();
        let assign52360_e86580: f64 = (-assign52360_e86579);
        let assign52360_e86581: f64 = (assign52360_e86577 * assign52360_e86580);
        (assign52360_e86581, (((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52360_e86583;
        locals.var_qbsj2_dn3 = assign52360_e86583_d_n3;
        locals.var_qbsj2_dn4 = assign52360_e86583_d_n4;
        locals.var_qbsj2_dn5 = assign52360_e86583_d_n5;
        locals.var_qbsj2_dn6 = assign52360_e86583_d_n6;
        locals.var_qbsj2_dn7 = assign52360_e86583_d_n7;
        locals.var_qbsj2_dn8 = assign52360_e86583_d_n8;
        locals.var_qbsj2_dn9 = assign52360_e86583_d_n9;
        locals.var_qbsj2_dn10 = assign52360_e86583_d_n10;
        locals.var_qbsj2_dn11 = assign52360_e86583_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let (assign52370_e86609, assign52370_e86609_d_n3, assign52370_e86609_d_n4, assign52370_e86609_d_n5, assign52370_e86609_d_n6, assign52370_e86609_d_n7, assign52370_e86609_d_n8, assign52370_e86609_d_n9, assign52370_e86609_d_n10, assign52370_e86609_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 == 0.0)) {
        let assign52370_e86594: f64 = (locals.var_t1 - 1.0);
        let assign52370_e86595: f64 = (locals.var_czbssw_p1 * assign52370_e86594);
        let assign52370_e86598: f64 = (5.0 * p.p915);
        let assign52370_e86601: f64 = (locals.var_t1 - 1.0);
        let assign52370_e86602: f64 = (assign52370_e86598 * assign52370_e86601);
        let assign52370_e86605: f64 = (1.0 + p.p915);
        let assign52370_e86606: f64 = (assign52370_e86602 + assign52370_e86605);
        let assign52370_e86607: f64 = (assign52370_e86595 * assign52370_e86606);
        (assign52370_e86607, (((locals.var_czbssw_p1 * locals.var_t1_dn3) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn3))), (((locals.var_czbssw_p1 * locals.var_t1_dn4) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn4))), (((locals.var_czbssw_p1 * locals.var_t1_dn5) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn5))), (((locals.var_czbssw_p1 * locals.var_t1_dn6) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn6))), (((locals.var_czbssw_p1 * locals.var_t1_dn7) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn7))), (((locals.var_czbssw_p1 * locals.var_t1_dn8) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn8))), (((locals.var_czbssw_p1 * locals.var_t1_dn9) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn9))), (((locals.var_czbssw_p1 * locals.var_t1_dn10) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn10))), (((locals.var_czbssw_p1 * locals.var_t1_dn11) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52370_e86609;
        locals.var_t2_dn3 = assign52370_e86609_d_n3;
        locals.var_t2_dn4 = assign52370_e86609_d_n4;
        locals.var_t2_dn5 = assign52370_e86609_d_n5;
        locals.var_t2_dn6 = assign52370_e86609_d_n6;
        locals.var_t2_dn7 = assign52370_e86609_d_n7;
        locals.var_t2_dn8 = assign52370_e86609_d_n8;
        locals.var_t2_dn9 = assign52370_e86609_d_n9;
        locals.var_t2_dn10 = assign52370_e86609_d_n10;
        locals.var_t2_dn11 = assign52370_e86609_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign52380_e86625, assign52380_e86625_d_n3, assign52380_e86625_d_n4, assign52380_e86625_d_n5, assign52380_e86625_d_n6, assign52380_e86625_d_n7, assign52380_e86625_d_n8, assign52380_e86625_d_n9, assign52380_e86625_d_n10, assign52380_e86625_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 == 0.0)) {
        let assign52380_e86619: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign52380_e86622: f64 = (locals.var_t2 + locals.var_czbssw_p2);
        let assign52380_e86623: f64 = (assign52380_e86619 * assign52380_e86622);
        (assign52380_e86623, (((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn3)), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn4)), ((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn5)), (((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn6)), (((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn7)), (((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn8)), (((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn9)), (((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn10)), (((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52380_e86625;
        locals.var_qbsj2_dn3 = assign52380_e86625_d_n3;
        locals.var_qbsj2_dn4 = assign52380_e86625_d_n4;
        locals.var_qbsj2_dn5 = assign52380_e86625_d_n5;
        locals.var_qbsj2_dn6 = assign52380_e86625_d_n6;
        locals.var_qbsj2_dn7 = assign52380_e86625_d_n7;
        locals.var_qbsj2_dn8 = assign52380_e86625_d_n8;
        locals.var_qbsj2_dn9 = assign52380_e86625_d_n9;
        locals.var_qbsj2_dn10 = assign52380_e86625_d_n10;
        locals.var_qbsj2_dn11 = assign52380_e86625_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let (assign52390_e86633, assign52390_e86633_d_n3, assign52390_e86633_d_n4, assign52390_e86633_d_n5, assign52390_e86633_d_n6, assign52390_e86633_d_n7, assign52390_e86633_d_n8, assign52390_e86633_d_n9, assign52390_e86633_d_n10, assign52390_e86633_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard796 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52390_e86633;
        locals.var_qbsj2_dn3 = assign52390_e86633_d_n3;
        locals.var_qbsj2_dn4 = assign52390_e86633_d_n4;
        locals.var_qbsj2_dn5 = assign52390_e86633_d_n5;
        locals.var_qbsj2_dn6 = assign52390_e86633_d_n6;
        locals.var_qbsj2_dn7 = assign52390_e86633_d_n7;
        locals.var_qbsj2_dn8 = assign52390_e86633_d_n8;
        locals.var_qbsj2_dn9 = assign52390_e86633_d_n9;
        locals.var_qbsj2_dn10 = assign52390_e86633_d_n10;
        locals.var_qbsj2_dn11 = assign52390_e86633_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let assign52400_e86636: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard800 = assign52400_e86636;
        locals.var_guard800_rv = 0.0;

        let (assign52410_e86645, assign52410_e86645_d_n3, assign52410_e86645_d_n4, assign52410_e86645_d_n5, assign52410_e86645_d_n6, assign52410_e86645_d_n7, assign52410_e86645_d_n8, assign52410_e86645_d_n9, assign52410_e86645_d_n10, assign52410_e86645_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) {
        let assign52410_e86643: f64 = (locals.var_vbs_jct / locals.var_pbswgs_t);
        (assign52410_e86643, 0.0, (-((locals.var_vbs_jct * locals.var_pbswgs_t_dn4) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), (-((locals.var_vbs_jct * locals.var_pbswgs_t_dn5) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbswgs_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbswgs_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52410_e86645;
        locals.var_t1_dn3 = assign52410_e86645_d_n3;
        locals.var_t1_dn4 = assign52410_e86645_d_n4;
        locals.var_t1_dn5 = assign52410_e86645_d_n5;
        locals.var_t1_dn6 = assign52410_e86645_d_n6;
        locals.var_t1_dn7 = assign52410_e86645_d_n7;
        locals.var_t1_dn8 = assign52410_e86645_d_n8;
        locals.var_t1_dn9 = assign52410_e86645_d_n9;
        locals.var_t1_dn10 = assign52410_e86645_d_n10;
        locals.var_t1_dn11 = assign52410_e86645_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52420_e86648: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard801 = assign52420_e86648;
        locals.var_guard801_rv = 0.0;

        let (assign52430_e86659, assign52430_e86659_d_n3, assign52430_e86659_d_n4, assign52430_e86659_d_n5, assign52430_e86659_d_n6, assign52430_e86659_d_n7, assign52430_e86659_d_n8, assign52430_e86659_d_n9, assign52430_e86659_d_n10, assign52430_e86659_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) {
        let assign52430_e86657: f64 = (1.0 - locals.var_t1);
        (assign52430_e86657, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52430_e86659;
        locals.var_arg_dn3 = assign52430_e86659_d_n3;
        locals.var_arg_dn4 = assign52430_e86659_d_n4;
        locals.var_arg_dn5 = assign52430_e86659_d_n5;
        locals.var_arg_dn6 = assign52430_e86659_d_n6;
        locals.var_arg_dn7 = assign52430_e86659_d_n7;
        locals.var_arg_dn8 = assign52430_e86659_d_n8;
        locals.var_arg_dn9 = assign52430_e86659_d_n9;
        locals.var_arg_dn10 = assign52430_e86659_d_n10;
        locals.var_arg_dn11 = assign52430_e86659_d_n11;
        locals.var_arg_rv = 0.0;

        let assign52440_e86662: f64 = if p.p917 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard802 = assign52440_e86662;
        locals.var_guard802_rv = 0.0;

        let assign52450_e86665: f64 = if p.p917 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard803 = assign52450_e86665;
        locals.var_guard803_rv = 0.0;

        let (assign52460_e86681, assign52460_e86681_d_n3, assign52460_e86681_d_n4, assign52460_e86681_d_n5, assign52460_e86681_d_n6, assign52460_e86681_d_n7, assign52460_e86681_d_n8, assign52460_e86681_d_n9, assign52460_e86681_d_n10, assign52460_e86681_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) {
        let assign52460_e86678: f64 = (locals.var_arg).sqrt();
        let assign52460_e86679: f64 = (1.0 / assign52460_e86678);
        (assign52460_e86679, (-((locals.var_arg_dn3 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn4 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn5 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn6 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn7 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn8 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn9 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn10 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn11 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52460_e86681;
        locals.var_sarg_dn3 = assign52460_e86681_d_n3;
        locals.var_sarg_dn4 = assign52460_e86681_d_n4;
        locals.var_sarg_dn5 = assign52460_e86681_d_n5;
        locals.var_sarg_dn6 = assign52460_e86681_d_n6;
        locals.var_sarg_dn7 = assign52460_e86681_d_n7;
        locals.var_sarg_dn8 = assign52460_e86681_d_n8;
        locals.var_sarg_dn9 = assign52460_e86681_d_n9;
        locals.var_sarg_dn10 = assign52460_e86681_d_n10;
        locals.var_sarg_dn11 = assign52460_e86681_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52470_e86700, assign52470_e86700_d_n3, assign52470_e86700_d_n4, assign52470_e86700_d_n5, assign52470_e86700_d_n6, assign52470_e86700_d_n7, assign52470_e86700_d_n8, assign52470_e86700_d_n9, assign52470_e86700_d_n10, assign52470_e86700_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 == 0.0)) {
        let assign52470_e86694: f64 = (-p.p917);
        let assign52470_e86696: f64 = (locals.var_arg).ln();
        let assign52470_e86697: f64 = (assign52470_e86694 * assign52470_e86696);
        let assign52470_e86698: f64 = { let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52470_e86698, ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52470_e86700;
        locals.var_sarg_dn3 = assign52470_e86700_d_n3;
        locals.var_sarg_dn4 = assign52470_e86700_d_n4;
        locals.var_sarg_dn5 = assign52470_e86700_d_n5;
        locals.var_sarg_dn6 = assign52470_e86700_d_n6;
        locals.var_sarg_dn7 = assign52470_e86700_d_n7;
        locals.var_sarg_dn8 = assign52470_e86700_d_n8;
        locals.var_sarg_dn9 = assign52470_e86700_d_n9;
        locals.var_sarg_dn10 = assign52470_e86700_d_n10;
        locals.var_sarg_dn11 = assign52470_e86700_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52480_e86723, assign52480_e86723_d_n3, assign52480_e86723_d_n4, assign52480_e86723_d_n5, assign52480_e86723_d_n6, assign52480_e86723_d_n7, assign52480_e86723_d_n8, assign52480_e86723_d_n9, assign52480_e86723_d_n10, assign52480_e86723_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign52480_e86711: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign52480_e86715: f64 = (locals.var_arg * locals.var_sarg);
        let assign52480_e86716: f64 = (1.0 - assign52480_e86715);
        let assign52480_e86717: f64 = (assign52480_e86711 * assign52480_e86716);
        let assign52480_e86720: f64 = (1.0 - p.p917);
        let assign52480_e86721: f64 = (assign52480_e86717 / assign52480_e86720);
        (assign52480_e86721, ((assign52480_e86711 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign52480_e86720), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign52480_e86716) + (assign52480_e86711 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52480_e86720), (((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign52480_e86716) + (assign52480_e86711 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11)))) / assign52480_e86720),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52480_e86723;
        locals.var_qbsj3_dn3 = assign52480_e86723_d_n3;
        locals.var_qbsj3_dn4 = assign52480_e86723_d_n4;
        locals.var_qbsj3_dn5 = assign52480_e86723_d_n5;
        locals.var_qbsj3_dn6 = assign52480_e86723_d_n6;
        locals.var_qbsj3_dn7 = assign52480_e86723_d_n7;
        locals.var_qbsj3_dn8 = assign52480_e86723_d_n8;
        locals.var_qbsj3_dn9 = assign52480_e86723_d_n9;
        locals.var_qbsj3_dn10 = assign52480_e86723_d_n10;
        locals.var_qbsj3_dn11 = assign52480_e86723_d_n11;
        locals.var_qbsj3_rv = 0.0;

        let (assign52490_e86741, assign52490_e86741_d_n3, assign52490_e86741_d_n4, assign52490_e86741_d_n5, assign52490_e86741_d_n6, assign52490_e86741_d_n7, assign52490_e86741_d_n8, assign52490_e86741_d_n9, assign52490_e86741_d_n10, assign52490_e86741_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 == 0.0)) {
        let assign52490_e86735: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign52490_e86737: f64 = (locals.var_arg).ln();
        let assign52490_e86738: f64 = (-assign52490_e86737);
        let assign52490_e86739: f64 = (assign52490_e86735 * assign52490_e86738);
        (assign52490_e86739, (assign52490_e86735 * (-(locals.var_arg_dn3 / locals.var_arg))), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign52490_e86738) + (assign52490_e86735 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign52490_e86738) + (assign52490_e86735 * (-(locals.var_arg_dn5 / locals.var_arg)))), (assign52490_e86735 * (-(locals.var_arg_dn6 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn7 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn8 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn9 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn10 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52490_e86741;
        locals.var_qbsj3_dn3 = assign52490_e86741_d_n3;
        locals.var_qbsj3_dn4 = assign52490_e86741_d_n4;
        locals.var_qbsj3_dn5 = assign52490_e86741_d_n5;
        locals.var_qbsj3_dn6 = assign52490_e86741_d_n6;
        locals.var_qbsj3_dn7 = assign52490_e86741_d_n7;
        locals.var_qbsj3_dn8 = assign52490_e86741_d_n8;
        locals.var_qbsj3_dn9 = assign52490_e86741_d_n9;
        locals.var_qbsj3_dn10 = assign52490_e86741_d_n10;
        locals.var_qbsj3_dn11 = assign52490_e86741_d_n11;
        locals.var_qbsj3_rv = 0.0;

        let (assign52500_e86767, assign52500_e86767_d_n3, assign52500_e86767_d_n4, assign52500_e86767_d_n5, assign52500_e86767_d_n6, assign52500_e86767_d_n7, assign52500_e86767_d_n8, assign52500_e86767_d_n9, assign52500_e86767_d_n10, assign52500_e86767_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 == 0.0)) {
        let assign52500_e86752: f64 = (locals.var_t1 - 1.0);
        let assign52500_e86753: f64 = (locals.var_czbsswg_p1 * assign52500_e86752);
        let assign52500_e86756: f64 = (5.0 * p.p917);
        let assign52500_e86759: f64 = (locals.var_t1 - 1.0);
        let assign52500_e86760: f64 = (assign52500_e86756 * assign52500_e86759);
        let assign52500_e86763: f64 = (1.0 + p.p917);
        let assign52500_e86764: f64 = (assign52500_e86760 + assign52500_e86763);
        let assign52500_e86765: f64 = (assign52500_e86753 * assign52500_e86764);
        (assign52500_e86765, (((locals.var_czbsswg_p1 * locals.var_t1_dn3) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn3))), (((locals.var_czbsswg_p1 * locals.var_t1_dn4) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn4))), (((locals.var_czbsswg_p1 * locals.var_t1_dn5) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn5))), (((locals.var_czbsswg_p1 * locals.var_t1_dn6) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn6))), (((locals.var_czbsswg_p1 * locals.var_t1_dn7) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn7))), (((locals.var_czbsswg_p1 * locals.var_t1_dn8) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn8))), (((locals.var_czbsswg_p1 * locals.var_t1_dn9) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn9))), (((locals.var_czbsswg_p1 * locals.var_t1_dn10) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn10))), (((locals.var_czbsswg_p1 * locals.var_t1_dn11) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52500_e86767;
        locals.var_t2_dn3 = assign52500_e86767_d_n3;
        locals.var_t2_dn4 = assign52500_e86767_d_n4;
        locals.var_t2_dn5 = assign52500_e86767_d_n5;
        locals.var_t2_dn6 = assign52500_e86767_d_n6;
        locals.var_t2_dn7 = assign52500_e86767_d_n7;
        locals.var_t2_dn8 = assign52500_e86767_d_n8;
        locals.var_t2_dn9 = assign52500_e86767_d_n9;
        locals.var_t2_dn10 = assign52500_e86767_d_n10;
        locals.var_t2_dn11 = assign52500_e86767_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign52510_e86783, assign52510_e86783_d_n3, assign52510_e86783_d_n4, assign52510_e86783_d_n5, assign52510_e86783_d_n6, assign52510_e86783_d_n7, assign52510_e86783_d_n8, assign52510_e86783_d_n9, assign52510_e86783_d_n10, assign52510_e86783_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 == 0.0)) {
        let assign52510_e86777: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign52510_e86780: f64 = (locals.var_t2 + locals.var_czbsswg_p2);
        let assign52510_e86781: f64 = (assign52510_e86777 * assign52510_e86780);
        (assign52510_e86781, (assign52510_e86777 * locals.var_t2_dn3), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign52510_e86780) + (assign52510_e86777 * locals.var_t2_dn4)), ((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign52510_e86780) + (assign52510_e86777 * locals.var_t2_dn5)), (assign52510_e86777 * locals.var_t2_dn6), (assign52510_e86777 * locals.var_t2_dn7), (assign52510_e86777 * locals.var_t2_dn8), (assign52510_e86777 * locals.var_t2_dn9), (assign52510_e86777 * locals.var_t2_dn10), (assign52510_e86777 * locals.var_t2_dn11),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52510_e86783;
        locals.var_qbsj3_dn3 = assign52510_e86783_d_n3;
        locals.var_qbsj3_dn4 = assign52510_e86783_d_n4;
        locals.var_qbsj3_dn5 = assign52510_e86783_d_n5;
        locals.var_qbsj3_dn6 = assign52510_e86783_d_n6;
        locals.var_qbsj3_dn7 = assign52510_e86783_d_n7;
        locals.var_qbsj3_dn8 = assign52510_e86783_d_n8;
        locals.var_qbsj3_dn9 = assign52510_e86783_d_n9;
        locals.var_qbsj3_dn10 = assign52510_e86783_d_n10;
        locals.var_qbsj3_dn11 = assign52510_e86783_d_n11;
        locals.var_qbsj3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_181(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52520_e86791, assign52520_e86791_d_n3, assign52520_e86791_d_n4, assign52520_e86791_d_n5, assign52520_e86791_d_n6, assign52520_e86791_d_n7, assign52520_e86791_d_n8, assign52520_e86791_d_n9, assign52520_e86791_d_n10, assign52520_e86791_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard800 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52520_e86791;
        locals.var_qbsj3_dn3 = assign52520_e86791_d_n3;
        locals.var_qbsj3_dn4 = assign52520_e86791_d_n4;
        locals.var_qbsj3_dn5 = assign52520_e86791_d_n5;
        locals.var_qbsj3_dn6 = assign52520_e86791_d_n6;
        locals.var_qbsj3_dn7 = assign52520_e86791_d_n7;
        locals.var_qbsj3_dn8 = assign52520_e86791_d_n8;
        locals.var_qbsj3_dn9 = assign52520_e86791_d_n9;
        locals.var_qbsj3_dn10 = assign52520_e86791_d_n10;
        locals.var_qbsj3_dn11 = assign52520_e86791_d_n11;
        locals.var_qbsj3_rv = 0.0;

        let (assign52530_e86800, assign52530_e86800_d_n3, assign52530_e86800_d_n4, assign52530_e86800_d_n5, assign52530_e86800_d_n6, assign52530_e86800_d_n7, assign52530_e86800_d_n8, assign52530_e86800_d_n9, assign52530_e86800_d_n10, assign52530_e86800_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52530_e86796: f64 = (p.p919 * locals.var_ibsdif);
        let assign52530_e86798: f64 = (assign52530_e86796 * p.p2);
        (assign52530_e86798, ((p.p919 * locals.var_ibsdif_dn3) * p.p2), ((p.p919 * locals.var_ibsdif_dn4) * p.p2), ((p.p919 * locals.var_ibsdif_dn5) * p.p2), ((p.p919 * locals.var_ibsdif_dn6) * p.p2), ((p.p919 * locals.var_ibsdif_dn7) * p.p2), ((p.p919 * locals.var_ibsdif_dn8) * p.p2), ((p.p919 * locals.var_ibsdif_dn9) * p.p2), ((p.p919 * locals.var_ibsdif_dn10) * p.p2), ((p.p919 * locals.var_ibsdif_dn11) * p.p2),)
    } else {
        (locals.var_qbsj4, locals.var_qbsj4_dn3, locals.var_qbsj4_dn4, locals.var_qbsj4_dn5, locals.var_qbsj4_dn6, locals.var_qbsj4_dn7, locals.var_qbsj4_dn8, locals.var_qbsj4_dn9, locals.var_qbsj4_dn10, locals.var_qbsj4_dn11,)
    }
};
        locals.var_qbsj4 = assign52530_e86800;
        locals.var_qbsj4_dn3 = assign52530_e86800_d_n3;
        locals.var_qbsj4_dn4 = assign52530_e86800_d_n4;
        locals.var_qbsj4_dn5 = assign52530_e86800_d_n5;
        locals.var_qbsj4_dn6 = assign52530_e86800_d_n6;
        locals.var_qbsj4_dn7 = assign52530_e86800_d_n7;
        locals.var_qbsj4_dn8 = assign52530_e86800_d_n8;
        locals.var_qbsj4_dn9 = assign52530_e86800_d_n9;
        locals.var_qbsj4_dn10 = assign52530_e86800_d_n10;
        locals.var_qbsj4_dn11 = assign52530_e86800_d_n11;
        locals.var_qbsj4_rv = 0.0;

        let (assign52540_e86811, assign52540_e86811_d_n3, assign52540_e86811_d_n4, assign52540_e86811_d_n5, assign52540_e86811_d_n6, assign52540_e86811_d_n7, assign52540_e86811_d_n8, assign52540_e86811_d_n9, assign52540_e86811_d_n10, assign52540_e86811_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52540_e86805: f64 = (locals.var_qbsj1 + locals.var_qbsj2);
        let assign52540_e86807: f64 = (assign52540_e86805 + locals.var_qbsj3);
        let assign52540_e86809: f64 = (assign52540_e86807 + locals.var_qbsj4);
        (assign52540_e86809, (((locals.var_qbsj1_dn3 + locals.var_qbsj2_dn3) + locals.var_qbsj3_dn3) + locals.var_qbsj4_dn3), (((locals.var_qbsj1_dn4 + locals.var_qbsj2_dn4) + locals.var_qbsj3_dn4) + locals.var_qbsj4_dn4), (((locals.var_qbsj1_dn5 + locals.var_qbsj2_dn5) + locals.var_qbsj3_dn5) + locals.var_qbsj4_dn5), (((locals.var_qbsj1_dn6 + locals.var_qbsj2_dn6) + locals.var_qbsj3_dn6) + locals.var_qbsj4_dn6), (((locals.var_qbsj1_dn7 + locals.var_qbsj2_dn7) + locals.var_qbsj3_dn7) + locals.var_qbsj4_dn7), (((locals.var_qbsj1_dn8 + locals.var_qbsj2_dn8) + locals.var_qbsj3_dn8) + locals.var_qbsj4_dn8), (((locals.var_qbsj1_dn9 + locals.var_qbsj2_dn9) + locals.var_qbsj3_dn9) + locals.var_qbsj4_dn9), (((locals.var_qbsj1_dn10 + locals.var_qbsj2_dn10) + locals.var_qbsj3_dn10) + locals.var_qbsj4_dn10), (((locals.var_qbsj1_dn11 + locals.var_qbsj2_dn11) + locals.var_qbsj3_dn11) + locals.var_qbsj4_dn11),)
    } else {
        (locals.var_qbsj, locals.var_qbsj_dn3, locals.var_qbsj_dn4, locals.var_qbsj_dn5, locals.var_qbsj_dn6, locals.var_qbsj_dn7, locals.var_qbsj_dn8, locals.var_qbsj_dn9, locals.var_qbsj_dn10, locals.var_qbsj_dn11,)
    }
};
        locals.var_qbsj = assign52540_e86811;
        locals.var_qbsj_dn3 = assign52540_e86811_d_n3;
        locals.var_qbsj_dn4 = assign52540_e86811_d_n4;
        locals.var_qbsj_dn5 = assign52540_e86811_d_n5;
        locals.var_qbsj_dn6 = assign52540_e86811_d_n6;
        locals.var_qbsj_dn7 = assign52540_e86811_d_n7;
        locals.var_qbsj_dn8 = assign52540_e86811_d_n8;
        locals.var_qbsj_dn9 = assign52540_e86811_d_n9;
        locals.var_qbsj_dn10 = assign52540_e86811_d_n10;
        locals.var_qbsj_dn11 = assign52540_e86811_d_n11;
        locals.var_qbsj_rv = 0.0;

        let (assign52550_e86818, assign52550_e86818_d_n3, assign52550_e86818_d_n4, assign52550_e86818_d_n5, assign52550_e86818_d_n6, assign52550_e86818_d_n7, assign52550_e86818_d_n8, assign52550_e86818_d_n9, assign52550_e86818_d_n10, assign52550_e86818_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52550_e86816: f64 = (locals.var_cjd_t * locals.var_adeff);
        (assign52550_e86816, (locals.var_cjd_t * locals.var_adeff_dn3), ((locals.var_cjd_t_dn4 * locals.var_adeff) + (locals.var_cjd_t * locals.var_adeff_dn4)), ((locals.var_cjd_t_dn5 * locals.var_adeff) + (locals.var_cjd_t * locals.var_adeff_dn5)), (locals.var_cjd_t * locals.var_adeff_dn6), (locals.var_cjd_t * locals.var_adeff_dn7), (locals.var_cjd_t * locals.var_adeff_dn8), (locals.var_cjd_t * locals.var_adeff_dn9), (locals.var_cjd_t * locals.var_adeff_dn10), (locals.var_cjd_t * locals.var_adeff_dn11),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn3, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11,)
    }
};
        locals.var_czbd = assign52550_e86818;
        locals.var_czbd_dn3 = assign52550_e86818_d_n3;
        locals.var_czbd_dn4 = assign52550_e86818_d_n4;
        locals.var_czbd_dn5 = assign52550_e86818_d_n5;
        locals.var_czbd_dn6 = assign52550_e86818_d_n6;
        locals.var_czbd_dn7 = assign52550_e86818_d_n7;
        locals.var_czbd_dn8 = assign52550_e86818_d_n8;
        locals.var_czbd_dn9 = assign52550_e86818_d_n9;
        locals.var_czbd_dn10 = assign52550_e86818_d_n10;
        locals.var_czbd_dn11 = assign52550_e86818_d_n11;
        locals.var_czbd_rv = 0.0;

        let (assign52560_e86825, assign52560_e86825_d_n3, assign52560_e86825_d_n4, assign52560_e86825_d_n5, assign52560_e86825_d_n6, assign52560_e86825_d_n7, assign52560_e86825_d_n8, assign52560_e86825_d_n9, assign52560_e86825_d_n10, assign52560_e86825_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52560_e86823: f64 = (locals.var_cjswd_t * locals.var_pdeff);
        (assign52560_e86823, (locals.var_cjswd_t * locals.var_pdeff_dn3), ((locals.var_cjswd_t_dn4 * locals.var_pdeff) + (locals.var_cjswd_t * locals.var_pdeff_dn4)), ((locals.var_cjswd_t_dn5 * locals.var_pdeff) + (locals.var_cjswd_t * locals.var_pdeff_dn5)), (locals.var_cjswd_t * locals.var_pdeff_dn6), (locals.var_cjswd_t * locals.var_pdeff_dn7), (locals.var_cjswd_t * locals.var_pdeff_dn8), (locals.var_cjswd_t * locals.var_pdeff_dn9), (locals.var_cjswd_t * locals.var_pdeff_dn10), (locals.var_cjswd_t * locals.var_pdeff_dn11),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn3, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11,)
    }
};
        locals.var_czbdsw = assign52560_e86825;
        locals.var_czbdsw_dn3 = assign52560_e86825_d_n3;
        locals.var_czbdsw_dn4 = assign52560_e86825_d_n4;
        locals.var_czbdsw_dn5 = assign52560_e86825_d_n5;
        locals.var_czbdsw_dn6 = assign52560_e86825_d_n6;
        locals.var_czbdsw_dn7 = assign52560_e86825_d_n7;
        locals.var_czbdsw_dn8 = assign52560_e86825_d_n8;
        locals.var_czbdsw_dn9 = assign52560_e86825_d_n9;
        locals.var_czbdsw_dn10 = assign52560_e86825_d_n10;
        locals.var_czbdsw_dn11 = assign52560_e86825_d_n11;
        locals.var_czbdsw_rv = 0.0;

        let (assign52570_e86834, assign52570_e86834_d_n4, assign52570_e86834_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52570_e86830: f64 = (locals.var_cjswgd_t * locals.var_weffcj);
        let assign52570_e86832: f64 = (assign52570_e86830 * p.p2);
        (assign52570_e86832, ((locals.var_cjswgd_t_dn4 * locals.var_weffcj) * p.p2), ((locals.var_cjswgd_t_dn5 * locals.var_weffcj) * p.p2),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5,)
    }
};
        locals.var_czbdswg = assign52570_e86834;
        locals.var_czbdswg_dn4 = assign52570_e86834_d_n4;
        locals.var_czbdswg_dn5 = assign52570_e86834_d_n5;
        locals.var_czbdswg_rv = 0.0;

        let (assign52580_e86842,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52580_e86839: f64 = (-p.p914);
        let assign52580_e86840: f64 = (0.1_f64).powf(assign52580_e86839);
        (assign52580_e86840,)
    } else {
        (locals.var_czbd_p1,)
    }
};
        locals.var_czbd_p1 = assign52580_e86842;
        locals.var_czbd_p1_rv = 0.0;

        let assign52590_e86845: f64 = if p.p914 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard804 = assign52590_e86845;
        locals.var_guard804_rv = 0.0;

        let (assign52600_e86855,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard804 != 0.0)) {
        let assign52600_e86852: f64 = (0.1_f64).ln();
        let assign52600_e86853: f64 = (1.5 - assign52600_e86852);
        (assign52600_e86853,)
    } else {
        (locals.var_czbd_p2,)
    }
};
        locals.var_czbd_p2 = assign52600_e86855;
        locals.var_czbd_p2_rv = 0.0;

        let (assign52610_e86879,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard804 == 0.0)) {
        let assign52610_e86864: f64 = (1.0 - p.p914);
        let assign52610_e86865: f64 = (1.0 / assign52610_e86864);
        let assign52610_e86869: f64 = (0.05 * p.p914);
        let assign52610_e86872: f64 = (1.0 + p.p914);
        let assign52610_e86873: f64 = (assign52610_e86869 * assign52610_e86872);
        let assign52610_e86875: f64 = (assign52610_e86873 * locals.var_czbd_p1);
        let assign52610_e86876: f64 = (1.0 - assign52610_e86875);
        let assign52610_e86877: f64 = (assign52610_e86865 * assign52610_e86876);
        (assign52610_e86877,)
    } else {
        (locals.var_czbd_p2,)
    }
};
        locals.var_czbd_p2 = assign52610_e86879;
        locals.var_czbd_p2_rv = 0.0;

        let (assign52620_e86887,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52620_e86884: f64 = (-p.p916);
        let assign52620_e86885: f64 = (0.1_f64).powf(assign52620_e86884);
        (assign52620_e86885,)
    } else {
        (locals.var_czbdsw_p1,)
    }
};
        locals.var_czbdsw_p1 = assign52620_e86887;
        locals.var_czbdsw_p1_rv = 0.0;

        let assign52630_e86890: f64 = if p.p916 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard805 = assign52630_e86890;
        locals.var_guard805_rv = 0.0;

        let (assign52640_e86900,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard805 != 0.0)) {
        let assign52640_e86897: f64 = (0.1_f64).ln();
        let assign52640_e86898: f64 = (1.5 - assign52640_e86897);
        (assign52640_e86898,)
    } else {
        (locals.var_czbdsw_p2,)
    }
};
        locals.var_czbdsw_p2 = assign52640_e86900;
        locals.var_czbdsw_p2_rv = 0.0;

        let (assign52650_e86924,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard805 == 0.0)) {
        let assign52650_e86909: f64 = (1.0 - p.p916);
        let assign52650_e86910: f64 = (1.0 / assign52650_e86909);
        let assign52650_e86914: f64 = (0.05 * p.p916);
        let assign52650_e86917: f64 = (1.0 + p.p916);
        let assign52650_e86918: f64 = (assign52650_e86914 * assign52650_e86917);
        let assign52650_e86920: f64 = (assign52650_e86918 * locals.var_czbdsw_p1);
        let assign52650_e86921: f64 = (1.0 - assign52650_e86920);
        let assign52650_e86922: f64 = (assign52650_e86910 * assign52650_e86921);
        (assign52650_e86922,)
    } else {
        (locals.var_czbdsw_p2,)
    }
};
        locals.var_czbdsw_p2 = assign52650_e86924;
        locals.var_czbdsw_p2_rv = 0.0;

        let (assign52660_e86932,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52660_e86929: f64 = (-p.p918);
        let assign52660_e86930: f64 = (0.1_f64).powf(assign52660_e86929);
        (assign52660_e86930,)
    } else {
        (locals.var_czbdswg_p1,)
    }
};
        locals.var_czbdswg_p1 = assign52660_e86932;
        locals.var_czbdswg_p1_rv = 0.0;

        let assign52670_e86935: f64 = if p.p918 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard806 = assign52670_e86935;
        locals.var_guard806_rv = 0.0;

        let (assign52680_e86945,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard806 != 0.0)) {
        let assign52680_e86942: f64 = (0.1_f64).ln();
        let assign52680_e86943: f64 = (1.5 - assign52680_e86942);
        (assign52680_e86943,)
    } else {
        (locals.var_czbdswg_p2,)
    }
};
        locals.var_czbdswg_p2 = assign52680_e86945;
        locals.var_czbdswg_p2_rv = 0.0;

        let (assign52690_e86969,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard806 == 0.0)) {
        let assign52690_e86954: f64 = (1.0 - p.p918);
        let assign52690_e86955: f64 = (1.0 / assign52690_e86954);
        let assign52690_e86959: f64 = (0.05 * p.p918);
        let assign52690_e86962: f64 = (1.0 + p.p918);
        let assign52690_e86963: f64 = (assign52690_e86959 * assign52690_e86962);
        let assign52690_e86965: f64 = (assign52690_e86963 * locals.var_czbdswg_p1);
        let assign52690_e86966: f64 = (1.0 - assign52690_e86965);
        let assign52690_e86967: f64 = (assign52690_e86955 * assign52690_e86966);
        (assign52690_e86967,)
    } else {
        (locals.var_czbdswg_p2,)
    }
};
        locals.var_czbdswg_p2 = assign52690_e86969;
        locals.var_czbdswg_p2_rv = 0.0;

        let assign52700_e86972: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard807 = assign52700_e86972;
        locals.var_guard807_rv = 0.0;

        let (assign52710_e86981, assign52710_e86981_d_n3, assign52710_e86981_d_n4, assign52710_e86981_d_n5, assign52710_e86981_d_n6, assign52710_e86981_d_n7, assign52710_e86981_d_n8, assign52710_e86981_d_n9, assign52710_e86981_d_n10, assign52710_e86981_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) {
        let assign52710_e86979: f64 = (locals.var_vbd_jct / locals.var_pbd_t);
        (assign52710_e86979, 0.0, (-((locals.var_vbd_jct * locals.var_pbd_t_dn4) / (locals.var_pbd_t * locals.var_pbd_t))), (-((locals.var_vbd_jct * locals.var_pbd_t_dn5) / (locals.var_pbd_t * locals.var_pbd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52710_e86981;
        locals.var_t1_dn3 = assign52710_e86981_d_n3;
        locals.var_t1_dn4 = assign52710_e86981_d_n4;
        locals.var_t1_dn5 = assign52710_e86981_d_n5;
        locals.var_t1_dn6 = assign52710_e86981_d_n6;
        locals.var_t1_dn7 = assign52710_e86981_d_n7;
        locals.var_t1_dn8 = assign52710_e86981_d_n8;
        locals.var_t1_dn9 = assign52710_e86981_d_n9;
        locals.var_t1_dn10 = assign52710_e86981_d_n10;
        locals.var_t1_dn11 = assign52710_e86981_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52720_e86984: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard808 = assign52720_e86984;
        locals.var_guard808_rv = 0.0;

        let (assign52730_e86995, assign52730_e86995_d_n3, assign52730_e86995_d_n4, assign52730_e86995_d_n5, assign52730_e86995_d_n6, assign52730_e86995_d_n7, assign52730_e86995_d_n8, assign52730_e86995_d_n9, assign52730_e86995_d_n10, assign52730_e86995_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign52730_e86993: f64 = (1.0 - locals.var_t1);
        (assign52730_e86993, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52730_e86995;
        locals.var_arg_dn3 = assign52730_e86995_d_n3;
        locals.var_arg_dn4 = assign52730_e86995_d_n4;
        locals.var_arg_dn5 = assign52730_e86995_d_n5;
        locals.var_arg_dn6 = assign52730_e86995_d_n6;
        locals.var_arg_dn7 = assign52730_e86995_d_n7;
        locals.var_arg_dn8 = assign52730_e86995_d_n8;
        locals.var_arg_dn9 = assign52730_e86995_d_n9;
        locals.var_arg_dn10 = assign52730_e86995_d_n10;
        locals.var_arg_dn11 = assign52730_e86995_d_n11;
        locals.var_arg_rv = 0.0;

        let assign52740_e86998: f64 = if p.p914 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard809 = assign52740_e86998;
        locals.var_guard809_rv = 0.0;

        let assign52750_e87001: f64 = if p.p914 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard810 = assign52750_e87001;
        locals.var_guard810_rv = 0.0;

        let (assign52760_e87017, assign52760_e87017_d_n3, assign52760_e87017_d_n4, assign52760_e87017_d_n5, assign52760_e87017_d_n6, assign52760_e87017_d_n7, assign52760_e87017_d_n8, assign52760_e87017_d_n9, assign52760_e87017_d_n10, assign52760_e87017_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign52760_e87014: f64 = (locals.var_arg).sqrt();
        let assign52760_e87015: f64 = (1.0 / assign52760_e87014);
        (assign52760_e87015, (-((locals.var_arg_dn3 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn4 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn5 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn6 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn7 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn8 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn9 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn10 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn11 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52760_e87017;
        locals.var_sarg_dn3 = assign52760_e87017_d_n3;
        locals.var_sarg_dn4 = assign52760_e87017_d_n4;
        locals.var_sarg_dn5 = assign52760_e87017_d_n5;
        locals.var_sarg_dn6 = assign52760_e87017_d_n6;
        locals.var_sarg_dn7 = assign52760_e87017_d_n7;
        locals.var_sarg_dn8 = assign52760_e87017_d_n8;
        locals.var_sarg_dn9 = assign52760_e87017_d_n9;
        locals.var_sarg_dn10 = assign52760_e87017_d_n10;
        locals.var_sarg_dn11 = assign52760_e87017_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52770_e87036, assign52770_e87036_d_n3, assign52770_e87036_d_n4, assign52770_e87036_d_n5, assign52770_e87036_d_n6, assign52770_e87036_d_n7, assign52770_e87036_d_n8, assign52770_e87036_d_n9, assign52770_e87036_d_n10, assign52770_e87036_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 == 0.0)) {
        let assign52770_e87030: f64 = (-p.p914);
        let assign52770_e87032: f64 = (locals.var_arg).ln();
        let assign52770_e87033: f64 = (assign52770_e87030 * assign52770_e87032);
        let assign52770_e87034: f64 = { let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52770_e87034, ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52770_e87036;
        locals.var_sarg_dn3 = assign52770_e87036_d_n3;
        locals.var_sarg_dn4 = assign52770_e87036_d_n4;
        locals.var_sarg_dn5 = assign52770_e87036_d_n5;
        locals.var_sarg_dn6 = assign52770_e87036_d_n6;
        locals.var_sarg_dn7 = assign52770_e87036_d_n7;
        locals.var_sarg_dn8 = assign52770_e87036_d_n8;
        locals.var_sarg_dn9 = assign52770_e87036_d_n9;
        locals.var_sarg_dn10 = assign52770_e87036_d_n10;
        locals.var_sarg_dn11 = assign52770_e87036_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52780_e87059, assign52780_e87059_d_n3, assign52780_e87059_d_n4, assign52780_e87059_d_n5, assign52780_e87059_d_n6, assign52780_e87059_d_n7, assign52780_e87059_d_n8, assign52780_e87059_d_n9, assign52780_e87059_d_n10, assign52780_e87059_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign52780_e87047: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign52780_e87051: f64 = (locals.var_arg * locals.var_sarg);
        let assign52780_e87052: f64 = (1.0 - assign52780_e87051);
        let assign52780_e87053: f64 = (assign52780_e87047 * assign52780_e87052);
        let assign52780_e87056: f64 = (1.0 - p.p914);
        let assign52780_e87057: f64 = (assign52780_e87053 / assign52780_e87056);
        (assign52780_e87057, ((((locals.var_pbd_t * locals.var_czbd_dn3) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52780_e87056), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52780_e87056), (((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn6) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn7) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn8) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn9) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn10) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn11) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52780_e87056),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52780_e87059;
        locals.var_qbdj1_dn3 = assign52780_e87059_d_n3;
        locals.var_qbdj1_dn4 = assign52780_e87059_d_n4;
        locals.var_qbdj1_dn5 = assign52780_e87059_d_n5;
        locals.var_qbdj1_dn6 = assign52780_e87059_d_n6;
        locals.var_qbdj1_dn7 = assign52780_e87059_d_n7;
        locals.var_qbdj1_dn8 = assign52780_e87059_d_n8;
        locals.var_qbdj1_dn9 = assign52780_e87059_d_n9;
        locals.var_qbdj1_dn10 = assign52780_e87059_d_n10;
        locals.var_qbdj1_dn11 = assign52780_e87059_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let (assign52790_e87077, assign52790_e87077_d_n3, assign52790_e87077_d_n4, assign52790_e87077_d_n5, assign52790_e87077_d_n6, assign52790_e87077_d_n7, assign52790_e87077_d_n8, assign52790_e87077_d_n9, assign52790_e87077_d_n10, assign52790_e87077_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 == 0.0)) {
        let assign52790_e87071: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign52790_e87073: f64 = (locals.var_arg).ln();
        let assign52790_e87074: f64 = (-assign52790_e87073);
        let assign52790_e87075: f64 = (assign52790_e87071 * assign52790_e87074);
        (assign52790_e87075, (((locals.var_pbd_t * locals.var_czbd_dn3) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52790_e87077;
        locals.var_qbdj1_dn3 = assign52790_e87077_d_n3;
        locals.var_qbdj1_dn4 = assign52790_e87077_d_n4;
        locals.var_qbdj1_dn5 = assign52790_e87077_d_n5;
        locals.var_qbdj1_dn6 = assign52790_e87077_d_n6;
        locals.var_qbdj1_dn7 = assign52790_e87077_d_n7;
        locals.var_qbdj1_dn8 = assign52790_e87077_d_n8;
        locals.var_qbdj1_dn9 = assign52790_e87077_d_n9;
        locals.var_qbdj1_dn10 = assign52790_e87077_d_n10;
        locals.var_qbdj1_dn11 = assign52790_e87077_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let (assign52800_e87103, assign52800_e87103_d_n3, assign52800_e87103_d_n4, assign52800_e87103_d_n5, assign52800_e87103_d_n6, assign52800_e87103_d_n7, assign52800_e87103_d_n8, assign52800_e87103_d_n9, assign52800_e87103_d_n10, assign52800_e87103_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 == 0.0)) {
        let assign52800_e87088: f64 = (locals.var_t1 - 1.0);
        let assign52800_e87089: f64 = (locals.var_czbd_p1 * assign52800_e87088);
        let assign52800_e87092: f64 = (5.0 * p.p914);
        let assign52800_e87095: f64 = (locals.var_t1 - 1.0);
        let assign52800_e87096: f64 = (assign52800_e87092 * assign52800_e87095);
        let assign52800_e87099: f64 = (1.0 + p.p914);
        let assign52800_e87100: f64 = (assign52800_e87096 + assign52800_e87099);
        let assign52800_e87101: f64 = (assign52800_e87089 * assign52800_e87100);
        (assign52800_e87101, (((locals.var_czbd_p1 * locals.var_t1_dn3) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn3))), (((locals.var_czbd_p1 * locals.var_t1_dn4) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn4))), (((locals.var_czbd_p1 * locals.var_t1_dn5) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn5))), (((locals.var_czbd_p1 * locals.var_t1_dn6) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn6))), (((locals.var_czbd_p1 * locals.var_t1_dn7) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn7))), (((locals.var_czbd_p1 * locals.var_t1_dn8) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn8))), (((locals.var_czbd_p1 * locals.var_t1_dn9) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn9))), (((locals.var_czbd_p1 * locals.var_t1_dn10) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn10))), (((locals.var_czbd_p1 * locals.var_t1_dn11) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52800_e87103;
        locals.var_t2_dn3 = assign52800_e87103_d_n3;
        locals.var_t2_dn4 = assign52800_e87103_d_n4;
        locals.var_t2_dn5 = assign52800_e87103_d_n5;
        locals.var_t2_dn6 = assign52800_e87103_d_n6;
        locals.var_t2_dn7 = assign52800_e87103_d_n7;
        locals.var_t2_dn8 = assign52800_e87103_d_n8;
        locals.var_t2_dn9 = assign52800_e87103_d_n9;
        locals.var_t2_dn10 = assign52800_e87103_d_n10;
        locals.var_t2_dn11 = assign52800_e87103_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign52810_e87119, assign52810_e87119_d_n3, assign52810_e87119_d_n4, assign52810_e87119_d_n5, assign52810_e87119_d_n6, assign52810_e87119_d_n7, assign52810_e87119_d_n8, assign52810_e87119_d_n9, assign52810_e87119_d_n10, assign52810_e87119_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 == 0.0)) {
        let assign52810_e87113: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign52810_e87116: f64 = (locals.var_t2 + locals.var_czbd_p2);
        let assign52810_e87117: f64 = (assign52810_e87113 * assign52810_e87116);
        (assign52810_e87117, (((locals.var_pbd_t * locals.var_czbd_dn3) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn3)), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn4)), ((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn5)), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn6)), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn7)), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn8)), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn9)), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn10)), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52810_e87119;
        locals.var_qbdj1_dn3 = assign52810_e87119_d_n3;
        locals.var_qbdj1_dn4 = assign52810_e87119_d_n4;
        locals.var_qbdj1_dn5 = assign52810_e87119_d_n5;
        locals.var_qbdj1_dn6 = assign52810_e87119_d_n6;
        locals.var_qbdj1_dn7 = assign52810_e87119_d_n7;
        locals.var_qbdj1_dn8 = assign52810_e87119_d_n8;
        locals.var_qbdj1_dn9 = assign52810_e87119_d_n9;
        locals.var_qbdj1_dn10 = assign52810_e87119_d_n10;
        locals.var_qbdj1_dn11 = assign52810_e87119_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let (assign52820_e87127, assign52820_e87127_d_n3, assign52820_e87127_d_n4, assign52820_e87127_d_n5, assign52820_e87127_d_n6, assign52820_e87127_d_n7, assign52820_e87127_d_n8, assign52820_e87127_d_n9, assign52820_e87127_d_n10, assign52820_e87127_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard807 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52820_e87127;
        locals.var_qbdj1_dn3 = assign52820_e87127_d_n3;
        locals.var_qbdj1_dn4 = assign52820_e87127_d_n4;
        locals.var_qbdj1_dn5 = assign52820_e87127_d_n5;
        locals.var_qbdj1_dn6 = assign52820_e87127_d_n6;
        locals.var_qbdj1_dn7 = assign52820_e87127_d_n7;
        locals.var_qbdj1_dn8 = assign52820_e87127_d_n8;
        locals.var_qbdj1_dn9 = assign52820_e87127_d_n9;
        locals.var_qbdj1_dn10 = assign52820_e87127_d_n10;
        locals.var_qbdj1_dn11 = assign52820_e87127_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let assign52830_e87130: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard811 = assign52830_e87130;
        locals.var_guard811_rv = 0.0;

        let (assign52840_e87139, assign52840_e87139_d_n3, assign52840_e87139_d_n4, assign52840_e87139_d_n5, assign52840_e87139_d_n6, assign52840_e87139_d_n7, assign52840_e87139_d_n8, assign52840_e87139_d_n9, assign52840_e87139_d_n10, assign52840_e87139_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) {
        let assign52840_e87137: f64 = (locals.var_vbd_jct / locals.var_pbswd_t);
        (assign52840_e87137, 0.0, (-((locals.var_vbd_jct * locals.var_pbswd_t_dn4) / (locals.var_pbswd_t * locals.var_pbswd_t))), (-((locals.var_vbd_jct * locals.var_pbswd_t_dn5) / (locals.var_pbswd_t * locals.var_pbswd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbswd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbswd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52840_e87139;
        locals.var_t1_dn3 = assign52840_e87139_d_n3;
        locals.var_t1_dn4 = assign52840_e87139_d_n4;
        locals.var_t1_dn5 = assign52840_e87139_d_n5;
        locals.var_t1_dn6 = assign52840_e87139_d_n6;
        locals.var_t1_dn7 = assign52840_e87139_d_n7;
        locals.var_t1_dn8 = assign52840_e87139_d_n8;
        locals.var_t1_dn9 = assign52840_e87139_d_n9;
        locals.var_t1_dn10 = assign52840_e87139_d_n10;
        locals.var_t1_dn11 = assign52840_e87139_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52850_e87142: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard812 = assign52850_e87142;
        locals.var_guard812_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_182(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52860_e87153, assign52860_e87153_d_n3, assign52860_e87153_d_n4, assign52860_e87153_d_n5, assign52860_e87153_d_n6, assign52860_e87153_d_n7, assign52860_e87153_d_n8, assign52860_e87153_d_n9, assign52860_e87153_d_n10, assign52860_e87153_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign52860_e87151: f64 = (1.0 - locals.var_t1);
        (assign52860_e87151, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52860_e87153;
        locals.var_arg_dn3 = assign52860_e87153_d_n3;
        locals.var_arg_dn4 = assign52860_e87153_d_n4;
        locals.var_arg_dn5 = assign52860_e87153_d_n5;
        locals.var_arg_dn6 = assign52860_e87153_d_n6;
        locals.var_arg_dn7 = assign52860_e87153_d_n7;
        locals.var_arg_dn8 = assign52860_e87153_d_n8;
        locals.var_arg_dn9 = assign52860_e87153_d_n9;
        locals.var_arg_dn10 = assign52860_e87153_d_n10;
        locals.var_arg_dn11 = assign52860_e87153_d_n11;
        locals.var_arg_rv = 0.0;

        let assign52870_e87156: f64 = if p.p916 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard813 = assign52870_e87156;
        locals.var_guard813_rv = 0.0;

        let assign52880_e87159: f64 = if p.p916 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard814 = assign52880_e87159;
        locals.var_guard814_rv = 0.0;

        let (assign52890_e87175, assign52890_e87175_d_n3, assign52890_e87175_d_n4, assign52890_e87175_d_n5, assign52890_e87175_d_n6, assign52890_e87175_d_n7, assign52890_e87175_d_n8, assign52890_e87175_d_n9, assign52890_e87175_d_n10, assign52890_e87175_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 != 0.0)) {
        let assign52890_e87172: f64 = (locals.var_arg).sqrt();
        let assign52890_e87173: f64 = (1.0 / assign52890_e87172);
        (assign52890_e87173, (-((locals.var_arg_dn3 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn4 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn5 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn6 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn7 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn8 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn9 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn10 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn11 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52890_e87175;
        locals.var_sarg_dn3 = assign52890_e87175_d_n3;
        locals.var_sarg_dn4 = assign52890_e87175_d_n4;
        locals.var_sarg_dn5 = assign52890_e87175_d_n5;
        locals.var_sarg_dn6 = assign52890_e87175_d_n6;
        locals.var_sarg_dn7 = assign52890_e87175_d_n7;
        locals.var_sarg_dn8 = assign52890_e87175_d_n8;
        locals.var_sarg_dn9 = assign52890_e87175_d_n9;
        locals.var_sarg_dn10 = assign52890_e87175_d_n10;
        locals.var_sarg_dn11 = assign52890_e87175_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52900_e87194, assign52900_e87194_d_n3, assign52900_e87194_d_n4, assign52900_e87194_d_n5, assign52900_e87194_d_n6, assign52900_e87194_d_n7, assign52900_e87194_d_n8, assign52900_e87194_d_n9, assign52900_e87194_d_n10, assign52900_e87194_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 == 0.0)) {
        let assign52900_e87188: f64 = (-p.p916);
        let assign52900_e87190: f64 = (locals.var_arg).ln();
        let assign52900_e87191: f64 = (assign52900_e87188 * assign52900_e87190);
        let assign52900_e87192: f64 = { let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52900_e87192, ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52900_e87194;
        locals.var_sarg_dn3 = assign52900_e87194_d_n3;
        locals.var_sarg_dn4 = assign52900_e87194_d_n4;
        locals.var_sarg_dn5 = assign52900_e87194_d_n5;
        locals.var_sarg_dn6 = assign52900_e87194_d_n6;
        locals.var_sarg_dn7 = assign52900_e87194_d_n7;
        locals.var_sarg_dn8 = assign52900_e87194_d_n8;
        locals.var_sarg_dn9 = assign52900_e87194_d_n9;
        locals.var_sarg_dn10 = assign52900_e87194_d_n10;
        locals.var_sarg_dn11 = assign52900_e87194_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52910_e87217, assign52910_e87217_d_n3, assign52910_e87217_d_n4, assign52910_e87217_d_n5, assign52910_e87217_d_n6, assign52910_e87217_d_n7, assign52910_e87217_d_n8, assign52910_e87217_d_n9, assign52910_e87217_d_n10, assign52910_e87217_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign52910_e87205: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign52910_e87209: f64 = (locals.var_arg * locals.var_sarg);
        let assign52910_e87210: f64 = (1.0 - assign52910_e87209);
        let assign52910_e87211: f64 = (assign52910_e87205 * assign52910_e87210);
        let assign52910_e87214: f64 = (1.0 - p.p916);
        let assign52910_e87215: f64 = (assign52910_e87211 / assign52910_e87214);
        (assign52910_e87215, ((((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52910_e87214), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52910_e87214), (((((locals.var_pbswd_t_dn5 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn5)) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52910_e87214),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign52910_e87217;
        locals.var_qbdj2_dn3 = assign52910_e87217_d_n3;
        locals.var_qbdj2_dn4 = assign52910_e87217_d_n4;
        locals.var_qbdj2_dn5 = assign52910_e87217_d_n5;
        locals.var_qbdj2_dn6 = assign52910_e87217_d_n6;
        locals.var_qbdj2_dn7 = assign52910_e87217_d_n7;
        locals.var_qbdj2_dn8 = assign52910_e87217_d_n8;
        locals.var_qbdj2_dn9 = assign52910_e87217_d_n9;
        locals.var_qbdj2_dn10 = assign52910_e87217_d_n10;
        locals.var_qbdj2_dn11 = assign52910_e87217_d_n11;
        locals.var_qbdj2_rv = 0.0;

        let (assign52920_e87235, assign52920_e87235_d_n3, assign52920_e87235_d_n4, assign52920_e87235_d_n5, assign52920_e87235_d_n6, assign52920_e87235_d_n7, assign52920_e87235_d_n8, assign52920_e87235_d_n9, assign52920_e87235_d_n10, assign52920_e87235_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 == 0.0)) {
        let assign52920_e87229: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign52920_e87231: f64 = (locals.var_arg).ln();
        let assign52920_e87232: f64 = (-assign52920_e87231);
        let assign52920_e87233: f64 = (assign52920_e87229 * assign52920_e87232);
        (assign52920_e87233, (((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbswd_t_dn5 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn5)) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign52920_e87235;
        locals.var_qbdj2_dn3 = assign52920_e87235_d_n3;
        locals.var_qbdj2_dn4 = assign52920_e87235_d_n4;
        locals.var_qbdj2_dn5 = assign52920_e87235_d_n5;
        locals.var_qbdj2_dn6 = assign52920_e87235_d_n6;
        locals.var_qbdj2_dn7 = assign52920_e87235_d_n7;
        locals.var_qbdj2_dn8 = assign52920_e87235_d_n8;
        locals.var_qbdj2_dn9 = assign52920_e87235_d_n9;
        locals.var_qbdj2_dn10 = assign52920_e87235_d_n10;
        locals.var_qbdj2_dn11 = assign52920_e87235_d_n11;
        locals.var_qbdj2_rv = 0.0;

        let (assign52930_e87261, assign52930_e87261_d_n3, assign52930_e87261_d_n4, assign52930_e87261_d_n5, assign52930_e87261_d_n6, assign52930_e87261_d_n7, assign52930_e87261_d_n8, assign52930_e87261_d_n9, assign52930_e87261_d_n10, assign52930_e87261_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 == 0.0)) {
        let assign52930_e87246: f64 = (locals.var_t1 - 1.0);
        let assign52930_e87247: f64 = (locals.var_czbdsw_p1 * assign52930_e87246);
        let assign52930_e87250: f64 = (5.0 * p.p916);
        let assign52930_e87253: f64 = (locals.var_t1 - 1.0);
        let assign52930_e87254: f64 = (assign52930_e87250 * assign52930_e87253);
        let assign52930_e87257: f64 = (1.0 + p.p916);
        let assign52930_e87258: f64 = (assign52930_e87254 + assign52930_e87257);
        let assign52930_e87259: f64 = (assign52930_e87247 * assign52930_e87258);
        (assign52930_e87259, (((locals.var_czbdsw_p1 * locals.var_t1_dn3) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn3))), (((locals.var_czbdsw_p1 * locals.var_t1_dn4) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn4))), (((locals.var_czbdsw_p1 * locals.var_t1_dn5) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn5))), (((locals.var_czbdsw_p1 * locals.var_t1_dn6) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn6))), (((locals.var_czbdsw_p1 * locals.var_t1_dn7) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn7))), (((locals.var_czbdsw_p1 * locals.var_t1_dn8) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn8))), (((locals.var_czbdsw_p1 * locals.var_t1_dn9) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn9))), (((locals.var_czbdsw_p1 * locals.var_t1_dn10) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn10))), (((locals.var_czbdsw_p1 * locals.var_t1_dn11) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52930_e87261;
        locals.var_t2_dn3 = assign52930_e87261_d_n3;
        locals.var_t2_dn4 = assign52930_e87261_d_n4;
        locals.var_t2_dn5 = assign52930_e87261_d_n5;
        locals.var_t2_dn6 = assign52930_e87261_d_n6;
        locals.var_t2_dn7 = assign52930_e87261_d_n7;
        locals.var_t2_dn8 = assign52930_e87261_d_n8;
        locals.var_t2_dn9 = assign52930_e87261_d_n9;
        locals.var_t2_dn10 = assign52930_e87261_d_n10;
        locals.var_t2_dn11 = assign52930_e87261_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign52940_e87277, assign52940_e87277_d_n3, assign52940_e87277_d_n4, assign52940_e87277_d_n5, assign52940_e87277_d_n6, assign52940_e87277_d_n7, assign52940_e87277_d_n8, assign52940_e87277_d_n9, assign52940_e87277_d_n10, assign52940_e87277_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 == 0.0)) {
        let assign52940_e87271: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign52940_e87274: f64 = (locals.var_t2 + locals.var_czbdsw_p2);
        let assign52940_e87275: f64 = (assign52940_e87271 * assign52940_e87274);
        (assign52940_e87275, (((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn3)), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn4)), ((((locals.var_pbswd_t_dn5 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn5)) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn5)), (((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn6)), (((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn7)), (((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn8)), (((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn9)), (((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn10)), (((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign52940_e87277;
        locals.var_qbdj2_dn3 = assign52940_e87277_d_n3;
        locals.var_qbdj2_dn4 = assign52940_e87277_d_n4;
        locals.var_qbdj2_dn5 = assign52940_e87277_d_n5;
        locals.var_qbdj2_dn6 = assign52940_e87277_d_n6;
        locals.var_qbdj2_dn7 = assign52940_e87277_d_n7;
        locals.var_qbdj2_dn8 = assign52940_e87277_d_n8;
        locals.var_qbdj2_dn9 = assign52940_e87277_d_n9;
        locals.var_qbdj2_dn10 = assign52940_e87277_d_n10;
        locals.var_qbdj2_dn11 = assign52940_e87277_d_n11;
        locals.var_qbdj2_rv = 0.0;

        let (assign52950_e87285, assign52950_e87285_d_n3, assign52950_e87285_d_n4, assign52950_e87285_d_n5, assign52950_e87285_d_n6, assign52950_e87285_d_n7, assign52950_e87285_d_n8, assign52950_e87285_d_n9, assign52950_e87285_d_n10, assign52950_e87285_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard811 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign52950_e87285;
        locals.var_qbdj2_dn3 = assign52950_e87285_d_n3;
        locals.var_qbdj2_dn4 = assign52950_e87285_d_n4;
        locals.var_qbdj2_dn5 = assign52950_e87285_d_n5;
        locals.var_qbdj2_dn6 = assign52950_e87285_d_n6;
        locals.var_qbdj2_dn7 = assign52950_e87285_d_n7;
        locals.var_qbdj2_dn8 = assign52950_e87285_d_n8;
        locals.var_qbdj2_dn9 = assign52950_e87285_d_n9;
        locals.var_qbdj2_dn10 = assign52950_e87285_d_n10;
        locals.var_qbdj2_dn11 = assign52950_e87285_d_n11;
        locals.var_qbdj2_rv = 0.0;

        let assign52960_e87288: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard815 = assign52960_e87288;
        locals.var_guard815_rv = 0.0;

        let (assign52970_e87297, assign52970_e87297_d_n3, assign52970_e87297_d_n4, assign52970_e87297_d_n5, assign52970_e87297_d_n6, assign52970_e87297_d_n7, assign52970_e87297_d_n8, assign52970_e87297_d_n9, assign52970_e87297_d_n10, assign52970_e87297_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) {
        let assign52970_e87295: f64 = (locals.var_vbd_jct / locals.var_pbswgd_t);
        (assign52970_e87295, 0.0, (-((locals.var_vbd_jct * locals.var_pbswgd_t_dn4) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), (-((locals.var_vbd_jct * locals.var_pbswgd_t_dn5) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbswgd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbswgd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52970_e87297;
        locals.var_t1_dn3 = assign52970_e87297_d_n3;
        locals.var_t1_dn4 = assign52970_e87297_d_n4;
        locals.var_t1_dn5 = assign52970_e87297_d_n5;
        locals.var_t1_dn6 = assign52970_e87297_d_n6;
        locals.var_t1_dn7 = assign52970_e87297_d_n7;
        locals.var_t1_dn8 = assign52970_e87297_d_n8;
        locals.var_t1_dn9 = assign52970_e87297_d_n9;
        locals.var_t1_dn10 = assign52970_e87297_d_n10;
        locals.var_t1_dn11 = assign52970_e87297_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52980_e87300: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard816 = assign52980_e87300;
        locals.var_guard816_rv = 0.0;

        let (assign52990_e87311, assign52990_e87311_d_n3, assign52990_e87311_d_n4, assign52990_e87311_d_n5, assign52990_e87311_d_n6, assign52990_e87311_d_n7, assign52990_e87311_d_n8, assign52990_e87311_d_n9, assign52990_e87311_d_n10, assign52990_e87311_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) {
        let assign52990_e87309: f64 = (1.0 - locals.var_t1);
        (assign52990_e87309, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52990_e87311;
        locals.var_arg_dn3 = assign52990_e87311_d_n3;
        locals.var_arg_dn4 = assign52990_e87311_d_n4;
        locals.var_arg_dn5 = assign52990_e87311_d_n5;
        locals.var_arg_dn6 = assign52990_e87311_d_n6;
        locals.var_arg_dn7 = assign52990_e87311_d_n7;
        locals.var_arg_dn8 = assign52990_e87311_d_n8;
        locals.var_arg_dn9 = assign52990_e87311_d_n9;
        locals.var_arg_dn10 = assign52990_e87311_d_n10;
        locals.var_arg_dn11 = assign52990_e87311_d_n11;
        locals.var_arg_rv = 0.0;

        let assign53000_e87314: f64 = if p.p918 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard817 = assign53000_e87314;
        locals.var_guard817_rv = 0.0;

        let assign53010_e87317: f64 = if p.p918 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard818 = assign53010_e87317;
        locals.var_guard818_rv = 0.0;

        let (assign53020_e87333, assign53020_e87333_d_n3, assign53020_e87333_d_n4, assign53020_e87333_d_n5, assign53020_e87333_d_n6, assign53020_e87333_d_n7, assign53020_e87333_d_n8, assign53020_e87333_d_n9, assign53020_e87333_d_n10, assign53020_e87333_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 != 0.0)) {
        let assign53020_e87330: f64 = (locals.var_arg).sqrt();
        let assign53020_e87331: f64 = (1.0 / assign53020_e87330);
        (assign53020_e87331, (-((locals.var_arg_dn3 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn4 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn5 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn6 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn7 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn8 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn9 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn10 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn11 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign53020_e87333;
        locals.var_sarg_dn3 = assign53020_e87333_d_n3;
        locals.var_sarg_dn4 = assign53020_e87333_d_n4;
        locals.var_sarg_dn5 = assign53020_e87333_d_n5;
        locals.var_sarg_dn6 = assign53020_e87333_d_n6;
        locals.var_sarg_dn7 = assign53020_e87333_d_n7;
        locals.var_sarg_dn8 = assign53020_e87333_d_n8;
        locals.var_sarg_dn9 = assign53020_e87333_d_n9;
        locals.var_sarg_dn10 = assign53020_e87333_d_n10;
        locals.var_sarg_dn11 = assign53020_e87333_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign53030_e87352, assign53030_e87352_d_n3, assign53030_e87352_d_n4, assign53030_e87352_d_n5, assign53030_e87352_d_n6, assign53030_e87352_d_n7, assign53030_e87352_d_n8, assign53030_e87352_d_n9, assign53030_e87352_d_n10, assign53030_e87352_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 == 0.0)) {
        let assign53030_e87346: f64 = (-p.p918);
        let assign53030_e87348: f64 = (locals.var_arg).ln();
        let assign53030_e87349: f64 = (assign53030_e87346 * assign53030_e87348);
        let assign53030_e87350: f64 = { let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53030_e87350, ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign53030_e87352;
        locals.var_sarg_dn3 = assign53030_e87352_d_n3;
        locals.var_sarg_dn4 = assign53030_e87352_d_n4;
        locals.var_sarg_dn5 = assign53030_e87352_d_n5;
        locals.var_sarg_dn6 = assign53030_e87352_d_n6;
        locals.var_sarg_dn7 = assign53030_e87352_d_n7;
        locals.var_sarg_dn8 = assign53030_e87352_d_n8;
        locals.var_sarg_dn9 = assign53030_e87352_d_n9;
        locals.var_sarg_dn10 = assign53030_e87352_d_n10;
        locals.var_sarg_dn11 = assign53030_e87352_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign53040_e87375, assign53040_e87375_d_n3, assign53040_e87375_d_n4, assign53040_e87375_d_n5, assign53040_e87375_d_n6, assign53040_e87375_d_n7, assign53040_e87375_d_n8, assign53040_e87375_d_n9, assign53040_e87375_d_n10, assign53040_e87375_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign53040_e87363: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign53040_e87367: f64 = (locals.var_arg * locals.var_sarg);
        let assign53040_e87368: f64 = (1.0 - assign53040_e87367);
        let assign53040_e87369: f64 = (assign53040_e87363 * assign53040_e87368);
        let assign53040_e87372: f64 = (1.0 - p.p918);
        let assign53040_e87373: f64 = (assign53040_e87369 / assign53040_e87372);
        (assign53040_e87373, ((assign53040_e87363 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign53040_e87372), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign53040_e87368) + (assign53040_e87363 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign53040_e87372), (((((locals.var_pbswgd_t_dn5 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn5)) * assign53040_e87368) + (assign53040_e87363 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7)))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8)))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9)))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10)))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11)))) / assign53040_e87372),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign53040_e87375;
        locals.var_qbdj3_dn3 = assign53040_e87375_d_n3;
        locals.var_qbdj3_dn4 = assign53040_e87375_d_n4;
        locals.var_qbdj3_dn5 = assign53040_e87375_d_n5;
        locals.var_qbdj3_dn6 = assign53040_e87375_d_n6;
        locals.var_qbdj3_dn7 = assign53040_e87375_d_n7;
        locals.var_qbdj3_dn8 = assign53040_e87375_d_n8;
        locals.var_qbdj3_dn9 = assign53040_e87375_d_n9;
        locals.var_qbdj3_dn10 = assign53040_e87375_d_n10;
        locals.var_qbdj3_dn11 = assign53040_e87375_d_n11;
        locals.var_qbdj3_rv = 0.0;

        let (assign53050_e87393, assign53050_e87393_d_n3, assign53050_e87393_d_n4, assign53050_e87393_d_n5, assign53050_e87393_d_n6, assign53050_e87393_d_n7, assign53050_e87393_d_n8, assign53050_e87393_d_n9, assign53050_e87393_d_n10, assign53050_e87393_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 == 0.0)) {
        let assign53050_e87387: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign53050_e87389: f64 = (locals.var_arg).ln();
        let assign53050_e87390: f64 = (-assign53050_e87389);
        let assign53050_e87391: f64 = (assign53050_e87387 * assign53050_e87390);
        (assign53050_e87391, (assign53050_e87387 * (-(locals.var_arg_dn3 / locals.var_arg))), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign53050_e87390) + (assign53050_e87387 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbswgd_t_dn5 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn5)) * assign53050_e87390) + (assign53050_e87387 * (-(locals.var_arg_dn5 / locals.var_arg)))), (assign53050_e87387 * (-(locals.var_arg_dn6 / locals.var_arg))), (assign53050_e87387 * (-(locals.var_arg_dn7 / locals.var_arg))), (assign53050_e87387 * (-(locals.var_arg_dn8 / locals.var_arg))), (assign53050_e87387 * (-(locals.var_arg_dn9 / locals.var_arg))), (assign53050_e87387 * (-(locals.var_arg_dn10 / locals.var_arg))), (assign53050_e87387 * (-(locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign53050_e87393;
        locals.var_qbdj3_dn3 = assign53050_e87393_d_n3;
        locals.var_qbdj3_dn4 = assign53050_e87393_d_n4;
        locals.var_qbdj3_dn5 = assign53050_e87393_d_n5;
        locals.var_qbdj3_dn6 = assign53050_e87393_d_n6;
        locals.var_qbdj3_dn7 = assign53050_e87393_d_n7;
        locals.var_qbdj3_dn8 = assign53050_e87393_d_n8;
        locals.var_qbdj3_dn9 = assign53050_e87393_d_n9;
        locals.var_qbdj3_dn10 = assign53050_e87393_d_n10;
        locals.var_qbdj3_dn11 = assign53050_e87393_d_n11;
        locals.var_qbdj3_rv = 0.0;

        let (assign53060_e87419, assign53060_e87419_d_n3, assign53060_e87419_d_n4, assign53060_e87419_d_n5, assign53060_e87419_d_n6, assign53060_e87419_d_n7, assign53060_e87419_d_n8, assign53060_e87419_d_n9, assign53060_e87419_d_n10, assign53060_e87419_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 == 0.0)) {
        let assign53060_e87404: f64 = (locals.var_t1 - 1.0);
        let assign53060_e87405: f64 = (locals.var_czbdswg_p1 * assign53060_e87404);
        let assign53060_e87408: f64 = (5.0 * p.p918);
        let assign53060_e87411: f64 = (locals.var_t1 - 1.0);
        let assign53060_e87412: f64 = (assign53060_e87408 * assign53060_e87411);
        let assign53060_e87415: f64 = (1.0 + p.p918);
        let assign53060_e87416: f64 = (assign53060_e87412 + assign53060_e87415);
        let assign53060_e87417: f64 = (assign53060_e87405 * assign53060_e87416);
        (assign53060_e87417, (((locals.var_czbdswg_p1 * locals.var_t1_dn3) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn3))), (((locals.var_czbdswg_p1 * locals.var_t1_dn4) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn4))), (((locals.var_czbdswg_p1 * locals.var_t1_dn5) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn5))), (((locals.var_czbdswg_p1 * locals.var_t1_dn6) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn6))), (((locals.var_czbdswg_p1 * locals.var_t1_dn7) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn7))), (((locals.var_czbdswg_p1 * locals.var_t1_dn8) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn8))), (((locals.var_czbdswg_p1 * locals.var_t1_dn9) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn9))), (((locals.var_czbdswg_p1 * locals.var_t1_dn10) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn10))), (((locals.var_czbdswg_p1 * locals.var_t1_dn11) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53060_e87419;
        locals.var_t2_dn3 = assign53060_e87419_d_n3;
        locals.var_t2_dn4 = assign53060_e87419_d_n4;
        locals.var_t2_dn5 = assign53060_e87419_d_n5;
        locals.var_t2_dn6 = assign53060_e87419_d_n6;
        locals.var_t2_dn7 = assign53060_e87419_d_n7;
        locals.var_t2_dn8 = assign53060_e87419_d_n8;
        locals.var_t2_dn9 = assign53060_e87419_d_n9;
        locals.var_t2_dn10 = assign53060_e87419_d_n10;
        locals.var_t2_dn11 = assign53060_e87419_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign53070_e87435, assign53070_e87435_d_n3, assign53070_e87435_d_n4, assign53070_e87435_d_n5, assign53070_e87435_d_n6, assign53070_e87435_d_n7, assign53070_e87435_d_n8, assign53070_e87435_d_n9, assign53070_e87435_d_n10, assign53070_e87435_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 == 0.0)) {
        let assign53070_e87429: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign53070_e87432: f64 = (locals.var_t2 + locals.var_czbdswg_p2);
        let assign53070_e87433: f64 = (assign53070_e87429 * assign53070_e87432);
        (assign53070_e87433, (assign53070_e87429 * locals.var_t2_dn3), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign53070_e87432) + (assign53070_e87429 * locals.var_t2_dn4)), ((((locals.var_pbswgd_t_dn5 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn5)) * assign53070_e87432) + (assign53070_e87429 * locals.var_t2_dn5)), (assign53070_e87429 * locals.var_t2_dn6), (assign53070_e87429 * locals.var_t2_dn7), (assign53070_e87429 * locals.var_t2_dn8), (assign53070_e87429 * locals.var_t2_dn9), (assign53070_e87429 * locals.var_t2_dn10), (assign53070_e87429 * locals.var_t2_dn11),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign53070_e87435;
        locals.var_qbdj3_dn3 = assign53070_e87435_d_n3;
        locals.var_qbdj3_dn4 = assign53070_e87435_d_n4;
        locals.var_qbdj3_dn5 = assign53070_e87435_d_n5;
        locals.var_qbdj3_dn6 = assign53070_e87435_d_n6;
        locals.var_qbdj3_dn7 = assign53070_e87435_d_n7;
        locals.var_qbdj3_dn8 = assign53070_e87435_d_n8;
        locals.var_qbdj3_dn9 = assign53070_e87435_d_n9;
        locals.var_qbdj3_dn10 = assign53070_e87435_d_n10;
        locals.var_qbdj3_dn11 = assign53070_e87435_d_n11;
        locals.var_qbdj3_rv = 0.0;

        let (assign53080_e87443, assign53080_e87443_d_n3, assign53080_e87443_d_n4, assign53080_e87443_d_n5, assign53080_e87443_d_n6, assign53080_e87443_d_n7, assign53080_e87443_d_n8, assign53080_e87443_d_n9, assign53080_e87443_d_n10, assign53080_e87443_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard815 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign53080_e87443;
        locals.var_qbdj3_dn3 = assign53080_e87443_d_n3;
        locals.var_qbdj3_dn4 = assign53080_e87443_d_n4;
        locals.var_qbdj3_dn5 = assign53080_e87443_d_n5;
        locals.var_qbdj3_dn6 = assign53080_e87443_d_n6;
        locals.var_qbdj3_dn7 = assign53080_e87443_d_n7;
        locals.var_qbdj3_dn8 = assign53080_e87443_d_n8;
        locals.var_qbdj3_dn9 = assign53080_e87443_d_n9;
        locals.var_qbdj3_dn10 = assign53080_e87443_d_n10;
        locals.var_qbdj3_dn11 = assign53080_e87443_d_n11;
        locals.var_qbdj3_rv = 0.0;

        let (assign53090_e87452, assign53090_e87452_d_n3, assign53090_e87452_d_n4, assign53090_e87452_d_n5, assign53090_e87452_d_n6, assign53090_e87452_d_n7, assign53090_e87452_d_n8, assign53090_e87452_d_n9, assign53090_e87452_d_n10, assign53090_e87452_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53090_e87448: f64 = (p.p919 * locals.var_ibddif);
        let assign53090_e87450: f64 = (assign53090_e87448 * p.p2);
        (assign53090_e87450, ((p.p919 * locals.var_ibddif_dn3) * p.p2), ((p.p919 * locals.var_ibddif_dn4) * p.p2), ((p.p919 * locals.var_ibddif_dn5) * p.p2), ((p.p919 * locals.var_ibddif_dn6) * p.p2), ((p.p919 * locals.var_ibddif_dn7) * p.p2), ((p.p919 * locals.var_ibddif_dn8) * p.p2), ((p.p919 * locals.var_ibddif_dn9) * p.p2), ((p.p919 * locals.var_ibddif_dn10) * p.p2), ((p.p919 * locals.var_ibddif_dn11) * p.p2),)
    } else {
        (locals.var_qbdj4, locals.var_qbdj4_dn3, locals.var_qbdj4_dn4, locals.var_qbdj4_dn5, locals.var_qbdj4_dn6, locals.var_qbdj4_dn7, locals.var_qbdj4_dn8, locals.var_qbdj4_dn9, locals.var_qbdj4_dn10, locals.var_qbdj4_dn11,)
    }
};
        locals.var_qbdj4 = assign53090_e87452;
        locals.var_qbdj4_dn3 = assign53090_e87452_d_n3;
        locals.var_qbdj4_dn4 = assign53090_e87452_d_n4;
        locals.var_qbdj4_dn5 = assign53090_e87452_d_n5;
        locals.var_qbdj4_dn6 = assign53090_e87452_d_n6;
        locals.var_qbdj4_dn7 = assign53090_e87452_d_n7;
        locals.var_qbdj4_dn8 = assign53090_e87452_d_n8;
        locals.var_qbdj4_dn9 = assign53090_e87452_d_n9;
        locals.var_qbdj4_dn10 = assign53090_e87452_d_n10;
        locals.var_qbdj4_dn11 = assign53090_e87452_d_n11;
        locals.var_qbdj4_rv = 0.0;

        let (assign53100_e87463, assign53100_e87463_d_n3, assign53100_e87463_d_n4, assign53100_e87463_d_n5, assign53100_e87463_d_n6, assign53100_e87463_d_n7, assign53100_e87463_d_n8, assign53100_e87463_d_n9, assign53100_e87463_d_n10, assign53100_e87463_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53100_e87457: f64 = (locals.var_qbdj1 + locals.var_qbdj2);
        let assign53100_e87459: f64 = (assign53100_e87457 + locals.var_qbdj3);
        let assign53100_e87461: f64 = (assign53100_e87459 + locals.var_qbdj4);
        (assign53100_e87461, (((locals.var_qbdj1_dn3 + locals.var_qbdj2_dn3) + locals.var_qbdj3_dn3) + locals.var_qbdj4_dn3), (((locals.var_qbdj1_dn4 + locals.var_qbdj2_dn4) + locals.var_qbdj3_dn4) + locals.var_qbdj4_dn4), (((locals.var_qbdj1_dn5 + locals.var_qbdj2_dn5) + locals.var_qbdj3_dn5) + locals.var_qbdj4_dn5), (((locals.var_qbdj1_dn6 + locals.var_qbdj2_dn6) + locals.var_qbdj3_dn6) + locals.var_qbdj4_dn6), (((locals.var_qbdj1_dn7 + locals.var_qbdj2_dn7) + locals.var_qbdj3_dn7) + locals.var_qbdj4_dn7), (((locals.var_qbdj1_dn8 + locals.var_qbdj2_dn8) + locals.var_qbdj3_dn8) + locals.var_qbdj4_dn8), (((locals.var_qbdj1_dn9 + locals.var_qbdj2_dn9) + locals.var_qbdj3_dn9) + locals.var_qbdj4_dn9), (((locals.var_qbdj1_dn10 + locals.var_qbdj2_dn10) + locals.var_qbdj3_dn10) + locals.var_qbdj4_dn10), (((locals.var_qbdj1_dn11 + locals.var_qbdj2_dn11) + locals.var_qbdj3_dn11) + locals.var_qbdj4_dn11),)
    } else {
        (locals.var_qbdj, locals.var_qbdj_dn3, locals.var_qbdj_dn4, locals.var_qbdj_dn5, locals.var_qbdj_dn6, locals.var_qbdj_dn7, locals.var_qbdj_dn8, locals.var_qbdj_dn9, locals.var_qbdj_dn10, locals.var_qbdj_dn11,)
    }
};
        locals.var_qbdj = assign53100_e87463;
        locals.var_qbdj_dn3 = assign53100_e87463_d_n3;
        locals.var_qbdj_dn4 = assign53100_e87463_d_n4;
        locals.var_qbdj_dn5 = assign53100_e87463_d_n5;
        locals.var_qbdj_dn6 = assign53100_e87463_d_n6;
        locals.var_qbdj_dn7 = assign53100_e87463_d_n7;
        locals.var_qbdj_dn8 = assign53100_e87463_d_n8;
        locals.var_qbdj_dn9 = assign53100_e87463_d_n9;
        locals.var_qbdj_dn10 = assign53100_e87463_d_n10;
        locals.var_qbdj_dn11 = assign53100_e87463_d_n11;
        locals.var_qbdj_rv = 0.0;

        let assign53110_e87466: f64 = if p.p28 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard819 = assign53110_e87466;
        locals.var_guard819_rv = 0.0;

        let (assign53120_e87477, assign53120_e87477_d_n3, assign53120_e87477_d_n4, assign53120_e87477_d_n5, assign53120_e87477_d_n6, assign53120_e87477_d_n7, assign53120_e87477_d_n8, assign53120_e87477_d_n9, assign53120_e87477_d_n10, assign53120_e87477_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard819 != 0.0)) {
        let assign53120_e87473: f64 = (locals.var_ndep_i / 1e23);
        let assign53120_e87475: f64 = (assign53120_e87473).powf(p.p1144);
        (assign53120_e87475, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn3 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn3 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn4 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn4 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn5 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn5 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn6 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn6 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn7 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn7 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn8 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn8 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn9 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn9 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn10 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn10 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn11 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn11 / 1e23) / assign53120_e87473))) },)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign53120_e87477;
        locals.var_t1_dn3 = assign53120_e87477_d_n3;
        locals.var_t1_dn4 = assign53120_e87477_d_n4;
        locals.var_t1_dn5 = assign53120_e87477_d_n5;
        locals.var_t1_dn6 = assign53120_e87477_d_n6;
        locals.var_t1_dn7 = assign53120_e87477_d_n7;
        locals.var_t1_dn8 = assign53120_e87477_d_n8;
        locals.var_t1_dn9 = assign53120_e87477_d_n9;
        locals.var_t1_dn10 = assign53120_e87477_d_n10;
        locals.var_t1_dn11 = assign53120_e87477_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign53130_e87488, assign53130_e87488_d_n3, assign53130_e87488_d_n4, assign53130_e87488_d_n5, assign53130_e87488_d_n6, assign53130_e87488_d_n7, assign53130_e87488_d_n8, assign53130_e87488_d_n9, assign53130_e87488_d_n10, assign53130_e87488_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard819 != 0.0)) {
        let assign53130_e87484: f64 = (300.0 / locals.var_devtemp);
        let assign53130_e87486: f64 = (assign53130_e87484).powf(p.p1145);
        (assign53130_e87486, 0.0, if 0.0 == 0.0 && ((p.p1145) as f64).is_finite() && ((p.p1145) as f64).fract() == 0.0 { if p.p1145 == 0.0 { 0.0 } else { (p.p1145 * ((assign53130_e87484).powf(p.p1145 - 1.0) * (-((300.0 * locals.var_devtemp_dn4) / (locals.var_devtemp * locals.var_devtemp))))) } } else { (assign53130_e87486 * (p.p1145 * ((-((300.0 * locals.var_devtemp_dn4) / (locals.var_devtemp * locals.var_devtemp))) / assign53130_e87484))) }, if 0.0 == 0.0 && ((p.p1145) as f64).is_finite() && ((p.p1145) as f64).fract() == 0.0 { if p.p1145 == 0.0 { 0.0 } else { (p.p1145 * ((assign53130_e87484).powf(p.p1145 - 1.0) * (-((300.0 * locals.var_devtemp_dn5) / (locals.var_devtemp * locals.var_devtemp))))) } } else { (assign53130_e87486 * (p.p1145 * ((-((300.0 * locals.var_devtemp_dn5) / (locals.var_devtemp * locals.var_devtemp))) / assign53130_e87484))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53130_e87488;
        locals.var_t2_dn3 = assign53130_e87488_d_n3;
        locals.var_t2_dn4 = assign53130_e87488_d_n4;
        locals.var_t2_dn5 = assign53130_e87488_d_n5;
        locals.var_t2_dn6 = assign53130_e87488_d_n6;
        locals.var_t2_dn7 = assign53130_e87488_d_n7;
        locals.var_t2_dn8 = assign53130_e87488_d_n8;
        locals.var_t2_dn9 = assign53130_e87488_d_n9;
        locals.var_t2_dn10 = assign53130_e87488_d_n10;
        locals.var_t2_dn11 = assign53130_e87488_d_n11;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_183(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (assign53140_e87501, assign53140_e87501_d_n3, assign53140_e87501_d_n4, assign53140_e87501_d_n5, assign53140_e87501_d_n6, assign53140_e87501_d_n7, assign53140_e87501_d_n8, assign53140_e87501_d_n9, assign53140_e87501_d_n10, assign53140_e87501_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard819 != 0.0)) {
        let assign53140_e87495: f64 = (locals.var_devsign * p.p1143);
        let assign53140_e87497: f64 = (assign53140_e87495 * (nv10 - nv7));
        let assign53140_e87499: f64 = (assign53140_e87497 / locals.var_vt);
        (assign53140_e87499, 0.0, (-((assign53140_e87497 * locals.var_vt_dn4) / (locals.var_vt * locals.var_vt))), (-((assign53140_e87497 * locals.var_vt_dn5) / (locals.var_vt * locals.var_vt))), 0.0, ((-assign53140_e87495) / locals.var_vt), 0.0, 0.0, (assign53140_e87495 / locals.var_vt), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53140_e87501;
        locals.var_t3_dn3 = assign53140_e87501_d_n3;
        locals.var_t3_dn4 = assign53140_e87501_d_n4;
        locals.var_t3_dn5 = assign53140_e87501_d_n5;
        locals.var_t3_dn6 = assign53140_e87501_d_n6;
        locals.var_t3_dn7 = assign53140_e87501_d_n7;
        locals.var_t3_dn8 = assign53140_e87501_d_n8;
        locals.var_t3_dn9 = assign53140_e87501_d_n9;
        locals.var_t3_dn10 = assign53140_e87501_d_n10;
        locals.var_t3_dn11 = assign53140_e87501_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign53200_e87598, assign53200_e87598_d_n3, assign53200_e87598_d_n4, assign53200_e87598_d_n5, assign53200_e87598_d_n6, assign53200_e87598_d_n7, assign53200_e87598_d_n8, assign53200_e87598_d_n9, assign53200_e87598_d_n10, assign53200_e87598_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53200_e87594: f64 = (2.0 * locals.var_vsat_a);
        let assign53200_e87596: f64 = (assign53200_e87594 / locals.var_ueff);
        (assign53200_e87596, ((((2.0 * locals.var_vsat_a_dn3) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn3)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn4) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn4)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn5) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn5)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn6) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn6)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn7) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn7)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn8) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn8)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn9) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn9)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn10) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn10)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn11) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn11)) / (locals.var_ueff * locals.var_ueff)),)
    } else {
        (locals.var_esatnoi, locals.var_esatnoi_dn3, locals.var_esatnoi_dn4, locals.var_esatnoi_dn5, locals.var_esatnoi_dn6, locals.var_esatnoi_dn7, locals.var_esatnoi_dn8, locals.var_esatnoi_dn9, locals.var_esatnoi_dn10, locals.var_esatnoi_dn11,)
    }
};
        locals.var_esatnoi = assign53200_e87598;
        locals.var_esatnoi_dn3 = assign53200_e87598_d_n3;
        locals.var_esatnoi_dn4 = assign53200_e87598_d_n4;
        locals.var_esatnoi_dn5 = assign53200_e87598_d_n5;
        locals.var_esatnoi_dn6 = assign53200_e87598_d_n6;
        locals.var_esatnoi_dn7 = assign53200_e87598_d_n7;
        locals.var_esatnoi_dn8 = assign53200_e87598_d_n8;
        locals.var_esatnoi_dn9 = assign53200_e87598_d_n9;
        locals.var_esatnoi_dn10 = assign53200_e87598_d_n10;
        locals.var_esatnoi_dn11 = assign53200_e87598_d_n11;
        locals.var_esatnoi_rv = 0.0;

        let assign53210_e87601: f64 = if p.p1011 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard820 = assign53210_e87601;
        locals.var_guard820_rv = 0.0;

        let (assign53220_e87608, assign53220_e87608_d_n3, assign53220_e87608_d_n4, assign53220_e87608_d_n5, assign53220_e87608_d_n6, assign53220_e87608_d_n7, assign53220_e87608_d_n8, assign53220_e87608_d_n9, assign53220_e87608_d_n10, assign53220_e87608_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard820 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11,)
    }
};
        locals.var_delclm = assign53220_e87608;
        locals.var_delclm_dn3 = assign53220_e87608_d_n3;
        locals.var_delclm_dn4 = assign53220_e87608_d_n4;
        locals.var_delclm_dn5 = assign53220_e87608_d_n5;
        locals.var_delclm_dn6 = assign53220_e87608_d_n6;
        locals.var_delclm_dn7 = assign53220_e87608_d_n7;
        locals.var_delclm_dn8 = assign53220_e87608_d_n8;
        locals.var_delclm_dn9 = assign53220_e87608_d_n9;
        locals.var_delclm_dn10 = assign53220_e87608_d_n10;
        locals.var_delclm_dn11 = assign53220_e87608_d_n11;
        locals.var_delclm_rv = 0.0;

        let (assign53230_e87622, assign53230_e87622_d_n3, assign53230_e87622_d_n4, assign53230_e87622_d_n5, assign53230_e87622_d_n6, assign53230_e87622_d_n7, assign53230_e87622_d_n8, assign53230_e87622_d_n9, assign53230_e87622_d_n10, assign53230_e87622_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard820 == 0.0)) {
        let assign53230_e87616: f64 = (locals.var_diffvds / locals.var_litl);
        let assign53230_e87618: f64 = (assign53230_e87616 + p.p1011);
        let assign53230_e87620: f64 = (assign53230_e87618 / locals.var_esatnoi);
        (assign53230_e87620, ((((locals.var_diffvds_dn3 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn3)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn4 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn4)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn5 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn5)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn6 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn6)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn7 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn7)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn8 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn8)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn9 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn9)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn10 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn10)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn11 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn11)) / (locals.var_esatnoi * locals.var_esatnoi)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign53230_e87622;
        locals.var_t0_dn3 = assign53230_e87622_d_n3;
        locals.var_t0_dn4 = assign53230_e87622_d_n4;
        locals.var_t0_dn5 = assign53230_e87622_d_n5;
        locals.var_t0_dn6 = assign53230_e87622_d_n6;
        locals.var_t0_dn7 = assign53230_e87622_d_n7;
        locals.var_t0_dn8 = assign53230_e87622_d_n8;
        locals.var_t0_dn9 = assign53230_e87622_d_n9;
        locals.var_t0_dn10 = assign53230_e87622_d_n10;
        locals.var_t0_dn11 = assign53230_e87622_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign53240_e87635, assign53240_e87635_d_n3, assign53240_e87635_d_n4, assign53240_e87635_d_n5, assign53240_e87635_d_n6, assign53240_e87635_d_n7, assign53240_e87635_d_n8, assign53240_e87635_d_n9, assign53240_e87635_d_n10, assign53240_e87635_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard820 == 0.0)) {
        let assign53240_e87631: f64 = (locals.var_t0).max(1e-38);
        let assign53240_e87632: f64 = (assign53240_e87631).ln();
        let assign53240_e87633: f64 = (locals.var_litl * assign53240_e87632);
        (assign53240_e87633, (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn3 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn4 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn5 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn6 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn7 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn8 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn9 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn10 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn11 } else { 0.0 } / assign53240_e87631)),)
    } else {
        (locals.var_delclm, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11,)
    }
};
        locals.var_delclm = assign53240_e87635;
        locals.var_delclm_dn3 = assign53240_e87635_d_n3;
        locals.var_delclm_dn4 = assign53240_e87635_d_n4;
        locals.var_delclm_dn5 = assign53240_e87635_d_n5;
        locals.var_delclm_dn6 = assign53240_e87635_d_n6;
        locals.var_delclm_dn7 = assign53240_e87635_d_n7;
        locals.var_delclm_dn8 = assign53240_e87635_d_n8;
        locals.var_delclm_dn9 = assign53240_e87635_d_n9;
        locals.var_delclm_dn10 = assign53240_e87635_d_n10;
        locals.var_delclm_dn11 = assign53240_e87635_d_n11;
        locals.var_delclm_rv = 0.0;

        let assign53250_e87638: f64 = if locals.var_delclm < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard821 = assign53250_e87638;
        locals.var_guard821_rv = 0.0;

        let (assign53260_e87648, assign53260_e87648_d_n3, assign53260_e87648_d_n4, assign53260_e87648_d_n5, assign53260_e87648_d_n6, assign53260_e87648_d_n7, assign53260_e87648_d_n8, assign53260_e87648_d_n9, assign53260_e87648_d_n10, assign53260_e87648_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard820 == 0.0)) && (locals.var_guard821 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11,)
    }
};
        locals.var_delclm = assign53260_e87648;
        locals.var_delclm_dn3 = assign53260_e87648_d_n3;
        locals.var_delclm_dn4 = assign53260_e87648_d_n4;
        locals.var_delclm_dn5 = assign53260_e87648_d_n5;
        locals.var_delclm_dn6 = assign53260_e87648_d_n6;
        locals.var_delclm_dn7 = assign53260_e87648_d_n7;
        locals.var_delclm_dn8 = assign53260_e87648_d_n8;
        locals.var_delclm_dn9 = assign53260_e87648_d_n9;
        locals.var_delclm_dn10 = assign53260_e87648_d_n10;
        locals.var_delclm_dn11 = assign53260_e87648_d_n11;
        locals.var_delclm_rv = 0.0;

        let (assign53270_e87661, assign53270_e87661_d_n3, assign53270_e87661_d_n4, assign53270_e87661_d_n5, assign53270_e87661_d_n6, assign53270_e87661_d_n7, assign53270_e87661_d_n8, assign53270_e87661_d_n9, assign53270_e87661_d_n10, assign53270_e87661_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53270_e87653: f64 = (locals.var_vt / 1.602176462e-19);
        let assign53270_e87656: f64 = (locals.var_cox + locals.var_cdep);
        let assign53270_e87658: f64 = (assign53270_e87656 + locals.var_cit_i);
        let assign53270_e87659: f64 = (assign53270_e87653 * assign53270_e87658);
        (assign53270_e87659, (assign53270_e87653 * locals.var_cdep_dn3), (((locals.var_vt_dn4 / 1.602176462e-19) * assign53270_e87658) + (assign53270_e87653 * locals.var_cdep_dn4)), (((locals.var_vt_dn5 / 1.602176462e-19) * assign53270_e87658) + (assign53270_e87653 * locals.var_cdep_dn5)), (assign53270_e87653 * locals.var_cdep_dn6), (assign53270_e87653 * locals.var_cdep_dn7), (assign53270_e87653 * locals.var_cdep_dn8), (assign53270_e87653 * locals.var_cdep_dn9), (assign53270_e87653 * locals.var_cdep_dn10), (assign53270_e87653 * locals.var_cdep_dn11),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11,)
    }
};
        locals.var_nstar = assign53270_e87661;
        locals.var_nstar_dn3 = assign53270_e87661_d_n3;
        locals.var_nstar_dn4 = assign53270_e87661_d_n4;
        locals.var_nstar_dn5 = assign53270_e87661_d_n5;
        locals.var_nstar_dn6 = assign53270_e87661_d_n6;
        locals.var_nstar_dn7 = assign53270_e87661_d_n7;
        locals.var_nstar_dn8 = assign53270_e87661_d_n8;
        locals.var_nstar_dn9 = assign53270_e87661_d_n9;
        locals.var_nstar_dn10 = assign53270_e87661_d_n10;
        locals.var_nstar_dn11 = assign53270_e87661_d_n11;
        locals.var_nstar_rv = 0.0;

        let (assign53280_e87680, assign53280_e87680_d_n3, assign53280_e87680_d_n4, assign53280_e87680_d_n5, assign53280_e87680_d_n6, assign53280_e87680_d_n7, assign53280_e87680_d_n8, assign53280_e87680_d_n9, assign53280_e87680_d_n10, assign53280_e87680_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53280_e87666: f64 = (2.0 * locals.var_nq);
        let assign53280_e87668: f64 = (assign53280_e87666 * locals.var_cox);
        let assign53280_e87670: f64 = (assign53280_e87668 * locals.var_vt);
        let assign53280_e87672: f64 = (assign53280_e87670 * locals.var_qdeff);
        let assign53280_e87674: f64 = (assign53280_e87672 * locals.var_mnud1);
        let assign53280_e87676: f64 = (assign53280_e87674 * locals.var_mnud);
        let assign53280_e87678: f64 = (assign53280_e87676 / 1.602176462e-19);
        (assign53280_e87678, ((((((((((2.0 * locals.var_nq_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn3)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn3)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn3)) / 1.602176462e-19), (((((((((((2.0 * locals.var_nq_dn4) * locals.var_cox) * locals.var_vt) + (assign53280_e87668 * locals.var_vt_dn4)) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn4)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn4)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn4)) / 1.602176462e-19), (((((((((((2.0 * locals.var_nq_dn5) * locals.var_cox) * locals.var_vt) + (assign53280_e87668 * locals.var_vt_dn5)) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn5)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn5)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn5)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn6)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn6)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn6)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn7)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn7)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn7)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn8)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn8)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn8)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn9)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn9)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn9)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn10)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn10)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn10)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn11)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn11)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn11)) / 1.602176462e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11,)
    }
};
        locals.var_nl = assign53280_e87680;
        locals.var_nl_dn3 = assign53280_e87680_d_n3;
        locals.var_nl_dn4 = assign53280_e87680_d_n4;
        locals.var_nl_dn5 = assign53280_e87680_d_n5;
        locals.var_nl_dn6 = assign53280_e87680_d_n6;
        locals.var_nl_dn7 = assign53280_e87680_d_n7;
        locals.var_nl_dn8 = assign53280_e87680_d_n8;
        locals.var_nl_dn9 = assign53280_e87680_d_n9;
        locals.var_nl_dn10 = assign53280_e87680_d_n10;
        locals.var_nl_dn11 = assign53280_e87680_d_n11;
        locals.var_nl_rv = 0.0;

        let (assign53290_e87696, assign53290_e87696_d_n3, assign53290_e87696_d_n4, assign53290_e87696_d_n5, assign53290_e87696_d_n6, assign53290_e87696_d_n7, assign53290_e87696_d_n8, assign53290_e87696_d_n9, assign53290_e87696_d_n10, assign53290_e87696_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53290_e87685: f64 = (1.602176462e-19 * 1.602176462e-19);
        let assign53290_e87687: f64 = (assign53290_e87685 * 1.602176462e-19);
        let assign53290_e87689: f64 = (assign53290_e87687 * locals.var_vt);
        let assign53290_e87691: f64 = (locals.var_ids).abs();
        let assign53290_e87692: f64 = (assign53290_e87689 * assign53290_e87691);
        let assign53290_e87694: f64 = (assign53290_e87692 * locals.var_ueff);
        (assign53290_e87694, (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn3 } else { (-locals.var_ids_dn3) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn3)), (((((assign53290_e87687 * locals.var_vt_dn4) * assign53290_e87691) + (assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn4 } else { (-locals.var_ids_dn4) })) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn4)), (((((assign53290_e87687 * locals.var_vt_dn5) * assign53290_e87691) + (assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn5 } else { (-locals.var_ids_dn5) })) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn5)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn6 } else { (-locals.var_ids_dn6) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn6)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn7 } else { (-locals.var_ids_dn7) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn7)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn8 } else { (-locals.var_ids_dn8) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn8)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn9 } else { (-locals.var_ids_dn9) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn9)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn10 } else { (-locals.var_ids_dn10) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn10)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn11 } else { (-locals.var_ids_dn11) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn11)),)
    } else {
        (locals.var_t0a, locals.var_t0a_dn3, locals.var_t0a_dn4, locals.var_t0a_dn5, locals.var_t0a_dn6, locals.var_t0a_dn7, locals.var_t0a_dn8, locals.var_t0a_dn9, locals.var_t0a_dn10, locals.var_t0a_dn11,)
    }
};
        locals.var_t0a = assign53290_e87696;
        locals.var_t0a_dn3 = assign53290_e87696_d_n3;
        locals.var_t0a_dn4 = assign53290_e87696_d_n4;
        locals.var_t0a_dn5 = assign53290_e87696_d_n5;
        locals.var_t0a_dn6 = assign53290_e87696_d_n6;
        locals.var_t0a_dn7 = assign53290_e87696_d_n7;
        locals.var_t0a_dn8 = assign53290_e87696_d_n8;
        locals.var_t0a_dn9 = assign53290_e87696_d_n9;
        locals.var_t0a_dn10 = assign53290_e87696_d_n10;
        locals.var_t0a_dn11 = assign53290_e87696_d_n11;
        locals.var_t0a_rv = 0.0;

        let (assign53300_e87707, assign53300_e87707_d_n3, assign53300_e87707_d_n4, assign53300_e87707_d_n5, assign53300_e87707_d_n6, assign53300_e87707_d_n7, assign53300_e87707_d_n8, assign53300_e87707_d_n9, assign53300_e87707_d_n10, assign53300_e87707_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53300_e87701: f64 = (1.602176462e-19 * locals.var_vt);
        let assign53300_e87703: f64 = (assign53300_e87701 * locals.var_ids);
        let assign53300_e87705: f64 = (assign53300_e87703 * locals.var_ids);
        (assign53300_e87705, (((assign53300_e87701 * locals.var_ids_dn3) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn3)), (((((1.602176462e-19 * locals.var_vt_dn4) * locals.var_ids) + (assign53300_e87701 * locals.var_ids_dn4)) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn4)), (((((1.602176462e-19 * locals.var_vt_dn5) * locals.var_ids) + (assign53300_e87701 * locals.var_ids_dn5)) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn5)), (((assign53300_e87701 * locals.var_ids_dn6) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn6)), (((assign53300_e87701 * locals.var_ids_dn7) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn7)), (((assign53300_e87701 * locals.var_ids_dn8) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn8)), (((assign53300_e87701 * locals.var_ids_dn9) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn9)), (((assign53300_e87701 * locals.var_ids_dn10) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn10)), (((assign53300_e87701 * locals.var_ids_dn11) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn11)),)
    } else {
        (locals.var_t0b, locals.var_t0b_dn3, locals.var_t0b_dn4, locals.var_t0b_dn5, locals.var_t0b_dn6, locals.var_t0b_dn7, locals.var_t0b_dn8, locals.var_t0b_dn9, locals.var_t0b_dn10, locals.var_t0b_dn11,)
    }
};
        locals.var_t0b = assign53300_e87707;
        locals.var_t0b_dn3 = assign53300_e87707_d_n3;
        locals.var_t0b_dn4 = assign53300_e87707_d_n4;
        locals.var_t0b_dn5 = assign53300_e87707_d_n5;
        locals.var_t0b_dn6 = assign53300_e87707_d_n6;
        locals.var_t0b_dn7 = assign53300_e87707_d_n7;
        locals.var_t0b_dn8 = assign53300_e87707_d_n8;
        locals.var_t0b_dn9 = assign53300_e87707_d_n9;
        locals.var_t0b_dn10 = assign53300_e87707_d_n10;
        locals.var_t0b_dn11 = assign53300_e87707_d_n11;
        locals.var_t0b_rv = 0.0;

        let (assign53310_e87722, assign53310_e87722_d_n3, assign53310_e87722_d_n4, assign53310_e87722_d_n5, assign53310_e87722_d_n6, assign53310_e87722_d_n7, assign53310_e87722_d_n8, assign53310_e87722_d_n9, assign53310_e87722_d_n10, assign53310_e87722_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53310_e87713: f64 = (p.p1013 * locals.var_nl);
        let assign53310_e87714: f64 = (p.p1012 + assign53310_e87713);
        let assign53310_e87717: f64 = (p.p1014 * locals.var_nl);
        let assign53310_e87719: f64 = (assign53310_e87717 * locals.var_nl);
        let assign53310_e87720: f64 = (assign53310_e87714 + assign53310_e87719);
        (assign53310_e87720, ((p.p1013 * locals.var_nl_dn3) + (((p.p1014 * locals.var_nl_dn3) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn3))), ((p.p1013 * locals.var_nl_dn4) + (((p.p1014 * locals.var_nl_dn4) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn4))), ((p.p1013 * locals.var_nl_dn5) + (((p.p1014 * locals.var_nl_dn5) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn5))), ((p.p1013 * locals.var_nl_dn6) + (((p.p1014 * locals.var_nl_dn6) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn6))), ((p.p1013 * locals.var_nl_dn7) + (((p.p1014 * locals.var_nl_dn7) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn7))), ((p.p1013 * locals.var_nl_dn8) + (((p.p1014 * locals.var_nl_dn8) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn8))), ((p.p1013 * locals.var_nl_dn9) + (((p.p1014 * locals.var_nl_dn9) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn9))), ((p.p1013 * locals.var_nl_dn10) + (((p.p1014 * locals.var_nl_dn10) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn10))), ((p.p1013 * locals.var_nl_dn11) + (((p.p1014 * locals.var_nl_dn11) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn11))),)
    } else {
        (locals.var_t0c, locals.var_t0c_dn3, locals.var_t0c_dn4, locals.var_t0c_dn5, locals.var_t0c_dn6, locals.var_t0c_dn7, locals.var_t0c_dn8, locals.var_t0c_dn9, locals.var_t0c_dn10, locals.var_t0c_dn11,)
    }
};
        locals.var_t0c = assign53310_e87722;
        locals.var_t0c_dn3 = assign53310_e87722_d_n3;
        locals.var_t0c_dn4 = assign53310_e87722_d_n4;
        locals.var_t0c_dn5 = assign53310_e87722_d_n5;
        locals.var_t0c_dn6 = assign53310_e87722_d_n6;
        locals.var_t0c_dn7 = assign53310_e87722_d_n7;
        locals.var_t0c_dn8 = assign53310_e87722_d_n8;
        locals.var_t0c_dn9 = assign53310_e87722_d_n9;
        locals.var_t0c_dn10 = assign53310_e87722_d_n10;
        locals.var_t0c_dn11 = assign53310_e87722_d_n11;
        locals.var_t0c_rv = 0.0;

        let (assign53320_e87733, assign53320_e87733_d_n3, assign53320_e87733_d_n4, assign53320_e87733_d_n5, assign53320_e87733_d_n6, assign53320_e87733_d_n7, assign53320_e87733_d_n8, assign53320_e87733_d_n9, assign53320_e87733_d_n10, assign53320_e87733_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53320_e87727: f64 = (locals.var_nl + locals.var_nstar);
        let assign53320_e87730: f64 = (locals.var_nl + locals.var_nstar);
        let assign53320_e87731: f64 = (assign53320_e87727 * assign53320_e87730);
        (assign53320_e87731, (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn11 + locals.var_nstar_dn11))),)
    } else {
        (locals.var_t0d, locals.var_t0d_dn3, locals.var_t0d_dn4, locals.var_t0d_dn5, locals.var_t0d_dn6, locals.var_t0d_dn7, locals.var_t0d_dn8, locals.var_t0d_dn9, locals.var_t0d_dn10, locals.var_t0d_dn11,)
    }
};
        locals.var_t0d = assign53320_e87733;
        locals.var_t0d_dn3 = assign53320_e87733_d_n3;
        locals.var_t0d_dn4 = assign53320_e87733_d_n4;
        locals.var_t0d_dn5 = assign53320_e87733_d_n5;
        locals.var_t0d_dn6 = assign53320_e87733_d_n6;
        locals.var_t0d_dn7 = assign53320_e87733_d_n7;
        locals.var_t0d_dn8 = assign53320_e87733_d_n8;
        locals.var_t0d_dn9 = assign53320_e87733_d_n9;
        locals.var_t0d_dn10 = assign53320_e87733_d_n10;
        locals.var_t0d_dn11 = assign53320_e87733_d_n11;
        locals.var_t0d_rv = 0.0;

        let (assign53330_e87742, assign53330_e87742_d_n4, assign53330_e87742_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53330_e87738: f64 = (p.p1012 * 1.602176462e-19);
        let assign53330_e87740: f64 = (assign53330_e87738 * locals.var_vt);
        (assign53330_e87740, (assign53330_e87738 * locals.var_vt_dn4), (assign53330_e87738 * locals.var_vt_dn5),)
    } else {
        (locals.var_t0e, locals.var_t0e_dn4, locals.var_t0e_dn5,)
    }
};
        locals.var_t0e = assign53330_e87742;
        locals.var_t0e_dn4 = assign53330_e87742_d_n4;
        locals.var_t0e_dn5 = assign53330_e87742_d_n5;
        locals.var_t0e_rv = 0.0;

        let assign53340_e87745: f64 = if p.p1319 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard822 = assign53340_e87745;
        locals.var_guard822_rv = 0.0;

        let (assign53350_e87752,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        (p.p1320,)
    } else {
        (locals.var_lh1,)
    }
};
        locals.var_lh1 = assign53350_e87752;
        locals.var_lh1_rv = 0.0;

        let assign53360_e87755: f64 = if locals.var_leff > locals.var_lh1 { 1.0 } else { 0.0 };
        locals.var_guard823 = assign53360_e87755;
        locals.var_guard823_rv = 0.0;

        let (assign53370_e87766, assign53370_e87766_d_n3, assign53370_e87766_d_n4, assign53370_e87766_d_n5, assign53370_e87766_d_n6, assign53370_e87766_d_n7, assign53370_e87766_d_n8, assign53370_e87766_d_n9, assign53370_e87766_d_n10, assign53370_e87766_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign53370_e87764: f64 = (locals.var_leff - locals.var_lh1);
        (assign53370_e87764, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign53370_e87766;
        locals.var_t0_dn3 = assign53370_e87766_d_n3;
        locals.var_t0_dn4 = assign53370_e87766_d_n4;
        locals.var_t0_dn5 = assign53370_e87766_d_n5;
        locals.var_t0_dn6 = assign53370_e87766_d_n6;
        locals.var_t0_dn7 = assign53370_e87766_d_n7;
        locals.var_t0_dn8 = assign53370_e87766_d_n8;
        locals.var_t0_dn9 = assign53370_e87766_d_n9;
        locals.var_t0_dn10 = assign53370_e87766_d_n10;
        locals.var_t0_dn11 = assign53370_e87766_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign53380_e87776,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard823 == 0.0)) {
        (locals.var_leff,)
    } else {
        (locals.var_lh1,)
    }
};
        locals.var_lh1 = assign53380_e87776;
        locals.var_lh1_rv = 0.0;

        let (assign53390_e87786, assign53390_e87786_d_n3, assign53390_e87786_d_n4, assign53390_e87786_d_n5, assign53390_e87786_d_n6, assign53390_e87786_d_n7, assign53390_e87786_d_n8, assign53390_e87786_d_n9, assign53390_e87786_d_n10, assign53390_e87786_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard823 == 0.0)) {
        (locals.var_lh1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign53390_e87786;
        locals.var_t0_dn3 = assign53390_e87786_d_n3;
        locals.var_t0_dn4 = assign53390_e87786_d_n4;
        locals.var_t0_dn5 = assign53390_e87786_d_n5;
        locals.var_t0_dn6 = assign53390_e87786_d_n6;
        locals.var_t0_dn7 = assign53390_e87786_d_n7;
        locals.var_t0_dn8 = assign53390_e87786_d_n8;
        locals.var_t0_dn9 = assign53390_e87786_d_n9;
        locals.var_t0_dn10 = assign53390_e87786_d_n10;
        locals.var_t0_dn11 = assign53390_e87786_d_n11;
        locals.var_t0_rv = 0.0;

        let assign53400_e87790: f64 = (locals.var_t0 / 2.0);
        let assign53400_e87791: f64 = if p.p1015 >= assign53400_e87790 { 1.0 } else { 0.0 };
        locals.var_guard824 = assign53400_e87791;
        locals.var_guard824_rv = 0.0;

        let (assign53410_e87800,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard824 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign53410_e87800;
        locals.var_lintnoi_i_rv = 0.0;

        let (assign53420_e87810,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard824 == 0.0)) {
        (p.p1015,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign53420_e87810;
        locals.var_lintnoi_i_rv = 0.0;

        let (assign53430_e87817,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        (locals.var_leff,)
    } else {
        (locals.var_leffnoih,)
    }
};
        locals.var_leffnoih = assign53430_e87817;
        locals.var_leffnoih_rv = 0.0;

        let (assign53440_e87828, assign53440_e87828_d_n3, assign53440_e87828_d_n4, assign53440_e87828_d_n5, assign53440_e87828_d_n6, assign53440_e87828_d_n7, assign53440_e87828_d_n8, assign53440_e87828_d_n9, assign53440_e87828_d_n10, assign53440_e87828_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53440_e87824: f64 = (locals.var_vg - locals.var_vfb_i);
        let assign53440_e87826: f64 = (assign53440_e87824 / locals.var_vt);
        (assign53440_e87826, ((-locals.var_vfb_i_dn3) / locals.var_vt), ((((-locals.var_vfb_i_dn4) * locals.var_vt) - (assign53440_e87824 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt)), ((((-locals.var_vfb_i_dn5) * locals.var_vt) - (assign53440_e87824 * locals.var_vt_dn5)) / (locals.var_vt * locals.var_vt)), ((-locals.var_vfb_i_dn6) / locals.var_vt), ((-locals.var_vfb_i_dn7) / locals.var_vt), ((locals.var_vg_dn8 - locals.var_vfb_i_dn8) / locals.var_vt), ((-locals.var_vfb_i_dn9) / locals.var_vt), ((locals.var_vg_dn10 - locals.var_vfb_i_dn10) / locals.var_vt), ((-locals.var_vfb_i_dn11) / locals.var_vt),)
    } else {
        (locals.var_vgfbh, locals.var_vgfbh_dn3, locals.var_vgfbh_dn4, locals.var_vgfbh_dn5, locals.var_vgfbh_dn6, locals.var_vgfbh_dn7, locals.var_vgfbh_dn8, locals.var_vgfbh_dn9, locals.var_vgfbh_dn10, locals.var_vgfbh_dn11,)
    }
};
        locals.var_vgfbh = assign53440_e87828;
        locals.var_vgfbh_dn3 = assign53440_e87828_d_n3;
        locals.var_vgfbh_dn4 = assign53440_e87828_d_n4;
        locals.var_vgfbh_dn5 = assign53440_e87828_d_n5;
        locals.var_vgfbh_dn6 = assign53440_e87828_d_n6;
        locals.var_vgfbh_dn7 = assign53440_e87828_d_n7;
        locals.var_vgfbh_dn8 = assign53440_e87828_d_n8;
        locals.var_vgfbh_dn9 = assign53440_e87828_d_n9;
        locals.var_vgfbh_dn10 = assign53440_e87828_d_n10;
        locals.var_vgfbh_dn11 = assign53440_e87828_d_n11;
        locals.var_vgfbh_rv = 0.0;

        let (assign53450_e87846, assign53450_e87846_d_n4, assign53450_e87846_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53450_e87835: f64 = (2.0 * 1.602176462e-19);
        let assign53450_e87837: f64 = (assign53450_e87835 * locals.var_epssi);
        let assign53450_e87839: f64 = (assign53450_e87837 * p.p1322);
        let assign53450_e87841: f64 = (assign53450_e87839 / locals.var_vt);
        let assign53450_e87842: f64 = (assign53450_e87841).sqrt();
        let assign53450_e87844: f64 = (assign53450_e87842 / locals.var_cox);
        (assign53450_e87844, (((-((assign53450_e87839 * locals.var_vt_dn4) / (locals.var_vt * locals.var_vt))) / (2.0 * assign53450_e87842)) / locals.var_cox), (((-((assign53450_e87839 * locals.var_vt_dn5) / (locals.var_vt * locals.var_vt))) / (2.0 * assign53450_e87842)) / locals.var_cox),)
    } else {
        (locals.var_gam_h, locals.var_gam_h_dn4, locals.var_gam_h_dn5,)
    }
};
        locals.var_gam_h = assign53450_e87846;
        locals.var_gam_h_dn4 = assign53450_e87846_d_n4;
        locals.var_gam_h_dn5 = assign53450_e87846_d_n5;
        locals.var_gam_h_rv = 0.0;

        let (assign53460_e87856, assign53460_e87856_d_n3, assign53460_e87856_d_n4, assign53460_e87856_d_n5, assign53460_e87856_d_n6, assign53460_e87856_d_n7, assign53460_e87856_d_n8, assign53460_e87856_d_n9, assign53460_e87856_d_n10, assign53460_e87856_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53460_e87853: f64 = (p.p1322 / locals.var_ni);
        let assign53460_e87854: f64 = (assign53460_e87853).ln();
        (assign53460_e87854, ((-((p.p1322 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign53460_e87853),)
    } else {
        (locals.var_phib_h, locals.var_phib_h_dn3, locals.var_phib_h_dn4, locals.var_phib_h_dn5, locals.var_phib_h_dn6, locals.var_phib_h_dn7, locals.var_phib_h_dn8, locals.var_phib_h_dn9, locals.var_phib_h_dn10, locals.var_phib_h_dn11,)
    }
};
        locals.var_phib_h = assign53460_e87856;
        locals.var_phib_h_dn3 = assign53460_e87856_d_n3;
        locals.var_phib_h_dn4 = assign53460_e87856_d_n4;
        locals.var_phib_h_dn5 = assign53460_e87856_d_n5;
        locals.var_phib_h_dn6 = assign53460_e87856_d_n6;
        locals.var_phib_h_dn7 = assign53460_e87856_d_n7;
        locals.var_phib_h_dn8 = assign53460_e87856_d_n8;
        locals.var_phib_h_dn9 = assign53460_e87856_d_n9;
        locals.var_phib_h_dn10 = assign53460_e87856_d_n10;
        locals.var_phib_h_dn11 = assign53460_e87856_d_n11;
        locals.var_phib_h_rv = 0.0;

        let (assign53470_e87865, assign53470_e87865_d_n3, assign53470_e87865_d_n4, assign53470_e87865_d_n5, assign53470_e87865_d_n6, assign53470_e87865_d_n7, assign53470_e87865_d_n8, assign53470_e87865_d_n9, assign53470_e87865_d_n10, assign53470_e87865_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53470_e87863: f64 = 1.0;
        (assign53470_e87863, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign53470_e87865;
        locals.var_t1_dn3 = assign53470_e87865_d_n3;
        locals.var_t1_dn4 = assign53470_e87865_d_n4;
        locals.var_t1_dn5 = assign53470_e87865_d_n5;
        locals.var_t1_dn6 = assign53470_e87865_d_n6;
        locals.var_t1_dn7 = assign53470_e87865_d_n7;
        locals.var_t1_dn8 = assign53470_e87865_d_n8;
        locals.var_t1_dn9 = assign53470_e87865_d_n9;
        locals.var_t1_dn10 = assign53470_e87865_d_n10;
        locals.var_t1_dn11 = assign53470_e87865_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign53480_e87874, assign53480_e87874_d_n3, assign53480_e87874_d_n4, assign53480_e87874_d_n5, assign53480_e87874_d_n6, assign53480_e87874_d_n7, assign53480_e87874_d_n8, assign53480_e87874_d_n9, assign53480_e87874_d_n10, assign53480_e87874_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53480_e87872: f64 = (locals.var_vgfbh / locals.var_t1);
        (assign53480_e87872, (((locals.var_vgfbh_dn3 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn4 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn5 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn6 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn7 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn8 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn9 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn10 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn11 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_vgfbpd, locals.var_vgfbpd_dn3, locals.var_vgfbpd_dn4, locals.var_vgfbpd_dn5, locals.var_vgfbpd_dn6, locals.var_vgfbpd_dn7, locals.var_vgfbpd_dn8, locals.var_vgfbpd_dn9, locals.var_vgfbpd_dn10, locals.var_vgfbpd_dn11,)
    }
};
        locals.var_vgfbpd = assign53480_e87874;
        locals.var_vgfbpd_dn3 = assign53480_e87874_d_n3;
        locals.var_vgfbpd_dn4 = assign53480_e87874_d_n4;
        locals.var_vgfbpd_dn5 = assign53480_e87874_d_n5;
        locals.var_vgfbpd_dn6 = assign53480_e87874_d_n6;
        locals.var_vgfbpd_dn7 = assign53480_e87874_d_n7;
        locals.var_vgfbpd_dn8 = assign53480_e87874_d_n8;
        locals.var_vgfbpd_dn9 = assign53480_e87874_d_n9;
        locals.var_vgfbpd_dn10 = assign53480_e87874_d_n10;
        locals.var_vgfbpd_dn11 = assign53480_e87874_d_n11;
        locals.var_vgfbpd_rv = 0.0;

        let (assign53490_e87883, assign53490_e87883_d_n3, assign53490_e87883_d_n4, assign53490_e87883_d_n5, assign53490_e87883_d_n6, assign53490_e87883_d_n7, assign53490_e87883_d_n8, assign53490_e87883_d_n9, assign53490_e87883_d_n10, assign53490_e87883_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53490_e87881: f64 = (locals.var_gam_h / locals.var_t1);
        (assign53490_e87881, (-((locals.var_gam_h * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (((locals.var_gam_h_dn4 * locals.var_t1) - (locals.var_gam_h * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_h_dn5 * locals.var_t1) - (locals.var_gam_h * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (-((locals.var_gam_h * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_gammapd, locals.var_gammapd_dn3, locals.var_gammapd_dn4, locals.var_gammapd_dn5, locals.var_gammapd_dn6, locals.var_gammapd_dn7, locals.var_gammapd_dn8, locals.var_gammapd_dn9, locals.var_gammapd_dn10, locals.var_gammapd_dn11,)
    }
};
        locals.var_gammapd = assign53490_e87883;
        locals.var_gammapd_dn3 = assign53490_e87883_d_n3;
        locals.var_gammapd_dn4 = assign53490_e87883_d_n4;
        locals.var_gammapd_dn5 = assign53490_e87883_d_n5;
        locals.var_gammapd_dn6 = assign53490_e87883_d_n6;
        locals.var_gammapd_dn7 = assign53490_e87883_d_n7;
        locals.var_gammapd_dn8 = assign53490_e87883_d_n8;
        locals.var_gammapd_dn9 = assign53490_e87883_d_n9;
        locals.var_gammapd_dn10 = assign53490_e87883_d_n10;
        locals.var_gammapd_dn11 = assign53490_e87883_d_n11;
        locals.var_gammapd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_184(
        locals: &mut StampLocals,
    ) {
        let (assign53500_e87900, assign53500_e87900_d_n3, assign53500_e87900_d_n4, assign53500_e87900_d_n5, assign53500_e87900_d_n6, assign53500_e87900_d_n7, assign53500_e87900_d_n8, assign53500_e87900_d_n9, assign53500_e87900_d_n10, assign53500_e87900_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53500_e87890: f64 = (0.5 * locals.var_vgfbpd);
        let assign53500_e87895: f64 = (locals.var_gammapd / 1.4142135623730951);
        let assign53500_e87896: f64 = (1.0 + assign53500_e87895);
        let assign53500_e87897: f64 = (3.0 * assign53500_e87896);
        let assign53500_e87898: f64 = (assign53500_e87890 - assign53500_e87897);
        (assign53500_e87898, ((0.5 * locals.var_vgfbpd_dn3) - (3.0 * (locals.var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn4) - (3.0 * (locals.var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn5) - (3.0 * (locals.var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn6) - (3.0 * (locals.var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn7) - (3.0 * (locals.var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn8) - (3.0 * (locals.var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn9) - (3.0 * (locals.var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn10) - (3.0 * (locals.var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn11) - (3.0 * (locals.var_gammapd_dn11 / 1.4142135623730951))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign53500_e87900;
        locals.var_t1_dn3 = assign53500_e87900_d_n3;
        locals.var_t1_dn4 = assign53500_e87900_d_n4;
        locals.var_t1_dn5 = assign53500_e87900_d_n5;
        locals.var_t1_dn6 = assign53500_e87900_d_n6;
        locals.var_t1_dn7 = assign53500_e87900_d_n7;
        locals.var_t1_dn8 = assign53500_e87900_d_n8;
        locals.var_t1_dn9 = assign53500_e87900_d_n9;
        locals.var_t1_dn10 = assign53500_e87900_d_n10;
        locals.var_t1_dn11 = assign53500_e87900_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign53510_e87916, assign53510_e87916_d_n3, assign53510_e87916_d_n4, assign53510_e87916_d_n5, assign53510_e87916_d_n6, assign53510_e87916_d_n7, assign53510_e87916_d_n8, assign53510_e87916_d_n9, assign53510_e87916_d_n10, assign53510_e87916_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53510_e87908: f64 = (locals.var_t1 * locals.var_t1);
        let assign53510_e87911: f64 = (6.0 * locals.var_vgfbpd);
        let assign53510_e87912: f64 = (assign53510_e87908 + assign53510_e87911);
        let assign53510_e87913: f64 = (assign53510_e87912).sqrt();
        let assign53510_e87914: f64 = (locals.var_t1 + assign53510_e87913);
        (assign53510_e87914, (locals.var_t1_dn3 + ((((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) + (6.0 * locals.var_vgfbpd_dn3)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn4 + ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (6.0 * locals.var_vgfbpd_dn4)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn5 + ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (6.0 * locals.var_vgfbpd_dn5)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn6 + ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (6.0 * locals.var_vgfbpd_dn6)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn7 + ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (6.0 * locals.var_vgfbpd_dn7)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn8 + ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (6.0 * locals.var_vgfbpd_dn8)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn9 + ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (6.0 * locals.var_vgfbpd_dn9)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn10 + ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (6.0 * locals.var_vgfbpd_dn10)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn11 + ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (6.0 * locals.var_vgfbpd_dn11)) / (2.0 * assign53510_e87913))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53510_e87916;
        locals.var_t2_dn3 = assign53510_e87916_d_n3;
        locals.var_t2_dn4 = assign53510_e87916_d_n4;
        locals.var_t2_dn5 = assign53510_e87916_d_n5;
        locals.var_t2_dn6 = assign53510_e87916_d_n6;
        locals.var_t2_dn7 = assign53510_e87916_d_n7;
        locals.var_t2_dn8 = assign53510_e87916_d_n8;
        locals.var_t2_dn9 = assign53510_e87916_d_n9;
        locals.var_t2_dn10 = assign53510_e87916_d_n10;
        locals.var_t2_dn11 = assign53510_e87916_d_n11;
        locals.var_t2_rv = 0.0;

        let assign53520_e87919: f64 = if locals.var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard825 = assign53520_e87919;
        locals.var_guard825_rv = 0.0;

        let (assign53530_e87932, assign53530_e87932_d_n3, assign53530_e87932_d_n4, assign53530_e87932_d_n5, assign53530_e87932_d_n6, assign53530_e87932_d_n7, assign53530_e87932_d_n8, assign53530_e87932_d_n9, assign53530_e87932_d_n10, assign53530_e87932_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign53530_e87928: f64 = (locals.var_vgfbpd - locals.var_t2);
        let assign53530_e87930: f64 = (assign53530_e87928 / locals.var_gammapd);
        (assign53530_e87930, ((((locals.var_vgfbpd_dn3 - locals.var_t2_dn3) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn3)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn4 - locals.var_t2_dn4) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn4)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn5 - locals.var_t2_dn5) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn5)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn6 - locals.var_t2_dn6) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn6)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn7 - locals.var_t2_dn7) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn7)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn8 - locals.var_t2_dn8) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn8)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn9 - locals.var_t2_dn9) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn9)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn10 - locals.var_t2_dn10) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn10)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn11 - locals.var_t2_dn11) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn11)) / (locals.var_gammapd * locals.var_gammapd)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53530_e87932;
        locals.var_t3_dn3 = assign53530_e87932_d_n3;
        locals.var_t3_dn4 = assign53530_e87932_d_n4;
        locals.var_t3_dn5 = assign53530_e87932_d_n5;
        locals.var_t3_dn6 = assign53530_e87932_d_n6;
        locals.var_t3_dn7 = assign53530_e87932_d_n7;
        locals.var_t3_dn8 = assign53530_e87932_d_n8;
        locals.var_t3_dn9 = assign53530_e87932_d_n9;
        locals.var_t3_dn10 = assign53530_e87932_d_n10;
        locals.var_t3_dn11 = assign53530_e87932_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign53540_e87951, assign53540_e87951_d_n3, assign53540_e87951_d_n4, assign53540_e87951_d_n5, assign53540_e87951_d_n6, assign53540_e87951_d_n7, assign53540_e87951_d_n8, assign53540_e87951_d_n9, assign53540_e87951_d_n10, assign53540_e87951_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign53540_e87941: f64 = (1.0 - locals.var_t2);
        let assign53540_e87944: f64 = (locals.var_t3 * locals.var_t3);
        let assign53540_e87945: f64 = (assign53540_e87941 + assign53540_e87944);
        let assign53540_e87947: f64 = (assign53540_e87945).max(1e-38);
        let assign53540_e87948: f64 = (assign53540_e87947).ln();
        let assign53540_e87949: f64 = (-assign53540_e87948);
        (assign53540_e87949, (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn3) + ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn4) + ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn5) + ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn6) + ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn7) + ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn8) + ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn9) + ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn10) + ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn11) + ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11))) } else { 0.0 } / assign53540_e87947)),)
    } else {
        (locals.var_psiph, locals.var_psiph_dn3, locals.var_psiph_dn4, locals.var_psiph_dn5, locals.var_psiph_dn6, locals.var_psiph_dn7, locals.var_psiph_dn8, locals.var_psiph_dn9, locals.var_psiph_dn10, locals.var_psiph_dn11,)
    }
};
        locals.var_psiph = assign53540_e87951;
        locals.var_psiph_dn3 = assign53540_e87951_d_n3;
        locals.var_psiph_dn4 = assign53540_e87951_d_n4;
        locals.var_psiph_dn5 = assign53540_e87951_d_n5;
        locals.var_psiph_dn6 = assign53540_e87951_d_n6;
        locals.var_psiph_dn7 = assign53540_e87951_d_n7;
        locals.var_psiph_dn8 = assign53540_e87951_d_n8;
        locals.var_psiph_dn9 = assign53540_e87951_d_n9;
        locals.var_psiph_dn10 = assign53540_e87951_d_n10;
        locals.var_psiph_dn11 = assign53540_e87951_d_n11;
        locals.var_psiph_rv = 0.0;

        let (assign53550_e87963, assign53550_e87963_d_n3, assign53550_e87963_d_n4, assign53550_e87963_d_n5, assign53550_e87963_d_n6, assign53550_e87963_d_n7, assign53550_e87963_d_n8, assign53550_e87963_d_n9, assign53550_e87963_d_n10, assign53550_e87963_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 == 0.0)) {
        let assign53550_e87960: f64 = (-locals.var_t2);
        let assign53550_e87961: f64 = { let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53550_e87961, ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53550_e87963;
        locals.var_t3_dn3 = assign53550_e87963_d_n3;
        locals.var_t3_dn4 = assign53550_e87963_d_n4;
        locals.var_t3_dn5 = assign53550_e87963_d_n5;
        locals.var_t3_dn6 = assign53550_e87963_d_n6;
        locals.var_t3_dn7 = assign53550_e87963_d_n7;
        locals.var_t3_dn8 = assign53550_e87963_d_n8;
        locals.var_t3_dn9 = assign53550_e87963_d_n9;
        locals.var_t3_dn10 = assign53550_e87963_d_n10;
        locals.var_t3_dn11 = assign53550_e87963_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign53560_e87975, assign53560_e87975_d_n3, assign53560_e87975_d_n4, assign53560_e87975_d_n5, assign53560_e87975_d_n6, assign53560_e87975_d_n7, assign53560_e87975_d_n8, assign53560_e87975_d_n9, assign53560_e87975_d_n10, assign53560_e87975_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 == 0.0)) {
        let assign53560_e87973: f64 = (0.5 * locals.var_gammapd);
        (assign53560_e87973, (0.5 * locals.var_gammapd_dn3), (0.5 * locals.var_gammapd_dn4), (0.5 * locals.var_gammapd_dn5), (0.5 * locals.var_gammapd_dn6), (0.5 * locals.var_gammapd_dn7), (0.5 * locals.var_gammapd_dn8), (0.5 * locals.var_gammapd_dn9), (0.5 * locals.var_gammapd_dn10), (0.5 * locals.var_gammapd_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign53560_e87975;
        locals.var_t1_dn3 = assign53560_e87975_d_n3;
        locals.var_t1_dn4 = assign53560_e87975_d_n4;
        locals.var_t1_dn5 = assign53560_e87975_d_n5;
        locals.var_t1_dn6 = assign53560_e87975_d_n6;
        locals.var_t1_dn7 = assign53560_e87975_d_n7;
        locals.var_t1_dn8 = assign53560_e87975_d_n8;
        locals.var_t1_dn9 = assign53560_e87975_d_n9;
        locals.var_t1_dn10 = assign53560_e87975_d_n10;
        locals.var_t1_dn11 = assign53560_e87975_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign53570_e87996, assign53570_e87996_d_n3, assign53570_e87996_d_n4, assign53570_e87996_d_n5, assign53570_e87996_d_n6, assign53570_e87996_d_n7, assign53570_e87996_d_n8, assign53570_e87996_d_n9, assign53570_e87996_d_n10, assign53570_e87996_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 == 0.0)) {
        let assign53570_e87985: f64 = (locals.var_vgfbpd - 1.0);
        let assign53570_e87987: f64 = (assign53570_e87985 + locals.var_t3);
        let assign53570_e87990: f64 = (locals.var_t1 * locals.var_t1);
        let assign53570_e87991: f64 = (assign53570_e87987 + assign53570_e87990);
        let assign53570_e87992: f64 = (assign53570_e87991).sqrt();
        let assign53570_e87994: f64 = (assign53570_e87992 - locals.var_t1);
        (assign53570_e87994, ((((locals.var_vgfbpd_dn3 + locals.var_t3_dn3) + ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn3), ((((locals.var_vgfbpd_dn4 + locals.var_t3_dn4) + ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn4), ((((locals.var_vgfbpd_dn5 + locals.var_t3_dn5) + ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn5), ((((locals.var_vgfbpd_dn6 + locals.var_t3_dn6) + ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn6), ((((locals.var_vgfbpd_dn7 + locals.var_t3_dn7) + ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn7), ((((locals.var_vgfbpd_dn8 + locals.var_t3_dn8) + ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn8), ((((locals.var_vgfbpd_dn9 + locals.var_t3_dn9) + ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn9), ((((locals.var_vgfbpd_dn10 + locals.var_t3_dn10) + ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn10), ((((locals.var_vgfbpd_dn11 + locals.var_t3_dn11) + ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53570_e87996;
        locals.var_t2_dn3 = assign53570_e87996_d_n3;
        locals.var_t2_dn4 = assign53570_e87996_d_n4;
        locals.var_t2_dn5 = assign53570_e87996_d_n5;
        locals.var_t2_dn6 = assign53570_e87996_d_n6;
        locals.var_t2_dn7 = assign53570_e87996_d_n7;
        locals.var_t2_dn8 = assign53570_e87996_d_n8;
        locals.var_t2_dn9 = assign53570_e87996_d_n9;
        locals.var_t2_dn10 = assign53570_e87996_d_n10;
        locals.var_t2_dn11 = assign53570_e87996_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign53580_e88012, assign53580_e88012_d_n3, assign53580_e88012_d_n4, assign53580_e88012_d_n5, assign53580_e88012_d_n6, assign53580_e88012_d_n7, assign53580_e88012_d_n8, assign53580_e88012_d_n9, assign53580_e88012_d_n10, assign53580_e88012_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 == 0.0)) {
        let assign53580_e88006: f64 = (locals.var_t2 * locals.var_t2);
        let assign53580_e88008: f64 = (assign53580_e88006 + 1.0);
        let assign53580_e88010: f64 = (assign53580_e88008 - locals.var_t3);
        (assign53580_e88010, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) - locals.var_t3_dn3), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) - locals.var_t3_dn4), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) - locals.var_t3_dn5), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) - locals.var_t3_dn6), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) - locals.var_t3_dn7), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) - locals.var_t3_dn8), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) - locals.var_t3_dn9), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) - locals.var_t3_dn10), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) - locals.var_t3_dn11),)
    } else {
        (locals.var_psiph, locals.var_psiph_dn3, locals.var_psiph_dn4, locals.var_psiph_dn5, locals.var_psiph_dn6, locals.var_psiph_dn7, locals.var_psiph_dn8, locals.var_psiph_dn9, locals.var_psiph_dn10, locals.var_psiph_dn11,)
    }
};
        locals.var_psiph = assign53580_e88012;
        locals.var_psiph_dn3 = assign53580_e88012_d_n3;
        locals.var_psiph_dn4 = assign53580_e88012_d_n4;
        locals.var_psiph_dn5 = assign53580_e88012_d_n5;
        locals.var_psiph_dn6 = assign53580_e88012_d_n6;
        locals.var_psiph_dn7 = assign53580_e88012_d_n7;
        locals.var_psiph_dn8 = assign53580_e88012_d_n8;
        locals.var_psiph_dn9 = assign53580_e88012_d_n9;
        locals.var_psiph_dn10 = assign53580_e88012_d_n10;
        locals.var_psiph_dn11 = assign53580_e88012_d_n11;
        locals.var_psiph_rv = 0.0;

        let (assign53590_e88038, assign53590_e88038_d_n3, assign53590_e88038_d_n4, assign53590_e88038_d_n5, assign53590_e88038_d_n6, assign53590_e88038_d_n7, assign53590_e88038_d_n8, assign53590_e88038_d_n9, assign53590_e88038_d_n10, assign53590_e88038_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53590_e88020: f64 = (locals.var_psiph + 1.0);
        let assign53590_e88023: f64 = (locals.var_psiph - 1.0);
        let assign53590_e88026: f64 = (locals.var_psiph - 1.0);
        let assign53590_e88027: f64 = (assign53590_e88023 * assign53590_e88026);
        let assign53590_e88030: f64 = (0.25 * 2.0);
        let assign53590_e88032: f64 = (assign53590_e88030 * 2.0);
        let assign53590_e88033: f64 = (assign53590_e88027 + assign53590_e88032);
        let assign53590_e88034: f64 = (assign53590_e88033).sqrt();
        let assign53590_e88035: f64 = (assign53590_e88020 + assign53590_e88034);
        let assign53590_e88036: f64 = (0.5 * assign53590_e88035);
        (assign53590_e88036, (0.5 * (locals.var_psiph_dn3 + (((locals.var_psiph_dn3 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn3)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn4 + (((locals.var_psiph_dn4 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn4)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn5 + (((locals.var_psiph_dn5 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn5)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn6 + (((locals.var_psiph_dn6 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn6)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn7 + (((locals.var_psiph_dn7 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn7)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn8 + (((locals.var_psiph_dn8 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn8)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn9 + (((locals.var_psiph_dn9 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn9)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn10 + (((locals.var_psiph_dn10 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn10)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn11 + (((locals.var_psiph_dn11 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn11)) / (2.0 * assign53590_e88034)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign53590_e88038;
        locals.var_t8_dn3 = assign53590_e88038_d_n3;
        locals.var_t8_dn4 = assign53590_e88038_d_n4;
        locals.var_t8_dn5 = assign53590_e88038_d_n5;
        locals.var_t8_dn6 = assign53590_e88038_d_n6;
        locals.var_t8_dn7 = assign53590_e88038_d_n7;
        locals.var_t8_dn8 = assign53590_e88038_d_n8;
        locals.var_t8_dn9 = assign53590_e88038_d_n9;
        locals.var_t8_dn10 = assign53590_e88038_d_n10;
        locals.var_t8_dn11 = assign53590_e88038_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign53600_e88046, assign53600_e88046_d_n3, assign53600_e88046_d_n4, assign53600_e88046_d_n5, assign53600_e88046_d_n6, assign53600_e88046_d_n7, assign53600_e88046_d_n8, assign53600_e88046_d_n9, assign53600_e88046_d_n10, assign53600_e88046_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53600_e88044: f64 = (locals.var_t8).sqrt();
        (assign53600_e88044, (locals.var_t8_dn3 / (2.0 * assign53600_e88044)), (locals.var_t8_dn4 / (2.0 * assign53600_e88044)), (locals.var_t8_dn5 / (2.0 * assign53600_e88044)), (locals.var_t8_dn6 / (2.0 * assign53600_e88044)), (locals.var_t8_dn7 / (2.0 * assign53600_e88044)), (locals.var_t8_dn8 / (2.0 * assign53600_e88044)), (locals.var_t8_dn9 / (2.0 * assign53600_e88044)), (locals.var_t8_dn10 / (2.0 * assign53600_e88044)), (locals.var_t8_dn11 / (2.0 * assign53600_e88044)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign53600_e88046;
        locals.var_sqrtpsip_dn3 = assign53600_e88046_d_n3;
        locals.var_sqrtpsip_dn4 = assign53600_e88046_d_n4;
        locals.var_sqrtpsip_dn5 = assign53600_e88046_d_n5;
        locals.var_sqrtpsip_dn6 = assign53600_e88046_d_n6;
        locals.var_sqrtpsip_dn7 = assign53600_e88046_d_n7;
        locals.var_sqrtpsip_dn8 = assign53600_e88046_d_n8;
        locals.var_sqrtpsip_dn9 = assign53600_e88046_d_n9;
        locals.var_sqrtpsip_dn10 = assign53600_e88046_d_n10;
        locals.var_sqrtpsip_dn11 = assign53600_e88046_d_n11;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign53610_e88061, assign53610_e88061_d_n3, assign53610_e88061_d_n4, assign53610_e88061_d_n5, assign53610_e88061_d_n6, assign53610_e88061_d_n7, assign53610_e88061_d_n8, assign53610_e88061_d_n9, assign53610_e88061_d_n10, assign53610_e88061_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53610_e88055: f64 = (2.0 * locals.var_sqrtpsip);
        let assign53610_e88056: f64 = (locals.var_gam_h / assign53610_e88055);
        let assign53610_e88057: f64 = (1.0 + assign53610_e88056);
        let assign53610_e88059: f64 = (assign53610_e88057 / locals.var_gam_h);
        (assign53610_e88059, ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn3)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((((((locals.var_gam_h_dn4 * assign53610_e88055) - (locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn4))) / (assign53610_e88055 * assign53610_e88055)) * locals.var_gam_h) - (assign53610_e88057 * locals.var_gam_h_dn4)) / (locals.var_gam_h * locals.var_gam_h)), ((((((locals.var_gam_h_dn5 * assign53610_e88055) - (locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn5))) / (assign53610_e88055 * assign53610_e88055)) * locals.var_gam_h) - (assign53610_e88057 * locals.var_gam_h_dn5)) / (locals.var_gam_h * locals.var_gam_h)), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn6)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn7)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn8)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn9)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn10)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn11)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign53610_e88061;
        locals.var_t0_dn3 = assign53610_e88061_d_n3;
        locals.var_t0_dn4 = assign53610_e88061_d_n4;
        locals.var_t0_dn5 = assign53610_e88061_d_n5;
        locals.var_t0_dn6 = assign53610_e88061_d_n6;
        locals.var_t0_dn7 = assign53610_e88061_d_n7;
        locals.var_t0_dn8 = assign53610_e88061_d_n8;
        locals.var_t0_dn9 = assign53610_e88061_d_n9;
        locals.var_t0_dn10 = assign53610_e88061_d_n10;
        locals.var_t0_dn11 = assign53610_e88061_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign53620_e88074, assign53620_e88074_d_n3, assign53620_e88074_d_n4, assign53620_e88074_d_n5, assign53620_e88074_d_n6, assign53620_e88074_d_n7, assign53620_e88074_d_n8, assign53620_e88074_d_n9, assign53620_e88074_d_n10, assign53620_e88074_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53620_e88069: f64 = (2.0 * locals.var_phib_h);
        let assign53620_e88070: f64 = (locals.var_psiph - assign53620_e88069);
        let assign53620_e88072: f64 = (assign53620_e88070 - locals.var_vs_1);
        (assign53620_e88072, ((locals.var_psiph_dn3 - (2.0 * locals.var_phib_h_dn3)) - locals.var_vs_1_dn3), ((locals.var_psiph_dn4 - (2.0 * locals.var_phib_h_dn4)) - locals.var_vs_1_dn4), ((locals.var_psiph_dn5 - (2.0 * locals.var_phib_h_dn5)) - locals.var_vs_1_dn5), ((locals.var_psiph_dn6 - (2.0 * locals.var_phib_h_dn6)) - locals.var_vs_1_dn6), ((locals.var_psiph_dn7 - (2.0 * locals.var_phib_h_dn7)) - locals.var_vs_1_dn7), ((locals.var_psiph_dn8 - (2.0 * locals.var_phib_h_dn8)) - locals.var_vs_1_dn8), ((locals.var_psiph_dn9 - (2.0 * locals.var_phib_h_dn9)) - locals.var_vs_1_dn9), ((locals.var_psiph_dn10 - (2.0 * locals.var_phib_h_dn10)) - locals.var_vs_1_dn10), ((locals.var_psiph_dn11 - (2.0 * locals.var_phib_h_dn11)) - locals.var_vs_1_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign53620_e88074;
        locals.var_t1_dn3 = assign53620_e88074_d_n3;
        locals.var_t1_dn4 = assign53620_e88074_d_n4;
        locals.var_t1_dn5 = assign53620_e88074_d_n5;
        locals.var_t1_dn6 = assign53620_e88074_d_n6;
        locals.var_t1_dn7 = assign53620_e88074_d_n7;
        locals.var_t1_dn8 = assign53620_e88074_d_n8;
        locals.var_t1_dn9 = assign53620_e88074_d_n9;
        locals.var_t1_dn10 = assign53620_e88074_d_n10;
        locals.var_t1_dn11 = assign53620_e88074_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign53630_e88090, assign53630_e88090_d_n3, assign53630_e88090_d_n4, assign53630_e88090_d_n5, assign53630_e88090_d_n6, assign53630_e88090_d_n7, assign53630_e88090_d_n8, assign53630_e88090_d_n9, assign53630_e88090_d_n10, assign53630_e88090_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53630_e88082: f64 = (4.0 * locals.var_t0);
        let assign53630_e88084: f64 = (assign53630_e88082 * locals.var_sqrtpsip);
        let assign53630_e88086: f64 = (assign53630_e88084).max(1e-38);
        let assign53630_e88087: f64 = (assign53630_e88086).ln();
        let assign53630_e88088: f64 = (locals.var_t1 - assign53630_e88087);
        (assign53630_e88088, (locals.var_t1_dn3 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn4 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn5 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn6 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn7 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn8 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn9 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn10 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn11 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign53630_e88086)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53630_e88090;
        locals.var_t2_dn3 = assign53630_e88090_d_n3;
        locals.var_t2_dn4 = assign53630_e88090_d_n4;
        locals.var_t2_dn5 = assign53630_e88090_d_n5;
        locals.var_t2_dn6 = assign53630_e88090_d_n6;
        locals.var_t2_dn7 = assign53630_e88090_d_n7;
        locals.var_t2_dn8 = assign53630_e88090_d_n8;
        locals.var_t2_dn9 = assign53630_e88090_d_n9;
        locals.var_t2_dn10 = assign53630_e88090_d_n10;
        locals.var_t2_dn11 = assign53630_e88090_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign53640_e88110, assign53640_e88110_d_n3, assign53640_e88110_d_n4, assign53640_e88110_d_n5, assign53640_e88110_d_n6, assign53640_e88110_d_n7, assign53640_e88110_d_n8, assign53640_e88110_d_n9, assign53640_e88110_d_n10, assign53640_e88110_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53640_e88098: f64 = (locals.var_t2 - 0.201491);
        let assign53640_e88102: f64 = (locals.var_t2 + 0.402982);
        let assign53640_e88103: f64 = (locals.var_t2 * assign53640_e88102);
        let assign53640_e88105: f64 = (assign53640_e88103 + 2.446562);
        let assign53640_e88106: f64 = (assign53640_e88105).sqrt();
        let assign53640_e88107: f64 = (assign53640_e88098 - assign53640_e88106);
        let assign53640_e88108: f64 = (0.5 * assign53640_e88107);
        (assign53640_e88108, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign53640_e88106)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign53640_e88110;
        locals.var_t8_dn3 = assign53640_e88110_d_n3;
        locals.var_t8_dn4 = assign53640_e88110_d_n4;
        locals.var_t8_dn5 = assign53640_e88110_d_n5;
        locals.var_t8_dn6 = assign53640_e88110_d_n6;
        locals.var_t8_dn7 = assign53640_e88110_d_n7;
        locals.var_t8_dn8 = assign53640_e88110_d_n8;
        locals.var_t8_dn9 = assign53640_e88110_d_n9;
        locals.var_t8_dn10 = assign53640_e88110_d_n10;
        locals.var_t8_dn11 = assign53640_e88110_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign53650_e88117, assign53650_e88117_d_n3, assign53650_e88117_d_n4, assign53650_e88117_d_n5, assign53650_e88117_d_n6, assign53650_e88117_d_n7, assign53650_e88117_d_n8, assign53650_e88117_d_n9, assign53650_e88117_d_n10, assign53650_e88117_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign53650_e88117;
        locals.var_sqrtpsisa_dn3 = assign53650_e88117_d_n3;
        locals.var_sqrtpsisa_dn4 = assign53650_e88117_d_n4;
        locals.var_sqrtpsisa_dn5 = assign53650_e88117_d_n5;
        locals.var_sqrtpsisa_dn6 = assign53650_e88117_d_n6;
        locals.var_sqrtpsisa_dn7 = assign53650_e88117_d_n7;
        locals.var_sqrtpsisa_dn8 = assign53650_e88117_d_n8;
        locals.var_sqrtpsisa_dn9 = assign53650_e88117_d_n9;
        locals.var_sqrtpsisa_dn10 = assign53650_e88117_d_n10;
        locals.var_sqrtpsisa_dn11 = assign53650_e88117_d_n11;
        locals.var_sqrtpsisa_rv = 0.0;

        let assign53660_e88120: f64 = (-68.0);
        let assign53660_e88121: f64 = if locals.var_t8 <= assign53660_e88120 { 1.0 } else { 0.0 };
        locals.var_guard826 = assign53660_e88121;
        locals.var_guard826_rv = 0.0;

        let (assign53670_e88131, assign53670_e88131_d_n3, assign53670_e88131_d_n4, assign53670_e88131_d_n5, assign53670_e88131_d_n6, assign53670_e88131_d_n7, assign53670_e88131_d_n8, assign53670_e88131_d_n9, assign53670_e88131_d_n10, assign53670_e88131_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign53670_e88129: f64 = (-100.0);
        (assign53670_e88129, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign53670_e88131;
        locals.var_t4_dn3 = assign53670_e88131_d_n3;
        locals.var_t4_dn4 = assign53670_e88131_d_n4;
        locals.var_t4_dn5 = assign53670_e88131_d_n5;
        locals.var_t4_dn6 = assign53670_e88131_d_n6;
        locals.var_t4_dn7 = assign53670_e88131_d_n7;
        locals.var_t4_dn8 = assign53670_e88131_d_n8;
        locals.var_t4_dn9 = assign53670_e88131_d_n9;
        locals.var_t4_dn10 = assign53670_e88131_d_n10;
        locals.var_t4_dn11 = assign53670_e88131_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign53680_e88140, assign53680_e88140_d_n3, assign53680_e88140_d_n4, assign53680_e88140_d_n5, assign53680_e88140_d_n6, assign53680_e88140_d_n7, assign53680_e88140_d_n8, assign53680_e88140_d_n9, assign53680_e88140_d_n10, assign53680_e88140_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign53680_e88140;
        locals.var_t5_dn3 = assign53680_e88140_d_n3;
        locals.var_t5_dn4 = assign53680_e88140_d_n4;
        locals.var_t5_dn5 = assign53680_e88140_d_n5;
        locals.var_t5_dn6 = assign53680_e88140_d_n6;
        locals.var_t5_dn7 = assign53680_e88140_d_n7;
        locals.var_t5_dn8 = assign53680_e88140_d_n8;
        locals.var_t5_dn9 = assign53680_e88140_d_n9;
        locals.var_t5_dn10 = assign53680_e88140_d_n10;
        locals.var_t5_dn11 = assign53680_e88140_d_n11;
        locals.var_t5_rv = 0.0;

        let assign53690_e88145: f64 = (0.5 * locals.var_t5);
        let assign53690_e88146: f64 = (locals.var_t4 - assign53690_e88145);
        let assign53690_e88147: f64 = if locals.var_t8 < assign53690_e88146 { 1.0 } else { 0.0 };
        locals.var_guard827 = assign53690_e88147;
        locals.var_guard827_rv = 0.0;

        let (assign53700_e88159, assign53700_e88159_d_n3, assign53700_e88159_d_n4, assign53700_e88159_d_n5, assign53700_e88159_d_n6, assign53700_e88159_d_n7, assign53700_e88159_d_n8, assign53700_e88159_d_n9, assign53700_e88159_d_n10, assign53700_e88159_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) && (locals.var_guard827 != 0.0)) {
        let assign53700_e88157: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53700_e88157, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53700_e88159;
        locals.var_t3_dn3 = assign53700_e88159_d_n3;
        locals.var_t3_dn4 = assign53700_e88159_d_n4;
        locals.var_t3_dn5 = assign53700_e88159_d_n5;
        locals.var_t3_dn6 = assign53700_e88159_d_n6;
        locals.var_t3_dn7 = assign53700_e88159_d_n7;
        locals.var_t3_dn8 = assign53700_e88159_d_n8;
        locals.var_t3_dn9 = assign53700_e88159_d_n9;
        locals.var_t3_dn10 = assign53700_e88159_d_n10;
        locals.var_t3_dn11 = assign53700_e88159_d_n11;
        locals.var_t3_rv = 0.0;

        let assign53710_e88164: f64 = (0.5 * locals.var_t5);
        let assign53710_e88165: f64 = (locals.var_t4 + assign53710_e88164);
        let assign53710_e88166: f64 = if locals.var_t8 > assign53710_e88165 { 1.0 } else { 0.0 };
        locals.var_guard828 = assign53710_e88166;
        locals.var_guard828_rv = 0.0;

        let (assign53720_e88181, assign53720_e88181_d_n3, assign53720_e88181_d_n4, assign53720_e88181_d_n5, assign53720_e88181_d_n6, assign53720_e88181_d_n7, assign53720_e88181_d_n8, assign53720_e88181_d_n9, assign53720_e88181_d_n10, assign53720_e88181_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) && (locals.var_guard827 == 0.0)) && (locals.var_guard828 != 0.0)) {
        let assign53720_e88179: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53720_e88179, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53720_e88181;
        locals.var_t3_dn3 = assign53720_e88181_d_n3;
        locals.var_t3_dn4 = assign53720_e88181_d_n4;
        locals.var_t3_dn5 = assign53720_e88181_d_n5;
        locals.var_t3_dn6 = assign53720_e88181_d_n6;
        locals.var_t3_dn7 = assign53720_e88181_d_n7;
        locals.var_t3_dn8 = assign53720_e88181_d_n8;
        locals.var_t3_dn9 = assign53720_e88181_d_n9;
        locals.var_t3_dn10 = assign53720_e88181_d_n10;
        locals.var_t3_dn11 = assign53720_e88181_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign53730_e88200, assign53730_e88200_d_n3, assign53730_e88200_d_n4, assign53730_e88200_d_n5, assign53730_e88200_d_n6, assign53730_e88200_d_n7, assign53730_e88200_d_n8, assign53730_e88200_d_n9, assign53730_e88200_d_n10, assign53730_e88200_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) && (locals.var_guard827 == 0.0)) && (locals.var_guard828 == 0.0)) {
        let assign53730_e88196: f64 = (locals.var_t8 - locals.var_t4);
        let assign53730_e88198: f64 = (assign53730_e88196 / locals.var_t5);
        (assign53730_e88198, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53730_e88200;
        locals.var_t2_dn3 = assign53730_e88200_d_n3;
        locals.var_t2_dn4 = assign53730_e88200_d_n4;
        locals.var_t2_dn5 = assign53730_e88200_d_n5;
        locals.var_t2_dn6 = assign53730_e88200_d_n6;
        locals.var_t2_dn7 = assign53730_e88200_d_n7;
        locals.var_t2_dn8 = assign53730_e88200_d_n8;
        locals.var_t2_dn9 = assign53730_e88200_d_n9;
        locals.var_t2_dn10 = assign53730_e88200_d_n10;
        locals.var_t2_dn11 = assign53730_e88200_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign53740_e88217, assign53740_e88217_d_n3, assign53740_e88217_d_n4, assign53740_e88217_d_n5, assign53740_e88217_d_n6, assign53740_e88217_d_n7, assign53740_e88217_d_n8, assign53740_e88217_d_n9, assign53740_e88217_d_n10, assign53740_e88217_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) && (locals.var_guard827 == 0.0)) && (locals.var_guard828 == 0.0)) {
        let assign53740_e88215: f64 = (locals.var_t2 * locals.var_t2);
        (assign53740_e88215, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign53740_e88217;
        locals.var_t6_dn3 = assign53740_e88217_d_n3;
        locals.var_t6_dn4 = assign53740_e88217_d_n4;
        locals.var_t6_dn5 = assign53740_e88217_d_n5;
        locals.var_t6_dn6 = assign53740_e88217_d_n6;
        locals.var_t6_dn7 = assign53740_e88217_d_n7;
        locals.var_t6_dn8 = assign53740_e88217_d_n8;
        locals.var_t6_dn9 = assign53740_e88217_d_n9;
        locals.var_t6_dn10 = assign53740_e88217_d_n10;
        locals.var_t6_dn11 = assign53740_e88217_d_n11;
        locals.var_t6_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_185(
        locals: &mut StampLocals,
    ) {
        let (assign53750_e88255, assign53750_e88255_d_n3, assign53750_e88255_d_n4, assign53750_e88255_d_n5, assign53750_e88255_d_n6, assign53750_e88255_d_n7, assign53750_e88255_d_n8, assign53750_e88255_d_n9, assign53750_e88255_d_n10, assign53750_e88255_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) && (locals.var_guard827 == 0.0)) && (locals.var_guard828 == 0.0)) {
        let assign53750_e88234: f64 = (5.0 / 64.0);
        let assign53750_e88237: f64 = (0.5 * locals.var_t2);
        let assign53750_e88238: f64 = (assign53750_e88234 + assign53750_e88237);
        let assign53750_e88242: f64 = (15.0 / 16.0);
        let assign53750_e88246: f64 = (1.25 - locals.var_t6);
        let assign53750_e88247: f64 = (locals.var_t6 * assign53750_e88246);
        let assign53750_e88248: f64 = (assign53750_e88242 - assign53750_e88247);
        let assign53750_e88249: f64 = (locals.var_t6 * assign53750_e88248);
        let assign53750_e88250: f64 = (assign53750_e88238 + assign53750_e88249);
        let assign53750_e88251: f64 = (locals.var_t5 * assign53750_e88250);
        let assign53750_e88252: f64 = (locals.var_t4 + assign53750_e88251);
        let assign53750_e88253: f64 = { let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53750_e88253, ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53750_e88255;
        locals.var_t3_dn3 = assign53750_e88255_d_n3;
        locals.var_t3_dn4 = assign53750_e88255_d_n4;
        locals.var_t3_dn5 = assign53750_e88255_d_n5;
        locals.var_t3_dn6 = assign53750_e88255_d_n6;
        locals.var_t3_dn7 = assign53750_e88255_d_n7;
        locals.var_t3_dn8 = assign53750_e88255_d_n8;
        locals.var_t3_dn9 = assign53750_e88255_d_n9;
        locals.var_t3_dn10 = assign53750_e88255_d_n10;
        locals.var_t3_dn11 = assign53750_e88255_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign53760_e88287, assign53760_e88287_d_n3, assign53760_e88287_d_n4, assign53760_e88287_d_n5, assign53760_e88287_d_n6, assign53760_e88287_d_n7, assign53760_e88287_d_n8, assign53760_e88287_d_n9, assign53760_e88287_d_n10, assign53760_e88287_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign53760_e88265: f64 = (1.0 + locals.var_t1);
        let assign53760_e88267: f64 = (assign53760_e88265 - locals.var_t8);
        let assign53760_e88270: f64 = (2.0 * locals.var_t0);
        let assign53760_e88273: f64 = (locals.var_t3 * 2.0);
        let assign53760_e88275: f64 = (assign53760_e88273 * locals.var_t0);
        let assign53760_e88278: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign53760_e88279: f64 = (assign53760_e88275 + assign53760_e88278);
        let assign53760_e88280: f64 = (assign53760_e88270 * assign53760_e88279);
        let assign53760_e88282: f64 = (assign53760_e88280).max(1e-38);
        let assign53760_e88283: f64 = (assign53760_e88282).ln();
        let assign53760_e88284: f64 = (assign53760_e88267 - assign53760_e88283);
        let assign53760_e88285: f64 = (locals.var_t3 * assign53760_e88284);
        (assign53760_e88285, ((locals.var_t3_dn3 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn4 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn5 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn6 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn7 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn8 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn9 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn10 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn11 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign53760_e88282)))),)
    } else {
        (locals.var_qsh, locals.var_qsh_dn3, locals.var_qsh_dn4, locals.var_qsh_dn5, locals.var_qsh_dn6, locals.var_qsh_dn7, locals.var_qsh_dn8, locals.var_qsh_dn9, locals.var_qsh_dn10, locals.var_qsh_dn11,)
    }
};
        locals.var_qsh = assign53760_e88287;
        locals.var_qsh_dn3 = assign53760_e88287_d_n3;
        locals.var_qsh_dn4 = assign53760_e88287_d_n4;
        locals.var_qsh_dn5 = assign53760_e88287_d_n5;
        locals.var_qsh_dn6 = assign53760_e88287_d_n6;
        locals.var_qsh_dn7 = assign53760_e88287_d_n7;
        locals.var_qsh_dn8 = assign53760_e88287_d_n8;
        locals.var_qsh_dn9 = assign53760_e88287_d_n9;
        locals.var_qsh_dn10 = assign53760_e88287_d_n10;
        locals.var_qsh_dn11 = assign53760_e88287_d_n11;
        locals.var_qsh_rv = 0.0;

        let (assign53770_e88298, assign53770_e88298_d_n3, assign53770_e88298_d_n4, assign53770_e88298_d_n5, assign53770_e88298_d_n6, assign53770_e88298_d_n7, assign53770_e88298_d_n8, assign53770_e88298_d_n9, assign53770_e88298_d_n10, assign53770_e88298_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53770_e88296: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53770_e88296, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53770_e88298;
        locals.var_t3_dn3 = assign53770_e88298_d_n3;
        locals.var_t3_dn4 = assign53770_e88298_d_n4;
        locals.var_t3_dn5 = assign53770_e88298_d_n5;
        locals.var_t3_dn6 = assign53770_e88298_d_n6;
        locals.var_t3_dn7 = assign53770_e88298_d_n7;
        locals.var_t3_dn8 = assign53770_e88298_d_n8;
        locals.var_t3_dn9 = assign53770_e88298_d_n9;
        locals.var_t3_dn10 = assign53770_e88298_d_n10;
        locals.var_t3_dn11 = assign53770_e88298_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign53780_e88310, assign53780_e88310_d_n3, assign53780_e88310_d_n4, assign53780_e88310_d_n5, assign53780_e88310_d_n6, assign53780_e88310_d_n7, assign53780_e88310_d_n8, assign53780_e88310_d_n9, assign53780_e88310_d_n10, assign53780_e88310_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53780_e88308: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign53780_e88308, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign53780_e88310;
        locals.var_sqrtpsisainv_dn3 = assign53780_e88310_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign53780_e88310_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign53780_e88310_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign53780_e88310_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign53780_e88310_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign53780_e88310_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign53780_e88310_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign53780_e88310_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign53780_e88310_d_n11;
        locals.var_sqrtpsisainv_rv = 0.0;

        let (assign53790_e88343, assign53790_e88343_d_n3, assign53790_e88343_d_n4, assign53790_e88343_d_n5, assign53790_e88343_d_n6, assign53790_e88343_d_n7, assign53790_e88343_d_n8, assign53790_e88343_d_n9, assign53790_e88343_d_n10, assign53790_e88343_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53790_e88320: f64 = (2.0 * locals.var_t3);
        let assign53790_e88323: f64 = (locals.var_t3 * 2.0);
        let assign53790_e88325: f64 = (assign53790_e88323 * locals.var_t0);
        let assign53790_e88328: f64 = (locals.var_t3 * 2.0);
        let assign53790_e88330: f64 = (assign53790_e88328 * locals.var_t0);
        let assign53790_e88333: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign53790_e88334: f64 = (assign53790_e88330 + assign53790_e88333);
        let assign53790_e88335: f64 = (assign53790_e88325 * assign53790_e88334);
        let assign53790_e88337: f64 = (assign53790_e88335).max(1e-38);
        let assign53790_e88338: f64 = (assign53790_e88337).ln();
        let assign53790_e88339: f64 = (assign53790_e88320 + assign53790_e88338);
        let assign53790_e88341: f64 = (assign53790_e88339 - locals.var_t1);
        (assign53790_e88341, (((2.0 * locals.var_t3_dn3) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn3)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn4)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn5)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn6)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn7)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn8)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn9)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn10)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn11)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign53790_e88343;
        locals.var_t4_dn3 = assign53790_e88343_d_n3;
        locals.var_t4_dn4 = assign53790_e88343_d_n4;
        locals.var_t4_dn5 = assign53790_e88343_d_n5;
        locals.var_t4_dn6 = assign53790_e88343_d_n6;
        locals.var_t4_dn7 = assign53790_e88343_d_n7;
        locals.var_t4_dn8 = assign53790_e88343_d_n8;
        locals.var_t4_dn9 = assign53790_e88343_d_n9;
        locals.var_t4_dn10 = assign53790_e88343_d_n10;
        locals.var_t4_dn11 = assign53790_e88343_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign53800_e88367, assign53800_e88367_d_n3, assign53800_e88367_d_n4, assign53800_e88367_d_n5, assign53800_e88367_d_n6, assign53800_e88367_d_n7, assign53800_e88367_d_n8, assign53800_e88367_d_n9, assign53800_e88367_d_n10, assign53800_e88367_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53800_e88354: f64 = (1.0 / locals.var_t3);
        let assign53800_e88355: f64 = (2.0 + assign53800_e88354);
        let assign53800_e88358: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign53800_e88361: f64 = (locals.var_t0 * locals.var_t3);
        let assign53800_e88363: f64 = (assign53800_e88361 + locals.var_sqrtpsisa);
        let assign53800_e88364: f64 = (assign53800_e88358 / assign53800_e88363);
        let assign53800_e88365: f64 = (assign53800_e88355 + assign53800_e88364);
        (assign53800_e88365, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign53800_e88363 * assign53800_e88363))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign53800_e88367;
        locals.var_t5_dn3 = assign53800_e88367_d_n3;
        locals.var_t5_dn4 = assign53800_e88367_d_n4;
        locals.var_t5_dn5 = assign53800_e88367_d_n5;
        locals.var_t5_dn6 = assign53800_e88367_d_n6;
        locals.var_t5_dn7 = assign53800_e88367_d_n7;
        locals.var_t5_dn8 = assign53800_e88367_d_n8;
        locals.var_t5_dn9 = assign53800_e88367_d_n9;
        locals.var_t5_dn10 = assign53800_e88367_d_n10;
        locals.var_t5_dn11 = assign53800_e88367_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign53810_e88381, assign53810_e88381_d_n3, assign53810_e88381_d_n4, assign53810_e88381_d_n5, assign53810_e88381_d_n6, assign53810_e88381_d_n7, assign53810_e88381_d_n8, assign53810_e88381_d_n9, assign53810_e88381_d_n10, assign53810_e88381_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53810_e88378: f64 = (locals.var_t4 / locals.var_t5);
        let assign53810_e88379: f64 = (locals.var_t3 - assign53810_e88378);
        (assign53810_e88379, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53810_e88381;
        locals.var_t3_dn3 = assign53810_e88381_d_n3;
        locals.var_t3_dn4 = assign53810_e88381_d_n4;
        locals.var_t3_dn5 = assign53810_e88381_d_n5;
        locals.var_t3_dn6 = assign53810_e88381_d_n6;
        locals.var_t3_dn7 = assign53810_e88381_d_n7;
        locals.var_t3_dn8 = assign53810_e88381_d_n8;
        locals.var_t3_dn9 = assign53810_e88381_d_n9;
        locals.var_t3_dn10 = assign53810_e88381_d_n10;
        locals.var_t3_dn11 = assign53810_e88381_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign53820_e88414, assign53820_e88414_d_n3, assign53820_e88414_d_n4, assign53820_e88414_d_n5, assign53820_e88414_d_n6, assign53820_e88414_d_n7, assign53820_e88414_d_n8, assign53820_e88414_d_n9, assign53820_e88414_d_n10, assign53820_e88414_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53820_e88391: f64 = (2.0 * locals.var_t3);
        let assign53820_e88394: f64 = (locals.var_t3 * 2.0);
        let assign53820_e88396: f64 = (assign53820_e88394 * locals.var_t0);
        let assign53820_e88399: f64 = (locals.var_t3 * 2.0);
        let assign53820_e88401: f64 = (assign53820_e88399 * locals.var_t0);
        let assign53820_e88404: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign53820_e88405: f64 = (assign53820_e88401 + assign53820_e88404);
        let assign53820_e88406: f64 = (assign53820_e88396 * assign53820_e88405);
        let assign53820_e88408: f64 = (assign53820_e88406).max(1e-38);
        let assign53820_e88409: f64 = (assign53820_e88408).ln();
        let assign53820_e88410: f64 = (assign53820_e88391 + assign53820_e88409);
        let assign53820_e88412: f64 = (assign53820_e88410 - locals.var_t1);
        (assign53820_e88412, (((2.0 * locals.var_t3_dn3) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn3)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn4)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn5)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn6)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn7)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn8)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn9)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn10)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn11)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign53820_e88414;
        locals.var_t4_dn3 = assign53820_e88414_d_n3;
        locals.var_t4_dn4 = assign53820_e88414_d_n4;
        locals.var_t4_dn5 = assign53820_e88414_d_n5;
        locals.var_t4_dn6 = assign53820_e88414_d_n6;
        locals.var_t4_dn7 = assign53820_e88414_d_n7;
        locals.var_t4_dn8 = assign53820_e88414_d_n8;
        locals.var_t4_dn9 = assign53820_e88414_d_n9;
        locals.var_t4_dn10 = assign53820_e88414_d_n10;
        locals.var_t4_dn11 = assign53820_e88414_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign53830_e88438, assign53830_e88438_d_n3, assign53830_e88438_d_n4, assign53830_e88438_d_n5, assign53830_e88438_d_n6, assign53830_e88438_d_n7, assign53830_e88438_d_n8, assign53830_e88438_d_n9, assign53830_e88438_d_n10, assign53830_e88438_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53830_e88425: f64 = (1.0 / locals.var_t3);
        let assign53830_e88426: f64 = (2.0 + assign53830_e88425);
        let assign53830_e88429: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign53830_e88432: f64 = (locals.var_t0 * locals.var_t3);
        let assign53830_e88434: f64 = (assign53830_e88432 + locals.var_sqrtpsisa);
        let assign53830_e88435: f64 = (assign53830_e88429 / assign53830_e88434);
        let assign53830_e88436: f64 = (assign53830_e88426 + assign53830_e88435);
        (assign53830_e88436, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign53830_e88434 * assign53830_e88434))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign53830_e88438;
        locals.var_t5_dn3 = assign53830_e88438_d_n3;
        locals.var_t5_dn4 = assign53830_e88438_d_n4;
        locals.var_t5_dn5 = assign53830_e88438_d_n5;
        locals.var_t5_dn6 = assign53830_e88438_d_n6;
        locals.var_t5_dn7 = assign53830_e88438_d_n7;
        locals.var_t5_dn8 = assign53830_e88438_d_n8;
        locals.var_t5_dn9 = assign53830_e88438_d_n9;
        locals.var_t5_dn10 = assign53830_e88438_d_n10;
        locals.var_t5_dn11 = assign53830_e88438_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign53840_e88466, assign53840_e88466_d_n3, assign53840_e88466_d_n4, assign53840_e88466_d_n5, assign53840_e88466_d_n6, assign53840_e88466_d_n7, assign53840_e88466_d_n8, assign53840_e88466_d_n9, assign53840_e88466_d_n10, assign53840_e88466_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53840_e88448: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign53840_e88451: f64 = (locals.var_t0 * locals.var_t3);
        let assign53840_e88453: f64 = (assign53840_e88451 + locals.var_sqrtpsisa);
        let assign53840_e88454: f64 = (assign53840_e88448 / assign53840_e88453);
        let assign53840_e88457: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign53840_e88460: f64 = (locals.var_t0 * locals.var_t3);
        let assign53840_e88462: f64 = (assign53840_e88460 + locals.var_sqrtpsisa);
        let assign53840_e88463: f64 = (assign53840_e88457 / assign53840_e88462);
        let assign53840_e88464: f64 = (assign53840_e88454 * assign53840_e88463);
        (assign53840_e88464, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign53840_e88462 * assign53840_e88462)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign53840_e88466;
        locals.var_t6_dn3 = assign53840_e88466_d_n3;
        locals.var_t6_dn4 = assign53840_e88466_d_n4;
        locals.var_t6_dn5 = assign53840_e88466_d_n5;
        locals.var_t6_dn6 = assign53840_e88466_d_n6;
        locals.var_t6_dn7 = assign53840_e88466_d_n7;
        locals.var_t6_dn8 = assign53840_e88466_d_n8;
        locals.var_t6_dn9 = assign53840_e88466_d_n9;
        locals.var_t6_dn10 = assign53840_e88466_d_n10;
        locals.var_t6_dn11 = assign53840_e88466_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign53850_e88499, assign53850_e88499_d_n3, assign53850_e88499_d_n4, assign53850_e88499_d_n5, assign53850_e88499_d_n6, assign53850_e88499_d_n7, assign53850_e88499_d_n8, assign53850_e88499_d_n9, assign53850_e88499_d_n10, assign53850_e88499_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign53850_e88476: f64 = (1.0 * __rspice_inv_cse_0);
        let assign53850_e88479: f64 = (1.0 * __rspice_inv_cse_0);
        let assign53850_e88480: f64 = (assign53850_e88476 * assign53850_e88479);
        let assign53850_e88481: f64 = (-assign53850_e88480);
        let assign53850_e88485: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign53850_e88487: f64 = (assign53850_e88485 * locals.var_sqrtpsisa);
        let assign53850_e88490: f64 = (locals.var_t0 * locals.var_t3);
        let assign53850_e88492: f64 = (assign53850_e88490 + locals.var_sqrtpsisa);
        let assign53850_e88493: f64 = (assign53850_e88487 * assign53850_e88492);
        let assign53850_e88494: f64 = (1.0 / assign53850_e88493);
        let assign53850_e88495: f64 = (assign53850_e88481 - assign53850_e88494);
        let assign53850_e88497: f64 = (assign53850_e88495 - locals.var_t6);
        (assign53850_e88497, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn3)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn4)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn5)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn6)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn7)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn8)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn9)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn10)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn11)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign53850_e88499;
        locals.var_t7_dn3 = assign53850_e88499_d_n3;
        locals.var_t7_dn4 = assign53850_e88499_d_n4;
        locals.var_t7_dn5 = assign53850_e88499_d_n5;
        locals.var_t7_dn6 = assign53850_e88499_d_n6;
        locals.var_t7_dn7 = assign53850_e88499_d_n7;
        locals.var_t7_dn8 = assign53850_e88499_d_n8;
        locals.var_t7_dn9 = assign53850_e88499_d_n9;
        locals.var_t7_dn10 = assign53850_e88499_d_n10;
        locals.var_t7_dn11 = assign53850_e88499_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign53860_e88525, assign53860_e88525_d_n3, assign53860_e88525_d_n4, assign53860_e88525_d_n5, assign53860_e88525_d_n6, assign53860_e88525_d_n7, assign53860_e88525_d_n8, assign53860_e88525_d_n9, assign53860_e88525_d_n10, assign53860_e88525_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53860_e88510: f64 = (locals.var_t4 / locals.var_t5);
        let assign53860_e88514: f64 = (locals.var_t4 * locals.var_t7);
        let assign53860_e88517: f64 = (2.0 * locals.var_t5);
        let assign53860_e88519: f64 = (assign53860_e88517 * locals.var_t5);
        let assign53860_e88520: f64 = (assign53860_e88514 / assign53860_e88519);
        let assign53860_e88521: f64 = (1.0 + assign53860_e88520);
        let assign53860_e88522: f64 = (assign53860_e88510 * assign53860_e88521);
        let assign53860_e88523: f64 = (locals.var_t3 - assign53860_e88522);
        (assign53860_e88523, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn3)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn4)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn5)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn6)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn7)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn8)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn9)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn10)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn11)))) / (assign53860_e88519 * assign53860_e88519))))),)
    } else {
        (locals.var_qsh, locals.var_qsh_dn3, locals.var_qsh_dn4, locals.var_qsh_dn5, locals.var_qsh_dn6, locals.var_qsh_dn7, locals.var_qsh_dn8, locals.var_qsh_dn9, locals.var_qsh_dn10, locals.var_qsh_dn11,)
    }
};
        locals.var_qsh = assign53860_e88525;
        locals.var_qsh_dn3 = assign53860_e88525_d_n3;
        locals.var_qsh_dn4 = assign53860_e88525_d_n4;
        locals.var_qsh_dn5 = assign53860_e88525_d_n5;
        locals.var_qsh_dn6 = assign53860_e88525_d_n6;
        locals.var_qsh_dn7 = assign53860_e88525_d_n7;
        locals.var_qsh_dn8 = assign53860_e88525_d_n8;
        locals.var_qsh_dn9 = assign53860_e88525_d_n9;
        locals.var_qsh_dn10 = assign53860_e88525_d_n10;
        locals.var_qsh_dn11 = assign53860_e88525_d_n11;
        locals.var_qsh_rv = 0.0;

        let (assign53870_e88551, assign53870_e88551_d_n3, assign53870_e88551_d_n4, assign53870_e88551_d_n5, assign53870_e88551_d_n6, assign53870_e88551_d_n7, assign53870_e88551_d_n8, assign53870_e88551_d_n9, assign53870_e88551_d_n10, assign53870_e88551_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53870_e88533: f64 = (locals.var_psiph + 1.0);
        let assign53870_e88536: f64 = (locals.var_psiph - 1.0);
        let assign53870_e88539: f64 = (locals.var_psiph - 1.0);
        let assign53870_e88540: f64 = (assign53870_e88536 * assign53870_e88539);
        let assign53870_e88543: f64 = (0.25 * 2.0);
        let assign53870_e88545: f64 = (assign53870_e88543 * 2.0);
        let assign53870_e88546: f64 = (assign53870_e88540 + assign53870_e88545);
        let assign53870_e88547: f64 = (assign53870_e88546).sqrt();
        let assign53870_e88548: f64 = (assign53870_e88533 + assign53870_e88547);
        let assign53870_e88549: f64 = (0.5 * assign53870_e88548);
        (assign53870_e88549, (0.5 * (locals.var_psiph_dn3 + (((locals.var_psiph_dn3 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn3)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn4 + (((locals.var_psiph_dn4 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn4)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn5 + (((locals.var_psiph_dn5 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn5)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn6 + (((locals.var_psiph_dn6 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn6)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn7 + (((locals.var_psiph_dn7 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn7)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn8 + (((locals.var_psiph_dn8 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn8)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn9 + (((locals.var_psiph_dn9 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn9)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn10 + (((locals.var_psiph_dn10 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn10)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn11 + (((locals.var_psiph_dn11 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn11)) / (2.0 * assign53870_e88547)))),)
    } else {
        (locals.var_psiphclamp, locals.var_psiphclamp_dn3, locals.var_psiphclamp_dn4, locals.var_psiphclamp_dn5, locals.var_psiphclamp_dn6, locals.var_psiphclamp_dn7, locals.var_psiphclamp_dn8, locals.var_psiphclamp_dn9, locals.var_psiphclamp_dn10, locals.var_psiphclamp_dn11,)
    }
};
        locals.var_psiphclamp = assign53870_e88551;
        locals.var_psiphclamp_dn3 = assign53870_e88551_d_n3;
        locals.var_psiphclamp_dn4 = assign53870_e88551_d_n4;
        locals.var_psiphclamp_dn5 = assign53870_e88551_d_n5;
        locals.var_psiphclamp_dn6 = assign53870_e88551_d_n6;
        locals.var_psiphclamp_dn7 = assign53870_e88551_d_n7;
        locals.var_psiphclamp_dn8 = assign53870_e88551_d_n8;
        locals.var_psiphclamp_dn9 = assign53870_e88551_d_n9;
        locals.var_psiphclamp_dn10 = assign53870_e88551_d_n10;
        locals.var_psiphclamp_dn11 = assign53870_e88551_d_n11;
        locals.var_psiphclamp_rv = 0.0;

        let (assign53880_e88565, assign53880_e88565_d_n3, assign53880_e88565_d_n4, assign53880_e88565_d_n5, assign53880_e88565_d_n6, assign53880_e88565_d_n7, assign53880_e88565_d_n8, assign53880_e88565_d_n9, assign53880_e88565_d_n10, assign53880_e88565_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53880_e88560: f64 = (locals.var_psiphclamp).sqrt();
        let assign53880_e88561: f64 = (2.0 * assign53880_e88560);
        let assign53880_e88562: f64 = (locals.var_gam_h / assign53880_e88561);
        let assign53880_e88563: f64 = (1.0 + assign53880_e88562);
        (assign53880_e88563, (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn3 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (((locals.var_gam_h_dn4 * assign53880_e88561) - (locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn4 / (2.0 * assign53880_e88560))))) / (assign53880_e88561 * assign53880_e88561)), (((locals.var_gam_h_dn5 * assign53880_e88561) - (locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn5 / (2.0 * assign53880_e88560))))) / (assign53880_e88561 * assign53880_e88561)), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn6 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn7 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn8 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn9 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn10 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn11 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))),)
    } else {
        (locals.var_nq_h, locals.var_nq_h_dn3, locals.var_nq_h_dn4, locals.var_nq_h_dn5, locals.var_nq_h_dn6, locals.var_nq_h_dn7, locals.var_nq_h_dn8, locals.var_nq_h_dn9, locals.var_nq_h_dn10, locals.var_nq_h_dn11,)
    }
};
        locals.var_nq_h = assign53880_e88565;
        locals.var_nq_h_dn3 = assign53880_e88565_d_n3;
        locals.var_nq_h_dn4 = assign53880_e88565_d_n4;
        locals.var_nq_h_dn5 = assign53880_e88565_d_n5;
        locals.var_nq_h_dn6 = assign53880_e88565_d_n6;
        locals.var_nq_h_dn7 = assign53880_e88565_d_n7;
        locals.var_nq_h_dn8 = assign53880_e88565_d_n8;
        locals.var_nq_h_dn9 = assign53880_e88565_d_n9;
        locals.var_nq_h_dn10 = assign53880_e88565_d_n10;
        locals.var_nq_h_dn11 = assign53880_e88565_d_n11;
        locals.var_nq_h_rv = 0.0;

        let (assign53890_e88572, assign53890_e88572_d_n3, assign53890_e88572_d_n4, assign53890_e88572_d_n5, assign53890_e88572_d_n6, assign53890_e88572_d_n7, assign53890_e88572_d_n8, assign53890_e88572_d_n9, assign53890_e88572_d_n10, assign53890_e88572_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        (locals.var_ueff, locals.var_ueff_dn3, locals.var_ueff_dn4, locals.var_ueff_dn5, locals.var_ueff_dn6, locals.var_ueff_dn7, locals.var_ueff_dn8, locals.var_ueff_dn9, locals.var_ueff_dn10, locals.var_ueff_dn11,)
    } else {
        (locals.var_u0_i_h, locals.var_u0_i_h_dn3, locals.var_u0_i_h_dn4, locals.var_u0_i_h_dn5, locals.var_u0_i_h_dn6, locals.var_u0_i_h_dn7, locals.var_u0_i_h_dn8, locals.var_u0_i_h_dn9, locals.var_u0_i_h_dn10, locals.var_u0_i_h_dn11,)
    }
};
        locals.var_u0_i_h = assign53890_e88572;
        locals.var_u0_i_h_dn3 = assign53890_e88572_d_n3;
        locals.var_u0_i_h_dn4 = assign53890_e88572_d_n4;
        locals.var_u0_i_h_dn5 = assign53890_e88572_d_n5;
        locals.var_u0_i_h_dn6 = assign53890_e88572_d_n6;
        locals.var_u0_i_h_dn7 = assign53890_e88572_d_n7;
        locals.var_u0_i_h_dn8 = assign53890_e88572_d_n8;
        locals.var_u0_i_h_dn9 = assign53890_e88572_d_n9;
        locals.var_u0_i_h_dn10 = assign53890_e88572_d_n10;
        locals.var_u0_i_h_dn11 = assign53890_e88572_d_n11;
        locals.var_u0_i_h_rv = 0.0;

        let (assign53900_e88583, assign53900_e88583_d_n3, assign53900_e88583_d_n4, assign53900_e88583_d_n5, assign53900_e88583_d_n6, assign53900_e88583_d_n7, assign53900_e88583_d_n8, assign53900_e88583_d_n9, assign53900_e88583_d_n10, assign53900_e88583_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53900_e88579: f64 = (locals.var_u0_i_h * locals.var_cox);
        let assign53900_e88581: f64 = (assign53900_e88579 * locals.var_weff);
        (assign53900_e88581, ((locals.var_u0_i_h_dn3 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn4 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn5 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn6 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn7 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn8 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn9 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn10 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn11 * locals.var_cox) * locals.var_weff),)
    } else {
        (locals.var_beta_h, locals.var_beta_h_dn3, locals.var_beta_h_dn4, locals.var_beta_h_dn5, locals.var_beta_h_dn6, locals.var_beta_h_dn7, locals.var_beta_h_dn8, locals.var_beta_h_dn9, locals.var_beta_h_dn10, locals.var_beta_h_dn11,)
    }
};
        locals.var_beta_h = assign53900_e88583;
        locals.var_beta_h_dn3 = assign53900_e88583_d_n3;
        locals.var_beta_h_dn4 = assign53900_e88583_d_n4;
        locals.var_beta_h_dn5 = assign53900_e88583_d_n5;
        locals.var_beta_h_dn6 = assign53900_e88583_d_n6;
        locals.var_beta_h_dn7 = assign53900_e88583_d_n7;
        locals.var_beta_h_dn8 = assign53900_e88583_d_n8;
        locals.var_beta_h_dn9 = assign53900_e88583_d_n9;
        locals.var_beta_h_dn10 = assign53900_e88583_d_n10;
        locals.var_beta_h_dn11 = assign53900_e88583_d_n11;
        locals.var_beta_h_rv = 0.0;

        let (assign53910_e88594, assign53910_e88594_d_n3, assign53910_e88594_d_n4, assign53910_e88594_d_n5, assign53910_e88594_d_n6, assign53910_e88594_d_n7, assign53910_e88594_d_n8, assign53910_e88594_d_n9, assign53910_e88594_d_n10, assign53910_e88594_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53910_e88590: f64 = (locals.var_ueff * locals.var_cox);
        let assign53910_e88592: f64 = (assign53910_e88590 * locals.var_weff);
        (assign53910_e88592, ((locals.var_ueff_dn3 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn4 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn5 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn6 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn7 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn8 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn9 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn10 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn11 * locals.var_cox) * locals.var_weff),)
    } else {
        (locals.var_beta_ch, locals.var_beta_ch_dn3, locals.var_beta_ch_dn4, locals.var_beta_ch_dn5, locals.var_beta_ch_dn6, locals.var_beta_ch_dn7, locals.var_beta_ch_dn8, locals.var_beta_ch_dn9, locals.var_beta_ch_dn10, locals.var_beta_ch_dn11,)
    }
};
        locals.var_beta_ch = assign53910_e88594;
        locals.var_beta_ch_dn3 = assign53910_e88594_d_n3;
        locals.var_beta_ch_dn4 = assign53910_e88594_d_n4;
        locals.var_beta_ch_dn5 = assign53910_e88594_d_n5;
        locals.var_beta_ch_dn6 = assign53910_e88594_d_n6;
        locals.var_beta_ch_dn7 = assign53910_e88594_d_n7;
        locals.var_beta_ch_dn8 = assign53910_e88594_d_n8;
        locals.var_beta_ch_dn9 = assign53910_e88594_d_n9;
        locals.var_beta_ch_dn10 = assign53910_e88594_d_n10;
        locals.var_beta_ch_dn11 = assign53910_e88594_d_n11;
        locals.var_beta_ch_rv = 0.0;

        let (assign53920_e88613, assign53920_e88613_d_n3, assign53920_e88613_d_n4, assign53920_e88613_d_n5, assign53920_e88613_d_n6, assign53920_e88613_d_n7, assign53920_e88613_d_n8, assign53920_e88613_d_n9, assign53920_e88613_d_n10, assign53920_e88613_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53920_e88601: f64 = (locals.var_ids * locals.var_lh1);
        let assign53920_e88604: f64 = (2.0 * locals.var_nq_h);
        let assign53920_e88606: f64 = (assign53920_e88604 * locals.var_beta_h);
        let assign53920_e88608: f64 = (assign53920_e88606 * locals.var_vt);
        let assign53920_e88610: f64 = (assign53920_e88608 * locals.var_vt);
        let assign53920_e88611: f64 = (assign53920_e88601 / assign53920_e88610);
        (assign53920_e88611, ((((locals.var_ids_dn3 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn3) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn3)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn4 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((((2.0 * locals.var_nq_h_dn4) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn4)) * locals.var_vt) + (assign53920_e88606 * locals.var_vt_dn4)) * locals.var_vt) + (assign53920_e88608 * locals.var_vt_dn4)))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn5 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((((2.0 * locals.var_nq_h_dn5) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn5)) * locals.var_vt) + (assign53920_e88606 * locals.var_vt_dn5)) * locals.var_vt) + (assign53920_e88608 * locals.var_vt_dn5)))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn6 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn6) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn6)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn7 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn7) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn7)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn8 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn8) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn8)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn9 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn9) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn9)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn10 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn10) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn10)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn11 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn11) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn11)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)),)
    } else {
        (locals.var_i1, locals.var_i1_dn3, locals.var_i1_dn4, locals.var_i1_dn5, locals.var_i1_dn6, locals.var_i1_dn7, locals.var_i1_dn8, locals.var_i1_dn9, locals.var_i1_dn10, locals.var_i1_dn11,)
    }
};
        locals.var_i1 = assign53920_e88613;
        locals.var_i1_dn3 = assign53920_e88613_d_n3;
        locals.var_i1_dn4 = assign53920_e88613_d_n4;
        locals.var_i1_dn5 = assign53920_e88613_d_n5;
        locals.var_i1_dn6 = assign53920_e88613_d_n6;
        locals.var_i1_dn7 = assign53920_e88613_d_n7;
        locals.var_i1_dn8 = assign53920_e88613_d_n8;
        locals.var_i1_dn9 = assign53920_e88613_d_n9;
        locals.var_i1_dn10 = assign53920_e88613_d_n10;
        locals.var_i1_dn11 = assign53920_e88613_d_n11;
        locals.var_i1_rv = 0.0;

        let (assign53930_e88634, assign53930_e88634_d_n3, assign53930_e88634_d_n4, assign53930_e88634_d_n5, assign53930_e88634_d_n6, assign53930_e88634_d_n7, assign53930_e88634_d_n8, assign53930_e88634_d_n9, assign53930_e88634_d_n10, assign53930_e88634_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53930_e88621: f64 = (locals.var_leffnoih - locals.var_lh1);
        let assign53930_e88622: f64 = (locals.var_ids * assign53930_e88621);
        let assign53930_e88625: f64 = (2.0 * locals.var_nq);
        let assign53930_e88627: f64 = (assign53930_e88625 * locals.var_beta_ch);
        let assign53930_e88629: f64 = (assign53930_e88627 * locals.var_nvt);
        let assign53930_e88631: f64 = (assign53930_e88629 * locals.var_nvt);
        let assign53930_e88632: f64 = (assign53930_e88622 / assign53930_e88631);
        (assign53930_e88632, ((((locals.var_ids_dn3 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn3) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn3)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn3)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn4 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn4) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn4)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn4)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn5 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn5) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn5)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn5)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn6 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn6) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn6)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn6)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn7 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn7) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn7)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn7)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn8 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn8) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn8)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn8)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn9 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn9) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn9)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn9)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn10 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn10) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn10)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn10)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn11 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn11) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn11)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn11)))) / (assign53930_e88631 * assign53930_e88631)),)
    } else {
        (locals.var_i2, locals.var_i2_dn3, locals.var_i2_dn4, locals.var_i2_dn5, locals.var_i2_dn6, locals.var_i2_dn7, locals.var_i2_dn8, locals.var_i2_dn9, locals.var_i2_dn10, locals.var_i2_dn11,)
    }
};
        locals.var_i2 = assign53930_e88634;
        locals.var_i2_dn3 = assign53930_e88634_d_n3;
        locals.var_i2_dn4 = assign53930_e88634_d_n4;
        locals.var_i2_dn5 = assign53930_e88634_d_n5;
        locals.var_i2_dn6 = assign53930_e88634_d_n6;
        locals.var_i2_dn7 = assign53930_e88634_d_n7;
        locals.var_i2_dn8 = assign53930_e88634_d_n8;
        locals.var_i2_dn9 = assign53930_e88634_d_n9;
        locals.var_i2_dn10 = assign53930_e88634_d_n10;
        locals.var_i2_dn11 = assign53930_e88634_d_n11;
        locals.var_i2_rv = 0.0;

        let (assign53940_e88651, assign53940_e88651_d_n3, assign53940_e88651_d_n4, assign53940_e88651_d_n5, assign53940_e88651_d_n6, assign53940_e88651_d_n7, assign53940_e88651_d_n8, assign53940_e88651_d_n9, assign53940_e88651_d_n10, assign53940_e88651_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53940_e88643: f64 = (locals.var_qsh * locals.var_qsh);
        let assign53940_e88645: f64 = (assign53940_e88643 + locals.var_qsh);
        let assign53940_e88647: f64 = (assign53940_e88645 - locals.var_i1);
        let assign53940_e88648: f64 = (4.0 * assign53940_e88647);
        let assign53940_e88649: f64 = (1.0 + assign53940_e88648);
        (assign53940_e88649, (4.0 * ((((locals.var_qsh_dn3 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn3)) + locals.var_qsh_dn3) - locals.var_i1_dn3)), (4.0 * ((((locals.var_qsh_dn4 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn4)) + locals.var_qsh_dn4) - locals.var_i1_dn4)), (4.0 * ((((locals.var_qsh_dn5 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn5)) + locals.var_qsh_dn5) - locals.var_i1_dn5)), (4.0 * ((((locals.var_qsh_dn6 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn6)) + locals.var_qsh_dn6) - locals.var_i1_dn6)), (4.0 * ((((locals.var_qsh_dn7 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn7)) + locals.var_qsh_dn7) - locals.var_i1_dn7)), (4.0 * ((((locals.var_qsh_dn8 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn8)) + locals.var_qsh_dn8) - locals.var_i1_dn8)), (4.0 * ((((locals.var_qsh_dn9 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn9)) + locals.var_qsh_dn9) - locals.var_i1_dn9)), (4.0 * ((((locals.var_qsh_dn10 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn10)) + locals.var_qsh_dn10) - locals.var_i1_dn10)), (4.0 * ((((locals.var_qsh_dn11 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn11)) + locals.var_qsh_dn11) - locals.var_i1_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign53940_e88651;
        locals.var_t0_dn3 = assign53940_e88651_d_n3;
        locals.var_t0_dn4 = assign53940_e88651_d_n4;
        locals.var_t0_dn5 = assign53940_e88651_d_n5;
        locals.var_t0_dn6 = assign53940_e88651_d_n6;
        locals.var_t0_dn7 = assign53940_e88651_d_n7;
        locals.var_t0_dn8 = assign53940_e88651_d_n8;
        locals.var_t0_dn9 = assign53940_e88651_d_n9;
        locals.var_t0_dn10 = assign53940_e88651_d_n10;
        locals.var_t0_dn11 = assign53940_e88651_d_n11;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_186(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign53980_e88702, assign53980_e88702_d_n3, assign53980_e88702_d_n4, assign53980_e88702_d_n5, assign53980_e88702_d_n6, assign53980_e88702_d_n7, assign53980_e88702_d_n8, assign53980_e88702_d_n9, assign53980_e88702_d_n10, assign53980_e88702_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53980_e88685: f64 = (-0.5);
        let assign53980_e88691: f64 = (locals.var_qdeff * locals.var_qdeff);
        let assign53980_e88693: f64 = (assign53980_e88691 + locals.var_qdeff);
        let assign53980_e88695: f64 = (assign53980_e88693 + locals.var_i2);
        let assign53980_e88696: f64 = (4.0 * assign53980_e88695);
        let assign53980_e88697: f64 = (1.0 + assign53980_e88696);
        let assign53980_e88698: f64 = (assign53980_e88697).sqrt();
        let assign53980_e88699: f64 = (0.5 * assign53980_e88698);
        let assign53980_e88700: f64 = (assign53980_e88685 + assign53980_e88699);
        (assign53980_e88700, (0.5 * ((4.0 * ((((locals.var_qdeff_dn3 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn3)) + locals.var_qdeff_dn3) + locals.var_i2_dn3)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn4 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn4)) + locals.var_qdeff_dn4) + locals.var_i2_dn4)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn5 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn5)) + locals.var_qdeff_dn5) + locals.var_i2_dn5)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn6 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn6)) + locals.var_qdeff_dn6) + locals.var_i2_dn6)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn7 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn7)) + locals.var_qdeff_dn7) + locals.var_i2_dn7)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn8 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn8)) + locals.var_qdeff_dn8) + locals.var_i2_dn8)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn9 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn9)) + locals.var_qdeff_dn9) + locals.var_i2_dn9)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn10 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn10)) + locals.var_qdeff_dn10) + locals.var_i2_dn10)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn11 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn11)) + locals.var_qdeff_dn11) + locals.var_i2_dn11)) / (2.0 * assign53980_e88698))),)
    } else {
        (locals.var_qsch, locals.var_qsch_dn3, locals.var_qsch_dn4, locals.var_qsch_dn5, locals.var_qsch_dn6, locals.var_qsch_dn7, locals.var_qsch_dn8, locals.var_qsch_dn9, locals.var_qsch_dn10, locals.var_qsch_dn11,)
    }
};
        locals.var_qsch = assign53980_e88702;
        locals.var_qsch_dn3 = assign53980_e88702_d_n3;
        locals.var_qsch_dn4 = assign53980_e88702_d_n4;
        locals.var_qsch_dn5 = assign53980_e88702_d_n5;
        locals.var_qsch_dn6 = assign53980_e88702_d_n6;
        locals.var_qsch_dn7 = assign53980_e88702_d_n7;
        locals.var_qsch_dn8 = assign53980_e88702_d_n8;
        locals.var_qsch_dn9 = assign53980_e88702_d_n9;
        locals.var_qsch_dn10 = assign53980_e88702_d_n10;
        locals.var_qsch_dn11 = assign53980_e88702_d_n11;
        locals.var_qsch_rv = 0.0;

        let assign54070_e88811: f64 = if locals.var_leff != locals.var_lh1 { 1.0 } else { 0.0 };
        locals.var_guard830 = assign54070_e88811;
        locals.var_guard830_rv = 0.0;

        let (assign54080_e88830, assign54080_e88830_d_n3, assign54080_e88830_d_n4, assign54080_e88830_d_n5, assign54080_e88830_d_n6, assign54080_e88830_d_n7, assign54080_e88830_d_n8, assign54080_e88830_d_n9, assign54080_e88830_d_n10, assign54080_e88830_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54080_e88820: f64 = (2.0 * locals.var_nq);
        let assign54080_e88822: f64 = (assign54080_e88820 * locals.var_cox);
        let assign54080_e88824: f64 = (assign54080_e88822 * locals.var_vt);
        let assign54080_e88826: f64 = (assign54080_e88824 * locals.var_qsch);
        let assign54080_e88828: f64 = (assign54080_e88826 / 1.602176462e-19);
        (assign54080_e88828, ((((((2.0 * locals.var_nq_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn3)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_dn4) * locals.var_cox) * locals.var_vt) + (assign54080_e88822 * locals.var_vt_dn4)) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn4)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_dn5) * locals.var_cox) * locals.var_vt) + (assign54080_e88822 * locals.var_vt_dn5)) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn5)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn6)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn7)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn8)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn9)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn10)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn11)) / 1.602176462e-19),)
    } else {
        (locals.var_np2, locals.var_np2_dn3, locals.var_np2_dn4, locals.var_np2_dn5, locals.var_np2_dn6, locals.var_np2_dn7, locals.var_np2_dn8, locals.var_np2_dn9, locals.var_np2_dn10, locals.var_np2_dn11,)
    }
};
        locals.var_np2 = assign54080_e88830;
        locals.var_np2_dn3 = assign54080_e88830_d_n3;
        locals.var_np2_dn4 = assign54080_e88830_d_n4;
        locals.var_np2_dn5 = assign54080_e88830_d_n5;
        locals.var_np2_dn6 = assign54080_e88830_d_n6;
        locals.var_np2_dn7 = assign54080_e88830_d_n7;
        locals.var_np2_dn8 = assign54080_e88830_d_n8;
        locals.var_np2_dn9 = assign54080_e88830_d_n9;
        locals.var_np2_dn10 = assign54080_e88830_d_n10;
        locals.var_np2_dn11 = assign54080_e88830_d_n11;
        locals.var_np2_rv = 0.0;

        let (assign54090_e88845,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54090_e88840: f64 = (2.0 * locals.var_lintnoi_i);
        let assign54090_e88841: f64 = (locals.var_leffnoih - assign54090_e88840);
        let assign54090_e88843: f64 = (assign54090_e88841 - locals.var_lh1);
        (assign54090_e88843,)
    } else {
        (locals.var_leffnoi,)
    }
};
        locals.var_leffnoi = assign54090_e88845;
        locals.var_leffnoi_rv = 0.0;

        let (assign54100_e88856,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54100_e88854: f64 = (locals.var_leffnoi * locals.var_leffnoi);
        (assign54100_e88854,)
    } else {
        (locals.var_leffnoisq,)
    }
};
        locals.var_leffnoisq = assign54100_e88856;
        locals.var_leffnoisq_rv = 0.0;

        let (assign54110_e88869, assign54110_e88869_d_n3, assign54110_e88869_d_n4, assign54110_e88869_d_n5, assign54110_e88869_d_n6, assign54110_e88869_d_n7, assign54110_e88869_d_n8, assign54110_e88869_d_n9, assign54110_e88869_d_n10, assign54110_e88869_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54110_e88865: f64 = (10000000000.0 * locals.var_cox);
        let assign54110_e88867: f64 = (assign54110_e88865 * locals.var_leffnoisq);
        (assign54110_e88867, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign54110_e88869;
        locals.var_t1_dn3 = assign54110_e88869_d_n3;
        locals.var_t1_dn4 = assign54110_e88869_d_n4;
        locals.var_t1_dn5 = assign54110_e88869_d_n5;
        locals.var_t1_dn6 = assign54110_e88869_d_n6;
        locals.var_t1_dn7 = assign54110_e88869_d_n7;
        locals.var_t1_dn8 = assign54110_e88869_d_n8;
        locals.var_t1_dn9 = assign54110_e88869_d_n9;
        locals.var_t1_dn10 = assign54110_e88869_d_n10;
        locals.var_t1_dn11 = assign54110_e88869_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign54120_e88889, assign54120_e88889_d_n3, assign54120_e88889_d_n4, assign54120_e88889_d_n5, assign54120_e88889_d_n6, assign54120_e88889_d_n7, assign54120_e88889_d_n8, assign54120_e88889_d_n9, assign54120_e88889_d_n10, assign54120_e88889_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54120_e88879: f64 = (locals.var_np2 + locals.var_nstar);
        let assign54120_e88882: f64 = (locals.var_nl + locals.var_nstar);
        let assign54120_e88883: f64 = (assign54120_e88879 / assign54120_e88882);
        let assign54120_e88885: f64 = (assign54120_e88883).max(1e-38);
        let assign54120_e88886: f64 = (assign54120_e88885).ln();
        let assign54120_e88887: f64 = (p.p1012 * assign54120_e88886);
        (assign54120_e88887, (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn3 + locals.var_nstar_dn3) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn4 + locals.var_nstar_dn4) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn5 + locals.var_nstar_dn5) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn6 + locals.var_nstar_dn6) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn7 + locals.var_nstar_dn7) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn8 + locals.var_nstar_dn8) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn9 + locals.var_nstar_dn9) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn10 + locals.var_nstar_dn10) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn11 + locals.var_nstar_dn11) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign54120_e88889;
        locals.var_t2_dn3 = assign54120_e88889_d_n3;
        locals.var_t2_dn4 = assign54120_e88889_d_n4;
        locals.var_t2_dn5 = assign54120_e88889_d_n5;
        locals.var_t2_dn6 = assign54120_e88889_d_n6;
        locals.var_t2_dn7 = assign54120_e88889_d_n7;
        locals.var_t2_dn8 = assign54120_e88889_d_n8;
        locals.var_t2_dn9 = assign54120_e88889_d_n9;
        locals.var_t2_dn10 = assign54120_e88889_d_n10;
        locals.var_t2_dn11 = assign54120_e88889_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign54130_e88902, assign54130_e88902_d_n3, assign54130_e88902_d_n4, assign54130_e88902_d_n5, assign54130_e88902_d_n6, assign54130_e88902_d_n7, assign54130_e88902_d_n8, assign54130_e88902_d_n9, assign54130_e88902_d_n10, assign54130_e88902_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54130_e88899: f64 = (locals.var_np2 - locals.var_nl);
        let assign54130_e88900: f64 = (p.p1013 * assign54130_e88899);
        (assign54130_e88900, (p.p1013 * (locals.var_np2_dn3 - locals.var_nl_dn3)), (p.p1013 * (locals.var_np2_dn4 - locals.var_nl_dn4)), (p.p1013 * (locals.var_np2_dn5 - locals.var_nl_dn5)), (p.p1013 * (locals.var_np2_dn6 - locals.var_nl_dn6)), (p.p1013 * (locals.var_np2_dn7 - locals.var_nl_dn7)), (p.p1013 * (locals.var_np2_dn8 - locals.var_nl_dn8)), (p.p1013 * (locals.var_np2_dn9 - locals.var_nl_dn9)), (p.p1013 * (locals.var_np2_dn10 - locals.var_nl_dn10)), (p.p1013 * (locals.var_np2_dn11 - locals.var_nl_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign54130_e88902;
        locals.var_t3_dn3 = assign54130_e88902_d_n3;
        locals.var_t3_dn4 = assign54130_e88902_d_n4;
        locals.var_t3_dn5 = assign54130_e88902_d_n5;
        locals.var_t3_dn6 = assign54130_e88902_d_n6;
        locals.var_t3_dn7 = assign54130_e88902_d_n7;
        locals.var_t3_dn8 = assign54130_e88902_d_n8;
        locals.var_t3_dn9 = assign54130_e88902_d_n9;
        locals.var_t3_dn10 = assign54130_e88902_d_n10;
        locals.var_t3_dn11 = assign54130_e88902_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign54140_e88921, assign54140_e88921_d_n3, assign54140_e88921_d_n4, assign54140_e88921_d_n5, assign54140_e88921_d_n6, assign54140_e88921_d_n7, assign54140_e88921_d_n8, assign54140_e88921_d_n9, assign54140_e88921_d_n10, assign54140_e88921_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54140_e88911: f64 = (0.5 * p.p1014);
        let assign54140_e88914: f64 = (locals.var_np2 * locals.var_np2);
        let assign54140_e88917: f64 = (locals.var_nl * locals.var_nl);
        let assign54140_e88918: f64 = (assign54140_e88914 - assign54140_e88917);
        let assign54140_e88919: f64 = (assign54140_e88911 * assign54140_e88918);
        (assign54140_e88919, (assign54140_e88911 * (((locals.var_np2_dn3 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign54140_e88911 * (((locals.var_np2_dn4 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign54140_e88911 * (((locals.var_np2_dn5 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign54140_e88911 * (((locals.var_np2_dn6 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign54140_e88911 * (((locals.var_np2_dn7 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign54140_e88911 * (((locals.var_np2_dn8 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign54140_e88911 * (((locals.var_np2_dn9 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign54140_e88911 * (((locals.var_np2_dn10 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign54140_e88911 * (((locals.var_np2_dn11 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign54140_e88921;
        locals.var_t4_dn3 = assign54140_e88921_d_n3;
        locals.var_t4_dn4 = assign54140_e88921_d_n4;
        locals.var_t4_dn5 = assign54140_e88921_d_n5;
        locals.var_t4_dn6 = assign54140_e88921_d_n6;
        locals.var_t4_dn7 = assign54140_e88921_d_n7;
        locals.var_t4_dn8 = assign54140_e88921_d_n8;
        locals.var_t4_dn9 = assign54140_e88921_d_n9;
        locals.var_t4_dn10 = assign54140_e88921_d_n10;
        locals.var_t4_dn11 = assign54140_e88921_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign54150_e88936, assign54150_e88936_d_n3, assign54150_e88936_d_n4, assign54150_e88936_d_n5, assign54150_e88936_d_n6, assign54150_e88936_d_n7, assign54150_e88936_d_n8, assign54150_e88936_d_n9, assign54150_e88936_d_n10, assign54150_e88936_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54150_e88930: f64 = (10000000000.0 * locals.var_leffnoisq);
        let assign54150_e88932: f64 = (assign54150_e88930 * locals.var_weff);
        let assign54150_e88934: f64 = (assign54150_e88932 * p.p2);
        (assign54150_e88934, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign54150_e88936;
        locals.var_t5_dn3 = assign54150_e88936_d_n3;
        locals.var_t5_dn4 = assign54150_e88936_d_n4;
        locals.var_t5_dn5 = assign54150_e88936_d_n5;
        locals.var_t5_dn6 = assign54150_e88936_d_n6;
        locals.var_t5_dn7 = assign54150_e88936_d_n7;
        locals.var_t5_dn8 = assign54150_e88936_d_n8;
        locals.var_t5_dn9 = assign54150_e88936_d_n9;
        locals.var_t5_dn10 = assign54150_e88936_d_n10;
        locals.var_t5_dn11 = assign54150_e88936_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign54160_e88963, assign54160_e88963_d_n3, assign54160_e88963_d_n4, assign54160_e88963_d_n5, assign54160_e88963_d_n6, assign54160_e88963_d_n7, assign54160_e88963_d_n8, assign54160_e88963_d_n9, assign54160_e88963_d_n10, assign54160_e88963_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54160_e88945: f64 = (locals.var_t0a / locals.var_t1);
        let assign54160_e88948: f64 = (locals.var_t2 + locals.var_t3);
        let assign54160_e88950: f64 = (assign54160_e88948 + locals.var_t4);
        let assign54160_e88951: f64 = (assign54160_e88945 * assign54160_e88950);
        let assign54160_e88954: f64 = (locals.var_t0b / locals.var_t5);
        let assign54160_e88956: f64 = (assign54160_e88954 * locals.var_delclm);
        let assign54160_e88958: f64 = (assign54160_e88956 * locals.var_t0c);
        let assign54160_e88960: f64 = (assign54160_e88958 / locals.var_t0d);
        let assign54160_e88961: f64 = (assign54160_e88951 + assign54160_e88960);
        (assign54160_e88961, ((((((locals.var_t0a_dn3 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn3 + locals.var_t3_dn3) + locals.var_t4_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn4 + locals.var_t3_dn4) + locals.var_t4_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn5 + locals.var_t3_dn5) + locals.var_t4_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn6 + locals.var_t3_dn6) + locals.var_t4_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn7 + locals.var_t3_dn7) + locals.var_t4_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn8 + locals.var_t3_dn8) + locals.var_t4_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn9 + locals.var_t3_dn9) + locals.var_t4_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn10 + locals.var_t3_dn10) + locals.var_t4_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn11 + locals.var_t3_dn11) + locals.var_t4_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi_ch, locals.var_ssi_ch_dn3, locals.var_ssi_ch_dn4, locals.var_ssi_ch_dn5, locals.var_ssi_ch_dn6, locals.var_ssi_ch_dn7, locals.var_ssi_ch_dn8, locals.var_ssi_ch_dn9, locals.var_ssi_ch_dn10, locals.var_ssi_ch_dn11,)
    }
};
        locals.var_ssi_ch = assign54160_e88963;
        locals.var_ssi_ch_dn3 = assign54160_e88963_d_n3;
        locals.var_ssi_ch_dn4 = assign54160_e88963_d_n4;
        locals.var_ssi_ch_dn5 = assign54160_e88963_d_n5;
        locals.var_ssi_ch_dn6 = assign54160_e88963_d_n6;
        locals.var_ssi_ch_dn7 = assign54160_e88963_d_n7;
        locals.var_ssi_ch_dn8 = assign54160_e88963_d_n8;
        locals.var_ssi_ch_dn9 = assign54160_e88963_d_n9;
        locals.var_ssi_ch_dn10 = assign54160_e88963_d_n10;
        locals.var_ssi_ch_dn11 = assign54160_e88963_d_n11;
        locals.var_ssi_ch_rv = 0.0;

        let (assign54170_e88982, assign54170_e88982_d_n3, assign54170_e88982_d_n4, assign54170_e88982_d_n5, assign54170_e88982_d_n6, assign54170_e88982_d_n7, assign54170_e88982_d_n8, assign54170_e88982_d_n9, assign54170_e88982_d_n10, assign54170_e88982_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54170_e88972: f64 = (locals.var_weff * p.p2);
        let assign54170_e88974: f64 = (assign54170_e88972 * locals.var_leffnoi);
        let assign54170_e88976: f64 = (assign54170_e88974 * 10000000000.0);
        let assign54170_e88978: f64 = (assign54170_e88976 * locals.var_nstar);
        let assign54170_e88980: f64 = (assign54170_e88978 * locals.var_nstar);
        (assign54170_e88980, (((assign54170_e88976 * locals.var_nstar_dn3) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn3)), (((assign54170_e88976 * locals.var_nstar_dn4) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn4)), (((assign54170_e88976 * locals.var_nstar_dn5) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn5)), (((assign54170_e88976 * locals.var_nstar_dn6) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn6)), (((assign54170_e88976 * locals.var_nstar_dn7) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn7)), (((assign54170_e88976 * locals.var_nstar_dn8) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn8)), (((assign54170_e88976 * locals.var_nstar_dn9) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn9)), (((assign54170_e88976 * locals.var_nstar_dn10) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn10)), (((assign54170_e88976 * locals.var_nstar_dn11) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign54170_e88982;
        locals.var_t6_dn3 = assign54170_e88982_d_n3;
        locals.var_t6_dn4 = assign54170_e88982_d_n4;
        locals.var_t6_dn5 = assign54170_e88982_d_n5;
        locals.var_t6_dn6 = assign54170_e88982_d_n6;
        locals.var_t6_dn7 = assign54170_e88982_d_n7;
        locals.var_t6_dn8 = assign54170_e88982_d_n8;
        locals.var_t6_dn9 = assign54170_e88982_d_n9;
        locals.var_t6_dn10 = assign54170_e88982_d_n10;
        locals.var_t6_dn11 = assign54170_e88982_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign54180_e88997, assign54180_e88997_d_n3, assign54180_e88997_d_n4, assign54180_e88997_d_n5, assign54180_e88997_d_n6, assign54180_e88997_d_n7, assign54180_e88997_d_n8, assign54180_e88997_d_n9, assign54180_e88997_d_n10, assign54180_e88997_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54180_e88991: f64 = (locals.var_t0e / locals.var_t6);
        let assign54180_e88993: f64 = (assign54180_e88991 * locals.var_ids);
        let assign54180_e88995: f64 = (assign54180_e88993 * locals.var_ids);
        (assign54180_e88995, (((((-((locals.var_t0e * locals.var_t6_dn3) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn3)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t6) - (locals.var_t0e * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn4)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t6) - (locals.var_t0e * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn5)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn5)), (((((-((locals.var_t0e * locals.var_t6_dn6) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn6)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn6)), (((((-((locals.var_t0e * locals.var_t6_dn7) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn7)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn7)), (((((-((locals.var_t0e * locals.var_t6_dn8) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn8)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn8)), (((((-((locals.var_t0e * locals.var_t6_dn9) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn9)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn9)), (((((-((locals.var_t0e * locals.var_t6_dn10) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn10)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn10)), (((((-((locals.var_t0e * locals.var_t6_dn11) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn11)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn11)),)
    } else {
        (locals.var_swi_ch, locals.var_swi_ch_dn3, locals.var_swi_ch_dn4, locals.var_swi_ch_dn5, locals.var_swi_ch_dn6, locals.var_swi_ch_dn7, locals.var_swi_ch_dn8, locals.var_swi_ch_dn9, locals.var_swi_ch_dn10, locals.var_swi_ch_dn11,)
    }
};
        locals.var_swi_ch = assign54180_e88997;
        locals.var_swi_ch_dn3 = assign54180_e88997_d_n3;
        locals.var_swi_ch_dn4 = assign54180_e88997_d_n4;
        locals.var_swi_ch_dn5 = assign54180_e88997_d_n5;
        locals.var_swi_ch_dn6 = assign54180_e88997_d_n6;
        locals.var_swi_ch_dn7 = assign54180_e88997_d_n7;
        locals.var_swi_ch_dn8 = assign54180_e88997_d_n8;
        locals.var_swi_ch_dn9 = assign54180_e88997_d_n9;
        locals.var_swi_ch_dn10 = assign54180_e88997_d_n10;
        locals.var_swi_ch_dn11 = assign54180_e88997_d_n11;
        locals.var_swi_ch_rv = 0.0;

        let (assign54190_e89008, assign54190_e89008_d_n3, assign54190_e89008_d_n4, assign54190_e89008_d_n5, assign54190_e89008_d_n6, assign54190_e89008_d_n7, assign54190_e89008_d_n8, assign54190_e89008_d_n9, assign54190_e89008_d_n10, assign54190_e89008_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54190_e89006: f64 = (locals.var_swi_ch + locals.var_ssi_ch);
        (assign54190_e89006, (locals.var_swi_ch_dn3 + locals.var_ssi_ch_dn3), (locals.var_swi_ch_dn4 + locals.var_ssi_ch_dn4), (locals.var_swi_ch_dn5 + locals.var_ssi_ch_dn5), (locals.var_swi_ch_dn6 + locals.var_ssi_ch_dn6), (locals.var_swi_ch_dn7 + locals.var_ssi_ch_dn7), (locals.var_swi_ch_dn8 + locals.var_ssi_ch_dn8), (locals.var_swi_ch_dn9 + locals.var_ssi_ch_dn9), (locals.var_swi_ch_dn10 + locals.var_ssi_ch_dn10), (locals.var_swi_ch_dn11 + locals.var_ssi_ch_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign54190_e89008;
        locals.var_t7_dn3 = assign54190_e89008_d_n3;
        locals.var_t7_dn4 = assign54190_e89008_d_n4;
        locals.var_t7_dn5 = assign54190_e89008_d_n5;
        locals.var_t7_dn6 = assign54190_e89008_d_n6;
        locals.var_t7_dn7 = assign54190_e89008_d_n7;
        locals.var_t7_dn8 = assign54190_e89008_d_n8;
        locals.var_t7_dn9 = assign54190_e89008_d_n9;
        locals.var_t7_dn10 = assign54190_e89008_d_n10;
        locals.var_t7_dn11 = assign54190_e89008_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign54240_e89059, assign54240_e89059_d_n3, assign54240_e89059_d_n4, assign54240_e89059_d_n5, assign54240_e89059_d_n6, assign54240_e89059_d_n7, assign54240_e89059_d_n8, assign54240_e89059_d_n9, assign54240_e89059_d_n10, assign54240_e89059_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign54240_e89055: f64 = (p.p1321 * 1.602176462e-19);
        let assign54240_e89057: f64 = (assign54240_e89055 * locals.var_vt);
        (assign54240_e89057, 0.0, (assign54240_e89055 * locals.var_vt_dn4), (assign54240_e89055 * locals.var_vt_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign54240_e89059;
        locals.var_t8_dn3 = assign54240_e89059_d_n3;
        locals.var_t8_dn4 = assign54240_e89059_d_n4;
        locals.var_t8_dn5 = assign54240_e89059_d_n5;
        locals.var_t8_dn6 = assign54240_e89059_d_n6;
        locals.var_t8_dn7 = assign54240_e89059_d_n7;
        locals.var_t8_dn8 = assign54240_e89059_d_n8;
        locals.var_t8_dn9 = assign54240_e89059_d_n9;
        locals.var_t8_dn10 = assign54240_e89059_d_n10;
        locals.var_t8_dn11 = assign54240_e89059_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign54250_e89076, assign54250_e89076_d_n3, assign54250_e89076_d_n4, assign54250_e89076_d_n5, assign54250_e89076_d_n6, assign54250_e89076_d_n7, assign54250_e89076_d_n8, assign54250_e89076_d_n9, assign54250_e89076_d_n10, assign54250_e89076_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign54250_e89066: f64 = (locals.var_weff * p.p2);
        let assign54250_e89068: f64 = (assign54250_e89066 * locals.var_lh1);
        let assign54250_e89070: f64 = (assign54250_e89068 * 10000000000.0);
        let assign54250_e89072: f64 = (assign54250_e89070 * locals.var_nstar);
        let assign54250_e89074: f64 = (assign54250_e89072 * locals.var_nstar);
        (assign54250_e89074, (((assign54250_e89070 * locals.var_nstar_dn3) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn3)), (((assign54250_e89070 * locals.var_nstar_dn4) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn4)), (((assign54250_e89070 * locals.var_nstar_dn5) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn5)), (((assign54250_e89070 * locals.var_nstar_dn6) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn6)), (((assign54250_e89070 * locals.var_nstar_dn7) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn7)), (((assign54250_e89070 * locals.var_nstar_dn8) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn8)), (((assign54250_e89070 * locals.var_nstar_dn9) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn9)), (((assign54250_e89070 * locals.var_nstar_dn10) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn10)), (((assign54250_e89070 * locals.var_nstar_dn11) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn11)),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign54250_e89076;
        locals.var_t9_dn3 = assign54250_e89076_d_n3;
        locals.var_t9_dn4 = assign54250_e89076_d_n4;
        locals.var_t9_dn5 = assign54250_e89076_d_n5;
        locals.var_t9_dn6 = assign54250_e89076_d_n6;
        locals.var_t9_dn7 = assign54250_e89076_d_n7;
        locals.var_t9_dn8 = assign54250_e89076_d_n8;
        locals.var_t9_dn9 = assign54250_e89076_d_n9;
        locals.var_t9_dn10 = assign54250_e89076_d_n10;
        locals.var_t9_dn11 = assign54250_e89076_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign54260_e89089, assign54260_e89089_d_n3, assign54260_e89089_d_n4, assign54260_e89089_d_n5, assign54260_e89089_d_n6, assign54260_e89089_d_n7, assign54260_e89089_d_n8, assign54260_e89089_d_n9, assign54260_e89089_d_n10, assign54260_e89089_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign54260_e89083: f64 = (locals.var_t8 / locals.var_t9);
        let assign54260_e89085: f64 = (assign54260_e89083 * locals.var_ids);
        let assign54260_e89087: f64 = (assign54260_e89085 * locals.var_ids);
        (assign54260_e89087, (((((((locals.var_t8_dn3 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn3)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn3)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn3)), (((((((locals.var_t8_dn4 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn4)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn4)), (((((((locals.var_t8_dn5 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn5)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn5)), (((((((locals.var_t8_dn6 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn6)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn6)), (((((((locals.var_t8_dn7 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn7)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn7)), (((((((locals.var_t8_dn8 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn8)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn8)), (((((((locals.var_t8_dn9 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn9)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn9)), (((((((locals.var_t8_dn10 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn10)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn10)), (((((((locals.var_t8_dn11 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn11)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn11)),)
    } else {
        (locals.var_swi_h, locals.var_swi_h_dn3, locals.var_swi_h_dn4, locals.var_swi_h_dn5, locals.var_swi_h_dn6, locals.var_swi_h_dn7, locals.var_swi_h_dn8, locals.var_swi_h_dn9, locals.var_swi_h_dn10, locals.var_swi_h_dn11,)
    }
};
        locals.var_swi_h = assign54260_e89089;
        locals.var_swi_h_dn3 = assign54260_e89089_d_n3;
        locals.var_swi_h_dn4 = assign54260_e89089_d_n4;
        locals.var_swi_h_dn5 = assign54260_e89089_d_n5;
        locals.var_swi_h_dn6 = assign54260_e89089_d_n6;
        locals.var_swi_h_dn7 = assign54260_e89089_d_n7;
        locals.var_swi_h_dn8 = assign54260_e89089_d_n8;
        locals.var_swi_h_dn9 = assign54260_e89089_d_n9;
        locals.var_swi_h_dn10 = assign54260_e89089_d_n10;
        locals.var_swi_h_dn11 = assign54260_e89089_d_n11;
        locals.var_swi_h_rv = 0.0;

        let (assign54270_e89096, assign54270_e89096_d_n3, assign54270_e89096_d_n4, assign54270_e89096_d_n5, assign54270_e89096_d_n6, assign54270_e89096_d_n7, assign54270_e89096_d_n8, assign54270_e89096_d_n9, assign54270_e89096_d_n10, assign54270_e89096_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        (locals.var_swi_h, locals.var_swi_h_dn3, locals.var_swi_h_dn4, locals.var_swi_h_dn5, locals.var_swi_h_dn6, locals.var_swi_h_dn7, locals.var_swi_h_dn8, locals.var_swi_h_dn9, locals.var_swi_h_dn10, locals.var_swi_h_dn11,)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign54270_e89096;
        locals.var_t10_dn3 = assign54270_e89096_d_n3;
        locals.var_t10_dn4 = assign54270_e89096_d_n4;
        locals.var_t10_dn5 = assign54270_e89096_d_n5;
        locals.var_t10_dn6 = assign54270_e89096_d_n6;
        locals.var_t10_dn7 = assign54270_e89096_d_n7;
        locals.var_t10_dn8 = assign54270_e89096_d_n8;
        locals.var_t10_dn9 = assign54270_e89096_d_n9;
        locals.var_t10_dn10 = assign54270_e89096_d_n10;
        locals.var_t10_dn11 = assign54270_e89096_d_n11;
        locals.var_t10_rv = 0.0;

        let assign54320_e89135: f64 = (locals.var_leff / 2.0);
        let assign54320_e89136: f64 = if p.p1015 >= assign54320_e89135 { 1.0 } else { 0.0 };
        locals.var_guard833 = assign54320_e89136;
        locals.var_guard833_rv = 0.0;

        let (assign54330_e89146,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard833 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign54330_e89146;
        locals.var_lintnoi_i_rv = 0.0;

        let (assign54340_e89157,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard833 == 0.0)) {
        (p.p1015,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign54340_e89157;
        locals.var_lintnoi_i_rv = 0.0;

        let assign54350_e89168: f64 = if (((p.p1012 > 0.0) || (p.p1013 > 0.0)) || (p.p1014 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard834 = assign54350_e89168;
        locals.var_guard834_rv = 0.0;

        let (assign54360_e89182,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54360_e89179: f64 = (2.0 * locals.var_lintnoi_i);
        let assign54360_e89180: f64 = (locals.var_leff - assign54360_e89179);
        (assign54360_e89180,)
    } else {
        (locals.var_leffnoi,)
    }
};
        locals.var_leffnoi = assign54360_e89182;
        locals.var_leffnoi_rv = 0.0;

        let (assign54370_e89194,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54370_e89192: f64 = (locals.var_leffnoi * locals.var_leffnoi);
        (assign54370_e89192,)
    } else {
        (locals.var_leffnoisq,)
    }
};
        locals.var_leffnoisq = assign54370_e89194;
        locals.var_leffnoisq_rv = 0.0;

        let (assign54380_e89208, assign54380_e89208_d_n3, assign54380_e89208_d_n4, assign54380_e89208_d_n5, assign54380_e89208_d_n6, assign54380_e89208_d_n7, assign54380_e89208_d_n8, assign54380_e89208_d_n9, assign54380_e89208_d_n10, assign54380_e89208_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54380_e89204: f64 = (10000000000.0 * locals.var_cox);
        let assign54380_e89206: f64 = (assign54380_e89204 * locals.var_leffnoisq);
        (assign54380_e89206, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign54380_e89208;
        locals.var_t0_dn3 = assign54380_e89208_d_n3;
        locals.var_t0_dn4 = assign54380_e89208_d_n4;
        locals.var_t0_dn5 = assign54380_e89208_d_n5;
        locals.var_t0_dn6 = assign54380_e89208_d_n6;
        locals.var_t0_dn7 = assign54380_e89208_d_n7;
        locals.var_t0_dn8 = assign54380_e89208_d_n8;
        locals.var_t0_dn9 = assign54380_e89208_d_n9;
        locals.var_t0_dn10 = assign54380_e89208_d_n10;
        locals.var_t0_dn11 = assign54380_e89208_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign54390_e89232, assign54390_e89232_d_n3, assign54390_e89232_d_n4, assign54390_e89232_d_n5, assign54390_e89232_d_n6, assign54390_e89232_d_n7, assign54390_e89232_d_n8, assign54390_e89232_d_n9, assign54390_e89232_d_n10, assign54390_e89232_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54390_e89218: f64 = (2.0 * locals.var_nq);
        let assign54390_e89220: f64 = (assign54390_e89218 * locals.var_cox);
        let assign54390_e89222: f64 = (assign54390_e89220 * locals.var_vt);
        let assign54390_e89224: f64 = (assign54390_e89222 * locals.var_qs_1);
        let assign54390_e89226: f64 = (assign54390_e89224 * locals.var_mnud1);
        let assign54390_e89228: f64 = (assign54390_e89226 * locals.var_mnud);
        let assign54390_e89230: f64 = (assign54390_e89228 / 1.602176462e-19);
        (assign54390_e89230, ((((((((((2.0 * locals.var_nq_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn3)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn3)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn3)) / 1.602176462e-19), (((((((((((2.0 * locals.var_nq_dn4) * locals.var_cox) * locals.var_vt) + (assign54390_e89220 * locals.var_vt_dn4)) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn4)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn4)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn4)) / 1.602176462e-19), (((((((((((2.0 * locals.var_nq_dn5) * locals.var_cox) * locals.var_vt) + (assign54390_e89220 * locals.var_vt_dn5)) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn5)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn5)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn5)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn6)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn6)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn6)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn7)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn7)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn7)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn8)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn8)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn8)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn9)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn9)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn9)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn10)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn10)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn10)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn11)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn11)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn11)) / 1.602176462e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11,)
    }
};
        locals.var_n0 = assign54390_e89232;
        locals.var_n0_dn3 = assign54390_e89232_d_n3;
        locals.var_n0_dn4 = assign54390_e89232_d_n4;
        locals.var_n0_dn5 = assign54390_e89232_d_n5;
        locals.var_n0_dn6 = assign54390_e89232_d_n6;
        locals.var_n0_dn7 = assign54390_e89232_d_n7;
        locals.var_n0_dn8 = assign54390_e89232_d_n8;
        locals.var_n0_dn9 = assign54390_e89232_d_n9;
        locals.var_n0_dn10 = assign54390_e89232_d_n10;
        locals.var_n0_dn11 = assign54390_e89232_d_n11;
        locals.var_n0_rv = 0.0;

        let (assign54400_e89253, assign54400_e89253_d_n3, assign54400_e89253_d_n4, assign54400_e89253_d_n5, assign54400_e89253_d_n6, assign54400_e89253_d_n7, assign54400_e89253_d_n8, assign54400_e89253_d_n9, assign54400_e89253_d_n10, assign54400_e89253_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54400_e89243: f64 = (locals.var_n0 + locals.var_nstar);
        let assign54400_e89246: f64 = (locals.var_nl + locals.var_nstar);
        let assign54400_e89247: f64 = (assign54400_e89243 / assign54400_e89246);
        let assign54400_e89249: f64 = (assign54400_e89247).max(1e-38);
        let assign54400_e89250: f64 = (assign54400_e89249).ln();
        let assign54400_e89251: f64 = (p.p1012 * assign54400_e89250);
        (assign54400_e89251, (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign54400_e89253;
        locals.var_t1_dn3 = assign54400_e89253_d_n3;
        locals.var_t1_dn4 = assign54400_e89253_d_n4;
        locals.var_t1_dn5 = assign54400_e89253_d_n5;
        locals.var_t1_dn6 = assign54400_e89253_d_n6;
        locals.var_t1_dn7 = assign54400_e89253_d_n7;
        locals.var_t1_dn8 = assign54400_e89253_d_n8;
        locals.var_t1_dn9 = assign54400_e89253_d_n9;
        locals.var_t1_dn10 = assign54400_e89253_d_n10;
        locals.var_t1_dn11 = assign54400_e89253_d_n11;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_187(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign54410_e89267, assign54410_e89267_d_n3, assign54410_e89267_d_n4, assign54410_e89267_d_n5, assign54410_e89267_d_n6, assign54410_e89267_d_n7, assign54410_e89267_d_n8, assign54410_e89267_d_n9, assign54410_e89267_d_n10, assign54410_e89267_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54410_e89264: f64 = (locals.var_n0 - locals.var_nl);
        let assign54410_e89265: f64 = (p.p1013 * assign54410_e89264);
        (assign54410_e89265, (p.p1013 * (locals.var_n0_dn3 - locals.var_nl_dn3)), (p.p1013 * (locals.var_n0_dn4 - locals.var_nl_dn4)), (p.p1013 * (locals.var_n0_dn5 - locals.var_nl_dn5)), (p.p1013 * (locals.var_n0_dn6 - locals.var_nl_dn6)), (p.p1013 * (locals.var_n0_dn7 - locals.var_nl_dn7)), (p.p1013 * (locals.var_n0_dn8 - locals.var_nl_dn8)), (p.p1013 * (locals.var_n0_dn9 - locals.var_nl_dn9)), (p.p1013 * (locals.var_n0_dn10 - locals.var_nl_dn10)), (p.p1013 * (locals.var_n0_dn11 - locals.var_nl_dn11)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign54410_e89267;
        locals.var_t2_dn3 = assign54410_e89267_d_n3;
        locals.var_t2_dn4 = assign54410_e89267_d_n4;
        locals.var_t2_dn5 = assign54410_e89267_d_n5;
        locals.var_t2_dn6 = assign54410_e89267_d_n6;
        locals.var_t2_dn7 = assign54410_e89267_d_n7;
        locals.var_t2_dn8 = assign54410_e89267_d_n8;
        locals.var_t2_dn9 = assign54410_e89267_d_n9;
        locals.var_t2_dn10 = assign54410_e89267_d_n10;
        locals.var_t2_dn11 = assign54410_e89267_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign54420_e89287, assign54420_e89287_d_n3, assign54420_e89287_d_n4, assign54420_e89287_d_n5, assign54420_e89287_d_n6, assign54420_e89287_d_n7, assign54420_e89287_d_n8, assign54420_e89287_d_n9, assign54420_e89287_d_n10, assign54420_e89287_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54420_e89277: f64 = (0.5 * p.p1014);
        let assign54420_e89280: f64 = (locals.var_n0 * locals.var_n0);
        let assign54420_e89283: f64 = (locals.var_nl * locals.var_nl);
        let assign54420_e89284: f64 = (assign54420_e89280 - assign54420_e89283);
        let assign54420_e89285: f64 = (assign54420_e89277 * assign54420_e89284);
        (assign54420_e89285, (assign54420_e89277 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign54420_e89277 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign54420_e89277 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign54420_e89277 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign54420_e89277 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign54420_e89277 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign54420_e89277 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign54420_e89277 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign54420_e89277 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign54420_e89287;
        locals.var_t3_dn3 = assign54420_e89287_d_n3;
        locals.var_t3_dn4 = assign54420_e89287_d_n4;
        locals.var_t3_dn5 = assign54420_e89287_d_n5;
        locals.var_t3_dn6 = assign54420_e89287_d_n6;
        locals.var_t3_dn7 = assign54420_e89287_d_n7;
        locals.var_t3_dn8 = assign54420_e89287_d_n8;
        locals.var_t3_dn9 = assign54420_e89287_d_n9;
        locals.var_t3_dn10 = assign54420_e89287_d_n10;
        locals.var_t3_dn11 = assign54420_e89287_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign54430_e89303, assign54430_e89303_d_n3, assign54430_e89303_d_n4, assign54430_e89303_d_n5, assign54430_e89303_d_n6, assign54430_e89303_d_n7, assign54430_e89303_d_n8, assign54430_e89303_d_n9, assign54430_e89303_d_n10, assign54430_e89303_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54430_e89297: f64 = (10000000000.0 * locals.var_leffnoisq);
        let assign54430_e89299: f64 = (assign54430_e89297 * locals.var_weff);
        let assign54430_e89301: f64 = (assign54430_e89299 * p.p2);
        (assign54430_e89301, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign54430_e89303;
        locals.var_t4_dn3 = assign54430_e89303_d_n3;
        locals.var_t4_dn4 = assign54430_e89303_d_n4;
        locals.var_t4_dn5 = assign54430_e89303_d_n5;
        locals.var_t4_dn6 = assign54430_e89303_d_n6;
        locals.var_t4_dn7 = assign54430_e89303_d_n7;
        locals.var_t4_dn8 = assign54430_e89303_d_n8;
        locals.var_t4_dn9 = assign54430_e89303_d_n9;
        locals.var_t4_dn10 = assign54430_e89303_d_n10;
        locals.var_t4_dn11 = assign54430_e89303_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign54440_e89331, assign54440_e89331_d_n3, assign54440_e89331_d_n4, assign54440_e89331_d_n5, assign54440_e89331_d_n6, assign54440_e89331_d_n7, assign54440_e89331_d_n8, assign54440_e89331_d_n9, assign54440_e89331_d_n10, assign54440_e89331_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54440_e89313: f64 = (locals.var_t0a / locals.var_t0);
        let assign54440_e89316: f64 = (locals.var_t1 + locals.var_t2);
        let assign54440_e89318: f64 = (assign54440_e89316 + locals.var_t3);
        let assign54440_e89319: f64 = (assign54440_e89313 * assign54440_e89318);
        let assign54440_e89322: f64 = (locals.var_t0b / locals.var_t4);
        let assign54440_e89324: f64 = (assign54440_e89322 * locals.var_delclm);
        let assign54440_e89326: f64 = (assign54440_e89324 * locals.var_t0c);
        let assign54440_e89328: f64 = (assign54440_e89326 / locals.var_t0d);
        let assign54440_e89329: f64 = (assign54440_e89319 + assign54440_e89328);
        (assign54440_e89329, ((((((locals.var_t0a_dn3 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn3 + locals.var_t2_dn3) + locals.var_t3_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn4 + locals.var_t2_dn4) + locals.var_t3_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn5 + locals.var_t2_dn5) + locals.var_t3_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn6 + locals.var_t2_dn6) + locals.var_t3_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn7 + locals.var_t2_dn7) + locals.var_t3_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn8 + locals.var_t2_dn8) + locals.var_t3_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn9 + locals.var_t2_dn9) + locals.var_t3_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn10 + locals.var_t2_dn10) + locals.var_t3_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn11 + locals.var_t2_dn11) + locals.var_t3_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11,)
    }
};
        locals.var_ssi = assign54440_e89331;
        locals.var_ssi_dn3 = assign54440_e89331_d_n3;
        locals.var_ssi_dn4 = assign54440_e89331_d_n4;
        locals.var_ssi_dn5 = assign54440_e89331_d_n5;
        locals.var_ssi_dn6 = assign54440_e89331_d_n6;
        locals.var_ssi_dn7 = assign54440_e89331_d_n7;
        locals.var_ssi_dn8 = assign54440_e89331_d_n8;
        locals.var_ssi_dn9 = assign54440_e89331_d_n9;
        locals.var_ssi_dn10 = assign54440_e89331_d_n10;
        locals.var_ssi_dn11 = assign54440_e89331_d_n11;
        locals.var_ssi_rv = 0.0;

        let (assign54450_e89351, assign54450_e89351_d_n3, assign54450_e89351_d_n4, assign54450_e89351_d_n5, assign54450_e89351_d_n6, assign54450_e89351_d_n7, assign54450_e89351_d_n8, assign54450_e89351_d_n9, assign54450_e89351_d_n10, assign54450_e89351_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54450_e89341: f64 = (locals.var_weff * p.p2);
        let assign54450_e89343: f64 = (assign54450_e89341 * locals.var_leffnoi);
        let assign54450_e89345: f64 = (assign54450_e89343 * 10000000000.0);
        let assign54450_e89347: f64 = (assign54450_e89345 * locals.var_nstar);
        let assign54450_e89349: f64 = (assign54450_e89347 * locals.var_nstar);
        (assign54450_e89349, (((assign54450_e89345 * locals.var_nstar_dn3) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn3)), (((assign54450_e89345 * locals.var_nstar_dn4) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn4)), (((assign54450_e89345 * locals.var_nstar_dn5) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn5)), (((assign54450_e89345 * locals.var_nstar_dn6) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn6)), (((assign54450_e89345 * locals.var_nstar_dn7) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn7)), (((assign54450_e89345 * locals.var_nstar_dn8) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn8)), (((assign54450_e89345 * locals.var_nstar_dn9) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn9)), (((assign54450_e89345 * locals.var_nstar_dn10) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn10)), (((assign54450_e89345 * locals.var_nstar_dn11) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign54450_e89351;
        locals.var_t5_dn3 = assign54450_e89351_d_n3;
        locals.var_t5_dn4 = assign54450_e89351_d_n4;
        locals.var_t5_dn5 = assign54450_e89351_d_n5;
        locals.var_t5_dn6 = assign54450_e89351_d_n6;
        locals.var_t5_dn7 = assign54450_e89351_d_n7;
        locals.var_t5_dn8 = assign54450_e89351_d_n8;
        locals.var_t5_dn9 = assign54450_e89351_d_n9;
        locals.var_t5_dn10 = assign54450_e89351_d_n10;
        locals.var_t5_dn11 = assign54450_e89351_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign54460_e89367, assign54460_e89367_d_n3, assign54460_e89367_d_n4, assign54460_e89367_d_n5, assign54460_e89367_d_n6, assign54460_e89367_d_n7, assign54460_e89367_d_n8, assign54460_e89367_d_n9, assign54460_e89367_d_n10, assign54460_e89367_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54460_e89361: f64 = (locals.var_t0e / locals.var_t5);
        let assign54460_e89363: f64 = (assign54460_e89361 * locals.var_ids);
        let assign54460_e89365: f64 = (assign54460_e89363 * locals.var_ids);
        (assign54460_e89365, (((((-((locals.var_t0e * locals.var_t5_dn3) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn3)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn4)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn5)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn5)), (((((-((locals.var_t0e * locals.var_t5_dn6) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn6)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn6)), (((((-((locals.var_t0e * locals.var_t5_dn7) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn7)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn7)), (((((-((locals.var_t0e * locals.var_t5_dn8) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn8)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn8)), (((((-((locals.var_t0e * locals.var_t5_dn9) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn9)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn9)), (((((-((locals.var_t0e * locals.var_t5_dn10) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn10)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn10)), (((((-((locals.var_t0e * locals.var_t5_dn11) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn11)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn11)),)
    } else {
        (locals.var_swi, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11,)
    }
};
        locals.var_swi = assign54460_e89367;
        locals.var_swi_dn3 = assign54460_e89367_d_n3;
        locals.var_swi_dn4 = assign54460_e89367_d_n4;
        locals.var_swi_dn5 = assign54460_e89367_d_n5;
        locals.var_swi_dn6 = assign54460_e89367_d_n6;
        locals.var_swi_dn7 = assign54460_e89367_d_n7;
        locals.var_swi_dn8 = assign54460_e89367_d_n8;
        locals.var_swi_dn9 = assign54460_e89367_d_n9;
        locals.var_swi_dn10 = assign54460_e89367_d_n10;
        locals.var_swi_dn11 = assign54460_e89367_d_n11;
        locals.var_swi_rv = 0.0;

        let (assign54470_e89379, assign54470_e89379_d_n3, assign54470_e89379_d_n4, assign54470_e89379_d_n5, assign54470_e89379_d_n6, assign54470_e89379_d_n7, assign54470_e89379_d_n8, assign54470_e89379_d_n9, assign54470_e89379_d_n10, assign54470_e89379_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54470_e89377: f64 = (locals.var_swi + locals.var_ssi);
        (assign54470_e89377, (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign54470_e89379;
        locals.var_t6_dn3 = assign54470_e89379_d_n3;
        locals.var_t6_dn4 = assign54470_e89379_d_n4;
        locals.var_t6_dn5 = assign54470_e89379_d_n5;
        locals.var_t6_dn6 = assign54470_e89379_d_n6;
        locals.var_t6_dn7 = assign54470_e89379_d_n7;
        locals.var_t6_dn8 = assign54470_e89379_d_n8;
        locals.var_t6_dn9 = assign54470_e89379_d_n9;
        locals.var_t6_dn10 = assign54470_e89379_d_n10;
        locals.var_t6_dn11 = assign54470_e89379_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign54520_e89441, assign54520_e89441_d_n3, assign54520_e89441_d_n4, assign54520_e89441_d_n5, assign54520_e89441_d_n6, assign54520_e89441_d_n7, assign54520_e89441_d_n8, assign54520_e89441_d_n9, assign54520_e89441_d_n10, assign54520_e89441_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54520_e89437: f64 = (locals.var_qia / locals.var_esatnoi);
        let assign54520_e89439: f64 = (assign54520_e89437 / locals.var_leff);
        (assign54520_e89439, ((((locals.var_qia_dn3 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn3)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn4 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn4)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn5 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn5)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn6 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn6)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn7 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn7)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn8 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn8)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn9 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn9)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn10 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn10)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn11 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn11)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign54520_e89441;
        locals.var_t0_dn3 = assign54520_e89441_d_n3;
        locals.var_t0_dn4 = assign54520_e89441_d_n4;
        locals.var_t0_dn5 = assign54520_e89441_d_n5;
        locals.var_t0_dn6 = assign54520_e89441_d_n6;
        locals.var_t0_dn7 = assign54520_e89441_d_n7;
        locals.var_t0_dn8 = assign54520_e89441_d_n8;
        locals.var_t0_dn9 = assign54520_e89441_d_n9;
        locals.var_t0_dn10 = assign54520_e89441_d_n10;
        locals.var_t0_dn11 = assign54520_e89441_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign54530_e89448, assign54530_e89448_d_n3, assign54530_e89448_d_n4, assign54530_e89448_d_n5, assign54530_e89448_d_n6, assign54530_e89448_d_n7, assign54530_e89448_d_n8, assign54530_e89448_d_n9, assign54530_e89448_d_n10, assign54530_e89448_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54530_e89446: f64 = (locals.var_t0 * locals.var_t0);
        (assign54530_e89446, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign54530_e89448;
        locals.var_t1_dn3 = assign54530_e89448_d_n3;
        locals.var_t1_dn4 = assign54530_e89448_d_n4;
        locals.var_t1_dn5 = assign54530_e89448_d_n5;
        locals.var_t1_dn6 = assign54530_e89448_d_n6;
        locals.var_t1_dn7 = assign54530_e89448_d_n7;
        locals.var_t1_dn8 = assign54530_e89448_d_n8;
        locals.var_t1_dn9 = assign54530_e89448_d_n9;
        locals.var_t1_dn10 = assign54530_e89448_d_n10;
        locals.var_t1_dn11 = assign54530_e89448_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign54540_e89461, assign54540_e89461_d_n3, assign54540_e89461_d_n4, assign54540_e89461_d_n5, assign54540_e89461_d_n6, assign54540_e89461_d_n7, assign54540_e89461_d_n8, assign54540_e89461_d_n9, assign54540_e89461_d_n10, assign54540_e89461_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54540_e89455: f64 = (p.p1022 * locals.var_leff);
        let assign54540_e89457: f64 = (assign54540_e89455 * locals.var_t1);
        let assign54540_e89458: f64 = (1.0 + assign54540_e89457);
        let assign54540_e89459: f64 = (p.p1019 * assign54540_e89458);
        (assign54540_e89459, (p.p1019 * (assign54540_e89455 * locals.var_t1_dn3)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn4)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn5)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn6)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn7)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn8)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn9)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn10)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign54540_e89461;
        locals.var_t3_dn3 = assign54540_e89461_d_n3;
        locals.var_t3_dn4 = assign54540_e89461_d_n4;
        locals.var_t3_dn5 = assign54540_e89461_d_n5;
        locals.var_t3_dn6 = assign54540_e89461_d_n6;
        locals.var_t3_dn7 = assign54540_e89461_d_n7;
        locals.var_t3_dn8 = assign54540_e89461_d_n8;
        locals.var_t3_dn9 = assign54540_e89461_d_n9;
        locals.var_t3_dn10 = assign54540_e89461_d_n10;
        locals.var_t3_dn11 = assign54540_e89461_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign54550_e89474, assign54550_e89474_d_n3, assign54550_e89474_d_n4, assign54550_e89474_d_n5, assign54550_e89474_d_n6, assign54550_e89474_d_n7, assign54550_e89474_d_n8, assign54550_e89474_d_n9, assign54550_e89474_d_n10, assign54550_e89474_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54550_e89468: f64 = (p.p1023 * locals.var_leff);
        let assign54550_e89470: f64 = (assign54550_e89468 * locals.var_t1);
        let assign54550_e89471: f64 = (1.0 + assign54550_e89470);
        let assign54550_e89472: f64 = (p.p1020 * assign54550_e89471);
        (assign54550_e89472, (p.p1020 * (assign54550_e89468 * locals.var_t1_dn3)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn4)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn5)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn6)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn7)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn8)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn9)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn10)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn11)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign54550_e89474;
        locals.var_t4_dn3 = assign54550_e89474_d_n3;
        locals.var_t4_dn4 = assign54550_e89474_d_n4;
        locals.var_t4_dn5 = assign54550_e89474_d_n5;
        locals.var_t4_dn6 = assign54550_e89474_d_n6;
        locals.var_t4_dn7 = assign54550_e89474_d_n7;
        locals.var_t4_dn8 = assign54550_e89474_d_n8;
        locals.var_t4_dn9 = assign54550_e89474_d_n9;
        locals.var_t4_dn10 = assign54550_e89474_d_n10;
        locals.var_t4_dn11 = assign54550_e89474_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign54560_e89487, assign54560_e89487_d_n3, assign54560_e89487_d_n4, assign54560_e89487_d_n5, assign54560_e89487_d_n6, assign54560_e89487_d_n7, assign54560_e89487_d_n8, assign54560_e89487_d_n9, assign54560_e89487_d_n10, assign54560_e89487_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54560_e89481: f64 = (p.p1298 * locals.var_leff);
        let assign54560_e89483: f64 = (assign54560_e89481 * locals.var_t1);
        let assign54560_e89484: f64 = (1.0 + assign54560_e89483);
        let assign54560_e89485: f64 = (p.p1297 * assign54560_e89484);
        (assign54560_e89485, (p.p1297 * (assign54560_e89481 * locals.var_t1_dn3)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn4)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn5)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn6)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn7)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn8)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn9)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn10)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign54560_e89487;
        locals.var_t5_dn3 = assign54560_e89487_d_n3;
        locals.var_t5_dn4 = assign54560_e89487_d_n4;
        locals.var_t5_dn5 = assign54560_e89487_d_n5;
        locals.var_t5_dn6 = assign54560_e89487_d_n6;
        locals.var_t5_dn7 = assign54560_e89487_d_n7;
        locals.var_t5_dn8 = assign54560_e89487_d_n8;
        locals.var_t5_dn9 = assign54560_e89487_d_n9;
        locals.var_t5_dn10 = assign54560_e89487_d_n10;
        locals.var_t5_dn11 = assign54560_e89487_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign54600_e89531, assign54600_e89531_d_n3, assign54600_e89531_d_n4, assign54600_e89531_d_n5, assign54600_e89531_d_n6, assign54600_e89531_d_n7, assign54600_e89531_d_n8, assign54600_e89531_d_n9, assign54600_e89531_d_n10, assign54600_e89531_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54600_e89529: f64 = (locals.var_t5 * locals.var_t5);
        (assign54600_e89529, ((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)),)
    } else {
        (locals.var_betalowid, locals.var_betalowid_dn3, locals.var_betalowid_dn4, locals.var_betalowid_dn5, locals.var_betalowid_dn6, locals.var_betalowid_dn7, locals.var_betalowid_dn8, locals.var_betalowid_dn9, locals.var_betalowid_dn10, locals.var_betalowid_dn11,)
    }
};
        locals.var_betalowid = assign54600_e89531;
        locals.var_betalowid_dn3 = assign54600_e89531_d_n3;
        locals.var_betalowid_dn4 = assign54600_e89531_d_n4;
        locals.var_betalowid_dn5 = assign54600_e89531_d_n5;
        locals.var_betalowid_dn6 = assign54600_e89531_d_n6;
        locals.var_betalowid_dn7 = assign54600_e89531_d_n7;
        locals.var_betalowid_dn8 = assign54600_e89531_d_n8;
        locals.var_betalowid_dn9 = assign54600_e89531_d_n9;
        locals.var_betalowid_dn10 = assign54600_e89531_d_n10;
        locals.var_betalowid_dn11 = assign54600_e89531_d_n11;
        locals.var_betalowid_rv = 0.0;

        let (assign54610_e89538, assign54610_e89538_d_n3, assign54610_e89538_d_n4, assign54610_e89538_d_n5, assign54610_e89538_d_n6, assign54610_e89538_d_n7, assign54610_e89538_d_n8, assign54610_e89538_d_n9, assign54610_e89538_d_n10, assign54610_e89538_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54610_e89536: f64 = (locals.var_t4 * locals.var_t4);
        (assign54610_e89536, ((locals.var_t4_dn3 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn3)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)),)
    } else {
        (locals.var_thetanoisq, locals.var_thetanoisq_dn3, locals.var_thetanoisq_dn4, locals.var_thetanoisq_dn5, locals.var_thetanoisq_dn6, locals.var_thetanoisq_dn7, locals.var_thetanoisq_dn8, locals.var_thetanoisq_dn9, locals.var_thetanoisq_dn10, locals.var_thetanoisq_dn11,)
    }
};
        locals.var_thetanoisq = assign54610_e89538;
        locals.var_thetanoisq_dn3 = assign54610_e89538_d_n3;
        locals.var_thetanoisq_dn4 = assign54610_e89538_d_n4;
        locals.var_thetanoisq_dn5 = assign54610_e89538_d_n5;
        locals.var_thetanoisq_dn6 = assign54610_e89538_d_n6;
        locals.var_thetanoisq_dn7 = assign54610_e89538_d_n7;
        locals.var_thetanoisq_dn8 = assign54610_e89538_d_n8;
        locals.var_thetanoisq_dn9 = assign54610_e89538_d_n9;
        locals.var_thetanoisq_dn10 = assign54610_e89538_d_n10;
        locals.var_thetanoisq_dn11 = assign54610_e89538_d_n11;
        locals.var_thetanoisq_rv = 0.0;

        let assign54630_e89546: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard836 = assign54630_e89546;
        locals.var_guard836_rv = 0.0;

        let assign54640_e89549: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard837 = assign54640_e89549;
        locals.var_guard837_rv = 0.0;

        let (assign54650_e89567, assign54650_e89567_d_n3, assign54650_e89567_d_n4, assign54650_e89567_d_n5, assign54650_e89567_d_n6, assign54650_e89567_d_n7, assign54650_e89567_d_n8, assign54650_e89567_d_n9, assign54650_e89567_d_n10, assign54650_e89567_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard836 != 0.0)) {
        let assign54650_e89555: f64 = (-p.p2);
        let assign54650_e89557: f64 = (assign54650_e89555 * locals.var_weff);
        let assign54650_e89559: f64 = (assign54650_e89557 * locals.var_leff);
        let assign54650_e89561: f64 = (assign54650_e89559 * locals.var_cox);
        let assign54650_e89563: f64 = (assign54650_e89561 * locals.var_vt);
        let assign54650_e89565: f64 = (assign54650_e89563 * locals.var_qs);
        (assign54650_e89565, (assign54650_e89563 * locals.var_qs_dn3), (((assign54650_e89561 * locals.var_vt_dn4) * locals.var_qs) + (assign54650_e89563 * locals.var_qs_dn4)), (((assign54650_e89561 * locals.var_vt_dn5) * locals.var_qs) + (assign54650_e89563 * locals.var_qs_dn5)), (assign54650_e89563 * locals.var_qs_dn6), (assign54650_e89563 * locals.var_qs_dn7), (assign54650_e89563 * locals.var_qs_dn8), (assign54650_e89563 * locals.var_qs_dn9), (assign54650_e89563 * locals.var_qs_dn10), (assign54650_e89563 * locals.var_qs_dn11),)
    } else {
        (locals.var_qsi, locals.var_qsi_dn3, locals.var_qsi_dn4, locals.var_qsi_dn5, locals.var_qsi_dn6, locals.var_qsi_dn7, locals.var_qsi_dn8, locals.var_qsi_dn9, locals.var_qsi_dn10, locals.var_qsi_dn11,)
    }
};
        locals.var_qsi = assign54650_e89567;
        locals.var_qsi_dn3 = assign54650_e89567_d_n3;
        locals.var_qsi_dn4 = assign54650_e89567_d_n4;
        locals.var_qsi_dn5 = assign54650_e89567_d_n5;
        locals.var_qsi_dn6 = assign54650_e89567_d_n6;
        locals.var_qsi_dn7 = assign54650_e89567_d_n7;
        locals.var_qsi_dn8 = assign54650_e89567_d_n8;
        locals.var_qsi_dn9 = assign54650_e89567_d_n9;
        locals.var_qsi_dn10 = assign54650_e89567_d_n10;
        locals.var_qsi_dn11 = assign54650_e89567_d_n11;
        locals.var_qsi_rv = 0.0;

        let (assign54660_e89585, assign54660_e89585_d_n3, assign54660_e89585_d_n4, assign54660_e89585_d_n5, assign54660_e89585_d_n6, assign54660_e89585_d_n7, assign54660_e89585_d_n8, assign54660_e89585_d_n9, assign54660_e89585_d_n10, assign54660_e89585_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard836 != 0.0)) {
        let assign54660_e89573: f64 = (-p.p2);
        let assign54660_e89575: f64 = (assign54660_e89573 * locals.var_weff);
        let assign54660_e89577: f64 = (assign54660_e89575 * locals.var_leff);
        let assign54660_e89579: f64 = (assign54660_e89577 * locals.var_cox);
        let assign54660_e89581: f64 = (assign54660_e89579 * locals.var_vt);
        let assign54660_e89583: f64 = (assign54660_e89581 * locals.var_qd);
        (assign54660_e89583, (assign54660_e89581 * locals.var_qd_dn3), (((assign54660_e89579 * locals.var_vt_dn4) * locals.var_qd) + (assign54660_e89581 * locals.var_qd_dn4)), (((assign54660_e89579 * locals.var_vt_dn5) * locals.var_qd) + (assign54660_e89581 * locals.var_qd_dn5)), (assign54660_e89581 * locals.var_qd_dn6), (assign54660_e89581 * locals.var_qd_dn7), (assign54660_e89581 * locals.var_qd_dn8), (assign54660_e89581 * locals.var_qd_dn9), (assign54660_e89581 * locals.var_qd_dn10), (assign54660_e89581 * locals.var_qd_dn11),)
    } else {
        (locals.var_qdi, locals.var_qdi_dn3, locals.var_qdi_dn4, locals.var_qdi_dn5, locals.var_qdi_dn6, locals.var_qdi_dn7, locals.var_qdi_dn8, locals.var_qdi_dn9, locals.var_qdi_dn10, locals.var_qdi_dn11,)
    }
};
        locals.var_qdi = assign54660_e89585;
        locals.var_qdi_dn3 = assign54660_e89585_d_n3;
        locals.var_qdi_dn4 = assign54660_e89585_d_n4;
        locals.var_qdi_dn5 = assign54660_e89585_d_n5;
        locals.var_qdi_dn6 = assign54660_e89585_d_n6;
        locals.var_qdi_dn7 = assign54660_e89585_d_n7;
        locals.var_qdi_dn8 = assign54660_e89585_d_n8;
        locals.var_qdi_dn9 = assign54660_e89585_d_n9;
        locals.var_qdi_dn10 = assign54660_e89585_d_n10;
        locals.var_qdi_dn11 = assign54660_e89585_d_n11;
        locals.var_qdi_rv = 0.0;

        let (assign54670_e89597, assign54670_e89597_d_n3, assign54670_e89597_d_n4, assign54670_e89597_d_n5, assign54670_e89597_d_n6, assign54670_e89597_d_n7, assign54670_e89597_d_n8, assign54670_e89597_d_n9, assign54670_e89597_d_n10, assign54670_e89597_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard836 != 0.0)) {
        let assign54670_e89593: f64 = (locals.var_qsi + locals.var_qdi);
        let assign54670_e89594: f64 = (assign54670_e89593).abs();
        let assign54670_e89595: f64 = (locals.var_ueff * assign54670_e89594);
        (assign54670_e89595, ((locals.var_ueff_dn3 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn3 + locals.var_qdi_dn3) } else { (-(locals.var_qsi_dn3 + locals.var_qdi_dn3)) })), ((locals.var_ueff_dn4 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn4 + locals.var_qdi_dn4) } else { (-(locals.var_qsi_dn4 + locals.var_qdi_dn4)) })), ((locals.var_ueff_dn5 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn5 + locals.var_qdi_dn5) } else { (-(locals.var_qsi_dn5 + locals.var_qdi_dn5)) })), ((locals.var_ueff_dn6 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn6 + locals.var_qdi_dn6) } else { (-(locals.var_qsi_dn6 + locals.var_qdi_dn6)) })), ((locals.var_ueff_dn7 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn7 + locals.var_qdi_dn7) } else { (-(locals.var_qsi_dn7 + locals.var_qdi_dn7)) })), ((locals.var_ueff_dn8 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn8 + locals.var_qdi_dn8) } else { (-(locals.var_qsi_dn8 + locals.var_qdi_dn8)) })), ((locals.var_ueff_dn9 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn9 + locals.var_qdi_dn9) } else { (-(locals.var_qsi_dn9 + locals.var_qdi_dn9)) })), ((locals.var_ueff_dn10 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn10 + locals.var_qdi_dn10) } else { (-(locals.var_qsi_dn10 + locals.var_qdi_dn10)) })), ((locals.var_ueff_dn11 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn11 + locals.var_qdi_dn11) } else { (-(locals.var_qsi_dn11 + locals.var_qdi_dn11)) })),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign54670_e89597;
        locals.var_t0_dn3 = assign54670_e89597_d_n3;
        locals.var_t0_dn4 = assign54670_e89597_d_n4;
        locals.var_t0_dn5 = assign54670_e89597_d_n5;
        locals.var_t0_dn6 = assign54670_e89597_d_n6;
        locals.var_t0_dn7 = assign54670_e89597_d_n7;
        locals.var_t0_dn8 = assign54670_e89597_d_n8;
        locals.var_t0_dn9 = assign54670_e89597_d_n9;
        locals.var_t0_dn10 = assign54670_e89597_d_n10;
        locals.var_t0_dn11 = assign54670_e89597_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign54680_e89610, assign54680_e89610_d_n3, assign54680_e89610_d_n4, assign54680_e89610_d_n5, assign54680_e89610_d_n6, assign54680_e89610_d_n7, assign54680_e89610_d_n8, assign54680_e89610_d_n9, assign54680_e89610_d_n10, assign54680_e89610_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard836 != 0.0)) {
        let assign54680_e89604: f64 = (locals.var_t0 * locals.var_rdsi);
        let assign54680_e89607: f64 = (locals.var_leff * locals.var_leff);
        let assign54680_e89608: f64 = (assign54680_e89604 + assign54680_e89607);
        (assign54680_e89608, ((locals.var_t0_dn3 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn3)), ((locals.var_t0_dn4 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn4)), ((locals.var_t0_dn5 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn5)), ((locals.var_t0_dn6 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn6)), ((locals.var_t0_dn7 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn7)), ((locals.var_t0_dn8 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn8)), ((locals.var_t0_dn9 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn9)), ((locals.var_t0_dn10 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn10)), ((locals.var_t0_dn11 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign54680_e89610;
        locals.var_t1_dn3 = assign54680_e89610_d_n3;
        locals.var_t1_dn4 = assign54680_e89610_d_n4;
        locals.var_t1_dn5 = assign54680_e89610_d_n5;
        locals.var_t1_dn6 = assign54680_e89610_d_n6;
        locals.var_t1_dn7 = assign54680_e89610_d_n7;
        locals.var_t1_dn8 = assign54680_e89610_d_n8;
        locals.var_t1_dn9 = assign54680_e89610_d_n9;
        locals.var_t1_dn10 = assign54680_e89610_d_n10;
        locals.var_t1_dn11 = assign54680_e89610_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign54710_e89644, assign54710_e89644_d_n3, assign54710_e89644_d_n4, assign54710_e89644_d_n5, assign54710_e89644_d_n6, assign54710_e89644_d_n7, assign54710_e89644_d_n8, assign54710_e89644_d_n9, assign54710_e89644_d_n10, assign54710_e89644_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54710_e89640: f64 = (2.0 * locals.var_nq);
        let assign54710_e89642: f64 = (assign54710_e89640 * locals.var_nvt);
        (assign54710_e89642, (((2.0 * locals.var_nq_dn3) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn3)), (((2.0 * locals.var_nq_dn4) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn4)), (((2.0 * locals.var_nq_dn5) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn5)), (((2.0 * locals.var_nq_dn6) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn6)), (((2.0 * locals.var_nq_dn7) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn7)), (((2.0 * locals.var_nq_dn8) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn8)), (((2.0 * locals.var_nq_dn9) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn9)), (((2.0 * locals.var_nq_dn10) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn10)), (((2.0 * locals.var_nq_dn11) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn11)),)
    } else {
        (locals.var_vtn, locals.var_vtn_dn3, locals.var_vtn_dn4, locals.var_vtn_dn5, locals.var_vtn_dn6, locals.var_vtn_dn7, locals.var_vtn_dn8, locals.var_vtn_dn9, locals.var_vtn_dn10, locals.var_vtn_dn11,)
    }
};
        locals.var_vtn = assign54710_e89644;
        locals.var_vtn_dn3 = assign54710_e89644_d_n3;
        locals.var_vtn_dn4 = assign54710_e89644_d_n4;
        locals.var_vtn_dn5 = assign54710_e89644_d_n5;
        locals.var_vtn_dn6 = assign54710_e89644_d_n6;
        locals.var_vtn_dn7 = assign54710_e89644_d_n7;
        locals.var_vtn_dn8 = assign54710_e89644_d_n8;
        locals.var_vtn_dn9 = assign54710_e89644_d_n9;
        locals.var_vtn_dn10 = assign54710_e89644_d_n10;
        locals.var_vtn_dn11 = assign54710_e89644_d_n11;
        locals.var_vtn_rv = 0.0;

        let (assign54720_e89662, assign54720_e89662_d_n3, assign54720_e89662_d_n4, assign54720_e89662_d_n5, assign54720_e89662_d_n6, assign54720_e89662_d_n7, assign54720_e89662_d_n8, assign54720_e89662_d_n9, assign54720_e89662_d_n10, assign54720_e89662_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54720_e89654: f64 = (locals.var_ueff * locals.var_dptwg);
        let assign54720_e89656: f64 = (assign54720_e89654 * locals.var_moc);
        let assign54720_e89658: f64 = (assign54720_e89656 * locals.var_cox);
        let assign54720_e89660: f64 = (assign54720_e89658 * locals.var_vtn);
        (assign54720_e89660, (((((((locals.var_ueff_dn3 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn3)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn3)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn3)), (((((((locals.var_ueff_dn4 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn4)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn4)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn4)), (((((((locals.var_ueff_dn5 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn5)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn5)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn5)), (((((((locals.var_ueff_dn6 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn6)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn6)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn6)), (((((((locals.var_ueff_dn7 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn7)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn7)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn7)), (((((((locals.var_ueff_dn8 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn8)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn8)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn8)), (((((((locals.var_ueff_dn9 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn9)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn9)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn9)), (((((((locals.var_ueff_dn10 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn10)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn10)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn10)), (((((((locals.var_ueff_dn11 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn11)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn11)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign54720_e89662;
        locals.var_t0_dn3 = assign54720_e89662_d_n3;
        locals.var_t0_dn4 = assign54720_e89662_d_n4;
        locals.var_t0_dn5 = assign54720_e89662_d_n5;
        locals.var_t0_dn6 = assign54720_e89662_d_n6;
        locals.var_t0_dn7 = assign54720_e89662_d_n7;
        locals.var_t0_dn8 = assign54720_e89662_d_n8;
        locals.var_t0_dn9 = assign54720_e89662_d_n9;
        locals.var_t0_dn10 = assign54720_e89662_d_n10;
        locals.var_t0_dn11 = assign54720_e89662_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign54730_e89676, assign54730_e89676_d_n3, assign54730_e89676_d_n4, assign54730_e89676_d_n5, assign54730_e89676_d_n6, assign54730_e89676_d_n7, assign54730_e89676_d_n8, assign54730_e89676_d_n9, assign54730_e89676_d_n10, assign54730_e89676_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54730_e89673: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign54730_e89674: f64 = (0.5 * assign54730_e89673);
        (assign54730_e89674, (0.5 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3)), (0.5 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4)), (0.5 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5)), (0.5 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6)), (0.5 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7)), (0.5 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8)), (0.5 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9)), (0.5 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10)), (0.5 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign54730_e89676;
        locals.var_t1_dn3 = assign54730_e89676_d_n3;
        locals.var_t1_dn4 = assign54730_e89676_d_n4;
        locals.var_t1_dn5 = assign54730_e89676_d_n5;
        locals.var_t1_dn6 = assign54730_e89676_d_n6;
        locals.var_t1_dn7 = assign54730_e89676_d_n7;
        locals.var_t1_dn8 = assign54730_e89676_d_n8;
        locals.var_t1_dn9 = assign54730_e89676_d_n9;
        locals.var_t1_dn10 = assign54730_e89676_d_n10;
        locals.var_t1_dn11 = assign54730_e89676_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign54740_e89688, assign54740_e89688_d_n3, assign54740_e89688_d_n4, assign54740_e89688_d_n5, assign54740_e89688_d_n6, assign54740_e89688_d_n7, assign54740_e89688_d_n8, assign54740_e89688_d_n9, assign54740_e89688_d_n10, assign54740_e89688_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54740_e89686: f64 = (locals.var_t1 + 0.5);
        (assign54740_e89686, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign54740_e89688;
        locals.var_t3_dn3 = assign54740_e89688_d_n3;
        locals.var_t3_dn4 = assign54740_e89688_d_n4;
        locals.var_t3_dn5 = assign54740_e89688_d_n5;
        locals.var_t3_dn6 = assign54740_e89688_d_n6;
        locals.var_t3_dn7 = assign54740_e89688_d_n7;
        locals.var_t3_dn8 = assign54740_e89688_d_n8;
        locals.var_t3_dn9 = assign54740_e89688_d_n9;
        locals.var_t3_dn10 = assign54740_e89688_d_n10;
        locals.var_t3_dn11 = assign54740_e89688_d_n11;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_188(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign54750_e89700, assign54750_e89700_d_n3, assign54750_e89700_d_n4, assign54750_e89700_d_n5, assign54750_e89700_d_n6, assign54750_e89700_d_n7, assign54750_e89700_d_n8, assign54750_e89700_d_n9, assign54750_e89700_d_n10, assign54750_e89700_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54750_e89698: f64 = (locals.var_t3 * locals.var_t3);
        (assign54750_e89698, ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3)), ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)), ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)), ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)), ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)), ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)), ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)), ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)), ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign54750_e89700;
        locals.var_t4_dn3 = assign54750_e89700_d_n3;
        locals.var_t4_dn4 = assign54750_e89700_d_n4;
        locals.var_t4_dn5 = assign54750_e89700_d_n5;
        locals.var_t4_dn6 = assign54750_e89700_d_n6;
        locals.var_t4_dn7 = assign54750_e89700_d_n7;
        locals.var_t4_dn8 = assign54750_e89700_d_n8;
        locals.var_t4_dn9 = assign54750_e89700_d_n9;
        locals.var_t4_dn10 = assign54750_e89700_d_n10;
        locals.var_t4_dn11 = assign54750_e89700_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign54760_e89712, assign54760_e89712_d_n3, assign54760_e89712_d_n4, assign54760_e89712_d_n5, assign54760_e89712_d_n6, assign54760_e89712_d_n7, assign54760_e89712_d_n8, assign54760_e89712_d_n9, assign54760_e89712_d_n10, assign54760_e89712_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54760_e89710: f64 = (locals.var_t4 * locals.var_t3);
        (assign54760_e89710, ((locals.var_t4_dn3 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn3)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign54760_e89712;
        locals.var_t5_dn3 = assign54760_e89712_d_n3;
        locals.var_t5_dn4 = assign54760_e89712_d_n4;
        locals.var_t5_dn5 = assign54760_e89712_d_n5;
        locals.var_t5_dn6 = assign54760_e89712_d_n6;
        locals.var_t5_dn7 = assign54760_e89712_d_n7;
        locals.var_t5_dn8 = assign54760_e89712_d_n8;
        locals.var_t5_dn9 = assign54760_e89712_d_n9;
        locals.var_t5_dn10 = assign54760_e89712_d_n10;
        locals.var_t5_dn11 = assign54760_e89712_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign54770_e89724, assign54770_e89724_d_n3, assign54770_e89724_d_n4, assign54770_e89724_d_n5, assign54770_e89724_d_n6, assign54770_e89724_d_n7, assign54770_e89724_d_n8, assign54770_e89724_d_n9, assign54770_e89724_d_n10, assign54770_e89724_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54770_e89722: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign54770_e89722, (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign54770_e89724;
        locals.var_t6_dn3 = assign54770_e89724_d_n3;
        locals.var_t6_dn4 = assign54770_e89724_d_n4;
        locals.var_t6_dn5 = assign54770_e89724_d_n5;
        locals.var_t6_dn6 = assign54770_e89724_d_n6;
        locals.var_t6_dn7 = assign54770_e89724_d_n7;
        locals.var_t6_dn8 = assign54770_e89724_d_n8;
        locals.var_t6_dn9 = assign54770_e89724_d_n9;
        locals.var_t6_dn10 = assign54770_e89724_d_n10;
        locals.var_t6_dn11 = assign54770_e89724_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign54780_e89736, assign54780_e89736_d_n3, assign54780_e89736_d_n4, assign54780_e89736_d_n5, assign54780_e89736_d_n6, assign54780_e89736_d_n7, assign54780_e89736_d_n8, assign54780_e89736_d_n9, assign54780_e89736_d_n10, assign54780_e89736_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54780_e89734: f64 = (locals.var_t6 * locals.var_t6);
        (assign54780_e89734, ((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)), ((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)), ((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)), ((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)), ((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)), ((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)), ((locals.var_t6_dn9 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn9)), ((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)), ((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign54780_e89736;
        locals.var_t7_dn3 = assign54780_e89736_d_n3;
        locals.var_t7_dn4 = assign54780_e89736_d_n4;
        locals.var_t7_dn5 = assign54780_e89736_d_n5;
        locals.var_t7_dn6 = assign54780_e89736_d_n6;
        locals.var_t7_dn7 = assign54780_e89736_d_n7;
        locals.var_t7_dn8 = assign54780_e89736_d_n8;
        locals.var_t7_dn9 = assign54780_e89736_d_n9;
        locals.var_t7_dn10 = assign54780_e89736_d_n10;
        locals.var_t7_dn11 = assign54780_e89736_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign54790_e89748, assign54790_e89748_d_n3, assign54790_e89748_d_n4, assign54790_e89748_d_n5, assign54790_e89748_d_n6, assign54790_e89748_d_n7, assign54790_e89748_d_n8, assign54790_e89748_d_n9, assign54790_e89748_d_n10, assign54790_e89748_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54790_e89746: f64 = (locals.var_t7 * locals.var_t6);
        (assign54790_e89746, ((locals.var_t7_dn3 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn3)), ((locals.var_t7_dn4 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn4)), ((locals.var_t7_dn5 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn5)), ((locals.var_t7_dn6 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn6)), ((locals.var_t7_dn7 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn7)), ((locals.var_t7_dn8 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn8)), ((locals.var_t7_dn9 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn9)), ((locals.var_t7_dn10 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn10)), ((locals.var_t7_dn11 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn11)),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign54790_e89748;
        locals.var_t8_dn3 = assign54790_e89748_d_n3;
        locals.var_t8_dn4 = assign54790_e89748_d_n4;
        locals.var_t8_dn5 = assign54790_e89748_d_n5;
        locals.var_t8_dn6 = assign54790_e89748_d_n6;
        locals.var_t8_dn7 = assign54790_e89748_d_n7;
        locals.var_t8_dn8 = assign54790_e89748_d_n8;
        locals.var_t8_dn9 = assign54790_e89748_d_n9;
        locals.var_t8_dn10 = assign54790_e89748_d_n10;
        locals.var_t8_dn11 = assign54790_e89748_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign54800_e89764, assign54800_e89764_d_n3, assign54800_e89764_d_n4, assign54800_e89764_d_n5, assign54800_e89764_d_n6, assign54800_e89764_d_n7, assign54800_e89764_d_n8, assign54800_e89764_d_n9, assign54800_e89764_d_n10, assign54800_e89764_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54800_e89758: f64 = (6.0 * locals.var_t1);
        let assign54800_e89760: f64 = (assign54800_e89758 + 0.5);
        let assign54800_e89762: f64 = (assign54800_e89760 * locals.var_t7);
        (assign54800_e89762, (((6.0 * locals.var_t1_dn3) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn3)), (((6.0 * locals.var_t1_dn4) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn4)), (((6.0 * locals.var_t1_dn5) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn5)), (((6.0 * locals.var_t1_dn6) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn6)), (((6.0 * locals.var_t1_dn7) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn7)), (((6.0 * locals.var_t1_dn8) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn8)), (((6.0 * locals.var_t1_dn9) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn9)), (((6.0 * locals.var_t1_dn10) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn10)), (((6.0 * locals.var_t1_dn11) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn11)),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign54800_e89764;
        locals.var_t9_dn3 = assign54800_e89764_d_n3;
        locals.var_t9_dn4 = assign54800_e89764_d_n4;
        locals.var_t9_dn5 = assign54800_e89764_d_n5;
        locals.var_t9_dn6 = assign54800_e89764_d_n6;
        locals.var_t9_dn7 = assign54800_e89764_d_n7;
        locals.var_t9_dn8 = assign54800_e89764_d_n8;
        locals.var_t9_dn9 = assign54800_e89764_d_n9;
        locals.var_t9_dn10 = assign54800_e89764_d_n10;
        locals.var_t9_dn11 = assign54800_e89764_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign54810_e89776, assign54810_e89776_d_n3, assign54810_e89776_d_n4, assign54810_e89776_d_n5, assign54810_e89776_d_n6, assign54810_e89776_d_n7, assign54810_e89776_d_n8, assign54810_e89776_d_n9, assign54810_e89776_d_n10, assign54810_e89776_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54810_e89774: f64 = (locals.var_leff * locals.var_dptwg);
        (assign54810_e89774, (locals.var_leff * locals.var_dptwg_dn3), (locals.var_leff * locals.var_dptwg_dn4), (locals.var_leff * locals.var_dptwg_dn5), (locals.var_leff * locals.var_dptwg_dn6), (locals.var_leff * locals.var_dptwg_dn7), (locals.var_leff * locals.var_dptwg_dn8), (locals.var_leff * locals.var_dptwg_dn9), (locals.var_leff * locals.var_dptwg_dn10), (locals.var_leff * locals.var_dptwg_dn11),)
    } else {
        (locals.var_lvsat, locals.var_lvsat_dn3, locals.var_lvsat_dn4, locals.var_lvsat_dn5, locals.var_lvsat_dn6, locals.var_lvsat_dn7, locals.var_lvsat_dn8, locals.var_lvsat_dn9, locals.var_lvsat_dn10, locals.var_lvsat_dn11,)
    }
};
        locals.var_lvsat = assign54810_e89776;
        locals.var_lvsat_dn3 = assign54810_e89776_d_n3;
        locals.var_lvsat_dn4 = assign54810_e89776_d_n4;
        locals.var_lvsat_dn5 = assign54810_e89776_d_n5;
        locals.var_lvsat_dn6 = assign54810_e89776_d_n6;
        locals.var_lvsat_dn7 = assign54810_e89776_d_n7;
        locals.var_lvsat_dn8 = assign54810_e89776_d_n8;
        locals.var_lvsat_dn9 = assign54810_e89776_d_n9;
        locals.var_lvsat_dn10 = assign54810_e89776_d_n10;
        locals.var_lvsat_dn11 = assign54810_e89776_d_n11;
        locals.var_lvsat_rv = 0.0;

        let (assign54820_e89788, assign54820_e89788_d_n3, assign54820_e89788_d_n4, assign54820_e89788_d_n5, assign54820_e89788_d_n6, assign54820_e89788_d_n7, assign54820_e89788_d_n8, assign54820_e89788_d_n9, assign54820_e89788_d_n10, assign54820_e89788_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54820_e89786: f64 = (locals.var_lvsat / locals.var_leff);
        (assign54820_e89786, (locals.var_lvsat_dn3 / locals.var_leff), (locals.var_lvsat_dn4 / locals.var_leff), (locals.var_lvsat_dn5 / locals.var_leff), (locals.var_lvsat_dn6 / locals.var_leff), (locals.var_lvsat_dn7 / locals.var_leff), (locals.var_lvsat_dn8 / locals.var_leff), (locals.var_lvsat_dn9 / locals.var_leff), (locals.var_lvsat_dn10 / locals.var_leff), (locals.var_lvsat_dn11 / locals.var_leff),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign54820_e89788;
        locals.var_t10_dn3 = assign54820_e89788_d_n3;
        locals.var_t10_dn4 = assign54820_e89788_d_n4;
        locals.var_t10_dn5 = assign54820_e89788_d_n5;
        locals.var_t10_dn6 = assign54820_e89788_d_n6;
        locals.var_t10_dn7 = assign54820_e89788_d_n7;
        locals.var_t10_dn8 = assign54820_e89788_d_n8;
        locals.var_t10_dn9 = assign54820_e89788_d_n9;
        locals.var_t10_dn10 = assign54820_e89788_d_n10;
        locals.var_t10_dn11 = assign54820_e89788_d_n11;
        locals.var_t10_rv = 0.0;

        let (assign54830_e89808, assign54830_e89808_d_n3, assign54830_e89808_d_n4, assign54830_e89808_d_n5, assign54830_e89808_d_n6, assign54830_e89808_d_n7, assign54830_e89808_d_n8, assign54830_e89808_d_n9, assign54830_e89808_d_n10, assign54830_e89808_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54830_e89800: f64 = (locals.var_vdseff / locals.var_vdssat);
        let assign54830_e89801: f64 = (locals.var_betalowid * assign54830_e89800);
        let assign54830_e89804: f64 = (p.p1299 + locals.var_qia);
        let assign54830_e89805: f64 = (assign54830_e89801 / assign54830_e89804);
        let assign54830_e89806: f64 = (1.0 + assign54830_e89805);
        (assign54830_e89806, (((((locals.var_betalowid_dn3 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn3 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn3)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn3)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn4 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn4 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn4)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn4)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn5 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn5 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn5)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn5)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn6 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn6 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn6)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn6)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn7 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn7 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn7)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn7)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn8 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn8 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn8)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn8)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn9 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn9 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn9)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn9)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn10 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn10 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn10)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn10)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn11 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn11 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn11)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn11)) / (assign54830_e89804 * assign54830_e89804)),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign54830_e89808;
        locals.var_t12_dn3 = assign54830_e89808_d_n3;
        locals.var_t12_dn4 = assign54830_e89808_d_n4;
        locals.var_t12_dn5 = assign54830_e89808_d_n5;
        locals.var_t12_dn6 = assign54830_e89808_d_n6;
        locals.var_t12_dn7 = assign54830_e89808_d_n7;
        locals.var_t12_dn8 = assign54830_e89808_d_n8;
        locals.var_t12_dn9 = assign54830_e89808_d_n9;
        locals.var_t12_dn10 = assign54830_e89808_d_n10;
        locals.var_t12_dn11 = assign54830_e89808_d_n11;
        locals.var_t12_rv = 0.0;

        let (assign54840_e89828, assign54840_e89828_d_n3, assign54840_e89828_d_n4, assign54840_e89828_d_n5, assign54840_e89828_d_n6, assign54840_e89828_d_n7, assign54840_e89828_d_n8, assign54840_e89828_d_n9, assign54840_e89828_d_n10, assign54840_e89828_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54840_e89818: f64 = (locals.var_t12 - 1.0);
        let assign54840_e89820: f64 = (-locals.var_leff);
        let assign54840_e89822: f64 = (assign54840_e89820 / p.p1296);
        let assign54840_e89823: f64 = { let limited_exp_arg = assign54840_e89822; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign54840_e89824: f64 = (assign54840_e89818 * assign54840_e89823);
        let assign54840_e89826: f64 = (assign54840_e89824 + 1.0);
        (assign54840_e89826, (locals.var_t12_dn3 * assign54840_e89823), (locals.var_t12_dn4 * assign54840_e89823), (locals.var_t12_dn5 * assign54840_e89823), (locals.var_t12_dn6 * assign54840_e89823), (locals.var_t12_dn7 * assign54840_e89823), (locals.var_t12_dn8 * assign54840_e89823), (locals.var_t12_dn9 * assign54840_e89823), (locals.var_t12_dn10 * assign54840_e89823), (locals.var_t12_dn11 * assign54840_e89823),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign54840_e89828;
        locals.var_t12_dn3 = assign54840_e89828_d_n3;
        locals.var_t12_dn4 = assign54840_e89828_d_n4;
        locals.var_t12_dn5 = assign54840_e89828_d_n5;
        locals.var_t12_dn6 = assign54840_e89828_d_n6;
        locals.var_t12_dn7 = assign54840_e89828_d_n7;
        locals.var_t12_dn8 = assign54840_e89828_d_n8;
        locals.var_t12_dn9 = assign54840_e89828_d_n9;
        locals.var_t12_dn10 = assign54840_e89828_d_n10;
        locals.var_t12_dn11 = assign54840_e89828_d_n11;
        locals.var_t12_rv = 0.0;

        let (assign54850_e89857, assign54850_e89857_d_n3, assign54850_e89857_d_n4, assign54850_e89857_d_n5, assign54850_e89857_d_n6, assign54850_e89857_d_n7, assign54850_e89857_d_n8, assign54850_e89857_d_n9, assign54850_e89857_d_n10, assign54850_e89857_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54850_e89839: f64 = locals.var_t12;
        let assign54850_e89842: f64 = locals.var_t12;
        let assign54850_e89845: f64 = locals.var_t12;
        let assign54850_e89846: f64 = (assign54850_e89842 * assign54850_e89845);
        let assign54850_e89849: f64 = (0.25 * 0.1);
        let assign54850_e89851: f64 = (assign54850_e89849 * 0.1);
        let assign54850_e89852: f64 = (assign54850_e89846 + assign54850_e89851);
        let assign54850_e89853: f64 = (assign54850_e89852).sqrt();
        let assign54850_e89854: f64 = (assign54850_e89839 + assign54850_e89853);
        let assign54850_e89855: f64 = (0.5 * assign54850_e89854);
        (assign54850_e89855, (0.5 * (locals.var_t12_dn3 + (((locals.var_t12_dn3 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn3)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn4 + (((locals.var_t12_dn4 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn4)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn5 + (((locals.var_t12_dn5 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn5)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn6 + (((locals.var_t12_dn6 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn6)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn7 + (((locals.var_t12_dn7 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn7)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn8 + (((locals.var_t12_dn8 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn8)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn9 + (((locals.var_t12_dn9 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn9)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn10 + (((locals.var_t12_dn10 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn10)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn11 + (((locals.var_t12_dn11 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn11)) / (2.0 * assign54850_e89853)))),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign54850_e89857;
        locals.var_t12_dn3 = assign54850_e89857_d_n3;
        locals.var_t12_dn4 = assign54850_e89857_d_n4;
        locals.var_t12_dn5 = assign54850_e89857_d_n5;
        locals.var_t12_dn6 = assign54850_e89857_d_n6;
        locals.var_t12_dn7 = assign54850_e89857_d_n7;
        locals.var_t12_dn8 = assign54850_e89857_d_n8;
        locals.var_t12_dn9 = assign54850_e89857_d_n9;
        locals.var_t12_dn10 = assign54850_e89857_d_n10;
        locals.var_t12_dn11 = assign54850_e89857_d_n11;
        locals.var_t12_rv = 0.0;

        let (assign54870_e89935, assign54870_e89935_d_n3, assign54870_e89935_d_n4, assign54870_e89935_d_n5, assign54870_e89935_d_n6, assign54870_e89935_d_n7, assign54870_e89935_d_n8, assign54870_e89935_d_n9, assign54870_e89935_d_n10, assign54870_e89935_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54870_e89895: f64 = (locals.var_lvsat * locals.var_t10);
        let assign54870_e89897: f64 = (assign54870_e89895 * locals.var_t10);
        let assign54870_e89900: f64 = (locals.var_t1 / locals.var_t4);
        let assign54870_e89904: f64 = (60.0 * locals.var_t4);
        let assign54870_e89906: f64 = (assign54870_e89904 * locals.var_t4);
        let assign54870_e89907: f64 = (locals.var_t9 / assign54870_e89906);
        let assign54870_e89908: f64 = (assign54870_e89900 - assign54870_e89907);
        let assign54870_e89911: f64 = (locals.var_t7 * locals.var_t7);
        let assign54870_e89914: f64 = (144.0 * locals.var_t4);
        let assign54870_e89916: f64 = (assign54870_e89914 * locals.var_t5);
        let assign54870_e89917: f64 = (assign54870_e89911 / assign54870_e89916);
        let assign54870_e89918: f64 = (assign54870_e89908 + assign54870_e89917);
        let assign54870_e89919: f64 = (assign54870_e89897 * assign54870_e89918);
        let assign54870_e89921: f64 = (assign54870_e89919 * 15.0);
        let assign54870_e89923: f64 = (assign54870_e89921 / 4.0);
        let assign54870_e89925: f64 = (assign54870_e89923 * locals.var_thetanoisq);
        let assign54870_e89928: f64 = (p.p2 * locals.var_weff);
        let assign54870_e89930: f64 = (assign54870_e89928 * 12.0);
        let assign54870_e89932: f64 = (assign54870_e89930 * locals.var_t0);
        let assign54870_e89933: f64 = (assign54870_e89925 / assign54870_e89932);
        (assign54870_e89933, (((((((((((((locals.var_lvsat_dn3 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn3)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn3)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn3 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn3 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn3) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn3)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn3 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn3)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn3) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn3)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn3)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn3))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn4 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn4)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn4)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn4 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn4 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn4) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn4)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn4) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn4)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn4)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn4))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn5 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn5)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn5)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn5 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn5 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn5) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn5)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn5) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn5)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn5)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn5))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn6 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn6)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn6)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn6 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn6 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn6) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn6)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn6) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn6)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn6)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn6))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn7 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn7)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn7)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn7 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn7 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn7) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn7)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn7) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn7)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn7)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn7))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn8 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn8)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn8)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn8 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn8 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn8) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn8)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn8) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn8)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn8)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn8))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn9 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn9)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn9)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn9 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn9 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn9) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn9)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn9) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn9)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn9)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn9))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn10 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn10)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn10)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn10 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn10 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn10) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn10)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn10) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn10)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn10)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn10))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn11 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn11)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn11)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn11 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn11 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn11) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn11)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn11) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn11)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn11)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn11))) / (assign54870_e89932 * assign54870_e89932)),)
    } else {
        (locals.var_mig, locals.var_mig_dn3, locals.var_mig_dn4, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn9, locals.var_mig_dn10, locals.var_mig_dn11,)
    }
};
        locals.var_mig = assign54870_e89935;
        locals.var_mig_dn3 = assign54870_e89935_d_n3;
        locals.var_mig_dn4 = assign54870_e89935_d_n4;
        locals.var_mig_dn5 = assign54870_e89935_d_n5;
        locals.var_mig_dn6 = assign54870_e89935_d_n6;
        locals.var_mig_dn7 = assign54870_e89935_d_n7;
        locals.var_mig_dn8 = assign54870_e89935_d_n8;
        locals.var_mig_dn9 = assign54870_e89935_d_n9;
        locals.var_mig_dn10 = assign54870_e89935_d_n10;
        locals.var_mig_dn11 = assign54870_e89935_d_n11;
        locals.var_mig_rv = 0.0;

        let (assign54990_e90065, assign54990_e90065_d_n3, assign54990_e90065_d_n4, assign54990_e90065_d_n5, assign54990_e90065_d_n6, assign54990_e90065_d_n7, assign54990_e90065_d_n8, assign54990_e90065_d_n9, assign54990_e90065_d_n10, assign54990_e90065_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (locals.var_vgfb, locals.var_vgfb_dn3, locals.var_vgfb_dn4, locals.var_vgfb_dn5, locals.var_vgfb_dn6, locals.var_vgfb_dn7, locals.var_vgfb_dn8, locals.var_vgfb_dn9, locals.var_vgfb_dn10, locals.var_vgfb_dn11,)
    } else {
        (locals.var_vgfbcv, locals.var_vgfbcv_dn3, locals.var_vgfbcv_dn4, locals.var_vgfbcv_dn5, locals.var_vgfbcv_dn6, locals.var_vgfbcv_dn7, locals.var_vgfbcv_dn8, locals.var_vgfbcv_dn9, locals.var_vgfbcv_dn10, locals.var_vgfbcv_dn11,)
    }
};
        locals.var_vgfbcv = assign54990_e90065;
        locals.var_vgfbcv_dn3 = assign54990_e90065_d_n3;
        locals.var_vgfbcv_dn4 = assign54990_e90065_d_n4;
        locals.var_vgfbcv_dn5 = assign54990_e90065_d_n5;
        locals.var_vgfbcv_dn6 = assign54990_e90065_d_n6;
        locals.var_vgfbcv_dn7 = assign54990_e90065_d_n7;
        locals.var_vgfbcv_dn8 = assign54990_e90065_d_n8;
        locals.var_vgfbcv_dn9 = assign54990_e90065_d_n9;
        locals.var_vgfbcv_dn10 = assign54990_e90065_d_n10;
        locals.var_vgfbcv_dn11 = assign54990_e90065_d_n11;
        locals.var_vgfbcv_rv = 0.0;

        let (assign55000_e90070, assign55000_e90070_d_n4, assign55000_e90070_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_invgamg2, locals.var_invgamg2_dn4, locals.var_invgamg2_dn5,)
    }
};
        locals.var_invgamg2 = assign55000_e90070;
        locals.var_invgamg2_dn4 = assign55000_e90070_d_n4;
        locals.var_invgamg2_dn5 = assign55000_e90070_d_n5;
        locals.var_invgamg2_rv = 0.0;

        let assign55010_e90073: f64 = if p.p31 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard842 = assign55010_e90073;
        locals.var_guard842_rv = 0.0;

        let (assign55020_e90082, assign55020_e90082_d_n3, assign55020_e90082_d_n4, assign55020_e90082_d_n5, assign55020_e90082_d_n6, assign55020_e90082_d_n7, assign55020_e90082_d_n8, assign55020_e90082_d_n9, assign55020_e90082_d_n10, assign55020_e90082_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55020_e90080: f64 = (locals.var_vfbcv_i + p.p25);
        (assign55020_e90080, locals.var_vfbcv_i_dn3, locals.var_vfbcv_i_dn4, locals.var_vfbcv_i_dn5, locals.var_vfbcv_i_dn6, locals.var_vfbcv_i_dn7, locals.var_vfbcv_i_dn8, locals.var_vfbcv_i_dn9, locals.var_vfbcv_i_dn10, locals.var_vfbcv_i_dn11,)
    } else {
        (locals.var_vfbcv_i, locals.var_vfbcv_i_dn3, locals.var_vfbcv_i_dn4, locals.var_vfbcv_i_dn5, locals.var_vfbcv_i_dn6, locals.var_vfbcv_i_dn7, locals.var_vfbcv_i_dn8, locals.var_vfbcv_i_dn9, locals.var_vfbcv_i_dn10, locals.var_vfbcv_i_dn11,)
    }
};
        locals.var_vfbcv_i = assign55020_e90082;
        locals.var_vfbcv_i_dn3 = assign55020_e90082_d_n3;
        locals.var_vfbcv_i_dn4 = assign55020_e90082_d_n4;
        locals.var_vfbcv_i_dn5 = assign55020_e90082_d_n5;
        locals.var_vfbcv_i_dn6 = assign55020_e90082_d_n6;
        locals.var_vfbcv_i_dn7 = assign55020_e90082_d_n7;
        locals.var_vfbcv_i_dn8 = assign55020_e90082_d_n8;
        locals.var_vfbcv_i_dn9 = assign55020_e90082_d_n9;
        locals.var_vfbcv_i_dn10 = assign55020_e90082_d_n10;
        locals.var_vfbcv_i_dn11 = assign55020_e90082_d_n11;
        locals.var_vfbcv_i_rv = 0.0;

        let (assign55030_e90091, assign55030_e90091_d_n3, assign55030_e90091_d_n4, assign55030_e90091_d_n5, assign55030_e90091_d_n6, assign55030_e90091_d_n7, assign55030_e90091_d_n8, assign55030_e90091_d_n9, assign55030_e90091_d_n10, assign55030_e90091_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55030_e90089: f64 = (locals.var_vg * locals.var_inv_vt);
        (assign55030_e90089, 0.0, (locals.var_vg * locals.var_inv_vt_dn4), (locals.var_vg * locals.var_inv_vt_dn5), 0.0, 0.0, (locals.var_vg_dn8 * locals.var_inv_vt), 0.0, (locals.var_vg_dn10 * locals.var_inv_vt), 0.0,)
    } else {
        (locals.var_vg_1, locals.var_vg_1_dn3, locals.var_vg_1_dn4, locals.var_vg_1_dn5, locals.var_vg_1_dn6, locals.var_vg_1_dn7, locals.var_vg_1_dn8, locals.var_vg_1_dn9, locals.var_vg_1_dn10, locals.var_vg_1_dn11,)
    }
};
        locals.var_vg_1 = assign55030_e90091;
        locals.var_vg_1_dn3 = assign55030_e90091_d_n3;
        locals.var_vg_1_dn4 = assign55030_e90091_d_n4;
        locals.var_vg_1_dn5 = assign55030_e90091_d_n5;
        locals.var_vg_1_dn6 = assign55030_e90091_d_n6;
        locals.var_vg_1_dn7 = assign55030_e90091_d_n7;
        locals.var_vg_1_dn8 = assign55030_e90091_d_n8;
        locals.var_vg_1_dn9 = assign55030_e90091_d_n9;
        locals.var_vg_1_dn10 = assign55030_e90091_d_n10;
        locals.var_vg_1_dn11 = assign55030_e90091_d_n11;
        locals.var_vg_1_rv = 0.0;

        let (assign55040_e90100, assign55040_e90100_d_n3, assign55040_e90100_d_n4, assign55040_e90100_d_n5, assign55040_e90100_d_n6, assign55040_e90100_d_n7, assign55040_e90100_d_n8, assign55040_e90100_d_n9, assign55040_e90100_d_n10, assign55040_e90100_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55040_e90098: f64 = (locals.var_vs * locals.var_inv_vt);
        (assign55040_e90098, 0.0, (locals.var_vs * locals.var_inv_vt_dn4), (locals.var_vs * locals.var_inv_vt_dn5), (locals.var_vs_dn6 * locals.var_inv_vt), (locals.var_vs_dn7 * locals.var_inv_vt), 0.0, 0.0, (locals.var_vs_dn10 * locals.var_inv_vt), 0.0,)
    } else {
        (locals.var_vs_1, locals.var_vs_1_dn3, locals.var_vs_1_dn4, locals.var_vs_1_dn5, locals.var_vs_1_dn6, locals.var_vs_1_dn7, locals.var_vs_1_dn8, locals.var_vs_1_dn9, locals.var_vs_1_dn10, locals.var_vs_1_dn11,)
    }
};
        locals.var_vs_1 = assign55040_e90100;
        locals.var_vs_1_dn3 = assign55040_e90100_d_n3;
        locals.var_vs_1_dn4 = assign55040_e90100_d_n4;
        locals.var_vs_1_dn5 = assign55040_e90100_d_n5;
        locals.var_vs_1_dn6 = assign55040_e90100_d_n6;
        locals.var_vs_1_dn7 = assign55040_e90100_d_n7;
        locals.var_vs_1_dn8 = assign55040_e90100_d_n8;
        locals.var_vs_1_dn9 = assign55040_e90100_d_n9;
        locals.var_vs_1_dn10 = assign55040_e90100_d_n10;
        locals.var_vs_1_dn11 = assign55040_e90100_d_n11;
        locals.var_vs_1_rv = 0.0;

        let (assign55050_e90109, assign55050_e90109_d_n3, assign55050_e90109_d_n4, assign55050_e90109_d_n5, assign55050_e90109_d_n6, assign55050_e90109_d_n7, assign55050_e90109_d_n8, assign55050_e90109_d_n9, assign55050_e90109_d_n10, assign55050_e90109_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55050_e90107: f64 = (locals.var_vfbcv_i * locals.var_inv_vt);
        (assign55050_e90107, (locals.var_vfbcv_i_dn3 * locals.var_inv_vt), ((locals.var_vfbcv_i_dn4 * locals.var_inv_vt) + (locals.var_vfbcv_i * locals.var_inv_vt_dn4)), ((locals.var_vfbcv_i_dn5 * locals.var_inv_vt) + (locals.var_vfbcv_i * locals.var_inv_vt_dn5)), (locals.var_vfbcv_i_dn6 * locals.var_inv_vt), (locals.var_vfbcv_i_dn7 * locals.var_inv_vt), (locals.var_vfbcv_i_dn8 * locals.var_inv_vt), (locals.var_vfbcv_i_dn9 * locals.var_inv_vt), (locals.var_vfbcv_i_dn10 * locals.var_inv_vt), (locals.var_vfbcv_i_dn11 * locals.var_inv_vt),)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11,)
    }
};
        locals.var_vfb = assign55050_e90109;
        locals.var_vfb_dn3 = assign55050_e90109_d_n3;
        locals.var_vfb_dn4 = assign55050_e90109_d_n4;
        locals.var_vfb_dn5 = assign55050_e90109_d_n5;
        locals.var_vfb_dn6 = assign55050_e90109_d_n6;
        locals.var_vfb_dn7 = assign55050_e90109_d_n7;
        locals.var_vfb_dn8 = assign55050_e90109_d_n8;
        locals.var_vfb_dn9 = assign55050_e90109_d_n9;
        locals.var_vfb_dn10 = assign55050_e90109_d_n10;
        locals.var_vfb_dn11 = assign55050_e90109_d_n11;
        locals.var_vfb_rv = 0.0;

        let (assign55060_e90118, assign55060_e90118_d_n3, assign55060_e90118_d_n4, assign55060_e90118_d_n5, assign55060_e90118_d_n6, assign55060_e90118_d_n7, assign55060_e90118_d_n8, assign55060_e90118_d_n9, assign55060_e90118_d_n10, assign55060_e90118_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55060_e90116: f64 = (locals.var_vg_1 - locals.var_vfb);
        (assign55060_e90116, (locals.var_vg_1_dn3 - locals.var_vfb_dn3), (locals.var_vg_1_dn4 - locals.var_vfb_dn4), (locals.var_vg_1_dn5 - locals.var_vfb_dn5), (locals.var_vg_1_dn6 - locals.var_vfb_dn6), (locals.var_vg_1_dn7 - locals.var_vfb_dn7), (locals.var_vg_1_dn8 - locals.var_vfb_dn8), (locals.var_vg_1_dn9 - locals.var_vfb_dn9), (locals.var_vg_1_dn10 - locals.var_vfb_dn10), (locals.var_vg_1_dn11 - locals.var_vfb_dn11),)
    } else {
        (locals.var_vgfbcv, locals.var_vgfbcv_dn3, locals.var_vgfbcv_dn4, locals.var_vgfbcv_dn5, locals.var_vgfbcv_dn6, locals.var_vgfbcv_dn7, locals.var_vgfbcv_dn8, locals.var_vgfbcv_dn9, locals.var_vgfbcv_dn10, locals.var_vgfbcv_dn11,)
    }
};
        locals.var_vgfbcv = assign55060_e90118;
        locals.var_vgfbcv_dn3 = assign55060_e90118_d_n3;
        locals.var_vgfbcv_dn4 = assign55060_e90118_d_n4;
        locals.var_vgfbcv_dn5 = assign55060_e90118_d_n5;
        locals.var_vgfbcv_dn6 = assign55060_e90118_d_n6;
        locals.var_vgfbcv_dn7 = assign55060_e90118_d_n7;
        locals.var_vgfbcv_dn8 = assign55060_e90118_d_n8;
        locals.var_vgfbcv_dn9 = assign55060_e90118_d_n9;
        locals.var_vgfbcv_dn10 = assign55060_e90118_d_n10;
        locals.var_vgfbcv_dn11 = assign55060_e90118_d_n11;
        locals.var_vgfbcv_rv = 0.0;

        let (assign55070_e90130, assign55070_e90130_d_n3, assign55070_e90130_d_n4, assign55070_e90130_d_n5, assign55070_e90130_d_n6, assign55070_e90130_d_n7, assign55070_e90130_d_n8, assign55070_e90130_d_n9, assign55070_e90130_d_n10, assign55070_e90130_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55070_e90125: f64 = (locals.var_ndepcv_i / locals.var_ni);
        let assign55070_e90127: f64 = (assign55070_e90125).max(1e-38);
        let assign55070_e90128: f64 = (assign55070_e90127).ln();
        (assign55070_e90128, (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn3 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn4 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn5 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn6 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn7 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn8 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn9 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn10 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn11 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127),)
    } else {
        (locals.var_phibcv, locals.var_phibcv_dn3, locals.var_phibcv_dn4, locals.var_phibcv_dn5, locals.var_phibcv_dn6, locals.var_phibcv_dn7, locals.var_phibcv_dn8, locals.var_phibcv_dn9, locals.var_phibcv_dn10, locals.var_phibcv_dn11,)
    }
};
        locals.var_phibcv = assign55070_e90130;
        locals.var_phibcv_dn3 = assign55070_e90130_d_n3;
        locals.var_phibcv_dn4 = assign55070_e90130_d_n4;
        locals.var_phibcv_dn5 = assign55070_e90130_d_n5;
        locals.var_phibcv_dn6 = assign55070_e90130_d_n6;
        locals.var_phibcv_dn7 = assign55070_e90130_d_n7;
        locals.var_phibcv_dn8 = assign55070_e90130_d_n8;
        locals.var_phibcv_dn9 = assign55070_e90130_d_n9;
        locals.var_phibcv_dn10 = assign55070_e90130_d_n10;
        locals.var_phibcv_dn11 = assign55070_e90130_d_n11;
        locals.var_phibcv_rv = 0.0;

        let (assign55080_e90148, assign55080_e90148_d_n3, assign55080_e90148_d_n4, assign55080_e90148_d_n5, assign55080_e90148_d_n6, assign55080_e90148_d_n7, assign55080_e90148_d_n8, assign55080_e90148_d_n9, assign55080_e90148_d_n10, assign55080_e90148_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55080_e90137: f64 = (2.0 * 1.602176462e-19);
        let assign55080_e90139: f64 = (assign55080_e90137 * locals.var_epssi);
        let assign55080_e90141: f64 = (assign55080_e90139 * locals.var_ndepcv_i);
        let assign55080_e90143: f64 = (assign55080_e90141 * locals.var_inv_vt);
        let assign55080_e90144: f64 = (assign55080_e90143).sqrt();
        let assign55080_e90146: f64 = (assign55080_e90144 / locals.var_cox);
        (assign55080_e90146, ((((assign55080_e90139 * locals.var_ndepcv_i_dn3) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), (((((assign55080_e90139 * locals.var_ndepcv_i_dn4) * locals.var_inv_vt) + (assign55080_e90141 * locals.var_inv_vt_dn4)) / (2.0 * assign55080_e90144)) / locals.var_cox), (((((assign55080_e90139 * locals.var_ndepcv_i_dn5) * locals.var_inv_vt) + (assign55080_e90141 * locals.var_inv_vt_dn5)) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn6) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn7) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn8) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn9) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn10) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn11) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox),)
    } else {
        (locals.var_gamcv, locals.var_gamcv_dn3, locals.var_gamcv_dn4, locals.var_gamcv_dn5, locals.var_gamcv_dn6, locals.var_gamcv_dn7, locals.var_gamcv_dn8, locals.var_gamcv_dn9, locals.var_gamcv_dn10, locals.var_gamcv_dn11,)
    }
};
        locals.var_gamcv = assign55080_e90148;
        locals.var_gamcv_dn3 = assign55080_e90148_d_n3;
        locals.var_gamcv_dn4 = assign55080_e90148_d_n4;
        locals.var_gamcv_dn5 = assign55080_e90148_d_n5;
        locals.var_gamcv_dn6 = assign55080_e90148_d_n6;
        locals.var_gamcv_dn7 = assign55080_e90148_d_n7;
        locals.var_gamcv_dn8 = assign55080_e90148_d_n8;
        locals.var_gamcv_dn9 = assign55080_e90148_d_n9;
        locals.var_gamcv_dn10 = assign55080_e90148_d_n10;
        locals.var_gamcv_dn11 = assign55080_e90148_d_n11;
        locals.var_gamcv_rv = 0.0;

        let (assign55090_e90157, assign55090_e90157_d_n3, assign55090_e90157_d_n4, assign55090_e90157_d_n5, assign55090_e90157_d_n6, assign55090_e90157_d_n7, assign55090_e90157_d_n8, assign55090_e90157_d_n9, assign55090_e90157_d_n10, assign55090_e90157_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55090_e90155: f64 = (1.0 / locals.var_gamcv);
        (assign55090_e90155, (-(locals.var_gamcv_dn3 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn4 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn5 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn6 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn7 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn8 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn9 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn10 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn11 / (locals.var_gamcv * locals.var_gamcv))),)
    } else {
        (locals.var_inv_gam, locals.var_inv_gam_dn3, locals.var_inv_gam_dn4, locals.var_inv_gam_dn5, locals.var_inv_gam_dn6, locals.var_inv_gam_dn7, locals.var_inv_gam_dn8, locals.var_inv_gam_dn9, locals.var_inv_gam_dn10, locals.var_inv_gam_dn11,)
    }
};
        locals.var_inv_gam = assign55090_e90157;
        locals.var_inv_gam_dn3 = assign55090_e90157_d_n3;
        locals.var_inv_gam_dn4 = assign55090_e90157_d_n4;
        locals.var_inv_gam_dn5 = assign55090_e90157_d_n5;
        locals.var_inv_gam_dn6 = assign55090_e90157_d_n6;
        locals.var_inv_gam_dn7 = assign55090_e90157_d_n7;
        locals.var_inv_gam_dn8 = assign55090_e90157_d_n8;
        locals.var_inv_gam_dn9 = assign55090_e90157_d_n9;
        locals.var_inv_gam_dn10 = assign55090_e90157_d_n10;
        locals.var_inv_gam_dn11 = assign55090_e90157_d_n11;
        locals.var_inv_gam_rv = 0.0;

        let (assign55100_e90176, assign55100_e90176_d_n4, assign55100_e90176_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55100_e90164: f64 = (2.0 * 1.602176462e-19);
        let assign55100_e90166: f64 = (assign55100_e90164 * locals.var_epssi);
        let assign55100_e90168: f64 = (assign55100_e90166 * locals.var_ngate_i);
        let assign55100_e90171: f64 = (locals.var_cox * locals.var_cox);
        let assign55100_e90173: f64 = (assign55100_e90171 * locals.var_vt);
        let assign55100_e90174: f64 = (assign55100_e90168 / assign55100_e90173);
        (assign55100_e90174, (-((assign55100_e90168 * (assign55100_e90171 * locals.var_vt_dn4)) / (assign55100_e90173 * assign55100_e90173))), (-((assign55100_e90168 * (assign55100_e90171 * locals.var_vt_dn5)) / (assign55100_e90173 * assign55100_e90173))),)
    } else {
        (locals.var_gamg2, locals.var_gamg2_dn4, locals.var_gamg2_dn5,)
    }
};
        locals.var_gamg2 = assign55100_e90176;
        locals.var_gamg2_dn4 = assign55100_e90176_d_n4;
        locals.var_gamg2_dn5 = assign55100_e90176_d_n5;
        locals.var_gamg2_rv = 0.0;

    }
}
