#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        locals: &mut StampLocals,
    ) {
        let (assign28020_e26891, assign28020_e26891_d_n0, assign28020_e26891_d_n2, assign28020_e26891_d_n4, assign28020_e26891_d_n5, assign28020_e26891_d_n6, assign28020_e26891_d_n7, assign28020_e26891_d_n8, assign28020_e26891_d_n9, assign28020_e26891_d_n10, assign28020_e26891_d_n11, assign28020_e26891_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign28020_e26885: f64 = (0.05 * locals.var_xmp);
        let assign28020_e26887: f64 = (assign28020_e26885 * locals.var_dnm);
        let assign28020_e26889: f64 = (assign28020_e26887 / locals.var_arg);
        (assign28020_e26889, ((((((0.05 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn11) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn11)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn14) * locals.var_dnm) + (assign28020_e26885 * locals.var_dnm_dn14)) * locals.var_arg) - (assign28020_e26887 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28020_e26891;
        locals.var_t0_dn0 = assign28020_e26891_d_n0;
        locals.var_t0_dn2 = assign28020_e26891_d_n2;
        locals.var_t0_dn4 = assign28020_e26891_d_n4;
        locals.var_t0_dn5 = assign28020_e26891_d_n5;
        locals.var_t0_dn6 = assign28020_e26891_d_n6;
        locals.var_t0_dn7 = assign28020_e26891_d_n7;
        locals.var_t0_dn8 = assign28020_e26891_d_n8;
        locals.var_t0_dn9 = assign28020_e26891_d_n9;
        locals.var_t0_dn10 = assign28020_e26891_d_n10;
        locals.var_t0_dn11 = assign28020_e26891_d_n11;
        locals.var_t0_dn14 = assign28020_e26891_d_n14;

        let (assign28030_e26903, assign28030_e26903_d_n0, assign28030_e26903_d_n2, assign28030_e26903_d_n4, assign28030_e26903_d_n5, assign28030_e26903_d_n6, assign28030_e26903_d_n7, assign28030_e26903_d_n8, assign28030_e26903_d_n9, assign28030_e26903_d_n10, assign28030_e26903_d_n11, assign28030_e26903_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign28030_e26899: f64 = 0.05;
        let assign28030_e26901: f64 = (assign28030_e26899 - locals.var_tmf0);
        (assign28030_e26901, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28030_e26903;
        locals.var_t2_dn0 = assign28030_e26903_d_n0;
        locals.var_t2_dn2 = assign28030_e26903_d_n2;
        locals.var_t2_dn4 = assign28030_e26903_d_n4;
        locals.var_t2_dn5 = assign28030_e26903_d_n5;
        locals.var_t2_dn6 = assign28030_e26903_d_n6;
        locals.var_t2_dn7 = assign28030_e26903_d_n7;
        locals.var_t2_dn8 = assign28030_e26903_d_n8;
        locals.var_t2_dn9 = assign28030_e26903_d_n9;
        locals.var_t2_dn10 = assign28030_e26903_d_n10;
        locals.var_t2_dn11 = assign28030_e26903_d_n11;
        locals.var_t2_dn14 = assign28030_e26903_d_n14;

        let (assign28040_e26911, assign28040_e26911_d_n0, assign28040_e26911_d_n2, assign28040_e26911_d_n4, assign28040_e26911_d_n5, assign28040_e26911_d_n6, assign28040_e26911_d_n7, assign28040_e26911_d_n8, assign28040_e26911_d_n9, assign28040_e26911_d_n10, assign28040_e26911_d_n11, assign28040_e26911_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28040_e26911;
        locals.var_t0_dn0 = assign28040_e26911_d_n0;
        locals.var_t0_dn2 = assign28040_e26911_d_n2;
        locals.var_t0_dn4 = assign28040_e26911_d_n4;
        locals.var_t0_dn5 = assign28040_e26911_d_n5;
        locals.var_t0_dn6 = assign28040_e26911_d_n6;
        locals.var_t0_dn7 = assign28040_e26911_d_n7;
        locals.var_t0_dn8 = assign28040_e26911_d_n8;
        locals.var_t0_dn9 = assign28040_e26911_d_n9;
        locals.var_t0_dn10 = assign28040_e26911_d_n10;
        locals.var_t0_dn11 = assign28040_e26911_d_n11;
        locals.var_t0_dn14 = assign28040_e26911_d_n14;

        let (assign28050_e26920, assign28050_e26920_d_n0, assign28050_e26920_d_n2, assign28050_e26920_d_n4, assign28050_e26920_d_n5, assign28050_e26920_d_n6, assign28050_e26920_d_n7, assign28050_e26920_d_n8, assign28050_e26920_d_n9, assign28050_e26920_d_n10, assign28050_e26920_d_n11, assign28050_e26920_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28050_e26920;
        locals.var_t2_dn0 = assign28050_e26920_d_n0;
        locals.var_t2_dn2 = assign28050_e26920_d_n2;
        locals.var_t2_dn4 = assign28050_e26920_d_n4;
        locals.var_t2_dn5 = assign28050_e26920_d_n5;
        locals.var_t2_dn6 = assign28050_e26920_d_n6;
        locals.var_t2_dn7 = assign28050_e26920_d_n7;
        locals.var_t2_dn8 = assign28050_e26920_d_n8;
        locals.var_t2_dn9 = assign28050_e26920_d_n9;
        locals.var_t2_dn10 = assign28050_e26920_d_n10;
        locals.var_t2_dn11 = assign28050_e26920_d_n11;
        locals.var_t2_dn14 = assign28050_e26920_d_n14;

        let (assign28060_e26929, assign28060_e26929_d_n0, assign28060_e26929_d_n2, assign28060_e26929_d_n4, assign28060_e26929_d_n5, assign28060_e26929_d_n6, assign28060_e26929_d_n7, assign28060_e26929_d_n8, assign28060_e26929_d_n9, assign28060_e26929_d_n10, assign28060_e26929_d_n11, assign28060_e26929_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard659 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28060_e26929;
        locals.var_t0_dn0 = assign28060_e26929_d_n0;
        locals.var_t0_dn2 = assign28060_e26929_d_n2;
        locals.var_t0_dn4 = assign28060_e26929_d_n4;
        locals.var_t0_dn5 = assign28060_e26929_d_n5;
        locals.var_t0_dn6 = assign28060_e26929_d_n6;
        locals.var_t0_dn7 = assign28060_e26929_d_n7;
        locals.var_t0_dn8 = assign28060_e26929_d_n8;
        locals.var_t0_dn9 = assign28060_e26929_d_n9;
        locals.var_t0_dn10 = assign28060_e26929_d_n10;
        locals.var_t0_dn11 = assign28060_e26929_d_n11;
        locals.var_t0_dn14 = assign28060_e26929_d_n14;

        let (assign28070_e26938, assign28070_e26938_d_n0, assign28070_e26938_d_n2, assign28070_e26938_d_n4, assign28070_e26938_d_n5, assign28070_e26938_d_n6, assign28070_e26938_d_n7, assign28070_e26938_d_n8, assign28070_e26938_d_n9, assign28070_e26938_d_n10, assign28070_e26938_d_n11, assign28070_e26938_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign28070_e26935: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign28070_e26936: f64 = (assign28070_e26935).sqrt();
        (assign28070_e26936, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn11)) / (2.0 * assign28070_e26936)), (((locals.var_c_2esipq_ndepm_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn14)) / (2.0 * assign28070_e26936)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn11, locals.var_w_s0_dn14,)
    }
};
        locals.var_w_s0 = assign28070_e26938;
        locals.var_w_s0_dn0 = assign28070_e26938_d_n0;
        locals.var_w_s0_dn2 = assign28070_e26938_d_n2;
        locals.var_w_s0_dn4 = assign28070_e26938_d_n4;
        locals.var_w_s0_dn5 = assign28070_e26938_d_n5;
        locals.var_w_s0_dn6 = assign28070_e26938_d_n6;
        locals.var_w_s0_dn7 = assign28070_e26938_d_n7;
        locals.var_w_s0_dn8 = assign28070_e26938_d_n8;
        locals.var_w_s0_dn9 = assign28070_e26938_d_n9;
        locals.var_w_s0_dn10 = assign28070_e26938_d_n10;
        locals.var_w_s0_dn11 = assign28070_e26938_d_n11;
        locals.var_w_s0_dn14 = assign28070_e26938_d_n14;

        let (assign28080_e26948, assign28080_e26948_d_n0, assign28080_e26948_d_n2, assign28080_e26948_d_n4, assign28080_e26948_d_n5, assign28080_e26948_d_n6, assign28080_e26948_d_n7, assign28080_e26948_d_n8, assign28080_e26948_d_n9, assign28080_e26948_d_n10, assign28080_e26948_d_n11, assign28080_e26948_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign28080_e26944: f64 = (locals.var_uc_depthn - locals.var_w_b0);
        let assign28080_e26946: f64 = (assign28080_e26944 - locals.var_w_s0);
        (assign28080_e26946, ((locals.var_uc_depthn_dn0 - locals.var_w_b0_dn0) - locals.var_w_s0_dn0), ((locals.var_uc_depthn_dn2 - locals.var_w_b0_dn2) - locals.var_w_s0_dn2), ((locals.var_uc_depthn_dn4 - locals.var_w_b0_dn4) - locals.var_w_s0_dn4), ((locals.var_uc_depthn_dn5 - locals.var_w_b0_dn5) - locals.var_w_s0_dn5), ((locals.var_uc_depthn_dn6 - locals.var_w_b0_dn6) - locals.var_w_s0_dn6), ((locals.var_uc_depthn_dn7 - locals.var_w_b0_dn7) - locals.var_w_s0_dn7), ((locals.var_uc_depthn_dn8 - locals.var_w_b0_dn8) - locals.var_w_s0_dn8), ((locals.var_uc_depthn_dn9 - locals.var_w_b0_dn9) - locals.var_w_s0_dn9), ((locals.var_uc_depthn_dn10 - locals.var_w_b0_dn10) - locals.var_w_s0_dn10), ((locals.var_uc_depthn_dn11 - locals.var_w_b0_dn11) - locals.var_w_s0_dn11), ((locals.var_uc_depthn_dn14 - locals.var_w_b0_dn14) - locals.var_w_s0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28080_e26948;
        locals.var_t1_dn0 = assign28080_e26948_d_n0;
        locals.var_t1_dn2 = assign28080_e26948_d_n2;
        locals.var_t1_dn4 = assign28080_e26948_d_n4;
        locals.var_t1_dn5 = assign28080_e26948_d_n5;
        locals.var_t1_dn6 = assign28080_e26948_d_n6;
        locals.var_t1_dn7 = assign28080_e26948_d_n7;
        locals.var_t1_dn8 = assign28080_e26948_d_n8;
        locals.var_t1_dn9 = assign28080_e26948_d_n9;
        locals.var_t1_dn10 = assign28080_e26948_d_n10;
        locals.var_t1_dn11 = assign28080_e26948_d_n11;
        locals.var_t1_dn14 = assign28080_e26948_d_n14;

        let assign28090_e26952: f64 = (1e-25 + 1e-18);
        let assign28090_e26957: f64 = if ((locals.var_t1 < assign28090_e26952) && (1e-18 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard665 = assign28090_e26957;

        let (assign28100_e26969, assign28100_e26969_d_n0, assign28100_e26969_d_n2, assign28100_e26969_d_n4, assign28100_e26969_d_n5, assign28100_e26969_d_n6, assign28100_e26969_d_n7, assign28100_e26969_d_n8, assign28100_e26969_d_n9, assign28100_e26969_d_n10, assign28100_e26969_d_n11, assign28100_e26969_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28100_e26965: f64 = (1e-25 + 1e-18);
        let assign28100_e26967: f64 = (assign28100_e26965 - locals.var_t1);
        (assign28100_e26967, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign28100_e26969;
        locals.var_tmf1_dn0 = assign28100_e26969_d_n0;
        locals.var_tmf1_dn2 = assign28100_e26969_d_n2;
        locals.var_tmf1_dn4 = assign28100_e26969_d_n4;
        locals.var_tmf1_dn5 = assign28100_e26969_d_n5;
        locals.var_tmf1_dn6 = assign28100_e26969_d_n6;
        locals.var_tmf1_dn7 = assign28100_e26969_d_n7;
        locals.var_tmf1_dn8 = assign28100_e26969_d_n8;
        locals.var_tmf1_dn9 = assign28100_e26969_d_n9;
        locals.var_tmf1_dn10 = assign28100_e26969_d_n10;
        locals.var_tmf1_dn11 = assign28100_e26969_d_n11;
        locals.var_tmf1_dn14 = assign28100_e26969_d_n14;

        let (assign28110_e26979, assign28110_e26979_d_n0, assign28110_e26979_d_n2, assign28110_e26979_d_n4, assign28110_e26979_d_n5, assign28110_e26979_d_n6, assign28110_e26979_d_n7, assign28110_e26979_d_n8, assign28110_e26979_d_n9, assign28110_e26979_d_n10, assign28110_e26979_d_n11, assign28110_e26979_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28110_e26977: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28110_e26977, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign28110_e26979;
        locals.var_x2_dn0 = assign28110_e26979_d_n0;
        locals.var_x2_dn2 = assign28110_e26979_d_n2;
        locals.var_x2_dn4 = assign28110_e26979_d_n4;
        locals.var_x2_dn5 = assign28110_e26979_d_n5;
        locals.var_x2_dn6 = assign28110_e26979_d_n6;
        locals.var_x2_dn7 = assign28110_e26979_d_n7;
        locals.var_x2_dn8 = assign28110_e26979_d_n8;
        locals.var_x2_dn9 = assign28110_e26979_d_n9;
        locals.var_x2_dn10 = assign28110_e26979_d_n10;
        locals.var_x2_dn11 = assign28110_e26979_d_n11;
        locals.var_x2_dn14 = assign28110_e26979_d_n14;

        let (assign28120_e26989, assign28120_e26989_d_n0, assign28120_e26989_d_n2, assign28120_e26989_d_n4, assign28120_e26989_d_n5, assign28120_e26989_d_n6, assign28120_e26989_d_n7, assign28120_e26989_d_n8, assign28120_e26989_d_n9, assign28120_e26989_d_n10, assign28120_e26989_d_n11, assign28120_e26989_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28120_e26987: f64 = (1e-18 * 1e-18);
        (assign28120_e26987, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign28120_e26989;
        locals.var_xmax2_dn0 = assign28120_e26989_d_n0;
        locals.var_xmax2_dn2 = assign28120_e26989_d_n2;
        locals.var_xmax2_dn4 = assign28120_e26989_d_n4;
        locals.var_xmax2_dn5 = assign28120_e26989_d_n5;
        locals.var_xmax2_dn6 = assign28120_e26989_d_n6;
        locals.var_xmax2_dn7 = assign28120_e26989_d_n7;
        locals.var_xmax2_dn8 = assign28120_e26989_d_n8;
        locals.var_xmax2_dn9 = assign28120_e26989_d_n9;
        locals.var_xmax2_dn10 = assign28120_e26989_d_n10;
        locals.var_xmax2_dn11 = assign28120_e26989_d_n11;
        locals.var_xmax2_dn14 = assign28120_e26989_d_n14;

        let (assign28130_e26997, assign28130_e26997_d_n0, assign28130_e26997_d_n2, assign28130_e26997_d_n4, assign28130_e26997_d_n5, assign28130_e26997_d_n6, assign28130_e26997_d_n7, assign28130_e26997_d_n8, assign28130_e26997_d_n9, assign28130_e26997_d_n10, assign28130_e26997_d_n11, assign28130_e26997_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28130_e26997;
        locals.var_xp_dn0 = assign28130_e26997_d_n0;
        locals.var_xp_dn2 = assign28130_e26997_d_n2;
        locals.var_xp_dn4 = assign28130_e26997_d_n4;
        locals.var_xp_dn5 = assign28130_e26997_d_n5;
        locals.var_xp_dn6 = assign28130_e26997_d_n6;
        locals.var_xp_dn7 = assign28130_e26997_d_n7;
        locals.var_xp_dn8 = assign28130_e26997_d_n8;
        locals.var_xp_dn9 = assign28130_e26997_d_n9;
        locals.var_xp_dn10 = assign28130_e26997_d_n10;
        locals.var_xp_dn11 = assign28130_e26997_d_n11;
        locals.var_xp_dn14 = assign28130_e26997_d_n14;

        let (assign28140_e27005, assign28140_e27005_d_n0, assign28140_e27005_d_n2, assign28140_e27005_d_n4, assign28140_e27005_d_n5, assign28140_e27005_d_n6, assign28140_e27005_d_n7, assign28140_e27005_d_n8, assign28140_e27005_d_n9, assign28140_e27005_d_n10, assign28140_e27005_d_n11, assign28140_e27005_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28140_e27005;
        locals.var_xmp_dn0 = assign28140_e27005_d_n0;
        locals.var_xmp_dn2 = assign28140_e27005_d_n2;
        locals.var_xmp_dn4 = assign28140_e27005_d_n4;
        locals.var_xmp_dn5 = assign28140_e27005_d_n5;
        locals.var_xmp_dn6 = assign28140_e27005_d_n6;
        locals.var_xmp_dn7 = assign28140_e27005_d_n7;
        locals.var_xmp_dn8 = assign28140_e27005_d_n8;
        locals.var_xmp_dn9 = assign28140_e27005_d_n9;
        locals.var_xmp_dn10 = assign28140_e27005_d_n10;
        locals.var_xmp_dn11 = assign28140_e27005_d_n11;
        locals.var_xmp_dn14 = assign28140_e27005_d_n14;

        let (assign28150_e27013,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28150_e27013;

        let (assign28160_e27021,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28160_e27021;

        let (assign28170_e27029, assign28170_e27029_d_n0, assign28170_e27029_d_n2, assign28170_e27029_d_n4, assign28170_e27029_d_n5, assign28170_e27029_d_n6, assign28170_e27029_d_n7, assign28170_e27029_d_n8, assign28170_e27029_d_n9, assign28170_e27029_d_n10, assign28170_e27029_d_n11, assign28170_e27029_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28170_e27029;
        locals.var_arg_dn0 = assign28170_e27029_d_n0;
        locals.var_arg_dn2 = assign28170_e27029_d_n2;
        locals.var_arg_dn4 = assign28170_e27029_d_n4;
        locals.var_arg_dn5 = assign28170_e27029_d_n5;
        locals.var_arg_dn6 = assign28170_e27029_d_n6;
        locals.var_arg_dn7 = assign28170_e27029_d_n7;
        locals.var_arg_dn8 = assign28170_e27029_d_n8;
        locals.var_arg_dn9 = assign28170_e27029_d_n9;
        locals.var_arg_dn10 = assign28170_e27029_d_n10;
        locals.var_arg_dn11 = assign28170_e27029_d_n11;
        locals.var_arg_dn14 = assign28170_e27029_d_n14;

        let (assign28180_e27037, assign28180_e27037_d_n0, assign28180_e27037_d_n2, assign28180_e27037_d_n4, assign28180_e27037_d_n5, assign28180_e27037_d_n6, assign28180_e27037_d_n7, assign28180_e27037_d_n8, assign28180_e27037_d_n9, assign28180_e27037_d_n10, assign28180_e27037_d_n11, assign28180_e27037_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28180_e27037;
        locals.var_dnm_dn0 = assign28180_e27037_d_n0;
        locals.var_dnm_dn2 = assign28180_e27037_d_n2;
        locals.var_dnm_dn4 = assign28180_e27037_d_n4;
        locals.var_dnm_dn5 = assign28180_e27037_d_n5;
        locals.var_dnm_dn6 = assign28180_e27037_d_n6;
        locals.var_dnm_dn7 = assign28180_e27037_d_n7;
        locals.var_dnm_dn8 = assign28180_e27037_d_n8;
        locals.var_dnm_dn9 = assign28180_e27037_d_n9;
        locals.var_dnm_dn10 = assign28180_e27037_d_n10;
        locals.var_dnm_dn11 = assign28180_e27037_d_n11;
        locals.var_dnm_dn14 = assign28180_e27037_d_n14;

        let (assign28190_e27047, assign28190_e27047_d_n0, assign28190_e27047_d_n2, assign28190_e27047_d_n4, assign28190_e27047_d_n5, assign28190_e27047_d_n6, assign28190_e27047_d_n7, assign28190_e27047_d_n8, assign28190_e27047_d_n9, assign28190_e27047_d_n10, assign28190_e27047_d_n11, assign28190_e27047_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28190_e27045: f64 = (locals.var_xp * locals.var_x2);
        (assign28190_e27045, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28190_e27047;
        locals.var_xp_dn0 = assign28190_e27047_d_n0;
        locals.var_xp_dn2 = assign28190_e27047_d_n2;
        locals.var_xp_dn4 = assign28190_e27047_d_n4;
        locals.var_xp_dn5 = assign28190_e27047_d_n5;
        locals.var_xp_dn6 = assign28190_e27047_d_n6;
        locals.var_xp_dn7 = assign28190_e27047_d_n7;
        locals.var_xp_dn8 = assign28190_e27047_d_n8;
        locals.var_xp_dn9 = assign28190_e27047_d_n9;
        locals.var_xp_dn10 = assign28190_e27047_d_n10;
        locals.var_xp_dn11 = assign28190_e27047_d_n11;
        locals.var_xp_dn14 = assign28190_e27047_d_n14;

        let (assign28200_e27057, assign28200_e27057_d_n0, assign28200_e27057_d_n2, assign28200_e27057_d_n4, assign28200_e27057_d_n5, assign28200_e27057_d_n6, assign28200_e27057_d_n7, assign28200_e27057_d_n8, assign28200_e27057_d_n9, assign28200_e27057_d_n10, assign28200_e27057_d_n11, assign28200_e27057_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28200_e27055: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28200_e27055, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28200_e27057;
        locals.var_xmp_dn0 = assign28200_e27057_d_n0;
        locals.var_xmp_dn2 = assign28200_e27057_d_n2;
        locals.var_xmp_dn4 = assign28200_e27057_d_n4;
        locals.var_xmp_dn5 = assign28200_e27057_d_n5;
        locals.var_xmp_dn6 = assign28200_e27057_d_n6;
        locals.var_xmp_dn7 = assign28200_e27057_d_n7;
        locals.var_xmp_dn8 = assign28200_e27057_d_n8;
        locals.var_xmp_dn9 = assign28200_e27057_d_n9;
        locals.var_xmp_dn10 = assign28200_e27057_d_n10;
        locals.var_xmp_dn11 = assign28200_e27057_d_n11;
        locals.var_xmp_dn14 = assign28200_e27057_d_n14;

        let (assign28210_e27067, assign28210_e27067_d_n0, assign28210_e27067_d_n2, assign28210_e27067_d_n4, assign28210_e27067_d_n5, assign28210_e27067_d_n6, assign28210_e27067_d_n7, assign28210_e27067_d_n8, assign28210_e27067_d_n9, assign28210_e27067_d_n10, assign28210_e27067_d_n11, assign28210_e27067_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28210_e27065: f64 = (locals.var_xp * locals.var_x2);
        (assign28210_e27065, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28210_e27067;
        locals.var_xp_dn0 = assign28210_e27067_d_n0;
        locals.var_xp_dn2 = assign28210_e27067_d_n2;
        locals.var_xp_dn4 = assign28210_e27067_d_n4;
        locals.var_xp_dn5 = assign28210_e27067_d_n5;
        locals.var_xp_dn6 = assign28210_e27067_d_n6;
        locals.var_xp_dn7 = assign28210_e27067_d_n7;
        locals.var_xp_dn8 = assign28210_e27067_d_n8;
        locals.var_xp_dn9 = assign28210_e27067_d_n9;
        locals.var_xp_dn10 = assign28210_e27067_d_n10;
        locals.var_xp_dn11 = assign28210_e27067_d_n11;
        locals.var_xp_dn14 = assign28210_e27067_d_n14;

        let (assign28220_e27077, assign28220_e27077_d_n0, assign28220_e27077_d_n2, assign28220_e27077_d_n4, assign28220_e27077_d_n5, assign28220_e27077_d_n6, assign28220_e27077_d_n7, assign28220_e27077_d_n8, assign28220_e27077_d_n9, assign28220_e27077_d_n10, assign28220_e27077_d_n11, assign28220_e27077_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28220_e27075: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28220_e27075, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28220_e27077;
        locals.var_xmp_dn0 = assign28220_e27077_d_n0;
        locals.var_xmp_dn2 = assign28220_e27077_d_n2;
        locals.var_xmp_dn4 = assign28220_e27077_d_n4;
        locals.var_xmp_dn5 = assign28220_e27077_d_n5;
        locals.var_xmp_dn6 = assign28220_e27077_d_n6;
        locals.var_xmp_dn7 = assign28220_e27077_d_n7;
        locals.var_xmp_dn8 = assign28220_e27077_d_n8;
        locals.var_xmp_dn9 = assign28220_e27077_d_n9;
        locals.var_xmp_dn10 = assign28220_e27077_d_n10;
        locals.var_xmp_dn11 = assign28220_e27077_d_n11;
        locals.var_xmp_dn14 = assign28220_e27077_d_n14;

        let (assign28230_e27087, assign28230_e27087_d_n0, assign28230_e27087_d_n2, assign28230_e27087_d_n4, assign28230_e27087_d_n5, assign28230_e27087_d_n6, assign28230_e27087_d_n7, assign28230_e27087_d_n8, assign28230_e27087_d_n9, assign28230_e27087_d_n10, assign28230_e27087_d_n11, assign28230_e27087_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28230_e27085: f64 = (locals.var_xp + locals.var_xmp);
        (assign28230_e27085, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28230_e27087;
        locals.var_arg_dn0 = assign28230_e27087_d_n0;
        locals.var_arg_dn2 = assign28230_e27087_d_n2;
        locals.var_arg_dn4 = assign28230_e27087_d_n4;
        locals.var_arg_dn5 = assign28230_e27087_d_n5;
        locals.var_arg_dn6 = assign28230_e27087_d_n6;
        locals.var_arg_dn7 = assign28230_e27087_d_n7;
        locals.var_arg_dn8 = assign28230_e27087_d_n8;
        locals.var_arg_dn9 = assign28230_e27087_d_n9;
        locals.var_arg_dn10 = assign28230_e27087_d_n10;
        locals.var_arg_dn11 = assign28230_e27087_d_n11;
        locals.var_arg_dn14 = assign28230_e27087_d_n14;

        let (assign28240_e27095, assign28240_e27095_d_n0, assign28240_e27095_d_n2, assign28240_e27095_d_n4, assign28240_e27095_d_n5, assign28240_e27095_d_n6, assign28240_e27095_d_n7, assign28240_e27095_d_n8, assign28240_e27095_d_n9, assign28240_e27095_d_n10, assign28240_e27095_d_n11, assign28240_e27095_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28240_e27095;
        locals.var_dnm_dn0 = assign28240_e27095_d_n0;
        locals.var_dnm_dn2 = assign28240_e27095_d_n2;
        locals.var_dnm_dn4 = assign28240_e27095_d_n4;
        locals.var_dnm_dn5 = assign28240_e27095_d_n5;
        locals.var_dnm_dn6 = assign28240_e27095_d_n6;
        locals.var_dnm_dn7 = assign28240_e27095_d_n7;
        locals.var_dnm_dn8 = assign28240_e27095_d_n8;
        locals.var_dnm_dn9 = assign28240_e27095_d_n9;
        locals.var_dnm_dn10 = assign28240_e27095_d_n10;
        locals.var_dnm_dn11 = assign28240_e27095_d_n11;
        locals.var_dnm_dn14 = assign28240_e27095_d_n14;

        let assign28250_e27110: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard666 = assign28250_e27110;

        let assign28260_e27113: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard667 = assign28260_e27113;

        let (assign28270_e27125,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28270_e27125;

        let assign28280_e27128: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard668 = assign28280_e27128;

        let (assign28290_e27143,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28290_e27143;

        let assign28300_e27146: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard669 = assign28300_e27146;

        let (assign28310_e27164,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 == 0.0)) && (locals.var_guard669 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28310_e27164;

        let assign28320_e27167: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard670 = assign28320_e27167;

        let (assign28330_e27188,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 == 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard670 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28330_e27188;

        let (assign28340_e27198,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28340_e27198;

    }

    pub(super) fn stamp_transient_block_81(
        locals: &mut StampLocals,
    ) {
        let mut assign28350_loop_guard: usize = 0;
        while {
            let assign28350_cond_e27209: f64 = if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign28350_cond_e27209 != 0.0
        } {
            assign28350_loop_guard += 1;
            assert!(assign28350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign28350_body0_e27220, assign28350_body0_e27220_d_n0, assign28350_body0_e27220_d_n2, assign28350_body0_e27220_d_n4, assign28350_body0_e27220_d_n5, assign28350_body0_e27220_d_n6, assign28350_body0_e27220_d_n7, assign28350_body0_e27220_d_n8, assign28350_body0_e27220_d_n9, assign28350_body0_e27220_d_n10, assign28350_body0_e27220_d_n11, assign28350_body0_e27220_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) {
        let assign28350_body0_e27218: f64 = (locals.var_dnm).sqrt();
        (assign28350_body0_e27218, (locals.var_dnm_dn0 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn2 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn4 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn5 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn6 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn7 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn8 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn9 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn10 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn11 / (2.0 * assign28350_body0_e27218)), (locals.var_dnm_dn14 / (2.0 * assign28350_body0_e27218)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign28350_body0_e27220;
            locals.var_dnm_dn0 = assign28350_body0_e27220_d_n0;
            locals.var_dnm_dn2 = assign28350_body0_e27220_d_n2;
            locals.var_dnm_dn4 = assign28350_body0_e27220_d_n4;
            locals.var_dnm_dn5 = assign28350_body0_e27220_d_n5;
            locals.var_dnm_dn6 = assign28350_body0_e27220_d_n6;
            locals.var_dnm_dn7 = assign28350_body0_e27220_d_n7;
            locals.var_dnm_dn8 = assign28350_body0_e27220_d_n8;
            locals.var_dnm_dn9 = assign28350_body0_e27220_d_n9;
            locals.var_dnm_dn10 = assign28350_body0_e27220_d_n10;
            locals.var_dnm_dn11 = assign28350_body0_e27220_d_n11;
            locals.var_dnm_dn14 = assign28350_body0_e27220_d_n14;
            let (assign28350_body1_e27232,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 != 0.0)) {
        let assign28350_body1_e27230: f64 = (locals.var_m0 + 1.0);
        (assign28350_body1_e27230,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign28350_body1_e27232;
        }

        let (assign28360_e27254, assign28360_e27254_d_n0, assign28360_e27254_d_n2, assign28360_e27254_d_n4, assign28360_e27254_d_n5, assign28360_e27254_d_n6, assign28360_e27254_d_n7, assign28360_e27254_d_n8, assign28360_e27254_d_n9, assign28360_e27254_d_n10, assign28360_e27254_d_n11, assign28360_e27254_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) && (locals.var_guard666 == 0.0)) {
        let (assign28360_e27252, assign28360_e27252_d_n0, assign28360_e27252_d_n2, assign28360_e27252_d_n4, assign28360_e27252_d_n5, assign28360_e27252_d_n6, assign28360_e27252_d_n7, assign28360_e27252_d_n8, assign28360_e27252_d_n9, assign28360_e27252_d_n10, assign28360_e27252_d_n11, assign28360_e27252_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign28360_e27249: f64 = (2.0 * 2.0);
                let assign28360_e27250: f64 = (1.0 / assign28360_e27249);
                let assign28360_e27251: f64 = (locals.var_dnm).powf(assign28360_e27250);
                (assign28360_e27251, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn0)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn2)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn4)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn5)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn6)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn7)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn8)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn9)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn10)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn11)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28360_e27250) as f64).is_finite() && ((assign28360_e27250) as f64).fract() == 0.0 { if assign28360_e27250 == 0.0 { 0.0 } else { (assign28360_e27250 * ((locals.var_dnm).powf(assign28360_e27250 - 1.0) * locals.var_dnm_dn14)) } } else { (assign28360_e27251 * (assign28360_e27250 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign28360_e27252, assign28360_e27252_d_n0, assign28360_e27252_d_n2, assign28360_e27252_d_n4, assign28360_e27252_d_n5, assign28360_e27252_d_n6, assign28360_e27252_d_n7, assign28360_e27252_d_n8, assign28360_e27252_d_n9, assign28360_e27252_d_n10, assign28360_e27252_d_n11, assign28360_e27252_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28360_e27254;
        locals.var_dnm_dn0 = assign28360_e27254_d_n0;
        locals.var_dnm_dn2 = assign28360_e27254_d_n2;
        locals.var_dnm_dn4 = assign28360_e27254_d_n4;
        locals.var_dnm_dn5 = assign28360_e27254_d_n5;
        locals.var_dnm_dn6 = assign28360_e27254_d_n6;
        locals.var_dnm_dn7 = assign28360_e27254_d_n7;
        locals.var_dnm_dn8 = assign28360_e27254_d_n8;
        locals.var_dnm_dn9 = assign28360_e27254_d_n9;
        locals.var_dnm_dn10 = assign28360_e27254_d_n10;
        locals.var_dnm_dn11 = assign28360_e27254_d_n11;
        locals.var_dnm_dn14 = assign28360_e27254_d_n14;

        let (assign28370_e27264, assign28370_e27264_d_n0, assign28370_e27264_d_n2, assign28370_e27264_d_n4, assign28370_e27264_d_n5, assign28370_e27264_d_n6, assign28370_e27264_d_n7, assign28370_e27264_d_n8, assign28370_e27264_d_n9, assign28370_e27264_d_n10, assign28370_e27264_d_n11, assign28370_e27264_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28370_e27262: f64 = (1.0 / locals.var_dnm);
        (assign28370_e27262, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28370_e27264;
        locals.var_dnm_dn0 = assign28370_e27264_d_n0;
        locals.var_dnm_dn2 = assign28370_e27264_d_n2;
        locals.var_dnm_dn4 = assign28370_e27264_d_n4;
        locals.var_dnm_dn5 = assign28370_e27264_d_n5;
        locals.var_dnm_dn6 = assign28370_e27264_d_n6;
        locals.var_dnm_dn7 = assign28370_e27264_d_n7;
        locals.var_dnm_dn8 = assign28370_e27264_d_n8;
        locals.var_dnm_dn9 = assign28370_e27264_d_n9;
        locals.var_dnm_dn10 = assign28370_e27264_d_n10;
        locals.var_dnm_dn11 = assign28370_e27264_d_n11;
        locals.var_dnm_dn14 = assign28370_e27264_d_n14;

        let (assign28380_e27276, assign28380_e27276_d_n0, assign28380_e27276_d_n2, assign28380_e27276_d_n4, assign28380_e27276_d_n5, assign28380_e27276_d_n6, assign28380_e27276_d_n7, assign28380_e27276_d_n8, assign28380_e27276_d_n9, assign28380_e27276_d_n10, assign28380_e27276_d_n11, assign28380_e27276_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28380_e27272: f64 = (locals.var_tmf1 * 1e-18);
        let assign28380_e27274: f64 = (assign28380_e27272 * locals.var_dnm);
        (assign28380_e27274, (((locals.var_tmf1_dn0 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-18) * locals.var_dnm) + (assign28380_e27272 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign28380_e27276;
        locals.var_tmf0_dn0 = assign28380_e27276_d_n0;
        locals.var_tmf0_dn2 = assign28380_e27276_d_n2;
        locals.var_tmf0_dn4 = assign28380_e27276_d_n4;
        locals.var_tmf0_dn5 = assign28380_e27276_d_n5;
        locals.var_tmf0_dn6 = assign28380_e27276_d_n6;
        locals.var_tmf0_dn7 = assign28380_e27276_d_n7;
        locals.var_tmf0_dn8 = assign28380_e27276_d_n8;
        locals.var_tmf0_dn9 = assign28380_e27276_d_n9;
        locals.var_tmf0_dn10 = assign28380_e27276_d_n10;
        locals.var_tmf0_dn11 = assign28380_e27276_d_n11;
        locals.var_tmf0_dn14 = assign28380_e27276_d_n14;

        let (assign28390_e27290, assign28390_e27290_d_n0, assign28390_e27290_d_n2, assign28390_e27290_d_n4, assign28390_e27290_d_n5, assign28390_e27290_d_n6, assign28390_e27290_d_n7, assign28390_e27290_d_n8, assign28390_e27290_d_n9, assign28390_e27290_d_n10, assign28390_e27290_d_n11, assign28390_e27290_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28390_e27284: f64 = (1e-18 * locals.var_xmp);
        let assign28390_e27286: f64 = (assign28390_e27284 * locals.var_dnm);
        let assign28390_e27288: f64 = (assign28390_e27286 / locals.var_arg);
        (assign28390_e27288, ((((((1e-18 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn11) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn11)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn14) * locals.var_dnm) + (assign28390_e27284 * locals.var_dnm_dn14)) * locals.var_arg) - (assign28390_e27286 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28390_e27290;
        locals.var_t0_dn0 = assign28390_e27290_d_n0;
        locals.var_t0_dn2 = assign28390_e27290_d_n2;
        locals.var_t0_dn4 = assign28390_e27290_d_n4;
        locals.var_t0_dn5 = assign28390_e27290_d_n5;
        locals.var_t0_dn6 = assign28390_e27290_d_n6;
        locals.var_t0_dn7 = assign28390_e27290_d_n7;
        locals.var_t0_dn8 = assign28390_e27290_d_n8;
        locals.var_t0_dn9 = assign28390_e27290_d_n9;
        locals.var_t0_dn10 = assign28390_e27290_d_n10;
        locals.var_t0_dn11 = assign28390_e27290_d_n11;
        locals.var_t0_dn14 = assign28390_e27290_d_n14;

        let (assign28400_e27302, assign28400_e27302_d_n0, assign28400_e27302_d_n2, assign28400_e27302_d_n4, assign28400_e27302_d_n5, assign28400_e27302_d_n6, assign28400_e27302_d_n7, assign28400_e27302_d_n8, assign28400_e27302_d_n9, assign28400_e27302_d_n10, assign28400_e27302_d_n11, assign28400_e27302_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign28400_e27298: f64 = (1e-25 + 1e-18);
        let assign28400_e27300: f64 = (assign28400_e27298 - locals.var_tmf0);
        (assign28400_e27300, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_res0, locals.var_w_res0_dn0, locals.var_w_res0_dn2, locals.var_w_res0_dn4, locals.var_w_res0_dn5, locals.var_w_res0_dn6, locals.var_w_res0_dn7, locals.var_w_res0_dn8, locals.var_w_res0_dn9, locals.var_w_res0_dn10, locals.var_w_res0_dn11, locals.var_w_res0_dn14,)
    }
};
        locals.var_w_res0 = assign28400_e27302;
        locals.var_w_res0_dn0 = assign28400_e27302_d_n0;
        locals.var_w_res0_dn2 = assign28400_e27302_d_n2;
        locals.var_w_res0_dn4 = assign28400_e27302_d_n4;
        locals.var_w_res0_dn5 = assign28400_e27302_d_n5;
        locals.var_w_res0_dn6 = assign28400_e27302_d_n6;
        locals.var_w_res0_dn7 = assign28400_e27302_d_n7;
        locals.var_w_res0_dn8 = assign28400_e27302_d_n8;
        locals.var_w_res0_dn9 = assign28400_e27302_d_n9;
        locals.var_w_res0_dn10 = assign28400_e27302_d_n10;
        locals.var_w_res0_dn11 = assign28400_e27302_d_n11;
        locals.var_w_res0_dn14 = assign28400_e27302_d_n14;

        let (assign28410_e27310, assign28410_e27310_d_n0, assign28410_e27310_d_n2, assign28410_e27310_d_n4, assign28410_e27310_d_n5, assign28410_e27310_d_n6, assign28410_e27310_d_n7, assign28410_e27310_d_n8, assign28410_e27310_d_n9, assign28410_e27310_d_n10, assign28410_e27310_d_n11, assign28410_e27310_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28410_e27310;
        locals.var_t0_dn0 = assign28410_e27310_d_n0;
        locals.var_t0_dn2 = assign28410_e27310_d_n2;
        locals.var_t0_dn4 = assign28410_e27310_d_n4;
        locals.var_t0_dn5 = assign28410_e27310_d_n5;
        locals.var_t0_dn6 = assign28410_e27310_d_n6;
        locals.var_t0_dn7 = assign28410_e27310_d_n7;
        locals.var_t0_dn8 = assign28410_e27310_d_n8;
        locals.var_t0_dn9 = assign28410_e27310_d_n9;
        locals.var_t0_dn10 = assign28410_e27310_d_n10;
        locals.var_t0_dn11 = assign28410_e27310_d_n11;
        locals.var_t0_dn14 = assign28410_e27310_d_n14;

        let (assign28420_e27319, assign28420_e27319_d_n0, assign28420_e27319_d_n2, assign28420_e27319_d_n4, assign28420_e27319_d_n5, assign28420_e27319_d_n6, assign28420_e27319_d_n7, assign28420_e27319_d_n8, assign28420_e27319_d_n9, assign28420_e27319_d_n10, assign28420_e27319_d_n11, assign28420_e27319_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_w_res0, locals.var_w_res0_dn0, locals.var_w_res0_dn2, locals.var_w_res0_dn4, locals.var_w_res0_dn5, locals.var_w_res0_dn6, locals.var_w_res0_dn7, locals.var_w_res0_dn8, locals.var_w_res0_dn9, locals.var_w_res0_dn10, locals.var_w_res0_dn11, locals.var_w_res0_dn14,)
    }
};
        locals.var_w_res0 = assign28420_e27319;
        locals.var_w_res0_dn0 = assign28420_e27319_d_n0;
        locals.var_w_res0_dn2 = assign28420_e27319_d_n2;
        locals.var_w_res0_dn4 = assign28420_e27319_d_n4;
        locals.var_w_res0_dn5 = assign28420_e27319_d_n5;
        locals.var_w_res0_dn6 = assign28420_e27319_d_n6;
        locals.var_w_res0_dn7 = assign28420_e27319_d_n7;
        locals.var_w_res0_dn8 = assign28420_e27319_d_n8;
        locals.var_w_res0_dn9 = assign28420_e27319_d_n9;
        locals.var_w_res0_dn10 = assign28420_e27319_d_n10;
        locals.var_w_res0_dn11 = assign28420_e27319_d_n11;
        locals.var_w_res0_dn14 = assign28420_e27319_d_n14;

        let (assign28430_e27328, assign28430_e27328_d_n0, assign28430_e27328_d_n2, assign28430_e27328_d_n4, assign28430_e27328_d_n5, assign28430_e27328_d_n6, assign28430_e27328_d_n7, assign28430_e27328_d_n8, assign28430_e27328_d_n9, assign28430_e27328_d_n10, assign28430_e27328_d_n11, assign28430_e27328_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard665 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28430_e27328;
        locals.var_t0_dn0 = assign28430_e27328_d_n0;
        locals.var_t0_dn2 = assign28430_e27328_d_n2;
        locals.var_t0_dn4 = assign28430_e27328_d_n4;
        locals.var_t0_dn5 = assign28430_e27328_d_n5;
        locals.var_t0_dn6 = assign28430_e27328_d_n6;
        locals.var_t0_dn7 = assign28430_e27328_d_n7;
        locals.var_t0_dn8 = assign28430_e27328_d_n8;
        locals.var_t0_dn9 = assign28430_e27328_d_n9;
        locals.var_t0_dn10 = assign28430_e27328_d_n10;
        locals.var_t0_dn11 = assign28430_e27328_d_n11;
        locals.var_t0_dn14 = assign28430_e27328_d_n14;

        let (assign28440_e27337, assign28440_e27337_d_n0, assign28440_e27337_d_n2, assign28440_e27337_d_n4, assign28440_e27337_d_n5, assign28440_e27337_d_n6, assign28440_e27337_d_n7, assign28440_e27337_d_n8, assign28440_e27337_d_n9, assign28440_e27337_d_n10, assign28440_e27337_d_n11, assign28440_e27337_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign28440_e27333: f64 = (-locals.var_w_res0);
        let assign28440_e27335: f64 = (assign28440_e27333 * locals.var_q_ndepm);
        (assign28440_e27335, (((-locals.var_w_res0_dn0) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn0)), (((-locals.var_w_res0_dn2) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn2)), (((-locals.var_w_res0_dn4) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn4)), (((-locals.var_w_res0_dn5) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn5)), (((-locals.var_w_res0_dn6) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn6)), (((-locals.var_w_res0_dn7) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn7)), (((-locals.var_w_res0_dn8) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn8)), (((-locals.var_w_res0_dn9) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn9)), (((-locals.var_w_res0_dn10) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn10)), (((-locals.var_w_res0_dn11) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn11)), (((-locals.var_w_res0_dn14) * locals.var_q_ndepm) + (assign28440_e27333 * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_qn_res0, locals.var_qn_res0_dn0, locals.var_qn_res0_dn2, locals.var_qn_res0_dn4, locals.var_qn_res0_dn5, locals.var_qn_res0_dn6, locals.var_qn_res0_dn7, locals.var_qn_res0_dn8, locals.var_qn_res0_dn9, locals.var_qn_res0_dn10, locals.var_qn_res0_dn11, locals.var_qn_res0_dn14,)
    }
};
        locals.var_qn_res0 = assign28440_e27337;
        locals.var_qn_res0_dn0 = assign28440_e27337_d_n0;
        locals.var_qn_res0_dn2 = assign28440_e27337_d_n2;
        locals.var_qn_res0_dn4 = assign28440_e27337_d_n4;
        locals.var_qn_res0_dn5 = assign28440_e27337_d_n5;
        locals.var_qn_res0_dn6 = assign28440_e27337_d_n6;
        locals.var_qn_res0_dn7 = assign28440_e27337_d_n7;
        locals.var_qn_res0_dn8 = assign28440_e27337_d_n8;
        locals.var_qn_res0_dn9 = assign28440_e27337_d_n9;
        locals.var_qn_res0_dn10 = assign28440_e27337_d_n10;
        locals.var_qn_res0_dn11 = assign28440_e27337_d_n11;
        locals.var_qn_res0_dn14 = assign28440_e27337_d_n14;

        let assign28450_e27344: f64 = if ((locals.var_w_bsub0 > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard671 = assign28450_e27344;

        let assign28460_e27348: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28460_e27353: f64 = if ((locals.var_phi_s0_dep > assign28460_e27348) && (0.8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard672 = assign28460_e27353;

        let (assign28470_e27367, assign28470_e27367_d_n0, assign28470_e27367_d_n2, assign28470_e27367_d_n4, assign28470_e27367_d_n5, assign28470_e27367_d_n6, assign28470_e27367_d_n7, assign28470_e27367_d_n8, assign28470_e27367_d_n9, assign28470_e27367_d_n10, assign28470_e27367_d_n11, assign28470_e27367_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28470_e27363: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign28470_e27365: f64 = (assign28470_e27363 + 0.8);
        (assign28470_e27365, (locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phi_s0_dep_dn11 - locals.var_vds_maxb0_dn11), (locals.var_phi_s0_dep_dn14 - locals.var_vds_maxb0_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign28470_e27367;
        locals.var_tmf1_dn0 = assign28470_e27367_d_n0;
        locals.var_tmf1_dn2 = assign28470_e27367_d_n2;
        locals.var_tmf1_dn4 = assign28470_e27367_d_n4;
        locals.var_tmf1_dn5 = assign28470_e27367_d_n5;
        locals.var_tmf1_dn6 = assign28470_e27367_d_n6;
        locals.var_tmf1_dn7 = assign28470_e27367_d_n7;
        locals.var_tmf1_dn8 = assign28470_e27367_d_n8;
        locals.var_tmf1_dn9 = assign28470_e27367_d_n9;
        locals.var_tmf1_dn10 = assign28470_e27367_d_n10;
        locals.var_tmf1_dn11 = assign28470_e27367_d_n11;
        locals.var_tmf1_dn14 = assign28470_e27367_d_n14;

        let (assign28480_e27379, assign28480_e27379_d_n0, assign28480_e27379_d_n2, assign28480_e27379_d_n4, assign28480_e27379_d_n5, assign28480_e27379_d_n6, assign28480_e27379_d_n7, assign28480_e27379_d_n8, assign28480_e27379_d_n9, assign28480_e27379_d_n10, assign28480_e27379_d_n11, assign28480_e27379_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28480_e27377: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28480_e27377, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign28480_e27379;
        locals.var_x2_dn0 = assign28480_e27379_d_n0;
        locals.var_x2_dn2 = assign28480_e27379_d_n2;
        locals.var_x2_dn4 = assign28480_e27379_d_n4;
        locals.var_x2_dn5 = assign28480_e27379_d_n5;
        locals.var_x2_dn6 = assign28480_e27379_d_n6;
        locals.var_x2_dn7 = assign28480_e27379_d_n7;
        locals.var_x2_dn8 = assign28480_e27379_d_n8;
        locals.var_x2_dn9 = assign28480_e27379_d_n9;
        locals.var_x2_dn10 = assign28480_e27379_d_n10;
        locals.var_x2_dn11 = assign28480_e27379_d_n11;
        locals.var_x2_dn14 = assign28480_e27379_d_n14;

        let (assign28490_e27391, assign28490_e27391_d_n0, assign28490_e27391_d_n2, assign28490_e27391_d_n4, assign28490_e27391_d_n5, assign28490_e27391_d_n6, assign28490_e27391_d_n7, assign28490_e27391_d_n8, assign28490_e27391_d_n9, assign28490_e27391_d_n10, assign28490_e27391_d_n11, assign28490_e27391_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28490_e27389: f64 = (0.8 * 0.8);
        (assign28490_e27389, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign28490_e27391;
        locals.var_xmax2_dn0 = assign28490_e27391_d_n0;
        locals.var_xmax2_dn2 = assign28490_e27391_d_n2;
        locals.var_xmax2_dn4 = assign28490_e27391_d_n4;
        locals.var_xmax2_dn5 = assign28490_e27391_d_n5;
        locals.var_xmax2_dn6 = assign28490_e27391_d_n6;
        locals.var_xmax2_dn7 = assign28490_e27391_d_n7;
        locals.var_xmax2_dn8 = assign28490_e27391_d_n8;
        locals.var_xmax2_dn9 = assign28490_e27391_d_n9;
        locals.var_xmax2_dn10 = assign28490_e27391_d_n10;
        locals.var_xmax2_dn11 = assign28490_e27391_d_n11;
        locals.var_xmax2_dn14 = assign28490_e27391_d_n14;

        let (assign28500_e27401, assign28500_e27401_d_n0, assign28500_e27401_d_n2, assign28500_e27401_d_n4, assign28500_e27401_d_n5, assign28500_e27401_d_n6, assign28500_e27401_d_n7, assign28500_e27401_d_n8, assign28500_e27401_d_n9, assign28500_e27401_d_n10, assign28500_e27401_d_n11, assign28500_e27401_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28500_e27401;
        locals.var_xp_dn0 = assign28500_e27401_d_n0;
        locals.var_xp_dn2 = assign28500_e27401_d_n2;
        locals.var_xp_dn4 = assign28500_e27401_d_n4;
        locals.var_xp_dn5 = assign28500_e27401_d_n5;
        locals.var_xp_dn6 = assign28500_e27401_d_n6;
        locals.var_xp_dn7 = assign28500_e27401_d_n7;
        locals.var_xp_dn8 = assign28500_e27401_d_n8;
        locals.var_xp_dn9 = assign28500_e27401_d_n9;
        locals.var_xp_dn10 = assign28500_e27401_d_n10;
        locals.var_xp_dn11 = assign28500_e27401_d_n11;
        locals.var_xp_dn14 = assign28500_e27401_d_n14;

        let (assign28510_e27411, assign28510_e27411_d_n0, assign28510_e27411_d_n2, assign28510_e27411_d_n4, assign28510_e27411_d_n5, assign28510_e27411_d_n6, assign28510_e27411_d_n7, assign28510_e27411_d_n8, assign28510_e27411_d_n9, assign28510_e27411_d_n10, assign28510_e27411_d_n11, assign28510_e27411_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28510_e27411;
        locals.var_xmp_dn0 = assign28510_e27411_d_n0;
        locals.var_xmp_dn2 = assign28510_e27411_d_n2;
        locals.var_xmp_dn4 = assign28510_e27411_d_n4;
        locals.var_xmp_dn5 = assign28510_e27411_d_n5;
        locals.var_xmp_dn6 = assign28510_e27411_d_n6;
        locals.var_xmp_dn7 = assign28510_e27411_d_n7;
        locals.var_xmp_dn8 = assign28510_e27411_d_n8;
        locals.var_xmp_dn9 = assign28510_e27411_d_n9;
        locals.var_xmp_dn10 = assign28510_e27411_d_n10;
        locals.var_xmp_dn11 = assign28510_e27411_d_n11;
        locals.var_xmp_dn14 = assign28510_e27411_d_n14;

        let (assign28520_e27421,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28520_e27421;

        let (assign28530_e27431,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28530_e27431;

        let (assign28540_e27441, assign28540_e27441_d_n0, assign28540_e27441_d_n2, assign28540_e27441_d_n4, assign28540_e27441_d_n5, assign28540_e27441_d_n6, assign28540_e27441_d_n7, assign28540_e27441_d_n8, assign28540_e27441_d_n9, assign28540_e27441_d_n10, assign28540_e27441_d_n11, assign28540_e27441_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28540_e27441;
        locals.var_arg_dn0 = assign28540_e27441_d_n0;
        locals.var_arg_dn2 = assign28540_e27441_d_n2;
        locals.var_arg_dn4 = assign28540_e27441_d_n4;
        locals.var_arg_dn5 = assign28540_e27441_d_n5;
        locals.var_arg_dn6 = assign28540_e27441_d_n6;
        locals.var_arg_dn7 = assign28540_e27441_d_n7;
        locals.var_arg_dn8 = assign28540_e27441_d_n8;
        locals.var_arg_dn9 = assign28540_e27441_d_n9;
        locals.var_arg_dn10 = assign28540_e27441_d_n10;
        locals.var_arg_dn11 = assign28540_e27441_d_n11;
        locals.var_arg_dn14 = assign28540_e27441_d_n14;

        let (assign28550_e27451, assign28550_e27451_d_n0, assign28550_e27451_d_n2, assign28550_e27451_d_n4, assign28550_e27451_d_n5, assign28550_e27451_d_n6, assign28550_e27451_d_n7, assign28550_e27451_d_n8, assign28550_e27451_d_n9, assign28550_e27451_d_n10, assign28550_e27451_d_n11, assign28550_e27451_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28550_e27451;
        locals.var_dnm_dn0 = assign28550_e27451_d_n0;
        locals.var_dnm_dn2 = assign28550_e27451_d_n2;
        locals.var_dnm_dn4 = assign28550_e27451_d_n4;
        locals.var_dnm_dn5 = assign28550_e27451_d_n5;
        locals.var_dnm_dn6 = assign28550_e27451_d_n6;
        locals.var_dnm_dn7 = assign28550_e27451_d_n7;
        locals.var_dnm_dn8 = assign28550_e27451_d_n8;
        locals.var_dnm_dn9 = assign28550_e27451_d_n9;
        locals.var_dnm_dn10 = assign28550_e27451_d_n10;
        locals.var_dnm_dn11 = assign28550_e27451_d_n11;
        locals.var_dnm_dn14 = assign28550_e27451_d_n14;

        let (assign28560_e27463, assign28560_e27463_d_n0, assign28560_e27463_d_n2, assign28560_e27463_d_n4, assign28560_e27463_d_n5, assign28560_e27463_d_n6, assign28560_e27463_d_n7, assign28560_e27463_d_n8, assign28560_e27463_d_n9, assign28560_e27463_d_n10, assign28560_e27463_d_n11, assign28560_e27463_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28560_e27461: f64 = (locals.var_xp * locals.var_x2);
        (assign28560_e27461, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28560_e27463;
        locals.var_xp_dn0 = assign28560_e27463_d_n0;
        locals.var_xp_dn2 = assign28560_e27463_d_n2;
        locals.var_xp_dn4 = assign28560_e27463_d_n4;
        locals.var_xp_dn5 = assign28560_e27463_d_n5;
        locals.var_xp_dn6 = assign28560_e27463_d_n6;
        locals.var_xp_dn7 = assign28560_e27463_d_n7;
        locals.var_xp_dn8 = assign28560_e27463_d_n8;
        locals.var_xp_dn9 = assign28560_e27463_d_n9;
        locals.var_xp_dn10 = assign28560_e27463_d_n10;
        locals.var_xp_dn11 = assign28560_e27463_d_n11;
        locals.var_xp_dn14 = assign28560_e27463_d_n14;

        let (assign28570_e27475, assign28570_e27475_d_n0, assign28570_e27475_d_n2, assign28570_e27475_d_n4, assign28570_e27475_d_n5, assign28570_e27475_d_n6, assign28570_e27475_d_n7, assign28570_e27475_d_n8, assign28570_e27475_d_n9, assign28570_e27475_d_n10, assign28570_e27475_d_n11, assign28570_e27475_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28570_e27473: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28570_e27473, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28570_e27475;
        locals.var_xmp_dn0 = assign28570_e27475_d_n0;
        locals.var_xmp_dn2 = assign28570_e27475_d_n2;
        locals.var_xmp_dn4 = assign28570_e27475_d_n4;
        locals.var_xmp_dn5 = assign28570_e27475_d_n5;
        locals.var_xmp_dn6 = assign28570_e27475_d_n6;
        locals.var_xmp_dn7 = assign28570_e27475_d_n7;
        locals.var_xmp_dn8 = assign28570_e27475_d_n8;
        locals.var_xmp_dn9 = assign28570_e27475_d_n9;
        locals.var_xmp_dn10 = assign28570_e27475_d_n10;
        locals.var_xmp_dn11 = assign28570_e27475_d_n11;
        locals.var_xmp_dn14 = assign28570_e27475_d_n14;

        let (assign28580_e27487, assign28580_e27487_d_n0, assign28580_e27487_d_n2, assign28580_e27487_d_n4, assign28580_e27487_d_n5, assign28580_e27487_d_n6, assign28580_e27487_d_n7, assign28580_e27487_d_n8, assign28580_e27487_d_n9, assign28580_e27487_d_n10, assign28580_e27487_d_n11, assign28580_e27487_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28580_e27485: f64 = (locals.var_xp * locals.var_x2);
        (assign28580_e27485, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28580_e27487;
        locals.var_xp_dn0 = assign28580_e27487_d_n0;
        locals.var_xp_dn2 = assign28580_e27487_d_n2;
        locals.var_xp_dn4 = assign28580_e27487_d_n4;
        locals.var_xp_dn5 = assign28580_e27487_d_n5;
        locals.var_xp_dn6 = assign28580_e27487_d_n6;
        locals.var_xp_dn7 = assign28580_e27487_d_n7;
        locals.var_xp_dn8 = assign28580_e27487_d_n8;
        locals.var_xp_dn9 = assign28580_e27487_d_n9;
        locals.var_xp_dn10 = assign28580_e27487_d_n10;
        locals.var_xp_dn11 = assign28580_e27487_d_n11;
        locals.var_xp_dn14 = assign28580_e27487_d_n14;

        let (assign28590_e27499, assign28590_e27499_d_n0, assign28590_e27499_d_n2, assign28590_e27499_d_n4, assign28590_e27499_d_n5, assign28590_e27499_d_n6, assign28590_e27499_d_n7, assign28590_e27499_d_n8, assign28590_e27499_d_n9, assign28590_e27499_d_n10, assign28590_e27499_d_n11, assign28590_e27499_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28590_e27497: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28590_e27497, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28590_e27499;
        locals.var_xmp_dn0 = assign28590_e27499_d_n0;
        locals.var_xmp_dn2 = assign28590_e27499_d_n2;
        locals.var_xmp_dn4 = assign28590_e27499_d_n4;
        locals.var_xmp_dn5 = assign28590_e27499_d_n5;
        locals.var_xmp_dn6 = assign28590_e27499_d_n6;
        locals.var_xmp_dn7 = assign28590_e27499_d_n7;
        locals.var_xmp_dn8 = assign28590_e27499_d_n8;
        locals.var_xmp_dn9 = assign28590_e27499_d_n9;
        locals.var_xmp_dn10 = assign28590_e27499_d_n10;
        locals.var_xmp_dn11 = assign28590_e27499_d_n11;
        locals.var_xmp_dn14 = assign28590_e27499_d_n14;

        let (assign28600_e27511, assign28600_e27511_d_n0, assign28600_e27511_d_n2, assign28600_e27511_d_n4, assign28600_e27511_d_n5, assign28600_e27511_d_n6, assign28600_e27511_d_n7, assign28600_e27511_d_n8, assign28600_e27511_d_n9, assign28600_e27511_d_n10, assign28600_e27511_d_n11, assign28600_e27511_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28600_e27509: f64 = (locals.var_xp + locals.var_xmp);
        (assign28600_e27509, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28600_e27511;
        locals.var_arg_dn0 = assign28600_e27511_d_n0;
        locals.var_arg_dn2 = assign28600_e27511_d_n2;
        locals.var_arg_dn4 = assign28600_e27511_d_n4;
        locals.var_arg_dn5 = assign28600_e27511_d_n5;
        locals.var_arg_dn6 = assign28600_e27511_d_n6;
        locals.var_arg_dn7 = assign28600_e27511_d_n7;
        locals.var_arg_dn8 = assign28600_e27511_d_n8;
        locals.var_arg_dn9 = assign28600_e27511_d_n9;
        locals.var_arg_dn10 = assign28600_e27511_d_n10;
        locals.var_arg_dn11 = assign28600_e27511_d_n11;
        locals.var_arg_dn14 = assign28600_e27511_d_n14;

    }

    pub(super) fn stamp_transient_block_82(
        locals: &mut StampLocals,
    ) {
        let (assign28610_e27521, assign28610_e27521_d_n0, assign28610_e27521_d_n2, assign28610_e27521_d_n4, assign28610_e27521_d_n5, assign28610_e27521_d_n6, assign28610_e27521_d_n7, assign28610_e27521_d_n8, assign28610_e27521_d_n9, assign28610_e27521_d_n10, assign28610_e27521_d_n11, assign28610_e27521_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28610_e27521;
        locals.var_dnm_dn0 = assign28610_e27521_d_n0;
        locals.var_dnm_dn2 = assign28610_e27521_d_n2;
        locals.var_dnm_dn4 = assign28610_e27521_d_n4;
        locals.var_dnm_dn5 = assign28610_e27521_d_n5;
        locals.var_dnm_dn6 = assign28610_e27521_d_n6;
        locals.var_dnm_dn7 = assign28610_e27521_d_n7;
        locals.var_dnm_dn8 = assign28610_e27521_d_n8;
        locals.var_dnm_dn9 = assign28610_e27521_d_n9;
        locals.var_dnm_dn10 = assign28610_e27521_d_n10;
        locals.var_dnm_dn11 = assign28610_e27521_d_n11;
        locals.var_dnm_dn14 = assign28610_e27521_d_n14;

        let assign28620_e27536: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard673 = assign28620_e27536;

        let assign28630_e27539: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard674 = assign28630_e27539;

        let (assign28640_e27553,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28640_e27553;

        let assign28650_e27556: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard675 = assign28650_e27556;

        let (assign28660_e27573,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 == 0.0)) && (locals.var_guard675 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28660_e27573;

        let assign28670_e27576: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard676 = assign28670_e27576;

        let (assign28680_e27596,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 == 0.0)) && (locals.var_guard675 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28680_e27596;

        let assign28690_e27599: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard677 = assign28690_e27599;

        let (assign28700_e27622,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 == 0.0)) && (locals.var_guard675 == 0.0)) && (locals.var_guard676 == 0.0)) && (locals.var_guard677 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28700_e27622;

        let (assign28710_e27634,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28710_e27634;

        let mut assign28720_loop_guard: usize = 0;
        while {
            let assign28720_cond_e27647: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign28720_cond_e27647 != 0.0
        } {
            assign28720_loop_guard += 1;
            assert!(assign28720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign28720_body0_e27660, assign28720_body0_e27660_d_n0, assign28720_body0_e27660_d_n2, assign28720_body0_e27660_d_n4, assign28720_body0_e27660_d_n5, assign28720_body0_e27660_d_n6, assign28720_body0_e27660_d_n7, assign28720_body0_e27660_d_n8, assign28720_body0_e27660_d_n9, assign28720_body0_e27660_d_n10, assign28720_body0_e27660_d_n11, assign28720_body0_e27660_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) {
        let assign28720_body0_e27658: f64 = (locals.var_dnm).sqrt();
        (assign28720_body0_e27658, (locals.var_dnm_dn0 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn2 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn4 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn5 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn6 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn7 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn8 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn9 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn10 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn11 / (2.0 * assign28720_body0_e27658)), (locals.var_dnm_dn14 / (2.0 * assign28720_body0_e27658)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign28720_body0_e27660;
            locals.var_dnm_dn0 = assign28720_body0_e27660_d_n0;
            locals.var_dnm_dn2 = assign28720_body0_e27660_d_n2;
            locals.var_dnm_dn4 = assign28720_body0_e27660_d_n4;
            locals.var_dnm_dn5 = assign28720_body0_e27660_d_n5;
            locals.var_dnm_dn6 = assign28720_body0_e27660_d_n6;
            locals.var_dnm_dn7 = assign28720_body0_e27660_d_n7;
            locals.var_dnm_dn8 = assign28720_body0_e27660_d_n8;
            locals.var_dnm_dn9 = assign28720_body0_e27660_d_n9;
            locals.var_dnm_dn10 = assign28720_body0_e27660_d_n10;
            locals.var_dnm_dn11 = assign28720_body0_e27660_d_n11;
            locals.var_dnm_dn14 = assign28720_body0_e27660_d_n14;
            let (assign28720_body1_e27674,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) {
        let assign28720_body1_e27672: f64 = (locals.var_m0 + 1.0);
        (assign28720_body1_e27672,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign28720_body1_e27674;
        }

        let (assign28730_e27698, assign28730_e27698_d_n0, assign28730_e27698_d_n2, assign28730_e27698_d_n4, assign28730_e27698_d_n5, assign28730_e27698_d_n6, assign28730_e27698_d_n7, assign28730_e27698_d_n8, assign28730_e27698_d_n9, assign28730_e27698_d_n10, assign28730_e27698_d_n11, assign28730_e27698_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 == 0.0)) {
        let (assign28730_e27696, assign28730_e27696_d_n0, assign28730_e27696_d_n2, assign28730_e27696_d_n4, assign28730_e27696_d_n5, assign28730_e27696_d_n6, assign28730_e27696_d_n7, assign28730_e27696_d_n8, assign28730_e27696_d_n9, assign28730_e27696_d_n10, assign28730_e27696_d_n11, assign28730_e27696_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign28730_e27693: f64 = (2.0 * 2.0);
                let assign28730_e27694: f64 = (1.0 / assign28730_e27693);
                let assign28730_e27695: f64 = (locals.var_dnm).powf(assign28730_e27694);
                (assign28730_e27695, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn0)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn2)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn4)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn5)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn6)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn7)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn8)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn9)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn10)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn11)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28730_e27694) as f64).is_finite() && ((assign28730_e27694) as f64).fract() == 0.0 { if assign28730_e27694 == 0.0 { 0.0 } else { (assign28730_e27694 * ((locals.var_dnm).powf(assign28730_e27694 - 1.0) * locals.var_dnm_dn14)) } } else { (assign28730_e27695 * (assign28730_e27694 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign28730_e27696, assign28730_e27696_d_n0, assign28730_e27696_d_n2, assign28730_e27696_d_n4, assign28730_e27696_d_n5, assign28730_e27696_d_n6, assign28730_e27696_d_n7, assign28730_e27696_d_n8, assign28730_e27696_d_n9, assign28730_e27696_d_n10, assign28730_e27696_d_n11, assign28730_e27696_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28730_e27698;
        locals.var_dnm_dn0 = assign28730_e27698_d_n0;
        locals.var_dnm_dn2 = assign28730_e27698_d_n2;
        locals.var_dnm_dn4 = assign28730_e27698_d_n4;
        locals.var_dnm_dn5 = assign28730_e27698_d_n5;
        locals.var_dnm_dn6 = assign28730_e27698_d_n6;
        locals.var_dnm_dn7 = assign28730_e27698_d_n7;
        locals.var_dnm_dn8 = assign28730_e27698_d_n8;
        locals.var_dnm_dn9 = assign28730_e27698_d_n9;
        locals.var_dnm_dn10 = assign28730_e27698_d_n10;
        locals.var_dnm_dn11 = assign28730_e27698_d_n11;
        locals.var_dnm_dn14 = assign28730_e27698_d_n14;

        let (assign28740_e27710, assign28740_e27710_d_n0, assign28740_e27710_d_n2, assign28740_e27710_d_n4, assign28740_e27710_d_n5, assign28740_e27710_d_n6, assign28740_e27710_d_n7, assign28740_e27710_d_n8, assign28740_e27710_d_n9, assign28740_e27710_d_n10, assign28740_e27710_d_n11, assign28740_e27710_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28740_e27708: f64 = (1.0 / locals.var_dnm);
        (assign28740_e27708, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28740_e27710;
        locals.var_dnm_dn0 = assign28740_e27710_d_n0;
        locals.var_dnm_dn2 = assign28740_e27710_d_n2;
        locals.var_dnm_dn4 = assign28740_e27710_d_n4;
        locals.var_dnm_dn5 = assign28740_e27710_d_n5;
        locals.var_dnm_dn6 = assign28740_e27710_d_n6;
        locals.var_dnm_dn7 = assign28740_e27710_d_n7;
        locals.var_dnm_dn8 = assign28740_e27710_d_n8;
        locals.var_dnm_dn9 = assign28740_e27710_d_n9;
        locals.var_dnm_dn10 = assign28740_e27710_d_n10;
        locals.var_dnm_dn11 = assign28740_e27710_d_n11;
        locals.var_dnm_dn14 = assign28740_e27710_d_n14;

        let (assign28750_e27724, assign28750_e27724_d_n0, assign28750_e27724_d_n2, assign28750_e27724_d_n4, assign28750_e27724_d_n5, assign28750_e27724_d_n6, assign28750_e27724_d_n7, assign28750_e27724_d_n8, assign28750_e27724_d_n9, assign28750_e27724_d_n10, assign28750_e27724_d_n11, assign28750_e27724_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28750_e27720: f64 = (locals.var_tmf1 * 0.8);
        let assign28750_e27722: f64 = (assign28750_e27720 * locals.var_dnm);
        (assign28750_e27722, (((locals.var_tmf1_dn0 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.8) * locals.var_dnm) + (assign28750_e27720 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign28750_e27724;
        locals.var_tmf0_dn0 = assign28750_e27724_d_n0;
        locals.var_tmf0_dn2 = assign28750_e27724_d_n2;
        locals.var_tmf0_dn4 = assign28750_e27724_d_n4;
        locals.var_tmf0_dn5 = assign28750_e27724_d_n5;
        locals.var_tmf0_dn6 = assign28750_e27724_d_n6;
        locals.var_tmf0_dn7 = assign28750_e27724_d_n7;
        locals.var_tmf0_dn8 = assign28750_e27724_d_n8;
        locals.var_tmf0_dn9 = assign28750_e27724_d_n9;
        locals.var_tmf0_dn10 = assign28750_e27724_d_n10;
        locals.var_tmf0_dn11 = assign28750_e27724_d_n11;
        locals.var_tmf0_dn14 = assign28750_e27724_d_n14;

        let (assign28760_e27740, assign28760_e27740_d_n0, assign28760_e27740_d_n2, assign28760_e27740_d_n4, assign28760_e27740_d_n5, assign28760_e27740_d_n6, assign28760_e27740_d_n7, assign28760_e27740_d_n8, assign28760_e27740_d_n9, assign28760_e27740_d_n10, assign28760_e27740_d_n11, assign28760_e27740_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28760_e27734: f64 = (0.8 * locals.var_xmp);
        let assign28760_e27736: f64 = (assign28760_e27734 * locals.var_dnm);
        let assign28760_e27738: f64 = (assign28760_e27736 / locals.var_arg);
        (assign28760_e27738, ((((((0.8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn11)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign28760_e27734 * locals.var_dnm_dn14)) * locals.var_arg) - (assign28760_e27736 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28760_e27740;
        locals.var_t1_dn0 = assign28760_e27740_d_n0;
        locals.var_t1_dn2 = assign28760_e27740_d_n2;
        locals.var_t1_dn4 = assign28760_e27740_d_n4;
        locals.var_t1_dn5 = assign28760_e27740_d_n5;
        locals.var_t1_dn6 = assign28760_e27740_d_n6;
        locals.var_t1_dn7 = assign28760_e27740_d_n7;
        locals.var_t1_dn8 = assign28760_e27740_d_n8;
        locals.var_t1_dn9 = assign28760_e27740_d_n9;
        locals.var_t1_dn10 = assign28760_e27740_d_n10;
        locals.var_t1_dn11 = assign28760_e27740_d_n11;
        locals.var_t1_dn14 = assign28760_e27740_d_n14;

        let (assign28770_e27754, assign28770_e27754_d_n0, assign28770_e27754_d_n2, assign28770_e27754_d_n4, assign28770_e27754_d_n5, assign28770_e27754_d_n6, assign28770_e27754_d_n7, assign28770_e27754_d_n8, assign28770_e27754_d_n9, assign28770_e27754_d_n10, assign28770_e27754_d_n11, assign28770_e27754_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        let assign28770_e27750: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28770_e27752: f64 = (assign28770_e27750 + locals.var_tmf0);
        (assign28770_e27752, (locals.var_vds_maxb0_dn0 + locals.var_tmf0_dn0), (locals.var_vds_maxb0_dn2 + locals.var_tmf0_dn2), (locals.var_vds_maxb0_dn4 + locals.var_tmf0_dn4), (locals.var_vds_maxb0_dn5 + locals.var_tmf0_dn5), (locals.var_vds_maxb0_dn6 + locals.var_tmf0_dn6), (locals.var_vds_maxb0_dn7 + locals.var_tmf0_dn7), (locals.var_vds_maxb0_dn8 + locals.var_tmf0_dn8), (locals.var_vds_maxb0_dn9 + locals.var_tmf0_dn9), (locals.var_vds_maxb0_dn10 + locals.var_tmf0_dn10), (locals.var_vds_maxb0_dn11 + locals.var_tmf0_dn11), (locals.var_vds_maxb0_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28770_e27754;
        locals.var_t2_dn0 = assign28770_e27754_d_n0;
        locals.var_t2_dn2 = assign28770_e27754_d_n2;
        locals.var_t2_dn4 = assign28770_e27754_d_n4;
        locals.var_t2_dn5 = assign28770_e27754_d_n5;
        locals.var_t2_dn6 = assign28770_e27754_d_n6;
        locals.var_t2_dn7 = assign28770_e27754_d_n7;
        locals.var_t2_dn8 = assign28770_e27754_d_n8;
        locals.var_t2_dn9 = assign28770_e27754_d_n9;
        locals.var_t2_dn10 = assign28770_e27754_d_n10;
        locals.var_t2_dn11 = assign28770_e27754_d_n11;
        locals.var_t2_dn14 = assign28770_e27754_d_n14;

        let (assign28780_e27764, assign28780_e27764_d_n0, assign28780_e27764_d_n2, assign28780_e27764_d_n4, assign28780_e27764_d_n5, assign28780_e27764_d_n6, assign28780_e27764_d_n7, assign28780_e27764_d_n8, assign28780_e27764_d_n9, assign28780_e27764_d_n10, assign28780_e27764_d_n11, assign28780_e27764_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28780_e27764;
        locals.var_t1_dn0 = assign28780_e27764_d_n0;
        locals.var_t1_dn2 = assign28780_e27764_d_n2;
        locals.var_t1_dn4 = assign28780_e27764_d_n4;
        locals.var_t1_dn5 = assign28780_e27764_d_n5;
        locals.var_t1_dn6 = assign28780_e27764_d_n6;
        locals.var_t1_dn7 = assign28780_e27764_d_n7;
        locals.var_t1_dn8 = assign28780_e27764_d_n8;
        locals.var_t1_dn9 = assign28780_e27764_d_n9;
        locals.var_t1_dn10 = assign28780_e27764_d_n10;
        locals.var_t1_dn11 = assign28780_e27764_d_n11;
        locals.var_t1_dn14 = assign28780_e27764_d_n14;

        let (assign28790_e27775, assign28790_e27775_d_n0, assign28790_e27775_d_n2, assign28790_e27775_d_n4, assign28790_e27775_d_n5, assign28790_e27775_d_n6, assign28790_e27775_d_n7, assign28790_e27775_d_n8, assign28790_e27775_d_n9, assign28790_e27775_d_n10, assign28790_e27775_d_n11, assign28790_e27775_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 == 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28790_e27775;
        locals.var_t2_dn0 = assign28790_e27775_d_n0;
        locals.var_t2_dn2 = assign28790_e27775_d_n2;
        locals.var_t2_dn4 = assign28790_e27775_d_n4;
        locals.var_t2_dn5 = assign28790_e27775_d_n5;
        locals.var_t2_dn6 = assign28790_e27775_d_n6;
        locals.var_t2_dn7 = assign28790_e27775_d_n7;
        locals.var_t2_dn8 = assign28790_e27775_d_n8;
        locals.var_t2_dn9 = assign28790_e27775_d_n9;
        locals.var_t2_dn10 = assign28790_e27775_d_n10;
        locals.var_t2_dn11 = assign28790_e27775_d_n11;
        locals.var_t2_dn14 = assign28790_e27775_d_n14;

        let (assign28800_e27786, assign28800_e27786_d_n0, assign28800_e27786_d_n2, assign28800_e27786_d_n4, assign28800_e27786_d_n5, assign28800_e27786_d_n6, assign28800_e27786_d_n7, assign28800_e27786_d_n8, assign28800_e27786_d_n9, assign28800_e27786_d_n10, assign28800_e27786_d_n11, assign28800_e27786_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28800_e27786;
        locals.var_t1_dn0 = assign28800_e27786_d_n0;
        locals.var_t1_dn2 = assign28800_e27786_d_n2;
        locals.var_t1_dn4 = assign28800_e27786_d_n4;
        locals.var_t1_dn5 = assign28800_e27786_d_n5;
        locals.var_t1_dn6 = assign28800_e27786_d_n6;
        locals.var_t1_dn7 = assign28800_e27786_d_n7;
        locals.var_t1_dn8 = assign28800_e27786_d_n8;
        locals.var_t1_dn9 = assign28800_e27786_d_n9;
        locals.var_t1_dn10 = assign28800_e27786_d_n10;
        locals.var_t1_dn11 = assign28800_e27786_d_n11;
        locals.var_t1_dn14 = assign28800_e27786_d_n14;

        let assign28810_e27790: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28810_e27795: f64 = if ((locals.var_phib_ref > assign28810_e27790) && (0.8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard678 = assign28810_e27795;

        let (assign28820_e27810, assign28820_e27810_d_n0, assign28820_e27810_d_n2, assign28820_e27810_d_n4, assign28820_e27810_d_n5, assign28820_e27810_d_n6, assign28820_e27810_d_n7, assign28820_e27810_d_n8, assign28820_e27810_d_n9, assign28820_e27810_d_n10, assign28820_e27810_d_n11, assign28820_e27810_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28820_e27806: f64 = (locals.var_phib_ref - locals.var_vds_maxb0);
        let assign28820_e27808: f64 = (assign28820_e27806 + 0.8);
        (assign28820_e27808, (locals.var_phib_ref_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phib_ref_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phib_ref_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phib_ref_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phib_ref_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phib_ref_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phib_ref_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phib_ref_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phib_ref_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phib_ref_dn11 - locals.var_vds_maxb0_dn11), (locals.var_phib_ref_dn14 - locals.var_vds_maxb0_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign28820_e27810;
        locals.var_tmf1_dn0 = assign28820_e27810_d_n0;
        locals.var_tmf1_dn2 = assign28820_e27810_d_n2;
        locals.var_tmf1_dn4 = assign28820_e27810_d_n4;
        locals.var_tmf1_dn5 = assign28820_e27810_d_n5;
        locals.var_tmf1_dn6 = assign28820_e27810_d_n6;
        locals.var_tmf1_dn7 = assign28820_e27810_d_n7;
        locals.var_tmf1_dn8 = assign28820_e27810_d_n8;
        locals.var_tmf1_dn9 = assign28820_e27810_d_n9;
        locals.var_tmf1_dn10 = assign28820_e27810_d_n10;
        locals.var_tmf1_dn11 = assign28820_e27810_d_n11;
        locals.var_tmf1_dn14 = assign28820_e27810_d_n14;

        let (assign28830_e27823, assign28830_e27823_d_n0, assign28830_e27823_d_n2, assign28830_e27823_d_n4, assign28830_e27823_d_n5, assign28830_e27823_d_n6, assign28830_e27823_d_n7, assign28830_e27823_d_n8, assign28830_e27823_d_n9, assign28830_e27823_d_n10, assign28830_e27823_d_n11, assign28830_e27823_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28830_e27821: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28830_e27821, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign28830_e27823;
        locals.var_x2_dn0 = assign28830_e27823_d_n0;
        locals.var_x2_dn2 = assign28830_e27823_d_n2;
        locals.var_x2_dn4 = assign28830_e27823_d_n4;
        locals.var_x2_dn5 = assign28830_e27823_d_n5;
        locals.var_x2_dn6 = assign28830_e27823_d_n6;
        locals.var_x2_dn7 = assign28830_e27823_d_n7;
        locals.var_x2_dn8 = assign28830_e27823_d_n8;
        locals.var_x2_dn9 = assign28830_e27823_d_n9;
        locals.var_x2_dn10 = assign28830_e27823_d_n10;
        locals.var_x2_dn11 = assign28830_e27823_d_n11;
        locals.var_x2_dn14 = assign28830_e27823_d_n14;

        let (assign28840_e27836, assign28840_e27836_d_n0, assign28840_e27836_d_n2, assign28840_e27836_d_n4, assign28840_e27836_d_n5, assign28840_e27836_d_n6, assign28840_e27836_d_n7, assign28840_e27836_d_n8, assign28840_e27836_d_n9, assign28840_e27836_d_n10, assign28840_e27836_d_n11, assign28840_e27836_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28840_e27834: f64 = (0.8 * 0.8);
        (assign28840_e27834, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign28840_e27836;
        locals.var_xmax2_dn0 = assign28840_e27836_d_n0;
        locals.var_xmax2_dn2 = assign28840_e27836_d_n2;
        locals.var_xmax2_dn4 = assign28840_e27836_d_n4;
        locals.var_xmax2_dn5 = assign28840_e27836_d_n5;
        locals.var_xmax2_dn6 = assign28840_e27836_d_n6;
        locals.var_xmax2_dn7 = assign28840_e27836_d_n7;
        locals.var_xmax2_dn8 = assign28840_e27836_d_n8;
        locals.var_xmax2_dn9 = assign28840_e27836_d_n9;
        locals.var_xmax2_dn10 = assign28840_e27836_d_n10;
        locals.var_xmax2_dn11 = assign28840_e27836_d_n11;
        locals.var_xmax2_dn14 = assign28840_e27836_d_n14;

        let (assign28850_e27847, assign28850_e27847_d_n0, assign28850_e27847_d_n2, assign28850_e27847_d_n4, assign28850_e27847_d_n5, assign28850_e27847_d_n6, assign28850_e27847_d_n7, assign28850_e27847_d_n8, assign28850_e27847_d_n9, assign28850_e27847_d_n10, assign28850_e27847_d_n11, assign28850_e27847_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28850_e27847;
        locals.var_xp_dn0 = assign28850_e27847_d_n0;
        locals.var_xp_dn2 = assign28850_e27847_d_n2;
        locals.var_xp_dn4 = assign28850_e27847_d_n4;
        locals.var_xp_dn5 = assign28850_e27847_d_n5;
        locals.var_xp_dn6 = assign28850_e27847_d_n6;
        locals.var_xp_dn7 = assign28850_e27847_d_n7;
        locals.var_xp_dn8 = assign28850_e27847_d_n8;
        locals.var_xp_dn9 = assign28850_e27847_d_n9;
        locals.var_xp_dn10 = assign28850_e27847_d_n10;
        locals.var_xp_dn11 = assign28850_e27847_d_n11;
        locals.var_xp_dn14 = assign28850_e27847_d_n14;

        let (assign28860_e27858, assign28860_e27858_d_n0, assign28860_e27858_d_n2, assign28860_e27858_d_n4, assign28860_e27858_d_n5, assign28860_e27858_d_n6, assign28860_e27858_d_n7, assign28860_e27858_d_n8, assign28860_e27858_d_n9, assign28860_e27858_d_n10, assign28860_e27858_d_n11, assign28860_e27858_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28860_e27858;
        locals.var_xmp_dn0 = assign28860_e27858_d_n0;
        locals.var_xmp_dn2 = assign28860_e27858_d_n2;
        locals.var_xmp_dn4 = assign28860_e27858_d_n4;
        locals.var_xmp_dn5 = assign28860_e27858_d_n5;
        locals.var_xmp_dn6 = assign28860_e27858_d_n6;
        locals.var_xmp_dn7 = assign28860_e27858_d_n7;
        locals.var_xmp_dn8 = assign28860_e27858_d_n8;
        locals.var_xmp_dn9 = assign28860_e27858_d_n9;
        locals.var_xmp_dn10 = assign28860_e27858_d_n10;
        locals.var_xmp_dn11 = assign28860_e27858_d_n11;
        locals.var_xmp_dn14 = assign28860_e27858_d_n14;

        let (assign28870_e27869,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28870_e27869;

        let (assign28880_e27880,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28880_e27880;

        let (assign28890_e27891, assign28890_e27891_d_n0, assign28890_e27891_d_n2, assign28890_e27891_d_n4, assign28890_e27891_d_n5, assign28890_e27891_d_n6, assign28890_e27891_d_n7, assign28890_e27891_d_n8, assign28890_e27891_d_n9, assign28890_e27891_d_n10, assign28890_e27891_d_n11, assign28890_e27891_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28890_e27891;
        locals.var_arg_dn0 = assign28890_e27891_d_n0;
        locals.var_arg_dn2 = assign28890_e27891_d_n2;
        locals.var_arg_dn4 = assign28890_e27891_d_n4;
        locals.var_arg_dn5 = assign28890_e27891_d_n5;
        locals.var_arg_dn6 = assign28890_e27891_d_n6;
        locals.var_arg_dn7 = assign28890_e27891_d_n7;
        locals.var_arg_dn8 = assign28890_e27891_d_n8;
        locals.var_arg_dn9 = assign28890_e27891_d_n9;
        locals.var_arg_dn10 = assign28890_e27891_d_n10;
        locals.var_arg_dn11 = assign28890_e27891_d_n11;
        locals.var_arg_dn14 = assign28890_e27891_d_n14;

        let (assign28900_e27902, assign28900_e27902_d_n0, assign28900_e27902_d_n2, assign28900_e27902_d_n4, assign28900_e27902_d_n5, assign28900_e27902_d_n6, assign28900_e27902_d_n7, assign28900_e27902_d_n8, assign28900_e27902_d_n9, assign28900_e27902_d_n10, assign28900_e27902_d_n11, assign28900_e27902_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28900_e27902;
        locals.var_dnm_dn0 = assign28900_e27902_d_n0;
        locals.var_dnm_dn2 = assign28900_e27902_d_n2;
        locals.var_dnm_dn4 = assign28900_e27902_d_n4;
        locals.var_dnm_dn5 = assign28900_e27902_d_n5;
        locals.var_dnm_dn6 = assign28900_e27902_d_n6;
        locals.var_dnm_dn7 = assign28900_e27902_d_n7;
        locals.var_dnm_dn8 = assign28900_e27902_d_n8;
        locals.var_dnm_dn9 = assign28900_e27902_d_n9;
        locals.var_dnm_dn10 = assign28900_e27902_d_n10;
        locals.var_dnm_dn11 = assign28900_e27902_d_n11;
        locals.var_dnm_dn14 = assign28900_e27902_d_n14;

        let (assign28910_e27915, assign28910_e27915_d_n0, assign28910_e27915_d_n2, assign28910_e27915_d_n4, assign28910_e27915_d_n5, assign28910_e27915_d_n6, assign28910_e27915_d_n7, assign28910_e27915_d_n8, assign28910_e27915_d_n9, assign28910_e27915_d_n10, assign28910_e27915_d_n11, assign28910_e27915_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28910_e27913: f64 = (locals.var_xp * locals.var_x2);
        (assign28910_e27913, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28910_e27915;
        locals.var_xp_dn0 = assign28910_e27915_d_n0;
        locals.var_xp_dn2 = assign28910_e27915_d_n2;
        locals.var_xp_dn4 = assign28910_e27915_d_n4;
        locals.var_xp_dn5 = assign28910_e27915_d_n5;
        locals.var_xp_dn6 = assign28910_e27915_d_n6;
        locals.var_xp_dn7 = assign28910_e27915_d_n7;
        locals.var_xp_dn8 = assign28910_e27915_d_n8;
        locals.var_xp_dn9 = assign28910_e27915_d_n9;
        locals.var_xp_dn10 = assign28910_e27915_d_n10;
        locals.var_xp_dn11 = assign28910_e27915_d_n11;
        locals.var_xp_dn14 = assign28910_e27915_d_n14;

        let (assign28920_e27928, assign28920_e27928_d_n0, assign28920_e27928_d_n2, assign28920_e27928_d_n4, assign28920_e27928_d_n5, assign28920_e27928_d_n6, assign28920_e27928_d_n7, assign28920_e27928_d_n8, assign28920_e27928_d_n9, assign28920_e27928_d_n10, assign28920_e27928_d_n11, assign28920_e27928_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28920_e27926: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28920_e27926, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28920_e27928;
        locals.var_xmp_dn0 = assign28920_e27928_d_n0;
        locals.var_xmp_dn2 = assign28920_e27928_d_n2;
        locals.var_xmp_dn4 = assign28920_e27928_d_n4;
        locals.var_xmp_dn5 = assign28920_e27928_d_n5;
        locals.var_xmp_dn6 = assign28920_e27928_d_n6;
        locals.var_xmp_dn7 = assign28920_e27928_d_n7;
        locals.var_xmp_dn8 = assign28920_e27928_d_n8;
        locals.var_xmp_dn9 = assign28920_e27928_d_n9;
        locals.var_xmp_dn10 = assign28920_e27928_d_n10;
        locals.var_xmp_dn11 = assign28920_e27928_d_n11;
        locals.var_xmp_dn14 = assign28920_e27928_d_n14;

    }

    pub(super) fn stamp_transient_block_83(
        locals: &mut StampLocals,
    ) {
        let (assign28930_e27941, assign28930_e27941_d_n0, assign28930_e27941_d_n2, assign28930_e27941_d_n4, assign28930_e27941_d_n5, assign28930_e27941_d_n6, assign28930_e27941_d_n7, assign28930_e27941_d_n8, assign28930_e27941_d_n9, assign28930_e27941_d_n10, assign28930_e27941_d_n11, assign28930_e27941_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28930_e27939: f64 = (locals.var_xp * locals.var_x2);
        (assign28930_e27939, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28930_e27941;
        locals.var_xp_dn0 = assign28930_e27941_d_n0;
        locals.var_xp_dn2 = assign28930_e27941_d_n2;
        locals.var_xp_dn4 = assign28930_e27941_d_n4;
        locals.var_xp_dn5 = assign28930_e27941_d_n5;
        locals.var_xp_dn6 = assign28930_e27941_d_n6;
        locals.var_xp_dn7 = assign28930_e27941_d_n7;
        locals.var_xp_dn8 = assign28930_e27941_d_n8;
        locals.var_xp_dn9 = assign28930_e27941_d_n9;
        locals.var_xp_dn10 = assign28930_e27941_d_n10;
        locals.var_xp_dn11 = assign28930_e27941_d_n11;
        locals.var_xp_dn14 = assign28930_e27941_d_n14;

        let (assign28940_e27954, assign28940_e27954_d_n0, assign28940_e27954_d_n2, assign28940_e27954_d_n4, assign28940_e27954_d_n5, assign28940_e27954_d_n6, assign28940_e27954_d_n7, assign28940_e27954_d_n8, assign28940_e27954_d_n9, assign28940_e27954_d_n10, assign28940_e27954_d_n11, assign28940_e27954_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28940_e27952: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28940_e27952, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28940_e27954;
        locals.var_xmp_dn0 = assign28940_e27954_d_n0;
        locals.var_xmp_dn2 = assign28940_e27954_d_n2;
        locals.var_xmp_dn4 = assign28940_e27954_d_n4;
        locals.var_xmp_dn5 = assign28940_e27954_d_n5;
        locals.var_xmp_dn6 = assign28940_e27954_d_n6;
        locals.var_xmp_dn7 = assign28940_e27954_d_n7;
        locals.var_xmp_dn8 = assign28940_e27954_d_n8;
        locals.var_xmp_dn9 = assign28940_e27954_d_n9;
        locals.var_xmp_dn10 = assign28940_e27954_d_n10;
        locals.var_xmp_dn11 = assign28940_e27954_d_n11;
        locals.var_xmp_dn14 = assign28940_e27954_d_n14;

        let (assign28950_e27967, assign28950_e27967_d_n0, assign28950_e27967_d_n2, assign28950_e27967_d_n4, assign28950_e27967_d_n5, assign28950_e27967_d_n6, assign28950_e27967_d_n7, assign28950_e27967_d_n8, assign28950_e27967_d_n9, assign28950_e27967_d_n10, assign28950_e27967_d_n11, assign28950_e27967_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign28950_e27965: f64 = (locals.var_xp + locals.var_xmp);
        (assign28950_e27965, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28950_e27967;
        locals.var_arg_dn0 = assign28950_e27967_d_n0;
        locals.var_arg_dn2 = assign28950_e27967_d_n2;
        locals.var_arg_dn4 = assign28950_e27967_d_n4;
        locals.var_arg_dn5 = assign28950_e27967_d_n5;
        locals.var_arg_dn6 = assign28950_e27967_d_n6;
        locals.var_arg_dn7 = assign28950_e27967_d_n7;
        locals.var_arg_dn8 = assign28950_e27967_d_n8;
        locals.var_arg_dn9 = assign28950_e27967_d_n9;
        locals.var_arg_dn10 = assign28950_e27967_d_n10;
        locals.var_arg_dn11 = assign28950_e27967_d_n11;
        locals.var_arg_dn14 = assign28950_e27967_d_n14;

        let (assign28960_e27978, assign28960_e27978_d_n0, assign28960_e27978_d_n2, assign28960_e27978_d_n4, assign28960_e27978_d_n5, assign28960_e27978_d_n6, assign28960_e27978_d_n7, assign28960_e27978_d_n8, assign28960_e27978_d_n9, assign28960_e27978_d_n10, assign28960_e27978_d_n11, assign28960_e27978_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28960_e27978;
        locals.var_dnm_dn0 = assign28960_e27978_d_n0;
        locals.var_dnm_dn2 = assign28960_e27978_d_n2;
        locals.var_dnm_dn4 = assign28960_e27978_d_n4;
        locals.var_dnm_dn5 = assign28960_e27978_d_n5;
        locals.var_dnm_dn6 = assign28960_e27978_d_n6;
        locals.var_dnm_dn7 = assign28960_e27978_d_n7;
        locals.var_dnm_dn8 = assign28960_e27978_d_n8;
        locals.var_dnm_dn9 = assign28960_e27978_d_n9;
        locals.var_dnm_dn10 = assign28960_e27978_d_n10;
        locals.var_dnm_dn11 = assign28960_e27978_d_n11;
        locals.var_dnm_dn14 = assign28960_e27978_d_n14;

        let assign28970_e27993: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard679 = assign28970_e27993;

        let assign28980_e27996: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard680 = assign28980_e27996;

        let (assign28990_e28011,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28990_e28011;

        let assign29000_e28014: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard681 = assign29000_e28014;

        let (assign29010_e28032,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29010_e28032;

        let assign29020_e28035: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard682 = assign29020_e28035;

        let (assign29030_e28056,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29030_e28056;

        let assign29040_e28059: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard683 = assign29040_e28059;

        let (assign29050_e28083,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 == 0.0)) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29050_e28083;

        let (assign29060_e28096,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29060_e28096;

        let mut assign29070_loop_guard: usize = 0;
        while {
            let assign29070_cond_e28110: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29070_cond_e28110 != 0.0
        } {
            assign29070_loop_guard += 1;
            assert!(assign29070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29070_body0_e28124, assign29070_body0_e28124_d_n0, assign29070_body0_e28124_d_n2, assign29070_body0_e28124_d_n4, assign29070_body0_e28124_d_n5, assign29070_body0_e28124_d_n6, assign29070_body0_e28124_d_n7, assign29070_body0_e28124_d_n8, assign29070_body0_e28124_d_n9, assign29070_body0_e28124_d_n10, assign29070_body0_e28124_d_n11, assign29070_body0_e28124_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) {
        let assign29070_body0_e28122: f64 = (locals.var_dnm).sqrt();
        (assign29070_body0_e28122, (locals.var_dnm_dn0 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn2 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn4 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn5 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn6 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn7 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn8 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn9 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn10 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn11 / (2.0 * assign29070_body0_e28122)), (locals.var_dnm_dn14 / (2.0 * assign29070_body0_e28122)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign29070_body0_e28124;
            locals.var_dnm_dn0 = assign29070_body0_e28124_d_n0;
            locals.var_dnm_dn2 = assign29070_body0_e28124_d_n2;
            locals.var_dnm_dn4 = assign29070_body0_e28124_d_n4;
            locals.var_dnm_dn5 = assign29070_body0_e28124_d_n5;
            locals.var_dnm_dn6 = assign29070_body0_e28124_d_n6;
            locals.var_dnm_dn7 = assign29070_body0_e28124_d_n7;
            locals.var_dnm_dn8 = assign29070_body0_e28124_d_n8;
            locals.var_dnm_dn9 = assign29070_body0_e28124_d_n9;
            locals.var_dnm_dn10 = assign29070_body0_e28124_d_n10;
            locals.var_dnm_dn11 = assign29070_body0_e28124_d_n11;
            locals.var_dnm_dn14 = assign29070_body0_e28124_d_n14;
            let (assign29070_body1_e28139,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) {
        let assign29070_body1_e28137: f64 = (locals.var_m0 + 1.0);
        (assign29070_body1_e28137,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29070_body1_e28139;
        }

        let (assign29080_e28164, assign29080_e28164_d_n0, assign29080_e28164_d_n2, assign29080_e28164_d_n4, assign29080_e28164_d_n5, assign29080_e28164_d_n6, assign29080_e28164_d_n7, assign29080_e28164_d_n8, assign29080_e28164_d_n9, assign29080_e28164_d_n10, assign29080_e28164_d_n11, assign29080_e28164_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 == 0.0)) {
        let (assign29080_e28162, assign29080_e28162_d_n0, assign29080_e28162_d_n2, assign29080_e28162_d_n4, assign29080_e28162_d_n5, assign29080_e28162_d_n6, assign29080_e28162_d_n7, assign29080_e28162_d_n8, assign29080_e28162_d_n9, assign29080_e28162_d_n10, assign29080_e28162_d_n11, assign29080_e28162_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29080_e28159: f64 = (2.0 * 2.0);
                let assign29080_e28160: f64 = (1.0 / assign29080_e28159);
                let assign29080_e28161: f64 = (locals.var_dnm).powf(assign29080_e28160);
                (assign29080_e28161, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn11)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29080_e28160) as f64).is_finite() && ((assign29080_e28160) as f64).fract() == 0.0 { if assign29080_e28160 == 0.0 { 0.0 } else { (assign29080_e28160 * ((locals.var_dnm).powf(assign29080_e28160 - 1.0) * locals.var_dnm_dn14)) } } else { (assign29080_e28161 * (assign29080_e28160 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign29080_e28162, assign29080_e28162_d_n0, assign29080_e28162_d_n2, assign29080_e28162_d_n4, assign29080_e28162_d_n5, assign29080_e28162_d_n6, assign29080_e28162_d_n7, assign29080_e28162_d_n8, assign29080_e28162_d_n9, assign29080_e28162_d_n10, assign29080_e28162_d_n11, assign29080_e28162_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29080_e28164;
        locals.var_dnm_dn0 = assign29080_e28164_d_n0;
        locals.var_dnm_dn2 = assign29080_e28164_d_n2;
        locals.var_dnm_dn4 = assign29080_e28164_d_n4;
        locals.var_dnm_dn5 = assign29080_e28164_d_n5;
        locals.var_dnm_dn6 = assign29080_e28164_d_n6;
        locals.var_dnm_dn7 = assign29080_e28164_d_n7;
        locals.var_dnm_dn8 = assign29080_e28164_d_n8;
        locals.var_dnm_dn9 = assign29080_e28164_d_n9;
        locals.var_dnm_dn10 = assign29080_e28164_d_n10;
        locals.var_dnm_dn11 = assign29080_e28164_d_n11;
        locals.var_dnm_dn14 = assign29080_e28164_d_n14;

        let (assign29090_e28177, assign29090_e28177_d_n0, assign29090_e28177_d_n2, assign29090_e28177_d_n4, assign29090_e28177_d_n5, assign29090_e28177_d_n6, assign29090_e28177_d_n7, assign29090_e28177_d_n8, assign29090_e28177_d_n9, assign29090_e28177_d_n10, assign29090_e28177_d_n11, assign29090_e28177_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign29090_e28175: f64 = (1.0 / locals.var_dnm);
        (assign29090_e28175, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29090_e28177;
        locals.var_dnm_dn0 = assign29090_e28177_d_n0;
        locals.var_dnm_dn2 = assign29090_e28177_d_n2;
        locals.var_dnm_dn4 = assign29090_e28177_d_n4;
        locals.var_dnm_dn5 = assign29090_e28177_d_n5;
        locals.var_dnm_dn6 = assign29090_e28177_d_n6;
        locals.var_dnm_dn7 = assign29090_e28177_d_n7;
        locals.var_dnm_dn8 = assign29090_e28177_d_n8;
        locals.var_dnm_dn9 = assign29090_e28177_d_n9;
        locals.var_dnm_dn10 = assign29090_e28177_d_n10;
        locals.var_dnm_dn11 = assign29090_e28177_d_n11;
        locals.var_dnm_dn14 = assign29090_e28177_d_n14;

        let (assign29100_e28192, assign29100_e28192_d_n0, assign29100_e28192_d_n2, assign29100_e28192_d_n4, assign29100_e28192_d_n5, assign29100_e28192_d_n6, assign29100_e28192_d_n7, assign29100_e28192_d_n8, assign29100_e28192_d_n9, assign29100_e28192_d_n10, assign29100_e28192_d_n11, assign29100_e28192_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign29100_e28188: f64 = (locals.var_tmf1 * 0.8);
        let assign29100_e28190: f64 = (assign29100_e28188 * locals.var_dnm);
        (assign29100_e28190, (((locals.var_tmf1_dn0 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.8) * locals.var_dnm) + (assign29100_e28188 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign29100_e28192;
        locals.var_tmf0_dn0 = assign29100_e28192_d_n0;
        locals.var_tmf0_dn2 = assign29100_e28192_d_n2;
        locals.var_tmf0_dn4 = assign29100_e28192_d_n4;
        locals.var_tmf0_dn5 = assign29100_e28192_d_n5;
        locals.var_tmf0_dn6 = assign29100_e28192_d_n6;
        locals.var_tmf0_dn7 = assign29100_e28192_d_n7;
        locals.var_tmf0_dn8 = assign29100_e28192_d_n8;
        locals.var_tmf0_dn9 = assign29100_e28192_d_n9;
        locals.var_tmf0_dn10 = assign29100_e28192_d_n10;
        locals.var_tmf0_dn11 = assign29100_e28192_d_n11;
        locals.var_tmf0_dn14 = assign29100_e28192_d_n14;

        let (assign29110_e28209, assign29110_e28209_d_n0, assign29110_e28209_d_n2, assign29110_e28209_d_n4, assign29110_e28209_d_n5, assign29110_e28209_d_n6, assign29110_e28209_d_n7, assign29110_e28209_d_n8, assign29110_e28209_d_n9, assign29110_e28209_d_n10, assign29110_e28209_d_n11, assign29110_e28209_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign29110_e28203: f64 = (0.8 * locals.var_xmp);
        let assign29110_e28205: f64 = (assign29110_e28203 * locals.var_dnm);
        let assign29110_e28207: f64 = (assign29110_e28205 / locals.var_arg);
        (assign29110_e28207, ((((((0.8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn11)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign29110_e28203 * locals.var_dnm_dn14)) * locals.var_arg) - (assign29110_e28205 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29110_e28209;
        locals.var_t0_dn0 = assign29110_e28209_d_n0;
        locals.var_t0_dn2 = assign29110_e28209_d_n2;
        locals.var_t0_dn4 = assign29110_e28209_d_n4;
        locals.var_t0_dn5 = assign29110_e28209_d_n5;
        locals.var_t0_dn6 = assign29110_e28209_d_n6;
        locals.var_t0_dn7 = assign29110_e28209_d_n7;
        locals.var_t0_dn8 = assign29110_e28209_d_n8;
        locals.var_t0_dn9 = assign29110_e28209_d_n9;
        locals.var_t0_dn10 = assign29110_e28209_d_n10;
        locals.var_t0_dn11 = assign29110_e28209_d_n11;
        locals.var_t0_dn14 = assign29110_e28209_d_n14;

        let (assign29120_e28224, assign29120_e28224_d_n0, assign29120_e28224_d_n2, assign29120_e28224_d_n4, assign29120_e28224_d_n5, assign29120_e28224_d_n6, assign29120_e28224_d_n7, assign29120_e28224_d_n8, assign29120_e28224_d_n9, assign29120_e28224_d_n10, assign29120_e28224_d_n11, assign29120_e28224_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign29120_e28220: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign29120_e28222: f64 = (assign29120_e28220 + locals.var_tmf0);
        (assign29120_e28222, (locals.var_vds_maxb0_dn0 + locals.var_tmf0_dn0), (locals.var_vds_maxb0_dn2 + locals.var_tmf0_dn2), (locals.var_vds_maxb0_dn4 + locals.var_tmf0_dn4), (locals.var_vds_maxb0_dn5 + locals.var_tmf0_dn5), (locals.var_vds_maxb0_dn6 + locals.var_tmf0_dn6), (locals.var_vds_maxb0_dn7 + locals.var_tmf0_dn7), (locals.var_vds_maxb0_dn8 + locals.var_tmf0_dn8), (locals.var_vds_maxb0_dn9 + locals.var_tmf0_dn9), (locals.var_vds_maxb0_dn10 + locals.var_tmf0_dn10), (locals.var_vds_maxb0_dn11 + locals.var_tmf0_dn11), (locals.var_vds_maxb0_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29120_e28224;
        locals.var_t2_dn0 = assign29120_e28224_d_n0;
        locals.var_t2_dn2 = assign29120_e28224_d_n2;
        locals.var_t2_dn4 = assign29120_e28224_d_n4;
        locals.var_t2_dn5 = assign29120_e28224_d_n5;
        locals.var_t2_dn6 = assign29120_e28224_d_n6;
        locals.var_t2_dn7 = assign29120_e28224_d_n7;
        locals.var_t2_dn8 = assign29120_e28224_d_n8;
        locals.var_t2_dn9 = assign29120_e28224_d_n9;
        locals.var_t2_dn10 = assign29120_e28224_d_n10;
        locals.var_t2_dn11 = assign29120_e28224_d_n11;
        locals.var_t2_dn14 = assign29120_e28224_d_n14;

        let (assign29130_e28235, assign29130_e28235_d_n0, assign29130_e28235_d_n2, assign29130_e28235_d_n4, assign29130_e28235_d_n5, assign29130_e28235_d_n6, assign29130_e28235_d_n7, assign29130_e28235_d_n8, assign29130_e28235_d_n9, assign29130_e28235_d_n10, assign29130_e28235_d_n11, assign29130_e28235_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29130_e28235;
        locals.var_t0_dn0 = assign29130_e28235_d_n0;
        locals.var_t0_dn2 = assign29130_e28235_d_n2;
        locals.var_t0_dn4 = assign29130_e28235_d_n4;
        locals.var_t0_dn5 = assign29130_e28235_d_n5;
        locals.var_t0_dn6 = assign29130_e28235_d_n6;
        locals.var_t0_dn7 = assign29130_e28235_d_n7;
        locals.var_t0_dn8 = assign29130_e28235_d_n8;
        locals.var_t0_dn9 = assign29130_e28235_d_n9;
        locals.var_t0_dn10 = assign29130_e28235_d_n10;
        locals.var_t0_dn11 = assign29130_e28235_d_n11;
        locals.var_t0_dn14 = assign29130_e28235_d_n14;

        let (assign29140_e28247, assign29140_e28247_d_n0, assign29140_e28247_d_n2, assign29140_e28247_d_n4, assign29140_e28247_d_n5, assign29140_e28247_d_n6, assign29140_e28247_d_n7, assign29140_e28247_d_n8, assign29140_e28247_d_n9, assign29140_e28247_d_n10, assign29140_e28247_d_n11, assign29140_e28247_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 == 0.0)) {
        (locals.var_phib_ref, locals.var_phib_ref_dn0, locals.var_phib_ref_dn2, locals.var_phib_ref_dn4, locals.var_phib_ref_dn5, locals.var_phib_ref_dn6, locals.var_phib_ref_dn7, locals.var_phib_ref_dn8, locals.var_phib_ref_dn9, locals.var_phib_ref_dn10, locals.var_phib_ref_dn11, locals.var_phib_ref_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29140_e28247;
        locals.var_t2_dn0 = assign29140_e28247_d_n0;
        locals.var_t2_dn2 = assign29140_e28247_d_n2;
        locals.var_t2_dn4 = assign29140_e28247_d_n4;
        locals.var_t2_dn5 = assign29140_e28247_d_n5;
        locals.var_t2_dn6 = assign29140_e28247_d_n6;
        locals.var_t2_dn7 = assign29140_e28247_d_n7;
        locals.var_t2_dn8 = assign29140_e28247_d_n8;
        locals.var_t2_dn9 = assign29140_e28247_d_n9;
        locals.var_t2_dn10 = assign29140_e28247_d_n10;
        locals.var_t2_dn11 = assign29140_e28247_d_n11;
        locals.var_t2_dn14 = assign29140_e28247_d_n14;

        let (assign29150_e28259, assign29150_e28259_d_n0, assign29150_e28259_d_n2, assign29150_e28259_d_n4, assign29150_e28259_d_n5, assign29150_e28259_d_n6, assign29150_e28259_d_n7, assign29150_e28259_d_n8, assign29150_e28259_d_n9, assign29150_e28259_d_n10, assign29150_e28259_d_n11, assign29150_e28259_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard671 == 0.0)) && (locals.var_guard678 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29150_e28259;
        locals.var_t0_dn0 = assign29150_e28259_d_n0;
        locals.var_t0_dn2 = assign29150_e28259_d_n2;
        locals.var_t0_dn4 = assign29150_e28259_d_n4;
        locals.var_t0_dn5 = assign29150_e28259_d_n5;
        locals.var_t0_dn6 = assign29150_e28259_d_n6;
        locals.var_t0_dn7 = assign29150_e28259_d_n7;
        locals.var_t0_dn8 = assign29150_e28259_d_n8;
        locals.var_t0_dn9 = assign29150_e28259_d_n9;
        locals.var_t0_dn10 = assign29150_e28259_d_n10;
        locals.var_t0_dn11 = assign29150_e28259_d_n11;
        locals.var_t0_dn14 = assign29150_e28259_d_n14;

        let (assign29160_e28277, assign29160_e28277_d_n0, assign29160_e28277_d_n2, assign29160_e28277_d_n4, assign29160_e28277_d_n5, assign29160_e28277_d_n6, assign29160_e28277_d_n7, assign29160_e28277_d_n8, assign29160_e28277_d_n9, assign29160_e28277_d_n10, assign29160_e28277_d_n11, assign29160_e28277_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign29160_e28264: f64 = (-1.6021918e-19);
        let assign29160_e28266: f64 = (assign29160_e28264 * locals.var_uc_ndepm);
        let assign29160_e28270: f64 = (locals.var_t2 - locals.var_vds_maxb0);
        let assign29160_e28271: f64 = (locals.var_beta * assign29160_e28270);
        let assign29160_e28272: f64 = (assign29160_e28271).exp();
        let assign29160_e28273: f64 = (assign29160_e28266 * assign29160_e28272);
        let assign29160_e28275: f64 = (assign29160_e28273 * locals.var_w_b0);
        (assign29160_e28275, (((((assign29160_e28264 * locals.var_uc_ndepm_dn0) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn0 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn0 - locals.var_vds_maxb0_dn0)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn0)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn2) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn2 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn2 - locals.var_vds_maxb0_dn2)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn2)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn4) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn4 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn4 - locals.var_vds_maxb0_dn4)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn4)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn5) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn5 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn5 - locals.var_vds_maxb0_dn5)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn5)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn6) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn6 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn6 - locals.var_vds_maxb0_dn6)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn6)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn7) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn7 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn7 - locals.var_vds_maxb0_dn7)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn7)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn8) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn8 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn8 - locals.var_vds_maxb0_dn8)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn8)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn9) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn9 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn9 - locals.var_vds_maxb0_dn9)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn9)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn10) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn10 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn10 - locals.var_vds_maxb0_dn10)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn10)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn11) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn11 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn11 - locals.var_vds_maxb0_dn11)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn11)), (((((assign29160_e28264 * locals.var_uc_ndepm_dn14) * assign29160_e28272) + (assign29160_e28266 * (assign29160_e28272 * ((locals.var_beta_dn14 * assign29160_e28270) + (locals.var_beta * (locals.var_t2_dn14 - locals.var_vds_maxb0_dn14)))))) * locals.var_w_b0) + (assign29160_e28273 * locals.var_w_b0_dn14)),)
    } else {
        (locals.var_qn_bac, locals.var_qn_bac_dn0, locals.var_qn_bac_dn2, locals.var_qn_bac_dn4, locals.var_qn_bac_dn5, locals.var_qn_bac_dn6, locals.var_qn_bac_dn7, locals.var_qn_bac_dn8, locals.var_qn_bac_dn9, locals.var_qn_bac_dn10, locals.var_qn_bac_dn11, locals.var_qn_bac_dn14,)
    }
};
        locals.var_qn_bac = assign29160_e28277;
        locals.var_qn_bac_dn0 = assign29160_e28277_d_n0;
        locals.var_qn_bac_dn2 = assign29160_e28277_d_n2;
        locals.var_qn_bac_dn4 = assign29160_e28277_d_n4;
        locals.var_qn_bac_dn5 = assign29160_e28277_d_n5;
        locals.var_qn_bac_dn6 = assign29160_e28277_d_n6;
        locals.var_qn_bac_dn7 = assign29160_e28277_d_n7;
        locals.var_qn_bac_dn8 = assign29160_e28277_d_n8;
        locals.var_qn_bac_dn9 = assign29160_e28277_d_n9;
        locals.var_qn_bac_dn10 = assign29160_e28277_d_n10;
        locals.var_qn_bac_dn11 = assign29160_e28277_d_n11;
        locals.var_qn_bac_dn14 = assign29160_e28277_d_n14;

        let assign29170_e28280: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign29170_e28283: f64 = 0.06;
        let assign29170_e28288: f64 = if ((assign29170_e28280 < assign29170_e28283) && (0.06 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard684 = assign29170_e28288;

        let (assign29180_e28302, assign29180_e28302_d_n0, assign29180_e28302_d_n2, assign29180_e28302_d_n4, assign29180_e28302_d_n5, assign29180_e28302_d_n6, assign29180_e28302_d_n7, assign29180_e28302_d_n8, assign29180_e28302_d_n9, assign29180_e28302_d_n10, assign29180_e28302_d_n11, assign29180_e28302_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29180_e28296: f64 = 0.06;
        let assign29180_e28299: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign29180_e28300: f64 = (assign29180_e28296 - assign29180_e28299);
        (assign29180_e28300, (-(locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0)), (-(locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2)), (-(locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4)), (-(locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5)), (-(locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6)), (-(locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7)), (-(locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8)), (-(locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9)), (-(locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10)), (-(locals.var_phi_s0_dep_dn11 - locals.var_vds_maxb0_dn11)), (-(locals.var_phi_s0_dep_dn14 - locals.var_vds_maxb0_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign29180_e28302;
        locals.var_tmf1_dn0 = assign29180_e28302_d_n0;
        locals.var_tmf1_dn2 = assign29180_e28302_d_n2;
        locals.var_tmf1_dn4 = assign29180_e28302_d_n4;
        locals.var_tmf1_dn5 = assign29180_e28302_d_n5;
        locals.var_tmf1_dn6 = assign29180_e28302_d_n6;
        locals.var_tmf1_dn7 = assign29180_e28302_d_n7;
        locals.var_tmf1_dn8 = assign29180_e28302_d_n8;
        locals.var_tmf1_dn9 = assign29180_e28302_d_n9;
        locals.var_tmf1_dn10 = assign29180_e28302_d_n10;
        locals.var_tmf1_dn11 = assign29180_e28302_d_n11;
        locals.var_tmf1_dn14 = assign29180_e28302_d_n14;

        let (assign29190_e28312, assign29190_e28312_d_n0, assign29190_e28312_d_n2, assign29190_e28312_d_n4, assign29190_e28312_d_n5, assign29190_e28312_d_n6, assign29190_e28312_d_n7, assign29190_e28312_d_n8, assign29190_e28312_d_n9, assign29190_e28312_d_n10, assign29190_e28312_d_n11, assign29190_e28312_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29190_e28310: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign29190_e28310, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign29190_e28312;
        locals.var_x2_dn0 = assign29190_e28312_d_n0;
        locals.var_x2_dn2 = assign29190_e28312_d_n2;
        locals.var_x2_dn4 = assign29190_e28312_d_n4;
        locals.var_x2_dn5 = assign29190_e28312_d_n5;
        locals.var_x2_dn6 = assign29190_e28312_d_n6;
        locals.var_x2_dn7 = assign29190_e28312_d_n7;
        locals.var_x2_dn8 = assign29190_e28312_d_n8;
        locals.var_x2_dn9 = assign29190_e28312_d_n9;
        locals.var_x2_dn10 = assign29190_e28312_d_n10;
        locals.var_x2_dn11 = assign29190_e28312_d_n11;
        locals.var_x2_dn14 = assign29190_e28312_d_n14;

        let (assign29200_e28322, assign29200_e28322_d_n0, assign29200_e28322_d_n2, assign29200_e28322_d_n4, assign29200_e28322_d_n5, assign29200_e28322_d_n6, assign29200_e28322_d_n7, assign29200_e28322_d_n8, assign29200_e28322_d_n9, assign29200_e28322_d_n10, assign29200_e28322_d_n11, assign29200_e28322_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29200_e28320: f64 = (0.06 * 0.06);
        (assign29200_e28320, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign29200_e28322;
        locals.var_xmax2_dn0 = assign29200_e28322_d_n0;
        locals.var_xmax2_dn2 = assign29200_e28322_d_n2;
        locals.var_xmax2_dn4 = assign29200_e28322_d_n4;
        locals.var_xmax2_dn5 = assign29200_e28322_d_n5;
        locals.var_xmax2_dn6 = assign29200_e28322_d_n6;
        locals.var_xmax2_dn7 = assign29200_e28322_d_n7;
        locals.var_xmax2_dn8 = assign29200_e28322_d_n8;
        locals.var_xmax2_dn9 = assign29200_e28322_d_n9;
        locals.var_xmax2_dn10 = assign29200_e28322_d_n10;
        locals.var_xmax2_dn11 = assign29200_e28322_d_n11;
        locals.var_xmax2_dn14 = assign29200_e28322_d_n14;

        let (assign29210_e28330, assign29210_e28330_d_n0, assign29210_e28330_d_n2, assign29210_e28330_d_n4, assign29210_e28330_d_n5, assign29210_e28330_d_n6, assign29210_e28330_d_n7, assign29210_e28330_d_n8, assign29210_e28330_d_n9, assign29210_e28330_d_n10, assign29210_e28330_d_n11, assign29210_e28330_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29210_e28330;
        locals.var_xp_dn0 = assign29210_e28330_d_n0;
        locals.var_xp_dn2 = assign29210_e28330_d_n2;
        locals.var_xp_dn4 = assign29210_e28330_d_n4;
        locals.var_xp_dn5 = assign29210_e28330_d_n5;
        locals.var_xp_dn6 = assign29210_e28330_d_n6;
        locals.var_xp_dn7 = assign29210_e28330_d_n7;
        locals.var_xp_dn8 = assign29210_e28330_d_n8;
        locals.var_xp_dn9 = assign29210_e28330_d_n9;
        locals.var_xp_dn10 = assign29210_e28330_d_n10;
        locals.var_xp_dn11 = assign29210_e28330_d_n11;
        locals.var_xp_dn14 = assign29210_e28330_d_n14;

        let (assign29220_e28338, assign29220_e28338_d_n0, assign29220_e28338_d_n2, assign29220_e28338_d_n4, assign29220_e28338_d_n5, assign29220_e28338_d_n6, assign29220_e28338_d_n7, assign29220_e28338_d_n8, assign29220_e28338_d_n9, assign29220_e28338_d_n10, assign29220_e28338_d_n11, assign29220_e28338_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29220_e28338;
        locals.var_xmp_dn0 = assign29220_e28338_d_n0;
        locals.var_xmp_dn2 = assign29220_e28338_d_n2;
        locals.var_xmp_dn4 = assign29220_e28338_d_n4;
        locals.var_xmp_dn5 = assign29220_e28338_d_n5;
        locals.var_xmp_dn6 = assign29220_e28338_d_n6;
        locals.var_xmp_dn7 = assign29220_e28338_d_n7;
        locals.var_xmp_dn8 = assign29220_e28338_d_n8;
        locals.var_xmp_dn9 = assign29220_e28338_d_n9;
        locals.var_xmp_dn10 = assign29220_e28338_d_n10;
        locals.var_xmp_dn11 = assign29220_e28338_d_n11;
        locals.var_xmp_dn14 = assign29220_e28338_d_n14;

        let (assign29230_e28346,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29230_e28346;

    }

    pub(super) fn stamp_transient_block_84(
        locals: &mut StampLocals,
    ) {
        let (assign29240_e28354,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29240_e28354;

        let (assign29250_e28362, assign29250_e28362_d_n0, assign29250_e28362_d_n2, assign29250_e28362_d_n4, assign29250_e28362_d_n5, assign29250_e28362_d_n6, assign29250_e28362_d_n7, assign29250_e28362_d_n8, assign29250_e28362_d_n9, assign29250_e28362_d_n10, assign29250_e28362_d_n11, assign29250_e28362_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29250_e28362;
        locals.var_arg_dn0 = assign29250_e28362_d_n0;
        locals.var_arg_dn2 = assign29250_e28362_d_n2;
        locals.var_arg_dn4 = assign29250_e28362_d_n4;
        locals.var_arg_dn5 = assign29250_e28362_d_n5;
        locals.var_arg_dn6 = assign29250_e28362_d_n6;
        locals.var_arg_dn7 = assign29250_e28362_d_n7;
        locals.var_arg_dn8 = assign29250_e28362_d_n8;
        locals.var_arg_dn9 = assign29250_e28362_d_n9;
        locals.var_arg_dn10 = assign29250_e28362_d_n10;
        locals.var_arg_dn11 = assign29250_e28362_d_n11;
        locals.var_arg_dn14 = assign29250_e28362_d_n14;

        let (assign29260_e28370, assign29260_e28370_d_n0, assign29260_e28370_d_n2, assign29260_e28370_d_n4, assign29260_e28370_d_n5, assign29260_e28370_d_n6, assign29260_e28370_d_n7, assign29260_e28370_d_n8, assign29260_e28370_d_n9, assign29260_e28370_d_n10, assign29260_e28370_d_n11, assign29260_e28370_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29260_e28370;
        locals.var_dnm_dn0 = assign29260_e28370_d_n0;
        locals.var_dnm_dn2 = assign29260_e28370_d_n2;
        locals.var_dnm_dn4 = assign29260_e28370_d_n4;
        locals.var_dnm_dn5 = assign29260_e28370_d_n5;
        locals.var_dnm_dn6 = assign29260_e28370_d_n6;
        locals.var_dnm_dn7 = assign29260_e28370_d_n7;
        locals.var_dnm_dn8 = assign29260_e28370_d_n8;
        locals.var_dnm_dn9 = assign29260_e28370_d_n9;
        locals.var_dnm_dn10 = assign29260_e28370_d_n10;
        locals.var_dnm_dn11 = assign29260_e28370_d_n11;
        locals.var_dnm_dn14 = assign29260_e28370_d_n14;

        let (assign29270_e28380, assign29270_e28380_d_n0, assign29270_e28380_d_n2, assign29270_e28380_d_n4, assign29270_e28380_d_n5, assign29270_e28380_d_n6, assign29270_e28380_d_n7, assign29270_e28380_d_n8, assign29270_e28380_d_n9, assign29270_e28380_d_n10, assign29270_e28380_d_n11, assign29270_e28380_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29270_e28378: f64 = (locals.var_xp * locals.var_x2);
        (assign29270_e28378, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29270_e28380;
        locals.var_xp_dn0 = assign29270_e28380_d_n0;
        locals.var_xp_dn2 = assign29270_e28380_d_n2;
        locals.var_xp_dn4 = assign29270_e28380_d_n4;
        locals.var_xp_dn5 = assign29270_e28380_d_n5;
        locals.var_xp_dn6 = assign29270_e28380_d_n6;
        locals.var_xp_dn7 = assign29270_e28380_d_n7;
        locals.var_xp_dn8 = assign29270_e28380_d_n8;
        locals.var_xp_dn9 = assign29270_e28380_d_n9;
        locals.var_xp_dn10 = assign29270_e28380_d_n10;
        locals.var_xp_dn11 = assign29270_e28380_d_n11;
        locals.var_xp_dn14 = assign29270_e28380_d_n14;

        let (assign29280_e28390, assign29280_e28390_d_n0, assign29280_e28390_d_n2, assign29280_e28390_d_n4, assign29280_e28390_d_n5, assign29280_e28390_d_n6, assign29280_e28390_d_n7, assign29280_e28390_d_n8, assign29280_e28390_d_n9, assign29280_e28390_d_n10, assign29280_e28390_d_n11, assign29280_e28390_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29280_e28388: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29280_e28388, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29280_e28390;
        locals.var_xmp_dn0 = assign29280_e28390_d_n0;
        locals.var_xmp_dn2 = assign29280_e28390_d_n2;
        locals.var_xmp_dn4 = assign29280_e28390_d_n4;
        locals.var_xmp_dn5 = assign29280_e28390_d_n5;
        locals.var_xmp_dn6 = assign29280_e28390_d_n6;
        locals.var_xmp_dn7 = assign29280_e28390_d_n7;
        locals.var_xmp_dn8 = assign29280_e28390_d_n8;
        locals.var_xmp_dn9 = assign29280_e28390_d_n9;
        locals.var_xmp_dn10 = assign29280_e28390_d_n10;
        locals.var_xmp_dn11 = assign29280_e28390_d_n11;
        locals.var_xmp_dn14 = assign29280_e28390_d_n14;

        let (assign29290_e28400, assign29290_e28400_d_n0, assign29290_e28400_d_n2, assign29290_e28400_d_n4, assign29290_e28400_d_n5, assign29290_e28400_d_n6, assign29290_e28400_d_n7, assign29290_e28400_d_n8, assign29290_e28400_d_n9, assign29290_e28400_d_n10, assign29290_e28400_d_n11, assign29290_e28400_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29290_e28398: f64 = (locals.var_xp * locals.var_x2);
        (assign29290_e28398, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29290_e28400;
        locals.var_xp_dn0 = assign29290_e28400_d_n0;
        locals.var_xp_dn2 = assign29290_e28400_d_n2;
        locals.var_xp_dn4 = assign29290_e28400_d_n4;
        locals.var_xp_dn5 = assign29290_e28400_d_n5;
        locals.var_xp_dn6 = assign29290_e28400_d_n6;
        locals.var_xp_dn7 = assign29290_e28400_d_n7;
        locals.var_xp_dn8 = assign29290_e28400_d_n8;
        locals.var_xp_dn9 = assign29290_e28400_d_n9;
        locals.var_xp_dn10 = assign29290_e28400_d_n10;
        locals.var_xp_dn11 = assign29290_e28400_d_n11;
        locals.var_xp_dn14 = assign29290_e28400_d_n14;

        let (assign29300_e28410, assign29300_e28410_d_n0, assign29300_e28410_d_n2, assign29300_e28410_d_n4, assign29300_e28410_d_n5, assign29300_e28410_d_n6, assign29300_e28410_d_n7, assign29300_e28410_d_n8, assign29300_e28410_d_n9, assign29300_e28410_d_n10, assign29300_e28410_d_n11, assign29300_e28410_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29300_e28408: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29300_e28408, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29300_e28410;
        locals.var_xmp_dn0 = assign29300_e28410_d_n0;
        locals.var_xmp_dn2 = assign29300_e28410_d_n2;
        locals.var_xmp_dn4 = assign29300_e28410_d_n4;
        locals.var_xmp_dn5 = assign29300_e28410_d_n5;
        locals.var_xmp_dn6 = assign29300_e28410_d_n6;
        locals.var_xmp_dn7 = assign29300_e28410_d_n7;
        locals.var_xmp_dn8 = assign29300_e28410_d_n8;
        locals.var_xmp_dn9 = assign29300_e28410_d_n9;
        locals.var_xmp_dn10 = assign29300_e28410_d_n10;
        locals.var_xmp_dn11 = assign29300_e28410_d_n11;
        locals.var_xmp_dn14 = assign29300_e28410_d_n14;

        let (assign29310_e28420, assign29310_e28420_d_n0, assign29310_e28420_d_n2, assign29310_e28420_d_n4, assign29310_e28420_d_n5, assign29310_e28420_d_n6, assign29310_e28420_d_n7, assign29310_e28420_d_n8, assign29310_e28420_d_n9, assign29310_e28420_d_n10, assign29310_e28420_d_n11, assign29310_e28420_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29310_e28418: f64 = (locals.var_xp + locals.var_xmp);
        (assign29310_e28418, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29310_e28420;
        locals.var_arg_dn0 = assign29310_e28420_d_n0;
        locals.var_arg_dn2 = assign29310_e28420_d_n2;
        locals.var_arg_dn4 = assign29310_e28420_d_n4;
        locals.var_arg_dn5 = assign29310_e28420_d_n5;
        locals.var_arg_dn6 = assign29310_e28420_d_n6;
        locals.var_arg_dn7 = assign29310_e28420_d_n7;
        locals.var_arg_dn8 = assign29310_e28420_d_n8;
        locals.var_arg_dn9 = assign29310_e28420_d_n9;
        locals.var_arg_dn10 = assign29310_e28420_d_n10;
        locals.var_arg_dn11 = assign29310_e28420_d_n11;
        locals.var_arg_dn14 = assign29310_e28420_d_n14;

        let (assign29320_e28428, assign29320_e28428_d_n0, assign29320_e28428_d_n2, assign29320_e28428_d_n4, assign29320_e28428_d_n5, assign29320_e28428_d_n6, assign29320_e28428_d_n7, assign29320_e28428_d_n8, assign29320_e28428_d_n9, assign29320_e28428_d_n10, assign29320_e28428_d_n11, assign29320_e28428_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29320_e28428;
        locals.var_dnm_dn0 = assign29320_e28428_d_n0;
        locals.var_dnm_dn2 = assign29320_e28428_d_n2;
        locals.var_dnm_dn4 = assign29320_e28428_d_n4;
        locals.var_dnm_dn5 = assign29320_e28428_d_n5;
        locals.var_dnm_dn6 = assign29320_e28428_d_n6;
        locals.var_dnm_dn7 = assign29320_e28428_d_n7;
        locals.var_dnm_dn8 = assign29320_e28428_d_n8;
        locals.var_dnm_dn9 = assign29320_e28428_d_n9;
        locals.var_dnm_dn10 = assign29320_e28428_d_n10;
        locals.var_dnm_dn11 = assign29320_e28428_d_n11;
        locals.var_dnm_dn14 = assign29320_e28428_d_n14;

        let assign29330_e28443: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard685 = assign29330_e28443;

        let assign29340_e28446: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard686 = assign29340_e28446;

        let (assign29350_e28458,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) && (locals.var_guard686 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29350_e28458;

        let assign29360_e28461: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard687 = assign29360_e28461;

        let (assign29370_e28476,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29370_e28476;

        let assign29380_e28479: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard688 = assign29380_e28479;

        let (assign29390_e28497,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard688 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29390_e28497;

        let assign29400_e28500: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard689 = assign29400_e28500;

        let (assign29410_e28521,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard688 == 0.0)) && (locals.var_guard689 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29410_e28521;

        let (assign29420_e28531,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29420_e28531;

        let mut assign29430_loop_guard: usize = 0;
        while {
            let assign29430_cond_e28542: f64 = if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29430_cond_e28542 != 0.0
        } {
            assign29430_loop_guard += 1;
            assert!(assign29430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29430_body0_e28553, assign29430_body0_e28553_d_n0, assign29430_body0_e28553_d_n2, assign29430_body0_e28553_d_n4, assign29430_body0_e28553_d_n5, assign29430_body0_e28553_d_n6, assign29430_body0_e28553_d_n7, assign29430_body0_e28553_d_n8, assign29430_body0_e28553_d_n9, assign29430_body0_e28553_d_n10, assign29430_body0_e28553_d_n11, assign29430_body0_e28553_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) {
        let assign29430_body0_e28551: f64 = (locals.var_dnm).sqrt();
        (assign29430_body0_e28551, (locals.var_dnm_dn0 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn2 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn4 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn5 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn6 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn7 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn8 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn9 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn10 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn11 / (2.0 * assign29430_body0_e28551)), (locals.var_dnm_dn14 / (2.0 * assign29430_body0_e28551)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign29430_body0_e28553;
            locals.var_dnm_dn0 = assign29430_body0_e28553_d_n0;
            locals.var_dnm_dn2 = assign29430_body0_e28553_d_n2;
            locals.var_dnm_dn4 = assign29430_body0_e28553_d_n4;
            locals.var_dnm_dn5 = assign29430_body0_e28553_d_n5;
            locals.var_dnm_dn6 = assign29430_body0_e28553_d_n6;
            locals.var_dnm_dn7 = assign29430_body0_e28553_d_n7;
            locals.var_dnm_dn8 = assign29430_body0_e28553_d_n8;
            locals.var_dnm_dn9 = assign29430_body0_e28553_d_n9;
            locals.var_dnm_dn10 = assign29430_body0_e28553_d_n10;
            locals.var_dnm_dn11 = assign29430_body0_e28553_d_n11;
            locals.var_dnm_dn14 = assign29430_body0_e28553_d_n14;
            let (assign29430_body1_e28565,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) {
        let assign29430_body1_e28563: f64 = (locals.var_m0 + 1.0);
        (assign29430_body1_e28563,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29430_body1_e28565;
        }

        let (assign29440_e28587, assign29440_e28587_d_n0, assign29440_e28587_d_n2, assign29440_e28587_d_n4, assign29440_e28587_d_n5, assign29440_e28587_d_n6, assign29440_e28587_d_n7, assign29440_e28587_d_n8, assign29440_e28587_d_n9, assign29440_e28587_d_n10, assign29440_e28587_d_n11, assign29440_e28587_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 == 0.0)) {
        let (assign29440_e28585, assign29440_e28585_d_n0, assign29440_e28585_d_n2, assign29440_e28585_d_n4, assign29440_e28585_d_n5, assign29440_e28585_d_n6, assign29440_e28585_d_n7, assign29440_e28585_d_n8, assign29440_e28585_d_n9, assign29440_e28585_d_n10, assign29440_e28585_d_n11, assign29440_e28585_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29440_e28582: f64 = (2.0 * 2.0);
                let assign29440_e28583: f64 = (1.0 / assign29440_e28582);
                let assign29440_e28584: f64 = (locals.var_dnm).powf(assign29440_e28583);
                (assign29440_e28584, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn11)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29440_e28583) as f64).is_finite() && ((assign29440_e28583) as f64).fract() == 0.0 { if assign29440_e28583 == 0.0 { 0.0 } else { (assign29440_e28583 * ((locals.var_dnm).powf(assign29440_e28583 - 1.0) * locals.var_dnm_dn14)) } } else { (assign29440_e28584 * (assign29440_e28583 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign29440_e28585, assign29440_e28585_d_n0, assign29440_e28585_d_n2, assign29440_e28585_d_n4, assign29440_e28585_d_n5, assign29440_e28585_d_n6, assign29440_e28585_d_n7, assign29440_e28585_d_n8, assign29440_e28585_d_n9, assign29440_e28585_d_n10, assign29440_e28585_d_n11, assign29440_e28585_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29440_e28587;
        locals.var_dnm_dn0 = assign29440_e28587_d_n0;
        locals.var_dnm_dn2 = assign29440_e28587_d_n2;
        locals.var_dnm_dn4 = assign29440_e28587_d_n4;
        locals.var_dnm_dn5 = assign29440_e28587_d_n5;
        locals.var_dnm_dn6 = assign29440_e28587_d_n6;
        locals.var_dnm_dn7 = assign29440_e28587_d_n7;
        locals.var_dnm_dn8 = assign29440_e28587_d_n8;
        locals.var_dnm_dn9 = assign29440_e28587_d_n9;
        locals.var_dnm_dn10 = assign29440_e28587_d_n10;
        locals.var_dnm_dn11 = assign29440_e28587_d_n11;
        locals.var_dnm_dn14 = assign29440_e28587_d_n14;

        let (assign29450_e28597, assign29450_e28597_d_n0, assign29450_e28597_d_n2, assign29450_e28597_d_n4, assign29450_e28597_d_n5, assign29450_e28597_d_n6, assign29450_e28597_d_n7, assign29450_e28597_d_n8, assign29450_e28597_d_n9, assign29450_e28597_d_n10, assign29450_e28597_d_n11, assign29450_e28597_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29450_e28595: f64 = (1.0 / locals.var_dnm);
        (assign29450_e28595, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29450_e28597;
        locals.var_dnm_dn0 = assign29450_e28597_d_n0;
        locals.var_dnm_dn2 = assign29450_e28597_d_n2;
        locals.var_dnm_dn4 = assign29450_e28597_d_n4;
        locals.var_dnm_dn5 = assign29450_e28597_d_n5;
        locals.var_dnm_dn6 = assign29450_e28597_d_n6;
        locals.var_dnm_dn7 = assign29450_e28597_d_n7;
        locals.var_dnm_dn8 = assign29450_e28597_d_n8;
        locals.var_dnm_dn9 = assign29450_e28597_d_n9;
        locals.var_dnm_dn10 = assign29450_e28597_d_n10;
        locals.var_dnm_dn11 = assign29450_e28597_d_n11;
        locals.var_dnm_dn14 = assign29450_e28597_d_n14;

        let (assign29460_e28609, assign29460_e28609_d_n0, assign29460_e28609_d_n2, assign29460_e28609_d_n4, assign29460_e28609_d_n5, assign29460_e28609_d_n6, assign29460_e28609_d_n7, assign29460_e28609_d_n8, assign29460_e28609_d_n9, assign29460_e28609_d_n10, assign29460_e28609_d_n11, assign29460_e28609_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29460_e28605: f64 = (locals.var_tmf1 * 0.06);
        let assign29460_e28607: f64 = (assign29460_e28605 * locals.var_dnm);
        (assign29460_e28607, (((locals.var_tmf1_dn0 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.06) * locals.var_dnm) + (assign29460_e28605 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign29460_e28609;
        locals.var_tmf0_dn0 = assign29460_e28609_d_n0;
        locals.var_tmf0_dn2 = assign29460_e28609_d_n2;
        locals.var_tmf0_dn4 = assign29460_e28609_d_n4;
        locals.var_tmf0_dn5 = assign29460_e28609_d_n5;
        locals.var_tmf0_dn6 = assign29460_e28609_d_n6;
        locals.var_tmf0_dn7 = assign29460_e28609_d_n7;
        locals.var_tmf0_dn8 = assign29460_e28609_d_n8;
        locals.var_tmf0_dn9 = assign29460_e28609_d_n9;
        locals.var_tmf0_dn10 = assign29460_e28609_d_n10;
        locals.var_tmf0_dn11 = assign29460_e28609_d_n11;
        locals.var_tmf0_dn14 = assign29460_e28609_d_n14;

        let (assign29470_e28623, assign29470_e28623_d_n0, assign29470_e28623_d_n2, assign29470_e28623_d_n4, assign29470_e28623_d_n5, assign29470_e28623_d_n6, assign29470_e28623_d_n7, assign29470_e28623_d_n8, assign29470_e28623_d_n9, assign29470_e28623_d_n10, assign29470_e28623_d_n11, assign29470_e28623_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29470_e28617: f64 = (0.06 * locals.var_xmp);
        let assign29470_e28619: f64 = (assign29470_e28617 * locals.var_dnm);
        let assign29470_e28621: f64 = (assign29470_e28619 / locals.var_arg);
        (assign29470_e28621, ((((((0.06 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn11) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn11)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn14) * locals.var_dnm) + (assign29470_e28617 * locals.var_dnm_dn14)) * locals.var_arg) - (assign29470_e28619 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29470_e28623;
        locals.var_t0_dn0 = assign29470_e28623_d_n0;
        locals.var_t0_dn2 = assign29470_e28623_d_n2;
        locals.var_t0_dn4 = assign29470_e28623_d_n4;
        locals.var_t0_dn5 = assign29470_e28623_d_n5;
        locals.var_t0_dn6 = assign29470_e28623_d_n6;
        locals.var_t0_dn7 = assign29470_e28623_d_n7;
        locals.var_t0_dn8 = assign29470_e28623_d_n8;
        locals.var_t0_dn9 = assign29470_e28623_d_n9;
        locals.var_t0_dn10 = assign29470_e28623_d_n10;
        locals.var_t0_dn11 = assign29470_e28623_d_n11;
        locals.var_t0_dn14 = assign29470_e28623_d_n14;

        let (assign29480_e28635, assign29480_e28635_d_n0, assign29480_e28635_d_n2, assign29480_e28635_d_n4, assign29480_e28635_d_n5, assign29480_e28635_d_n6, assign29480_e28635_d_n7, assign29480_e28635_d_n8, assign29480_e28635_d_n9, assign29480_e28635_d_n10, assign29480_e28635_d_n11, assign29480_e28635_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign29480_e28631: f64 = 0.06;
        let assign29480_e28633: f64 = (assign29480_e28631 - locals.var_tmf0);
        (assign29480_e28633, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29480_e28635;
        locals.var_t2_dn0 = assign29480_e28635_d_n0;
        locals.var_t2_dn2 = assign29480_e28635_d_n2;
        locals.var_t2_dn4 = assign29480_e28635_d_n4;
        locals.var_t2_dn5 = assign29480_e28635_d_n5;
        locals.var_t2_dn6 = assign29480_e28635_d_n6;
        locals.var_t2_dn7 = assign29480_e28635_d_n7;
        locals.var_t2_dn8 = assign29480_e28635_d_n8;
        locals.var_t2_dn9 = assign29480_e28635_d_n9;
        locals.var_t2_dn10 = assign29480_e28635_d_n10;
        locals.var_t2_dn11 = assign29480_e28635_d_n11;
        locals.var_t2_dn14 = assign29480_e28635_d_n14;

        let (assign29490_e28643, assign29490_e28643_d_n0, assign29490_e28643_d_n2, assign29490_e28643_d_n4, assign29490_e28643_d_n5, assign29490_e28643_d_n6, assign29490_e28643_d_n7, assign29490_e28643_d_n8, assign29490_e28643_d_n9, assign29490_e28643_d_n10, assign29490_e28643_d_n11, assign29490_e28643_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29490_e28643;
        locals.var_t0_dn0 = assign29490_e28643_d_n0;
        locals.var_t0_dn2 = assign29490_e28643_d_n2;
        locals.var_t0_dn4 = assign29490_e28643_d_n4;
        locals.var_t0_dn5 = assign29490_e28643_d_n5;
        locals.var_t0_dn6 = assign29490_e28643_d_n6;
        locals.var_t0_dn7 = assign29490_e28643_d_n7;
        locals.var_t0_dn8 = assign29490_e28643_d_n8;
        locals.var_t0_dn9 = assign29490_e28643_d_n9;
        locals.var_t0_dn10 = assign29490_e28643_d_n10;
        locals.var_t0_dn11 = assign29490_e28643_d_n11;
        locals.var_t0_dn14 = assign29490_e28643_d_n14;

        let (assign29500_e28654, assign29500_e28654_d_n0, assign29500_e28654_d_n2, assign29500_e28654_d_n4, assign29500_e28654_d_n5, assign29500_e28654_d_n6, assign29500_e28654_d_n7, assign29500_e28654_d_n8, assign29500_e28654_d_n9, assign29500_e28654_d_n10, assign29500_e28654_d_n11, assign29500_e28654_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign29500_e28652: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        (assign29500_e28652, (locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phi_s0_dep_dn11 - locals.var_vds_maxb0_dn11), (locals.var_phi_s0_dep_dn14 - locals.var_vds_maxb0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29500_e28654;
        locals.var_t2_dn0 = assign29500_e28654_d_n0;
        locals.var_t2_dn2 = assign29500_e28654_d_n2;
        locals.var_t2_dn4 = assign29500_e28654_d_n4;
        locals.var_t2_dn5 = assign29500_e28654_d_n5;
        locals.var_t2_dn6 = assign29500_e28654_d_n6;
        locals.var_t2_dn7 = assign29500_e28654_d_n7;
        locals.var_t2_dn8 = assign29500_e28654_d_n8;
        locals.var_t2_dn9 = assign29500_e28654_d_n9;
        locals.var_t2_dn10 = assign29500_e28654_d_n10;
        locals.var_t2_dn11 = assign29500_e28654_d_n11;
        locals.var_t2_dn14 = assign29500_e28654_d_n14;

        let (assign29510_e28663, assign29510_e28663_d_n0, assign29510_e28663_d_n2, assign29510_e28663_d_n4, assign29510_e28663_d_n5, assign29510_e28663_d_n6, assign29510_e28663_d_n7, assign29510_e28663_d_n8, assign29510_e28663_d_n9, assign29510_e28663_d_n10, assign29510_e28663_d_n11, assign29510_e28663_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard684 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29510_e28663;
        locals.var_t0_dn0 = assign29510_e28663_d_n0;
        locals.var_t0_dn2 = assign29510_e28663_d_n2;
        locals.var_t0_dn4 = assign29510_e28663_d_n4;
        locals.var_t0_dn5 = assign29510_e28663_d_n5;
        locals.var_t0_dn6 = assign29510_e28663_d_n6;
        locals.var_t0_dn7 = assign29510_e28663_d_n7;
        locals.var_t0_dn8 = assign29510_e28663_d_n8;
        locals.var_t0_dn9 = assign29510_e28663_d_n9;
        locals.var_t0_dn10 = assign29510_e28663_d_n10;
        locals.var_t0_dn11 = assign29510_e28663_d_n11;
        locals.var_t0_dn14 = assign29510_e28663_d_n14;

        let (assign29520_e28682, assign29520_e28682_d_n0, assign29520_e28682_d_n2, assign29520_e28682_d_n4, assign29520_e28682_d_n5, assign29520_e28682_d_n6, assign29520_e28682_d_n7, assign29520_e28682_d_n8, assign29520_e28682_d_n9, assign29520_e28682_d_n10, assign29520_e28682_d_n11, assign29520_e28682_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign29520_e28669: f64 = (locals.var_beta * locals.var_t2);
        let assign29520_e28670: f64 = (assign29520_e28669).exp();
        let assign29520_e28672: f64 = (assign29520_e28670 - 1.0);
        let assign29520_e28675: f64 = (locals.var_beta * locals.var_t2);
        let assign29520_e28676: f64 = (assign29520_e28672 - assign29520_e28675);
        let assign29520_e28679: f64 = (10.0 * 2.220446049250313e-16);
        let assign29520_e28680: f64 = (assign29520_e28676 + assign29520_e28679);
        (assign29520_e28680, ((assign29520_e28670 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign29520_e28670 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign29520_e28670 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign29520_e28670 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign29520_e28670 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign29520_e28670 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign29520_e28670 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign29520_e28670 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign29520_e28670 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign29520_e28670 * ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))) - ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))), ((assign29520_e28670 * ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))) - ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29520_e28682;
        locals.var_t4_dn0 = assign29520_e28682_d_n0;
        locals.var_t4_dn2 = assign29520_e28682_d_n2;
        locals.var_t4_dn4 = assign29520_e28682_d_n4;
        locals.var_t4_dn5 = assign29520_e28682_d_n5;
        locals.var_t4_dn6 = assign29520_e28682_d_n6;
        locals.var_t4_dn7 = assign29520_e28682_d_n7;
        locals.var_t4_dn8 = assign29520_e28682_d_n8;
        locals.var_t4_dn9 = assign29520_e28682_d_n9;
        locals.var_t4_dn10 = assign29520_e28682_d_n10;
        locals.var_t4_dn11 = assign29520_e28682_d_n11;
        locals.var_t4_dn14 = assign29520_e28682_d_n14;

        let (assign29530_e28692, assign29530_e28692_d_n0, assign29530_e28692_d_n2, assign29530_e28692_d_n4, assign29530_e28692_d_n5, assign29530_e28692_d_n6, assign29530_e28692_d_n7, assign29530_e28692_d_n8, assign29530_e28692_d_n9, assign29530_e28692_d_n10, assign29530_e28692_d_n11, assign29530_e28692_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign29530_e28687: f64 = (-locals.var_cnst0);
        let assign29530_e28689: f64 = (locals.var_t4).sqrt();
        let assign29530_e28690: f64 = (assign29530_e28687 * assign29530_e28689);
        (assign29530_e28690, (((-locals.var_cnst0_dn0) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn0 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn2) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn2 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn4) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn4 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn5) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn5 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn6) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn6 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn7) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn7 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn8) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn8 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn9) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn9 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn10) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn10 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn11) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn11 / (2.0 * assign29530_e28689)))), (((-locals.var_cnst0_dn14) * assign29530_e28689) + (assign29530_e28687 * (locals.var_t4_dn14 / (2.0 * assign29530_e28689)))),)
    } else {
        (locals.var_q_n0_cur, locals.var_q_n0_cur_dn0, locals.var_q_n0_cur_dn2, locals.var_q_n0_cur_dn4, locals.var_q_n0_cur_dn5, locals.var_q_n0_cur_dn6, locals.var_q_n0_cur_dn7, locals.var_q_n0_cur_dn8, locals.var_q_n0_cur_dn9, locals.var_q_n0_cur_dn10, locals.var_q_n0_cur_dn11, locals.var_q_n0_cur_dn14,)
    }
};
        locals.var_q_n0_cur = assign29530_e28692;
        locals.var_q_n0_cur_dn0 = assign29530_e28692_d_n0;
        locals.var_q_n0_cur_dn2 = assign29530_e28692_d_n2;
        locals.var_q_n0_cur_dn4 = assign29530_e28692_d_n4;
        locals.var_q_n0_cur_dn5 = assign29530_e28692_d_n5;
        locals.var_q_n0_cur_dn6 = assign29530_e28692_d_n6;
        locals.var_q_n0_cur_dn7 = assign29530_e28692_d_n7;
        locals.var_q_n0_cur_dn8 = assign29530_e28692_d_n8;
        locals.var_q_n0_cur_dn9 = assign29530_e28692_d_n9;
        locals.var_q_n0_cur_dn10 = assign29530_e28692_d_n10;
        locals.var_q_n0_cur_dn11 = assign29530_e28692_d_n11;
        locals.var_q_n0_cur_dn14 = assign29530_e28692_d_n14;

    }

    pub(super) fn stamp_transient_block_85(
        locals: &mut StampLocals,
    ) {
        let (assign29540_e28707, assign29540_e28707_d_n0, assign29540_e28707_d_n2, assign29540_e28707_d_n4, assign29540_e28707_d_n5, assign29540_e28707_d_n6, assign29540_e28707_d_n7, assign29540_e28707_d_n8, assign29540_e28707_d_n9, assign29540_e28707_d_n10, assign29540_e28707_d_n11, assign29540_e28707_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign29540_e28698: f64 = (locals.var_beta * 0.1);
        let assign29540_e28699: f64 = (assign29540_e28698).exp();
        let assign29540_e28701: f64 = (assign29540_e28699 - 1.0);
        let assign29540_e28704: f64 = (locals.var_beta * 0.1);
        let assign29540_e28705: f64 = (assign29540_e28701 - assign29540_e28704);
        (assign29540_e28705, ((assign29540_e28699 * (locals.var_beta_dn0 * 0.1)) - (locals.var_beta_dn0 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn2 * 0.1)) - (locals.var_beta_dn2 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn4 * 0.1)) - (locals.var_beta_dn4 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn5 * 0.1)) - (locals.var_beta_dn5 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn6 * 0.1)) - (locals.var_beta_dn6 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn7 * 0.1)) - (locals.var_beta_dn7 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn8 * 0.1)) - (locals.var_beta_dn8 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn9 * 0.1)) - (locals.var_beta_dn9 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn10 * 0.1)) - (locals.var_beta_dn10 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn11 * 0.1)) - (locals.var_beta_dn11 * 0.1)), ((assign29540_e28699 * (locals.var_beta_dn14 * 0.1)) - (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29540_e28707;
        locals.var_t4_dn0 = assign29540_e28707_d_n0;
        locals.var_t4_dn2 = assign29540_e28707_d_n2;
        locals.var_t4_dn4 = assign29540_e28707_d_n4;
        locals.var_t4_dn5 = assign29540_e28707_d_n5;
        locals.var_t4_dn6 = assign29540_e28707_d_n6;
        locals.var_t4_dn7 = assign29540_e28707_d_n7;
        locals.var_t4_dn8 = assign29540_e28707_d_n8;
        locals.var_t4_dn9 = assign29540_e28707_d_n9;
        locals.var_t4_dn10 = assign29540_e28707_d_n10;
        locals.var_t4_dn11 = assign29540_e28707_d_n11;
        locals.var_t4_dn14 = assign29540_e28707_d_n14;

        let (assign29550_e28716, assign29550_e28716_d_n0, assign29550_e28716_d_n2, assign29550_e28716_d_n4, assign29550_e28716_d_n5, assign29550_e28716_d_n6, assign29550_e28716_d_n7, assign29550_e28716_d_n8, assign29550_e28716_d_n9, assign29550_e28716_d_n10, assign29550_e28716_d_n11, assign29550_e28716_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign29550_e28713: f64 = (locals.var_t4).sqrt();
        let assign29550_e28714: f64 = (locals.var_cnst0 * assign29550_e28713);
        (assign29550_e28714, ((locals.var_cnst0_dn0 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn0 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn2 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn2 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn4 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn4 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn5 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn5 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn6 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn6 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn7 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn7 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn8 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn8 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn9 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn9 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn10 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn10 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn11 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn11 / (2.0 * assign29550_e28713)))), ((locals.var_cnst0_dn14 * assign29550_e28713) + (locals.var_cnst0 * (locals.var_t4_dn14 / (2.0 * assign29550_e28713)))),)
    } else {
        (locals.var_qn_delta, locals.var_qn_delta_dn0, locals.var_qn_delta_dn2, locals.var_qn_delta_dn4, locals.var_qn_delta_dn5, locals.var_qn_delta_dn6, locals.var_qn_delta_dn7, locals.var_qn_delta_dn8, locals.var_qn_delta_dn9, locals.var_qn_delta_dn10, locals.var_qn_delta_dn11, locals.var_qn_delta_dn14,)
    }
};
        locals.var_qn_delta = assign29550_e28716;
        locals.var_qn_delta_dn0 = assign29550_e28716_d_n0;
        locals.var_qn_delta_dn2 = assign29550_e28716_d_n2;
        locals.var_qn_delta_dn4 = assign29550_e28716_d_n4;
        locals.var_qn_delta_dn5 = assign29550_e28716_d_n5;
        locals.var_qn_delta_dn6 = assign29550_e28716_d_n6;
        locals.var_qn_delta_dn7 = assign29550_e28716_d_n7;
        locals.var_qn_delta_dn8 = assign29550_e28716_d_n8;
        locals.var_qn_delta_dn9 = assign29550_e28716_d_n9;
        locals.var_qn_delta_dn10 = assign29550_e28716_d_n10;
        locals.var_qn_delta_dn11 = assign29550_e28716_d_n11;
        locals.var_qn_delta_dn14 = assign29550_e28716_d_n14;

        let (assign29560_e28722, assign29560_e28722_d_n0, assign29560_e28722_d_n2, assign29560_e28722_d_n4, assign29560_e28722_d_n5, assign29560_e28722_d_n6, assign29560_e28722_d_n7, assign29560_e28722_d_n8, assign29560_e28722_d_n9, assign29560_e28722_d_n10, assign29560_e28722_d_n11, assign29560_e28722_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    }
};
        locals.var_vdsorg = assign29560_e28722;
        locals.var_vdsorg_dn0 = assign29560_e28722_d_n0;
        locals.var_vdsorg_dn2 = assign29560_e28722_d_n2;
        locals.var_vdsorg_dn4 = assign29560_e28722_d_n4;
        locals.var_vdsorg_dn5 = assign29560_e28722_d_n5;
        locals.var_vdsorg_dn6 = assign29560_e28722_d_n6;
        locals.var_vdsorg_dn7 = assign29560_e28722_d_n7;
        locals.var_vdsorg_dn8 = assign29560_e28722_d_n8;
        locals.var_vdsorg_dn9 = assign29560_e28722_d_n9;
        locals.var_vdsorg_dn10 = assign29560_e28722_d_n10;
        locals.var_vdsorg_dn11 = assign29560_e28722_d_n11;
        locals.var_vdsorg_dn14 = assign29560_e28722_d_n14;

        let assign29570_e28725: f64 = if locals.var_vds > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard690 = assign29570_e28725;

        let (assign29580_e28737, assign29580_e28737_d_n0, assign29580_e28737_d_n2, assign29580_e28737_d_n4, assign29580_e28737_d_n5, assign29580_e28737_d_n6, assign29580_e28737_d_n7, assign29580_e28737_d_n8, assign29580_e28737_d_n9, assign29580_e28737_d_n10, assign29580_e28737_d_n11, assign29580_e28737_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29580_e28734: f64 = (locals.var_cox * locals.var_cox);
        let assign29580_e28735: f64 = (locals.var_q_ndepm_esi / assign29580_e28734);
        (assign29580_e28735, (((locals.var_q_ndepm_esi_dn0 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn2 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn4 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn5 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn6 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn7 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn8 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn9 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn10 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn11 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn11 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn11)))) / (assign29580_e28734 * assign29580_e28734)), (((locals.var_q_ndepm_esi_dn14 * assign29580_e28734) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn14 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn14)))) / (assign29580_e28734 * assign29580_e28734)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29580_e28737;
        locals.var_t2_dn0 = assign29580_e28737_d_n0;
        locals.var_t2_dn2 = assign29580_e28737_d_n2;
        locals.var_t2_dn4 = assign29580_e28737_d_n4;
        locals.var_t2_dn5 = assign29580_e28737_d_n5;
        locals.var_t2_dn6 = assign29580_e28737_d_n6;
        locals.var_t2_dn7 = assign29580_e28737_d_n7;
        locals.var_t2_dn8 = assign29580_e28737_d_n8;
        locals.var_t2_dn9 = assign29580_e28737_d_n9;
        locals.var_t2_dn10 = assign29580_e28737_d_n10;
        locals.var_t2_dn11 = assign29580_e28737_d_n11;
        locals.var_t2_dn14 = assign29580_e28737_d_n14;

        let (assign29590_e28751, assign29590_e28751_d_n0, assign29590_e28751_d_n2, assign29590_e28751_d_n4, assign29590_e28751_d_n5, assign29590_e28751_d_n6, assign29590_e28751_d_n7, assign29590_e28751_d_n8, assign29590_e28751_d_n9, assign29590_e28751_d_n10, assign29590_e28751_d_n11, assign29590_e28751_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29590_e28745: f64 = (locals.var_vgp + 2.0);
        let assign29590_e28747: f64 = (assign29590_e28745 - locals.var_beta_inv);
        let assign29590_e28749: f64 = (assign29590_e28747 - locals.var_vbsz__blk442);
        (assign29590_e28749, ((locals.var_vgp_dn0 - locals.var_beta_inv_dn0) - locals.var_vbsz__blk442_dn0), ((locals.var_vgp_dn2 - locals.var_beta_inv_dn2) - locals.var_vbsz__blk442_dn2), ((locals.var_vgp_dn4 - locals.var_beta_inv_dn4) - locals.var_vbsz__blk442_dn4), ((locals.var_vgp_dn5 - locals.var_beta_inv_dn5) - locals.var_vbsz__blk442_dn5), ((locals.var_vgp_dn6 - locals.var_beta_inv_dn6) - locals.var_vbsz__blk442_dn6), ((locals.var_vgp_dn7 - locals.var_beta_inv_dn7) - locals.var_vbsz__blk442_dn7), ((locals.var_vgp_dn8 - locals.var_beta_inv_dn8) - locals.var_vbsz__blk442_dn8), ((locals.var_vgp_dn9 - locals.var_beta_inv_dn9) - locals.var_vbsz__blk442_dn9), ((locals.var_vgp_dn10 - locals.var_beta_inv_dn10) - locals.var_vbsz__blk442_dn10), ((locals.var_vgp_dn11 - locals.var_beta_inv_dn11) - locals.var_vbsz__blk442_dn11), ((locals.var_vgp_dn14 - locals.var_beta_inv_dn14) - locals.var_vbsz__blk442_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29590_e28751;
        locals.var_t0_dn0 = assign29590_e28751_d_n0;
        locals.var_t0_dn2 = assign29590_e28751_d_n2;
        locals.var_t0_dn4 = assign29590_e28751_d_n4;
        locals.var_t0_dn5 = assign29590_e28751_d_n5;
        locals.var_t0_dn6 = assign29590_e28751_d_n6;
        locals.var_t0_dn7 = assign29590_e28751_d_n7;
        locals.var_t0_dn8 = assign29590_e28751_d_n8;
        locals.var_t0_dn9 = assign29590_e28751_d_n9;
        locals.var_t0_dn10 = assign29590_e28751_d_n10;
        locals.var_t0_dn11 = assign29590_e28751_d_n11;
        locals.var_t0_dn14 = assign29590_e28751_d_n14;

        let (assign29600_e28765, assign29600_e28765_d_n0, assign29600_e28765_d_n2, assign29600_e28765_d_n4, assign29600_e28765_d_n5, assign29600_e28765_d_n6, assign29600_e28765_d_n7, assign29600_e28765_d_n8, assign29600_e28765_d_n9, assign29600_e28765_d_n10, assign29600_e28765_d_n11, assign29600_e28765_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29600_e28760: f64 = (2.0 / locals.var_t2);
        let assign29600_e28762: f64 = (assign29600_e28760 * locals.var_t0);
        let assign29600_e28763: f64 = (1.0 + assign29600_e28762);
        (assign29600_e28763, (((-((2.0 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn0)), (((-((2.0 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn2)), (((-((2.0 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn4)), (((-((2.0 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn5)), (((-((2.0 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn6)), (((-((2.0 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn7)), (((-((2.0 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn8)), (((-((2.0 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn9)), (((-((2.0 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn10)), (((-((2.0 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn11)), (((-((2.0 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29600_e28760 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29600_e28765;
        locals.var_t4_dn0 = assign29600_e28765_d_n0;
        locals.var_t4_dn2 = assign29600_e28765_d_n2;
        locals.var_t4_dn4 = assign29600_e28765_d_n4;
        locals.var_t4_dn5 = assign29600_e28765_d_n5;
        locals.var_t4_dn6 = assign29600_e28765_d_n6;
        locals.var_t4_dn7 = assign29600_e28765_d_n7;
        locals.var_t4_dn8 = assign29600_e28765_d_n8;
        locals.var_t4_dn9 = assign29600_e28765_d_n9;
        locals.var_t4_dn10 = assign29600_e28765_d_n10;
        locals.var_t4_dn11 = assign29600_e28765_d_n11;
        locals.var_t4_dn14 = assign29600_e28765_d_n14;

        let assign29610_e28769: f64 = 2.0;
        let assign29610_e28774: f64 = if ((locals.var_t4 < assign29610_e28769) && (2.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard691 = assign29610_e28774;

        let (assign29620_e28788, assign29620_e28788_d_n0, assign29620_e28788_d_n2, assign29620_e28788_d_n4, assign29620_e28788_d_n5, assign29620_e28788_d_n6, assign29620_e28788_d_n7, assign29620_e28788_d_n8, assign29620_e28788_d_n9, assign29620_e28788_d_n10, assign29620_e28788_d_n11, assign29620_e28788_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29620_e28784: f64 = 2.0;
        let assign29620_e28786: f64 = (assign29620_e28784 - locals.var_t4);
        (assign29620_e28786, (-locals.var_t4_dn0), (-locals.var_t4_dn2), (-locals.var_t4_dn4), (-locals.var_t4_dn5), (-locals.var_t4_dn6), (-locals.var_t4_dn7), (-locals.var_t4_dn8), (-locals.var_t4_dn9), (-locals.var_t4_dn10), (-locals.var_t4_dn11), (-locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign29620_e28788;
        locals.var_tmf1_dn0 = assign29620_e28788_d_n0;
        locals.var_tmf1_dn2 = assign29620_e28788_d_n2;
        locals.var_tmf1_dn4 = assign29620_e28788_d_n4;
        locals.var_tmf1_dn5 = assign29620_e28788_d_n5;
        locals.var_tmf1_dn6 = assign29620_e28788_d_n6;
        locals.var_tmf1_dn7 = assign29620_e28788_d_n7;
        locals.var_tmf1_dn8 = assign29620_e28788_d_n8;
        locals.var_tmf1_dn9 = assign29620_e28788_d_n9;
        locals.var_tmf1_dn10 = assign29620_e28788_d_n10;
        locals.var_tmf1_dn11 = assign29620_e28788_d_n11;
        locals.var_tmf1_dn14 = assign29620_e28788_d_n14;

        let (assign29630_e28800, assign29630_e28800_d_n0, assign29630_e28800_d_n2, assign29630_e28800_d_n4, assign29630_e28800_d_n5, assign29630_e28800_d_n6, assign29630_e28800_d_n7, assign29630_e28800_d_n8, assign29630_e28800_d_n9, assign29630_e28800_d_n10, assign29630_e28800_d_n11, assign29630_e28800_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29630_e28798: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign29630_e28798, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign29630_e28800;
        locals.var_x2_dn0 = assign29630_e28800_d_n0;
        locals.var_x2_dn2 = assign29630_e28800_d_n2;
        locals.var_x2_dn4 = assign29630_e28800_d_n4;
        locals.var_x2_dn5 = assign29630_e28800_d_n5;
        locals.var_x2_dn6 = assign29630_e28800_d_n6;
        locals.var_x2_dn7 = assign29630_e28800_d_n7;
        locals.var_x2_dn8 = assign29630_e28800_d_n8;
        locals.var_x2_dn9 = assign29630_e28800_d_n9;
        locals.var_x2_dn10 = assign29630_e28800_d_n10;
        locals.var_x2_dn11 = assign29630_e28800_d_n11;
        locals.var_x2_dn14 = assign29630_e28800_d_n14;

        let (assign29640_e28812, assign29640_e28812_d_n0, assign29640_e28812_d_n2, assign29640_e28812_d_n4, assign29640_e28812_d_n5, assign29640_e28812_d_n6, assign29640_e28812_d_n7, assign29640_e28812_d_n8, assign29640_e28812_d_n9, assign29640_e28812_d_n10, assign29640_e28812_d_n11, assign29640_e28812_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29640_e28810: f64 = (2.0 * 2.0);
        (assign29640_e28810, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign29640_e28812;
        locals.var_xmax2_dn0 = assign29640_e28812_d_n0;
        locals.var_xmax2_dn2 = assign29640_e28812_d_n2;
        locals.var_xmax2_dn4 = assign29640_e28812_d_n4;
        locals.var_xmax2_dn5 = assign29640_e28812_d_n5;
        locals.var_xmax2_dn6 = assign29640_e28812_d_n6;
        locals.var_xmax2_dn7 = assign29640_e28812_d_n7;
        locals.var_xmax2_dn8 = assign29640_e28812_d_n8;
        locals.var_xmax2_dn9 = assign29640_e28812_d_n9;
        locals.var_xmax2_dn10 = assign29640_e28812_d_n10;
        locals.var_xmax2_dn11 = assign29640_e28812_d_n11;
        locals.var_xmax2_dn14 = assign29640_e28812_d_n14;

        let (assign29650_e28822, assign29650_e28822_d_n0, assign29650_e28822_d_n2, assign29650_e28822_d_n4, assign29650_e28822_d_n5, assign29650_e28822_d_n6, assign29650_e28822_d_n7, assign29650_e28822_d_n8, assign29650_e28822_d_n9, assign29650_e28822_d_n10, assign29650_e28822_d_n11, assign29650_e28822_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29650_e28822;
        locals.var_xp_dn0 = assign29650_e28822_d_n0;
        locals.var_xp_dn2 = assign29650_e28822_d_n2;
        locals.var_xp_dn4 = assign29650_e28822_d_n4;
        locals.var_xp_dn5 = assign29650_e28822_d_n5;
        locals.var_xp_dn6 = assign29650_e28822_d_n6;
        locals.var_xp_dn7 = assign29650_e28822_d_n7;
        locals.var_xp_dn8 = assign29650_e28822_d_n8;
        locals.var_xp_dn9 = assign29650_e28822_d_n9;
        locals.var_xp_dn10 = assign29650_e28822_d_n10;
        locals.var_xp_dn11 = assign29650_e28822_d_n11;
        locals.var_xp_dn14 = assign29650_e28822_d_n14;

        let (assign29660_e28832, assign29660_e28832_d_n0, assign29660_e28832_d_n2, assign29660_e28832_d_n4, assign29660_e28832_d_n5, assign29660_e28832_d_n6, assign29660_e28832_d_n7, assign29660_e28832_d_n8, assign29660_e28832_d_n9, assign29660_e28832_d_n10, assign29660_e28832_d_n11, assign29660_e28832_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29660_e28832;
        locals.var_xmp_dn0 = assign29660_e28832_d_n0;
        locals.var_xmp_dn2 = assign29660_e28832_d_n2;
        locals.var_xmp_dn4 = assign29660_e28832_d_n4;
        locals.var_xmp_dn5 = assign29660_e28832_d_n5;
        locals.var_xmp_dn6 = assign29660_e28832_d_n6;
        locals.var_xmp_dn7 = assign29660_e28832_d_n7;
        locals.var_xmp_dn8 = assign29660_e28832_d_n8;
        locals.var_xmp_dn9 = assign29660_e28832_d_n9;
        locals.var_xmp_dn10 = assign29660_e28832_d_n10;
        locals.var_xmp_dn11 = assign29660_e28832_d_n11;
        locals.var_xmp_dn14 = assign29660_e28832_d_n14;

        let (assign29670_e28842,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29670_e28842;

        let (assign29680_e28852,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29680_e28852;

        let (assign29690_e28862, assign29690_e28862_d_n0, assign29690_e28862_d_n2, assign29690_e28862_d_n4, assign29690_e28862_d_n5, assign29690_e28862_d_n6, assign29690_e28862_d_n7, assign29690_e28862_d_n8, assign29690_e28862_d_n9, assign29690_e28862_d_n10, assign29690_e28862_d_n11, assign29690_e28862_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29690_e28862;
        locals.var_arg_dn0 = assign29690_e28862_d_n0;
        locals.var_arg_dn2 = assign29690_e28862_d_n2;
        locals.var_arg_dn4 = assign29690_e28862_d_n4;
        locals.var_arg_dn5 = assign29690_e28862_d_n5;
        locals.var_arg_dn6 = assign29690_e28862_d_n6;
        locals.var_arg_dn7 = assign29690_e28862_d_n7;
        locals.var_arg_dn8 = assign29690_e28862_d_n8;
        locals.var_arg_dn9 = assign29690_e28862_d_n9;
        locals.var_arg_dn10 = assign29690_e28862_d_n10;
        locals.var_arg_dn11 = assign29690_e28862_d_n11;
        locals.var_arg_dn14 = assign29690_e28862_d_n14;

        let (assign29700_e28872, assign29700_e28872_d_n0, assign29700_e28872_d_n2, assign29700_e28872_d_n4, assign29700_e28872_d_n5, assign29700_e28872_d_n6, assign29700_e28872_d_n7, assign29700_e28872_d_n8, assign29700_e28872_d_n9, assign29700_e28872_d_n10, assign29700_e28872_d_n11, assign29700_e28872_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29700_e28872;
        locals.var_dnm_dn0 = assign29700_e28872_d_n0;
        locals.var_dnm_dn2 = assign29700_e28872_d_n2;
        locals.var_dnm_dn4 = assign29700_e28872_d_n4;
        locals.var_dnm_dn5 = assign29700_e28872_d_n5;
        locals.var_dnm_dn6 = assign29700_e28872_d_n6;
        locals.var_dnm_dn7 = assign29700_e28872_d_n7;
        locals.var_dnm_dn8 = assign29700_e28872_d_n8;
        locals.var_dnm_dn9 = assign29700_e28872_d_n9;
        locals.var_dnm_dn10 = assign29700_e28872_d_n10;
        locals.var_dnm_dn11 = assign29700_e28872_d_n11;
        locals.var_dnm_dn14 = assign29700_e28872_d_n14;

        let (assign29710_e28884, assign29710_e28884_d_n0, assign29710_e28884_d_n2, assign29710_e28884_d_n4, assign29710_e28884_d_n5, assign29710_e28884_d_n6, assign29710_e28884_d_n7, assign29710_e28884_d_n8, assign29710_e28884_d_n9, assign29710_e28884_d_n10, assign29710_e28884_d_n11, assign29710_e28884_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29710_e28882: f64 = (locals.var_xp * locals.var_x2);
        (assign29710_e28882, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29710_e28884;
        locals.var_xp_dn0 = assign29710_e28884_d_n0;
        locals.var_xp_dn2 = assign29710_e28884_d_n2;
        locals.var_xp_dn4 = assign29710_e28884_d_n4;
        locals.var_xp_dn5 = assign29710_e28884_d_n5;
        locals.var_xp_dn6 = assign29710_e28884_d_n6;
        locals.var_xp_dn7 = assign29710_e28884_d_n7;
        locals.var_xp_dn8 = assign29710_e28884_d_n8;
        locals.var_xp_dn9 = assign29710_e28884_d_n9;
        locals.var_xp_dn10 = assign29710_e28884_d_n10;
        locals.var_xp_dn11 = assign29710_e28884_d_n11;
        locals.var_xp_dn14 = assign29710_e28884_d_n14;

        let (assign29720_e28896, assign29720_e28896_d_n0, assign29720_e28896_d_n2, assign29720_e28896_d_n4, assign29720_e28896_d_n5, assign29720_e28896_d_n6, assign29720_e28896_d_n7, assign29720_e28896_d_n8, assign29720_e28896_d_n9, assign29720_e28896_d_n10, assign29720_e28896_d_n11, assign29720_e28896_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29720_e28894: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29720_e28894, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29720_e28896;
        locals.var_xmp_dn0 = assign29720_e28896_d_n0;
        locals.var_xmp_dn2 = assign29720_e28896_d_n2;
        locals.var_xmp_dn4 = assign29720_e28896_d_n4;
        locals.var_xmp_dn5 = assign29720_e28896_d_n5;
        locals.var_xmp_dn6 = assign29720_e28896_d_n6;
        locals.var_xmp_dn7 = assign29720_e28896_d_n7;
        locals.var_xmp_dn8 = assign29720_e28896_d_n8;
        locals.var_xmp_dn9 = assign29720_e28896_d_n9;
        locals.var_xmp_dn10 = assign29720_e28896_d_n10;
        locals.var_xmp_dn11 = assign29720_e28896_d_n11;
        locals.var_xmp_dn14 = assign29720_e28896_d_n14;

        let (assign29730_e28908, assign29730_e28908_d_n0, assign29730_e28908_d_n2, assign29730_e28908_d_n4, assign29730_e28908_d_n5, assign29730_e28908_d_n6, assign29730_e28908_d_n7, assign29730_e28908_d_n8, assign29730_e28908_d_n9, assign29730_e28908_d_n10, assign29730_e28908_d_n11, assign29730_e28908_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29730_e28906: f64 = (locals.var_xp * locals.var_x2);
        (assign29730_e28906, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29730_e28908;
        locals.var_xp_dn0 = assign29730_e28908_d_n0;
        locals.var_xp_dn2 = assign29730_e28908_d_n2;
        locals.var_xp_dn4 = assign29730_e28908_d_n4;
        locals.var_xp_dn5 = assign29730_e28908_d_n5;
        locals.var_xp_dn6 = assign29730_e28908_d_n6;
        locals.var_xp_dn7 = assign29730_e28908_d_n7;
        locals.var_xp_dn8 = assign29730_e28908_d_n8;
        locals.var_xp_dn9 = assign29730_e28908_d_n9;
        locals.var_xp_dn10 = assign29730_e28908_d_n10;
        locals.var_xp_dn11 = assign29730_e28908_d_n11;
        locals.var_xp_dn14 = assign29730_e28908_d_n14;

        let (assign29740_e28920, assign29740_e28920_d_n0, assign29740_e28920_d_n2, assign29740_e28920_d_n4, assign29740_e28920_d_n5, assign29740_e28920_d_n6, assign29740_e28920_d_n7, assign29740_e28920_d_n8, assign29740_e28920_d_n9, assign29740_e28920_d_n10, assign29740_e28920_d_n11, assign29740_e28920_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29740_e28918: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29740_e28918, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29740_e28920;
        locals.var_xmp_dn0 = assign29740_e28920_d_n0;
        locals.var_xmp_dn2 = assign29740_e28920_d_n2;
        locals.var_xmp_dn4 = assign29740_e28920_d_n4;
        locals.var_xmp_dn5 = assign29740_e28920_d_n5;
        locals.var_xmp_dn6 = assign29740_e28920_d_n6;
        locals.var_xmp_dn7 = assign29740_e28920_d_n7;
        locals.var_xmp_dn8 = assign29740_e28920_d_n8;
        locals.var_xmp_dn9 = assign29740_e28920_d_n9;
        locals.var_xmp_dn10 = assign29740_e28920_d_n10;
        locals.var_xmp_dn11 = assign29740_e28920_d_n11;
        locals.var_xmp_dn14 = assign29740_e28920_d_n14;

        let (assign29750_e28932, assign29750_e28932_d_n0, assign29750_e28932_d_n2, assign29750_e28932_d_n4, assign29750_e28932_d_n5, assign29750_e28932_d_n6, assign29750_e28932_d_n7, assign29750_e28932_d_n8, assign29750_e28932_d_n9, assign29750_e28932_d_n10, assign29750_e28932_d_n11, assign29750_e28932_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29750_e28930: f64 = (locals.var_xp + locals.var_xmp);
        (assign29750_e28930, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29750_e28932;
        locals.var_arg_dn0 = assign29750_e28932_d_n0;
        locals.var_arg_dn2 = assign29750_e28932_d_n2;
        locals.var_arg_dn4 = assign29750_e28932_d_n4;
        locals.var_arg_dn5 = assign29750_e28932_d_n5;
        locals.var_arg_dn6 = assign29750_e28932_d_n6;
        locals.var_arg_dn7 = assign29750_e28932_d_n7;
        locals.var_arg_dn8 = assign29750_e28932_d_n8;
        locals.var_arg_dn9 = assign29750_e28932_d_n9;
        locals.var_arg_dn10 = assign29750_e28932_d_n10;
        locals.var_arg_dn11 = assign29750_e28932_d_n11;
        locals.var_arg_dn14 = assign29750_e28932_d_n14;

        let (assign29760_e28942, assign29760_e28942_d_n0, assign29760_e28942_d_n2, assign29760_e28942_d_n4, assign29760_e28942_d_n5, assign29760_e28942_d_n6, assign29760_e28942_d_n7, assign29760_e28942_d_n8, assign29760_e28942_d_n9, assign29760_e28942_d_n10, assign29760_e28942_d_n11, assign29760_e28942_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29760_e28942;
        locals.var_dnm_dn0 = assign29760_e28942_d_n0;
        locals.var_dnm_dn2 = assign29760_e28942_d_n2;
        locals.var_dnm_dn4 = assign29760_e28942_d_n4;
        locals.var_dnm_dn5 = assign29760_e28942_d_n5;
        locals.var_dnm_dn6 = assign29760_e28942_d_n6;
        locals.var_dnm_dn7 = assign29760_e28942_d_n7;
        locals.var_dnm_dn8 = assign29760_e28942_d_n8;
        locals.var_dnm_dn9 = assign29760_e28942_d_n9;
        locals.var_dnm_dn10 = assign29760_e28942_d_n10;
        locals.var_dnm_dn11 = assign29760_e28942_d_n11;
        locals.var_dnm_dn14 = assign29760_e28942_d_n14;

        let assign29770_e28957: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard692 = assign29770_e28957;

        let assign29780_e28960: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard693 = assign29780_e28960;

        let (assign29790_e28974,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) && (locals.var_guard693 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29790_e28974;

        let assign29800_e28977: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard694 = assign29800_e28977;

        let (assign29810_e28994,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29810_e28994;

        let assign29820_e28997: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard695 = assign29820_e28997;

        let (assign29830_e29017,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 == 0.0)) && (locals.var_guard695 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29830_e29017;

        let assign29840_e29020: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard696 = assign29840_e29020;

        let (assign29850_e29043,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 == 0.0)) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29850_e29043;

        let (assign29860_e29055,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29860_e29055;

    }

    pub(super) fn stamp_transient_block_86(
        locals: &mut StampLocals,
    ) {
        let mut assign29870_loop_guard: usize = 0;
        while {
            let assign29870_cond_e29068: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29870_cond_e29068 != 0.0
        } {
            assign29870_loop_guard += 1;
            assert!(assign29870_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29870_body0_e29081, assign29870_body0_e29081_d_n0, assign29870_body0_e29081_d_n2, assign29870_body0_e29081_d_n4, assign29870_body0_e29081_d_n5, assign29870_body0_e29081_d_n6, assign29870_body0_e29081_d_n7, assign29870_body0_e29081_d_n8, assign29870_body0_e29081_d_n9, assign29870_body0_e29081_d_n10, assign29870_body0_e29081_d_n11, assign29870_body0_e29081_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) {
        let assign29870_body0_e29079: f64 = (locals.var_dnm).sqrt();
        (assign29870_body0_e29079, (locals.var_dnm_dn0 / (2.0 * assign29870_body0_e29079)), (locals.var_dnm_dn2 / (2.0 * assign29870_body0_e29079)), (locals.var_dnm_dn4 / (2.0 * assign29870_body0_e29079)), (locals.var_dnm_dn5 / (2.0 * assign29870_body0_e29079)), (locals.var_dnm_dn6 / (2.0 * assign29870_body0_e29079)), (locals.var_dnm_dn7 / (2.0 * assign29870_body0_e29079)), (locals.var_dnm_dn8 / (2.0 * assign29870_body0_e29079)), (locals.var_dnm_dn9 / (2.0 * assign29870_body0_e29079)), (locals.var_dnm_dn10 / (2.0 * assign29870_body0_e29079)), (locals.var_dnm_dn11 / (2.0 * assign29870_body0_e29079)), (locals.var_dnm_dn14 / (2.0 * assign29870_body0_e29079)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign29870_body0_e29081;
            locals.var_dnm_dn0 = assign29870_body0_e29081_d_n0;
            locals.var_dnm_dn2 = assign29870_body0_e29081_d_n2;
            locals.var_dnm_dn4 = assign29870_body0_e29081_d_n4;
            locals.var_dnm_dn5 = assign29870_body0_e29081_d_n5;
            locals.var_dnm_dn6 = assign29870_body0_e29081_d_n6;
            locals.var_dnm_dn7 = assign29870_body0_e29081_d_n7;
            locals.var_dnm_dn8 = assign29870_body0_e29081_d_n8;
            locals.var_dnm_dn9 = assign29870_body0_e29081_d_n9;
            locals.var_dnm_dn10 = assign29870_body0_e29081_d_n10;
            locals.var_dnm_dn11 = assign29870_body0_e29081_d_n11;
            locals.var_dnm_dn14 = assign29870_body0_e29081_d_n14;
            let (assign29870_body1_e29095,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 != 0.0)) {
        let assign29870_body1_e29093: f64 = (locals.var_m0 + 1.0);
        (assign29870_body1_e29093,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29870_body1_e29095;
        }

        let (assign29880_e29119, assign29880_e29119_d_n0, assign29880_e29119_d_n2, assign29880_e29119_d_n4, assign29880_e29119_d_n5, assign29880_e29119_d_n6, assign29880_e29119_d_n7, assign29880_e29119_d_n8, assign29880_e29119_d_n9, assign29880_e29119_d_n10, assign29880_e29119_d_n11, assign29880_e29119_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) && (locals.var_guard692 == 0.0)) {
        let (assign29880_e29117, assign29880_e29117_d_n0, assign29880_e29117_d_n2, assign29880_e29117_d_n4, assign29880_e29117_d_n5, assign29880_e29117_d_n6, assign29880_e29117_d_n7, assign29880_e29117_d_n8, assign29880_e29117_d_n9, assign29880_e29117_d_n10, assign29880_e29117_d_n11, assign29880_e29117_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29880_e29114: f64 = (2.0 * 2.0);
                let assign29880_e29115: f64 = (1.0 / assign29880_e29114);
                let assign29880_e29116: f64 = (locals.var_dnm).powf(assign29880_e29115);
                (assign29880_e29116, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn11)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29880_e29115) as f64).is_finite() && ((assign29880_e29115) as f64).fract() == 0.0 { if assign29880_e29115 == 0.0 { 0.0 } else { (assign29880_e29115 * ((locals.var_dnm).powf(assign29880_e29115 - 1.0) * locals.var_dnm_dn14)) } } else { (assign29880_e29116 * (assign29880_e29115 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign29880_e29117, assign29880_e29117_d_n0, assign29880_e29117_d_n2, assign29880_e29117_d_n4, assign29880_e29117_d_n5, assign29880_e29117_d_n6, assign29880_e29117_d_n7, assign29880_e29117_d_n8, assign29880_e29117_d_n9, assign29880_e29117_d_n10, assign29880_e29117_d_n11, assign29880_e29117_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29880_e29119;
        locals.var_dnm_dn0 = assign29880_e29119_d_n0;
        locals.var_dnm_dn2 = assign29880_e29119_d_n2;
        locals.var_dnm_dn4 = assign29880_e29119_d_n4;
        locals.var_dnm_dn5 = assign29880_e29119_d_n5;
        locals.var_dnm_dn6 = assign29880_e29119_d_n6;
        locals.var_dnm_dn7 = assign29880_e29119_d_n7;
        locals.var_dnm_dn8 = assign29880_e29119_d_n8;
        locals.var_dnm_dn9 = assign29880_e29119_d_n9;
        locals.var_dnm_dn10 = assign29880_e29119_d_n10;
        locals.var_dnm_dn11 = assign29880_e29119_d_n11;
        locals.var_dnm_dn14 = assign29880_e29119_d_n14;

        let (assign29890_e29131, assign29890_e29131_d_n0, assign29890_e29131_d_n2, assign29890_e29131_d_n4, assign29890_e29131_d_n5, assign29890_e29131_d_n6, assign29890_e29131_d_n7, assign29890_e29131_d_n8, assign29890_e29131_d_n9, assign29890_e29131_d_n10, assign29890_e29131_d_n11, assign29890_e29131_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29890_e29129: f64 = (1.0 / locals.var_dnm);
        (assign29890_e29129, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29890_e29131;
        locals.var_dnm_dn0 = assign29890_e29131_d_n0;
        locals.var_dnm_dn2 = assign29890_e29131_d_n2;
        locals.var_dnm_dn4 = assign29890_e29131_d_n4;
        locals.var_dnm_dn5 = assign29890_e29131_d_n5;
        locals.var_dnm_dn6 = assign29890_e29131_d_n6;
        locals.var_dnm_dn7 = assign29890_e29131_d_n7;
        locals.var_dnm_dn8 = assign29890_e29131_d_n8;
        locals.var_dnm_dn9 = assign29890_e29131_d_n9;
        locals.var_dnm_dn10 = assign29890_e29131_d_n10;
        locals.var_dnm_dn11 = assign29890_e29131_d_n11;
        locals.var_dnm_dn14 = assign29890_e29131_d_n14;

        let (assign29900_e29145, assign29900_e29145_d_n0, assign29900_e29145_d_n2, assign29900_e29145_d_n4, assign29900_e29145_d_n5, assign29900_e29145_d_n6, assign29900_e29145_d_n7, assign29900_e29145_d_n8, assign29900_e29145_d_n9, assign29900_e29145_d_n10, assign29900_e29145_d_n11, assign29900_e29145_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29900_e29141: f64 = (locals.var_tmf1 * 2.0);
        let assign29900_e29143: f64 = (assign29900_e29141 * locals.var_dnm);
        (assign29900_e29143, (((locals.var_tmf1_dn0 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 2.0) * locals.var_dnm) + (assign29900_e29141 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign29900_e29145;
        locals.var_tmf0_dn0 = assign29900_e29145_d_n0;
        locals.var_tmf0_dn2 = assign29900_e29145_d_n2;
        locals.var_tmf0_dn4 = assign29900_e29145_d_n4;
        locals.var_tmf0_dn5 = assign29900_e29145_d_n5;
        locals.var_tmf0_dn6 = assign29900_e29145_d_n6;
        locals.var_tmf0_dn7 = assign29900_e29145_d_n7;
        locals.var_tmf0_dn8 = assign29900_e29145_d_n8;
        locals.var_tmf0_dn9 = assign29900_e29145_d_n9;
        locals.var_tmf0_dn10 = assign29900_e29145_d_n10;
        locals.var_tmf0_dn11 = assign29900_e29145_d_n11;
        locals.var_tmf0_dn14 = assign29900_e29145_d_n14;

        let (assign29910_e29161, assign29910_e29161_d_n0, assign29910_e29161_d_n2, assign29910_e29161_d_n4, assign29910_e29161_d_n5, assign29910_e29161_d_n6, assign29910_e29161_d_n7, assign29910_e29161_d_n8, assign29910_e29161_d_n9, assign29910_e29161_d_n10, assign29910_e29161_d_n11, assign29910_e29161_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29910_e29155: f64 = (2.0 * locals.var_xmp);
        let assign29910_e29157: f64 = (assign29910_e29155 * locals.var_dnm);
        let assign29910_e29159: f64 = (assign29910_e29157 / locals.var_arg);
        (assign29910_e29159, ((((((2.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn11)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign29910_e29155 * locals.var_dnm_dn14)) * locals.var_arg) - (assign29910_e29157 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29910_e29161;
        locals.var_t0_dn0 = assign29910_e29161_d_n0;
        locals.var_t0_dn2 = assign29910_e29161_d_n2;
        locals.var_t0_dn4 = assign29910_e29161_d_n4;
        locals.var_t0_dn5 = assign29910_e29161_d_n5;
        locals.var_t0_dn6 = assign29910_e29161_d_n6;
        locals.var_t0_dn7 = assign29910_e29161_d_n7;
        locals.var_t0_dn8 = assign29910_e29161_d_n8;
        locals.var_t0_dn9 = assign29910_e29161_d_n9;
        locals.var_t0_dn10 = assign29910_e29161_d_n10;
        locals.var_t0_dn11 = assign29910_e29161_d_n11;
        locals.var_t0_dn14 = assign29910_e29161_d_n14;

        let (assign29920_e29175, assign29920_e29175_d_n0, assign29920_e29175_d_n2, assign29920_e29175_d_n4, assign29920_e29175_d_n5, assign29920_e29175_d_n6, assign29920_e29175_d_n7, assign29920_e29175_d_n8, assign29920_e29175_d_n9, assign29920_e29175_d_n10, assign29920_e29175_d_n11, assign29920_e29175_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign29920_e29171: f64 = 2.0;
        let assign29920_e29173: f64 = (assign29920_e29171 - locals.var_tmf0);
        (assign29920_e29173, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign29920_e29175;
        locals.var_t9_dn0 = assign29920_e29175_d_n0;
        locals.var_t9_dn2 = assign29920_e29175_d_n2;
        locals.var_t9_dn4 = assign29920_e29175_d_n4;
        locals.var_t9_dn5 = assign29920_e29175_d_n5;
        locals.var_t9_dn6 = assign29920_e29175_d_n6;
        locals.var_t9_dn7 = assign29920_e29175_d_n7;
        locals.var_t9_dn8 = assign29920_e29175_d_n8;
        locals.var_t9_dn9 = assign29920_e29175_d_n9;
        locals.var_t9_dn10 = assign29920_e29175_d_n10;
        locals.var_t9_dn11 = assign29920_e29175_d_n11;
        locals.var_t9_dn14 = assign29920_e29175_d_n14;

        let (assign29930_e29185, assign29930_e29185_d_n0, assign29930_e29185_d_n2, assign29930_e29185_d_n4, assign29930_e29185_d_n5, assign29930_e29185_d_n6, assign29930_e29185_d_n7, assign29930_e29185_d_n8, assign29930_e29185_d_n9, assign29930_e29185_d_n10, assign29930_e29185_d_n11, assign29930_e29185_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29930_e29185;
        locals.var_t0_dn0 = assign29930_e29185_d_n0;
        locals.var_t0_dn2 = assign29930_e29185_d_n2;
        locals.var_t0_dn4 = assign29930_e29185_d_n4;
        locals.var_t0_dn5 = assign29930_e29185_d_n5;
        locals.var_t0_dn6 = assign29930_e29185_d_n6;
        locals.var_t0_dn7 = assign29930_e29185_d_n7;
        locals.var_t0_dn8 = assign29930_e29185_d_n8;
        locals.var_t0_dn9 = assign29930_e29185_d_n9;
        locals.var_t0_dn10 = assign29930_e29185_d_n10;
        locals.var_t0_dn11 = assign29930_e29185_d_n11;
        locals.var_t0_dn14 = assign29930_e29185_d_n14;

        let (assign29940_e29196, assign29940_e29196_d_n0, assign29940_e29196_d_n2, assign29940_e29196_d_n4, assign29940_e29196_d_n5, assign29940_e29196_d_n6, assign29940_e29196_d_n7, assign29940_e29196_d_n8, assign29940_e29196_d_n9, assign29940_e29196_d_n10, assign29940_e29196_d_n11, assign29940_e29196_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign29940_e29196;
        locals.var_t9_dn0 = assign29940_e29196_d_n0;
        locals.var_t9_dn2 = assign29940_e29196_d_n2;
        locals.var_t9_dn4 = assign29940_e29196_d_n4;
        locals.var_t9_dn5 = assign29940_e29196_d_n5;
        locals.var_t9_dn6 = assign29940_e29196_d_n6;
        locals.var_t9_dn7 = assign29940_e29196_d_n7;
        locals.var_t9_dn8 = assign29940_e29196_d_n8;
        locals.var_t9_dn9 = assign29940_e29196_d_n9;
        locals.var_t9_dn10 = assign29940_e29196_d_n10;
        locals.var_t9_dn11 = assign29940_e29196_d_n11;
        locals.var_t9_dn14 = assign29940_e29196_d_n14;

        let (assign29950_e29207, assign29950_e29207_d_n0, assign29950_e29207_d_n2, assign29950_e29207_d_n4, assign29950_e29207_d_n5, assign29950_e29207_d_n6, assign29950_e29207_d_n7, assign29950_e29207_d_n8, assign29950_e29207_d_n9, assign29950_e29207_d_n10, assign29950_e29207_d_n11, assign29950_e29207_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29950_e29207;
        locals.var_t0_dn0 = assign29950_e29207_d_n0;
        locals.var_t0_dn2 = assign29950_e29207_d_n2;
        locals.var_t0_dn4 = assign29950_e29207_d_n4;
        locals.var_t0_dn5 = assign29950_e29207_d_n5;
        locals.var_t0_dn6 = assign29950_e29207_d_n6;
        locals.var_t0_dn7 = assign29950_e29207_d_n7;
        locals.var_t0_dn8 = assign29950_e29207_d_n8;
        locals.var_t0_dn9 = assign29950_e29207_d_n9;
        locals.var_t0_dn10 = assign29950_e29207_d_n10;
        locals.var_t0_dn11 = assign29950_e29207_d_n11;
        locals.var_t0_dn14 = assign29950_e29207_d_n14;

        let (assign29960_e29217, assign29960_e29217_d_n0, assign29960_e29217_d_n2, assign29960_e29217_d_n4, assign29960_e29217_d_n5, assign29960_e29217_d_n6, assign29960_e29217_d_n7, assign29960_e29217_d_n8, assign29960_e29217_d_n9, assign29960_e29217_d_n10, assign29960_e29217_d_n11, assign29960_e29217_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29960_e29215: f64 = (locals.var_t9 + 1e-25);
        (assign29960_e29215, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign29960_e29217;
        locals.var_t9_dn0 = assign29960_e29217_d_n0;
        locals.var_t9_dn2 = assign29960_e29217_d_n2;
        locals.var_t9_dn4 = assign29960_e29217_d_n4;
        locals.var_t9_dn5 = assign29960_e29217_d_n5;
        locals.var_t9_dn6 = assign29960_e29217_d_n6;
        locals.var_t9_dn7 = assign29960_e29217_d_n7;
        locals.var_t9_dn8 = assign29960_e29217_d_n8;
        locals.var_t9_dn9 = assign29960_e29217_d_n9;
        locals.var_t9_dn10 = assign29960_e29217_d_n10;
        locals.var_t9_dn11 = assign29960_e29217_d_n11;
        locals.var_t9_dn14 = assign29960_e29217_d_n14;

        let (assign29970_e29226, assign29970_e29226_d_n0, assign29970_e29226_d_n2, assign29970_e29226_d_n4, assign29970_e29226_d_n5, assign29970_e29226_d_n6, assign29970_e29226_d_n7, assign29970_e29226_d_n8, assign29970_e29226_d_n9, assign29970_e29226_d_n10, assign29970_e29226_d_n11, assign29970_e29226_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29970_e29224: f64 = (locals.var_t9).sqrt();
        (assign29970_e29224, (locals.var_t9_dn0 / (2.0 * assign29970_e29224)), (locals.var_t9_dn2 / (2.0 * assign29970_e29224)), (locals.var_t9_dn4 / (2.0 * assign29970_e29224)), (locals.var_t9_dn5 / (2.0 * assign29970_e29224)), (locals.var_t9_dn6 / (2.0 * assign29970_e29224)), (locals.var_t9_dn7 / (2.0 * assign29970_e29224)), (locals.var_t9_dn8 / (2.0 * assign29970_e29224)), (locals.var_t9_dn9 / (2.0 * assign29970_e29224)), (locals.var_t9_dn10 / (2.0 * assign29970_e29224)), (locals.var_t9_dn11 / (2.0 * assign29970_e29224)), (locals.var_t9_dn14 / (2.0 * assign29970_e29224)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign29970_e29226;
        locals.var_t3_dn0 = assign29970_e29226_d_n0;
        locals.var_t3_dn2 = assign29970_e29226_d_n2;
        locals.var_t3_dn4 = assign29970_e29226_d_n4;
        locals.var_t3_dn5 = assign29970_e29226_d_n5;
        locals.var_t3_dn6 = assign29970_e29226_d_n6;
        locals.var_t3_dn7 = assign29970_e29226_d_n7;
        locals.var_t3_dn8 = assign29970_e29226_d_n8;
        locals.var_t3_dn9 = assign29970_e29226_d_n9;
        locals.var_t3_dn10 = assign29970_e29226_d_n10;
        locals.var_t3_dn11 = assign29970_e29226_d_n11;
        locals.var_t3_dn14 = assign29970_e29226_d_n14;

        let (assign29980_e29238, assign29980_e29238_d_n0, assign29980_e29238_d_n2, assign29980_e29238_d_n4, assign29980_e29238_d_n5, assign29980_e29238_d_n6, assign29980_e29238_d_n7, assign29980_e29238_d_n8, assign29980_e29238_d_n9, assign29980_e29238_d_n10, assign29980_e29238_d_n11, assign29980_e29238_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29980_e29235: f64 = (1.0 - locals.var_t3);
        let assign29980_e29236: f64 = (locals.var_t2 * assign29980_e29235);
        (assign29980_e29236, ((locals.var_t2_dn0 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn0))), ((locals.var_t2_dn2 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn2))), ((locals.var_t2_dn4 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn4))), ((locals.var_t2_dn5 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn5))), ((locals.var_t2_dn6 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn6))), ((locals.var_t2_dn7 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn7))), ((locals.var_t2_dn8 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn8))), ((locals.var_t2_dn9 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn9))), ((locals.var_t2_dn10 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn10))), ((locals.var_t2_dn11 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn11))), ((locals.var_t2_dn14 * assign29980_e29235) + (locals.var_t2 * (-locals.var_t3_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29980_e29238;
        locals.var_t4_dn0 = assign29980_e29238_d_n0;
        locals.var_t4_dn2 = assign29980_e29238_d_n2;
        locals.var_t4_dn4 = assign29980_e29238_d_n4;
        locals.var_t4_dn5 = assign29980_e29238_d_n5;
        locals.var_t4_dn6 = assign29980_e29238_d_n6;
        locals.var_t4_dn7 = assign29980_e29238_d_n7;
        locals.var_t4_dn8 = assign29980_e29238_d_n8;
        locals.var_t4_dn9 = assign29980_e29238_d_n9;
        locals.var_t4_dn10 = assign29980_e29238_d_n10;
        locals.var_t4_dn11 = assign29980_e29238_d_n11;
        locals.var_t4_dn14 = assign29980_e29238_d_n14;

        let (assign29990_e29250, assign29990_e29250_d_n0, assign29990_e29250_d_n2, assign29990_e29250_d_n4, assign29990_e29250_d_n5, assign29990_e29250_d_n6, assign29990_e29250_d_n7, assign29990_e29250_d_n8, assign29990_e29250_d_n9, assign29990_e29250_d_n10, assign29990_e29250_d_n11, assign29990_e29250_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29990_e29246: f64 = (locals.var_vgp + 2.0);
        let assign29990_e29248: f64 = (assign29990_e29246 + locals.var_t4);
        (assign29990_e29248, (locals.var_vgp_dn0 + locals.var_t4_dn0), (locals.var_vgp_dn2 + locals.var_t4_dn2), (locals.var_vgp_dn4 + locals.var_t4_dn4), (locals.var_vgp_dn5 + locals.var_t4_dn5), (locals.var_vgp_dn6 + locals.var_t4_dn6), (locals.var_vgp_dn7 + locals.var_t4_dn7), (locals.var_vgp_dn8 + locals.var_t4_dn8), (locals.var_vgp_dn9 + locals.var_t4_dn9), (locals.var_vgp_dn10 + locals.var_t4_dn10), (locals.var_vgp_dn11 + locals.var_t4_dn11), (locals.var_vgp_dn14 + locals.var_t4_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign29990_e29250;
        locals.var_t10_dn0 = assign29990_e29250_d_n0;
        locals.var_t10_dn2 = assign29990_e29250_d_n2;
        locals.var_t10_dn4 = assign29990_e29250_d_n4;
        locals.var_t10_dn5 = assign29990_e29250_d_n5;
        locals.var_t10_dn6 = assign29990_e29250_d_n6;
        locals.var_t10_dn7 = assign29990_e29250_d_n7;
        locals.var_t10_dn8 = assign29990_e29250_d_n8;
        locals.var_t10_dn9 = assign29990_e29250_d_n9;
        locals.var_t10_dn10 = assign29990_e29250_d_n10;
        locals.var_t10_dn11 = assign29990_e29250_d_n11;
        locals.var_t10_dn14 = assign29990_e29250_d_n14;

        let assign30000_e29254: f64 = (0.3 + 0.2);
        let assign30000_e29259: f64 = if ((locals.var_t10 < assign30000_e29254) && (0.2 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard697 = assign30000_e29259;

        let (assign30010_e29273, assign30010_e29273_d_n0, assign30010_e29273_d_n2, assign30010_e29273_d_n4, assign30010_e29273_d_n5, assign30010_e29273_d_n6, assign30010_e29273_d_n7, assign30010_e29273_d_n8, assign30010_e29273_d_n9, assign30010_e29273_d_n10, assign30010_e29273_d_n11, assign30010_e29273_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30010_e29269: f64 = (0.3 + 0.2);
        let assign30010_e29271: f64 = (assign30010_e29269 - locals.var_t10);
        (assign30010_e29271, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign30010_e29273;
        locals.var_tmf1_dn0 = assign30010_e29273_d_n0;
        locals.var_tmf1_dn2 = assign30010_e29273_d_n2;
        locals.var_tmf1_dn4 = assign30010_e29273_d_n4;
        locals.var_tmf1_dn5 = assign30010_e29273_d_n5;
        locals.var_tmf1_dn6 = assign30010_e29273_d_n6;
        locals.var_tmf1_dn7 = assign30010_e29273_d_n7;
        locals.var_tmf1_dn8 = assign30010_e29273_d_n8;
        locals.var_tmf1_dn9 = assign30010_e29273_d_n9;
        locals.var_tmf1_dn10 = assign30010_e29273_d_n10;
        locals.var_tmf1_dn11 = assign30010_e29273_d_n11;
        locals.var_tmf1_dn14 = assign30010_e29273_d_n14;

        let (assign30020_e29285, assign30020_e29285_d_n0, assign30020_e29285_d_n2, assign30020_e29285_d_n4, assign30020_e29285_d_n5, assign30020_e29285_d_n6, assign30020_e29285_d_n7, assign30020_e29285_d_n8, assign30020_e29285_d_n9, assign30020_e29285_d_n10, assign30020_e29285_d_n11, assign30020_e29285_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30020_e29283: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign30020_e29283, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign30020_e29285;
        locals.var_x2_dn0 = assign30020_e29285_d_n0;
        locals.var_x2_dn2 = assign30020_e29285_d_n2;
        locals.var_x2_dn4 = assign30020_e29285_d_n4;
        locals.var_x2_dn5 = assign30020_e29285_d_n5;
        locals.var_x2_dn6 = assign30020_e29285_d_n6;
        locals.var_x2_dn7 = assign30020_e29285_d_n7;
        locals.var_x2_dn8 = assign30020_e29285_d_n8;
        locals.var_x2_dn9 = assign30020_e29285_d_n9;
        locals.var_x2_dn10 = assign30020_e29285_d_n10;
        locals.var_x2_dn11 = assign30020_e29285_d_n11;
        locals.var_x2_dn14 = assign30020_e29285_d_n14;

        let (assign30030_e29297, assign30030_e29297_d_n0, assign30030_e29297_d_n2, assign30030_e29297_d_n4, assign30030_e29297_d_n5, assign30030_e29297_d_n6, assign30030_e29297_d_n7, assign30030_e29297_d_n8, assign30030_e29297_d_n9, assign30030_e29297_d_n10, assign30030_e29297_d_n11, assign30030_e29297_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30030_e29295: f64 = (0.2 * 0.2);
        (assign30030_e29295, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign30030_e29297;
        locals.var_xmax2_dn0 = assign30030_e29297_d_n0;
        locals.var_xmax2_dn2 = assign30030_e29297_d_n2;
        locals.var_xmax2_dn4 = assign30030_e29297_d_n4;
        locals.var_xmax2_dn5 = assign30030_e29297_d_n5;
        locals.var_xmax2_dn6 = assign30030_e29297_d_n6;
        locals.var_xmax2_dn7 = assign30030_e29297_d_n7;
        locals.var_xmax2_dn8 = assign30030_e29297_d_n8;
        locals.var_xmax2_dn9 = assign30030_e29297_d_n9;
        locals.var_xmax2_dn10 = assign30030_e29297_d_n10;
        locals.var_xmax2_dn11 = assign30030_e29297_d_n11;
        locals.var_xmax2_dn14 = assign30030_e29297_d_n14;

        let (assign30040_e29307, assign30040_e29307_d_n0, assign30040_e29307_d_n2, assign30040_e29307_d_n4, assign30040_e29307_d_n5, assign30040_e29307_d_n6, assign30040_e29307_d_n7, assign30040_e29307_d_n8, assign30040_e29307_d_n9, assign30040_e29307_d_n10, assign30040_e29307_d_n11, assign30040_e29307_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30040_e29307;
        locals.var_xp_dn0 = assign30040_e29307_d_n0;
        locals.var_xp_dn2 = assign30040_e29307_d_n2;
        locals.var_xp_dn4 = assign30040_e29307_d_n4;
        locals.var_xp_dn5 = assign30040_e29307_d_n5;
        locals.var_xp_dn6 = assign30040_e29307_d_n6;
        locals.var_xp_dn7 = assign30040_e29307_d_n7;
        locals.var_xp_dn8 = assign30040_e29307_d_n8;
        locals.var_xp_dn9 = assign30040_e29307_d_n9;
        locals.var_xp_dn10 = assign30040_e29307_d_n10;
        locals.var_xp_dn11 = assign30040_e29307_d_n11;
        locals.var_xp_dn14 = assign30040_e29307_d_n14;

        let (assign30050_e29317, assign30050_e29317_d_n0, assign30050_e29317_d_n2, assign30050_e29317_d_n4, assign30050_e29317_d_n5, assign30050_e29317_d_n6, assign30050_e29317_d_n7, assign30050_e29317_d_n8, assign30050_e29317_d_n9, assign30050_e29317_d_n10, assign30050_e29317_d_n11, assign30050_e29317_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30050_e29317;
        locals.var_xmp_dn0 = assign30050_e29317_d_n0;
        locals.var_xmp_dn2 = assign30050_e29317_d_n2;
        locals.var_xmp_dn4 = assign30050_e29317_d_n4;
        locals.var_xmp_dn5 = assign30050_e29317_d_n5;
        locals.var_xmp_dn6 = assign30050_e29317_d_n6;
        locals.var_xmp_dn7 = assign30050_e29317_d_n7;
        locals.var_xmp_dn8 = assign30050_e29317_d_n8;
        locals.var_xmp_dn9 = assign30050_e29317_d_n9;
        locals.var_xmp_dn10 = assign30050_e29317_d_n10;
        locals.var_xmp_dn11 = assign30050_e29317_d_n11;
        locals.var_xmp_dn14 = assign30050_e29317_d_n14;

        let (assign30060_e29327,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30060_e29327;

        let (assign30070_e29337,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30070_e29337;

        let (assign30080_e29347, assign30080_e29347_d_n0, assign30080_e29347_d_n2, assign30080_e29347_d_n4, assign30080_e29347_d_n5, assign30080_e29347_d_n6, assign30080_e29347_d_n7, assign30080_e29347_d_n8, assign30080_e29347_d_n9, assign30080_e29347_d_n10, assign30080_e29347_d_n11, assign30080_e29347_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30080_e29347;
        locals.var_arg_dn0 = assign30080_e29347_d_n0;
        locals.var_arg_dn2 = assign30080_e29347_d_n2;
        locals.var_arg_dn4 = assign30080_e29347_d_n4;
        locals.var_arg_dn5 = assign30080_e29347_d_n5;
        locals.var_arg_dn6 = assign30080_e29347_d_n6;
        locals.var_arg_dn7 = assign30080_e29347_d_n7;
        locals.var_arg_dn8 = assign30080_e29347_d_n8;
        locals.var_arg_dn9 = assign30080_e29347_d_n9;
        locals.var_arg_dn10 = assign30080_e29347_d_n10;
        locals.var_arg_dn11 = assign30080_e29347_d_n11;
        locals.var_arg_dn14 = assign30080_e29347_d_n14;

        let (assign30090_e29357, assign30090_e29357_d_n0, assign30090_e29357_d_n2, assign30090_e29357_d_n4, assign30090_e29357_d_n5, assign30090_e29357_d_n6, assign30090_e29357_d_n7, assign30090_e29357_d_n8, assign30090_e29357_d_n9, assign30090_e29357_d_n10, assign30090_e29357_d_n11, assign30090_e29357_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30090_e29357;
        locals.var_dnm_dn0 = assign30090_e29357_d_n0;
        locals.var_dnm_dn2 = assign30090_e29357_d_n2;
        locals.var_dnm_dn4 = assign30090_e29357_d_n4;
        locals.var_dnm_dn5 = assign30090_e29357_d_n5;
        locals.var_dnm_dn6 = assign30090_e29357_d_n6;
        locals.var_dnm_dn7 = assign30090_e29357_d_n7;
        locals.var_dnm_dn8 = assign30090_e29357_d_n8;
        locals.var_dnm_dn9 = assign30090_e29357_d_n9;
        locals.var_dnm_dn10 = assign30090_e29357_d_n10;
        locals.var_dnm_dn11 = assign30090_e29357_d_n11;
        locals.var_dnm_dn14 = assign30090_e29357_d_n14;

        let (assign30100_e29369, assign30100_e29369_d_n0, assign30100_e29369_d_n2, assign30100_e29369_d_n4, assign30100_e29369_d_n5, assign30100_e29369_d_n6, assign30100_e29369_d_n7, assign30100_e29369_d_n8, assign30100_e29369_d_n9, assign30100_e29369_d_n10, assign30100_e29369_d_n11, assign30100_e29369_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30100_e29367: f64 = (locals.var_xp * locals.var_x2);
        (assign30100_e29367, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30100_e29369;
        locals.var_xp_dn0 = assign30100_e29369_d_n0;
        locals.var_xp_dn2 = assign30100_e29369_d_n2;
        locals.var_xp_dn4 = assign30100_e29369_d_n4;
        locals.var_xp_dn5 = assign30100_e29369_d_n5;
        locals.var_xp_dn6 = assign30100_e29369_d_n6;
        locals.var_xp_dn7 = assign30100_e29369_d_n7;
        locals.var_xp_dn8 = assign30100_e29369_d_n8;
        locals.var_xp_dn9 = assign30100_e29369_d_n9;
        locals.var_xp_dn10 = assign30100_e29369_d_n10;
        locals.var_xp_dn11 = assign30100_e29369_d_n11;
        locals.var_xp_dn14 = assign30100_e29369_d_n14;

        let (assign30110_e29381, assign30110_e29381_d_n0, assign30110_e29381_d_n2, assign30110_e29381_d_n4, assign30110_e29381_d_n5, assign30110_e29381_d_n6, assign30110_e29381_d_n7, assign30110_e29381_d_n8, assign30110_e29381_d_n9, assign30110_e29381_d_n10, assign30110_e29381_d_n11, assign30110_e29381_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30110_e29379: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30110_e29379, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30110_e29381;
        locals.var_xmp_dn0 = assign30110_e29381_d_n0;
        locals.var_xmp_dn2 = assign30110_e29381_d_n2;
        locals.var_xmp_dn4 = assign30110_e29381_d_n4;
        locals.var_xmp_dn5 = assign30110_e29381_d_n5;
        locals.var_xmp_dn6 = assign30110_e29381_d_n6;
        locals.var_xmp_dn7 = assign30110_e29381_d_n7;
        locals.var_xmp_dn8 = assign30110_e29381_d_n8;
        locals.var_xmp_dn9 = assign30110_e29381_d_n9;
        locals.var_xmp_dn10 = assign30110_e29381_d_n10;
        locals.var_xmp_dn11 = assign30110_e29381_d_n11;
        locals.var_xmp_dn14 = assign30110_e29381_d_n14;

    }

    pub(super) fn stamp_transient_block_87(
        locals: &mut StampLocals,
    ) {
        let (assign30120_e29393, assign30120_e29393_d_n0, assign30120_e29393_d_n2, assign30120_e29393_d_n4, assign30120_e29393_d_n5, assign30120_e29393_d_n6, assign30120_e29393_d_n7, assign30120_e29393_d_n8, assign30120_e29393_d_n9, assign30120_e29393_d_n10, assign30120_e29393_d_n11, assign30120_e29393_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30120_e29391: f64 = (locals.var_xp * locals.var_x2);
        (assign30120_e29391, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30120_e29393;
        locals.var_xp_dn0 = assign30120_e29393_d_n0;
        locals.var_xp_dn2 = assign30120_e29393_d_n2;
        locals.var_xp_dn4 = assign30120_e29393_d_n4;
        locals.var_xp_dn5 = assign30120_e29393_d_n5;
        locals.var_xp_dn6 = assign30120_e29393_d_n6;
        locals.var_xp_dn7 = assign30120_e29393_d_n7;
        locals.var_xp_dn8 = assign30120_e29393_d_n8;
        locals.var_xp_dn9 = assign30120_e29393_d_n9;
        locals.var_xp_dn10 = assign30120_e29393_d_n10;
        locals.var_xp_dn11 = assign30120_e29393_d_n11;
        locals.var_xp_dn14 = assign30120_e29393_d_n14;

        let (assign30130_e29405, assign30130_e29405_d_n0, assign30130_e29405_d_n2, assign30130_e29405_d_n4, assign30130_e29405_d_n5, assign30130_e29405_d_n6, assign30130_e29405_d_n7, assign30130_e29405_d_n8, assign30130_e29405_d_n9, assign30130_e29405_d_n10, assign30130_e29405_d_n11, assign30130_e29405_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30130_e29403: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30130_e29403, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30130_e29405;
        locals.var_xmp_dn0 = assign30130_e29405_d_n0;
        locals.var_xmp_dn2 = assign30130_e29405_d_n2;
        locals.var_xmp_dn4 = assign30130_e29405_d_n4;
        locals.var_xmp_dn5 = assign30130_e29405_d_n5;
        locals.var_xmp_dn6 = assign30130_e29405_d_n6;
        locals.var_xmp_dn7 = assign30130_e29405_d_n7;
        locals.var_xmp_dn8 = assign30130_e29405_d_n8;
        locals.var_xmp_dn9 = assign30130_e29405_d_n9;
        locals.var_xmp_dn10 = assign30130_e29405_d_n10;
        locals.var_xmp_dn11 = assign30130_e29405_d_n11;
        locals.var_xmp_dn14 = assign30130_e29405_d_n14;

        let (assign30140_e29417, assign30140_e29417_d_n0, assign30140_e29417_d_n2, assign30140_e29417_d_n4, assign30140_e29417_d_n5, assign30140_e29417_d_n6, assign30140_e29417_d_n7, assign30140_e29417_d_n8, assign30140_e29417_d_n9, assign30140_e29417_d_n10, assign30140_e29417_d_n11, assign30140_e29417_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30140_e29415: f64 = (locals.var_xp * locals.var_x2);
        (assign30140_e29415, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30140_e29417;
        locals.var_xp_dn0 = assign30140_e29417_d_n0;
        locals.var_xp_dn2 = assign30140_e29417_d_n2;
        locals.var_xp_dn4 = assign30140_e29417_d_n4;
        locals.var_xp_dn5 = assign30140_e29417_d_n5;
        locals.var_xp_dn6 = assign30140_e29417_d_n6;
        locals.var_xp_dn7 = assign30140_e29417_d_n7;
        locals.var_xp_dn8 = assign30140_e29417_d_n8;
        locals.var_xp_dn9 = assign30140_e29417_d_n9;
        locals.var_xp_dn10 = assign30140_e29417_d_n10;
        locals.var_xp_dn11 = assign30140_e29417_d_n11;
        locals.var_xp_dn14 = assign30140_e29417_d_n14;

        let (assign30150_e29429, assign30150_e29429_d_n0, assign30150_e29429_d_n2, assign30150_e29429_d_n4, assign30150_e29429_d_n5, assign30150_e29429_d_n6, assign30150_e29429_d_n7, assign30150_e29429_d_n8, assign30150_e29429_d_n9, assign30150_e29429_d_n10, assign30150_e29429_d_n11, assign30150_e29429_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30150_e29427: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30150_e29427, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30150_e29429;
        locals.var_xmp_dn0 = assign30150_e29429_d_n0;
        locals.var_xmp_dn2 = assign30150_e29429_d_n2;
        locals.var_xmp_dn4 = assign30150_e29429_d_n4;
        locals.var_xmp_dn5 = assign30150_e29429_d_n5;
        locals.var_xmp_dn6 = assign30150_e29429_d_n6;
        locals.var_xmp_dn7 = assign30150_e29429_d_n7;
        locals.var_xmp_dn8 = assign30150_e29429_d_n8;
        locals.var_xmp_dn9 = assign30150_e29429_d_n9;
        locals.var_xmp_dn10 = assign30150_e29429_d_n10;
        locals.var_xmp_dn11 = assign30150_e29429_d_n11;
        locals.var_xmp_dn14 = assign30150_e29429_d_n14;

        let (assign30160_e29441, assign30160_e29441_d_n0, assign30160_e29441_d_n2, assign30160_e29441_d_n4, assign30160_e29441_d_n5, assign30160_e29441_d_n6, assign30160_e29441_d_n7, assign30160_e29441_d_n8, assign30160_e29441_d_n9, assign30160_e29441_d_n10, assign30160_e29441_d_n11, assign30160_e29441_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30160_e29439: f64 = (locals.var_xp * locals.var_x2);
        (assign30160_e29439, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30160_e29441;
        locals.var_xp_dn0 = assign30160_e29441_d_n0;
        locals.var_xp_dn2 = assign30160_e29441_d_n2;
        locals.var_xp_dn4 = assign30160_e29441_d_n4;
        locals.var_xp_dn5 = assign30160_e29441_d_n5;
        locals.var_xp_dn6 = assign30160_e29441_d_n6;
        locals.var_xp_dn7 = assign30160_e29441_d_n7;
        locals.var_xp_dn8 = assign30160_e29441_d_n8;
        locals.var_xp_dn9 = assign30160_e29441_d_n9;
        locals.var_xp_dn10 = assign30160_e29441_d_n10;
        locals.var_xp_dn11 = assign30160_e29441_d_n11;
        locals.var_xp_dn14 = assign30160_e29441_d_n14;

        let (assign30170_e29453, assign30170_e29453_d_n0, assign30170_e29453_d_n2, assign30170_e29453_d_n4, assign30170_e29453_d_n5, assign30170_e29453_d_n6, assign30170_e29453_d_n7, assign30170_e29453_d_n8, assign30170_e29453_d_n9, assign30170_e29453_d_n10, assign30170_e29453_d_n11, assign30170_e29453_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30170_e29451: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30170_e29451, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30170_e29453;
        locals.var_xmp_dn0 = assign30170_e29453_d_n0;
        locals.var_xmp_dn2 = assign30170_e29453_d_n2;
        locals.var_xmp_dn4 = assign30170_e29453_d_n4;
        locals.var_xmp_dn5 = assign30170_e29453_d_n5;
        locals.var_xmp_dn6 = assign30170_e29453_d_n6;
        locals.var_xmp_dn7 = assign30170_e29453_d_n7;
        locals.var_xmp_dn8 = assign30170_e29453_d_n8;
        locals.var_xmp_dn9 = assign30170_e29453_d_n9;
        locals.var_xmp_dn10 = assign30170_e29453_d_n10;
        locals.var_xmp_dn11 = assign30170_e29453_d_n11;
        locals.var_xmp_dn14 = assign30170_e29453_d_n14;

        let (assign30180_e29465, assign30180_e29465_d_n0, assign30180_e29465_d_n2, assign30180_e29465_d_n4, assign30180_e29465_d_n5, assign30180_e29465_d_n6, assign30180_e29465_d_n7, assign30180_e29465_d_n8, assign30180_e29465_d_n9, assign30180_e29465_d_n10, assign30180_e29465_d_n11, assign30180_e29465_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30180_e29463: f64 = (locals.var_xp + locals.var_xmp);
        (assign30180_e29463, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30180_e29465;
        locals.var_arg_dn0 = assign30180_e29465_d_n0;
        locals.var_arg_dn2 = assign30180_e29465_d_n2;
        locals.var_arg_dn4 = assign30180_e29465_d_n4;
        locals.var_arg_dn5 = assign30180_e29465_d_n5;
        locals.var_arg_dn6 = assign30180_e29465_d_n6;
        locals.var_arg_dn7 = assign30180_e29465_d_n7;
        locals.var_arg_dn8 = assign30180_e29465_d_n8;
        locals.var_arg_dn9 = assign30180_e29465_d_n9;
        locals.var_arg_dn10 = assign30180_e29465_d_n10;
        locals.var_arg_dn11 = assign30180_e29465_d_n11;
        locals.var_arg_dn14 = assign30180_e29465_d_n14;

        let (assign30190_e29475, assign30190_e29475_d_n0, assign30190_e29475_d_n2, assign30190_e29475_d_n4, assign30190_e29475_d_n5, assign30190_e29475_d_n6, assign30190_e29475_d_n7, assign30190_e29475_d_n8, assign30190_e29475_d_n9, assign30190_e29475_d_n10, assign30190_e29475_d_n11, assign30190_e29475_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30190_e29475;
        locals.var_dnm_dn0 = assign30190_e29475_d_n0;
        locals.var_dnm_dn2 = assign30190_e29475_d_n2;
        locals.var_dnm_dn4 = assign30190_e29475_d_n4;
        locals.var_dnm_dn5 = assign30190_e29475_d_n5;
        locals.var_dnm_dn6 = assign30190_e29475_d_n6;
        locals.var_dnm_dn7 = assign30190_e29475_d_n7;
        locals.var_dnm_dn8 = assign30190_e29475_d_n8;
        locals.var_dnm_dn9 = assign30190_e29475_d_n9;
        locals.var_dnm_dn10 = assign30190_e29475_d_n10;
        locals.var_dnm_dn11 = assign30190_e29475_d_n11;
        locals.var_dnm_dn14 = assign30190_e29475_d_n14;

        let assign30200_e29490: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard698 = assign30200_e29490;

        let assign30210_e29493: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard699 = assign30210_e29493;

        let (assign30220_e29507,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) && (locals.var_guard698 != 0.0)) && (locals.var_guard699 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30220_e29507;

        let assign30230_e29510: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard700 = assign30230_e29510;

        let (assign30240_e29527,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) && (locals.var_guard698 != 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard700 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30240_e29527;

        let assign30250_e29530: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard701 = assign30250_e29530;

        let (assign30260_e29550,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) && (locals.var_guard698 != 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard700 == 0.0)) && (locals.var_guard701 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30260_e29550;

        let assign30270_e29553: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard702 = assign30270_e29553;

        let (assign30280_e29576,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) && (locals.var_guard698 != 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard700 == 0.0)) && (locals.var_guard701 == 0.0)) && (locals.var_guard702 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30280_e29576;

        let (assign30290_e29588,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) && (locals.var_guard698 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30290_e29588;

        let mut assign30300_loop_guard: usize = 0;
        while {
            let assign30300_cond_e29601: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) && (locals.var_guard698 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign30300_cond_e29601 != 0.0
        } {
            assign30300_loop_guard += 1;
            assert!(assign30300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign30300_body0_e29614, assign30300_body0_e29614_d_n0, assign30300_body0_e29614_d_n2, assign30300_body0_e29614_d_n4, assign30300_body0_e29614_d_n5, assign30300_body0_e29614_d_n6, assign30300_body0_e29614_d_n7, assign30300_body0_e29614_d_n8, assign30300_body0_e29614_d_n9, assign30300_body0_e29614_d_n10, assign30300_body0_e29614_d_n11, assign30300_body0_e29614_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) && (locals.var_guard698 != 0.0)) {
        let assign30300_body0_e29612: f64 = (locals.var_dnm).sqrt();
        (assign30300_body0_e29612, (locals.var_dnm_dn0 / (2.0 * assign30300_body0_e29612)), (locals.var_dnm_dn2 / (2.0 * assign30300_body0_e29612)), (locals.var_dnm_dn4 / (2.0 * assign30300_body0_e29612)), (locals.var_dnm_dn5 / (2.0 * assign30300_body0_e29612)), (locals.var_dnm_dn6 / (2.0 * assign30300_body0_e29612)), (locals.var_dnm_dn7 / (2.0 * assign30300_body0_e29612)), (locals.var_dnm_dn8 / (2.0 * assign30300_body0_e29612)), (locals.var_dnm_dn9 / (2.0 * assign30300_body0_e29612)), (locals.var_dnm_dn10 / (2.0 * assign30300_body0_e29612)), (locals.var_dnm_dn11 / (2.0 * assign30300_body0_e29612)), (locals.var_dnm_dn14 / (2.0 * assign30300_body0_e29612)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign30300_body0_e29614;
            locals.var_dnm_dn0 = assign30300_body0_e29614_d_n0;
            locals.var_dnm_dn2 = assign30300_body0_e29614_d_n2;
            locals.var_dnm_dn4 = assign30300_body0_e29614_d_n4;
            locals.var_dnm_dn5 = assign30300_body0_e29614_d_n5;
            locals.var_dnm_dn6 = assign30300_body0_e29614_d_n6;
            locals.var_dnm_dn7 = assign30300_body0_e29614_d_n7;
            locals.var_dnm_dn8 = assign30300_body0_e29614_d_n8;
            locals.var_dnm_dn9 = assign30300_body0_e29614_d_n9;
            locals.var_dnm_dn10 = assign30300_body0_e29614_d_n10;
            locals.var_dnm_dn11 = assign30300_body0_e29614_d_n11;
            locals.var_dnm_dn14 = assign30300_body0_e29614_d_n14;
            let (assign30300_body1_e29628,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) && (locals.var_guard698 != 0.0)) {
        let assign30300_body1_e29626: f64 = (locals.var_m0 + 1.0);
        (assign30300_body1_e29626,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign30300_body1_e29628;
        }

        let (assign30310_e29652, assign30310_e29652_d_n0, assign30310_e29652_d_n2, assign30310_e29652_d_n4, assign30310_e29652_d_n5, assign30310_e29652_d_n6, assign30310_e29652_d_n7, assign30310_e29652_d_n8, assign30310_e29652_d_n9, assign30310_e29652_d_n10, assign30310_e29652_d_n11, assign30310_e29652_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) && (locals.var_guard698 == 0.0)) {
        let (assign30310_e29650, assign30310_e29650_d_n0, assign30310_e29650_d_n2, assign30310_e29650_d_n4, assign30310_e29650_d_n5, assign30310_e29650_d_n6, assign30310_e29650_d_n7, assign30310_e29650_d_n8, assign30310_e29650_d_n9, assign30310_e29650_d_n10, assign30310_e29650_d_n11, assign30310_e29650_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign30310_e29647: f64 = (2.0 * 4.0);
                let assign30310_e29648: f64 = (1.0 / assign30310_e29647);
                let assign30310_e29649: f64 = (locals.var_dnm).powf(assign30310_e29648);
                (assign30310_e29649, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn0)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn2)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn4)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn5)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn6)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn7)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn8)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn9)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn10)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn11)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30310_e29648) as f64).is_finite() && ((assign30310_e29648) as f64).fract() == 0.0 { if assign30310_e29648 == 0.0 { 0.0 } else { (assign30310_e29648 * ((locals.var_dnm).powf(assign30310_e29648 - 1.0) * locals.var_dnm_dn14)) } } else { (assign30310_e29649 * (assign30310_e29648 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign30310_e29650, assign30310_e29650_d_n0, assign30310_e29650_d_n2, assign30310_e29650_d_n4, assign30310_e29650_d_n5, assign30310_e29650_d_n6, assign30310_e29650_d_n7, assign30310_e29650_d_n8, assign30310_e29650_d_n9, assign30310_e29650_d_n10, assign30310_e29650_d_n11, assign30310_e29650_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30310_e29652;
        locals.var_dnm_dn0 = assign30310_e29652_d_n0;
        locals.var_dnm_dn2 = assign30310_e29652_d_n2;
        locals.var_dnm_dn4 = assign30310_e29652_d_n4;
        locals.var_dnm_dn5 = assign30310_e29652_d_n5;
        locals.var_dnm_dn6 = assign30310_e29652_d_n6;
        locals.var_dnm_dn7 = assign30310_e29652_d_n7;
        locals.var_dnm_dn8 = assign30310_e29652_d_n8;
        locals.var_dnm_dn9 = assign30310_e29652_d_n9;
        locals.var_dnm_dn10 = assign30310_e29652_d_n10;
        locals.var_dnm_dn11 = assign30310_e29652_d_n11;
        locals.var_dnm_dn14 = assign30310_e29652_d_n14;

        let (assign30320_e29664, assign30320_e29664_d_n0, assign30320_e29664_d_n2, assign30320_e29664_d_n4, assign30320_e29664_d_n5, assign30320_e29664_d_n6, assign30320_e29664_d_n7, assign30320_e29664_d_n8, assign30320_e29664_d_n9, assign30320_e29664_d_n10, assign30320_e29664_d_n11, assign30320_e29664_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30320_e29662: f64 = (1.0 / locals.var_dnm);
        (assign30320_e29662, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30320_e29664;
        locals.var_dnm_dn0 = assign30320_e29664_d_n0;
        locals.var_dnm_dn2 = assign30320_e29664_d_n2;
        locals.var_dnm_dn4 = assign30320_e29664_d_n4;
        locals.var_dnm_dn5 = assign30320_e29664_d_n5;
        locals.var_dnm_dn6 = assign30320_e29664_d_n6;
        locals.var_dnm_dn7 = assign30320_e29664_d_n7;
        locals.var_dnm_dn8 = assign30320_e29664_d_n8;
        locals.var_dnm_dn9 = assign30320_e29664_d_n9;
        locals.var_dnm_dn10 = assign30320_e29664_d_n10;
        locals.var_dnm_dn11 = assign30320_e29664_d_n11;
        locals.var_dnm_dn14 = assign30320_e29664_d_n14;

        let (assign30330_e29678, assign30330_e29678_d_n0, assign30330_e29678_d_n2, assign30330_e29678_d_n4, assign30330_e29678_d_n5, assign30330_e29678_d_n6, assign30330_e29678_d_n7, assign30330_e29678_d_n8, assign30330_e29678_d_n9, assign30330_e29678_d_n10, assign30330_e29678_d_n11, assign30330_e29678_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30330_e29674: f64 = (locals.var_tmf1 * 0.2);
        let assign30330_e29676: f64 = (assign30330_e29674 * locals.var_dnm);
        (assign30330_e29676, (((locals.var_tmf1_dn0 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.2) * locals.var_dnm) + (assign30330_e29674 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign30330_e29678;
        locals.var_tmf0_dn0 = assign30330_e29678_d_n0;
        locals.var_tmf0_dn2 = assign30330_e29678_d_n2;
        locals.var_tmf0_dn4 = assign30330_e29678_d_n4;
        locals.var_tmf0_dn5 = assign30330_e29678_d_n5;
        locals.var_tmf0_dn6 = assign30330_e29678_d_n6;
        locals.var_tmf0_dn7 = assign30330_e29678_d_n7;
        locals.var_tmf0_dn8 = assign30330_e29678_d_n8;
        locals.var_tmf0_dn9 = assign30330_e29678_d_n9;
        locals.var_tmf0_dn10 = assign30330_e29678_d_n10;
        locals.var_tmf0_dn11 = assign30330_e29678_d_n11;
        locals.var_tmf0_dn14 = assign30330_e29678_d_n14;

        let (assign30340_e29694, assign30340_e29694_d_n0, assign30340_e29694_d_n2, assign30340_e29694_d_n4, assign30340_e29694_d_n5, assign30340_e29694_d_n6, assign30340_e29694_d_n7, assign30340_e29694_d_n8, assign30340_e29694_d_n9, assign30340_e29694_d_n10, assign30340_e29694_d_n11, assign30340_e29694_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30340_e29688: f64 = (0.2 * locals.var_xmp);
        let assign30340_e29690: f64 = (assign30340_e29688 * locals.var_dnm);
        let assign30340_e29692: f64 = (assign30340_e29690 / locals.var_arg);
        (assign30340_e29692, ((((((0.2 * locals.var_xmp_dn0) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn0)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn2) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn2)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn4) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn4)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn5) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn5)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn6) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn6)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn7) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn7)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn8) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn8)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn9) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn9)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn10) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn10)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn11) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn11)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn14) * locals.var_dnm) + (assign30340_e29688 * locals.var_dnm_dn14)) * locals.var_arg) - (assign30340_e29690 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30340_e29694;
        locals.var_t0_dn0 = assign30340_e29694_d_n0;
        locals.var_t0_dn2 = assign30340_e29694_d_n2;
        locals.var_t0_dn4 = assign30340_e29694_d_n4;
        locals.var_t0_dn5 = assign30340_e29694_d_n5;
        locals.var_t0_dn6 = assign30340_e29694_d_n6;
        locals.var_t0_dn7 = assign30340_e29694_d_n7;
        locals.var_t0_dn8 = assign30340_e29694_d_n8;
        locals.var_t0_dn9 = assign30340_e29694_d_n9;
        locals.var_t0_dn10 = assign30340_e29694_d_n10;
        locals.var_t0_dn11 = assign30340_e29694_d_n11;
        locals.var_t0_dn14 = assign30340_e29694_d_n14;

        let (assign30350_e29708, assign30350_e29708_d_n0, assign30350_e29708_d_n2, assign30350_e29708_d_n4, assign30350_e29708_d_n5, assign30350_e29708_d_n6, assign30350_e29708_d_n7, assign30350_e29708_d_n8, assign30350_e29708_d_n9, assign30350_e29708_d_n10, assign30350_e29708_d_n11, assign30350_e29708_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign30350_e29704: f64 = (0.3 + 0.2);
        let assign30350_e29706: f64 = (assign30350_e29704 - locals.var_tmf0);
        (assign30350_e29706, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign30350_e29708;
        locals.var_t10_dn0 = assign30350_e29708_d_n0;
        locals.var_t10_dn2 = assign30350_e29708_d_n2;
        locals.var_t10_dn4 = assign30350_e29708_d_n4;
        locals.var_t10_dn5 = assign30350_e29708_d_n5;
        locals.var_t10_dn6 = assign30350_e29708_d_n6;
        locals.var_t10_dn7 = assign30350_e29708_d_n7;
        locals.var_t10_dn8 = assign30350_e29708_d_n8;
        locals.var_t10_dn9 = assign30350_e29708_d_n9;
        locals.var_t10_dn10 = assign30350_e29708_d_n10;
        locals.var_t10_dn11 = assign30350_e29708_d_n11;
        locals.var_t10_dn14 = assign30350_e29708_d_n14;

        let (assign30360_e29718, assign30360_e29718_d_n0, assign30360_e29718_d_n2, assign30360_e29718_d_n4, assign30360_e29718_d_n5, assign30360_e29718_d_n6, assign30360_e29718_d_n7, assign30360_e29718_d_n8, assign30360_e29718_d_n9, assign30360_e29718_d_n10, assign30360_e29718_d_n11, assign30360_e29718_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30360_e29718;
        locals.var_t0_dn0 = assign30360_e29718_d_n0;
        locals.var_t0_dn2 = assign30360_e29718_d_n2;
        locals.var_t0_dn4 = assign30360_e29718_d_n4;
        locals.var_t0_dn5 = assign30360_e29718_d_n5;
        locals.var_t0_dn6 = assign30360_e29718_d_n6;
        locals.var_t0_dn7 = assign30360_e29718_d_n7;
        locals.var_t0_dn8 = assign30360_e29718_d_n8;
        locals.var_t0_dn9 = assign30360_e29718_d_n9;
        locals.var_t0_dn10 = assign30360_e29718_d_n10;
        locals.var_t0_dn11 = assign30360_e29718_d_n11;
        locals.var_t0_dn14 = assign30360_e29718_d_n14;

        let (assign30370_e29729, assign30370_e29729_d_n0, assign30370_e29729_d_n2, assign30370_e29729_d_n4, assign30370_e29729_d_n5, assign30370_e29729_d_n6, assign30370_e29729_d_n7, assign30370_e29729_d_n8, assign30370_e29729_d_n9, assign30370_e29729_d_n10, assign30370_e29729_d_n11, assign30370_e29729_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign30370_e29729;
        locals.var_t10_dn0 = assign30370_e29729_d_n0;
        locals.var_t10_dn2 = assign30370_e29729_d_n2;
        locals.var_t10_dn4 = assign30370_e29729_d_n4;
        locals.var_t10_dn5 = assign30370_e29729_d_n5;
        locals.var_t10_dn6 = assign30370_e29729_d_n6;
        locals.var_t10_dn7 = assign30370_e29729_d_n7;
        locals.var_t10_dn8 = assign30370_e29729_d_n8;
        locals.var_t10_dn9 = assign30370_e29729_d_n9;
        locals.var_t10_dn10 = assign30370_e29729_d_n10;
        locals.var_t10_dn11 = assign30370_e29729_d_n11;
        locals.var_t10_dn14 = assign30370_e29729_d_n14;

        let (assign30380_e29740, assign30380_e29740_d_n0, assign30380_e29740_d_n2, assign30380_e29740_d_n4, assign30380_e29740_d_n5, assign30380_e29740_d_n6, assign30380_e29740_d_n7, assign30380_e29740_d_n8, assign30380_e29740_d_n9, assign30380_e29740_d_n10, assign30380_e29740_d_n11, assign30380_e29740_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard697 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30380_e29740;
        locals.var_t0_dn0 = assign30380_e29740_d_n0;
        locals.var_t0_dn2 = assign30380_e29740_d_n2;
        locals.var_t0_dn4 = assign30380_e29740_d_n4;
        locals.var_t0_dn5 = assign30380_e29740_d_n5;
        locals.var_t0_dn6 = assign30380_e29740_d_n6;
        locals.var_t0_dn7 = assign30380_e29740_d_n7;
        locals.var_t0_dn8 = assign30380_e29740_d_n8;
        locals.var_t0_dn9 = assign30380_e29740_d_n9;
        locals.var_t0_dn10 = assign30380_e29740_d_n10;
        locals.var_t0_dn11 = assign30380_e29740_d_n11;
        locals.var_t0_dn14 = assign30380_e29740_d_n14;

        let (assign30390_e29752, assign30390_e29752_d_n0, assign30390_e29752_d_n2, assign30390_e29752_d_n4, assign30390_e29752_d_n5, assign30390_e29752_d_n6, assign30390_e29752_d_n7, assign30390_e29752_d_n8, assign30390_e29752_d_n9, assign30390_e29752_d_n10, assign30390_e29752_d_n11, assign30390_e29752_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign30390_e29749: f64 = (10.0 * 2.220446049250313e-16);
        let assign30390_e29750: f64 = (locals.var_t10 + assign30390_e29749);
        (assign30390_e29750, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign30390_e29752;
        locals.var_t10_dn0 = assign30390_e29752_d_n0;
        locals.var_t10_dn2 = assign30390_e29752_d_n2;
        locals.var_t10_dn4 = assign30390_e29752_d_n4;
        locals.var_t10_dn5 = assign30390_e29752_d_n5;
        locals.var_t10_dn6 = assign30390_e29752_d_n6;
        locals.var_t10_dn7 = assign30390_e29752_d_n7;
        locals.var_t10_dn8 = assign30390_e29752_d_n8;
        locals.var_t10_dn9 = assign30390_e29752_d_n9;
        locals.var_t10_dn10 = assign30390_e29752_d_n10;
        locals.var_t10_dn11 = assign30390_e29752_d_n11;
        locals.var_t10_dn14 = assign30390_e29752_d_n14;

        let (assign30400_e29762, assign30400_e29762_d_n0, assign30400_e29762_d_n2, assign30400_e29762_d_n4, assign30400_e29762_d_n5, assign30400_e29762_d_n6, assign30400_e29762_d_n7, assign30400_e29762_d_n8, assign30400_e29762_d_n9, assign30400_e29762_d_n10, assign30400_e29762_d_n11, assign30400_e29762_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign30400_e29760: f64 = (locals.var_vds / locals.var_t10);
        (assign30400_e29760, (((locals.var_vds_dn0 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn2 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn4 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn5 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn6 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn7 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn8 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn9 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn10 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn11 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn14 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign30400_e29762;
        locals.var_t1_dn0 = assign30400_e29762_d_n0;
        locals.var_t1_dn2 = assign30400_e29762_d_n2;
        locals.var_t1_dn4 = assign30400_e29762_d_n4;
        locals.var_t1_dn5 = assign30400_e29762_d_n5;
        locals.var_t1_dn6 = assign30400_e29762_d_n6;
        locals.var_t1_dn7 = assign30400_e29762_d_n7;
        locals.var_t1_dn8 = assign30400_e29762_d_n8;
        locals.var_t1_dn9 = assign30400_e29762_d_n9;
        locals.var_t1_dn10 = assign30400_e29762_d_n10;
        locals.var_t1_dn11 = assign30400_e29762_d_n11;
        locals.var_t1_dn14 = assign30400_e29762_d_n14;

    }

    pub(super) fn stamp_transient_block_88(
        locals: &mut StampLocals,
    ) {
        let (assign30410_e29779, assign30410_e29779_d_n0, assign30410_e29779_d_n2, assign30410_e29779_d_n4, assign30410_e29779_d_n5, assign30410_e29779_d_n6, assign30410_e29779_d_n7, assign30410_e29779_d_n8, assign30410_e29779_d_n9, assign30410_e29779_d_n10, assign30410_e29779_d_n11, assign30410_e29779_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let (assign30410_e29777, assign30410_e29777_d_n0, assign30410_e29777_d_n2, assign30410_e29777_d_n4, assign30410_e29777_d_n5, assign30410_e29777_d_n6, assign30410_e29777_d_n7, assign30410_e29777_d_n8, assign30410_e29777_d_n9, assign30410_e29777_d_n10, assign30410_e29777_d_n11, assign30410_e29777_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign30410_e29775: f64 = (locals.var_ddlte - 1.0);
                let assign30410_e29776: f64 = (locals.var_t1).powf(assign30410_e29775);
                (assign30410_e29776, if locals.var_ddlte_dn0 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn0)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn0 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_ddlte_dn2 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn2)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn2 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_ddlte_dn4 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn4)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn4 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_ddlte_dn5 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn5)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn5 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_ddlte_dn6 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn6)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn6 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_ddlte_dn7 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn7)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn7 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_ddlte_dn8 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn8)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn8 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_ddlte_dn9 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn9)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn9 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_ddlte_dn10 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn10)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn10 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_ddlte_dn11 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn11)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn11 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_ddlte_dn14 == 0.0 && ((assign30410_e29775) as f64).is_finite() && ((assign30410_e29775) as f64).fract() == 0.0 { if assign30410_e29775 == 0.0 { 0.0 } else { (assign30410_e29775 * ((locals.var_t1).powf(assign30410_e29775 - 1.0) * locals.var_t1_dn14)) } } else { (assign30410_e29776 * ((locals.var_ddlte_dn14 * (locals.var_t1).ln()) + (assign30410_e29775 * (locals.var_t1_dn14 / locals.var_t1)))) },)
            }
        };
        (assign30410_e29777, assign30410_e29777_d_n0, assign30410_e29777_d_n2, assign30410_e29777_d_n4, assign30410_e29777_d_n5, assign30410_e29777_d_n6, assign30410_e29777_d_n7, assign30410_e29777_d_n8, assign30410_e29777_d_n9, assign30410_e29777_d_n10, assign30410_e29777_d_n11, assign30410_e29777_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign30410_e29779;
        locals.var_t2_dn0 = assign30410_e29779_d_n0;
        locals.var_t2_dn2 = assign30410_e29779_d_n2;
        locals.var_t2_dn4 = assign30410_e29779_d_n4;
        locals.var_t2_dn5 = assign30410_e29779_d_n5;
        locals.var_t2_dn6 = assign30410_e29779_d_n6;
        locals.var_t2_dn7 = assign30410_e29779_d_n7;
        locals.var_t2_dn8 = assign30410_e29779_d_n8;
        locals.var_t2_dn9 = assign30410_e29779_d_n9;
        locals.var_t2_dn10 = assign30410_e29779_d_n10;
        locals.var_t2_dn11 = assign30410_e29779_d_n11;
        locals.var_t2_dn14 = assign30410_e29779_d_n14;

        let (assign30420_e29789, assign30420_e29789_d_n0, assign30420_e29789_d_n2, assign30420_e29789_d_n4, assign30420_e29789_d_n5, assign30420_e29789_d_n6, assign30420_e29789_d_n7, assign30420_e29789_d_n8, assign30420_e29789_d_n9, assign30420_e29789_d_n10, assign30420_e29789_d_n11, assign30420_e29789_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign30420_e29787: f64 = (locals.var_t2 * locals.var_t1);
        (assign30420_e29787, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign30420_e29789;
        locals.var_t7_dn0 = assign30420_e29789_d_n0;
        locals.var_t7_dn2 = assign30420_e29789_d_n2;
        locals.var_t7_dn4 = assign30420_e29789_d_n4;
        locals.var_t7_dn5 = assign30420_e29789_d_n5;
        locals.var_t7_dn6 = assign30420_e29789_d_n6;
        locals.var_t7_dn7 = assign30420_e29789_d_n7;
        locals.var_t7_dn8 = assign30420_e29789_d_n8;
        locals.var_t7_dn9 = assign30420_e29789_d_n9;
        locals.var_t7_dn10 = assign30420_e29789_d_n10;
        locals.var_t7_dn11 = assign30420_e29789_d_n11;
        locals.var_t7_dn14 = assign30420_e29789_d_n14;

        let (assign30430_e29799, assign30430_e29799_d_n0, assign30430_e29799_d_n2, assign30430_e29799_d_n4, assign30430_e29799_d_n5, assign30430_e29799_d_n6, assign30430_e29799_d_n7, assign30430_e29799_d_n8, assign30430_e29799_d_n9, assign30430_e29799_d_n10, assign30430_e29799_d_n11, assign30430_e29799_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign30430_e29797: f64 = (1.0 + locals.var_t7);
        (assign30430_e29797, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign30430_e29799;
        locals.var_t3_dn0 = assign30430_e29799_d_n0;
        locals.var_t3_dn2 = assign30430_e29799_d_n2;
        locals.var_t3_dn4 = assign30430_e29799_d_n4;
        locals.var_t3_dn5 = assign30430_e29799_d_n5;
        locals.var_t3_dn6 = assign30430_e29799_d_n6;
        locals.var_t3_dn7 = assign30430_e29799_d_n7;
        locals.var_t3_dn8 = assign30430_e29799_d_n8;
        locals.var_t3_dn9 = assign30430_e29799_d_n9;
        locals.var_t3_dn10 = assign30430_e29799_d_n10;
        locals.var_t3_dn11 = assign30430_e29799_d_n11;
        locals.var_t3_dn14 = assign30430_e29799_d_n14;

        let (assign30440_e29818, assign30440_e29818_d_n0, assign30440_e29818_d_n2, assign30440_e29818_d_n4, assign30440_e29818_d_n5, assign30440_e29818_d_n6, assign30440_e29818_d_n7, assign30440_e29818_d_n8, assign30440_e29818_d_n9, assign30440_e29818_d_n10, assign30440_e29818_d_n11, assign30440_e29818_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let (assign30440_e29816, assign30440_e29816_d_n0, assign30440_e29816_d_n2, assign30440_e29816_d_n4, assign30440_e29816_d_n5, assign30440_e29816_d_n6, assign30440_e29816_d_n7, assign30440_e29816_d_n8, assign30440_e29816_d_n9, assign30440_e29816_d_n10, assign30440_e29816_d_n11, assign30440_e29816_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign30440_e29812: f64 = (1.0 / locals.var_ddlte);
                let assign30440_e29814: f64 = (assign30440_e29812 - 1.0);
                let assign30440_e29815: f64 = (locals.var_t3).powf(assign30440_e29814);
                (assign30440_e29815, if (-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn0)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn0 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn2)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn2 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn4)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn4 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn5)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn5 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn6)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn6 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn7)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn7 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn8)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn8 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn9)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn9 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn10)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn10 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn11)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn11 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30440_e29814) as f64).is_finite() && ((assign30440_e29814) as f64).fract() == 0.0 { if assign30440_e29814 == 0.0 { 0.0 } else { (assign30440_e29814 * ((locals.var_t3).powf(assign30440_e29814 - 1.0) * locals.var_t3_dn14)) } } else { (assign30440_e29815 * (((-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30440_e29814 * (locals.var_t3_dn14 / locals.var_t3)))) },)
            }
        };
        (assign30440_e29816, assign30440_e29816_d_n0, assign30440_e29816_d_n2, assign30440_e29816_d_n4, assign30440_e29816_d_n5, assign30440_e29816_d_n6, assign30440_e29816_d_n7, assign30440_e29816_d_n8, assign30440_e29816_d_n9, assign30440_e29816_d_n10, assign30440_e29816_d_n11, assign30440_e29816_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign30440_e29818;
        locals.var_t4_dn0 = assign30440_e29818_d_n0;
        locals.var_t4_dn2 = assign30440_e29818_d_n2;
        locals.var_t4_dn4 = assign30440_e29818_d_n4;
        locals.var_t4_dn5 = assign30440_e29818_d_n5;
        locals.var_t4_dn6 = assign30440_e29818_d_n6;
        locals.var_t4_dn7 = assign30440_e29818_d_n7;
        locals.var_t4_dn8 = assign30440_e29818_d_n8;
        locals.var_t4_dn9 = assign30440_e29818_d_n9;
        locals.var_t4_dn10 = assign30440_e29818_d_n10;
        locals.var_t4_dn11 = assign30440_e29818_d_n11;
        locals.var_t4_dn14 = assign30440_e29818_d_n14;

        let (assign30450_e29828, assign30450_e29828_d_n0, assign30450_e29828_d_n2, assign30450_e29828_d_n4, assign30450_e29828_d_n5, assign30450_e29828_d_n6, assign30450_e29828_d_n7, assign30450_e29828_d_n8, assign30450_e29828_d_n9, assign30450_e29828_d_n10, assign30450_e29828_d_n11, assign30450_e29828_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign30450_e29826: f64 = (locals.var_t4 * locals.var_t3);
        (assign30450_e29826, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign30450_e29828;
        locals.var_t6_dn0 = assign30450_e29828_d_n0;
        locals.var_t6_dn2 = assign30450_e29828_d_n2;
        locals.var_t6_dn4 = assign30450_e29828_d_n4;
        locals.var_t6_dn5 = assign30450_e29828_d_n5;
        locals.var_t6_dn6 = assign30450_e29828_d_n6;
        locals.var_t6_dn7 = assign30450_e29828_d_n7;
        locals.var_t6_dn8 = assign30450_e29828_d_n8;
        locals.var_t6_dn9 = assign30450_e29828_d_n9;
        locals.var_t6_dn10 = assign30450_e29828_d_n10;
        locals.var_t6_dn11 = assign30450_e29828_d_n11;
        locals.var_t6_dn14 = assign30450_e29828_d_n14;

        let (assign30460_e29838, assign30460_e29838_d_n0, assign30460_e29838_d_n2, assign30460_e29838_d_n4, assign30460_e29838_d_n5, assign30460_e29838_d_n6, assign30460_e29838_d_n7, assign30460_e29838_d_n8, assign30460_e29838_d_n9, assign30460_e29838_d_n10, assign30460_e29838_d_n11, assign30460_e29838_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign30460_e29836: f64 = (locals.var_vds / locals.var_t6);
        (assign30460_e29836, (((locals.var_vds_dn0 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn2 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn4 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn5 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn6 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn7 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn8 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn9 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn10 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn11 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn14 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn14,)
    }
};
        locals.var_vdseff = assign30460_e29838;
        locals.var_vdseff_dn0 = assign30460_e29838_d_n0;
        locals.var_vdseff_dn2 = assign30460_e29838_d_n2;
        locals.var_vdseff_dn4 = assign30460_e29838_d_n4;
        locals.var_vdseff_dn5 = assign30460_e29838_d_n5;
        locals.var_vdseff_dn6 = assign30460_e29838_d_n6;
        locals.var_vdseff_dn7 = assign30460_e29838_d_n7;
        locals.var_vdseff_dn8 = assign30460_e29838_d_n8;
        locals.var_vdseff_dn9 = assign30460_e29838_d_n9;
        locals.var_vdseff_dn10 = assign30460_e29838_d_n10;
        locals.var_vdseff_dn11 = assign30460_e29838_d_n11;
        locals.var_vdseff_dn14 = assign30460_e29838_d_n14;

        let assign30470_e29842: f64 = 0.5;
        let assign30470_e29847: f64 = if ((locals.var_vgp < assign30470_e29842) && (0.5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard703 = assign30470_e29847;

        let (assign30480_e29861, assign30480_e29861_d_n0, assign30480_e29861_d_n2, assign30480_e29861_d_n4, assign30480_e29861_d_n5, assign30480_e29861_d_n6, assign30480_e29861_d_n7, assign30480_e29861_d_n8, assign30480_e29861_d_n9, assign30480_e29861_d_n10, assign30480_e29861_d_n11, assign30480_e29861_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30480_e29857: f64 = 0.5;
        let assign30480_e29859: f64 = (assign30480_e29857 - locals.var_vgp);
        (assign30480_e29859, (-locals.var_vgp_dn0), (-locals.var_vgp_dn2), (-locals.var_vgp_dn4), (-locals.var_vgp_dn5), (-locals.var_vgp_dn6), (-locals.var_vgp_dn7), (-locals.var_vgp_dn8), (-locals.var_vgp_dn9), (-locals.var_vgp_dn10), (-locals.var_vgp_dn11), (-locals.var_vgp_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign30480_e29861;
        locals.var_tmf1_dn0 = assign30480_e29861_d_n0;
        locals.var_tmf1_dn2 = assign30480_e29861_d_n2;
        locals.var_tmf1_dn4 = assign30480_e29861_d_n4;
        locals.var_tmf1_dn5 = assign30480_e29861_d_n5;
        locals.var_tmf1_dn6 = assign30480_e29861_d_n6;
        locals.var_tmf1_dn7 = assign30480_e29861_d_n7;
        locals.var_tmf1_dn8 = assign30480_e29861_d_n8;
        locals.var_tmf1_dn9 = assign30480_e29861_d_n9;
        locals.var_tmf1_dn10 = assign30480_e29861_d_n10;
        locals.var_tmf1_dn11 = assign30480_e29861_d_n11;
        locals.var_tmf1_dn14 = assign30480_e29861_d_n14;

        let (assign30490_e29873, assign30490_e29873_d_n0, assign30490_e29873_d_n2, assign30490_e29873_d_n4, assign30490_e29873_d_n5, assign30490_e29873_d_n6, assign30490_e29873_d_n7, assign30490_e29873_d_n8, assign30490_e29873_d_n9, assign30490_e29873_d_n10, assign30490_e29873_d_n11, assign30490_e29873_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30490_e29871: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign30490_e29871, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign30490_e29873;
        locals.var_x2_dn0 = assign30490_e29873_d_n0;
        locals.var_x2_dn2 = assign30490_e29873_d_n2;
        locals.var_x2_dn4 = assign30490_e29873_d_n4;
        locals.var_x2_dn5 = assign30490_e29873_d_n5;
        locals.var_x2_dn6 = assign30490_e29873_d_n6;
        locals.var_x2_dn7 = assign30490_e29873_d_n7;
        locals.var_x2_dn8 = assign30490_e29873_d_n8;
        locals.var_x2_dn9 = assign30490_e29873_d_n9;
        locals.var_x2_dn10 = assign30490_e29873_d_n10;
        locals.var_x2_dn11 = assign30490_e29873_d_n11;
        locals.var_x2_dn14 = assign30490_e29873_d_n14;

        let (assign30500_e29885, assign30500_e29885_d_n0, assign30500_e29885_d_n2, assign30500_e29885_d_n4, assign30500_e29885_d_n5, assign30500_e29885_d_n6, assign30500_e29885_d_n7, assign30500_e29885_d_n8, assign30500_e29885_d_n9, assign30500_e29885_d_n10, assign30500_e29885_d_n11, assign30500_e29885_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30500_e29883: f64 = (0.5 * 0.5);
        (assign30500_e29883, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign30500_e29885;
        locals.var_xmax2_dn0 = assign30500_e29885_d_n0;
        locals.var_xmax2_dn2 = assign30500_e29885_d_n2;
        locals.var_xmax2_dn4 = assign30500_e29885_d_n4;
        locals.var_xmax2_dn5 = assign30500_e29885_d_n5;
        locals.var_xmax2_dn6 = assign30500_e29885_d_n6;
        locals.var_xmax2_dn7 = assign30500_e29885_d_n7;
        locals.var_xmax2_dn8 = assign30500_e29885_d_n8;
        locals.var_xmax2_dn9 = assign30500_e29885_d_n9;
        locals.var_xmax2_dn10 = assign30500_e29885_d_n10;
        locals.var_xmax2_dn11 = assign30500_e29885_d_n11;
        locals.var_xmax2_dn14 = assign30500_e29885_d_n14;

        let (assign30510_e29895, assign30510_e29895_d_n0, assign30510_e29895_d_n2, assign30510_e29895_d_n4, assign30510_e29895_d_n5, assign30510_e29895_d_n6, assign30510_e29895_d_n7, assign30510_e29895_d_n8, assign30510_e29895_d_n9, assign30510_e29895_d_n10, assign30510_e29895_d_n11, assign30510_e29895_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30510_e29895;
        locals.var_xp_dn0 = assign30510_e29895_d_n0;
        locals.var_xp_dn2 = assign30510_e29895_d_n2;
        locals.var_xp_dn4 = assign30510_e29895_d_n4;
        locals.var_xp_dn5 = assign30510_e29895_d_n5;
        locals.var_xp_dn6 = assign30510_e29895_d_n6;
        locals.var_xp_dn7 = assign30510_e29895_d_n7;
        locals.var_xp_dn8 = assign30510_e29895_d_n8;
        locals.var_xp_dn9 = assign30510_e29895_d_n9;
        locals.var_xp_dn10 = assign30510_e29895_d_n10;
        locals.var_xp_dn11 = assign30510_e29895_d_n11;
        locals.var_xp_dn14 = assign30510_e29895_d_n14;

        let (assign30520_e29905, assign30520_e29905_d_n0, assign30520_e29905_d_n2, assign30520_e29905_d_n4, assign30520_e29905_d_n5, assign30520_e29905_d_n6, assign30520_e29905_d_n7, assign30520_e29905_d_n8, assign30520_e29905_d_n9, assign30520_e29905_d_n10, assign30520_e29905_d_n11, assign30520_e29905_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30520_e29905;
        locals.var_xmp_dn0 = assign30520_e29905_d_n0;
        locals.var_xmp_dn2 = assign30520_e29905_d_n2;
        locals.var_xmp_dn4 = assign30520_e29905_d_n4;
        locals.var_xmp_dn5 = assign30520_e29905_d_n5;
        locals.var_xmp_dn6 = assign30520_e29905_d_n6;
        locals.var_xmp_dn7 = assign30520_e29905_d_n7;
        locals.var_xmp_dn8 = assign30520_e29905_d_n8;
        locals.var_xmp_dn9 = assign30520_e29905_d_n9;
        locals.var_xmp_dn10 = assign30520_e29905_d_n10;
        locals.var_xmp_dn11 = assign30520_e29905_d_n11;
        locals.var_xmp_dn14 = assign30520_e29905_d_n14;

        let (assign30530_e29915,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30530_e29915;

        let (assign30540_e29925,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30540_e29925;

        let (assign30550_e29935, assign30550_e29935_d_n0, assign30550_e29935_d_n2, assign30550_e29935_d_n4, assign30550_e29935_d_n5, assign30550_e29935_d_n6, assign30550_e29935_d_n7, assign30550_e29935_d_n8, assign30550_e29935_d_n9, assign30550_e29935_d_n10, assign30550_e29935_d_n11, assign30550_e29935_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30550_e29935;
        locals.var_arg_dn0 = assign30550_e29935_d_n0;
        locals.var_arg_dn2 = assign30550_e29935_d_n2;
        locals.var_arg_dn4 = assign30550_e29935_d_n4;
        locals.var_arg_dn5 = assign30550_e29935_d_n5;
        locals.var_arg_dn6 = assign30550_e29935_d_n6;
        locals.var_arg_dn7 = assign30550_e29935_d_n7;
        locals.var_arg_dn8 = assign30550_e29935_d_n8;
        locals.var_arg_dn9 = assign30550_e29935_d_n9;
        locals.var_arg_dn10 = assign30550_e29935_d_n10;
        locals.var_arg_dn11 = assign30550_e29935_d_n11;
        locals.var_arg_dn14 = assign30550_e29935_d_n14;

        let (assign30560_e29945, assign30560_e29945_d_n0, assign30560_e29945_d_n2, assign30560_e29945_d_n4, assign30560_e29945_d_n5, assign30560_e29945_d_n6, assign30560_e29945_d_n7, assign30560_e29945_d_n8, assign30560_e29945_d_n9, assign30560_e29945_d_n10, assign30560_e29945_d_n11, assign30560_e29945_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30560_e29945;
        locals.var_dnm_dn0 = assign30560_e29945_d_n0;
        locals.var_dnm_dn2 = assign30560_e29945_d_n2;
        locals.var_dnm_dn4 = assign30560_e29945_d_n4;
        locals.var_dnm_dn5 = assign30560_e29945_d_n5;
        locals.var_dnm_dn6 = assign30560_e29945_d_n6;
        locals.var_dnm_dn7 = assign30560_e29945_d_n7;
        locals.var_dnm_dn8 = assign30560_e29945_d_n8;
        locals.var_dnm_dn9 = assign30560_e29945_d_n9;
        locals.var_dnm_dn10 = assign30560_e29945_d_n10;
        locals.var_dnm_dn11 = assign30560_e29945_d_n11;
        locals.var_dnm_dn14 = assign30560_e29945_d_n14;

        let (assign30570_e29957, assign30570_e29957_d_n0, assign30570_e29957_d_n2, assign30570_e29957_d_n4, assign30570_e29957_d_n5, assign30570_e29957_d_n6, assign30570_e29957_d_n7, assign30570_e29957_d_n8, assign30570_e29957_d_n9, assign30570_e29957_d_n10, assign30570_e29957_d_n11, assign30570_e29957_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30570_e29955: f64 = (locals.var_xp * locals.var_x2);
        (assign30570_e29955, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30570_e29957;
        locals.var_xp_dn0 = assign30570_e29957_d_n0;
        locals.var_xp_dn2 = assign30570_e29957_d_n2;
        locals.var_xp_dn4 = assign30570_e29957_d_n4;
        locals.var_xp_dn5 = assign30570_e29957_d_n5;
        locals.var_xp_dn6 = assign30570_e29957_d_n6;
        locals.var_xp_dn7 = assign30570_e29957_d_n7;
        locals.var_xp_dn8 = assign30570_e29957_d_n8;
        locals.var_xp_dn9 = assign30570_e29957_d_n9;
        locals.var_xp_dn10 = assign30570_e29957_d_n10;
        locals.var_xp_dn11 = assign30570_e29957_d_n11;
        locals.var_xp_dn14 = assign30570_e29957_d_n14;

        let (assign30580_e29969, assign30580_e29969_d_n0, assign30580_e29969_d_n2, assign30580_e29969_d_n4, assign30580_e29969_d_n5, assign30580_e29969_d_n6, assign30580_e29969_d_n7, assign30580_e29969_d_n8, assign30580_e29969_d_n9, assign30580_e29969_d_n10, assign30580_e29969_d_n11, assign30580_e29969_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30580_e29967: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30580_e29967, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30580_e29969;
        locals.var_xmp_dn0 = assign30580_e29969_d_n0;
        locals.var_xmp_dn2 = assign30580_e29969_d_n2;
        locals.var_xmp_dn4 = assign30580_e29969_d_n4;
        locals.var_xmp_dn5 = assign30580_e29969_d_n5;
        locals.var_xmp_dn6 = assign30580_e29969_d_n6;
        locals.var_xmp_dn7 = assign30580_e29969_d_n7;
        locals.var_xmp_dn8 = assign30580_e29969_d_n8;
        locals.var_xmp_dn9 = assign30580_e29969_d_n9;
        locals.var_xmp_dn10 = assign30580_e29969_d_n10;
        locals.var_xmp_dn11 = assign30580_e29969_d_n11;
        locals.var_xmp_dn14 = assign30580_e29969_d_n14;

        let (assign30590_e29981, assign30590_e29981_d_n0, assign30590_e29981_d_n2, assign30590_e29981_d_n4, assign30590_e29981_d_n5, assign30590_e29981_d_n6, assign30590_e29981_d_n7, assign30590_e29981_d_n8, assign30590_e29981_d_n9, assign30590_e29981_d_n10, assign30590_e29981_d_n11, assign30590_e29981_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30590_e29979: f64 = (locals.var_xp * locals.var_x2);
        (assign30590_e29979, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30590_e29981;
        locals.var_xp_dn0 = assign30590_e29981_d_n0;
        locals.var_xp_dn2 = assign30590_e29981_d_n2;
        locals.var_xp_dn4 = assign30590_e29981_d_n4;
        locals.var_xp_dn5 = assign30590_e29981_d_n5;
        locals.var_xp_dn6 = assign30590_e29981_d_n6;
        locals.var_xp_dn7 = assign30590_e29981_d_n7;
        locals.var_xp_dn8 = assign30590_e29981_d_n8;
        locals.var_xp_dn9 = assign30590_e29981_d_n9;
        locals.var_xp_dn10 = assign30590_e29981_d_n10;
        locals.var_xp_dn11 = assign30590_e29981_d_n11;
        locals.var_xp_dn14 = assign30590_e29981_d_n14;

        let (assign30600_e29993, assign30600_e29993_d_n0, assign30600_e29993_d_n2, assign30600_e29993_d_n4, assign30600_e29993_d_n5, assign30600_e29993_d_n6, assign30600_e29993_d_n7, assign30600_e29993_d_n8, assign30600_e29993_d_n9, assign30600_e29993_d_n10, assign30600_e29993_d_n11, assign30600_e29993_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30600_e29991: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30600_e29991, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30600_e29993;
        locals.var_xmp_dn0 = assign30600_e29993_d_n0;
        locals.var_xmp_dn2 = assign30600_e29993_d_n2;
        locals.var_xmp_dn4 = assign30600_e29993_d_n4;
        locals.var_xmp_dn5 = assign30600_e29993_d_n5;
        locals.var_xmp_dn6 = assign30600_e29993_d_n6;
        locals.var_xmp_dn7 = assign30600_e29993_d_n7;
        locals.var_xmp_dn8 = assign30600_e29993_d_n8;
        locals.var_xmp_dn9 = assign30600_e29993_d_n9;
        locals.var_xmp_dn10 = assign30600_e29993_d_n10;
        locals.var_xmp_dn11 = assign30600_e29993_d_n11;
        locals.var_xmp_dn14 = assign30600_e29993_d_n14;

        let (assign30610_e30005, assign30610_e30005_d_n0, assign30610_e30005_d_n2, assign30610_e30005_d_n4, assign30610_e30005_d_n5, assign30610_e30005_d_n6, assign30610_e30005_d_n7, assign30610_e30005_d_n8, assign30610_e30005_d_n9, assign30610_e30005_d_n10, assign30610_e30005_d_n11, assign30610_e30005_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30610_e30003: f64 = (locals.var_xp + locals.var_xmp);
        (assign30610_e30003, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30610_e30005;
        locals.var_arg_dn0 = assign30610_e30005_d_n0;
        locals.var_arg_dn2 = assign30610_e30005_d_n2;
        locals.var_arg_dn4 = assign30610_e30005_d_n4;
        locals.var_arg_dn5 = assign30610_e30005_d_n5;
        locals.var_arg_dn6 = assign30610_e30005_d_n6;
        locals.var_arg_dn7 = assign30610_e30005_d_n7;
        locals.var_arg_dn8 = assign30610_e30005_d_n8;
        locals.var_arg_dn9 = assign30610_e30005_d_n9;
        locals.var_arg_dn10 = assign30610_e30005_d_n10;
        locals.var_arg_dn11 = assign30610_e30005_d_n11;
        locals.var_arg_dn14 = assign30610_e30005_d_n14;

        let (assign30620_e30015, assign30620_e30015_d_n0, assign30620_e30015_d_n2, assign30620_e30015_d_n4, assign30620_e30015_d_n5, assign30620_e30015_d_n6, assign30620_e30015_d_n7, assign30620_e30015_d_n8, assign30620_e30015_d_n9, assign30620_e30015_d_n10, assign30620_e30015_d_n11, assign30620_e30015_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30620_e30015;
        locals.var_dnm_dn0 = assign30620_e30015_d_n0;
        locals.var_dnm_dn2 = assign30620_e30015_d_n2;
        locals.var_dnm_dn4 = assign30620_e30015_d_n4;
        locals.var_dnm_dn5 = assign30620_e30015_d_n5;
        locals.var_dnm_dn6 = assign30620_e30015_d_n6;
        locals.var_dnm_dn7 = assign30620_e30015_d_n7;
        locals.var_dnm_dn8 = assign30620_e30015_d_n8;
        locals.var_dnm_dn9 = assign30620_e30015_d_n9;
        locals.var_dnm_dn10 = assign30620_e30015_d_n10;
        locals.var_dnm_dn11 = assign30620_e30015_d_n11;
        locals.var_dnm_dn14 = assign30620_e30015_d_n14;

        let assign30630_e30030: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard704 = assign30630_e30030;

        let assign30640_e30033: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard705 = assign30640_e30033;

        let (assign30650_e30047,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) && (locals.var_guard704 != 0.0)) && (locals.var_guard705 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30650_e30047;

        let assign30660_e30050: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard706 = assign30660_e30050;

        let (assign30670_e30067,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) && (locals.var_guard704 != 0.0)) && (locals.var_guard705 == 0.0)) && (locals.var_guard706 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30670_e30067;

        let assign30680_e30070: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard707 = assign30680_e30070;

        let (assign30690_e30090,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) && (locals.var_guard704 != 0.0)) && (locals.var_guard705 == 0.0)) && (locals.var_guard706 == 0.0)) && (locals.var_guard707 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30690_e30090;

        let assign30700_e30093: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard708 = assign30700_e30093;

        let (assign30710_e30116,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) && (locals.var_guard704 != 0.0)) && (locals.var_guard705 == 0.0)) && (locals.var_guard706 == 0.0)) && (locals.var_guard707 == 0.0)) && (locals.var_guard708 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30710_e30116;

        let (assign30720_e30128,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) && (locals.var_guard704 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30720_e30128;

    }

    pub(super) fn stamp_transient_block_89(
        locals: &mut StampLocals,
    ) {
        let mut assign30730_loop_guard: usize = 0;
        while {
            let assign30730_cond_e30141: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) && (locals.var_guard704 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign30730_cond_e30141 != 0.0
        } {
            assign30730_loop_guard += 1;
            assert!(assign30730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign30730_body0_e30154, assign30730_body0_e30154_d_n0, assign30730_body0_e30154_d_n2, assign30730_body0_e30154_d_n4, assign30730_body0_e30154_d_n5, assign30730_body0_e30154_d_n6, assign30730_body0_e30154_d_n7, assign30730_body0_e30154_d_n8, assign30730_body0_e30154_d_n9, assign30730_body0_e30154_d_n10, assign30730_body0_e30154_d_n11, assign30730_body0_e30154_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) && (locals.var_guard704 != 0.0)) {
        let assign30730_body0_e30152: f64 = (locals.var_dnm).sqrt();
        (assign30730_body0_e30152, (locals.var_dnm_dn0 / (2.0 * assign30730_body0_e30152)), (locals.var_dnm_dn2 / (2.0 * assign30730_body0_e30152)), (locals.var_dnm_dn4 / (2.0 * assign30730_body0_e30152)), (locals.var_dnm_dn5 / (2.0 * assign30730_body0_e30152)), (locals.var_dnm_dn6 / (2.0 * assign30730_body0_e30152)), (locals.var_dnm_dn7 / (2.0 * assign30730_body0_e30152)), (locals.var_dnm_dn8 / (2.0 * assign30730_body0_e30152)), (locals.var_dnm_dn9 / (2.0 * assign30730_body0_e30152)), (locals.var_dnm_dn10 / (2.0 * assign30730_body0_e30152)), (locals.var_dnm_dn11 / (2.0 * assign30730_body0_e30152)), (locals.var_dnm_dn14 / (2.0 * assign30730_body0_e30152)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign30730_body0_e30154;
            locals.var_dnm_dn0 = assign30730_body0_e30154_d_n0;
            locals.var_dnm_dn2 = assign30730_body0_e30154_d_n2;
            locals.var_dnm_dn4 = assign30730_body0_e30154_d_n4;
            locals.var_dnm_dn5 = assign30730_body0_e30154_d_n5;
            locals.var_dnm_dn6 = assign30730_body0_e30154_d_n6;
            locals.var_dnm_dn7 = assign30730_body0_e30154_d_n7;
            locals.var_dnm_dn8 = assign30730_body0_e30154_d_n8;
            locals.var_dnm_dn9 = assign30730_body0_e30154_d_n9;
            locals.var_dnm_dn10 = assign30730_body0_e30154_d_n10;
            locals.var_dnm_dn11 = assign30730_body0_e30154_d_n11;
            locals.var_dnm_dn14 = assign30730_body0_e30154_d_n14;
            let (assign30730_body1_e30168,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) && (locals.var_guard704 != 0.0)) {
        let assign30730_body1_e30166: f64 = (locals.var_m0 + 1.0);
        (assign30730_body1_e30166,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign30730_body1_e30168;
        }

        let (assign30740_e30192, assign30740_e30192_d_n0, assign30740_e30192_d_n2, assign30740_e30192_d_n4, assign30740_e30192_d_n5, assign30740_e30192_d_n6, assign30740_e30192_d_n7, assign30740_e30192_d_n8, assign30740_e30192_d_n9, assign30740_e30192_d_n10, assign30740_e30192_d_n11, assign30740_e30192_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) && (locals.var_guard704 == 0.0)) {
        let (assign30740_e30190, assign30740_e30190_d_n0, assign30740_e30190_d_n2, assign30740_e30190_d_n4, assign30740_e30190_d_n5, assign30740_e30190_d_n6, assign30740_e30190_d_n7, assign30740_e30190_d_n8, assign30740_e30190_d_n9, assign30740_e30190_d_n10, assign30740_e30190_d_n11, assign30740_e30190_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign30740_e30187: f64 = (2.0 * 2.0);
                let assign30740_e30188: f64 = (1.0 / assign30740_e30187);
                let assign30740_e30189: f64 = (locals.var_dnm).powf(assign30740_e30188);
                (assign30740_e30189, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn0)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn2)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn4)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn5)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn6)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn7)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn8)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn9)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn10)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn11)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30740_e30188) as f64).is_finite() && ((assign30740_e30188) as f64).fract() == 0.0 { if assign30740_e30188 == 0.0 { 0.0 } else { (assign30740_e30188 * ((locals.var_dnm).powf(assign30740_e30188 - 1.0) * locals.var_dnm_dn14)) } } else { (assign30740_e30189 * (assign30740_e30188 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign30740_e30190, assign30740_e30190_d_n0, assign30740_e30190_d_n2, assign30740_e30190_d_n4, assign30740_e30190_d_n5, assign30740_e30190_d_n6, assign30740_e30190_d_n7, assign30740_e30190_d_n8, assign30740_e30190_d_n9, assign30740_e30190_d_n10, assign30740_e30190_d_n11, assign30740_e30190_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30740_e30192;
        locals.var_dnm_dn0 = assign30740_e30192_d_n0;
        locals.var_dnm_dn2 = assign30740_e30192_d_n2;
        locals.var_dnm_dn4 = assign30740_e30192_d_n4;
        locals.var_dnm_dn5 = assign30740_e30192_d_n5;
        locals.var_dnm_dn6 = assign30740_e30192_d_n6;
        locals.var_dnm_dn7 = assign30740_e30192_d_n7;
        locals.var_dnm_dn8 = assign30740_e30192_d_n8;
        locals.var_dnm_dn9 = assign30740_e30192_d_n9;
        locals.var_dnm_dn10 = assign30740_e30192_d_n10;
        locals.var_dnm_dn11 = assign30740_e30192_d_n11;
        locals.var_dnm_dn14 = assign30740_e30192_d_n14;

        let (assign30750_e30204, assign30750_e30204_d_n0, assign30750_e30204_d_n2, assign30750_e30204_d_n4, assign30750_e30204_d_n5, assign30750_e30204_d_n6, assign30750_e30204_d_n7, assign30750_e30204_d_n8, assign30750_e30204_d_n9, assign30750_e30204_d_n10, assign30750_e30204_d_n11, assign30750_e30204_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30750_e30202: f64 = (1.0 / locals.var_dnm);
        (assign30750_e30202, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30750_e30204;
        locals.var_dnm_dn0 = assign30750_e30204_d_n0;
        locals.var_dnm_dn2 = assign30750_e30204_d_n2;
        locals.var_dnm_dn4 = assign30750_e30204_d_n4;
        locals.var_dnm_dn5 = assign30750_e30204_d_n5;
        locals.var_dnm_dn6 = assign30750_e30204_d_n6;
        locals.var_dnm_dn7 = assign30750_e30204_d_n7;
        locals.var_dnm_dn8 = assign30750_e30204_d_n8;
        locals.var_dnm_dn9 = assign30750_e30204_d_n9;
        locals.var_dnm_dn10 = assign30750_e30204_d_n10;
        locals.var_dnm_dn11 = assign30750_e30204_d_n11;
        locals.var_dnm_dn14 = assign30750_e30204_d_n14;

        let (assign30760_e30218, assign30760_e30218_d_n0, assign30760_e30218_d_n2, assign30760_e30218_d_n4, assign30760_e30218_d_n5, assign30760_e30218_d_n6, assign30760_e30218_d_n7, assign30760_e30218_d_n8, assign30760_e30218_d_n9, assign30760_e30218_d_n10, assign30760_e30218_d_n11, assign30760_e30218_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30760_e30214: f64 = (locals.var_tmf1 * 0.5);
        let assign30760_e30216: f64 = (assign30760_e30214 * locals.var_dnm);
        (assign30760_e30216, (((locals.var_tmf1_dn0 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.5) * locals.var_dnm) + (assign30760_e30214 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign30760_e30218;
        locals.var_tmf0_dn0 = assign30760_e30218_d_n0;
        locals.var_tmf0_dn2 = assign30760_e30218_d_n2;
        locals.var_tmf0_dn4 = assign30760_e30218_d_n4;
        locals.var_tmf0_dn5 = assign30760_e30218_d_n5;
        locals.var_tmf0_dn6 = assign30760_e30218_d_n6;
        locals.var_tmf0_dn7 = assign30760_e30218_d_n7;
        locals.var_tmf0_dn8 = assign30760_e30218_d_n8;
        locals.var_tmf0_dn9 = assign30760_e30218_d_n9;
        locals.var_tmf0_dn10 = assign30760_e30218_d_n10;
        locals.var_tmf0_dn11 = assign30760_e30218_d_n11;
        locals.var_tmf0_dn14 = assign30760_e30218_d_n14;

        let (assign30770_e30234, assign30770_e30234_d_n0, assign30770_e30234_d_n2, assign30770_e30234_d_n4, assign30770_e30234_d_n5, assign30770_e30234_d_n6, assign30770_e30234_d_n7, assign30770_e30234_d_n8, assign30770_e30234_d_n9, assign30770_e30234_d_n10, assign30770_e30234_d_n11, assign30770_e30234_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30770_e30228: f64 = (0.5 * locals.var_xmp);
        let assign30770_e30230: f64 = (assign30770_e30228 * locals.var_dnm);
        let assign30770_e30232: f64 = (assign30770_e30230 / locals.var_arg);
        (assign30770_e30232, ((((((0.5 * locals.var_xmp_dn0) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn0)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn2) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn2)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn4) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn4)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn5) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn5)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn6) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn6)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn7) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn7)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn8) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn8)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn9) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn9)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn10) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn10)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn11) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn11)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn14) * locals.var_dnm) + (assign30770_e30228 * locals.var_dnm_dn14)) * locals.var_arg) - (assign30770_e30230 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30770_e30234;
        locals.var_t0_dn0 = assign30770_e30234_d_n0;
        locals.var_t0_dn2 = assign30770_e30234_d_n2;
        locals.var_t0_dn4 = assign30770_e30234_d_n4;
        locals.var_t0_dn5 = assign30770_e30234_d_n5;
        locals.var_t0_dn6 = assign30770_e30234_d_n6;
        locals.var_t0_dn7 = assign30770_e30234_d_n7;
        locals.var_t0_dn8 = assign30770_e30234_d_n8;
        locals.var_t0_dn9 = assign30770_e30234_d_n9;
        locals.var_t0_dn10 = assign30770_e30234_d_n10;
        locals.var_t0_dn11 = assign30770_e30234_d_n11;
        locals.var_t0_dn14 = assign30770_e30234_d_n14;

        let (assign30780_e30248, assign30780_e30248_d_n0, assign30780_e30248_d_n2, assign30780_e30248_d_n4, assign30780_e30248_d_n5, assign30780_e30248_d_n6, assign30780_e30248_d_n7, assign30780_e30248_d_n8, assign30780_e30248_d_n9, assign30780_e30248_d_n10, assign30780_e30248_d_n11, assign30780_e30248_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        let assign30780_e30244: f64 = 0.5;
        let assign30780_e30246: f64 = (assign30780_e30244 - locals.var_tmf0);
        (assign30780_e30246, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_vgpp, locals.var_vgpp_dn0, locals.var_vgpp_dn2, locals.var_vgpp_dn4, locals.var_vgpp_dn5, locals.var_vgpp_dn6, locals.var_vgpp_dn7, locals.var_vgpp_dn8, locals.var_vgpp_dn9, locals.var_vgpp_dn10, locals.var_vgpp_dn11, locals.var_vgpp_dn14,)
    }
};
        locals.var_vgpp = assign30780_e30248;
        locals.var_vgpp_dn0 = assign30780_e30248_d_n0;
        locals.var_vgpp_dn2 = assign30780_e30248_d_n2;
        locals.var_vgpp_dn4 = assign30780_e30248_d_n4;
        locals.var_vgpp_dn5 = assign30780_e30248_d_n5;
        locals.var_vgpp_dn6 = assign30780_e30248_d_n6;
        locals.var_vgpp_dn7 = assign30780_e30248_d_n7;
        locals.var_vgpp_dn8 = assign30780_e30248_d_n8;
        locals.var_vgpp_dn9 = assign30780_e30248_d_n9;
        locals.var_vgpp_dn10 = assign30780_e30248_d_n10;
        locals.var_vgpp_dn11 = assign30780_e30248_d_n11;
        locals.var_vgpp_dn14 = assign30780_e30248_d_n14;

        let (assign30790_e30258, assign30790_e30258_d_n0, assign30790_e30258_d_n2, assign30790_e30258_d_n4, assign30790_e30258_d_n5, assign30790_e30258_d_n6, assign30790_e30258_d_n7, assign30790_e30258_d_n8, assign30790_e30258_d_n9, assign30790_e30258_d_n10, assign30790_e30258_d_n11, assign30790_e30258_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30790_e30258;
        locals.var_t0_dn0 = assign30790_e30258_d_n0;
        locals.var_t0_dn2 = assign30790_e30258_d_n2;
        locals.var_t0_dn4 = assign30790_e30258_d_n4;
        locals.var_t0_dn5 = assign30790_e30258_d_n5;
        locals.var_t0_dn6 = assign30790_e30258_d_n6;
        locals.var_t0_dn7 = assign30790_e30258_d_n7;
        locals.var_t0_dn8 = assign30790_e30258_d_n8;
        locals.var_t0_dn9 = assign30790_e30258_d_n9;
        locals.var_t0_dn10 = assign30790_e30258_d_n10;
        locals.var_t0_dn11 = assign30790_e30258_d_n11;
        locals.var_t0_dn14 = assign30790_e30258_d_n14;

        let (assign30800_e30269, assign30800_e30269_d_n0, assign30800_e30269_d_n2, assign30800_e30269_d_n4, assign30800_e30269_d_n5, assign30800_e30269_d_n6, assign30800_e30269_d_n7, assign30800_e30269_d_n8, assign30800_e30269_d_n9, assign30800_e30269_d_n10, assign30800_e30269_d_n11, assign30800_e30269_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 == 0.0)) {
        (locals.var_vgp, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn11, locals.var_vgp_dn14,)
    } else {
        (locals.var_vgpp, locals.var_vgpp_dn0, locals.var_vgpp_dn2, locals.var_vgpp_dn4, locals.var_vgpp_dn5, locals.var_vgpp_dn6, locals.var_vgpp_dn7, locals.var_vgpp_dn8, locals.var_vgpp_dn9, locals.var_vgpp_dn10, locals.var_vgpp_dn11, locals.var_vgpp_dn14,)
    }
};
        locals.var_vgpp = assign30800_e30269;
        locals.var_vgpp_dn0 = assign30800_e30269_d_n0;
        locals.var_vgpp_dn2 = assign30800_e30269_d_n2;
        locals.var_vgpp_dn4 = assign30800_e30269_d_n4;
        locals.var_vgpp_dn5 = assign30800_e30269_d_n5;
        locals.var_vgpp_dn6 = assign30800_e30269_d_n6;
        locals.var_vgpp_dn7 = assign30800_e30269_d_n7;
        locals.var_vgpp_dn8 = assign30800_e30269_d_n8;
        locals.var_vgpp_dn9 = assign30800_e30269_d_n9;
        locals.var_vgpp_dn10 = assign30800_e30269_d_n10;
        locals.var_vgpp_dn11 = assign30800_e30269_d_n11;
        locals.var_vgpp_dn14 = assign30800_e30269_d_n14;

        let (assign30810_e30280, assign30810_e30280_d_n0, assign30810_e30280_d_n2, assign30810_e30280_d_n4, assign30810_e30280_d_n5, assign30810_e30280_d_n6, assign30810_e30280_d_n7, assign30810_e30280_d_n8, assign30810_e30280_d_n9, assign30810_e30280_d_n10, assign30810_e30280_d_n11, assign30810_e30280_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard703 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30810_e30280;
        locals.var_t0_dn0 = assign30810_e30280_d_n0;
        locals.var_t0_dn2 = assign30810_e30280_d_n2;
        locals.var_t0_dn4 = assign30810_e30280_d_n4;
        locals.var_t0_dn5 = assign30810_e30280_d_n5;
        locals.var_t0_dn6 = assign30810_e30280_d_n6;
        locals.var_t0_dn7 = assign30810_e30280_d_n7;
        locals.var_t0_dn8 = assign30810_e30280_d_n8;
        locals.var_t0_dn9 = assign30810_e30280_d_n9;
        locals.var_t0_dn10 = assign30810_e30280_d_n10;
        locals.var_t0_dn11 = assign30810_e30280_d_n11;
        locals.var_t0_dn14 = assign30810_e30280_d_n14;

        let (assign30820_e30290, assign30820_e30290_d_n0, assign30820_e30290_d_n2, assign30820_e30290_d_n4, assign30820_e30290_d_n5, assign30820_e30290_d_n6, assign30820_e30290_d_n7, assign30820_e30290_d_n8, assign30820_e30290_d_n9, assign30820_e30290_d_n10, assign30820_e30290_d_n11, assign30820_e30290_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign30820_e30288: f64 = (locals.var_vgpp * 0.8);
        (assign30820_e30288, (locals.var_vgpp_dn0 * 0.8), (locals.var_vgpp_dn2 * 0.8), (locals.var_vgpp_dn4 * 0.8), (locals.var_vgpp_dn5 * 0.8), (locals.var_vgpp_dn6 * 0.8), (locals.var_vgpp_dn7 * 0.8), (locals.var_vgpp_dn8 * 0.8), (locals.var_vgpp_dn9 * 0.8), (locals.var_vgpp_dn10 * 0.8), (locals.var_vgpp_dn11 * 0.8), (locals.var_vgpp_dn14 * 0.8),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign30820_e30290;
        locals.var_t1_dn0 = assign30820_e30290_d_n0;
        locals.var_t1_dn2 = assign30820_e30290_d_n2;
        locals.var_t1_dn4 = assign30820_e30290_d_n4;
        locals.var_t1_dn5 = assign30820_e30290_d_n5;
        locals.var_t1_dn6 = assign30820_e30290_d_n6;
        locals.var_t1_dn7 = assign30820_e30290_d_n7;
        locals.var_t1_dn8 = assign30820_e30290_d_n8;
        locals.var_t1_dn9 = assign30820_e30290_d_n9;
        locals.var_t1_dn10 = assign30820_e30290_d_n10;
        locals.var_t1_dn11 = assign30820_e30290_d_n11;
        locals.var_t1_dn14 = assign30820_e30290_d_n14;

        let assign30830_e30294: f64 = (locals.var_vgpp - locals.var_t1);
        let assign30830_e30299: f64 = if ((locals.var_vdseff > assign30830_e30294) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard709 = assign30830_e30299;

        let (assign30840_e30313, assign30840_e30313_d_n0, assign30840_e30313_d_n2, assign30840_e30313_d_n4, assign30840_e30313_d_n5, assign30840_e30313_d_n6, assign30840_e30313_d_n7, assign30840_e30313_d_n8, assign30840_e30313_d_n9, assign30840_e30313_d_n10, assign30840_e30313_d_n11, assign30840_e30313_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign30840_e30309: f64 = (locals.var_vdseff - locals.var_vgpp);
        let assign30840_e30311: f64 = (assign30840_e30309 + locals.var_t1);
        (assign30840_e30311, ((locals.var_vdseff_dn0 - locals.var_vgpp_dn0) + locals.var_t1_dn0), ((locals.var_vdseff_dn2 - locals.var_vgpp_dn2) + locals.var_t1_dn2), ((locals.var_vdseff_dn4 - locals.var_vgpp_dn4) + locals.var_t1_dn4), ((locals.var_vdseff_dn5 - locals.var_vgpp_dn5) + locals.var_t1_dn5), ((locals.var_vdseff_dn6 - locals.var_vgpp_dn6) + locals.var_t1_dn6), ((locals.var_vdseff_dn7 - locals.var_vgpp_dn7) + locals.var_t1_dn7), ((locals.var_vdseff_dn8 - locals.var_vgpp_dn8) + locals.var_t1_dn8), ((locals.var_vdseff_dn9 - locals.var_vgpp_dn9) + locals.var_t1_dn9), ((locals.var_vdseff_dn10 - locals.var_vgpp_dn10) + locals.var_t1_dn10), ((locals.var_vdseff_dn11 - locals.var_vgpp_dn11) + locals.var_t1_dn11), ((locals.var_vdseff_dn14 - locals.var_vgpp_dn14) + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign30840_e30313;
        locals.var_tmf1_dn0 = assign30840_e30313_d_n0;
        locals.var_tmf1_dn2 = assign30840_e30313_d_n2;
        locals.var_tmf1_dn4 = assign30840_e30313_d_n4;
        locals.var_tmf1_dn5 = assign30840_e30313_d_n5;
        locals.var_tmf1_dn6 = assign30840_e30313_d_n6;
        locals.var_tmf1_dn7 = assign30840_e30313_d_n7;
        locals.var_tmf1_dn8 = assign30840_e30313_d_n8;
        locals.var_tmf1_dn9 = assign30840_e30313_d_n9;
        locals.var_tmf1_dn10 = assign30840_e30313_d_n10;
        locals.var_tmf1_dn11 = assign30840_e30313_d_n11;
        locals.var_tmf1_dn14 = assign30840_e30313_d_n14;

        let (assign30850_e30325, assign30850_e30325_d_n0, assign30850_e30325_d_n2, assign30850_e30325_d_n4, assign30850_e30325_d_n5, assign30850_e30325_d_n6, assign30850_e30325_d_n7, assign30850_e30325_d_n8, assign30850_e30325_d_n9, assign30850_e30325_d_n10, assign30850_e30325_d_n11, assign30850_e30325_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign30850_e30323: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign30850_e30323, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign30850_e30325;
        locals.var_x2_dn0 = assign30850_e30325_d_n0;
        locals.var_x2_dn2 = assign30850_e30325_d_n2;
        locals.var_x2_dn4 = assign30850_e30325_d_n4;
        locals.var_x2_dn5 = assign30850_e30325_d_n5;
        locals.var_x2_dn6 = assign30850_e30325_d_n6;
        locals.var_x2_dn7 = assign30850_e30325_d_n7;
        locals.var_x2_dn8 = assign30850_e30325_d_n8;
        locals.var_x2_dn9 = assign30850_e30325_d_n9;
        locals.var_x2_dn10 = assign30850_e30325_d_n10;
        locals.var_x2_dn11 = assign30850_e30325_d_n11;
        locals.var_x2_dn14 = assign30850_e30325_d_n14;

        let (assign30860_e30337, assign30860_e30337_d_n0, assign30860_e30337_d_n2, assign30860_e30337_d_n4, assign30860_e30337_d_n5, assign30860_e30337_d_n6, assign30860_e30337_d_n7, assign30860_e30337_d_n8, assign30860_e30337_d_n9, assign30860_e30337_d_n10, assign30860_e30337_d_n11, assign30860_e30337_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign30860_e30335: f64 = (locals.var_t1 * locals.var_t1);
        (assign30860_e30335, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign30860_e30337;
        locals.var_xmax2_dn0 = assign30860_e30337_d_n0;
        locals.var_xmax2_dn2 = assign30860_e30337_d_n2;
        locals.var_xmax2_dn4 = assign30860_e30337_d_n4;
        locals.var_xmax2_dn5 = assign30860_e30337_d_n5;
        locals.var_xmax2_dn6 = assign30860_e30337_d_n6;
        locals.var_xmax2_dn7 = assign30860_e30337_d_n7;
        locals.var_xmax2_dn8 = assign30860_e30337_d_n8;
        locals.var_xmax2_dn9 = assign30860_e30337_d_n9;
        locals.var_xmax2_dn10 = assign30860_e30337_d_n10;
        locals.var_xmax2_dn11 = assign30860_e30337_d_n11;
        locals.var_xmax2_dn14 = assign30860_e30337_d_n14;

        let (assign30870_e30347, assign30870_e30347_d_n0, assign30870_e30347_d_n2, assign30870_e30347_d_n4, assign30870_e30347_d_n5, assign30870_e30347_d_n6, assign30870_e30347_d_n7, assign30870_e30347_d_n8, assign30870_e30347_d_n9, assign30870_e30347_d_n10, assign30870_e30347_d_n11, assign30870_e30347_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30870_e30347;
        locals.var_xp_dn0 = assign30870_e30347_d_n0;
        locals.var_xp_dn2 = assign30870_e30347_d_n2;
        locals.var_xp_dn4 = assign30870_e30347_d_n4;
        locals.var_xp_dn5 = assign30870_e30347_d_n5;
        locals.var_xp_dn6 = assign30870_e30347_d_n6;
        locals.var_xp_dn7 = assign30870_e30347_d_n7;
        locals.var_xp_dn8 = assign30870_e30347_d_n8;
        locals.var_xp_dn9 = assign30870_e30347_d_n9;
        locals.var_xp_dn10 = assign30870_e30347_d_n10;
        locals.var_xp_dn11 = assign30870_e30347_d_n11;
        locals.var_xp_dn14 = assign30870_e30347_d_n14;

        let (assign30880_e30357, assign30880_e30357_d_n0, assign30880_e30357_d_n2, assign30880_e30357_d_n4, assign30880_e30357_d_n5, assign30880_e30357_d_n6, assign30880_e30357_d_n7, assign30880_e30357_d_n8, assign30880_e30357_d_n9, assign30880_e30357_d_n10, assign30880_e30357_d_n11, assign30880_e30357_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30880_e30357;
        locals.var_xmp_dn0 = assign30880_e30357_d_n0;
        locals.var_xmp_dn2 = assign30880_e30357_d_n2;
        locals.var_xmp_dn4 = assign30880_e30357_d_n4;
        locals.var_xmp_dn5 = assign30880_e30357_d_n5;
        locals.var_xmp_dn6 = assign30880_e30357_d_n6;
        locals.var_xmp_dn7 = assign30880_e30357_d_n7;
        locals.var_xmp_dn8 = assign30880_e30357_d_n8;
        locals.var_xmp_dn9 = assign30880_e30357_d_n9;
        locals.var_xmp_dn10 = assign30880_e30357_d_n10;
        locals.var_xmp_dn11 = assign30880_e30357_d_n11;
        locals.var_xmp_dn14 = assign30880_e30357_d_n14;

        let (assign30890_e30367,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30890_e30367;

        let (assign30900_e30377,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30900_e30377;

        let (assign30910_e30387, assign30910_e30387_d_n0, assign30910_e30387_d_n2, assign30910_e30387_d_n4, assign30910_e30387_d_n5, assign30910_e30387_d_n6, assign30910_e30387_d_n7, assign30910_e30387_d_n8, assign30910_e30387_d_n9, assign30910_e30387_d_n10, assign30910_e30387_d_n11, assign30910_e30387_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30910_e30387;
        locals.var_arg_dn0 = assign30910_e30387_d_n0;
        locals.var_arg_dn2 = assign30910_e30387_d_n2;
        locals.var_arg_dn4 = assign30910_e30387_d_n4;
        locals.var_arg_dn5 = assign30910_e30387_d_n5;
        locals.var_arg_dn6 = assign30910_e30387_d_n6;
        locals.var_arg_dn7 = assign30910_e30387_d_n7;
        locals.var_arg_dn8 = assign30910_e30387_d_n8;
        locals.var_arg_dn9 = assign30910_e30387_d_n9;
        locals.var_arg_dn10 = assign30910_e30387_d_n10;
        locals.var_arg_dn11 = assign30910_e30387_d_n11;
        locals.var_arg_dn14 = assign30910_e30387_d_n14;

        let (assign30920_e30397, assign30920_e30397_d_n0, assign30920_e30397_d_n2, assign30920_e30397_d_n4, assign30920_e30397_d_n5, assign30920_e30397_d_n6, assign30920_e30397_d_n7, assign30920_e30397_d_n8, assign30920_e30397_d_n9, assign30920_e30397_d_n10, assign30920_e30397_d_n11, assign30920_e30397_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30920_e30397;
        locals.var_dnm_dn0 = assign30920_e30397_d_n0;
        locals.var_dnm_dn2 = assign30920_e30397_d_n2;
        locals.var_dnm_dn4 = assign30920_e30397_d_n4;
        locals.var_dnm_dn5 = assign30920_e30397_d_n5;
        locals.var_dnm_dn6 = assign30920_e30397_d_n6;
        locals.var_dnm_dn7 = assign30920_e30397_d_n7;
        locals.var_dnm_dn8 = assign30920_e30397_d_n8;
        locals.var_dnm_dn9 = assign30920_e30397_d_n9;
        locals.var_dnm_dn10 = assign30920_e30397_d_n10;
        locals.var_dnm_dn11 = assign30920_e30397_d_n11;
        locals.var_dnm_dn14 = assign30920_e30397_d_n14;

        let (assign30930_e30409, assign30930_e30409_d_n0, assign30930_e30409_d_n2, assign30930_e30409_d_n4, assign30930_e30409_d_n5, assign30930_e30409_d_n6, assign30930_e30409_d_n7, assign30930_e30409_d_n8, assign30930_e30409_d_n9, assign30930_e30409_d_n10, assign30930_e30409_d_n11, assign30930_e30409_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign30930_e30407: f64 = (locals.var_xp * locals.var_x2);
        (assign30930_e30407, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30930_e30409;
        locals.var_xp_dn0 = assign30930_e30409_d_n0;
        locals.var_xp_dn2 = assign30930_e30409_d_n2;
        locals.var_xp_dn4 = assign30930_e30409_d_n4;
        locals.var_xp_dn5 = assign30930_e30409_d_n5;
        locals.var_xp_dn6 = assign30930_e30409_d_n6;
        locals.var_xp_dn7 = assign30930_e30409_d_n7;
        locals.var_xp_dn8 = assign30930_e30409_d_n8;
        locals.var_xp_dn9 = assign30930_e30409_d_n9;
        locals.var_xp_dn10 = assign30930_e30409_d_n10;
        locals.var_xp_dn11 = assign30930_e30409_d_n11;
        locals.var_xp_dn14 = assign30930_e30409_d_n14;

        let (assign30940_e30421, assign30940_e30421_d_n0, assign30940_e30421_d_n2, assign30940_e30421_d_n4, assign30940_e30421_d_n5, assign30940_e30421_d_n6, assign30940_e30421_d_n7, assign30940_e30421_d_n8, assign30940_e30421_d_n9, assign30940_e30421_d_n10, assign30940_e30421_d_n11, assign30940_e30421_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign30940_e30419: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30940_e30419, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30940_e30421;
        locals.var_xmp_dn0 = assign30940_e30421_d_n0;
        locals.var_xmp_dn2 = assign30940_e30421_d_n2;
        locals.var_xmp_dn4 = assign30940_e30421_d_n4;
        locals.var_xmp_dn5 = assign30940_e30421_d_n5;
        locals.var_xmp_dn6 = assign30940_e30421_d_n6;
        locals.var_xmp_dn7 = assign30940_e30421_d_n7;
        locals.var_xmp_dn8 = assign30940_e30421_d_n8;
        locals.var_xmp_dn9 = assign30940_e30421_d_n9;
        locals.var_xmp_dn10 = assign30940_e30421_d_n10;
        locals.var_xmp_dn11 = assign30940_e30421_d_n11;
        locals.var_xmp_dn14 = assign30940_e30421_d_n14;

        let (assign30950_e30433, assign30950_e30433_d_n0, assign30950_e30433_d_n2, assign30950_e30433_d_n4, assign30950_e30433_d_n5, assign30950_e30433_d_n6, assign30950_e30433_d_n7, assign30950_e30433_d_n8, assign30950_e30433_d_n9, assign30950_e30433_d_n10, assign30950_e30433_d_n11, assign30950_e30433_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign30950_e30431: f64 = (locals.var_xp * locals.var_x2);
        (assign30950_e30431, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30950_e30433;
        locals.var_xp_dn0 = assign30950_e30433_d_n0;
        locals.var_xp_dn2 = assign30950_e30433_d_n2;
        locals.var_xp_dn4 = assign30950_e30433_d_n4;
        locals.var_xp_dn5 = assign30950_e30433_d_n5;
        locals.var_xp_dn6 = assign30950_e30433_d_n6;
        locals.var_xp_dn7 = assign30950_e30433_d_n7;
        locals.var_xp_dn8 = assign30950_e30433_d_n8;
        locals.var_xp_dn9 = assign30950_e30433_d_n9;
        locals.var_xp_dn10 = assign30950_e30433_d_n10;
        locals.var_xp_dn11 = assign30950_e30433_d_n11;
        locals.var_xp_dn14 = assign30950_e30433_d_n14;

        let (assign30960_e30445, assign30960_e30445_d_n0, assign30960_e30445_d_n2, assign30960_e30445_d_n4, assign30960_e30445_d_n5, assign30960_e30445_d_n6, assign30960_e30445_d_n7, assign30960_e30445_d_n8, assign30960_e30445_d_n9, assign30960_e30445_d_n10, assign30960_e30445_d_n11, assign30960_e30445_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign30960_e30443: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30960_e30443, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30960_e30445;
        locals.var_xmp_dn0 = assign30960_e30445_d_n0;
        locals.var_xmp_dn2 = assign30960_e30445_d_n2;
        locals.var_xmp_dn4 = assign30960_e30445_d_n4;
        locals.var_xmp_dn5 = assign30960_e30445_d_n5;
        locals.var_xmp_dn6 = assign30960_e30445_d_n6;
        locals.var_xmp_dn7 = assign30960_e30445_d_n7;
        locals.var_xmp_dn8 = assign30960_e30445_d_n8;
        locals.var_xmp_dn9 = assign30960_e30445_d_n9;
        locals.var_xmp_dn10 = assign30960_e30445_d_n10;
        locals.var_xmp_dn11 = assign30960_e30445_d_n11;
        locals.var_xmp_dn14 = assign30960_e30445_d_n14;

        let (assign30970_e30457, assign30970_e30457_d_n0, assign30970_e30457_d_n2, assign30970_e30457_d_n4, assign30970_e30457_d_n5, assign30970_e30457_d_n6, assign30970_e30457_d_n7, assign30970_e30457_d_n8, assign30970_e30457_d_n9, assign30970_e30457_d_n10, assign30970_e30457_d_n11, assign30970_e30457_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign30970_e30455: f64 = (locals.var_xp + locals.var_xmp);
        (assign30970_e30455, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30970_e30457;
        locals.var_arg_dn0 = assign30970_e30457_d_n0;
        locals.var_arg_dn2 = assign30970_e30457_d_n2;
        locals.var_arg_dn4 = assign30970_e30457_d_n4;
        locals.var_arg_dn5 = assign30970_e30457_d_n5;
        locals.var_arg_dn6 = assign30970_e30457_d_n6;
        locals.var_arg_dn7 = assign30970_e30457_d_n7;
        locals.var_arg_dn8 = assign30970_e30457_d_n8;
        locals.var_arg_dn9 = assign30970_e30457_d_n9;
        locals.var_arg_dn10 = assign30970_e30457_d_n10;
        locals.var_arg_dn11 = assign30970_e30457_d_n11;
        locals.var_arg_dn14 = assign30970_e30457_d_n14;

    }

    pub(super) fn stamp_transient_block_90(
        locals: &mut StampLocals,
    ) {
        let (assign30980_e30467, assign30980_e30467_d_n0, assign30980_e30467_d_n2, assign30980_e30467_d_n4, assign30980_e30467_d_n5, assign30980_e30467_d_n6, assign30980_e30467_d_n7, assign30980_e30467_d_n8, assign30980_e30467_d_n9, assign30980_e30467_d_n10, assign30980_e30467_d_n11, assign30980_e30467_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30980_e30467;
        locals.var_dnm_dn0 = assign30980_e30467_d_n0;
        locals.var_dnm_dn2 = assign30980_e30467_d_n2;
        locals.var_dnm_dn4 = assign30980_e30467_d_n4;
        locals.var_dnm_dn5 = assign30980_e30467_d_n5;
        locals.var_dnm_dn6 = assign30980_e30467_d_n6;
        locals.var_dnm_dn7 = assign30980_e30467_d_n7;
        locals.var_dnm_dn8 = assign30980_e30467_d_n8;
        locals.var_dnm_dn9 = assign30980_e30467_d_n9;
        locals.var_dnm_dn10 = assign30980_e30467_d_n10;
        locals.var_dnm_dn11 = assign30980_e30467_d_n11;
        locals.var_dnm_dn14 = assign30980_e30467_d_n14;

        let assign30990_e30482: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard710 = assign30990_e30482;

        let assign31000_e30485: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard711 = assign31000_e30485;

        let (assign31010_e30499,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) && (locals.var_guard710 != 0.0)) && (locals.var_guard711 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign31010_e30499;

        let assign31020_e30502: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard712 = assign31020_e30502;

        let (assign31030_e30519,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) && (locals.var_guard710 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign31030_e30519;

        let assign31040_e30522: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard713 = assign31040_e30522;

        let (assign31050_e30542,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) && (locals.var_guard710 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) && (locals.var_guard713 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign31050_e30542;

        let assign31060_e30545: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard714 = assign31060_e30545;

        let (assign31070_e30568,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) && (locals.var_guard710 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 == 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign31070_e30568;

        let (assign31080_e30580,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) && (locals.var_guard710 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign31080_e30580;

        let mut assign31090_loop_guard: usize = 0;
        while {
            let assign31090_cond_e30593: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) && (locals.var_guard710 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign31090_cond_e30593 != 0.0
        } {
            assign31090_loop_guard += 1;
            assert!(assign31090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign31090_body0_e30606, assign31090_body0_e30606_d_n0, assign31090_body0_e30606_d_n2, assign31090_body0_e30606_d_n4, assign31090_body0_e30606_d_n5, assign31090_body0_e30606_d_n6, assign31090_body0_e30606_d_n7, assign31090_body0_e30606_d_n8, assign31090_body0_e30606_d_n9, assign31090_body0_e30606_d_n10, assign31090_body0_e30606_d_n11, assign31090_body0_e30606_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) && (locals.var_guard710 != 0.0)) {
        let assign31090_body0_e30604: f64 = (locals.var_dnm).sqrt();
        (assign31090_body0_e30604, (locals.var_dnm_dn0 / (2.0 * assign31090_body0_e30604)), (locals.var_dnm_dn2 / (2.0 * assign31090_body0_e30604)), (locals.var_dnm_dn4 / (2.0 * assign31090_body0_e30604)), (locals.var_dnm_dn5 / (2.0 * assign31090_body0_e30604)), (locals.var_dnm_dn6 / (2.0 * assign31090_body0_e30604)), (locals.var_dnm_dn7 / (2.0 * assign31090_body0_e30604)), (locals.var_dnm_dn8 / (2.0 * assign31090_body0_e30604)), (locals.var_dnm_dn9 / (2.0 * assign31090_body0_e30604)), (locals.var_dnm_dn10 / (2.0 * assign31090_body0_e30604)), (locals.var_dnm_dn11 / (2.0 * assign31090_body0_e30604)), (locals.var_dnm_dn14 / (2.0 * assign31090_body0_e30604)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31090_body0_e30606;
            locals.var_dnm_dn0 = assign31090_body0_e30606_d_n0;
            locals.var_dnm_dn2 = assign31090_body0_e30606_d_n2;
            locals.var_dnm_dn4 = assign31090_body0_e30606_d_n4;
            locals.var_dnm_dn5 = assign31090_body0_e30606_d_n5;
            locals.var_dnm_dn6 = assign31090_body0_e30606_d_n6;
            locals.var_dnm_dn7 = assign31090_body0_e30606_d_n7;
            locals.var_dnm_dn8 = assign31090_body0_e30606_d_n8;
            locals.var_dnm_dn9 = assign31090_body0_e30606_d_n9;
            locals.var_dnm_dn10 = assign31090_body0_e30606_d_n10;
            locals.var_dnm_dn11 = assign31090_body0_e30606_d_n11;
            locals.var_dnm_dn14 = assign31090_body0_e30606_d_n14;
            let (assign31090_body1_e30620,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) && (locals.var_guard710 != 0.0)) {
        let assign31090_body1_e30618: f64 = (locals.var_m0 + 1.0);
        (assign31090_body1_e30618,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign31090_body1_e30620;
        }

        let (assign31100_e30644, assign31100_e30644_d_n0, assign31100_e30644_d_n2, assign31100_e30644_d_n4, assign31100_e30644_d_n5, assign31100_e30644_d_n6, assign31100_e30644_d_n7, assign31100_e30644_d_n8, assign31100_e30644_d_n9, assign31100_e30644_d_n10, assign31100_e30644_d_n11, assign31100_e30644_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) && (locals.var_guard710 == 0.0)) {
        let (assign31100_e30642, assign31100_e30642_d_n0, assign31100_e30642_d_n2, assign31100_e30642_d_n4, assign31100_e30642_d_n5, assign31100_e30642_d_n6, assign31100_e30642_d_n7, assign31100_e30642_d_n8, assign31100_e30642_d_n9, assign31100_e30642_d_n10, assign31100_e30642_d_n11, assign31100_e30642_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign31100_e30639: f64 = (2.0 * 2.0);
                let assign31100_e30640: f64 = (1.0 / assign31100_e30639);
                let assign31100_e30641: f64 = (locals.var_dnm).powf(assign31100_e30640);
                (assign31100_e30641, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn0)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn2)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn4)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn5)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn6)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn7)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn8)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn9)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn10)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn11)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31100_e30640) as f64).is_finite() && ((assign31100_e30640) as f64).fract() == 0.0 { if assign31100_e30640 == 0.0 { 0.0 } else { (assign31100_e30640 * ((locals.var_dnm).powf(assign31100_e30640 - 1.0) * locals.var_dnm_dn14)) } } else { (assign31100_e30641 * (assign31100_e30640 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign31100_e30642, assign31100_e30642_d_n0, assign31100_e30642_d_n2, assign31100_e30642_d_n4, assign31100_e30642_d_n5, assign31100_e30642_d_n6, assign31100_e30642_d_n7, assign31100_e30642_d_n8, assign31100_e30642_d_n9, assign31100_e30642_d_n10, assign31100_e30642_d_n11, assign31100_e30642_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign31100_e30644;
        locals.var_dnm_dn0 = assign31100_e30644_d_n0;
        locals.var_dnm_dn2 = assign31100_e30644_d_n2;
        locals.var_dnm_dn4 = assign31100_e30644_d_n4;
        locals.var_dnm_dn5 = assign31100_e30644_d_n5;
        locals.var_dnm_dn6 = assign31100_e30644_d_n6;
        locals.var_dnm_dn7 = assign31100_e30644_d_n7;
        locals.var_dnm_dn8 = assign31100_e30644_d_n8;
        locals.var_dnm_dn9 = assign31100_e30644_d_n9;
        locals.var_dnm_dn10 = assign31100_e30644_d_n10;
        locals.var_dnm_dn11 = assign31100_e30644_d_n11;
        locals.var_dnm_dn14 = assign31100_e30644_d_n14;

        let (assign31110_e30656, assign31110_e30656_d_n0, assign31110_e30656_d_n2, assign31110_e30656_d_n4, assign31110_e30656_d_n5, assign31110_e30656_d_n6, assign31110_e30656_d_n7, assign31110_e30656_d_n8, assign31110_e30656_d_n9, assign31110_e30656_d_n10, assign31110_e30656_d_n11, assign31110_e30656_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign31110_e30654: f64 = (1.0 / locals.var_dnm);
        (assign31110_e30654, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign31110_e30656;
        locals.var_dnm_dn0 = assign31110_e30656_d_n0;
        locals.var_dnm_dn2 = assign31110_e30656_d_n2;
        locals.var_dnm_dn4 = assign31110_e30656_d_n4;
        locals.var_dnm_dn5 = assign31110_e30656_d_n5;
        locals.var_dnm_dn6 = assign31110_e30656_d_n6;
        locals.var_dnm_dn7 = assign31110_e30656_d_n7;
        locals.var_dnm_dn8 = assign31110_e30656_d_n8;
        locals.var_dnm_dn9 = assign31110_e30656_d_n9;
        locals.var_dnm_dn10 = assign31110_e30656_d_n10;
        locals.var_dnm_dn11 = assign31110_e30656_d_n11;
        locals.var_dnm_dn14 = assign31110_e30656_d_n14;

        let (assign31120_e30670, assign31120_e30670_d_n0, assign31120_e30670_d_n2, assign31120_e30670_d_n4, assign31120_e30670_d_n5, assign31120_e30670_d_n6, assign31120_e30670_d_n7, assign31120_e30670_d_n8, assign31120_e30670_d_n9, assign31120_e30670_d_n10, assign31120_e30670_d_n11, assign31120_e30670_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign31120_e30666: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign31120_e30668: f64 = (assign31120_e30666 * locals.var_dnm);
        (assign31120_e30668, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign31120_e30666 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign31120_e30670;
        locals.var_tmf0_dn0 = assign31120_e30670_d_n0;
        locals.var_tmf0_dn2 = assign31120_e30670_d_n2;
        locals.var_tmf0_dn4 = assign31120_e30670_d_n4;
        locals.var_tmf0_dn5 = assign31120_e30670_d_n5;
        locals.var_tmf0_dn6 = assign31120_e30670_d_n6;
        locals.var_tmf0_dn7 = assign31120_e30670_d_n7;
        locals.var_tmf0_dn8 = assign31120_e30670_d_n8;
        locals.var_tmf0_dn9 = assign31120_e30670_d_n9;
        locals.var_tmf0_dn10 = assign31120_e30670_d_n10;
        locals.var_tmf0_dn11 = assign31120_e30670_d_n11;
        locals.var_tmf0_dn14 = assign31120_e30670_d_n14;

        let (assign31130_e30686, assign31130_e30686_d_n0, assign31130_e30686_d_n2, assign31130_e30686_d_n4, assign31130_e30686_d_n5, assign31130_e30686_d_n6, assign31130_e30686_d_n7, assign31130_e30686_d_n8, assign31130_e30686_d_n9, assign31130_e30686_d_n10, assign31130_e30686_d_n11, assign31130_e30686_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign31130_e30680: f64 = (locals.var_t1 * locals.var_xmp);
        let assign31130_e30682: f64 = (assign31130_e30680 * locals.var_dnm);
        let assign31130_e30684: f64 = (assign31130_e30682 / locals.var_arg);
        (assign31130_e30684, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn0)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn2)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn4)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn5)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn6)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn7)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn8)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn9)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn10)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn11)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign31130_e30680 * locals.var_dnm_dn14)) * locals.var_arg) - (assign31130_e30682 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31130_e30686;
        locals.var_t0_dn0 = assign31130_e30686_d_n0;
        locals.var_t0_dn2 = assign31130_e30686_d_n2;
        locals.var_t0_dn4 = assign31130_e30686_d_n4;
        locals.var_t0_dn5 = assign31130_e30686_d_n5;
        locals.var_t0_dn6 = assign31130_e30686_d_n6;
        locals.var_t0_dn7 = assign31130_e30686_d_n7;
        locals.var_t0_dn8 = assign31130_e30686_d_n8;
        locals.var_t0_dn9 = assign31130_e30686_d_n9;
        locals.var_t0_dn10 = assign31130_e30686_d_n10;
        locals.var_t0_dn11 = assign31130_e30686_d_n11;
        locals.var_t0_dn14 = assign31130_e30686_d_n14;

        let (assign31140_e30700, assign31140_e30700_d_n0, assign31140_e30700_d_n2, assign31140_e30700_d_n4, assign31140_e30700_d_n5, assign31140_e30700_d_n6, assign31140_e30700_d_n7, assign31140_e30700_d_n8, assign31140_e30700_d_n9, assign31140_e30700_d_n10, assign31140_e30700_d_n11, assign31140_e30700_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign31140_e30696: f64 = (locals.var_vgpp - locals.var_t1);
        let assign31140_e30698: f64 = (assign31140_e30696 + locals.var_tmf0);
        (assign31140_e30698, ((locals.var_vgpp_dn0 - locals.var_t1_dn0) + locals.var_tmf0_dn0), ((locals.var_vgpp_dn2 - locals.var_t1_dn2) + locals.var_tmf0_dn2), ((locals.var_vgpp_dn4 - locals.var_t1_dn4) + locals.var_tmf0_dn4), ((locals.var_vgpp_dn5 - locals.var_t1_dn5) + locals.var_tmf0_dn5), ((locals.var_vgpp_dn6 - locals.var_t1_dn6) + locals.var_tmf0_dn6), ((locals.var_vgpp_dn7 - locals.var_t1_dn7) + locals.var_tmf0_dn7), ((locals.var_vgpp_dn8 - locals.var_t1_dn8) + locals.var_tmf0_dn8), ((locals.var_vgpp_dn9 - locals.var_t1_dn9) + locals.var_tmf0_dn9), ((locals.var_vgpp_dn10 - locals.var_t1_dn10) + locals.var_tmf0_dn10), ((locals.var_vgpp_dn11 - locals.var_t1_dn11) + locals.var_tmf0_dn11), ((locals.var_vgpp_dn14 - locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign31140_e30700;
        locals.var_vds_dn0 = assign31140_e30700_d_n0;
        locals.var_vds_dn2 = assign31140_e30700_d_n2;
        locals.var_vds_dn4 = assign31140_e30700_d_n4;
        locals.var_vds_dn5 = assign31140_e30700_d_n5;
        locals.var_vds_dn6 = assign31140_e30700_d_n6;
        locals.var_vds_dn7 = assign31140_e30700_d_n7;
        locals.var_vds_dn8 = assign31140_e30700_d_n8;
        locals.var_vds_dn9 = assign31140_e30700_d_n9;
        locals.var_vds_dn10 = assign31140_e30700_d_n10;
        locals.var_vds_dn11 = assign31140_e30700_d_n11;
        locals.var_vds_dn14 = assign31140_e30700_d_n14;

        let (assign31150_e30710, assign31150_e30710_d_n0, assign31150_e30710_d_n2, assign31150_e30710_d_n4, assign31150_e30710_d_n5, assign31150_e30710_d_n6, assign31150_e30710_d_n7, assign31150_e30710_d_n8, assign31150_e30710_d_n9, assign31150_e30710_d_n10, assign31150_e30710_d_n11, assign31150_e30710_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31150_e30710;
        locals.var_t0_dn0 = assign31150_e30710_d_n0;
        locals.var_t0_dn2 = assign31150_e30710_d_n2;
        locals.var_t0_dn4 = assign31150_e30710_d_n4;
        locals.var_t0_dn5 = assign31150_e30710_d_n5;
        locals.var_t0_dn6 = assign31150_e30710_d_n6;
        locals.var_t0_dn7 = assign31150_e30710_d_n7;
        locals.var_t0_dn8 = assign31150_e30710_d_n8;
        locals.var_t0_dn9 = assign31150_e30710_d_n9;
        locals.var_t0_dn10 = assign31150_e30710_d_n10;
        locals.var_t0_dn11 = assign31150_e30710_d_n11;
        locals.var_t0_dn14 = assign31150_e30710_d_n14;

        let (assign31160_e30721, assign31160_e30721_d_n0, assign31160_e30721_d_n2, assign31160_e30721_d_n4, assign31160_e30721_d_n5, assign31160_e30721_d_n6, assign31160_e30721_d_n7, assign31160_e30721_d_n8, assign31160_e30721_d_n9, assign31160_e30721_d_n10, assign31160_e30721_d_n11, assign31160_e30721_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 == 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign31160_e30721;
        locals.var_vds_dn0 = assign31160_e30721_d_n0;
        locals.var_vds_dn2 = assign31160_e30721_d_n2;
        locals.var_vds_dn4 = assign31160_e30721_d_n4;
        locals.var_vds_dn5 = assign31160_e30721_d_n5;
        locals.var_vds_dn6 = assign31160_e30721_d_n6;
        locals.var_vds_dn7 = assign31160_e30721_d_n7;
        locals.var_vds_dn8 = assign31160_e30721_d_n8;
        locals.var_vds_dn9 = assign31160_e30721_d_n9;
        locals.var_vds_dn10 = assign31160_e30721_d_n10;
        locals.var_vds_dn11 = assign31160_e30721_d_n11;
        locals.var_vds_dn14 = assign31160_e30721_d_n14;

        let (assign31170_e30732, assign31170_e30732_d_n0, assign31170_e30732_d_n2, assign31170_e30732_d_n4, assign31170_e30732_d_n5, assign31170_e30732_d_n6, assign31170_e30732_d_n7, assign31170_e30732_d_n8, assign31170_e30732_d_n9, assign31170_e30732_d_n10, assign31170_e30732_d_n11, assign31170_e30732_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard709 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31170_e30732;
        locals.var_t0_dn0 = assign31170_e30732_d_n0;
        locals.var_t0_dn2 = assign31170_e30732_d_n2;
        locals.var_t0_dn4 = assign31170_e30732_d_n4;
        locals.var_t0_dn5 = assign31170_e30732_d_n5;
        locals.var_t0_dn6 = assign31170_e30732_d_n6;
        locals.var_t0_dn7 = assign31170_e30732_d_n7;
        locals.var_t0_dn8 = assign31170_e30732_d_n8;
        locals.var_t0_dn9 = assign31170_e30732_d_n9;
        locals.var_t0_dn10 = assign31170_e30732_d_n10;
        locals.var_t0_dn11 = assign31170_e30732_d_n11;
        locals.var_t0_dn14 = assign31170_e30732_d_n14;

        let (assign31180_e30741, assign31180_e30741_d_n0, assign31180_e30741_d_n2, assign31180_e30741_d_n4, assign31180_e30741_d_n5, assign31180_e30741_d_n6, assign31180_e30741_d_n7, assign31180_e30741_d_n8, assign31180_e30741_d_n9, assign31180_e30741_d_n10, assign31180_e30741_d_n11, assign31180_e30741_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard690 == 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn14,)
    }
};
        locals.var_vdseff = assign31180_e30741;
        locals.var_vdseff_dn0 = assign31180_e30741_d_n0;
        locals.var_vdseff_dn2 = assign31180_e30741_d_n2;
        locals.var_vdseff_dn4 = assign31180_e30741_d_n4;
        locals.var_vdseff_dn5 = assign31180_e30741_d_n5;
        locals.var_vdseff_dn6 = assign31180_e30741_d_n6;
        locals.var_vdseff_dn7 = assign31180_e30741_d_n7;
        locals.var_vdseff_dn8 = assign31180_e30741_d_n8;
        locals.var_vdseff_dn9 = assign31180_e30741_d_n9;
        locals.var_vdseff_dn10 = assign31180_e30741_d_n10;
        locals.var_vdseff_dn11 = assign31180_e30741_d_n11;
        locals.var_vdseff_dn14 = assign31180_e30741_d_n14;

        let assign31190_e30744: f64 = if locals.var_vds <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard715 = assign31190_e30744;

        let (assign31200_e30752, assign31200_e30752_d_n0, assign31200_e30752_d_n2, assign31200_e30752_d_n4, assign31200_e30752_d_n5, assign31200_e30752_d_n6, assign31200_e30752_d_n7, assign31200_e30752_d_n8, assign31200_e30752_d_n9, assign31200_e30752_d_n10, assign31200_e30752_d_n11, assign31200_e30752_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    }
};
        locals.var_phi_sl_dep = assign31200_e30752;
        locals.var_phi_sl_dep_dn0 = assign31200_e30752_d_n0;
        locals.var_phi_sl_dep_dn2 = assign31200_e30752_d_n2;
        locals.var_phi_sl_dep_dn4 = assign31200_e30752_d_n4;
        locals.var_phi_sl_dep_dn5 = assign31200_e30752_d_n5;
        locals.var_phi_sl_dep_dn6 = assign31200_e30752_d_n6;
        locals.var_phi_sl_dep_dn7 = assign31200_e30752_d_n7;
        locals.var_phi_sl_dep_dn8 = assign31200_e30752_d_n8;
        locals.var_phi_sl_dep_dn9 = assign31200_e30752_d_n9;
        locals.var_phi_sl_dep_dn10 = assign31200_e30752_d_n10;
        locals.var_phi_sl_dep_dn11 = assign31200_e30752_d_n11;
        locals.var_phi_sl_dep_dn14 = assign31200_e30752_d_n14;

        let (assign31210_e30760, assign31210_e30760_d_n0, assign31210_e30760_d_n2, assign31210_e30760_d_n4, assign31210_e30760_d_n5, assign31210_e30760_d_n6, assign31210_e30760_d_n7, assign31210_e30760_d_n8, assign31210_e30760_d_n9, assign31210_e30760_d_n10, assign31210_e30760_d_n11, assign31210_e30760_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31210_e30760;
        locals.var_phi_bl_dep_dn0 = assign31210_e30760_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31210_e30760_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31210_e30760_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31210_e30760_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31210_e30760_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31210_e30760_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31210_e30760_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31210_e30760_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31210_e30760_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31210_e30760_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31210_e30760_d_n14;

        let (assign31220_e30768, assign31220_e30768_d_n0, assign31220_e30768_d_n2, assign31220_e30768_d_n4, assign31220_e30768_d_n5, assign31220_e30768_d_n6, assign31220_e30768_d_n7, assign31220_e30768_d_n8, assign31220_e30768_d_n9, assign31220_e30768_d_n10, assign31220_e30768_d_n11, assign31220_e30768_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31220_e30768;
        locals.var_phi_jl_dep_dn0 = assign31220_e30768_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31220_e30768_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31220_e30768_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31220_e30768_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31220_e30768_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31220_e30768_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31220_e30768_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31220_e30768_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31220_e30768_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31220_e30768_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31220_e30768_d_n14;

        let (assign31230_e30776, assign31230_e30776_d_n0, assign31230_e30776_d_n2, assign31230_e30776_d_n4, assign31230_e30776_d_n5, assign31230_e30776_d_n6, assign31230_e30776_d_n7, assign31230_e30776_d_n8, assign31230_e30776_d_n9, assign31230_e30776_d_n10, assign31230_e30776_d_n11, assign31230_e30776_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn11, locals.var_q_sub0_dn14,)
    } else {
        (locals.var_q_subl, locals.var_q_subl_dn0, locals.var_q_subl_dn2, locals.var_q_subl_dn4, locals.var_q_subl_dn5, locals.var_q_subl_dn6, locals.var_q_subl_dn7, locals.var_q_subl_dn8, locals.var_q_subl_dn9, locals.var_q_subl_dn10, locals.var_q_subl_dn11, locals.var_q_subl_dn14,)
    }
};
        locals.var_q_subl = assign31230_e30776;
        locals.var_q_subl_dn0 = assign31230_e30776_d_n0;
        locals.var_q_subl_dn2 = assign31230_e30776_d_n2;
        locals.var_q_subl_dn4 = assign31230_e30776_d_n4;
        locals.var_q_subl_dn5 = assign31230_e30776_d_n5;
        locals.var_q_subl_dn6 = assign31230_e30776_d_n6;
        locals.var_q_subl_dn7 = assign31230_e30776_d_n7;
        locals.var_q_subl_dn8 = assign31230_e30776_d_n8;
        locals.var_q_subl_dn9 = assign31230_e30776_d_n9;
        locals.var_q_subl_dn10 = assign31230_e30776_d_n10;
        locals.var_q_subl_dn11 = assign31230_e30776_d_n11;
        locals.var_q_subl_dn14 = assign31230_e30776_d_n14;

        let (assign31240_e30784, assign31240_e30784_d_n0, assign31240_e30784_d_n2, assign31240_e30784_d_n4, assign31240_e30784_d_n5, assign31240_e30784_d_n6, assign31240_e30784_d_n7, assign31240_e30784_d_n8, assign31240_e30784_d_n9, assign31240_e30784_d_n10, assign31240_e30784_d_n11, assign31240_e30784_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_q_n0__blk542, locals.var_q_n0__blk542_dn0, locals.var_q_n0__blk542_dn2, locals.var_q_n0__blk542_dn4, locals.var_q_n0__blk542_dn5, locals.var_q_n0__blk542_dn6, locals.var_q_n0__blk542_dn7, locals.var_q_n0__blk542_dn8, locals.var_q_n0__blk542_dn9, locals.var_q_n0__blk542_dn10, locals.var_q_n0__blk542_dn11, locals.var_q_n0__blk542_dn14,)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn4, locals.var_q_nl_dn5, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn8, locals.var_q_nl_dn9, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn14,)
    }
};
        locals.var_q_nl = assign31240_e30784;
        locals.var_q_nl_dn0 = assign31240_e30784_d_n0;
        locals.var_q_nl_dn2 = assign31240_e30784_d_n2;
        locals.var_q_nl_dn4 = assign31240_e30784_d_n4;
        locals.var_q_nl_dn5 = assign31240_e30784_d_n5;
        locals.var_q_nl_dn6 = assign31240_e30784_d_n6;
        locals.var_q_nl_dn7 = assign31240_e30784_d_n7;
        locals.var_q_nl_dn8 = assign31240_e30784_d_n8;
        locals.var_q_nl_dn9 = assign31240_e30784_d_n9;
        locals.var_q_nl_dn10 = assign31240_e30784_d_n10;
        locals.var_q_nl_dn11 = assign31240_e30784_d_n11;
        locals.var_q_nl_dn14 = assign31240_e30784_d_n14;

        let (assign31250_e30792, assign31250_e30792_d_n0, assign31250_e30792_d_n2, assign31250_e30792_d_n4, assign31250_e30792_d_n5, assign31250_e30792_d_n6, assign31250_e30792_d_n7, assign31250_e30792_d_n8, assign31250_e30792_d_n9, assign31250_e30792_d_n10, assign31250_e30792_d_n11, assign31250_e30792_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn14,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
        locals.var_q_bl_dep = assign31250_e30792;
        locals.var_q_bl_dep_dn0 = assign31250_e30792_d_n0;
        locals.var_q_bl_dep_dn2 = assign31250_e30792_d_n2;
        locals.var_q_bl_dep_dn4 = assign31250_e30792_d_n4;
        locals.var_q_bl_dep_dn5 = assign31250_e30792_d_n5;
        locals.var_q_bl_dep_dn6 = assign31250_e30792_d_n6;
        locals.var_q_bl_dep_dn7 = assign31250_e30792_d_n7;
        locals.var_q_bl_dep_dn8 = assign31250_e30792_d_n8;
        locals.var_q_bl_dep_dn9 = assign31250_e30792_d_n9;
        locals.var_q_bl_dep_dn10 = assign31250_e30792_d_n10;
        locals.var_q_bl_dep_dn11 = assign31250_e30792_d_n11;
        locals.var_q_bl_dep_dn14 = assign31250_e30792_d_n14;

        let (assign31260_e30800, assign31260_e30800_d_n0, assign31260_e30800_d_n2, assign31260_e30800_d_n4, assign31260_e30800_d_n5, assign31260_e30800_d_n6, assign31260_e30800_d_n7, assign31260_e30800_d_n8, assign31260_e30800_d_n9, assign31260_e30800_d_n10, assign31260_e30800_d_n11, assign31260_e30800_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn11, locals.var_q_sub0_dep_dn14,)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn11, locals.var_q_subl_dep_dn14,)
    }
};
        locals.var_q_subl_dep = assign31260_e30800;
        locals.var_q_subl_dep_dn0 = assign31260_e30800_d_n0;
        locals.var_q_subl_dep_dn2 = assign31260_e30800_d_n2;
        locals.var_q_subl_dep_dn4 = assign31260_e30800_d_n4;
        locals.var_q_subl_dep_dn5 = assign31260_e30800_d_n5;
        locals.var_q_subl_dep_dn6 = assign31260_e30800_d_n6;
        locals.var_q_subl_dep_dn7 = assign31260_e30800_d_n7;
        locals.var_q_subl_dep_dn8 = assign31260_e30800_d_n8;
        locals.var_q_subl_dep_dn9 = assign31260_e30800_d_n9;
        locals.var_q_subl_dep_dn10 = assign31260_e30800_d_n10;
        locals.var_q_subl_dep_dn11 = assign31260_e30800_d_n11;
        locals.var_q_subl_dep_dn14 = assign31260_e30800_d_n14;

        let (assign31270_e30808, assign31270_e30808_d_n0, assign31270_e30808_d_n2, assign31270_e30808_d_n4, assign31270_e30808_d_n5, assign31270_e30808_d_n6, assign31270_e30808_d_n7, assign31270_e30808_d_n8, assign31270_e30808_d_n9, assign31270_e30808_d_n10, assign31270_e30808_d_n11, assign31270_e30808_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn11, locals.var_q_s0_dep_dn14,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn14,)
    }
};
        locals.var_q_sl_dep = assign31270_e30808;
        locals.var_q_sl_dep_dn0 = assign31270_e30808_d_n0;
        locals.var_q_sl_dep_dn2 = assign31270_e30808_d_n2;
        locals.var_q_sl_dep_dn4 = assign31270_e30808_d_n4;
        locals.var_q_sl_dep_dn5 = assign31270_e30808_d_n5;
        locals.var_q_sl_dep_dn6 = assign31270_e30808_d_n6;
        locals.var_q_sl_dep_dn7 = assign31270_e30808_d_n7;
        locals.var_q_sl_dep_dn8 = assign31270_e30808_d_n8;
        locals.var_q_sl_dep_dn9 = assign31270_e30808_d_n9;
        locals.var_q_sl_dep_dn10 = assign31270_e30808_d_n10;
        locals.var_q_sl_dep_dn11 = assign31270_e30808_d_n11;
        locals.var_q_sl_dep_dn14 = assign31270_e30808_d_n14;

        let (assign31280_e30816, assign31280_e30816_d_n0, assign31280_e30816_d_n2, assign31280_e30816_d_n4, assign31280_e30816_d_n5, assign31280_e30816_d_n6, assign31280_e30816_d_n7, assign31280_e30816_d_n8, assign31280_e30816_d_n9, assign31280_e30816_d_n10, assign31280_e30816_d_n11, assign31280_e30816_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_q_n0_cur, locals.var_q_n0_cur_dn0, locals.var_q_n0_cur_dn2, locals.var_q_n0_cur_dn4, locals.var_q_n0_cur_dn5, locals.var_q_n0_cur_dn6, locals.var_q_n0_cur_dn7, locals.var_q_n0_cur_dn8, locals.var_q_n0_cur_dn9, locals.var_q_n0_cur_dn10, locals.var_q_n0_cur_dn11, locals.var_q_n0_cur_dn14,)
    } else {
        (locals.var_q_nl_cur, locals.var_q_nl_cur_dn0, locals.var_q_nl_cur_dn2, locals.var_q_nl_cur_dn4, locals.var_q_nl_cur_dn5, locals.var_q_nl_cur_dn6, locals.var_q_nl_cur_dn7, locals.var_q_nl_cur_dn8, locals.var_q_nl_cur_dn9, locals.var_q_nl_cur_dn10, locals.var_q_nl_cur_dn11, locals.var_q_nl_cur_dn14,)
    }
};
        locals.var_q_nl_cur = assign31280_e30816;
        locals.var_q_nl_cur_dn0 = assign31280_e30816_d_n0;
        locals.var_q_nl_cur_dn2 = assign31280_e30816_d_n2;
        locals.var_q_nl_cur_dn4 = assign31280_e30816_d_n4;
        locals.var_q_nl_cur_dn5 = assign31280_e30816_d_n5;
        locals.var_q_nl_cur_dn6 = assign31280_e30816_d_n6;
        locals.var_q_nl_cur_dn7 = assign31280_e30816_d_n7;
        locals.var_q_nl_cur_dn8 = assign31280_e30816_d_n8;
        locals.var_q_nl_cur_dn9 = assign31280_e30816_d_n9;
        locals.var_q_nl_cur_dn10 = assign31280_e30816_d_n10;
        locals.var_q_nl_cur_dn11 = assign31280_e30816_d_n11;
        locals.var_q_nl_cur_dn14 = assign31280_e30816_d_n14;

    }

    pub(super) fn stamp_transient_block_91(
        locals: &mut StampLocals,
    ) {
        let (assign31290_e30838, assign31290_e30838_d_n0, assign31290_e30838_d_n2, assign31290_e30838_d_n4, assign31290_e30838_d_n5, assign31290_e30838_d_n6, assign31290_e30838_d_n7, assign31290_e30838_d_n8, assign31290_e30838_d_n9, assign31290_e30838_d_n10, assign31290_e30838_d_n11, assign31290_e30838_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) {
        let assign31290_e30825: f64 = (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc);
        let assign31290_e30828: f64 = (locals.var_ef_nsubc + locals.var_uc_ndepm);
        let assign31290_e30829: f64 = (assign31290_e30825 / assign31290_e30828);
        let assign31290_e30832: f64 = (locals.var_vds - locals.var_vbscl__blk439);
        let assign31290_e30834: f64 = (assign31290_e30832 + locals.var_vbi_dep);
        let assign31290_e30835: f64 = (assign31290_e30829 * assign31290_e30834);
        let assign31290_e30836: f64 = (assign31290_e30835).sqrt();
        (assign31290_e30836, ((((((((locals.var_c_2esipq_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn0)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn0 + locals.var_uc_ndepm_dn0))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn0 - locals.var_vbscl__blk439_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign31290_e30836)), ((((((((locals.var_c_2esipq_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn2)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn2 + locals.var_uc_ndepm_dn2))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn2 - locals.var_vbscl__blk439_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign31290_e30836)), ((((((((locals.var_c_2esipq_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn4)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn4 + locals.var_uc_ndepm_dn4))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn4 - locals.var_vbscl__blk439_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign31290_e30836)), ((((((((locals.var_c_2esipq_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn5)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn5 + locals.var_uc_ndepm_dn5))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn5 - locals.var_vbscl__blk439_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign31290_e30836)), ((((((((locals.var_c_2esipq_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn6)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn6 + locals.var_uc_ndepm_dn6))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn6 - locals.var_vbscl__blk439_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign31290_e30836)), ((((((((locals.var_c_2esipq_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn7)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn7 + locals.var_uc_ndepm_dn7))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn7 - locals.var_vbscl__blk439_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign31290_e30836)), ((((((((locals.var_c_2esipq_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn8)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn8 + locals.var_uc_ndepm_dn8))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn8 - locals.var_vbscl__blk439_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign31290_e30836)), ((((((((locals.var_c_2esipq_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn9)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn9 + locals.var_uc_ndepm_dn9))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn9 - locals.var_vbscl__blk439_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign31290_e30836)), ((((((((locals.var_c_2esipq_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn10)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn10 + locals.var_uc_ndepm_dn10))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn10 - locals.var_vbscl__blk439_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign31290_e30836)), ((((((((locals.var_c_2esipq_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn11)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn11 + locals.var_uc_ndepm_dn11))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn11 - locals.var_vbscl__blk439_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign31290_e30836)), ((((((((locals.var_c_2esipq_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn14)) * assign31290_e30828) - (assign31290_e30825 * (locals.var_ef_nsubc_dn14 + locals.var_uc_ndepm_dn14))) / (assign31290_e30828 * assign31290_e30828)) * assign31290_e30834) + (assign31290_e30829 * ((locals.var_vds_dn14 - locals.var_vbscl__blk439_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign31290_e30836)),)
    } else {
        (locals.var_w_bsubl, locals.var_w_bsubl_dn0, locals.var_w_bsubl_dn2, locals.var_w_bsubl_dn4, locals.var_w_bsubl_dn5, locals.var_w_bsubl_dn6, locals.var_w_bsubl_dn7, locals.var_w_bsubl_dn8, locals.var_w_bsubl_dn9, locals.var_w_bsubl_dn10, locals.var_w_bsubl_dn11, locals.var_w_bsubl_dn14,)
    }
};
        locals.var_w_bsubl = assign31290_e30838;
        locals.var_w_bsubl_dn0 = assign31290_e30838_d_n0;
        locals.var_w_bsubl_dn2 = assign31290_e30838_d_n2;
        locals.var_w_bsubl_dn4 = assign31290_e30838_d_n4;
        locals.var_w_bsubl_dn5 = assign31290_e30838_d_n5;
        locals.var_w_bsubl_dn6 = assign31290_e30838_d_n6;
        locals.var_w_bsubl_dn7 = assign31290_e30838_d_n7;
        locals.var_w_bsubl_dn8 = assign31290_e30838_d_n8;
        locals.var_w_bsubl_dn9 = assign31290_e30838_d_n9;
        locals.var_w_bsubl_dn10 = assign31290_e30838_d_n10;
        locals.var_w_bsubl_dn11 = assign31290_e30838_d_n11;
        locals.var_w_bsubl_dn14 = assign31290_e30838_d_n14;

        let assign31300_e30841: f64 = if locals.var_w_bsubl > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard716 = assign31300_e30841;

        let (assign31310_e30852, assign31310_e30852_d_n0, assign31310_e30852_d_n2, assign31310_e30852_d_n4, assign31310_e30852_d_n5, assign31310_e30852_d_n6, assign31310_e30852_d_n7, assign31310_e30852_d_n8, assign31310_e30852_d_n9, assign31310_e30852_d_n10, assign31310_e30852_d_n11, assign31310_e30852_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
        locals.var_vgp0 = assign31310_e30852;
        locals.var_vgp0_dn0 = assign31310_e30852_d_n0;
        locals.var_vgp0_dn2 = assign31310_e30852_d_n2;
        locals.var_vgp0_dn4 = assign31310_e30852_d_n4;
        locals.var_vgp0_dn5 = assign31310_e30852_d_n5;
        locals.var_vgp0_dn6 = assign31310_e30852_d_n6;
        locals.var_vgp0_dn7 = assign31310_e30852_d_n7;
        locals.var_vgp0_dn8 = assign31310_e30852_d_n8;
        locals.var_vgp0_dn9 = assign31310_e30852_d_n9;
        locals.var_vgp0_dn10 = assign31310_e30852_d_n10;
        locals.var_vgp0_dn11 = assign31310_e30852_d_n11;
        locals.var_vgp0_dn14 = assign31310_e30852_d_n14;

        let (assign31320_e30863, assign31320_e30863_d_n0, assign31320_e30863_d_n2, assign31320_e30863_d_n4, assign31320_e30863_d_n5, assign31320_e30863_d_n6, assign31320_e30863_d_n7, assign31320_e30863_d_n8, assign31320_e30863_d_n9, assign31320_e30863_d_n10, assign31320_e30863_d_n11, assign31320_e30863_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign31320_e30863;
        locals.var_w_bl_dn0 = assign31320_e30863_d_n0;
        locals.var_w_bl_dn2 = assign31320_e30863_d_n2;
        locals.var_w_bl_dn4 = assign31320_e30863_d_n4;
        locals.var_w_bl_dn5 = assign31320_e30863_d_n5;
        locals.var_w_bl_dn6 = assign31320_e30863_d_n6;
        locals.var_w_bl_dn7 = assign31320_e30863_d_n7;
        locals.var_w_bl_dn8 = assign31320_e30863_d_n8;
        locals.var_w_bl_dn9 = assign31320_e30863_d_n9;
        locals.var_w_bl_dn10 = assign31320_e30863_d_n10;
        locals.var_w_bl_dn11 = assign31320_e30863_d_n11;
        locals.var_w_bl_dn14 = assign31320_e30863_d_n14;

        let (assign31330_e30874, assign31330_e30874_d_n0, assign31330_e30874_d_n2, assign31330_e30874_d_n4, assign31330_e30874_d_n5, assign31330_e30874_d_n6, assign31330_e30874_d_n7, assign31330_e30874_d_n8, assign31330_e30874_d_n9, assign31330_e30874_d_n10, assign31330_e30874_d_n11, assign31330_e30874_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31330_e30874;
        locals.var_phi_bl_dep_dn0 = assign31330_e30874_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31330_e30874_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31330_e30874_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31330_e30874_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31330_e30874_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31330_e30874_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31330_e30874_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31330_e30874_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31330_e30874_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31330_e30874_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31330_e30874_d_n14;

        let (assign31340_e30885, assign31340_e30885_d_n0, assign31340_e30885_d_n2, assign31340_e30885_d_n4, assign31340_e30885_d_n5, assign31340_e30885_d_n6, assign31340_e30885_d_n7, assign31340_e30885_d_n8, assign31340_e30885_d_n9, assign31340_e30885_d_n10, assign31340_e30885_d_n11, assign31340_e30885_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vds_maxbl, locals.var_vds_maxbl_dn0, locals.var_vds_maxbl_dn2, locals.var_vds_maxbl_dn4, locals.var_vds_maxbl_dn5, locals.var_vds_maxbl_dn6, locals.var_vds_maxbl_dn7, locals.var_vds_maxbl_dn8, locals.var_vds_maxbl_dn9, locals.var_vds_maxbl_dn10, locals.var_vds_maxbl_dn11, locals.var_vds_maxbl_dn14,)
    }
};
        locals.var_vds_maxbl = assign31340_e30885;
        locals.var_vds_maxbl_dn0 = assign31340_e30885_d_n0;
        locals.var_vds_maxbl_dn2 = assign31340_e30885_d_n2;
        locals.var_vds_maxbl_dn4 = assign31340_e30885_d_n4;
        locals.var_vds_maxbl_dn5 = assign31340_e30885_d_n5;
        locals.var_vds_maxbl_dn6 = assign31340_e30885_d_n6;
        locals.var_vds_maxbl_dn7 = assign31340_e30885_d_n7;
        locals.var_vds_maxbl_dn8 = assign31340_e30885_d_n8;
        locals.var_vds_maxbl_dn9 = assign31340_e30885_d_n9;
        locals.var_vds_maxbl_dn10 = assign31340_e30885_d_n10;
        locals.var_vds_maxbl_dn11 = assign31340_e30885_d_n11;
        locals.var_vds_maxbl_dn14 = assign31340_e30885_d_n14;

        let (assign31350_e30902, assign31350_e30902_d_n0, assign31350_e30902_d_n2, assign31350_e30902_d_n4, assign31350_e30902_d_n5, assign31350_e30902_d_n6, assign31350_e30902_d_n7, assign31350_e30902_d_n8, assign31350_e30902_d_n9, assign31350_e30902_d_n10, assign31350_e30902_d_n11, assign31350_e30902_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31350_e30897: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl);
        let assign31350_e30899: f64 = (assign31350_e30897 * locals.var_w_bl);
        let assign31350_e30900: f64 = (locals.var_phi_bl_dep - assign31350_e30899);
        (assign31350_e30900, (locals.var_phi_bl_dep_dn0 - ((((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn0)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn0))), (locals.var_phi_bl_dep_dn2 - ((((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn2)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn2))), (locals.var_phi_bl_dep_dn4 - ((((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn4)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn4))), (locals.var_phi_bl_dep_dn5 - ((((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn5)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn5))), (locals.var_phi_bl_dep_dn6 - ((((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn6)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn6))), (locals.var_phi_bl_dep_dn7 - ((((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn7)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn7))), (locals.var_phi_bl_dep_dn8 - ((((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn8)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn8))), (locals.var_phi_bl_dep_dn9 - ((((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn9)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn9))), (locals.var_phi_bl_dep_dn10 - ((((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn10)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn10))), (locals.var_phi_bl_dep_dn11 - ((((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn11)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn11))), (locals.var_phi_bl_dep_dn14 - ((((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn14)) * locals.var_w_bl) + (assign31350_e30897 * locals.var_w_bl_dn14))),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31350_e30902;
        locals.var_phi_jl_dep_dn0 = assign31350_e30902_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31350_e30902_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31350_e30902_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31350_e30902_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31350_e30902_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31350_e30902_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31350_e30902_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31350_e30902_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31350_e30902_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31350_e30902_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31350_e30902_d_n14;

        let (assign31360_e30913,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_vgp0,)
    } else {
        (locals.var_vgp0old,)
    }
};
        locals.var_vgp0old = assign31360_e30913;

        let (assign31370_e30924,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_phi_jl_dep,)
    } else {
        (locals.var_phi_jl_dep_old,)
    }
};
        locals.var_phi_jl_dep_old = assign31370_e30924;

        let (assign31380_e30937, assign31380_e30937_d_n0, assign31380_e30937_d_n2, assign31380_e30937_d_n4, assign31380_e30937_d_n5, assign31380_e30937_d_n6, assign31380_e30937_d_n7, assign31380_e30937_d_n8, assign31380_e30937_d_n9, assign31380_e30937_d_n10, assign31380_e30937_d_n11, assign31380_e30937_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31380_e30935: f64 = (locals.var_w_bl * locals.var_q_ndepm);
        (assign31380_e30935, ((locals.var_w_bl_dn0 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn0)), ((locals.var_w_bl_dn2 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn2)), ((locals.var_w_bl_dn4 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn4)), ((locals.var_w_bl_dn5 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn5)), ((locals.var_w_bl_dn6 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn6)), ((locals.var_w_bl_dn7 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn7)), ((locals.var_w_bl_dn8 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn8)), ((locals.var_w_bl_dn9 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn9)), ((locals.var_w_bl_dn10 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn10)), ((locals.var_w_bl_dn11 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn11)), ((locals.var_w_bl_dn14 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
        locals.var_q_bl_dep = assign31380_e30937;
        locals.var_q_bl_dep_dn0 = assign31380_e30937_d_n0;
        locals.var_q_bl_dep_dn2 = assign31380_e30937_d_n2;
        locals.var_q_bl_dep_dn4 = assign31380_e30937_d_n4;
        locals.var_q_bl_dep_dn5 = assign31380_e30937_d_n5;
        locals.var_q_bl_dep_dn6 = assign31380_e30937_d_n6;
        locals.var_q_bl_dep_dn7 = assign31380_e30937_d_n7;
        locals.var_q_bl_dep_dn8 = assign31380_e30937_d_n8;
        locals.var_q_bl_dep_dn9 = assign31380_e30937_d_n9;
        locals.var_q_bl_dep_dn10 = assign31380_e30937_d_n10;
        locals.var_q_bl_dep_dn11 = assign31380_e30937_d_n11;
        locals.var_q_bl_dep_dn14 = assign31380_e30937_d_n14;

        let (assign31390_e30948,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign31390_e30948;

    }

    pub(super) fn stamp_transient_block_92(
        locals: &mut StampLocals,
    ) {
        let mut assign31400_loop_guard: usize = 0;
        while {
            let assign31400_cond_e30960: f64 = (150.0 + 1.0);
            let assign31400_cond_e30962: f64 = if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_lp_s0 <= assign31400_cond_e30960)) { 1.0 } else { 0.0 };
            assign31400_cond_e30962 != 0.0
        } {
            assign31400_loop_guard += 1;
            assert!(assign31400_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign31400_body0_e30978, assign31400_body0_e30978_d_n0, assign31400_body0_e30978_d_n2, assign31400_body0_e30978_d_n4, assign31400_body0_e30978_d_n5, assign31400_body0_e30978_d_n6, assign31400_body0_e30978_d_n7, assign31400_body0_e30978_d_n8, assign31400_body0_e30978_d_n9, assign31400_body0_e30978_d_n10, assign31400_body0_e30978_d_n11, assign31400_body0_e30978_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body0_e30974: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        let assign31400_body0_e30975: f64 = (locals.var_c_2esipq_ndepm * assign31400_body0_e30974);
        let assign31400_body0_e30976: f64 = (assign31400_body0_e30975).sqrt();
        (assign31400_body0_e30976, (((locals.var_c_2esipq_ndepm_dn0 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0))) / (2.0 * assign31400_body0_e30976)), (((locals.var_c_2esipq_ndepm_dn2 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2))) / (2.0 * assign31400_body0_e30976)), (((locals.var_c_2esipq_ndepm_dn4 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4))) / (2.0 * assign31400_body0_e30976)), (((locals.var_c_2esipq_ndepm_dn5 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5))) / (2.0 * assign31400_body0_e30976)), (((locals.var_c_2esipq_ndepm_dn6 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6))) / (2.0 * assign31400_body0_e30976)), (((locals.var_c_2esipq_ndepm_dn7 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7))) / (2.0 * assign31400_body0_e30976)), (((locals.var_c_2esipq_ndepm_dn8 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8))) / (2.0 * assign31400_body0_e30976)), (((locals.var_c_2esipq_ndepm_dn9 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9))) / (2.0 * assign31400_body0_e30976)), (((locals.var_c_2esipq_ndepm_dn10 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10))) / (2.0 * assign31400_body0_e30976)), (((locals.var_c_2esipq_ndepm_dn11 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_jl_dep_dn11))) / (2.0 * assign31400_body0_e30976)), (((locals.var_c_2esipq_ndepm_dn14 * assign31400_body0_e30974) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_jl_dep_dn14))) / (2.0 * assign31400_body0_e30976)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
            locals.var_w_bl = assign31400_body0_e30978;
            locals.var_w_bl_dn0 = assign31400_body0_e30978_d_n0;
            locals.var_w_bl_dn2 = assign31400_body0_e30978_d_n2;
            locals.var_w_bl_dn4 = assign31400_body0_e30978_d_n4;
            locals.var_w_bl_dn5 = assign31400_body0_e30978_d_n5;
            locals.var_w_bl_dn6 = assign31400_body0_e30978_d_n6;
            locals.var_w_bl_dn7 = assign31400_body0_e30978_d_n7;
            locals.var_w_bl_dn8 = assign31400_body0_e30978_d_n8;
            locals.var_w_bl_dn9 = assign31400_body0_e30978_d_n9;
            locals.var_w_bl_dn10 = assign31400_body0_e30978_d_n10;
            locals.var_w_bl_dn11 = assign31400_body0_e30978_d_n11;
            locals.var_w_bl_dn14 = assign31400_body0_e30978_d_n14;
            let assign31400_body1_e30982: f64 = (locals.var_uc_depthn - 1e-8);
            let assign31400_body1_e30987: f64 = if ((locals.var_w_bl > assign31400_body1_e30982) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard717 = assign31400_body1_e30987;
            let (assign31400_body2_e31004, assign31400_body2_e31004_d_n0, assign31400_body2_e31004_d_n2, assign31400_body2_e31004_d_n4, assign31400_body2_e31004_d_n5, assign31400_body2_e31004_d_n6, assign31400_body2_e31004_d_n7, assign31400_body2_e31004_d_n8, assign31400_body2_e31004_d_n9, assign31400_body2_e31004_d_n10, assign31400_body2_e31004_d_n11, assign31400_body2_e31004_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body2_e31000: f64 = (locals.var_w_bl - locals.var_uc_depthn);
        let assign31400_body2_e31002: f64 = (assign31400_body2_e31000 + 1e-8);
        (assign31400_body2_e31002, (locals.var_w_bl_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_bl_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_bl_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_bl_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_bl_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_bl_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_bl_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_bl_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_bl_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_bl_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_bl_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign31400_body2_e31004;
            locals.var_tmf1_dn0 = assign31400_body2_e31004_d_n0;
            locals.var_tmf1_dn2 = assign31400_body2_e31004_d_n2;
            locals.var_tmf1_dn4 = assign31400_body2_e31004_d_n4;
            locals.var_tmf1_dn5 = assign31400_body2_e31004_d_n5;
            locals.var_tmf1_dn6 = assign31400_body2_e31004_d_n6;
            locals.var_tmf1_dn7 = assign31400_body2_e31004_d_n7;
            locals.var_tmf1_dn8 = assign31400_body2_e31004_d_n8;
            locals.var_tmf1_dn9 = assign31400_body2_e31004_d_n9;
            locals.var_tmf1_dn10 = assign31400_body2_e31004_d_n10;
            locals.var_tmf1_dn11 = assign31400_body2_e31004_d_n11;
            locals.var_tmf1_dn14 = assign31400_body2_e31004_d_n14;
            let (assign31400_body3_e31019, assign31400_body3_e31019_d_n0, assign31400_body3_e31019_d_n2, assign31400_body3_e31019_d_n4, assign31400_body3_e31019_d_n5, assign31400_body3_e31019_d_n6, assign31400_body3_e31019_d_n7, assign31400_body3_e31019_d_n8, assign31400_body3_e31019_d_n9, assign31400_body3_e31019_d_n10, assign31400_body3_e31019_d_n11, assign31400_body3_e31019_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body3_e31017: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign31400_body3_e31017, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
            locals.var_x2 = assign31400_body3_e31019;
            locals.var_x2_dn0 = assign31400_body3_e31019_d_n0;
            locals.var_x2_dn2 = assign31400_body3_e31019_d_n2;
            locals.var_x2_dn4 = assign31400_body3_e31019_d_n4;
            locals.var_x2_dn5 = assign31400_body3_e31019_d_n5;
            locals.var_x2_dn6 = assign31400_body3_e31019_d_n6;
            locals.var_x2_dn7 = assign31400_body3_e31019_d_n7;
            locals.var_x2_dn8 = assign31400_body3_e31019_d_n8;
            locals.var_x2_dn9 = assign31400_body3_e31019_d_n9;
            locals.var_x2_dn10 = assign31400_body3_e31019_d_n10;
            locals.var_x2_dn11 = assign31400_body3_e31019_d_n11;
            locals.var_x2_dn14 = assign31400_body3_e31019_d_n14;
            let (assign31400_body4_e31034, assign31400_body4_e31034_d_n0, assign31400_body4_e31034_d_n2, assign31400_body4_e31034_d_n4, assign31400_body4_e31034_d_n5, assign31400_body4_e31034_d_n6, assign31400_body4_e31034_d_n7, assign31400_body4_e31034_d_n8, assign31400_body4_e31034_d_n9, assign31400_body4_e31034_d_n10, assign31400_body4_e31034_d_n11, assign31400_body4_e31034_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body4_e31032: f64 = (1e-8 * 1e-8);
        (assign31400_body4_e31032, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
            locals.var_xmax2 = assign31400_body4_e31034;
            locals.var_xmax2_dn0 = assign31400_body4_e31034_d_n0;
            locals.var_xmax2_dn2 = assign31400_body4_e31034_d_n2;
            locals.var_xmax2_dn4 = assign31400_body4_e31034_d_n4;
            locals.var_xmax2_dn5 = assign31400_body4_e31034_d_n5;
            locals.var_xmax2_dn6 = assign31400_body4_e31034_d_n6;
            locals.var_xmax2_dn7 = assign31400_body4_e31034_d_n7;
            locals.var_xmax2_dn8 = assign31400_body4_e31034_d_n8;
            locals.var_xmax2_dn9 = assign31400_body4_e31034_d_n9;
            locals.var_xmax2_dn10 = assign31400_body4_e31034_d_n10;
            locals.var_xmax2_dn11 = assign31400_body4_e31034_d_n11;
            locals.var_xmax2_dn14 = assign31400_body4_e31034_d_n14;
            let (assign31400_body5_e31047, assign31400_body5_e31047_d_n0, assign31400_body5_e31047_d_n2, assign31400_body5_e31047_d_n4, assign31400_body5_e31047_d_n5, assign31400_body5_e31047_d_n6, assign31400_body5_e31047_d_n7, assign31400_body5_e31047_d_n8, assign31400_body5_e31047_d_n9, assign31400_body5_e31047_d_n10, assign31400_body5_e31047_d_n11, assign31400_body5_e31047_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31400_body5_e31047;
            locals.var_xp_dn0 = assign31400_body5_e31047_d_n0;
            locals.var_xp_dn2 = assign31400_body5_e31047_d_n2;
            locals.var_xp_dn4 = assign31400_body5_e31047_d_n4;
            locals.var_xp_dn5 = assign31400_body5_e31047_d_n5;
            locals.var_xp_dn6 = assign31400_body5_e31047_d_n6;
            locals.var_xp_dn7 = assign31400_body5_e31047_d_n7;
            locals.var_xp_dn8 = assign31400_body5_e31047_d_n8;
            locals.var_xp_dn9 = assign31400_body5_e31047_d_n9;
            locals.var_xp_dn10 = assign31400_body5_e31047_d_n10;
            locals.var_xp_dn11 = assign31400_body5_e31047_d_n11;
            locals.var_xp_dn14 = assign31400_body5_e31047_d_n14;
            let (assign31400_body6_e31060, assign31400_body6_e31060_d_n0, assign31400_body6_e31060_d_n2, assign31400_body6_e31060_d_n4, assign31400_body6_e31060_d_n5, assign31400_body6_e31060_d_n6, assign31400_body6_e31060_d_n7, assign31400_body6_e31060_d_n8, assign31400_body6_e31060_d_n9, assign31400_body6_e31060_d_n10, assign31400_body6_e31060_d_n11, assign31400_body6_e31060_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31400_body6_e31060;
            locals.var_xmp_dn0 = assign31400_body6_e31060_d_n0;
            locals.var_xmp_dn2 = assign31400_body6_e31060_d_n2;
            locals.var_xmp_dn4 = assign31400_body6_e31060_d_n4;
            locals.var_xmp_dn5 = assign31400_body6_e31060_d_n5;
            locals.var_xmp_dn6 = assign31400_body6_e31060_d_n6;
            locals.var_xmp_dn7 = assign31400_body6_e31060_d_n7;
            locals.var_xmp_dn8 = assign31400_body6_e31060_d_n8;
            locals.var_xmp_dn9 = assign31400_body6_e31060_d_n9;
            locals.var_xmp_dn10 = assign31400_body6_e31060_d_n10;
            locals.var_xmp_dn11 = assign31400_body6_e31060_d_n11;
            locals.var_xmp_dn14 = assign31400_body6_e31060_d_n14;
            let (assign31400_body7_e31073,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign31400_body7_e31073;
            let (assign31400_body8_e31086,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31400_body8_e31086;
            let (assign31400_body9_e31099, assign31400_body9_e31099_d_n0, assign31400_body9_e31099_d_n2, assign31400_body9_e31099_d_n4, assign31400_body9_e31099_d_n5, assign31400_body9_e31099_d_n6, assign31400_body9_e31099_d_n7, assign31400_body9_e31099_d_n8, assign31400_body9_e31099_d_n9, assign31400_body9_e31099_d_n10, assign31400_body9_e31099_d_n11, assign31400_body9_e31099_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign31400_body9_e31099;
            locals.var_arg_dn0 = assign31400_body9_e31099_d_n0;
            locals.var_arg_dn2 = assign31400_body9_e31099_d_n2;
            locals.var_arg_dn4 = assign31400_body9_e31099_d_n4;
            locals.var_arg_dn5 = assign31400_body9_e31099_d_n5;
            locals.var_arg_dn6 = assign31400_body9_e31099_d_n6;
            locals.var_arg_dn7 = assign31400_body9_e31099_d_n7;
            locals.var_arg_dn8 = assign31400_body9_e31099_d_n8;
            locals.var_arg_dn9 = assign31400_body9_e31099_d_n9;
            locals.var_arg_dn10 = assign31400_body9_e31099_d_n10;
            locals.var_arg_dn11 = assign31400_body9_e31099_d_n11;
            locals.var_arg_dn14 = assign31400_body9_e31099_d_n14;
            let (assign31400_body10_e31112, assign31400_body10_e31112_d_n0, assign31400_body10_e31112_d_n2, assign31400_body10_e31112_d_n4, assign31400_body10_e31112_d_n5, assign31400_body10_e31112_d_n6, assign31400_body10_e31112_d_n7, assign31400_body10_e31112_d_n8, assign31400_body10_e31112_d_n9, assign31400_body10_e31112_d_n10, assign31400_body10_e31112_d_n11, assign31400_body10_e31112_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31400_body10_e31112;
            locals.var_dnm_dn0 = assign31400_body10_e31112_d_n0;
            locals.var_dnm_dn2 = assign31400_body10_e31112_d_n2;
            locals.var_dnm_dn4 = assign31400_body10_e31112_d_n4;
            locals.var_dnm_dn5 = assign31400_body10_e31112_d_n5;
            locals.var_dnm_dn6 = assign31400_body10_e31112_d_n6;
            locals.var_dnm_dn7 = assign31400_body10_e31112_d_n7;
            locals.var_dnm_dn8 = assign31400_body10_e31112_d_n8;
            locals.var_dnm_dn9 = assign31400_body10_e31112_d_n9;
            locals.var_dnm_dn10 = assign31400_body10_e31112_d_n10;
            locals.var_dnm_dn11 = assign31400_body10_e31112_d_n11;
            locals.var_dnm_dn14 = assign31400_body10_e31112_d_n14;
            let (assign31400_body11_e31127, assign31400_body11_e31127_d_n0, assign31400_body11_e31127_d_n2, assign31400_body11_e31127_d_n4, assign31400_body11_e31127_d_n5, assign31400_body11_e31127_d_n6, assign31400_body11_e31127_d_n7, assign31400_body11_e31127_d_n8, assign31400_body11_e31127_d_n9, assign31400_body11_e31127_d_n10, assign31400_body11_e31127_d_n11, assign31400_body11_e31127_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body11_e31125: f64 = (locals.var_xp * locals.var_x2);
        (assign31400_body11_e31125, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31400_body11_e31127;
            locals.var_xp_dn0 = assign31400_body11_e31127_d_n0;
            locals.var_xp_dn2 = assign31400_body11_e31127_d_n2;
            locals.var_xp_dn4 = assign31400_body11_e31127_d_n4;
            locals.var_xp_dn5 = assign31400_body11_e31127_d_n5;
            locals.var_xp_dn6 = assign31400_body11_e31127_d_n6;
            locals.var_xp_dn7 = assign31400_body11_e31127_d_n7;
            locals.var_xp_dn8 = assign31400_body11_e31127_d_n8;
            locals.var_xp_dn9 = assign31400_body11_e31127_d_n9;
            locals.var_xp_dn10 = assign31400_body11_e31127_d_n10;
            locals.var_xp_dn11 = assign31400_body11_e31127_d_n11;
            locals.var_xp_dn14 = assign31400_body11_e31127_d_n14;
            let (assign31400_body12_e31142, assign31400_body12_e31142_d_n0, assign31400_body12_e31142_d_n2, assign31400_body12_e31142_d_n4, assign31400_body12_e31142_d_n5, assign31400_body12_e31142_d_n6, assign31400_body12_e31142_d_n7, assign31400_body12_e31142_d_n8, assign31400_body12_e31142_d_n9, assign31400_body12_e31142_d_n10, assign31400_body12_e31142_d_n11, assign31400_body12_e31142_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body12_e31140: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign31400_body12_e31140, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31400_body12_e31142;
            locals.var_xmp_dn0 = assign31400_body12_e31142_d_n0;
            locals.var_xmp_dn2 = assign31400_body12_e31142_d_n2;
            locals.var_xmp_dn4 = assign31400_body12_e31142_d_n4;
            locals.var_xmp_dn5 = assign31400_body12_e31142_d_n5;
            locals.var_xmp_dn6 = assign31400_body12_e31142_d_n6;
            locals.var_xmp_dn7 = assign31400_body12_e31142_d_n7;
            locals.var_xmp_dn8 = assign31400_body12_e31142_d_n8;
            locals.var_xmp_dn9 = assign31400_body12_e31142_d_n9;
            locals.var_xmp_dn10 = assign31400_body12_e31142_d_n10;
            locals.var_xmp_dn11 = assign31400_body12_e31142_d_n11;
            locals.var_xmp_dn14 = assign31400_body12_e31142_d_n14;
            let (assign31400_body13_e31157, assign31400_body13_e31157_d_n0, assign31400_body13_e31157_d_n2, assign31400_body13_e31157_d_n4, assign31400_body13_e31157_d_n5, assign31400_body13_e31157_d_n6, assign31400_body13_e31157_d_n7, assign31400_body13_e31157_d_n8, assign31400_body13_e31157_d_n9, assign31400_body13_e31157_d_n10, assign31400_body13_e31157_d_n11, assign31400_body13_e31157_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body13_e31155: f64 = (locals.var_xp * locals.var_x2);
        (assign31400_body13_e31155, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31400_body13_e31157;
            locals.var_xp_dn0 = assign31400_body13_e31157_d_n0;
            locals.var_xp_dn2 = assign31400_body13_e31157_d_n2;
            locals.var_xp_dn4 = assign31400_body13_e31157_d_n4;
            locals.var_xp_dn5 = assign31400_body13_e31157_d_n5;
            locals.var_xp_dn6 = assign31400_body13_e31157_d_n6;
            locals.var_xp_dn7 = assign31400_body13_e31157_d_n7;
            locals.var_xp_dn8 = assign31400_body13_e31157_d_n8;
            locals.var_xp_dn9 = assign31400_body13_e31157_d_n9;
            locals.var_xp_dn10 = assign31400_body13_e31157_d_n10;
            locals.var_xp_dn11 = assign31400_body13_e31157_d_n11;
            locals.var_xp_dn14 = assign31400_body13_e31157_d_n14;
            let (assign31400_body14_e31172, assign31400_body14_e31172_d_n0, assign31400_body14_e31172_d_n2, assign31400_body14_e31172_d_n4, assign31400_body14_e31172_d_n5, assign31400_body14_e31172_d_n6, assign31400_body14_e31172_d_n7, assign31400_body14_e31172_d_n8, assign31400_body14_e31172_d_n9, assign31400_body14_e31172_d_n10, assign31400_body14_e31172_d_n11, assign31400_body14_e31172_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body14_e31170: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign31400_body14_e31170, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31400_body14_e31172;
            locals.var_xmp_dn0 = assign31400_body14_e31172_d_n0;
            locals.var_xmp_dn2 = assign31400_body14_e31172_d_n2;
            locals.var_xmp_dn4 = assign31400_body14_e31172_d_n4;
            locals.var_xmp_dn5 = assign31400_body14_e31172_d_n5;
            locals.var_xmp_dn6 = assign31400_body14_e31172_d_n6;
            locals.var_xmp_dn7 = assign31400_body14_e31172_d_n7;
            locals.var_xmp_dn8 = assign31400_body14_e31172_d_n8;
            locals.var_xmp_dn9 = assign31400_body14_e31172_d_n9;
            locals.var_xmp_dn10 = assign31400_body14_e31172_d_n10;
            locals.var_xmp_dn11 = assign31400_body14_e31172_d_n11;
            locals.var_xmp_dn14 = assign31400_body14_e31172_d_n14;
            let (assign31400_body15_e31187, assign31400_body15_e31187_d_n0, assign31400_body15_e31187_d_n2, assign31400_body15_e31187_d_n4, assign31400_body15_e31187_d_n5, assign31400_body15_e31187_d_n6, assign31400_body15_e31187_d_n7, assign31400_body15_e31187_d_n8, assign31400_body15_e31187_d_n9, assign31400_body15_e31187_d_n10, assign31400_body15_e31187_d_n11, assign31400_body15_e31187_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body15_e31185: f64 = (locals.var_xp + locals.var_xmp);
        (assign31400_body15_e31185, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign31400_body15_e31187;
            locals.var_arg_dn0 = assign31400_body15_e31187_d_n0;
            locals.var_arg_dn2 = assign31400_body15_e31187_d_n2;
            locals.var_arg_dn4 = assign31400_body15_e31187_d_n4;
            locals.var_arg_dn5 = assign31400_body15_e31187_d_n5;
            locals.var_arg_dn6 = assign31400_body15_e31187_d_n6;
            locals.var_arg_dn7 = assign31400_body15_e31187_d_n7;
            locals.var_arg_dn8 = assign31400_body15_e31187_d_n8;
            locals.var_arg_dn9 = assign31400_body15_e31187_d_n9;
            locals.var_arg_dn10 = assign31400_body15_e31187_d_n10;
            locals.var_arg_dn11 = assign31400_body15_e31187_d_n11;
            locals.var_arg_dn14 = assign31400_body15_e31187_d_n14;
            let (assign31400_body16_e31200, assign31400_body16_e31200_d_n0, assign31400_body16_e31200_d_n2, assign31400_body16_e31200_d_n4, assign31400_body16_e31200_d_n5, assign31400_body16_e31200_d_n6, assign31400_body16_e31200_d_n7, assign31400_body16_e31200_d_n8, assign31400_body16_e31200_d_n9, assign31400_body16_e31200_d_n10, assign31400_body16_e31200_d_n11, assign31400_body16_e31200_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31400_body16_e31200;
            locals.var_dnm_dn0 = assign31400_body16_e31200_d_n0;
            locals.var_dnm_dn2 = assign31400_body16_e31200_d_n2;
            locals.var_dnm_dn4 = assign31400_body16_e31200_d_n4;
            locals.var_dnm_dn5 = assign31400_body16_e31200_d_n5;
            locals.var_dnm_dn6 = assign31400_body16_e31200_d_n6;
            locals.var_dnm_dn7 = assign31400_body16_e31200_d_n7;
            locals.var_dnm_dn8 = assign31400_body16_e31200_d_n8;
            locals.var_dnm_dn9 = assign31400_body16_e31200_d_n9;
            locals.var_dnm_dn10 = assign31400_body16_e31200_d_n10;
            locals.var_dnm_dn11 = assign31400_body16_e31200_d_n11;
            locals.var_dnm_dn14 = assign31400_body16_e31200_d_n14;
            let assign31400_body17_e31215: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard718 = assign31400_body17_e31215;
            let assign31400_body18_e31218: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard719 = assign31400_body18_e31218;
            let (assign31400_body19_e31235,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) && (locals.var_guard718 != 0.0)) && (locals.var_guard719 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31400_body19_e31235;
            let assign31400_body20_e31238: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard720 = assign31400_body20_e31238;
            let (assign31400_body21_e31258,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) && (locals.var_guard718 != 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31400_body21_e31258;
            let assign31400_body22_e31261: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard721 = assign31400_body22_e31261;
            let (assign31400_body23_e31284,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) && (locals.var_guard718 != 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 == 0.0)) && (locals.var_guard721 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31400_body23_e31284;
            let assign31400_body24_e31287: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard722 = assign31400_body24_e31287;
            let (assign31400_body25_e31313,) = {
    if ((((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) && (locals.var_guard718 != 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 == 0.0)) && (locals.var_guard721 == 0.0)) && (locals.var_guard722 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31400_body25_e31313;
            let (assign31400_body26_e31328,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) && (locals.var_guard718 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign31400_body26_e31328;
            let mut assign31400_body27_loop_guard: usize = 0;
            while {
                let assign31400_body27_cond_e31344: f64 = if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) && (locals.var_guard718 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign31400_body27_cond_e31344 != 0.0
            } {
                assign31400_body27_loop_guard += 1;
                assert!(assign31400_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign31400_body27_body0_e31360, assign31400_body27_body0_e31360_d_n0, assign31400_body27_body0_e31360_d_n2, assign31400_body27_body0_e31360_d_n4, assign31400_body27_body0_e31360_d_n5, assign31400_body27_body0_e31360_d_n6, assign31400_body27_body0_e31360_d_n7, assign31400_body27_body0_e31360_d_n8, assign31400_body27_body0_e31360_d_n9, assign31400_body27_body0_e31360_d_n10, assign31400_body27_body0_e31360_d_n11, assign31400_body27_body0_e31360_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) && (locals.var_guard718 != 0.0)) {
        let assign31400_body27_body0_e31358: f64 = (locals.var_dnm).sqrt();
        (assign31400_body27_body0_e31358, (locals.var_dnm_dn0 / (2.0 * assign31400_body27_body0_e31358)), (locals.var_dnm_dn2 / (2.0 * assign31400_body27_body0_e31358)), (locals.var_dnm_dn4 / (2.0 * assign31400_body27_body0_e31358)), (locals.var_dnm_dn5 / (2.0 * assign31400_body27_body0_e31358)), (locals.var_dnm_dn6 / (2.0 * assign31400_body27_body0_e31358)), (locals.var_dnm_dn7 / (2.0 * assign31400_body27_body0_e31358)), (locals.var_dnm_dn8 / (2.0 * assign31400_body27_body0_e31358)), (locals.var_dnm_dn9 / (2.0 * assign31400_body27_body0_e31358)), (locals.var_dnm_dn10 / (2.0 * assign31400_body27_body0_e31358)), (locals.var_dnm_dn11 / (2.0 * assign31400_body27_body0_e31358)), (locals.var_dnm_dn14 / (2.0 * assign31400_body27_body0_e31358)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
                locals.var_dnm = assign31400_body27_body0_e31360;
                locals.var_dnm_dn0 = assign31400_body27_body0_e31360_d_n0;
                locals.var_dnm_dn2 = assign31400_body27_body0_e31360_d_n2;
                locals.var_dnm_dn4 = assign31400_body27_body0_e31360_d_n4;
                locals.var_dnm_dn5 = assign31400_body27_body0_e31360_d_n5;
                locals.var_dnm_dn6 = assign31400_body27_body0_e31360_d_n6;
                locals.var_dnm_dn7 = assign31400_body27_body0_e31360_d_n7;
                locals.var_dnm_dn8 = assign31400_body27_body0_e31360_d_n8;
                locals.var_dnm_dn9 = assign31400_body27_body0_e31360_d_n9;
                locals.var_dnm_dn10 = assign31400_body27_body0_e31360_d_n10;
                locals.var_dnm_dn11 = assign31400_body27_body0_e31360_d_n11;
                locals.var_dnm_dn14 = assign31400_body27_body0_e31360_d_n14;
                let (assign31400_body27_body1_e31377,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) && (locals.var_guard718 != 0.0)) {
        let assign31400_body27_body1_e31375: f64 = (locals.var_m0 + 1.0);
        (assign31400_body27_body1_e31375,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign31400_body27_body1_e31377;
            }
            let (assign31400_body28_e31404, assign31400_body28_e31404_d_n0, assign31400_body28_e31404_d_n2, assign31400_body28_e31404_d_n4, assign31400_body28_e31404_d_n5, assign31400_body28_e31404_d_n6, assign31400_body28_e31404_d_n7, assign31400_body28_e31404_d_n8, assign31400_body28_e31404_d_n9, assign31400_body28_e31404_d_n10, assign31400_body28_e31404_d_n11, assign31400_body28_e31404_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) && (locals.var_guard718 == 0.0)) {
        let (assign31400_body28_e31402, assign31400_body28_e31402_d_n0, assign31400_body28_e31402_d_n2, assign31400_body28_e31402_d_n4, assign31400_body28_e31402_d_n5, assign31400_body28_e31402_d_n6, assign31400_body28_e31402_d_n7, assign31400_body28_e31402_d_n8, assign31400_body28_e31402_d_n9, assign31400_body28_e31402_d_n10, assign31400_body28_e31402_d_n11, assign31400_body28_e31402_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign31400_body28_e31399: f64 = (2.0 * 2.0);
                let assign31400_body28_e31400: f64 = (1.0 / assign31400_body28_e31399);
                let assign31400_body28_e31401: f64 = (locals.var_dnm).powf(assign31400_body28_e31400);
                (assign31400_body28_e31401, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn0)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn2)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn4)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn5)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn6)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn7)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn8)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn9)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn10)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn11)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body28_e31400) as f64).is_finite() && ((assign31400_body28_e31400) as f64).fract() == 0.0 { if assign31400_body28_e31400 == 0.0 { 0.0 } else { (assign31400_body28_e31400 * ((locals.var_dnm).powf(assign31400_body28_e31400 - 1.0) * locals.var_dnm_dn14)) } } else { (assign31400_body28_e31401 * (assign31400_body28_e31400 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign31400_body28_e31402, assign31400_body28_e31402_d_n0, assign31400_body28_e31402_d_n2, assign31400_body28_e31402_d_n4, assign31400_body28_e31402_d_n5, assign31400_body28_e31402_d_n6, assign31400_body28_e31402_d_n7, assign31400_body28_e31402_d_n8, assign31400_body28_e31402_d_n9, assign31400_body28_e31402_d_n10, assign31400_body28_e31402_d_n11, assign31400_body28_e31402_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31400_body28_e31404;
            locals.var_dnm_dn0 = assign31400_body28_e31404_d_n0;
            locals.var_dnm_dn2 = assign31400_body28_e31404_d_n2;
            locals.var_dnm_dn4 = assign31400_body28_e31404_d_n4;
            locals.var_dnm_dn5 = assign31400_body28_e31404_d_n5;
            locals.var_dnm_dn6 = assign31400_body28_e31404_d_n6;
            locals.var_dnm_dn7 = assign31400_body28_e31404_d_n7;
            locals.var_dnm_dn8 = assign31400_body28_e31404_d_n8;
            locals.var_dnm_dn9 = assign31400_body28_e31404_d_n9;
            locals.var_dnm_dn10 = assign31400_body28_e31404_d_n10;
            locals.var_dnm_dn11 = assign31400_body28_e31404_d_n11;
            locals.var_dnm_dn14 = assign31400_body28_e31404_d_n14;
            let (assign31400_body29_e31419, assign31400_body29_e31419_d_n0, assign31400_body29_e31419_d_n2, assign31400_body29_e31419_d_n4, assign31400_body29_e31419_d_n5, assign31400_body29_e31419_d_n6, assign31400_body29_e31419_d_n7, assign31400_body29_e31419_d_n8, assign31400_body29_e31419_d_n9, assign31400_body29_e31419_d_n10, assign31400_body29_e31419_d_n11, assign31400_body29_e31419_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body29_e31417: f64 = (1.0 / locals.var_dnm);
        (assign31400_body29_e31417, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31400_body29_e31419;
            locals.var_dnm_dn0 = assign31400_body29_e31419_d_n0;
            locals.var_dnm_dn2 = assign31400_body29_e31419_d_n2;
            locals.var_dnm_dn4 = assign31400_body29_e31419_d_n4;
            locals.var_dnm_dn5 = assign31400_body29_e31419_d_n5;
            locals.var_dnm_dn6 = assign31400_body29_e31419_d_n6;
            locals.var_dnm_dn7 = assign31400_body29_e31419_d_n7;
            locals.var_dnm_dn8 = assign31400_body29_e31419_d_n8;
            locals.var_dnm_dn9 = assign31400_body29_e31419_d_n9;
            locals.var_dnm_dn10 = assign31400_body29_e31419_d_n10;
            locals.var_dnm_dn11 = assign31400_body29_e31419_d_n11;
            locals.var_dnm_dn14 = assign31400_body29_e31419_d_n14;
            let (assign31400_body30_e31436, assign31400_body30_e31436_d_n0, assign31400_body30_e31436_d_n2, assign31400_body30_e31436_d_n4, assign31400_body30_e31436_d_n5, assign31400_body30_e31436_d_n6, assign31400_body30_e31436_d_n7, assign31400_body30_e31436_d_n8, assign31400_body30_e31436_d_n9, assign31400_body30_e31436_d_n10, assign31400_body30_e31436_d_n11, assign31400_body30_e31436_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body30_e31432: f64 = (locals.var_tmf1 * 1e-8);
        let assign31400_body30_e31434: f64 = (assign31400_body30_e31432 * locals.var_dnm);
        (assign31400_body30_e31434, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign31400_body30_e31432 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
            locals.var_tmf0 = assign31400_body30_e31436;
            locals.var_tmf0_dn0 = assign31400_body30_e31436_d_n0;
            locals.var_tmf0_dn2 = assign31400_body30_e31436_d_n2;
            locals.var_tmf0_dn4 = assign31400_body30_e31436_d_n4;
            locals.var_tmf0_dn5 = assign31400_body30_e31436_d_n5;
            locals.var_tmf0_dn6 = assign31400_body30_e31436_d_n6;
            locals.var_tmf0_dn7 = assign31400_body30_e31436_d_n7;
            locals.var_tmf0_dn8 = assign31400_body30_e31436_d_n8;
            locals.var_tmf0_dn9 = assign31400_body30_e31436_d_n9;
            locals.var_tmf0_dn10 = assign31400_body30_e31436_d_n10;
            locals.var_tmf0_dn11 = assign31400_body30_e31436_d_n11;
            locals.var_tmf0_dn14 = assign31400_body30_e31436_d_n14;
            let (assign31400_body31_e31455, assign31400_body31_e31455_d_n0, assign31400_body31_e31455_d_n2, assign31400_body31_e31455_d_n4, assign31400_body31_e31455_d_n5, assign31400_body31_e31455_d_n6, assign31400_body31_e31455_d_n7, assign31400_body31_e31455_d_n8, assign31400_body31_e31455_d_n9, assign31400_body31_e31455_d_n10, assign31400_body31_e31455_d_n11, assign31400_body31_e31455_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body31_e31449: f64 = (1e-8 * locals.var_xmp);
        let assign31400_body31_e31451: f64 = (assign31400_body31_e31449 * locals.var_dnm);
        let assign31400_body31_e31453: f64 = (assign31400_body31_e31451 / locals.var_arg);
        (assign31400_body31_e31453, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn0)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn2)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn4)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn5)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn6)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn7)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn8)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn9)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn10)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn11)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign31400_body31_e31449 * locals.var_dnm_dn14)) * locals.var_arg) - (assign31400_body31_e31451 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign31400_body31_e31455;
            locals.var_t0_dn0 = assign31400_body31_e31455_d_n0;
            locals.var_t0_dn2 = assign31400_body31_e31455_d_n2;
            locals.var_t0_dn4 = assign31400_body31_e31455_d_n4;
            locals.var_t0_dn5 = assign31400_body31_e31455_d_n5;
            locals.var_t0_dn6 = assign31400_body31_e31455_d_n6;
            locals.var_t0_dn7 = assign31400_body31_e31455_d_n7;
            locals.var_t0_dn8 = assign31400_body31_e31455_d_n8;
            locals.var_t0_dn9 = assign31400_body31_e31455_d_n9;
            locals.var_t0_dn10 = assign31400_body31_e31455_d_n10;
            locals.var_t0_dn11 = assign31400_body31_e31455_d_n11;
            locals.var_t0_dn14 = assign31400_body31_e31455_d_n14;
            let (assign31400_body32_e31472, assign31400_body32_e31472_d_n0, assign31400_body32_e31472_d_n2, assign31400_body32_e31472_d_n4, assign31400_body32_e31472_d_n5, assign31400_body32_e31472_d_n6, assign31400_body32_e31472_d_n7, assign31400_body32_e31472_d_n8, assign31400_body32_e31472_d_n9, assign31400_body32_e31472_d_n10, assign31400_body32_e31472_d_n11, assign31400_body32_e31472_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign31400_body32_e31468: f64 = (locals.var_uc_depthn - 1e-8);
        let assign31400_body32_e31470: f64 = (assign31400_body32_e31468 + locals.var_tmf0);
        (assign31400_body32_e31470, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
            locals.var_w_bl = assign31400_body32_e31472;
            locals.var_w_bl_dn0 = assign31400_body32_e31472_d_n0;
            locals.var_w_bl_dn2 = assign31400_body32_e31472_d_n2;
            locals.var_w_bl_dn4 = assign31400_body32_e31472_d_n4;
            locals.var_w_bl_dn5 = assign31400_body32_e31472_d_n5;
            locals.var_w_bl_dn6 = assign31400_body32_e31472_d_n6;
            locals.var_w_bl_dn7 = assign31400_body32_e31472_d_n7;
            locals.var_w_bl_dn8 = assign31400_body32_e31472_d_n8;
            locals.var_w_bl_dn9 = assign31400_body32_e31472_d_n9;
            locals.var_w_bl_dn10 = assign31400_body32_e31472_d_n10;
            locals.var_w_bl_dn11 = assign31400_body32_e31472_d_n11;
            locals.var_w_bl_dn14 = assign31400_body32_e31472_d_n14;
            let (assign31400_body33_e31485, assign31400_body33_e31485_d_n0, assign31400_body33_e31485_d_n2, assign31400_body33_e31485_d_n4, assign31400_body33_e31485_d_n5, assign31400_body33_e31485_d_n6, assign31400_body33_e31485_d_n7, assign31400_body33_e31485_d_n8, assign31400_body33_e31485_d_n9, assign31400_body33_e31485_d_n10, assign31400_body33_e31485_d_n11, assign31400_body33_e31485_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign31400_body33_e31485;
            locals.var_t0_dn0 = assign31400_body33_e31485_d_n0;
            locals.var_t0_dn2 = assign31400_body33_e31485_d_n2;
            locals.var_t0_dn4 = assign31400_body33_e31485_d_n4;
            locals.var_t0_dn5 = assign31400_body33_e31485_d_n5;
            locals.var_t0_dn6 = assign31400_body33_e31485_d_n6;
            locals.var_t0_dn7 = assign31400_body33_e31485_d_n7;
            locals.var_t0_dn8 = assign31400_body33_e31485_d_n8;
            locals.var_t0_dn9 = assign31400_body33_e31485_d_n9;
            locals.var_t0_dn10 = assign31400_body33_e31485_d_n10;
            locals.var_t0_dn11 = assign31400_body33_e31485_d_n11;
            locals.var_t0_dn14 = assign31400_body33_e31485_d_n14;
            let (assign31400_body34_e31499, assign31400_body34_e31499_d_n0, assign31400_body34_e31499_d_n2, assign31400_body34_e31499_d_n4, assign31400_body34_e31499_d_n5, assign31400_body34_e31499_d_n6, assign31400_body34_e31499_d_n7, assign31400_body34_e31499_d_n8, assign31400_body34_e31499_d_n9, assign31400_body34_e31499_d_n10, assign31400_body34_e31499_d_n11, assign31400_body34_e31499_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 == 0.0)) {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
            locals.var_w_bl = assign31400_body34_e31499;
            locals.var_w_bl_dn0 = assign31400_body34_e31499_d_n0;
            locals.var_w_bl_dn2 = assign31400_body34_e31499_d_n2;
            locals.var_w_bl_dn4 = assign31400_body34_e31499_d_n4;
            locals.var_w_bl_dn5 = assign31400_body34_e31499_d_n5;
            locals.var_w_bl_dn6 = assign31400_body34_e31499_d_n6;
            locals.var_w_bl_dn7 = assign31400_body34_e31499_d_n7;
            locals.var_w_bl_dn8 = assign31400_body34_e31499_d_n8;
            locals.var_w_bl_dn9 = assign31400_body34_e31499_d_n9;
            locals.var_w_bl_dn10 = assign31400_body34_e31499_d_n10;
            locals.var_w_bl_dn11 = assign31400_body34_e31499_d_n11;
            locals.var_w_bl_dn14 = assign31400_body34_e31499_d_n14;
            let (assign31400_body35_e31513, assign31400_body35_e31513_d_n0, assign31400_body35_e31513_d_n2, assign31400_body35_e31513_d_n4, assign31400_body35_e31513_d_n5, assign31400_body35_e31513_d_n6, assign31400_body35_e31513_d_n7, assign31400_body35_e31513_d_n8, assign31400_body35_e31513_d_n9, assign31400_body35_e31513_d_n10, assign31400_body35_e31513_d_n11, assign31400_body35_e31513_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign31400_body35_e31513;
            locals.var_t0_dn0 = assign31400_body35_e31513_d_n0;
            locals.var_t0_dn2 = assign31400_body35_e31513_d_n2;
            locals.var_t0_dn4 = assign31400_body35_e31513_d_n4;
            locals.var_t0_dn5 = assign31400_body35_e31513_d_n5;
            locals.var_t0_dn6 = assign31400_body35_e31513_d_n6;
            locals.var_t0_dn7 = assign31400_body35_e31513_d_n7;
            locals.var_t0_dn8 = assign31400_body35_e31513_d_n8;
            locals.var_t0_dn9 = assign31400_body35_e31513_d_n9;
            locals.var_t0_dn10 = assign31400_body35_e31513_d_n10;
            locals.var_t0_dn11 = assign31400_body35_e31513_d_n11;
            locals.var_t0_dn14 = assign31400_body35_e31513_d_n14;
            let (assign31400_body36_e31528, assign31400_body36_e31528_d_n0, assign31400_body36_e31528_d_n2, assign31400_body36_e31528_d_n4, assign31400_body36_e31528_d_n5, assign31400_body36_e31528_d_n6, assign31400_body36_e31528_d_n7, assign31400_body36_e31528_d_n8, assign31400_body36_e31528_d_n9, assign31400_body36_e31528_d_n10, assign31400_body36_e31528_d_n11, assign31400_body36_e31528_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body36_e31524: f64 = (locals.var_phi_jl_dep - locals.var_vbscl__blk439);
        let assign31400_body36_e31526: f64 = (assign31400_body36_e31524 + locals.var_vbi_dep);
        (assign31400_body36_e31526, ((locals.var_phi_jl_dep_dn0 - locals.var_vbscl__blk439_dn0) + locals.var_vbi_dep_dn0), ((locals.var_phi_jl_dep_dn2 - locals.var_vbscl__blk439_dn2) + locals.var_vbi_dep_dn2), ((locals.var_phi_jl_dep_dn4 - locals.var_vbscl__blk439_dn4) + locals.var_vbi_dep_dn4), ((locals.var_phi_jl_dep_dn5 - locals.var_vbscl__blk439_dn5) + locals.var_vbi_dep_dn5), ((locals.var_phi_jl_dep_dn6 - locals.var_vbscl__blk439_dn6) + locals.var_vbi_dep_dn6), ((locals.var_phi_jl_dep_dn7 - locals.var_vbscl__blk439_dn7) + locals.var_vbi_dep_dn7), ((locals.var_phi_jl_dep_dn8 - locals.var_vbscl__blk439_dn8) + locals.var_vbi_dep_dn8), ((locals.var_phi_jl_dep_dn9 - locals.var_vbscl__blk439_dn9) + locals.var_vbi_dep_dn9), ((locals.var_phi_jl_dep_dn10 - locals.var_vbscl__blk439_dn10) + locals.var_vbi_dep_dn10), ((locals.var_phi_jl_dep_dn11 - locals.var_vbscl__blk439_dn11) + locals.var_vbi_dep_dn11), ((locals.var_phi_jl_dep_dn14 - locals.var_vbscl__blk439_dn14) + locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign31400_body36_e31528;
            locals.var_t1_dn0 = assign31400_body36_e31528_d_n0;
            locals.var_t1_dn2 = assign31400_body36_e31528_d_n2;
            locals.var_t1_dn4 = assign31400_body36_e31528_d_n4;
            locals.var_t1_dn5 = assign31400_body36_e31528_d_n5;
            locals.var_t1_dn6 = assign31400_body36_e31528_d_n6;
            locals.var_t1_dn7 = assign31400_body36_e31528_d_n7;
            locals.var_t1_dn8 = assign31400_body36_e31528_d_n8;
            locals.var_t1_dn9 = assign31400_body36_e31528_d_n9;
            locals.var_t1_dn10 = assign31400_body36_e31528_d_n10;
            locals.var_t1_dn11 = assign31400_body36_e31528_d_n11;
            locals.var_t1_dn14 = assign31400_body36_e31528_d_n14;
            let assign31400_body37_e31532: f64 = 0.1;
            let assign31400_body37_e31537: f64 = if ((locals.var_t1 < assign31400_body37_e31532) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard723 = assign31400_body37_e31537;
            let (assign31400_body38_e31554, assign31400_body38_e31554_d_n0, assign31400_body38_e31554_d_n2, assign31400_body38_e31554_d_n4, assign31400_body38_e31554_d_n5, assign31400_body38_e31554_d_n6, assign31400_body38_e31554_d_n7, assign31400_body38_e31554_d_n8, assign31400_body38_e31554_d_n9, assign31400_body38_e31554_d_n10, assign31400_body38_e31554_d_n11, assign31400_body38_e31554_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body38_e31550: f64 = 0.1;
        let assign31400_body38_e31552: f64 = (assign31400_body38_e31550 - locals.var_t1);
        (assign31400_body38_e31552, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign31400_body38_e31554;
            locals.var_tmf1_dn0 = assign31400_body38_e31554_d_n0;
            locals.var_tmf1_dn2 = assign31400_body38_e31554_d_n2;
            locals.var_tmf1_dn4 = assign31400_body38_e31554_d_n4;
            locals.var_tmf1_dn5 = assign31400_body38_e31554_d_n5;
            locals.var_tmf1_dn6 = assign31400_body38_e31554_d_n6;
            locals.var_tmf1_dn7 = assign31400_body38_e31554_d_n7;
            locals.var_tmf1_dn8 = assign31400_body38_e31554_d_n8;
            locals.var_tmf1_dn9 = assign31400_body38_e31554_d_n9;
            locals.var_tmf1_dn10 = assign31400_body38_e31554_d_n10;
            locals.var_tmf1_dn11 = assign31400_body38_e31554_d_n11;
            locals.var_tmf1_dn14 = assign31400_body38_e31554_d_n14;
            let (assign31400_body39_e31569, assign31400_body39_e31569_d_n0, assign31400_body39_e31569_d_n2, assign31400_body39_e31569_d_n4, assign31400_body39_e31569_d_n5, assign31400_body39_e31569_d_n6, assign31400_body39_e31569_d_n7, assign31400_body39_e31569_d_n8, assign31400_body39_e31569_d_n9, assign31400_body39_e31569_d_n10, assign31400_body39_e31569_d_n11, assign31400_body39_e31569_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body39_e31567: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign31400_body39_e31567, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
            locals.var_x2 = assign31400_body39_e31569;
            locals.var_x2_dn0 = assign31400_body39_e31569_d_n0;
            locals.var_x2_dn2 = assign31400_body39_e31569_d_n2;
            locals.var_x2_dn4 = assign31400_body39_e31569_d_n4;
            locals.var_x2_dn5 = assign31400_body39_e31569_d_n5;
            locals.var_x2_dn6 = assign31400_body39_e31569_d_n6;
            locals.var_x2_dn7 = assign31400_body39_e31569_d_n7;
            locals.var_x2_dn8 = assign31400_body39_e31569_d_n8;
            locals.var_x2_dn9 = assign31400_body39_e31569_d_n9;
            locals.var_x2_dn10 = assign31400_body39_e31569_d_n10;
            locals.var_x2_dn11 = assign31400_body39_e31569_d_n11;
            locals.var_x2_dn14 = assign31400_body39_e31569_d_n14;
            let (assign31400_body40_e31584, assign31400_body40_e31584_d_n0, assign31400_body40_e31584_d_n2, assign31400_body40_e31584_d_n4, assign31400_body40_e31584_d_n5, assign31400_body40_e31584_d_n6, assign31400_body40_e31584_d_n7, assign31400_body40_e31584_d_n8, assign31400_body40_e31584_d_n9, assign31400_body40_e31584_d_n10, assign31400_body40_e31584_d_n11, assign31400_body40_e31584_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body40_e31582: f64 = (0.1 * 0.1);
        (assign31400_body40_e31582, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
            locals.var_xmax2 = assign31400_body40_e31584;
            locals.var_xmax2_dn0 = assign31400_body40_e31584_d_n0;
            locals.var_xmax2_dn2 = assign31400_body40_e31584_d_n2;
            locals.var_xmax2_dn4 = assign31400_body40_e31584_d_n4;
            locals.var_xmax2_dn5 = assign31400_body40_e31584_d_n5;
            locals.var_xmax2_dn6 = assign31400_body40_e31584_d_n6;
            locals.var_xmax2_dn7 = assign31400_body40_e31584_d_n7;
            locals.var_xmax2_dn8 = assign31400_body40_e31584_d_n8;
            locals.var_xmax2_dn9 = assign31400_body40_e31584_d_n9;
            locals.var_xmax2_dn10 = assign31400_body40_e31584_d_n10;
            locals.var_xmax2_dn11 = assign31400_body40_e31584_d_n11;
            locals.var_xmax2_dn14 = assign31400_body40_e31584_d_n14;
            let (assign31400_body41_e31597, assign31400_body41_e31597_d_n0, assign31400_body41_e31597_d_n2, assign31400_body41_e31597_d_n4, assign31400_body41_e31597_d_n5, assign31400_body41_e31597_d_n6, assign31400_body41_e31597_d_n7, assign31400_body41_e31597_d_n8, assign31400_body41_e31597_d_n9, assign31400_body41_e31597_d_n10, assign31400_body41_e31597_d_n11, assign31400_body41_e31597_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31400_body41_e31597;
            locals.var_xp_dn0 = assign31400_body41_e31597_d_n0;
            locals.var_xp_dn2 = assign31400_body41_e31597_d_n2;
            locals.var_xp_dn4 = assign31400_body41_e31597_d_n4;
            locals.var_xp_dn5 = assign31400_body41_e31597_d_n5;
            locals.var_xp_dn6 = assign31400_body41_e31597_d_n6;
            locals.var_xp_dn7 = assign31400_body41_e31597_d_n7;
            locals.var_xp_dn8 = assign31400_body41_e31597_d_n8;
            locals.var_xp_dn9 = assign31400_body41_e31597_d_n9;
            locals.var_xp_dn10 = assign31400_body41_e31597_d_n10;
            locals.var_xp_dn11 = assign31400_body41_e31597_d_n11;
            locals.var_xp_dn14 = assign31400_body41_e31597_d_n14;
            let (assign31400_body42_e31610, assign31400_body42_e31610_d_n0, assign31400_body42_e31610_d_n2, assign31400_body42_e31610_d_n4, assign31400_body42_e31610_d_n5, assign31400_body42_e31610_d_n6, assign31400_body42_e31610_d_n7, assign31400_body42_e31610_d_n8, assign31400_body42_e31610_d_n9, assign31400_body42_e31610_d_n10, assign31400_body42_e31610_d_n11, assign31400_body42_e31610_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31400_body42_e31610;
            locals.var_xmp_dn0 = assign31400_body42_e31610_d_n0;
            locals.var_xmp_dn2 = assign31400_body42_e31610_d_n2;
            locals.var_xmp_dn4 = assign31400_body42_e31610_d_n4;
            locals.var_xmp_dn5 = assign31400_body42_e31610_d_n5;
            locals.var_xmp_dn6 = assign31400_body42_e31610_d_n6;
            locals.var_xmp_dn7 = assign31400_body42_e31610_d_n7;
            locals.var_xmp_dn8 = assign31400_body42_e31610_d_n8;
            locals.var_xmp_dn9 = assign31400_body42_e31610_d_n9;
            locals.var_xmp_dn10 = assign31400_body42_e31610_d_n10;
            locals.var_xmp_dn11 = assign31400_body42_e31610_d_n11;
            locals.var_xmp_dn14 = assign31400_body42_e31610_d_n14;
            let (assign31400_body43_e31623,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign31400_body43_e31623;
            let (assign31400_body44_e31636,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31400_body44_e31636;
            let (assign31400_body45_e31649, assign31400_body45_e31649_d_n0, assign31400_body45_e31649_d_n2, assign31400_body45_e31649_d_n4, assign31400_body45_e31649_d_n5, assign31400_body45_e31649_d_n6, assign31400_body45_e31649_d_n7, assign31400_body45_e31649_d_n8, assign31400_body45_e31649_d_n9, assign31400_body45_e31649_d_n10, assign31400_body45_e31649_d_n11, assign31400_body45_e31649_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign31400_body45_e31649;
            locals.var_arg_dn0 = assign31400_body45_e31649_d_n0;
            locals.var_arg_dn2 = assign31400_body45_e31649_d_n2;
            locals.var_arg_dn4 = assign31400_body45_e31649_d_n4;
            locals.var_arg_dn5 = assign31400_body45_e31649_d_n5;
            locals.var_arg_dn6 = assign31400_body45_e31649_d_n6;
            locals.var_arg_dn7 = assign31400_body45_e31649_d_n7;
            locals.var_arg_dn8 = assign31400_body45_e31649_d_n8;
            locals.var_arg_dn9 = assign31400_body45_e31649_d_n9;
            locals.var_arg_dn10 = assign31400_body45_e31649_d_n10;
            locals.var_arg_dn11 = assign31400_body45_e31649_d_n11;
            locals.var_arg_dn14 = assign31400_body45_e31649_d_n14;
            let (assign31400_body46_e31662, assign31400_body46_e31662_d_n0, assign31400_body46_e31662_d_n2, assign31400_body46_e31662_d_n4, assign31400_body46_e31662_d_n5, assign31400_body46_e31662_d_n6, assign31400_body46_e31662_d_n7, assign31400_body46_e31662_d_n8, assign31400_body46_e31662_d_n9, assign31400_body46_e31662_d_n10, assign31400_body46_e31662_d_n11, assign31400_body46_e31662_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31400_body46_e31662;
            locals.var_dnm_dn0 = assign31400_body46_e31662_d_n0;
            locals.var_dnm_dn2 = assign31400_body46_e31662_d_n2;
            locals.var_dnm_dn4 = assign31400_body46_e31662_d_n4;
            locals.var_dnm_dn5 = assign31400_body46_e31662_d_n5;
            locals.var_dnm_dn6 = assign31400_body46_e31662_d_n6;
            locals.var_dnm_dn7 = assign31400_body46_e31662_d_n7;
            locals.var_dnm_dn8 = assign31400_body46_e31662_d_n8;
            locals.var_dnm_dn9 = assign31400_body46_e31662_d_n9;
            locals.var_dnm_dn10 = assign31400_body46_e31662_d_n10;
            locals.var_dnm_dn11 = assign31400_body46_e31662_d_n11;
            locals.var_dnm_dn14 = assign31400_body46_e31662_d_n14;
            let (assign31400_body47_e31677, assign31400_body47_e31677_d_n0, assign31400_body47_e31677_d_n2, assign31400_body47_e31677_d_n4, assign31400_body47_e31677_d_n5, assign31400_body47_e31677_d_n6, assign31400_body47_e31677_d_n7, assign31400_body47_e31677_d_n8, assign31400_body47_e31677_d_n9, assign31400_body47_e31677_d_n10, assign31400_body47_e31677_d_n11, assign31400_body47_e31677_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body47_e31675: f64 = (locals.var_xp * locals.var_x2);
        (assign31400_body47_e31675, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31400_body47_e31677;
            locals.var_xp_dn0 = assign31400_body47_e31677_d_n0;
            locals.var_xp_dn2 = assign31400_body47_e31677_d_n2;
            locals.var_xp_dn4 = assign31400_body47_e31677_d_n4;
            locals.var_xp_dn5 = assign31400_body47_e31677_d_n5;
            locals.var_xp_dn6 = assign31400_body47_e31677_d_n6;
            locals.var_xp_dn7 = assign31400_body47_e31677_d_n7;
            locals.var_xp_dn8 = assign31400_body47_e31677_d_n8;
            locals.var_xp_dn9 = assign31400_body47_e31677_d_n9;
            locals.var_xp_dn10 = assign31400_body47_e31677_d_n10;
            locals.var_xp_dn11 = assign31400_body47_e31677_d_n11;
            locals.var_xp_dn14 = assign31400_body47_e31677_d_n14;
            let (assign31400_body48_e31692, assign31400_body48_e31692_d_n0, assign31400_body48_e31692_d_n2, assign31400_body48_e31692_d_n4, assign31400_body48_e31692_d_n5, assign31400_body48_e31692_d_n6, assign31400_body48_e31692_d_n7, assign31400_body48_e31692_d_n8, assign31400_body48_e31692_d_n9, assign31400_body48_e31692_d_n10, assign31400_body48_e31692_d_n11, assign31400_body48_e31692_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body48_e31690: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign31400_body48_e31690, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31400_body48_e31692;
            locals.var_xmp_dn0 = assign31400_body48_e31692_d_n0;
            locals.var_xmp_dn2 = assign31400_body48_e31692_d_n2;
            locals.var_xmp_dn4 = assign31400_body48_e31692_d_n4;
            locals.var_xmp_dn5 = assign31400_body48_e31692_d_n5;
            locals.var_xmp_dn6 = assign31400_body48_e31692_d_n6;
            locals.var_xmp_dn7 = assign31400_body48_e31692_d_n7;
            locals.var_xmp_dn8 = assign31400_body48_e31692_d_n8;
            locals.var_xmp_dn9 = assign31400_body48_e31692_d_n9;
            locals.var_xmp_dn10 = assign31400_body48_e31692_d_n10;
            locals.var_xmp_dn11 = assign31400_body48_e31692_d_n11;
            locals.var_xmp_dn14 = assign31400_body48_e31692_d_n14;
            let (assign31400_body49_e31707, assign31400_body49_e31707_d_n0, assign31400_body49_e31707_d_n2, assign31400_body49_e31707_d_n4, assign31400_body49_e31707_d_n5, assign31400_body49_e31707_d_n6, assign31400_body49_e31707_d_n7, assign31400_body49_e31707_d_n8, assign31400_body49_e31707_d_n9, assign31400_body49_e31707_d_n10, assign31400_body49_e31707_d_n11, assign31400_body49_e31707_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body49_e31705: f64 = (locals.var_xp * locals.var_x2);
        (assign31400_body49_e31705, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31400_body49_e31707;
            locals.var_xp_dn0 = assign31400_body49_e31707_d_n0;
            locals.var_xp_dn2 = assign31400_body49_e31707_d_n2;
            locals.var_xp_dn4 = assign31400_body49_e31707_d_n4;
            locals.var_xp_dn5 = assign31400_body49_e31707_d_n5;
            locals.var_xp_dn6 = assign31400_body49_e31707_d_n6;
            locals.var_xp_dn7 = assign31400_body49_e31707_d_n7;
            locals.var_xp_dn8 = assign31400_body49_e31707_d_n8;
            locals.var_xp_dn9 = assign31400_body49_e31707_d_n9;
            locals.var_xp_dn10 = assign31400_body49_e31707_d_n10;
            locals.var_xp_dn11 = assign31400_body49_e31707_d_n11;
            locals.var_xp_dn14 = assign31400_body49_e31707_d_n14;
            let (assign31400_body50_e31722, assign31400_body50_e31722_d_n0, assign31400_body50_e31722_d_n2, assign31400_body50_e31722_d_n4, assign31400_body50_e31722_d_n5, assign31400_body50_e31722_d_n6, assign31400_body50_e31722_d_n7, assign31400_body50_e31722_d_n8, assign31400_body50_e31722_d_n9, assign31400_body50_e31722_d_n10, assign31400_body50_e31722_d_n11, assign31400_body50_e31722_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body50_e31720: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign31400_body50_e31720, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31400_body50_e31722;
            locals.var_xmp_dn0 = assign31400_body50_e31722_d_n0;
            locals.var_xmp_dn2 = assign31400_body50_e31722_d_n2;
            locals.var_xmp_dn4 = assign31400_body50_e31722_d_n4;
            locals.var_xmp_dn5 = assign31400_body50_e31722_d_n5;
            locals.var_xmp_dn6 = assign31400_body50_e31722_d_n6;
            locals.var_xmp_dn7 = assign31400_body50_e31722_d_n7;
            locals.var_xmp_dn8 = assign31400_body50_e31722_d_n8;
            locals.var_xmp_dn9 = assign31400_body50_e31722_d_n9;
            locals.var_xmp_dn10 = assign31400_body50_e31722_d_n10;
            locals.var_xmp_dn11 = assign31400_body50_e31722_d_n11;
            locals.var_xmp_dn14 = assign31400_body50_e31722_d_n14;
            let (assign31400_body51_e31737, assign31400_body51_e31737_d_n0, assign31400_body51_e31737_d_n2, assign31400_body51_e31737_d_n4, assign31400_body51_e31737_d_n5, assign31400_body51_e31737_d_n6, assign31400_body51_e31737_d_n7, assign31400_body51_e31737_d_n8, assign31400_body51_e31737_d_n9, assign31400_body51_e31737_d_n10, assign31400_body51_e31737_d_n11, assign31400_body51_e31737_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body51_e31735: f64 = (locals.var_xp + locals.var_xmp);
        (assign31400_body51_e31735, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign31400_body51_e31737;
            locals.var_arg_dn0 = assign31400_body51_e31737_d_n0;
            locals.var_arg_dn2 = assign31400_body51_e31737_d_n2;
            locals.var_arg_dn4 = assign31400_body51_e31737_d_n4;
            locals.var_arg_dn5 = assign31400_body51_e31737_d_n5;
            locals.var_arg_dn6 = assign31400_body51_e31737_d_n6;
            locals.var_arg_dn7 = assign31400_body51_e31737_d_n7;
            locals.var_arg_dn8 = assign31400_body51_e31737_d_n8;
            locals.var_arg_dn9 = assign31400_body51_e31737_d_n9;
            locals.var_arg_dn10 = assign31400_body51_e31737_d_n10;
            locals.var_arg_dn11 = assign31400_body51_e31737_d_n11;
            locals.var_arg_dn14 = assign31400_body51_e31737_d_n14;
            let (assign31400_body52_e31750, assign31400_body52_e31750_d_n0, assign31400_body52_e31750_d_n2, assign31400_body52_e31750_d_n4, assign31400_body52_e31750_d_n5, assign31400_body52_e31750_d_n6, assign31400_body52_e31750_d_n7, assign31400_body52_e31750_d_n8, assign31400_body52_e31750_d_n9, assign31400_body52_e31750_d_n10, assign31400_body52_e31750_d_n11, assign31400_body52_e31750_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31400_body52_e31750;
            locals.var_dnm_dn0 = assign31400_body52_e31750_d_n0;
            locals.var_dnm_dn2 = assign31400_body52_e31750_d_n2;
            locals.var_dnm_dn4 = assign31400_body52_e31750_d_n4;
            locals.var_dnm_dn5 = assign31400_body52_e31750_d_n5;
            locals.var_dnm_dn6 = assign31400_body52_e31750_d_n6;
            locals.var_dnm_dn7 = assign31400_body52_e31750_d_n7;
            locals.var_dnm_dn8 = assign31400_body52_e31750_d_n8;
            locals.var_dnm_dn9 = assign31400_body52_e31750_d_n9;
            locals.var_dnm_dn10 = assign31400_body52_e31750_d_n10;
            locals.var_dnm_dn11 = assign31400_body52_e31750_d_n11;
            locals.var_dnm_dn14 = assign31400_body52_e31750_d_n14;
            let assign31400_body53_e31765: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard724 = assign31400_body53_e31765;
            let assign31400_body54_e31768: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard725 = assign31400_body54_e31768;
            let (assign31400_body55_e31785,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) && (locals.var_guard724 != 0.0)) && (locals.var_guard725 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31400_body55_e31785;
            let assign31400_body56_e31788: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard726 = assign31400_body56_e31788;
            let (assign31400_body57_e31808,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) && (locals.var_guard724 != 0.0)) && (locals.var_guard725 == 0.0)) && (locals.var_guard726 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31400_body57_e31808;
            let assign31400_body58_e31811: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard727 = assign31400_body58_e31811;
            let (assign31400_body59_e31834,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) && (locals.var_guard724 != 0.0)) && (locals.var_guard725 == 0.0)) && (locals.var_guard726 == 0.0)) && (locals.var_guard727 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31400_body59_e31834;
            let assign31400_body60_e31837: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard728 = assign31400_body60_e31837;
            let (assign31400_body61_e31863,) = {
    if ((((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) && (locals.var_guard724 != 0.0)) && (locals.var_guard725 == 0.0)) && (locals.var_guard726 == 0.0)) && (locals.var_guard727 == 0.0)) && (locals.var_guard728 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31400_body61_e31863;
            let (assign31400_body62_e31878,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) && (locals.var_guard724 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign31400_body62_e31878;
            let mut assign31400_body63_loop_guard: usize = 0;
            while {
                let assign31400_body63_cond_e31894: f64 = if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) && (locals.var_guard724 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign31400_body63_cond_e31894 != 0.0
            } {
                assign31400_body63_loop_guard += 1;
                assert!(assign31400_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign31400_body63_body0_e31910, assign31400_body63_body0_e31910_d_n0, assign31400_body63_body0_e31910_d_n2, assign31400_body63_body0_e31910_d_n4, assign31400_body63_body0_e31910_d_n5, assign31400_body63_body0_e31910_d_n6, assign31400_body63_body0_e31910_d_n7, assign31400_body63_body0_e31910_d_n8, assign31400_body63_body0_e31910_d_n9, assign31400_body63_body0_e31910_d_n10, assign31400_body63_body0_e31910_d_n11, assign31400_body63_body0_e31910_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) && (locals.var_guard724 != 0.0)) {
        let assign31400_body63_body0_e31908: f64 = (locals.var_dnm).sqrt();
        (assign31400_body63_body0_e31908, (locals.var_dnm_dn0 / (2.0 * assign31400_body63_body0_e31908)), (locals.var_dnm_dn2 / (2.0 * assign31400_body63_body0_e31908)), (locals.var_dnm_dn4 / (2.0 * assign31400_body63_body0_e31908)), (locals.var_dnm_dn5 / (2.0 * assign31400_body63_body0_e31908)), (locals.var_dnm_dn6 / (2.0 * assign31400_body63_body0_e31908)), (locals.var_dnm_dn7 / (2.0 * assign31400_body63_body0_e31908)), (locals.var_dnm_dn8 / (2.0 * assign31400_body63_body0_e31908)), (locals.var_dnm_dn9 / (2.0 * assign31400_body63_body0_e31908)), (locals.var_dnm_dn10 / (2.0 * assign31400_body63_body0_e31908)), (locals.var_dnm_dn11 / (2.0 * assign31400_body63_body0_e31908)), (locals.var_dnm_dn14 / (2.0 * assign31400_body63_body0_e31908)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
                locals.var_dnm = assign31400_body63_body0_e31910;
                locals.var_dnm_dn0 = assign31400_body63_body0_e31910_d_n0;
                locals.var_dnm_dn2 = assign31400_body63_body0_e31910_d_n2;
                locals.var_dnm_dn4 = assign31400_body63_body0_e31910_d_n4;
                locals.var_dnm_dn5 = assign31400_body63_body0_e31910_d_n5;
                locals.var_dnm_dn6 = assign31400_body63_body0_e31910_d_n6;
                locals.var_dnm_dn7 = assign31400_body63_body0_e31910_d_n7;
                locals.var_dnm_dn8 = assign31400_body63_body0_e31910_d_n8;
                locals.var_dnm_dn9 = assign31400_body63_body0_e31910_d_n9;
                locals.var_dnm_dn10 = assign31400_body63_body0_e31910_d_n10;
                locals.var_dnm_dn11 = assign31400_body63_body0_e31910_d_n11;
                locals.var_dnm_dn14 = assign31400_body63_body0_e31910_d_n14;
                let (assign31400_body63_body1_e31927,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) && (locals.var_guard724 != 0.0)) {
        let assign31400_body63_body1_e31925: f64 = (locals.var_m0 + 1.0);
        (assign31400_body63_body1_e31925,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign31400_body63_body1_e31927;
            }
            let (assign31400_body64_e31954, assign31400_body64_e31954_d_n0, assign31400_body64_e31954_d_n2, assign31400_body64_e31954_d_n4, assign31400_body64_e31954_d_n5, assign31400_body64_e31954_d_n6, assign31400_body64_e31954_d_n7, assign31400_body64_e31954_d_n8, assign31400_body64_e31954_d_n9, assign31400_body64_e31954_d_n10, assign31400_body64_e31954_d_n11, assign31400_body64_e31954_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) && (locals.var_guard724 == 0.0)) {
        let (assign31400_body64_e31952, assign31400_body64_e31952_d_n0, assign31400_body64_e31952_d_n2, assign31400_body64_e31952_d_n4, assign31400_body64_e31952_d_n5, assign31400_body64_e31952_d_n6, assign31400_body64_e31952_d_n7, assign31400_body64_e31952_d_n8, assign31400_body64_e31952_d_n9, assign31400_body64_e31952_d_n10, assign31400_body64_e31952_d_n11, assign31400_body64_e31952_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign31400_body64_e31949: f64 = (2.0 * 2.0);
                let assign31400_body64_e31950: f64 = (1.0 / assign31400_body64_e31949);
                let assign31400_body64_e31951: f64 = (locals.var_dnm).powf(assign31400_body64_e31950);
                (assign31400_body64_e31951, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn0)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn2)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn4)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn5)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn6)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn7)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn8)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn9)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn10)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn11)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31400_body64_e31950) as f64).is_finite() && ((assign31400_body64_e31950) as f64).fract() == 0.0 { if assign31400_body64_e31950 == 0.0 { 0.0 } else { (assign31400_body64_e31950 * ((locals.var_dnm).powf(assign31400_body64_e31950 - 1.0) * locals.var_dnm_dn14)) } } else { (assign31400_body64_e31951 * (assign31400_body64_e31950 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign31400_body64_e31952, assign31400_body64_e31952_d_n0, assign31400_body64_e31952_d_n2, assign31400_body64_e31952_d_n4, assign31400_body64_e31952_d_n5, assign31400_body64_e31952_d_n6, assign31400_body64_e31952_d_n7, assign31400_body64_e31952_d_n8, assign31400_body64_e31952_d_n9, assign31400_body64_e31952_d_n10, assign31400_body64_e31952_d_n11, assign31400_body64_e31952_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31400_body64_e31954;
            locals.var_dnm_dn0 = assign31400_body64_e31954_d_n0;
            locals.var_dnm_dn2 = assign31400_body64_e31954_d_n2;
            locals.var_dnm_dn4 = assign31400_body64_e31954_d_n4;
            locals.var_dnm_dn5 = assign31400_body64_e31954_d_n5;
            locals.var_dnm_dn6 = assign31400_body64_e31954_d_n6;
            locals.var_dnm_dn7 = assign31400_body64_e31954_d_n7;
            locals.var_dnm_dn8 = assign31400_body64_e31954_d_n8;
            locals.var_dnm_dn9 = assign31400_body64_e31954_d_n9;
            locals.var_dnm_dn10 = assign31400_body64_e31954_d_n10;
            locals.var_dnm_dn11 = assign31400_body64_e31954_d_n11;
            locals.var_dnm_dn14 = assign31400_body64_e31954_d_n14;
            let (assign31400_body65_e31969, assign31400_body65_e31969_d_n0, assign31400_body65_e31969_d_n2, assign31400_body65_e31969_d_n4, assign31400_body65_e31969_d_n5, assign31400_body65_e31969_d_n6, assign31400_body65_e31969_d_n7, assign31400_body65_e31969_d_n8, assign31400_body65_e31969_d_n9, assign31400_body65_e31969_d_n10, assign31400_body65_e31969_d_n11, assign31400_body65_e31969_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body65_e31967: f64 = (1.0 / locals.var_dnm);
        (assign31400_body65_e31967, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31400_body65_e31969;
            locals.var_dnm_dn0 = assign31400_body65_e31969_d_n0;
            locals.var_dnm_dn2 = assign31400_body65_e31969_d_n2;
            locals.var_dnm_dn4 = assign31400_body65_e31969_d_n4;
            locals.var_dnm_dn5 = assign31400_body65_e31969_d_n5;
            locals.var_dnm_dn6 = assign31400_body65_e31969_d_n6;
            locals.var_dnm_dn7 = assign31400_body65_e31969_d_n7;
            locals.var_dnm_dn8 = assign31400_body65_e31969_d_n8;
            locals.var_dnm_dn9 = assign31400_body65_e31969_d_n9;
            locals.var_dnm_dn10 = assign31400_body65_e31969_d_n10;
            locals.var_dnm_dn11 = assign31400_body65_e31969_d_n11;
            locals.var_dnm_dn14 = assign31400_body65_e31969_d_n14;
            let (assign31400_body66_e31986, assign31400_body66_e31986_d_n0, assign31400_body66_e31986_d_n2, assign31400_body66_e31986_d_n4, assign31400_body66_e31986_d_n5, assign31400_body66_e31986_d_n6, assign31400_body66_e31986_d_n7, assign31400_body66_e31986_d_n8, assign31400_body66_e31986_d_n9, assign31400_body66_e31986_d_n10, assign31400_body66_e31986_d_n11, assign31400_body66_e31986_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body66_e31982: f64 = (locals.var_tmf1 * 0.1);
        let assign31400_body66_e31984: f64 = (assign31400_body66_e31982 * locals.var_dnm);
        (assign31400_body66_e31984, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign31400_body66_e31982 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
            locals.var_tmf0 = assign31400_body66_e31986;
            locals.var_tmf0_dn0 = assign31400_body66_e31986_d_n0;
            locals.var_tmf0_dn2 = assign31400_body66_e31986_d_n2;
            locals.var_tmf0_dn4 = assign31400_body66_e31986_d_n4;
            locals.var_tmf0_dn5 = assign31400_body66_e31986_d_n5;
            locals.var_tmf0_dn6 = assign31400_body66_e31986_d_n6;
            locals.var_tmf0_dn7 = assign31400_body66_e31986_d_n7;
            locals.var_tmf0_dn8 = assign31400_body66_e31986_d_n8;
            locals.var_tmf0_dn9 = assign31400_body66_e31986_d_n9;
            locals.var_tmf0_dn10 = assign31400_body66_e31986_d_n10;
            locals.var_tmf0_dn11 = assign31400_body66_e31986_d_n11;
            locals.var_tmf0_dn14 = assign31400_body66_e31986_d_n14;
            let (assign31400_body67_e32005, assign31400_body67_e32005_d_n0, assign31400_body67_e32005_d_n2, assign31400_body67_e32005_d_n4, assign31400_body67_e32005_d_n5, assign31400_body67_e32005_d_n6, assign31400_body67_e32005_d_n7, assign31400_body67_e32005_d_n8, assign31400_body67_e32005_d_n9, assign31400_body67_e32005_d_n10, assign31400_body67_e32005_d_n11, assign31400_body67_e32005_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body67_e31999: f64 = (0.1 * locals.var_xmp);
        let assign31400_body67_e32001: f64 = (assign31400_body67_e31999 * locals.var_dnm);
        let assign31400_body67_e32003: f64 = (assign31400_body67_e32001 / locals.var_arg);
        (assign31400_body67_e32003, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn0)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn2)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn4)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn5)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn6)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn7)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn8)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn9)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn10)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn11)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign31400_body67_e31999 * locals.var_dnm_dn14)) * locals.var_arg) - (assign31400_body67_e32001 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign31400_body67_e32005;
            locals.var_t7_dn0 = assign31400_body67_e32005_d_n0;
            locals.var_t7_dn2 = assign31400_body67_e32005_d_n2;
            locals.var_t7_dn4 = assign31400_body67_e32005_d_n4;
            locals.var_t7_dn5 = assign31400_body67_e32005_d_n5;
            locals.var_t7_dn6 = assign31400_body67_e32005_d_n6;
            locals.var_t7_dn7 = assign31400_body67_e32005_d_n7;
            locals.var_t7_dn8 = assign31400_body67_e32005_d_n8;
            locals.var_t7_dn9 = assign31400_body67_e32005_d_n9;
            locals.var_t7_dn10 = assign31400_body67_e32005_d_n10;
            locals.var_t7_dn11 = assign31400_body67_e32005_d_n11;
            locals.var_t7_dn14 = assign31400_body67_e32005_d_n14;
            let (assign31400_body68_e32022, assign31400_body68_e32022_d_n0, assign31400_body68_e32022_d_n2, assign31400_body68_e32022_d_n4, assign31400_body68_e32022_d_n5, assign31400_body68_e32022_d_n6, assign31400_body68_e32022_d_n7, assign31400_body68_e32022_d_n8, assign31400_body68_e32022_d_n9, assign31400_body68_e32022_d_n10, assign31400_body68_e32022_d_n11, assign31400_body68_e32022_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign31400_body68_e32018: f64 = 0.1;
        let assign31400_body68_e32020: f64 = (assign31400_body68_e32018 - locals.var_tmf0);
        (assign31400_body68_e32020, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign31400_body68_e32022;
            locals.var_t2_dn0 = assign31400_body68_e32022_d_n0;
            locals.var_t2_dn2 = assign31400_body68_e32022_d_n2;
            locals.var_t2_dn4 = assign31400_body68_e32022_d_n4;
            locals.var_t2_dn5 = assign31400_body68_e32022_d_n5;
            locals.var_t2_dn6 = assign31400_body68_e32022_d_n6;
            locals.var_t2_dn7 = assign31400_body68_e32022_d_n7;
            locals.var_t2_dn8 = assign31400_body68_e32022_d_n8;
            locals.var_t2_dn9 = assign31400_body68_e32022_d_n9;
            locals.var_t2_dn10 = assign31400_body68_e32022_d_n10;
            locals.var_t2_dn11 = assign31400_body68_e32022_d_n11;
            locals.var_t2_dn14 = assign31400_body68_e32022_d_n14;
            let (assign31400_body69_e32035, assign31400_body69_e32035_d_n0, assign31400_body69_e32035_d_n2, assign31400_body69_e32035_d_n4, assign31400_body69_e32035_d_n5, assign31400_body69_e32035_d_n6, assign31400_body69_e32035_d_n7, assign31400_body69_e32035_d_n8, assign31400_body69_e32035_d_n9, assign31400_body69_e32035_d_n10, assign31400_body69_e32035_d_n11, assign31400_body69_e32035_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 != 0.0)) {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign31400_body69_e32035;
            locals.var_t7_dn0 = assign31400_body69_e32035_d_n0;
            locals.var_t7_dn2 = assign31400_body69_e32035_d_n2;
            locals.var_t7_dn4 = assign31400_body69_e32035_d_n4;
            locals.var_t7_dn5 = assign31400_body69_e32035_d_n5;
            locals.var_t7_dn6 = assign31400_body69_e32035_d_n6;
            locals.var_t7_dn7 = assign31400_body69_e32035_d_n7;
            locals.var_t7_dn8 = assign31400_body69_e32035_d_n8;
            locals.var_t7_dn9 = assign31400_body69_e32035_d_n9;
            locals.var_t7_dn10 = assign31400_body69_e32035_d_n10;
            locals.var_t7_dn11 = assign31400_body69_e32035_d_n11;
            locals.var_t7_dn14 = assign31400_body69_e32035_d_n14;
            let (assign31400_body70_e32049, assign31400_body70_e32049_d_n0, assign31400_body70_e32049_d_n2, assign31400_body70_e32049_d_n4, assign31400_body70_e32049_d_n5, assign31400_body70_e32049_d_n6, assign31400_body70_e32049_d_n7, assign31400_body70_e32049_d_n8, assign31400_body70_e32049_d_n9, assign31400_body70_e32049_d_n10, assign31400_body70_e32049_d_n11, assign31400_body70_e32049_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign31400_body70_e32049;
            locals.var_t2_dn0 = assign31400_body70_e32049_d_n0;
            locals.var_t2_dn2 = assign31400_body70_e32049_d_n2;
            locals.var_t2_dn4 = assign31400_body70_e32049_d_n4;
            locals.var_t2_dn5 = assign31400_body70_e32049_d_n5;
            locals.var_t2_dn6 = assign31400_body70_e32049_d_n6;
            locals.var_t2_dn7 = assign31400_body70_e32049_d_n7;
            locals.var_t2_dn8 = assign31400_body70_e32049_d_n8;
            locals.var_t2_dn9 = assign31400_body70_e32049_d_n9;
            locals.var_t2_dn10 = assign31400_body70_e32049_d_n10;
            locals.var_t2_dn11 = assign31400_body70_e32049_d_n11;
            locals.var_t2_dn14 = assign31400_body70_e32049_d_n14;
            let (assign31400_body71_e32063, assign31400_body71_e32063_d_n0, assign31400_body71_e32063_d_n2, assign31400_body71_e32063_d_n4, assign31400_body71_e32063_d_n5, assign31400_body71_e32063_d_n6, assign31400_body71_e32063_d_n7, assign31400_body71_e32063_d_n8, assign31400_body71_e32063_d_n9, assign31400_body71_e32063_d_n10, assign31400_body71_e32063_d_n11, assign31400_body71_e32063_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard723 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign31400_body71_e32063;
            locals.var_t7_dn0 = assign31400_body71_e32063_d_n0;
            locals.var_t7_dn2 = assign31400_body71_e32063_d_n2;
            locals.var_t7_dn4 = assign31400_body71_e32063_d_n4;
            locals.var_t7_dn5 = assign31400_body71_e32063_d_n5;
            locals.var_t7_dn6 = assign31400_body71_e32063_d_n6;
            locals.var_t7_dn7 = assign31400_body71_e32063_d_n7;
            locals.var_t7_dn8 = assign31400_body71_e32063_d_n8;
            locals.var_t7_dn9 = assign31400_body71_e32063_d_n9;
            locals.var_t7_dn10 = assign31400_body71_e32063_d_n10;
            locals.var_t7_dn11 = assign31400_body71_e32063_d_n11;
            locals.var_t7_dn14 = assign31400_body71_e32063_d_n14;
            let (assign31400_body72_e32077, assign31400_body72_e32077_d_n0, assign31400_body72_e32077_d_n2, assign31400_body72_e32077_d_n4, assign31400_body72_e32077_d_n5, assign31400_body72_e32077_d_n6, assign31400_body72_e32077_d_n7, assign31400_body72_e32077_d_n8, assign31400_body72_e32077_d_n9, assign31400_body72_e32077_d_n10, assign31400_body72_e32077_d_n11, assign31400_body72_e32077_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body72_e32074: f64 = (locals.var_c_2esipq_nsub * locals.var_t2);
        let assign31400_body72_e32075: f64 = (assign31400_body72_e32074).sqrt();
        (assign31400_body72_e32075, (((locals.var_c_2esipq_nsub_dn0 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn0)) / (2.0 * assign31400_body72_e32075)), (((locals.var_c_2esipq_nsub_dn2 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn2)) / (2.0 * assign31400_body72_e32075)), (((locals.var_c_2esipq_nsub_dn4 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn4)) / (2.0 * assign31400_body72_e32075)), (((locals.var_c_2esipq_nsub_dn5 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn5)) / (2.0 * assign31400_body72_e32075)), (((locals.var_c_2esipq_nsub_dn6 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn6)) / (2.0 * assign31400_body72_e32075)), (((locals.var_c_2esipq_nsub_dn7 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn7)) / (2.0 * assign31400_body72_e32075)), (((locals.var_c_2esipq_nsub_dn8 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn8)) / (2.0 * assign31400_body72_e32075)), (((locals.var_c_2esipq_nsub_dn9 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn9)) / (2.0 * assign31400_body72_e32075)), (((locals.var_c_2esipq_nsub_dn10 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn10)) / (2.0 * assign31400_body72_e32075)), (((locals.var_c_2esipq_nsub_dn11 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn11)) / (2.0 * assign31400_body72_e32075)), (((locals.var_c_2esipq_nsub_dn14 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn14)) / (2.0 * assign31400_body72_e32075)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn11, locals.var_w_subl_dn14,)
    }
};
            locals.var_w_subl = assign31400_body72_e32077;
            locals.var_w_subl_dn0 = assign31400_body72_e32077_d_n0;
            locals.var_w_subl_dn2 = assign31400_body72_e32077_d_n2;
            locals.var_w_subl_dn4 = assign31400_body72_e32077_d_n4;
            locals.var_w_subl_dn5 = assign31400_body72_e32077_d_n5;
            locals.var_w_subl_dn6 = assign31400_body72_e32077_d_n6;
            locals.var_w_subl_dn7 = assign31400_body72_e32077_d_n7;
            locals.var_w_subl_dn8 = assign31400_body72_e32077_d_n8;
            locals.var_w_subl_dn9 = assign31400_body72_e32077_d_n9;
            locals.var_w_subl_dn10 = assign31400_body72_e32077_d_n10;
            locals.var_w_subl_dn11 = assign31400_body72_e32077_d_n11;
            locals.var_w_subl_dn14 = assign31400_body72_e32077_d_n14;
            let (assign31400_body73_e32090, assign31400_body73_e32090_d_n0, assign31400_body73_e32090_d_n2, assign31400_body73_e32090_d_n4, assign31400_body73_e32090_d_n5, assign31400_body73_e32090_d_n6, assign31400_body73_e32090_d_n7, assign31400_body73_e32090_d_n8, assign31400_body73_e32090_d_n9, assign31400_body73_e32090_d_n10, assign31400_body73_e32090_d_n11, assign31400_body73_e32090_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body73_e32088: f64 = (locals.var_w_bl * locals.var_q_ndepm);
        (assign31400_body73_e32088, ((locals.var_w_bl_dn0 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn0)), ((locals.var_w_bl_dn2 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn2)), ((locals.var_w_bl_dn4 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn4)), ((locals.var_w_bl_dn5 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn5)), ((locals.var_w_bl_dn6 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn6)), ((locals.var_w_bl_dn7 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn7)), ((locals.var_w_bl_dn8 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn8)), ((locals.var_w_bl_dn9 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn9)), ((locals.var_w_bl_dn10 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn10)), ((locals.var_w_bl_dn11 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn11)), ((locals.var_w_bl_dn14 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
            locals.var_q_bl_dep = assign31400_body73_e32090;
            locals.var_q_bl_dep_dn0 = assign31400_body73_e32090_d_n0;
            locals.var_q_bl_dep_dn2 = assign31400_body73_e32090_d_n2;
            locals.var_q_bl_dep_dn4 = assign31400_body73_e32090_d_n4;
            locals.var_q_bl_dep_dn5 = assign31400_body73_e32090_d_n5;
            locals.var_q_bl_dep_dn6 = assign31400_body73_e32090_d_n6;
            locals.var_q_bl_dep_dn7 = assign31400_body73_e32090_d_n7;
            locals.var_q_bl_dep_dn8 = assign31400_body73_e32090_d_n8;
            locals.var_q_bl_dep_dn9 = assign31400_body73_e32090_d_n9;
            locals.var_q_bl_dep_dn10 = assign31400_body73_e32090_d_n10;
            locals.var_q_bl_dep_dn11 = assign31400_body73_e32090_d_n11;
            locals.var_q_bl_dep_dn14 = assign31400_body73_e32090_d_n14;
            let (assign31400_body74_e32106, assign31400_body74_e32106_d_n0, assign31400_body74_e32106_d_n2, assign31400_body74_e32106_d_n4, assign31400_body74_e32106_d_n5, assign31400_body74_e32106_d_n6, assign31400_body74_e32106_d_n7, assign31400_body74_e32106_d_n8, assign31400_body74_e32106_d_n9, assign31400_body74_e32106_d_n10, assign31400_body74_e32106_d_n11, assign31400_body74_e32106_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body74_e32100: f64 = (-1.034943e-10);
        let assign31400_body74_e32102: f64 = (assign31400_body74_e32100 / locals.var_w_bl);
        let assign31400_body74_e32104: f64 = (assign31400_body74_e32102 * locals.var_t0);
        (assign31400_body74_e32104, (((-((assign31400_body74_e32100 * locals.var_w_bl_dn0) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn0)), (((-((assign31400_body74_e32100 * locals.var_w_bl_dn2) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn2)), (((-((assign31400_body74_e32100 * locals.var_w_bl_dn4) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn4)), (((-((assign31400_body74_e32100 * locals.var_w_bl_dn5) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn5)), (((-((assign31400_body74_e32100 * locals.var_w_bl_dn6) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn6)), (((-((assign31400_body74_e32100 * locals.var_w_bl_dn7) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn7)), (((-((assign31400_body74_e32100 * locals.var_w_bl_dn8) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn8)), (((-((assign31400_body74_e32100 * locals.var_w_bl_dn9) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn9)), (((-((assign31400_body74_e32100 * locals.var_w_bl_dn10) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn10)), (((-((assign31400_body74_e32100 * locals.var_w_bl_dn11) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn11)), (((-((assign31400_body74_e32100 * locals.var_w_bl_dn14) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31400_body74_e32102 * locals.var_t0_dn14)),)
    } else {
        (locals.var_q_bl_dep_dpd, locals.var_q_bl_dep_dpd_dn0, locals.var_q_bl_dep_dpd_dn2, locals.var_q_bl_dep_dpd_dn4, locals.var_q_bl_dep_dpd_dn5, locals.var_q_bl_dep_dpd_dn6, locals.var_q_bl_dep_dpd_dn7, locals.var_q_bl_dep_dpd_dn8, locals.var_q_bl_dep_dpd_dn9, locals.var_q_bl_dep_dpd_dn10, locals.var_q_bl_dep_dpd_dn11, locals.var_q_bl_dep_dpd_dn14,)
    }
};
            locals.var_q_bl_dep_dpd = assign31400_body74_e32106;
            locals.var_q_bl_dep_dpd_dn0 = assign31400_body74_e32106_d_n0;
            locals.var_q_bl_dep_dpd_dn2 = assign31400_body74_e32106_d_n2;
            locals.var_q_bl_dep_dpd_dn4 = assign31400_body74_e32106_d_n4;
            locals.var_q_bl_dep_dpd_dn5 = assign31400_body74_e32106_d_n5;
            locals.var_q_bl_dep_dpd_dn6 = assign31400_body74_e32106_d_n6;
            locals.var_q_bl_dep_dpd_dn7 = assign31400_body74_e32106_d_n7;
            locals.var_q_bl_dep_dpd_dn8 = assign31400_body74_e32106_d_n8;
            locals.var_q_bl_dep_dpd_dn9 = assign31400_body74_e32106_d_n9;
            locals.var_q_bl_dep_dpd_dn10 = assign31400_body74_e32106_d_n10;
            locals.var_q_bl_dep_dpd_dn11 = assign31400_body74_e32106_d_n11;
            locals.var_q_bl_dep_dpd_dn14 = assign31400_body74_e32106_d_n14;
            let (assign31400_body75_e32120, assign31400_body75_e32120_d_n0, assign31400_body75_e32120_d_n2, assign31400_body75_e32120_d_n4, assign31400_body75_e32120_d_n5, assign31400_body75_e32120_d_n6, assign31400_body75_e32120_d_n7, assign31400_body75_e32120_d_n8, assign31400_body75_e32120_d_n9, assign31400_body75_e32120_d_n10, assign31400_body75_e32120_d_n11, assign31400_body75_e32120_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body75_e32116: f64 = (-locals.var_w_subl);
        let assign31400_body75_e32118: f64 = (assign31400_body75_e32116 * locals.var_q_nsub__blk548);
        (assign31400_body75_e32118, (((-locals.var_w_subl_dn0) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn0)), (((-locals.var_w_subl_dn2) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn2)), (((-locals.var_w_subl_dn4) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn4)), (((-locals.var_w_subl_dn5) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn5)), (((-locals.var_w_subl_dn6) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn6)), (((-locals.var_w_subl_dn7) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn7)), (((-locals.var_w_subl_dn8) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn8)), (((-locals.var_w_subl_dn9) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn9)), (((-locals.var_w_subl_dn10) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn10)), (((-locals.var_w_subl_dn11) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn11)), (((-locals.var_w_subl_dn14) * locals.var_q_nsub__blk548) + (assign31400_body75_e32116 * locals.var_q_nsub__blk548_dn14)),)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn11, locals.var_q_subl_dep_dn14,)
    }
};
            locals.var_q_subl_dep = assign31400_body75_e32120;
            locals.var_q_subl_dep_dn0 = assign31400_body75_e32120_d_n0;
            locals.var_q_subl_dep_dn2 = assign31400_body75_e32120_d_n2;
            locals.var_q_subl_dep_dn4 = assign31400_body75_e32120_d_n4;
            locals.var_q_subl_dep_dn5 = assign31400_body75_e32120_d_n5;
            locals.var_q_subl_dep_dn6 = assign31400_body75_e32120_d_n6;
            locals.var_q_subl_dep_dn7 = assign31400_body75_e32120_d_n7;
            locals.var_q_subl_dep_dn8 = assign31400_body75_e32120_d_n8;
            locals.var_q_subl_dep_dn9 = assign31400_body75_e32120_d_n9;
            locals.var_q_subl_dep_dn10 = assign31400_body75_e32120_d_n10;
            locals.var_q_subl_dep_dn11 = assign31400_body75_e32120_d_n11;
            locals.var_q_subl_dep_dn14 = assign31400_body75_e32120_d_n14;
            let (assign31400_body76_e32136, assign31400_body76_e32136_d_n0, assign31400_body76_e32136_d_n2, assign31400_body76_e32136_d_n4, assign31400_body76_e32136_d_n5, assign31400_body76_e32136_d_n6, assign31400_body76_e32136_d_n7, assign31400_body76_e32136_d_n8, assign31400_body76_e32136_d_n9, assign31400_body76_e32136_d_n10, assign31400_body76_e32136_d_n11, assign31400_body76_e32136_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body76_e32130: f64 = (-1.034943e-10);
        let assign31400_body76_e32132: f64 = (assign31400_body76_e32130 / locals.var_w_subl);
        let assign31400_body76_e32134: f64 = (assign31400_body76_e32132 * locals.var_t7);
        (assign31400_body76_e32134, (((-((assign31400_body76_e32130 * locals.var_w_subl_dn0) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn0)), (((-((assign31400_body76_e32130 * locals.var_w_subl_dn2) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn2)), (((-((assign31400_body76_e32130 * locals.var_w_subl_dn4) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn4)), (((-((assign31400_body76_e32130 * locals.var_w_subl_dn5) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn5)), (((-((assign31400_body76_e32130 * locals.var_w_subl_dn6) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn6)), (((-((assign31400_body76_e32130 * locals.var_w_subl_dn7) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn7)), (((-((assign31400_body76_e32130 * locals.var_w_subl_dn8) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn8)), (((-((assign31400_body76_e32130 * locals.var_w_subl_dn9) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn9)), (((-((assign31400_body76_e32130 * locals.var_w_subl_dn10) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn10)), (((-((assign31400_body76_e32130 * locals.var_w_subl_dn11) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn11)), (((-((assign31400_body76_e32130 * locals.var_w_subl_dn14) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31400_body76_e32132 * locals.var_t7_dn14)),)
    } else {
        (locals.var_q_subl_dep_dpd, locals.var_q_subl_dep_dpd_dn0, locals.var_q_subl_dep_dpd_dn2, locals.var_q_subl_dep_dpd_dn4, locals.var_q_subl_dep_dpd_dn5, locals.var_q_subl_dep_dpd_dn6, locals.var_q_subl_dep_dpd_dn7, locals.var_q_subl_dep_dpd_dn8, locals.var_q_subl_dep_dpd_dn9, locals.var_q_subl_dep_dpd_dn10, locals.var_q_subl_dep_dpd_dn11, locals.var_q_subl_dep_dpd_dn14,)
    }
};
            locals.var_q_subl_dep_dpd = assign31400_body76_e32136;
            locals.var_q_subl_dep_dpd_dn0 = assign31400_body76_e32136_d_n0;
            locals.var_q_subl_dep_dpd_dn2 = assign31400_body76_e32136_d_n2;
            locals.var_q_subl_dep_dpd_dn4 = assign31400_body76_e32136_d_n4;
            locals.var_q_subl_dep_dpd_dn5 = assign31400_body76_e32136_d_n5;
            locals.var_q_subl_dep_dpd_dn6 = assign31400_body76_e32136_d_n6;
            locals.var_q_subl_dep_dpd_dn7 = assign31400_body76_e32136_d_n7;
            locals.var_q_subl_dep_dpd_dn8 = assign31400_body76_e32136_d_n8;
            locals.var_q_subl_dep_dpd_dn9 = assign31400_body76_e32136_d_n9;
            locals.var_q_subl_dep_dpd_dn10 = assign31400_body76_e32136_d_n10;
            locals.var_q_subl_dep_dpd_dn11 = assign31400_body76_e32136_d_n11;
            locals.var_q_subl_dep_dpd_dn14 = assign31400_body76_e32136_d_n14;
            let (assign31400_body77_e32155, assign31400_body77_e32155_d_n0, assign31400_body77_e32155_d_n2, assign31400_body77_e32155_d_n4, assign31400_body77_e32155_d_n5, assign31400_body77_e32155_d_n6, assign31400_body77_e32155_d_n7, assign31400_body77_e32155_d_n8, assign31400_body77_e32155_d_n9, assign31400_body77_e32155_d_n10, assign31400_body77_e32155_d_n11, assign31400_body77_e32155_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body77_e32148: f64 = (locals.var_vgp0 - locals.var_phi_bl_dep);
        let assign31400_body77_e32149: f64 = (locals.var_cox * assign31400_body77_e32148);
        let assign31400_body77_e32151: f64 = (assign31400_body77_e32149 + locals.var_q_bl_dep);
        let assign31400_body77_e32153: f64 = (assign31400_body77_e32151 + locals.var_q_subl_dep);
        (assign31400_body77_e32153, ((((locals.var_cox_dn0 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn0 - locals.var_phi_bl_dep_dn0))) + locals.var_q_bl_dep_dn0) + locals.var_q_subl_dep_dn0), ((((locals.var_cox_dn2 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn2 - locals.var_phi_bl_dep_dn2))) + locals.var_q_bl_dep_dn2) + locals.var_q_subl_dep_dn2), ((((locals.var_cox_dn4 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn4 - locals.var_phi_bl_dep_dn4))) + locals.var_q_bl_dep_dn4) + locals.var_q_subl_dep_dn4), ((((locals.var_cox_dn5 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn5 - locals.var_phi_bl_dep_dn5))) + locals.var_q_bl_dep_dn5) + locals.var_q_subl_dep_dn5), ((((locals.var_cox_dn6 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn6 - locals.var_phi_bl_dep_dn6))) + locals.var_q_bl_dep_dn6) + locals.var_q_subl_dep_dn6), ((((locals.var_cox_dn7 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn7 - locals.var_phi_bl_dep_dn7))) + locals.var_q_bl_dep_dn7) + locals.var_q_subl_dep_dn7), ((((locals.var_cox_dn8 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn8 - locals.var_phi_bl_dep_dn8))) + locals.var_q_bl_dep_dn8) + locals.var_q_subl_dep_dn8), ((((locals.var_cox_dn9 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn9 - locals.var_phi_bl_dep_dn9))) + locals.var_q_bl_dep_dn9) + locals.var_q_subl_dep_dn9), ((((locals.var_cox_dn10 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn10 - locals.var_phi_bl_dep_dn10))) + locals.var_q_bl_dep_dn10) + locals.var_q_subl_dep_dn10), ((((locals.var_cox_dn11 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn11 - locals.var_phi_bl_dep_dn11))) + locals.var_q_bl_dep_dn11) + locals.var_q_subl_dep_dn11), ((((locals.var_cox_dn14 * assign31400_body77_e32148) + (locals.var_cox * (locals.var_vgp0_dn14 - locals.var_phi_bl_dep_dn14))) + locals.var_q_bl_dep_dn14) + locals.var_q_subl_dep_dn14),)
    } else {
        (locals.var_y1, locals.var_y1_dn0, locals.var_y1_dn2, locals.var_y1_dn4, locals.var_y1_dn5, locals.var_y1_dn6, locals.var_y1_dn7, locals.var_y1_dn8, locals.var_y1_dn9, locals.var_y1_dn10, locals.var_y1_dn11, locals.var_y1_dn14,)
    }
};
            locals.var_y1 = assign31400_body77_e32155;
            locals.var_y1_dn0 = assign31400_body77_e32155_d_n0;
            locals.var_y1_dn2 = assign31400_body77_e32155_d_n2;
            locals.var_y1_dn4 = assign31400_body77_e32155_d_n4;
            locals.var_y1_dn5 = assign31400_body77_e32155_d_n5;
            locals.var_y1_dn6 = assign31400_body77_e32155_d_n6;
            locals.var_y1_dn7 = assign31400_body77_e32155_d_n7;
            locals.var_y1_dn8 = assign31400_body77_e32155_d_n8;
            locals.var_y1_dn9 = assign31400_body77_e32155_d_n9;
            locals.var_y1_dn10 = assign31400_body77_e32155_d_n10;
            locals.var_y1_dn11 = assign31400_body77_e32155_d_n11;
            locals.var_y1_dn14 = assign31400_body77_e32155_d_n14;
            let (assign31400_body78_e32166, assign31400_body78_e32166_d_n0, assign31400_body78_e32166_d_n2, assign31400_body78_e32166_d_n4, assign31400_body78_e32166_d_n5, assign31400_body78_e32166_d_n6, assign31400_body78_e32166_d_n7, assign31400_body78_e32166_d_n8, assign31400_body78_e32166_d_n9, assign31400_body78_e32166_d_n10, assign31400_body78_e32166_d_n11, assign31400_body78_e32166_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn14,)
    } else {
        (locals.var_y11, locals.var_y11_dn0, locals.var_y11_dn2, locals.var_y11_dn4, locals.var_y11_dn5, locals.var_y11_dn6, locals.var_y11_dn7, locals.var_y11_dn8, locals.var_y11_dn9, locals.var_y11_dn10, locals.var_y11_dn11, locals.var_y11_dn14,)
    }
};
            locals.var_y11 = assign31400_body78_e32166;
            locals.var_y11_dn0 = assign31400_body78_e32166_d_n0;
            locals.var_y11_dn2 = assign31400_body78_e32166_d_n2;
            locals.var_y11_dn4 = assign31400_body78_e32166_d_n4;
            locals.var_y11_dn5 = assign31400_body78_e32166_d_n5;
            locals.var_y11_dn6 = assign31400_body78_e32166_d_n6;
            locals.var_y11_dn7 = assign31400_body78_e32166_d_n7;
            locals.var_y11_dn8 = assign31400_body78_e32166_d_n8;
            locals.var_y11_dn9 = assign31400_body78_e32166_d_n9;
            locals.var_y11_dn10 = assign31400_body78_e32166_d_n10;
            locals.var_y11_dn11 = assign31400_body78_e32166_d_n11;
            locals.var_y11_dn14 = assign31400_body78_e32166_d_n14;
            let (assign31400_body79_e32179, assign31400_body79_e32179_d_n0, assign31400_body79_e32179_d_n2, assign31400_body79_e32179_d_n4, assign31400_body79_e32179_d_n5, assign31400_body79_e32179_d_n6, assign31400_body79_e32179_d_n7, assign31400_body79_e32179_d_n8, assign31400_body79_e32179_d_n9, assign31400_body79_e32179_d_n10, assign31400_body79_e32179_d_n11, assign31400_body79_e32179_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body79_e32177: f64 = (locals.var_q_bl_dep_dpd + locals.var_q_subl_dep_dpd);
        (assign31400_body79_e32177, (locals.var_q_bl_dep_dpd_dn0 + locals.var_q_subl_dep_dpd_dn0), (locals.var_q_bl_dep_dpd_dn2 + locals.var_q_subl_dep_dpd_dn2), (locals.var_q_bl_dep_dpd_dn4 + locals.var_q_subl_dep_dpd_dn4), (locals.var_q_bl_dep_dpd_dn5 + locals.var_q_subl_dep_dpd_dn5), (locals.var_q_bl_dep_dpd_dn6 + locals.var_q_subl_dep_dpd_dn6), (locals.var_q_bl_dep_dpd_dn7 + locals.var_q_subl_dep_dpd_dn7), (locals.var_q_bl_dep_dpd_dn8 + locals.var_q_subl_dep_dpd_dn8), (locals.var_q_bl_dep_dpd_dn9 + locals.var_q_subl_dep_dpd_dn9), (locals.var_q_bl_dep_dpd_dn10 + locals.var_q_subl_dep_dpd_dn10), (locals.var_q_bl_dep_dpd_dn11 + locals.var_q_subl_dep_dpd_dn11), (locals.var_q_bl_dep_dpd_dn14 + locals.var_q_subl_dep_dpd_dn14),)
    } else {
        (locals.var_y12, locals.var_y12_dn0, locals.var_y12_dn2, locals.var_y12_dn4, locals.var_y12_dn5, locals.var_y12_dn6, locals.var_y12_dn7, locals.var_y12_dn8, locals.var_y12_dn9, locals.var_y12_dn10, locals.var_y12_dn11, locals.var_y12_dn14,)
    }
};
            locals.var_y12 = assign31400_body79_e32179;
            locals.var_y12_dn0 = assign31400_body79_e32179_d_n0;
            locals.var_y12_dn2 = assign31400_body79_e32179_d_n2;
            locals.var_y12_dn4 = assign31400_body79_e32179_d_n4;
            locals.var_y12_dn5 = assign31400_body79_e32179_d_n5;
            locals.var_y12_dn6 = assign31400_body79_e32179_d_n6;
            locals.var_y12_dn7 = assign31400_body79_e32179_d_n7;
            locals.var_y12_dn8 = assign31400_body79_e32179_d_n8;
            locals.var_y12_dn9 = assign31400_body79_e32179_d_n9;
            locals.var_y12_dn10 = assign31400_body79_e32179_d_n10;
            locals.var_y12_dn11 = assign31400_body79_e32179_d_n11;
            locals.var_y12_dn14 = assign31400_body79_e32179_d_n14;
            let (assign31400_body80_e32200, assign31400_body80_e32200_d_n0, assign31400_body80_e32200_d_n2, assign31400_body80_e32200_d_n4, assign31400_body80_e32200_d_n5, assign31400_body80_e32200_d_n6, assign31400_body80_e32200_d_n7, assign31400_body80_e32200_d_n8, assign31400_body80_e32200_d_n9, assign31400_body80_e32200_d_n10, assign31400_body80_e32200_d_n11, assign31400_body80_e32200_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body80_e32192: f64 = (locals.var_ndepmpnsub * locals.var_phi_bl_dep);
        let assign31400_body80_e32194: f64 = (assign31400_body80_e32192 + locals.var_vbscl__blk439);
        let assign31400_body80_e32196: f64 = (assign31400_body80_e32194 - locals.var_vbi_dep);
        let assign31400_body80_e32197: f64 = (locals.var_ndepmpnsub_inv1 * assign31400_body80_e32196);
        let assign31400_body80_e32198: f64 = (locals.var_phi_jl_dep - assign31400_body80_e32197);
        (assign31400_body80_e32198, (locals.var_phi_jl_dep_dn0 - ((locals.var_ndepmpnsub_inv1_dn0 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn0 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn0)) + locals.var_vbscl__blk439_dn0) - locals.var_vbi_dep_dn0)))), (locals.var_phi_jl_dep_dn2 - ((locals.var_ndepmpnsub_inv1_dn2 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn2 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn2)) + locals.var_vbscl__blk439_dn2) - locals.var_vbi_dep_dn2)))), (locals.var_phi_jl_dep_dn4 - ((locals.var_ndepmpnsub_inv1_dn4 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn4 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn4)) + locals.var_vbscl__blk439_dn4) - locals.var_vbi_dep_dn4)))), (locals.var_phi_jl_dep_dn5 - ((locals.var_ndepmpnsub_inv1_dn5 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn5 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn5)) + locals.var_vbscl__blk439_dn5) - locals.var_vbi_dep_dn5)))), (locals.var_phi_jl_dep_dn6 - ((locals.var_ndepmpnsub_inv1_dn6 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn6 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn6)) + locals.var_vbscl__blk439_dn6) - locals.var_vbi_dep_dn6)))), (locals.var_phi_jl_dep_dn7 - ((locals.var_ndepmpnsub_inv1_dn7 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn7 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn7)) + locals.var_vbscl__blk439_dn7) - locals.var_vbi_dep_dn7)))), (locals.var_phi_jl_dep_dn8 - ((locals.var_ndepmpnsub_inv1_dn8 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn8 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn8)) + locals.var_vbscl__blk439_dn8) - locals.var_vbi_dep_dn8)))), (locals.var_phi_jl_dep_dn9 - ((locals.var_ndepmpnsub_inv1_dn9 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn9 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn9)) + locals.var_vbscl__blk439_dn9) - locals.var_vbi_dep_dn9)))), (locals.var_phi_jl_dep_dn10 - ((locals.var_ndepmpnsub_inv1_dn10 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn10 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn10)) + locals.var_vbscl__blk439_dn10) - locals.var_vbi_dep_dn10)))), (locals.var_phi_jl_dep_dn11 - ((locals.var_ndepmpnsub_inv1_dn11 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn11 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn11)) + locals.var_vbscl__blk439_dn11) - locals.var_vbi_dep_dn11)))), (locals.var_phi_jl_dep_dn14 - ((locals.var_ndepmpnsub_inv1_dn14 * assign31400_body80_e32196) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn14 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn14)) + locals.var_vbscl__blk439_dn14) - locals.var_vbi_dep_dn14)))),)
    } else {
        (locals.var_y2, locals.var_y2_dn0, locals.var_y2_dn2, locals.var_y2_dn4, locals.var_y2_dn5, locals.var_y2_dn6, locals.var_y2_dn7, locals.var_y2_dn8, locals.var_y2_dn9, locals.var_y2_dn10, locals.var_y2_dn11, locals.var_y2_dn14,)
    }
};
            locals.var_y2 = assign31400_body80_e32200;
            locals.var_y2_dn0 = assign31400_body80_e32200_d_n0;
            locals.var_y2_dn2 = assign31400_body80_e32200_d_n2;
            locals.var_y2_dn4 = assign31400_body80_e32200_d_n4;
            locals.var_y2_dn5 = assign31400_body80_e32200_d_n5;
            locals.var_y2_dn6 = assign31400_body80_e32200_d_n6;
            locals.var_y2_dn7 = assign31400_body80_e32200_d_n7;
            locals.var_y2_dn8 = assign31400_body80_e32200_d_n8;
            locals.var_y2_dn9 = assign31400_body80_e32200_d_n9;
            locals.var_y2_dn10 = assign31400_body80_e32200_d_n10;
            locals.var_y2_dn11 = assign31400_body80_e32200_d_n11;
            locals.var_y2_dn14 = assign31400_body80_e32200_d_n14;
            let (assign31400_body81_e32211, assign31400_body81_e32211_d_n0, assign31400_body81_e32211_d_n2, assign31400_body81_e32211_d_n4, assign31400_body81_e32211_d_n5, assign31400_body81_e32211_d_n6, assign31400_body81_e32211_d_n7, assign31400_body81_e32211_d_n8, assign31400_body81_e32211_d_n9, assign31400_body81_e32211_d_n10, assign31400_body81_e32211_d_n11, assign31400_body81_e32211_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_y21, locals.var_y21_dn0, locals.var_y21_dn2, locals.var_y21_dn4, locals.var_y21_dn5, locals.var_y21_dn6, locals.var_y21_dn7, locals.var_y21_dn8, locals.var_y21_dn9, locals.var_y21_dn10, locals.var_y21_dn11, locals.var_y21_dn14,)
    }
};
            locals.var_y21 = assign31400_body81_e32211;
            locals.var_y21_dn0 = assign31400_body81_e32211_d_n0;
            locals.var_y21_dn2 = assign31400_body81_e32211_d_n2;
            locals.var_y21_dn4 = assign31400_body81_e32211_d_n4;
            locals.var_y21_dn5 = assign31400_body81_e32211_d_n5;
            locals.var_y21_dn6 = assign31400_body81_e32211_d_n6;
            locals.var_y21_dn7 = assign31400_body81_e32211_d_n7;
            locals.var_y21_dn8 = assign31400_body81_e32211_d_n8;
            locals.var_y21_dn9 = assign31400_body81_e32211_d_n9;
            locals.var_y21_dn10 = assign31400_body81_e32211_d_n10;
            locals.var_y21_dn11 = assign31400_body81_e32211_d_n11;
            locals.var_y21_dn14 = assign31400_body81_e32211_d_n14;
            let (assign31400_body82_e32222, assign31400_body82_e32222_d_n0, assign31400_body82_e32222_d_n2, assign31400_body82_e32222_d_n4, assign31400_body82_e32222_d_n5, assign31400_body82_e32222_d_n6, assign31400_body82_e32222_d_n7, assign31400_body82_e32222_d_n8, assign31400_body82_e32222_d_n9, assign31400_body82_e32222_d_n10, assign31400_body82_e32222_d_n11, assign31400_body82_e32222_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_y22, locals.var_y22_dn0, locals.var_y22_dn2, locals.var_y22_dn4, locals.var_y22_dn5, locals.var_y22_dn6, locals.var_y22_dn7, locals.var_y22_dn8, locals.var_y22_dn9, locals.var_y22_dn10, locals.var_y22_dn11, locals.var_y22_dn14,)
    }
};
            locals.var_y22 = assign31400_body82_e32222;
            locals.var_y22_dn0 = assign31400_body82_e32222_d_n0;
            locals.var_y22_dn2 = assign31400_body82_e32222_d_n2;
            locals.var_y22_dn4 = assign31400_body82_e32222_d_n4;
            locals.var_y22_dn5 = assign31400_body82_e32222_d_n5;
            locals.var_y22_dn6 = assign31400_body82_e32222_d_n6;
            locals.var_y22_dn7 = assign31400_body82_e32222_d_n7;
            locals.var_y22_dn8 = assign31400_body82_e32222_d_n8;
            locals.var_y22_dn9 = assign31400_body82_e32222_d_n9;
            locals.var_y22_dn10 = assign31400_body82_e32222_d_n10;
            locals.var_y22_dn11 = assign31400_body82_e32222_d_n11;
            locals.var_y22_dn14 = assign31400_body82_e32222_d_n14;
            let (assign31400_body83_e32239, assign31400_body83_e32239_d_n0, assign31400_body83_e32239_d_n2, assign31400_body83_e32239_d_n4, assign31400_body83_e32239_d_n5, assign31400_body83_e32239_d_n6, assign31400_body83_e32239_d_n7, assign31400_body83_e32239_d_n8, assign31400_body83_e32239_d_n9, assign31400_body83_e32239_d_n10, assign31400_body83_e32239_d_n11, assign31400_body83_e32239_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body83_e32233: f64 = (locals.var_y11 * locals.var_y22);
        let assign31400_body83_e32236: f64 = (locals.var_y21 * locals.var_y12);
        let assign31400_body83_e32237: f64 = (assign31400_body83_e32233 - assign31400_body83_e32236);
        (assign31400_body83_e32237, (((locals.var_y11_dn0 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn0)) - ((locals.var_y21_dn0 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn0))), (((locals.var_y11_dn2 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn2)) - ((locals.var_y21_dn2 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn2))), (((locals.var_y11_dn4 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn4)) - ((locals.var_y21_dn4 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn4))), (((locals.var_y11_dn5 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn5)) - ((locals.var_y21_dn5 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn5))), (((locals.var_y11_dn6 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn6)) - ((locals.var_y21_dn6 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn6))), (((locals.var_y11_dn7 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn7)) - ((locals.var_y21_dn7 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn7))), (((locals.var_y11_dn8 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn8)) - ((locals.var_y21_dn8 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn8))), (((locals.var_y11_dn9 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn9)) - ((locals.var_y21_dn9 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn9))), (((locals.var_y11_dn10 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn10)) - ((locals.var_y21_dn10 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn10))), (((locals.var_y11_dn11 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn11)) - ((locals.var_y21_dn11 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn11))), (((locals.var_y11_dn14 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn14)) - ((locals.var_y21_dn14 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn14))),)
    } else {
        (locals.var_dety, locals.var_dety_dn0, locals.var_dety_dn2, locals.var_dety_dn4, locals.var_dety_dn5, locals.var_dety_dn6, locals.var_dety_dn7, locals.var_dety_dn8, locals.var_dety_dn9, locals.var_dety_dn10, locals.var_dety_dn11, locals.var_dety_dn14,)
    }
};
            locals.var_dety = assign31400_body83_e32239;
            locals.var_dety_dn0 = assign31400_body83_e32239_d_n0;
            locals.var_dety_dn2 = assign31400_body83_e32239_d_n2;
            locals.var_dety_dn4 = assign31400_body83_e32239_d_n4;
            locals.var_dety_dn5 = assign31400_body83_e32239_d_n5;
            locals.var_dety_dn6 = assign31400_body83_e32239_d_n6;
            locals.var_dety_dn7 = assign31400_body83_e32239_d_n7;
            locals.var_dety_dn8 = assign31400_body83_e32239_d_n8;
            locals.var_dety_dn9 = assign31400_body83_e32239_d_n9;
            locals.var_dety_dn10 = assign31400_body83_e32239_d_n10;
            locals.var_dety_dn11 = assign31400_body83_e32239_d_n11;
            locals.var_dety_dn14 = assign31400_body83_e32239_d_n14;
            let (assign31400_body84_e32252, assign31400_body84_e32252_d_n0, assign31400_body84_e32252_d_n2, assign31400_body84_e32252_d_n4, assign31400_body84_e32252_d_n5, assign31400_body84_e32252_d_n6, assign31400_body84_e32252_d_n7, assign31400_body84_e32252_d_n8, assign31400_body84_e32252_d_n9, assign31400_body84_e32252_d_n10, assign31400_body84_e32252_d_n11, assign31400_body84_e32252_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body84_e32250: f64 = (locals.var_y22 / locals.var_dety);
        (assign31400_body84_e32250, (((locals.var_y22_dn0 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn2 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn4 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn5 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn6 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn7 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn8 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn9 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn10 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn11 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn14 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev11, locals.var_rev11_dn0, locals.var_rev11_dn2, locals.var_rev11_dn4, locals.var_rev11_dn5, locals.var_rev11_dn6, locals.var_rev11_dn7, locals.var_rev11_dn8, locals.var_rev11_dn9, locals.var_rev11_dn10, locals.var_rev11_dn11, locals.var_rev11_dn14,)
    }
};
            locals.var_rev11 = assign31400_body84_e32252;
            locals.var_rev11_dn0 = assign31400_body84_e32252_d_n0;
            locals.var_rev11_dn2 = assign31400_body84_e32252_d_n2;
            locals.var_rev11_dn4 = assign31400_body84_e32252_d_n4;
            locals.var_rev11_dn5 = assign31400_body84_e32252_d_n5;
            locals.var_rev11_dn6 = assign31400_body84_e32252_d_n6;
            locals.var_rev11_dn7 = assign31400_body84_e32252_d_n7;
            locals.var_rev11_dn8 = assign31400_body84_e32252_d_n8;
            locals.var_rev11_dn9 = assign31400_body84_e32252_d_n9;
            locals.var_rev11_dn10 = assign31400_body84_e32252_d_n10;
            locals.var_rev11_dn11 = assign31400_body84_e32252_d_n11;
            locals.var_rev11_dn14 = assign31400_body84_e32252_d_n14;
            let (assign31400_body85_e32266, assign31400_body85_e32266_d_n0, assign31400_body85_e32266_d_n2, assign31400_body85_e32266_d_n4, assign31400_body85_e32266_d_n5, assign31400_body85_e32266_d_n6, assign31400_body85_e32266_d_n7, assign31400_body85_e32266_d_n8, assign31400_body85_e32266_d_n9, assign31400_body85_e32266_d_n10, assign31400_body85_e32266_d_n11, assign31400_body85_e32266_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body85_e32262: f64 = (-locals.var_y12);
        let assign31400_body85_e32264: f64 = (assign31400_body85_e32262 / locals.var_dety);
        (assign31400_body85_e32264, ((((-locals.var_y12_dn0) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn2) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn4) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn5) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn6) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn7) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn8) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn9) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn10) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn11) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn14) * locals.var_dety) - (assign31400_body85_e32262 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev12, locals.var_rev12_dn0, locals.var_rev12_dn2, locals.var_rev12_dn4, locals.var_rev12_dn5, locals.var_rev12_dn6, locals.var_rev12_dn7, locals.var_rev12_dn8, locals.var_rev12_dn9, locals.var_rev12_dn10, locals.var_rev12_dn11, locals.var_rev12_dn14,)
    }
};
            locals.var_rev12 = assign31400_body85_e32266;
            locals.var_rev12_dn0 = assign31400_body85_e32266_d_n0;
            locals.var_rev12_dn2 = assign31400_body85_e32266_d_n2;
            locals.var_rev12_dn4 = assign31400_body85_e32266_d_n4;
            locals.var_rev12_dn5 = assign31400_body85_e32266_d_n5;
            locals.var_rev12_dn6 = assign31400_body85_e32266_d_n6;
            locals.var_rev12_dn7 = assign31400_body85_e32266_d_n7;
            locals.var_rev12_dn8 = assign31400_body85_e32266_d_n8;
            locals.var_rev12_dn9 = assign31400_body85_e32266_d_n9;
            locals.var_rev12_dn10 = assign31400_body85_e32266_d_n10;
            locals.var_rev12_dn11 = assign31400_body85_e32266_d_n11;
            locals.var_rev12_dn14 = assign31400_body85_e32266_d_n14;
            let (assign31400_body86_e32280, assign31400_body86_e32280_d_n0, assign31400_body86_e32280_d_n2, assign31400_body86_e32280_d_n4, assign31400_body86_e32280_d_n5, assign31400_body86_e32280_d_n6, assign31400_body86_e32280_d_n7, assign31400_body86_e32280_d_n8, assign31400_body86_e32280_d_n9, assign31400_body86_e32280_d_n10, assign31400_body86_e32280_d_n11, assign31400_body86_e32280_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body86_e32276: f64 = (-locals.var_y21);
        let assign31400_body86_e32278: f64 = (assign31400_body86_e32276 / locals.var_dety);
        (assign31400_body86_e32278, ((((-locals.var_y21_dn0) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn2) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn4) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn5) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn6) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn7) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn8) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn9) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn10) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn11) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn14) * locals.var_dety) - (assign31400_body86_e32276 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev21, locals.var_rev21_dn0, locals.var_rev21_dn2, locals.var_rev21_dn4, locals.var_rev21_dn5, locals.var_rev21_dn6, locals.var_rev21_dn7, locals.var_rev21_dn8, locals.var_rev21_dn9, locals.var_rev21_dn10, locals.var_rev21_dn11, locals.var_rev21_dn14,)
    }
};
            locals.var_rev21 = assign31400_body86_e32280;
            locals.var_rev21_dn0 = assign31400_body86_e32280_d_n0;
            locals.var_rev21_dn2 = assign31400_body86_e32280_d_n2;
            locals.var_rev21_dn4 = assign31400_body86_e32280_d_n4;
            locals.var_rev21_dn5 = assign31400_body86_e32280_d_n5;
            locals.var_rev21_dn6 = assign31400_body86_e32280_d_n6;
            locals.var_rev21_dn7 = assign31400_body86_e32280_d_n7;
            locals.var_rev21_dn8 = assign31400_body86_e32280_d_n8;
            locals.var_rev21_dn9 = assign31400_body86_e32280_d_n9;
            locals.var_rev21_dn10 = assign31400_body86_e32280_d_n10;
            locals.var_rev21_dn11 = assign31400_body86_e32280_d_n11;
            locals.var_rev21_dn14 = assign31400_body86_e32280_d_n14;
            let (assign31400_body87_e32293, assign31400_body87_e32293_d_n0, assign31400_body87_e32293_d_n2, assign31400_body87_e32293_d_n4, assign31400_body87_e32293_d_n5, assign31400_body87_e32293_d_n6, assign31400_body87_e32293_d_n7, assign31400_body87_e32293_d_n8, assign31400_body87_e32293_d_n9, assign31400_body87_e32293_d_n10, assign31400_body87_e32293_d_n11, assign31400_body87_e32293_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body87_e32291: f64 = (locals.var_y11 / locals.var_dety);
        (assign31400_body87_e32291, (((locals.var_y11_dn0 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn2 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn4 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn5 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn6 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn7 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn8 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn9 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn10 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn11 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn14 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev22, locals.var_rev22_dn0, locals.var_rev22_dn2, locals.var_rev22_dn4, locals.var_rev22_dn5, locals.var_rev22_dn6, locals.var_rev22_dn7, locals.var_rev22_dn8, locals.var_rev22_dn9, locals.var_rev22_dn10, locals.var_rev22_dn11, locals.var_rev22_dn14,)
    }
};
            locals.var_rev22 = assign31400_body87_e32293;
            locals.var_rev22_dn0 = assign31400_body87_e32293_d_n0;
            locals.var_rev22_dn2 = assign31400_body87_e32293_d_n2;
            locals.var_rev22_dn4 = assign31400_body87_e32293_d_n4;
            locals.var_rev22_dn5 = assign31400_body87_e32293_d_n5;
            locals.var_rev22_dn6 = assign31400_body87_e32293_d_n6;
            locals.var_rev22_dn7 = assign31400_body87_e32293_d_n7;
            locals.var_rev22_dn8 = assign31400_body87_e32293_d_n8;
            locals.var_rev22_dn9 = assign31400_body87_e32293_d_n9;
            locals.var_rev22_dn10 = assign31400_body87_e32293_d_n10;
            locals.var_rev22_dn11 = assign31400_body87_e32293_d_n11;
            locals.var_rev22_dn14 = assign31400_body87_e32293_d_n14;
            let assign31400_body88_e32296: f64 = (locals.var_rev11 * locals.var_y1);
            let assign31400_body88_e32299: f64 = (locals.var_rev12 * locals.var_y2);
            let assign31400_body88_e32300: f64 = (assign31400_body88_e32296 + assign31400_body88_e32299);
            let assign31400_body88_e32301: f64 = (assign31400_body88_e32300).abs();
            let assign31400_body88_e32303: f64 = if assign31400_body88_e32301 > 0.5 { 1.0 } else { 0.0 };
            locals.var_guard729 = assign31400_body88_e32303;
            let (assign31400_body89_e32332, assign31400_body89_e32332_d_n0, assign31400_body89_e32332_d_n2, assign31400_body89_e32332_d_n4, assign31400_body89_e32332_d_n5, assign31400_body89_e32332_d_n6, assign31400_body89_e32332_d_n7, assign31400_body89_e32332_d_n8, assign31400_body89_e32332_d_n9, assign31400_body89_e32332_d_n10, assign31400_body89_e32332_d_n11, assign31400_body89_e32332_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard729 != 0.0)) {
        let assign31400_body89_e32318: f64 = (locals.var_rev11 * locals.var_y1);
        let assign31400_body89_e32321: f64 = (locals.var_rev12 * locals.var_y2);
        let assign31400_body89_e32322: f64 = (assign31400_body89_e32318 + assign31400_body89_e32321);
        let (assign31400_body89_e32328,) = {
            if (assign31400_body89_e32322 >= 0.0) {
                (1.0,)
            } else {
                let assign31400_body89_e32327: f64 = (-1.0);
                (assign31400_body89_e32327,)
            }
        };
        let assign31400_body89_e32329: f64 = (0.5 * assign31400_body89_e32328);
        let assign31400_body89_e32330: f64 = (locals.var_vgp0 - assign31400_body89_e32329);
        (assign31400_body89_e32330, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
            locals.var_vgp0 = assign31400_body89_e32332;
            locals.var_vgp0_dn0 = assign31400_body89_e32332_d_n0;
            locals.var_vgp0_dn2 = assign31400_body89_e32332_d_n2;
            locals.var_vgp0_dn4 = assign31400_body89_e32332_d_n4;
            locals.var_vgp0_dn5 = assign31400_body89_e32332_d_n5;
            locals.var_vgp0_dn6 = assign31400_body89_e32332_d_n6;
            locals.var_vgp0_dn7 = assign31400_body89_e32332_d_n7;
            locals.var_vgp0_dn8 = assign31400_body89_e32332_d_n8;
            locals.var_vgp0_dn9 = assign31400_body89_e32332_d_n9;
            locals.var_vgp0_dn10 = assign31400_body89_e32332_d_n10;
            locals.var_vgp0_dn11 = assign31400_body89_e32332_d_n11;
            locals.var_vgp0_dn14 = assign31400_body89_e32332_d_n14;
            let (assign31400_body90_e32361, assign31400_body90_e32361_d_n0, assign31400_body90_e32361_d_n2, assign31400_body90_e32361_d_n4, assign31400_body90_e32361_d_n5, assign31400_body90_e32361_d_n6, assign31400_body90_e32361_d_n7, assign31400_body90_e32361_d_n8, assign31400_body90_e32361_d_n9, assign31400_body90_e32361_d_n10, assign31400_body90_e32361_d_n11, assign31400_body90_e32361_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard729 != 0.0)) {
        let assign31400_body90_e32347: f64 = (locals.var_rev21 * locals.var_y1);
        let assign31400_body90_e32350: f64 = (locals.var_rev22 * locals.var_y2);
        let assign31400_body90_e32351: f64 = (assign31400_body90_e32347 + assign31400_body90_e32350);
        let (assign31400_body90_e32357,) = {
            if (assign31400_body90_e32351 >= 0.0) {
                (1.0,)
            } else {
                let assign31400_body90_e32356: f64 = (-1.0);
                (assign31400_body90_e32356,)
            }
        };
        let assign31400_body90_e32358: f64 = (0.5 * assign31400_body90_e32357);
        let assign31400_body90_e32359: f64 = (locals.var_phi_jl_dep - assign31400_body90_e32358);
        (assign31400_body90_e32359, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
            locals.var_phi_jl_dep = assign31400_body90_e32361;
            locals.var_phi_jl_dep_dn0 = assign31400_body90_e32361_d_n0;
            locals.var_phi_jl_dep_dn2 = assign31400_body90_e32361_d_n2;
            locals.var_phi_jl_dep_dn4 = assign31400_body90_e32361_d_n4;
            locals.var_phi_jl_dep_dn5 = assign31400_body90_e32361_d_n5;
            locals.var_phi_jl_dep_dn6 = assign31400_body90_e32361_d_n6;
            locals.var_phi_jl_dep_dn7 = assign31400_body90_e32361_d_n7;
            locals.var_phi_jl_dep_dn8 = assign31400_body90_e32361_d_n8;
            locals.var_phi_jl_dep_dn9 = assign31400_body90_e32361_d_n9;
            locals.var_phi_jl_dep_dn10 = assign31400_body90_e32361_d_n10;
            locals.var_phi_jl_dep_dn11 = assign31400_body90_e32361_d_n11;
            locals.var_phi_jl_dep_dn14 = assign31400_body90_e32361_d_n14;
            let (assign31400_body91_e32383, assign31400_body91_e32383_d_n0, assign31400_body91_e32383_d_n2, assign31400_body91_e32383_d_n4, assign31400_body91_e32383_d_n5, assign31400_body91_e32383_d_n6, assign31400_body91_e32383_d_n7, assign31400_body91_e32383_d_n8, assign31400_body91_e32383_d_n9, assign31400_body91_e32383_d_n10, assign31400_body91_e32383_d_n11, assign31400_body91_e32383_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard729 == 0.0)) {
        let assign31400_body91_e32376: f64 = (locals.var_rev11 * locals.var_y1);
        let assign31400_body91_e32379: f64 = (locals.var_rev12 * locals.var_y2);
        let assign31400_body91_e32380: f64 = (assign31400_body91_e32376 + assign31400_body91_e32379);
        let assign31400_body91_e32381: f64 = (locals.var_vgp0 - assign31400_body91_e32380);
        (assign31400_body91_e32381, (locals.var_vgp0_dn0 - (((locals.var_rev11_dn0 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn0)) + ((locals.var_rev12_dn0 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn0)))), (locals.var_vgp0_dn2 - (((locals.var_rev11_dn2 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn2)) + ((locals.var_rev12_dn2 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn2)))), (locals.var_vgp0_dn4 - (((locals.var_rev11_dn4 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn4)) + ((locals.var_rev12_dn4 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn4)))), (locals.var_vgp0_dn5 - (((locals.var_rev11_dn5 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn5)) + ((locals.var_rev12_dn5 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn5)))), (locals.var_vgp0_dn6 - (((locals.var_rev11_dn6 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn6)) + ((locals.var_rev12_dn6 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn6)))), (locals.var_vgp0_dn7 - (((locals.var_rev11_dn7 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn7)) + ((locals.var_rev12_dn7 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn7)))), (locals.var_vgp0_dn8 - (((locals.var_rev11_dn8 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn8)) + ((locals.var_rev12_dn8 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn8)))), (locals.var_vgp0_dn9 - (((locals.var_rev11_dn9 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn9)) + ((locals.var_rev12_dn9 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn9)))), (locals.var_vgp0_dn10 - (((locals.var_rev11_dn10 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn10)) + ((locals.var_rev12_dn10 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn10)))), (locals.var_vgp0_dn11 - (((locals.var_rev11_dn11 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn11)) + ((locals.var_rev12_dn11 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn11)))), (locals.var_vgp0_dn14 - (((locals.var_rev11_dn14 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn14)) + ((locals.var_rev12_dn14 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn14)))),)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
            locals.var_vgp0 = assign31400_body91_e32383;
            locals.var_vgp0_dn0 = assign31400_body91_e32383_d_n0;
            locals.var_vgp0_dn2 = assign31400_body91_e32383_d_n2;
            locals.var_vgp0_dn4 = assign31400_body91_e32383_d_n4;
            locals.var_vgp0_dn5 = assign31400_body91_e32383_d_n5;
            locals.var_vgp0_dn6 = assign31400_body91_e32383_d_n6;
            locals.var_vgp0_dn7 = assign31400_body91_e32383_d_n7;
            locals.var_vgp0_dn8 = assign31400_body91_e32383_d_n8;
            locals.var_vgp0_dn9 = assign31400_body91_e32383_d_n9;
            locals.var_vgp0_dn10 = assign31400_body91_e32383_d_n10;
            locals.var_vgp0_dn11 = assign31400_body91_e32383_d_n11;
            locals.var_vgp0_dn14 = assign31400_body91_e32383_d_n14;
            let (assign31400_body92_e32405, assign31400_body92_e32405_d_n0, assign31400_body92_e32405_d_n2, assign31400_body92_e32405_d_n4, assign31400_body92_e32405_d_n5, assign31400_body92_e32405_d_n6, assign31400_body92_e32405_d_n7, assign31400_body92_e32405_d_n8, assign31400_body92_e32405_d_n9, assign31400_body92_e32405_d_n10, assign31400_body92_e32405_d_n11, assign31400_body92_e32405_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard729 == 0.0)) {
        let assign31400_body92_e32398: f64 = (locals.var_rev21 * locals.var_y1);
        let assign31400_body92_e32401: f64 = (locals.var_rev22 * locals.var_y2);
        let assign31400_body92_e32402: f64 = (assign31400_body92_e32398 + assign31400_body92_e32401);
        let assign31400_body92_e32403: f64 = (locals.var_phi_jl_dep - assign31400_body92_e32402);
        (assign31400_body92_e32403, (locals.var_phi_jl_dep_dn0 - (((locals.var_rev21_dn0 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn0)) + ((locals.var_rev22_dn0 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn0)))), (locals.var_phi_jl_dep_dn2 - (((locals.var_rev21_dn2 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn2)) + ((locals.var_rev22_dn2 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn2)))), (locals.var_phi_jl_dep_dn4 - (((locals.var_rev21_dn4 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn4)) + ((locals.var_rev22_dn4 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn4)))), (locals.var_phi_jl_dep_dn5 - (((locals.var_rev21_dn5 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn5)) + ((locals.var_rev22_dn5 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn5)))), (locals.var_phi_jl_dep_dn6 - (((locals.var_rev21_dn6 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn6)) + ((locals.var_rev22_dn6 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn6)))), (locals.var_phi_jl_dep_dn7 - (((locals.var_rev21_dn7 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn7)) + ((locals.var_rev22_dn7 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn7)))), (locals.var_phi_jl_dep_dn8 - (((locals.var_rev21_dn8 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn8)) + ((locals.var_rev22_dn8 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn8)))), (locals.var_phi_jl_dep_dn9 - (((locals.var_rev21_dn9 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn9)) + ((locals.var_rev22_dn9 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn9)))), (locals.var_phi_jl_dep_dn10 - (((locals.var_rev21_dn10 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn10)) + ((locals.var_rev22_dn10 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn10)))), (locals.var_phi_jl_dep_dn11 - (((locals.var_rev21_dn11 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn11)) + ((locals.var_rev22_dn11 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn11)))), (locals.var_phi_jl_dep_dn14 - (((locals.var_rev21_dn14 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn14)) + ((locals.var_rev22_dn14 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn14)))),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
            locals.var_phi_jl_dep = assign31400_body92_e32405;
            locals.var_phi_jl_dep_dn0 = assign31400_body92_e32405_d_n0;
            locals.var_phi_jl_dep_dn2 = assign31400_body92_e32405_d_n2;
            locals.var_phi_jl_dep_dn4 = assign31400_body92_e32405_d_n4;
            locals.var_phi_jl_dep_dn5 = assign31400_body92_e32405_d_n5;
            locals.var_phi_jl_dep_dn6 = assign31400_body92_e32405_d_n6;
            locals.var_phi_jl_dep_dn7 = assign31400_body92_e32405_d_n7;
            locals.var_phi_jl_dep_dn8 = assign31400_body92_e32405_d_n8;
            locals.var_phi_jl_dep_dn9 = assign31400_body92_e32405_d_n9;
            locals.var_phi_jl_dep_dn10 = assign31400_body92_e32405_d_n10;
            locals.var_phi_jl_dep_dn11 = assign31400_body92_e32405_d_n11;
            locals.var_phi_jl_dep_dn14 = assign31400_body92_e32405_d_n14;
            let assign31400_body93_e32408: f64 = (locals.var_vgp0 - locals.var_vgp0old);
            let assign31400_body93_e32409: f64 = (assign31400_body93_e32408).abs();
            let assign31400_body93_e32414: f64 = (locals.var_phi_jl_dep - locals.var_phi_jl_dep_old);
            let assign31400_body93_e32415: f64 = (assign31400_body93_e32414).abs();
            let assign31400_body93_e32418: f64 = if ((assign31400_body93_e32409 <= 1e-12) && (assign31400_body93_e32415 <= 1e-12)) { 1.0 } else { 0.0 };
            locals.var_guard730 = assign31400_body93_e32418;
            let (assign31400_body94_e32433,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard730 != 0.0)) {
        let assign31400_body94_e32431: f64 = (150.0 + 1.0);
        (assign31400_body94_e32431,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31400_body94_e32433;
            let (assign31400_body95_e32444,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_vgp0,)
    } else {
        (locals.var_vgp0old,)
    }
};
            locals.var_vgp0old = assign31400_body95_e32444;
            let (assign31400_body96_e32455,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_phi_jl_dep,)
    } else {
        (locals.var_phi_jl_dep_old,)
    }
};
            locals.var_phi_jl_dep_old = assign31400_body96_e32455;
            let (assign31400_body97_e32468,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31400_body97_e32466: f64 = (locals.var_lp_s0 + 1.0);
        (assign31400_body97_e32466,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31400_body97_e32468;
        }

    }

    pub(super) fn stamp_transient_block_93(
        locals: &mut StampLocals,
    ) {
        let (assign31410_e32479, assign31410_e32479_d_n0, assign31410_e32479_d_n2, assign31410_e32479_d_n4, assign31410_e32479_d_n5, assign31410_e32479_d_n6, assign31410_e32479_d_n7, assign31410_e32479_d_n8, assign31410_e32479_d_n9, assign31410_e32479_d_n10, assign31410_e32479_d_n11, assign31410_e32479_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    } else {
        (locals.var_phi_jl_dep_acc, locals.var_phi_jl_dep_acc_dn0, locals.var_phi_jl_dep_acc_dn2, locals.var_phi_jl_dep_acc_dn4, locals.var_phi_jl_dep_acc_dn5, locals.var_phi_jl_dep_acc_dn6, locals.var_phi_jl_dep_acc_dn7, locals.var_phi_jl_dep_acc_dn8, locals.var_phi_jl_dep_acc_dn9, locals.var_phi_jl_dep_acc_dn10, locals.var_phi_jl_dep_acc_dn11, locals.var_phi_jl_dep_acc_dn14,)
    }
};
        locals.var_phi_jl_dep_acc = assign31410_e32479;
        locals.var_phi_jl_dep_acc_dn0 = assign31410_e32479_d_n0;
        locals.var_phi_jl_dep_acc_dn2 = assign31410_e32479_d_n2;
        locals.var_phi_jl_dep_acc_dn4 = assign31410_e32479_d_n4;
        locals.var_phi_jl_dep_acc_dn5 = assign31410_e32479_d_n5;
        locals.var_phi_jl_dep_acc_dn6 = assign31410_e32479_d_n6;
        locals.var_phi_jl_dep_acc_dn7 = assign31410_e32479_d_n7;
        locals.var_phi_jl_dep_acc_dn8 = assign31410_e32479_d_n8;
        locals.var_phi_jl_dep_acc_dn9 = assign31410_e32479_d_n9;
        locals.var_phi_jl_dep_acc_dn10 = assign31410_e32479_d_n10;
        locals.var_phi_jl_dep_acc_dn11 = assign31410_e32479_d_n11;
        locals.var_phi_jl_dep_acc_dn14 = assign31410_e32479_d_n14;

        let (assign31420_e32492, assign31420_e32492_d_n0, assign31420_e32492_d_n2, assign31420_e32492_d_n4, assign31420_e32492_d_n5, assign31420_e32492_d_n6, assign31420_e32492_d_n7, assign31420_e32492_d_n8, assign31420_e32492_d_n9, assign31420_e32492_d_n10, assign31420_e32492_d_n11, assign31420_e32492_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31420_e32490: f64 = (locals.var_uc_depthn * locals.var_ndepmpnsub);
        (assign31420_e32490, ((locals.var_uc_depthn_dn0 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn0)), ((locals.var_uc_depthn_dn2 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn2)), ((locals.var_uc_depthn_dn4 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn4)), ((locals.var_uc_depthn_dn5 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn5)), ((locals.var_uc_depthn_dn6 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn6)), ((locals.var_uc_depthn_dn7 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn7)), ((locals.var_uc_depthn_dn8 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn8)), ((locals.var_uc_depthn_dn9 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn9)), ((locals.var_uc_depthn_dn10 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn10)), ((locals.var_uc_depthn_dn11 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn11)), ((locals.var_uc_depthn_dn14 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn14)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn11, locals.var_w_subl_dn14,)
    }
};
        locals.var_w_subl = assign31420_e32492;
        locals.var_w_subl_dn0 = assign31420_e32492_d_n0;
        locals.var_w_subl_dn2 = assign31420_e32492_d_n2;
        locals.var_w_subl_dn4 = assign31420_e32492_d_n4;
        locals.var_w_subl_dn5 = assign31420_e32492_d_n5;
        locals.var_w_subl_dn6 = assign31420_e32492_d_n6;
        locals.var_w_subl_dn7 = assign31420_e32492_d_n7;
        locals.var_w_subl_dn8 = assign31420_e32492_d_n8;
        locals.var_w_subl_dn9 = assign31420_e32492_d_n9;
        locals.var_w_subl_dn10 = assign31420_e32492_d_n10;
        locals.var_w_subl_dn11 = assign31420_e32492_d_n11;
        locals.var_w_subl_dn14 = assign31420_e32492_d_n14;

        let (assign31430_e32511, assign31430_e32511_d_n0, assign31430_e32511_d_n2, assign31430_e32511_d_n4, assign31430_e32511_d_n5, assign31430_e32511_d_n6, assign31430_e32511_d_n7, assign31430_e32511_d_n8, assign31430_e32511_d_n9, assign31430_e32511_d_n10, assign31430_e32511_d_n11, assign31430_e32511_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31430_e32503: f64 = (locals.var_c_2esipq_nsub_inv * locals.var_w_subl);
        let assign31430_e32505: f64 = (assign31430_e32503 * locals.var_w_subl);
        let assign31430_e32507: f64 = (assign31430_e32505 + locals.var_vbscl__blk439);
        let assign31430_e32509: f64 = (assign31430_e32507 - locals.var_vbi_dep);
        (assign31430_e32509, ((((((locals.var_c_2esipq_nsub_inv_dn0 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn0)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn0)) + locals.var_vbscl__blk439_dn0) - locals.var_vbi_dep_dn0), ((((((locals.var_c_2esipq_nsub_inv_dn2 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn2)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn2)) + locals.var_vbscl__blk439_dn2) - locals.var_vbi_dep_dn2), ((((((locals.var_c_2esipq_nsub_inv_dn4 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn4)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn4)) + locals.var_vbscl__blk439_dn4) - locals.var_vbi_dep_dn4), ((((((locals.var_c_2esipq_nsub_inv_dn5 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn5)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn5)) + locals.var_vbscl__blk439_dn5) - locals.var_vbi_dep_dn5), ((((((locals.var_c_2esipq_nsub_inv_dn6 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn6)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn6)) + locals.var_vbscl__blk439_dn6) - locals.var_vbi_dep_dn6), ((((((locals.var_c_2esipq_nsub_inv_dn7 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn7)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn7)) + locals.var_vbscl__blk439_dn7) - locals.var_vbi_dep_dn7), ((((((locals.var_c_2esipq_nsub_inv_dn8 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn8)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn8)) + locals.var_vbscl__blk439_dn8) - locals.var_vbi_dep_dn8), ((((((locals.var_c_2esipq_nsub_inv_dn9 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn9)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn9)) + locals.var_vbscl__blk439_dn9) - locals.var_vbi_dep_dn9), ((((((locals.var_c_2esipq_nsub_inv_dn10 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn10)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn10)) + locals.var_vbscl__blk439_dn10) - locals.var_vbi_dep_dn10), ((((((locals.var_c_2esipq_nsub_inv_dn11 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn11)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn11)) + locals.var_vbscl__blk439_dn11) - locals.var_vbi_dep_dn11), ((((((locals.var_c_2esipq_nsub_inv_dn14 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn14)) * locals.var_w_subl) + (assign31430_e32503 * locals.var_w_subl_dn14)) + locals.var_vbscl__blk439_dn14) - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31430_e32511;
        locals.var_phi_jl_dep_dn0 = assign31430_e32511_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31430_e32511_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31430_e32511_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31430_e32511_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31430_e32511_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31430_e32511_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31430_e32511_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31430_e32511_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31430_e32511_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31430_e32511_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31430_e32511_d_n14;

        let (assign31440_e32526, assign31440_e32526_d_n0, assign31440_e32526_d_n2, assign31440_e32526_d_n4, assign31440_e32526_d_n5, assign31440_e32526_d_n6, assign31440_e32526_d_n7, assign31440_e32526_d_n8, assign31440_e32526_d_n9, assign31440_e32526_d_n10, assign31440_e32526_d_n11, assign31440_e32526_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31440_e32523: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_tn2);
        let assign31440_e32524: f64 = (locals.var_phi_jl_dep + assign31440_e32523);
        (assign31440_e32524, (locals.var_phi_jl_dep_dn0 + ((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn0))), (locals.var_phi_jl_dep_dn2 + ((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn2))), (locals.var_phi_jl_dep_dn4 + ((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn4))), (locals.var_phi_jl_dep_dn5 + ((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn5))), (locals.var_phi_jl_dep_dn6 + ((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn6))), (locals.var_phi_jl_dep_dn7 + ((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn7))), (locals.var_phi_jl_dep_dn8 + ((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn8))), (locals.var_phi_jl_dep_dn9 + ((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn9))), (locals.var_phi_jl_dep_dn10 + ((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn10))), (locals.var_phi_jl_dep_dn11 + ((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn11))), (locals.var_phi_jl_dep_dn14 + ((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn14))),)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31440_e32526;
        locals.var_phi_bl_dep_dn0 = assign31440_e32526_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31440_e32526_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31440_e32526_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31440_e32526_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31440_e32526_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31440_e32526_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31440_e32526_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31440_e32526_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31440_e32526_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31440_e32526_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31440_e32526_d_n14;

        let (assign31450_e32537, assign31450_e32537_d_n0, assign31450_e32537_d_n2, assign31450_e32537_d_n4, assign31450_e32537_d_n5, assign31450_e32537_d_n6, assign31450_e32537_d_n7, assign31450_e32537_d_n8, assign31450_e32537_d_n9, assign31450_e32537_d_n10, assign31450_e32537_d_n11, assign31450_e32537_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    }
};
        locals.var_phi_sl_dep = assign31450_e32537;
        locals.var_phi_sl_dep_dn0 = assign31450_e32537_d_n0;
        locals.var_phi_sl_dep_dn2 = assign31450_e32537_d_n2;
        locals.var_phi_sl_dep_dn4 = assign31450_e32537_d_n4;
        locals.var_phi_sl_dep_dn5 = assign31450_e32537_d_n5;
        locals.var_phi_sl_dep_dn6 = assign31450_e32537_d_n6;
        locals.var_phi_sl_dep_dn7 = assign31450_e32537_d_n7;
        locals.var_phi_sl_dep_dn8 = assign31450_e32537_d_n8;
        locals.var_phi_sl_dep_dn9 = assign31450_e32537_d_n9;
        locals.var_phi_sl_dep_dn10 = assign31450_e32537_d_n10;
        locals.var_phi_sl_dep_dn11 = assign31450_e32537_d_n11;
        locals.var_phi_sl_dep_dn14 = assign31450_e32537_d_n14;

        let (assign31460_e32548, assign31460_e32548_d_n0, assign31460_e32548_d_n2, assign31460_e32548_d_n4, assign31460_e32548_d_n5, assign31460_e32548_d_n6, assign31460_e32548_d_n7, assign31460_e32548_d_n8, assign31460_e32548_d_n9, assign31460_e32548_d_n10, assign31460_e32548_d_n11, assign31460_e32548_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    }
};
        locals.var_psbmax = assign31460_e32548;
        locals.var_psbmax_dn0 = assign31460_e32548_d_n0;
        locals.var_psbmax_dn2 = assign31460_e32548_d_n2;
        locals.var_psbmax_dn4 = assign31460_e32548_d_n4;
        locals.var_psbmax_dn5 = assign31460_e32548_d_n5;
        locals.var_psbmax_dn6 = assign31460_e32548_d_n6;
        locals.var_psbmax_dn7 = assign31460_e32548_d_n7;
        locals.var_psbmax_dn8 = assign31460_e32548_d_n8;
        locals.var_psbmax_dn9 = assign31460_e32548_d_n9;
        locals.var_psbmax_dn10 = assign31460_e32548_d_n10;
        locals.var_psbmax_dn11 = assign31460_e32548_d_n11;
        locals.var_psbmax_dn14 = assign31460_e32548_d_n14;

        let (assign31470_e32559,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        (locals.var_phi_bl_dep,)
    } else {
        (locals.var_vgp1,)
    }
};
        locals.var_vgp1 = assign31470_e32559;

        let assign31480_e32562: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard731 = assign31480_e32562;

        let (assign31490_e32575,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard731 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign31490_e32575;

        let assign31500_e32578: f64 = if locals.var_vgp > locals.var_vgp1 { 1.0 } else { 0.0 };
        locals.var_guard732 = assign31500_e32578;

        let (assign31510_e32594,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard731 == 0.0)) && (locals.var_guard732 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign31510_e32594;

        let (assign31520_e32611,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard731 == 0.0)) && (locals.var_guard732 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign31520_e32611;

        let (assign31530_e32623, assign31530_e32623_d_n0, assign31530_e32623_d_n2, assign31530_e32623_d_n4, assign31530_e32623_d_n5, assign31530_e32623_d_n6, assign31530_e32623_d_n7, assign31530_e32623_d_n8, assign31530_e32623_d_n9, assign31530_e32623_d_n10, assign31530_e32623_d_n11, assign31530_e32623_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
        locals.var_vgp0 = assign31530_e32623;
        locals.var_vgp0_dn0 = assign31530_e32623_d_n0;
        locals.var_vgp0_dn2 = assign31530_e32623_d_n2;
        locals.var_vgp0_dn4 = assign31530_e32623_d_n4;
        locals.var_vgp0_dn5 = assign31530_e32623_d_n5;
        locals.var_vgp0_dn6 = assign31530_e32623_d_n6;
        locals.var_vgp0_dn7 = assign31530_e32623_d_n7;
        locals.var_vgp0_dn8 = assign31530_e32623_d_n8;
        locals.var_vgp0_dn9 = assign31530_e32623_d_n9;
        locals.var_vgp0_dn10 = assign31530_e32623_d_n10;
        locals.var_vgp0_dn11 = assign31530_e32623_d_n11;
        locals.var_vgp0_dn14 = assign31530_e32623_d_n14;

        let (assign31540_e32635,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        (locals.var_vgp0,)
    } else {
        (locals.var_vgp1,)
    }
};
        locals.var_vgp1 = assign31540_e32635;

        let (assign31550_e32647, assign31550_e32647_d_n0, assign31550_e32647_d_n2, assign31550_e32647_d_n4, assign31550_e32647_d_n5, assign31550_e32647_d_n6, assign31550_e32647_d_n7, assign31550_e32647_d_n8, assign31550_e32647_d_n9, assign31550_e32647_d_n10, assign31550_e32647_d_n11, assign31550_e32647_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    }
};
        locals.var_psbmax = assign31550_e32647;
        locals.var_psbmax_dn0 = assign31550_e32647_d_n0;
        locals.var_psbmax_dn2 = assign31550_e32647_d_n2;
        locals.var_psbmax_dn4 = assign31550_e32647_d_n4;
        locals.var_psbmax_dn5 = assign31550_e32647_d_n5;
        locals.var_psbmax_dn6 = assign31550_e32647_d_n6;
        locals.var_psbmax_dn7 = assign31550_e32647_d_n7;
        locals.var_psbmax_dn8 = assign31550_e32647_d_n8;
        locals.var_psbmax_dn9 = assign31550_e32647_d_n9;
        locals.var_psbmax_dn10 = assign31550_e32647_d_n10;
        locals.var_psbmax_dn11 = assign31550_e32647_d_n11;
        locals.var_psbmax_dn14 = assign31550_e32647_d_n14;

        let (assign31560_e32659, assign31560_e32659_d_n0, assign31560_e32659_d_n2, assign31560_e32659_d_n4, assign31560_e32659_d_n5, assign31560_e32659_d_n6, assign31560_e32659_d_n7, assign31560_e32659_d_n8, assign31560_e32659_d_n9, assign31560_e32659_d_n10, assign31560_e32659_d_n11, assign31560_e32659_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vds_maxbl, locals.var_vds_maxbl_dn0, locals.var_vds_maxbl_dn2, locals.var_vds_maxbl_dn4, locals.var_vds_maxbl_dn5, locals.var_vds_maxbl_dn6, locals.var_vds_maxbl_dn7, locals.var_vds_maxbl_dn8, locals.var_vds_maxbl_dn9, locals.var_vds_maxbl_dn10, locals.var_vds_maxbl_dn11, locals.var_vds_maxbl_dn14,)
    }
};
        locals.var_vds_maxbl = assign31560_e32659;
        locals.var_vds_maxbl_dn0 = assign31560_e32659_d_n0;
        locals.var_vds_maxbl_dn2 = assign31560_e32659_d_n2;
        locals.var_vds_maxbl_dn4 = assign31560_e32659_d_n4;
        locals.var_vds_maxbl_dn5 = assign31560_e32659_d_n5;
        locals.var_vds_maxbl_dn6 = assign31560_e32659_d_n6;
        locals.var_vds_maxbl_dn7 = assign31560_e32659_d_n7;
        locals.var_vds_maxbl_dn8 = assign31560_e32659_d_n8;
        locals.var_vds_maxbl_dn9 = assign31560_e32659_d_n9;
        locals.var_vds_maxbl_dn10 = assign31560_e32659_d_n10;
        locals.var_vds_maxbl_dn11 = assign31560_e32659_d_n11;
        locals.var_vds_maxbl_dn14 = assign31560_e32659_d_n14;

        let (assign31570_e32671, assign31570_e32671_d_n0, assign31570_e32671_d_n2, assign31570_e32671_d_n4, assign31570_e32671_d_n5, assign31570_e32671_d_n6, assign31570_e32671_d_n7, assign31570_e32671_d_n8, assign31570_e32671_d_n9, assign31570_e32671_d_n10, assign31570_e32671_d_n11, assign31570_e32671_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        (locals.var_w_bsubl, locals.var_w_bsubl_dn0, locals.var_w_bsubl_dn2, locals.var_w_bsubl_dn4, locals.var_w_bsubl_dn5, locals.var_w_bsubl_dn6, locals.var_w_bsubl_dn7, locals.var_w_bsubl_dn8, locals.var_w_bsubl_dn9, locals.var_w_bsubl_dn10, locals.var_w_bsubl_dn11, locals.var_w_bsubl_dn14,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign31570_e32671;
        locals.var_w_bl_dn0 = assign31570_e32671_d_n0;
        locals.var_w_bl_dn2 = assign31570_e32671_d_n2;
        locals.var_w_bl_dn4 = assign31570_e32671_d_n4;
        locals.var_w_bl_dn5 = assign31570_e32671_d_n5;
        locals.var_w_bl_dn6 = assign31570_e32671_d_n6;
        locals.var_w_bl_dn7 = assign31570_e32671_d_n7;
        locals.var_w_bl_dn8 = assign31570_e32671_d_n8;
        locals.var_w_bl_dn9 = assign31570_e32671_d_n9;
        locals.var_w_bl_dn10 = assign31570_e32671_d_n10;
        locals.var_w_bl_dn11 = assign31570_e32671_d_n11;
        locals.var_w_bl_dn14 = assign31570_e32671_d_n14;

        let (assign31580_e32685, assign31580_e32685_d_n0, assign31580_e32685_d_n2, assign31580_e32685_d_n4, assign31580_e32685_d_n5, assign31580_e32685_d_n6, assign31580_e32685_d_n7, assign31580_e32685_d_n8, assign31580_e32685_d_n9, assign31580_e32685_d_n10, assign31580_e32685_d_n11, assign31580_e32685_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        let assign31580_e32683: f64 = (locals.var_w_bl * locals.var_ndepmpnsub);
        (assign31580_e32683, ((locals.var_w_bl_dn0 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn0)), ((locals.var_w_bl_dn2 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn2)), ((locals.var_w_bl_dn4 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn4)), ((locals.var_w_bl_dn5 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn5)), ((locals.var_w_bl_dn6 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn6)), ((locals.var_w_bl_dn7 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn7)), ((locals.var_w_bl_dn8 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn8)), ((locals.var_w_bl_dn9 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn9)), ((locals.var_w_bl_dn10 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn10)), ((locals.var_w_bl_dn11 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn11)), ((locals.var_w_bl_dn14 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn14)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn11, locals.var_w_subl_dn14,)
    }
};
        locals.var_w_subl = assign31580_e32685;
        locals.var_w_subl_dn0 = assign31580_e32685_d_n0;
        locals.var_w_subl_dn2 = assign31580_e32685_d_n2;
        locals.var_w_subl_dn4 = assign31580_e32685_d_n4;
        locals.var_w_subl_dn5 = assign31580_e32685_d_n5;
        locals.var_w_subl_dn6 = assign31580_e32685_d_n6;
        locals.var_w_subl_dn7 = assign31580_e32685_d_n7;
        locals.var_w_subl_dn8 = assign31580_e32685_d_n8;
        locals.var_w_subl_dn9 = assign31580_e32685_d_n9;
        locals.var_w_subl_dn10 = assign31580_e32685_d_n10;
        locals.var_w_subl_dn11 = assign31580_e32685_d_n11;
        locals.var_w_subl_dn14 = assign31580_e32685_d_n14;

        let (assign31590_e32705, assign31590_e32705_d_n0, assign31590_e32705_d_n2, assign31590_e32705_d_n4, assign31590_e32705_d_n5, assign31590_e32705_d_n6, assign31590_e32705_d_n7, assign31590_e32705_d_n8, assign31590_e32705_d_n9, assign31590_e32705_d_n10, assign31590_e32705_d_n11, assign31590_e32705_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        let assign31590_e32697: f64 = (locals.var_c_2esipq_nsub_inv * locals.var_w_subl);
        let assign31590_e32699: f64 = (assign31590_e32697 * locals.var_w_subl);
        let assign31590_e32701: f64 = (assign31590_e32699 + locals.var_vbscl__blk439);
        let assign31590_e32703: f64 = (assign31590_e32701 - locals.var_vbi_dep);
        (assign31590_e32703, ((((((locals.var_c_2esipq_nsub_inv_dn0 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn0)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn0)) + locals.var_vbscl__blk439_dn0) - locals.var_vbi_dep_dn0), ((((((locals.var_c_2esipq_nsub_inv_dn2 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn2)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn2)) + locals.var_vbscl__blk439_dn2) - locals.var_vbi_dep_dn2), ((((((locals.var_c_2esipq_nsub_inv_dn4 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn4)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn4)) + locals.var_vbscl__blk439_dn4) - locals.var_vbi_dep_dn4), ((((((locals.var_c_2esipq_nsub_inv_dn5 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn5)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn5)) + locals.var_vbscl__blk439_dn5) - locals.var_vbi_dep_dn5), ((((((locals.var_c_2esipq_nsub_inv_dn6 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn6)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn6)) + locals.var_vbscl__blk439_dn6) - locals.var_vbi_dep_dn6), ((((((locals.var_c_2esipq_nsub_inv_dn7 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn7)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn7)) + locals.var_vbscl__blk439_dn7) - locals.var_vbi_dep_dn7), ((((((locals.var_c_2esipq_nsub_inv_dn8 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn8)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn8)) + locals.var_vbscl__blk439_dn8) - locals.var_vbi_dep_dn8), ((((((locals.var_c_2esipq_nsub_inv_dn9 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn9)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn9)) + locals.var_vbscl__blk439_dn9) - locals.var_vbi_dep_dn9), ((((((locals.var_c_2esipq_nsub_inv_dn10 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn10)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn10)) + locals.var_vbscl__blk439_dn10) - locals.var_vbi_dep_dn10), ((((((locals.var_c_2esipq_nsub_inv_dn11 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn11)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn11)) + locals.var_vbscl__blk439_dn11) - locals.var_vbi_dep_dn11), ((((((locals.var_c_2esipq_nsub_inv_dn14 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn14)) * locals.var_w_subl) + (assign31590_e32697 * locals.var_w_subl_dn14)) + locals.var_vbscl__blk439_dn14) - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31590_e32705;
        locals.var_phi_jl_dep_dn0 = assign31590_e32705_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31590_e32705_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31590_e32705_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31590_e32705_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31590_e32705_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31590_e32705_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31590_e32705_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31590_e32705_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31590_e32705_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31590_e32705_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31590_e32705_d_n14;

        let (assign31600_e32723, assign31600_e32723_d_n0, assign31600_e32723_d_n2, assign31600_e32723_d_n4, assign31600_e32723_d_n5, assign31600_e32723_d_n6, assign31600_e32723_d_n7, assign31600_e32723_d_n8, assign31600_e32723_d_n9, assign31600_e32723_d_n10, assign31600_e32723_d_n11, assign31600_e32723_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        let assign31600_e32717: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl);
        let assign31600_e32719: f64 = (assign31600_e32717 * locals.var_w_bl);
        let assign31600_e32721: f64 = (assign31600_e32719 + locals.var_phi_jl_dep);
        (assign31600_e32721, (((((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn0)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn0)) + locals.var_phi_jl_dep_dn0), (((((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn2)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn2)) + locals.var_phi_jl_dep_dn2), (((((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn4)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn4)) + locals.var_phi_jl_dep_dn4), (((((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn5)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn5)) + locals.var_phi_jl_dep_dn5), (((((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn6)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn6)) + locals.var_phi_jl_dep_dn6), (((((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn7)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn7)) + locals.var_phi_jl_dep_dn7), (((((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn8)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn8)) + locals.var_phi_jl_dep_dn8), (((((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn9)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn9)) + locals.var_phi_jl_dep_dn9), (((((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn10)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn10)) + locals.var_phi_jl_dep_dn10), (((((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn11)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn11)) + locals.var_phi_jl_dep_dn11), (((((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn14)) * locals.var_w_bl) + (assign31600_e32717 * locals.var_w_bl_dn14)) + locals.var_phi_jl_dep_dn14),)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31600_e32723;
        locals.var_phi_bl_dep_dn0 = assign31600_e32723_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31600_e32723_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31600_e32723_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31600_e32723_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31600_e32723_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31600_e32723_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31600_e32723_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31600_e32723_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31600_e32723_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31600_e32723_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31600_e32723_d_n14;

        let (assign31610_e32735, assign31610_e32735_d_n0, assign31610_e32735_d_n2, assign31610_e32735_d_n4, assign31610_e32735_d_n5, assign31610_e32735_d_n6, assign31610_e32735_d_n7, assign31610_e32735_d_n8, assign31610_e32735_d_n9, assign31610_e32735_d_n10, assign31610_e32735_d_n11, assign31610_e32735_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    } else {
        (locals.var_phi_jl_dep_acc, locals.var_phi_jl_dep_acc_dn0, locals.var_phi_jl_dep_acc_dn2, locals.var_phi_jl_dep_acc_dn4, locals.var_phi_jl_dep_acc_dn5, locals.var_phi_jl_dep_acc_dn6, locals.var_phi_jl_dep_acc_dn7, locals.var_phi_jl_dep_acc_dn8, locals.var_phi_jl_dep_acc_dn9, locals.var_phi_jl_dep_acc_dn10, locals.var_phi_jl_dep_acc_dn11, locals.var_phi_jl_dep_acc_dn14,)
    }
};
        locals.var_phi_jl_dep_acc = assign31610_e32735;
        locals.var_phi_jl_dep_acc_dn0 = assign31610_e32735_d_n0;
        locals.var_phi_jl_dep_acc_dn2 = assign31610_e32735_d_n2;
        locals.var_phi_jl_dep_acc_dn4 = assign31610_e32735_d_n4;
        locals.var_phi_jl_dep_acc_dn5 = assign31610_e32735_d_n5;
        locals.var_phi_jl_dep_acc_dn6 = assign31610_e32735_d_n6;
        locals.var_phi_jl_dep_acc_dn7 = assign31610_e32735_d_n7;
        locals.var_phi_jl_dep_acc_dn8 = assign31610_e32735_d_n8;
        locals.var_phi_jl_dep_acc_dn9 = assign31610_e32735_d_n9;
        locals.var_phi_jl_dep_acc_dn10 = assign31610_e32735_d_n10;
        locals.var_phi_jl_dep_acc_dn11 = assign31610_e32735_d_n11;
        locals.var_phi_jl_dep_acc_dn14 = assign31610_e32735_d_n14;

        let assign31620_e32738: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard733 = assign31620_e32738;

        let (assign31630_e32752,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) && (locals.var_guard733 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign31630_e32752;

        let (assign31640_e32767,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) && (locals.var_guard733 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign31640_e32767;

        let (assign31650_e32783, assign31650_e32783_d_n0, assign31650_e32783_d_n2, assign31650_e32783_d_n4, assign31650_e32783_d_n5, assign31650_e32783_d_n6, assign31650_e32783_d_n7, assign31650_e32783_d_n8, assign31650_e32783_d_n9, assign31650_e32783_d_n10, assign31650_e32783_d_n11, assign31650_e32783_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) {
        let assign31650_e32777: f64 = (-locals.var_pb2n);
        let assign31650_e32779: f64 = (assign31650_e32777 + locals.var_vbscl__blk439);
        let assign31650_e32780: f64 = (locals.var_psbmax - assign31650_e32779);
        let assign31650_e32781: f64 = (locals.var_c_2esi_q_ndepm * assign31650_e32780);
        (assign31650_e32781, ((locals.var_c_2esi_q_ndepm_dn0 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn0 - ((-locals.var_pb2n_dn0) + locals.var_vbscl__blk439_dn0)))), ((locals.var_c_2esi_q_ndepm_dn2 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn2 - ((-locals.var_pb2n_dn2) + locals.var_vbscl__blk439_dn2)))), ((locals.var_c_2esi_q_ndepm_dn4 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn4 - ((-locals.var_pb2n_dn4) + locals.var_vbscl__blk439_dn4)))), ((locals.var_c_2esi_q_ndepm_dn5 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn5 - ((-locals.var_pb2n_dn5) + locals.var_vbscl__blk439_dn5)))), ((locals.var_c_2esi_q_ndepm_dn6 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn6 - ((-locals.var_pb2n_dn6) + locals.var_vbscl__blk439_dn6)))), ((locals.var_c_2esi_q_ndepm_dn7 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn7 - ((-locals.var_pb2n_dn7) + locals.var_vbscl__blk439_dn7)))), ((locals.var_c_2esi_q_ndepm_dn8 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn8 - ((-locals.var_pb2n_dn8) + locals.var_vbscl__blk439_dn8)))), ((locals.var_c_2esi_q_ndepm_dn9 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn9 - ((-locals.var_pb2n_dn9) + locals.var_vbscl__blk439_dn9)))), ((locals.var_c_2esi_q_ndepm_dn10 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn10 - ((-locals.var_pb2n_dn10) + locals.var_vbscl__blk439_dn10)))), ((locals.var_c_2esi_q_ndepm_dn11 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn11 - ((-locals.var_pb2n_dn11) + locals.var_vbscl__blk439_dn11)))), ((locals.var_c_2esi_q_ndepm_dn14 * assign31650_e32780) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn14 - ((-locals.var_pb2n_dn14) + locals.var_vbscl__blk439_dn14)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31650_e32783;
        locals.var_t1_dn0 = assign31650_e32783_d_n0;
        locals.var_t1_dn2 = assign31650_e32783_d_n2;
        locals.var_t1_dn4 = assign31650_e32783_d_n4;
        locals.var_t1_dn5 = assign31650_e32783_d_n5;
        locals.var_t1_dn6 = assign31650_e32783_d_n6;
        locals.var_t1_dn7 = assign31650_e32783_d_n7;
        locals.var_t1_dn8 = assign31650_e32783_d_n8;
        locals.var_t1_dn9 = assign31650_e32783_d_n9;
        locals.var_t1_dn10 = assign31650_e32783_d_n10;
        locals.var_t1_dn11 = assign31650_e32783_d_n11;
        locals.var_t1_dn14 = assign31650_e32783_d_n14;

        let assign31660_e32786: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard734 = assign31660_e32786;

        let (assign31670_e32805,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard734 != 0.0)) {
        let assign31670_e32796: f64 = (-locals.var_pb2n);
        let assign31670_e32798: f64 = (assign31670_e32796 + locals.var_vbscl__blk439);
        let assign31670_e32800: f64 = (locals.var_t1).sqrt();
        let assign31670_e32802: f64 = (assign31670_e32800 / locals.var_cox);
        let assign31670_e32803: f64 = (assign31670_e32798 - assign31670_e32802);
        (assign31670_e32803,)
    } else {
        (locals.var_vthn,)
    }
};
        locals.var_vthn = assign31670_e32805;

        let (assign31680_e32820,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard734 == 0.0)) {
        let assign31680_e32816: f64 = (-locals.var_pb2n);
        let assign31680_e32818: f64 = (assign31680_e32816 + locals.var_vbscl__blk439);
        (assign31680_e32818,)
    } else {
        (locals.var_vthn,)
    }
};
        locals.var_vthn = assign31680_e32820;

        let assign31690_e32823: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard735 = assign31690_e32823;

        let (assign31700_e32834, assign31700_e32834_d_n0, assign31700_e32834_d_n2, assign31700_e32834_d_n4, assign31700_e32834_d_n5, assign31700_e32834_d_n6, assign31700_e32834_d_n7, assign31700_e32834_d_n8, assign31700_e32834_d_n9, assign31700_e32834_d_n10, assign31700_e32834_d_n11, assign31700_e32834_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 != 0.0)) {
        (locals.var_phi_jl_dep_acc, locals.var_phi_jl_dep_acc_dn0, locals.var_phi_jl_dep_acc_dn2, locals.var_phi_jl_dep_acc_dn4, locals.var_phi_jl_dep_acc_dn5, locals.var_phi_jl_dep_acc_dn6, locals.var_phi_jl_dep_acc_dn7, locals.var_phi_jl_dep_acc_dn8, locals.var_phi_jl_dep_acc_dn9, locals.var_phi_jl_dep_acc_dn10, locals.var_phi_jl_dep_acc_dn11, locals.var_phi_jl_dep_acc_dn14,)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31700_e32834;
        locals.var_phi_jl_dep_dn0 = assign31700_e32834_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31700_e32834_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31700_e32834_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31700_e32834_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31700_e32834_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31700_e32834_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31700_e32834_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31700_e32834_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31700_e32834_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31700_e32834_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31700_e32834_d_n14;

        let (assign31710_e32845, assign31710_e32845_d_n0, assign31710_e32845_d_n2, assign31710_e32845_d_n4, assign31710_e32845_d_n5, assign31710_e32845_d_n6, assign31710_e32845_d_n7, assign31710_e32845_d_n8, assign31710_e32845_d_n9, assign31710_e32845_d_n10, assign31710_e32845_d_n11, assign31710_e32845_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31710_e32845;
        locals.var_phi_bl_dep_dn0 = assign31710_e32845_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31710_e32845_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31710_e32845_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31710_e32845_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31710_e32845_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31710_e32845_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31710_e32845_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31710_e32845_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31710_e32845_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31710_e32845_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31710_e32845_d_n14;

        let (assign31720_e32869, assign31720_e32869_d_n0, assign31720_e32869_d_n2, assign31720_e32869_d_n4, assign31720_e32869_d_n5, assign31720_e32869_d_n6, assign31720_e32869_d_n7, assign31720_e32869_d_n8, assign31720_e32869_d_n9, assign31720_e32869_d_n10, assign31720_e32869_d_n11, assign31720_e32869_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 != 0.0)) {
        let assign31720_e32856: f64 = (locals.var_afact * locals.var_vgp);
        let assign31720_e32858: f64 = (assign31720_e32856 * locals.var_vgp);
        let assign31720_e32859: f64 = (assign31720_e32858).ln();
        let assign31720_e32863: f64 = (2.0 / locals.var_vgp);
        let assign31720_e32864: f64 = (locals.var_beta + assign31720_e32863);
        let assign31720_e32865: f64 = (assign31720_e32859 / assign31720_e32864);
        let assign31720_e32867: f64 = (assign31720_e32865 + locals.var_vds);
        (assign31720_e32867, (((((((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn0)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn0), (((((((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn2)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn2), (((((((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn4)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp_dn4) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn4), (((((((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn5)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp_dn5) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn5), (((((((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn6)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn6), (((((((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn7)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn7), (((((((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn8)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp_dn8) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn8), (((((((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn9)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp_dn9) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn9), (((((((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn10)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn10), (((((((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn11)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn11 + (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn11), (((((((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign31720_e32856 * locals.var_vgp_dn14)) / assign31720_e32858) * assign31720_e32864) - (assign31720_e32859 * (locals.var_beta_dn14 + (-((2.0 * locals.var_vgp_dn14) / (locals.var_vgp * locals.var_vgp)))))) / (assign31720_e32864 * assign31720_e32864)) + locals.var_vds_dn14),)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31720_e32869;
        locals.var_phi_sl_dep_ini_dn0 = assign31720_e32869_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31720_e32869_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31720_e32869_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31720_e32869_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31720_e32869_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31720_e32869_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31720_e32869_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31720_e32869_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31720_e32869_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31720_e32869_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31720_e32869_d_n14;

        let assign31730_e32873: f64 = (locals.var_vds_maxbl + locals.var_ps_conv23);
        let assign31730_e32874: f64 = if locals.var_phi_sl_dep_ini < assign31730_e32873 { 1.0 } else { 0.0 };
        locals.var_guard736 = assign31730_e32874;

    }

    pub(super) fn stamp_transient_block_94(
        locals: &mut StampLocals,
    ) {
        let (assign31740_e32889, assign31740_e32889_d_n0, assign31740_e32889_d_n2, assign31740_e32889_d_n4, assign31740_e32889_d_n5, assign31740_e32889_d_n6, assign31740_e32889_d_n7, assign31740_e32889_d_n8, assign31740_e32889_d_n9, assign31740_e32889_d_n10, assign31740_e32889_d_n11, assign31740_e32889_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 != 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign31740_e32887: f64 = (locals.var_vds_maxbl + locals.var_ps_conv23);
        (assign31740_e32887, locals.var_vds_maxbl_dn0, locals.var_vds_maxbl_dn2, locals.var_vds_maxbl_dn4, locals.var_vds_maxbl_dn5, locals.var_vds_maxbl_dn6, locals.var_vds_maxbl_dn7, locals.var_vds_maxbl_dn8, locals.var_vds_maxbl_dn9, locals.var_vds_maxbl_dn10, locals.var_vds_maxbl_dn11, locals.var_vds_maxbl_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31740_e32889;
        locals.var_phi_sl_dep_ini_dn0 = assign31740_e32889_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31740_e32889_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31740_e32889_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31740_e32889_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31740_e32889_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31740_e32889_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31740_e32889_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31740_e32889_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31740_e32889_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31740_e32889_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31740_e32889_d_n14;

        let assign31750_e32892: f64 = if locals.var_vgp > locals.var_vgp1 { 1.0 } else { 0.0 };
        locals.var_guard737 = assign31750_e32892;

        let (assign31760_e32906, assign31760_e32906_d_n0, assign31760_e32906_d_n2, assign31760_e32906_d_n4, assign31760_e32906_d_n5, assign31760_e32906_d_n6, assign31760_e32906_d_n7, assign31760_e32906_d_n8, assign31760_e32906_d_n9, assign31760_e32906_d_n10, assign31760_e32906_d_n11, assign31760_e32906_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 != 0.0)) {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31760_e32906;
        locals.var_phi_sl_dep_ini_dn0 = assign31760_e32906_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31760_e32906_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31760_e32906_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31760_e32906_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31760_e32906_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31760_e32906_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31760_e32906_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31760_e32906_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31760_e32906_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31760_e32906_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31760_e32906_d_n14;

        let assign31770_e32909: f64 = if locals.var_vgp > locals.var_vthn { 1.0 } else { 0.0 };
        locals.var_guard738 = assign31770_e32909;

        let (assign31780_e32933, assign31780_e32933_d_n0, assign31780_e32933_d_n2, assign31780_e32933_d_n4, assign31780_e32933_d_n5, assign31780_e32933_d_n6, assign31780_e32933_d_n7, assign31780_e32933_d_n8, assign31780_e32933_d_n9, assign31780_e32933_d_n10, assign31780_e32933_d_n11, assign31780_e32933_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31780_e32925: f64 = (-2.0);
        let assign31780_e32927: f64 = (assign31780_e32925 * locals.var_afact);
        let assign31780_e32929: f64 = (assign31780_e32927 * locals.var_vgp);
        let assign31780_e32931: f64 = (assign31780_e32929 + locals.var_beta);
        (assign31780_e32931, ((((assign31780_e32925 * locals.var_afact_dn0) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn0)) + locals.var_beta_dn0), ((((assign31780_e32925 * locals.var_afact_dn2) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn2)) + locals.var_beta_dn2), ((((assign31780_e32925 * locals.var_afact_dn4) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn4)) + locals.var_beta_dn4), ((((assign31780_e32925 * locals.var_afact_dn5) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn5)) + locals.var_beta_dn5), ((((assign31780_e32925 * locals.var_afact_dn6) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn6)) + locals.var_beta_dn6), ((((assign31780_e32925 * locals.var_afact_dn7) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn7)) + locals.var_beta_dn7), ((((assign31780_e32925 * locals.var_afact_dn8) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn8)) + locals.var_beta_dn8), ((((assign31780_e32925 * locals.var_afact_dn9) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn9)) + locals.var_beta_dn9), ((((assign31780_e32925 * locals.var_afact_dn10) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn10)) + locals.var_beta_dn10), ((((assign31780_e32925 * locals.var_afact_dn11) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn11)) + locals.var_beta_dn11), ((((assign31780_e32925 * locals.var_afact_dn14) * locals.var_vgp) + (assign31780_e32927 * locals.var_vgp_dn14)) + locals.var_beta_dn14),)
    } else {
        (locals.var_bfact, locals.var_bfact_dn0, locals.var_bfact_dn2, locals.var_bfact_dn4, locals.var_bfact_dn5, locals.var_bfact_dn6, locals.var_bfact_dn7, locals.var_bfact_dn8, locals.var_bfact_dn9, locals.var_bfact_dn10, locals.var_bfact_dn11, locals.var_bfact_dn14,)
    }
};
        locals.var_bfact = assign31780_e32933;
        locals.var_bfact_dn0 = assign31780_e32933_d_n0;
        locals.var_bfact_dn2 = assign31780_e32933_d_n2;
        locals.var_bfact_dn4 = assign31780_e32933_d_n4;
        locals.var_bfact_dn5 = assign31780_e32933_d_n5;
        locals.var_bfact_dn6 = assign31780_e32933_d_n6;
        locals.var_bfact_dn7 = assign31780_e32933_d_n7;
        locals.var_bfact_dn8 = assign31780_e32933_d_n8;
        locals.var_bfact_dn9 = assign31780_e32933_d_n9;
        locals.var_bfact_dn10 = assign31780_e32933_d_n10;
        locals.var_bfact_dn11 = assign31780_e32933_d_n11;
        locals.var_bfact_dn14 = assign31780_e32933_d_n14;

        let (assign31790_e32958, assign31790_e32958_d_n0, assign31790_e32958_d_n2, assign31790_e32958_d_n4, assign31790_e32958_d_n5, assign31790_e32958_d_n6, assign31790_e32958_d_n7, assign31790_e32958_d_n8, assign31790_e32958_d_n9, assign31790_e32958_d_n10, assign31790_e32958_d_n11, assign31790_e32958_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31790_e32950: f64 = (locals.var_afact * locals.var_vgp);
        let assign31790_e32952: f64 = (assign31790_e32950 * locals.var_vgp);
        let assign31790_e32955: f64 = (locals.var_beta * locals.var_phi_bl_dep);
        let assign31790_e32956: f64 = (assign31790_e32952 - assign31790_e32955);
        (assign31790_e32956, (((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn0)) - ((locals.var_beta_dn0 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn0))), (((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn2)) - ((locals.var_beta_dn2 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn2))), (((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn4)) - ((locals.var_beta_dn4 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn4))), (((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn5)) - ((locals.var_beta_dn5 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn5))), (((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn6)) - ((locals.var_beta_dn6 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn6))), (((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn7)) - ((locals.var_beta_dn7 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn7))), (((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn8)) - ((locals.var_beta_dn8 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn8))), (((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn9)) - ((locals.var_beta_dn9 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn9))), (((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn10)) - ((locals.var_beta_dn10 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn10))), (((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn11)) - ((locals.var_beta_dn11 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn11))), (((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign31790_e32950 * locals.var_vgp_dn14)) - ((locals.var_beta_dn14 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn14))),)
    } else {
        (locals.var_cfact, locals.var_cfact_dn0, locals.var_cfact_dn2, locals.var_cfact_dn4, locals.var_cfact_dn5, locals.var_cfact_dn6, locals.var_cfact_dn7, locals.var_cfact_dn8, locals.var_cfact_dn9, locals.var_cfact_dn10, locals.var_cfact_dn11, locals.var_cfact_dn14,)
    }
};
        locals.var_cfact = assign31790_e32958;
        locals.var_cfact_dn0 = assign31790_e32958_d_n0;
        locals.var_cfact_dn2 = assign31790_e32958_d_n2;
        locals.var_cfact_dn4 = assign31790_e32958_d_n4;
        locals.var_cfact_dn5 = assign31790_e32958_d_n5;
        locals.var_cfact_dn6 = assign31790_e32958_d_n6;
        locals.var_cfact_dn7 = assign31790_e32958_d_n7;
        locals.var_cfact_dn8 = assign31790_e32958_d_n8;
        locals.var_cfact_dn9 = assign31790_e32958_d_n9;
        locals.var_cfact_dn10 = assign31790_e32958_d_n10;
        locals.var_cfact_dn11 = assign31790_e32958_d_n11;
        locals.var_cfact_dn14 = assign31790_e32958_d_n14;

        let (assign31800_e32975,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        (locals.var_phi_bl_dep,)
    } else {
        (locals.var_phi_bl_dep_old,)
    }
};
        locals.var_phi_bl_dep_old = assign31800_e32975;

        let (assign31810_e33008, assign31810_e33008_d_n0, assign31810_e33008_d_n2, assign31810_e33008_d_n4, assign31810_e33008_d_n5, assign31810_e33008_d_n6, assign31810_e33008_d_n7, assign31810_e33008_d_n8, assign31810_e33008_d_n9, assign31810_e33008_d_n10, assign31810_e33008_d_n11, assign31810_e33008_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31810_e32991: f64 = (-locals.var_bfact);
        let assign31810_e32994: f64 = (locals.var_bfact * locals.var_bfact);
        let assign31810_e32997: f64 = (4.0 * locals.var_afact);
        let assign31810_e32999: f64 = (assign31810_e32997 * locals.var_cfact);
        let assign31810_e33000: f64 = (assign31810_e32994 - assign31810_e32999);
        let assign31810_e33001: f64 = (assign31810_e33000).sqrt();
        let assign31810_e33002: f64 = (assign31810_e32991 + assign31810_e33001);
        let assign31810_e33004: f64 = (assign31810_e33002 / 2.0);
        let assign31810_e33006: f64 = (assign31810_e33004 / locals.var_afact);
        (assign31810_e33006, ((((((-locals.var_bfact_dn0) + ((((locals.var_bfact_dn0 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn0)) - (((4.0 * locals.var_afact_dn0) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn0))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn2) + ((((locals.var_bfact_dn2 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn2)) - (((4.0 * locals.var_afact_dn2) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn2))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn4) + ((((locals.var_bfact_dn4 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn4)) - (((4.0 * locals.var_afact_dn4) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn4))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn5) + ((((locals.var_bfact_dn5 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn5)) - (((4.0 * locals.var_afact_dn5) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn5))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn6) + ((((locals.var_bfact_dn6 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn6)) - (((4.0 * locals.var_afact_dn6) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn6))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn7) + ((((locals.var_bfact_dn7 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn7)) - (((4.0 * locals.var_afact_dn7) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn7))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn8) + ((((locals.var_bfact_dn8 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn8)) - (((4.0 * locals.var_afact_dn8) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn8))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn9) + ((((locals.var_bfact_dn9 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn9)) - (((4.0 * locals.var_afact_dn9) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn9))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn10) + ((((locals.var_bfact_dn10 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn10)) - (((4.0 * locals.var_afact_dn10) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn10))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn11) + ((((locals.var_bfact_dn11 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn11)) - (((4.0 * locals.var_afact_dn11) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn11))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn11)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn14) + ((((locals.var_bfact_dn14 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn14)) - (((4.0 * locals.var_afact_dn14) * locals.var_cfact) + (assign31810_e32997 * locals.var_cfact_dn14))) / (2.0 * assign31810_e33001))) / 2.0) * locals.var_afact) - (assign31810_e33004 * locals.var_afact_dn14)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31810_e33008;
        locals.var_phi_sl_dep_ini_dn0 = assign31810_e33008_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31810_e33008_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31810_e33008_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31810_e33008_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31810_e33008_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31810_e33008_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31810_e33008_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31810_e33008_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31810_e33008_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31810_e33008_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31810_e33008_d_n14;

        let assign31820_e33012: f64 = (locals.var_psbmax - locals.var_ps_conv23);
        let assign31820_e33013: f64 = if locals.var_phi_sl_dep_ini > assign31820_e33012 { 1.0 } else { 0.0 };
        locals.var_guard739 = assign31820_e33013;

        let (assign31830_e33034, assign31830_e33034_d_n0, assign31830_e33034_d_n2, assign31830_e33034_d_n4, assign31830_e33034_d_n5, assign31830_e33034_d_n6, assign31830_e33034_d_n7, assign31830_e33034_d_n8, assign31830_e33034_d_n9, assign31830_e33034_d_n10, assign31830_e33034_d_n11, assign31830_e33034_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard739 != 0.0)) {
        let assign31830_e33032: f64 = (locals.var_psbmax - locals.var_ps_conv23);
        (assign31830_e33032, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31830_e33034;
        locals.var_phi_sl_dep_ini_dn0 = assign31830_e33034_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31830_e33034_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31830_e33034_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31830_e33034_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31830_e33034_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31830_e33034_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31830_e33034_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31830_e33034_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31830_e33034_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31830_e33034_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31830_e33034_d_n14;

        let (assign31840_e33056, assign31840_e33056_d_n0, assign31840_e33056_d_n2, assign31840_e33056_d_n4, assign31840_e33056_d_n5, assign31840_e33056_d_n6, assign31840_e33056_d_n7, assign31840_e33056_d_n8, assign31840_e33056_d_n9, assign31840_e33056_d_n10, assign31840_e33056_d_n11, assign31840_e33056_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31840_e33052: f64 = (locals.var_phi_bl_dep - locals.var_phi_sl_dep_ini);
        let assign31840_e33053: f64 = (locals.var_c_2esipq_ndepm * assign31840_e33052);
        let assign31840_e33054: f64 = (assign31840_e33053).sqrt();
        (assign31840_e33054, (((locals.var_c_2esipq_ndepm_dn0 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_sl_dep_ini_dn0))) / (2.0 * assign31840_e33054)), (((locals.var_c_2esipq_ndepm_dn2 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_sl_dep_ini_dn2))) / (2.0 * assign31840_e33054)), (((locals.var_c_2esipq_ndepm_dn4 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_sl_dep_ini_dn4))) / (2.0 * assign31840_e33054)), (((locals.var_c_2esipq_ndepm_dn5 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_sl_dep_ini_dn5))) / (2.0 * assign31840_e33054)), (((locals.var_c_2esipq_ndepm_dn6 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_sl_dep_ini_dn6))) / (2.0 * assign31840_e33054)), (((locals.var_c_2esipq_ndepm_dn7 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_sl_dep_ini_dn7))) / (2.0 * assign31840_e33054)), (((locals.var_c_2esipq_ndepm_dn8 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_sl_dep_ini_dn8))) / (2.0 * assign31840_e33054)), (((locals.var_c_2esipq_ndepm_dn9 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_sl_dep_ini_dn9))) / (2.0 * assign31840_e33054)), (((locals.var_c_2esipq_ndepm_dn10 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_sl_dep_ini_dn10))) / (2.0 * assign31840_e33054)), (((locals.var_c_2esipq_ndepm_dn11 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_sl_dep_ini_dn11))) / (2.0 * assign31840_e33054)), (((locals.var_c_2esipq_ndepm_dn14 * assign31840_e33052) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_sl_dep_ini_dn14))) / (2.0 * assign31840_e33054)),)
    } else {
        (locals.var_w_sl, locals.var_w_sl_dn0, locals.var_w_sl_dn2, locals.var_w_sl_dn4, locals.var_w_sl_dn5, locals.var_w_sl_dn6, locals.var_w_sl_dn7, locals.var_w_sl_dn8, locals.var_w_sl_dn9, locals.var_w_sl_dn10, locals.var_w_sl_dn11, locals.var_w_sl_dn14,)
    }
};
        locals.var_w_sl = assign31840_e33056;
        locals.var_w_sl_dn0 = assign31840_e33056_d_n0;
        locals.var_w_sl_dn2 = assign31840_e33056_d_n2;
        locals.var_w_sl_dn4 = assign31840_e33056_d_n4;
        locals.var_w_sl_dn5 = assign31840_e33056_d_n5;
        locals.var_w_sl_dn6 = assign31840_e33056_d_n6;
        locals.var_w_sl_dn7 = assign31840_e33056_d_n7;
        locals.var_w_sl_dn8 = assign31840_e33056_d_n8;
        locals.var_w_sl_dn9 = assign31840_e33056_d_n9;
        locals.var_w_sl_dn10 = assign31840_e33056_d_n10;
        locals.var_w_sl_dn11 = assign31840_e33056_d_n11;
        locals.var_w_sl_dn14 = assign31840_e33056_d_n14;

        let (assign31850_e33078, assign31850_e33078_d_n0, assign31850_e33078_d_n2, assign31850_e33078_d_n4, assign31850_e33078_d_n5, assign31850_e33078_d_n6, assign31850_e33078_d_n7, assign31850_e33078_d_n8, assign31850_e33078_d_n9, assign31850_e33078_d_n10, assign31850_e33078_d_n11, assign31850_e33078_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31850_e33074: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        let assign31850_e33075: f64 = (locals.var_c_2esipq_ndepm * assign31850_e33074);
        let assign31850_e33076: f64 = (assign31850_e33075).sqrt();
        (assign31850_e33076, (((locals.var_c_2esipq_ndepm_dn0 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0))) / (2.0 * assign31850_e33076)), (((locals.var_c_2esipq_ndepm_dn2 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2))) / (2.0 * assign31850_e33076)), (((locals.var_c_2esipq_ndepm_dn4 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4))) / (2.0 * assign31850_e33076)), (((locals.var_c_2esipq_ndepm_dn5 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5))) / (2.0 * assign31850_e33076)), (((locals.var_c_2esipq_ndepm_dn6 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6))) / (2.0 * assign31850_e33076)), (((locals.var_c_2esipq_ndepm_dn7 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7))) / (2.0 * assign31850_e33076)), (((locals.var_c_2esipq_ndepm_dn8 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8))) / (2.0 * assign31850_e33076)), (((locals.var_c_2esipq_ndepm_dn9 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9))) / (2.0 * assign31850_e33076)), (((locals.var_c_2esipq_ndepm_dn10 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10))) / (2.0 * assign31850_e33076)), (((locals.var_c_2esipq_ndepm_dn11 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_jl_dep_dn11))) / (2.0 * assign31850_e33076)), (((locals.var_c_2esipq_ndepm_dn14 * assign31850_e33074) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_jl_dep_dn14))) / (2.0 * assign31850_e33076)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign31850_e33078;
        locals.var_w_bl_dn0 = assign31850_e33078_d_n0;
        locals.var_w_bl_dn2 = assign31850_e33078_d_n2;
        locals.var_w_bl_dn4 = assign31850_e33078_d_n4;
        locals.var_w_bl_dn5 = assign31850_e33078_d_n5;
        locals.var_w_bl_dn6 = assign31850_e33078_d_n6;
        locals.var_w_bl_dn7 = assign31850_e33078_d_n7;
        locals.var_w_bl_dn8 = assign31850_e33078_d_n8;
        locals.var_w_bl_dn9 = assign31850_e33078_d_n9;
        locals.var_w_bl_dn10 = assign31850_e33078_d_n10;
        locals.var_w_bl_dn11 = assign31850_e33078_d_n11;
        locals.var_w_bl_dn14 = assign31850_e33078_d_n14;

        let assign31860_e33081: f64 = (locals.var_w_sl + locals.var_w_bl);
        let assign31860_e33083: f64 = if assign31860_e33081 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard740 = assign31860_e33083;

        let (assign31870_e33102,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign31870_e33102;

    }

    pub(super) fn stamp_transient_block_95(
        locals: &mut StampLocals,
    ) {
        let mut assign31880_loop_guard: usize = 0;
        while {
            let assign31880_cond_e33122: f64 = (150.0 + 1.0);
            let assign31880_cond_e33124: f64 = if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) && (locals.var_lp_s0 <= assign31880_cond_e33122)) { 1.0 } else { 0.0 };
            assign31880_cond_e33124 != 0.0
        } {
            assign31880_loop_guard += 1;
            assert!(assign31880_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign31880_body0_e33147, assign31880_body0_e33147_d_n0, assign31880_body0_e33147_d_n2, assign31880_body0_e33147_d_n4, assign31880_body0_e33147_d_n5, assign31880_body0_e33147_d_n6, assign31880_body0_e33147_d_n7, assign31880_body0_e33147_d_n8, assign31880_body0_e33147_d_n9, assign31880_body0_e33147_d_n10, assign31880_body0_e33147_d_n11, assign31880_body0_e33147_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign31880_body0_e33143: f64 = (locals.var_w_sl + locals.var_w_bl);
        let assign31880_body0_e33145: f64 = (assign31880_body0_e33143 - locals.var_uc_depthn);
        (assign31880_body0_e33145, ((locals.var_w_sl_dn0 + locals.var_w_bl_dn0) - locals.var_uc_depthn_dn0), ((locals.var_w_sl_dn2 + locals.var_w_bl_dn2) - locals.var_uc_depthn_dn2), ((locals.var_w_sl_dn4 + locals.var_w_bl_dn4) - locals.var_uc_depthn_dn4), ((locals.var_w_sl_dn5 + locals.var_w_bl_dn5) - locals.var_uc_depthn_dn5), ((locals.var_w_sl_dn6 + locals.var_w_bl_dn6) - locals.var_uc_depthn_dn6), ((locals.var_w_sl_dn7 + locals.var_w_bl_dn7) - locals.var_uc_depthn_dn7), ((locals.var_w_sl_dn8 + locals.var_w_bl_dn8) - locals.var_uc_depthn_dn8), ((locals.var_w_sl_dn9 + locals.var_w_bl_dn9) - locals.var_uc_depthn_dn9), ((locals.var_w_sl_dn10 + locals.var_w_bl_dn10) - locals.var_uc_depthn_dn10), ((locals.var_w_sl_dn11 + locals.var_w_bl_dn11) - locals.var_uc_depthn_dn11), ((locals.var_w_sl_dn14 + locals.var_w_bl_dn14) - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_y0, locals.var_y0_dn0, locals.var_y0_dn2, locals.var_y0_dn4, locals.var_y0_dn5, locals.var_y0_dn6, locals.var_y0_dn7, locals.var_y0_dn8, locals.var_y0_dn9, locals.var_y0_dn10, locals.var_y0_dn11, locals.var_y0_dn14,)
    }
};
            locals.var_y0 = assign31880_body0_e33147;
            locals.var_y0_dn0 = assign31880_body0_e33147_d_n0;
            locals.var_y0_dn2 = assign31880_body0_e33147_d_n2;
            locals.var_y0_dn4 = assign31880_body0_e33147_d_n4;
            locals.var_y0_dn5 = assign31880_body0_e33147_d_n5;
            locals.var_y0_dn6 = assign31880_body0_e33147_d_n6;
            locals.var_y0_dn7 = assign31880_body0_e33147_d_n7;
            locals.var_y0_dn8 = assign31880_body0_e33147_d_n8;
            locals.var_y0_dn9 = assign31880_body0_e33147_d_n9;
            locals.var_y0_dn10 = assign31880_body0_e33147_d_n10;
            locals.var_y0_dn11 = assign31880_body0_e33147_d_n11;
            locals.var_y0_dn14 = assign31880_body0_e33147_d_n14;
            let (assign31880_body1_e33184, assign31880_body1_e33184_d_n0, assign31880_body1_e33184_d_n2, assign31880_body1_e33184_d_n4, assign31880_body1_e33184_d_n5, assign31880_body1_e33184_d_n6, assign31880_body1_e33184_d_n7, assign31880_body1_e33184_d_n8, assign31880_body1_e33184_d_n9, assign31880_body1_e33184_d_n10, assign31880_body1_e33184_d_n11, assign31880_body1_e33184_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign31880_body1_e33166: f64 = (1.034943e-10 / locals.var_q_ndepm);
        let assign31880_body1_e33168: f64 = (assign31880_body1_e33166 / locals.var_w_sl);
        let assign31880_body1_e33171: f64 = (1.034943e-10 / locals.var_q_ndepm);
        let assign31880_body1_e33176: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign31880_body1_e33177: f64 = (locals.var_ndepmpnsub / assign31880_body1_e33176);
        let assign31880_body1_e33178: f64 = (1.0 - assign31880_body1_e33177);
        let assign31880_body1_e33179: f64 = (assign31880_body1_e33171 * assign31880_body1_e33178);
        let assign31880_body1_e33181: f64 = (assign31880_body1_e33179 / locals.var_w_bl);
        let assign31880_body1_e33182: f64 = (assign31880_body1_e33168 + assign31880_body1_e33181);
        (assign31880_body1_e33182, (((((-((1.034943e-10 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn0)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn0 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn0)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn0)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn2)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn2 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn2)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn2)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn4)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn4 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn4)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn4)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn5)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn5 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn5)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn5)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn6)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn6 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn6)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn6)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn7)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn7 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn7)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn7)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn8)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn8 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn8)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn8)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn9)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn9 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn9)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn9)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn10)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn10 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn10)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn10)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn11) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn11)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn11) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn11 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn11)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn11)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn14) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31880_body1_e33166 * locals.var_w_sl_dn14)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn14) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31880_body1_e33178) + (assign31880_body1_e33171 * (-(((locals.var_ndepmpnsub_dn14 * assign31880_body1_e33176) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn14)) / (assign31880_body1_e33176 * assign31880_body1_e33176))))) * locals.var_w_bl) - (assign31880_body1_e33179 * locals.var_w_bl_dn14)) / (locals.var_w_bl * locals.var_w_bl))),)
    } else {
        (locals.var_dydpsm, locals.var_dydpsm_dn0, locals.var_dydpsm_dn2, locals.var_dydpsm_dn4, locals.var_dydpsm_dn5, locals.var_dydpsm_dn6, locals.var_dydpsm_dn7, locals.var_dydpsm_dn8, locals.var_dydpsm_dn9, locals.var_dydpsm_dn10, locals.var_dydpsm_dn11, locals.var_dydpsm_dn14,)
    }
};
            locals.var_dydpsm = assign31880_body1_e33184;
            locals.var_dydpsm_dn0 = assign31880_body1_e33184_d_n0;
            locals.var_dydpsm_dn2 = assign31880_body1_e33184_d_n2;
            locals.var_dydpsm_dn4 = assign31880_body1_e33184_d_n4;
            locals.var_dydpsm_dn5 = assign31880_body1_e33184_d_n5;
            locals.var_dydpsm_dn6 = assign31880_body1_e33184_d_n6;
            locals.var_dydpsm_dn7 = assign31880_body1_e33184_d_n7;
            locals.var_dydpsm_dn8 = assign31880_body1_e33184_d_n8;
            locals.var_dydpsm_dn9 = assign31880_body1_e33184_d_n9;
            locals.var_dydpsm_dn10 = assign31880_body1_e33184_d_n10;
            locals.var_dydpsm_dn11 = assign31880_body1_e33184_d_n11;
            locals.var_dydpsm_dn14 = assign31880_body1_e33184_d_n14;
            let assign31880_body2_e33187: f64 = (locals.var_y0 / locals.var_dydpsm);
            let assign31880_body2_e33188: f64 = (assign31880_body2_e33187).abs();
            let assign31880_body2_e33190: f64 = if assign31880_body2_e33188 > 0.5 { 1.0 } else { 0.0 };
            locals.var_guard741 = assign31880_body2_e33190;
            let (assign31880_body3_e33223, assign31880_body3_e33223_d_n0, assign31880_body3_e33223_d_n2, assign31880_body3_e33223_d_n4, assign31880_body3_e33223_d_n5, assign31880_body3_e33223_d_n6, assign31880_body3_e33223_d_n7, assign31880_body3_e33223_d_n8, assign31880_body3_e33223_d_n9, assign31880_body3_e33223_d_n10, assign31880_body3_e33223_d_n11, assign31880_body3_e33223_d_n14,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 != 0.0)) {
        let assign31880_body3_e33213: f64 = (locals.var_y0 / locals.var_dydpsm);
        let (assign31880_body3_e33219,) = {
            if (assign31880_body3_e33213 >= 0.0) {
                (1.0,)
            } else {
                let assign31880_body3_e33218: f64 = (-1.0);
                (assign31880_body3_e33218,)
            }
        };
        let assign31880_body3_e33220: f64 = (0.5 * assign31880_body3_e33219);
        let assign31880_body3_e33221: f64 = (locals.var_phi_bl_dep - assign31880_body3_e33220);
        (assign31880_body3_e33221, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
            locals.var_phi_bl_dep = assign31880_body3_e33223;
            locals.var_phi_bl_dep_dn0 = assign31880_body3_e33223_d_n0;
            locals.var_phi_bl_dep_dn2 = assign31880_body3_e33223_d_n2;
            locals.var_phi_bl_dep_dn4 = assign31880_body3_e33223_d_n4;
            locals.var_phi_bl_dep_dn5 = assign31880_body3_e33223_d_n5;
            locals.var_phi_bl_dep_dn6 = assign31880_body3_e33223_d_n6;
            locals.var_phi_bl_dep_dn7 = assign31880_body3_e33223_d_n7;
            locals.var_phi_bl_dep_dn8 = assign31880_body3_e33223_d_n8;
            locals.var_phi_bl_dep_dn9 = assign31880_body3_e33223_d_n9;
            locals.var_phi_bl_dep_dn10 = assign31880_body3_e33223_d_n10;
            locals.var_phi_bl_dep_dn11 = assign31880_body3_e33223_d_n11;
            locals.var_phi_bl_dep_dn14 = assign31880_body3_e33223_d_n14;
            let (assign31880_body4_e33249, assign31880_body4_e33249_d_n0, assign31880_body4_e33249_d_n2, assign31880_body4_e33249_d_n4, assign31880_body4_e33249_d_n5, assign31880_body4_e33249_d_n6, assign31880_body4_e33249_d_n7, assign31880_body4_e33249_d_n8, assign31880_body4_e33249_d_n9, assign31880_body4_e33249_d_n10, assign31880_body4_e33249_d_n11, assign31880_body4_e33249_d_n14,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) {
        let assign31880_body4_e33246: f64 = (locals.var_y0 / locals.var_dydpsm);
        let assign31880_body4_e33247: f64 = (locals.var_phi_bl_dep - assign31880_body4_e33246);
        (assign31880_body4_e33247, (locals.var_phi_bl_dep_dn0 - (((locals.var_y0_dn0 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn0)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn2 - (((locals.var_y0_dn2 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn2)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn4 - (((locals.var_y0_dn4 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn4)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn5 - (((locals.var_y0_dn5 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn5)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn6 - (((locals.var_y0_dn6 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn6)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn7 - (((locals.var_y0_dn7 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn7)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn8 - (((locals.var_y0_dn8 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn8)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn9 - (((locals.var_y0_dn9 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn9)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn10 - (((locals.var_y0_dn10 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn10)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn11 - (((locals.var_y0_dn11 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn11)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn14 - (((locals.var_y0_dn14 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn14)) / (locals.var_dydpsm * locals.var_dydpsm))),)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
            locals.var_phi_bl_dep = assign31880_body4_e33249;
            locals.var_phi_bl_dep_dn0 = assign31880_body4_e33249_d_n0;
            locals.var_phi_bl_dep_dn2 = assign31880_body4_e33249_d_n2;
            locals.var_phi_bl_dep_dn4 = assign31880_body4_e33249_d_n4;
            locals.var_phi_bl_dep_dn5 = assign31880_body4_e33249_d_n5;
            locals.var_phi_bl_dep_dn6 = assign31880_body4_e33249_d_n6;
            locals.var_phi_bl_dep_dn7 = assign31880_body4_e33249_d_n7;
            locals.var_phi_bl_dep_dn8 = assign31880_body4_e33249_d_n8;
            locals.var_phi_bl_dep_dn9 = assign31880_body4_e33249_d_n9;
            locals.var_phi_bl_dep_dn10 = assign31880_body4_e33249_d_n10;
            locals.var_phi_bl_dep_dn11 = assign31880_body4_e33249_d_n11;
            locals.var_phi_bl_dep_dn14 = assign31880_body4_e33249_d_n14;
            let assign31880_body5_e33252: f64 = (locals.var_phi_bl_dep - locals.var_vbscl__blk439);
            let assign31880_body5_e33254: f64 = (assign31880_body5_e33252 + locals.var_vbi_dep);
            let assign31880_body5_e33257: f64 = (10.0 * 2.220446049250313e-16);
            let assign31880_body5_e33258: f64 = if assign31880_body5_e33254 < assign31880_body5_e33257 { 1.0 } else { 0.0 };
            locals.var_guard742 = assign31880_body5_e33258;
            let (assign31880_body6_e33285, assign31880_body6_e33285_d_n0, assign31880_body6_e33285_d_n2, assign31880_body6_e33285_d_n4, assign31880_body6_e33285_d_n5, assign31880_body6_e33285_d_n6, assign31880_body6_e33285_d_n7, assign31880_body6_e33285_d_n8, assign31880_body6_e33285_d_n9, assign31880_body6_e33285_d_n10, assign31880_body6_e33285_d_n11, assign31880_body6_e33285_d_n14,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign31880_body6_e33279: f64 = (locals.var_vbscl__blk439 - locals.var_vbi_dep);
        let assign31880_body6_e33282: f64 = (10.0 * 2.220446049250313e-16);
        let assign31880_body6_e33283: f64 = (assign31880_body6_e33279 + assign31880_body6_e33282);
        (assign31880_body6_e33283, (locals.var_vbscl__blk439_dn0 - locals.var_vbi_dep_dn0), (locals.var_vbscl__blk439_dn2 - locals.var_vbi_dep_dn2), (locals.var_vbscl__blk439_dn4 - locals.var_vbi_dep_dn4), (locals.var_vbscl__blk439_dn5 - locals.var_vbi_dep_dn5), (locals.var_vbscl__blk439_dn6 - locals.var_vbi_dep_dn6), (locals.var_vbscl__blk439_dn7 - locals.var_vbi_dep_dn7), (locals.var_vbscl__blk439_dn8 - locals.var_vbi_dep_dn8), (locals.var_vbscl__blk439_dn9 - locals.var_vbi_dep_dn9), (locals.var_vbscl__blk439_dn10 - locals.var_vbi_dep_dn10), (locals.var_vbscl__blk439_dn11 - locals.var_vbi_dep_dn11), (locals.var_vbscl__blk439_dn14 - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
            locals.var_phi_bl_dep = assign31880_body6_e33285;
            locals.var_phi_bl_dep_dn0 = assign31880_body6_e33285_d_n0;
            locals.var_phi_bl_dep_dn2 = assign31880_body6_e33285_d_n2;
            locals.var_phi_bl_dep_dn4 = assign31880_body6_e33285_d_n4;
            locals.var_phi_bl_dep_dn5 = assign31880_body6_e33285_d_n5;
            locals.var_phi_bl_dep_dn6 = assign31880_body6_e33285_d_n6;
            locals.var_phi_bl_dep_dn7 = assign31880_body6_e33285_d_n7;
            locals.var_phi_bl_dep_dn8 = assign31880_body6_e33285_d_n8;
            locals.var_phi_bl_dep_dn9 = assign31880_body6_e33285_d_n9;
            locals.var_phi_bl_dep_dn10 = assign31880_body6_e33285_d_n10;
            locals.var_phi_bl_dep_dn11 = assign31880_body6_e33285_d_n11;
            locals.var_phi_bl_dep_dn14 = assign31880_body6_e33285_d_n14;
            let (assign31880_body7_e33312, assign31880_body7_e33312_d_n0, assign31880_body7_e33312_d_n2, assign31880_body7_e33312_d_n4, assign31880_body7_e33312_d_n5, assign31880_body7_e33312_d_n6, assign31880_body7_e33312_d_n7, assign31880_body7_e33312_d_n8, assign31880_body7_e33312_d_n9, assign31880_body7_e33312_d_n10, assign31880_body7_e33312_d_n11, assign31880_body7_e33312_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign31880_body7_e33304: f64 = (locals.var_afact * locals.var_vgp);
        let assign31880_body7_e33306: f64 = (assign31880_body7_e33304 * locals.var_vgp);
        let assign31880_body7_e33309: f64 = (locals.var_beta * locals.var_phi_bl_dep);
        let assign31880_body7_e33310: f64 = (assign31880_body7_e33306 - assign31880_body7_e33309);
        (assign31880_body7_e33310, (((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn0)) - ((locals.var_beta_dn0 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn0))), (((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn2)) - ((locals.var_beta_dn2 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn2))), (((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn4)) - ((locals.var_beta_dn4 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn4))), (((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn5)) - ((locals.var_beta_dn5 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn5))), (((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn6)) - ((locals.var_beta_dn6 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn6))), (((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn7)) - ((locals.var_beta_dn7 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn7))), (((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn8)) - ((locals.var_beta_dn8 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn8))), (((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn9)) - ((locals.var_beta_dn9 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn9))), (((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn10)) - ((locals.var_beta_dn10 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn10))), (((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn11)) - ((locals.var_beta_dn11 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn11))), (((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign31880_body7_e33304 * locals.var_vgp_dn14)) - ((locals.var_beta_dn14 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn14))),)
    } else {
        (locals.var_cfact, locals.var_cfact_dn0, locals.var_cfact_dn2, locals.var_cfact_dn4, locals.var_cfact_dn5, locals.var_cfact_dn6, locals.var_cfact_dn7, locals.var_cfact_dn8, locals.var_cfact_dn9, locals.var_cfact_dn10, locals.var_cfact_dn11, locals.var_cfact_dn14,)
    }
};
            locals.var_cfact = assign31880_body7_e33312;
            locals.var_cfact_dn0 = assign31880_body7_e33312_d_n0;
            locals.var_cfact_dn2 = assign31880_body7_e33312_d_n2;
            locals.var_cfact_dn4 = assign31880_body7_e33312_d_n4;
            locals.var_cfact_dn5 = assign31880_body7_e33312_d_n5;
            locals.var_cfact_dn6 = assign31880_body7_e33312_d_n6;
            locals.var_cfact_dn7 = assign31880_body7_e33312_d_n7;
            locals.var_cfact_dn8 = assign31880_body7_e33312_d_n8;
            locals.var_cfact_dn9 = assign31880_body7_e33312_d_n9;
            locals.var_cfact_dn10 = assign31880_body7_e33312_d_n10;
            locals.var_cfact_dn11 = assign31880_body7_e33312_d_n11;
            locals.var_cfact_dn14 = assign31880_body7_e33312_d_n14;
            let (assign31880_body8_e33339, assign31880_body8_e33339_d_n0, assign31880_body8_e33339_d_n2, assign31880_body8_e33339_d_n4, assign31880_body8_e33339_d_n5, assign31880_body8_e33339_d_n6, assign31880_body8_e33339_d_n7, assign31880_body8_e33339_d_n8, assign31880_body8_e33339_d_n9, assign31880_body8_e33339_d_n10, assign31880_body8_e33339_d_n11, assign31880_body8_e33339_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign31880_body8_e33331: f64 = (locals.var_bfact * locals.var_bfact);
        let assign31880_body8_e33334: f64 = (4.0 * locals.var_afact);
        let assign31880_body8_e33336: f64 = (assign31880_body8_e33334 * locals.var_cfact);
        let assign31880_body8_e33337: f64 = (assign31880_body8_e33331 - assign31880_body8_e33336);
        (assign31880_body8_e33337, (((locals.var_bfact_dn0 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn0)) - (((4.0 * locals.var_afact_dn0) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn0))), (((locals.var_bfact_dn2 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn2)) - (((4.0 * locals.var_afact_dn2) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn2))), (((locals.var_bfact_dn4 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn4)) - (((4.0 * locals.var_afact_dn4) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn4))), (((locals.var_bfact_dn5 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn5)) - (((4.0 * locals.var_afact_dn5) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn5))), (((locals.var_bfact_dn6 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn6)) - (((4.0 * locals.var_afact_dn6) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn6))), (((locals.var_bfact_dn7 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn7)) - (((4.0 * locals.var_afact_dn7) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn7))), (((locals.var_bfact_dn8 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn8)) - (((4.0 * locals.var_afact_dn8) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn8))), (((locals.var_bfact_dn9 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn9)) - (((4.0 * locals.var_afact_dn9) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn9))), (((locals.var_bfact_dn10 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn10)) - (((4.0 * locals.var_afact_dn10) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn10))), (((locals.var_bfact_dn11 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn11)) - (((4.0 * locals.var_afact_dn11) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn11))), (((locals.var_bfact_dn14 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn14)) - (((4.0 * locals.var_afact_dn14) * locals.var_cfact) + (assign31880_body8_e33334 * locals.var_cfact_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign31880_body8_e33339;
            locals.var_t1_dn0 = assign31880_body8_e33339_d_n0;
            locals.var_t1_dn2 = assign31880_body8_e33339_d_n2;
            locals.var_t1_dn4 = assign31880_body8_e33339_d_n4;
            locals.var_t1_dn5 = assign31880_body8_e33339_d_n5;
            locals.var_t1_dn6 = assign31880_body8_e33339_d_n6;
            locals.var_t1_dn7 = assign31880_body8_e33339_d_n7;
            locals.var_t1_dn8 = assign31880_body8_e33339_d_n8;
            locals.var_t1_dn9 = assign31880_body8_e33339_d_n9;
            locals.var_t1_dn10 = assign31880_body8_e33339_d_n10;
            locals.var_t1_dn11 = assign31880_body8_e33339_d_n11;
            locals.var_t1_dn14 = assign31880_body8_e33339_d_n14;
            let assign31880_body9_e33342: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard743 = assign31880_body9_e33342;
            let (assign31880_body10_e33371, assign31880_body10_e33371_d_n0, assign31880_body10_e33371_d_n2, assign31880_body10_e33371_d_n4, assign31880_body10_e33371_d_n5, assign31880_body10_e33371_d_n6, assign31880_body10_e33371_d_n7, assign31880_body10_e33371_d_n8, assign31880_body10_e33371_d_n9, assign31880_body10_e33371_d_n10, assign31880_body10_e33371_d_n11, assign31880_body10_e33371_d_n14,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) && (locals.var_guard743 != 0.0)) {
        let assign31880_body10_e33362: f64 = (-locals.var_bfact);
        let assign31880_body10_e33364: f64 = (locals.var_t1).sqrt();
        let assign31880_body10_e33365: f64 = (assign31880_body10_e33362 + assign31880_body10_e33364);
        let assign31880_body10_e33367: f64 = (assign31880_body10_e33365 / 2.0);
        let assign31880_body10_e33369: f64 = (assign31880_body10_e33367 / locals.var_afact);
        (assign31880_body10_e33369, ((((((-locals.var_bfact_dn0) + (locals.var_t1_dn0 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn2) + (locals.var_t1_dn2 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn4) + (locals.var_t1_dn4 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn5) + (locals.var_t1_dn5 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn6) + (locals.var_t1_dn6 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn7) + (locals.var_t1_dn7 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn8) + (locals.var_t1_dn8 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn9) + (locals.var_t1_dn9 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn10) + (locals.var_t1_dn10 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn11) + (locals.var_t1_dn11 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn11)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn14) + (locals.var_t1_dn14 / (2.0 * assign31880_body10_e33364))) / 2.0) * locals.var_afact) - (assign31880_body10_e33367 * locals.var_afact_dn14)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
            locals.var_phi_sl_dep_ini = assign31880_body10_e33371;
            locals.var_phi_sl_dep_ini_dn0 = assign31880_body10_e33371_d_n0;
            locals.var_phi_sl_dep_ini_dn2 = assign31880_body10_e33371_d_n2;
            locals.var_phi_sl_dep_ini_dn4 = assign31880_body10_e33371_d_n4;
            locals.var_phi_sl_dep_ini_dn5 = assign31880_body10_e33371_d_n5;
            locals.var_phi_sl_dep_ini_dn6 = assign31880_body10_e33371_d_n6;
            locals.var_phi_sl_dep_ini_dn7 = assign31880_body10_e33371_d_n7;
            locals.var_phi_sl_dep_ini_dn8 = assign31880_body10_e33371_d_n8;
            locals.var_phi_sl_dep_ini_dn9 = assign31880_body10_e33371_d_n9;
            locals.var_phi_sl_dep_ini_dn10 = assign31880_body10_e33371_d_n10;
            locals.var_phi_sl_dep_ini_dn11 = assign31880_body10_e33371_d_n11;
            locals.var_phi_sl_dep_ini_dn14 = assign31880_body10_e33371_d_n14;
            let (assign31880_body11_e33398, assign31880_body11_e33398_d_n0, assign31880_body11_e33398_d_n2, assign31880_body11_e33398_d_n4, assign31880_body11_e33398_d_n5, assign31880_body11_e33398_d_n6, assign31880_body11_e33398_d_n7, assign31880_body11_e33398_d_n8, assign31880_body11_e33398_d_n9, assign31880_body11_e33398_d_n10, assign31880_body11_e33398_d_n11, assign31880_body11_e33398_d_n14,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) && (locals.var_guard743 == 0.0)) {
        let assign31880_body11_e33392: f64 = (-locals.var_bfact);
        let assign31880_body11_e33394: f64 = (assign31880_body11_e33392 / 2.0);
        let assign31880_body11_e33396: f64 = (assign31880_body11_e33394 / locals.var_afact);
        (assign31880_body11_e33396, (((((-locals.var_bfact_dn0) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn2) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn4) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn5) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn6) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn7) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn8) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn9) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn10) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn11) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn11)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn14) / 2.0) * locals.var_afact) - (assign31880_body11_e33394 * locals.var_afact_dn14)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
            locals.var_phi_sl_dep_ini = assign31880_body11_e33398;
            locals.var_phi_sl_dep_ini_dn0 = assign31880_body11_e33398_d_n0;
            locals.var_phi_sl_dep_ini_dn2 = assign31880_body11_e33398_d_n2;
            locals.var_phi_sl_dep_ini_dn4 = assign31880_body11_e33398_d_n4;
            locals.var_phi_sl_dep_ini_dn5 = assign31880_body11_e33398_d_n5;
            locals.var_phi_sl_dep_ini_dn6 = assign31880_body11_e33398_d_n6;
            locals.var_phi_sl_dep_ini_dn7 = assign31880_body11_e33398_d_n7;
            locals.var_phi_sl_dep_ini_dn8 = assign31880_body11_e33398_d_n8;
            locals.var_phi_sl_dep_ini_dn9 = assign31880_body11_e33398_d_n9;
            locals.var_phi_sl_dep_ini_dn10 = assign31880_body11_e33398_d_n10;
            locals.var_phi_sl_dep_ini_dn11 = assign31880_body11_e33398_d_n11;
            locals.var_phi_sl_dep_ini_dn14 = assign31880_body11_e33398_d_n14;
            let assign31880_body12_e33401: f64 = if locals.var_phi_sl_dep_ini > locals.var_psbmax { 1.0 } else { 0.0 };
            locals.var_guard744 = assign31880_body12_e33401;
            let (assign31880_body13_e33422, assign31880_body13_e33422_d_n0, assign31880_body13_e33422_d_n2, assign31880_body13_e33422_d_n4, assign31880_body13_e33422_d_n5, assign31880_body13_e33422_d_n6, assign31880_body13_e33422_d_n7, assign31880_body13_e33422_d_n8, assign31880_body13_e33422_d_n9, assign31880_body13_e33422_d_n10, assign31880_body13_e33422_d_n11, assign31880_body13_e33422_d_n14,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) && (locals.var_guard744 != 0.0)) {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
            locals.var_phi_sl_dep_ini = assign31880_body13_e33422;
            locals.var_phi_sl_dep_ini_dn0 = assign31880_body13_e33422_d_n0;
            locals.var_phi_sl_dep_ini_dn2 = assign31880_body13_e33422_d_n2;
            locals.var_phi_sl_dep_ini_dn4 = assign31880_body13_e33422_d_n4;
            locals.var_phi_sl_dep_ini_dn5 = assign31880_body13_e33422_d_n5;
            locals.var_phi_sl_dep_ini_dn6 = assign31880_body13_e33422_d_n6;
            locals.var_phi_sl_dep_ini_dn7 = assign31880_body13_e33422_d_n7;
            locals.var_phi_sl_dep_ini_dn8 = assign31880_body13_e33422_d_n8;
            locals.var_phi_sl_dep_ini_dn9 = assign31880_body13_e33422_d_n9;
            locals.var_phi_sl_dep_ini_dn10 = assign31880_body13_e33422_d_n10;
            locals.var_phi_sl_dep_ini_dn11 = assign31880_body13_e33422_d_n11;
            locals.var_phi_sl_dep_ini_dn14 = assign31880_body13_e33422_d_n14;
            let assign31880_body14_e33425: f64 = if locals.var_phi_sl_dep_ini > locals.var_phi_bl_dep { 1.0 } else { 0.0 };
            locals.var_guard745 = assign31880_body14_e33425;
            let (assign31880_body15_e33448, assign31880_body15_e33448_d_n0, assign31880_body15_e33448_d_n2, assign31880_body15_e33448_d_n4, assign31880_body15_e33448_d_n5, assign31880_body15_e33448_d_n6, assign31880_body15_e33448_d_n7, assign31880_body15_e33448_d_n8, assign31880_body15_e33448_d_n9, assign31880_body15_e33448_d_n10, assign31880_body15_e33448_d_n11, assign31880_body15_e33448_d_n14,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) && (locals.var_guard745 != 0.0)) {
        let assign31880_body15_e33446: f64 = (locals.var_phi_bl_dep - locals.var_ps_conv23);
        (assign31880_body15_e33446, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
            locals.var_phi_sl_dep_ini = assign31880_body15_e33448;
            locals.var_phi_sl_dep_ini_dn0 = assign31880_body15_e33448_d_n0;
            locals.var_phi_sl_dep_ini_dn2 = assign31880_body15_e33448_d_n2;
            locals.var_phi_sl_dep_ini_dn4 = assign31880_body15_e33448_d_n4;
            locals.var_phi_sl_dep_ini_dn5 = assign31880_body15_e33448_d_n5;
            locals.var_phi_sl_dep_ini_dn6 = assign31880_body15_e33448_d_n6;
            locals.var_phi_sl_dep_ini_dn7 = assign31880_body15_e33448_d_n7;
            locals.var_phi_sl_dep_ini_dn8 = assign31880_body15_e33448_d_n8;
            locals.var_phi_sl_dep_ini_dn9 = assign31880_body15_e33448_d_n9;
            locals.var_phi_sl_dep_ini_dn10 = assign31880_body15_e33448_d_n10;
            locals.var_phi_sl_dep_ini_dn11 = assign31880_body15_e33448_d_n11;
            locals.var_phi_sl_dep_ini_dn14 = assign31880_body15_e33448_d_n14;
            let (assign31880_body16_e33471,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) && (locals.var_guard745 != 0.0)) {
        let assign31880_body16_e33469: f64 = (150.0 + 1.0);
        (assign31880_body16_e33469,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31880_body16_e33471;
            let (assign31880_body17_e33495, assign31880_body17_e33495_d_n0, assign31880_body17_e33495_d_n2, assign31880_body17_e33495_d_n4, assign31880_body17_e33495_d_n5, assign31880_body17_e33495_d_n6, assign31880_body17_e33495_d_n7, assign31880_body17_e33495_d_n8, assign31880_body17_e33495_d_n9, assign31880_body17_e33495_d_n10, assign31880_body17_e33495_d_n11, assign31880_body17_e33495_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign31880_body17_e33491: f64 = (locals.var_phi_bl_dep - locals.var_phi_sl_dep_ini);
        let assign31880_body17_e33492: f64 = (locals.var_c_2esipq_ndepm * assign31880_body17_e33491);
        let assign31880_body17_e33493: f64 = (assign31880_body17_e33492).sqrt();
        (assign31880_body17_e33493, (((locals.var_c_2esipq_ndepm_dn0 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_sl_dep_ini_dn0))) / (2.0 * assign31880_body17_e33493)), (((locals.var_c_2esipq_ndepm_dn2 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_sl_dep_ini_dn2))) / (2.0 * assign31880_body17_e33493)), (((locals.var_c_2esipq_ndepm_dn4 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_sl_dep_ini_dn4))) / (2.0 * assign31880_body17_e33493)), (((locals.var_c_2esipq_ndepm_dn5 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_sl_dep_ini_dn5))) / (2.0 * assign31880_body17_e33493)), (((locals.var_c_2esipq_ndepm_dn6 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_sl_dep_ini_dn6))) / (2.0 * assign31880_body17_e33493)), (((locals.var_c_2esipq_ndepm_dn7 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_sl_dep_ini_dn7))) / (2.0 * assign31880_body17_e33493)), (((locals.var_c_2esipq_ndepm_dn8 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_sl_dep_ini_dn8))) / (2.0 * assign31880_body17_e33493)), (((locals.var_c_2esipq_ndepm_dn9 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_sl_dep_ini_dn9))) / (2.0 * assign31880_body17_e33493)), (((locals.var_c_2esipq_ndepm_dn10 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_sl_dep_ini_dn10))) / (2.0 * assign31880_body17_e33493)), (((locals.var_c_2esipq_ndepm_dn11 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_sl_dep_ini_dn11))) / (2.0 * assign31880_body17_e33493)), (((locals.var_c_2esipq_ndepm_dn14 * assign31880_body17_e33491) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_sl_dep_ini_dn14))) / (2.0 * assign31880_body17_e33493)),)
    } else {
        (locals.var_w_sl, locals.var_w_sl_dn0, locals.var_w_sl_dn2, locals.var_w_sl_dn4, locals.var_w_sl_dn5, locals.var_w_sl_dn6, locals.var_w_sl_dn7, locals.var_w_sl_dn8, locals.var_w_sl_dn9, locals.var_w_sl_dn10, locals.var_w_sl_dn11, locals.var_w_sl_dn14,)
    }
};
            locals.var_w_sl = assign31880_body17_e33495;
            locals.var_w_sl_dn0 = assign31880_body17_e33495_d_n0;
            locals.var_w_sl_dn2 = assign31880_body17_e33495_d_n2;
            locals.var_w_sl_dn4 = assign31880_body17_e33495_d_n4;
            locals.var_w_sl_dn5 = assign31880_body17_e33495_d_n5;
            locals.var_w_sl_dn6 = assign31880_body17_e33495_d_n6;
            locals.var_w_sl_dn7 = assign31880_body17_e33495_d_n7;
            locals.var_w_sl_dn8 = assign31880_body17_e33495_d_n8;
            locals.var_w_sl_dn9 = assign31880_body17_e33495_d_n9;
            locals.var_w_sl_dn10 = assign31880_body17_e33495_d_n10;
            locals.var_w_sl_dn11 = assign31880_body17_e33495_d_n11;
            locals.var_w_sl_dn14 = assign31880_body17_e33495_d_n14;
            let (assign31880_body18_e33524, assign31880_body18_e33524_d_n0, assign31880_body18_e33524_d_n2, assign31880_body18_e33524_d_n4, assign31880_body18_e33524_d_n5, assign31880_body18_e33524_d_n6, assign31880_body18_e33524_d_n7, assign31880_body18_e33524_d_n8, assign31880_body18_e33524_d_n9, assign31880_body18_e33524_d_n10, assign31880_body18_e33524_d_n11, assign31880_body18_e33524_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign31880_body18_e33514: f64 = (locals.var_ndepmpnsub * locals.var_phi_bl_dep);
        let assign31880_body18_e33516: f64 = (assign31880_body18_e33514 + locals.var_vbscl__blk439);
        let assign31880_body18_e33518: f64 = (assign31880_body18_e33516 - locals.var_vbi_dep);
        let assign31880_body18_e33521: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign31880_body18_e33522: f64 = (assign31880_body18_e33518 / assign31880_body18_e33521);
        (assign31880_body18_e33522, (((((((locals.var_ndepmpnsub_dn0 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn0)) + locals.var_vbscl__blk439_dn0) - locals.var_vbi_dep_dn0) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn0)) / (assign31880_body18_e33521 * assign31880_body18_e33521)), (((((((locals.var_ndepmpnsub_dn2 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn2)) + locals.var_vbscl__blk439_dn2) - locals.var_vbi_dep_dn2) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn2)) / (assign31880_body18_e33521 * assign31880_body18_e33521)), (((((((locals.var_ndepmpnsub_dn4 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn4)) + locals.var_vbscl__blk439_dn4) - locals.var_vbi_dep_dn4) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn4)) / (assign31880_body18_e33521 * assign31880_body18_e33521)), (((((((locals.var_ndepmpnsub_dn5 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn5)) + locals.var_vbscl__blk439_dn5) - locals.var_vbi_dep_dn5) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn5)) / (assign31880_body18_e33521 * assign31880_body18_e33521)), (((((((locals.var_ndepmpnsub_dn6 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn6)) + locals.var_vbscl__blk439_dn6) - locals.var_vbi_dep_dn6) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn6)) / (assign31880_body18_e33521 * assign31880_body18_e33521)), (((((((locals.var_ndepmpnsub_dn7 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn7)) + locals.var_vbscl__blk439_dn7) - locals.var_vbi_dep_dn7) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn7)) / (assign31880_body18_e33521 * assign31880_body18_e33521)), (((((((locals.var_ndepmpnsub_dn8 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn8)) + locals.var_vbscl__blk439_dn8) - locals.var_vbi_dep_dn8) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn8)) / (assign31880_body18_e33521 * assign31880_body18_e33521)), (((((((locals.var_ndepmpnsub_dn9 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn9)) + locals.var_vbscl__blk439_dn9) - locals.var_vbi_dep_dn9) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn9)) / (assign31880_body18_e33521 * assign31880_body18_e33521)), (((((((locals.var_ndepmpnsub_dn10 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn10)) + locals.var_vbscl__blk439_dn10) - locals.var_vbi_dep_dn10) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn10)) / (assign31880_body18_e33521 * assign31880_body18_e33521)), (((((((locals.var_ndepmpnsub_dn11 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn11)) + locals.var_vbscl__blk439_dn11) - locals.var_vbi_dep_dn11) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn11)) / (assign31880_body18_e33521 * assign31880_body18_e33521)), (((((((locals.var_ndepmpnsub_dn14 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn14)) + locals.var_vbscl__blk439_dn14) - locals.var_vbi_dep_dn14) * assign31880_body18_e33521) - (assign31880_body18_e33518 * locals.var_ndepmpnsub_dn14)) / (assign31880_body18_e33521 * assign31880_body18_e33521)),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
            locals.var_phi_jl_dep = assign31880_body18_e33524;
            locals.var_phi_jl_dep_dn0 = assign31880_body18_e33524_d_n0;
            locals.var_phi_jl_dep_dn2 = assign31880_body18_e33524_d_n2;
            locals.var_phi_jl_dep_dn4 = assign31880_body18_e33524_d_n4;
            locals.var_phi_jl_dep_dn5 = assign31880_body18_e33524_d_n5;
            locals.var_phi_jl_dep_dn6 = assign31880_body18_e33524_d_n6;
            locals.var_phi_jl_dep_dn7 = assign31880_body18_e33524_d_n7;
            locals.var_phi_jl_dep_dn8 = assign31880_body18_e33524_d_n8;
            locals.var_phi_jl_dep_dn9 = assign31880_body18_e33524_d_n9;
            locals.var_phi_jl_dep_dn10 = assign31880_body18_e33524_d_n10;
            locals.var_phi_jl_dep_dn11 = assign31880_body18_e33524_d_n11;
            locals.var_phi_jl_dep_dn14 = assign31880_body18_e33524_d_n14;
            let (assign31880_body19_e33548, assign31880_body19_e33548_d_n0, assign31880_body19_e33548_d_n2, assign31880_body19_e33548_d_n4, assign31880_body19_e33548_d_n5, assign31880_body19_e33548_d_n6, assign31880_body19_e33548_d_n7, assign31880_body19_e33548_d_n8, assign31880_body19_e33548_d_n9, assign31880_body19_e33548_d_n10, assign31880_body19_e33548_d_n11, assign31880_body19_e33548_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign31880_body19_e33544: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        let assign31880_body19_e33545: f64 = (locals.var_c_2esipq_ndepm * assign31880_body19_e33544);
        let assign31880_body19_e33546: f64 = (assign31880_body19_e33545).sqrt();
        (assign31880_body19_e33546, (((locals.var_c_2esipq_ndepm_dn0 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0))) / (2.0 * assign31880_body19_e33546)), (((locals.var_c_2esipq_ndepm_dn2 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2))) / (2.0 * assign31880_body19_e33546)), (((locals.var_c_2esipq_ndepm_dn4 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4))) / (2.0 * assign31880_body19_e33546)), (((locals.var_c_2esipq_ndepm_dn5 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5))) / (2.0 * assign31880_body19_e33546)), (((locals.var_c_2esipq_ndepm_dn6 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6))) / (2.0 * assign31880_body19_e33546)), (((locals.var_c_2esipq_ndepm_dn7 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7))) / (2.0 * assign31880_body19_e33546)), (((locals.var_c_2esipq_ndepm_dn8 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8))) / (2.0 * assign31880_body19_e33546)), (((locals.var_c_2esipq_ndepm_dn9 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9))) / (2.0 * assign31880_body19_e33546)), (((locals.var_c_2esipq_ndepm_dn10 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10))) / (2.0 * assign31880_body19_e33546)), (((locals.var_c_2esipq_ndepm_dn11 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_jl_dep_dn11))) / (2.0 * assign31880_body19_e33546)), (((locals.var_c_2esipq_ndepm_dn14 * assign31880_body19_e33544) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_jl_dep_dn14))) / (2.0 * assign31880_body19_e33546)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
            locals.var_w_bl = assign31880_body19_e33548;
            locals.var_w_bl_dn0 = assign31880_body19_e33548_d_n0;
            locals.var_w_bl_dn2 = assign31880_body19_e33548_d_n2;
            locals.var_w_bl_dn4 = assign31880_body19_e33548_d_n4;
            locals.var_w_bl_dn5 = assign31880_body19_e33548_d_n5;
            locals.var_w_bl_dn6 = assign31880_body19_e33548_d_n6;
            locals.var_w_bl_dn7 = assign31880_body19_e33548_d_n7;
            locals.var_w_bl_dn8 = assign31880_body19_e33548_d_n8;
            locals.var_w_bl_dn9 = assign31880_body19_e33548_d_n9;
            locals.var_w_bl_dn10 = assign31880_body19_e33548_d_n10;
            locals.var_w_bl_dn11 = assign31880_body19_e33548_d_n11;
            locals.var_w_bl_dn14 = assign31880_body19_e33548_d_n14;
            let assign31880_body20_e33551: f64 = (locals.var_phi_bl_dep - locals.var_phi_bl_dep_old);
            let assign31880_body20_e33552: f64 = (assign31880_body20_e33551).abs();
            let assign31880_body20_e33554: f64 = if assign31880_body20_e33552 <= 1e-8 { 1.0 } else { 0.0 };
            locals.var_guard746 = assign31880_body20_e33554;
            let (assign31880_body21_e33577,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) && (locals.var_guard746 != 0.0)) {
        let assign31880_body21_e33575: f64 = (150.0 + 1.0);
        (assign31880_body21_e33575,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31880_body21_e33577;
            let (assign31880_body22_e33596,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        (locals.var_phi_bl_dep,)
    } else {
        (locals.var_phi_bl_dep_old,)
    }
};
            locals.var_phi_bl_dep_old = assign31880_body22_e33596;
            let (assign31880_body23_e33617,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign31880_body23_e33615: f64 = (locals.var_lp_s0 + 1.0);
        (assign31880_body23_e33615,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31880_body23_e33617;
        }

        let (assign31890_e33635, assign31890_e33635_d_n0, assign31890_e33635_d_n2, assign31890_e33635_d_n4, assign31890_e33635_d_n5, assign31890_e33635_d_n6, assign31890_e33635_d_n7, assign31890_e33635_d_n8, assign31890_e33635_d_n9, assign31890_e33635_d_n10, assign31890_e33635_d_n11, assign31890_e33635_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31890_e33635;
        locals.var_phi_bl_dep_dn0 = assign31890_e33635_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31890_e33635_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31890_e33635_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31890_e33635_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31890_e33635_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31890_e33635_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31890_e33635_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31890_e33635_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31890_e33635_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31890_e33635_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31890_e33635_d_n14;

        let (assign31900_e33653, assign31900_e33653_d_n0, assign31900_e33653_d_n2, assign31900_e33653_d_n4, assign31900_e33653_d_n5, assign31900_e33653_d_n6, assign31900_e33653_d_n7, assign31900_e33653_d_n8, assign31900_e33653_d_n9, assign31900_e33653_d_n10, assign31900_e33653_d_n11, assign31900_e33653_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31900_e33653;
        locals.var_phi_jl_dep_dn0 = assign31900_e33653_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31900_e33653_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31900_e33653_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31900_e33653_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31900_e33653_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31900_e33653_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31900_e33653_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31900_e33653_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31900_e33653_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31900_e33653_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31900_e33653_d_n14;

        let (assign31910_e33671, assign31910_e33671_d_n0, assign31910_e33671_d_n2, assign31910_e33671_d_n4, assign31910_e33671_d_n5, assign31910_e33671_d_n6, assign31910_e33671_d_n7, assign31910_e33671_d_n8, assign31910_e33671_d_n9, assign31910_e33671_d_n10, assign31910_e33671_d_n11, assign31910_e33671_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31910_e33671;
        locals.var_phi_sl_dep_ini_dn0 = assign31910_e33671_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31910_e33671_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31910_e33671_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31910_e33671_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31910_e33671_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31910_e33671_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31910_e33671_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31910_e33671_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31910_e33671_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31910_e33671_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31910_e33671_d_n14;

        let (assign31920_e33680, assign31920_e33680_d_n0, assign31920_e33680_d_n2, assign31920_e33680_d_n4, assign31920_e33680_d_n5, assign31920_e33680_d_n6, assign31920_e33680_d_n7, assign31920_e33680_d_n8, assign31920_e33680_d_n9, assign31920_e33680_d_n10, assign31920_e33680_d_n11, assign31920_e33680_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_phi_b0_dep_ini, locals.var_phi_b0_dep_ini_dn0, locals.var_phi_b0_dep_ini_dn2, locals.var_phi_b0_dep_ini_dn4, locals.var_phi_b0_dep_ini_dn5, locals.var_phi_b0_dep_ini_dn6, locals.var_phi_b0_dep_ini_dn7, locals.var_phi_b0_dep_ini_dn8, locals.var_phi_b0_dep_ini_dn9, locals.var_phi_b0_dep_ini_dn10, locals.var_phi_b0_dep_ini_dn11, locals.var_phi_b0_dep_ini_dn14,)
    }
};
        locals.var_phi_b0_dep_ini = assign31920_e33680;
        locals.var_phi_b0_dep_ini_dn0 = assign31920_e33680_d_n0;
        locals.var_phi_b0_dep_ini_dn2 = assign31920_e33680_d_n2;
        locals.var_phi_b0_dep_ini_dn4 = assign31920_e33680_d_n4;
        locals.var_phi_b0_dep_ini_dn5 = assign31920_e33680_d_n5;
        locals.var_phi_b0_dep_ini_dn6 = assign31920_e33680_d_n6;
        locals.var_phi_b0_dep_ini_dn7 = assign31920_e33680_d_n7;
        locals.var_phi_b0_dep_ini_dn8 = assign31920_e33680_d_n8;
        locals.var_phi_b0_dep_ini_dn9 = assign31920_e33680_d_n9;
        locals.var_phi_b0_dep_ini_dn10 = assign31920_e33680_d_n10;
        locals.var_phi_b0_dep_ini_dn11 = assign31920_e33680_d_n11;
        locals.var_phi_b0_dep_ini_dn14 = assign31920_e33680_d_n14;

        let (assign31930_e33689,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign31930_e33689;

        let (assign31940_e33698, assign31940_e33698_d_n0, assign31940_e33698_d_n2, assign31940_e33698_d_n4, assign31940_e33698_d_n5, assign31940_e33698_d_n6, assign31940_e33698_d_n7, assign31940_e33698_d_n8, assign31940_e33698_d_n9, assign31940_e33698_d_n10, assign31940_e33698_d_n11, assign31940_e33698_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    } else {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    }
};
        locals.var_phi_sl_dep = assign31940_e33698;
        locals.var_phi_sl_dep_dn0 = assign31940_e33698_d_n0;
        locals.var_phi_sl_dep_dn2 = assign31940_e33698_d_n2;
        locals.var_phi_sl_dep_dn4 = assign31940_e33698_d_n4;
        locals.var_phi_sl_dep_dn5 = assign31940_e33698_d_n5;
        locals.var_phi_sl_dep_dn6 = assign31940_e33698_d_n6;
        locals.var_phi_sl_dep_dn7 = assign31940_e33698_d_n7;
        locals.var_phi_sl_dep_dn8 = assign31940_e33698_d_n8;
        locals.var_phi_sl_dep_dn9 = assign31940_e33698_d_n9;
        locals.var_phi_sl_dep_dn10 = assign31940_e33698_d_n10;
        locals.var_phi_sl_dep_dn11 = assign31940_e33698_d_n11;
        locals.var_phi_sl_dep_dn14 = assign31940_e33698_d_n14;

    }
}
