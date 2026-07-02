#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_352(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95310_e147971, assign95310_e147971_d_n0, assign95310_e147971_d_n2, assign95310_e147971_d_n4, assign95310_e147971_d_n5, assign95310_e147971_d_n6, assign95310_e147971_d_n7, assign95310_e147971_d_n8, assign95310_e147971_d_n9, assign95310_e147971_d_n10, assign95310_e147971_d_n11, assign95310_e147971_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95310_e147969: f64 = (p.p63 * locals.var_t1);
        (assign95310_e147969, (p.p63 * locals.var_t1_dn0), (p.p63 * locals.var_t1_dn2), (p.p63 * locals.var_t1_dn4), (p.p63 * locals.var_t1_dn5), (p.p63 * locals.var_t1_dn6), (p.p63 * locals.var_t1_dn7), (p.p63 * locals.var_t1_dn8), (p.p63 * locals.var_t1_dn9), (p.p63 * locals.var_t1_dn10), (p.p63 * locals.var_t1_dn11), (p.p63 * locals.var_t1_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign95310_e147971;
        locals.var_t5_dn0 = assign95310_e147971_d_n0;
        locals.var_t5_dn2 = assign95310_e147971_d_n2;
        locals.var_t5_dn4 = assign95310_e147971_d_n4;
        locals.var_t5_dn5 = assign95310_e147971_d_n5;
        locals.var_t5_dn6 = assign95310_e147971_d_n6;
        locals.var_t5_dn7 = assign95310_e147971_d_n7;
        locals.var_t5_dn8 = assign95310_e147971_d_n8;
        locals.var_t5_dn9 = assign95310_e147971_d_n9;
        locals.var_t5_dn10 = assign95310_e147971_d_n10;
        locals.var_t5_dn11 = assign95310_e147971_d_n11;
        locals.var_t5_dn14 = assign95310_e147971_d_n14;

        let (assign95320_e147980, assign95320_e147980_d_n0, assign95320_e147980_d_n2, assign95320_e147980_d_n4, assign95320_e147980_d_n5, assign95320_e147980_d_n6, assign95320_e147980_d_n7, assign95320_e147980_d_n8, assign95320_e147980_d_n9, assign95320_e147980_d_n10, assign95320_e147980_d_n11, assign95320_e147980_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95320_e147978: f64 = (1.2 - locals.var_ps0);
        (assign95320_e147978, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn4), (-locals.var_ps0_dn5), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn8), (-locals.var_ps0_dn9), (-locals.var_ps0_dn10), (-locals.var_ps0_dn11), (-locals.var_ps0_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign95320_e147980;
        locals.var_t9_dn0 = assign95320_e147980_d_n0;
        locals.var_t9_dn2 = assign95320_e147980_d_n2;
        locals.var_t9_dn4 = assign95320_e147980_d_n4;
        locals.var_t9_dn5 = assign95320_e147980_d_n5;
        locals.var_t9_dn6 = assign95320_e147980_d_n6;
        locals.var_t9_dn7 = assign95320_e147980_d_n7;
        locals.var_t9_dn8 = assign95320_e147980_d_n8;
        locals.var_t9_dn9 = assign95320_e147980_d_n9;
        locals.var_t9_dn10 = assign95320_e147980_d_n10;
        locals.var_t9_dn11 = assign95320_e147980_d_n11;
        locals.var_t9_dn14 = assign95320_e147980_d_n14;

        let (assign95330_e147993, assign95330_e147993_d_n0, assign95330_e147993_d_n2, assign95330_e147993_d_n4, assign95330_e147993_d_n5, assign95330_e147993_d_n6, assign95330_e147993_d_n7, assign95330_e147993_d_n8, assign95330_e147993_d_n9, assign95330_e147993_d_n10, assign95330_e147993_d_n11, assign95330_e147993_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95330_e147987: f64 = (locals.var_vgs * locals.var_t5);
        let assign95330_e147990: f64 = (locals.var_t4 * locals.var_t9);
        let assign95330_e147991: f64 = (assign95330_e147987 - assign95330_e147990);
        (assign95330_e147991, ((locals.var_vgs * locals.var_t5_dn0) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((locals.var_vgs * locals.var_t5_dn2) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((locals.var_vgs * locals.var_t5_dn4) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((locals.var_vgs * locals.var_t5_dn5) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), (((locals.var_vgs_dn6 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), (((locals.var_vgs_dn7 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), (((locals.var_vgs_dn8 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((locals.var_vgs * locals.var_t5_dn9) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((locals.var_vgs * locals.var_t5_dn10) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((locals.var_vgs * locals.var_t5_dn11) - ((locals.var_t4_dn11 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn11))), ((locals.var_vgs * locals.var_t5_dn14) - ((locals.var_t4_dn14 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn14))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn8, locals.var_qgod_dn9, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn14,)
    }
};
        locals.var_qgod = assign95330_e147993;
        locals.var_qgod_dn0 = assign95330_e147993_d_n0;
        locals.var_qgod_dn2 = assign95330_e147993_d_n2;
        locals.var_qgod_dn4 = assign95330_e147993_d_n4;
        locals.var_qgod_dn5 = assign95330_e147993_d_n5;
        locals.var_qgod_dn6 = assign95330_e147993_d_n6;
        locals.var_qgod_dn7 = assign95330_e147993_d_n7;
        locals.var_qgod_dn8 = assign95330_e147993_d_n8;
        locals.var_qgod_dn9 = assign95330_e147993_d_n9;
        locals.var_qgod_dn10 = assign95330_e147993_d_n10;
        locals.var_qgod_dn11 = assign95330_e147993_d_n11;
        locals.var_qgod_dn14 = assign95330_e147993_d_n14;

        let (assign95340_e148000,) = {
    if (locals.var_cgso_given != 0.0) {
        let assign95340_e147997: f64 = (-locals.var_weffcv_nf);
        let assign95340_e147998: f64 = (locals.var_uc_cgso * assign95340_e147997);
        (assign95340_e147998,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95340_e148000;

        let assign95350_e148003: f64 = if locals.var_flg_coovlps == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2221 = assign95350_e148003;

        let (assign95360_e148015,) = {
    if ((locals.var_cgso_given == 0.0) && (locals.var_guard2221 != 0.0)) {
        let assign95360_e148009: f64 = (-locals.var_cox0);
        let assign95360_e148011: f64 = (assign95360_e148009 * p.p66);
        let assign95360_e148013: f64 = (assign95360_e148011 * locals.var_weffcv_nf);
        (assign95360_e148013,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95360_e148015;

        let assign95370_e148017: f64 = (-locals.var_cgsoe);
        let assign95370_e148019: f64 = (assign95370_e148017 * locals.var_vgsei);
        locals.var_qgso = assign95370_e148019;
        locals.var_qgso_dn2 = (assign95370_e148017 * locals.var_vgsei_dn2);
        locals.var_qgso_dn7 = (assign95370_e148017 * locals.var_vgsei_dn7);

        let (assign95380_e148026,) = {
    if (locals.var_cgdo_given != 0.0) {
        let assign95380_e148023: f64 = (-locals.var_weffcv_nf);
        let assign95380_e148024: f64 = (locals.var_uc_cgdo * assign95380_e148023);
        (assign95380_e148024,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95380_e148026;

        let assign95390_e148029: f64 = if locals.var_flg_coovlp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2222 = assign95390_e148029;

        let (assign95400_e148041,) = {
    if ((locals.var_cgdo_given == 0.0) && (locals.var_guard2222 != 0.0)) {
        let assign95400_e148035: f64 = (-locals.var_coxb0);
        let assign95400_e148037: f64 = (assign95400_e148035 * p.p63);
        let assign95400_e148039: f64 = (assign95400_e148037 * locals.var_weffcv_nf);
        (assign95400_e148039,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95400_e148041;

        let assign95410_e148043: f64 = (-locals.var_cgdoe);
        let assign95410_e148046: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign95410_e148047: f64 = (assign95410_e148043 * assign95410_e148046);
        locals.var_qgdo = assign95410_e148047;
        locals.var_qgdo_dn0 = (assign95410_e148043 * (-locals.var_vdsei_dn0));
        locals.var_qgdo_dn2 = (assign95410_e148043 * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qgdo_dn7 = (assign95410_e148043 * locals.var_vgsei_dn7);

        let assign95420_e148050: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2223 = assign95420_e148050;

        let (assign95430_e148058, assign95430_e148058_d_n0, assign95430_e148058_d_n2, assign95430_e148058_d_n4, assign95430_e148058_d_n5, assign95430_e148058_d_n6, assign95430_e148058_d_n7, assign95430_e148058_d_n8, assign95430_e148058_d_n9, assign95430_e148058_d_n10, assign95430_e148058_d_n11, assign95430_e148058_d_n14,) = {
    if (locals.var_guard2223 != 0.0) {
        let assign95430_e148055: f64 = (locals.var_vds - locals.var_pds);
        let assign95430_e148056: f64 = (p.p431 * assign95430_e148055);
        (assign95430_e148056, (p.p431 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (p.p431 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (p.p431 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (p.p431 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (p.p431 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (p.p431 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (p.p431 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (p.p431 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (p.p431 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (p.p431 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (p.p431 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn11, locals.var_qodad_dn14,)
    }
};
        locals.var_qodad = assign95430_e148058;
        locals.var_qodad_dn0 = assign95430_e148058_d_n0;
        locals.var_qodad_dn2 = assign95430_e148058_d_n2;
        locals.var_qodad_dn4 = assign95430_e148058_d_n4;
        locals.var_qodad_dn5 = assign95430_e148058_d_n5;
        locals.var_qodad_dn6 = assign95430_e148058_d_n6;
        locals.var_qodad_dn7 = assign95430_e148058_d_n7;
        locals.var_qodad_dn8 = assign95430_e148058_d_n8;
        locals.var_qodad_dn9 = assign95430_e148058_d_n9;
        locals.var_qodad_dn10 = assign95430_e148058_d_n10;
        locals.var_qodad_dn11 = assign95430_e148058_d_n11;
        locals.var_qodad_dn14 = assign95430_e148058_d_n14;

        let (assign95440_e148064, assign95440_e148064_d_n0, assign95440_e148064_d_n2, assign95440_e148064_d_n4, assign95440_e148064_d_n5, assign95440_e148064_d_n6, assign95440_e148064_d_n7, assign95440_e148064_d_n8, assign95440_e148064_d_n9, assign95440_e148064_d_n10, assign95440_e148064_d_n11, assign95440_e148064_d_n14,) = {
    if (locals.var_guard2223 != 0.0) {
        let assign95440_e148062: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95440_e148062, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qovd_add, locals.var_qovd_add_dn0, locals.var_qovd_add_dn2, locals.var_qovd_add_dn4, locals.var_qovd_add_dn5, locals.var_qovd_add_dn6, locals.var_qovd_add_dn7, locals.var_qovd_add_dn8, locals.var_qovd_add_dn9, locals.var_qovd_add_dn10, locals.var_qovd_add_dn11, locals.var_qovd_add_dn14,)
    }
};
        locals.var_qovd_add = assign95440_e148064;
        locals.var_qovd_add_dn0 = assign95440_e148064_d_n0;
        locals.var_qovd_add_dn2 = assign95440_e148064_d_n2;
        locals.var_qovd_add_dn4 = assign95440_e148064_d_n4;
        locals.var_qovd_add_dn5 = assign95440_e148064_d_n5;
        locals.var_qovd_add_dn6 = assign95440_e148064_d_n6;
        locals.var_qovd_add_dn7 = assign95440_e148064_d_n7;
        locals.var_qovd_add_dn8 = assign95440_e148064_d_n8;
        locals.var_qovd_add_dn9 = assign95440_e148064_d_n9;
        locals.var_qovd_add_dn10 = assign95440_e148064_d_n10;
        locals.var_qovd_add_dn11 = assign95440_e148064_d_n11;
        locals.var_qovd_add_dn14 = assign95440_e148064_d_n14;

        let (assign95450_e148070, assign95450_e148070_d_n0, assign95450_e148070_d_n2, assign95450_e148070_d_n4, assign95450_e148070_d_n5, assign95450_e148070_d_n6, assign95450_e148070_d_n7, assign95450_e148070_d_n8, assign95450_e148070_d_n9, assign95450_e148070_d_n10, assign95450_e148070_d_n11, assign95450_e148070_d_n14,) = {
    if (locals.var_guard2223 != 0.0) {
        let assign95450_e148068: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95450_e148068, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qbdld_add, locals.var_qbdld_add_dn0, locals.var_qbdld_add_dn2, locals.var_qbdld_add_dn4, locals.var_qbdld_add_dn5, locals.var_qbdld_add_dn6, locals.var_qbdld_add_dn7, locals.var_qbdld_add_dn8, locals.var_qbdld_add_dn9, locals.var_qbdld_add_dn10, locals.var_qbdld_add_dn11, locals.var_qbdld_add_dn14,)
    }
};
        locals.var_qbdld_add = assign95450_e148070;
        locals.var_qbdld_add_dn0 = assign95450_e148070_d_n0;
        locals.var_qbdld_add_dn2 = assign95450_e148070_d_n2;
        locals.var_qbdld_add_dn4 = assign95450_e148070_d_n4;
        locals.var_qbdld_add_dn5 = assign95450_e148070_d_n5;
        locals.var_qbdld_add_dn6 = assign95450_e148070_d_n6;
        locals.var_qbdld_add_dn7 = assign95450_e148070_d_n7;
        locals.var_qbdld_add_dn8 = assign95450_e148070_d_n8;
        locals.var_qbdld_add_dn9 = assign95450_e148070_d_n9;
        locals.var_qbdld_add_dn10 = assign95450_e148070_d_n10;
        locals.var_qbdld_add_dn11 = assign95450_e148070_d_n11;
        locals.var_qbdld_add_dn14 = assign95450_e148070_d_n14;

        let (assign95460_e148080, assign95460_e148080_d_n0, assign95460_e148080_d_n2, assign95460_e148080_d_n4, assign95460_e148080_d_n5, assign95460_e148080_d_n6, assign95460_e148080_d_n7, assign95460_e148080_d_n8, assign95460_e148080_d_n9, assign95460_e148080_d_n10, assign95460_e148080_d_n11, assign95460_e148080_d_n14,) = {
    if (locals.var_guard2223 == 0.0) {
        let assign95460_e148074: f64 = (-p.p431);
        let assign95460_e148077: f64 = (locals.var_vds - locals.var_pds);
        let assign95460_e148078: f64 = (assign95460_e148074 * assign95460_e148077);
        (assign95460_e148078, (assign95460_e148074 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (assign95460_e148074 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (assign95460_e148074 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (assign95460_e148074 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (assign95460_e148074 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (assign95460_e148074 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (assign95460_e148074 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (assign95460_e148074 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (assign95460_e148074 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (assign95460_e148074 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (assign95460_e148074 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn11, locals.var_qodad_dn14,)
    }
};
        locals.var_qodad = assign95460_e148080;
        locals.var_qodad_dn0 = assign95460_e148080_d_n0;
        locals.var_qodad_dn2 = assign95460_e148080_d_n2;
        locals.var_qodad_dn4 = assign95460_e148080_d_n4;
        locals.var_qodad_dn5 = assign95460_e148080_d_n5;
        locals.var_qodad_dn6 = assign95460_e148080_d_n6;
        locals.var_qodad_dn7 = assign95460_e148080_d_n7;
        locals.var_qodad_dn8 = assign95460_e148080_d_n8;
        locals.var_qodad_dn9 = assign95460_e148080_d_n9;
        locals.var_qodad_dn10 = assign95460_e148080_d_n10;
        locals.var_qodad_dn11 = assign95460_e148080_d_n11;
        locals.var_qodad_dn14 = assign95460_e148080_d_n14;

        let (assign95470_e148087, assign95470_e148087_d_n0, assign95470_e148087_d_n2, assign95470_e148087_d_n4, assign95470_e148087_d_n5, assign95470_e148087_d_n6, assign95470_e148087_d_n7, assign95470_e148087_d_n8, assign95470_e148087_d_n9, assign95470_e148087_d_n10, assign95470_e148087_d_n11, assign95470_e148087_d_n14,) = {
    if (locals.var_guard2223 == 0.0) {
        let assign95470_e148085: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95470_e148085, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qovs_add, locals.var_qovs_add_dn0, locals.var_qovs_add_dn2, locals.var_qovs_add_dn4, locals.var_qovs_add_dn5, locals.var_qovs_add_dn6, locals.var_qovs_add_dn7, locals.var_qovs_add_dn8, locals.var_qovs_add_dn9, locals.var_qovs_add_dn10, locals.var_qovs_add_dn11, locals.var_qovs_add_dn14,)
    }
};
        locals.var_qovs_add = assign95470_e148087;
        locals.var_qovs_add_dn0 = assign95470_e148087_d_n0;
        locals.var_qovs_add_dn2 = assign95470_e148087_d_n2;
        locals.var_qovs_add_dn4 = assign95470_e148087_d_n4;
        locals.var_qovs_add_dn5 = assign95470_e148087_d_n5;
        locals.var_qovs_add_dn6 = assign95470_e148087_d_n6;
        locals.var_qovs_add_dn7 = assign95470_e148087_d_n7;
        locals.var_qovs_add_dn8 = assign95470_e148087_d_n8;
        locals.var_qovs_add_dn9 = assign95470_e148087_d_n9;
        locals.var_qovs_add_dn10 = assign95470_e148087_d_n10;
        locals.var_qovs_add_dn11 = assign95470_e148087_d_n11;
        locals.var_qovs_add_dn14 = assign95470_e148087_d_n14;

        let (assign95480_e148094, assign95480_e148094_d_n0, assign95480_e148094_d_n2, assign95480_e148094_d_n4, assign95480_e148094_d_n5, assign95480_e148094_d_n6, assign95480_e148094_d_n7, assign95480_e148094_d_n8, assign95480_e148094_d_n9, assign95480_e148094_d_n10, assign95480_e148094_d_n11, assign95480_e148094_d_n14,) = {
    if (locals.var_guard2223 == 0.0) {
        let assign95480_e148092: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95480_e148092, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qbsld_add, locals.var_qbsld_add_dn0, locals.var_qbsld_add_dn2, locals.var_qbsld_add_dn4, locals.var_qbsld_add_dn5, locals.var_qbsld_add_dn6, locals.var_qbsld_add_dn7, locals.var_qbsld_add_dn8, locals.var_qbsld_add_dn9, locals.var_qbsld_add_dn10, locals.var_qbsld_add_dn11, locals.var_qbsld_add_dn14,)
    }
};
        locals.var_qbsld_add = assign95480_e148094;
        locals.var_qbsld_add_dn0 = assign95480_e148094_d_n0;
        locals.var_qbsld_add_dn2 = assign95480_e148094_d_n2;
        locals.var_qbsld_add_dn4 = assign95480_e148094_d_n4;
        locals.var_qbsld_add_dn5 = assign95480_e148094_d_n5;
        locals.var_qbsld_add_dn6 = assign95480_e148094_d_n6;
        locals.var_qbsld_add_dn7 = assign95480_e148094_d_n7;
        locals.var_qbsld_add_dn8 = assign95480_e148094_d_n8;
        locals.var_qbsld_add_dn9 = assign95480_e148094_d_n9;
        locals.var_qbsld_add_dn10 = assign95480_e148094_d_n10;
        locals.var_qbsld_add_dn11 = assign95480_e148094_d_n11;
        locals.var_qbsld_add_dn14 = assign95480_e148094_d_n14;

        let assign95490_e148096: f64 = (-locals.var_uc_cgbo);
        let assign95490_e148098: f64 = (assign95490_e148096 * locals.var_lgate);
        locals.var_cgbo_loc = assign95490_e148098;

        let assign95500_e148100: f64 = (-locals.var_cgbo_loc);
        let assign95500_e148103: f64 = (locals.var_vgsi - locals.var_vbsi);
        let assign95500_e148104: f64 = (assign95500_e148100 * assign95500_e148103);
        locals.var_qgbo = assign95500_e148104;
        locals.var_qgbo_dn7 = (assign95500_e148100 * locals.var_vgsi_dn7);
        locals.var_qgbo_dn8 = (assign95500_e148100 * (locals.var_vgsi_dn8 - locals.var_vbsi_dn8));
        locals.var_qgbo_dn9 = (assign95500_e148100 * (-locals.var_vbsi_dn9));

        locals.var_aclm = locals.var_uc_clm1;

        let assign95520_e148108: f64 = if locals.var_flg_noqi != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2224 = assign95520_e148108;

        let (assign95530_e148122, assign95530_e148122_d_n0, assign95530_e148122_d_n2, assign95530_e148122_d_n4, assign95530_e148122_d_n5, assign95530_e148122_d_n6, assign95530_e148122_d_n7, assign95530_e148122_d_n8, assign95530_e148122_d_n9, assign95530_e148122_d_n10, assign95530_e148122_d_n11, assign95530_e148122_d_n14,) = {
    if (locals.var_guard2224 != 0.0) {
        let assign95530_e148113: f64 = (locals.var_vds + locals.var_ps0);
        let assign95530_e148114: f64 = (locals.var_aclm * assign95530_e148113);
        let assign95530_e148117: f64 = (1.0 - locals.var_aclm);
        let assign95530_e148119: f64 = (assign95530_e148117 * locals.var_psl);
        let assign95530_e148120: f64 = (assign95530_e148114 + assign95530_e148119);
        (assign95530_e148120, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign95530_e148117 * locals.var_psl_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign95530_e148117 * locals.var_psl_dn2)), ((locals.var_aclm * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + (assign95530_e148117 * locals.var_psl_dn4)), ((locals.var_aclm * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + (assign95530_e148117 * locals.var_psl_dn5)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign95530_e148117 * locals.var_psl_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign95530_e148117 * locals.var_psl_dn7)), ((locals.var_aclm * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + (assign95530_e148117 * locals.var_psl_dn8)), ((locals.var_aclm * (locals.var_vds_dn9 + locals.var_ps0_dn9)) + (assign95530_e148117 * locals.var_psl_dn9)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign95530_e148117 * locals.var_psl_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign95530_e148117 * locals.var_psl_dn11)), ((locals.var_aclm * (locals.var_vds_dn14 + locals.var_ps0_dn14)) + (assign95530_e148117 * locals.var_psl_dn14)),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95530_e148122;
        locals.var_psdl_dn0 = assign95530_e148122_d_n0;
        locals.var_psdl_dn2 = assign95530_e148122_d_n2;
        locals.var_psdl_dn4 = assign95530_e148122_d_n4;
        locals.var_psdl_dn5 = assign95530_e148122_d_n5;
        locals.var_psdl_dn6 = assign95530_e148122_d_n6;
        locals.var_psdl_dn7 = assign95530_e148122_d_n7;
        locals.var_psdl_dn8 = assign95530_e148122_d_n8;
        locals.var_psdl_dn9 = assign95530_e148122_d_n9;
        locals.var_psdl_dn10 = assign95530_e148122_d_n10;
        locals.var_psdl_dn11 = assign95530_e148122_d_n11;
        locals.var_psdl_dn14 = assign95530_e148122_d_n14;

        let assign95540_e148126: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95540_e148129: f64 = (10.0 * 2.220446049250313e-16);
        let assign95540_e148130: f64 = (assign95540_e148126 - assign95540_e148129);
        let assign95540_e148133: f64 = (10.0 * 2.220446049250313e-16);
        let assign95540_e148134: f64 = (assign95540_e148130 - assign95540_e148133);
        let assign95540_e148138: f64 = (10.0 * 2.220446049250313e-16);
        let assign95540_e148141: f64 = if ((locals.var_psdl > assign95540_e148134) && (assign95540_e148138 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2225 = assign95540_e148141;

        let (assign95550_e148159, assign95550_e148159_d_n0, assign95550_e148159_d_n2, assign95550_e148159_d_n4, assign95550_e148159_d_n5, assign95550_e148159_d_n6, assign95550_e148159_d_n7, assign95550_e148159_d_n8, assign95550_e148159_d_n9, assign95550_e148159_d_n10, assign95550_e148159_d_n11, assign95550_e148159_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95550_e148148: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95550_e148151: f64 = (10.0 * 2.220446049250313e-16);
        let assign95550_e148152: f64 = (assign95550_e148148 - assign95550_e148151);
        let assign95550_e148153: f64 = (locals.var_psdl - assign95550_e148152);
        let assign95550_e148156: f64 = (10.0 * 2.220446049250313e-16);
        let assign95550_e148157: f64 = (assign95550_e148153 + assign95550_e148156);
        (assign95550_e148157, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn11 - (locals.var_ps0_dn11 + locals.var_vds_dn11)), (locals.var_psdl_dn14 - (locals.var_ps0_dn14 + locals.var_vds_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign95550_e148159;
        locals.var_tmf1_dn0 = assign95550_e148159_d_n0;
        locals.var_tmf1_dn2 = assign95550_e148159_d_n2;
        locals.var_tmf1_dn4 = assign95550_e148159_d_n4;
        locals.var_tmf1_dn5 = assign95550_e148159_d_n5;
        locals.var_tmf1_dn6 = assign95550_e148159_d_n6;
        locals.var_tmf1_dn7 = assign95550_e148159_d_n7;
        locals.var_tmf1_dn8 = assign95550_e148159_d_n8;
        locals.var_tmf1_dn9 = assign95550_e148159_d_n9;
        locals.var_tmf1_dn10 = assign95550_e148159_d_n10;
        locals.var_tmf1_dn11 = assign95550_e148159_d_n11;
        locals.var_tmf1_dn14 = assign95550_e148159_d_n14;

        let (assign95560_e148167, assign95560_e148167_d_n0, assign95560_e148167_d_n2, assign95560_e148167_d_n4, assign95560_e148167_d_n5, assign95560_e148167_d_n6, assign95560_e148167_d_n7, assign95560_e148167_d_n8, assign95560_e148167_d_n9, assign95560_e148167_d_n10, assign95560_e148167_d_n11, assign95560_e148167_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95560_e148165: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign95560_e148165, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign95560_e148167;
        locals.var_x2_dn0 = assign95560_e148167_d_n0;
        locals.var_x2_dn2 = assign95560_e148167_d_n2;
        locals.var_x2_dn4 = assign95560_e148167_d_n4;
        locals.var_x2_dn5 = assign95560_e148167_d_n5;
        locals.var_x2_dn6 = assign95560_e148167_d_n6;
        locals.var_x2_dn7 = assign95560_e148167_d_n7;
        locals.var_x2_dn8 = assign95560_e148167_d_n8;
        locals.var_x2_dn9 = assign95560_e148167_d_n9;
        locals.var_x2_dn10 = assign95560_e148167_d_n10;
        locals.var_x2_dn11 = assign95560_e148167_d_n11;
        locals.var_x2_dn14 = assign95560_e148167_d_n14;

        let (assign95570_e148179, assign95570_e148179_d_n0, assign95570_e148179_d_n2, assign95570_e148179_d_n4, assign95570_e148179_d_n5, assign95570_e148179_d_n6, assign95570_e148179_d_n7, assign95570_e148179_d_n8, assign95570_e148179_d_n9, assign95570_e148179_d_n10, assign95570_e148179_d_n11, assign95570_e148179_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95570_e148173: f64 = (10.0 * 2.220446049250313e-16);
        let assign95570_e148176: f64 = (10.0 * 2.220446049250313e-16);
        let assign95570_e148177: f64 = (assign95570_e148173 * assign95570_e148176);
        (assign95570_e148177, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign95570_e148179;
        locals.var_xmax2_dn0 = assign95570_e148179_d_n0;
        locals.var_xmax2_dn2 = assign95570_e148179_d_n2;
        locals.var_xmax2_dn4 = assign95570_e148179_d_n4;
        locals.var_xmax2_dn5 = assign95570_e148179_d_n5;
        locals.var_xmax2_dn6 = assign95570_e148179_d_n6;
        locals.var_xmax2_dn7 = assign95570_e148179_d_n7;
        locals.var_xmax2_dn8 = assign95570_e148179_d_n8;
        locals.var_xmax2_dn9 = assign95570_e148179_d_n9;
        locals.var_xmax2_dn10 = assign95570_e148179_d_n10;
        locals.var_xmax2_dn11 = assign95570_e148179_d_n11;
        locals.var_xmax2_dn14 = assign95570_e148179_d_n14;

        let (assign95580_e148185, assign95580_e148185_d_n0, assign95580_e148185_d_n2, assign95580_e148185_d_n4, assign95580_e148185_d_n5, assign95580_e148185_d_n6, assign95580_e148185_d_n7, assign95580_e148185_d_n8, assign95580_e148185_d_n9, assign95580_e148185_d_n10, assign95580_e148185_d_n11, assign95580_e148185_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95580_e148185;
        locals.var_xp_dn0 = assign95580_e148185_d_n0;
        locals.var_xp_dn2 = assign95580_e148185_d_n2;
        locals.var_xp_dn4 = assign95580_e148185_d_n4;
        locals.var_xp_dn5 = assign95580_e148185_d_n5;
        locals.var_xp_dn6 = assign95580_e148185_d_n6;
        locals.var_xp_dn7 = assign95580_e148185_d_n7;
        locals.var_xp_dn8 = assign95580_e148185_d_n8;
        locals.var_xp_dn9 = assign95580_e148185_d_n9;
        locals.var_xp_dn10 = assign95580_e148185_d_n10;
        locals.var_xp_dn11 = assign95580_e148185_d_n11;
        locals.var_xp_dn14 = assign95580_e148185_d_n14;

        let (assign95590_e148191, assign95590_e148191_d_n0, assign95590_e148191_d_n2, assign95590_e148191_d_n4, assign95590_e148191_d_n5, assign95590_e148191_d_n6, assign95590_e148191_d_n7, assign95590_e148191_d_n8, assign95590_e148191_d_n9, assign95590_e148191_d_n10, assign95590_e148191_d_n11, assign95590_e148191_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95590_e148191;
        locals.var_xmp_dn0 = assign95590_e148191_d_n0;
        locals.var_xmp_dn2 = assign95590_e148191_d_n2;
        locals.var_xmp_dn4 = assign95590_e148191_d_n4;
        locals.var_xmp_dn5 = assign95590_e148191_d_n5;
        locals.var_xmp_dn6 = assign95590_e148191_d_n6;
        locals.var_xmp_dn7 = assign95590_e148191_d_n7;
        locals.var_xmp_dn8 = assign95590_e148191_d_n8;
        locals.var_xmp_dn9 = assign95590_e148191_d_n9;
        locals.var_xmp_dn10 = assign95590_e148191_d_n10;
        locals.var_xmp_dn11 = assign95590_e148191_d_n11;
        locals.var_xmp_dn14 = assign95590_e148191_d_n14;

        let (assign95600_e148197,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95600_e148197;

        let (assign95610_e148203,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95610_e148203;

        let (assign95620_e148209, assign95620_e148209_d_n0, assign95620_e148209_d_n2, assign95620_e148209_d_n4, assign95620_e148209_d_n5, assign95620_e148209_d_n6, assign95620_e148209_d_n7, assign95620_e148209_d_n8, assign95620_e148209_d_n9, assign95620_e148209_d_n10, assign95620_e148209_d_n11, assign95620_e148209_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign95620_e148209;
        locals.var_arg_dn0 = assign95620_e148209_d_n0;
        locals.var_arg_dn2 = assign95620_e148209_d_n2;
        locals.var_arg_dn4 = assign95620_e148209_d_n4;
        locals.var_arg_dn5 = assign95620_e148209_d_n5;
        locals.var_arg_dn6 = assign95620_e148209_d_n6;
        locals.var_arg_dn7 = assign95620_e148209_d_n7;
        locals.var_arg_dn8 = assign95620_e148209_d_n8;
        locals.var_arg_dn9 = assign95620_e148209_d_n9;
        locals.var_arg_dn10 = assign95620_e148209_d_n10;
        locals.var_arg_dn11 = assign95620_e148209_d_n11;
        locals.var_arg_dn14 = assign95620_e148209_d_n14;

        let (assign95630_e148215, assign95630_e148215_d_n0, assign95630_e148215_d_n2, assign95630_e148215_d_n4, assign95630_e148215_d_n5, assign95630_e148215_d_n6, assign95630_e148215_d_n7, assign95630_e148215_d_n8, assign95630_e148215_d_n9, assign95630_e148215_d_n10, assign95630_e148215_d_n11, assign95630_e148215_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95630_e148215;
        locals.var_dnm_dn0 = assign95630_e148215_d_n0;
        locals.var_dnm_dn2 = assign95630_e148215_d_n2;
        locals.var_dnm_dn4 = assign95630_e148215_d_n4;
        locals.var_dnm_dn5 = assign95630_e148215_d_n5;
        locals.var_dnm_dn6 = assign95630_e148215_d_n6;
        locals.var_dnm_dn7 = assign95630_e148215_d_n7;
        locals.var_dnm_dn8 = assign95630_e148215_d_n8;
        locals.var_dnm_dn9 = assign95630_e148215_d_n9;
        locals.var_dnm_dn10 = assign95630_e148215_d_n10;
        locals.var_dnm_dn11 = assign95630_e148215_d_n11;
        locals.var_dnm_dn14 = assign95630_e148215_d_n14;

        let (assign95640_e148223, assign95640_e148223_d_n0, assign95640_e148223_d_n2, assign95640_e148223_d_n4, assign95640_e148223_d_n5, assign95640_e148223_d_n6, assign95640_e148223_d_n7, assign95640_e148223_d_n8, assign95640_e148223_d_n9, assign95640_e148223_d_n10, assign95640_e148223_d_n11, assign95640_e148223_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95640_e148221: f64 = (locals.var_xp * locals.var_x2);
        (assign95640_e148221, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95640_e148223;
        locals.var_xp_dn0 = assign95640_e148223_d_n0;
        locals.var_xp_dn2 = assign95640_e148223_d_n2;
        locals.var_xp_dn4 = assign95640_e148223_d_n4;
        locals.var_xp_dn5 = assign95640_e148223_d_n5;
        locals.var_xp_dn6 = assign95640_e148223_d_n6;
        locals.var_xp_dn7 = assign95640_e148223_d_n7;
        locals.var_xp_dn8 = assign95640_e148223_d_n8;
        locals.var_xp_dn9 = assign95640_e148223_d_n9;
        locals.var_xp_dn10 = assign95640_e148223_d_n10;
        locals.var_xp_dn11 = assign95640_e148223_d_n11;
        locals.var_xp_dn14 = assign95640_e148223_d_n14;

    }

    pub(super) fn stamp_transient_block_353(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95650_e148231, assign95650_e148231_d_n0, assign95650_e148231_d_n2, assign95650_e148231_d_n4, assign95650_e148231_d_n5, assign95650_e148231_d_n6, assign95650_e148231_d_n7, assign95650_e148231_d_n8, assign95650_e148231_d_n9, assign95650_e148231_d_n10, assign95650_e148231_d_n11, assign95650_e148231_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95650_e148229: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95650_e148229, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95650_e148231;
        locals.var_xmp_dn0 = assign95650_e148231_d_n0;
        locals.var_xmp_dn2 = assign95650_e148231_d_n2;
        locals.var_xmp_dn4 = assign95650_e148231_d_n4;
        locals.var_xmp_dn5 = assign95650_e148231_d_n5;
        locals.var_xmp_dn6 = assign95650_e148231_d_n6;
        locals.var_xmp_dn7 = assign95650_e148231_d_n7;
        locals.var_xmp_dn8 = assign95650_e148231_d_n8;
        locals.var_xmp_dn9 = assign95650_e148231_d_n9;
        locals.var_xmp_dn10 = assign95650_e148231_d_n10;
        locals.var_xmp_dn11 = assign95650_e148231_d_n11;
        locals.var_xmp_dn14 = assign95650_e148231_d_n14;

        let (assign95660_e148239, assign95660_e148239_d_n0, assign95660_e148239_d_n2, assign95660_e148239_d_n4, assign95660_e148239_d_n5, assign95660_e148239_d_n6, assign95660_e148239_d_n7, assign95660_e148239_d_n8, assign95660_e148239_d_n9, assign95660_e148239_d_n10, assign95660_e148239_d_n11, assign95660_e148239_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95660_e148237: f64 = (locals.var_xp * locals.var_x2);
        (assign95660_e148237, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95660_e148239;
        locals.var_xp_dn0 = assign95660_e148239_d_n0;
        locals.var_xp_dn2 = assign95660_e148239_d_n2;
        locals.var_xp_dn4 = assign95660_e148239_d_n4;
        locals.var_xp_dn5 = assign95660_e148239_d_n5;
        locals.var_xp_dn6 = assign95660_e148239_d_n6;
        locals.var_xp_dn7 = assign95660_e148239_d_n7;
        locals.var_xp_dn8 = assign95660_e148239_d_n8;
        locals.var_xp_dn9 = assign95660_e148239_d_n9;
        locals.var_xp_dn10 = assign95660_e148239_d_n10;
        locals.var_xp_dn11 = assign95660_e148239_d_n11;
        locals.var_xp_dn14 = assign95660_e148239_d_n14;

        let (assign95670_e148247, assign95670_e148247_d_n0, assign95670_e148247_d_n2, assign95670_e148247_d_n4, assign95670_e148247_d_n5, assign95670_e148247_d_n6, assign95670_e148247_d_n7, assign95670_e148247_d_n8, assign95670_e148247_d_n9, assign95670_e148247_d_n10, assign95670_e148247_d_n11, assign95670_e148247_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95670_e148245: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95670_e148245, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95670_e148247;
        locals.var_xmp_dn0 = assign95670_e148247_d_n0;
        locals.var_xmp_dn2 = assign95670_e148247_d_n2;
        locals.var_xmp_dn4 = assign95670_e148247_d_n4;
        locals.var_xmp_dn5 = assign95670_e148247_d_n5;
        locals.var_xmp_dn6 = assign95670_e148247_d_n6;
        locals.var_xmp_dn7 = assign95670_e148247_d_n7;
        locals.var_xmp_dn8 = assign95670_e148247_d_n8;
        locals.var_xmp_dn9 = assign95670_e148247_d_n9;
        locals.var_xmp_dn10 = assign95670_e148247_d_n10;
        locals.var_xmp_dn11 = assign95670_e148247_d_n11;
        locals.var_xmp_dn14 = assign95670_e148247_d_n14;

        let (assign95680_e148255, assign95680_e148255_d_n0, assign95680_e148255_d_n2, assign95680_e148255_d_n4, assign95680_e148255_d_n5, assign95680_e148255_d_n6, assign95680_e148255_d_n7, assign95680_e148255_d_n8, assign95680_e148255_d_n9, assign95680_e148255_d_n10, assign95680_e148255_d_n11, assign95680_e148255_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95680_e148253: f64 = (locals.var_xp + locals.var_xmp);
        (assign95680_e148253, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign95680_e148255;
        locals.var_arg_dn0 = assign95680_e148255_d_n0;
        locals.var_arg_dn2 = assign95680_e148255_d_n2;
        locals.var_arg_dn4 = assign95680_e148255_d_n4;
        locals.var_arg_dn5 = assign95680_e148255_d_n5;
        locals.var_arg_dn6 = assign95680_e148255_d_n6;
        locals.var_arg_dn7 = assign95680_e148255_d_n7;
        locals.var_arg_dn8 = assign95680_e148255_d_n8;
        locals.var_arg_dn9 = assign95680_e148255_d_n9;
        locals.var_arg_dn10 = assign95680_e148255_d_n10;
        locals.var_arg_dn11 = assign95680_e148255_d_n11;
        locals.var_arg_dn14 = assign95680_e148255_d_n14;

        let (assign95690_e148261, assign95690_e148261_d_n0, assign95690_e148261_d_n2, assign95690_e148261_d_n4, assign95690_e148261_d_n5, assign95690_e148261_d_n6, assign95690_e148261_d_n7, assign95690_e148261_d_n8, assign95690_e148261_d_n9, assign95690_e148261_d_n10, assign95690_e148261_d_n11, assign95690_e148261_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95690_e148261;
        locals.var_dnm_dn0 = assign95690_e148261_d_n0;
        locals.var_dnm_dn2 = assign95690_e148261_d_n2;
        locals.var_dnm_dn4 = assign95690_e148261_d_n4;
        locals.var_dnm_dn5 = assign95690_e148261_d_n5;
        locals.var_dnm_dn6 = assign95690_e148261_d_n6;
        locals.var_dnm_dn7 = assign95690_e148261_d_n7;
        locals.var_dnm_dn8 = assign95690_e148261_d_n8;
        locals.var_dnm_dn9 = assign95690_e148261_d_n9;
        locals.var_dnm_dn10 = assign95690_e148261_d_n10;
        locals.var_dnm_dn11 = assign95690_e148261_d_n11;
        locals.var_dnm_dn14 = assign95690_e148261_d_n14;

        let assign95700_e148276: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2226 = assign95700_e148276;

        let assign95710_e148279: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2227 = assign95710_e148279;

        let (assign95720_e148289,) = {
    if ((((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) && (locals.var_guard2227 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95720_e148289;

        let assign95730_e148292: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2228 = assign95730_e148292;

        let (assign95740_e148305,) = {
    if (((((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) && (locals.var_guard2227 == 0.0)) && (locals.var_guard2228 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95740_e148305;

        let assign95750_e148308: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2229 = assign95750_e148308;

        let (assign95760_e148324,) = {
    if ((((((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) && (locals.var_guard2227 == 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95760_e148324;

        let assign95770_e148327: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2230 = assign95770_e148327;

        let (assign95780_e148346,) = {
    if (((((((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) && (locals.var_guard2227 == 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2230 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95780_e148346;

        let (assign95790_e148354,) = {
    if (((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95790_e148354;

        let mut assign95800_loop_guard: usize = 0;
        while {
            let assign95800_cond_e148363: f64 = if ((((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign95800_cond_e148363 != 0.0
        } {
            assign95800_loop_guard += 1;
            assert!(assign95800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign95800_body0_e148372, assign95800_body0_e148372_d_n0, assign95800_body0_e148372_d_n2, assign95800_body0_e148372_d_n4, assign95800_body0_e148372_d_n5, assign95800_body0_e148372_d_n6, assign95800_body0_e148372_d_n7, assign95800_body0_e148372_d_n8, assign95800_body0_e148372_d_n9, assign95800_body0_e148372_d_n10, assign95800_body0_e148372_d_n11, assign95800_body0_e148372_d_n14,) = {
    if (((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) {
        let assign95800_body0_e148370: f64 = (locals.var_dnm).sqrt();
        (assign95800_body0_e148370, (locals.var_dnm_dn0 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn2 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn4 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn5 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn6 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn7 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn8 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn9 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn10 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn11 / (2.0 * assign95800_body0_e148370)), (locals.var_dnm_dn14 / (2.0 * assign95800_body0_e148370)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign95800_body0_e148372;
            locals.var_dnm_dn0 = assign95800_body0_e148372_d_n0;
            locals.var_dnm_dn2 = assign95800_body0_e148372_d_n2;
            locals.var_dnm_dn4 = assign95800_body0_e148372_d_n4;
            locals.var_dnm_dn5 = assign95800_body0_e148372_d_n5;
            locals.var_dnm_dn6 = assign95800_body0_e148372_d_n6;
            locals.var_dnm_dn7 = assign95800_body0_e148372_d_n7;
            locals.var_dnm_dn8 = assign95800_body0_e148372_d_n8;
            locals.var_dnm_dn9 = assign95800_body0_e148372_d_n9;
            locals.var_dnm_dn10 = assign95800_body0_e148372_d_n10;
            locals.var_dnm_dn11 = assign95800_body0_e148372_d_n11;
            locals.var_dnm_dn14 = assign95800_body0_e148372_d_n14;
            let (assign95800_body1_e148382,) = {
    if (((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 != 0.0)) {
        let assign95800_body1_e148380: f64 = (locals.var_m0 + 1.0);
        (assign95800_body1_e148380,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign95800_body1_e148382;
        }

        let (assign95810_e148402, assign95810_e148402_d_n0, assign95810_e148402_d_n2, assign95810_e148402_d_n4, assign95810_e148402_d_n5, assign95810_e148402_d_n6, assign95810_e148402_d_n7, assign95810_e148402_d_n8, assign95810_e148402_d_n9, assign95810_e148402_d_n10, assign95810_e148402_d_n11, assign95810_e148402_d_n14,) = {
    if (((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) && (locals.var_guard2226 == 0.0)) {
        let (assign95810_e148400, assign95810_e148400_d_n0, assign95810_e148400_d_n2, assign95810_e148400_d_n4, assign95810_e148400_d_n5, assign95810_e148400_d_n6, assign95810_e148400_d_n7, assign95810_e148400_d_n8, assign95810_e148400_d_n9, assign95810_e148400_d_n10, assign95810_e148400_d_n11, assign95810_e148400_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign95810_e148397: f64 = (2.0 * 2.0);
                let assign95810_e148398: f64 = (1.0 / assign95810_e148397);
                let assign95810_e148399: f64 = (locals.var_dnm).powf(assign95810_e148398);
                (assign95810_e148399, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn0)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn2)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn4)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn5)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn6)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn7)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn8)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn9)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn10)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn11)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95810_e148398) as f64).is_finite() && ((assign95810_e148398) as f64).fract() == 0.0 { if assign95810_e148398 == 0.0 { 0.0 } else { (assign95810_e148398 * ((locals.var_dnm).powf(assign95810_e148398 - 1.0) * locals.var_dnm_dn14)) } } else { (assign95810_e148399 * (assign95810_e148398 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign95810_e148400, assign95810_e148400_d_n0, assign95810_e148400_d_n2, assign95810_e148400_d_n4, assign95810_e148400_d_n5, assign95810_e148400_d_n6, assign95810_e148400_d_n7, assign95810_e148400_d_n8, assign95810_e148400_d_n9, assign95810_e148400_d_n10, assign95810_e148400_d_n11, assign95810_e148400_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95810_e148402;
        locals.var_dnm_dn0 = assign95810_e148402_d_n0;
        locals.var_dnm_dn2 = assign95810_e148402_d_n2;
        locals.var_dnm_dn4 = assign95810_e148402_d_n4;
        locals.var_dnm_dn5 = assign95810_e148402_d_n5;
        locals.var_dnm_dn6 = assign95810_e148402_d_n6;
        locals.var_dnm_dn7 = assign95810_e148402_d_n7;
        locals.var_dnm_dn8 = assign95810_e148402_d_n8;
        locals.var_dnm_dn9 = assign95810_e148402_d_n9;
        locals.var_dnm_dn10 = assign95810_e148402_d_n10;
        locals.var_dnm_dn11 = assign95810_e148402_d_n11;
        locals.var_dnm_dn14 = assign95810_e148402_d_n14;

        let (assign95820_e148410, assign95820_e148410_d_n0, assign95820_e148410_d_n2, assign95820_e148410_d_n4, assign95820_e148410_d_n5, assign95820_e148410_d_n6, assign95820_e148410_d_n7, assign95820_e148410_d_n8, assign95820_e148410_d_n9, assign95820_e148410_d_n10, assign95820_e148410_d_n11, assign95820_e148410_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95820_e148408: f64 = (1.0 / locals.var_dnm);
        (assign95820_e148408, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95820_e148410;
        locals.var_dnm_dn0 = assign95820_e148410_d_n0;
        locals.var_dnm_dn2 = assign95820_e148410_d_n2;
        locals.var_dnm_dn4 = assign95820_e148410_d_n4;
        locals.var_dnm_dn5 = assign95820_e148410_d_n5;
        locals.var_dnm_dn6 = assign95820_e148410_d_n6;
        locals.var_dnm_dn7 = assign95820_e148410_d_n7;
        locals.var_dnm_dn8 = assign95820_e148410_d_n8;
        locals.var_dnm_dn9 = assign95820_e148410_d_n9;
        locals.var_dnm_dn10 = assign95820_e148410_d_n10;
        locals.var_dnm_dn11 = assign95820_e148410_d_n11;
        locals.var_dnm_dn14 = assign95820_e148410_d_n14;

        let (assign95830_e148422, assign95830_e148422_d_n0, assign95830_e148422_d_n2, assign95830_e148422_d_n4, assign95830_e148422_d_n5, assign95830_e148422_d_n6, assign95830_e148422_d_n7, assign95830_e148422_d_n8, assign95830_e148422_d_n9, assign95830_e148422_d_n10, assign95830_e148422_d_n11, assign95830_e148422_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95830_e148417: f64 = (10.0 * 2.220446049250313e-16);
        let assign95830_e148418: f64 = (locals.var_tmf1 * assign95830_e148417);
        let assign95830_e148420: f64 = (assign95830_e148418 * locals.var_dnm);
        (assign95830_e148420, (((locals.var_tmf1_dn0 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign95830_e148417) * locals.var_dnm) + (assign95830_e148418 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign95830_e148422;
        locals.var_tmf0_dn0 = assign95830_e148422_d_n0;
        locals.var_tmf0_dn2 = assign95830_e148422_d_n2;
        locals.var_tmf0_dn4 = assign95830_e148422_d_n4;
        locals.var_tmf0_dn5 = assign95830_e148422_d_n5;
        locals.var_tmf0_dn6 = assign95830_e148422_d_n6;
        locals.var_tmf0_dn7 = assign95830_e148422_d_n7;
        locals.var_tmf0_dn8 = assign95830_e148422_d_n8;
        locals.var_tmf0_dn9 = assign95830_e148422_d_n9;
        locals.var_tmf0_dn10 = assign95830_e148422_d_n10;
        locals.var_tmf0_dn11 = assign95830_e148422_d_n11;
        locals.var_tmf0_dn14 = assign95830_e148422_d_n14;

        let (assign95840_e148436, assign95840_e148436_d_n0, assign95840_e148436_d_n2, assign95840_e148436_d_n4, assign95840_e148436_d_n5, assign95840_e148436_d_n6, assign95840_e148436_d_n7, assign95840_e148436_d_n8, assign95840_e148436_d_n9, assign95840_e148436_d_n10, assign95840_e148436_d_n11, assign95840_e148436_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95840_e148428: f64 = (10.0 * 2.220446049250313e-16);
        let assign95840_e148430: f64 = (assign95840_e148428 * locals.var_xmp);
        let assign95840_e148432: f64 = (assign95840_e148430 * locals.var_dnm);
        let assign95840_e148434: f64 = (assign95840_e148432 / locals.var_arg);
        (assign95840_e148434, ((((((assign95840_e148428 * locals.var_xmp_dn0) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn0)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn2) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn2)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn4) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn4)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn5) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn5)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn6) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn6)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn7) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn7)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn8) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn8)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn9) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn9)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn10) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn10)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn11) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn11)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign95840_e148428 * locals.var_xmp_dn14) * locals.var_dnm) + (assign95840_e148430 * locals.var_dnm_dn14)) * locals.var_arg) - (assign95840_e148432 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95840_e148436;
        locals.var_t0_dn0 = assign95840_e148436_d_n0;
        locals.var_t0_dn2 = assign95840_e148436_d_n2;
        locals.var_t0_dn4 = assign95840_e148436_d_n4;
        locals.var_t0_dn5 = assign95840_e148436_d_n5;
        locals.var_t0_dn6 = assign95840_e148436_d_n6;
        locals.var_t0_dn7 = assign95840_e148436_d_n7;
        locals.var_t0_dn8 = assign95840_e148436_d_n8;
        locals.var_t0_dn9 = assign95840_e148436_d_n9;
        locals.var_t0_dn10 = assign95840_e148436_d_n10;
        locals.var_t0_dn11 = assign95840_e148436_d_n11;
        locals.var_t0_dn14 = assign95840_e148436_d_n14;

        let (assign95850_e148454, assign95850_e148454_d_n0, assign95850_e148454_d_n2, assign95850_e148454_d_n4, assign95850_e148454_d_n5, assign95850_e148454_d_n6, assign95850_e148454_d_n7, assign95850_e148454_d_n8, assign95850_e148454_d_n9, assign95850_e148454_d_n10, assign95850_e148454_d_n11, assign95850_e148454_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        let assign95850_e148442: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95850_e148445: f64 = (10.0 * 2.220446049250313e-16);
        let assign95850_e148446: f64 = (assign95850_e148442 - assign95850_e148445);
        let assign95850_e148449: f64 = (10.0 * 2.220446049250313e-16);
        let assign95850_e148450: f64 = (assign95850_e148446 - assign95850_e148449);
        let assign95850_e148452: f64 = (assign95850_e148450 + locals.var_tmf0);
        (assign95850_e148452, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn11 + locals.var_vds_dn11) + locals.var_tmf0_dn11), ((locals.var_ps0_dn14 + locals.var_vds_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95850_e148454;
        locals.var_psdl_dn0 = assign95850_e148454_d_n0;
        locals.var_psdl_dn2 = assign95850_e148454_d_n2;
        locals.var_psdl_dn4 = assign95850_e148454_d_n4;
        locals.var_psdl_dn5 = assign95850_e148454_d_n5;
        locals.var_psdl_dn6 = assign95850_e148454_d_n6;
        locals.var_psdl_dn7 = assign95850_e148454_d_n7;
        locals.var_psdl_dn8 = assign95850_e148454_d_n8;
        locals.var_psdl_dn9 = assign95850_e148454_d_n9;
        locals.var_psdl_dn10 = assign95850_e148454_d_n10;
        locals.var_psdl_dn11 = assign95850_e148454_d_n11;
        locals.var_psdl_dn14 = assign95850_e148454_d_n14;

        let (assign95860_e148460, assign95860_e148460_d_n0, assign95860_e148460_d_n2, assign95860_e148460_d_n4, assign95860_e148460_d_n5, assign95860_e148460_d_n6, assign95860_e148460_d_n7, assign95860_e148460_d_n8, assign95860_e148460_d_n9, assign95860_e148460_d_n10, assign95860_e148460_d_n11, assign95860_e148460_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95860_e148460;
        locals.var_t0_dn0 = assign95860_e148460_d_n0;
        locals.var_t0_dn2 = assign95860_e148460_d_n2;
        locals.var_t0_dn4 = assign95860_e148460_d_n4;
        locals.var_t0_dn5 = assign95860_e148460_d_n5;
        locals.var_t0_dn6 = assign95860_e148460_d_n6;
        locals.var_t0_dn7 = assign95860_e148460_d_n7;
        locals.var_t0_dn8 = assign95860_e148460_d_n8;
        locals.var_t0_dn9 = assign95860_e148460_d_n9;
        locals.var_t0_dn10 = assign95860_e148460_d_n10;
        locals.var_t0_dn11 = assign95860_e148460_d_n11;
        locals.var_t0_dn14 = assign95860_e148460_d_n14;

        let (assign95870_e148467, assign95870_e148467_d_n0, assign95870_e148467_d_n2, assign95870_e148467_d_n4, assign95870_e148467_d_n5, assign95870_e148467_d_n6, assign95870_e148467_d_n7, assign95870_e148467_d_n8, assign95870_e148467_d_n9, assign95870_e148467_d_n10, assign95870_e148467_d_n11, assign95870_e148467_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95870_e148467;
        locals.var_psdl_dn0 = assign95870_e148467_d_n0;
        locals.var_psdl_dn2 = assign95870_e148467_d_n2;
        locals.var_psdl_dn4 = assign95870_e148467_d_n4;
        locals.var_psdl_dn5 = assign95870_e148467_d_n5;
        locals.var_psdl_dn6 = assign95870_e148467_d_n6;
        locals.var_psdl_dn7 = assign95870_e148467_d_n7;
        locals.var_psdl_dn8 = assign95870_e148467_d_n8;
        locals.var_psdl_dn9 = assign95870_e148467_d_n9;
        locals.var_psdl_dn10 = assign95870_e148467_d_n10;
        locals.var_psdl_dn11 = assign95870_e148467_d_n11;
        locals.var_psdl_dn14 = assign95870_e148467_d_n14;

        let (assign95880_e148474, assign95880_e148474_d_n0, assign95880_e148474_d_n2, assign95880_e148474_d_n4, assign95880_e148474_d_n5, assign95880_e148474_d_n6, assign95880_e148474_d_n7, assign95880_e148474_d_n8, assign95880_e148474_d_n9, assign95880_e148474_d_n10, assign95880_e148474_d_n11, assign95880_e148474_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_guard2225 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95880_e148474;
        locals.var_t0_dn0 = assign95880_e148474_d_n0;
        locals.var_t0_dn2 = assign95880_e148474_d_n2;
        locals.var_t0_dn4 = assign95880_e148474_d_n4;
        locals.var_t0_dn5 = assign95880_e148474_d_n5;
        locals.var_t0_dn6 = assign95880_e148474_d_n6;
        locals.var_t0_dn7 = assign95880_e148474_d_n7;
        locals.var_t0_dn8 = assign95880_e148474_d_n8;
        locals.var_t0_dn9 = assign95880_e148474_d_n9;
        locals.var_t0_dn10 = assign95880_e148474_d_n10;
        locals.var_t0_dn11 = assign95880_e148474_d_n11;
        locals.var_t0_dn14 = assign95880_e148474_d_n14;

        let (assign95890_e148480, assign95890_e148480_d_n0, assign95890_e148480_d_n2, assign95890_e148480_d_n4, assign95890_e148480_d_n5, assign95890_e148480_d_n6, assign95890_e148480_d_n7, assign95890_e148480_d_n8, assign95890_e148480_d_n9, assign95890_e148480_d_n10, assign95890_e148480_d_n11, assign95890_e148480_d_n14,) = {
    if ((locals.var_guard2224 != 0.0) && (locals.var_flg_qy != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95890_e148480;
        locals.var_ec_dn0 = assign95890_e148480_d_n0;
        locals.var_ec_dn2 = assign95890_e148480_d_n2;
        locals.var_ec_dn4 = assign95890_e148480_d_n4;
        locals.var_ec_dn5 = assign95890_e148480_d_n5;
        locals.var_ec_dn6 = assign95890_e148480_d_n6;
        locals.var_ec_dn7 = assign95890_e148480_d_n7;
        locals.var_ec_dn8 = assign95890_e148480_d_n8;
        locals.var_ec_dn9 = assign95890_e148480_d_n9;
        locals.var_ec_dn10 = assign95890_e148480_d_n10;
        locals.var_ec_dn11 = assign95890_e148480_d_n11;
        locals.var_ec_dn14 = assign95890_e148480_d_n14;

        let assign95900_e148487: f64 = if ((locals.var_idd < 1e-15) || (locals.var_vdseff < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard2231 = assign95900_e148487;

        let (assign95910_e148496, assign95910_e148496_d_n0, assign95910_e148496_d_n2, assign95910_e148496_d_n4, assign95910_e148496_d_n5, assign95910_e148496_d_n6, assign95910_e148496_d_n7, assign95910_e148496_d_n8, assign95910_e148496_d_n9, assign95910_e148496_d_n10, assign95910_e148496_d_n11, assign95910_e148496_d_n14,) = {
    if (((locals.var_guard2224 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2231 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95910_e148496;
        locals.var_ec_dn0 = assign95910_e148496_d_n0;
        locals.var_ec_dn2 = assign95910_e148496_d_n2;
        locals.var_ec_dn4 = assign95910_e148496_d_n4;
        locals.var_ec_dn5 = assign95910_e148496_d_n5;
        locals.var_ec_dn6 = assign95910_e148496_d_n6;
        locals.var_ec_dn7 = assign95910_e148496_d_n7;
        locals.var_ec_dn8 = assign95910_e148496_d_n8;
        locals.var_ec_dn9 = assign95910_e148496_d_n9;
        locals.var_ec_dn10 = assign95910_e148496_d_n10;
        locals.var_ec_dn11 = assign95910_e148496_d_n11;
        locals.var_ec_dn14 = assign95910_e148496_d_n14;

        let (assign95920_e148512, assign95920_e148512_d_n0, assign95920_e148512_d_n2, assign95920_e148512_d_n4, assign95920_e148512_d_n5, assign95920_e148512_d_n6, assign95920_e148512_d_n7, assign95920_e148512_d_n8, assign95920_e148512_d_n9, assign95920_e148512_d_n10, assign95920_e148512_d_n11, assign95920_e148512_d_n14,) = {
    if (((locals.var_guard2224 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2231 == 0.0)) {
        let assign95920_e148506: f64 = (locals.var_idd / locals.var_qn0);
        let assign95920_e148508: f64 = (assign95920_e148506 * locals.var_beta_inv);
        let assign95920_e148510: f64 = (assign95920_e148508 / locals.var_leff);
        (assign95920_e148510, ((((((locals.var_idd_dn0 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn0)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn0)) / locals.var_leff), ((((((locals.var_idd_dn2 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn2)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn2)) / locals.var_leff), ((((((locals.var_idd_dn4 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn4)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn4)) / locals.var_leff), ((((((locals.var_idd_dn5 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn5)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn5)) / locals.var_leff), ((((((locals.var_idd_dn6 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn6)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn6)) / locals.var_leff), ((((((locals.var_idd_dn7 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn7)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn7)) / locals.var_leff), ((((((locals.var_idd_dn8 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn8)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn8)) / locals.var_leff), ((((((locals.var_idd_dn9 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn9)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn9)) / locals.var_leff), ((((((locals.var_idd_dn10 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn10)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn10)) / locals.var_leff), ((((((locals.var_idd_dn11 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn11)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn11)) / locals.var_leff), ((((((locals.var_idd_dn14 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn14)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95920_e148506 * locals.var_beta_inv_dn14)) / locals.var_leff),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95920_e148512;
        locals.var_ec_dn0 = assign95920_e148512_d_n0;
        locals.var_ec_dn2 = assign95920_e148512_d_n2;
        locals.var_ec_dn4 = assign95920_e148512_d_n4;
        locals.var_ec_dn5 = assign95920_e148512_d_n5;
        locals.var_ec_dn6 = assign95920_e148512_d_n6;
        locals.var_ec_dn7 = assign95920_e148512_d_n7;
        locals.var_ec_dn8 = assign95920_e148512_d_n8;
        locals.var_ec_dn9 = assign95920_e148512_d_n9;
        locals.var_ec_dn10 = assign95920_e148512_d_n10;
        locals.var_ec_dn11 = assign95920_e148512_d_n11;
        locals.var_ec_dn14 = assign95920_e148512_d_n14;

        let assign95930_e148515: f64 = if locals.var_flg_qy == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2232 = assign95930_e148515;

        let (assign95940_e148519, assign95940_e148519_d_n0, assign95940_e148519_d_n2, assign95940_e148519_d_n4, assign95940_e148519_d_n5, assign95940_e148519_d_n6, assign95940_e148519_d_n7, assign95940_e148519_d_n8, assign95940_e148519_d_n9, assign95940_e148519_d_n10, assign95940_e148519_d_n11, assign95940_e148519_d_n14,) = {
    if (locals.var_guard2232 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign95940_e148519;
        locals.var_qy_dn0 = assign95940_e148519_d_n0;
        locals.var_qy_dn2 = assign95940_e148519_d_n2;
        locals.var_qy_dn4 = assign95940_e148519_d_n4;
        locals.var_qy_dn5 = assign95940_e148519_d_n5;
        locals.var_qy_dn6 = assign95940_e148519_d_n6;
        locals.var_qy_dn7 = assign95940_e148519_d_n7;
        locals.var_qy_dn8 = assign95940_e148519_d_n8;
        locals.var_qy_dn9 = assign95940_e148519_d_n9;
        locals.var_qy_dn10 = assign95940_e148519_d_n10;
        locals.var_qy_dn11 = assign95940_e148519_d_n11;
        locals.var_qy_dn14 = assign95940_e148519_d_n14;

        let (assign95950_e148530, assign95950_e148530_d_n0, assign95950_e148530_d_n2, assign95950_e148530_d_n4, assign95950_e148530_d_n5, assign95950_e148530_d_n6, assign95950_e148530_d_n7, assign95950_e148530_d_n8, assign95950_e148530_d_n9, assign95950_e148530_d_n10, assign95950_e148530_d_n11, assign95950_e148530_d_n14,) = {
    if (locals.var_guard2232 == 0.0) {
        let assign95950_e148524: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign95950_e148526: f64 = (assign95950_e148524 * locals.var_wdpl);
        let assign95950_e148528: f64 = (assign95950_e148526 * 1.3);
        (assign95950_e148528, ((assign95950_e148524 * locals.var_wdpl_dn0) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn2) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn4) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn5) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn6) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn7) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn8) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn9) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn10) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn11) * 1.3), ((assign95950_e148524 * locals.var_wdpl_dn14) * 1.3),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign95950_e148530;
        locals.var_t2_dn0 = assign95950_e148530_d_n0;
        locals.var_t2_dn2 = assign95950_e148530_d_n2;
        locals.var_t2_dn4 = assign95950_e148530_d_n4;
        locals.var_t2_dn5 = assign95950_e148530_d_n5;
        locals.var_t2_dn6 = assign95950_e148530_d_n6;
        locals.var_t2_dn7 = assign95950_e148530_d_n7;
        locals.var_t2_dn8 = assign95950_e148530_d_n8;
        locals.var_t2_dn9 = assign95950_e148530_d_n9;
        locals.var_t2_dn10 = assign95950_e148530_d_n10;
        locals.var_t2_dn11 = assign95950_e148530_d_n11;
        locals.var_t2_dn14 = assign95950_e148530_d_n14;

        let assign95960_e148533: f64 = if p.p133 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2233 = assign95960_e148533;

    }

    pub(super) fn stamp_transient_block_354(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95970_e148544, assign95970_e148544_d_n0, assign95970_e148544_d_n2, assign95970_e148544_d_n4, assign95970_e148544_d_n5, assign95970_e148544_d_n6, assign95970_e148544_d_n7, assign95970_e148544_d_n8, assign95970_e148544_d_n9, assign95970_e148544_d_n10, assign95970_e148544_d_n11, assign95970_e148544_d_n14,) = {
    if ((locals.var_guard2232 == 0.0) && (locals.var_guard2233 != 0.0)) {
        let assign95970_e148540: f64 = (locals.var_ec * locals.var_leff);
        let assign95970_e148542: f64 = (assign95970_e148540 + locals.var_ps0);
        (assign95970_e148542, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn4 * locals.var_leff) + locals.var_ps0_dn4), ((locals.var_ec_dn5 * locals.var_leff) + locals.var_ps0_dn5), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn8 * locals.var_leff) + locals.var_ps0_dn8), ((locals.var_ec_dn9 * locals.var_leff) + locals.var_ps0_dn9), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn14 * locals.var_leff) + locals.var_ps0_dn14),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn8, locals.var_pslk_dn9, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn14,)
    }
};
        locals.var_pslk = assign95970_e148544;
        locals.var_pslk_dn0 = assign95970_e148544_d_n0;
        locals.var_pslk_dn2 = assign95970_e148544_d_n2;
        locals.var_pslk_dn4 = assign95970_e148544_d_n4;
        locals.var_pslk_dn5 = assign95970_e148544_d_n5;
        locals.var_pslk_dn6 = assign95970_e148544_d_n6;
        locals.var_pslk_dn7 = assign95970_e148544_d_n7;
        locals.var_pslk_dn8 = assign95970_e148544_d_n8;
        locals.var_pslk_dn9 = assign95970_e148544_d_n9;
        locals.var_pslk_dn10 = assign95970_e148544_d_n10;
        locals.var_pslk_dn11 = assign95970_e148544_d_n11;
        locals.var_pslk_dn14 = assign95970_e148544_d_n14;

        let (assign95980_e148561, assign95980_e148561_d_n0, assign95980_e148561_d_n2, assign95980_e148561_d_n4, assign95980_e148561_d_n5, assign95980_e148561_d_n6, assign95980_e148561_d_n7, assign95980_e148561_d_n8, assign95980_e148561_d_n9, assign95980_e148561_d_n10, assign95980_e148561_d_n11, assign95980_e148561_d_n14,) = {
    if ((locals.var_guard2232 == 0.0) && (locals.var_guard2233 != 0.0)) {
        let assign95980_e148552: f64 = (locals.var_vdsz__blk441 + locals.var_ps0);
        let assign95980_e148553: f64 = (locals.var_aclm * assign95980_e148552);
        let assign95980_e148556: f64 = (1.0 - locals.var_aclm);
        let assign95980_e148558: f64 = (assign95980_e148556 * locals.var_pslk);
        let assign95980_e148559: f64 = (assign95980_e148553 + assign95980_e148558);
        (assign95980_e148559, ((locals.var_aclm * (locals.var_vdsz__blk441_dn0 + locals.var_ps0_dn0)) + (assign95980_e148556 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn2 + locals.var_ps0_dn2)) + (assign95980_e148556 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn4 + locals.var_ps0_dn4)) + (assign95980_e148556 * locals.var_pslk_dn4)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn5 + locals.var_ps0_dn5)) + (assign95980_e148556 * locals.var_pslk_dn5)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn6 + locals.var_ps0_dn6)) + (assign95980_e148556 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn7 + locals.var_ps0_dn7)) + (assign95980_e148556 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn8 + locals.var_ps0_dn8)) + (assign95980_e148556 * locals.var_pslk_dn8)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn9 + locals.var_ps0_dn9)) + (assign95980_e148556 * locals.var_pslk_dn9)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn10 + locals.var_ps0_dn10)) + (assign95980_e148556 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn11 + locals.var_ps0_dn11)) + (assign95980_e148556 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vdsz__blk441_dn14 + locals.var_ps0_dn14)) + (assign95980_e148556 * locals.var_pslk_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign95980_e148561;
        locals.var_t1_dn0 = assign95980_e148561_d_n0;
        locals.var_t1_dn2 = assign95980_e148561_d_n2;
        locals.var_t1_dn4 = assign95980_e148561_d_n4;
        locals.var_t1_dn5 = assign95980_e148561_d_n5;
        locals.var_t1_dn6 = assign95980_e148561_d_n6;
        locals.var_t1_dn7 = assign95980_e148561_d_n7;
        locals.var_t1_dn8 = assign95980_e148561_d_n8;
        locals.var_t1_dn9 = assign95980_e148561_d_n9;
        locals.var_t1_dn10 = assign95980_e148561_d_n10;
        locals.var_t1_dn11 = assign95980_e148561_d_n11;
        locals.var_t1_dn14 = assign95980_e148561_d_n14;

        let (assign95990_e148577, assign95990_e148577_d_n0, assign95990_e148577_d_n2, assign95990_e148577_d_n4, assign95990_e148577_d_n5, assign95990_e148577_d_n6, assign95990_e148577_d_n7, assign95990_e148577_d_n8, assign95990_e148577_d_n9, assign95990_e148577_d_n10, assign95990_e148577_d_n11, assign95990_e148577_d_n14,) = {
    if ((locals.var_guard2232 == 0.0) && (locals.var_guard2233 != 0.0)) {
        let assign95990_e148568: f64 = (locals.var_ps0 + locals.var_vdsz__blk441);
        let assign95990_e148570: f64 = (assign95990_e148568 - locals.var_t1);
        let assign95990_e148572: f64 = (assign95990_e148570 / p.p133);
        let assign95990_e148573: f64 = (-assign95990_e148572);
        let assign95990_e148575: f64 = (assign95990_e148573 * locals.var_t2);
        (assign95990_e148575, (((-(((locals.var_ps0_dn0 + locals.var_vdsz__blk441_dn0) - locals.var_t1_dn0) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn0)), (((-(((locals.var_ps0_dn2 + locals.var_vdsz__blk441_dn2) - locals.var_t1_dn2) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn2)), (((-(((locals.var_ps0_dn4 + locals.var_vdsz__blk441_dn4) - locals.var_t1_dn4) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn4)), (((-(((locals.var_ps0_dn5 + locals.var_vdsz__blk441_dn5) - locals.var_t1_dn5) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn5)), (((-(((locals.var_ps0_dn6 + locals.var_vdsz__blk441_dn6) - locals.var_t1_dn6) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn6)), (((-(((locals.var_ps0_dn7 + locals.var_vdsz__blk441_dn7) - locals.var_t1_dn7) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn7)), (((-(((locals.var_ps0_dn8 + locals.var_vdsz__blk441_dn8) - locals.var_t1_dn8) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn8)), (((-(((locals.var_ps0_dn9 + locals.var_vdsz__blk441_dn9) - locals.var_t1_dn9) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn9)), (((-(((locals.var_ps0_dn10 + locals.var_vdsz__blk441_dn10) - locals.var_t1_dn10) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn10)), (((-(((locals.var_ps0_dn11 + locals.var_vdsz__blk441_dn11) - locals.var_t1_dn11) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn11)), (((-(((locals.var_ps0_dn14 + locals.var_vdsz__blk441_dn14) - locals.var_t1_dn14) / p.p133)) * locals.var_t2) + (assign95990_e148573 * locals.var_t2_dn14)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign95990_e148577;
        locals.var_qy_dn0 = assign95990_e148577_d_n0;
        locals.var_qy_dn2 = assign95990_e148577_d_n2;
        locals.var_qy_dn4 = assign95990_e148577_d_n4;
        locals.var_qy_dn5 = assign95990_e148577_d_n5;
        locals.var_qy_dn6 = assign95990_e148577_d_n6;
        locals.var_qy_dn7 = assign95990_e148577_d_n7;
        locals.var_qy_dn8 = assign95990_e148577_d_n8;
        locals.var_qy_dn9 = assign95990_e148577_d_n9;
        locals.var_qy_dn10 = assign95990_e148577_d_n10;
        locals.var_qy_dn11 = assign95990_e148577_d_n11;
        locals.var_qy_dn14 = assign95990_e148577_d_n14;

        let assign96000_e148580: f64 = if p.p134 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2234 = assign96000_e148580;

        let (assign96010_e148591, assign96010_e148591_d_n0, assign96010_e148591_d_n2, assign96010_e148591_d_n4, assign96010_e148591_d_n5, assign96010_e148591_d_n6, assign96010_e148591_d_n7, assign96010_e148591_d_n8, assign96010_e148591_d_n9, assign96010_e148591_d_n10, assign96010_e148591_d_n11, assign96010_e148591_d_n14,) = {
    if ((locals.var_guard2232 == 0.0) && (locals.var_guard2234 != 0.0)) {
        let assign96010_e148588: f64 = (locals.var_cqyb0 * locals.var_vbs);
        let assign96010_e148589: f64 = (locals.var_qy + assign96010_e148588);
        (assign96010_e148589, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbs_dn6)), locals.var_qy_dn7, (locals.var_qy_dn8 + (locals.var_cqyb0 * locals.var_vbs_dn8)), (locals.var_qy_dn9 + (locals.var_cqyb0 * locals.var_vbs_dn9)), locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign96010_e148591;
        locals.var_qy_dn0 = assign96010_e148591_d_n0;
        locals.var_qy_dn2 = assign96010_e148591_d_n2;
        locals.var_qy_dn4 = assign96010_e148591_d_n4;
        locals.var_qy_dn5 = assign96010_e148591_d_n5;
        locals.var_qy_dn6 = assign96010_e148591_d_n6;
        locals.var_qy_dn7 = assign96010_e148591_d_n7;
        locals.var_qy_dn8 = assign96010_e148591_d_n8;
        locals.var_qy_dn9 = assign96010_e148591_d_n9;
        locals.var_qy_dn10 = assign96010_e148591_d_n10;
        locals.var_qy_dn11 = assign96010_e148591_d_n11;
        locals.var_qy_dn14 = assign96010_e148591_d_n14;

        locals.var_cfd = locals.var_cfrng;

        locals.var_cfs = locals.var_cfrng;

        let assign96040_e148597: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign96040_e148598: f64 = (locals.var_cfd * assign96040_e148597);
        locals.var_qfd = assign96040_e148598;
        locals.var_qfd_dn0 = (locals.var_cfd * (-locals.var_vdsei_dn0));
        locals.var_qfd_dn2 = (locals.var_cfd * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qfd_dn7 = (locals.var_cfd * locals.var_vgsei_dn7);

        let assign96050_e148601: f64 = (locals.var_cfs * locals.var_vgsei);
        locals.var_qfs = assign96050_e148601;
        locals.var_qfs_dn2 = (locals.var_cfs * locals.var_vgsei_dn2);
        locals.var_qfs_dn7 = (locals.var_cfs * locals.var_vgsei_dn7);

        let assign96060_e148608: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2235 = assign96060_e148608;

        let (assign96070_e148614, assign96070_e148614_d_n0, assign96070_e148614_d_n2, assign96070_e148614_d_n4, assign96070_e148614_d_n5, assign96070_e148614_d_n6, assign96070_e148614_d_n7, assign96070_e148614_d_n8, assign96070_e148614_d_n9, assign96070_e148614_d_n10, assign96070_e148614_d_n11, assign96070_e148614_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96070_e148612: f64 = (locals.var_tratio * locals.var_tratio);
        (assign96070_e148612, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn11 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn11)), ((locals.var_tratio_dn14 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign96070_e148614;
        locals.var_t0_dn0 = assign96070_e148614_d_n0;
        locals.var_t0_dn2 = assign96070_e148614_d_n2;
        locals.var_t0_dn4 = assign96070_e148614_d_n4;
        locals.var_t0_dn5 = assign96070_e148614_d_n5;
        locals.var_t0_dn6 = assign96070_e148614_d_n6;
        locals.var_t0_dn7 = assign96070_e148614_d_n7;
        locals.var_t0_dn8 = assign96070_e148614_d_n8;
        locals.var_t0_dn9 = assign96070_e148614_d_n9;
        locals.var_t0_dn10 = assign96070_e148614_d_n10;
        locals.var_t0_dn11 = assign96070_e148614_d_n11;
        locals.var_t0_dn14 = assign96070_e148614_d_n14;

        let (assign96080_e148633, assign96080_e148633_d_n0, assign96080_e148633_d_n2, assign96080_e148633_d_n4, assign96080_e148633_d_n5, assign96080_e148633_d_n6, assign96080_e148633_d_n7, assign96080_e148633_d_n8, assign96080_e148633_d_n9, assign96080_e148633_d_n10, assign96080_e148633_d_n11, assign96080_e148633_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96080_e148619: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96080_e148622: f64 = (locals.var_eg * locals.var_beta);
        let assign96080_e148623: f64 = (assign96080_e148619 - assign96080_e148622);
        let assign96080_e148626: f64 = (p.p499 * locals.var_log_tratio);
        let assign96080_e148627: f64 = (assign96080_e148623 + assign96080_e148626);
        let assign96080_e148629: f64 = (assign96080_e148627 / locals.var_uc_njd);
        let assign96080_e148630: f64 = (assign96080_e148629).exp();
        let assign96080_e148631: f64 = (locals.var_uc_js0d * assign96080_e148630);
        (assign96080_e148631, (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96080_e148630 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign96080_e148633;
        locals.var_js_dn0 = assign96080_e148633_d_n0;
        locals.var_js_dn2 = assign96080_e148633_d_n2;
        locals.var_js_dn4 = assign96080_e148633_d_n4;
        locals.var_js_dn5 = assign96080_e148633_d_n5;
        locals.var_js_dn6 = assign96080_e148633_d_n6;
        locals.var_js_dn7 = assign96080_e148633_d_n7;
        locals.var_js_dn8 = assign96080_e148633_d_n8;
        locals.var_js_dn9 = assign96080_e148633_d_n9;
        locals.var_js_dn10 = assign96080_e148633_d_n10;
        locals.var_js_dn11 = assign96080_e148633_d_n11;
        locals.var_js_dn14 = assign96080_e148633_d_n14;

        let (assign96090_e148652, assign96090_e148652_d_n0, assign96090_e148652_d_n2, assign96090_e148652_d_n4, assign96090_e148652_d_n5, assign96090_e148652_d_n6, assign96090_e148652_d_n7, assign96090_e148652_d_n8, assign96090_e148652_d_n9, assign96090_e148652_d_n10, assign96090_e148652_d_n11, assign96090_e148652_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96090_e148638: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96090_e148641: f64 = (locals.var_eg * locals.var_beta);
        let assign96090_e148642: f64 = (assign96090_e148638 - assign96090_e148641);
        let assign96090_e148645: f64 = (p.p499 * locals.var_log_tratio);
        let assign96090_e148646: f64 = (assign96090_e148642 + assign96090_e148645);
        let assign96090_e148648: f64 = (assign96090_e148646 / p.p497);
        let assign96090_e148649: f64 = (assign96090_e148648).exp();
        let assign96090_e148650: f64 = (locals.var_uc_js0swd * assign96090_e148649);
        (assign96090_e148650, (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign96090_e148649 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign96090_e148652;
        locals.var_jssw_dn0 = assign96090_e148652_d_n0;
        locals.var_jssw_dn2 = assign96090_e148652_d_n2;
        locals.var_jssw_dn4 = assign96090_e148652_d_n4;
        locals.var_jssw_dn5 = assign96090_e148652_d_n5;
        locals.var_jssw_dn6 = assign96090_e148652_d_n6;
        locals.var_jssw_dn7 = assign96090_e148652_d_n7;
        locals.var_jssw_dn8 = assign96090_e148652_d_n8;
        locals.var_jssw_dn9 = assign96090_e148652_d_n9;
        locals.var_jssw_dn10 = assign96090_e148652_d_n10;
        locals.var_jssw_dn11 = assign96090_e148652_d_n11;
        locals.var_jssw_dn14 = assign96090_e148652_d_n14;

        let (assign96100_e148671, assign96100_e148671_d_n0, assign96100_e148671_d_n2, assign96100_e148671_d_n4, assign96100_e148671_d_n5, assign96100_e148671_d_n6, assign96100_e148671_d_n7, assign96100_e148671_d_n8, assign96100_e148671_d_n9, assign96100_e148671_d_n10, assign96100_e148671_d_n11, assign96100_e148671_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96100_e148657: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96100_e148660: f64 = (locals.var_eg * locals.var_beta);
        let assign96100_e148661: f64 = (assign96100_e148657 - assign96100_e148660);
        let assign96100_e148664: f64 = (p.p499 * locals.var_log_tratio);
        let assign96100_e148665: f64 = (assign96100_e148661 + assign96100_e148664);
        let assign96100_e148667: f64 = (assign96100_e148665 / p.p498);
        let assign96100_e148668: f64 = (assign96100_e148667).exp();
        let assign96100_e148669: f64 = (p.p495 * assign96100_e148668);
        (assign96100_e148669, (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign96100_e148668 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign96100_e148671;
        locals.var_jsswg_dn0 = assign96100_e148671_d_n0;
        locals.var_jsswg_dn2 = assign96100_e148671_d_n2;
        locals.var_jsswg_dn4 = assign96100_e148671_d_n4;
        locals.var_jsswg_dn5 = assign96100_e148671_d_n5;
        locals.var_jsswg_dn6 = assign96100_e148671_d_n6;
        locals.var_jsswg_dn7 = assign96100_e148671_d_n7;
        locals.var_jsswg_dn8 = assign96100_e148671_d_n8;
        locals.var_jsswg_dn9 = assign96100_e148671_d_n9;
        locals.var_jsswg_dn10 = assign96100_e148671_d_n10;
        locals.var_jsswg_dn11 = assign96100_e148671_d_n11;
        locals.var_jsswg_dn14 = assign96100_e148671_d_n14;

        let (assign96110_e148690, assign96110_e148690_d_n0, assign96110_e148690_d_n2, assign96110_e148690_d_n4, assign96110_e148690_d_n5, assign96110_e148690_d_n6, assign96110_e148690_d_n7, assign96110_e148690_d_n8, assign96110_e148690_d_n9, assign96110_e148690_d_n10, assign96110_e148690_d_n11, assign96110_e148690_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96110_e148676: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96110_e148679: f64 = (locals.var_eg * locals.var_beta);
        let assign96110_e148680: f64 = (assign96110_e148676 - assign96110_e148679);
        let assign96110_e148683: f64 = (p.p509 * locals.var_log_tratio);
        let assign96110_e148684: f64 = (assign96110_e148680 + assign96110_e148683);
        let assign96110_e148686: f64 = (assign96110_e148684 / locals.var_uc_njd);
        let assign96110_e148687: f64 = (assign96110_e148686).exp();
        let assign96110_e148688: f64 = (locals.var_uc_js0d * assign96110_e148687);
        (assign96110_e148688, (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96110_e148687 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign96110_e148690;
        locals.var_js2_dn0 = assign96110_e148690_d_n0;
        locals.var_js2_dn2 = assign96110_e148690_d_n2;
        locals.var_js2_dn4 = assign96110_e148690_d_n4;
        locals.var_js2_dn5 = assign96110_e148690_d_n5;
        locals.var_js2_dn6 = assign96110_e148690_d_n6;
        locals.var_js2_dn7 = assign96110_e148690_d_n7;
        locals.var_js2_dn8 = assign96110_e148690_d_n8;
        locals.var_js2_dn9 = assign96110_e148690_d_n9;
        locals.var_js2_dn10 = assign96110_e148690_d_n10;
        locals.var_js2_dn11 = assign96110_e148690_d_n11;
        locals.var_js2_dn14 = assign96110_e148690_d_n14;

        let (assign96120_e148709, assign96120_e148709_d_n0, assign96120_e148709_d_n2, assign96120_e148709_d_n4, assign96120_e148709_d_n5, assign96120_e148709_d_n6, assign96120_e148709_d_n7, assign96120_e148709_d_n8, assign96120_e148709_d_n9, assign96120_e148709_d_n10, assign96120_e148709_d_n11, assign96120_e148709_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96120_e148695: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96120_e148698: f64 = (locals.var_eg * locals.var_beta);
        let assign96120_e148699: f64 = (assign96120_e148695 - assign96120_e148698);
        let assign96120_e148702: f64 = (p.p509 * locals.var_log_tratio);
        let assign96120_e148703: f64 = (assign96120_e148699 + assign96120_e148702);
        let assign96120_e148705: f64 = (assign96120_e148703 / p.p497);
        let assign96120_e148706: f64 = (assign96120_e148705).exp();
        let assign96120_e148707: f64 = (locals.var_uc_js0swd * assign96120_e148706);
        (assign96120_e148707, (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign96120_e148706 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign96120_e148709;
        locals.var_jssw2_dn0 = assign96120_e148709_d_n0;
        locals.var_jssw2_dn2 = assign96120_e148709_d_n2;
        locals.var_jssw2_dn4 = assign96120_e148709_d_n4;
        locals.var_jssw2_dn5 = assign96120_e148709_d_n5;
        locals.var_jssw2_dn6 = assign96120_e148709_d_n6;
        locals.var_jssw2_dn7 = assign96120_e148709_d_n7;
        locals.var_jssw2_dn8 = assign96120_e148709_d_n8;
        locals.var_jssw2_dn9 = assign96120_e148709_d_n9;
        locals.var_jssw2_dn10 = assign96120_e148709_d_n10;
        locals.var_jssw2_dn11 = assign96120_e148709_d_n11;
        locals.var_jssw2_dn14 = assign96120_e148709_d_n14;

        let (assign96130_e148728, assign96130_e148728_d_n0, assign96130_e148728_d_n2, assign96130_e148728_d_n4, assign96130_e148728_d_n5, assign96130_e148728_d_n6, assign96130_e148728_d_n7, assign96130_e148728_d_n8, assign96130_e148728_d_n9, assign96130_e148728_d_n10, assign96130_e148728_d_n11, assign96130_e148728_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96130_e148714: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96130_e148717: f64 = (locals.var_eg * locals.var_beta);
        let assign96130_e148718: f64 = (assign96130_e148714 - assign96130_e148717);
        let assign96130_e148721: f64 = (p.p509 * locals.var_log_tratio);
        let assign96130_e148722: f64 = (assign96130_e148718 + assign96130_e148721);
        let assign96130_e148724: f64 = (assign96130_e148722 / p.p498);
        let assign96130_e148725: f64 = (assign96130_e148724).exp();
        let assign96130_e148726: f64 = (p.p495 * assign96130_e148725);
        (assign96130_e148726, (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign96130_e148725 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign96130_e148728;
        locals.var_jsswg2_dn0 = assign96130_e148728_d_n0;
        locals.var_jsswg2_dn2 = assign96130_e148728_d_n2;
        locals.var_jsswg2_dn4 = assign96130_e148728_d_n4;
        locals.var_jsswg2_dn5 = assign96130_e148728_d_n5;
        locals.var_jsswg2_dn6 = assign96130_e148728_d_n6;
        locals.var_jsswg2_dn7 = assign96130_e148728_d_n7;
        locals.var_jsswg2_dn8 = assign96130_e148728_d_n8;
        locals.var_jsswg2_dn9 = assign96130_e148728_d_n9;
        locals.var_jsswg2_dn10 = assign96130_e148728_d_n10;
        locals.var_jsswg2_dn11 = assign96130_e148728_d_n11;
        locals.var_jsswg2_dn14 = assign96130_e148728_d_n14;

        let assign96140_e148731: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2236 = assign96140_e148731;

        let assign96150_e148734: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2237 = assign96150_e148734;

        let (assign96160_e148744, assign96160_e148744_d_n0, assign96160_e148744_d_n2, assign96160_e148744_d_n4, assign96160_e148744_d_n5, assign96160_e148744_d_n6, assign96160_e148744_d_n7, assign96160_e148744_d_n8, assign96160_e148744_d_n9, assign96160_e148744_d_n10, assign96160_e148744_d_n11, assign96160_e148744_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96160_e148742: f64 = (p.p13 * locals.var_js);
        (assign96160_e148742, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96160_e148744;
        locals.var_isbd_btm_dn0 = assign96160_e148744_d_n0;
        locals.var_isbd_btm_dn2 = assign96160_e148744_d_n2;
        locals.var_isbd_btm_dn4 = assign96160_e148744_d_n4;
        locals.var_isbd_btm_dn5 = assign96160_e148744_d_n5;
        locals.var_isbd_btm_dn6 = assign96160_e148744_d_n6;
        locals.var_isbd_btm_dn7 = assign96160_e148744_d_n7;
        locals.var_isbd_btm_dn8 = assign96160_e148744_d_n8;
        locals.var_isbd_btm_dn9 = assign96160_e148744_d_n9;
        locals.var_isbd_btm_dn10 = assign96160_e148744_d_n10;
        locals.var_isbd_btm_dn11 = assign96160_e148744_d_n11;
        locals.var_isbd_btm_dn14 = assign96160_e148744_d_n14;

        let (assign96170_e148754, assign96170_e148754_d_n0, assign96170_e148754_d_n2, assign96170_e148754_d_n4, assign96170_e148754_d_n5, assign96170_e148754_d_n6, assign96170_e148754_d_n7, assign96170_e148754_d_n8, assign96170_e148754_d_n9, assign96170_e148754_d_n10, assign96170_e148754_d_n11, assign96170_e148754_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96170_e148752: f64 = (p.p13 * locals.var_js2);
        (assign96170_e148752, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96170_e148754;
        locals.var_isbd2_btm_dn0 = assign96170_e148754_d_n0;
        locals.var_isbd2_btm_dn2 = assign96170_e148754_d_n2;
        locals.var_isbd2_btm_dn4 = assign96170_e148754_d_n4;
        locals.var_isbd2_btm_dn5 = assign96170_e148754_d_n5;
        locals.var_isbd2_btm_dn6 = assign96170_e148754_d_n6;
        locals.var_isbd2_btm_dn7 = assign96170_e148754_d_n7;
        locals.var_isbd2_btm_dn8 = assign96170_e148754_d_n8;
        locals.var_isbd2_btm_dn9 = assign96170_e148754_d_n9;
        locals.var_isbd2_btm_dn10 = assign96170_e148754_d_n10;
        locals.var_isbd2_btm_dn11 = assign96170_e148754_d_n11;
        locals.var_isbd2_btm_dn14 = assign96170_e148754_d_n14;

        let (assign96180_e148766, assign96180_e148766_d_n0, assign96180_e148766_d_n2, assign96180_e148766_d_n4, assign96180_e148766_d_n5, assign96180_e148766_d_n6, assign96180_e148766_d_n7, assign96180_e148766_d_n8, assign96180_e148766_d_n9, assign96180_e148766_d_n10, assign96180_e148766_d_n11, assign96180_e148766_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96180_e148762: f64 = (p.p15 - locals.var_weff_nf);
        let assign96180_e148764: f64 = (assign96180_e148762 * locals.var_jssw);
        (assign96180_e148764, (assign96180_e148762 * locals.var_jssw_dn0), (assign96180_e148762 * locals.var_jssw_dn2), (assign96180_e148762 * locals.var_jssw_dn4), (assign96180_e148762 * locals.var_jssw_dn5), (assign96180_e148762 * locals.var_jssw_dn6), (assign96180_e148762 * locals.var_jssw_dn7), (assign96180_e148762 * locals.var_jssw_dn8), (assign96180_e148762 * locals.var_jssw_dn9), (assign96180_e148762 * locals.var_jssw_dn10), (assign96180_e148762 * locals.var_jssw_dn11), (assign96180_e148762 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96180_e148766;
        locals.var_isbd_sws_dn0 = assign96180_e148766_d_n0;
        locals.var_isbd_sws_dn2 = assign96180_e148766_d_n2;
        locals.var_isbd_sws_dn4 = assign96180_e148766_d_n4;
        locals.var_isbd_sws_dn5 = assign96180_e148766_d_n5;
        locals.var_isbd_sws_dn6 = assign96180_e148766_d_n6;
        locals.var_isbd_sws_dn7 = assign96180_e148766_d_n7;
        locals.var_isbd_sws_dn8 = assign96180_e148766_d_n8;
        locals.var_isbd_sws_dn9 = assign96180_e148766_d_n9;
        locals.var_isbd_sws_dn10 = assign96180_e148766_d_n10;
        locals.var_isbd_sws_dn11 = assign96180_e148766_d_n11;
        locals.var_isbd_sws_dn14 = assign96180_e148766_d_n14;

        let (assign96190_e148778, assign96190_e148778_d_n0, assign96190_e148778_d_n2, assign96190_e148778_d_n4, assign96190_e148778_d_n5, assign96190_e148778_d_n6, assign96190_e148778_d_n7, assign96190_e148778_d_n8, assign96190_e148778_d_n9, assign96190_e148778_d_n10, assign96190_e148778_d_n11, assign96190_e148778_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96190_e148774: f64 = (p.p15 - locals.var_weff_nf);
        let assign96190_e148776: f64 = (assign96190_e148774 * locals.var_jssw2);
        (assign96190_e148776, (assign96190_e148774 * locals.var_jssw2_dn0), (assign96190_e148774 * locals.var_jssw2_dn2), (assign96190_e148774 * locals.var_jssw2_dn4), (assign96190_e148774 * locals.var_jssw2_dn5), (assign96190_e148774 * locals.var_jssw2_dn6), (assign96190_e148774 * locals.var_jssw2_dn7), (assign96190_e148774 * locals.var_jssw2_dn8), (assign96190_e148774 * locals.var_jssw2_dn9), (assign96190_e148774 * locals.var_jssw2_dn10), (assign96190_e148774 * locals.var_jssw2_dn11), (assign96190_e148774 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96190_e148778;
        locals.var_isbd2_sws_dn0 = assign96190_e148778_d_n0;
        locals.var_isbd2_sws_dn2 = assign96190_e148778_d_n2;
        locals.var_isbd2_sws_dn4 = assign96190_e148778_d_n4;
        locals.var_isbd2_sws_dn5 = assign96190_e148778_d_n5;
        locals.var_isbd2_sws_dn6 = assign96190_e148778_d_n6;
        locals.var_isbd2_sws_dn7 = assign96190_e148778_d_n7;
        locals.var_isbd2_sws_dn8 = assign96190_e148778_d_n8;
        locals.var_isbd2_sws_dn9 = assign96190_e148778_d_n9;
        locals.var_isbd2_sws_dn10 = assign96190_e148778_d_n10;
        locals.var_isbd2_sws_dn11 = assign96190_e148778_d_n11;
        locals.var_isbd2_sws_dn14 = assign96190_e148778_d_n14;

        let (assign96200_e148788, assign96200_e148788_d_n0, assign96200_e148788_d_n2, assign96200_e148788_d_n4, assign96200_e148788_d_n5, assign96200_e148788_d_n6, assign96200_e148788_d_n7, assign96200_e148788_d_n8, assign96200_e148788_d_n9, assign96200_e148788_d_n10, assign96200_e148788_d_n11, assign96200_e148788_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96200_e148786: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96200_e148786, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96200_e148788;
        locals.var_isbd_swg_dn0 = assign96200_e148788_d_n0;
        locals.var_isbd_swg_dn2 = assign96200_e148788_d_n2;
        locals.var_isbd_swg_dn4 = assign96200_e148788_d_n4;
        locals.var_isbd_swg_dn5 = assign96200_e148788_d_n5;
        locals.var_isbd_swg_dn6 = assign96200_e148788_d_n6;
        locals.var_isbd_swg_dn7 = assign96200_e148788_d_n7;
        locals.var_isbd_swg_dn8 = assign96200_e148788_d_n8;
        locals.var_isbd_swg_dn9 = assign96200_e148788_d_n9;
        locals.var_isbd_swg_dn10 = assign96200_e148788_d_n10;
        locals.var_isbd_swg_dn11 = assign96200_e148788_d_n11;
        locals.var_isbd_swg_dn14 = assign96200_e148788_d_n14;

        let (assign96210_e148798, assign96210_e148798_d_n0, assign96210_e148798_d_n2, assign96210_e148798_d_n4, assign96210_e148798_d_n5, assign96210_e148798_d_n6, assign96210_e148798_d_n7, assign96210_e148798_d_n8, assign96210_e148798_d_n9, assign96210_e148798_d_n10, assign96210_e148798_d_n11, assign96210_e148798_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign96210_e148796: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96210_e148796, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96210_e148798;
        locals.var_isbd2_swg_dn0 = assign96210_e148798_d_n0;
        locals.var_isbd2_swg_dn2 = assign96210_e148798_d_n2;
        locals.var_isbd2_swg_dn4 = assign96210_e148798_d_n4;
        locals.var_isbd2_swg_dn5 = assign96210_e148798_d_n5;
        locals.var_isbd2_swg_dn6 = assign96210_e148798_d_n6;
        locals.var_isbd2_swg_dn7 = assign96210_e148798_d_n7;
        locals.var_isbd2_swg_dn8 = assign96210_e148798_d_n8;
        locals.var_isbd2_swg_dn9 = assign96210_e148798_d_n9;
        locals.var_isbd2_swg_dn10 = assign96210_e148798_d_n10;
        locals.var_isbd2_swg_dn11 = assign96210_e148798_d_n11;
        locals.var_isbd2_swg_dn14 = assign96210_e148798_d_n14;

        let (assign96220_e148809, assign96220_e148809_d_n0, assign96220_e148809_d_n2, assign96220_e148809_d_n4, assign96220_e148809_d_n5, assign96220_e148809_d_n6, assign96220_e148809_d_n7, assign96220_e148809_d_n8, assign96220_e148809_d_n9, assign96220_e148809_d_n10, assign96220_e148809_d_n11, assign96220_e148809_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign96220_e148807: f64 = (p.p13 * locals.var_js);
        (assign96220_e148807, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96220_e148809;
        locals.var_isbd_btm_dn0 = assign96220_e148809_d_n0;
        locals.var_isbd_btm_dn2 = assign96220_e148809_d_n2;
        locals.var_isbd_btm_dn4 = assign96220_e148809_d_n4;
        locals.var_isbd_btm_dn5 = assign96220_e148809_d_n5;
        locals.var_isbd_btm_dn6 = assign96220_e148809_d_n6;
        locals.var_isbd_btm_dn7 = assign96220_e148809_d_n7;
        locals.var_isbd_btm_dn8 = assign96220_e148809_d_n8;
        locals.var_isbd_btm_dn9 = assign96220_e148809_d_n9;
        locals.var_isbd_btm_dn10 = assign96220_e148809_d_n10;
        locals.var_isbd_btm_dn11 = assign96220_e148809_d_n11;
        locals.var_isbd_btm_dn14 = assign96220_e148809_d_n14;

        let (assign96230_e148820, assign96230_e148820_d_n0, assign96230_e148820_d_n2, assign96230_e148820_d_n4, assign96230_e148820_d_n5, assign96230_e148820_d_n6, assign96230_e148820_d_n7, assign96230_e148820_d_n8, assign96230_e148820_d_n9, assign96230_e148820_d_n10, assign96230_e148820_d_n11, assign96230_e148820_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign96230_e148818: f64 = (p.p13 * locals.var_js2);
        (assign96230_e148818, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96230_e148820;
        locals.var_isbd2_btm_dn0 = assign96230_e148820_d_n0;
        locals.var_isbd2_btm_dn2 = assign96230_e148820_d_n2;
        locals.var_isbd2_btm_dn4 = assign96230_e148820_d_n4;
        locals.var_isbd2_btm_dn5 = assign96230_e148820_d_n5;
        locals.var_isbd2_btm_dn6 = assign96230_e148820_d_n6;
        locals.var_isbd2_btm_dn7 = assign96230_e148820_d_n7;
        locals.var_isbd2_btm_dn8 = assign96230_e148820_d_n8;
        locals.var_isbd2_btm_dn9 = assign96230_e148820_d_n9;
        locals.var_isbd2_btm_dn10 = assign96230_e148820_d_n10;
        locals.var_isbd2_btm_dn11 = assign96230_e148820_d_n11;
        locals.var_isbd2_btm_dn14 = assign96230_e148820_d_n14;

        let (assign96240_e148829, assign96240_e148829_d_n0, assign96240_e148829_d_n2, assign96240_e148829_d_n4, assign96240_e148829_d_n5, assign96240_e148829_d_n6, assign96240_e148829_d_n7, assign96240_e148829_d_n8, assign96240_e148829_d_n9, assign96240_e148829_d_n10, assign96240_e148829_d_n11, assign96240_e148829_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96240_e148829;
        locals.var_isbd_sws_dn0 = assign96240_e148829_d_n0;
        locals.var_isbd_sws_dn2 = assign96240_e148829_d_n2;
        locals.var_isbd_sws_dn4 = assign96240_e148829_d_n4;
        locals.var_isbd_sws_dn5 = assign96240_e148829_d_n5;
        locals.var_isbd_sws_dn6 = assign96240_e148829_d_n6;
        locals.var_isbd_sws_dn7 = assign96240_e148829_d_n7;
        locals.var_isbd_sws_dn8 = assign96240_e148829_d_n8;
        locals.var_isbd_sws_dn9 = assign96240_e148829_d_n9;
        locals.var_isbd_sws_dn10 = assign96240_e148829_d_n10;
        locals.var_isbd_sws_dn11 = assign96240_e148829_d_n11;
        locals.var_isbd_sws_dn14 = assign96240_e148829_d_n14;

    }

    pub(super) fn stamp_transient_block_355(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96250_e148838, assign96250_e148838_d_n0, assign96250_e148838_d_n2, assign96250_e148838_d_n4, assign96250_e148838_d_n5, assign96250_e148838_d_n6, assign96250_e148838_d_n7, assign96250_e148838_d_n8, assign96250_e148838_d_n9, assign96250_e148838_d_n10, assign96250_e148838_d_n11, assign96250_e148838_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96250_e148838;
        locals.var_isbd2_sws_dn0 = assign96250_e148838_d_n0;
        locals.var_isbd2_sws_dn2 = assign96250_e148838_d_n2;
        locals.var_isbd2_sws_dn4 = assign96250_e148838_d_n4;
        locals.var_isbd2_sws_dn5 = assign96250_e148838_d_n5;
        locals.var_isbd2_sws_dn6 = assign96250_e148838_d_n6;
        locals.var_isbd2_sws_dn7 = assign96250_e148838_d_n7;
        locals.var_isbd2_sws_dn8 = assign96250_e148838_d_n8;
        locals.var_isbd2_sws_dn9 = assign96250_e148838_d_n9;
        locals.var_isbd2_sws_dn10 = assign96250_e148838_d_n10;
        locals.var_isbd2_sws_dn11 = assign96250_e148838_d_n11;
        locals.var_isbd2_sws_dn14 = assign96250_e148838_d_n14;

        let (assign96260_e148849, assign96260_e148849_d_n0, assign96260_e148849_d_n2, assign96260_e148849_d_n4, assign96260_e148849_d_n5, assign96260_e148849_d_n6, assign96260_e148849_d_n7, assign96260_e148849_d_n8, assign96260_e148849_d_n9, assign96260_e148849_d_n10, assign96260_e148849_d_n11, assign96260_e148849_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign96260_e148847: f64 = (p.p15 * locals.var_jsswg);
        (assign96260_e148847, (p.p15 * locals.var_jsswg_dn0), (p.p15 * locals.var_jsswg_dn2), (p.p15 * locals.var_jsswg_dn4), (p.p15 * locals.var_jsswg_dn5), (p.p15 * locals.var_jsswg_dn6), (p.p15 * locals.var_jsswg_dn7), (p.p15 * locals.var_jsswg_dn8), (p.p15 * locals.var_jsswg_dn9), (p.p15 * locals.var_jsswg_dn10), (p.p15 * locals.var_jsswg_dn11), (p.p15 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96260_e148849;
        locals.var_isbd_swg_dn0 = assign96260_e148849_d_n0;
        locals.var_isbd_swg_dn2 = assign96260_e148849_d_n2;
        locals.var_isbd_swg_dn4 = assign96260_e148849_d_n4;
        locals.var_isbd_swg_dn5 = assign96260_e148849_d_n5;
        locals.var_isbd_swg_dn6 = assign96260_e148849_d_n6;
        locals.var_isbd_swg_dn7 = assign96260_e148849_d_n7;
        locals.var_isbd_swg_dn8 = assign96260_e148849_d_n8;
        locals.var_isbd_swg_dn9 = assign96260_e148849_d_n9;
        locals.var_isbd_swg_dn10 = assign96260_e148849_d_n10;
        locals.var_isbd_swg_dn11 = assign96260_e148849_d_n11;
        locals.var_isbd_swg_dn14 = assign96260_e148849_d_n14;

        let (assign96270_e148860, assign96270_e148860_d_n0, assign96270_e148860_d_n2, assign96270_e148860_d_n4, assign96270_e148860_d_n5, assign96270_e148860_d_n6, assign96270_e148860_d_n7, assign96270_e148860_d_n8, assign96270_e148860_d_n9, assign96270_e148860_d_n10, assign96270_e148860_d_n11, assign96270_e148860_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2236 != 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign96270_e148858: f64 = (p.p15 * locals.var_jsswg2);
        (assign96270_e148858, (p.p15 * locals.var_jsswg2_dn0), (p.p15 * locals.var_jsswg2_dn2), (p.p15 * locals.var_jsswg2_dn4), (p.p15 * locals.var_jsswg2_dn5), (p.p15 * locals.var_jsswg2_dn6), (p.p15 * locals.var_jsswg2_dn7), (p.p15 * locals.var_jsswg2_dn8), (p.p15 * locals.var_jsswg2_dn9), (p.p15 * locals.var_jsswg2_dn10), (p.p15 * locals.var_jsswg2_dn11), (p.p15 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96270_e148860;
        locals.var_isbd2_swg_dn0 = assign96270_e148860_d_n0;
        locals.var_isbd2_swg_dn2 = assign96270_e148860_d_n2;
        locals.var_isbd2_swg_dn4 = assign96270_e148860_d_n4;
        locals.var_isbd2_swg_dn5 = assign96270_e148860_d_n5;
        locals.var_isbd2_swg_dn6 = assign96270_e148860_d_n6;
        locals.var_isbd2_swg_dn7 = assign96270_e148860_d_n7;
        locals.var_isbd2_swg_dn8 = assign96270_e148860_d_n8;
        locals.var_isbd2_swg_dn9 = assign96270_e148860_d_n9;
        locals.var_isbd2_swg_dn10 = assign96270_e148860_d_n10;
        locals.var_isbd2_swg_dn11 = assign96270_e148860_d_n11;
        locals.var_isbd2_swg_dn14 = assign96270_e148860_d_n14;

        let (assign96280_e148869, assign96280_e148869_d_n0, assign96280_e148869_d_n2, assign96280_e148869_d_n4, assign96280_e148869_d_n5, assign96280_e148869_d_n6, assign96280_e148869_d_n7, assign96280_e148869_d_n8, assign96280_e148869_d_n9, assign96280_e148869_d_n10, assign96280_e148869_d_n11, assign96280_e148869_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        let assign96280_e148867: f64 = (p.p13 * locals.var_js);
        (assign96280_e148867, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96280_e148869;
        locals.var_isbd_btm_dn0 = assign96280_e148869_d_n0;
        locals.var_isbd_btm_dn2 = assign96280_e148869_d_n2;
        locals.var_isbd_btm_dn4 = assign96280_e148869_d_n4;
        locals.var_isbd_btm_dn5 = assign96280_e148869_d_n5;
        locals.var_isbd_btm_dn6 = assign96280_e148869_d_n6;
        locals.var_isbd_btm_dn7 = assign96280_e148869_d_n7;
        locals.var_isbd_btm_dn8 = assign96280_e148869_d_n8;
        locals.var_isbd_btm_dn9 = assign96280_e148869_d_n9;
        locals.var_isbd_btm_dn10 = assign96280_e148869_d_n10;
        locals.var_isbd_btm_dn11 = assign96280_e148869_d_n11;
        locals.var_isbd_btm_dn14 = assign96280_e148869_d_n14;

        let (assign96290_e148878, assign96290_e148878_d_n0, assign96290_e148878_d_n2, assign96290_e148878_d_n4, assign96290_e148878_d_n5, assign96290_e148878_d_n6, assign96290_e148878_d_n7, assign96290_e148878_d_n8, assign96290_e148878_d_n9, assign96290_e148878_d_n10, assign96290_e148878_d_n11, assign96290_e148878_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        let assign96290_e148876: f64 = (p.p13 * locals.var_js2);
        (assign96290_e148876, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96290_e148878;
        locals.var_isbd2_btm_dn0 = assign96290_e148878_d_n0;
        locals.var_isbd2_btm_dn2 = assign96290_e148878_d_n2;
        locals.var_isbd2_btm_dn4 = assign96290_e148878_d_n4;
        locals.var_isbd2_btm_dn5 = assign96290_e148878_d_n5;
        locals.var_isbd2_btm_dn6 = assign96290_e148878_d_n6;
        locals.var_isbd2_btm_dn7 = assign96290_e148878_d_n7;
        locals.var_isbd2_btm_dn8 = assign96290_e148878_d_n8;
        locals.var_isbd2_btm_dn9 = assign96290_e148878_d_n9;
        locals.var_isbd2_btm_dn10 = assign96290_e148878_d_n10;
        locals.var_isbd2_btm_dn11 = assign96290_e148878_d_n11;
        locals.var_isbd2_btm_dn14 = assign96290_e148878_d_n14;

        let (assign96300_e148887, assign96300_e148887_d_n0, assign96300_e148887_d_n2, assign96300_e148887_d_n4, assign96300_e148887_d_n5, assign96300_e148887_d_n6, assign96300_e148887_d_n7, assign96300_e148887_d_n8, assign96300_e148887_d_n9, assign96300_e148887_d_n10, assign96300_e148887_d_n11, assign96300_e148887_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        let assign96300_e148885: f64 = (p.p15 * locals.var_jssw);
        (assign96300_e148885, (p.p15 * locals.var_jssw_dn0), (p.p15 * locals.var_jssw_dn2), (p.p15 * locals.var_jssw_dn4), (p.p15 * locals.var_jssw_dn5), (p.p15 * locals.var_jssw_dn6), (p.p15 * locals.var_jssw_dn7), (p.p15 * locals.var_jssw_dn8), (p.p15 * locals.var_jssw_dn9), (p.p15 * locals.var_jssw_dn10), (p.p15 * locals.var_jssw_dn11), (p.p15 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96300_e148887;
        locals.var_isbd_sws_dn0 = assign96300_e148887_d_n0;
        locals.var_isbd_sws_dn2 = assign96300_e148887_d_n2;
        locals.var_isbd_sws_dn4 = assign96300_e148887_d_n4;
        locals.var_isbd_sws_dn5 = assign96300_e148887_d_n5;
        locals.var_isbd_sws_dn6 = assign96300_e148887_d_n6;
        locals.var_isbd_sws_dn7 = assign96300_e148887_d_n7;
        locals.var_isbd_sws_dn8 = assign96300_e148887_d_n8;
        locals.var_isbd_sws_dn9 = assign96300_e148887_d_n9;
        locals.var_isbd_sws_dn10 = assign96300_e148887_d_n10;
        locals.var_isbd_sws_dn11 = assign96300_e148887_d_n11;
        locals.var_isbd_sws_dn14 = assign96300_e148887_d_n14;

        let (assign96310_e148896, assign96310_e148896_d_n0, assign96310_e148896_d_n2, assign96310_e148896_d_n4, assign96310_e148896_d_n5, assign96310_e148896_d_n6, assign96310_e148896_d_n7, assign96310_e148896_d_n8, assign96310_e148896_d_n9, assign96310_e148896_d_n10, assign96310_e148896_d_n11, assign96310_e148896_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        let assign96310_e148894: f64 = (p.p15 * locals.var_jssw2);
        (assign96310_e148894, (p.p15 * locals.var_jssw2_dn0), (p.p15 * locals.var_jssw2_dn2), (p.p15 * locals.var_jssw2_dn4), (p.p15 * locals.var_jssw2_dn5), (p.p15 * locals.var_jssw2_dn6), (p.p15 * locals.var_jssw2_dn7), (p.p15 * locals.var_jssw2_dn8), (p.p15 * locals.var_jssw2_dn9), (p.p15 * locals.var_jssw2_dn10), (p.p15 * locals.var_jssw2_dn11), (p.p15 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96310_e148896;
        locals.var_isbd2_sws_dn0 = assign96310_e148896_d_n0;
        locals.var_isbd2_sws_dn2 = assign96310_e148896_d_n2;
        locals.var_isbd2_sws_dn4 = assign96310_e148896_d_n4;
        locals.var_isbd2_sws_dn5 = assign96310_e148896_d_n5;
        locals.var_isbd2_sws_dn6 = assign96310_e148896_d_n6;
        locals.var_isbd2_sws_dn7 = assign96310_e148896_d_n7;
        locals.var_isbd2_sws_dn8 = assign96310_e148896_d_n8;
        locals.var_isbd2_sws_dn9 = assign96310_e148896_d_n9;
        locals.var_isbd2_sws_dn10 = assign96310_e148896_d_n10;
        locals.var_isbd2_sws_dn11 = assign96310_e148896_d_n11;
        locals.var_isbd2_sws_dn14 = assign96310_e148896_d_n14;

        let (assign96320_e148903, assign96320_e148903_d_n0, assign96320_e148903_d_n2, assign96320_e148903_d_n4, assign96320_e148903_d_n5, assign96320_e148903_d_n6, assign96320_e148903_d_n7, assign96320_e148903_d_n8, assign96320_e148903_d_n9, assign96320_e148903_d_n10, assign96320_e148903_d_n11, assign96320_e148903_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96320_e148903;
        locals.var_isbd_swg_dn0 = assign96320_e148903_d_n0;
        locals.var_isbd_swg_dn2 = assign96320_e148903_d_n2;
        locals.var_isbd_swg_dn4 = assign96320_e148903_d_n4;
        locals.var_isbd_swg_dn5 = assign96320_e148903_d_n5;
        locals.var_isbd_swg_dn6 = assign96320_e148903_d_n6;
        locals.var_isbd_swg_dn7 = assign96320_e148903_d_n7;
        locals.var_isbd_swg_dn8 = assign96320_e148903_d_n8;
        locals.var_isbd_swg_dn9 = assign96320_e148903_d_n9;
        locals.var_isbd_swg_dn10 = assign96320_e148903_d_n10;
        locals.var_isbd_swg_dn11 = assign96320_e148903_d_n11;
        locals.var_isbd_swg_dn14 = assign96320_e148903_d_n14;

        let (assign96330_e148910, assign96330_e148910_d_n0, assign96330_e148910_d_n2, assign96330_e148910_d_n4, assign96330_e148910_d_n5, assign96330_e148910_d_n6, assign96330_e148910_d_n7, assign96330_e148910_d_n8, assign96330_e148910_d_n9, assign96330_e148910_d_n10, assign96330_e148910_d_n11, assign96330_e148910_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2236 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96330_e148910;
        locals.var_isbd2_swg_dn0 = assign96330_e148910_d_n0;
        locals.var_isbd2_swg_dn2 = assign96330_e148910_d_n2;
        locals.var_isbd2_swg_dn4 = assign96330_e148910_d_n4;
        locals.var_isbd2_swg_dn5 = assign96330_e148910_d_n5;
        locals.var_isbd2_swg_dn6 = assign96330_e148910_d_n6;
        locals.var_isbd2_swg_dn7 = assign96330_e148910_d_n7;
        locals.var_isbd2_swg_dn8 = assign96330_e148910_d_n8;
        locals.var_isbd2_swg_dn9 = assign96330_e148910_d_n9;
        locals.var_isbd2_swg_dn10 = assign96330_e148910_d_n10;
        locals.var_isbd2_swg_dn11 = assign96330_e148910_d_n11;
        locals.var_isbd2_swg_dn14 = assign96330_e148910_d_n14;

        let (assign96340_e148918, assign96340_e148918_d_n0, assign96340_e148918_d_n2, assign96340_e148918_d_n4, assign96340_e148918_d_n5, assign96340_e148918_d_n6, assign96340_e148918_d_n7, assign96340_e148918_d_n8, assign96340_e148918_d_n9, assign96340_e148918_d_n10, assign96340_e148918_d_n11, assign96340_e148918_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96340_e148914: f64 = (locals.var_isbd_btm + locals.var_isbd_sws);
        let assign96340_e148916: f64 = (assign96340_e148914 + locals.var_isbd_swg);
        (assign96340_e148916, ((locals.var_isbd_btm_dn0 + locals.var_isbd_sws_dn0) + locals.var_isbd_swg_dn0), ((locals.var_isbd_btm_dn2 + locals.var_isbd_sws_dn2) + locals.var_isbd_swg_dn2), ((locals.var_isbd_btm_dn4 + locals.var_isbd_sws_dn4) + locals.var_isbd_swg_dn4), ((locals.var_isbd_btm_dn5 + locals.var_isbd_sws_dn5) + locals.var_isbd_swg_dn5), ((locals.var_isbd_btm_dn6 + locals.var_isbd_sws_dn6) + locals.var_isbd_swg_dn6), ((locals.var_isbd_btm_dn7 + locals.var_isbd_sws_dn7) + locals.var_isbd_swg_dn7), ((locals.var_isbd_btm_dn8 + locals.var_isbd_sws_dn8) + locals.var_isbd_swg_dn8), ((locals.var_isbd_btm_dn9 + locals.var_isbd_sws_dn9) + locals.var_isbd_swg_dn9), ((locals.var_isbd_btm_dn10 + locals.var_isbd_sws_dn10) + locals.var_isbd_swg_dn10), ((locals.var_isbd_btm_dn11 + locals.var_isbd_sws_dn11) + locals.var_isbd_swg_dn11), ((locals.var_isbd_btm_dn14 + locals.var_isbd_sws_dn14) + locals.var_isbd_swg_dn14),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    }
};
        locals.var_isbd = assign96340_e148918;
        locals.var_isbd_dn0 = assign96340_e148918_d_n0;
        locals.var_isbd_dn2 = assign96340_e148918_d_n2;
        locals.var_isbd_dn4 = assign96340_e148918_d_n4;
        locals.var_isbd_dn5 = assign96340_e148918_d_n5;
        locals.var_isbd_dn6 = assign96340_e148918_d_n6;
        locals.var_isbd_dn7 = assign96340_e148918_d_n7;
        locals.var_isbd_dn8 = assign96340_e148918_d_n8;
        locals.var_isbd_dn9 = assign96340_e148918_d_n9;
        locals.var_isbd_dn10 = assign96340_e148918_d_n10;
        locals.var_isbd_dn11 = assign96340_e148918_d_n11;
        locals.var_isbd_dn14 = assign96340_e148918_d_n14;

        let assign96350_e148921: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2238 = assign96350_e148921;

        let (assign96360_e148929, assign96360_e148929_d_n0, assign96360_e148929_d_n2, assign96360_e148929_d_n4, assign96360_e148929_d_n5, assign96360_e148929_d_n6, assign96360_e148929_d_n7, assign96360_e148929_d_n8, assign96360_e148929_d_n9, assign96360_e148929_d_n10, assign96360_e148929_d_n11, assign96360_e148929_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2238 != 0.0)) {
        let assign96360_e148927: f64 = (locals.var_isbd + 1e-25);
        (assign96360_e148927, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign96360_e148929;
        locals.var_t2_dn0 = assign96360_e148929_d_n0;
        locals.var_t2_dn2 = assign96360_e148929_d_n2;
        locals.var_t2_dn4 = assign96360_e148929_d_n4;
        locals.var_t2_dn5 = assign96360_e148929_d_n5;
        locals.var_t2_dn6 = assign96360_e148929_d_n6;
        locals.var_t2_dn7 = assign96360_e148929_d_n7;
        locals.var_t2_dn8 = assign96360_e148929_d_n8;
        locals.var_t2_dn9 = assign96360_e148929_d_n9;
        locals.var_t2_dn10 = assign96360_e148929_d_n10;
        locals.var_t2_dn11 = assign96360_e148929_d_n11;
        locals.var_t2_dn14 = assign96360_e148929_d_n14;

        let (assign96370_e148946, assign96370_e148946_d_n0, assign96370_e148946_d_n2, assign96370_e148946_d_n4, assign96370_e148946_d_n5, assign96370_e148946_d_n6, assign96370_e148946_d_n7, assign96370_e148946_d_n8, assign96370_e148946_d_n9, assign96370_e148946_d_n10, assign96370_e148946_d_n11, assign96370_e148946_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2238 != 0.0)) {
        let assign96370_e148935: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96370_e148938: f64 = (locals.var_uc_vdiffjd * locals.var_t0);
        let assign96370_e148940: f64 = (assign96370_e148938 / locals.var_t2);
        let assign96370_e148942: f64 = (assign96370_e148940 + 1.0);
        let assign96370_e148943: f64 = (assign96370_e148942).ln();
        let assign96370_e148944: f64 = (assign96370_e148935 * assign96370_e148943);
        (assign96370_e148944, (((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn0) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn2) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn4) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn5) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn6) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn7) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn8) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn9) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn10) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn11) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))), (((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign96370_e148943) + (assign96370_e148935 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn14) * locals.var_t2) - (assign96370_e148938 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) / assign96370_e148942))),)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn0, locals.var_vbdt_dn2, locals.var_vbdt_dn4, locals.var_vbdt_dn5, locals.var_vbdt_dn6, locals.var_vbdt_dn7, locals.var_vbdt_dn8, locals.var_vbdt_dn9, locals.var_vbdt_dn10, locals.var_vbdt_dn11, locals.var_vbdt_dn14,)
    }
};
        locals.var_vbdt = assign96370_e148946;
        locals.var_vbdt_dn0 = assign96370_e148946_d_n0;
        locals.var_vbdt_dn2 = assign96370_e148946_d_n2;
        locals.var_vbdt_dn4 = assign96370_e148946_d_n4;
        locals.var_vbdt_dn5 = assign96370_e148946_d_n5;
        locals.var_vbdt_dn6 = assign96370_e148946_d_n6;
        locals.var_vbdt_dn7 = assign96370_e148946_d_n7;
        locals.var_vbdt_dn8 = assign96370_e148946_d_n8;
        locals.var_vbdt_dn9 = assign96370_e148946_d_n9;
        locals.var_vbdt_dn10 = assign96370_e148946_d_n10;
        locals.var_vbdt_dn11 = assign96370_e148946_d_n11;
        locals.var_vbdt_dn14 = assign96370_e148946_d_n14;

        let (assign96380_e148957, assign96380_e148957_d_n0, assign96380_e148957_d_n2, assign96380_e148957_d_n4, assign96380_e148957_d_n5, assign96380_e148957_d_n6, assign96380_e148957_d_n7, assign96380_e148957_d_n8, assign96380_e148957_d_n9, assign96380_e148957_d_n10, assign96380_e148957_d_n11, assign96380_e148957_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2238 != 0.0)) {
        let assign96380_e148952: f64 = (locals.var_tratio - 1.0);
        let assign96380_e148954: f64 = (assign96380_e148952 * p.p512);
        let assign96380_e148955: f64 = (assign96380_e148954).exp();
        (assign96380_e148955, (assign96380_e148955 * (locals.var_tratio_dn0 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn2 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn4 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn5 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn6 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn7 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn8 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn9 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn10 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn11 * p.p512)), (assign96380_e148955 * (locals.var_tratio_dn14 * p.p512)),)
    } else {
        (locals.var_exptempd, locals.var_exptempd_dn0, locals.var_exptempd_dn2, locals.var_exptempd_dn4, locals.var_exptempd_dn5, locals.var_exptempd_dn6, locals.var_exptempd_dn7, locals.var_exptempd_dn8, locals.var_exptempd_dn9, locals.var_exptempd_dn10, locals.var_exptempd_dn11, locals.var_exptempd_dn14,)
    }
};
        locals.var_exptempd = assign96380_e148957;
        locals.var_exptempd_dn0 = assign96380_e148957_d_n0;
        locals.var_exptempd_dn2 = assign96380_e148957_d_n2;
        locals.var_exptempd_dn4 = assign96380_e148957_d_n4;
        locals.var_exptempd_dn5 = assign96380_e148957_d_n5;
        locals.var_exptempd_dn6 = assign96380_e148957_d_n6;
        locals.var_exptempd_dn7 = assign96380_e148957_d_n7;
        locals.var_exptempd_dn8 = assign96380_e148957_d_n8;
        locals.var_exptempd_dn9 = assign96380_e148957_d_n9;
        locals.var_exptempd_dn10 = assign96380_e148957_d_n10;
        locals.var_exptempd_dn11 = assign96380_e148957_d_n11;
        locals.var_exptempd_dn14 = assign96380_e148957_d_n14;

        let (assign96390_e148967, assign96390_e148967_d_n0, assign96390_e148967_d_n2, assign96390_e148967_d_n4, assign96390_e148967_d_n5, assign96390_e148967_d_n6, assign96390_e148967_d_n7, assign96390_e148967_d_n8, assign96390_e148967_d_n9, assign96390_e148967_d_n10, assign96390_e148967_d_n11, assign96390_e148967_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2238 != 0.0)) {
        let assign96390_e148964: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96390_e148965: f64 = (1.0 / assign96390_e148964);
        (assign96390_e148965, (-((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))), (-((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign96390_e148964 * assign96390_e148964))),)
    } else {
        (locals.var_jd_nvtm_invd, locals.var_jd_nvtm_invd_dn0, locals.var_jd_nvtm_invd_dn2, locals.var_jd_nvtm_invd_dn4, locals.var_jd_nvtm_invd_dn5, locals.var_jd_nvtm_invd_dn6, locals.var_jd_nvtm_invd_dn7, locals.var_jd_nvtm_invd_dn8, locals.var_jd_nvtm_invd_dn9, locals.var_jd_nvtm_invd_dn10, locals.var_jd_nvtm_invd_dn11, locals.var_jd_nvtm_invd_dn14,)
    }
};
        locals.var_jd_nvtm_invd = assign96390_e148967;
        locals.var_jd_nvtm_invd_dn0 = assign96390_e148967_d_n0;
        locals.var_jd_nvtm_invd_dn2 = assign96390_e148967_d_n2;
        locals.var_jd_nvtm_invd_dn4 = assign96390_e148967_d_n4;
        locals.var_jd_nvtm_invd_dn5 = assign96390_e148967_d_n5;
        locals.var_jd_nvtm_invd_dn6 = assign96390_e148967_d_n6;
        locals.var_jd_nvtm_invd_dn7 = assign96390_e148967_d_n7;
        locals.var_jd_nvtm_invd_dn8 = assign96390_e148967_d_n8;
        locals.var_jd_nvtm_invd_dn9 = assign96390_e148967_d_n9;
        locals.var_jd_nvtm_invd_dn10 = assign96390_e148967_d_n10;
        locals.var_jd_nvtm_invd_dn11 = assign96390_e148967_d_n11;
        locals.var_jd_nvtm_invd_dn14 = assign96390_e148967_d_n14;

        let (assign96400_e148976, assign96400_e148976_d_n0, assign96400_e148976_d_n2, assign96400_e148976_d_n4, assign96400_e148976_d_n5, assign96400_e148976_d_n6, assign96400_e148976_d_n7, assign96400_e148976_d_n8, assign96400_e148976_d_n9, assign96400_e148976_d_n10, assign96400_e148976_d_n11, assign96400_e148976_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2238 != 0.0)) {
        let assign96400_e148973: f64 = (locals.var_vbdt * locals.var_jd_nvtm_invd);
        let assign96400_e148974: f64 = (assign96400_e148973).exp();
        (assign96400_e148974, (assign96400_e148974 * ((locals.var_vbdt_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn0))), (assign96400_e148974 * ((locals.var_vbdt_dn2 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn2))), (assign96400_e148974 * ((locals.var_vbdt_dn4 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn4))), (assign96400_e148974 * ((locals.var_vbdt_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn5))), (assign96400_e148974 * ((locals.var_vbdt_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn6))), (assign96400_e148974 * ((locals.var_vbdt_dn7 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn7))), (assign96400_e148974 * ((locals.var_vbdt_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn8))), (assign96400_e148974 * ((locals.var_vbdt_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn9))), (assign96400_e148974 * ((locals.var_vbdt_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn10))), (assign96400_e148974 * ((locals.var_vbdt_dn11 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn11))), (assign96400_e148974 * ((locals.var_vbdt_dn14 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn14))),)
    } else {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    }
};
        locals.var_jd_expcd = assign96400_e148976;
        locals.var_jd_expcd_dn0 = assign96400_e148976_d_n0;
        locals.var_jd_expcd_dn2 = assign96400_e148976_d_n2;
        locals.var_jd_expcd_dn4 = assign96400_e148976_d_n4;
        locals.var_jd_expcd_dn5 = assign96400_e148976_d_n5;
        locals.var_jd_expcd_dn6 = assign96400_e148976_d_n6;
        locals.var_jd_expcd_dn7 = assign96400_e148976_d_n7;
        locals.var_jd_expcd_dn8 = assign96400_e148976_d_n8;
        locals.var_jd_expcd_dn9 = assign96400_e148976_d_n9;
        locals.var_jd_expcd_dn10 = assign96400_e148976_d_n10;
        locals.var_jd_expcd_dn11 = assign96400_e148976_d_n11;
        locals.var_jd_expcd_dn14 = assign96400_e148976_d_n14;

        let (assign96410_e148995, assign96410_e148995_d_n0, assign96410_e148995_d_n2, assign96410_e148995_d_n4, assign96410_e148995_d_n5, assign96410_e148995_d_n6, assign96410_e148995_d_n7, assign96410_e148995_d_n8, assign96410_e148995_d_n9, assign96410_e148995_d_n10, assign96410_e148995_d_n11, assign96410_e148995_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96410_e148981: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96410_e148984: f64 = (locals.var_eg * locals.var_beta);
        let assign96410_e148985: f64 = (assign96410_e148981 - assign96410_e148984);
        let assign96410_e148988: f64 = (p.p522 * locals.var_log_tratio);
        let assign96410_e148989: f64 = (assign96410_e148985 + assign96410_e148988);
        let assign96410_e148991: f64 = (assign96410_e148989 / locals.var_uc_njs);
        let assign96410_e148992: f64 = (assign96410_e148991).exp();
        let assign96410_e148993: f64 = (locals.var_uc_js0s * assign96410_e148992);
        (assign96410_e148993, (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96410_e148992 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign96410_e148995;
        locals.var_js_dn0 = assign96410_e148995_d_n0;
        locals.var_js_dn2 = assign96410_e148995_d_n2;
        locals.var_js_dn4 = assign96410_e148995_d_n4;
        locals.var_js_dn5 = assign96410_e148995_d_n5;
        locals.var_js_dn6 = assign96410_e148995_d_n6;
        locals.var_js_dn7 = assign96410_e148995_d_n7;
        locals.var_js_dn8 = assign96410_e148995_d_n8;
        locals.var_js_dn9 = assign96410_e148995_d_n9;
        locals.var_js_dn10 = assign96410_e148995_d_n10;
        locals.var_js_dn11 = assign96410_e148995_d_n11;
        locals.var_js_dn14 = assign96410_e148995_d_n14;

        let (assign96420_e149014, assign96420_e149014_d_n0, assign96420_e149014_d_n2, assign96420_e149014_d_n4, assign96420_e149014_d_n5, assign96420_e149014_d_n6, assign96420_e149014_d_n7, assign96420_e149014_d_n8, assign96420_e149014_d_n9, assign96420_e149014_d_n10, assign96420_e149014_d_n11, assign96420_e149014_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96420_e149000: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96420_e149003: f64 = (locals.var_eg * locals.var_beta);
        let assign96420_e149004: f64 = (assign96420_e149000 - assign96420_e149003);
        let assign96420_e149007: f64 = (p.p522 * locals.var_log_tratio);
        let assign96420_e149008: f64 = (assign96420_e149004 + assign96420_e149007);
        let assign96420_e149010: f64 = (assign96420_e149008 / p.p520);
        let assign96420_e149011: f64 = (assign96420_e149010).exp();
        let assign96420_e149012: f64 = (locals.var_uc_js0sws * assign96420_e149011);
        (assign96420_e149012, (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign96420_e149011 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign96420_e149014;
        locals.var_jssw_dn0 = assign96420_e149014_d_n0;
        locals.var_jssw_dn2 = assign96420_e149014_d_n2;
        locals.var_jssw_dn4 = assign96420_e149014_d_n4;
        locals.var_jssw_dn5 = assign96420_e149014_d_n5;
        locals.var_jssw_dn6 = assign96420_e149014_d_n6;
        locals.var_jssw_dn7 = assign96420_e149014_d_n7;
        locals.var_jssw_dn8 = assign96420_e149014_d_n8;
        locals.var_jssw_dn9 = assign96420_e149014_d_n9;
        locals.var_jssw_dn10 = assign96420_e149014_d_n10;
        locals.var_jssw_dn11 = assign96420_e149014_d_n11;
        locals.var_jssw_dn14 = assign96420_e149014_d_n14;

        let (assign96430_e149033, assign96430_e149033_d_n0, assign96430_e149033_d_n2, assign96430_e149033_d_n4, assign96430_e149033_d_n5, assign96430_e149033_d_n6, assign96430_e149033_d_n7, assign96430_e149033_d_n8, assign96430_e149033_d_n9, assign96430_e149033_d_n10, assign96430_e149033_d_n11, assign96430_e149033_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96430_e149019: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96430_e149022: f64 = (locals.var_eg * locals.var_beta);
        let assign96430_e149023: f64 = (assign96430_e149019 - assign96430_e149022);
        let assign96430_e149026: f64 = (p.p522 * locals.var_log_tratio);
        let assign96430_e149027: f64 = (assign96430_e149023 + assign96430_e149026);
        let assign96430_e149029: f64 = (assign96430_e149027 / p.p521);
        let assign96430_e149030: f64 = (assign96430_e149029).exp();
        let assign96430_e149031: f64 = (p.p518 * assign96430_e149030);
        (assign96430_e149031, (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign96430_e149030 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign96430_e149033;
        locals.var_jsswg_dn0 = assign96430_e149033_d_n0;
        locals.var_jsswg_dn2 = assign96430_e149033_d_n2;
        locals.var_jsswg_dn4 = assign96430_e149033_d_n4;
        locals.var_jsswg_dn5 = assign96430_e149033_d_n5;
        locals.var_jsswg_dn6 = assign96430_e149033_d_n6;
        locals.var_jsswg_dn7 = assign96430_e149033_d_n7;
        locals.var_jsswg_dn8 = assign96430_e149033_d_n8;
        locals.var_jsswg_dn9 = assign96430_e149033_d_n9;
        locals.var_jsswg_dn10 = assign96430_e149033_d_n10;
        locals.var_jsswg_dn11 = assign96430_e149033_d_n11;
        locals.var_jsswg_dn14 = assign96430_e149033_d_n14;

        let (assign96440_e149052, assign96440_e149052_d_n0, assign96440_e149052_d_n2, assign96440_e149052_d_n4, assign96440_e149052_d_n5, assign96440_e149052_d_n6, assign96440_e149052_d_n7, assign96440_e149052_d_n8, assign96440_e149052_d_n9, assign96440_e149052_d_n10, assign96440_e149052_d_n11, assign96440_e149052_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96440_e149038: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96440_e149041: f64 = (locals.var_eg * locals.var_beta);
        let assign96440_e149042: f64 = (assign96440_e149038 - assign96440_e149041);
        let assign96440_e149045: f64 = (p.p532 * locals.var_log_tratio);
        let assign96440_e149046: f64 = (assign96440_e149042 + assign96440_e149045);
        let assign96440_e149048: f64 = (assign96440_e149046 / locals.var_uc_njs);
        let assign96440_e149049: f64 = (assign96440_e149048).exp();
        let assign96440_e149050: f64 = (locals.var_uc_js0s * assign96440_e149049);
        (assign96440_e149050, (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96440_e149049 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign96440_e149052;
        locals.var_js2_dn0 = assign96440_e149052_d_n0;
        locals.var_js2_dn2 = assign96440_e149052_d_n2;
        locals.var_js2_dn4 = assign96440_e149052_d_n4;
        locals.var_js2_dn5 = assign96440_e149052_d_n5;
        locals.var_js2_dn6 = assign96440_e149052_d_n6;
        locals.var_js2_dn7 = assign96440_e149052_d_n7;
        locals.var_js2_dn8 = assign96440_e149052_d_n8;
        locals.var_js2_dn9 = assign96440_e149052_d_n9;
        locals.var_js2_dn10 = assign96440_e149052_d_n10;
        locals.var_js2_dn11 = assign96440_e149052_d_n11;
        locals.var_js2_dn14 = assign96440_e149052_d_n14;

        let (assign96450_e149071, assign96450_e149071_d_n0, assign96450_e149071_d_n2, assign96450_e149071_d_n4, assign96450_e149071_d_n5, assign96450_e149071_d_n6, assign96450_e149071_d_n7, assign96450_e149071_d_n8, assign96450_e149071_d_n9, assign96450_e149071_d_n10, assign96450_e149071_d_n11, assign96450_e149071_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96450_e149057: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96450_e149060: f64 = (locals.var_eg * locals.var_beta);
        let assign96450_e149061: f64 = (assign96450_e149057 - assign96450_e149060);
        let assign96450_e149064: f64 = (p.p532 * locals.var_log_tratio);
        let assign96450_e149065: f64 = (assign96450_e149061 + assign96450_e149064);
        let assign96450_e149067: f64 = (assign96450_e149065 / p.p520);
        let assign96450_e149068: f64 = (assign96450_e149067).exp();
        let assign96450_e149069: f64 = (locals.var_uc_js0sws * assign96450_e149068);
        (assign96450_e149069, (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign96450_e149068 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign96450_e149071;
        locals.var_jssw2_dn0 = assign96450_e149071_d_n0;
        locals.var_jssw2_dn2 = assign96450_e149071_d_n2;
        locals.var_jssw2_dn4 = assign96450_e149071_d_n4;
        locals.var_jssw2_dn5 = assign96450_e149071_d_n5;
        locals.var_jssw2_dn6 = assign96450_e149071_d_n6;
        locals.var_jssw2_dn7 = assign96450_e149071_d_n7;
        locals.var_jssw2_dn8 = assign96450_e149071_d_n8;
        locals.var_jssw2_dn9 = assign96450_e149071_d_n9;
        locals.var_jssw2_dn10 = assign96450_e149071_d_n10;
        locals.var_jssw2_dn11 = assign96450_e149071_d_n11;
        locals.var_jssw2_dn14 = assign96450_e149071_d_n14;

        let (assign96460_e149090, assign96460_e149090_d_n0, assign96460_e149090_d_n2, assign96460_e149090_d_n4, assign96460_e149090_d_n5, assign96460_e149090_d_n6, assign96460_e149090_d_n7, assign96460_e149090_d_n8, assign96460_e149090_d_n9, assign96460_e149090_d_n10, assign96460_e149090_d_n11, assign96460_e149090_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96460_e149076: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96460_e149079: f64 = (locals.var_eg * locals.var_beta);
        let assign96460_e149080: f64 = (assign96460_e149076 - assign96460_e149079);
        let assign96460_e149083: f64 = (p.p532 * locals.var_log_tratio);
        let assign96460_e149084: f64 = (assign96460_e149080 + assign96460_e149083);
        let assign96460_e149086: f64 = (assign96460_e149084 / p.p521);
        let assign96460_e149087: f64 = (assign96460_e149086).exp();
        let assign96460_e149088: f64 = (p.p518 * assign96460_e149087);
        (assign96460_e149088, (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign96460_e149087 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign96460_e149090;
        locals.var_jsswg2_dn0 = assign96460_e149090_d_n0;
        locals.var_jsswg2_dn2 = assign96460_e149090_d_n2;
        locals.var_jsswg2_dn4 = assign96460_e149090_d_n4;
        locals.var_jsswg2_dn5 = assign96460_e149090_d_n5;
        locals.var_jsswg2_dn6 = assign96460_e149090_d_n6;
        locals.var_jsswg2_dn7 = assign96460_e149090_d_n7;
        locals.var_jsswg2_dn8 = assign96460_e149090_d_n8;
        locals.var_jsswg2_dn9 = assign96460_e149090_d_n9;
        locals.var_jsswg2_dn10 = assign96460_e149090_d_n10;
        locals.var_jsswg2_dn11 = assign96460_e149090_d_n11;
        locals.var_jsswg2_dn14 = assign96460_e149090_d_n14;

        let assign96470_e149093: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2239 = assign96470_e149093;

        let assign96480_e149096: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2240 = assign96480_e149096;

    }

    pub(super) fn stamp_transient_block_356(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96490_e149106, assign96490_e149106_d_n0, assign96490_e149106_d_n2, assign96490_e149106_d_n4, assign96490_e149106_d_n5, assign96490_e149106_d_n6, assign96490_e149106_d_n7, assign96490_e149106_d_n8, assign96490_e149106_d_n9, assign96490_e149106_d_n10, assign96490_e149106_d_n11, assign96490_e149106_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96490_e149104: f64 = (p.p14 * locals.var_js);
        (assign96490_e149104, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96490_e149106;
        locals.var_isbs_btm_dn0 = assign96490_e149106_d_n0;
        locals.var_isbs_btm_dn2 = assign96490_e149106_d_n2;
        locals.var_isbs_btm_dn4 = assign96490_e149106_d_n4;
        locals.var_isbs_btm_dn5 = assign96490_e149106_d_n5;
        locals.var_isbs_btm_dn6 = assign96490_e149106_d_n6;
        locals.var_isbs_btm_dn7 = assign96490_e149106_d_n7;
        locals.var_isbs_btm_dn8 = assign96490_e149106_d_n8;
        locals.var_isbs_btm_dn9 = assign96490_e149106_d_n9;
        locals.var_isbs_btm_dn10 = assign96490_e149106_d_n10;
        locals.var_isbs_btm_dn11 = assign96490_e149106_d_n11;
        locals.var_isbs_btm_dn14 = assign96490_e149106_d_n14;

        let (assign96500_e149116, assign96500_e149116_d_n0, assign96500_e149116_d_n2, assign96500_e149116_d_n4, assign96500_e149116_d_n5, assign96500_e149116_d_n6, assign96500_e149116_d_n7, assign96500_e149116_d_n8, assign96500_e149116_d_n9, assign96500_e149116_d_n10, assign96500_e149116_d_n11, assign96500_e149116_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96500_e149114: f64 = (p.p14 * locals.var_js2);
        (assign96500_e149114, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96500_e149116;
        locals.var_isbs2_btm_dn0 = assign96500_e149116_d_n0;
        locals.var_isbs2_btm_dn2 = assign96500_e149116_d_n2;
        locals.var_isbs2_btm_dn4 = assign96500_e149116_d_n4;
        locals.var_isbs2_btm_dn5 = assign96500_e149116_d_n5;
        locals.var_isbs2_btm_dn6 = assign96500_e149116_d_n6;
        locals.var_isbs2_btm_dn7 = assign96500_e149116_d_n7;
        locals.var_isbs2_btm_dn8 = assign96500_e149116_d_n8;
        locals.var_isbs2_btm_dn9 = assign96500_e149116_d_n9;
        locals.var_isbs2_btm_dn10 = assign96500_e149116_d_n10;
        locals.var_isbs2_btm_dn11 = assign96500_e149116_d_n11;
        locals.var_isbs2_btm_dn14 = assign96500_e149116_d_n14;

        let (assign96510_e149128, assign96510_e149128_d_n0, assign96510_e149128_d_n2, assign96510_e149128_d_n4, assign96510_e149128_d_n5, assign96510_e149128_d_n6, assign96510_e149128_d_n7, assign96510_e149128_d_n8, assign96510_e149128_d_n9, assign96510_e149128_d_n10, assign96510_e149128_d_n11, assign96510_e149128_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96510_e149124: f64 = (p.p16 - locals.var_weff_nf);
        let assign96510_e149126: f64 = (assign96510_e149124 * locals.var_jssw);
        (assign96510_e149126, (assign96510_e149124 * locals.var_jssw_dn0), (assign96510_e149124 * locals.var_jssw_dn2), (assign96510_e149124 * locals.var_jssw_dn4), (assign96510_e149124 * locals.var_jssw_dn5), (assign96510_e149124 * locals.var_jssw_dn6), (assign96510_e149124 * locals.var_jssw_dn7), (assign96510_e149124 * locals.var_jssw_dn8), (assign96510_e149124 * locals.var_jssw_dn9), (assign96510_e149124 * locals.var_jssw_dn10), (assign96510_e149124 * locals.var_jssw_dn11), (assign96510_e149124 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96510_e149128;
        locals.var_isbs_sws_dn0 = assign96510_e149128_d_n0;
        locals.var_isbs_sws_dn2 = assign96510_e149128_d_n2;
        locals.var_isbs_sws_dn4 = assign96510_e149128_d_n4;
        locals.var_isbs_sws_dn5 = assign96510_e149128_d_n5;
        locals.var_isbs_sws_dn6 = assign96510_e149128_d_n6;
        locals.var_isbs_sws_dn7 = assign96510_e149128_d_n7;
        locals.var_isbs_sws_dn8 = assign96510_e149128_d_n8;
        locals.var_isbs_sws_dn9 = assign96510_e149128_d_n9;
        locals.var_isbs_sws_dn10 = assign96510_e149128_d_n10;
        locals.var_isbs_sws_dn11 = assign96510_e149128_d_n11;
        locals.var_isbs_sws_dn14 = assign96510_e149128_d_n14;

        let (assign96520_e149140, assign96520_e149140_d_n0, assign96520_e149140_d_n2, assign96520_e149140_d_n4, assign96520_e149140_d_n5, assign96520_e149140_d_n6, assign96520_e149140_d_n7, assign96520_e149140_d_n8, assign96520_e149140_d_n9, assign96520_e149140_d_n10, assign96520_e149140_d_n11, assign96520_e149140_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96520_e149136: f64 = (p.p16 - locals.var_weff_nf);
        let assign96520_e149138: f64 = (assign96520_e149136 * locals.var_jssw2);
        (assign96520_e149138, (assign96520_e149136 * locals.var_jssw2_dn0), (assign96520_e149136 * locals.var_jssw2_dn2), (assign96520_e149136 * locals.var_jssw2_dn4), (assign96520_e149136 * locals.var_jssw2_dn5), (assign96520_e149136 * locals.var_jssw2_dn6), (assign96520_e149136 * locals.var_jssw2_dn7), (assign96520_e149136 * locals.var_jssw2_dn8), (assign96520_e149136 * locals.var_jssw2_dn9), (assign96520_e149136 * locals.var_jssw2_dn10), (assign96520_e149136 * locals.var_jssw2_dn11), (assign96520_e149136 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96520_e149140;
        locals.var_isbs2_sws_dn0 = assign96520_e149140_d_n0;
        locals.var_isbs2_sws_dn2 = assign96520_e149140_d_n2;
        locals.var_isbs2_sws_dn4 = assign96520_e149140_d_n4;
        locals.var_isbs2_sws_dn5 = assign96520_e149140_d_n5;
        locals.var_isbs2_sws_dn6 = assign96520_e149140_d_n6;
        locals.var_isbs2_sws_dn7 = assign96520_e149140_d_n7;
        locals.var_isbs2_sws_dn8 = assign96520_e149140_d_n8;
        locals.var_isbs2_sws_dn9 = assign96520_e149140_d_n9;
        locals.var_isbs2_sws_dn10 = assign96520_e149140_d_n10;
        locals.var_isbs2_sws_dn11 = assign96520_e149140_d_n11;
        locals.var_isbs2_sws_dn14 = assign96520_e149140_d_n14;

        let (assign96530_e149150, assign96530_e149150_d_n0, assign96530_e149150_d_n2, assign96530_e149150_d_n4, assign96530_e149150_d_n5, assign96530_e149150_d_n6, assign96530_e149150_d_n7, assign96530_e149150_d_n8, assign96530_e149150_d_n9, assign96530_e149150_d_n10, assign96530_e149150_d_n11, assign96530_e149150_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96530_e149148: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96530_e149148, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96530_e149150;
        locals.var_isbs_swg_dn0 = assign96530_e149150_d_n0;
        locals.var_isbs_swg_dn2 = assign96530_e149150_d_n2;
        locals.var_isbs_swg_dn4 = assign96530_e149150_d_n4;
        locals.var_isbs_swg_dn5 = assign96530_e149150_d_n5;
        locals.var_isbs_swg_dn6 = assign96530_e149150_d_n6;
        locals.var_isbs_swg_dn7 = assign96530_e149150_d_n7;
        locals.var_isbs_swg_dn8 = assign96530_e149150_d_n8;
        locals.var_isbs_swg_dn9 = assign96530_e149150_d_n9;
        locals.var_isbs_swg_dn10 = assign96530_e149150_d_n10;
        locals.var_isbs_swg_dn11 = assign96530_e149150_d_n11;
        locals.var_isbs_swg_dn14 = assign96530_e149150_d_n14;

        let (assign96540_e149160, assign96540_e149160_d_n0, assign96540_e149160_d_n2, assign96540_e149160_d_n4, assign96540_e149160_d_n5, assign96540_e149160_d_n6, assign96540_e149160_d_n7, assign96540_e149160_d_n8, assign96540_e149160_d_n9, assign96540_e149160_d_n10, assign96540_e149160_d_n11, assign96540_e149160_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign96540_e149158: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96540_e149158, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96540_e149160;
        locals.var_isbs2_swg_dn0 = assign96540_e149160_d_n0;
        locals.var_isbs2_swg_dn2 = assign96540_e149160_d_n2;
        locals.var_isbs2_swg_dn4 = assign96540_e149160_d_n4;
        locals.var_isbs2_swg_dn5 = assign96540_e149160_d_n5;
        locals.var_isbs2_swg_dn6 = assign96540_e149160_d_n6;
        locals.var_isbs2_swg_dn7 = assign96540_e149160_d_n7;
        locals.var_isbs2_swg_dn8 = assign96540_e149160_d_n8;
        locals.var_isbs2_swg_dn9 = assign96540_e149160_d_n9;
        locals.var_isbs2_swg_dn10 = assign96540_e149160_d_n10;
        locals.var_isbs2_swg_dn11 = assign96540_e149160_d_n11;
        locals.var_isbs2_swg_dn14 = assign96540_e149160_d_n14;

        let (assign96550_e149171, assign96550_e149171_d_n0, assign96550_e149171_d_n2, assign96550_e149171_d_n4, assign96550_e149171_d_n5, assign96550_e149171_d_n6, assign96550_e149171_d_n7, assign96550_e149171_d_n8, assign96550_e149171_d_n9, assign96550_e149171_d_n10, assign96550_e149171_d_n11, assign96550_e149171_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        let assign96550_e149169: f64 = (p.p14 * locals.var_js);
        (assign96550_e149169, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96550_e149171;
        locals.var_isbs_btm_dn0 = assign96550_e149171_d_n0;
        locals.var_isbs_btm_dn2 = assign96550_e149171_d_n2;
        locals.var_isbs_btm_dn4 = assign96550_e149171_d_n4;
        locals.var_isbs_btm_dn5 = assign96550_e149171_d_n5;
        locals.var_isbs_btm_dn6 = assign96550_e149171_d_n6;
        locals.var_isbs_btm_dn7 = assign96550_e149171_d_n7;
        locals.var_isbs_btm_dn8 = assign96550_e149171_d_n8;
        locals.var_isbs_btm_dn9 = assign96550_e149171_d_n9;
        locals.var_isbs_btm_dn10 = assign96550_e149171_d_n10;
        locals.var_isbs_btm_dn11 = assign96550_e149171_d_n11;
        locals.var_isbs_btm_dn14 = assign96550_e149171_d_n14;

        let (assign96560_e149182, assign96560_e149182_d_n0, assign96560_e149182_d_n2, assign96560_e149182_d_n4, assign96560_e149182_d_n5, assign96560_e149182_d_n6, assign96560_e149182_d_n7, assign96560_e149182_d_n8, assign96560_e149182_d_n9, assign96560_e149182_d_n10, assign96560_e149182_d_n11, assign96560_e149182_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        let assign96560_e149180: f64 = (p.p14 * locals.var_js2);
        (assign96560_e149180, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96560_e149182;
        locals.var_isbs2_btm_dn0 = assign96560_e149182_d_n0;
        locals.var_isbs2_btm_dn2 = assign96560_e149182_d_n2;
        locals.var_isbs2_btm_dn4 = assign96560_e149182_d_n4;
        locals.var_isbs2_btm_dn5 = assign96560_e149182_d_n5;
        locals.var_isbs2_btm_dn6 = assign96560_e149182_d_n6;
        locals.var_isbs2_btm_dn7 = assign96560_e149182_d_n7;
        locals.var_isbs2_btm_dn8 = assign96560_e149182_d_n8;
        locals.var_isbs2_btm_dn9 = assign96560_e149182_d_n9;
        locals.var_isbs2_btm_dn10 = assign96560_e149182_d_n10;
        locals.var_isbs2_btm_dn11 = assign96560_e149182_d_n11;
        locals.var_isbs2_btm_dn14 = assign96560_e149182_d_n14;

        let (assign96570_e149191, assign96570_e149191_d_n0, assign96570_e149191_d_n2, assign96570_e149191_d_n4, assign96570_e149191_d_n5, assign96570_e149191_d_n6, assign96570_e149191_d_n7, assign96570_e149191_d_n8, assign96570_e149191_d_n9, assign96570_e149191_d_n10, assign96570_e149191_d_n11, assign96570_e149191_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96570_e149191;
        locals.var_isbs_sws_dn0 = assign96570_e149191_d_n0;
        locals.var_isbs_sws_dn2 = assign96570_e149191_d_n2;
        locals.var_isbs_sws_dn4 = assign96570_e149191_d_n4;
        locals.var_isbs_sws_dn5 = assign96570_e149191_d_n5;
        locals.var_isbs_sws_dn6 = assign96570_e149191_d_n6;
        locals.var_isbs_sws_dn7 = assign96570_e149191_d_n7;
        locals.var_isbs_sws_dn8 = assign96570_e149191_d_n8;
        locals.var_isbs_sws_dn9 = assign96570_e149191_d_n9;
        locals.var_isbs_sws_dn10 = assign96570_e149191_d_n10;
        locals.var_isbs_sws_dn11 = assign96570_e149191_d_n11;
        locals.var_isbs_sws_dn14 = assign96570_e149191_d_n14;

        let (assign96580_e149200, assign96580_e149200_d_n0, assign96580_e149200_d_n2, assign96580_e149200_d_n4, assign96580_e149200_d_n5, assign96580_e149200_d_n6, assign96580_e149200_d_n7, assign96580_e149200_d_n8, assign96580_e149200_d_n9, assign96580_e149200_d_n10, assign96580_e149200_d_n11, assign96580_e149200_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96580_e149200;
        locals.var_isbs2_sws_dn0 = assign96580_e149200_d_n0;
        locals.var_isbs2_sws_dn2 = assign96580_e149200_d_n2;
        locals.var_isbs2_sws_dn4 = assign96580_e149200_d_n4;
        locals.var_isbs2_sws_dn5 = assign96580_e149200_d_n5;
        locals.var_isbs2_sws_dn6 = assign96580_e149200_d_n6;
        locals.var_isbs2_sws_dn7 = assign96580_e149200_d_n7;
        locals.var_isbs2_sws_dn8 = assign96580_e149200_d_n8;
        locals.var_isbs2_sws_dn9 = assign96580_e149200_d_n9;
        locals.var_isbs2_sws_dn10 = assign96580_e149200_d_n10;
        locals.var_isbs2_sws_dn11 = assign96580_e149200_d_n11;
        locals.var_isbs2_sws_dn14 = assign96580_e149200_d_n14;

        let (assign96590_e149211, assign96590_e149211_d_n0, assign96590_e149211_d_n2, assign96590_e149211_d_n4, assign96590_e149211_d_n5, assign96590_e149211_d_n6, assign96590_e149211_d_n7, assign96590_e149211_d_n8, assign96590_e149211_d_n9, assign96590_e149211_d_n10, assign96590_e149211_d_n11, assign96590_e149211_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        let assign96590_e149209: f64 = (p.p16 * locals.var_jsswg);
        (assign96590_e149209, (p.p16 * locals.var_jsswg_dn0), (p.p16 * locals.var_jsswg_dn2), (p.p16 * locals.var_jsswg_dn4), (p.p16 * locals.var_jsswg_dn5), (p.p16 * locals.var_jsswg_dn6), (p.p16 * locals.var_jsswg_dn7), (p.p16 * locals.var_jsswg_dn8), (p.p16 * locals.var_jsswg_dn9), (p.p16 * locals.var_jsswg_dn10), (p.p16 * locals.var_jsswg_dn11), (p.p16 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96590_e149211;
        locals.var_isbs_swg_dn0 = assign96590_e149211_d_n0;
        locals.var_isbs_swg_dn2 = assign96590_e149211_d_n2;
        locals.var_isbs_swg_dn4 = assign96590_e149211_d_n4;
        locals.var_isbs_swg_dn5 = assign96590_e149211_d_n5;
        locals.var_isbs_swg_dn6 = assign96590_e149211_d_n6;
        locals.var_isbs_swg_dn7 = assign96590_e149211_d_n7;
        locals.var_isbs_swg_dn8 = assign96590_e149211_d_n8;
        locals.var_isbs_swg_dn9 = assign96590_e149211_d_n9;
        locals.var_isbs_swg_dn10 = assign96590_e149211_d_n10;
        locals.var_isbs_swg_dn11 = assign96590_e149211_d_n11;
        locals.var_isbs_swg_dn14 = assign96590_e149211_d_n14;

        let (assign96600_e149222, assign96600_e149222_d_n0, assign96600_e149222_d_n2, assign96600_e149222_d_n4, assign96600_e149222_d_n5, assign96600_e149222_d_n6, assign96600_e149222_d_n7, assign96600_e149222_d_n8, assign96600_e149222_d_n9, assign96600_e149222_d_n10, assign96600_e149222_d_n11, assign96600_e149222_d_n14,) = {
    if (((locals.var_guard2235 != 0.0) && (locals.var_guard2239 != 0.0)) && (locals.var_guard2240 == 0.0)) {
        let assign96600_e149220: f64 = (p.p16 * locals.var_jsswg2);
        (assign96600_e149220, (p.p16 * locals.var_jsswg2_dn0), (p.p16 * locals.var_jsswg2_dn2), (p.p16 * locals.var_jsswg2_dn4), (p.p16 * locals.var_jsswg2_dn5), (p.p16 * locals.var_jsswg2_dn6), (p.p16 * locals.var_jsswg2_dn7), (p.p16 * locals.var_jsswg2_dn8), (p.p16 * locals.var_jsswg2_dn9), (p.p16 * locals.var_jsswg2_dn10), (p.p16 * locals.var_jsswg2_dn11), (p.p16 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96600_e149222;
        locals.var_isbs2_swg_dn0 = assign96600_e149222_d_n0;
        locals.var_isbs2_swg_dn2 = assign96600_e149222_d_n2;
        locals.var_isbs2_swg_dn4 = assign96600_e149222_d_n4;
        locals.var_isbs2_swg_dn5 = assign96600_e149222_d_n5;
        locals.var_isbs2_swg_dn6 = assign96600_e149222_d_n6;
        locals.var_isbs2_swg_dn7 = assign96600_e149222_d_n7;
        locals.var_isbs2_swg_dn8 = assign96600_e149222_d_n8;
        locals.var_isbs2_swg_dn9 = assign96600_e149222_d_n9;
        locals.var_isbs2_swg_dn10 = assign96600_e149222_d_n10;
        locals.var_isbs2_swg_dn11 = assign96600_e149222_d_n11;
        locals.var_isbs2_swg_dn14 = assign96600_e149222_d_n14;

        let (assign96610_e149231, assign96610_e149231_d_n0, assign96610_e149231_d_n2, assign96610_e149231_d_n4, assign96610_e149231_d_n5, assign96610_e149231_d_n6, assign96610_e149231_d_n7, assign96610_e149231_d_n8, assign96610_e149231_d_n9, assign96610_e149231_d_n10, assign96610_e149231_d_n11, assign96610_e149231_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        let assign96610_e149229: f64 = (p.p14 * locals.var_js);
        (assign96610_e149229, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96610_e149231;
        locals.var_isbs_btm_dn0 = assign96610_e149231_d_n0;
        locals.var_isbs_btm_dn2 = assign96610_e149231_d_n2;
        locals.var_isbs_btm_dn4 = assign96610_e149231_d_n4;
        locals.var_isbs_btm_dn5 = assign96610_e149231_d_n5;
        locals.var_isbs_btm_dn6 = assign96610_e149231_d_n6;
        locals.var_isbs_btm_dn7 = assign96610_e149231_d_n7;
        locals.var_isbs_btm_dn8 = assign96610_e149231_d_n8;
        locals.var_isbs_btm_dn9 = assign96610_e149231_d_n9;
        locals.var_isbs_btm_dn10 = assign96610_e149231_d_n10;
        locals.var_isbs_btm_dn11 = assign96610_e149231_d_n11;
        locals.var_isbs_btm_dn14 = assign96610_e149231_d_n14;

        let (assign96620_e149240, assign96620_e149240_d_n0, assign96620_e149240_d_n2, assign96620_e149240_d_n4, assign96620_e149240_d_n5, assign96620_e149240_d_n6, assign96620_e149240_d_n7, assign96620_e149240_d_n8, assign96620_e149240_d_n9, assign96620_e149240_d_n10, assign96620_e149240_d_n11, assign96620_e149240_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        let assign96620_e149238: f64 = (p.p14 * locals.var_js2);
        (assign96620_e149238, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96620_e149240;
        locals.var_isbs2_btm_dn0 = assign96620_e149240_d_n0;
        locals.var_isbs2_btm_dn2 = assign96620_e149240_d_n2;
        locals.var_isbs2_btm_dn4 = assign96620_e149240_d_n4;
        locals.var_isbs2_btm_dn5 = assign96620_e149240_d_n5;
        locals.var_isbs2_btm_dn6 = assign96620_e149240_d_n6;
        locals.var_isbs2_btm_dn7 = assign96620_e149240_d_n7;
        locals.var_isbs2_btm_dn8 = assign96620_e149240_d_n8;
        locals.var_isbs2_btm_dn9 = assign96620_e149240_d_n9;
        locals.var_isbs2_btm_dn10 = assign96620_e149240_d_n10;
        locals.var_isbs2_btm_dn11 = assign96620_e149240_d_n11;
        locals.var_isbs2_btm_dn14 = assign96620_e149240_d_n14;

        let (assign96630_e149249, assign96630_e149249_d_n0, assign96630_e149249_d_n2, assign96630_e149249_d_n4, assign96630_e149249_d_n5, assign96630_e149249_d_n6, assign96630_e149249_d_n7, assign96630_e149249_d_n8, assign96630_e149249_d_n9, assign96630_e149249_d_n10, assign96630_e149249_d_n11, assign96630_e149249_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        let assign96630_e149247: f64 = (p.p16 * locals.var_jssw);
        (assign96630_e149247, (p.p16 * locals.var_jssw_dn0), (p.p16 * locals.var_jssw_dn2), (p.p16 * locals.var_jssw_dn4), (p.p16 * locals.var_jssw_dn5), (p.p16 * locals.var_jssw_dn6), (p.p16 * locals.var_jssw_dn7), (p.p16 * locals.var_jssw_dn8), (p.p16 * locals.var_jssw_dn9), (p.p16 * locals.var_jssw_dn10), (p.p16 * locals.var_jssw_dn11), (p.p16 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96630_e149249;
        locals.var_isbs_sws_dn0 = assign96630_e149249_d_n0;
        locals.var_isbs_sws_dn2 = assign96630_e149249_d_n2;
        locals.var_isbs_sws_dn4 = assign96630_e149249_d_n4;
        locals.var_isbs_sws_dn5 = assign96630_e149249_d_n5;
        locals.var_isbs_sws_dn6 = assign96630_e149249_d_n6;
        locals.var_isbs_sws_dn7 = assign96630_e149249_d_n7;
        locals.var_isbs_sws_dn8 = assign96630_e149249_d_n8;
        locals.var_isbs_sws_dn9 = assign96630_e149249_d_n9;
        locals.var_isbs_sws_dn10 = assign96630_e149249_d_n10;
        locals.var_isbs_sws_dn11 = assign96630_e149249_d_n11;
        locals.var_isbs_sws_dn14 = assign96630_e149249_d_n14;

        let (assign96640_e149258, assign96640_e149258_d_n0, assign96640_e149258_d_n2, assign96640_e149258_d_n4, assign96640_e149258_d_n5, assign96640_e149258_d_n6, assign96640_e149258_d_n7, assign96640_e149258_d_n8, assign96640_e149258_d_n9, assign96640_e149258_d_n10, assign96640_e149258_d_n11, assign96640_e149258_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        let assign96640_e149256: f64 = (p.p16 * locals.var_jssw2);
        (assign96640_e149256, (p.p16 * locals.var_jssw2_dn0), (p.p16 * locals.var_jssw2_dn2), (p.p16 * locals.var_jssw2_dn4), (p.p16 * locals.var_jssw2_dn5), (p.p16 * locals.var_jssw2_dn6), (p.p16 * locals.var_jssw2_dn7), (p.p16 * locals.var_jssw2_dn8), (p.p16 * locals.var_jssw2_dn9), (p.p16 * locals.var_jssw2_dn10), (p.p16 * locals.var_jssw2_dn11), (p.p16 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96640_e149258;
        locals.var_isbs2_sws_dn0 = assign96640_e149258_d_n0;
        locals.var_isbs2_sws_dn2 = assign96640_e149258_d_n2;
        locals.var_isbs2_sws_dn4 = assign96640_e149258_d_n4;
        locals.var_isbs2_sws_dn5 = assign96640_e149258_d_n5;
        locals.var_isbs2_sws_dn6 = assign96640_e149258_d_n6;
        locals.var_isbs2_sws_dn7 = assign96640_e149258_d_n7;
        locals.var_isbs2_sws_dn8 = assign96640_e149258_d_n8;
        locals.var_isbs2_sws_dn9 = assign96640_e149258_d_n9;
        locals.var_isbs2_sws_dn10 = assign96640_e149258_d_n10;
        locals.var_isbs2_sws_dn11 = assign96640_e149258_d_n11;
        locals.var_isbs2_sws_dn14 = assign96640_e149258_d_n14;

        let (assign96650_e149265, assign96650_e149265_d_n0, assign96650_e149265_d_n2, assign96650_e149265_d_n4, assign96650_e149265_d_n5, assign96650_e149265_d_n6, assign96650_e149265_d_n7, assign96650_e149265_d_n8, assign96650_e149265_d_n9, assign96650_e149265_d_n10, assign96650_e149265_d_n11, assign96650_e149265_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96650_e149265;
        locals.var_isbs_swg_dn0 = assign96650_e149265_d_n0;
        locals.var_isbs_swg_dn2 = assign96650_e149265_d_n2;
        locals.var_isbs_swg_dn4 = assign96650_e149265_d_n4;
        locals.var_isbs_swg_dn5 = assign96650_e149265_d_n5;
        locals.var_isbs_swg_dn6 = assign96650_e149265_d_n6;
        locals.var_isbs_swg_dn7 = assign96650_e149265_d_n7;
        locals.var_isbs_swg_dn8 = assign96650_e149265_d_n8;
        locals.var_isbs_swg_dn9 = assign96650_e149265_d_n9;
        locals.var_isbs_swg_dn10 = assign96650_e149265_d_n10;
        locals.var_isbs_swg_dn11 = assign96650_e149265_d_n11;
        locals.var_isbs_swg_dn14 = assign96650_e149265_d_n14;

        let (assign96660_e149272, assign96660_e149272_d_n0, assign96660_e149272_d_n2, assign96660_e149272_d_n4, assign96660_e149272_d_n5, assign96660_e149272_d_n6, assign96660_e149272_d_n7, assign96660_e149272_d_n8, assign96660_e149272_d_n9, assign96660_e149272_d_n10, assign96660_e149272_d_n11, assign96660_e149272_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2239 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96660_e149272;
        locals.var_isbs2_swg_dn0 = assign96660_e149272_d_n0;
        locals.var_isbs2_swg_dn2 = assign96660_e149272_d_n2;
        locals.var_isbs2_swg_dn4 = assign96660_e149272_d_n4;
        locals.var_isbs2_swg_dn5 = assign96660_e149272_d_n5;
        locals.var_isbs2_swg_dn6 = assign96660_e149272_d_n6;
        locals.var_isbs2_swg_dn7 = assign96660_e149272_d_n7;
        locals.var_isbs2_swg_dn8 = assign96660_e149272_d_n8;
        locals.var_isbs2_swg_dn9 = assign96660_e149272_d_n9;
        locals.var_isbs2_swg_dn10 = assign96660_e149272_d_n10;
        locals.var_isbs2_swg_dn11 = assign96660_e149272_d_n11;
        locals.var_isbs2_swg_dn14 = assign96660_e149272_d_n14;

        let (assign96670_e149280, assign96670_e149280_d_n0, assign96670_e149280_d_n2, assign96670_e149280_d_n4, assign96670_e149280_d_n5, assign96670_e149280_d_n6, assign96670_e149280_d_n7, assign96670_e149280_d_n8, assign96670_e149280_d_n9, assign96670_e149280_d_n10, assign96670_e149280_d_n11, assign96670_e149280_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96670_e149276: f64 = (locals.var_isbs_btm + locals.var_isbs_sws);
        let assign96670_e149278: f64 = (assign96670_e149276 + locals.var_isbs_swg);
        (assign96670_e149278, ((locals.var_isbs_btm_dn0 + locals.var_isbs_sws_dn0) + locals.var_isbs_swg_dn0), ((locals.var_isbs_btm_dn2 + locals.var_isbs_sws_dn2) + locals.var_isbs_swg_dn2), ((locals.var_isbs_btm_dn4 + locals.var_isbs_sws_dn4) + locals.var_isbs_swg_dn4), ((locals.var_isbs_btm_dn5 + locals.var_isbs_sws_dn5) + locals.var_isbs_swg_dn5), ((locals.var_isbs_btm_dn6 + locals.var_isbs_sws_dn6) + locals.var_isbs_swg_dn6), ((locals.var_isbs_btm_dn7 + locals.var_isbs_sws_dn7) + locals.var_isbs_swg_dn7), ((locals.var_isbs_btm_dn8 + locals.var_isbs_sws_dn8) + locals.var_isbs_swg_dn8), ((locals.var_isbs_btm_dn9 + locals.var_isbs_sws_dn9) + locals.var_isbs_swg_dn9), ((locals.var_isbs_btm_dn10 + locals.var_isbs_sws_dn10) + locals.var_isbs_swg_dn10), ((locals.var_isbs_btm_dn11 + locals.var_isbs_sws_dn11) + locals.var_isbs_swg_dn11), ((locals.var_isbs_btm_dn14 + locals.var_isbs_sws_dn14) + locals.var_isbs_swg_dn14),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    }
};
        locals.var_isbs = assign96670_e149280;
        locals.var_isbs_dn0 = assign96670_e149280_d_n0;
        locals.var_isbs_dn2 = assign96670_e149280_d_n2;
        locals.var_isbs_dn4 = assign96670_e149280_d_n4;
        locals.var_isbs_dn5 = assign96670_e149280_d_n5;
        locals.var_isbs_dn6 = assign96670_e149280_d_n6;
        locals.var_isbs_dn7 = assign96670_e149280_d_n7;
        locals.var_isbs_dn8 = assign96670_e149280_d_n8;
        locals.var_isbs_dn9 = assign96670_e149280_d_n9;
        locals.var_isbs_dn10 = assign96670_e149280_d_n10;
        locals.var_isbs_dn11 = assign96670_e149280_d_n11;
        locals.var_isbs_dn14 = assign96670_e149280_d_n14;

        let assign96680_e149283: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2241 = assign96680_e149283;

        let (assign96690_e149291, assign96690_e149291_d_n0, assign96690_e149291_d_n2, assign96690_e149291_d_n4, assign96690_e149291_d_n5, assign96690_e149291_d_n6, assign96690_e149291_d_n7, assign96690_e149291_d_n8, assign96690_e149291_d_n9, assign96690_e149291_d_n10, assign96690_e149291_d_n11, assign96690_e149291_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2241 != 0.0)) {
        let assign96690_e149289: f64 = (locals.var_isbs + 1e-25);
        (assign96690_e149289, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign96690_e149291;
        locals.var_t3_dn0 = assign96690_e149291_d_n0;
        locals.var_t3_dn2 = assign96690_e149291_d_n2;
        locals.var_t3_dn4 = assign96690_e149291_d_n4;
        locals.var_t3_dn5 = assign96690_e149291_d_n5;
        locals.var_t3_dn6 = assign96690_e149291_d_n6;
        locals.var_t3_dn7 = assign96690_e149291_d_n7;
        locals.var_t3_dn8 = assign96690_e149291_d_n8;
        locals.var_t3_dn9 = assign96690_e149291_d_n9;
        locals.var_t3_dn10 = assign96690_e149291_d_n10;
        locals.var_t3_dn11 = assign96690_e149291_d_n11;
        locals.var_t3_dn14 = assign96690_e149291_d_n14;

        let (assign96700_e149308, assign96700_e149308_d_n0, assign96700_e149308_d_n2, assign96700_e149308_d_n4, assign96700_e149308_d_n5, assign96700_e149308_d_n6, assign96700_e149308_d_n7, assign96700_e149308_d_n8, assign96700_e149308_d_n9, assign96700_e149308_d_n10, assign96700_e149308_d_n11, assign96700_e149308_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2241 != 0.0)) {
        let assign96700_e149297: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96700_e149300: f64 = (locals.var_uc_vdiffjs * locals.var_t0);
        let assign96700_e149302: f64 = (assign96700_e149300 / locals.var_t3);
        let assign96700_e149304: f64 = (assign96700_e149302 + 1.0);
        let assign96700_e149305: f64 = (assign96700_e149304).ln();
        let assign96700_e149306: f64 = (assign96700_e149297 * assign96700_e149305);
        (assign96700_e149306, (((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn0) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn2) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn4) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn5) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn6) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn7) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn8) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn9) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn10) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn11) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))), (((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign96700_e149305) + (assign96700_e149297 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn14) * locals.var_t3) - (assign96700_e149300 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) / assign96700_e149304))),)
    } else {
        (locals.var_vbst, locals.var_vbst_dn0, locals.var_vbst_dn2, locals.var_vbst_dn4, locals.var_vbst_dn5, locals.var_vbst_dn6, locals.var_vbst_dn7, locals.var_vbst_dn8, locals.var_vbst_dn9, locals.var_vbst_dn10, locals.var_vbst_dn11, locals.var_vbst_dn14,)
    }
};
        locals.var_vbst = assign96700_e149308;
        locals.var_vbst_dn0 = assign96700_e149308_d_n0;
        locals.var_vbst_dn2 = assign96700_e149308_d_n2;
        locals.var_vbst_dn4 = assign96700_e149308_d_n4;
        locals.var_vbst_dn5 = assign96700_e149308_d_n5;
        locals.var_vbst_dn6 = assign96700_e149308_d_n6;
        locals.var_vbst_dn7 = assign96700_e149308_d_n7;
        locals.var_vbst_dn8 = assign96700_e149308_d_n8;
        locals.var_vbst_dn9 = assign96700_e149308_d_n9;
        locals.var_vbst_dn10 = assign96700_e149308_d_n10;
        locals.var_vbst_dn11 = assign96700_e149308_d_n11;
        locals.var_vbst_dn14 = assign96700_e149308_d_n14;

        let (assign96710_e149319, assign96710_e149319_d_n0, assign96710_e149319_d_n2, assign96710_e149319_d_n4, assign96710_e149319_d_n5, assign96710_e149319_d_n6, assign96710_e149319_d_n7, assign96710_e149319_d_n8, assign96710_e149319_d_n9, assign96710_e149319_d_n10, assign96710_e149319_d_n11, assign96710_e149319_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2241 != 0.0)) {
        let assign96710_e149314: f64 = (locals.var_tratio - 1.0);
        let assign96710_e149316: f64 = (assign96710_e149314 * p.p535);
        let assign96710_e149317: f64 = (assign96710_e149316).exp();
        (assign96710_e149317, (assign96710_e149317 * (locals.var_tratio_dn0 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn2 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn4 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn5 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn6 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn7 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn8 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn9 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn10 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn11 * p.p535)), (assign96710_e149317 * (locals.var_tratio_dn14 * p.p535)),)
    } else {
        (locals.var_exptemps, locals.var_exptemps_dn0, locals.var_exptemps_dn2, locals.var_exptemps_dn4, locals.var_exptemps_dn5, locals.var_exptemps_dn6, locals.var_exptemps_dn7, locals.var_exptemps_dn8, locals.var_exptemps_dn9, locals.var_exptemps_dn10, locals.var_exptemps_dn11, locals.var_exptemps_dn14,)
    }
};
        locals.var_exptemps = assign96710_e149319;
        locals.var_exptemps_dn0 = assign96710_e149319_d_n0;
        locals.var_exptemps_dn2 = assign96710_e149319_d_n2;
        locals.var_exptemps_dn4 = assign96710_e149319_d_n4;
        locals.var_exptemps_dn5 = assign96710_e149319_d_n5;
        locals.var_exptemps_dn6 = assign96710_e149319_d_n6;
        locals.var_exptemps_dn7 = assign96710_e149319_d_n7;
        locals.var_exptemps_dn8 = assign96710_e149319_d_n8;
        locals.var_exptemps_dn9 = assign96710_e149319_d_n9;
        locals.var_exptemps_dn10 = assign96710_e149319_d_n10;
        locals.var_exptemps_dn11 = assign96710_e149319_d_n11;
        locals.var_exptemps_dn14 = assign96710_e149319_d_n14;

        let (assign96720_e149329, assign96720_e149329_d_n0, assign96720_e149329_d_n2, assign96720_e149329_d_n4, assign96720_e149329_d_n5, assign96720_e149329_d_n6, assign96720_e149329_d_n7, assign96720_e149329_d_n8, assign96720_e149329_d_n9, assign96720_e149329_d_n10, assign96720_e149329_d_n11, assign96720_e149329_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2241 != 0.0)) {
        let assign96720_e149326: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96720_e149327: f64 = (1.0 / assign96720_e149326);
        (assign96720_e149327, (-((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))), (-((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign96720_e149326 * assign96720_e149326))),)
    } else {
        (locals.var_jd_nvtm_invs, locals.var_jd_nvtm_invs_dn0, locals.var_jd_nvtm_invs_dn2, locals.var_jd_nvtm_invs_dn4, locals.var_jd_nvtm_invs_dn5, locals.var_jd_nvtm_invs_dn6, locals.var_jd_nvtm_invs_dn7, locals.var_jd_nvtm_invs_dn8, locals.var_jd_nvtm_invs_dn9, locals.var_jd_nvtm_invs_dn10, locals.var_jd_nvtm_invs_dn11, locals.var_jd_nvtm_invs_dn14,)
    }
};
        locals.var_jd_nvtm_invs = assign96720_e149329;
        locals.var_jd_nvtm_invs_dn0 = assign96720_e149329_d_n0;
        locals.var_jd_nvtm_invs_dn2 = assign96720_e149329_d_n2;
        locals.var_jd_nvtm_invs_dn4 = assign96720_e149329_d_n4;
        locals.var_jd_nvtm_invs_dn5 = assign96720_e149329_d_n5;
        locals.var_jd_nvtm_invs_dn6 = assign96720_e149329_d_n6;
        locals.var_jd_nvtm_invs_dn7 = assign96720_e149329_d_n7;
        locals.var_jd_nvtm_invs_dn8 = assign96720_e149329_d_n8;
        locals.var_jd_nvtm_invs_dn9 = assign96720_e149329_d_n9;
        locals.var_jd_nvtm_invs_dn10 = assign96720_e149329_d_n10;
        locals.var_jd_nvtm_invs_dn11 = assign96720_e149329_d_n11;
        locals.var_jd_nvtm_invs_dn14 = assign96720_e149329_d_n14;

    }

    pub(super) fn stamp_transient_block_357(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96730_e149338, assign96730_e149338_d_n0, assign96730_e149338_d_n2, assign96730_e149338_d_n4, assign96730_e149338_d_n5, assign96730_e149338_d_n6, assign96730_e149338_d_n7, assign96730_e149338_d_n8, assign96730_e149338_d_n9, assign96730_e149338_d_n10, assign96730_e149338_d_n11, assign96730_e149338_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2241 != 0.0)) {
        let assign96730_e149335: f64 = (locals.var_vbst * locals.var_jd_nvtm_invs);
        let assign96730_e149336: f64 = (assign96730_e149335).exp();
        (assign96730_e149336, (assign96730_e149336 * ((locals.var_vbst_dn0 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn0))), (assign96730_e149336 * ((locals.var_vbst_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn2))), (assign96730_e149336 * ((locals.var_vbst_dn4 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn4))), (assign96730_e149336 * ((locals.var_vbst_dn5 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn5))), (assign96730_e149336 * ((locals.var_vbst_dn6 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn6))), (assign96730_e149336 * ((locals.var_vbst_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn7))), (assign96730_e149336 * ((locals.var_vbst_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn8))), (assign96730_e149336 * ((locals.var_vbst_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn9))), (assign96730_e149336 * ((locals.var_vbst_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn10))), (assign96730_e149336 * ((locals.var_vbst_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn11))), (assign96730_e149336 * ((locals.var_vbst_dn14 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn14))),)
    } else {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    }
};
        locals.var_jd_expcs = assign96730_e149338;
        locals.var_jd_expcs_dn0 = assign96730_e149338_d_n0;
        locals.var_jd_expcs_dn2 = assign96730_e149338_d_n2;
        locals.var_jd_expcs_dn4 = assign96730_e149338_d_n4;
        locals.var_jd_expcs_dn5 = assign96730_e149338_d_n5;
        locals.var_jd_expcs_dn6 = assign96730_e149338_d_n6;
        locals.var_jd_expcs_dn7 = assign96730_e149338_d_n7;
        locals.var_jd_expcs_dn8 = assign96730_e149338_d_n8;
        locals.var_jd_expcs_dn9 = assign96730_e149338_d_n9;
        locals.var_jd_expcs_dn10 = assign96730_e149338_d_n10;
        locals.var_jd_expcs_dn11 = assign96730_e149338_d_n11;
        locals.var_jd_expcs_dn14 = assign96730_e149338_d_n14;

        let (assign96740_e149350, assign96740_e149350_d_n0, assign96740_e149350_d_n2, assign96740_e149350_d_n4, assign96740_e149350_d_n5, assign96740_e149350_d_n6, assign96740_e149350_d_n7, assign96740_e149350_d_n8, assign96740_e149350_d_n9, assign96740_e149350_d_n10, assign96740_e149350_d_n11, assign96740_e149350_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96740_e149342: f64 = (p.p500 * p.p13);
        let assign96740_e149346: f64 = (p.p481 * locals.var_tdiff);
        let assign96740_e149347: f64 = (1.0 + assign96740_e149346);
        let assign96740_e149348: f64 = (assign96740_e149342 * assign96740_e149347);
        (assign96740_e149348, (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn0)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn2)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn4)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn5)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn6)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn7)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn8)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn9)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn10)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn11)), (assign96740_e149342 * (p.p481 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign96740_e149350;
        locals.var_czbd_dn0 = assign96740_e149350_d_n0;
        locals.var_czbd_dn2 = assign96740_e149350_d_n2;
        locals.var_czbd_dn4 = assign96740_e149350_d_n4;
        locals.var_czbd_dn5 = assign96740_e149350_d_n5;
        locals.var_czbd_dn6 = assign96740_e149350_d_n6;
        locals.var_czbd_dn7 = assign96740_e149350_d_n7;
        locals.var_czbd_dn8 = assign96740_e149350_d_n8;
        locals.var_czbd_dn9 = assign96740_e149350_d_n9;
        locals.var_czbd_dn10 = assign96740_e149350_d_n10;
        locals.var_czbd_dn11 = assign96740_e149350_d_n11;
        locals.var_czbd_dn14 = assign96740_e149350_d_n14;

        let assign96750_e149353: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2242 = assign96750_e149353;

        let (assign96760_e149369, assign96760_e149369_d_n0, assign96760_e149369_d_n2, assign96760_e149369_d_n4, assign96760_e149369_d_n5, assign96760_e149369_d_n6, assign96760_e149369_d_n7, assign96760_e149369_d_n8, assign96760_e149369_d_n9, assign96760_e149369_d_n10, assign96760_e149369_d_n11, assign96760_e149369_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2242 != 0.0)) {
        let assign96760_e149360: f64 = (p.p15 - locals.var_weff_nf);
        let assign96760_e149361: f64 = (p.p501 * assign96760_e149360);
        let assign96760_e149365: f64 = (p.p483 * locals.var_tdiff);
        let assign96760_e149366: f64 = (1.0 + assign96760_e149365);
        let assign96760_e149367: f64 = (assign96760_e149361 * assign96760_e149366);
        (assign96760_e149367, (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn0)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn2)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn4)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn5)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn6)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn7)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn8)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn9)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn10)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn11)), (assign96760_e149361 * (p.p483 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96760_e149369;
        locals.var_czbdsw_dn0 = assign96760_e149369_d_n0;
        locals.var_czbdsw_dn2 = assign96760_e149369_d_n2;
        locals.var_czbdsw_dn4 = assign96760_e149369_d_n4;
        locals.var_czbdsw_dn5 = assign96760_e149369_d_n5;
        locals.var_czbdsw_dn6 = assign96760_e149369_d_n6;
        locals.var_czbdsw_dn7 = assign96760_e149369_d_n7;
        locals.var_czbdsw_dn8 = assign96760_e149369_d_n8;
        locals.var_czbdsw_dn9 = assign96760_e149369_d_n9;
        locals.var_czbdsw_dn10 = assign96760_e149369_d_n10;
        locals.var_czbdsw_dn11 = assign96760_e149369_d_n11;
        locals.var_czbdsw_dn14 = assign96760_e149369_d_n14;

        let (assign96770_e149383, assign96770_e149383_d_n0, assign96770_e149383_d_n2, assign96770_e149383_d_n4, assign96770_e149383_d_n5, assign96770_e149383_d_n6, assign96770_e149383_d_n7, assign96770_e149383_d_n8, assign96770_e149383_d_n9, assign96770_e149383_d_n10, assign96770_e149383_d_n11, assign96770_e149383_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2242 != 0.0)) {
        let assign96770_e149375: f64 = (p.p502 * locals.var_weff_nf);
        let assign96770_e149379: f64 = (p.p485 * locals.var_tdiff);
        let assign96770_e149380: f64 = (1.0 + assign96770_e149379);
        let assign96770_e149381: f64 = (assign96770_e149375 * assign96770_e149380);
        (assign96770_e149381, (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn0)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn2)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn4)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn5)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn6)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn7)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn8)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn9)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn10)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn11)), (assign96770_e149375 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96770_e149383;
        locals.var_czbdswg_dn0 = assign96770_e149383_d_n0;
        locals.var_czbdswg_dn2 = assign96770_e149383_d_n2;
        locals.var_czbdswg_dn4 = assign96770_e149383_d_n4;
        locals.var_czbdswg_dn5 = assign96770_e149383_d_n5;
        locals.var_czbdswg_dn6 = assign96770_e149383_d_n6;
        locals.var_czbdswg_dn7 = assign96770_e149383_d_n7;
        locals.var_czbdswg_dn8 = assign96770_e149383_d_n8;
        locals.var_czbdswg_dn9 = assign96770_e149383_d_n9;
        locals.var_czbdswg_dn10 = assign96770_e149383_d_n10;
        locals.var_czbdswg_dn11 = assign96770_e149383_d_n11;
        locals.var_czbdswg_dn14 = assign96770_e149383_d_n14;

        let (assign96780_e149390, assign96780_e149390_d_n0, assign96780_e149390_d_n2, assign96780_e149390_d_n4, assign96780_e149390_d_n5, assign96780_e149390_d_n6, assign96780_e149390_d_n7, assign96780_e149390_d_n8, assign96780_e149390_d_n9, assign96780_e149390_d_n10, assign96780_e149390_d_n11, assign96780_e149390_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2242 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96780_e149390;
        locals.var_czbdsw_dn0 = assign96780_e149390_d_n0;
        locals.var_czbdsw_dn2 = assign96780_e149390_d_n2;
        locals.var_czbdsw_dn4 = assign96780_e149390_d_n4;
        locals.var_czbdsw_dn5 = assign96780_e149390_d_n5;
        locals.var_czbdsw_dn6 = assign96780_e149390_d_n6;
        locals.var_czbdsw_dn7 = assign96780_e149390_d_n7;
        locals.var_czbdsw_dn8 = assign96780_e149390_d_n8;
        locals.var_czbdsw_dn9 = assign96780_e149390_d_n9;
        locals.var_czbdsw_dn10 = assign96780_e149390_d_n10;
        locals.var_czbdsw_dn11 = assign96780_e149390_d_n11;
        locals.var_czbdsw_dn14 = assign96780_e149390_d_n14;

        let (assign96790_e149405, assign96790_e149405_d_n0, assign96790_e149405_d_n2, assign96790_e149405_d_n4, assign96790_e149405_d_n5, assign96790_e149405_d_n6, assign96790_e149405_d_n7, assign96790_e149405_d_n8, assign96790_e149405_d_n9, assign96790_e149405_d_n10, assign96790_e149405_d_n11, assign96790_e149405_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2242 == 0.0)) {
        let assign96790_e149397: f64 = (p.p502 * p.p15);
        let assign96790_e149401: f64 = (p.p485 * locals.var_tdiff);
        let assign96790_e149402: f64 = (1.0 + assign96790_e149401);
        let assign96790_e149403: f64 = (assign96790_e149397 * assign96790_e149402);
        (assign96790_e149403, (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn0)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn2)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn4)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn5)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn6)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn7)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn8)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn9)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn10)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn11)), (assign96790_e149397 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96790_e149405;
        locals.var_czbdswg_dn0 = assign96790_e149405_d_n0;
        locals.var_czbdswg_dn2 = assign96790_e149405_d_n2;
        locals.var_czbdswg_dn4 = assign96790_e149405_d_n4;
        locals.var_czbdswg_dn5 = assign96790_e149405_d_n5;
        locals.var_czbdswg_dn6 = assign96790_e149405_d_n6;
        locals.var_czbdswg_dn7 = assign96790_e149405_d_n7;
        locals.var_czbdswg_dn8 = assign96790_e149405_d_n8;
        locals.var_czbdswg_dn9 = assign96790_e149405_d_n9;
        locals.var_czbdswg_dn10 = assign96790_e149405_d_n10;
        locals.var_czbdswg_dn11 = assign96790_e149405_d_n11;
        locals.var_czbdswg_dn14 = assign96790_e149405_d_n14;

        let assign96800_e149408: f64 = if locals.var_czbd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2243 = assign96800_e149408;

        let (assign96810_e149414, assign96810_e149414_d_n0, assign96810_e149414_d_n2, assign96810_e149414_d_n4, assign96810_e149414_d_n5, assign96810_e149414_d_n6, assign96810_e149414_d_n7, assign96810_e149414_d_n8, assign96810_e149414_d_n9, assign96810_e149414_d_n10, assign96810_e149414_d_n11, assign96810_e149414_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2243 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign96810_e149414;
        locals.var_czbd_dn0 = assign96810_e149414_d_n0;
        locals.var_czbd_dn2 = assign96810_e149414_d_n2;
        locals.var_czbd_dn4 = assign96810_e149414_d_n4;
        locals.var_czbd_dn5 = assign96810_e149414_d_n5;
        locals.var_czbd_dn6 = assign96810_e149414_d_n6;
        locals.var_czbd_dn7 = assign96810_e149414_d_n7;
        locals.var_czbd_dn8 = assign96810_e149414_d_n8;
        locals.var_czbd_dn9 = assign96810_e149414_d_n9;
        locals.var_czbd_dn10 = assign96810_e149414_d_n10;
        locals.var_czbd_dn11 = assign96810_e149414_d_n11;
        locals.var_czbd_dn14 = assign96810_e149414_d_n14;

        let assign96820_e149417: f64 = if locals.var_czbdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2244 = assign96820_e149417;

        let (assign96830_e149423, assign96830_e149423_d_n0, assign96830_e149423_d_n2, assign96830_e149423_d_n4, assign96830_e149423_d_n5, assign96830_e149423_d_n6, assign96830_e149423_d_n7, assign96830_e149423_d_n8, assign96830_e149423_d_n9, assign96830_e149423_d_n10, assign96830_e149423_d_n11, assign96830_e149423_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2244 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96830_e149423;
        locals.var_czbdsw_dn0 = assign96830_e149423_d_n0;
        locals.var_czbdsw_dn2 = assign96830_e149423_d_n2;
        locals.var_czbdsw_dn4 = assign96830_e149423_d_n4;
        locals.var_czbdsw_dn5 = assign96830_e149423_d_n5;
        locals.var_czbdsw_dn6 = assign96830_e149423_d_n6;
        locals.var_czbdsw_dn7 = assign96830_e149423_d_n7;
        locals.var_czbdsw_dn8 = assign96830_e149423_d_n8;
        locals.var_czbdsw_dn9 = assign96830_e149423_d_n9;
        locals.var_czbdsw_dn10 = assign96830_e149423_d_n10;
        locals.var_czbdsw_dn11 = assign96830_e149423_d_n11;
        locals.var_czbdsw_dn14 = assign96830_e149423_d_n14;

        let assign96840_e149426: f64 = if locals.var_czbdswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2245 = assign96840_e149426;

        let (assign96850_e149432, assign96850_e149432_d_n0, assign96850_e149432_d_n2, assign96850_e149432_d_n4, assign96850_e149432_d_n5, assign96850_e149432_d_n6, assign96850_e149432_d_n7, assign96850_e149432_d_n8, assign96850_e149432_d_n9, assign96850_e149432_d_n10, assign96850_e149432_d_n11, assign96850_e149432_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2245 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96850_e149432;
        locals.var_czbdswg_dn0 = assign96850_e149432_d_n0;
        locals.var_czbdswg_dn2 = assign96850_e149432_d_n2;
        locals.var_czbdswg_dn4 = assign96850_e149432_d_n4;
        locals.var_czbdswg_dn5 = assign96850_e149432_d_n5;
        locals.var_czbdswg_dn6 = assign96850_e149432_d_n6;
        locals.var_czbdswg_dn7 = assign96850_e149432_d_n7;
        locals.var_czbdswg_dn8 = assign96850_e149432_d_n8;
        locals.var_czbdswg_dn9 = assign96850_e149432_d_n9;
        locals.var_czbdswg_dn10 = assign96850_e149432_d_n10;
        locals.var_czbdswg_dn11 = assign96850_e149432_d_n11;
        locals.var_czbdswg_dn14 = assign96850_e149432_d_n14;

        let (assign96860_e149440, assign96860_e149440_d_n0, assign96860_e149440_d_n2, assign96860_e149440_d_n4, assign96860_e149440_d_n5, assign96860_e149440_d_n6, assign96860_e149440_d_n7, assign96860_e149440_d_n8, assign96860_e149440_d_n9, assign96860_e149440_d_n10, assign96860_e149440_d_n11, assign96860_e149440_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96860_e149437: f64 = (p.p487 * locals.var_tdiff);
        let assign96860_e149438: f64 = (p.p506 - assign96860_e149437);
        (assign96860_e149438, (-(p.p487 * locals.var_tdiff_dn0)), (-(p.p487 * locals.var_tdiff_dn2)), (-(p.p487 * locals.var_tdiff_dn4)), (-(p.p487 * locals.var_tdiff_dn5)), (-(p.p487 * locals.var_tdiff_dn6)), (-(p.p487 * locals.var_tdiff_dn7)), (-(p.p487 * locals.var_tdiff_dn8)), (-(p.p487 * locals.var_tdiff_dn9)), (-(p.p487 * locals.var_tdiff_dn10)), (-(p.p487 * locals.var_tdiff_dn11)), (-(p.p487 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign96860_e149440;
        locals.var_pzbd_dn0 = assign96860_e149440_d_n0;
        locals.var_pzbd_dn2 = assign96860_e149440_d_n2;
        locals.var_pzbd_dn4 = assign96860_e149440_d_n4;
        locals.var_pzbd_dn5 = assign96860_e149440_d_n5;
        locals.var_pzbd_dn6 = assign96860_e149440_d_n6;
        locals.var_pzbd_dn7 = assign96860_e149440_d_n7;
        locals.var_pzbd_dn8 = assign96860_e149440_d_n8;
        locals.var_pzbd_dn9 = assign96860_e149440_d_n9;
        locals.var_pzbd_dn10 = assign96860_e149440_d_n10;
        locals.var_pzbd_dn11 = assign96860_e149440_d_n11;
        locals.var_pzbd_dn14 = assign96860_e149440_d_n14;

        let (assign96870_e149448, assign96870_e149448_d_n0, assign96870_e149448_d_n2, assign96870_e149448_d_n4, assign96870_e149448_d_n5, assign96870_e149448_d_n6, assign96870_e149448_d_n7, assign96870_e149448_d_n8, assign96870_e149448_d_n9, assign96870_e149448_d_n10, assign96870_e149448_d_n11, assign96870_e149448_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96870_e149445: f64 = (p.p489 * locals.var_tdiff);
        let assign96870_e149446: f64 = (p.p507 - assign96870_e149445);
        (assign96870_e149446, (-(p.p489 * locals.var_tdiff_dn0)), (-(p.p489 * locals.var_tdiff_dn2)), (-(p.p489 * locals.var_tdiff_dn4)), (-(p.p489 * locals.var_tdiff_dn5)), (-(p.p489 * locals.var_tdiff_dn6)), (-(p.p489 * locals.var_tdiff_dn7)), (-(p.p489 * locals.var_tdiff_dn8)), (-(p.p489 * locals.var_tdiff_dn9)), (-(p.p489 * locals.var_tdiff_dn10)), (-(p.p489 * locals.var_tdiff_dn11)), (-(p.p489 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign96870_e149448;
        locals.var_pzbdsw_dn0 = assign96870_e149448_d_n0;
        locals.var_pzbdsw_dn2 = assign96870_e149448_d_n2;
        locals.var_pzbdsw_dn4 = assign96870_e149448_d_n4;
        locals.var_pzbdsw_dn5 = assign96870_e149448_d_n5;
        locals.var_pzbdsw_dn6 = assign96870_e149448_d_n6;
        locals.var_pzbdsw_dn7 = assign96870_e149448_d_n7;
        locals.var_pzbdsw_dn8 = assign96870_e149448_d_n8;
        locals.var_pzbdsw_dn9 = assign96870_e149448_d_n9;
        locals.var_pzbdsw_dn10 = assign96870_e149448_d_n10;
        locals.var_pzbdsw_dn11 = assign96870_e149448_d_n11;
        locals.var_pzbdsw_dn14 = assign96870_e149448_d_n14;

        let (assign96880_e149456, assign96880_e149456_d_n0, assign96880_e149456_d_n2, assign96880_e149456_d_n4, assign96880_e149456_d_n5, assign96880_e149456_d_n6, assign96880_e149456_d_n7, assign96880_e149456_d_n8, assign96880_e149456_d_n9, assign96880_e149456_d_n10, assign96880_e149456_d_n11, assign96880_e149456_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96880_e149453: f64 = (p.p491 * locals.var_tdiff);
        let assign96880_e149454: f64 = (p.p508 - assign96880_e149453);
        (assign96880_e149454, (-(p.p491 * locals.var_tdiff_dn0)), (-(p.p491 * locals.var_tdiff_dn2)), (-(p.p491 * locals.var_tdiff_dn4)), (-(p.p491 * locals.var_tdiff_dn5)), (-(p.p491 * locals.var_tdiff_dn6)), (-(p.p491 * locals.var_tdiff_dn7)), (-(p.p491 * locals.var_tdiff_dn8)), (-(p.p491 * locals.var_tdiff_dn9)), (-(p.p491 * locals.var_tdiff_dn10)), (-(p.p491 * locals.var_tdiff_dn11)), (-(p.p491 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign96880_e149456;
        locals.var_pzbdswg_dn0 = assign96880_e149456_d_n0;
        locals.var_pzbdswg_dn2 = assign96880_e149456_d_n2;
        locals.var_pzbdswg_dn4 = assign96880_e149456_d_n4;
        locals.var_pzbdswg_dn5 = assign96880_e149456_d_n5;
        locals.var_pzbdswg_dn6 = assign96880_e149456_d_n6;
        locals.var_pzbdswg_dn7 = assign96880_e149456_d_n7;
        locals.var_pzbdswg_dn8 = assign96880_e149456_d_n8;
        locals.var_pzbdswg_dn9 = assign96880_e149456_d_n9;
        locals.var_pzbdswg_dn10 = assign96880_e149456_d_n10;
        locals.var_pzbdswg_dn11 = assign96880_e149456_d_n11;
        locals.var_pzbdswg_dn14 = assign96880_e149456_d_n14;

        let assign96890_e149463: f64 = if ((locals.var_pzbd < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2246 = assign96890_e149463;

        let (assign96900_e149469, assign96900_e149469_d_n0, assign96900_e149469_d_n2, assign96900_e149469_d_n4, assign96900_e149469_d_n5, assign96900_e149469_d_n6, assign96900_e149469_d_n7, assign96900_e149469_d_n8, assign96900_e149469_d_n9, assign96900_e149469_d_n10, assign96900_e149469_d_n11, assign96900_e149469_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2246 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign96900_e149469;
        locals.var_pzbd_dn0 = assign96900_e149469_d_n0;
        locals.var_pzbd_dn2 = assign96900_e149469_d_n2;
        locals.var_pzbd_dn4 = assign96900_e149469_d_n4;
        locals.var_pzbd_dn5 = assign96900_e149469_d_n5;
        locals.var_pzbd_dn6 = assign96900_e149469_d_n6;
        locals.var_pzbd_dn7 = assign96900_e149469_d_n7;
        locals.var_pzbd_dn8 = assign96900_e149469_d_n8;
        locals.var_pzbd_dn9 = assign96900_e149469_d_n9;
        locals.var_pzbd_dn10 = assign96900_e149469_d_n10;
        locals.var_pzbd_dn11 = assign96900_e149469_d_n11;
        locals.var_pzbd_dn14 = assign96900_e149469_d_n14;

        let assign96910_e149476: f64 = if ((locals.var_pzbdsw < 0.01) && (p.p15 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2247 = assign96910_e149476;

        let (assign96920_e149482, assign96920_e149482_d_n0, assign96920_e149482_d_n2, assign96920_e149482_d_n4, assign96920_e149482_d_n5, assign96920_e149482_d_n6, assign96920_e149482_d_n7, assign96920_e149482_d_n8, assign96920_e149482_d_n9, assign96920_e149482_d_n10, assign96920_e149482_d_n11, assign96920_e149482_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2247 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign96920_e149482;
        locals.var_pzbdsw_dn0 = assign96920_e149482_d_n0;
        locals.var_pzbdsw_dn2 = assign96920_e149482_d_n2;
        locals.var_pzbdsw_dn4 = assign96920_e149482_d_n4;
        locals.var_pzbdsw_dn5 = assign96920_e149482_d_n5;
        locals.var_pzbdsw_dn6 = assign96920_e149482_d_n6;
        locals.var_pzbdsw_dn7 = assign96920_e149482_d_n7;
        locals.var_pzbdsw_dn8 = assign96920_e149482_d_n8;
        locals.var_pzbdsw_dn9 = assign96920_e149482_d_n9;
        locals.var_pzbdsw_dn10 = assign96920_e149482_d_n10;
        locals.var_pzbdsw_dn11 = assign96920_e149482_d_n11;
        locals.var_pzbdsw_dn14 = assign96920_e149482_d_n14;

        let assign96930_e149489: f64 = if ((locals.var_pzbdswg < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2248 = assign96930_e149489;

        let (assign96940_e149495, assign96940_e149495_d_n0, assign96940_e149495_d_n2, assign96940_e149495_d_n4, assign96940_e149495_d_n5, assign96940_e149495_d_n6, assign96940_e149495_d_n7, assign96940_e149495_d_n8, assign96940_e149495_d_n9, assign96940_e149495_d_n10, assign96940_e149495_d_n11, assign96940_e149495_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2248 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign96940_e149495;
        locals.var_pzbdswg_dn0 = assign96940_e149495_d_n0;
        locals.var_pzbdswg_dn2 = assign96940_e149495_d_n2;
        locals.var_pzbdswg_dn4 = assign96940_e149495_d_n4;
        locals.var_pzbdswg_dn5 = assign96940_e149495_d_n5;
        locals.var_pzbdswg_dn6 = assign96940_e149495_d_n6;
        locals.var_pzbdswg_dn7 = assign96940_e149495_d_n7;
        locals.var_pzbdswg_dn8 = assign96940_e149495_d_n8;
        locals.var_pzbdswg_dn9 = assign96940_e149495_d_n9;
        locals.var_pzbdswg_dn10 = assign96940_e149495_d_n10;
        locals.var_pzbdswg_dn11 = assign96940_e149495_d_n11;
        locals.var_pzbdswg_dn14 = assign96940_e149495_d_n14;

        let (assign96950_e149507, assign96950_e149507_d_n0, assign96950_e149507_d_n2, assign96950_e149507_d_n4, assign96950_e149507_d_n5, assign96950_e149507_d_n6, assign96950_e149507_d_n7, assign96950_e149507_d_n8, assign96950_e149507_d_n9, assign96950_e149507_d_n10, assign96950_e149507_d_n11, assign96950_e149507_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign96950_e149499: f64 = (p.p523 * p.p14);
        let assign96950_e149503: f64 = (p.p482 * locals.var_tdiff);
        let assign96950_e149504: f64 = (1.0 + assign96950_e149503);
        let assign96950_e149505: f64 = (assign96950_e149499 * assign96950_e149504);
        (assign96950_e149505, (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn0)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn2)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn4)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn5)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn6)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn7)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn8)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn9)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn10)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn11)), (assign96950_e149499 * (p.p482 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign96950_e149507;
        locals.var_czbs_dn0 = assign96950_e149507_d_n0;
        locals.var_czbs_dn2 = assign96950_e149507_d_n2;
        locals.var_czbs_dn4 = assign96950_e149507_d_n4;
        locals.var_czbs_dn5 = assign96950_e149507_d_n5;
        locals.var_czbs_dn6 = assign96950_e149507_d_n6;
        locals.var_czbs_dn7 = assign96950_e149507_d_n7;
        locals.var_czbs_dn8 = assign96950_e149507_d_n8;
        locals.var_czbs_dn9 = assign96950_e149507_d_n9;
        locals.var_czbs_dn10 = assign96950_e149507_d_n10;
        locals.var_czbs_dn11 = assign96950_e149507_d_n11;
        locals.var_czbs_dn14 = assign96950_e149507_d_n14;

        let assign96960_e149510: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2249 = assign96960_e149510;

        let (assign96970_e149526, assign96970_e149526_d_n0, assign96970_e149526_d_n2, assign96970_e149526_d_n4, assign96970_e149526_d_n5, assign96970_e149526_d_n6, assign96970_e149526_d_n7, assign96970_e149526_d_n8, assign96970_e149526_d_n9, assign96970_e149526_d_n10, assign96970_e149526_d_n11, assign96970_e149526_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2249 != 0.0)) {
        let assign96970_e149517: f64 = (p.p16 - locals.var_weff_nf);
        let assign96970_e149518: f64 = (p.p524 * assign96970_e149517);
        let assign96970_e149522: f64 = (p.p484 * locals.var_tdiff);
        let assign96970_e149523: f64 = (1.0 + assign96970_e149522);
        let assign96970_e149524: f64 = (assign96970_e149518 * assign96970_e149523);
        (assign96970_e149524, (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn0)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn2)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn4)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn5)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn6)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn7)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn8)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn9)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn10)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn11)), (assign96970_e149518 * (p.p484 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign96970_e149526;
        locals.var_czbssw_dn0 = assign96970_e149526_d_n0;
        locals.var_czbssw_dn2 = assign96970_e149526_d_n2;
        locals.var_czbssw_dn4 = assign96970_e149526_d_n4;
        locals.var_czbssw_dn5 = assign96970_e149526_d_n5;
        locals.var_czbssw_dn6 = assign96970_e149526_d_n6;
        locals.var_czbssw_dn7 = assign96970_e149526_d_n7;
        locals.var_czbssw_dn8 = assign96970_e149526_d_n8;
        locals.var_czbssw_dn9 = assign96970_e149526_d_n9;
        locals.var_czbssw_dn10 = assign96970_e149526_d_n10;
        locals.var_czbssw_dn11 = assign96970_e149526_d_n11;
        locals.var_czbssw_dn14 = assign96970_e149526_d_n14;

        let (assign96980_e149540, assign96980_e149540_d_n0, assign96980_e149540_d_n2, assign96980_e149540_d_n4, assign96980_e149540_d_n5, assign96980_e149540_d_n6, assign96980_e149540_d_n7, assign96980_e149540_d_n8, assign96980_e149540_d_n9, assign96980_e149540_d_n10, assign96980_e149540_d_n11, assign96980_e149540_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2249 != 0.0)) {
        let assign96980_e149532: f64 = (p.p525 * locals.var_weff_nf);
        let assign96980_e149536: f64 = (p.p486 * locals.var_tdiff);
        let assign96980_e149537: f64 = (1.0 + assign96980_e149536);
        let assign96980_e149538: f64 = (assign96980_e149532 * assign96980_e149537);
        (assign96980_e149538, (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn0)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn2)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn4)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn5)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn6)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn7)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn8)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn9)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn10)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn11)), (assign96980_e149532 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign96980_e149540;
        locals.var_czbsswg_dn0 = assign96980_e149540_d_n0;
        locals.var_czbsswg_dn2 = assign96980_e149540_d_n2;
        locals.var_czbsswg_dn4 = assign96980_e149540_d_n4;
        locals.var_czbsswg_dn5 = assign96980_e149540_d_n5;
        locals.var_czbsswg_dn6 = assign96980_e149540_d_n6;
        locals.var_czbsswg_dn7 = assign96980_e149540_d_n7;
        locals.var_czbsswg_dn8 = assign96980_e149540_d_n8;
        locals.var_czbsswg_dn9 = assign96980_e149540_d_n9;
        locals.var_czbsswg_dn10 = assign96980_e149540_d_n10;
        locals.var_czbsswg_dn11 = assign96980_e149540_d_n11;
        locals.var_czbsswg_dn14 = assign96980_e149540_d_n14;

        let (assign96990_e149547, assign96990_e149547_d_n0, assign96990_e149547_d_n2, assign96990_e149547_d_n4, assign96990_e149547_d_n5, assign96990_e149547_d_n6, assign96990_e149547_d_n7, assign96990_e149547_d_n8, assign96990_e149547_d_n9, assign96990_e149547_d_n10, assign96990_e149547_d_n11, assign96990_e149547_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2249 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign96990_e149547;
        locals.var_czbssw_dn0 = assign96990_e149547_d_n0;
        locals.var_czbssw_dn2 = assign96990_e149547_d_n2;
        locals.var_czbssw_dn4 = assign96990_e149547_d_n4;
        locals.var_czbssw_dn5 = assign96990_e149547_d_n5;
        locals.var_czbssw_dn6 = assign96990_e149547_d_n6;
        locals.var_czbssw_dn7 = assign96990_e149547_d_n7;
        locals.var_czbssw_dn8 = assign96990_e149547_d_n8;
        locals.var_czbssw_dn9 = assign96990_e149547_d_n9;
        locals.var_czbssw_dn10 = assign96990_e149547_d_n10;
        locals.var_czbssw_dn11 = assign96990_e149547_d_n11;
        locals.var_czbssw_dn14 = assign96990_e149547_d_n14;

        let (assign97000_e149562, assign97000_e149562_d_n0, assign97000_e149562_d_n2, assign97000_e149562_d_n4, assign97000_e149562_d_n5, assign97000_e149562_d_n6, assign97000_e149562_d_n7, assign97000_e149562_d_n8, assign97000_e149562_d_n9, assign97000_e149562_d_n10, assign97000_e149562_d_n11, assign97000_e149562_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2249 == 0.0)) {
        let assign97000_e149554: f64 = (p.p525 * p.p16);
        let assign97000_e149558: f64 = (p.p486 * locals.var_tdiff);
        let assign97000_e149559: f64 = (1.0 + assign97000_e149558);
        let assign97000_e149560: f64 = (assign97000_e149554 * assign97000_e149559);
        (assign97000_e149560, (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn0)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn2)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn4)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn5)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn6)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn7)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn8)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn9)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn10)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn11)), (assign97000_e149554 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign97000_e149562;
        locals.var_czbsswg_dn0 = assign97000_e149562_d_n0;
        locals.var_czbsswg_dn2 = assign97000_e149562_d_n2;
        locals.var_czbsswg_dn4 = assign97000_e149562_d_n4;
        locals.var_czbsswg_dn5 = assign97000_e149562_d_n5;
        locals.var_czbsswg_dn6 = assign97000_e149562_d_n6;
        locals.var_czbsswg_dn7 = assign97000_e149562_d_n7;
        locals.var_czbsswg_dn8 = assign97000_e149562_d_n8;
        locals.var_czbsswg_dn9 = assign97000_e149562_d_n9;
        locals.var_czbsswg_dn10 = assign97000_e149562_d_n10;
        locals.var_czbsswg_dn11 = assign97000_e149562_d_n11;
        locals.var_czbsswg_dn14 = assign97000_e149562_d_n14;

        let assign97010_e149565: f64 = if locals.var_czbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2250 = assign97010_e149565;

        let (assign97020_e149571, assign97020_e149571_d_n0, assign97020_e149571_d_n2, assign97020_e149571_d_n4, assign97020_e149571_d_n5, assign97020_e149571_d_n6, assign97020_e149571_d_n7, assign97020_e149571_d_n8, assign97020_e149571_d_n9, assign97020_e149571_d_n10, assign97020_e149571_d_n11, assign97020_e149571_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2250 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign97020_e149571;
        locals.var_czbs_dn0 = assign97020_e149571_d_n0;
        locals.var_czbs_dn2 = assign97020_e149571_d_n2;
        locals.var_czbs_dn4 = assign97020_e149571_d_n4;
        locals.var_czbs_dn5 = assign97020_e149571_d_n5;
        locals.var_czbs_dn6 = assign97020_e149571_d_n6;
        locals.var_czbs_dn7 = assign97020_e149571_d_n7;
        locals.var_czbs_dn8 = assign97020_e149571_d_n8;
        locals.var_czbs_dn9 = assign97020_e149571_d_n9;
        locals.var_czbs_dn10 = assign97020_e149571_d_n10;
        locals.var_czbs_dn11 = assign97020_e149571_d_n11;
        locals.var_czbs_dn14 = assign97020_e149571_d_n14;

        let assign97030_e149574: f64 = if locals.var_czbssw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2251 = assign97030_e149574;

        let (assign97040_e149580, assign97040_e149580_d_n0, assign97040_e149580_d_n2, assign97040_e149580_d_n4, assign97040_e149580_d_n5, assign97040_e149580_d_n6, assign97040_e149580_d_n7, assign97040_e149580_d_n8, assign97040_e149580_d_n9, assign97040_e149580_d_n10, assign97040_e149580_d_n11, assign97040_e149580_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2251 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign97040_e149580;
        locals.var_czbssw_dn0 = assign97040_e149580_d_n0;
        locals.var_czbssw_dn2 = assign97040_e149580_d_n2;
        locals.var_czbssw_dn4 = assign97040_e149580_d_n4;
        locals.var_czbssw_dn5 = assign97040_e149580_d_n5;
        locals.var_czbssw_dn6 = assign97040_e149580_d_n6;
        locals.var_czbssw_dn7 = assign97040_e149580_d_n7;
        locals.var_czbssw_dn8 = assign97040_e149580_d_n8;
        locals.var_czbssw_dn9 = assign97040_e149580_d_n9;
        locals.var_czbssw_dn10 = assign97040_e149580_d_n10;
        locals.var_czbssw_dn11 = assign97040_e149580_d_n11;
        locals.var_czbssw_dn14 = assign97040_e149580_d_n14;

    }

    pub(super) fn stamp_transient_block_358(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign97050_e149583: f64 = if locals.var_czbsswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2252 = assign97050_e149583;

        let (assign97060_e149589, assign97060_e149589_d_n0, assign97060_e149589_d_n2, assign97060_e149589_d_n4, assign97060_e149589_d_n5, assign97060_e149589_d_n6, assign97060_e149589_d_n7, assign97060_e149589_d_n8, assign97060_e149589_d_n9, assign97060_e149589_d_n10, assign97060_e149589_d_n11, assign97060_e149589_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2252 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign97060_e149589;
        locals.var_czbsswg_dn0 = assign97060_e149589_d_n0;
        locals.var_czbsswg_dn2 = assign97060_e149589_d_n2;
        locals.var_czbsswg_dn4 = assign97060_e149589_d_n4;
        locals.var_czbsswg_dn5 = assign97060_e149589_d_n5;
        locals.var_czbsswg_dn6 = assign97060_e149589_d_n6;
        locals.var_czbsswg_dn7 = assign97060_e149589_d_n7;
        locals.var_czbsswg_dn8 = assign97060_e149589_d_n8;
        locals.var_czbsswg_dn9 = assign97060_e149589_d_n9;
        locals.var_czbsswg_dn10 = assign97060_e149589_d_n10;
        locals.var_czbsswg_dn11 = assign97060_e149589_d_n11;
        locals.var_czbsswg_dn14 = assign97060_e149589_d_n14;

        let (assign97070_e149597, assign97070_e149597_d_n0, assign97070_e149597_d_n2, assign97070_e149597_d_n4, assign97070_e149597_d_n5, assign97070_e149597_d_n6, assign97070_e149597_d_n7, assign97070_e149597_d_n8, assign97070_e149597_d_n9, assign97070_e149597_d_n10, assign97070_e149597_d_n11, assign97070_e149597_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign97070_e149594: f64 = (p.p488 * locals.var_tdiff);
        let assign97070_e149595: f64 = (p.p529 - assign97070_e149594);
        (assign97070_e149595, (-(p.p488 * locals.var_tdiff_dn0)), (-(p.p488 * locals.var_tdiff_dn2)), (-(p.p488 * locals.var_tdiff_dn4)), (-(p.p488 * locals.var_tdiff_dn5)), (-(p.p488 * locals.var_tdiff_dn6)), (-(p.p488 * locals.var_tdiff_dn7)), (-(p.p488 * locals.var_tdiff_dn8)), (-(p.p488 * locals.var_tdiff_dn9)), (-(p.p488 * locals.var_tdiff_dn10)), (-(p.p488 * locals.var_tdiff_dn11)), (-(p.p488 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign97070_e149597;
        locals.var_pzbs_dn0 = assign97070_e149597_d_n0;
        locals.var_pzbs_dn2 = assign97070_e149597_d_n2;
        locals.var_pzbs_dn4 = assign97070_e149597_d_n4;
        locals.var_pzbs_dn5 = assign97070_e149597_d_n5;
        locals.var_pzbs_dn6 = assign97070_e149597_d_n6;
        locals.var_pzbs_dn7 = assign97070_e149597_d_n7;
        locals.var_pzbs_dn8 = assign97070_e149597_d_n8;
        locals.var_pzbs_dn9 = assign97070_e149597_d_n9;
        locals.var_pzbs_dn10 = assign97070_e149597_d_n10;
        locals.var_pzbs_dn11 = assign97070_e149597_d_n11;
        locals.var_pzbs_dn14 = assign97070_e149597_d_n14;

        let (assign97080_e149605, assign97080_e149605_d_n0, assign97080_e149605_d_n2, assign97080_e149605_d_n4, assign97080_e149605_d_n5, assign97080_e149605_d_n6, assign97080_e149605_d_n7, assign97080_e149605_d_n8, assign97080_e149605_d_n9, assign97080_e149605_d_n10, assign97080_e149605_d_n11, assign97080_e149605_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign97080_e149602: f64 = (p.p490 * locals.var_tdiff);
        let assign97080_e149603: f64 = (p.p530 - assign97080_e149602);
        (assign97080_e149603, (-(p.p490 * locals.var_tdiff_dn0)), (-(p.p490 * locals.var_tdiff_dn2)), (-(p.p490 * locals.var_tdiff_dn4)), (-(p.p490 * locals.var_tdiff_dn5)), (-(p.p490 * locals.var_tdiff_dn6)), (-(p.p490 * locals.var_tdiff_dn7)), (-(p.p490 * locals.var_tdiff_dn8)), (-(p.p490 * locals.var_tdiff_dn9)), (-(p.p490 * locals.var_tdiff_dn10)), (-(p.p490 * locals.var_tdiff_dn11)), (-(p.p490 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign97080_e149605;
        locals.var_pzbssw_dn0 = assign97080_e149605_d_n0;
        locals.var_pzbssw_dn2 = assign97080_e149605_d_n2;
        locals.var_pzbssw_dn4 = assign97080_e149605_d_n4;
        locals.var_pzbssw_dn5 = assign97080_e149605_d_n5;
        locals.var_pzbssw_dn6 = assign97080_e149605_d_n6;
        locals.var_pzbssw_dn7 = assign97080_e149605_d_n7;
        locals.var_pzbssw_dn8 = assign97080_e149605_d_n8;
        locals.var_pzbssw_dn9 = assign97080_e149605_d_n9;
        locals.var_pzbssw_dn10 = assign97080_e149605_d_n10;
        locals.var_pzbssw_dn11 = assign97080_e149605_d_n11;
        locals.var_pzbssw_dn14 = assign97080_e149605_d_n14;

        let (assign97090_e149613, assign97090_e149613_d_n0, assign97090_e149613_d_n2, assign97090_e149613_d_n4, assign97090_e149613_d_n5, assign97090_e149613_d_n6, assign97090_e149613_d_n7, assign97090_e149613_d_n8, assign97090_e149613_d_n9, assign97090_e149613_d_n10, assign97090_e149613_d_n11, assign97090_e149613_d_n14,) = {
    if (locals.var_guard2235 != 0.0) {
        let assign97090_e149610: f64 = (p.p492 * locals.var_tdiff);
        let assign97090_e149611: f64 = (p.p531 - assign97090_e149610);
        (assign97090_e149611, (-(p.p492 * locals.var_tdiff_dn0)), (-(p.p492 * locals.var_tdiff_dn2)), (-(p.p492 * locals.var_tdiff_dn4)), (-(p.p492 * locals.var_tdiff_dn5)), (-(p.p492 * locals.var_tdiff_dn6)), (-(p.p492 * locals.var_tdiff_dn7)), (-(p.p492 * locals.var_tdiff_dn8)), (-(p.p492 * locals.var_tdiff_dn9)), (-(p.p492 * locals.var_tdiff_dn10)), (-(p.p492 * locals.var_tdiff_dn11)), (-(p.p492 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign97090_e149613;
        locals.var_pzbsswg_dn0 = assign97090_e149613_d_n0;
        locals.var_pzbsswg_dn2 = assign97090_e149613_d_n2;
        locals.var_pzbsswg_dn4 = assign97090_e149613_d_n4;
        locals.var_pzbsswg_dn5 = assign97090_e149613_d_n5;
        locals.var_pzbsswg_dn6 = assign97090_e149613_d_n6;
        locals.var_pzbsswg_dn7 = assign97090_e149613_d_n7;
        locals.var_pzbsswg_dn8 = assign97090_e149613_d_n8;
        locals.var_pzbsswg_dn9 = assign97090_e149613_d_n9;
        locals.var_pzbsswg_dn10 = assign97090_e149613_d_n10;
        locals.var_pzbsswg_dn11 = assign97090_e149613_d_n11;
        locals.var_pzbsswg_dn14 = assign97090_e149613_d_n14;

        let assign97100_e149620: f64 = if ((locals.var_pzbs < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2253 = assign97100_e149620;

        let (assign97110_e149626, assign97110_e149626_d_n0, assign97110_e149626_d_n2, assign97110_e149626_d_n4, assign97110_e149626_d_n5, assign97110_e149626_d_n6, assign97110_e149626_d_n7, assign97110_e149626_d_n8, assign97110_e149626_d_n9, assign97110_e149626_d_n10, assign97110_e149626_d_n11, assign97110_e149626_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2253 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign97110_e149626;
        locals.var_pzbs_dn0 = assign97110_e149626_d_n0;
        locals.var_pzbs_dn2 = assign97110_e149626_d_n2;
        locals.var_pzbs_dn4 = assign97110_e149626_d_n4;
        locals.var_pzbs_dn5 = assign97110_e149626_d_n5;
        locals.var_pzbs_dn6 = assign97110_e149626_d_n6;
        locals.var_pzbs_dn7 = assign97110_e149626_d_n7;
        locals.var_pzbs_dn8 = assign97110_e149626_d_n8;
        locals.var_pzbs_dn9 = assign97110_e149626_d_n9;
        locals.var_pzbs_dn10 = assign97110_e149626_d_n10;
        locals.var_pzbs_dn11 = assign97110_e149626_d_n11;
        locals.var_pzbs_dn14 = assign97110_e149626_d_n14;

        let assign97120_e149633: f64 = if ((locals.var_pzbssw < 0.01) && (p.p16 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2254 = assign97120_e149633;

        let (assign97130_e149639, assign97130_e149639_d_n0, assign97130_e149639_d_n2, assign97130_e149639_d_n4, assign97130_e149639_d_n5, assign97130_e149639_d_n6, assign97130_e149639_d_n7, assign97130_e149639_d_n8, assign97130_e149639_d_n9, assign97130_e149639_d_n10, assign97130_e149639_d_n11, assign97130_e149639_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2254 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign97130_e149639;
        locals.var_pzbssw_dn0 = assign97130_e149639_d_n0;
        locals.var_pzbssw_dn2 = assign97130_e149639_d_n2;
        locals.var_pzbssw_dn4 = assign97130_e149639_d_n4;
        locals.var_pzbssw_dn5 = assign97130_e149639_d_n5;
        locals.var_pzbssw_dn6 = assign97130_e149639_d_n6;
        locals.var_pzbssw_dn7 = assign97130_e149639_d_n7;
        locals.var_pzbssw_dn8 = assign97130_e149639_d_n8;
        locals.var_pzbssw_dn9 = assign97130_e149639_d_n9;
        locals.var_pzbssw_dn10 = assign97130_e149639_d_n10;
        locals.var_pzbssw_dn11 = assign97130_e149639_d_n11;
        locals.var_pzbssw_dn14 = assign97130_e149639_d_n14;

        let assign97140_e149646: f64 = if ((locals.var_pzbsswg < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2255 = assign97140_e149646;

        let (assign97150_e149652, assign97150_e149652_d_n0, assign97150_e149652_d_n2, assign97150_e149652_d_n4, assign97150_e149652_d_n5, assign97150_e149652_d_n6, assign97150_e149652_d_n7, assign97150_e149652_d_n8, assign97150_e149652_d_n9, assign97150_e149652_d_n10, assign97150_e149652_d_n11, assign97150_e149652_d_n14,) = {
    if ((locals.var_guard2235 != 0.0) && (locals.var_guard2255 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign97150_e149652;
        locals.var_pzbsswg_dn0 = assign97150_e149652_d_n0;
        locals.var_pzbsswg_dn2 = assign97150_e149652_d_n2;
        locals.var_pzbsswg_dn4 = assign97150_e149652_d_n4;
        locals.var_pzbsswg_dn5 = assign97150_e149652_d_n5;
        locals.var_pzbsswg_dn6 = assign97150_e149652_d_n6;
        locals.var_pzbsswg_dn7 = assign97150_e149652_d_n7;
        locals.var_pzbsswg_dn8 = assign97150_e149652_d_n8;
        locals.var_pzbsswg_dn9 = assign97150_e149652_d_n9;
        locals.var_pzbsswg_dn10 = assign97150_e149652_d_n10;
        locals.var_pzbsswg_dn11 = assign97150_e149652_d_n11;
        locals.var_pzbsswg_dn14 = assign97150_e149652_d_n14;

        let (assign97160_e149659, assign97160_e149659_d_n0, assign97160_e149659_d_n2, assign97160_e149659_d_n4, assign97160_e149659_d_n5, assign97160_e149659_d_n6, assign97160_e149659_d_n7, assign97160_e149659_d_n8, assign97160_e149659_d_n9, assign97160_e149659_d_n10, assign97160_e149659_d_n11, assign97160_e149659_d_n14,) = {
    if (locals.var_guard2235 == 0.0) {
        let assign97160_e149655: f64 = ctx_temp;
        let assign97160_e149657: f64 = (assign97160_e149655 + p.p11);
        (assign97160_e149657, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign97160_e149659;
        locals.var_ttemp_dn0 = assign97160_e149659_d_n0;
        locals.var_ttemp_dn2 = assign97160_e149659_d_n2;
        locals.var_ttemp_dn4 = assign97160_e149659_d_n4;
        locals.var_ttemp_dn5 = assign97160_e149659_d_n5;
        locals.var_ttemp_dn6 = assign97160_e149659_d_n6;
        locals.var_ttemp_dn7 = assign97160_e149659_d_n7;
        locals.var_ttemp_dn8 = assign97160_e149659_d_n8;
        locals.var_ttemp_dn9 = assign97160_e149659_d_n9;
        locals.var_ttemp_dn10 = assign97160_e149659_d_n10;
        locals.var_ttemp_dn11 = assign97160_e149659_d_n11;
        locals.var_ttemp_dn14 = assign97160_e149659_d_n14;

        let assign97170_e149662: f64 = (p.p511 * locals.var_jd_nvtm_invd);
        locals.var_t10 = assign97170_e149662;
        locals.var_t10_dn0 = (p.p511 * locals.var_jd_nvtm_invd_dn0);
        locals.var_t10_dn2 = (p.p511 * locals.var_jd_nvtm_invd_dn2);
        locals.var_t10_dn4 = (p.p511 * locals.var_jd_nvtm_invd_dn4);
        locals.var_t10_dn5 = (p.p511 * locals.var_jd_nvtm_invd_dn5);
        locals.var_t10_dn6 = (p.p511 * locals.var_jd_nvtm_invd_dn6);
        locals.var_t10_dn7 = (p.p511 * locals.var_jd_nvtm_invd_dn7);
        locals.var_t10_dn8 = (p.p511 * locals.var_jd_nvtm_invd_dn8);
        locals.var_t10_dn9 = (p.p511 * locals.var_jd_nvtm_invd_dn9);
        locals.var_t10_dn10 = (p.p511 * locals.var_jd_nvtm_invd_dn10);
        locals.var_t10_dn11 = (p.p511 * locals.var_jd_nvtm_invd_dn11);
        locals.var_t10_dn14 = (p.p511 * locals.var_jd_nvtm_invd_dn14);

        let assign97180_e149665: f64 = (p.p510 * locals.var_exptempd);
        locals.var_t9 = assign97180_e149665;
        locals.var_t9_dn0 = (p.p510 * locals.var_exptempd_dn0);
        locals.var_t9_dn2 = (p.p510 * locals.var_exptempd_dn2);
        locals.var_t9_dn4 = (p.p510 * locals.var_exptempd_dn4);
        locals.var_t9_dn5 = (p.p510 * locals.var_exptempd_dn5);
        locals.var_t9_dn6 = (p.p510 * locals.var_exptempd_dn6);
        locals.var_t9_dn7 = (p.p510 * locals.var_exptempd_dn7);
        locals.var_t9_dn8 = (p.p510 * locals.var_exptempd_dn8);
        locals.var_t9_dn9 = (p.p510 * locals.var_exptempd_dn9);
        locals.var_t9_dn10 = (p.p510 * locals.var_exptempd_dn10);
        locals.var_t9_dn11 = (p.p510 * locals.var_exptempd_dn11);
        locals.var_t9_dn14 = (p.p510 * locals.var_exptempd_dn14);

        let assign97190_e149668: f64 = if locals.var_isbd_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2256 = assign97190_e149668;

        let (assign97200_e149674, assign97200_e149674_d_n0, assign97200_e149674_d_n2, assign97200_e149674_d_n4, assign97200_e149674_d_n5, assign97200_e149674_d_n6, assign97200_e149674_d_n7, assign97200_e149674_d_n8, assign97200_e149674_d_n9, assign97200_e149674_d_n10, assign97200_e149674_d_n11, assign97200_e149674_d_n14,) = {
    if (locals.var_guard2256 != 0.0) {
        let assign97200_e149672: f64 = (locals.var_isbd2_btm * locals.var_t9);
        (assign97200_e149672, ((locals.var_isbd2_btm_dn0 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn0)), ((locals.var_isbd2_btm_dn2 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn2)), ((locals.var_isbd2_btm_dn4 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn4)), ((locals.var_isbd2_btm_dn5 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn5)), ((locals.var_isbd2_btm_dn6 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn6)), ((locals.var_isbd2_btm_dn7 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn7)), ((locals.var_isbd2_btm_dn8 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn8)), ((locals.var_isbd2_btm_dn9 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn9)), ((locals.var_isbd2_btm_dn10 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn10)), ((locals.var_isbd2_btm_dn11 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn11)), ((locals.var_isbd2_btm_dn14 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97200_e149674;
        locals.var_t0_dn0 = assign97200_e149674_d_n0;
        locals.var_t0_dn2 = assign97200_e149674_d_n2;
        locals.var_t0_dn4 = assign97200_e149674_d_n4;
        locals.var_t0_dn5 = assign97200_e149674_d_n5;
        locals.var_t0_dn6 = assign97200_e149674_d_n6;
        locals.var_t0_dn7 = assign97200_e149674_d_n7;
        locals.var_t0_dn8 = assign97200_e149674_d_n8;
        locals.var_t0_dn9 = assign97200_e149674_d_n9;
        locals.var_t0_dn10 = assign97200_e149674_d_n10;
        locals.var_t0_dn11 = assign97200_e149674_d_n11;
        locals.var_t0_dn14 = assign97200_e149674_d_n14;

        let (assign97210_e149681, assign97210_e149681_d_n0, assign97210_e149681_d_n2, assign97210_e149681_d_n4, assign97210_e149681_d_n5, assign97210_e149681_d_n6, assign97210_e149681_d_n7, assign97210_e149681_d_n8, assign97210_e149681_d_n9, assign97210_e149681_d_n10, assign97210_e149681_d_n11, assign97210_e149681_d_n14,) = {
    if (locals.var_guard2256 != 0.0) {
        let assign97210_e149677: f64 = (-locals.var_vbd_jct);
        let assign97210_e149679: f64 = (assign97210_e149677 * locals.var_t10);
        (assign97210_e149679, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97210_e149677 * locals.var_t10_dn0)), (assign97210_e149677 * locals.var_t10_dn2), (assign97210_e149677 * locals.var_t10_dn4), (assign97210_e149677 * locals.var_t10_dn5), (assign97210_e149677 * locals.var_t10_dn6), (assign97210_e149677 * locals.var_t10_dn7), (assign97210_e149677 * locals.var_t10_dn8), (assign97210_e149677 * locals.var_t10_dn9), (((-locals.var_vbd_jct_dn10) * locals.var_t10) + (assign97210_e149677 * locals.var_t10_dn10)), (assign97210_e149677 * locals.var_t10_dn11), (assign97210_e149677 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97210_e149681;
        locals.var_tx_dn0 = assign97210_e149681_d_n0;
        locals.var_tx_dn2 = assign97210_e149681_d_n2;
        locals.var_tx_dn4 = assign97210_e149681_d_n4;
        locals.var_tx_dn5 = assign97210_e149681_d_n5;
        locals.var_tx_dn6 = assign97210_e149681_d_n6;
        locals.var_tx_dn7 = assign97210_e149681_d_n7;
        locals.var_tx_dn8 = assign97210_e149681_d_n8;
        locals.var_tx_dn9 = assign97210_e149681_d_n9;
        locals.var_tx_dn10 = assign97210_e149681_d_n10;
        locals.var_tx_dn11 = assign97210_e149681_d_n11;
        locals.var_tx_dn14 = assign97210_e149681_d_n14;

        let (assign97220_e149686, assign97220_e149686_d_n0, assign97220_e149686_d_n2, assign97220_e149686_d_n4, assign97220_e149686_d_n5, assign97220_e149686_d_n6, assign97220_e149686_d_n7, assign97220_e149686_d_n8, assign97220_e149686_d_n9, assign97220_e149686_d_n10, assign97220_e149686_d_n11, assign97220_e149686_d_n14,) = {
    if (locals.var_guard2256 != 0.0) {
        let assign97220_e149684: f64 = (locals.var_tx).exp();
        (assign97220_e149684, (assign97220_e149684 * locals.var_tx_dn0), (assign97220_e149684 * locals.var_tx_dn2), (assign97220_e149684 * locals.var_tx_dn4), (assign97220_e149684 * locals.var_tx_dn5), (assign97220_e149684 * locals.var_tx_dn6), (assign97220_e149684 * locals.var_tx_dn7), (assign97220_e149684 * locals.var_tx_dn8), (assign97220_e149684 * locals.var_tx_dn9), (assign97220_e149684 * locals.var_tx_dn10), (assign97220_e149684 * locals.var_tx_dn11), (assign97220_e149684 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97220_e149686;
        locals.var_t2_dn0 = assign97220_e149686_d_n0;
        locals.var_t2_dn2 = assign97220_e149686_d_n2;
        locals.var_t2_dn4 = assign97220_e149686_d_n4;
        locals.var_t2_dn5 = assign97220_e149686_d_n5;
        locals.var_t2_dn6 = assign97220_e149686_d_n6;
        locals.var_t2_dn7 = assign97220_e149686_d_n7;
        locals.var_t2_dn8 = assign97220_e149686_d_n8;
        locals.var_t2_dn9 = assign97220_e149686_d_n9;
        locals.var_t2_dn10 = assign97220_e149686_d_n10;
        locals.var_t2_dn11 = assign97220_e149686_d_n11;
        locals.var_t2_dn14 = assign97220_e149686_d_n14;

        let (assign97230_e149690, assign97230_e149690_d_n0, assign97230_e149690_d_n2, assign97230_e149690_d_n4, assign97230_e149690_d_n5, assign97230_e149690_d_n6, assign97230_e149690_d_n7, assign97230_e149690_d_n8, assign97230_e149690_d_n9, assign97230_e149690_d_n10, assign97230_e149690_d_n11, assign97230_e149690_d_n14,) = {
    if (locals.var_guard2256 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97230_e149690;
        locals.var_t3_dn0 = assign97230_e149690_d_n0;
        locals.var_t3_dn2 = assign97230_e149690_d_n2;
        locals.var_t3_dn4 = assign97230_e149690_d_n4;
        locals.var_t3_dn5 = assign97230_e149690_d_n5;
        locals.var_t3_dn6 = assign97230_e149690_d_n6;
        locals.var_t3_dn7 = assign97230_e149690_d_n7;
        locals.var_t3_dn8 = assign97230_e149690_d_n8;
        locals.var_t3_dn9 = assign97230_e149690_d_n9;
        locals.var_t3_dn10 = assign97230_e149690_d_n10;
        locals.var_t3_dn11 = assign97230_e149690_d_n11;
        locals.var_t3_dn14 = assign97230_e149690_d_n14;

        let assign97240_e149693: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2257 = assign97240_e149693;

        let (assign97250_e149701, assign97250_e149701_d_n0, assign97250_e149701_d_n2, assign97250_e149701_d_n4, assign97250_e149701_d_n5, assign97250_e149701_d_n6, assign97250_e149701_d_n7, assign97250_e149701_d_n8, assign97250_e149701_d_n9, assign97250_e149701_d_n10, assign97250_e149701_d_n11, assign97250_e149701_d_n14,) = {
    if ((locals.var_guard2256 != 0.0) && (locals.var_guard2257 != 0.0)) {
        let assign97250_e149699: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97250_e149699, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9), ((locals.var_vbd_jct_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97250_e149701;
        locals.var_tx_dn0 = assign97250_e149701_d_n0;
        locals.var_tx_dn2 = assign97250_e149701_d_n2;
        locals.var_tx_dn4 = assign97250_e149701_d_n4;
        locals.var_tx_dn5 = assign97250_e149701_d_n5;
        locals.var_tx_dn6 = assign97250_e149701_d_n6;
        locals.var_tx_dn7 = assign97250_e149701_d_n7;
        locals.var_tx_dn8 = assign97250_e149701_d_n8;
        locals.var_tx_dn9 = assign97250_e149701_d_n9;
        locals.var_tx_dn10 = assign97250_e149701_d_n10;
        locals.var_tx_dn11 = assign97250_e149701_d_n11;
        locals.var_tx_dn14 = assign97250_e149701_d_n14;

        let assign97260_e149704: f64 = (-3.0);
        let assign97260_e149706: f64 = (assign97260_e149704 * 34.0);
        let assign97260_e149707: f64 = if locals.var_tx < assign97260_e149706 { 1.0 } else { 0.0 };
        locals.var_guard2258 = assign97260_e149707;

        let (assign97270_e149715, assign97270_e149715_d_n0, assign97270_e149715_d_n2, assign97270_e149715_d_n4, assign97270_e149715_d_n5, assign97270_e149715_d_n6, assign97270_e149715_d_n7, assign97270_e149715_d_n8, assign97270_e149715_d_n9, assign97270_e149715_d_n10, assign97270_e149715_d_n11, assign97270_e149715_d_n14,) = {
    if (((locals.var_guard2256 != 0.0) && (locals.var_guard2257 != 0.0)) && (locals.var_guard2258 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97270_e149715;
        locals.var_t1_dn0 = assign97270_e149715_d_n0;
        locals.var_t1_dn2 = assign97270_e149715_d_n2;
        locals.var_t1_dn4 = assign97270_e149715_d_n4;
        locals.var_t1_dn5 = assign97270_e149715_d_n5;
        locals.var_t1_dn6 = assign97270_e149715_d_n6;
        locals.var_t1_dn7 = assign97270_e149715_d_n7;
        locals.var_t1_dn8 = assign97270_e149715_d_n8;
        locals.var_t1_dn9 = assign97270_e149715_d_n9;
        locals.var_t1_dn10 = assign97270_e149715_d_n10;
        locals.var_t1_dn11 = assign97270_e149715_d_n11;
        locals.var_t1_dn14 = assign97270_e149715_d_n14;

        let (assign97280_e149725, assign97280_e149725_d_n0, assign97280_e149725_d_n2, assign97280_e149725_d_n4, assign97280_e149725_d_n5, assign97280_e149725_d_n6, assign97280_e149725_d_n7, assign97280_e149725_d_n8, assign97280_e149725_d_n9, assign97280_e149725_d_n10, assign97280_e149725_d_n11, assign97280_e149725_d_n14,) = {
    if (((locals.var_guard2256 != 0.0) && (locals.var_guard2257 != 0.0)) && (locals.var_guard2258 == 0.0)) {
        let assign97280_e149723: f64 = (locals.var_tx).exp();
        (assign97280_e149723, (assign97280_e149723 * locals.var_tx_dn0), (assign97280_e149723 * locals.var_tx_dn2), (assign97280_e149723 * locals.var_tx_dn4), (assign97280_e149723 * locals.var_tx_dn5), (assign97280_e149723 * locals.var_tx_dn6), (assign97280_e149723 * locals.var_tx_dn7), (assign97280_e149723 * locals.var_tx_dn8), (assign97280_e149723 * locals.var_tx_dn9), (assign97280_e149723 * locals.var_tx_dn10), (assign97280_e149723 * locals.var_tx_dn11), (assign97280_e149723 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97280_e149725;
        locals.var_t1_dn0 = assign97280_e149725_d_n0;
        locals.var_t1_dn2 = assign97280_e149725_d_n2;
        locals.var_t1_dn4 = assign97280_e149725_d_n4;
        locals.var_t1_dn5 = assign97280_e149725_d_n5;
        locals.var_t1_dn6 = assign97280_e149725_d_n6;
        locals.var_t1_dn7 = assign97280_e149725_d_n7;
        locals.var_t1_dn8 = assign97280_e149725_d_n8;
        locals.var_t1_dn9 = assign97280_e149725_d_n9;
        locals.var_t1_dn10 = assign97280_e149725_d_n10;
        locals.var_t1_dn11 = assign97280_e149725_d_n11;
        locals.var_t1_dn14 = assign97280_e149725_d_n14;

        let (assign97290_e149747, assign97290_e149747_d_n0, assign97290_e149747_d_n2, assign97290_e149747_d_n4, assign97290_e149747_d_n5, assign97290_e149747_d_n6, assign97290_e149747_d_n7, assign97290_e149747_d_n8, assign97290_e149747_d_n9, assign97290_e149747_d_n10, assign97290_e149747_d_n11, assign97290_e149747_d_n14,) = {
    if ((locals.var_guard2256 != 0.0) && (locals.var_guard2257 != 0.0)) {
        let assign97290_e149732: f64 = (locals.var_t1 - 1.0);
        let assign97290_e149733: f64 = (locals.var_isbd_btm * assign97290_e149732);
        let assign97290_e149737: f64 = (locals.var_t2 - 1.0);
        let assign97290_e149738: f64 = (locals.var_t0 * assign97290_e149737);
        let assign97290_e149739: f64 = (assign97290_e149733 + assign97290_e149738);
        let assign97290_e149743: f64 = (locals.var_t3 - 1.0);
        let assign97290_e149744: f64 = (locals.var_uc_cisbkd * assign97290_e149743);
        let assign97290_e149745: f64 = (assign97290_e149739 + assign97290_e149744);
        (assign97290_e149745, ((((locals.var_isbd_btm_dn0 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), ((((locals.var_isbd_btm_dn2 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), ((((locals.var_isbd_btm_dn4 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), ((((locals.var_isbd_btm_dn5 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), ((((locals.var_isbd_btm_dn6 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), ((((locals.var_isbd_btm_dn7 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), ((((locals.var_isbd_btm_dn8 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), ((((locals.var_isbd_btm_dn9 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), ((((locals.var_isbd_btm_dn10 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), ((((locals.var_isbd_btm_dn11 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn11)) + ((locals.var_t0_dn11 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbkd * locals.var_t3_dn11)), ((((locals.var_isbd_btm_dn14 * assign97290_e149732) + (locals.var_isbd_btm * locals.var_t1_dn14)) + ((locals.var_t0_dn14 * assign97290_e149737) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbkd * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibd_btm, locals.var_ibd_btm_dn0, locals.var_ibd_btm_dn2, locals.var_ibd_btm_dn4, locals.var_ibd_btm_dn5, locals.var_ibd_btm_dn6, locals.var_ibd_btm_dn7, locals.var_ibd_btm_dn8, locals.var_ibd_btm_dn9, locals.var_ibd_btm_dn10, locals.var_ibd_btm_dn11, locals.var_ibd_btm_dn14,)
    }
};
        locals.var_ibd_btm = assign97290_e149747;
        locals.var_ibd_btm_dn0 = assign97290_e149747_d_n0;
        locals.var_ibd_btm_dn2 = assign97290_e149747_d_n2;
        locals.var_ibd_btm_dn4 = assign97290_e149747_d_n4;
        locals.var_ibd_btm_dn5 = assign97290_e149747_d_n5;
        locals.var_ibd_btm_dn6 = assign97290_e149747_d_n6;
        locals.var_ibd_btm_dn7 = assign97290_e149747_d_n7;
        locals.var_ibd_btm_dn8 = assign97290_e149747_d_n8;
        locals.var_ibd_btm_dn9 = assign97290_e149747_d_n9;
        locals.var_ibd_btm_dn10 = assign97290_e149747_d_n10;
        locals.var_ibd_btm_dn11 = assign97290_e149747_d_n11;
        locals.var_ibd_btm_dn14 = assign97290_e149747_d_n14;

        let (assign97300_e149754, assign97300_e149754_d_n0, assign97300_e149754_d_n2, assign97300_e149754_d_n4, assign97300_e149754_d_n5, assign97300_e149754_d_n6, assign97300_e149754_d_n7, assign97300_e149754_d_n8, assign97300_e149754_d_n9, assign97300_e149754_d_n10, assign97300_e149754_d_n11, assign97300_e149754_d_n14,) = {
    if ((locals.var_guard2256 != 0.0) && (locals.var_guard2257 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97300_e149754;
        locals.var_t1_dn0 = assign97300_e149754_d_n0;
        locals.var_t1_dn2 = assign97300_e149754_d_n2;
        locals.var_t1_dn4 = assign97300_e149754_d_n4;
        locals.var_t1_dn5 = assign97300_e149754_d_n5;
        locals.var_t1_dn6 = assign97300_e149754_d_n6;
        locals.var_t1_dn7 = assign97300_e149754_d_n7;
        locals.var_t1_dn8 = assign97300_e149754_d_n8;
        locals.var_t1_dn9 = assign97300_e149754_d_n9;
        locals.var_t1_dn10 = assign97300_e149754_d_n10;
        locals.var_t1_dn11 = assign97300_e149754_d_n11;
        locals.var_t1_dn14 = assign97300_e149754_d_n14;

        let (assign97310_e149765, assign97310_e149765_d_n0, assign97310_e149765_d_n2, assign97310_e149765_d_n4, assign97310_e149765_d_n5, assign97310_e149765_d_n6, assign97310_e149765_d_n7, assign97310_e149765_d_n8, assign97310_e149765_d_n9, assign97310_e149765_d_n10, assign97310_e149765_d_n11, assign97310_e149765_d_n14,) = {
    if ((locals.var_guard2256 != 0.0) && (locals.var_guard2257 == 0.0)) {
        let assign97310_e149761: f64 = (locals.var_isbd_btm * locals.var_jd_nvtm_invd);
        let assign97310_e149763: f64 = (assign97310_e149761 * locals.var_t1);
        (assign97310_e149763, ((((locals.var_isbd_btm_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn0)), ((((locals.var_isbd_btm_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn2)), ((((locals.var_isbd_btm_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn4)), ((((locals.var_isbd_btm_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn5)), ((((locals.var_isbd_btm_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn6)), ((((locals.var_isbd_btm_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn7)), ((((locals.var_isbd_btm_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn8)), ((((locals.var_isbd_btm_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn9)), ((((locals.var_isbd_btm_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn10)), ((((locals.var_isbd_btm_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn11)), ((((locals.var_isbd_btm_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97310_e149761 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97310_e149765;
        locals.var_t4_dn0 = assign97310_e149765_d_n0;
        locals.var_t4_dn2 = assign97310_e149765_d_n2;
        locals.var_t4_dn4 = assign97310_e149765_d_n4;
        locals.var_t4_dn5 = assign97310_e149765_d_n5;
        locals.var_t4_dn6 = assign97310_e149765_d_n6;
        locals.var_t4_dn7 = assign97310_e149765_d_n7;
        locals.var_t4_dn8 = assign97310_e149765_d_n8;
        locals.var_t4_dn9 = assign97310_e149765_d_n9;
        locals.var_t4_dn10 = assign97310_e149765_d_n10;
        locals.var_t4_dn11 = assign97310_e149765_d_n11;
        locals.var_t4_dn14 = assign97310_e149765_d_n14;

        let (assign97320_e149794, assign97320_e149794_d_n0, assign97320_e149794_d_n2, assign97320_e149794_d_n4, assign97320_e149794_d_n5, assign97320_e149794_d_n6, assign97320_e149794_d_n7, assign97320_e149794_d_n8, assign97320_e149794_d_n9, assign97320_e149794_d_n10, assign97320_e149794_d_n11, assign97320_e149794_d_n14,) = {
    if ((locals.var_guard2256 != 0.0) && (locals.var_guard2257 == 0.0)) {
        let assign97320_e149773: f64 = (locals.var_t1 - 1.0);
        let assign97320_e149774: f64 = (locals.var_isbd_btm * assign97320_e149773);
        let assign97320_e149778: f64 = (locals.var_vbd_jct - locals.var_vbdt);
        let assign97320_e149779: f64 = (locals.var_t4 * assign97320_e149778);
        let assign97320_e149780: f64 = (assign97320_e149774 + assign97320_e149779);
        let assign97320_e149784: f64 = (locals.var_t2 - 1.0);
        let assign97320_e149785: f64 = (locals.var_t0 * assign97320_e149784);
        let assign97320_e149786: f64 = (assign97320_e149780 + assign97320_e149785);
        let assign97320_e149790: f64 = (locals.var_t3 - 1.0);
        let assign97320_e149791: f64 = (locals.var_uc_cisbkd * assign97320_e149790);
        let assign97320_e149792: f64 = (assign97320_e149786 + assign97320_e149791);
        (assign97320_e149792, (((((locals.var_isbd_btm_dn0 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign97320_e149778) + (locals.var_t4 * (locals.var_vbd_jct_dn0 - locals.var_vbdt_dn0)))) + ((locals.var_t0_dn0 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), (((((locals.var_isbd_btm_dn2 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign97320_e149778) + (locals.var_t4 * (-locals.var_vbdt_dn2)))) + ((locals.var_t0_dn2 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), (((((locals.var_isbd_btm_dn4 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign97320_e149778) + (locals.var_t4 * (-locals.var_vbdt_dn4)))) + ((locals.var_t0_dn4 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), (((((locals.var_isbd_btm_dn5 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign97320_e149778) + (locals.var_t4 * (-locals.var_vbdt_dn5)))) + ((locals.var_t0_dn5 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), (((((locals.var_isbd_btm_dn6 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign97320_e149778) + (locals.var_t4 * (-locals.var_vbdt_dn6)))) + ((locals.var_t0_dn6 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), (((((locals.var_isbd_btm_dn7 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign97320_e149778) + (locals.var_t4 * (-locals.var_vbdt_dn7)))) + ((locals.var_t0_dn7 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), (((((locals.var_isbd_btm_dn8 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign97320_e149778) + (locals.var_t4 * (-locals.var_vbdt_dn8)))) + ((locals.var_t0_dn8 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), (((((locals.var_isbd_btm_dn9 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign97320_e149778) + (locals.var_t4 * (-locals.var_vbdt_dn9)))) + ((locals.var_t0_dn9 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), (((((locals.var_isbd_btm_dn10 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign97320_e149778) + (locals.var_t4 * (locals.var_vbd_jct_dn10 - locals.var_vbdt_dn10)))) + ((locals.var_t0_dn10 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), (((((locals.var_isbd_btm_dn11 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * assign97320_e149778) + (locals.var_t4 * (-locals.var_vbdt_dn11)))) + ((locals.var_t0_dn11 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbkd * locals.var_t3_dn11)), (((((locals.var_isbd_btm_dn14 * assign97320_e149773) + (locals.var_isbd_btm * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * assign97320_e149778) + (locals.var_t4 * (-locals.var_vbdt_dn14)))) + ((locals.var_t0_dn14 * assign97320_e149784) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbkd * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibd_btm, locals.var_ibd_btm_dn0, locals.var_ibd_btm_dn2, locals.var_ibd_btm_dn4, locals.var_ibd_btm_dn5, locals.var_ibd_btm_dn6, locals.var_ibd_btm_dn7, locals.var_ibd_btm_dn8, locals.var_ibd_btm_dn9, locals.var_ibd_btm_dn10, locals.var_ibd_btm_dn11, locals.var_ibd_btm_dn14,)
    }
};
        locals.var_ibd_btm = assign97320_e149794;
        locals.var_ibd_btm_dn0 = assign97320_e149794_d_n0;
        locals.var_ibd_btm_dn2 = assign97320_e149794_d_n2;
        locals.var_ibd_btm_dn4 = assign97320_e149794_d_n4;
        locals.var_ibd_btm_dn5 = assign97320_e149794_d_n5;
        locals.var_ibd_btm_dn6 = assign97320_e149794_d_n6;
        locals.var_ibd_btm_dn7 = assign97320_e149794_d_n7;
        locals.var_ibd_btm_dn8 = assign97320_e149794_d_n8;
        locals.var_ibd_btm_dn9 = assign97320_e149794_d_n9;
        locals.var_ibd_btm_dn10 = assign97320_e149794_d_n10;
        locals.var_ibd_btm_dn11 = assign97320_e149794_d_n11;
        locals.var_ibd_btm_dn14 = assign97320_e149794_d_n14;

        let (assign97330_e149799, assign97330_e149799_d_n0, assign97330_e149799_d_n2, assign97330_e149799_d_n4, assign97330_e149799_d_n5, assign97330_e149799_d_n6, assign97330_e149799_d_n7, assign97330_e149799_d_n8, assign97330_e149799_d_n9, assign97330_e149799_d_n10, assign97330_e149799_d_n11, assign97330_e149799_d_n14,) = {
    if (locals.var_guard2256 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd_btm, locals.var_ibd_btm_dn0, locals.var_ibd_btm_dn2, locals.var_ibd_btm_dn4, locals.var_ibd_btm_dn5, locals.var_ibd_btm_dn6, locals.var_ibd_btm_dn7, locals.var_ibd_btm_dn8, locals.var_ibd_btm_dn9, locals.var_ibd_btm_dn10, locals.var_ibd_btm_dn11, locals.var_ibd_btm_dn14,)
    }
};
        locals.var_ibd_btm = assign97330_e149799;
        locals.var_ibd_btm_dn0 = assign97330_e149799_d_n0;
        locals.var_ibd_btm_dn2 = assign97330_e149799_d_n2;
        locals.var_ibd_btm_dn4 = assign97330_e149799_d_n4;
        locals.var_ibd_btm_dn5 = assign97330_e149799_d_n5;
        locals.var_ibd_btm_dn6 = assign97330_e149799_d_n6;
        locals.var_ibd_btm_dn7 = assign97330_e149799_d_n7;
        locals.var_ibd_btm_dn8 = assign97330_e149799_d_n8;
        locals.var_ibd_btm_dn9 = assign97330_e149799_d_n9;
        locals.var_ibd_btm_dn10 = assign97330_e149799_d_n10;
        locals.var_ibd_btm_dn11 = assign97330_e149799_d_n11;
        locals.var_ibd_btm_dn14 = assign97330_e149799_d_n14;

        let assign97340_e149802: f64 = (p.p514 * locals.var_isbd2_btm);
        locals.var_t12 = assign97340_e149802;
        locals.var_t12_dn0 = (p.p514 * locals.var_isbd2_btm_dn0);
        locals.var_t12_dn2 = (p.p514 * locals.var_isbd2_btm_dn2);
        locals.var_t12_dn4 = (p.p514 * locals.var_isbd2_btm_dn4);
        locals.var_t12_dn5 = (p.p514 * locals.var_isbd2_btm_dn5);
        locals.var_t12_dn6 = (p.p514 * locals.var_isbd2_btm_dn6);
        locals.var_t12_dn7 = (p.p514 * locals.var_isbd2_btm_dn7);
        locals.var_t12_dn8 = (p.p514 * locals.var_isbd2_btm_dn8);
        locals.var_t12_dn9 = (p.p514 * locals.var_isbd2_btm_dn9);
        locals.var_t12_dn10 = (p.p514 * locals.var_isbd2_btm_dn10);
        locals.var_t12_dn11 = (p.p514 * locals.var_isbd2_btm_dn11);
        locals.var_t12_dn14 = (p.p514 * locals.var_isbd2_btm_dn14);

    }

    pub(super) fn stamp_transient_block_359(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign97350_e149806: f64 = (locals.var_t12 * locals.var_vbd_jct);
        let assign97350_e149807: f64 = (locals.var_ibd_btm + assign97350_e149806);
        locals.var_ibd_btm = assign97350_e149807;
        locals.var_ibd_btm_dn0 = (locals.var_ibd_btm_dn0 + ((locals.var_t12_dn0 * locals.var_vbd_jct) + (locals.var_t12 * locals.var_vbd_jct_dn0)));
        locals.var_ibd_btm_dn2 = (locals.var_ibd_btm_dn2 + (locals.var_t12_dn2 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn4 = (locals.var_ibd_btm_dn4 + (locals.var_t12_dn4 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn5 = (locals.var_ibd_btm_dn5 + (locals.var_t12_dn5 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn6 = (locals.var_ibd_btm_dn6 + (locals.var_t12_dn6 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn7 = (locals.var_ibd_btm_dn7 + (locals.var_t12_dn7 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn8 = (locals.var_ibd_btm_dn8 + (locals.var_t12_dn8 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn9 = (locals.var_ibd_btm_dn9 + (locals.var_t12_dn9 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn10 = (locals.var_ibd_btm_dn10 + ((locals.var_t12_dn10 * locals.var_vbd_jct) + (locals.var_t12 * locals.var_vbd_jct_dn10)));
        locals.var_ibd_btm_dn11 = (locals.var_ibd_btm_dn11 + (locals.var_t12_dn11 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn14 = (locals.var_ibd_btm_dn14 + (locals.var_t12_dn14 * locals.var_vbd_jct));

        let assign97360_e149810: f64 = if locals.var_isbd_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2259 = assign97360_e149810;

        let (assign97370_e149816, assign97370_e149816_d_n0, assign97370_e149816_d_n2, assign97370_e149816_d_n4, assign97370_e149816_d_n5, assign97370_e149816_d_n6, assign97370_e149816_d_n7, assign97370_e149816_d_n8, assign97370_e149816_d_n9, assign97370_e149816_d_n10, assign97370_e149816_d_n11, assign97370_e149816_d_n14,) = {
    if (locals.var_guard2259 != 0.0) {
        let assign97370_e149814: f64 = (locals.var_isbd2_sws * locals.var_t9);
        (assign97370_e149814, ((locals.var_isbd2_sws_dn0 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn0)), ((locals.var_isbd2_sws_dn2 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn2)), ((locals.var_isbd2_sws_dn4 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn4)), ((locals.var_isbd2_sws_dn5 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn5)), ((locals.var_isbd2_sws_dn6 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn6)), ((locals.var_isbd2_sws_dn7 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn7)), ((locals.var_isbd2_sws_dn8 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn8)), ((locals.var_isbd2_sws_dn9 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn9)), ((locals.var_isbd2_sws_dn10 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn10)), ((locals.var_isbd2_sws_dn11 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn11)), ((locals.var_isbd2_sws_dn14 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97370_e149816;
        locals.var_t0_dn0 = assign97370_e149816_d_n0;
        locals.var_t0_dn2 = assign97370_e149816_d_n2;
        locals.var_t0_dn4 = assign97370_e149816_d_n4;
        locals.var_t0_dn5 = assign97370_e149816_d_n5;
        locals.var_t0_dn6 = assign97370_e149816_d_n6;
        locals.var_t0_dn7 = assign97370_e149816_d_n7;
        locals.var_t0_dn8 = assign97370_e149816_d_n8;
        locals.var_t0_dn9 = assign97370_e149816_d_n9;
        locals.var_t0_dn10 = assign97370_e149816_d_n10;
        locals.var_t0_dn11 = assign97370_e149816_d_n11;
        locals.var_t0_dn14 = assign97370_e149816_d_n14;

        let (assign97380_e149823, assign97380_e149823_d_n0, assign97380_e149823_d_n2, assign97380_e149823_d_n4, assign97380_e149823_d_n5, assign97380_e149823_d_n6, assign97380_e149823_d_n7, assign97380_e149823_d_n8, assign97380_e149823_d_n9, assign97380_e149823_d_n10, assign97380_e149823_d_n11, assign97380_e149823_d_n14,) = {
    if (locals.var_guard2259 != 0.0) {
        let assign97380_e149819: f64 = (-locals.var_vbd_jct);
        let assign97380_e149821: f64 = (assign97380_e149819 * locals.var_t10);
        (assign97380_e149821, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97380_e149819 * locals.var_t10_dn0)), (assign97380_e149819 * locals.var_t10_dn2), (assign97380_e149819 * locals.var_t10_dn4), (assign97380_e149819 * locals.var_t10_dn5), (assign97380_e149819 * locals.var_t10_dn6), (assign97380_e149819 * locals.var_t10_dn7), (assign97380_e149819 * locals.var_t10_dn8), (assign97380_e149819 * locals.var_t10_dn9), (((-locals.var_vbd_jct_dn10) * locals.var_t10) + (assign97380_e149819 * locals.var_t10_dn10)), (assign97380_e149819 * locals.var_t10_dn11), (assign97380_e149819 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97380_e149823;
        locals.var_tx_dn0 = assign97380_e149823_d_n0;
        locals.var_tx_dn2 = assign97380_e149823_d_n2;
        locals.var_tx_dn4 = assign97380_e149823_d_n4;
        locals.var_tx_dn5 = assign97380_e149823_d_n5;
        locals.var_tx_dn6 = assign97380_e149823_d_n6;
        locals.var_tx_dn7 = assign97380_e149823_d_n7;
        locals.var_tx_dn8 = assign97380_e149823_d_n8;
        locals.var_tx_dn9 = assign97380_e149823_d_n9;
        locals.var_tx_dn10 = assign97380_e149823_d_n10;
        locals.var_tx_dn11 = assign97380_e149823_d_n11;
        locals.var_tx_dn14 = assign97380_e149823_d_n14;

        let (assign97390_e149828, assign97390_e149828_d_n0, assign97390_e149828_d_n2, assign97390_e149828_d_n4, assign97390_e149828_d_n5, assign97390_e149828_d_n6, assign97390_e149828_d_n7, assign97390_e149828_d_n8, assign97390_e149828_d_n9, assign97390_e149828_d_n10, assign97390_e149828_d_n11, assign97390_e149828_d_n14,) = {
    if (locals.var_guard2259 != 0.0) {
        let assign97390_e149826: f64 = (locals.var_tx).exp();
        (assign97390_e149826, (assign97390_e149826 * locals.var_tx_dn0), (assign97390_e149826 * locals.var_tx_dn2), (assign97390_e149826 * locals.var_tx_dn4), (assign97390_e149826 * locals.var_tx_dn5), (assign97390_e149826 * locals.var_tx_dn6), (assign97390_e149826 * locals.var_tx_dn7), (assign97390_e149826 * locals.var_tx_dn8), (assign97390_e149826 * locals.var_tx_dn9), (assign97390_e149826 * locals.var_tx_dn10), (assign97390_e149826 * locals.var_tx_dn11), (assign97390_e149826 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97390_e149828;
        locals.var_t2_dn0 = assign97390_e149828_d_n0;
        locals.var_t2_dn2 = assign97390_e149828_d_n2;
        locals.var_t2_dn4 = assign97390_e149828_d_n4;
        locals.var_t2_dn5 = assign97390_e149828_d_n5;
        locals.var_t2_dn6 = assign97390_e149828_d_n6;
        locals.var_t2_dn7 = assign97390_e149828_d_n7;
        locals.var_t2_dn8 = assign97390_e149828_d_n8;
        locals.var_t2_dn9 = assign97390_e149828_d_n9;
        locals.var_t2_dn10 = assign97390_e149828_d_n10;
        locals.var_t2_dn11 = assign97390_e149828_d_n11;
        locals.var_t2_dn14 = assign97390_e149828_d_n14;

        let (assign97400_e149832, assign97400_e149832_d_n0, assign97400_e149832_d_n2, assign97400_e149832_d_n4, assign97400_e149832_d_n5, assign97400_e149832_d_n6, assign97400_e149832_d_n7, assign97400_e149832_d_n8, assign97400_e149832_d_n9, assign97400_e149832_d_n10, assign97400_e149832_d_n11, assign97400_e149832_d_n14,) = {
    if (locals.var_guard2259 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97400_e149832;
        locals.var_t3_dn0 = assign97400_e149832_d_n0;
        locals.var_t3_dn2 = assign97400_e149832_d_n2;
        locals.var_t3_dn4 = assign97400_e149832_d_n4;
        locals.var_t3_dn5 = assign97400_e149832_d_n5;
        locals.var_t3_dn6 = assign97400_e149832_d_n6;
        locals.var_t3_dn7 = assign97400_e149832_d_n7;
        locals.var_t3_dn8 = assign97400_e149832_d_n8;
        locals.var_t3_dn9 = assign97400_e149832_d_n9;
        locals.var_t3_dn10 = assign97400_e149832_d_n10;
        locals.var_t3_dn11 = assign97400_e149832_d_n11;
        locals.var_t3_dn14 = assign97400_e149832_d_n14;

        let assign97410_e149835: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2260 = assign97410_e149835;

        let (assign97420_e149843, assign97420_e149843_d_n0, assign97420_e149843_d_n2, assign97420_e149843_d_n4, assign97420_e149843_d_n5, assign97420_e149843_d_n6, assign97420_e149843_d_n7, assign97420_e149843_d_n8, assign97420_e149843_d_n9, assign97420_e149843_d_n10, assign97420_e149843_d_n11, assign97420_e149843_d_n14,) = {
    if ((locals.var_guard2259 != 0.0) && (locals.var_guard2260 != 0.0)) {
        let assign97420_e149841: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97420_e149841, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9), ((locals.var_vbd_jct_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97420_e149843;
        locals.var_tx_dn0 = assign97420_e149843_d_n0;
        locals.var_tx_dn2 = assign97420_e149843_d_n2;
        locals.var_tx_dn4 = assign97420_e149843_d_n4;
        locals.var_tx_dn5 = assign97420_e149843_d_n5;
        locals.var_tx_dn6 = assign97420_e149843_d_n6;
        locals.var_tx_dn7 = assign97420_e149843_d_n7;
        locals.var_tx_dn8 = assign97420_e149843_d_n8;
        locals.var_tx_dn9 = assign97420_e149843_d_n9;
        locals.var_tx_dn10 = assign97420_e149843_d_n10;
        locals.var_tx_dn11 = assign97420_e149843_d_n11;
        locals.var_tx_dn14 = assign97420_e149843_d_n14;

        let assign97430_e149846: f64 = (-3.0);
        let assign97430_e149848: f64 = (assign97430_e149846 * 34.0);
        let assign97430_e149849: f64 = if locals.var_tx < assign97430_e149848 { 1.0 } else { 0.0 };
        locals.var_guard2261 = assign97430_e149849;

        let (assign97440_e149857, assign97440_e149857_d_n0, assign97440_e149857_d_n2, assign97440_e149857_d_n4, assign97440_e149857_d_n5, assign97440_e149857_d_n6, assign97440_e149857_d_n7, assign97440_e149857_d_n8, assign97440_e149857_d_n9, assign97440_e149857_d_n10, assign97440_e149857_d_n11, assign97440_e149857_d_n14,) = {
    if (((locals.var_guard2259 != 0.0) && (locals.var_guard2260 != 0.0)) && (locals.var_guard2261 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97440_e149857;
        locals.var_t1_dn0 = assign97440_e149857_d_n0;
        locals.var_t1_dn2 = assign97440_e149857_d_n2;
        locals.var_t1_dn4 = assign97440_e149857_d_n4;
        locals.var_t1_dn5 = assign97440_e149857_d_n5;
        locals.var_t1_dn6 = assign97440_e149857_d_n6;
        locals.var_t1_dn7 = assign97440_e149857_d_n7;
        locals.var_t1_dn8 = assign97440_e149857_d_n8;
        locals.var_t1_dn9 = assign97440_e149857_d_n9;
        locals.var_t1_dn10 = assign97440_e149857_d_n10;
        locals.var_t1_dn11 = assign97440_e149857_d_n11;
        locals.var_t1_dn14 = assign97440_e149857_d_n14;

        let (assign97450_e149867, assign97450_e149867_d_n0, assign97450_e149867_d_n2, assign97450_e149867_d_n4, assign97450_e149867_d_n5, assign97450_e149867_d_n6, assign97450_e149867_d_n7, assign97450_e149867_d_n8, assign97450_e149867_d_n9, assign97450_e149867_d_n10, assign97450_e149867_d_n11, assign97450_e149867_d_n14,) = {
    if (((locals.var_guard2259 != 0.0) && (locals.var_guard2260 != 0.0)) && (locals.var_guard2261 == 0.0)) {
        let assign97450_e149865: f64 = (locals.var_tx).exp();
        (assign97450_e149865, (assign97450_e149865 * locals.var_tx_dn0), (assign97450_e149865 * locals.var_tx_dn2), (assign97450_e149865 * locals.var_tx_dn4), (assign97450_e149865 * locals.var_tx_dn5), (assign97450_e149865 * locals.var_tx_dn6), (assign97450_e149865 * locals.var_tx_dn7), (assign97450_e149865 * locals.var_tx_dn8), (assign97450_e149865 * locals.var_tx_dn9), (assign97450_e149865 * locals.var_tx_dn10), (assign97450_e149865 * locals.var_tx_dn11), (assign97450_e149865 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97450_e149867;
        locals.var_t1_dn0 = assign97450_e149867_d_n0;
        locals.var_t1_dn2 = assign97450_e149867_d_n2;
        locals.var_t1_dn4 = assign97450_e149867_d_n4;
        locals.var_t1_dn5 = assign97450_e149867_d_n5;
        locals.var_t1_dn6 = assign97450_e149867_d_n6;
        locals.var_t1_dn7 = assign97450_e149867_d_n7;
        locals.var_t1_dn8 = assign97450_e149867_d_n8;
        locals.var_t1_dn9 = assign97450_e149867_d_n9;
        locals.var_t1_dn10 = assign97450_e149867_d_n10;
        locals.var_t1_dn11 = assign97450_e149867_d_n11;
        locals.var_t1_dn14 = assign97450_e149867_d_n14;

        let (assign97460_e149889, assign97460_e149889_d_n0, assign97460_e149889_d_n2, assign97460_e149889_d_n4, assign97460_e149889_d_n5, assign97460_e149889_d_n6, assign97460_e149889_d_n7, assign97460_e149889_d_n8, assign97460_e149889_d_n9, assign97460_e149889_d_n10, assign97460_e149889_d_n11, assign97460_e149889_d_n14,) = {
    if ((locals.var_guard2259 != 0.0) && (locals.var_guard2260 != 0.0)) {
        let assign97460_e149874: f64 = (locals.var_t1 - 1.0);
        let assign97460_e149875: f64 = (locals.var_isbd_sws * assign97460_e149874);
        let assign97460_e149879: f64 = (locals.var_t2 - 1.0);
        let assign97460_e149880: f64 = (locals.var_t0 * assign97460_e149879);
        let assign97460_e149881: f64 = (assign97460_e149875 + assign97460_e149880);
        let assign97460_e149885: f64 = (locals.var_t3 - 1.0);
        let assign97460_e149886: f64 = (locals.var_uc_cisbkd * assign97460_e149885);
        let assign97460_e149887: f64 = (assign97460_e149881 + assign97460_e149886);
        (assign97460_e149887, ((((locals.var_isbd_sws_dn0 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), ((((locals.var_isbd_sws_dn2 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), ((((locals.var_isbd_sws_dn4 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), ((((locals.var_isbd_sws_dn5 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), ((((locals.var_isbd_sws_dn6 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), ((((locals.var_isbd_sws_dn7 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), ((((locals.var_isbd_sws_dn8 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), ((((locals.var_isbd_sws_dn9 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), ((((locals.var_isbd_sws_dn10 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), ((((locals.var_isbd_sws_dn11 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn11)) + ((locals.var_t0_dn11 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbkd * locals.var_t3_dn11)), ((((locals.var_isbd_sws_dn14 * assign97460_e149874) + (locals.var_isbd_sws * locals.var_t1_dn14)) + ((locals.var_t0_dn14 * assign97460_e149879) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbkd * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibd_sws, locals.var_ibd_sws_dn0, locals.var_ibd_sws_dn2, locals.var_ibd_sws_dn4, locals.var_ibd_sws_dn5, locals.var_ibd_sws_dn6, locals.var_ibd_sws_dn7, locals.var_ibd_sws_dn8, locals.var_ibd_sws_dn9, locals.var_ibd_sws_dn10, locals.var_ibd_sws_dn11, locals.var_ibd_sws_dn14,)
    }
};
        locals.var_ibd_sws = assign97460_e149889;
        locals.var_ibd_sws_dn0 = assign97460_e149889_d_n0;
        locals.var_ibd_sws_dn2 = assign97460_e149889_d_n2;
        locals.var_ibd_sws_dn4 = assign97460_e149889_d_n4;
        locals.var_ibd_sws_dn5 = assign97460_e149889_d_n5;
        locals.var_ibd_sws_dn6 = assign97460_e149889_d_n6;
        locals.var_ibd_sws_dn7 = assign97460_e149889_d_n7;
        locals.var_ibd_sws_dn8 = assign97460_e149889_d_n8;
        locals.var_ibd_sws_dn9 = assign97460_e149889_d_n9;
        locals.var_ibd_sws_dn10 = assign97460_e149889_d_n10;
        locals.var_ibd_sws_dn11 = assign97460_e149889_d_n11;
        locals.var_ibd_sws_dn14 = assign97460_e149889_d_n14;

        let (assign97470_e149896, assign97470_e149896_d_n0, assign97470_e149896_d_n2, assign97470_e149896_d_n4, assign97470_e149896_d_n5, assign97470_e149896_d_n6, assign97470_e149896_d_n7, assign97470_e149896_d_n8, assign97470_e149896_d_n9, assign97470_e149896_d_n10, assign97470_e149896_d_n11, assign97470_e149896_d_n14,) = {
    if ((locals.var_guard2259 != 0.0) && (locals.var_guard2260 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97470_e149896;
        locals.var_t1_dn0 = assign97470_e149896_d_n0;
        locals.var_t1_dn2 = assign97470_e149896_d_n2;
        locals.var_t1_dn4 = assign97470_e149896_d_n4;
        locals.var_t1_dn5 = assign97470_e149896_d_n5;
        locals.var_t1_dn6 = assign97470_e149896_d_n6;
        locals.var_t1_dn7 = assign97470_e149896_d_n7;
        locals.var_t1_dn8 = assign97470_e149896_d_n8;
        locals.var_t1_dn9 = assign97470_e149896_d_n9;
        locals.var_t1_dn10 = assign97470_e149896_d_n10;
        locals.var_t1_dn11 = assign97470_e149896_d_n11;
        locals.var_t1_dn14 = assign97470_e149896_d_n14;

        let (assign97480_e149907, assign97480_e149907_d_n0, assign97480_e149907_d_n2, assign97480_e149907_d_n4, assign97480_e149907_d_n5, assign97480_e149907_d_n6, assign97480_e149907_d_n7, assign97480_e149907_d_n8, assign97480_e149907_d_n9, assign97480_e149907_d_n10, assign97480_e149907_d_n11, assign97480_e149907_d_n14,) = {
    if ((locals.var_guard2259 != 0.0) && (locals.var_guard2260 == 0.0)) {
        let assign97480_e149903: f64 = (locals.var_isbd_sws * locals.var_jd_nvtm_invd);
        let assign97480_e149905: f64 = (assign97480_e149903 * locals.var_t1);
        (assign97480_e149905, ((((locals.var_isbd_sws_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn0)), ((((locals.var_isbd_sws_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn2)), ((((locals.var_isbd_sws_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn4)), ((((locals.var_isbd_sws_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn5)), ((((locals.var_isbd_sws_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn6)), ((((locals.var_isbd_sws_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn7)), ((((locals.var_isbd_sws_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn8)), ((((locals.var_isbd_sws_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn9)), ((((locals.var_isbd_sws_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn10)), ((((locals.var_isbd_sws_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn11)), ((((locals.var_isbd_sws_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97480_e149903 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97480_e149907;
        locals.var_t4_dn0 = assign97480_e149907_d_n0;
        locals.var_t4_dn2 = assign97480_e149907_d_n2;
        locals.var_t4_dn4 = assign97480_e149907_d_n4;
        locals.var_t4_dn5 = assign97480_e149907_d_n5;
        locals.var_t4_dn6 = assign97480_e149907_d_n6;
        locals.var_t4_dn7 = assign97480_e149907_d_n7;
        locals.var_t4_dn8 = assign97480_e149907_d_n8;
        locals.var_t4_dn9 = assign97480_e149907_d_n9;
        locals.var_t4_dn10 = assign97480_e149907_d_n10;
        locals.var_t4_dn11 = assign97480_e149907_d_n11;
        locals.var_t4_dn14 = assign97480_e149907_d_n14;

        let (assign97490_e149936, assign97490_e149936_d_n0, assign97490_e149936_d_n2, assign97490_e149936_d_n4, assign97490_e149936_d_n5, assign97490_e149936_d_n6, assign97490_e149936_d_n7, assign97490_e149936_d_n8, assign97490_e149936_d_n9, assign97490_e149936_d_n10, assign97490_e149936_d_n11, assign97490_e149936_d_n14,) = {
    if ((locals.var_guard2259 != 0.0) && (locals.var_guard2260 == 0.0)) {
        let assign97490_e149915: f64 = (locals.var_t1 - 1.0);
        let assign97490_e149916: f64 = (locals.var_isbd_sws * assign97490_e149915);
        let assign97490_e149920: f64 = (locals.var_vbd_jct - locals.var_vbdt);
        let assign97490_e149921: f64 = (locals.var_t4 * assign97490_e149920);
        let assign97490_e149922: f64 = (assign97490_e149916 + assign97490_e149921);
        let assign97490_e149926: f64 = (locals.var_t2 - 1.0);
        let assign97490_e149927: f64 = (locals.var_t0 * assign97490_e149926);
        let assign97490_e149928: f64 = (assign97490_e149922 + assign97490_e149927);
        let assign97490_e149932: f64 = (locals.var_t3 - 1.0);
        let assign97490_e149933: f64 = (locals.var_uc_cisbkd * assign97490_e149932);
        let assign97490_e149934: f64 = (assign97490_e149928 + assign97490_e149933);
        (assign97490_e149934, (((((locals.var_isbd_sws_dn0 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign97490_e149920) + (locals.var_t4 * (locals.var_vbd_jct_dn0 - locals.var_vbdt_dn0)))) + ((locals.var_t0_dn0 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), (((((locals.var_isbd_sws_dn2 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign97490_e149920) + (locals.var_t4 * (-locals.var_vbdt_dn2)))) + ((locals.var_t0_dn2 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), (((((locals.var_isbd_sws_dn4 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign97490_e149920) + (locals.var_t4 * (-locals.var_vbdt_dn4)))) + ((locals.var_t0_dn4 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), (((((locals.var_isbd_sws_dn5 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign97490_e149920) + (locals.var_t4 * (-locals.var_vbdt_dn5)))) + ((locals.var_t0_dn5 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), (((((locals.var_isbd_sws_dn6 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign97490_e149920) + (locals.var_t4 * (-locals.var_vbdt_dn6)))) + ((locals.var_t0_dn6 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), (((((locals.var_isbd_sws_dn7 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign97490_e149920) + (locals.var_t4 * (-locals.var_vbdt_dn7)))) + ((locals.var_t0_dn7 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), (((((locals.var_isbd_sws_dn8 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign97490_e149920) + (locals.var_t4 * (-locals.var_vbdt_dn8)))) + ((locals.var_t0_dn8 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), (((((locals.var_isbd_sws_dn9 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign97490_e149920) + (locals.var_t4 * (-locals.var_vbdt_dn9)))) + ((locals.var_t0_dn9 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), (((((locals.var_isbd_sws_dn10 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign97490_e149920) + (locals.var_t4 * (locals.var_vbd_jct_dn10 - locals.var_vbdt_dn10)))) + ((locals.var_t0_dn10 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), (((((locals.var_isbd_sws_dn11 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * assign97490_e149920) + (locals.var_t4 * (-locals.var_vbdt_dn11)))) + ((locals.var_t0_dn11 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbkd * locals.var_t3_dn11)), (((((locals.var_isbd_sws_dn14 * assign97490_e149915) + (locals.var_isbd_sws * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * assign97490_e149920) + (locals.var_t4 * (-locals.var_vbdt_dn14)))) + ((locals.var_t0_dn14 * assign97490_e149926) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbkd * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibd_sws, locals.var_ibd_sws_dn0, locals.var_ibd_sws_dn2, locals.var_ibd_sws_dn4, locals.var_ibd_sws_dn5, locals.var_ibd_sws_dn6, locals.var_ibd_sws_dn7, locals.var_ibd_sws_dn8, locals.var_ibd_sws_dn9, locals.var_ibd_sws_dn10, locals.var_ibd_sws_dn11, locals.var_ibd_sws_dn14,)
    }
};
        locals.var_ibd_sws = assign97490_e149936;
        locals.var_ibd_sws_dn0 = assign97490_e149936_d_n0;
        locals.var_ibd_sws_dn2 = assign97490_e149936_d_n2;
        locals.var_ibd_sws_dn4 = assign97490_e149936_d_n4;
        locals.var_ibd_sws_dn5 = assign97490_e149936_d_n5;
        locals.var_ibd_sws_dn6 = assign97490_e149936_d_n6;
        locals.var_ibd_sws_dn7 = assign97490_e149936_d_n7;
        locals.var_ibd_sws_dn8 = assign97490_e149936_d_n8;
        locals.var_ibd_sws_dn9 = assign97490_e149936_d_n9;
        locals.var_ibd_sws_dn10 = assign97490_e149936_d_n10;
        locals.var_ibd_sws_dn11 = assign97490_e149936_d_n11;
        locals.var_ibd_sws_dn14 = assign97490_e149936_d_n14;

        let (assign97500_e149941, assign97500_e149941_d_n0, assign97500_e149941_d_n2, assign97500_e149941_d_n4, assign97500_e149941_d_n5, assign97500_e149941_d_n6, assign97500_e149941_d_n7, assign97500_e149941_d_n8, assign97500_e149941_d_n9, assign97500_e149941_d_n10, assign97500_e149941_d_n11, assign97500_e149941_d_n14,) = {
    if (locals.var_guard2259 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd_sws, locals.var_ibd_sws_dn0, locals.var_ibd_sws_dn2, locals.var_ibd_sws_dn4, locals.var_ibd_sws_dn5, locals.var_ibd_sws_dn6, locals.var_ibd_sws_dn7, locals.var_ibd_sws_dn8, locals.var_ibd_sws_dn9, locals.var_ibd_sws_dn10, locals.var_ibd_sws_dn11, locals.var_ibd_sws_dn14,)
    }
};
        locals.var_ibd_sws = assign97500_e149941;
        locals.var_ibd_sws_dn0 = assign97500_e149941_d_n0;
        locals.var_ibd_sws_dn2 = assign97500_e149941_d_n2;
        locals.var_ibd_sws_dn4 = assign97500_e149941_d_n4;
        locals.var_ibd_sws_dn5 = assign97500_e149941_d_n5;
        locals.var_ibd_sws_dn6 = assign97500_e149941_d_n6;
        locals.var_ibd_sws_dn7 = assign97500_e149941_d_n7;
        locals.var_ibd_sws_dn8 = assign97500_e149941_d_n8;
        locals.var_ibd_sws_dn9 = assign97500_e149941_d_n9;
        locals.var_ibd_sws_dn10 = assign97500_e149941_d_n10;
        locals.var_ibd_sws_dn11 = assign97500_e149941_d_n11;
        locals.var_ibd_sws_dn14 = assign97500_e149941_d_n14;

        let assign97510_e149944: f64 = (p.p514 * locals.var_isbd2_sws);
        locals.var_t12 = assign97510_e149944;
        locals.var_t12_dn0 = (p.p514 * locals.var_isbd2_sws_dn0);
        locals.var_t12_dn2 = (p.p514 * locals.var_isbd2_sws_dn2);
        locals.var_t12_dn4 = (p.p514 * locals.var_isbd2_sws_dn4);
        locals.var_t12_dn5 = (p.p514 * locals.var_isbd2_sws_dn5);
        locals.var_t12_dn6 = (p.p514 * locals.var_isbd2_sws_dn6);
        locals.var_t12_dn7 = (p.p514 * locals.var_isbd2_sws_dn7);
        locals.var_t12_dn8 = (p.p514 * locals.var_isbd2_sws_dn8);
        locals.var_t12_dn9 = (p.p514 * locals.var_isbd2_sws_dn9);
        locals.var_t12_dn10 = (p.p514 * locals.var_isbd2_sws_dn10);
        locals.var_t12_dn11 = (p.p514 * locals.var_isbd2_sws_dn11);
        locals.var_t12_dn14 = (p.p514 * locals.var_isbd2_sws_dn14);

        let assign97520_e149948: f64 = (locals.var_t12 * locals.var_vbd_jct);
        let assign97520_e149949: f64 = (locals.var_ibd_sws + assign97520_e149948);
        locals.var_ibd_sws = assign97520_e149949;
        locals.var_ibd_sws_dn0 = (locals.var_ibd_sws_dn0 + ((locals.var_t12_dn0 * locals.var_vbd_jct) + (locals.var_t12 * locals.var_vbd_jct_dn0)));
        locals.var_ibd_sws_dn2 = (locals.var_ibd_sws_dn2 + (locals.var_t12_dn2 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn4 = (locals.var_ibd_sws_dn4 + (locals.var_t12_dn4 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn5 = (locals.var_ibd_sws_dn5 + (locals.var_t12_dn5 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn6 = (locals.var_ibd_sws_dn6 + (locals.var_t12_dn6 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn7 = (locals.var_ibd_sws_dn7 + (locals.var_t12_dn7 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn8 = (locals.var_ibd_sws_dn8 + (locals.var_t12_dn8 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn9 = (locals.var_ibd_sws_dn9 + (locals.var_t12_dn9 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn10 = (locals.var_ibd_sws_dn10 + ((locals.var_t12_dn10 * locals.var_vbd_jct) + (locals.var_t12 * locals.var_vbd_jct_dn10)));
        locals.var_ibd_sws_dn11 = (locals.var_ibd_sws_dn11 + (locals.var_t12_dn11 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn14 = (locals.var_ibd_sws_dn14 + (locals.var_t12_dn14 * locals.var_vbd_jct));

        let assign97530_e149952: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2262 = assign97530_e149952;

        let assign97540_e149955: f64 = if locals.var_isbd_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2263 = assign97540_e149955;

        let (assign97550_e149963, assign97550_e149963_d_n0, assign97550_e149963_d_n2, assign97550_e149963_d_n4, assign97550_e149963_d_n5, assign97550_e149963_d_n6, assign97550_e149963_d_n7, assign97550_e149963_d_n8, assign97550_e149963_d_n9, assign97550_e149963_d_n10, assign97550_e149963_d_n11, assign97550_e149963_d_n14,) = {
    if ((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) {
        let assign97550_e149961: f64 = (locals.var_isbd2_swg * locals.var_t9);
        (assign97550_e149961, ((locals.var_isbd2_swg_dn0 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn0)), ((locals.var_isbd2_swg_dn2 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn2)), ((locals.var_isbd2_swg_dn4 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn4)), ((locals.var_isbd2_swg_dn5 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn5)), ((locals.var_isbd2_swg_dn6 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn6)), ((locals.var_isbd2_swg_dn7 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn7)), ((locals.var_isbd2_swg_dn8 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn8)), ((locals.var_isbd2_swg_dn9 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn9)), ((locals.var_isbd2_swg_dn10 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn10)), ((locals.var_isbd2_swg_dn11 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn11)), ((locals.var_isbd2_swg_dn14 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97550_e149963;
        locals.var_t0_dn0 = assign97550_e149963_d_n0;
        locals.var_t0_dn2 = assign97550_e149963_d_n2;
        locals.var_t0_dn4 = assign97550_e149963_d_n4;
        locals.var_t0_dn5 = assign97550_e149963_d_n5;
        locals.var_t0_dn6 = assign97550_e149963_d_n6;
        locals.var_t0_dn7 = assign97550_e149963_d_n7;
        locals.var_t0_dn8 = assign97550_e149963_d_n8;
        locals.var_t0_dn9 = assign97550_e149963_d_n9;
        locals.var_t0_dn10 = assign97550_e149963_d_n10;
        locals.var_t0_dn11 = assign97550_e149963_d_n11;
        locals.var_t0_dn14 = assign97550_e149963_d_n14;

        let (assign97560_e149972, assign97560_e149972_d_n0, assign97560_e149972_d_n2, assign97560_e149972_d_n4, assign97560_e149972_d_n5, assign97560_e149972_d_n6, assign97560_e149972_d_n7, assign97560_e149972_d_n8, assign97560_e149972_d_n9, assign97560_e149972_d_n10, assign97560_e149972_d_n11, assign97560_e149972_d_n14,) = {
    if ((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) {
        let assign97560_e149968: f64 = (-locals.var_vbdi_jct);
        let assign97560_e149970: f64 = (assign97560_e149968 * locals.var_t10);
        (assign97560_e149970, (assign97560_e149968 * locals.var_t10_dn0), (assign97560_e149968 * locals.var_t10_dn2), (assign97560_e149968 * locals.var_t10_dn4), (assign97560_e149968 * locals.var_t10_dn5), (((-locals.var_vbdi_jct_dn6) * locals.var_t10) + (assign97560_e149968 * locals.var_t10_dn6)), (assign97560_e149968 * locals.var_t10_dn7), (assign97560_e149968 * locals.var_t10_dn8), (((-locals.var_vbdi_jct_dn9) * locals.var_t10) + (assign97560_e149968 * locals.var_t10_dn9)), (assign97560_e149968 * locals.var_t10_dn10), (assign97560_e149968 * locals.var_t10_dn11), (assign97560_e149968 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97560_e149972;
        locals.var_tx_dn0 = assign97560_e149972_d_n0;
        locals.var_tx_dn2 = assign97560_e149972_d_n2;
        locals.var_tx_dn4 = assign97560_e149972_d_n4;
        locals.var_tx_dn5 = assign97560_e149972_d_n5;
        locals.var_tx_dn6 = assign97560_e149972_d_n6;
        locals.var_tx_dn7 = assign97560_e149972_d_n7;
        locals.var_tx_dn8 = assign97560_e149972_d_n8;
        locals.var_tx_dn9 = assign97560_e149972_d_n9;
        locals.var_tx_dn10 = assign97560_e149972_d_n10;
        locals.var_tx_dn11 = assign97560_e149972_d_n11;
        locals.var_tx_dn14 = assign97560_e149972_d_n14;

        let (assign97570_e149979, assign97570_e149979_d_n0, assign97570_e149979_d_n2, assign97570_e149979_d_n4, assign97570_e149979_d_n5, assign97570_e149979_d_n6, assign97570_e149979_d_n7, assign97570_e149979_d_n8, assign97570_e149979_d_n9, assign97570_e149979_d_n10, assign97570_e149979_d_n11, assign97570_e149979_d_n14,) = {
    if ((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) {
        let assign97570_e149977: f64 = (locals.var_tx).exp();
        (assign97570_e149977, (assign97570_e149977 * locals.var_tx_dn0), (assign97570_e149977 * locals.var_tx_dn2), (assign97570_e149977 * locals.var_tx_dn4), (assign97570_e149977 * locals.var_tx_dn5), (assign97570_e149977 * locals.var_tx_dn6), (assign97570_e149977 * locals.var_tx_dn7), (assign97570_e149977 * locals.var_tx_dn8), (assign97570_e149977 * locals.var_tx_dn9), (assign97570_e149977 * locals.var_tx_dn10), (assign97570_e149977 * locals.var_tx_dn11), (assign97570_e149977 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97570_e149979;
        locals.var_t2_dn0 = assign97570_e149979_d_n0;
        locals.var_t2_dn2 = assign97570_e149979_d_n2;
        locals.var_t2_dn4 = assign97570_e149979_d_n4;
        locals.var_t2_dn5 = assign97570_e149979_d_n5;
        locals.var_t2_dn6 = assign97570_e149979_d_n6;
        locals.var_t2_dn7 = assign97570_e149979_d_n7;
        locals.var_t2_dn8 = assign97570_e149979_d_n8;
        locals.var_t2_dn9 = assign97570_e149979_d_n9;
        locals.var_t2_dn10 = assign97570_e149979_d_n10;
        locals.var_t2_dn11 = assign97570_e149979_d_n11;
        locals.var_t2_dn14 = assign97570_e149979_d_n14;

        let (assign97580_e149985, assign97580_e149985_d_n0, assign97580_e149985_d_n2, assign97580_e149985_d_n4, assign97580_e149985_d_n5, assign97580_e149985_d_n6, assign97580_e149985_d_n7, assign97580_e149985_d_n8, assign97580_e149985_d_n9, assign97580_e149985_d_n10, assign97580_e149985_d_n11, assign97580_e149985_d_n14,) = {
    if ((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97580_e149985;
        locals.var_t3_dn0 = assign97580_e149985_d_n0;
        locals.var_t3_dn2 = assign97580_e149985_d_n2;
        locals.var_t3_dn4 = assign97580_e149985_d_n4;
        locals.var_t3_dn5 = assign97580_e149985_d_n5;
        locals.var_t3_dn6 = assign97580_e149985_d_n6;
        locals.var_t3_dn7 = assign97580_e149985_d_n7;
        locals.var_t3_dn8 = assign97580_e149985_d_n8;
        locals.var_t3_dn9 = assign97580_e149985_d_n9;
        locals.var_t3_dn10 = assign97580_e149985_d_n10;
        locals.var_t3_dn11 = assign97580_e149985_d_n11;
        locals.var_t3_dn14 = assign97580_e149985_d_n14;

        let assign97590_e149988: f64 = if locals.var_vbdi_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2264 = assign97590_e149988;

        let (assign97600_e149998, assign97600_e149998_d_n0, assign97600_e149998_d_n2, assign97600_e149998_d_n4, assign97600_e149998_d_n5, assign97600_e149998_d_n6, assign97600_e149998_d_n7, assign97600_e149998_d_n8, assign97600_e149998_d_n9, assign97600_e149998_d_n10, assign97600_e149998_d_n11, assign97600_e149998_d_n14,) = {
    if (((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) && (locals.var_guard2264 != 0.0)) {
        let assign97600_e149996: f64 = (locals.var_vbdi_jct * locals.var_jd_nvtm_invd);
        (assign97600_e149996, (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn0), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn5), ((locals.var_vbdi_jct_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn6)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn8), ((locals.var_vbdi_jct_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn9)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn10), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97600_e149998;
        locals.var_tx_dn0 = assign97600_e149998_d_n0;
        locals.var_tx_dn2 = assign97600_e149998_d_n2;
        locals.var_tx_dn4 = assign97600_e149998_d_n4;
        locals.var_tx_dn5 = assign97600_e149998_d_n5;
        locals.var_tx_dn6 = assign97600_e149998_d_n6;
        locals.var_tx_dn7 = assign97600_e149998_d_n7;
        locals.var_tx_dn8 = assign97600_e149998_d_n8;
        locals.var_tx_dn9 = assign97600_e149998_d_n9;
        locals.var_tx_dn10 = assign97600_e149998_d_n10;
        locals.var_tx_dn11 = assign97600_e149998_d_n11;
        locals.var_tx_dn14 = assign97600_e149998_d_n14;

        let assign97610_e150001: f64 = (-3.0);
        let assign97610_e150003: f64 = (assign97610_e150001 * 34.0);
        let assign97610_e150004: f64 = if locals.var_tx < assign97610_e150003 { 1.0 } else { 0.0 };
        locals.var_guard2265 = assign97610_e150004;

        let (assign97620_e150014, assign97620_e150014_d_n0, assign97620_e150014_d_n2, assign97620_e150014_d_n4, assign97620_e150014_d_n5, assign97620_e150014_d_n6, assign97620_e150014_d_n7, assign97620_e150014_d_n8, assign97620_e150014_d_n9, assign97620_e150014_d_n10, assign97620_e150014_d_n11, assign97620_e150014_d_n14,) = {
    if ((((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) && (locals.var_guard2264 != 0.0)) && (locals.var_guard2265 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97620_e150014;
        locals.var_t1_dn0 = assign97620_e150014_d_n0;
        locals.var_t1_dn2 = assign97620_e150014_d_n2;
        locals.var_t1_dn4 = assign97620_e150014_d_n4;
        locals.var_t1_dn5 = assign97620_e150014_d_n5;
        locals.var_t1_dn6 = assign97620_e150014_d_n6;
        locals.var_t1_dn7 = assign97620_e150014_d_n7;
        locals.var_t1_dn8 = assign97620_e150014_d_n8;
        locals.var_t1_dn9 = assign97620_e150014_d_n9;
        locals.var_t1_dn10 = assign97620_e150014_d_n10;
        locals.var_t1_dn11 = assign97620_e150014_d_n11;
        locals.var_t1_dn14 = assign97620_e150014_d_n14;

        let (assign97630_e150026, assign97630_e150026_d_n0, assign97630_e150026_d_n2, assign97630_e150026_d_n4, assign97630_e150026_d_n5, assign97630_e150026_d_n6, assign97630_e150026_d_n7, assign97630_e150026_d_n8, assign97630_e150026_d_n9, assign97630_e150026_d_n10, assign97630_e150026_d_n11, assign97630_e150026_d_n14,) = {
    if ((((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) && (locals.var_guard2264 != 0.0)) && (locals.var_guard2265 == 0.0)) {
        let assign97630_e150024: f64 = (locals.var_tx).exp();
        (assign97630_e150024, (assign97630_e150024 * locals.var_tx_dn0), (assign97630_e150024 * locals.var_tx_dn2), (assign97630_e150024 * locals.var_tx_dn4), (assign97630_e150024 * locals.var_tx_dn5), (assign97630_e150024 * locals.var_tx_dn6), (assign97630_e150024 * locals.var_tx_dn7), (assign97630_e150024 * locals.var_tx_dn8), (assign97630_e150024 * locals.var_tx_dn9), (assign97630_e150024 * locals.var_tx_dn10), (assign97630_e150024 * locals.var_tx_dn11), (assign97630_e150024 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97630_e150026;
        locals.var_t1_dn0 = assign97630_e150026_d_n0;
        locals.var_t1_dn2 = assign97630_e150026_d_n2;
        locals.var_t1_dn4 = assign97630_e150026_d_n4;
        locals.var_t1_dn5 = assign97630_e150026_d_n5;
        locals.var_t1_dn6 = assign97630_e150026_d_n6;
        locals.var_t1_dn7 = assign97630_e150026_d_n7;
        locals.var_t1_dn8 = assign97630_e150026_d_n8;
        locals.var_t1_dn9 = assign97630_e150026_d_n9;
        locals.var_t1_dn10 = assign97630_e150026_d_n10;
        locals.var_t1_dn11 = assign97630_e150026_d_n11;
        locals.var_t1_dn14 = assign97630_e150026_d_n14;

        let (assign97650_e150059, assign97650_e150059_d_n0, assign97650_e150059_d_n2, assign97650_e150059_d_n4, assign97650_e150059_d_n5, assign97650_e150059_d_n6, assign97650_e150059_d_n7, assign97650_e150059_d_n8, assign97650_e150059_d_n9, assign97650_e150059_d_n10, assign97650_e150059_d_n11, assign97650_e150059_d_n14,) = {
    if (((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) && (locals.var_guard2264 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97650_e150059;
        locals.var_t1_dn0 = assign97650_e150059_d_n0;
        locals.var_t1_dn2 = assign97650_e150059_d_n2;
        locals.var_t1_dn4 = assign97650_e150059_d_n4;
        locals.var_t1_dn5 = assign97650_e150059_d_n5;
        locals.var_t1_dn6 = assign97650_e150059_d_n6;
        locals.var_t1_dn7 = assign97650_e150059_d_n7;
        locals.var_t1_dn8 = assign97650_e150059_d_n8;
        locals.var_t1_dn9 = assign97650_e150059_d_n9;
        locals.var_t1_dn10 = assign97650_e150059_d_n10;
        locals.var_t1_dn11 = assign97650_e150059_d_n11;
        locals.var_t1_dn14 = assign97650_e150059_d_n14;

    }

    pub(super) fn stamp_transient_block_360(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign97660_e150072, assign97660_e150072_d_n0, assign97660_e150072_d_n2, assign97660_e150072_d_n4, assign97660_e150072_d_n5, assign97660_e150072_d_n6, assign97660_e150072_d_n7, assign97660_e150072_d_n8, assign97660_e150072_d_n9, assign97660_e150072_d_n10, assign97660_e150072_d_n11, assign97660_e150072_d_n14,) = {
    if (((locals.var_guard2262 != 0.0) && (locals.var_guard2263 != 0.0)) && (locals.var_guard2264 == 0.0)) {
        let assign97660_e150068: f64 = (locals.var_isbd_swg * locals.var_jd_nvtm_invd);
        let assign97660_e150070: f64 = (assign97660_e150068 * locals.var_t1);
        (assign97660_e150070, ((((locals.var_isbd_swg_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn0)), ((((locals.var_isbd_swg_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn2)), ((((locals.var_isbd_swg_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn4)), ((((locals.var_isbd_swg_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn5)), ((((locals.var_isbd_swg_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn6)), ((((locals.var_isbd_swg_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn7)), ((((locals.var_isbd_swg_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn8)), ((((locals.var_isbd_swg_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn9)), ((((locals.var_isbd_swg_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn10)), ((((locals.var_isbd_swg_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn11)), ((((locals.var_isbd_swg_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97660_e150068 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97660_e150072;
        locals.var_t4_dn0 = assign97660_e150072_d_n0;
        locals.var_t4_dn2 = assign97660_e150072_d_n2;
        locals.var_t4_dn4 = assign97660_e150072_d_n4;
        locals.var_t4_dn5 = assign97660_e150072_d_n5;
        locals.var_t4_dn6 = assign97660_e150072_d_n6;
        locals.var_t4_dn7 = assign97660_e150072_d_n7;
        locals.var_t4_dn8 = assign97660_e150072_d_n8;
        locals.var_t4_dn9 = assign97660_e150072_d_n9;
        locals.var_t4_dn10 = assign97660_e150072_d_n10;
        locals.var_t4_dn11 = assign97660_e150072_d_n11;
        locals.var_t4_dn14 = assign97660_e150072_d_n14;

        let (assign97690_e150116, assign97690_e150116_d_n0, assign97690_e150116_d_n2, assign97690_e150116_d_n4, assign97690_e150116_d_n5, assign97690_e150116_d_n6, assign97690_e150116_d_n7, assign97690_e150116_d_n8, assign97690_e150116_d_n9, assign97690_e150116_d_n10, assign97690_e150116_d_n11, assign97690_e150116_d_n14,) = {
    if (locals.var_guard2262 != 0.0) {
        let assign97690_e150114: f64 = (p.p514 * locals.var_isbd2_swg);
        (assign97690_e150114, (p.p514 * locals.var_isbd2_swg_dn0), (p.p514 * locals.var_isbd2_swg_dn2), (p.p514 * locals.var_isbd2_swg_dn4), (p.p514 * locals.var_isbd2_swg_dn5), (p.p514 * locals.var_isbd2_swg_dn6), (p.p514 * locals.var_isbd2_swg_dn7), (p.p514 * locals.var_isbd2_swg_dn8), (p.p514 * locals.var_isbd2_swg_dn9), (p.p514 * locals.var_isbd2_swg_dn10), (p.p514 * locals.var_isbd2_swg_dn11), (p.p514 * locals.var_isbd2_swg_dn14),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign97690_e150116;
        locals.var_t12_dn0 = assign97690_e150116_d_n0;
        locals.var_t12_dn2 = assign97690_e150116_d_n2;
        locals.var_t12_dn4 = assign97690_e150116_d_n4;
        locals.var_t12_dn5 = assign97690_e150116_d_n5;
        locals.var_t12_dn6 = assign97690_e150116_d_n6;
        locals.var_t12_dn7 = assign97690_e150116_d_n7;
        locals.var_t12_dn8 = assign97690_e150116_d_n8;
        locals.var_t12_dn9 = assign97690_e150116_d_n9;
        locals.var_t12_dn10 = assign97690_e150116_d_n10;
        locals.var_t12_dn11 = assign97690_e150116_d_n11;
        locals.var_t12_dn14 = assign97690_e150116_d_n14;

        let assign97720_e150132: f64 = (p.p534 * locals.var_jd_nvtm_invs);
        locals.var_t10 = assign97720_e150132;
        locals.var_t10_dn0 = (p.p534 * locals.var_jd_nvtm_invs_dn0);
        locals.var_t10_dn2 = (p.p534 * locals.var_jd_nvtm_invs_dn2);
        locals.var_t10_dn4 = (p.p534 * locals.var_jd_nvtm_invs_dn4);
        locals.var_t10_dn5 = (p.p534 * locals.var_jd_nvtm_invs_dn5);
        locals.var_t10_dn6 = (p.p534 * locals.var_jd_nvtm_invs_dn6);
        locals.var_t10_dn7 = (p.p534 * locals.var_jd_nvtm_invs_dn7);
        locals.var_t10_dn8 = (p.p534 * locals.var_jd_nvtm_invs_dn8);
        locals.var_t10_dn9 = (p.p534 * locals.var_jd_nvtm_invs_dn9);
        locals.var_t10_dn10 = (p.p534 * locals.var_jd_nvtm_invs_dn10);
        locals.var_t10_dn11 = (p.p534 * locals.var_jd_nvtm_invs_dn11);
        locals.var_t10_dn14 = (p.p534 * locals.var_jd_nvtm_invs_dn14);

        let assign97730_e150135: f64 = (p.p533 * locals.var_exptemps);
        locals.var_t9 = assign97730_e150135;
        locals.var_t9_dn0 = (p.p533 * locals.var_exptemps_dn0);
        locals.var_t9_dn2 = (p.p533 * locals.var_exptemps_dn2);
        locals.var_t9_dn4 = (p.p533 * locals.var_exptemps_dn4);
        locals.var_t9_dn5 = (p.p533 * locals.var_exptemps_dn5);
        locals.var_t9_dn6 = (p.p533 * locals.var_exptemps_dn6);
        locals.var_t9_dn7 = (p.p533 * locals.var_exptemps_dn7);
        locals.var_t9_dn8 = (p.p533 * locals.var_exptemps_dn8);
        locals.var_t9_dn9 = (p.p533 * locals.var_exptemps_dn9);
        locals.var_t9_dn10 = (p.p533 * locals.var_exptemps_dn10);
        locals.var_t9_dn11 = (p.p533 * locals.var_exptemps_dn11);
        locals.var_t9_dn14 = (p.p533 * locals.var_exptemps_dn14);

        let assign97740_e150138: f64 = if locals.var_isbs_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2266 = assign97740_e150138;

        let (assign97750_e150144, assign97750_e150144_d_n0, assign97750_e150144_d_n2, assign97750_e150144_d_n4, assign97750_e150144_d_n5, assign97750_e150144_d_n6, assign97750_e150144_d_n7, assign97750_e150144_d_n8, assign97750_e150144_d_n9, assign97750_e150144_d_n10, assign97750_e150144_d_n11, assign97750_e150144_d_n14,) = {
    if (locals.var_guard2266 != 0.0) {
        let assign97750_e150142: f64 = (locals.var_isbs2_btm * locals.var_t9);
        (assign97750_e150142, ((locals.var_isbs2_btm_dn0 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn0)), ((locals.var_isbs2_btm_dn2 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn2)), ((locals.var_isbs2_btm_dn4 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn4)), ((locals.var_isbs2_btm_dn5 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn5)), ((locals.var_isbs2_btm_dn6 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn6)), ((locals.var_isbs2_btm_dn7 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn7)), ((locals.var_isbs2_btm_dn8 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn8)), ((locals.var_isbs2_btm_dn9 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn9)), ((locals.var_isbs2_btm_dn10 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn10)), ((locals.var_isbs2_btm_dn11 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn11)), ((locals.var_isbs2_btm_dn14 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97750_e150144;
        locals.var_t0_dn0 = assign97750_e150144_d_n0;
        locals.var_t0_dn2 = assign97750_e150144_d_n2;
        locals.var_t0_dn4 = assign97750_e150144_d_n4;
        locals.var_t0_dn5 = assign97750_e150144_d_n5;
        locals.var_t0_dn6 = assign97750_e150144_d_n6;
        locals.var_t0_dn7 = assign97750_e150144_d_n7;
        locals.var_t0_dn8 = assign97750_e150144_d_n8;
        locals.var_t0_dn9 = assign97750_e150144_d_n9;
        locals.var_t0_dn10 = assign97750_e150144_d_n10;
        locals.var_t0_dn11 = assign97750_e150144_d_n11;
        locals.var_t0_dn14 = assign97750_e150144_d_n14;

        let (assign97760_e150151, assign97760_e150151_d_n0, assign97760_e150151_d_n2, assign97760_e150151_d_n4, assign97760_e150151_d_n5, assign97760_e150151_d_n6, assign97760_e150151_d_n7, assign97760_e150151_d_n8, assign97760_e150151_d_n9, assign97760_e150151_d_n10, assign97760_e150151_d_n11, assign97760_e150151_d_n14,) = {
    if (locals.var_guard2266 != 0.0) {
        let assign97760_e150147: f64 = (-locals.var_vbs_jct);
        let assign97760_e150149: f64 = (assign97760_e150147 * locals.var_t10);
        (assign97760_e150149, (assign97760_e150147 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97760_e150147 * locals.var_t10_dn2)), (assign97760_e150147 * locals.var_t10_dn4), (assign97760_e150147 * locals.var_t10_dn5), (assign97760_e150147 * locals.var_t10_dn6), (assign97760_e150147 * locals.var_t10_dn7), (assign97760_e150147 * locals.var_t10_dn8), (assign97760_e150147 * locals.var_t10_dn9), (assign97760_e150147 * locals.var_t10_dn10), (((-locals.var_vbs_jct_dn11) * locals.var_t10) + (assign97760_e150147 * locals.var_t10_dn11)), (assign97760_e150147 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97760_e150151;
        locals.var_tx_dn0 = assign97760_e150151_d_n0;
        locals.var_tx_dn2 = assign97760_e150151_d_n2;
        locals.var_tx_dn4 = assign97760_e150151_d_n4;
        locals.var_tx_dn5 = assign97760_e150151_d_n5;
        locals.var_tx_dn6 = assign97760_e150151_d_n6;
        locals.var_tx_dn7 = assign97760_e150151_d_n7;
        locals.var_tx_dn8 = assign97760_e150151_d_n8;
        locals.var_tx_dn9 = assign97760_e150151_d_n9;
        locals.var_tx_dn10 = assign97760_e150151_d_n10;
        locals.var_tx_dn11 = assign97760_e150151_d_n11;
        locals.var_tx_dn14 = assign97760_e150151_d_n14;

        let (assign97770_e150156, assign97770_e150156_d_n0, assign97770_e150156_d_n2, assign97770_e150156_d_n4, assign97770_e150156_d_n5, assign97770_e150156_d_n6, assign97770_e150156_d_n7, assign97770_e150156_d_n8, assign97770_e150156_d_n9, assign97770_e150156_d_n10, assign97770_e150156_d_n11, assign97770_e150156_d_n14,) = {
    if (locals.var_guard2266 != 0.0) {
        let assign97770_e150154: f64 = (locals.var_tx).exp();
        (assign97770_e150154, (assign97770_e150154 * locals.var_tx_dn0), (assign97770_e150154 * locals.var_tx_dn2), (assign97770_e150154 * locals.var_tx_dn4), (assign97770_e150154 * locals.var_tx_dn5), (assign97770_e150154 * locals.var_tx_dn6), (assign97770_e150154 * locals.var_tx_dn7), (assign97770_e150154 * locals.var_tx_dn8), (assign97770_e150154 * locals.var_tx_dn9), (assign97770_e150154 * locals.var_tx_dn10), (assign97770_e150154 * locals.var_tx_dn11), (assign97770_e150154 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97770_e150156;
        locals.var_t2_dn0 = assign97770_e150156_d_n0;
        locals.var_t2_dn2 = assign97770_e150156_d_n2;
        locals.var_t2_dn4 = assign97770_e150156_d_n4;
        locals.var_t2_dn5 = assign97770_e150156_d_n5;
        locals.var_t2_dn6 = assign97770_e150156_d_n6;
        locals.var_t2_dn7 = assign97770_e150156_d_n7;
        locals.var_t2_dn8 = assign97770_e150156_d_n8;
        locals.var_t2_dn9 = assign97770_e150156_d_n9;
        locals.var_t2_dn10 = assign97770_e150156_d_n10;
        locals.var_t2_dn11 = assign97770_e150156_d_n11;
        locals.var_t2_dn14 = assign97770_e150156_d_n14;

        let (assign97780_e150160, assign97780_e150160_d_n0, assign97780_e150160_d_n2, assign97780_e150160_d_n4, assign97780_e150160_d_n5, assign97780_e150160_d_n6, assign97780_e150160_d_n7, assign97780_e150160_d_n8, assign97780_e150160_d_n9, assign97780_e150160_d_n10, assign97780_e150160_d_n11, assign97780_e150160_d_n14,) = {
    if (locals.var_guard2266 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97780_e150160;
        locals.var_t3_dn0 = assign97780_e150160_d_n0;
        locals.var_t3_dn2 = assign97780_e150160_d_n2;
        locals.var_t3_dn4 = assign97780_e150160_d_n4;
        locals.var_t3_dn5 = assign97780_e150160_d_n5;
        locals.var_t3_dn6 = assign97780_e150160_d_n6;
        locals.var_t3_dn7 = assign97780_e150160_d_n7;
        locals.var_t3_dn8 = assign97780_e150160_d_n8;
        locals.var_t3_dn9 = assign97780_e150160_d_n9;
        locals.var_t3_dn10 = assign97780_e150160_d_n10;
        locals.var_t3_dn11 = assign97780_e150160_d_n11;
        locals.var_t3_dn14 = assign97780_e150160_d_n14;

        let assign97790_e150163: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2267 = assign97790_e150163;

        let (assign97800_e150171, assign97800_e150171_d_n0, assign97800_e150171_d_n2, assign97800_e150171_d_n4, assign97800_e150171_d_n5, assign97800_e150171_d_n6, assign97800_e150171_d_n7, assign97800_e150171_d_n8, assign97800_e150171_d_n9, assign97800_e150171_d_n10, assign97800_e150171_d_n11, assign97800_e150171_d_n14,) = {
    if ((locals.var_guard2266 != 0.0) && (locals.var_guard2267 != 0.0)) {
        let assign97800_e150169: f64 = (locals.var_vbs_jct * locals.var_jd_nvtm_invs);
        (assign97800_e150169, (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn0), ((locals.var_vbs_jct_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn2)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn7), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn8), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn9), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn10), ((locals.var_vbs_jct_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn11)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97800_e150171;
        locals.var_tx_dn0 = assign97800_e150171_d_n0;
        locals.var_tx_dn2 = assign97800_e150171_d_n2;
        locals.var_tx_dn4 = assign97800_e150171_d_n4;
        locals.var_tx_dn5 = assign97800_e150171_d_n5;
        locals.var_tx_dn6 = assign97800_e150171_d_n6;
        locals.var_tx_dn7 = assign97800_e150171_d_n7;
        locals.var_tx_dn8 = assign97800_e150171_d_n8;
        locals.var_tx_dn9 = assign97800_e150171_d_n9;
        locals.var_tx_dn10 = assign97800_e150171_d_n10;
        locals.var_tx_dn11 = assign97800_e150171_d_n11;
        locals.var_tx_dn14 = assign97800_e150171_d_n14;

        let assign97810_e150174: f64 = (-3.0);
        let assign97810_e150176: f64 = (assign97810_e150174 * 34.0);
        let assign97810_e150177: f64 = if locals.var_tx < assign97810_e150176 { 1.0 } else { 0.0 };
        locals.var_guard2268 = assign97810_e150177;

        let (assign97820_e150185, assign97820_e150185_d_n0, assign97820_e150185_d_n2, assign97820_e150185_d_n4, assign97820_e150185_d_n5, assign97820_e150185_d_n6, assign97820_e150185_d_n7, assign97820_e150185_d_n8, assign97820_e150185_d_n9, assign97820_e150185_d_n10, assign97820_e150185_d_n11, assign97820_e150185_d_n14,) = {
    if (((locals.var_guard2266 != 0.0) && (locals.var_guard2267 != 0.0)) && (locals.var_guard2268 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97820_e150185;
        locals.var_t1_dn0 = assign97820_e150185_d_n0;
        locals.var_t1_dn2 = assign97820_e150185_d_n2;
        locals.var_t1_dn4 = assign97820_e150185_d_n4;
        locals.var_t1_dn5 = assign97820_e150185_d_n5;
        locals.var_t1_dn6 = assign97820_e150185_d_n6;
        locals.var_t1_dn7 = assign97820_e150185_d_n7;
        locals.var_t1_dn8 = assign97820_e150185_d_n8;
        locals.var_t1_dn9 = assign97820_e150185_d_n9;
        locals.var_t1_dn10 = assign97820_e150185_d_n10;
        locals.var_t1_dn11 = assign97820_e150185_d_n11;
        locals.var_t1_dn14 = assign97820_e150185_d_n14;

        let (assign97830_e150195, assign97830_e150195_d_n0, assign97830_e150195_d_n2, assign97830_e150195_d_n4, assign97830_e150195_d_n5, assign97830_e150195_d_n6, assign97830_e150195_d_n7, assign97830_e150195_d_n8, assign97830_e150195_d_n9, assign97830_e150195_d_n10, assign97830_e150195_d_n11, assign97830_e150195_d_n14,) = {
    if (((locals.var_guard2266 != 0.0) && (locals.var_guard2267 != 0.0)) && (locals.var_guard2268 == 0.0)) {
        let assign97830_e150193: f64 = (locals.var_tx).exp();
        (assign97830_e150193, (assign97830_e150193 * locals.var_tx_dn0), (assign97830_e150193 * locals.var_tx_dn2), (assign97830_e150193 * locals.var_tx_dn4), (assign97830_e150193 * locals.var_tx_dn5), (assign97830_e150193 * locals.var_tx_dn6), (assign97830_e150193 * locals.var_tx_dn7), (assign97830_e150193 * locals.var_tx_dn8), (assign97830_e150193 * locals.var_tx_dn9), (assign97830_e150193 * locals.var_tx_dn10), (assign97830_e150193 * locals.var_tx_dn11), (assign97830_e150193 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97830_e150195;
        locals.var_t1_dn0 = assign97830_e150195_d_n0;
        locals.var_t1_dn2 = assign97830_e150195_d_n2;
        locals.var_t1_dn4 = assign97830_e150195_d_n4;
        locals.var_t1_dn5 = assign97830_e150195_d_n5;
        locals.var_t1_dn6 = assign97830_e150195_d_n6;
        locals.var_t1_dn7 = assign97830_e150195_d_n7;
        locals.var_t1_dn8 = assign97830_e150195_d_n8;
        locals.var_t1_dn9 = assign97830_e150195_d_n9;
        locals.var_t1_dn10 = assign97830_e150195_d_n10;
        locals.var_t1_dn11 = assign97830_e150195_d_n11;
        locals.var_t1_dn14 = assign97830_e150195_d_n14;

        let (assign97840_e150217, assign97840_e150217_d_n0, assign97840_e150217_d_n2, assign97840_e150217_d_n4, assign97840_e150217_d_n5, assign97840_e150217_d_n6, assign97840_e150217_d_n7, assign97840_e150217_d_n8, assign97840_e150217_d_n9, assign97840_e150217_d_n10, assign97840_e150217_d_n11, assign97840_e150217_d_n14,) = {
    if ((locals.var_guard2266 != 0.0) && (locals.var_guard2267 != 0.0)) {
        let assign97840_e150202: f64 = (locals.var_t1 - 1.0);
        let assign97840_e150203: f64 = (locals.var_isbs_btm * assign97840_e150202);
        let assign97840_e150207: f64 = (locals.var_t2 - 1.0);
        let assign97840_e150208: f64 = (locals.var_t0 * assign97840_e150207);
        let assign97840_e150209: f64 = (assign97840_e150203 + assign97840_e150208);
        let assign97840_e150213: f64 = (locals.var_t3 - 1.0);
        let assign97840_e150214: f64 = (locals.var_uc_cisbks * assign97840_e150213);
        let assign97840_e150215: f64 = (assign97840_e150209 + assign97840_e150214);
        (assign97840_e150215, ((((locals.var_isbs_btm_dn0 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), ((((locals.var_isbs_btm_dn2 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), ((((locals.var_isbs_btm_dn4 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), ((((locals.var_isbs_btm_dn5 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), ((((locals.var_isbs_btm_dn6 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), ((((locals.var_isbs_btm_dn7 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), ((((locals.var_isbs_btm_dn8 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), ((((locals.var_isbs_btm_dn9 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), ((((locals.var_isbs_btm_dn10 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), ((((locals.var_isbs_btm_dn11 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn11)) + ((locals.var_t0_dn11 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), ((((locals.var_isbs_btm_dn14 * assign97840_e150202) + (locals.var_isbs_btm * locals.var_t1_dn14)) + ((locals.var_t0_dn14 * assign97840_e150207) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_btm, locals.var_ibs_btm_dn0, locals.var_ibs_btm_dn2, locals.var_ibs_btm_dn4, locals.var_ibs_btm_dn5, locals.var_ibs_btm_dn6, locals.var_ibs_btm_dn7, locals.var_ibs_btm_dn8, locals.var_ibs_btm_dn9, locals.var_ibs_btm_dn10, locals.var_ibs_btm_dn11, locals.var_ibs_btm_dn14,)
    }
};
        locals.var_ibs_btm = assign97840_e150217;
        locals.var_ibs_btm_dn0 = assign97840_e150217_d_n0;
        locals.var_ibs_btm_dn2 = assign97840_e150217_d_n2;
        locals.var_ibs_btm_dn4 = assign97840_e150217_d_n4;
        locals.var_ibs_btm_dn5 = assign97840_e150217_d_n5;
        locals.var_ibs_btm_dn6 = assign97840_e150217_d_n6;
        locals.var_ibs_btm_dn7 = assign97840_e150217_d_n7;
        locals.var_ibs_btm_dn8 = assign97840_e150217_d_n8;
        locals.var_ibs_btm_dn9 = assign97840_e150217_d_n9;
        locals.var_ibs_btm_dn10 = assign97840_e150217_d_n10;
        locals.var_ibs_btm_dn11 = assign97840_e150217_d_n11;
        locals.var_ibs_btm_dn14 = assign97840_e150217_d_n14;

        let (assign97850_e150224, assign97850_e150224_d_n0, assign97850_e150224_d_n2, assign97850_e150224_d_n4, assign97850_e150224_d_n5, assign97850_e150224_d_n6, assign97850_e150224_d_n7, assign97850_e150224_d_n8, assign97850_e150224_d_n9, assign97850_e150224_d_n10, assign97850_e150224_d_n11, assign97850_e150224_d_n14,) = {
    if ((locals.var_guard2266 != 0.0) && (locals.var_guard2267 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97850_e150224;
        locals.var_t1_dn0 = assign97850_e150224_d_n0;
        locals.var_t1_dn2 = assign97850_e150224_d_n2;
        locals.var_t1_dn4 = assign97850_e150224_d_n4;
        locals.var_t1_dn5 = assign97850_e150224_d_n5;
        locals.var_t1_dn6 = assign97850_e150224_d_n6;
        locals.var_t1_dn7 = assign97850_e150224_d_n7;
        locals.var_t1_dn8 = assign97850_e150224_d_n8;
        locals.var_t1_dn9 = assign97850_e150224_d_n9;
        locals.var_t1_dn10 = assign97850_e150224_d_n10;
        locals.var_t1_dn11 = assign97850_e150224_d_n11;
        locals.var_t1_dn14 = assign97850_e150224_d_n14;

        let (assign97860_e150235, assign97860_e150235_d_n0, assign97860_e150235_d_n2, assign97860_e150235_d_n4, assign97860_e150235_d_n5, assign97860_e150235_d_n6, assign97860_e150235_d_n7, assign97860_e150235_d_n8, assign97860_e150235_d_n9, assign97860_e150235_d_n10, assign97860_e150235_d_n11, assign97860_e150235_d_n14,) = {
    if ((locals.var_guard2266 != 0.0) && (locals.var_guard2267 == 0.0)) {
        let assign97860_e150231: f64 = (locals.var_isbs_btm * locals.var_jd_nvtm_invs);
        let assign97860_e150233: f64 = (assign97860_e150231 * locals.var_t1);
        (assign97860_e150233, ((((locals.var_isbs_btm_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn0)), ((((locals.var_isbs_btm_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn2)), ((((locals.var_isbs_btm_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn4)), ((((locals.var_isbs_btm_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn5)), ((((locals.var_isbs_btm_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn6)), ((((locals.var_isbs_btm_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn7)), ((((locals.var_isbs_btm_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn8)), ((((locals.var_isbs_btm_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn9)), ((((locals.var_isbs_btm_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn10)), ((((locals.var_isbs_btm_dn11 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn11)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn11)), ((((locals.var_isbs_btm_dn14 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn14)) * locals.var_t1) + (assign97860_e150231 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97860_e150235;
        locals.var_t4_dn0 = assign97860_e150235_d_n0;
        locals.var_t4_dn2 = assign97860_e150235_d_n2;
        locals.var_t4_dn4 = assign97860_e150235_d_n4;
        locals.var_t4_dn5 = assign97860_e150235_d_n5;
        locals.var_t4_dn6 = assign97860_e150235_d_n6;
        locals.var_t4_dn7 = assign97860_e150235_d_n7;
        locals.var_t4_dn8 = assign97860_e150235_d_n8;
        locals.var_t4_dn9 = assign97860_e150235_d_n9;
        locals.var_t4_dn10 = assign97860_e150235_d_n10;
        locals.var_t4_dn11 = assign97860_e150235_d_n11;
        locals.var_t4_dn14 = assign97860_e150235_d_n14;

        let (assign97870_e150264, assign97870_e150264_d_n0, assign97870_e150264_d_n2, assign97870_e150264_d_n4, assign97870_e150264_d_n5, assign97870_e150264_d_n6, assign97870_e150264_d_n7, assign97870_e150264_d_n8, assign97870_e150264_d_n9, assign97870_e150264_d_n10, assign97870_e150264_d_n11, assign97870_e150264_d_n14,) = {
    if ((locals.var_guard2266 != 0.0) && (locals.var_guard2267 == 0.0)) {
        let assign97870_e150243: f64 = (locals.var_t1 - 1.0);
        let assign97870_e150244: f64 = (locals.var_isbs_btm * assign97870_e150243);
        let assign97870_e150248: f64 = (locals.var_vbs_jct - locals.var_vbst);
        let assign97870_e150249: f64 = (locals.var_t4 * assign97870_e150248);
        let assign97870_e150250: f64 = (assign97870_e150244 + assign97870_e150249);
        let assign97870_e150254: f64 = (locals.var_t2 - 1.0);
        let assign97870_e150255: f64 = (locals.var_t0 * assign97870_e150254);
        let assign97870_e150256: f64 = (assign97870_e150250 + assign97870_e150255);
        let assign97870_e150260: f64 = (locals.var_t3 - 1.0);
        let assign97870_e150261: f64 = (locals.var_uc_cisbks * assign97870_e150260);
        let assign97870_e150262: f64 = (assign97870_e150256 + assign97870_e150261);
        (assign97870_e150262, (((((locals.var_isbs_btm_dn0 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign97870_e150248) + (locals.var_t4 * (-locals.var_vbst_dn0)))) + ((locals.var_t0_dn0 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), (((((locals.var_isbs_btm_dn2 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign97870_e150248) + (locals.var_t4 * (locals.var_vbs_jct_dn2 - locals.var_vbst_dn2)))) + ((locals.var_t0_dn2 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), (((((locals.var_isbs_btm_dn4 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign97870_e150248) + (locals.var_t4 * (-locals.var_vbst_dn4)))) + ((locals.var_t0_dn4 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), (((((locals.var_isbs_btm_dn5 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign97870_e150248) + (locals.var_t4 * (-locals.var_vbst_dn5)))) + ((locals.var_t0_dn5 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), (((((locals.var_isbs_btm_dn6 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign97870_e150248) + (locals.var_t4 * (-locals.var_vbst_dn6)))) + ((locals.var_t0_dn6 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), (((((locals.var_isbs_btm_dn7 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign97870_e150248) + (locals.var_t4 * (-locals.var_vbst_dn7)))) + ((locals.var_t0_dn7 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), (((((locals.var_isbs_btm_dn8 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign97870_e150248) + (locals.var_t4 * (-locals.var_vbst_dn8)))) + ((locals.var_t0_dn8 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), (((((locals.var_isbs_btm_dn9 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign97870_e150248) + (locals.var_t4 * (-locals.var_vbst_dn9)))) + ((locals.var_t0_dn9 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), (((((locals.var_isbs_btm_dn10 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign97870_e150248) + (locals.var_t4 * (-locals.var_vbst_dn10)))) + ((locals.var_t0_dn10 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), (((((locals.var_isbs_btm_dn11 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * assign97870_e150248) + (locals.var_t4 * (locals.var_vbs_jct_dn11 - locals.var_vbst_dn11)))) + ((locals.var_t0_dn11 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), (((((locals.var_isbs_btm_dn14 * assign97870_e150243) + (locals.var_isbs_btm * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * assign97870_e150248) + (locals.var_t4 * (-locals.var_vbst_dn14)))) + ((locals.var_t0_dn14 * assign97870_e150254) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_btm, locals.var_ibs_btm_dn0, locals.var_ibs_btm_dn2, locals.var_ibs_btm_dn4, locals.var_ibs_btm_dn5, locals.var_ibs_btm_dn6, locals.var_ibs_btm_dn7, locals.var_ibs_btm_dn8, locals.var_ibs_btm_dn9, locals.var_ibs_btm_dn10, locals.var_ibs_btm_dn11, locals.var_ibs_btm_dn14,)
    }
};
        locals.var_ibs_btm = assign97870_e150264;
        locals.var_ibs_btm_dn0 = assign97870_e150264_d_n0;
        locals.var_ibs_btm_dn2 = assign97870_e150264_d_n2;
        locals.var_ibs_btm_dn4 = assign97870_e150264_d_n4;
        locals.var_ibs_btm_dn5 = assign97870_e150264_d_n5;
        locals.var_ibs_btm_dn6 = assign97870_e150264_d_n6;
        locals.var_ibs_btm_dn7 = assign97870_e150264_d_n7;
        locals.var_ibs_btm_dn8 = assign97870_e150264_d_n8;
        locals.var_ibs_btm_dn9 = assign97870_e150264_d_n9;
        locals.var_ibs_btm_dn10 = assign97870_e150264_d_n10;
        locals.var_ibs_btm_dn11 = assign97870_e150264_d_n11;
        locals.var_ibs_btm_dn14 = assign97870_e150264_d_n14;

        let (assign97880_e150269, assign97880_e150269_d_n0, assign97880_e150269_d_n2, assign97880_e150269_d_n4, assign97880_e150269_d_n5, assign97880_e150269_d_n6, assign97880_e150269_d_n7, assign97880_e150269_d_n8, assign97880_e150269_d_n9, assign97880_e150269_d_n10, assign97880_e150269_d_n11, assign97880_e150269_d_n14,) = {
    if (locals.var_guard2266 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_btm, locals.var_ibs_btm_dn0, locals.var_ibs_btm_dn2, locals.var_ibs_btm_dn4, locals.var_ibs_btm_dn5, locals.var_ibs_btm_dn6, locals.var_ibs_btm_dn7, locals.var_ibs_btm_dn8, locals.var_ibs_btm_dn9, locals.var_ibs_btm_dn10, locals.var_ibs_btm_dn11, locals.var_ibs_btm_dn14,)
    }
};
        locals.var_ibs_btm = assign97880_e150269;
        locals.var_ibs_btm_dn0 = assign97880_e150269_d_n0;
        locals.var_ibs_btm_dn2 = assign97880_e150269_d_n2;
        locals.var_ibs_btm_dn4 = assign97880_e150269_d_n4;
        locals.var_ibs_btm_dn5 = assign97880_e150269_d_n5;
        locals.var_ibs_btm_dn6 = assign97880_e150269_d_n6;
        locals.var_ibs_btm_dn7 = assign97880_e150269_d_n7;
        locals.var_ibs_btm_dn8 = assign97880_e150269_d_n8;
        locals.var_ibs_btm_dn9 = assign97880_e150269_d_n9;
        locals.var_ibs_btm_dn10 = assign97880_e150269_d_n10;
        locals.var_ibs_btm_dn11 = assign97880_e150269_d_n11;
        locals.var_ibs_btm_dn14 = assign97880_e150269_d_n14;

        let assign97890_e150272: f64 = (p.p537 * locals.var_isbs2_btm);
        locals.var_t12 = assign97890_e150272;
        locals.var_t12_dn0 = (p.p537 * locals.var_isbs2_btm_dn0);
        locals.var_t12_dn2 = (p.p537 * locals.var_isbs2_btm_dn2);
        locals.var_t12_dn4 = (p.p537 * locals.var_isbs2_btm_dn4);
        locals.var_t12_dn5 = (p.p537 * locals.var_isbs2_btm_dn5);
        locals.var_t12_dn6 = (p.p537 * locals.var_isbs2_btm_dn6);
        locals.var_t12_dn7 = (p.p537 * locals.var_isbs2_btm_dn7);
        locals.var_t12_dn8 = (p.p537 * locals.var_isbs2_btm_dn8);
        locals.var_t12_dn9 = (p.p537 * locals.var_isbs2_btm_dn9);
        locals.var_t12_dn10 = (p.p537 * locals.var_isbs2_btm_dn10);
        locals.var_t12_dn11 = (p.p537 * locals.var_isbs2_btm_dn11);
        locals.var_t12_dn14 = (p.p537 * locals.var_isbs2_btm_dn14);

        let assign97900_e150276: f64 = (locals.var_t12 * locals.var_vbs_jct);
        let assign97900_e150277: f64 = (locals.var_ibs_btm + assign97900_e150276);
        locals.var_ibs_btm = assign97900_e150277;
        locals.var_ibs_btm_dn0 = (locals.var_ibs_btm_dn0 + (locals.var_t12_dn0 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn2 = (locals.var_ibs_btm_dn2 + ((locals.var_t12_dn2 * locals.var_vbs_jct) + (locals.var_t12 * locals.var_vbs_jct_dn2)));
        locals.var_ibs_btm_dn4 = (locals.var_ibs_btm_dn4 + (locals.var_t12_dn4 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn5 = (locals.var_ibs_btm_dn5 + (locals.var_t12_dn5 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn6 = (locals.var_ibs_btm_dn6 + (locals.var_t12_dn6 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn7 = (locals.var_ibs_btm_dn7 + (locals.var_t12_dn7 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn8 = (locals.var_ibs_btm_dn8 + (locals.var_t12_dn8 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn9 = (locals.var_ibs_btm_dn9 + (locals.var_t12_dn9 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn10 = (locals.var_ibs_btm_dn10 + (locals.var_t12_dn10 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn11 = (locals.var_ibs_btm_dn11 + ((locals.var_t12_dn11 * locals.var_vbs_jct) + (locals.var_t12 * locals.var_vbs_jct_dn11)));
        locals.var_ibs_btm_dn14 = (locals.var_ibs_btm_dn14 + (locals.var_t12_dn14 * locals.var_vbs_jct));

        let assign97910_e150280: f64 = if locals.var_isbs_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2269 = assign97910_e150280;

        let (assign97920_e150286, assign97920_e150286_d_n0, assign97920_e150286_d_n2, assign97920_e150286_d_n4, assign97920_e150286_d_n5, assign97920_e150286_d_n6, assign97920_e150286_d_n7, assign97920_e150286_d_n8, assign97920_e150286_d_n9, assign97920_e150286_d_n10, assign97920_e150286_d_n11, assign97920_e150286_d_n14,) = {
    if (locals.var_guard2269 != 0.0) {
        let assign97920_e150284: f64 = (locals.var_isbs2_sws * locals.var_t9);
        (assign97920_e150284, ((locals.var_isbs2_sws_dn0 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn0)), ((locals.var_isbs2_sws_dn2 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn2)), ((locals.var_isbs2_sws_dn4 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn4)), ((locals.var_isbs2_sws_dn5 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn5)), ((locals.var_isbs2_sws_dn6 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn6)), ((locals.var_isbs2_sws_dn7 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn7)), ((locals.var_isbs2_sws_dn8 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn8)), ((locals.var_isbs2_sws_dn9 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn9)), ((locals.var_isbs2_sws_dn10 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn10)), ((locals.var_isbs2_sws_dn11 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn11)), ((locals.var_isbs2_sws_dn14 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97920_e150286;
        locals.var_t0_dn0 = assign97920_e150286_d_n0;
        locals.var_t0_dn2 = assign97920_e150286_d_n2;
        locals.var_t0_dn4 = assign97920_e150286_d_n4;
        locals.var_t0_dn5 = assign97920_e150286_d_n5;
        locals.var_t0_dn6 = assign97920_e150286_d_n6;
        locals.var_t0_dn7 = assign97920_e150286_d_n7;
        locals.var_t0_dn8 = assign97920_e150286_d_n8;
        locals.var_t0_dn9 = assign97920_e150286_d_n9;
        locals.var_t0_dn10 = assign97920_e150286_d_n10;
        locals.var_t0_dn11 = assign97920_e150286_d_n11;
        locals.var_t0_dn14 = assign97920_e150286_d_n14;

        let (assign97930_e150293, assign97930_e150293_d_n0, assign97930_e150293_d_n2, assign97930_e150293_d_n4, assign97930_e150293_d_n5, assign97930_e150293_d_n6, assign97930_e150293_d_n7, assign97930_e150293_d_n8, assign97930_e150293_d_n9, assign97930_e150293_d_n10, assign97930_e150293_d_n11, assign97930_e150293_d_n14,) = {
    if (locals.var_guard2269 != 0.0) {
        let assign97930_e150289: f64 = (-locals.var_vbs_jct);
        let assign97930_e150291: f64 = (assign97930_e150289 * locals.var_t10);
        (assign97930_e150291, (assign97930_e150289 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97930_e150289 * locals.var_t10_dn2)), (assign97930_e150289 * locals.var_t10_dn4), (assign97930_e150289 * locals.var_t10_dn5), (assign97930_e150289 * locals.var_t10_dn6), (assign97930_e150289 * locals.var_t10_dn7), (assign97930_e150289 * locals.var_t10_dn8), (assign97930_e150289 * locals.var_t10_dn9), (assign97930_e150289 * locals.var_t10_dn10), (((-locals.var_vbs_jct_dn11) * locals.var_t10) + (assign97930_e150289 * locals.var_t10_dn11)), (assign97930_e150289 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97930_e150293;
        locals.var_tx_dn0 = assign97930_e150293_d_n0;
        locals.var_tx_dn2 = assign97930_e150293_d_n2;
        locals.var_tx_dn4 = assign97930_e150293_d_n4;
        locals.var_tx_dn5 = assign97930_e150293_d_n5;
        locals.var_tx_dn6 = assign97930_e150293_d_n6;
        locals.var_tx_dn7 = assign97930_e150293_d_n7;
        locals.var_tx_dn8 = assign97930_e150293_d_n8;
        locals.var_tx_dn9 = assign97930_e150293_d_n9;
        locals.var_tx_dn10 = assign97930_e150293_d_n10;
        locals.var_tx_dn11 = assign97930_e150293_d_n11;
        locals.var_tx_dn14 = assign97930_e150293_d_n14;

        let (assign97940_e150298, assign97940_e150298_d_n0, assign97940_e150298_d_n2, assign97940_e150298_d_n4, assign97940_e150298_d_n5, assign97940_e150298_d_n6, assign97940_e150298_d_n7, assign97940_e150298_d_n8, assign97940_e150298_d_n9, assign97940_e150298_d_n10, assign97940_e150298_d_n11, assign97940_e150298_d_n14,) = {
    if (locals.var_guard2269 != 0.0) {
        let assign97940_e150296: f64 = (locals.var_tx).exp();
        (assign97940_e150296, (assign97940_e150296 * locals.var_tx_dn0), (assign97940_e150296 * locals.var_tx_dn2), (assign97940_e150296 * locals.var_tx_dn4), (assign97940_e150296 * locals.var_tx_dn5), (assign97940_e150296 * locals.var_tx_dn6), (assign97940_e150296 * locals.var_tx_dn7), (assign97940_e150296 * locals.var_tx_dn8), (assign97940_e150296 * locals.var_tx_dn9), (assign97940_e150296 * locals.var_tx_dn10), (assign97940_e150296 * locals.var_tx_dn11), (assign97940_e150296 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97940_e150298;
        locals.var_t2_dn0 = assign97940_e150298_d_n0;
        locals.var_t2_dn2 = assign97940_e150298_d_n2;
        locals.var_t2_dn4 = assign97940_e150298_d_n4;
        locals.var_t2_dn5 = assign97940_e150298_d_n5;
        locals.var_t2_dn6 = assign97940_e150298_d_n6;
        locals.var_t2_dn7 = assign97940_e150298_d_n7;
        locals.var_t2_dn8 = assign97940_e150298_d_n8;
        locals.var_t2_dn9 = assign97940_e150298_d_n9;
        locals.var_t2_dn10 = assign97940_e150298_d_n10;
        locals.var_t2_dn11 = assign97940_e150298_d_n11;
        locals.var_t2_dn14 = assign97940_e150298_d_n14;

        let (assign97950_e150302, assign97950_e150302_d_n0, assign97950_e150302_d_n2, assign97950_e150302_d_n4, assign97950_e150302_d_n5, assign97950_e150302_d_n6, assign97950_e150302_d_n7, assign97950_e150302_d_n8, assign97950_e150302_d_n9, assign97950_e150302_d_n10, assign97950_e150302_d_n11, assign97950_e150302_d_n14,) = {
    if (locals.var_guard2269 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97950_e150302;
        locals.var_t3_dn0 = assign97950_e150302_d_n0;
        locals.var_t3_dn2 = assign97950_e150302_d_n2;
        locals.var_t3_dn4 = assign97950_e150302_d_n4;
        locals.var_t3_dn5 = assign97950_e150302_d_n5;
        locals.var_t3_dn6 = assign97950_e150302_d_n6;
        locals.var_t3_dn7 = assign97950_e150302_d_n7;
        locals.var_t3_dn8 = assign97950_e150302_d_n8;
        locals.var_t3_dn9 = assign97950_e150302_d_n9;
        locals.var_t3_dn10 = assign97950_e150302_d_n10;
        locals.var_t3_dn11 = assign97950_e150302_d_n11;
        locals.var_t3_dn14 = assign97950_e150302_d_n14;

        let assign97960_e150305: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2270 = assign97960_e150305;

        let (assign97970_e150313, assign97970_e150313_d_n0, assign97970_e150313_d_n2, assign97970_e150313_d_n4, assign97970_e150313_d_n5, assign97970_e150313_d_n6, assign97970_e150313_d_n7, assign97970_e150313_d_n8, assign97970_e150313_d_n9, assign97970_e150313_d_n10, assign97970_e150313_d_n11, assign97970_e150313_d_n14,) = {
    if ((locals.var_guard2269 != 0.0) && (locals.var_guard2270 != 0.0)) {
        let assign97970_e150311: f64 = (locals.var_vbs_jct * locals.var_jd_nvtm_invs);
        (assign97970_e150311, (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn0), ((locals.var_vbs_jct_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn2)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn7), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn8), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn9), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn10), ((locals.var_vbs_jct_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn11)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97970_e150313;
        locals.var_tx_dn0 = assign97970_e150313_d_n0;
        locals.var_tx_dn2 = assign97970_e150313_d_n2;
        locals.var_tx_dn4 = assign97970_e150313_d_n4;
        locals.var_tx_dn5 = assign97970_e150313_d_n5;
        locals.var_tx_dn6 = assign97970_e150313_d_n6;
        locals.var_tx_dn7 = assign97970_e150313_d_n7;
        locals.var_tx_dn8 = assign97970_e150313_d_n8;
        locals.var_tx_dn9 = assign97970_e150313_d_n9;
        locals.var_tx_dn10 = assign97970_e150313_d_n10;
        locals.var_tx_dn11 = assign97970_e150313_d_n11;
        locals.var_tx_dn14 = assign97970_e150313_d_n14;

        let assign97980_e150316: f64 = (-3.0);
        let assign97980_e150318: f64 = (assign97980_e150316 * 34.0);
        let assign97980_e150319: f64 = if locals.var_tx < assign97980_e150318 { 1.0 } else { 0.0 };
        locals.var_guard2271 = assign97980_e150319;

    }

    pub(super) fn stamp_transient_block_361(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign97990_e150327, assign97990_e150327_d_n0, assign97990_e150327_d_n2, assign97990_e150327_d_n4, assign97990_e150327_d_n5, assign97990_e150327_d_n6, assign97990_e150327_d_n7, assign97990_e150327_d_n8, assign97990_e150327_d_n9, assign97990_e150327_d_n10, assign97990_e150327_d_n11, assign97990_e150327_d_n14,) = {
    if (((locals.var_guard2269 != 0.0) && (locals.var_guard2270 != 0.0)) && (locals.var_guard2271 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97990_e150327;
        locals.var_t1_dn0 = assign97990_e150327_d_n0;
        locals.var_t1_dn2 = assign97990_e150327_d_n2;
        locals.var_t1_dn4 = assign97990_e150327_d_n4;
        locals.var_t1_dn5 = assign97990_e150327_d_n5;
        locals.var_t1_dn6 = assign97990_e150327_d_n6;
        locals.var_t1_dn7 = assign97990_e150327_d_n7;
        locals.var_t1_dn8 = assign97990_e150327_d_n8;
        locals.var_t1_dn9 = assign97990_e150327_d_n9;
        locals.var_t1_dn10 = assign97990_e150327_d_n10;
        locals.var_t1_dn11 = assign97990_e150327_d_n11;
        locals.var_t1_dn14 = assign97990_e150327_d_n14;

        let (assign98000_e150337, assign98000_e150337_d_n0, assign98000_e150337_d_n2, assign98000_e150337_d_n4, assign98000_e150337_d_n5, assign98000_e150337_d_n6, assign98000_e150337_d_n7, assign98000_e150337_d_n8, assign98000_e150337_d_n9, assign98000_e150337_d_n10, assign98000_e150337_d_n11, assign98000_e150337_d_n14,) = {
    if (((locals.var_guard2269 != 0.0) && (locals.var_guard2270 != 0.0)) && (locals.var_guard2271 == 0.0)) {
        let assign98000_e150335: f64 = (locals.var_tx).exp();
        (assign98000_e150335, (assign98000_e150335 * locals.var_tx_dn0), (assign98000_e150335 * locals.var_tx_dn2), (assign98000_e150335 * locals.var_tx_dn4), (assign98000_e150335 * locals.var_tx_dn5), (assign98000_e150335 * locals.var_tx_dn6), (assign98000_e150335 * locals.var_tx_dn7), (assign98000_e150335 * locals.var_tx_dn8), (assign98000_e150335 * locals.var_tx_dn9), (assign98000_e150335 * locals.var_tx_dn10), (assign98000_e150335 * locals.var_tx_dn11), (assign98000_e150335 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98000_e150337;
        locals.var_t1_dn0 = assign98000_e150337_d_n0;
        locals.var_t1_dn2 = assign98000_e150337_d_n2;
        locals.var_t1_dn4 = assign98000_e150337_d_n4;
        locals.var_t1_dn5 = assign98000_e150337_d_n5;
        locals.var_t1_dn6 = assign98000_e150337_d_n6;
        locals.var_t1_dn7 = assign98000_e150337_d_n7;
        locals.var_t1_dn8 = assign98000_e150337_d_n8;
        locals.var_t1_dn9 = assign98000_e150337_d_n9;
        locals.var_t1_dn10 = assign98000_e150337_d_n10;
        locals.var_t1_dn11 = assign98000_e150337_d_n11;
        locals.var_t1_dn14 = assign98000_e150337_d_n14;

        let (assign98010_e150359, assign98010_e150359_d_n0, assign98010_e150359_d_n2, assign98010_e150359_d_n4, assign98010_e150359_d_n5, assign98010_e150359_d_n6, assign98010_e150359_d_n7, assign98010_e150359_d_n8, assign98010_e150359_d_n9, assign98010_e150359_d_n10, assign98010_e150359_d_n11, assign98010_e150359_d_n14,) = {
    if ((locals.var_guard2269 != 0.0) && (locals.var_guard2270 != 0.0)) {
        let assign98010_e150344: f64 = (locals.var_t1 - 1.0);
        let assign98010_e150345: f64 = (locals.var_isbs_sws * assign98010_e150344);
        let assign98010_e150349: f64 = (locals.var_t2 - 1.0);
        let assign98010_e150350: f64 = (locals.var_t0 * assign98010_e150349);
        let assign98010_e150351: f64 = (assign98010_e150345 + assign98010_e150350);
        let assign98010_e150355: f64 = (locals.var_t3 - 1.0);
        let assign98010_e150356: f64 = (locals.var_uc_cisbks * assign98010_e150355);
        let assign98010_e150357: f64 = (assign98010_e150351 + assign98010_e150356);
        (assign98010_e150357, ((((locals.var_isbs_sws_dn0 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), ((((locals.var_isbs_sws_dn2 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), ((((locals.var_isbs_sws_dn4 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), ((((locals.var_isbs_sws_dn5 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), ((((locals.var_isbs_sws_dn6 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), ((((locals.var_isbs_sws_dn7 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), ((((locals.var_isbs_sws_dn8 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), ((((locals.var_isbs_sws_dn9 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), ((((locals.var_isbs_sws_dn10 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), ((((locals.var_isbs_sws_dn11 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn11)) + ((locals.var_t0_dn11 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), ((((locals.var_isbs_sws_dn14 * assign98010_e150344) + (locals.var_isbs_sws * locals.var_t1_dn14)) + ((locals.var_t0_dn14 * assign98010_e150349) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_sws, locals.var_ibs_sws_dn0, locals.var_ibs_sws_dn2, locals.var_ibs_sws_dn4, locals.var_ibs_sws_dn5, locals.var_ibs_sws_dn6, locals.var_ibs_sws_dn7, locals.var_ibs_sws_dn8, locals.var_ibs_sws_dn9, locals.var_ibs_sws_dn10, locals.var_ibs_sws_dn11, locals.var_ibs_sws_dn14,)
    }
};
        locals.var_ibs_sws = assign98010_e150359;
        locals.var_ibs_sws_dn0 = assign98010_e150359_d_n0;
        locals.var_ibs_sws_dn2 = assign98010_e150359_d_n2;
        locals.var_ibs_sws_dn4 = assign98010_e150359_d_n4;
        locals.var_ibs_sws_dn5 = assign98010_e150359_d_n5;
        locals.var_ibs_sws_dn6 = assign98010_e150359_d_n6;
        locals.var_ibs_sws_dn7 = assign98010_e150359_d_n7;
        locals.var_ibs_sws_dn8 = assign98010_e150359_d_n8;
        locals.var_ibs_sws_dn9 = assign98010_e150359_d_n9;
        locals.var_ibs_sws_dn10 = assign98010_e150359_d_n10;
        locals.var_ibs_sws_dn11 = assign98010_e150359_d_n11;
        locals.var_ibs_sws_dn14 = assign98010_e150359_d_n14;

        let (assign98020_e150366, assign98020_e150366_d_n0, assign98020_e150366_d_n2, assign98020_e150366_d_n4, assign98020_e150366_d_n5, assign98020_e150366_d_n6, assign98020_e150366_d_n7, assign98020_e150366_d_n8, assign98020_e150366_d_n9, assign98020_e150366_d_n10, assign98020_e150366_d_n11, assign98020_e150366_d_n14,) = {
    if ((locals.var_guard2269 != 0.0) && (locals.var_guard2270 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98020_e150366;
        locals.var_t1_dn0 = assign98020_e150366_d_n0;
        locals.var_t1_dn2 = assign98020_e150366_d_n2;
        locals.var_t1_dn4 = assign98020_e150366_d_n4;
        locals.var_t1_dn5 = assign98020_e150366_d_n5;
        locals.var_t1_dn6 = assign98020_e150366_d_n6;
        locals.var_t1_dn7 = assign98020_e150366_d_n7;
        locals.var_t1_dn8 = assign98020_e150366_d_n8;
        locals.var_t1_dn9 = assign98020_e150366_d_n9;
        locals.var_t1_dn10 = assign98020_e150366_d_n10;
        locals.var_t1_dn11 = assign98020_e150366_d_n11;
        locals.var_t1_dn14 = assign98020_e150366_d_n14;

        let (assign98030_e150377, assign98030_e150377_d_n0, assign98030_e150377_d_n2, assign98030_e150377_d_n4, assign98030_e150377_d_n5, assign98030_e150377_d_n6, assign98030_e150377_d_n7, assign98030_e150377_d_n8, assign98030_e150377_d_n9, assign98030_e150377_d_n10, assign98030_e150377_d_n11, assign98030_e150377_d_n14,) = {
    if ((locals.var_guard2269 != 0.0) && (locals.var_guard2270 == 0.0)) {
        let assign98030_e150373: f64 = (locals.var_isbs_sws * locals.var_jd_nvtm_invs);
        let assign98030_e150375: f64 = (assign98030_e150373 * locals.var_t1);
        (assign98030_e150375, ((((locals.var_isbs_sws_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn0)), ((((locals.var_isbs_sws_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn2)), ((((locals.var_isbs_sws_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn4)), ((((locals.var_isbs_sws_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn5)), ((((locals.var_isbs_sws_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn6)), ((((locals.var_isbs_sws_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn7)), ((((locals.var_isbs_sws_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn8)), ((((locals.var_isbs_sws_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn9)), ((((locals.var_isbs_sws_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn10)), ((((locals.var_isbs_sws_dn11 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn11)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn11)), ((((locals.var_isbs_sws_dn14 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn14)) * locals.var_t1) + (assign98030_e150373 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign98030_e150377;
        locals.var_t4_dn0 = assign98030_e150377_d_n0;
        locals.var_t4_dn2 = assign98030_e150377_d_n2;
        locals.var_t4_dn4 = assign98030_e150377_d_n4;
        locals.var_t4_dn5 = assign98030_e150377_d_n5;
        locals.var_t4_dn6 = assign98030_e150377_d_n6;
        locals.var_t4_dn7 = assign98030_e150377_d_n7;
        locals.var_t4_dn8 = assign98030_e150377_d_n8;
        locals.var_t4_dn9 = assign98030_e150377_d_n9;
        locals.var_t4_dn10 = assign98030_e150377_d_n10;
        locals.var_t4_dn11 = assign98030_e150377_d_n11;
        locals.var_t4_dn14 = assign98030_e150377_d_n14;

        let (assign98040_e150406, assign98040_e150406_d_n0, assign98040_e150406_d_n2, assign98040_e150406_d_n4, assign98040_e150406_d_n5, assign98040_e150406_d_n6, assign98040_e150406_d_n7, assign98040_e150406_d_n8, assign98040_e150406_d_n9, assign98040_e150406_d_n10, assign98040_e150406_d_n11, assign98040_e150406_d_n14,) = {
    if ((locals.var_guard2269 != 0.0) && (locals.var_guard2270 == 0.0)) {
        let assign98040_e150385: f64 = (locals.var_t1 - 1.0);
        let assign98040_e150386: f64 = (locals.var_isbs_sws * assign98040_e150385);
        let assign98040_e150390: f64 = (locals.var_vbs_jct - locals.var_vbst);
        let assign98040_e150391: f64 = (locals.var_t4 * assign98040_e150390);
        let assign98040_e150392: f64 = (assign98040_e150386 + assign98040_e150391);
        let assign98040_e150396: f64 = (locals.var_t2 - 1.0);
        let assign98040_e150397: f64 = (locals.var_t0 * assign98040_e150396);
        let assign98040_e150398: f64 = (assign98040_e150392 + assign98040_e150397);
        let assign98040_e150402: f64 = (locals.var_t3 - 1.0);
        let assign98040_e150403: f64 = (locals.var_uc_cisbks * assign98040_e150402);
        let assign98040_e150404: f64 = (assign98040_e150398 + assign98040_e150403);
        (assign98040_e150404, (((((locals.var_isbs_sws_dn0 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign98040_e150390) + (locals.var_t4 * (-locals.var_vbst_dn0)))) + ((locals.var_t0_dn0 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), (((((locals.var_isbs_sws_dn2 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign98040_e150390) + (locals.var_t4 * (locals.var_vbs_jct_dn2 - locals.var_vbst_dn2)))) + ((locals.var_t0_dn2 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), (((((locals.var_isbs_sws_dn4 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign98040_e150390) + (locals.var_t4 * (-locals.var_vbst_dn4)))) + ((locals.var_t0_dn4 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), (((((locals.var_isbs_sws_dn5 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign98040_e150390) + (locals.var_t4 * (-locals.var_vbst_dn5)))) + ((locals.var_t0_dn5 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), (((((locals.var_isbs_sws_dn6 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign98040_e150390) + (locals.var_t4 * (-locals.var_vbst_dn6)))) + ((locals.var_t0_dn6 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), (((((locals.var_isbs_sws_dn7 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign98040_e150390) + (locals.var_t4 * (-locals.var_vbst_dn7)))) + ((locals.var_t0_dn7 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), (((((locals.var_isbs_sws_dn8 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign98040_e150390) + (locals.var_t4 * (-locals.var_vbst_dn8)))) + ((locals.var_t0_dn8 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), (((((locals.var_isbs_sws_dn9 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign98040_e150390) + (locals.var_t4 * (-locals.var_vbst_dn9)))) + ((locals.var_t0_dn9 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), (((((locals.var_isbs_sws_dn10 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign98040_e150390) + (locals.var_t4 * (-locals.var_vbst_dn10)))) + ((locals.var_t0_dn10 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), (((((locals.var_isbs_sws_dn11 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * assign98040_e150390) + (locals.var_t4 * (locals.var_vbs_jct_dn11 - locals.var_vbst_dn11)))) + ((locals.var_t0_dn11 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), (((((locals.var_isbs_sws_dn14 * assign98040_e150385) + (locals.var_isbs_sws * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * assign98040_e150390) + (locals.var_t4 * (-locals.var_vbst_dn14)))) + ((locals.var_t0_dn14 * assign98040_e150396) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_sws, locals.var_ibs_sws_dn0, locals.var_ibs_sws_dn2, locals.var_ibs_sws_dn4, locals.var_ibs_sws_dn5, locals.var_ibs_sws_dn6, locals.var_ibs_sws_dn7, locals.var_ibs_sws_dn8, locals.var_ibs_sws_dn9, locals.var_ibs_sws_dn10, locals.var_ibs_sws_dn11, locals.var_ibs_sws_dn14,)
    }
};
        locals.var_ibs_sws = assign98040_e150406;
        locals.var_ibs_sws_dn0 = assign98040_e150406_d_n0;
        locals.var_ibs_sws_dn2 = assign98040_e150406_d_n2;
        locals.var_ibs_sws_dn4 = assign98040_e150406_d_n4;
        locals.var_ibs_sws_dn5 = assign98040_e150406_d_n5;
        locals.var_ibs_sws_dn6 = assign98040_e150406_d_n6;
        locals.var_ibs_sws_dn7 = assign98040_e150406_d_n7;
        locals.var_ibs_sws_dn8 = assign98040_e150406_d_n8;
        locals.var_ibs_sws_dn9 = assign98040_e150406_d_n9;
        locals.var_ibs_sws_dn10 = assign98040_e150406_d_n10;
        locals.var_ibs_sws_dn11 = assign98040_e150406_d_n11;
        locals.var_ibs_sws_dn14 = assign98040_e150406_d_n14;

        let (assign98050_e150411, assign98050_e150411_d_n0, assign98050_e150411_d_n2, assign98050_e150411_d_n4, assign98050_e150411_d_n5, assign98050_e150411_d_n6, assign98050_e150411_d_n7, assign98050_e150411_d_n8, assign98050_e150411_d_n9, assign98050_e150411_d_n10, assign98050_e150411_d_n11, assign98050_e150411_d_n14,) = {
    if (locals.var_guard2269 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_sws, locals.var_ibs_sws_dn0, locals.var_ibs_sws_dn2, locals.var_ibs_sws_dn4, locals.var_ibs_sws_dn5, locals.var_ibs_sws_dn6, locals.var_ibs_sws_dn7, locals.var_ibs_sws_dn8, locals.var_ibs_sws_dn9, locals.var_ibs_sws_dn10, locals.var_ibs_sws_dn11, locals.var_ibs_sws_dn14,)
    }
};
        locals.var_ibs_sws = assign98050_e150411;
        locals.var_ibs_sws_dn0 = assign98050_e150411_d_n0;
        locals.var_ibs_sws_dn2 = assign98050_e150411_d_n2;
        locals.var_ibs_sws_dn4 = assign98050_e150411_d_n4;
        locals.var_ibs_sws_dn5 = assign98050_e150411_d_n5;
        locals.var_ibs_sws_dn6 = assign98050_e150411_d_n6;
        locals.var_ibs_sws_dn7 = assign98050_e150411_d_n7;
        locals.var_ibs_sws_dn8 = assign98050_e150411_d_n8;
        locals.var_ibs_sws_dn9 = assign98050_e150411_d_n9;
        locals.var_ibs_sws_dn10 = assign98050_e150411_d_n10;
        locals.var_ibs_sws_dn11 = assign98050_e150411_d_n11;
        locals.var_ibs_sws_dn14 = assign98050_e150411_d_n14;

        let assign98060_e150414: f64 = (p.p537 * locals.var_isbs2_sws);
        locals.var_t12 = assign98060_e150414;
        locals.var_t12_dn0 = (p.p537 * locals.var_isbs2_sws_dn0);
        locals.var_t12_dn2 = (p.p537 * locals.var_isbs2_sws_dn2);
        locals.var_t12_dn4 = (p.p537 * locals.var_isbs2_sws_dn4);
        locals.var_t12_dn5 = (p.p537 * locals.var_isbs2_sws_dn5);
        locals.var_t12_dn6 = (p.p537 * locals.var_isbs2_sws_dn6);
        locals.var_t12_dn7 = (p.p537 * locals.var_isbs2_sws_dn7);
        locals.var_t12_dn8 = (p.p537 * locals.var_isbs2_sws_dn8);
        locals.var_t12_dn9 = (p.p537 * locals.var_isbs2_sws_dn9);
        locals.var_t12_dn10 = (p.p537 * locals.var_isbs2_sws_dn10);
        locals.var_t12_dn11 = (p.p537 * locals.var_isbs2_sws_dn11);
        locals.var_t12_dn14 = (p.p537 * locals.var_isbs2_sws_dn14);

        let assign98070_e150418: f64 = (locals.var_t12 * locals.var_vbs_jct);
        let assign98070_e150419: f64 = (locals.var_ibs_sws + assign98070_e150418);
        locals.var_ibs_sws = assign98070_e150419;
        locals.var_ibs_sws_dn0 = (locals.var_ibs_sws_dn0 + (locals.var_t12_dn0 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn2 = (locals.var_ibs_sws_dn2 + ((locals.var_t12_dn2 * locals.var_vbs_jct) + (locals.var_t12 * locals.var_vbs_jct_dn2)));
        locals.var_ibs_sws_dn4 = (locals.var_ibs_sws_dn4 + (locals.var_t12_dn4 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn5 = (locals.var_ibs_sws_dn5 + (locals.var_t12_dn5 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn6 = (locals.var_ibs_sws_dn6 + (locals.var_t12_dn6 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn7 = (locals.var_ibs_sws_dn7 + (locals.var_t12_dn7 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn8 = (locals.var_ibs_sws_dn8 + (locals.var_t12_dn8 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn9 = (locals.var_ibs_sws_dn9 + (locals.var_t12_dn9 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn10 = (locals.var_ibs_sws_dn10 + (locals.var_t12_dn10 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn11 = (locals.var_ibs_sws_dn11 + ((locals.var_t12_dn11 * locals.var_vbs_jct) + (locals.var_t12 * locals.var_vbs_jct_dn11)));
        locals.var_ibs_sws_dn14 = (locals.var_ibs_sws_dn14 + (locals.var_t12_dn14 * locals.var_vbs_jct));

        let assign98080_e150422: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2272 = assign98080_e150422;

        let assign98090_e150425: f64 = if locals.var_isbs_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2273 = assign98090_e150425;

        let (assign98100_e150433, assign98100_e150433_d_n0, assign98100_e150433_d_n2, assign98100_e150433_d_n4, assign98100_e150433_d_n5, assign98100_e150433_d_n6, assign98100_e150433_d_n7, assign98100_e150433_d_n8, assign98100_e150433_d_n9, assign98100_e150433_d_n10, assign98100_e150433_d_n11, assign98100_e150433_d_n14,) = {
    if ((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) {
        let assign98100_e150431: f64 = (locals.var_isbs2_swg * locals.var_t9);
        (assign98100_e150431, ((locals.var_isbs2_swg_dn0 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn0)), ((locals.var_isbs2_swg_dn2 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn2)), ((locals.var_isbs2_swg_dn4 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn4)), ((locals.var_isbs2_swg_dn5 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn5)), ((locals.var_isbs2_swg_dn6 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn6)), ((locals.var_isbs2_swg_dn7 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn7)), ((locals.var_isbs2_swg_dn8 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn8)), ((locals.var_isbs2_swg_dn9 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn9)), ((locals.var_isbs2_swg_dn10 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn10)), ((locals.var_isbs2_swg_dn11 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn11)), ((locals.var_isbs2_swg_dn14 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign98100_e150433;
        locals.var_t0_dn0 = assign98100_e150433_d_n0;
        locals.var_t0_dn2 = assign98100_e150433_d_n2;
        locals.var_t0_dn4 = assign98100_e150433_d_n4;
        locals.var_t0_dn5 = assign98100_e150433_d_n5;
        locals.var_t0_dn6 = assign98100_e150433_d_n6;
        locals.var_t0_dn7 = assign98100_e150433_d_n7;
        locals.var_t0_dn8 = assign98100_e150433_d_n8;
        locals.var_t0_dn9 = assign98100_e150433_d_n9;
        locals.var_t0_dn10 = assign98100_e150433_d_n10;
        locals.var_t0_dn11 = assign98100_e150433_d_n11;
        locals.var_t0_dn14 = assign98100_e150433_d_n14;

        let (assign98110_e150442, assign98110_e150442_d_n0, assign98110_e150442_d_n2, assign98110_e150442_d_n4, assign98110_e150442_d_n5, assign98110_e150442_d_n6, assign98110_e150442_d_n7, assign98110_e150442_d_n8, assign98110_e150442_d_n9, assign98110_e150442_d_n10, assign98110_e150442_d_n11, assign98110_e150442_d_n14,) = {
    if ((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) {
        let assign98110_e150438: f64 = (-locals.var_vbsi_jct);
        let assign98110_e150440: f64 = (assign98110_e150438 * locals.var_t10);
        (assign98110_e150440, (assign98110_e150438 * locals.var_t10_dn0), (assign98110_e150438 * locals.var_t10_dn2), (assign98110_e150438 * locals.var_t10_dn4), (assign98110_e150438 * locals.var_t10_dn5), (assign98110_e150438 * locals.var_t10_dn6), (assign98110_e150438 * locals.var_t10_dn7), (((-locals.var_vbsi_jct_dn8) * locals.var_t10) + (assign98110_e150438 * locals.var_t10_dn8)), (((-locals.var_vbsi_jct_dn9) * locals.var_t10) + (assign98110_e150438 * locals.var_t10_dn9)), (assign98110_e150438 * locals.var_t10_dn10), (assign98110_e150438 * locals.var_t10_dn11), (assign98110_e150438 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign98110_e150442;
        locals.var_tx_dn0 = assign98110_e150442_d_n0;
        locals.var_tx_dn2 = assign98110_e150442_d_n2;
        locals.var_tx_dn4 = assign98110_e150442_d_n4;
        locals.var_tx_dn5 = assign98110_e150442_d_n5;
        locals.var_tx_dn6 = assign98110_e150442_d_n6;
        locals.var_tx_dn7 = assign98110_e150442_d_n7;
        locals.var_tx_dn8 = assign98110_e150442_d_n8;
        locals.var_tx_dn9 = assign98110_e150442_d_n9;
        locals.var_tx_dn10 = assign98110_e150442_d_n10;
        locals.var_tx_dn11 = assign98110_e150442_d_n11;
        locals.var_tx_dn14 = assign98110_e150442_d_n14;

        let (assign98120_e150449, assign98120_e150449_d_n0, assign98120_e150449_d_n2, assign98120_e150449_d_n4, assign98120_e150449_d_n5, assign98120_e150449_d_n6, assign98120_e150449_d_n7, assign98120_e150449_d_n8, assign98120_e150449_d_n9, assign98120_e150449_d_n10, assign98120_e150449_d_n11, assign98120_e150449_d_n14,) = {
    if ((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) {
        let assign98120_e150447: f64 = (locals.var_tx).exp();
        (assign98120_e150447, (assign98120_e150447 * locals.var_tx_dn0), (assign98120_e150447 * locals.var_tx_dn2), (assign98120_e150447 * locals.var_tx_dn4), (assign98120_e150447 * locals.var_tx_dn5), (assign98120_e150447 * locals.var_tx_dn6), (assign98120_e150447 * locals.var_tx_dn7), (assign98120_e150447 * locals.var_tx_dn8), (assign98120_e150447 * locals.var_tx_dn9), (assign98120_e150447 * locals.var_tx_dn10), (assign98120_e150447 * locals.var_tx_dn11), (assign98120_e150447 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98120_e150449;
        locals.var_t2_dn0 = assign98120_e150449_d_n0;
        locals.var_t2_dn2 = assign98120_e150449_d_n2;
        locals.var_t2_dn4 = assign98120_e150449_d_n4;
        locals.var_t2_dn5 = assign98120_e150449_d_n5;
        locals.var_t2_dn6 = assign98120_e150449_d_n6;
        locals.var_t2_dn7 = assign98120_e150449_d_n7;
        locals.var_t2_dn8 = assign98120_e150449_d_n8;
        locals.var_t2_dn9 = assign98120_e150449_d_n9;
        locals.var_t2_dn10 = assign98120_e150449_d_n10;
        locals.var_t2_dn11 = assign98120_e150449_d_n11;
        locals.var_t2_dn14 = assign98120_e150449_d_n14;

        let (assign98130_e150455, assign98130_e150455_d_n0, assign98130_e150455_d_n2, assign98130_e150455_d_n4, assign98130_e150455_d_n5, assign98130_e150455_d_n6, assign98130_e150455_d_n7, assign98130_e150455_d_n8, assign98130_e150455_d_n9, assign98130_e150455_d_n10, assign98130_e150455_d_n11, assign98130_e150455_d_n14,) = {
    if ((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign98130_e150455;
        locals.var_t3_dn0 = assign98130_e150455_d_n0;
        locals.var_t3_dn2 = assign98130_e150455_d_n2;
        locals.var_t3_dn4 = assign98130_e150455_d_n4;
        locals.var_t3_dn5 = assign98130_e150455_d_n5;
        locals.var_t3_dn6 = assign98130_e150455_d_n6;
        locals.var_t3_dn7 = assign98130_e150455_d_n7;
        locals.var_t3_dn8 = assign98130_e150455_d_n8;
        locals.var_t3_dn9 = assign98130_e150455_d_n9;
        locals.var_t3_dn10 = assign98130_e150455_d_n10;
        locals.var_t3_dn11 = assign98130_e150455_d_n11;
        locals.var_t3_dn14 = assign98130_e150455_d_n14;

        let assign98140_e150458: f64 = if locals.var_vbsi_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2274 = assign98140_e150458;

        let (assign98150_e150468, assign98150_e150468_d_n0, assign98150_e150468_d_n2, assign98150_e150468_d_n4, assign98150_e150468_d_n5, assign98150_e150468_d_n6, assign98150_e150468_d_n7, assign98150_e150468_d_n8, assign98150_e150468_d_n9, assign98150_e150468_d_n10, assign98150_e150468_d_n11, assign98150_e150468_d_n14,) = {
    if (((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) && (locals.var_guard2274 != 0.0)) {
        let assign98150_e150466: f64 = (locals.var_vbsi_jct * locals.var_jd_nvtm_invs);
        (assign98150_e150466, (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn0), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn2), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn7), ((locals.var_vbsi_jct_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn8)), ((locals.var_vbsi_jct_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn9)), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn10), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn11), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign98150_e150468;
        locals.var_tx_dn0 = assign98150_e150468_d_n0;
        locals.var_tx_dn2 = assign98150_e150468_d_n2;
        locals.var_tx_dn4 = assign98150_e150468_d_n4;
        locals.var_tx_dn5 = assign98150_e150468_d_n5;
        locals.var_tx_dn6 = assign98150_e150468_d_n6;
        locals.var_tx_dn7 = assign98150_e150468_d_n7;
        locals.var_tx_dn8 = assign98150_e150468_d_n8;
        locals.var_tx_dn9 = assign98150_e150468_d_n9;
        locals.var_tx_dn10 = assign98150_e150468_d_n10;
        locals.var_tx_dn11 = assign98150_e150468_d_n11;
        locals.var_tx_dn14 = assign98150_e150468_d_n14;

        let assign98160_e150471: f64 = (-3.0);
        let assign98160_e150473: f64 = (assign98160_e150471 * 34.0);
        let assign98160_e150474: f64 = if locals.var_tx < assign98160_e150473 { 1.0 } else { 0.0 };
        locals.var_guard2275 = assign98160_e150474;

        let (assign98170_e150484, assign98170_e150484_d_n0, assign98170_e150484_d_n2, assign98170_e150484_d_n4, assign98170_e150484_d_n5, assign98170_e150484_d_n6, assign98170_e150484_d_n7, assign98170_e150484_d_n8, assign98170_e150484_d_n9, assign98170_e150484_d_n10, assign98170_e150484_d_n11, assign98170_e150484_d_n14,) = {
    if ((((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) && (locals.var_guard2274 != 0.0)) && (locals.var_guard2275 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98170_e150484;
        locals.var_t1_dn0 = assign98170_e150484_d_n0;
        locals.var_t1_dn2 = assign98170_e150484_d_n2;
        locals.var_t1_dn4 = assign98170_e150484_d_n4;
        locals.var_t1_dn5 = assign98170_e150484_d_n5;
        locals.var_t1_dn6 = assign98170_e150484_d_n6;
        locals.var_t1_dn7 = assign98170_e150484_d_n7;
        locals.var_t1_dn8 = assign98170_e150484_d_n8;
        locals.var_t1_dn9 = assign98170_e150484_d_n9;
        locals.var_t1_dn10 = assign98170_e150484_d_n10;
        locals.var_t1_dn11 = assign98170_e150484_d_n11;
        locals.var_t1_dn14 = assign98170_e150484_d_n14;

        let (assign98180_e150496, assign98180_e150496_d_n0, assign98180_e150496_d_n2, assign98180_e150496_d_n4, assign98180_e150496_d_n5, assign98180_e150496_d_n6, assign98180_e150496_d_n7, assign98180_e150496_d_n8, assign98180_e150496_d_n9, assign98180_e150496_d_n10, assign98180_e150496_d_n11, assign98180_e150496_d_n14,) = {
    if ((((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) && (locals.var_guard2274 != 0.0)) && (locals.var_guard2275 == 0.0)) {
        let assign98180_e150494: f64 = (locals.var_tx).exp();
        (assign98180_e150494, (assign98180_e150494 * locals.var_tx_dn0), (assign98180_e150494 * locals.var_tx_dn2), (assign98180_e150494 * locals.var_tx_dn4), (assign98180_e150494 * locals.var_tx_dn5), (assign98180_e150494 * locals.var_tx_dn6), (assign98180_e150494 * locals.var_tx_dn7), (assign98180_e150494 * locals.var_tx_dn8), (assign98180_e150494 * locals.var_tx_dn9), (assign98180_e150494 * locals.var_tx_dn10), (assign98180_e150494 * locals.var_tx_dn11), (assign98180_e150494 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98180_e150496;
        locals.var_t1_dn0 = assign98180_e150496_d_n0;
        locals.var_t1_dn2 = assign98180_e150496_d_n2;
        locals.var_t1_dn4 = assign98180_e150496_d_n4;
        locals.var_t1_dn5 = assign98180_e150496_d_n5;
        locals.var_t1_dn6 = assign98180_e150496_d_n6;
        locals.var_t1_dn7 = assign98180_e150496_d_n7;
        locals.var_t1_dn8 = assign98180_e150496_d_n8;
        locals.var_t1_dn9 = assign98180_e150496_d_n9;
        locals.var_t1_dn10 = assign98180_e150496_d_n10;
        locals.var_t1_dn11 = assign98180_e150496_d_n11;
        locals.var_t1_dn14 = assign98180_e150496_d_n14;

        let (assign98190_e150520, assign98190_e150520_d_n0, assign98190_e150520_d_n2, assign98190_e150520_d_n4, assign98190_e150520_d_n5, assign98190_e150520_d_n6, assign98190_e150520_d_n7, assign98190_e150520_d_n8, assign98190_e150520_d_n9, assign98190_e150520_d_n10, assign98190_e150520_d_n11, assign98190_e150520_d_n14,) = {
    if (((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) && (locals.var_guard2274 != 0.0)) {
        let assign98190_e150505: f64 = (locals.var_t1 - 1.0);
        let assign98190_e150506: f64 = (locals.var_isbs_swg * assign98190_e150505);
        let assign98190_e150510: f64 = (locals.var_t2 - 1.0);
        let assign98190_e150511: f64 = (locals.var_t0 * assign98190_e150510);
        let assign98190_e150512: f64 = (assign98190_e150506 + assign98190_e150511);
        let assign98190_e150516: f64 = (locals.var_t3 - 1.0);
        let assign98190_e150517: f64 = (locals.var_uc_cisbks * assign98190_e150516);
        let assign98190_e150518: f64 = (assign98190_e150512 + assign98190_e150517);
        (assign98190_e150518, ((((locals.var_isbs_swg_dn0 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), ((((locals.var_isbs_swg_dn2 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), ((((locals.var_isbs_swg_dn4 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), ((((locals.var_isbs_swg_dn5 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), ((((locals.var_isbs_swg_dn6 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), ((((locals.var_isbs_swg_dn7 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), ((((locals.var_isbs_swg_dn8 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), ((((locals.var_isbs_swg_dn9 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), ((((locals.var_isbs_swg_dn10 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), ((((locals.var_isbs_swg_dn11 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn11)) + ((locals.var_t0_dn11 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), ((((locals.var_isbs_swg_dn14 * assign98190_e150505) + (locals.var_isbs_swg * locals.var_t1_dn14)) + ((locals.var_t0_dn14 * assign98190_e150510) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn11, locals.var_ibs_swg_dn14,)
    }
};
        locals.var_ibs_swg = assign98190_e150520;
        locals.var_ibs_swg_dn0 = assign98190_e150520_d_n0;
        locals.var_ibs_swg_dn2 = assign98190_e150520_d_n2;
        locals.var_ibs_swg_dn4 = assign98190_e150520_d_n4;
        locals.var_ibs_swg_dn5 = assign98190_e150520_d_n5;
        locals.var_ibs_swg_dn6 = assign98190_e150520_d_n6;
        locals.var_ibs_swg_dn7 = assign98190_e150520_d_n7;
        locals.var_ibs_swg_dn8 = assign98190_e150520_d_n8;
        locals.var_ibs_swg_dn9 = assign98190_e150520_d_n9;
        locals.var_ibs_swg_dn10 = assign98190_e150520_d_n10;
        locals.var_ibs_swg_dn11 = assign98190_e150520_d_n11;
        locals.var_ibs_swg_dn14 = assign98190_e150520_d_n14;

        let (assign98200_e150529, assign98200_e150529_d_n0, assign98200_e150529_d_n2, assign98200_e150529_d_n4, assign98200_e150529_d_n5, assign98200_e150529_d_n6, assign98200_e150529_d_n7, assign98200_e150529_d_n8, assign98200_e150529_d_n9, assign98200_e150529_d_n10, assign98200_e150529_d_n11, assign98200_e150529_d_n14,) = {
    if (((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) && (locals.var_guard2274 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98200_e150529;
        locals.var_t1_dn0 = assign98200_e150529_d_n0;
        locals.var_t1_dn2 = assign98200_e150529_d_n2;
        locals.var_t1_dn4 = assign98200_e150529_d_n4;
        locals.var_t1_dn5 = assign98200_e150529_d_n5;
        locals.var_t1_dn6 = assign98200_e150529_d_n6;
        locals.var_t1_dn7 = assign98200_e150529_d_n7;
        locals.var_t1_dn8 = assign98200_e150529_d_n8;
        locals.var_t1_dn9 = assign98200_e150529_d_n9;
        locals.var_t1_dn10 = assign98200_e150529_d_n10;
        locals.var_t1_dn11 = assign98200_e150529_d_n11;
        locals.var_t1_dn14 = assign98200_e150529_d_n14;

        let (assign98210_e150542, assign98210_e150542_d_n0, assign98210_e150542_d_n2, assign98210_e150542_d_n4, assign98210_e150542_d_n5, assign98210_e150542_d_n6, assign98210_e150542_d_n7, assign98210_e150542_d_n8, assign98210_e150542_d_n9, assign98210_e150542_d_n10, assign98210_e150542_d_n11, assign98210_e150542_d_n14,) = {
    if (((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) && (locals.var_guard2274 == 0.0)) {
        let assign98210_e150538: f64 = (locals.var_isbs_swg * locals.var_jd_nvtm_invs);
        let assign98210_e150540: f64 = (assign98210_e150538 * locals.var_t1);
        (assign98210_e150540, ((((locals.var_isbs_swg_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn0)), ((((locals.var_isbs_swg_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn2)), ((((locals.var_isbs_swg_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn4)), ((((locals.var_isbs_swg_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn5)), ((((locals.var_isbs_swg_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn6)), ((((locals.var_isbs_swg_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn7)), ((((locals.var_isbs_swg_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn8)), ((((locals.var_isbs_swg_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn9)), ((((locals.var_isbs_swg_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn10)), ((((locals.var_isbs_swg_dn11 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn11)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn11)), ((((locals.var_isbs_swg_dn14 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn14)) * locals.var_t1) + (assign98210_e150538 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign98210_e150542;
        locals.var_t4_dn0 = assign98210_e150542_d_n0;
        locals.var_t4_dn2 = assign98210_e150542_d_n2;
        locals.var_t4_dn4 = assign98210_e150542_d_n4;
        locals.var_t4_dn5 = assign98210_e150542_d_n5;
        locals.var_t4_dn6 = assign98210_e150542_d_n6;
        locals.var_t4_dn7 = assign98210_e150542_d_n7;
        locals.var_t4_dn8 = assign98210_e150542_d_n8;
        locals.var_t4_dn9 = assign98210_e150542_d_n9;
        locals.var_t4_dn10 = assign98210_e150542_d_n10;
        locals.var_t4_dn11 = assign98210_e150542_d_n11;
        locals.var_t4_dn14 = assign98210_e150542_d_n14;

        let (assign98220_e150573, assign98220_e150573_d_n0, assign98220_e150573_d_n2, assign98220_e150573_d_n4, assign98220_e150573_d_n5, assign98220_e150573_d_n6, assign98220_e150573_d_n7, assign98220_e150573_d_n8, assign98220_e150573_d_n9, assign98220_e150573_d_n10, assign98220_e150573_d_n11, assign98220_e150573_d_n14,) = {
    if (((locals.var_guard2272 != 0.0) && (locals.var_guard2273 != 0.0)) && (locals.var_guard2274 == 0.0)) {
        let assign98220_e150552: f64 = (locals.var_t1 - 1.0);
        let assign98220_e150553: f64 = (locals.var_isbs_swg * assign98220_e150552);
        let assign98220_e150557: f64 = (locals.var_vbsi_jct - locals.var_vbst);
        let assign98220_e150558: f64 = (locals.var_t4 * assign98220_e150557);
        let assign98220_e150559: f64 = (assign98220_e150553 + assign98220_e150558);
        let assign98220_e150563: f64 = (locals.var_t2 - 1.0);
        let assign98220_e150564: f64 = (locals.var_t0 * assign98220_e150563);
        let assign98220_e150565: f64 = (assign98220_e150559 + assign98220_e150564);
        let assign98220_e150569: f64 = (locals.var_t3 - 1.0);
        let assign98220_e150570: f64 = (locals.var_uc_cisbks * assign98220_e150569);
        let assign98220_e150571: f64 = (assign98220_e150565 + assign98220_e150570);
        (assign98220_e150571, (((((locals.var_isbs_swg_dn0 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign98220_e150557) + (locals.var_t4 * (-locals.var_vbst_dn0)))) + ((locals.var_t0_dn0 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), (((((locals.var_isbs_swg_dn2 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign98220_e150557) + (locals.var_t4 * (-locals.var_vbst_dn2)))) + ((locals.var_t0_dn2 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), (((((locals.var_isbs_swg_dn4 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign98220_e150557) + (locals.var_t4 * (-locals.var_vbst_dn4)))) + ((locals.var_t0_dn4 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), (((((locals.var_isbs_swg_dn5 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign98220_e150557) + (locals.var_t4 * (-locals.var_vbst_dn5)))) + ((locals.var_t0_dn5 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), (((((locals.var_isbs_swg_dn6 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign98220_e150557) + (locals.var_t4 * (-locals.var_vbst_dn6)))) + ((locals.var_t0_dn6 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), (((((locals.var_isbs_swg_dn7 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign98220_e150557) + (locals.var_t4 * (-locals.var_vbst_dn7)))) + ((locals.var_t0_dn7 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), (((((locals.var_isbs_swg_dn8 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign98220_e150557) + (locals.var_t4 * (locals.var_vbsi_jct_dn8 - locals.var_vbst_dn8)))) + ((locals.var_t0_dn8 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), (((((locals.var_isbs_swg_dn9 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign98220_e150557) + (locals.var_t4 * (locals.var_vbsi_jct_dn9 - locals.var_vbst_dn9)))) + ((locals.var_t0_dn9 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), (((((locals.var_isbs_swg_dn10 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign98220_e150557) + (locals.var_t4 * (-locals.var_vbst_dn10)))) + ((locals.var_t0_dn10 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), (((((locals.var_isbs_swg_dn11 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * assign98220_e150557) + (locals.var_t4 * (-locals.var_vbst_dn11)))) + ((locals.var_t0_dn11 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), (((((locals.var_isbs_swg_dn14 * assign98220_e150552) + (locals.var_isbs_swg * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * assign98220_e150557) + (locals.var_t4 * (-locals.var_vbst_dn14)))) + ((locals.var_t0_dn14 * assign98220_e150563) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn11, locals.var_ibs_swg_dn14,)
    }
};
        locals.var_ibs_swg = assign98220_e150573;
        locals.var_ibs_swg_dn0 = assign98220_e150573_d_n0;
        locals.var_ibs_swg_dn2 = assign98220_e150573_d_n2;
        locals.var_ibs_swg_dn4 = assign98220_e150573_d_n4;
        locals.var_ibs_swg_dn5 = assign98220_e150573_d_n5;
        locals.var_ibs_swg_dn6 = assign98220_e150573_d_n6;
        locals.var_ibs_swg_dn7 = assign98220_e150573_d_n7;
        locals.var_ibs_swg_dn8 = assign98220_e150573_d_n8;
        locals.var_ibs_swg_dn9 = assign98220_e150573_d_n9;
        locals.var_ibs_swg_dn10 = assign98220_e150573_d_n10;
        locals.var_ibs_swg_dn11 = assign98220_e150573_d_n11;
        locals.var_ibs_swg_dn14 = assign98220_e150573_d_n14;

        let (assign98230_e150580, assign98230_e150580_d_n0, assign98230_e150580_d_n2, assign98230_e150580_d_n4, assign98230_e150580_d_n5, assign98230_e150580_d_n6, assign98230_e150580_d_n7, assign98230_e150580_d_n8, assign98230_e150580_d_n9, assign98230_e150580_d_n10, assign98230_e150580_d_n11, assign98230_e150580_d_n14,) = {
    if ((locals.var_guard2272 != 0.0) && (locals.var_guard2273 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn11, locals.var_ibs_swg_dn14,)
    }
};
        locals.var_ibs_swg = assign98230_e150580;
        locals.var_ibs_swg_dn0 = assign98230_e150580_d_n0;
        locals.var_ibs_swg_dn2 = assign98230_e150580_d_n2;
        locals.var_ibs_swg_dn4 = assign98230_e150580_d_n4;
        locals.var_ibs_swg_dn5 = assign98230_e150580_d_n5;
        locals.var_ibs_swg_dn6 = assign98230_e150580_d_n6;
        locals.var_ibs_swg_dn7 = assign98230_e150580_d_n7;
        locals.var_ibs_swg_dn8 = assign98230_e150580_d_n8;
        locals.var_ibs_swg_dn9 = assign98230_e150580_d_n9;
        locals.var_ibs_swg_dn10 = assign98230_e150580_d_n10;
        locals.var_ibs_swg_dn11 = assign98230_e150580_d_n11;
        locals.var_ibs_swg_dn14 = assign98230_e150580_d_n14;

        let (assign98240_e150586, assign98240_e150586_d_n0, assign98240_e150586_d_n2, assign98240_e150586_d_n4, assign98240_e150586_d_n5, assign98240_e150586_d_n6, assign98240_e150586_d_n7, assign98240_e150586_d_n8, assign98240_e150586_d_n9, assign98240_e150586_d_n10, assign98240_e150586_d_n11, assign98240_e150586_d_n14,) = {
    if (locals.var_guard2272 != 0.0) {
        let assign98240_e150584: f64 = (p.p537 * locals.var_isbs2_swg);
        (assign98240_e150584, (p.p537 * locals.var_isbs2_swg_dn0), (p.p537 * locals.var_isbs2_swg_dn2), (p.p537 * locals.var_isbs2_swg_dn4), (p.p537 * locals.var_isbs2_swg_dn5), (p.p537 * locals.var_isbs2_swg_dn6), (p.p537 * locals.var_isbs2_swg_dn7), (p.p537 * locals.var_isbs2_swg_dn8), (p.p537 * locals.var_isbs2_swg_dn9), (p.p537 * locals.var_isbs2_swg_dn10), (p.p537 * locals.var_isbs2_swg_dn11), (p.p537 * locals.var_isbs2_swg_dn14),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign98240_e150586;
        locals.var_t12_dn0 = assign98240_e150586_d_n0;
        locals.var_t12_dn2 = assign98240_e150586_d_n2;
        locals.var_t12_dn4 = assign98240_e150586_d_n4;
        locals.var_t12_dn5 = assign98240_e150586_d_n5;
        locals.var_t12_dn6 = assign98240_e150586_d_n6;
        locals.var_t12_dn7 = assign98240_e150586_d_n7;
        locals.var_t12_dn8 = assign98240_e150586_d_n8;
        locals.var_t12_dn9 = assign98240_e150586_d_n9;
        locals.var_t12_dn10 = assign98240_e150586_d_n10;
        locals.var_t12_dn11 = assign98240_e150586_d_n11;
        locals.var_t12_dn14 = assign98240_e150586_d_n14;

    }

    pub(super) fn stamp_transient_block_362(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98250_e150594, assign98250_e150594_d_n0, assign98250_e150594_d_n2, assign98250_e150594_d_n4, assign98250_e150594_d_n5, assign98250_e150594_d_n6, assign98250_e150594_d_n7, assign98250_e150594_d_n8, assign98250_e150594_d_n9, assign98250_e150594_d_n10, assign98250_e150594_d_n11, assign98250_e150594_d_n14,) = {
    if (locals.var_guard2272 != 0.0) {
        let assign98250_e150591: f64 = (locals.var_t12 * locals.var_vbsi_jct);
        let assign98250_e150592: f64 = (locals.var_ibs_swg + assign98250_e150591);
        (assign98250_e150592, (locals.var_ibs_swg_dn0 + (locals.var_t12_dn0 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn2 + (locals.var_t12_dn2 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn4 + (locals.var_t12_dn4 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn5 + (locals.var_t12_dn5 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn6 + (locals.var_t12_dn6 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn7 + (locals.var_t12_dn7 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn8 + ((locals.var_t12_dn8 * locals.var_vbsi_jct) + (locals.var_t12 * locals.var_vbsi_jct_dn8))), (locals.var_ibs_swg_dn9 + ((locals.var_t12_dn9 * locals.var_vbsi_jct) + (locals.var_t12 * locals.var_vbsi_jct_dn9))), (locals.var_ibs_swg_dn10 + (locals.var_t12_dn10 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn11 + (locals.var_t12_dn11 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn14 + (locals.var_t12_dn14 * locals.var_vbsi_jct)),)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn11, locals.var_ibs_swg_dn14,)
    }
};
        locals.var_ibs_swg = assign98250_e150594;
        locals.var_ibs_swg_dn0 = assign98250_e150594_d_n0;
        locals.var_ibs_swg_dn2 = assign98250_e150594_d_n2;
        locals.var_ibs_swg_dn4 = assign98250_e150594_d_n4;
        locals.var_ibs_swg_dn5 = assign98250_e150594_d_n5;
        locals.var_ibs_swg_dn6 = assign98250_e150594_d_n6;
        locals.var_ibs_swg_dn7 = assign98250_e150594_d_n7;
        locals.var_ibs_swg_dn8 = assign98250_e150594_d_n8;
        locals.var_ibs_swg_dn9 = assign98250_e150594_d_n9;
        locals.var_ibs_swg_dn10 = assign98250_e150594_d_n10;
        locals.var_ibs_swg_dn11 = assign98250_e150594_d_n11;
        locals.var_ibs_swg_dn14 = assign98250_e150594_d_n14;

        let (assign98260_e150599, assign98260_e150599_d_n0, assign98260_e150599_d_n2, assign98260_e150599_d_n4, assign98260_e150599_d_n5, assign98260_e150599_d_n6, assign98260_e150599_d_n7, assign98260_e150599_d_n8, assign98260_e150599_d_n9, assign98260_e150599_d_n10, assign98260_e150599_d_n11, assign98260_e150599_d_n14,) = {
    if (locals.var_guard2272 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn11, locals.var_ibs_swg_dn14,)
    }
};
        locals.var_ibs_swg = assign98260_e150599;
        locals.var_ibs_swg_dn0 = assign98260_e150599_d_n0;
        locals.var_ibs_swg_dn2 = assign98260_e150599_d_n2;
        locals.var_ibs_swg_dn4 = assign98260_e150599_d_n4;
        locals.var_ibs_swg_dn5 = assign98260_e150599_d_n5;
        locals.var_ibs_swg_dn6 = assign98260_e150599_d_n6;
        locals.var_ibs_swg_dn7 = assign98260_e150599_d_n7;
        locals.var_ibs_swg_dn8 = assign98260_e150599_d_n8;
        locals.var_ibs_swg_dn9 = assign98260_e150599_d_n9;
        locals.var_ibs_swg_dn10 = assign98260_e150599_d_n10;
        locals.var_ibs_swg_dn11 = assign98260_e150599_d_n11;
        locals.var_ibs_swg_dn14 = assign98260_e150599_d_n14;

        let assign98270_e150602: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2276 = assign98270_e150602;

        let assign98280_e150605: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2277 = assign98280_e150605;

        let (assign98290_e150615, assign98290_e150615_d_n0, assign98290_e150615_d_n2, assign98290_e150615_d_n4, assign98290_e150615_d_n5, assign98290_e150615_d_n6, assign98290_e150615_d_n7, assign98290_e150615_d_n8, assign98290_e150615_d_n9, assign98290_e150615_d_n10, assign98290_e150615_d_n11, assign98290_e150615_d_n14,) = {
    if ((locals.var_guard2276 != 0.0) && (locals.var_guard2277 != 0.0)) {
        let assign98290_e150612: f64 = (locals.var_vbd_jct / locals.var_pzbd);
        let assign98290_e150613: f64 = (1.0 - assign98290_e150612);
        (assign98290_e150613, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbd) - (locals.var_vbd_jct * locals.var_pzbd_dn0)) / (locals.var_pzbd * locals.var_pzbd))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn2) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn4) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn5) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn6) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn7) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn8) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn9) / (locals.var_pzbd * locals.var_pzbd)))), (-(((locals.var_vbd_jct_dn10 * locals.var_pzbd) - (locals.var_vbd_jct * locals.var_pzbd_dn10)) / (locals.var_pzbd * locals.var_pzbd))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn11) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn14) / (locals.var_pzbd * locals.var_pzbd)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign98290_e150615;
        locals.var_arg_dn0 = assign98290_e150615_d_n0;
        locals.var_arg_dn2 = assign98290_e150615_d_n2;
        locals.var_arg_dn4 = assign98290_e150615_d_n4;
        locals.var_arg_dn5 = assign98290_e150615_d_n5;
        locals.var_arg_dn6 = assign98290_e150615_d_n6;
        locals.var_arg_dn7 = assign98290_e150615_d_n7;
        locals.var_arg_dn8 = assign98290_e150615_d_n8;
        locals.var_arg_dn9 = assign98290_e150615_d_n9;
        locals.var_arg_dn10 = assign98290_e150615_d_n10;
        locals.var_arg_dn11 = assign98290_e150615_d_n11;
        locals.var_arg_dn14 = assign98290_e150615_d_n14;

        let assign98300_e150618: f64 = if p.p503 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2278 = assign98300_e150618;

        let (assign98310_e150629, assign98310_e150629_d_n0, assign98310_e150629_d_n2, assign98310_e150629_d_n4, assign98310_e150629_d_n5, assign98310_e150629_d_n6, assign98310_e150629_d_n7, assign98310_e150629_d_n8, assign98310_e150629_d_n9, assign98310_e150629_d_n10, assign98310_e150629_d_n11, assign98310_e150629_d_n14,) = {
    if (((locals.var_guard2276 != 0.0) && (locals.var_guard2277 != 0.0)) && (locals.var_guard2278 != 0.0)) {
        let assign98310_e150626: f64 = (locals.var_arg).sqrt();
        let assign98310_e150627: f64 = (1.0 / assign98310_e150626);
        (assign98310_e150627, (-((locals.var_arg_dn0 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))), (-((locals.var_arg_dn2 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))), (-((locals.var_arg_dn4 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))), (-((locals.var_arg_dn5 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))), (-((locals.var_arg_dn6 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))), (-((locals.var_arg_dn7 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))), (-((locals.var_arg_dn8 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))), (-((locals.var_arg_dn9 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))), (-((locals.var_arg_dn10 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))), (-((locals.var_arg_dn11 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))), (-((locals.var_arg_dn14 / (2.0 * assign98310_e150626)) / (assign98310_e150626 * assign98310_e150626))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98310_e150629;
        locals.var_sarg_dn0 = assign98310_e150629_d_n0;
        locals.var_sarg_dn2 = assign98310_e150629_d_n2;
        locals.var_sarg_dn4 = assign98310_e150629_d_n4;
        locals.var_sarg_dn5 = assign98310_e150629_d_n5;
        locals.var_sarg_dn6 = assign98310_e150629_d_n6;
        locals.var_sarg_dn7 = assign98310_e150629_d_n7;
        locals.var_sarg_dn8 = assign98310_e150629_d_n8;
        locals.var_sarg_dn9 = assign98310_e150629_d_n9;
        locals.var_sarg_dn10 = assign98310_e150629_d_n10;
        locals.var_sarg_dn11 = assign98310_e150629_d_n11;
        locals.var_sarg_dn14 = assign98310_e150629_d_n14;

        let (assign98320_e150646, assign98320_e150646_d_n0, assign98320_e150646_d_n2, assign98320_e150646_d_n4, assign98320_e150646_d_n5, assign98320_e150646_d_n6, assign98320_e150646_d_n7, assign98320_e150646_d_n8, assign98320_e150646_d_n9, assign98320_e150646_d_n10, assign98320_e150646_d_n11, assign98320_e150646_d_n14,) = {
    if (((locals.var_guard2276 != 0.0) && (locals.var_guard2277 != 0.0)) && (locals.var_guard2278 == 0.0)) {
        let (assign98320_e150644, assign98320_e150644_d_n0, assign98320_e150644_d_n2, assign98320_e150644_d_n4, assign98320_e150644_d_n5, assign98320_e150644_d_n6, assign98320_e150644_d_n7, assign98320_e150644_d_n8, assign98320_e150644_d_n9, assign98320_e150644_d_n10, assign98320_e150644_d_n11, assign98320_e150644_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98320_e150642: f64 = (-p.p503);
                let assign98320_e150643: f64 = (locals.var_arg).powf(assign98320_e150642);
                (assign98320_e150643, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn0)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn2)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn4)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn5)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn6)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn7)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn8)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn9)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn10)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn11)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98320_e150642) as f64).is_finite() && ((assign98320_e150642) as f64).fract() == 0.0 { if assign98320_e150642 == 0.0 { 0.0 } else { (assign98320_e150642 * ((locals.var_arg).powf(assign98320_e150642 - 1.0) * locals.var_arg_dn14)) } } else { (assign98320_e150643 * (assign98320_e150642 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign98320_e150644, assign98320_e150644_d_n0, assign98320_e150644_d_n2, assign98320_e150644_d_n4, assign98320_e150644_d_n5, assign98320_e150644_d_n6, assign98320_e150644_d_n7, assign98320_e150644_d_n8, assign98320_e150644_d_n9, assign98320_e150644_d_n10, assign98320_e150644_d_n11, assign98320_e150644_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98320_e150646;
        locals.var_sarg_dn0 = assign98320_e150646_d_n0;
        locals.var_sarg_dn2 = assign98320_e150646_d_n2;
        locals.var_sarg_dn4 = assign98320_e150646_d_n4;
        locals.var_sarg_dn5 = assign98320_e150646_d_n5;
        locals.var_sarg_dn6 = assign98320_e150646_d_n6;
        locals.var_sarg_dn7 = assign98320_e150646_d_n7;
        locals.var_sarg_dn8 = assign98320_e150646_d_n8;
        locals.var_sarg_dn9 = assign98320_e150646_d_n9;
        locals.var_sarg_dn10 = assign98320_e150646_d_n10;
        locals.var_sarg_dn11 = assign98320_e150646_d_n11;
        locals.var_sarg_dn14 = assign98320_e150646_d_n14;

        let (assign98330_e150664, assign98330_e150664_d_n0, assign98330_e150664_d_n2, assign98330_e150664_d_n4, assign98330_e150664_d_n5, assign98330_e150664_d_n6, assign98330_e150664_d_n7, assign98330_e150664_d_n8, assign98330_e150664_d_n9, assign98330_e150664_d_n10, assign98330_e150664_d_n11, assign98330_e150664_d_n14,) = {
    if ((locals.var_guard2276 != 0.0) && (locals.var_guard2277 != 0.0)) {
        let assign98330_e150652: f64 = (locals.var_pzbd * locals.var_czbd);
        let assign98330_e150656: f64 = (locals.var_arg * locals.var_sarg);
        let assign98330_e150657: f64 = (1.0 - assign98330_e150656);
        let assign98330_e150658: f64 = (assign98330_e150652 * assign98330_e150657);
        let assign98330_e150661: f64 = (1.0 - p.p503);
        let assign98330_e150662: f64 = (assign98330_e150658 / assign98330_e150661);
        (assign98330_e150662, (((((locals.var_pzbd_dn0 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn0)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98330_e150661), (((((locals.var_pzbd_dn2 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn2)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98330_e150661), (((((locals.var_pzbd_dn4 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn4)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98330_e150661), (((((locals.var_pzbd_dn5 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn5)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98330_e150661), (((((locals.var_pzbd_dn6 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn6)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98330_e150661), (((((locals.var_pzbd_dn7 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn7)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98330_e150661), (((((locals.var_pzbd_dn8 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn8)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98330_e150661), (((((locals.var_pzbd_dn9 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn9)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98330_e150661), (((((locals.var_pzbd_dn10 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn10)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98330_e150661), (((((locals.var_pzbd_dn11 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn11)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign98330_e150661), (((((locals.var_pzbd_dn14 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn14)) * assign98330_e150657) + (assign98330_e150652 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign98330_e150661),)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn11, locals.var_qbd_btm_dn14,)
    }
};
        locals.var_qbd_btm = assign98330_e150664;
        locals.var_qbd_btm_dn0 = assign98330_e150664_d_n0;
        locals.var_qbd_btm_dn2 = assign98330_e150664_d_n2;
        locals.var_qbd_btm_dn4 = assign98330_e150664_d_n4;
        locals.var_qbd_btm_dn5 = assign98330_e150664_d_n5;
        locals.var_qbd_btm_dn6 = assign98330_e150664_d_n6;
        locals.var_qbd_btm_dn7 = assign98330_e150664_d_n7;
        locals.var_qbd_btm_dn8 = assign98330_e150664_d_n8;
        locals.var_qbd_btm_dn9 = assign98330_e150664_d_n9;
        locals.var_qbd_btm_dn10 = assign98330_e150664_d_n10;
        locals.var_qbd_btm_dn11 = assign98330_e150664_d_n11;
        locals.var_qbd_btm_dn14 = assign98330_e150664_d_n14;

        let (assign98350_e150679, assign98350_e150679_d_n0, assign98350_e150679_d_n2, assign98350_e150679_d_n4, assign98350_e150679_d_n5, assign98350_e150679_d_n6, assign98350_e150679_d_n7, assign98350_e150679_d_n8, assign98350_e150679_d_n9, assign98350_e150679_d_n10, assign98350_e150679_d_n11, assign98350_e150679_d_n14,) = {
    if ((locals.var_guard2276 != 0.0) && (locals.var_guard2277 == 0.0)) {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98350_e150679;
        locals.var_t1_dn0 = assign98350_e150679_d_n0;
        locals.var_t1_dn2 = assign98350_e150679_d_n2;
        locals.var_t1_dn4 = assign98350_e150679_d_n4;
        locals.var_t1_dn5 = assign98350_e150679_d_n5;
        locals.var_t1_dn6 = assign98350_e150679_d_n6;
        locals.var_t1_dn7 = assign98350_e150679_d_n7;
        locals.var_t1_dn8 = assign98350_e150679_d_n8;
        locals.var_t1_dn9 = assign98350_e150679_d_n9;
        locals.var_t1_dn10 = assign98350_e150679_d_n10;
        locals.var_t1_dn11 = assign98350_e150679_d_n11;
        locals.var_t1_dn14 = assign98350_e150679_d_n14;

        let (assign98360_e150690, assign98360_e150690_d_n0, assign98360_e150690_d_n2, assign98360_e150690_d_n4, assign98360_e150690_d_n5, assign98360_e150690_d_n6, assign98360_e150690_d_n7, assign98360_e150690_d_n8, assign98360_e150690_d_n9, assign98360_e150690_d_n10, assign98360_e150690_d_n11, assign98360_e150690_d_n14,) = {
    if ((locals.var_guard2276 != 0.0) && (locals.var_guard2277 == 0.0)) {
        let assign98360_e150686: f64 = (locals.var_czbd * p.p503);
        let assign98360_e150688: f64 = (assign98360_e150686 / locals.var_pzbd);
        (assign98360_e150688, ((((locals.var_czbd_dn0 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn0)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn2 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn2)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn4 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn4)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn5 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn5)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn6 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn6)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn7 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn7)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn8 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn8)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn9 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn9)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn10 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn10)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn11 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn11)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn14 * p.p503) * locals.var_pzbd) - (assign98360_e150686 * locals.var_pzbd_dn14)) / (locals.var_pzbd * locals.var_pzbd)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98360_e150690;
        locals.var_t2_dn0 = assign98360_e150690_d_n0;
        locals.var_t2_dn2 = assign98360_e150690_d_n2;
        locals.var_t2_dn4 = assign98360_e150690_d_n4;
        locals.var_t2_dn5 = assign98360_e150690_d_n5;
        locals.var_t2_dn6 = assign98360_e150690_d_n6;
        locals.var_t2_dn7 = assign98360_e150690_d_n7;
        locals.var_t2_dn8 = assign98360_e150690_d_n8;
        locals.var_t2_dn9 = assign98360_e150690_d_n9;
        locals.var_t2_dn10 = assign98360_e150690_d_n10;
        locals.var_t2_dn11 = assign98360_e150690_d_n11;
        locals.var_t2_dn14 = assign98360_e150690_d_n14;

        let (assign98370_e150705, assign98370_e150705_d_n0, assign98370_e150705_d_n2, assign98370_e150705_d_n4, assign98370_e150705_d_n5, assign98370_e150705_d_n6, assign98370_e150705_d_n7, assign98370_e150705_d_n8, assign98370_e150705_d_n9, assign98370_e150705_d_n10, assign98370_e150705_d_n11, assign98370_e150705_d_n14,) = {
    if ((locals.var_guard2276 != 0.0) && (locals.var_guard2277 == 0.0)) {
        let assign98370_e150699: f64 = (locals.var_vbd_jct * 0.5);
        let assign98370_e150701: f64 = (assign98370_e150699 * locals.var_t2);
        let assign98370_e150702: f64 = (locals.var_t1 + assign98370_e150701);
        let assign98370_e150703: f64 = (locals.var_vbd_jct * assign98370_e150702);
        (assign98370_e150703, ((locals.var_vbd_jct_dn0 * assign98370_e150702) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98370_e150699 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98370_e150699 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98370_e150699 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98370_e150699 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98370_e150699 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98370_e150699 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98370_e150699 * locals.var_t2_dn8))), (locals.var_vbd_jct * (locals.var_t1_dn9 + (assign98370_e150699 * locals.var_t2_dn9))), ((locals.var_vbd_jct_dn10 * assign98370_e150702) + (locals.var_vbd_jct * (locals.var_t1_dn10 + (((locals.var_vbd_jct_dn10 * 0.5) * locals.var_t2) + (assign98370_e150699 * locals.var_t2_dn10))))), (locals.var_vbd_jct * (locals.var_t1_dn11 + (assign98370_e150699 * locals.var_t2_dn11))), (locals.var_vbd_jct * (locals.var_t1_dn14 + (assign98370_e150699 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn11, locals.var_qbd_btm_dn14,)
    }
};
        locals.var_qbd_btm = assign98370_e150705;
        locals.var_qbd_btm_dn0 = assign98370_e150705_d_n0;
        locals.var_qbd_btm_dn2 = assign98370_e150705_d_n2;
        locals.var_qbd_btm_dn4 = assign98370_e150705_d_n4;
        locals.var_qbd_btm_dn5 = assign98370_e150705_d_n5;
        locals.var_qbd_btm_dn6 = assign98370_e150705_d_n6;
        locals.var_qbd_btm_dn7 = assign98370_e150705_d_n7;
        locals.var_qbd_btm_dn8 = assign98370_e150705_d_n8;
        locals.var_qbd_btm_dn9 = assign98370_e150705_d_n9;
        locals.var_qbd_btm_dn10 = assign98370_e150705_d_n10;
        locals.var_qbd_btm_dn11 = assign98370_e150705_d_n11;
        locals.var_qbd_btm_dn14 = assign98370_e150705_d_n14;

        let (assign98390_e150721, assign98390_e150721_d_n0, assign98390_e150721_d_n2, assign98390_e150721_d_n4, assign98390_e150721_d_n5, assign98390_e150721_d_n6, assign98390_e150721_d_n7, assign98390_e150721_d_n8, assign98390_e150721_d_n9, assign98390_e150721_d_n10, assign98390_e150721_d_n11, assign98390_e150721_d_n14,) = {
    if (locals.var_guard2276 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn11, locals.var_qbd_btm_dn14,)
    }
};
        locals.var_qbd_btm = assign98390_e150721;
        locals.var_qbd_btm_dn0 = assign98390_e150721_d_n0;
        locals.var_qbd_btm_dn2 = assign98390_e150721_d_n2;
        locals.var_qbd_btm_dn4 = assign98390_e150721_d_n4;
        locals.var_qbd_btm_dn5 = assign98390_e150721_d_n5;
        locals.var_qbd_btm_dn6 = assign98390_e150721_d_n6;
        locals.var_qbd_btm_dn7 = assign98390_e150721_d_n7;
        locals.var_qbd_btm_dn8 = assign98390_e150721_d_n8;
        locals.var_qbd_btm_dn9 = assign98390_e150721_d_n9;
        locals.var_qbd_btm_dn10 = assign98390_e150721_d_n10;
        locals.var_qbd_btm_dn11 = assign98390_e150721_d_n11;
        locals.var_qbd_btm_dn14 = assign98390_e150721_d_n14;

        let assign98410_e150729: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2279 = assign98410_e150729;

        let assign98420_e150732: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2280 = assign98420_e150732;

        let (assign98430_e150742, assign98430_e150742_d_n0, assign98430_e150742_d_n2, assign98430_e150742_d_n4, assign98430_e150742_d_n5, assign98430_e150742_d_n6, assign98430_e150742_d_n7, assign98430_e150742_d_n8, assign98430_e150742_d_n9, assign98430_e150742_d_n10, assign98430_e150742_d_n11, assign98430_e150742_d_n14,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2280 != 0.0)) {
        let assign98430_e150739: f64 = (locals.var_vbd_jct / locals.var_pzbdsw);
        let assign98430_e150740: f64 = (1.0 - assign98430_e150739);
        (assign98430_e150740, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbdsw) - (locals.var_vbd_jct * locals.var_pzbdsw_dn0)) / (locals.var_pzbdsw * locals.var_pzbdsw))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn2) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn4) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn5) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn6) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn7) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn8) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn9) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(((locals.var_vbd_jct_dn10 * locals.var_pzbdsw) - (locals.var_vbd_jct * locals.var_pzbdsw_dn10)) / (locals.var_pzbdsw * locals.var_pzbdsw))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn11) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn14) / (locals.var_pzbdsw * locals.var_pzbdsw)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign98430_e150742;
        locals.var_arg_dn0 = assign98430_e150742_d_n0;
        locals.var_arg_dn2 = assign98430_e150742_d_n2;
        locals.var_arg_dn4 = assign98430_e150742_d_n4;
        locals.var_arg_dn5 = assign98430_e150742_d_n5;
        locals.var_arg_dn6 = assign98430_e150742_d_n6;
        locals.var_arg_dn7 = assign98430_e150742_d_n7;
        locals.var_arg_dn8 = assign98430_e150742_d_n8;
        locals.var_arg_dn9 = assign98430_e150742_d_n9;
        locals.var_arg_dn10 = assign98430_e150742_d_n10;
        locals.var_arg_dn11 = assign98430_e150742_d_n11;
        locals.var_arg_dn14 = assign98430_e150742_d_n14;

        let assign98440_e150745: f64 = if p.p504 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2281 = assign98440_e150745;

        let (assign98450_e150756, assign98450_e150756_d_n0, assign98450_e150756_d_n2, assign98450_e150756_d_n4, assign98450_e150756_d_n5, assign98450_e150756_d_n6, assign98450_e150756_d_n7, assign98450_e150756_d_n8, assign98450_e150756_d_n9, assign98450_e150756_d_n10, assign98450_e150756_d_n11, assign98450_e150756_d_n14,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2280 != 0.0)) && (locals.var_guard2281 != 0.0)) {
        let assign98450_e150753: f64 = (locals.var_arg).sqrt();
        let assign98450_e150754: f64 = (1.0 / assign98450_e150753);
        (assign98450_e150754, (-((locals.var_arg_dn0 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))), (-((locals.var_arg_dn2 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))), (-((locals.var_arg_dn4 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))), (-((locals.var_arg_dn5 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))), (-((locals.var_arg_dn6 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))), (-((locals.var_arg_dn7 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))), (-((locals.var_arg_dn8 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))), (-((locals.var_arg_dn9 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))), (-((locals.var_arg_dn10 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))), (-((locals.var_arg_dn11 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))), (-((locals.var_arg_dn14 / (2.0 * assign98450_e150753)) / (assign98450_e150753 * assign98450_e150753))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98450_e150756;
        locals.var_sarg_dn0 = assign98450_e150756_d_n0;
        locals.var_sarg_dn2 = assign98450_e150756_d_n2;
        locals.var_sarg_dn4 = assign98450_e150756_d_n4;
        locals.var_sarg_dn5 = assign98450_e150756_d_n5;
        locals.var_sarg_dn6 = assign98450_e150756_d_n6;
        locals.var_sarg_dn7 = assign98450_e150756_d_n7;
        locals.var_sarg_dn8 = assign98450_e150756_d_n8;
        locals.var_sarg_dn9 = assign98450_e150756_d_n9;
        locals.var_sarg_dn10 = assign98450_e150756_d_n10;
        locals.var_sarg_dn11 = assign98450_e150756_d_n11;
        locals.var_sarg_dn14 = assign98450_e150756_d_n14;

        let (assign98460_e150773, assign98460_e150773_d_n0, assign98460_e150773_d_n2, assign98460_e150773_d_n4, assign98460_e150773_d_n5, assign98460_e150773_d_n6, assign98460_e150773_d_n7, assign98460_e150773_d_n8, assign98460_e150773_d_n9, assign98460_e150773_d_n10, assign98460_e150773_d_n11, assign98460_e150773_d_n14,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2280 != 0.0)) && (locals.var_guard2281 == 0.0)) {
        let (assign98460_e150771, assign98460_e150771_d_n0, assign98460_e150771_d_n2, assign98460_e150771_d_n4, assign98460_e150771_d_n5, assign98460_e150771_d_n6, assign98460_e150771_d_n7, assign98460_e150771_d_n8, assign98460_e150771_d_n9, assign98460_e150771_d_n10, assign98460_e150771_d_n11, assign98460_e150771_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98460_e150769: f64 = (-p.p504);
                let assign98460_e150770: f64 = (locals.var_arg).powf(assign98460_e150769);
                (assign98460_e150770, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn0)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn2)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn4)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn5)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn6)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn7)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn8)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn9)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn10)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn11)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98460_e150769) as f64).is_finite() && ((assign98460_e150769) as f64).fract() == 0.0 { if assign98460_e150769 == 0.0 { 0.0 } else { (assign98460_e150769 * ((locals.var_arg).powf(assign98460_e150769 - 1.0) * locals.var_arg_dn14)) } } else { (assign98460_e150770 * (assign98460_e150769 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign98460_e150771, assign98460_e150771_d_n0, assign98460_e150771_d_n2, assign98460_e150771_d_n4, assign98460_e150771_d_n5, assign98460_e150771_d_n6, assign98460_e150771_d_n7, assign98460_e150771_d_n8, assign98460_e150771_d_n9, assign98460_e150771_d_n10, assign98460_e150771_d_n11, assign98460_e150771_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98460_e150773;
        locals.var_sarg_dn0 = assign98460_e150773_d_n0;
        locals.var_sarg_dn2 = assign98460_e150773_d_n2;
        locals.var_sarg_dn4 = assign98460_e150773_d_n4;
        locals.var_sarg_dn5 = assign98460_e150773_d_n5;
        locals.var_sarg_dn6 = assign98460_e150773_d_n6;
        locals.var_sarg_dn7 = assign98460_e150773_d_n7;
        locals.var_sarg_dn8 = assign98460_e150773_d_n8;
        locals.var_sarg_dn9 = assign98460_e150773_d_n9;
        locals.var_sarg_dn10 = assign98460_e150773_d_n10;
        locals.var_sarg_dn11 = assign98460_e150773_d_n11;
        locals.var_sarg_dn14 = assign98460_e150773_d_n14;

        let (assign98470_e150791, assign98470_e150791_d_n0, assign98470_e150791_d_n2, assign98470_e150791_d_n4, assign98470_e150791_d_n5, assign98470_e150791_d_n6, assign98470_e150791_d_n7, assign98470_e150791_d_n8, assign98470_e150791_d_n9, assign98470_e150791_d_n10, assign98470_e150791_d_n11, assign98470_e150791_d_n14,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2280 != 0.0)) {
        let assign98470_e150779: f64 = (locals.var_pzbdsw * locals.var_czbdsw);
        let assign98470_e150783: f64 = (locals.var_arg * locals.var_sarg);
        let assign98470_e150784: f64 = (1.0 - assign98470_e150783);
        let assign98470_e150785: f64 = (assign98470_e150779 * assign98470_e150784);
        let assign98470_e150788: f64 = (1.0 - p.p504);
        let assign98470_e150789: f64 = (assign98470_e150785 / assign98470_e150788);
        (assign98470_e150789, (((((locals.var_pzbdsw_dn0 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn0)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98470_e150788), (((((locals.var_pzbdsw_dn2 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn2)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98470_e150788), (((((locals.var_pzbdsw_dn4 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn4)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98470_e150788), (((((locals.var_pzbdsw_dn5 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn5)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98470_e150788), (((((locals.var_pzbdsw_dn6 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn6)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98470_e150788), (((((locals.var_pzbdsw_dn7 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn7)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98470_e150788), (((((locals.var_pzbdsw_dn8 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn8)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98470_e150788), (((((locals.var_pzbdsw_dn9 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn9)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98470_e150788), (((((locals.var_pzbdsw_dn10 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn10)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98470_e150788), (((((locals.var_pzbdsw_dn11 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn11)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign98470_e150788), (((((locals.var_pzbdsw_dn14 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn14)) * assign98470_e150784) + (assign98470_e150779 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign98470_e150788),)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn11, locals.var_qbd_sws_dn14,)
    }
};
        locals.var_qbd_sws = assign98470_e150791;
        locals.var_qbd_sws_dn0 = assign98470_e150791_d_n0;
        locals.var_qbd_sws_dn2 = assign98470_e150791_d_n2;
        locals.var_qbd_sws_dn4 = assign98470_e150791_d_n4;
        locals.var_qbd_sws_dn5 = assign98470_e150791_d_n5;
        locals.var_qbd_sws_dn6 = assign98470_e150791_d_n6;
        locals.var_qbd_sws_dn7 = assign98470_e150791_d_n7;
        locals.var_qbd_sws_dn8 = assign98470_e150791_d_n8;
        locals.var_qbd_sws_dn9 = assign98470_e150791_d_n9;
        locals.var_qbd_sws_dn10 = assign98470_e150791_d_n10;
        locals.var_qbd_sws_dn11 = assign98470_e150791_d_n11;
        locals.var_qbd_sws_dn14 = assign98470_e150791_d_n14;

        let (assign98490_e150806, assign98490_e150806_d_n0, assign98490_e150806_d_n2, assign98490_e150806_d_n4, assign98490_e150806_d_n5, assign98490_e150806_d_n6, assign98490_e150806_d_n7, assign98490_e150806_d_n8, assign98490_e150806_d_n9, assign98490_e150806_d_n10, assign98490_e150806_d_n11, assign98490_e150806_d_n14,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2280 == 0.0)) {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98490_e150806;
        locals.var_t1_dn0 = assign98490_e150806_d_n0;
        locals.var_t1_dn2 = assign98490_e150806_d_n2;
        locals.var_t1_dn4 = assign98490_e150806_d_n4;
        locals.var_t1_dn5 = assign98490_e150806_d_n5;
        locals.var_t1_dn6 = assign98490_e150806_d_n6;
        locals.var_t1_dn7 = assign98490_e150806_d_n7;
        locals.var_t1_dn8 = assign98490_e150806_d_n8;
        locals.var_t1_dn9 = assign98490_e150806_d_n9;
        locals.var_t1_dn10 = assign98490_e150806_d_n10;
        locals.var_t1_dn11 = assign98490_e150806_d_n11;
        locals.var_t1_dn14 = assign98490_e150806_d_n14;

        let (assign98500_e150817, assign98500_e150817_d_n0, assign98500_e150817_d_n2, assign98500_e150817_d_n4, assign98500_e150817_d_n5, assign98500_e150817_d_n6, assign98500_e150817_d_n7, assign98500_e150817_d_n8, assign98500_e150817_d_n9, assign98500_e150817_d_n10, assign98500_e150817_d_n11, assign98500_e150817_d_n14,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2280 == 0.0)) {
        let assign98500_e150813: f64 = (locals.var_czbdsw * p.p504);
        let assign98500_e150815: f64 = (assign98500_e150813 / locals.var_pzbdsw);
        (assign98500_e150815, ((((locals.var_czbdsw_dn0 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn0)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn2 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn2)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn4 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn4)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn5 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn5)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn6 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn6)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn7 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn7)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn8 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn8)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn9 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn9)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn10 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn10)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn11 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn11)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn14 * p.p504) * locals.var_pzbdsw) - (assign98500_e150813 * locals.var_pzbdsw_dn14)) / (locals.var_pzbdsw * locals.var_pzbdsw)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98500_e150817;
        locals.var_t2_dn0 = assign98500_e150817_d_n0;
        locals.var_t2_dn2 = assign98500_e150817_d_n2;
        locals.var_t2_dn4 = assign98500_e150817_d_n4;
        locals.var_t2_dn5 = assign98500_e150817_d_n5;
        locals.var_t2_dn6 = assign98500_e150817_d_n6;
        locals.var_t2_dn7 = assign98500_e150817_d_n7;
        locals.var_t2_dn8 = assign98500_e150817_d_n8;
        locals.var_t2_dn9 = assign98500_e150817_d_n9;
        locals.var_t2_dn10 = assign98500_e150817_d_n10;
        locals.var_t2_dn11 = assign98500_e150817_d_n11;
        locals.var_t2_dn14 = assign98500_e150817_d_n14;

        let (assign98510_e150832, assign98510_e150832_d_n0, assign98510_e150832_d_n2, assign98510_e150832_d_n4, assign98510_e150832_d_n5, assign98510_e150832_d_n6, assign98510_e150832_d_n7, assign98510_e150832_d_n8, assign98510_e150832_d_n9, assign98510_e150832_d_n10, assign98510_e150832_d_n11, assign98510_e150832_d_n14,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2280 == 0.0)) {
        let assign98510_e150826: f64 = (locals.var_vbd_jct * 0.5);
        let assign98510_e150828: f64 = (assign98510_e150826 * locals.var_t2);
        let assign98510_e150829: f64 = (locals.var_t1 + assign98510_e150828);
        let assign98510_e150830: f64 = (locals.var_vbd_jct * assign98510_e150829);
        (assign98510_e150830, ((locals.var_vbd_jct_dn0 * assign98510_e150829) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98510_e150826 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98510_e150826 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98510_e150826 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98510_e150826 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98510_e150826 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98510_e150826 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98510_e150826 * locals.var_t2_dn8))), (locals.var_vbd_jct * (locals.var_t1_dn9 + (assign98510_e150826 * locals.var_t2_dn9))), ((locals.var_vbd_jct_dn10 * assign98510_e150829) + (locals.var_vbd_jct * (locals.var_t1_dn10 + (((locals.var_vbd_jct_dn10 * 0.5) * locals.var_t2) + (assign98510_e150826 * locals.var_t2_dn10))))), (locals.var_vbd_jct * (locals.var_t1_dn11 + (assign98510_e150826 * locals.var_t2_dn11))), (locals.var_vbd_jct * (locals.var_t1_dn14 + (assign98510_e150826 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn11, locals.var_qbd_sws_dn14,)
    }
};
        locals.var_qbd_sws = assign98510_e150832;
        locals.var_qbd_sws_dn0 = assign98510_e150832_d_n0;
        locals.var_qbd_sws_dn2 = assign98510_e150832_d_n2;
        locals.var_qbd_sws_dn4 = assign98510_e150832_d_n4;
        locals.var_qbd_sws_dn5 = assign98510_e150832_d_n5;
        locals.var_qbd_sws_dn6 = assign98510_e150832_d_n6;
        locals.var_qbd_sws_dn7 = assign98510_e150832_d_n7;
        locals.var_qbd_sws_dn8 = assign98510_e150832_d_n8;
        locals.var_qbd_sws_dn9 = assign98510_e150832_d_n9;
        locals.var_qbd_sws_dn10 = assign98510_e150832_d_n10;
        locals.var_qbd_sws_dn11 = assign98510_e150832_d_n11;
        locals.var_qbd_sws_dn14 = assign98510_e150832_d_n14;

        let (assign98530_e150848, assign98530_e150848_d_n0, assign98530_e150848_d_n2, assign98530_e150848_d_n4, assign98530_e150848_d_n5, assign98530_e150848_d_n6, assign98530_e150848_d_n7, assign98530_e150848_d_n8, assign98530_e150848_d_n9, assign98530_e150848_d_n10, assign98530_e150848_d_n11, assign98530_e150848_d_n14,) = {
    if (locals.var_guard2279 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn11, locals.var_qbd_sws_dn14,)
    }
};
        locals.var_qbd_sws = assign98530_e150848;
        locals.var_qbd_sws_dn0 = assign98530_e150848_d_n0;
        locals.var_qbd_sws_dn2 = assign98530_e150848_d_n2;
        locals.var_qbd_sws_dn4 = assign98530_e150848_d_n4;
        locals.var_qbd_sws_dn5 = assign98530_e150848_d_n5;
        locals.var_qbd_sws_dn6 = assign98530_e150848_d_n6;
        locals.var_qbd_sws_dn7 = assign98530_e150848_d_n7;
        locals.var_qbd_sws_dn8 = assign98530_e150848_d_n8;
        locals.var_qbd_sws_dn9 = assign98530_e150848_d_n9;
        locals.var_qbd_sws_dn10 = assign98530_e150848_d_n10;
        locals.var_qbd_sws_dn11 = assign98530_e150848_d_n11;
        locals.var_qbd_sws_dn14 = assign98530_e150848_d_n14;

        let assign98550_e150856: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2282 = assign98550_e150856;

        let assign98560_e150859: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2283 = assign98560_e150859;

        let assign98570_e150862: f64 = if locals.var_vbdi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2284 = assign98570_e150862;

        let (assign98580_e150874, assign98580_e150874_d_n0, assign98580_e150874_d_n2, assign98580_e150874_d_n4, assign98580_e150874_d_n5, assign98580_e150874_d_n6, assign98580_e150874_d_n7, assign98580_e150874_d_n8, assign98580_e150874_d_n9, assign98580_e150874_d_n10, assign98580_e150874_d_n11, assign98580_e150874_d_n14,) = {
    if (((locals.var_guard2282 != 0.0) && (locals.var_guard2283 != 0.0)) && (locals.var_guard2284 != 0.0)) {
        let assign98580_e150871: f64 = (locals.var_vbdi_jct / locals.var_pzbdswg);
        let assign98580_e150872: f64 = (1.0 - assign98580_e150871);
        (assign98580_e150872, (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn0) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn2) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn4) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn5) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbdi_jct_dn6 * locals.var_pzbdswg) - (locals.var_vbdi_jct * locals.var_pzbdswg_dn6)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn7) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn8) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbdi_jct_dn9 * locals.var_pzbdswg) - (locals.var_vbdi_jct * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn10) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn11) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn14) / (locals.var_pzbdswg * locals.var_pzbdswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign98580_e150874;
        locals.var_arg_dn0 = assign98580_e150874_d_n0;
        locals.var_arg_dn2 = assign98580_e150874_d_n2;
        locals.var_arg_dn4 = assign98580_e150874_d_n4;
        locals.var_arg_dn5 = assign98580_e150874_d_n5;
        locals.var_arg_dn6 = assign98580_e150874_d_n6;
        locals.var_arg_dn7 = assign98580_e150874_d_n7;
        locals.var_arg_dn8 = assign98580_e150874_d_n8;
        locals.var_arg_dn9 = assign98580_e150874_d_n9;
        locals.var_arg_dn10 = assign98580_e150874_d_n10;
        locals.var_arg_dn11 = assign98580_e150874_d_n11;
        locals.var_arg_dn14 = assign98580_e150874_d_n14;

        let assign98590_e150877: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2285 = assign98590_e150877;

        let (assign98600_e150890, assign98600_e150890_d_n0, assign98600_e150890_d_n2, assign98600_e150890_d_n4, assign98600_e150890_d_n5, assign98600_e150890_d_n6, assign98600_e150890_d_n7, assign98600_e150890_d_n8, assign98600_e150890_d_n9, assign98600_e150890_d_n10, assign98600_e150890_d_n11, assign98600_e150890_d_n14,) = {
    if ((((locals.var_guard2282 != 0.0) && (locals.var_guard2283 != 0.0)) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) {
        let assign98600_e150887: f64 = (locals.var_arg).sqrt();
        let assign98600_e150888: f64 = (1.0 / assign98600_e150887);
        (assign98600_e150888, (-((locals.var_arg_dn0 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))), (-((locals.var_arg_dn2 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))), (-((locals.var_arg_dn4 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))), (-((locals.var_arg_dn5 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))), (-((locals.var_arg_dn6 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))), (-((locals.var_arg_dn7 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))), (-((locals.var_arg_dn8 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))), (-((locals.var_arg_dn9 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))), (-((locals.var_arg_dn10 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))), (-((locals.var_arg_dn11 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))), (-((locals.var_arg_dn14 / (2.0 * assign98600_e150887)) / (assign98600_e150887 * assign98600_e150887))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98600_e150890;
        locals.var_sarg_dn0 = assign98600_e150890_d_n0;
        locals.var_sarg_dn2 = assign98600_e150890_d_n2;
        locals.var_sarg_dn4 = assign98600_e150890_d_n4;
        locals.var_sarg_dn5 = assign98600_e150890_d_n5;
        locals.var_sarg_dn6 = assign98600_e150890_d_n6;
        locals.var_sarg_dn7 = assign98600_e150890_d_n7;
        locals.var_sarg_dn8 = assign98600_e150890_d_n8;
        locals.var_sarg_dn9 = assign98600_e150890_d_n9;
        locals.var_sarg_dn10 = assign98600_e150890_d_n10;
        locals.var_sarg_dn11 = assign98600_e150890_d_n11;
        locals.var_sarg_dn14 = assign98600_e150890_d_n14;

    }

    pub(super) fn stamp_transient_block_363(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98610_e150909, assign98610_e150909_d_n0, assign98610_e150909_d_n2, assign98610_e150909_d_n4, assign98610_e150909_d_n5, assign98610_e150909_d_n6, assign98610_e150909_d_n7, assign98610_e150909_d_n8, assign98610_e150909_d_n9, assign98610_e150909_d_n10, assign98610_e150909_d_n11, assign98610_e150909_d_n14,) = {
    if ((((locals.var_guard2282 != 0.0) && (locals.var_guard2283 != 0.0)) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 == 0.0)) {
        let (assign98610_e150907, assign98610_e150907_d_n0, assign98610_e150907_d_n2, assign98610_e150907_d_n4, assign98610_e150907_d_n5, assign98610_e150907_d_n6, assign98610_e150907_d_n7, assign98610_e150907_d_n8, assign98610_e150907_d_n9, assign98610_e150907_d_n10, assign98610_e150907_d_n11, assign98610_e150907_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98610_e150905: f64 = (-p.p505);
                let assign98610_e150906: f64 = (locals.var_arg).powf(assign98610_e150905);
                (assign98610_e150906, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn0)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn2)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn4)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn5)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn6)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn7)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn8)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn9)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn10)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn11)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98610_e150905) as f64).is_finite() && ((assign98610_e150905) as f64).fract() == 0.0 { if assign98610_e150905 == 0.0 { 0.0 } else { (assign98610_e150905 * ((locals.var_arg).powf(assign98610_e150905 - 1.0) * locals.var_arg_dn14)) } } else { (assign98610_e150906 * (assign98610_e150905 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign98610_e150907, assign98610_e150907_d_n0, assign98610_e150907_d_n2, assign98610_e150907_d_n4, assign98610_e150907_d_n5, assign98610_e150907_d_n6, assign98610_e150907_d_n7, assign98610_e150907_d_n8, assign98610_e150907_d_n9, assign98610_e150907_d_n10, assign98610_e150907_d_n11, assign98610_e150907_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98610_e150909;
        locals.var_sarg_dn0 = assign98610_e150909_d_n0;
        locals.var_sarg_dn2 = assign98610_e150909_d_n2;
        locals.var_sarg_dn4 = assign98610_e150909_d_n4;
        locals.var_sarg_dn5 = assign98610_e150909_d_n5;
        locals.var_sarg_dn6 = assign98610_e150909_d_n6;
        locals.var_sarg_dn7 = assign98610_e150909_d_n7;
        locals.var_sarg_dn8 = assign98610_e150909_d_n8;
        locals.var_sarg_dn9 = assign98610_e150909_d_n9;
        locals.var_sarg_dn10 = assign98610_e150909_d_n10;
        locals.var_sarg_dn11 = assign98610_e150909_d_n11;
        locals.var_sarg_dn14 = assign98610_e150909_d_n14;

        let (assign98620_e150929, assign98620_e150929_d_n0, assign98620_e150929_d_n2, assign98620_e150929_d_n4, assign98620_e150929_d_n5, assign98620_e150929_d_n6, assign98620_e150929_d_n7, assign98620_e150929_d_n8, assign98620_e150929_d_n9, assign98620_e150929_d_n10, assign98620_e150929_d_n11, assign98620_e150929_d_n14,) = {
    if (((locals.var_guard2282 != 0.0) && (locals.var_guard2283 != 0.0)) && (locals.var_guard2284 != 0.0)) {
        let assign98620_e150917: f64 = (locals.var_pzbdswg * locals.var_czbdswg);
        let assign98620_e150921: f64 = (locals.var_arg * locals.var_sarg);
        let assign98620_e150922: f64 = (1.0 - assign98620_e150921);
        let assign98620_e150923: f64 = (assign98620_e150917 * assign98620_e150922);
        let assign98620_e150926: f64 = (1.0 - p.p505);
        let assign98620_e150927: f64 = (assign98620_e150923 / assign98620_e150926);
        (assign98620_e150927, (((((locals.var_pzbdswg_dn0 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn0)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98620_e150926), (((((locals.var_pzbdswg_dn2 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn2)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98620_e150926), (((((locals.var_pzbdswg_dn4 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn4)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98620_e150926), (((((locals.var_pzbdswg_dn5 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn5)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98620_e150926), (((((locals.var_pzbdswg_dn6 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn6)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98620_e150926), (((((locals.var_pzbdswg_dn7 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn7)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98620_e150926), (((((locals.var_pzbdswg_dn8 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn8)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98620_e150926), (((((locals.var_pzbdswg_dn9 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn9)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98620_e150926), (((((locals.var_pzbdswg_dn10 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn10)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98620_e150926), (((((locals.var_pzbdswg_dn11 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn11)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign98620_e150926), (((((locals.var_pzbdswg_dn14 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn14)) * assign98620_e150922) + (assign98620_e150917 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign98620_e150926),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98620_e150929;
        locals.var_qbd_swg_dn0 = assign98620_e150929_d_n0;
        locals.var_qbd_swg_dn2 = assign98620_e150929_d_n2;
        locals.var_qbd_swg_dn4 = assign98620_e150929_d_n4;
        locals.var_qbd_swg_dn5 = assign98620_e150929_d_n5;
        locals.var_qbd_swg_dn6 = assign98620_e150929_d_n6;
        locals.var_qbd_swg_dn7 = assign98620_e150929_d_n7;
        locals.var_qbd_swg_dn8 = assign98620_e150929_d_n8;
        locals.var_qbd_swg_dn9 = assign98620_e150929_d_n9;
        locals.var_qbd_swg_dn10 = assign98620_e150929_d_n10;
        locals.var_qbd_swg_dn11 = assign98620_e150929_d_n11;
        locals.var_qbd_swg_dn14 = assign98620_e150929_d_n14;

        let (assign98640_e150948, assign98640_e150948_d_n0, assign98640_e150948_d_n2, assign98640_e150948_d_n4, assign98640_e150948_d_n5, assign98640_e150948_d_n6, assign98640_e150948_d_n7, assign98640_e150948_d_n8, assign98640_e150948_d_n9, assign98640_e150948_d_n10, assign98640_e150948_d_n11, assign98640_e150948_d_n14,) = {
    if (((locals.var_guard2282 != 0.0) && (locals.var_guard2283 != 0.0)) && (locals.var_guard2284 == 0.0)) {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98640_e150948;
        locals.var_t1_dn0 = assign98640_e150948_d_n0;
        locals.var_t1_dn2 = assign98640_e150948_d_n2;
        locals.var_t1_dn4 = assign98640_e150948_d_n4;
        locals.var_t1_dn5 = assign98640_e150948_d_n5;
        locals.var_t1_dn6 = assign98640_e150948_d_n6;
        locals.var_t1_dn7 = assign98640_e150948_d_n7;
        locals.var_t1_dn8 = assign98640_e150948_d_n8;
        locals.var_t1_dn9 = assign98640_e150948_d_n9;
        locals.var_t1_dn10 = assign98640_e150948_d_n10;
        locals.var_t1_dn11 = assign98640_e150948_d_n11;
        locals.var_t1_dn14 = assign98640_e150948_d_n14;

        let (assign98650_e150961, assign98650_e150961_d_n0, assign98650_e150961_d_n2, assign98650_e150961_d_n4, assign98650_e150961_d_n5, assign98650_e150961_d_n6, assign98650_e150961_d_n7, assign98650_e150961_d_n8, assign98650_e150961_d_n9, assign98650_e150961_d_n10, assign98650_e150961_d_n11, assign98650_e150961_d_n14,) = {
    if (((locals.var_guard2282 != 0.0) && (locals.var_guard2283 != 0.0)) && (locals.var_guard2284 == 0.0)) {
        let assign98650_e150957: f64 = (locals.var_czbdswg * p.p505);
        let assign98650_e150959: f64 = (assign98650_e150957 / locals.var_pzbdswg);
        (assign98650_e150959, ((((locals.var_czbdswg_dn0 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn2 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn2)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn4 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn4)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn5 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn5)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn6 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn6)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn7 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn7)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn8 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn8)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn9 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn10 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn10)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn11 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn11)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn14 * p.p505) * locals.var_pzbdswg) - (assign98650_e150957 * locals.var_pzbdswg_dn14)) / (locals.var_pzbdswg * locals.var_pzbdswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98650_e150961;
        locals.var_t2_dn0 = assign98650_e150961_d_n0;
        locals.var_t2_dn2 = assign98650_e150961_d_n2;
        locals.var_t2_dn4 = assign98650_e150961_d_n4;
        locals.var_t2_dn5 = assign98650_e150961_d_n5;
        locals.var_t2_dn6 = assign98650_e150961_d_n6;
        locals.var_t2_dn7 = assign98650_e150961_d_n7;
        locals.var_t2_dn8 = assign98650_e150961_d_n8;
        locals.var_t2_dn9 = assign98650_e150961_d_n9;
        locals.var_t2_dn10 = assign98650_e150961_d_n10;
        locals.var_t2_dn11 = assign98650_e150961_d_n11;
        locals.var_t2_dn14 = assign98650_e150961_d_n14;

        let (assign98660_e150978, assign98660_e150978_d_n0, assign98660_e150978_d_n2, assign98660_e150978_d_n4, assign98660_e150978_d_n5, assign98660_e150978_d_n6, assign98660_e150978_d_n7, assign98660_e150978_d_n8, assign98660_e150978_d_n9, assign98660_e150978_d_n10, assign98660_e150978_d_n11, assign98660_e150978_d_n14,) = {
    if (((locals.var_guard2282 != 0.0) && (locals.var_guard2283 != 0.0)) && (locals.var_guard2284 == 0.0)) {
        let assign98660_e150972: f64 = (locals.var_vbdi_jct * 0.5);
        let assign98660_e150974: f64 = (assign98660_e150972 * locals.var_t2);
        let assign98660_e150975: f64 = (locals.var_t1 + assign98660_e150974);
        let assign98660_e150976: f64 = (locals.var_vbdi_jct * assign98660_e150975);
        (assign98660_e150976, (locals.var_vbdi_jct * (locals.var_t1_dn0 + (assign98660_e150972 * locals.var_t2_dn0))), (locals.var_vbdi_jct * (locals.var_t1_dn2 + (assign98660_e150972 * locals.var_t2_dn2))), (locals.var_vbdi_jct * (locals.var_t1_dn4 + (assign98660_e150972 * locals.var_t2_dn4))), (locals.var_vbdi_jct * (locals.var_t1_dn5 + (assign98660_e150972 * locals.var_t2_dn5))), ((locals.var_vbdi_jct_dn6 * assign98660_e150975) + (locals.var_vbdi_jct * (locals.var_t1_dn6 + (((locals.var_vbdi_jct_dn6 * 0.5) * locals.var_t2) + (assign98660_e150972 * locals.var_t2_dn6))))), (locals.var_vbdi_jct * (locals.var_t1_dn7 + (assign98660_e150972 * locals.var_t2_dn7))), (locals.var_vbdi_jct * (locals.var_t1_dn8 + (assign98660_e150972 * locals.var_t2_dn8))), ((locals.var_vbdi_jct_dn9 * assign98660_e150975) + (locals.var_vbdi_jct * (locals.var_t1_dn9 + (((locals.var_vbdi_jct_dn9 * 0.5) * locals.var_t2) + (assign98660_e150972 * locals.var_t2_dn9))))), (locals.var_vbdi_jct * (locals.var_t1_dn10 + (assign98660_e150972 * locals.var_t2_dn10))), (locals.var_vbdi_jct * (locals.var_t1_dn11 + (assign98660_e150972 * locals.var_t2_dn11))), (locals.var_vbdi_jct * (locals.var_t1_dn14 + (assign98660_e150972 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98660_e150978;
        locals.var_qbd_swg_dn0 = assign98660_e150978_d_n0;
        locals.var_qbd_swg_dn2 = assign98660_e150978_d_n2;
        locals.var_qbd_swg_dn4 = assign98660_e150978_d_n4;
        locals.var_qbd_swg_dn5 = assign98660_e150978_d_n5;
        locals.var_qbd_swg_dn6 = assign98660_e150978_d_n6;
        locals.var_qbd_swg_dn7 = assign98660_e150978_d_n7;
        locals.var_qbd_swg_dn8 = assign98660_e150978_d_n8;
        locals.var_qbd_swg_dn9 = assign98660_e150978_d_n9;
        locals.var_qbd_swg_dn10 = assign98660_e150978_d_n10;
        locals.var_qbd_swg_dn11 = assign98660_e150978_d_n11;
        locals.var_qbd_swg_dn14 = assign98660_e150978_d_n14;

        let (assign98680_e150998, assign98680_e150998_d_n0, assign98680_e150998_d_n2, assign98680_e150998_d_n4, assign98680_e150998_d_n5, assign98680_e150998_d_n6, assign98680_e150998_d_n7, assign98680_e150998_d_n8, assign98680_e150998_d_n9, assign98680_e150998_d_n10, assign98680_e150998_d_n11, assign98680_e150998_d_n14,) = {
    if ((locals.var_guard2282 != 0.0) && (locals.var_guard2283 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98680_e150998;
        locals.var_qbd_swg_dn0 = assign98680_e150998_d_n0;
        locals.var_qbd_swg_dn2 = assign98680_e150998_d_n2;
        locals.var_qbd_swg_dn4 = assign98680_e150998_d_n4;
        locals.var_qbd_swg_dn5 = assign98680_e150998_d_n5;
        locals.var_qbd_swg_dn6 = assign98680_e150998_d_n6;
        locals.var_qbd_swg_dn7 = assign98680_e150998_d_n7;
        locals.var_qbd_swg_dn8 = assign98680_e150998_d_n8;
        locals.var_qbd_swg_dn9 = assign98680_e150998_d_n9;
        locals.var_qbd_swg_dn10 = assign98680_e150998_d_n10;
        locals.var_qbd_swg_dn11 = assign98680_e150998_d_n11;
        locals.var_qbd_swg_dn14 = assign98680_e150998_d_n14;

        let assign98700_e151008: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2286 = assign98700_e151008;

        let assign98710_e151011: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2287 = assign98710_e151011;

        let (assign98720_e151024, assign98720_e151024_d_n0, assign98720_e151024_d_n2, assign98720_e151024_d_n4, assign98720_e151024_d_n5, assign98720_e151024_d_n6, assign98720_e151024_d_n7, assign98720_e151024_d_n8, assign98720_e151024_d_n9, assign98720_e151024_d_n10, assign98720_e151024_d_n11, assign98720_e151024_d_n14,) = {
    if (((locals.var_guard2282 == 0.0) && (locals.var_guard2286 != 0.0)) && (locals.var_guard2287 != 0.0)) {
        let assign98720_e151021: f64 = (locals.var_vbd_jct / locals.var_pzbdswg);
        let assign98720_e151022: f64 = (1.0 - assign98720_e151021);
        (assign98720_e151022, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbdswg) - (locals.var_vbd_jct * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn2) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn4) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn5) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn6) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn7) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn8) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn9) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbd_jct_dn10 * locals.var_pzbdswg) - (locals.var_vbd_jct * locals.var_pzbdswg_dn10)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn11) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn14) / (locals.var_pzbdswg * locals.var_pzbdswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign98720_e151024;
        locals.var_arg_dn0 = assign98720_e151024_d_n0;
        locals.var_arg_dn2 = assign98720_e151024_d_n2;
        locals.var_arg_dn4 = assign98720_e151024_d_n4;
        locals.var_arg_dn5 = assign98720_e151024_d_n5;
        locals.var_arg_dn6 = assign98720_e151024_d_n6;
        locals.var_arg_dn7 = assign98720_e151024_d_n7;
        locals.var_arg_dn8 = assign98720_e151024_d_n8;
        locals.var_arg_dn9 = assign98720_e151024_d_n9;
        locals.var_arg_dn10 = assign98720_e151024_d_n10;
        locals.var_arg_dn11 = assign98720_e151024_d_n11;
        locals.var_arg_dn14 = assign98720_e151024_d_n14;

        let assign98730_e151027: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2288 = assign98730_e151027;

        let (assign98740_e151041, assign98740_e151041_d_n0, assign98740_e151041_d_n2, assign98740_e151041_d_n4, assign98740_e151041_d_n5, assign98740_e151041_d_n6, assign98740_e151041_d_n7, assign98740_e151041_d_n8, assign98740_e151041_d_n9, assign98740_e151041_d_n10, assign98740_e151041_d_n11, assign98740_e151041_d_n14,) = {
    if ((((locals.var_guard2282 == 0.0) && (locals.var_guard2286 != 0.0)) && (locals.var_guard2287 != 0.0)) && (locals.var_guard2288 != 0.0)) {
        let assign98740_e151038: f64 = (locals.var_arg).sqrt();
        let assign98740_e151039: f64 = (1.0 / assign98740_e151038);
        (assign98740_e151039, (-((locals.var_arg_dn0 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))), (-((locals.var_arg_dn2 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))), (-((locals.var_arg_dn4 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))), (-((locals.var_arg_dn5 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))), (-((locals.var_arg_dn6 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))), (-((locals.var_arg_dn7 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))), (-((locals.var_arg_dn8 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))), (-((locals.var_arg_dn9 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))), (-((locals.var_arg_dn10 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))), (-((locals.var_arg_dn11 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))), (-((locals.var_arg_dn14 / (2.0 * assign98740_e151038)) / (assign98740_e151038 * assign98740_e151038))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98740_e151041;
        locals.var_sarg_dn0 = assign98740_e151041_d_n0;
        locals.var_sarg_dn2 = assign98740_e151041_d_n2;
        locals.var_sarg_dn4 = assign98740_e151041_d_n4;
        locals.var_sarg_dn5 = assign98740_e151041_d_n5;
        locals.var_sarg_dn6 = assign98740_e151041_d_n6;
        locals.var_sarg_dn7 = assign98740_e151041_d_n7;
        locals.var_sarg_dn8 = assign98740_e151041_d_n8;
        locals.var_sarg_dn9 = assign98740_e151041_d_n9;
        locals.var_sarg_dn10 = assign98740_e151041_d_n10;
        locals.var_sarg_dn11 = assign98740_e151041_d_n11;
        locals.var_sarg_dn14 = assign98740_e151041_d_n14;

        let (assign98750_e151061, assign98750_e151061_d_n0, assign98750_e151061_d_n2, assign98750_e151061_d_n4, assign98750_e151061_d_n5, assign98750_e151061_d_n6, assign98750_e151061_d_n7, assign98750_e151061_d_n8, assign98750_e151061_d_n9, assign98750_e151061_d_n10, assign98750_e151061_d_n11, assign98750_e151061_d_n14,) = {
    if ((((locals.var_guard2282 == 0.0) && (locals.var_guard2286 != 0.0)) && (locals.var_guard2287 != 0.0)) && (locals.var_guard2288 == 0.0)) {
        let (assign98750_e151059, assign98750_e151059_d_n0, assign98750_e151059_d_n2, assign98750_e151059_d_n4, assign98750_e151059_d_n5, assign98750_e151059_d_n6, assign98750_e151059_d_n7, assign98750_e151059_d_n8, assign98750_e151059_d_n9, assign98750_e151059_d_n10, assign98750_e151059_d_n11, assign98750_e151059_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98750_e151057: f64 = (-p.p505);
                let assign98750_e151058: f64 = (locals.var_arg).powf(assign98750_e151057);
                (assign98750_e151058, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn0)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn2)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn4)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn5)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn6)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn7)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn8)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn9)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn10)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn11)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98750_e151057) as f64).is_finite() && ((assign98750_e151057) as f64).fract() == 0.0 { if assign98750_e151057 == 0.0 { 0.0 } else { (assign98750_e151057 * ((locals.var_arg).powf(assign98750_e151057 - 1.0) * locals.var_arg_dn14)) } } else { (assign98750_e151058 * (assign98750_e151057 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign98750_e151059, assign98750_e151059_d_n0, assign98750_e151059_d_n2, assign98750_e151059_d_n4, assign98750_e151059_d_n5, assign98750_e151059_d_n6, assign98750_e151059_d_n7, assign98750_e151059_d_n8, assign98750_e151059_d_n9, assign98750_e151059_d_n10, assign98750_e151059_d_n11, assign98750_e151059_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98750_e151061;
        locals.var_sarg_dn0 = assign98750_e151061_d_n0;
        locals.var_sarg_dn2 = assign98750_e151061_d_n2;
        locals.var_sarg_dn4 = assign98750_e151061_d_n4;
        locals.var_sarg_dn5 = assign98750_e151061_d_n5;
        locals.var_sarg_dn6 = assign98750_e151061_d_n6;
        locals.var_sarg_dn7 = assign98750_e151061_d_n7;
        locals.var_sarg_dn8 = assign98750_e151061_d_n8;
        locals.var_sarg_dn9 = assign98750_e151061_d_n9;
        locals.var_sarg_dn10 = assign98750_e151061_d_n10;
        locals.var_sarg_dn11 = assign98750_e151061_d_n11;
        locals.var_sarg_dn14 = assign98750_e151061_d_n14;

        let (assign98760_e151082, assign98760_e151082_d_n0, assign98760_e151082_d_n2, assign98760_e151082_d_n4, assign98760_e151082_d_n5, assign98760_e151082_d_n6, assign98760_e151082_d_n7, assign98760_e151082_d_n8, assign98760_e151082_d_n9, assign98760_e151082_d_n10, assign98760_e151082_d_n11, assign98760_e151082_d_n14,) = {
    if (((locals.var_guard2282 == 0.0) && (locals.var_guard2286 != 0.0)) && (locals.var_guard2287 != 0.0)) {
        let assign98760_e151070: f64 = (locals.var_pzbdswg * locals.var_czbdswg);
        let assign98760_e151074: f64 = (locals.var_arg * locals.var_sarg);
        let assign98760_e151075: f64 = (1.0 - assign98760_e151074);
        let assign98760_e151076: f64 = (assign98760_e151070 * assign98760_e151075);
        let assign98760_e151079: f64 = (1.0 - p.p505);
        let assign98760_e151080: f64 = (assign98760_e151076 / assign98760_e151079);
        (assign98760_e151080, (((((locals.var_pzbdswg_dn0 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn0)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98760_e151079), (((((locals.var_pzbdswg_dn2 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn2)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98760_e151079), (((((locals.var_pzbdswg_dn4 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn4)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98760_e151079), (((((locals.var_pzbdswg_dn5 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn5)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98760_e151079), (((((locals.var_pzbdswg_dn6 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn6)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98760_e151079), (((((locals.var_pzbdswg_dn7 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn7)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98760_e151079), (((((locals.var_pzbdswg_dn8 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn8)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98760_e151079), (((((locals.var_pzbdswg_dn9 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn9)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98760_e151079), (((((locals.var_pzbdswg_dn10 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn10)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98760_e151079), (((((locals.var_pzbdswg_dn11 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn11)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign98760_e151079), (((((locals.var_pzbdswg_dn14 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn14)) * assign98760_e151075) + (assign98760_e151070 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign98760_e151079),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98760_e151082;
        locals.var_qbd_swg_dn0 = assign98760_e151082_d_n0;
        locals.var_qbd_swg_dn2 = assign98760_e151082_d_n2;
        locals.var_qbd_swg_dn4 = assign98760_e151082_d_n4;
        locals.var_qbd_swg_dn5 = assign98760_e151082_d_n5;
        locals.var_qbd_swg_dn6 = assign98760_e151082_d_n6;
        locals.var_qbd_swg_dn7 = assign98760_e151082_d_n7;
        locals.var_qbd_swg_dn8 = assign98760_e151082_d_n8;
        locals.var_qbd_swg_dn9 = assign98760_e151082_d_n9;
        locals.var_qbd_swg_dn10 = assign98760_e151082_d_n10;
        locals.var_qbd_swg_dn11 = assign98760_e151082_d_n11;
        locals.var_qbd_swg_dn14 = assign98760_e151082_d_n14;

        let (assign98780_e151103, assign98780_e151103_d_n0, assign98780_e151103_d_n2, assign98780_e151103_d_n4, assign98780_e151103_d_n5, assign98780_e151103_d_n6, assign98780_e151103_d_n7, assign98780_e151103_d_n8, assign98780_e151103_d_n9, assign98780_e151103_d_n10, assign98780_e151103_d_n11, assign98780_e151103_d_n14,) = {
    if (((locals.var_guard2282 == 0.0) && (locals.var_guard2286 != 0.0)) && (locals.var_guard2287 == 0.0)) {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98780_e151103;
        locals.var_t1_dn0 = assign98780_e151103_d_n0;
        locals.var_t1_dn2 = assign98780_e151103_d_n2;
        locals.var_t1_dn4 = assign98780_e151103_d_n4;
        locals.var_t1_dn5 = assign98780_e151103_d_n5;
        locals.var_t1_dn6 = assign98780_e151103_d_n6;
        locals.var_t1_dn7 = assign98780_e151103_d_n7;
        locals.var_t1_dn8 = assign98780_e151103_d_n8;
        locals.var_t1_dn9 = assign98780_e151103_d_n9;
        locals.var_t1_dn10 = assign98780_e151103_d_n10;
        locals.var_t1_dn11 = assign98780_e151103_d_n11;
        locals.var_t1_dn14 = assign98780_e151103_d_n14;

        let (assign98790_e151117, assign98790_e151117_d_n0, assign98790_e151117_d_n2, assign98790_e151117_d_n4, assign98790_e151117_d_n5, assign98790_e151117_d_n6, assign98790_e151117_d_n7, assign98790_e151117_d_n8, assign98790_e151117_d_n9, assign98790_e151117_d_n10, assign98790_e151117_d_n11, assign98790_e151117_d_n14,) = {
    if (((locals.var_guard2282 == 0.0) && (locals.var_guard2286 != 0.0)) && (locals.var_guard2287 == 0.0)) {
        let assign98790_e151113: f64 = (locals.var_czbdswg * p.p505);
        let assign98790_e151115: f64 = (assign98790_e151113 / locals.var_pzbdswg);
        (assign98790_e151115, ((((locals.var_czbdswg_dn0 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn2 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn2)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn4 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn4)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn5 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn5)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn6 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn6)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn7 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn7)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn8 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn8)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn9 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn10 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn10)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn11 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn11)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn14 * p.p505) * locals.var_pzbdswg) - (assign98790_e151113 * locals.var_pzbdswg_dn14)) / (locals.var_pzbdswg * locals.var_pzbdswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98790_e151117;
        locals.var_t2_dn0 = assign98790_e151117_d_n0;
        locals.var_t2_dn2 = assign98790_e151117_d_n2;
        locals.var_t2_dn4 = assign98790_e151117_d_n4;
        locals.var_t2_dn5 = assign98790_e151117_d_n5;
        locals.var_t2_dn6 = assign98790_e151117_d_n6;
        locals.var_t2_dn7 = assign98790_e151117_d_n7;
        locals.var_t2_dn8 = assign98790_e151117_d_n8;
        locals.var_t2_dn9 = assign98790_e151117_d_n9;
        locals.var_t2_dn10 = assign98790_e151117_d_n10;
        locals.var_t2_dn11 = assign98790_e151117_d_n11;
        locals.var_t2_dn14 = assign98790_e151117_d_n14;

        let (assign98800_e151135, assign98800_e151135_d_n0, assign98800_e151135_d_n2, assign98800_e151135_d_n4, assign98800_e151135_d_n5, assign98800_e151135_d_n6, assign98800_e151135_d_n7, assign98800_e151135_d_n8, assign98800_e151135_d_n9, assign98800_e151135_d_n10, assign98800_e151135_d_n11, assign98800_e151135_d_n14,) = {
    if (((locals.var_guard2282 == 0.0) && (locals.var_guard2286 != 0.0)) && (locals.var_guard2287 == 0.0)) {
        let assign98800_e151129: f64 = (locals.var_vbd_jct * 0.5);
        let assign98800_e151131: f64 = (assign98800_e151129 * locals.var_t2);
        let assign98800_e151132: f64 = (locals.var_t1 + assign98800_e151131);
        let assign98800_e151133: f64 = (locals.var_vbd_jct * assign98800_e151132);
        (assign98800_e151133, ((locals.var_vbd_jct_dn0 * assign98800_e151132) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98800_e151129 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98800_e151129 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98800_e151129 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98800_e151129 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98800_e151129 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98800_e151129 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98800_e151129 * locals.var_t2_dn8))), (locals.var_vbd_jct * (locals.var_t1_dn9 + (assign98800_e151129 * locals.var_t2_dn9))), ((locals.var_vbd_jct_dn10 * assign98800_e151132) + (locals.var_vbd_jct * (locals.var_t1_dn10 + (((locals.var_vbd_jct_dn10 * 0.5) * locals.var_t2) + (assign98800_e151129 * locals.var_t2_dn10))))), (locals.var_vbd_jct * (locals.var_t1_dn11 + (assign98800_e151129 * locals.var_t2_dn11))), (locals.var_vbd_jct * (locals.var_t1_dn14 + (assign98800_e151129 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98800_e151135;
        locals.var_qbd_swg_dn0 = assign98800_e151135_d_n0;
        locals.var_qbd_swg_dn2 = assign98800_e151135_d_n2;
        locals.var_qbd_swg_dn4 = assign98800_e151135_d_n4;
        locals.var_qbd_swg_dn5 = assign98800_e151135_d_n5;
        locals.var_qbd_swg_dn6 = assign98800_e151135_d_n6;
        locals.var_qbd_swg_dn7 = assign98800_e151135_d_n7;
        locals.var_qbd_swg_dn8 = assign98800_e151135_d_n8;
        locals.var_qbd_swg_dn9 = assign98800_e151135_d_n9;
        locals.var_qbd_swg_dn10 = assign98800_e151135_d_n10;
        locals.var_qbd_swg_dn11 = assign98800_e151135_d_n11;
        locals.var_qbd_swg_dn14 = assign98800_e151135_d_n14;

        let (assign98820_e151157, assign98820_e151157_d_n0, assign98820_e151157_d_n2, assign98820_e151157_d_n4, assign98820_e151157_d_n5, assign98820_e151157_d_n6, assign98820_e151157_d_n7, assign98820_e151157_d_n8, assign98820_e151157_d_n9, assign98820_e151157_d_n10, assign98820_e151157_d_n11, assign98820_e151157_d_n14,) = {
    if ((locals.var_guard2282 == 0.0) && (locals.var_guard2286 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98820_e151157;
        locals.var_qbd_swg_dn0 = assign98820_e151157_d_n0;
        locals.var_qbd_swg_dn2 = assign98820_e151157_d_n2;
        locals.var_qbd_swg_dn4 = assign98820_e151157_d_n4;
        locals.var_qbd_swg_dn5 = assign98820_e151157_d_n5;
        locals.var_qbd_swg_dn6 = assign98820_e151157_d_n6;
        locals.var_qbd_swg_dn7 = assign98820_e151157_d_n7;
        locals.var_qbd_swg_dn8 = assign98820_e151157_d_n8;
        locals.var_qbd_swg_dn9 = assign98820_e151157_d_n9;
        locals.var_qbd_swg_dn10 = assign98820_e151157_d_n10;
        locals.var_qbd_swg_dn11 = assign98820_e151157_d_n11;
        locals.var_qbd_swg_dn14 = assign98820_e151157_d_n14;

        let assign98840_e151168: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2289 = assign98840_e151168;

        let assign98850_e151171: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2290 = assign98850_e151171;

        let (assign98860_e151181, assign98860_e151181_d_n0, assign98860_e151181_d_n2, assign98860_e151181_d_n4, assign98860_e151181_d_n5, assign98860_e151181_d_n6, assign98860_e151181_d_n7, assign98860_e151181_d_n8, assign98860_e151181_d_n9, assign98860_e151181_d_n10, assign98860_e151181_d_n11, assign98860_e151181_d_n14,) = {
    if ((locals.var_guard2289 != 0.0) && (locals.var_guard2290 != 0.0)) {
        let assign98860_e151178: f64 = (locals.var_vbs_jct / locals.var_pzbs);
        let assign98860_e151179: f64 = (1.0 - assign98860_e151178);
        (assign98860_e151179, (-(-((locals.var_vbs_jct * locals.var_pzbs_dn0) / (locals.var_pzbs * locals.var_pzbs)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbs) - (locals.var_vbs_jct * locals.var_pzbs_dn2)) / (locals.var_pzbs * locals.var_pzbs))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn4) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn5) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn6) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn7) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn8) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn9) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn10) / (locals.var_pzbs * locals.var_pzbs)))), (-(((locals.var_vbs_jct_dn11 * locals.var_pzbs) - (locals.var_vbs_jct * locals.var_pzbs_dn11)) / (locals.var_pzbs * locals.var_pzbs))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn14) / (locals.var_pzbs * locals.var_pzbs)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign98860_e151181;
        locals.var_arg_dn0 = assign98860_e151181_d_n0;
        locals.var_arg_dn2 = assign98860_e151181_d_n2;
        locals.var_arg_dn4 = assign98860_e151181_d_n4;
        locals.var_arg_dn5 = assign98860_e151181_d_n5;
        locals.var_arg_dn6 = assign98860_e151181_d_n6;
        locals.var_arg_dn7 = assign98860_e151181_d_n7;
        locals.var_arg_dn8 = assign98860_e151181_d_n8;
        locals.var_arg_dn9 = assign98860_e151181_d_n9;
        locals.var_arg_dn10 = assign98860_e151181_d_n10;
        locals.var_arg_dn11 = assign98860_e151181_d_n11;
        locals.var_arg_dn14 = assign98860_e151181_d_n14;

        let assign98870_e151184: f64 = if p.p526 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2291 = assign98870_e151184;

        let (assign98880_e151195, assign98880_e151195_d_n0, assign98880_e151195_d_n2, assign98880_e151195_d_n4, assign98880_e151195_d_n5, assign98880_e151195_d_n6, assign98880_e151195_d_n7, assign98880_e151195_d_n8, assign98880_e151195_d_n9, assign98880_e151195_d_n10, assign98880_e151195_d_n11, assign98880_e151195_d_n14,) = {
    if (((locals.var_guard2289 != 0.0) && (locals.var_guard2290 != 0.0)) && (locals.var_guard2291 != 0.0)) {
        let assign98880_e151192: f64 = (locals.var_arg).sqrt();
        let assign98880_e151193: f64 = (1.0 / assign98880_e151192);
        (assign98880_e151193, (-((locals.var_arg_dn0 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))), (-((locals.var_arg_dn2 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))), (-((locals.var_arg_dn4 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))), (-((locals.var_arg_dn5 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))), (-((locals.var_arg_dn6 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))), (-((locals.var_arg_dn7 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))), (-((locals.var_arg_dn8 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))), (-((locals.var_arg_dn9 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))), (-((locals.var_arg_dn10 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))), (-((locals.var_arg_dn11 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))), (-((locals.var_arg_dn14 / (2.0 * assign98880_e151192)) / (assign98880_e151192 * assign98880_e151192))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98880_e151195;
        locals.var_sarg_dn0 = assign98880_e151195_d_n0;
        locals.var_sarg_dn2 = assign98880_e151195_d_n2;
        locals.var_sarg_dn4 = assign98880_e151195_d_n4;
        locals.var_sarg_dn5 = assign98880_e151195_d_n5;
        locals.var_sarg_dn6 = assign98880_e151195_d_n6;
        locals.var_sarg_dn7 = assign98880_e151195_d_n7;
        locals.var_sarg_dn8 = assign98880_e151195_d_n8;
        locals.var_sarg_dn9 = assign98880_e151195_d_n9;
        locals.var_sarg_dn10 = assign98880_e151195_d_n10;
        locals.var_sarg_dn11 = assign98880_e151195_d_n11;
        locals.var_sarg_dn14 = assign98880_e151195_d_n14;

        let (assign98890_e151212, assign98890_e151212_d_n0, assign98890_e151212_d_n2, assign98890_e151212_d_n4, assign98890_e151212_d_n5, assign98890_e151212_d_n6, assign98890_e151212_d_n7, assign98890_e151212_d_n8, assign98890_e151212_d_n9, assign98890_e151212_d_n10, assign98890_e151212_d_n11, assign98890_e151212_d_n14,) = {
    if (((locals.var_guard2289 != 0.0) && (locals.var_guard2290 != 0.0)) && (locals.var_guard2291 == 0.0)) {
        let (assign98890_e151210, assign98890_e151210_d_n0, assign98890_e151210_d_n2, assign98890_e151210_d_n4, assign98890_e151210_d_n5, assign98890_e151210_d_n6, assign98890_e151210_d_n7, assign98890_e151210_d_n8, assign98890_e151210_d_n9, assign98890_e151210_d_n10, assign98890_e151210_d_n11, assign98890_e151210_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98890_e151208: f64 = (-p.p526);
                let assign98890_e151209: f64 = (locals.var_arg).powf(assign98890_e151208);
                (assign98890_e151209, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn0)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn2)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn4)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn5)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn6)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn7)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn8)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn9)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn10)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn11)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98890_e151208) as f64).is_finite() && ((assign98890_e151208) as f64).fract() == 0.0 { if assign98890_e151208 == 0.0 { 0.0 } else { (assign98890_e151208 * ((locals.var_arg).powf(assign98890_e151208 - 1.0) * locals.var_arg_dn14)) } } else { (assign98890_e151209 * (assign98890_e151208 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign98890_e151210, assign98890_e151210_d_n0, assign98890_e151210_d_n2, assign98890_e151210_d_n4, assign98890_e151210_d_n5, assign98890_e151210_d_n6, assign98890_e151210_d_n7, assign98890_e151210_d_n8, assign98890_e151210_d_n9, assign98890_e151210_d_n10, assign98890_e151210_d_n11, assign98890_e151210_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98890_e151212;
        locals.var_sarg_dn0 = assign98890_e151212_d_n0;
        locals.var_sarg_dn2 = assign98890_e151212_d_n2;
        locals.var_sarg_dn4 = assign98890_e151212_d_n4;
        locals.var_sarg_dn5 = assign98890_e151212_d_n5;
        locals.var_sarg_dn6 = assign98890_e151212_d_n6;
        locals.var_sarg_dn7 = assign98890_e151212_d_n7;
        locals.var_sarg_dn8 = assign98890_e151212_d_n8;
        locals.var_sarg_dn9 = assign98890_e151212_d_n9;
        locals.var_sarg_dn10 = assign98890_e151212_d_n10;
        locals.var_sarg_dn11 = assign98890_e151212_d_n11;
        locals.var_sarg_dn14 = assign98890_e151212_d_n14;

        let (assign98900_e151230, assign98900_e151230_d_n0, assign98900_e151230_d_n2, assign98900_e151230_d_n4, assign98900_e151230_d_n5, assign98900_e151230_d_n6, assign98900_e151230_d_n7, assign98900_e151230_d_n8, assign98900_e151230_d_n9, assign98900_e151230_d_n10, assign98900_e151230_d_n11, assign98900_e151230_d_n14,) = {
    if ((locals.var_guard2289 != 0.0) && (locals.var_guard2290 != 0.0)) {
        let assign98900_e151218: f64 = (locals.var_pzbs * locals.var_czbs);
        let assign98900_e151222: f64 = (locals.var_arg * locals.var_sarg);
        let assign98900_e151223: f64 = (1.0 - assign98900_e151222);
        let assign98900_e151224: f64 = (assign98900_e151218 * assign98900_e151223);
        let assign98900_e151227: f64 = (1.0 - p.p526);
        let assign98900_e151228: f64 = (assign98900_e151224 / assign98900_e151227);
        (assign98900_e151228, (((((locals.var_pzbs_dn0 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn0)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98900_e151227), (((((locals.var_pzbs_dn2 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn2)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98900_e151227), (((((locals.var_pzbs_dn4 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn4)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98900_e151227), (((((locals.var_pzbs_dn5 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn5)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98900_e151227), (((((locals.var_pzbs_dn6 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn6)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98900_e151227), (((((locals.var_pzbs_dn7 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn7)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98900_e151227), (((((locals.var_pzbs_dn8 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn8)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98900_e151227), (((((locals.var_pzbs_dn9 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn9)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98900_e151227), (((((locals.var_pzbs_dn10 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn10)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98900_e151227), (((((locals.var_pzbs_dn11 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn11)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign98900_e151227), (((((locals.var_pzbs_dn14 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn14)) * assign98900_e151223) + (assign98900_e151218 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign98900_e151227),)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn11, locals.var_qbs_btm_dn14,)
    }
};
        locals.var_qbs_btm = assign98900_e151230;
        locals.var_qbs_btm_dn0 = assign98900_e151230_d_n0;
        locals.var_qbs_btm_dn2 = assign98900_e151230_d_n2;
        locals.var_qbs_btm_dn4 = assign98900_e151230_d_n4;
        locals.var_qbs_btm_dn5 = assign98900_e151230_d_n5;
        locals.var_qbs_btm_dn6 = assign98900_e151230_d_n6;
        locals.var_qbs_btm_dn7 = assign98900_e151230_d_n7;
        locals.var_qbs_btm_dn8 = assign98900_e151230_d_n8;
        locals.var_qbs_btm_dn9 = assign98900_e151230_d_n9;
        locals.var_qbs_btm_dn10 = assign98900_e151230_d_n10;
        locals.var_qbs_btm_dn11 = assign98900_e151230_d_n11;
        locals.var_qbs_btm_dn14 = assign98900_e151230_d_n14;

        let (assign98920_e151245, assign98920_e151245_d_n0, assign98920_e151245_d_n2, assign98920_e151245_d_n4, assign98920_e151245_d_n5, assign98920_e151245_d_n6, assign98920_e151245_d_n7, assign98920_e151245_d_n8, assign98920_e151245_d_n9, assign98920_e151245_d_n10, assign98920_e151245_d_n11, assign98920_e151245_d_n14,) = {
    if ((locals.var_guard2289 != 0.0) && (locals.var_guard2290 == 0.0)) {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98920_e151245;
        locals.var_t1_dn0 = assign98920_e151245_d_n0;
        locals.var_t1_dn2 = assign98920_e151245_d_n2;
        locals.var_t1_dn4 = assign98920_e151245_d_n4;
        locals.var_t1_dn5 = assign98920_e151245_d_n5;
        locals.var_t1_dn6 = assign98920_e151245_d_n6;
        locals.var_t1_dn7 = assign98920_e151245_d_n7;
        locals.var_t1_dn8 = assign98920_e151245_d_n8;
        locals.var_t1_dn9 = assign98920_e151245_d_n9;
        locals.var_t1_dn10 = assign98920_e151245_d_n10;
        locals.var_t1_dn11 = assign98920_e151245_d_n11;
        locals.var_t1_dn14 = assign98920_e151245_d_n14;

        let (assign98930_e151256, assign98930_e151256_d_n0, assign98930_e151256_d_n2, assign98930_e151256_d_n4, assign98930_e151256_d_n5, assign98930_e151256_d_n6, assign98930_e151256_d_n7, assign98930_e151256_d_n8, assign98930_e151256_d_n9, assign98930_e151256_d_n10, assign98930_e151256_d_n11, assign98930_e151256_d_n14,) = {
    if ((locals.var_guard2289 != 0.0) && (locals.var_guard2290 == 0.0)) {
        let assign98930_e151252: f64 = (locals.var_czbs * p.p526);
        let assign98930_e151254: f64 = (assign98930_e151252 / locals.var_pzbs);
        (assign98930_e151254, ((((locals.var_czbs_dn0 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn0)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn2 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn2)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn4 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn4)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn5 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn5)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn6 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn6)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn7 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn7)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn8 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn8)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn9 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn9)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn10 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn10)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn11 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn11)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn14 * p.p526) * locals.var_pzbs) - (assign98930_e151252 * locals.var_pzbs_dn14)) / (locals.var_pzbs * locals.var_pzbs)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98930_e151256;
        locals.var_t2_dn0 = assign98930_e151256_d_n0;
        locals.var_t2_dn2 = assign98930_e151256_d_n2;
        locals.var_t2_dn4 = assign98930_e151256_d_n4;
        locals.var_t2_dn5 = assign98930_e151256_d_n5;
        locals.var_t2_dn6 = assign98930_e151256_d_n6;
        locals.var_t2_dn7 = assign98930_e151256_d_n7;
        locals.var_t2_dn8 = assign98930_e151256_d_n8;
        locals.var_t2_dn9 = assign98930_e151256_d_n9;
        locals.var_t2_dn10 = assign98930_e151256_d_n10;
        locals.var_t2_dn11 = assign98930_e151256_d_n11;
        locals.var_t2_dn14 = assign98930_e151256_d_n14;

        let (assign98940_e151271, assign98940_e151271_d_n0, assign98940_e151271_d_n2, assign98940_e151271_d_n4, assign98940_e151271_d_n5, assign98940_e151271_d_n6, assign98940_e151271_d_n7, assign98940_e151271_d_n8, assign98940_e151271_d_n9, assign98940_e151271_d_n10, assign98940_e151271_d_n11, assign98940_e151271_d_n14,) = {
    if ((locals.var_guard2289 != 0.0) && (locals.var_guard2290 == 0.0)) {
        let assign98940_e151265: f64 = (locals.var_vbs_jct * 0.5);
        let assign98940_e151267: f64 = (assign98940_e151265 * locals.var_t2);
        let assign98940_e151268: f64 = (locals.var_t1 + assign98940_e151267);
        let assign98940_e151269: f64 = (locals.var_vbs_jct * assign98940_e151268);
        (assign98940_e151269, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign98940_e151265 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign98940_e151268) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign98940_e151265 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign98940_e151265 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign98940_e151265 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign98940_e151265 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign98940_e151265 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign98940_e151265 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign98940_e151265 * locals.var_t2_dn9))), (locals.var_vbs_jct * (locals.var_t1_dn10 + (assign98940_e151265 * locals.var_t2_dn10))), ((locals.var_vbs_jct_dn11 * assign98940_e151268) + (locals.var_vbs_jct * (locals.var_t1_dn11 + (((locals.var_vbs_jct_dn11 * 0.5) * locals.var_t2) + (assign98940_e151265 * locals.var_t2_dn11))))), (locals.var_vbs_jct * (locals.var_t1_dn14 + (assign98940_e151265 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn11, locals.var_qbs_btm_dn14,)
    }
};
        locals.var_qbs_btm = assign98940_e151271;
        locals.var_qbs_btm_dn0 = assign98940_e151271_d_n0;
        locals.var_qbs_btm_dn2 = assign98940_e151271_d_n2;
        locals.var_qbs_btm_dn4 = assign98940_e151271_d_n4;
        locals.var_qbs_btm_dn5 = assign98940_e151271_d_n5;
        locals.var_qbs_btm_dn6 = assign98940_e151271_d_n6;
        locals.var_qbs_btm_dn7 = assign98940_e151271_d_n7;
        locals.var_qbs_btm_dn8 = assign98940_e151271_d_n8;
        locals.var_qbs_btm_dn9 = assign98940_e151271_d_n9;
        locals.var_qbs_btm_dn10 = assign98940_e151271_d_n10;
        locals.var_qbs_btm_dn11 = assign98940_e151271_d_n11;
        locals.var_qbs_btm_dn14 = assign98940_e151271_d_n14;

    }

    pub(super) fn stamp_transient_block_364(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98960_e151287, assign98960_e151287_d_n0, assign98960_e151287_d_n2, assign98960_e151287_d_n4, assign98960_e151287_d_n5, assign98960_e151287_d_n6, assign98960_e151287_d_n7, assign98960_e151287_d_n8, assign98960_e151287_d_n9, assign98960_e151287_d_n10, assign98960_e151287_d_n11, assign98960_e151287_d_n14,) = {
    if (locals.var_guard2289 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn11, locals.var_qbs_btm_dn14,)
    }
};
        locals.var_qbs_btm = assign98960_e151287;
        locals.var_qbs_btm_dn0 = assign98960_e151287_d_n0;
        locals.var_qbs_btm_dn2 = assign98960_e151287_d_n2;
        locals.var_qbs_btm_dn4 = assign98960_e151287_d_n4;
        locals.var_qbs_btm_dn5 = assign98960_e151287_d_n5;
        locals.var_qbs_btm_dn6 = assign98960_e151287_d_n6;
        locals.var_qbs_btm_dn7 = assign98960_e151287_d_n7;
        locals.var_qbs_btm_dn8 = assign98960_e151287_d_n8;
        locals.var_qbs_btm_dn9 = assign98960_e151287_d_n9;
        locals.var_qbs_btm_dn10 = assign98960_e151287_d_n10;
        locals.var_qbs_btm_dn11 = assign98960_e151287_d_n11;
        locals.var_qbs_btm_dn14 = assign98960_e151287_d_n14;

        let assign98980_e151295: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2292 = assign98980_e151295;

        let assign98990_e151298: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2293 = assign98990_e151298;

        let (assign99000_e151308, assign99000_e151308_d_n0, assign99000_e151308_d_n2, assign99000_e151308_d_n4, assign99000_e151308_d_n5, assign99000_e151308_d_n6, assign99000_e151308_d_n7, assign99000_e151308_d_n8, assign99000_e151308_d_n9, assign99000_e151308_d_n10, assign99000_e151308_d_n11, assign99000_e151308_d_n14,) = {
    if ((locals.var_guard2292 != 0.0) && (locals.var_guard2293 != 0.0)) {
        let assign99000_e151305: f64 = (locals.var_vbs_jct / locals.var_pzbssw);
        let assign99000_e151306: f64 = (1.0 - assign99000_e151305);
        (assign99000_e151306, (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn0) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbssw) - (locals.var_vbs_jct * locals.var_pzbssw_dn2)) / (locals.var_pzbssw * locals.var_pzbssw))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn4) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn5) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn6) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn7) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn8) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn9) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn10) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(((locals.var_vbs_jct_dn11 * locals.var_pzbssw) - (locals.var_vbs_jct * locals.var_pzbssw_dn11)) / (locals.var_pzbssw * locals.var_pzbssw))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn14) / (locals.var_pzbssw * locals.var_pzbssw)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign99000_e151308;
        locals.var_arg_dn0 = assign99000_e151308_d_n0;
        locals.var_arg_dn2 = assign99000_e151308_d_n2;
        locals.var_arg_dn4 = assign99000_e151308_d_n4;
        locals.var_arg_dn5 = assign99000_e151308_d_n5;
        locals.var_arg_dn6 = assign99000_e151308_d_n6;
        locals.var_arg_dn7 = assign99000_e151308_d_n7;
        locals.var_arg_dn8 = assign99000_e151308_d_n8;
        locals.var_arg_dn9 = assign99000_e151308_d_n9;
        locals.var_arg_dn10 = assign99000_e151308_d_n10;
        locals.var_arg_dn11 = assign99000_e151308_d_n11;
        locals.var_arg_dn14 = assign99000_e151308_d_n14;

        let assign99010_e151311: f64 = if p.p527 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2294 = assign99010_e151311;

        let (assign99020_e151322, assign99020_e151322_d_n0, assign99020_e151322_d_n2, assign99020_e151322_d_n4, assign99020_e151322_d_n5, assign99020_e151322_d_n6, assign99020_e151322_d_n7, assign99020_e151322_d_n8, assign99020_e151322_d_n9, assign99020_e151322_d_n10, assign99020_e151322_d_n11, assign99020_e151322_d_n14,) = {
    if (((locals.var_guard2292 != 0.0) && (locals.var_guard2293 != 0.0)) && (locals.var_guard2294 != 0.0)) {
        let assign99020_e151319: f64 = (locals.var_arg).sqrt();
        let assign99020_e151320: f64 = (1.0 / assign99020_e151319);
        (assign99020_e151320, (-((locals.var_arg_dn0 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))), (-((locals.var_arg_dn2 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))), (-((locals.var_arg_dn4 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))), (-((locals.var_arg_dn5 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))), (-((locals.var_arg_dn6 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))), (-((locals.var_arg_dn7 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))), (-((locals.var_arg_dn8 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))), (-((locals.var_arg_dn9 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))), (-((locals.var_arg_dn10 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))), (-((locals.var_arg_dn11 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))), (-((locals.var_arg_dn14 / (2.0 * assign99020_e151319)) / (assign99020_e151319 * assign99020_e151319))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99020_e151322;
        locals.var_sarg_dn0 = assign99020_e151322_d_n0;
        locals.var_sarg_dn2 = assign99020_e151322_d_n2;
        locals.var_sarg_dn4 = assign99020_e151322_d_n4;
        locals.var_sarg_dn5 = assign99020_e151322_d_n5;
        locals.var_sarg_dn6 = assign99020_e151322_d_n6;
        locals.var_sarg_dn7 = assign99020_e151322_d_n7;
        locals.var_sarg_dn8 = assign99020_e151322_d_n8;
        locals.var_sarg_dn9 = assign99020_e151322_d_n9;
        locals.var_sarg_dn10 = assign99020_e151322_d_n10;
        locals.var_sarg_dn11 = assign99020_e151322_d_n11;
        locals.var_sarg_dn14 = assign99020_e151322_d_n14;

        let (assign99030_e151339, assign99030_e151339_d_n0, assign99030_e151339_d_n2, assign99030_e151339_d_n4, assign99030_e151339_d_n5, assign99030_e151339_d_n6, assign99030_e151339_d_n7, assign99030_e151339_d_n8, assign99030_e151339_d_n9, assign99030_e151339_d_n10, assign99030_e151339_d_n11, assign99030_e151339_d_n14,) = {
    if (((locals.var_guard2292 != 0.0) && (locals.var_guard2293 != 0.0)) && (locals.var_guard2294 == 0.0)) {
        let (assign99030_e151337, assign99030_e151337_d_n0, assign99030_e151337_d_n2, assign99030_e151337_d_n4, assign99030_e151337_d_n5, assign99030_e151337_d_n6, assign99030_e151337_d_n7, assign99030_e151337_d_n8, assign99030_e151337_d_n9, assign99030_e151337_d_n10, assign99030_e151337_d_n11, assign99030_e151337_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99030_e151335: f64 = (-p.p527);
                let assign99030_e151336: f64 = (locals.var_arg).powf(assign99030_e151335);
                (assign99030_e151336, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn0)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn2)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn4)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn5)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn6)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn7)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn8)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn9)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn10)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn11)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99030_e151335) as f64).is_finite() && ((assign99030_e151335) as f64).fract() == 0.0 { if assign99030_e151335 == 0.0 { 0.0 } else { (assign99030_e151335 * ((locals.var_arg).powf(assign99030_e151335 - 1.0) * locals.var_arg_dn14)) } } else { (assign99030_e151336 * (assign99030_e151335 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign99030_e151337, assign99030_e151337_d_n0, assign99030_e151337_d_n2, assign99030_e151337_d_n4, assign99030_e151337_d_n5, assign99030_e151337_d_n6, assign99030_e151337_d_n7, assign99030_e151337_d_n8, assign99030_e151337_d_n9, assign99030_e151337_d_n10, assign99030_e151337_d_n11, assign99030_e151337_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99030_e151339;
        locals.var_sarg_dn0 = assign99030_e151339_d_n0;
        locals.var_sarg_dn2 = assign99030_e151339_d_n2;
        locals.var_sarg_dn4 = assign99030_e151339_d_n4;
        locals.var_sarg_dn5 = assign99030_e151339_d_n5;
        locals.var_sarg_dn6 = assign99030_e151339_d_n6;
        locals.var_sarg_dn7 = assign99030_e151339_d_n7;
        locals.var_sarg_dn8 = assign99030_e151339_d_n8;
        locals.var_sarg_dn9 = assign99030_e151339_d_n9;
        locals.var_sarg_dn10 = assign99030_e151339_d_n10;
        locals.var_sarg_dn11 = assign99030_e151339_d_n11;
        locals.var_sarg_dn14 = assign99030_e151339_d_n14;

        let (assign99040_e151357, assign99040_e151357_d_n0, assign99040_e151357_d_n2, assign99040_e151357_d_n4, assign99040_e151357_d_n5, assign99040_e151357_d_n6, assign99040_e151357_d_n7, assign99040_e151357_d_n8, assign99040_e151357_d_n9, assign99040_e151357_d_n10, assign99040_e151357_d_n11, assign99040_e151357_d_n14,) = {
    if ((locals.var_guard2292 != 0.0) && (locals.var_guard2293 != 0.0)) {
        let assign99040_e151345: f64 = (locals.var_pzbssw * locals.var_czbssw);
        let assign99040_e151349: f64 = (locals.var_arg * locals.var_sarg);
        let assign99040_e151350: f64 = (1.0 - assign99040_e151349);
        let assign99040_e151351: f64 = (assign99040_e151345 * assign99040_e151350);
        let assign99040_e151354: f64 = (1.0 - p.p527);
        let assign99040_e151355: f64 = (assign99040_e151351 / assign99040_e151354);
        (assign99040_e151355, (((((locals.var_pzbssw_dn0 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn0)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99040_e151354), (((((locals.var_pzbssw_dn2 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn2)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99040_e151354), (((((locals.var_pzbssw_dn4 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn4)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99040_e151354), (((((locals.var_pzbssw_dn5 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn5)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99040_e151354), (((((locals.var_pzbssw_dn6 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn6)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99040_e151354), (((((locals.var_pzbssw_dn7 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn7)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99040_e151354), (((((locals.var_pzbssw_dn8 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn8)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99040_e151354), (((((locals.var_pzbssw_dn9 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn9)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99040_e151354), (((((locals.var_pzbssw_dn10 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn10)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99040_e151354), (((((locals.var_pzbssw_dn11 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn11)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign99040_e151354), (((((locals.var_pzbssw_dn14 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn14)) * assign99040_e151350) + (assign99040_e151345 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign99040_e151354),)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn11, locals.var_qbs_sws_dn14,)
    }
};
        locals.var_qbs_sws = assign99040_e151357;
        locals.var_qbs_sws_dn0 = assign99040_e151357_d_n0;
        locals.var_qbs_sws_dn2 = assign99040_e151357_d_n2;
        locals.var_qbs_sws_dn4 = assign99040_e151357_d_n4;
        locals.var_qbs_sws_dn5 = assign99040_e151357_d_n5;
        locals.var_qbs_sws_dn6 = assign99040_e151357_d_n6;
        locals.var_qbs_sws_dn7 = assign99040_e151357_d_n7;
        locals.var_qbs_sws_dn8 = assign99040_e151357_d_n8;
        locals.var_qbs_sws_dn9 = assign99040_e151357_d_n9;
        locals.var_qbs_sws_dn10 = assign99040_e151357_d_n10;
        locals.var_qbs_sws_dn11 = assign99040_e151357_d_n11;
        locals.var_qbs_sws_dn14 = assign99040_e151357_d_n14;

        let (assign99060_e151372, assign99060_e151372_d_n0, assign99060_e151372_d_n2, assign99060_e151372_d_n4, assign99060_e151372_d_n5, assign99060_e151372_d_n6, assign99060_e151372_d_n7, assign99060_e151372_d_n8, assign99060_e151372_d_n9, assign99060_e151372_d_n10, assign99060_e151372_d_n11, assign99060_e151372_d_n14,) = {
    if ((locals.var_guard2292 != 0.0) && (locals.var_guard2293 == 0.0)) {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign99060_e151372;
        locals.var_t1_dn0 = assign99060_e151372_d_n0;
        locals.var_t1_dn2 = assign99060_e151372_d_n2;
        locals.var_t1_dn4 = assign99060_e151372_d_n4;
        locals.var_t1_dn5 = assign99060_e151372_d_n5;
        locals.var_t1_dn6 = assign99060_e151372_d_n6;
        locals.var_t1_dn7 = assign99060_e151372_d_n7;
        locals.var_t1_dn8 = assign99060_e151372_d_n8;
        locals.var_t1_dn9 = assign99060_e151372_d_n9;
        locals.var_t1_dn10 = assign99060_e151372_d_n10;
        locals.var_t1_dn11 = assign99060_e151372_d_n11;
        locals.var_t1_dn14 = assign99060_e151372_d_n14;

        let (assign99070_e151383, assign99070_e151383_d_n0, assign99070_e151383_d_n2, assign99070_e151383_d_n4, assign99070_e151383_d_n5, assign99070_e151383_d_n6, assign99070_e151383_d_n7, assign99070_e151383_d_n8, assign99070_e151383_d_n9, assign99070_e151383_d_n10, assign99070_e151383_d_n11, assign99070_e151383_d_n14,) = {
    if ((locals.var_guard2292 != 0.0) && (locals.var_guard2293 == 0.0)) {
        let assign99070_e151379: f64 = (locals.var_czbssw * p.p527);
        let assign99070_e151381: f64 = (assign99070_e151379 / locals.var_pzbssw);
        (assign99070_e151381, ((((locals.var_czbssw_dn0 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn0)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn2 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn2)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn4 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn4)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn5 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn5)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn6 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn6)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn7 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn7)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn8 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn8)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn9 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn9)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn10 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn10)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn11 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn11)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn14 * p.p527) * locals.var_pzbssw) - (assign99070_e151379 * locals.var_pzbssw_dn14)) / (locals.var_pzbssw * locals.var_pzbssw)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign99070_e151383;
        locals.var_t2_dn0 = assign99070_e151383_d_n0;
        locals.var_t2_dn2 = assign99070_e151383_d_n2;
        locals.var_t2_dn4 = assign99070_e151383_d_n4;
        locals.var_t2_dn5 = assign99070_e151383_d_n5;
        locals.var_t2_dn6 = assign99070_e151383_d_n6;
        locals.var_t2_dn7 = assign99070_e151383_d_n7;
        locals.var_t2_dn8 = assign99070_e151383_d_n8;
        locals.var_t2_dn9 = assign99070_e151383_d_n9;
        locals.var_t2_dn10 = assign99070_e151383_d_n10;
        locals.var_t2_dn11 = assign99070_e151383_d_n11;
        locals.var_t2_dn14 = assign99070_e151383_d_n14;

        let (assign99080_e151398, assign99080_e151398_d_n0, assign99080_e151398_d_n2, assign99080_e151398_d_n4, assign99080_e151398_d_n5, assign99080_e151398_d_n6, assign99080_e151398_d_n7, assign99080_e151398_d_n8, assign99080_e151398_d_n9, assign99080_e151398_d_n10, assign99080_e151398_d_n11, assign99080_e151398_d_n14,) = {
    if ((locals.var_guard2292 != 0.0) && (locals.var_guard2293 == 0.0)) {
        let assign99080_e151392: f64 = (locals.var_vbs_jct * 0.5);
        let assign99080_e151394: f64 = (assign99080_e151392 * locals.var_t2);
        let assign99080_e151395: f64 = (locals.var_t1 + assign99080_e151394);
        let assign99080_e151396: f64 = (locals.var_vbs_jct * assign99080_e151395);
        (assign99080_e151396, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign99080_e151392 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign99080_e151395) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign99080_e151392 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign99080_e151392 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign99080_e151392 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign99080_e151392 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign99080_e151392 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign99080_e151392 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign99080_e151392 * locals.var_t2_dn9))), (locals.var_vbs_jct * (locals.var_t1_dn10 + (assign99080_e151392 * locals.var_t2_dn10))), ((locals.var_vbs_jct_dn11 * assign99080_e151395) + (locals.var_vbs_jct * (locals.var_t1_dn11 + (((locals.var_vbs_jct_dn11 * 0.5) * locals.var_t2) + (assign99080_e151392 * locals.var_t2_dn11))))), (locals.var_vbs_jct * (locals.var_t1_dn14 + (assign99080_e151392 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn11, locals.var_qbs_sws_dn14,)
    }
};
        locals.var_qbs_sws = assign99080_e151398;
        locals.var_qbs_sws_dn0 = assign99080_e151398_d_n0;
        locals.var_qbs_sws_dn2 = assign99080_e151398_d_n2;
        locals.var_qbs_sws_dn4 = assign99080_e151398_d_n4;
        locals.var_qbs_sws_dn5 = assign99080_e151398_d_n5;
        locals.var_qbs_sws_dn6 = assign99080_e151398_d_n6;
        locals.var_qbs_sws_dn7 = assign99080_e151398_d_n7;
        locals.var_qbs_sws_dn8 = assign99080_e151398_d_n8;
        locals.var_qbs_sws_dn9 = assign99080_e151398_d_n9;
        locals.var_qbs_sws_dn10 = assign99080_e151398_d_n10;
        locals.var_qbs_sws_dn11 = assign99080_e151398_d_n11;
        locals.var_qbs_sws_dn14 = assign99080_e151398_d_n14;

        let (assign99100_e151414, assign99100_e151414_d_n0, assign99100_e151414_d_n2, assign99100_e151414_d_n4, assign99100_e151414_d_n5, assign99100_e151414_d_n6, assign99100_e151414_d_n7, assign99100_e151414_d_n8, assign99100_e151414_d_n9, assign99100_e151414_d_n10, assign99100_e151414_d_n11, assign99100_e151414_d_n14,) = {
    if (locals.var_guard2292 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn11, locals.var_qbs_sws_dn14,)
    }
};
        locals.var_qbs_sws = assign99100_e151414;
        locals.var_qbs_sws_dn0 = assign99100_e151414_d_n0;
        locals.var_qbs_sws_dn2 = assign99100_e151414_d_n2;
        locals.var_qbs_sws_dn4 = assign99100_e151414_d_n4;
        locals.var_qbs_sws_dn5 = assign99100_e151414_d_n5;
        locals.var_qbs_sws_dn6 = assign99100_e151414_d_n6;
        locals.var_qbs_sws_dn7 = assign99100_e151414_d_n7;
        locals.var_qbs_sws_dn8 = assign99100_e151414_d_n8;
        locals.var_qbs_sws_dn9 = assign99100_e151414_d_n9;
        locals.var_qbs_sws_dn10 = assign99100_e151414_d_n10;
        locals.var_qbs_sws_dn11 = assign99100_e151414_d_n11;
        locals.var_qbs_sws_dn14 = assign99100_e151414_d_n14;

        let assign99120_e151422: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2295 = assign99120_e151422;

        let assign99130_e151425: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2296 = assign99130_e151425;

        let assign99140_e151428: f64 = if locals.var_vbsi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2297 = assign99140_e151428;

        let (assign99150_e151440, assign99150_e151440_d_n0, assign99150_e151440_d_n2, assign99150_e151440_d_n4, assign99150_e151440_d_n5, assign99150_e151440_d_n6, assign99150_e151440_d_n7, assign99150_e151440_d_n8, assign99150_e151440_d_n9, assign99150_e151440_d_n10, assign99150_e151440_d_n11, assign99150_e151440_d_n14,) = {
    if (((locals.var_guard2295 != 0.0) && (locals.var_guard2296 != 0.0)) && (locals.var_guard2297 != 0.0)) {
        let assign99150_e151437: f64 = (locals.var_vbsi_jct / locals.var_pzbsswg);
        let assign99150_e151438: f64 = (1.0 - assign99150_e151437);
        (assign99150_e151438, (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn0) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn2) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn4) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn5) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn6) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn7) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbsi_jct_dn8 * locals.var_pzbsswg) - (locals.var_vbsi_jct * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(((locals.var_vbsi_jct_dn9 * locals.var_pzbsswg) - (locals.var_vbsi_jct * locals.var_pzbsswg_dn9)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn10) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn11) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn14) / (locals.var_pzbsswg * locals.var_pzbsswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign99150_e151440;
        locals.var_arg_dn0 = assign99150_e151440_d_n0;
        locals.var_arg_dn2 = assign99150_e151440_d_n2;
        locals.var_arg_dn4 = assign99150_e151440_d_n4;
        locals.var_arg_dn5 = assign99150_e151440_d_n5;
        locals.var_arg_dn6 = assign99150_e151440_d_n6;
        locals.var_arg_dn7 = assign99150_e151440_d_n7;
        locals.var_arg_dn8 = assign99150_e151440_d_n8;
        locals.var_arg_dn9 = assign99150_e151440_d_n9;
        locals.var_arg_dn10 = assign99150_e151440_d_n10;
        locals.var_arg_dn11 = assign99150_e151440_d_n11;
        locals.var_arg_dn14 = assign99150_e151440_d_n14;

        let assign99160_e151443: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2298 = assign99160_e151443;

        let (assign99170_e151456, assign99170_e151456_d_n0, assign99170_e151456_d_n2, assign99170_e151456_d_n4, assign99170_e151456_d_n5, assign99170_e151456_d_n6, assign99170_e151456_d_n7, assign99170_e151456_d_n8, assign99170_e151456_d_n9, assign99170_e151456_d_n10, assign99170_e151456_d_n11, assign99170_e151456_d_n14,) = {
    if ((((locals.var_guard2295 != 0.0) && (locals.var_guard2296 != 0.0)) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 != 0.0)) {
        let assign99170_e151453: f64 = (locals.var_arg).sqrt();
        let assign99170_e151454: f64 = (1.0 / assign99170_e151453);
        (assign99170_e151454, (-((locals.var_arg_dn0 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))), (-((locals.var_arg_dn2 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))), (-((locals.var_arg_dn4 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))), (-((locals.var_arg_dn5 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))), (-((locals.var_arg_dn6 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))), (-((locals.var_arg_dn7 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))), (-((locals.var_arg_dn8 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))), (-((locals.var_arg_dn9 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))), (-((locals.var_arg_dn10 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))), (-((locals.var_arg_dn11 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))), (-((locals.var_arg_dn14 / (2.0 * assign99170_e151453)) / (assign99170_e151453 * assign99170_e151453))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99170_e151456;
        locals.var_sarg_dn0 = assign99170_e151456_d_n0;
        locals.var_sarg_dn2 = assign99170_e151456_d_n2;
        locals.var_sarg_dn4 = assign99170_e151456_d_n4;
        locals.var_sarg_dn5 = assign99170_e151456_d_n5;
        locals.var_sarg_dn6 = assign99170_e151456_d_n6;
        locals.var_sarg_dn7 = assign99170_e151456_d_n7;
        locals.var_sarg_dn8 = assign99170_e151456_d_n8;
        locals.var_sarg_dn9 = assign99170_e151456_d_n9;
        locals.var_sarg_dn10 = assign99170_e151456_d_n10;
        locals.var_sarg_dn11 = assign99170_e151456_d_n11;
        locals.var_sarg_dn14 = assign99170_e151456_d_n14;

        let (assign99180_e151475, assign99180_e151475_d_n0, assign99180_e151475_d_n2, assign99180_e151475_d_n4, assign99180_e151475_d_n5, assign99180_e151475_d_n6, assign99180_e151475_d_n7, assign99180_e151475_d_n8, assign99180_e151475_d_n9, assign99180_e151475_d_n10, assign99180_e151475_d_n11, assign99180_e151475_d_n14,) = {
    if ((((locals.var_guard2295 != 0.0) && (locals.var_guard2296 != 0.0)) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 == 0.0)) {
        let (assign99180_e151473, assign99180_e151473_d_n0, assign99180_e151473_d_n2, assign99180_e151473_d_n4, assign99180_e151473_d_n5, assign99180_e151473_d_n6, assign99180_e151473_d_n7, assign99180_e151473_d_n8, assign99180_e151473_d_n9, assign99180_e151473_d_n10, assign99180_e151473_d_n11, assign99180_e151473_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99180_e151471: f64 = (-p.p528);
                let assign99180_e151472: f64 = (locals.var_arg).powf(assign99180_e151471);
                (assign99180_e151472, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn0)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn2)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn4)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn5)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn6)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn7)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn8)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn9)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn10)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn11)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99180_e151471) as f64).is_finite() && ((assign99180_e151471) as f64).fract() == 0.0 { if assign99180_e151471 == 0.0 { 0.0 } else { (assign99180_e151471 * ((locals.var_arg).powf(assign99180_e151471 - 1.0) * locals.var_arg_dn14)) } } else { (assign99180_e151472 * (assign99180_e151471 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign99180_e151473, assign99180_e151473_d_n0, assign99180_e151473_d_n2, assign99180_e151473_d_n4, assign99180_e151473_d_n5, assign99180_e151473_d_n6, assign99180_e151473_d_n7, assign99180_e151473_d_n8, assign99180_e151473_d_n9, assign99180_e151473_d_n10, assign99180_e151473_d_n11, assign99180_e151473_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99180_e151475;
        locals.var_sarg_dn0 = assign99180_e151475_d_n0;
        locals.var_sarg_dn2 = assign99180_e151475_d_n2;
        locals.var_sarg_dn4 = assign99180_e151475_d_n4;
        locals.var_sarg_dn5 = assign99180_e151475_d_n5;
        locals.var_sarg_dn6 = assign99180_e151475_d_n6;
        locals.var_sarg_dn7 = assign99180_e151475_d_n7;
        locals.var_sarg_dn8 = assign99180_e151475_d_n8;
        locals.var_sarg_dn9 = assign99180_e151475_d_n9;
        locals.var_sarg_dn10 = assign99180_e151475_d_n10;
        locals.var_sarg_dn11 = assign99180_e151475_d_n11;
        locals.var_sarg_dn14 = assign99180_e151475_d_n14;

        let (assign99190_e151495, assign99190_e151495_d_n0, assign99190_e151495_d_n2, assign99190_e151495_d_n4, assign99190_e151495_d_n5, assign99190_e151495_d_n6, assign99190_e151495_d_n7, assign99190_e151495_d_n8, assign99190_e151495_d_n9, assign99190_e151495_d_n10, assign99190_e151495_d_n11, assign99190_e151495_d_n14,) = {
    if (((locals.var_guard2295 != 0.0) && (locals.var_guard2296 != 0.0)) && (locals.var_guard2297 != 0.0)) {
        let assign99190_e151483: f64 = (locals.var_pzbsswg * locals.var_czbsswg);
        let assign99190_e151487: f64 = (locals.var_arg * locals.var_sarg);
        let assign99190_e151488: f64 = (1.0 - assign99190_e151487);
        let assign99190_e151489: f64 = (assign99190_e151483 * assign99190_e151488);
        let assign99190_e151492: f64 = (1.0 - p.p528);
        let assign99190_e151493: f64 = (assign99190_e151489 / assign99190_e151492);
        (assign99190_e151493, (((((locals.var_pzbsswg_dn0 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn0)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99190_e151492), (((((locals.var_pzbsswg_dn2 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn2)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99190_e151492), (((((locals.var_pzbsswg_dn4 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn4)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99190_e151492), (((((locals.var_pzbsswg_dn5 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn5)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99190_e151492), (((((locals.var_pzbsswg_dn6 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn6)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99190_e151492), (((((locals.var_pzbsswg_dn7 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn7)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99190_e151492), (((((locals.var_pzbsswg_dn8 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn8)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99190_e151492), (((((locals.var_pzbsswg_dn9 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn9)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99190_e151492), (((((locals.var_pzbsswg_dn10 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn10)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99190_e151492), (((((locals.var_pzbsswg_dn11 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn11)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign99190_e151492), (((((locals.var_pzbsswg_dn14 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn14)) * assign99190_e151488) + (assign99190_e151483 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign99190_e151492),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99190_e151495;
        locals.var_qbs_swg_dn0 = assign99190_e151495_d_n0;
        locals.var_qbs_swg_dn2 = assign99190_e151495_d_n2;
        locals.var_qbs_swg_dn4 = assign99190_e151495_d_n4;
        locals.var_qbs_swg_dn5 = assign99190_e151495_d_n5;
        locals.var_qbs_swg_dn6 = assign99190_e151495_d_n6;
        locals.var_qbs_swg_dn7 = assign99190_e151495_d_n7;
        locals.var_qbs_swg_dn8 = assign99190_e151495_d_n8;
        locals.var_qbs_swg_dn9 = assign99190_e151495_d_n9;
        locals.var_qbs_swg_dn10 = assign99190_e151495_d_n10;
        locals.var_qbs_swg_dn11 = assign99190_e151495_d_n11;
        locals.var_qbs_swg_dn14 = assign99190_e151495_d_n14;

        let (assign99210_e151514, assign99210_e151514_d_n0, assign99210_e151514_d_n2, assign99210_e151514_d_n4, assign99210_e151514_d_n5, assign99210_e151514_d_n6, assign99210_e151514_d_n7, assign99210_e151514_d_n8, assign99210_e151514_d_n9, assign99210_e151514_d_n10, assign99210_e151514_d_n11, assign99210_e151514_d_n14,) = {
    if (((locals.var_guard2295 != 0.0) && (locals.var_guard2296 != 0.0)) && (locals.var_guard2297 == 0.0)) {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign99210_e151514;
        locals.var_t1_dn0 = assign99210_e151514_d_n0;
        locals.var_t1_dn2 = assign99210_e151514_d_n2;
        locals.var_t1_dn4 = assign99210_e151514_d_n4;
        locals.var_t1_dn5 = assign99210_e151514_d_n5;
        locals.var_t1_dn6 = assign99210_e151514_d_n6;
        locals.var_t1_dn7 = assign99210_e151514_d_n7;
        locals.var_t1_dn8 = assign99210_e151514_d_n8;
        locals.var_t1_dn9 = assign99210_e151514_d_n9;
        locals.var_t1_dn10 = assign99210_e151514_d_n10;
        locals.var_t1_dn11 = assign99210_e151514_d_n11;
        locals.var_t1_dn14 = assign99210_e151514_d_n14;

        let (assign99220_e151527, assign99220_e151527_d_n0, assign99220_e151527_d_n2, assign99220_e151527_d_n4, assign99220_e151527_d_n5, assign99220_e151527_d_n6, assign99220_e151527_d_n7, assign99220_e151527_d_n8, assign99220_e151527_d_n9, assign99220_e151527_d_n10, assign99220_e151527_d_n11, assign99220_e151527_d_n14,) = {
    if (((locals.var_guard2295 != 0.0) && (locals.var_guard2296 != 0.0)) && (locals.var_guard2297 == 0.0)) {
        let assign99220_e151523: f64 = (locals.var_czbsswg * p.p528);
        let assign99220_e151525: f64 = (assign99220_e151523 / locals.var_pzbsswg);
        (assign99220_e151525, ((((locals.var_czbsswg_dn0 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn0)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn2 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn4 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn4)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn5 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn5)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn6 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn6)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn7 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn8 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn9 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn9)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn10 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn10)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn11 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn11)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn14 * p.p528) * locals.var_pzbsswg) - (assign99220_e151523 * locals.var_pzbsswg_dn14)) / (locals.var_pzbsswg * locals.var_pzbsswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign99220_e151527;
        locals.var_t2_dn0 = assign99220_e151527_d_n0;
        locals.var_t2_dn2 = assign99220_e151527_d_n2;
        locals.var_t2_dn4 = assign99220_e151527_d_n4;
        locals.var_t2_dn5 = assign99220_e151527_d_n5;
        locals.var_t2_dn6 = assign99220_e151527_d_n6;
        locals.var_t2_dn7 = assign99220_e151527_d_n7;
        locals.var_t2_dn8 = assign99220_e151527_d_n8;
        locals.var_t2_dn9 = assign99220_e151527_d_n9;
        locals.var_t2_dn10 = assign99220_e151527_d_n10;
        locals.var_t2_dn11 = assign99220_e151527_d_n11;
        locals.var_t2_dn14 = assign99220_e151527_d_n14;

        let (assign99230_e151544, assign99230_e151544_d_n0, assign99230_e151544_d_n2, assign99230_e151544_d_n4, assign99230_e151544_d_n5, assign99230_e151544_d_n6, assign99230_e151544_d_n7, assign99230_e151544_d_n8, assign99230_e151544_d_n9, assign99230_e151544_d_n10, assign99230_e151544_d_n11, assign99230_e151544_d_n14,) = {
    if (((locals.var_guard2295 != 0.0) && (locals.var_guard2296 != 0.0)) && (locals.var_guard2297 == 0.0)) {
        let assign99230_e151538: f64 = (locals.var_vbsi_jct * 0.5);
        let assign99230_e151540: f64 = (assign99230_e151538 * locals.var_t2);
        let assign99230_e151541: f64 = (locals.var_t1 + assign99230_e151540);
        let assign99230_e151542: f64 = (locals.var_vbsi_jct * assign99230_e151541);
        (assign99230_e151542, (locals.var_vbsi_jct * (locals.var_t1_dn0 + (assign99230_e151538 * locals.var_t2_dn0))), (locals.var_vbsi_jct * (locals.var_t1_dn2 + (assign99230_e151538 * locals.var_t2_dn2))), (locals.var_vbsi_jct * (locals.var_t1_dn4 + (assign99230_e151538 * locals.var_t2_dn4))), (locals.var_vbsi_jct * (locals.var_t1_dn5 + (assign99230_e151538 * locals.var_t2_dn5))), (locals.var_vbsi_jct * (locals.var_t1_dn6 + (assign99230_e151538 * locals.var_t2_dn6))), (locals.var_vbsi_jct * (locals.var_t1_dn7 + (assign99230_e151538 * locals.var_t2_dn7))), ((locals.var_vbsi_jct_dn8 * assign99230_e151541) + (locals.var_vbsi_jct * (locals.var_t1_dn8 + (((locals.var_vbsi_jct_dn8 * 0.5) * locals.var_t2) + (assign99230_e151538 * locals.var_t2_dn8))))), ((locals.var_vbsi_jct_dn9 * assign99230_e151541) + (locals.var_vbsi_jct * (locals.var_t1_dn9 + (((locals.var_vbsi_jct_dn9 * 0.5) * locals.var_t2) + (assign99230_e151538 * locals.var_t2_dn9))))), (locals.var_vbsi_jct * (locals.var_t1_dn10 + (assign99230_e151538 * locals.var_t2_dn10))), (locals.var_vbsi_jct * (locals.var_t1_dn11 + (assign99230_e151538 * locals.var_t2_dn11))), (locals.var_vbsi_jct * (locals.var_t1_dn14 + (assign99230_e151538 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99230_e151544;
        locals.var_qbs_swg_dn0 = assign99230_e151544_d_n0;
        locals.var_qbs_swg_dn2 = assign99230_e151544_d_n2;
        locals.var_qbs_swg_dn4 = assign99230_e151544_d_n4;
        locals.var_qbs_swg_dn5 = assign99230_e151544_d_n5;
        locals.var_qbs_swg_dn6 = assign99230_e151544_d_n6;
        locals.var_qbs_swg_dn7 = assign99230_e151544_d_n7;
        locals.var_qbs_swg_dn8 = assign99230_e151544_d_n8;
        locals.var_qbs_swg_dn9 = assign99230_e151544_d_n9;
        locals.var_qbs_swg_dn10 = assign99230_e151544_d_n10;
        locals.var_qbs_swg_dn11 = assign99230_e151544_d_n11;
        locals.var_qbs_swg_dn14 = assign99230_e151544_d_n14;

        let (assign99250_e151564, assign99250_e151564_d_n0, assign99250_e151564_d_n2, assign99250_e151564_d_n4, assign99250_e151564_d_n5, assign99250_e151564_d_n6, assign99250_e151564_d_n7, assign99250_e151564_d_n8, assign99250_e151564_d_n9, assign99250_e151564_d_n10, assign99250_e151564_d_n11, assign99250_e151564_d_n14,) = {
    if ((locals.var_guard2295 != 0.0) && (locals.var_guard2296 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99250_e151564;
        locals.var_qbs_swg_dn0 = assign99250_e151564_d_n0;
        locals.var_qbs_swg_dn2 = assign99250_e151564_d_n2;
        locals.var_qbs_swg_dn4 = assign99250_e151564_d_n4;
        locals.var_qbs_swg_dn5 = assign99250_e151564_d_n5;
        locals.var_qbs_swg_dn6 = assign99250_e151564_d_n6;
        locals.var_qbs_swg_dn7 = assign99250_e151564_d_n7;
        locals.var_qbs_swg_dn8 = assign99250_e151564_d_n8;
        locals.var_qbs_swg_dn9 = assign99250_e151564_d_n9;
        locals.var_qbs_swg_dn10 = assign99250_e151564_d_n10;
        locals.var_qbs_swg_dn11 = assign99250_e151564_d_n11;
        locals.var_qbs_swg_dn14 = assign99250_e151564_d_n14;

        let assign99270_e151574: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2299 = assign99270_e151574;

        let assign99280_e151577: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2300 = assign99280_e151577;

        let (assign99290_e151590, assign99290_e151590_d_n0, assign99290_e151590_d_n2, assign99290_e151590_d_n4, assign99290_e151590_d_n5, assign99290_e151590_d_n6, assign99290_e151590_d_n7, assign99290_e151590_d_n8, assign99290_e151590_d_n9, assign99290_e151590_d_n10, assign99290_e151590_d_n11, assign99290_e151590_d_n14,) = {
    if (((locals.var_guard2295 == 0.0) && (locals.var_guard2299 != 0.0)) && (locals.var_guard2300 != 0.0)) {
        let assign99290_e151587: f64 = (locals.var_vbs_jct / locals.var_pzbsswg);
        let assign99290_e151588: f64 = (1.0 - assign99290_e151587);
        (assign99290_e151588, (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn0) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbsswg) - (locals.var_vbs_jct * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn4) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn5) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn6) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn7) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn8) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn9) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn10) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbs_jct_dn11 * locals.var_pzbsswg) - (locals.var_vbs_jct * locals.var_pzbsswg_dn11)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn14) / (locals.var_pzbsswg * locals.var_pzbsswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign99290_e151590;
        locals.var_arg_dn0 = assign99290_e151590_d_n0;
        locals.var_arg_dn2 = assign99290_e151590_d_n2;
        locals.var_arg_dn4 = assign99290_e151590_d_n4;
        locals.var_arg_dn5 = assign99290_e151590_d_n5;
        locals.var_arg_dn6 = assign99290_e151590_d_n6;
        locals.var_arg_dn7 = assign99290_e151590_d_n7;
        locals.var_arg_dn8 = assign99290_e151590_d_n8;
        locals.var_arg_dn9 = assign99290_e151590_d_n9;
        locals.var_arg_dn10 = assign99290_e151590_d_n10;
        locals.var_arg_dn11 = assign99290_e151590_d_n11;
        locals.var_arg_dn14 = assign99290_e151590_d_n14;

        let assign99300_e151593: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2301 = assign99300_e151593;

        let (assign99310_e151607, assign99310_e151607_d_n0, assign99310_e151607_d_n2, assign99310_e151607_d_n4, assign99310_e151607_d_n5, assign99310_e151607_d_n6, assign99310_e151607_d_n7, assign99310_e151607_d_n8, assign99310_e151607_d_n9, assign99310_e151607_d_n10, assign99310_e151607_d_n11, assign99310_e151607_d_n14,) = {
    if ((((locals.var_guard2295 == 0.0) && (locals.var_guard2299 != 0.0)) && (locals.var_guard2300 != 0.0)) && (locals.var_guard2301 != 0.0)) {
        let assign99310_e151604: f64 = (locals.var_arg).sqrt();
        let assign99310_e151605: f64 = (1.0 / assign99310_e151604);
        (assign99310_e151605, (-((locals.var_arg_dn0 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))), (-((locals.var_arg_dn2 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))), (-((locals.var_arg_dn4 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))), (-((locals.var_arg_dn5 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))), (-((locals.var_arg_dn6 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))), (-((locals.var_arg_dn7 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))), (-((locals.var_arg_dn8 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))), (-((locals.var_arg_dn9 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))), (-((locals.var_arg_dn10 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))), (-((locals.var_arg_dn11 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))), (-((locals.var_arg_dn14 / (2.0 * assign99310_e151604)) / (assign99310_e151604 * assign99310_e151604))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99310_e151607;
        locals.var_sarg_dn0 = assign99310_e151607_d_n0;
        locals.var_sarg_dn2 = assign99310_e151607_d_n2;
        locals.var_sarg_dn4 = assign99310_e151607_d_n4;
        locals.var_sarg_dn5 = assign99310_e151607_d_n5;
        locals.var_sarg_dn6 = assign99310_e151607_d_n6;
        locals.var_sarg_dn7 = assign99310_e151607_d_n7;
        locals.var_sarg_dn8 = assign99310_e151607_d_n8;
        locals.var_sarg_dn9 = assign99310_e151607_d_n9;
        locals.var_sarg_dn10 = assign99310_e151607_d_n10;
        locals.var_sarg_dn11 = assign99310_e151607_d_n11;
        locals.var_sarg_dn14 = assign99310_e151607_d_n14;

        let (assign99320_e151627, assign99320_e151627_d_n0, assign99320_e151627_d_n2, assign99320_e151627_d_n4, assign99320_e151627_d_n5, assign99320_e151627_d_n6, assign99320_e151627_d_n7, assign99320_e151627_d_n8, assign99320_e151627_d_n9, assign99320_e151627_d_n10, assign99320_e151627_d_n11, assign99320_e151627_d_n14,) = {
    if ((((locals.var_guard2295 == 0.0) && (locals.var_guard2299 != 0.0)) && (locals.var_guard2300 != 0.0)) && (locals.var_guard2301 == 0.0)) {
        let (assign99320_e151625, assign99320_e151625_d_n0, assign99320_e151625_d_n2, assign99320_e151625_d_n4, assign99320_e151625_d_n5, assign99320_e151625_d_n6, assign99320_e151625_d_n7, assign99320_e151625_d_n8, assign99320_e151625_d_n9, assign99320_e151625_d_n10, assign99320_e151625_d_n11, assign99320_e151625_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99320_e151623: f64 = (-p.p528);
                let assign99320_e151624: f64 = (locals.var_arg).powf(assign99320_e151623);
                (assign99320_e151624, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn0)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn2)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn4)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn5)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn6)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn7)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn8)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn9)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn10)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn11)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99320_e151623) as f64).is_finite() && ((assign99320_e151623) as f64).fract() == 0.0 { if assign99320_e151623 == 0.0 { 0.0 } else { (assign99320_e151623 * ((locals.var_arg).powf(assign99320_e151623 - 1.0) * locals.var_arg_dn14)) } } else { (assign99320_e151624 * (assign99320_e151623 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign99320_e151625, assign99320_e151625_d_n0, assign99320_e151625_d_n2, assign99320_e151625_d_n4, assign99320_e151625_d_n5, assign99320_e151625_d_n6, assign99320_e151625_d_n7, assign99320_e151625_d_n8, assign99320_e151625_d_n9, assign99320_e151625_d_n10, assign99320_e151625_d_n11, assign99320_e151625_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99320_e151627;
        locals.var_sarg_dn0 = assign99320_e151627_d_n0;
        locals.var_sarg_dn2 = assign99320_e151627_d_n2;
        locals.var_sarg_dn4 = assign99320_e151627_d_n4;
        locals.var_sarg_dn5 = assign99320_e151627_d_n5;
        locals.var_sarg_dn6 = assign99320_e151627_d_n6;
        locals.var_sarg_dn7 = assign99320_e151627_d_n7;
        locals.var_sarg_dn8 = assign99320_e151627_d_n8;
        locals.var_sarg_dn9 = assign99320_e151627_d_n9;
        locals.var_sarg_dn10 = assign99320_e151627_d_n10;
        locals.var_sarg_dn11 = assign99320_e151627_d_n11;
        locals.var_sarg_dn14 = assign99320_e151627_d_n14;

    }

    pub(super) fn stamp_transient_block_365(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign99330_e151648, assign99330_e151648_d_n0, assign99330_e151648_d_n2, assign99330_e151648_d_n4, assign99330_e151648_d_n5, assign99330_e151648_d_n6, assign99330_e151648_d_n7, assign99330_e151648_d_n8, assign99330_e151648_d_n9, assign99330_e151648_d_n10, assign99330_e151648_d_n11, assign99330_e151648_d_n14,) = {
    if (((locals.var_guard2295 == 0.0) && (locals.var_guard2299 != 0.0)) && (locals.var_guard2300 != 0.0)) {
        let assign99330_e151636: f64 = (locals.var_pzbsswg * locals.var_czbsswg);
        let assign99330_e151640: f64 = (locals.var_arg * locals.var_sarg);
        let assign99330_e151641: f64 = (1.0 - assign99330_e151640);
        let assign99330_e151642: f64 = (assign99330_e151636 * assign99330_e151641);
        let assign99330_e151645: f64 = (1.0 - p.p528);
        let assign99330_e151646: f64 = (assign99330_e151642 / assign99330_e151645);
        (assign99330_e151646, (((((locals.var_pzbsswg_dn0 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn0)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99330_e151645), (((((locals.var_pzbsswg_dn2 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn2)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99330_e151645), (((((locals.var_pzbsswg_dn4 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn4)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99330_e151645), (((((locals.var_pzbsswg_dn5 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn5)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99330_e151645), (((((locals.var_pzbsswg_dn6 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn6)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99330_e151645), (((((locals.var_pzbsswg_dn7 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn7)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99330_e151645), (((((locals.var_pzbsswg_dn8 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn8)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99330_e151645), (((((locals.var_pzbsswg_dn9 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn9)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99330_e151645), (((((locals.var_pzbsswg_dn10 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn10)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99330_e151645), (((((locals.var_pzbsswg_dn11 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn11)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign99330_e151645), (((((locals.var_pzbsswg_dn14 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn14)) * assign99330_e151641) + (assign99330_e151636 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign99330_e151645),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99330_e151648;
        locals.var_qbs_swg_dn0 = assign99330_e151648_d_n0;
        locals.var_qbs_swg_dn2 = assign99330_e151648_d_n2;
        locals.var_qbs_swg_dn4 = assign99330_e151648_d_n4;
        locals.var_qbs_swg_dn5 = assign99330_e151648_d_n5;
        locals.var_qbs_swg_dn6 = assign99330_e151648_d_n6;
        locals.var_qbs_swg_dn7 = assign99330_e151648_d_n7;
        locals.var_qbs_swg_dn8 = assign99330_e151648_d_n8;
        locals.var_qbs_swg_dn9 = assign99330_e151648_d_n9;
        locals.var_qbs_swg_dn10 = assign99330_e151648_d_n10;
        locals.var_qbs_swg_dn11 = assign99330_e151648_d_n11;
        locals.var_qbs_swg_dn14 = assign99330_e151648_d_n14;

        let (assign99350_e151669, assign99350_e151669_d_n0, assign99350_e151669_d_n2, assign99350_e151669_d_n4, assign99350_e151669_d_n5, assign99350_e151669_d_n6, assign99350_e151669_d_n7, assign99350_e151669_d_n8, assign99350_e151669_d_n9, assign99350_e151669_d_n10, assign99350_e151669_d_n11, assign99350_e151669_d_n14,) = {
    if (((locals.var_guard2295 == 0.0) && (locals.var_guard2299 != 0.0)) && (locals.var_guard2300 == 0.0)) {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign99350_e151669;
        locals.var_t1_dn0 = assign99350_e151669_d_n0;
        locals.var_t1_dn2 = assign99350_e151669_d_n2;
        locals.var_t1_dn4 = assign99350_e151669_d_n4;
        locals.var_t1_dn5 = assign99350_e151669_d_n5;
        locals.var_t1_dn6 = assign99350_e151669_d_n6;
        locals.var_t1_dn7 = assign99350_e151669_d_n7;
        locals.var_t1_dn8 = assign99350_e151669_d_n8;
        locals.var_t1_dn9 = assign99350_e151669_d_n9;
        locals.var_t1_dn10 = assign99350_e151669_d_n10;
        locals.var_t1_dn11 = assign99350_e151669_d_n11;
        locals.var_t1_dn14 = assign99350_e151669_d_n14;

        let (assign99360_e151683, assign99360_e151683_d_n0, assign99360_e151683_d_n2, assign99360_e151683_d_n4, assign99360_e151683_d_n5, assign99360_e151683_d_n6, assign99360_e151683_d_n7, assign99360_e151683_d_n8, assign99360_e151683_d_n9, assign99360_e151683_d_n10, assign99360_e151683_d_n11, assign99360_e151683_d_n14,) = {
    if (((locals.var_guard2295 == 0.0) && (locals.var_guard2299 != 0.0)) && (locals.var_guard2300 == 0.0)) {
        let assign99360_e151679: f64 = (locals.var_czbsswg * p.p528);
        let assign99360_e151681: f64 = (assign99360_e151679 / locals.var_pzbsswg);
        (assign99360_e151681, ((((locals.var_czbsswg_dn0 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn0)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn2 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn4 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn4)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn5 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn5)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn6 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn6)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn7 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn8 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn9 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn9)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn10 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn10)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn11 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn11)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn14 * p.p528) * locals.var_pzbsswg) - (assign99360_e151679 * locals.var_pzbsswg_dn14)) / (locals.var_pzbsswg * locals.var_pzbsswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign99360_e151683;
        locals.var_t2_dn0 = assign99360_e151683_d_n0;
        locals.var_t2_dn2 = assign99360_e151683_d_n2;
        locals.var_t2_dn4 = assign99360_e151683_d_n4;
        locals.var_t2_dn5 = assign99360_e151683_d_n5;
        locals.var_t2_dn6 = assign99360_e151683_d_n6;
        locals.var_t2_dn7 = assign99360_e151683_d_n7;
        locals.var_t2_dn8 = assign99360_e151683_d_n8;
        locals.var_t2_dn9 = assign99360_e151683_d_n9;
        locals.var_t2_dn10 = assign99360_e151683_d_n10;
        locals.var_t2_dn11 = assign99360_e151683_d_n11;
        locals.var_t2_dn14 = assign99360_e151683_d_n14;

        let (assign99370_e151701, assign99370_e151701_d_n0, assign99370_e151701_d_n2, assign99370_e151701_d_n4, assign99370_e151701_d_n5, assign99370_e151701_d_n6, assign99370_e151701_d_n7, assign99370_e151701_d_n8, assign99370_e151701_d_n9, assign99370_e151701_d_n10, assign99370_e151701_d_n11, assign99370_e151701_d_n14,) = {
    if (((locals.var_guard2295 == 0.0) && (locals.var_guard2299 != 0.0)) && (locals.var_guard2300 == 0.0)) {
        let assign99370_e151695: f64 = (locals.var_vbs_jct * 0.5);
        let assign99370_e151697: f64 = (assign99370_e151695 * locals.var_t2);
        let assign99370_e151698: f64 = (locals.var_t1 + assign99370_e151697);
        let assign99370_e151699: f64 = (locals.var_vbs_jct * assign99370_e151698);
        (assign99370_e151699, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign99370_e151695 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign99370_e151698) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign99370_e151695 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign99370_e151695 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign99370_e151695 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign99370_e151695 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign99370_e151695 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign99370_e151695 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign99370_e151695 * locals.var_t2_dn9))), (locals.var_vbs_jct * (locals.var_t1_dn10 + (assign99370_e151695 * locals.var_t2_dn10))), ((locals.var_vbs_jct_dn11 * assign99370_e151698) + (locals.var_vbs_jct * (locals.var_t1_dn11 + (((locals.var_vbs_jct_dn11 * 0.5) * locals.var_t2) + (assign99370_e151695 * locals.var_t2_dn11))))), (locals.var_vbs_jct * (locals.var_t1_dn14 + (assign99370_e151695 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99370_e151701;
        locals.var_qbs_swg_dn0 = assign99370_e151701_d_n0;
        locals.var_qbs_swg_dn2 = assign99370_e151701_d_n2;
        locals.var_qbs_swg_dn4 = assign99370_e151701_d_n4;
        locals.var_qbs_swg_dn5 = assign99370_e151701_d_n5;
        locals.var_qbs_swg_dn6 = assign99370_e151701_d_n6;
        locals.var_qbs_swg_dn7 = assign99370_e151701_d_n7;
        locals.var_qbs_swg_dn8 = assign99370_e151701_d_n8;
        locals.var_qbs_swg_dn9 = assign99370_e151701_d_n9;
        locals.var_qbs_swg_dn10 = assign99370_e151701_d_n10;
        locals.var_qbs_swg_dn11 = assign99370_e151701_d_n11;
        locals.var_qbs_swg_dn14 = assign99370_e151701_d_n14;

        let (assign99390_e151723, assign99390_e151723_d_n0, assign99390_e151723_d_n2, assign99390_e151723_d_n4, assign99390_e151723_d_n5, assign99390_e151723_d_n6, assign99390_e151723_d_n7, assign99390_e151723_d_n8, assign99390_e151723_d_n9, assign99390_e151723_d_n10, assign99390_e151723_d_n11, assign99390_e151723_d_n14,) = {
    if ((locals.var_guard2295 == 0.0) && (locals.var_guard2299 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99390_e151723;
        locals.var_qbs_swg_dn0 = assign99390_e151723_d_n0;
        locals.var_qbs_swg_dn2 = assign99390_e151723_d_n2;
        locals.var_qbs_swg_dn4 = assign99390_e151723_d_n4;
        locals.var_qbs_swg_dn5 = assign99390_e151723_d_n5;
        locals.var_qbs_swg_dn6 = assign99390_e151723_d_n6;
        locals.var_qbs_swg_dn7 = assign99390_e151723_d_n7;
        locals.var_qbs_swg_dn8 = assign99390_e151723_d_n8;
        locals.var_qbs_swg_dn9 = assign99390_e151723_d_n9;
        locals.var_qbs_swg_dn10 = assign99390_e151723_d_n10;
        locals.var_qbs_swg_dn11 = assign99390_e151723_d_n11;
        locals.var_qbs_swg_dn14 = assign99390_e151723_d_n14;

        let assign99410_e151735: f64 = (locals.var_ibs_btm + locals.var_ibs_sws);
        let assign99410_e151736: f64 = (locals.var_mfactor * assign99410_e151735);
        locals.var_ibs = assign99410_e151736;
        locals.var_ibs_dn0 = (locals.var_mfactor * (locals.var_ibs_btm_dn0 + locals.var_ibs_sws_dn0));
        locals.var_ibs_dn2 = (locals.var_mfactor * (locals.var_ibs_btm_dn2 + locals.var_ibs_sws_dn2));
        locals.var_ibs_dn4 = (locals.var_mfactor * (locals.var_ibs_btm_dn4 + locals.var_ibs_sws_dn4));
        locals.var_ibs_dn5 = (locals.var_mfactor * (locals.var_ibs_btm_dn5 + locals.var_ibs_sws_dn5));
        locals.var_ibs_dn6 = (locals.var_mfactor * (locals.var_ibs_btm_dn6 + locals.var_ibs_sws_dn6));
        locals.var_ibs_dn7 = (locals.var_mfactor * (locals.var_ibs_btm_dn7 + locals.var_ibs_sws_dn7));
        locals.var_ibs_dn8 = (locals.var_mfactor * (locals.var_ibs_btm_dn8 + locals.var_ibs_sws_dn8));
        locals.var_ibs_dn9 = (locals.var_mfactor * (locals.var_ibs_btm_dn9 + locals.var_ibs_sws_dn9));
        locals.var_ibs_dn10 = (locals.var_mfactor * (locals.var_ibs_btm_dn10 + locals.var_ibs_sws_dn10));
        locals.var_ibs_dn11 = (locals.var_mfactor * (locals.var_ibs_btm_dn11 + locals.var_ibs_sws_dn11));
        locals.var_ibs_dn14 = (locals.var_mfactor * (locals.var_ibs_btm_dn14 + locals.var_ibs_sws_dn14));

        let assign99420_e151740: f64 = (locals.var_ibd_btm + locals.var_ibd_sws);
        let assign99420_e151741: f64 = (locals.var_mfactor * assign99420_e151740);
        locals.var_ibd = assign99420_e151741;
        locals.var_ibd_dn0 = (locals.var_mfactor * (locals.var_ibd_btm_dn0 + locals.var_ibd_sws_dn0));
        locals.var_ibd_dn2 = (locals.var_mfactor * (locals.var_ibd_btm_dn2 + locals.var_ibd_sws_dn2));
        locals.var_ibd_dn4 = (locals.var_mfactor * (locals.var_ibd_btm_dn4 + locals.var_ibd_sws_dn4));
        locals.var_ibd_dn5 = (locals.var_mfactor * (locals.var_ibd_btm_dn5 + locals.var_ibd_sws_dn5));
        locals.var_ibd_dn6 = (locals.var_mfactor * (locals.var_ibd_btm_dn6 + locals.var_ibd_sws_dn6));
        locals.var_ibd_dn7 = (locals.var_mfactor * (locals.var_ibd_btm_dn7 + locals.var_ibd_sws_dn7));
        locals.var_ibd_dn8 = (locals.var_mfactor * (locals.var_ibd_btm_dn8 + locals.var_ibd_sws_dn8));
        locals.var_ibd_dn9 = (locals.var_mfactor * (locals.var_ibd_btm_dn9 + locals.var_ibd_sws_dn9));
        locals.var_ibd_dn10 = (locals.var_mfactor * (locals.var_ibd_btm_dn10 + locals.var_ibd_sws_dn10));
        locals.var_ibd_dn11 = (locals.var_mfactor * (locals.var_ibd_btm_dn11 + locals.var_ibd_sws_dn11));
        locals.var_ibd_dn14 = (locals.var_mfactor * (locals.var_ibd_btm_dn14 + locals.var_ibd_sws_dn14));

        let assign99430_e151744: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2302 = assign99430_e151744;

        let (assign99440_e151750, assign99440_e151750_d_n0, assign99440_e151750_d_n2, assign99440_e151750_d_n4, assign99440_e151750_d_n5, assign99440_e151750_d_n6, assign99440_e151750_d_n7, assign99440_e151750_d_n8, assign99440_e151750_d_n9, assign99440_e151750_d_n10, assign99440_e151750_d_n11, assign99440_e151750_d_n14,) = {
    if (locals.var_guard2302 != 0.0) {
        let assign99440_e151748: f64 = (locals.var_mfactor * locals.var_ibs_swg);
        (assign99440_e151748, (locals.var_mfactor * locals.var_ibs_swg_dn0), (locals.var_mfactor * locals.var_ibs_swg_dn2), (locals.var_mfactor * locals.var_ibs_swg_dn4), (locals.var_mfactor * locals.var_ibs_swg_dn5), (locals.var_mfactor * locals.var_ibs_swg_dn6), (locals.var_mfactor * locals.var_ibs_swg_dn7), (locals.var_mfactor * locals.var_ibs_swg_dn8), (locals.var_mfactor * locals.var_ibs_swg_dn9), (locals.var_mfactor * locals.var_ibs_swg_dn10), (locals.var_mfactor * locals.var_ibs_swg_dn11), (locals.var_mfactor * locals.var_ibs_swg_dn14),)
    } else {
        (locals.var_ibsi, locals.var_ibsi_dn0, locals.var_ibsi_dn2, locals.var_ibsi_dn4, locals.var_ibsi_dn5, locals.var_ibsi_dn6, locals.var_ibsi_dn7, locals.var_ibsi_dn8, locals.var_ibsi_dn9, locals.var_ibsi_dn10, locals.var_ibsi_dn11, locals.var_ibsi_dn14,)
    }
};
        locals.var_ibsi = assign99440_e151750;
        locals.var_ibsi_dn0 = assign99440_e151750_d_n0;
        locals.var_ibsi_dn2 = assign99440_e151750_d_n2;
        locals.var_ibsi_dn4 = assign99440_e151750_d_n4;
        locals.var_ibsi_dn5 = assign99440_e151750_d_n5;
        locals.var_ibsi_dn6 = assign99440_e151750_d_n6;
        locals.var_ibsi_dn7 = assign99440_e151750_d_n7;
        locals.var_ibsi_dn8 = assign99440_e151750_d_n8;
        locals.var_ibsi_dn9 = assign99440_e151750_d_n9;
        locals.var_ibsi_dn10 = assign99440_e151750_d_n10;
        locals.var_ibsi_dn11 = assign99440_e151750_d_n11;
        locals.var_ibsi_dn14 = assign99440_e151750_d_n14;

        let (assign99460_e151764, assign99460_e151764_d_n0, assign99460_e151764_d_n2, assign99460_e151764_d_n4, assign99460_e151764_d_n5, assign99460_e151764_d_n6, assign99460_e151764_d_n7, assign99460_e151764_d_n8, assign99460_e151764_d_n9, assign99460_e151764_d_n10, assign99460_e151764_d_n11, assign99460_e151764_d_n14,) = {
    if (locals.var_guard2302 != 0.0) {
        let assign99460_e151761: f64 = (locals.var_qbs_btm + locals.var_qbs_sws);
        let assign99460_e151762: f64 = (locals.var_mfactor * assign99460_e151761);
        (assign99460_e151762, (locals.var_mfactor * (locals.var_qbs_btm_dn0 + locals.var_qbs_sws_dn0)), (locals.var_mfactor * (locals.var_qbs_btm_dn2 + locals.var_qbs_sws_dn2)), (locals.var_mfactor * (locals.var_qbs_btm_dn4 + locals.var_qbs_sws_dn4)), (locals.var_mfactor * (locals.var_qbs_btm_dn5 + locals.var_qbs_sws_dn5)), (locals.var_mfactor * (locals.var_qbs_btm_dn6 + locals.var_qbs_sws_dn6)), (locals.var_mfactor * (locals.var_qbs_btm_dn7 + locals.var_qbs_sws_dn7)), (locals.var_mfactor * (locals.var_qbs_btm_dn8 + locals.var_qbs_sws_dn8)), (locals.var_mfactor * (locals.var_qbs_btm_dn9 + locals.var_qbs_sws_dn9)), (locals.var_mfactor * (locals.var_qbs_btm_dn10 + locals.var_qbs_sws_dn10)), (locals.var_mfactor * (locals.var_qbs_btm_dn11 + locals.var_qbs_sws_dn11)), (locals.var_mfactor * (locals.var_qbs_btm_dn14 + locals.var_qbs_sws_dn14)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn14,)
    }
};
        locals.var_qbs = assign99460_e151764;
        locals.var_qbs_dn0 = assign99460_e151764_d_n0;
        locals.var_qbs_dn2 = assign99460_e151764_d_n2;
        locals.var_qbs_dn4 = assign99460_e151764_d_n4;
        locals.var_qbs_dn5 = assign99460_e151764_d_n5;
        locals.var_qbs_dn6 = assign99460_e151764_d_n6;
        locals.var_qbs_dn7 = assign99460_e151764_d_n7;
        locals.var_qbs_dn8 = assign99460_e151764_d_n8;
        locals.var_qbs_dn9 = assign99460_e151764_d_n9;
        locals.var_qbs_dn10 = assign99460_e151764_d_n10;
        locals.var_qbs_dn11 = assign99460_e151764_d_n11;
        locals.var_qbs_dn14 = assign99460_e151764_d_n14;

        let (assign99470_e151772, assign99470_e151772_d_n0, assign99470_e151772_d_n2, assign99470_e151772_d_n4, assign99470_e151772_d_n5, assign99470_e151772_d_n6, assign99470_e151772_d_n7, assign99470_e151772_d_n8, assign99470_e151772_d_n9, assign99470_e151772_d_n10, assign99470_e151772_d_n11, assign99470_e151772_d_n14, assign99470_e151772_d_n16, assign99470_e151772_d_n17, assign99470_e151772_d_n18,) = {
    if (locals.var_guard2302 != 0.0) {
        let assign99470_e151769: f64 = (locals.var_qbd_btm + locals.var_qbd_sws);
        let assign99470_e151770: f64 = (locals.var_mfactor * assign99470_e151769);
        (assign99470_e151770, (locals.var_mfactor * (locals.var_qbd_btm_dn0 + locals.var_qbd_sws_dn0)), (locals.var_mfactor * (locals.var_qbd_btm_dn2 + locals.var_qbd_sws_dn2)), (locals.var_mfactor * (locals.var_qbd_btm_dn4 + locals.var_qbd_sws_dn4)), (locals.var_mfactor * (locals.var_qbd_btm_dn5 + locals.var_qbd_sws_dn5)), (locals.var_mfactor * (locals.var_qbd_btm_dn6 + locals.var_qbd_sws_dn6)), (locals.var_mfactor * (locals.var_qbd_btm_dn7 + locals.var_qbd_sws_dn7)), (locals.var_mfactor * (locals.var_qbd_btm_dn8 + locals.var_qbd_sws_dn8)), (locals.var_mfactor * (locals.var_qbd_btm_dn9 + locals.var_qbd_sws_dn9)), (locals.var_mfactor * (locals.var_qbd_btm_dn10 + locals.var_qbd_sws_dn10)), (locals.var_mfactor * (locals.var_qbd_btm_dn11 + locals.var_qbd_sws_dn11)), (locals.var_mfactor * (locals.var_qbd_btm_dn14 + locals.var_qbd_sws_dn14)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn14, locals.var_qbd_dn16, locals.var_qbd_dn17, locals.var_qbd_dn18,)
    }
};
        locals.var_qbd = assign99470_e151772;
        locals.var_qbd_dn0 = assign99470_e151772_d_n0;
        locals.var_qbd_dn2 = assign99470_e151772_d_n2;
        locals.var_qbd_dn4 = assign99470_e151772_d_n4;
        locals.var_qbd_dn5 = assign99470_e151772_d_n5;
        locals.var_qbd_dn6 = assign99470_e151772_d_n6;
        locals.var_qbd_dn7 = assign99470_e151772_d_n7;
        locals.var_qbd_dn8 = assign99470_e151772_d_n8;
        locals.var_qbd_dn9 = assign99470_e151772_d_n9;
        locals.var_qbd_dn10 = assign99470_e151772_d_n10;
        locals.var_qbd_dn11 = assign99470_e151772_d_n11;
        locals.var_qbd_dn14 = assign99470_e151772_d_n14;
        locals.var_qbd_dn16 = assign99470_e151772_d_n16;
        locals.var_qbd_dn17 = assign99470_e151772_d_n17;
        locals.var_qbd_dn18 = assign99470_e151772_d_n18;

        let (assign99480_e151778, assign99480_e151778_d_n0, assign99480_e151778_d_n2, assign99480_e151778_d_n4, assign99480_e151778_d_n5, assign99480_e151778_d_n6, assign99480_e151778_d_n7, assign99480_e151778_d_n8, assign99480_e151778_d_n9, assign99480_e151778_d_n10, assign99480_e151778_d_n11, assign99480_e151778_d_n14,) = {
    if (locals.var_guard2302 != 0.0) {
        let assign99480_e151776: f64 = (locals.var_mfactor * locals.var_qbs_swg);
        (assign99480_e151776, (locals.var_mfactor * locals.var_qbs_swg_dn0), (locals.var_mfactor * locals.var_qbs_swg_dn2), (locals.var_mfactor * locals.var_qbs_swg_dn4), (locals.var_mfactor * locals.var_qbs_swg_dn5), (locals.var_mfactor * locals.var_qbs_swg_dn6), (locals.var_mfactor * locals.var_qbs_swg_dn7), (locals.var_mfactor * locals.var_qbs_swg_dn8), (locals.var_mfactor * locals.var_qbs_swg_dn9), (locals.var_mfactor * locals.var_qbs_swg_dn10), (locals.var_mfactor * locals.var_qbs_swg_dn11), (locals.var_mfactor * locals.var_qbs_swg_dn14),)
    } else {
        (locals.var_qbsi, locals.var_qbsi_dn0, locals.var_qbsi_dn2, locals.var_qbsi_dn4, locals.var_qbsi_dn5, locals.var_qbsi_dn6, locals.var_qbsi_dn7, locals.var_qbsi_dn8, locals.var_qbsi_dn9, locals.var_qbsi_dn10, locals.var_qbsi_dn11, locals.var_qbsi_dn14,)
    }
};
        locals.var_qbsi = assign99480_e151778;
        locals.var_qbsi_dn0 = assign99480_e151778_d_n0;
        locals.var_qbsi_dn2 = assign99480_e151778_d_n2;
        locals.var_qbsi_dn4 = assign99480_e151778_d_n4;
        locals.var_qbsi_dn5 = assign99480_e151778_d_n5;
        locals.var_qbsi_dn6 = assign99480_e151778_d_n6;
        locals.var_qbsi_dn7 = assign99480_e151778_d_n7;
        locals.var_qbsi_dn8 = assign99480_e151778_d_n8;
        locals.var_qbsi_dn9 = assign99480_e151778_d_n9;
        locals.var_qbsi_dn10 = assign99480_e151778_d_n10;
        locals.var_qbsi_dn11 = assign99480_e151778_d_n11;
        locals.var_qbsi_dn14 = assign99480_e151778_d_n14;

        let (assign99490_e151784, assign99490_e151784_d_n0, assign99490_e151784_d_n2, assign99490_e151784_d_n4, assign99490_e151784_d_n5, assign99490_e151784_d_n6, assign99490_e151784_d_n7, assign99490_e151784_d_n8, assign99490_e151784_d_n9, assign99490_e151784_d_n10, assign99490_e151784_d_n11, assign99490_e151784_d_n14,) = {
    if (locals.var_guard2302 != 0.0) {
        let assign99490_e151782: f64 = (locals.var_mfactor * locals.var_qbd_swg);
        (assign99490_e151782, (locals.var_mfactor * locals.var_qbd_swg_dn0), (locals.var_mfactor * locals.var_qbd_swg_dn2), (locals.var_mfactor * locals.var_qbd_swg_dn4), (locals.var_mfactor * locals.var_qbd_swg_dn5), (locals.var_mfactor * locals.var_qbd_swg_dn6), (locals.var_mfactor * locals.var_qbd_swg_dn7), (locals.var_mfactor * locals.var_qbd_swg_dn8), (locals.var_mfactor * locals.var_qbd_swg_dn9), (locals.var_mfactor * locals.var_qbd_swg_dn10), (locals.var_mfactor * locals.var_qbd_swg_dn11), (locals.var_mfactor * locals.var_qbd_swg_dn14),)
    } else {
        (locals.var_qbdi, locals.var_qbdi_dn0, locals.var_qbdi_dn2, locals.var_qbdi_dn4, locals.var_qbdi_dn5, locals.var_qbdi_dn6, locals.var_qbdi_dn7, locals.var_qbdi_dn8, locals.var_qbdi_dn9, locals.var_qbdi_dn10, locals.var_qbdi_dn11, locals.var_qbdi_dn14,)
    }
};
        locals.var_qbdi = assign99490_e151784;
        locals.var_qbdi_dn0 = assign99490_e151784_d_n0;
        locals.var_qbdi_dn2 = assign99490_e151784_d_n2;
        locals.var_qbdi_dn4 = assign99490_e151784_d_n4;
        locals.var_qbdi_dn5 = assign99490_e151784_d_n5;
        locals.var_qbdi_dn6 = assign99490_e151784_d_n6;
        locals.var_qbdi_dn7 = assign99490_e151784_d_n7;
        locals.var_qbdi_dn8 = assign99490_e151784_d_n8;
        locals.var_qbdi_dn9 = assign99490_e151784_d_n9;
        locals.var_qbdi_dn10 = assign99490_e151784_d_n10;
        locals.var_qbdi_dn11 = assign99490_e151784_d_n11;
        locals.var_qbdi_dn14 = assign99490_e151784_d_n14;

        let (assign99540_e151817, assign99540_e151817_d_n0, assign99540_e151817_d_n2, assign99540_e151817_d_n4, assign99540_e151817_d_n5, assign99540_e151817_d_n6, assign99540_e151817_d_n7, assign99540_e151817_d_n8, assign99540_e151817_d_n9, assign99540_e151817_d_n10, assign99540_e151817_d_n11, assign99540_e151817_d_n14,) = {
    if (locals.var_guard2302 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibsi, locals.var_ibsi_dn0, locals.var_ibsi_dn2, locals.var_ibsi_dn4, locals.var_ibsi_dn5, locals.var_ibsi_dn6, locals.var_ibsi_dn7, locals.var_ibsi_dn8, locals.var_ibsi_dn9, locals.var_ibsi_dn10, locals.var_ibsi_dn11, locals.var_ibsi_dn14,)
    }
};
        locals.var_ibsi = assign99540_e151817;
        locals.var_ibsi_dn0 = assign99540_e151817_d_n0;
        locals.var_ibsi_dn2 = assign99540_e151817_d_n2;
        locals.var_ibsi_dn4 = assign99540_e151817_d_n4;
        locals.var_ibsi_dn5 = assign99540_e151817_d_n5;
        locals.var_ibsi_dn6 = assign99540_e151817_d_n6;
        locals.var_ibsi_dn7 = assign99540_e151817_d_n7;
        locals.var_ibsi_dn8 = assign99540_e151817_d_n8;
        locals.var_ibsi_dn9 = assign99540_e151817_d_n9;
        locals.var_ibsi_dn10 = assign99540_e151817_d_n10;
        locals.var_ibsi_dn11 = assign99540_e151817_d_n11;
        locals.var_ibsi_dn14 = assign99540_e151817_d_n14;

        let (assign99560_e151833, assign99560_e151833_d_n0, assign99560_e151833_d_n2, assign99560_e151833_d_n4, assign99560_e151833_d_n5, assign99560_e151833_d_n6, assign99560_e151833_d_n7, assign99560_e151833_d_n8, assign99560_e151833_d_n9, assign99560_e151833_d_n10, assign99560_e151833_d_n11, assign99560_e151833_d_n14,) = {
    if (locals.var_guard2302 == 0.0) {
        let assign99560_e151828: f64 = (locals.var_qbs_btm + locals.var_qbs_sws);
        let assign99560_e151830: f64 = (assign99560_e151828 + locals.var_qbs_swg);
        let assign99560_e151831: f64 = (locals.var_mfactor * assign99560_e151830);
        (assign99560_e151831, (locals.var_mfactor * ((locals.var_qbs_btm_dn0 + locals.var_qbs_sws_dn0) + locals.var_qbs_swg_dn0)), (locals.var_mfactor * ((locals.var_qbs_btm_dn2 + locals.var_qbs_sws_dn2) + locals.var_qbs_swg_dn2)), (locals.var_mfactor * ((locals.var_qbs_btm_dn4 + locals.var_qbs_sws_dn4) + locals.var_qbs_swg_dn4)), (locals.var_mfactor * ((locals.var_qbs_btm_dn5 + locals.var_qbs_sws_dn5) + locals.var_qbs_swg_dn5)), (locals.var_mfactor * ((locals.var_qbs_btm_dn6 + locals.var_qbs_sws_dn6) + locals.var_qbs_swg_dn6)), (locals.var_mfactor * ((locals.var_qbs_btm_dn7 + locals.var_qbs_sws_dn7) + locals.var_qbs_swg_dn7)), (locals.var_mfactor * ((locals.var_qbs_btm_dn8 + locals.var_qbs_sws_dn8) + locals.var_qbs_swg_dn8)), (locals.var_mfactor * ((locals.var_qbs_btm_dn9 + locals.var_qbs_sws_dn9) + locals.var_qbs_swg_dn9)), (locals.var_mfactor * ((locals.var_qbs_btm_dn10 + locals.var_qbs_sws_dn10) + locals.var_qbs_swg_dn10)), (locals.var_mfactor * ((locals.var_qbs_btm_dn11 + locals.var_qbs_sws_dn11) + locals.var_qbs_swg_dn11)), (locals.var_mfactor * ((locals.var_qbs_btm_dn14 + locals.var_qbs_sws_dn14) + locals.var_qbs_swg_dn14)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn14,)
    }
};
        locals.var_qbs = assign99560_e151833;
        locals.var_qbs_dn0 = assign99560_e151833_d_n0;
        locals.var_qbs_dn2 = assign99560_e151833_d_n2;
        locals.var_qbs_dn4 = assign99560_e151833_d_n4;
        locals.var_qbs_dn5 = assign99560_e151833_d_n5;
        locals.var_qbs_dn6 = assign99560_e151833_d_n6;
        locals.var_qbs_dn7 = assign99560_e151833_d_n7;
        locals.var_qbs_dn8 = assign99560_e151833_d_n8;
        locals.var_qbs_dn9 = assign99560_e151833_d_n9;
        locals.var_qbs_dn10 = assign99560_e151833_d_n10;
        locals.var_qbs_dn11 = assign99560_e151833_d_n11;
        locals.var_qbs_dn14 = assign99560_e151833_d_n14;

        let (assign99570_e151844, assign99570_e151844_d_n0, assign99570_e151844_d_n2, assign99570_e151844_d_n4, assign99570_e151844_d_n5, assign99570_e151844_d_n6, assign99570_e151844_d_n7, assign99570_e151844_d_n8, assign99570_e151844_d_n9, assign99570_e151844_d_n10, assign99570_e151844_d_n11, assign99570_e151844_d_n14, assign99570_e151844_d_n16, assign99570_e151844_d_n17, assign99570_e151844_d_n18,) = {
    if (locals.var_guard2302 == 0.0) {
        let assign99570_e151839: f64 = (locals.var_qbd_btm + locals.var_qbd_sws);
        let assign99570_e151841: f64 = (assign99570_e151839 + locals.var_qbd_swg);
        let assign99570_e151842: f64 = (locals.var_mfactor * assign99570_e151841);
        (assign99570_e151842, (locals.var_mfactor * ((locals.var_qbd_btm_dn0 + locals.var_qbd_sws_dn0) + locals.var_qbd_swg_dn0)), (locals.var_mfactor * ((locals.var_qbd_btm_dn2 + locals.var_qbd_sws_dn2) + locals.var_qbd_swg_dn2)), (locals.var_mfactor * ((locals.var_qbd_btm_dn4 + locals.var_qbd_sws_dn4) + locals.var_qbd_swg_dn4)), (locals.var_mfactor * ((locals.var_qbd_btm_dn5 + locals.var_qbd_sws_dn5) + locals.var_qbd_swg_dn5)), (locals.var_mfactor * ((locals.var_qbd_btm_dn6 + locals.var_qbd_sws_dn6) + locals.var_qbd_swg_dn6)), (locals.var_mfactor * ((locals.var_qbd_btm_dn7 + locals.var_qbd_sws_dn7) + locals.var_qbd_swg_dn7)), (locals.var_mfactor * ((locals.var_qbd_btm_dn8 + locals.var_qbd_sws_dn8) + locals.var_qbd_swg_dn8)), (locals.var_mfactor * ((locals.var_qbd_btm_dn9 + locals.var_qbd_sws_dn9) + locals.var_qbd_swg_dn9)), (locals.var_mfactor * ((locals.var_qbd_btm_dn10 + locals.var_qbd_sws_dn10) + locals.var_qbd_swg_dn10)), (locals.var_mfactor * ((locals.var_qbd_btm_dn11 + locals.var_qbd_sws_dn11) + locals.var_qbd_swg_dn11)), (locals.var_mfactor * ((locals.var_qbd_btm_dn14 + locals.var_qbd_sws_dn14) + locals.var_qbd_swg_dn14)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn14, locals.var_qbd_dn16, locals.var_qbd_dn17, locals.var_qbd_dn18,)
    }
};
        locals.var_qbd = assign99570_e151844;
        locals.var_qbd_dn0 = assign99570_e151844_d_n0;
        locals.var_qbd_dn2 = assign99570_e151844_d_n2;
        locals.var_qbd_dn4 = assign99570_e151844_d_n4;
        locals.var_qbd_dn5 = assign99570_e151844_d_n5;
        locals.var_qbd_dn6 = assign99570_e151844_d_n6;
        locals.var_qbd_dn7 = assign99570_e151844_d_n7;
        locals.var_qbd_dn8 = assign99570_e151844_d_n8;
        locals.var_qbd_dn9 = assign99570_e151844_d_n9;
        locals.var_qbd_dn10 = assign99570_e151844_d_n10;
        locals.var_qbd_dn11 = assign99570_e151844_d_n11;
        locals.var_qbd_dn14 = assign99570_e151844_d_n14;
        locals.var_qbd_dn16 = assign99570_e151844_d_n16;
        locals.var_qbd_dn17 = assign99570_e151844_d_n17;
        locals.var_qbd_dn18 = assign99570_e151844_d_n18;

        let (assign99600_e151871, assign99600_e151871_d_n0, assign99600_e151871_d_n2, assign99600_e151871_d_n4, assign99600_e151871_d_n5, assign99600_e151871_d_n6, assign99600_e151871_d_n7, assign99600_e151871_d_n8, assign99600_e151871_d_n9, assign99600_e151871_d_n10, assign99600_e151871_d_n11, assign99600_e151871_d_n14,) = {
    if (locals.var_guard2302 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsi, locals.var_qbsi_dn0, locals.var_qbsi_dn2, locals.var_qbsi_dn4, locals.var_qbsi_dn5, locals.var_qbsi_dn6, locals.var_qbsi_dn7, locals.var_qbsi_dn8, locals.var_qbsi_dn9, locals.var_qbsi_dn10, locals.var_qbsi_dn11, locals.var_qbsi_dn14,)
    }
};
        locals.var_qbsi = assign99600_e151871;
        locals.var_qbsi_dn0 = assign99600_e151871_d_n0;
        locals.var_qbsi_dn2 = assign99600_e151871_d_n2;
        locals.var_qbsi_dn4 = assign99600_e151871_d_n4;
        locals.var_qbsi_dn5 = assign99600_e151871_d_n5;
        locals.var_qbsi_dn6 = assign99600_e151871_d_n6;
        locals.var_qbsi_dn7 = assign99600_e151871_d_n7;
        locals.var_qbsi_dn8 = assign99600_e151871_d_n8;
        locals.var_qbsi_dn9 = assign99600_e151871_d_n9;
        locals.var_qbsi_dn10 = assign99600_e151871_d_n10;
        locals.var_qbsi_dn11 = assign99600_e151871_d_n11;
        locals.var_qbsi_dn14 = assign99600_e151871_d_n14;

        let (assign99610_e151876, assign99610_e151876_d_n0, assign99610_e151876_d_n2, assign99610_e151876_d_n4, assign99610_e151876_d_n5, assign99610_e151876_d_n6, assign99610_e151876_d_n7, assign99610_e151876_d_n8, assign99610_e151876_d_n9, assign99610_e151876_d_n10, assign99610_e151876_d_n11, assign99610_e151876_d_n14,) = {
    if (locals.var_guard2302 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdi, locals.var_qbdi_dn0, locals.var_qbdi_dn2, locals.var_qbdi_dn4, locals.var_qbdi_dn5, locals.var_qbdi_dn6, locals.var_qbdi_dn7, locals.var_qbdi_dn8, locals.var_qbdi_dn9, locals.var_qbdi_dn10, locals.var_qbdi_dn11, locals.var_qbdi_dn14,)
    }
};
        locals.var_qbdi = assign99610_e151876;
        locals.var_qbdi_dn0 = assign99610_e151876_d_n0;
        locals.var_qbdi_dn2 = assign99610_e151876_d_n2;
        locals.var_qbdi_dn4 = assign99610_e151876_d_n4;
        locals.var_qbdi_dn5 = assign99610_e151876_d_n5;
        locals.var_qbdi_dn6 = assign99610_e151876_d_n6;
        locals.var_qbdi_dn7 = assign99610_e151876_d_n7;
        locals.var_qbdi_dn8 = assign99610_e151876_d_n8;
        locals.var_qbdi_dn9 = assign99610_e151876_d_n9;
        locals.var_qbdi_dn10 = assign99610_e151876_d_n10;
        locals.var_qbdi_dn11 = assign99610_e151876_d_n11;
        locals.var_qbdi_dn14 = assign99610_e151876_d_n14;

        let assign99640_e151889: f64 = (p.p540 / 1e-6);
        locals.var_ndi_i = assign99640_e151889;

        locals.var_njl = locals.var_uc_njd;

        let assign99660_e151893: f64 = (1450.0 / 10000.0);
        locals.var_muen_i = assign99660_e151893;

        let assign99670_e151896: f64 = (500.0 / 10000.0);
        locals.var_muep_i = assign99670_e151896;

        locals.var_juncdlt = 0.001;

        let assign99690_e151901: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign99690_e151904: f64 = (locals.var_eg * locals.var_beta);
        let assign99690_e151905: f64 = (assign99690_e151901 - assign99690_e151904);
        let assign99690_e151908: f64 = (p.p499 * locals.var_log_tratio);
        let assign99690_e151909: f64 = (assign99690_e151905 + assign99690_e151908);
        let assign99690_e151911: f64 = (assign99690_e151909 / locals.var_uc_njd);
        let assign99690_e151912: f64 = (assign99690_e151911).exp();
        let assign99690_e151913: f64 = (1.45e16 * assign99690_e151912);
        locals.var_nin_dio = assign99690_e151913;
        locals.var_nin_dio_dn0 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn2 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn4 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn5 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn6 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn7 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn8 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn9 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn10 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn11 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn14 = (1.45e16 * (assign99690_e151912 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / locals.var_uc_njd)));

        let assign99700_e151916: f64 = (locals.var_nin_dio * locals.var_nin_dio);
        let assign99700_e151918: f64 = (assign99700_e151916 / locals.var_ndi_i);
        locals.var_pn0 = assign99700_e151918;
        locals.var_pn0_dn0 = (((locals.var_nin_dio_dn0 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn0)) / locals.var_ndi_i);
        locals.var_pn0_dn2 = (((locals.var_nin_dio_dn2 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn2)) / locals.var_ndi_i);
        locals.var_pn0_dn4 = (((locals.var_nin_dio_dn4 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn4)) / locals.var_ndi_i);
        locals.var_pn0_dn5 = (((locals.var_nin_dio_dn5 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn5)) / locals.var_ndi_i);
        locals.var_pn0_dn6 = (((locals.var_nin_dio_dn6 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn6)) / locals.var_ndi_i);
        locals.var_pn0_dn7 = (((locals.var_nin_dio_dn7 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn7)) / locals.var_ndi_i);
        locals.var_pn0_dn8 = (((locals.var_nin_dio_dn8 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn8)) / locals.var_ndi_i);
        locals.var_pn0_dn9 = (((locals.var_nin_dio_dn9 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn9)) / locals.var_ndi_i);
        locals.var_pn0_dn10 = (((locals.var_nin_dio_dn10 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn10)) / locals.var_ndi_i);
        locals.var_pn0_dn11 = (((locals.var_nin_dio_dn11 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn11)) / locals.var_ndi_i);
        locals.var_pn0_dn14 = (((locals.var_nin_dio_dn14 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn14)) / locals.var_ndi_i);

        let assign99710_e151921: f64 = (-1.5);
        let assign99710_e151922: f64 = (locals.var_tratio).powf(assign99710_e151921);
        locals.var_t1 = assign99710_e151922;
        locals.var_t1_dn0 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn0)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn0 / locals.var_tratio))) };
        locals.var_t1_dn2 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn2)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn2 / locals.var_tratio))) };
        locals.var_t1_dn4 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn4)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_t1_dn5 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn5)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn5 / locals.var_tratio))) };
        locals.var_t1_dn6 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn6)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn6 / locals.var_tratio))) };
        locals.var_t1_dn7 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn7)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn7 / locals.var_tratio))) };
        locals.var_t1_dn8 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn8)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn8 / locals.var_tratio))) };
        locals.var_t1_dn9 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn9)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn9 / locals.var_tratio))) };
        locals.var_t1_dn10 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn10)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn10 / locals.var_tratio))) };
        locals.var_t1_dn11 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn11)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn11 / locals.var_tratio))) };
        locals.var_t1_dn14 = if 0.0 == 0.0 && ((assign99710_e151921) as f64).is_finite() && ((assign99710_e151921) as f64).fract() == 0.0 { if assign99710_e151921 == 0.0 { 0.0 } else { (assign99710_e151921 * ((locals.var_tratio).powf(assign99710_e151921 - 1.0) * locals.var_tratio_dn14)) } } else { (assign99710_e151922 * (assign99710_e151921 * (locals.var_tratio_dn14 / locals.var_tratio))) };

        let assign99720_e151925: f64 = (locals.var_muen_i * locals.var_t1);
        let assign99720_e151927: f64 = (assign99720_e151925 * locals.var_beta_inv);
        locals.var_dn = assign99720_e151927;
        locals.var_dn_dn0 = (((locals.var_muen_i * locals.var_t1_dn0) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn0));
        locals.var_dn_dn2 = (((locals.var_muen_i * locals.var_t1_dn2) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn2));
        locals.var_dn_dn4 = (((locals.var_muen_i * locals.var_t1_dn4) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn4));
        locals.var_dn_dn5 = (((locals.var_muen_i * locals.var_t1_dn5) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn5));
        locals.var_dn_dn6 = (((locals.var_muen_i * locals.var_t1_dn6) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn6));
        locals.var_dn_dn7 = (((locals.var_muen_i * locals.var_t1_dn7) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn7));
        locals.var_dn_dn8 = (((locals.var_muen_i * locals.var_t1_dn8) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn8));
        locals.var_dn_dn9 = (((locals.var_muen_i * locals.var_t1_dn9) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn9));
        locals.var_dn_dn10 = (((locals.var_muen_i * locals.var_t1_dn10) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn10));
        locals.var_dn_dn11 = (((locals.var_muen_i * locals.var_t1_dn11) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn11));
        locals.var_dn_dn14 = (((locals.var_muen_i * locals.var_t1_dn14) * locals.var_beta_inv) + (assign99720_e151925 * locals.var_beta_inv_dn14));

        let assign99730_e151930: f64 = (locals.var_muep_i * locals.var_t1);
        let assign99730_e151932: f64 = (assign99730_e151930 * locals.var_beta_inv);
        locals.var_dp = assign99730_e151932;
        locals.var_dp_dn0 = (((locals.var_muep_i * locals.var_t1_dn0) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn0));
        locals.var_dp_dn2 = (((locals.var_muep_i * locals.var_t1_dn2) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn2));
        locals.var_dp_dn4 = (((locals.var_muep_i * locals.var_t1_dn4) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn4));
        locals.var_dp_dn5 = (((locals.var_muep_i * locals.var_t1_dn5) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn5));
        locals.var_dp_dn6 = (((locals.var_muep_i * locals.var_t1_dn6) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn6));
        locals.var_dp_dn7 = (((locals.var_muep_i * locals.var_t1_dn7) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn7));
        locals.var_dp_dn8 = (((locals.var_muep_i * locals.var_t1_dn8) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn8));
        locals.var_dp_dn9 = (((locals.var_muep_i * locals.var_t1_dn9) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn9));
        locals.var_dp_dn10 = (((locals.var_muep_i * locals.var_t1_dn10) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn10));
        locals.var_dp_dn11 = (((locals.var_muep_i * locals.var_t1_dn11) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn11));
        locals.var_dp_dn14 = (((locals.var_muep_i * locals.var_t1_dn14) * locals.var_beta_inv) + (assign99730_e151930 * locals.var_beta_inv_dn14));

        let assign99740_e151935: f64 = (2.0 * locals.var_dn);
        let assign99740_e151937: f64 = (assign99740_e151935 * locals.var_dp);
        let assign99740_e151940: f64 = (locals.var_dn + locals.var_dp);
        let assign99740_e151941: f64 = (assign99740_e151937 / assign99740_e151940);
        locals.var_da = assign99740_e151941;
        locals.var_da_dn0 = ((((((2.0 * locals.var_dn_dn0) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn0)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn0 + locals.var_dp_dn0))) / (assign99740_e151940 * assign99740_e151940));
        locals.var_da_dn2 = ((((((2.0 * locals.var_dn_dn2) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn2)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn2 + locals.var_dp_dn2))) / (assign99740_e151940 * assign99740_e151940));
        locals.var_da_dn4 = ((((((2.0 * locals.var_dn_dn4) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn4)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn4 + locals.var_dp_dn4))) / (assign99740_e151940 * assign99740_e151940));
        locals.var_da_dn5 = ((((((2.0 * locals.var_dn_dn5) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn5)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn5 + locals.var_dp_dn5))) / (assign99740_e151940 * assign99740_e151940));
        locals.var_da_dn6 = ((((((2.0 * locals.var_dn_dn6) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn6)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn6 + locals.var_dp_dn6))) / (assign99740_e151940 * assign99740_e151940));
        locals.var_da_dn7 = ((((((2.0 * locals.var_dn_dn7) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn7)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn7 + locals.var_dp_dn7))) / (assign99740_e151940 * assign99740_e151940));
        locals.var_da_dn8 = ((((((2.0 * locals.var_dn_dn8) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn8)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn8 + locals.var_dp_dn8))) / (assign99740_e151940 * assign99740_e151940));
        locals.var_da_dn9 = ((((((2.0 * locals.var_dn_dn9) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn9)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn9 + locals.var_dp_dn9))) / (assign99740_e151940 * assign99740_e151940));
        locals.var_da_dn10 = ((((((2.0 * locals.var_dn_dn10) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn10)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn10 + locals.var_dp_dn10))) / (assign99740_e151940 * assign99740_e151940));
        locals.var_da_dn11 = ((((((2.0 * locals.var_dn_dn11) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn11)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn11 + locals.var_dp_dn11))) / (assign99740_e151940 * assign99740_e151940));
        locals.var_da_dn14 = ((((((2.0 * locals.var_dn_dn14) * locals.var_dp) + (assign99740_e151935 * locals.var_dp_dn14)) * assign99740_e151940) - (assign99740_e151937 * (locals.var_dn_dn14 + locals.var_dp_dn14))) / (assign99740_e151940 * assign99740_e151940));

        let assign99750_e151944: f64 = (locals.var_tratio).powf(p.p547);
        locals.var_t2 = assign99750_e151944;
        locals.var_t2_dn0 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn0)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn0 / locals.var_tratio))) };
        locals.var_t2_dn2 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn2)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn2 / locals.var_tratio))) };
        locals.var_t2_dn4 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn4)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_t2_dn5 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn5)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn5 / locals.var_tratio))) };
        locals.var_t2_dn6 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn6)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn6 / locals.var_tratio))) };
        locals.var_t2_dn7 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn7)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn7 / locals.var_tratio))) };
        locals.var_t2_dn8 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn8)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn8 / locals.var_tratio))) };
        locals.var_t2_dn9 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn9)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn9 / locals.var_tratio))) };
        locals.var_t2_dn10 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn10)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn10 / locals.var_tratio))) };
        locals.var_t2_dn11 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn11)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn11 / locals.var_tratio))) };
        locals.var_t2_dn14 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn14)) } } else { (assign99750_e151944 * (p.p547 * (locals.var_tratio_dn14 / locals.var_tratio))) };

        let assign99760_e151947: f64 = (p.p544 * locals.var_t2);
        locals.var_tau_hl = assign99760_e151947;
        locals.var_tau_hl_dn0 = (p.p544 * locals.var_t2_dn0);
        locals.var_tau_hl_dn2 = (p.p544 * locals.var_t2_dn2);
        locals.var_tau_hl_dn4 = (p.p544 * locals.var_t2_dn4);
        locals.var_tau_hl_dn5 = (p.p544 * locals.var_t2_dn5);
        locals.var_tau_hl_dn6 = (p.p544 * locals.var_t2_dn6);
        locals.var_tau_hl_dn7 = (p.p544 * locals.var_t2_dn7);
        locals.var_tau_hl_dn8 = (p.p544 * locals.var_t2_dn8);
        locals.var_tau_hl_dn9 = (p.p544 * locals.var_t2_dn9);
        locals.var_tau_hl_dn10 = (p.p544 * locals.var_t2_dn10);
        locals.var_tau_hl_dn11 = (p.p544 * locals.var_t2_dn11);
        locals.var_tau_hl_dn14 = (p.p544 * locals.var_t2_dn14);

    }

    pub(super) fn stamp_transient_block_366(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv16 = ctx.node_voltage(nodes[16]);
        let assign99770_e151950: f64 = (locals.var_tau_hl * locals.var_da);
        let assign99770_e151951: f64 = (assign99770_e151950).sqrt();
        locals.var_la = assign99770_e151951;
        locals.var_la_dn0 = (((locals.var_tau_hl_dn0 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn0)) / (2.0 * assign99770_e151951));
        locals.var_la_dn2 = (((locals.var_tau_hl_dn2 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn2)) / (2.0 * assign99770_e151951));
        locals.var_la_dn4 = (((locals.var_tau_hl_dn4 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn4)) / (2.0 * assign99770_e151951));
        locals.var_la_dn5 = (((locals.var_tau_hl_dn5 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn5)) / (2.0 * assign99770_e151951));
        locals.var_la_dn6 = (((locals.var_tau_hl_dn6 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn6)) / (2.0 * assign99770_e151951));
        locals.var_la_dn7 = (((locals.var_tau_hl_dn7 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn7)) / (2.0 * assign99770_e151951));
        locals.var_la_dn8 = (((locals.var_tau_hl_dn8 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn8)) / (2.0 * assign99770_e151951));
        locals.var_la_dn9 = (((locals.var_tau_hl_dn9 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn9)) / (2.0 * assign99770_e151951));
        locals.var_la_dn10 = (((locals.var_tau_hl_dn10 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn10)) / (2.0 * assign99770_e151951));
        locals.var_la_dn11 = (((locals.var_tau_hl_dn11 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn11)) / (2.0 * assign99770_e151951));
        locals.var_la_dn14 = (((locals.var_tau_hl_dn14 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn14)) / (2.0 * assign99770_e151951));

        let assign99780_e151954: f64 = (locals.var_njl * locals.var_beta_inv);
        let assign99780_e151957: f64 = (locals.var_ndi_i / locals.var_pn0);
        let assign99780_e151958: f64 = (assign99780_e151957).ln();
        let assign99780_e151959: f64 = (assign99780_e151954 * assign99780_e151958);
        locals.var_v_ha = assign99780_e151959;
        locals.var_v_ha_dn0 = (((locals.var_njl * locals.var_beta_inv_dn0) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn0) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));
        locals.var_v_ha_dn2 = (((locals.var_njl * locals.var_beta_inv_dn2) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn2) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));
        locals.var_v_ha_dn4 = (((locals.var_njl * locals.var_beta_inv_dn4) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn4) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));
        locals.var_v_ha_dn5 = (((locals.var_njl * locals.var_beta_inv_dn5) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn5) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));
        locals.var_v_ha_dn6 = (((locals.var_njl * locals.var_beta_inv_dn6) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn6) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));
        locals.var_v_ha_dn7 = (((locals.var_njl * locals.var_beta_inv_dn7) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn7) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));
        locals.var_v_ha_dn8 = (((locals.var_njl * locals.var_beta_inv_dn8) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn8) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));
        locals.var_v_ha_dn9 = (((locals.var_njl * locals.var_beta_inv_dn9) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn9) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));
        locals.var_v_ha_dn10 = (((locals.var_njl * locals.var_beta_inv_dn10) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn10) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));
        locals.var_v_ha_dn11 = (((locals.var_njl * locals.var_beta_inv_dn11) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn11) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));
        locals.var_v_ha_dn14 = (((locals.var_njl * locals.var_beta_inv_dn14) * assign99780_e151958) + (assign99780_e151954 * ((-((locals.var_ndi_i * locals.var_pn0_dn14) / (locals.var_pn0 * locals.var_pn0))) / assign99780_e151957)));

        let assign99790_e151962: f64 = (locals.var_njl * locals.var_beta_inv);
        let assign99790_e151965: f64 = (locals.var_ndi_i / locals.var_pn0);
        let assign99790_e151966: f64 = (assign99790_e151965).ln();
        let assign99790_e151969: f64 = (p.p545 / locals.var_la);
        let assign99790_e151970: f64 = (assign99790_e151966 + assign99790_e151969);
        let assign99790_e151971: f64 = (assign99790_e151962 * assign99790_e151970);
        locals.var_v_hk = assign99790_e151971;
        locals.var_v_hk_dn0 = (((locals.var_njl * locals.var_beta_inv_dn0) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn0) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn0) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn2 = (((locals.var_njl * locals.var_beta_inv_dn2) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn2) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn2) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn4 = (((locals.var_njl * locals.var_beta_inv_dn4) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn4) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn4) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn5 = (((locals.var_njl * locals.var_beta_inv_dn5) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn5) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn5) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn6 = (((locals.var_njl * locals.var_beta_inv_dn6) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn6) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn6) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn7 = (((locals.var_njl * locals.var_beta_inv_dn7) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn7) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn7) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn8 = (((locals.var_njl * locals.var_beta_inv_dn8) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn8) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn8) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn9 = (((locals.var_njl * locals.var_beta_inv_dn9) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn9) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn9) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn10 = (((locals.var_njl * locals.var_beta_inv_dn10) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn10) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn10) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn11 = (((locals.var_njl * locals.var_beta_inv_dn11) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn11) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn11) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn14 = (((locals.var_njl * locals.var_beta_inv_dn14) * assign99790_e151970) + (assign99790_e151962 * (((-((locals.var_ndi_i * locals.var_pn0_dn14) / (locals.var_pn0 * locals.var_pn0))) / assign99790_e151965) + (-((p.p545 * locals.var_la_dn14) / (locals.var_la * locals.var_la))))));

        let assign99800_e151974: f64 = if p.p539 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2303 = assign99800_e151974;

        let (assign99810_e151978,) = {
    if (locals.var_guard2303 != 0.0) {
        (locals.var_uc_njd,)
    } else {
        (locals.var_nj_k,)
    }
};
        locals.var_nj_k = assign99810_e151978;

        let (assign99820_e151985, assign99820_e151985_d_n0, assign99820_e151985_d_n2, assign99820_e151985_d_n4, assign99820_e151985_d_n5, assign99820_e151985_d_n6, assign99820_e151985_d_n7, assign99820_e151985_d_n8, assign99820_e151985_d_n9, assign99820_e151985_d_n10, assign99820_e151985_d_n11, assign99820_e151985_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign99820_e151982: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        let assign99820_e151983: f64 = (assign99820_e151982).exp();
        (assign99820_e151983, (assign99820_e151983 * ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0))), (assign99820_e151983 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2)), (assign99820_e151983 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4)), (assign99820_e151983 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5)), (assign99820_e151983 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6)), (assign99820_e151983 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7)), (assign99820_e151983 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8)), (assign99820_e151983 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9)), (assign99820_e151983 * ((locals.var_vbd_jct_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10))), (assign99820_e151983 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn11)), (assign99820_e151983 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn14)),)
    } else {
        (locals.var_exp_a, locals.var_exp_a_dn0, locals.var_exp_a_dn2, locals.var_exp_a_dn4, locals.var_exp_a_dn5, locals.var_exp_a_dn6, locals.var_exp_a_dn7, locals.var_exp_a_dn8, locals.var_exp_a_dn9, locals.var_exp_a_dn10, locals.var_exp_a_dn11, locals.var_exp_a_dn14,)
    }
};
        locals.var_exp_a = assign99820_e151985;
        locals.var_exp_a_dn0 = assign99820_e151985_d_n0;
        locals.var_exp_a_dn2 = assign99820_e151985_d_n2;
        locals.var_exp_a_dn4 = assign99820_e151985_d_n4;
        locals.var_exp_a_dn5 = assign99820_e151985_d_n5;
        locals.var_exp_a_dn6 = assign99820_e151985_d_n6;
        locals.var_exp_a_dn7 = assign99820_e151985_d_n7;
        locals.var_exp_a_dn8 = assign99820_e151985_d_n8;
        locals.var_exp_a_dn9 = assign99820_e151985_d_n9;
        locals.var_exp_a_dn10 = assign99820_e151985_d_n10;
        locals.var_exp_a_dn11 = assign99820_e151985_d_n11;
        locals.var_exp_a_dn14 = assign99820_e151985_d_n14;

        let assign99830_e151989: f64 = (locals.var_v_hk - locals.var_v_ha);
        let assign99830_e151990: f64 = (locals.var_vbd_jct - assign99830_e151989);
        let assign99830_e151992: f64 = if assign99830_e151990 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2304 = assign99830_e151992;

        let (assign99840_e152009, assign99840_e152009_d_n0, assign99840_e152009_d_n2, assign99840_e152009_d_n4, assign99840_e152009_d_n5, assign99840_e152009_d_n6, assign99840_e152009_d_n7, assign99840_e152009_d_n8, assign99840_e152009_d_n9, assign99840_e152009_d_n10, assign99840_e152009_d_n11, assign99840_e152009_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2304 != 0.0)) {
        let assign99840_e151999: f64 = (locals.var_vbd_jct / locals.var_nj_k);
        let assign99840_e152002: f64 = (locals.var_v_hk - locals.var_v_ha);
        let assign99840_e152004: f64 = (assign99840_e152002 / locals.var_nj_k);
        let assign99840_e152005: f64 = (assign99840_e151999 - assign99840_e152004);
        let assign99840_e152006: f64 = (locals.var_beta * assign99840_e152005);
        let assign99840_e152007: f64 = (assign99840_e152006).exp();
        (assign99840_e152007, (assign99840_e152007 * ((locals.var_beta_dn0 * assign99840_e152005) + (locals.var_beta * ((locals.var_vbd_jct_dn0 / locals.var_nj_k) - ((locals.var_v_hk_dn0 - locals.var_v_ha_dn0) / locals.var_nj_k))))), (assign99840_e152007 * ((locals.var_beta_dn2 * assign99840_e152005) + (locals.var_beta * (-((locals.var_v_hk_dn2 - locals.var_v_ha_dn2) / locals.var_nj_k))))), (assign99840_e152007 * ((locals.var_beta_dn4 * assign99840_e152005) + (locals.var_beta * (-((locals.var_v_hk_dn4 - locals.var_v_ha_dn4) / locals.var_nj_k))))), (assign99840_e152007 * ((locals.var_beta_dn5 * assign99840_e152005) + (locals.var_beta * (-((locals.var_v_hk_dn5 - locals.var_v_ha_dn5) / locals.var_nj_k))))), (assign99840_e152007 * ((locals.var_beta_dn6 * assign99840_e152005) + (locals.var_beta * (-((locals.var_v_hk_dn6 - locals.var_v_ha_dn6) / locals.var_nj_k))))), (assign99840_e152007 * ((locals.var_beta_dn7 * assign99840_e152005) + (locals.var_beta * (-((locals.var_v_hk_dn7 - locals.var_v_ha_dn7) / locals.var_nj_k))))), (assign99840_e152007 * ((locals.var_beta_dn8 * assign99840_e152005) + (locals.var_beta * (-((locals.var_v_hk_dn8 - locals.var_v_ha_dn8) / locals.var_nj_k))))), (assign99840_e152007 * ((locals.var_beta_dn9 * assign99840_e152005) + (locals.var_beta * (-((locals.var_v_hk_dn9 - locals.var_v_ha_dn9) / locals.var_nj_k))))), (assign99840_e152007 * ((locals.var_beta_dn10 * assign99840_e152005) + (locals.var_beta * ((locals.var_vbd_jct_dn10 / locals.var_nj_k) - ((locals.var_v_hk_dn10 - locals.var_v_ha_dn10) / locals.var_nj_k))))), (assign99840_e152007 * ((locals.var_beta_dn11 * assign99840_e152005) + (locals.var_beta * (-((locals.var_v_hk_dn11 - locals.var_v_ha_dn11) / locals.var_nj_k))))), (assign99840_e152007 * ((locals.var_beta_dn14 * assign99840_e152005) + (locals.var_beta * (-((locals.var_v_hk_dn14 - locals.var_v_ha_dn14) / locals.var_nj_k))))),)
    } else {
        (locals.var_exp_k, locals.var_exp_k_dn0, locals.var_exp_k_dn2, locals.var_exp_k_dn4, locals.var_exp_k_dn5, locals.var_exp_k_dn6, locals.var_exp_k_dn7, locals.var_exp_k_dn8, locals.var_exp_k_dn9, locals.var_exp_k_dn10, locals.var_exp_k_dn11, locals.var_exp_k_dn14,)
    }
};
        locals.var_exp_k = assign99840_e152009;
        locals.var_exp_k_dn0 = assign99840_e152009_d_n0;
        locals.var_exp_k_dn2 = assign99840_e152009_d_n2;
        locals.var_exp_k_dn4 = assign99840_e152009_d_n4;
        locals.var_exp_k_dn5 = assign99840_e152009_d_n5;
        locals.var_exp_k_dn6 = assign99840_e152009_d_n6;
        locals.var_exp_k_dn7 = assign99840_e152009_d_n7;
        locals.var_exp_k_dn8 = assign99840_e152009_d_n8;
        locals.var_exp_k_dn9 = assign99840_e152009_d_n9;
        locals.var_exp_k_dn10 = assign99840_e152009_d_n10;
        locals.var_exp_k_dn11 = assign99840_e152009_d_n11;
        locals.var_exp_k_dn14 = assign99840_e152009_d_n14;

        let (assign99850_e152016, assign99850_e152016_d_n0, assign99850_e152016_d_n2, assign99850_e152016_d_n4, assign99850_e152016_d_n5, assign99850_e152016_d_n6, assign99850_e152016_d_n7, assign99850_e152016_d_n8, assign99850_e152016_d_n9, assign99850_e152016_d_n10, assign99850_e152016_d_n11, assign99850_e152016_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2304 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_k, locals.var_exp_k_dn0, locals.var_exp_k_dn2, locals.var_exp_k_dn4, locals.var_exp_k_dn5, locals.var_exp_k_dn6, locals.var_exp_k_dn7, locals.var_exp_k_dn8, locals.var_exp_k_dn9, locals.var_exp_k_dn10, locals.var_exp_k_dn11, locals.var_exp_k_dn14,)
    }
};
        locals.var_exp_k = assign99850_e152016;
        locals.var_exp_k_dn0 = assign99850_e152016_d_n0;
        locals.var_exp_k_dn2 = assign99850_e152016_d_n2;
        locals.var_exp_k_dn4 = assign99850_e152016_d_n4;
        locals.var_exp_k_dn5 = assign99850_e152016_d_n5;
        locals.var_exp_k_dn6 = assign99850_e152016_d_n6;
        locals.var_exp_k_dn7 = assign99850_e152016_d_n7;
        locals.var_exp_k_dn8 = assign99850_e152016_d_n8;
        locals.var_exp_k_dn9 = assign99850_e152016_d_n9;
        locals.var_exp_k_dn10 = assign99850_e152016_d_n10;
        locals.var_exp_k_dn11 = assign99850_e152016_d_n11;
        locals.var_exp_k_dn14 = assign99850_e152016_d_n14;

        let assign99860_e152023: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_ha)) { 1.0 } else { 0.0 };
        locals.var_guard2305 = assign99860_e152023;

        let (assign99870_e152031, assign99870_e152031_d_n0, assign99870_e152031_d_n2, assign99870_e152031_d_n4, assign99870_e152031_d_n5, assign99870_e152031_d_n6, assign99870_e152031_d_n7, assign99870_e152031_d_n8, assign99870_e152031_d_n9, assign99870_e152031_d_n10, assign99870_e152031_d_n11, assign99870_e152031_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2305 != 0.0)) {
        let assign99870_e152029: f64 = (locals.var_exp_a * p.p541);
        (assign99870_e152029, (locals.var_exp_a_dn0 * p.p541), (locals.var_exp_a_dn2 * p.p541), (locals.var_exp_a_dn4 * p.p541), (locals.var_exp_a_dn5 * p.p541), (locals.var_exp_a_dn6 * p.p541), (locals.var_exp_a_dn7 * p.p541), (locals.var_exp_a_dn8 * p.p541), (locals.var_exp_a_dn9 * p.p541), (locals.var_exp_a_dn10 * p.p541), (locals.var_exp_a_dn11 * p.p541), (locals.var_exp_a_dn14 * p.p541),)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn11, locals.var_exp_a2_dn14,)
    }
};
        locals.var_exp_a2 = assign99870_e152031;
        locals.var_exp_a2_dn0 = assign99870_e152031_d_n0;
        locals.var_exp_a2_dn2 = assign99870_e152031_d_n2;
        locals.var_exp_a2_dn4 = assign99870_e152031_d_n4;
        locals.var_exp_a2_dn5 = assign99870_e152031_d_n5;
        locals.var_exp_a2_dn6 = assign99870_e152031_d_n6;
        locals.var_exp_a2_dn7 = assign99870_e152031_d_n7;
        locals.var_exp_a2_dn8 = assign99870_e152031_d_n8;
        locals.var_exp_a2_dn9 = assign99870_e152031_d_n9;
        locals.var_exp_a2_dn10 = assign99870_e152031_d_n10;
        locals.var_exp_a2_dn11 = assign99870_e152031_d_n11;
        locals.var_exp_a2_dn14 = assign99870_e152031_d_n14;

        let (assign99880_e152060, assign99880_e152060_d_n0, assign99880_e152060_d_n2, assign99880_e152060_d_n4, assign99880_e152060_d_n5, assign99880_e152060_d_n6, assign99880_e152060_d_n7, assign99880_e152060_d_n8, assign99880_e152060_d_n9, assign99880_e152060_d_n10, assign99880_e152060_d_n11, assign99880_e152060_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2305 == 0.0)) {
        let assign99880_e152038: f64 = (locals.var_exp_a * p.p541);
        let assign99880_e152040: f64 = (-p.p542);
        let assign99880_e152043: f64 = (locals.var_vbd_jct - locals.var_v_ha);
        let assign99880_e152044: f64 = (assign99880_e152040 * assign99880_e152043);
        let assign99880_e152047: f64 = (locals.var_vbd_jct - locals.var_v_ha);
        let assign99880_e152048: f64 = (assign99880_e152044 * assign99880_e152047);
        let assign99880_e152052: f64 = (1.0 / locals.var_tratio);
        let assign99880_e152053: f64 = (assign99880_e152052).ln();
        let assign99880_e152054: f64 = (p.p548 * assign99880_e152053);
        let assign99880_e152055: f64 = (assign99880_e152054).exp();
        let assign99880_e152056: f64 = (assign99880_e152048 * assign99880_e152055);
        let assign99880_e152057: f64 = (assign99880_e152056).exp();
        let assign99880_e152058: f64 = (assign99880_e152038 * assign99880_e152057);
        (assign99880_e152058, (((locals.var_exp_a_dn0 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (locals.var_vbd_jct_dn0 - locals.var_v_ha_dn0)) * assign99880_e152047) + (assign99880_e152044 * (locals.var_vbd_jct_dn0 - locals.var_v_ha_dn0))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn0 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))), (((locals.var_exp_a_dn2 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (-locals.var_v_ha_dn2)) * assign99880_e152047) + (assign99880_e152044 * (-locals.var_v_ha_dn2))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn2 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))), (((locals.var_exp_a_dn4 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (-locals.var_v_ha_dn4)) * assign99880_e152047) + (assign99880_e152044 * (-locals.var_v_ha_dn4))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn4 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))), (((locals.var_exp_a_dn5 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (-locals.var_v_ha_dn5)) * assign99880_e152047) + (assign99880_e152044 * (-locals.var_v_ha_dn5))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn5 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))), (((locals.var_exp_a_dn6 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (-locals.var_v_ha_dn6)) * assign99880_e152047) + (assign99880_e152044 * (-locals.var_v_ha_dn6))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn6 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))), (((locals.var_exp_a_dn7 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (-locals.var_v_ha_dn7)) * assign99880_e152047) + (assign99880_e152044 * (-locals.var_v_ha_dn7))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn7 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))), (((locals.var_exp_a_dn8 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (-locals.var_v_ha_dn8)) * assign99880_e152047) + (assign99880_e152044 * (-locals.var_v_ha_dn8))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn8 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))), (((locals.var_exp_a_dn9 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (-locals.var_v_ha_dn9)) * assign99880_e152047) + (assign99880_e152044 * (-locals.var_v_ha_dn9))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn9 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))), (((locals.var_exp_a_dn10 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (locals.var_vbd_jct_dn10 - locals.var_v_ha_dn10)) * assign99880_e152047) + (assign99880_e152044 * (locals.var_vbd_jct_dn10 - locals.var_v_ha_dn10))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn10 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))), (((locals.var_exp_a_dn11 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (-locals.var_v_ha_dn11)) * assign99880_e152047) + (assign99880_e152044 * (-locals.var_v_ha_dn11))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn11 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))), (((locals.var_exp_a_dn14 * p.p541) * assign99880_e152057) + (assign99880_e152038 * (assign99880_e152057 * (((((assign99880_e152040 * (-locals.var_v_ha_dn14)) * assign99880_e152047) + (assign99880_e152044 * (-locals.var_v_ha_dn14))) * assign99880_e152055) + (assign99880_e152048 * (assign99880_e152055 * (p.p548 * ((-(locals.var_tratio_dn14 / (locals.var_tratio * locals.var_tratio))) / assign99880_e152052)))))))),)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn11, locals.var_exp_a2_dn14,)
    }
};
        locals.var_exp_a2 = assign99880_e152060;
        locals.var_exp_a2_dn0 = assign99880_e152060_d_n0;
        locals.var_exp_a2_dn2 = assign99880_e152060_d_n2;
        locals.var_exp_a2_dn4 = assign99880_e152060_d_n4;
        locals.var_exp_a2_dn5 = assign99880_e152060_d_n5;
        locals.var_exp_a2_dn6 = assign99880_e152060_d_n6;
        locals.var_exp_a2_dn7 = assign99880_e152060_d_n7;
        locals.var_exp_a2_dn8 = assign99880_e152060_d_n8;
        locals.var_exp_a2_dn9 = assign99880_e152060_d_n9;
        locals.var_exp_a2_dn10 = assign99880_e152060_d_n10;
        locals.var_exp_a2_dn11 = assign99880_e152060_d_n11;
        locals.var_exp_a2_dn14 = assign99880_e152060_d_n14;

        let (assign99890_e152069, assign99890_e152069_d_n0, assign99890_e152069_d_n2, assign99890_e152069_d_n4, assign99890_e152069_d_n5, assign99890_e152069_d_n6, assign99890_e152069_d_n7, assign99890_e152069_d_n8, assign99890_e152069_d_n9, assign99890_e152069_d_n10, assign99890_e152069_d_n11, assign99890_e152069_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let (assign99890_e152067, assign99890_e152067_d_n0, assign99890_e152067_d_n2, assign99890_e152067_d_n4, assign99890_e152067_d_n5, assign99890_e152067_d_n6, assign99890_e152067_d_n7, assign99890_e152067_d_n8, assign99890_e152067_d_n9, assign99890_e152067_d_n10, assign99890_e152067_d_n11, assign99890_e152067_d_n14,) = {
            if (locals.var_exp_a2 > 1e20) {
                (1e20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn11, locals.var_exp_a2_dn14,)
            }
        };
        (assign99890_e152067, assign99890_e152067_d_n0, assign99890_e152067_d_n2, assign99890_e152067_d_n4, assign99890_e152067_d_n5, assign99890_e152067_d_n6, assign99890_e152067_d_n7, assign99890_e152067_d_n8, assign99890_e152067_d_n9, assign99890_e152067_d_n10, assign99890_e152067_d_n11, assign99890_e152067_d_n14,)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn11, locals.var_exp_a2_dn14,)
    }
};
        locals.var_exp_a2 = assign99890_e152069;
        locals.var_exp_a2_dn0 = assign99890_e152069_d_n0;
        locals.var_exp_a2_dn2 = assign99890_e152069_d_n2;
        locals.var_exp_a2_dn4 = assign99890_e152069_d_n4;
        locals.var_exp_a2_dn5 = assign99890_e152069_d_n5;
        locals.var_exp_a2_dn6 = assign99890_e152069_d_n6;
        locals.var_exp_a2_dn7 = assign99890_e152069_d_n7;
        locals.var_exp_a2_dn8 = assign99890_e152069_d_n8;
        locals.var_exp_a2_dn9 = assign99890_e152069_d_n9;
        locals.var_exp_a2_dn10 = assign99890_e152069_d_n10;
        locals.var_exp_a2_dn11 = assign99890_e152069_d_n11;
        locals.var_exp_a2_dn14 = assign99890_e152069_d_n14;

        let (assign99900_e152075, assign99900_e152075_d_n0, assign99900_e152075_d_n2, assign99900_e152075_d_n4, assign99900_e152075_d_n5, assign99900_e152075_d_n6, assign99900_e152075_d_n7, assign99900_e152075_d_n8, assign99900_e152075_d_n9, assign99900_e152075_d_n10, assign99900_e152075_d_n11, assign99900_e152075_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign99900_e152073: f64 = (locals.var_pn0 * locals.var_exp_a2);
        (assign99900_e152073, ((locals.var_pn0_dn0 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn0)), ((locals.var_pn0_dn2 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn2)), ((locals.var_pn0_dn4 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn4)), ((locals.var_pn0_dn5 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn5)), ((locals.var_pn0_dn6 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn6)), ((locals.var_pn0_dn7 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn7)), ((locals.var_pn0_dn8 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn8)), ((locals.var_pn0_dn9 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn9)), ((locals.var_pn0_dn10 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn10)), ((locals.var_pn0_dn11 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn11)), ((locals.var_pn0_dn14 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn14)),)
    } else {
        (locals.var_p_na, locals.var_p_na_dn0, locals.var_p_na_dn2, locals.var_p_na_dn4, locals.var_p_na_dn5, locals.var_p_na_dn6, locals.var_p_na_dn7, locals.var_p_na_dn8, locals.var_p_na_dn9, locals.var_p_na_dn10, locals.var_p_na_dn11, locals.var_p_na_dn14,)
    }
};
        locals.var_p_na = assign99900_e152075;
        locals.var_p_na_dn0 = assign99900_e152075_d_n0;
        locals.var_p_na_dn2 = assign99900_e152075_d_n2;
        locals.var_p_na_dn4 = assign99900_e152075_d_n4;
        locals.var_p_na_dn5 = assign99900_e152075_d_n5;
        locals.var_p_na_dn6 = assign99900_e152075_d_n6;
        locals.var_p_na_dn7 = assign99900_e152075_d_n7;
        locals.var_p_na_dn8 = assign99900_e152075_d_n8;
        locals.var_p_na_dn9 = assign99900_e152075_d_n9;
        locals.var_p_na_dn10 = assign99900_e152075_d_n10;
        locals.var_p_na_dn11 = assign99900_e152075_d_n11;
        locals.var_p_na_dn14 = assign99900_e152075_d_n14;

        let (assign99910_e152085, assign99910_e152085_d_n0, assign99910_e152085_d_n2, assign99910_e152085_d_n4, assign99910_e152085_d_n5, assign99910_e152085_d_n6, assign99910_e152085_d_n7, assign99910_e152085_d_n8, assign99910_e152085_d_n9, assign99910_e152085_d_n10, assign99910_e152085_d_n11, assign99910_e152085_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign99910_e152079: f64 = (1.6021918e-19 * p.p13);
        let assign99910_e152082: f64 = (locals.var_p_na - locals.var_pn0);
        let assign99910_e152083: f64 = (assign99910_e152079 * assign99910_e152082);
        (assign99910_e152083, (assign99910_e152079 * (locals.var_p_na_dn0 - locals.var_pn0_dn0)), (assign99910_e152079 * (locals.var_p_na_dn2 - locals.var_pn0_dn2)), (assign99910_e152079 * (locals.var_p_na_dn4 - locals.var_pn0_dn4)), (assign99910_e152079 * (locals.var_p_na_dn5 - locals.var_pn0_dn5)), (assign99910_e152079 * (locals.var_p_na_dn6 - locals.var_pn0_dn6)), (assign99910_e152079 * (locals.var_p_na_dn7 - locals.var_pn0_dn7)), (assign99910_e152079 * (locals.var_p_na_dn8 - locals.var_pn0_dn8)), (assign99910_e152079 * (locals.var_p_na_dn9 - locals.var_pn0_dn9)), (assign99910_e152079 * (locals.var_p_na_dn10 - locals.var_pn0_dn10)), (assign99910_e152079 * (locals.var_p_na_dn11 - locals.var_pn0_dn11)), (assign99910_e152079 * (locals.var_p_na_dn14 - locals.var_pn0_dn14)),)
    } else {
        (locals.var_q_pexa, locals.var_q_pexa_dn0, locals.var_q_pexa_dn2, locals.var_q_pexa_dn4, locals.var_q_pexa_dn5, locals.var_q_pexa_dn6, locals.var_q_pexa_dn7, locals.var_q_pexa_dn8, locals.var_q_pexa_dn9, locals.var_q_pexa_dn10, locals.var_q_pexa_dn11, locals.var_q_pexa_dn14,)
    }
};
        locals.var_q_pexa = assign99910_e152085;
        locals.var_q_pexa_dn0 = assign99910_e152085_d_n0;
        locals.var_q_pexa_dn2 = assign99910_e152085_d_n2;
        locals.var_q_pexa_dn4 = assign99910_e152085_d_n4;
        locals.var_q_pexa_dn5 = assign99910_e152085_d_n5;
        locals.var_q_pexa_dn6 = assign99910_e152085_d_n6;
        locals.var_q_pexa_dn7 = assign99910_e152085_d_n7;
        locals.var_q_pexa_dn8 = assign99910_e152085_d_n8;
        locals.var_q_pexa_dn9 = assign99910_e152085_d_n9;
        locals.var_q_pexa_dn10 = assign99910_e152085_d_n10;
        locals.var_q_pexa_dn11 = assign99910_e152085_d_n11;
        locals.var_q_pexa_dn14 = assign99910_e152085_d_n14;

        let assign99920_e152088: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2306 = assign99920_e152088;

        let (assign99930_e152096, assign99930_e152096_d_n0, assign99930_e152096_d_n2, assign99930_e152096_d_n4, assign99930_e152096_d_n5, assign99930_e152096_d_n6, assign99930_e152096_d_n7, assign99930_e152096_d_n8, assign99930_e152096_d_n9, assign99930_e152096_d_n10, assign99930_e152096_d_n11, assign99930_e152096_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign99930_e152094: f64 = (locals.var_q_pexa * p.p543);
        (assign99930_e152094, (locals.var_q_pexa_dn0 * p.p543), (locals.var_q_pexa_dn2 * p.p543), (locals.var_q_pexa_dn4 * p.p543), (locals.var_q_pexa_dn5 * p.p543), (locals.var_q_pexa_dn6 * p.p543), (locals.var_q_pexa_dn7 * p.p543), (locals.var_q_pexa_dn8 * p.p543), (locals.var_q_pexa_dn9 * p.p543), (locals.var_q_pexa_dn10 * p.p543), (locals.var_q_pexa_dn11 * p.p543), (locals.var_q_pexa_dn14 * p.p543),)
    } else {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn11, locals.var_q_qs_a_dn14,)
    }
};
        locals.var_q_qs_a = assign99930_e152096;
        locals.var_q_qs_a_dn0 = assign99930_e152096_d_n0;
        locals.var_q_qs_a_dn2 = assign99930_e152096_d_n2;
        locals.var_q_qs_a_dn4 = assign99930_e152096_d_n4;
        locals.var_q_qs_a_dn5 = assign99930_e152096_d_n5;
        locals.var_q_qs_a_dn6 = assign99930_e152096_d_n6;
        locals.var_q_qs_a_dn7 = assign99930_e152096_d_n7;
        locals.var_q_qs_a_dn8 = assign99930_e152096_d_n8;
        locals.var_q_qs_a_dn9 = assign99930_e152096_d_n9;
        locals.var_q_qs_a_dn10 = assign99930_e152096_d_n10;
        locals.var_q_qs_a_dn11 = assign99930_e152096_d_n11;
        locals.var_q_qs_a_dn14 = assign99930_e152096_d_n14;

        let (assign99940_e152104, assign99940_e152104_d_n16,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign99940_e152102: f64 = (p.p543 * (nv16 - 0.0));
        (assign99940_e152102, p.p543,)
    } else {
        (locals.var_q_nqs_a, locals.var_q_nqs_a_dn16,)
    }
};
        locals.var_q_nqs_a = assign99940_e152104;
        locals.var_q_nqs_a_dn16 = assign99940_e152104_d_n16;

        let (assign99950_e152114, assign99950_e152114_d_n0, assign99950_e152114_d_n2, assign99950_e152114_d_n4, assign99950_e152114_d_n5, assign99950_e152114_d_n6, assign99950_e152114_d_n7, assign99950_e152114_d_n8, assign99950_e152114_d_n9, assign99950_e152114_d_n10, assign99950_e152114_d_n11, assign99950_e152114_d_n14, assign99950_e152114_d_n16,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign99950_e152110: f64 = (locals.var_q_nqs_a - locals.var_q_qs_a);
        let assign99950_e152112: f64 = (assign99950_e152110 / p.p543);
        (assign99950_e152112, ((-locals.var_q_qs_a_dn0) / p.p543), ((-locals.var_q_qs_a_dn2) / p.p543), ((-locals.var_q_qs_a_dn4) / p.p543), ((-locals.var_q_qs_a_dn5) / p.p543), ((-locals.var_q_qs_a_dn6) / p.p543), ((-locals.var_q_qs_a_dn7) / p.p543), ((-locals.var_q_qs_a_dn8) / p.p543), ((-locals.var_q_qs_a_dn9) / p.p543), ((-locals.var_q_qs_a_dn10) / p.p543), ((-locals.var_q_qs_a_dn11) / p.p543), ((-locals.var_q_qs_a_dn14) / p.p543), (locals.var_q_nqs_a_dn16 / p.p543),)
    } else {
        (locals.var_inqs0_a, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn11, locals.var_inqs0_a_dn14, locals.var_inqs0_a_dn16,)
    }
};
        locals.var_inqs0_a = assign99950_e152114;
        locals.var_inqs0_a_dn0 = assign99950_e152114_d_n0;
        locals.var_inqs0_a_dn2 = assign99950_e152114_d_n2;
        locals.var_inqs0_a_dn4 = assign99950_e152114_d_n4;
        locals.var_inqs0_a_dn5 = assign99950_e152114_d_n5;
        locals.var_inqs0_a_dn6 = assign99950_e152114_d_n6;
        locals.var_inqs0_a_dn7 = assign99950_e152114_d_n7;
        locals.var_inqs0_a_dn8 = assign99950_e152114_d_n8;
        locals.var_inqs0_a_dn9 = assign99950_e152114_d_n9;
        locals.var_inqs0_a_dn10 = assign99950_e152114_d_n10;
        locals.var_inqs0_a_dn11 = assign99950_e152114_d_n11;
        locals.var_inqs0_a_dn14 = assign99950_e152114_d_n14;
        locals.var_inqs0_a_dn16 = assign99950_e152114_d_n16;

        let (assign99960_e152122, assign99960_e152122_d_n0, assign99960_e152122_d_n2, assign99960_e152122_d_n4, assign99960_e152122_d_n5, assign99960_e152122_d_n6, assign99960_e152122_d_n7, assign99960_e152122_d_n8, assign99960_e152122_d_n9, assign99960_e152122_d_n10, assign99960_e152122_d_n11, assign99960_e152122_d_n14, assign99960_e152122_d_n16,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign99960_e152120: f64 = (locals.var_q_nqs_a / p.p543);
        (assign99960_e152120, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_q_nqs_a_dn16 / p.p543),)
    } else {
        (locals.var_q_pexa_nqs, locals.var_q_pexa_nqs_dn0, locals.var_q_pexa_nqs_dn2, locals.var_q_pexa_nqs_dn4, locals.var_q_pexa_nqs_dn5, locals.var_q_pexa_nqs_dn6, locals.var_q_pexa_nqs_dn7, locals.var_q_pexa_nqs_dn8, locals.var_q_pexa_nqs_dn9, locals.var_q_pexa_nqs_dn10, locals.var_q_pexa_nqs_dn11, locals.var_q_pexa_nqs_dn14, locals.var_q_pexa_nqs_dn16,)
    }
};
        locals.var_q_pexa_nqs = assign99960_e152122;
        locals.var_q_pexa_nqs_dn0 = assign99960_e152122_d_n0;
        locals.var_q_pexa_nqs_dn2 = assign99960_e152122_d_n2;
        locals.var_q_pexa_nqs_dn4 = assign99960_e152122_d_n4;
        locals.var_q_pexa_nqs_dn5 = assign99960_e152122_d_n5;
        locals.var_q_pexa_nqs_dn6 = assign99960_e152122_d_n6;
        locals.var_q_pexa_nqs_dn7 = assign99960_e152122_d_n7;
        locals.var_q_pexa_nqs_dn8 = assign99960_e152122_d_n8;
        locals.var_q_pexa_nqs_dn9 = assign99960_e152122_d_n9;
        locals.var_q_pexa_nqs_dn10 = assign99960_e152122_d_n10;
        locals.var_q_pexa_nqs_dn11 = assign99960_e152122_d_n11;
        locals.var_q_pexa_nqs_dn14 = assign99960_e152122_d_n14;
        locals.var_q_pexa_nqs_dn16 = assign99960_e152122_d_n16;

        let (assign99970_e152129, assign99970_e152129_d_n0, assign99970_e152129_d_n2, assign99970_e152129_d_n4, assign99970_e152129_d_n5, assign99970_e152129_d_n6, assign99970_e152129_d_n7, assign99970_e152129_d_n8, assign99970_e152129_d_n9, assign99970_e152129_d_n10, assign99970_e152129_d_n11, assign99970_e152129_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2306 == 0.0)) {
        (locals.var_q_pexa, locals.var_q_pexa_dn0, locals.var_q_pexa_dn2, locals.var_q_pexa_dn4, locals.var_q_pexa_dn5, locals.var_q_pexa_dn6, locals.var_q_pexa_dn7, locals.var_q_pexa_dn8, locals.var_q_pexa_dn9, locals.var_q_pexa_dn10, locals.var_q_pexa_dn11, locals.var_q_pexa_dn14,)
    } else {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn11, locals.var_q_qs_a_dn14,)
    }
};
        locals.var_q_qs_a = assign99970_e152129;
        locals.var_q_qs_a_dn0 = assign99970_e152129_d_n0;
        locals.var_q_qs_a_dn2 = assign99970_e152129_d_n2;
        locals.var_q_qs_a_dn4 = assign99970_e152129_d_n4;
        locals.var_q_qs_a_dn5 = assign99970_e152129_d_n5;
        locals.var_q_qs_a_dn6 = assign99970_e152129_d_n6;
        locals.var_q_qs_a_dn7 = assign99970_e152129_d_n7;
        locals.var_q_qs_a_dn8 = assign99970_e152129_d_n8;
        locals.var_q_qs_a_dn9 = assign99970_e152129_d_n9;
        locals.var_q_qs_a_dn10 = assign99970_e152129_d_n10;
        locals.var_q_qs_a_dn11 = assign99970_e152129_d_n11;
        locals.var_q_qs_a_dn14 = assign99970_e152129_d_n14;

        let (assign99980_e152136, assign99980_e152136_d_n0, assign99980_e152136_d_n2, assign99980_e152136_d_n4, assign99980_e152136_d_n5, assign99980_e152136_d_n6, assign99980_e152136_d_n7, assign99980_e152136_d_n8, assign99980_e152136_d_n9, assign99980_e152136_d_n10, assign99980_e152136_d_n11, assign99980_e152136_d_n14, assign99980_e152136_d_n16,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2306 == 0.0)) {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn11, locals.var_q_qs_a_dn14, 0.0,)
    } else {
        (locals.var_q_pexa_nqs, locals.var_q_pexa_nqs_dn0, locals.var_q_pexa_nqs_dn2, locals.var_q_pexa_nqs_dn4, locals.var_q_pexa_nqs_dn5, locals.var_q_pexa_nqs_dn6, locals.var_q_pexa_nqs_dn7, locals.var_q_pexa_nqs_dn8, locals.var_q_pexa_nqs_dn9, locals.var_q_pexa_nqs_dn10, locals.var_q_pexa_nqs_dn11, locals.var_q_pexa_nqs_dn14, locals.var_q_pexa_nqs_dn16,)
    }
};
        locals.var_q_pexa_nqs = assign99980_e152136;
        locals.var_q_pexa_nqs_dn0 = assign99980_e152136_d_n0;
        locals.var_q_pexa_nqs_dn2 = assign99980_e152136_d_n2;
        locals.var_q_pexa_nqs_dn4 = assign99980_e152136_d_n4;
        locals.var_q_pexa_nqs_dn5 = assign99980_e152136_d_n5;
        locals.var_q_pexa_nqs_dn6 = assign99980_e152136_d_n6;
        locals.var_q_pexa_nqs_dn7 = assign99980_e152136_d_n7;
        locals.var_q_pexa_nqs_dn8 = assign99980_e152136_d_n8;
        locals.var_q_pexa_nqs_dn9 = assign99980_e152136_d_n9;
        locals.var_q_pexa_nqs_dn10 = assign99980_e152136_d_n10;
        locals.var_q_pexa_nqs_dn11 = assign99980_e152136_d_n11;
        locals.var_q_pexa_nqs_dn14 = assign99980_e152136_d_n14;
        locals.var_q_pexa_nqs_dn16 = assign99980_e152136_d_n16;

        let assign99990_e152143: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_hk)) { 1.0 } else { 0.0 };
        locals.var_guard2307 = assign99990_e152143;

        let (assign100000_e152151, assign100000_e152151_d_n0, assign100000_e152151_d_n2, assign100000_e152151_d_n4, assign100000_e152151_d_n5, assign100000_e152151_d_n6, assign100000_e152151_d_n7, assign100000_e152151_d_n8, assign100000_e152151_d_n9, assign100000_e152151_d_n10, assign100000_e152151_d_n11, assign100000_e152151_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2307 != 0.0)) {
        let assign100000_e152149: f64 = (locals.var_exp_k * p.p541);
        (assign100000_e152149, (locals.var_exp_k_dn0 * p.p541), (locals.var_exp_k_dn2 * p.p541), (locals.var_exp_k_dn4 * p.p541), (locals.var_exp_k_dn5 * p.p541), (locals.var_exp_k_dn6 * p.p541), (locals.var_exp_k_dn7 * p.p541), (locals.var_exp_k_dn8 * p.p541), (locals.var_exp_k_dn9 * p.p541), (locals.var_exp_k_dn10 * p.p541), (locals.var_exp_k_dn11 * p.p541), (locals.var_exp_k_dn14 * p.p541),)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
    }
};
        locals.var_exp_k2 = assign100000_e152151;
        locals.var_exp_k2_dn0 = assign100000_e152151_d_n0;
        locals.var_exp_k2_dn2 = assign100000_e152151_d_n2;
        locals.var_exp_k2_dn4 = assign100000_e152151_d_n4;
        locals.var_exp_k2_dn5 = assign100000_e152151_d_n5;
        locals.var_exp_k2_dn6 = assign100000_e152151_d_n6;
        locals.var_exp_k2_dn7 = assign100000_e152151_d_n7;
        locals.var_exp_k2_dn8 = assign100000_e152151_d_n8;
        locals.var_exp_k2_dn9 = assign100000_e152151_d_n9;
        locals.var_exp_k2_dn10 = assign100000_e152151_d_n10;
        locals.var_exp_k2_dn11 = assign100000_e152151_d_n11;
        locals.var_exp_k2_dn14 = assign100000_e152151_d_n14;

        let (assign100010_e152180, assign100010_e152180_d_n0, assign100010_e152180_d_n2, assign100010_e152180_d_n4, assign100010_e152180_d_n5, assign100010_e152180_d_n6, assign100010_e152180_d_n7, assign100010_e152180_d_n8, assign100010_e152180_d_n9, assign100010_e152180_d_n10, assign100010_e152180_d_n11, assign100010_e152180_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2307 == 0.0)) {
        let assign100010_e152158: f64 = (locals.var_exp_k * p.p541);
        let assign100010_e152160: f64 = (-p.p542);
        let assign100010_e152163: f64 = (locals.var_vbd_jct - locals.var_v_hk);
        let assign100010_e152164: f64 = (assign100010_e152160 * assign100010_e152163);
        let assign100010_e152167: f64 = (locals.var_vbd_jct - locals.var_v_hk);
        let assign100010_e152168: f64 = (assign100010_e152164 * assign100010_e152167);
        let assign100010_e152172: f64 = (1.0 / locals.var_tratio);
        let assign100010_e152173: f64 = (assign100010_e152172).ln();
        let assign100010_e152174: f64 = (p.p548 * assign100010_e152173);
        let assign100010_e152175: f64 = (assign100010_e152174).exp();
        let assign100010_e152176: f64 = (assign100010_e152168 * assign100010_e152175);
        let assign100010_e152177: f64 = (assign100010_e152176).exp();
        let assign100010_e152178: f64 = (assign100010_e152158 * assign100010_e152177);
        (assign100010_e152178, (((locals.var_exp_k_dn0 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (locals.var_vbd_jct_dn0 - locals.var_v_hk_dn0)) * assign100010_e152167) + (assign100010_e152164 * (locals.var_vbd_jct_dn0 - locals.var_v_hk_dn0))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn0 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))), (((locals.var_exp_k_dn2 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (-locals.var_v_hk_dn2)) * assign100010_e152167) + (assign100010_e152164 * (-locals.var_v_hk_dn2))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn2 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))), (((locals.var_exp_k_dn4 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (-locals.var_v_hk_dn4)) * assign100010_e152167) + (assign100010_e152164 * (-locals.var_v_hk_dn4))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn4 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))), (((locals.var_exp_k_dn5 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (-locals.var_v_hk_dn5)) * assign100010_e152167) + (assign100010_e152164 * (-locals.var_v_hk_dn5))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn5 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))), (((locals.var_exp_k_dn6 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (-locals.var_v_hk_dn6)) * assign100010_e152167) + (assign100010_e152164 * (-locals.var_v_hk_dn6))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn6 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))), (((locals.var_exp_k_dn7 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (-locals.var_v_hk_dn7)) * assign100010_e152167) + (assign100010_e152164 * (-locals.var_v_hk_dn7))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn7 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))), (((locals.var_exp_k_dn8 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (-locals.var_v_hk_dn8)) * assign100010_e152167) + (assign100010_e152164 * (-locals.var_v_hk_dn8))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn8 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))), (((locals.var_exp_k_dn9 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (-locals.var_v_hk_dn9)) * assign100010_e152167) + (assign100010_e152164 * (-locals.var_v_hk_dn9))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn9 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))), (((locals.var_exp_k_dn10 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (locals.var_vbd_jct_dn10 - locals.var_v_hk_dn10)) * assign100010_e152167) + (assign100010_e152164 * (locals.var_vbd_jct_dn10 - locals.var_v_hk_dn10))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn10 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))), (((locals.var_exp_k_dn11 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (-locals.var_v_hk_dn11)) * assign100010_e152167) + (assign100010_e152164 * (-locals.var_v_hk_dn11))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn11 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))), (((locals.var_exp_k_dn14 * p.p541) * assign100010_e152177) + (assign100010_e152158 * (assign100010_e152177 * (((((assign100010_e152160 * (-locals.var_v_hk_dn14)) * assign100010_e152167) + (assign100010_e152164 * (-locals.var_v_hk_dn14))) * assign100010_e152175) + (assign100010_e152168 * (assign100010_e152175 * (p.p548 * ((-(locals.var_tratio_dn14 / (locals.var_tratio * locals.var_tratio))) / assign100010_e152172)))))))),)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
    }
};
        locals.var_exp_k2 = assign100010_e152180;
        locals.var_exp_k2_dn0 = assign100010_e152180_d_n0;
        locals.var_exp_k2_dn2 = assign100010_e152180_d_n2;
        locals.var_exp_k2_dn4 = assign100010_e152180_d_n4;
        locals.var_exp_k2_dn5 = assign100010_e152180_d_n5;
        locals.var_exp_k2_dn6 = assign100010_e152180_d_n6;
        locals.var_exp_k2_dn7 = assign100010_e152180_d_n7;
        locals.var_exp_k2_dn8 = assign100010_e152180_d_n8;
        locals.var_exp_k2_dn9 = assign100010_e152180_d_n9;
        locals.var_exp_k2_dn10 = assign100010_e152180_d_n10;
        locals.var_exp_k2_dn11 = assign100010_e152180_d_n11;
        locals.var_exp_k2_dn14 = assign100010_e152180_d_n14;

        let (assign100020_e152189, assign100020_e152189_d_n0, assign100020_e152189_d_n2, assign100020_e152189_d_n4, assign100020_e152189_d_n5, assign100020_e152189_d_n6, assign100020_e152189_d_n7, assign100020_e152189_d_n8, assign100020_e152189_d_n9, assign100020_e152189_d_n10, assign100020_e152189_d_n11, assign100020_e152189_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let (assign100020_e152187, assign100020_e152187_d_n0, assign100020_e152187_d_n2, assign100020_e152187_d_n4, assign100020_e152187_d_n5, assign100020_e152187_d_n6, assign100020_e152187_d_n7, assign100020_e152187_d_n8, assign100020_e152187_d_n9, assign100020_e152187_d_n10, assign100020_e152187_d_n11, assign100020_e152187_d_n14,) = {
            if (locals.var_exp_k2 > 1e20) {
                (1e20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
            }
        };
        (assign100020_e152187, assign100020_e152187_d_n0, assign100020_e152187_d_n2, assign100020_e152187_d_n4, assign100020_e152187_d_n5, assign100020_e152187_d_n6, assign100020_e152187_d_n7, assign100020_e152187_d_n8, assign100020_e152187_d_n9, assign100020_e152187_d_n10, assign100020_e152187_d_n11, assign100020_e152187_d_n14,)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
    }
};
        locals.var_exp_k2 = assign100020_e152189;
        locals.var_exp_k2_dn0 = assign100020_e152189_d_n0;
        locals.var_exp_k2_dn2 = assign100020_e152189_d_n2;
        locals.var_exp_k2_dn4 = assign100020_e152189_d_n4;
        locals.var_exp_k2_dn5 = assign100020_e152189_d_n5;
        locals.var_exp_k2_dn6 = assign100020_e152189_d_n6;
        locals.var_exp_k2_dn7 = assign100020_e152189_d_n7;
        locals.var_exp_k2_dn8 = assign100020_e152189_d_n8;
        locals.var_exp_k2_dn9 = assign100020_e152189_d_n9;
        locals.var_exp_k2_dn10 = assign100020_e152189_d_n10;
        locals.var_exp_k2_dn11 = assign100020_e152189_d_n11;
        locals.var_exp_k2_dn14 = assign100020_e152189_d_n14;

        let (assign100030_e152195, assign100030_e152195_d_n0, assign100030_e152195_d_n2, assign100030_e152195_d_n4, assign100030_e152195_d_n5, assign100030_e152195_d_n6, assign100030_e152195_d_n7, assign100030_e152195_d_n8, assign100030_e152195_d_n9, assign100030_e152195_d_n10, assign100030_e152195_d_n11, assign100030_e152195_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100030_e152193: f64 = (locals.var_pn0 * locals.var_exp_k2);
        (assign100030_e152193, ((locals.var_pn0_dn0 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn0)), ((locals.var_pn0_dn2 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn2)), ((locals.var_pn0_dn4 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn4)), ((locals.var_pn0_dn5 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn5)), ((locals.var_pn0_dn6 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn6)), ((locals.var_pn0_dn7 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn7)), ((locals.var_pn0_dn8 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn8)), ((locals.var_pn0_dn9 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn9)), ((locals.var_pn0_dn10 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn10)), ((locals.var_pn0_dn11 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn11)), ((locals.var_pn0_dn14 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn14)),)
    } else {
        (locals.var_p_nk, locals.var_p_nk_dn0, locals.var_p_nk_dn2, locals.var_p_nk_dn4, locals.var_p_nk_dn5, locals.var_p_nk_dn6, locals.var_p_nk_dn7, locals.var_p_nk_dn8, locals.var_p_nk_dn9, locals.var_p_nk_dn10, locals.var_p_nk_dn11, locals.var_p_nk_dn14,)
    }
};
        locals.var_p_nk = assign100030_e152195;
        locals.var_p_nk_dn0 = assign100030_e152195_d_n0;
        locals.var_p_nk_dn2 = assign100030_e152195_d_n2;
        locals.var_p_nk_dn4 = assign100030_e152195_d_n4;
        locals.var_p_nk_dn5 = assign100030_e152195_d_n5;
        locals.var_p_nk_dn6 = assign100030_e152195_d_n6;
        locals.var_p_nk_dn7 = assign100030_e152195_d_n7;
        locals.var_p_nk_dn8 = assign100030_e152195_d_n8;
        locals.var_p_nk_dn9 = assign100030_e152195_d_n9;
        locals.var_p_nk_dn10 = assign100030_e152195_d_n10;
        locals.var_p_nk_dn11 = assign100030_e152195_d_n11;
        locals.var_p_nk_dn14 = assign100030_e152195_d_n14;

    }

    pub(super) fn stamp_transient_block_367(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (assign100040_e152205, assign100040_e152205_d_n0, assign100040_e152205_d_n2, assign100040_e152205_d_n4, assign100040_e152205_d_n5, assign100040_e152205_d_n6, assign100040_e152205_d_n7, assign100040_e152205_d_n8, assign100040_e152205_d_n9, assign100040_e152205_d_n10, assign100040_e152205_d_n11, assign100040_e152205_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100040_e152199: f64 = (1.6021918e-19 * p.p13);
        let assign100040_e152202: f64 = (locals.var_p_nk - locals.var_pn0);
        let assign100040_e152203: f64 = (assign100040_e152199 * assign100040_e152202);
        (assign100040_e152203, (assign100040_e152199 * (locals.var_p_nk_dn0 - locals.var_pn0_dn0)), (assign100040_e152199 * (locals.var_p_nk_dn2 - locals.var_pn0_dn2)), (assign100040_e152199 * (locals.var_p_nk_dn4 - locals.var_pn0_dn4)), (assign100040_e152199 * (locals.var_p_nk_dn5 - locals.var_pn0_dn5)), (assign100040_e152199 * (locals.var_p_nk_dn6 - locals.var_pn0_dn6)), (assign100040_e152199 * (locals.var_p_nk_dn7 - locals.var_pn0_dn7)), (assign100040_e152199 * (locals.var_p_nk_dn8 - locals.var_pn0_dn8)), (assign100040_e152199 * (locals.var_p_nk_dn9 - locals.var_pn0_dn9)), (assign100040_e152199 * (locals.var_p_nk_dn10 - locals.var_pn0_dn10)), (assign100040_e152199 * (locals.var_p_nk_dn11 - locals.var_pn0_dn11)), (assign100040_e152199 * (locals.var_p_nk_dn14 - locals.var_pn0_dn14)),)
    } else {
        (locals.var_q_pexk, locals.var_q_pexk_dn0, locals.var_q_pexk_dn2, locals.var_q_pexk_dn4, locals.var_q_pexk_dn5, locals.var_q_pexk_dn6, locals.var_q_pexk_dn7, locals.var_q_pexk_dn8, locals.var_q_pexk_dn9, locals.var_q_pexk_dn10, locals.var_q_pexk_dn11, locals.var_q_pexk_dn14,)
    }
};
        locals.var_q_pexk = assign100040_e152205;
        locals.var_q_pexk_dn0 = assign100040_e152205_d_n0;
        locals.var_q_pexk_dn2 = assign100040_e152205_d_n2;
        locals.var_q_pexk_dn4 = assign100040_e152205_d_n4;
        locals.var_q_pexk_dn5 = assign100040_e152205_d_n5;
        locals.var_q_pexk_dn6 = assign100040_e152205_d_n6;
        locals.var_q_pexk_dn7 = assign100040_e152205_d_n7;
        locals.var_q_pexk_dn8 = assign100040_e152205_d_n8;
        locals.var_q_pexk_dn9 = assign100040_e152205_d_n9;
        locals.var_q_pexk_dn10 = assign100040_e152205_d_n10;
        locals.var_q_pexk_dn11 = assign100040_e152205_d_n11;
        locals.var_q_pexk_dn14 = assign100040_e152205_d_n14;

        let assign100050_e152208: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2308 = assign100050_e152208;

        let (assign100060_e152216, assign100060_e152216_d_n0, assign100060_e152216_d_n2, assign100060_e152216_d_n4, assign100060_e152216_d_n5, assign100060_e152216_d_n6, assign100060_e152216_d_n7, assign100060_e152216_d_n8, assign100060_e152216_d_n9, assign100060_e152216_d_n10, assign100060_e152216_d_n11, assign100060_e152216_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100060_e152214: f64 = (locals.var_q_pexk * p.p543);
        (assign100060_e152214, (locals.var_q_pexk_dn0 * p.p543), (locals.var_q_pexk_dn2 * p.p543), (locals.var_q_pexk_dn4 * p.p543), (locals.var_q_pexk_dn5 * p.p543), (locals.var_q_pexk_dn6 * p.p543), (locals.var_q_pexk_dn7 * p.p543), (locals.var_q_pexk_dn8 * p.p543), (locals.var_q_pexk_dn9 * p.p543), (locals.var_q_pexk_dn10 * p.p543), (locals.var_q_pexk_dn11 * p.p543), (locals.var_q_pexk_dn14 * p.p543),)
    } else {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn11, locals.var_q_qs_k_dn14,)
    }
};
        locals.var_q_qs_k = assign100060_e152216;
        locals.var_q_qs_k_dn0 = assign100060_e152216_d_n0;
        locals.var_q_qs_k_dn2 = assign100060_e152216_d_n2;
        locals.var_q_qs_k_dn4 = assign100060_e152216_d_n4;
        locals.var_q_qs_k_dn5 = assign100060_e152216_d_n5;
        locals.var_q_qs_k_dn6 = assign100060_e152216_d_n6;
        locals.var_q_qs_k_dn7 = assign100060_e152216_d_n7;
        locals.var_q_qs_k_dn8 = assign100060_e152216_d_n8;
        locals.var_q_qs_k_dn9 = assign100060_e152216_d_n9;
        locals.var_q_qs_k_dn10 = assign100060_e152216_d_n10;
        locals.var_q_qs_k_dn11 = assign100060_e152216_d_n11;
        locals.var_q_qs_k_dn14 = assign100060_e152216_d_n14;

        let (assign100070_e152224, assign100070_e152224_d_n17,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100070_e152222: f64 = (p.p543 * (nv17 - 0.0));
        (assign100070_e152222, p.p543,)
    } else {
        (locals.var_q_nqs_k, locals.var_q_nqs_k_dn17,)
    }
};
        locals.var_q_nqs_k = assign100070_e152224;
        locals.var_q_nqs_k_dn17 = assign100070_e152224_d_n17;

        let (assign100080_e152234, assign100080_e152234_d_n0, assign100080_e152234_d_n2, assign100080_e152234_d_n4, assign100080_e152234_d_n5, assign100080_e152234_d_n6, assign100080_e152234_d_n7, assign100080_e152234_d_n8, assign100080_e152234_d_n9, assign100080_e152234_d_n10, assign100080_e152234_d_n11, assign100080_e152234_d_n14, assign100080_e152234_d_n17,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100080_e152230: f64 = (locals.var_q_nqs_k - locals.var_q_qs_k);
        let assign100080_e152232: f64 = (assign100080_e152230 / p.p543);
        (assign100080_e152232, ((-locals.var_q_qs_k_dn0) / p.p543), ((-locals.var_q_qs_k_dn2) / p.p543), ((-locals.var_q_qs_k_dn4) / p.p543), ((-locals.var_q_qs_k_dn5) / p.p543), ((-locals.var_q_qs_k_dn6) / p.p543), ((-locals.var_q_qs_k_dn7) / p.p543), ((-locals.var_q_qs_k_dn8) / p.p543), ((-locals.var_q_qs_k_dn9) / p.p543), ((-locals.var_q_qs_k_dn10) / p.p543), ((-locals.var_q_qs_k_dn11) / p.p543), ((-locals.var_q_qs_k_dn14) / p.p543), (locals.var_q_nqs_k_dn17 / p.p543),)
    } else {
        (locals.var_inqs0_k, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn11, locals.var_inqs0_k_dn14, locals.var_inqs0_k_dn17,)
    }
};
        locals.var_inqs0_k = assign100080_e152234;
        locals.var_inqs0_k_dn0 = assign100080_e152234_d_n0;
        locals.var_inqs0_k_dn2 = assign100080_e152234_d_n2;
        locals.var_inqs0_k_dn4 = assign100080_e152234_d_n4;
        locals.var_inqs0_k_dn5 = assign100080_e152234_d_n5;
        locals.var_inqs0_k_dn6 = assign100080_e152234_d_n6;
        locals.var_inqs0_k_dn7 = assign100080_e152234_d_n7;
        locals.var_inqs0_k_dn8 = assign100080_e152234_d_n8;
        locals.var_inqs0_k_dn9 = assign100080_e152234_d_n9;
        locals.var_inqs0_k_dn10 = assign100080_e152234_d_n10;
        locals.var_inqs0_k_dn11 = assign100080_e152234_d_n11;
        locals.var_inqs0_k_dn14 = assign100080_e152234_d_n14;
        locals.var_inqs0_k_dn17 = assign100080_e152234_d_n17;

        let (assign100090_e152242, assign100090_e152242_d_n0, assign100090_e152242_d_n2, assign100090_e152242_d_n4, assign100090_e152242_d_n5, assign100090_e152242_d_n6, assign100090_e152242_d_n7, assign100090_e152242_d_n8, assign100090_e152242_d_n9, assign100090_e152242_d_n10, assign100090_e152242_d_n11, assign100090_e152242_d_n14, assign100090_e152242_d_n17,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100090_e152240: f64 = (locals.var_q_nqs_k / p.p543);
        (assign100090_e152240, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_q_nqs_k_dn17 / p.p543),)
    } else {
        (locals.var_q_pexk_nqs, locals.var_q_pexk_nqs_dn0, locals.var_q_pexk_nqs_dn2, locals.var_q_pexk_nqs_dn4, locals.var_q_pexk_nqs_dn5, locals.var_q_pexk_nqs_dn6, locals.var_q_pexk_nqs_dn7, locals.var_q_pexk_nqs_dn8, locals.var_q_pexk_nqs_dn9, locals.var_q_pexk_nqs_dn10, locals.var_q_pexk_nqs_dn11, locals.var_q_pexk_nqs_dn14, locals.var_q_pexk_nqs_dn17,)
    }
};
        locals.var_q_pexk_nqs = assign100090_e152242;
        locals.var_q_pexk_nqs_dn0 = assign100090_e152242_d_n0;
        locals.var_q_pexk_nqs_dn2 = assign100090_e152242_d_n2;
        locals.var_q_pexk_nqs_dn4 = assign100090_e152242_d_n4;
        locals.var_q_pexk_nqs_dn5 = assign100090_e152242_d_n5;
        locals.var_q_pexk_nqs_dn6 = assign100090_e152242_d_n6;
        locals.var_q_pexk_nqs_dn7 = assign100090_e152242_d_n7;
        locals.var_q_pexk_nqs_dn8 = assign100090_e152242_d_n8;
        locals.var_q_pexk_nqs_dn9 = assign100090_e152242_d_n9;
        locals.var_q_pexk_nqs_dn10 = assign100090_e152242_d_n10;
        locals.var_q_pexk_nqs_dn11 = assign100090_e152242_d_n11;
        locals.var_q_pexk_nqs_dn14 = assign100090_e152242_d_n14;
        locals.var_q_pexk_nqs_dn17 = assign100090_e152242_d_n17;

        let (assign100100_e152249, assign100100_e152249_d_n0, assign100100_e152249_d_n2, assign100100_e152249_d_n4, assign100100_e152249_d_n5, assign100100_e152249_d_n6, assign100100_e152249_d_n7, assign100100_e152249_d_n8, assign100100_e152249_d_n9, assign100100_e152249_d_n10, assign100100_e152249_d_n11, assign100100_e152249_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2308 == 0.0)) {
        (locals.var_q_pexk, locals.var_q_pexk_dn0, locals.var_q_pexk_dn2, locals.var_q_pexk_dn4, locals.var_q_pexk_dn5, locals.var_q_pexk_dn6, locals.var_q_pexk_dn7, locals.var_q_pexk_dn8, locals.var_q_pexk_dn9, locals.var_q_pexk_dn10, locals.var_q_pexk_dn11, locals.var_q_pexk_dn14,)
    } else {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn11, locals.var_q_qs_k_dn14,)
    }
};
        locals.var_q_qs_k = assign100100_e152249;
        locals.var_q_qs_k_dn0 = assign100100_e152249_d_n0;
        locals.var_q_qs_k_dn2 = assign100100_e152249_d_n2;
        locals.var_q_qs_k_dn4 = assign100100_e152249_d_n4;
        locals.var_q_qs_k_dn5 = assign100100_e152249_d_n5;
        locals.var_q_qs_k_dn6 = assign100100_e152249_d_n6;
        locals.var_q_qs_k_dn7 = assign100100_e152249_d_n7;
        locals.var_q_qs_k_dn8 = assign100100_e152249_d_n8;
        locals.var_q_qs_k_dn9 = assign100100_e152249_d_n9;
        locals.var_q_qs_k_dn10 = assign100100_e152249_d_n10;
        locals.var_q_qs_k_dn11 = assign100100_e152249_d_n11;
        locals.var_q_qs_k_dn14 = assign100100_e152249_d_n14;

        let (assign100110_e152256, assign100110_e152256_d_n0, assign100110_e152256_d_n2, assign100110_e152256_d_n4, assign100110_e152256_d_n5, assign100110_e152256_d_n6, assign100110_e152256_d_n7, assign100110_e152256_d_n8, assign100110_e152256_d_n9, assign100110_e152256_d_n10, assign100110_e152256_d_n11, assign100110_e152256_d_n14, assign100110_e152256_d_n17,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2308 == 0.0)) {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn11, locals.var_q_qs_k_dn14, 0.0,)
    } else {
        (locals.var_q_pexk_nqs, locals.var_q_pexk_nqs_dn0, locals.var_q_pexk_nqs_dn2, locals.var_q_pexk_nqs_dn4, locals.var_q_pexk_nqs_dn5, locals.var_q_pexk_nqs_dn6, locals.var_q_pexk_nqs_dn7, locals.var_q_pexk_nqs_dn8, locals.var_q_pexk_nqs_dn9, locals.var_q_pexk_nqs_dn10, locals.var_q_pexk_nqs_dn11, locals.var_q_pexk_nqs_dn14, locals.var_q_pexk_nqs_dn17,)
    }
};
        locals.var_q_pexk_nqs = assign100110_e152256;
        locals.var_q_pexk_nqs_dn0 = assign100110_e152256_d_n0;
        locals.var_q_pexk_nqs_dn2 = assign100110_e152256_d_n2;
        locals.var_q_pexk_nqs_dn4 = assign100110_e152256_d_n4;
        locals.var_q_pexk_nqs_dn5 = assign100110_e152256_d_n5;
        locals.var_q_pexk_nqs_dn6 = assign100110_e152256_d_n6;
        locals.var_q_pexk_nqs_dn7 = assign100110_e152256_d_n7;
        locals.var_q_pexk_nqs_dn8 = assign100110_e152256_d_n8;
        locals.var_q_pexk_nqs_dn9 = assign100110_e152256_d_n9;
        locals.var_q_pexk_nqs_dn10 = assign100110_e152256_d_n10;
        locals.var_q_pexk_nqs_dn11 = assign100110_e152256_d_n11;
        locals.var_q_pexk_nqs_dn14 = assign100110_e152256_d_n14;
        locals.var_q_pexk_nqs_dn17 = assign100110_e152256_d_n17;

        let (assign100120_e152262, assign100120_e152262_d_n0, assign100120_e152262_d_n2, assign100120_e152262_d_n4, assign100120_e152262_d_n5, assign100120_e152262_d_n6, assign100120_e152262_d_n7, assign100120_e152262_d_n8, assign100120_e152262_d_n9, assign100120_e152262_d_n10, assign100120_e152262_d_n11, assign100120_e152262_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100120_e152260: f64 = (p.p506 - locals.var_vbd_jct);
        (assign100120_e152260, (-locals.var_vbd_jct_dn0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbd_jct_dn10), 0.0, 0.0,)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn11, locals.var_vjunc_a_dn14,)
    }
};
        locals.var_vjunc_a = assign100120_e152262;
        locals.var_vjunc_a_dn0 = assign100120_e152262_d_n0;
        locals.var_vjunc_a_dn2 = assign100120_e152262_d_n2;
        locals.var_vjunc_a_dn4 = assign100120_e152262_d_n4;
        locals.var_vjunc_a_dn5 = assign100120_e152262_d_n5;
        locals.var_vjunc_a_dn6 = assign100120_e152262_d_n6;
        locals.var_vjunc_a_dn7 = assign100120_e152262_d_n7;
        locals.var_vjunc_a_dn8 = assign100120_e152262_d_n8;
        locals.var_vjunc_a_dn9 = assign100120_e152262_d_n9;
        locals.var_vjunc_a_dn10 = assign100120_e152262_d_n10;
        locals.var_vjunc_a_dn11 = assign100120_e152262_d_n11;
        locals.var_vjunc_a_dn14 = assign100120_e152262_d_n14;

        let (assign100130_e152275, assign100130_e152275_d_n0, assign100130_e152275_d_n2, assign100130_e152275_d_n4, assign100130_e152275_d_n5, assign100130_e152275_d_n6, assign100130_e152275_d_n7, assign100130_e152275_d_n8, assign100130_e152275_d_n9, assign100130_e152275_d_n10, assign100130_e152275_d_n11, assign100130_e152275_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100130_e152266: f64 = (locals.var_vjunc_a * locals.var_vjunc_a);
        let assign100130_e152269: f64 = (4.0 * locals.var_juncdlt);
        let assign100130_e152271: f64 = (assign100130_e152269 * locals.var_juncdlt);
        let assign100130_e152272: f64 = (assign100130_e152266 + assign100130_e152271);
        let assign100130_e152273: f64 = (assign100130_e152272).sqrt();
        (assign100130_e152273, (((locals.var_vjunc_a_dn0 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn0)) / (2.0 * assign100130_e152273)), (((locals.var_vjunc_a_dn2 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn2)) / (2.0 * assign100130_e152273)), (((locals.var_vjunc_a_dn4 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn4)) / (2.0 * assign100130_e152273)), (((locals.var_vjunc_a_dn5 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn5)) / (2.0 * assign100130_e152273)), (((locals.var_vjunc_a_dn6 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn6)) / (2.0 * assign100130_e152273)), (((locals.var_vjunc_a_dn7 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn7)) / (2.0 * assign100130_e152273)), (((locals.var_vjunc_a_dn8 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn8)) / (2.0 * assign100130_e152273)), (((locals.var_vjunc_a_dn9 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn9)) / (2.0 * assign100130_e152273)), (((locals.var_vjunc_a_dn10 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn10)) / (2.0 * assign100130_e152273)), (((locals.var_vjunc_a_dn11 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn11)) / (2.0 * assign100130_e152273)), (((locals.var_vjunc_a_dn14 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn14)) / (2.0 * assign100130_e152273)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100130_e152275;
        locals.var_tmf2_dn0 = assign100130_e152275_d_n0;
        locals.var_tmf2_dn2 = assign100130_e152275_d_n2;
        locals.var_tmf2_dn4 = assign100130_e152275_d_n4;
        locals.var_tmf2_dn5 = assign100130_e152275_d_n5;
        locals.var_tmf2_dn6 = assign100130_e152275_d_n6;
        locals.var_tmf2_dn7 = assign100130_e152275_d_n7;
        locals.var_tmf2_dn8 = assign100130_e152275_d_n8;
        locals.var_tmf2_dn9 = assign100130_e152275_d_n9;
        locals.var_tmf2_dn10 = assign100130_e152275_d_n10;
        locals.var_tmf2_dn11 = assign100130_e152275_d_n11;
        locals.var_tmf2_dn14 = assign100130_e152275_d_n14;

        let (assign100140_e152285, assign100140_e152285_d_n0, assign100140_e152285_d_n2, assign100140_e152285_d_n4, assign100140_e152285_d_n5, assign100140_e152285_d_n6, assign100140_e152285_d_n7, assign100140_e152285_d_n8, assign100140_e152285_d_n9, assign100140_e152285_d_n10, assign100140_e152285_d_n11, assign100140_e152285_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100140_e152281: f64 = (locals.var_vjunc_a / locals.var_tmf2);
        let assign100140_e152282: f64 = (1.0 + assign100140_e152281);
        let assign100140_e152283: f64 = (0.5 * assign100140_e152282);
        (assign100140_e152283, (0.5 * (((locals.var_vjunc_a_dn0 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn2 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn4 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn5 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn6 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn7 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn8 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn9 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn10 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn11 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn14 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100140_e152285;
        locals.var_t0_dn0 = assign100140_e152285_d_n0;
        locals.var_t0_dn2 = assign100140_e152285_d_n2;
        locals.var_t0_dn4 = assign100140_e152285_d_n4;
        locals.var_t0_dn5 = assign100140_e152285_d_n5;
        locals.var_t0_dn6 = assign100140_e152285_d_n6;
        locals.var_t0_dn7 = assign100140_e152285_d_n7;
        locals.var_t0_dn8 = assign100140_e152285_d_n8;
        locals.var_t0_dn9 = assign100140_e152285_d_n9;
        locals.var_t0_dn10 = assign100140_e152285_d_n10;
        locals.var_t0_dn11 = assign100140_e152285_d_n11;
        locals.var_t0_dn14 = assign100140_e152285_d_n14;

        let (assign100150_e152293, assign100150_e152293_d_n0, assign100150_e152293_d_n2, assign100150_e152293_d_n4, assign100150_e152293_d_n5, assign100150_e152293_d_n6, assign100150_e152293_d_n7, assign100150_e152293_d_n8, assign100150_e152293_d_n9, assign100150_e152293_d_n10, assign100150_e152293_d_n11, assign100150_e152293_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100150_e152290: f64 = (locals.var_vjunc_a + locals.var_tmf2);
        let assign100150_e152291: f64 = (0.5 * assign100150_e152290);
        (assign100150_e152291, (0.5 * (locals.var_vjunc_a_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vjunc_a_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vjunc_a_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vjunc_a_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vjunc_a_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vjunc_a_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vjunc_a_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vjunc_a_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vjunc_a_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vjunc_a_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vjunc_a_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn11, locals.var_vjunc_a_dn14,)
    }
};
        locals.var_vjunc_a = assign100150_e152293;
        locals.var_vjunc_a_dn0 = assign100150_e152293_d_n0;
        locals.var_vjunc_a_dn2 = assign100150_e152293_d_n2;
        locals.var_vjunc_a_dn4 = assign100150_e152293_d_n4;
        locals.var_vjunc_a_dn5 = assign100150_e152293_d_n5;
        locals.var_vjunc_a_dn6 = assign100150_e152293_d_n6;
        locals.var_vjunc_a_dn7 = assign100150_e152293_d_n7;
        locals.var_vjunc_a_dn8 = assign100150_e152293_d_n8;
        locals.var_vjunc_a_dn9 = assign100150_e152293_d_n9;
        locals.var_vjunc_a_dn10 = assign100150_e152293_d_n10;
        locals.var_vjunc_a_dn11 = assign100150_e152293_d_n11;
        locals.var_vjunc_a_dn14 = assign100150_e152293_d_n14;

        let assign100160_e152296: f64 = if locals.var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2309 = assign100160_e152296;

        let (assign100170_e152302, assign100170_e152302_d_n0, assign100170_e152302_d_n2, assign100170_e152302_d_n4, assign100170_e152302_d_n5, assign100170_e152302_d_n6, assign100170_e152302_d_n7, assign100170_e152302_d_n8, assign100170_e152302_d_n9, assign100170_e152302_d_n10, assign100170_e152302_d_n11, assign100170_e152302_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2309 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn11, locals.var_vjunc_a_dn14,)
    }
};
        locals.var_vjunc_a = assign100170_e152302;
        locals.var_vjunc_a_dn0 = assign100170_e152302_d_n0;
        locals.var_vjunc_a_dn2 = assign100170_e152302_d_n2;
        locals.var_vjunc_a_dn4 = assign100170_e152302_d_n4;
        locals.var_vjunc_a_dn5 = assign100170_e152302_d_n5;
        locals.var_vjunc_a_dn6 = assign100170_e152302_d_n6;
        locals.var_vjunc_a_dn7 = assign100170_e152302_d_n7;
        locals.var_vjunc_a_dn8 = assign100170_e152302_d_n8;
        locals.var_vjunc_a_dn9 = assign100170_e152302_d_n9;
        locals.var_vjunc_a_dn10 = assign100170_e152302_d_n10;
        locals.var_vjunc_a_dn11 = assign100170_e152302_d_n11;
        locals.var_vjunc_a_dn14 = assign100170_e152302_d_n14;

        let (assign100180_e152308, assign100180_e152308_d_n0, assign100180_e152308_d_n2, assign100180_e152308_d_n4, assign100180_e152308_d_n5, assign100180_e152308_d_n6, assign100180_e152308_d_n7, assign100180_e152308_d_n8, assign100180_e152308_d_n9, assign100180_e152308_d_n10, assign100180_e152308_d_n11, assign100180_e152308_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2309 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100180_e152308;
        locals.var_t0_dn0 = assign100180_e152308_d_n0;
        locals.var_t0_dn2 = assign100180_e152308_d_n2;
        locals.var_t0_dn4 = assign100180_e152308_d_n4;
        locals.var_t0_dn5 = assign100180_e152308_d_n5;
        locals.var_t0_dn6 = assign100180_e152308_d_n6;
        locals.var_t0_dn7 = assign100180_e152308_d_n7;
        locals.var_t0_dn8 = assign100180_e152308_d_n8;
        locals.var_t0_dn9 = assign100180_e152308_d_n9;
        locals.var_t0_dn10 = assign100180_e152308_d_n10;
        locals.var_t0_dn11 = assign100180_e152308_d_n11;
        locals.var_t0_dn14 = assign100180_e152308_d_n14;

        let (assign100190_e152321, assign100190_e152321_d_n0, assign100190_e152321_d_n2, assign100190_e152321_d_n4, assign100190_e152321_d_n5, assign100190_e152321_d_n6, assign100190_e152321_d_n7, assign100190_e152321_d_n8, assign100190_e152321_d_n9, assign100190_e152321_d_n10, assign100190_e152321_d_n11, assign100190_e152321_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100190_e152312: f64 = (2.0 * 1.034943e-10);
        let assign100190_e152314: f64 = (assign100190_e152312 * locals.var_vjunc_a);
        let assign100190_e152317: f64 = (1.6021918e-19 * locals.var_ndi_i);
        let assign100190_e152318: f64 = (assign100190_e152314 / assign100190_e152317);
        let assign100190_e152319: f64 = (assign100190_e152318).sqrt();
        (assign100190_e152319, (((assign100190_e152312 * locals.var_vjunc_a_dn0) / assign100190_e152317) / (2.0 * assign100190_e152319)), (((assign100190_e152312 * locals.var_vjunc_a_dn2) / assign100190_e152317) / (2.0 * assign100190_e152319)), (((assign100190_e152312 * locals.var_vjunc_a_dn4) / assign100190_e152317) / (2.0 * assign100190_e152319)), (((assign100190_e152312 * locals.var_vjunc_a_dn5) / assign100190_e152317) / (2.0 * assign100190_e152319)), (((assign100190_e152312 * locals.var_vjunc_a_dn6) / assign100190_e152317) / (2.0 * assign100190_e152319)), (((assign100190_e152312 * locals.var_vjunc_a_dn7) / assign100190_e152317) / (2.0 * assign100190_e152319)), (((assign100190_e152312 * locals.var_vjunc_a_dn8) / assign100190_e152317) / (2.0 * assign100190_e152319)), (((assign100190_e152312 * locals.var_vjunc_a_dn9) / assign100190_e152317) / (2.0 * assign100190_e152319)), (((assign100190_e152312 * locals.var_vjunc_a_dn10) / assign100190_e152317) / (2.0 * assign100190_e152319)), (((assign100190_e152312 * locals.var_vjunc_a_dn11) / assign100190_e152317) / (2.0 * assign100190_e152319)), (((assign100190_e152312 * locals.var_vjunc_a_dn14) / assign100190_e152317) / (2.0 * assign100190_e152319)),)
    } else {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn11, locals.var_w_depa_dn14,)
    }
};
        locals.var_w_depa = assign100190_e152321;
        locals.var_w_depa_dn0 = assign100190_e152321_d_n0;
        locals.var_w_depa_dn2 = assign100190_e152321_d_n2;
        locals.var_w_depa_dn4 = assign100190_e152321_d_n4;
        locals.var_w_depa_dn5 = assign100190_e152321_d_n5;
        locals.var_w_depa_dn6 = assign100190_e152321_d_n6;
        locals.var_w_depa_dn7 = assign100190_e152321_d_n7;
        locals.var_w_depa_dn8 = assign100190_e152321_d_n8;
        locals.var_w_depa_dn9 = assign100190_e152321_d_n9;
        locals.var_w_depa_dn10 = assign100190_e152321_d_n10;
        locals.var_w_depa_dn11 = assign100190_e152321_d_n11;
        locals.var_w_depa_dn14 = assign100190_e152321_d_n14;

        let (assign100200_e152329, assign100200_e152329_d_n0, assign100200_e152329_d_n2, assign100200_e152329_d_n4, assign100200_e152329_d_n5, assign100200_e152329_d_n6, assign100200_e152329_d_n7, assign100200_e152329_d_n8, assign100200_e152329_d_n9, assign100200_e152329_d_n10, assign100200_e152329_d_n11, assign100200_e152329_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100200_e152325: f64 = (p.p545 - locals.var_w_depa);
        let assign100200_e152327: f64 = (assign100200_e152325 - 1e-7);
        (assign100200_e152327, (-locals.var_w_depa_dn0), (-locals.var_w_depa_dn2), (-locals.var_w_depa_dn4), (-locals.var_w_depa_dn5), (-locals.var_w_depa_dn6), (-locals.var_w_depa_dn7), (-locals.var_w_depa_dn8), (-locals.var_w_depa_dn9), (-locals.var_w_depa_dn10), (-locals.var_w_depa_dn11), (-locals.var_w_depa_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign100200_e152329;
        locals.var_tmf1_dn0 = assign100200_e152329_d_n0;
        locals.var_tmf1_dn2 = assign100200_e152329_d_n2;
        locals.var_tmf1_dn4 = assign100200_e152329_d_n4;
        locals.var_tmf1_dn5 = assign100200_e152329_d_n5;
        locals.var_tmf1_dn6 = assign100200_e152329_d_n6;
        locals.var_tmf1_dn7 = assign100200_e152329_d_n7;
        locals.var_tmf1_dn8 = assign100200_e152329_d_n8;
        locals.var_tmf1_dn9 = assign100200_e152329_d_n9;
        locals.var_tmf1_dn10 = assign100200_e152329_d_n10;
        locals.var_tmf1_dn11 = assign100200_e152329_d_n11;
        locals.var_tmf1_dn14 = assign100200_e152329_d_n14;

        let (assign100210_e152337, assign100210_e152337_d_n0, assign100210_e152337_d_n2, assign100210_e152337_d_n4, assign100210_e152337_d_n5, assign100210_e152337_d_n6, assign100210_e152337_d_n7, assign100210_e152337_d_n8, assign100210_e152337_d_n9, assign100210_e152337_d_n10, assign100210_e152337_d_n11, assign100210_e152337_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100210_e152333: f64 = (4.0 * p.p545);
        let assign100210_e152335: f64 = (assign100210_e152333 * 1e-7);
        (assign100210_e152335, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100210_e152337;
        locals.var_tmf2_dn0 = assign100210_e152337_d_n0;
        locals.var_tmf2_dn2 = assign100210_e152337_d_n2;
        locals.var_tmf2_dn4 = assign100210_e152337_d_n4;
        locals.var_tmf2_dn5 = assign100210_e152337_d_n5;
        locals.var_tmf2_dn6 = assign100210_e152337_d_n6;
        locals.var_tmf2_dn7 = assign100210_e152337_d_n7;
        locals.var_tmf2_dn8 = assign100210_e152337_d_n8;
        locals.var_tmf2_dn9 = assign100210_e152337_d_n9;
        locals.var_tmf2_dn10 = assign100210_e152337_d_n10;
        locals.var_tmf2_dn11 = assign100210_e152337_d_n11;
        locals.var_tmf2_dn14 = assign100210_e152337_d_n14;

        let (assign100220_e152347, assign100220_e152347_d_n0, assign100220_e152347_d_n2, assign100220_e152347_d_n4, assign100220_e152347_d_n5, assign100220_e152347_d_n6, assign100220_e152347_d_n7, assign100220_e152347_d_n8, assign100220_e152347_d_n9, assign100220_e152347_d_n10, assign100220_e152347_d_n11, assign100220_e152347_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let (assign100220_e152345, assign100220_e152345_d_n0, assign100220_e152345_d_n2, assign100220_e152345_d_n4, assign100220_e152345_d_n5, assign100220_e152345_d_n6, assign100220_e152345_d_n7, assign100220_e152345_d_n8, assign100220_e152345_d_n9, assign100220_e152345_d_n10, assign100220_e152345_d_n11, assign100220_e152345_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign100220_e152344: f64 = (-locals.var_tmf2);
                (assign100220_e152344, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign100220_e152345, assign100220_e152345_d_n0, assign100220_e152345_d_n2, assign100220_e152345_d_n4, assign100220_e152345_d_n5, assign100220_e152345_d_n6, assign100220_e152345_d_n7, assign100220_e152345_d_n8, assign100220_e152345_d_n9, assign100220_e152345_d_n10, assign100220_e152345_d_n11, assign100220_e152345_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100220_e152347;
        locals.var_tmf2_dn0 = assign100220_e152347_d_n0;
        locals.var_tmf2_dn2 = assign100220_e152347_d_n2;
        locals.var_tmf2_dn4 = assign100220_e152347_d_n4;
        locals.var_tmf2_dn5 = assign100220_e152347_d_n5;
        locals.var_tmf2_dn6 = assign100220_e152347_d_n6;
        locals.var_tmf2_dn7 = assign100220_e152347_d_n7;
        locals.var_tmf2_dn8 = assign100220_e152347_d_n8;
        locals.var_tmf2_dn9 = assign100220_e152347_d_n9;
        locals.var_tmf2_dn10 = assign100220_e152347_d_n10;
        locals.var_tmf2_dn11 = assign100220_e152347_d_n11;
        locals.var_tmf2_dn14 = assign100220_e152347_d_n14;

        let (assign100230_e152356, assign100230_e152356_d_n0, assign100230_e152356_d_n2, assign100230_e152356_d_n4, assign100230_e152356_d_n5, assign100230_e152356_d_n6, assign100230_e152356_d_n7, assign100230_e152356_d_n8, assign100230_e152356_d_n9, assign100230_e152356_d_n10, assign100230_e152356_d_n11, assign100230_e152356_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100230_e152351: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign100230_e152353: f64 = (assign100230_e152351 + locals.var_tmf2);
        let assign100230_e152354: f64 = (assign100230_e152353).sqrt();
        (assign100230_e152354, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign100230_e152354)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign100230_e152354)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign100230_e152354)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign100230_e152354)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign100230_e152354)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign100230_e152354)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign100230_e152354)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign100230_e152354)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign100230_e152354)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign100230_e152354)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign100230_e152354)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100230_e152356;
        locals.var_tmf2_dn0 = assign100230_e152356_d_n0;
        locals.var_tmf2_dn2 = assign100230_e152356_d_n2;
        locals.var_tmf2_dn4 = assign100230_e152356_d_n4;
        locals.var_tmf2_dn5 = assign100230_e152356_d_n5;
        locals.var_tmf2_dn6 = assign100230_e152356_d_n6;
        locals.var_tmf2_dn7 = assign100230_e152356_d_n7;
        locals.var_tmf2_dn8 = assign100230_e152356_d_n8;
        locals.var_tmf2_dn9 = assign100230_e152356_d_n9;
        locals.var_tmf2_dn10 = assign100230_e152356_d_n10;
        locals.var_tmf2_dn11 = assign100230_e152356_d_n11;
        locals.var_tmf2_dn14 = assign100230_e152356_d_n14;

        let (assign100240_e152366, assign100240_e152366_d_n0, assign100240_e152366_d_n2, assign100240_e152366_d_n4, assign100240_e152366_d_n5, assign100240_e152366_d_n6, assign100240_e152366_d_n7, assign100240_e152366_d_n8, assign100240_e152366_d_n9, assign100240_e152366_d_n10, assign100240_e152366_d_n11, assign100240_e152366_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100240_e152362: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign100240_e152363: f64 = (1.0 + assign100240_e152362);
        let assign100240_e152364: f64 = (0.5 * assign100240_e152363);
        (assign100240_e152364, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100240_e152366;
        locals.var_t0_dn0 = assign100240_e152366_d_n0;
        locals.var_t0_dn2 = assign100240_e152366_d_n2;
        locals.var_t0_dn4 = assign100240_e152366_d_n4;
        locals.var_t0_dn5 = assign100240_e152366_d_n5;
        locals.var_t0_dn6 = assign100240_e152366_d_n6;
        locals.var_t0_dn7 = assign100240_e152366_d_n7;
        locals.var_t0_dn8 = assign100240_e152366_d_n8;
        locals.var_t0_dn9 = assign100240_e152366_d_n9;
        locals.var_t0_dn10 = assign100240_e152366_d_n10;
        locals.var_t0_dn11 = assign100240_e152366_d_n11;
        locals.var_t0_dn14 = assign100240_e152366_d_n14;

        let (assign100250_e152376, assign100250_e152376_d_n0, assign100250_e152376_d_n2, assign100250_e152376_d_n4, assign100250_e152376_d_n5, assign100250_e152376_d_n6, assign100250_e152376_d_n7, assign100250_e152376_d_n8, assign100250_e152376_d_n9, assign100250_e152376_d_n10, assign100250_e152376_d_n11, assign100250_e152376_d_n14,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100250_e152372: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign100250_e152373: f64 = (0.5 * assign100250_e152372);
        let assign100250_e152374: f64 = (p.p545 - assign100250_e152373);
        (assign100250_e152374, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn11, locals.var_w_depa_dn14,)
    }
};
        locals.var_w_depa = assign100250_e152376;
        locals.var_w_depa_dn0 = assign100250_e152376_d_n0;
        locals.var_w_depa_dn2 = assign100250_e152376_d_n2;
        locals.var_w_depa_dn4 = assign100250_e152376_d_n4;
        locals.var_w_depa_dn5 = assign100250_e152376_d_n5;
        locals.var_w_depa_dn6 = assign100250_e152376_d_n6;
        locals.var_w_depa_dn7 = assign100250_e152376_d_n7;
        locals.var_w_depa_dn8 = assign100250_e152376_d_n8;
        locals.var_w_depa_dn9 = assign100250_e152376_d_n9;
        locals.var_w_depa_dn10 = assign100250_e152376_d_n10;
        locals.var_w_depa_dn11 = assign100250_e152376_d_n11;
        locals.var_w_depa_dn14 = assign100250_e152376_d_n14;

        let assign100260_e152379: f64 = if p.p546 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2310 = assign100260_e152379;

        let (assign100270_e152387, assign100270_e152387_d_n0, assign100270_e152387_d_n2, assign100270_e152387_d_n4, assign100270_e152387_d_n5, assign100270_e152387_d_n6, assign100270_e152387_d_n7, assign100270_e152387_d_n8, assign100270_e152387_d_n9, assign100270_e152387_d_n10, assign100270_e152387_d_n11, assign100270_e152387_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100270_e152385: f64 = (locals.var_w_depa * p.p546);
        (assign100270_e152385, (locals.var_w_depa_dn0 * p.p546), (locals.var_w_depa_dn2 * p.p546), (locals.var_w_depa_dn4 * p.p546), (locals.var_w_depa_dn5 * p.p546), (locals.var_w_depa_dn6 * p.p546), (locals.var_w_depa_dn7 * p.p546), (locals.var_w_depa_dn8 * p.p546), (locals.var_w_depa_dn9 * p.p546), (locals.var_w_depa_dn10 * p.p546), (locals.var_w_depa_dn11 * p.p546), (locals.var_w_depa_dn14 * p.p546),)
    } else {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn11, locals.var_w_qs_a_dn14,)
    }
};
        locals.var_w_qs_a = assign100270_e152387;
        locals.var_w_qs_a_dn0 = assign100270_e152387_d_n0;
        locals.var_w_qs_a_dn2 = assign100270_e152387_d_n2;
        locals.var_w_qs_a_dn4 = assign100270_e152387_d_n4;
        locals.var_w_qs_a_dn5 = assign100270_e152387_d_n5;
        locals.var_w_qs_a_dn6 = assign100270_e152387_d_n6;
        locals.var_w_qs_a_dn7 = assign100270_e152387_d_n7;
        locals.var_w_qs_a_dn8 = assign100270_e152387_d_n8;
        locals.var_w_qs_a_dn9 = assign100270_e152387_d_n9;
        locals.var_w_qs_a_dn10 = assign100270_e152387_d_n10;
        locals.var_w_qs_a_dn11 = assign100270_e152387_d_n11;
        locals.var_w_qs_a_dn14 = assign100270_e152387_d_n14;

        let (assign100280_e152395, assign100280_e152395_d_n18,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100280_e152393: f64 = (p.p546 * (nv18 - 0.0));
        (assign100280_e152393, p.p546,)
    } else {
        (locals.var_w_nqs_a, locals.var_w_nqs_a_dn18,)
    }
};
        locals.var_w_nqs_a = assign100280_e152395;
        locals.var_w_nqs_a_dn18 = assign100280_e152395_d_n18;

        let (assign100290_e152405, assign100290_e152405_d_n0, assign100290_e152405_d_n2, assign100290_e152405_d_n4, assign100290_e152405_d_n5, assign100290_e152405_d_n6, assign100290_e152405_d_n7, assign100290_e152405_d_n8, assign100290_e152405_d_n9, assign100290_e152405_d_n10, assign100290_e152405_d_n11, assign100290_e152405_d_n14, assign100290_e152405_d_n18,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100290_e152401: f64 = (locals.var_w_nqs_a - locals.var_w_qs_a);
        let assign100290_e152403: f64 = (assign100290_e152401 / p.p546);
        (assign100290_e152403, ((-locals.var_w_qs_a_dn0) / p.p546), ((-locals.var_w_qs_a_dn2) / p.p546), ((-locals.var_w_qs_a_dn4) / p.p546), ((-locals.var_w_qs_a_dn5) / p.p546), ((-locals.var_w_qs_a_dn6) / p.p546), ((-locals.var_w_qs_a_dn7) / p.p546), ((-locals.var_w_qs_a_dn8) / p.p546), ((-locals.var_w_qs_a_dn9) / p.p546), ((-locals.var_w_qs_a_dn10) / p.p546), ((-locals.var_w_qs_a_dn11) / p.p546), ((-locals.var_w_qs_a_dn14) / p.p546), (locals.var_w_nqs_a_dn18 / p.p546),)
    } else {
        (locals.var_iwnqs0_a, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn11, locals.var_iwnqs0_a_dn14, locals.var_iwnqs0_a_dn18,)
    }
};
        locals.var_iwnqs0_a = assign100290_e152405;
        locals.var_iwnqs0_a_dn0 = assign100290_e152405_d_n0;
        locals.var_iwnqs0_a_dn2 = assign100290_e152405_d_n2;
        locals.var_iwnqs0_a_dn4 = assign100290_e152405_d_n4;
        locals.var_iwnqs0_a_dn5 = assign100290_e152405_d_n5;
        locals.var_iwnqs0_a_dn6 = assign100290_e152405_d_n6;
        locals.var_iwnqs0_a_dn7 = assign100290_e152405_d_n7;
        locals.var_iwnqs0_a_dn8 = assign100290_e152405_d_n8;
        locals.var_iwnqs0_a_dn9 = assign100290_e152405_d_n9;
        locals.var_iwnqs0_a_dn10 = assign100290_e152405_d_n10;
        locals.var_iwnqs0_a_dn11 = assign100290_e152405_d_n11;
        locals.var_iwnqs0_a_dn14 = assign100290_e152405_d_n14;
        locals.var_iwnqs0_a_dn18 = assign100290_e152405_d_n18;

    }
}
