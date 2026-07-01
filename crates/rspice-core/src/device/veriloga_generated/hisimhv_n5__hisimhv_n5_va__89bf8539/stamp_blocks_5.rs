#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        locals: &mut StampLocals,
    ) {
        let (assign28000_e26884, assign28000_e26884_d_n0, assign28000_e26884_d_n2, assign28000_e26884_d_n4, assign28000_e26884_d_n5, assign28000_e26884_d_n6, assign28000_e26884_d_n7, assign28000_e26884_d_n8, assign28000_e26884_d_n9, assign28000_e26884_d_n10, assign28000_e26884_d_n11, assign28000_e26884_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign28000_e26878: f64 = (0.05 * locals.var_xmp);
        let assign28000_e26880: f64 = (assign28000_e26878 * locals.var_dnm);
        let assign28000_e26882: f64 = (assign28000_e26880 / locals.var_arg);
        (assign28000_e26882, ((((((0.05 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn11) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn11)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.05 * locals.var_xmp_dn14) * locals.var_dnm) + (assign28000_e26878 * locals.var_dnm_dn14)) * locals.var_arg) - (assign28000_e26880 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28000_e26884;
        locals.var_t0_dn0 = assign28000_e26884_d_n0;
        locals.var_t0_dn2 = assign28000_e26884_d_n2;
        locals.var_t0_dn4 = assign28000_e26884_d_n4;
        locals.var_t0_dn5 = assign28000_e26884_d_n5;
        locals.var_t0_dn6 = assign28000_e26884_d_n6;
        locals.var_t0_dn7 = assign28000_e26884_d_n7;
        locals.var_t0_dn8 = assign28000_e26884_d_n8;
        locals.var_t0_dn9 = assign28000_e26884_d_n9;
        locals.var_t0_dn10 = assign28000_e26884_d_n10;
        locals.var_t0_dn11 = assign28000_e26884_d_n11;
        locals.var_t0_dn14 = assign28000_e26884_d_n14;

        let (assign28010_e26896, assign28010_e26896_d_n0, assign28010_e26896_d_n2, assign28010_e26896_d_n4, assign28010_e26896_d_n5, assign28010_e26896_d_n6, assign28010_e26896_d_n7, assign28010_e26896_d_n8, assign28010_e26896_d_n9, assign28010_e26896_d_n10, assign28010_e26896_d_n11, assign28010_e26896_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign28010_e26892: f64 = 0.05;
        let assign28010_e26894: f64 = (assign28010_e26892 - locals.var_tmf0);
        (assign28010_e26894, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28010_e26896;
        locals.var_t2_dn0 = assign28010_e26896_d_n0;
        locals.var_t2_dn2 = assign28010_e26896_d_n2;
        locals.var_t2_dn4 = assign28010_e26896_d_n4;
        locals.var_t2_dn5 = assign28010_e26896_d_n5;
        locals.var_t2_dn6 = assign28010_e26896_d_n6;
        locals.var_t2_dn7 = assign28010_e26896_d_n7;
        locals.var_t2_dn8 = assign28010_e26896_d_n8;
        locals.var_t2_dn9 = assign28010_e26896_d_n9;
        locals.var_t2_dn10 = assign28010_e26896_d_n10;
        locals.var_t2_dn11 = assign28010_e26896_d_n11;
        locals.var_t2_dn14 = assign28010_e26896_d_n14;

        let (assign28020_e26904, assign28020_e26904_d_n0, assign28020_e26904_d_n2, assign28020_e26904_d_n4, assign28020_e26904_d_n5, assign28020_e26904_d_n6, assign28020_e26904_d_n7, assign28020_e26904_d_n8, assign28020_e26904_d_n9, assign28020_e26904_d_n10, assign28020_e26904_d_n11, assign28020_e26904_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28020_e26904;
        locals.var_t0_dn0 = assign28020_e26904_d_n0;
        locals.var_t0_dn2 = assign28020_e26904_d_n2;
        locals.var_t0_dn4 = assign28020_e26904_d_n4;
        locals.var_t0_dn5 = assign28020_e26904_d_n5;
        locals.var_t0_dn6 = assign28020_e26904_d_n6;
        locals.var_t0_dn7 = assign28020_e26904_d_n7;
        locals.var_t0_dn8 = assign28020_e26904_d_n8;
        locals.var_t0_dn9 = assign28020_e26904_d_n9;
        locals.var_t0_dn10 = assign28020_e26904_d_n10;
        locals.var_t0_dn11 = assign28020_e26904_d_n11;
        locals.var_t0_dn14 = assign28020_e26904_d_n14;

        let (assign28030_e26913, assign28030_e26913_d_n0, assign28030_e26913_d_n2, assign28030_e26913_d_n4, assign28030_e26913_d_n5, assign28030_e26913_d_n6, assign28030_e26913_d_n7, assign28030_e26913_d_n8, assign28030_e26913_d_n9, assign28030_e26913_d_n10, assign28030_e26913_d_n11, assign28030_e26913_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28030_e26913;
        locals.var_t2_dn0 = assign28030_e26913_d_n0;
        locals.var_t2_dn2 = assign28030_e26913_d_n2;
        locals.var_t2_dn4 = assign28030_e26913_d_n4;
        locals.var_t2_dn5 = assign28030_e26913_d_n5;
        locals.var_t2_dn6 = assign28030_e26913_d_n6;
        locals.var_t2_dn7 = assign28030_e26913_d_n7;
        locals.var_t2_dn8 = assign28030_e26913_d_n8;
        locals.var_t2_dn9 = assign28030_e26913_d_n9;
        locals.var_t2_dn10 = assign28030_e26913_d_n10;
        locals.var_t2_dn11 = assign28030_e26913_d_n11;
        locals.var_t2_dn14 = assign28030_e26913_d_n14;

        let (assign28040_e26922, assign28040_e26922_d_n0, assign28040_e26922_d_n2, assign28040_e26922_d_n4, assign28040_e26922_d_n5, assign28040_e26922_d_n6, assign28040_e26922_d_n7, assign28040_e26922_d_n8, assign28040_e26922_d_n9, assign28040_e26922_d_n10, assign28040_e26922_d_n11, assign28040_e26922_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28040_e26922;
        locals.var_t0_dn0 = assign28040_e26922_d_n0;
        locals.var_t0_dn2 = assign28040_e26922_d_n2;
        locals.var_t0_dn4 = assign28040_e26922_d_n4;
        locals.var_t0_dn5 = assign28040_e26922_d_n5;
        locals.var_t0_dn6 = assign28040_e26922_d_n6;
        locals.var_t0_dn7 = assign28040_e26922_d_n7;
        locals.var_t0_dn8 = assign28040_e26922_d_n8;
        locals.var_t0_dn9 = assign28040_e26922_d_n9;
        locals.var_t0_dn10 = assign28040_e26922_d_n10;
        locals.var_t0_dn11 = assign28040_e26922_d_n11;
        locals.var_t0_dn14 = assign28040_e26922_d_n14;

        let (assign28050_e26931, assign28050_e26931_d_n0, assign28050_e26931_d_n2, assign28050_e26931_d_n4, assign28050_e26931_d_n5, assign28050_e26931_d_n6, assign28050_e26931_d_n7, assign28050_e26931_d_n8, assign28050_e26931_d_n9, assign28050_e26931_d_n10, assign28050_e26931_d_n11, assign28050_e26931_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign28050_e26928: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign28050_e26929: f64 = (assign28050_e26928).sqrt();
        (assign28050_e26929, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign28050_e26929)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign28050_e26929)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign28050_e26929)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign28050_e26929)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign28050_e26929)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign28050_e26929)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign28050_e26929)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign28050_e26929)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign28050_e26929)), (((locals.var_c_2esipq_ndepm_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn11)) / (2.0 * assign28050_e26929)), (((locals.var_c_2esipq_ndepm_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn14)) / (2.0 * assign28050_e26929)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn11, locals.var_w_s0_dn14,)
    }
};
        locals.var_w_s0 = assign28050_e26931;
        locals.var_w_s0_dn0 = assign28050_e26931_d_n0;
        locals.var_w_s0_dn2 = assign28050_e26931_d_n2;
        locals.var_w_s0_dn4 = assign28050_e26931_d_n4;
        locals.var_w_s0_dn5 = assign28050_e26931_d_n5;
        locals.var_w_s0_dn6 = assign28050_e26931_d_n6;
        locals.var_w_s0_dn7 = assign28050_e26931_d_n7;
        locals.var_w_s0_dn8 = assign28050_e26931_d_n8;
        locals.var_w_s0_dn9 = assign28050_e26931_d_n9;
        locals.var_w_s0_dn10 = assign28050_e26931_d_n10;
        locals.var_w_s0_dn11 = assign28050_e26931_d_n11;
        locals.var_w_s0_dn14 = assign28050_e26931_d_n14;

        let (assign28060_e26941, assign28060_e26941_d_n0, assign28060_e26941_d_n2, assign28060_e26941_d_n4, assign28060_e26941_d_n5, assign28060_e26941_d_n6, assign28060_e26941_d_n7, assign28060_e26941_d_n8, assign28060_e26941_d_n9, assign28060_e26941_d_n10, assign28060_e26941_d_n11, assign28060_e26941_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign28060_e26937: f64 = (locals.var_uc_depthn - locals.var_w_b0);
        let assign28060_e26939: f64 = (assign28060_e26937 - locals.var_w_s0);
        (assign28060_e26939, ((locals.var_uc_depthn_dn0 - locals.var_w_b0_dn0) - locals.var_w_s0_dn0), ((locals.var_uc_depthn_dn2 - locals.var_w_b0_dn2) - locals.var_w_s0_dn2), ((locals.var_uc_depthn_dn4 - locals.var_w_b0_dn4) - locals.var_w_s0_dn4), ((locals.var_uc_depthn_dn5 - locals.var_w_b0_dn5) - locals.var_w_s0_dn5), ((locals.var_uc_depthn_dn6 - locals.var_w_b0_dn6) - locals.var_w_s0_dn6), ((locals.var_uc_depthn_dn7 - locals.var_w_b0_dn7) - locals.var_w_s0_dn7), ((locals.var_uc_depthn_dn8 - locals.var_w_b0_dn8) - locals.var_w_s0_dn8), ((locals.var_uc_depthn_dn9 - locals.var_w_b0_dn9) - locals.var_w_s0_dn9), ((locals.var_uc_depthn_dn10 - locals.var_w_b0_dn10) - locals.var_w_s0_dn10), ((locals.var_uc_depthn_dn11 - locals.var_w_b0_dn11) - locals.var_w_s0_dn11), ((locals.var_uc_depthn_dn14 - locals.var_w_b0_dn14) - locals.var_w_s0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28060_e26941;
        locals.var_t1_dn0 = assign28060_e26941_d_n0;
        locals.var_t1_dn2 = assign28060_e26941_d_n2;
        locals.var_t1_dn4 = assign28060_e26941_d_n4;
        locals.var_t1_dn5 = assign28060_e26941_d_n5;
        locals.var_t1_dn6 = assign28060_e26941_d_n6;
        locals.var_t1_dn7 = assign28060_e26941_d_n7;
        locals.var_t1_dn8 = assign28060_e26941_d_n8;
        locals.var_t1_dn9 = assign28060_e26941_d_n9;
        locals.var_t1_dn10 = assign28060_e26941_d_n10;
        locals.var_t1_dn11 = assign28060_e26941_d_n11;
        locals.var_t1_dn14 = assign28060_e26941_d_n14;

        let assign28070_e26945: f64 = (1e-25 + 1e-18);
        let assign28070_e26950: f64 = if ((locals.var_t1 < assign28070_e26945) && (1e-18 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard663 = assign28070_e26950;

        let (assign28080_e26962, assign28080_e26962_d_n0, assign28080_e26962_d_n2, assign28080_e26962_d_n4, assign28080_e26962_d_n5, assign28080_e26962_d_n6, assign28080_e26962_d_n7, assign28080_e26962_d_n8, assign28080_e26962_d_n9, assign28080_e26962_d_n10, assign28080_e26962_d_n11, assign28080_e26962_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28080_e26958: f64 = (1e-25 + 1e-18);
        let assign28080_e26960: f64 = (assign28080_e26958 - locals.var_t1);
        (assign28080_e26960, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign28080_e26962;
        locals.var_tmf1_dn0 = assign28080_e26962_d_n0;
        locals.var_tmf1_dn2 = assign28080_e26962_d_n2;
        locals.var_tmf1_dn4 = assign28080_e26962_d_n4;
        locals.var_tmf1_dn5 = assign28080_e26962_d_n5;
        locals.var_tmf1_dn6 = assign28080_e26962_d_n6;
        locals.var_tmf1_dn7 = assign28080_e26962_d_n7;
        locals.var_tmf1_dn8 = assign28080_e26962_d_n8;
        locals.var_tmf1_dn9 = assign28080_e26962_d_n9;
        locals.var_tmf1_dn10 = assign28080_e26962_d_n10;
        locals.var_tmf1_dn11 = assign28080_e26962_d_n11;
        locals.var_tmf1_dn14 = assign28080_e26962_d_n14;

        let (assign28090_e26972, assign28090_e26972_d_n0, assign28090_e26972_d_n2, assign28090_e26972_d_n4, assign28090_e26972_d_n5, assign28090_e26972_d_n6, assign28090_e26972_d_n7, assign28090_e26972_d_n8, assign28090_e26972_d_n9, assign28090_e26972_d_n10, assign28090_e26972_d_n11, assign28090_e26972_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28090_e26970: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28090_e26970, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign28090_e26972;
        locals.var_x2_dn0 = assign28090_e26972_d_n0;
        locals.var_x2_dn2 = assign28090_e26972_d_n2;
        locals.var_x2_dn4 = assign28090_e26972_d_n4;
        locals.var_x2_dn5 = assign28090_e26972_d_n5;
        locals.var_x2_dn6 = assign28090_e26972_d_n6;
        locals.var_x2_dn7 = assign28090_e26972_d_n7;
        locals.var_x2_dn8 = assign28090_e26972_d_n8;
        locals.var_x2_dn9 = assign28090_e26972_d_n9;
        locals.var_x2_dn10 = assign28090_e26972_d_n10;
        locals.var_x2_dn11 = assign28090_e26972_d_n11;
        locals.var_x2_dn14 = assign28090_e26972_d_n14;

        let (assign28100_e26982, assign28100_e26982_d_n0, assign28100_e26982_d_n2, assign28100_e26982_d_n4, assign28100_e26982_d_n5, assign28100_e26982_d_n6, assign28100_e26982_d_n7, assign28100_e26982_d_n8, assign28100_e26982_d_n9, assign28100_e26982_d_n10, assign28100_e26982_d_n11, assign28100_e26982_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28100_e26980: f64 = (1e-18 * 1e-18);
        (assign28100_e26980, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign28100_e26982;
        locals.var_xmax2_dn0 = assign28100_e26982_d_n0;
        locals.var_xmax2_dn2 = assign28100_e26982_d_n2;
        locals.var_xmax2_dn4 = assign28100_e26982_d_n4;
        locals.var_xmax2_dn5 = assign28100_e26982_d_n5;
        locals.var_xmax2_dn6 = assign28100_e26982_d_n6;
        locals.var_xmax2_dn7 = assign28100_e26982_d_n7;
        locals.var_xmax2_dn8 = assign28100_e26982_d_n8;
        locals.var_xmax2_dn9 = assign28100_e26982_d_n9;
        locals.var_xmax2_dn10 = assign28100_e26982_d_n10;
        locals.var_xmax2_dn11 = assign28100_e26982_d_n11;
        locals.var_xmax2_dn14 = assign28100_e26982_d_n14;

        let (assign28110_e26990, assign28110_e26990_d_n0, assign28110_e26990_d_n2, assign28110_e26990_d_n4, assign28110_e26990_d_n5, assign28110_e26990_d_n6, assign28110_e26990_d_n7, assign28110_e26990_d_n8, assign28110_e26990_d_n9, assign28110_e26990_d_n10, assign28110_e26990_d_n11, assign28110_e26990_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28110_e26990;
        locals.var_xp_dn0 = assign28110_e26990_d_n0;
        locals.var_xp_dn2 = assign28110_e26990_d_n2;
        locals.var_xp_dn4 = assign28110_e26990_d_n4;
        locals.var_xp_dn5 = assign28110_e26990_d_n5;
        locals.var_xp_dn6 = assign28110_e26990_d_n6;
        locals.var_xp_dn7 = assign28110_e26990_d_n7;
        locals.var_xp_dn8 = assign28110_e26990_d_n8;
        locals.var_xp_dn9 = assign28110_e26990_d_n9;
        locals.var_xp_dn10 = assign28110_e26990_d_n10;
        locals.var_xp_dn11 = assign28110_e26990_d_n11;
        locals.var_xp_dn14 = assign28110_e26990_d_n14;

        let (assign28120_e26998, assign28120_e26998_d_n0, assign28120_e26998_d_n2, assign28120_e26998_d_n4, assign28120_e26998_d_n5, assign28120_e26998_d_n6, assign28120_e26998_d_n7, assign28120_e26998_d_n8, assign28120_e26998_d_n9, assign28120_e26998_d_n10, assign28120_e26998_d_n11, assign28120_e26998_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28120_e26998;
        locals.var_xmp_dn0 = assign28120_e26998_d_n0;
        locals.var_xmp_dn2 = assign28120_e26998_d_n2;
        locals.var_xmp_dn4 = assign28120_e26998_d_n4;
        locals.var_xmp_dn5 = assign28120_e26998_d_n5;
        locals.var_xmp_dn6 = assign28120_e26998_d_n6;
        locals.var_xmp_dn7 = assign28120_e26998_d_n7;
        locals.var_xmp_dn8 = assign28120_e26998_d_n8;
        locals.var_xmp_dn9 = assign28120_e26998_d_n9;
        locals.var_xmp_dn10 = assign28120_e26998_d_n10;
        locals.var_xmp_dn11 = assign28120_e26998_d_n11;
        locals.var_xmp_dn14 = assign28120_e26998_d_n14;

        let (assign28130_e27006,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28130_e27006;

        let (assign28140_e27014,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28140_e27014;

        let (assign28150_e27022, assign28150_e27022_d_n0, assign28150_e27022_d_n2, assign28150_e27022_d_n4, assign28150_e27022_d_n5, assign28150_e27022_d_n6, assign28150_e27022_d_n7, assign28150_e27022_d_n8, assign28150_e27022_d_n9, assign28150_e27022_d_n10, assign28150_e27022_d_n11, assign28150_e27022_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28150_e27022;
        locals.var_arg_dn0 = assign28150_e27022_d_n0;
        locals.var_arg_dn2 = assign28150_e27022_d_n2;
        locals.var_arg_dn4 = assign28150_e27022_d_n4;
        locals.var_arg_dn5 = assign28150_e27022_d_n5;
        locals.var_arg_dn6 = assign28150_e27022_d_n6;
        locals.var_arg_dn7 = assign28150_e27022_d_n7;
        locals.var_arg_dn8 = assign28150_e27022_d_n8;
        locals.var_arg_dn9 = assign28150_e27022_d_n9;
        locals.var_arg_dn10 = assign28150_e27022_d_n10;
        locals.var_arg_dn11 = assign28150_e27022_d_n11;
        locals.var_arg_dn14 = assign28150_e27022_d_n14;

        let (assign28160_e27030, assign28160_e27030_d_n0, assign28160_e27030_d_n2, assign28160_e27030_d_n4, assign28160_e27030_d_n5, assign28160_e27030_d_n6, assign28160_e27030_d_n7, assign28160_e27030_d_n8, assign28160_e27030_d_n9, assign28160_e27030_d_n10, assign28160_e27030_d_n11, assign28160_e27030_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28160_e27030;
        locals.var_dnm_dn0 = assign28160_e27030_d_n0;
        locals.var_dnm_dn2 = assign28160_e27030_d_n2;
        locals.var_dnm_dn4 = assign28160_e27030_d_n4;
        locals.var_dnm_dn5 = assign28160_e27030_d_n5;
        locals.var_dnm_dn6 = assign28160_e27030_d_n6;
        locals.var_dnm_dn7 = assign28160_e27030_d_n7;
        locals.var_dnm_dn8 = assign28160_e27030_d_n8;
        locals.var_dnm_dn9 = assign28160_e27030_d_n9;
        locals.var_dnm_dn10 = assign28160_e27030_d_n10;
        locals.var_dnm_dn11 = assign28160_e27030_d_n11;
        locals.var_dnm_dn14 = assign28160_e27030_d_n14;

        let (assign28170_e27040, assign28170_e27040_d_n0, assign28170_e27040_d_n2, assign28170_e27040_d_n4, assign28170_e27040_d_n5, assign28170_e27040_d_n6, assign28170_e27040_d_n7, assign28170_e27040_d_n8, assign28170_e27040_d_n9, assign28170_e27040_d_n10, assign28170_e27040_d_n11, assign28170_e27040_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28170_e27038: f64 = (locals.var_xp * locals.var_x2);
        (assign28170_e27038, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28170_e27040;
        locals.var_xp_dn0 = assign28170_e27040_d_n0;
        locals.var_xp_dn2 = assign28170_e27040_d_n2;
        locals.var_xp_dn4 = assign28170_e27040_d_n4;
        locals.var_xp_dn5 = assign28170_e27040_d_n5;
        locals.var_xp_dn6 = assign28170_e27040_d_n6;
        locals.var_xp_dn7 = assign28170_e27040_d_n7;
        locals.var_xp_dn8 = assign28170_e27040_d_n8;
        locals.var_xp_dn9 = assign28170_e27040_d_n9;
        locals.var_xp_dn10 = assign28170_e27040_d_n10;
        locals.var_xp_dn11 = assign28170_e27040_d_n11;
        locals.var_xp_dn14 = assign28170_e27040_d_n14;

        let (assign28180_e27050, assign28180_e27050_d_n0, assign28180_e27050_d_n2, assign28180_e27050_d_n4, assign28180_e27050_d_n5, assign28180_e27050_d_n6, assign28180_e27050_d_n7, assign28180_e27050_d_n8, assign28180_e27050_d_n9, assign28180_e27050_d_n10, assign28180_e27050_d_n11, assign28180_e27050_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28180_e27048: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28180_e27048, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28180_e27050;
        locals.var_xmp_dn0 = assign28180_e27050_d_n0;
        locals.var_xmp_dn2 = assign28180_e27050_d_n2;
        locals.var_xmp_dn4 = assign28180_e27050_d_n4;
        locals.var_xmp_dn5 = assign28180_e27050_d_n5;
        locals.var_xmp_dn6 = assign28180_e27050_d_n6;
        locals.var_xmp_dn7 = assign28180_e27050_d_n7;
        locals.var_xmp_dn8 = assign28180_e27050_d_n8;
        locals.var_xmp_dn9 = assign28180_e27050_d_n9;
        locals.var_xmp_dn10 = assign28180_e27050_d_n10;
        locals.var_xmp_dn11 = assign28180_e27050_d_n11;
        locals.var_xmp_dn14 = assign28180_e27050_d_n14;

        let (assign28190_e27060, assign28190_e27060_d_n0, assign28190_e27060_d_n2, assign28190_e27060_d_n4, assign28190_e27060_d_n5, assign28190_e27060_d_n6, assign28190_e27060_d_n7, assign28190_e27060_d_n8, assign28190_e27060_d_n9, assign28190_e27060_d_n10, assign28190_e27060_d_n11, assign28190_e27060_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28190_e27058: f64 = (locals.var_xp * locals.var_x2);
        (assign28190_e27058, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28190_e27060;
        locals.var_xp_dn0 = assign28190_e27060_d_n0;
        locals.var_xp_dn2 = assign28190_e27060_d_n2;
        locals.var_xp_dn4 = assign28190_e27060_d_n4;
        locals.var_xp_dn5 = assign28190_e27060_d_n5;
        locals.var_xp_dn6 = assign28190_e27060_d_n6;
        locals.var_xp_dn7 = assign28190_e27060_d_n7;
        locals.var_xp_dn8 = assign28190_e27060_d_n8;
        locals.var_xp_dn9 = assign28190_e27060_d_n9;
        locals.var_xp_dn10 = assign28190_e27060_d_n10;
        locals.var_xp_dn11 = assign28190_e27060_d_n11;
        locals.var_xp_dn14 = assign28190_e27060_d_n14;

        let (assign28200_e27070, assign28200_e27070_d_n0, assign28200_e27070_d_n2, assign28200_e27070_d_n4, assign28200_e27070_d_n5, assign28200_e27070_d_n6, assign28200_e27070_d_n7, assign28200_e27070_d_n8, assign28200_e27070_d_n9, assign28200_e27070_d_n10, assign28200_e27070_d_n11, assign28200_e27070_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28200_e27068: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28200_e27068, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28200_e27070;
        locals.var_xmp_dn0 = assign28200_e27070_d_n0;
        locals.var_xmp_dn2 = assign28200_e27070_d_n2;
        locals.var_xmp_dn4 = assign28200_e27070_d_n4;
        locals.var_xmp_dn5 = assign28200_e27070_d_n5;
        locals.var_xmp_dn6 = assign28200_e27070_d_n6;
        locals.var_xmp_dn7 = assign28200_e27070_d_n7;
        locals.var_xmp_dn8 = assign28200_e27070_d_n8;
        locals.var_xmp_dn9 = assign28200_e27070_d_n9;
        locals.var_xmp_dn10 = assign28200_e27070_d_n10;
        locals.var_xmp_dn11 = assign28200_e27070_d_n11;
        locals.var_xmp_dn14 = assign28200_e27070_d_n14;

        let (assign28210_e27080, assign28210_e27080_d_n0, assign28210_e27080_d_n2, assign28210_e27080_d_n4, assign28210_e27080_d_n5, assign28210_e27080_d_n6, assign28210_e27080_d_n7, assign28210_e27080_d_n8, assign28210_e27080_d_n9, assign28210_e27080_d_n10, assign28210_e27080_d_n11, assign28210_e27080_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28210_e27078: f64 = (locals.var_xp + locals.var_xmp);
        (assign28210_e27078, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28210_e27080;
        locals.var_arg_dn0 = assign28210_e27080_d_n0;
        locals.var_arg_dn2 = assign28210_e27080_d_n2;
        locals.var_arg_dn4 = assign28210_e27080_d_n4;
        locals.var_arg_dn5 = assign28210_e27080_d_n5;
        locals.var_arg_dn6 = assign28210_e27080_d_n6;
        locals.var_arg_dn7 = assign28210_e27080_d_n7;
        locals.var_arg_dn8 = assign28210_e27080_d_n8;
        locals.var_arg_dn9 = assign28210_e27080_d_n9;
        locals.var_arg_dn10 = assign28210_e27080_d_n10;
        locals.var_arg_dn11 = assign28210_e27080_d_n11;
        locals.var_arg_dn14 = assign28210_e27080_d_n14;

        let (assign28220_e27088, assign28220_e27088_d_n0, assign28220_e27088_d_n2, assign28220_e27088_d_n4, assign28220_e27088_d_n5, assign28220_e27088_d_n6, assign28220_e27088_d_n7, assign28220_e27088_d_n8, assign28220_e27088_d_n9, assign28220_e27088_d_n10, assign28220_e27088_d_n11, assign28220_e27088_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28220_e27088;
        locals.var_dnm_dn0 = assign28220_e27088_d_n0;
        locals.var_dnm_dn2 = assign28220_e27088_d_n2;
        locals.var_dnm_dn4 = assign28220_e27088_d_n4;
        locals.var_dnm_dn5 = assign28220_e27088_d_n5;
        locals.var_dnm_dn6 = assign28220_e27088_d_n6;
        locals.var_dnm_dn7 = assign28220_e27088_d_n7;
        locals.var_dnm_dn8 = assign28220_e27088_d_n8;
        locals.var_dnm_dn9 = assign28220_e27088_d_n9;
        locals.var_dnm_dn10 = assign28220_e27088_d_n10;
        locals.var_dnm_dn11 = assign28220_e27088_d_n11;
        locals.var_dnm_dn14 = assign28220_e27088_d_n14;

        let assign28230_e27103: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard664 = assign28230_e27103;

        let assign28240_e27106: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard665 = assign28240_e27106;

        let (assign28250_e27118,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28250_e27118;

        let assign28260_e27121: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard666 = assign28260_e27121;

        let (assign28270_e27136,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28270_e27136;

        let assign28280_e27139: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard667 = assign28280_e27139;

        let (assign28290_e27157,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 == 0.0)) && (locals.var_guard667 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28290_e27157;

        let assign28300_e27160: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard668 = assign28300_e27160;

        let (assign28310_e27181,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 == 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28310_e27181;

        let (assign28320_e27191,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28320_e27191;

    }

    pub(super) fn stamp_transient_block_81(
        locals: &mut StampLocals,
    ) {
        let mut assign28330_loop_guard: usize = 0;
        while {
            let assign28330_cond_e27202: f64 = if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign28330_cond_e27202 != 0.0
        } {
            assign28330_loop_guard += 1;
            assert!(assign28330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign28330_body0_e27213, assign28330_body0_e27213_d_n0, assign28330_body0_e27213_d_n2, assign28330_body0_e27213_d_n4, assign28330_body0_e27213_d_n5, assign28330_body0_e27213_d_n6, assign28330_body0_e27213_d_n7, assign28330_body0_e27213_d_n8, assign28330_body0_e27213_d_n9, assign28330_body0_e27213_d_n10, assign28330_body0_e27213_d_n11, assign28330_body0_e27213_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        let assign28330_body0_e27211: f64 = (locals.var_dnm).sqrt();
        (assign28330_body0_e27211, (locals.var_dnm_dn0 / (2.0 * assign28330_body0_e27211)), (locals.var_dnm_dn2 / (2.0 * assign28330_body0_e27211)), (locals.var_dnm_dn4 / (2.0 * assign28330_body0_e27211)), (locals.var_dnm_dn5 / (2.0 * assign28330_body0_e27211)), (locals.var_dnm_dn6 / (2.0 * assign28330_body0_e27211)), (locals.var_dnm_dn7 / (2.0 * assign28330_body0_e27211)), (locals.var_dnm_dn8 / (2.0 * assign28330_body0_e27211)), (locals.var_dnm_dn9 / (2.0 * assign28330_body0_e27211)), (locals.var_dnm_dn10 / (2.0 * assign28330_body0_e27211)), (locals.var_dnm_dn11 / (2.0 * assign28330_body0_e27211)), (locals.var_dnm_dn14 / (2.0 * assign28330_body0_e27211)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign28330_body0_e27213;
            locals.var_dnm_dn0 = assign28330_body0_e27213_d_n0;
            locals.var_dnm_dn2 = assign28330_body0_e27213_d_n2;
            locals.var_dnm_dn4 = assign28330_body0_e27213_d_n4;
            locals.var_dnm_dn5 = assign28330_body0_e27213_d_n5;
            locals.var_dnm_dn6 = assign28330_body0_e27213_d_n6;
            locals.var_dnm_dn7 = assign28330_body0_e27213_d_n7;
            locals.var_dnm_dn8 = assign28330_body0_e27213_d_n8;
            locals.var_dnm_dn9 = assign28330_body0_e27213_d_n9;
            locals.var_dnm_dn10 = assign28330_body0_e27213_d_n10;
            locals.var_dnm_dn11 = assign28330_body0_e27213_d_n11;
            locals.var_dnm_dn14 = assign28330_body0_e27213_d_n14;
            let (assign28330_body1_e27225,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        let assign28330_body1_e27223: f64 = (locals.var_m0 + 1.0);
        (assign28330_body1_e27223,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign28330_body1_e27225;
        }

        let (assign28340_e27247, assign28340_e27247_d_n0, assign28340_e27247_d_n2, assign28340_e27247_d_n4, assign28340_e27247_d_n5, assign28340_e27247_d_n6, assign28340_e27247_d_n7, assign28340_e27247_d_n8, assign28340_e27247_d_n9, assign28340_e27247_d_n10, assign28340_e27247_d_n11, assign28340_e27247_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 == 0.0)) {
        let (assign28340_e27245, assign28340_e27245_d_n0, assign28340_e27245_d_n2, assign28340_e27245_d_n4, assign28340_e27245_d_n5, assign28340_e27245_d_n6, assign28340_e27245_d_n7, assign28340_e27245_d_n8, assign28340_e27245_d_n9, assign28340_e27245_d_n10, assign28340_e27245_d_n11, assign28340_e27245_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign28340_e27242: f64 = (2.0 * 2.0);
                let assign28340_e27243: f64 = (1.0 / assign28340_e27242);
                let assign28340_e27244: f64 = (locals.var_dnm).powf(assign28340_e27243);
                (assign28340_e27244, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn0)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn2)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn4)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn5)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn6)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn7)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn8)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn9)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn10)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn11)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28340_e27243) as f64).is_finite() && ((assign28340_e27243) as f64).fract() == 0.0 { if assign28340_e27243 == 0.0 { 0.0 } else { (assign28340_e27243 * ((locals.var_dnm).powf(assign28340_e27243 - 1.0) * locals.var_dnm_dn14)) } } else { (assign28340_e27244 * (assign28340_e27243 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign28340_e27245, assign28340_e27245_d_n0, assign28340_e27245_d_n2, assign28340_e27245_d_n4, assign28340_e27245_d_n5, assign28340_e27245_d_n6, assign28340_e27245_d_n7, assign28340_e27245_d_n8, assign28340_e27245_d_n9, assign28340_e27245_d_n10, assign28340_e27245_d_n11, assign28340_e27245_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28340_e27247;
        locals.var_dnm_dn0 = assign28340_e27247_d_n0;
        locals.var_dnm_dn2 = assign28340_e27247_d_n2;
        locals.var_dnm_dn4 = assign28340_e27247_d_n4;
        locals.var_dnm_dn5 = assign28340_e27247_d_n5;
        locals.var_dnm_dn6 = assign28340_e27247_d_n6;
        locals.var_dnm_dn7 = assign28340_e27247_d_n7;
        locals.var_dnm_dn8 = assign28340_e27247_d_n8;
        locals.var_dnm_dn9 = assign28340_e27247_d_n9;
        locals.var_dnm_dn10 = assign28340_e27247_d_n10;
        locals.var_dnm_dn11 = assign28340_e27247_d_n11;
        locals.var_dnm_dn14 = assign28340_e27247_d_n14;

        let (assign28350_e27257, assign28350_e27257_d_n0, assign28350_e27257_d_n2, assign28350_e27257_d_n4, assign28350_e27257_d_n5, assign28350_e27257_d_n6, assign28350_e27257_d_n7, assign28350_e27257_d_n8, assign28350_e27257_d_n9, assign28350_e27257_d_n10, assign28350_e27257_d_n11, assign28350_e27257_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28350_e27255: f64 = (1.0 / locals.var_dnm);
        (assign28350_e27255, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28350_e27257;
        locals.var_dnm_dn0 = assign28350_e27257_d_n0;
        locals.var_dnm_dn2 = assign28350_e27257_d_n2;
        locals.var_dnm_dn4 = assign28350_e27257_d_n4;
        locals.var_dnm_dn5 = assign28350_e27257_d_n5;
        locals.var_dnm_dn6 = assign28350_e27257_d_n6;
        locals.var_dnm_dn7 = assign28350_e27257_d_n7;
        locals.var_dnm_dn8 = assign28350_e27257_d_n8;
        locals.var_dnm_dn9 = assign28350_e27257_d_n9;
        locals.var_dnm_dn10 = assign28350_e27257_d_n10;
        locals.var_dnm_dn11 = assign28350_e27257_d_n11;
        locals.var_dnm_dn14 = assign28350_e27257_d_n14;

        let (assign28360_e27269, assign28360_e27269_d_n0, assign28360_e27269_d_n2, assign28360_e27269_d_n4, assign28360_e27269_d_n5, assign28360_e27269_d_n6, assign28360_e27269_d_n7, assign28360_e27269_d_n8, assign28360_e27269_d_n9, assign28360_e27269_d_n10, assign28360_e27269_d_n11, assign28360_e27269_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28360_e27265: f64 = (locals.var_tmf1 * 1e-18);
        let assign28360_e27267: f64 = (assign28360_e27265 * locals.var_dnm);
        (assign28360_e27267, (((locals.var_tmf1_dn0 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-18) * locals.var_dnm) + (assign28360_e27265 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign28360_e27269;
        locals.var_tmf0_dn0 = assign28360_e27269_d_n0;
        locals.var_tmf0_dn2 = assign28360_e27269_d_n2;
        locals.var_tmf0_dn4 = assign28360_e27269_d_n4;
        locals.var_tmf0_dn5 = assign28360_e27269_d_n5;
        locals.var_tmf0_dn6 = assign28360_e27269_d_n6;
        locals.var_tmf0_dn7 = assign28360_e27269_d_n7;
        locals.var_tmf0_dn8 = assign28360_e27269_d_n8;
        locals.var_tmf0_dn9 = assign28360_e27269_d_n9;
        locals.var_tmf0_dn10 = assign28360_e27269_d_n10;
        locals.var_tmf0_dn11 = assign28360_e27269_d_n11;
        locals.var_tmf0_dn14 = assign28360_e27269_d_n14;

        let (assign28370_e27283, assign28370_e27283_d_n0, assign28370_e27283_d_n2, assign28370_e27283_d_n4, assign28370_e27283_d_n5, assign28370_e27283_d_n6, assign28370_e27283_d_n7, assign28370_e27283_d_n8, assign28370_e27283_d_n9, assign28370_e27283_d_n10, assign28370_e27283_d_n11, assign28370_e27283_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28370_e27277: f64 = (1e-18 * locals.var_xmp);
        let assign28370_e27279: f64 = (assign28370_e27277 * locals.var_dnm);
        let assign28370_e27281: f64 = (assign28370_e27279 / locals.var_arg);
        (assign28370_e27281, ((((((1e-18 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn11) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn11)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-18 * locals.var_xmp_dn14) * locals.var_dnm) + (assign28370_e27277 * locals.var_dnm_dn14)) * locals.var_arg) - (assign28370_e27279 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28370_e27283;
        locals.var_t0_dn0 = assign28370_e27283_d_n0;
        locals.var_t0_dn2 = assign28370_e27283_d_n2;
        locals.var_t0_dn4 = assign28370_e27283_d_n4;
        locals.var_t0_dn5 = assign28370_e27283_d_n5;
        locals.var_t0_dn6 = assign28370_e27283_d_n6;
        locals.var_t0_dn7 = assign28370_e27283_d_n7;
        locals.var_t0_dn8 = assign28370_e27283_d_n8;
        locals.var_t0_dn9 = assign28370_e27283_d_n9;
        locals.var_t0_dn10 = assign28370_e27283_d_n10;
        locals.var_t0_dn11 = assign28370_e27283_d_n11;
        locals.var_t0_dn14 = assign28370_e27283_d_n14;

        let (assign28380_e27295, assign28380_e27295_d_n0, assign28380_e27295_d_n2, assign28380_e27295_d_n4, assign28380_e27295_d_n5, assign28380_e27295_d_n6, assign28380_e27295_d_n7, assign28380_e27295_d_n8, assign28380_e27295_d_n9, assign28380_e27295_d_n10, assign28380_e27295_d_n11, assign28380_e27295_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign28380_e27291: f64 = (1e-25 + 1e-18);
        let assign28380_e27293: f64 = (assign28380_e27291 - locals.var_tmf0);
        (assign28380_e27293, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_res0, locals.var_w_res0_dn0, locals.var_w_res0_dn2, locals.var_w_res0_dn4, locals.var_w_res0_dn5, locals.var_w_res0_dn6, locals.var_w_res0_dn7, locals.var_w_res0_dn8, locals.var_w_res0_dn9, locals.var_w_res0_dn10, locals.var_w_res0_dn11, locals.var_w_res0_dn14,)
    }
};
        locals.var_w_res0 = assign28380_e27295;
        locals.var_w_res0_dn0 = assign28380_e27295_d_n0;
        locals.var_w_res0_dn2 = assign28380_e27295_d_n2;
        locals.var_w_res0_dn4 = assign28380_e27295_d_n4;
        locals.var_w_res0_dn5 = assign28380_e27295_d_n5;
        locals.var_w_res0_dn6 = assign28380_e27295_d_n6;
        locals.var_w_res0_dn7 = assign28380_e27295_d_n7;
        locals.var_w_res0_dn8 = assign28380_e27295_d_n8;
        locals.var_w_res0_dn9 = assign28380_e27295_d_n9;
        locals.var_w_res0_dn10 = assign28380_e27295_d_n10;
        locals.var_w_res0_dn11 = assign28380_e27295_d_n11;
        locals.var_w_res0_dn14 = assign28380_e27295_d_n14;

        let (assign28390_e27303, assign28390_e27303_d_n0, assign28390_e27303_d_n2, assign28390_e27303_d_n4, assign28390_e27303_d_n5, assign28390_e27303_d_n6, assign28390_e27303_d_n7, assign28390_e27303_d_n8, assign28390_e27303_d_n9, assign28390_e27303_d_n10, assign28390_e27303_d_n11, assign28390_e27303_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28390_e27303;
        locals.var_t0_dn0 = assign28390_e27303_d_n0;
        locals.var_t0_dn2 = assign28390_e27303_d_n2;
        locals.var_t0_dn4 = assign28390_e27303_d_n4;
        locals.var_t0_dn5 = assign28390_e27303_d_n5;
        locals.var_t0_dn6 = assign28390_e27303_d_n6;
        locals.var_t0_dn7 = assign28390_e27303_d_n7;
        locals.var_t0_dn8 = assign28390_e27303_d_n8;
        locals.var_t0_dn9 = assign28390_e27303_d_n9;
        locals.var_t0_dn10 = assign28390_e27303_d_n10;
        locals.var_t0_dn11 = assign28390_e27303_d_n11;
        locals.var_t0_dn14 = assign28390_e27303_d_n14;

        let (assign28400_e27312, assign28400_e27312_d_n0, assign28400_e27312_d_n2, assign28400_e27312_d_n4, assign28400_e27312_d_n5, assign28400_e27312_d_n6, assign28400_e27312_d_n7, assign28400_e27312_d_n8, assign28400_e27312_d_n9, assign28400_e27312_d_n10, assign28400_e27312_d_n11, assign28400_e27312_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_w_res0, locals.var_w_res0_dn0, locals.var_w_res0_dn2, locals.var_w_res0_dn4, locals.var_w_res0_dn5, locals.var_w_res0_dn6, locals.var_w_res0_dn7, locals.var_w_res0_dn8, locals.var_w_res0_dn9, locals.var_w_res0_dn10, locals.var_w_res0_dn11, locals.var_w_res0_dn14,)
    }
};
        locals.var_w_res0 = assign28400_e27312;
        locals.var_w_res0_dn0 = assign28400_e27312_d_n0;
        locals.var_w_res0_dn2 = assign28400_e27312_d_n2;
        locals.var_w_res0_dn4 = assign28400_e27312_d_n4;
        locals.var_w_res0_dn5 = assign28400_e27312_d_n5;
        locals.var_w_res0_dn6 = assign28400_e27312_d_n6;
        locals.var_w_res0_dn7 = assign28400_e27312_d_n7;
        locals.var_w_res0_dn8 = assign28400_e27312_d_n8;
        locals.var_w_res0_dn9 = assign28400_e27312_d_n9;
        locals.var_w_res0_dn10 = assign28400_e27312_d_n10;
        locals.var_w_res0_dn11 = assign28400_e27312_d_n11;
        locals.var_w_res0_dn14 = assign28400_e27312_d_n14;

        let (assign28410_e27321, assign28410_e27321_d_n0, assign28410_e27321_d_n2, assign28410_e27321_d_n4, assign28410_e27321_d_n5, assign28410_e27321_d_n6, assign28410_e27321_d_n7, assign28410_e27321_d_n8, assign28410_e27321_d_n9, assign28410_e27321_d_n10, assign28410_e27321_d_n11, assign28410_e27321_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign28410_e27321;
        locals.var_t0_dn0 = assign28410_e27321_d_n0;
        locals.var_t0_dn2 = assign28410_e27321_d_n2;
        locals.var_t0_dn4 = assign28410_e27321_d_n4;
        locals.var_t0_dn5 = assign28410_e27321_d_n5;
        locals.var_t0_dn6 = assign28410_e27321_d_n6;
        locals.var_t0_dn7 = assign28410_e27321_d_n7;
        locals.var_t0_dn8 = assign28410_e27321_d_n8;
        locals.var_t0_dn9 = assign28410_e27321_d_n9;
        locals.var_t0_dn10 = assign28410_e27321_d_n10;
        locals.var_t0_dn11 = assign28410_e27321_d_n11;
        locals.var_t0_dn14 = assign28410_e27321_d_n14;

        let (assign28420_e27330, assign28420_e27330_d_n0, assign28420_e27330_d_n2, assign28420_e27330_d_n4, assign28420_e27330_d_n5, assign28420_e27330_d_n6, assign28420_e27330_d_n7, assign28420_e27330_d_n8, assign28420_e27330_d_n9, assign28420_e27330_d_n10, assign28420_e27330_d_n11, assign28420_e27330_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign28420_e27326: f64 = (-locals.var_w_res0);
        let assign28420_e27328: f64 = (assign28420_e27326 * locals.var_q_ndepm);
        (assign28420_e27328, (((-locals.var_w_res0_dn0) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn0)), (((-locals.var_w_res0_dn2) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn2)), (((-locals.var_w_res0_dn4) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn4)), (((-locals.var_w_res0_dn5) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn5)), (((-locals.var_w_res0_dn6) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn6)), (((-locals.var_w_res0_dn7) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn7)), (((-locals.var_w_res0_dn8) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn8)), (((-locals.var_w_res0_dn9) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn9)), (((-locals.var_w_res0_dn10) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn10)), (((-locals.var_w_res0_dn11) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn11)), (((-locals.var_w_res0_dn14) * locals.var_q_ndepm) + (assign28420_e27326 * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_qn_res0, locals.var_qn_res0_dn0, locals.var_qn_res0_dn2, locals.var_qn_res0_dn4, locals.var_qn_res0_dn5, locals.var_qn_res0_dn6, locals.var_qn_res0_dn7, locals.var_qn_res0_dn8, locals.var_qn_res0_dn9, locals.var_qn_res0_dn10, locals.var_qn_res0_dn11, locals.var_qn_res0_dn14,)
    }
};
        locals.var_qn_res0 = assign28420_e27330;
        locals.var_qn_res0_dn0 = assign28420_e27330_d_n0;
        locals.var_qn_res0_dn2 = assign28420_e27330_d_n2;
        locals.var_qn_res0_dn4 = assign28420_e27330_d_n4;
        locals.var_qn_res0_dn5 = assign28420_e27330_d_n5;
        locals.var_qn_res0_dn6 = assign28420_e27330_d_n6;
        locals.var_qn_res0_dn7 = assign28420_e27330_d_n7;
        locals.var_qn_res0_dn8 = assign28420_e27330_d_n8;
        locals.var_qn_res0_dn9 = assign28420_e27330_d_n9;
        locals.var_qn_res0_dn10 = assign28420_e27330_d_n10;
        locals.var_qn_res0_dn11 = assign28420_e27330_d_n11;
        locals.var_qn_res0_dn14 = assign28420_e27330_d_n14;

        let assign28430_e27337: f64 = if ((locals.var_w_bsub0 > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard669 = assign28430_e27337;

        let assign28440_e27341: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28440_e27346: f64 = if ((locals.var_phi_s0_dep > assign28440_e27341) && (0.8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard670 = assign28440_e27346;

        let (assign28450_e27360, assign28450_e27360_d_n0, assign28450_e27360_d_n2, assign28450_e27360_d_n4, assign28450_e27360_d_n5, assign28450_e27360_d_n6, assign28450_e27360_d_n7, assign28450_e27360_d_n8, assign28450_e27360_d_n9, assign28450_e27360_d_n10, assign28450_e27360_d_n11, assign28450_e27360_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28450_e27356: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign28450_e27358: f64 = (assign28450_e27356 + 0.8);
        (assign28450_e27358, (locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phi_s0_dep_dn11 - locals.var_vds_maxb0_dn11), (locals.var_phi_s0_dep_dn14 - locals.var_vds_maxb0_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign28450_e27360;
        locals.var_tmf1_dn0 = assign28450_e27360_d_n0;
        locals.var_tmf1_dn2 = assign28450_e27360_d_n2;
        locals.var_tmf1_dn4 = assign28450_e27360_d_n4;
        locals.var_tmf1_dn5 = assign28450_e27360_d_n5;
        locals.var_tmf1_dn6 = assign28450_e27360_d_n6;
        locals.var_tmf1_dn7 = assign28450_e27360_d_n7;
        locals.var_tmf1_dn8 = assign28450_e27360_d_n8;
        locals.var_tmf1_dn9 = assign28450_e27360_d_n9;
        locals.var_tmf1_dn10 = assign28450_e27360_d_n10;
        locals.var_tmf1_dn11 = assign28450_e27360_d_n11;
        locals.var_tmf1_dn14 = assign28450_e27360_d_n14;

        let (assign28460_e27372, assign28460_e27372_d_n0, assign28460_e27372_d_n2, assign28460_e27372_d_n4, assign28460_e27372_d_n5, assign28460_e27372_d_n6, assign28460_e27372_d_n7, assign28460_e27372_d_n8, assign28460_e27372_d_n9, assign28460_e27372_d_n10, assign28460_e27372_d_n11, assign28460_e27372_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28460_e27370: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28460_e27370, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign28460_e27372;
        locals.var_x2_dn0 = assign28460_e27372_d_n0;
        locals.var_x2_dn2 = assign28460_e27372_d_n2;
        locals.var_x2_dn4 = assign28460_e27372_d_n4;
        locals.var_x2_dn5 = assign28460_e27372_d_n5;
        locals.var_x2_dn6 = assign28460_e27372_d_n6;
        locals.var_x2_dn7 = assign28460_e27372_d_n7;
        locals.var_x2_dn8 = assign28460_e27372_d_n8;
        locals.var_x2_dn9 = assign28460_e27372_d_n9;
        locals.var_x2_dn10 = assign28460_e27372_d_n10;
        locals.var_x2_dn11 = assign28460_e27372_d_n11;
        locals.var_x2_dn14 = assign28460_e27372_d_n14;

        let (assign28470_e27384, assign28470_e27384_d_n0, assign28470_e27384_d_n2, assign28470_e27384_d_n4, assign28470_e27384_d_n5, assign28470_e27384_d_n6, assign28470_e27384_d_n7, assign28470_e27384_d_n8, assign28470_e27384_d_n9, assign28470_e27384_d_n10, assign28470_e27384_d_n11, assign28470_e27384_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28470_e27382: f64 = (0.8 * 0.8);
        (assign28470_e27382, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign28470_e27384;
        locals.var_xmax2_dn0 = assign28470_e27384_d_n0;
        locals.var_xmax2_dn2 = assign28470_e27384_d_n2;
        locals.var_xmax2_dn4 = assign28470_e27384_d_n4;
        locals.var_xmax2_dn5 = assign28470_e27384_d_n5;
        locals.var_xmax2_dn6 = assign28470_e27384_d_n6;
        locals.var_xmax2_dn7 = assign28470_e27384_d_n7;
        locals.var_xmax2_dn8 = assign28470_e27384_d_n8;
        locals.var_xmax2_dn9 = assign28470_e27384_d_n9;
        locals.var_xmax2_dn10 = assign28470_e27384_d_n10;
        locals.var_xmax2_dn11 = assign28470_e27384_d_n11;
        locals.var_xmax2_dn14 = assign28470_e27384_d_n14;

        let (assign28480_e27394, assign28480_e27394_d_n0, assign28480_e27394_d_n2, assign28480_e27394_d_n4, assign28480_e27394_d_n5, assign28480_e27394_d_n6, assign28480_e27394_d_n7, assign28480_e27394_d_n8, assign28480_e27394_d_n9, assign28480_e27394_d_n10, assign28480_e27394_d_n11, assign28480_e27394_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28480_e27394;
        locals.var_xp_dn0 = assign28480_e27394_d_n0;
        locals.var_xp_dn2 = assign28480_e27394_d_n2;
        locals.var_xp_dn4 = assign28480_e27394_d_n4;
        locals.var_xp_dn5 = assign28480_e27394_d_n5;
        locals.var_xp_dn6 = assign28480_e27394_d_n6;
        locals.var_xp_dn7 = assign28480_e27394_d_n7;
        locals.var_xp_dn8 = assign28480_e27394_d_n8;
        locals.var_xp_dn9 = assign28480_e27394_d_n9;
        locals.var_xp_dn10 = assign28480_e27394_d_n10;
        locals.var_xp_dn11 = assign28480_e27394_d_n11;
        locals.var_xp_dn14 = assign28480_e27394_d_n14;

        let (assign28490_e27404, assign28490_e27404_d_n0, assign28490_e27404_d_n2, assign28490_e27404_d_n4, assign28490_e27404_d_n5, assign28490_e27404_d_n6, assign28490_e27404_d_n7, assign28490_e27404_d_n8, assign28490_e27404_d_n9, assign28490_e27404_d_n10, assign28490_e27404_d_n11, assign28490_e27404_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28490_e27404;
        locals.var_xmp_dn0 = assign28490_e27404_d_n0;
        locals.var_xmp_dn2 = assign28490_e27404_d_n2;
        locals.var_xmp_dn4 = assign28490_e27404_d_n4;
        locals.var_xmp_dn5 = assign28490_e27404_d_n5;
        locals.var_xmp_dn6 = assign28490_e27404_d_n6;
        locals.var_xmp_dn7 = assign28490_e27404_d_n7;
        locals.var_xmp_dn8 = assign28490_e27404_d_n8;
        locals.var_xmp_dn9 = assign28490_e27404_d_n9;
        locals.var_xmp_dn10 = assign28490_e27404_d_n10;
        locals.var_xmp_dn11 = assign28490_e27404_d_n11;
        locals.var_xmp_dn14 = assign28490_e27404_d_n14;

        let (assign28500_e27414,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28500_e27414;

        let (assign28510_e27424,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28510_e27424;

        let (assign28520_e27434, assign28520_e27434_d_n0, assign28520_e27434_d_n2, assign28520_e27434_d_n4, assign28520_e27434_d_n5, assign28520_e27434_d_n6, assign28520_e27434_d_n7, assign28520_e27434_d_n8, assign28520_e27434_d_n9, assign28520_e27434_d_n10, assign28520_e27434_d_n11, assign28520_e27434_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28520_e27434;
        locals.var_arg_dn0 = assign28520_e27434_d_n0;
        locals.var_arg_dn2 = assign28520_e27434_d_n2;
        locals.var_arg_dn4 = assign28520_e27434_d_n4;
        locals.var_arg_dn5 = assign28520_e27434_d_n5;
        locals.var_arg_dn6 = assign28520_e27434_d_n6;
        locals.var_arg_dn7 = assign28520_e27434_d_n7;
        locals.var_arg_dn8 = assign28520_e27434_d_n8;
        locals.var_arg_dn9 = assign28520_e27434_d_n9;
        locals.var_arg_dn10 = assign28520_e27434_d_n10;
        locals.var_arg_dn11 = assign28520_e27434_d_n11;
        locals.var_arg_dn14 = assign28520_e27434_d_n14;

        let (assign28530_e27444, assign28530_e27444_d_n0, assign28530_e27444_d_n2, assign28530_e27444_d_n4, assign28530_e27444_d_n5, assign28530_e27444_d_n6, assign28530_e27444_d_n7, assign28530_e27444_d_n8, assign28530_e27444_d_n9, assign28530_e27444_d_n10, assign28530_e27444_d_n11, assign28530_e27444_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28530_e27444;
        locals.var_dnm_dn0 = assign28530_e27444_d_n0;
        locals.var_dnm_dn2 = assign28530_e27444_d_n2;
        locals.var_dnm_dn4 = assign28530_e27444_d_n4;
        locals.var_dnm_dn5 = assign28530_e27444_d_n5;
        locals.var_dnm_dn6 = assign28530_e27444_d_n6;
        locals.var_dnm_dn7 = assign28530_e27444_d_n7;
        locals.var_dnm_dn8 = assign28530_e27444_d_n8;
        locals.var_dnm_dn9 = assign28530_e27444_d_n9;
        locals.var_dnm_dn10 = assign28530_e27444_d_n10;
        locals.var_dnm_dn11 = assign28530_e27444_d_n11;
        locals.var_dnm_dn14 = assign28530_e27444_d_n14;

        let (assign28540_e27456, assign28540_e27456_d_n0, assign28540_e27456_d_n2, assign28540_e27456_d_n4, assign28540_e27456_d_n5, assign28540_e27456_d_n6, assign28540_e27456_d_n7, assign28540_e27456_d_n8, assign28540_e27456_d_n9, assign28540_e27456_d_n10, assign28540_e27456_d_n11, assign28540_e27456_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28540_e27454: f64 = (locals.var_xp * locals.var_x2);
        (assign28540_e27454, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28540_e27456;
        locals.var_xp_dn0 = assign28540_e27456_d_n0;
        locals.var_xp_dn2 = assign28540_e27456_d_n2;
        locals.var_xp_dn4 = assign28540_e27456_d_n4;
        locals.var_xp_dn5 = assign28540_e27456_d_n5;
        locals.var_xp_dn6 = assign28540_e27456_d_n6;
        locals.var_xp_dn7 = assign28540_e27456_d_n7;
        locals.var_xp_dn8 = assign28540_e27456_d_n8;
        locals.var_xp_dn9 = assign28540_e27456_d_n9;
        locals.var_xp_dn10 = assign28540_e27456_d_n10;
        locals.var_xp_dn11 = assign28540_e27456_d_n11;
        locals.var_xp_dn14 = assign28540_e27456_d_n14;

        let (assign28550_e27468, assign28550_e27468_d_n0, assign28550_e27468_d_n2, assign28550_e27468_d_n4, assign28550_e27468_d_n5, assign28550_e27468_d_n6, assign28550_e27468_d_n7, assign28550_e27468_d_n8, assign28550_e27468_d_n9, assign28550_e27468_d_n10, assign28550_e27468_d_n11, assign28550_e27468_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28550_e27466: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28550_e27466, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28550_e27468;
        locals.var_xmp_dn0 = assign28550_e27468_d_n0;
        locals.var_xmp_dn2 = assign28550_e27468_d_n2;
        locals.var_xmp_dn4 = assign28550_e27468_d_n4;
        locals.var_xmp_dn5 = assign28550_e27468_d_n5;
        locals.var_xmp_dn6 = assign28550_e27468_d_n6;
        locals.var_xmp_dn7 = assign28550_e27468_d_n7;
        locals.var_xmp_dn8 = assign28550_e27468_d_n8;
        locals.var_xmp_dn9 = assign28550_e27468_d_n9;
        locals.var_xmp_dn10 = assign28550_e27468_d_n10;
        locals.var_xmp_dn11 = assign28550_e27468_d_n11;
        locals.var_xmp_dn14 = assign28550_e27468_d_n14;

        let (assign28560_e27480, assign28560_e27480_d_n0, assign28560_e27480_d_n2, assign28560_e27480_d_n4, assign28560_e27480_d_n5, assign28560_e27480_d_n6, assign28560_e27480_d_n7, assign28560_e27480_d_n8, assign28560_e27480_d_n9, assign28560_e27480_d_n10, assign28560_e27480_d_n11, assign28560_e27480_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28560_e27478: f64 = (locals.var_xp * locals.var_x2);
        (assign28560_e27478, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28560_e27480;
        locals.var_xp_dn0 = assign28560_e27480_d_n0;
        locals.var_xp_dn2 = assign28560_e27480_d_n2;
        locals.var_xp_dn4 = assign28560_e27480_d_n4;
        locals.var_xp_dn5 = assign28560_e27480_d_n5;
        locals.var_xp_dn6 = assign28560_e27480_d_n6;
        locals.var_xp_dn7 = assign28560_e27480_d_n7;
        locals.var_xp_dn8 = assign28560_e27480_d_n8;
        locals.var_xp_dn9 = assign28560_e27480_d_n9;
        locals.var_xp_dn10 = assign28560_e27480_d_n10;
        locals.var_xp_dn11 = assign28560_e27480_d_n11;
        locals.var_xp_dn14 = assign28560_e27480_d_n14;

        let (assign28570_e27492, assign28570_e27492_d_n0, assign28570_e27492_d_n2, assign28570_e27492_d_n4, assign28570_e27492_d_n5, assign28570_e27492_d_n6, assign28570_e27492_d_n7, assign28570_e27492_d_n8, assign28570_e27492_d_n9, assign28570_e27492_d_n10, assign28570_e27492_d_n11, assign28570_e27492_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28570_e27490: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28570_e27490, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28570_e27492;
        locals.var_xmp_dn0 = assign28570_e27492_d_n0;
        locals.var_xmp_dn2 = assign28570_e27492_d_n2;
        locals.var_xmp_dn4 = assign28570_e27492_d_n4;
        locals.var_xmp_dn5 = assign28570_e27492_d_n5;
        locals.var_xmp_dn6 = assign28570_e27492_d_n6;
        locals.var_xmp_dn7 = assign28570_e27492_d_n7;
        locals.var_xmp_dn8 = assign28570_e27492_d_n8;
        locals.var_xmp_dn9 = assign28570_e27492_d_n9;
        locals.var_xmp_dn10 = assign28570_e27492_d_n10;
        locals.var_xmp_dn11 = assign28570_e27492_d_n11;
        locals.var_xmp_dn14 = assign28570_e27492_d_n14;

        let (assign28580_e27504, assign28580_e27504_d_n0, assign28580_e27504_d_n2, assign28580_e27504_d_n4, assign28580_e27504_d_n5, assign28580_e27504_d_n6, assign28580_e27504_d_n7, assign28580_e27504_d_n8, assign28580_e27504_d_n9, assign28580_e27504_d_n10, assign28580_e27504_d_n11, assign28580_e27504_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28580_e27502: f64 = (locals.var_xp + locals.var_xmp);
        (assign28580_e27502, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28580_e27504;
        locals.var_arg_dn0 = assign28580_e27504_d_n0;
        locals.var_arg_dn2 = assign28580_e27504_d_n2;
        locals.var_arg_dn4 = assign28580_e27504_d_n4;
        locals.var_arg_dn5 = assign28580_e27504_d_n5;
        locals.var_arg_dn6 = assign28580_e27504_d_n6;
        locals.var_arg_dn7 = assign28580_e27504_d_n7;
        locals.var_arg_dn8 = assign28580_e27504_d_n8;
        locals.var_arg_dn9 = assign28580_e27504_d_n9;
        locals.var_arg_dn10 = assign28580_e27504_d_n10;
        locals.var_arg_dn11 = assign28580_e27504_d_n11;
        locals.var_arg_dn14 = assign28580_e27504_d_n14;

    }

    pub(super) fn stamp_transient_block_82(
        locals: &mut StampLocals,
    ) {
        let (assign28590_e27514, assign28590_e27514_d_n0, assign28590_e27514_d_n2, assign28590_e27514_d_n4, assign28590_e27514_d_n5, assign28590_e27514_d_n6, assign28590_e27514_d_n7, assign28590_e27514_d_n8, assign28590_e27514_d_n9, assign28590_e27514_d_n10, assign28590_e27514_d_n11, assign28590_e27514_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28590_e27514;
        locals.var_dnm_dn0 = assign28590_e27514_d_n0;
        locals.var_dnm_dn2 = assign28590_e27514_d_n2;
        locals.var_dnm_dn4 = assign28590_e27514_d_n4;
        locals.var_dnm_dn5 = assign28590_e27514_d_n5;
        locals.var_dnm_dn6 = assign28590_e27514_d_n6;
        locals.var_dnm_dn7 = assign28590_e27514_d_n7;
        locals.var_dnm_dn8 = assign28590_e27514_d_n8;
        locals.var_dnm_dn9 = assign28590_e27514_d_n9;
        locals.var_dnm_dn10 = assign28590_e27514_d_n10;
        locals.var_dnm_dn11 = assign28590_e27514_d_n11;
        locals.var_dnm_dn14 = assign28590_e27514_d_n14;

        let assign28600_e27529: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard671 = assign28600_e27529;

        let assign28610_e27532: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard672 = assign28610_e27532;

        let (assign28620_e27546,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28620_e27546;

        let assign28630_e27549: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard673 = assign28630_e27549;

        let (assign28640_e27566,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28640_e27566;

        let assign28650_e27569: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard674 = assign28650_e27569;

        let (assign28660_e27589,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) && (locals.var_guard674 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28660_e27589;

        let assign28670_e27592: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard675 = assign28670_e27592;

        let (assign28680_e27615,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) && (locals.var_guard674 == 0.0)) && (locals.var_guard675 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28680_e27615;

        let (assign28690_e27627,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28690_e27627;

        let mut assign28700_loop_guard: usize = 0;
        while {
            let assign28700_cond_e27640: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign28700_cond_e27640 != 0.0
        } {
            assign28700_loop_guard += 1;
            assert!(assign28700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign28700_body0_e27653, assign28700_body0_e27653_d_n0, assign28700_body0_e27653_d_n2, assign28700_body0_e27653_d_n4, assign28700_body0_e27653_d_n5, assign28700_body0_e27653_d_n6, assign28700_body0_e27653_d_n7, assign28700_body0_e27653_d_n8, assign28700_body0_e27653_d_n9, assign28700_body0_e27653_d_n10, assign28700_body0_e27653_d_n11, assign28700_body0_e27653_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) {
        let assign28700_body0_e27651: f64 = (locals.var_dnm).sqrt();
        (assign28700_body0_e27651, (locals.var_dnm_dn0 / (2.0 * assign28700_body0_e27651)), (locals.var_dnm_dn2 / (2.0 * assign28700_body0_e27651)), (locals.var_dnm_dn4 / (2.0 * assign28700_body0_e27651)), (locals.var_dnm_dn5 / (2.0 * assign28700_body0_e27651)), (locals.var_dnm_dn6 / (2.0 * assign28700_body0_e27651)), (locals.var_dnm_dn7 / (2.0 * assign28700_body0_e27651)), (locals.var_dnm_dn8 / (2.0 * assign28700_body0_e27651)), (locals.var_dnm_dn9 / (2.0 * assign28700_body0_e27651)), (locals.var_dnm_dn10 / (2.0 * assign28700_body0_e27651)), (locals.var_dnm_dn11 / (2.0 * assign28700_body0_e27651)), (locals.var_dnm_dn14 / (2.0 * assign28700_body0_e27651)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign28700_body0_e27653;
            locals.var_dnm_dn0 = assign28700_body0_e27653_d_n0;
            locals.var_dnm_dn2 = assign28700_body0_e27653_d_n2;
            locals.var_dnm_dn4 = assign28700_body0_e27653_d_n4;
            locals.var_dnm_dn5 = assign28700_body0_e27653_d_n5;
            locals.var_dnm_dn6 = assign28700_body0_e27653_d_n6;
            locals.var_dnm_dn7 = assign28700_body0_e27653_d_n7;
            locals.var_dnm_dn8 = assign28700_body0_e27653_d_n8;
            locals.var_dnm_dn9 = assign28700_body0_e27653_d_n9;
            locals.var_dnm_dn10 = assign28700_body0_e27653_d_n10;
            locals.var_dnm_dn11 = assign28700_body0_e27653_d_n11;
            locals.var_dnm_dn14 = assign28700_body0_e27653_d_n14;
            let (assign28700_body1_e27667,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) {
        let assign28700_body1_e27665: f64 = (locals.var_m0 + 1.0);
        (assign28700_body1_e27665,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign28700_body1_e27667;
        }

        let (assign28710_e27691, assign28710_e27691_d_n0, assign28710_e27691_d_n2, assign28710_e27691_d_n4, assign28710_e27691_d_n5, assign28710_e27691_d_n6, assign28710_e27691_d_n7, assign28710_e27691_d_n8, assign28710_e27691_d_n9, assign28710_e27691_d_n10, assign28710_e27691_d_n11, assign28710_e27691_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 == 0.0)) {
        let (assign28710_e27689, assign28710_e27689_d_n0, assign28710_e27689_d_n2, assign28710_e27689_d_n4, assign28710_e27689_d_n5, assign28710_e27689_d_n6, assign28710_e27689_d_n7, assign28710_e27689_d_n8, assign28710_e27689_d_n9, assign28710_e27689_d_n10, assign28710_e27689_d_n11, assign28710_e27689_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign28710_e27686: f64 = (2.0 * 2.0);
                let assign28710_e27687: f64 = (1.0 / assign28710_e27686);
                let assign28710_e27688: f64 = (locals.var_dnm).powf(assign28710_e27687);
                (assign28710_e27688, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn0)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn2)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn4)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn5)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn6)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn7)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn8)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn9)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn10)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn11)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign28710_e27687) as f64).is_finite() && ((assign28710_e27687) as f64).fract() == 0.0 { if assign28710_e27687 == 0.0 { 0.0 } else { (assign28710_e27687 * ((locals.var_dnm).powf(assign28710_e27687 - 1.0) * locals.var_dnm_dn14)) } } else { (assign28710_e27688 * (assign28710_e27687 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign28710_e27689, assign28710_e27689_d_n0, assign28710_e27689_d_n2, assign28710_e27689_d_n4, assign28710_e27689_d_n5, assign28710_e27689_d_n6, assign28710_e27689_d_n7, assign28710_e27689_d_n8, assign28710_e27689_d_n9, assign28710_e27689_d_n10, assign28710_e27689_d_n11, assign28710_e27689_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28710_e27691;
        locals.var_dnm_dn0 = assign28710_e27691_d_n0;
        locals.var_dnm_dn2 = assign28710_e27691_d_n2;
        locals.var_dnm_dn4 = assign28710_e27691_d_n4;
        locals.var_dnm_dn5 = assign28710_e27691_d_n5;
        locals.var_dnm_dn6 = assign28710_e27691_d_n6;
        locals.var_dnm_dn7 = assign28710_e27691_d_n7;
        locals.var_dnm_dn8 = assign28710_e27691_d_n8;
        locals.var_dnm_dn9 = assign28710_e27691_d_n9;
        locals.var_dnm_dn10 = assign28710_e27691_d_n10;
        locals.var_dnm_dn11 = assign28710_e27691_d_n11;
        locals.var_dnm_dn14 = assign28710_e27691_d_n14;

        let (assign28720_e27703, assign28720_e27703_d_n0, assign28720_e27703_d_n2, assign28720_e27703_d_n4, assign28720_e27703_d_n5, assign28720_e27703_d_n6, assign28720_e27703_d_n7, assign28720_e27703_d_n8, assign28720_e27703_d_n9, assign28720_e27703_d_n10, assign28720_e27703_d_n11, assign28720_e27703_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28720_e27701: f64 = (1.0 / locals.var_dnm);
        (assign28720_e27701, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28720_e27703;
        locals.var_dnm_dn0 = assign28720_e27703_d_n0;
        locals.var_dnm_dn2 = assign28720_e27703_d_n2;
        locals.var_dnm_dn4 = assign28720_e27703_d_n4;
        locals.var_dnm_dn5 = assign28720_e27703_d_n5;
        locals.var_dnm_dn6 = assign28720_e27703_d_n6;
        locals.var_dnm_dn7 = assign28720_e27703_d_n7;
        locals.var_dnm_dn8 = assign28720_e27703_d_n8;
        locals.var_dnm_dn9 = assign28720_e27703_d_n9;
        locals.var_dnm_dn10 = assign28720_e27703_d_n10;
        locals.var_dnm_dn11 = assign28720_e27703_d_n11;
        locals.var_dnm_dn14 = assign28720_e27703_d_n14;

        let (assign28730_e27717, assign28730_e27717_d_n0, assign28730_e27717_d_n2, assign28730_e27717_d_n4, assign28730_e27717_d_n5, assign28730_e27717_d_n6, assign28730_e27717_d_n7, assign28730_e27717_d_n8, assign28730_e27717_d_n9, assign28730_e27717_d_n10, assign28730_e27717_d_n11, assign28730_e27717_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28730_e27713: f64 = (locals.var_tmf1 * 0.8);
        let assign28730_e27715: f64 = (assign28730_e27713 * locals.var_dnm);
        (assign28730_e27715, (((locals.var_tmf1_dn0 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.8) * locals.var_dnm) + (assign28730_e27713 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign28730_e27717;
        locals.var_tmf0_dn0 = assign28730_e27717_d_n0;
        locals.var_tmf0_dn2 = assign28730_e27717_d_n2;
        locals.var_tmf0_dn4 = assign28730_e27717_d_n4;
        locals.var_tmf0_dn5 = assign28730_e27717_d_n5;
        locals.var_tmf0_dn6 = assign28730_e27717_d_n6;
        locals.var_tmf0_dn7 = assign28730_e27717_d_n7;
        locals.var_tmf0_dn8 = assign28730_e27717_d_n8;
        locals.var_tmf0_dn9 = assign28730_e27717_d_n9;
        locals.var_tmf0_dn10 = assign28730_e27717_d_n10;
        locals.var_tmf0_dn11 = assign28730_e27717_d_n11;
        locals.var_tmf0_dn14 = assign28730_e27717_d_n14;

        let (assign28740_e27733, assign28740_e27733_d_n0, assign28740_e27733_d_n2, assign28740_e27733_d_n4, assign28740_e27733_d_n5, assign28740_e27733_d_n6, assign28740_e27733_d_n7, assign28740_e27733_d_n8, assign28740_e27733_d_n9, assign28740_e27733_d_n10, assign28740_e27733_d_n11, assign28740_e27733_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28740_e27727: f64 = (0.8 * locals.var_xmp);
        let assign28740_e27729: f64 = (assign28740_e27727 * locals.var_dnm);
        let assign28740_e27731: f64 = (assign28740_e27729 / locals.var_arg);
        (assign28740_e27731, ((((((0.8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn0)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn2)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn4)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn5)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn6)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn7)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn8)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn9)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn10)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn11)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign28740_e27727 * locals.var_dnm_dn14)) * locals.var_arg) - (assign28740_e27729 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28740_e27733;
        locals.var_t1_dn0 = assign28740_e27733_d_n0;
        locals.var_t1_dn2 = assign28740_e27733_d_n2;
        locals.var_t1_dn4 = assign28740_e27733_d_n4;
        locals.var_t1_dn5 = assign28740_e27733_d_n5;
        locals.var_t1_dn6 = assign28740_e27733_d_n6;
        locals.var_t1_dn7 = assign28740_e27733_d_n7;
        locals.var_t1_dn8 = assign28740_e27733_d_n8;
        locals.var_t1_dn9 = assign28740_e27733_d_n9;
        locals.var_t1_dn10 = assign28740_e27733_d_n10;
        locals.var_t1_dn11 = assign28740_e27733_d_n11;
        locals.var_t1_dn14 = assign28740_e27733_d_n14;

        let (assign28750_e27747, assign28750_e27747_d_n0, assign28750_e27747_d_n2, assign28750_e27747_d_n4, assign28750_e27747_d_n5, assign28750_e27747_d_n6, assign28750_e27747_d_n7, assign28750_e27747_d_n8, assign28750_e27747_d_n9, assign28750_e27747_d_n10, assign28750_e27747_d_n11, assign28750_e27747_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        let assign28750_e27743: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28750_e27745: f64 = (assign28750_e27743 + locals.var_tmf0);
        (assign28750_e27745, (locals.var_vds_maxb0_dn0 + locals.var_tmf0_dn0), (locals.var_vds_maxb0_dn2 + locals.var_tmf0_dn2), (locals.var_vds_maxb0_dn4 + locals.var_tmf0_dn4), (locals.var_vds_maxb0_dn5 + locals.var_tmf0_dn5), (locals.var_vds_maxb0_dn6 + locals.var_tmf0_dn6), (locals.var_vds_maxb0_dn7 + locals.var_tmf0_dn7), (locals.var_vds_maxb0_dn8 + locals.var_tmf0_dn8), (locals.var_vds_maxb0_dn9 + locals.var_tmf0_dn9), (locals.var_vds_maxb0_dn10 + locals.var_tmf0_dn10), (locals.var_vds_maxb0_dn11 + locals.var_tmf0_dn11), (locals.var_vds_maxb0_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28750_e27747;
        locals.var_t2_dn0 = assign28750_e27747_d_n0;
        locals.var_t2_dn2 = assign28750_e27747_d_n2;
        locals.var_t2_dn4 = assign28750_e27747_d_n4;
        locals.var_t2_dn5 = assign28750_e27747_d_n5;
        locals.var_t2_dn6 = assign28750_e27747_d_n6;
        locals.var_t2_dn7 = assign28750_e27747_d_n7;
        locals.var_t2_dn8 = assign28750_e27747_d_n8;
        locals.var_t2_dn9 = assign28750_e27747_d_n9;
        locals.var_t2_dn10 = assign28750_e27747_d_n10;
        locals.var_t2_dn11 = assign28750_e27747_d_n11;
        locals.var_t2_dn14 = assign28750_e27747_d_n14;

        let (assign28760_e27757, assign28760_e27757_d_n0, assign28760_e27757_d_n2, assign28760_e27757_d_n4, assign28760_e27757_d_n5, assign28760_e27757_d_n6, assign28760_e27757_d_n7, assign28760_e27757_d_n8, assign28760_e27757_d_n9, assign28760_e27757_d_n10, assign28760_e27757_d_n11, assign28760_e27757_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28760_e27757;
        locals.var_t1_dn0 = assign28760_e27757_d_n0;
        locals.var_t1_dn2 = assign28760_e27757_d_n2;
        locals.var_t1_dn4 = assign28760_e27757_d_n4;
        locals.var_t1_dn5 = assign28760_e27757_d_n5;
        locals.var_t1_dn6 = assign28760_e27757_d_n6;
        locals.var_t1_dn7 = assign28760_e27757_d_n7;
        locals.var_t1_dn8 = assign28760_e27757_d_n8;
        locals.var_t1_dn9 = assign28760_e27757_d_n9;
        locals.var_t1_dn10 = assign28760_e27757_d_n10;
        locals.var_t1_dn11 = assign28760_e27757_d_n11;
        locals.var_t1_dn14 = assign28760_e27757_d_n14;

        let (assign28770_e27768, assign28770_e27768_d_n0, assign28770_e27768_d_n2, assign28770_e27768_d_n4, assign28770_e27768_d_n5, assign28770_e27768_d_n6, assign28770_e27768_d_n7, assign28770_e27768_d_n8, assign28770_e27768_d_n9, assign28770_e27768_d_n10, assign28770_e27768_d_n11, assign28770_e27768_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 == 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign28770_e27768;
        locals.var_t2_dn0 = assign28770_e27768_d_n0;
        locals.var_t2_dn2 = assign28770_e27768_d_n2;
        locals.var_t2_dn4 = assign28770_e27768_d_n4;
        locals.var_t2_dn5 = assign28770_e27768_d_n5;
        locals.var_t2_dn6 = assign28770_e27768_d_n6;
        locals.var_t2_dn7 = assign28770_e27768_d_n7;
        locals.var_t2_dn8 = assign28770_e27768_d_n8;
        locals.var_t2_dn9 = assign28770_e27768_d_n9;
        locals.var_t2_dn10 = assign28770_e27768_d_n10;
        locals.var_t2_dn11 = assign28770_e27768_d_n11;
        locals.var_t2_dn14 = assign28770_e27768_d_n14;

        let (assign28780_e27779, assign28780_e27779_d_n0, assign28780_e27779_d_n2, assign28780_e27779_d_n4, assign28780_e27779_d_n5, assign28780_e27779_d_n6, assign28780_e27779_d_n7, assign28780_e27779_d_n8, assign28780_e27779_d_n9, assign28780_e27779_d_n10, assign28780_e27779_d_n11, assign28780_e27779_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign28780_e27779;
        locals.var_t1_dn0 = assign28780_e27779_d_n0;
        locals.var_t1_dn2 = assign28780_e27779_d_n2;
        locals.var_t1_dn4 = assign28780_e27779_d_n4;
        locals.var_t1_dn5 = assign28780_e27779_d_n5;
        locals.var_t1_dn6 = assign28780_e27779_d_n6;
        locals.var_t1_dn7 = assign28780_e27779_d_n7;
        locals.var_t1_dn8 = assign28780_e27779_d_n8;
        locals.var_t1_dn9 = assign28780_e27779_d_n9;
        locals.var_t1_dn10 = assign28780_e27779_d_n10;
        locals.var_t1_dn11 = assign28780_e27779_d_n11;
        locals.var_t1_dn14 = assign28780_e27779_d_n14;

        let assign28790_e27783: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28790_e27788: f64 = if ((locals.var_phib_ref > assign28790_e27783) && (0.8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard676 = assign28790_e27788;

        let (assign28800_e27803, assign28800_e27803_d_n0, assign28800_e27803_d_n2, assign28800_e27803_d_n4, assign28800_e27803_d_n5, assign28800_e27803_d_n6, assign28800_e27803_d_n7, assign28800_e27803_d_n8, assign28800_e27803_d_n9, assign28800_e27803_d_n10, assign28800_e27803_d_n11, assign28800_e27803_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign28800_e27799: f64 = (locals.var_phib_ref - locals.var_vds_maxb0);
        let assign28800_e27801: f64 = (assign28800_e27799 + 0.8);
        (assign28800_e27801, (locals.var_phib_ref_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phib_ref_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phib_ref_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phib_ref_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phib_ref_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phib_ref_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phib_ref_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phib_ref_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phib_ref_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phib_ref_dn11 - locals.var_vds_maxb0_dn11), (locals.var_phib_ref_dn14 - locals.var_vds_maxb0_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign28800_e27803;
        locals.var_tmf1_dn0 = assign28800_e27803_d_n0;
        locals.var_tmf1_dn2 = assign28800_e27803_d_n2;
        locals.var_tmf1_dn4 = assign28800_e27803_d_n4;
        locals.var_tmf1_dn5 = assign28800_e27803_d_n5;
        locals.var_tmf1_dn6 = assign28800_e27803_d_n6;
        locals.var_tmf1_dn7 = assign28800_e27803_d_n7;
        locals.var_tmf1_dn8 = assign28800_e27803_d_n8;
        locals.var_tmf1_dn9 = assign28800_e27803_d_n9;
        locals.var_tmf1_dn10 = assign28800_e27803_d_n10;
        locals.var_tmf1_dn11 = assign28800_e27803_d_n11;
        locals.var_tmf1_dn14 = assign28800_e27803_d_n14;

        let (assign28810_e27816, assign28810_e27816_d_n0, assign28810_e27816_d_n2, assign28810_e27816_d_n4, assign28810_e27816_d_n5, assign28810_e27816_d_n6, assign28810_e27816_d_n7, assign28810_e27816_d_n8, assign28810_e27816_d_n9, assign28810_e27816_d_n10, assign28810_e27816_d_n11, assign28810_e27816_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign28810_e27814: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28810_e27814, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign28810_e27816;
        locals.var_x2_dn0 = assign28810_e27816_d_n0;
        locals.var_x2_dn2 = assign28810_e27816_d_n2;
        locals.var_x2_dn4 = assign28810_e27816_d_n4;
        locals.var_x2_dn5 = assign28810_e27816_d_n5;
        locals.var_x2_dn6 = assign28810_e27816_d_n6;
        locals.var_x2_dn7 = assign28810_e27816_d_n7;
        locals.var_x2_dn8 = assign28810_e27816_d_n8;
        locals.var_x2_dn9 = assign28810_e27816_d_n9;
        locals.var_x2_dn10 = assign28810_e27816_d_n10;
        locals.var_x2_dn11 = assign28810_e27816_d_n11;
        locals.var_x2_dn14 = assign28810_e27816_d_n14;

        let (assign28820_e27829, assign28820_e27829_d_n0, assign28820_e27829_d_n2, assign28820_e27829_d_n4, assign28820_e27829_d_n5, assign28820_e27829_d_n6, assign28820_e27829_d_n7, assign28820_e27829_d_n8, assign28820_e27829_d_n9, assign28820_e27829_d_n10, assign28820_e27829_d_n11, assign28820_e27829_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign28820_e27827: f64 = (0.8 * 0.8);
        (assign28820_e27827, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign28820_e27829;
        locals.var_xmax2_dn0 = assign28820_e27829_d_n0;
        locals.var_xmax2_dn2 = assign28820_e27829_d_n2;
        locals.var_xmax2_dn4 = assign28820_e27829_d_n4;
        locals.var_xmax2_dn5 = assign28820_e27829_d_n5;
        locals.var_xmax2_dn6 = assign28820_e27829_d_n6;
        locals.var_xmax2_dn7 = assign28820_e27829_d_n7;
        locals.var_xmax2_dn8 = assign28820_e27829_d_n8;
        locals.var_xmax2_dn9 = assign28820_e27829_d_n9;
        locals.var_xmax2_dn10 = assign28820_e27829_d_n10;
        locals.var_xmax2_dn11 = assign28820_e27829_d_n11;
        locals.var_xmax2_dn14 = assign28820_e27829_d_n14;

        let (assign28830_e27840, assign28830_e27840_d_n0, assign28830_e27840_d_n2, assign28830_e27840_d_n4, assign28830_e27840_d_n5, assign28830_e27840_d_n6, assign28830_e27840_d_n7, assign28830_e27840_d_n8, assign28830_e27840_d_n9, assign28830_e27840_d_n10, assign28830_e27840_d_n11, assign28830_e27840_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28830_e27840;
        locals.var_xp_dn0 = assign28830_e27840_d_n0;
        locals.var_xp_dn2 = assign28830_e27840_d_n2;
        locals.var_xp_dn4 = assign28830_e27840_d_n4;
        locals.var_xp_dn5 = assign28830_e27840_d_n5;
        locals.var_xp_dn6 = assign28830_e27840_d_n6;
        locals.var_xp_dn7 = assign28830_e27840_d_n7;
        locals.var_xp_dn8 = assign28830_e27840_d_n8;
        locals.var_xp_dn9 = assign28830_e27840_d_n9;
        locals.var_xp_dn10 = assign28830_e27840_d_n10;
        locals.var_xp_dn11 = assign28830_e27840_d_n11;
        locals.var_xp_dn14 = assign28830_e27840_d_n14;

        let (assign28840_e27851, assign28840_e27851_d_n0, assign28840_e27851_d_n2, assign28840_e27851_d_n4, assign28840_e27851_d_n5, assign28840_e27851_d_n6, assign28840_e27851_d_n7, assign28840_e27851_d_n8, assign28840_e27851_d_n9, assign28840_e27851_d_n10, assign28840_e27851_d_n11, assign28840_e27851_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28840_e27851;
        locals.var_xmp_dn0 = assign28840_e27851_d_n0;
        locals.var_xmp_dn2 = assign28840_e27851_d_n2;
        locals.var_xmp_dn4 = assign28840_e27851_d_n4;
        locals.var_xmp_dn5 = assign28840_e27851_d_n5;
        locals.var_xmp_dn6 = assign28840_e27851_d_n6;
        locals.var_xmp_dn7 = assign28840_e27851_d_n7;
        locals.var_xmp_dn8 = assign28840_e27851_d_n8;
        locals.var_xmp_dn9 = assign28840_e27851_d_n9;
        locals.var_xmp_dn10 = assign28840_e27851_d_n10;
        locals.var_xmp_dn11 = assign28840_e27851_d_n11;
        locals.var_xmp_dn14 = assign28840_e27851_d_n14;

        let (assign28850_e27862,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28850_e27862;

        let (assign28860_e27873,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28860_e27873;

        let (assign28870_e27884, assign28870_e27884_d_n0, assign28870_e27884_d_n2, assign28870_e27884_d_n4, assign28870_e27884_d_n5, assign28870_e27884_d_n6, assign28870_e27884_d_n7, assign28870_e27884_d_n8, assign28870_e27884_d_n9, assign28870_e27884_d_n10, assign28870_e27884_d_n11, assign28870_e27884_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28870_e27884;
        locals.var_arg_dn0 = assign28870_e27884_d_n0;
        locals.var_arg_dn2 = assign28870_e27884_d_n2;
        locals.var_arg_dn4 = assign28870_e27884_d_n4;
        locals.var_arg_dn5 = assign28870_e27884_d_n5;
        locals.var_arg_dn6 = assign28870_e27884_d_n6;
        locals.var_arg_dn7 = assign28870_e27884_d_n7;
        locals.var_arg_dn8 = assign28870_e27884_d_n8;
        locals.var_arg_dn9 = assign28870_e27884_d_n9;
        locals.var_arg_dn10 = assign28870_e27884_d_n10;
        locals.var_arg_dn11 = assign28870_e27884_d_n11;
        locals.var_arg_dn14 = assign28870_e27884_d_n14;

        let (assign28880_e27895, assign28880_e27895_d_n0, assign28880_e27895_d_n2, assign28880_e27895_d_n4, assign28880_e27895_d_n5, assign28880_e27895_d_n6, assign28880_e27895_d_n7, assign28880_e27895_d_n8, assign28880_e27895_d_n9, assign28880_e27895_d_n10, assign28880_e27895_d_n11, assign28880_e27895_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28880_e27895;
        locals.var_dnm_dn0 = assign28880_e27895_d_n0;
        locals.var_dnm_dn2 = assign28880_e27895_d_n2;
        locals.var_dnm_dn4 = assign28880_e27895_d_n4;
        locals.var_dnm_dn5 = assign28880_e27895_d_n5;
        locals.var_dnm_dn6 = assign28880_e27895_d_n6;
        locals.var_dnm_dn7 = assign28880_e27895_d_n7;
        locals.var_dnm_dn8 = assign28880_e27895_d_n8;
        locals.var_dnm_dn9 = assign28880_e27895_d_n9;
        locals.var_dnm_dn10 = assign28880_e27895_d_n10;
        locals.var_dnm_dn11 = assign28880_e27895_d_n11;
        locals.var_dnm_dn14 = assign28880_e27895_d_n14;

        let (assign28890_e27908, assign28890_e27908_d_n0, assign28890_e27908_d_n2, assign28890_e27908_d_n4, assign28890_e27908_d_n5, assign28890_e27908_d_n6, assign28890_e27908_d_n7, assign28890_e27908_d_n8, assign28890_e27908_d_n9, assign28890_e27908_d_n10, assign28890_e27908_d_n11, assign28890_e27908_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign28890_e27906: f64 = (locals.var_xp * locals.var_x2);
        (assign28890_e27906, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28890_e27908;
        locals.var_xp_dn0 = assign28890_e27908_d_n0;
        locals.var_xp_dn2 = assign28890_e27908_d_n2;
        locals.var_xp_dn4 = assign28890_e27908_d_n4;
        locals.var_xp_dn5 = assign28890_e27908_d_n5;
        locals.var_xp_dn6 = assign28890_e27908_d_n6;
        locals.var_xp_dn7 = assign28890_e27908_d_n7;
        locals.var_xp_dn8 = assign28890_e27908_d_n8;
        locals.var_xp_dn9 = assign28890_e27908_d_n9;
        locals.var_xp_dn10 = assign28890_e27908_d_n10;
        locals.var_xp_dn11 = assign28890_e27908_d_n11;
        locals.var_xp_dn14 = assign28890_e27908_d_n14;

        let (assign28900_e27921, assign28900_e27921_d_n0, assign28900_e27921_d_n2, assign28900_e27921_d_n4, assign28900_e27921_d_n5, assign28900_e27921_d_n6, assign28900_e27921_d_n7, assign28900_e27921_d_n8, assign28900_e27921_d_n9, assign28900_e27921_d_n10, assign28900_e27921_d_n11, assign28900_e27921_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign28900_e27919: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28900_e27919, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28900_e27921;
        locals.var_xmp_dn0 = assign28900_e27921_d_n0;
        locals.var_xmp_dn2 = assign28900_e27921_d_n2;
        locals.var_xmp_dn4 = assign28900_e27921_d_n4;
        locals.var_xmp_dn5 = assign28900_e27921_d_n5;
        locals.var_xmp_dn6 = assign28900_e27921_d_n6;
        locals.var_xmp_dn7 = assign28900_e27921_d_n7;
        locals.var_xmp_dn8 = assign28900_e27921_d_n8;
        locals.var_xmp_dn9 = assign28900_e27921_d_n9;
        locals.var_xmp_dn10 = assign28900_e27921_d_n10;
        locals.var_xmp_dn11 = assign28900_e27921_d_n11;
        locals.var_xmp_dn14 = assign28900_e27921_d_n14;

    }

    pub(super) fn stamp_transient_block_83(
        locals: &mut StampLocals,
    ) {
        let (assign28910_e27934, assign28910_e27934_d_n0, assign28910_e27934_d_n2, assign28910_e27934_d_n4, assign28910_e27934_d_n5, assign28910_e27934_d_n6, assign28910_e27934_d_n7, assign28910_e27934_d_n8, assign28910_e27934_d_n9, assign28910_e27934_d_n10, assign28910_e27934_d_n11, assign28910_e27934_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign28910_e27932: f64 = (locals.var_xp * locals.var_x2);
        (assign28910_e27932, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign28910_e27934;
        locals.var_xp_dn0 = assign28910_e27934_d_n0;
        locals.var_xp_dn2 = assign28910_e27934_d_n2;
        locals.var_xp_dn4 = assign28910_e27934_d_n4;
        locals.var_xp_dn5 = assign28910_e27934_d_n5;
        locals.var_xp_dn6 = assign28910_e27934_d_n6;
        locals.var_xp_dn7 = assign28910_e27934_d_n7;
        locals.var_xp_dn8 = assign28910_e27934_d_n8;
        locals.var_xp_dn9 = assign28910_e27934_d_n9;
        locals.var_xp_dn10 = assign28910_e27934_d_n10;
        locals.var_xp_dn11 = assign28910_e27934_d_n11;
        locals.var_xp_dn14 = assign28910_e27934_d_n14;

        let (assign28920_e27947, assign28920_e27947_d_n0, assign28920_e27947_d_n2, assign28920_e27947_d_n4, assign28920_e27947_d_n5, assign28920_e27947_d_n6, assign28920_e27947_d_n7, assign28920_e27947_d_n8, assign28920_e27947_d_n9, assign28920_e27947_d_n10, assign28920_e27947_d_n11, assign28920_e27947_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign28920_e27945: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign28920_e27945, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign28920_e27947;
        locals.var_xmp_dn0 = assign28920_e27947_d_n0;
        locals.var_xmp_dn2 = assign28920_e27947_d_n2;
        locals.var_xmp_dn4 = assign28920_e27947_d_n4;
        locals.var_xmp_dn5 = assign28920_e27947_d_n5;
        locals.var_xmp_dn6 = assign28920_e27947_d_n6;
        locals.var_xmp_dn7 = assign28920_e27947_d_n7;
        locals.var_xmp_dn8 = assign28920_e27947_d_n8;
        locals.var_xmp_dn9 = assign28920_e27947_d_n9;
        locals.var_xmp_dn10 = assign28920_e27947_d_n10;
        locals.var_xmp_dn11 = assign28920_e27947_d_n11;
        locals.var_xmp_dn14 = assign28920_e27947_d_n14;

        let (assign28930_e27960, assign28930_e27960_d_n0, assign28930_e27960_d_n2, assign28930_e27960_d_n4, assign28930_e27960_d_n5, assign28930_e27960_d_n6, assign28930_e27960_d_n7, assign28930_e27960_d_n8, assign28930_e27960_d_n9, assign28930_e27960_d_n10, assign28930_e27960_d_n11, assign28930_e27960_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign28930_e27958: f64 = (locals.var_xp + locals.var_xmp);
        (assign28930_e27958, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign28930_e27960;
        locals.var_arg_dn0 = assign28930_e27960_d_n0;
        locals.var_arg_dn2 = assign28930_e27960_d_n2;
        locals.var_arg_dn4 = assign28930_e27960_d_n4;
        locals.var_arg_dn5 = assign28930_e27960_d_n5;
        locals.var_arg_dn6 = assign28930_e27960_d_n6;
        locals.var_arg_dn7 = assign28930_e27960_d_n7;
        locals.var_arg_dn8 = assign28930_e27960_d_n8;
        locals.var_arg_dn9 = assign28930_e27960_d_n9;
        locals.var_arg_dn10 = assign28930_e27960_d_n10;
        locals.var_arg_dn11 = assign28930_e27960_d_n11;
        locals.var_arg_dn14 = assign28930_e27960_d_n14;

        let (assign28940_e27971, assign28940_e27971_d_n0, assign28940_e27971_d_n2, assign28940_e27971_d_n4, assign28940_e27971_d_n5, assign28940_e27971_d_n6, assign28940_e27971_d_n7, assign28940_e27971_d_n8, assign28940_e27971_d_n9, assign28940_e27971_d_n10, assign28940_e27971_d_n11, assign28940_e27971_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign28940_e27971;
        locals.var_dnm_dn0 = assign28940_e27971_d_n0;
        locals.var_dnm_dn2 = assign28940_e27971_d_n2;
        locals.var_dnm_dn4 = assign28940_e27971_d_n4;
        locals.var_dnm_dn5 = assign28940_e27971_d_n5;
        locals.var_dnm_dn6 = assign28940_e27971_d_n6;
        locals.var_dnm_dn7 = assign28940_e27971_d_n7;
        locals.var_dnm_dn8 = assign28940_e27971_d_n8;
        locals.var_dnm_dn9 = assign28940_e27971_d_n9;
        locals.var_dnm_dn10 = assign28940_e27971_d_n10;
        locals.var_dnm_dn11 = assign28940_e27971_d_n11;
        locals.var_dnm_dn14 = assign28940_e27971_d_n14;

        let assign28950_e27986: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard677 = assign28950_e27986;

        let assign28960_e27989: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard678 = assign28960_e27989;

        let (assign28970_e28004,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28970_e28004;

        let assign28980_e28007: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard679 = assign28980_e28007;

        let (assign28990_e28025,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 == 0.0)) && (locals.var_guard679 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28990_e28025;

        let assign29000_e28028: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard680 = assign29000_e28028;

        let (assign29010_e28049,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 == 0.0)) && (locals.var_guard679 == 0.0)) && (locals.var_guard680 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29010_e28049;

        let assign29020_e28052: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard681 = assign29020_e28052;

        let (assign29030_e28076,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 == 0.0)) && (locals.var_guard679 == 0.0)) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29030_e28076;

        let (assign29040_e28089,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29040_e28089;

        let mut assign29050_loop_guard: usize = 0;
        while {
            let assign29050_cond_e28103: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29050_cond_e28103 != 0.0
        } {
            assign29050_loop_guard += 1;
            assert!(assign29050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29050_body0_e28117, assign29050_body0_e28117_d_n0, assign29050_body0_e28117_d_n2, assign29050_body0_e28117_d_n4, assign29050_body0_e28117_d_n5, assign29050_body0_e28117_d_n6, assign29050_body0_e28117_d_n7, assign29050_body0_e28117_d_n8, assign29050_body0_e28117_d_n9, assign29050_body0_e28117_d_n10, assign29050_body0_e28117_d_n11, assign29050_body0_e28117_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) {
        let assign29050_body0_e28115: f64 = (locals.var_dnm).sqrt();
        (assign29050_body0_e28115, (locals.var_dnm_dn0 / (2.0 * assign29050_body0_e28115)), (locals.var_dnm_dn2 / (2.0 * assign29050_body0_e28115)), (locals.var_dnm_dn4 / (2.0 * assign29050_body0_e28115)), (locals.var_dnm_dn5 / (2.0 * assign29050_body0_e28115)), (locals.var_dnm_dn6 / (2.0 * assign29050_body0_e28115)), (locals.var_dnm_dn7 / (2.0 * assign29050_body0_e28115)), (locals.var_dnm_dn8 / (2.0 * assign29050_body0_e28115)), (locals.var_dnm_dn9 / (2.0 * assign29050_body0_e28115)), (locals.var_dnm_dn10 / (2.0 * assign29050_body0_e28115)), (locals.var_dnm_dn11 / (2.0 * assign29050_body0_e28115)), (locals.var_dnm_dn14 / (2.0 * assign29050_body0_e28115)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign29050_body0_e28117;
            locals.var_dnm_dn0 = assign29050_body0_e28117_d_n0;
            locals.var_dnm_dn2 = assign29050_body0_e28117_d_n2;
            locals.var_dnm_dn4 = assign29050_body0_e28117_d_n4;
            locals.var_dnm_dn5 = assign29050_body0_e28117_d_n5;
            locals.var_dnm_dn6 = assign29050_body0_e28117_d_n6;
            locals.var_dnm_dn7 = assign29050_body0_e28117_d_n7;
            locals.var_dnm_dn8 = assign29050_body0_e28117_d_n8;
            locals.var_dnm_dn9 = assign29050_body0_e28117_d_n9;
            locals.var_dnm_dn10 = assign29050_body0_e28117_d_n10;
            locals.var_dnm_dn11 = assign29050_body0_e28117_d_n11;
            locals.var_dnm_dn14 = assign29050_body0_e28117_d_n14;
            let (assign29050_body1_e28132,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) {
        let assign29050_body1_e28130: f64 = (locals.var_m0 + 1.0);
        (assign29050_body1_e28130,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29050_body1_e28132;
        }

        let (assign29060_e28157, assign29060_e28157_d_n0, assign29060_e28157_d_n2, assign29060_e28157_d_n4, assign29060_e28157_d_n5, assign29060_e28157_d_n6, assign29060_e28157_d_n7, assign29060_e28157_d_n8, assign29060_e28157_d_n9, assign29060_e28157_d_n10, assign29060_e28157_d_n11, assign29060_e28157_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 == 0.0)) {
        let (assign29060_e28155, assign29060_e28155_d_n0, assign29060_e28155_d_n2, assign29060_e28155_d_n4, assign29060_e28155_d_n5, assign29060_e28155_d_n6, assign29060_e28155_d_n7, assign29060_e28155_d_n8, assign29060_e28155_d_n9, assign29060_e28155_d_n10, assign29060_e28155_d_n11, assign29060_e28155_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29060_e28152: f64 = (2.0 * 2.0);
                let assign29060_e28153: f64 = (1.0 / assign29060_e28152);
                let assign29060_e28154: f64 = (locals.var_dnm).powf(assign29060_e28153);
                (assign29060_e28154, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn11)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29060_e28153) as f64).is_finite() && ((assign29060_e28153) as f64).fract() == 0.0 { if assign29060_e28153 == 0.0 { 0.0 } else { (assign29060_e28153 * ((locals.var_dnm).powf(assign29060_e28153 - 1.0) * locals.var_dnm_dn14)) } } else { (assign29060_e28154 * (assign29060_e28153 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign29060_e28155, assign29060_e28155_d_n0, assign29060_e28155_d_n2, assign29060_e28155_d_n4, assign29060_e28155_d_n5, assign29060_e28155_d_n6, assign29060_e28155_d_n7, assign29060_e28155_d_n8, assign29060_e28155_d_n9, assign29060_e28155_d_n10, assign29060_e28155_d_n11, assign29060_e28155_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29060_e28157;
        locals.var_dnm_dn0 = assign29060_e28157_d_n0;
        locals.var_dnm_dn2 = assign29060_e28157_d_n2;
        locals.var_dnm_dn4 = assign29060_e28157_d_n4;
        locals.var_dnm_dn5 = assign29060_e28157_d_n5;
        locals.var_dnm_dn6 = assign29060_e28157_d_n6;
        locals.var_dnm_dn7 = assign29060_e28157_d_n7;
        locals.var_dnm_dn8 = assign29060_e28157_d_n8;
        locals.var_dnm_dn9 = assign29060_e28157_d_n9;
        locals.var_dnm_dn10 = assign29060_e28157_d_n10;
        locals.var_dnm_dn11 = assign29060_e28157_d_n11;
        locals.var_dnm_dn14 = assign29060_e28157_d_n14;

        let (assign29070_e28170, assign29070_e28170_d_n0, assign29070_e28170_d_n2, assign29070_e28170_d_n4, assign29070_e28170_d_n5, assign29070_e28170_d_n6, assign29070_e28170_d_n7, assign29070_e28170_d_n8, assign29070_e28170_d_n9, assign29070_e28170_d_n10, assign29070_e28170_d_n11, assign29070_e28170_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign29070_e28168: f64 = (1.0 / locals.var_dnm);
        (assign29070_e28168, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29070_e28170;
        locals.var_dnm_dn0 = assign29070_e28170_d_n0;
        locals.var_dnm_dn2 = assign29070_e28170_d_n2;
        locals.var_dnm_dn4 = assign29070_e28170_d_n4;
        locals.var_dnm_dn5 = assign29070_e28170_d_n5;
        locals.var_dnm_dn6 = assign29070_e28170_d_n6;
        locals.var_dnm_dn7 = assign29070_e28170_d_n7;
        locals.var_dnm_dn8 = assign29070_e28170_d_n8;
        locals.var_dnm_dn9 = assign29070_e28170_d_n9;
        locals.var_dnm_dn10 = assign29070_e28170_d_n10;
        locals.var_dnm_dn11 = assign29070_e28170_d_n11;
        locals.var_dnm_dn14 = assign29070_e28170_d_n14;

        let (assign29080_e28185, assign29080_e28185_d_n0, assign29080_e28185_d_n2, assign29080_e28185_d_n4, assign29080_e28185_d_n5, assign29080_e28185_d_n6, assign29080_e28185_d_n7, assign29080_e28185_d_n8, assign29080_e28185_d_n9, assign29080_e28185_d_n10, assign29080_e28185_d_n11, assign29080_e28185_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign29080_e28181: f64 = (locals.var_tmf1 * 0.8);
        let assign29080_e28183: f64 = (assign29080_e28181 * locals.var_dnm);
        (assign29080_e28183, (((locals.var_tmf1_dn0 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.8) * locals.var_dnm) + (assign29080_e28181 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign29080_e28185;
        locals.var_tmf0_dn0 = assign29080_e28185_d_n0;
        locals.var_tmf0_dn2 = assign29080_e28185_d_n2;
        locals.var_tmf0_dn4 = assign29080_e28185_d_n4;
        locals.var_tmf0_dn5 = assign29080_e28185_d_n5;
        locals.var_tmf0_dn6 = assign29080_e28185_d_n6;
        locals.var_tmf0_dn7 = assign29080_e28185_d_n7;
        locals.var_tmf0_dn8 = assign29080_e28185_d_n8;
        locals.var_tmf0_dn9 = assign29080_e28185_d_n9;
        locals.var_tmf0_dn10 = assign29080_e28185_d_n10;
        locals.var_tmf0_dn11 = assign29080_e28185_d_n11;
        locals.var_tmf0_dn14 = assign29080_e28185_d_n14;

        let (assign29090_e28202, assign29090_e28202_d_n0, assign29090_e28202_d_n2, assign29090_e28202_d_n4, assign29090_e28202_d_n5, assign29090_e28202_d_n6, assign29090_e28202_d_n7, assign29090_e28202_d_n8, assign29090_e28202_d_n9, assign29090_e28202_d_n10, assign29090_e28202_d_n11, assign29090_e28202_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign29090_e28196: f64 = (0.8 * locals.var_xmp);
        let assign29090_e28198: f64 = (assign29090_e28196 * locals.var_dnm);
        let assign29090_e28200: f64 = (assign29090_e28198 / locals.var_arg);
        (assign29090_e28200, ((((((0.8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn11)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign29090_e28196 * locals.var_dnm_dn14)) * locals.var_arg) - (assign29090_e28198 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29090_e28202;
        locals.var_t0_dn0 = assign29090_e28202_d_n0;
        locals.var_t0_dn2 = assign29090_e28202_d_n2;
        locals.var_t0_dn4 = assign29090_e28202_d_n4;
        locals.var_t0_dn5 = assign29090_e28202_d_n5;
        locals.var_t0_dn6 = assign29090_e28202_d_n6;
        locals.var_t0_dn7 = assign29090_e28202_d_n7;
        locals.var_t0_dn8 = assign29090_e28202_d_n8;
        locals.var_t0_dn9 = assign29090_e28202_d_n9;
        locals.var_t0_dn10 = assign29090_e28202_d_n10;
        locals.var_t0_dn11 = assign29090_e28202_d_n11;
        locals.var_t0_dn14 = assign29090_e28202_d_n14;

        let (assign29100_e28217, assign29100_e28217_d_n0, assign29100_e28217_d_n2, assign29100_e28217_d_n4, assign29100_e28217_d_n5, assign29100_e28217_d_n6, assign29100_e28217_d_n7, assign29100_e28217_d_n8, assign29100_e28217_d_n9, assign29100_e28217_d_n10, assign29100_e28217_d_n11, assign29100_e28217_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        let assign29100_e28213: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign29100_e28215: f64 = (assign29100_e28213 + locals.var_tmf0);
        (assign29100_e28215, (locals.var_vds_maxb0_dn0 + locals.var_tmf0_dn0), (locals.var_vds_maxb0_dn2 + locals.var_tmf0_dn2), (locals.var_vds_maxb0_dn4 + locals.var_tmf0_dn4), (locals.var_vds_maxb0_dn5 + locals.var_tmf0_dn5), (locals.var_vds_maxb0_dn6 + locals.var_tmf0_dn6), (locals.var_vds_maxb0_dn7 + locals.var_tmf0_dn7), (locals.var_vds_maxb0_dn8 + locals.var_tmf0_dn8), (locals.var_vds_maxb0_dn9 + locals.var_tmf0_dn9), (locals.var_vds_maxb0_dn10 + locals.var_tmf0_dn10), (locals.var_vds_maxb0_dn11 + locals.var_tmf0_dn11), (locals.var_vds_maxb0_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29100_e28217;
        locals.var_t2_dn0 = assign29100_e28217_d_n0;
        locals.var_t2_dn2 = assign29100_e28217_d_n2;
        locals.var_t2_dn4 = assign29100_e28217_d_n4;
        locals.var_t2_dn5 = assign29100_e28217_d_n5;
        locals.var_t2_dn6 = assign29100_e28217_d_n6;
        locals.var_t2_dn7 = assign29100_e28217_d_n7;
        locals.var_t2_dn8 = assign29100_e28217_d_n8;
        locals.var_t2_dn9 = assign29100_e28217_d_n9;
        locals.var_t2_dn10 = assign29100_e28217_d_n10;
        locals.var_t2_dn11 = assign29100_e28217_d_n11;
        locals.var_t2_dn14 = assign29100_e28217_d_n14;

        let (assign29110_e28228, assign29110_e28228_d_n0, assign29110_e28228_d_n2, assign29110_e28228_d_n4, assign29110_e28228_d_n5, assign29110_e28228_d_n6, assign29110_e28228_d_n7, assign29110_e28228_d_n8, assign29110_e28228_d_n9, assign29110_e28228_d_n10, assign29110_e28228_d_n11, assign29110_e28228_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29110_e28228;
        locals.var_t0_dn0 = assign29110_e28228_d_n0;
        locals.var_t0_dn2 = assign29110_e28228_d_n2;
        locals.var_t0_dn4 = assign29110_e28228_d_n4;
        locals.var_t0_dn5 = assign29110_e28228_d_n5;
        locals.var_t0_dn6 = assign29110_e28228_d_n6;
        locals.var_t0_dn7 = assign29110_e28228_d_n7;
        locals.var_t0_dn8 = assign29110_e28228_d_n8;
        locals.var_t0_dn9 = assign29110_e28228_d_n9;
        locals.var_t0_dn10 = assign29110_e28228_d_n10;
        locals.var_t0_dn11 = assign29110_e28228_d_n11;
        locals.var_t0_dn14 = assign29110_e28228_d_n14;

        let (assign29120_e28240, assign29120_e28240_d_n0, assign29120_e28240_d_n2, assign29120_e28240_d_n4, assign29120_e28240_d_n5, assign29120_e28240_d_n6, assign29120_e28240_d_n7, assign29120_e28240_d_n8, assign29120_e28240_d_n9, assign29120_e28240_d_n10, assign29120_e28240_d_n11, assign29120_e28240_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 == 0.0)) {
        (locals.var_phib_ref, locals.var_phib_ref_dn0, locals.var_phib_ref_dn2, locals.var_phib_ref_dn4, locals.var_phib_ref_dn5, locals.var_phib_ref_dn6, locals.var_phib_ref_dn7, locals.var_phib_ref_dn8, locals.var_phib_ref_dn9, locals.var_phib_ref_dn10, locals.var_phib_ref_dn11, locals.var_phib_ref_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29120_e28240;
        locals.var_t2_dn0 = assign29120_e28240_d_n0;
        locals.var_t2_dn2 = assign29120_e28240_d_n2;
        locals.var_t2_dn4 = assign29120_e28240_d_n4;
        locals.var_t2_dn5 = assign29120_e28240_d_n5;
        locals.var_t2_dn6 = assign29120_e28240_d_n6;
        locals.var_t2_dn7 = assign29120_e28240_d_n7;
        locals.var_t2_dn8 = assign29120_e28240_d_n8;
        locals.var_t2_dn9 = assign29120_e28240_d_n9;
        locals.var_t2_dn10 = assign29120_e28240_d_n10;
        locals.var_t2_dn11 = assign29120_e28240_d_n11;
        locals.var_t2_dn14 = assign29120_e28240_d_n14;

        let (assign29130_e28252, assign29130_e28252_d_n0, assign29130_e28252_d_n2, assign29130_e28252_d_n4, assign29130_e28252_d_n5, assign29130_e28252_d_n6, assign29130_e28252_d_n7, assign29130_e28252_d_n8, assign29130_e28252_d_n9, assign29130_e28252_d_n10, assign29130_e28252_d_n11, assign29130_e28252_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29130_e28252;
        locals.var_t0_dn0 = assign29130_e28252_d_n0;
        locals.var_t0_dn2 = assign29130_e28252_d_n2;
        locals.var_t0_dn4 = assign29130_e28252_d_n4;
        locals.var_t0_dn5 = assign29130_e28252_d_n5;
        locals.var_t0_dn6 = assign29130_e28252_d_n6;
        locals.var_t0_dn7 = assign29130_e28252_d_n7;
        locals.var_t0_dn8 = assign29130_e28252_d_n8;
        locals.var_t0_dn9 = assign29130_e28252_d_n9;
        locals.var_t0_dn10 = assign29130_e28252_d_n10;
        locals.var_t0_dn11 = assign29130_e28252_d_n11;
        locals.var_t0_dn14 = assign29130_e28252_d_n14;

        let (assign29140_e28270, assign29140_e28270_d_n0, assign29140_e28270_d_n2, assign29140_e28270_d_n4, assign29140_e28270_d_n5, assign29140_e28270_d_n6, assign29140_e28270_d_n7, assign29140_e28270_d_n8, assign29140_e28270_d_n9, assign29140_e28270_d_n10, assign29140_e28270_d_n11, assign29140_e28270_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign29140_e28257: f64 = (-1.6021918e-19);
        let assign29140_e28259: f64 = (assign29140_e28257 * locals.var_uc_ndepm);
        let assign29140_e28263: f64 = (locals.var_t2 - locals.var_vds_maxb0);
        let assign29140_e28264: f64 = (locals.var_beta * assign29140_e28263);
        let assign29140_e28265: f64 = (assign29140_e28264).exp();
        let assign29140_e28266: f64 = (assign29140_e28259 * assign29140_e28265);
        let assign29140_e28268: f64 = (assign29140_e28266 * locals.var_w_b0);
        (assign29140_e28268, (((((assign29140_e28257 * locals.var_uc_ndepm_dn0) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn0 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn0 - locals.var_vds_maxb0_dn0)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn0)), (((((assign29140_e28257 * locals.var_uc_ndepm_dn2) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn2 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn2 - locals.var_vds_maxb0_dn2)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn2)), (((((assign29140_e28257 * locals.var_uc_ndepm_dn4) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn4 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn4 - locals.var_vds_maxb0_dn4)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn4)), (((((assign29140_e28257 * locals.var_uc_ndepm_dn5) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn5 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn5 - locals.var_vds_maxb0_dn5)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn5)), (((((assign29140_e28257 * locals.var_uc_ndepm_dn6) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn6 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn6 - locals.var_vds_maxb0_dn6)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn6)), (((((assign29140_e28257 * locals.var_uc_ndepm_dn7) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn7 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn7 - locals.var_vds_maxb0_dn7)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn7)), (((((assign29140_e28257 * locals.var_uc_ndepm_dn8) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn8 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn8 - locals.var_vds_maxb0_dn8)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn8)), (((((assign29140_e28257 * locals.var_uc_ndepm_dn9) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn9 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn9 - locals.var_vds_maxb0_dn9)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn9)), (((((assign29140_e28257 * locals.var_uc_ndepm_dn10) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn10 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn10 - locals.var_vds_maxb0_dn10)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn10)), (((((assign29140_e28257 * locals.var_uc_ndepm_dn11) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn11 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn11 - locals.var_vds_maxb0_dn11)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn11)), (((((assign29140_e28257 * locals.var_uc_ndepm_dn14) * assign29140_e28265) + (assign29140_e28259 * (assign29140_e28265 * ((locals.var_beta_dn14 * assign29140_e28263) + (locals.var_beta * (locals.var_t2_dn14 - locals.var_vds_maxb0_dn14)))))) * locals.var_w_b0) + (assign29140_e28266 * locals.var_w_b0_dn14)),)
    } else {
        (locals.var_qn_bac, locals.var_qn_bac_dn0, locals.var_qn_bac_dn2, locals.var_qn_bac_dn4, locals.var_qn_bac_dn5, locals.var_qn_bac_dn6, locals.var_qn_bac_dn7, locals.var_qn_bac_dn8, locals.var_qn_bac_dn9, locals.var_qn_bac_dn10, locals.var_qn_bac_dn11, locals.var_qn_bac_dn14,)
    }
};
        locals.var_qn_bac = assign29140_e28270;
        locals.var_qn_bac_dn0 = assign29140_e28270_d_n0;
        locals.var_qn_bac_dn2 = assign29140_e28270_d_n2;
        locals.var_qn_bac_dn4 = assign29140_e28270_d_n4;
        locals.var_qn_bac_dn5 = assign29140_e28270_d_n5;
        locals.var_qn_bac_dn6 = assign29140_e28270_d_n6;
        locals.var_qn_bac_dn7 = assign29140_e28270_d_n7;
        locals.var_qn_bac_dn8 = assign29140_e28270_d_n8;
        locals.var_qn_bac_dn9 = assign29140_e28270_d_n9;
        locals.var_qn_bac_dn10 = assign29140_e28270_d_n10;
        locals.var_qn_bac_dn11 = assign29140_e28270_d_n11;
        locals.var_qn_bac_dn14 = assign29140_e28270_d_n14;

        let assign29150_e28273: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign29150_e28276: f64 = 0.06;
        let assign29150_e28281: f64 = if ((assign29150_e28273 < assign29150_e28276) && (0.06 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard682 = assign29150_e28281;

        let (assign29160_e28295, assign29160_e28295_d_n0, assign29160_e28295_d_n2, assign29160_e28295_d_n4, assign29160_e28295_d_n5, assign29160_e28295_d_n6, assign29160_e28295_d_n7, assign29160_e28295_d_n8, assign29160_e28295_d_n9, assign29160_e28295_d_n10, assign29160_e28295_d_n11, assign29160_e28295_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29160_e28289: f64 = 0.06;
        let assign29160_e28292: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign29160_e28293: f64 = (assign29160_e28289 - assign29160_e28292);
        (assign29160_e28293, (-(locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0)), (-(locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2)), (-(locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4)), (-(locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5)), (-(locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6)), (-(locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7)), (-(locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8)), (-(locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9)), (-(locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10)), (-(locals.var_phi_s0_dep_dn11 - locals.var_vds_maxb0_dn11)), (-(locals.var_phi_s0_dep_dn14 - locals.var_vds_maxb0_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign29160_e28295;
        locals.var_tmf1_dn0 = assign29160_e28295_d_n0;
        locals.var_tmf1_dn2 = assign29160_e28295_d_n2;
        locals.var_tmf1_dn4 = assign29160_e28295_d_n4;
        locals.var_tmf1_dn5 = assign29160_e28295_d_n5;
        locals.var_tmf1_dn6 = assign29160_e28295_d_n6;
        locals.var_tmf1_dn7 = assign29160_e28295_d_n7;
        locals.var_tmf1_dn8 = assign29160_e28295_d_n8;
        locals.var_tmf1_dn9 = assign29160_e28295_d_n9;
        locals.var_tmf1_dn10 = assign29160_e28295_d_n10;
        locals.var_tmf1_dn11 = assign29160_e28295_d_n11;
        locals.var_tmf1_dn14 = assign29160_e28295_d_n14;

        let (assign29170_e28305, assign29170_e28305_d_n0, assign29170_e28305_d_n2, assign29170_e28305_d_n4, assign29170_e28305_d_n5, assign29170_e28305_d_n6, assign29170_e28305_d_n7, assign29170_e28305_d_n8, assign29170_e28305_d_n9, assign29170_e28305_d_n10, assign29170_e28305_d_n11, assign29170_e28305_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29170_e28303: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign29170_e28303, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign29170_e28305;
        locals.var_x2_dn0 = assign29170_e28305_d_n0;
        locals.var_x2_dn2 = assign29170_e28305_d_n2;
        locals.var_x2_dn4 = assign29170_e28305_d_n4;
        locals.var_x2_dn5 = assign29170_e28305_d_n5;
        locals.var_x2_dn6 = assign29170_e28305_d_n6;
        locals.var_x2_dn7 = assign29170_e28305_d_n7;
        locals.var_x2_dn8 = assign29170_e28305_d_n8;
        locals.var_x2_dn9 = assign29170_e28305_d_n9;
        locals.var_x2_dn10 = assign29170_e28305_d_n10;
        locals.var_x2_dn11 = assign29170_e28305_d_n11;
        locals.var_x2_dn14 = assign29170_e28305_d_n14;

        let (assign29180_e28315, assign29180_e28315_d_n0, assign29180_e28315_d_n2, assign29180_e28315_d_n4, assign29180_e28315_d_n5, assign29180_e28315_d_n6, assign29180_e28315_d_n7, assign29180_e28315_d_n8, assign29180_e28315_d_n9, assign29180_e28315_d_n10, assign29180_e28315_d_n11, assign29180_e28315_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29180_e28313: f64 = (0.06 * 0.06);
        (assign29180_e28313, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign29180_e28315;
        locals.var_xmax2_dn0 = assign29180_e28315_d_n0;
        locals.var_xmax2_dn2 = assign29180_e28315_d_n2;
        locals.var_xmax2_dn4 = assign29180_e28315_d_n4;
        locals.var_xmax2_dn5 = assign29180_e28315_d_n5;
        locals.var_xmax2_dn6 = assign29180_e28315_d_n6;
        locals.var_xmax2_dn7 = assign29180_e28315_d_n7;
        locals.var_xmax2_dn8 = assign29180_e28315_d_n8;
        locals.var_xmax2_dn9 = assign29180_e28315_d_n9;
        locals.var_xmax2_dn10 = assign29180_e28315_d_n10;
        locals.var_xmax2_dn11 = assign29180_e28315_d_n11;
        locals.var_xmax2_dn14 = assign29180_e28315_d_n14;

        let (assign29190_e28323, assign29190_e28323_d_n0, assign29190_e28323_d_n2, assign29190_e28323_d_n4, assign29190_e28323_d_n5, assign29190_e28323_d_n6, assign29190_e28323_d_n7, assign29190_e28323_d_n8, assign29190_e28323_d_n9, assign29190_e28323_d_n10, assign29190_e28323_d_n11, assign29190_e28323_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29190_e28323;
        locals.var_xp_dn0 = assign29190_e28323_d_n0;
        locals.var_xp_dn2 = assign29190_e28323_d_n2;
        locals.var_xp_dn4 = assign29190_e28323_d_n4;
        locals.var_xp_dn5 = assign29190_e28323_d_n5;
        locals.var_xp_dn6 = assign29190_e28323_d_n6;
        locals.var_xp_dn7 = assign29190_e28323_d_n7;
        locals.var_xp_dn8 = assign29190_e28323_d_n8;
        locals.var_xp_dn9 = assign29190_e28323_d_n9;
        locals.var_xp_dn10 = assign29190_e28323_d_n10;
        locals.var_xp_dn11 = assign29190_e28323_d_n11;
        locals.var_xp_dn14 = assign29190_e28323_d_n14;

        let (assign29200_e28331, assign29200_e28331_d_n0, assign29200_e28331_d_n2, assign29200_e28331_d_n4, assign29200_e28331_d_n5, assign29200_e28331_d_n6, assign29200_e28331_d_n7, assign29200_e28331_d_n8, assign29200_e28331_d_n9, assign29200_e28331_d_n10, assign29200_e28331_d_n11, assign29200_e28331_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29200_e28331;
        locals.var_xmp_dn0 = assign29200_e28331_d_n0;
        locals.var_xmp_dn2 = assign29200_e28331_d_n2;
        locals.var_xmp_dn4 = assign29200_e28331_d_n4;
        locals.var_xmp_dn5 = assign29200_e28331_d_n5;
        locals.var_xmp_dn6 = assign29200_e28331_d_n6;
        locals.var_xmp_dn7 = assign29200_e28331_d_n7;
        locals.var_xmp_dn8 = assign29200_e28331_d_n8;
        locals.var_xmp_dn9 = assign29200_e28331_d_n9;
        locals.var_xmp_dn10 = assign29200_e28331_d_n10;
        locals.var_xmp_dn11 = assign29200_e28331_d_n11;
        locals.var_xmp_dn14 = assign29200_e28331_d_n14;

        let (assign29210_e28339,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29210_e28339;

    }

    pub(super) fn stamp_transient_block_84(
        locals: &mut StampLocals,
    ) {
        let (assign29220_e28347,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29220_e28347;

        let (assign29230_e28355, assign29230_e28355_d_n0, assign29230_e28355_d_n2, assign29230_e28355_d_n4, assign29230_e28355_d_n5, assign29230_e28355_d_n6, assign29230_e28355_d_n7, assign29230_e28355_d_n8, assign29230_e28355_d_n9, assign29230_e28355_d_n10, assign29230_e28355_d_n11, assign29230_e28355_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29230_e28355;
        locals.var_arg_dn0 = assign29230_e28355_d_n0;
        locals.var_arg_dn2 = assign29230_e28355_d_n2;
        locals.var_arg_dn4 = assign29230_e28355_d_n4;
        locals.var_arg_dn5 = assign29230_e28355_d_n5;
        locals.var_arg_dn6 = assign29230_e28355_d_n6;
        locals.var_arg_dn7 = assign29230_e28355_d_n7;
        locals.var_arg_dn8 = assign29230_e28355_d_n8;
        locals.var_arg_dn9 = assign29230_e28355_d_n9;
        locals.var_arg_dn10 = assign29230_e28355_d_n10;
        locals.var_arg_dn11 = assign29230_e28355_d_n11;
        locals.var_arg_dn14 = assign29230_e28355_d_n14;

        let (assign29240_e28363, assign29240_e28363_d_n0, assign29240_e28363_d_n2, assign29240_e28363_d_n4, assign29240_e28363_d_n5, assign29240_e28363_d_n6, assign29240_e28363_d_n7, assign29240_e28363_d_n8, assign29240_e28363_d_n9, assign29240_e28363_d_n10, assign29240_e28363_d_n11, assign29240_e28363_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29240_e28363;
        locals.var_dnm_dn0 = assign29240_e28363_d_n0;
        locals.var_dnm_dn2 = assign29240_e28363_d_n2;
        locals.var_dnm_dn4 = assign29240_e28363_d_n4;
        locals.var_dnm_dn5 = assign29240_e28363_d_n5;
        locals.var_dnm_dn6 = assign29240_e28363_d_n6;
        locals.var_dnm_dn7 = assign29240_e28363_d_n7;
        locals.var_dnm_dn8 = assign29240_e28363_d_n8;
        locals.var_dnm_dn9 = assign29240_e28363_d_n9;
        locals.var_dnm_dn10 = assign29240_e28363_d_n10;
        locals.var_dnm_dn11 = assign29240_e28363_d_n11;
        locals.var_dnm_dn14 = assign29240_e28363_d_n14;

        let (assign29250_e28373, assign29250_e28373_d_n0, assign29250_e28373_d_n2, assign29250_e28373_d_n4, assign29250_e28373_d_n5, assign29250_e28373_d_n6, assign29250_e28373_d_n7, assign29250_e28373_d_n8, assign29250_e28373_d_n9, assign29250_e28373_d_n10, assign29250_e28373_d_n11, assign29250_e28373_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29250_e28371: f64 = (locals.var_xp * locals.var_x2);
        (assign29250_e28371, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29250_e28373;
        locals.var_xp_dn0 = assign29250_e28373_d_n0;
        locals.var_xp_dn2 = assign29250_e28373_d_n2;
        locals.var_xp_dn4 = assign29250_e28373_d_n4;
        locals.var_xp_dn5 = assign29250_e28373_d_n5;
        locals.var_xp_dn6 = assign29250_e28373_d_n6;
        locals.var_xp_dn7 = assign29250_e28373_d_n7;
        locals.var_xp_dn8 = assign29250_e28373_d_n8;
        locals.var_xp_dn9 = assign29250_e28373_d_n9;
        locals.var_xp_dn10 = assign29250_e28373_d_n10;
        locals.var_xp_dn11 = assign29250_e28373_d_n11;
        locals.var_xp_dn14 = assign29250_e28373_d_n14;

        let (assign29260_e28383, assign29260_e28383_d_n0, assign29260_e28383_d_n2, assign29260_e28383_d_n4, assign29260_e28383_d_n5, assign29260_e28383_d_n6, assign29260_e28383_d_n7, assign29260_e28383_d_n8, assign29260_e28383_d_n9, assign29260_e28383_d_n10, assign29260_e28383_d_n11, assign29260_e28383_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29260_e28381: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29260_e28381, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29260_e28383;
        locals.var_xmp_dn0 = assign29260_e28383_d_n0;
        locals.var_xmp_dn2 = assign29260_e28383_d_n2;
        locals.var_xmp_dn4 = assign29260_e28383_d_n4;
        locals.var_xmp_dn5 = assign29260_e28383_d_n5;
        locals.var_xmp_dn6 = assign29260_e28383_d_n6;
        locals.var_xmp_dn7 = assign29260_e28383_d_n7;
        locals.var_xmp_dn8 = assign29260_e28383_d_n8;
        locals.var_xmp_dn9 = assign29260_e28383_d_n9;
        locals.var_xmp_dn10 = assign29260_e28383_d_n10;
        locals.var_xmp_dn11 = assign29260_e28383_d_n11;
        locals.var_xmp_dn14 = assign29260_e28383_d_n14;

        let (assign29270_e28393, assign29270_e28393_d_n0, assign29270_e28393_d_n2, assign29270_e28393_d_n4, assign29270_e28393_d_n5, assign29270_e28393_d_n6, assign29270_e28393_d_n7, assign29270_e28393_d_n8, assign29270_e28393_d_n9, assign29270_e28393_d_n10, assign29270_e28393_d_n11, assign29270_e28393_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29270_e28391: f64 = (locals.var_xp * locals.var_x2);
        (assign29270_e28391, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29270_e28393;
        locals.var_xp_dn0 = assign29270_e28393_d_n0;
        locals.var_xp_dn2 = assign29270_e28393_d_n2;
        locals.var_xp_dn4 = assign29270_e28393_d_n4;
        locals.var_xp_dn5 = assign29270_e28393_d_n5;
        locals.var_xp_dn6 = assign29270_e28393_d_n6;
        locals.var_xp_dn7 = assign29270_e28393_d_n7;
        locals.var_xp_dn8 = assign29270_e28393_d_n8;
        locals.var_xp_dn9 = assign29270_e28393_d_n9;
        locals.var_xp_dn10 = assign29270_e28393_d_n10;
        locals.var_xp_dn11 = assign29270_e28393_d_n11;
        locals.var_xp_dn14 = assign29270_e28393_d_n14;

        let (assign29280_e28403, assign29280_e28403_d_n0, assign29280_e28403_d_n2, assign29280_e28403_d_n4, assign29280_e28403_d_n5, assign29280_e28403_d_n6, assign29280_e28403_d_n7, assign29280_e28403_d_n8, assign29280_e28403_d_n9, assign29280_e28403_d_n10, assign29280_e28403_d_n11, assign29280_e28403_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29280_e28401: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29280_e28401, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29280_e28403;
        locals.var_xmp_dn0 = assign29280_e28403_d_n0;
        locals.var_xmp_dn2 = assign29280_e28403_d_n2;
        locals.var_xmp_dn4 = assign29280_e28403_d_n4;
        locals.var_xmp_dn5 = assign29280_e28403_d_n5;
        locals.var_xmp_dn6 = assign29280_e28403_d_n6;
        locals.var_xmp_dn7 = assign29280_e28403_d_n7;
        locals.var_xmp_dn8 = assign29280_e28403_d_n8;
        locals.var_xmp_dn9 = assign29280_e28403_d_n9;
        locals.var_xmp_dn10 = assign29280_e28403_d_n10;
        locals.var_xmp_dn11 = assign29280_e28403_d_n11;
        locals.var_xmp_dn14 = assign29280_e28403_d_n14;

        let (assign29290_e28413, assign29290_e28413_d_n0, assign29290_e28413_d_n2, assign29290_e28413_d_n4, assign29290_e28413_d_n5, assign29290_e28413_d_n6, assign29290_e28413_d_n7, assign29290_e28413_d_n8, assign29290_e28413_d_n9, assign29290_e28413_d_n10, assign29290_e28413_d_n11, assign29290_e28413_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29290_e28411: f64 = (locals.var_xp + locals.var_xmp);
        (assign29290_e28411, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29290_e28413;
        locals.var_arg_dn0 = assign29290_e28413_d_n0;
        locals.var_arg_dn2 = assign29290_e28413_d_n2;
        locals.var_arg_dn4 = assign29290_e28413_d_n4;
        locals.var_arg_dn5 = assign29290_e28413_d_n5;
        locals.var_arg_dn6 = assign29290_e28413_d_n6;
        locals.var_arg_dn7 = assign29290_e28413_d_n7;
        locals.var_arg_dn8 = assign29290_e28413_d_n8;
        locals.var_arg_dn9 = assign29290_e28413_d_n9;
        locals.var_arg_dn10 = assign29290_e28413_d_n10;
        locals.var_arg_dn11 = assign29290_e28413_d_n11;
        locals.var_arg_dn14 = assign29290_e28413_d_n14;

        let (assign29300_e28421, assign29300_e28421_d_n0, assign29300_e28421_d_n2, assign29300_e28421_d_n4, assign29300_e28421_d_n5, assign29300_e28421_d_n6, assign29300_e28421_d_n7, assign29300_e28421_d_n8, assign29300_e28421_d_n9, assign29300_e28421_d_n10, assign29300_e28421_d_n11, assign29300_e28421_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29300_e28421;
        locals.var_dnm_dn0 = assign29300_e28421_d_n0;
        locals.var_dnm_dn2 = assign29300_e28421_d_n2;
        locals.var_dnm_dn4 = assign29300_e28421_d_n4;
        locals.var_dnm_dn5 = assign29300_e28421_d_n5;
        locals.var_dnm_dn6 = assign29300_e28421_d_n6;
        locals.var_dnm_dn7 = assign29300_e28421_d_n7;
        locals.var_dnm_dn8 = assign29300_e28421_d_n8;
        locals.var_dnm_dn9 = assign29300_e28421_d_n9;
        locals.var_dnm_dn10 = assign29300_e28421_d_n10;
        locals.var_dnm_dn11 = assign29300_e28421_d_n11;
        locals.var_dnm_dn14 = assign29300_e28421_d_n14;

        let assign29310_e28436: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard683 = assign29310_e28436;

        let assign29320_e28439: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard684 = assign29320_e28439;

        let (assign29330_e28451,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29330_e28451;

        let assign29340_e28454: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard685 = assign29340_e28454;

        let (assign29350_e28469,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29350_e28469;

        let assign29360_e28472: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard686 = assign29360_e28472;

        let (assign29370_e28490,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29370_e28490;

        let assign29380_e28493: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard687 = assign29380_e28493;

        let (assign29390_e28514,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29390_e28514;

        let (assign29400_e28524,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29400_e28524;

        let mut assign29410_loop_guard: usize = 0;
        while {
            let assign29410_cond_e28535: f64 = if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29410_cond_e28535 != 0.0
        } {
            assign29410_loop_guard += 1;
            assert!(assign29410_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29410_body0_e28546, assign29410_body0_e28546_d_n0, assign29410_body0_e28546_d_n2, assign29410_body0_e28546_d_n4, assign29410_body0_e28546_d_n5, assign29410_body0_e28546_d_n6, assign29410_body0_e28546_d_n7, assign29410_body0_e28546_d_n8, assign29410_body0_e28546_d_n9, assign29410_body0_e28546_d_n10, assign29410_body0_e28546_d_n11, assign29410_body0_e28546_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) {
        let assign29410_body0_e28544: f64 = (locals.var_dnm).sqrt();
        (assign29410_body0_e28544, (locals.var_dnm_dn0 / (2.0 * assign29410_body0_e28544)), (locals.var_dnm_dn2 / (2.0 * assign29410_body0_e28544)), (locals.var_dnm_dn4 / (2.0 * assign29410_body0_e28544)), (locals.var_dnm_dn5 / (2.0 * assign29410_body0_e28544)), (locals.var_dnm_dn6 / (2.0 * assign29410_body0_e28544)), (locals.var_dnm_dn7 / (2.0 * assign29410_body0_e28544)), (locals.var_dnm_dn8 / (2.0 * assign29410_body0_e28544)), (locals.var_dnm_dn9 / (2.0 * assign29410_body0_e28544)), (locals.var_dnm_dn10 / (2.0 * assign29410_body0_e28544)), (locals.var_dnm_dn11 / (2.0 * assign29410_body0_e28544)), (locals.var_dnm_dn14 / (2.0 * assign29410_body0_e28544)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign29410_body0_e28546;
            locals.var_dnm_dn0 = assign29410_body0_e28546_d_n0;
            locals.var_dnm_dn2 = assign29410_body0_e28546_d_n2;
            locals.var_dnm_dn4 = assign29410_body0_e28546_d_n4;
            locals.var_dnm_dn5 = assign29410_body0_e28546_d_n5;
            locals.var_dnm_dn6 = assign29410_body0_e28546_d_n6;
            locals.var_dnm_dn7 = assign29410_body0_e28546_d_n7;
            locals.var_dnm_dn8 = assign29410_body0_e28546_d_n8;
            locals.var_dnm_dn9 = assign29410_body0_e28546_d_n9;
            locals.var_dnm_dn10 = assign29410_body0_e28546_d_n10;
            locals.var_dnm_dn11 = assign29410_body0_e28546_d_n11;
            locals.var_dnm_dn14 = assign29410_body0_e28546_d_n14;
            let (assign29410_body1_e28558,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) {
        let assign29410_body1_e28556: f64 = (locals.var_m0 + 1.0);
        (assign29410_body1_e28556,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29410_body1_e28558;
        }

        let (assign29420_e28580, assign29420_e28580_d_n0, assign29420_e28580_d_n2, assign29420_e28580_d_n4, assign29420_e28580_d_n5, assign29420_e28580_d_n6, assign29420_e28580_d_n7, assign29420_e28580_d_n8, assign29420_e28580_d_n9, assign29420_e28580_d_n10, assign29420_e28580_d_n11, assign29420_e28580_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 == 0.0)) {
        let (assign29420_e28578, assign29420_e28578_d_n0, assign29420_e28578_d_n2, assign29420_e28578_d_n4, assign29420_e28578_d_n5, assign29420_e28578_d_n6, assign29420_e28578_d_n7, assign29420_e28578_d_n8, assign29420_e28578_d_n9, assign29420_e28578_d_n10, assign29420_e28578_d_n11, assign29420_e28578_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29420_e28575: f64 = (2.0 * 2.0);
                let assign29420_e28576: f64 = (1.0 / assign29420_e28575);
                let assign29420_e28577: f64 = (locals.var_dnm).powf(assign29420_e28576);
                (assign29420_e28577, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn11)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29420_e28576) as f64).is_finite() && ((assign29420_e28576) as f64).fract() == 0.0 { if assign29420_e28576 == 0.0 { 0.0 } else { (assign29420_e28576 * ((locals.var_dnm).powf(assign29420_e28576 - 1.0) * locals.var_dnm_dn14)) } } else { (assign29420_e28577 * (assign29420_e28576 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign29420_e28578, assign29420_e28578_d_n0, assign29420_e28578_d_n2, assign29420_e28578_d_n4, assign29420_e28578_d_n5, assign29420_e28578_d_n6, assign29420_e28578_d_n7, assign29420_e28578_d_n8, assign29420_e28578_d_n9, assign29420_e28578_d_n10, assign29420_e28578_d_n11, assign29420_e28578_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29420_e28580;
        locals.var_dnm_dn0 = assign29420_e28580_d_n0;
        locals.var_dnm_dn2 = assign29420_e28580_d_n2;
        locals.var_dnm_dn4 = assign29420_e28580_d_n4;
        locals.var_dnm_dn5 = assign29420_e28580_d_n5;
        locals.var_dnm_dn6 = assign29420_e28580_d_n6;
        locals.var_dnm_dn7 = assign29420_e28580_d_n7;
        locals.var_dnm_dn8 = assign29420_e28580_d_n8;
        locals.var_dnm_dn9 = assign29420_e28580_d_n9;
        locals.var_dnm_dn10 = assign29420_e28580_d_n10;
        locals.var_dnm_dn11 = assign29420_e28580_d_n11;
        locals.var_dnm_dn14 = assign29420_e28580_d_n14;

        let (assign29430_e28590, assign29430_e28590_d_n0, assign29430_e28590_d_n2, assign29430_e28590_d_n4, assign29430_e28590_d_n5, assign29430_e28590_d_n6, assign29430_e28590_d_n7, assign29430_e28590_d_n8, assign29430_e28590_d_n9, assign29430_e28590_d_n10, assign29430_e28590_d_n11, assign29430_e28590_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29430_e28588: f64 = (1.0 / locals.var_dnm);
        (assign29430_e28588, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29430_e28590;
        locals.var_dnm_dn0 = assign29430_e28590_d_n0;
        locals.var_dnm_dn2 = assign29430_e28590_d_n2;
        locals.var_dnm_dn4 = assign29430_e28590_d_n4;
        locals.var_dnm_dn5 = assign29430_e28590_d_n5;
        locals.var_dnm_dn6 = assign29430_e28590_d_n6;
        locals.var_dnm_dn7 = assign29430_e28590_d_n7;
        locals.var_dnm_dn8 = assign29430_e28590_d_n8;
        locals.var_dnm_dn9 = assign29430_e28590_d_n9;
        locals.var_dnm_dn10 = assign29430_e28590_d_n10;
        locals.var_dnm_dn11 = assign29430_e28590_d_n11;
        locals.var_dnm_dn14 = assign29430_e28590_d_n14;

        let (assign29440_e28602, assign29440_e28602_d_n0, assign29440_e28602_d_n2, assign29440_e28602_d_n4, assign29440_e28602_d_n5, assign29440_e28602_d_n6, assign29440_e28602_d_n7, assign29440_e28602_d_n8, assign29440_e28602_d_n9, assign29440_e28602_d_n10, assign29440_e28602_d_n11, assign29440_e28602_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29440_e28598: f64 = (locals.var_tmf1 * 0.06);
        let assign29440_e28600: f64 = (assign29440_e28598 * locals.var_dnm);
        (assign29440_e28600, (((locals.var_tmf1_dn0 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.06) * locals.var_dnm) + (assign29440_e28598 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign29440_e28602;
        locals.var_tmf0_dn0 = assign29440_e28602_d_n0;
        locals.var_tmf0_dn2 = assign29440_e28602_d_n2;
        locals.var_tmf0_dn4 = assign29440_e28602_d_n4;
        locals.var_tmf0_dn5 = assign29440_e28602_d_n5;
        locals.var_tmf0_dn6 = assign29440_e28602_d_n6;
        locals.var_tmf0_dn7 = assign29440_e28602_d_n7;
        locals.var_tmf0_dn8 = assign29440_e28602_d_n8;
        locals.var_tmf0_dn9 = assign29440_e28602_d_n9;
        locals.var_tmf0_dn10 = assign29440_e28602_d_n10;
        locals.var_tmf0_dn11 = assign29440_e28602_d_n11;
        locals.var_tmf0_dn14 = assign29440_e28602_d_n14;

        let (assign29450_e28616, assign29450_e28616_d_n0, assign29450_e28616_d_n2, assign29450_e28616_d_n4, assign29450_e28616_d_n5, assign29450_e28616_d_n6, assign29450_e28616_d_n7, assign29450_e28616_d_n8, assign29450_e28616_d_n9, assign29450_e28616_d_n10, assign29450_e28616_d_n11, assign29450_e28616_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29450_e28610: f64 = (0.06 * locals.var_xmp);
        let assign29450_e28612: f64 = (assign29450_e28610 * locals.var_dnm);
        let assign29450_e28614: f64 = (assign29450_e28612 / locals.var_arg);
        (assign29450_e28614, ((((((0.06 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn11) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn11)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn14) * locals.var_dnm) + (assign29450_e28610 * locals.var_dnm_dn14)) * locals.var_arg) - (assign29450_e28612 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29450_e28616;
        locals.var_t0_dn0 = assign29450_e28616_d_n0;
        locals.var_t0_dn2 = assign29450_e28616_d_n2;
        locals.var_t0_dn4 = assign29450_e28616_d_n4;
        locals.var_t0_dn5 = assign29450_e28616_d_n5;
        locals.var_t0_dn6 = assign29450_e28616_d_n6;
        locals.var_t0_dn7 = assign29450_e28616_d_n7;
        locals.var_t0_dn8 = assign29450_e28616_d_n8;
        locals.var_t0_dn9 = assign29450_e28616_d_n9;
        locals.var_t0_dn10 = assign29450_e28616_d_n10;
        locals.var_t0_dn11 = assign29450_e28616_d_n11;
        locals.var_t0_dn14 = assign29450_e28616_d_n14;

        let (assign29460_e28628, assign29460_e28628_d_n0, assign29460_e28628_d_n2, assign29460_e28628_d_n4, assign29460_e28628_d_n5, assign29460_e28628_d_n6, assign29460_e28628_d_n7, assign29460_e28628_d_n8, assign29460_e28628_d_n9, assign29460_e28628_d_n10, assign29460_e28628_d_n11, assign29460_e28628_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        let assign29460_e28624: f64 = 0.06;
        let assign29460_e28626: f64 = (assign29460_e28624 - locals.var_tmf0);
        (assign29460_e28626, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29460_e28628;
        locals.var_t2_dn0 = assign29460_e28628_d_n0;
        locals.var_t2_dn2 = assign29460_e28628_d_n2;
        locals.var_t2_dn4 = assign29460_e28628_d_n4;
        locals.var_t2_dn5 = assign29460_e28628_d_n5;
        locals.var_t2_dn6 = assign29460_e28628_d_n6;
        locals.var_t2_dn7 = assign29460_e28628_d_n7;
        locals.var_t2_dn8 = assign29460_e28628_d_n8;
        locals.var_t2_dn9 = assign29460_e28628_d_n9;
        locals.var_t2_dn10 = assign29460_e28628_d_n10;
        locals.var_t2_dn11 = assign29460_e28628_d_n11;
        locals.var_t2_dn14 = assign29460_e28628_d_n14;

        let (assign29470_e28636, assign29470_e28636_d_n0, assign29470_e28636_d_n2, assign29470_e28636_d_n4, assign29470_e28636_d_n5, assign29470_e28636_d_n6, assign29470_e28636_d_n7, assign29470_e28636_d_n8, assign29470_e28636_d_n9, assign29470_e28636_d_n10, assign29470_e28636_d_n11, assign29470_e28636_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29470_e28636;
        locals.var_t0_dn0 = assign29470_e28636_d_n0;
        locals.var_t0_dn2 = assign29470_e28636_d_n2;
        locals.var_t0_dn4 = assign29470_e28636_d_n4;
        locals.var_t0_dn5 = assign29470_e28636_d_n5;
        locals.var_t0_dn6 = assign29470_e28636_d_n6;
        locals.var_t0_dn7 = assign29470_e28636_d_n7;
        locals.var_t0_dn8 = assign29470_e28636_d_n8;
        locals.var_t0_dn9 = assign29470_e28636_d_n9;
        locals.var_t0_dn10 = assign29470_e28636_d_n10;
        locals.var_t0_dn11 = assign29470_e28636_d_n11;
        locals.var_t0_dn14 = assign29470_e28636_d_n14;

        let (assign29480_e28647, assign29480_e28647_d_n0, assign29480_e28647_d_n2, assign29480_e28647_d_n4, assign29480_e28647_d_n5, assign29480_e28647_d_n6, assign29480_e28647_d_n7, assign29480_e28647_d_n8, assign29480_e28647_d_n9, assign29480_e28647_d_n10, assign29480_e28647_d_n11, assign29480_e28647_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 == 0.0)) {
        let assign29480_e28645: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        (assign29480_e28645, (locals.var_phi_s0_dep_dn0 - locals.var_vds_maxb0_dn0), (locals.var_phi_s0_dep_dn2 - locals.var_vds_maxb0_dn2), (locals.var_phi_s0_dep_dn4 - locals.var_vds_maxb0_dn4), (locals.var_phi_s0_dep_dn5 - locals.var_vds_maxb0_dn5), (locals.var_phi_s0_dep_dn6 - locals.var_vds_maxb0_dn6), (locals.var_phi_s0_dep_dn7 - locals.var_vds_maxb0_dn7), (locals.var_phi_s0_dep_dn8 - locals.var_vds_maxb0_dn8), (locals.var_phi_s0_dep_dn9 - locals.var_vds_maxb0_dn9), (locals.var_phi_s0_dep_dn10 - locals.var_vds_maxb0_dn10), (locals.var_phi_s0_dep_dn11 - locals.var_vds_maxb0_dn11), (locals.var_phi_s0_dep_dn14 - locals.var_vds_maxb0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29480_e28647;
        locals.var_t2_dn0 = assign29480_e28647_d_n0;
        locals.var_t2_dn2 = assign29480_e28647_d_n2;
        locals.var_t2_dn4 = assign29480_e28647_d_n4;
        locals.var_t2_dn5 = assign29480_e28647_d_n5;
        locals.var_t2_dn6 = assign29480_e28647_d_n6;
        locals.var_t2_dn7 = assign29480_e28647_d_n7;
        locals.var_t2_dn8 = assign29480_e28647_d_n8;
        locals.var_t2_dn9 = assign29480_e28647_d_n9;
        locals.var_t2_dn10 = assign29480_e28647_d_n10;
        locals.var_t2_dn11 = assign29480_e28647_d_n11;
        locals.var_t2_dn14 = assign29480_e28647_d_n14;

        let (assign29490_e28656, assign29490_e28656_d_n0, assign29490_e28656_d_n2, assign29490_e28656_d_n4, assign29490_e28656_d_n5, assign29490_e28656_d_n6, assign29490_e28656_d_n7, assign29490_e28656_d_n8, assign29490_e28656_d_n9, assign29490_e28656_d_n10, assign29490_e28656_d_n11, assign29490_e28656_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29490_e28656;
        locals.var_t0_dn0 = assign29490_e28656_d_n0;
        locals.var_t0_dn2 = assign29490_e28656_d_n2;
        locals.var_t0_dn4 = assign29490_e28656_d_n4;
        locals.var_t0_dn5 = assign29490_e28656_d_n5;
        locals.var_t0_dn6 = assign29490_e28656_d_n6;
        locals.var_t0_dn7 = assign29490_e28656_d_n7;
        locals.var_t0_dn8 = assign29490_e28656_d_n8;
        locals.var_t0_dn9 = assign29490_e28656_d_n9;
        locals.var_t0_dn10 = assign29490_e28656_d_n10;
        locals.var_t0_dn11 = assign29490_e28656_d_n11;
        locals.var_t0_dn14 = assign29490_e28656_d_n14;

        let (assign29500_e28675, assign29500_e28675_d_n0, assign29500_e28675_d_n2, assign29500_e28675_d_n4, assign29500_e28675_d_n5, assign29500_e28675_d_n6, assign29500_e28675_d_n7, assign29500_e28675_d_n8, assign29500_e28675_d_n9, assign29500_e28675_d_n10, assign29500_e28675_d_n11, assign29500_e28675_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign29500_e28662: f64 = (locals.var_beta * locals.var_t2);
        let assign29500_e28663: f64 = (assign29500_e28662).exp();
        let assign29500_e28665: f64 = (assign29500_e28663 - 1.0);
        let assign29500_e28668: f64 = (locals.var_beta * locals.var_t2);
        let assign29500_e28669: f64 = (assign29500_e28665 - assign29500_e28668);
        let assign29500_e28672: f64 = (10.0 * 2.220446049250313e-16);
        let assign29500_e28673: f64 = (assign29500_e28669 + assign29500_e28672);
        (assign29500_e28673, ((assign29500_e28663 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign29500_e28663 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign29500_e28663 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign29500_e28663 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign29500_e28663 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign29500_e28663 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign29500_e28663 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign29500_e28663 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign29500_e28663 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign29500_e28663 * ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))) - ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))), ((assign29500_e28663 * ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))) - ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29500_e28675;
        locals.var_t4_dn0 = assign29500_e28675_d_n0;
        locals.var_t4_dn2 = assign29500_e28675_d_n2;
        locals.var_t4_dn4 = assign29500_e28675_d_n4;
        locals.var_t4_dn5 = assign29500_e28675_d_n5;
        locals.var_t4_dn6 = assign29500_e28675_d_n6;
        locals.var_t4_dn7 = assign29500_e28675_d_n7;
        locals.var_t4_dn8 = assign29500_e28675_d_n8;
        locals.var_t4_dn9 = assign29500_e28675_d_n9;
        locals.var_t4_dn10 = assign29500_e28675_d_n10;
        locals.var_t4_dn11 = assign29500_e28675_d_n11;
        locals.var_t4_dn14 = assign29500_e28675_d_n14;

        let (assign29510_e28685, assign29510_e28685_d_n0, assign29510_e28685_d_n2, assign29510_e28685_d_n4, assign29510_e28685_d_n5, assign29510_e28685_d_n6, assign29510_e28685_d_n7, assign29510_e28685_d_n8, assign29510_e28685_d_n9, assign29510_e28685_d_n10, assign29510_e28685_d_n11, assign29510_e28685_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign29510_e28680: f64 = (-locals.var_cnst0);
        let assign29510_e28682: f64 = (locals.var_t4).sqrt();
        let assign29510_e28683: f64 = (assign29510_e28680 * assign29510_e28682);
        (assign29510_e28683, (((-locals.var_cnst0_dn0) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn0 / (2.0 * assign29510_e28682)))), (((-locals.var_cnst0_dn2) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn2 / (2.0 * assign29510_e28682)))), (((-locals.var_cnst0_dn4) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn4 / (2.0 * assign29510_e28682)))), (((-locals.var_cnst0_dn5) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn5 / (2.0 * assign29510_e28682)))), (((-locals.var_cnst0_dn6) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn6 / (2.0 * assign29510_e28682)))), (((-locals.var_cnst0_dn7) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn7 / (2.0 * assign29510_e28682)))), (((-locals.var_cnst0_dn8) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn8 / (2.0 * assign29510_e28682)))), (((-locals.var_cnst0_dn9) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn9 / (2.0 * assign29510_e28682)))), (((-locals.var_cnst0_dn10) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn10 / (2.0 * assign29510_e28682)))), (((-locals.var_cnst0_dn11) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn11 / (2.0 * assign29510_e28682)))), (((-locals.var_cnst0_dn14) * assign29510_e28682) + (assign29510_e28680 * (locals.var_t4_dn14 / (2.0 * assign29510_e28682)))),)
    } else {
        (locals.var_q_n0_cur, locals.var_q_n0_cur_dn0, locals.var_q_n0_cur_dn2, locals.var_q_n0_cur_dn4, locals.var_q_n0_cur_dn5, locals.var_q_n0_cur_dn6, locals.var_q_n0_cur_dn7, locals.var_q_n0_cur_dn8, locals.var_q_n0_cur_dn9, locals.var_q_n0_cur_dn10, locals.var_q_n0_cur_dn11, locals.var_q_n0_cur_dn14,)
    }
};
        locals.var_q_n0_cur = assign29510_e28685;
        locals.var_q_n0_cur_dn0 = assign29510_e28685_d_n0;
        locals.var_q_n0_cur_dn2 = assign29510_e28685_d_n2;
        locals.var_q_n0_cur_dn4 = assign29510_e28685_d_n4;
        locals.var_q_n0_cur_dn5 = assign29510_e28685_d_n5;
        locals.var_q_n0_cur_dn6 = assign29510_e28685_d_n6;
        locals.var_q_n0_cur_dn7 = assign29510_e28685_d_n7;
        locals.var_q_n0_cur_dn8 = assign29510_e28685_d_n8;
        locals.var_q_n0_cur_dn9 = assign29510_e28685_d_n9;
        locals.var_q_n0_cur_dn10 = assign29510_e28685_d_n10;
        locals.var_q_n0_cur_dn11 = assign29510_e28685_d_n11;
        locals.var_q_n0_cur_dn14 = assign29510_e28685_d_n14;

    }

    pub(super) fn stamp_transient_block_85(
        locals: &mut StampLocals,
    ) {
        let (assign29520_e28700, assign29520_e28700_d_n0, assign29520_e28700_d_n2, assign29520_e28700_d_n4, assign29520_e28700_d_n5, assign29520_e28700_d_n6, assign29520_e28700_d_n7, assign29520_e28700_d_n8, assign29520_e28700_d_n9, assign29520_e28700_d_n10, assign29520_e28700_d_n11, assign29520_e28700_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign29520_e28691: f64 = (locals.var_beta * 0.1);
        let assign29520_e28692: f64 = (assign29520_e28691).exp();
        let assign29520_e28694: f64 = (assign29520_e28692 - 1.0);
        let assign29520_e28697: f64 = (locals.var_beta * 0.1);
        let assign29520_e28698: f64 = (assign29520_e28694 - assign29520_e28697);
        (assign29520_e28698, ((assign29520_e28692 * (locals.var_beta_dn0 * 0.1)) - (locals.var_beta_dn0 * 0.1)), ((assign29520_e28692 * (locals.var_beta_dn2 * 0.1)) - (locals.var_beta_dn2 * 0.1)), ((assign29520_e28692 * (locals.var_beta_dn4 * 0.1)) - (locals.var_beta_dn4 * 0.1)), ((assign29520_e28692 * (locals.var_beta_dn5 * 0.1)) - (locals.var_beta_dn5 * 0.1)), ((assign29520_e28692 * (locals.var_beta_dn6 * 0.1)) - (locals.var_beta_dn6 * 0.1)), ((assign29520_e28692 * (locals.var_beta_dn7 * 0.1)) - (locals.var_beta_dn7 * 0.1)), ((assign29520_e28692 * (locals.var_beta_dn8 * 0.1)) - (locals.var_beta_dn8 * 0.1)), ((assign29520_e28692 * (locals.var_beta_dn9 * 0.1)) - (locals.var_beta_dn9 * 0.1)), ((assign29520_e28692 * (locals.var_beta_dn10 * 0.1)) - (locals.var_beta_dn10 * 0.1)), ((assign29520_e28692 * (locals.var_beta_dn11 * 0.1)) - (locals.var_beta_dn11 * 0.1)), ((assign29520_e28692 * (locals.var_beta_dn14 * 0.1)) - (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29520_e28700;
        locals.var_t4_dn0 = assign29520_e28700_d_n0;
        locals.var_t4_dn2 = assign29520_e28700_d_n2;
        locals.var_t4_dn4 = assign29520_e28700_d_n4;
        locals.var_t4_dn5 = assign29520_e28700_d_n5;
        locals.var_t4_dn6 = assign29520_e28700_d_n6;
        locals.var_t4_dn7 = assign29520_e28700_d_n7;
        locals.var_t4_dn8 = assign29520_e28700_d_n8;
        locals.var_t4_dn9 = assign29520_e28700_d_n9;
        locals.var_t4_dn10 = assign29520_e28700_d_n10;
        locals.var_t4_dn11 = assign29520_e28700_d_n11;
        locals.var_t4_dn14 = assign29520_e28700_d_n14;

        let (assign29530_e28709, assign29530_e28709_d_n0, assign29530_e28709_d_n2, assign29530_e28709_d_n4, assign29530_e28709_d_n5, assign29530_e28709_d_n6, assign29530_e28709_d_n7, assign29530_e28709_d_n8, assign29530_e28709_d_n9, assign29530_e28709_d_n10, assign29530_e28709_d_n11, assign29530_e28709_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign29530_e28706: f64 = (locals.var_t4).sqrt();
        let assign29530_e28707: f64 = (locals.var_cnst0 * assign29530_e28706);
        (assign29530_e28707, ((locals.var_cnst0_dn0 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn0 / (2.0 * assign29530_e28706)))), ((locals.var_cnst0_dn2 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn2 / (2.0 * assign29530_e28706)))), ((locals.var_cnst0_dn4 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn4 / (2.0 * assign29530_e28706)))), ((locals.var_cnst0_dn5 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn5 / (2.0 * assign29530_e28706)))), ((locals.var_cnst0_dn6 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn6 / (2.0 * assign29530_e28706)))), ((locals.var_cnst0_dn7 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn7 / (2.0 * assign29530_e28706)))), ((locals.var_cnst0_dn8 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn8 / (2.0 * assign29530_e28706)))), ((locals.var_cnst0_dn9 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn9 / (2.0 * assign29530_e28706)))), ((locals.var_cnst0_dn10 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn10 / (2.0 * assign29530_e28706)))), ((locals.var_cnst0_dn11 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn11 / (2.0 * assign29530_e28706)))), ((locals.var_cnst0_dn14 * assign29530_e28706) + (locals.var_cnst0 * (locals.var_t4_dn14 / (2.0 * assign29530_e28706)))),)
    } else {
        (locals.var_qn_delta, locals.var_qn_delta_dn0, locals.var_qn_delta_dn2, locals.var_qn_delta_dn4, locals.var_qn_delta_dn5, locals.var_qn_delta_dn6, locals.var_qn_delta_dn7, locals.var_qn_delta_dn8, locals.var_qn_delta_dn9, locals.var_qn_delta_dn10, locals.var_qn_delta_dn11, locals.var_qn_delta_dn14,)
    }
};
        locals.var_qn_delta = assign29530_e28709;
        locals.var_qn_delta_dn0 = assign29530_e28709_d_n0;
        locals.var_qn_delta_dn2 = assign29530_e28709_d_n2;
        locals.var_qn_delta_dn4 = assign29530_e28709_d_n4;
        locals.var_qn_delta_dn5 = assign29530_e28709_d_n5;
        locals.var_qn_delta_dn6 = assign29530_e28709_d_n6;
        locals.var_qn_delta_dn7 = assign29530_e28709_d_n7;
        locals.var_qn_delta_dn8 = assign29530_e28709_d_n8;
        locals.var_qn_delta_dn9 = assign29530_e28709_d_n9;
        locals.var_qn_delta_dn10 = assign29530_e28709_d_n10;
        locals.var_qn_delta_dn11 = assign29530_e28709_d_n11;
        locals.var_qn_delta_dn14 = assign29530_e28709_d_n14;

        let (assign29540_e28715, assign29540_e28715_d_n0, assign29540_e28715_d_n2, assign29540_e28715_d_n4, assign29540_e28715_d_n5, assign29540_e28715_d_n6, assign29540_e28715_d_n7, assign29540_e28715_d_n8, assign29540_e28715_d_n9, assign29540_e28715_d_n10, assign29540_e28715_d_n11, assign29540_e28715_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    }
};
        locals.var_vdsorg = assign29540_e28715;
        locals.var_vdsorg_dn0 = assign29540_e28715_d_n0;
        locals.var_vdsorg_dn2 = assign29540_e28715_d_n2;
        locals.var_vdsorg_dn4 = assign29540_e28715_d_n4;
        locals.var_vdsorg_dn5 = assign29540_e28715_d_n5;
        locals.var_vdsorg_dn6 = assign29540_e28715_d_n6;
        locals.var_vdsorg_dn7 = assign29540_e28715_d_n7;
        locals.var_vdsorg_dn8 = assign29540_e28715_d_n8;
        locals.var_vdsorg_dn9 = assign29540_e28715_d_n9;
        locals.var_vdsorg_dn10 = assign29540_e28715_d_n10;
        locals.var_vdsorg_dn11 = assign29540_e28715_d_n11;
        locals.var_vdsorg_dn14 = assign29540_e28715_d_n14;

        let assign29550_e28718: f64 = if locals.var_vds > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard688 = assign29550_e28718;

        let (assign29560_e28730, assign29560_e28730_d_n0, assign29560_e28730_d_n2, assign29560_e28730_d_n4, assign29560_e28730_d_n5, assign29560_e28730_d_n6, assign29560_e28730_d_n7, assign29560_e28730_d_n8, assign29560_e28730_d_n9, assign29560_e28730_d_n10, assign29560_e28730_d_n11, assign29560_e28730_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign29560_e28727: f64 = (locals.var_cox * locals.var_cox);
        let assign29560_e28728: f64 = (locals.var_q_ndepm_esi / assign29560_e28727);
        (assign29560_e28728, (((locals.var_q_ndepm_esi_dn0 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign29560_e28727 * assign29560_e28727)), (((locals.var_q_ndepm_esi_dn2 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign29560_e28727 * assign29560_e28727)), (((locals.var_q_ndepm_esi_dn4 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign29560_e28727 * assign29560_e28727)), (((locals.var_q_ndepm_esi_dn5 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign29560_e28727 * assign29560_e28727)), (((locals.var_q_ndepm_esi_dn6 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign29560_e28727 * assign29560_e28727)), (((locals.var_q_ndepm_esi_dn7 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign29560_e28727 * assign29560_e28727)), (((locals.var_q_ndepm_esi_dn8 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign29560_e28727 * assign29560_e28727)), (((locals.var_q_ndepm_esi_dn9 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign29560_e28727 * assign29560_e28727)), (((locals.var_q_ndepm_esi_dn10 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign29560_e28727 * assign29560_e28727)), (((locals.var_q_ndepm_esi_dn11 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn11 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn11)))) / (assign29560_e28727 * assign29560_e28727)), (((locals.var_q_ndepm_esi_dn14 * assign29560_e28727) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn14 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn14)))) / (assign29560_e28727 * assign29560_e28727)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign29560_e28730;
        locals.var_t2_dn0 = assign29560_e28730_d_n0;
        locals.var_t2_dn2 = assign29560_e28730_d_n2;
        locals.var_t2_dn4 = assign29560_e28730_d_n4;
        locals.var_t2_dn5 = assign29560_e28730_d_n5;
        locals.var_t2_dn6 = assign29560_e28730_d_n6;
        locals.var_t2_dn7 = assign29560_e28730_d_n7;
        locals.var_t2_dn8 = assign29560_e28730_d_n8;
        locals.var_t2_dn9 = assign29560_e28730_d_n9;
        locals.var_t2_dn10 = assign29560_e28730_d_n10;
        locals.var_t2_dn11 = assign29560_e28730_d_n11;
        locals.var_t2_dn14 = assign29560_e28730_d_n14;

        let (assign29570_e28744, assign29570_e28744_d_n0, assign29570_e28744_d_n2, assign29570_e28744_d_n4, assign29570_e28744_d_n5, assign29570_e28744_d_n6, assign29570_e28744_d_n7, assign29570_e28744_d_n8, assign29570_e28744_d_n9, assign29570_e28744_d_n10, assign29570_e28744_d_n11, assign29570_e28744_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign29570_e28738: f64 = (locals.var_vgp + 2.0);
        let assign29570_e28740: f64 = (assign29570_e28738 - locals.var_beta_inv);
        let assign29570_e28742: f64 = (assign29570_e28740 - locals.var_vbsz__blk440);
        (assign29570_e28742, ((locals.var_vgp_dn0 - locals.var_beta_inv_dn0) - locals.var_vbsz__blk440_dn0), ((locals.var_vgp_dn2 - locals.var_beta_inv_dn2) - locals.var_vbsz__blk440_dn2), ((locals.var_vgp_dn4 - locals.var_beta_inv_dn4) - locals.var_vbsz__blk440_dn4), ((locals.var_vgp_dn5 - locals.var_beta_inv_dn5) - locals.var_vbsz__blk440_dn5), ((locals.var_vgp_dn6 - locals.var_beta_inv_dn6) - locals.var_vbsz__blk440_dn6), ((locals.var_vgp_dn7 - locals.var_beta_inv_dn7) - locals.var_vbsz__blk440_dn7), ((locals.var_vgp_dn8 - locals.var_beta_inv_dn8) - locals.var_vbsz__blk440_dn8), ((locals.var_vgp_dn9 - locals.var_beta_inv_dn9) - locals.var_vbsz__blk440_dn9), ((locals.var_vgp_dn10 - locals.var_beta_inv_dn10) - locals.var_vbsz__blk440_dn10), ((locals.var_vgp_dn11 - locals.var_beta_inv_dn11) - locals.var_vbsz__blk440_dn11), ((locals.var_vgp_dn14 - locals.var_beta_inv_dn14) - locals.var_vbsz__blk440_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29570_e28744;
        locals.var_t0_dn0 = assign29570_e28744_d_n0;
        locals.var_t0_dn2 = assign29570_e28744_d_n2;
        locals.var_t0_dn4 = assign29570_e28744_d_n4;
        locals.var_t0_dn5 = assign29570_e28744_d_n5;
        locals.var_t0_dn6 = assign29570_e28744_d_n6;
        locals.var_t0_dn7 = assign29570_e28744_d_n7;
        locals.var_t0_dn8 = assign29570_e28744_d_n8;
        locals.var_t0_dn9 = assign29570_e28744_d_n9;
        locals.var_t0_dn10 = assign29570_e28744_d_n10;
        locals.var_t0_dn11 = assign29570_e28744_d_n11;
        locals.var_t0_dn14 = assign29570_e28744_d_n14;

        let (assign29580_e28758, assign29580_e28758_d_n0, assign29580_e28758_d_n2, assign29580_e28758_d_n4, assign29580_e28758_d_n5, assign29580_e28758_d_n6, assign29580_e28758_d_n7, assign29580_e28758_d_n8, assign29580_e28758_d_n9, assign29580_e28758_d_n10, assign29580_e28758_d_n11, assign29580_e28758_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign29580_e28753: f64 = (2.0 / locals.var_t2);
        let assign29580_e28755: f64 = (assign29580_e28753 * locals.var_t0);
        let assign29580_e28756: f64 = (1.0 + assign29580_e28755);
        (assign29580_e28756, (((-((2.0 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn0)), (((-((2.0 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn2)), (((-((2.0 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn4)), (((-((2.0 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn5)), (((-((2.0 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn6)), (((-((2.0 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn7)), (((-((2.0 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn8)), (((-((2.0 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn9)), (((-((2.0 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn10)), (((-((2.0 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn11)), (((-((2.0 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign29580_e28753 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29580_e28758;
        locals.var_t4_dn0 = assign29580_e28758_d_n0;
        locals.var_t4_dn2 = assign29580_e28758_d_n2;
        locals.var_t4_dn4 = assign29580_e28758_d_n4;
        locals.var_t4_dn5 = assign29580_e28758_d_n5;
        locals.var_t4_dn6 = assign29580_e28758_d_n6;
        locals.var_t4_dn7 = assign29580_e28758_d_n7;
        locals.var_t4_dn8 = assign29580_e28758_d_n8;
        locals.var_t4_dn9 = assign29580_e28758_d_n9;
        locals.var_t4_dn10 = assign29580_e28758_d_n10;
        locals.var_t4_dn11 = assign29580_e28758_d_n11;
        locals.var_t4_dn14 = assign29580_e28758_d_n14;

        let assign29590_e28762: f64 = 2.0;
        let assign29590_e28767: f64 = if ((locals.var_t4 < assign29590_e28762) && (2.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard689 = assign29590_e28767;

        let (assign29600_e28781, assign29600_e28781_d_n0, assign29600_e28781_d_n2, assign29600_e28781_d_n4, assign29600_e28781_d_n5, assign29600_e28781_d_n6, assign29600_e28781_d_n7, assign29600_e28781_d_n8, assign29600_e28781_d_n9, assign29600_e28781_d_n10, assign29600_e28781_d_n11, assign29600_e28781_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29600_e28777: f64 = 2.0;
        let assign29600_e28779: f64 = (assign29600_e28777 - locals.var_t4);
        (assign29600_e28779, (-locals.var_t4_dn0), (-locals.var_t4_dn2), (-locals.var_t4_dn4), (-locals.var_t4_dn5), (-locals.var_t4_dn6), (-locals.var_t4_dn7), (-locals.var_t4_dn8), (-locals.var_t4_dn9), (-locals.var_t4_dn10), (-locals.var_t4_dn11), (-locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign29600_e28781;
        locals.var_tmf1_dn0 = assign29600_e28781_d_n0;
        locals.var_tmf1_dn2 = assign29600_e28781_d_n2;
        locals.var_tmf1_dn4 = assign29600_e28781_d_n4;
        locals.var_tmf1_dn5 = assign29600_e28781_d_n5;
        locals.var_tmf1_dn6 = assign29600_e28781_d_n6;
        locals.var_tmf1_dn7 = assign29600_e28781_d_n7;
        locals.var_tmf1_dn8 = assign29600_e28781_d_n8;
        locals.var_tmf1_dn9 = assign29600_e28781_d_n9;
        locals.var_tmf1_dn10 = assign29600_e28781_d_n10;
        locals.var_tmf1_dn11 = assign29600_e28781_d_n11;
        locals.var_tmf1_dn14 = assign29600_e28781_d_n14;

        let (assign29610_e28793, assign29610_e28793_d_n0, assign29610_e28793_d_n2, assign29610_e28793_d_n4, assign29610_e28793_d_n5, assign29610_e28793_d_n6, assign29610_e28793_d_n7, assign29610_e28793_d_n8, assign29610_e28793_d_n9, assign29610_e28793_d_n10, assign29610_e28793_d_n11, assign29610_e28793_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29610_e28791: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign29610_e28791, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign29610_e28793;
        locals.var_x2_dn0 = assign29610_e28793_d_n0;
        locals.var_x2_dn2 = assign29610_e28793_d_n2;
        locals.var_x2_dn4 = assign29610_e28793_d_n4;
        locals.var_x2_dn5 = assign29610_e28793_d_n5;
        locals.var_x2_dn6 = assign29610_e28793_d_n6;
        locals.var_x2_dn7 = assign29610_e28793_d_n7;
        locals.var_x2_dn8 = assign29610_e28793_d_n8;
        locals.var_x2_dn9 = assign29610_e28793_d_n9;
        locals.var_x2_dn10 = assign29610_e28793_d_n10;
        locals.var_x2_dn11 = assign29610_e28793_d_n11;
        locals.var_x2_dn14 = assign29610_e28793_d_n14;

        let (assign29620_e28805, assign29620_e28805_d_n0, assign29620_e28805_d_n2, assign29620_e28805_d_n4, assign29620_e28805_d_n5, assign29620_e28805_d_n6, assign29620_e28805_d_n7, assign29620_e28805_d_n8, assign29620_e28805_d_n9, assign29620_e28805_d_n10, assign29620_e28805_d_n11, assign29620_e28805_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29620_e28803: f64 = (2.0 * 2.0);
        (assign29620_e28803, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign29620_e28805;
        locals.var_xmax2_dn0 = assign29620_e28805_d_n0;
        locals.var_xmax2_dn2 = assign29620_e28805_d_n2;
        locals.var_xmax2_dn4 = assign29620_e28805_d_n4;
        locals.var_xmax2_dn5 = assign29620_e28805_d_n5;
        locals.var_xmax2_dn6 = assign29620_e28805_d_n6;
        locals.var_xmax2_dn7 = assign29620_e28805_d_n7;
        locals.var_xmax2_dn8 = assign29620_e28805_d_n8;
        locals.var_xmax2_dn9 = assign29620_e28805_d_n9;
        locals.var_xmax2_dn10 = assign29620_e28805_d_n10;
        locals.var_xmax2_dn11 = assign29620_e28805_d_n11;
        locals.var_xmax2_dn14 = assign29620_e28805_d_n14;

        let (assign29630_e28815, assign29630_e28815_d_n0, assign29630_e28815_d_n2, assign29630_e28815_d_n4, assign29630_e28815_d_n5, assign29630_e28815_d_n6, assign29630_e28815_d_n7, assign29630_e28815_d_n8, assign29630_e28815_d_n9, assign29630_e28815_d_n10, assign29630_e28815_d_n11, assign29630_e28815_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29630_e28815;
        locals.var_xp_dn0 = assign29630_e28815_d_n0;
        locals.var_xp_dn2 = assign29630_e28815_d_n2;
        locals.var_xp_dn4 = assign29630_e28815_d_n4;
        locals.var_xp_dn5 = assign29630_e28815_d_n5;
        locals.var_xp_dn6 = assign29630_e28815_d_n6;
        locals.var_xp_dn7 = assign29630_e28815_d_n7;
        locals.var_xp_dn8 = assign29630_e28815_d_n8;
        locals.var_xp_dn9 = assign29630_e28815_d_n9;
        locals.var_xp_dn10 = assign29630_e28815_d_n10;
        locals.var_xp_dn11 = assign29630_e28815_d_n11;
        locals.var_xp_dn14 = assign29630_e28815_d_n14;

        let (assign29640_e28825, assign29640_e28825_d_n0, assign29640_e28825_d_n2, assign29640_e28825_d_n4, assign29640_e28825_d_n5, assign29640_e28825_d_n6, assign29640_e28825_d_n7, assign29640_e28825_d_n8, assign29640_e28825_d_n9, assign29640_e28825_d_n10, assign29640_e28825_d_n11, assign29640_e28825_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29640_e28825;
        locals.var_xmp_dn0 = assign29640_e28825_d_n0;
        locals.var_xmp_dn2 = assign29640_e28825_d_n2;
        locals.var_xmp_dn4 = assign29640_e28825_d_n4;
        locals.var_xmp_dn5 = assign29640_e28825_d_n5;
        locals.var_xmp_dn6 = assign29640_e28825_d_n6;
        locals.var_xmp_dn7 = assign29640_e28825_d_n7;
        locals.var_xmp_dn8 = assign29640_e28825_d_n8;
        locals.var_xmp_dn9 = assign29640_e28825_d_n9;
        locals.var_xmp_dn10 = assign29640_e28825_d_n10;
        locals.var_xmp_dn11 = assign29640_e28825_d_n11;
        locals.var_xmp_dn14 = assign29640_e28825_d_n14;

        let (assign29650_e28835,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29650_e28835;

        let (assign29660_e28845,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29660_e28845;

        let (assign29670_e28855, assign29670_e28855_d_n0, assign29670_e28855_d_n2, assign29670_e28855_d_n4, assign29670_e28855_d_n5, assign29670_e28855_d_n6, assign29670_e28855_d_n7, assign29670_e28855_d_n8, assign29670_e28855_d_n9, assign29670_e28855_d_n10, assign29670_e28855_d_n11, assign29670_e28855_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29670_e28855;
        locals.var_arg_dn0 = assign29670_e28855_d_n0;
        locals.var_arg_dn2 = assign29670_e28855_d_n2;
        locals.var_arg_dn4 = assign29670_e28855_d_n4;
        locals.var_arg_dn5 = assign29670_e28855_d_n5;
        locals.var_arg_dn6 = assign29670_e28855_d_n6;
        locals.var_arg_dn7 = assign29670_e28855_d_n7;
        locals.var_arg_dn8 = assign29670_e28855_d_n8;
        locals.var_arg_dn9 = assign29670_e28855_d_n9;
        locals.var_arg_dn10 = assign29670_e28855_d_n10;
        locals.var_arg_dn11 = assign29670_e28855_d_n11;
        locals.var_arg_dn14 = assign29670_e28855_d_n14;

        let (assign29680_e28865, assign29680_e28865_d_n0, assign29680_e28865_d_n2, assign29680_e28865_d_n4, assign29680_e28865_d_n5, assign29680_e28865_d_n6, assign29680_e28865_d_n7, assign29680_e28865_d_n8, assign29680_e28865_d_n9, assign29680_e28865_d_n10, assign29680_e28865_d_n11, assign29680_e28865_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29680_e28865;
        locals.var_dnm_dn0 = assign29680_e28865_d_n0;
        locals.var_dnm_dn2 = assign29680_e28865_d_n2;
        locals.var_dnm_dn4 = assign29680_e28865_d_n4;
        locals.var_dnm_dn5 = assign29680_e28865_d_n5;
        locals.var_dnm_dn6 = assign29680_e28865_d_n6;
        locals.var_dnm_dn7 = assign29680_e28865_d_n7;
        locals.var_dnm_dn8 = assign29680_e28865_d_n8;
        locals.var_dnm_dn9 = assign29680_e28865_d_n9;
        locals.var_dnm_dn10 = assign29680_e28865_d_n10;
        locals.var_dnm_dn11 = assign29680_e28865_d_n11;
        locals.var_dnm_dn14 = assign29680_e28865_d_n14;

        let (assign29690_e28877, assign29690_e28877_d_n0, assign29690_e28877_d_n2, assign29690_e28877_d_n4, assign29690_e28877_d_n5, assign29690_e28877_d_n6, assign29690_e28877_d_n7, assign29690_e28877_d_n8, assign29690_e28877_d_n9, assign29690_e28877_d_n10, assign29690_e28877_d_n11, assign29690_e28877_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29690_e28875: f64 = (locals.var_xp * locals.var_x2);
        (assign29690_e28875, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29690_e28877;
        locals.var_xp_dn0 = assign29690_e28877_d_n0;
        locals.var_xp_dn2 = assign29690_e28877_d_n2;
        locals.var_xp_dn4 = assign29690_e28877_d_n4;
        locals.var_xp_dn5 = assign29690_e28877_d_n5;
        locals.var_xp_dn6 = assign29690_e28877_d_n6;
        locals.var_xp_dn7 = assign29690_e28877_d_n7;
        locals.var_xp_dn8 = assign29690_e28877_d_n8;
        locals.var_xp_dn9 = assign29690_e28877_d_n9;
        locals.var_xp_dn10 = assign29690_e28877_d_n10;
        locals.var_xp_dn11 = assign29690_e28877_d_n11;
        locals.var_xp_dn14 = assign29690_e28877_d_n14;

        let (assign29700_e28889, assign29700_e28889_d_n0, assign29700_e28889_d_n2, assign29700_e28889_d_n4, assign29700_e28889_d_n5, assign29700_e28889_d_n6, assign29700_e28889_d_n7, assign29700_e28889_d_n8, assign29700_e28889_d_n9, assign29700_e28889_d_n10, assign29700_e28889_d_n11, assign29700_e28889_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29700_e28887: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29700_e28887, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29700_e28889;
        locals.var_xmp_dn0 = assign29700_e28889_d_n0;
        locals.var_xmp_dn2 = assign29700_e28889_d_n2;
        locals.var_xmp_dn4 = assign29700_e28889_d_n4;
        locals.var_xmp_dn5 = assign29700_e28889_d_n5;
        locals.var_xmp_dn6 = assign29700_e28889_d_n6;
        locals.var_xmp_dn7 = assign29700_e28889_d_n7;
        locals.var_xmp_dn8 = assign29700_e28889_d_n8;
        locals.var_xmp_dn9 = assign29700_e28889_d_n9;
        locals.var_xmp_dn10 = assign29700_e28889_d_n10;
        locals.var_xmp_dn11 = assign29700_e28889_d_n11;
        locals.var_xmp_dn14 = assign29700_e28889_d_n14;

        let (assign29710_e28901, assign29710_e28901_d_n0, assign29710_e28901_d_n2, assign29710_e28901_d_n4, assign29710_e28901_d_n5, assign29710_e28901_d_n6, assign29710_e28901_d_n7, assign29710_e28901_d_n8, assign29710_e28901_d_n9, assign29710_e28901_d_n10, assign29710_e28901_d_n11, assign29710_e28901_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29710_e28899: f64 = (locals.var_xp * locals.var_x2);
        (assign29710_e28899, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign29710_e28901;
        locals.var_xp_dn0 = assign29710_e28901_d_n0;
        locals.var_xp_dn2 = assign29710_e28901_d_n2;
        locals.var_xp_dn4 = assign29710_e28901_d_n4;
        locals.var_xp_dn5 = assign29710_e28901_d_n5;
        locals.var_xp_dn6 = assign29710_e28901_d_n6;
        locals.var_xp_dn7 = assign29710_e28901_d_n7;
        locals.var_xp_dn8 = assign29710_e28901_d_n8;
        locals.var_xp_dn9 = assign29710_e28901_d_n9;
        locals.var_xp_dn10 = assign29710_e28901_d_n10;
        locals.var_xp_dn11 = assign29710_e28901_d_n11;
        locals.var_xp_dn14 = assign29710_e28901_d_n14;

        let (assign29720_e28913, assign29720_e28913_d_n0, assign29720_e28913_d_n2, assign29720_e28913_d_n4, assign29720_e28913_d_n5, assign29720_e28913_d_n6, assign29720_e28913_d_n7, assign29720_e28913_d_n8, assign29720_e28913_d_n9, assign29720_e28913_d_n10, assign29720_e28913_d_n11, assign29720_e28913_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29720_e28911: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign29720_e28911, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign29720_e28913;
        locals.var_xmp_dn0 = assign29720_e28913_d_n0;
        locals.var_xmp_dn2 = assign29720_e28913_d_n2;
        locals.var_xmp_dn4 = assign29720_e28913_d_n4;
        locals.var_xmp_dn5 = assign29720_e28913_d_n5;
        locals.var_xmp_dn6 = assign29720_e28913_d_n6;
        locals.var_xmp_dn7 = assign29720_e28913_d_n7;
        locals.var_xmp_dn8 = assign29720_e28913_d_n8;
        locals.var_xmp_dn9 = assign29720_e28913_d_n9;
        locals.var_xmp_dn10 = assign29720_e28913_d_n10;
        locals.var_xmp_dn11 = assign29720_e28913_d_n11;
        locals.var_xmp_dn14 = assign29720_e28913_d_n14;

        let (assign29730_e28925, assign29730_e28925_d_n0, assign29730_e28925_d_n2, assign29730_e28925_d_n4, assign29730_e28925_d_n5, assign29730_e28925_d_n6, assign29730_e28925_d_n7, assign29730_e28925_d_n8, assign29730_e28925_d_n9, assign29730_e28925_d_n10, assign29730_e28925_d_n11, assign29730_e28925_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29730_e28923: f64 = (locals.var_xp + locals.var_xmp);
        (assign29730_e28923, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign29730_e28925;
        locals.var_arg_dn0 = assign29730_e28925_d_n0;
        locals.var_arg_dn2 = assign29730_e28925_d_n2;
        locals.var_arg_dn4 = assign29730_e28925_d_n4;
        locals.var_arg_dn5 = assign29730_e28925_d_n5;
        locals.var_arg_dn6 = assign29730_e28925_d_n6;
        locals.var_arg_dn7 = assign29730_e28925_d_n7;
        locals.var_arg_dn8 = assign29730_e28925_d_n8;
        locals.var_arg_dn9 = assign29730_e28925_d_n9;
        locals.var_arg_dn10 = assign29730_e28925_d_n10;
        locals.var_arg_dn11 = assign29730_e28925_d_n11;
        locals.var_arg_dn14 = assign29730_e28925_d_n14;

        let (assign29740_e28935, assign29740_e28935_d_n0, assign29740_e28935_d_n2, assign29740_e28935_d_n4, assign29740_e28935_d_n5, assign29740_e28935_d_n6, assign29740_e28935_d_n7, assign29740_e28935_d_n8, assign29740_e28935_d_n9, assign29740_e28935_d_n10, assign29740_e28935_d_n11, assign29740_e28935_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29740_e28935;
        locals.var_dnm_dn0 = assign29740_e28935_d_n0;
        locals.var_dnm_dn2 = assign29740_e28935_d_n2;
        locals.var_dnm_dn4 = assign29740_e28935_d_n4;
        locals.var_dnm_dn5 = assign29740_e28935_d_n5;
        locals.var_dnm_dn6 = assign29740_e28935_d_n6;
        locals.var_dnm_dn7 = assign29740_e28935_d_n7;
        locals.var_dnm_dn8 = assign29740_e28935_d_n8;
        locals.var_dnm_dn9 = assign29740_e28935_d_n9;
        locals.var_dnm_dn10 = assign29740_e28935_d_n10;
        locals.var_dnm_dn11 = assign29740_e28935_d_n11;
        locals.var_dnm_dn14 = assign29740_e28935_d_n14;

        let assign29750_e28950: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard690 = assign29750_e28950;

        let assign29760_e28953: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard691 = assign29760_e28953;

        let (assign29770_e28967,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29770_e28967;

        let assign29780_e28970: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard692 = assign29780_e28970;

        let (assign29790_e28987,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29790_e28987;

        let assign29800_e28990: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard693 = assign29800_e28990;

        let (assign29810_e29010,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 == 0.0)) && (locals.var_guard693 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29810_e29010;

        let assign29820_e29013: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard694 = assign29820_e29013;

        let (assign29830_e29036,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29830_e29036;

        let (assign29840_e29048,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29840_e29048;

    }

    pub(super) fn stamp_transient_block_86(
        locals: &mut StampLocals,
    ) {
        let mut assign29850_loop_guard: usize = 0;
        while {
            let assign29850_cond_e29061: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign29850_cond_e29061 != 0.0
        } {
            assign29850_loop_guard += 1;
            assert!(assign29850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29850_body0_e29074, assign29850_body0_e29074_d_n0, assign29850_body0_e29074_d_n2, assign29850_body0_e29074_d_n4, assign29850_body0_e29074_d_n5, assign29850_body0_e29074_d_n6, assign29850_body0_e29074_d_n7, assign29850_body0_e29074_d_n8, assign29850_body0_e29074_d_n9, assign29850_body0_e29074_d_n10, assign29850_body0_e29074_d_n11, assign29850_body0_e29074_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29850_body0_e29072: f64 = (locals.var_dnm).sqrt();
        (assign29850_body0_e29072, (locals.var_dnm_dn0 / (2.0 * assign29850_body0_e29072)), (locals.var_dnm_dn2 / (2.0 * assign29850_body0_e29072)), (locals.var_dnm_dn4 / (2.0 * assign29850_body0_e29072)), (locals.var_dnm_dn5 / (2.0 * assign29850_body0_e29072)), (locals.var_dnm_dn6 / (2.0 * assign29850_body0_e29072)), (locals.var_dnm_dn7 / (2.0 * assign29850_body0_e29072)), (locals.var_dnm_dn8 / (2.0 * assign29850_body0_e29072)), (locals.var_dnm_dn9 / (2.0 * assign29850_body0_e29072)), (locals.var_dnm_dn10 / (2.0 * assign29850_body0_e29072)), (locals.var_dnm_dn11 / (2.0 * assign29850_body0_e29072)), (locals.var_dnm_dn14 / (2.0 * assign29850_body0_e29072)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign29850_body0_e29074;
            locals.var_dnm_dn0 = assign29850_body0_e29074_d_n0;
            locals.var_dnm_dn2 = assign29850_body0_e29074_d_n2;
            locals.var_dnm_dn4 = assign29850_body0_e29074_d_n4;
            locals.var_dnm_dn5 = assign29850_body0_e29074_d_n5;
            locals.var_dnm_dn6 = assign29850_body0_e29074_d_n6;
            locals.var_dnm_dn7 = assign29850_body0_e29074_d_n7;
            locals.var_dnm_dn8 = assign29850_body0_e29074_d_n8;
            locals.var_dnm_dn9 = assign29850_body0_e29074_d_n9;
            locals.var_dnm_dn10 = assign29850_body0_e29074_d_n10;
            locals.var_dnm_dn11 = assign29850_body0_e29074_d_n11;
            locals.var_dnm_dn14 = assign29850_body0_e29074_d_n14;
            let (assign29850_body1_e29088,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign29850_body1_e29086: f64 = (locals.var_m0 + 1.0);
        (assign29850_body1_e29086,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29850_body1_e29088;
        }

        let (assign29860_e29112, assign29860_e29112_d_n0, assign29860_e29112_d_n2, assign29860_e29112_d_n4, assign29860_e29112_d_n5, assign29860_e29112_d_n6, assign29860_e29112_d_n7, assign29860_e29112_d_n8, assign29860_e29112_d_n9, assign29860_e29112_d_n10, assign29860_e29112_d_n11, assign29860_e29112_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 == 0.0)) {
        let (assign29860_e29110, assign29860_e29110_d_n0, assign29860_e29110_d_n2, assign29860_e29110_d_n4, assign29860_e29110_d_n5, assign29860_e29110_d_n6, assign29860_e29110_d_n7, assign29860_e29110_d_n8, assign29860_e29110_d_n9, assign29860_e29110_d_n10, assign29860_e29110_d_n11, assign29860_e29110_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29860_e29107: f64 = (2.0 * 2.0);
                let assign29860_e29108: f64 = (1.0 / assign29860_e29107);
                let assign29860_e29109: f64 = (locals.var_dnm).powf(assign29860_e29108);
                (assign29860_e29109, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn0)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn2)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn4)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn5)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn6)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn7)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn8)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn9)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn10)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn11)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign29860_e29108) as f64).is_finite() && ((assign29860_e29108) as f64).fract() == 0.0 { if assign29860_e29108 == 0.0 { 0.0 } else { (assign29860_e29108 * ((locals.var_dnm).powf(assign29860_e29108 - 1.0) * locals.var_dnm_dn14)) } } else { (assign29860_e29109 * (assign29860_e29108 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign29860_e29110, assign29860_e29110_d_n0, assign29860_e29110_d_n2, assign29860_e29110_d_n4, assign29860_e29110_d_n5, assign29860_e29110_d_n6, assign29860_e29110_d_n7, assign29860_e29110_d_n8, assign29860_e29110_d_n9, assign29860_e29110_d_n10, assign29860_e29110_d_n11, assign29860_e29110_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29860_e29112;
        locals.var_dnm_dn0 = assign29860_e29112_d_n0;
        locals.var_dnm_dn2 = assign29860_e29112_d_n2;
        locals.var_dnm_dn4 = assign29860_e29112_d_n4;
        locals.var_dnm_dn5 = assign29860_e29112_d_n5;
        locals.var_dnm_dn6 = assign29860_e29112_d_n6;
        locals.var_dnm_dn7 = assign29860_e29112_d_n7;
        locals.var_dnm_dn8 = assign29860_e29112_d_n8;
        locals.var_dnm_dn9 = assign29860_e29112_d_n9;
        locals.var_dnm_dn10 = assign29860_e29112_d_n10;
        locals.var_dnm_dn11 = assign29860_e29112_d_n11;
        locals.var_dnm_dn14 = assign29860_e29112_d_n14;

        let (assign29870_e29124, assign29870_e29124_d_n0, assign29870_e29124_d_n2, assign29870_e29124_d_n4, assign29870_e29124_d_n5, assign29870_e29124_d_n6, assign29870_e29124_d_n7, assign29870_e29124_d_n8, assign29870_e29124_d_n9, assign29870_e29124_d_n10, assign29870_e29124_d_n11, assign29870_e29124_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29870_e29122: f64 = (1.0 / locals.var_dnm);
        (assign29870_e29122, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign29870_e29124;
        locals.var_dnm_dn0 = assign29870_e29124_d_n0;
        locals.var_dnm_dn2 = assign29870_e29124_d_n2;
        locals.var_dnm_dn4 = assign29870_e29124_d_n4;
        locals.var_dnm_dn5 = assign29870_e29124_d_n5;
        locals.var_dnm_dn6 = assign29870_e29124_d_n6;
        locals.var_dnm_dn7 = assign29870_e29124_d_n7;
        locals.var_dnm_dn8 = assign29870_e29124_d_n8;
        locals.var_dnm_dn9 = assign29870_e29124_d_n9;
        locals.var_dnm_dn10 = assign29870_e29124_d_n10;
        locals.var_dnm_dn11 = assign29870_e29124_d_n11;
        locals.var_dnm_dn14 = assign29870_e29124_d_n14;

        let (assign29880_e29138, assign29880_e29138_d_n0, assign29880_e29138_d_n2, assign29880_e29138_d_n4, assign29880_e29138_d_n5, assign29880_e29138_d_n6, assign29880_e29138_d_n7, assign29880_e29138_d_n8, assign29880_e29138_d_n9, assign29880_e29138_d_n10, assign29880_e29138_d_n11, assign29880_e29138_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29880_e29134: f64 = (locals.var_tmf1 * 2.0);
        let assign29880_e29136: f64 = (assign29880_e29134 * locals.var_dnm);
        (assign29880_e29136, (((locals.var_tmf1_dn0 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 2.0) * locals.var_dnm) + (assign29880_e29134 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign29880_e29138;
        locals.var_tmf0_dn0 = assign29880_e29138_d_n0;
        locals.var_tmf0_dn2 = assign29880_e29138_d_n2;
        locals.var_tmf0_dn4 = assign29880_e29138_d_n4;
        locals.var_tmf0_dn5 = assign29880_e29138_d_n5;
        locals.var_tmf0_dn6 = assign29880_e29138_d_n6;
        locals.var_tmf0_dn7 = assign29880_e29138_d_n7;
        locals.var_tmf0_dn8 = assign29880_e29138_d_n8;
        locals.var_tmf0_dn9 = assign29880_e29138_d_n9;
        locals.var_tmf0_dn10 = assign29880_e29138_d_n10;
        locals.var_tmf0_dn11 = assign29880_e29138_d_n11;
        locals.var_tmf0_dn14 = assign29880_e29138_d_n14;

        let (assign29890_e29154, assign29890_e29154_d_n0, assign29890_e29154_d_n2, assign29890_e29154_d_n4, assign29890_e29154_d_n5, assign29890_e29154_d_n6, assign29890_e29154_d_n7, assign29890_e29154_d_n8, assign29890_e29154_d_n9, assign29890_e29154_d_n10, assign29890_e29154_d_n11, assign29890_e29154_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29890_e29148: f64 = (2.0 * locals.var_xmp);
        let assign29890_e29150: f64 = (assign29890_e29148 * locals.var_dnm);
        let assign29890_e29152: f64 = (assign29890_e29150 / locals.var_arg);
        (assign29890_e29152, ((((((2.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn0)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn2)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn4)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn5)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn6)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn7)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn8)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn9)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn10)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn11)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign29890_e29148 * locals.var_dnm_dn14)) * locals.var_arg) - (assign29890_e29150 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29890_e29154;
        locals.var_t0_dn0 = assign29890_e29154_d_n0;
        locals.var_t0_dn2 = assign29890_e29154_d_n2;
        locals.var_t0_dn4 = assign29890_e29154_d_n4;
        locals.var_t0_dn5 = assign29890_e29154_d_n5;
        locals.var_t0_dn6 = assign29890_e29154_d_n6;
        locals.var_t0_dn7 = assign29890_e29154_d_n7;
        locals.var_t0_dn8 = assign29890_e29154_d_n8;
        locals.var_t0_dn9 = assign29890_e29154_d_n9;
        locals.var_t0_dn10 = assign29890_e29154_d_n10;
        locals.var_t0_dn11 = assign29890_e29154_d_n11;
        locals.var_t0_dn14 = assign29890_e29154_d_n14;

        let (assign29900_e29168, assign29900_e29168_d_n0, assign29900_e29168_d_n2, assign29900_e29168_d_n4, assign29900_e29168_d_n5, assign29900_e29168_d_n6, assign29900_e29168_d_n7, assign29900_e29168_d_n8, assign29900_e29168_d_n9, assign29900_e29168_d_n10, assign29900_e29168_d_n11, assign29900_e29168_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign29900_e29164: f64 = 2.0;
        let assign29900_e29166: f64 = (assign29900_e29164 - locals.var_tmf0);
        (assign29900_e29166, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign29900_e29168;
        locals.var_t9_dn0 = assign29900_e29168_d_n0;
        locals.var_t9_dn2 = assign29900_e29168_d_n2;
        locals.var_t9_dn4 = assign29900_e29168_d_n4;
        locals.var_t9_dn5 = assign29900_e29168_d_n5;
        locals.var_t9_dn6 = assign29900_e29168_d_n6;
        locals.var_t9_dn7 = assign29900_e29168_d_n7;
        locals.var_t9_dn8 = assign29900_e29168_d_n8;
        locals.var_t9_dn9 = assign29900_e29168_d_n9;
        locals.var_t9_dn10 = assign29900_e29168_d_n10;
        locals.var_t9_dn11 = assign29900_e29168_d_n11;
        locals.var_t9_dn14 = assign29900_e29168_d_n14;

        let (assign29910_e29178, assign29910_e29178_d_n0, assign29910_e29178_d_n2, assign29910_e29178_d_n4, assign29910_e29178_d_n5, assign29910_e29178_d_n6, assign29910_e29178_d_n7, assign29910_e29178_d_n8, assign29910_e29178_d_n9, assign29910_e29178_d_n10, assign29910_e29178_d_n11, assign29910_e29178_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29910_e29178;
        locals.var_t0_dn0 = assign29910_e29178_d_n0;
        locals.var_t0_dn2 = assign29910_e29178_d_n2;
        locals.var_t0_dn4 = assign29910_e29178_d_n4;
        locals.var_t0_dn5 = assign29910_e29178_d_n5;
        locals.var_t0_dn6 = assign29910_e29178_d_n6;
        locals.var_t0_dn7 = assign29910_e29178_d_n7;
        locals.var_t0_dn8 = assign29910_e29178_d_n8;
        locals.var_t0_dn9 = assign29910_e29178_d_n9;
        locals.var_t0_dn10 = assign29910_e29178_d_n10;
        locals.var_t0_dn11 = assign29910_e29178_d_n11;
        locals.var_t0_dn14 = assign29910_e29178_d_n14;

        let (assign29920_e29189, assign29920_e29189_d_n0, assign29920_e29189_d_n2, assign29920_e29189_d_n4, assign29920_e29189_d_n5, assign29920_e29189_d_n6, assign29920_e29189_d_n7, assign29920_e29189_d_n8, assign29920_e29189_d_n9, assign29920_e29189_d_n10, assign29920_e29189_d_n11, assign29920_e29189_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign29920_e29189;
        locals.var_t9_dn0 = assign29920_e29189_d_n0;
        locals.var_t9_dn2 = assign29920_e29189_d_n2;
        locals.var_t9_dn4 = assign29920_e29189_d_n4;
        locals.var_t9_dn5 = assign29920_e29189_d_n5;
        locals.var_t9_dn6 = assign29920_e29189_d_n6;
        locals.var_t9_dn7 = assign29920_e29189_d_n7;
        locals.var_t9_dn8 = assign29920_e29189_d_n8;
        locals.var_t9_dn9 = assign29920_e29189_d_n9;
        locals.var_t9_dn10 = assign29920_e29189_d_n10;
        locals.var_t9_dn11 = assign29920_e29189_d_n11;
        locals.var_t9_dn14 = assign29920_e29189_d_n14;

        let (assign29930_e29200, assign29930_e29200_d_n0, assign29930_e29200_d_n2, assign29930_e29200_d_n4, assign29930_e29200_d_n5, assign29930_e29200_d_n6, assign29930_e29200_d_n7, assign29930_e29200_d_n8, assign29930_e29200_d_n9, assign29930_e29200_d_n10, assign29930_e29200_d_n11, assign29930_e29200_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign29930_e29200;
        locals.var_t0_dn0 = assign29930_e29200_d_n0;
        locals.var_t0_dn2 = assign29930_e29200_d_n2;
        locals.var_t0_dn4 = assign29930_e29200_d_n4;
        locals.var_t0_dn5 = assign29930_e29200_d_n5;
        locals.var_t0_dn6 = assign29930_e29200_d_n6;
        locals.var_t0_dn7 = assign29930_e29200_d_n7;
        locals.var_t0_dn8 = assign29930_e29200_d_n8;
        locals.var_t0_dn9 = assign29930_e29200_d_n9;
        locals.var_t0_dn10 = assign29930_e29200_d_n10;
        locals.var_t0_dn11 = assign29930_e29200_d_n11;
        locals.var_t0_dn14 = assign29930_e29200_d_n14;

        let (assign29940_e29210, assign29940_e29210_d_n0, assign29940_e29210_d_n2, assign29940_e29210_d_n4, assign29940_e29210_d_n5, assign29940_e29210_d_n6, assign29940_e29210_d_n7, assign29940_e29210_d_n8, assign29940_e29210_d_n9, assign29940_e29210_d_n10, assign29940_e29210_d_n11, assign29940_e29210_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign29940_e29208: f64 = (locals.var_t9 + 1e-25);
        (assign29940_e29208, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign29940_e29210;
        locals.var_t9_dn0 = assign29940_e29210_d_n0;
        locals.var_t9_dn2 = assign29940_e29210_d_n2;
        locals.var_t9_dn4 = assign29940_e29210_d_n4;
        locals.var_t9_dn5 = assign29940_e29210_d_n5;
        locals.var_t9_dn6 = assign29940_e29210_d_n6;
        locals.var_t9_dn7 = assign29940_e29210_d_n7;
        locals.var_t9_dn8 = assign29940_e29210_d_n8;
        locals.var_t9_dn9 = assign29940_e29210_d_n9;
        locals.var_t9_dn10 = assign29940_e29210_d_n10;
        locals.var_t9_dn11 = assign29940_e29210_d_n11;
        locals.var_t9_dn14 = assign29940_e29210_d_n14;

        let (assign29950_e29219, assign29950_e29219_d_n0, assign29950_e29219_d_n2, assign29950_e29219_d_n4, assign29950_e29219_d_n5, assign29950_e29219_d_n6, assign29950_e29219_d_n7, assign29950_e29219_d_n8, assign29950_e29219_d_n9, assign29950_e29219_d_n10, assign29950_e29219_d_n11, assign29950_e29219_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign29950_e29217: f64 = (locals.var_t9).sqrt();
        (assign29950_e29217, (locals.var_t9_dn0 / (2.0 * assign29950_e29217)), (locals.var_t9_dn2 / (2.0 * assign29950_e29217)), (locals.var_t9_dn4 / (2.0 * assign29950_e29217)), (locals.var_t9_dn5 / (2.0 * assign29950_e29217)), (locals.var_t9_dn6 / (2.0 * assign29950_e29217)), (locals.var_t9_dn7 / (2.0 * assign29950_e29217)), (locals.var_t9_dn8 / (2.0 * assign29950_e29217)), (locals.var_t9_dn9 / (2.0 * assign29950_e29217)), (locals.var_t9_dn10 / (2.0 * assign29950_e29217)), (locals.var_t9_dn11 / (2.0 * assign29950_e29217)), (locals.var_t9_dn14 / (2.0 * assign29950_e29217)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign29950_e29219;
        locals.var_t3_dn0 = assign29950_e29219_d_n0;
        locals.var_t3_dn2 = assign29950_e29219_d_n2;
        locals.var_t3_dn4 = assign29950_e29219_d_n4;
        locals.var_t3_dn5 = assign29950_e29219_d_n5;
        locals.var_t3_dn6 = assign29950_e29219_d_n6;
        locals.var_t3_dn7 = assign29950_e29219_d_n7;
        locals.var_t3_dn8 = assign29950_e29219_d_n8;
        locals.var_t3_dn9 = assign29950_e29219_d_n9;
        locals.var_t3_dn10 = assign29950_e29219_d_n10;
        locals.var_t3_dn11 = assign29950_e29219_d_n11;
        locals.var_t3_dn14 = assign29950_e29219_d_n14;

        let (assign29960_e29231, assign29960_e29231_d_n0, assign29960_e29231_d_n2, assign29960_e29231_d_n4, assign29960_e29231_d_n5, assign29960_e29231_d_n6, assign29960_e29231_d_n7, assign29960_e29231_d_n8, assign29960_e29231_d_n9, assign29960_e29231_d_n10, assign29960_e29231_d_n11, assign29960_e29231_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign29960_e29228: f64 = (1.0 - locals.var_t3);
        let assign29960_e29229: f64 = (locals.var_t2 * assign29960_e29228);
        (assign29960_e29229, ((locals.var_t2_dn0 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn0))), ((locals.var_t2_dn2 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn2))), ((locals.var_t2_dn4 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn4))), ((locals.var_t2_dn5 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn5))), ((locals.var_t2_dn6 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn6))), ((locals.var_t2_dn7 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn7))), ((locals.var_t2_dn8 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn8))), ((locals.var_t2_dn9 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn9))), ((locals.var_t2_dn10 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn10))), ((locals.var_t2_dn11 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn11))), ((locals.var_t2_dn14 * assign29960_e29228) + (locals.var_t2 * (-locals.var_t3_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign29960_e29231;
        locals.var_t4_dn0 = assign29960_e29231_d_n0;
        locals.var_t4_dn2 = assign29960_e29231_d_n2;
        locals.var_t4_dn4 = assign29960_e29231_d_n4;
        locals.var_t4_dn5 = assign29960_e29231_d_n5;
        locals.var_t4_dn6 = assign29960_e29231_d_n6;
        locals.var_t4_dn7 = assign29960_e29231_d_n7;
        locals.var_t4_dn8 = assign29960_e29231_d_n8;
        locals.var_t4_dn9 = assign29960_e29231_d_n9;
        locals.var_t4_dn10 = assign29960_e29231_d_n10;
        locals.var_t4_dn11 = assign29960_e29231_d_n11;
        locals.var_t4_dn14 = assign29960_e29231_d_n14;

        let (assign29970_e29243, assign29970_e29243_d_n0, assign29970_e29243_d_n2, assign29970_e29243_d_n4, assign29970_e29243_d_n5, assign29970_e29243_d_n6, assign29970_e29243_d_n7, assign29970_e29243_d_n8, assign29970_e29243_d_n9, assign29970_e29243_d_n10, assign29970_e29243_d_n11, assign29970_e29243_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign29970_e29239: f64 = (locals.var_vgp + 2.0);
        let assign29970_e29241: f64 = (assign29970_e29239 + locals.var_t4);
        (assign29970_e29241, (locals.var_vgp_dn0 + locals.var_t4_dn0), (locals.var_vgp_dn2 + locals.var_t4_dn2), (locals.var_vgp_dn4 + locals.var_t4_dn4), (locals.var_vgp_dn5 + locals.var_t4_dn5), (locals.var_vgp_dn6 + locals.var_t4_dn6), (locals.var_vgp_dn7 + locals.var_t4_dn7), (locals.var_vgp_dn8 + locals.var_t4_dn8), (locals.var_vgp_dn9 + locals.var_t4_dn9), (locals.var_vgp_dn10 + locals.var_t4_dn10), (locals.var_vgp_dn11 + locals.var_t4_dn11), (locals.var_vgp_dn14 + locals.var_t4_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign29970_e29243;
        locals.var_t10_dn0 = assign29970_e29243_d_n0;
        locals.var_t10_dn2 = assign29970_e29243_d_n2;
        locals.var_t10_dn4 = assign29970_e29243_d_n4;
        locals.var_t10_dn5 = assign29970_e29243_d_n5;
        locals.var_t10_dn6 = assign29970_e29243_d_n6;
        locals.var_t10_dn7 = assign29970_e29243_d_n7;
        locals.var_t10_dn8 = assign29970_e29243_d_n8;
        locals.var_t10_dn9 = assign29970_e29243_d_n9;
        locals.var_t10_dn10 = assign29970_e29243_d_n10;
        locals.var_t10_dn11 = assign29970_e29243_d_n11;
        locals.var_t10_dn14 = assign29970_e29243_d_n14;

        let assign29980_e29247: f64 = (0.3 + 0.2);
        let assign29980_e29252: f64 = if ((locals.var_t10 < assign29980_e29247) && (0.2 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard695 = assign29980_e29252;

        let (assign29990_e29266, assign29990_e29266_d_n0, assign29990_e29266_d_n2, assign29990_e29266_d_n4, assign29990_e29266_d_n5, assign29990_e29266_d_n6, assign29990_e29266_d_n7, assign29990_e29266_d_n8, assign29990_e29266_d_n9, assign29990_e29266_d_n10, assign29990_e29266_d_n11, assign29990_e29266_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign29990_e29262: f64 = (0.3 + 0.2);
        let assign29990_e29264: f64 = (assign29990_e29262 - locals.var_t10);
        (assign29990_e29264, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign29990_e29266;
        locals.var_tmf1_dn0 = assign29990_e29266_d_n0;
        locals.var_tmf1_dn2 = assign29990_e29266_d_n2;
        locals.var_tmf1_dn4 = assign29990_e29266_d_n4;
        locals.var_tmf1_dn5 = assign29990_e29266_d_n5;
        locals.var_tmf1_dn6 = assign29990_e29266_d_n6;
        locals.var_tmf1_dn7 = assign29990_e29266_d_n7;
        locals.var_tmf1_dn8 = assign29990_e29266_d_n8;
        locals.var_tmf1_dn9 = assign29990_e29266_d_n9;
        locals.var_tmf1_dn10 = assign29990_e29266_d_n10;
        locals.var_tmf1_dn11 = assign29990_e29266_d_n11;
        locals.var_tmf1_dn14 = assign29990_e29266_d_n14;

        let (assign30000_e29278, assign30000_e29278_d_n0, assign30000_e29278_d_n2, assign30000_e29278_d_n4, assign30000_e29278_d_n5, assign30000_e29278_d_n6, assign30000_e29278_d_n7, assign30000_e29278_d_n8, assign30000_e29278_d_n9, assign30000_e29278_d_n10, assign30000_e29278_d_n11, assign30000_e29278_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30000_e29276: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign30000_e29276, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign30000_e29278;
        locals.var_x2_dn0 = assign30000_e29278_d_n0;
        locals.var_x2_dn2 = assign30000_e29278_d_n2;
        locals.var_x2_dn4 = assign30000_e29278_d_n4;
        locals.var_x2_dn5 = assign30000_e29278_d_n5;
        locals.var_x2_dn6 = assign30000_e29278_d_n6;
        locals.var_x2_dn7 = assign30000_e29278_d_n7;
        locals.var_x2_dn8 = assign30000_e29278_d_n8;
        locals.var_x2_dn9 = assign30000_e29278_d_n9;
        locals.var_x2_dn10 = assign30000_e29278_d_n10;
        locals.var_x2_dn11 = assign30000_e29278_d_n11;
        locals.var_x2_dn14 = assign30000_e29278_d_n14;

        let (assign30010_e29290, assign30010_e29290_d_n0, assign30010_e29290_d_n2, assign30010_e29290_d_n4, assign30010_e29290_d_n5, assign30010_e29290_d_n6, assign30010_e29290_d_n7, assign30010_e29290_d_n8, assign30010_e29290_d_n9, assign30010_e29290_d_n10, assign30010_e29290_d_n11, assign30010_e29290_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30010_e29288: f64 = (0.2 * 0.2);
        (assign30010_e29288, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign30010_e29290;
        locals.var_xmax2_dn0 = assign30010_e29290_d_n0;
        locals.var_xmax2_dn2 = assign30010_e29290_d_n2;
        locals.var_xmax2_dn4 = assign30010_e29290_d_n4;
        locals.var_xmax2_dn5 = assign30010_e29290_d_n5;
        locals.var_xmax2_dn6 = assign30010_e29290_d_n6;
        locals.var_xmax2_dn7 = assign30010_e29290_d_n7;
        locals.var_xmax2_dn8 = assign30010_e29290_d_n8;
        locals.var_xmax2_dn9 = assign30010_e29290_d_n9;
        locals.var_xmax2_dn10 = assign30010_e29290_d_n10;
        locals.var_xmax2_dn11 = assign30010_e29290_d_n11;
        locals.var_xmax2_dn14 = assign30010_e29290_d_n14;

        let (assign30020_e29300, assign30020_e29300_d_n0, assign30020_e29300_d_n2, assign30020_e29300_d_n4, assign30020_e29300_d_n5, assign30020_e29300_d_n6, assign30020_e29300_d_n7, assign30020_e29300_d_n8, assign30020_e29300_d_n9, assign30020_e29300_d_n10, assign30020_e29300_d_n11, assign30020_e29300_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30020_e29300;
        locals.var_xp_dn0 = assign30020_e29300_d_n0;
        locals.var_xp_dn2 = assign30020_e29300_d_n2;
        locals.var_xp_dn4 = assign30020_e29300_d_n4;
        locals.var_xp_dn5 = assign30020_e29300_d_n5;
        locals.var_xp_dn6 = assign30020_e29300_d_n6;
        locals.var_xp_dn7 = assign30020_e29300_d_n7;
        locals.var_xp_dn8 = assign30020_e29300_d_n8;
        locals.var_xp_dn9 = assign30020_e29300_d_n9;
        locals.var_xp_dn10 = assign30020_e29300_d_n10;
        locals.var_xp_dn11 = assign30020_e29300_d_n11;
        locals.var_xp_dn14 = assign30020_e29300_d_n14;

        let (assign30030_e29310, assign30030_e29310_d_n0, assign30030_e29310_d_n2, assign30030_e29310_d_n4, assign30030_e29310_d_n5, assign30030_e29310_d_n6, assign30030_e29310_d_n7, assign30030_e29310_d_n8, assign30030_e29310_d_n9, assign30030_e29310_d_n10, assign30030_e29310_d_n11, assign30030_e29310_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30030_e29310;
        locals.var_xmp_dn0 = assign30030_e29310_d_n0;
        locals.var_xmp_dn2 = assign30030_e29310_d_n2;
        locals.var_xmp_dn4 = assign30030_e29310_d_n4;
        locals.var_xmp_dn5 = assign30030_e29310_d_n5;
        locals.var_xmp_dn6 = assign30030_e29310_d_n6;
        locals.var_xmp_dn7 = assign30030_e29310_d_n7;
        locals.var_xmp_dn8 = assign30030_e29310_d_n8;
        locals.var_xmp_dn9 = assign30030_e29310_d_n9;
        locals.var_xmp_dn10 = assign30030_e29310_d_n10;
        locals.var_xmp_dn11 = assign30030_e29310_d_n11;
        locals.var_xmp_dn14 = assign30030_e29310_d_n14;

        let (assign30040_e29320,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30040_e29320;

        let (assign30050_e29330,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30050_e29330;

        let (assign30060_e29340, assign30060_e29340_d_n0, assign30060_e29340_d_n2, assign30060_e29340_d_n4, assign30060_e29340_d_n5, assign30060_e29340_d_n6, assign30060_e29340_d_n7, assign30060_e29340_d_n8, assign30060_e29340_d_n9, assign30060_e29340_d_n10, assign30060_e29340_d_n11, assign30060_e29340_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30060_e29340;
        locals.var_arg_dn0 = assign30060_e29340_d_n0;
        locals.var_arg_dn2 = assign30060_e29340_d_n2;
        locals.var_arg_dn4 = assign30060_e29340_d_n4;
        locals.var_arg_dn5 = assign30060_e29340_d_n5;
        locals.var_arg_dn6 = assign30060_e29340_d_n6;
        locals.var_arg_dn7 = assign30060_e29340_d_n7;
        locals.var_arg_dn8 = assign30060_e29340_d_n8;
        locals.var_arg_dn9 = assign30060_e29340_d_n9;
        locals.var_arg_dn10 = assign30060_e29340_d_n10;
        locals.var_arg_dn11 = assign30060_e29340_d_n11;
        locals.var_arg_dn14 = assign30060_e29340_d_n14;

        let (assign30070_e29350, assign30070_e29350_d_n0, assign30070_e29350_d_n2, assign30070_e29350_d_n4, assign30070_e29350_d_n5, assign30070_e29350_d_n6, assign30070_e29350_d_n7, assign30070_e29350_d_n8, assign30070_e29350_d_n9, assign30070_e29350_d_n10, assign30070_e29350_d_n11, assign30070_e29350_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30070_e29350;
        locals.var_dnm_dn0 = assign30070_e29350_d_n0;
        locals.var_dnm_dn2 = assign30070_e29350_d_n2;
        locals.var_dnm_dn4 = assign30070_e29350_d_n4;
        locals.var_dnm_dn5 = assign30070_e29350_d_n5;
        locals.var_dnm_dn6 = assign30070_e29350_d_n6;
        locals.var_dnm_dn7 = assign30070_e29350_d_n7;
        locals.var_dnm_dn8 = assign30070_e29350_d_n8;
        locals.var_dnm_dn9 = assign30070_e29350_d_n9;
        locals.var_dnm_dn10 = assign30070_e29350_d_n10;
        locals.var_dnm_dn11 = assign30070_e29350_d_n11;
        locals.var_dnm_dn14 = assign30070_e29350_d_n14;

        let (assign30080_e29362, assign30080_e29362_d_n0, assign30080_e29362_d_n2, assign30080_e29362_d_n4, assign30080_e29362_d_n5, assign30080_e29362_d_n6, assign30080_e29362_d_n7, assign30080_e29362_d_n8, assign30080_e29362_d_n9, assign30080_e29362_d_n10, assign30080_e29362_d_n11, assign30080_e29362_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30080_e29360: f64 = (locals.var_xp * locals.var_x2);
        (assign30080_e29360, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30080_e29362;
        locals.var_xp_dn0 = assign30080_e29362_d_n0;
        locals.var_xp_dn2 = assign30080_e29362_d_n2;
        locals.var_xp_dn4 = assign30080_e29362_d_n4;
        locals.var_xp_dn5 = assign30080_e29362_d_n5;
        locals.var_xp_dn6 = assign30080_e29362_d_n6;
        locals.var_xp_dn7 = assign30080_e29362_d_n7;
        locals.var_xp_dn8 = assign30080_e29362_d_n8;
        locals.var_xp_dn9 = assign30080_e29362_d_n9;
        locals.var_xp_dn10 = assign30080_e29362_d_n10;
        locals.var_xp_dn11 = assign30080_e29362_d_n11;
        locals.var_xp_dn14 = assign30080_e29362_d_n14;

        let (assign30090_e29374, assign30090_e29374_d_n0, assign30090_e29374_d_n2, assign30090_e29374_d_n4, assign30090_e29374_d_n5, assign30090_e29374_d_n6, assign30090_e29374_d_n7, assign30090_e29374_d_n8, assign30090_e29374_d_n9, assign30090_e29374_d_n10, assign30090_e29374_d_n11, assign30090_e29374_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30090_e29372: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30090_e29372, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30090_e29374;
        locals.var_xmp_dn0 = assign30090_e29374_d_n0;
        locals.var_xmp_dn2 = assign30090_e29374_d_n2;
        locals.var_xmp_dn4 = assign30090_e29374_d_n4;
        locals.var_xmp_dn5 = assign30090_e29374_d_n5;
        locals.var_xmp_dn6 = assign30090_e29374_d_n6;
        locals.var_xmp_dn7 = assign30090_e29374_d_n7;
        locals.var_xmp_dn8 = assign30090_e29374_d_n8;
        locals.var_xmp_dn9 = assign30090_e29374_d_n9;
        locals.var_xmp_dn10 = assign30090_e29374_d_n10;
        locals.var_xmp_dn11 = assign30090_e29374_d_n11;
        locals.var_xmp_dn14 = assign30090_e29374_d_n14;

    }

    pub(super) fn stamp_transient_block_87(
        locals: &mut StampLocals,
    ) {
        let (assign30100_e29386, assign30100_e29386_d_n0, assign30100_e29386_d_n2, assign30100_e29386_d_n4, assign30100_e29386_d_n5, assign30100_e29386_d_n6, assign30100_e29386_d_n7, assign30100_e29386_d_n8, assign30100_e29386_d_n9, assign30100_e29386_d_n10, assign30100_e29386_d_n11, assign30100_e29386_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30100_e29384: f64 = (locals.var_xp * locals.var_x2);
        (assign30100_e29384, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30100_e29386;
        locals.var_xp_dn0 = assign30100_e29386_d_n0;
        locals.var_xp_dn2 = assign30100_e29386_d_n2;
        locals.var_xp_dn4 = assign30100_e29386_d_n4;
        locals.var_xp_dn5 = assign30100_e29386_d_n5;
        locals.var_xp_dn6 = assign30100_e29386_d_n6;
        locals.var_xp_dn7 = assign30100_e29386_d_n7;
        locals.var_xp_dn8 = assign30100_e29386_d_n8;
        locals.var_xp_dn9 = assign30100_e29386_d_n9;
        locals.var_xp_dn10 = assign30100_e29386_d_n10;
        locals.var_xp_dn11 = assign30100_e29386_d_n11;
        locals.var_xp_dn14 = assign30100_e29386_d_n14;

        let (assign30110_e29398, assign30110_e29398_d_n0, assign30110_e29398_d_n2, assign30110_e29398_d_n4, assign30110_e29398_d_n5, assign30110_e29398_d_n6, assign30110_e29398_d_n7, assign30110_e29398_d_n8, assign30110_e29398_d_n9, assign30110_e29398_d_n10, assign30110_e29398_d_n11, assign30110_e29398_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30110_e29396: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30110_e29396, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30110_e29398;
        locals.var_xmp_dn0 = assign30110_e29398_d_n0;
        locals.var_xmp_dn2 = assign30110_e29398_d_n2;
        locals.var_xmp_dn4 = assign30110_e29398_d_n4;
        locals.var_xmp_dn5 = assign30110_e29398_d_n5;
        locals.var_xmp_dn6 = assign30110_e29398_d_n6;
        locals.var_xmp_dn7 = assign30110_e29398_d_n7;
        locals.var_xmp_dn8 = assign30110_e29398_d_n8;
        locals.var_xmp_dn9 = assign30110_e29398_d_n9;
        locals.var_xmp_dn10 = assign30110_e29398_d_n10;
        locals.var_xmp_dn11 = assign30110_e29398_d_n11;
        locals.var_xmp_dn14 = assign30110_e29398_d_n14;

        let (assign30120_e29410, assign30120_e29410_d_n0, assign30120_e29410_d_n2, assign30120_e29410_d_n4, assign30120_e29410_d_n5, assign30120_e29410_d_n6, assign30120_e29410_d_n7, assign30120_e29410_d_n8, assign30120_e29410_d_n9, assign30120_e29410_d_n10, assign30120_e29410_d_n11, assign30120_e29410_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30120_e29408: f64 = (locals.var_xp * locals.var_x2);
        (assign30120_e29408, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30120_e29410;
        locals.var_xp_dn0 = assign30120_e29410_d_n0;
        locals.var_xp_dn2 = assign30120_e29410_d_n2;
        locals.var_xp_dn4 = assign30120_e29410_d_n4;
        locals.var_xp_dn5 = assign30120_e29410_d_n5;
        locals.var_xp_dn6 = assign30120_e29410_d_n6;
        locals.var_xp_dn7 = assign30120_e29410_d_n7;
        locals.var_xp_dn8 = assign30120_e29410_d_n8;
        locals.var_xp_dn9 = assign30120_e29410_d_n9;
        locals.var_xp_dn10 = assign30120_e29410_d_n10;
        locals.var_xp_dn11 = assign30120_e29410_d_n11;
        locals.var_xp_dn14 = assign30120_e29410_d_n14;

        let (assign30130_e29422, assign30130_e29422_d_n0, assign30130_e29422_d_n2, assign30130_e29422_d_n4, assign30130_e29422_d_n5, assign30130_e29422_d_n6, assign30130_e29422_d_n7, assign30130_e29422_d_n8, assign30130_e29422_d_n9, assign30130_e29422_d_n10, assign30130_e29422_d_n11, assign30130_e29422_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30130_e29420: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30130_e29420, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30130_e29422;
        locals.var_xmp_dn0 = assign30130_e29422_d_n0;
        locals.var_xmp_dn2 = assign30130_e29422_d_n2;
        locals.var_xmp_dn4 = assign30130_e29422_d_n4;
        locals.var_xmp_dn5 = assign30130_e29422_d_n5;
        locals.var_xmp_dn6 = assign30130_e29422_d_n6;
        locals.var_xmp_dn7 = assign30130_e29422_d_n7;
        locals.var_xmp_dn8 = assign30130_e29422_d_n8;
        locals.var_xmp_dn9 = assign30130_e29422_d_n9;
        locals.var_xmp_dn10 = assign30130_e29422_d_n10;
        locals.var_xmp_dn11 = assign30130_e29422_d_n11;
        locals.var_xmp_dn14 = assign30130_e29422_d_n14;

        let (assign30140_e29434, assign30140_e29434_d_n0, assign30140_e29434_d_n2, assign30140_e29434_d_n4, assign30140_e29434_d_n5, assign30140_e29434_d_n6, assign30140_e29434_d_n7, assign30140_e29434_d_n8, assign30140_e29434_d_n9, assign30140_e29434_d_n10, assign30140_e29434_d_n11, assign30140_e29434_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30140_e29432: f64 = (locals.var_xp * locals.var_x2);
        (assign30140_e29432, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30140_e29434;
        locals.var_xp_dn0 = assign30140_e29434_d_n0;
        locals.var_xp_dn2 = assign30140_e29434_d_n2;
        locals.var_xp_dn4 = assign30140_e29434_d_n4;
        locals.var_xp_dn5 = assign30140_e29434_d_n5;
        locals.var_xp_dn6 = assign30140_e29434_d_n6;
        locals.var_xp_dn7 = assign30140_e29434_d_n7;
        locals.var_xp_dn8 = assign30140_e29434_d_n8;
        locals.var_xp_dn9 = assign30140_e29434_d_n9;
        locals.var_xp_dn10 = assign30140_e29434_d_n10;
        locals.var_xp_dn11 = assign30140_e29434_d_n11;
        locals.var_xp_dn14 = assign30140_e29434_d_n14;

        let (assign30150_e29446, assign30150_e29446_d_n0, assign30150_e29446_d_n2, assign30150_e29446_d_n4, assign30150_e29446_d_n5, assign30150_e29446_d_n6, assign30150_e29446_d_n7, assign30150_e29446_d_n8, assign30150_e29446_d_n9, assign30150_e29446_d_n10, assign30150_e29446_d_n11, assign30150_e29446_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30150_e29444: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30150_e29444, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30150_e29446;
        locals.var_xmp_dn0 = assign30150_e29446_d_n0;
        locals.var_xmp_dn2 = assign30150_e29446_d_n2;
        locals.var_xmp_dn4 = assign30150_e29446_d_n4;
        locals.var_xmp_dn5 = assign30150_e29446_d_n5;
        locals.var_xmp_dn6 = assign30150_e29446_d_n6;
        locals.var_xmp_dn7 = assign30150_e29446_d_n7;
        locals.var_xmp_dn8 = assign30150_e29446_d_n8;
        locals.var_xmp_dn9 = assign30150_e29446_d_n9;
        locals.var_xmp_dn10 = assign30150_e29446_d_n10;
        locals.var_xmp_dn11 = assign30150_e29446_d_n11;
        locals.var_xmp_dn14 = assign30150_e29446_d_n14;

        let (assign30160_e29458, assign30160_e29458_d_n0, assign30160_e29458_d_n2, assign30160_e29458_d_n4, assign30160_e29458_d_n5, assign30160_e29458_d_n6, assign30160_e29458_d_n7, assign30160_e29458_d_n8, assign30160_e29458_d_n9, assign30160_e29458_d_n10, assign30160_e29458_d_n11, assign30160_e29458_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30160_e29456: f64 = (locals.var_xp + locals.var_xmp);
        (assign30160_e29456, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30160_e29458;
        locals.var_arg_dn0 = assign30160_e29458_d_n0;
        locals.var_arg_dn2 = assign30160_e29458_d_n2;
        locals.var_arg_dn4 = assign30160_e29458_d_n4;
        locals.var_arg_dn5 = assign30160_e29458_d_n5;
        locals.var_arg_dn6 = assign30160_e29458_d_n6;
        locals.var_arg_dn7 = assign30160_e29458_d_n7;
        locals.var_arg_dn8 = assign30160_e29458_d_n8;
        locals.var_arg_dn9 = assign30160_e29458_d_n9;
        locals.var_arg_dn10 = assign30160_e29458_d_n10;
        locals.var_arg_dn11 = assign30160_e29458_d_n11;
        locals.var_arg_dn14 = assign30160_e29458_d_n14;

        let (assign30170_e29468, assign30170_e29468_d_n0, assign30170_e29468_d_n2, assign30170_e29468_d_n4, assign30170_e29468_d_n5, assign30170_e29468_d_n6, assign30170_e29468_d_n7, assign30170_e29468_d_n8, assign30170_e29468_d_n9, assign30170_e29468_d_n10, assign30170_e29468_d_n11, assign30170_e29468_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30170_e29468;
        locals.var_dnm_dn0 = assign30170_e29468_d_n0;
        locals.var_dnm_dn2 = assign30170_e29468_d_n2;
        locals.var_dnm_dn4 = assign30170_e29468_d_n4;
        locals.var_dnm_dn5 = assign30170_e29468_d_n5;
        locals.var_dnm_dn6 = assign30170_e29468_d_n6;
        locals.var_dnm_dn7 = assign30170_e29468_d_n7;
        locals.var_dnm_dn8 = assign30170_e29468_d_n8;
        locals.var_dnm_dn9 = assign30170_e29468_d_n9;
        locals.var_dnm_dn10 = assign30170_e29468_d_n10;
        locals.var_dnm_dn11 = assign30170_e29468_d_n11;
        locals.var_dnm_dn14 = assign30170_e29468_d_n14;

        let assign30180_e29483: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard696 = assign30180_e29483;

        let assign30190_e29486: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard697 = assign30190_e29486;

        let (assign30200_e29500,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_guard697 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30200_e29500;

        let assign30210_e29503: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard698 = assign30210_e29503;

        let (assign30220_e29520,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_guard697 == 0.0)) && (locals.var_guard698 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30220_e29520;

        let assign30230_e29523: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard699 = assign30230_e29523;

        let (assign30240_e29543,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_guard697 == 0.0)) && (locals.var_guard698 == 0.0)) && (locals.var_guard699 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30240_e29543;

        let assign30250_e29546: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard700 = assign30250_e29546;

        let (assign30260_e29569,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_guard697 == 0.0)) && (locals.var_guard698 == 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard700 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30260_e29569;

        let (assign30270_e29581,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) && (locals.var_guard696 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30270_e29581;

        let mut assign30280_loop_guard: usize = 0;
        while {
            let assign30280_cond_e29594: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign30280_cond_e29594 != 0.0
        } {
            assign30280_loop_guard += 1;
            assert!(assign30280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign30280_body0_e29607, assign30280_body0_e29607_d_n0, assign30280_body0_e29607_d_n2, assign30280_body0_e29607_d_n4, assign30280_body0_e29607_d_n5, assign30280_body0_e29607_d_n6, assign30280_body0_e29607_d_n7, assign30280_body0_e29607_d_n8, assign30280_body0_e29607_d_n9, assign30280_body0_e29607_d_n10, assign30280_body0_e29607_d_n11, assign30280_body0_e29607_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign30280_body0_e29605: f64 = (locals.var_dnm).sqrt();
        (assign30280_body0_e29605, (locals.var_dnm_dn0 / (2.0 * assign30280_body0_e29605)), (locals.var_dnm_dn2 / (2.0 * assign30280_body0_e29605)), (locals.var_dnm_dn4 / (2.0 * assign30280_body0_e29605)), (locals.var_dnm_dn5 / (2.0 * assign30280_body0_e29605)), (locals.var_dnm_dn6 / (2.0 * assign30280_body0_e29605)), (locals.var_dnm_dn7 / (2.0 * assign30280_body0_e29605)), (locals.var_dnm_dn8 / (2.0 * assign30280_body0_e29605)), (locals.var_dnm_dn9 / (2.0 * assign30280_body0_e29605)), (locals.var_dnm_dn10 / (2.0 * assign30280_body0_e29605)), (locals.var_dnm_dn11 / (2.0 * assign30280_body0_e29605)), (locals.var_dnm_dn14 / (2.0 * assign30280_body0_e29605)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign30280_body0_e29607;
            locals.var_dnm_dn0 = assign30280_body0_e29607_d_n0;
            locals.var_dnm_dn2 = assign30280_body0_e29607_d_n2;
            locals.var_dnm_dn4 = assign30280_body0_e29607_d_n4;
            locals.var_dnm_dn5 = assign30280_body0_e29607_d_n5;
            locals.var_dnm_dn6 = assign30280_body0_e29607_d_n6;
            locals.var_dnm_dn7 = assign30280_body0_e29607_d_n7;
            locals.var_dnm_dn8 = assign30280_body0_e29607_d_n8;
            locals.var_dnm_dn9 = assign30280_body0_e29607_d_n9;
            locals.var_dnm_dn10 = assign30280_body0_e29607_d_n10;
            locals.var_dnm_dn11 = assign30280_body0_e29607_d_n11;
            locals.var_dnm_dn14 = assign30280_body0_e29607_d_n14;
            let (assign30280_body1_e29621,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign30280_body1_e29619: f64 = (locals.var_m0 + 1.0);
        (assign30280_body1_e29619,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign30280_body1_e29621;
        }

        let (assign30290_e29645, assign30290_e29645_d_n0, assign30290_e29645_d_n2, assign30290_e29645_d_n4, assign30290_e29645_d_n5, assign30290_e29645_d_n6, assign30290_e29645_d_n7, assign30290_e29645_d_n8, assign30290_e29645_d_n9, assign30290_e29645_d_n10, assign30290_e29645_d_n11, assign30290_e29645_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) && (locals.var_guard696 == 0.0)) {
        let (assign30290_e29643, assign30290_e29643_d_n0, assign30290_e29643_d_n2, assign30290_e29643_d_n4, assign30290_e29643_d_n5, assign30290_e29643_d_n6, assign30290_e29643_d_n7, assign30290_e29643_d_n8, assign30290_e29643_d_n9, assign30290_e29643_d_n10, assign30290_e29643_d_n11, assign30290_e29643_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign30290_e29640: f64 = (2.0 * 4.0);
                let assign30290_e29641: f64 = (1.0 / assign30290_e29640);
                let assign30290_e29642: f64 = (locals.var_dnm).powf(assign30290_e29641);
                (assign30290_e29642, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn0)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn2)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn4)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn5)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn6)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn7)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn8)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn9)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn10)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn11)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30290_e29641) as f64).is_finite() && ((assign30290_e29641) as f64).fract() == 0.0 { if assign30290_e29641 == 0.0 { 0.0 } else { (assign30290_e29641 * ((locals.var_dnm).powf(assign30290_e29641 - 1.0) * locals.var_dnm_dn14)) } } else { (assign30290_e29642 * (assign30290_e29641 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign30290_e29643, assign30290_e29643_d_n0, assign30290_e29643_d_n2, assign30290_e29643_d_n4, assign30290_e29643_d_n5, assign30290_e29643_d_n6, assign30290_e29643_d_n7, assign30290_e29643_d_n8, assign30290_e29643_d_n9, assign30290_e29643_d_n10, assign30290_e29643_d_n11, assign30290_e29643_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30290_e29645;
        locals.var_dnm_dn0 = assign30290_e29645_d_n0;
        locals.var_dnm_dn2 = assign30290_e29645_d_n2;
        locals.var_dnm_dn4 = assign30290_e29645_d_n4;
        locals.var_dnm_dn5 = assign30290_e29645_d_n5;
        locals.var_dnm_dn6 = assign30290_e29645_d_n6;
        locals.var_dnm_dn7 = assign30290_e29645_d_n7;
        locals.var_dnm_dn8 = assign30290_e29645_d_n8;
        locals.var_dnm_dn9 = assign30290_e29645_d_n9;
        locals.var_dnm_dn10 = assign30290_e29645_d_n10;
        locals.var_dnm_dn11 = assign30290_e29645_d_n11;
        locals.var_dnm_dn14 = assign30290_e29645_d_n14;

        let (assign30300_e29657, assign30300_e29657_d_n0, assign30300_e29657_d_n2, assign30300_e29657_d_n4, assign30300_e29657_d_n5, assign30300_e29657_d_n6, assign30300_e29657_d_n7, assign30300_e29657_d_n8, assign30300_e29657_d_n9, assign30300_e29657_d_n10, assign30300_e29657_d_n11, assign30300_e29657_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30300_e29655: f64 = (1.0 / locals.var_dnm);
        (assign30300_e29655, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30300_e29657;
        locals.var_dnm_dn0 = assign30300_e29657_d_n0;
        locals.var_dnm_dn2 = assign30300_e29657_d_n2;
        locals.var_dnm_dn4 = assign30300_e29657_d_n4;
        locals.var_dnm_dn5 = assign30300_e29657_d_n5;
        locals.var_dnm_dn6 = assign30300_e29657_d_n6;
        locals.var_dnm_dn7 = assign30300_e29657_d_n7;
        locals.var_dnm_dn8 = assign30300_e29657_d_n8;
        locals.var_dnm_dn9 = assign30300_e29657_d_n9;
        locals.var_dnm_dn10 = assign30300_e29657_d_n10;
        locals.var_dnm_dn11 = assign30300_e29657_d_n11;
        locals.var_dnm_dn14 = assign30300_e29657_d_n14;

        let (assign30310_e29671, assign30310_e29671_d_n0, assign30310_e29671_d_n2, assign30310_e29671_d_n4, assign30310_e29671_d_n5, assign30310_e29671_d_n6, assign30310_e29671_d_n7, assign30310_e29671_d_n8, assign30310_e29671_d_n9, assign30310_e29671_d_n10, assign30310_e29671_d_n11, assign30310_e29671_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30310_e29667: f64 = (locals.var_tmf1 * 0.2);
        let assign30310_e29669: f64 = (assign30310_e29667 * locals.var_dnm);
        (assign30310_e29669, (((locals.var_tmf1_dn0 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.2) * locals.var_dnm) + (assign30310_e29667 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign30310_e29671;
        locals.var_tmf0_dn0 = assign30310_e29671_d_n0;
        locals.var_tmf0_dn2 = assign30310_e29671_d_n2;
        locals.var_tmf0_dn4 = assign30310_e29671_d_n4;
        locals.var_tmf0_dn5 = assign30310_e29671_d_n5;
        locals.var_tmf0_dn6 = assign30310_e29671_d_n6;
        locals.var_tmf0_dn7 = assign30310_e29671_d_n7;
        locals.var_tmf0_dn8 = assign30310_e29671_d_n8;
        locals.var_tmf0_dn9 = assign30310_e29671_d_n9;
        locals.var_tmf0_dn10 = assign30310_e29671_d_n10;
        locals.var_tmf0_dn11 = assign30310_e29671_d_n11;
        locals.var_tmf0_dn14 = assign30310_e29671_d_n14;

        let (assign30320_e29687, assign30320_e29687_d_n0, assign30320_e29687_d_n2, assign30320_e29687_d_n4, assign30320_e29687_d_n5, assign30320_e29687_d_n6, assign30320_e29687_d_n7, assign30320_e29687_d_n8, assign30320_e29687_d_n9, assign30320_e29687_d_n10, assign30320_e29687_d_n11, assign30320_e29687_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30320_e29681: f64 = (0.2 * locals.var_xmp);
        let assign30320_e29683: f64 = (assign30320_e29681 * locals.var_dnm);
        let assign30320_e29685: f64 = (assign30320_e29683 / locals.var_arg);
        (assign30320_e29685, ((((((0.2 * locals.var_xmp_dn0) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn0)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn2) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn2)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn4) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn4)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn5) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn5)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn6) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn6)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn7) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn7)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn8) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn8)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn9) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn9)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn10) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn10)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn11) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn11)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn14) * locals.var_dnm) + (assign30320_e29681 * locals.var_dnm_dn14)) * locals.var_arg) - (assign30320_e29683 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30320_e29687;
        locals.var_t0_dn0 = assign30320_e29687_d_n0;
        locals.var_t0_dn2 = assign30320_e29687_d_n2;
        locals.var_t0_dn4 = assign30320_e29687_d_n4;
        locals.var_t0_dn5 = assign30320_e29687_d_n5;
        locals.var_t0_dn6 = assign30320_e29687_d_n6;
        locals.var_t0_dn7 = assign30320_e29687_d_n7;
        locals.var_t0_dn8 = assign30320_e29687_d_n8;
        locals.var_t0_dn9 = assign30320_e29687_d_n9;
        locals.var_t0_dn10 = assign30320_e29687_d_n10;
        locals.var_t0_dn11 = assign30320_e29687_d_n11;
        locals.var_t0_dn14 = assign30320_e29687_d_n14;

        let (assign30330_e29701, assign30330_e29701_d_n0, assign30330_e29701_d_n2, assign30330_e29701_d_n4, assign30330_e29701_d_n5, assign30330_e29701_d_n6, assign30330_e29701_d_n7, assign30330_e29701_d_n8, assign30330_e29701_d_n9, assign30330_e29701_d_n10, assign30330_e29701_d_n11, assign30330_e29701_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        let assign30330_e29697: f64 = (0.3 + 0.2);
        let assign30330_e29699: f64 = (assign30330_e29697 - locals.var_tmf0);
        (assign30330_e29699, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign30330_e29701;
        locals.var_t10_dn0 = assign30330_e29701_d_n0;
        locals.var_t10_dn2 = assign30330_e29701_d_n2;
        locals.var_t10_dn4 = assign30330_e29701_d_n4;
        locals.var_t10_dn5 = assign30330_e29701_d_n5;
        locals.var_t10_dn6 = assign30330_e29701_d_n6;
        locals.var_t10_dn7 = assign30330_e29701_d_n7;
        locals.var_t10_dn8 = assign30330_e29701_d_n8;
        locals.var_t10_dn9 = assign30330_e29701_d_n9;
        locals.var_t10_dn10 = assign30330_e29701_d_n10;
        locals.var_t10_dn11 = assign30330_e29701_d_n11;
        locals.var_t10_dn14 = assign30330_e29701_d_n14;

        let (assign30340_e29711, assign30340_e29711_d_n0, assign30340_e29711_d_n2, assign30340_e29711_d_n4, assign30340_e29711_d_n5, assign30340_e29711_d_n6, assign30340_e29711_d_n7, assign30340_e29711_d_n8, assign30340_e29711_d_n9, assign30340_e29711_d_n10, assign30340_e29711_d_n11, assign30340_e29711_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30340_e29711;
        locals.var_t0_dn0 = assign30340_e29711_d_n0;
        locals.var_t0_dn2 = assign30340_e29711_d_n2;
        locals.var_t0_dn4 = assign30340_e29711_d_n4;
        locals.var_t0_dn5 = assign30340_e29711_d_n5;
        locals.var_t0_dn6 = assign30340_e29711_d_n6;
        locals.var_t0_dn7 = assign30340_e29711_d_n7;
        locals.var_t0_dn8 = assign30340_e29711_d_n8;
        locals.var_t0_dn9 = assign30340_e29711_d_n9;
        locals.var_t0_dn10 = assign30340_e29711_d_n10;
        locals.var_t0_dn11 = assign30340_e29711_d_n11;
        locals.var_t0_dn14 = assign30340_e29711_d_n14;

        let (assign30350_e29722, assign30350_e29722_d_n0, assign30350_e29722_d_n2, assign30350_e29722_d_n4, assign30350_e29722_d_n5, assign30350_e29722_d_n6, assign30350_e29722_d_n7, assign30350_e29722_d_n8, assign30350_e29722_d_n9, assign30350_e29722_d_n10, assign30350_e29722_d_n11, assign30350_e29722_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign30350_e29722;
        locals.var_t10_dn0 = assign30350_e29722_d_n0;
        locals.var_t10_dn2 = assign30350_e29722_d_n2;
        locals.var_t10_dn4 = assign30350_e29722_d_n4;
        locals.var_t10_dn5 = assign30350_e29722_d_n5;
        locals.var_t10_dn6 = assign30350_e29722_d_n6;
        locals.var_t10_dn7 = assign30350_e29722_d_n7;
        locals.var_t10_dn8 = assign30350_e29722_d_n8;
        locals.var_t10_dn9 = assign30350_e29722_d_n9;
        locals.var_t10_dn10 = assign30350_e29722_d_n10;
        locals.var_t10_dn11 = assign30350_e29722_d_n11;
        locals.var_t10_dn14 = assign30350_e29722_d_n14;

        let (assign30360_e29733, assign30360_e29733_d_n0, assign30360_e29733_d_n2, assign30360_e29733_d_n4, assign30360_e29733_d_n5, assign30360_e29733_d_n6, assign30360_e29733_d_n7, assign30360_e29733_d_n8, assign30360_e29733_d_n9, assign30360_e29733_d_n10, assign30360_e29733_d_n11, assign30360_e29733_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard695 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30360_e29733;
        locals.var_t0_dn0 = assign30360_e29733_d_n0;
        locals.var_t0_dn2 = assign30360_e29733_d_n2;
        locals.var_t0_dn4 = assign30360_e29733_d_n4;
        locals.var_t0_dn5 = assign30360_e29733_d_n5;
        locals.var_t0_dn6 = assign30360_e29733_d_n6;
        locals.var_t0_dn7 = assign30360_e29733_d_n7;
        locals.var_t0_dn8 = assign30360_e29733_d_n8;
        locals.var_t0_dn9 = assign30360_e29733_d_n9;
        locals.var_t0_dn10 = assign30360_e29733_d_n10;
        locals.var_t0_dn11 = assign30360_e29733_d_n11;
        locals.var_t0_dn14 = assign30360_e29733_d_n14;

        let (assign30370_e29745, assign30370_e29745_d_n0, assign30370_e29745_d_n2, assign30370_e29745_d_n4, assign30370_e29745_d_n5, assign30370_e29745_d_n6, assign30370_e29745_d_n7, assign30370_e29745_d_n8, assign30370_e29745_d_n9, assign30370_e29745_d_n10, assign30370_e29745_d_n11, assign30370_e29745_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign30370_e29742: f64 = (10.0 * 2.220446049250313e-16);
        let assign30370_e29743: f64 = (locals.var_t10 + assign30370_e29742);
        (assign30370_e29743, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign30370_e29745;
        locals.var_t10_dn0 = assign30370_e29745_d_n0;
        locals.var_t10_dn2 = assign30370_e29745_d_n2;
        locals.var_t10_dn4 = assign30370_e29745_d_n4;
        locals.var_t10_dn5 = assign30370_e29745_d_n5;
        locals.var_t10_dn6 = assign30370_e29745_d_n6;
        locals.var_t10_dn7 = assign30370_e29745_d_n7;
        locals.var_t10_dn8 = assign30370_e29745_d_n8;
        locals.var_t10_dn9 = assign30370_e29745_d_n9;
        locals.var_t10_dn10 = assign30370_e29745_d_n10;
        locals.var_t10_dn11 = assign30370_e29745_d_n11;
        locals.var_t10_dn14 = assign30370_e29745_d_n14;

        let (assign30380_e29755, assign30380_e29755_d_n0, assign30380_e29755_d_n2, assign30380_e29755_d_n4, assign30380_e29755_d_n5, assign30380_e29755_d_n6, assign30380_e29755_d_n7, assign30380_e29755_d_n8, assign30380_e29755_d_n9, assign30380_e29755_d_n10, assign30380_e29755_d_n11, assign30380_e29755_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign30380_e29753: f64 = (locals.var_vds / locals.var_t10);
        (assign30380_e29753, (((locals.var_vds_dn0 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn2 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn4 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn5 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn6 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn7 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn8 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn9 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn10 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn11 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn14 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign30380_e29755;
        locals.var_t1_dn0 = assign30380_e29755_d_n0;
        locals.var_t1_dn2 = assign30380_e29755_d_n2;
        locals.var_t1_dn4 = assign30380_e29755_d_n4;
        locals.var_t1_dn5 = assign30380_e29755_d_n5;
        locals.var_t1_dn6 = assign30380_e29755_d_n6;
        locals.var_t1_dn7 = assign30380_e29755_d_n7;
        locals.var_t1_dn8 = assign30380_e29755_d_n8;
        locals.var_t1_dn9 = assign30380_e29755_d_n9;
        locals.var_t1_dn10 = assign30380_e29755_d_n10;
        locals.var_t1_dn11 = assign30380_e29755_d_n11;
        locals.var_t1_dn14 = assign30380_e29755_d_n14;

    }

    pub(super) fn stamp_transient_block_88(
        locals: &mut StampLocals,
    ) {
        let (assign30390_e29772, assign30390_e29772_d_n0, assign30390_e29772_d_n2, assign30390_e29772_d_n4, assign30390_e29772_d_n5, assign30390_e29772_d_n6, assign30390_e29772_d_n7, assign30390_e29772_d_n8, assign30390_e29772_d_n9, assign30390_e29772_d_n10, assign30390_e29772_d_n11, assign30390_e29772_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let (assign30390_e29770, assign30390_e29770_d_n0, assign30390_e29770_d_n2, assign30390_e29770_d_n4, assign30390_e29770_d_n5, assign30390_e29770_d_n6, assign30390_e29770_d_n7, assign30390_e29770_d_n8, assign30390_e29770_d_n9, assign30390_e29770_d_n10, assign30390_e29770_d_n11, assign30390_e29770_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign30390_e29768: f64 = (locals.var_ddlte - 1.0);
                let assign30390_e29769: f64 = (locals.var_t1).powf(assign30390_e29768);
                (assign30390_e29769, if locals.var_ddlte_dn0 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn0)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn0 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_ddlte_dn2 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn2)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn2 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_ddlte_dn4 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn4)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn4 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_ddlte_dn5 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn5)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn5 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_ddlte_dn6 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn6)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn6 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_ddlte_dn7 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn7)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn7 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_ddlte_dn8 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn8)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn8 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_ddlte_dn9 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn9)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn9 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_ddlte_dn10 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn10)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn10 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_ddlte_dn11 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn11)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn11 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_ddlte_dn14 == 0.0 && ((assign30390_e29768) as f64).is_finite() && ((assign30390_e29768) as f64).fract() == 0.0 { if assign30390_e29768 == 0.0 { 0.0 } else { (assign30390_e29768 * ((locals.var_t1).powf(assign30390_e29768 - 1.0) * locals.var_t1_dn14)) } } else { (assign30390_e29769 * ((locals.var_ddlte_dn14 * (locals.var_t1).ln()) + (assign30390_e29768 * (locals.var_t1_dn14 / locals.var_t1)))) },)
            }
        };
        (assign30390_e29770, assign30390_e29770_d_n0, assign30390_e29770_d_n2, assign30390_e29770_d_n4, assign30390_e29770_d_n5, assign30390_e29770_d_n6, assign30390_e29770_d_n7, assign30390_e29770_d_n8, assign30390_e29770_d_n9, assign30390_e29770_d_n10, assign30390_e29770_d_n11, assign30390_e29770_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign30390_e29772;
        locals.var_t2_dn0 = assign30390_e29772_d_n0;
        locals.var_t2_dn2 = assign30390_e29772_d_n2;
        locals.var_t2_dn4 = assign30390_e29772_d_n4;
        locals.var_t2_dn5 = assign30390_e29772_d_n5;
        locals.var_t2_dn6 = assign30390_e29772_d_n6;
        locals.var_t2_dn7 = assign30390_e29772_d_n7;
        locals.var_t2_dn8 = assign30390_e29772_d_n8;
        locals.var_t2_dn9 = assign30390_e29772_d_n9;
        locals.var_t2_dn10 = assign30390_e29772_d_n10;
        locals.var_t2_dn11 = assign30390_e29772_d_n11;
        locals.var_t2_dn14 = assign30390_e29772_d_n14;

        let (assign30400_e29782, assign30400_e29782_d_n0, assign30400_e29782_d_n2, assign30400_e29782_d_n4, assign30400_e29782_d_n5, assign30400_e29782_d_n6, assign30400_e29782_d_n7, assign30400_e29782_d_n8, assign30400_e29782_d_n9, assign30400_e29782_d_n10, assign30400_e29782_d_n11, assign30400_e29782_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign30400_e29780: f64 = (locals.var_t2 * locals.var_t1);
        (assign30400_e29780, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign30400_e29782;
        locals.var_t7_dn0 = assign30400_e29782_d_n0;
        locals.var_t7_dn2 = assign30400_e29782_d_n2;
        locals.var_t7_dn4 = assign30400_e29782_d_n4;
        locals.var_t7_dn5 = assign30400_e29782_d_n5;
        locals.var_t7_dn6 = assign30400_e29782_d_n6;
        locals.var_t7_dn7 = assign30400_e29782_d_n7;
        locals.var_t7_dn8 = assign30400_e29782_d_n8;
        locals.var_t7_dn9 = assign30400_e29782_d_n9;
        locals.var_t7_dn10 = assign30400_e29782_d_n10;
        locals.var_t7_dn11 = assign30400_e29782_d_n11;
        locals.var_t7_dn14 = assign30400_e29782_d_n14;

        let (assign30410_e29792, assign30410_e29792_d_n0, assign30410_e29792_d_n2, assign30410_e29792_d_n4, assign30410_e29792_d_n5, assign30410_e29792_d_n6, assign30410_e29792_d_n7, assign30410_e29792_d_n8, assign30410_e29792_d_n9, assign30410_e29792_d_n10, assign30410_e29792_d_n11, assign30410_e29792_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign30410_e29790: f64 = (1.0 + locals.var_t7);
        (assign30410_e29790, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign30410_e29792;
        locals.var_t3_dn0 = assign30410_e29792_d_n0;
        locals.var_t3_dn2 = assign30410_e29792_d_n2;
        locals.var_t3_dn4 = assign30410_e29792_d_n4;
        locals.var_t3_dn5 = assign30410_e29792_d_n5;
        locals.var_t3_dn6 = assign30410_e29792_d_n6;
        locals.var_t3_dn7 = assign30410_e29792_d_n7;
        locals.var_t3_dn8 = assign30410_e29792_d_n8;
        locals.var_t3_dn9 = assign30410_e29792_d_n9;
        locals.var_t3_dn10 = assign30410_e29792_d_n10;
        locals.var_t3_dn11 = assign30410_e29792_d_n11;
        locals.var_t3_dn14 = assign30410_e29792_d_n14;

        let (assign30420_e29811, assign30420_e29811_d_n0, assign30420_e29811_d_n2, assign30420_e29811_d_n4, assign30420_e29811_d_n5, assign30420_e29811_d_n6, assign30420_e29811_d_n7, assign30420_e29811_d_n8, assign30420_e29811_d_n9, assign30420_e29811_d_n10, assign30420_e29811_d_n11, assign30420_e29811_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let (assign30420_e29809, assign30420_e29809_d_n0, assign30420_e29809_d_n2, assign30420_e29809_d_n4, assign30420_e29809_d_n5, assign30420_e29809_d_n6, assign30420_e29809_d_n7, assign30420_e29809_d_n8, assign30420_e29809_d_n9, assign30420_e29809_d_n10, assign30420_e29809_d_n11, assign30420_e29809_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign30420_e29805: f64 = (1.0 / locals.var_ddlte);
                let assign30420_e29807: f64 = (assign30420_e29805 - 1.0);
                let assign30420_e29808: f64 = (locals.var_t3).powf(assign30420_e29807);
                (assign30420_e29808, if (-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn0)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn0 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn2)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn2 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn4)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn4 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn5)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn5 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn6)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn6 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn7)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn7 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn8)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn8 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn9)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn9 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn10)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn10 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn11)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn11 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign30420_e29807) as f64).is_finite() && ((assign30420_e29807) as f64).fract() == 0.0 { if assign30420_e29807 == 0.0 { 0.0 } else { (assign30420_e29807 * ((locals.var_t3).powf(assign30420_e29807 - 1.0) * locals.var_t3_dn14)) } } else { (assign30420_e29808 * (((-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign30420_e29807 * (locals.var_t3_dn14 / locals.var_t3)))) },)
            }
        };
        (assign30420_e29809, assign30420_e29809_d_n0, assign30420_e29809_d_n2, assign30420_e29809_d_n4, assign30420_e29809_d_n5, assign30420_e29809_d_n6, assign30420_e29809_d_n7, assign30420_e29809_d_n8, assign30420_e29809_d_n9, assign30420_e29809_d_n10, assign30420_e29809_d_n11, assign30420_e29809_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign30420_e29811;
        locals.var_t4_dn0 = assign30420_e29811_d_n0;
        locals.var_t4_dn2 = assign30420_e29811_d_n2;
        locals.var_t4_dn4 = assign30420_e29811_d_n4;
        locals.var_t4_dn5 = assign30420_e29811_d_n5;
        locals.var_t4_dn6 = assign30420_e29811_d_n6;
        locals.var_t4_dn7 = assign30420_e29811_d_n7;
        locals.var_t4_dn8 = assign30420_e29811_d_n8;
        locals.var_t4_dn9 = assign30420_e29811_d_n9;
        locals.var_t4_dn10 = assign30420_e29811_d_n10;
        locals.var_t4_dn11 = assign30420_e29811_d_n11;
        locals.var_t4_dn14 = assign30420_e29811_d_n14;

        let (assign30430_e29821, assign30430_e29821_d_n0, assign30430_e29821_d_n2, assign30430_e29821_d_n4, assign30430_e29821_d_n5, assign30430_e29821_d_n6, assign30430_e29821_d_n7, assign30430_e29821_d_n8, assign30430_e29821_d_n9, assign30430_e29821_d_n10, assign30430_e29821_d_n11, assign30430_e29821_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign30430_e29819: f64 = (locals.var_t4 * locals.var_t3);
        (assign30430_e29819, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign30430_e29821;
        locals.var_t6_dn0 = assign30430_e29821_d_n0;
        locals.var_t6_dn2 = assign30430_e29821_d_n2;
        locals.var_t6_dn4 = assign30430_e29821_d_n4;
        locals.var_t6_dn5 = assign30430_e29821_d_n5;
        locals.var_t6_dn6 = assign30430_e29821_d_n6;
        locals.var_t6_dn7 = assign30430_e29821_d_n7;
        locals.var_t6_dn8 = assign30430_e29821_d_n8;
        locals.var_t6_dn9 = assign30430_e29821_d_n9;
        locals.var_t6_dn10 = assign30430_e29821_d_n10;
        locals.var_t6_dn11 = assign30430_e29821_d_n11;
        locals.var_t6_dn14 = assign30430_e29821_d_n14;

        let (assign30440_e29831, assign30440_e29831_d_n0, assign30440_e29831_d_n2, assign30440_e29831_d_n4, assign30440_e29831_d_n5, assign30440_e29831_d_n6, assign30440_e29831_d_n7, assign30440_e29831_d_n8, assign30440_e29831_d_n9, assign30440_e29831_d_n10, assign30440_e29831_d_n11, assign30440_e29831_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign30440_e29829: f64 = (locals.var_vds / locals.var_t6);
        (assign30440_e29829, (((locals.var_vds_dn0 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn2 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn4 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn5 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn6 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn7 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn8 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn9 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn10 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn11 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn14 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn14,)
    }
};
        locals.var_vdseff = assign30440_e29831;
        locals.var_vdseff_dn0 = assign30440_e29831_d_n0;
        locals.var_vdseff_dn2 = assign30440_e29831_d_n2;
        locals.var_vdseff_dn4 = assign30440_e29831_d_n4;
        locals.var_vdseff_dn5 = assign30440_e29831_d_n5;
        locals.var_vdseff_dn6 = assign30440_e29831_d_n6;
        locals.var_vdseff_dn7 = assign30440_e29831_d_n7;
        locals.var_vdseff_dn8 = assign30440_e29831_d_n8;
        locals.var_vdseff_dn9 = assign30440_e29831_d_n9;
        locals.var_vdseff_dn10 = assign30440_e29831_d_n10;
        locals.var_vdseff_dn11 = assign30440_e29831_d_n11;
        locals.var_vdseff_dn14 = assign30440_e29831_d_n14;

        let assign30450_e29835: f64 = 0.5;
        let assign30450_e29840: f64 = if ((locals.var_vgp < assign30450_e29835) && (0.5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard701 = assign30450_e29840;

        let (assign30460_e29854, assign30460_e29854_d_n0, assign30460_e29854_d_n2, assign30460_e29854_d_n4, assign30460_e29854_d_n5, assign30460_e29854_d_n6, assign30460_e29854_d_n7, assign30460_e29854_d_n8, assign30460_e29854_d_n9, assign30460_e29854_d_n10, assign30460_e29854_d_n11, assign30460_e29854_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30460_e29850: f64 = 0.5;
        let assign30460_e29852: f64 = (assign30460_e29850 - locals.var_vgp);
        (assign30460_e29852, (-locals.var_vgp_dn0), (-locals.var_vgp_dn2), (-locals.var_vgp_dn4), (-locals.var_vgp_dn5), (-locals.var_vgp_dn6), (-locals.var_vgp_dn7), (-locals.var_vgp_dn8), (-locals.var_vgp_dn9), (-locals.var_vgp_dn10), (-locals.var_vgp_dn11), (-locals.var_vgp_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign30460_e29854;
        locals.var_tmf1_dn0 = assign30460_e29854_d_n0;
        locals.var_tmf1_dn2 = assign30460_e29854_d_n2;
        locals.var_tmf1_dn4 = assign30460_e29854_d_n4;
        locals.var_tmf1_dn5 = assign30460_e29854_d_n5;
        locals.var_tmf1_dn6 = assign30460_e29854_d_n6;
        locals.var_tmf1_dn7 = assign30460_e29854_d_n7;
        locals.var_tmf1_dn8 = assign30460_e29854_d_n8;
        locals.var_tmf1_dn9 = assign30460_e29854_d_n9;
        locals.var_tmf1_dn10 = assign30460_e29854_d_n10;
        locals.var_tmf1_dn11 = assign30460_e29854_d_n11;
        locals.var_tmf1_dn14 = assign30460_e29854_d_n14;

        let (assign30470_e29866, assign30470_e29866_d_n0, assign30470_e29866_d_n2, assign30470_e29866_d_n4, assign30470_e29866_d_n5, assign30470_e29866_d_n6, assign30470_e29866_d_n7, assign30470_e29866_d_n8, assign30470_e29866_d_n9, assign30470_e29866_d_n10, assign30470_e29866_d_n11, assign30470_e29866_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30470_e29864: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign30470_e29864, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign30470_e29866;
        locals.var_x2_dn0 = assign30470_e29866_d_n0;
        locals.var_x2_dn2 = assign30470_e29866_d_n2;
        locals.var_x2_dn4 = assign30470_e29866_d_n4;
        locals.var_x2_dn5 = assign30470_e29866_d_n5;
        locals.var_x2_dn6 = assign30470_e29866_d_n6;
        locals.var_x2_dn7 = assign30470_e29866_d_n7;
        locals.var_x2_dn8 = assign30470_e29866_d_n8;
        locals.var_x2_dn9 = assign30470_e29866_d_n9;
        locals.var_x2_dn10 = assign30470_e29866_d_n10;
        locals.var_x2_dn11 = assign30470_e29866_d_n11;
        locals.var_x2_dn14 = assign30470_e29866_d_n14;

        let (assign30480_e29878, assign30480_e29878_d_n0, assign30480_e29878_d_n2, assign30480_e29878_d_n4, assign30480_e29878_d_n5, assign30480_e29878_d_n6, assign30480_e29878_d_n7, assign30480_e29878_d_n8, assign30480_e29878_d_n9, assign30480_e29878_d_n10, assign30480_e29878_d_n11, assign30480_e29878_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30480_e29876: f64 = (0.5 * 0.5);
        (assign30480_e29876, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign30480_e29878;
        locals.var_xmax2_dn0 = assign30480_e29878_d_n0;
        locals.var_xmax2_dn2 = assign30480_e29878_d_n2;
        locals.var_xmax2_dn4 = assign30480_e29878_d_n4;
        locals.var_xmax2_dn5 = assign30480_e29878_d_n5;
        locals.var_xmax2_dn6 = assign30480_e29878_d_n6;
        locals.var_xmax2_dn7 = assign30480_e29878_d_n7;
        locals.var_xmax2_dn8 = assign30480_e29878_d_n8;
        locals.var_xmax2_dn9 = assign30480_e29878_d_n9;
        locals.var_xmax2_dn10 = assign30480_e29878_d_n10;
        locals.var_xmax2_dn11 = assign30480_e29878_d_n11;
        locals.var_xmax2_dn14 = assign30480_e29878_d_n14;

        let (assign30490_e29888, assign30490_e29888_d_n0, assign30490_e29888_d_n2, assign30490_e29888_d_n4, assign30490_e29888_d_n5, assign30490_e29888_d_n6, assign30490_e29888_d_n7, assign30490_e29888_d_n8, assign30490_e29888_d_n9, assign30490_e29888_d_n10, assign30490_e29888_d_n11, assign30490_e29888_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30490_e29888;
        locals.var_xp_dn0 = assign30490_e29888_d_n0;
        locals.var_xp_dn2 = assign30490_e29888_d_n2;
        locals.var_xp_dn4 = assign30490_e29888_d_n4;
        locals.var_xp_dn5 = assign30490_e29888_d_n5;
        locals.var_xp_dn6 = assign30490_e29888_d_n6;
        locals.var_xp_dn7 = assign30490_e29888_d_n7;
        locals.var_xp_dn8 = assign30490_e29888_d_n8;
        locals.var_xp_dn9 = assign30490_e29888_d_n9;
        locals.var_xp_dn10 = assign30490_e29888_d_n10;
        locals.var_xp_dn11 = assign30490_e29888_d_n11;
        locals.var_xp_dn14 = assign30490_e29888_d_n14;

        let (assign30500_e29898, assign30500_e29898_d_n0, assign30500_e29898_d_n2, assign30500_e29898_d_n4, assign30500_e29898_d_n5, assign30500_e29898_d_n6, assign30500_e29898_d_n7, assign30500_e29898_d_n8, assign30500_e29898_d_n9, assign30500_e29898_d_n10, assign30500_e29898_d_n11, assign30500_e29898_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30500_e29898;
        locals.var_xmp_dn0 = assign30500_e29898_d_n0;
        locals.var_xmp_dn2 = assign30500_e29898_d_n2;
        locals.var_xmp_dn4 = assign30500_e29898_d_n4;
        locals.var_xmp_dn5 = assign30500_e29898_d_n5;
        locals.var_xmp_dn6 = assign30500_e29898_d_n6;
        locals.var_xmp_dn7 = assign30500_e29898_d_n7;
        locals.var_xmp_dn8 = assign30500_e29898_d_n8;
        locals.var_xmp_dn9 = assign30500_e29898_d_n9;
        locals.var_xmp_dn10 = assign30500_e29898_d_n10;
        locals.var_xmp_dn11 = assign30500_e29898_d_n11;
        locals.var_xmp_dn14 = assign30500_e29898_d_n14;

        let (assign30510_e29908,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30510_e29908;

        let (assign30520_e29918,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30520_e29918;

        let (assign30530_e29928, assign30530_e29928_d_n0, assign30530_e29928_d_n2, assign30530_e29928_d_n4, assign30530_e29928_d_n5, assign30530_e29928_d_n6, assign30530_e29928_d_n7, assign30530_e29928_d_n8, assign30530_e29928_d_n9, assign30530_e29928_d_n10, assign30530_e29928_d_n11, assign30530_e29928_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30530_e29928;
        locals.var_arg_dn0 = assign30530_e29928_d_n0;
        locals.var_arg_dn2 = assign30530_e29928_d_n2;
        locals.var_arg_dn4 = assign30530_e29928_d_n4;
        locals.var_arg_dn5 = assign30530_e29928_d_n5;
        locals.var_arg_dn6 = assign30530_e29928_d_n6;
        locals.var_arg_dn7 = assign30530_e29928_d_n7;
        locals.var_arg_dn8 = assign30530_e29928_d_n8;
        locals.var_arg_dn9 = assign30530_e29928_d_n9;
        locals.var_arg_dn10 = assign30530_e29928_d_n10;
        locals.var_arg_dn11 = assign30530_e29928_d_n11;
        locals.var_arg_dn14 = assign30530_e29928_d_n14;

        let (assign30540_e29938, assign30540_e29938_d_n0, assign30540_e29938_d_n2, assign30540_e29938_d_n4, assign30540_e29938_d_n5, assign30540_e29938_d_n6, assign30540_e29938_d_n7, assign30540_e29938_d_n8, assign30540_e29938_d_n9, assign30540_e29938_d_n10, assign30540_e29938_d_n11, assign30540_e29938_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30540_e29938;
        locals.var_dnm_dn0 = assign30540_e29938_d_n0;
        locals.var_dnm_dn2 = assign30540_e29938_d_n2;
        locals.var_dnm_dn4 = assign30540_e29938_d_n4;
        locals.var_dnm_dn5 = assign30540_e29938_d_n5;
        locals.var_dnm_dn6 = assign30540_e29938_d_n6;
        locals.var_dnm_dn7 = assign30540_e29938_d_n7;
        locals.var_dnm_dn8 = assign30540_e29938_d_n8;
        locals.var_dnm_dn9 = assign30540_e29938_d_n9;
        locals.var_dnm_dn10 = assign30540_e29938_d_n10;
        locals.var_dnm_dn11 = assign30540_e29938_d_n11;
        locals.var_dnm_dn14 = assign30540_e29938_d_n14;

        let (assign30550_e29950, assign30550_e29950_d_n0, assign30550_e29950_d_n2, assign30550_e29950_d_n4, assign30550_e29950_d_n5, assign30550_e29950_d_n6, assign30550_e29950_d_n7, assign30550_e29950_d_n8, assign30550_e29950_d_n9, assign30550_e29950_d_n10, assign30550_e29950_d_n11, assign30550_e29950_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30550_e29948: f64 = (locals.var_xp * locals.var_x2);
        (assign30550_e29948, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30550_e29950;
        locals.var_xp_dn0 = assign30550_e29950_d_n0;
        locals.var_xp_dn2 = assign30550_e29950_d_n2;
        locals.var_xp_dn4 = assign30550_e29950_d_n4;
        locals.var_xp_dn5 = assign30550_e29950_d_n5;
        locals.var_xp_dn6 = assign30550_e29950_d_n6;
        locals.var_xp_dn7 = assign30550_e29950_d_n7;
        locals.var_xp_dn8 = assign30550_e29950_d_n8;
        locals.var_xp_dn9 = assign30550_e29950_d_n9;
        locals.var_xp_dn10 = assign30550_e29950_d_n10;
        locals.var_xp_dn11 = assign30550_e29950_d_n11;
        locals.var_xp_dn14 = assign30550_e29950_d_n14;

        let (assign30560_e29962, assign30560_e29962_d_n0, assign30560_e29962_d_n2, assign30560_e29962_d_n4, assign30560_e29962_d_n5, assign30560_e29962_d_n6, assign30560_e29962_d_n7, assign30560_e29962_d_n8, assign30560_e29962_d_n9, assign30560_e29962_d_n10, assign30560_e29962_d_n11, assign30560_e29962_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30560_e29960: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30560_e29960, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30560_e29962;
        locals.var_xmp_dn0 = assign30560_e29962_d_n0;
        locals.var_xmp_dn2 = assign30560_e29962_d_n2;
        locals.var_xmp_dn4 = assign30560_e29962_d_n4;
        locals.var_xmp_dn5 = assign30560_e29962_d_n5;
        locals.var_xmp_dn6 = assign30560_e29962_d_n6;
        locals.var_xmp_dn7 = assign30560_e29962_d_n7;
        locals.var_xmp_dn8 = assign30560_e29962_d_n8;
        locals.var_xmp_dn9 = assign30560_e29962_d_n9;
        locals.var_xmp_dn10 = assign30560_e29962_d_n10;
        locals.var_xmp_dn11 = assign30560_e29962_d_n11;
        locals.var_xmp_dn14 = assign30560_e29962_d_n14;

        let (assign30570_e29974, assign30570_e29974_d_n0, assign30570_e29974_d_n2, assign30570_e29974_d_n4, assign30570_e29974_d_n5, assign30570_e29974_d_n6, assign30570_e29974_d_n7, assign30570_e29974_d_n8, assign30570_e29974_d_n9, assign30570_e29974_d_n10, assign30570_e29974_d_n11, assign30570_e29974_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30570_e29972: f64 = (locals.var_xp * locals.var_x2);
        (assign30570_e29972, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30570_e29974;
        locals.var_xp_dn0 = assign30570_e29974_d_n0;
        locals.var_xp_dn2 = assign30570_e29974_d_n2;
        locals.var_xp_dn4 = assign30570_e29974_d_n4;
        locals.var_xp_dn5 = assign30570_e29974_d_n5;
        locals.var_xp_dn6 = assign30570_e29974_d_n6;
        locals.var_xp_dn7 = assign30570_e29974_d_n7;
        locals.var_xp_dn8 = assign30570_e29974_d_n8;
        locals.var_xp_dn9 = assign30570_e29974_d_n9;
        locals.var_xp_dn10 = assign30570_e29974_d_n10;
        locals.var_xp_dn11 = assign30570_e29974_d_n11;
        locals.var_xp_dn14 = assign30570_e29974_d_n14;

        let (assign30580_e29986, assign30580_e29986_d_n0, assign30580_e29986_d_n2, assign30580_e29986_d_n4, assign30580_e29986_d_n5, assign30580_e29986_d_n6, assign30580_e29986_d_n7, assign30580_e29986_d_n8, assign30580_e29986_d_n9, assign30580_e29986_d_n10, assign30580_e29986_d_n11, assign30580_e29986_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30580_e29984: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30580_e29984, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30580_e29986;
        locals.var_xmp_dn0 = assign30580_e29986_d_n0;
        locals.var_xmp_dn2 = assign30580_e29986_d_n2;
        locals.var_xmp_dn4 = assign30580_e29986_d_n4;
        locals.var_xmp_dn5 = assign30580_e29986_d_n5;
        locals.var_xmp_dn6 = assign30580_e29986_d_n6;
        locals.var_xmp_dn7 = assign30580_e29986_d_n7;
        locals.var_xmp_dn8 = assign30580_e29986_d_n8;
        locals.var_xmp_dn9 = assign30580_e29986_d_n9;
        locals.var_xmp_dn10 = assign30580_e29986_d_n10;
        locals.var_xmp_dn11 = assign30580_e29986_d_n11;
        locals.var_xmp_dn14 = assign30580_e29986_d_n14;

        let (assign30590_e29998, assign30590_e29998_d_n0, assign30590_e29998_d_n2, assign30590_e29998_d_n4, assign30590_e29998_d_n5, assign30590_e29998_d_n6, assign30590_e29998_d_n7, assign30590_e29998_d_n8, assign30590_e29998_d_n9, assign30590_e29998_d_n10, assign30590_e29998_d_n11, assign30590_e29998_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30590_e29996: f64 = (locals.var_xp + locals.var_xmp);
        (assign30590_e29996, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30590_e29998;
        locals.var_arg_dn0 = assign30590_e29998_d_n0;
        locals.var_arg_dn2 = assign30590_e29998_d_n2;
        locals.var_arg_dn4 = assign30590_e29998_d_n4;
        locals.var_arg_dn5 = assign30590_e29998_d_n5;
        locals.var_arg_dn6 = assign30590_e29998_d_n6;
        locals.var_arg_dn7 = assign30590_e29998_d_n7;
        locals.var_arg_dn8 = assign30590_e29998_d_n8;
        locals.var_arg_dn9 = assign30590_e29998_d_n9;
        locals.var_arg_dn10 = assign30590_e29998_d_n10;
        locals.var_arg_dn11 = assign30590_e29998_d_n11;
        locals.var_arg_dn14 = assign30590_e29998_d_n14;

        let (assign30600_e30008, assign30600_e30008_d_n0, assign30600_e30008_d_n2, assign30600_e30008_d_n4, assign30600_e30008_d_n5, assign30600_e30008_d_n6, assign30600_e30008_d_n7, assign30600_e30008_d_n8, assign30600_e30008_d_n9, assign30600_e30008_d_n10, assign30600_e30008_d_n11, assign30600_e30008_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30600_e30008;
        locals.var_dnm_dn0 = assign30600_e30008_d_n0;
        locals.var_dnm_dn2 = assign30600_e30008_d_n2;
        locals.var_dnm_dn4 = assign30600_e30008_d_n4;
        locals.var_dnm_dn5 = assign30600_e30008_d_n5;
        locals.var_dnm_dn6 = assign30600_e30008_d_n6;
        locals.var_dnm_dn7 = assign30600_e30008_d_n7;
        locals.var_dnm_dn8 = assign30600_e30008_d_n8;
        locals.var_dnm_dn9 = assign30600_e30008_d_n9;
        locals.var_dnm_dn10 = assign30600_e30008_d_n10;
        locals.var_dnm_dn11 = assign30600_e30008_d_n11;
        locals.var_dnm_dn14 = assign30600_e30008_d_n14;

        let assign30610_e30023: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard702 = assign30610_e30023;

        let assign30620_e30026: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard703 = assign30620_e30026;

        let (assign30630_e30040,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) && (locals.var_guard702 != 0.0)) && (locals.var_guard703 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30630_e30040;

        let assign30640_e30043: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard704 = assign30640_e30043;

        let (assign30650_e30060,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) && (locals.var_guard702 != 0.0)) && (locals.var_guard703 == 0.0)) && (locals.var_guard704 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30650_e30060;

        let assign30660_e30063: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard705 = assign30660_e30063;

        let (assign30670_e30083,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) && (locals.var_guard702 != 0.0)) && (locals.var_guard703 == 0.0)) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30670_e30083;

        let assign30680_e30086: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard706 = assign30680_e30086;

        let (assign30690_e30109,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) && (locals.var_guard702 != 0.0)) && (locals.var_guard703 == 0.0)) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) && (locals.var_guard706 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30690_e30109;

        let (assign30700_e30121,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) && (locals.var_guard702 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30700_e30121;

    }

    pub(super) fn stamp_transient_block_89(
        locals: &mut StampLocals,
    ) {
        let mut assign30710_loop_guard: usize = 0;
        while {
            let assign30710_cond_e30134: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) && (locals.var_guard702 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign30710_cond_e30134 != 0.0
        } {
            assign30710_loop_guard += 1;
            assert!(assign30710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign30710_body0_e30147, assign30710_body0_e30147_d_n0, assign30710_body0_e30147_d_n2, assign30710_body0_e30147_d_n4, assign30710_body0_e30147_d_n5, assign30710_body0_e30147_d_n6, assign30710_body0_e30147_d_n7, assign30710_body0_e30147_d_n8, assign30710_body0_e30147_d_n9, assign30710_body0_e30147_d_n10, assign30710_body0_e30147_d_n11, assign30710_body0_e30147_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) && (locals.var_guard702 != 0.0)) {
        let assign30710_body0_e30145: f64 = (locals.var_dnm).sqrt();
        (assign30710_body0_e30145, (locals.var_dnm_dn0 / (2.0 * assign30710_body0_e30145)), (locals.var_dnm_dn2 / (2.0 * assign30710_body0_e30145)), (locals.var_dnm_dn4 / (2.0 * assign30710_body0_e30145)), (locals.var_dnm_dn5 / (2.0 * assign30710_body0_e30145)), (locals.var_dnm_dn6 / (2.0 * assign30710_body0_e30145)), (locals.var_dnm_dn7 / (2.0 * assign30710_body0_e30145)), (locals.var_dnm_dn8 / (2.0 * assign30710_body0_e30145)), (locals.var_dnm_dn9 / (2.0 * assign30710_body0_e30145)), (locals.var_dnm_dn10 / (2.0 * assign30710_body0_e30145)), (locals.var_dnm_dn11 / (2.0 * assign30710_body0_e30145)), (locals.var_dnm_dn14 / (2.0 * assign30710_body0_e30145)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign30710_body0_e30147;
            locals.var_dnm_dn0 = assign30710_body0_e30147_d_n0;
            locals.var_dnm_dn2 = assign30710_body0_e30147_d_n2;
            locals.var_dnm_dn4 = assign30710_body0_e30147_d_n4;
            locals.var_dnm_dn5 = assign30710_body0_e30147_d_n5;
            locals.var_dnm_dn6 = assign30710_body0_e30147_d_n6;
            locals.var_dnm_dn7 = assign30710_body0_e30147_d_n7;
            locals.var_dnm_dn8 = assign30710_body0_e30147_d_n8;
            locals.var_dnm_dn9 = assign30710_body0_e30147_d_n9;
            locals.var_dnm_dn10 = assign30710_body0_e30147_d_n10;
            locals.var_dnm_dn11 = assign30710_body0_e30147_d_n11;
            locals.var_dnm_dn14 = assign30710_body0_e30147_d_n14;
            let (assign30710_body1_e30161,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) && (locals.var_guard702 != 0.0)) {
        let assign30710_body1_e30159: f64 = (locals.var_m0 + 1.0);
        (assign30710_body1_e30159,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign30710_body1_e30161;
        }

        let (assign30720_e30185, assign30720_e30185_d_n0, assign30720_e30185_d_n2, assign30720_e30185_d_n4, assign30720_e30185_d_n5, assign30720_e30185_d_n6, assign30720_e30185_d_n7, assign30720_e30185_d_n8, assign30720_e30185_d_n9, assign30720_e30185_d_n10, assign30720_e30185_d_n11, assign30720_e30185_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) && (locals.var_guard702 == 0.0)) {
        let (assign30720_e30183, assign30720_e30183_d_n0, assign30720_e30183_d_n2, assign30720_e30183_d_n4, assign30720_e30183_d_n5, assign30720_e30183_d_n6, assign30720_e30183_d_n7, assign30720_e30183_d_n8, assign30720_e30183_d_n9, assign30720_e30183_d_n10, assign30720_e30183_d_n11, assign30720_e30183_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign30720_e30180: f64 = (2.0 * 2.0);
                let assign30720_e30181: f64 = (1.0 / assign30720_e30180);
                let assign30720_e30182: f64 = (locals.var_dnm).powf(assign30720_e30181);
                (assign30720_e30182, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn0)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn2)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn4)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn5)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn6)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn7)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn8)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn9)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn10)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn11)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign30720_e30181) as f64).is_finite() && ((assign30720_e30181) as f64).fract() == 0.0 { if assign30720_e30181 == 0.0 { 0.0 } else { (assign30720_e30181 * ((locals.var_dnm).powf(assign30720_e30181 - 1.0) * locals.var_dnm_dn14)) } } else { (assign30720_e30182 * (assign30720_e30181 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign30720_e30183, assign30720_e30183_d_n0, assign30720_e30183_d_n2, assign30720_e30183_d_n4, assign30720_e30183_d_n5, assign30720_e30183_d_n6, assign30720_e30183_d_n7, assign30720_e30183_d_n8, assign30720_e30183_d_n9, assign30720_e30183_d_n10, assign30720_e30183_d_n11, assign30720_e30183_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30720_e30185;
        locals.var_dnm_dn0 = assign30720_e30185_d_n0;
        locals.var_dnm_dn2 = assign30720_e30185_d_n2;
        locals.var_dnm_dn4 = assign30720_e30185_d_n4;
        locals.var_dnm_dn5 = assign30720_e30185_d_n5;
        locals.var_dnm_dn6 = assign30720_e30185_d_n6;
        locals.var_dnm_dn7 = assign30720_e30185_d_n7;
        locals.var_dnm_dn8 = assign30720_e30185_d_n8;
        locals.var_dnm_dn9 = assign30720_e30185_d_n9;
        locals.var_dnm_dn10 = assign30720_e30185_d_n10;
        locals.var_dnm_dn11 = assign30720_e30185_d_n11;
        locals.var_dnm_dn14 = assign30720_e30185_d_n14;

        let (assign30730_e30197, assign30730_e30197_d_n0, assign30730_e30197_d_n2, assign30730_e30197_d_n4, assign30730_e30197_d_n5, assign30730_e30197_d_n6, assign30730_e30197_d_n7, assign30730_e30197_d_n8, assign30730_e30197_d_n9, assign30730_e30197_d_n10, assign30730_e30197_d_n11, assign30730_e30197_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30730_e30195: f64 = (1.0 / locals.var_dnm);
        (assign30730_e30195, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30730_e30197;
        locals.var_dnm_dn0 = assign30730_e30197_d_n0;
        locals.var_dnm_dn2 = assign30730_e30197_d_n2;
        locals.var_dnm_dn4 = assign30730_e30197_d_n4;
        locals.var_dnm_dn5 = assign30730_e30197_d_n5;
        locals.var_dnm_dn6 = assign30730_e30197_d_n6;
        locals.var_dnm_dn7 = assign30730_e30197_d_n7;
        locals.var_dnm_dn8 = assign30730_e30197_d_n8;
        locals.var_dnm_dn9 = assign30730_e30197_d_n9;
        locals.var_dnm_dn10 = assign30730_e30197_d_n10;
        locals.var_dnm_dn11 = assign30730_e30197_d_n11;
        locals.var_dnm_dn14 = assign30730_e30197_d_n14;

        let (assign30740_e30211, assign30740_e30211_d_n0, assign30740_e30211_d_n2, assign30740_e30211_d_n4, assign30740_e30211_d_n5, assign30740_e30211_d_n6, assign30740_e30211_d_n7, assign30740_e30211_d_n8, assign30740_e30211_d_n9, assign30740_e30211_d_n10, assign30740_e30211_d_n11, assign30740_e30211_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30740_e30207: f64 = (locals.var_tmf1 * 0.5);
        let assign30740_e30209: f64 = (assign30740_e30207 * locals.var_dnm);
        (assign30740_e30209, (((locals.var_tmf1_dn0 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.5) * locals.var_dnm) + (assign30740_e30207 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign30740_e30211;
        locals.var_tmf0_dn0 = assign30740_e30211_d_n0;
        locals.var_tmf0_dn2 = assign30740_e30211_d_n2;
        locals.var_tmf0_dn4 = assign30740_e30211_d_n4;
        locals.var_tmf0_dn5 = assign30740_e30211_d_n5;
        locals.var_tmf0_dn6 = assign30740_e30211_d_n6;
        locals.var_tmf0_dn7 = assign30740_e30211_d_n7;
        locals.var_tmf0_dn8 = assign30740_e30211_d_n8;
        locals.var_tmf0_dn9 = assign30740_e30211_d_n9;
        locals.var_tmf0_dn10 = assign30740_e30211_d_n10;
        locals.var_tmf0_dn11 = assign30740_e30211_d_n11;
        locals.var_tmf0_dn14 = assign30740_e30211_d_n14;

        let (assign30750_e30227, assign30750_e30227_d_n0, assign30750_e30227_d_n2, assign30750_e30227_d_n4, assign30750_e30227_d_n5, assign30750_e30227_d_n6, assign30750_e30227_d_n7, assign30750_e30227_d_n8, assign30750_e30227_d_n9, assign30750_e30227_d_n10, assign30750_e30227_d_n11, assign30750_e30227_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30750_e30221: f64 = (0.5 * locals.var_xmp);
        let assign30750_e30223: f64 = (assign30750_e30221 * locals.var_dnm);
        let assign30750_e30225: f64 = (assign30750_e30223 / locals.var_arg);
        (assign30750_e30225, ((((((0.5 * locals.var_xmp_dn0) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn0)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn2) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn2)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn4) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn4)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn5) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn5)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn6) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn6)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn7) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn7)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn8) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn8)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn9) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn9)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn10) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn10)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn11) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn11)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.5 * locals.var_xmp_dn14) * locals.var_dnm) + (assign30750_e30221 * locals.var_dnm_dn14)) * locals.var_arg) - (assign30750_e30223 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30750_e30227;
        locals.var_t0_dn0 = assign30750_e30227_d_n0;
        locals.var_t0_dn2 = assign30750_e30227_d_n2;
        locals.var_t0_dn4 = assign30750_e30227_d_n4;
        locals.var_t0_dn5 = assign30750_e30227_d_n5;
        locals.var_t0_dn6 = assign30750_e30227_d_n6;
        locals.var_t0_dn7 = assign30750_e30227_d_n7;
        locals.var_t0_dn8 = assign30750_e30227_d_n8;
        locals.var_t0_dn9 = assign30750_e30227_d_n9;
        locals.var_t0_dn10 = assign30750_e30227_d_n10;
        locals.var_t0_dn11 = assign30750_e30227_d_n11;
        locals.var_t0_dn14 = assign30750_e30227_d_n14;

        let (assign30760_e30241, assign30760_e30241_d_n0, assign30760_e30241_d_n2, assign30760_e30241_d_n4, assign30760_e30241_d_n5, assign30760_e30241_d_n6, assign30760_e30241_d_n7, assign30760_e30241_d_n8, assign30760_e30241_d_n9, assign30760_e30241_d_n10, assign30760_e30241_d_n11, assign30760_e30241_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign30760_e30237: f64 = 0.5;
        let assign30760_e30239: f64 = (assign30760_e30237 - locals.var_tmf0);
        (assign30760_e30239, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_vgpp, locals.var_vgpp_dn0, locals.var_vgpp_dn2, locals.var_vgpp_dn4, locals.var_vgpp_dn5, locals.var_vgpp_dn6, locals.var_vgpp_dn7, locals.var_vgpp_dn8, locals.var_vgpp_dn9, locals.var_vgpp_dn10, locals.var_vgpp_dn11, locals.var_vgpp_dn14,)
    }
};
        locals.var_vgpp = assign30760_e30241;
        locals.var_vgpp_dn0 = assign30760_e30241_d_n0;
        locals.var_vgpp_dn2 = assign30760_e30241_d_n2;
        locals.var_vgpp_dn4 = assign30760_e30241_d_n4;
        locals.var_vgpp_dn5 = assign30760_e30241_d_n5;
        locals.var_vgpp_dn6 = assign30760_e30241_d_n6;
        locals.var_vgpp_dn7 = assign30760_e30241_d_n7;
        locals.var_vgpp_dn8 = assign30760_e30241_d_n8;
        locals.var_vgpp_dn9 = assign30760_e30241_d_n9;
        locals.var_vgpp_dn10 = assign30760_e30241_d_n10;
        locals.var_vgpp_dn11 = assign30760_e30241_d_n11;
        locals.var_vgpp_dn14 = assign30760_e30241_d_n14;

        let (assign30770_e30251, assign30770_e30251_d_n0, assign30770_e30251_d_n2, assign30770_e30251_d_n4, assign30770_e30251_d_n5, assign30770_e30251_d_n6, assign30770_e30251_d_n7, assign30770_e30251_d_n8, assign30770_e30251_d_n9, assign30770_e30251_d_n10, assign30770_e30251_d_n11, assign30770_e30251_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30770_e30251;
        locals.var_t0_dn0 = assign30770_e30251_d_n0;
        locals.var_t0_dn2 = assign30770_e30251_d_n2;
        locals.var_t0_dn4 = assign30770_e30251_d_n4;
        locals.var_t0_dn5 = assign30770_e30251_d_n5;
        locals.var_t0_dn6 = assign30770_e30251_d_n6;
        locals.var_t0_dn7 = assign30770_e30251_d_n7;
        locals.var_t0_dn8 = assign30770_e30251_d_n8;
        locals.var_t0_dn9 = assign30770_e30251_d_n9;
        locals.var_t0_dn10 = assign30770_e30251_d_n10;
        locals.var_t0_dn11 = assign30770_e30251_d_n11;
        locals.var_t0_dn14 = assign30770_e30251_d_n14;

        let (assign30780_e30262, assign30780_e30262_d_n0, assign30780_e30262_d_n2, assign30780_e30262_d_n4, assign30780_e30262_d_n5, assign30780_e30262_d_n6, assign30780_e30262_d_n7, assign30780_e30262_d_n8, assign30780_e30262_d_n9, assign30780_e30262_d_n10, assign30780_e30262_d_n11, assign30780_e30262_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 == 0.0)) {
        (locals.var_vgp, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn11, locals.var_vgp_dn14,)
    } else {
        (locals.var_vgpp, locals.var_vgpp_dn0, locals.var_vgpp_dn2, locals.var_vgpp_dn4, locals.var_vgpp_dn5, locals.var_vgpp_dn6, locals.var_vgpp_dn7, locals.var_vgpp_dn8, locals.var_vgpp_dn9, locals.var_vgpp_dn10, locals.var_vgpp_dn11, locals.var_vgpp_dn14,)
    }
};
        locals.var_vgpp = assign30780_e30262;
        locals.var_vgpp_dn0 = assign30780_e30262_d_n0;
        locals.var_vgpp_dn2 = assign30780_e30262_d_n2;
        locals.var_vgpp_dn4 = assign30780_e30262_d_n4;
        locals.var_vgpp_dn5 = assign30780_e30262_d_n5;
        locals.var_vgpp_dn6 = assign30780_e30262_d_n6;
        locals.var_vgpp_dn7 = assign30780_e30262_d_n7;
        locals.var_vgpp_dn8 = assign30780_e30262_d_n8;
        locals.var_vgpp_dn9 = assign30780_e30262_d_n9;
        locals.var_vgpp_dn10 = assign30780_e30262_d_n10;
        locals.var_vgpp_dn11 = assign30780_e30262_d_n11;
        locals.var_vgpp_dn14 = assign30780_e30262_d_n14;

        let (assign30790_e30273, assign30790_e30273_d_n0, assign30790_e30273_d_n2, assign30790_e30273_d_n4, assign30790_e30273_d_n5, assign30790_e30273_d_n6, assign30790_e30273_d_n7, assign30790_e30273_d_n8, assign30790_e30273_d_n9, assign30790_e30273_d_n10, assign30790_e30273_d_n11, assign30790_e30273_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard701 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign30790_e30273;
        locals.var_t0_dn0 = assign30790_e30273_d_n0;
        locals.var_t0_dn2 = assign30790_e30273_d_n2;
        locals.var_t0_dn4 = assign30790_e30273_d_n4;
        locals.var_t0_dn5 = assign30790_e30273_d_n5;
        locals.var_t0_dn6 = assign30790_e30273_d_n6;
        locals.var_t0_dn7 = assign30790_e30273_d_n7;
        locals.var_t0_dn8 = assign30790_e30273_d_n8;
        locals.var_t0_dn9 = assign30790_e30273_d_n9;
        locals.var_t0_dn10 = assign30790_e30273_d_n10;
        locals.var_t0_dn11 = assign30790_e30273_d_n11;
        locals.var_t0_dn14 = assign30790_e30273_d_n14;

        let (assign30800_e30283, assign30800_e30283_d_n0, assign30800_e30283_d_n2, assign30800_e30283_d_n4, assign30800_e30283_d_n5, assign30800_e30283_d_n6, assign30800_e30283_d_n7, assign30800_e30283_d_n8, assign30800_e30283_d_n9, assign30800_e30283_d_n10, assign30800_e30283_d_n11, assign30800_e30283_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign30800_e30281: f64 = (locals.var_vgpp * 0.8);
        (assign30800_e30281, (locals.var_vgpp_dn0 * 0.8), (locals.var_vgpp_dn2 * 0.8), (locals.var_vgpp_dn4 * 0.8), (locals.var_vgpp_dn5 * 0.8), (locals.var_vgpp_dn6 * 0.8), (locals.var_vgpp_dn7 * 0.8), (locals.var_vgpp_dn8 * 0.8), (locals.var_vgpp_dn9 * 0.8), (locals.var_vgpp_dn10 * 0.8), (locals.var_vgpp_dn11 * 0.8), (locals.var_vgpp_dn14 * 0.8),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign30800_e30283;
        locals.var_t1_dn0 = assign30800_e30283_d_n0;
        locals.var_t1_dn2 = assign30800_e30283_d_n2;
        locals.var_t1_dn4 = assign30800_e30283_d_n4;
        locals.var_t1_dn5 = assign30800_e30283_d_n5;
        locals.var_t1_dn6 = assign30800_e30283_d_n6;
        locals.var_t1_dn7 = assign30800_e30283_d_n7;
        locals.var_t1_dn8 = assign30800_e30283_d_n8;
        locals.var_t1_dn9 = assign30800_e30283_d_n9;
        locals.var_t1_dn10 = assign30800_e30283_d_n10;
        locals.var_t1_dn11 = assign30800_e30283_d_n11;
        locals.var_t1_dn14 = assign30800_e30283_d_n14;

        let assign30810_e30287: f64 = (locals.var_vgpp - locals.var_t1);
        let assign30810_e30292: f64 = if ((locals.var_vdseff > assign30810_e30287) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard707 = assign30810_e30292;

        let (assign30820_e30306, assign30820_e30306_d_n0, assign30820_e30306_d_n2, assign30820_e30306_d_n4, assign30820_e30306_d_n5, assign30820_e30306_d_n6, assign30820_e30306_d_n7, assign30820_e30306_d_n8, assign30820_e30306_d_n9, assign30820_e30306_d_n10, assign30820_e30306_d_n11, assign30820_e30306_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign30820_e30302: f64 = (locals.var_vdseff - locals.var_vgpp);
        let assign30820_e30304: f64 = (assign30820_e30302 + locals.var_t1);
        (assign30820_e30304, ((locals.var_vdseff_dn0 - locals.var_vgpp_dn0) + locals.var_t1_dn0), ((locals.var_vdseff_dn2 - locals.var_vgpp_dn2) + locals.var_t1_dn2), ((locals.var_vdseff_dn4 - locals.var_vgpp_dn4) + locals.var_t1_dn4), ((locals.var_vdseff_dn5 - locals.var_vgpp_dn5) + locals.var_t1_dn5), ((locals.var_vdseff_dn6 - locals.var_vgpp_dn6) + locals.var_t1_dn6), ((locals.var_vdseff_dn7 - locals.var_vgpp_dn7) + locals.var_t1_dn7), ((locals.var_vdseff_dn8 - locals.var_vgpp_dn8) + locals.var_t1_dn8), ((locals.var_vdseff_dn9 - locals.var_vgpp_dn9) + locals.var_t1_dn9), ((locals.var_vdseff_dn10 - locals.var_vgpp_dn10) + locals.var_t1_dn10), ((locals.var_vdseff_dn11 - locals.var_vgpp_dn11) + locals.var_t1_dn11), ((locals.var_vdseff_dn14 - locals.var_vgpp_dn14) + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign30820_e30306;
        locals.var_tmf1_dn0 = assign30820_e30306_d_n0;
        locals.var_tmf1_dn2 = assign30820_e30306_d_n2;
        locals.var_tmf1_dn4 = assign30820_e30306_d_n4;
        locals.var_tmf1_dn5 = assign30820_e30306_d_n5;
        locals.var_tmf1_dn6 = assign30820_e30306_d_n6;
        locals.var_tmf1_dn7 = assign30820_e30306_d_n7;
        locals.var_tmf1_dn8 = assign30820_e30306_d_n8;
        locals.var_tmf1_dn9 = assign30820_e30306_d_n9;
        locals.var_tmf1_dn10 = assign30820_e30306_d_n10;
        locals.var_tmf1_dn11 = assign30820_e30306_d_n11;
        locals.var_tmf1_dn14 = assign30820_e30306_d_n14;

        let (assign30830_e30318, assign30830_e30318_d_n0, assign30830_e30318_d_n2, assign30830_e30318_d_n4, assign30830_e30318_d_n5, assign30830_e30318_d_n6, assign30830_e30318_d_n7, assign30830_e30318_d_n8, assign30830_e30318_d_n9, assign30830_e30318_d_n10, assign30830_e30318_d_n11, assign30830_e30318_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign30830_e30316: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign30830_e30316, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign30830_e30318;
        locals.var_x2_dn0 = assign30830_e30318_d_n0;
        locals.var_x2_dn2 = assign30830_e30318_d_n2;
        locals.var_x2_dn4 = assign30830_e30318_d_n4;
        locals.var_x2_dn5 = assign30830_e30318_d_n5;
        locals.var_x2_dn6 = assign30830_e30318_d_n6;
        locals.var_x2_dn7 = assign30830_e30318_d_n7;
        locals.var_x2_dn8 = assign30830_e30318_d_n8;
        locals.var_x2_dn9 = assign30830_e30318_d_n9;
        locals.var_x2_dn10 = assign30830_e30318_d_n10;
        locals.var_x2_dn11 = assign30830_e30318_d_n11;
        locals.var_x2_dn14 = assign30830_e30318_d_n14;

        let (assign30840_e30330, assign30840_e30330_d_n0, assign30840_e30330_d_n2, assign30840_e30330_d_n4, assign30840_e30330_d_n5, assign30840_e30330_d_n6, assign30840_e30330_d_n7, assign30840_e30330_d_n8, assign30840_e30330_d_n9, assign30840_e30330_d_n10, assign30840_e30330_d_n11, assign30840_e30330_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign30840_e30328: f64 = (locals.var_t1 * locals.var_t1);
        (assign30840_e30328, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign30840_e30330;
        locals.var_xmax2_dn0 = assign30840_e30330_d_n0;
        locals.var_xmax2_dn2 = assign30840_e30330_d_n2;
        locals.var_xmax2_dn4 = assign30840_e30330_d_n4;
        locals.var_xmax2_dn5 = assign30840_e30330_d_n5;
        locals.var_xmax2_dn6 = assign30840_e30330_d_n6;
        locals.var_xmax2_dn7 = assign30840_e30330_d_n7;
        locals.var_xmax2_dn8 = assign30840_e30330_d_n8;
        locals.var_xmax2_dn9 = assign30840_e30330_d_n9;
        locals.var_xmax2_dn10 = assign30840_e30330_d_n10;
        locals.var_xmax2_dn11 = assign30840_e30330_d_n11;
        locals.var_xmax2_dn14 = assign30840_e30330_d_n14;

        let (assign30850_e30340, assign30850_e30340_d_n0, assign30850_e30340_d_n2, assign30850_e30340_d_n4, assign30850_e30340_d_n5, assign30850_e30340_d_n6, assign30850_e30340_d_n7, assign30850_e30340_d_n8, assign30850_e30340_d_n9, assign30850_e30340_d_n10, assign30850_e30340_d_n11, assign30850_e30340_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30850_e30340;
        locals.var_xp_dn0 = assign30850_e30340_d_n0;
        locals.var_xp_dn2 = assign30850_e30340_d_n2;
        locals.var_xp_dn4 = assign30850_e30340_d_n4;
        locals.var_xp_dn5 = assign30850_e30340_d_n5;
        locals.var_xp_dn6 = assign30850_e30340_d_n6;
        locals.var_xp_dn7 = assign30850_e30340_d_n7;
        locals.var_xp_dn8 = assign30850_e30340_d_n8;
        locals.var_xp_dn9 = assign30850_e30340_d_n9;
        locals.var_xp_dn10 = assign30850_e30340_d_n10;
        locals.var_xp_dn11 = assign30850_e30340_d_n11;
        locals.var_xp_dn14 = assign30850_e30340_d_n14;

        let (assign30860_e30350, assign30860_e30350_d_n0, assign30860_e30350_d_n2, assign30860_e30350_d_n4, assign30860_e30350_d_n5, assign30860_e30350_d_n6, assign30860_e30350_d_n7, assign30860_e30350_d_n8, assign30860_e30350_d_n9, assign30860_e30350_d_n10, assign30860_e30350_d_n11, assign30860_e30350_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30860_e30350;
        locals.var_xmp_dn0 = assign30860_e30350_d_n0;
        locals.var_xmp_dn2 = assign30860_e30350_d_n2;
        locals.var_xmp_dn4 = assign30860_e30350_d_n4;
        locals.var_xmp_dn5 = assign30860_e30350_d_n5;
        locals.var_xmp_dn6 = assign30860_e30350_d_n6;
        locals.var_xmp_dn7 = assign30860_e30350_d_n7;
        locals.var_xmp_dn8 = assign30860_e30350_d_n8;
        locals.var_xmp_dn9 = assign30860_e30350_d_n9;
        locals.var_xmp_dn10 = assign30860_e30350_d_n10;
        locals.var_xmp_dn11 = assign30860_e30350_d_n11;
        locals.var_xmp_dn14 = assign30860_e30350_d_n14;

        let (assign30870_e30360,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign30870_e30360;

        let (assign30880_e30370,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30880_e30370;

        let (assign30890_e30380, assign30890_e30380_d_n0, assign30890_e30380_d_n2, assign30890_e30380_d_n4, assign30890_e30380_d_n5, assign30890_e30380_d_n6, assign30890_e30380_d_n7, assign30890_e30380_d_n8, assign30890_e30380_d_n9, assign30890_e30380_d_n10, assign30890_e30380_d_n11, assign30890_e30380_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30890_e30380;
        locals.var_arg_dn0 = assign30890_e30380_d_n0;
        locals.var_arg_dn2 = assign30890_e30380_d_n2;
        locals.var_arg_dn4 = assign30890_e30380_d_n4;
        locals.var_arg_dn5 = assign30890_e30380_d_n5;
        locals.var_arg_dn6 = assign30890_e30380_d_n6;
        locals.var_arg_dn7 = assign30890_e30380_d_n7;
        locals.var_arg_dn8 = assign30890_e30380_d_n8;
        locals.var_arg_dn9 = assign30890_e30380_d_n9;
        locals.var_arg_dn10 = assign30890_e30380_d_n10;
        locals.var_arg_dn11 = assign30890_e30380_d_n11;
        locals.var_arg_dn14 = assign30890_e30380_d_n14;

        let (assign30900_e30390, assign30900_e30390_d_n0, assign30900_e30390_d_n2, assign30900_e30390_d_n4, assign30900_e30390_d_n5, assign30900_e30390_d_n6, assign30900_e30390_d_n7, assign30900_e30390_d_n8, assign30900_e30390_d_n9, assign30900_e30390_d_n10, assign30900_e30390_d_n11, assign30900_e30390_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30900_e30390;
        locals.var_dnm_dn0 = assign30900_e30390_d_n0;
        locals.var_dnm_dn2 = assign30900_e30390_d_n2;
        locals.var_dnm_dn4 = assign30900_e30390_d_n4;
        locals.var_dnm_dn5 = assign30900_e30390_d_n5;
        locals.var_dnm_dn6 = assign30900_e30390_d_n6;
        locals.var_dnm_dn7 = assign30900_e30390_d_n7;
        locals.var_dnm_dn8 = assign30900_e30390_d_n8;
        locals.var_dnm_dn9 = assign30900_e30390_d_n9;
        locals.var_dnm_dn10 = assign30900_e30390_d_n10;
        locals.var_dnm_dn11 = assign30900_e30390_d_n11;
        locals.var_dnm_dn14 = assign30900_e30390_d_n14;

        let (assign30910_e30402, assign30910_e30402_d_n0, assign30910_e30402_d_n2, assign30910_e30402_d_n4, assign30910_e30402_d_n5, assign30910_e30402_d_n6, assign30910_e30402_d_n7, assign30910_e30402_d_n8, assign30910_e30402_d_n9, assign30910_e30402_d_n10, assign30910_e30402_d_n11, assign30910_e30402_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign30910_e30400: f64 = (locals.var_xp * locals.var_x2);
        (assign30910_e30400, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30910_e30402;
        locals.var_xp_dn0 = assign30910_e30402_d_n0;
        locals.var_xp_dn2 = assign30910_e30402_d_n2;
        locals.var_xp_dn4 = assign30910_e30402_d_n4;
        locals.var_xp_dn5 = assign30910_e30402_d_n5;
        locals.var_xp_dn6 = assign30910_e30402_d_n6;
        locals.var_xp_dn7 = assign30910_e30402_d_n7;
        locals.var_xp_dn8 = assign30910_e30402_d_n8;
        locals.var_xp_dn9 = assign30910_e30402_d_n9;
        locals.var_xp_dn10 = assign30910_e30402_d_n10;
        locals.var_xp_dn11 = assign30910_e30402_d_n11;
        locals.var_xp_dn14 = assign30910_e30402_d_n14;

        let (assign30920_e30414, assign30920_e30414_d_n0, assign30920_e30414_d_n2, assign30920_e30414_d_n4, assign30920_e30414_d_n5, assign30920_e30414_d_n6, assign30920_e30414_d_n7, assign30920_e30414_d_n8, assign30920_e30414_d_n9, assign30920_e30414_d_n10, assign30920_e30414_d_n11, assign30920_e30414_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign30920_e30412: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30920_e30412, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30920_e30414;
        locals.var_xmp_dn0 = assign30920_e30414_d_n0;
        locals.var_xmp_dn2 = assign30920_e30414_d_n2;
        locals.var_xmp_dn4 = assign30920_e30414_d_n4;
        locals.var_xmp_dn5 = assign30920_e30414_d_n5;
        locals.var_xmp_dn6 = assign30920_e30414_d_n6;
        locals.var_xmp_dn7 = assign30920_e30414_d_n7;
        locals.var_xmp_dn8 = assign30920_e30414_d_n8;
        locals.var_xmp_dn9 = assign30920_e30414_d_n9;
        locals.var_xmp_dn10 = assign30920_e30414_d_n10;
        locals.var_xmp_dn11 = assign30920_e30414_d_n11;
        locals.var_xmp_dn14 = assign30920_e30414_d_n14;

        let (assign30930_e30426, assign30930_e30426_d_n0, assign30930_e30426_d_n2, assign30930_e30426_d_n4, assign30930_e30426_d_n5, assign30930_e30426_d_n6, assign30930_e30426_d_n7, assign30930_e30426_d_n8, assign30930_e30426_d_n9, assign30930_e30426_d_n10, assign30930_e30426_d_n11, assign30930_e30426_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign30930_e30424: f64 = (locals.var_xp * locals.var_x2);
        (assign30930_e30424, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign30930_e30426;
        locals.var_xp_dn0 = assign30930_e30426_d_n0;
        locals.var_xp_dn2 = assign30930_e30426_d_n2;
        locals.var_xp_dn4 = assign30930_e30426_d_n4;
        locals.var_xp_dn5 = assign30930_e30426_d_n5;
        locals.var_xp_dn6 = assign30930_e30426_d_n6;
        locals.var_xp_dn7 = assign30930_e30426_d_n7;
        locals.var_xp_dn8 = assign30930_e30426_d_n8;
        locals.var_xp_dn9 = assign30930_e30426_d_n9;
        locals.var_xp_dn10 = assign30930_e30426_d_n10;
        locals.var_xp_dn11 = assign30930_e30426_d_n11;
        locals.var_xp_dn14 = assign30930_e30426_d_n14;

        let (assign30940_e30438, assign30940_e30438_d_n0, assign30940_e30438_d_n2, assign30940_e30438_d_n4, assign30940_e30438_d_n5, assign30940_e30438_d_n6, assign30940_e30438_d_n7, assign30940_e30438_d_n8, assign30940_e30438_d_n9, assign30940_e30438_d_n10, assign30940_e30438_d_n11, assign30940_e30438_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign30940_e30436: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign30940_e30436, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign30940_e30438;
        locals.var_xmp_dn0 = assign30940_e30438_d_n0;
        locals.var_xmp_dn2 = assign30940_e30438_d_n2;
        locals.var_xmp_dn4 = assign30940_e30438_d_n4;
        locals.var_xmp_dn5 = assign30940_e30438_d_n5;
        locals.var_xmp_dn6 = assign30940_e30438_d_n6;
        locals.var_xmp_dn7 = assign30940_e30438_d_n7;
        locals.var_xmp_dn8 = assign30940_e30438_d_n8;
        locals.var_xmp_dn9 = assign30940_e30438_d_n9;
        locals.var_xmp_dn10 = assign30940_e30438_d_n10;
        locals.var_xmp_dn11 = assign30940_e30438_d_n11;
        locals.var_xmp_dn14 = assign30940_e30438_d_n14;

        let (assign30950_e30450, assign30950_e30450_d_n0, assign30950_e30450_d_n2, assign30950_e30450_d_n4, assign30950_e30450_d_n5, assign30950_e30450_d_n6, assign30950_e30450_d_n7, assign30950_e30450_d_n8, assign30950_e30450_d_n9, assign30950_e30450_d_n10, assign30950_e30450_d_n11, assign30950_e30450_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign30950_e30448: f64 = (locals.var_xp + locals.var_xmp);
        (assign30950_e30448, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign30950_e30450;
        locals.var_arg_dn0 = assign30950_e30450_d_n0;
        locals.var_arg_dn2 = assign30950_e30450_d_n2;
        locals.var_arg_dn4 = assign30950_e30450_d_n4;
        locals.var_arg_dn5 = assign30950_e30450_d_n5;
        locals.var_arg_dn6 = assign30950_e30450_d_n6;
        locals.var_arg_dn7 = assign30950_e30450_d_n7;
        locals.var_arg_dn8 = assign30950_e30450_d_n8;
        locals.var_arg_dn9 = assign30950_e30450_d_n9;
        locals.var_arg_dn10 = assign30950_e30450_d_n10;
        locals.var_arg_dn11 = assign30950_e30450_d_n11;
        locals.var_arg_dn14 = assign30950_e30450_d_n14;

    }

    pub(super) fn stamp_transient_block_90(
        locals: &mut StampLocals,
    ) {
        let (assign30960_e30460, assign30960_e30460_d_n0, assign30960_e30460_d_n2, assign30960_e30460_d_n4, assign30960_e30460_d_n5, assign30960_e30460_d_n6, assign30960_e30460_d_n7, assign30960_e30460_d_n8, assign30960_e30460_d_n9, assign30960_e30460_d_n10, assign30960_e30460_d_n11, assign30960_e30460_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign30960_e30460;
        locals.var_dnm_dn0 = assign30960_e30460_d_n0;
        locals.var_dnm_dn2 = assign30960_e30460_d_n2;
        locals.var_dnm_dn4 = assign30960_e30460_d_n4;
        locals.var_dnm_dn5 = assign30960_e30460_d_n5;
        locals.var_dnm_dn6 = assign30960_e30460_d_n6;
        locals.var_dnm_dn7 = assign30960_e30460_d_n7;
        locals.var_dnm_dn8 = assign30960_e30460_d_n8;
        locals.var_dnm_dn9 = assign30960_e30460_d_n9;
        locals.var_dnm_dn10 = assign30960_e30460_d_n10;
        locals.var_dnm_dn11 = assign30960_e30460_d_n11;
        locals.var_dnm_dn14 = assign30960_e30460_d_n14;

        let assign30970_e30475: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard708 = assign30970_e30475;

        let assign30980_e30478: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard709 = assign30980_e30478;

        let (assign30990_e30492,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 != 0.0)) && (locals.var_guard709 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign30990_e30492;

        let assign31000_e30495: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard710 = assign31000_e30495;

        let (assign31010_e30512,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 != 0.0)) && (locals.var_guard709 == 0.0)) && (locals.var_guard710 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign31010_e30512;

        let assign31020_e30515: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard711 = assign31020_e30515;

        let (assign31030_e30535,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 != 0.0)) && (locals.var_guard709 == 0.0)) && (locals.var_guard710 == 0.0)) && (locals.var_guard711 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign31030_e30535;

        let assign31040_e30538: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard712 = assign31040_e30538;

        let (assign31050_e30561,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 != 0.0)) && (locals.var_guard709 == 0.0)) && (locals.var_guard710 == 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard712 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign31050_e30561;

        let (assign31060_e30573,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign31060_e30573;

        let mut assign31070_loop_guard: usize = 0;
        while {
            let assign31070_cond_e30586: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign31070_cond_e30586 != 0.0
        } {
            assign31070_loop_guard += 1;
            assert!(assign31070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign31070_body0_e30599, assign31070_body0_e30599_d_n0, assign31070_body0_e30599_d_n2, assign31070_body0_e30599_d_n4, assign31070_body0_e30599_d_n5, assign31070_body0_e30599_d_n6, assign31070_body0_e30599_d_n7, assign31070_body0_e30599_d_n8, assign31070_body0_e30599_d_n9, assign31070_body0_e30599_d_n10, assign31070_body0_e30599_d_n11, assign31070_body0_e30599_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 != 0.0)) {
        let assign31070_body0_e30597: f64 = (locals.var_dnm).sqrt();
        (assign31070_body0_e30597, (locals.var_dnm_dn0 / (2.0 * assign31070_body0_e30597)), (locals.var_dnm_dn2 / (2.0 * assign31070_body0_e30597)), (locals.var_dnm_dn4 / (2.0 * assign31070_body0_e30597)), (locals.var_dnm_dn5 / (2.0 * assign31070_body0_e30597)), (locals.var_dnm_dn6 / (2.0 * assign31070_body0_e30597)), (locals.var_dnm_dn7 / (2.0 * assign31070_body0_e30597)), (locals.var_dnm_dn8 / (2.0 * assign31070_body0_e30597)), (locals.var_dnm_dn9 / (2.0 * assign31070_body0_e30597)), (locals.var_dnm_dn10 / (2.0 * assign31070_body0_e30597)), (locals.var_dnm_dn11 / (2.0 * assign31070_body0_e30597)), (locals.var_dnm_dn14 / (2.0 * assign31070_body0_e30597)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31070_body0_e30599;
            locals.var_dnm_dn0 = assign31070_body0_e30599_d_n0;
            locals.var_dnm_dn2 = assign31070_body0_e30599_d_n2;
            locals.var_dnm_dn4 = assign31070_body0_e30599_d_n4;
            locals.var_dnm_dn5 = assign31070_body0_e30599_d_n5;
            locals.var_dnm_dn6 = assign31070_body0_e30599_d_n6;
            locals.var_dnm_dn7 = assign31070_body0_e30599_d_n7;
            locals.var_dnm_dn8 = assign31070_body0_e30599_d_n8;
            locals.var_dnm_dn9 = assign31070_body0_e30599_d_n9;
            locals.var_dnm_dn10 = assign31070_body0_e30599_d_n10;
            locals.var_dnm_dn11 = assign31070_body0_e30599_d_n11;
            locals.var_dnm_dn14 = assign31070_body0_e30599_d_n14;
            let (assign31070_body1_e30613,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 != 0.0)) {
        let assign31070_body1_e30611: f64 = (locals.var_m0 + 1.0);
        (assign31070_body1_e30611,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign31070_body1_e30613;
        }

        let (assign31080_e30637, assign31080_e30637_d_n0, assign31080_e30637_d_n2, assign31080_e30637_d_n4, assign31080_e30637_d_n5, assign31080_e30637_d_n6, assign31080_e30637_d_n7, assign31080_e30637_d_n8, assign31080_e30637_d_n9, assign31080_e30637_d_n10, assign31080_e30637_d_n11, assign31080_e30637_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) && (locals.var_guard708 == 0.0)) {
        let (assign31080_e30635, assign31080_e30635_d_n0, assign31080_e30635_d_n2, assign31080_e30635_d_n4, assign31080_e30635_d_n5, assign31080_e30635_d_n6, assign31080_e30635_d_n7, assign31080_e30635_d_n8, assign31080_e30635_d_n9, assign31080_e30635_d_n10, assign31080_e30635_d_n11, assign31080_e30635_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign31080_e30632: f64 = (2.0 * 2.0);
                let assign31080_e30633: f64 = (1.0 / assign31080_e30632);
                let assign31080_e30634: f64 = (locals.var_dnm).powf(assign31080_e30633);
                (assign31080_e30634, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn0)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn2)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn4)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn5)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn6)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn7)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn8)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn9)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn10)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn11)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31080_e30633) as f64).is_finite() && ((assign31080_e30633) as f64).fract() == 0.0 { if assign31080_e30633 == 0.0 { 0.0 } else { (assign31080_e30633 * ((locals.var_dnm).powf(assign31080_e30633 - 1.0) * locals.var_dnm_dn14)) } } else { (assign31080_e30634 * (assign31080_e30633 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign31080_e30635, assign31080_e30635_d_n0, assign31080_e30635_d_n2, assign31080_e30635_d_n4, assign31080_e30635_d_n5, assign31080_e30635_d_n6, assign31080_e30635_d_n7, assign31080_e30635_d_n8, assign31080_e30635_d_n9, assign31080_e30635_d_n10, assign31080_e30635_d_n11, assign31080_e30635_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign31080_e30637;
        locals.var_dnm_dn0 = assign31080_e30637_d_n0;
        locals.var_dnm_dn2 = assign31080_e30637_d_n2;
        locals.var_dnm_dn4 = assign31080_e30637_d_n4;
        locals.var_dnm_dn5 = assign31080_e30637_d_n5;
        locals.var_dnm_dn6 = assign31080_e30637_d_n6;
        locals.var_dnm_dn7 = assign31080_e30637_d_n7;
        locals.var_dnm_dn8 = assign31080_e30637_d_n8;
        locals.var_dnm_dn9 = assign31080_e30637_d_n9;
        locals.var_dnm_dn10 = assign31080_e30637_d_n10;
        locals.var_dnm_dn11 = assign31080_e30637_d_n11;
        locals.var_dnm_dn14 = assign31080_e30637_d_n14;

        let (assign31090_e30649, assign31090_e30649_d_n0, assign31090_e30649_d_n2, assign31090_e30649_d_n4, assign31090_e30649_d_n5, assign31090_e30649_d_n6, assign31090_e30649_d_n7, assign31090_e30649_d_n8, assign31090_e30649_d_n9, assign31090_e30649_d_n10, assign31090_e30649_d_n11, assign31090_e30649_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign31090_e30647: f64 = (1.0 / locals.var_dnm);
        (assign31090_e30647, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign31090_e30649;
        locals.var_dnm_dn0 = assign31090_e30649_d_n0;
        locals.var_dnm_dn2 = assign31090_e30649_d_n2;
        locals.var_dnm_dn4 = assign31090_e30649_d_n4;
        locals.var_dnm_dn5 = assign31090_e30649_d_n5;
        locals.var_dnm_dn6 = assign31090_e30649_d_n6;
        locals.var_dnm_dn7 = assign31090_e30649_d_n7;
        locals.var_dnm_dn8 = assign31090_e30649_d_n8;
        locals.var_dnm_dn9 = assign31090_e30649_d_n9;
        locals.var_dnm_dn10 = assign31090_e30649_d_n10;
        locals.var_dnm_dn11 = assign31090_e30649_d_n11;
        locals.var_dnm_dn14 = assign31090_e30649_d_n14;

        let (assign31100_e30663, assign31100_e30663_d_n0, assign31100_e30663_d_n2, assign31100_e30663_d_n4, assign31100_e30663_d_n5, assign31100_e30663_d_n6, assign31100_e30663_d_n7, assign31100_e30663_d_n8, assign31100_e30663_d_n9, assign31100_e30663_d_n10, assign31100_e30663_d_n11, assign31100_e30663_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign31100_e30659: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign31100_e30661: f64 = (assign31100_e30659 * locals.var_dnm);
        (assign31100_e30661, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign31100_e30659 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign31100_e30663;
        locals.var_tmf0_dn0 = assign31100_e30663_d_n0;
        locals.var_tmf0_dn2 = assign31100_e30663_d_n2;
        locals.var_tmf0_dn4 = assign31100_e30663_d_n4;
        locals.var_tmf0_dn5 = assign31100_e30663_d_n5;
        locals.var_tmf0_dn6 = assign31100_e30663_d_n6;
        locals.var_tmf0_dn7 = assign31100_e30663_d_n7;
        locals.var_tmf0_dn8 = assign31100_e30663_d_n8;
        locals.var_tmf0_dn9 = assign31100_e30663_d_n9;
        locals.var_tmf0_dn10 = assign31100_e30663_d_n10;
        locals.var_tmf0_dn11 = assign31100_e30663_d_n11;
        locals.var_tmf0_dn14 = assign31100_e30663_d_n14;

        let (assign31110_e30679, assign31110_e30679_d_n0, assign31110_e30679_d_n2, assign31110_e30679_d_n4, assign31110_e30679_d_n5, assign31110_e30679_d_n6, assign31110_e30679_d_n7, assign31110_e30679_d_n8, assign31110_e30679_d_n9, assign31110_e30679_d_n10, assign31110_e30679_d_n11, assign31110_e30679_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign31110_e30673: f64 = (locals.var_t1 * locals.var_xmp);
        let assign31110_e30675: f64 = (assign31110_e30673 * locals.var_dnm);
        let assign31110_e30677: f64 = (assign31110_e30675 / locals.var_arg);
        (assign31110_e30677, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn0)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn2)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn4)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn5)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn6)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn7)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn8)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn9)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn10)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn11)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign31110_e30673 * locals.var_dnm_dn14)) * locals.var_arg) - (assign31110_e30675 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31110_e30679;
        locals.var_t0_dn0 = assign31110_e30679_d_n0;
        locals.var_t0_dn2 = assign31110_e30679_d_n2;
        locals.var_t0_dn4 = assign31110_e30679_d_n4;
        locals.var_t0_dn5 = assign31110_e30679_d_n5;
        locals.var_t0_dn6 = assign31110_e30679_d_n6;
        locals.var_t0_dn7 = assign31110_e30679_d_n7;
        locals.var_t0_dn8 = assign31110_e30679_d_n8;
        locals.var_t0_dn9 = assign31110_e30679_d_n9;
        locals.var_t0_dn10 = assign31110_e30679_d_n10;
        locals.var_t0_dn11 = assign31110_e30679_d_n11;
        locals.var_t0_dn14 = assign31110_e30679_d_n14;

        let (assign31120_e30693, assign31120_e30693_d_n0, assign31120_e30693_d_n2, assign31120_e30693_d_n4, assign31120_e30693_d_n5, assign31120_e30693_d_n6, assign31120_e30693_d_n7, assign31120_e30693_d_n8, assign31120_e30693_d_n9, assign31120_e30693_d_n10, assign31120_e30693_d_n11, assign31120_e30693_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign31120_e30689: f64 = (locals.var_vgpp - locals.var_t1);
        let assign31120_e30691: f64 = (assign31120_e30689 + locals.var_tmf0);
        (assign31120_e30691, ((locals.var_vgpp_dn0 - locals.var_t1_dn0) + locals.var_tmf0_dn0), ((locals.var_vgpp_dn2 - locals.var_t1_dn2) + locals.var_tmf0_dn2), ((locals.var_vgpp_dn4 - locals.var_t1_dn4) + locals.var_tmf0_dn4), ((locals.var_vgpp_dn5 - locals.var_t1_dn5) + locals.var_tmf0_dn5), ((locals.var_vgpp_dn6 - locals.var_t1_dn6) + locals.var_tmf0_dn6), ((locals.var_vgpp_dn7 - locals.var_t1_dn7) + locals.var_tmf0_dn7), ((locals.var_vgpp_dn8 - locals.var_t1_dn8) + locals.var_tmf0_dn8), ((locals.var_vgpp_dn9 - locals.var_t1_dn9) + locals.var_tmf0_dn9), ((locals.var_vgpp_dn10 - locals.var_t1_dn10) + locals.var_tmf0_dn10), ((locals.var_vgpp_dn11 - locals.var_t1_dn11) + locals.var_tmf0_dn11), ((locals.var_vgpp_dn14 - locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign31120_e30693;
        locals.var_vds_dn0 = assign31120_e30693_d_n0;
        locals.var_vds_dn2 = assign31120_e30693_d_n2;
        locals.var_vds_dn4 = assign31120_e30693_d_n4;
        locals.var_vds_dn5 = assign31120_e30693_d_n5;
        locals.var_vds_dn6 = assign31120_e30693_d_n6;
        locals.var_vds_dn7 = assign31120_e30693_d_n7;
        locals.var_vds_dn8 = assign31120_e30693_d_n8;
        locals.var_vds_dn9 = assign31120_e30693_d_n9;
        locals.var_vds_dn10 = assign31120_e30693_d_n10;
        locals.var_vds_dn11 = assign31120_e30693_d_n11;
        locals.var_vds_dn14 = assign31120_e30693_d_n14;

        let (assign31130_e30703, assign31130_e30703_d_n0, assign31130_e30703_d_n2, assign31130_e30703_d_n4, assign31130_e30703_d_n5, assign31130_e30703_d_n6, assign31130_e30703_d_n7, assign31130_e30703_d_n8, assign31130_e30703_d_n9, assign31130_e30703_d_n10, assign31130_e30703_d_n11, assign31130_e30703_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31130_e30703;
        locals.var_t0_dn0 = assign31130_e30703_d_n0;
        locals.var_t0_dn2 = assign31130_e30703_d_n2;
        locals.var_t0_dn4 = assign31130_e30703_d_n4;
        locals.var_t0_dn5 = assign31130_e30703_d_n5;
        locals.var_t0_dn6 = assign31130_e30703_d_n6;
        locals.var_t0_dn7 = assign31130_e30703_d_n7;
        locals.var_t0_dn8 = assign31130_e30703_d_n8;
        locals.var_t0_dn9 = assign31130_e30703_d_n9;
        locals.var_t0_dn10 = assign31130_e30703_d_n10;
        locals.var_t0_dn11 = assign31130_e30703_d_n11;
        locals.var_t0_dn14 = assign31130_e30703_d_n14;

        let (assign31140_e30714, assign31140_e30714_d_n0, assign31140_e30714_d_n2, assign31140_e30714_d_n4, assign31140_e30714_d_n5, assign31140_e30714_d_n6, assign31140_e30714_d_n7, assign31140_e30714_d_n8, assign31140_e30714_d_n9, assign31140_e30714_d_n10, assign31140_e30714_d_n11, assign31140_e30714_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 == 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign31140_e30714;
        locals.var_vds_dn0 = assign31140_e30714_d_n0;
        locals.var_vds_dn2 = assign31140_e30714_d_n2;
        locals.var_vds_dn4 = assign31140_e30714_d_n4;
        locals.var_vds_dn5 = assign31140_e30714_d_n5;
        locals.var_vds_dn6 = assign31140_e30714_d_n6;
        locals.var_vds_dn7 = assign31140_e30714_d_n7;
        locals.var_vds_dn8 = assign31140_e30714_d_n8;
        locals.var_vds_dn9 = assign31140_e30714_d_n9;
        locals.var_vds_dn10 = assign31140_e30714_d_n10;
        locals.var_vds_dn11 = assign31140_e30714_d_n11;
        locals.var_vds_dn14 = assign31140_e30714_d_n14;

        let (assign31150_e30725, assign31150_e30725_d_n0, assign31150_e30725_d_n2, assign31150_e30725_d_n4, assign31150_e30725_d_n5, assign31150_e30725_d_n6, assign31150_e30725_d_n7, assign31150_e30725_d_n8, assign31150_e30725_d_n9, assign31150_e30725_d_n10, assign31150_e30725_d_n11, assign31150_e30725_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard707 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31150_e30725;
        locals.var_t0_dn0 = assign31150_e30725_d_n0;
        locals.var_t0_dn2 = assign31150_e30725_d_n2;
        locals.var_t0_dn4 = assign31150_e30725_d_n4;
        locals.var_t0_dn5 = assign31150_e30725_d_n5;
        locals.var_t0_dn6 = assign31150_e30725_d_n6;
        locals.var_t0_dn7 = assign31150_e30725_d_n7;
        locals.var_t0_dn8 = assign31150_e30725_d_n8;
        locals.var_t0_dn9 = assign31150_e30725_d_n9;
        locals.var_t0_dn10 = assign31150_e30725_d_n10;
        locals.var_t0_dn11 = assign31150_e30725_d_n11;
        locals.var_t0_dn14 = assign31150_e30725_d_n14;

        let (assign31160_e30734, assign31160_e30734_d_n0, assign31160_e30734_d_n2, assign31160_e30734_d_n4, assign31160_e30734_d_n5, assign31160_e30734_d_n6, assign31160_e30734_d_n7, assign31160_e30734_d_n8, assign31160_e30734_d_n9, assign31160_e30734_d_n10, assign31160_e30734_d_n11, assign31160_e30734_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 == 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn14,)
    }
};
        locals.var_vdseff = assign31160_e30734;
        locals.var_vdseff_dn0 = assign31160_e30734_d_n0;
        locals.var_vdseff_dn2 = assign31160_e30734_d_n2;
        locals.var_vdseff_dn4 = assign31160_e30734_d_n4;
        locals.var_vdseff_dn5 = assign31160_e30734_d_n5;
        locals.var_vdseff_dn6 = assign31160_e30734_d_n6;
        locals.var_vdseff_dn7 = assign31160_e30734_d_n7;
        locals.var_vdseff_dn8 = assign31160_e30734_d_n8;
        locals.var_vdseff_dn9 = assign31160_e30734_d_n9;
        locals.var_vdseff_dn10 = assign31160_e30734_d_n10;
        locals.var_vdseff_dn11 = assign31160_e30734_d_n11;
        locals.var_vdseff_dn14 = assign31160_e30734_d_n14;

        let assign31170_e30737: f64 = if locals.var_vds <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard713 = assign31170_e30737;

        let (assign31180_e30745, assign31180_e30745_d_n0, assign31180_e30745_d_n2, assign31180_e30745_d_n4, assign31180_e30745_d_n5, assign31180_e30745_d_n6, assign31180_e30745_d_n7, assign31180_e30745_d_n8, assign31180_e30745_d_n9, assign31180_e30745_d_n10, assign31180_e30745_d_n11, assign31180_e30745_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 != 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    }
};
        locals.var_phi_sl_dep = assign31180_e30745;
        locals.var_phi_sl_dep_dn0 = assign31180_e30745_d_n0;
        locals.var_phi_sl_dep_dn2 = assign31180_e30745_d_n2;
        locals.var_phi_sl_dep_dn4 = assign31180_e30745_d_n4;
        locals.var_phi_sl_dep_dn5 = assign31180_e30745_d_n5;
        locals.var_phi_sl_dep_dn6 = assign31180_e30745_d_n6;
        locals.var_phi_sl_dep_dn7 = assign31180_e30745_d_n7;
        locals.var_phi_sl_dep_dn8 = assign31180_e30745_d_n8;
        locals.var_phi_sl_dep_dn9 = assign31180_e30745_d_n9;
        locals.var_phi_sl_dep_dn10 = assign31180_e30745_d_n10;
        locals.var_phi_sl_dep_dn11 = assign31180_e30745_d_n11;
        locals.var_phi_sl_dep_dn14 = assign31180_e30745_d_n14;

        let (assign31190_e30753, assign31190_e30753_d_n0, assign31190_e30753_d_n2, assign31190_e30753_d_n4, assign31190_e30753_d_n5, assign31190_e30753_d_n6, assign31190_e30753_d_n7, assign31190_e30753_d_n8, assign31190_e30753_d_n9, assign31190_e30753_d_n10, assign31190_e30753_d_n11, assign31190_e30753_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31190_e30753;
        locals.var_phi_bl_dep_dn0 = assign31190_e30753_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31190_e30753_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31190_e30753_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31190_e30753_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31190_e30753_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31190_e30753_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31190_e30753_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31190_e30753_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31190_e30753_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31190_e30753_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31190_e30753_d_n14;

        let (assign31200_e30761, assign31200_e30761_d_n0, assign31200_e30761_d_n2, assign31200_e30761_d_n4, assign31200_e30761_d_n5, assign31200_e30761_d_n6, assign31200_e30761_d_n7, assign31200_e30761_d_n8, assign31200_e30761_d_n9, assign31200_e30761_d_n10, assign31200_e30761_d_n11, assign31200_e30761_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31200_e30761;
        locals.var_phi_jl_dep_dn0 = assign31200_e30761_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31200_e30761_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31200_e30761_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31200_e30761_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31200_e30761_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31200_e30761_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31200_e30761_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31200_e30761_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31200_e30761_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31200_e30761_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31200_e30761_d_n14;

        let (assign31210_e30769, assign31210_e30769_d_n0, assign31210_e30769_d_n2, assign31210_e30769_d_n4, assign31210_e30769_d_n5, assign31210_e30769_d_n6, assign31210_e30769_d_n7, assign31210_e30769_d_n8, assign31210_e30769_d_n9, assign31210_e30769_d_n10, assign31210_e30769_d_n11, assign31210_e30769_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 != 0.0)) {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn11, locals.var_q_sub0_dn14,)
    } else {
        (locals.var_q_subl, locals.var_q_subl_dn0, locals.var_q_subl_dn2, locals.var_q_subl_dn4, locals.var_q_subl_dn5, locals.var_q_subl_dn6, locals.var_q_subl_dn7, locals.var_q_subl_dn8, locals.var_q_subl_dn9, locals.var_q_subl_dn10, locals.var_q_subl_dn11, locals.var_q_subl_dn14,)
    }
};
        locals.var_q_subl = assign31210_e30769;
        locals.var_q_subl_dn0 = assign31210_e30769_d_n0;
        locals.var_q_subl_dn2 = assign31210_e30769_d_n2;
        locals.var_q_subl_dn4 = assign31210_e30769_d_n4;
        locals.var_q_subl_dn5 = assign31210_e30769_d_n5;
        locals.var_q_subl_dn6 = assign31210_e30769_d_n6;
        locals.var_q_subl_dn7 = assign31210_e30769_d_n7;
        locals.var_q_subl_dn8 = assign31210_e30769_d_n8;
        locals.var_q_subl_dn9 = assign31210_e30769_d_n9;
        locals.var_q_subl_dn10 = assign31210_e30769_d_n10;
        locals.var_q_subl_dn11 = assign31210_e30769_d_n11;
        locals.var_q_subl_dn14 = assign31210_e30769_d_n14;

        let (assign31220_e30777, assign31220_e30777_d_n0, assign31220_e30777_d_n2, assign31220_e30777_d_n4, assign31220_e30777_d_n5, assign31220_e30777_d_n6, assign31220_e30777_d_n7, assign31220_e30777_d_n8, assign31220_e30777_d_n9, assign31220_e30777_d_n10, assign31220_e30777_d_n11, assign31220_e30777_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 != 0.0)) {
        (locals.var_q_n0__blk540, locals.var_q_n0__blk540_dn0, locals.var_q_n0__blk540_dn2, locals.var_q_n0__blk540_dn4, locals.var_q_n0__blk540_dn5, locals.var_q_n0__blk540_dn6, locals.var_q_n0__blk540_dn7, locals.var_q_n0__blk540_dn8, locals.var_q_n0__blk540_dn9, locals.var_q_n0__blk540_dn10, locals.var_q_n0__blk540_dn11, locals.var_q_n0__blk540_dn14,)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn4, locals.var_q_nl_dn5, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn8, locals.var_q_nl_dn9, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn14,)
    }
};
        locals.var_q_nl = assign31220_e30777;
        locals.var_q_nl_dn0 = assign31220_e30777_d_n0;
        locals.var_q_nl_dn2 = assign31220_e30777_d_n2;
        locals.var_q_nl_dn4 = assign31220_e30777_d_n4;
        locals.var_q_nl_dn5 = assign31220_e30777_d_n5;
        locals.var_q_nl_dn6 = assign31220_e30777_d_n6;
        locals.var_q_nl_dn7 = assign31220_e30777_d_n7;
        locals.var_q_nl_dn8 = assign31220_e30777_d_n8;
        locals.var_q_nl_dn9 = assign31220_e30777_d_n9;
        locals.var_q_nl_dn10 = assign31220_e30777_d_n10;
        locals.var_q_nl_dn11 = assign31220_e30777_d_n11;
        locals.var_q_nl_dn14 = assign31220_e30777_d_n14;

        let (assign31230_e30785, assign31230_e30785_d_n0, assign31230_e30785_d_n2, assign31230_e30785_d_n4, assign31230_e30785_d_n5, assign31230_e30785_d_n6, assign31230_e30785_d_n7, assign31230_e30785_d_n8, assign31230_e30785_d_n9, assign31230_e30785_d_n10, assign31230_e30785_d_n11, assign31230_e30785_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 != 0.0)) {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn14,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
        locals.var_q_bl_dep = assign31230_e30785;
        locals.var_q_bl_dep_dn0 = assign31230_e30785_d_n0;
        locals.var_q_bl_dep_dn2 = assign31230_e30785_d_n2;
        locals.var_q_bl_dep_dn4 = assign31230_e30785_d_n4;
        locals.var_q_bl_dep_dn5 = assign31230_e30785_d_n5;
        locals.var_q_bl_dep_dn6 = assign31230_e30785_d_n6;
        locals.var_q_bl_dep_dn7 = assign31230_e30785_d_n7;
        locals.var_q_bl_dep_dn8 = assign31230_e30785_d_n8;
        locals.var_q_bl_dep_dn9 = assign31230_e30785_d_n9;
        locals.var_q_bl_dep_dn10 = assign31230_e30785_d_n10;
        locals.var_q_bl_dep_dn11 = assign31230_e30785_d_n11;
        locals.var_q_bl_dep_dn14 = assign31230_e30785_d_n14;

        let (assign31240_e30793, assign31240_e30793_d_n0, assign31240_e30793_d_n2, assign31240_e30793_d_n4, assign31240_e30793_d_n5, assign31240_e30793_d_n6, assign31240_e30793_d_n7, assign31240_e30793_d_n8, assign31240_e30793_d_n9, assign31240_e30793_d_n10, assign31240_e30793_d_n11, assign31240_e30793_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 != 0.0)) {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn11, locals.var_q_sub0_dep_dn14,)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn11, locals.var_q_subl_dep_dn14,)
    }
};
        locals.var_q_subl_dep = assign31240_e30793;
        locals.var_q_subl_dep_dn0 = assign31240_e30793_d_n0;
        locals.var_q_subl_dep_dn2 = assign31240_e30793_d_n2;
        locals.var_q_subl_dep_dn4 = assign31240_e30793_d_n4;
        locals.var_q_subl_dep_dn5 = assign31240_e30793_d_n5;
        locals.var_q_subl_dep_dn6 = assign31240_e30793_d_n6;
        locals.var_q_subl_dep_dn7 = assign31240_e30793_d_n7;
        locals.var_q_subl_dep_dn8 = assign31240_e30793_d_n8;
        locals.var_q_subl_dep_dn9 = assign31240_e30793_d_n9;
        locals.var_q_subl_dep_dn10 = assign31240_e30793_d_n10;
        locals.var_q_subl_dep_dn11 = assign31240_e30793_d_n11;
        locals.var_q_subl_dep_dn14 = assign31240_e30793_d_n14;

        let (assign31250_e30801, assign31250_e30801_d_n0, assign31250_e30801_d_n2, assign31250_e30801_d_n4, assign31250_e30801_d_n5, assign31250_e30801_d_n6, assign31250_e30801_d_n7, assign31250_e30801_d_n8, assign31250_e30801_d_n9, assign31250_e30801_d_n10, assign31250_e30801_d_n11, assign31250_e30801_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 != 0.0)) {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn11, locals.var_q_s0_dep_dn14,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn14,)
    }
};
        locals.var_q_sl_dep = assign31250_e30801;
        locals.var_q_sl_dep_dn0 = assign31250_e30801_d_n0;
        locals.var_q_sl_dep_dn2 = assign31250_e30801_d_n2;
        locals.var_q_sl_dep_dn4 = assign31250_e30801_d_n4;
        locals.var_q_sl_dep_dn5 = assign31250_e30801_d_n5;
        locals.var_q_sl_dep_dn6 = assign31250_e30801_d_n6;
        locals.var_q_sl_dep_dn7 = assign31250_e30801_d_n7;
        locals.var_q_sl_dep_dn8 = assign31250_e30801_d_n8;
        locals.var_q_sl_dep_dn9 = assign31250_e30801_d_n9;
        locals.var_q_sl_dep_dn10 = assign31250_e30801_d_n10;
        locals.var_q_sl_dep_dn11 = assign31250_e30801_d_n11;
        locals.var_q_sl_dep_dn14 = assign31250_e30801_d_n14;

        let (assign31260_e30809, assign31260_e30809_d_n0, assign31260_e30809_d_n2, assign31260_e30809_d_n4, assign31260_e30809_d_n5, assign31260_e30809_d_n6, assign31260_e30809_d_n7, assign31260_e30809_d_n8, assign31260_e30809_d_n9, assign31260_e30809_d_n10, assign31260_e30809_d_n11, assign31260_e30809_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 != 0.0)) {
        (locals.var_q_n0_cur, locals.var_q_n0_cur_dn0, locals.var_q_n0_cur_dn2, locals.var_q_n0_cur_dn4, locals.var_q_n0_cur_dn5, locals.var_q_n0_cur_dn6, locals.var_q_n0_cur_dn7, locals.var_q_n0_cur_dn8, locals.var_q_n0_cur_dn9, locals.var_q_n0_cur_dn10, locals.var_q_n0_cur_dn11, locals.var_q_n0_cur_dn14,)
    } else {
        (locals.var_q_nl_cur, locals.var_q_nl_cur_dn0, locals.var_q_nl_cur_dn2, locals.var_q_nl_cur_dn4, locals.var_q_nl_cur_dn5, locals.var_q_nl_cur_dn6, locals.var_q_nl_cur_dn7, locals.var_q_nl_cur_dn8, locals.var_q_nl_cur_dn9, locals.var_q_nl_cur_dn10, locals.var_q_nl_cur_dn11, locals.var_q_nl_cur_dn14,)
    }
};
        locals.var_q_nl_cur = assign31260_e30809;
        locals.var_q_nl_cur_dn0 = assign31260_e30809_d_n0;
        locals.var_q_nl_cur_dn2 = assign31260_e30809_d_n2;
        locals.var_q_nl_cur_dn4 = assign31260_e30809_d_n4;
        locals.var_q_nl_cur_dn5 = assign31260_e30809_d_n5;
        locals.var_q_nl_cur_dn6 = assign31260_e30809_d_n6;
        locals.var_q_nl_cur_dn7 = assign31260_e30809_d_n7;
        locals.var_q_nl_cur_dn8 = assign31260_e30809_d_n8;
        locals.var_q_nl_cur_dn9 = assign31260_e30809_d_n9;
        locals.var_q_nl_cur_dn10 = assign31260_e30809_d_n10;
        locals.var_q_nl_cur_dn11 = assign31260_e30809_d_n11;
        locals.var_q_nl_cur_dn14 = assign31260_e30809_d_n14;

    }

    pub(super) fn stamp_transient_block_91(
        locals: &mut StampLocals,
    ) {
        let (assign31270_e30831, assign31270_e30831_d_n0, assign31270_e30831_d_n2, assign31270_e30831_d_n4, assign31270_e30831_d_n5, assign31270_e30831_d_n6, assign31270_e30831_d_n7, assign31270_e30831_d_n8, assign31270_e30831_d_n9, assign31270_e30831_d_n10, assign31270_e30831_d_n11, assign31270_e30831_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) {
        let assign31270_e30818: f64 = (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc);
        let assign31270_e30821: f64 = (locals.var_ef_nsubc + locals.var_uc_ndepm);
        let assign31270_e30822: f64 = (assign31270_e30818 / assign31270_e30821);
        let assign31270_e30825: f64 = (locals.var_vds - locals.var_vbscl__blk437);
        let assign31270_e30827: f64 = (assign31270_e30825 + locals.var_vbi_dep);
        let assign31270_e30828: f64 = (assign31270_e30822 * assign31270_e30827);
        let assign31270_e30829: f64 = (assign31270_e30828).sqrt();
        (assign31270_e30829, ((((((((locals.var_c_2esipq_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn0)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn0 + locals.var_uc_ndepm_dn0))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn0 - locals.var_vbscl__blk437_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign31270_e30829)), ((((((((locals.var_c_2esipq_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn2)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn2 + locals.var_uc_ndepm_dn2))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn2 - locals.var_vbscl__blk437_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign31270_e30829)), ((((((((locals.var_c_2esipq_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn4)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn4 + locals.var_uc_ndepm_dn4))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn4 - locals.var_vbscl__blk437_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign31270_e30829)), ((((((((locals.var_c_2esipq_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn5)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn5 + locals.var_uc_ndepm_dn5))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn5 - locals.var_vbscl__blk437_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign31270_e30829)), ((((((((locals.var_c_2esipq_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn6)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn6 + locals.var_uc_ndepm_dn6))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn6 - locals.var_vbscl__blk437_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign31270_e30829)), ((((((((locals.var_c_2esipq_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn7)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn7 + locals.var_uc_ndepm_dn7))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn7 - locals.var_vbscl__blk437_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign31270_e30829)), ((((((((locals.var_c_2esipq_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn8)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn8 + locals.var_uc_ndepm_dn8))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn8 - locals.var_vbscl__blk437_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign31270_e30829)), ((((((((locals.var_c_2esipq_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn9)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn9 + locals.var_uc_ndepm_dn9))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn9 - locals.var_vbscl__blk437_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign31270_e30829)), ((((((((locals.var_c_2esipq_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn10)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn10 + locals.var_uc_ndepm_dn10))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn10 - locals.var_vbscl__blk437_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign31270_e30829)), ((((((((locals.var_c_2esipq_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn11)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn11 + locals.var_uc_ndepm_dn11))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn11 - locals.var_vbscl__blk437_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign31270_e30829)), ((((((((locals.var_c_2esipq_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn14)) * assign31270_e30821) - (assign31270_e30818 * (locals.var_ef_nsubc_dn14 + locals.var_uc_ndepm_dn14))) / (assign31270_e30821 * assign31270_e30821)) * assign31270_e30827) + (assign31270_e30822 * ((locals.var_vds_dn14 - locals.var_vbscl__blk437_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign31270_e30829)),)
    } else {
        (locals.var_w_bsubl, locals.var_w_bsubl_dn0, locals.var_w_bsubl_dn2, locals.var_w_bsubl_dn4, locals.var_w_bsubl_dn5, locals.var_w_bsubl_dn6, locals.var_w_bsubl_dn7, locals.var_w_bsubl_dn8, locals.var_w_bsubl_dn9, locals.var_w_bsubl_dn10, locals.var_w_bsubl_dn11, locals.var_w_bsubl_dn14,)
    }
};
        locals.var_w_bsubl = assign31270_e30831;
        locals.var_w_bsubl_dn0 = assign31270_e30831_d_n0;
        locals.var_w_bsubl_dn2 = assign31270_e30831_d_n2;
        locals.var_w_bsubl_dn4 = assign31270_e30831_d_n4;
        locals.var_w_bsubl_dn5 = assign31270_e30831_d_n5;
        locals.var_w_bsubl_dn6 = assign31270_e30831_d_n6;
        locals.var_w_bsubl_dn7 = assign31270_e30831_d_n7;
        locals.var_w_bsubl_dn8 = assign31270_e30831_d_n8;
        locals.var_w_bsubl_dn9 = assign31270_e30831_d_n9;
        locals.var_w_bsubl_dn10 = assign31270_e30831_d_n10;
        locals.var_w_bsubl_dn11 = assign31270_e30831_d_n11;
        locals.var_w_bsubl_dn14 = assign31270_e30831_d_n14;

        let assign31280_e30834: f64 = if locals.var_w_bsubl > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard714 = assign31280_e30834;

        let (assign31290_e30845, assign31290_e30845_d_n0, assign31290_e30845_d_n2, assign31290_e30845_d_n4, assign31290_e30845_d_n5, assign31290_e30845_d_n6, assign31290_e30845_d_n7, assign31290_e30845_d_n8, assign31290_e30845_d_n9, assign31290_e30845_d_n10, assign31290_e30845_d_n11, assign31290_e30845_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
        locals.var_vgp0 = assign31290_e30845;
        locals.var_vgp0_dn0 = assign31290_e30845_d_n0;
        locals.var_vgp0_dn2 = assign31290_e30845_d_n2;
        locals.var_vgp0_dn4 = assign31290_e30845_d_n4;
        locals.var_vgp0_dn5 = assign31290_e30845_d_n5;
        locals.var_vgp0_dn6 = assign31290_e30845_d_n6;
        locals.var_vgp0_dn7 = assign31290_e30845_d_n7;
        locals.var_vgp0_dn8 = assign31290_e30845_d_n8;
        locals.var_vgp0_dn9 = assign31290_e30845_d_n9;
        locals.var_vgp0_dn10 = assign31290_e30845_d_n10;
        locals.var_vgp0_dn11 = assign31290_e30845_d_n11;
        locals.var_vgp0_dn14 = assign31290_e30845_d_n14;

        let (assign31300_e30856, assign31300_e30856_d_n0, assign31300_e30856_d_n2, assign31300_e30856_d_n4, assign31300_e30856_d_n5, assign31300_e30856_d_n6, assign31300_e30856_d_n7, assign31300_e30856_d_n8, assign31300_e30856_d_n9, assign31300_e30856_d_n10, assign31300_e30856_d_n11, assign31300_e30856_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign31300_e30856;
        locals.var_w_bl_dn0 = assign31300_e30856_d_n0;
        locals.var_w_bl_dn2 = assign31300_e30856_d_n2;
        locals.var_w_bl_dn4 = assign31300_e30856_d_n4;
        locals.var_w_bl_dn5 = assign31300_e30856_d_n5;
        locals.var_w_bl_dn6 = assign31300_e30856_d_n6;
        locals.var_w_bl_dn7 = assign31300_e30856_d_n7;
        locals.var_w_bl_dn8 = assign31300_e30856_d_n8;
        locals.var_w_bl_dn9 = assign31300_e30856_d_n9;
        locals.var_w_bl_dn10 = assign31300_e30856_d_n10;
        locals.var_w_bl_dn11 = assign31300_e30856_d_n11;
        locals.var_w_bl_dn14 = assign31300_e30856_d_n14;

        let (assign31310_e30867, assign31310_e30867_d_n0, assign31310_e30867_d_n2, assign31310_e30867_d_n4, assign31310_e30867_d_n5, assign31310_e30867_d_n6, assign31310_e30867_d_n7, assign31310_e30867_d_n8, assign31310_e30867_d_n9, assign31310_e30867_d_n10, assign31310_e30867_d_n11, assign31310_e30867_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31310_e30867;
        locals.var_phi_bl_dep_dn0 = assign31310_e30867_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31310_e30867_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31310_e30867_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31310_e30867_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31310_e30867_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31310_e30867_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31310_e30867_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31310_e30867_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31310_e30867_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31310_e30867_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31310_e30867_d_n14;

        let (assign31320_e30878, assign31320_e30878_d_n0, assign31320_e30878_d_n2, assign31320_e30878_d_n4, assign31320_e30878_d_n5, assign31320_e30878_d_n6, assign31320_e30878_d_n7, assign31320_e30878_d_n8, assign31320_e30878_d_n9, assign31320_e30878_d_n10, assign31320_e30878_d_n11, assign31320_e30878_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vds_maxbl, locals.var_vds_maxbl_dn0, locals.var_vds_maxbl_dn2, locals.var_vds_maxbl_dn4, locals.var_vds_maxbl_dn5, locals.var_vds_maxbl_dn6, locals.var_vds_maxbl_dn7, locals.var_vds_maxbl_dn8, locals.var_vds_maxbl_dn9, locals.var_vds_maxbl_dn10, locals.var_vds_maxbl_dn11, locals.var_vds_maxbl_dn14,)
    }
};
        locals.var_vds_maxbl = assign31320_e30878;
        locals.var_vds_maxbl_dn0 = assign31320_e30878_d_n0;
        locals.var_vds_maxbl_dn2 = assign31320_e30878_d_n2;
        locals.var_vds_maxbl_dn4 = assign31320_e30878_d_n4;
        locals.var_vds_maxbl_dn5 = assign31320_e30878_d_n5;
        locals.var_vds_maxbl_dn6 = assign31320_e30878_d_n6;
        locals.var_vds_maxbl_dn7 = assign31320_e30878_d_n7;
        locals.var_vds_maxbl_dn8 = assign31320_e30878_d_n8;
        locals.var_vds_maxbl_dn9 = assign31320_e30878_d_n9;
        locals.var_vds_maxbl_dn10 = assign31320_e30878_d_n10;
        locals.var_vds_maxbl_dn11 = assign31320_e30878_d_n11;
        locals.var_vds_maxbl_dn14 = assign31320_e30878_d_n14;

        let (assign31330_e30895, assign31330_e30895_d_n0, assign31330_e30895_d_n2, assign31330_e30895_d_n4, assign31330_e30895_d_n5, assign31330_e30895_d_n6, assign31330_e30895_d_n7, assign31330_e30895_d_n8, assign31330_e30895_d_n9, assign31330_e30895_d_n10, assign31330_e30895_d_n11, assign31330_e30895_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31330_e30890: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl);
        let assign31330_e30892: f64 = (assign31330_e30890 * locals.var_w_bl);
        let assign31330_e30893: f64 = (locals.var_phi_bl_dep - assign31330_e30892);
        (assign31330_e30893, (locals.var_phi_bl_dep_dn0 - ((((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn0)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn0))), (locals.var_phi_bl_dep_dn2 - ((((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn2)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn2))), (locals.var_phi_bl_dep_dn4 - ((((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn4)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn4))), (locals.var_phi_bl_dep_dn5 - ((((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn5)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn5))), (locals.var_phi_bl_dep_dn6 - ((((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn6)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn6))), (locals.var_phi_bl_dep_dn7 - ((((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn7)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn7))), (locals.var_phi_bl_dep_dn8 - ((((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn8)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn8))), (locals.var_phi_bl_dep_dn9 - ((((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn9)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn9))), (locals.var_phi_bl_dep_dn10 - ((((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn10)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn10))), (locals.var_phi_bl_dep_dn11 - ((((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn11)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn11))), (locals.var_phi_bl_dep_dn14 - ((((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn14)) * locals.var_w_bl) + (assign31330_e30890 * locals.var_w_bl_dn14))),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31330_e30895;
        locals.var_phi_jl_dep_dn0 = assign31330_e30895_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31330_e30895_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31330_e30895_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31330_e30895_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31330_e30895_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31330_e30895_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31330_e30895_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31330_e30895_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31330_e30895_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31330_e30895_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31330_e30895_d_n14;

        let (assign31340_e30906,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_vgp0,)
    } else {
        (locals.var_vgp0old,)
    }
};
        locals.var_vgp0old = assign31340_e30906;

        let (assign31350_e30917,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_phi_jl_dep,)
    } else {
        (locals.var_phi_jl_dep_old,)
    }
};
        locals.var_phi_jl_dep_old = assign31350_e30917;

        let (assign31360_e30930, assign31360_e30930_d_n0, assign31360_e30930_d_n2, assign31360_e30930_d_n4, assign31360_e30930_d_n5, assign31360_e30930_d_n6, assign31360_e30930_d_n7, assign31360_e30930_d_n8, assign31360_e30930_d_n9, assign31360_e30930_d_n10, assign31360_e30930_d_n11, assign31360_e30930_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31360_e30928: f64 = (locals.var_w_bl * locals.var_q_ndepm);
        (assign31360_e30928, ((locals.var_w_bl_dn0 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn0)), ((locals.var_w_bl_dn2 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn2)), ((locals.var_w_bl_dn4 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn4)), ((locals.var_w_bl_dn5 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn5)), ((locals.var_w_bl_dn6 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn6)), ((locals.var_w_bl_dn7 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn7)), ((locals.var_w_bl_dn8 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn8)), ((locals.var_w_bl_dn9 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn9)), ((locals.var_w_bl_dn10 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn10)), ((locals.var_w_bl_dn11 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn11)), ((locals.var_w_bl_dn14 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
        locals.var_q_bl_dep = assign31360_e30930;
        locals.var_q_bl_dep_dn0 = assign31360_e30930_d_n0;
        locals.var_q_bl_dep_dn2 = assign31360_e30930_d_n2;
        locals.var_q_bl_dep_dn4 = assign31360_e30930_d_n4;
        locals.var_q_bl_dep_dn5 = assign31360_e30930_d_n5;
        locals.var_q_bl_dep_dn6 = assign31360_e30930_d_n6;
        locals.var_q_bl_dep_dn7 = assign31360_e30930_d_n7;
        locals.var_q_bl_dep_dn8 = assign31360_e30930_d_n8;
        locals.var_q_bl_dep_dn9 = assign31360_e30930_d_n9;
        locals.var_q_bl_dep_dn10 = assign31360_e30930_d_n10;
        locals.var_q_bl_dep_dn11 = assign31360_e30930_d_n11;
        locals.var_q_bl_dep_dn14 = assign31360_e30930_d_n14;

        let (assign31370_e30941,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign31370_e30941;

    }

    pub(super) fn stamp_transient_block_92(
        locals: &mut StampLocals,
    ) {
        let mut assign31380_loop_guard: usize = 0;
        while {
            let assign31380_cond_e30953: f64 = (150.0 + 1.0);
            let assign31380_cond_e30955: f64 = if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_lp_s0 <= assign31380_cond_e30953)) { 1.0 } else { 0.0 };
            assign31380_cond_e30955 != 0.0
        } {
            assign31380_loop_guard += 1;
            assert!(assign31380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign31380_body0_e30971, assign31380_body0_e30971_d_n0, assign31380_body0_e30971_d_n2, assign31380_body0_e30971_d_n4, assign31380_body0_e30971_d_n5, assign31380_body0_e30971_d_n6, assign31380_body0_e30971_d_n7, assign31380_body0_e30971_d_n8, assign31380_body0_e30971_d_n9, assign31380_body0_e30971_d_n10, assign31380_body0_e30971_d_n11, assign31380_body0_e30971_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body0_e30967: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        let assign31380_body0_e30968: f64 = (locals.var_c_2esipq_ndepm * assign31380_body0_e30967);
        let assign31380_body0_e30969: f64 = (assign31380_body0_e30968).sqrt();
        (assign31380_body0_e30969, (((locals.var_c_2esipq_ndepm_dn0 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0))) / (2.0 * assign31380_body0_e30969)), (((locals.var_c_2esipq_ndepm_dn2 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2))) / (2.0 * assign31380_body0_e30969)), (((locals.var_c_2esipq_ndepm_dn4 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4))) / (2.0 * assign31380_body0_e30969)), (((locals.var_c_2esipq_ndepm_dn5 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5))) / (2.0 * assign31380_body0_e30969)), (((locals.var_c_2esipq_ndepm_dn6 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6))) / (2.0 * assign31380_body0_e30969)), (((locals.var_c_2esipq_ndepm_dn7 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7))) / (2.0 * assign31380_body0_e30969)), (((locals.var_c_2esipq_ndepm_dn8 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8))) / (2.0 * assign31380_body0_e30969)), (((locals.var_c_2esipq_ndepm_dn9 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9))) / (2.0 * assign31380_body0_e30969)), (((locals.var_c_2esipq_ndepm_dn10 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10))) / (2.0 * assign31380_body0_e30969)), (((locals.var_c_2esipq_ndepm_dn11 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_jl_dep_dn11))) / (2.0 * assign31380_body0_e30969)), (((locals.var_c_2esipq_ndepm_dn14 * assign31380_body0_e30967) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_jl_dep_dn14))) / (2.0 * assign31380_body0_e30969)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
            locals.var_w_bl = assign31380_body0_e30971;
            locals.var_w_bl_dn0 = assign31380_body0_e30971_d_n0;
            locals.var_w_bl_dn2 = assign31380_body0_e30971_d_n2;
            locals.var_w_bl_dn4 = assign31380_body0_e30971_d_n4;
            locals.var_w_bl_dn5 = assign31380_body0_e30971_d_n5;
            locals.var_w_bl_dn6 = assign31380_body0_e30971_d_n6;
            locals.var_w_bl_dn7 = assign31380_body0_e30971_d_n7;
            locals.var_w_bl_dn8 = assign31380_body0_e30971_d_n8;
            locals.var_w_bl_dn9 = assign31380_body0_e30971_d_n9;
            locals.var_w_bl_dn10 = assign31380_body0_e30971_d_n10;
            locals.var_w_bl_dn11 = assign31380_body0_e30971_d_n11;
            locals.var_w_bl_dn14 = assign31380_body0_e30971_d_n14;
            let assign31380_body1_e30975: f64 = (locals.var_uc_depthn - 1e-8);
            let assign31380_body1_e30980: f64 = if ((locals.var_w_bl > assign31380_body1_e30975) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard715 = assign31380_body1_e30980;
            let (assign31380_body2_e30997, assign31380_body2_e30997_d_n0, assign31380_body2_e30997_d_n2, assign31380_body2_e30997_d_n4, assign31380_body2_e30997_d_n5, assign31380_body2_e30997_d_n6, assign31380_body2_e30997_d_n7, assign31380_body2_e30997_d_n8, assign31380_body2_e30997_d_n9, assign31380_body2_e30997_d_n10, assign31380_body2_e30997_d_n11, assign31380_body2_e30997_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body2_e30993: f64 = (locals.var_w_bl - locals.var_uc_depthn);
        let assign31380_body2_e30995: f64 = (assign31380_body2_e30993 + 1e-8);
        (assign31380_body2_e30995, (locals.var_w_bl_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_bl_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_bl_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_bl_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_bl_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_bl_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_bl_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_bl_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_bl_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_bl_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_bl_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign31380_body2_e30997;
            locals.var_tmf1_dn0 = assign31380_body2_e30997_d_n0;
            locals.var_tmf1_dn2 = assign31380_body2_e30997_d_n2;
            locals.var_tmf1_dn4 = assign31380_body2_e30997_d_n4;
            locals.var_tmf1_dn5 = assign31380_body2_e30997_d_n5;
            locals.var_tmf1_dn6 = assign31380_body2_e30997_d_n6;
            locals.var_tmf1_dn7 = assign31380_body2_e30997_d_n7;
            locals.var_tmf1_dn8 = assign31380_body2_e30997_d_n8;
            locals.var_tmf1_dn9 = assign31380_body2_e30997_d_n9;
            locals.var_tmf1_dn10 = assign31380_body2_e30997_d_n10;
            locals.var_tmf1_dn11 = assign31380_body2_e30997_d_n11;
            locals.var_tmf1_dn14 = assign31380_body2_e30997_d_n14;
            let (assign31380_body3_e31012, assign31380_body3_e31012_d_n0, assign31380_body3_e31012_d_n2, assign31380_body3_e31012_d_n4, assign31380_body3_e31012_d_n5, assign31380_body3_e31012_d_n6, assign31380_body3_e31012_d_n7, assign31380_body3_e31012_d_n8, assign31380_body3_e31012_d_n9, assign31380_body3_e31012_d_n10, assign31380_body3_e31012_d_n11, assign31380_body3_e31012_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body3_e31010: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign31380_body3_e31010, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
            locals.var_x2 = assign31380_body3_e31012;
            locals.var_x2_dn0 = assign31380_body3_e31012_d_n0;
            locals.var_x2_dn2 = assign31380_body3_e31012_d_n2;
            locals.var_x2_dn4 = assign31380_body3_e31012_d_n4;
            locals.var_x2_dn5 = assign31380_body3_e31012_d_n5;
            locals.var_x2_dn6 = assign31380_body3_e31012_d_n6;
            locals.var_x2_dn7 = assign31380_body3_e31012_d_n7;
            locals.var_x2_dn8 = assign31380_body3_e31012_d_n8;
            locals.var_x2_dn9 = assign31380_body3_e31012_d_n9;
            locals.var_x2_dn10 = assign31380_body3_e31012_d_n10;
            locals.var_x2_dn11 = assign31380_body3_e31012_d_n11;
            locals.var_x2_dn14 = assign31380_body3_e31012_d_n14;
            let (assign31380_body4_e31027, assign31380_body4_e31027_d_n0, assign31380_body4_e31027_d_n2, assign31380_body4_e31027_d_n4, assign31380_body4_e31027_d_n5, assign31380_body4_e31027_d_n6, assign31380_body4_e31027_d_n7, assign31380_body4_e31027_d_n8, assign31380_body4_e31027_d_n9, assign31380_body4_e31027_d_n10, assign31380_body4_e31027_d_n11, assign31380_body4_e31027_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body4_e31025: f64 = (1e-8 * 1e-8);
        (assign31380_body4_e31025, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
            locals.var_xmax2 = assign31380_body4_e31027;
            locals.var_xmax2_dn0 = assign31380_body4_e31027_d_n0;
            locals.var_xmax2_dn2 = assign31380_body4_e31027_d_n2;
            locals.var_xmax2_dn4 = assign31380_body4_e31027_d_n4;
            locals.var_xmax2_dn5 = assign31380_body4_e31027_d_n5;
            locals.var_xmax2_dn6 = assign31380_body4_e31027_d_n6;
            locals.var_xmax2_dn7 = assign31380_body4_e31027_d_n7;
            locals.var_xmax2_dn8 = assign31380_body4_e31027_d_n8;
            locals.var_xmax2_dn9 = assign31380_body4_e31027_d_n9;
            locals.var_xmax2_dn10 = assign31380_body4_e31027_d_n10;
            locals.var_xmax2_dn11 = assign31380_body4_e31027_d_n11;
            locals.var_xmax2_dn14 = assign31380_body4_e31027_d_n14;
            let (assign31380_body5_e31040, assign31380_body5_e31040_d_n0, assign31380_body5_e31040_d_n2, assign31380_body5_e31040_d_n4, assign31380_body5_e31040_d_n5, assign31380_body5_e31040_d_n6, assign31380_body5_e31040_d_n7, assign31380_body5_e31040_d_n8, assign31380_body5_e31040_d_n9, assign31380_body5_e31040_d_n10, assign31380_body5_e31040_d_n11, assign31380_body5_e31040_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31380_body5_e31040;
            locals.var_xp_dn0 = assign31380_body5_e31040_d_n0;
            locals.var_xp_dn2 = assign31380_body5_e31040_d_n2;
            locals.var_xp_dn4 = assign31380_body5_e31040_d_n4;
            locals.var_xp_dn5 = assign31380_body5_e31040_d_n5;
            locals.var_xp_dn6 = assign31380_body5_e31040_d_n6;
            locals.var_xp_dn7 = assign31380_body5_e31040_d_n7;
            locals.var_xp_dn8 = assign31380_body5_e31040_d_n8;
            locals.var_xp_dn9 = assign31380_body5_e31040_d_n9;
            locals.var_xp_dn10 = assign31380_body5_e31040_d_n10;
            locals.var_xp_dn11 = assign31380_body5_e31040_d_n11;
            locals.var_xp_dn14 = assign31380_body5_e31040_d_n14;
            let (assign31380_body6_e31053, assign31380_body6_e31053_d_n0, assign31380_body6_e31053_d_n2, assign31380_body6_e31053_d_n4, assign31380_body6_e31053_d_n5, assign31380_body6_e31053_d_n6, assign31380_body6_e31053_d_n7, assign31380_body6_e31053_d_n8, assign31380_body6_e31053_d_n9, assign31380_body6_e31053_d_n10, assign31380_body6_e31053_d_n11, assign31380_body6_e31053_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31380_body6_e31053;
            locals.var_xmp_dn0 = assign31380_body6_e31053_d_n0;
            locals.var_xmp_dn2 = assign31380_body6_e31053_d_n2;
            locals.var_xmp_dn4 = assign31380_body6_e31053_d_n4;
            locals.var_xmp_dn5 = assign31380_body6_e31053_d_n5;
            locals.var_xmp_dn6 = assign31380_body6_e31053_d_n6;
            locals.var_xmp_dn7 = assign31380_body6_e31053_d_n7;
            locals.var_xmp_dn8 = assign31380_body6_e31053_d_n8;
            locals.var_xmp_dn9 = assign31380_body6_e31053_d_n9;
            locals.var_xmp_dn10 = assign31380_body6_e31053_d_n10;
            locals.var_xmp_dn11 = assign31380_body6_e31053_d_n11;
            locals.var_xmp_dn14 = assign31380_body6_e31053_d_n14;
            let (assign31380_body7_e31066,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign31380_body7_e31066;
            let (assign31380_body8_e31079,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31380_body8_e31079;
            let (assign31380_body9_e31092, assign31380_body9_e31092_d_n0, assign31380_body9_e31092_d_n2, assign31380_body9_e31092_d_n4, assign31380_body9_e31092_d_n5, assign31380_body9_e31092_d_n6, assign31380_body9_e31092_d_n7, assign31380_body9_e31092_d_n8, assign31380_body9_e31092_d_n9, assign31380_body9_e31092_d_n10, assign31380_body9_e31092_d_n11, assign31380_body9_e31092_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign31380_body9_e31092;
            locals.var_arg_dn0 = assign31380_body9_e31092_d_n0;
            locals.var_arg_dn2 = assign31380_body9_e31092_d_n2;
            locals.var_arg_dn4 = assign31380_body9_e31092_d_n4;
            locals.var_arg_dn5 = assign31380_body9_e31092_d_n5;
            locals.var_arg_dn6 = assign31380_body9_e31092_d_n6;
            locals.var_arg_dn7 = assign31380_body9_e31092_d_n7;
            locals.var_arg_dn8 = assign31380_body9_e31092_d_n8;
            locals.var_arg_dn9 = assign31380_body9_e31092_d_n9;
            locals.var_arg_dn10 = assign31380_body9_e31092_d_n10;
            locals.var_arg_dn11 = assign31380_body9_e31092_d_n11;
            locals.var_arg_dn14 = assign31380_body9_e31092_d_n14;
            let (assign31380_body10_e31105, assign31380_body10_e31105_d_n0, assign31380_body10_e31105_d_n2, assign31380_body10_e31105_d_n4, assign31380_body10_e31105_d_n5, assign31380_body10_e31105_d_n6, assign31380_body10_e31105_d_n7, assign31380_body10_e31105_d_n8, assign31380_body10_e31105_d_n9, assign31380_body10_e31105_d_n10, assign31380_body10_e31105_d_n11, assign31380_body10_e31105_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31380_body10_e31105;
            locals.var_dnm_dn0 = assign31380_body10_e31105_d_n0;
            locals.var_dnm_dn2 = assign31380_body10_e31105_d_n2;
            locals.var_dnm_dn4 = assign31380_body10_e31105_d_n4;
            locals.var_dnm_dn5 = assign31380_body10_e31105_d_n5;
            locals.var_dnm_dn6 = assign31380_body10_e31105_d_n6;
            locals.var_dnm_dn7 = assign31380_body10_e31105_d_n7;
            locals.var_dnm_dn8 = assign31380_body10_e31105_d_n8;
            locals.var_dnm_dn9 = assign31380_body10_e31105_d_n9;
            locals.var_dnm_dn10 = assign31380_body10_e31105_d_n10;
            locals.var_dnm_dn11 = assign31380_body10_e31105_d_n11;
            locals.var_dnm_dn14 = assign31380_body10_e31105_d_n14;
            let (assign31380_body11_e31120, assign31380_body11_e31120_d_n0, assign31380_body11_e31120_d_n2, assign31380_body11_e31120_d_n4, assign31380_body11_e31120_d_n5, assign31380_body11_e31120_d_n6, assign31380_body11_e31120_d_n7, assign31380_body11_e31120_d_n8, assign31380_body11_e31120_d_n9, assign31380_body11_e31120_d_n10, assign31380_body11_e31120_d_n11, assign31380_body11_e31120_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body11_e31118: f64 = (locals.var_xp * locals.var_x2);
        (assign31380_body11_e31118, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31380_body11_e31120;
            locals.var_xp_dn0 = assign31380_body11_e31120_d_n0;
            locals.var_xp_dn2 = assign31380_body11_e31120_d_n2;
            locals.var_xp_dn4 = assign31380_body11_e31120_d_n4;
            locals.var_xp_dn5 = assign31380_body11_e31120_d_n5;
            locals.var_xp_dn6 = assign31380_body11_e31120_d_n6;
            locals.var_xp_dn7 = assign31380_body11_e31120_d_n7;
            locals.var_xp_dn8 = assign31380_body11_e31120_d_n8;
            locals.var_xp_dn9 = assign31380_body11_e31120_d_n9;
            locals.var_xp_dn10 = assign31380_body11_e31120_d_n10;
            locals.var_xp_dn11 = assign31380_body11_e31120_d_n11;
            locals.var_xp_dn14 = assign31380_body11_e31120_d_n14;
            let (assign31380_body12_e31135, assign31380_body12_e31135_d_n0, assign31380_body12_e31135_d_n2, assign31380_body12_e31135_d_n4, assign31380_body12_e31135_d_n5, assign31380_body12_e31135_d_n6, assign31380_body12_e31135_d_n7, assign31380_body12_e31135_d_n8, assign31380_body12_e31135_d_n9, assign31380_body12_e31135_d_n10, assign31380_body12_e31135_d_n11, assign31380_body12_e31135_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body12_e31133: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign31380_body12_e31133, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31380_body12_e31135;
            locals.var_xmp_dn0 = assign31380_body12_e31135_d_n0;
            locals.var_xmp_dn2 = assign31380_body12_e31135_d_n2;
            locals.var_xmp_dn4 = assign31380_body12_e31135_d_n4;
            locals.var_xmp_dn5 = assign31380_body12_e31135_d_n5;
            locals.var_xmp_dn6 = assign31380_body12_e31135_d_n6;
            locals.var_xmp_dn7 = assign31380_body12_e31135_d_n7;
            locals.var_xmp_dn8 = assign31380_body12_e31135_d_n8;
            locals.var_xmp_dn9 = assign31380_body12_e31135_d_n9;
            locals.var_xmp_dn10 = assign31380_body12_e31135_d_n10;
            locals.var_xmp_dn11 = assign31380_body12_e31135_d_n11;
            locals.var_xmp_dn14 = assign31380_body12_e31135_d_n14;
            let (assign31380_body13_e31150, assign31380_body13_e31150_d_n0, assign31380_body13_e31150_d_n2, assign31380_body13_e31150_d_n4, assign31380_body13_e31150_d_n5, assign31380_body13_e31150_d_n6, assign31380_body13_e31150_d_n7, assign31380_body13_e31150_d_n8, assign31380_body13_e31150_d_n9, assign31380_body13_e31150_d_n10, assign31380_body13_e31150_d_n11, assign31380_body13_e31150_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body13_e31148: f64 = (locals.var_xp * locals.var_x2);
        (assign31380_body13_e31148, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31380_body13_e31150;
            locals.var_xp_dn0 = assign31380_body13_e31150_d_n0;
            locals.var_xp_dn2 = assign31380_body13_e31150_d_n2;
            locals.var_xp_dn4 = assign31380_body13_e31150_d_n4;
            locals.var_xp_dn5 = assign31380_body13_e31150_d_n5;
            locals.var_xp_dn6 = assign31380_body13_e31150_d_n6;
            locals.var_xp_dn7 = assign31380_body13_e31150_d_n7;
            locals.var_xp_dn8 = assign31380_body13_e31150_d_n8;
            locals.var_xp_dn9 = assign31380_body13_e31150_d_n9;
            locals.var_xp_dn10 = assign31380_body13_e31150_d_n10;
            locals.var_xp_dn11 = assign31380_body13_e31150_d_n11;
            locals.var_xp_dn14 = assign31380_body13_e31150_d_n14;
            let (assign31380_body14_e31165, assign31380_body14_e31165_d_n0, assign31380_body14_e31165_d_n2, assign31380_body14_e31165_d_n4, assign31380_body14_e31165_d_n5, assign31380_body14_e31165_d_n6, assign31380_body14_e31165_d_n7, assign31380_body14_e31165_d_n8, assign31380_body14_e31165_d_n9, assign31380_body14_e31165_d_n10, assign31380_body14_e31165_d_n11, assign31380_body14_e31165_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body14_e31163: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign31380_body14_e31163, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31380_body14_e31165;
            locals.var_xmp_dn0 = assign31380_body14_e31165_d_n0;
            locals.var_xmp_dn2 = assign31380_body14_e31165_d_n2;
            locals.var_xmp_dn4 = assign31380_body14_e31165_d_n4;
            locals.var_xmp_dn5 = assign31380_body14_e31165_d_n5;
            locals.var_xmp_dn6 = assign31380_body14_e31165_d_n6;
            locals.var_xmp_dn7 = assign31380_body14_e31165_d_n7;
            locals.var_xmp_dn8 = assign31380_body14_e31165_d_n8;
            locals.var_xmp_dn9 = assign31380_body14_e31165_d_n9;
            locals.var_xmp_dn10 = assign31380_body14_e31165_d_n10;
            locals.var_xmp_dn11 = assign31380_body14_e31165_d_n11;
            locals.var_xmp_dn14 = assign31380_body14_e31165_d_n14;
            let (assign31380_body15_e31180, assign31380_body15_e31180_d_n0, assign31380_body15_e31180_d_n2, assign31380_body15_e31180_d_n4, assign31380_body15_e31180_d_n5, assign31380_body15_e31180_d_n6, assign31380_body15_e31180_d_n7, assign31380_body15_e31180_d_n8, assign31380_body15_e31180_d_n9, assign31380_body15_e31180_d_n10, assign31380_body15_e31180_d_n11, assign31380_body15_e31180_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body15_e31178: f64 = (locals.var_xp + locals.var_xmp);
        (assign31380_body15_e31178, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign31380_body15_e31180;
            locals.var_arg_dn0 = assign31380_body15_e31180_d_n0;
            locals.var_arg_dn2 = assign31380_body15_e31180_d_n2;
            locals.var_arg_dn4 = assign31380_body15_e31180_d_n4;
            locals.var_arg_dn5 = assign31380_body15_e31180_d_n5;
            locals.var_arg_dn6 = assign31380_body15_e31180_d_n6;
            locals.var_arg_dn7 = assign31380_body15_e31180_d_n7;
            locals.var_arg_dn8 = assign31380_body15_e31180_d_n8;
            locals.var_arg_dn9 = assign31380_body15_e31180_d_n9;
            locals.var_arg_dn10 = assign31380_body15_e31180_d_n10;
            locals.var_arg_dn11 = assign31380_body15_e31180_d_n11;
            locals.var_arg_dn14 = assign31380_body15_e31180_d_n14;
            let (assign31380_body16_e31193, assign31380_body16_e31193_d_n0, assign31380_body16_e31193_d_n2, assign31380_body16_e31193_d_n4, assign31380_body16_e31193_d_n5, assign31380_body16_e31193_d_n6, assign31380_body16_e31193_d_n7, assign31380_body16_e31193_d_n8, assign31380_body16_e31193_d_n9, assign31380_body16_e31193_d_n10, assign31380_body16_e31193_d_n11, assign31380_body16_e31193_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31380_body16_e31193;
            locals.var_dnm_dn0 = assign31380_body16_e31193_d_n0;
            locals.var_dnm_dn2 = assign31380_body16_e31193_d_n2;
            locals.var_dnm_dn4 = assign31380_body16_e31193_d_n4;
            locals.var_dnm_dn5 = assign31380_body16_e31193_d_n5;
            locals.var_dnm_dn6 = assign31380_body16_e31193_d_n6;
            locals.var_dnm_dn7 = assign31380_body16_e31193_d_n7;
            locals.var_dnm_dn8 = assign31380_body16_e31193_d_n8;
            locals.var_dnm_dn9 = assign31380_body16_e31193_d_n9;
            locals.var_dnm_dn10 = assign31380_body16_e31193_d_n10;
            locals.var_dnm_dn11 = assign31380_body16_e31193_d_n11;
            locals.var_dnm_dn14 = assign31380_body16_e31193_d_n14;
            let assign31380_body17_e31208: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard716 = assign31380_body17_e31208;
            let assign31380_body18_e31211: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard717 = assign31380_body18_e31211;
            let (assign31380_body19_e31228,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31380_body19_e31228;
            let assign31380_body20_e31231: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard718 = assign31380_body20_e31231;
            let (assign31380_body21_e31251,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 == 0.0)) && (locals.var_guard718 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31380_body21_e31251;
            let assign31380_body22_e31254: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard719 = assign31380_body22_e31254;
            let (assign31380_body23_e31277,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 == 0.0)) && (locals.var_guard718 == 0.0)) && (locals.var_guard719 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31380_body23_e31277;
            let assign31380_body24_e31280: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard720 = assign31380_body24_e31280;
            let (assign31380_body25_e31306,) = {
    if ((((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_guard717 == 0.0)) && (locals.var_guard718 == 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31380_body25_e31306;
            let (assign31380_body26_e31321,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign31380_body26_e31321;
            let mut assign31380_body27_loop_guard: usize = 0;
            while {
                let assign31380_body27_cond_e31337: f64 = if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign31380_body27_cond_e31337 != 0.0
            } {
                assign31380_body27_loop_guard += 1;
                assert!(assign31380_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign31380_body27_body0_e31353, assign31380_body27_body0_e31353_d_n0, assign31380_body27_body0_e31353_d_n2, assign31380_body27_body0_e31353_d_n4, assign31380_body27_body0_e31353_d_n5, assign31380_body27_body0_e31353_d_n6, assign31380_body27_body0_e31353_d_n7, assign31380_body27_body0_e31353_d_n8, assign31380_body27_body0_e31353_d_n9, assign31380_body27_body0_e31353_d_n10, assign31380_body27_body0_e31353_d_n11, assign31380_body27_body0_e31353_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31380_body27_body0_e31351: f64 = (locals.var_dnm).sqrt();
        (assign31380_body27_body0_e31351, (locals.var_dnm_dn0 / (2.0 * assign31380_body27_body0_e31351)), (locals.var_dnm_dn2 / (2.0 * assign31380_body27_body0_e31351)), (locals.var_dnm_dn4 / (2.0 * assign31380_body27_body0_e31351)), (locals.var_dnm_dn5 / (2.0 * assign31380_body27_body0_e31351)), (locals.var_dnm_dn6 / (2.0 * assign31380_body27_body0_e31351)), (locals.var_dnm_dn7 / (2.0 * assign31380_body27_body0_e31351)), (locals.var_dnm_dn8 / (2.0 * assign31380_body27_body0_e31351)), (locals.var_dnm_dn9 / (2.0 * assign31380_body27_body0_e31351)), (locals.var_dnm_dn10 / (2.0 * assign31380_body27_body0_e31351)), (locals.var_dnm_dn11 / (2.0 * assign31380_body27_body0_e31351)), (locals.var_dnm_dn14 / (2.0 * assign31380_body27_body0_e31351)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
                locals.var_dnm = assign31380_body27_body0_e31353;
                locals.var_dnm_dn0 = assign31380_body27_body0_e31353_d_n0;
                locals.var_dnm_dn2 = assign31380_body27_body0_e31353_d_n2;
                locals.var_dnm_dn4 = assign31380_body27_body0_e31353_d_n4;
                locals.var_dnm_dn5 = assign31380_body27_body0_e31353_d_n5;
                locals.var_dnm_dn6 = assign31380_body27_body0_e31353_d_n6;
                locals.var_dnm_dn7 = assign31380_body27_body0_e31353_d_n7;
                locals.var_dnm_dn8 = assign31380_body27_body0_e31353_d_n8;
                locals.var_dnm_dn9 = assign31380_body27_body0_e31353_d_n9;
                locals.var_dnm_dn10 = assign31380_body27_body0_e31353_d_n10;
                locals.var_dnm_dn11 = assign31380_body27_body0_e31353_d_n11;
                locals.var_dnm_dn14 = assign31380_body27_body0_e31353_d_n14;
                let (assign31380_body27_body1_e31370,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign31380_body27_body1_e31368: f64 = (locals.var_m0 + 1.0);
        (assign31380_body27_body1_e31368,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign31380_body27_body1_e31370;
            }
            let (assign31380_body28_e31397, assign31380_body28_e31397_d_n0, assign31380_body28_e31397_d_n2, assign31380_body28_e31397_d_n4, assign31380_body28_e31397_d_n5, assign31380_body28_e31397_d_n6, assign31380_body28_e31397_d_n7, assign31380_body28_e31397_d_n8, assign31380_body28_e31397_d_n9, assign31380_body28_e31397_d_n10, assign31380_body28_e31397_d_n11, assign31380_body28_e31397_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 == 0.0)) {
        let (assign31380_body28_e31395, assign31380_body28_e31395_d_n0, assign31380_body28_e31395_d_n2, assign31380_body28_e31395_d_n4, assign31380_body28_e31395_d_n5, assign31380_body28_e31395_d_n6, assign31380_body28_e31395_d_n7, assign31380_body28_e31395_d_n8, assign31380_body28_e31395_d_n9, assign31380_body28_e31395_d_n10, assign31380_body28_e31395_d_n11, assign31380_body28_e31395_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign31380_body28_e31392: f64 = (2.0 * 2.0);
                let assign31380_body28_e31393: f64 = (1.0 / assign31380_body28_e31392);
                let assign31380_body28_e31394: f64 = (locals.var_dnm).powf(assign31380_body28_e31393);
                (assign31380_body28_e31394, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn0)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn2)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn4)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn5)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn6)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn7)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn8)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn9)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn10)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn11)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body28_e31393) as f64).is_finite() && ((assign31380_body28_e31393) as f64).fract() == 0.0 { if assign31380_body28_e31393 == 0.0 { 0.0 } else { (assign31380_body28_e31393 * ((locals.var_dnm).powf(assign31380_body28_e31393 - 1.0) * locals.var_dnm_dn14)) } } else { (assign31380_body28_e31394 * (assign31380_body28_e31393 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign31380_body28_e31395, assign31380_body28_e31395_d_n0, assign31380_body28_e31395_d_n2, assign31380_body28_e31395_d_n4, assign31380_body28_e31395_d_n5, assign31380_body28_e31395_d_n6, assign31380_body28_e31395_d_n7, assign31380_body28_e31395_d_n8, assign31380_body28_e31395_d_n9, assign31380_body28_e31395_d_n10, assign31380_body28_e31395_d_n11, assign31380_body28_e31395_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31380_body28_e31397;
            locals.var_dnm_dn0 = assign31380_body28_e31397_d_n0;
            locals.var_dnm_dn2 = assign31380_body28_e31397_d_n2;
            locals.var_dnm_dn4 = assign31380_body28_e31397_d_n4;
            locals.var_dnm_dn5 = assign31380_body28_e31397_d_n5;
            locals.var_dnm_dn6 = assign31380_body28_e31397_d_n6;
            locals.var_dnm_dn7 = assign31380_body28_e31397_d_n7;
            locals.var_dnm_dn8 = assign31380_body28_e31397_d_n8;
            locals.var_dnm_dn9 = assign31380_body28_e31397_d_n9;
            locals.var_dnm_dn10 = assign31380_body28_e31397_d_n10;
            locals.var_dnm_dn11 = assign31380_body28_e31397_d_n11;
            locals.var_dnm_dn14 = assign31380_body28_e31397_d_n14;
            let (assign31380_body29_e31412, assign31380_body29_e31412_d_n0, assign31380_body29_e31412_d_n2, assign31380_body29_e31412_d_n4, assign31380_body29_e31412_d_n5, assign31380_body29_e31412_d_n6, assign31380_body29_e31412_d_n7, assign31380_body29_e31412_d_n8, assign31380_body29_e31412_d_n9, assign31380_body29_e31412_d_n10, assign31380_body29_e31412_d_n11, assign31380_body29_e31412_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body29_e31410: f64 = (1.0 / locals.var_dnm);
        (assign31380_body29_e31410, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31380_body29_e31412;
            locals.var_dnm_dn0 = assign31380_body29_e31412_d_n0;
            locals.var_dnm_dn2 = assign31380_body29_e31412_d_n2;
            locals.var_dnm_dn4 = assign31380_body29_e31412_d_n4;
            locals.var_dnm_dn5 = assign31380_body29_e31412_d_n5;
            locals.var_dnm_dn6 = assign31380_body29_e31412_d_n6;
            locals.var_dnm_dn7 = assign31380_body29_e31412_d_n7;
            locals.var_dnm_dn8 = assign31380_body29_e31412_d_n8;
            locals.var_dnm_dn9 = assign31380_body29_e31412_d_n9;
            locals.var_dnm_dn10 = assign31380_body29_e31412_d_n10;
            locals.var_dnm_dn11 = assign31380_body29_e31412_d_n11;
            locals.var_dnm_dn14 = assign31380_body29_e31412_d_n14;
            let (assign31380_body30_e31429, assign31380_body30_e31429_d_n0, assign31380_body30_e31429_d_n2, assign31380_body30_e31429_d_n4, assign31380_body30_e31429_d_n5, assign31380_body30_e31429_d_n6, assign31380_body30_e31429_d_n7, assign31380_body30_e31429_d_n8, assign31380_body30_e31429_d_n9, assign31380_body30_e31429_d_n10, assign31380_body30_e31429_d_n11, assign31380_body30_e31429_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body30_e31425: f64 = (locals.var_tmf1 * 1e-8);
        let assign31380_body30_e31427: f64 = (assign31380_body30_e31425 * locals.var_dnm);
        (assign31380_body30_e31427, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign31380_body30_e31425 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
            locals.var_tmf0 = assign31380_body30_e31429;
            locals.var_tmf0_dn0 = assign31380_body30_e31429_d_n0;
            locals.var_tmf0_dn2 = assign31380_body30_e31429_d_n2;
            locals.var_tmf0_dn4 = assign31380_body30_e31429_d_n4;
            locals.var_tmf0_dn5 = assign31380_body30_e31429_d_n5;
            locals.var_tmf0_dn6 = assign31380_body30_e31429_d_n6;
            locals.var_tmf0_dn7 = assign31380_body30_e31429_d_n7;
            locals.var_tmf0_dn8 = assign31380_body30_e31429_d_n8;
            locals.var_tmf0_dn9 = assign31380_body30_e31429_d_n9;
            locals.var_tmf0_dn10 = assign31380_body30_e31429_d_n10;
            locals.var_tmf0_dn11 = assign31380_body30_e31429_d_n11;
            locals.var_tmf0_dn14 = assign31380_body30_e31429_d_n14;
            let (assign31380_body31_e31448, assign31380_body31_e31448_d_n0, assign31380_body31_e31448_d_n2, assign31380_body31_e31448_d_n4, assign31380_body31_e31448_d_n5, assign31380_body31_e31448_d_n6, assign31380_body31_e31448_d_n7, assign31380_body31_e31448_d_n8, assign31380_body31_e31448_d_n9, assign31380_body31_e31448_d_n10, assign31380_body31_e31448_d_n11, assign31380_body31_e31448_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body31_e31442: f64 = (1e-8 * locals.var_xmp);
        let assign31380_body31_e31444: f64 = (assign31380_body31_e31442 * locals.var_dnm);
        let assign31380_body31_e31446: f64 = (assign31380_body31_e31444 / locals.var_arg);
        (assign31380_body31_e31446, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn0)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn2)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn4)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn5)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn6)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn7)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn8)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn9)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn10)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn11)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign31380_body31_e31442 * locals.var_dnm_dn14)) * locals.var_arg) - (assign31380_body31_e31444 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign31380_body31_e31448;
            locals.var_t0_dn0 = assign31380_body31_e31448_d_n0;
            locals.var_t0_dn2 = assign31380_body31_e31448_d_n2;
            locals.var_t0_dn4 = assign31380_body31_e31448_d_n4;
            locals.var_t0_dn5 = assign31380_body31_e31448_d_n5;
            locals.var_t0_dn6 = assign31380_body31_e31448_d_n6;
            locals.var_t0_dn7 = assign31380_body31_e31448_d_n7;
            locals.var_t0_dn8 = assign31380_body31_e31448_d_n8;
            locals.var_t0_dn9 = assign31380_body31_e31448_d_n9;
            locals.var_t0_dn10 = assign31380_body31_e31448_d_n10;
            locals.var_t0_dn11 = assign31380_body31_e31448_d_n11;
            locals.var_t0_dn14 = assign31380_body31_e31448_d_n14;
            let (assign31380_body32_e31465, assign31380_body32_e31465_d_n0, assign31380_body32_e31465_d_n2, assign31380_body32_e31465_d_n4, assign31380_body32_e31465_d_n5, assign31380_body32_e31465_d_n6, assign31380_body32_e31465_d_n7, assign31380_body32_e31465_d_n8, assign31380_body32_e31465_d_n9, assign31380_body32_e31465_d_n10, assign31380_body32_e31465_d_n11, assign31380_body32_e31465_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign31380_body32_e31461: f64 = (locals.var_uc_depthn - 1e-8);
        let assign31380_body32_e31463: f64 = (assign31380_body32_e31461 + locals.var_tmf0);
        (assign31380_body32_e31463, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
            locals.var_w_bl = assign31380_body32_e31465;
            locals.var_w_bl_dn0 = assign31380_body32_e31465_d_n0;
            locals.var_w_bl_dn2 = assign31380_body32_e31465_d_n2;
            locals.var_w_bl_dn4 = assign31380_body32_e31465_d_n4;
            locals.var_w_bl_dn5 = assign31380_body32_e31465_d_n5;
            locals.var_w_bl_dn6 = assign31380_body32_e31465_d_n6;
            locals.var_w_bl_dn7 = assign31380_body32_e31465_d_n7;
            locals.var_w_bl_dn8 = assign31380_body32_e31465_d_n8;
            locals.var_w_bl_dn9 = assign31380_body32_e31465_d_n9;
            locals.var_w_bl_dn10 = assign31380_body32_e31465_d_n10;
            locals.var_w_bl_dn11 = assign31380_body32_e31465_d_n11;
            locals.var_w_bl_dn14 = assign31380_body32_e31465_d_n14;
            let (assign31380_body33_e31478, assign31380_body33_e31478_d_n0, assign31380_body33_e31478_d_n2, assign31380_body33_e31478_d_n4, assign31380_body33_e31478_d_n5, assign31380_body33_e31478_d_n6, assign31380_body33_e31478_d_n7, assign31380_body33_e31478_d_n8, assign31380_body33_e31478_d_n9, assign31380_body33_e31478_d_n10, assign31380_body33_e31478_d_n11, assign31380_body33_e31478_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign31380_body33_e31478;
            locals.var_t0_dn0 = assign31380_body33_e31478_d_n0;
            locals.var_t0_dn2 = assign31380_body33_e31478_d_n2;
            locals.var_t0_dn4 = assign31380_body33_e31478_d_n4;
            locals.var_t0_dn5 = assign31380_body33_e31478_d_n5;
            locals.var_t0_dn6 = assign31380_body33_e31478_d_n6;
            locals.var_t0_dn7 = assign31380_body33_e31478_d_n7;
            locals.var_t0_dn8 = assign31380_body33_e31478_d_n8;
            locals.var_t0_dn9 = assign31380_body33_e31478_d_n9;
            locals.var_t0_dn10 = assign31380_body33_e31478_d_n10;
            locals.var_t0_dn11 = assign31380_body33_e31478_d_n11;
            locals.var_t0_dn14 = assign31380_body33_e31478_d_n14;
            let (assign31380_body34_e31492, assign31380_body34_e31492_d_n0, assign31380_body34_e31492_d_n2, assign31380_body34_e31492_d_n4, assign31380_body34_e31492_d_n5, assign31380_body34_e31492_d_n6, assign31380_body34_e31492_d_n7, assign31380_body34_e31492_d_n8, assign31380_body34_e31492_d_n9, assign31380_body34_e31492_d_n10, assign31380_body34_e31492_d_n11, assign31380_body34_e31492_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 == 0.0)) {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
            locals.var_w_bl = assign31380_body34_e31492;
            locals.var_w_bl_dn0 = assign31380_body34_e31492_d_n0;
            locals.var_w_bl_dn2 = assign31380_body34_e31492_d_n2;
            locals.var_w_bl_dn4 = assign31380_body34_e31492_d_n4;
            locals.var_w_bl_dn5 = assign31380_body34_e31492_d_n5;
            locals.var_w_bl_dn6 = assign31380_body34_e31492_d_n6;
            locals.var_w_bl_dn7 = assign31380_body34_e31492_d_n7;
            locals.var_w_bl_dn8 = assign31380_body34_e31492_d_n8;
            locals.var_w_bl_dn9 = assign31380_body34_e31492_d_n9;
            locals.var_w_bl_dn10 = assign31380_body34_e31492_d_n10;
            locals.var_w_bl_dn11 = assign31380_body34_e31492_d_n11;
            locals.var_w_bl_dn14 = assign31380_body34_e31492_d_n14;
            let (assign31380_body35_e31506, assign31380_body35_e31506_d_n0, assign31380_body35_e31506_d_n2, assign31380_body35_e31506_d_n4, assign31380_body35_e31506_d_n5, assign31380_body35_e31506_d_n6, assign31380_body35_e31506_d_n7, assign31380_body35_e31506_d_n8, assign31380_body35_e31506_d_n9, assign31380_body35_e31506_d_n10, assign31380_body35_e31506_d_n11, assign31380_body35_e31506_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign31380_body35_e31506;
            locals.var_t0_dn0 = assign31380_body35_e31506_d_n0;
            locals.var_t0_dn2 = assign31380_body35_e31506_d_n2;
            locals.var_t0_dn4 = assign31380_body35_e31506_d_n4;
            locals.var_t0_dn5 = assign31380_body35_e31506_d_n5;
            locals.var_t0_dn6 = assign31380_body35_e31506_d_n6;
            locals.var_t0_dn7 = assign31380_body35_e31506_d_n7;
            locals.var_t0_dn8 = assign31380_body35_e31506_d_n8;
            locals.var_t0_dn9 = assign31380_body35_e31506_d_n9;
            locals.var_t0_dn10 = assign31380_body35_e31506_d_n10;
            locals.var_t0_dn11 = assign31380_body35_e31506_d_n11;
            locals.var_t0_dn14 = assign31380_body35_e31506_d_n14;
            let (assign31380_body36_e31521, assign31380_body36_e31521_d_n0, assign31380_body36_e31521_d_n2, assign31380_body36_e31521_d_n4, assign31380_body36_e31521_d_n5, assign31380_body36_e31521_d_n6, assign31380_body36_e31521_d_n7, assign31380_body36_e31521_d_n8, assign31380_body36_e31521_d_n9, assign31380_body36_e31521_d_n10, assign31380_body36_e31521_d_n11, assign31380_body36_e31521_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body36_e31517: f64 = (locals.var_phi_jl_dep - locals.var_vbscl__blk437);
        let assign31380_body36_e31519: f64 = (assign31380_body36_e31517 + locals.var_vbi_dep);
        (assign31380_body36_e31519, ((locals.var_phi_jl_dep_dn0 - locals.var_vbscl__blk437_dn0) + locals.var_vbi_dep_dn0), ((locals.var_phi_jl_dep_dn2 - locals.var_vbscl__blk437_dn2) + locals.var_vbi_dep_dn2), ((locals.var_phi_jl_dep_dn4 - locals.var_vbscl__blk437_dn4) + locals.var_vbi_dep_dn4), ((locals.var_phi_jl_dep_dn5 - locals.var_vbscl__blk437_dn5) + locals.var_vbi_dep_dn5), ((locals.var_phi_jl_dep_dn6 - locals.var_vbscl__blk437_dn6) + locals.var_vbi_dep_dn6), ((locals.var_phi_jl_dep_dn7 - locals.var_vbscl__blk437_dn7) + locals.var_vbi_dep_dn7), ((locals.var_phi_jl_dep_dn8 - locals.var_vbscl__blk437_dn8) + locals.var_vbi_dep_dn8), ((locals.var_phi_jl_dep_dn9 - locals.var_vbscl__blk437_dn9) + locals.var_vbi_dep_dn9), ((locals.var_phi_jl_dep_dn10 - locals.var_vbscl__blk437_dn10) + locals.var_vbi_dep_dn10), ((locals.var_phi_jl_dep_dn11 - locals.var_vbscl__blk437_dn11) + locals.var_vbi_dep_dn11), ((locals.var_phi_jl_dep_dn14 - locals.var_vbscl__blk437_dn14) + locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign31380_body36_e31521;
            locals.var_t1_dn0 = assign31380_body36_e31521_d_n0;
            locals.var_t1_dn2 = assign31380_body36_e31521_d_n2;
            locals.var_t1_dn4 = assign31380_body36_e31521_d_n4;
            locals.var_t1_dn5 = assign31380_body36_e31521_d_n5;
            locals.var_t1_dn6 = assign31380_body36_e31521_d_n6;
            locals.var_t1_dn7 = assign31380_body36_e31521_d_n7;
            locals.var_t1_dn8 = assign31380_body36_e31521_d_n8;
            locals.var_t1_dn9 = assign31380_body36_e31521_d_n9;
            locals.var_t1_dn10 = assign31380_body36_e31521_d_n10;
            locals.var_t1_dn11 = assign31380_body36_e31521_d_n11;
            locals.var_t1_dn14 = assign31380_body36_e31521_d_n14;
            let assign31380_body37_e31525: f64 = 0.1;
            let assign31380_body37_e31530: f64 = if ((locals.var_t1 < assign31380_body37_e31525) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard721 = assign31380_body37_e31530;
            let (assign31380_body38_e31547, assign31380_body38_e31547_d_n0, assign31380_body38_e31547_d_n2, assign31380_body38_e31547_d_n4, assign31380_body38_e31547_d_n5, assign31380_body38_e31547_d_n6, assign31380_body38_e31547_d_n7, assign31380_body38_e31547_d_n8, assign31380_body38_e31547_d_n9, assign31380_body38_e31547_d_n10, assign31380_body38_e31547_d_n11, assign31380_body38_e31547_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body38_e31543: f64 = 0.1;
        let assign31380_body38_e31545: f64 = (assign31380_body38_e31543 - locals.var_t1);
        (assign31380_body38_e31545, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign31380_body38_e31547;
            locals.var_tmf1_dn0 = assign31380_body38_e31547_d_n0;
            locals.var_tmf1_dn2 = assign31380_body38_e31547_d_n2;
            locals.var_tmf1_dn4 = assign31380_body38_e31547_d_n4;
            locals.var_tmf1_dn5 = assign31380_body38_e31547_d_n5;
            locals.var_tmf1_dn6 = assign31380_body38_e31547_d_n6;
            locals.var_tmf1_dn7 = assign31380_body38_e31547_d_n7;
            locals.var_tmf1_dn8 = assign31380_body38_e31547_d_n8;
            locals.var_tmf1_dn9 = assign31380_body38_e31547_d_n9;
            locals.var_tmf1_dn10 = assign31380_body38_e31547_d_n10;
            locals.var_tmf1_dn11 = assign31380_body38_e31547_d_n11;
            locals.var_tmf1_dn14 = assign31380_body38_e31547_d_n14;
            let (assign31380_body39_e31562, assign31380_body39_e31562_d_n0, assign31380_body39_e31562_d_n2, assign31380_body39_e31562_d_n4, assign31380_body39_e31562_d_n5, assign31380_body39_e31562_d_n6, assign31380_body39_e31562_d_n7, assign31380_body39_e31562_d_n8, assign31380_body39_e31562_d_n9, assign31380_body39_e31562_d_n10, assign31380_body39_e31562_d_n11, assign31380_body39_e31562_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body39_e31560: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign31380_body39_e31560, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
            locals.var_x2 = assign31380_body39_e31562;
            locals.var_x2_dn0 = assign31380_body39_e31562_d_n0;
            locals.var_x2_dn2 = assign31380_body39_e31562_d_n2;
            locals.var_x2_dn4 = assign31380_body39_e31562_d_n4;
            locals.var_x2_dn5 = assign31380_body39_e31562_d_n5;
            locals.var_x2_dn6 = assign31380_body39_e31562_d_n6;
            locals.var_x2_dn7 = assign31380_body39_e31562_d_n7;
            locals.var_x2_dn8 = assign31380_body39_e31562_d_n8;
            locals.var_x2_dn9 = assign31380_body39_e31562_d_n9;
            locals.var_x2_dn10 = assign31380_body39_e31562_d_n10;
            locals.var_x2_dn11 = assign31380_body39_e31562_d_n11;
            locals.var_x2_dn14 = assign31380_body39_e31562_d_n14;
            let (assign31380_body40_e31577, assign31380_body40_e31577_d_n0, assign31380_body40_e31577_d_n2, assign31380_body40_e31577_d_n4, assign31380_body40_e31577_d_n5, assign31380_body40_e31577_d_n6, assign31380_body40_e31577_d_n7, assign31380_body40_e31577_d_n8, assign31380_body40_e31577_d_n9, assign31380_body40_e31577_d_n10, assign31380_body40_e31577_d_n11, assign31380_body40_e31577_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body40_e31575: f64 = (0.1 * 0.1);
        (assign31380_body40_e31575, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
            locals.var_xmax2 = assign31380_body40_e31577;
            locals.var_xmax2_dn0 = assign31380_body40_e31577_d_n0;
            locals.var_xmax2_dn2 = assign31380_body40_e31577_d_n2;
            locals.var_xmax2_dn4 = assign31380_body40_e31577_d_n4;
            locals.var_xmax2_dn5 = assign31380_body40_e31577_d_n5;
            locals.var_xmax2_dn6 = assign31380_body40_e31577_d_n6;
            locals.var_xmax2_dn7 = assign31380_body40_e31577_d_n7;
            locals.var_xmax2_dn8 = assign31380_body40_e31577_d_n8;
            locals.var_xmax2_dn9 = assign31380_body40_e31577_d_n9;
            locals.var_xmax2_dn10 = assign31380_body40_e31577_d_n10;
            locals.var_xmax2_dn11 = assign31380_body40_e31577_d_n11;
            locals.var_xmax2_dn14 = assign31380_body40_e31577_d_n14;
            let (assign31380_body41_e31590, assign31380_body41_e31590_d_n0, assign31380_body41_e31590_d_n2, assign31380_body41_e31590_d_n4, assign31380_body41_e31590_d_n5, assign31380_body41_e31590_d_n6, assign31380_body41_e31590_d_n7, assign31380_body41_e31590_d_n8, assign31380_body41_e31590_d_n9, assign31380_body41_e31590_d_n10, assign31380_body41_e31590_d_n11, assign31380_body41_e31590_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31380_body41_e31590;
            locals.var_xp_dn0 = assign31380_body41_e31590_d_n0;
            locals.var_xp_dn2 = assign31380_body41_e31590_d_n2;
            locals.var_xp_dn4 = assign31380_body41_e31590_d_n4;
            locals.var_xp_dn5 = assign31380_body41_e31590_d_n5;
            locals.var_xp_dn6 = assign31380_body41_e31590_d_n6;
            locals.var_xp_dn7 = assign31380_body41_e31590_d_n7;
            locals.var_xp_dn8 = assign31380_body41_e31590_d_n8;
            locals.var_xp_dn9 = assign31380_body41_e31590_d_n9;
            locals.var_xp_dn10 = assign31380_body41_e31590_d_n10;
            locals.var_xp_dn11 = assign31380_body41_e31590_d_n11;
            locals.var_xp_dn14 = assign31380_body41_e31590_d_n14;
            let (assign31380_body42_e31603, assign31380_body42_e31603_d_n0, assign31380_body42_e31603_d_n2, assign31380_body42_e31603_d_n4, assign31380_body42_e31603_d_n5, assign31380_body42_e31603_d_n6, assign31380_body42_e31603_d_n7, assign31380_body42_e31603_d_n8, assign31380_body42_e31603_d_n9, assign31380_body42_e31603_d_n10, assign31380_body42_e31603_d_n11, assign31380_body42_e31603_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31380_body42_e31603;
            locals.var_xmp_dn0 = assign31380_body42_e31603_d_n0;
            locals.var_xmp_dn2 = assign31380_body42_e31603_d_n2;
            locals.var_xmp_dn4 = assign31380_body42_e31603_d_n4;
            locals.var_xmp_dn5 = assign31380_body42_e31603_d_n5;
            locals.var_xmp_dn6 = assign31380_body42_e31603_d_n6;
            locals.var_xmp_dn7 = assign31380_body42_e31603_d_n7;
            locals.var_xmp_dn8 = assign31380_body42_e31603_d_n8;
            locals.var_xmp_dn9 = assign31380_body42_e31603_d_n9;
            locals.var_xmp_dn10 = assign31380_body42_e31603_d_n10;
            locals.var_xmp_dn11 = assign31380_body42_e31603_d_n11;
            locals.var_xmp_dn14 = assign31380_body42_e31603_d_n14;
            let (assign31380_body43_e31616,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign31380_body43_e31616;
            let (assign31380_body44_e31629,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31380_body44_e31629;
            let (assign31380_body45_e31642, assign31380_body45_e31642_d_n0, assign31380_body45_e31642_d_n2, assign31380_body45_e31642_d_n4, assign31380_body45_e31642_d_n5, assign31380_body45_e31642_d_n6, assign31380_body45_e31642_d_n7, assign31380_body45_e31642_d_n8, assign31380_body45_e31642_d_n9, assign31380_body45_e31642_d_n10, assign31380_body45_e31642_d_n11, assign31380_body45_e31642_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign31380_body45_e31642;
            locals.var_arg_dn0 = assign31380_body45_e31642_d_n0;
            locals.var_arg_dn2 = assign31380_body45_e31642_d_n2;
            locals.var_arg_dn4 = assign31380_body45_e31642_d_n4;
            locals.var_arg_dn5 = assign31380_body45_e31642_d_n5;
            locals.var_arg_dn6 = assign31380_body45_e31642_d_n6;
            locals.var_arg_dn7 = assign31380_body45_e31642_d_n7;
            locals.var_arg_dn8 = assign31380_body45_e31642_d_n8;
            locals.var_arg_dn9 = assign31380_body45_e31642_d_n9;
            locals.var_arg_dn10 = assign31380_body45_e31642_d_n10;
            locals.var_arg_dn11 = assign31380_body45_e31642_d_n11;
            locals.var_arg_dn14 = assign31380_body45_e31642_d_n14;
            let (assign31380_body46_e31655, assign31380_body46_e31655_d_n0, assign31380_body46_e31655_d_n2, assign31380_body46_e31655_d_n4, assign31380_body46_e31655_d_n5, assign31380_body46_e31655_d_n6, assign31380_body46_e31655_d_n7, assign31380_body46_e31655_d_n8, assign31380_body46_e31655_d_n9, assign31380_body46_e31655_d_n10, assign31380_body46_e31655_d_n11, assign31380_body46_e31655_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31380_body46_e31655;
            locals.var_dnm_dn0 = assign31380_body46_e31655_d_n0;
            locals.var_dnm_dn2 = assign31380_body46_e31655_d_n2;
            locals.var_dnm_dn4 = assign31380_body46_e31655_d_n4;
            locals.var_dnm_dn5 = assign31380_body46_e31655_d_n5;
            locals.var_dnm_dn6 = assign31380_body46_e31655_d_n6;
            locals.var_dnm_dn7 = assign31380_body46_e31655_d_n7;
            locals.var_dnm_dn8 = assign31380_body46_e31655_d_n8;
            locals.var_dnm_dn9 = assign31380_body46_e31655_d_n9;
            locals.var_dnm_dn10 = assign31380_body46_e31655_d_n10;
            locals.var_dnm_dn11 = assign31380_body46_e31655_d_n11;
            locals.var_dnm_dn14 = assign31380_body46_e31655_d_n14;
            let (assign31380_body47_e31670, assign31380_body47_e31670_d_n0, assign31380_body47_e31670_d_n2, assign31380_body47_e31670_d_n4, assign31380_body47_e31670_d_n5, assign31380_body47_e31670_d_n6, assign31380_body47_e31670_d_n7, assign31380_body47_e31670_d_n8, assign31380_body47_e31670_d_n9, assign31380_body47_e31670_d_n10, assign31380_body47_e31670_d_n11, assign31380_body47_e31670_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body47_e31668: f64 = (locals.var_xp * locals.var_x2);
        (assign31380_body47_e31668, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31380_body47_e31670;
            locals.var_xp_dn0 = assign31380_body47_e31670_d_n0;
            locals.var_xp_dn2 = assign31380_body47_e31670_d_n2;
            locals.var_xp_dn4 = assign31380_body47_e31670_d_n4;
            locals.var_xp_dn5 = assign31380_body47_e31670_d_n5;
            locals.var_xp_dn6 = assign31380_body47_e31670_d_n6;
            locals.var_xp_dn7 = assign31380_body47_e31670_d_n7;
            locals.var_xp_dn8 = assign31380_body47_e31670_d_n8;
            locals.var_xp_dn9 = assign31380_body47_e31670_d_n9;
            locals.var_xp_dn10 = assign31380_body47_e31670_d_n10;
            locals.var_xp_dn11 = assign31380_body47_e31670_d_n11;
            locals.var_xp_dn14 = assign31380_body47_e31670_d_n14;
            let (assign31380_body48_e31685, assign31380_body48_e31685_d_n0, assign31380_body48_e31685_d_n2, assign31380_body48_e31685_d_n4, assign31380_body48_e31685_d_n5, assign31380_body48_e31685_d_n6, assign31380_body48_e31685_d_n7, assign31380_body48_e31685_d_n8, assign31380_body48_e31685_d_n9, assign31380_body48_e31685_d_n10, assign31380_body48_e31685_d_n11, assign31380_body48_e31685_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body48_e31683: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign31380_body48_e31683, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31380_body48_e31685;
            locals.var_xmp_dn0 = assign31380_body48_e31685_d_n0;
            locals.var_xmp_dn2 = assign31380_body48_e31685_d_n2;
            locals.var_xmp_dn4 = assign31380_body48_e31685_d_n4;
            locals.var_xmp_dn5 = assign31380_body48_e31685_d_n5;
            locals.var_xmp_dn6 = assign31380_body48_e31685_d_n6;
            locals.var_xmp_dn7 = assign31380_body48_e31685_d_n7;
            locals.var_xmp_dn8 = assign31380_body48_e31685_d_n8;
            locals.var_xmp_dn9 = assign31380_body48_e31685_d_n9;
            locals.var_xmp_dn10 = assign31380_body48_e31685_d_n10;
            locals.var_xmp_dn11 = assign31380_body48_e31685_d_n11;
            locals.var_xmp_dn14 = assign31380_body48_e31685_d_n14;
            let (assign31380_body49_e31700, assign31380_body49_e31700_d_n0, assign31380_body49_e31700_d_n2, assign31380_body49_e31700_d_n4, assign31380_body49_e31700_d_n5, assign31380_body49_e31700_d_n6, assign31380_body49_e31700_d_n7, assign31380_body49_e31700_d_n8, assign31380_body49_e31700_d_n9, assign31380_body49_e31700_d_n10, assign31380_body49_e31700_d_n11, assign31380_body49_e31700_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body49_e31698: f64 = (locals.var_xp * locals.var_x2);
        (assign31380_body49_e31698, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign31380_body49_e31700;
            locals.var_xp_dn0 = assign31380_body49_e31700_d_n0;
            locals.var_xp_dn2 = assign31380_body49_e31700_d_n2;
            locals.var_xp_dn4 = assign31380_body49_e31700_d_n4;
            locals.var_xp_dn5 = assign31380_body49_e31700_d_n5;
            locals.var_xp_dn6 = assign31380_body49_e31700_d_n6;
            locals.var_xp_dn7 = assign31380_body49_e31700_d_n7;
            locals.var_xp_dn8 = assign31380_body49_e31700_d_n8;
            locals.var_xp_dn9 = assign31380_body49_e31700_d_n9;
            locals.var_xp_dn10 = assign31380_body49_e31700_d_n10;
            locals.var_xp_dn11 = assign31380_body49_e31700_d_n11;
            locals.var_xp_dn14 = assign31380_body49_e31700_d_n14;
            let (assign31380_body50_e31715, assign31380_body50_e31715_d_n0, assign31380_body50_e31715_d_n2, assign31380_body50_e31715_d_n4, assign31380_body50_e31715_d_n5, assign31380_body50_e31715_d_n6, assign31380_body50_e31715_d_n7, assign31380_body50_e31715_d_n8, assign31380_body50_e31715_d_n9, assign31380_body50_e31715_d_n10, assign31380_body50_e31715_d_n11, assign31380_body50_e31715_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body50_e31713: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign31380_body50_e31713, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign31380_body50_e31715;
            locals.var_xmp_dn0 = assign31380_body50_e31715_d_n0;
            locals.var_xmp_dn2 = assign31380_body50_e31715_d_n2;
            locals.var_xmp_dn4 = assign31380_body50_e31715_d_n4;
            locals.var_xmp_dn5 = assign31380_body50_e31715_d_n5;
            locals.var_xmp_dn6 = assign31380_body50_e31715_d_n6;
            locals.var_xmp_dn7 = assign31380_body50_e31715_d_n7;
            locals.var_xmp_dn8 = assign31380_body50_e31715_d_n8;
            locals.var_xmp_dn9 = assign31380_body50_e31715_d_n9;
            locals.var_xmp_dn10 = assign31380_body50_e31715_d_n10;
            locals.var_xmp_dn11 = assign31380_body50_e31715_d_n11;
            locals.var_xmp_dn14 = assign31380_body50_e31715_d_n14;
            let (assign31380_body51_e31730, assign31380_body51_e31730_d_n0, assign31380_body51_e31730_d_n2, assign31380_body51_e31730_d_n4, assign31380_body51_e31730_d_n5, assign31380_body51_e31730_d_n6, assign31380_body51_e31730_d_n7, assign31380_body51_e31730_d_n8, assign31380_body51_e31730_d_n9, assign31380_body51_e31730_d_n10, assign31380_body51_e31730_d_n11, assign31380_body51_e31730_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body51_e31728: f64 = (locals.var_xp + locals.var_xmp);
        (assign31380_body51_e31728, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
            locals.var_arg = assign31380_body51_e31730;
            locals.var_arg_dn0 = assign31380_body51_e31730_d_n0;
            locals.var_arg_dn2 = assign31380_body51_e31730_d_n2;
            locals.var_arg_dn4 = assign31380_body51_e31730_d_n4;
            locals.var_arg_dn5 = assign31380_body51_e31730_d_n5;
            locals.var_arg_dn6 = assign31380_body51_e31730_d_n6;
            locals.var_arg_dn7 = assign31380_body51_e31730_d_n7;
            locals.var_arg_dn8 = assign31380_body51_e31730_d_n8;
            locals.var_arg_dn9 = assign31380_body51_e31730_d_n9;
            locals.var_arg_dn10 = assign31380_body51_e31730_d_n10;
            locals.var_arg_dn11 = assign31380_body51_e31730_d_n11;
            locals.var_arg_dn14 = assign31380_body51_e31730_d_n14;
            let (assign31380_body52_e31743, assign31380_body52_e31743_d_n0, assign31380_body52_e31743_d_n2, assign31380_body52_e31743_d_n4, assign31380_body52_e31743_d_n5, assign31380_body52_e31743_d_n6, assign31380_body52_e31743_d_n7, assign31380_body52_e31743_d_n8, assign31380_body52_e31743_d_n9, assign31380_body52_e31743_d_n10, assign31380_body52_e31743_d_n11, assign31380_body52_e31743_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31380_body52_e31743;
            locals.var_dnm_dn0 = assign31380_body52_e31743_d_n0;
            locals.var_dnm_dn2 = assign31380_body52_e31743_d_n2;
            locals.var_dnm_dn4 = assign31380_body52_e31743_d_n4;
            locals.var_dnm_dn5 = assign31380_body52_e31743_d_n5;
            locals.var_dnm_dn6 = assign31380_body52_e31743_d_n6;
            locals.var_dnm_dn7 = assign31380_body52_e31743_d_n7;
            locals.var_dnm_dn8 = assign31380_body52_e31743_d_n8;
            locals.var_dnm_dn9 = assign31380_body52_e31743_d_n9;
            locals.var_dnm_dn10 = assign31380_body52_e31743_d_n10;
            locals.var_dnm_dn11 = assign31380_body52_e31743_d_n11;
            locals.var_dnm_dn14 = assign31380_body52_e31743_d_n14;
            let assign31380_body53_e31758: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard722 = assign31380_body53_e31758;
            let assign31380_body54_e31761: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard723 = assign31380_body54_e31761;
            let (assign31380_body55_e31778,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31380_body55_e31778;
            let assign31380_body56_e31781: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard724 = assign31380_body56_e31781;
            let (assign31380_body57_e31801,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 == 0.0)) && (locals.var_guard724 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31380_body57_e31801;
            let assign31380_body58_e31804: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard725 = assign31380_body58_e31804;
            let (assign31380_body59_e31827,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 == 0.0)) && (locals.var_guard724 == 0.0)) && (locals.var_guard725 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31380_body59_e31827;
            let assign31380_body60_e31830: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard726 = assign31380_body60_e31830;
            let (assign31380_body61_e31856,) = {
    if ((((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 == 0.0)) && (locals.var_guard724 == 0.0)) && (locals.var_guard725 == 0.0)) && (locals.var_guard726 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign31380_body61_e31856;
            let (assign31380_body62_e31871,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) && (locals.var_guard722 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign31380_body62_e31871;
            let mut assign31380_body63_loop_guard: usize = 0;
            while {
                let assign31380_body63_cond_e31887: f64 = if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) && (locals.var_guard722 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign31380_body63_cond_e31887 != 0.0
            } {
                assign31380_body63_loop_guard += 1;
                assert!(assign31380_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign31380_body63_body0_e31903, assign31380_body63_body0_e31903_d_n0, assign31380_body63_body0_e31903_d_n2, assign31380_body63_body0_e31903_d_n4, assign31380_body63_body0_e31903_d_n5, assign31380_body63_body0_e31903_d_n6, assign31380_body63_body0_e31903_d_n7, assign31380_body63_body0_e31903_d_n8, assign31380_body63_body0_e31903_d_n9, assign31380_body63_body0_e31903_d_n10, assign31380_body63_body0_e31903_d_n11, assign31380_body63_body0_e31903_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) && (locals.var_guard722 != 0.0)) {
        let assign31380_body63_body0_e31901: f64 = (locals.var_dnm).sqrt();
        (assign31380_body63_body0_e31901, (locals.var_dnm_dn0 / (2.0 * assign31380_body63_body0_e31901)), (locals.var_dnm_dn2 / (2.0 * assign31380_body63_body0_e31901)), (locals.var_dnm_dn4 / (2.0 * assign31380_body63_body0_e31901)), (locals.var_dnm_dn5 / (2.0 * assign31380_body63_body0_e31901)), (locals.var_dnm_dn6 / (2.0 * assign31380_body63_body0_e31901)), (locals.var_dnm_dn7 / (2.0 * assign31380_body63_body0_e31901)), (locals.var_dnm_dn8 / (2.0 * assign31380_body63_body0_e31901)), (locals.var_dnm_dn9 / (2.0 * assign31380_body63_body0_e31901)), (locals.var_dnm_dn10 / (2.0 * assign31380_body63_body0_e31901)), (locals.var_dnm_dn11 / (2.0 * assign31380_body63_body0_e31901)), (locals.var_dnm_dn14 / (2.0 * assign31380_body63_body0_e31901)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
                locals.var_dnm = assign31380_body63_body0_e31903;
                locals.var_dnm_dn0 = assign31380_body63_body0_e31903_d_n0;
                locals.var_dnm_dn2 = assign31380_body63_body0_e31903_d_n2;
                locals.var_dnm_dn4 = assign31380_body63_body0_e31903_d_n4;
                locals.var_dnm_dn5 = assign31380_body63_body0_e31903_d_n5;
                locals.var_dnm_dn6 = assign31380_body63_body0_e31903_d_n6;
                locals.var_dnm_dn7 = assign31380_body63_body0_e31903_d_n7;
                locals.var_dnm_dn8 = assign31380_body63_body0_e31903_d_n8;
                locals.var_dnm_dn9 = assign31380_body63_body0_e31903_d_n9;
                locals.var_dnm_dn10 = assign31380_body63_body0_e31903_d_n10;
                locals.var_dnm_dn11 = assign31380_body63_body0_e31903_d_n11;
                locals.var_dnm_dn14 = assign31380_body63_body0_e31903_d_n14;
                let (assign31380_body63_body1_e31920,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) && (locals.var_guard722 != 0.0)) {
        let assign31380_body63_body1_e31918: f64 = (locals.var_m0 + 1.0);
        (assign31380_body63_body1_e31918,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign31380_body63_body1_e31920;
            }
            let (assign31380_body64_e31947, assign31380_body64_e31947_d_n0, assign31380_body64_e31947_d_n2, assign31380_body64_e31947_d_n4, assign31380_body64_e31947_d_n5, assign31380_body64_e31947_d_n6, assign31380_body64_e31947_d_n7, assign31380_body64_e31947_d_n8, assign31380_body64_e31947_d_n9, assign31380_body64_e31947_d_n10, assign31380_body64_e31947_d_n11, assign31380_body64_e31947_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) && (locals.var_guard722 == 0.0)) {
        let (assign31380_body64_e31945, assign31380_body64_e31945_d_n0, assign31380_body64_e31945_d_n2, assign31380_body64_e31945_d_n4, assign31380_body64_e31945_d_n5, assign31380_body64_e31945_d_n6, assign31380_body64_e31945_d_n7, assign31380_body64_e31945_d_n8, assign31380_body64_e31945_d_n9, assign31380_body64_e31945_d_n10, assign31380_body64_e31945_d_n11, assign31380_body64_e31945_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign31380_body64_e31942: f64 = (2.0 * 2.0);
                let assign31380_body64_e31943: f64 = (1.0 / assign31380_body64_e31942);
                let assign31380_body64_e31944: f64 = (locals.var_dnm).powf(assign31380_body64_e31943);
                (assign31380_body64_e31944, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn0)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn2)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn4)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn5)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn6)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn7)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn8)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn9)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn10)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn11)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign31380_body64_e31943) as f64).is_finite() && ((assign31380_body64_e31943) as f64).fract() == 0.0 { if assign31380_body64_e31943 == 0.0 { 0.0 } else { (assign31380_body64_e31943 * ((locals.var_dnm).powf(assign31380_body64_e31943 - 1.0) * locals.var_dnm_dn14)) } } else { (assign31380_body64_e31944 * (assign31380_body64_e31943 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign31380_body64_e31945, assign31380_body64_e31945_d_n0, assign31380_body64_e31945_d_n2, assign31380_body64_e31945_d_n4, assign31380_body64_e31945_d_n5, assign31380_body64_e31945_d_n6, assign31380_body64_e31945_d_n7, assign31380_body64_e31945_d_n8, assign31380_body64_e31945_d_n9, assign31380_body64_e31945_d_n10, assign31380_body64_e31945_d_n11, assign31380_body64_e31945_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31380_body64_e31947;
            locals.var_dnm_dn0 = assign31380_body64_e31947_d_n0;
            locals.var_dnm_dn2 = assign31380_body64_e31947_d_n2;
            locals.var_dnm_dn4 = assign31380_body64_e31947_d_n4;
            locals.var_dnm_dn5 = assign31380_body64_e31947_d_n5;
            locals.var_dnm_dn6 = assign31380_body64_e31947_d_n6;
            locals.var_dnm_dn7 = assign31380_body64_e31947_d_n7;
            locals.var_dnm_dn8 = assign31380_body64_e31947_d_n8;
            locals.var_dnm_dn9 = assign31380_body64_e31947_d_n9;
            locals.var_dnm_dn10 = assign31380_body64_e31947_d_n10;
            locals.var_dnm_dn11 = assign31380_body64_e31947_d_n11;
            locals.var_dnm_dn14 = assign31380_body64_e31947_d_n14;
            let (assign31380_body65_e31962, assign31380_body65_e31962_d_n0, assign31380_body65_e31962_d_n2, assign31380_body65_e31962_d_n4, assign31380_body65_e31962_d_n5, assign31380_body65_e31962_d_n6, assign31380_body65_e31962_d_n7, assign31380_body65_e31962_d_n8, assign31380_body65_e31962_d_n9, assign31380_body65_e31962_d_n10, assign31380_body65_e31962_d_n11, assign31380_body65_e31962_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body65_e31960: f64 = (1.0 / locals.var_dnm);
        (assign31380_body65_e31960, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign31380_body65_e31962;
            locals.var_dnm_dn0 = assign31380_body65_e31962_d_n0;
            locals.var_dnm_dn2 = assign31380_body65_e31962_d_n2;
            locals.var_dnm_dn4 = assign31380_body65_e31962_d_n4;
            locals.var_dnm_dn5 = assign31380_body65_e31962_d_n5;
            locals.var_dnm_dn6 = assign31380_body65_e31962_d_n6;
            locals.var_dnm_dn7 = assign31380_body65_e31962_d_n7;
            locals.var_dnm_dn8 = assign31380_body65_e31962_d_n8;
            locals.var_dnm_dn9 = assign31380_body65_e31962_d_n9;
            locals.var_dnm_dn10 = assign31380_body65_e31962_d_n10;
            locals.var_dnm_dn11 = assign31380_body65_e31962_d_n11;
            locals.var_dnm_dn14 = assign31380_body65_e31962_d_n14;
            let (assign31380_body66_e31979, assign31380_body66_e31979_d_n0, assign31380_body66_e31979_d_n2, assign31380_body66_e31979_d_n4, assign31380_body66_e31979_d_n5, assign31380_body66_e31979_d_n6, assign31380_body66_e31979_d_n7, assign31380_body66_e31979_d_n8, assign31380_body66_e31979_d_n9, assign31380_body66_e31979_d_n10, assign31380_body66_e31979_d_n11, assign31380_body66_e31979_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body66_e31975: f64 = (locals.var_tmf1 * 0.1);
        let assign31380_body66_e31977: f64 = (assign31380_body66_e31975 * locals.var_dnm);
        (assign31380_body66_e31977, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign31380_body66_e31975 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
            locals.var_tmf0 = assign31380_body66_e31979;
            locals.var_tmf0_dn0 = assign31380_body66_e31979_d_n0;
            locals.var_tmf0_dn2 = assign31380_body66_e31979_d_n2;
            locals.var_tmf0_dn4 = assign31380_body66_e31979_d_n4;
            locals.var_tmf0_dn5 = assign31380_body66_e31979_d_n5;
            locals.var_tmf0_dn6 = assign31380_body66_e31979_d_n6;
            locals.var_tmf0_dn7 = assign31380_body66_e31979_d_n7;
            locals.var_tmf0_dn8 = assign31380_body66_e31979_d_n8;
            locals.var_tmf0_dn9 = assign31380_body66_e31979_d_n9;
            locals.var_tmf0_dn10 = assign31380_body66_e31979_d_n10;
            locals.var_tmf0_dn11 = assign31380_body66_e31979_d_n11;
            locals.var_tmf0_dn14 = assign31380_body66_e31979_d_n14;
            let (assign31380_body67_e31998, assign31380_body67_e31998_d_n0, assign31380_body67_e31998_d_n2, assign31380_body67_e31998_d_n4, assign31380_body67_e31998_d_n5, assign31380_body67_e31998_d_n6, assign31380_body67_e31998_d_n7, assign31380_body67_e31998_d_n8, assign31380_body67_e31998_d_n9, assign31380_body67_e31998_d_n10, assign31380_body67_e31998_d_n11, assign31380_body67_e31998_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body67_e31992: f64 = (0.1 * locals.var_xmp);
        let assign31380_body67_e31994: f64 = (assign31380_body67_e31992 * locals.var_dnm);
        let assign31380_body67_e31996: f64 = (assign31380_body67_e31994 / locals.var_arg);
        (assign31380_body67_e31996, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn0)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn2)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn4)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn5)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn6)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn7)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn8)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn9)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn10)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn11)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign31380_body67_e31992 * locals.var_dnm_dn14)) * locals.var_arg) - (assign31380_body67_e31994 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign31380_body67_e31998;
            locals.var_t7_dn0 = assign31380_body67_e31998_d_n0;
            locals.var_t7_dn2 = assign31380_body67_e31998_d_n2;
            locals.var_t7_dn4 = assign31380_body67_e31998_d_n4;
            locals.var_t7_dn5 = assign31380_body67_e31998_d_n5;
            locals.var_t7_dn6 = assign31380_body67_e31998_d_n6;
            locals.var_t7_dn7 = assign31380_body67_e31998_d_n7;
            locals.var_t7_dn8 = assign31380_body67_e31998_d_n8;
            locals.var_t7_dn9 = assign31380_body67_e31998_d_n9;
            locals.var_t7_dn10 = assign31380_body67_e31998_d_n10;
            locals.var_t7_dn11 = assign31380_body67_e31998_d_n11;
            locals.var_t7_dn14 = assign31380_body67_e31998_d_n14;
            let (assign31380_body68_e32015, assign31380_body68_e32015_d_n0, assign31380_body68_e32015_d_n2, assign31380_body68_e32015_d_n4, assign31380_body68_e32015_d_n5, assign31380_body68_e32015_d_n6, assign31380_body68_e32015_d_n7, assign31380_body68_e32015_d_n8, assign31380_body68_e32015_d_n9, assign31380_body68_e32015_d_n10, assign31380_body68_e32015_d_n11, assign31380_body68_e32015_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign31380_body68_e32011: f64 = 0.1;
        let assign31380_body68_e32013: f64 = (assign31380_body68_e32011 - locals.var_tmf0);
        (assign31380_body68_e32013, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign31380_body68_e32015;
            locals.var_t2_dn0 = assign31380_body68_e32015_d_n0;
            locals.var_t2_dn2 = assign31380_body68_e32015_d_n2;
            locals.var_t2_dn4 = assign31380_body68_e32015_d_n4;
            locals.var_t2_dn5 = assign31380_body68_e32015_d_n5;
            locals.var_t2_dn6 = assign31380_body68_e32015_d_n6;
            locals.var_t2_dn7 = assign31380_body68_e32015_d_n7;
            locals.var_t2_dn8 = assign31380_body68_e32015_d_n8;
            locals.var_t2_dn9 = assign31380_body68_e32015_d_n9;
            locals.var_t2_dn10 = assign31380_body68_e32015_d_n10;
            locals.var_t2_dn11 = assign31380_body68_e32015_d_n11;
            locals.var_t2_dn14 = assign31380_body68_e32015_d_n14;
            let (assign31380_body69_e32028, assign31380_body69_e32028_d_n0, assign31380_body69_e32028_d_n2, assign31380_body69_e32028_d_n4, assign31380_body69_e32028_d_n5, assign31380_body69_e32028_d_n6, assign31380_body69_e32028_d_n7, assign31380_body69_e32028_d_n8, assign31380_body69_e32028_d_n9, assign31380_body69_e32028_d_n10, assign31380_body69_e32028_d_n11, assign31380_body69_e32028_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 != 0.0)) {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign31380_body69_e32028;
            locals.var_t7_dn0 = assign31380_body69_e32028_d_n0;
            locals.var_t7_dn2 = assign31380_body69_e32028_d_n2;
            locals.var_t7_dn4 = assign31380_body69_e32028_d_n4;
            locals.var_t7_dn5 = assign31380_body69_e32028_d_n5;
            locals.var_t7_dn6 = assign31380_body69_e32028_d_n6;
            locals.var_t7_dn7 = assign31380_body69_e32028_d_n7;
            locals.var_t7_dn8 = assign31380_body69_e32028_d_n8;
            locals.var_t7_dn9 = assign31380_body69_e32028_d_n9;
            locals.var_t7_dn10 = assign31380_body69_e32028_d_n10;
            locals.var_t7_dn11 = assign31380_body69_e32028_d_n11;
            locals.var_t7_dn14 = assign31380_body69_e32028_d_n14;
            let (assign31380_body70_e32042, assign31380_body70_e32042_d_n0, assign31380_body70_e32042_d_n2, assign31380_body70_e32042_d_n4, assign31380_body70_e32042_d_n5, assign31380_body70_e32042_d_n6, assign31380_body70_e32042_d_n7, assign31380_body70_e32042_d_n8, assign31380_body70_e32042_d_n9, assign31380_body70_e32042_d_n10, assign31380_body70_e32042_d_n11, assign31380_body70_e32042_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign31380_body70_e32042;
            locals.var_t2_dn0 = assign31380_body70_e32042_d_n0;
            locals.var_t2_dn2 = assign31380_body70_e32042_d_n2;
            locals.var_t2_dn4 = assign31380_body70_e32042_d_n4;
            locals.var_t2_dn5 = assign31380_body70_e32042_d_n5;
            locals.var_t2_dn6 = assign31380_body70_e32042_d_n6;
            locals.var_t2_dn7 = assign31380_body70_e32042_d_n7;
            locals.var_t2_dn8 = assign31380_body70_e32042_d_n8;
            locals.var_t2_dn9 = assign31380_body70_e32042_d_n9;
            locals.var_t2_dn10 = assign31380_body70_e32042_d_n10;
            locals.var_t2_dn11 = assign31380_body70_e32042_d_n11;
            locals.var_t2_dn14 = assign31380_body70_e32042_d_n14;
            let (assign31380_body71_e32056, assign31380_body71_e32056_d_n0, assign31380_body71_e32056_d_n2, assign31380_body71_e32056_d_n4, assign31380_body71_e32056_d_n5, assign31380_body71_e32056_d_n6, assign31380_body71_e32056_d_n7, assign31380_body71_e32056_d_n8, assign31380_body71_e32056_d_n9, assign31380_body71_e32056_d_n10, assign31380_body71_e32056_d_n11, assign31380_body71_e32056_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard721 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
            locals.var_t7 = assign31380_body71_e32056;
            locals.var_t7_dn0 = assign31380_body71_e32056_d_n0;
            locals.var_t7_dn2 = assign31380_body71_e32056_d_n2;
            locals.var_t7_dn4 = assign31380_body71_e32056_d_n4;
            locals.var_t7_dn5 = assign31380_body71_e32056_d_n5;
            locals.var_t7_dn6 = assign31380_body71_e32056_d_n6;
            locals.var_t7_dn7 = assign31380_body71_e32056_d_n7;
            locals.var_t7_dn8 = assign31380_body71_e32056_d_n8;
            locals.var_t7_dn9 = assign31380_body71_e32056_d_n9;
            locals.var_t7_dn10 = assign31380_body71_e32056_d_n10;
            locals.var_t7_dn11 = assign31380_body71_e32056_d_n11;
            locals.var_t7_dn14 = assign31380_body71_e32056_d_n14;
            let (assign31380_body72_e32070, assign31380_body72_e32070_d_n0, assign31380_body72_e32070_d_n2, assign31380_body72_e32070_d_n4, assign31380_body72_e32070_d_n5, assign31380_body72_e32070_d_n6, assign31380_body72_e32070_d_n7, assign31380_body72_e32070_d_n8, assign31380_body72_e32070_d_n9, assign31380_body72_e32070_d_n10, assign31380_body72_e32070_d_n11, assign31380_body72_e32070_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body72_e32067: f64 = (locals.var_c_2esipq_nsub * locals.var_t2);
        let assign31380_body72_e32068: f64 = (assign31380_body72_e32067).sqrt();
        (assign31380_body72_e32068, (((locals.var_c_2esipq_nsub_dn0 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn0)) / (2.0 * assign31380_body72_e32068)), (((locals.var_c_2esipq_nsub_dn2 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn2)) / (2.0 * assign31380_body72_e32068)), (((locals.var_c_2esipq_nsub_dn4 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn4)) / (2.0 * assign31380_body72_e32068)), (((locals.var_c_2esipq_nsub_dn5 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn5)) / (2.0 * assign31380_body72_e32068)), (((locals.var_c_2esipq_nsub_dn6 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn6)) / (2.0 * assign31380_body72_e32068)), (((locals.var_c_2esipq_nsub_dn7 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn7)) / (2.0 * assign31380_body72_e32068)), (((locals.var_c_2esipq_nsub_dn8 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn8)) / (2.0 * assign31380_body72_e32068)), (((locals.var_c_2esipq_nsub_dn9 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn9)) / (2.0 * assign31380_body72_e32068)), (((locals.var_c_2esipq_nsub_dn10 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn10)) / (2.0 * assign31380_body72_e32068)), (((locals.var_c_2esipq_nsub_dn11 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn11)) / (2.0 * assign31380_body72_e32068)), (((locals.var_c_2esipq_nsub_dn14 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn14)) / (2.0 * assign31380_body72_e32068)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn11, locals.var_w_subl_dn14,)
    }
};
            locals.var_w_subl = assign31380_body72_e32070;
            locals.var_w_subl_dn0 = assign31380_body72_e32070_d_n0;
            locals.var_w_subl_dn2 = assign31380_body72_e32070_d_n2;
            locals.var_w_subl_dn4 = assign31380_body72_e32070_d_n4;
            locals.var_w_subl_dn5 = assign31380_body72_e32070_d_n5;
            locals.var_w_subl_dn6 = assign31380_body72_e32070_d_n6;
            locals.var_w_subl_dn7 = assign31380_body72_e32070_d_n7;
            locals.var_w_subl_dn8 = assign31380_body72_e32070_d_n8;
            locals.var_w_subl_dn9 = assign31380_body72_e32070_d_n9;
            locals.var_w_subl_dn10 = assign31380_body72_e32070_d_n10;
            locals.var_w_subl_dn11 = assign31380_body72_e32070_d_n11;
            locals.var_w_subl_dn14 = assign31380_body72_e32070_d_n14;
            let (assign31380_body73_e32083, assign31380_body73_e32083_d_n0, assign31380_body73_e32083_d_n2, assign31380_body73_e32083_d_n4, assign31380_body73_e32083_d_n5, assign31380_body73_e32083_d_n6, assign31380_body73_e32083_d_n7, assign31380_body73_e32083_d_n8, assign31380_body73_e32083_d_n9, assign31380_body73_e32083_d_n10, assign31380_body73_e32083_d_n11, assign31380_body73_e32083_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body73_e32081: f64 = (locals.var_w_bl * locals.var_q_ndepm);
        (assign31380_body73_e32081, ((locals.var_w_bl_dn0 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn0)), ((locals.var_w_bl_dn2 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn2)), ((locals.var_w_bl_dn4 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn4)), ((locals.var_w_bl_dn5 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn5)), ((locals.var_w_bl_dn6 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn6)), ((locals.var_w_bl_dn7 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn7)), ((locals.var_w_bl_dn8 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn8)), ((locals.var_w_bl_dn9 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn9)), ((locals.var_w_bl_dn10 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn10)), ((locals.var_w_bl_dn11 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn11)), ((locals.var_w_bl_dn14 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
            locals.var_q_bl_dep = assign31380_body73_e32083;
            locals.var_q_bl_dep_dn0 = assign31380_body73_e32083_d_n0;
            locals.var_q_bl_dep_dn2 = assign31380_body73_e32083_d_n2;
            locals.var_q_bl_dep_dn4 = assign31380_body73_e32083_d_n4;
            locals.var_q_bl_dep_dn5 = assign31380_body73_e32083_d_n5;
            locals.var_q_bl_dep_dn6 = assign31380_body73_e32083_d_n6;
            locals.var_q_bl_dep_dn7 = assign31380_body73_e32083_d_n7;
            locals.var_q_bl_dep_dn8 = assign31380_body73_e32083_d_n8;
            locals.var_q_bl_dep_dn9 = assign31380_body73_e32083_d_n9;
            locals.var_q_bl_dep_dn10 = assign31380_body73_e32083_d_n10;
            locals.var_q_bl_dep_dn11 = assign31380_body73_e32083_d_n11;
            locals.var_q_bl_dep_dn14 = assign31380_body73_e32083_d_n14;
            let (assign31380_body74_e32099, assign31380_body74_e32099_d_n0, assign31380_body74_e32099_d_n2, assign31380_body74_e32099_d_n4, assign31380_body74_e32099_d_n5, assign31380_body74_e32099_d_n6, assign31380_body74_e32099_d_n7, assign31380_body74_e32099_d_n8, assign31380_body74_e32099_d_n9, assign31380_body74_e32099_d_n10, assign31380_body74_e32099_d_n11, assign31380_body74_e32099_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body74_e32093: f64 = (-1.034943e-10);
        let assign31380_body74_e32095: f64 = (assign31380_body74_e32093 / locals.var_w_bl);
        let assign31380_body74_e32097: f64 = (assign31380_body74_e32095 * locals.var_t0);
        (assign31380_body74_e32097, (((-((assign31380_body74_e32093 * locals.var_w_bl_dn0) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn0)), (((-((assign31380_body74_e32093 * locals.var_w_bl_dn2) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn2)), (((-((assign31380_body74_e32093 * locals.var_w_bl_dn4) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn4)), (((-((assign31380_body74_e32093 * locals.var_w_bl_dn5) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn5)), (((-((assign31380_body74_e32093 * locals.var_w_bl_dn6) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn6)), (((-((assign31380_body74_e32093 * locals.var_w_bl_dn7) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn7)), (((-((assign31380_body74_e32093 * locals.var_w_bl_dn8) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn8)), (((-((assign31380_body74_e32093 * locals.var_w_bl_dn9) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn9)), (((-((assign31380_body74_e32093 * locals.var_w_bl_dn10) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn10)), (((-((assign31380_body74_e32093 * locals.var_w_bl_dn11) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn11)), (((-((assign31380_body74_e32093 * locals.var_w_bl_dn14) / (locals.var_w_bl * locals.var_w_bl))) * locals.var_t0) + (assign31380_body74_e32095 * locals.var_t0_dn14)),)
    } else {
        (locals.var_q_bl_dep_dpd, locals.var_q_bl_dep_dpd_dn0, locals.var_q_bl_dep_dpd_dn2, locals.var_q_bl_dep_dpd_dn4, locals.var_q_bl_dep_dpd_dn5, locals.var_q_bl_dep_dpd_dn6, locals.var_q_bl_dep_dpd_dn7, locals.var_q_bl_dep_dpd_dn8, locals.var_q_bl_dep_dpd_dn9, locals.var_q_bl_dep_dpd_dn10, locals.var_q_bl_dep_dpd_dn11, locals.var_q_bl_dep_dpd_dn14,)
    }
};
            locals.var_q_bl_dep_dpd = assign31380_body74_e32099;
            locals.var_q_bl_dep_dpd_dn0 = assign31380_body74_e32099_d_n0;
            locals.var_q_bl_dep_dpd_dn2 = assign31380_body74_e32099_d_n2;
            locals.var_q_bl_dep_dpd_dn4 = assign31380_body74_e32099_d_n4;
            locals.var_q_bl_dep_dpd_dn5 = assign31380_body74_e32099_d_n5;
            locals.var_q_bl_dep_dpd_dn6 = assign31380_body74_e32099_d_n6;
            locals.var_q_bl_dep_dpd_dn7 = assign31380_body74_e32099_d_n7;
            locals.var_q_bl_dep_dpd_dn8 = assign31380_body74_e32099_d_n8;
            locals.var_q_bl_dep_dpd_dn9 = assign31380_body74_e32099_d_n9;
            locals.var_q_bl_dep_dpd_dn10 = assign31380_body74_e32099_d_n10;
            locals.var_q_bl_dep_dpd_dn11 = assign31380_body74_e32099_d_n11;
            locals.var_q_bl_dep_dpd_dn14 = assign31380_body74_e32099_d_n14;
            let (assign31380_body75_e32113, assign31380_body75_e32113_d_n0, assign31380_body75_e32113_d_n2, assign31380_body75_e32113_d_n4, assign31380_body75_e32113_d_n5, assign31380_body75_e32113_d_n6, assign31380_body75_e32113_d_n7, assign31380_body75_e32113_d_n8, assign31380_body75_e32113_d_n9, assign31380_body75_e32113_d_n10, assign31380_body75_e32113_d_n11, assign31380_body75_e32113_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body75_e32109: f64 = (-locals.var_w_subl);
        let assign31380_body75_e32111: f64 = (assign31380_body75_e32109 * locals.var_q_nsub__blk546);
        (assign31380_body75_e32111, (((-locals.var_w_subl_dn0) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn0)), (((-locals.var_w_subl_dn2) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn2)), (((-locals.var_w_subl_dn4) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn4)), (((-locals.var_w_subl_dn5) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn5)), (((-locals.var_w_subl_dn6) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn6)), (((-locals.var_w_subl_dn7) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn7)), (((-locals.var_w_subl_dn8) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn8)), (((-locals.var_w_subl_dn9) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn9)), (((-locals.var_w_subl_dn10) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn10)), (((-locals.var_w_subl_dn11) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn11)), (((-locals.var_w_subl_dn14) * locals.var_q_nsub__blk546) + (assign31380_body75_e32109 * locals.var_q_nsub__blk546_dn14)),)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn11, locals.var_q_subl_dep_dn14,)
    }
};
            locals.var_q_subl_dep = assign31380_body75_e32113;
            locals.var_q_subl_dep_dn0 = assign31380_body75_e32113_d_n0;
            locals.var_q_subl_dep_dn2 = assign31380_body75_e32113_d_n2;
            locals.var_q_subl_dep_dn4 = assign31380_body75_e32113_d_n4;
            locals.var_q_subl_dep_dn5 = assign31380_body75_e32113_d_n5;
            locals.var_q_subl_dep_dn6 = assign31380_body75_e32113_d_n6;
            locals.var_q_subl_dep_dn7 = assign31380_body75_e32113_d_n7;
            locals.var_q_subl_dep_dn8 = assign31380_body75_e32113_d_n8;
            locals.var_q_subl_dep_dn9 = assign31380_body75_e32113_d_n9;
            locals.var_q_subl_dep_dn10 = assign31380_body75_e32113_d_n10;
            locals.var_q_subl_dep_dn11 = assign31380_body75_e32113_d_n11;
            locals.var_q_subl_dep_dn14 = assign31380_body75_e32113_d_n14;
            let (assign31380_body76_e32129, assign31380_body76_e32129_d_n0, assign31380_body76_e32129_d_n2, assign31380_body76_e32129_d_n4, assign31380_body76_e32129_d_n5, assign31380_body76_e32129_d_n6, assign31380_body76_e32129_d_n7, assign31380_body76_e32129_d_n8, assign31380_body76_e32129_d_n9, assign31380_body76_e32129_d_n10, assign31380_body76_e32129_d_n11, assign31380_body76_e32129_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body76_e32123: f64 = (-1.034943e-10);
        let assign31380_body76_e32125: f64 = (assign31380_body76_e32123 / locals.var_w_subl);
        let assign31380_body76_e32127: f64 = (assign31380_body76_e32125 * locals.var_t7);
        (assign31380_body76_e32127, (((-((assign31380_body76_e32123 * locals.var_w_subl_dn0) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn0)), (((-((assign31380_body76_e32123 * locals.var_w_subl_dn2) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn2)), (((-((assign31380_body76_e32123 * locals.var_w_subl_dn4) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn4)), (((-((assign31380_body76_e32123 * locals.var_w_subl_dn5) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn5)), (((-((assign31380_body76_e32123 * locals.var_w_subl_dn6) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn6)), (((-((assign31380_body76_e32123 * locals.var_w_subl_dn7) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn7)), (((-((assign31380_body76_e32123 * locals.var_w_subl_dn8) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn8)), (((-((assign31380_body76_e32123 * locals.var_w_subl_dn9) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn9)), (((-((assign31380_body76_e32123 * locals.var_w_subl_dn10) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn10)), (((-((assign31380_body76_e32123 * locals.var_w_subl_dn11) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn11)), (((-((assign31380_body76_e32123 * locals.var_w_subl_dn14) / (locals.var_w_subl * locals.var_w_subl))) * locals.var_t7) + (assign31380_body76_e32125 * locals.var_t7_dn14)),)
    } else {
        (locals.var_q_subl_dep_dpd, locals.var_q_subl_dep_dpd_dn0, locals.var_q_subl_dep_dpd_dn2, locals.var_q_subl_dep_dpd_dn4, locals.var_q_subl_dep_dpd_dn5, locals.var_q_subl_dep_dpd_dn6, locals.var_q_subl_dep_dpd_dn7, locals.var_q_subl_dep_dpd_dn8, locals.var_q_subl_dep_dpd_dn9, locals.var_q_subl_dep_dpd_dn10, locals.var_q_subl_dep_dpd_dn11, locals.var_q_subl_dep_dpd_dn14,)
    }
};
            locals.var_q_subl_dep_dpd = assign31380_body76_e32129;
            locals.var_q_subl_dep_dpd_dn0 = assign31380_body76_e32129_d_n0;
            locals.var_q_subl_dep_dpd_dn2 = assign31380_body76_e32129_d_n2;
            locals.var_q_subl_dep_dpd_dn4 = assign31380_body76_e32129_d_n4;
            locals.var_q_subl_dep_dpd_dn5 = assign31380_body76_e32129_d_n5;
            locals.var_q_subl_dep_dpd_dn6 = assign31380_body76_e32129_d_n6;
            locals.var_q_subl_dep_dpd_dn7 = assign31380_body76_e32129_d_n7;
            locals.var_q_subl_dep_dpd_dn8 = assign31380_body76_e32129_d_n8;
            locals.var_q_subl_dep_dpd_dn9 = assign31380_body76_e32129_d_n9;
            locals.var_q_subl_dep_dpd_dn10 = assign31380_body76_e32129_d_n10;
            locals.var_q_subl_dep_dpd_dn11 = assign31380_body76_e32129_d_n11;
            locals.var_q_subl_dep_dpd_dn14 = assign31380_body76_e32129_d_n14;
            let (assign31380_body77_e32148, assign31380_body77_e32148_d_n0, assign31380_body77_e32148_d_n2, assign31380_body77_e32148_d_n4, assign31380_body77_e32148_d_n5, assign31380_body77_e32148_d_n6, assign31380_body77_e32148_d_n7, assign31380_body77_e32148_d_n8, assign31380_body77_e32148_d_n9, assign31380_body77_e32148_d_n10, assign31380_body77_e32148_d_n11, assign31380_body77_e32148_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body77_e32141: f64 = (locals.var_vgp0 - locals.var_phi_bl_dep);
        let assign31380_body77_e32142: f64 = (locals.var_cox * assign31380_body77_e32141);
        let assign31380_body77_e32144: f64 = (assign31380_body77_e32142 + locals.var_q_bl_dep);
        let assign31380_body77_e32146: f64 = (assign31380_body77_e32144 + locals.var_q_subl_dep);
        (assign31380_body77_e32146, ((((locals.var_cox_dn0 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn0 - locals.var_phi_bl_dep_dn0))) + locals.var_q_bl_dep_dn0) + locals.var_q_subl_dep_dn0), ((((locals.var_cox_dn2 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn2 - locals.var_phi_bl_dep_dn2))) + locals.var_q_bl_dep_dn2) + locals.var_q_subl_dep_dn2), ((((locals.var_cox_dn4 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn4 - locals.var_phi_bl_dep_dn4))) + locals.var_q_bl_dep_dn4) + locals.var_q_subl_dep_dn4), ((((locals.var_cox_dn5 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn5 - locals.var_phi_bl_dep_dn5))) + locals.var_q_bl_dep_dn5) + locals.var_q_subl_dep_dn5), ((((locals.var_cox_dn6 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn6 - locals.var_phi_bl_dep_dn6))) + locals.var_q_bl_dep_dn6) + locals.var_q_subl_dep_dn6), ((((locals.var_cox_dn7 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn7 - locals.var_phi_bl_dep_dn7))) + locals.var_q_bl_dep_dn7) + locals.var_q_subl_dep_dn7), ((((locals.var_cox_dn8 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn8 - locals.var_phi_bl_dep_dn8))) + locals.var_q_bl_dep_dn8) + locals.var_q_subl_dep_dn8), ((((locals.var_cox_dn9 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn9 - locals.var_phi_bl_dep_dn9))) + locals.var_q_bl_dep_dn9) + locals.var_q_subl_dep_dn9), ((((locals.var_cox_dn10 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn10 - locals.var_phi_bl_dep_dn10))) + locals.var_q_bl_dep_dn10) + locals.var_q_subl_dep_dn10), ((((locals.var_cox_dn11 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn11 - locals.var_phi_bl_dep_dn11))) + locals.var_q_bl_dep_dn11) + locals.var_q_subl_dep_dn11), ((((locals.var_cox_dn14 * assign31380_body77_e32141) + (locals.var_cox * (locals.var_vgp0_dn14 - locals.var_phi_bl_dep_dn14))) + locals.var_q_bl_dep_dn14) + locals.var_q_subl_dep_dn14),)
    } else {
        (locals.var_y1, locals.var_y1_dn0, locals.var_y1_dn2, locals.var_y1_dn4, locals.var_y1_dn5, locals.var_y1_dn6, locals.var_y1_dn7, locals.var_y1_dn8, locals.var_y1_dn9, locals.var_y1_dn10, locals.var_y1_dn11, locals.var_y1_dn14,)
    }
};
            locals.var_y1 = assign31380_body77_e32148;
            locals.var_y1_dn0 = assign31380_body77_e32148_d_n0;
            locals.var_y1_dn2 = assign31380_body77_e32148_d_n2;
            locals.var_y1_dn4 = assign31380_body77_e32148_d_n4;
            locals.var_y1_dn5 = assign31380_body77_e32148_d_n5;
            locals.var_y1_dn6 = assign31380_body77_e32148_d_n6;
            locals.var_y1_dn7 = assign31380_body77_e32148_d_n7;
            locals.var_y1_dn8 = assign31380_body77_e32148_d_n8;
            locals.var_y1_dn9 = assign31380_body77_e32148_d_n9;
            locals.var_y1_dn10 = assign31380_body77_e32148_d_n10;
            locals.var_y1_dn11 = assign31380_body77_e32148_d_n11;
            locals.var_y1_dn14 = assign31380_body77_e32148_d_n14;
            let (assign31380_body78_e32159, assign31380_body78_e32159_d_n0, assign31380_body78_e32159_d_n2, assign31380_body78_e32159_d_n4, assign31380_body78_e32159_d_n5, assign31380_body78_e32159_d_n6, assign31380_body78_e32159_d_n7, assign31380_body78_e32159_d_n8, assign31380_body78_e32159_d_n9, assign31380_body78_e32159_d_n10, assign31380_body78_e32159_d_n11, assign31380_body78_e32159_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn14,)
    } else {
        (locals.var_y11, locals.var_y11_dn0, locals.var_y11_dn2, locals.var_y11_dn4, locals.var_y11_dn5, locals.var_y11_dn6, locals.var_y11_dn7, locals.var_y11_dn8, locals.var_y11_dn9, locals.var_y11_dn10, locals.var_y11_dn11, locals.var_y11_dn14,)
    }
};
            locals.var_y11 = assign31380_body78_e32159;
            locals.var_y11_dn0 = assign31380_body78_e32159_d_n0;
            locals.var_y11_dn2 = assign31380_body78_e32159_d_n2;
            locals.var_y11_dn4 = assign31380_body78_e32159_d_n4;
            locals.var_y11_dn5 = assign31380_body78_e32159_d_n5;
            locals.var_y11_dn6 = assign31380_body78_e32159_d_n6;
            locals.var_y11_dn7 = assign31380_body78_e32159_d_n7;
            locals.var_y11_dn8 = assign31380_body78_e32159_d_n8;
            locals.var_y11_dn9 = assign31380_body78_e32159_d_n9;
            locals.var_y11_dn10 = assign31380_body78_e32159_d_n10;
            locals.var_y11_dn11 = assign31380_body78_e32159_d_n11;
            locals.var_y11_dn14 = assign31380_body78_e32159_d_n14;
            let (assign31380_body79_e32172, assign31380_body79_e32172_d_n0, assign31380_body79_e32172_d_n2, assign31380_body79_e32172_d_n4, assign31380_body79_e32172_d_n5, assign31380_body79_e32172_d_n6, assign31380_body79_e32172_d_n7, assign31380_body79_e32172_d_n8, assign31380_body79_e32172_d_n9, assign31380_body79_e32172_d_n10, assign31380_body79_e32172_d_n11, assign31380_body79_e32172_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body79_e32170: f64 = (locals.var_q_bl_dep_dpd + locals.var_q_subl_dep_dpd);
        (assign31380_body79_e32170, (locals.var_q_bl_dep_dpd_dn0 + locals.var_q_subl_dep_dpd_dn0), (locals.var_q_bl_dep_dpd_dn2 + locals.var_q_subl_dep_dpd_dn2), (locals.var_q_bl_dep_dpd_dn4 + locals.var_q_subl_dep_dpd_dn4), (locals.var_q_bl_dep_dpd_dn5 + locals.var_q_subl_dep_dpd_dn5), (locals.var_q_bl_dep_dpd_dn6 + locals.var_q_subl_dep_dpd_dn6), (locals.var_q_bl_dep_dpd_dn7 + locals.var_q_subl_dep_dpd_dn7), (locals.var_q_bl_dep_dpd_dn8 + locals.var_q_subl_dep_dpd_dn8), (locals.var_q_bl_dep_dpd_dn9 + locals.var_q_subl_dep_dpd_dn9), (locals.var_q_bl_dep_dpd_dn10 + locals.var_q_subl_dep_dpd_dn10), (locals.var_q_bl_dep_dpd_dn11 + locals.var_q_subl_dep_dpd_dn11), (locals.var_q_bl_dep_dpd_dn14 + locals.var_q_subl_dep_dpd_dn14),)
    } else {
        (locals.var_y12, locals.var_y12_dn0, locals.var_y12_dn2, locals.var_y12_dn4, locals.var_y12_dn5, locals.var_y12_dn6, locals.var_y12_dn7, locals.var_y12_dn8, locals.var_y12_dn9, locals.var_y12_dn10, locals.var_y12_dn11, locals.var_y12_dn14,)
    }
};
            locals.var_y12 = assign31380_body79_e32172;
            locals.var_y12_dn0 = assign31380_body79_e32172_d_n0;
            locals.var_y12_dn2 = assign31380_body79_e32172_d_n2;
            locals.var_y12_dn4 = assign31380_body79_e32172_d_n4;
            locals.var_y12_dn5 = assign31380_body79_e32172_d_n5;
            locals.var_y12_dn6 = assign31380_body79_e32172_d_n6;
            locals.var_y12_dn7 = assign31380_body79_e32172_d_n7;
            locals.var_y12_dn8 = assign31380_body79_e32172_d_n8;
            locals.var_y12_dn9 = assign31380_body79_e32172_d_n9;
            locals.var_y12_dn10 = assign31380_body79_e32172_d_n10;
            locals.var_y12_dn11 = assign31380_body79_e32172_d_n11;
            locals.var_y12_dn14 = assign31380_body79_e32172_d_n14;
            let (assign31380_body80_e32193, assign31380_body80_e32193_d_n0, assign31380_body80_e32193_d_n2, assign31380_body80_e32193_d_n4, assign31380_body80_e32193_d_n5, assign31380_body80_e32193_d_n6, assign31380_body80_e32193_d_n7, assign31380_body80_e32193_d_n8, assign31380_body80_e32193_d_n9, assign31380_body80_e32193_d_n10, assign31380_body80_e32193_d_n11, assign31380_body80_e32193_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body80_e32185: f64 = (locals.var_ndepmpnsub * locals.var_phi_bl_dep);
        let assign31380_body80_e32187: f64 = (assign31380_body80_e32185 + locals.var_vbscl__blk437);
        let assign31380_body80_e32189: f64 = (assign31380_body80_e32187 - locals.var_vbi_dep);
        let assign31380_body80_e32190: f64 = (locals.var_ndepmpnsub_inv1 * assign31380_body80_e32189);
        let assign31380_body80_e32191: f64 = (locals.var_phi_jl_dep - assign31380_body80_e32190);
        (assign31380_body80_e32191, (locals.var_phi_jl_dep_dn0 - ((locals.var_ndepmpnsub_inv1_dn0 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn0 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn0)) + locals.var_vbscl__blk437_dn0) - locals.var_vbi_dep_dn0)))), (locals.var_phi_jl_dep_dn2 - ((locals.var_ndepmpnsub_inv1_dn2 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn2 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn2)) + locals.var_vbscl__blk437_dn2) - locals.var_vbi_dep_dn2)))), (locals.var_phi_jl_dep_dn4 - ((locals.var_ndepmpnsub_inv1_dn4 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn4 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn4)) + locals.var_vbscl__blk437_dn4) - locals.var_vbi_dep_dn4)))), (locals.var_phi_jl_dep_dn5 - ((locals.var_ndepmpnsub_inv1_dn5 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn5 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn5)) + locals.var_vbscl__blk437_dn5) - locals.var_vbi_dep_dn5)))), (locals.var_phi_jl_dep_dn6 - ((locals.var_ndepmpnsub_inv1_dn6 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn6 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn6)) + locals.var_vbscl__blk437_dn6) - locals.var_vbi_dep_dn6)))), (locals.var_phi_jl_dep_dn7 - ((locals.var_ndepmpnsub_inv1_dn7 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn7 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn7)) + locals.var_vbscl__blk437_dn7) - locals.var_vbi_dep_dn7)))), (locals.var_phi_jl_dep_dn8 - ((locals.var_ndepmpnsub_inv1_dn8 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn8 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn8)) + locals.var_vbscl__blk437_dn8) - locals.var_vbi_dep_dn8)))), (locals.var_phi_jl_dep_dn9 - ((locals.var_ndepmpnsub_inv1_dn9 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn9 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn9)) + locals.var_vbscl__blk437_dn9) - locals.var_vbi_dep_dn9)))), (locals.var_phi_jl_dep_dn10 - ((locals.var_ndepmpnsub_inv1_dn10 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn10 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn10)) + locals.var_vbscl__blk437_dn10) - locals.var_vbi_dep_dn10)))), (locals.var_phi_jl_dep_dn11 - ((locals.var_ndepmpnsub_inv1_dn11 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn11 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn11)) + locals.var_vbscl__blk437_dn11) - locals.var_vbi_dep_dn11)))), (locals.var_phi_jl_dep_dn14 - ((locals.var_ndepmpnsub_inv1_dn14 * assign31380_body80_e32189) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn14 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn14)) + locals.var_vbscl__blk437_dn14) - locals.var_vbi_dep_dn14)))),)
    } else {
        (locals.var_y2, locals.var_y2_dn0, locals.var_y2_dn2, locals.var_y2_dn4, locals.var_y2_dn5, locals.var_y2_dn6, locals.var_y2_dn7, locals.var_y2_dn8, locals.var_y2_dn9, locals.var_y2_dn10, locals.var_y2_dn11, locals.var_y2_dn14,)
    }
};
            locals.var_y2 = assign31380_body80_e32193;
            locals.var_y2_dn0 = assign31380_body80_e32193_d_n0;
            locals.var_y2_dn2 = assign31380_body80_e32193_d_n2;
            locals.var_y2_dn4 = assign31380_body80_e32193_d_n4;
            locals.var_y2_dn5 = assign31380_body80_e32193_d_n5;
            locals.var_y2_dn6 = assign31380_body80_e32193_d_n6;
            locals.var_y2_dn7 = assign31380_body80_e32193_d_n7;
            locals.var_y2_dn8 = assign31380_body80_e32193_d_n8;
            locals.var_y2_dn9 = assign31380_body80_e32193_d_n9;
            locals.var_y2_dn10 = assign31380_body80_e32193_d_n10;
            locals.var_y2_dn11 = assign31380_body80_e32193_d_n11;
            locals.var_y2_dn14 = assign31380_body80_e32193_d_n14;
            let (assign31380_body81_e32204, assign31380_body81_e32204_d_n0, assign31380_body81_e32204_d_n2, assign31380_body81_e32204_d_n4, assign31380_body81_e32204_d_n5, assign31380_body81_e32204_d_n6, assign31380_body81_e32204_d_n7, assign31380_body81_e32204_d_n8, assign31380_body81_e32204_d_n9, assign31380_body81_e32204_d_n10, assign31380_body81_e32204_d_n11, assign31380_body81_e32204_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_y21, locals.var_y21_dn0, locals.var_y21_dn2, locals.var_y21_dn4, locals.var_y21_dn5, locals.var_y21_dn6, locals.var_y21_dn7, locals.var_y21_dn8, locals.var_y21_dn9, locals.var_y21_dn10, locals.var_y21_dn11, locals.var_y21_dn14,)
    }
};
            locals.var_y21 = assign31380_body81_e32204;
            locals.var_y21_dn0 = assign31380_body81_e32204_d_n0;
            locals.var_y21_dn2 = assign31380_body81_e32204_d_n2;
            locals.var_y21_dn4 = assign31380_body81_e32204_d_n4;
            locals.var_y21_dn5 = assign31380_body81_e32204_d_n5;
            locals.var_y21_dn6 = assign31380_body81_e32204_d_n6;
            locals.var_y21_dn7 = assign31380_body81_e32204_d_n7;
            locals.var_y21_dn8 = assign31380_body81_e32204_d_n8;
            locals.var_y21_dn9 = assign31380_body81_e32204_d_n9;
            locals.var_y21_dn10 = assign31380_body81_e32204_d_n10;
            locals.var_y21_dn11 = assign31380_body81_e32204_d_n11;
            locals.var_y21_dn14 = assign31380_body81_e32204_d_n14;
            let (assign31380_body82_e32215, assign31380_body82_e32215_d_n0, assign31380_body82_e32215_d_n2, assign31380_body82_e32215_d_n4, assign31380_body82_e32215_d_n5, assign31380_body82_e32215_d_n6, assign31380_body82_e32215_d_n7, assign31380_body82_e32215_d_n8, assign31380_body82_e32215_d_n9, assign31380_body82_e32215_d_n10, assign31380_body82_e32215_d_n11, assign31380_body82_e32215_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_y22, locals.var_y22_dn0, locals.var_y22_dn2, locals.var_y22_dn4, locals.var_y22_dn5, locals.var_y22_dn6, locals.var_y22_dn7, locals.var_y22_dn8, locals.var_y22_dn9, locals.var_y22_dn10, locals.var_y22_dn11, locals.var_y22_dn14,)
    }
};
            locals.var_y22 = assign31380_body82_e32215;
            locals.var_y22_dn0 = assign31380_body82_e32215_d_n0;
            locals.var_y22_dn2 = assign31380_body82_e32215_d_n2;
            locals.var_y22_dn4 = assign31380_body82_e32215_d_n4;
            locals.var_y22_dn5 = assign31380_body82_e32215_d_n5;
            locals.var_y22_dn6 = assign31380_body82_e32215_d_n6;
            locals.var_y22_dn7 = assign31380_body82_e32215_d_n7;
            locals.var_y22_dn8 = assign31380_body82_e32215_d_n8;
            locals.var_y22_dn9 = assign31380_body82_e32215_d_n9;
            locals.var_y22_dn10 = assign31380_body82_e32215_d_n10;
            locals.var_y22_dn11 = assign31380_body82_e32215_d_n11;
            locals.var_y22_dn14 = assign31380_body82_e32215_d_n14;
            let (assign31380_body83_e32232, assign31380_body83_e32232_d_n0, assign31380_body83_e32232_d_n2, assign31380_body83_e32232_d_n4, assign31380_body83_e32232_d_n5, assign31380_body83_e32232_d_n6, assign31380_body83_e32232_d_n7, assign31380_body83_e32232_d_n8, assign31380_body83_e32232_d_n9, assign31380_body83_e32232_d_n10, assign31380_body83_e32232_d_n11, assign31380_body83_e32232_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body83_e32226: f64 = (locals.var_y11 * locals.var_y22);
        let assign31380_body83_e32229: f64 = (locals.var_y21 * locals.var_y12);
        let assign31380_body83_e32230: f64 = (assign31380_body83_e32226 - assign31380_body83_e32229);
        (assign31380_body83_e32230, (((locals.var_y11_dn0 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn0)) - ((locals.var_y21_dn0 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn0))), (((locals.var_y11_dn2 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn2)) - ((locals.var_y21_dn2 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn2))), (((locals.var_y11_dn4 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn4)) - ((locals.var_y21_dn4 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn4))), (((locals.var_y11_dn5 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn5)) - ((locals.var_y21_dn5 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn5))), (((locals.var_y11_dn6 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn6)) - ((locals.var_y21_dn6 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn6))), (((locals.var_y11_dn7 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn7)) - ((locals.var_y21_dn7 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn7))), (((locals.var_y11_dn8 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn8)) - ((locals.var_y21_dn8 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn8))), (((locals.var_y11_dn9 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn9)) - ((locals.var_y21_dn9 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn9))), (((locals.var_y11_dn10 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn10)) - ((locals.var_y21_dn10 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn10))), (((locals.var_y11_dn11 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn11)) - ((locals.var_y21_dn11 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn11))), (((locals.var_y11_dn14 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn14)) - ((locals.var_y21_dn14 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn14))),)
    } else {
        (locals.var_dety, locals.var_dety_dn0, locals.var_dety_dn2, locals.var_dety_dn4, locals.var_dety_dn5, locals.var_dety_dn6, locals.var_dety_dn7, locals.var_dety_dn8, locals.var_dety_dn9, locals.var_dety_dn10, locals.var_dety_dn11, locals.var_dety_dn14,)
    }
};
            locals.var_dety = assign31380_body83_e32232;
            locals.var_dety_dn0 = assign31380_body83_e32232_d_n0;
            locals.var_dety_dn2 = assign31380_body83_e32232_d_n2;
            locals.var_dety_dn4 = assign31380_body83_e32232_d_n4;
            locals.var_dety_dn5 = assign31380_body83_e32232_d_n5;
            locals.var_dety_dn6 = assign31380_body83_e32232_d_n6;
            locals.var_dety_dn7 = assign31380_body83_e32232_d_n7;
            locals.var_dety_dn8 = assign31380_body83_e32232_d_n8;
            locals.var_dety_dn9 = assign31380_body83_e32232_d_n9;
            locals.var_dety_dn10 = assign31380_body83_e32232_d_n10;
            locals.var_dety_dn11 = assign31380_body83_e32232_d_n11;
            locals.var_dety_dn14 = assign31380_body83_e32232_d_n14;
            let (assign31380_body84_e32245, assign31380_body84_e32245_d_n0, assign31380_body84_e32245_d_n2, assign31380_body84_e32245_d_n4, assign31380_body84_e32245_d_n5, assign31380_body84_e32245_d_n6, assign31380_body84_e32245_d_n7, assign31380_body84_e32245_d_n8, assign31380_body84_e32245_d_n9, assign31380_body84_e32245_d_n10, assign31380_body84_e32245_d_n11, assign31380_body84_e32245_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body84_e32243: f64 = (locals.var_y22 / locals.var_dety);
        (assign31380_body84_e32243, (((locals.var_y22_dn0 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn2 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn4 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn5 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn6 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn7 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn8 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn9 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn10 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn11 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn14 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev11, locals.var_rev11_dn0, locals.var_rev11_dn2, locals.var_rev11_dn4, locals.var_rev11_dn5, locals.var_rev11_dn6, locals.var_rev11_dn7, locals.var_rev11_dn8, locals.var_rev11_dn9, locals.var_rev11_dn10, locals.var_rev11_dn11, locals.var_rev11_dn14,)
    }
};
            locals.var_rev11 = assign31380_body84_e32245;
            locals.var_rev11_dn0 = assign31380_body84_e32245_d_n0;
            locals.var_rev11_dn2 = assign31380_body84_e32245_d_n2;
            locals.var_rev11_dn4 = assign31380_body84_e32245_d_n4;
            locals.var_rev11_dn5 = assign31380_body84_e32245_d_n5;
            locals.var_rev11_dn6 = assign31380_body84_e32245_d_n6;
            locals.var_rev11_dn7 = assign31380_body84_e32245_d_n7;
            locals.var_rev11_dn8 = assign31380_body84_e32245_d_n8;
            locals.var_rev11_dn9 = assign31380_body84_e32245_d_n9;
            locals.var_rev11_dn10 = assign31380_body84_e32245_d_n10;
            locals.var_rev11_dn11 = assign31380_body84_e32245_d_n11;
            locals.var_rev11_dn14 = assign31380_body84_e32245_d_n14;
            let (assign31380_body85_e32259, assign31380_body85_e32259_d_n0, assign31380_body85_e32259_d_n2, assign31380_body85_e32259_d_n4, assign31380_body85_e32259_d_n5, assign31380_body85_e32259_d_n6, assign31380_body85_e32259_d_n7, assign31380_body85_e32259_d_n8, assign31380_body85_e32259_d_n9, assign31380_body85_e32259_d_n10, assign31380_body85_e32259_d_n11, assign31380_body85_e32259_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body85_e32255: f64 = (-locals.var_y12);
        let assign31380_body85_e32257: f64 = (assign31380_body85_e32255 / locals.var_dety);
        (assign31380_body85_e32257, ((((-locals.var_y12_dn0) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn2) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn4) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn5) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn6) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn7) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn8) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn9) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn10) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn11) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn14) * locals.var_dety) - (assign31380_body85_e32255 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev12, locals.var_rev12_dn0, locals.var_rev12_dn2, locals.var_rev12_dn4, locals.var_rev12_dn5, locals.var_rev12_dn6, locals.var_rev12_dn7, locals.var_rev12_dn8, locals.var_rev12_dn9, locals.var_rev12_dn10, locals.var_rev12_dn11, locals.var_rev12_dn14,)
    }
};
            locals.var_rev12 = assign31380_body85_e32259;
            locals.var_rev12_dn0 = assign31380_body85_e32259_d_n0;
            locals.var_rev12_dn2 = assign31380_body85_e32259_d_n2;
            locals.var_rev12_dn4 = assign31380_body85_e32259_d_n4;
            locals.var_rev12_dn5 = assign31380_body85_e32259_d_n5;
            locals.var_rev12_dn6 = assign31380_body85_e32259_d_n6;
            locals.var_rev12_dn7 = assign31380_body85_e32259_d_n7;
            locals.var_rev12_dn8 = assign31380_body85_e32259_d_n8;
            locals.var_rev12_dn9 = assign31380_body85_e32259_d_n9;
            locals.var_rev12_dn10 = assign31380_body85_e32259_d_n10;
            locals.var_rev12_dn11 = assign31380_body85_e32259_d_n11;
            locals.var_rev12_dn14 = assign31380_body85_e32259_d_n14;
            let (assign31380_body86_e32273, assign31380_body86_e32273_d_n0, assign31380_body86_e32273_d_n2, assign31380_body86_e32273_d_n4, assign31380_body86_e32273_d_n5, assign31380_body86_e32273_d_n6, assign31380_body86_e32273_d_n7, assign31380_body86_e32273_d_n8, assign31380_body86_e32273_d_n9, assign31380_body86_e32273_d_n10, assign31380_body86_e32273_d_n11, assign31380_body86_e32273_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body86_e32269: f64 = (-locals.var_y21);
        let assign31380_body86_e32271: f64 = (assign31380_body86_e32269 / locals.var_dety);
        (assign31380_body86_e32271, ((((-locals.var_y21_dn0) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn2) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn4) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn5) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn6) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn7) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn8) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn9) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn10) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn11) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn14) * locals.var_dety) - (assign31380_body86_e32269 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev21, locals.var_rev21_dn0, locals.var_rev21_dn2, locals.var_rev21_dn4, locals.var_rev21_dn5, locals.var_rev21_dn6, locals.var_rev21_dn7, locals.var_rev21_dn8, locals.var_rev21_dn9, locals.var_rev21_dn10, locals.var_rev21_dn11, locals.var_rev21_dn14,)
    }
};
            locals.var_rev21 = assign31380_body86_e32273;
            locals.var_rev21_dn0 = assign31380_body86_e32273_d_n0;
            locals.var_rev21_dn2 = assign31380_body86_e32273_d_n2;
            locals.var_rev21_dn4 = assign31380_body86_e32273_d_n4;
            locals.var_rev21_dn5 = assign31380_body86_e32273_d_n5;
            locals.var_rev21_dn6 = assign31380_body86_e32273_d_n6;
            locals.var_rev21_dn7 = assign31380_body86_e32273_d_n7;
            locals.var_rev21_dn8 = assign31380_body86_e32273_d_n8;
            locals.var_rev21_dn9 = assign31380_body86_e32273_d_n9;
            locals.var_rev21_dn10 = assign31380_body86_e32273_d_n10;
            locals.var_rev21_dn11 = assign31380_body86_e32273_d_n11;
            locals.var_rev21_dn14 = assign31380_body86_e32273_d_n14;
            let (assign31380_body87_e32286, assign31380_body87_e32286_d_n0, assign31380_body87_e32286_d_n2, assign31380_body87_e32286_d_n4, assign31380_body87_e32286_d_n5, assign31380_body87_e32286_d_n6, assign31380_body87_e32286_d_n7, assign31380_body87_e32286_d_n8, assign31380_body87_e32286_d_n9, assign31380_body87_e32286_d_n10, assign31380_body87_e32286_d_n11, assign31380_body87_e32286_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body87_e32284: f64 = (locals.var_y11 / locals.var_dety);
        (assign31380_body87_e32284, (((locals.var_y11_dn0 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn2 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn4 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn5 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn6 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn7 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn8 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn9 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn10 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn11 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn11)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn14 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn14)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev22, locals.var_rev22_dn0, locals.var_rev22_dn2, locals.var_rev22_dn4, locals.var_rev22_dn5, locals.var_rev22_dn6, locals.var_rev22_dn7, locals.var_rev22_dn8, locals.var_rev22_dn9, locals.var_rev22_dn10, locals.var_rev22_dn11, locals.var_rev22_dn14,)
    }
};
            locals.var_rev22 = assign31380_body87_e32286;
            locals.var_rev22_dn0 = assign31380_body87_e32286_d_n0;
            locals.var_rev22_dn2 = assign31380_body87_e32286_d_n2;
            locals.var_rev22_dn4 = assign31380_body87_e32286_d_n4;
            locals.var_rev22_dn5 = assign31380_body87_e32286_d_n5;
            locals.var_rev22_dn6 = assign31380_body87_e32286_d_n6;
            locals.var_rev22_dn7 = assign31380_body87_e32286_d_n7;
            locals.var_rev22_dn8 = assign31380_body87_e32286_d_n8;
            locals.var_rev22_dn9 = assign31380_body87_e32286_d_n9;
            locals.var_rev22_dn10 = assign31380_body87_e32286_d_n10;
            locals.var_rev22_dn11 = assign31380_body87_e32286_d_n11;
            locals.var_rev22_dn14 = assign31380_body87_e32286_d_n14;
            let assign31380_body88_e32289: f64 = (locals.var_rev11 * locals.var_y1);
            let assign31380_body88_e32292: f64 = (locals.var_rev12 * locals.var_y2);
            let assign31380_body88_e32293: f64 = (assign31380_body88_e32289 + assign31380_body88_e32292);
            let assign31380_body88_e32294: f64 = (assign31380_body88_e32293).abs();
            let assign31380_body88_e32296: f64 = if assign31380_body88_e32294 > 0.5 { 1.0 } else { 0.0 };
            locals.var_guard727 = assign31380_body88_e32296;
            let (assign31380_body89_e32325, assign31380_body89_e32325_d_n0, assign31380_body89_e32325_d_n2, assign31380_body89_e32325_d_n4, assign31380_body89_e32325_d_n5, assign31380_body89_e32325_d_n6, assign31380_body89_e32325_d_n7, assign31380_body89_e32325_d_n8, assign31380_body89_e32325_d_n9, assign31380_body89_e32325_d_n10, assign31380_body89_e32325_d_n11, assign31380_body89_e32325_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard727 != 0.0)) {
        let assign31380_body89_e32311: f64 = (locals.var_rev11 * locals.var_y1);
        let assign31380_body89_e32314: f64 = (locals.var_rev12 * locals.var_y2);
        let assign31380_body89_e32315: f64 = (assign31380_body89_e32311 + assign31380_body89_e32314);
        let (assign31380_body89_e32321,) = {
            if (assign31380_body89_e32315 >= 0.0) {
                (1.0,)
            } else {
                let assign31380_body89_e32320: f64 = (-1.0);
                (assign31380_body89_e32320,)
            }
        };
        let assign31380_body89_e32322: f64 = (0.5 * assign31380_body89_e32321);
        let assign31380_body89_e32323: f64 = (locals.var_vgp0 - assign31380_body89_e32322);
        (assign31380_body89_e32323, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
            locals.var_vgp0 = assign31380_body89_e32325;
            locals.var_vgp0_dn0 = assign31380_body89_e32325_d_n0;
            locals.var_vgp0_dn2 = assign31380_body89_e32325_d_n2;
            locals.var_vgp0_dn4 = assign31380_body89_e32325_d_n4;
            locals.var_vgp0_dn5 = assign31380_body89_e32325_d_n5;
            locals.var_vgp0_dn6 = assign31380_body89_e32325_d_n6;
            locals.var_vgp0_dn7 = assign31380_body89_e32325_d_n7;
            locals.var_vgp0_dn8 = assign31380_body89_e32325_d_n8;
            locals.var_vgp0_dn9 = assign31380_body89_e32325_d_n9;
            locals.var_vgp0_dn10 = assign31380_body89_e32325_d_n10;
            locals.var_vgp0_dn11 = assign31380_body89_e32325_d_n11;
            locals.var_vgp0_dn14 = assign31380_body89_e32325_d_n14;
            let (assign31380_body90_e32354, assign31380_body90_e32354_d_n0, assign31380_body90_e32354_d_n2, assign31380_body90_e32354_d_n4, assign31380_body90_e32354_d_n5, assign31380_body90_e32354_d_n6, assign31380_body90_e32354_d_n7, assign31380_body90_e32354_d_n8, assign31380_body90_e32354_d_n9, assign31380_body90_e32354_d_n10, assign31380_body90_e32354_d_n11, assign31380_body90_e32354_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard727 != 0.0)) {
        let assign31380_body90_e32340: f64 = (locals.var_rev21 * locals.var_y1);
        let assign31380_body90_e32343: f64 = (locals.var_rev22 * locals.var_y2);
        let assign31380_body90_e32344: f64 = (assign31380_body90_e32340 + assign31380_body90_e32343);
        let (assign31380_body90_e32350,) = {
            if (assign31380_body90_e32344 >= 0.0) {
                (1.0,)
            } else {
                let assign31380_body90_e32349: f64 = (-1.0);
                (assign31380_body90_e32349,)
            }
        };
        let assign31380_body90_e32351: f64 = (0.5 * assign31380_body90_e32350);
        let assign31380_body90_e32352: f64 = (locals.var_phi_jl_dep - assign31380_body90_e32351);
        (assign31380_body90_e32352, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
            locals.var_phi_jl_dep = assign31380_body90_e32354;
            locals.var_phi_jl_dep_dn0 = assign31380_body90_e32354_d_n0;
            locals.var_phi_jl_dep_dn2 = assign31380_body90_e32354_d_n2;
            locals.var_phi_jl_dep_dn4 = assign31380_body90_e32354_d_n4;
            locals.var_phi_jl_dep_dn5 = assign31380_body90_e32354_d_n5;
            locals.var_phi_jl_dep_dn6 = assign31380_body90_e32354_d_n6;
            locals.var_phi_jl_dep_dn7 = assign31380_body90_e32354_d_n7;
            locals.var_phi_jl_dep_dn8 = assign31380_body90_e32354_d_n8;
            locals.var_phi_jl_dep_dn9 = assign31380_body90_e32354_d_n9;
            locals.var_phi_jl_dep_dn10 = assign31380_body90_e32354_d_n10;
            locals.var_phi_jl_dep_dn11 = assign31380_body90_e32354_d_n11;
            locals.var_phi_jl_dep_dn14 = assign31380_body90_e32354_d_n14;
            let (assign31380_body91_e32376, assign31380_body91_e32376_d_n0, assign31380_body91_e32376_d_n2, assign31380_body91_e32376_d_n4, assign31380_body91_e32376_d_n5, assign31380_body91_e32376_d_n6, assign31380_body91_e32376_d_n7, assign31380_body91_e32376_d_n8, assign31380_body91_e32376_d_n9, assign31380_body91_e32376_d_n10, assign31380_body91_e32376_d_n11, assign31380_body91_e32376_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard727 == 0.0)) {
        let assign31380_body91_e32369: f64 = (locals.var_rev11 * locals.var_y1);
        let assign31380_body91_e32372: f64 = (locals.var_rev12 * locals.var_y2);
        let assign31380_body91_e32373: f64 = (assign31380_body91_e32369 + assign31380_body91_e32372);
        let assign31380_body91_e32374: f64 = (locals.var_vgp0 - assign31380_body91_e32373);
        (assign31380_body91_e32374, (locals.var_vgp0_dn0 - (((locals.var_rev11_dn0 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn0)) + ((locals.var_rev12_dn0 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn0)))), (locals.var_vgp0_dn2 - (((locals.var_rev11_dn2 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn2)) + ((locals.var_rev12_dn2 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn2)))), (locals.var_vgp0_dn4 - (((locals.var_rev11_dn4 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn4)) + ((locals.var_rev12_dn4 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn4)))), (locals.var_vgp0_dn5 - (((locals.var_rev11_dn5 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn5)) + ((locals.var_rev12_dn5 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn5)))), (locals.var_vgp0_dn6 - (((locals.var_rev11_dn6 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn6)) + ((locals.var_rev12_dn6 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn6)))), (locals.var_vgp0_dn7 - (((locals.var_rev11_dn7 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn7)) + ((locals.var_rev12_dn7 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn7)))), (locals.var_vgp0_dn8 - (((locals.var_rev11_dn8 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn8)) + ((locals.var_rev12_dn8 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn8)))), (locals.var_vgp0_dn9 - (((locals.var_rev11_dn9 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn9)) + ((locals.var_rev12_dn9 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn9)))), (locals.var_vgp0_dn10 - (((locals.var_rev11_dn10 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn10)) + ((locals.var_rev12_dn10 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn10)))), (locals.var_vgp0_dn11 - (((locals.var_rev11_dn11 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn11)) + ((locals.var_rev12_dn11 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn11)))), (locals.var_vgp0_dn14 - (((locals.var_rev11_dn14 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn14)) + ((locals.var_rev12_dn14 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn14)))),)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
            locals.var_vgp0 = assign31380_body91_e32376;
            locals.var_vgp0_dn0 = assign31380_body91_e32376_d_n0;
            locals.var_vgp0_dn2 = assign31380_body91_e32376_d_n2;
            locals.var_vgp0_dn4 = assign31380_body91_e32376_d_n4;
            locals.var_vgp0_dn5 = assign31380_body91_e32376_d_n5;
            locals.var_vgp0_dn6 = assign31380_body91_e32376_d_n6;
            locals.var_vgp0_dn7 = assign31380_body91_e32376_d_n7;
            locals.var_vgp0_dn8 = assign31380_body91_e32376_d_n8;
            locals.var_vgp0_dn9 = assign31380_body91_e32376_d_n9;
            locals.var_vgp0_dn10 = assign31380_body91_e32376_d_n10;
            locals.var_vgp0_dn11 = assign31380_body91_e32376_d_n11;
            locals.var_vgp0_dn14 = assign31380_body91_e32376_d_n14;
            let (assign31380_body92_e32398, assign31380_body92_e32398_d_n0, assign31380_body92_e32398_d_n2, assign31380_body92_e32398_d_n4, assign31380_body92_e32398_d_n5, assign31380_body92_e32398_d_n6, assign31380_body92_e32398_d_n7, assign31380_body92_e32398_d_n8, assign31380_body92_e32398_d_n9, assign31380_body92_e32398_d_n10, assign31380_body92_e32398_d_n11, assign31380_body92_e32398_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard727 == 0.0)) {
        let assign31380_body92_e32391: f64 = (locals.var_rev21 * locals.var_y1);
        let assign31380_body92_e32394: f64 = (locals.var_rev22 * locals.var_y2);
        let assign31380_body92_e32395: f64 = (assign31380_body92_e32391 + assign31380_body92_e32394);
        let assign31380_body92_e32396: f64 = (locals.var_phi_jl_dep - assign31380_body92_e32395);
        (assign31380_body92_e32396, (locals.var_phi_jl_dep_dn0 - (((locals.var_rev21_dn0 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn0)) + ((locals.var_rev22_dn0 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn0)))), (locals.var_phi_jl_dep_dn2 - (((locals.var_rev21_dn2 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn2)) + ((locals.var_rev22_dn2 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn2)))), (locals.var_phi_jl_dep_dn4 - (((locals.var_rev21_dn4 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn4)) + ((locals.var_rev22_dn4 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn4)))), (locals.var_phi_jl_dep_dn5 - (((locals.var_rev21_dn5 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn5)) + ((locals.var_rev22_dn5 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn5)))), (locals.var_phi_jl_dep_dn6 - (((locals.var_rev21_dn6 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn6)) + ((locals.var_rev22_dn6 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn6)))), (locals.var_phi_jl_dep_dn7 - (((locals.var_rev21_dn7 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn7)) + ((locals.var_rev22_dn7 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn7)))), (locals.var_phi_jl_dep_dn8 - (((locals.var_rev21_dn8 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn8)) + ((locals.var_rev22_dn8 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn8)))), (locals.var_phi_jl_dep_dn9 - (((locals.var_rev21_dn9 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn9)) + ((locals.var_rev22_dn9 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn9)))), (locals.var_phi_jl_dep_dn10 - (((locals.var_rev21_dn10 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn10)) + ((locals.var_rev22_dn10 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn10)))), (locals.var_phi_jl_dep_dn11 - (((locals.var_rev21_dn11 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn11)) + ((locals.var_rev22_dn11 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn11)))), (locals.var_phi_jl_dep_dn14 - (((locals.var_rev21_dn14 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn14)) + ((locals.var_rev22_dn14 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn14)))),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
            locals.var_phi_jl_dep = assign31380_body92_e32398;
            locals.var_phi_jl_dep_dn0 = assign31380_body92_e32398_d_n0;
            locals.var_phi_jl_dep_dn2 = assign31380_body92_e32398_d_n2;
            locals.var_phi_jl_dep_dn4 = assign31380_body92_e32398_d_n4;
            locals.var_phi_jl_dep_dn5 = assign31380_body92_e32398_d_n5;
            locals.var_phi_jl_dep_dn6 = assign31380_body92_e32398_d_n6;
            locals.var_phi_jl_dep_dn7 = assign31380_body92_e32398_d_n7;
            locals.var_phi_jl_dep_dn8 = assign31380_body92_e32398_d_n8;
            locals.var_phi_jl_dep_dn9 = assign31380_body92_e32398_d_n9;
            locals.var_phi_jl_dep_dn10 = assign31380_body92_e32398_d_n10;
            locals.var_phi_jl_dep_dn11 = assign31380_body92_e32398_d_n11;
            locals.var_phi_jl_dep_dn14 = assign31380_body92_e32398_d_n14;
            let assign31380_body93_e32401: f64 = (locals.var_vgp0 - locals.var_vgp0old);
            let assign31380_body93_e32402: f64 = (assign31380_body93_e32401).abs();
            let assign31380_body93_e32407: f64 = (locals.var_phi_jl_dep - locals.var_phi_jl_dep_old);
            let assign31380_body93_e32408: f64 = (assign31380_body93_e32407).abs();
            let assign31380_body93_e32411: f64 = if ((assign31380_body93_e32402 <= 1e-12) && (assign31380_body93_e32408 <= 1e-12)) { 1.0 } else { 0.0 };
            locals.var_guard728 = assign31380_body93_e32411;
            let (assign31380_body94_e32426,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard728 != 0.0)) {
        let assign31380_body94_e32424: f64 = (150.0 + 1.0);
        (assign31380_body94_e32424,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31380_body94_e32426;
            let (assign31380_body95_e32437,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_vgp0,)
    } else {
        (locals.var_vgp0old,)
    }
};
            locals.var_vgp0old = assign31380_body95_e32437;
            let (assign31380_body96_e32448,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_phi_jl_dep,)
    } else {
        (locals.var_phi_jl_dep_old,)
    }
};
            locals.var_phi_jl_dep_old = assign31380_body96_e32448;
            let (assign31380_body97_e32461,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31380_body97_e32459: f64 = (locals.var_lp_s0 + 1.0);
        (assign31380_body97_e32459,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31380_body97_e32461;
        }

    }

    pub(super) fn stamp_transient_block_93(
        locals: &mut StampLocals,
    ) {
        let (assign31390_e32472, assign31390_e32472_d_n0, assign31390_e32472_d_n2, assign31390_e32472_d_n4, assign31390_e32472_d_n5, assign31390_e32472_d_n6, assign31390_e32472_d_n7, assign31390_e32472_d_n8, assign31390_e32472_d_n9, assign31390_e32472_d_n10, assign31390_e32472_d_n11, assign31390_e32472_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    } else {
        (locals.var_phi_jl_dep_acc, locals.var_phi_jl_dep_acc_dn0, locals.var_phi_jl_dep_acc_dn2, locals.var_phi_jl_dep_acc_dn4, locals.var_phi_jl_dep_acc_dn5, locals.var_phi_jl_dep_acc_dn6, locals.var_phi_jl_dep_acc_dn7, locals.var_phi_jl_dep_acc_dn8, locals.var_phi_jl_dep_acc_dn9, locals.var_phi_jl_dep_acc_dn10, locals.var_phi_jl_dep_acc_dn11, locals.var_phi_jl_dep_acc_dn14,)
    }
};
        locals.var_phi_jl_dep_acc = assign31390_e32472;
        locals.var_phi_jl_dep_acc_dn0 = assign31390_e32472_d_n0;
        locals.var_phi_jl_dep_acc_dn2 = assign31390_e32472_d_n2;
        locals.var_phi_jl_dep_acc_dn4 = assign31390_e32472_d_n4;
        locals.var_phi_jl_dep_acc_dn5 = assign31390_e32472_d_n5;
        locals.var_phi_jl_dep_acc_dn6 = assign31390_e32472_d_n6;
        locals.var_phi_jl_dep_acc_dn7 = assign31390_e32472_d_n7;
        locals.var_phi_jl_dep_acc_dn8 = assign31390_e32472_d_n8;
        locals.var_phi_jl_dep_acc_dn9 = assign31390_e32472_d_n9;
        locals.var_phi_jl_dep_acc_dn10 = assign31390_e32472_d_n10;
        locals.var_phi_jl_dep_acc_dn11 = assign31390_e32472_d_n11;
        locals.var_phi_jl_dep_acc_dn14 = assign31390_e32472_d_n14;

        let (assign31400_e32485, assign31400_e32485_d_n0, assign31400_e32485_d_n2, assign31400_e32485_d_n4, assign31400_e32485_d_n5, assign31400_e32485_d_n6, assign31400_e32485_d_n7, assign31400_e32485_d_n8, assign31400_e32485_d_n9, assign31400_e32485_d_n10, assign31400_e32485_d_n11, assign31400_e32485_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31400_e32483: f64 = (locals.var_uc_depthn * locals.var_ndepmpnsub);
        (assign31400_e32483, ((locals.var_uc_depthn_dn0 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn0)), ((locals.var_uc_depthn_dn2 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn2)), ((locals.var_uc_depthn_dn4 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn4)), ((locals.var_uc_depthn_dn5 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn5)), ((locals.var_uc_depthn_dn6 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn6)), ((locals.var_uc_depthn_dn7 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn7)), ((locals.var_uc_depthn_dn8 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn8)), ((locals.var_uc_depthn_dn9 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn9)), ((locals.var_uc_depthn_dn10 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn10)), ((locals.var_uc_depthn_dn11 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn11)), ((locals.var_uc_depthn_dn14 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn14)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn11, locals.var_w_subl_dn14,)
    }
};
        locals.var_w_subl = assign31400_e32485;
        locals.var_w_subl_dn0 = assign31400_e32485_d_n0;
        locals.var_w_subl_dn2 = assign31400_e32485_d_n2;
        locals.var_w_subl_dn4 = assign31400_e32485_d_n4;
        locals.var_w_subl_dn5 = assign31400_e32485_d_n5;
        locals.var_w_subl_dn6 = assign31400_e32485_d_n6;
        locals.var_w_subl_dn7 = assign31400_e32485_d_n7;
        locals.var_w_subl_dn8 = assign31400_e32485_d_n8;
        locals.var_w_subl_dn9 = assign31400_e32485_d_n9;
        locals.var_w_subl_dn10 = assign31400_e32485_d_n10;
        locals.var_w_subl_dn11 = assign31400_e32485_d_n11;
        locals.var_w_subl_dn14 = assign31400_e32485_d_n14;

        let (assign31410_e32504, assign31410_e32504_d_n0, assign31410_e32504_d_n2, assign31410_e32504_d_n4, assign31410_e32504_d_n5, assign31410_e32504_d_n6, assign31410_e32504_d_n7, assign31410_e32504_d_n8, assign31410_e32504_d_n9, assign31410_e32504_d_n10, assign31410_e32504_d_n11, assign31410_e32504_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31410_e32496: f64 = (locals.var_c_2esipq_nsub_inv * locals.var_w_subl);
        let assign31410_e32498: f64 = (assign31410_e32496 * locals.var_w_subl);
        let assign31410_e32500: f64 = (assign31410_e32498 + locals.var_vbscl__blk437);
        let assign31410_e32502: f64 = (assign31410_e32500 - locals.var_vbi_dep);
        (assign31410_e32502, ((((((locals.var_c_2esipq_nsub_inv_dn0 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn0)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn0)) + locals.var_vbscl__blk437_dn0) - locals.var_vbi_dep_dn0), ((((((locals.var_c_2esipq_nsub_inv_dn2 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn2)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn2)) + locals.var_vbscl__blk437_dn2) - locals.var_vbi_dep_dn2), ((((((locals.var_c_2esipq_nsub_inv_dn4 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn4)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn4)) + locals.var_vbscl__blk437_dn4) - locals.var_vbi_dep_dn4), ((((((locals.var_c_2esipq_nsub_inv_dn5 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn5)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn5)) + locals.var_vbscl__blk437_dn5) - locals.var_vbi_dep_dn5), ((((((locals.var_c_2esipq_nsub_inv_dn6 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn6)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn6)) + locals.var_vbscl__blk437_dn6) - locals.var_vbi_dep_dn6), ((((((locals.var_c_2esipq_nsub_inv_dn7 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn7)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn7)) + locals.var_vbscl__blk437_dn7) - locals.var_vbi_dep_dn7), ((((((locals.var_c_2esipq_nsub_inv_dn8 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn8)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn8)) + locals.var_vbscl__blk437_dn8) - locals.var_vbi_dep_dn8), ((((((locals.var_c_2esipq_nsub_inv_dn9 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn9)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn9)) + locals.var_vbscl__blk437_dn9) - locals.var_vbi_dep_dn9), ((((((locals.var_c_2esipq_nsub_inv_dn10 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn10)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn10)) + locals.var_vbscl__blk437_dn10) - locals.var_vbi_dep_dn10), ((((((locals.var_c_2esipq_nsub_inv_dn11 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn11)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn11)) + locals.var_vbscl__blk437_dn11) - locals.var_vbi_dep_dn11), ((((((locals.var_c_2esipq_nsub_inv_dn14 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn14)) * locals.var_w_subl) + (assign31410_e32496 * locals.var_w_subl_dn14)) + locals.var_vbscl__blk437_dn14) - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31410_e32504;
        locals.var_phi_jl_dep_dn0 = assign31410_e32504_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31410_e32504_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31410_e32504_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31410_e32504_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31410_e32504_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31410_e32504_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31410_e32504_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31410_e32504_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31410_e32504_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31410_e32504_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31410_e32504_d_n14;

        let (assign31420_e32519, assign31420_e32519_d_n0, assign31420_e32519_d_n2, assign31420_e32519_d_n4, assign31420_e32519_d_n5, assign31420_e32519_d_n6, assign31420_e32519_d_n7, assign31420_e32519_d_n8, assign31420_e32519_d_n9, assign31420_e32519_d_n10, assign31420_e32519_d_n11, assign31420_e32519_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign31420_e32516: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_tn2);
        let assign31420_e32517: f64 = (locals.var_phi_jl_dep + assign31420_e32516);
        (assign31420_e32517, (locals.var_phi_jl_dep_dn0 + ((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn0))), (locals.var_phi_jl_dep_dn2 + ((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn2))), (locals.var_phi_jl_dep_dn4 + ((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn4))), (locals.var_phi_jl_dep_dn5 + ((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn5))), (locals.var_phi_jl_dep_dn6 + ((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn6))), (locals.var_phi_jl_dep_dn7 + ((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn7))), (locals.var_phi_jl_dep_dn8 + ((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn8))), (locals.var_phi_jl_dep_dn9 + ((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn9))), (locals.var_phi_jl_dep_dn10 + ((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn10))), (locals.var_phi_jl_dep_dn11 + ((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn11))), (locals.var_phi_jl_dep_dn14 + ((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn14))),)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31420_e32519;
        locals.var_phi_bl_dep_dn0 = assign31420_e32519_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31420_e32519_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31420_e32519_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31420_e32519_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31420_e32519_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31420_e32519_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31420_e32519_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31420_e32519_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31420_e32519_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31420_e32519_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31420_e32519_d_n14;

        let (assign31430_e32530, assign31430_e32530_d_n0, assign31430_e32530_d_n2, assign31430_e32530_d_n4, assign31430_e32530_d_n5, assign31430_e32530_d_n6, assign31430_e32530_d_n7, assign31430_e32530_d_n8, assign31430_e32530_d_n9, assign31430_e32530_d_n10, assign31430_e32530_d_n11, assign31430_e32530_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    }
};
        locals.var_phi_sl_dep = assign31430_e32530;
        locals.var_phi_sl_dep_dn0 = assign31430_e32530_d_n0;
        locals.var_phi_sl_dep_dn2 = assign31430_e32530_d_n2;
        locals.var_phi_sl_dep_dn4 = assign31430_e32530_d_n4;
        locals.var_phi_sl_dep_dn5 = assign31430_e32530_d_n5;
        locals.var_phi_sl_dep_dn6 = assign31430_e32530_d_n6;
        locals.var_phi_sl_dep_dn7 = assign31430_e32530_d_n7;
        locals.var_phi_sl_dep_dn8 = assign31430_e32530_d_n8;
        locals.var_phi_sl_dep_dn9 = assign31430_e32530_d_n9;
        locals.var_phi_sl_dep_dn10 = assign31430_e32530_d_n10;
        locals.var_phi_sl_dep_dn11 = assign31430_e32530_d_n11;
        locals.var_phi_sl_dep_dn14 = assign31430_e32530_d_n14;

        let (assign31440_e32541, assign31440_e32541_d_n0, assign31440_e32541_d_n2, assign31440_e32541_d_n4, assign31440_e32541_d_n5, assign31440_e32541_d_n6, assign31440_e32541_d_n7, assign31440_e32541_d_n8, assign31440_e32541_d_n9, assign31440_e32541_d_n10, assign31440_e32541_d_n11, assign31440_e32541_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    }
};
        locals.var_psbmax = assign31440_e32541;
        locals.var_psbmax_dn0 = assign31440_e32541_d_n0;
        locals.var_psbmax_dn2 = assign31440_e32541_d_n2;
        locals.var_psbmax_dn4 = assign31440_e32541_d_n4;
        locals.var_psbmax_dn5 = assign31440_e32541_d_n5;
        locals.var_psbmax_dn6 = assign31440_e32541_d_n6;
        locals.var_psbmax_dn7 = assign31440_e32541_d_n7;
        locals.var_psbmax_dn8 = assign31440_e32541_d_n8;
        locals.var_psbmax_dn9 = assign31440_e32541_d_n9;
        locals.var_psbmax_dn10 = assign31440_e32541_d_n10;
        locals.var_psbmax_dn11 = assign31440_e32541_d_n11;
        locals.var_psbmax_dn14 = assign31440_e32541_d_n14;

        let (assign31450_e32552,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        (locals.var_phi_bl_dep,)
    } else {
        (locals.var_vgp1,)
    }
};
        locals.var_vgp1 = assign31450_e32552;

        let assign31460_e32555: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard729 = assign31460_e32555;

        let (assign31470_e32568,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard729 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign31470_e32568;

        let assign31480_e32571: f64 = if locals.var_vgp > locals.var_vgp1 { 1.0 } else { 0.0 };
        locals.var_guard730 = assign31480_e32571;

        let (assign31490_e32587,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard729 == 0.0)) && (locals.var_guard730 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign31490_e32587;

        let (assign31500_e32604,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard729 == 0.0)) && (locals.var_guard730 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign31500_e32604;

        let (assign31510_e32616, assign31510_e32616_d_n0, assign31510_e32616_d_n2, assign31510_e32616_d_n4, assign31510_e32616_d_n5, assign31510_e32616_d_n6, assign31510_e32616_d_n7, assign31510_e32616_d_n8, assign31510_e32616_d_n9, assign31510_e32616_d_n10, assign31510_e32616_d_n11, assign31510_e32616_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    }
};
        locals.var_vgp0 = assign31510_e32616;
        locals.var_vgp0_dn0 = assign31510_e32616_d_n0;
        locals.var_vgp0_dn2 = assign31510_e32616_d_n2;
        locals.var_vgp0_dn4 = assign31510_e32616_d_n4;
        locals.var_vgp0_dn5 = assign31510_e32616_d_n5;
        locals.var_vgp0_dn6 = assign31510_e32616_d_n6;
        locals.var_vgp0_dn7 = assign31510_e32616_d_n7;
        locals.var_vgp0_dn8 = assign31510_e32616_d_n8;
        locals.var_vgp0_dn9 = assign31510_e32616_d_n9;
        locals.var_vgp0_dn10 = assign31510_e32616_d_n10;
        locals.var_vgp0_dn11 = assign31510_e32616_d_n11;
        locals.var_vgp0_dn14 = assign31510_e32616_d_n14;

        let (assign31520_e32628,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        (locals.var_vgp0,)
    } else {
        (locals.var_vgp1,)
    }
};
        locals.var_vgp1 = assign31520_e32628;

        let (assign31530_e32640, assign31530_e32640_d_n0, assign31530_e32640_d_n2, assign31530_e32640_d_n4, assign31530_e32640_d_n5, assign31530_e32640_d_n6, assign31530_e32640_d_n7, assign31530_e32640_d_n8, assign31530_e32640_d_n9, assign31530_e32640_d_n10, assign31530_e32640_d_n11, assign31530_e32640_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    }
};
        locals.var_psbmax = assign31530_e32640;
        locals.var_psbmax_dn0 = assign31530_e32640_d_n0;
        locals.var_psbmax_dn2 = assign31530_e32640_d_n2;
        locals.var_psbmax_dn4 = assign31530_e32640_d_n4;
        locals.var_psbmax_dn5 = assign31530_e32640_d_n5;
        locals.var_psbmax_dn6 = assign31530_e32640_d_n6;
        locals.var_psbmax_dn7 = assign31530_e32640_d_n7;
        locals.var_psbmax_dn8 = assign31530_e32640_d_n8;
        locals.var_psbmax_dn9 = assign31530_e32640_d_n9;
        locals.var_psbmax_dn10 = assign31530_e32640_d_n10;
        locals.var_psbmax_dn11 = assign31530_e32640_d_n11;
        locals.var_psbmax_dn14 = assign31530_e32640_d_n14;

        let (assign31540_e32652, assign31540_e32652_d_n0, assign31540_e32652_d_n2, assign31540_e32652_d_n4, assign31540_e32652_d_n5, assign31540_e32652_d_n6, assign31540_e32652_d_n7, assign31540_e32652_d_n8, assign31540_e32652_d_n9, assign31540_e32652_d_n10, assign31540_e32652_d_n11, assign31540_e32652_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn11, locals.var_vgp0_dn14,)
    } else {
        (locals.var_vds_maxbl, locals.var_vds_maxbl_dn0, locals.var_vds_maxbl_dn2, locals.var_vds_maxbl_dn4, locals.var_vds_maxbl_dn5, locals.var_vds_maxbl_dn6, locals.var_vds_maxbl_dn7, locals.var_vds_maxbl_dn8, locals.var_vds_maxbl_dn9, locals.var_vds_maxbl_dn10, locals.var_vds_maxbl_dn11, locals.var_vds_maxbl_dn14,)
    }
};
        locals.var_vds_maxbl = assign31540_e32652;
        locals.var_vds_maxbl_dn0 = assign31540_e32652_d_n0;
        locals.var_vds_maxbl_dn2 = assign31540_e32652_d_n2;
        locals.var_vds_maxbl_dn4 = assign31540_e32652_d_n4;
        locals.var_vds_maxbl_dn5 = assign31540_e32652_d_n5;
        locals.var_vds_maxbl_dn6 = assign31540_e32652_d_n6;
        locals.var_vds_maxbl_dn7 = assign31540_e32652_d_n7;
        locals.var_vds_maxbl_dn8 = assign31540_e32652_d_n8;
        locals.var_vds_maxbl_dn9 = assign31540_e32652_d_n9;
        locals.var_vds_maxbl_dn10 = assign31540_e32652_d_n10;
        locals.var_vds_maxbl_dn11 = assign31540_e32652_d_n11;
        locals.var_vds_maxbl_dn14 = assign31540_e32652_d_n14;

        let (assign31550_e32664, assign31550_e32664_d_n0, assign31550_e32664_d_n2, assign31550_e32664_d_n4, assign31550_e32664_d_n5, assign31550_e32664_d_n6, assign31550_e32664_d_n7, assign31550_e32664_d_n8, assign31550_e32664_d_n9, assign31550_e32664_d_n10, assign31550_e32664_d_n11, assign31550_e32664_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        (locals.var_w_bsubl, locals.var_w_bsubl_dn0, locals.var_w_bsubl_dn2, locals.var_w_bsubl_dn4, locals.var_w_bsubl_dn5, locals.var_w_bsubl_dn6, locals.var_w_bsubl_dn7, locals.var_w_bsubl_dn8, locals.var_w_bsubl_dn9, locals.var_w_bsubl_dn10, locals.var_w_bsubl_dn11, locals.var_w_bsubl_dn14,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign31550_e32664;
        locals.var_w_bl_dn0 = assign31550_e32664_d_n0;
        locals.var_w_bl_dn2 = assign31550_e32664_d_n2;
        locals.var_w_bl_dn4 = assign31550_e32664_d_n4;
        locals.var_w_bl_dn5 = assign31550_e32664_d_n5;
        locals.var_w_bl_dn6 = assign31550_e32664_d_n6;
        locals.var_w_bl_dn7 = assign31550_e32664_d_n7;
        locals.var_w_bl_dn8 = assign31550_e32664_d_n8;
        locals.var_w_bl_dn9 = assign31550_e32664_d_n9;
        locals.var_w_bl_dn10 = assign31550_e32664_d_n10;
        locals.var_w_bl_dn11 = assign31550_e32664_d_n11;
        locals.var_w_bl_dn14 = assign31550_e32664_d_n14;

        let (assign31560_e32678, assign31560_e32678_d_n0, assign31560_e32678_d_n2, assign31560_e32678_d_n4, assign31560_e32678_d_n5, assign31560_e32678_d_n6, assign31560_e32678_d_n7, assign31560_e32678_d_n8, assign31560_e32678_d_n9, assign31560_e32678_d_n10, assign31560_e32678_d_n11, assign31560_e32678_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign31560_e32676: f64 = (locals.var_w_bl * locals.var_ndepmpnsub);
        (assign31560_e32676, ((locals.var_w_bl_dn0 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn0)), ((locals.var_w_bl_dn2 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn2)), ((locals.var_w_bl_dn4 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn4)), ((locals.var_w_bl_dn5 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn5)), ((locals.var_w_bl_dn6 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn6)), ((locals.var_w_bl_dn7 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn7)), ((locals.var_w_bl_dn8 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn8)), ((locals.var_w_bl_dn9 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn9)), ((locals.var_w_bl_dn10 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn10)), ((locals.var_w_bl_dn11 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn11)), ((locals.var_w_bl_dn14 * locals.var_ndepmpnsub) + (locals.var_w_bl * locals.var_ndepmpnsub_dn14)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn11, locals.var_w_subl_dn14,)
    }
};
        locals.var_w_subl = assign31560_e32678;
        locals.var_w_subl_dn0 = assign31560_e32678_d_n0;
        locals.var_w_subl_dn2 = assign31560_e32678_d_n2;
        locals.var_w_subl_dn4 = assign31560_e32678_d_n4;
        locals.var_w_subl_dn5 = assign31560_e32678_d_n5;
        locals.var_w_subl_dn6 = assign31560_e32678_d_n6;
        locals.var_w_subl_dn7 = assign31560_e32678_d_n7;
        locals.var_w_subl_dn8 = assign31560_e32678_d_n8;
        locals.var_w_subl_dn9 = assign31560_e32678_d_n9;
        locals.var_w_subl_dn10 = assign31560_e32678_d_n10;
        locals.var_w_subl_dn11 = assign31560_e32678_d_n11;
        locals.var_w_subl_dn14 = assign31560_e32678_d_n14;

        let (assign31570_e32698, assign31570_e32698_d_n0, assign31570_e32698_d_n2, assign31570_e32698_d_n4, assign31570_e32698_d_n5, assign31570_e32698_d_n6, assign31570_e32698_d_n7, assign31570_e32698_d_n8, assign31570_e32698_d_n9, assign31570_e32698_d_n10, assign31570_e32698_d_n11, assign31570_e32698_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign31570_e32690: f64 = (locals.var_c_2esipq_nsub_inv * locals.var_w_subl);
        let assign31570_e32692: f64 = (assign31570_e32690 * locals.var_w_subl);
        let assign31570_e32694: f64 = (assign31570_e32692 + locals.var_vbscl__blk437);
        let assign31570_e32696: f64 = (assign31570_e32694 - locals.var_vbi_dep);
        (assign31570_e32696, ((((((locals.var_c_2esipq_nsub_inv_dn0 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn0)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn0)) + locals.var_vbscl__blk437_dn0) - locals.var_vbi_dep_dn0), ((((((locals.var_c_2esipq_nsub_inv_dn2 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn2)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn2)) + locals.var_vbscl__blk437_dn2) - locals.var_vbi_dep_dn2), ((((((locals.var_c_2esipq_nsub_inv_dn4 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn4)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn4)) + locals.var_vbscl__blk437_dn4) - locals.var_vbi_dep_dn4), ((((((locals.var_c_2esipq_nsub_inv_dn5 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn5)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn5)) + locals.var_vbscl__blk437_dn5) - locals.var_vbi_dep_dn5), ((((((locals.var_c_2esipq_nsub_inv_dn6 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn6)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn6)) + locals.var_vbscl__blk437_dn6) - locals.var_vbi_dep_dn6), ((((((locals.var_c_2esipq_nsub_inv_dn7 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn7)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn7)) + locals.var_vbscl__blk437_dn7) - locals.var_vbi_dep_dn7), ((((((locals.var_c_2esipq_nsub_inv_dn8 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn8)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn8)) + locals.var_vbscl__blk437_dn8) - locals.var_vbi_dep_dn8), ((((((locals.var_c_2esipq_nsub_inv_dn9 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn9)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn9)) + locals.var_vbscl__blk437_dn9) - locals.var_vbi_dep_dn9), ((((((locals.var_c_2esipq_nsub_inv_dn10 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn10)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn10)) + locals.var_vbscl__blk437_dn10) - locals.var_vbi_dep_dn10), ((((((locals.var_c_2esipq_nsub_inv_dn11 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn11)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn11)) + locals.var_vbscl__blk437_dn11) - locals.var_vbi_dep_dn11), ((((((locals.var_c_2esipq_nsub_inv_dn14 * locals.var_w_subl) + (locals.var_c_2esipq_nsub_inv * locals.var_w_subl_dn14)) * locals.var_w_subl) + (assign31570_e32690 * locals.var_w_subl_dn14)) + locals.var_vbscl__blk437_dn14) - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31570_e32698;
        locals.var_phi_jl_dep_dn0 = assign31570_e32698_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31570_e32698_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31570_e32698_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31570_e32698_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31570_e32698_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31570_e32698_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31570_e32698_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31570_e32698_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31570_e32698_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31570_e32698_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31570_e32698_d_n14;

        let (assign31580_e32716, assign31580_e32716_d_n0, assign31580_e32716_d_n2, assign31580_e32716_d_n4, assign31580_e32716_d_n5, assign31580_e32716_d_n6, assign31580_e32716_d_n7, assign31580_e32716_d_n8, assign31580_e32716_d_n9, assign31580_e32716_d_n10, assign31580_e32716_d_n11, assign31580_e32716_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign31580_e32710: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl);
        let assign31580_e32712: f64 = (assign31580_e32710 * locals.var_w_bl);
        let assign31580_e32714: f64 = (assign31580_e32712 + locals.var_phi_jl_dep);
        (assign31580_e32714, (((((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn0)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn0)) + locals.var_phi_jl_dep_dn0), (((((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn2)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn2)) + locals.var_phi_jl_dep_dn2), (((((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn4)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn4)) + locals.var_phi_jl_dep_dn4), (((((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn5)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn5)) + locals.var_phi_jl_dep_dn5), (((((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn6)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn6)) + locals.var_phi_jl_dep_dn6), (((((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn7)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn7)) + locals.var_phi_jl_dep_dn7), (((((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn8)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn8)) + locals.var_phi_jl_dep_dn8), (((((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn9)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn9)) + locals.var_phi_jl_dep_dn9), (((((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn10)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn10)) + locals.var_phi_jl_dep_dn10), (((((locals.var_c_2esipq_ndepm_inv_dn11 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn11)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn11)) + locals.var_phi_jl_dep_dn11), (((((locals.var_c_2esipq_ndepm_inv_dn14 * locals.var_w_bl) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_bl_dn14)) * locals.var_w_bl) + (assign31580_e32710 * locals.var_w_bl_dn14)) + locals.var_phi_jl_dep_dn14),)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31580_e32716;
        locals.var_phi_bl_dep_dn0 = assign31580_e32716_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31580_e32716_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31580_e32716_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31580_e32716_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31580_e32716_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31580_e32716_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31580_e32716_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31580_e32716_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31580_e32716_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31580_e32716_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31580_e32716_d_n14;

        let (assign31590_e32728, assign31590_e32728_d_n0, assign31590_e32728_d_n2, assign31590_e32728_d_n4, assign31590_e32728_d_n5, assign31590_e32728_d_n6, assign31590_e32728_d_n7, assign31590_e32728_d_n8, assign31590_e32728_d_n9, assign31590_e32728_d_n10, assign31590_e32728_d_n11, assign31590_e32728_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    } else {
        (locals.var_phi_jl_dep_acc, locals.var_phi_jl_dep_acc_dn0, locals.var_phi_jl_dep_acc_dn2, locals.var_phi_jl_dep_acc_dn4, locals.var_phi_jl_dep_acc_dn5, locals.var_phi_jl_dep_acc_dn6, locals.var_phi_jl_dep_acc_dn7, locals.var_phi_jl_dep_acc_dn8, locals.var_phi_jl_dep_acc_dn9, locals.var_phi_jl_dep_acc_dn10, locals.var_phi_jl_dep_acc_dn11, locals.var_phi_jl_dep_acc_dn14,)
    }
};
        locals.var_phi_jl_dep_acc = assign31590_e32728;
        locals.var_phi_jl_dep_acc_dn0 = assign31590_e32728_d_n0;
        locals.var_phi_jl_dep_acc_dn2 = assign31590_e32728_d_n2;
        locals.var_phi_jl_dep_acc_dn4 = assign31590_e32728_d_n4;
        locals.var_phi_jl_dep_acc_dn5 = assign31590_e32728_d_n5;
        locals.var_phi_jl_dep_acc_dn6 = assign31590_e32728_d_n6;
        locals.var_phi_jl_dep_acc_dn7 = assign31590_e32728_d_n7;
        locals.var_phi_jl_dep_acc_dn8 = assign31590_e32728_d_n8;
        locals.var_phi_jl_dep_acc_dn9 = assign31590_e32728_d_n9;
        locals.var_phi_jl_dep_acc_dn10 = assign31590_e32728_d_n10;
        locals.var_phi_jl_dep_acc_dn11 = assign31590_e32728_d_n11;
        locals.var_phi_jl_dep_acc_dn14 = assign31590_e32728_d_n14;

        let assign31600_e32731: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard731 = assign31600_e32731;

        let (assign31610_e32745,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard731 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign31610_e32745;

        let (assign31620_e32760,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard731 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign31620_e32760;

        let (assign31630_e32776, assign31630_e32776_d_n0, assign31630_e32776_d_n2, assign31630_e32776_d_n4, assign31630_e32776_d_n5, assign31630_e32776_d_n6, assign31630_e32776_d_n7, assign31630_e32776_d_n8, assign31630_e32776_d_n9, assign31630_e32776_d_n10, assign31630_e32776_d_n11, assign31630_e32776_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) {
        let assign31630_e32770: f64 = (-locals.var_pb2n);
        let assign31630_e32772: f64 = (assign31630_e32770 + locals.var_vbscl__blk437);
        let assign31630_e32773: f64 = (locals.var_psbmax - assign31630_e32772);
        let assign31630_e32774: f64 = (locals.var_c_2esi_q_ndepm * assign31630_e32773);
        (assign31630_e32774, ((locals.var_c_2esi_q_ndepm_dn0 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn0 - ((-locals.var_pb2n_dn0) + locals.var_vbscl__blk437_dn0)))), ((locals.var_c_2esi_q_ndepm_dn2 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn2 - ((-locals.var_pb2n_dn2) + locals.var_vbscl__blk437_dn2)))), ((locals.var_c_2esi_q_ndepm_dn4 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn4 - ((-locals.var_pb2n_dn4) + locals.var_vbscl__blk437_dn4)))), ((locals.var_c_2esi_q_ndepm_dn5 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn5 - ((-locals.var_pb2n_dn5) + locals.var_vbscl__blk437_dn5)))), ((locals.var_c_2esi_q_ndepm_dn6 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn6 - ((-locals.var_pb2n_dn6) + locals.var_vbscl__blk437_dn6)))), ((locals.var_c_2esi_q_ndepm_dn7 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn7 - ((-locals.var_pb2n_dn7) + locals.var_vbscl__blk437_dn7)))), ((locals.var_c_2esi_q_ndepm_dn8 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn8 - ((-locals.var_pb2n_dn8) + locals.var_vbscl__blk437_dn8)))), ((locals.var_c_2esi_q_ndepm_dn9 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn9 - ((-locals.var_pb2n_dn9) + locals.var_vbscl__blk437_dn9)))), ((locals.var_c_2esi_q_ndepm_dn10 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn10 - ((-locals.var_pb2n_dn10) + locals.var_vbscl__blk437_dn10)))), ((locals.var_c_2esi_q_ndepm_dn11 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn11 - ((-locals.var_pb2n_dn11) + locals.var_vbscl__blk437_dn11)))), ((locals.var_c_2esi_q_ndepm_dn14 * assign31630_e32773) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn14 - ((-locals.var_pb2n_dn14) + locals.var_vbscl__blk437_dn14)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31630_e32776;
        locals.var_t1_dn0 = assign31630_e32776_d_n0;
        locals.var_t1_dn2 = assign31630_e32776_d_n2;
        locals.var_t1_dn4 = assign31630_e32776_d_n4;
        locals.var_t1_dn5 = assign31630_e32776_d_n5;
        locals.var_t1_dn6 = assign31630_e32776_d_n6;
        locals.var_t1_dn7 = assign31630_e32776_d_n7;
        locals.var_t1_dn8 = assign31630_e32776_d_n8;
        locals.var_t1_dn9 = assign31630_e32776_d_n9;
        locals.var_t1_dn10 = assign31630_e32776_d_n10;
        locals.var_t1_dn11 = assign31630_e32776_d_n11;
        locals.var_t1_dn14 = assign31630_e32776_d_n14;

        let assign31640_e32779: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard732 = assign31640_e32779;

        let (assign31650_e32798,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard732 != 0.0)) {
        let assign31650_e32789: f64 = (-locals.var_pb2n);
        let assign31650_e32791: f64 = (assign31650_e32789 + locals.var_vbscl__blk437);
        let assign31650_e32793: f64 = (locals.var_t1).sqrt();
        let assign31650_e32795: f64 = (assign31650_e32793 / locals.var_cox);
        let assign31650_e32796: f64 = (assign31650_e32791 - assign31650_e32795);
        (assign31650_e32796,)
    } else {
        (locals.var_vthn,)
    }
};
        locals.var_vthn = assign31650_e32798;

        let (assign31660_e32813,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard732 == 0.0)) {
        let assign31660_e32809: f64 = (-locals.var_pb2n);
        let assign31660_e32811: f64 = (assign31660_e32809 + locals.var_vbscl__blk437);
        (assign31660_e32811,)
    } else {
        (locals.var_vthn,)
    }
};
        locals.var_vthn = assign31660_e32813;

        let assign31670_e32816: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard733 = assign31670_e32816;

        let (assign31680_e32827, assign31680_e32827_d_n0, assign31680_e32827_d_n2, assign31680_e32827_d_n4, assign31680_e32827_d_n5, assign31680_e32827_d_n6, assign31680_e32827_d_n7, assign31680_e32827_d_n8, assign31680_e32827_d_n9, assign31680_e32827_d_n10, assign31680_e32827_d_n11, assign31680_e32827_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 != 0.0)) {
        (locals.var_phi_jl_dep_acc, locals.var_phi_jl_dep_acc_dn0, locals.var_phi_jl_dep_acc_dn2, locals.var_phi_jl_dep_acc_dn4, locals.var_phi_jl_dep_acc_dn5, locals.var_phi_jl_dep_acc_dn6, locals.var_phi_jl_dep_acc_dn7, locals.var_phi_jl_dep_acc_dn8, locals.var_phi_jl_dep_acc_dn9, locals.var_phi_jl_dep_acc_dn10, locals.var_phi_jl_dep_acc_dn11, locals.var_phi_jl_dep_acc_dn14,)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31680_e32827;
        locals.var_phi_jl_dep_dn0 = assign31680_e32827_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31680_e32827_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31680_e32827_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31680_e32827_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31680_e32827_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31680_e32827_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31680_e32827_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31680_e32827_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31680_e32827_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31680_e32827_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31680_e32827_d_n14;

        let (assign31690_e32838, assign31690_e32838_d_n0, assign31690_e32838_d_n2, assign31690_e32838_d_n4, assign31690_e32838_d_n5, assign31690_e32838_d_n6, assign31690_e32838_d_n7, assign31690_e32838_d_n8, assign31690_e32838_d_n9, assign31690_e32838_d_n10, assign31690_e32838_d_n11, assign31690_e32838_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31690_e32838;
        locals.var_phi_bl_dep_dn0 = assign31690_e32838_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31690_e32838_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31690_e32838_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31690_e32838_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31690_e32838_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31690_e32838_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31690_e32838_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31690_e32838_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31690_e32838_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31690_e32838_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31690_e32838_d_n14;

        let (assign31700_e32862, assign31700_e32862_d_n0, assign31700_e32862_d_n2, assign31700_e32862_d_n4, assign31700_e32862_d_n5, assign31700_e32862_d_n6, assign31700_e32862_d_n7, assign31700_e32862_d_n8, assign31700_e32862_d_n9, assign31700_e32862_d_n10, assign31700_e32862_d_n11, assign31700_e32862_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 != 0.0)) {
        let assign31700_e32849: f64 = (locals.var_afact * locals.var_vgp);
        let assign31700_e32851: f64 = (assign31700_e32849 * locals.var_vgp);
        let assign31700_e32852: f64 = (assign31700_e32851).ln();
        let assign31700_e32856: f64 = (2.0 / locals.var_vgp);
        let assign31700_e32857: f64 = (locals.var_beta + assign31700_e32856);
        let assign31700_e32858: f64 = (assign31700_e32852 / assign31700_e32857);
        let assign31700_e32860: f64 = (assign31700_e32858 + locals.var_vds);
        (assign31700_e32860, (((((((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn0)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn0), (((((((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn2)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn2), (((((((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn4)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp_dn4) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn4), (((((((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn5)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp_dn5) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn5), (((((((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn6)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn6), (((((((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn7)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn7), (((((((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn8)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp_dn8) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn8), (((((((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn9)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp_dn9) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn9), (((((((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn10)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn10), (((((((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn11)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn11 + (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn11), (((((((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign31700_e32849 * locals.var_vgp_dn14)) / assign31700_e32851) * assign31700_e32857) - (assign31700_e32852 * (locals.var_beta_dn14 + (-((2.0 * locals.var_vgp_dn14) / (locals.var_vgp * locals.var_vgp)))))) / (assign31700_e32857 * assign31700_e32857)) + locals.var_vds_dn14),)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31700_e32862;
        locals.var_phi_sl_dep_ini_dn0 = assign31700_e32862_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31700_e32862_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31700_e32862_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31700_e32862_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31700_e32862_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31700_e32862_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31700_e32862_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31700_e32862_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31700_e32862_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31700_e32862_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31700_e32862_d_n14;

        let assign31710_e32866: f64 = (locals.var_vds_maxbl + locals.var_ps_conv23);
        let assign31710_e32867: f64 = if locals.var_phi_sl_dep_ini < assign31710_e32866 { 1.0 } else { 0.0 };
        locals.var_guard734 = assign31710_e32867;

    }

    pub(super) fn stamp_transient_block_94(
        locals: &mut StampLocals,
    ) {
        let (assign31720_e32882, assign31720_e32882_d_n0, assign31720_e32882_d_n2, assign31720_e32882_d_n4, assign31720_e32882_d_n5, assign31720_e32882_d_n6, assign31720_e32882_d_n7, assign31720_e32882_d_n8, assign31720_e32882_d_n9, assign31720_e32882_d_n10, assign31720_e32882_d_n11, assign31720_e32882_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 != 0.0)) && (locals.var_guard734 != 0.0)) {
        let assign31720_e32880: f64 = (locals.var_vds_maxbl + locals.var_ps_conv23);
        (assign31720_e32880, locals.var_vds_maxbl_dn0, locals.var_vds_maxbl_dn2, locals.var_vds_maxbl_dn4, locals.var_vds_maxbl_dn5, locals.var_vds_maxbl_dn6, locals.var_vds_maxbl_dn7, locals.var_vds_maxbl_dn8, locals.var_vds_maxbl_dn9, locals.var_vds_maxbl_dn10, locals.var_vds_maxbl_dn11, locals.var_vds_maxbl_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31720_e32882;
        locals.var_phi_sl_dep_ini_dn0 = assign31720_e32882_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31720_e32882_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31720_e32882_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31720_e32882_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31720_e32882_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31720_e32882_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31720_e32882_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31720_e32882_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31720_e32882_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31720_e32882_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31720_e32882_d_n14;

        let assign31730_e32885: f64 = if locals.var_vgp > locals.var_vgp1 { 1.0 } else { 0.0 };
        locals.var_guard735 = assign31730_e32885;

        let (assign31740_e32899, assign31740_e32899_d_n0, assign31740_e32899_d_n2, assign31740_e32899_d_n4, assign31740_e32899_d_n5, assign31740_e32899_d_n6, assign31740_e32899_d_n7, assign31740_e32899_d_n8, assign31740_e32899_d_n9, assign31740_e32899_d_n10, assign31740_e32899_d_n11, assign31740_e32899_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 != 0.0)) {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31740_e32899;
        locals.var_phi_sl_dep_ini_dn0 = assign31740_e32899_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31740_e32899_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31740_e32899_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31740_e32899_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31740_e32899_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31740_e32899_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31740_e32899_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31740_e32899_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31740_e32899_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31740_e32899_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31740_e32899_d_n14;

        let assign31750_e32902: f64 = if locals.var_vgp > locals.var_vthn { 1.0 } else { 0.0 };
        locals.var_guard736 = assign31750_e32902;

        let (assign31760_e32926, assign31760_e32926_d_n0, assign31760_e32926_d_n2, assign31760_e32926_d_n4, assign31760_e32926_d_n5, assign31760_e32926_d_n6, assign31760_e32926_d_n7, assign31760_e32926_d_n8, assign31760_e32926_d_n9, assign31760_e32926_d_n10, assign31760_e32926_d_n11, assign31760_e32926_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign31760_e32918: f64 = (-2.0);
        let assign31760_e32920: f64 = (assign31760_e32918 * locals.var_afact);
        let assign31760_e32922: f64 = (assign31760_e32920 * locals.var_vgp);
        let assign31760_e32924: f64 = (assign31760_e32922 + locals.var_beta);
        (assign31760_e32924, ((((assign31760_e32918 * locals.var_afact_dn0) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn0)) + locals.var_beta_dn0), ((((assign31760_e32918 * locals.var_afact_dn2) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn2)) + locals.var_beta_dn2), ((((assign31760_e32918 * locals.var_afact_dn4) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn4)) + locals.var_beta_dn4), ((((assign31760_e32918 * locals.var_afact_dn5) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn5)) + locals.var_beta_dn5), ((((assign31760_e32918 * locals.var_afact_dn6) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn6)) + locals.var_beta_dn6), ((((assign31760_e32918 * locals.var_afact_dn7) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn7)) + locals.var_beta_dn7), ((((assign31760_e32918 * locals.var_afact_dn8) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn8)) + locals.var_beta_dn8), ((((assign31760_e32918 * locals.var_afact_dn9) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn9)) + locals.var_beta_dn9), ((((assign31760_e32918 * locals.var_afact_dn10) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn10)) + locals.var_beta_dn10), ((((assign31760_e32918 * locals.var_afact_dn11) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn11)) + locals.var_beta_dn11), ((((assign31760_e32918 * locals.var_afact_dn14) * locals.var_vgp) + (assign31760_e32920 * locals.var_vgp_dn14)) + locals.var_beta_dn14),)
    } else {
        (locals.var_bfact, locals.var_bfact_dn0, locals.var_bfact_dn2, locals.var_bfact_dn4, locals.var_bfact_dn5, locals.var_bfact_dn6, locals.var_bfact_dn7, locals.var_bfact_dn8, locals.var_bfact_dn9, locals.var_bfact_dn10, locals.var_bfact_dn11, locals.var_bfact_dn14,)
    }
};
        locals.var_bfact = assign31760_e32926;
        locals.var_bfact_dn0 = assign31760_e32926_d_n0;
        locals.var_bfact_dn2 = assign31760_e32926_d_n2;
        locals.var_bfact_dn4 = assign31760_e32926_d_n4;
        locals.var_bfact_dn5 = assign31760_e32926_d_n5;
        locals.var_bfact_dn6 = assign31760_e32926_d_n6;
        locals.var_bfact_dn7 = assign31760_e32926_d_n7;
        locals.var_bfact_dn8 = assign31760_e32926_d_n8;
        locals.var_bfact_dn9 = assign31760_e32926_d_n9;
        locals.var_bfact_dn10 = assign31760_e32926_d_n10;
        locals.var_bfact_dn11 = assign31760_e32926_d_n11;
        locals.var_bfact_dn14 = assign31760_e32926_d_n14;

        let (assign31770_e32951, assign31770_e32951_d_n0, assign31770_e32951_d_n2, assign31770_e32951_d_n4, assign31770_e32951_d_n5, assign31770_e32951_d_n6, assign31770_e32951_d_n7, assign31770_e32951_d_n8, assign31770_e32951_d_n9, assign31770_e32951_d_n10, assign31770_e32951_d_n11, assign31770_e32951_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign31770_e32943: f64 = (locals.var_afact * locals.var_vgp);
        let assign31770_e32945: f64 = (assign31770_e32943 * locals.var_vgp);
        let assign31770_e32948: f64 = (locals.var_beta * locals.var_phi_bl_dep);
        let assign31770_e32949: f64 = (assign31770_e32945 - assign31770_e32948);
        (assign31770_e32949, (((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn0)) - ((locals.var_beta_dn0 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn0))), (((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn2)) - ((locals.var_beta_dn2 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn2))), (((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn4)) - ((locals.var_beta_dn4 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn4))), (((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn5)) - ((locals.var_beta_dn5 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn5))), (((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn6)) - ((locals.var_beta_dn6 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn6))), (((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn7)) - ((locals.var_beta_dn7 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn7))), (((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn8)) - ((locals.var_beta_dn8 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn8))), (((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn9)) - ((locals.var_beta_dn9 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn9))), (((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn10)) - ((locals.var_beta_dn10 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn10))), (((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn11)) - ((locals.var_beta_dn11 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn11))), (((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign31770_e32943 * locals.var_vgp_dn14)) - ((locals.var_beta_dn14 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn14))),)
    } else {
        (locals.var_cfact, locals.var_cfact_dn0, locals.var_cfact_dn2, locals.var_cfact_dn4, locals.var_cfact_dn5, locals.var_cfact_dn6, locals.var_cfact_dn7, locals.var_cfact_dn8, locals.var_cfact_dn9, locals.var_cfact_dn10, locals.var_cfact_dn11, locals.var_cfact_dn14,)
    }
};
        locals.var_cfact = assign31770_e32951;
        locals.var_cfact_dn0 = assign31770_e32951_d_n0;
        locals.var_cfact_dn2 = assign31770_e32951_d_n2;
        locals.var_cfact_dn4 = assign31770_e32951_d_n4;
        locals.var_cfact_dn5 = assign31770_e32951_d_n5;
        locals.var_cfact_dn6 = assign31770_e32951_d_n6;
        locals.var_cfact_dn7 = assign31770_e32951_d_n7;
        locals.var_cfact_dn8 = assign31770_e32951_d_n8;
        locals.var_cfact_dn9 = assign31770_e32951_d_n9;
        locals.var_cfact_dn10 = assign31770_e32951_d_n10;
        locals.var_cfact_dn11 = assign31770_e32951_d_n11;
        locals.var_cfact_dn14 = assign31770_e32951_d_n14;

        let (assign31780_e32968,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) {
        (locals.var_phi_bl_dep,)
    } else {
        (locals.var_phi_bl_dep_old,)
    }
};
        locals.var_phi_bl_dep_old = assign31780_e32968;

        let (assign31790_e33001, assign31790_e33001_d_n0, assign31790_e33001_d_n2, assign31790_e33001_d_n4, assign31790_e33001_d_n5, assign31790_e33001_d_n6, assign31790_e33001_d_n7, assign31790_e33001_d_n8, assign31790_e33001_d_n9, assign31790_e33001_d_n10, assign31790_e33001_d_n11, assign31790_e33001_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign31790_e32984: f64 = (-locals.var_bfact);
        let assign31790_e32987: f64 = (locals.var_bfact * locals.var_bfact);
        let assign31790_e32990: f64 = (4.0 * locals.var_afact);
        let assign31790_e32992: f64 = (assign31790_e32990 * locals.var_cfact);
        let assign31790_e32993: f64 = (assign31790_e32987 - assign31790_e32992);
        let assign31790_e32994: f64 = (assign31790_e32993).sqrt();
        let assign31790_e32995: f64 = (assign31790_e32984 + assign31790_e32994);
        let assign31790_e32997: f64 = (assign31790_e32995 / 2.0);
        let assign31790_e32999: f64 = (assign31790_e32997 / locals.var_afact);
        (assign31790_e32999, ((((((-locals.var_bfact_dn0) + ((((locals.var_bfact_dn0 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn0)) - (((4.0 * locals.var_afact_dn0) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn0))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn2) + ((((locals.var_bfact_dn2 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn2)) - (((4.0 * locals.var_afact_dn2) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn2))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn4) + ((((locals.var_bfact_dn4 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn4)) - (((4.0 * locals.var_afact_dn4) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn4))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn5) + ((((locals.var_bfact_dn5 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn5)) - (((4.0 * locals.var_afact_dn5) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn5))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn6) + ((((locals.var_bfact_dn6 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn6)) - (((4.0 * locals.var_afact_dn6) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn6))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn7) + ((((locals.var_bfact_dn7 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn7)) - (((4.0 * locals.var_afact_dn7) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn7))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn8) + ((((locals.var_bfact_dn8 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn8)) - (((4.0 * locals.var_afact_dn8) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn8))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn9) + ((((locals.var_bfact_dn9 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn9)) - (((4.0 * locals.var_afact_dn9) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn9))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn10) + ((((locals.var_bfact_dn10 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn10)) - (((4.0 * locals.var_afact_dn10) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn10))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn11) + ((((locals.var_bfact_dn11 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn11)) - (((4.0 * locals.var_afact_dn11) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn11))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn11)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn14) + ((((locals.var_bfact_dn14 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn14)) - (((4.0 * locals.var_afact_dn14) * locals.var_cfact) + (assign31790_e32990 * locals.var_cfact_dn14))) / (2.0 * assign31790_e32994))) / 2.0) * locals.var_afact) - (assign31790_e32997 * locals.var_afact_dn14)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31790_e33001;
        locals.var_phi_sl_dep_ini_dn0 = assign31790_e33001_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31790_e33001_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31790_e33001_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31790_e33001_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31790_e33001_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31790_e33001_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31790_e33001_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31790_e33001_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31790_e33001_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31790_e33001_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31790_e33001_d_n14;

        let assign31800_e33005: f64 = (locals.var_psbmax - locals.var_ps_conv23);
        let assign31800_e33006: f64 = if locals.var_phi_sl_dep_ini > assign31800_e33005 { 1.0 } else { 0.0 };
        locals.var_guard737 = assign31800_e33006;

        let (assign31810_e33027, assign31810_e33027_d_n0, assign31810_e33027_d_n2, assign31810_e33027_d_n4, assign31810_e33027_d_n5, assign31810_e33027_d_n6, assign31810_e33027_d_n7, assign31810_e33027_d_n8, assign31810_e33027_d_n9, assign31810_e33027_d_n10, assign31810_e33027_d_n11, assign31810_e33027_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 != 0.0)) {
        let assign31810_e33025: f64 = (locals.var_psbmax - locals.var_ps_conv23);
        (assign31810_e33025, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31810_e33027;
        locals.var_phi_sl_dep_ini_dn0 = assign31810_e33027_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31810_e33027_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31810_e33027_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31810_e33027_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31810_e33027_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31810_e33027_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31810_e33027_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31810_e33027_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31810_e33027_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31810_e33027_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31810_e33027_d_n14;

        let (assign31820_e33049, assign31820_e33049_d_n0, assign31820_e33049_d_n2, assign31820_e33049_d_n4, assign31820_e33049_d_n5, assign31820_e33049_d_n6, assign31820_e33049_d_n7, assign31820_e33049_d_n8, assign31820_e33049_d_n9, assign31820_e33049_d_n10, assign31820_e33049_d_n11, assign31820_e33049_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign31820_e33045: f64 = (locals.var_phi_bl_dep - locals.var_phi_sl_dep_ini);
        let assign31820_e33046: f64 = (locals.var_c_2esipq_ndepm * assign31820_e33045);
        let assign31820_e33047: f64 = (assign31820_e33046).sqrt();
        (assign31820_e33047, (((locals.var_c_2esipq_ndepm_dn0 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_sl_dep_ini_dn0))) / (2.0 * assign31820_e33047)), (((locals.var_c_2esipq_ndepm_dn2 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_sl_dep_ini_dn2))) / (2.0 * assign31820_e33047)), (((locals.var_c_2esipq_ndepm_dn4 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_sl_dep_ini_dn4))) / (2.0 * assign31820_e33047)), (((locals.var_c_2esipq_ndepm_dn5 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_sl_dep_ini_dn5))) / (2.0 * assign31820_e33047)), (((locals.var_c_2esipq_ndepm_dn6 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_sl_dep_ini_dn6))) / (2.0 * assign31820_e33047)), (((locals.var_c_2esipq_ndepm_dn7 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_sl_dep_ini_dn7))) / (2.0 * assign31820_e33047)), (((locals.var_c_2esipq_ndepm_dn8 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_sl_dep_ini_dn8))) / (2.0 * assign31820_e33047)), (((locals.var_c_2esipq_ndepm_dn9 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_sl_dep_ini_dn9))) / (2.0 * assign31820_e33047)), (((locals.var_c_2esipq_ndepm_dn10 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_sl_dep_ini_dn10))) / (2.0 * assign31820_e33047)), (((locals.var_c_2esipq_ndepm_dn11 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_sl_dep_ini_dn11))) / (2.0 * assign31820_e33047)), (((locals.var_c_2esipq_ndepm_dn14 * assign31820_e33045) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_sl_dep_ini_dn14))) / (2.0 * assign31820_e33047)),)
    } else {
        (locals.var_w_sl, locals.var_w_sl_dn0, locals.var_w_sl_dn2, locals.var_w_sl_dn4, locals.var_w_sl_dn5, locals.var_w_sl_dn6, locals.var_w_sl_dn7, locals.var_w_sl_dn8, locals.var_w_sl_dn9, locals.var_w_sl_dn10, locals.var_w_sl_dn11, locals.var_w_sl_dn14,)
    }
};
        locals.var_w_sl = assign31820_e33049;
        locals.var_w_sl_dn0 = assign31820_e33049_d_n0;
        locals.var_w_sl_dn2 = assign31820_e33049_d_n2;
        locals.var_w_sl_dn4 = assign31820_e33049_d_n4;
        locals.var_w_sl_dn5 = assign31820_e33049_d_n5;
        locals.var_w_sl_dn6 = assign31820_e33049_d_n6;
        locals.var_w_sl_dn7 = assign31820_e33049_d_n7;
        locals.var_w_sl_dn8 = assign31820_e33049_d_n8;
        locals.var_w_sl_dn9 = assign31820_e33049_d_n9;
        locals.var_w_sl_dn10 = assign31820_e33049_d_n10;
        locals.var_w_sl_dn11 = assign31820_e33049_d_n11;
        locals.var_w_sl_dn14 = assign31820_e33049_d_n14;

        let (assign31830_e33071, assign31830_e33071_d_n0, assign31830_e33071_d_n2, assign31830_e33071_d_n4, assign31830_e33071_d_n5, assign31830_e33071_d_n6, assign31830_e33071_d_n7, assign31830_e33071_d_n8, assign31830_e33071_d_n9, assign31830_e33071_d_n10, assign31830_e33071_d_n11, assign31830_e33071_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign31830_e33067: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        let assign31830_e33068: f64 = (locals.var_c_2esipq_ndepm * assign31830_e33067);
        let assign31830_e33069: f64 = (assign31830_e33068).sqrt();
        (assign31830_e33069, (((locals.var_c_2esipq_ndepm_dn0 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0))) / (2.0 * assign31830_e33069)), (((locals.var_c_2esipq_ndepm_dn2 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2))) / (2.0 * assign31830_e33069)), (((locals.var_c_2esipq_ndepm_dn4 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4))) / (2.0 * assign31830_e33069)), (((locals.var_c_2esipq_ndepm_dn5 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5))) / (2.0 * assign31830_e33069)), (((locals.var_c_2esipq_ndepm_dn6 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6))) / (2.0 * assign31830_e33069)), (((locals.var_c_2esipq_ndepm_dn7 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7))) / (2.0 * assign31830_e33069)), (((locals.var_c_2esipq_ndepm_dn8 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8))) / (2.0 * assign31830_e33069)), (((locals.var_c_2esipq_ndepm_dn9 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9))) / (2.0 * assign31830_e33069)), (((locals.var_c_2esipq_ndepm_dn10 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10))) / (2.0 * assign31830_e33069)), (((locals.var_c_2esipq_ndepm_dn11 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_jl_dep_dn11))) / (2.0 * assign31830_e33069)), (((locals.var_c_2esipq_ndepm_dn14 * assign31830_e33067) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_jl_dep_dn14))) / (2.0 * assign31830_e33069)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign31830_e33071;
        locals.var_w_bl_dn0 = assign31830_e33071_d_n0;
        locals.var_w_bl_dn2 = assign31830_e33071_d_n2;
        locals.var_w_bl_dn4 = assign31830_e33071_d_n4;
        locals.var_w_bl_dn5 = assign31830_e33071_d_n5;
        locals.var_w_bl_dn6 = assign31830_e33071_d_n6;
        locals.var_w_bl_dn7 = assign31830_e33071_d_n7;
        locals.var_w_bl_dn8 = assign31830_e33071_d_n8;
        locals.var_w_bl_dn9 = assign31830_e33071_d_n9;
        locals.var_w_bl_dn10 = assign31830_e33071_d_n10;
        locals.var_w_bl_dn11 = assign31830_e33071_d_n11;
        locals.var_w_bl_dn14 = assign31830_e33071_d_n14;

        let assign31840_e33074: f64 = (locals.var_w_sl + locals.var_w_bl);
        let assign31840_e33076: f64 = if assign31840_e33074 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard738 = assign31840_e33076;

        let (assign31850_e33095,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign31850_e33095;

    }

    pub(super) fn stamp_transient_block_95(
        locals: &mut StampLocals,
    ) {
        let mut assign31860_loop_guard: usize = 0;
        while {
            let assign31860_cond_e33115: f64 = (150.0 + 1.0);
            let assign31860_cond_e33117: f64 = if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_lp_s0 <= assign31860_cond_e33115)) { 1.0 } else { 0.0 };
            assign31860_cond_e33117 != 0.0
        } {
            assign31860_loop_guard += 1;
            assert!(assign31860_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign31860_body0_e33140, assign31860_body0_e33140_d_n0, assign31860_body0_e33140_d_n2, assign31860_body0_e33140_d_n4, assign31860_body0_e33140_d_n5, assign31860_body0_e33140_d_n6, assign31860_body0_e33140_d_n7, assign31860_body0_e33140_d_n8, assign31860_body0_e33140_d_n9, assign31860_body0_e33140_d_n10, assign31860_body0_e33140_d_n11, assign31860_body0_e33140_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31860_body0_e33136: f64 = (locals.var_w_sl + locals.var_w_bl);
        let assign31860_body0_e33138: f64 = (assign31860_body0_e33136 - locals.var_uc_depthn);
        (assign31860_body0_e33138, ((locals.var_w_sl_dn0 + locals.var_w_bl_dn0) - locals.var_uc_depthn_dn0), ((locals.var_w_sl_dn2 + locals.var_w_bl_dn2) - locals.var_uc_depthn_dn2), ((locals.var_w_sl_dn4 + locals.var_w_bl_dn4) - locals.var_uc_depthn_dn4), ((locals.var_w_sl_dn5 + locals.var_w_bl_dn5) - locals.var_uc_depthn_dn5), ((locals.var_w_sl_dn6 + locals.var_w_bl_dn6) - locals.var_uc_depthn_dn6), ((locals.var_w_sl_dn7 + locals.var_w_bl_dn7) - locals.var_uc_depthn_dn7), ((locals.var_w_sl_dn8 + locals.var_w_bl_dn8) - locals.var_uc_depthn_dn8), ((locals.var_w_sl_dn9 + locals.var_w_bl_dn9) - locals.var_uc_depthn_dn9), ((locals.var_w_sl_dn10 + locals.var_w_bl_dn10) - locals.var_uc_depthn_dn10), ((locals.var_w_sl_dn11 + locals.var_w_bl_dn11) - locals.var_uc_depthn_dn11), ((locals.var_w_sl_dn14 + locals.var_w_bl_dn14) - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_y0, locals.var_y0_dn0, locals.var_y0_dn2, locals.var_y0_dn4, locals.var_y0_dn5, locals.var_y0_dn6, locals.var_y0_dn7, locals.var_y0_dn8, locals.var_y0_dn9, locals.var_y0_dn10, locals.var_y0_dn11, locals.var_y0_dn14,)
    }
};
            locals.var_y0 = assign31860_body0_e33140;
            locals.var_y0_dn0 = assign31860_body0_e33140_d_n0;
            locals.var_y0_dn2 = assign31860_body0_e33140_d_n2;
            locals.var_y0_dn4 = assign31860_body0_e33140_d_n4;
            locals.var_y0_dn5 = assign31860_body0_e33140_d_n5;
            locals.var_y0_dn6 = assign31860_body0_e33140_d_n6;
            locals.var_y0_dn7 = assign31860_body0_e33140_d_n7;
            locals.var_y0_dn8 = assign31860_body0_e33140_d_n8;
            locals.var_y0_dn9 = assign31860_body0_e33140_d_n9;
            locals.var_y0_dn10 = assign31860_body0_e33140_d_n10;
            locals.var_y0_dn11 = assign31860_body0_e33140_d_n11;
            locals.var_y0_dn14 = assign31860_body0_e33140_d_n14;
            let (assign31860_body1_e33177, assign31860_body1_e33177_d_n0, assign31860_body1_e33177_d_n2, assign31860_body1_e33177_d_n4, assign31860_body1_e33177_d_n5, assign31860_body1_e33177_d_n6, assign31860_body1_e33177_d_n7, assign31860_body1_e33177_d_n8, assign31860_body1_e33177_d_n9, assign31860_body1_e33177_d_n10, assign31860_body1_e33177_d_n11, assign31860_body1_e33177_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31860_body1_e33159: f64 = (1.034943e-10 / locals.var_q_ndepm);
        let assign31860_body1_e33161: f64 = (assign31860_body1_e33159 / locals.var_w_sl);
        let assign31860_body1_e33164: f64 = (1.034943e-10 / locals.var_q_ndepm);
        let assign31860_body1_e33169: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign31860_body1_e33170: f64 = (locals.var_ndepmpnsub / assign31860_body1_e33169);
        let assign31860_body1_e33171: f64 = (1.0 - assign31860_body1_e33170);
        let assign31860_body1_e33172: f64 = (assign31860_body1_e33164 * assign31860_body1_e33171);
        let assign31860_body1_e33174: f64 = (assign31860_body1_e33172 / locals.var_w_bl);
        let assign31860_body1_e33175: f64 = (assign31860_body1_e33161 + assign31860_body1_e33174);
        (assign31860_body1_e33175, (((((-((1.034943e-10 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn0)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn0 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn0)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn0)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn2)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn2 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn2)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn2)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn4)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn4 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn4)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn4)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn5)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn5 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn5)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn5)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn6)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn6 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn6)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn6)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn7)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn7 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn7)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn7)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn8)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn8 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn8)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn8)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn9)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn9 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn9)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn9)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn10)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn10 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn10)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn10)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn11) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn11)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn11) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn11 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn11)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn11)) / (locals.var_w_bl * locals.var_w_bl))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn14) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_sl) - (assign31860_body1_e33159 * locals.var_w_sl_dn14)) / (locals.var_w_sl * locals.var_w_sl)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn14) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign31860_body1_e33171) + (assign31860_body1_e33164 * (-(((locals.var_ndepmpnsub_dn14 * assign31860_body1_e33169) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn14)) / (assign31860_body1_e33169 * assign31860_body1_e33169))))) * locals.var_w_bl) - (assign31860_body1_e33172 * locals.var_w_bl_dn14)) / (locals.var_w_bl * locals.var_w_bl))),)
    } else {
        (locals.var_dydpsm, locals.var_dydpsm_dn0, locals.var_dydpsm_dn2, locals.var_dydpsm_dn4, locals.var_dydpsm_dn5, locals.var_dydpsm_dn6, locals.var_dydpsm_dn7, locals.var_dydpsm_dn8, locals.var_dydpsm_dn9, locals.var_dydpsm_dn10, locals.var_dydpsm_dn11, locals.var_dydpsm_dn14,)
    }
};
            locals.var_dydpsm = assign31860_body1_e33177;
            locals.var_dydpsm_dn0 = assign31860_body1_e33177_d_n0;
            locals.var_dydpsm_dn2 = assign31860_body1_e33177_d_n2;
            locals.var_dydpsm_dn4 = assign31860_body1_e33177_d_n4;
            locals.var_dydpsm_dn5 = assign31860_body1_e33177_d_n5;
            locals.var_dydpsm_dn6 = assign31860_body1_e33177_d_n6;
            locals.var_dydpsm_dn7 = assign31860_body1_e33177_d_n7;
            locals.var_dydpsm_dn8 = assign31860_body1_e33177_d_n8;
            locals.var_dydpsm_dn9 = assign31860_body1_e33177_d_n9;
            locals.var_dydpsm_dn10 = assign31860_body1_e33177_d_n10;
            locals.var_dydpsm_dn11 = assign31860_body1_e33177_d_n11;
            locals.var_dydpsm_dn14 = assign31860_body1_e33177_d_n14;
            let assign31860_body2_e33180: f64 = (locals.var_y0 / locals.var_dydpsm);
            let assign31860_body2_e33181: f64 = (assign31860_body2_e33180).abs();
            let assign31860_body2_e33183: f64 = if assign31860_body2_e33181 > 0.5 { 1.0 } else { 0.0 };
            locals.var_guard739 = assign31860_body2_e33183;
            let (assign31860_body3_e33216, assign31860_body3_e33216_d_n0, assign31860_body3_e33216_d_n2, assign31860_body3_e33216_d_n4, assign31860_body3_e33216_d_n5, assign31860_body3_e33216_d_n6, assign31860_body3_e33216_d_n7, assign31860_body3_e33216_d_n8, assign31860_body3_e33216_d_n9, assign31860_body3_e33216_d_n10, assign31860_body3_e33216_d_n11, assign31860_body3_e33216_d_n14,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard739 != 0.0)) {
        let assign31860_body3_e33206: f64 = (locals.var_y0 / locals.var_dydpsm);
        let (assign31860_body3_e33212,) = {
            if (assign31860_body3_e33206 >= 0.0) {
                (1.0,)
            } else {
                let assign31860_body3_e33211: f64 = (-1.0);
                (assign31860_body3_e33211,)
            }
        };
        let assign31860_body3_e33213: f64 = (0.5 * assign31860_body3_e33212);
        let assign31860_body3_e33214: f64 = (locals.var_phi_bl_dep - assign31860_body3_e33213);
        (assign31860_body3_e33214, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
            locals.var_phi_bl_dep = assign31860_body3_e33216;
            locals.var_phi_bl_dep_dn0 = assign31860_body3_e33216_d_n0;
            locals.var_phi_bl_dep_dn2 = assign31860_body3_e33216_d_n2;
            locals.var_phi_bl_dep_dn4 = assign31860_body3_e33216_d_n4;
            locals.var_phi_bl_dep_dn5 = assign31860_body3_e33216_d_n5;
            locals.var_phi_bl_dep_dn6 = assign31860_body3_e33216_d_n6;
            locals.var_phi_bl_dep_dn7 = assign31860_body3_e33216_d_n7;
            locals.var_phi_bl_dep_dn8 = assign31860_body3_e33216_d_n8;
            locals.var_phi_bl_dep_dn9 = assign31860_body3_e33216_d_n9;
            locals.var_phi_bl_dep_dn10 = assign31860_body3_e33216_d_n10;
            locals.var_phi_bl_dep_dn11 = assign31860_body3_e33216_d_n11;
            locals.var_phi_bl_dep_dn14 = assign31860_body3_e33216_d_n14;
            let (assign31860_body4_e33242, assign31860_body4_e33242_d_n0, assign31860_body4_e33242_d_n2, assign31860_body4_e33242_d_n4, assign31860_body4_e33242_d_n5, assign31860_body4_e33242_d_n6, assign31860_body4_e33242_d_n7, assign31860_body4_e33242_d_n8, assign31860_body4_e33242_d_n9, assign31860_body4_e33242_d_n10, assign31860_body4_e33242_d_n11, assign31860_body4_e33242_d_n14,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard739 == 0.0)) {
        let assign31860_body4_e33239: f64 = (locals.var_y0 / locals.var_dydpsm);
        let assign31860_body4_e33240: f64 = (locals.var_phi_bl_dep - assign31860_body4_e33239);
        (assign31860_body4_e33240, (locals.var_phi_bl_dep_dn0 - (((locals.var_y0_dn0 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn0)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn2 - (((locals.var_y0_dn2 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn2)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn4 - (((locals.var_y0_dn4 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn4)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn5 - (((locals.var_y0_dn5 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn5)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn6 - (((locals.var_y0_dn6 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn6)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn7 - (((locals.var_y0_dn7 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn7)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn8 - (((locals.var_y0_dn8 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn8)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn9 - (((locals.var_y0_dn9 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn9)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn10 - (((locals.var_y0_dn10 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn10)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn11 - (((locals.var_y0_dn11 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn11)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_bl_dep_dn14 - (((locals.var_y0_dn14 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn14)) / (locals.var_dydpsm * locals.var_dydpsm))),)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
            locals.var_phi_bl_dep = assign31860_body4_e33242;
            locals.var_phi_bl_dep_dn0 = assign31860_body4_e33242_d_n0;
            locals.var_phi_bl_dep_dn2 = assign31860_body4_e33242_d_n2;
            locals.var_phi_bl_dep_dn4 = assign31860_body4_e33242_d_n4;
            locals.var_phi_bl_dep_dn5 = assign31860_body4_e33242_d_n5;
            locals.var_phi_bl_dep_dn6 = assign31860_body4_e33242_d_n6;
            locals.var_phi_bl_dep_dn7 = assign31860_body4_e33242_d_n7;
            locals.var_phi_bl_dep_dn8 = assign31860_body4_e33242_d_n8;
            locals.var_phi_bl_dep_dn9 = assign31860_body4_e33242_d_n9;
            locals.var_phi_bl_dep_dn10 = assign31860_body4_e33242_d_n10;
            locals.var_phi_bl_dep_dn11 = assign31860_body4_e33242_d_n11;
            locals.var_phi_bl_dep_dn14 = assign31860_body4_e33242_d_n14;
            let assign31860_body5_e33245: f64 = (locals.var_phi_bl_dep - locals.var_vbscl__blk437);
            let assign31860_body5_e33247: f64 = (assign31860_body5_e33245 + locals.var_vbi_dep);
            let assign31860_body5_e33250: f64 = (10.0 * 2.220446049250313e-16);
            let assign31860_body5_e33251: f64 = if assign31860_body5_e33247 < assign31860_body5_e33250 { 1.0 } else { 0.0 };
            locals.var_guard740 = assign31860_body5_e33251;
            let (assign31860_body6_e33278, assign31860_body6_e33278_d_n0, assign31860_body6_e33278_d_n2, assign31860_body6_e33278_d_n4, assign31860_body6_e33278_d_n5, assign31860_body6_e33278_d_n6, assign31860_body6_e33278_d_n7, assign31860_body6_e33278_d_n8, assign31860_body6_e33278_d_n9, assign31860_body6_e33278_d_n10, assign31860_body6_e33278_d_n11, assign31860_body6_e33278_d_n14,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign31860_body6_e33272: f64 = (locals.var_vbscl__blk437 - locals.var_vbi_dep);
        let assign31860_body6_e33275: f64 = (10.0 * 2.220446049250313e-16);
        let assign31860_body6_e33276: f64 = (assign31860_body6_e33272 + assign31860_body6_e33275);
        (assign31860_body6_e33276, (locals.var_vbscl__blk437_dn0 - locals.var_vbi_dep_dn0), (locals.var_vbscl__blk437_dn2 - locals.var_vbi_dep_dn2), (locals.var_vbscl__blk437_dn4 - locals.var_vbi_dep_dn4), (locals.var_vbscl__blk437_dn5 - locals.var_vbi_dep_dn5), (locals.var_vbscl__blk437_dn6 - locals.var_vbi_dep_dn6), (locals.var_vbscl__blk437_dn7 - locals.var_vbi_dep_dn7), (locals.var_vbscl__blk437_dn8 - locals.var_vbi_dep_dn8), (locals.var_vbscl__blk437_dn9 - locals.var_vbi_dep_dn9), (locals.var_vbscl__blk437_dn10 - locals.var_vbi_dep_dn10), (locals.var_vbscl__blk437_dn11 - locals.var_vbi_dep_dn11), (locals.var_vbscl__blk437_dn14 - locals.var_vbi_dep_dn14),)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
            locals.var_phi_bl_dep = assign31860_body6_e33278;
            locals.var_phi_bl_dep_dn0 = assign31860_body6_e33278_d_n0;
            locals.var_phi_bl_dep_dn2 = assign31860_body6_e33278_d_n2;
            locals.var_phi_bl_dep_dn4 = assign31860_body6_e33278_d_n4;
            locals.var_phi_bl_dep_dn5 = assign31860_body6_e33278_d_n5;
            locals.var_phi_bl_dep_dn6 = assign31860_body6_e33278_d_n6;
            locals.var_phi_bl_dep_dn7 = assign31860_body6_e33278_d_n7;
            locals.var_phi_bl_dep_dn8 = assign31860_body6_e33278_d_n8;
            locals.var_phi_bl_dep_dn9 = assign31860_body6_e33278_d_n9;
            locals.var_phi_bl_dep_dn10 = assign31860_body6_e33278_d_n10;
            locals.var_phi_bl_dep_dn11 = assign31860_body6_e33278_d_n11;
            locals.var_phi_bl_dep_dn14 = assign31860_body6_e33278_d_n14;
            let (assign31860_body7_e33305, assign31860_body7_e33305_d_n0, assign31860_body7_e33305_d_n2, assign31860_body7_e33305_d_n4, assign31860_body7_e33305_d_n5, assign31860_body7_e33305_d_n6, assign31860_body7_e33305_d_n7, assign31860_body7_e33305_d_n8, assign31860_body7_e33305_d_n9, assign31860_body7_e33305_d_n10, assign31860_body7_e33305_d_n11, assign31860_body7_e33305_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31860_body7_e33297: f64 = (locals.var_afact * locals.var_vgp);
        let assign31860_body7_e33299: f64 = (assign31860_body7_e33297 * locals.var_vgp);
        let assign31860_body7_e33302: f64 = (locals.var_beta * locals.var_phi_bl_dep);
        let assign31860_body7_e33303: f64 = (assign31860_body7_e33299 - assign31860_body7_e33302);
        (assign31860_body7_e33303, (((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn0)) - ((locals.var_beta_dn0 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn0))), (((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn2)) - ((locals.var_beta_dn2 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn2))), (((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn4)) - ((locals.var_beta_dn4 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn4))), (((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn5)) - ((locals.var_beta_dn5 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn5))), (((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn6)) - ((locals.var_beta_dn6 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn6))), (((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn7)) - ((locals.var_beta_dn7 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn7))), (((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn8)) - ((locals.var_beta_dn8 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn8))), (((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn9)) - ((locals.var_beta_dn9 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn9))), (((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn10)) - ((locals.var_beta_dn10 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn10))), (((((locals.var_afact_dn11 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn11)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn11)) - ((locals.var_beta_dn11 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn11))), (((((locals.var_afact_dn14 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn14)) * locals.var_vgp) + (assign31860_body7_e33297 * locals.var_vgp_dn14)) - ((locals.var_beta_dn14 * locals.var_phi_bl_dep) + (locals.var_beta * locals.var_phi_bl_dep_dn14))),)
    } else {
        (locals.var_cfact, locals.var_cfact_dn0, locals.var_cfact_dn2, locals.var_cfact_dn4, locals.var_cfact_dn5, locals.var_cfact_dn6, locals.var_cfact_dn7, locals.var_cfact_dn8, locals.var_cfact_dn9, locals.var_cfact_dn10, locals.var_cfact_dn11, locals.var_cfact_dn14,)
    }
};
            locals.var_cfact = assign31860_body7_e33305;
            locals.var_cfact_dn0 = assign31860_body7_e33305_d_n0;
            locals.var_cfact_dn2 = assign31860_body7_e33305_d_n2;
            locals.var_cfact_dn4 = assign31860_body7_e33305_d_n4;
            locals.var_cfact_dn5 = assign31860_body7_e33305_d_n5;
            locals.var_cfact_dn6 = assign31860_body7_e33305_d_n6;
            locals.var_cfact_dn7 = assign31860_body7_e33305_d_n7;
            locals.var_cfact_dn8 = assign31860_body7_e33305_d_n8;
            locals.var_cfact_dn9 = assign31860_body7_e33305_d_n9;
            locals.var_cfact_dn10 = assign31860_body7_e33305_d_n10;
            locals.var_cfact_dn11 = assign31860_body7_e33305_d_n11;
            locals.var_cfact_dn14 = assign31860_body7_e33305_d_n14;
            let (assign31860_body8_e33332, assign31860_body8_e33332_d_n0, assign31860_body8_e33332_d_n2, assign31860_body8_e33332_d_n4, assign31860_body8_e33332_d_n5, assign31860_body8_e33332_d_n6, assign31860_body8_e33332_d_n7, assign31860_body8_e33332_d_n8, assign31860_body8_e33332_d_n9, assign31860_body8_e33332_d_n10, assign31860_body8_e33332_d_n11, assign31860_body8_e33332_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31860_body8_e33324: f64 = (locals.var_bfact * locals.var_bfact);
        let assign31860_body8_e33327: f64 = (4.0 * locals.var_afact);
        let assign31860_body8_e33329: f64 = (assign31860_body8_e33327 * locals.var_cfact);
        let assign31860_body8_e33330: f64 = (assign31860_body8_e33324 - assign31860_body8_e33329);
        (assign31860_body8_e33330, (((locals.var_bfact_dn0 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn0)) - (((4.0 * locals.var_afact_dn0) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn0))), (((locals.var_bfact_dn2 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn2)) - (((4.0 * locals.var_afact_dn2) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn2))), (((locals.var_bfact_dn4 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn4)) - (((4.0 * locals.var_afact_dn4) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn4))), (((locals.var_bfact_dn5 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn5)) - (((4.0 * locals.var_afact_dn5) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn5))), (((locals.var_bfact_dn6 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn6)) - (((4.0 * locals.var_afact_dn6) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn6))), (((locals.var_bfact_dn7 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn7)) - (((4.0 * locals.var_afact_dn7) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn7))), (((locals.var_bfact_dn8 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn8)) - (((4.0 * locals.var_afact_dn8) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn8))), (((locals.var_bfact_dn9 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn9)) - (((4.0 * locals.var_afact_dn9) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn9))), (((locals.var_bfact_dn10 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn10)) - (((4.0 * locals.var_afact_dn10) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn10))), (((locals.var_bfact_dn11 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn11)) - (((4.0 * locals.var_afact_dn11) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn11))), (((locals.var_bfact_dn14 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn14)) - (((4.0 * locals.var_afact_dn14) * locals.var_cfact) + (assign31860_body8_e33327 * locals.var_cfact_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign31860_body8_e33332;
            locals.var_t1_dn0 = assign31860_body8_e33332_d_n0;
            locals.var_t1_dn2 = assign31860_body8_e33332_d_n2;
            locals.var_t1_dn4 = assign31860_body8_e33332_d_n4;
            locals.var_t1_dn5 = assign31860_body8_e33332_d_n5;
            locals.var_t1_dn6 = assign31860_body8_e33332_d_n6;
            locals.var_t1_dn7 = assign31860_body8_e33332_d_n7;
            locals.var_t1_dn8 = assign31860_body8_e33332_d_n8;
            locals.var_t1_dn9 = assign31860_body8_e33332_d_n9;
            locals.var_t1_dn10 = assign31860_body8_e33332_d_n10;
            locals.var_t1_dn11 = assign31860_body8_e33332_d_n11;
            locals.var_t1_dn14 = assign31860_body8_e33332_d_n14;
            let assign31860_body9_e33335: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard741 = assign31860_body9_e33335;
            let (assign31860_body10_e33364, assign31860_body10_e33364_d_n0, assign31860_body10_e33364_d_n2, assign31860_body10_e33364_d_n4, assign31860_body10_e33364_d_n5, assign31860_body10_e33364_d_n6, assign31860_body10_e33364_d_n7, assign31860_body10_e33364_d_n8, assign31860_body10_e33364_d_n9, assign31860_body10_e33364_d_n10, assign31860_body10_e33364_d_n11, assign31860_body10_e33364_d_n14,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard741 != 0.0)) {
        let assign31860_body10_e33355: f64 = (-locals.var_bfact);
        let assign31860_body10_e33357: f64 = (locals.var_t1).sqrt();
        let assign31860_body10_e33358: f64 = (assign31860_body10_e33355 + assign31860_body10_e33357);
        let assign31860_body10_e33360: f64 = (assign31860_body10_e33358 / 2.0);
        let assign31860_body10_e33362: f64 = (assign31860_body10_e33360 / locals.var_afact);
        (assign31860_body10_e33362, ((((((-locals.var_bfact_dn0) + (locals.var_t1_dn0 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn2) + (locals.var_t1_dn2 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn4) + (locals.var_t1_dn4 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn5) + (locals.var_t1_dn5 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn6) + (locals.var_t1_dn6 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn7) + (locals.var_t1_dn7 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn8) + (locals.var_t1_dn8 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn9) + (locals.var_t1_dn9 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn10) + (locals.var_t1_dn10 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn11) + (locals.var_t1_dn11 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn11)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn14) + (locals.var_t1_dn14 / (2.0 * assign31860_body10_e33357))) / 2.0) * locals.var_afact) - (assign31860_body10_e33360 * locals.var_afact_dn14)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
            locals.var_phi_sl_dep_ini = assign31860_body10_e33364;
            locals.var_phi_sl_dep_ini_dn0 = assign31860_body10_e33364_d_n0;
            locals.var_phi_sl_dep_ini_dn2 = assign31860_body10_e33364_d_n2;
            locals.var_phi_sl_dep_ini_dn4 = assign31860_body10_e33364_d_n4;
            locals.var_phi_sl_dep_ini_dn5 = assign31860_body10_e33364_d_n5;
            locals.var_phi_sl_dep_ini_dn6 = assign31860_body10_e33364_d_n6;
            locals.var_phi_sl_dep_ini_dn7 = assign31860_body10_e33364_d_n7;
            locals.var_phi_sl_dep_ini_dn8 = assign31860_body10_e33364_d_n8;
            locals.var_phi_sl_dep_ini_dn9 = assign31860_body10_e33364_d_n9;
            locals.var_phi_sl_dep_ini_dn10 = assign31860_body10_e33364_d_n10;
            locals.var_phi_sl_dep_ini_dn11 = assign31860_body10_e33364_d_n11;
            locals.var_phi_sl_dep_ini_dn14 = assign31860_body10_e33364_d_n14;
            let (assign31860_body11_e33391, assign31860_body11_e33391_d_n0, assign31860_body11_e33391_d_n2, assign31860_body11_e33391_d_n4, assign31860_body11_e33391_d_n5, assign31860_body11_e33391_d_n6, assign31860_body11_e33391_d_n7, assign31860_body11_e33391_d_n8, assign31860_body11_e33391_d_n9, assign31860_body11_e33391_d_n10, assign31860_body11_e33391_d_n11, assign31860_body11_e33391_d_n14,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard741 == 0.0)) {
        let assign31860_body11_e33385: f64 = (-locals.var_bfact);
        let assign31860_body11_e33387: f64 = (assign31860_body11_e33385 / 2.0);
        let assign31860_body11_e33389: f64 = (assign31860_body11_e33387 / locals.var_afact);
        (assign31860_body11_e33389, (((((-locals.var_bfact_dn0) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn2) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn4) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn5) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn6) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn7) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn8) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn9) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn10) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn11) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn11)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn14) / 2.0) * locals.var_afact) - (assign31860_body11_e33387 * locals.var_afact_dn14)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
            locals.var_phi_sl_dep_ini = assign31860_body11_e33391;
            locals.var_phi_sl_dep_ini_dn0 = assign31860_body11_e33391_d_n0;
            locals.var_phi_sl_dep_ini_dn2 = assign31860_body11_e33391_d_n2;
            locals.var_phi_sl_dep_ini_dn4 = assign31860_body11_e33391_d_n4;
            locals.var_phi_sl_dep_ini_dn5 = assign31860_body11_e33391_d_n5;
            locals.var_phi_sl_dep_ini_dn6 = assign31860_body11_e33391_d_n6;
            locals.var_phi_sl_dep_ini_dn7 = assign31860_body11_e33391_d_n7;
            locals.var_phi_sl_dep_ini_dn8 = assign31860_body11_e33391_d_n8;
            locals.var_phi_sl_dep_ini_dn9 = assign31860_body11_e33391_d_n9;
            locals.var_phi_sl_dep_ini_dn10 = assign31860_body11_e33391_d_n10;
            locals.var_phi_sl_dep_ini_dn11 = assign31860_body11_e33391_d_n11;
            locals.var_phi_sl_dep_ini_dn14 = assign31860_body11_e33391_d_n14;
            let assign31860_body12_e33394: f64 = if locals.var_phi_sl_dep_ini > locals.var_psbmax { 1.0 } else { 0.0 };
            locals.var_guard742 = assign31860_body12_e33394;
            let (assign31860_body13_e33415, assign31860_body13_e33415_d_n0, assign31860_body13_e33415_d_n2, assign31860_body13_e33415_d_n4, assign31860_body13_e33415_d_n5, assign31860_body13_e33415_d_n6, assign31860_body13_e33415_d_n7, assign31860_body13_e33415_d_n8, assign31860_body13_e33415_d_n9, assign31860_body13_e33415_d_n10, assign31860_body13_e33415_d_n11, assign31860_body13_e33415_d_n14,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard742 != 0.0)) {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn11, locals.var_psbmax_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
            locals.var_phi_sl_dep_ini = assign31860_body13_e33415;
            locals.var_phi_sl_dep_ini_dn0 = assign31860_body13_e33415_d_n0;
            locals.var_phi_sl_dep_ini_dn2 = assign31860_body13_e33415_d_n2;
            locals.var_phi_sl_dep_ini_dn4 = assign31860_body13_e33415_d_n4;
            locals.var_phi_sl_dep_ini_dn5 = assign31860_body13_e33415_d_n5;
            locals.var_phi_sl_dep_ini_dn6 = assign31860_body13_e33415_d_n6;
            locals.var_phi_sl_dep_ini_dn7 = assign31860_body13_e33415_d_n7;
            locals.var_phi_sl_dep_ini_dn8 = assign31860_body13_e33415_d_n8;
            locals.var_phi_sl_dep_ini_dn9 = assign31860_body13_e33415_d_n9;
            locals.var_phi_sl_dep_ini_dn10 = assign31860_body13_e33415_d_n10;
            locals.var_phi_sl_dep_ini_dn11 = assign31860_body13_e33415_d_n11;
            locals.var_phi_sl_dep_ini_dn14 = assign31860_body13_e33415_d_n14;
            let assign31860_body14_e33418: f64 = if locals.var_phi_sl_dep_ini > locals.var_phi_bl_dep { 1.0 } else { 0.0 };
            locals.var_guard743 = assign31860_body14_e33418;
            let (assign31860_body15_e33441, assign31860_body15_e33441_d_n0, assign31860_body15_e33441_d_n2, assign31860_body15_e33441_d_n4, assign31860_body15_e33441_d_n5, assign31860_body15_e33441_d_n6, assign31860_body15_e33441_d_n7, assign31860_body15_e33441_d_n8, assign31860_body15_e33441_d_n9, assign31860_body15_e33441_d_n10, assign31860_body15_e33441_d_n11, assign31860_body15_e33441_d_n14,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard743 != 0.0)) {
        let assign31860_body15_e33439: f64 = (locals.var_phi_bl_dep - locals.var_ps_conv23);
        (assign31860_body15_e33439, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
            locals.var_phi_sl_dep_ini = assign31860_body15_e33441;
            locals.var_phi_sl_dep_ini_dn0 = assign31860_body15_e33441_d_n0;
            locals.var_phi_sl_dep_ini_dn2 = assign31860_body15_e33441_d_n2;
            locals.var_phi_sl_dep_ini_dn4 = assign31860_body15_e33441_d_n4;
            locals.var_phi_sl_dep_ini_dn5 = assign31860_body15_e33441_d_n5;
            locals.var_phi_sl_dep_ini_dn6 = assign31860_body15_e33441_d_n6;
            locals.var_phi_sl_dep_ini_dn7 = assign31860_body15_e33441_d_n7;
            locals.var_phi_sl_dep_ini_dn8 = assign31860_body15_e33441_d_n8;
            locals.var_phi_sl_dep_ini_dn9 = assign31860_body15_e33441_d_n9;
            locals.var_phi_sl_dep_ini_dn10 = assign31860_body15_e33441_d_n10;
            locals.var_phi_sl_dep_ini_dn11 = assign31860_body15_e33441_d_n11;
            locals.var_phi_sl_dep_ini_dn14 = assign31860_body15_e33441_d_n14;
            let (assign31860_body16_e33464,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard743 != 0.0)) {
        let assign31860_body16_e33462: f64 = (150.0 + 1.0);
        (assign31860_body16_e33462,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31860_body16_e33464;
            let (assign31860_body17_e33488, assign31860_body17_e33488_d_n0, assign31860_body17_e33488_d_n2, assign31860_body17_e33488_d_n4, assign31860_body17_e33488_d_n5, assign31860_body17_e33488_d_n6, assign31860_body17_e33488_d_n7, assign31860_body17_e33488_d_n8, assign31860_body17_e33488_d_n9, assign31860_body17_e33488_d_n10, assign31860_body17_e33488_d_n11, assign31860_body17_e33488_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31860_body17_e33484: f64 = (locals.var_phi_bl_dep - locals.var_phi_sl_dep_ini);
        let assign31860_body17_e33485: f64 = (locals.var_c_2esipq_ndepm * assign31860_body17_e33484);
        let assign31860_body17_e33486: f64 = (assign31860_body17_e33485).sqrt();
        (assign31860_body17_e33486, (((locals.var_c_2esipq_ndepm_dn0 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_sl_dep_ini_dn0))) / (2.0 * assign31860_body17_e33486)), (((locals.var_c_2esipq_ndepm_dn2 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_sl_dep_ini_dn2))) / (2.0 * assign31860_body17_e33486)), (((locals.var_c_2esipq_ndepm_dn4 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_sl_dep_ini_dn4))) / (2.0 * assign31860_body17_e33486)), (((locals.var_c_2esipq_ndepm_dn5 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_sl_dep_ini_dn5))) / (2.0 * assign31860_body17_e33486)), (((locals.var_c_2esipq_ndepm_dn6 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_sl_dep_ini_dn6))) / (2.0 * assign31860_body17_e33486)), (((locals.var_c_2esipq_ndepm_dn7 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_sl_dep_ini_dn7))) / (2.0 * assign31860_body17_e33486)), (((locals.var_c_2esipq_ndepm_dn8 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_sl_dep_ini_dn8))) / (2.0 * assign31860_body17_e33486)), (((locals.var_c_2esipq_ndepm_dn9 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_sl_dep_ini_dn9))) / (2.0 * assign31860_body17_e33486)), (((locals.var_c_2esipq_ndepm_dn10 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_sl_dep_ini_dn10))) / (2.0 * assign31860_body17_e33486)), (((locals.var_c_2esipq_ndepm_dn11 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_sl_dep_ini_dn11))) / (2.0 * assign31860_body17_e33486)), (((locals.var_c_2esipq_ndepm_dn14 * assign31860_body17_e33484) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_sl_dep_ini_dn14))) / (2.0 * assign31860_body17_e33486)),)
    } else {
        (locals.var_w_sl, locals.var_w_sl_dn0, locals.var_w_sl_dn2, locals.var_w_sl_dn4, locals.var_w_sl_dn5, locals.var_w_sl_dn6, locals.var_w_sl_dn7, locals.var_w_sl_dn8, locals.var_w_sl_dn9, locals.var_w_sl_dn10, locals.var_w_sl_dn11, locals.var_w_sl_dn14,)
    }
};
            locals.var_w_sl = assign31860_body17_e33488;
            locals.var_w_sl_dn0 = assign31860_body17_e33488_d_n0;
            locals.var_w_sl_dn2 = assign31860_body17_e33488_d_n2;
            locals.var_w_sl_dn4 = assign31860_body17_e33488_d_n4;
            locals.var_w_sl_dn5 = assign31860_body17_e33488_d_n5;
            locals.var_w_sl_dn6 = assign31860_body17_e33488_d_n6;
            locals.var_w_sl_dn7 = assign31860_body17_e33488_d_n7;
            locals.var_w_sl_dn8 = assign31860_body17_e33488_d_n8;
            locals.var_w_sl_dn9 = assign31860_body17_e33488_d_n9;
            locals.var_w_sl_dn10 = assign31860_body17_e33488_d_n10;
            locals.var_w_sl_dn11 = assign31860_body17_e33488_d_n11;
            locals.var_w_sl_dn14 = assign31860_body17_e33488_d_n14;
            let (assign31860_body18_e33517, assign31860_body18_e33517_d_n0, assign31860_body18_e33517_d_n2, assign31860_body18_e33517_d_n4, assign31860_body18_e33517_d_n5, assign31860_body18_e33517_d_n6, assign31860_body18_e33517_d_n7, assign31860_body18_e33517_d_n8, assign31860_body18_e33517_d_n9, assign31860_body18_e33517_d_n10, assign31860_body18_e33517_d_n11, assign31860_body18_e33517_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31860_body18_e33507: f64 = (locals.var_ndepmpnsub * locals.var_phi_bl_dep);
        let assign31860_body18_e33509: f64 = (assign31860_body18_e33507 + locals.var_vbscl__blk437);
        let assign31860_body18_e33511: f64 = (assign31860_body18_e33509 - locals.var_vbi_dep);
        let assign31860_body18_e33514: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign31860_body18_e33515: f64 = (assign31860_body18_e33511 / assign31860_body18_e33514);
        (assign31860_body18_e33515, (((((((locals.var_ndepmpnsub_dn0 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn0)) + locals.var_vbscl__blk437_dn0) - locals.var_vbi_dep_dn0) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn0)) / (assign31860_body18_e33514 * assign31860_body18_e33514)), (((((((locals.var_ndepmpnsub_dn2 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn2)) + locals.var_vbscl__blk437_dn2) - locals.var_vbi_dep_dn2) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn2)) / (assign31860_body18_e33514 * assign31860_body18_e33514)), (((((((locals.var_ndepmpnsub_dn4 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn4)) + locals.var_vbscl__blk437_dn4) - locals.var_vbi_dep_dn4) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn4)) / (assign31860_body18_e33514 * assign31860_body18_e33514)), (((((((locals.var_ndepmpnsub_dn5 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn5)) + locals.var_vbscl__blk437_dn5) - locals.var_vbi_dep_dn5) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn5)) / (assign31860_body18_e33514 * assign31860_body18_e33514)), (((((((locals.var_ndepmpnsub_dn6 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn6)) + locals.var_vbscl__blk437_dn6) - locals.var_vbi_dep_dn6) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn6)) / (assign31860_body18_e33514 * assign31860_body18_e33514)), (((((((locals.var_ndepmpnsub_dn7 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn7)) + locals.var_vbscl__blk437_dn7) - locals.var_vbi_dep_dn7) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn7)) / (assign31860_body18_e33514 * assign31860_body18_e33514)), (((((((locals.var_ndepmpnsub_dn8 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn8)) + locals.var_vbscl__blk437_dn8) - locals.var_vbi_dep_dn8) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn8)) / (assign31860_body18_e33514 * assign31860_body18_e33514)), (((((((locals.var_ndepmpnsub_dn9 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn9)) + locals.var_vbscl__blk437_dn9) - locals.var_vbi_dep_dn9) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn9)) / (assign31860_body18_e33514 * assign31860_body18_e33514)), (((((((locals.var_ndepmpnsub_dn10 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn10)) + locals.var_vbscl__blk437_dn10) - locals.var_vbi_dep_dn10) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn10)) / (assign31860_body18_e33514 * assign31860_body18_e33514)), (((((((locals.var_ndepmpnsub_dn11 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn11)) + locals.var_vbscl__blk437_dn11) - locals.var_vbi_dep_dn11) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn11)) / (assign31860_body18_e33514 * assign31860_body18_e33514)), (((((((locals.var_ndepmpnsub_dn14 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn14)) + locals.var_vbscl__blk437_dn14) - locals.var_vbi_dep_dn14) * assign31860_body18_e33514) - (assign31860_body18_e33511 * locals.var_ndepmpnsub_dn14)) / (assign31860_body18_e33514 * assign31860_body18_e33514)),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
            locals.var_phi_jl_dep = assign31860_body18_e33517;
            locals.var_phi_jl_dep_dn0 = assign31860_body18_e33517_d_n0;
            locals.var_phi_jl_dep_dn2 = assign31860_body18_e33517_d_n2;
            locals.var_phi_jl_dep_dn4 = assign31860_body18_e33517_d_n4;
            locals.var_phi_jl_dep_dn5 = assign31860_body18_e33517_d_n5;
            locals.var_phi_jl_dep_dn6 = assign31860_body18_e33517_d_n6;
            locals.var_phi_jl_dep_dn7 = assign31860_body18_e33517_d_n7;
            locals.var_phi_jl_dep_dn8 = assign31860_body18_e33517_d_n8;
            locals.var_phi_jl_dep_dn9 = assign31860_body18_e33517_d_n9;
            locals.var_phi_jl_dep_dn10 = assign31860_body18_e33517_d_n10;
            locals.var_phi_jl_dep_dn11 = assign31860_body18_e33517_d_n11;
            locals.var_phi_jl_dep_dn14 = assign31860_body18_e33517_d_n14;
            let (assign31860_body19_e33541, assign31860_body19_e33541_d_n0, assign31860_body19_e33541_d_n2, assign31860_body19_e33541_d_n4, assign31860_body19_e33541_d_n5, assign31860_body19_e33541_d_n6, assign31860_body19_e33541_d_n7, assign31860_body19_e33541_d_n8, assign31860_body19_e33541_d_n9, assign31860_body19_e33541_d_n10, assign31860_body19_e33541_d_n11, assign31860_body19_e33541_d_n14,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31860_body19_e33537: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        let assign31860_body19_e33538: f64 = (locals.var_c_2esipq_ndepm * assign31860_body19_e33537);
        let assign31860_body19_e33539: f64 = (assign31860_body19_e33538).sqrt();
        (assign31860_body19_e33539, (((locals.var_c_2esipq_ndepm_dn0 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0))) / (2.0 * assign31860_body19_e33539)), (((locals.var_c_2esipq_ndepm_dn2 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2))) / (2.0 * assign31860_body19_e33539)), (((locals.var_c_2esipq_ndepm_dn4 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4))) / (2.0 * assign31860_body19_e33539)), (((locals.var_c_2esipq_ndepm_dn5 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5))) / (2.0 * assign31860_body19_e33539)), (((locals.var_c_2esipq_ndepm_dn6 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6))) / (2.0 * assign31860_body19_e33539)), (((locals.var_c_2esipq_ndepm_dn7 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7))) / (2.0 * assign31860_body19_e33539)), (((locals.var_c_2esipq_ndepm_dn8 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8))) / (2.0 * assign31860_body19_e33539)), (((locals.var_c_2esipq_ndepm_dn9 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9))) / (2.0 * assign31860_body19_e33539)), (((locals.var_c_2esipq_ndepm_dn10 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10))) / (2.0 * assign31860_body19_e33539)), (((locals.var_c_2esipq_ndepm_dn11 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_jl_dep_dn11))) / (2.0 * assign31860_body19_e33539)), (((locals.var_c_2esipq_ndepm_dn14 * assign31860_body19_e33537) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_jl_dep_dn14))) / (2.0 * assign31860_body19_e33539)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
            locals.var_w_bl = assign31860_body19_e33541;
            locals.var_w_bl_dn0 = assign31860_body19_e33541_d_n0;
            locals.var_w_bl_dn2 = assign31860_body19_e33541_d_n2;
            locals.var_w_bl_dn4 = assign31860_body19_e33541_d_n4;
            locals.var_w_bl_dn5 = assign31860_body19_e33541_d_n5;
            locals.var_w_bl_dn6 = assign31860_body19_e33541_d_n6;
            locals.var_w_bl_dn7 = assign31860_body19_e33541_d_n7;
            locals.var_w_bl_dn8 = assign31860_body19_e33541_d_n8;
            locals.var_w_bl_dn9 = assign31860_body19_e33541_d_n9;
            locals.var_w_bl_dn10 = assign31860_body19_e33541_d_n10;
            locals.var_w_bl_dn11 = assign31860_body19_e33541_d_n11;
            locals.var_w_bl_dn14 = assign31860_body19_e33541_d_n14;
            let assign31860_body20_e33544: f64 = (locals.var_phi_bl_dep - locals.var_phi_bl_dep_old);
            let assign31860_body20_e33545: f64 = (assign31860_body20_e33544).abs();
            let assign31860_body20_e33547: f64 = if assign31860_body20_e33545 <= 1e-8 { 1.0 } else { 0.0 };
            locals.var_guard744 = assign31860_body20_e33547;
            let (assign31860_body21_e33570,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) && (locals.var_guard744 != 0.0)) {
        let assign31860_body21_e33568: f64 = (150.0 + 1.0);
        (assign31860_body21_e33568,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31860_body21_e33570;
            let (assign31860_body22_e33589,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) {
        (locals.var_phi_bl_dep,)
    } else {
        (locals.var_phi_bl_dep_old,)
    }
};
            locals.var_phi_bl_dep_old = assign31860_body22_e33589;
            let (assign31860_body23_e33610,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign31860_body23_e33608: f64 = (locals.var_lp_s0 + 1.0);
        (assign31860_body23_e33608,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign31860_body23_e33610;
        }

        let (assign31870_e33628, assign31870_e33628_d_n0, assign31870_e33628_d_n2, assign31870_e33628_d_n4, assign31870_e33628_d_n5, assign31870_e33628_d_n6, assign31870_e33628_d_n7, assign31870_e33628_d_n8, assign31870_e33628_d_n9, assign31870_e33628_d_n10, assign31870_e33628_d_n11, assign31870_e33628_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 == 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign31870_e33628;
        locals.var_phi_bl_dep_dn0 = assign31870_e33628_d_n0;
        locals.var_phi_bl_dep_dn2 = assign31870_e33628_d_n2;
        locals.var_phi_bl_dep_dn4 = assign31870_e33628_d_n4;
        locals.var_phi_bl_dep_dn5 = assign31870_e33628_d_n5;
        locals.var_phi_bl_dep_dn6 = assign31870_e33628_d_n6;
        locals.var_phi_bl_dep_dn7 = assign31870_e33628_d_n7;
        locals.var_phi_bl_dep_dn8 = assign31870_e33628_d_n8;
        locals.var_phi_bl_dep_dn9 = assign31870_e33628_d_n9;
        locals.var_phi_bl_dep_dn10 = assign31870_e33628_d_n10;
        locals.var_phi_bl_dep_dn11 = assign31870_e33628_d_n11;
        locals.var_phi_bl_dep_dn14 = assign31870_e33628_d_n14;

        let (assign31880_e33646, assign31880_e33646_d_n0, assign31880_e33646_d_n2, assign31880_e33646_d_n4, assign31880_e33646_d_n5, assign31880_e33646_d_n6, assign31880_e33646_d_n7, assign31880_e33646_d_n8, assign31880_e33646_d_n9, assign31880_e33646_d_n10, assign31880_e33646_d_n11, assign31880_e33646_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 == 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign31880_e33646;
        locals.var_phi_jl_dep_dn0 = assign31880_e33646_d_n0;
        locals.var_phi_jl_dep_dn2 = assign31880_e33646_d_n2;
        locals.var_phi_jl_dep_dn4 = assign31880_e33646_d_n4;
        locals.var_phi_jl_dep_dn5 = assign31880_e33646_d_n5;
        locals.var_phi_jl_dep_dn6 = assign31880_e33646_d_n6;
        locals.var_phi_jl_dep_dn7 = assign31880_e33646_d_n7;
        locals.var_phi_jl_dep_dn8 = assign31880_e33646_d_n8;
        locals.var_phi_jl_dep_dn9 = assign31880_e33646_d_n9;
        locals.var_phi_jl_dep_dn10 = assign31880_e33646_d_n10;
        locals.var_phi_jl_dep_dn11 = assign31880_e33646_d_n11;
        locals.var_phi_jl_dep_dn14 = assign31880_e33646_d_n14;

        let (assign31890_e33664, assign31890_e33664_d_n0, assign31890_e33664_d_n2, assign31890_e33664_d_n4, assign31890_e33664_d_n5, assign31890_e33664_d_n6, assign31890_e33664_d_n7, assign31890_e33664_d_n8, assign31890_e33664_d_n9, assign31890_e33664_d_n10, assign31890_e33664_d_n11, assign31890_e33664_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard733 == 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 == 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    }
};
        locals.var_phi_sl_dep_ini = assign31890_e33664;
        locals.var_phi_sl_dep_ini_dn0 = assign31890_e33664_d_n0;
        locals.var_phi_sl_dep_ini_dn2 = assign31890_e33664_d_n2;
        locals.var_phi_sl_dep_ini_dn4 = assign31890_e33664_d_n4;
        locals.var_phi_sl_dep_ini_dn5 = assign31890_e33664_d_n5;
        locals.var_phi_sl_dep_ini_dn6 = assign31890_e33664_d_n6;
        locals.var_phi_sl_dep_ini_dn7 = assign31890_e33664_d_n7;
        locals.var_phi_sl_dep_ini_dn8 = assign31890_e33664_d_n8;
        locals.var_phi_sl_dep_ini_dn9 = assign31890_e33664_d_n9;
        locals.var_phi_sl_dep_ini_dn10 = assign31890_e33664_d_n10;
        locals.var_phi_sl_dep_ini_dn11 = assign31890_e33664_d_n11;
        locals.var_phi_sl_dep_ini_dn14 = assign31890_e33664_d_n14;

        let (assign31900_e33673, assign31900_e33673_d_n0, assign31900_e33673_d_n2, assign31900_e33673_d_n4, assign31900_e33673_d_n5, assign31900_e33673_d_n6, assign31900_e33673_d_n7, assign31900_e33673_d_n8, assign31900_e33673_d_n9, assign31900_e33673_d_n10, assign31900_e33673_d_n11, assign31900_e33673_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_phi_b0_dep_ini, locals.var_phi_b0_dep_ini_dn0, locals.var_phi_b0_dep_ini_dn2, locals.var_phi_b0_dep_ini_dn4, locals.var_phi_b0_dep_ini_dn5, locals.var_phi_b0_dep_ini_dn6, locals.var_phi_b0_dep_ini_dn7, locals.var_phi_b0_dep_ini_dn8, locals.var_phi_b0_dep_ini_dn9, locals.var_phi_b0_dep_ini_dn10, locals.var_phi_b0_dep_ini_dn11, locals.var_phi_b0_dep_ini_dn14,)
    }
};
        locals.var_phi_b0_dep_ini = assign31900_e33673;
        locals.var_phi_b0_dep_ini_dn0 = assign31900_e33673_d_n0;
        locals.var_phi_b0_dep_ini_dn2 = assign31900_e33673_d_n2;
        locals.var_phi_b0_dep_ini_dn4 = assign31900_e33673_d_n4;
        locals.var_phi_b0_dep_ini_dn5 = assign31900_e33673_d_n5;
        locals.var_phi_b0_dep_ini_dn6 = assign31900_e33673_d_n6;
        locals.var_phi_b0_dep_ini_dn7 = assign31900_e33673_d_n7;
        locals.var_phi_b0_dep_ini_dn8 = assign31900_e33673_d_n8;
        locals.var_phi_b0_dep_ini_dn9 = assign31900_e33673_d_n9;
        locals.var_phi_b0_dep_ini_dn10 = assign31900_e33673_d_n10;
        locals.var_phi_b0_dep_ini_dn11 = assign31900_e33673_d_n11;
        locals.var_phi_b0_dep_ini_dn14 = assign31900_e33673_d_n14;

        let (assign31910_e33682,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign31910_e33682;

        let (assign31920_e33691, assign31920_e33691_d_n0, assign31920_e33691_d_n2, assign31920_e33691_d_n4, assign31920_e33691_d_n5, assign31920_e33691_d_n6, assign31920_e33691_d_n7, assign31920_e33691_d_n8, assign31920_e33691_d_n9, assign31920_e33691_d_n10, assign31920_e33691_d_n11, assign31920_e33691_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) {
        (locals.var_phi_sl_dep_ini, locals.var_phi_sl_dep_ini_dn0, locals.var_phi_sl_dep_ini_dn2, locals.var_phi_sl_dep_ini_dn4, locals.var_phi_sl_dep_ini_dn5, locals.var_phi_sl_dep_ini_dn6, locals.var_phi_sl_dep_ini_dn7, locals.var_phi_sl_dep_ini_dn8, locals.var_phi_sl_dep_ini_dn9, locals.var_phi_sl_dep_ini_dn10, locals.var_phi_sl_dep_ini_dn11, locals.var_phi_sl_dep_ini_dn14,)
    } else {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    }
};
        locals.var_phi_sl_dep = assign31920_e33691;
        locals.var_phi_sl_dep_dn0 = assign31920_e33691_d_n0;
        locals.var_phi_sl_dep_dn2 = assign31920_e33691_d_n2;
        locals.var_phi_sl_dep_dn4 = assign31920_e33691_d_n4;
        locals.var_phi_sl_dep_dn5 = assign31920_e33691_d_n5;
        locals.var_phi_sl_dep_dn6 = assign31920_e33691_d_n6;
        locals.var_phi_sl_dep_dn7 = assign31920_e33691_d_n7;
        locals.var_phi_sl_dep_dn8 = assign31920_e33691_d_n8;
        locals.var_phi_sl_dep_dn9 = assign31920_e33691_d_n9;
        locals.var_phi_sl_dep_dn10 = assign31920_e33691_d_n10;
        locals.var_phi_sl_dep_dn11 = assign31920_e33691_d_n11;
        locals.var_phi_sl_dep_dn14 = assign31920_e33691_d_n14;

    }
}
