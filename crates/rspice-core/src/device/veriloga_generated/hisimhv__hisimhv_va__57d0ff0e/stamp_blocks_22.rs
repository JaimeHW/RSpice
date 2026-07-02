#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_352(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95330_e147978, assign95330_e147978_d_n0, assign95330_e147978_d_n2, assign95330_e147978_d_n4, assign95330_e147978_d_n5, assign95330_e147978_d_n6, assign95330_e147978_d_n7, assign95330_e147978_d_n8, assign95330_e147978_d_n9, assign95330_e147978_d_n10, assign95330_e147978_d_n11, assign95330_e147978_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 == 0.0)) {
        let assign95330_e147976: f64 = (p.p63 * locals.var_t1);
        (assign95330_e147976, (p.p63 * locals.var_t1_dn0), (p.p63 * locals.var_t1_dn2), (p.p63 * locals.var_t1_dn4), (p.p63 * locals.var_t1_dn5), (p.p63 * locals.var_t1_dn6), (p.p63 * locals.var_t1_dn7), (p.p63 * locals.var_t1_dn8), (p.p63 * locals.var_t1_dn9), (p.p63 * locals.var_t1_dn10), (p.p63 * locals.var_t1_dn11), (p.p63 * locals.var_t1_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign95330_e147978;
        locals.var_t5_dn0 = assign95330_e147978_d_n0;
        locals.var_t5_dn2 = assign95330_e147978_d_n2;
        locals.var_t5_dn4 = assign95330_e147978_d_n4;
        locals.var_t5_dn5 = assign95330_e147978_d_n5;
        locals.var_t5_dn6 = assign95330_e147978_d_n6;
        locals.var_t5_dn7 = assign95330_e147978_d_n7;
        locals.var_t5_dn8 = assign95330_e147978_d_n8;
        locals.var_t5_dn9 = assign95330_e147978_d_n9;
        locals.var_t5_dn10 = assign95330_e147978_d_n10;
        locals.var_t5_dn11 = assign95330_e147978_d_n11;
        locals.var_t5_dn14 = assign95330_e147978_d_n14;

        let (assign95340_e147987, assign95340_e147987_d_n0, assign95340_e147987_d_n2, assign95340_e147987_d_n4, assign95340_e147987_d_n5, assign95340_e147987_d_n6, assign95340_e147987_d_n7, assign95340_e147987_d_n8, assign95340_e147987_d_n9, assign95340_e147987_d_n10, assign95340_e147987_d_n11, assign95340_e147987_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 == 0.0)) {
        let assign95340_e147985: f64 = (1.2 - locals.var_ps0);
        (assign95340_e147985, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn4), (-locals.var_ps0_dn5), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn8), (-locals.var_ps0_dn9), (-locals.var_ps0_dn10), (-locals.var_ps0_dn11), (-locals.var_ps0_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign95340_e147987;
        locals.var_t9_dn0 = assign95340_e147987_d_n0;
        locals.var_t9_dn2 = assign95340_e147987_d_n2;
        locals.var_t9_dn4 = assign95340_e147987_d_n4;
        locals.var_t9_dn5 = assign95340_e147987_d_n5;
        locals.var_t9_dn6 = assign95340_e147987_d_n6;
        locals.var_t9_dn7 = assign95340_e147987_d_n7;
        locals.var_t9_dn8 = assign95340_e147987_d_n8;
        locals.var_t9_dn9 = assign95340_e147987_d_n9;
        locals.var_t9_dn10 = assign95340_e147987_d_n10;
        locals.var_t9_dn11 = assign95340_e147987_d_n11;
        locals.var_t9_dn14 = assign95340_e147987_d_n14;

        let (assign95350_e148000, assign95350_e148000_d_n0, assign95350_e148000_d_n2, assign95350_e148000_d_n4, assign95350_e148000_d_n5, assign95350_e148000_d_n6, assign95350_e148000_d_n7, assign95350_e148000_d_n8, assign95350_e148000_d_n9, assign95350_e148000_d_n10, assign95350_e148000_d_n11, assign95350_e148000_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 == 0.0)) {
        let assign95350_e147994: f64 = (locals.var_vgs * locals.var_t5);
        let assign95350_e147997: f64 = (locals.var_t4 * locals.var_t9);
        let assign95350_e147998: f64 = (assign95350_e147994 - assign95350_e147997);
        (assign95350_e147998, ((locals.var_vgs * locals.var_t5_dn0) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((locals.var_vgs * locals.var_t5_dn2) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((locals.var_vgs * locals.var_t5_dn4) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((locals.var_vgs * locals.var_t5_dn5) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), (((locals.var_vgs_dn6 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), (((locals.var_vgs_dn7 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), (((locals.var_vgs_dn8 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((locals.var_vgs * locals.var_t5_dn9) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((locals.var_vgs * locals.var_t5_dn10) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((locals.var_vgs * locals.var_t5_dn11) - ((locals.var_t4_dn11 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn11))), ((locals.var_vgs * locals.var_t5_dn14) - ((locals.var_t4_dn14 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn14))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn8, locals.var_qgod_dn9, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn14,)
    }
};
        locals.var_qgod = assign95350_e148000;
        locals.var_qgod_dn0 = assign95350_e148000_d_n0;
        locals.var_qgod_dn2 = assign95350_e148000_d_n2;
        locals.var_qgod_dn4 = assign95350_e148000_d_n4;
        locals.var_qgod_dn5 = assign95350_e148000_d_n5;
        locals.var_qgod_dn6 = assign95350_e148000_d_n6;
        locals.var_qgod_dn7 = assign95350_e148000_d_n7;
        locals.var_qgod_dn8 = assign95350_e148000_d_n8;
        locals.var_qgod_dn9 = assign95350_e148000_d_n9;
        locals.var_qgod_dn10 = assign95350_e148000_d_n10;
        locals.var_qgod_dn11 = assign95350_e148000_d_n11;
        locals.var_qgod_dn14 = assign95350_e148000_d_n14;

        let (assign95360_e148007,) = {
    if (locals.var_cgso_given != 0.0) {
        let assign95360_e148004: f64 = (-locals.var_weffcv_nf);
        let assign95360_e148005: f64 = (locals.var_uc_cgso * assign95360_e148004);
        (assign95360_e148005,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95360_e148007;

        let assign95370_e148010: f64 = if locals.var_flg_coovlps == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2223 = assign95370_e148010;

        let (assign95380_e148022,) = {
    if ((locals.var_cgso_given == 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95380_e148016: f64 = (-locals.var_cox0);
        let assign95380_e148018: f64 = (assign95380_e148016 * p.p66);
        let assign95380_e148020: f64 = (assign95380_e148018 * locals.var_weffcv_nf);
        (assign95380_e148020,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95380_e148022;

        let assign95390_e148024: f64 = (-locals.var_cgsoe);
        let assign95390_e148026: f64 = (assign95390_e148024 * locals.var_vgsei);
        locals.var_qgso = assign95390_e148026;
        locals.var_qgso_dn2 = (assign95390_e148024 * locals.var_vgsei_dn2);
        locals.var_qgso_dn7 = (assign95390_e148024 * locals.var_vgsei_dn7);

        let (assign95400_e148033,) = {
    if (locals.var_cgdo_given != 0.0) {
        let assign95400_e148030: f64 = (-locals.var_weffcv_nf);
        let assign95400_e148031: f64 = (locals.var_uc_cgdo * assign95400_e148030);
        (assign95400_e148031,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95400_e148033;

        let assign95410_e148036: f64 = if locals.var_flg_coovlp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2224 = assign95410_e148036;

        let (assign95420_e148048,) = {
    if ((locals.var_cgdo_given == 0.0) && (locals.var_guard2224 != 0.0)) {
        let assign95420_e148042: f64 = (-locals.var_coxb0);
        let assign95420_e148044: f64 = (assign95420_e148042 * p.p63);
        let assign95420_e148046: f64 = (assign95420_e148044 * locals.var_weffcv_nf);
        (assign95420_e148046,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95420_e148048;

        let assign95430_e148050: f64 = (-locals.var_cgdoe);
        let assign95430_e148053: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign95430_e148054: f64 = (assign95430_e148050 * assign95430_e148053);
        locals.var_qgdo = assign95430_e148054;
        locals.var_qgdo_dn0 = (assign95430_e148050 * (-locals.var_vdsei_dn0));
        locals.var_qgdo_dn2 = (assign95430_e148050 * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qgdo_dn7 = (assign95430_e148050 * locals.var_vgsei_dn7);

        let assign95440_e148057: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2225 = assign95440_e148057;

        let (assign95450_e148065, assign95450_e148065_d_n0, assign95450_e148065_d_n2, assign95450_e148065_d_n4, assign95450_e148065_d_n5, assign95450_e148065_d_n6, assign95450_e148065_d_n7, assign95450_e148065_d_n8, assign95450_e148065_d_n9, assign95450_e148065_d_n10, assign95450_e148065_d_n11, assign95450_e148065_d_n14,) = {
    if (locals.var_guard2225 != 0.0) {
        let assign95450_e148062: f64 = (locals.var_vds - locals.var_pds);
        let assign95450_e148063: f64 = (p.p431 * assign95450_e148062);
        (assign95450_e148063, (p.p431 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (p.p431 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (p.p431 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (p.p431 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (p.p431 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (p.p431 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (p.p431 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (p.p431 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (p.p431 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (p.p431 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (p.p431 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn11, locals.var_qodad_dn14,)
    }
};
        locals.var_qodad = assign95450_e148065;
        locals.var_qodad_dn0 = assign95450_e148065_d_n0;
        locals.var_qodad_dn2 = assign95450_e148065_d_n2;
        locals.var_qodad_dn4 = assign95450_e148065_d_n4;
        locals.var_qodad_dn5 = assign95450_e148065_d_n5;
        locals.var_qodad_dn6 = assign95450_e148065_d_n6;
        locals.var_qodad_dn7 = assign95450_e148065_d_n7;
        locals.var_qodad_dn8 = assign95450_e148065_d_n8;
        locals.var_qodad_dn9 = assign95450_e148065_d_n9;
        locals.var_qodad_dn10 = assign95450_e148065_d_n10;
        locals.var_qodad_dn11 = assign95450_e148065_d_n11;
        locals.var_qodad_dn14 = assign95450_e148065_d_n14;

        let (assign95460_e148071, assign95460_e148071_d_n0, assign95460_e148071_d_n2, assign95460_e148071_d_n4, assign95460_e148071_d_n5, assign95460_e148071_d_n6, assign95460_e148071_d_n7, assign95460_e148071_d_n8, assign95460_e148071_d_n9, assign95460_e148071_d_n10, assign95460_e148071_d_n11, assign95460_e148071_d_n14,) = {
    if (locals.var_guard2225 != 0.0) {
        let assign95460_e148069: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95460_e148069, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qovd_add, locals.var_qovd_add_dn0, locals.var_qovd_add_dn2, locals.var_qovd_add_dn4, locals.var_qovd_add_dn5, locals.var_qovd_add_dn6, locals.var_qovd_add_dn7, locals.var_qovd_add_dn8, locals.var_qovd_add_dn9, locals.var_qovd_add_dn10, locals.var_qovd_add_dn11, locals.var_qovd_add_dn14,)
    }
};
        locals.var_qovd_add = assign95460_e148071;
        locals.var_qovd_add_dn0 = assign95460_e148071_d_n0;
        locals.var_qovd_add_dn2 = assign95460_e148071_d_n2;
        locals.var_qovd_add_dn4 = assign95460_e148071_d_n4;
        locals.var_qovd_add_dn5 = assign95460_e148071_d_n5;
        locals.var_qovd_add_dn6 = assign95460_e148071_d_n6;
        locals.var_qovd_add_dn7 = assign95460_e148071_d_n7;
        locals.var_qovd_add_dn8 = assign95460_e148071_d_n8;
        locals.var_qovd_add_dn9 = assign95460_e148071_d_n9;
        locals.var_qovd_add_dn10 = assign95460_e148071_d_n10;
        locals.var_qovd_add_dn11 = assign95460_e148071_d_n11;
        locals.var_qovd_add_dn14 = assign95460_e148071_d_n14;

        let (assign95470_e148077, assign95470_e148077_d_n0, assign95470_e148077_d_n2, assign95470_e148077_d_n4, assign95470_e148077_d_n5, assign95470_e148077_d_n6, assign95470_e148077_d_n7, assign95470_e148077_d_n8, assign95470_e148077_d_n9, assign95470_e148077_d_n10, assign95470_e148077_d_n11, assign95470_e148077_d_n14,) = {
    if (locals.var_guard2225 != 0.0) {
        let assign95470_e148075: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95470_e148075, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qbdld_add, locals.var_qbdld_add_dn0, locals.var_qbdld_add_dn2, locals.var_qbdld_add_dn4, locals.var_qbdld_add_dn5, locals.var_qbdld_add_dn6, locals.var_qbdld_add_dn7, locals.var_qbdld_add_dn8, locals.var_qbdld_add_dn9, locals.var_qbdld_add_dn10, locals.var_qbdld_add_dn11, locals.var_qbdld_add_dn14,)
    }
};
        locals.var_qbdld_add = assign95470_e148077;
        locals.var_qbdld_add_dn0 = assign95470_e148077_d_n0;
        locals.var_qbdld_add_dn2 = assign95470_e148077_d_n2;
        locals.var_qbdld_add_dn4 = assign95470_e148077_d_n4;
        locals.var_qbdld_add_dn5 = assign95470_e148077_d_n5;
        locals.var_qbdld_add_dn6 = assign95470_e148077_d_n6;
        locals.var_qbdld_add_dn7 = assign95470_e148077_d_n7;
        locals.var_qbdld_add_dn8 = assign95470_e148077_d_n8;
        locals.var_qbdld_add_dn9 = assign95470_e148077_d_n9;
        locals.var_qbdld_add_dn10 = assign95470_e148077_d_n10;
        locals.var_qbdld_add_dn11 = assign95470_e148077_d_n11;
        locals.var_qbdld_add_dn14 = assign95470_e148077_d_n14;

        let (assign95480_e148087, assign95480_e148087_d_n0, assign95480_e148087_d_n2, assign95480_e148087_d_n4, assign95480_e148087_d_n5, assign95480_e148087_d_n6, assign95480_e148087_d_n7, assign95480_e148087_d_n8, assign95480_e148087_d_n9, assign95480_e148087_d_n10, assign95480_e148087_d_n11, assign95480_e148087_d_n14,) = {
    if (locals.var_guard2225 == 0.0) {
        let assign95480_e148081: f64 = (-p.p431);
        let assign95480_e148084: f64 = (locals.var_vds - locals.var_pds);
        let assign95480_e148085: f64 = (assign95480_e148081 * assign95480_e148084);
        (assign95480_e148085, (assign95480_e148081 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (assign95480_e148081 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (assign95480_e148081 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (assign95480_e148081 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (assign95480_e148081 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (assign95480_e148081 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (assign95480_e148081 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (assign95480_e148081 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (assign95480_e148081 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (assign95480_e148081 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (assign95480_e148081 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn11, locals.var_qodad_dn14,)
    }
};
        locals.var_qodad = assign95480_e148087;
        locals.var_qodad_dn0 = assign95480_e148087_d_n0;
        locals.var_qodad_dn2 = assign95480_e148087_d_n2;
        locals.var_qodad_dn4 = assign95480_e148087_d_n4;
        locals.var_qodad_dn5 = assign95480_e148087_d_n5;
        locals.var_qodad_dn6 = assign95480_e148087_d_n6;
        locals.var_qodad_dn7 = assign95480_e148087_d_n7;
        locals.var_qodad_dn8 = assign95480_e148087_d_n8;
        locals.var_qodad_dn9 = assign95480_e148087_d_n9;
        locals.var_qodad_dn10 = assign95480_e148087_d_n10;
        locals.var_qodad_dn11 = assign95480_e148087_d_n11;
        locals.var_qodad_dn14 = assign95480_e148087_d_n14;

        let (assign95490_e148094, assign95490_e148094_d_n0, assign95490_e148094_d_n2, assign95490_e148094_d_n4, assign95490_e148094_d_n5, assign95490_e148094_d_n6, assign95490_e148094_d_n7, assign95490_e148094_d_n8, assign95490_e148094_d_n9, assign95490_e148094_d_n10, assign95490_e148094_d_n11, assign95490_e148094_d_n14,) = {
    if (locals.var_guard2225 == 0.0) {
        let assign95490_e148092: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95490_e148092, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qovs_add, locals.var_qovs_add_dn0, locals.var_qovs_add_dn2, locals.var_qovs_add_dn4, locals.var_qovs_add_dn5, locals.var_qovs_add_dn6, locals.var_qovs_add_dn7, locals.var_qovs_add_dn8, locals.var_qovs_add_dn9, locals.var_qovs_add_dn10, locals.var_qovs_add_dn11, locals.var_qovs_add_dn14,)
    }
};
        locals.var_qovs_add = assign95490_e148094;
        locals.var_qovs_add_dn0 = assign95490_e148094_d_n0;
        locals.var_qovs_add_dn2 = assign95490_e148094_d_n2;
        locals.var_qovs_add_dn4 = assign95490_e148094_d_n4;
        locals.var_qovs_add_dn5 = assign95490_e148094_d_n5;
        locals.var_qovs_add_dn6 = assign95490_e148094_d_n6;
        locals.var_qovs_add_dn7 = assign95490_e148094_d_n7;
        locals.var_qovs_add_dn8 = assign95490_e148094_d_n8;
        locals.var_qovs_add_dn9 = assign95490_e148094_d_n9;
        locals.var_qovs_add_dn10 = assign95490_e148094_d_n10;
        locals.var_qovs_add_dn11 = assign95490_e148094_d_n11;
        locals.var_qovs_add_dn14 = assign95490_e148094_d_n14;

        let (assign95500_e148101, assign95500_e148101_d_n0, assign95500_e148101_d_n2, assign95500_e148101_d_n4, assign95500_e148101_d_n5, assign95500_e148101_d_n6, assign95500_e148101_d_n7, assign95500_e148101_d_n8, assign95500_e148101_d_n9, assign95500_e148101_d_n10, assign95500_e148101_d_n11, assign95500_e148101_d_n14,) = {
    if (locals.var_guard2225 == 0.0) {
        let assign95500_e148099: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95500_e148099, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qbsld_add, locals.var_qbsld_add_dn0, locals.var_qbsld_add_dn2, locals.var_qbsld_add_dn4, locals.var_qbsld_add_dn5, locals.var_qbsld_add_dn6, locals.var_qbsld_add_dn7, locals.var_qbsld_add_dn8, locals.var_qbsld_add_dn9, locals.var_qbsld_add_dn10, locals.var_qbsld_add_dn11, locals.var_qbsld_add_dn14,)
    }
};
        locals.var_qbsld_add = assign95500_e148101;
        locals.var_qbsld_add_dn0 = assign95500_e148101_d_n0;
        locals.var_qbsld_add_dn2 = assign95500_e148101_d_n2;
        locals.var_qbsld_add_dn4 = assign95500_e148101_d_n4;
        locals.var_qbsld_add_dn5 = assign95500_e148101_d_n5;
        locals.var_qbsld_add_dn6 = assign95500_e148101_d_n6;
        locals.var_qbsld_add_dn7 = assign95500_e148101_d_n7;
        locals.var_qbsld_add_dn8 = assign95500_e148101_d_n8;
        locals.var_qbsld_add_dn9 = assign95500_e148101_d_n9;
        locals.var_qbsld_add_dn10 = assign95500_e148101_d_n10;
        locals.var_qbsld_add_dn11 = assign95500_e148101_d_n11;
        locals.var_qbsld_add_dn14 = assign95500_e148101_d_n14;

        let assign95510_e148103: f64 = (-locals.var_uc_cgbo);
        let assign95510_e148105: f64 = (assign95510_e148103 * locals.var_lgate);
        locals.var_cgbo_loc = assign95510_e148105;

        let assign95520_e148107: f64 = (-locals.var_cgbo_loc);
        let assign95520_e148110: f64 = (locals.var_vgsi - locals.var_vbsi);
        let assign95520_e148111: f64 = (assign95520_e148107 * assign95520_e148110);
        locals.var_qgbo = assign95520_e148111;
        locals.var_qgbo_dn7 = (assign95520_e148107 * locals.var_vgsi_dn7);
        locals.var_qgbo_dn8 = (assign95520_e148107 * (locals.var_vgsi_dn8 - locals.var_vbsi_dn8));
        locals.var_qgbo_dn9 = (assign95520_e148107 * (-locals.var_vbsi_dn9));

        locals.var_aclm = locals.var_uc_clm1;

        let assign95540_e148115: f64 = if locals.var_flg_noqi != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2226 = assign95540_e148115;

        let (assign95550_e148129, assign95550_e148129_d_n0, assign95550_e148129_d_n2, assign95550_e148129_d_n4, assign95550_e148129_d_n5, assign95550_e148129_d_n6, assign95550_e148129_d_n7, assign95550_e148129_d_n8, assign95550_e148129_d_n9, assign95550_e148129_d_n10, assign95550_e148129_d_n11, assign95550_e148129_d_n14,) = {
    if (locals.var_guard2226 != 0.0) {
        let assign95550_e148120: f64 = (locals.var_vds + locals.var_ps0);
        let assign95550_e148121: f64 = (locals.var_aclm * assign95550_e148120);
        let assign95550_e148124: f64 = (1.0 - locals.var_aclm);
        let assign95550_e148126: f64 = (assign95550_e148124 * locals.var_psl);
        let assign95550_e148127: f64 = (assign95550_e148121 + assign95550_e148126);
        (assign95550_e148127, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign95550_e148124 * locals.var_psl_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign95550_e148124 * locals.var_psl_dn2)), ((locals.var_aclm * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + (assign95550_e148124 * locals.var_psl_dn4)), ((locals.var_aclm * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + (assign95550_e148124 * locals.var_psl_dn5)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign95550_e148124 * locals.var_psl_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign95550_e148124 * locals.var_psl_dn7)), ((locals.var_aclm * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + (assign95550_e148124 * locals.var_psl_dn8)), ((locals.var_aclm * (locals.var_vds_dn9 + locals.var_ps0_dn9)) + (assign95550_e148124 * locals.var_psl_dn9)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign95550_e148124 * locals.var_psl_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign95550_e148124 * locals.var_psl_dn11)), ((locals.var_aclm * (locals.var_vds_dn14 + locals.var_ps0_dn14)) + (assign95550_e148124 * locals.var_psl_dn14)),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95550_e148129;
        locals.var_psdl_dn0 = assign95550_e148129_d_n0;
        locals.var_psdl_dn2 = assign95550_e148129_d_n2;
        locals.var_psdl_dn4 = assign95550_e148129_d_n4;
        locals.var_psdl_dn5 = assign95550_e148129_d_n5;
        locals.var_psdl_dn6 = assign95550_e148129_d_n6;
        locals.var_psdl_dn7 = assign95550_e148129_d_n7;
        locals.var_psdl_dn8 = assign95550_e148129_d_n8;
        locals.var_psdl_dn9 = assign95550_e148129_d_n9;
        locals.var_psdl_dn10 = assign95550_e148129_d_n10;
        locals.var_psdl_dn11 = assign95550_e148129_d_n11;
        locals.var_psdl_dn14 = assign95550_e148129_d_n14;

        let assign95560_e148133: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95560_e148136: f64 = (10.0 * 2.220446049250313e-16);
        let assign95560_e148137: f64 = (assign95560_e148133 - assign95560_e148136);
        let assign95560_e148140: f64 = (10.0 * 2.220446049250313e-16);
        let assign95560_e148141: f64 = (assign95560_e148137 - assign95560_e148140);
        let assign95560_e148145: f64 = (10.0 * 2.220446049250313e-16);
        let assign95560_e148148: f64 = if ((locals.var_psdl > assign95560_e148141) && (assign95560_e148145 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2227 = assign95560_e148148;

        let (assign95570_e148166, assign95570_e148166_d_n0, assign95570_e148166_d_n2, assign95570_e148166_d_n4, assign95570_e148166_d_n5, assign95570_e148166_d_n6, assign95570_e148166_d_n7, assign95570_e148166_d_n8, assign95570_e148166_d_n9, assign95570_e148166_d_n10, assign95570_e148166_d_n11, assign95570_e148166_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95570_e148155: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95570_e148158: f64 = (10.0 * 2.220446049250313e-16);
        let assign95570_e148159: f64 = (assign95570_e148155 - assign95570_e148158);
        let assign95570_e148160: f64 = (locals.var_psdl - assign95570_e148159);
        let assign95570_e148163: f64 = (10.0 * 2.220446049250313e-16);
        let assign95570_e148164: f64 = (assign95570_e148160 + assign95570_e148163);
        (assign95570_e148164, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn11 - (locals.var_ps0_dn11 + locals.var_vds_dn11)), (locals.var_psdl_dn14 - (locals.var_ps0_dn14 + locals.var_vds_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign95570_e148166;
        locals.var_tmf1_dn0 = assign95570_e148166_d_n0;
        locals.var_tmf1_dn2 = assign95570_e148166_d_n2;
        locals.var_tmf1_dn4 = assign95570_e148166_d_n4;
        locals.var_tmf1_dn5 = assign95570_e148166_d_n5;
        locals.var_tmf1_dn6 = assign95570_e148166_d_n6;
        locals.var_tmf1_dn7 = assign95570_e148166_d_n7;
        locals.var_tmf1_dn8 = assign95570_e148166_d_n8;
        locals.var_tmf1_dn9 = assign95570_e148166_d_n9;
        locals.var_tmf1_dn10 = assign95570_e148166_d_n10;
        locals.var_tmf1_dn11 = assign95570_e148166_d_n11;
        locals.var_tmf1_dn14 = assign95570_e148166_d_n14;

        let (assign95580_e148174, assign95580_e148174_d_n0, assign95580_e148174_d_n2, assign95580_e148174_d_n4, assign95580_e148174_d_n5, assign95580_e148174_d_n6, assign95580_e148174_d_n7, assign95580_e148174_d_n8, assign95580_e148174_d_n9, assign95580_e148174_d_n10, assign95580_e148174_d_n11, assign95580_e148174_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95580_e148172: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign95580_e148172, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign95580_e148174;
        locals.var_x2_dn0 = assign95580_e148174_d_n0;
        locals.var_x2_dn2 = assign95580_e148174_d_n2;
        locals.var_x2_dn4 = assign95580_e148174_d_n4;
        locals.var_x2_dn5 = assign95580_e148174_d_n5;
        locals.var_x2_dn6 = assign95580_e148174_d_n6;
        locals.var_x2_dn7 = assign95580_e148174_d_n7;
        locals.var_x2_dn8 = assign95580_e148174_d_n8;
        locals.var_x2_dn9 = assign95580_e148174_d_n9;
        locals.var_x2_dn10 = assign95580_e148174_d_n10;
        locals.var_x2_dn11 = assign95580_e148174_d_n11;
        locals.var_x2_dn14 = assign95580_e148174_d_n14;

        let (assign95590_e148186, assign95590_e148186_d_n0, assign95590_e148186_d_n2, assign95590_e148186_d_n4, assign95590_e148186_d_n5, assign95590_e148186_d_n6, assign95590_e148186_d_n7, assign95590_e148186_d_n8, assign95590_e148186_d_n9, assign95590_e148186_d_n10, assign95590_e148186_d_n11, assign95590_e148186_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95590_e148180: f64 = (10.0 * 2.220446049250313e-16);
        let assign95590_e148183: f64 = (10.0 * 2.220446049250313e-16);
        let assign95590_e148184: f64 = (assign95590_e148180 * assign95590_e148183);
        (assign95590_e148184, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign95590_e148186;
        locals.var_xmax2_dn0 = assign95590_e148186_d_n0;
        locals.var_xmax2_dn2 = assign95590_e148186_d_n2;
        locals.var_xmax2_dn4 = assign95590_e148186_d_n4;
        locals.var_xmax2_dn5 = assign95590_e148186_d_n5;
        locals.var_xmax2_dn6 = assign95590_e148186_d_n6;
        locals.var_xmax2_dn7 = assign95590_e148186_d_n7;
        locals.var_xmax2_dn8 = assign95590_e148186_d_n8;
        locals.var_xmax2_dn9 = assign95590_e148186_d_n9;
        locals.var_xmax2_dn10 = assign95590_e148186_d_n10;
        locals.var_xmax2_dn11 = assign95590_e148186_d_n11;
        locals.var_xmax2_dn14 = assign95590_e148186_d_n14;

        let (assign95600_e148192, assign95600_e148192_d_n0, assign95600_e148192_d_n2, assign95600_e148192_d_n4, assign95600_e148192_d_n5, assign95600_e148192_d_n6, assign95600_e148192_d_n7, assign95600_e148192_d_n8, assign95600_e148192_d_n9, assign95600_e148192_d_n10, assign95600_e148192_d_n11, assign95600_e148192_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95600_e148192;
        locals.var_xp_dn0 = assign95600_e148192_d_n0;
        locals.var_xp_dn2 = assign95600_e148192_d_n2;
        locals.var_xp_dn4 = assign95600_e148192_d_n4;
        locals.var_xp_dn5 = assign95600_e148192_d_n5;
        locals.var_xp_dn6 = assign95600_e148192_d_n6;
        locals.var_xp_dn7 = assign95600_e148192_d_n7;
        locals.var_xp_dn8 = assign95600_e148192_d_n8;
        locals.var_xp_dn9 = assign95600_e148192_d_n9;
        locals.var_xp_dn10 = assign95600_e148192_d_n10;
        locals.var_xp_dn11 = assign95600_e148192_d_n11;
        locals.var_xp_dn14 = assign95600_e148192_d_n14;

        let (assign95610_e148198, assign95610_e148198_d_n0, assign95610_e148198_d_n2, assign95610_e148198_d_n4, assign95610_e148198_d_n5, assign95610_e148198_d_n6, assign95610_e148198_d_n7, assign95610_e148198_d_n8, assign95610_e148198_d_n9, assign95610_e148198_d_n10, assign95610_e148198_d_n11, assign95610_e148198_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95610_e148198;
        locals.var_xmp_dn0 = assign95610_e148198_d_n0;
        locals.var_xmp_dn2 = assign95610_e148198_d_n2;
        locals.var_xmp_dn4 = assign95610_e148198_d_n4;
        locals.var_xmp_dn5 = assign95610_e148198_d_n5;
        locals.var_xmp_dn6 = assign95610_e148198_d_n6;
        locals.var_xmp_dn7 = assign95610_e148198_d_n7;
        locals.var_xmp_dn8 = assign95610_e148198_d_n8;
        locals.var_xmp_dn9 = assign95610_e148198_d_n9;
        locals.var_xmp_dn10 = assign95610_e148198_d_n10;
        locals.var_xmp_dn11 = assign95610_e148198_d_n11;
        locals.var_xmp_dn14 = assign95610_e148198_d_n14;

        let (assign95620_e148204,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95620_e148204;

        let (assign95630_e148210,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95630_e148210;

        let (assign95640_e148216, assign95640_e148216_d_n0, assign95640_e148216_d_n2, assign95640_e148216_d_n4, assign95640_e148216_d_n5, assign95640_e148216_d_n6, assign95640_e148216_d_n7, assign95640_e148216_d_n8, assign95640_e148216_d_n9, assign95640_e148216_d_n10, assign95640_e148216_d_n11, assign95640_e148216_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign95640_e148216;
        locals.var_arg_dn0 = assign95640_e148216_d_n0;
        locals.var_arg_dn2 = assign95640_e148216_d_n2;
        locals.var_arg_dn4 = assign95640_e148216_d_n4;
        locals.var_arg_dn5 = assign95640_e148216_d_n5;
        locals.var_arg_dn6 = assign95640_e148216_d_n6;
        locals.var_arg_dn7 = assign95640_e148216_d_n7;
        locals.var_arg_dn8 = assign95640_e148216_d_n8;
        locals.var_arg_dn9 = assign95640_e148216_d_n9;
        locals.var_arg_dn10 = assign95640_e148216_d_n10;
        locals.var_arg_dn11 = assign95640_e148216_d_n11;
        locals.var_arg_dn14 = assign95640_e148216_d_n14;

        let (assign95650_e148222, assign95650_e148222_d_n0, assign95650_e148222_d_n2, assign95650_e148222_d_n4, assign95650_e148222_d_n5, assign95650_e148222_d_n6, assign95650_e148222_d_n7, assign95650_e148222_d_n8, assign95650_e148222_d_n9, assign95650_e148222_d_n10, assign95650_e148222_d_n11, assign95650_e148222_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95650_e148222;
        locals.var_dnm_dn0 = assign95650_e148222_d_n0;
        locals.var_dnm_dn2 = assign95650_e148222_d_n2;
        locals.var_dnm_dn4 = assign95650_e148222_d_n4;
        locals.var_dnm_dn5 = assign95650_e148222_d_n5;
        locals.var_dnm_dn6 = assign95650_e148222_d_n6;
        locals.var_dnm_dn7 = assign95650_e148222_d_n7;
        locals.var_dnm_dn8 = assign95650_e148222_d_n8;
        locals.var_dnm_dn9 = assign95650_e148222_d_n9;
        locals.var_dnm_dn10 = assign95650_e148222_d_n10;
        locals.var_dnm_dn11 = assign95650_e148222_d_n11;
        locals.var_dnm_dn14 = assign95650_e148222_d_n14;

        let (assign95660_e148230, assign95660_e148230_d_n0, assign95660_e148230_d_n2, assign95660_e148230_d_n4, assign95660_e148230_d_n5, assign95660_e148230_d_n6, assign95660_e148230_d_n7, assign95660_e148230_d_n8, assign95660_e148230_d_n9, assign95660_e148230_d_n10, assign95660_e148230_d_n11, assign95660_e148230_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95660_e148228: f64 = (locals.var_xp * locals.var_x2);
        (assign95660_e148228, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95660_e148230;
        locals.var_xp_dn0 = assign95660_e148230_d_n0;
        locals.var_xp_dn2 = assign95660_e148230_d_n2;
        locals.var_xp_dn4 = assign95660_e148230_d_n4;
        locals.var_xp_dn5 = assign95660_e148230_d_n5;
        locals.var_xp_dn6 = assign95660_e148230_d_n6;
        locals.var_xp_dn7 = assign95660_e148230_d_n7;
        locals.var_xp_dn8 = assign95660_e148230_d_n8;
        locals.var_xp_dn9 = assign95660_e148230_d_n9;
        locals.var_xp_dn10 = assign95660_e148230_d_n10;
        locals.var_xp_dn11 = assign95660_e148230_d_n11;
        locals.var_xp_dn14 = assign95660_e148230_d_n14;

    }

    pub(super) fn stamp_transient_block_353(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95670_e148238, assign95670_e148238_d_n0, assign95670_e148238_d_n2, assign95670_e148238_d_n4, assign95670_e148238_d_n5, assign95670_e148238_d_n6, assign95670_e148238_d_n7, assign95670_e148238_d_n8, assign95670_e148238_d_n9, assign95670_e148238_d_n10, assign95670_e148238_d_n11, assign95670_e148238_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95670_e148236: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95670_e148236, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95670_e148238;
        locals.var_xmp_dn0 = assign95670_e148238_d_n0;
        locals.var_xmp_dn2 = assign95670_e148238_d_n2;
        locals.var_xmp_dn4 = assign95670_e148238_d_n4;
        locals.var_xmp_dn5 = assign95670_e148238_d_n5;
        locals.var_xmp_dn6 = assign95670_e148238_d_n6;
        locals.var_xmp_dn7 = assign95670_e148238_d_n7;
        locals.var_xmp_dn8 = assign95670_e148238_d_n8;
        locals.var_xmp_dn9 = assign95670_e148238_d_n9;
        locals.var_xmp_dn10 = assign95670_e148238_d_n10;
        locals.var_xmp_dn11 = assign95670_e148238_d_n11;
        locals.var_xmp_dn14 = assign95670_e148238_d_n14;

        let (assign95680_e148246, assign95680_e148246_d_n0, assign95680_e148246_d_n2, assign95680_e148246_d_n4, assign95680_e148246_d_n5, assign95680_e148246_d_n6, assign95680_e148246_d_n7, assign95680_e148246_d_n8, assign95680_e148246_d_n9, assign95680_e148246_d_n10, assign95680_e148246_d_n11, assign95680_e148246_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95680_e148244: f64 = (locals.var_xp * locals.var_x2);
        (assign95680_e148244, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95680_e148246;
        locals.var_xp_dn0 = assign95680_e148246_d_n0;
        locals.var_xp_dn2 = assign95680_e148246_d_n2;
        locals.var_xp_dn4 = assign95680_e148246_d_n4;
        locals.var_xp_dn5 = assign95680_e148246_d_n5;
        locals.var_xp_dn6 = assign95680_e148246_d_n6;
        locals.var_xp_dn7 = assign95680_e148246_d_n7;
        locals.var_xp_dn8 = assign95680_e148246_d_n8;
        locals.var_xp_dn9 = assign95680_e148246_d_n9;
        locals.var_xp_dn10 = assign95680_e148246_d_n10;
        locals.var_xp_dn11 = assign95680_e148246_d_n11;
        locals.var_xp_dn14 = assign95680_e148246_d_n14;

        let (assign95690_e148254, assign95690_e148254_d_n0, assign95690_e148254_d_n2, assign95690_e148254_d_n4, assign95690_e148254_d_n5, assign95690_e148254_d_n6, assign95690_e148254_d_n7, assign95690_e148254_d_n8, assign95690_e148254_d_n9, assign95690_e148254_d_n10, assign95690_e148254_d_n11, assign95690_e148254_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95690_e148252: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95690_e148252, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95690_e148254;
        locals.var_xmp_dn0 = assign95690_e148254_d_n0;
        locals.var_xmp_dn2 = assign95690_e148254_d_n2;
        locals.var_xmp_dn4 = assign95690_e148254_d_n4;
        locals.var_xmp_dn5 = assign95690_e148254_d_n5;
        locals.var_xmp_dn6 = assign95690_e148254_d_n6;
        locals.var_xmp_dn7 = assign95690_e148254_d_n7;
        locals.var_xmp_dn8 = assign95690_e148254_d_n8;
        locals.var_xmp_dn9 = assign95690_e148254_d_n9;
        locals.var_xmp_dn10 = assign95690_e148254_d_n10;
        locals.var_xmp_dn11 = assign95690_e148254_d_n11;
        locals.var_xmp_dn14 = assign95690_e148254_d_n14;

        let (assign95700_e148262, assign95700_e148262_d_n0, assign95700_e148262_d_n2, assign95700_e148262_d_n4, assign95700_e148262_d_n5, assign95700_e148262_d_n6, assign95700_e148262_d_n7, assign95700_e148262_d_n8, assign95700_e148262_d_n9, assign95700_e148262_d_n10, assign95700_e148262_d_n11, assign95700_e148262_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95700_e148260: f64 = (locals.var_xp + locals.var_xmp);
        (assign95700_e148260, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign95700_e148262;
        locals.var_arg_dn0 = assign95700_e148262_d_n0;
        locals.var_arg_dn2 = assign95700_e148262_d_n2;
        locals.var_arg_dn4 = assign95700_e148262_d_n4;
        locals.var_arg_dn5 = assign95700_e148262_d_n5;
        locals.var_arg_dn6 = assign95700_e148262_d_n6;
        locals.var_arg_dn7 = assign95700_e148262_d_n7;
        locals.var_arg_dn8 = assign95700_e148262_d_n8;
        locals.var_arg_dn9 = assign95700_e148262_d_n9;
        locals.var_arg_dn10 = assign95700_e148262_d_n10;
        locals.var_arg_dn11 = assign95700_e148262_d_n11;
        locals.var_arg_dn14 = assign95700_e148262_d_n14;

        let (assign95710_e148268, assign95710_e148268_d_n0, assign95710_e148268_d_n2, assign95710_e148268_d_n4, assign95710_e148268_d_n5, assign95710_e148268_d_n6, assign95710_e148268_d_n7, assign95710_e148268_d_n8, assign95710_e148268_d_n9, assign95710_e148268_d_n10, assign95710_e148268_d_n11, assign95710_e148268_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95710_e148268;
        locals.var_dnm_dn0 = assign95710_e148268_d_n0;
        locals.var_dnm_dn2 = assign95710_e148268_d_n2;
        locals.var_dnm_dn4 = assign95710_e148268_d_n4;
        locals.var_dnm_dn5 = assign95710_e148268_d_n5;
        locals.var_dnm_dn6 = assign95710_e148268_d_n6;
        locals.var_dnm_dn7 = assign95710_e148268_d_n7;
        locals.var_dnm_dn8 = assign95710_e148268_d_n8;
        locals.var_dnm_dn9 = assign95710_e148268_d_n9;
        locals.var_dnm_dn10 = assign95710_e148268_d_n10;
        locals.var_dnm_dn11 = assign95710_e148268_d_n11;
        locals.var_dnm_dn14 = assign95710_e148268_d_n14;

        let assign95720_e148283: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2228 = assign95720_e148283;

        let assign95730_e148286: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2229 = assign95730_e148286;

        let (assign95740_e148296,) = {
    if ((((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) && (locals.var_guard2229 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95740_e148296;

        let assign95750_e148299: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2230 = assign95750_e148299;

        let (assign95760_e148312,) = {
    if (((((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2230 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95760_e148312;

        let assign95770_e148315: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2231 = assign95770_e148315;

        let (assign95780_e148331,) = {
    if ((((((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2230 == 0.0)) && (locals.var_guard2231 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95780_e148331;

        let assign95790_e148334: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2232 = assign95790_e148334;

        let (assign95800_e148353,) = {
    if (((((((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2230 == 0.0)) && (locals.var_guard2231 == 0.0)) && (locals.var_guard2232 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95800_e148353;

        let (assign95810_e148361,) = {
    if (((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95810_e148361;

        let mut assign95820_loop_guard: usize = 0;
        while {
            let assign95820_cond_e148370: f64 = if ((((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign95820_cond_e148370 != 0.0
        } {
            assign95820_loop_guard += 1;
            assert!(assign95820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign95820_body0_e148379, assign95820_body0_e148379_d_n0, assign95820_body0_e148379_d_n2, assign95820_body0_e148379_d_n4, assign95820_body0_e148379_d_n5, assign95820_body0_e148379_d_n6, assign95820_body0_e148379_d_n7, assign95820_body0_e148379_d_n8, assign95820_body0_e148379_d_n9, assign95820_body0_e148379_d_n10, assign95820_body0_e148379_d_n11, assign95820_body0_e148379_d_n14,) = {
    if (((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) {
        let assign95820_body0_e148377: f64 = (locals.var_dnm).sqrt();
        (assign95820_body0_e148377, (locals.var_dnm_dn0 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn2 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn4 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn5 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn6 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn7 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn8 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn9 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn10 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn11 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn14 / (2.0 * assign95820_body0_e148377)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign95820_body0_e148379;
            locals.var_dnm_dn0 = assign95820_body0_e148379_d_n0;
            locals.var_dnm_dn2 = assign95820_body0_e148379_d_n2;
            locals.var_dnm_dn4 = assign95820_body0_e148379_d_n4;
            locals.var_dnm_dn5 = assign95820_body0_e148379_d_n5;
            locals.var_dnm_dn6 = assign95820_body0_e148379_d_n6;
            locals.var_dnm_dn7 = assign95820_body0_e148379_d_n7;
            locals.var_dnm_dn8 = assign95820_body0_e148379_d_n8;
            locals.var_dnm_dn9 = assign95820_body0_e148379_d_n9;
            locals.var_dnm_dn10 = assign95820_body0_e148379_d_n10;
            locals.var_dnm_dn11 = assign95820_body0_e148379_d_n11;
            locals.var_dnm_dn14 = assign95820_body0_e148379_d_n14;
            let (assign95820_body1_e148389,) = {
    if (((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) {
        let assign95820_body1_e148387: f64 = (locals.var_m0 + 1.0);
        (assign95820_body1_e148387,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign95820_body1_e148389;
        }

        let (assign95830_e148409, assign95830_e148409_d_n0, assign95830_e148409_d_n2, assign95830_e148409_d_n4, assign95830_e148409_d_n5, assign95830_e148409_d_n6, assign95830_e148409_d_n7, assign95830_e148409_d_n8, assign95830_e148409_d_n9, assign95830_e148409_d_n10, assign95830_e148409_d_n11, assign95830_e148409_d_n14,) = {
    if (((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 == 0.0)) {
        let (assign95830_e148407, assign95830_e148407_d_n0, assign95830_e148407_d_n2, assign95830_e148407_d_n4, assign95830_e148407_d_n5, assign95830_e148407_d_n6, assign95830_e148407_d_n7, assign95830_e148407_d_n8, assign95830_e148407_d_n9, assign95830_e148407_d_n10, assign95830_e148407_d_n11, assign95830_e148407_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign95830_e148404: f64 = (2.0 * 2.0);
                let assign95830_e148405: f64 = (1.0 / assign95830_e148404);
                let assign95830_e148406: f64 = (locals.var_dnm).powf(assign95830_e148405);
                (assign95830_e148406, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn0)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn2)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn4)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn5)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn6)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn7)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn8)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn9)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn10)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn11)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn14)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign95830_e148407, assign95830_e148407_d_n0, assign95830_e148407_d_n2, assign95830_e148407_d_n4, assign95830_e148407_d_n5, assign95830_e148407_d_n6, assign95830_e148407_d_n7, assign95830_e148407_d_n8, assign95830_e148407_d_n9, assign95830_e148407_d_n10, assign95830_e148407_d_n11, assign95830_e148407_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95830_e148409;
        locals.var_dnm_dn0 = assign95830_e148409_d_n0;
        locals.var_dnm_dn2 = assign95830_e148409_d_n2;
        locals.var_dnm_dn4 = assign95830_e148409_d_n4;
        locals.var_dnm_dn5 = assign95830_e148409_d_n5;
        locals.var_dnm_dn6 = assign95830_e148409_d_n6;
        locals.var_dnm_dn7 = assign95830_e148409_d_n7;
        locals.var_dnm_dn8 = assign95830_e148409_d_n8;
        locals.var_dnm_dn9 = assign95830_e148409_d_n9;
        locals.var_dnm_dn10 = assign95830_e148409_d_n10;
        locals.var_dnm_dn11 = assign95830_e148409_d_n11;
        locals.var_dnm_dn14 = assign95830_e148409_d_n14;

        let (assign95840_e148417, assign95840_e148417_d_n0, assign95840_e148417_d_n2, assign95840_e148417_d_n4, assign95840_e148417_d_n5, assign95840_e148417_d_n6, assign95840_e148417_d_n7, assign95840_e148417_d_n8, assign95840_e148417_d_n9, assign95840_e148417_d_n10, assign95840_e148417_d_n11, assign95840_e148417_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95840_e148415: f64 = (1.0 / locals.var_dnm);
        (assign95840_e148415, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95840_e148417;
        locals.var_dnm_dn0 = assign95840_e148417_d_n0;
        locals.var_dnm_dn2 = assign95840_e148417_d_n2;
        locals.var_dnm_dn4 = assign95840_e148417_d_n4;
        locals.var_dnm_dn5 = assign95840_e148417_d_n5;
        locals.var_dnm_dn6 = assign95840_e148417_d_n6;
        locals.var_dnm_dn7 = assign95840_e148417_d_n7;
        locals.var_dnm_dn8 = assign95840_e148417_d_n8;
        locals.var_dnm_dn9 = assign95840_e148417_d_n9;
        locals.var_dnm_dn10 = assign95840_e148417_d_n10;
        locals.var_dnm_dn11 = assign95840_e148417_d_n11;
        locals.var_dnm_dn14 = assign95840_e148417_d_n14;

        let (assign95850_e148429, assign95850_e148429_d_n0, assign95850_e148429_d_n2, assign95850_e148429_d_n4, assign95850_e148429_d_n5, assign95850_e148429_d_n6, assign95850_e148429_d_n7, assign95850_e148429_d_n8, assign95850_e148429_d_n9, assign95850_e148429_d_n10, assign95850_e148429_d_n11, assign95850_e148429_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95850_e148424: f64 = (10.0 * 2.220446049250313e-16);
        let assign95850_e148425: f64 = (locals.var_tmf1 * assign95850_e148424);
        let assign95850_e148427: f64 = (assign95850_e148425 * locals.var_dnm);
        (assign95850_e148427, (((locals.var_tmf1_dn0 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign95850_e148429;
        locals.var_tmf0_dn0 = assign95850_e148429_d_n0;
        locals.var_tmf0_dn2 = assign95850_e148429_d_n2;
        locals.var_tmf0_dn4 = assign95850_e148429_d_n4;
        locals.var_tmf0_dn5 = assign95850_e148429_d_n5;
        locals.var_tmf0_dn6 = assign95850_e148429_d_n6;
        locals.var_tmf0_dn7 = assign95850_e148429_d_n7;
        locals.var_tmf0_dn8 = assign95850_e148429_d_n8;
        locals.var_tmf0_dn9 = assign95850_e148429_d_n9;
        locals.var_tmf0_dn10 = assign95850_e148429_d_n10;
        locals.var_tmf0_dn11 = assign95850_e148429_d_n11;
        locals.var_tmf0_dn14 = assign95850_e148429_d_n14;

        let (assign95860_e148443, assign95860_e148443_d_n0, assign95860_e148443_d_n2, assign95860_e148443_d_n4, assign95860_e148443_d_n5, assign95860_e148443_d_n6, assign95860_e148443_d_n7, assign95860_e148443_d_n8, assign95860_e148443_d_n9, assign95860_e148443_d_n10, assign95860_e148443_d_n11, assign95860_e148443_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95860_e148435: f64 = (10.0 * 2.220446049250313e-16);
        let assign95860_e148437: f64 = (assign95860_e148435 * locals.var_xmp);
        let assign95860_e148439: f64 = (assign95860_e148437 * locals.var_dnm);
        let assign95860_e148441: f64 = (assign95860_e148439 / locals.var_arg);
        (assign95860_e148441, ((((((assign95860_e148435 * locals.var_xmp_dn0) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn0)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn2) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn2)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn4) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn4)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn5) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn5)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn6) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn6)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn7) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn7)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn8) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn8)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn9) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn9)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn10) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn10)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn11) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn11)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn14) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn14)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95860_e148443;
        locals.var_t0_dn0 = assign95860_e148443_d_n0;
        locals.var_t0_dn2 = assign95860_e148443_d_n2;
        locals.var_t0_dn4 = assign95860_e148443_d_n4;
        locals.var_t0_dn5 = assign95860_e148443_d_n5;
        locals.var_t0_dn6 = assign95860_e148443_d_n6;
        locals.var_t0_dn7 = assign95860_e148443_d_n7;
        locals.var_t0_dn8 = assign95860_e148443_d_n8;
        locals.var_t0_dn9 = assign95860_e148443_d_n9;
        locals.var_t0_dn10 = assign95860_e148443_d_n10;
        locals.var_t0_dn11 = assign95860_e148443_d_n11;
        locals.var_t0_dn14 = assign95860_e148443_d_n14;

        let (assign95870_e148461, assign95870_e148461_d_n0, assign95870_e148461_d_n2, assign95870_e148461_d_n4, assign95870_e148461_d_n5, assign95870_e148461_d_n6, assign95870_e148461_d_n7, assign95870_e148461_d_n8, assign95870_e148461_d_n9, assign95870_e148461_d_n10, assign95870_e148461_d_n11, assign95870_e148461_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95870_e148449: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95870_e148452: f64 = (10.0 * 2.220446049250313e-16);
        let assign95870_e148453: f64 = (assign95870_e148449 - assign95870_e148452);
        let assign95870_e148456: f64 = (10.0 * 2.220446049250313e-16);
        let assign95870_e148457: f64 = (assign95870_e148453 - assign95870_e148456);
        let assign95870_e148459: f64 = (assign95870_e148457 + locals.var_tmf0);
        (assign95870_e148459, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn11 + locals.var_vds_dn11) + locals.var_tmf0_dn11), ((locals.var_ps0_dn14 + locals.var_vds_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95870_e148461;
        locals.var_psdl_dn0 = assign95870_e148461_d_n0;
        locals.var_psdl_dn2 = assign95870_e148461_d_n2;
        locals.var_psdl_dn4 = assign95870_e148461_d_n4;
        locals.var_psdl_dn5 = assign95870_e148461_d_n5;
        locals.var_psdl_dn6 = assign95870_e148461_d_n6;
        locals.var_psdl_dn7 = assign95870_e148461_d_n7;
        locals.var_psdl_dn8 = assign95870_e148461_d_n8;
        locals.var_psdl_dn9 = assign95870_e148461_d_n9;
        locals.var_psdl_dn10 = assign95870_e148461_d_n10;
        locals.var_psdl_dn11 = assign95870_e148461_d_n11;
        locals.var_psdl_dn14 = assign95870_e148461_d_n14;

        let (assign95880_e148467, assign95880_e148467_d_n0, assign95880_e148467_d_n2, assign95880_e148467_d_n4, assign95880_e148467_d_n5, assign95880_e148467_d_n6, assign95880_e148467_d_n7, assign95880_e148467_d_n8, assign95880_e148467_d_n9, assign95880_e148467_d_n10, assign95880_e148467_d_n11, assign95880_e148467_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95880_e148467;
        locals.var_t0_dn0 = assign95880_e148467_d_n0;
        locals.var_t0_dn2 = assign95880_e148467_d_n2;
        locals.var_t0_dn4 = assign95880_e148467_d_n4;
        locals.var_t0_dn5 = assign95880_e148467_d_n5;
        locals.var_t0_dn6 = assign95880_e148467_d_n6;
        locals.var_t0_dn7 = assign95880_e148467_d_n7;
        locals.var_t0_dn8 = assign95880_e148467_d_n8;
        locals.var_t0_dn9 = assign95880_e148467_d_n9;
        locals.var_t0_dn10 = assign95880_e148467_d_n10;
        locals.var_t0_dn11 = assign95880_e148467_d_n11;
        locals.var_t0_dn14 = assign95880_e148467_d_n14;

        let (assign95890_e148474, assign95890_e148474_d_n0, assign95890_e148474_d_n2, assign95890_e148474_d_n4, assign95890_e148474_d_n5, assign95890_e148474_d_n6, assign95890_e148474_d_n7, assign95890_e148474_d_n8, assign95890_e148474_d_n9, assign95890_e148474_d_n10, assign95890_e148474_d_n11, assign95890_e148474_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95890_e148474;
        locals.var_psdl_dn0 = assign95890_e148474_d_n0;
        locals.var_psdl_dn2 = assign95890_e148474_d_n2;
        locals.var_psdl_dn4 = assign95890_e148474_d_n4;
        locals.var_psdl_dn5 = assign95890_e148474_d_n5;
        locals.var_psdl_dn6 = assign95890_e148474_d_n6;
        locals.var_psdl_dn7 = assign95890_e148474_d_n7;
        locals.var_psdl_dn8 = assign95890_e148474_d_n8;
        locals.var_psdl_dn9 = assign95890_e148474_d_n9;
        locals.var_psdl_dn10 = assign95890_e148474_d_n10;
        locals.var_psdl_dn11 = assign95890_e148474_d_n11;
        locals.var_psdl_dn14 = assign95890_e148474_d_n14;

        let (assign95900_e148481, assign95900_e148481_d_n0, assign95900_e148481_d_n2, assign95900_e148481_d_n4, assign95900_e148481_d_n5, assign95900_e148481_d_n6, assign95900_e148481_d_n7, assign95900_e148481_d_n8, assign95900_e148481_d_n9, assign95900_e148481_d_n10, assign95900_e148481_d_n11, assign95900_e148481_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95900_e148481;
        locals.var_t0_dn0 = assign95900_e148481_d_n0;
        locals.var_t0_dn2 = assign95900_e148481_d_n2;
        locals.var_t0_dn4 = assign95900_e148481_d_n4;
        locals.var_t0_dn5 = assign95900_e148481_d_n5;
        locals.var_t0_dn6 = assign95900_e148481_d_n6;
        locals.var_t0_dn7 = assign95900_e148481_d_n7;
        locals.var_t0_dn8 = assign95900_e148481_d_n8;
        locals.var_t0_dn9 = assign95900_e148481_d_n9;
        locals.var_t0_dn10 = assign95900_e148481_d_n10;
        locals.var_t0_dn11 = assign95900_e148481_d_n11;
        locals.var_t0_dn14 = assign95900_e148481_d_n14;

        let (assign95910_e148487, assign95910_e148487_d_n0, assign95910_e148487_d_n2, assign95910_e148487_d_n4, assign95910_e148487_d_n5, assign95910_e148487_d_n6, assign95910_e148487_d_n7, assign95910_e148487_d_n8, assign95910_e148487_d_n9, assign95910_e148487_d_n10, assign95910_e148487_d_n11, assign95910_e148487_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_flg_qy != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95910_e148487;
        locals.var_ec_dn0 = assign95910_e148487_d_n0;
        locals.var_ec_dn2 = assign95910_e148487_d_n2;
        locals.var_ec_dn4 = assign95910_e148487_d_n4;
        locals.var_ec_dn5 = assign95910_e148487_d_n5;
        locals.var_ec_dn6 = assign95910_e148487_d_n6;
        locals.var_ec_dn7 = assign95910_e148487_d_n7;
        locals.var_ec_dn8 = assign95910_e148487_d_n8;
        locals.var_ec_dn9 = assign95910_e148487_d_n9;
        locals.var_ec_dn10 = assign95910_e148487_d_n10;
        locals.var_ec_dn11 = assign95910_e148487_d_n11;
        locals.var_ec_dn14 = assign95910_e148487_d_n14;

        let assign95920_e148494: f64 = if ((locals.var_idd < 1e-15) || (locals.var_vdseff < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard2233 = assign95920_e148494;

        let (assign95930_e148503, assign95930_e148503_d_n0, assign95930_e148503_d_n2, assign95930_e148503_d_n4, assign95930_e148503_d_n5, assign95930_e148503_d_n6, assign95930_e148503_d_n7, assign95930_e148503_d_n8, assign95930_e148503_d_n9, assign95930_e148503_d_n10, assign95930_e148503_d_n11, assign95930_e148503_d_n14,) = {
    if (((locals.var_guard2226 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2233 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95930_e148503;
        locals.var_ec_dn0 = assign95930_e148503_d_n0;
        locals.var_ec_dn2 = assign95930_e148503_d_n2;
        locals.var_ec_dn4 = assign95930_e148503_d_n4;
        locals.var_ec_dn5 = assign95930_e148503_d_n5;
        locals.var_ec_dn6 = assign95930_e148503_d_n6;
        locals.var_ec_dn7 = assign95930_e148503_d_n7;
        locals.var_ec_dn8 = assign95930_e148503_d_n8;
        locals.var_ec_dn9 = assign95930_e148503_d_n9;
        locals.var_ec_dn10 = assign95930_e148503_d_n10;
        locals.var_ec_dn11 = assign95930_e148503_d_n11;
        locals.var_ec_dn14 = assign95930_e148503_d_n14;

        let (assign95940_e148519, assign95940_e148519_d_n0, assign95940_e148519_d_n2, assign95940_e148519_d_n4, assign95940_e148519_d_n5, assign95940_e148519_d_n6, assign95940_e148519_d_n7, assign95940_e148519_d_n8, assign95940_e148519_d_n9, assign95940_e148519_d_n10, assign95940_e148519_d_n11, assign95940_e148519_d_n14,) = {
    if (((locals.var_guard2226 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2233 == 0.0)) {
        let assign95940_e148513: f64 = (locals.var_idd / locals.var_qn0);
        let assign95940_e148515: f64 = (assign95940_e148513 * locals.var_beta_inv);
        let assign95940_e148517: f64 = (assign95940_e148515 / locals.var_leff);
        (assign95940_e148517, ((((((locals.var_idd_dn0 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn0)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn0)) / locals.var_leff), ((((((locals.var_idd_dn2 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn2)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn2)) / locals.var_leff), ((((((locals.var_idd_dn4 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn4)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn4)) / locals.var_leff), ((((((locals.var_idd_dn5 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn5)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn5)) / locals.var_leff), ((((((locals.var_idd_dn6 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn6)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn6)) / locals.var_leff), ((((((locals.var_idd_dn7 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn7)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn7)) / locals.var_leff), ((((((locals.var_idd_dn8 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn8)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn8)) / locals.var_leff), ((((((locals.var_idd_dn9 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn9)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn9)) / locals.var_leff), ((((((locals.var_idd_dn10 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn10)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn10)) / locals.var_leff), ((((((locals.var_idd_dn11 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn11)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn11)) / locals.var_leff), ((((((locals.var_idd_dn14 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn14)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn14)) / locals.var_leff),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95940_e148519;
        locals.var_ec_dn0 = assign95940_e148519_d_n0;
        locals.var_ec_dn2 = assign95940_e148519_d_n2;
        locals.var_ec_dn4 = assign95940_e148519_d_n4;
        locals.var_ec_dn5 = assign95940_e148519_d_n5;
        locals.var_ec_dn6 = assign95940_e148519_d_n6;
        locals.var_ec_dn7 = assign95940_e148519_d_n7;
        locals.var_ec_dn8 = assign95940_e148519_d_n8;
        locals.var_ec_dn9 = assign95940_e148519_d_n9;
        locals.var_ec_dn10 = assign95940_e148519_d_n10;
        locals.var_ec_dn11 = assign95940_e148519_d_n11;
        locals.var_ec_dn14 = assign95940_e148519_d_n14;

        let assign95950_e148522: f64 = if locals.var_flg_qy == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2234 = assign95950_e148522;

        let (assign95960_e148526, assign95960_e148526_d_n0, assign95960_e148526_d_n2, assign95960_e148526_d_n4, assign95960_e148526_d_n5, assign95960_e148526_d_n6, assign95960_e148526_d_n7, assign95960_e148526_d_n8, assign95960_e148526_d_n9, assign95960_e148526_d_n10, assign95960_e148526_d_n11, assign95960_e148526_d_n14,) = {
    if (locals.var_guard2234 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign95960_e148526;
        locals.var_qy_dn0 = assign95960_e148526_d_n0;
        locals.var_qy_dn2 = assign95960_e148526_d_n2;
        locals.var_qy_dn4 = assign95960_e148526_d_n4;
        locals.var_qy_dn5 = assign95960_e148526_d_n5;
        locals.var_qy_dn6 = assign95960_e148526_d_n6;
        locals.var_qy_dn7 = assign95960_e148526_d_n7;
        locals.var_qy_dn8 = assign95960_e148526_d_n8;
        locals.var_qy_dn9 = assign95960_e148526_d_n9;
        locals.var_qy_dn10 = assign95960_e148526_d_n10;
        locals.var_qy_dn11 = assign95960_e148526_d_n11;
        locals.var_qy_dn14 = assign95960_e148526_d_n14;

        let (assign95970_e148537, assign95970_e148537_d_n0, assign95970_e148537_d_n2, assign95970_e148537_d_n4, assign95970_e148537_d_n5, assign95970_e148537_d_n6, assign95970_e148537_d_n7, assign95970_e148537_d_n8, assign95970_e148537_d_n9, assign95970_e148537_d_n10, assign95970_e148537_d_n11, assign95970_e148537_d_n14,) = {
    if (locals.var_guard2234 == 0.0) {
        let assign95970_e148531: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign95970_e148533: f64 = (assign95970_e148531 * locals.var_wdpl);
        let assign95970_e148535: f64 = (assign95970_e148533 * 1.3);
        (assign95970_e148535, ((assign95970_e148531 * locals.var_wdpl_dn0) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn2) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn4) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn5) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn6) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn7) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn8) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn9) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn10) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn11) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn14) * 1.3),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign95970_e148537;
        locals.var_t2_dn0 = assign95970_e148537_d_n0;
        locals.var_t2_dn2 = assign95970_e148537_d_n2;
        locals.var_t2_dn4 = assign95970_e148537_d_n4;
        locals.var_t2_dn5 = assign95970_e148537_d_n5;
        locals.var_t2_dn6 = assign95970_e148537_d_n6;
        locals.var_t2_dn7 = assign95970_e148537_d_n7;
        locals.var_t2_dn8 = assign95970_e148537_d_n8;
        locals.var_t2_dn9 = assign95970_e148537_d_n9;
        locals.var_t2_dn10 = assign95970_e148537_d_n10;
        locals.var_t2_dn11 = assign95970_e148537_d_n11;
        locals.var_t2_dn14 = assign95970_e148537_d_n14;

        let assign95980_e148540: f64 = if p.p133 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2235 = assign95980_e148540;

    }

    pub(super) fn stamp_transient_block_354(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95990_e148551, assign95990_e148551_d_n0, assign95990_e148551_d_n2, assign95990_e148551_d_n4, assign95990_e148551_d_n5, assign95990_e148551_d_n6, assign95990_e148551_d_n7, assign95990_e148551_d_n8, assign95990_e148551_d_n9, assign95990_e148551_d_n10, assign95990_e148551_d_n11, assign95990_e148551_d_n14,) = {
    if ((locals.var_guard2234 == 0.0) && (locals.var_guard2235 != 0.0)) {
        let assign95990_e148547: f64 = (locals.var_ec * locals.var_leff);
        let assign95990_e148549: f64 = (assign95990_e148547 + locals.var_ps0);
        (assign95990_e148549, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn4 * locals.var_leff) + locals.var_ps0_dn4), ((locals.var_ec_dn5 * locals.var_leff) + locals.var_ps0_dn5), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn8 * locals.var_leff) + locals.var_ps0_dn8), ((locals.var_ec_dn9 * locals.var_leff) + locals.var_ps0_dn9), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn14 * locals.var_leff) + locals.var_ps0_dn14),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn8, locals.var_pslk_dn9, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn14,)
    }
};
        locals.var_pslk = assign95990_e148551;
        locals.var_pslk_dn0 = assign95990_e148551_d_n0;
        locals.var_pslk_dn2 = assign95990_e148551_d_n2;
        locals.var_pslk_dn4 = assign95990_e148551_d_n4;
        locals.var_pslk_dn5 = assign95990_e148551_d_n5;
        locals.var_pslk_dn6 = assign95990_e148551_d_n6;
        locals.var_pslk_dn7 = assign95990_e148551_d_n7;
        locals.var_pslk_dn8 = assign95990_e148551_d_n8;
        locals.var_pslk_dn9 = assign95990_e148551_d_n9;
        locals.var_pslk_dn10 = assign95990_e148551_d_n10;
        locals.var_pslk_dn11 = assign95990_e148551_d_n11;
        locals.var_pslk_dn14 = assign95990_e148551_d_n14;

        let (assign96000_e148568, assign96000_e148568_d_n0, assign96000_e148568_d_n2, assign96000_e148568_d_n4, assign96000_e148568_d_n5, assign96000_e148568_d_n6, assign96000_e148568_d_n7, assign96000_e148568_d_n8, assign96000_e148568_d_n9, assign96000_e148568_d_n10, assign96000_e148568_d_n11, assign96000_e148568_d_n14,) = {
    if ((locals.var_guard2234 == 0.0) && (locals.var_guard2235 != 0.0)) {
        let assign96000_e148559: f64 = (locals.var_vdsz__blk443 + locals.var_ps0);
        let assign96000_e148560: f64 = (locals.var_aclm * assign96000_e148559);
        let assign96000_e148563: f64 = (1.0 - locals.var_aclm);
        let assign96000_e148565: f64 = (assign96000_e148563 * locals.var_pslk);
        let assign96000_e148566: f64 = (assign96000_e148560 + assign96000_e148565);
        (assign96000_e148566, ((locals.var_aclm * (locals.var_vdsz__blk443_dn0 + locals.var_ps0_dn0)) + (assign96000_e148563 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn2 + locals.var_ps0_dn2)) + (assign96000_e148563 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn4 + locals.var_ps0_dn4)) + (assign96000_e148563 * locals.var_pslk_dn4)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn5 + locals.var_ps0_dn5)) + (assign96000_e148563 * locals.var_pslk_dn5)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn6 + locals.var_ps0_dn6)) + (assign96000_e148563 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn7 + locals.var_ps0_dn7)) + (assign96000_e148563 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn8 + locals.var_ps0_dn8)) + (assign96000_e148563 * locals.var_pslk_dn8)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn9 + locals.var_ps0_dn9)) + (assign96000_e148563 * locals.var_pslk_dn9)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn10 + locals.var_ps0_dn10)) + (assign96000_e148563 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn11 + locals.var_ps0_dn11)) + (assign96000_e148563 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn14 + locals.var_ps0_dn14)) + (assign96000_e148563 * locals.var_pslk_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign96000_e148568;
        locals.var_t1_dn0 = assign96000_e148568_d_n0;
        locals.var_t1_dn2 = assign96000_e148568_d_n2;
        locals.var_t1_dn4 = assign96000_e148568_d_n4;
        locals.var_t1_dn5 = assign96000_e148568_d_n5;
        locals.var_t1_dn6 = assign96000_e148568_d_n6;
        locals.var_t1_dn7 = assign96000_e148568_d_n7;
        locals.var_t1_dn8 = assign96000_e148568_d_n8;
        locals.var_t1_dn9 = assign96000_e148568_d_n9;
        locals.var_t1_dn10 = assign96000_e148568_d_n10;
        locals.var_t1_dn11 = assign96000_e148568_d_n11;
        locals.var_t1_dn14 = assign96000_e148568_d_n14;

        let (assign96010_e148584, assign96010_e148584_d_n0, assign96010_e148584_d_n2, assign96010_e148584_d_n4, assign96010_e148584_d_n5, assign96010_e148584_d_n6, assign96010_e148584_d_n7, assign96010_e148584_d_n8, assign96010_e148584_d_n9, assign96010_e148584_d_n10, assign96010_e148584_d_n11, assign96010_e148584_d_n14,) = {
    if ((locals.var_guard2234 == 0.0) && (locals.var_guard2235 != 0.0)) {
        let assign96010_e148575: f64 = (locals.var_ps0 + locals.var_vdsz__blk443);
        let assign96010_e148577: f64 = (assign96010_e148575 - locals.var_t1);
        let assign96010_e148579: f64 = (assign96010_e148577 / p.p133);
        let assign96010_e148580: f64 = (-assign96010_e148579);
        let assign96010_e148582: f64 = (assign96010_e148580 * locals.var_t2);
        (assign96010_e148582, (((-(((locals.var_ps0_dn0 + locals.var_vdsz__blk443_dn0) - locals.var_t1_dn0) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn0)), (((-(((locals.var_ps0_dn2 + locals.var_vdsz__blk443_dn2) - locals.var_t1_dn2) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn2)), (((-(((locals.var_ps0_dn4 + locals.var_vdsz__blk443_dn4) - locals.var_t1_dn4) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn4)), (((-(((locals.var_ps0_dn5 + locals.var_vdsz__blk443_dn5) - locals.var_t1_dn5) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn5)), (((-(((locals.var_ps0_dn6 + locals.var_vdsz__blk443_dn6) - locals.var_t1_dn6) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn6)), (((-(((locals.var_ps0_dn7 + locals.var_vdsz__blk443_dn7) - locals.var_t1_dn7) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn7)), (((-(((locals.var_ps0_dn8 + locals.var_vdsz__blk443_dn8) - locals.var_t1_dn8) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn8)), (((-(((locals.var_ps0_dn9 + locals.var_vdsz__blk443_dn9) - locals.var_t1_dn9) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn9)), (((-(((locals.var_ps0_dn10 + locals.var_vdsz__blk443_dn10) - locals.var_t1_dn10) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn10)), (((-(((locals.var_ps0_dn11 + locals.var_vdsz__blk443_dn11) - locals.var_t1_dn11) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn11)), (((-(((locals.var_ps0_dn14 + locals.var_vdsz__blk443_dn14) - locals.var_t1_dn14) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn14)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign96010_e148584;
        locals.var_qy_dn0 = assign96010_e148584_d_n0;
        locals.var_qy_dn2 = assign96010_e148584_d_n2;
        locals.var_qy_dn4 = assign96010_e148584_d_n4;
        locals.var_qy_dn5 = assign96010_e148584_d_n5;
        locals.var_qy_dn6 = assign96010_e148584_d_n6;
        locals.var_qy_dn7 = assign96010_e148584_d_n7;
        locals.var_qy_dn8 = assign96010_e148584_d_n8;
        locals.var_qy_dn9 = assign96010_e148584_d_n9;
        locals.var_qy_dn10 = assign96010_e148584_d_n10;
        locals.var_qy_dn11 = assign96010_e148584_d_n11;
        locals.var_qy_dn14 = assign96010_e148584_d_n14;

        let assign96020_e148587: f64 = if p.p134 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2236 = assign96020_e148587;

        let (assign96030_e148598, assign96030_e148598_d_n0, assign96030_e148598_d_n2, assign96030_e148598_d_n4, assign96030_e148598_d_n5, assign96030_e148598_d_n6, assign96030_e148598_d_n7, assign96030_e148598_d_n8, assign96030_e148598_d_n9, assign96030_e148598_d_n10, assign96030_e148598_d_n11, assign96030_e148598_d_n14,) = {
    if ((locals.var_guard2234 == 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96030_e148595: f64 = (locals.var_cqyb0 * locals.var_vbs);
        let assign96030_e148596: f64 = (locals.var_qy + assign96030_e148595);
        (assign96030_e148596, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbs_dn6)), locals.var_qy_dn7, (locals.var_qy_dn8 + (locals.var_cqyb0 * locals.var_vbs_dn8)), (locals.var_qy_dn9 + (locals.var_cqyb0 * locals.var_vbs_dn9)), locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign96030_e148598;
        locals.var_qy_dn0 = assign96030_e148598_d_n0;
        locals.var_qy_dn2 = assign96030_e148598_d_n2;
        locals.var_qy_dn4 = assign96030_e148598_d_n4;
        locals.var_qy_dn5 = assign96030_e148598_d_n5;
        locals.var_qy_dn6 = assign96030_e148598_d_n6;
        locals.var_qy_dn7 = assign96030_e148598_d_n7;
        locals.var_qy_dn8 = assign96030_e148598_d_n8;
        locals.var_qy_dn9 = assign96030_e148598_d_n9;
        locals.var_qy_dn10 = assign96030_e148598_d_n10;
        locals.var_qy_dn11 = assign96030_e148598_d_n11;
        locals.var_qy_dn14 = assign96030_e148598_d_n14;

        locals.var_cfd = locals.var_cfrng;

        locals.var_cfs = locals.var_cfrng;

        let assign96060_e148604: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign96060_e148605: f64 = (locals.var_cfd * assign96060_e148604);
        locals.var_qfd = assign96060_e148605;
        locals.var_qfd_dn0 = (locals.var_cfd * (-locals.var_vdsei_dn0));
        locals.var_qfd_dn2 = (locals.var_cfd * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qfd_dn7 = (locals.var_cfd * locals.var_vgsei_dn7);

        let assign96070_e148608: f64 = (locals.var_cfs * locals.var_vgsei);
        locals.var_qfs = assign96070_e148608;
        locals.var_qfs_dn2 = (locals.var_cfs * locals.var_vgsei_dn2);
        locals.var_qfs_dn7 = (locals.var_cfs * locals.var_vgsei_dn7);

        let assign96080_e148615: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2237 = assign96080_e148615;

        let (assign96090_e148621, assign96090_e148621_d_n0, assign96090_e148621_d_n2, assign96090_e148621_d_n4, assign96090_e148621_d_n5, assign96090_e148621_d_n6, assign96090_e148621_d_n7, assign96090_e148621_d_n8, assign96090_e148621_d_n9, assign96090_e148621_d_n10, assign96090_e148621_d_n11, assign96090_e148621_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96090_e148619: f64 = (locals.var_tratio * locals.var_tratio);
        (assign96090_e148619, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn11 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn11)), ((locals.var_tratio_dn14 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign96090_e148621;
        locals.var_t0_dn0 = assign96090_e148621_d_n0;
        locals.var_t0_dn2 = assign96090_e148621_d_n2;
        locals.var_t0_dn4 = assign96090_e148621_d_n4;
        locals.var_t0_dn5 = assign96090_e148621_d_n5;
        locals.var_t0_dn6 = assign96090_e148621_d_n6;
        locals.var_t0_dn7 = assign96090_e148621_d_n7;
        locals.var_t0_dn8 = assign96090_e148621_d_n8;
        locals.var_t0_dn9 = assign96090_e148621_d_n9;
        locals.var_t0_dn10 = assign96090_e148621_d_n10;
        locals.var_t0_dn11 = assign96090_e148621_d_n11;
        locals.var_t0_dn14 = assign96090_e148621_d_n14;

        let (assign96100_e148640, assign96100_e148640_d_n0, assign96100_e148640_d_n2, assign96100_e148640_d_n4, assign96100_e148640_d_n5, assign96100_e148640_d_n6, assign96100_e148640_d_n7, assign96100_e148640_d_n8, assign96100_e148640_d_n9, assign96100_e148640_d_n10, assign96100_e148640_d_n11, assign96100_e148640_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96100_e148626: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96100_e148629: f64 = (locals.var_eg * locals.var_beta);
        let assign96100_e148630: f64 = (assign96100_e148626 - assign96100_e148629);
        let assign96100_e148633: f64 = (p.p499 * locals.var_log_tratio);
        let assign96100_e148634: f64 = (assign96100_e148630 + assign96100_e148633);
        let assign96100_e148636: f64 = (assign96100_e148634 / locals.var_uc_njd);
        let assign96100_e148637: f64 = (assign96100_e148636).exp();
        let assign96100_e148638: f64 = (locals.var_uc_js0d * assign96100_e148637);
        (assign96100_e148638, (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign96100_e148640;
        locals.var_js_dn0 = assign96100_e148640_d_n0;
        locals.var_js_dn2 = assign96100_e148640_d_n2;
        locals.var_js_dn4 = assign96100_e148640_d_n4;
        locals.var_js_dn5 = assign96100_e148640_d_n5;
        locals.var_js_dn6 = assign96100_e148640_d_n6;
        locals.var_js_dn7 = assign96100_e148640_d_n7;
        locals.var_js_dn8 = assign96100_e148640_d_n8;
        locals.var_js_dn9 = assign96100_e148640_d_n9;
        locals.var_js_dn10 = assign96100_e148640_d_n10;
        locals.var_js_dn11 = assign96100_e148640_d_n11;
        locals.var_js_dn14 = assign96100_e148640_d_n14;

        let (assign96110_e148659, assign96110_e148659_d_n0, assign96110_e148659_d_n2, assign96110_e148659_d_n4, assign96110_e148659_d_n5, assign96110_e148659_d_n6, assign96110_e148659_d_n7, assign96110_e148659_d_n8, assign96110_e148659_d_n9, assign96110_e148659_d_n10, assign96110_e148659_d_n11, assign96110_e148659_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96110_e148645: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96110_e148648: f64 = (locals.var_eg * locals.var_beta);
        let assign96110_e148649: f64 = (assign96110_e148645 - assign96110_e148648);
        let assign96110_e148652: f64 = (p.p499 * locals.var_log_tratio);
        let assign96110_e148653: f64 = (assign96110_e148649 + assign96110_e148652);
        let assign96110_e148655: f64 = (assign96110_e148653 / p.p497);
        let assign96110_e148656: f64 = (assign96110_e148655).exp();
        let assign96110_e148657: f64 = (locals.var_uc_js0swd * assign96110_e148656);
        (assign96110_e148657, (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign96110_e148659;
        locals.var_jssw_dn0 = assign96110_e148659_d_n0;
        locals.var_jssw_dn2 = assign96110_e148659_d_n2;
        locals.var_jssw_dn4 = assign96110_e148659_d_n4;
        locals.var_jssw_dn5 = assign96110_e148659_d_n5;
        locals.var_jssw_dn6 = assign96110_e148659_d_n6;
        locals.var_jssw_dn7 = assign96110_e148659_d_n7;
        locals.var_jssw_dn8 = assign96110_e148659_d_n8;
        locals.var_jssw_dn9 = assign96110_e148659_d_n9;
        locals.var_jssw_dn10 = assign96110_e148659_d_n10;
        locals.var_jssw_dn11 = assign96110_e148659_d_n11;
        locals.var_jssw_dn14 = assign96110_e148659_d_n14;

        let (assign96120_e148678, assign96120_e148678_d_n0, assign96120_e148678_d_n2, assign96120_e148678_d_n4, assign96120_e148678_d_n5, assign96120_e148678_d_n6, assign96120_e148678_d_n7, assign96120_e148678_d_n8, assign96120_e148678_d_n9, assign96120_e148678_d_n10, assign96120_e148678_d_n11, assign96120_e148678_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96120_e148664: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96120_e148667: f64 = (locals.var_eg * locals.var_beta);
        let assign96120_e148668: f64 = (assign96120_e148664 - assign96120_e148667);
        let assign96120_e148671: f64 = (p.p499 * locals.var_log_tratio);
        let assign96120_e148672: f64 = (assign96120_e148668 + assign96120_e148671);
        let assign96120_e148674: f64 = (assign96120_e148672 / p.p498);
        let assign96120_e148675: f64 = (assign96120_e148674).exp();
        let assign96120_e148676: f64 = (p.p495 * assign96120_e148675);
        (assign96120_e148676, (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign96120_e148678;
        locals.var_jsswg_dn0 = assign96120_e148678_d_n0;
        locals.var_jsswg_dn2 = assign96120_e148678_d_n2;
        locals.var_jsswg_dn4 = assign96120_e148678_d_n4;
        locals.var_jsswg_dn5 = assign96120_e148678_d_n5;
        locals.var_jsswg_dn6 = assign96120_e148678_d_n6;
        locals.var_jsswg_dn7 = assign96120_e148678_d_n7;
        locals.var_jsswg_dn8 = assign96120_e148678_d_n8;
        locals.var_jsswg_dn9 = assign96120_e148678_d_n9;
        locals.var_jsswg_dn10 = assign96120_e148678_d_n10;
        locals.var_jsswg_dn11 = assign96120_e148678_d_n11;
        locals.var_jsswg_dn14 = assign96120_e148678_d_n14;

        let (assign96130_e148697, assign96130_e148697_d_n0, assign96130_e148697_d_n2, assign96130_e148697_d_n4, assign96130_e148697_d_n5, assign96130_e148697_d_n6, assign96130_e148697_d_n7, assign96130_e148697_d_n8, assign96130_e148697_d_n9, assign96130_e148697_d_n10, assign96130_e148697_d_n11, assign96130_e148697_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96130_e148683: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96130_e148686: f64 = (locals.var_eg * locals.var_beta);
        let assign96130_e148687: f64 = (assign96130_e148683 - assign96130_e148686);
        let assign96130_e148690: f64 = (p.p509 * locals.var_log_tratio);
        let assign96130_e148691: f64 = (assign96130_e148687 + assign96130_e148690);
        let assign96130_e148693: f64 = (assign96130_e148691 / locals.var_uc_njd);
        let assign96130_e148694: f64 = (assign96130_e148693).exp();
        let assign96130_e148695: f64 = (locals.var_uc_js0d * assign96130_e148694);
        (assign96130_e148695, (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign96130_e148697;
        locals.var_js2_dn0 = assign96130_e148697_d_n0;
        locals.var_js2_dn2 = assign96130_e148697_d_n2;
        locals.var_js2_dn4 = assign96130_e148697_d_n4;
        locals.var_js2_dn5 = assign96130_e148697_d_n5;
        locals.var_js2_dn6 = assign96130_e148697_d_n6;
        locals.var_js2_dn7 = assign96130_e148697_d_n7;
        locals.var_js2_dn8 = assign96130_e148697_d_n8;
        locals.var_js2_dn9 = assign96130_e148697_d_n9;
        locals.var_js2_dn10 = assign96130_e148697_d_n10;
        locals.var_js2_dn11 = assign96130_e148697_d_n11;
        locals.var_js2_dn14 = assign96130_e148697_d_n14;

        let (assign96140_e148716, assign96140_e148716_d_n0, assign96140_e148716_d_n2, assign96140_e148716_d_n4, assign96140_e148716_d_n5, assign96140_e148716_d_n6, assign96140_e148716_d_n7, assign96140_e148716_d_n8, assign96140_e148716_d_n9, assign96140_e148716_d_n10, assign96140_e148716_d_n11, assign96140_e148716_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96140_e148702: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96140_e148705: f64 = (locals.var_eg * locals.var_beta);
        let assign96140_e148706: f64 = (assign96140_e148702 - assign96140_e148705);
        let assign96140_e148709: f64 = (p.p509 * locals.var_log_tratio);
        let assign96140_e148710: f64 = (assign96140_e148706 + assign96140_e148709);
        let assign96140_e148712: f64 = (assign96140_e148710 / p.p497);
        let assign96140_e148713: f64 = (assign96140_e148712).exp();
        let assign96140_e148714: f64 = (locals.var_uc_js0swd * assign96140_e148713);
        (assign96140_e148714, (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign96140_e148716;
        locals.var_jssw2_dn0 = assign96140_e148716_d_n0;
        locals.var_jssw2_dn2 = assign96140_e148716_d_n2;
        locals.var_jssw2_dn4 = assign96140_e148716_d_n4;
        locals.var_jssw2_dn5 = assign96140_e148716_d_n5;
        locals.var_jssw2_dn6 = assign96140_e148716_d_n6;
        locals.var_jssw2_dn7 = assign96140_e148716_d_n7;
        locals.var_jssw2_dn8 = assign96140_e148716_d_n8;
        locals.var_jssw2_dn9 = assign96140_e148716_d_n9;
        locals.var_jssw2_dn10 = assign96140_e148716_d_n10;
        locals.var_jssw2_dn11 = assign96140_e148716_d_n11;
        locals.var_jssw2_dn14 = assign96140_e148716_d_n14;

        let (assign96150_e148735, assign96150_e148735_d_n0, assign96150_e148735_d_n2, assign96150_e148735_d_n4, assign96150_e148735_d_n5, assign96150_e148735_d_n6, assign96150_e148735_d_n7, assign96150_e148735_d_n8, assign96150_e148735_d_n9, assign96150_e148735_d_n10, assign96150_e148735_d_n11, assign96150_e148735_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96150_e148721: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96150_e148724: f64 = (locals.var_eg * locals.var_beta);
        let assign96150_e148725: f64 = (assign96150_e148721 - assign96150_e148724);
        let assign96150_e148728: f64 = (p.p509 * locals.var_log_tratio);
        let assign96150_e148729: f64 = (assign96150_e148725 + assign96150_e148728);
        let assign96150_e148731: f64 = (assign96150_e148729 / p.p498);
        let assign96150_e148732: f64 = (assign96150_e148731).exp();
        let assign96150_e148733: f64 = (p.p495 * assign96150_e148732);
        (assign96150_e148733, (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign96150_e148735;
        locals.var_jsswg2_dn0 = assign96150_e148735_d_n0;
        locals.var_jsswg2_dn2 = assign96150_e148735_d_n2;
        locals.var_jsswg2_dn4 = assign96150_e148735_d_n4;
        locals.var_jsswg2_dn5 = assign96150_e148735_d_n5;
        locals.var_jsswg2_dn6 = assign96150_e148735_d_n6;
        locals.var_jsswg2_dn7 = assign96150_e148735_d_n7;
        locals.var_jsswg2_dn8 = assign96150_e148735_d_n8;
        locals.var_jsswg2_dn9 = assign96150_e148735_d_n9;
        locals.var_jsswg2_dn10 = assign96150_e148735_d_n10;
        locals.var_jsswg2_dn11 = assign96150_e148735_d_n11;
        locals.var_jsswg2_dn14 = assign96150_e148735_d_n14;

        let assign96160_e148738: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2238 = assign96160_e148738;

        let assign96170_e148741: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2239 = assign96170_e148741;

        let (assign96180_e148751, assign96180_e148751_d_n0, assign96180_e148751_d_n2, assign96180_e148751_d_n4, assign96180_e148751_d_n5, assign96180_e148751_d_n6, assign96180_e148751_d_n7, assign96180_e148751_d_n8, assign96180_e148751_d_n9, assign96180_e148751_d_n10, assign96180_e148751_d_n11, assign96180_e148751_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96180_e148749: f64 = (p.p13 * locals.var_js);
        (assign96180_e148749, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96180_e148751;
        locals.var_isbd_btm_dn0 = assign96180_e148751_d_n0;
        locals.var_isbd_btm_dn2 = assign96180_e148751_d_n2;
        locals.var_isbd_btm_dn4 = assign96180_e148751_d_n4;
        locals.var_isbd_btm_dn5 = assign96180_e148751_d_n5;
        locals.var_isbd_btm_dn6 = assign96180_e148751_d_n6;
        locals.var_isbd_btm_dn7 = assign96180_e148751_d_n7;
        locals.var_isbd_btm_dn8 = assign96180_e148751_d_n8;
        locals.var_isbd_btm_dn9 = assign96180_e148751_d_n9;
        locals.var_isbd_btm_dn10 = assign96180_e148751_d_n10;
        locals.var_isbd_btm_dn11 = assign96180_e148751_d_n11;
        locals.var_isbd_btm_dn14 = assign96180_e148751_d_n14;

        let (assign96190_e148761, assign96190_e148761_d_n0, assign96190_e148761_d_n2, assign96190_e148761_d_n4, assign96190_e148761_d_n5, assign96190_e148761_d_n6, assign96190_e148761_d_n7, assign96190_e148761_d_n8, assign96190_e148761_d_n9, assign96190_e148761_d_n10, assign96190_e148761_d_n11, assign96190_e148761_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96190_e148759: f64 = (p.p13 * locals.var_js2);
        (assign96190_e148759, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96190_e148761;
        locals.var_isbd2_btm_dn0 = assign96190_e148761_d_n0;
        locals.var_isbd2_btm_dn2 = assign96190_e148761_d_n2;
        locals.var_isbd2_btm_dn4 = assign96190_e148761_d_n4;
        locals.var_isbd2_btm_dn5 = assign96190_e148761_d_n5;
        locals.var_isbd2_btm_dn6 = assign96190_e148761_d_n6;
        locals.var_isbd2_btm_dn7 = assign96190_e148761_d_n7;
        locals.var_isbd2_btm_dn8 = assign96190_e148761_d_n8;
        locals.var_isbd2_btm_dn9 = assign96190_e148761_d_n9;
        locals.var_isbd2_btm_dn10 = assign96190_e148761_d_n10;
        locals.var_isbd2_btm_dn11 = assign96190_e148761_d_n11;
        locals.var_isbd2_btm_dn14 = assign96190_e148761_d_n14;

        let (assign96200_e148773, assign96200_e148773_d_n0, assign96200_e148773_d_n2, assign96200_e148773_d_n4, assign96200_e148773_d_n5, assign96200_e148773_d_n6, assign96200_e148773_d_n7, assign96200_e148773_d_n8, assign96200_e148773_d_n9, assign96200_e148773_d_n10, assign96200_e148773_d_n11, assign96200_e148773_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96200_e148769: f64 = (p.p15 - locals.var_weff_nf);
        let assign96200_e148771: f64 = (assign96200_e148769 * locals.var_jssw);
        (assign96200_e148771, (assign96200_e148769 * locals.var_jssw_dn0), (assign96200_e148769 * locals.var_jssw_dn2), (assign96200_e148769 * locals.var_jssw_dn4), (assign96200_e148769 * locals.var_jssw_dn5), (assign96200_e148769 * locals.var_jssw_dn6), (assign96200_e148769 * locals.var_jssw_dn7), (assign96200_e148769 * locals.var_jssw_dn8), (assign96200_e148769 * locals.var_jssw_dn9), (assign96200_e148769 * locals.var_jssw_dn10), (assign96200_e148769 * locals.var_jssw_dn11), (assign96200_e148769 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96200_e148773;
        locals.var_isbd_sws_dn0 = assign96200_e148773_d_n0;
        locals.var_isbd_sws_dn2 = assign96200_e148773_d_n2;
        locals.var_isbd_sws_dn4 = assign96200_e148773_d_n4;
        locals.var_isbd_sws_dn5 = assign96200_e148773_d_n5;
        locals.var_isbd_sws_dn6 = assign96200_e148773_d_n6;
        locals.var_isbd_sws_dn7 = assign96200_e148773_d_n7;
        locals.var_isbd_sws_dn8 = assign96200_e148773_d_n8;
        locals.var_isbd_sws_dn9 = assign96200_e148773_d_n9;
        locals.var_isbd_sws_dn10 = assign96200_e148773_d_n10;
        locals.var_isbd_sws_dn11 = assign96200_e148773_d_n11;
        locals.var_isbd_sws_dn14 = assign96200_e148773_d_n14;

        let (assign96210_e148785, assign96210_e148785_d_n0, assign96210_e148785_d_n2, assign96210_e148785_d_n4, assign96210_e148785_d_n5, assign96210_e148785_d_n6, assign96210_e148785_d_n7, assign96210_e148785_d_n8, assign96210_e148785_d_n9, assign96210_e148785_d_n10, assign96210_e148785_d_n11, assign96210_e148785_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96210_e148781: f64 = (p.p15 - locals.var_weff_nf);
        let assign96210_e148783: f64 = (assign96210_e148781 * locals.var_jssw2);
        (assign96210_e148783, (assign96210_e148781 * locals.var_jssw2_dn0), (assign96210_e148781 * locals.var_jssw2_dn2), (assign96210_e148781 * locals.var_jssw2_dn4), (assign96210_e148781 * locals.var_jssw2_dn5), (assign96210_e148781 * locals.var_jssw2_dn6), (assign96210_e148781 * locals.var_jssw2_dn7), (assign96210_e148781 * locals.var_jssw2_dn8), (assign96210_e148781 * locals.var_jssw2_dn9), (assign96210_e148781 * locals.var_jssw2_dn10), (assign96210_e148781 * locals.var_jssw2_dn11), (assign96210_e148781 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96210_e148785;
        locals.var_isbd2_sws_dn0 = assign96210_e148785_d_n0;
        locals.var_isbd2_sws_dn2 = assign96210_e148785_d_n2;
        locals.var_isbd2_sws_dn4 = assign96210_e148785_d_n4;
        locals.var_isbd2_sws_dn5 = assign96210_e148785_d_n5;
        locals.var_isbd2_sws_dn6 = assign96210_e148785_d_n6;
        locals.var_isbd2_sws_dn7 = assign96210_e148785_d_n7;
        locals.var_isbd2_sws_dn8 = assign96210_e148785_d_n8;
        locals.var_isbd2_sws_dn9 = assign96210_e148785_d_n9;
        locals.var_isbd2_sws_dn10 = assign96210_e148785_d_n10;
        locals.var_isbd2_sws_dn11 = assign96210_e148785_d_n11;
        locals.var_isbd2_sws_dn14 = assign96210_e148785_d_n14;

        let (assign96220_e148795, assign96220_e148795_d_n0, assign96220_e148795_d_n2, assign96220_e148795_d_n4, assign96220_e148795_d_n5, assign96220_e148795_d_n6, assign96220_e148795_d_n7, assign96220_e148795_d_n8, assign96220_e148795_d_n9, assign96220_e148795_d_n10, assign96220_e148795_d_n11, assign96220_e148795_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96220_e148793: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96220_e148793, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96220_e148795;
        locals.var_isbd_swg_dn0 = assign96220_e148795_d_n0;
        locals.var_isbd_swg_dn2 = assign96220_e148795_d_n2;
        locals.var_isbd_swg_dn4 = assign96220_e148795_d_n4;
        locals.var_isbd_swg_dn5 = assign96220_e148795_d_n5;
        locals.var_isbd_swg_dn6 = assign96220_e148795_d_n6;
        locals.var_isbd_swg_dn7 = assign96220_e148795_d_n7;
        locals.var_isbd_swg_dn8 = assign96220_e148795_d_n8;
        locals.var_isbd_swg_dn9 = assign96220_e148795_d_n9;
        locals.var_isbd_swg_dn10 = assign96220_e148795_d_n10;
        locals.var_isbd_swg_dn11 = assign96220_e148795_d_n11;
        locals.var_isbd_swg_dn14 = assign96220_e148795_d_n14;

        let (assign96230_e148805, assign96230_e148805_d_n0, assign96230_e148805_d_n2, assign96230_e148805_d_n4, assign96230_e148805_d_n5, assign96230_e148805_d_n6, assign96230_e148805_d_n7, assign96230_e148805_d_n8, assign96230_e148805_d_n9, assign96230_e148805_d_n10, assign96230_e148805_d_n11, assign96230_e148805_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96230_e148803: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96230_e148803, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96230_e148805;
        locals.var_isbd2_swg_dn0 = assign96230_e148805_d_n0;
        locals.var_isbd2_swg_dn2 = assign96230_e148805_d_n2;
        locals.var_isbd2_swg_dn4 = assign96230_e148805_d_n4;
        locals.var_isbd2_swg_dn5 = assign96230_e148805_d_n5;
        locals.var_isbd2_swg_dn6 = assign96230_e148805_d_n6;
        locals.var_isbd2_swg_dn7 = assign96230_e148805_d_n7;
        locals.var_isbd2_swg_dn8 = assign96230_e148805_d_n8;
        locals.var_isbd2_swg_dn9 = assign96230_e148805_d_n9;
        locals.var_isbd2_swg_dn10 = assign96230_e148805_d_n10;
        locals.var_isbd2_swg_dn11 = assign96230_e148805_d_n11;
        locals.var_isbd2_swg_dn14 = assign96230_e148805_d_n14;

        let (assign96240_e148816, assign96240_e148816_d_n0, assign96240_e148816_d_n2, assign96240_e148816_d_n4, assign96240_e148816_d_n5, assign96240_e148816_d_n6, assign96240_e148816_d_n7, assign96240_e148816_d_n8, assign96240_e148816_d_n9, assign96240_e148816_d_n10, assign96240_e148816_d_n11, assign96240_e148816_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        let assign96240_e148814: f64 = (p.p13 * locals.var_js);
        (assign96240_e148814, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96240_e148816;
        locals.var_isbd_btm_dn0 = assign96240_e148816_d_n0;
        locals.var_isbd_btm_dn2 = assign96240_e148816_d_n2;
        locals.var_isbd_btm_dn4 = assign96240_e148816_d_n4;
        locals.var_isbd_btm_dn5 = assign96240_e148816_d_n5;
        locals.var_isbd_btm_dn6 = assign96240_e148816_d_n6;
        locals.var_isbd_btm_dn7 = assign96240_e148816_d_n7;
        locals.var_isbd_btm_dn8 = assign96240_e148816_d_n8;
        locals.var_isbd_btm_dn9 = assign96240_e148816_d_n9;
        locals.var_isbd_btm_dn10 = assign96240_e148816_d_n10;
        locals.var_isbd_btm_dn11 = assign96240_e148816_d_n11;
        locals.var_isbd_btm_dn14 = assign96240_e148816_d_n14;

        let (assign96250_e148827, assign96250_e148827_d_n0, assign96250_e148827_d_n2, assign96250_e148827_d_n4, assign96250_e148827_d_n5, assign96250_e148827_d_n6, assign96250_e148827_d_n7, assign96250_e148827_d_n8, assign96250_e148827_d_n9, assign96250_e148827_d_n10, assign96250_e148827_d_n11, assign96250_e148827_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        let assign96250_e148825: f64 = (p.p13 * locals.var_js2);
        (assign96250_e148825, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96250_e148827;
        locals.var_isbd2_btm_dn0 = assign96250_e148827_d_n0;
        locals.var_isbd2_btm_dn2 = assign96250_e148827_d_n2;
        locals.var_isbd2_btm_dn4 = assign96250_e148827_d_n4;
        locals.var_isbd2_btm_dn5 = assign96250_e148827_d_n5;
        locals.var_isbd2_btm_dn6 = assign96250_e148827_d_n6;
        locals.var_isbd2_btm_dn7 = assign96250_e148827_d_n7;
        locals.var_isbd2_btm_dn8 = assign96250_e148827_d_n8;
        locals.var_isbd2_btm_dn9 = assign96250_e148827_d_n9;
        locals.var_isbd2_btm_dn10 = assign96250_e148827_d_n10;
        locals.var_isbd2_btm_dn11 = assign96250_e148827_d_n11;
        locals.var_isbd2_btm_dn14 = assign96250_e148827_d_n14;

        let (assign96260_e148836, assign96260_e148836_d_n0, assign96260_e148836_d_n2, assign96260_e148836_d_n4, assign96260_e148836_d_n5, assign96260_e148836_d_n6, assign96260_e148836_d_n7, assign96260_e148836_d_n8, assign96260_e148836_d_n9, assign96260_e148836_d_n10, assign96260_e148836_d_n11, assign96260_e148836_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96260_e148836;
        locals.var_isbd_sws_dn0 = assign96260_e148836_d_n0;
        locals.var_isbd_sws_dn2 = assign96260_e148836_d_n2;
        locals.var_isbd_sws_dn4 = assign96260_e148836_d_n4;
        locals.var_isbd_sws_dn5 = assign96260_e148836_d_n5;
        locals.var_isbd_sws_dn6 = assign96260_e148836_d_n6;
        locals.var_isbd_sws_dn7 = assign96260_e148836_d_n7;
        locals.var_isbd_sws_dn8 = assign96260_e148836_d_n8;
        locals.var_isbd_sws_dn9 = assign96260_e148836_d_n9;
        locals.var_isbd_sws_dn10 = assign96260_e148836_d_n10;
        locals.var_isbd_sws_dn11 = assign96260_e148836_d_n11;
        locals.var_isbd_sws_dn14 = assign96260_e148836_d_n14;

    }

    pub(super) fn stamp_transient_block_355(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96270_e148845, assign96270_e148845_d_n0, assign96270_e148845_d_n2, assign96270_e148845_d_n4, assign96270_e148845_d_n5, assign96270_e148845_d_n6, assign96270_e148845_d_n7, assign96270_e148845_d_n8, assign96270_e148845_d_n9, assign96270_e148845_d_n10, assign96270_e148845_d_n11, assign96270_e148845_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96270_e148845;
        locals.var_isbd2_sws_dn0 = assign96270_e148845_d_n0;
        locals.var_isbd2_sws_dn2 = assign96270_e148845_d_n2;
        locals.var_isbd2_sws_dn4 = assign96270_e148845_d_n4;
        locals.var_isbd2_sws_dn5 = assign96270_e148845_d_n5;
        locals.var_isbd2_sws_dn6 = assign96270_e148845_d_n6;
        locals.var_isbd2_sws_dn7 = assign96270_e148845_d_n7;
        locals.var_isbd2_sws_dn8 = assign96270_e148845_d_n8;
        locals.var_isbd2_sws_dn9 = assign96270_e148845_d_n9;
        locals.var_isbd2_sws_dn10 = assign96270_e148845_d_n10;
        locals.var_isbd2_sws_dn11 = assign96270_e148845_d_n11;
        locals.var_isbd2_sws_dn14 = assign96270_e148845_d_n14;

        let (assign96280_e148856, assign96280_e148856_d_n0, assign96280_e148856_d_n2, assign96280_e148856_d_n4, assign96280_e148856_d_n5, assign96280_e148856_d_n6, assign96280_e148856_d_n7, assign96280_e148856_d_n8, assign96280_e148856_d_n9, assign96280_e148856_d_n10, assign96280_e148856_d_n11, assign96280_e148856_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        let assign96280_e148854: f64 = (p.p15 * locals.var_jsswg);
        (assign96280_e148854, (p.p15 * locals.var_jsswg_dn0), (p.p15 * locals.var_jsswg_dn2), (p.p15 * locals.var_jsswg_dn4), (p.p15 * locals.var_jsswg_dn5), (p.p15 * locals.var_jsswg_dn6), (p.p15 * locals.var_jsswg_dn7), (p.p15 * locals.var_jsswg_dn8), (p.p15 * locals.var_jsswg_dn9), (p.p15 * locals.var_jsswg_dn10), (p.p15 * locals.var_jsswg_dn11), (p.p15 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96280_e148856;
        locals.var_isbd_swg_dn0 = assign96280_e148856_d_n0;
        locals.var_isbd_swg_dn2 = assign96280_e148856_d_n2;
        locals.var_isbd_swg_dn4 = assign96280_e148856_d_n4;
        locals.var_isbd_swg_dn5 = assign96280_e148856_d_n5;
        locals.var_isbd_swg_dn6 = assign96280_e148856_d_n6;
        locals.var_isbd_swg_dn7 = assign96280_e148856_d_n7;
        locals.var_isbd_swg_dn8 = assign96280_e148856_d_n8;
        locals.var_isbd_swg_dn9 = assign96280_e148856_d_n9;
        locals.var_isbd_swg_dn10 = assign96280_e148856_d_n10;
        locals.var_isbd_swg_dn11 = assign96280_e148856_d_n11;
        locals.var_isbd_swg_dn14 = assign96280_e148856_d_n14;

        let (assign96290_e148867, assign96290_e148867_d_n0, assign96290_e148867_d_n2, assign96290_e148867_d_n4, assign96290_e148867_d_n5, assign96290_e148867_d_n6, assign96290_e148867_d_n7, assign96290_e148867_d_n8, assign96290_e148867_d_n9, assign96290_e148867_d_n10, assign96290_e148867_d_n11, assign96290_e148867_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        let assign96290_e148865: f64 = (p.p15 * locals.var_jsswg2);
        (assign96290_e148865, (p.p15 * locals.var_jsswg2_dn0), (p.p15 * locals.var_jsswg2_dn2), (p.p15 * locals.var_jsswg2_dn4), (p.p15 * locals.var_jsswg2_dn5), (p.p15 * locals.var_jsswg2_dn6), (p.p15 * locals.var_jsswg2_dn7), (p.p15 * locals.var_jsswg2_dn8), (p.p15 * locals.var_jsswg2_dn9), (p.p15 * locals.var_jsswg2_dn10), (p.p15 * locals.var_jsswg2_dn11), (p.p15 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96290_e148867;
        locals.var_isbd2_swg_dn0 = assign96290_e148867_d_n0;
        locals.var_isbd2_swg_dn2 = assign96290_e148867_d_n2;
        locals.var_isbd2_swg_dn4 = assign96290_e148867_d_n4;
        locals.var_isbd2_swg_dn5 = assign96290_e148867_d_n5;
        locals.var_isbd2_swg_dn6 = assign96290_e148867_d_n6;
        locals.var_isbd2_swg_dn7 = assign96290_e148867_d_n7;
        locals.var_isbd2_swg_dn8 = assign96290_e148867_d_n8;
        locals.var_isbd2_swg_dn9 = assign96290_e148867_d_n9;
        locals.var_isbd2_swg_dn10 = assign96290_e148867_d_n10;
        locals.var_isbd2_swg_dn11 = assign96290_e148867_d_n11;
        locals.var_isbd2_swg_dn14 = assign96290_e148867_d_n14;

        let (assign96300_e148876, assign96300_e148876_d_n0, assign96300_e148876_d_n2, assign96300_e148876_d_n4, assign96300_e148876_d_n5, assign96300_e148876_d_n6, assign96300_e148876_d_n7, assign96300_e148876_d_n8, assign96300_e148876_d_n9, assign96300_e148876_d_n10, assign96300_e148876_d_n11, assign96300_e148876_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        let assign96300_e148874: f64 = (p.p13 * locals.var_js);
        (assign96300_e148874, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96300_e148876;
        locals.var_isbd_btm_dn0 = assign96300_e148876_d_n0;
        locals.var_isbd_btm_dn2 = assign96300_e148876_d_n2;
        locals.var_isbd_btm_dn4 = assign96300_e148876_d_n4;
        locals.var_isbd_btm_dn5 = assign96300_e148876_d_n5;
        locals.var_isbd_btm_dn6 = assign96300_e148876_d_n6;
        locals.var_isbd_btm_dn7 = assign96300_e148876_d_n7;
        locals.var_isbd_btm_dn8 = assign96300_e148876_d_n8;
        locals.var_isbd_btm_dn9 = assign96300_e148876_d_n9;
        locals.var_isbd_btm_dn10 = assign96300_e148876_d_n10;
        locals.var_isbd_btm_dn11 = assign96300_e148876_d_n11;
        locals.var_isbd_btm_dn14 = assign96300_e148876_d_n14;

        let (assign96310_e148885, assign96310_e148885_d_n0, assign96310_e148885_d_n2, assign96310_e148885_d_n4, assign96310_e148885_d_n5, assign96310_e148885_d_n6, assign96310_e148885_d_n7, assign96310_e148885_d_n8, assign96310_e148885_d_n9, assign96310_e148885_d_n10, assign96310_e148885_d_n11, assign96310_e148885_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        let assign96310_e148883: f64 = (p.p13 * locals.var_js2);
        (assign96310_e148883, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96310_e148885;
        locals.var_isbd2_btm_dn0 = assign96310_e148885_d_n0;
        locals.var_isbd2_btm_dn2 = assign96310_e148885_d_n2;
        locals.var_isbd2_btm_dn4 = assign96310_e148885_d_n4;
        locals.var_isbd2_btm_dn5 = assign96310_e148885_d_n5;
        locals.var_isbd2_btm_dn6 = assign96310_e148885_d_n6;
        locals.var_isbd2_btm_dn7 = assign96310_e148885_d_n7;
        locals.var_isbd2_btm_dn8 = assign96310_e148885_d_n8;
        locals.var_isbd2_btm_dn9 = assign96310_e148885_d_n9;
        locals.var_isbd2_btm_dn10 = assign96310_e148885_d_n10;
        locals.var_isbd2_btm_dn11 = assign96310_e148885_d_n11;
        locals.var_isbd2_btm_dn14 = assign96310_e148885_d_n14;

        let (assign96320_e148894, assign96320_e148894_d_n0, assign96320_e148894_d_n2, assign96320_e148894_d_n4, assign96320_e148894_d_n5, assign96320_e148894_d_n6, assign96320_e148894_d_n7, assign96320_e148894_d_n8, assign96320_e148894_d_n9, assign96320_e148894_d_n10, assign96320_e148894_d_n11, assign96320_e148894_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        let assign96320_e148892: f64 = (p.p15 * locals.var_jssw);
        (assign96320_e148892, (p.p15 * locals.var_jssw_dn0), (p.p15 * locals.var_jssw_dn2), (p.p15 * locals.var_jssw_dn4), (p.p15 * locals.var_jssw_dn5), (p.p15 * locals.var_jssw_dn6), (p.p15 * locals.var_jssw_dn7), (p.p15 * locals.var_jssw_dn8), (p.p15 * locals.var_jssw_dn9), (p.p15 * locals.var_jssw_dn10), (p.p15 * locals.var_jssw_dn11), (p.p15 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96320_e148894;
        locals.var_isbd_sws_dn0 = assign96320_e148894_d_n0;
        locals.var_isbd_sws_dn2 = assign96320_e148894_d_n2;
        locals.var_isbd_sws_dn4 = assign96320_e148894_d_n4;
        locals.var_isbd_sws_dn5 = assign96320_e148894_d_n5;
        locals.var_isbd_sws_dn6 = assign96320_e148894_d_n6;
        locals.var_isbd_sws_dn7 = assign96320_e148894_d_n7;
        locals.var_isbd_sws_dn8 = assign96320_e148894_d_n8;
        locals.var_isbd_sws_dn9 = assign96320_e148894_d_n9;
        locals.var_isbd_sws_dn10 = assign96320_e148894_d_n10;
        locals.var_isbd_sws_dn11 = assign96320_e148894_d_n11;
        locals.var_isbd_sws_dn14 = assign96320_e148894_d_n14;

        let (assign96330_e148903, assign96330_e148903_d_n0, assign96330_e148903_d_n2, assign96330_e148903_d_n4, assign96330_e148903_d_n5, assign96330_e148903_d_n6, assign96330_e148903_d_n7, assign96330_e148903_d_n8, assign96330_e148903_d_n9, assign96330_e148903_d_n10, assign96330_e148903_d_n11, assign96330_e148903_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        let assign96330_e148901: f64 = (p.p15 * locals.var_jssw2);
        (assign96330_e148901, (p.p15 * locals.var_jssw2_dn0), (p.p15 * locals.var_jssw2_dn2), (p.p15 * locals.var_jssw2_dn4), (p.p15 * locals.var_jssw2_dn5), (p.p15 * locals.var_jssw2_dn6), (p.p15 * locals.var_jssw2_dn7), (p.p15 * locals.var_jssw2_dn8), (p.p15 * locals.var_jssw2_dn9), (p.p15 * locals.var_jssw2_dn10), (p.p15 * locals.var_jssw2_dn11), (p.p15 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96330_e148903;
        locals.var_isbd2_sws_dn0 = assign96330_e148903_d_n0;
        locals.var_isbd2_sws_dn2 = assign96330_e148903_d_n2;
        locals.var_isbd2_sws_dn4 = assign96330_e148903_d_n4;
        locals.var_isbd2_sws_dn5 = assign96330_e148903_d_n5;
        locals.var_isbd2_sws_dn6 = assign96330_e148903_d_n6;
        locals.var_isbd2_sws_dn7 = assign96330_e148903_d_n7;
        locals.var_isbd2_sws_dn8 = assign96330_e148903_d_n8;
        locals.var_isbd2_sws_dn9 = assign96330_e148903_d_n9;
        locals.var_isbd2_sws_dn10 = assign96330_e148903_d_n10;
        locals.var_isbd2_sws_dn11 = assign96330_e148903_d_n11;
        locals.var_isbd2_sws_dn14 = assign96330_e148903_d_n14;

        let (assign96340_e148910, assign96340_e148910_d_n0, assign96340_e148910_d_n2, assign96340_e148910_d_n4, assign96340_e148910_d_n5, assign96340_e148910_d_n6, assign96340_e148910_d_n7, assign96340_e148910_d_n8, assign96340_e148910_d_n9, assign96340_e148910_d_n10, assign96340_e148910_d_n11, assign96340_e148910_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96340_e148910;
        locals.var_isbd_swg_dn0 = assign96340_e148910_d_n0;
        locals.var_isbd_swg_dn2 = assign96340_e148910_d_n2;
        locals.var_isbd_swg_dn4 = assign96340_e148910_d_n4;
        locals.var_isbd_swg_dn5 = assign96340_e148910_d_n5;
        locals.var_isbd_swg_dn6 = assign96340_e148910_d_n6;
        locals.var_isbd_swg_dn7 = assign96340_e148910_d_n7;
        locals.var_isbd_swg_dn8 = assign96340_e148910_d_n8;
        locals.var_isbd_swg_dn9 = assign96340_e148910_d_n9;
        locals.var_isbd_swg_dn10 = assign96340_e148910_d_n10;
        locals.var_isbd_swg_dn11 = assign96340_e148910_d_n11;
        locals.var_isbd_swg_dn14 = assign96340_e148910_d_n14;

        let (assign96350_e148917, assign96350_e148917_d_n0, assign96350_e148917_d_n2, assign96350_e148917_d_n4, assign96350_e148917_d_n5, assign96350_e148917_d_n6, assign96350_e148917_d_n7, assign96350_e148917_d_n8, assign96350_e148917_d_n9, assign96350_e148917_d_n10, assign96350_e148917_d_n11, assign96350_e148917_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96350_e148917;
        locals.var_isbd2_swg_dn0 = assign96350_e148917_d_n0;
        locals.var_isbd2_swg_dn2 = assign96350_e148917_d_n2;
        locals.var_isbd2_swg_dn4 = assign96350_e148917_d_n4;
        locals.var_isbd2_swg_dn5 = assign96350_e148917_d_n5;
        locals.var_isbd2_swg_dn6 = assign96350_e148917_d_n6;
        locals.var_isbd2_swg_dn7 = assign96350_e148917_d_n7;
        locals.var_isbd2_swg_dn8 = assign96350_e148917_d_n8;
        locals.var_isbd2_swg_dn9 = assign96350_e148917_d_n9;
        locals.var_isbd2_swg_dn10 = assign96350_e148917_d_n10;
        locals.var_isbd2_swg_dn11 = assign96350_e148917_d_n11;
        locals.var_isbd2_swg_dn14 = assign96350_e148917_d_n14;

        let (assign96360_e148925, assign96360_e148925_d_n0, assign96360_e148925_d_n2, assign96360_e148925_d_n4, assign96360_e148925_d_n5, assign96360_e148925_d_n6, assign96360_e148925_d_n7, assign96360_e148925_d_n8, assign96360_e148925_d_n9, assign96360_e148925_d_n10, assign96360_e148925_d_n11, assign96360_e148925_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96360_e148921: f64 = (locals.var_isbd_btm + locals.var_isbd_sws);
        let assign96360_e148923: f64 = (assign96360_e148921 + locals.var_isbd_swg);
        (assign96360_e148923, ((locals.var_isbd_btm_dn0 + locals.var_isbd_sws_dn0) + locals.var_isbd_swg_dn0), ((locals.var_isbd_btm_dn2 + locals.var_isbd_sws_dn2) + locals.var_isbd_swg_dn2), ((locals.var_isbd_btm_dn4 + locals.var_isbd_sws_dn4) + locals.var_isbd_swg_dn4), ((locals.var_isbd_btm_dn5 + locals.var_isbd_sws_dn5) + locals.var_isbd_swg_dn5), ((locals.var_isbd_btm_dn6 + locals.var_isbd_sws_dn6) + locals.var_isbd_swg_dn6), ((locals.var_isbd_btm_dn7 + locals.var_isbd_sws_dn7) + locals.var_isbd_swg_dn7), ((locals.var_isbd_btm_dn8 + locals.var_isbd_sws_dn8) + locals.var_isbd_swg_dn8), ((locals.var_isbd_btm_dn9 + locals.var_isbd_sws_dn9) + locals.var_isbd_swg_dn9), ((locals.var_isbd_btm_dn10 + locals.var_isbd_sws_dn10) + locals.var_isbd_swg_dn10), ((locals.var_isbd_btm_dn11 + locals.var_isbd_sws_dn11) + locals.var_isbd_swg_dn11), ((locals.var_isbd_btm_dn14 + locals.var_isbd_sws_dn14) + locals.var_isbd_swg_dn14),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    }
};
        locals.var_isbd = assign96360_e148925;
        locals.var_isbd_dn0 = assign96360_e148925_d_n0;
        locals.var_isbd_dn2 = assign96360_e148925_d_n2;
        locals.var_isbd_dn4 = assign96360_e148925_d_n4;
        locals.var_isbd_dn5 = assign96360_e148925_d_n5;
        locals.var_isbd_dn6 = assign96360_e148925_d_n6;
        locals.var_isbd_dn7 = assign96360_e148925_d_n7;
        locals.var_isbd_dn8 = assign96360_e148925_d_n8;
        locals.var_isbd_dn9 = assign96360_e148925_d_n9;
        locals.var_isbd_dn10 = assign96360_e148925_d_n10;
        locals.var_isbd_dn11 = assign96360_e148925_d_n11;
        locals.var_isbd_dn14 = assign96360_e148925_d_n14;

        let assign96370_e148928: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2240 = assign96370_e148928;

        let (assign96380_e148936, assign96380_e148936_d_n0, assign96380_e148936_d_n2, assign96380_e148936_d_n4, assign96380_e148936_d_n5, assign96380_e148936_d_n6, assign96380_e148936_d_n7, assign96380_e148936_d_n8, assign96380_e148936_d_n9, assign96380_e148936_d_n10, assign96380_e148936_d_n11, assign96380_e148936_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96380_e148934: f64 = (locals.var_isbd + 1e-25);
        (assign96380_e148934, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign96380_e148936;
        locals.var_t2_dn0 = assign96380_e148936_d_n0;
        locals.var_t2_dn2 = assign96380_e148936_d_n2;
        locals.var_t2_dn4 = assign96380_e148936_d_n4;
        locals.var_t2_dn5 = assign96380_e148936_d_n5;
        locals.var_t2_dn6 = assign96380_e148936_d_n6;
        locals.var_t2_dn7 = assign96380_e148936_d_n7;
        locals.var_t2_dn8 = assign96380_e148936_d_n8;
        locals.var_t2_dn9 = assign96380_e148936_d_n9;
        locals.var_t2_dn10 = assign96380_e148936_d_n10;
        locals.var_t2_dn11 = assign96380_e148936_d_n11;
        locals.var_t2_dn14 = assign96380_e148936_d_n14;

        let (assign96390_e148953, assign96390_e148953_d_n0, assign96390_e148953_d_n2, assign96390_e148953_d_n4, assign96390_e148953_d_n5, assign96390_e148953_d_n6, assign96390_e148953_d_n7, assign96390_e148953_d_n8, assign96390_e148953_d_n9, assign96390_e148953_d_n10, assign96390_e148953_d_n11, assign96390_e148953_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96390_e148942: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96390_e148945: f64 = (locals.var_uc_vdiffjd * locals.var_t0);
        let assign96390_e148947: f64 = (assign96390_e148945 / locals.var_t2);
        let assign96390_e148949: f64 = (assign96390_e148947 + 1.0);
        let assign96390_e148950: f64 = (assign96390_e148949).ln();
        let assign96390_e148951: f64 = (assign96390_e148942 * assign96390_e148950);
        (assign96390_e148951, (((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn0) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn2) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn4) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn5) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn6) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn7) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn8) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn9) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn10) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn11) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn14) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))),)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn0, locals.var_vbdt_dn2, locals.var_vbdt_dn4, locals.var_vbdt_dn5, locals.var_vbdt_dn6, locals.var_vbdt_dn7, locals.var_vbdt_dn8, locals.var_vbdt_dn9, locals.var_vbdt_dn10, locals.var_vbdt_dn11, locals.var_vbdt_dn14,)
    }
};
        locals.var_vbdt = assign96390_e148953;
        locals.var_vbdt_dn0 = assign96390_e148953_d_n0;
        locals.var_vbdt_dn2 = assign96390_e148953_d_n2;
        locals.var_vbdt_dn4 = assign96390_e148953_d_n4;
        locals.var_vbdt_dn5 = assign96390_e148953_d_n5;
        locals.var_vbdt_dn6 = assign96390_e148953_d_n6;
        locals.var_vbdt_dn7 = assign96390_e148953_d_n7;
        locals.var_vbdt_dn8 = assign96390_e148953_d_n8;
        locals.var_vbdt_dn9 = assign96390_e148953_d_n9;
        locals.var_vbdt_dn10 = assign96390_e148953_d_n10;
        locals.var_vbdt_dn11 = assign96390_e148953_d_n11;
        locals.var_vbdt_dn14 = assign96390_e148953_d_n14;

        let (assign96400_e148964, assign96400_e148964_d_n0, assign96400_e148964_d_n2, assign96400_e148964_d_n4, assign96400_e148964_d_n5, assign96400_e148964_d_n6, assign96400_e148964_d_n7, assign96400_e148964_d_n8, assign96400_e148964_d_n9, assign96400_e148964_d_n10, assign96400_e148964_d_n11, assign96400_e148964_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96400_e148959: f64 = (locals.var_tratio - 1.0);
        let assign96400_e148961: f64 = (assign96400_e148959 * p.p512);
        let assign96400_e148962: f64 = (assign96400_e148961).exp();
        (assign96400_e148962, (assign96400_e148962 * (locals.var_tratio_dn0 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn2 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn4 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn5 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn6 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn7 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn8 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn9 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn10 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn11 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn14 * p.p512)),)
    } else {
        (locals.var_exptempd, locals.var_exptempd_dn0, locals.var_exptempd_dn2, locals.var_exptempd_dn4, locals.var_exptempd_dn5, locals.var_exptempd_dn6, locals.var_exptempd_dn7, locals.var_exptempd_dn8, locals.var_exptempd_dn9, locals.var_exptempd_dn10, locals.var_exptempd_dn11, locals.var_exptempd_dn14,)
    }
};
        locals.var_exptempd = assign96400_e148964;
        locals.var_exptempd_dn0 = assign96400_e148964_d_n0;
        locals.var_exptempd_dn2 = assign96400_e148964_d_n2;
        locals.var_exptempd_dn4 = assign96400_e148964_d_n4;
        locals.var_exptempd_dn5 = assign96400_e148964_d_n5;
        locals.var_exptempd_dn6 = assign96400_e148964_d_n6;
        locals.var_exptempd_dn7 = assign96400_e148964_d_n7;
        locals.var_exptempd_dn8 = assign96400_e148964_d_n8;
        locals.var_exptempd_dn9 = assign96400_e148964_d_n9;
        locals.var_exptempd_dn10 = assign96400_e148964_d_n10;
        locals.var_exptempd_dn11 = assign96400_e148964_d_n11;
        locals.var_exptempd_dn14 = assign96400_e148964_d_n14;

        let (assign96410_e148974, assign96410_e148974_d_n0, assign96410_e148974_d_n2, assign96410_e148974_d_n4, assign96410_e148974_d_n5, assign96410_e148974_d_n6, assign96410_e148974_d_n7, assign96410_e148974_d_n8, assign96410_e148974_d_n9, assign96410_e148974_d_n10, assign96410_e148974_d_n11, assign96410_e148974_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96410_e148971: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96410_e148972: f64 = (1.0 / assign96410_e148971);
        (assign96410_e148972, (-((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))),)
    } else {
        (locals.var_jd_nvtm_invd, locals.var_jd_nvtm_invd_dn0, locals.var_jd_nvtm_invd_dn2, locals.var_jd_nvtm_invd_dn4, locals.var_jd_nvtm_invd_dn5, locals.var_jd_nvtm_invd_dn6, locals.var_jd_nvtm_invd_dn7, locals.var_jd_nvtm_invd_dn8, locals.var_jd_nvtm_invd_dn9, locals.var_jd_nvtm_invd_dn10, locals.var_jd_nvtm_invd_dn11, locals.var_jd_nvtm_invd_dn14,)
    }
};
        locals.var_jd_nvtm_invd = assign96410_e148974;
        locals.var_jd_nvtm_invd_dn0 = assign96410_e148974_d_n0;
        locals.var_jd_nvtm_invd_dn2 = assign96410_e148974_d_n2;
        locals.var_jd_nvtm_invd_dn4 = assign96410_e148974_d_n4;
        locals.var_jd_nvtm_invd_dn5 = assign96410_e148974_d_n5;
        locals.var_jd_nvtm_invd_dn6 = assign96410_e148974_d_n6;
        locals.var_jd_nvtm_invd_dn7 = assign96410_e148974_d_n7;
        locals.var_jd_nvtm_invd_dn8 = assign96410_e148974_d_n8;
        locals.var_jd_nvtm_invd_dn9 = assign96410_e148974_d_n9;
        locals.var_jd_nvtm_invd_dn10 = assign96410_e148974_d_n10;
        locals.var_jd_nvtm_invd_dn11 = assign96410_e148974_d_n11;
        locals.var_jd_nvtm_invd_dn14 = assign96410_e148974_d_n14;

        let (assign96420_e148983, assign96420_e148983_d_n0, assign96420_e148983_d_n2, assign96420_e148983_d_n4, assign96420_e148983_d_n5, assign96420_e148983_d_n6, assign96420_e148983_d_n7, assign96420_e148983_d_n8, assign96420_e148983_d_n9, assign96420_e148983_d_n10, assign96420_e148983_d_n11, assign96420_e148983_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96420_e148980: f64 = (locals.var_vbdt * locals.var_jd_nvtm_invd);
        let assign96420_e148981: f64 = (assign96420_e148980).exp();
        (assign96420_e148981, (assign96420_e148981 * ((locals.var_vbdt_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn0))), (assign96420_e148981 * ((locals.var_vbdt_dn2 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn2))), (assign96420_e148981 * ((locals.var_vbdt_dn4 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn4))), (assign96420_e148981 * ((locals.var_vbdt_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn5))), (assign96420_e148981 * ((locals.var_vbdt_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn6))), (assign96420_e148981 * ((locals.var_vbdt_dn7 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn7))), (assign96420_e148981 * ((locals.var_vbdt_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn8))), (assign96420_e148981 * ((locals.var_vbdt_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn9))), (assign96420_e148981 * ((locals.var_vbdt_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn10))), (assign96420_e148981 * ((locals.var_vbdt_dn11 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn11))), (assign96420_e148981 * ((locals.var_vbdt_dn14 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn14))),)
    } else {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    }
};
        locals.var_jd_expcd = assign96420_e148983;
        locals.var_jd_expcd_dn0 = assign96420_e148983_d_n0;
        locals.var_jd_expcd_dn2 = assign96420_e148983_d_n2;
        locals.var_jd_expcd_dn4 = assign96420_e148983_d_n4;
        locals.var_jd_expcd_dn5 = assign96420_e148983_d_n5;
        locals.var_jd_expcd_dn6 = assign96420_e148983_d_n6;
        locals.var_jd_expcd_dn7 = assign96420_e148983_d_n7;
        locals.var_jd_expcd_dn8 = assign96420_e148983_d_n8;
        locals.var_jd_expcd_dn9 = assign96420_e148983_d_n9;
        locals.var_jd_expcd_dn10 = assign96420_e148983_d_n10;
        locals.var_jd_expcd_dn11 = assign96420_e148983_d_n11;
        locals.var_jd_expcd_dn14 = assign96420_e148983_d_n14;

        let (assign96430_e149002, assign96430_e149002_d_n0, assign96430_e149002_d_n2, assign96430_e149002_d_n4, assign96430_e149002_d_n5, assign96430_e149002_d_n6, assign96430_e149002_d_n7, assign96430_e149002_d_n8, assign96430_e149002_d_n9, assign96430_e149002_d_n10, assign96430_e149002_d_n11, assign96430_e149002_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96430_e148988: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96430_e148991: f64 = (locals.var_eg * locals.var_beta);
        let assign96430_e148992: f64 = (assign96430_e148988 - assign96430_e148991);
        let assign96430_e148995: f64 = (p.p522 * locals.var_log_tratio);
        let assign96430_e148996: f64 = (assign96430_e148992 + assign96430_e148995);
        let assign96430_e148998: f64 = (assign96430_e148996 / locals.var_uc_njs);
        let assign96430_e148999: f64 = (assign96430_e148998).exp();
        let assign96430_e149000: f64 = (locals.var_uc_js0s * assign96430_e148999);
        (assign96430_e149000, (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign96430_e149002;
        locals.var_js_dn0 = assign96430_e149002_d_n0;
        locals.var_js_dn2 = assign96430_e149002_d_n2;
        locals.var_js_dn4 = assign96430_e149002_d_n4;
        locals.var_js_dn5 = assign96430_e149002_d_n5;
        locals.var_js_dn6 = assign96430_e149002_d_n6;
        locals.var_js_dn7 = assign96430_e149002_d_n7;
        locals.var_js_dn8 = assign96430_e149002_d_n8;
        locals.var_js_dn9 = assign96430_e149002_d_n9;
        locals.var_js_dn10 = assign96430_e149002_d_n10;
        locals.var_js_dn11 = assign96430_e149002_d_n11;
        locals.var_js_dn14 = assign96430_e149002_d_n14;

        let (assign96440_e149021, assign96440_e149021_d_n0, assign96440_e149021_d_n2, assign96440_e149021_d_n4, assign96440_e149021_d_n5, assign96440_e149021_d_n6, assign96440_e149021_d_n7, assign96440_e149021_d_n8, assign96440_e149021_d_n9, assign96440_e149021_d_n10, assign96440_e149021_d_n11, assign96440_e149021_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96440_e149007: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96440_e149010: f64 = (locals.var_eg * locals.var_beta);
        let assign96440_e149011: f64 = (assign96440_e149007 - assign96440_e149010);
        let assign96440_e149014: f64 = (p.p522 * locals.var_log_tratio);
        let assign96440_e149015: f64 = (assign96440_e149011 + assign96440_e149014);
        let assign96440_e149017: f64 = (assign96440_e149015 / p.p520);
        let assign96440_e149018: f64 = (assign96440_e149017).exp();
        let assign96440_e149019: f64 = (locals.var_uc_js0sws * assign96440_e149018);
        (assign96440_e149019, (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign96440_e149021;
        locals.var_jssw_dn0 = assign96440_e149021_d_n0;
        locals.var_jssw_dn2 = assign96440_e149021_d_n2;
        locals.var_jssw_dn4 = assign96440_e149021_d_n4;
        locals.var_jssw_dn5 = assign96440_e149021_d_n5;
        locals.var_jssw_dn6 = assign96440_e149021_d_n6;
        locals.var_jssw_dn7 = assign96440_e149021_d_n7;
        locals.var_jssw_dn8 = assign96440_e149021_d_n8;
        locals.var_jssw_dn9 = assign96440_e149021_d_n9;
        locals.var_jssw_dn10 = assign96440_e149021_d_n10;
        locals.var_jssw_dn11 = assign96440_e149021_d_n11;
        locals.var_jssw_dn14 = assign96440_e149021_d_n14;

        let (assign96450_e149040, assign96450_e149040_d_n0, assign96450_e149040_d_n2, assign96450_e149040_d_n4, assign96450_e149040_d_n5, assign96450_e149040_d_n6, assign96450_e149040_d_n7, assign96450_e149040_d_n8, assign96450_e149040_d_n9, assign96450_e149040_d_n10, assign96450_e149040_d_n11, assign96450_e149040_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96450_e149026: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96450_e149029: f64 = (locals.var_eg * locals.var_beta);
        let assign96450_e149030: f64 = (assign96450_e149026 - assign96450_e149029);
        let assign96450_e149033: f64 = (p.p522 * locals.var_log_tratio);
        let assign96450_e149034: f64 = (assign96450_e149030 + assign96450_e149033);
        let assign96450_e149036: f64 = (assign96450_e149034 / p.p521);
        let assign96450_e149037: f64 = (assign96450_e149036).exp();
        let assign96450_e149038: f64 = (p.p518 * assign96450_e149037);
        (assign96450_e149038, (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign96450_e149040;
        locals.var_jsswg_dn0 = assign96450_e149040_d_n0;
        locals.var_jsswg_dn2 = assign96450_e149040_d_n2;
        locals.var_jsswg_dn4 = assign96450_e149040_d_n4;
        locals.var_jsswg_dn5 = assign96450_e149040_d_n5;
        locals.var_jsswg_dn6 = assign96450_e149040_d_n6;
        locals.var_jsswg_dn7 = assign96450_e149040_d_n7;
        locals.var_jsswg_dn8 = assign96450_e149040_d_n8;
        locals.var_jsswg_dn9 = assign96450_e149040_d_n9;
        locals.var_jsswg_dn10 = assign96450_e149040_d_n10;
        locals.var_jsswg_dn11 = assign96450_e149040_d_n11;
        locals.var_jsswg_dn14 = assign96450_e149040_d_n14;

        let (assign96460_e149059, assign96460_e149059_d_n0, assign96460_e149059_d_n2, assign96460_e149059_d_n4, assign96460_e149059_d_n5, assign96460_e149059_d_n6, assign96460_e149059_d_n7, assign96460_e149059_d_n8, assign96460_e149059_d_n9, assign96460_e149059_d_n10, assign96460_e149059_d_n11, assign96460_e149059_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96460_e149045: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96460_e149048: f64 = (locals.var_eg * locals.var_beta);
        let assign96460_e149049: f64 = (assign96460_e149045 - assign96460_e149048);
        let assign96460_e149052: f64 = (p.p532 * locals.var_log_tratio);
        let assign96460_e149053: f64 = (assign96460_e149049 + assign96460_e149052);
        let assign96460_e149055: f64 = (assign96460_e149053 / locals.var_uc_njs);
        let assign96460_e149056: f64 = (assign96460_e149055).exp();
        let assign96460_e149057: f64 = (locals.var_uc_js0s * assign96460_e149056);
        (assign96460_e149057, (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign96460_e149059;
        locals.var_js2_dn0 = assign96460_e149059_d_n0;
        locals.var_js2_dn2 = assign96460_e149059_d_n2;
        locals.var_js2_dn4 = assign96460_e149059_d_n4;
        locals.var_js2_dn5 = assign96460_e149059_d_n5;
        locals.var_js2_dn6 = assign96460_e149059_d_n6;
        locals.var_js2_dn7 = assign96460_e149059_d_n7;
        locals.var_js2_dn8 = assign96460_e149059_d_n8;
        locals.var_js2_dn9 = assign96460_e149059_d_n9;
        locals.var_js2_dn10 = assign96460_e149059_d_n10;
        locals.var_js2_dn11 = assign96460_e149059_d_n11;
        locals.var_js2_dn14 = assign96460_e149059_d_n14;

        let (assign96470_e149078, assign96470_e149078_d_n0, assign96470_e149078_d_n2, assign96470_e149078_d_n4, assign96470_e149078_d_n5, assign96470_e149078_d_n6, assign96470_e149078_d_n7, assign96470_e149078_d_n8, assign96470_e149078_d_n9, assign96470_e149078_d_n10, assign96470_e149078_d_n11, assign96470_e149078_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96470_e149064: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96470_e149067: f64 = (locals.var_eg * locals.var_beta);
        let assign96470_e149068: f64 = (assign96470_e149064 - assign96470_e149067);
        let assign96470_e149071: f64 = (p.p532 * locals.var_log_tratio);
        let assign96470_e149072: f64 = (assign96470_e149068 + assign96470_e149071);
        let assign96470_e149074: f64 = (assign96470_e149072 / p.p520);
        let assign96470_e149075: f64 = (assign96470_e149074).exp();
        let assign96470_e149076: f64 = (locals.var_uc_js0sws * assign96470_e149075);
        (assign96470_e149076, (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign96470_e149078;
        locals.var_jssw2_dn0 = assign96470_e149078_d_n0;
        locals.var_jssw2_dn2 = assign96470_e149078_d_n2;
        locals.var_jssw2_dn4 = assign96470_e149078_d_n4;
        locals.var_jssw2_dn5 = assign96470_e149078_d_n5;
        locals.var_jssw2_dn6 = assign96470_e149078_d_n6;
        locals.var_jssw2_dn7 = assign96470_e149078_d_n7;
        locals.var_jssw2_dn8 = assign96470_e149078_d_n8;
        locals.var_jssw2_dn9 = assign96470_e149078_d_n9;
        locals.var_jssw2_dn10 = assign96470_e149078_d_n10;
        locals.var_jssw2_dn11 = assign96470_e149078_d_n11;
        locals.var_jssw2_dn14 = assign96470_e149078_d_n14;

        let (assign96480_e149097, assign96480_e149097_d_n0, assign96480_e149097_d_n2, assign96480_e149097_d_n4, assign96480_e149097_d_n5, assign96480_e149097_d_n6, assign96480_e149097_d_n7, assign96480_e149097_d_n8, assign96480_e149097_d_n9, assign96480_e149097_d_n10, assign96480_e149097_d_n11, assign96480_e149097_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96480_e149083: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96480_e149086: f64 = (locals.var_eg * locals.var_beta);
        let assign96480_e149087: f64 = (assign96480_e149083 - assign96480_e149086);
        let assign96480_e149090: f64 = (p.p532 * locals.var_log_tratio);
        let assign96480_e149091: f64 = (assign96480_e149087 + assign96480_e149090);
        let assign96480_e149093: f64 = (assign96480_e149091 / p.p521);
        let assign96480_e149094: f64 = (assign96480_e149093).exp();
        let assign96480_e149095: f64 = (p.p518 * assign96480_e149094);
        (assign96480_e149095, (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign96480_e149097;
        locals.var_jsswg2_dn0 = assign96480_e149097_d_n0;
        locals.var_jsswg2_dn2 = assign96480_e149097_d_n2;
        locals.var_jsswg2_dn4 = assign96480_e149097_d_n4;
        locals.var_jsswg2_dn5 = assign96480_e149097_d_n5;
        locals.var_jsswg2_dn6 = assign96480_e149097_d_n6;
        locals.var_jsswg2_dn7 = assign96480_e149097_d_n7;
        locals.var_jsswg2_dn8 = assign96480_e149097_d_n8;
        locals.var_jsswg2_dn9 = assign96480_e149097_d_n9;
        locals.var_jsswg2_dn10 = assign96480_e149097_d_n10;
        locals.var_jsswg2_dn11 = assign96480_e149097_d_n11;
        locals.var_jsswg2_dn14 = assign96480_e149097_d_n14;

        let assign96490_e149100: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2241 = assign96490_e149100;

        let assign96500_e149103: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2242 = assign96500_e149103;

    }

    pub(super) fn stamp_transient_block_356(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96510_e149113, assign96510_e149113_d_n0, assign96510_e149113_d_n2, assign96510_e149113_d_n4, assign96510_e149113_d_n5, assign96510_e149113_d_n6, assign96510_e149113_d_n7, assign96510_e149113_d_n8, assign96510_e149113_d_n9, assign96510_e149113_d_n10, assign96510_e149113_d_n11, assign96510_e149113_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96510_e149111: f64 = (p.p14 * locals.var_js);
        (assign96510_e149111, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96510_e149113;
        locals.var_isbs_btm_dn0 = assign96510_e149113_d_n0;
        locals.var_isbs_btm_dn2 = assign96510_e149113_d_n2;
        locals.var_isbs_btm_dn4 = assign96510_e149113_d_n4;
        locals.var_isbs_btm_dn5 = assign96510_e149113_d_n5;
        locals.var_isbs_btm_dn6 = assign96510_e149113_d_n6;
        locals.var_isbs_btm_dn7 = assign96510_e149113_d_n7;
        locals.var_isbs_btm_dn8 = assign96510_e149113_d_n8;
        locals.var_isbs_btm_dn9 = assign96510_e149113_d_n9;
        locals.var_isbs_btm_dn10 = assign96510_e149113_d_n10;
        locals.var_isbs_btm_dn11 = assign96510_e149113_d_n11;
        locals.var_isbs_btm_dn14 = assign96510_e149113_d_n14;

        let (assign96520_e149123, assign96520_e149123_d_n0, assign96520_e149123_d_n2, assign96520_e149123_d_n4, assign96520_e149123_d_n5, assign96520_e149123_d_n6, assign96520_e149123_d_n7, assign96520_e149123_d_n8, assign96520_e149123_d_n9, assign96520_e149123_d_n10, assign96520_e149123_d_n11, assign96520_e149123_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96520_e149121: f64 = (p.p14 * locals.var_js2);
        (assign96520_e149121, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96520_e149123;
        locals.var_isbs2_btm_dn0 = assign96520_e149123_d_n0;
        locals.var_isbs2_btm_dn2 = assign96520_e149123_d_n2;
        locals.var_isbs2_btm_dn4 = assign96520_e149123_d_n4;
        locals.var_isbs2_btm_dn5 = assign96520_e149123_d_n5;
        locals.var_isbs2_btm_dn6 = assign96520_e149123_d_n6;
        locals.var_isbs2_btm_dn7 = assign96520_e149123_d_n7;
        locals.var_isbs2_btm_dn8 = assign96520_e149123_d_n8;
        locals.var_isbs2_btm_dn9 = assign96520_e149123_d_n9;
        locals.var_isbs2_btm_dn10 = assign96520_e149123_d_n10;
        locals.var_isbs2_btm_dn11 = assign96520_e149123_d_n11;
        locals.var_isbs2_btm_dn14 = assign96520_e149123_d_n14;

        let (assign96530_e149135, assign96530_e149135_d_n0, assign96530_e149135_d_n2, assign96530_e149135_d_n4, assign96530_e149135_d_n5, assign96530_e149135_d_n6, assign96530_e149135_d_n7, assign96530_e149135_d_n8, assign96530_e149135_d_n9, assign96530_e149135_d_n10, assign96530_e149135_d_n11, assign96530_e149135_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96530_e149131: f64 = (p.p16 - locals.var_weff_nf);
        let assign96530_e149133: f64 = (assign96530_e149131 * locals.var_jssw);
        (assign96530_e149133, (assign96530_e149131 * locals.var_jssw_dn0), (assign96530_e149131 * locals.var_jssw_dn2), (assign96530_e149131 * locals.var_jssw_dn4), (assign96530_e149131 * locals.var_jssw_dn5), (assign96530_e149131 * locals.var_jssw_dn6), (assign96530_e149131 * locals.var_jssw_dn7), (assign96530_e149131 * locals.var_jssw_dn8), (assign96530_e149131 * locals.var_jssw_dn9), (assign96530_e149131 * locals.var_jssw_dn10), (assign96530_e149131 * locals.var_jssw_dn11), (assign96530_e149131 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96530_e149135;
        locals.var_isbs_sws_dn0 = assign96530_e149135_d_n0;
        locals.var_isbs_sws_dn2 = assign96530_e149135_d_n2;
        locals.var_isbs_sws_dn4 = assign96530_e149135_d_n4;
        locals.var_isbs_sws_dn5 = assign96530_e149135_d_n5;
        locals.var_isbs_sws_dn6 = assign96530_e149135_d_n6;
        locals.var_isbs_sws_dn7 = assign96530_e149135_d_n7;
        locals.var_isbs_sws_dn8 = assign96530_e149135_d_n8;
        locals.var_isbs_sws_dn9 = assign96530_e149135_d_n9;
        locals.var_isbs_sws_dn10 = assign96530_e149135_d_n10;
        locals.var_isbs_sws_dn11 = assign96530_e149135_d_n11;
        locals.var_isbs_sws_dn14 = assign96530_e149135_d_n14;

        let (assign96540_e149147, assign96540_e149147_d_n0, assign96540_e149147_d_n2, assign96540_e149147_d_n4, assign96540_e149147_d_n5, assign96540_e149147_d_n6, assign96540_e149147_d_n7, assign96540_e149147_d_n8, assign96540_e149147_d_n9, assign96540_e149147_d_n10, assign96540_e149147_d_n11, assign96540_e149147_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96540_e149143: f64 = (p.p16 - locals.var_weff_nf);
        let assign96540_e149145: f64 = (assign96540_e149143 * locals.var_jssw2);
        (assign96540_e149145, (assign96540_e149143 * locals.var_jssw2_dn0), (assign96540_e149143 * locals.var_jssw2_dn2), (assign96540_e149143 * locals.var_jssw2_dn4), (assign96540_e149143 * locals.var_jssw2_dn5), (assign96540_e149143 * locals.var_jssw2_dn6), (assign96540_e149143 * locals.var_jssw2_dn7), (assign96540_e149143 * locals.var_jssw2_dn8), (assign96540_e149143 * locals.var_jssw2_dn9), (assign96540_e149143 * locals.var_jssw2_dn10), (assign96540_e149143 * locals.var_jssw2_dn11), (assign96540_e149143 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96540_e149147;
        locals.var_isbs2_sws_dn0 = assign96540_e149147_d_n0;
        locals.var_isbs2_sws_dn2 = assign96540_e149147_d_n2;
        locals.var_isbs2_sws_dn4 = assign96540_e149147_d_n4;
        locals.var_isbs2_sws_dn5 = assign96540_e149147_d_n5;
        locals.var_isbs2_sws_dn6 = assign96540_e149147_d_n6;
        locals.var_isbs2_sws_dn7 = assign96540_e149147_d_n7;
        locals.var_isbs2_sws_dn8 = assign96540_e149147_d_n8;
        locals.var_isbs2_sws_dn9 = assign96540_e149147_d_n9;
        locals.var_isbs2_sws_dn10 = assign96540_e149147_d_n10;
        locals.var_isbs2_sws_dn11 = assign96540_e149147_d_n11;
        locals.var_isbs2_sws_dn14 = assign96540_e149147_d_n14;

        let (assign96550_e149157, assign96550_e149157_d_n0, assign96550_e149157_d_n2, assign96550_e149157_d_n4, assign96550_e149157_d_n5, assign96550_e149157_d_n6, assign96550_e149157_d_n7, assign96550_e149157_d_n8, assign96550_e149157_d_n9, assign96550_e149157_d_n10, assign96550_e149157_d_n11, assign96550_e149157_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96550_e149155: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96550_e149155, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96550_e149157;
        locals.var_isbs_swg_dn0 = assign96550_e149157_d_n0;
        locals.var_isbs_swg_dn2 = assign96550_e149157_d_n2;
        locals.var_isbs_swg_dn4 = assign96550_e149157_d_n4;
        locals.var_isbs_swg_dn5 = assign96550_e149157_d_n5;
        locals.var_isbs_swg_dn6 = assign96550_e149157_d_n6;
        locals.var_isbs_swg_dn7 = assign96550_e149157_d_n7;
        locals.var_isbs_swg_dn8 = assign96550_e149157_d_n8;
        locals.var_isbs_swg_dn9 = assign96550_e149157_d_n9;
        locals.var_isbs_swg_dn10 = assign96550_e149157_d_n10;
        locals.var_isbs_swg_dn11 = assign96550_e149157_d_n11;
        locals.var_isbs_swg_dn14 = assign96550_e149157_d_n14;

        let (assign96560_e149167, assign96560_e149167_d_n0, assign96560_e149167_d_n2, assign96560_e149167_d_n4, assign96560_e149167_d_n5, assign96560_e149167_d_n6, assign96560_e149167_d_n7, assign96560_e149167_d_n8, assign96560_e149167_d_n9, assign96560_e149167_d_n10, assign96560_e149167_d_n11, assign96560_e149167_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96560_e149165: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96560_e149165, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96560_e149167;
        locals.var_isbs2_swg_dn0 = assign96560_e149167_d_n0;
        locals.var_isbs2_swg_dn2 = assign96560_e149167_d_n2;
        locals.var_isbs2_swg_dn4 = assign96560_e149167_d_n4;
        locals.var_isbs2_swg_dn5 = assign96560_e149167_d_n5;
        locals.var_isbs2_swg_dn6 = assign96560_e149167_d_n6;
        locals.var_isbs2_swg_dn7 = assign96560_e149167_d_n7;
        locals.var_isbs2_swg_dn8 = assign96560_e149167_d_n8;
        locals.var_isbs2_swg_dn9 = assign96560_e149167_d_n9;
        locals.var_isbs2_swg_dn10 = assign96560_e149167_d_n10;
        locals.var_isbs2_swg_dn11 = assign96560_e149167_d_n11;
        locals.var_isbs2_swg_dn14 = assign96560_e149167_d_n14;

        let (assign96570_e149178, assign96570_e149178_d_n0, assign96570_e149178_d_n2, assign96570_e149178_d_n4, assign96570_e149178_d_n5, assign96570_e149178_d_n6, assign96570_e149178_d_n7, assign96570_e149178_d_n8, assign96570_e149178_d_n9, assign96570_e149178_d_n10, assign96570_e149178_d_n11, assign96570_e149178_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        let assign96570_e149176: f64 = (p.p14 * locals.var_js);
        (assign96570_e149176, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96570_e149178;
        locals.var_isbs_btm_dn0 = assign96570_e149178_d_n0;
        locals.var_isbs_btm_dn2 = assign96570_e149178_d_n2;
        locals.var_isbs_btm_dn4 = assign96570_e149178_d_n4;
        locals.var_isbs_btm_dn5 = assign96570_e149178_d_n5;
        locals.var_isbs_btm_dn6 = assign96570_e149178_d_n6;
        locals.var_isbs_btm_dn7 = assign96570_e149178_d_n7;
        locals.var_isbs_btm_dn8 = assign96570_e149178_d_n8;
        locals.var_isbs_btm_dn9 = assign96570_e149178_d_n9;
        locals.var_isbs_btm_dn10 = assign96570_e149178_d_n10;
        locals.var_isbs_btm_dn11 = assign96570_e149178_d_n11;
        locals.var_isbs_btm_dn14 = assign96570_e149178_d_n14;

        let (assign96580_e149189, assign96580_e149189_d_n0, assign96580_e149189_d_n2, assign96580_e149189_d_n4, assign96580_e149189_d_n5, assign96580_e149189_d_n6, assign96580_e149189_d_n7, assign96580_e149189_d_n8, assign96580_e149189_d_n9, assign96580_e149189_d_n10, assign96580_e149189_d_n11, assign96580_e149189_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        let assign96580_e149187: f64 = (p.p14 * locals.var_js2);
        (assign96580_e149187, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96580_e149189;
        locals.var_isbs2_btm_dn0 = assign96580_e149189_d_n0;
        locals.var_isbs2_btm_dn2 = assign96580_e149189_d_n2;
        locals.var_isbs2_btm_dn4 = assign96580_e149189_d_n4;
        locals.var_isbs2_btm_dn5 = assign96580_e149189_d_n5;
        locals.var_isbs2_btm_dn6 = assign96580_e149189_d_n6;
        locals.var_isbs2_btm_dn7 = assign96580_e149189_d_n7;
        locals.var_isbs2_btm_dn8 = assign96580_e149189_d_n8;
        locals.var_isbs2_btm_dn9 = assign96580_e149189_d_n9;
        locals.var_isbs2_btm_dn10 = assign96580_e149189_d_n10;
        locals.var_isbs2_btm_dn11 = assign96580_e149189_d_n11;
        locals.var_isbs2_btm_dn14 = assign96580_e149189_d_n14;

        let (assign96590_e149198, assign96590_e149198_d_n0, assign96590_e149198_d_n2, assign96590_e149198_d_n4, assign96590_e149198_d_n5, assign96590_e149198_d_n6, assign96590_e149198_d_n7, assign96590_e149198_d_n8, assign96590_e149198_d_n9, assign96590_e149198_d_n10, assign96590_e149198_d_n11, assign96590_e149198_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96590_e149198;
        locals.var_isbs_sws_dn0 = assign96590_e149198_d_n0;
        locals.var_isbs_sws_dn2 = assign96590_e149198_d_n2;
        locals.var_isbs_sws_dn4 = assign96590_e149198_d_n4;
        locals.var_isbs_sws_dn5 = assign96590_e149198_d_n5;
        locals.var_isbs_sws_dn6 = assign96590_e149198_d_n6;
        locals.var_isbs_sws_dn7 = assign96590_e149198_d_n7;
        locals.var_isbs_sws_dn8 = assign96590_e149198_d_n8;
        locals.var_isbs_sws_dn9 = assign96590_e149198_d_n9;
        locals.var_isbs_sws_dn10 = assign96590_e149198_d_n10;
        locals.var_isbs_sws_dn11 = assign96590_e149198_d_n11;
        locals.var_isbs_sws_dn14 = assign96590_e149198_d_n14;

        let (assign96600_e149207, assign96600_e149207_d_n0, assign96600_e149207_d_n2, assign96600_e149207_d_n4, assign96600_e149207_d_n5, assign96600_e149207_d_n6, assign96600_e149207_d_n7, assign96600_e149207_d_n8, assign96600_e149207_d_n9, assign96600_e149207_d_n10, assign96600_e149207_d_n11, assign96600_e149207_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96600_e149207;
        locals.var_isbs2_sws_dn0 = assign96600_e149207_d_n0;
        locals.var_isbs2_sws_dn2 = assign96600_e149207_d_n2;
        locals.var_isbs2_sws_dn4 = assign96600_e149207_d_n4;
        locals.var_isbs2_sws_dn5 = assign96600_e149207_d_n5;
        locals.var_isbs2_sws_dn6 = assign96600_e149207_d_n6;
        locals.var_isbs2_sws_dn7 = assign96600_e149207_d_n7;
        locals.var_isbs2_sws_dn8 = assign96600_e149207_d_n8;
        locals.var_isbs2_sws_dn9 = assign96600_e149207_d_n9;
        locals.var_isbs2_sws_dn10 = assign96600_e149207_d_n10;
        locals.var_isbs2_sws_dn11 = assign96600_e149207_d_n11;
        locals.var_isbs2_sws_dn14 = assign96600_e149207_d_n14;

        let (assign96610_e149218, assign96610_e149218_d_n0, assign96610_e149218_d_n2, assign96610_e149218_d_n4, assign96610_e149218_d_n5, assign96610_e149218_d_n6, assign96610_e149218_d_n7, assign96610_e149218_d_n8, assign96610_e149218_d_n9, assign96610_e149218_d_n10, assign96610_e149218_d_n11, assign96610_e149218_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        let assign96610_e149216: f64 = (p.p16 * locals.var_jsswg);
        (assign96610_e149216, (p.p16 * locals.var_jsswg_dn0), (p.p16 * locals.var_jsswg_dn2), (p.p16 * locals.var_jsswg_dn4), (p.p16 * locals.var_jsswg_dn5), (p.p16 * locals.var_jsswg_dn6), (p.p16 * locals.var_jsswg_dn7), (p.p16 * locals.var_jsswg_dn8), (p.p16 * locals.var_jsswg_dn9), (p.p16 * locals.var_jsswg_dn10), (p.p16 * locals.var_jsswg_dn11), (p.p16 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96610_e149218;
        locals.var_isbs_swg_dn0 = assign96610_e149218_d_n0;
        locals.var_isbs_swg_dn2 = assign96610_e149218_d_n2;
        locals.var_isbs_swg_dn4 = assign96610_e149218_d_n4;
        locals.var_isbs_swg_dn5 = assign96610_e149218_d_n5;
        locals.var_isbs_swg_dn6 = assign96610_e149218_d_n6;
        locals.var_isbs_swg_dn7 = assign96610_e149218_d_n7;
        locals.var_isbs_swg_dn8 = assign96610_e149218_d_n8;
        locals.var_isbs_swg_dn9 = assign96610_e149218_d_n9;
        locals.var_isbs_swg_dn10 = assign96610_e149218_d_n10;
        locals.var_isbs_swg_dn11 = assign96610_e149218_d_n11;
        locals.var_isbs_swg_dn14 = assign96610_e149218_d_n14;

        let (assign96620_e149229, assign96620_e149229_d_n0, assign96620_e149229_d_n2, assign96620_e149229_d_n4, assign96620_e149229_d_n5, assign96620_e149229_d_n6, assign96620_e149229_d_n7, assign96620_e149229_d_n8, assign96620_e149229_d_n9, assign96620_e149229_d_n10, assign96620_e149229_d_n11, assign96620_e149229_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        let assign96620_e149227: f64 = (p.p16 * locals.var_jsswg2);
        (assign96620_e149227, (p.p16 * locals.var_jsswg2_dn0), (p.p16 * locals.var_jsswg2_dn2), (p.p16 * locals.var_jsswg2_dn4), (p.p16 * locals.var_jsswg2_dn5), (p.p16 * locals.var_jsswg2_dn6), (p.p16 * locals.var_jsswg2_dn7), (p.p16 * locals.var_jsswg2_dn8), (p.p16 * locals.var_jsswg2_dn9), (p.p16 * locals.var_jsswg2_dn10), (p.p16 * locals.var_jsswg2_dn11), (p.p16 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96620_e149229;
        locals.var_isbs2_swg_dn0 = assign96620_e149229_d_n0;
        locals.var_isbs2_swg_dn2 = assign96620_e149229_d_n2;
        locals.var_isbs2_swg_dn4 = assign96620_e149229_d_n4;
        locals.var_isbs2_swg_dn5 = assign96620_e149229_d_n5;
        locals.var_isbs2_swg_dn6 = assign96620_e149229_d_n6;
        locals.var_isbs2_swg_dn7 = assign96620_e149229_d_n7;
        locals.var_isbs2_swg_dn8 = assign96620_e149229_d_n8;
        locals.var_isbs2_swg_dn9 = assign96620_e149229_d_n9;
        locals.var_isbs2_swg_dn10 = assign96620_e149229_d_n10;
        locals.var_isbs2_swg_dn11 = assign96620_e149229_d_n11;
        locals.var_isbs2_swg_dn14 = assign96620_e149229_d_n14;

        let (assign96630_e149238, assign96630_e149238_d_n0, assign96630_e149238_d_n2, assign96630_e149238_d_n4, assign96630_e149238_d_n5, assign96630_e149238_d_n6, assign96630_e149238_d_n7, assign96630_e149238_d_n8, assign96630_e149238_d_n9, assign96630_e149238_d_n10, assign96630_e149238_d_n11, assign96630_e149238_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        let assign96630_e149236: f64 = (p.p14 * locals.var_js);
        (assign96630_e149236, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96630_e149238;
        locals.var_isbs_btm_dn0 = assign96630_e149238_d_n0;
        locals.var_isbs_btm_dn2 = assign96630_e149238_d_n2;
        locals.var_isbs_btm_dn4 = assign96630_e149238_d_n4;
        locals.var_isbs_btm_dn5 = assign96630_e149238_d_n5;
        locals.var_isbs_btm_dn6 = assign96630_e149238_d_n6;
        locals.var_isbs_btm_dn7 = assign96630_e149238_d_n7;
        locals.var_isbs_btm_dn8 = assign96630_e149238_d_n8;
        locals.var_isbs_btm_dn9 = assign96630_e149238_d_n9;
        locals.var_isbs_btm_dn10 = assign96630_e149238_d_n10;
        locals.var_isbs_btm_dn11 = assign96630_e149238_d_n11;
        locals.var_isbs_btm_dn14 = assign96630_e149238_d_n14;

        let (assign96640_e149247, assign96640_e149247_d_n0, assign96640_e149247_d_n2, assign96640_e149247_d_n4, assign96640_e149247_d_n5, assign96640_e149247_d_n6, assign96640_e149247_d_n7, assign96640_e149247_d_n8, assign96640_e149247_d_n9, assign96640_e149247_d_n10, assign96640_e149247_d_n11, assign96640_e149247_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        let assign96640_e149245: f64 = (p.p14 * locals.var_js2);
        (assign96640_e149245, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96640_e149247;
        locals.var_isbs2_btm_dn0 = assign96640_e149247_d_n0;
        locals.var_isbs2_btm_dn2 = assign96640_e149247_d_n2;
        locals.var_isbs2_btm_dn4 = assign96640_e149247_d_n4;
        locals.var_isbs2_btm_dn5 = assign96640_e149247_d_n5;
        locals.var_isbs2_btm_dn6 = assign96640_e149247_d_n6;
        locals.var_isbs2_btm_dn7 = assign96640_e149247_d_n7;
        locals.var_isbs2_btm_dn8 = assign96640_e149247_d_n8;
        locals.var_isbs2_btm_dn9 = assign96640_e149247_d_n9;
        locals.var_isbs2_btm_dn10 = assign96640_e149247_d_n10;
        locals.var_isbs2_btm_dn11 = assign96640_e149247_d_n11;
        locals.var_isbs2_btm_dn14 = assign96640_e149247_d_n14;

        let (assign96650_e149256, assign96650_e149256_d_n0, assign96650_e149256_d_n2, assign96650_e149256_d_n4, assign96650_e149256_d_n5, assign96650_e149256_d_n6, assign96650_e149256_d_n7, assign96650_e149256_d_n8, assign96650_e149256_d_n9, assign96650_e149256_d_n10, assign96650_e149256_d_n11, assign96650_e149256_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        let assign96650_e149254: f64 = (p.p16 * locals.var_jssw);
        (assign96650_e149254, (p.p16 * locals.var_jssw_dn0), (p.p16 * locals.var_jssw_dn2), (p.p16 * locals.var_jssw_dn4), (p.p16 * locals.var_jssw_dn5), (p.p16 * locals.var_jssw_dn6), (p.p16 * locals.var_jssw_dn7), (p.p16 * locals.var_jssw_dn8), (p.p16 * locals.var_jssw_dn9), (p.p16 * locals.var_jssw_dn10), (p.p16 * locals.var_jssw_dn11), (p.p16 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96650_e149256;
        locals.var_isbs_sws_dn0 = assign96650_e149256_d_n0;
        locals.var_isbs_sws_dn2 = assign96650_e149256_d_n2;
        locals.var_isbs_sws_dn4 = assign96650_e149256_d_n4;
        locals.var_isbs_sws_dn5 = assign96650_e149256_d_n5;
        locals.var_isbs_sws_dn6 = assign96650_e149256_d_n6;
        locals.var_isbs_sws_dn7 = assign96650_e149256_d_n7;
        locals.var_isbs_sws_dn8 = assign96650_e149256_d_n8;
        locals.var_isbs_sws_dn9 = assign96650_e149256_d_n9;
        locals.var_isbs_sws_dn10 = assign96650_e149256_d_n10;
        locals.var_isbs_sws_dn11 = assign96650_e149256_d_n11;
        locals.var_isbs_sws_dn14 = assign96650_e149256_d_n14;

        let (assign96660_e149265, assign96660_e149265_d_n0, assign96660_e149265_d_n2, assign96660_e149265_d_n4, assign96660_e149265_d_n5, assign96660_e149265_d_n6, assign96660_e149265_d_n7, assign96660_e149265_d_n8, assign96660_e149265_d_n9, assign96660_e149265_d_n10, assign96660_e149265_d_n11, assign96660_e149265_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        let assign96660_e149263: f64 = (p.p16 * locals.var_jssw2);
        (assign96660_e149263, (p.p16 * locals.var_jssw2_dn0), (p.p16 * locals.var_jssw2_dn2), (p.p16 * locals.var_jssw2_dn4), (p.p16 * locals.var_jssw2_dn5), (p.p16 * locals.var_jssw2_dn6), (p.p16 * locals.var_jssw2_dn7), (p.p16 * locals.var_jssw2_dn8), (p.p16 * locals.var_jssw2_dn9), (p.p16 * locals.var_jssw2_dn10), (p.p16 * locals.var_jssw2_dn11), (p.p16 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96660_e149265;
        locals.var_isbs2_sws_dn0 = assign96660_e149265_d_n0;
        locals.var_isbs2_sws_dn2 = assign96660_e149265_d_n2;
        locals.var_isbs2_sws_dn4 = assign96660_e149265_d_n4;
        locals.var_isbs2_sws_dn5 = assign96660_e149265_d_n5;
        locals.var_isbs2_sws_dn6 = assign96660_e149265_d_n6;
        locals.var_isbs2_sws_dn7 = assign96660_e149265_d_n7;
        locals.var_isbs2_sws_dn8 = assign96660_e149265_d_n8;
        locals.var_isbs2_sws_dn9 = assign96660_e149265_d_n9;
        locals.var_isbs2_sws_dn10 = assign96660_e149265_d_n10;
        locals.var_isbs2_sws_dn11 = assign96660_e149265_d_n11;
        locals.var_isbs2_sws_dn14 = assign96660_e149265_d_n14;

        let (assign96670_e149272, assign96670_e149272_d_n0, assign96670_e149272_d_n2, assign96670_e149272_d_n4, assign96670_e149272_d_n5, assign96670_e149272_d_n6, assign96670_e149272_d_n7, assign96670_e149272_d_n8, assign96670_e149272_d_n9, assign96670_e149272_d_n10, assign96670_e149272_d_n11, assign96670_e149272_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96670_e149272;
        locals.var_isbs_swg_dn0 = assign96670_e149272_d_n0;
        locals.var_isbs_swg_dn2 = assign96670_e149272_d_n2;
        locals.var_isbs_swg_dn4 = assign96670_e149272_d_n4;
        locals.var_isbs_swg_dn5 = assign96670_e149272_d_n5;
        locals.var_isbs_swg_dn6 = assign96670_e149272_d_n6;
        locals.var_isbs_swg_dn7 = assign96670_e149272_d_n7;
        locals.var_isbs_swg_dn8 = assign96670_e149272_d_n8;
        locals.var_isbs_swg_dn9 = assign96670_e149272_d_n9;
        locals.var_isbs_swg_dn10 = assign96670_e149272_d_n10;
        locals.var_isbs_swg_dn11 = assign96670_e149272_d_n11;
        locals.var_isbs_swg_dn14 = assign96670_e149272_d_n14;

        let (assign96680_e149279, assign96680_e149279_d_n0, assign96680_e149279_d_n2, assign96680_e149279_d_n4, assign96680_e149279_d_n5, assign96680_e149279_d_n6, assign96680_e149279_d_n7, assign96680_e149279_d_n8, assign96680_e149279_d_n9, assign96680_e149279_d_n10, assign96680_e149279_d_n11, assign96680_e149279_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96680_e149279;
        locals.var_isbs2_swg_dn0 = assign96680_e149279_d_n0;
        locals.var_isbs2_swg_dn2 = assign96680_e149279_d_n2;
        locals.var_isbs2_swg_dn4 = assign96680_e149279_d_n4;
        locals.var_isbs2_swg_dn5 = assign96680_e149279_d_n5;
        locals.var_isbs2_swg_dn6 = assign96680_e149279_d_n6;
        locals.var_isbs2_swg_dn7 = assign96680_e149279_d_n7;
        locals.var_isbs2_swg_dn8 = assign96680_e149279_d_n8;
        locals.var_isbs2_swg_dn9 = assign96680_e149279_d_n9;
        locals.var_isbs2_swg_dn10 = assign96680_e149279_d_n10;
        locals.var_isbs2_swg_dn11 = assign96680_e149279_d_n11;
        locals.var_isbs2_swg_dn14 = assign96680_e149279_d_n14;

        let (assign96690_e149287, assign96690_e149287_d_n0, assign96690_e149287_d_n2, assign96690_e149287_d_n4, assign96690_e149287_d_n5, assign96690_e149287_d_n6, assign96690_e149287_d_n7, assign96690_e149287_d_n8, assign96690_e149287_d_n9, assign96690_e149287_d_n10, assign96690_e149287_d_n11, assign96690_e149287_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96690_e149283: f64 = (locals.var_isbs_btm + locals.var_isbs_sws);
        let assign96690_e149285: f64 = (assign96690_e149283 + locals.var_isbs_swg);
        (assign96690_e149285, ((locals.var_isbs_btm_dn0 + locals.var_isbs_sws_dn0) + locals.var_isbs_swg_dn0), ((locals.var_isbs_btm_dn2 + locals.var_isbs_sws_dn2) + locals.var_isbs_swg_dn2), ((locals.var_isbs_btm_dn4 + locals.var_isbs_sws_dn4) + locals.var_isbs_swg_dn4), ((locals.var_isbs_btm_dn5 + locals.var_isbs_sws_dn5) + locals.var_isbs_swg_dn5), ((locals.var_isbs_btm_dn6 + locals.var_isbs_sws_dn6) + locals.var_isbs_swg_dn6), ((locals.var_isbs_btm_dn7 + locals.var_isbs_sws_dn7) + locals.var_isbs_swg_dn7), ((locals.var_isbs_btm_dn8 + locals.var_isbs_sws_dn8) + locals.var_isbs_swg_dn8), ((locals.var_isbs_btm_dn9 + locals.var_isbs_sws_dn9) + locals.var_isbs_swg_dn9), ((locals.var_isbs_btm_dn10 + locals.var_isbs_sws_dn10) + locals.var_isbs_swg_dn10), ((locals.var_isbs_btm_dn11 + locals.var_isbs_sws_dn11) + locals.var_isbs_swg_dn11), ((locals.var_isbs_btm_dn14 + locals.var_isbs_sws_dn14) + locals.var_isbs_swg_dn14),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    }
};
        locals.var_isbs = assign96690_e149287;
        locals.var_isbs_dn0 = assign96690_e149287_d_n0;
        locals.var_isbs_dn2 = assign96690_e149287_d_n2;
        locals.var_isbs_dn4 = assign96690_e149287_d_n4;
        locals.var_isbs_dn5 = assign96690_e149287_d_n5;
        locals.var_isbs_dn6 = assign96690_e149287_d_n6;
        locals.var_isbs_dn7 = assign96690_e149287_d_n7;
        locals.var_isbs_dn8 = assign96690_e149287_d_n8;
        locals.var_isbs_dn9 = assign96690_e149287_d_n9;
        locals.var_isbs_dn10 = assign96690_e149287_d_n10;
        locals.var_isbs_dn11 = assign96690_e149287_d_n11;
        locals.var_isbs_dn14 = assign96690_e149287_d_n14;

        let assign96700_e149290: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2243 = assign96700_e149290;

        let (assign96710_e149298, assign96710_e149298_d_n0, assign96710_e149298_d_n2, assign96710_e149298_d_n4, assign96710_e149298_d_n5, assign96710_e149298_d_n6, assign96710_e149298_d_n7, assign96710_e149298_d_n8, assign96710_e149298_d_n9, assign96710_e149298_d_n10, assign96710_e149298_d_n11, assign96710_e149298_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2243 != 0.0)) {
        let assign96710_e149296: f64 = (locals.var_isbs + 1e-25);
        (assign96710_e149296, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign96710_e149298;
        locals.var_t3_dn0 = assign96710_e149298_d_n0;
        locals.var_t3_dn2 = assign96710_e149298_d_n2;
        locals.var_t3_dn4 = assign96710_e149298_d_n4;
        locals.var_t3_dn5 = assign96710_e149298_d_n5;
        locals.var_t3_dn6 = assign96710_e149298_d_n6;
        locals.var_t3_dn7 = assign96710_e149298_d_n7;
        locals.var_t3_dn8 = assign96710_e149298_d_n8;
        locals.var_t3_dn9 = assign96710_e149298_d_n9;
        locals.var_t3_dn10 = assign96710_e149298_d_n10;
        locals.var_t3_dn11 = assign96710_e149298_d_n11;
        locals.var_t3_dn14 = assign96710_e149298_d_n14;

        let (assign96720_e149315, assign96720_e149315_d_n0, assign96720_e149315_d_n2, assign96720_e149315_d_n4, assign96720_e149315_d_n5, assign96720_e149315_d_n6, assign96720_e149315_d_n7, assign96720_e149315_d_n8, assign96720_e149315_d_n9, assign96720_e149315_d_n10, assign96720_e149315_d_n11, assign96720_e149315_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2243 != 0.0)) {
        let assign96720_e149304: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96720_e149307: f64 = (locals.var_uc_vdiffjs * locals.var_t0);
        let assign96720_e149309: f64 = (assign96720_e149307 / locals.var_t3);
        let assign96720_e149311: f64 = (assign96720_e149309 + 1.0);
        let assign96720_e149312: f64 = (assign96720_e149311).ln();
        let assign96720_e149313: f64 = (assign96720_e149304 * assign96720_e149312);
        (assign96720_e149313, (((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn0) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn2) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn4) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn5) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn6) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn7) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn8) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn9) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn10) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn11) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn14) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))),)
    } else {
        (locals.var_vbst, locals.var_vbst_dn0, locals.var_vbst_dn2, locals.var_vbst_dn4, locals.var_vbst_dn5, locals.var_vbst_dn6, locals.var_vbst_dn7, locals.var_vbst_dn8, locals.var_vbst_dn9, locals.var_vbst_dn10, locals.var_vbst_dn11, locals.var_vbst_dn14,)
    }
};
        locals.var_vbst = assign96720_e149315;
        locals.var_vbst_dn0 = assign96720_e149315_d_n0;
        locals.var_vbst_dn2 = assign96720_e149315_d_n2;
        locals.var_vbst_dn4 = assign96720_e149315_d_n4;
        locals.var_vbst_dn5 = assign96720_e149315_d_n5;
        locals.var_vbst_dn6 = assign96720_e149315_d_n6;
        locals.var_vbst_dn7 = assign96720_e149315_d_n7;
        locals.var_vbst_dn8 = assign96720_e149315_d_n8;
        locals.var_vbst_dn9 = assign96720_e149315_d_n9;
        locals.var_vbst_dn10 = assign96720_e149315_d_n10;
        locals.var_vbst_dn11 = assign96720_e149315_d_n11;
        locals.var_vbst_dn14 = assign96720_e149315_d_n14;

        let (assign96730_e149326, assign96730_e149326_d_n0, assign96730_e149326_d_n2, assign96730_e149326_d_n4, assign96730_e149326_d_n5, assign96730_e149326_d_n6, assign96730_e149326_d_n7, assign96730_e149326_d_n8, assign96730_e149326_d_n9, assign96730_e149326_d_n10, assign96730_e149326_d_n11, assign96730_e149326_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2243 != 0.0)) {
        let assign96730_e149321: f64 = (locals.var_tratio - 1.0);
        let assign96730_e149323: f64 = (assign96730_e149321 * p.p535);
        let assign96730_e149324: f64 = (assign96730_e149323).exp();
        (assign96730_e149324, (assign96730_e149324 * (locals.var_tratio_dn0 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn2 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn4 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn5 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn6 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn7 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn8 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn9 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn10 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn11 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn14 * p.p535)),)
    } else {
        (locals.var_exptemps, locals.var_exptemps_dn0, locals.var_exptemps_dn2, locals.var_exptemps_dn4, locals.var_exptemps_dn5, locals.var_exptemps_dn6, locals.var_exptemps_dn7, locals.var_exptemps_dn8, locals.var_exptemps_dn9, locals.var_exptemps_dn10, locals.var_exptemps_dn11, locals.var_exptemps_dn14,)
    }
};
        locals.var_exptemps = assign96730_e149326;
        locals.var_exptemps_dn0 = assign96730_e149326_d_n0;
        locals.var_exptemps_dn2 = assign96730_e149326_d_n2;
        locals.var_exptemps_dn4 = assign96730_e149326_d_n4;
        locals.var_exptemps_dn5 = assign96730_e149326_d_n5;
        locals.var_exptemps_dn6 = assign96730_e149326_d_n6;
        locals.var_exptemps_dn7 = assign96730_e149326_d_n7;
        locals.var_exptemps_dn8 = assign96730_e149326_d_n8;
        locals.var_exptemps_dn9 = assign96730_e149326_d_n9;
        locals.var_exptemps_dn10 = assign96730_e149326_d_n10;
        locals.var_exptemps_dn11 = assign96730_e149326_d_n11;
        locals.var_exptemps_dn14 = assign96730_e149326_d_n14;

        let (assign96740_e149336, assign96740_e149336_d_n0, assign96740_e149336_d_n2, assign96740_e149336_d_n4, assign96740_e149336_d_n5, assign96740_e149336_d_n6, assign96740_e149336_d_n7, assign96740_e149336_d_n8, assign96740_e149336_d_n9, assign96740_e149336_d_n10, assign96740_e149336_d_n11, assign96740_e149336_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2243 != 0.0)) {
        let assign96740_e149333: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96740_e149334: f64 = (1.0 / assign96740_e149333);
        (assign96740_e149334, (-((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))),)
    } else {
        (locals.var_jd_nvtm_invs, locals.var_jd_nvtm_invs_dn0, locals.var_jd_nvtm_invs_dn2, locals.var_jd_nvtm_invs_dn4, locals.var_jd_nvtm_invs_dn5, locals.var_jd_nvtm_invs_dn6, locals.var_jd_nvtm_invs_dn7, locals.var_jd_nvtm_invs_dn8, locals.var_jd_nvtm_invs_dn9, locals.var_jd_nvtm_invs_dn10, locals.var_jd_nvtm_invs_dn11, locals.var_jd_nvtm_invs_dn14,)
    }
};
        locals.var_jd_nvtm_invs = assign96740_e149336;
        locals.var_jd_nvtm_invs_dn0 = assign96740_e149336_d_n0;
        locals.var_jd_nvtm_invs_dn2 = assign96740_e149336_d_n2;
        locals.var_jd_nvtm_invs_dn4 = assign96740_e149336_d_n4;
        locals.var_jd_nvtm_invs_dn5 = assign96740_e149336_d_n5;
        locals.var_jd_nvtm_invs_dn6 = assign96740_e149336_d_n6;
        locals.var_jd_nvtm_invs_dn7 = assign96740_e149336_d_n7;
        locals.var_jd_nvtm_invs_dn8 = assign96740_e149336_d_n8;
        locals.var_jd_nvtm_invs_dn9 = assign96740_e149336_d_n9;
        locals.var_jd_nvtm_invs_dn10 = assign96740_e149336_d_n10;
        locals.var_jd_nvtm_invs_dn11 = assign96740_e149336_d_n11;
        locals.var_jd_nvtm_invs_dn14 = assign96740_e149336_d_n14;

    }

    pub(super) fn stamp_transient_block_357(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96750_e149345, assign96750_e149345_d_n0, assign96750_e149345_d_n2, assign96750_e149345_d_n4, assign96750_e149345_d_n5, assign96750_e149345_d_n6, assign96750_e149345_d_n7, assign96750_e149345_d_n8, assign96750_e149345_d_n9, assign96750_e149345_d_n10, assign96750_e149345_d_n11, assign96750_e149345_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2243 != 0.0)) {
        let assign96750_e149342: f64 = (locals.var_vbst * locals.var_jd_nvtm_invs);
        let assign96750_e149343: f64 = (assign96750_e149342).exp();
        (assign96750_e149343, (assign96750_e149343 * ((locals.var_vbst_dn0 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn0))), (assign96750_e149343 * ((locals.var_vbst_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn2))), (assign96750_e149343 * ((locals.var_vbst_dn4 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn4))), (assign96750_e149343 * ((locals.var_vbst_dn5 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn5))), (assign96750_e149343 * ((locals.var_vbst_dn6 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn6))), (assign96750_e149343 * ((locals.var_vbst_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn7))), (assign96750_e149343 * ((locals.var_vbst_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn8))), (assign96750_e149343 * ((locals.var_vbst_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn9))), (assign96750_e149343 * ((locals.var_vbst_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn10))), (assign96750_e149343 * ((locals.var_vbst_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn11))), (assign96750_e149343 * ((locals.var_vbst_dn14 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn14))),)
    } else {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    }
};
        locals.var_jd_expcs = assign96750_e149345;
        locals.var_jd_expcs_dn0 = assign96750_e149345_d_n0;
        locals.var_jd_expcs_dn2 = assign96750_e149345_d_n2;
        locals.var_jd_expcs_dn4 = assign96750_e149345_d_n4;
        locals.var_jd_expcs_dn5 = assign96750_e149345_d_n5;
        locals.var_jd_expcs_dn6 = assign96750_e149345_d_n6;
        locals.var_jd_expcs_dn7 = assign96750_e149345_d_n7;
        locals.var_jd_expcs_dn8 = assign96750_e149345_d_n8;
        locals.var_jd_expcs_dn9 = assign96750_e149345_d_n9;
        locals.var_jd_expcs_dn10 = assign96750_e149345_d_n10;
        locals.var_jd_expcs_dn11 = assign96750_e149345_d_n11;
        locals.var_jd_expcs_dn14 = assign96750_e149345_d_n14;

        let (assign96760_e149357, assign96760_e149357_d_n0, assign96760_e149357_d_n2, assign96760_e149357_d_n4, assign96760_e149357_d_n5, assign96760_e149357_d_n6, assign96760_e149357_d_n7, assign96760_e149357_d_n8, assign96760_e149357_d_n9, assign96760_e149357_d_n10, assign96760_e149357_d_n11, assign96760_e149357_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96760_e149349: f64 = (p.p500 * p.p13);
        let assign96760_e149353: f64 = (p.p481 * locals.var_tdiff);
        let assign96760_e149354: f64 = (1.0 + assign96760_e149353);
        let assign96760_e149355: f64 = (assign96760_e149349 * assign96760_e149354);
        (assign96760_e149355, (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn0)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn2)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn4)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn5)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn6)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn7)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn8)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn9)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn10)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn11)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign96760_e149357;
        locals.var_czbd_dn0 = assign96760_e149357_d_n0;
        locals.var_czbd_dn2 = assign96760_e149357_d_n2;
        locals.var_czbd_dn4 = assign96760_e149357_d_n4;
        locals.var_czbd_dn5 = assign96760_e149357_d_n5;
        locals.var_czbd_dn6 = assign96760_e149357_d_n6;
        locals.var_czbd_dn7 = assign96760_e149357_d_n7;
        locals.var_czbd_dn8 = assign96760_e149357_d_n8;
        locals.var_czbd_dn9 = assign96760_e149357_d_n9;
        locals.var_czbd_dn10 = assign96760_e149357_d_n10;
        locals.var_czbd_dn11 = assign96760_e149357_d_n11;
        locals.var_czbd_dn14 = assign96760_e149357_d_n14;

        let assign96770_e149360: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2244 = assign96770_e149360;

        let (assign96780_e149376, assign96780_e149376_d_n0, assign96780_e149376_d_n2, assign96780_e149376_d_n4, assign96780_e149376_d_n5, assign96780_e149376_d_n6, assign96780_e149376_d_n7, assign96780_e149376_d_n8, assign96780_e149376_d_n9, assign96780_e149376_d_n10, assign96780_e149376_d_n11, assign96780_e149376_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2244 != 0.0)) {
        let assign96780_e149367: f64 = (p.p15 - locals.var_weff_nf);
        let assign96780_e149368: f64 = (p.p501 * assign96780_e149367);
        let assign96780_e149372: f64 = (p.p483 * locals.var_tdiff);
        let assign96780_e149373: f64 = (1.0 + assign96780_e149372);
        let assign96780_e149374: f64 = (assign96780_e149368 * assign96780_e149373);
        (assign96780_e149374, (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn0)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn2)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn4)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn5)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn6)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn7)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn8)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn9)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn10)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn11)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96780_e149376;
        locals.var_czbdsw_dn0 = assign96780_e149376_d_n0;
        locals.var_czbdsw_dn2 = assign96780_e149376_d_n2;
        locals.var_czbdsw_dn4 = assign96780_e149376_d_n4;
        locals.var_czbdsw_dn5 = assign96780_e149376_d_n5;
        locals.var_czbdsw_dn6 = assign96780_e149376_d_n6;
        locals.var_czbdsw_dn7 = assign96780_e149376_d_n7;
        locals.var_czbdsw_dn8 = assign96780_e149376_d_n8;
        locals.var_czbdsw_dn9 = assign96780_e149376_d_n9;
        locals.var_czbdsw_dn10 = assign96780_e149376_d_n10;
        locals.var_czbdsw_dn11 = assign96780_e149376_d_n11;
        locals.var_czbdsw_dn14 = assign96780_e149376_d_n14;

        let (assign96790_e149390, assign96790_e149390_d_n0, assign96790_e149390_d_n2, assign96790_e149390_d_n4, assign96790_e149390_d_n5, assign96790_e149390_d_n6, assign96790_e149390_d_n7, assign96790_e149390_d_n8, assign96790_e149390_d_n9, assign96790_e149390_d_n10, assign96790_e149390_d_n11, assign96790_e149390_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2244 != 0.0)) {
        let assign96790_e149382: f64 = (p.p502 * locals.var_weff_nf);
        let assign96790_e149386: f64 = (p.p485 * locals.var_tdiff);
        let assign96790_e149387: f64 = (1.0 + assign96790_e149386);
        let assign96790_e149388: f64 = (assign96790_e149382 * assign96790_e149387);
        (assign96790_e149388, (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn0)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn2)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn4)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn5)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn6)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn7)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn8)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn9)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn10)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn11)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96790_e149390;
        locals.var_czbdswg_dn0 = assign96790_e149390_d_n0;
        locals.var_czbdswg_dn2 = assign96790_e149390_d_n2;
        locals.var_czbdswg_dn4 = assign96790_e149390_d_n4;
        locals.var_czbdswg_dn5 = assign96790_e149390_d_n5;
        locals.var_czbdswg_dn6 = assign96790_e149390_d_n6;
        locals.var_czbdswg_dn7 = assign96790_e149390_d_n7;
        locals.var_czbdswg_dn8 = assign96790_e149390_d_n8;
        locals.var_czbdswg_dn9 = assign96790_e149390_d_n9;
        locals.var_czbdswg_dn10 = assign96790_e149390_d_n10;
        locals.var_czbdswg_dn11 = assign96790_e149390_d_n11;
        locals.var_czbdswg_dn14 = assign96790_e149390_d_n14;

        let (assign96800_e149397, assign96800_e149397_d_n0, assign96800_e149397_d_n2, assign96800_e149397_d_n4, assign96800_e149397_d_n5, assign96800_e149397_d_n6, assign96800_e149397_d_n7, assign96800_e149397_d_n8, assign96800_e149397_d_n9, assign96800_e149397_d_n10, assign96800_e149397_d_n11, assign96800_e149397_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2244 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96800_e149397;
        locals.var_czbdsw_dn0 = assign96800_e149397_d_n0;
        locals.var_czbdsw_dn2 = assign96800_e149397_d_n2;
        locals.var_czbdsw_dn4 = assign96800_e149397_d_n4;
        locals.var_czbdsw_dn5 = assign96800_e149397_d_n5;
        locals.var_czbdsw_dn6 = assign96800_e149397_d_n6;
        locals.var_czbdsw_dn7 = assign96800_e149397_d_n7;
        locals.var_czbdsw_dn8 = assign96800_e149397_d_n8;
        locals.var_czbdsw_dn9 = assign96800_e149397_d_n9;
        locals.var_czbdsw_dn10 = assign96800_e149397_d_n10;
        locals.var_czbdsw_dn11 = assign96800_e149397_d_n11;
        locals.var_czbdsw_dn14 = assign96800_e149397_d_n14;

        let (assign96810_e149412, assign96810_e149412_d_n0, assign96810_e149412_d_n2, assign96810_e149412_d_n4, assign96810_e149412_d_n5, assign96810_e149412_d_n6, assign96810_e149412_d_n7, assign96810_e149412_d_n8, assign96810_e149412_d_n9, assign96810_e149412_d_n10, assign96810_e149412_d_n11, assign96810_e149412_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2244 == 0.0)) {
        let assign96810_e149404: f64 = (p.p502 * p.p15);
        let assign96810_e149408: f64 = (p.p485 * locals.var_tdiff);
        let assign96810_e149409: f64 = (1.0 + assign96810_e149408);
        let assign96810_e149410: f64 = (assign96810_e149404 * assign96810_e149409);
        (assign96810_e149410, (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn0)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn2)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn4)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn5)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn6)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn7)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn8)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn9)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn10)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn11)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96810_e149412;
        locals.var_czbdswg_dn0 = assign96810_e149412_d_n0;
        locals.var_czbdswg_dn2 = assign96810_e149412_d_n2;
        locals.var_czbdswg_dn4 = assign96810_e149412_d_n4;
        locals.var_czbdswg_dn5 = assign96810_e149412_d_n5;
        locals.var_czbdswg_dn6 = assign96810_e149412_d_n6;
        locals.var_czbdswg_dn7 = assign96810_e149412_d_n7;
        locals.var_czbdswg_dn8 = assign96810_e149412_d_n8;
        locals.var_czbdswg_dn9 = assign96810_e149412_d_n9;
        locals.var_czbdswg_dn10 = assign96810_e149412_d_n10;
        locals.var_czbdswg_dn11 = assign96810_e149412_d_n11;
        locals.var_czbdswg_dn14 = assign96810_e149412_d_n14;

        let assign96820_e149415: f64 = if locals.var_czbd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2245 = assign96820_e149415;

        let (assign96830_e149421, assign96830_e149421_d_n0, assign96830_e149421_d_n2, assign96830_e149421_d_n4, assign96830_e149421_d_n5, assign96830_e149421_d_n6, assign96830_e149421_d_n7, assign96830_e149421_d_n8, assign96830_e149421_d_n9, assign96830_e149421_d_n10, assign96830_e149421_d_n11, assign96830_e149421_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2245 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign96830_e149421;
        locals.var_czbd_dn0 = assign96830_e149421_d_n0;
        locals.var_czbd_dn2 = assign96830_e149421_d_n2;
        locals.var_czbd_dn4 = assign96830_e149421_d_n4;
        locals.var_czbd_dn5 = assign96830_e149421_d_n5;
        locals.var_czbd_dn6 = assign96830_e149421_d_n6;
        locals.var_czbd_dn7 = assign96830_e149421_d_n7;
        locals.var_czbd_dn8 = assign96830_e149421_d_n8;
        locals.var_czbd_dn9 = assign96830_e149421_d_n9;
        locals.var_czbd_dn10 = assign96830_e149421_d_n10;
        locals.var_czbd_dn11 = assign96830_e149421_d_n11;
        locals.var_czbd_dn14 = assign96830_e149421_d_n14;

        let assign96840_e149424: f64 = if locals.var_czbdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2246 = assign96840_e149424;

        let (assign96850_e149430, assign96850_e149430_d_n0, assign96850_e149430_d_n2, assign96850_e149430_d_n4, assign96850_e149430_d_n5, assign96850_e149430_d_n6, assign96850_e149430_d_n7, assign96850_e149430_d_n8, assign96850_e149430_d_n9, assign96850_e149430_d_n10, assign96850_e149430_d_n11, assign96850_e149430_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2246 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96850_e149430;
        locals.var_czbdsw_dn0 = assign96850_e149430_d_n0;
        locals.var_czbdsw_dn2 = assign96850_e149430_d_n2;
        locals.var_czbdsw_dn4 = assign96850_e149430_d_n4;
        locals.var_czbdsw_dn5 = assign96850_e149430_d_n5;
        locals.var_czbdsw_dn6 = assign96850_e149430_d_n6;
        locals.var_czbdsw_dn7 = assign96850_e149430_d_n7;
        locals.var_czbdsw_dn8 = assign96850_e149430_d_n8;
        locals.var_czbdsw_dn9 = assign96850_e149430_d_n9;
        locals.var_czbdsw_dn10 = assign96850_e149430_d_n10;
        locals.var_czbdsw_dn11 = assign96850_e149430_d_n11;
        locals.var_czbdsw_dn14 = assign96850_e149430_d_n14;

        let assign96860_e149433: f64 = if locals.var_czbdswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2247 = assign96860_e149433;

        let (assign96870_e149439, assign96870_e149439_d_n0, assign96870_e149439_d_n2, assign96870_e149439_d_n4, assign96870_e149439_d_n5, assign96870_e149439_d_n6, assign96870_e149439_d_n7, assign96870_e149439_d_n8, assign96870_e149439_d_n9, assign96870_e149439_d_n10, assign96870_e149439_d_n11, assign96870_e149439_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2247 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96870_e149439;
        locals.var_czbdswg_dn0 = assign96870_e149439_d_n0;
        locals.var_czbdswg_dn2 = assign96870_e149439_d_n2;
        locals.var_czbdswg_dn4 = assign96870_e149439_d_n4;
        locals.var_czbdswg_dn5 = assign96870_e149439_d_n5;
        locals.var_czbdswg_dn6 = assign96870_e149439_d_n6;
        locals.var_czbdswg_dn7 = assign96870_e149439_d_n7;
        locals.var_czbdswg_dn8 = assign96870_e149439_d_n8;
        locals.var_czbdswg_dn9 = assign96870_e149439_d_n9;
        locals.var_czbdswg_dn10 = assign96870_e149439_d_n10;
        locals.var_czbdswg_dn11 = assign96870_e149439_d_n11;
        locals.var_czbdswg_dn14 = assign96870_e149439_d_n14;

        let (assign96880_e149447, assign96880_e149447_d_n0, assign96880_e149447_d_n2, assign96880_e149447_d_n4, assign96880_e149447_d_n5, assign96880_e149447_d_n6, assign96880_e149447_d_n7, assign96880_e149447_d_n8, assign96880_e149447_d_n9, assign96880_e149447_d_n10, assign96880_e149447_d_n11, assign96880_e149447_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96880_e149444: f64 = (p.p487 * locals.var_tdiff);
        let assign96880_e149445: f64 = (p.p506 - assign96880_e149444);
        (assign96880_e149445, (-(p.p487 * locals.var_tdiff_dn0)), (-(p.p487 * locals.var_tdiff_dn2)), (-(p.p487 * locals.var_tdiff_dn4)), (-(p.p487 * locals.var_tdiff_dn5)), (-(p.p487 * locals.var_tdiff_dn6)), (-(p.p487 * locals.var_tdiff_dn7)), (-(p.p487 * locals.var_tdiff_dn8)), (-(p.p487 * locals.var_tdiff_dn9)), (-(p.p487 * locals.var_tdiff_dn10)), (-(p.p487 * locals.var_tdiff_dn11)), (-(p.p487 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign96880_e149447;
        locals.var_pzbd_dn0 = assign96880_e149447_d_n0;
        locals.var_pzbd_dn2 = assign96880_e149447_d_n2;
        locals.var_pzbd_dn4 = assign96880_e149447_d_n4;
        locals.var_pzbd_dn5 = assign96880_e149447_d_n5;
        locals.var_pzbd_dn6 = assign96880_e149447_d_n6;
        locals.var_pzbd_dn7 = assign96880_e149447_d_n7;
        locals.var_pzbd_dn8 = assign96880_e149447_d_n8;
        locals.var_pzbd_dn9 = assign96880_e149447_d_n9;
        locals.var_pzbd_dn10 = assign96880_e149447_d_n10;
        locals.var_pzbd_dn11 = assign96880_e149447_d_n11;
        locals.var_pzbd_dn14 = assign96880_e149447_d_n14;

        let (assign96890_e149455, assign96890_e149455_d_n0, assign96890_e149455_d_n2, assign96890_e149455_d_n4, assign96890_e149455_d_n5, assign96890_e149455_d_n6, assign96890_e149455_d_n7, assign96890_e149455_d_n8, assign96890_e149455_d_n9, assign96890_e149455_d_n10, assign96890_e149455_d_n11, assign96890_e149455_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96890_e149452: f64 = (p.p489 * locals.var_tdiff);
        let assign96890_e149453: f64 = (p.p507 - assign96890_e149452);
        (assign96890_e149453, (-(p.p489 * locals.var_tdiff_dn0)), (-(p.p489 * locals.var_tdiff_dn2)), (-(p.p489 * locals.var_tdiff_dn4)), (-(p.p489 * locals.var_tdiff_dn5)), (-(p.p489 * locals.var_tdiff_dn6)), (-(p.p489 * locals.var_tdiff_dn7)), (-(p.p489 * locals.var_tdiff_dn8)), (-(p.p489 * locals.var_tdiff_dn9)), (-(p.p489 * locals.var_tdiff_dn10)), (-(p.p489 * locals.var_tdiff_dn11)), (-(p.p489 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign96890_e149455;
        locals.var_pzbdsw_dn0 = assign96890_e149455_d_n0;
        locals.var_pzbdsw_dn2 = assign96890_e149455_d_n2;
        locals.var_pzbdsw_dn4 = assign96890_e149455_d_n4;
        locals.var_pzbdsw_dn5 = assign96890_e149455_d_n5;
        locals.var_pzbdsw_dn6 = assign96890_e149455_d_n6;
        locals.var_pzbdsw_dn7 = assign96890_e149455_d_n7;
        locals.var_pzbdsw_dn8 = assign96890_e149455_d_n8;
        locals.var_pzbdsw_dn9 = assign96890_e149455_d_n9;
        locals.var_pzbdsw_dn10 = assign96890_e149455_d_n10;
        locals.var_pzbdsw_dn11 = assign96890_e149455_d_n11;
        locals.var_pzbdsw_dn14 = assign96890_e149455_d_n14;

        let (assign96900_e149463, assign96900_e149463_d_n0, assign96900_e149463_d_n2, assign96900_e149463_d_n4, assign96900_e149463_d_n5, assign96900_e149463_d_n6, assign96900_e149463_d_n7, assign96900_e149463_d_n8, assign96900_e149463_d_n9, assign96900_e149463_d_n10, assign96900_e149463_d_n11, assign96900_e149463_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96900_e149460: f64 = (p.p491 * locals.var_tdiff);
        let assign96900_e149461: f64 = (p.p508 - assign96900_e149460);
        (assign96900_e149461, (-(p.p491 * locals.var_tdiff_dn0)), (-(p.p491 * locals.var_tdiff_dn2)), (-(p.p491 * locals.var_tdiff_dn4)), (-(p.p491 * locals.var_tdiff_dn5)), (-(p.p491 * locals.var_tdiff_dn6)), (-(p.p491 * locals.var_tdiff_dn7)), (-(p.p491 * locals.var_tdiff_dn8)), (-(p.p491 * locals.var_tdiff_dn9)), (-(p.p491 * locals.var_tdiff_dn10)), (-(p.p491 * locals.var_tdiff_dn11)), (-(p.p491 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign96900_e149463;
        locals.var_pzbdswg_dn0 = assign96900_e149463_d_n0;
        locals.var_pzbdswg_dn2 = assign96900_e149463_d_n2;
        locals.var_pzbdswg_dn4 = assign96900_e149463_d_n4;
        locals.var_pzbdswg_dn5 = assign96900_e149463_d_n5;
        locals.var_pzbdswg_dn6 = assign96900_e149463_d_n6;
        locals.var_pzbdswg_dn7 = assign96900_e149463_d_n7;
        locals.var_pzbdswg_dn8 = assign96900_e149463_d_n8;
        locals.var_pzbdswg_dn9 = assign96900_e149463_d_n9;
        locals.var_pzbdswg_dn10 = assign96900_e149463_d_n10;
        locals.var_pzbdswg_dn11 = assign96900_e149463_d_n11;
        locals.var_pzbdswg_dn14 = assign96900_e149463_d_n14;

        let assign96910_e149470: f64 = if ((locals.var_pzbd < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2248 = assign96910_e149470;

        let (assign96920_e149476, assign96920_e149476_d_n0, assign96920_e149476_d_n2, assign96920_e149476_d_n4, assign96920_e149476_d_n5, assign96920_e149476_d_n6, assign96920_e149476_d_n7, assign96920_e149476_d_n8, assign96920_e149476_d_n9, assign96920_e149476_d_n10, assign96920_e149476_d_n11, assign96920_e149476_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2248 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign96920_e149476;
        locals.var_pzbd_dn0 = assign96920_e149476_d_n0;
        locals.var_pzbd_dn2 = assign96920_e149476_d_n2;
        locals.var_pzbd_dn4 = assign96920_e149476_d_n4;
        locals.var_pzbd_dn5 = assign96920_e149476_d_n5;
        locals.var_pzbd_dn6 = assign96920_e149476_d_n6;
        locals.var_pzbd_dn7 = assign96920_e149476_d_n7;
        locals.var_pzbd_dn8 = assign96920_e149476_d_n8;
        locals.var_pzbd_dn9 = assign96920_e149476_d_n9;
        locals.var_pzbd_dn10 = assign96920_e149476_d_n10;
        locals.var_pzbd_dn11 = assign96920_e149476_d_n11;
        locals.var_pzbd_dn14 = assign96920_e149476_d_n14;

        let assign96930_e149483: f64 = if ((locals.var_pzbdsw < 0.01) && (p.p15 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2249 = assign96930_e149483;

        let (assign96940_e149489, assign96940_e149489_d_n0, assign96940_e149489_d_n2, assign96940_e149489_d_n4, assign96940_e149489_d_n5, assign96940_e149489_d_n6, assign96940_e149489_d_n7, assign96940_e149489_d_n8, assign96940_e149489_d_n9, assign96940_e149489_d_n10, assign96940_e149489_d_n11, assign96940_e149489_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2249 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign96940_e149489;
        locals.var_pzbdsw_dn0 = assign96940_e149489_d_n0;
        locals.var_pzbdsw_dn2 = assign96940_e149489_d_n2;
        locals.var_pzbdsw_dn4 = assign96940_e149489_d_n4;
        locals.var_pzbdsw_dn5 = assign96940_e149489_d_n5;
        locals.var_pzbdsw_dn6 = assign96940_e149489_d_n6;
        locals.var_pzbdsw_dn7 = assign96940_e149489_d_n7;
        locals.var_pzbdsw_dn8 = assign96940_e149489_d_n8;
        locals.var_pzbdsw_dn9 = assign96940_e149489_d_n9;
        locals.var_pzbdsw_dn10 = assign96940_e149489_d_n10;
        locals.var_pzbdsw_dn11 = assign96940_e149489_d_n11;
        locals.var_pzbdsw_dn14 = assign96940_e149489_d_n14;

        let assign96950_e149496: f64 = if ((locals.var_pzbdswg < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2250 = assign96950_e149496;

        let (assign96960_e149502, assign96960_e149502_d_n0, assign96960_e149502_d_n2, assign96960_e149502_d_n4, assign96960_e149502_d_n5, assign96960_e149502_d_n6, assign96960_e149502_d_n7, assign96960_e149502_d_n8, assign96960_e149502_d_n9, assign96960_e149502_d_n10, assign96960_e149502_d_n11, assign96960_e149502_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2250 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign96960_e149502;
        locals.var_pzbdswg_dn0 = assign96960_e149502_d_n0;
        locals.var_pzbdswg_dn2 = assign96960_e149502_d_n2;
        locals.var_pzbdswg_dn4 = assign96960_e149502_d_n4;
        locals.var_pzbdswg_dn5 = assign96960_e149502_d_n5;
        locals.var_pzbdswg_dn6 = assign96960_e149502_d_n6;
        locals.var_pzbdswg_dn7 = assign96960_e149502_d_n7;
        locals.var_pzbdswg_dn8 = assign96960_e149502_d_n8;
        locals.var_pzbdswg_dn9 = assign96960_e149502_d_n9;
        locals.var_pzbdswg_dn10 = assign96960_e149502_d_n10;
        locals.var_pzbdswg_dn11 = assign96960_e149502_d_n11;
        locals.var_pzbdswg_dn14 = assign96960_e149502_d_n14;

        let (assign96970_e149514, assign96970_e149514_d_n0, assign96970_e149514_d_n2, assign96970_e149514_d_n4, assign96970_e149514_d_n5, assign96970_e149514_d_n6, assign96970_e149514_d_n7, assign96970_e149514_d_n8, assign96970_e149514_d_n9, assign96970_e149514_d_n10, assign96970_e149514_d_n11, assign96970_e149514_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96970_e149506: f64 = (p.p523 * p.p14);
        let assign96970_e149510: f64 = (p.p482 * locals.var_tdiff);
        let assign96970_e149511: f64 = (1.0 + assign96970_e149510);
        let assign96970_e149512: f64 = (assign96970_e149506 * assign96970_e149511);
        (assign96970_e149512, (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn0)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn2)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn4)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn5)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn6)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn7)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn8)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn9)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn10)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn11)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign96970_e149514;
        locals.var_czbs_dn0 = assign96970_e149514_d_n0;
        locals.var_czbs_dn2 = assign96970_e149514_d_n2;
        locals.var_czbs_dn4 = assign96970_e149514_d_n4;
        locals.var_czbs_dn5 = assign96970_e149514_d_n5;
        locals.var_czbs_dn6 = assign96970_e149514_d_n6;
        locals.var_czbs_dn7 = assign96970_e149514_d_n7;
        locals.var_czbs_dn8 = assign96970_e149514_d_n8;
        locals.var_czbs_dn9 = assign96970_e149514_d_n9;
        locals.var_czbs_dn10 = assign96970_e149514_d_n10;
        locals.var_czbs_dn11 = assign96970_e149514_d_n11;
        locals.var_czbs_dn14 = assign96970_e149514_d_n14;

        let assign96980_e149517: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2251 = assign96980_e149517;

        let (assign96990_e149533, assign96990_e149533_d_n0, assign96990_e149533_d_n2, assign96990_e149533_d_n4, assign96990_e149533_d_n5, assign96990_e149533_d_n6, assign96990_e149533_d_n7, assign96990_e149533_d_n8, assign96990_e149533_d_n9, assign96990_e149533_d_n10, assign96990_e149533_d_n11, assign96990_e149533_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2251 != 0.0)) {
        let assign96990_e149524: f64 = (p.p16 - locals.var_weff_nf);
        let assign96990_e149525: f64 = (p.p524 * assign96990_e149524);
        let assign96990_e149529: f64 = (p.p484 * locals.var_tdiff);
        let assign96990_e149530: f64 = (1.0 + assign96990_e149529);
        let assign96990_e149531: f64 = (assign96990_e149525 * assign96990_e149530);
        (assign96990_e149531, (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn0)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn2)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn4)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn5)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn6)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn7)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn8)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn9)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn10)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn11)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign96990_e149533;
        locals.var_czbssw_dn0 = assign96990_e149533_d_n0;
        locals.var_czbssw_dn2 = assign96990_e149533_d_n2;
        locals.var_czbssw_dn4 = assign96990_e149533_d_n4;
        locals.var_czbssw_dn5 = assign96990_e149533_d_n5;
        locals.var_czbssw_dn6 = assign96990_e149533_d_n6;
        locals.var_czbssw_dn7 = assign96990_e149533_d_n7;
        locals.var_czbssw_dn8 = assign96990_e149533_d_n8;
        locals.var_czbssw_dn9 = assign96990_e149533_d_n9;
        locals.var_czbssw_dn10 = assign96990_e149533_d_n10;
        locals.var_czbssw_dn11 = assign96990_e149533_d_n11;
        locals.var_czbssw_dn14 = assign96990_e149533_d_n14;

        let (assign97000_e149547, assign97000_e149547_d_n0, assign97000_e149547_d_n2, assign97000_e149547_d_n4, assign97000_e149547_d_n5, assign97000_e149547_d_n6, assign97000_e149547_d_n7, assign97000_e149547_d_n8, assign97000_e149547_d_n9, assign97000_e149547_d_n10, assign97000_e149547_d_n11, assign97000_e149547_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2251 != 0.0)) {
        let assign97000_e149539: f64 = (p.p525 * locals.var_weff_nf);
        let assign97000_e149543: f64 = (p.p486 * locals.var_tdiff);
        let assign97000_e149544: f64 = (1.0 + assign97000_e149543);
        let assign97000_e149545: f64 = (assign97000_e149539 * assign97000_e149544);
        (assign97000_e149545, (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn0)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn2)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn4)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn5)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn6)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn7)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn8)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn9)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn10)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn11)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign97000_e149547;
        locals.var_czbsswg_dn0 = assign97000_e149547_d_n0;
        locals.var_czbsswg_dn2 = assign97000_e149547_d_n2;
        locals.var_czbsswg_dn4 = assign97000_e149547_d_n4;
        locals.var_czbsswg_dn5 = assign97000_e149547_d_n5;
        locals.var_czbsswg_dn6 = assign97000_e149547_d_n6;
        locals.var_czbsswg_dn7 = assign97000_e149547_d_n7;
        locals.var_czbsswg_dn8 = assign97000_e149547_d_n8;
        locals.var_czbsswg_dn9 = assign97000_e149547_d_n9;
        locals.var_czbsswg_dn10 = assign97000_e149547_d_n10;
        locals.var_czbsswg_dn11 = assign97000_e149547_d_n11;
        locals.var_czbsswg_dn14 = assign97000_e149547_d_n14;

        let (assign97010_e149554, assign97010_e149554_d_n0, assign97010_e149554_d_n2, assign97010_e149554_d_n4, assign97010_e149554_d_n5, assign97010_e149554_d_n6, assign97010_e149554_d_n7, assign97010_e149554_d_n8, assign97010_e149554_d_n9, assign97010_e149554_d_n10, assign97010_e149554_d_n11, assign97010_e149554_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2251 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign97010_e149554;
        locals.var_czbssw_dn0 = assign97010_e149554_d_n0;
        locals.var_czbssw_dn2 = assign97010_e149554_d_n2;
        locals.var_czbssw_dn4 = assign97010_e149554_d_n4;
        locals.var_czbssw_dn5 = assign97010_e149554_d_n5;
        locals.var_czbssw_dn6 = assign97010_e149554_d_n6;
        locals.var_czbssw_dn7 = assign97010_e149554_d_n7;
        locals.var_czbssw_dn8 = assign97010_e149554_d_n8;
        locals.var_czbssw_dn9 = assign97010_e149554_d_n9;
        locals.var_czbssw_dn10 = assign97010_e149554_d_n10;
        locals.var_czbssw_dn11 = assign97010_e149554_d_n11;
        locals.var_czbssw_dn14 = assign97010_e149554_d_n14;

        let (assign97020_e149569, assign97020_e149569_d_n0, assign97020_e149569_d_n2, assign97020_e149569_d_n4, assign97020_e149569_d_n5, assign97020_e149569_d_n6, assign97020_e149569_d_n7, assign97020_e149569_d_n8, assign97020_e149569_d_n9, assign97020_e149569_d_n10, assign97020_e149569_d_n11, assign97020_e149569_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2251 == 0.0)) {
        let assign97020_e149561: f64 = (p.p525 * p.p16);
        let assign97020_e149565: f64 = (p.p486 * locals.var_tdiff);
        let assign97020_e149566: f64 = (1.0 + assign97020_e149565);
        let assign97020_e149567: f64 = (assign97020_e149561 * assign97020_e149566);
        (assign97020_e149567, (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn0)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn2)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn4)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn5)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn6)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn7)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn8)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn9)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn10)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn11)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign97020_e149569;
        locals.var_czbsswg_dn0 = assign97020_e149569_d_n0;
        locals.var_czbsswg_dn2 = assign97020_e149569_d_n2;
        locals.var_czbsswg_dn4 = assign97020_e149569_d_n4;
        locals.var_czbsswg_dn5 = assign97020_e149569_d_n5;
        locals.var_czbsswg_dn6 = assign97020_e149569_d_n6;
        locals.var_czbsswg_dn7 = assign97020_e149569_d_n7;
        locals.var_czbsswg_dn8 = assign97020_e149569_d_n8;
        locals.var_czbsswg_dn9 = assign97020_e149569_d_n9;
        locals.var_czbsswg_dn10 = assign97020_e149569_d_n10;
        locals.var_czbsswg_dn11 = assign97020_e149569_d_n11;
        locals.var_czbsswg_dn14 = assign97020_e149569_d_n14;

        let assign97030_e149572: f64 = if locals.var_czbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2252 = assign97030_e149572;

        let (assign97040_e149578, assign97040_e149578_d_n0, assign97040_e149578_d_n2, assign97040_e149578_d_n4, assign97040_e149578_d_n5, assign97040_e149578_d_n6, assign97040_e149578_d_n7, assign97040_e149578_d_n8, assign97040_e149578_d_n9, assign97040_e149578_d_n10, assign97040_e149578_d_n11, assign97040_e149578_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2252 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign97040_e149578;
        locals.var_czbs_dn0 = assign97040_e149578_d_n0;
        locals.var_czbs_dn2 = assign97040_e149578_d_n2;
        locals.var_czbs_dn4 = assign97040_e149578_d_n4;
        locals.var_czbs_dn5 = assign97040_e149578_d_n5;
        locals.var_czbs_dn6 = assign97040_e149578_d_n6;
        locals.var_czbs_dn7 = assign97040_e149578_d_n7;
        locals.var_czbs_dn8 = assign97040_e149578_d_n8;
        locals.var_czbs_dn9 = assign97040_e149578_d_n9;
        locals.var_czbs_dn10 = assign97040_e149578_d_n10;
        locals.var_czbs_dn11 = assign97040_e149578_d_n11;
        locals.var_czbs_dn14 = assign97040_e149578_d_n14;

        let assign97050_e149581: f64 = if locals.var_czbssw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2253 = assign97050_e149581;

        let (assign97060_e149587, assign97060_e149587_d_n0, assign97060_e149587_d_n2, assign97060_e149587_d_n4, assign97060_e149587_d_n5, assign97060_e149587_d_n6, assign97060_e149587_d_n7, assign97060_e149587_d_n8, assign97060_e149587_d_n9, assign97060_e149587_d_n10, assign97060_e149587_d_n11, assign97060_e149587_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2253 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign97060_e149587;
        locals.var_czbssw_dn0 = assign97060_e149587_d_n0;
        locals.var_czbssw_dn2 = assign97060_e149587_d_n2;
        locals.var_czbssw_dn4 = assign97060_e149587_d_n4;
        locals.var_czbssw_dn5 = assign97060_e149587_d_n5;
        locals.var_czbssw_dn6 = assign97060_e149587_d_n6;
        locals.var_czbssw_dn7 = assign97060_e149587_d_n7;
        locals.var_czbssw_dn8 = assign97060_e149587_d_n8;
        locals.var_czbssw_dn9 = assign97060_e149587_d_n9;
        locals.var_czbssw_dn10 = assign97060_e149587_d_n10;
        locals.var_czbssw_dn11 = assign97060_e149587_d_n11;
        locals.var_czbssw_dn14 = assign97060_e149587_d_n14;

    }

    pub(super) fn stamp_transient_block_358(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign97070_e149590: f64 = if locals.var_czbsswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2254 = assign97070_e149590;

        let (assign97080_e149596, assign97080_e149596_d_n0, assign97080_e149596_d_n2, assign97080_e149596_d_n4, assign97080_e149596_d_n5, assign97080_e149596_d_n6, assign97080_e149596_d_n7, assign97080_e149596_d_n8, assign97080_e149596_d_n9, assign97080_e149596_d_n10, assign97080_e149596_d_n11, assign97080_e149596_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2254 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign97080_e149596;
        locals.var_czbsswg_dn0 = assign97080_e149596_d_n0;
        locals.var_czbsswg_dn2 = assign97080_e149596_d_n2;
        locals.var_czbsswg_dn4 = assign97080_e149596_d_n4;
        locals.var_czbsswg_dn5 = assign97080_e149596_d_n5;
        locals.var_czbsswg_dn6 = assign97080_e149596_d_n6;
        locals.var_czbsswg_dn7 = assign97080_e149596_d_n7;
        locals.var_czbsswg_dn8 = assign97080_e149596_d_n8;
        locals.var_czbsswg_dn9 = assign97080_e149596_d_n9;
        locals.var_czbsswg_dn10 = assign97080_e149596_d_n10;
        locals.var_czbsswg_dn11 = assign97080_e149596_d_n11;
        locals.var_czbsswg_dn14 = assign97080_e149596_d_n14;

        let (assign97090_e149604, assign97090_e149604_d_n0, assign97090_e149604_d_n2, assign97090_e149604_d_n4, assign97090_e149604_d_n5, assign97090_e149604_d_n6, assign97090_e149604_d_n7, assign97090_e149604_d_n8, assign97090_e149604_d_n9, assign97090_e149604_d_n10, assign97090_e149604_d_n11, assign97090_e149604_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign97090_e149601: f64 = (p.p488 * locals.var_tdiff);
        let assign97090_e149602: f64 = (p.p529 - assign97090_e149601);
        (assign97090_e149602, (-(p.p488 * locals.var_tdiff_dn0)), (-(p.p488 * locals.var_tdiff_dn2)), (-(p.p488 * locals.var_tdiff_dn4)), (-(p.p488 * locals.var_tdiff_dn5)), (-(p.p488 * locals.var_tdiff_dn6)), (-(p.p488 * locals.var_tdiff_dn7)), (-(p.p488 * locals.var_tdiff_dn8)), (-(p.p488 * locals.var_tdiff_dn9)), (-(p.p488 * locals.var_tdiff_dn10)), (-(p.p488 * locals.var_tdiff_dn11)), (-(p.p488 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign97090_e149604;
        locals.var_pzbs_dn0 = assign97090_e149604_d_n0;
        locals.var_pzbs_dn2 = assign97090_e149604_d_n2;
        locals.var_pzbs_dn4 = assign97090_e149604_d_n4;
        locals.var_pzbs_dn5 = assign97090_e149604_d_n5;
        locals.var_pzbs_dn6 = assign97090_e149604_d_n6;
        locals.var_pzbs_dn7 = assign97090_e149604_d_n7;
        locals.var_pzbs_dn8 = assign97090_e149604_d_n8;
        locals.var_pzbs_dn9 = assign97090_e149604_d_n9;
        locals.var_pzbs_dn10 = assign97090_e149604_d_n10;
        locals.var_pzbs_dn11 = assign97090_e149604_d_n11;
        locals.var_pzbs_dn14 = assign97090_e149604_d_n14;

        let (assign97100_e149612, assign97100_e149612_d_n0, assign97100_e149612_d_n2, assign97100_e149612_d_n4, assign97100_e149612_d_n5, assign97100_e149612_d_n6, assign97100_e149612_d_n7, assign97100_e149612_d_n8, assign97100_e149612_d_n9, assign97100_e149612_d_n10, assign97100_e149612_d_n11, assign97100_e149612_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign97100_e149609: f64 = (p.p490 * locals.var_tdiff);
        let assign97100_e149610: f64 = (p.p530 - assign97100_e149609);
        (assign97100_e149610, (-(p.p490 * locals.var_tdiff_dn0)), (-(p.p490 * locals.var_tdiff_dn2)), (-(p.p490 * locals.var_tdiff_dn4)), (-(p.p490 * locals.var_tdiff_dn5)), (-(p.p490 * locals.var_tdiff_dn6)), (-(p.p490 * locals.var_tdiff_dn7)), (-(p.p490 * locals.var_tdiff_dn8)), (-(p.p490 * locals.var_tdiff_dn9)), (-(p.p490 * locals.var_tdiff_dn10)), (-(p.p490 * locals.var_tdiff_dn11)), (-(p.p490 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign97100_e149612;
        locals.var_pzbssw_dn0 = assign97100_e149612_d_n0;
        locals.var_pzbssw_dn2 = assign97100_e149612_d_n2;
        locals.var_pzbssw_dn4 = assign97100_e149612_d_n4;
        locals.var_pzbssw_dn5 = assign97100_e149612_d_n5;
        locals.var_pzbssw_dn6 = assign97100_e149612_d_n6;
        locals.var_pzbssw_dn7 = assign97100_e149612_d_n7;
        locals.var_pzbssw_dn8 = assign97100_e149612_d_n8;
        locals.var_pzbssw_dn9 = assign97100_e149612_d_n9;
        locals.var_pzbssw_dn10 = assign97100_e149612_d_n10;
        locals.var_pzbssw_dn11 = assign97100_e149612_d_n11;
        locals.var_pzbssw_dn14 = assign97100_e149612_d_n14;

        let (assign97110_e149620, assign97110_e149620_d_n0, assign97110_e149620_d_n2, assign97110_e149620_d_n4, assign97110_e149620_d_n5, assign97110_e149620_d_n6, assign97110_e149620_d_n7, assign97110_e149620_d_n8, assign97110_e149620_d_n9, assign97110_e149620_d_n10, assign97110_e149620_d_n11, assign97110_e149620_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign97110_e149617: f64 = (p.p492 * locals.var_tdiff);
        let assign97110_e149618: f64 = (p.p531 - assign97110_e149617);
        (assign97110_e149618, (-(p.p492 * locals.var_tdiff_dn0)), (-(p.p492 * locals.var_tdiff_dn2)), (-(p.p492 * locals.var_tdiff_dn4)), (-(p.p492 * locals.var_tdiff_dn5)), (-(p.p492 * locals.var_tdiff_dn6)), (-(p.p492 * locals.var_tdiff_dn7)), (-(p.p492 * locals.var_tdiff_dn8)), (-(p.p492 * locals.var_tdiff_dn9)), (-(p.p492 * locals.var_tdiff_dn10)), (-(p.p492 * locals.var_tdiff_dn11)), (-(p.p492 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign97110_e149620;
        locals.var_pzbsswg_dn0 = assign97110_e149620_d_n0;
        locals.var_pzbsswg_dn2 = assign97110_e149620_d_n2;
        locals.var_pzbsswg_dn4 = assign97110_e149620_d_n4;
        locals.var_pzbsswg_dn5 = assign97110_e149620_d_n5;
        locals.var_pzbsswg_dn6 = assign97110_e149620_d_n6;
        locals.var_pzbsswg_dn7 = assign97110_e149620_d_n7;
        locals.var_pzbsswg_dn8 = assign97110_e149620_d_n8;
        locals.var_pzbsswg_dn9 = assign97110_e149620_d_n9;
        locals.var_pzbsswg_dn10 = assign97110_e149620_d_n10;
        locals.var_pzbsswg_dn11 = assign97110_e149620_d_n11;
        locals.var_pzbsswg_dn14 = assign97110_e149620_d_n14;

        let assign97120_e149627: f64 = if ((locals.var_pzbs < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2255 = assign97120_e149627;

        let (assign97130_e149633, assign97130_e149633_d_n0, assign97130_e149633_d_n2, assign97130_e149633_d_n4, assign97130_e149633_d_n5, assign97130_e149633_d_n6, assign97130_e149633_d_n7, assign97130_e149633_d_n8, assign97130_e149633_d_n9, assign97130_e149633_d_n10, assign97130_e149633_d_n11, assign97130_e149633_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2255 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign97130_e149633;
        locals.var_pzbs_dn0 = assign97130_e149633_d_n0;
        locals.var_pzbs_dn2 = assign97130_e149633_d_n2;
        locals.var_pzbs_dn4 = assign97130_e149633_d_n4;
        locals.var_pzbs_dn5 = assign97130_e149633_d_n5;
        locals.var_pzbs_dn6 = assign97130_e149633_d_n6;
        locals.var_pzbs_dn7 = assign97130_e149633_d_n7;
        locals.var_pzbs_dn8 = assign97130_e149633_d_n8;
        locals.var_pzbs_dn9 = assign97130_e149633_d_n9;
        locals.var_pzbs_dn10 = assign97130_e149633_d_n10;
        locals.var_pzbs_dn11 = assign97130_e149633_d_n11;
        locals.var_pzbs_dn14 = assign97130_e149633_d_n14;

        let assign97140_e149640: f64 = if ((locals.var_pzbssw < 0.01) && (p.p16 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2256 = assign97140_e149640;

        let (assign97150_e149646, assign97150_e149646_d_n0, assign97150_e149646_d_n2, assign97150_e149646_d_n4, assign97150_e149646_d_n5, assign97150_e149646_d_n6, assign97150_e149646_d_n7, assign97150_e149646_d_n8, assign97150_e149646_d_n9, assign97150_e149646_d_n10, assign97150_e149646_d_n11, assign97150_e149646_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2256 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign97150_e149646;
        locals.var_pzbssw_dn0 = assign97150_e149646_d_n0;
        locals.var_pzbssw_dn2 = assign97150_e149646_d_n2;
        locals.var_pzbssw_dn4 = assign97150_e149646_d_n4;
        locals.var_pzbssw_dn5 = assign97150_e149646_d_n5;
        locals.var_pzbssw_dn6 = assign97150_e149646_d_n6;
        locals.var_pzbssw_dn7 = assign97150_e149646_d_n7;
        locals.var_pzbssw_dn8 = assign97150_e149646_d_n8;
        locals.var_pzbssw_dn9 = assign97150_e149646_d_n9;
        locals.var_pzbssw_dn10 = assign97150_e149646_d_n10;
        locals.var_pzbssw_dn11 = assign97150_e149646_d_n11;
        locals.var_pzbssw_dn14 = assign97150_e149646_d_n14;

        let assign97160_e149653: f64 = if ((locals.var_pzbsswg < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2257 = assign97160_e149653;

        let (assign97170_e149659, assign97170_e149659_d_n0, assign97170_e149659_d_n2, assign97170_e149659_d_n4, assign97170_e149659_d_n5, assign97170_e149659_d_n6, assign97170_e149659_d_n7, assign97170_e149659_d_n8, assign97170_e149659_d_n9, assign97170_e149659_d_n10, assign97170_e149659_d_n11, assign97170_e149659_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2257 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign97170_e149659;
        locals.var_pzbsswg_dn0 = assign97170_e149659_d_n0;
        locals.var_pzbsswg_dn2 = assign97170_e149659_d_n2;
        locals.var_pzbsswg_dn4 = assign97170_e149659_d_n4;
        locals.var_pzbsswg_dn5 = assign97170_e149659_d_n5;
        locals.var_pzbsswg_dn6 = assign97170_e149659_d_n6;
        locals.var_pzbsswg_dn7 = assign97170_e149659_d_n7;
        locals.var_pzbsswg_dn8 = assign97170_e149659_d_n8;
        locals.var_pzbsswg_dn9 = assign97170_e149659_d_n9;
        locals.var_pzbsswg_dn10 = assign97170_e149659_d_n10;
        locals.var_pzbsswg_dn11 = assign97170_e149659_d_n11;
        locals.var_pzbsswg_dn14 = assign97170_e149659_d_n14;

        let (assign97180_e149666, assign97180_e149666_d_n0, assign97180_e149666_d_n2, assign97180_e149666_d_n4, assign97180_e149666_d_n5, assign97180_e149666_d_n6, assign97180_e149666_d_n7, assign97180_e149666_d_n8, assign97180_e149666_d_n9, assign97180_e149666_d_n10, assign97180_e149666_d_n11, assign97180_e149666_d_n14,) = {
    if (locals.var_guard2237 == 0.0) {
        let assign97180_e149662: f64 = ctx_temp;
        let assign97180_e149664: f64 = (assign97180_e149662 + p.p11);
        (assign97180_e149664, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign97180_e149666;
        locals.var_ttemp_dn0 = assign97180_e149666_d_n0;
        locals.var_ttemp_dn2 = assign97180_e149666_d_n2;
        locals.var_ttemp_dn4 = assign97180_e149666_d_n4;
        locals.var_ttemp_dn5 = assign97180_e149666_d_n5;
        locals.var_ttemp_dn6 = assign97180_e149666_d_n6;
        locals.var_ttemp_dn7 = assign97180_e149666_d_n7;
        locals.var_ttemp_dn8 = assign97180_e149666_d_n8;
        locals.var_ttemp_dn9 = assign97180_e149666_d_n9;
        locals.var_ttemp_dn10 = assign97180_e149666_d_n10;
        locals.var_ttemp_dn11 = assign97180_e149666_d_n11;
        locals.var_ttemp_dn14 = assign97180_e149666_d_n14;

        let assign97190_e149669: f64 = (p.p511 * locals.var_jd_nvtm_invd);
        locals.var_t10 = assign97190_e149669;
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

        let assign97200_e149672: f64 = (p.p510 * locals.var_exptempd);
        locals.var_t9 = assign97200_e149672;
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

        let assign97210_e149675: f64 = if locals.var_isbd_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2258 = assign97210_e149675;

        let (assign97220_e149681, assign97220_e149681_d_n0, assign97220_e149681_d_n2, assign97220_e149681_d_n4, assign97220_e149681_d_n5, assign97220_e149681_d_n6, assign97220_e149681_d_n7, assign97220_e149681_d_n8, assign97220_e149681_d_n9, assign97220_e149681_d_n10, assign97220_e149681_d_n11, assign97220_e149681_d_n14,) = {
    if (locals.var_guard2258 != 0.0) {
        let assign97220_e149679: f64 = (locals.var_isbd2_btm * locals.var_t9);
        (assign97220_e149679, ((locals.var_isbd2_btm_dn0 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn0)), ((locals.var_isbd2_btm_dn2 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn2)), ((locals.var_isbd2_btm_dn4 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn4)), ((locals.var_isbd2_btm_dn5 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn5)), ((locals.var_isbd2_btm_dn6 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn6)), ((locals.var_isbd2_btm_dn7 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn7)), ((locals.var_isbd2_btm_dn8 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn8)), ((locals.var_isbd2_btm_dn9 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn9)), ((locals.var_isbd2_btm_dn10 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn10)), ((locals.var_isbd2_btm_dn11 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn11)), ((locals.var_isbd2_btm_dn14 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97220_e149681;
        locals.var_t0_dn0 = assign97220_e149681_d_n0;
        locals.var_t0_dn2 = assign97220_e149681_d_n2;
        locals.var_t0_dn4 = assign97220_e149681_d_n4;
        locals.var_t0_dn5 = assign97220_e149681_d_n5;
        locals.var_t0_dn6 = assign97220_e149681_d_n6;
        locals.var_t0_dn7 = assign97220_e149681_d_n7;
        locals.var_t0_dn8 = assign97220_e149681_d_n8;
        locals.var_t0_dn9 = assign97220_e149681_d_n9;
        locals.var_t0_dn10 = assign97220_e149681_d_n10;
        locals.var_t0_dn11 = assign97220_e149681_d_n11;
        locals.var_t0_dn14 = assign97220_e149681_d_n14;

        let (assign97230_e149688, assign97230_e149688_d_n0, assign97230_e149688_d_n2, assign97230_e149688_d_n4, assign97230_e149688_d_n5, assign97230_e149688_d_n6, assign97230_e149688_d_n7, assign97230_e149688_d_n8, assign97230_e149688_d_n9, assign97230_e149688_d_n10, assign97230_e149688_d_n11, assign97230_e149688_d_n14,) = {
    if (locals.var_guard2258 != 0.0) {
        let assign97230_e149684: f64 = (-locals.var_vbd_jct);
        let assign97230_e149686: f64 = (assign97230_e149684 * locals.var_t10);
        (assign97230_e149686, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97230_e149684 * locals.var_t10_dn0)), (assign97230_e149684 * locals.var_t10_dn2), (assign97230_e149684 * locals.var_t10_dn4), (assign97230_e149684 * locals.var_t10_dn5), (assign97230_e149684 * locals.var_t10_dn6), (assign97230_e149684 * locals.var_t10_dn7), (assign97230_e149684 * locals.var_t10_dn8), (assign97230_e149684 * locals.var_t10_dn9), (((-locals.var_vbd_jct_dn10) * locals.var_t10) + (assign97230_e149684 * locals.var_t10_dn10)), (assign97230_e149684 * locals.var_t10_dn11), (assign97230_e149684 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97230_e149688;
        locals.var_tx_dn0 = assign97230_e149688_d_n0;
        locals.var_tx_dn2 = assign97230_e149688_d_n2;
        locals.var_tx_dn4 = assign97230_e149688_d_n4;
        locals.var_tx_dn5 = assign97230_e149688_d_n5;
        locals.var_tx_dn6 = assign97230_e149688_d_n6;
        locals.var_tx_dn7 = assign97230_e149688_d_n7;
        locals.var_tx_dn8 = assign97230_e149688_d_n8;
        locals.var_tx_dn9 = assign97230_e149688_d_n9;
        locals.var_tx_dn10 = assign97230_e149688_d_n10;
        locals.var_tx_dn11 = assign97230_e149688_d_n11;
        locals.var_tx_dn14 = assign97230_e149688_d_n14;

        let (assign97240_e149693, assign97240_e149693_d_n0, assign97240_e149693_d_n2, assign97240_e149693_d_n4, assign97240_e149693_d_n5, assign97240_e149693_d_n6, assign97240_e149693_d_n7, assign97240_e149693_d_n8, assign97240_e149693_d_n9, assign97240_e149693_d_n10, assign97240_e149693_d_n11, assign97240_e149693_d_n14,) = {
    if (locals.var_guard2258 != 0.0) {
        let assign97240_e149691: f64 = (locals.var_tx).exp();
        (assign97240_e149691, (assign97240_e149691 * locals.var_tx_dn0), (assign97240_e149691 * locals.var_tx_dn2), (assign97240_e149691 * locals.var_tx_dn4), (assign97240_e149691 * locals.var_tx_dn5), (assign97240_e149691 * locals.var_tx_dn6), (assign97240_e149691 * locals.var_tx_dn7), (assign97240_e149691 * locals.var_tx_dn8), (assign97240_e149691 * locals.var_tx_dn9), (assign97240_e149691 * locals.var_tx_dn10), (assign97240_e149691 * locals.var_tx_dn11), (assign97240_e149691 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97240_e149693;
        locals.var_t2_dn0 = assign97240_e149693_d_n0;
        locals.var_t2_dn2 = assign97240_e149693_d_n2;
        locals.var_t2_dn4 = assign97240_e149693_d_n4;
        locals.var_t2_dn5 = assign97240_e149693_d_n5;
        locals.var_t2_dn6 = assign97240_e149693_d_n6;
        locals.var_t2_dn7 = assign97240_e149693_d_n7;
        locals.var_t2_dn8 = assign97240_e149693_d_n8;
        locals.var_t2_dn9 = assign97240_e149693_d_n9;
        locals.var_t2_dn10 = assign97240_e149693_d_n10;
        locals.var_t2_dn11 = assign97240_e149693_d_n11;
        locals.var_t2_dn14 = assign97240_e149693_d_n14;

        let (assign97250_e149697, assign97250_e149697_d_n0, assign97250_e149697_d_n2, assign97250_e149697_d_n4, assign97250_e149697_d_n5, assign97250_e149697_d_n6, assign97250_e149697_d_n7, assign97250_e149697_d_n8, assign97250_e149697_d_n9, assign97250_e149697_d_n10, assign97250_e149697_d_n11, assign97250_e149697_d_n14,) = {
    if (locals.var_guard2258 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97250_e149697;
        locals.var_t3_dn0 = assign97250_e149697_d_n0;
        locals.var_t3_dn2 = assign97250_e149697_d_n2;
        locals.var_t3_dn4 = assign97250_e149697_d_n4;
        locals.var_t3_dn5 = assign97250_e149697_d_n5;
        locals.var_t3_dn6 = assign97250_e149697_d_n6;
        locals.var_t3_dn7 = assign97250_e149697_d_n7;
        locals.var_t3_dn8 = assign97250_e149697_d_n8;
        locals.var_t3_dn9 = assign97250_e149697_d_n9;
        locals.var_t3_dn10 = assign97250_e149697_d_n10;
        locals.var_t3_dn11 = assign97250_e149697_d_n11;
        locals.var_t3_dn14 = assign97250_e149697_d_n14;

        let assign97260_e149700: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2259 = assign97260_e149700;

        let (assign97270_e149708, assign97270_e149708_d_n0, assign97270_e149708_d_n2, assign97270_e149708_d_n4, assign97270_e149708_d_n5, assign97270_e149708_d_n6, assign97270_e149708_d_n7, assign97270_e149708_d_n8, assign97270_e149708_d_n9, assign97270_e149708_d_n10, assign97270_e149708_d_n11, assign97270_e149708_d_n14,) = {
    if ((locals.var_guard2258 != 0.0) && (locals.var_guard2259 != 0.0)) {
        let assign97270_e149706: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97270_e149706, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9), ((locals.var_vbd_jct_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97270_e149708;
        locals.var_tx_dn0 = assign97270_e149708_d_n0;
        locals.var_tx_dn2 = assign97270_e149708_d_n2;
        locals.var_tx_dn4 = assign97270_e149708_d_n4;
        locals.var_tx_dn5 = assign97270_e149708_d_n5;
        locals.var_tx_dn6 = assign97270_e149708_d_n6;
        locals.var_tx_dn7 = assign97270_e149708_d_n7;
        locals.var_tx_dn8 = assign97270_e149708_d_n8;
        locals.var_tx_dn9 = assign97270_e149708_d_n9;
        locals.var_tx_dn10 = assign97270_e149708_d_n10;
        locals.var_tx_dn11 = assign97270_e149708_d_n11;
        locals.var_tx_dn14 = assign97270_e149708_d_n14;

        let assign97280_e149711: f64 = (-3.0);
        let assign97280_e149713: f64 = (assign97280_e149711 * 34.0);
        let assign97280_e149714: f64 = if locals.var_tx < assign97280_e149713 { 1.0 } else { 0.0 };
        locals.var_guard2260 = assign97280_e149714;

        let (assign97290_e149722, assign97290_e149722_d_n0, assign97290_e149722_d_n2, assign97290_e149722_d_n4, assign97290_e149722_d_n5, assign97290_e149722_d_n6, assign97290_e149722_d_n7, assign97290_e149722_d_n8, assign97290_e149722_d_n9, assign97290_e149722_d_n10, assign97290_e149722_d_n11, assign97290_e149722_d_n14,) = {
    if (((locals.var_guard2258 != 0.0) && (locals.var_guard2259 != 0.0)) && (locals.var_guard2260 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97290_e149722;
        locals.var_t1_dn0 = assign97290_e149722_d_n0;
        locals.var_t1_dn2 = assign97290_e149722_d_n2;
        locals.var_t1_dn4 = assign97290_e149722_d_n4;
        locals.var_t1_dn5 = assign97290_e149722_d_n5;
        locals.var_t1_dn6 = assign97290_e149722_d_n6;
        locals.var_t1_dn7 = assign97290_e149722_d_n7;
        locals.var_t1_dn8 = assign97290_e149722_d_n8;
        locals.var_t1_dn9 = assign97290_e149722_d_n9;
        locals.var_t1_dn10 = assign97290_e149722_d_n10;
        locals.var_t1_dn11 = assign97290_e149722_d_n11;
        locals.var_t1_dn14 = assign97290_e149722_d_n14;

        let (assign97300_e149732, assign97300_e149732_d_n0, assign97300_e149732_d_n2, assign97300_e149732_d_n4, assign97300_e149732_d_n5, assign97300_e149732_d_n6, assign97300_e149732_d_n7, assign97300_e149732_d_n8, assign97300_e149732_d_n9, assign97300_e149732_d_n10, assign97300_e149732_d_n11, assign97300_e149732_d_n14,) = {
    if (((locals.var_guard2258 != 0.0) && (locals.var_guard2259 != 0.0)) && (locals.var_guard2260 == 0.0)) {
        let assign97300_e149730: f64 = (locals.var_tx).exp();
        (assign97300_e149730, (assign97300_e149730 * locals.var_tx_dn0), (assign97300_e149730 * locals.var_tx_dn2), (assign97300_e149730 * locals.var_tx_dn4), (assign97300_e149730 * locals.var_tx_dn5), (assign97300_e149730 * locals.var_tx_dn6), (assign97300_e149730 * locals.var_tx_dn7), (assign97300_e149730 * locals.var_tx_dn8), (assign97300_e149730 * locals.var_tx_dn9), (assign97300_e149730 * locals.var_tx_dn10), (assign97300_e149730 * locals.var_tx_dn11), (assign97300_e149730 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97300_e149732;
        locals.var_t1_dn0 = assign97300_e149732_d_n0;
        locals.var_t1_dn2 = assign97300_e149732_d_n2;
        locals.var_t1_dn4 = assign97300_e149732_d_n4;
        locals.var_t1_dn5 = assign97300_e149732_d_n5;
        locals.var_t1_dn6 = assign97300_e149732_d_n6;
        locals.var_t1_dn7 = assign97300_e149732_d_n7;
        locals.var_t1_dn8 = assign97300_e149732_d_n8;
        locals.var_t1_dn9 = assign97300_e149732_d_n9;
        locals.var_t1_dn10 = assign97300_e149732_d_n10;
        locals.var_t1_dn11 = assign97300_e149732_d_n11;
        locals.var_t1_dn14 = assign97300_e149732_d_n14;

        let (assign97310_e149754, assign97310_e149754_d_n0, assign97310_e149754_d_n2, assign97310_e149754_d_n4, assign97310_e149754_d_n5, assign97310_e149754_d_n6, assign97310_e149754_d_n7, assign97310_e149754_d_n8, assign97310_e149754_d_n9, assign97310_e149754_d_n10, assign97310_e149754_d_n11, assign97310_e149754_d_n14,) = {
    if ((locals.var_guard2258 != 0.0) && (locals.var_guard2259 != 0.0)) {
        let assign97310_e149739: f64 = (locals.var_t1 - 1.0);
        let assign97310_e149740: f64 = (locals.var_isbd_btm * assign97310_e149739);
        let assign97310_e149744: f64 = (locals.var_t2 - 1.0);
        let assign97310_e149745: f64 = (locals.var_t0 * assign97310_e149744);
        let assign97310_e149746: f64 = (assign97310_e149740 + assign97310_e149745);
        let assign97310_e149750: f64 = (locals.var_t3 - 1.0);
        let assign97310_e149751: f64 = (locals.var_uc_cisbkd * assign97310_e149750);
        let assign97310_e149752: f64 = (assign97310_e149746 + assign97310_e149751);
        (assign97310_e149752, ((((locals.var_isbd_btm_dn0 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), ((((locals.var_isbd_btm_dn2 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), ((((locals.var_isbd_btm_dn4 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), ((((locals.var_isbd_btm_dn5 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), ((((locals.var_isbd_btm_dn6 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), ((((locals.var_isbd_btm_dn7 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), ((((locals.var_isbd_btm_dn8 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), ((((locals.var_isbd_btm_dn9 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), ((((locals.var_isbd_btm_dn10 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), ((((locals.var_isbd_btm_dn11 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn11)) + ((locals.var_t0_dn11 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbkd * locals.var_t3_dn11)), ((((locals.var_isbd_btm_dn14 * assign97310_e149739) + (locals.var_isbd_btm * locals.var_t1_dn14)) + ((locals.var_t0_dn14 * assign97310_e149744) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbkd * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibd_btm, locals.var_ibd_btm_dn0, locals.var_ibd_btm_dn2, locals.var_ibd_btm_dn4, locals.var_ibd_btm_dn5, locals.var_ibd_btm_dn6, locals.var_ibd_btm_dn7, locals.var_ibd_btm_dn8, locals.var_ibd_btm_dn9, locals.var_ibd_btm_dn10, locals.var_ibd_btm_dn11, locals.var_ibd_btm_dn14,)
    }
};
        locals.var_ibd_btm = assign97310_e149754;
        locals.var_ibd_btm_dn0 = assign97310_e149754_d_n0;
        locals.var_ibd_btm_dn2 = assign97310_e149754_d_n2;
        locals.var_ibd_btm_dn4 = assign97310_e149754_d_n4;
        locals.var_ibd_btm_dn5 = assign97310_e149754_d_n5;
        locals.var_ibd_btm_dn6 = assign97310_e149754_d_n6;
        locals.var_ibd_btm_dn7 = assign97310_e149754_d_n7;
        locals.var_ibd_btm_dn8 = assign97310_e149754_d_n8;
        locals.var_ibd_btm_dn9 = assign97310_e149754_d_n9;
        locals.var_ibd_btm_dn10 = assign97310_e149754_d_n10;
        locals.var_ibd_btm_dn11 = assign97310_e149754_d_n11;
        locals.var_ibd_btm_dn14 = assign97310_e149754_d_n14;

        let (assign97320_e149761, assign97320_e149761_d_n0, assign97320_e149761_d_n2, assign97320_e149761_d_n4, assign97320_e149761_d_n5, assign97320_e149761_d_n6, assign97320_e149761_d_n7, assign97320_e149761_d_n8, assign97320_e149761_d_n9, assign97320_e149761_d_n10, assign97320_e149761_d_n11, assign97320_e149761_d_n14,) = {
    if ((locals.var_guard2258 != 0.0) && (locals.var_guard2259 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97320_e149761;
        locals.var_t1_dn0 = assign97320_e149761_d_n0;
        locals.var_t1_dn2 = assign97320_e149761_d_n2;
        locals.var_t1_dn4 = assign97320_e149761_d_n4;
        locals.var_t1_dn5 = assign97320_e149761_d_n5;
        locals.var_t1_dn6 = assign97320_e149761_d_n6;
        locals.var_t1_dn7 = assign97320_e149761_d_n7;
        locals.var_t1_dn8 = assign97320_e149761_d_n8;
        locals.var_t1_dn9 = assign97320_e149761_d_n9;
        locals.var_t1_dn10 = assign97320_e149761_d_n10;
        locals.var_t1_dn11 = assign97320_e149761_d_n11;
        locals.var_t1_dn14 = assign97320_e149761_d_n14;

        let (assign97330_e149772, assign97330_e149772_d_n0, assign97330_e149772_d_n2, assign97330_e149772_d_n4, assign97330_e149772_d_n5, assign97330_e149772_d_n6, assign97330_e149772_d_n7, assign97330_e149772_d_n8, assign97330_e149772_d_n9, assign97330_e149772_d_n10, assign97330_e149772_d_n11, assign97330_e149772_d_n14,) = {
    if ((locals.var_guard2258 != 0.0) && (locals.var_guard2259 == 0.0)) {
        let assign97330_e149768: f64 = (locals.var_isbd_btm * locals.var_jd_nvtm_invd);
        let assign97330_e149770: f64 = (assign97330_e149768 * locals.var_t1);
        (assign97330_e149770, ((((locals.var_isbd_btm_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn0)), ((((locals.var_isbd_btm_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn2)), ((((locals.var_isbd_btm_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn4)), ((((locals.var_isbd_btm_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn5)), ((((locals.var_isbd_btm_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn6)), ((((locals.var_isbd_btm_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn7)), ((((locals.var_isbd_btm_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn8)), ((((locals.var_isbd_btm_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn9)), ((((locals.var_isbd_btm_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn10)), ((((locals.var_isbd_btm_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn11)), ((((locals.var_isbd_btm_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97330_e149772;
        locals.var_t4_dn0 = assign97330_e149772_d_n0;
        locals.var_t4_dn2 = assign97330_e149772_d_n2;
        locals.var_t4_dn4 = assign97330_e149772_d_n4;
        locals.var_t4_dn5 = assign97330_e149772_d_n5;
        locals.var_t4_dn6 = assign97330_e149772_d_n6;
        locals.var_t4_dn7 = assign97330_e149772_d_n7;
        locals.var_t4_dn8 = assign97330_e149772_d_n8;
        locals.var_t4_dn9 = assign97330_e149772_d_n9;
        locals.var_t4_dn10 = assign97330_e149772_d_n10;
        locals.var_t4_dn11 = assign97330_e149772_d_n11;
        locals.var_t4_dn14 = assign97330_e149772_d_n14;

        let (assign97340_e149801, assign97340_e149801_d_n0, assign97340_e149801_d_n2, assign97340_e149801_d_n4, assign97340_e149801_d_n5, assign97340_e149801_d_n6, assign97340_e149801_d_n7, assign97340_e149801_d_n8, assign97340_e149801_d_n9, assign97340_e149801_d_n10, assign97340_e149801_d_n11, assign97340_e149801_d_n14,) = {
    if ((locals.var_guard2258 != 0.0) && (locals.var_guard2259 == 0.0)) {
        let assign97340_e149780: f64 = (locals.var_t1 - 1.0);
        let assign97340_e149781: f64 = (locals.var_isbd_btm * assign97340_e149780);
        let assign97340_e149785: f64 = (locals.var_vbd_jct - locals.var_vbdt);
        let assign97340_e149786: f64 = (locals.var_t4 * assign97340_e149785);
        let assign97340_e149787: f64 = (assign97340_e149781 + assign97340_e149786);
        let assign97340_e149791: f64 = (locals.var_t2 - 1.0);
        let assign97340_e149792: f64 = (locals.var_t0 * assign97340_e149791);
        let assign97340_e149793: f64 = (assign97340_e149787 + assign97340_e149792);
        let assign97340_e149797: f64 = (locals.var_t3 - 1.0);
        let assign97340_e149798: f64 = (locals.var_uc_cisbkd * assign97340_e149797);
        let assign97340_e149799: f64 = (assign97340_e149793 + assign97340_e149798);
        (assign97340_e149799, (((((locals.var_isbd_btm_dn0 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign97340_e149785) + (locals.var_t4 * (locals.var_vbd_jct_dn0 - locals.var_vbdt_dn0)))) + ((locals.var_t0_dn0 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), (((((locals.var_isbd_btm_dn2 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign97340_e149785) + (locals.var_t4 * (-locals.var_vbdt_dn2)))) + ((locals.var_t0_dn2 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), (((((locals.var_isbd_btm_dn4 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign97340_e149785) + (locals.var_t4 * (-locals.var_vbdt_dn4)))) + ((locals.var_t0_dn4 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), (((((locals.var_isbd_btm_dn5 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign97340_e149785) + (locals.var_t4 * (-locals.var_vbdt_dn5)))) + ((locals.var_t0_dn5 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), (((((locals.var_isbd_btm_dn6 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign97340_e149785) + (locals.var_t4 * (-locals.var_vbdt_dn6)))) + ((locals.var_t0_dn6 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), (((((locals.var_isbd_btm_dn7 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign97340_e149785) + (locals.var_t4 * (-locals.var_vbdt_dn7)))) + ((locals.var_t0_dn7 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), (((((locals.var_isbd_btm_dn8 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign97340_e149785) + (locals.var_t4 * (-locals.var_vbdt_dn8)))) + ((locals.var_t0_dn8 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), (((((locals.var_isbd_btm_dn9 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign97340_e149785) + (locals.var_t4 * (-locals.var_vbdt_dn9)))) + ((locals.var_t0_dn9 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), (((((locals.var_isbd_btm_dn10 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign97340_e149785) + (locals.var_t4 * (locals.var_vbd_jct_dn10 - locals.var_vbdt_dn10)))) + ((locals.var_t0_dn10 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), (((((locals.var_isbd_btm_dn11 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * assign97340_e149785) + (locals.var_t4 * (-locals.var_vbdt_dn11)))) + ((locals.var_t0_dn11 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbkd * locals.var_t3_dn11)), (((((locals.var_isbd_btm_dn14 * assign97340_e149780) + (locals.var_isbd_btm * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * assign97340_e149785) + (locals.var_t4 * (-locals.var_vbdt_dn14)))) + ((locals.var_t0_dn14 * assign97340_e149791) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbkd * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibd_btm, locals.var_ibd_btm_dn0, locals.var_ibd_btm_dn2, locals.var_ibd_btm_dn4, locals.var_ibd_btm_dn5, locals.var_ibd_btm_dn6, locals.var_ibd_btm_dn7, locals.var_ibd_btm_dn8, locals.var_ibd_btm_dn9, locals.var_ibd_btm_dn10, locals.var_ibd_btm_dn11, locals.var_ibd_btm_dn14,)
    }
};
        locals.var_ibd_btm = assign97340_e149801;
        locals.var_ibd_btm_dn0 = assign97340_e149801_d_n0;
        locals.var_ibd_btm_dn2 = assign97340_e149801_d_n2;
        locals.var_ibd_btm_dn4 = assign97340_e149801_d_n4;
        locals.var_ibd_btm_dn5 = assign97340_e149801_d_n5;
        locals.var_ibd_btm_dn6 = assign97340_e149801_d_n6;
        locals.var_ibd_btm_dn7 = assign97340_e149801_d_n7;
        locals.var_ibd_btm_dn8 = assign97340_e149801_d_n8;
        locals.var_ibd_btm_dn9 = assign97340_e149801_d_n9;
        locals.var_ibd_btm_dn10 = assign97340_e149801_d_n10;
        locals.var_ibd_btm_dn11 = assign97340_e149801_d_n11;
        locals.var_ibd_btm_dn14 = assign97340_e149801_d_n14;

        let (assign97350_e149806, assign97350_e149806_d_n0, assign97350_e149806_d_n2, assign97350_e149806_d_n4, assign97350_e149806_d_n5, assign97350_e149806_d_n6, assign97350_e149806_d_n7, assign97350_e149806_d_n8, assign97350_e149806_d_n9, assign97350_e149806_d_n10, assign97350_e149806_d_n11, assign97350_e149806_d_n14,) = {
    if (locals.var_guard2258 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd_btm, locals.var_ibd_btm_dn0, locals.var_ibd_btm_dn2, locals.var_ibd_btm_dn4, locals.var_ibd_btm_dn5, locals.var_ibd_btm_dn6, locals.var_ibd_btm_dn7, locals.var_ibd_btm_dn8, locals.var_ibd_btm_dn9, locals.var_ibd_btm_dn10, locals.var_ibd_btm_dn11, locals.var_ibd_btm_dn14,)
    }
};
        locals.var_ibd_btm = assign97350_e149806;
        locals.var_ibd_btm_dn0 = assign97350_e149806_d_n0;
        locals.var_ibd_btm_dn2 = assign97350_e149806_d_n2;
        locals.var_ibd_btm_dn4 = assign97350_e149806_d_n4;
        locals.var_ibd_btm_dn5 = assign97350_e149806_d_n5;
        locals.var_ibd_btm_dn6 = assign97350_e149806_d_n6;
        locals.var_ibd_btm_dn7 = assign97350_e149806_d_n7;
        locals.var_ibd_btm_dn8 = assign97350_e149806_d_n8;
        locals.var_ibd_btm_dn9 = assign97350_e149806_d_n9;
        locals.var_ibd_btm_dn10 = assign97350_e149806_d_n10;
        locals.var_ibd_btm_dn11 = assign97350_e149806_d_n11;
        locals.var_ibd_btm_dn14 = assign97350_e149806_d_n14;

        let assign97360_e149809: f64 = (p.p514 * locals.var_isbd2_btm);
        locals.var_t12 = assign97360_e149809;
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
        let assign97370_e149813: f64 = (locals.var_t12 * locals.var_vbd_jct);
        let assign97370_e149814: f64 = (locals.var_ibd_btm + assign97370_e149813);
        locals.var_ibd_btm = assign97370_e149814;
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

        let assign97380_e149817: f64 = if locals.var_isbd_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2261 = assign97380_e149817;

        let (assign97390_e149823, assign97390_e149823_d_n0, assign97390_e149823_d_n2, assign97390_e149823_d_n4, assign97390_e149823_d_n5, assign97390_e149823_d_n6, assign97390_e149823_d_n7, assign97390_e149823_d_n8, assign97390_e149823_d_n9, assign97390_e149823_d_n10, assign97390_e149823_d_n11, assign97390_e149823_d_n14,) = {
    if (locals.var_guard2261 != 0.0) {
        let assign97390_e149821: f64 = (locals.var_isbd2_sws * locals.var_t9);
        (assign97390_e149821, ((locals.var_isbd2_sws_dn0 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn0)), ((locals.var_isbd2_sws_dn2 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn2)), ((locals.var_isbd2_sws_dn4 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn4)), ((locals.var_isbd2_sws_dn5 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn5)), ((locals.var_isbd2_sws_dn6 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn6)), ((locals.var_isbd2_sws_dn7 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn7)), ((locals.var_isbd2_sws_dn8 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn8)), ((locals.var_isbd2_sws_dn9 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn9)), ((locals.var_isbd2_sws_dn10 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn10)), ((locals.var_isbd2_sws_dn11 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn11)), ((locals.var_isbd2_sws_dn14 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97390_e149823;
        locals.var_t0_dn0 = assign97390_e149823_d_n0;
        locals.var_t0_dn2 = assign97390_e149823_d_n2;
        locals.var_t0_dn4 = assign97390_e149823_d_n4;
        locals.var_t0_dn5 = assign97390_e149823_d_n5;
        locals.var_t0_dn6 = assign97390_e149823_d_n6;
        locals.var_t0_dn7 = assign97390_e149823_d_n7;
        locals.var_t0_dn8 = assign97390_e149823_d_n8;
        locals.var_t0_dn9 = assign97390_e149823_d_n9;
        locals.var_t0_dn10 = assign97390_e149823_d_n10;
        locals.var_t0_dn11 = assign97390_e149823_d_n11;
        locals.var_t0_dn14 = assign97390_e149823_d_n14;

        let (assign97400_e149830, assign97400_e149830_d_n0, assign97400_e149830_d_n2, assign97400_e149830_d_n4, assign97400_e149830_d_n5, assign97400_e149830_d_n6, assign97400_e149830_d_n7, assign97400_e149830_d_n8, assign97400_e149830_d_n9, assign97400_e149830_d_n10, assign97400_e149830_d_n11, assign97400_e149830_d_n14,) = {
    if (locals.var_guard2261 != 0.0) {
        let assign97400_e149826: f64 = (-locals.var_vbd_jct);
        let assign97400_e149828: f64 = (assign97400_e149826 * locals.var_t10);
        (assign97400_e149828, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97400_e149826 * locals.var_t10_dn0)), (assign97400_e149826 * locals.var_t10_dn2), (assign97400_e149826 * locals.var_t10_dn4), (assign97400_e149826 * locals.var_t10_dn5), (assign97400_e149826 * locals.var_t10_dn6), (assign97400_e149826 * locals.var_t10_dn7), (assign97400_e149826 * locals.var_t10_dn8), (assign97400_e149826 * locals.var_t10_dn9), (((-locals.var_vbd_jct_dn10) * locals.var_t10) + (assign97400_e149826 * locals.var_t10_dn10)), (assign97400_e149826 * locals.var_t10_dn11), (assign97400_e149826 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97400_e149830;
        locals.var_tx_dn0 = assign97400_e149830_d_n0;
        locals.var_tx_dn2 = assign97400_e149830_d_n2;
        locals.var_tx_dn4 = assign97400_e149830_d_n4;
        locals.var_tx_dn5 = assign97400_e149830_d_n5;
        locals.var_tx_dn6 = assign97400_e149830_d_n6;
        locals.var_tx_dn7 = assign97400_e149830_d_n7;
        locals.var_tx_dn8 = assign97400_e149830_d_n8;
        locals.var_tx_dn9 = assign97400_e149830_d_n9;
        locals.var_tx_dn10 = assign97400_e149830_d_n10;
        locals.var_tx_dn11 = assign97400_e149830_d_n11;
        locals.var_tx_dn14 = assign97400_e149830_d_n14;

        let (assign97410_e149835, assign97410_e149835_d_n0, assign97410_e149835_d_n2, assign97410_e149835_d_n4, assign97410_e149835_d_n5, assign97410_e149835_d_n6, assign97410_e149835_d_n7, assign97410_e149835_d_n8, assign97410_e149835_d_n9, assign97410_e149835_d_n10, assign97410_e149835_d_n11, assign97410_e149835_d_n14,) = {
    if (locals.var_guard2261 != 0.0) {
        let assign97410_e149833: f64 = (locals.var_tx).exp();
        (assign97410_e149833, (assign97410_e149833 * locals.var_tx_dn0), (assign97410_e149833 * locals.var_tx_dn2), (assign97410_e149833 * locals.var_tx_dn4), (assign97410_e149833 * locals.var_tx_dn5), (assign97410_e149833 * locals.var_tx_dn6), (assign97410_e149833 * locals.var_tx_dn7), (assign97410_e149833 * locals.var_tx_dn8), (assign97410_e149833 * locals.var_tx_dn9), (assign97410_e149833 * locals.var_tx_dn10), (assign97410_e149833 * locals.var_tx_dn11), (assign97410_e149833 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97410_e149835;
        locals.var_t2_dn0 = assign97410_e149835_d_n0;
        locals.var_t2_dn2 = assign97410_e149835_d_n2;
        locals.var_t2_dn4 = assign97410_e149835_d_n4;
        locals.var_t2_dn5 = assign97410_e149835_d_n5;
        locals.var_t2_dn6 = assign97410_e149835_d_n6;
        locals.var_t2_dn7 = assign97410_e149835_d_n7;
        locals.var_t2_dn8 = assign97410_e149835_d_n8;
        locals.var_t2_dn9 = assign97410_e149835_d_n9;
        locals.var_t2_dn10 = assign97410_e149835_d_n10;
        locals.var_t2_dn11 = assign97410_e149835_d_n11;
        locals.var_t2_dn14 = assign97410_e149835_d_n14;

        let (assign97420_e149839, assign97420_e149839_d_n0, assign97420_e149839_d_n2, assign97420_e149839_d_n4, assign97420_e149839_d_n5, assign97420_e149839_d_n6, assign97420_e149839_d_n7, assign97420_e149839_d_n8, assign97420_e149839_d_n9, assign97420_e149839_d_n10, assign97420_e149839_d_n11, assign97420_e149839_d_n14,) = {
    if (locals.var_guard2261 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97420_e149839;
        locals.var_t3_dn0 = assign97420_e149839_d_n0;
        locals.var_t3_dn2 = assign97420_e149839_d_n2;
        locals.var_t3_dn4 = assign97420_e149839_d_n4;
        locals.var_t3_dn5 = assign97420_e149839_d_n5;
        locals.var_t3_dn6 = assign97420_e149839_d_n6;
        locals.var_t3_dn7 = assign97420_e149839_d_n7;
        locals.var_t3_dn8 = assign97420_e149839_d_n8;
        locals.var_t3_dn9 = assign97420_e149839_d_n9;
        locals.var_t3_dn10 = assign97420_e149839_d_n10;
        locals.var_t3_dn11 = assign97420_e149839_d_n11;
        locals.var_t3_dn14 = assign97420_e149839_d_n14;

        let assign97430_e149842: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2262 = assign97430_e149842;

        let (assign97440_e149850, assign97440_e149850_d_n0, assign97440_e149850_d_n2, assign97440_e149850_d_n4, assign97440_e149850_d_n5, assign97440_e149850_d_n6, assign97440_e149850_d_n7, assign97440_e149850_d_n8, assign97440_e149850_d_n9, assign97440_e149850_d_n10, assign97440_e149850_d_n11, assign97440_e149850_d_n14,) = {
    if ((locals.var_guard2261 != 0.0) && (locals.var_guard2262 != 0.0)) {
        let assign97440_e149848: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97440_e149848, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9), ((locals.var_vbd_jct_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97440_e149850;
        locals.var_tx_dn0 = assign97440_e149850_d_n0;
        locals.var_tx_dn2 = assign97440_e149850_d_n2;
        locals.var_tx_dn4 = assign97440_e149850_d_n4;
        locals.var_tx_dn5 = assign97440_e149850_d_n5;
        locals.var_tx_dn6 = assign97440_e149850_d_n6;
        locals.var_tx_dn7 = assign97440_e149850_d_n7;
        locals.var_tx_dn8 = assign97440_e149850_d_n8;
        locals.var_tx_dn9 = assign97440_e149850_d_n9;
        locals.var_tx_dn10 = assign97440_e149850_d_n10;
        locals.var_tx_dn11 = assign97440_e149850_d_n11;
        locals.var_tx_dn14 = assign97440_e149850_d_n14;

        let assign97450_e149853: f64 = (-3.0);
        let assign97450_e149855: f64 = (assign97450_e149853 * 34.0);
        let assign97450_e149856: f64 = if locals.var_tx < assign97450_e149855 { 1.0 } else { 0.0 };
        locals.var_guard2263 = assign97450_e149856;

        let (assign97460_e149864, assign97460_e149864_d_n0, assign97460_e149864_d_n2, assign97460_e149864_d_n4, assign97460_e149864_d_n5, assign97460_e149864_d_n6, assign97460_e149864_d_n7, assign97460_e149864_d_n8, assign97460_e149864_d_n9, assign97460_e149864_d_n10, assign97460_e149864_d_n11, assign97460_e149864_d_n14,) = {
    if (((locals.var_guard2261 != 0.0) && (locals.var_guard2262 != 0.0)) && (locals.var_guard2263 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97460_e149864;
        locals.var_t1_dn0 = assign97460_e149864_d_n0;
        locals.var_t1_dn2 = assign97460_e149864_d_n2;
        locals.var_t1_dn4 = assign97460_e149864_d_n4;
        locals.var_t1_dn5 = assign97460_e149864_d_n5;
        locals.var_t1_dn6 = assign97460_e149864_d_n6;
        locals.var_t1_dn7 = assign97460_e149864_d_n7;
        locals.var_t1_dn8 = assign97460_e149864_d_n8;
        locals.var_t1_dn9 = assign97460_e149864_d_n9;
        locals.var_t1_dn10 = assign97460_e149864_d_n10;
        locals.var_t1_dn11 = assign97460_e149864_d_n11;
        locals.var_t1_dn14 = assign97460_e149864_d_n14;

        let (assign97470_e149874, assign97470_e149874_d_n0, assign97470_e149874_d_n2, assign97470_e149874_d_n4, assign97470_e149874_d_n5, assign97470_e149874_d_n6, assign97470_e149874_d_n7, assign97470_e149874_d_n8, assign97470_e149874_d_n9, assign97470_e149874_d_n10, assign97470_e149874_d_n11, assign97470_e149874_d_n14,) = {
    if (((locals.var_guard2261 != 0.0) && (locals.var_guard2262 != 0.0)) && (locals.var_guard2263 == 0.0)) {
        let assign97470_e149872: f64 = (locals.var_tx).exp();
        (assign97470_e149872, (assign97470_e149872 * locals.var_tx_dn0), (assign97470_e149872 * locals.var_tx_dn2), (assign97470_e149872 * locals.var_tx_dn4), (assign97470_e149872 * locals.var_tx_dn5), (assign97470_e149872 * locals.var_tx_dn6), (assign97470_e149872 * locals.var_tx_dn7), (assign97470_e149872 * locals.var_tx_dn8), (assign97470_e149872 * locals.var_tx_dn9), (assign97470_e149872 * locals.var_tx_dn10), (assign97470_e149872 * locals.var_tx_dn11), (assign97470_e149872 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97470_e149874;
        locals.var_t1_dn0 = assign97470_e149874_d_n0;
        locals.var_t1_dn2 = assign97470_e149874_d_n2;
        locals.var_t1_dn4 = assign97470_e149874_d_n4;
        locals.var_t1_dn5 = assign97470_e149874_d_n5;
        locals.var_t1_dn6 = assign97470_e149874_d_n6;
        locals.var_t1_dn7 = assign97470_e149874_d_n7;
        locals.var_t1_dn8 = assign97470_e149874_d_n8;
        locals.var_t1_dn9 = assign97470_e149874_d_n9;
        locals.var_t1_dn10 = assign97470_e149874_d_n10;
        locals.var_t1_dn11 = assign97470_e149874_d_n11;
        locals.var_t1_dn14 = assign97470_e149874_d_n14;

        let (assign97480_e149896, assign97480_e149896_d_n0, assign97480_e149896_d_n2, assign97480_e149896_d_n4, assign97480_e149896_d_n5, assign97480_e149896_d_n6, assign97480_e149896_d_n7, assign97480_e149896_d_n8, assign97480_e149896_d_n9, assign97480_e149896_d_n10, assign97480_e149896_d_n11, assign97480_e149896_d_n14,) = {
    if ((locals.var_guard2261 != 0.0) && (locals.var_guard2262 != 0.0)) {
        let assign97480_e149881: f64 = (locals.var_t1 - 1.0);
        let assign97480_e149882: f64 = (locals.var_isbd_sws * assign97480_e149881);
        let assign97480_e149886: f64 = (locals.var_t2 - 1.0);
        let assign97480_e149887: f64 = (locals.var_t0 * assign97480_e149886);
        let assign97480_e149888: f64 = (assign97480_e149882 + assign97480_e149887);
        let assign97480_e149892: f64 = (locals.var_t3 - 1.0);
        let assign97480_e149893: f64 = (locals.var_uc_cisbkd * assign97480_e149892);
        let assign97480_e149894: f64 = (assign97480_e149888 + assign97480_e149893);
        (assign97480_e149894, ((((locals.var_isbd_sws_dn0 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), ((((locals.var_isbd_sws_dn2 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), ((((locals.var_isbd_sws_dn4 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), ((((locals.var_isbd_sws_dn5 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), ((((locals.var_isbd_sws_dn6 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), ((((locals.var_isbd_sws_dn7 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), ((((locals.var_isbd_sws_dn8 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), ((((locals.var_isbd_sws_dn9 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), ((((locals.var_isbd_sws_dn10 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), ((((locals.var_isbd_sws_dn11 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn11)) + ((locals.var_t0_dn11 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbkd * locals.var_t3_dn11)), ((((locals.var_isbd_sws_dn14 * assign97480_e149881) + (locals.var_isbd_sws * locals.var_t1_dn14)) + ((locals.var_t0_dn14 * assign97480_e149886) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbkd * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibd_sws, locals.var_ibd_sws_dn0, locals.var_ibd_sws_dn2, locals.var_ibd_sws_dn4, locals.var_ibd_sws_dn5, locals.var_ibd_sws_dn6, locals.var_ibd_sws_dn7, locals.var_ibd_sws_dn8, locals.var_ibd_sws_dn9, locals.var_ibd_sws_dn10, locals.var_ibd_sws_dn11, locals.var_ibd_sws_dn14,)
    }
};
        locals.var_ibd_sws = assign97480_e149896;
        locals.var_ibd_sws_dn0 = assign97480_e149896_d_n0;
        locals.var_ibd_sws_dn2 = assign97480_e149896_d_n2;
        locals.var_ibd_sws_dn4 = assign97480_e149896_d_n4;
        locals.var_ibd_sws_dn5 = assign97480_e149896_d_n5;
        locals.var_ibd_sws_dn6 = assign97480_e149896_d_n6;
        locals.var_ibd_sws_dn7 = assign97480_e149896_d_n7;
        locals.var_ibd_sws_dn8 = assign97480_e149896_d_n8;
        locals.var_ibd_sws_dn9 = assign97480_e149896_d_n9;
        locals.var_ibd_sws_dn10 = assign97480_e149896_d_n10;
        locals.var_ibd_sws_dn11 = assign97480_e149896_d_n11;
        locals.var_ibd_sws_dn14 = assign97480_e149896_d_n14;

        let (assign97490_e149903, assign97490_e149903_d_n0, assign97490_e149903_d_n2, assign97490_e149903_d_n4, assign97490_e149903_d_n5, assign97490_e149903_d_n6, assign97490_e149903_d_n7, assign97490_e149903_d_n8, assign97490_e149903_d_n9, assign97490_e149903_d_n10, assign97490_e149903_d_n11, assign97490_e149903_d_n14,) = {
    if ((locals.var_guard2261 != 0.0) && (locals.var_guard2262 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97490_e149903;
        locals.var_t1_dn0 = assign97490_e149903_d_n0;
        locals.var_t1_dn2 = assign97490_e149903_d_n2;
        locals.var_t1_dn4 = assign97490_e149903_d_n4;
        locals.var_t1_dn5 = assign97490_e149903_d_n5;
        locals.var_t1_dn6 = assign97490_e149903_d_n6;
        locals.var_t1_dn7 = assign97490_e149903_d_n7;
        locals.var_t1_dn8 = assign97490_e149903_d_n8;
        locals.var_t1_dn9 = assign97490_e149903_d_n9;
        locals.var_t1_dn10 = assign97490_e149903_d_n10;
        locals.var_t1_dn11 = assign97490_e149903_d_n11;
        locals.var_t1_dn14 = assign97490_e149903_d_n14;

        let (assign97500_e149914, assign97500_e149914_d_n0, assign97500_e149914_d_n2, assign97500_e149914_d_n4, assign97500_e149914_d_n5, assign97500_e149914_d_n6, assign97500_e149914_d_n7, assign97500_e149914_d_n8, assign97500_e149914_d_n9, assign97500_e149914_d_n10, assign97500_e149914_d_n11, assign97500_e149914_d_n14,) = {
    if ((locals.var_guard2261 != 0.0) && (locals.var_guard2262 == 0.0)) {
        let assign97500_e149910: f64 = (locals.var_isbd_sws * locals.var_jd_nvtm_invd);
        let assign97500_e149912: f64 = (assign97500_e149910 * locals.var_t1);
        (assign97500_e149912, ((((locals.var_isbd_sws_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn0)), ((((locals.var_isbd_sws_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn2)), ((((locals.var_isbd_sws_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn4)), ((((locals.var_isbd_sws_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn5)), ((((locals.var_isbd_sws_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn6)), ((((locals.var_isbd_sws_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn7)), ((((locals.var_isbd_sws_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn8)), ((((locals.var_isbd_sws_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn9)), ((((locals.var_isbd_sws_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn10)), ((((locals.var_isbd_sws_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn11)), ((((locals.var_isbd_sws_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97500_e149914;
        locals.var_t4_dn0 = assign97500_e149914_d_n0;
        locals.var_t4_dn2 = assign97500_e149914_d_n2;
        locals.var_t4_dn4 = assign97500_e149914_d_n4;
        locals.var_t4_dn5 = assign97500_e149914_d_n5;
        locals.var_t4_dn6 = assign97500_e149914_d_n6;
        locals.var_t4_dn7 = assign97500_e149914_d_n7;
        locals.var_t4_dn8 = assign97500_e149914_d_n8;
        locals.var_t4_dn9 = assign97500_e149914_d_n9;
        locals.var_t4_dn10 = assign97500_e149914_d_n10;
        locals.var_t4_dn11 = assign97500_e149914_d_n11;
        locals.var_t4_dn14 = assign97500_e149914_d_n14;

        let (assign97510_e149943, assign97510_e149943_d_n0, assign97510_e149943_d_n2, assign97510_e149943_d_n4, assign97510_e149943_d_n5, assign97510_e149943_d_n6, assign97510_e149943_d_n7, assign97510_e149943_d_n8, assign97510_e149943_d_n9, assign97510_e149943_d_n10, assign97510_e149943_d_n11, assign97510_e149943_d_n14,) = {
    if ((locals.var_guard2261 != 0.0) && (locals.var_guard2262 == 0.0)) {
        let assign97510_e149922: f64 = (locals.var_t1 - 1.0);
        let assign97510_e149923: f64 = (locals.var_isbd_sws * assign97510_e149922);
        let assign97510_e149927: f64 = (locals.var_vbd_jct - locals.var_vbdt);
        let assign97510_e149928: f64 = (locals.var_t4 * assign97510_e149927);
        let assign97510_e149929: f64 = (assign97510_e149923 + assign97510_e149928);
        let assign97510_e149933: f64 = (locals.var_t2 - 1.0);
        let assign97510_e149934: f64 = (locals.var_t0 * assign97510_e149933);
        let assign97510_e149935: f64 = (assign97510_e149929 + assign97510_e149934);
        let assign97510_e149939: f64 = (locals.var_t3 - 1.0);
        let assign97510_e149940: f64 = (locals.var_uc_cisbkd * assign97510_e149939);
        let assign97510_e149941: f64 = (assign97510_e149935 + assign97510_e149940);
        (assign97510_e149941, (((((locals.var_isbd_sws_dn0 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign97510_e149927) + (locals.var_t4 * (locals.var_vbd_jct_dn0 - locals.var_vbdt_dn0)))) + ((locals.var_t0_dn0 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), (((((locals.var_isbd_sws_dn2 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign97510_e149927) + (locals.var_t4 * (-locals.var_vbdt_dn2)))) + ((locals.var_t0_dn2 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), (((((locals.var_isbd_sws_dn4 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign97510_e149927) + (locals.var_t4 * (-locals.var_vbdt_dn4)))) + ((locals.var_t0_dn4 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), (((((locals.var_isbd_sws_dn5 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign97510_e149927) + (locals.var_t4 * (-locals.var_vbdt_dn5)))) + ((locals.var_t0_dn5 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), (((((locals.var_isbd_sws_dn6 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign97510_e149927) + (locals.var_t4 * (-locals.var_vbdt_dn6)))) + ((locals.var_t0_dn6 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), (((((locals.var_isbd_sws_dn7 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign97510_e149927) + (locals.var_t4 * (-locals.var_vbdt_dn7)))) + ((locals.var_t0_dn7 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), (((((locals.var_isbd_sws_dn8 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign97510_e149927) + (locals.var_t4 * (-locals.var_vbdt_dn8)))) + ((locals.var_t0_dn8 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), (((((locals.var_isbd_sws_dn9 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign97510_e149927) + (locals.var_t4 * (-locals.var_vbdt_dn9)))) + ((locals.var_t0_dn9 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), (((((locals.var_isbd_sws_dn10 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign97510_e149927) + (locals.var_t4 * (locals.var_vbd_jct_dn10 - locals.var_vbdt_dn10)))) + ((locals.var_t0_dn10 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), (((((locals.var_isbd_sws_dn11 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * assign97510_e149927) + (locals.var_t4 * (-locals.var_vbdt_dn11)))) + ((locals.var_t0_dn11 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbkd * locals.var_t3_dn11)), (((((locals.var_isbd_sws_dn14 * assign97510_e149922) + (locals.var_isbd_sws * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * assign97510_e149927) + (locals.var_t4 * (-locals.var_vbdt_dn14)))) + ((locals.var_t0_dn14 * assign97510_e149933) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbkd * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibd_sws, locals.var_ibd_sws_dn0, locals.var_ibd_sws_dn2, locals.var_ibd_sws_dn4, locals.var_ibd_sws_dn5, locals.var_ibd_sws_dn6, locals.var_ibd_sws_dn7, locals.var_ibd_sws_dn8, locals.var_ibd_sws_dn9, locals.var_ibd_sws_dn10, locals.var_ibd_sws_dn11, locals.var_ibd_sws_dn14,)
    }
};
        locals.var_ibd_sws = assign97510_e149943;
        locals.var_ibd_sws_dn0 = assign97510_e149943_d_n0;
        locals.var_ibd_sws_dn2 = assign97510_e149943_d_n2;
        locals.var_ibd_sws_dn4 = assign97510_e149943_d_n4;
        locals.var_ibd_sws_dn5 = assign97510_e149943_d_n5;
        locals.var_ibd_sws_dn6 = assign97510_e149943_d_n6;
        locals.var_ibd_sws_dn7 = assign97510_e149943_d_n7;
        locals.var_ibd_sws_dn8 = assign97510_e149943_d_n8;
        locals.var_ibd_sws_dn9 = assign97510_e149943_d_n9;
        locals.var_ibd_sws_dn10 = assign97510_e149943_d_n10;
        locals.var_ibd_sws_dn11 = assign97510_e149943_d_n11;
        locals.var_ibd_sws_dn14 = assign97510_e149943_d_n14;

        let (assign97520_e149948, assign97520_e149948_d_n0, assign97520_e149948_d_n2, assign97520_e149948_d_n4, assign97520_e149948_d_n5, assign97520_e149948_d_n6, assign97520_e149948_d_n7, assign97520_e149948_d_n8, assign97520_e149948_d_n9, assign97520_e149948_d_n10, assign97520_e149948_d_n11, assign97520_e149948_d_n14,) = {
    if (locals.var_guard2261 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd_sws, locals.var_ibd_sws_dn0, locals.var_ibd_sws_dn2, locals.var_ibd_sws_dn4, locals.var_ibd_sws_dn5, locals.var_ibd_sws_dn6, locals.var_ibd_sws_dn7, locals.var_ibd_sws_dn8, locals.var_ibd_sws_dn9, locals.var_ibd_sws_dn10, locals.var_ibd_sws_dn11, locals.var_ibd_sws_dn14,)
    }
};
        locals.var_ibd_sws = assign97520_e149948;
        locals.var_ibd_sws_dn0 = assign97520_e149948_d_n0;
        locals.var_ibd_sws_dn2 = assign97520_e149948_d_n2;
        locals.var_ibd_sws_dn4 = assign97520_e149948_d_n4;
        locals.var_ibd_sws_dn5 = assign97520_e149948_d_n5;
        locals.var_ibd_sws_dn6 = assign97520_e149948_d_n6;
        locals.var_ibd_sws_dn7 = assign97520_e149948_d_n7;
        locals.var_ibd_sws_dn8 = assign97520_e149948_d_n8;
        locals.var_ibd_sws_dn9 = assign97520_e149948_d_n9;
        locals.var_ibd_sws_dn10 = assign97520_e149948_d_n10;
        locals.var_ibd_sws_dn11 = assign97520_e149948_d_n11;
        locals.var_ibd_sws_dn14 = assign97520_e149948_d_n14;

        let assign97530_e149951: f64 = (p.p514 * locals.var_isbd2_sws);
        locals.var_t12 = assign97530_e149951;
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

        let assign97540_e149955: f64 = (locals.var_t12 * locals.var_vbd_jct);
        let assign97540_e149956: f64 = (locals.var_ibd_sws + assign97540_e149955);
        locals.var_ibd_sws = assign97540_e149956;
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

        let assign97550_e149959: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2264 = assign97550_e149959;

        let assign97560_e149962: f64 = if locals.var_isbd_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2265 = assign97560_e149962;

        let (assign97570_e149970, assign97570_e149970_d_n0, assign97570_e149970_d_n2, assign97570_e149970_d_n4, assign97570_e149970_d_n5, assign97570_e149970_d_n6, assign97570_e149970_d_n7, assign97570_e149970_d_n8, assign97570_e149970_d_n9, assign97570_e149970_d_n10, assign97570_e149970_d_n11, assign97570_e149970_d_n14,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        let assign97570_e149968: f64 = (locals.var_isbd2_swg * locals.var_t9);
        (assign97570_e149968, ((locals.var_isbd2_swg_dn0 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn0)), ((locals.var_isbd2_swg_dn2 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn2)), ((locals.var_isbd2_swg_dn4 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn4)), ((locals.var_isbd2_swg_dn5 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn5)), ((locals.var_isbd2_swg_dn6 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn6)), ((locals.var_isbd2_swg_dn7 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn7)), ((locals.var_isbd2_swg_dn8 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn8)), ((locals.var_isbd2_swg_dn9 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn9)), ((locals.var_isbd2_swg_dn10 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn10)), ((locals.var_isbd2_swg_dn11 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn11)), ((locals.var_isbd2_swg_dn14 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97570_e149970;
        locals.var_t0_dn0 = assign97570_e149970_d_n0;
        locals.var_t0_dn2 = assign97570_e149970_d_n2;
        locals.var_t0_dn4 = assign97570_e149970_d_n4;
        locals.var_t0_dn5 = assign97570_e149970_d_n5;
        locals.var_t0_dn6 = assign97570_e149970_d_n6;
        locals.var_t0_dn7 = assign97570_e149970_d_n7;
        locals.var_t0_dn8 = assign97570_e149970_d_n8;
        locals.var_t0_dn9 = assign97570_e149970_d_n9;
        locals.var_t0_dn10 = assign97570_e149970_d_n10;
        locals.var_t0_dn11 = assign97570_e149970_d_n11;
        locals.var_t0_dn14 = assign97570_e149970_d_n14;

        let (assign97580_e149979, assign97580_e149979_d_n0, assign97580_e149979_d_n2, assign97580_e149979_d_n4, assign97580_e149979_d_n5, assign97580_e149979_d_n6, assign97580_e149979_d_n7, assign97580_e149979_d_n8, assign97580_e149979_d_n9, assign97580_e149979_d_n10, assign97580_e149979_d_n11, assign97580_e149979_d_n14,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        let assign97580_e149975: f64 = (-locals.var_vbdi_jct);
        let assign97580_e149977: f64 = (assign97580_e149975 * locals.var_t10);
        (assign97580_e149977, (assign97580_e149975 * locals.var_t10_dn0), (assign97580_e149975 * locals.var_t10_dn2), (assign97580_e149975 * locals.var_t10_dn4), (assign97580_e149975 * locals.var_t10_dn5), (((-locals.var_vbdi_jct_dn6) * locals.var_t10) + (assign97580_e149975 * locals.var_t10_dn6)), (assign97580_e149975 * locals.var_t10_dn7), (assign97580_e149975 * locals.var_t10_dn8), (((-locals.var_vbdi_jct_dn9) * locals.var_t10) + (assign97580_e149975 * locals.var_t10_dn9)), (assign97580_e149975 * locals.var_t10_dn10), (assign97580_e149975 * locals.var_t10_dn11), (assign97580_e149975 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97580_e149979;
        locals.var_tx_dn0 = assign97580_e149979_d_n0;
        locals.var_tx_dn2 = assign97580_e149979_d_n2;
        locals.var_tx_dn4 = assign97580_e149979_d_n4;
        locals.var_tx_dn5 = assign97580_e149979_d_n5;
        locals.var_tx_dn6 = assign97580_e149979_d_n6;
        locals.var_tx_dn7 = assign97580_e149979_d_n7;
        locals.var_tx_dn8 = assign97580_e149979_d_n8;
        locals.var_tx_dn9 = assign97580_e149979_d_n9;
        locals.var_tx_dn10 = assign97580_e149979_d_n10;
        locals.var_tx_dn11 = assign97580_e149979_d_n11;
        locals.var_tx_dn14 = assign97580_e149979_d_n14;

        let (assign97590_e149986, assign97590_e149986_d_n0, assign97590_e149986_d_n2, assign97590_e149986_d_n4, assign97590_e149986_d_n5, assign97590_e149986_d_n6, assign97590_e149986_d_n7, assign97590_e149986_d_n8, assign97590_e149986_d_n9, assign97590_e149986_d_n10, assign97590_e149986_d_n11, assign97590_e149986_d_n14,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        let assign97590_e149984: f64 = (locals.var_tx).exp();
        (assign97590_e149984, (assign97590_e149984 * locals.var_tx_dn0), (assign97590_e149984 * locals.var_tx_dn2), (assign97590_e149984 * locals.var_tx_dn4), (assign97590_e149984 * locals.var_tx_dn5), (assign97590_e149984 * locals.var_tx_dn6), (assign97590_e149984 * locals.var_tx_dn7), (assign97590_e149984 * locals.var_tx_dn8), (assign97590_e149984 * locals.var_tx_dn9), (assign97590_e149984 * locals.var_tx_dn10), (assign97590_e149984 * locals.var_tx_dn11), (assign97590_e149984 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97590_e149986;
        locals.var_t2_dn0 = assign97590_e149986_d_n0;
        locals.var_t2_dn2 = assign97590_e149986_d_n2;
        locals.var_t2_dn4 = assign97590_e149986_d_n4;
        locals.var_t2_dn5 = assign97590_e149986_d_n5;
        locals.var_t2_dn6 = assign97590_e149986_d_n6;
        locals.var_t2_dn7 = assign97590_e149986_d_n7;
        locals.var_t2_dn8 = assign97590_e149986_d_n8;
        locals.var_t2_dn9 = assign97590_e149986_d_n9;
        locals.var_t2_dn10 = assign97590_e149986_d_n10;
        locals.var_t2_dn11 = assign97590_e149986_d_n11;
        locals.var_t2_dn14 = assign97590_e149986_d_n14;

        let (assign97600_e149992, assign97600_e149992_d_n0, assign97600_e149992_d_n2, assign97600_e149992_d_n4, assign97600_e149992_d_n5, assign97600_e149992_d_n6, assign97600_e149992_d_n7, assign97600_e149992_d_n8, assign97600_e149992_d_n9, assign97600_e149992_d_n10, assign97600_e149992_d_n11, assign97600_e149992_d_n14,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97600_e149992;
        locals.var_t3_dn0 = assign97600_e149992_d_n0;
        locals.var_t3_dn2 = assign97600_e149992_d_n2;
        locals.var_t3_dn4 = assign97600_e149992_d_n4;
        locals.var_t3_dn5 = assign97600_e149992_d_n5;
        locals.var_t3_dn6 = assign97600_e149992_d_n6;
        locals.var_t3_dn7 = assign97600_e149992_d_n7;
        locals.var_t3_dn8 = assign97600_e149992_d_n8;
        locals.var_t3_dn9 = assign97600_e149992_d_n9;
        locals.var_t3_dn10 = assign97600_e149992_d_n10;
        locals.var_t3_dn11 = assign97600_e149992_d_n11;
        locals.var_t3_dn14 = assign97600_e149992_d_n14;

        let assign97610_e149995: f64 = if locals.var_vbdi_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2266 = assign97610_e149995;

        let (assign97620_e150005, assign97620_e150005_d_n0, assign97620_e150005_d_n2, assign97620_e150005_d_n4, assign97620_e150005_d_n5, assign97620_e150005_d_n6, assign97620_e150005_d_n7, assign97620_e150005_d_n8, assign97620_e150005_d_n9, assign97620_e150005_d_n10, assign97620_e150005_d_n11, assign97620_e150005_d_n14,) = {
    if (((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 != 0.0)) {
        let assign97620_e150003: f64 = (locals.var_vbdi_jct * locals.var_jd_nvtm_invd);
        (assign97620_e150003, (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn0), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn5), ((locals.var_vbdi_jct_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn6)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn8), ((locals.var_vbdi_jct_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn9)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn10), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97620_e150005;
        locals.var_tx_dn0 = assign97620_e150005_d_n0;
        locals.var_tx_dn2 = assign97620_e150005_d_n2;
        locals.var_tx_dn4 = assign97620_e150005_d_n4;
        locals.var_tx_dn5 = assign97620_e150005_d_n5;
        locals.var_tx_dn6 = assign97620_e150005_d_n6;
        locals.var_tx_dn7 = assign97620_e150005_d_n7;
        locals.var_tx_dn8 = assign97620_e150005_d_n8;
        locals.var_tx_dn9 = assign97620_e150005_d_n9;
        locals.var_tx_dn10 = assign97620_e150005_d_n10;
        locals.var_tx_dn11 = assign97620_e150005_d_n11;
        locals.var_tx_dn14 = assign97620_e150005_d_n14;

        let assign97630_e150008: f64 = (-3.0);
        let assign97630_e150010: f64 = (assign97630_e150008 * 34.0);
        let assign97630_e150011: f64 = if locals.var_tx < assign97630_e150010 { 1.0 } else { 0.0 };
        locals.var_guard2267 = assign97630_e150011;

        let (assign97640_e150021, assign97640_e150021_d_n0, assign97640_e150021_d_n2, assign97640_e150021_d_n4, assign97640_e150021_d_n5, assign97640_e150021_d_n6, assign97640_e150021_d_n7, assign97640_e150021_d_n8, assign97640_e150021_d_n9, assign97640_e150021_d_n10, assign97640_e150021_d_n11, assign97640_e150021_d_n14,) = {
    if ((((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 != 0.0)) && (locals.var_guard2267 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97640_e150021;
        locals.var_t1_dn0 = assign97640_e150021_d_n0;
        locals.var_t1_dn2 = assign97640_e150021_d_n2;
        locals.var_t1_dn4 = assign97640_e150021_d_n4;
        locals.var_t1_dn5 = assign97640_e150021_d_n5;
        locals.var_t1_dn6 = assign97640_e150021_d_n6;
        locals.var_t1_dn7 = assign97640_e150021_d_n7;
        locals.var_t1_dn8 = assign97640_e150021_d_n8;
        locals.var_t1_dn9 = assign97640_e150021_d_n9;
        locals.var_t1_dn10 = assign97640_e150021_d_n10;
        locals.var_t1_dn11 = assign97640_e150021_d_n11;
        locals.var_t1_dn14 = assign97640_e150021_d_n14;

        let (assign97650_e150033, assign97650_e150033_d_n0, assign97650_e150033_d_n2, assign97650_e150033_d_n4, assign97650_e150033_d_n5, assign97650_e150033_d_n6, assign97650_e150033_d_n7, assign97650_e150033_d_n8, assign97650_e150033_d_n9, assign97650_e150033_d_n10, assign97650_e150033_d_n11, assign97650_e150033_d_n14,) = {
    if ((((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 != 0.0)) && (locals.var_guard2267 == 0.0)) {
        let assign97650_e150031: f64 = (locals.var_tx).exp();
        (assign97650_e150031, (assign97650_e150031 * locals.var_tx_dn0), (assign97650_e150031 * locals.var_tx_dn2), (assign97650_e150031 * locals.var_tx_dn4), (assign97650_e150031 * locals.var_tx_dn5), (assign97650_e150031 * locals.var_tx_dn6), (assign97650_e150031 * locals.var_tx_dn7), (assign97650_e150031 * locals.var_tx_dn8), (assign97650_e150031 * locals.var_tx_dn9), (assign97650_e150031 * locals.var_tx_dn10), (assign97650_e150031 * locals.var_tx_dn11), (assign97650_e150031 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97650_e150033;
        locals.var_t1_dn0 = assign97650_e150033_d_n0;
        locals.var_t1_dn2 = assign97650_e150033_d_n2;
        locals.var_t1_dn4 = assign97650_e150033_d_n4;
        locals.var_t1_dn5 = assign97650_e150033_d_n5;
        locals.var_t1_dn6 = assign97650_e150033_d_n6;
        locals.var_t1_dn7 = assign97650_e150033_d_n7;
        locals.var_t1_dn8 = assign97650_e150033_d_n8;
        locals.var_t1_dn9 = assign97650_e150033_d_n9;
        locals.var_t1_dn10 = assign97650_e150033_d_n10;
        locals.var_t1_dn11 = assign97650_e150033_d_n11;
        locals.var_t1_dn14 = assign97650_e150033_d_n14;

        let (assign97670_e150066, assign97670_e150066_d_n0, assign97670_e150066_d_n2, assign97670_e150066_d_n4, assign97670_e150066_d_n5, assign97670_e150066_d_n6, assign97670_e150066_d_n7, assign97670_e150066_d_n8, assign97670_e150066_d_n9, assign97670_e150066_d_n10, assign97670_e150066_d_n11, assign97670_e150066_d_n14,) = {
    if (((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97670_e150066;
        locals.var_t1_dn0 = assign97670_e150066_d_n0;
        locals.var_t1_dn2 = assign97670_e150066_d_n2;
        locals.var_t1_dn4 = assign97670_e150066_d_n4;
        locals.var_t1_dn5 = assign97670_e150066_d_n5;
        locals.var_t1_dn6 = assign97670_e150066_d_n6;
        locals.var_t1_dn7 = assign97670_e150066_d_n7;
        locals.var_t1_dn8 = assign97670_e150066_d_n8;
        locals.var_t1_dn9 = assign97670_e150066_d_n9;
        locals.var_t1_dn10 = assign97670_e150066_d_n10;
        locals.var_t1_dn11 = assign97670_e150066_d_n11;
        locals.var_t1_dn14 = assign97670_e150066_d_n14;

    }

    pub(super) fn stamp_transient_block_360(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign97680_e150079, assign97680_e150079_d_n0, assign97680_e150079_d_n2, assign97680_e150079_d_n4, assign97680_e150079_d_n5, assign97680_e150079_d_n6, assign97680_e150079_d_n7, assign97680_e150079_d_n8, assign97680_e150079_d_n9, assign97680_e150079_d_n10, assign97680_e150079_d_n11, assign97680_e150079_d_n14,) = {
    if (((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 == 0.0)) {
        let assign97680_e150075: f64 = (locals.var_isbd_swg * locals.var_jd_nvtm_invd);
        let assign97680_e150077: f64 = (assign97680_e150075 * locals.var_t1);
        (assign97680_e150077, ((((locals.var_isbd_swg_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn0)), ((((locals.var_isbd_swg_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn2)), ((((locals.var_isbd_swg_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn4)), ((((locals.var_isbd_swg_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn5)), ((((locals.var_isbd_swg_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn6)), ((((locals.var_isbd_swg_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn7)), ((((locals.var_isbd_swg_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn8)), ((((locals.var_isbd_swg_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn9)), ((((locals.var_isbd_swg_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn10)), ((((locals.var_isbd_swg_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn11)), ((((locals.var_isbd_swg_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97680_e150079;
        locals.var_t4_dn0 = assign97680_e150079_d_n0;
        locals.var_t4_dn2 = assign97680_e150079_d_n2;
        locals.var_t4_dn4 = assign97680_e150079_d_n4;
        locals.var_t4_dn5 = assign97680_e150079_d_n5;
        locals.var_t4_dn6 = assign97680_e150079_d_n6;
        locals.var_t4_dn7 = assign97680_e150079_d_n7;
        locals.var_t4_dn8 = assign97680_e150079_d_n8;
        locals.var_t4_dn9 = assign97680_e150079_d_n9;
        locals.var_t4_dn10 = assign97680_e150079_d_n10;
        locals.var_t4_dn11 = assign97680_e150079_d_n11;
        locals.var_t4_dn14 = assign97680_e150079_d_n14;

        let (assign97710_e150123, assign97710_e150123_d_n0, assign97710_e150123_d_n2, assign97710_e150123_d_n4, assign97710_e150123_d_n5, assign97710_e150123_d_n6, assign97710_e150123_d_n7, assign97710_e150123_d_n8, assign97710_e150123_d_n9, assign97710_e150123_d_n10, assign97710_e150123_d_n11, assign97710_e150123_d_n14,) = {
    if (locals.var_guard2264 != 0.0) {
        let assign97710_e150121: f64 = (p.p514 * locals.var_isbd2_swg);
        (assign97710_e150121, (p.p514 * locals.var_isbd2_swg_dn0), (p.p514 * locals.var_isbd2_swg_dn2), (p.p514 * locals.var_isbd2_swg_dn4), (p.p514 * locals.var_isbd2_swg_dn5), (p.p514 * locals.var_isbd2_swg_dn6), (p.p514 * locals.var_isbd2_swg_dn7), (p.p514 * locals.var_isbd2_swg_dn8), (p.p514 * locals.var_isbd2_swg_dn9), (p.p514 * locals.var_isbd2_swg_dn10), (p.p514 * locals.var_isbd2_swg_dn11), (p.p514 * locals.var_isbd2_swg_dn14),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign97710_e150123;
        locals.var_t12_dn0 = assign97710_e150123_d_n0;
        locals.var_t12_dn2 = assign97710_e150123_d_n2;
        locals.var_t12_dn4 = assign97710_e150123_d_n4;
        locals.var_t12_dn5 = assign97710_e150123_d_n5;
        locals.var_t12_dn6 = assign97710_e150123_d_n6;
        locals.var_t12_dn7 = assign97710_e150123_d_n7;
        locals.var_t12_dn8 = assign97710_e150123_d_n8;
        locals.var_t12_dn9 = assign97710_e150123_d_n9;
        locals.var_t12_dn10 = assign97710_e150123_d_n10;
        locals.var_t12_dn11 = assign97710_e150123_d_n11;
        locals.var_t12_dn14 = assign97710_e150123_d_n14;

        let assign97740_e150139: f64 = (p.p534 * locals.var_jd_nvtm_invs);
        locals.var_t10 = assign97740_e150139;
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

        let assign97750_e150142: f64 = (p.p533 * locals.var_exptemps);
        locals.var_t9 = assign97750_e150142;
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

        let assign97760_e150145: f64 = if locals.var_isbs_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2268 = assign97760_e150145;

        let (assign97770_e150151, assign97770_e150151_d_n0, assign97770_e150151_d_n2, assign97770_e150151_d_n4, assign97770_e150151_d_n5, assign97770_e150151_d_n6, assign97770_e150151_d_n7, assign97770_e150151_d_n8, assign97770_e150151_d_n9, assign97770_e150151_d_n10, assign97770_e150151_d_n11, assign97770_e150151_d_n14,) = {
    if (locals.var_guard2268 != 0.0) {
        let assign97770_e150149: f64 = (locals.var_isbs2_btm * locals.var_t9);
        (assign97770_e150149, ((locals.var_isbs2_btm_dn0 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn0)), ((locals.var_isbs2_btm_dn2 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn2)), ((locals.var_isbs2_btm_dn4 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn4)), ((locals.var_isbs2_btm_dn5 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn5)), ((locals.var_isbs2_btm_dn6 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn6)), ((locals.var_isbs2_btm_dn7 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn7)), ((locals.var_isbs2_btm_dn8 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn8)), ((locals.var_isbs2_btm_dn9 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn9)), ((locals.var_isbs2_btm_dn10 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn10)), ((locals.var_isbs2_btm_dn11 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn11)), ((locals.var_isbs2_btm_dn14 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97770_e150151;
        locals.var_t0_dn0 = assign97770_e150151_d_n0;
        locals.var_t0_dn2 = assign97770_e150151_d_n2;
        locals.var_t0_dn4 = assign97770_e150151_d_n4;
        locals.var_t0_dn5 = assign97770_e150151_d_n5;
        locals.var_t0_dn6 = assign97770_e150151_d_n6;
        locals.var_t0_dn7 = assign97770_e150151_d_n7;
        locals.var_t0_dn8 = assign97770_e150151_d_n8;
        locals.var_t0_dn9 = assign97770_e150151_d_n9;
        locals.var_t0_dn10 = assign97770_e150151_d_n10;
        locals.var_t0_dn11 = assign97770_e150151_d_n11;
        locals.var_t0_dn14 = assign97770_e150151_d_n14;

        let (assign97780_e150158, assign97780_e150158_d_n0, assign97780_e150158_d_n2, assign97780_e150158_d_n4, assign97780_e150158_d_n5, assign97780_e150158_d_n6, assign97780_e150158_d_n7, assign97780_e150158_d_n8, assign97780_e150158_d_n9, assign97780_e150158_d_n10, assign97780_e150158_d_n11, assign97780_e150158_d_n14,) = {
    if (locals.var_guard2268 != 0.0) {
        let assign97780_e150154: f64 = (-locals.var_vbs_jct);
        let assign97780_e150156: f64 = (assign97780_e150154 * locals.var_t10);
        (assign97780_e150156, (assign97780_e150154 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97780_e150154 * locals.var_t10_dn2)), (assign97780_e150154 * locals.var_t10_dn4), (assign97780_e150154 * locals.var_t10_dn5), (assign97780_e150154 * locals.var_t10_dn6), (assign97780_e150154 * locals.var_t10_dn7), (assign97780_e150154 * locals.var_t10_dn8), (assign97780_e150154 * locals.var_t10_dn9), (assign97780_e150154 * locals.var_t10_dn10), (((-locals.var_vbs_jct_dn11) * locals.var_t10) + (assign97780_e150154 * locals.var_t10_dn11)), (assign97780_e150154 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97780_e150158;
        locals.var_tx_dn0 = assign97780_e150158_d_n0;
        locals.var_tx_dn2 = assign97780_e150158_d_n2;
        locals.var_tx_dn4 = assign97780_e150158_d_n4;
        locals.var_tx_dn5 = assign97780_e150158_d_n5;
        locals.var_tx_dn6 = assign97780_e150158_d_n6;
        locals.var_tx_dn7 = assign97780_e150158_d_n7;
        locals.var_tx_dn8 = assign97780_e150158_d_n8;
        locals.var_tx_dn9 = assign97780_e150158_d_n9;
        locals.var_tx_dn10 = assign97780_e150158_d_n10;
        locals.var_tx_dn11 = assign97780_e150158_d_n11;
        locals.var_tx_dn14 = assign97780_e150158_d_n14;

        let (assign97790_e150163, assign97790_e150163_d_n0, assign97790_e150163_d_n2, assign97790_e150163_d_n4, assign97790_e150163_d_n5, assign97790_e150163_d_n6, assign97790_e150163_d_n7, assign97790_e150163_d_n8, assign97790_e150163_d_n9, assign97790_e150163_d_n10, assign97790_e150163_d_n11, assign97790_e150163_d_n14,) = {
    if (locals.var_guard2268 != 0.0) {
        let assign97790_e150161: f64 = (locals.var_tx).exp();
        (assign97790_e150161, (assign97790_e150161 * locals.var_tx_dn0), (assign97790_e150161 * locals.var_tx_dn2), (assign97790_e150161 * locals.var_tx_dn4), (assign97790_e150161 * locals.var_tx_dn5), (assign97790_e150161 * locals.var_tx_dn6), (assign97790_e150161 * locals.var_tx_dn7), (assign97790_e150161 * locals.var_tx_dn8), (assign97790_e150161 * locals.var_tx_dn9), (assign97790_e150161 * locals.var_tx_dn10), (assign97790_e150161 * locals.var_tx_dn11), (assign97790_e150161 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97790_e150163;
        locals.var_t2_dn0 = assign97790_e150163_d_n0;
        locals.var_t2_dn2 = assign97790_e150163_d_n2;
        locals.var_t2_dn4 = assign97790_e150163_d_n4;
        locals.var_t2_dn5 = assign97790_e150163_d_n5;
        locals.var_t2_dn6 = assign97790_e150163_d_n6;
        locals.var_t2_dn7 = assign97790_e150163_d_n7;
        locals.var_t2_dn8 = assign97790_e150163_d_n8;
        locals.var_t2_dn9 = assign97790_e150163_d_n9;
        locals.var_t2_dn10 = assign97790_e150163_d_n10;
        locals.var_t2_dn11 = assign97790_e150163_d_n11;
        locals.var_t2_dn14 = assign97790_e150163_d_n14;

        let (assign97800_e150167, assign97800_e150167_d_n0, assign97800_e150167_d_n2, assign97800_e150167_d_n4, assign97800_e150167_d_n5, assign97800_e150167_d_n6, assign97800_e150167_d_n7, assign97800_e150167_d_n8, assign97800_e150167_d_n9, assign97800_e150167_d_n10, assign97800_e150167_d_n11, assign97800_e150167_d_n14,) = {
    if (locals.var_guard2268 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97800_e150167;
        locals.var_t3_dn0 = assign97800_e150167_d_n0;
        locals.var_t3_dn2 = assign97800_e150167_d_n2;
        locals.var_t3_dn4 = assign97800_e150167_d_n4;
        locals.var_t3_dn5 = assign97800_e150167_d_n5;
        locals.var_t3_dn6 = assign97800_e150167_d_n6;
        locals.var_t3_dn7 = assign97800_e150167_d_n7;
        locals.var_t3_dn8 = assign97800_e150167_d_n8;
        locals.var_t3_dn9 = assign97800_e150167_d_n9;
        locals.var_t3_dn10 = assign97800_e150167_d_n10;
        locals.var_t3_dn11 = assign97800_e150167_d_n11;
        locals.var_t3_dn14 = assign97800_e150167_d_n14;

        let assign97810_e150170: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2269 = assign97810_e150170;

        let (assign97820_e150178, assign97820_e150178_d_n0, assign97820_e150178_d_n2, assign97820_e150178_d_n4, assign97820_e150178_d_n5, assign97820_e150178_d_n6, assign97820_e150178_d_n7, assign97820_e150178_d_n8, assign97820_e150178_d_n9, assign97820_e150178_d_n10, assign97820_e150178_d_n11, assign97820_e150178_d_n14,) = {
    if ((locals.var_guard2268 != 0.0) && (locals.var_guard2269 != 0.0)) {
        let assign97820_e150176: f64 = (locals.var_vbs_jct * locals.var_jd_nvtm_invs);
        (assign97820_e150176, (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn0), ((locals.var_vbs_jct_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn2)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn7), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn8), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn9), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn10), ((locals.var_vbs_jct_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn11)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97820_e150178;
        locals.var_tx_dn0 = assign97820_e150178_d_n0;
        locals.var_tx_dn2 = assign97820_e150178_d_n2;
        locals.var_tx_dn4 = assign97820_e150178_d_n4;
        locals.var_tx_dn5 = assign97820_e150178_d_n5;
        locals.var_tx_dn6 = assign97820_e150178_d_n6;
        locals.var_tx_dn7 = assign97820_e150178_d_n7;
        locals.var_tx_dn8 = assign97820_e150178_d_n8;
        locals.var_tx_dn9 = assign97820_e150178_d_n9;
        locals.var_tx_dn10 = assign97820_e150178_d_n10;
        locals.var_tx_dn11 = assign97820_e150178_d_n11;
        locals.var_tx_dn14 = assign97820_e150178_d_n14;

        let assign97830_e150181: f64 = (-3.0);
        let assign97830_e150183: f64 = (assign97830_e150181 * 34.0);
        let assign97830_e150184: f64 = if locals.var_tx < assign97830_e150183 { 1.0 } else { 0.0 };
        locals.var_guard2270 = assign97830_e150184;

        let (assign97840_e150192, assign97840_e150192_d_n0, assign97840_e150192_d_n2, assign97840_e150192_d_n4, assign97840_e150192_d_n5, assign97840_e150192_d_n6, assign97840_e150192_d_n7, assign97840_e150192_d_n8, assign97840_e150192_d_n9, assign97840_e150192_d_n10, assign97840_e150192_d_n11, assign97840_e150192_d_n14,) = {
    if (((locals.var_guard2268 != 0.0) && (locals.var_guard2269 != 0.0)) && (locals.var_guard2270 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97840_e150192;
        locals.var_t1_dn0 = assign97840_e150192_d_n0;
        locals.var_t1_dn2 = assign97840_e150192_d_n2;
        locals.var_t1_dn4 = assign97840_e150192_d_n4;
        locals.var_t1_dn5 = assign97840_e150192_d_n5;
        locals.var_t1_dn6 = assign97840_e150192_d_n6;
        locals.var_t1_dn7 = assign97840_e150192_d_n7;
        locals.var_t1_dn8 = assign97840_e150192_d_n8;
        locals.var_t1_dn9 = assign97840_e150192_d_n9;
        locals.var_t1_dn10 = assign97840_e150192_d_n10;
        locals.var_t1_dn11 = assign97840_e150192_d_n11;
        locals.var_t1_dn14 = assign97840_e150192_d_n14;

        let (assign97850_e150202, assign97850_e150202_d_n0, assign97850_e150202_d_n2, assign97850_e150202_d_n4, assign97850_e150202_d_n5, assign97850_e150202_d_n6, assign97850_e150202_d_n7, assign97850_e150202_d_n8, assign97850_e150202_d_n9, assign97850_e150202_d_n10, assign97850_e150202_d_n11, assign97850_e150202_d_n14,) = {
    if (((locals.var_guard2268 != 0.0) && (locals.var_guard2269 != 0.0)) && (locals.var_guard2270 == 0.0)) {
        let assign97850_e150200: f64 = (locals.var_tx).exp();
        (assign97850_e150200, (assign97850_e150200 * locals.var_tx_dn0), (assign97850_e150200 * locals.var_tx_dn2), (assign97850_e150200 * locals.var_tx_dn4), (assign97850_e150200 * locals.var_tx_dn5), (assign97850_e150200 * locals.var_tx_dn6), (assign97850_e150200 * locals.var_tx_dn7), (assign97850_e150200 * locals.var_tx_dn8), (assign97850_e150200 * locals.var_tx_dn9), (assign97850_e150200 * locals.var_tx_dn10), (assign97850_e150200 * locals.var_tx_dn11), (assign97850_e150200 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97850_e150202;
        locals.var_t1_dn0 = assign97850_e150202_d_n0;
        locals.var_t1_dn2 = assign97850_e150202_d_n2;
        locals.var_t1_dn4 = assign97850_e150202_d_n4;
        locals.var_t1_dn5 = assign97850_e150202_d_n5;
        locals.var_t1_dn6 = assign97850_e150202_d_n6;
        locals.var_t1_dn7 = assign97850_e150202_d_n7;
        locals.var_t1_dn8 = assign97850_e150202_d_n8;
        locals.var_t1_dn9 = assign97850_e150202_d_n9;
        locals.var_t1_dn10 = assign97850_e150202_d_n10;
        locals.var_t1_dn11 = assign97850_e150202_d_n11;
        locals.var_t1_dn14 = assign97850_e150202_d_n14;

        let (assign97860_e150224, assign97860_e150224_d_n0, assign97860_e150224_d_n2, assign97860_e150224_d_n4, assign97860_e150224_d_n5, assign97860_e150224_d_n6, assign97860_e150224_d_n7, assign97860_e150224_d_n8, assign97860_e150224_d_n9, assign97860_e150224_d_n10, assign97860_e150224_d_n11, assign97860_e150224_d_n14,) = {
    if ((locals.var_guard2268 != 0.0) && (locals.var_guard2269 != 0.0)) {
        let assign97860_e150209: f64 = (locals.var_t1 - 1.0);
        let assign97860_e150210: f64 = (locals.var_isbs_btm * assign97860_e150209);
        let assign97860_e150214: f64 = (locals.var_t2 - 1.0);
        let assign97860_e150215: f64 = (locals.var_t0 * assign97860_e150214);
        let assign97860_e150216: f64 = (assign97860_e150210 + assign97860_e150215);
        let assign97860_e150220: f64 = (locals.var_t3 - 1.0);
        let assign97860_e150221: f64 = (locals.var_uc_cisbks * assign97860_e150220);
        let assign97860_e150222: f64 = (assign97860_e150216 + assign97860_e150221);
        (assign97860_e150222, ((((locals.var_isbs_btm_dn0 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), ((((locals.var_isbs_btm_dn2 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), ((((locals.var_isbs_btm_dn4 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), ((((locals.var_isbs_btm_dn5 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), ((((locals.var_isbs_btm_dn6 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), ((((locals.var_isbs_btm_dn7 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), ((((locals.var_isbs_btm_dn8 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), ((((locals.var_isbs_btm_dn9 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), ((((locals.var_isbs_btm_dn10 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), ((((locals.var_isbs_btm_dn11 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn11)) + ((locals.var_t0_dn11 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), ((((locals.var_isbs_btm_dn14 * assign97860_e150209) + (locals.var_isbs_btm * locals.var_t1_dn14)) + ((locals.var_t0_dn14 * assign97860_e150214) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_btm, locals.var_ibs_btm_dn0, locals.var_ibs_btm_dn2, locals.var_ibs_btm_dn4, locals.var_ibs_btm_dn5, locals.var_ibs_btm_dn6, locals.var_ibs_btm_dn7, locals.var_ibs_btm_dn8, locals.var_ibs_btm_dn9, locals.var_ibs_btm_dn10, locals.var_ibs_btm_dn11, locals.var_ibs_btm_dn14,)
    }
};
        locals.var_ibs_btm = assign97860_e150224;
        locals.var_ibs_btm_dn0 = assign97860_e150224_d_n0;
        locals.var_ibs_btm_dn2 = assign97860_e150224_d_n2;
        locals.var_ibs_btm_dn4 = assign97860_e150224_d_n4;
        locals.var_ibs_btm_dn5 = assign97860_e150224_d_n5;
        locals.var_ibs_btm_dn6 = assign97860_e150224_d_n6;
        locals.var_ibs_btm_dn7 = assign97860_e150224_d_n7;
        locals.var_ibs_btm_dn8 = assign97860_e150224_d_n8;
        locals.var_ibs_btm_dn9 = assign97860_e150224_d_n9;
        locals.var_ibs_btm_dn10 = assign97860_e150224_d_n10;
        locals.var_ibs_btm_dn11 = assign97860_e150224_d_n11;
        locals.var_ibs_btm_dn14 = assign97860_e150224_d_n14;

        let (assign97870_e150231, assign97870_e150231_d_n0, assign97870_e150231_d_n2, assign97870_e150231_d_n4, assign97870_e150231_d_n5, assign97870_e150231_d_n6, assign97870_e150231_d_n7, assign97870_e150231_d_n8, assign97870_e150231_d_n9, assign97870_e150231_d_n10, assign97870_e150231_d_n11, assign97870_e150231_d_n14,) = {
    if ((locals.var_guard2268 != 0.0) && (locals.var_guard2269 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97870_e150231;
        locals.var_t1_dn0 = assign97870_e150231_d_n0;
        locals.var_t1_dn2 = assign97870_e150231_d_n2;
        locals.var_t1_dn4 = assign97870_e150231_d_n4;
        locals.var_t1_dn5 = assign97870_e150231_d_n5;
        locals.var_t1_dn6 = assign97870_e150231_d_n6;
        locals.var_t1_dn7 = assign97870_e150231_d_n7;
        locals.var_t1_dn8 = assign97870_e150231_d_n8;
        locals.var_t1_dn9 = assign97870_e150231_d_n9;
        locals.var_t1_dn10 = assign97870_e150231_d_n10;
        locals.var_t1_dn11 = assign97870_e150231_d_n11;
        locals.var_t1_dn14 = assign97870_e150231_d_n14;

        let (assign97880_e150242, assign97880_e150242_d_n0, assign97880_e150242_d_n2, assign97880_e150242_d_n4, assign97880_e150242_d_n5, assign97880_e150242_d_n6, assign97880_e150242_d_n7, assign97880_e150242_d_n8, assign97880_e150242_d_n9, assign97880_e150242_d_n10, assign97880_e150242_d_n11, assign97880_e150242_d_n14,) = {
    if ((locals.var_guard2268 != 0.0) && (locals.var_guard2269 == 0.0)) {
        let assign97880_e150238: f64 = (locals.var_isbs_btm * locals.var_jd_nvtm_invs);
        let assign97880_e150240: f64 = (assign97880_e150238 * locals.var_t1);
        (assign97880_e150240, ((((locals.var_isbs_btm_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn0)), ((((locals.var_isbs_btm_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn2)), ((((locals.var_isbs_btm_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn4)), ((((locals.var_isbs_btm_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn5)), ((((locals.var_isbs_btm_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn6)), ((((locals.var_isbs_btm_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn7)), ((((locals.var_isbs_btm_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn8)), ((((locals.var_isbs_btm_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn9)), ((((locals.var_isbs_btm_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn10)), ((((locals.var_isbs_btm_dn11 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn11)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn11)), ((((locals.var_isbs_btm_dn14 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn14)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97880_e150242;
        locals.var_t4_dn0 = assign97880_e150242_d_n0;
        locals.var_t4_dn2 = assign97880_e150242_d_n2;
        locals.var_t4_dn4 = assign97880_e150242_d_n4;
        locals.var_t4_dn5 = assign97880_e150242_d_n5;
        locals.var_t4_dn6 = assign97880_e150242_d_n6;
        locals.var_t4_dn7 = assign97880_e150242_d_n7;
        locals.var_t4_dn8 = assign97880_e150242_d_n8;
        locals.var_t4_dn9 = assign97880_e150242_d_n9;
        locals.var_t4_dn10 = assign97880_e150242_d_n10;
        locals.var_t4_dn11 = assign97880_e150242_d_n11;
        locals.var_t4_dn14 = assign97880_e150242_d_n14;

        let (assign97890_e150271, assign97890_e150271_d_n0, assign97890_e150271_d_n2, assign97890_e150271_d_n4, assign97890_e150271_d_n5, assign97890_e150271_d_n6, assign97890_e150271_d_n7, assign97890_e150271_d_n8, assign97890_e150271_d_n9, assign97890_e150271_d_n10, assign97890_e150271_d_n11, assign97890_e150271_d_n14,) = {
    if ((locals.var_guard2268 != 0.0) && (locals.var_guard2269 == 0.0)) {
        let assign97890_e150250: f64 = (locals.var_t1 - 1.0);
        let assign97890_e150251: f64 = (locals.var_isbs_btm * assign97890_e150250);
        let assign97890_e150255: f64 = (locals.var_vbs_jct - locals.var_vbst);
        let assign97890_e150256: f64 = (locals.var_t4 * assign97890_e150255);
        let assign97890_e150257: f64 = (assign97890_e150251 + assign97890_e150256);
        let assign97890_e150261: f64 = (locals.var_t2 - 1.0);
        let assign97890_e150262: f64 = (locals.var_t0 * assign97890_e150261);
        let assign97890_e150263: f64 = (assign97890_e150257 + assign97890_e150262);
        let assign97890_e150267: f64 = (locals.var_t3 - 1.0);
        let assign97890_e150268: f64 = (locals.var_uc_cisbks * assign97890_e150267);
        let assign97890_e150269: f64 = (assign97890_e150263 + assign97890_e150268);
        (assign97890_e150269, (((((locals.var_isbs_btm_dn0 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign97890_e150255) + (locals.var_t4 * (-locals.var_vbst_dn0)))) + ((locals.var_t0_dn0 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), (((((locals.var_isbs_btm_dn2 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign97890_e150255) + (locals.var_t4 * (locals.var_vbs_jct_dn2 - locals.var_vbst_dn2)))) + ((locals.var_t0_dn2 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), (((((locals.var_isbs_btm_dn4 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign97890_e150255) + (locals.var_t4 * (-locals.var_vbst_dn4)))) + ((locals.var_t0_dn4 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), (((((locals.var_isbs_btm_dn5 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign97890_e150255) + (locals.var_t4 * (-locals.var_vbst_dn5)))) + ((locals.var_t0_dn5 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), (((((locals.var_isbs_btm_dn6 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign97890_e150255) + (locals.var_t4 * (-locals.var_vbst_dn6)))) + ((locals.var_t0_dn6 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), (((((locals.var_isbs_btm_dn7 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign97890_e150255) + (locals.var_t4 * (-locals.var_vbst_dn7)))) + ((locals.var_t0_dn7 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), (((((locals.var_isbs_btm_dn8 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign97890_e150255) + (locals.var_t4 * (-locals.var_vbst_dn8)))) + ((locals.var_t0_dn8 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), (((((locals.var_isbs_btm_dn9 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign97890_e150255) + (locals.var_t4 * (-locals.var_vbst_dn9)))) + ((locals.var_t0_dn9 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), (((((locals.var_isbs_btm_dn10 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign97890_e150255) + (locals.var_t4 * (-locals.var_vbst_dn10)))) + ((locals.var_t0_dn10 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), (((((locals.var_isbs_btm_dn11 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * assign97890_e150255) + (locals.var_t4 * (locals.var_vbs_jct_dn11 - locals.var_vbst_dn11)))) + ((locals.var_t0_dn11 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), (((((locals.var_isbs_btm_dn14 * assign97890_e150250) + (locals.var_isbs_btm * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * assign97890_e150255) + (locals.var_t4 * (-locals.var_vbst_dn14)))) + ((locals.var_t0_dn14 * assign97890_e150261) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_btm, locals.var_ibs_btm_dn0, locals.var_ibs_btm_dn2, locals.var_ibs_btm_dn4, locals.var_ibs_btm_dn5, locals.var_ibs_btm_dn6, locals.var_ibs_btm_dn7, locals.var_ibs_btm_dn8, locals.var_ibs_btm_dn9, locals.var_ibs_btm_dn10, locals.var_ibs_btm_dn11, locals.var_ibs_btm_dn14,)
    }
};
        locals.var_ibs_btm = assign97890_e150271;
        locals.var_ibs_btm_dn0 = assign97890_e150271_d_n0;
        locals.var_ibs_btm_dn2 = assign97890_e150271_d_n2;
        locals.var_ibs_btm_dn4 = assign97890_e150271_d_n4;
        locals.var_ibs_btm_dn5 = assign97890_e150271_d_n5;
        locals.var_ibs_btm_dn6 = assign97890_e150271_d_n6;
        locals.var_ibs_btm_dn7 = assign97890_e150271_d_n7;
        locals.var_ibs_btm_dn8 = assign97890_e150271_d_n8;
        locals.var_ibs_btm_dn9 = assign97890_e150271_d_n9;
        locals.var_ibs_btm_dn10 = assign97890_e150271_d_n10;
        locals.var_ibs_btm_dn11 = assign97890_e150271_d_n11;
        locals.var_ibs_btm_dn14 = assign97890_e150271_d_n14;

        let (assign97900_e150276, assign97900_e150276_d_n0, assign97900_e150276_d_n2, assign97900_e150276_d_n4, assign97900_e150276_d_n5, assign97900_e150276_d_n6, assign97900_e150276_d_n7, assign97900_e150276_d_n8, assign97900_e150276_d_n9, assign97900_e150276_d_n10, assign97900_e150276_d_n11, assign97900_e150276_d_n14,) = {
    if (locals.var_guard2268 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_btm, locals.var_ibs_btm_dn0, locals.var_ibs_btm_dn2, locals.var_ibs_btm_dn4, locals.var_ibs_btm_dn5, locals.var_ibs_btm_dn6, locals.var_ibs_btm_dn7, locals.var_ibs_btm_dn8, locals.var_ibs_btm_dn9, locals.var_ibs_btm_dn10, locals.var_ibs_btm_dn11, locals.var_ibs_btm_dn14,)
    }
};
        locals.var_ibs_btm = assign97900_e150276;
        locals.var_ibs_btm_dn0 = assign97900_e150276_d_n0;
        locals.var_ibs_btm_dn2 = assign97900_e150276_d_n2;
        locals.var_ibs_btm_dn4 = assign97900_e150276_d_n4;
        locals.var_ibs_btm_dn5 = assign97900_e150276_d_n5;
        locals.var_ibs_btm_dn6 = assign97900_e150276_d_n6;
        locals.var_ibs_btm_dn7 = assign97900_e150276_d_n7;
        locals.var_ibs_btm_dn8 = assign97900_e150276_d_n8;
        locals.var_ibs_btm_dn9 = assign97900_e150276_d_n9;
        locals.var_ibs_btm_dn10 = assign97900_e150276_d_n10;
        locals.var_ibs_btm_dn11 = assign97900_e150276_d_n11;
        locals.var_ibs_btm_dn14 = assign97900_e150276_d_n14;

        let assign97910_e150279: f64 = (p.p537 * locals.var_isbs2_btm);
        locals.var_t12 = assign97910_e150279;
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

        let assign97920_e150283: f64 = (locals.var_t12 * locals.var_vbs_jct);
        let assign97920_e150284: f64 = (locals.var_ibs_btm + assign97920_e150283);
        locals.var_ibs_btm = assign97920_e150284;
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

        let assign97930_e150287: f64 = if locals.var_isbs_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2271 = assign97930_e150287;

        let (assign97940_e150293, assign97940_e150293_d_n0, assign97940_e150293_d_n2, assign97940_e150293_d_n4, assign97940_e150293_d_n5, assign97940_e150293_d_n6, assign97940_e150293_d_n7, assign97940_e150293_d_n8, assign97940_e150293_d_n9, assign97940_e150293_d_n10, assign97940_e150293_d_n11, assign97940_e150293_d_n14,) = {
    if (locals.var_guard2271 != 0.0) {
        let assign97940_e150291: f64 = (locals.var_isbs2_sws * locals.var_t9);
        (assign97940_e150291, ((locals.var_isbs2_sws_dn0 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn0)), ((locals.var_isbs2_sws_dn2 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn2)), ((locals.var_isbs2_sws_dn4 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn4)), ((locals.var_isbs2_sws_dn5 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn5)), ((locals.var_isbs2_sws_dn6 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn6)), ((locals.var_isbs2_sws_dn7 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn7)), ((locals.var_isbs2_sws_dn8 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn8)), ((locals.var_isbs2_sws_dn9 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn9)), ((locals.var_isbs2_sws_dn10 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn10)), ((locals.var_isbs2_sws_dn11 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn11)), ((locals.var_isbs2_sws_dn14 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97940_e150293;
        locals.var_t0_dn0 = assign97940_e150293_d_n0;
        locals.var_t0_dn2 = assign97940_e150293_d_n2;
        locals.var_t0_dn4 = assign97940_e150293_d_n4;
        locals.var_t0_dn5 = assign97940_e150293_d_n5;
        locals.var_t0_dn6 = assign97940_e150293_d_n6;
        locals.var_t0_dn7 = assign97940_e150293_d_n7;
        locals.var_t0_dn8 = assign97940_e150293_d_n8;
        locals.var_t0_dn9 = assign97940_e150293_d_n9;
        locals.var_t0_dn10 = assign97940_e150293_d_n10;
        locals.var_t0_dn11 = assign97940_e150293_d_n11;
        locals.var_t0_dn14 = assign97940_e150293_d_n14;

        let (assign97950_e150300, assign97950_e150300_d_n0, assign97950_e150300_d_n2, assign97950_e150300_d_n4, assign97950_e150300_d_n5, assign97950_e150300_d_n6, assign97950_e150300_d_n7, assign97950_e150300_d_n8, assign97950_e150300_d_n9, assign97950_e150300_d_n10, assign97950_e150300_d_n11, assign97950_e150300_d_n14,) = {
    if (locals.var_guard2271 != 0.0) {
        let assign97950_e150296: f64 = (-locals.var_vbs_jct);
        let assign97950_e150298: f64 = (assign97950_e150296 * locals.var_t10);
        (assign97950_e150298, (assign97950_e150296 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97950_e150296 * locals.var_t10_dn2)), (assign97950_e150296 * locals.var_t10_dn4), (assign97950_e150296 * locals.var_t10_dn5), (assign97950_e150296 * locals.var_t10_dn6), (assign97950_e150296 * locals.var_t10_dn7), (assign97950_e150296 * locals.var_t10_dn8), (assign97950_e150296 * locals.var_t10_dn9), (assign97950_e150296 * locals.var_t10_dn10), (((-locals.var_vbs_jct_dn11) * locals.var_t10) + (assign97950_e150296 * locals.var_t10_dn11)), (assign97950_e150296 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97950_e150300;
        locals.var_tx_dn0 = assign97950_e150300_d_n0;
        locals.var_tx_dn2 = assign97950_e150300_d_n2;
        locals.var_tx_dn4 = assign97950_e150300_d_n4;
        locals.var_tx_dn5 = assign97950_e150300_d_n5;
        locals.var_tx_dn6 = assign97950_e150300_d_n6;
        locals.var_tx_dn7 = assign97950_e150300_d_n7;
        locals.var_tx_dn8 = assign97950_e150300_d_n8;
        locals.var_tx_dn9 = assign97950_e150300_d_n9;
        locals.var_tx_dn10 = assign97950_e150300_d_n10;
        locals.var_tx_dn11 = assign97950_e150300_d_n11;
        locals.var_tx_dn14 = assign97950_e150300_d_n14;

        let (assign97960_e150305, assign97960_e150305_d_n0, assign97960_e150305_d_n2, assign97960_e150305_d_n4, assign97960_e150305_d_n5, assign97960_e150305_d_n6, assign97960_e150305_d_n7, assign97960_e150305_d_n8, assign97960_e150305_d_n9, assign97960_e150305_d_n10, assign97960_e150305_d_n11, assign97960_e150305_d_n14,) = {
    if (locals.var_guard2271 != 0.0) {
        let assign97960_e150303: f64 = (locals.var_tx).exp();
        (assign97960_e150303, (assign97960_e150303 * locals.var_tx_dn0), (assign97960_e150303 * locals.var_tx_dn2), (assign97960_e150303 * locals.var_tx_dn4), (assign97960_e150303 * locals.var_tx_dn5), (assign97960_e150303 * locals.var_tx_dn6), (assign97960_e150303 * locals.var_tx_dn7), (assign97960_e150303 * locals.var_tx_dn8), (assign97960_e150303 * locals.var_tx_dn9), (assign97960_e150303 * locals.var_tx_dn10), (assign97960_e150303 * locals.var_tx_dn11), (assign97960_e150303 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97960_e150305;
        locals.var_t2_dn0 = assign97960_e150305_d_n0;
        locals.var_t2_dn2 = assign97960_e150305_d_n2;
        locals.var_t2_dn4 = assign97960_e150305_d_n4;
        locals.var_t2_dn5 = assign97960_e150305_d_n5;
        locals.var_t2_dn6 = assign97960_e150305_d_n6;
        locals.var_t2_dn7 = assign97960_e150305_d_n7;
        locals.var_t2_dn8 = assign97960_e150305_d_n8;
        locals.var_t2_dn9 = assign97960_e150305_d_n9;
        locals.var_t2_dn10 = assign97960_e150305_d_n10;
        locals.var_t2_dn11 = assign97960_e150305_d_n11;
        locals.var_t2_dn14 = assign97960_e150305_d_n14;

        let (assign97970_e150309, assign97970_e150309_d_n0, assign97970_e150309_d_n2, assign97970_e150309_d_n4, assign97970_e150309_d_n5, assign97970_e150309_d_n6, assign97970_e150309_d_n7, assign97970_e150309_d_n8, assign97970_e150309_d_n9, assign97970_e150309_d_n10, assign97970_e150309_d_n11, assign97970_e150309_d_n14,) = {
    if (locals.var_guard2271 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97970_e150309;
        locals.var_t3_dn0 = assign97970_e150309_d_n0;
        locals.var_t3_dn2 = assign97970_e150309_d_n2;
        locals.var_t3_dn4 = assign97970_e150309_d_n4;
        locals.var_t3_dn5 = assign97970_e150309_d_n5;
        locals.var_t3_dn6 = assign97970_e150309_d_n6;
        locals.var_t3_dn7 = assign97970_e150309_d_n7;
        locals.var_t3_dn8 = assign97970_e150309_d_n8;
        locals.var_t3_dn9 = assign97970_e150309_d_n9;
        locals.var_t3_dn10 = assign97970_e150309_d_n10;
        locals.var_t3_dn11 = assign97970_e150309_d_n11;
        locals.var_t3_dn14 = assign97970_e150309_d_n14;

        let assign97980_e150312: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2272 = assign97980_e150312;

        let (assign97990_e150320, assign97990_e150320_d_n0, assign97990_e150320_d_n2, assign97990_e150320_d_n4, assign97990_e150320_d_n5, assign97990_e150320_d_n6, assign97990_e150320_d_n7, assign97990_e150320_d_n8, assign97990_e150320_d_n9, assign97990_e150320_d_n10, assign97990_e150320_d_n11, assign97990_e150320_d_n14,) = {
    if ((locals.var_guard2271 != 0.0) && (locals.var_guard2272 != 0.0)) {
        let assign97990_e150318: f64 = (locals.var_vbs_jct * locals.var_jd_nvtm_invs);
        (assign97990_e150318, (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn0), ((locals.var_vbs_jct_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn2)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn7), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn8), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn9), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn10), ((locals.var_vbs_jct_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn11)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97990_e150320;
        locals.var_tx_dn0 = assign97990_e150320_d_n0;
        locals.var_tx_dn2 = assign97990_e150320_d_n2;
        locals.var_tx_dn4 = assign97990_e150320_d_n4;
        locals.var_tx_dn5 = assign97990_e150320_d_n5;
        locals.var_tx_dn6 = assign97990_e150320_d_n6;
        locals.var_tx_dn7 = assign97990_e150320_d_n7;
        locals.var_tx_dn8 = assign97990_e150320_d_n8;
        locals.var_tx_dn9 = assign97990_e150320_d_n9;
        locals.var_tx_dn10 = assign97990_e150320_d_n10;
        locals.var_tx_dn11 = assign97990_e150320_d_n11;
        locals.var_tx_dn14 = assign97990_e150320_d_n14;

        let assign98000_e150323: f64 = (-3.0);
        let assign98000_e150325: f64 = (assign98000_e150323 * 34.0);
        let assign98000_e150326: f64 = if locals.var_tx < assign98000_e150325 { 1.0 } else { 0.0 };
        locals.var_guard2273 = assign98000_e150326;

    }

    pub(super) fn stamp_transient_block_361(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98010_e150334, assign98010_e150334_d_n0, assign98010_e150334_d_n2, assign98010_e150334_d_n4, assign98010_e150334_d_n5, assign98010_e150334_d_n6, assign98010_e150334_d_n7, assign98010_e150334_d_n8, assign98010_e150334_d_n9, assign98010_e150334_d_n10, assign98010_e150334_d_n11, assign98010_e150334_d_n14,) = {
    if (((locals.var_guard2271 != 0.0) && (locals.var_guard2272 != 0.0)) && (locals.var_guard2273 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98010_e150334;
        locals.var_t1_dn0 = assign98010_e150334_d_n0;
        locals.var_t1_dn2 = assign98010_e150334_d_n2;
        locals.var_t1_dn4 = assign98010_e150334_d_n4;
        locals.var_t1_dn5 = assign98010_e150334_d_n5;
        locals.var_t1_dn6 = assign98010_e150334_d_n6;
        locals.var_t1_dn7 = assign98010_e150334_d_n7;
        locals.var_t1_dn8 = assign98010_e150334_d_n8;
        locals.var_t1_dn9 = assign98010_e150334_d_n9;
        locals.var_t1_dn10 = assign98010_e150334_d_n10;
        locals.var_t1_dn11 = assign98010_e150334_d_n11;
        locals.var_t1_dn14 = assign98010_e150334_d_n14;

        let (assign98020_e150344, assign98020_e150344_d_n0, assign98020_e150344_d_n2, assign98020_e150344_d_n4, assign98020_e150344_d_n5, assign98020_e150344_d_n6, assign98020_e150344_d_n7, assign98020_e150344_d_n8, assign98020_e150344_d_n9, assign98020_e150344_d_n10, assign98020_e150344_d_n11, assign98020_e150344_d_n14,) = {
    if (((locals.var_guard2271 != 0.0) && (locals.var_guard2272 != 0.0)) && (locals.var_guard2273 == 0.0)) {
        let assign98020_e150342: f64 = (locals.var_tx).exp();
        (assign98020_e150342, (assign98020_e150342 * locals.var_tx_dn0), (assign98020_e150342 * locals.var_tx_dn2), (assign98020_e150342 * locals.var_tx_dn4), (assign98020_e150342 * locals.var_tx_dn5), (assign98020_e150342 * locals.var_tx_dn6), (assign98020_e150342 * locals.var_tx_dn7), (assign98020_e150342 * locals.var_tx_dn8), (assign98020_e150342 * locals.var_tx_dn9), (assign98020_e150342 * locals.var_tx_dn10), (assign98020_e150342 * locals.var_tx_dn11), (assign98020_e150342 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98020_e150344;
        locals.var_t1_dn0 = assign98020_e150344_d_n0;
        locals.var_t1_dn2 = assign98020_e150344_d_n2;
        locals.var_t1_dn4 = assign98020_e150344_d_n4;
        locals.var_t1_dn5 = assign98020_e150344_d_n5;
        locals.var_t1_dn6 = assign98020_e150344_d_n6;
        locals.var_t1_dn7 = assign98020_e150344_d_n7;
        locals.var_t1_dn8 = assign98020_e150344_d_n8;
        locals.var_t1_dn9 = assign98020_e150344_d_n9;
        locals.var_t1_dn10 = assign98020_e150344_d_n10;
        locals.var_t1_dn11 = assign98020_e150344_d_n11;
        locals.var_t1_dn14 = assign98020_e150344_d_n14;

        let (assign98030_e150366, assign98030_e150366_d_n0, assign98030_e150366_d_n2, assign98030_e150366_d_n4, assign98030_e150366_d_n5, assign98030_e150366_d_n6, assign98030_e150366_d_n7, assign98030_e150366_d_n8, assign98030_e150366_d_n9, assign98030_e150366_d_n10, assign98030_e150366_d_n11, assign98030_e150366_d_n14,) = {
    if ((locals.var_guard2271 != 0.0) && (locals.var_guard2272 != 0.0)) {
        let assign98030_e150351: f64 = (locals.var_t1 - 1.0);
        let assign98030_e150352: f64 = (locals.var_isbs_sws * assign98030_e150351);
        let assign98030_e150356: f64 = (locals.var_t2 - 1.0);
        let assign98030_e150357: f64 = (locals.var_t0 * assign98030_e150356);
        let assign98030_e150358: f64 = (assign98030_e150352 + assign98030_e150357);
        let assign98030_e150362: f64 = (locals.var_t3 - 1.0);
        let assign98030_e150363: f64 = (locals.var_uc_cisbks * assign98030_e150362);
        let assign98030_e150364: f64 = (assign98030_e150358 + assign98030_e150363);
        (assign98030_e150364, ((((locals.var_isbs_sws_dn0 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), ((((locals.var_isbs_sws_dn2 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), ((((locals.var_isbs_sws_dn4 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), ((((locals.var_isbs_sws_dn5 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), ((((locals.var_isbs_sws_dn6 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), ((((locals.var_isbs_sws_dn7 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), ((((locals.var_isbs_sws_dn8 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), ((((locals.var_isbs_sws_dn9 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), ((((locals.var_isbs_sws_dn10 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), ((((locals.var_isbs_sws_dn11 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn11)) + ((locals.var_t0_dn11 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), ((((locals.var_isbs_sws_dn14 * assign98030_e150351) + (locals.var_isbs_sws * locals.var_t1_dn14)) + ((locals.var_t0_dn14 * assign98030_e150356) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_sws, locals.var_ibs_sws_dn0, locals.var_ibs_sws_dn2, locals.var_ibs_sws_dn4, locals.var_ibs_sws_dn5, locals.var_ibs_sws_dn6, locals.var_ibs_sws_dn7, locals.var_ibs_sws_dn8, locals.var_ibs_sws_dn9, locals.var_ibs_sws_dn10, locals.var_ibs_sws_dn11, locals.var_ibs_sws_dn14,)
    }
};
        locals.var_ibs_sws = assign98030_e150366;
        locals.var_ibs_sws_dn0 = assign98030_e150366_d_n0;
        locals.var_ibs_sws_dn2 = assign98030_e150366_d_n2;
        locals.var_ibs_sws_dn4 = assign98030_e150366_d_n4;
        locals.var_ibs_sws_dn5 = assign98030_e150366_d_n5;
        locals.var_ibs_sws_dn6 = assign98030_e150366_d_n6;
        locals.var_ibs_sws_dn7 = assign98030_e150366_d_n7;
        locals.var_ibs_sws_dn8 = assign98030_e150366_d_n8;
        locals.var_ibs_sws_dn9 = assign98030_e150366_d_n9;
        locals.var_ibs_sws_dn10 = assign98030_e150366_d_n10;
        locals.var_ibs_sws_dn11 = assign98030_e150366_d_n11;
        locals.var_ibs_sws_dn14 = assign98030_e150366_d_n14;

        let (assign98040_e150373, assign98040_e150373_d_n0, assign98040_e150373_d_n2, assign98040_e150373_d_n4, assign98040_e150373_d_n5, assign98040_e150373_d_n6, assign98040_e150373_d_n7, assign98040_e150373_d_n8, assign98040_e150373_d_n9, assign98040_e150373_d_n10, assign98040_e150373_d_n11, assign98040_e150373_d_n14,) = {
    if ((locals.var_guard2271 != 0.0) && (locals.var_guard2272 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98040_e150373;
        locals.var_t1_dn0 = assign98040_e150373_d_n0;
        locals.var_t1_dn2 = assign98040_e150373_d_n2;
        locals.var_t1_dn4 = assign98040_e150373_d_n4;
        locals.var_t1_dn5 = assign98040_e150373_d_n5;
        locals.var_t1_dn6 = assign98040_e150373_d_n6;
        locals.var_t1_dn7 = assign98040_e150373_d_n7;
        locals.var_t1_dn8 = assign98040_e150373_d_n8;
        locals.var_t1_dn9 = assign98040_e150373_d_n9;
        locals.var_t1_dn10 = assign98040_e150373_d_n10;
        locals.var_t1_dn11 = assign98040_e150373_d_n11;
        locals.var_t1_dn14 = assign98040_e150373_d_n14;

        let (assign98050_e150384, assign98050_e150384_d_n0, assign98050_e150384_d_n2, assign98050_e150384_d_n4, assign98050_e150384_d_n5, assign98050_e150384_d_n6, assign98050_e150384_d_n7, assign98050_e150384_d_n8, assign98050_e150384_d_n9, assign98050_e150384_d_n10, assign98050_e150384_d_n11, assign98050_e150384_d_n14,) = {
    if ((locals.var_guard2271 != 0.0) && (locals.var_guard2272 == 0.0)) {
        let assign98050_e150380: f64 = (locals.var_isbs_sws * locals.var_jd_nvtm_invs);
        let assign98050_e150382: f64 = (assign98050_e150380 * locals.var_t1);
        (assign98050_e150382, ((((locals.var_isbs_sws_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn0)), ((((locals.var_isbs_sws_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn2)), ((((locals.var_isbs_sws_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn4)), ((((locals.var_isbs_sws_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn5)), ((((locals.var_isbs_sws_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn6)), ((((locals.var_isbs_sws_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn7)), ((((locals.var_isbs_sws_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn8)), ((((locals.var_isbs_sws_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn9)), ((((locals.var_isbs_sws_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn10)), ((((locals.var_isbs_sws_dn11 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn11)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn11)), ((((locals.var_isbs_sws_dn14 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn14)) * locals.var_t1) + (assign98050_e150380 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign98050_e150384;
        locals.var_t4_dn0 = assign98050_e150384_d_n0;
        locals.var_t4_dn2 = assign98050_e150384_d_n2;
        locals.var_t4_dn4 = assign98050_e150384_d_n4;
        locals.var_t4_dn5 = assign98050_e150384_d_n5;
        locals.var_t4_dn6 = assign98050_e150384_d_n6;
        locals.var_t4_dn7 = assign98050_e150384_d_n7;
        locals.var_t4_dn8 = assign98050_e150384_d_n8;
        locals.var_t4_dn9 = assign98050_e150384_d_n9;
        locals.var_t4_dn10 = assign98050_e150384_d_n10;
        locals.var_t4_dn11 = assign98050_e150384_d_n11;
        locals.var_t4_dn14 = assign98050_e150384_d_n14;

        let (assign98060_e150413, assign98060_e150413_d_n0, assign98060_e150413_d_n2, assign98060_e150413_d_n4, assign98060_e150413_d_n5, assign98060_e150413_d_n6, assign98060_e150413_d_n7, assign98060_e150413_d_n8, assign98060_e150413_d_n9, assign98060_e150413_d_n10, assign98060_e150413_d_n11, assign98060_e150413_d_n14,) = {
    if ((locals.var_guard2271 != 0.0) && (locals.var_guard2272 == 0.0)) {
        let assign98060_e150392: f64 = (locals.var_t1 - 1.0);
        let assign98060_e150393: f64 = (locals.var_isbs_sws * assign98060_e150392);
        let assign98060_e150397: f64 = (locals.var_vbs_jct - locals.var_vbst);
        let assign98060_e150398: f64 = (locals.var_t4 * assign98060_e150397);
        let assign98060_e150399: f64 = (assign98060_e150393 + assign98060_e150398);
        let assign98060_e150403: f64 = (locals.var_t2 - 1.0);
        let assign98060_e150404: f64 = (locals.var_t0 * assign98060_e150403);
        let assign98060_e150405: f64 = (assign98060_e150399 + assign98060_e150404);
        let assign98060_e150409: f64 = (locals.var_t3 - 1.0);
        let assign98060_e150410: f64 = (locals.var_uc_cisbks * assign98060_e150409);
        let assign98060_e150411: f64 = (assign98060_e150405 + assign98060_e150410);
        (assign98060_e150411, (((((locals.var_isbs_sws_dn0 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign98060_e150397) + (locals.var_t4 * (-locals.var_vbst_dn0)))) + ((locals.var_t0_dn0 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), (((((locals.var_isbs_sws_dn2 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign98060_e150397) + (locals.var_t4 * (locals.var_vbs_jct_dn2 - locals.var_vbst_dn2)))) + ((locals.var_t0_dn2 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), (((((locals.var_isbs_sws_dn4 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign98060_e150397) + (locals.var_t4 * (-locals.var_vbst_dn4)))) + ((locals.var_t0_dn4 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), (((((locals.var_isbs_sws_dn5 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign98060_e150397) + (locals.var_t4 * (-locals.var_vbst_dn5)))) + ((locals.var_t0_dn5 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), (((((locals.var_isbs_sws_dn6 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign98060_e150397) + (locals.var_t4 * (-locals.var_vbst_dn6)))) + ((locals.var_t0_dn6 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), (((((locals.var_isbs_sws_dn7 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign98060_e150397) + (locals.var_t4 * (-locals.var_vbst_dn7)))) + ((locals.var_t0_dn7 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), (((((locals.var_isbs_sws_dn8 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign98060_e150397) + (locals.var_t4 * (-locals.var_vbst_dn8)))) + ((locals.var_t0_dn8 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), (((((locals.var_isbs_sws_dn9 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign98060_e150397) + (locals.var_t4 * (-locals.var_vbst_dn9)))) + ((locals.var_t0_dn9 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), (((((locals.var_isbs_sws_dn10 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign98060_e150397) + (locals.var_t4 * (-locals.var_vbst_dn10)))) + ((locals.var_t0_dn10 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), (((((locals.var_isbs_sws_dn11 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * assign98060_e150397) + (locals.var_t4 * (locals.var_vbs_jct_dn11 - locals.var_vbst_dn11)))) + ((locals.var_t0_dn11 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), (((((locals.var_isbs_sws_dn14 * assign98060_e150392) + (locals.var_isbs_sws * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * assign98060_e150397) + (locals.var_t4 * (-locals.var_vbst_dn14)))) + ((locals.var_t0_dn14 * assign98060_e150403) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_sws, locals.var_ibs_sws_dn0, locals.var_ibs_sws_dn2, locals.var_ibs_sws_dn4, locals.var_ibs_sws_dn5, locals.var_ibs_sws_dn6, locals.var_ibs_sws_dn7, locals.var_ibs_sws_dn8, locals.var_ibs_sws_dn9, locals.var_ibs_sws_dn10, locals.var_ibs_sws_dn11, locals.var_ibs_sws_dn14,)
    }
};
        locals.var_ibs_sws = assign98060_e150413;
        locals.var_ibs_sws_dn0 = assign98060_e150413_d_n0;
        locals.var_ibs_sws_dn2 = assign98060_e150413_d_n2;
        locals.var_ibs_sws_dn4 = assign98060_e150413_d_n4;
        locals.var_ibs_sws_dn5 = assign98060_e150413_d_n5;
        locals.var_ibs_sws_dn6 = assign98060_e150413_d_n6;
        locals.var_ibs_sws_dn7 = assign98060_e150413_d_n7;
        locals.var_ibs_sws_dn8 = assign98060_e150413_d_n8;
        locals.var_ibs_sws_dn9 = assign98060_e150413_d_n9;
        locals.var_ibs_sws_dn10 = assign98060_e150413_d_n10;
        locals.var_ibs_sws_dn11 = assign98060_e150413_d_n11;
        locals.var_ibs_sws_dn14 = assign98060_e150413_d_n14;

        let (assign98070_e150418, assign98070_e150418_d_n0, assign98070_e150418_d_n2, assign98070_e150418_d_n4, assign98070_e150418_d_n5, assign98070_e150418_d_n6, assign98070_e150418_d_n7, assign98070_e150418_d_n8, assign98070_e150418_d_n9, assign98070_e150418_d_n10, assign98070_e150418_d_n11, assign98070_e150418_d_n14,) = {
    if (locals.var_guard2271 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_sws, locals.var_ibs_sws_dn0, locals.var_ibs_sws_dn2, locals.var_ibs_sws_dn4, locals.var_ibs_sws_dn5, locals.var_ibs_sws_dn6, locals.var_ibs_sws_dn7, locals.var_ibs_sws_dn8, locals.var_ibs_sws_dn9, locals.var_ibs_sws_dn10, locals.var_ibs_sws_dn11, locals.var_ibs_sws_dn14,)
    }
};
        locals.var_ibs_sws = assign98070_e150418;
        locals.var_ibs_sws_dn0 = assign98070_e150418_d_n0;
        locals.var_ibs_sws_dn2 = assign98070_e150418_d_n2;
        locals.var_ibs_sws_dn4 = assign98070_e150418_d_n4;
        locals.var_ibs_sws_dn5 = assign98070_e150418_d_n5;
        locals.var_ibs_sws_dn6 = assign98070_e150418_d_n6;
        locals.var_ibs_sws_dn7 = assign98070_e150418_d_n7;
        locals.var_ibs_sws_dn8 = assign98070_e150418_d_n8;
        locals.var_ibs_sws_dn9 = assign98070_e150418_d_n9;
        locals.var_ibs_sws_dn10 = assign98070_e150418_d_n10;
        locals.var_ibs_sws_dn11 = assign98070_e150418_d_n11;
        locals.var_ibs_sws_dn14 = assign98070_e150418_d_n14;

        let assign98080_e150421: f64 = (p.p537 * locals.var_isbs2_sws);
        locals.var_t12 = assign98080_e150421;
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

        let assign98090_e150425: f64 = (locals.var_t12 * locals.var_vbs_jct);
        let assign98090_e150426: f64 = (locals.var_ibs_sws + assign98090_e150425);
        locals.var_ibs_sws = assign98090_e150426;
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

        let assign98100_e150429: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2274 = assign98100_e150429;

        let assign98110_e150432: f64 = if locals.var_isbs_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2275 = assign98110_e150432;

        let (assign98120_e150440, assign98120_e150440_d_n0, assign98120_e150440_d_n2, assign98120_e150440_d_n4, assign98120_e150440_d_n5, assign98120_e150440_d_n6, assign98120_e150440_d_n7, assign98120_e150440_d_n8, assign98120_e150440_d_n9, assign98120_e150440_d_n10, assign98120_e150440_d_n11, assign98120_e150440_d_n14,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) {
        let assign98120_e150438: f64 = (locals.var_isbs2_swg * locals.var_t9);
        (assign98120_e150438, ((locals.var_isbs2_swg_dn0 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn0)), ((locals.var_isbs2_swg_dn2 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn2)), ((locals.var_isbs2_swg_dn4 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn4)), ((locals.var_isbs2_swg_dn5 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn5)), ((locals.var_isbs2_swg_dn6 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn6)), ((locals.var_isbs2_swg_dn7 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn7)), ((locals.var_isbs2_swg_dn8 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn8)), ((locals.var_isbs2_swg_dn9 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn9)), ((locals.var_isbs2_swg_dn10 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn10)), ((locals.var_isbs2_swg_dn11 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn11)), ((locals.var_isbs2_swg_dn14 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign98120_e150440;
        locals.var_t0_dn0 = assign98120_e150440_d_n0;
        locals.var_t0_dn2 = assign98120_e150440_d_n2;
        locals.var_t0_dn4 = assign98120_e150440_d_n4;
        locals.var_t0_dn5 = assign98120_e150440_d_n5;
        locals.var_t0_dn6 = assign98120_e150440_d_n6;
        locals.var_t0_dn7 = assign98120_e150440_d_n7;
        locals.var_t0_dn8 = assign98120_e150440_d_n8;
        locals.var_t0_dn9 = assign98120_e150440_d_n9;
        locals.var_t0_dn10 = assign98120_e150440_d_n10;
        locals.var_t0_dn11 = assign98120_e150440_d_n11;
        locals.var_t0_dn14 = assign98120_e150440_d_n14;

        let (assign98130_e150449, assign98130_e150449_d_n0, assign98130_e150449_d_n2, assign98130_e150449_d_n4, assign98130_e150449_d_n5, assign98130_e150449_d_n6, assign98130_e150449_d_n7, assign98130_e150449_d_n8, assign98130_e150449_d_n9, assign98130_e150449_d_n10, assign98130_e150449_d_n11, assign98130_e150449_d_n14,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) {
        let assign98130_e150445: f64 = (-locals.var_vbsi_jct);
        let assign98130_e150447: f64 = (assign98130_e150445 * locals.var_t10);
        (assign98130_e150447, (assign98130_e150445 * locals.var_t10_dn0), (assign98130_e150445 * locals.var_t10_dn2), (assign98130_e150445 * locals.var_t10_dn4), (assign98130_e150445 * locals.var_t10_dn5), (assign98130_e150445 * locals.var_t10_dn6), (assign98130_e150445 * locals.var_t10_dn7), (((-locals.var_vbsi_jct_dn8) * locals.var_t10) + (assign98130_e150445 * locals.var_t10_dn8)), (((-locals.var_vbsi_jct_dn9) * locals.var_t10) + (assign98130_e150445 * locals.var_t10_dn9)), (assign98130_e150445 * locals.var_t10_dn10), (assign98130_e150445 * locals.var_t10_dn11), (assign98130_e150445 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign98130_e150449;
        locals.var_tx_dn0 = assign98130_e150449_d_n0;
        locals.var_tx_dn2 = assign98130_e150449_d_n2;
        locals.var_tx_dn4 = assign98130_e150449_d_n4;
        locals.var_tx_dn5 = assign98130_e150449_d_n5;
        locals.var_tx_dn6 = assign98130_e150449_d_n6;
        locals.var_tx_dn7 = assign98130_e150449_d_n7;
        locals.var_tx_dn8 = assign98130_e150449_d_n8;
        locals.var_tx_dn9 = assign98130_e150449_d_n9;
        locals.var_tx_dn10 = assign98130_e150449_d_n10;
        locals.var_tx_dn11 = assign98130_e150449_d_n11;
        locals.var_tx_dn14 = assign98130_e150449_d_n14;

        let (assign98140_e150456, assign98140_e150456_d_n0, assign98140_e150456_d_n2, assign98140_e150456_d_n4, assign98140_e150456_d_n5, assign98140_e150456_d_n6, assign98140_e150456_d_n7, assign98140_e150456_d_n8, assign98140_e150456_d_n9, assign98140_e150456_d_n10, assign98140_e150456_d_n11, assign98140_e150456_d_n14,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) {
        let assign98140_e150454: f64 = (locals.var_tx).exp();
        (assign98140_e150454, (assign98140_e150454 * locals.var_tx_dn0), (assign98140_e150454 * locals.var_tx_dn2), (assign98140_e150454 * locals.var_tx_dn4), (assign98140_e150454 * locals.var_tx_dn5), (assign98140_e150454 * locals.var_tx_dn6), (assign98140_e150454 * locals.var_tx_dn7), (assign98140_e150454 * locals.var_tx_dn8), (assign98140_e150454 * locals.var_tx_dn9), (assign98140_e150454 * locals.var_tx_dn10), (assign98140_e150454 * locals.var_tx_dn11), (assign98140_e150454 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98140_e150456;
        locals.var_t2_dn0 = assign98140_e150456_d_n0;
        locals.var_t2_dn2 = assign98140_e150456_d_n2;
        locals.var_t2_dn4 = assign98140_e150456_d_n4;
        locals.var_t2_dn5 = assign98140_e150456_d_n5;
        locals.var_t2_dn6 = assign98140_e150456_d_n6;
        locals.var_t2_dn7 = assign98140_e150456_d_n7;
        locals.var_t2_dn8 = assign98140_e150456_d_n8;
        locals.var_t2_dn9 = assign98140_e150456_d_n9;
        locals.var_t2_dn10 = assign98140_e150456_d_n10;
        locals.var_t2_dn11 = assign98140_e150456_d_n11;
        locals.var_t2_dn14 = assign98140_e150456_d_n14;

        let (assign98150_e150462, assign98150_e150462_d_n0, assign98150_e150462_d_n2, assign98150_e150462_d_n4, assign98150_e150462_d_n5, assign98150_e150462_d_n6, assign98150_e150462_d_n7, assign98150_e150462_d_n8, assign98150_e150462_d_n9, assign98150_e150462_d_n10, assign98150_e150462_d_n11, assign98150_e150462_d_n14,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign98150_e150462;
        locals.var_t3_dn0 = assign98150_e150462_d_n0;
        locals.var_t3_dn2 = assign98150_e150462_d_n2;
        locals.var_t3_dn4 = assign98150_e150462_d_n4;
        locals.var_t3_dn5 = assign98150_e150462_d_n5;
        locals.var_t3_dn6 = assign98150_e150462_d_n6;
        locals.var_t3_dn7 = assign98150_e150462_d_n7;
        locals.var_t3_dn8 = assign98150_e150462_d_n8;
        locals.var_t3_dn9 = assign98150_e150462_d_n9;
        locals.var_t3_dn10 = assign98150_e150462_d_n10;
        locals.var_t3_dn11 = assign98150_e150462_d_n11;
        locals.var_t3_dn14 = assign98150_e150462_d_n14;

        let assign98160_e150465: f64 = if locals.var_vbsi_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2276 = assign98160_e150465;

        let (assign98170_e150475, assign98170_e150475_d_n0, assign98170_e150475_d_n2, assign98170_e150475_d_n4, assign98170_e150475_d_n5, assign98170_e150475_d_n6, assign98170_e150475_d_n7, assign98170_e150475_d_n8, assign98170_e150475_d_n9, assign98170_e150475_d_n10, assign98170_e150475_d_n11, assign98170_e150475_d_n14,) = {
    if (((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 != 0.0)) {
        let assign98170_e150473: f64 = (locals.var_vbsi_jct * locals.var_jd_nvtm_invs);
        (assign98170_e150473, (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn0), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn2), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn7), ((locals.var_vbsi_jct_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn8)), ((locals.var_vbsi_jct_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn9)), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn10), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn11), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign98170_e150475;
        locals.var_tx_dn0 = assign98170_e150475_d_n0;
        locals.var_tx_dn2 = assign98170_e150475_d_n2;
        locals.var_tx_dn4 = assign98170_e150475_d_n4;
        locals.var_tx_dn5 = assign98170_e150475_d_n5;
        locals.var_tx_dn6 = assign98170_e150475_d_n6;
        locals.var_tx_dn7 = assign98170_e150475_d_n7;
        locals.var_tx_dn8 = assign98170_e150475_d_n8;
        locals.var_tx_dn9 = assign98170_e150475_d_n9;
        locals.var_tx_dn10 = assign98170_e150475_d_n10;
        locals.var_tx_dn11 = assign98170_e150475_d_n11;
        locals.var_tx_dn14 = assign98170_e150475_d_n14;

        let assign98180_e150478: f64 = (-3.0);
        let assign98180_e150480: f64 = (assign98180_e150478 * 34.0);
        let assign98180_e150481: f64 = if locals.var_tx < assign98180_e150480 { 1.0 } else { 0.0 };
        locals.var_guard2277 = assign98180_e150481;

        let (assign98190_e150491, assign98190_e150491_d_n0, assign98190_e150491_d_n2, assign98190_e150491_d_n4, assign98190_e150491_d_n5, assign98190_e150491_d_n6, assign98190_e150491_d_n7, assign98190_e150491_d_n8, assign98190_e150491_d_n9, assign98190_e150491_d_n10, assign98190_e150491_d_n11, assign98190_e150491_d_n14,) = {
    if ((((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 != 0.0)) && (locals.var_guard2277 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98190_e150491;
        locals.var_t1_dn0 = assign98190_e150491_d_n0;
        locals.var_t1_dn2 = assign98190_e150491_d_n2;
        locals.var_t1_dn4 = assign98190_e150491_d_n4;
        locals.var_t1_dn5 = assign98190_e150491_d_n5;
        locals.var_t1_dn6 = assign98190_e150491_d_n6;
        locals.var_t1_dn7 = assign98190_e150491_d_n7;
        locals.var_t1_dn8 = assign98190_e150491_d_n8;
        locals.var_t1_dn9 = assign98190_e150491_d_n9;
        locals.var_t1_dn10 = assign98190_e150491_d_n10;
        locals.var_t1_dn11 = assign98190_e150491_d_n11;
        locals.var_t1_dn14 = assign98190_e150491_d_n14;

        let (assign98200_e150503, assign98200_e150503_d_n0, assign98200_e150503_d_n2, assign98200_e150503_d_n4, assign98200_e150503_d_n5, assign98200_e150503_d_n6, assign98200_e150503_d_n7, assign98200_e150503_d_n8, assign98200_e150503_d_n9, assign98200_e150503_d_n10, assign98200_e150503_d_n11, assign98200_e150503_d_n14,) = {
    if ((((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 != 0.0)) && (locals.var_guard2277 == 0.0)) {
        let assign98200_e150501: f64 = (locals.var_tx).exp();
        (assign98200_e150501, (assign98200_e150501 * locals.var_tx_dn0), (assign98200_e150501 * locals.var_tx_dn2), (assign98200_e150501 * locals.var_tx_dn4), (assign98200_e150501 * locals.var_tx_dn5), (assign98200_e150501 * locals.var_tx_dn6), (assign98200_e150501 * locals.var_tx_dn7), (assign98200_e150501 * locals.var_tx_dn8), (assign98200_e150501 * locals.var_tx_dn9), (assign98200_e150501 * locals.var_tx_dn10), (assign98200_e150501 * locals.var_tx_dn11), (assign98200_e150501 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98200_e150503;
        locals.var_t1_dn0 = assign98200_e150503_d_n0;
        locals.var_t1_dn2 = assign98200_e150503_d_n2;
        locals.var_t1_dn4 = assign98200_e150503_d_n4;
        locals.var_t1_dn5 = assign98200_e150503_d_n5;
        locals.var_t1_dn6 = assign98200_e150503_d_n6;
        locals.var_t1_dn7 = assign98200_e150503_d_n7;
        locals.var_t1_dn8 = assign98200_e150503_d_n8;
        locals.var_t1_dn9 = assign98200_e150503_d_n9;
        locals.var_t1_dn10 = assign98200_e150503_d_n10;
        locals.var_t1_dn11 = assign98200_e150503_d_n11;
        locals.var_t1_dn14 = assign98200_e150503_d_n14;

        let (assign98210_e150527, assign98210_e150527_d_n0, assign98210_e150527_d_n2, assign98210_e150527_d_n4, assign98210_e150527_d_n5, assign98210_e150527_d_n6, assign98210_e150527_d_n7, assign98210_e150527_d_n8, assign98210_e150527_d_n9, assign98210_e150527_d_n10, assign98210_e150527_d_n11, assign98210_e150527_d_n14,) = {
    if (((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 != 0.0)) {
        let assign98210_e150512: f64 = (locals.var_t1 - 1.0);
        let assign98210_e150513: f64 = (locals.var_isbs_swg * assign98210_e150512);
        let assign98210_e150517: f64 = (locals.var_t2 - 1.0);
        let assign98210_e150518: f64 = (locals.var_t0 * assign98210_e150517);
        let assign98210_e150519: f64 = (assign98210_e150513 + assign98210_e150518);
        let assign98210_e150523: f64 = (locals.var_t3 - 1.0);
        let assign98210_e150524: f64 = (locals.var_uc_cisbks * assign98210_e150523);
        let assign98210_e150525: f64 = (assign98210_e150519 + assign98210_e150524);
        (assign98210_e150525, ((((locals.var_isbs_swg_dn0 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), ((((locals.var_isbs_swg_dn2 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), ((((locals.var_isbs_swg_dn4 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), ((((locals.var_isbs_swg_dn5 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), ((((locals.var_isbs_swg_dn6 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), ((((locals.var_isbs_swg_dn7 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), ((((locals.var_isbs_swg_dn8 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), ((((locals.var_isbs_swg_dn9 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), ((((locals.var_isbs_swg_dn10 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), ((((locals.var_isbs_swg_dn11 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn11)) + ((locals.var_t0_dn11 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), ((((locals.var_isbs_swg_dn14 * assign98210_e150512) + (locals.var_isbs_swg * locals.var_t1_dn14)) + ((locals.var_t0_dn14 * assign98210_e150517) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn11, locals.var_ibs_swg_dn14,)
    }
};
        locals.var_ibs_swg = assign98210_e150527;
        locals.var_ibs_swg_dn0 = assign98210_e150527_d_n0;
        locals.var_ibs_swg_dn2 = assign98210_e150527_d_n2;
        locals.var_ibs_swg_dn4 = assign98210_e150527_d_n4;
        locals.var_ibs_swg_dn5 = assign98210_e150527_d_n5;
        locals.var_ibs_swg_dn6 = assign98210_e150527_d_n6;
        locals.var_ibs_swg_dn7 = assign98210_e150527_d_n7;
        locals.var_ibs_swg_dn8 = assign98210_e150527_d_n8;
        locals.var_ibs_swg_dn9 = assign98210_e150527_d_n9;
        locals.var_ibs_swg_dn10 = assign98210_e150527_d_n10;
        locals.var_ibs_swg_dn11 = assign98210_e150527_d_n11;
        locals.var_ibs_swg_dn14 = assign98210_e150527_d_n14;

        let (assign98220_e150536, assign98220_e150536_d_n0, assign98220_e150536_d_n2, assign98220_e150536_d_n4, assign98220_e150536_d_n5, assign98220_e150536_d_n6, assign98220_e150536_d_n7, assign98220_e150536_d_n8, assign98220_e150536_d_n9, assign98220_e150536_d_n10, assign98220_e150536_d_n11, assign98220_e150536_d_n14,) = {
    if (((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98220_e150536;
        locals.var_t1_dn0 = assign98220_e150536_d_n0;
        locals.var_t1_dn2 = assign98220_e150536_d_n2;
        locals.var_t1_dn4 = assign98220_e150536_d_n4;
        locals.var_t1_dn5 = assign98220_e150536_d_n5;
        locals.var_t1_dn6 = assign98220_e150536_d_n6;
        locals.var_t1_dn7 = assign98220_e150536_d_n7;
        locals.var_t1_dn8 = assign98220_e150536_d_n8;
        locals.var_t1_dn9 = assign98220_e150536_d_n9;
        locals.var_t1_dn10 = assign98220_e150536_d_n10;
        locals.var_t1_dn11 = assign98220_e150536_d_n11;
        locals.var_t1_dn14 = assign98220_e150536_d_n14;

        let (assign98230_e150549, assign98230_e150549_d_n0, assign98230_e150549_d_n2, assign98230_e150549_d_n4, assign98230_e150549_d_n5, assign98230_e150549_d_n6, assign98230_e150549_d_n7, assign98230_e150549_d_n8, assign98230_e150549_d_n9, assign98230_e150549_d_n10, assign98230_e150549_d_n11, assign98230_e150549_d_n14,) = {
    if (((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 == 0.0)) {
        let assign98230_e150545: f64 = (locals.var_isbs_swg * locals.var_jd_nvtm_invs);
        let assign98230_e150547: f64 = (assign98230_e150545 * locals.var_t1);
        (assign98230_e150547, ((((locals.var_isbs_swg_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn0)), ((((locals.var_isbs_swg_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn2)), ((((locals.var_isbs_swg_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn4)), ((((locals.var_isbs_swg_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn5)), ((((locals.var_isbs_swg_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn6)), ((((locals.var_isbs_swg_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn7)), ((((locals.var_isbs_swg_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn8)), ((((locals.var_isbs_swg_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn9)), ((((locals.var_isbs_swg_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn10)), ((((locals.var_isbs_swg_dn11 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn11)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn11)), ((((locals.var_isbs_swg_dn14 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn14)) * locals.var_t1) + (assign98230_e150545 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign98230_e150549;
        locals.var_t4_dn0 = assign98230_e150549_d_n0;
        locals.var_t4_dn2 = assign98230_e150549_d_n2;
        locals.var_t4_dn4 = assign98230_e150549_d_n4;
        locals.var_t4_dn5 = assign98230_e150549_d_n5;
        locals.var_t4_dn6 = assign98230_e150549_d_n6;
        locals.var_t4_dn7 = assign98230_e150549_d_n7;
        locals.var_t4_dn8 = assign98230_e150549_d_n8;
        locals.var_t4_dn9 = assign98230_e150549_d_n9;
        locals.var_t4_dn10 = assign98230_e150549_d_n10;
        locals.var_t4_dn11 = assign98230_e150549_d_n11;
        locals.var_t4_dn14 = assign98230_e150549_d_n14;

        let (assign98240_e150580, assign98240_e150580_d_n0, assign98240_e150580_d_n2, assign98240_e150580_d_n4, assign98240_e150580_d_n5, assign98240_e150580_d_n6, assign98240_e150580_d_n7, assign98240_e150580_d_n8, assign98240_e150580_d_n9, assign98240_e150580_d_n10, assign98240_e150580_d_n11, assign98240_e150580_d_n14,) = {
    if (((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 == 0.0)) {
        let assign98240_e150559: f64 = (locals.var_t1 - 1.0);
        let assign98240_e150560: f64 = (locals.var_isbs_swg * assign98240_e150559);
        let assign98240_e150564: f64 = (locals.var_vbsi_jct - locals.var_vbst);
        let assign98240_e150565: f64 = (locals.var_t4 * assign98240_e150564);
        let assign98240_e150566: f64 = (assign98240_e150560 + assign98240_e150565);
        let assign98240_e150570: f64 = (locals.var_t2 - 1.0);
        let assign98240_e150571: f64 = (locals.var_t0 * assign98240_e150570);
        let assign98240_e150572: f64 = (assign98240_e150566 + assign98240_e150571);
        let assign98240_e150576: f64 = (locals.var_t3 - 1.0);
        let assign98240_e150577: f64 = (locals.var_uc_cisbks * assign98240_e150576);
        let assign98240_e150578: f64 = (assign98240_e150572 + assign98240_e150577);
        (assign98240_e150578, (((((locals.var_isbs_swg_dn0 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign98240_e150564) + (locals.var_t4 * (-locals.var_vbst_dn0)))) + ((locals.var_t0_dn0 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), (((((locals.var_isbs_swg_dn2 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign98240_e150564) + (locals.var_t4 * (-locals.var_vbst_dn2)))) + ((locals.var_t0_dn2 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), (((((locals.var_isbs_swg_dn4 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign98240_e150564) + (locals.var_t4 * (-locals.var_vbst_dn4)))) + ((locals.var_t0_dn4 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), (((((locals.var_isbs_swg_dn5 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign98240_e150564) + (locals.var_t4 * (-locals.var_vbst_dn5)))) + ((locals.var_t0_dn5 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), (((((locals.var_isbs_swg_dn6 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign98240_e150564) + (locals.var_t4 * (-locals.var_vbst_dn6)))) + ((locals.var_t0_dn6 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), (((((locals.var_isbs_swg_dn7 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign98240_e150564) + (locals.var_t4 * (-locals.var_vbst_dn7)))) + ((locals.var_t0_dn7 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), (((((locals.var_isbs_swg_dn8 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign98240_e150564) + (locals.var_t4 * (locals.var_vbsi_jct_dn8 - locals.var_vbst_dn8)))) + ((locals.var_t0_dn8 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), (((((locals.var_isbs_swg_dn9 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign98240_e150564) + (locals.var_t4 * (locals.var_vbsi_jct_dn9 - locals.var_vbst_dn9)))) + ((locals.var_t0_dn9 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), (((((locals.var_isbs_swg_dn10 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign98240_e150564) + (locals.var_t4 * (-locals.var_vbst_dn10)))) + ((locals.var_t0_dn10 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), (((((locals.var_isbs_swg_dn11 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * assign98240_e150564) + (locals.var_t4 * (-locals.var_vbst_dn11)))) + ((locals.var_t0_dn11 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn11))) + (locals.var_uc_cisbks * locals.var_t3_dn11)), (((((locals.var_isbs_swg_dn14 * assign98240_e150559) + (locals.var_isbs_swg * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * assign98240_e150564) + (locals.var_t4 * (-locals.var_vbst_dn14)))) + ((locals.var_t0_dn14 * assign98240_e150570) + (locals.var_t0 * locals.var_t2_dn14))) + (locals.var_uc_cisbks * locals.var_t3_dn14)),)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn11, locals.var_ibs_swg_dn14,)
    }
};
        locals.var_ibs_swg = assign98240_e150580;
        locals.var_ibs_swg_dn0 = assign98240_e150580_d_n0;
        locals.var_ibs_swg_dn2 = assign98240_e150580_d_n2;
        locals.var_ibs_swg_dn4 = assign98240_e150580_d_n4;
        locals.var_ibs_swg_dn5 = assign98240_e150580_d_n5;
        locals.var_ibs_swg_dn6 = assign98240_e150580_d_n6;
        locals.var_ibs_swg_dn7 = assign98240_e150580_d_n7;
        locals.var_ibs_swg_dn8 = assign98240_e150580_d_n8;
        locals.var_ibs_swg_dn9 = assign98240_e150580_d_n9;
        locals.var_ibs_swg_dn10 = assign98240_e150580_d_n10;
        locals.var_ibs_swg_dn11 = assign98240_e150580_d_n11;
        locals.var_ibs_swg_dn14 = assign98240_e150580_d_n14;

        let (assign98250_e150587, assign98250_e150587_d_n0, assign98250_e150587_d_n2, assign98250_e150587_d_n4, assign98250_e150587_d_n5, assign98250_e150587_d_n6, assign98250_e150587_d_n7, assign98250_e150587_d_n8, assign98250_e150587_d_n9, assign98250_e150587_d_n10, assign98250_e150587_d_n11, assign98250_e150587_d_n14,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn11, locals.var_ibs_swg_dn14,)
    }
};
        locals.var_ibs_swg = assign98250_e150587;
        locals.var_ibs_swg_dn0 = assign98250_e150587_d_n0;
        locals.var_ibs_swg_dn2 = assign98250_e150587_d_n2;
        locals.var_ibs_swg_dn4 = assign98250_e150587_d_n4;
        locals.var_ibs_swg_dn5 = assign98250_e150587_d_n5;
        locals.var_ibs_swg_dn6 = assign98250_e150587_d_n6;
        locals.var_ibs_swg_dn7 = assign98250_e150587_d_n7;
        locals.var_ibs_swg_dn8 = assign98250_e150587_d_n8;
        locals.var_ibs_swg_dn9 = assign98250_e150587_d_n9;
        locals.var_ibs_swg_dn10 = assign98250_e150587_d_n10;
        locals.var_ibs_swg_dn11 = assign98250_e150587_d_n11;
        locals.var_ibs_swg_dn14 = assign98250_e150587_d_n14;

        let (assign98260_e150593, assign98260_e150593_d_n0, assign98260_e150593_d_n2, assign98260_e150593_d_n4, assign98260_e150593_d_n5, assign98260_e150593_d_n6, assign98260_e150593_d_n7, assign98260_e150593_d_n8, assign98260_e150593_d_n9, assign98260_e150593_d_n10, assign98260_e150593_d_n11, assign98260_e150593_d_n14,) = {
    if (locals.var_guard2274 != 0.0) {
        let assign98260_e150591: f64 = (p.p537 * locals.var_isbs2_swg);
        (assign98260_e150591, (p.p537 * locals.var_isbs2_swg_dn0), (p.p537 * locals.var_isbs2_swg_dn2), (p.p537 * locals.var_isbs2_swg_dn4), (p.p537 * locals.var_isbs2_swg_dn5), (p.p537 * locals.var_isbs2_swg_dn6), (p.p537 * locals.var_isbs2_swg_dn7), (p.p537 * locals.var_isbs2_swg_dn8), (p.p537 * locals.var_isbs2_swg_dn9), (p.p537 * locals.var_isbs2_swg_dn10), (p.p537 * locals.var_isbs2_swg_dn11), (p.p537 * locals.var_isbs2_swg_dn14),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign98260_e150593;
        locals.var_t12_dn0 = assign98260_e150593_d_n0;
        locals.var_t12_dn2 = assign98260_e150593_d_n2;
        locals.var_t12_dn4 = assign98260_e150593_d_n4;
        locals.var_t12_dn5 = assign98260_e150593_d_n5;
        locals.var_t12_dn6 = assign98260_e150593_d_n6;
        locals.var_t12_dn7 = assign98260_e150593_d_n7;
        locals.var_t12_dn8 = assign98260_e150593_d_n8;
        locals.var_t12_dn9 = assign98260_e150593_d_n9;
        locals.var_t12_dn10 = assign98260_e150593_d_n10;
        locals.var_t12_dn11 = assign98260_e150593_d_n11;
        locals.var_t12_dn14 = assign98260_e150593_d_n14;

    }

    pub(super) fn stamp_transient_block_362(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98270_e150601, assign98270_e150601_d_n0, assign98270_e150601_d_n2, assign98270_e150601_d_n4, assign98270_e150601_d_n5, assign98270_e150601_d_n6, assign98270_e150601_d_n7, assign98270_e150601_d_n8, assign98270_e150601_d_n9, assign98270_e150601_d_n10, assign98270_e150601_d_n11, assign98270_e150601_d_n14,) = {
    if (locals.var_guard2274 != 0.0) {
        let assign98270_e150598: f64 = (locals.var_t12 * locals.var_vbsi_jct);
        let assign98270_e150599: f64 = (locals.var_ibs_swg + assign98270_e150598);
        (assign98270_e150599, (locals.var_ibs_swg_dn0 + (locals.var_t12_dn0 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn2 + (locals.var_t12_dn2 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn4 + (locals.var_t12_dn4 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn5 + (locals.var_t12_dn5 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn6 + (locals.var_t12_dn6 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn7 + (locals.var_t12_dn7 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn8 + ((locals.var_t12_dn8 * locals.var_vbsi_jct) + (locals.var_t12 * locals.var_vbsi_jct_dn8))), (locals.var_ibs_swg_dn9 + ((locals.var_t12_dn9 * locals.var_vbsi_jct) + (locals.var_t12 * locals.var_vbsi_jct_dn9))), (locals.var_ibs_swg_dn10 + (locals.var_t12_dn10 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn11 + (locals.var_t12_dn11 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn14 + (locals.var_t12_dn14 * locals.var_vbsi_jct)),)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn11, locals.var_ibs_swg_dn14,)
    }
};
        locals.var_ibs_swg = assign98270_e150601;
        locals.var_ibs_swg_dn0 = assign98270_e150601_d_n0;
        locals.var_ibs_swg_dn2 = assign98270_e150601_d_n2;
        locals.var_ibs_swg_dn4 = assign98270_e150601_d_n4;
        locals.var_ibs_swg_dn5 = assign98270_e150601_d_n5;
        locals.var_ibs_swg_dn6 = assign98270_e150601_d_n6;
        locals.var_ibs_swg_dn7 = assign98270_e150601_d_n7;
        locals.var_ibs_swg_dn8 = assign98270_e150601_d_n8;
        locals.var_ibs_swg_dn9 = assign98270_e150601_d_n9;
        locals.var_ibs_swg_dn10 = assign98270_e150601_d_n10;
        locals.var_ibs_swg_dn11 = assign98270_e150601_d_n11;
        locals.var_ibs_swg_dn14 = assign98270_e150601_d_n14;

        let (assign98280_e150606, assign98280_e150606_d_n0, assign98280_e150606_d_n2, assign98280_e150606_d_n4, assign98280_e150606_d_n5, assign98280_e150606_d_n6, assign98280_e150606_d_n7, assign98280_e150606_d_n8, assign98280_e150606_d_n9, assign98280_e150606_d_n10, assign98280_e150606_d_n11, assign98280_e150606_d_n14,) = {
    if (locals.var_guard2274 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn11, locals.var_ibs_swg_dn14,)
    }
};
        locals.var_ibs_swg = assign98280_e150606;
        locals.var_ibs_swg_dn0 = assign98280_e150606_d_n0;
        locals.var_ibs_swg_dn2 = assign98280_e150606_d_n2;
        locals.var_ibs_swg_dn4 = assign98280_e150606_d_n4;
        locals.var_ibs_swg_dn5 = assign98280_e150606_d_n5;
        locals.var_ibs_swg_dn6 = assign98280_e150606_d_n6;
        locals.var_ibs_swg_dn7 = assign98280_e150606_d_n7;
        locals.var_ibs_swg_dn8 = assign98280_e150606_d_n8;
        locals.var_ibs_swg_dn9 = assign98280_e150606_d_n9;
        locals.var_ibs_swg_dn10 = assign98280_e150606_d_n10;
        locals.var_ibs_swg_dn11 = assign98280_e150606_d_n11;
        locals.var_ibs_swg_dn14 = assign98280_e150606_d_n14;

        let assign98290_e150609: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2278 = assign98290_e150609;

        let assign98300_e150612: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2279 = assign98300_e150612;

        let (assign98310_e150622, assign98310_e150622_d_n0, assign98310_e150622_d_n2, assign98310_e150622_d_n4, assign98310_e150622_d_n5, assign98310_e150622_d_n6, assign98310_e150622_d_n7, assign98310_e150622_d_n8, assign98310_e150622_d_n9, assign98310_e150622_d_n10, assign98310_e150622_d_n11, assign98310_e150622_d_n14,) = {
    if ((locals.var_guard2278 != 0.0) && (locals.var_guard2279 != 0.0)) {
        let assign98310_e150619: f64 = (locals.var_vbd_jct / locals.var_pzbd);
        let assign98310_e150620: f64 = (1.0 - assign98310_e150619);
        (assign98310_e150620, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbd) - (locals.var_vbd_jct * locals.var_pzbd_dn0)) / (locals.var_pzbd * locals.var_pzbd))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn2) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn4) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn5) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn6) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn7) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn8) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn9) / (locals.var_pzbd * locals.var_pzbd)))), (-(((locals.var_vbd_jct_dn10 * locals.var_pzbd) - (locals.var_vbd_jct * locals.var_pzbd_dn10)) / (locals.var_pzbd * locals.var_pzbd))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn11) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn14) / (locals.var_pzbd * locals.var_pzbd)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign98310_e150622;
        locals.var_arg_dn0 = assign98310_e150622_d_n0;
        locals.var_arg_dn2 = assign98310_e150622_d_n2;
        locals.var_arg_dn4 = assign98310_e150622_d_n4;
        locals.var_arg_dn5 = assign98310_e150622_d_n5;
        locals.var_arg_dn6 = assign98310_e150622_d_n6;
        locals.var_arg_dn7 = assign98310_e150622_d_n7;
        locals.var_arg_dn8 = assign98310_e150622_d_n8;
        locals.var_arg_dn9 = assign98310_e150622_d_n9;
        locals.var_arg_dn10 = assign98310_e150622_d_n10;
        locals.var_arg_dn11 = assign98310_e150622_d_n11;
        locals.var_arg_dn14 = assign98310_e150622_d_n14;

        let assign98320_e150625: f64 = if p.p503 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2280 = assign98320_e150625;

        let (assign98330_e150636, assign98330_e150636_d_n0, assign98330_e150636_d_n2, assign98330_e150636_d_n4, assign98330_e150636_d_n5, assign98330_e150636_d_n6, assign98330_e150636_d_n7, assign98330_e150636_d_n8, assign98330_e150636_d_n9, assign98330_e150636_d_n10, assign98330_e150636_d_n11, assign98330_e150636_d_n14,) = {
    if (((locals.var_guard2278 != 0.0) && (locals.var_guard2279 != 0.0)) && (locals.var_guard2280 != 0.0)) {
        let assign98330_e150633: f64 = (locals.var_arg).sqrt();
        let assign98330_e150634: f64 = (1.0 / assign98330_e150633);
        (assign98330_e150634, (-((locals.var_arg_dn0 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))), (-((locals.var_arg_dn2 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))), (-((locals.var_arg_dn4 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))), (-((locals.var_arg_dn5 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))), (-((locals.var_arg_dn6 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))), (-((locals.var_arg_dn7 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))), (-((locals.var_arg_dn8 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))), (-((locals.var_arg_dn9 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))), (-((locals.var_arg_dn10 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))), (-((locals.var_arg_dn11 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))), (-((locals.var_arg_dn14 / (2.0 * assign98330_e150633)) / (assign98330_e150633 * assign98330_e150633))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98330_e150636;
        locals.var_sarg_dn0 = assign98330_e150636_d_n0;
        locals.var_sarg_dn2 = assign98330_e150636_d_n2;
        locals.var_sarg_dn4 = assign98330_e150636_d_n4;
        locals.var_sarg_dn5 = assign98330_e150636_d_n5;
        locals.var_sarg_dn6 = assign98330_e150636_d_n6;
        locals.var_sarg_dn7 = assign98330_e150636_d_n7;
        locals.var_sarg_dn8 = assign98330_e150636_d_n8;
        locals.var_sarg_dn9 = assign98330_e150636_d_n9;
        locals.var_sarg_dn10 = assign98330_e150636_d_n10;
        locals.var_sarg_dn11 = assign98330_e150636_d_n11;
        locals.var_sarg_dn14 = assign98330_e150636_d_n14;

        let (assign98340_e150653, assign98340_e150653_d_n0, assign98340_e150653_d_n2, assign98340_e150653_d_n4, assign98340_e150653_d_n5, assign98340_e150653_d_n6, assign98340_e150653_d_n7, assign98340_e150653_d_n8, assign98340_e150653_d_n9, assign98340_e150653_d_n10, assign98340_e150653_d_n11, assign98340_e150653_d_n14,) = {
    if (((locals.var_guard2278 != 0.0) && (locals.var_guard2279 != 0.0)) && (locals.var_guard2280 == 0.0)) {
        let (assign98340_e150651, assign98340_e150651_d_n0, assign98340_e150651_d_n2, assign98340_e150651_d_n4, assign98340_e150651_d_n5, assign98340_e150651_d_n6, assign98340_e150651_d_n7, assign98340_e150651_d_n8, assign98340_e150651_d_n9, assign98340_e150651_d_n10, assign98340_e150651_d_n11, assign98340_e150651_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98340_e150649: f64 = (-p.p503);
                let assign98340_e150650: f64 = (locals.var_arg).powf(assign98340_e150649);
                (assign98340_e150650, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn0)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn2)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn4)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn5)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn6)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn7)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn8)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn9)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn10)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn11)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98340_e150649) as f64).is_finite() && ((assign98340_e150649) as f64).fract() == 0.0 { if assign98340_e150649 == 0.0 { 0.0 } else { (assign98340_e150649 * ((locals.var_arg).powf(assign98340_e150649 - 1.0) * locals.var_arg_dn14)) } } else { (assign98340_e150650 * (assign98340_e150649 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign98340_e150651, assign98340_e150651_d_n0, assign98340_e150651_d_n2, assign98340_e150651_d_n4, assign98340_e150651_d_n5, assign98340_e150651_d_n6, assign98340_e150651_d_n7, assign98340_e150651_d_n8, assign98340_e150651_d_n9, assign98340_e150651_d_n10, assign98340_e150651_d_n11, assign98340_e150651_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98340_e150653;
        locals.var_sarg_dn0 = assign98340_e150653_d_n0;
        locals.var_sarg_dn2 = assign98340_e150653_d_n2;
        locals.var_sarg_dn4 = assign98340_e150653_d_n4;
        locals.var_sarg_dn5 = assign98340_e150653_d_n5;
        locals.var_sarg_dn6 = assign98340_e150653_d_n6;
        locals.var_sarg_dn7 = assign98340_e150653_d_n7;
        locals.var_sarg_dn8 = assign98340_e150653_d_n8;
        locals.var_sarg_dn9 = assign98340_e150653_d_n9;
        locals.var_sarg_dn10 = assign98340_e150653_d_n10;
        locals.var_sarg_dn11 = assign98340_e150653_d_n11;
        locals.var_sarg_dn14 = assign98340_e150653_d_n14;

        let (assign98350_e150671, assign98350_e150671_d_n0, assign98350_e150671_d_n2, assign98350_e150671_d_n4, assign98350_e150671_d_n5, assign98350_e150671_d_n6, assign98350_e150671_d_n7, assign98350_e150671_d_n8, assign98350_e150671_d_n9, assign98350_e150671_d_n10, assign98350_e150671_d_n11, assign98350_e150671_d_n14,) = {
    if ((locals.var_guard2278 != 0.0) && (locals.var_guard2279 != 0.0)) {
        let assign98350_e150659: f64 = (locals.var_pzbd * locals.var_czbd);
        let assign98350_e150663: f64 = (locals.var_arg * locals.var_sarg);
        let assign98350_e150664: f64 = (1.0 - assign98350_e150663);
        let assign98350_e150665: f64 = (assign98350_e150659 * assign98350_e150664);
        let assign98350_e150668: f64 = (1.0 - p.p503);
        let assign98350_e150669: f64 = (assign98350_e150665 / assign98350_e150668);
        (assign98350_e150669, (((((locals.var_pzbd_dn0 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn0)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98350_e150668), (((((locals.var_pzbd_dn2 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn2)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98350_e150668), (((((locals.var_pzbd_dn4 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn4)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98350_e150668), (((((locals.var_pzbd_dn5 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn5)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98350_e150668), (((((locals.var_pzbd_dn6 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn6)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98350_e150668), (((((locals.var_pzbd_dn7 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn7)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98350_e150668), (((((locals.var_pzbd_dn8 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn8)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98350_e150668), (((((locals.var_pzbd_dn9 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn9)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98350_e150668), (((((locals.var_pzbd_dn10 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn10)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98350_e150668), (((((locals.var_pzbd_dn11 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn11)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign98350_e150668), (((((locals.var_pzbd_dn14 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn14)) * assign98350_e150664) + (assign98350_e150659 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign98350_e150668),)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn11, locals.var_qbd_btm_dn14,)
    }
};
        locals.var_qbd_btm = assign98350_e150671;
        locals.var_qbd_btm_dn0 = assign98350_e150671_d_n0;
        locals.var_qbd_btm_dn2 = assign98350_e150671_d_n2;
        locals.var_qbd_btm_dn4 = assign98350_e150671_d_n4;
        locals.var_qbd_btm_dn5 = assign98350_e150671_d_n5;
        locals.var_qbd_btm_dn6 = assign98350_e150671_d_n6;
        locals.var_qbd_btm_dn7 = assign98350_e150671_d_n7;
        locals.var_qbd_btm_dn8 = assign98350_e150671_d_n8;
        locals.var_qbd_btm_dn9 = assign98350_e150671_d_n9;
        locals.var_qbd_btm_dn10 = assign98350_e150671_d_n10;
        locals.var_qbd_btm_dn11 = assign98350_e150671_d_n11;
        locals.var_qbd_btm_dn14 = assign98350_e150671_d_n14;

        let (assign98370_e150686, assign98370_e150686_d_n0, assign98370_e150686_d_n2, assign98370_e150686_d_n4, assign98370_e150686_d_n5, assign98370_e150686_d_n6, assign98370_e150686_d_n7, assign98370_e150686_d_n8, assign98370_e150686_d_n9, assign98370_e150686_d_n10, assign98370_e150686_d_n11, assign98370_e150686_d_n14,) = {
    if ((locals.var_guard2278 != 0.0) && (locals.var_guard2279 == 0.0)) {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98370_e150686;
        locals.var_t1_dn0 = assign98370_e150686_d_n0;
        locals.var_t1_dn2 = assign98370_e150686_d_n2;
        locals.var_t1_dn4 = assign98370_e150686_d_n4;
        locals.var_t1_dn5 = assign98370_e150686_d_n5;
        locals.var_t1_dn6 = assign98370_e150686_d_n6;
        locals.var_t1_dn7 = assign98370_e150686_d_n7;
        locals.var_t1_dn8 = assign98370_e150686_d_n8;
        locals.var_t1_dn9 = assign98370_e150686_d_n9;
        locals.var_t1_dn10 = assign98370_e150686_d_n10;
        locals.var_t1_dn11 = assign98370_e150686_d_n11;
        locals.var_t1_dn14 = assign98370_e150686_d_n14;

        let (assign98380_e150697, assign98380_e150697_d_n0, assign98380_e150697_d_n2, assign98380_e150697_d_n4, assign98380_e150697_d_n5, assign98380_e150697_d_n6, assign98380_e150697_d_n7, assign98380_e150697_d_n8, assign98380_e150697_d_n9, assign98380_e150697_d_n10, assign98380_e150697_d_n11, assign98380_e150697_d_n14,) = {
    if ((locals.var_guard2278 != 0.0) && (locals.var_guard2279 == 0.0)) {
        let assign98380_e150693: f64 = (locals.var_czbd * p.p503);
        let assign98380_e150695: f64 = (assign98380_e150693 / locals.var_pzbd);
        (assign98380_e150695, ((((locals.var_czbd_dn0 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn0)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn2 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn2)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn4 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn4)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn5 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn5)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn6 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn6)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn7 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn7)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn8 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn8)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn9 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn9)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn10 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn10)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn11 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn11)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn14 * p.p503) * locals.var_pzbd) - (assign98380_e150693 * locals.var_pzbd_dn14)) / (locals.var_pzbd * locals.var_pzbd)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98380_e150697;
        locals.var_t2_dn0 = assign98380_e150697_d_n0;
        locals.var_t2_dn2 = assign98380_e150697_d_n2;
        locals.var_t2_dn4 = assign98380_e150697_d_n4;
        locals.var_t2_dn5 = assign98380_e150697_d_n5;
        locals.var_t2_dn6 = assign98380_e150697_d_n6;
        locals.var_t2_dn7 = assign98380_e150697_d_n7;
        locals.var_t2_dn8 = assign98380_e150697_d_n8;
        locals.var_t2_dn9 = assign98380_e150697_d_n9;
        locals.var_t2_dn10 = assign98380_e150697_d_n10;
        locals.var_t2_dn11 = assign98380_e150697_d_n11;
        locals.var_t2_dn14 = assign98380_e150697_d_n14;

        let (assign98390_e150712, assign98390_e150712_d_n0, assign98390_e150712_d_n2, assign98390_e150712_d_n4, assign98390_e150712_d_n5, assign98390_e150712_d_n6, assign98390_e150712_d_n7, assign98390_e150712_d_n8, assign98390_e150712_d_n9, assign98390_e150712_d_n10, assign98390_e150712_d_n11, assign98390_e150712_d_n14,) = {
    if ((locals.var_guard2278 != 0.0) && (locals.var_guard2279 == 0.0)) {
        let assign98390_e150706: f64 = (locals.var_vbd_jct * 0.5);
        let assign98390_e150708: f64 = (assign98390_e150706 * locals.var_t2);
        let assign98390_e150709: f64 = (locals.var_t1 + assign98390_e150708);
        let assign98390_e150710: f64 = (locals.var_vbd_jct * assign98390_e150709);
        (assign98390_e150710, ((locals.var_vbd_jct_dn0 * assign98390_e150709) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98390_e150706 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98390_e150706 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98390_e150706 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98390_e150706 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98390_e150706 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98390_e150706 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98390_e150706 * locals.var_t2_dn8))), (locals.var_vbd_jct * (locals.var_t1_dn9 + (assign98390_e150706 * locals.var_t2_dn9))), ((locals.var_vbd_jct_dn10 * assign98390_e150709) + (locals.var_vbd_jct * (locals.var_t1_dn10 + (((locals.var_vbd_jct_dn10 * 0.5) * locals.var_t2) + (assign98390_e150706 * locals.var_t2_dn10))))), (locals.var_vbd_jct * (locals.var_t1_dn11 + (assign98390_e150706 * locals.var_t2_dn11))), (locals.var_vbd_jct * (locals.var_t1_dn14 + (assign98390_e150706 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn11, locals.var_qbd_btm_dn14,)
    }
};
        locals.var_qbd_btm = assign98390_e150712;
        locals.var_qbd_btm_dn0 = assign98390_e150712_d_n0;
        locals.var_qbd_btm_dn2 = assign98390_e150712_d_n2;
        locals.var_qbd_btm_dn4 = assign98390_e150712_d_n4;
        locals.var_qbd_btm_dn5 = assign98390_e150712_d_n5;
        locals.var_qbd_btm_dn6 = assign98390_e150712_d_n6;
        locals.var_qbd_btm_dn7 = assign98390_e150712_d_n7;
        locals.var_qbd_btm_dn8 = assign98390_e150712_d_n8;
        locals.var_qbd_btm_dn9 = assign98390_e150712_d_n9;
        locals.var_qbd_btm_dn10 = assign98390_e150712_d_n10;
        locals.var_qbd_btm_dn11 = assign98390_e150712_d_n11;
        locals.var_qbd_btm_dn14 = assign98390_e150712_d_n14;

        let (assign98410_e150728, assign98410_e150728_d_n0, assign98410_e150728_d_n2, assign98410_e150728_d_n4, assign98410_e150728_d_n5, assign98410_e150728_d_n6, assign98410_e150728_d_n7, assign98410_e150728_d_n8, assign98410_e150728_d_n9, assign98410_e150728_d_n10, assign98410_e150728_d_n11, assign98410_e150728_d_n14,) = {
    if (locals.var_guard2278 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn11, locals.var_qbd_btm_dn14,)
    }
};
        locals.var_qbd_btm = assign98410_e150728;
        locals.var_qbd_btm_dn0 = assign98410_e150728_d_n0;
        locals.var_qbd_btm_dn2 = assign98410_e150728_d_n2;
        locals.var_qbd_btm_dn4 = assign98410_e150728_d_n4;
        locals.var_qbd_btm_dn5 = assign98410_e150728_d_n5;
        locals.var_qbd_btm_dn6 = assign98410_e150728_d_n6;
        locals.var_qbd_btm_dn7 = assign98410_e150728_d_n7;
        locals.var_qbd_btm_dn8 = assign98410_e150728_d_n8;
        locals.var_qbd_btm_dn9 = assign98410_e150728_d_n9;
        locals.var_qbd_btm_dn10 = assign98410_e150728_d_n10;
        locals.var_qbd_btm_dn11 = assign98410_e150728_d_n11;
        locals.var_qbd_btm_dn14 = assign98410_e150728_d_n14;

        let assign98430_e150736: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2281 = assign98430_e150736;

        let assign98440_e150739: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2282 = assign98440_e150739;

        let (assign98450_e150749, assign98450_e150749_d_n0, assign98450_e150749_d_n2, assign98450_e150749_d_n4, assign98450_e150749_d_n5, assign98450_e150749_d_n6, assign98450_e150749_d_n7, assign98450_e150749_d_n8, assign98450_e150749_d_n9, assign98450_e150749_d_n10, assign98450_e150749_d_n11, assign98450_e150749_d_n14,) = {
    if ((locals.var_guard2281 != 0.0) && (locals.var_guard2282 != 0.0)) {
        let assign98450_e150746: f64 = (locals.var_vbd_jct / locals.var_pzbdsw);
        let assign98450_e150747: f64 = (1.0 - assign98450_e150746);
        (assign98450_e150747, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbdsw) - (locals.var_vbd_jct * locals.var_pzbdsw_dn0)) / (locals.var_pzbdsw * locals.var_pzbdsw))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn2) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn4) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn5) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn6) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn7) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn8) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn9) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(((locals.var_vbd_jct_dn10 * locals.var_pzbdsw) - (locals.var_vbd_jct * locals.var_pzbdsw_dn10)) / (locals.var_pzbdsw * locals.var_pzbdsw))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn11) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn14) / (locals.var_pzbdsw * locals.var_pzbdsw)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign98450_e150749;
        locals.var_arg_dn0 = assign98450_e150749_d_n0;
        locals.var_arg_dn2 = assign98450_e150749_d_n2;
        locals.var_arg_dn4 = assign98450_e150749_d_n4;
        locals.var_arg_dn5 = assign98450_e150749_d_n5;
        locals.var_arg_dn6 = assign98450_e150749_d_n6;
        locals.var_arg_dn7 = assign98450_e150749_d_n7;
        locals.var_arg_dn8 = assign98450_e150749_d_n8;
        locals.var_arg_dn9 = assign98450_e150749_d_n9;
        locals.var_arg_dn10 = assign98450_e150749_d_n10;
        locals.var_arg_dn11 = assign98450_e150749_d_n11;
        locals.var_arg_dn14 = assign98450_e150749_d_n14;

        let assign98460_e150752: f64 = if p.p504 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2283 = assign98460_e150752;

        let (assign98470_e150763, assign98470_e150763_d_n0, assign98470_e150763_d_n2, assign98470_e150763_d_n4, assign98470_e150763_d_n5, assign98470_e150763_d_n6, assign98470_e150763_d_n7, assign98470_e150763_d_n8, assign98470_e150763_d_n9, assign98470_e150763_d_n10, assign98470_e150763_d_n11, assign98470_e150763_d_n14,) = {
    if (((locals.var_guard2281 != 0.0) && (locals.var_guard2282 != 0.0)) && (locals.var_guard2283 != 0.0)) {
        let assign98470_e150760: f64 = (locals.var_arg).sqrt();
        let assign98470_e150761: f64 = (1.0 / assign98470_e150760);
        (assign98470_e150761, (-((locals.var_arg_dn0 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))), (-((locals.var_arg_dn2 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))), (-((locals.var_arg_dn4 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))), (-((locals.var_arg_dn5 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))), (-((locals.var_arg_dn6 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))), (-((locals.var_arg_dn7 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))), (-((locals.var_arg_dn8 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))), (-((locals.var_arg_dn9 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))), (-((locals.var_arg_dn10 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))), (-((locals.var_arg_dn11 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))), (-((locals.var_arg_dn14 / (2.0 * assign98470_e150760)) / (assign98470_e150760 * assign98470_e150760))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98470_e150763;
        locals.var_sarg_dn0 = assign98470_e150763_d_n0;
        locals.var_sarg_dn2 = assign98470_e150763_d_n2;
        locals.var_sarg_dn4 = assign98470_e150763_d_n4;
        locals.var_sarg_dn5 = assign98470_e150763_d_n5;
        locals.var_sarg_dn6 = assign98470_e150763_d_n6;
        locals.var_sarg_dn7 = assign98470_e150763_d_n7;
        locals.var_sarg_dn8 = assign98470_e150763_d_n8;
        locals.var_sarg_dn9 = assign98470_e150763_d_n9;
        locals.var_sarg_dn10 = assign98470_e150763_d_n10;
        locals.var_sarg_dn11 = assign98470_e150763_d_n11;
        locals.var_sarg_dn14 = assign98470_e150763_d_n14;

        let (assign98480_e150780, assign98480_e150780_d_n0, assign98480_e150780_d_n2, assign98480_e150780_d_n4, assign98480_e150780_d_n5, assign98480_e150780_d_n6, assign98480_e150780_d_n7, assign98480_e150780_d_n8, assign98480_e150780_d_n9, assign98480_e150780_d_n10, assign98480_e150780_d_n11, assign98480_e150780_d_n14,) = {
    if (((locals.var_guard2281 != 0.0) && (locals.var_guard2282 != 0.0)) && (locals.var_guard2283 == 0.0)) {
        let (assign98480_e150778, assign98480_e150778_d_n0, assign98480_e150778_d_n2, assign98480_e150778_d_n4, assign98480_e150778_d_n5, assign98480_e150778_d_n6, assign98480_e150778_d_n7, assign98480_e150778_d_n8, assign98480_e150778_d_n9, assign98480_e150778_d_n10, assign98480_e150778_d_n11, assign98480_e150778_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98480_e150776: f64 = (-p.p504);
                let assign98480_e150777: f64 = (locals.var_arg).powf(assign98480_e150776);
                (assign98480_e150777, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn0)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn2)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn4)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn5)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn6)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn7)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn8)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn9)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn10)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn11)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98480_e150776) as f64).is_finite() && ((assign98480_e150776) as f64).fract() == 0.0 { if assign98480_e150776 == 0.0 { 0.0 } else { (assign98480_e150776 * ((locals.var_arg).powf(assign98480_e150776 - 1.0) * locals.var_arg_dn14)) } } else { (assign98480_e150777 * (assign98480_e150776 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign98480_e150778, assign98480_e150778_d_n0, assign98480_e150778_d_n2, assign98480_e150778_d_n4, assign98480_e150778_d_n5, assign98480_e150778_d_n6, assign98480_e150778_d_n7, assign98480_e150778_d_n8, assign98480_e150778_d_n9, assign98480_e150778_d_n10, assign98480_e150778_d_n11, assign98480_e150778_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98480_e150780;
        locals.var_sarg_dn0 = assign98480_e150780_d_n0;
        locals.var_sarg_dn2 = assign98480_e150780_d_n2;
        locals.var_sarg_dn4 = assign98480_e150780_d_n4;
        locals.var_sarg_dn5 = assign98480_e150780_d_n5;
        locals.var_sarg_dn6 = assign98480_e150780_d_n6;
        locals.var_sarg_dn7 = assign98480_e150780_d_n7;
        locals.var_sarg_dn8 = assign98480_e150780_d_n8;
        locals.var_sarg_dn9 = assign98480_e150780_d_n9;
        locals.var_sarg_dn10 = assign98480_e150780_d_n10;
        locals.var_sarg_dn11 = assign98480_e150780_d_n11;
        locals.var_sarg_dn14 = assign98480_e150780_d_n14;

        let (assign98490_e150798, assign98490_e150798_d_n0, assign98490_e150798_d_n2, assign98490_e150798_d_n4, assign98490_e150798_d_n5, assign98490_e150798_d_n6, assign98490_e150798_d_n7, assign98490_e150798_d_n8, assign98490_e150798_d_n9, assign98490_e150798_d_n10, assign98490_e150798_d_n11, assign98490_e150798_d_n14,) = {
    if ((locals.var_guard2281 != 0.0) && (locals.var_guard2282 != 0.0)) {
        let assign98490_e150786: f64 = (locals.var_pzbdsw * locals.var_czbdsw);
        let assign98490_e150790: f64 = (locals.var_arg * locals.var_sarg);
        let assign98490_e150791: f64 = (1.0 - assign98490_e150790);
        let assign98490_e150792: f64 = (assign98490_e150786 * assign98490_e150791);
        let assign98490_e150795: f64 = (1.0 - p.p504);
        let assign98490_e150796: f64 = (assign98490_e150792 / assign98490_e150795);
        (assign98490_e150796, (((((locals.var_pzbdsw_dn0 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn0)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98490_e150795), (((((locals.var_pzbdsw_dn2 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn2)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98490_e150795), (((((locals.var_pzbdsw_dn4 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn4)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98490_e150795), (((((locals.var_pzbdsw_dn5 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn5)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98490_e150795), (((((locals.var_pzbdsw_dn6 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn6)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98490_e150795), (((((locals.var_pzbdsw_dn7 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn7)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98490_e150795), (((((locals.var_pzbdsw_dn8 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn8)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98490_e150795), (((((locals.var_pzbdsw_dn9 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn9)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98490_e150795), (((((locals.var_pzbdsw_dn10 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn10)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98490_e150795), (((((locals.var_pzbdsw_dn11 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn11)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign98490_e150795), (((((locals.var_pzbdsw_dn14 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn14)) * assign98490_e150791) + (assign98490_e150786 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign98490_e150795),)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn11, locals.var_qbd_sws_dn14,)
    }
};
        locals.var_qbd_sws = assign98490_e150798;
        locals.var_qbd_sws_dn0 = assign98490_e150798_d_n0;
        locals.var_qbd_sws_dn2 = assign98490_e150798_d_n2;
        locals.var_qbd_sws_dn4 = assign98490_e150798_d_n4;
        locals.var_qbd_sws_dn5 = assign98490_e150798_d_n5;
        locals.var_qbd_sws_dn6 = assign98490_e150798_d_n6;
        locals.var_qbd_sws_dn7 = assign98490_e150798_d_n7;
        locals.var_qbd_sws_dn8 = assign98490_e150798_d_n8;
        locals.var_qbd_sws_dn9 = assign98490_e150798_d_n9;
        locals.var_qbd_sws_dn10 = assign98490_e150798_d_n10;
        locals.var_qbd_sws_dn11 = assign98490_e150798_d_n11;
        locals.var_qbd_sws_dn14 = assign98490_e150798_d_n14;

        let (assign98510_e150813, assign98510_e150813_d_n0, assign98510_e150813_d_n2, assign98510_e150813_d_n4, assign98510_e150813_d_n5, assign98510_e150813_d_n6, assign98510_e150813_d_n7, assign98510_e150813_d_n8, assign98510_e150813_d_n9, assign98510_e150813_d_n10, assign98510_e150813_d_n11, assign98510_e150813_d_n14,) = {
    if ((locals.var_guard2281 != 0.0) && (locals.var_guard2282 == 0.0)) {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98510_e150813;
        locals.var_t1_dn0 = assign98510_e150813_d_n0;
        locals.var_t1_dn2 = assign98510_e150813_d_n2;
        locals.var_t1_dn4 = assign98510_e150813_d_n4;
        locals.var_t1_dn5 = assign98510_e150813_d_n5;
        locals.var_t1_dn6 = assign98510_e150813_d_n6;
        locals.var_t1_dn7 = assign98510_e150813_d_n7;
        locals.var_t1_dn8 = assign98510_e150813_d_n8;
        locals.var_t1_dn9 = assign98510_e150813_d_n9;
        locals.var_t1_dn10 = assign98510_e150813_d_n10;
        locals.var_t1_dn11 = assign98510_e150813_d_n11;
        locals.var_t1_dn14 = assign98510_e150813_d_n14;

        let (assign98520_e150824, assign98520_e150824_d_n0, assign98520_e150824_d_n2, assign98520_e150824_d_n4, assign98520_e150824_d_n5, assign98520_e150824_d_n6, assign98520_e150824_d_n7, assign98520_e150824_d_n8, assign98520_e150824_d_n9, assign98520_e150824_d_n10, assign98520_e150824_d_n11, assign98520_e150824_d_n14,) = {
    if ((locals.var_guard2281 != 0.0) && (locals.var_guard2282 == 0.0)) {
        let assign98520_e150820: f64 = (locals.var_czbdsw * p.p504);
        let assign98520_e150822: f64 = (assign98520_e150820 / locals.var_pzbdsw);
        (assign98520_e150822, ((((locals.var_czbdsw_dn0 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn0)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn2 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn2)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn4 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn4)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn5 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn5)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn6 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn6)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn7 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn7)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn8 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn8)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn9 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn9)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn10 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn10)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn11 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn11)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn14 * p.p504) * locals.var_pzbdsw) - (assign98520_e150820 * locals.var_pzbdsw_dn14)) / (locals.var_pzbdsw * locals.var_pzbdsw)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98520_e150824;
        locals.var_t2_dn0 = assign98520_e150824_d_n0;
        locals.var_t2_dn2 = assign98520_e150824_d_n2;
        locals.var_t2_dn4 = assign98520_e150824_d_n4;
        locals.var_t2_dn5 = assign98520_e150824_d_n5;
        locals.var_t2_dn6 = assign98520_e150824_d_n6;
        locals.var_t2_dn7 = assign98520_e150824_d_n7;
        locals.var_t2_dn8 = assign98520_e150824_d_n8;
        locals.var_t2_dn9 = assign98520_e150824_d_n9;
        locals.var_t2_dn10 = assign98520_e150824_d_n10;
        locals.var_t2_dn11 = assign98520_e150824_d_n11;
        locals.var_t2_dn14 = assign98520_e150824_d_n14;

        let (assign98530_e150839, assign98530_e150839_d_n0, assign98530_e150839_d_n2, assign98530_e150839_d_n4, assign98530_e150839_d_n5, assign98530_e150839_d_n6, assign98530_e150839_d_n7, assign98530_e150839_d_n8, assign98530_e150839_d_n9, assign98530_e150839_d_n10, assign98530_e150839_d_n11, assign98530_e150839_d_n14,) = {
    if ((locals.var_guard2281 != 0.0) && (locals.var_guard2282 == 0.0)) {
        let assign98530_e150833: f64 = (locals.var_vbd_jct * 0.5);
        let assign98530_e150835: f64 = (assign98530_e150833 * locals.var_t2);
        let assign98530_e150836: f64 = (locals.var_t1 + assign98530_e150835);
        let assign98530_e150837: f64 = (locals.var_vbd_jct * assign98530_e150836);
        (assign98530_e150837, ((locals.var_vbd_jct_dn0 * assign98530_e150836) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98530_e150833 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98530_e150833 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98530_e150833 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98530_e150833 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98530_e150833 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98530_e150833 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98530_e150833 * locals.var_t2_dn8))), (locals.var_vbd_jct * (locals.var_t1_dn9 + (assign98530_e150833 * locals.var_t2_dn9))), ((locals.var_vbd_jct_dn10 * assign98530_e150836) + (locals.var_vbd_jct * (locals.var_t1_dn10 + (((locals.var_vbd_jct_dn10 * 0.5) * locals.var_t2) + (assign98530_e150833 * locals.var_t2_dn10))))), (locals.var_vbd_jct * (locals.var_t1_dn11 + (assign98530_e150833 * locals.var_t2_dn11))), (locals.var_vbd_jct * (locals.var_t1_dn14 + (assign98530_e150833 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn11, locals.var_qbd_sws_dn14,)
    }
};
        locals.var_qbd_sws = assign98530_e150839;
        locals.var_qbd_sws_dn0 = assign98530_e150839_d_n0;
        locals.var_qbd_sws_dn2 = assign98530_e150839_d_n2;
        locals.var_qbd_sws_dn4 = assign98530_e150839_d_n4;
        locals.var_qbd_sws_dn5 = assign98530_e150839_d_n5;
        locals.var_qbd_sws_dn6 = assign98530_e150839_d_n6;
        locals.var_qbd_sws_dn7 = assign98530_e150839_d_n7;
        locals.var_qbd_sws_dn8 = assign98530_e150839_d_n8;
        locals.var_qbd_sws_dn9 = assign98530_e150839_d_n9;
        locals.var_qbd_sws_dn10 = assign98530_e150839_d_n10;
        locals.var_qbd_sws_dn11 = assign98530_e150839_d_n11;
        locals.var_qbd_sws_dn14 = assign98530_e150839_d_n14;

        let (assign98550_e150855, assign98550_e150855_d_n0, assign98550_e150855_d_n2, assign98550_e150855_d_n4, assign98550_e150855_d_n5, assign98550_e150855_d_n6, assign98550_e150855_d_n7, assign98550_e150855_d_n8, assign98550_e150855_d_n9, assign98550_e150855_d_n10, assign98550_e150855_d_n11, assign98550_e150855_d_n14,) = {
    if (locals.var_guard2281 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn11, locals.var_qbd_sws_dn14,)
    }
};
        locals.var_qbd_sws = assign98550_e150855;
        locals.var_qbd_sws_dn0 = assign98550_e150855_d_n0;
        locals.var_qbd_sws_dn2 = assign98550_e150855_d_n2;
        locals.var_qbd_sws_dn4 = assign98550_e150855_d_n4;
        locals.var_qbd_sws_dn5 = assign98550_e150855_d_n5;
        locals.var_qbd_sws_dn6 = assign98550_e150855_d_n6;
        locals.var_qbd_sws_dn7 = assign98550_e150855_d_n7;
        locals.var_qbd_sws_dn8 = assign98550_e150855_d_n8;
        locals.var_qbd_sws_dn9 = assign98550_e150855_d_n9;
        locals.var_qbd_sws_dn10 = assign98550_e150855_d_n10;
        locals.var_qbd_sws_dn11 = assign98550_e150855_d_n11;
        locals.var_qbd_sws_dn14 = assign98550_e150855_d_n14;

        let assign98570_e150863: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2284 = assign98570_e150863;

        let assign98580_e150866: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2285 = assign98580_e150866;

        let assign98590_e150869: f64 = if locals.var_vbdi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2286 = assign98590_e150869;

        let (assign98600_e150881, assign98600_e150881_d_n0, assign98600_e150881_d_n2, assign98600_e150881_d_n4, assign98600_e150881_d_n5, assign98600_e150881_d_n6, assign98600_e150881_d_n7, assign98600_e150881_d_n8, assign98600_e150881_d_n9, assign98600_e150881_d_n10, assign98600_e150881_d_n11, assign98600_e150881_d_n14,) = {
    if (((locals.var_guard2284 != 0.0) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 != 0.0)) {
        let assign98600_e150878: f64 = (locals.var_vbdi_jct / locals.var_pzbdswg);
        let assign98600_e150879: f64 = (1.0 - assign98600_e150878);
        (assign98600_e150879, (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn0) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn2) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn4) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn5) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbdi_jct_dn6 * locals.var_pzbdswg) - (locals.var_vbdi_jct * locals.var_pzbdswg_dn6)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn7) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn8) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbdi_jct_dn9 * locals.var_pzbdswg) - (locals.var_vbdi_jct * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn10) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn11) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn14) / (locals.var_pzbdswg * locals.var_pzbdswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign98600_e150881;
        locals.var_arg_dn0 = assign98600_e150881_d_n0;
        locals.var_arg_dn2 = assign98600_e150881_d_n2;
        locals.var_arg_dn4 = assign98600_e150881_d_n4;
        locals.var_arg_dn5 = assign98600_e150881_d_n5;
        locals.var_arg_dn6 = assign98600_e150881_d_n6;
        locals.var_arg_dn7 = assign98600_e150881_d_n7;
        locals.var_arg_dn8 = assign98600_e150881_d_n8;
        locals.var_arg_dn9 = assign98600_e150881_d_n9;
        locals.var_arg_dn10 = assign98600_e150881_d_n10;
        locals.var_arg_dn11 = assign98600_e150881_d_n11;
        locals.var_arg_dn14 = assign98600_e150881_d_n14;

        let assign98610_e150884: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2287 = assign98610_e150884;

        let (assign98620_e150897, assign98620_e150897_d_n0, assign98620_e150897_d_n2, assign98620_e150897_d_n4, assign98620_e150897_d_n5, assign98620_e150897_d_n6, assign98620_e150897_d_n7, assign98620_e150897_d_n8, assign98620_e150897_d_n9, assign98620_e150897_d_n10, assign98620_e150897_d_n11, assign98620_e150897_d_n14,) = {
    if ((((locals.var_guard2284 != 0.0) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 != 0.0)) && (locals.var_guard2287 != 0.0)) {
        let assign98620_e150894: f64 = (locals.var_arg).sqrt();
        let assign98620_e150895: f64 = (1.0 / assign98620_e150894);
        (assign98620_e150895, (-((locals.var_arg_dn0 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))), (-((locals.var_arg_dn2 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))), (-((locals.var_arg_dn4 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))), (-((locals.var_arg_dn5 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))), (-((locals.var_arg_dn6 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))), (-((locals.var_arg_dn7 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))), (-((locals.var_arg_dn8 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))), (-((locals.var_arg_dn9 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))), (-((locals.var_arg_dn10 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))), (-((locals.var_arg_dn11 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))), (-((locals.var_arg_dn14 / (2.0 * assign98620_e150894)) / (assign98620_e150894 * assign98620_e150894))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98620_e150897;
        locals.var_sarg_dn0 = assign98620_e150897_d_n0;
        locals.var_sarg_dn2 = assign98620_e150897_d_n2;
        locals.var_sarg_dn4 = assign98620_e150897_d_n4;
        locals.var_sarg_dn5 = assign98620_e150897_d_n5;
        locals.var_sarg_dn6 = assign98620_e150897_d_n6;
        locals.var_sarg_dn7 = assign98620_e150897_d_n7;
        locals.var_sarg_dn8 = assign98620_e150897_d_n8;
        locals.var_sarg_dn9 = assign98620_e150897_d_n9;
        locals.var_sarg_dn10 = assign98620_e150897_d_n10;
        locals.var_sarg_dn11 = assign98620_e150897_d_n11;
        locals.var_sarg_dn14 = assign98620_e150897_d_n14;

    }

    pub(super) fn stamp_transient_block_363(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98630_e150916, assign98630_e150916_d_n0, assign98630_e150916_d_n2, assign98630_e150916_d_n4, assign98630_e150916_d_n5, assign98630_e150916_d_n6, assign98630_e150916_d_n7, assign98630_e150916_d_n8, assign98630_e150916_d_n9, assign98630_e150916_d_n10, assign98630_e150916_d_n11, assign98630_e150916_d_n14,) = {
    if ((((locals.var_guard2284 != 0.0) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 != 0.0)) && (locals.var_guard2287 == 0.0)) {
        let (assign98630_e150914, assign98630_e150914_d_n0, assign98630_e150914_d_n2, assign98630_e150914_d_n4, assign98630_e150914_d_n5, assign98630_e150914_d_n6, assign98630_e150914_d_n7, assign98630_e150914_d_n8, assign98630_e150914_d_n9, assign98630_e150914_d_n10, assign98630_e150914_d_n11, assign98630_e150914_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98630_e150912: f64 = (-p.p505);
                let assign98630_e150913: f64 = (locals.var_arg).powf(assign98630_e150912);
                (assign98630_e150913, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn0)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn2)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn4)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn5)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn6)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn7)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn8)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn9)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn10)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn11)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98630_e150912) as f64).is_finite() && ((assign98630_e150912) as f64).fract() == 0.0 { if assign98630_e150912 == 0.0 { 0.0 } else { (assign98630_e150912 * ((locals.var_arg).powf(assign98630_e150912 - 1.0) * locals.var_arg_dn14)) } } else { (assign98630_e150913 * (assign98630_e150912 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign98630_e150914, assign98630_e150914_d_n0, assign98630_e150914_d_n2, assign98630_e150914_d_n4, assign98630_e150914_d_n5, assign98630_e150914_d_n6, assign98630_e150914_d_n7, assign98630_e150914_d_n8, assign98630_e150914_d_n9, assign98630_e150914_d_n10, assign98630_e150914_d_n11, assign98630_e150914_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98630_e150916;
        locals.var_sarg_dn0 = assign98630_e150916_d_n0;
        locals.var_sarg_dn2 = assign98630_e150916_d_n2;
        locals.var_sarg_dn4 = assign98630_e150916_d_n4;
        locals.var_sarg_dn5 = assign98630_e150916_d_n5;
        locals.var_sarg_dn6 = assign98630_e150916_d_n6;
        locals.var_sarg_dn7 = assign98630_e150916_d_n7;
        locals.var_sarg_dn8 = assign98630_e150916_d_n8;
        locals.var_sarg_dn9 = assign98630_e150916_d_n9;
        locals.var_sarg_dn10 = assign98630_e150916_d_n10;
        locals.var_sarg_dn11 = assign98630_e150916_d_n11;
        locals.var_sarg_dn14 = assign98630_e150916_d_n14;

        let (assign98640_e150936, assign98640_e150936_d_n0, assign98640_e150936_d_n2, assign98640_e150936_d_n4, assign98640_e150936_d_n5, assign98640_e150936_d_n6, assign98640_e150936_d_n7, assign98640_e150936_d_n8, assign98640_e150936_d_n9, assign98640_e150936_d_n10, assign98640_e150936_d_n11, assign98640_e150936_d_n14,) = {
    if (((locals.var_guard2284 != 0.0) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 != 0.0)) {
        let assign98640_e150924: f64 = (locals.var_pzbdswg * locals.var_czbdswg);
        let assign98640_e150928: f64 = (locals.var_arg * locals.var_sarg);
        let assign98640_e150929: f64 = (1.0 - assign98640_e150928);
        let assign98640_e150930: f64 = (assign98640_e150924 * assign98640_e150929);
        let assign98640_e150933: f64 = (1.0 - p.p505);
        let assign98640_e150934: f64 = (assign98640_e150930 / assign98640_e150933);
        (assign98640_e150934, (((((locals.var_pzbdswg_dn0 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn0)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98640_e150933), (((((locals.var_pzbdswg_dn2 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn2)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98640_e150933), (((((locals.var_pzbdswg_dn4 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn4)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98640_e150933), (((((locals.var_pzbdswg_dn5 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn5)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98640_e150933), (((((locals.var_pzbdswg_dn6 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn6)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98640_e150933), (((((locals.var_pzbdswg_dn7 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn7)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98640_e150933), (((((locals.var_pzbdswg_dn8 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn8)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98640_e150933), (((((locals.var_pzbdswg_dn9 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn9)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98640_e150933), (((((locals.var_pzbdswg_dn10 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn10)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98640_e150933), (((((locals.var_pzbdswg_dn11 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn11)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign98640_e150933), (((((locals.var_pzbdswg_dn14 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn14)) * assign98640_e150929) + (assign98640_e150924 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign98640_e150933),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98640_e150936;
        locals.var_qbd_swg_dn0 = assign98640_e150936_d_n0;
        locals.var_qbd_swg_dn2 = assign98640_e150936_d_n2;
        locals.var_qbd_swg_dn4 = assign98640_e150936_d_n4;
        locals.var_qbd_swg_dn5 = assign98640_e150936_d_n5;
        locals.var_qbd_swg_dn6 = assign98640_e150936_d_n6;
        locals.var_qbd_swg_dn7 = assign98640_e150936_d_n7;
        locals.var_qbd_swg_dn8 = assign98640_e150936_d_n8;
        locals.var_qbd_swg_dn9 = assign98640_e150936_d_n9;
        locals.var_qbd_swg_dn10 = assign98640_e150936_d_n10;
        locals.var_qbd_swg_dn11 = assign98640_e150936_d_n11;
        locals.var_qbd_swg_dn14 = assign98640_e150936_d_n14;

        let (assign98660_e150955, assign98660_e150955_d_n0, assign98660_e150955_d_n2, assign98660_e150955_d_n4, assign98660_e150955_d_n5, assign98660_e150955_d_n6, assign98660_e150955_d_n7, assign98660_e150955_d_n8, assign98660_e150955_d_n9, assign98660_e150955_d_n10, assign98660_e150955_d_n11, assign98660_e150955_d_n14,) = {
    if (((locals.var_guard2284 != 0.0) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 == 0.0)) {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98660_e150955;
        locals.var_t1_dn0 = assign98660_e150955_d_n0;
        locals.var_t1_dn2 = assign98660_e150955_d_n2;
        locals.var_t1_dn4 = assign98660_e150955_d_n4;
        locals.var_t1_dn5 = assign98660_e150955_d_n5;
        locals.var_t1_dn6 = assign98660_e150955_d_n6;
        locals.var_t1_dn7 = assign98660_e150955_d_n7;
        locals.var_t1_dn8 = assign98660_e150955_d_n8;
        locals.var_t1_dn9 = assign98660_e150955_d_n9;
        locals.var_t1_dn10 = assign98660_e150955_d_n10;
        locals.var_t1_dn11 = assign98660_e150955_d_n11;
        locals.var_t1_dn14 = assign98660_e150955_d_n14;

        let (assign98670_e150968, assign98670_e150968_d_n0, assign98670_e150968_d_n2, assign98670_e150968_d_n4, assign98670_e150968_d_n5, assign98670_e150968_d_n6, assign98670_e150968_d_n7, assign98670_e150968_d_n8, assign98670_e150968_d_n9, assign98670_e150968_d_n10, assign98670_e150968_d_n11, assign98670_e150968_d_n14,) = {
    if (((locals.var_guard2284 != 0.0) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 == 0.0)) {
        let assign98670_e150964: f64 = (locals.var_czbdswg * p.p505);
        let assign98670_e150966: f64 = (assign98670_e150964 / locals.var_pzbdswg);
        (assign98670_e150966, ((((locals.var_czbdswg_dn0 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn2 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn2)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn4 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn4)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn5 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn5)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn6 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn6)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn7 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn7)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn8 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn8)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn9 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn10 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn10)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn11 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn11)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn14 * p.p505) * locals.var_pzbdswg) - (assign98670_e150964 * locals.var_pzbdswg_dn14)) / (locals.var_pzbdswg * locals.var_pzbdswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98670_e150968;
        locals.var_t2_dn0 = assign98670_e150968_d_n0;
        locals.var_t2_dn2 = assign98670_e150968_d_n2;
        locals.var_t2_dn4 = assign98670_e150968_d_n4;
        locals.var_t2_dn5 = assign98670_e150968_d_n5;
        locals.var_t2_dn6 = assign98670_e150968_d_n6;
        locals.var_t2_dn7 = assign98670_e150968_d_n7;
        locals.var_t2_dn8 = assign98670_e150968_d_n8;
        locals.var_t2_dn9 = assign98670_e150968_d_n9;
        locals.var_t2_dn10 = assign98670_e150968_d_n10;
        locals.var_t2_dn11 = assign98670_e150968_d_n11;
        locals.var_t2_dn14 = assign98670_e150968_d_n14;

        let (assign98680_e150985, assign98680_e150985_d_n0, assign98680_e150985_d_n2, assign98680_e150985_d_n4, assign98680_e150985_d_n5, assign98680_e150985_d_n6, assign98680_e150985_d_n7, assign98680_e150985_d_n8, assign98680_e150985_d_n9, assign98680_e150985_d_n10, assign98680_e150985_d_n11, assign98680_e150985_d_n14,) = {
    if (((locals.var_guard2284 != 0.0) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 == 0.0)) {
        let assign98680_e150979: f64 = (locals.var_vbdi_jct * 0.5);
        let assign98680_e150981: f64 = (assign98680_e150979 * locals.var_t2);
        let assign98680_e150982: f64 = (locals.var_t1 + assign98680_e150981);
        let assign98680_e150983: f64 = (locals.var_vbdi_jct * assign98680_e150982);
        (assign98680_e150983, (locals.var_vbdi_jct * (locals.var_t1_dn0 + (assign98680_e150979 * locals.var_t2_dn0))), (locals.var_vbdi_jct * (locals.var_t1_dn2 + (assign98680_e150979 * locals.var_t2_dn2))), (locals.var_vbdi_jct * (locals.var_t1_dn4 + (assign98680_e150979 * locals.var_t2_dn4))), (locals.var_vbdi_jct * (locals.var_t1_dn5 + (assign98680_e150979 * locals.var_t2_dn5))), ((locals.var_vbdi_jct_dn6 * assign98680_e150982) + (locals.var_vbdi_jct * (locals.var_t1_dn6 + (((locals.var_vbdi_jct_dn6 * 0.5) * locals.var_t2) + (assign98680_e150979 * locals.var_t2_dn6))))), (locals.var_vbdi_jct * (locals.var_t1_dn7 + (assign98680_e150979 * locals.var_t2_dn7))), (locals.var_vbdi_jct * (locals.var_t1_dn8 + (assign98680_e150979 * locals.var_t2_dn8))), ((locals.var_vbdi_jct_dn9 * assign98680_e150982) + (locals.var_vbdi_jct * (locals.var_t1_dn9 + (((locals.var_vbdi_jct_dn9 * 0.5) * locals.var_t2) + (assign98680_e150979 * locals.var_t2_dn9))))), (locals.var_vbdi_jct * (locals.var_t1_dn10 + (assign98680_e150979 * locals.var_t2_dn10))), (locals.var_vbdi_jct * (locals.var_t1_dn11 + (assign98680_e150979 * locals.var_t2_dn11))), (locals.var_vbdi_jct * (locals.var_t1_dn14 + (assign98680_e150979 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98680_e150985;
        locals.var_qbd_swg_dn0 = assign98680_e150985_d_n0;
        locals.var_qbd_swg_dn2 = assign98680_e150985_d_n2;
        locals.var_qbd_swg_dn4 = assign98680_e150985_d_n4;
        locals.var_qbd_swg_dn5 = assign98680_e150985_d_n5;
        locals.var_qbd_swg_dn6 = assign98680_e150985_d_n6;
        locals.var_qbd_swg_dn7 = assign98680_e150985_d_n7;
        locals.var_qbd_swg_dn8 = assign98680_e150985_d_n8;
        locals.var_qbd_swg_dn9 = assign98680_e150985_d_n9;
        locals.var_qbd_swg_dn10 = assign98680_e150985_d_n10;
        locals.var_qbd_swg_dn11 = assign98680_e150985_d_n11;
        locals.var_qbd_swg_dn14 = assign98680_e150985_d_n14;

        let (assign98700_e151005, assign98700_e151005_d_n0, assign98700_e151005_d_n2, assign98700_e151005_d_n4, assign98700_e151005_d_n5, assign98700_e151005_d_n6, assign98700_e151005_d_n7, assign98700_e151005_d_n8, assign98700_e151005_d_n9, assign98700_e151005_d_n10, assign98700_e151005_d_n11, assign98700_e151005_d_n14,) = {
    if ((locals.var_guard2284 != 0.0) && (locals.var_guard2285 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98700_e151005;
        locals.var_qbd_swg_dn0 = assign98700_e151005_d_n0;
        locals.var_qbd_swg_dn2 = assign98700_e151005_d_n2;
        locals.var_qbd_swg_dn4 = assign98700_e151005_d_n4;
        locals.var_qbd_swg_dn5 = assign98700_e151005_d_n5;
        locals.var_qbd_swg_dn6 = assign98700_e151005_d_n6;
        locals.var_qbd_swg_dn7 = assign98700_e151005_d_n7;
        locals.var_qbd_swg_dn8 = assign98700_e151005_d_n8;
        locals.var_qbd_swg_dn9 = assign98700_e151005_d_n9;
        locals.var_qbd_swg_dn10 = assign98700_e151005_d_n10;
        locals.var_qbd_swg_dn11 = assign98700_e151005_d_n11;
        locals.var_qbd_swg_dn14 = assign98700_e151005_d_n14;

        let assign98720_e151015: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2288 = assign98720_e151015;

        let assign98730_e151018: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2289 = assign98730_e151018;

        let (assign98740_e151031, assign98740_e151031_d_n0, assign98740_e151031_d_n2, assign98740_e151031_d_n4, assign98740_e151031_d_n5, assign98740_e151031_d_n6, assign98740_e151031_d_n7, assign98740_e151031_d_n8, assign98740_e151031_d_n9, assign98740_e151031_d_n10, assign98740_e151031_d_n11, assign98740_e151031_d_n14,) = {
    if (((locals.var_guard2284 == 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 != 0.0)) {
        let assign98740_e151028: f64 = (locals.var_vbd_jct / locals.var_pzbdswg);
        let assign98740_e151029: f64 = (1.0 - assign98740_e151028);
        (assign98740_e151029, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbdswg) - (locals.var_vbd_jct * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn2) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn4) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn5) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn6) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn7) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn8) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn9) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbd_jct_dn10 * locals.var_pzbdswg) - (locals.var_vbd_jct * locals.var_pzbdswg_dn10)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn11) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn14) / (locals.var_pzbdswg * locals.var_pzbdswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign98740_e151031;
        locals.var_arg_dn0 = assign98740_e151031_d_n0;
        locals.var_arg_dn2 = assign98740_e151031_d_n2;
        locals.var_arg_dn4 = assign98740_e151031_d_n4;
        locals.var_arg_dn5 = assign98740_e151031_d_n5;
        locals.var_arg_dn6 = assign98740_e151031_d_n6;
        locals.var_arg_dn7 = assign98740_e151031_d_n7;
        locals.var_arg_dn8 = assign98740_e151031_d_n8;
        locals.var_arg_dn9 = assign98740_e151031_d_n9;
        locals.var_arg_dn10 = assign98740_e151031_d_n10;
        locals.var_arg_dn11 = assign98740_e151031_d_n11;
        locals.var_arg_dn14 = assign98740_e151031_d_n14;

        let assign98750_e151034: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2290 = assign98750_e151034;

        let (assign98760_e151048, assign98760_e151048_d_n0, assign98760_e151048_d_n2, assign98760_e151048_d_n4, assign98760_e151048_d_n5, assign98760_e151048_d_n6, assign98760_e151048_d_n7, assign98760_e151048_d_n8, assign98760_e151048_d_n9, assign98760_e151048_d_n10, assign98760_e151048_d_n11, assign98760_e151048_d_n14,) = {
    if ((((locals.var_guard2284 == 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 != 0.0)) && (locals.var_guard2290 != 0.0)) {
        let assign98760_e151045: f64 = (locals.var_arg).sqrt();
        let assign98760_e151046: f64 = (1.0 / assign98760_e151045);
        (assign98760_e151046, (-((locals.var_arg_dn0 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))), (-((locals.var_arg_dn2 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))), (-((locals.var_arg_dn4 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))), (-((locals.var_arg_dn5 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))), (-((locals.var_arg_dn6 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))), (-((locals.var_arg_dn7 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))), (-((locals.var_arg_dn8 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))), (-((locals.var_arg_dn9 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))), (-((locals.var_arg_dn10 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))), (-((locals.var_arg_dn11 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))), (-((locals.var_arg_dn14 / (2.0 * assign98760_e151045)) / (assign98760_e151045 * assign98760_e151045))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98760_e151048;
        locals.var_sarg_dn0 = assign98760_e151048_d_n0;
        locals.var_sarg_dn2 = assign98760_e151048_d_n2;
        locals.var_sarg_dn4 = assign98760_e151048_d_n4;
        locals.var_sarg_dn5 = assign98760_e151048_d_n5;
        locals.var_sarg_dn6 = assign98760_e151048_d_n6;
        locals.var_sarg_dn7 = assign98760_e151048_d_n7;
        locals.var_sarg_dn8 = assign98760_e151048_d_n8;
        locals.var_sarg_dn9 = assign98760_e151048_d_n9;
        locals.var_sarg_dn10 = assign98760_e151048_d_n10;
        locals.var_sarg_dn11 = assign98760_e151048_d_n11;
        locals.var_sarg_dn14 = assign98760_e151048_d_n14;

        let (assign98770_e151068, assign98770_e151068_d_n0, assign98770_e151068_d_n2, assign98770_e151068_d_n4, assign98770_e151068_d_n5, assign98770_e151068_d_n6, assign98770_e151068_d_n7, assign98770_e151068_d_n8, assign98770_e151068_d_n9, assign98770_e151068_d_n10, assign98770_e151068_d_n11, assign98770_e151068_d_n14,) = {
    if ((((locals.var_guard2284 == 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 != 0.0)) && (locals.var_guard2290 == 0.0)) {
        let (assign98770_e151066, assign98770_e151066_d_n0, assign98770_e151066_d_n2, assign98770_e151066_d_n4, assign98770_e151066_d_n5, assign98770_e151066_d_n6, assign98770_e151066_d_n7, assign98770_e151066_d_n8, assign98770_e151066_d_n9, assign98770_e151066_d_n10, assign98770_e151066_d_n11, assign98770_e151066_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98770_e151064: f64 = (-p.p505);
                let assign98770_e151065: f64 = (locals.var_arg).powf(assign98770_e151064);
                (assign98770_e151065, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn0)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn2)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn4)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn5)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn6)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn7)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn8)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn9)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn10)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn11)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98770_e151064) as f64).is_finite() && ((assign98770_e151064) as f64).fract() == 0.0 { if assign98770_e151064 == 0.0 { 0.0 } else { (assign98770_e151064 * ((locals.var_arg).powf(assign98770_e151064 - 1.0) * locals.var_arg_dn14)) } } else { (assign98770_e151065 * (assign98770_e151064 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign98770_e151066, assign98770_e151066_d_n0, assign98770_e151066_d_n2, assign98770_e151066_d_n4, assign98770_e151066_d_n5, assign98770_e151066_d_n6, assign98770_e151066_d_n7, assign98770_e151066_d_n8, assign98770_e151066_d_n9, assign98770_e151066_d_n10, assign98770_e151066_d_n11, assign98770_e151066_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98770_e151068;
        locals.var_sarg_dn0 = assign98770_e151068_d_n0;
        locals.var_sarg_dn2 = assign98770_e151068_d_n2;
        locals.var_sarg_dn4 = assign98770_e151068_d_n4;
        locals.var_sarg_dn5 = assign98770_e151068_d_n5;
        locals.var_sarg_dn6 = assign98770_e151068_d_n6;
        locals.var_sarg_dn7 = assign98770_e151068_d_n7;
        locals.var_sarg_dn8 = assign98770_e151068_d_n8;
        locals.var_sarg_dn9 = assign98770_e151068_d_n9;
        locals.var_sarg_dn10 = assign98770_e151068_d_n10;
        locals.var_sarg_dn11 = assign98770_e151068_d_n11;
        locals.var_sarg_dn14 = assign98770_e151068_d_n14;

        let (assign98780_e151089, assign98780_e151089_d_n0, assign98780_e151089_d_n2, assign98780_e151089_d_n4, assign98780_e151089_d_n5, assign98780_e151089_d_n6, assign98780_e151089_d_n7, assign98780_e151089_d_n8, assign98780_e151089_d_n9, assign98780_e151089_d_n10, assign98780_e151089_d_n11, assign98780_e151089_d_n14,) = {
    if (((locals.var_guard2284 == 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 != 0.0)) {
        let assign98780_e151077: f64 = (locals.var_pzbdswg * locals.var_czbdswg);
        let assign98780_e151081: f64 = (locals.var_arg * locals.var_sarg);
        let assign98780_e151082: f64 = (1.0 - assign98780_e151081);
        let assign98780_e151083: f64 = (assign98780_e151077 * assign98780_e151082);
        let assign98780_e151086: f64 = (1.0 - p.p505);
        let assign98780_e151087: f64 = (assign98780_e151083 / assign98780_e151086);
        (assign98780_e151087, (((((locals.var_pzbdswg_dn0 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn0)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98780_e151086), (((((locals.var_pzbdswg_dn2 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn2)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98780_e151086), (((((locals.var_pzbdswg_dn4 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn4)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98780_e151086), (((((locals.var_pzbdswg_dn5 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn5)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98780_e151086), (((((locals.var_pzbdswg_dn6 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn6)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98780_e151086), (((((locals.var_pzbdswg_dn7 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn7)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98780_e151086), (((((locals.var_pzbdswg_dn8 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn8)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98780_e151086), (((((locals.var_pzbdswg_dn9 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn9)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98780_e151086), (((((locals.var_pzbdswg_dn10 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn10)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98780_e151086), (((((locals.var_pzbdswg_dn11 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn11)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign98780_e151086), (((((locals.var_pzbdswg_dn14 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn14)) * assign98780_e151082) + (assign98780_e151077 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign98780_e151086),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98780_e151089;
        locals.var_qbd_swg_dn0 = assign98780_e151089_d_n0;
        locals.var_qbd_swg_dn2 = assign98780_e151089_d_n2;
        locals.var_qbd_swg_dn4 = assign98780_e151089_d_n4;
        locals.var_qbd_swg_dn5 = assign98780_e151089_d_n5;
        locals.var_qbd_swg_dn6 = assign98780_e151089_d_n6;
        locals.var_qbd_swg_dn7 = assign98780_e151089_d_n7;
        locals.var_qbd_swg_dn8 = assign98780_e151089_d_n8;
        locals.var_qbd_swg_dn9 = assign98780_e151089_d_n9;
        locals.var_qbd_swg_dn10 = assign98780_e151089_d_n10;
        locals.var_qbd_swg_dn11 = assign98780_e151089_d_n11;
        locals.var_qbd_swg_dn14 = assign98780_e151089_d_n14;

        let (assign98800_e151110, assign98800_e151110_d_n0, assign98800_e151110_d_n2, assign98800_e151110_d_n4, assign98800_e151110_d_n5, assign98800_e151110_d_n6, assign98800_e151110_d_n7, assign98800_e151110_d_n8, assign98800_e151110_d_n9, assign98800_e151110_d_n10, assign98800_e151110_d_n11, assign98800_e151110_d_n14,) = {
    if (((locals.var_guard2284 == 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 == 0.0)) {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98800_e151110;
        locals.var_t1_dn0 = assign98800_e151110_d_n0;
        locals.var_t1_dn2 = assign98800_e151110_d_n2;
        locals.var_t1_dn4 = assign98800_e151110_d_n4;
        locals.var_t1_dn5 = assign98800_e151110_d_n5;
        locals.var_t1_dn6 = assign98800_e151110_d_n6;
        locals.var_t1_dn7 = assign98800_e151110_d_n7;
        locals.var_t1_dn8 = assign98800_e151110_d_n8;
        locals.var_t1_dn9 = assign98800_e151110_d_n9;
        locals.var_t1_dn10 = assign98800_e151110_d_n10;
        locals.var_t1_dn11 = assign98800_e151110_d_n11;
        locals.var_t1_dn14 = assign98800_e151110_d_n14;

        let (assign98810_e151124, assign98810_e151124_d_n0, assign98810_e151124_d_n2, assign98810_e151124_d_n4, assign98810_e151124_d_n5, assign98810_e151124_d_n6, assign98810_e151124_d_n7, assign98810_e151124_d_n8, assign98810_e151124_d_n9, assign98810_e151124_d_n10, assign98810_e151124_d_n11, assign98810_e151124_d_n14,) = {
    if (((locals.var_guard2284 == 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 == 0.0)) {
        let assign98810_e151120: f64 = (locals.var_czbdswg * p.p505);
        let assign98810_e151122: f64 = (assign98810_e151120 / locals.var_pzbdswg);
        (assign98810_e151122, ((((locals.var_czbdswg_dn0 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn2 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn2)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn4 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn4)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn5 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn5)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn6 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn6)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn7 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn7)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn8 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn8)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn9 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn10 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn10)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn11 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn11)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn14 * p.p505) * locals.var_pzbdswg) - (assign98810_e151120 * locals.var_pzbdswg_dn14)) / (locals.var_pzbdswg * locals.var_pzbdswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98810_e151124;
        locals.var_t2_dn0 = assign98810_e151124_d_n0;
        locals.var_t2_dn2 = assign98810_e151124_d_n2;
        locals.var_t2_dn4 = assign98810_e151124_d_n4;
        locals.var_t2_dn5 = assign98810_e151124_d_n5;
        locals.var_t2_dn6 = assign98810_e151124_d_n6;
        locals.var_t2_dn7 = assign98810_e151124_d_n7;
        locals.var_t2_dn8 = assign98810_e151124_d_n8;
        locals.var_t2_dn9 = assign98810_e151124_d_n9;
        locals.var_t2_dn10 = assign98810_e151124_d_n10;
        locals.var_t2_dn11 = assign98810_e151124_d_n11;
        locals.var_t2_dn14 = assign98810_e151124_d_n14;

        let (assign98820_e151142, assign98820_e151142_d_n0, assign98820_e151142_d_n2, assign98820_e151142_d_n4, assign98820_e151142_d_n5, assign98820_e151142_d_n6, assign98820_e151142_d_n7, assign98820_e151142_d_n8, assign98820_e151142_d_n9, assign98820_e151142_d_n10, assign98820_e151142_d_n11, assign98820_e151142_d_n14,) = {
    if (((locals.var_guard2284 == 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 == 0.0)) {
        let assign98820_e151136: f64 = (locals.var_vbd_jct * 0.5);
        let assign98820_e151138: f64 = (assign98820_e151136 * locals.var_t2);
        let assign98820_e151139: f64 = (locals.var_t1 + assign98820_e151138);
        let assign98820_e151140: f64 = (locals.var_vbd_jct * assign98820_e151139);
        (assign98820_e151140, ((locals.var_vbd_jct_dn0 * assign98820_e151139) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98820_e151136 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98820_e151136 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98820_e151136 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98820_e151136 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98820_e151136 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98820_e151136 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98820_e151136 * locals.var_t2_dn8))), (locals.var_vbd_jct * (locals.var_t1_dn9 + (assign98820_e151136 * locals.var_t2_dn9))), ((locals.var_vbd_jct_dn10 * assign98820_e151139) + (locals.var_vbd_jct * (locals.var_t1_dn10 + (((locals.var_vbd_jct_dn10 * 0.5) * locals.var_t2) + (assign98820_e151136 * locals.var_t2_dn10))))), (locals.var_vbd_jct * (locals.var_t1_dn11 + (assign98820_e151136 * locals.var_t2_dn11))), (locals.var_vbd_jct * (locals.var_t1_dn14 + (assign98820_e151136 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98820_e151142;
        locals.var_qbd_swg_dn0 = assign98820_e151142_d_n0;
        locals.var_qbd_swg_dn2 = assign98820_e151142_d_n2;
        locals.var_qbd_swg_dn4 = assign98820_e151142_d_n4;
        locals.var_qbd_swg_dn5 = assign98820_e151142_d_n5;
        locals.var_qbd_swg_dn6 = assign98820_e151142_d_n6;
        locals.var_qbd_swg_dn7 = assign98820_e151142_d_n7;
        locals.var_qbd_swg_dn8 = assign98820_e151142_d_n8;
        locals.var_qbd_swg_dn9 = assign98820_e151142_d_n9;
        locals.var_qbd_swg_dn10 = assign98820_e151142_d_n10;
        locals.var_qbd_swg_dn11 = assign98820_e151142_d_n11;
        locals.var_qbd_swg_dn14 = assign98820_e151142_d_n14;

        let (assign98840_e151164, assign98840_e151164_d_n0, assign98840_e151164_d_n2, assign98840_e151164_d_n4, assign98840_e151164_d_n5, assign98840_e151164_d_n6, assign98840_e151164_d_n7, assign98840_e151164_d_n8, assign98840_e151164_d_n9, assign98840_e151164_d_n10, assign98840_e151164_d_n11, assign98840_e151164_d_n14,) = {
    if ((locals.var_guard2284 == 0.0) && (locals.var_guard2288 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn11, locals.var_qbd_swg_dn14,)
    }
};
        locals.var_qbd_swg = assign98840_e151164;
        locals.var_qbd_swg_dn0 = assign98840_e151164_d_n0;
        locals.var_qbd_swg_dn2 = assign98840_e151164_d_n2;
        locals.var_qbd_swg_dn4 = assign98840_e151164_d_n4;
        locals.var_qbd_swg_dn5 = assign98840_e151164_d_n5;
        locals.var_qbd_swg_dn6 = assign98840_e151164_d_n6;
        locals.var_qbd_swg_dn7 = assign98840_e151164_d_n7;
        locals.var_qbd_swg_dn8 = assign98840_e151164_d_n8;
        locals.var_qbd_swg_dn9 = assign98840_e151164_d_n9;
        locals.var_qbd_swg_dn10 = assign98840_e151164_d_n10;
        locals.var_qbd_swg_dn11 = assign98840_e151164_d_n11;
        locals.var_qbd_swg_dn14 = assign98840_e151164_d_n14;

        let assign98860_e151175: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2291 = assign98860_e151175;

        let assign98870_e151178: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2292 = assign98870_e151178;

        let (assign98880_e151188, assign98880_e151188_d_n0, assign98880_e151188_d_n2, assign98880_e151188_d_n4, assign98880_e151188_d_n5, assign98880_e151188_d_n6, assign98880_e151188_d_n7, assign98880_e151188_d_n8, assign98880_e151188_d_n9, assign98880_e151188_d_n10, assign98880_e151188_d_n11, assign98880_e151188_d_n14,) = {
    if ((locals.var_guard2291 != 0.0) && (locals.var_guard2292 != 0.0)) {
        let assign98880_e151185: f64 = (locals.var_vbs_jct / locals.var_pzbs);
        let assign98880_e151186: f64 = (1.0 - assign98880_e151185);
        (assign98880_e151186, (-(-((locals.var_vbs_jct * locals.var_pzbs_dn0) / (locals.var_pzbs * locals.var_pzbs)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbs) - (locals.var_vbs_jct * locals.var_pzbs_dn2)) / (locals.var_pzbs * locals.var_pzbs))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn4) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn5) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn6) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn7) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn8) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn9) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn10) / (locals.var_pzbs * locals.var_pzbs)))), (-(((locals.var_vbs_jct_dn11 * locals.var_pzbs) - (locals.var_vbs_jct * locals.var_pzbs_dn11)) / (locals.var_pzbs * locals.var_pzbs))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn14) / (locals.var_pzbs * locals.var_pzbs)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign98880_e151188;
        locals.var_arg_dn0 = assign98880_e151188_d_n0;
        locals.var_arg_dn2 = assign98880_e151188_d_n2;
        locals.var_arg_dn4 = assign98880_e151188_d_n4;
        locals.var_arg_dn5 = assign98880_e151188_d_n5;
        locals.var_arg_dn6 = assign98880_e151188_d_n6;
        locals.var_arg_dn7 = assign98880_e151188_d_n7;
        locals.var_arg_dn8 = assign98880_e151188_d_n8;
        locals.var_arg_dn9 = assign98880_e151188_d_n9;
        locals.var_arg_dn10 = assign98880_e151188_d_n10;
        locals.var_arg_dn11 = assign98880_e151188_d_n11;
        locals.var_arg_dn14 = assign98880_e151188_d_n14;

        let assign98890_e151191: f64 = if p.p526 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2293 = assign98890_e151191;

        let (assign98900_e151202, assign98900_e151202_d_n0, assign98900_e151202_d_n2, assign98900_e151202_d_n4, assign98900_e151202_d_n5, assign98900_e151202_d_n6, assign98900_e151202_d_n7, assign98900_e151202_d_n8, assign98900_e151202_d_n9, assign98900_e151202_d_n10, assign98900_e151202_d_n11, assign98900_e151202_d_n14,) = {
    if (((locals.var_guard2291 != 0.0) && (locals.var_guard2292 != 0.0)) && (locals.var_guard2293 != 0.0)) {
        let assign98900_e151199: f64 = (locals.var_arg).sqrt();
        let assign98900_e151200: f64 = (1.0 / assign98900_e151199);
        (assign98900_e151200, (-((locals.var_arg_dn0 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))), (-((locals.var_arg_dn2 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))), (-((locals.var_arg_dn4 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))), (-((locals.var_arg_dn5 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))), (-((locals.var_arg_dn6 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))), (-((locals.var_arg_dn7 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))), (-((locals.var_arg_dn8 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))), (-((locals.var_arg_dn9 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))), (-((locals.var_arg_dn10 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))), (-((locals.var_arg_dn11 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))), (-((locals.var_arg_dn14 / (2.0 * assign98900_e151199)) / (assign98900_e151199 * assign98900_e151199))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98900_e151202;
        locals.var_sarg_dn0 = assign98900_e151202_d_n0;
        locals.var_sarg_dn2 = assign98900_e151202_d_n2;
        locals.var_sarg_dn4 = assign98900_e151202_d_n4;
        locals.var_sarg_dn5 = assign98900_e151202_d_n5;
        locals.var_sarg_dn6 = assign98900_e151202_d_n6;
        locals.var_sarg_dn7 = assign98900_e151202_d_n7;
        locals.var_sarg_dn8 = assign98900_e151202_d_n8;
        locals.var_sarg_dn9 = assign98900_e151202_d_n9;
        locals.var_sarg_dn10 = assign98900_e151202_d_n10;
        locals.var_sarg_dn11 = assign98900_e151202_d_n11;
        locals.var_sarg_dn14 = assign98900_e151202_d_n14;

        let (assign98910_e151219, assign98910_e151219_d_n0, assign98910_e151219_d_n2, assign98910_e151219_d_n4, assign98910_e151219_d_n5, assign98910_e151219_d_n6, assign98910_e151219_d_n7, assign98910_e151219_d_n8, assign98910_e151219_d_n9, assign98910_e151219_d_n10, assign98910_e151219_d_n11, assign98910_e151219_d_n14,) = {
    if (((locals.var_guard2291 != 0.0) && (locals.var_guard2292 != 0.0)) && (locals.var_guard2293 == 0.0)) {
        let (assign98910_e151217, assign98910_e151217_d_n0, assign98910_e151217_d_n2, assign98910_e151217_d_n4, assign98910_e151217_d_n5, assign98910_e151217_d_n6, assign98910_e151217_d_n7, assign98910_e151217_d_n8, assign98910_e151217_d_n9, assign98910_e151217_d_n10, assign98910_e151217_d_n11, assign98910_e151217_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98910_e151215: f64 = (-p.p526);
                let assign98910_e151216: f64 = (locals.var_arg).powf(assign98910_e151215);
                (assign98910_e151216, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn0)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn2)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn4)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn5)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn6)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn7)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn8)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn9)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn10)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn11)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98910_e151215) as f64).is_finite() && ((assign98910_e151215) as f64).fract() == 0.0 { if assign98910_e151215 == 0.0 { 0.0 } else { (assign98910_e151215 * ((locals.var_arg).powf(assign98910_e151215 - 1.0) * locals.var_arg_dn14)) } } else { (assign98910_e151216 * (assign98910_e151215 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign98910_e151217, assign98910_e151217_d_n0, assign98910_e151217_d_n2, assign98910_e151217_d_n4, assign98910_e151217_d_n5, assign98910_e151217_d_n6, assign98910_e151217_d_n7, assign98910_e151217_d_n8, assign98910_e151217_d_n9, assign98910_e151217_d_n10, assign98910_e151217_d_n11, assign98910_e151217_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign98910_e151219;
        locals.var_sarg_dn0 = assign98910_e151219_d_n0;
        locals.var_sarg_dn2 = assign98910_e151219_d_n2;
        locals.var_sarg_dn4 = assign98910_e151219_d_n4;
        locals.var_sarg_dn5 = assign98910_e151219_d_n5;
        locals.var_sarg_dn6 = assign98910_e151219_d_n6;
        locals.var_sarg_dn7 = assign98910_e151219_d_n7;
        locals.var_sarg_dn8 = assign98910_e151219_d_n8;
        locals.var_sarg_dn9 = assign98910_e151219_d_n9;
        locals.var_sarg_dn10 = assign98910_e151219_d_n10;
        locals.var_sarg_dn11 = assign98910_e151219_d_n11;
        locals.var_sarg_dn14 = assign98910_e151219_d_n14;

        let (assign98920_e151237, assign98920_e151237_d_n0, assign98920_e151237_d_n2, assign98920_e151237_d_n4, assign98920_e151237_d_n5, assign98920_e151237_d_n6, assign98920_e151237_d_n7, assign98920_e151237_d_n8, assign98920_e151237_d_n9, assign98920_e151237_d_n10, assign98920_e151237_d_n11, assign98920_e151237_d_n14,) = {
    if ((locals.var_guard2291 != 0.0) && (locals.var_guard2292 != 0.0)) {
        let assign98920_e151225: f64 = (locals.var_pzbs * locals.var_czbs);
        let assign98920_e151229: f64 = (locals.var_arg * locals.var_sarg);
        let assign98920_e151230: f64 = (1.0 - assign98920_e151229);
        let assign98920_e151231: f64 = (assign98920_e151225 * assign98920_e151230);
        let assign98920_e151234: f64 = (1.0 - p.p526);
        let assign98920_e151235: f64 = (assign98920_e151231 / assign98920_e151234);
        (assign98920_e151235, (((((locals.var_pzbs_dn0 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn0)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98920_e151234), (((((locals.var_pzbs_dn2 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn2)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98920_e151234), (((((locals.var_pzbs_dn4 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn4)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98920_e151234), (((((locals.var_pzbs_dn5 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn5)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98920_e151234), (((((locals.var_pzbs_dn6 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn6)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98920_e151234), (((((locals.var_pzbs_dn7 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn7)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98920_e151234), (((((locals.var_pzbs_dn8 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn8)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98920_e151234), (((((locals.var_pzbs_dn9 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn9)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98920_e151234), (((((locals.var_pzbs_dn10 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn10)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98920_e151234), (((((locals.var_pzbs_dn11 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn11)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign98920_e151234), (((((locals.var_pzbs_dn14 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn14)) * assign98920_e151230) + (assign98920_e151225 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign98920_e151234),)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn11, locals.var_qbs_btm_dn14,)
    }
};
        locals.var_qbs_btm = assign98920_e151237;
        locals.var_qbs_btm_dn0 = assign98920_e151237_d_n0;
        locals.var_qbs_btm_dn2 = assign98920_e151237_d_n2;
        locals.var_qbs_btm_dn4 = assign98920_e151237_d_n4;
        locals.var_qbs_btm_dn5 = assign98920_e151237_d_n5;
        locals.var_qbs_btm_dn6 = assign98920_e151237_d_n6;
        locals.var_qbs_btm_dn7 = assign98920_e151237_d_n7;
        locals.var_qbs_btm_dn8 = assign98920_e151237_d_n8;
        locals.var_qbs_btm_dn9 = assign98920_e151237_d_n9;
        locals.var_qbs_btm_dn10 = assign98920_e151237_d_n10;
        locals.var_qbs_btm_dn11 = assign98920_e151237_d_n11;
        locals.var_qbs_btm_dn14 = assign98920_e151237_d_n14;

        let (assign98940_e151252, assign98940_e151252_d_n0, assign98940_e151252_d_n2, assign98940_e151252_d_n4, assign98940_e151252_d_n5, assign98940_e151252_d_n6, assign98940_e151252_d_n7, assign98940_e151252_d_n8, assign98940_e151252_d_n9, assign98940_e151252_d_n10, assign98940_e151252_d_n11, assign98940_e151252_d_n14,) = {
    if ((locals.var_guard2291 != 0.0) && (locals.var_guard2292 == 0.0)) {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign98940_e151252;
        locals.var_t1_dn0 = assign98940_e151252_d_n0;
        locals.var_t1_dn2 = assign98940_e151252_d_n2;
        locals.var_t1_dn4 = assign98940_e151252_d_n4;
        locals.var_t1_dn5 = assign98940_e151252_d_n5;
        locals.var_t1_dn6 = assign98940_e151252_d_n6;
        locals.var_t1_dn7 = assign98940_e151252_d_n7;
        locals.var_t1_dn8 = assign98940_e151252_d_n8;
        locals.var_t1_dn9 = assign98940_e151252_d_n9;
        locals.var_t1_dn10 = assign98940_e151252_d_n10;
        locals.var_t1_dn11 = assign98940_e151252_d_n11;
        locals.var_t1_dn14 = assign98940_e151252_d_n14;

        let (assign98950_e151263, assign98950_e151263_d_n0, assign98950_e151263_d_n2, assign98950_e151263_d_n4, assign98950_e151263_d_n5, assign98950_e151263_d_n6, assign98950_e151263_d_n7, assign98950_e151263_d_n8, assign98950_e151263_d_n9, assign98950_e151263_d_n10, assign98950_e151263_d_n11, assign98950_e151263_d_n14,) = {
    if ((locals.var_guard2291 != 0.0) && (locals.var_guard2292 == 0.0)) {
        let assign98950_e151259: f64 = (locals.var_czbs * p.p526);
        let assign98950_e151261: f64 = (assign98950_e151259 / locals.var_pzbs);
        (assign98950_e151261, ((((locals.var_czbs_dn0 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn0)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn2 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn2)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn4 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn4)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn5 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn5)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn6 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn6)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn7 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn7)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn8 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn8)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn9 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn9)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn10 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn10)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn11 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn11)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn14 * p.p526) * locals.var_pzbs) - (assign98950_e151259 * locals.var_pzbs_dn14)) / (locals.var_pzbs * locals.var_pzbs)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign98950_e151263;
        locals.var_t2_dn0 = assign98950_e151263_d_n0;
        locals.var_t2_dn2 = assign98950_e151263_d_n2;
        locals.var_t2_dn4 = assign98950_e151263_d_n4;
        locals.var_t2_dn5 = assign98950_e151263_d_n5;
        locals.var_t2_dn6 = assign98950_e151263_d_n6;
        locals.var_t2_dn7 = assign98950_e151263_d_n7;
        locals.var_t2_dn8 = assign98950_e151263_d_n8;
        locals.var_t2_dn9 = assign98950_e151263_d_n9;
        locals.var_t2_dn10 = assign98950_e151263_d_n10;
        locals.var_t2_dn11 = assign98950_e151263_d_n11;
        locals.var_t2_dn14 = assign98950_e151263_d_n14;

        let (assign98960_e151278, assign98960_e151278_d_n0, assign98960_e151278_d_n2, assign98960_e151278_d_n4, assign98960_e151278_d_n5, assign98960_e151278_d_n6, assign98960_e151278_d_n7, assign98960_e151278_d_n8, assign98960_e151278_d_n9, assign98960_e151278_d_n10, assign98960_e151278_d_n11, assign98960_e151278_d_n14,) = {
    if ((locals.var_guard2291 != 0.0) && (locals.var_guard2292 == 0.0)) {
        let assign98960_e151272: f64 = (locals.var_vbs_jct * 0.5);
        let assign98960_e151274: f64 = (assign98960_e151272 * locals.var_t2);
        let assign98960_e151275: f64 = (locals.var_t1 + assign98960_e151274);
        let assign98960_e151276: f64 = (locals.var_vbs_jct * assign98960_e151275);
        (assign98960_e151276, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign98960_e151272 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign98960_e151275) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign98960_e151272 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign98960_e151272 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign98960_e151272 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign98960_e151272 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign98960_e151272 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign98960_e151272 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign98960_e151272 * locals.var_t2_dn9))), (locals.var_vbs_jct * (locals.var_t1_dn10 + (assign98960_e151272 * locals.var_t2_dn10))), ((locals.var_vbs_jct_dn11 * assign98960_e151275) + (locals.var_vbs_jct * (locals.var_t1_dn11 + (((locals.var_vbs_jct_dn11 * 0.5) * locals.var_t2) + (assign98960_e151272 * locals.var_t2_dn11))))), (locals.var_vbs_jct * (locals.var_t1_dn14 + (assign98960_e151272 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn11, locals.var_qbs_btm_dn14,)
    }
};
        locals.var_qbs_btm = assign98960_e151278;
        locals.var_qbs_btm_dn0 = assign98960_e151278_d_n0;
        locals.var_qbs_btm_dn2 = assign98960_e151278_d_n2;
        locals.var_qbs_btm_dn4 = assign98960_e151278_d_n4;
        locals.var_qbs_btm_dn5 = assign98960_e151278_d_n5;
        locals.var_qbs_btm_dn6 = assign98960_e151278_d_n6;
        locals.var_qbs_btm_dn7 = assign98960_e151278_d_n7;
        locals.var_qbs_btm_dn8 = assign98960_e151278_d_n8;
        locals.var_qbs_btm_dn9 = assign98960_e151278_d_n9;
        locals.var_qbs_btm_dn10 = assign98960_e151278_d_n10;
        locals.var_qbs_btm_dn11 = assign98960_e151278_d_n11;
        locals.var_qbs_btm_dn14 = assign98960_e151278_d_n14;

    }

    pub(super) fn stamp_transient_block_364(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98980_e151294, assign98980_e151294_d_n0, assign98980_e151294_d_n2, assign98980_e151294_d_n4, assign98980_e151294_d_n5, assign98980_e151294_d_n6, assign98980_e151294_d_n7, assign98980_e151294_d_n8, assign98980_e151294_d_n9, assign98980_e151294_d_n10, assign98980_e151294_d_n11, assign98980_e151294_d_n14,) = {
    if (locals.var_guard2291 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn11, locals.var_qbs_btm_dn14,)
    }
};
        locals.var_qbs_btm = assign98980_e151294;
        locals.var_qbs_btm_dn0 = assign98980_e151294_d_n0;
        locals.var_qbs_btm_dn2 = assign98980_e151294_d_n2;
        locals.var_qbs_btm_dn4 = assign98980_e151294_d_n4;
        locals.var_qbs_btm_dn5 = assign98980_e151294_d_n5;
        locals.var_qbs_btm_dn6 = assign98980_e151294_d_n6;
        locals.var_qbs_btm_dn7 = assign98980_e151294_d_n7;
        locals.var_qbs_btm_dn8 = assign98980_e151294_d_n8;
        locals.var_qbs_btm_dn9 = assign98980_e151294_d_n9;
        locals.var_qbs_btm_dn10 = assign98980_e151294_d_n10;
        locals.var_qbs_btm_dn11 = assign98980_e151294_d_n11;
        locals.var_qbs_btm_dn14 = assign98980_e151294_d_n14;

        let assign99000_e151302: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2294 = assign99000_e151302;

        let assign99010_e151305: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2295 = assign99010_e151305;

        let (assign99020_e151315, assign99020_e151315_d_n0, assign99020_e151315_d_n2, assign99020_e151315_d_n4, assign99020_e151315_d_n5, assign99020_e151315_d_n6, assign99020_e151315_d_n7, assign99020_e151315_d_n8, assign99020_e151315_d_n9, assign99020_e151315_d_n10, assign99020_e151315_d_n11, assign99020_e151315_d_n14,) = {
    if ((locals.var_guard2294 != 0.0) && (locals.var_guard2295 != 0.0)) {
        let assign99020_e151312: f64 = (locals.var_vbs_jct / locals.var_pzbssw);
        let assign99020_e151313: f64 = (1.0 - assign99020_e151312);
        (assign99020_e151313, (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn0) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbssw) - (locals.var_vbs_jct * locals.var_pzbssw_dn2)) / (locals.var_pzbssw * locals.var_pzbssw))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn4) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn5) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn6) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn7) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn8) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn9) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn10) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(((locals.var_vbs_jct_dn11 * locals.var_pzbssw) - (locals.var_vbs_jct * locals.var_pzbssw_dn11)) / (locals.var_pzbssw * locals.var_pzbssw))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn14) / (locals.var_pzbssw * locals.var_pzbssw)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign99020_e151315;
        locals.var_arg_dn0 = assign99020_e151315_d_n0;
        locals.var_arg_dn2 = assign99020_e151315_d_n2;
        locals.var_arg_dn4 = assign99020_e151315_d_n4;
        locals.var_arg_dn5 = assign99020_e151315_d_n5;
        locals.var_arg_dn6 = assign99020_e151315_d_n6;
        locals.var_arg_dn7 = assign99020_e151315_d_n7;
        locals.var_arg_dn8 = assign99020_e151315_d_n8;
        locals.var_arg_dn9 = assign99020_e151315_d_n9;
        locals.var_arg_dn10 = assign99020_e151315_d_n10;
        locals.var_arg_dn11 = assign99020_e151315_d_n11;
        locals.var_arg_dn14 = assign99020_e151315_d_n14;

        let assign99030_e151318: f64 = if p.p527 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2296 = assign99030_e151318;

        let (assign99040_e151329, assign99040_e151329_d_n0, assign99040_e151329_d_n2, assign99040_e151329_d_n4, assign99040_e151329_d_n5, assign99040_e151329_d_n6, assign99040_e151329_d_n7, assign99040_e151329_d_n8, assign99040_e151329_d_n9, assign99040_e151329_d_n10, assign99040_e151329_d_n11, assign99040_e151329_d_n14,) = {
    if (((locals.var_guard2294 != 0.0) && (locals.var_guard2295 != 0.0)) && (locals.var_guard2296 != 0.0)) {
        let assign99040_e151326: f64 = (locals.var_arg).sqrt();
        let assign99040_e151327: f64 = (1.0 / assign99040_e151326);
        (assign99040_e151327, (-((locals.var_arg_dn0 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))), (-((locals.var_arg_dn2 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))), (-((locals.var_arg_dn4 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))), (-((locals.var_arg_dn5 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))), (-((locals.var_arg_dn6 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))), (-((locals.var_arg_dn7 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))), (-((locals.var_arg_dn8 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))), (-((locals.var_arg_dn9 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))), (-((locals.var_arg_dn10 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))), (-((locals.var_arg_dn11 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))), (-((locals.var_arg_dn14 / (2.0 * assign99040_e151326)) / (assign99040_e151326 * assign99040_e151326))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99040_e151329;
        locals.var_sarg_dn0 = assign99040_e151329_d_n0;
        locals.var_sarg_dn2 = assign99040_e151329_d_n2;
        locals.var_sarg_dn4 = assign99040_e151329_d_n4;
        locals.var_sarg_dn5 = assign99040_e151329_d_n5;
        locals.var_sarg_dn6 = assign99040_e151329_d_n6;
        locals.var_sarg_dn7 = assign99040_e151329_d_n7;
        locals.var_sarg_dn8 = assign99040_e151329_d_n8;
        locals.var_sarg_dn9 = assign99040_e151329_d_n9;
        locals.var_sarg_dn10 = assign99040_e151329_d_n10;
        locals.var_sarg_dn11 = assign99040_e151329_d_n11;
        locals.var_sarg_dn14 = assign99040_e151329_d_n14;

        let (assign99050_e151346, assign99050_e151346_d_n0, assign99050_e151346_d_n2, assign99050_e151346_d_n4, assign99050_e151346_d_n5, assign99050_e151346_d_n6, assign99050_e151346_d_n7, assign99050_e151346_d_n8, assign99050_e151346_d_n9, assign99050_e151346_d_n10, assign99050_e151346_d_n11, assign99050_e151346_d_n14,) = {
    if (((locals.var_guard2294 != 0.0) && (locals.var_guard2295 != 0.0)) && (locals.var_guard2296 == 0.0)) {
        let (assign99050_e151344, assign99050_e151344_d_n0, assign99050_e151344_d_n2, assign99050_e151344_d_n4, assign99050_e151344_d_n5, assign99050_e151344_d_n6, assign99050_e151344_d_n7, assign99050_e151344_d_n8, assign99050_e151344_d_n9, assign99050_e151344_d_n10, assign99050_e151344_d_n11, assign99050_e151344_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99050_e151342: f64 = (-p.p527);
                let assign99050_e151343: f64 = (locals.var_arg).powf(assign99050_e151342);
                (assign99050_e151343, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn0)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn2)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn4)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn5)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn6)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn7)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn8)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn9)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn10)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn11)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99050_e151342) as f64).is_finite() && ((assign99050_e151342) as f64).fract() == 0.0 { if assign99050_e151342 == 0.0 { 0.0 } else { (assign99050_e151342 * ((locals.var_arg).powf(assign99050_e151342 - 1.0) * locals.var_arg_dn14)) } } else { (assign99050_e151343 * (assign99050_e151342 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign99050_e151344, assign99050_e151344_d_n0, assign99050_e151344_d_n2, assign99050_e151344_d_n4, assign99050_e151344_d_n5, assign99050_e151344_d_n6, assign99050_e151344_d_n7, assign99050_e151344_d_n8, assign99050_e151344_d_n9, assign99050_e151344_d_n10, assign99050_e151344_d_n11, assign99050_e151344_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99050_e151346;
        locals.var_sarg_dn0 = assign99050_e151346_d_n0;
        locals.var_sarg_dn2 = assign99050_e151346_d_n2;
        locals.var_sarg_dn4 = assign99050_e151346_d_n4;
        locals.var_sarg_dn5 = assign99050_e151346_d_n5;
        locals.var_sarg_dn6 = assign99050_e151346_d_n6;
        locals.var_sarg_dn7 = assign99050_e151346_d_n7;
        locals.var_sarg_dn8 = assign99050_e151346_d_n8;
        locals.var_sarg_dn9 = assign99050_e151346_d_n9;
        locals.var_sarg_dn10 = assign99050_e151346_d_n10;
        locals.var_sarg_dn11 = assign99050_e151346_d_n11;
        locals.var_sarg_dn14 = assign99050_e151346_d_n14;

        let (assign99060_e151364, assign99060_e151364_d_n0, assign99060_e151364_d_n2, assign99060_e151364_d_n4, assign99060_e151364_d_n5, assign99060_e151364_d_n6, assign99060_e151364_d_n7, assign99060_e151364_d_n8, assign99060_e151364_d_n9, assign99060_e151364_d_n10, assign99060_e151364_d_n11, assign99060_e151364_d_n14,) = {
    if ((locals.var_guard2294 != 0.0) && (locals.var_guard2295 != 0.0)) {
        let assign99060_e151352: f64 = (locals.var_pzbssw * locals.var_czbssw);
        let assign99060_e151356: f64 = (locals.var_arg * locals.var_sarg);
        let assign99060_e151357: f64 = (1.0 - assign99060_e151356);
        let assign99060_e151358: f64 = (assign99060_e151352 * assign99060_e151357);
        let assign99060_e151361: f64 = (1.0 - p.p527);
        let assign99060_e151362: f64 = (assign99060_e151358 / assign99060_e151361);
        (assign99060_e151362, (((((locals.var_pzbssw_dn0 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn0)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99060_e151361), (((((locals.var_pzbssw_dn2 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn2)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99060_e151361), (((((locals.var_pzbssw_dn4 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn4)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99060_e151361), (((((locals.var_pzbssw_dn5 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn5)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99060_e151361), (((((locals.var_pzbssw_dn6 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn6)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99060_e151361), (((((locals.var_pzbssw_dn7 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn7)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99060_e151361), (((((locals.var_pzbssw_dn8 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn8)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99060_e151361), (((((locals.var_pzbssw_dn9 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn9)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99060_e151361), (((((locals.var_pzbssw_dn10 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn10)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99060_e151361), (((((locals.var_pzbssw_dn11 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn11)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign99060_e151361), (((((locals.var_pzbssw_dn14 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn14)) * assign99060_e151357) + (assign99060_e151352 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign99060_e151361),)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn11, locals.var_qbs_sws_dn14,)
    }
};
        locals.var_qbs_sws = assign99060_e151364;
        locals.var_qbs_sws_dn0 = assign99060_e151364_d_n0;
        locals.var_qbs_sws_dn2 = assign99060_e151364_d_n2;
        locals.var_qbs_sws_dn4 = assign99060_e151364_d_n4;
        locals.var_qbs_sws_dn5 = assign99060_e151364_d_n5;
        locals.var_qbs_sws_dn6 = assign99060_e151364_d_n6;
        locals.var_qbs_sws_dn7 = assign99060_e151364_d_n7;
        locals.var_qbs_sws_dn8 = assign99060_e151364_d_n8;
        locals.var_qbs_sws_dn9 = assign99060_e151364_d_n9;
        locals.var_qbs_sws_dn10 = assign99060_e151364_d_n10;
        locals.var_qbs_sws_dn11 = assign99060_e151364_d_n11;
        locals.var_qbs_sws_dn14 = assign99060_e151364_d_n14;

        let (assign99080_e151379, assign99080_e151379_d_n0, assign99080_e151379_d_n2, assign99080_e151379_d_n4, assign99080_e151379_d_n5, assign99080_e151379_d_n6, assign99080_e151379_d_n7, assign99080_e151379_d_n8, assign99080_e151379_d_n9, assign99080_e151379_d_n10, assign99080_e151379_d_n11, assign99080_e151379_d_n14,) = {
    if ((locals.var_guard2294 != 0.0) && (locals.var_guard2295 == 0.0)) {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign99080_e151379;
        locals.var_t1_dn0 = assign99080_e151379_d_n0;
        locals.var_t1_dn2 = assign99080_e151379_d_n2;
        locals.var_t1_dn4 = assign99080_e151379_d_n4;
        locals.var_t1_dn5 = assign99080_e151379_d_n5;
        locals.var_t1_dn6 = assign99080_e151379_d_n6;
        locals.var_t1_dn7 = assign99080_e151379_d_n7;
        locals.var_t1_dn8 = assign99080_e151379_d_n8;
        locals.var_t1_dn9 = assign99080_e151379_d_n9;
        locals.var_t1_dn10 = assign99080_e151379_d_n10;
        locals.var_t1_dn11 = assign99080_e151379_d_n11;
        locals.var_t1_dn14 = assign99080_e151379_d_n14;

        let (assign99090_e151390, assign99090_e151390_d_n0, assign99090_e151390_d_n2, assign99090_e151390_d_n4, assign99090_e151390_d_n5, assign99090_e151390_d_n6, assign99090_e151390_d_n7, assign99090_e151390_d_n8, assign99090_e151390_d_n9, assign99090_e151390_d_n10, assign99090_e151390_d_n11, assign99090_e151390_d_n14,) = {
    if ((locals.var_guard2294 != 0.0) && (locals.var_guard2295 == 0.0)) {
        let assign99090_e151386: f64 = (locals.var_czbssw * p.p527);
        let assign99090_e151388: f64 = (assign99090_e151386 / locals.var_pzbssw);
        (assign99090_e151388, ((((locals.var_czbssw_dn0 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn0)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn2 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn2)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn4 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn4)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn5 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn5)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn6 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn6)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn7 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn7)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn8 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn8)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn9 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn9)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn10 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn10)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn11 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn11)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn14 * p.p527) * locals.var_pzbssw) - (assign99090_e151386 * locals.var_pzbssw_dn14)) / (locals.var_pzbssw * locals.var_pzbssw)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign99090_e151390;
        locals.var_t2_dn0 = assign99090_e151390_d_n0;
        locals.var_t2_dn2 = assign99090_e151390_d_n2;
        locals.var_t2_dn4 = assign99090_e151390_d_n4;
        locals.var_t2_dn5 = assign99090_e151390_d_n5;
        locals.var_t2_dn6 = assign99090_e151390_d_n6;
        locals.var_t2_dn7 = assign99090_e151390_d_n7;
        locals.var_t2_dn8 = assign99090_e151390_d_n8;
        locals.var_t2_dn9 = assign99090_e151390_d_n9;
        locals.var_t2_dn10 = assign99090_e151390_d_n10;
        locals.var_t2_dn11 = assign99090_e151390_d_n11;
        locals.var_t2_dn14 = assign99090_e151390_d_n14;

        let (assign99100_e151405, assign99100_e151405_d_n0, assign99100_e151405_d_n2, assign99100_e151405_d_n4, assign99100_e151405_d_n5, assign99100_e151405_d_n6, assign99100_e151405_d_n7, assign99100_e151405_d_n8, assign99100_e151405_d_n9, assign99100_e151405_d_n10, assign99100_e151405_d_n11, assign99100_e151405_d_n14,) = {
    if ((locals.var_guard2294 != 0.0) && (locals.var_guard2295 == 0.0)) {
        let assign99100_e151399: f64 = (locals.var_vbs_jct * 0.5);
        let assign99100_e151401: f64 = (assign99100_e151399 * locals.var_t2);
        let assign99100_e151402: f64 = (locals.var_t1 + assign99100_e151401);
        let assign99100_e151403: f64 = (locals.var_vbs_jct * assign99100_e151402);
        (assign99100_e151403, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign99100_e151399 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign99100_e151402) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign99100_e151399 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign99100_e151399 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign99100_e151399 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign99100_e151399 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign99100_e151399 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign99100_e151399 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign99100_e151399 * locals.var_t2_dn9))), (locals.var_vbs_jct * (locals.var_t1_dn10 + (assign99100_e151399 * locals.var_t2_dn10))), ((locals.var_vbs_jct_dn11 * assign99100_e151402) + (locals.var_vbs_jct * (locals.var_t1_dn11 + (((locals.var_vbs_jct_dn11 * 0.5) * locals.var_t2) + (assign99100_e151399 * locals.var_t2_dn11))))), (locals.var_vbs_jct * (locals.var_t1_dn14 + (assign99100_e151399 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn11, locals.var_qbs_sws_dn14,)
    }
};
        locals.var_qbs_sws = assign99100_e151405;
        locals.var_qbs_sws_dn0 = assign99100_e151405_d_n0;
        locals.var_qbs_sws_dn2 = assign99100_e151405_d_n2;
        locals.var_qbs_sws_dn4 = assign99100_e151405_d_n4;
        locals.var_qbs_sws_dn5 = assign99100_e151405_d_n5;
        locals.var_qbs_sws_dn6 = assign99100_e151405_d_n6;
        locals.var_qbs_sws_dn7 = assign99100_e151405_d_n7;
        locals.var_qbs_sws_dn8 = assign99100_e151405_d_n8;
        locals.var_qbs_sws_dn9 = assign99100_e151405_d_n9;
        locals.var_qbs_sws_dn10 = assign99100_e151405_d_n10;
        locals.var_qbs_sws_dn11 = assign99100_e151405_d_n11;
        locals.var_qbs_sws_dn14 = assign99100_e151405_d_n14;

        let (assign99120_e151421, assign99120_e151421_d_n0, assign99120_e151421_d_n2, assign99120_e151421_d_n4, assign99120_e151421_d_n5, assign99120_e151421_d_n6, assign99120_e151421_d_n7, assign99120_e151421_d_n8, assign99120_e151421_d_n9, assign99120_e151421_d_n10, assign99120_e151421_d_n11, assign99120_e151421_d_n14,) = {
    if (locals.var_guard2294 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn11, locals.var_qbs_sws_dn14,)
    }
};
        locals.var_qbs_sws = assign99120_e151421;
        locals.var_qbs_sws_dn0 = assign99120_e151421_d_n0;
        locals.var_qbs_sws_dn2 = assign99120_e151421_d_n2;
        locals.var_qbs_sws_dn4 = assign99120_e151421_d_n4;
        locals.var_qbs_sws_dn5 = assign99120_e151421_d_n5;
        locals.var_qbs_sws_dn6 = assign99120_e151421_d_n6;
        locals.var_qbs_sws_dn7 = assign99120_e151421_d_n7;
        locals.var_qbs_sws_dn8 = assign99120_e151421_d_n8;
        locals.var_qbs_sws_dn9 = assign99120_e151421_d_n9;
        locals.var_qbs_sws_dn10 = assign99120_e151421_d_n10;
        locals.var_qbs_sws_dn11 = assign99120_e151421_d_n11;
        locals.var_qbs_sws_dn14 = assign99120_e151421_d_n14;

        let assign99140_e151429: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2297 = assign99140_e151429;

        let assign99150_e151432: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2298 = assign99150_e151432;

        let assign99160_e151435: f64 = if locals.var_vbsi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2299 = assign99160_e151435;

        let (assign99170_e151447, assign99170_e151447_d_n0, assign99170_e151447_d_n2, assign99170_e151447_d_n4, assign99170_e151447_d_n5, assign99170_e151447_d_n6, assign99170_e151447_d_n7, assign99170_e151447_d_n8, assign99170_e151447_d_n9, assign99170_e151447_d_n10, assign99170_e151447_d_n11, assign99170_e151447_d_n14,) = {
    if (((locals.var_guard2297 != 0.0) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 != 0.0)) {
        let assign99170_e151444: f64 = (locals.var_vbsi_jct / locals.var_pzbsswg);
        let assign99170_e151445: f64 = (1.0 - assign99170_e151444);
        (assign99170_e151445, (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn0) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn2) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn4) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn5) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn6) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn7) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbsi_jct_dn8 * locals.var_pzbsswg) - (locals.var_vbsi_jct * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(((locals.var_vbsi_jct_dn9 * locals.var_pzbsswg) - (locals.var_vbsi_jct * locals.var_pzbsswg_dn9)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn10) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn11) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn14) / (locals.var_pzbsswg * locals.var_pzbsswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign99170_e151447;
        locals.var_arg_dn0 = assign99170_e151447_d_n0;
        locals.var_arg_dn2 = assign99170_e151447_d_n2;
        locals.var_arg_dn4 = assign99170_e151447_d_n4;
        locals.var_arg_dn5 = assign99170_e151447_d_n5;
        locals.var_arg_dn6 = assign99170_e151447_d_n6;
        locals.var_arg_dn7 = assign99170_e151447_d_n7;
        locals.var_arg_dn8 = assign99170_e151447_d_n8;
        locals.var_arg_dn9 = assign99170_e151447_d_n9;
        locals.var_arg_dn10 = assign99170_e151447_d_n10;
        locals.var_arg_dn11 = assign99170_e151447_d_n11;
        locals.var_arg_dn14 = assign99170_e151447_d_n14;

        let assign99180_e151450: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2300 = assign99180_e151450;

        let (assign99190_e151463, assign99190_e151463_d_n0, assign99190_e151463_d_n2, assign99190_e151463_d_n4, assign99190_e151463_d_n5, assign99190_e151463_d_n6, assign99190_e151463_d_n7, assign99190_e151463_d_n8, assign99190_e151463_d_n9, assign99190_e151463_d_n10, assign99190_e151463_d_n11, assign99190_e151463_d_n14,) = {
    if ((((locals.var_guard2297 != 0.0) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 != 0.0)) && (locals.var_guard2300 != 0.0)) {
        let assign99190_e151460: f64 = (locals.var_arg).sqrt();
        let assign99190_e151461: f64 = (1.0 / assign99190_e151460);
        (assign99190_e151461, (-((locals.var_arg_dn0 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))), (-((locals.var_arg_dn2 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))), (-((locals.var_arg_dn4 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))), (-((locals.var_arg_dn5 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))), (-((locals.var_arg_dn6 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))), (-((locals.var_arg_dn7 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))), (-((locals.var_arg_dn8 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))), (-((locals.var_arg_dn9 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))), (-((locals.var_arg_dn10 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))), (-((locals.var_arg_dn11 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))), (-((locals.var_arg_dn14 / (2.0 * assign99190_e151460)) / (assign99190_e151460 * assign99190_e151460))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99190_e151463;
        locals.var_sarg_dn0 = assign99190_e151463_d_n0;
        locals.var_sarg_dn2 = assign99190_e151463_d_n2;
        locals.var_sarg_dn4 = assign99190_e151463_d_n4;
        locals.var_sarg_dn5 = assign99190_e151463_d_n5;
        locals.var_sarg_dn6 = assign99190_e151463_d_n6;
        locals.var_sarg_dn7 = assign99190_e151463_d_n7;
        locals.var_sarg_dn8 = assign99190_e151463_d_n8;
        locals.var_sarg_dn9 = assign99190_e151463_d_n9;
        locals.var_sarg_dn10 = assign99190_e151463_d_n10;
        locals.var_sarg_dn11 = assign99190_e151463_d_n11;
        locals.var_sarg_dn14 = assign99190_e151463_d_n14;

        let (assign99200_e151482, assign99200_e151482_d_n0, assign99200_e151482_d_n2, assign99200_e151482_d_n4, assign99200_e151482_d_n5, assign99200_e151482_d_n6, assign99200_e151482_d_n7, assign99200_e151482_d_n8, assign99200_e151482_d_n9, assign99200_e151482_d_n10, assign99200_e151482_d_n11, assign99200_e151482_d_n14,) = {
    if ((((locals.var_guard2297 != 0.0) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 != 0.0)) && (locals.var_guard2300 == 0.0)) {
        let (assign99200_e151480, assign99200_e151480_d_n0, assign99200_e151480_d_n2, assign99200_e151480_d_n4, assign99200_e151480_d_n5, assign99200_e151480_d_n6, assign99200_e151480_d_n7, assign99200_e151480_d_n8, assign99200_e151480_d_n9, assign99200_e151480_d_n10, assign99200_e151480_d_n11, assign99200_e151480_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99200_e151478: f64 = (-p.p528);
                let assign99200_e151479: f64 = (locals.var_arg).powf(assign99200_e151478);
                (assign99200_e151479, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn0)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn2)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn4)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn5)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn6)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn7)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn8)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn9)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn10)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn11)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99200_e151478) as f64).is_finite() && ((assign99200_e151478) as f64).fract() == 0.0 { if assign99200_e151478 == 0.0 { 0.0 } else { (assign99200_e151478 * ((locals.var_arg).powf(assign99200_e151478 - 1.0) * locals.var_arg_dn14)) } } else { (assign99200_e151479 * (assign99200_e151478 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign99200_e151480, assign99200_e151480_d_n0, assign99200_e151480_d_n2, assign99200_e151480_d_n4, assign99200_e151480_d_n5, assign99200_e151480_d_n6, assign99200_e151480_d_n7, assign99200_e151480_d_n8, assign99200_e151480_d_n9, assign99200_e151480_d_n10, assign99200_e151480_d_n11, assign99200_e151480_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99200_e151482;
        locals.var_sarg_dn0 = assign99200_e151482_d_n0;
        locals.var_sarg_dn2 = assign99200_e151482_d_n2;
        locals.var_sarg_dn4 = assign99200_e151482_d_n4;
        locals.var_sarg_dn5 = assign99200_e151482_d_n5;
        locals.var_sarg_dn6 = assign99200_e151482_d_n6;
        locals.var_sarg_dn7 = assign99200_e151482_d_n7;
        locals.var_sarg_dn8 = assign99200_e151482_d_n8;
        locals.var_sarg_dn9 = assign99200_e151482_d_n9;
        locals.var_sarg_dn10 = assign99200_e151482_d_n10;
        locals.var_sarg_dn11 = assign99200_e151482_d_n11;
        locals.var_sarg_dn14 = assign99200_e151482_d_n14;

        let (assign99210_e151502, assign99210_e151502_d_n0, assign99210_e151502_d_n2, assign99210_e151502_d_n4, assign99210_e151502_d_n5, assign99210_e151502_d_n6, assign99210_e151502_d_n7, assign99210_e151502_d_n8, assign99210_e151502_d_n9, assign99210_e151502_d_n10, assign99210_e151502_d_n11, assign99210_e151502_d_n14,) = {
    if (((locals.var_guard2297 != 0.0) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 != 0.0)) {
        let assign99210_e151490: f64 = (locals.var_pzbsswg * locals.var_czbsswg);
        let assign99210_e151494: f64 = (locals.var_arg * locals.var_sarg);
        let assign99210_e151495: f64 = (1.0 - assign99210_e151494);
        let assign99210_e151496: f64 = (assign99210_e151490 * assign99210_e151495);
        let assign99210_e151499: f64 = (1.0 - p.p528);
        let assign99210_e151500: f64 = (assign99210_e151496 / assign99210_e151499);
        (assign99210_e151500, (((((locals.var_pzbsswg_dn0 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn0)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99210_e151499), (((((locals.var_pzbsswg_dn2 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn2)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99210_e151499), (((((locals.var_pzbsswg_dn4 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn4)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99210_e151499), (((((locals.var_pzbsswg_dn5 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn5)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99210_e151499), (((((locals.var_pzbsswg_dn6 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn6)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99210_e151499), (((((locals.var_pzbsswg_dn7 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn7)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99210_e151499), (((((locals.var_pzbsswg_dn8 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn8)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99210_e151499), (((((locals.var_pzbsswg_dn9 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn9)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99210_e151499), (((((locals.var_pzbsswg_dn10 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn10)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99210_e151499), (((((locals.var_pzbsswg_dn11 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn11)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign99210_e151499), (((((locals.var_pzbsswg_dn14 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn14)) * assign99210_e151495) + (assign99210_e151490 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign99210_e151499),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99210_e151502;
        locals.var_qbs_swg_dn0 = assign99210_e151502_d_n0;
        locals.var_qbs_swg_dn2 = assign99210_e151502_d_n2;
        locals.var_qbs_swg_dn4 = assign99210_e151502_d_n4;
        locals.var_qbs_swg_dn5 = assign99210_e151502_d_n5;
        locals.var_qbs_swg_dn6 = assign99210_e151502_d_n6;
        locals.var_qbs_swg_dn7 = assign99210_e151502_d_n7;
        locals.var_qbs_swg_dn8 = assign99210_e151502_d_n8;
        locals.var_qbs_swg_dn9 = assign99210_e151502_d_n9;
        locals.var_qbs_swg_dn10 = assign99210_e151502_d_n10;
        locals.var_qbs_swg_dn11 = assign99210_e151502_d_n11;
        locals.var_qbs_swg_dn14 = assign99210_e151502_d_n14;

        let (assign99230_e151521, assign99230_e151521_d_n0, assign99230_e151521_d_n2, assign99230_e151521_d_n4, assign99230_e151521_d_n5, assign99230_e151521_d_n6, assign99230_e151521_d_n7, assign99230_e151521_d_n8, assign99230_e151521_d_n9, assign99230_e151521_d_n10, assign99230_e151521_d_n11, assign99230_e151521_d_n14,) = {
    if (((locals.var_guard2297 != 0.0) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 == 0.0)) {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign99230_e151521;
        locals.var_t1_dn0 = assign99230_e151521_d_n0;
        locals.var_t1_dn2 = assign99230_e151521_d_n2;
        locals.var_t1_dn4 = assign99230_e151521_d_n4;
        locals.var_t1_dn5 = assign99230_e151521_d_n5;
        locals.var_t1_dn6 = assign99230_e151521_d_n6;
        locals.var_t1_dn7 = assign99230_e151521_d_n7;
        locals.var_t1_dn8 = assign99230_e151521_d_n8;
        locals.var_t1_dn9 = assign99230_e151521_d_n9;
        locals.var_t1_dn10 = assign99230_e151521_d_n10;
        locals.var_t1_dn11 = assign99230_e151521_d_n11;
        locals.var_t1_dn14 = assign99230_e151521_d_n14;

        let (assign99240_e151534, assign99240_e151534_d_n0, assign99240_e151534_d_n2, assign99240_e151534_d_n4, assign99240_e151534_d_n5, assign99240_e151534_d_n6, assign99240_e151534_d_n7, assign99240_e151534_d_n8, assign99240_e151534_d_n9, assign99240_e151534_d_n10, assign99240_e151534_d_n11, assign99240_e151534_d_n14,) = {
    if (((locals.var_guard2297 != 0.0) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 == 0.0)) {
        let assign99240_e151530: f64 = (locals.var_czbsswg * p.p528);
        let assign99240_e151532: f64 = (assign99240_e151530 / locals.var_pzbsswg);
        (assign99240_e151532, ((((locals.var_czbsswg_dn0 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn0)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn2 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn4 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn4)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn5 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn5)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn6 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn6)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn7 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn8 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn9 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn9)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn10 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn10)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn11 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn11)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn14 * p.p528) * locals.var_pzbsswg) - (assign99240_e151530 * locals.var_pzbsswg_dn14)) / (locals.var_pzbsswg * locals.var_pzbsswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign99240_e151534;
        locals.var_t2_dn0 = assign99240_e151534_d_n0;
        locals.var_t2_dn2 = assign99240_e151534_d_n2;
        locals.var_t2_dn4 = assign99240_e151534_d_n4;
        locals.var_t2_dn5 = assign99240_e151534_d_n5;
        locals.var_t2_dn6 = assign99240_e151534_d_n6;
        locals.var_t2_dn7 = assign99240_e151534_d_n7;
        locals.var_t2_dn8 = assign99240_e151534_d_n8;
        locals.var_t2_dn9 = assign99240_e151534_d_n9;
        locals.var_t2_dn10 = assign99240_e151534_d_n10;
        locals.var_t2_dn11 = assign99240_e151534_d_n11;
        locals.var_t2_dn14 = assign99240_e151534_d_n14;

        let (assign99250_e151551, assign99250_e151551_d_n0, assign99250_e151551_d_n2, assign99250_e151551_d_n4, assign99250_e151551_d_n5, assign99250_e151551_d_n6, assign99250_e151551_d_n7, assign99250_e151551_d_n8, assign99250_e151551_d_n9, assign99250_e151551_d_n10, assign99250_e151551_d_n11, assign99250_e151551_d_n14,) = {
    if (((locals.var_guard2297 != 0.0) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 == 0.0)) {
        let assign99250_e151545: f64 = (locals.var_vbsi_jct * 0.5);
        let assign99250_e151547: f64 = (assign99250_e151545 * locals.var_t2);
        let assign99250_e151548: f64 = (locals.var_t1 + assign99250_e151547);
        let assign99250_e151549: f64 = (locals.var_vbsi_jct * assign99250_e151548);
        (assign99250_e151549, (locals.var_vbsi_jct * (locals.var_t1_dn0 + (assign99250_e151545 * locals.var_t2_dn0))), (locals.var_vbsi_jct * (locals.var_t1_dn2 + (assign99250_e151545 * locals.var_t2_dn2))), (locals.var_vbsi_jct * (locals.var_t1_dn4 + (assign99250_e151545 * locals.var_t2_dn4))), (locals.var_vbsi_jct * (locals.var_t1_dn5 + (assign99250_e151545 * locals.var_t2_dn5))), (locals.var_vbsi_jct * (locals.var_t1_dn6 + (assign99250_e151545 * locals.var_t2_dn6))), (locals.var_vbsi_jct * (locals.var_t1_dn7 + (assign99250_e151545 * locals.var_t2_dn7))), ((locals.var_vbsi_jct_dn8 * assign99250_e151548) + (locals.var_vbsi_jct * (locals.var_t1_dn8 + (((locals.var_vbsi_jct_dn8 * 0.5) * locals.var_t2) + (assign99250_e151545 * locals.var_t2_dn8))))), ((locals.var_vbsi_jct_dn9 * assign99250_e151548) + (locals.var_vbsi_jct * (locals.var_t1_dn9 + (((locals.var_vbsi_jct_dn9 * 0.5) * locals.var_t2) + (assign99250_e151545 * locals.var_t2_dn9))))), (locals.var_vbsi_jct * (locals.var_t1_dn10 + (assign99250_e151545 * locals.var_t2_dn10))), (locals.var_vbsi_jct * (locals.var_t1_dn11 + (assign99250_e151545 * locals.var_t2_dn11))), (locals.var_vbsi_jct * (locals.var_t1_dn14 + (assign99250_e151545 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99250_e151551;
        locals.var_qbs_swg_dn0 = assign99250_e151551_d_n0;
        locals.var_qbs_swg_dn2 = assign99250_e151551_d_n2;
        locals.var_qbs_swg_dn4 = assign99250_e151551_d_n4;
        locals.var_qbs_swg_dn5 = assign99250_e151551_d_n5;
        locals.var_qbs_swg_dn6 = assign99250_e151551_d_n6;
        locals.var_qbs_swg_dn7 = assign99250_e151551_d_n7;
        locals.var_qbs_swg_dn8 = assign99250_e151551_d_n8;
        locals.var_qbs_swg_dn9 = assign99250_e151551_d_n9;
        locals.var_qbs_swg_dn10 = assign99250_e151551_d_n10;
        locals.var_qbs_swg_dn11 = assign99250_e151551_d_n11;
        locals.var_qbs_swg_dn14 = assign99250_e151551_d_n14;

        let (assign99270_e151571, assign99270_e151571_d_n0, assign99270_e151571_d_n2, assign99270_e151571_d_n4, assign99270_e151571_d_n5, assign99270_e151571_d_n6, assign99270_e151571_d_n7, assign99270_e151571_d_n8, assign99270_e151571_d_n9, assign99270_e151571_d_n10, assign99270_e151571_d_n11, assign99270_e151571_d_n14,) = {
    if ((locals.var_guard2297 != 0.0) && (locals.var_guard2298 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99270_e151571;
        locals.var_qbs_swg_dn0 = assign99270_e151571_d_n0;
        locals.var_qbs_swg_dn2 = assign99270_e151571_d_n2;
        locals.var_qbs_swg_dn4 = assign99270_e151571_d_n4;
        locals.var_qbs_swg_dn5 = assign99270_e151571_d_n5;
        locals.var_qbs_swg_dn6 = assign99270_e151571_d_n6;
        locals.var_qbs_swg_dn7 = assign99270_e151571_d_n7;
        locals.var_qbs_swg_dn8 = assign99270_e151571_d_n8;
        locals.var_qbs_swg_dn9 = assign99270_e151571_d_n9;
        locals.var_qbs_swg_dn10 = assign99270_e151571_d_n10;
        locals.var_qbs_swg_dn11 = assign99270_e151571_d_n11;
        locals.var_qbs_swg_dn14 = assign99270_e151571_d_n14;

        let assign99290_e151581: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2301 = assign99290_e151581;

        let assign99300_e151584: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2302 = assign99300_e151584;

        let (assign99310_e151597, assign99310_e151597_d_n0, assign99310_e151597_d_n2, assign99310_e151597_d_n4, assign99310_e151597_d_n5, assign99310_e151597_d_n6, assign99310_e151597_d_n7, assign99310_e151597_d_n8, assign99310_e151597_d_n9, assign99310_e151597_d_n10, assign99310_e151597_d_n11, assign99310_e151597_d_n14,) = {
    if (((locals.var_guard2297 == 0.0) && (locals.var_guard2301 != 0.0)) && (locals.var_guard2302 != 0.0)) {
        let assign99310_e151594: f64 = (locals.var_vbs_jct / locals.var_pzbsswg);
        let assign99310_e151595: f64 = (1.0 - assign99310_e151594);
        (assign99310_e151595, (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn0) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbsswg) - (locals.var_vbs_jct * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn4) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn5) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn6) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn7) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn8) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn9) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn10) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbs_jct_dn11 * locals.var_pzbsswg) - (locals.var_vbs_jct * locals.var_pzbsswg_dn11)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn14) / (locals.var_pzbsswg * locals.var_pzbsswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign99310_e151597;
        locals.var_arg_dn0 = assign99310_e151597_d_n0;
        locals.var_arg_dn2 = assign99310_e151597_d_n2;
        locals.var_arg_dn4 = assign99310_e151597_d_n4;
        locals.var_arg_dn5 = assign99310_e151597_d_n5;
        locals.var_arg_dn6 = assign99310_e151597_d_n6;
        locals.var_arg_dn7 = assign99310_e151597_d_n7;
        locals.var_arg_dn8 = assign99310_e151597_d_n8;
        locals.var_arg_dn9 = assign99310_e151597_d_n9;
        locals.var_arg_dn10 = assign99310_e151597_d_n10;
        locals.var_arg_dn11 = assign99310_e151597_d_n11;
        locals.var_arg_dn14 = assign99310_e151597_d_n14;

        let assign99320_e151600: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2303 = assign99320_e151600;

        let (assign99330_e151614, assign99330_e151614_d_n0, assign99330_e151614_d_n2, assign99330_e151614_d_n4, assign99330_e151614_d_n5, assign99330_e151614_d_n6, assign99330_e151614_d_n7, assign99330_e151614_d_n8, assign99330_e151614_d_n9, assign99330_e151614_d_n10, assign99330_e151614_d_n11, assign99330_e151614_d_n14,) = {
    if ((((locals.var_guard2297 == 0.0) && (locals.var_guard2301 != 0.0)) && (locals.var_guard2302 != 0.0)) && (locals.var_guard2303 != 0.0)) {
        let assign99330_e151611: f64 = (locals.var_arg).sqrt();
        let assign99330_e151612: f64 = (1.0 / assign99330_e151611);
        (assign99330_e151612, (-((locals.var_arg_dn0 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))), (-((locals.var_arg_dn2 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))), (-((locals.var_arg_dn4 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))), (-((locals.var_arg_dn5 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))), (-((locals.var_arg_dn6 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))), (-((locals.var_arg_dn7 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))), (-((locals.var_arg_dn8 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))), (-((locals.var_arg_dn9 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))), (-((locals.var_arg_dn10 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))), (-((locals.var_arg_dn11 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))), (-((locals.var_arg_dn14 / (2.0 * assign99330_e151611)) / (assign99330_e151611 * assign99330_e151611))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99330_e151614;
        locals.var_sarg_dn0 = assign99330_e151614_d_n0;
        locals.var_sarg_dn2 = assign99330_e151614_d_n2;
        locals.var_sarg_dn4 = assign99330_e151614_d_n4;
        locals.var_sarg_dn5 = assign99330_e151614_d_n5;
        locals.var_sarg_dn6 = assign99330_e151614_d_n6;
        locals.var_sarg_dn7 = assign99330_e151614_d_n7;
        locals.var_sarg_dn8 = assign99330_e151614_d_n8;
        locals.var_sarg_dn9 = assign99330_e151614_d_n9;
        locals.var_sarg_dn10 = assign99330_e151614_d_n10;
        locals.var_sarg_dn11 = assign99330_e151614_d_n11;
        locals.var_sarg_dn14 = assign99330_e151614_d_n14;

        let (assign99340_e151634, assign99340_e151634_d_n0, assign99340_e151634_d_n2, assign99340_e151634_d_n4, assign99340_e151634_d_n5, assign99340_e151634_d_n6, assign99340_e151634_d_n7, assign99340_e151634_d_n8, assign99340_e151634_d_n9, assign99340_e151634_d_n10, assign99340_e151634_d_n11, assign99340_e151634_d_n14,) = {
    if ((((locals.var_guard2297 == 0.0) && (locals.var_guard2301 != 0.0)) && (locals.var_guard2302 != 0.0)) && (locals.var_guard2303 == 0.0)) {
        let (assign99340_e151632, assign99340_e151632_d_n0, assign99340_e151632_d_n2, assign99340_e151632_d_n4, assign99340_e151632_d_n5, assign99340_e151632_d_n6, assign99340_e151632_d_n7, assign99340_e151632_d_n8, assign99340_e151632_d_n9, assign99340_e151632_d_n10, assign99340_e151632_d_n11, assign99340_e151632_d_n14,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99340_e151630: f64 = (-p.p528);
                let assign99340_e151631: f64 = (locals.var_arg).powf(assign99340_e151630);
                (assign99340_e151631, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn0)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn2)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn4)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn5)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn6)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn7)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn8)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn9)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn10)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn11)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn11 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99340_e151630) as f64).is_finite() && ((assign99340_e151630) as f64).fract() == 0.0 { if assign99340_e151630 == 0.0 { 0.0 } else { (assign99340_e151630 * ((locals.var_arg).powf(assign99340_e151630 - 1.0) * locals.var_arg_dn14)) } } else { (assign99340_e151631 * (assign99340_e151630 * (locals.var_arg_dn14 / locals.var_arg))) },)
            }
        };
        (assign99340_e151632, assign99340_e151632_d_n0, assign99340_e151632_d_n2, assign99340_e151632_d_n4, assign99340_e151632_d_n5, assign99340_e151632_d_n6, assign99340_e151632_d_n7, assign99340_e151632_d_n8, assign99340_e151632_d_n9, assign99340_e151632_d_n10, assign99340_e151632_d_n11, assign99340_e151632_d_n14,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign99340_e151634;
        locals.var_sarg_dn0 = assign99340_e151634_d_n0;
        locals.var_sarg_dn2 = assign99340_e151634_d_n2;
        locals.var_sarg_dn4 = assign99340_e151634_d_n4;
        locals.var_sarg_dn5 = assign99340_e151634_d_n5;
        locals.var_sarg_dn6 = assign99340_e151634_d_n6;
        locals.var_sarg_dn7 = assign99340_e151634_d_n7;
        locals.var_sarg_dn8 = assign99340_e151634_d_n8;
        locals.var_sarg_dn9 = assign99340_e151634_d_n9;
        locals.var_sarg_dn10 = assign99340_e151634_d_n10;
        locals.var_sarg_dn11 = assign99340_e151634_d_n11;
        locals.var_sarg_dn14 = assign99340_e151634_d_n14;

    }

    pub(super) fn stamp_transient_block_365(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign99350_e151655, assign99350_e151655_d_n0, assign99350_e151655_d_n2, assign99350_e151655_d_n4, assign99350_e151655_d_n5, assign99350_e151655_d_n6, assign99350_e151655_d_n7, assign99350_e151655_d_n8, assign99350_e151655_d_n9, assign99350_e151655_d_n10, assign99350_e151655_d_n11, assign99350_e151655_d_n14,) = {
    if (((locals.var_guard2297 == 0.0) && (locals.var_guard2301 != 0.0)) && (locals.var_guard2302 != 0.0)) {
        let assign99350_e151643: f64 = (locals.var_pzbsswg * locals.var_czbsswg);
        let assign99350_e151647: f64 = (locals.var_arg * locals.var_sarg);
        let assign99350_e151648: f64 = (1.0 - assign99350_e151647);
        let assign99350_e151649: f64 = (assign99350_e151643 * assign99350_e151648);
        let assign99350_e151652: f64 = (1.0 - p.p528);
        let assign99350_e151653: f64 = (assign99350_e151649 / assign99350_e151652);
        (assign99350_e151653, (((((locals.var_pzbsswg_dn0 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn0)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99350_e151652), (((((locals.var_pzbsswg_dn2 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn2)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99350_e151652), (((((locals.var_pzbsswg_dn4 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn4)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99350_e151652), (((((locals.var_pzbsswg_dn5 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn5)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99350_e151652), (((((locals.var_pzbsswg_dn6 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn6)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99350_e151652), (((((locals.var_pzbsswg_dn7 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn7)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99350_e151652), (((((locals.var_pzbsswg_dn8 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn8)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99350_e151652), (((((locals.var_pzbsswg_dn9 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn9)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99350_e151652), (((((locals.var_pzbsswg_dn10 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn10)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99350_e151652), (((((locals.var_pzbsswg_dn11 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn11)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign99350_e151652), (((((locals.var_pzbsswg_dn14 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn14)) * assign99350_e151648) + (assign99350_e151643 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign99350_e151652),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99350_e151655;
        locals.var_qbs_swg_dn0 = assign99350_e151655_d_n0;
        locals.var_qbs_swg_dn2 = assign99350_e151655_d_n2;
        locals.var_qbs_swg_dn4 = assign99350_e151655_d_n4;
        locals.var_qbs_swg_dn5 = assign99350_e151655_d_n5;
        locals.var_qbs_swg_dn6 = assign99350_e151655_d_n6;
        locals.var_qbs_swg_dn7 = assign99350_e151655_d_n7;
        locals.var_qbs_swg_dn8 = assign99350_e151655_d_n8;
        locals.var_qbs_swg_dn9 = assign99350_e151655_d_n9;
        locals.var_qbs_swg_dn10 = assign99350_e151655_d_n10;
        locals.var_qbs_swg_dn11 = assign99350_e151655_d_n11;
        locals.var_qbs_swg_dn14 = assign99350_e151655_d_n14;

        let (assign99370_e151676, assign99370_e151676_d_n0, assign99370_e151676_d_n2, assign99370_e151676_d_n4, assign99370_e151676_d_n5, assign99370_e151676_d_n6, assign99370_e151676_d_n7, assign99370_e151676_d_n8, assign99370_e151676_d_n9, assign99370_e151676_d_n10, assign99370_e151676_d_n11, assign99370_e151676_d_n14,) = {
    if (((locals.var_guard2297 == 0.0) && (locals.var_guard2301 != 0.0)) && (locals.var_guard2302 == 0.0)) {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign99370_e151676;
        locals.var_t1_dn0 = assign99370_e151676_d_n0;
        locals.var_t1_dn2 = assign99370_e151676_d_n2;
        locals.var_t1_dn4 = assign99370_e151676_d_n4;
        locals.var_t1_dn5 = assign99370_e151676_d_n5;
        locals.var_t1_dn6 = assign99370_e151676_d_n6;
        locals.var_t1_dn7 = assign99370_e151676_d_n7;
        locals.var_t1_dn8 = assign99370_e151676_d_n8;
        locals.var_t1_dn9 = assign99370_e151676_d_n9;
        locals.var_t1_dn10 = assign99370_e151676_d_n10;
        locals.var_t1_dn11 = assign99370_e151676_d_n11;
        locals.var_t1_dn14 = assign99370_e151676_d_n14;

        let (assign99380_e151690, assign99380_e151690_d_n0, assign99380_e151690_d_n2, assign99380_e151690_d_n4, assign99380_e151690_d_n5, assign99380_e151690_d_n6, assign99380_e151690_d_n7, assign99380_e151690_d_n8, assign99380_e151690_d_n9, assign99380_e151690_d_n10, assign99380_e151690_d_n11, assign99380_e151690_d_n14,) = {
    if (((locals.var_guard2297 == 0.0) && (locals.var_guard2301 != 0.0)) && (locals.var_guard2302 == 0.0)) {
        let assign99380_e151686: f64 = (locals.var_czbsswg * p.p528);
        let assign99380_e151688: f64 = (assign99380_e151686 / locals.var_pzbsswg);
        (assign99380_e151688, ((((locals.var_czbsswg_dn0 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn0)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn2 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn4 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn4)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn5 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn5)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn6 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn6)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn7 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn8 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn9 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn9)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn10 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn10)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn11 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn11)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn14 * p.p528) * locals.var_pzbsswg) - (assign99380_e151686 * locals.var_pzbsswg_dn14)) / (locals.var_pzbsswg * locals.var_pzbsswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign99380_e151690;
        locals.var_t2_dn0 = assign99380_e151690_d_n0;
        locals.var_t2_dn2 = assign99380_e151690_d_n2;
        locals.var_t2_dn4 = assign99380_e151690_d_n4;
        locals.var_t2_dn5 = assign99380_e151690_d_n5;
        locals.var_t2_dn6 = assign99380_e151690_d_n6;
        locals.var_t2_dn7 = assign99380_e151690_d_n7;
        locals.var_t2_dn8 = assign99380_e151690_d_n8;
        locals.var_t2_dn9 = assign99380_e151690_d_n9;
        locals.var_t2_dn10 = assign99380_e151690_d_n10;
        locals.var_t2_dn11 = assign99380_e151690_d_n11;
        locals.var_t2_dn14 = assign99380_e151690_d_n14;

        let (assign99390_e151708, assign99390_e151708_d_n0, assign99390_e151708_d_n2, assign99390_e151708_d_n4, assign99390_e151708_d_n5, assign99390_e151708_d_n6, assign99390_e151708_d_n7, assign99390_e151708_d_n8, assign99390_e151708_d_n9, assign99390_e151708_d_n10, assign99390_e151708_d_n11, assign99390_e151708_d_n14,) = {
    if (((locals.var_guard2297 == 0.0) && (locals.var_guard2301 != 0.0)) && (locals.var_guard2302 == 0.0)) {
        let assign99390_e151702: f64 = (locals.var_vbs_jct * 0.5);
        let assign99390_e151704: f64 = (assign99390_e151702 * locals.var_t2);
        let assign99390_e151705: f64 = (locals.var_t1 + assign99390_e151704);
        let assign99390_e151706: f64 = (locals.var_vbs_jct * assign99390_e151705);
        (assign99390_e151706, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign99390_e151702 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign99390_e151705) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign99390_e151702 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign99390_e151702 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign99390_e151702 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign99390_e151702 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign99390_e151702 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign99390_e151702 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign99390_e151702 * locals.var_t2_dn9))), (locals.var_vbs_jct * (locals.var_t1_dn10 + (assign99390_e151702 * locals.var_t2_dn10))), ((locals.var_vbs_jct_dn11 * assign99390_e151705) + (locals.var_vbs_jct * (locals.var_t1_dn11 + (((locals.var_vbs_jct_dn11 * 0.5) * locals.var_t2) + (assign99390_e151702 * locals.var_t2_dn11))))), (locals.var_vbs_jct * (locals.var_t1_dn14 + (assign99390_e151702 * locals.var_t2_dn14))),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99390_e151708;
        locals.var_qbs_swg_dn0 = assign99390_e151708_d_n0;
        locals.var_qbs_swg_dn2 = assign99390_e151708_d_n2;
        locals.var_qbs_swg_dn4 = assign99390_e151708_d_n4;
        locals.var_qbs_swg_dn5 = assign99390_e151708_d_n5;
        locals.var_qbs_swg_dn6 = assign99390_e151708_d_n6;
        locals.var_qbs_swg_dn7 = assign99390_e151708_d_n7;
        locals.var_qbs_swg_dn8 = assign99390_e151708_d_n8;
        locals.var_qbs_swg_dn9 = assign99390_e151708_d_n9;
        locals.var_qbs_swg_dn10 = assign99390_e151708_d_n10;
        locals.var_qbs_swg_dn11 = assign99390_e151708_d_n11;
        locals.var_qbs_swg_dn14 = assign99390_e151708_d_n14;

        let (assign99410_e151730, assign99410_e151730_d_n0, assign99410_e151730_d_n2, assign99410_e151730_d_n4, assign99410_e151730_d_n5, assign99410_e151730_d_n6, assign99410_e151730_d_n7, assign99410_e151730_d_n8, assign99410_e151730_d_n9, assign99410_e151730_d_n10, assign99410_e151730_d_n11, assign99410_e151730_d_n14,) = {
    if ((locals.var_guard2297 == 0.0) && (locals.var_guard2301 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn11, locals.var_qbs_swg_dn14,)
    }
};
        locals.var_qbs_swg = assign99410_e151730;
        locals.var_qbs_swg_dn0 = assign99410_e151730_d_n0;
        locals.var_qbs_swg_dn2 = assign99410_e151730_d_n2;
        locals.var_qbs_swg_dn4 = assign99410_e151730_d_n4;
        locals.var_qbs_swg_dn5 = assign99410_e151730_d_n5;
        locals.var_qbs_swg_dn6 = assign99410_e151730_d_n6;
        locals.var_qbs_swg_dn7 = assign99410_e151730_d_n7;
        locals.var_qbs_swg_dn8 = assign99410_e151730_d_n8;
        locals.var_qbs_swg_dn9 = assign99410_e151730_d_n9;
        locals.var_qbs_swg_dn10 = assign99410_e151730_d_n10;
        locals.var_qbs_swg_dn11 = assign99410_e151730_d_n11;
        locals.var_qbs_swg_dn14 = assign99410_e151730_d_n14;

        let assign99430_e151742: f64 = (locals.var_ibs_btm + locals.var_ibs_sws);
        let assign99430_e151743: f64 = (locals.var_mfactor * assign99430_e151742);
        locals.var_ibs = assign99430_e151743;
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

        let assign99440_e151747: f64 = (locals.var_ibd_btm + locals.var_ibd_sws);
        let assign99440_e151748: f64 = (locals.var_mfactor * assign99440_e151747);
        locals.var_ibd = assign99440_e151748;
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

        let assign99450_e151751: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2304 = assign99450_e151751;

        let (assign99460_e151757, assign99460_e151757_d_n0, assign99460_e151757_d_n2, assign99460_e151757_d_n4, assign99460_e151757_d_n5, assign99460_e151757_d_n6, assign99460_e151757_d_n7, assign99460_e151757_d_n8, assign99460_e151757_d_n9, assign99460_e151757_d_n10, assign99460_e151757_d_n11, assign99460_e151757_d_n14,) = {
    if (locals.var_guard2304 != 0.0) {
        let assign99460_e151755: f64 = (locals.var_mfactor * locals.var_ibs_swg);
        (assign99460_e151755, (locals.var_mfactor * locals.var_ibs_swg_dn0), (locals.var_mfactor * locals.var_ibs_swg_dn2), (locals.var_mfactor * locals.var_ibs_swg_dn4), (locals.var_mfactor * locals.var_ibs_swg_dn5), (locals.var_mfactor * locals.var_ibs_swg_dn6), (locals.var_mfactor * locals.var_ibs_swg_dn7), (locals.var_mfactor * locals.var_ibs_swg_dn8), (locals.var_mfactor * locals.var_ibs_swg_dn9), (locals.var_mfactor * locals.var_ibs_swg_dn10), (locals.var_mfactor * locals.var_ibs_swg_dn11), (locals.var_mfactor * locals.var_ibs_swg_dn14),)
    } else {
        (locals.var_ibsi, locals.var_ibsi_dn0, locals.var_ibsi_dn2, locals.var_ibsi_dn4, locals.var_ibsi_dn5, locals.var_ibsi_dn6, locals.var_ibsi_dn7, locals.var_ibsi_dn8, locals.var_ibsi_dn9, locals.var_ibsi_dn10, locals.var_ibsi_dn11, locals.var_ibsi_dn14,)
    }
};
        locals.var_ibsi = assign99460_e151757;
        locals.var_ibsi_dn0 = assign99460_e151757_d_n0;
        locals.var_ibsi_dn2 = assign99460_e151757_d_n2;
        locals.var_ibsi_dn4 = assign99460_e151757_d_n4;
        locals.var_ibsi_dn5 = assign99460_e151757_d_n5;
        locals.var_ibsi_dn6 = assign99460_e151757_d_n6;
        locals.var_ibsi_dn7 = assign99460_e151757_d_n7;
        locals.var_ibsi_dn8 = assign99460_e151757_d_n8;
        locals.var_ibsi_dn9 = assign99460_e151757_d_n9;
        locals.var_ibsi_dn10 = assign99460_e151757_d_n10;
        locals.var_ibsi_dn11 = assign99460_e151757_d_n11;
        locals.var_ibsi_dn14 = assign99460_e151757_d_n14;

        let (assign99480_e151771, assign99480_e151771_d_n0, assign99480_e151771_d_n2, assign99480_e151771_d_n4, assign99480_e151771_d_n5, assign99480_e151771_d_n6, assign99480_e151771_d_n7, assign99480_e151771_d_n8, assign99480_e151771_d_n9, assign99480_e151771_d_n10, assign99480_e151771_d_n11, assign99480_e151771_d_n14,) = {
    if (locals.var_guard2304 != 0.0) {
        let assign99480_e151768: f64 = (locals.var_qbs_btm + locals.var_qbs_sws);
        let assign99480_e151769: f64 = (locals.var_mfactor * assign99480_e151768);
        (assign99480_e151769, (locals.var_mfactor * (locals.var_qbs_btm_dn0 + locals.var_qbs_sws_dn0)), (locals.var_mfactor * (locals.var_qbs_btm_dn2 + locals.var_qbs_sws_dn2)), (locals.var_mfactor * (locals.var_qbs_btm_dn4 + locals.var_qbs_sws_dn4)), (locals.var_mfactor * (locals.var_qbs_btm_dn5 + locals.var_qbs_sws_dn5)), (locals.var_mfactor * (locals.var_qbs_btm_dn6 + locals.var_qbs_sws_dn6)), (locals.var_mfactor * (locals.var_qbs_btm_dn7 + locals.var_qbs_sws_dn7)), (locals.var_mfactor * (locals.var_qbs_btm_dn8 + locals.var_qbs_sws_dn8)), (locals.var_mfactor * (locals.var_qbs_btm_dn9 + locals.var_qbs_sws_dn9)), (locals.var_mfactor * (locals.var_qbs_btm_dn10 + locals.var_qbs_sws_dn10)), (locals.var_mfactor * (locals.var_qbs_btm_dn11 + locals.var_qbs_sws_dn11)), (locals.var_mfactor * (locals.var_qbs_btm_dn14 + locals.var_qbs_sws_dn14)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn14,)
    }
};
        locals.var_qbs = assign99480_e151771;
        locals.var_qbs_dn0 = assign99480_e151771_d_n0;
        locals.var_qbs_dn2 = assign99480_e151771_d_n2;
        locals.var_qbs_dn4 = assign99480_e151771_d_n4;
        locals.var_qbs_dn5 = assign99480_e151771_d_n5;
        locals.var_qbs_dn6 = assign99480_e151771_d_n6;
        locals.var_qbs_dn7 = assign99480_e151771_d_n7;
        locals.var_qbs_dn8 = assign99480_e151771_d_n8;
        locals.var_qbs_dn9 = assign99480_e151771_d_n9;
        locals.var_qbs_dn10 = assign99480_e151771_d_n10;
        locals.var_qbs_dn11 = assign99480_e151771_d_n11;
        locals.var_qbs_dn14 = assign99480_e151771_d_n14;

        let (assign99490_e151779, assign99490_e151779_d_n0, assign99490_e151779_d_n2, assign99490_e151779_d_n4, assign99490_e151779_d_n5, assign99490_e151779_d_n6, assign99490_e151779_d_n7, assign99490_e151779_d_n8, assign99490_e151779_d_n9, assign99490_e151779_d_n10, assign99490_e151779_d_n11, assign99490_e151779_d_n14, assign99490_e151779_d_n16, assign99490_e151779_d_n17, assign99490_e151779_d_n18,) = {
    if (locals.var_guard2304 != 0.0) {
        let assign99490_e151776: f64 = (locals.var_qbd_btm + locals.var_qbd_sws);
        let assign99490_e151777: f64 = (locals.var_mfactor * assign99490_e151776);
        (assign99490_e151777, (locals.var_mfactor * (locals.var_qbd_btm_dn0 + locals.var_qbd_sws_dn0)), (locals.var_mfactor * (locals.var_qbd_btm_dn2 + locals.var_qbd_sws_dn2)), (locals.var_mfactor * (locals.var_qbd_btm_dn4 + locals.var_qbd_sws_dn4)), (locals.var_mfactor * (locals.var_qbd_btm_dn5 + locals.var_qbd_sws_dn5)), (locals.var_mfactor * (locals.var_qbd_btm_dn6 + locals.var_qbd_sws_dn6)), (locals.var_mfactor * (locals.var_qbd_btm_dn7 + locals.var_qbd_sws_dn7)), (locals.var_mfactor * (locals.var_qbd_btm_dn8 + locals.var_qbd_sws_dn8)), (locals.var_mfactor * (locals.var_qbd_btm_dn9 + locals.var_qbd_sws_dn9)), (locals.var_mfactor * (locals.var_qbd_btm_dn10 + locals.var_qbd_sws_dn10)), (locals.var_mfactor * (locals.var_qbd_btm_dn11 + locals.var_qbd_sws_dn11)), (locals.var_mfactor * (locals.var_qbd_btm_dn14 + locals.var_qbd_sws_dn14)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn14, locals.var_qbd_dn16, locals.var_qbd_dn17, locals.var_qbd_dn18,)
    }
};
        locals.var_qbd = assign99490_e151779;
        locals.var_qbd_dn0 = assign99490_e151779_d_n0;
        locals.var_qbd_dn2 = assign99490_e151779_d_n2;
        locals.var_qbd_dn4 = assign99490_e151779_d_n4;
        locals.var_qbd_dn5 = assign99490_e151779_d_n5;
        locals.var_qbd_dn6 = assign99490_e151779_d_n6;
        locals.var_qbd_dn7 = assign99490_e151779_d_n7;
        locals.var_qbd_dn8 = assign99490_e151779_d_n8;
        locals.var_qbd_dn9 = assign99490_e151779_d_n9;
        locals.var_qbd_dn10 = assign99490_e151779_d_n10;
        locals.var_qbd_dn11 = assign99490_e151779_d_n11;
        locals.var_qbd_dn14 = assign99490_e151779_d_n14;
        locals.var_qbd_dn16 = assign99490_e151779_d_n16;
        locals.var_qbd_dn17 = assign99490_e151779_d_n17;
        locals.var_qbd_dn18 = assign99490_e151779_d_n18;

        let (assign99500_e151785, assign99500_e151785_d_n0, assign99500_e151785_d_n2, assign99500_e151785_d_n4, assign99500_e151785_d_n5, assign99500_e151785_d_n6, assign99500_e151785_d_n7, assign99500_e151785_d_n8, assign99500_e151785_d_n9, assign99500_e151785_d_n10, assign99500_e151785_d_n11, assign99500_e151785_d_n14,) = {
    if (locals.var_guard2304 != 0.0) {
        let assign99500_e151783: f64 = (locals.var_mfactor * locals.var_qbs_swg);
        (assign99500_e151783, (locals.var_mfactor * locals.var_qbs_swg_dn0), (locals.var_mfactor * locals.var_qbs_swg_dn2), (locals.var_mfactor * locals.var_qbs_swg_dn4), (locals.var_mfactor * locals.var_qbs_swg_dn5), (locals.var_mfactor * locals.var_qbs_swg_dn6), (locals.var_mfactor * locals.var_qbs_swg_dn7), (locals.var_mfactor * locals.var_qbs_swg_dn8), (locals.var_mfactor * locals.var_qbs_swg_dn9), (locals.var_mfactor * locals.var_qbs_swg_dn10), (locals.var_mfactor * locals.var_qbs_swg_dn11), (locals.var_mfactor * locals.var_qbs_swg_dn14),)
    } else {
        (locals.var_qbsi, locals.var_qbsi_dn0, locals.var_qbsi_dn2, locals.var_qbsi_dn4, locals.var_qbsi_dn5, locals.var_qbsi_dn6, locals.var_qbsi_dn7, locals.var_qbsi_dn8, locals.var_qbsi_dn9, locals.var_qbsi_dn10, locals.var_qbsi_dn11, locals.var_qbsi_dn14,)
    }
};
        locals.var_qbsi = assign99500_e151785;
        locals.var_qbsi_dn0 = assign99500_e151785_d_n0;
        locals.var_qbsi_dn2 = assign99500_e151785_d_n2;
        locals.var_qbsi_dn4 = assign99500_e151785_d_n4;
        locals.var_qbsi_dn5 = assign99500_e151785_d_n5;
        locals.var_qbsi_dn6 = assign99500_e151785_d_n6;
        locals.var_qbsi_dn7 = assign99500_e151785_d_n7;
        locals.var_qbsi_dn8 = assign99500_e151785_d_n8;
        locals.var_qbsi_dn9 = assign99500_e151785_d_n9;
        locals.var_qbsi_dn10 = assign99500_e151785_d_n10;
        locals.var_qbsi_dn11 = assign99500_e151785_d_n11;
        locals.var_qbsi_dn14 = assign99500_e151785_d_n14;

        let (assign99510_e151791, assign99510_e151791_d_n0, assign99510_e151791_d_n2, assign99510_e151791_d_n4, assign99510_e151791_d_n5, assign99510_e151791_d_n6, assign99510_e151791_d_n7, assign99510_e151791_d_n8, assign99510_e151791_d_n9, assign99510_e151791_d_n10, assign99510_e151791_d_n11, assign99510_e151791_d_n14,) = {
    if (locals.var_guard2304 != 0.0) {
        let assign99510_e151789: f64 = (locals.var_mfactor * locals.var_qbd_swg);
        (assign99510_e151789, (locals.var_mfactor * locals.var_qbd_swg_dn0), (locals.var_mfactor * locals.var_qbd_swg_dn2), (locals.var_mfactor * locals.var_qbd_swg_dn4), (locals.var_mfactor * locals.var_qbd_swg_dn5), (locals.var_mfactor * locals.var_qbd_swg_dn6), (locals.var_mfactor * locals.var_qbd_swg_dn7), (locals.var_mfactor * locals.var_qbd_swg_dn8), (locals.var_mfactor * locals.var_qbd_swg_dn9), (locals.var_mfactor * locals.var_qbd_swg_dn10), (locals.var_mfactor * locals.var_qbd_swg_dn11), (locals.var_mfactor * locals.var_qbd_swg_dn14),)
    } else {
        (locals.var_qbdi, locals.var_qbdi_dn0, locals.var_qbdi_dn2, locals.var_qbdi_dn4, locals.var_qbdi_dn5, locals.var_qbdi_dn6, locals.var_qbdi_dn7, locals.var_qbdi_dn8, locals.var_qbdi_dn9, locals.var_qbdi_dn10, locals.var_qbdi_dn11, locals.var_qbdi_dn14,)
    }
};
        locals.var_qbdi = assign99510_e151791;
        locals.var_qbdi_dn0 = assign99510_e151791_d_n0;
        locals.var_qbdi_dn2 = assign99510_e151791_d_n2;
        locals.var_qbdi_dn4 = assign99510_e151791_d_n4;
        locals.var_qbdi_dn5 = assign99510_e151791_d_n5;
        locals.var_qbdi_dn6 = assign99510_e151791_d_n6;
        locals.var_qbdi_dn7 = assign99510_e151791_d_n7;
        locals.var_qbdi_dn8 = assign99510_e151791_d_n8;
        locals.var_qbdi_dn9 = assign99510_e151791_d_n9;
        locals.var_qbdi_dn10 = assign99510_e151791_d_n10;
        locals.var_qbdi_dn11 = assign99510_e151791_d_n11;
        locals.var_qbdi_dn14 = assign99510_e151791_d_n14;

        let (assign99560_e151824, assign99560_e151824_d_n0, assign99560_e151824_d_n2, assign99560_e151824_d_n4, assign99560_e151824_d_n5, assign99560_e151824_d_n6, assign99560_e151824_d_n7, assign99560_e151824_d_n8, assign99560_e151824_d_n9, assign99560_e151824_d_n10, assign99560_e151824_d_n11, assign99560_e151824_d_n14,) = {
    if (locals.var_guard2304 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibsi, locals.var_ibsi_dn0, locals.var_ibsi_dn2, locals.var_ibsi_dn4, locals.var_ibsi_dn5, locals.var_ibsi_dn6, locals.var_ibsi_dn7, locals.var_ibsi_dn8, locals.var_ibsi_dn9, locals.var_ibsi_dn10, locals.var_ibsi_dn11, locals.var_ibsi_dn14,)
    }
};
        locals.var_ibsi = assign99560_e151824;
        locals.var_ibsi_dn0 = assign99560_e151824_d_n0;
        locals.var_ibsi_dn2 = assign99560_e151824_d_n2;
        locals.var_ibsi_dn4 = assign99560_e151824_d_n4;
        locals.var_ibsi_dn5 = assign99560_e151824_d_n5;
        locals.var_ibsi_dn6 = assign99560_e151824_d_n6;
        locals.var_ibsi_dn7 = assign99560_e151824_d_n7;
        locals.var_ibsi_dn8 = assign99560_e151824_d_n8;
        locals.var_ibsi_dn9 = assign99560_e151824_d_n9;
        locals.var_ibsi_dn10 = assign99560_e151824_d_n10;
        locals.var_ibsi_dn11 = assign99560_e151824_d_n11;
        locals.var_ibsi_dn14 = assign99560_e151824_d_n14;

        let (assign99580_e151840, assign99580_e151840_d_n0, assign99580_e151840_d_n2, assign99580_e151840_d_n4, assign99580_e151840_d_n5, assign99580_e151840_d_n6, assign99580_e151840_d_n7, assign99580_e151840_d_n8, assign99580_e151840_d_n9, assign99580_e151840_d_n10, assign99580_e151840_d_n11, assign99580_e151840_d_n14,) = {
    if (locals.var_guard2304 == 0.0) {
        let assign99580_e151835: f64 = (locals.var_qbs_btm + locals.var_qbs_sws);
        let assign99580_e151837: f64 = (assign99580_e151835 + locals.var_qbs_swg);
        let assign99580_e151838: f64 = (locals.var_mfactor * assign99580_e151837);
        (assign99580_e151838, (locals.var_mfactor * ((locals.var_qbs_btm_dn0 + locals.var_qbs_sws_dn0) + locals.var_qbs_swg_dn0)), (locals.var_mfactor * ((locals.var_qbs_btm_dn2 + locals.var_qbs_sws_dn2) + locals.var_qbs_swg_dn2)), (locals.var_mfactor * ((locals.var_qbs_btm_dn4 + locals.var_qbs_sws_dn4) + locals.var_qbs_swg_dn4)), (locals.var_mfactor * ((locals.var_qbs_btm_dn5 + locals.var_qbs_sws_dn5) + locals.var_qbs_swg_dn5)), (locals.var_mfactor * ((locals.var_qbs_btm_dn6 + locals.var_qbs_sws_dn6) + locals.var_qbs_swg_dn6)), (locals.var_mfactor * ((locals.var_qbs_btm_dn7 + locals.var_qbs_sws_dn7) + locals.var_qbs_swg_dn7)), (locals.var_mfactor * ((locals.var_qbs_btm_dn8 + locals.var_qbs_sws_dn8) + locals.var_qbs_swg_dn8)), (locals.var_mfactor * ((locals.var_qbs_btm_dn9 + locals.var_qbs_sws_dn9) + locals.var_qbs_swg_dn9)), (locals.var_mfactor * ((locals.var_qbs_btm_dn10 + locals.var_qbs_sws_dn10) + locals.var_qbs_swg_dn10)), (locals.var_mfactor * ((locals.var_qbs_btm_dn11 + locals.var_qbs_sws_dn11) + locals.var_qbs_swg_dn11)), (locals.var_mfactor * ((locals.var_qbs_btm_dn14 + locals.var_qbs_sws_dn14) + locals.var_qbs_swg_dn14)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn14,)
    }
};
        locals.var_qbs = assign99580_e151840;
        locals.var_qbs_dn0 = assign99580_e151840_d_n0;
        locals.var_qbs_dn2 = assign99580_e151840_d_n2;
        locals.var_qbs_dn4 = assign99580_e151840_d_n4;
        locals.var_qbs_dn5 = assign99580_e151840_d_n5;
        locals.var_qbs_dn6 = assign99580_e151840_d_n6;
        locals.var_qbs_dn7 = assign99580_e151840_d_n7;
        locals.var_qbs_dn8 = assign99580_e151840_d_n8;
        locals.var_qbs_dn9 = assign99580_e151840_d_n9;
        locals.var_qbs_dn10 = assign99580_e151840_d_n10;
        locals.var_qbs_dn11 = assign99580_e151840_d_n11;
        locals.var_qbs_dn14 = assign99580_e151840_d_n14;

        let (assign99590_e151851, assign99590_e151851_d_n0, assign99590_e151851_d_n2, assign99590_e151851_d_n4, assign99590_e151851_d_n5, assign99590_e151851_d_n6, assign99590_e151851_d_n7, assign99590_e151851_d_n8, assign99590_e151851_d_n9, assign99590_e151851_d_n10, assign99590_e151851_d_n11, assign99590_e151851_d_n14, assign99590_e151851_d_n16, assign99590_e151851_d_n17, assign99590_e151851_d_n18,) = {
    if (locals.var_guard2304 == 0.0) {
        let assign99590_e151846: f64 = (locals.var_qbd_btm + locals.var_qbd_sws);
        let assign99590_e151848: f64 = (assign99590_e151846 + locals.var_qbd_swg);
        let assign99590_e151849: f64 = (locals.var_mfactor * assign99590_e151848);
        (assign99590_e151849, (locals.var_mfactor * ((locals.var_qbd_btm_dn0 + locals.var_qbd_sws_dn0) + locals.var_qbd_swg_dn0)), (locals.var_mfactor * ((locals.var_qbd_btm_dn2 + locals.var_qbd_sws_dn2) + locals.var_qbd_swg_dn2)), (locals.var_mfactor * ((locals.var_qbd_btm_dn4 + locals.var_qbd_sws_dn4) + locals.var_qbd_swg_dn4)), (locals.var_mfactor * ((locals.var_qbd_btm_dn5 + locals.var_qbd_sws_dn5) + locals.var_qbd_swg_dn5)), (locals.var_mfactor * ((locals.var_qbd_btm_dn6 + locals.var_qbd_sws_dn6) + locals.var_qbd_swg_dn6)), (locals.var_mfactor * ((locals.var_qbd_btm_dn7 + locals.var_qbd_sws_dn7) + locals.var_qbd_swg_dn7)), (locals.var_mfactor * ((locals.var_qbd_btm_dn8 + locals.var_qbd_sws_dn8) + locals.var_qbd_swg_dn8)), (locals.var_mfactor * ((locals.var_qbd_btm_dn9 + locals.var_qbd_sws_dn9) + locals.var_qbd_swg_dn9)), (locals.var_mfactor * ((locals.var_qbd_btm_dn10 + locals.var_qbd_sws_dn10) + locals.var_qbd_swg_dn10)), (locals.var_mfactor * ((locals.var_qbd_btm_dn11 + locals.var_qbd_sws_dn11) + locals.var_qbd_swg_dn11)), (locals.var_mfactor * ((locals.var_qbd_btm_dn14 + locals.var_qbd_sws_dn14) + locals.var_qbd_swg_dn14)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn14, locals.var_qbd_dn16, locals.var_qbd_dn17, locals.var_qbd_dn18,)
    }
};
        locals.var_qbd = assign99590_e151851;
        locals.var_qbd_dn0 = assign99590_e151851_d_n0;
        locals.var_qbd_dn2 = assign99590_e151851_d_n2;
        locals.var_qbd_dn4 = assign99590_e151851_d_n4;
        locals.var_qbd_dn5 = assign99590_e151851_d_n5;
        locals.var_qbd_dn6 = assign99590_e151851_d_n6;
        locals.var_qbd_dn7 = assign99590_e151851_d_n7;
        locals.var_qbd_dn8 = assign99590_e151851_d_n8;
        locals.var_qbd_dn9 = assign99590_e151851_d_n9;
        locals.var_qbd_dn10 = assign99590_e151851_d_n10;
        locals.var_qbd_dn11 = assign99590_e151851_d_n11;
        locals.var_qbd_dn14 = assign99590_e151851_d_n14;
        locals.var_qbd_dn16 = assign99590_e151851_d_n16;
        locals.var_qbd_dn17 = assign99590_e151851_d_n17;
        locals.var_qbd_dn18 = assign99590_e151851_d_n18;

        let (assign99620_e151878, assign99620_e151878_d_n0, assign99620_e151878_d_n2, assign99620_e151878_d_n4, assign99620_e151878_d_n5, assign99620_e151878_d_n6, assign99620_e151878_d_n7, assign99620_e151878_d_n8, assign99620_e151878_d_n9, assign99620_e151878_d_n10, assign99620_e151878_d_n11, assign99620_e151878_d_n14,) = {
    if (locals.var_guard2304 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsi, locals.var_qbsi_dn0, locals.var_qbsi_dn2, locals.var_qbsi_dn4, locals.var_qbsi_dn5, locals.var_qbsi_dn6, locals.var_qbsi_dn7, locals.var_qbsi_dn8, locals.var_qbsi_dn9, locals.var_qbsi_dn10, locals.var_qbsi_dn11, locals.var_qbsi_dn14,)
    }
};
        locals.var_qbsi = assign99620_e151878;
        locals.var_qbsi_dn0 = assign99620_e151878_d_n0;
        locals.var_qbsi_dn2 = assign99620_e151878_d_n2;
        locals.var_qbsi_dn4 = assign99620_e151878_d_n4;
        locals.var_qbsi_dn5 = assign99620_e151878_d_n5;
        locals.var_qbsi_dn6 = assign99620_e151878_d_n6;
        locals.var_qbsi_dn7 = assign99620_e151878_d_n7;
        locals.var_qbsi_dn8 = assign99620_e151878_d_n8;
        locals.var_qbsi_dn9 = assign99620_e151878_d_n9;
        locals.var_qbsi_dn10 = assign99620_e151878_d_n10;
        locals.var_qbsi_dn11 = assign99620_e151878_d_n11;
        locals.var_qbsi_dn14 = assign99620_e151878_d_n14;

        let (assign99630_e151883, assign99630_e151883_d_n0, assign99630_e151883_d_n2, assign99630_e151883_d_n4, assign99630_e151883_d_n5, assign99630_e151883_d_n6, assign99630_e151883_d_n7, assign99630_e151883_d_n8, assign99630_e151883_d_n9, assign99630_e151883_d_n10, assign99630_e151883_d_n11, assign99630_e151883_d_n14,) = {
    if (locals.var_guard2304 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdi, locals.var_qbdi_dn0, locals.var_qbdi_dn2, locals.var_qbdi_dn4, locals.var_qbdi_dn5, locals.var_qbdi_dn6, locals.var_qbdi_dn7, locals.var_qbdi_dn8, locals.var_qbdi_dn9, locals.var_qbdi_dn10, locals.var_qbdi_dn11, locals.var_qbdi_dn14,)
    }
};
        locals.var_qbdi = assign99630_e151883;
        locals.var_qbdi_dn0 = assign99630_e151883_d_n0;
        locals.var_qbdi_dn2 = assign99630_e151883_d_n2;
        locals.var_qbdi_dn4 = assign99630_e151883_d_n4;
        locals.var_qbdi_dn5 = assign99630_e151883_d_n5;
        locals.var_qbdi_dn6 = assign99630_e151883_d_n6;
        locals.var_qbdi_dn7 = assign99630_e151883_d_n7;
        locals.var_qbdi_dn8 = assign99630_e151883_d_n8;
        locals.var_qbdi_dn9 = assign99630_e151883_d_n9;
        locals.var_qbdi_dn10 = assign99630_e151883_d_n10;
        locals.var_qbdi_dn11 = assign99630_e151883_d_n11;
        locals.var_qbdi_dn14 = assign99630_e151883_d_n14;

        let assign99660_e151896: f64 = (p.p540 / 1e-6);
        locals.var_ndi_i = assign99660_e151896;

        locals.var_njl = locals.var_uc_njd;

        let assign99680_e151900: f64 = (1450.0 / 10000.0);
        locals.var_muen_i = assign99680_e151900;

        let assign99690_e151903: f64 = (500.0 / 10000.0);
        locals.var_muep_i = assign99690_e151903;

        locals.var_juncdlt = 0.001;

        let assign99710_e151908: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign99710_e151911: f64 = (locals.var_eg * locals.var_beta);
        let assign99710_e151912: f64 = (assign99710_e151908 - assign99710_e151911);
        let assign99710_e151915: f64 = (p.p499 * locals.var_log_tratio);
        let assign99710_e151916: f64 = (assign99710_e151912 + assign99710_e151915);
        let assign99710_e151918: f64 = (assign99710_e151916 / locals.var_uc_njd);
        let assign99710_e151919: f64 = (assign99710_e151918).exp();
        let assign99710_e151920: f64 = (1.45e16 * assign99710_e151919);
        locals.var_nin_dio = assign99710_e151920;
        locals.var_nin_dio_dn0 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn2 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn4 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn5 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn6 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn7 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn8 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn9 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn10 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn11 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn14 = (1.45e16 * (assign99710_e151919 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / locals.var_uc_njd)));

        let assign99720_e151923: f64 = (locals.var_nin_dio * locals.var_nin_dio);
        let assign99720_e151925: f64 = (assign99720_e151923 / locals.var_ndi_i);
        locals.var_pn0 = assign99720_e151925;
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

        let assign99730_e151928: f64 = (-1.5);
        let assign99730_e151929: f64 = (locals.var_tratio).powf(assign99730_e151928);
        locals.var_t1 = assign99730_e151929;
        locals.var_t1_dn0 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn0)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn0 / locals.var_tratio))) };
        locals.var_t1_dn2 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn2)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn2 / locals.var_tratio))) };
        locals.var_t1_dn4 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn4)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_t1_dn5 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn5)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn5 / locals.var_tratio))) };
        locals.var_t1_dn6 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn6)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn6 / locals.var_tratio))) };
        locals.var_t1_dn7 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn7)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn7 / locals.var_tratio))) };
        locals.var_t1_dn8 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn8)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn8 / locals.var_tratio))) };
        locals.var_t1_dn9 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn9)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn9 / locals.var_tratio))) };
        locals.var_t1_dn10 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn10)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn10 / locals.var_tratio))) };
        locals.var_t1_dn11 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn11)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn11 / locals.var_tratio))) };
        locals.var_t1_dn14 = if 0.0 == 0.0 && ((assign99730_e151928) as f64).is_finite() && ((assign99730_e151928) as f64).fract() == 0.0 { if assign99730_e151928 == 0.0 { 0.0 } else { (assign99730_e151928 * ((locals.var_tratio).powf(assign99730_e151928 - 1.0) * locals.var_tratio_dn14)) } } else { (assign99730_e151929 * (assign99730_e151928 * (locals.var_tratio_dn14 / locals.var_tratio))) };

        let assign99740_e151932: f64 = (locals.var_muen_i * locals.var_t1);
        let assign99740_e151934: f64 = (assign99740_e151932 * locals.var_beta_inv);
        locals.var_dn = assign99740_e151934;
        locals.var_dn_dn0 = (((locals.var_muen_i * locals.var_t1_dn0) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn0));
        locals.var_dn_dn2 = (((locals.var_muen_i * locals.var_t1_dn2) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn2));
        locals.var_dn_dn4 = (((locals.var_muen_i * locals.var_t1_dn4) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn4));
        locals.var_dn_dn5 = (((locals.var_muen_i * locals.var_t1_dn5) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn5));
        locals.var_dn_dn6 = (((locals.var_muen_i * locals.var_t1_dn6) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn6));
        locals.var_dn_dn7 = (((locals.var_muen_i * locals.var_t1_dn7) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn7));
        locals.var_dn_dn8 = (((locals.var_muen_i * locals.var_t1_dn8) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn8));
        locals.var_dn_dn9 = (((locals.var_muen_i * locals.var_t1_dn9) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn9));
        locals.var_dn_dn10 = (((locals.var_muen_i * locals.var_t1_dn10) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn10));
        locals.var_dn_dn11 = (((locals.var_muen_i * locals.var_t1_dn11) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn11));
        locals.var_dn_dn14 = (((locals.var_muen_i * locals.var_t1_dn14) * locals.var_beta_inv) + (assign99740_e151932 * locals.var_beta_inv_dn14));

        let assign99750_e151937: f64 = (locals.var_muep_i * locals.var_t1);
        let assign99750_e151939: f64 = (assign99750_e151937 * locals.var_beta_inv);
        locals.var_dp = assign99750_e151939;
        locals.var_dp_dn0 = (((locals.var_muep_i * locals.var_t1_dn0) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn0));
        locals.var_dp_dn2 = (((locals.var_muep_i * locals.var_t1_dn2) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn2));
        locals.var_dp_dn4 = (((locals.var_muep_i * locals.var_t1_dn4) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn4));
        locals.var_dp_dn5 = (((locals.var_muep_i * locals.var_t1_dn5) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn5));
        locals.var_dp_dn6 = (((locals.var_muep_i * locals.var_t1_dn6) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn6));
        locals.var_dp_dn7 = (((locals.var_muep_i * locals.var_t1_dn7) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn7));
        locals.var_dp_dn8 = (((locals.var_muep_i * locals.var_t1_dn8) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn8));
        locals.var_dp_dn9 = (((locals.var_muep_i * locals.var_t1_dn9) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn9));
        locals.var_dp_dn10 = (((locals.var_muep_i * locals.var_t1_dn10) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn10));
        locals.var_dp_dn11 = (((locals.var_muep_i * locals.var_t1_dn11) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn11));
        locals.var_dp_dn14 = (((locals.var_muep_i * locals.var_t1_dn14) * locals.var_beta_inv) + (assign99750_e151937 * locals.var_beta_inv_dn14));

        let assign99760_e151942: f64 = (2.0 * locals.var_dn);
        let assign99760_e151944: f64 = (assign99760_e151942 * locals.var_dp);
        let assign99760_e151947: f64 = (locals.var_dn + locals.var_dp);
        let assign99760_e151948: f64 = (assign99760_e151944 / assign99760_e151947);
        locals.var_da = assign99760_e151948;
        locals.var_da_dn0 = ((((((2.0 * locals.var_dn_dn0) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn0)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn0 + locals.var_dp_dn0))) / (assign99760_e151947 * assign99760_e151947));
        locals.var_da_dn2 = ((((((2.0 * locals.var_dn_dn2) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn2)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn2 + locals.var_dp_dn2))) / (assign99760_e151947 * assign99760_e151947));
        locals.var_da_dn4 = ((((((2.0 * locals.var_dn_dn4) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn4)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn4 + locals.var_dp_dn4))) / (assign99760_e151947 * assign99760_e151947));
        locals.var_da_dn5 = ((((((2.0 * locals.var_dn_dn5) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn5)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn5 + locals.var_dp_dn5))) / (assign99760_e151947 * assign99760_e151947));
        locals.var_da_dn6 = ((((((2.0 * locals.var_dn_dn6) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn6)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn6 + locals.var_dp_dn6))) / (assign99760_e151947 * assign99760_e151947));
        locals.var_da_dn7 = ((((((2.0 * locals.var_dn_dn7) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn7)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn7 + locals.var_dp_dn7))) / (assign99760_e151947 * assign99760_e151947));
        locals.var_da_dn8 = ((((((2.0 * locals.var_dn_dn8) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn8)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn8 + locals.var_dp_dn8))) / (assign99760_e151947 * assign99760_e151947));
        locals.var_da_dn9 = ((((((2.0 * locals.var_dn_dn9) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn9)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn9 + locals.var_dp_dn9))) / (assign99760_e151947 * assign99760_e151947));
        locals.var_da_dn10 = ((((((2.0 * locals.var_dn_dn10) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn10)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn10 + locals.var_dp_dn10))) / (assign99760_e151947 * assign99760_e151947));
        locals.var_da_dn11 = ((((((2.0 * locals.var_dn_dn11) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn11)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn11 + locals.var_dp_dn11))) / (assign99760_e151947 * assign99760_e151947));
        locals.var_da_dn14 = ((((((2.0 * locals.var_dn_dn14) * locals.var_dp) + (assign99760_e151942 * locals.var_dp_dn14)) * assign99760_e151947) - (assign99760_e151944 * (locals.var_dn_dn14 + locals.var_dp_dn14))) / (assign99760_e151947 * assign99760_e151947));

        let assign99770_e151951: f64 = (locals.var_tratio).powf(p.p547);
        locals.var_t2 = assign99770_e151951;
        locals.var_t2_dn0 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn0)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn0 / locals.var_tratio))) };
        locals.var_t2_dn2 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn2)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn2 / locals.var_tratio))) };
        locals.var_t2_dn4 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn4)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_t2_dn5 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn5)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn5 / locals.var_tratio))) };
        locals.var_t2_dn6 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn6)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn6 / locals.var_tratio))) };
        locals.var_t2_dn7 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn7)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn7 / locals.var_tratio))) };
        locals.var_t2_dn8 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn8)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn8 / locals.var_tratio))) };
        locals.var_t2_dn9 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn9)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn9 / locals.var_tratio))) };
        locals.var_t2_dn10 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn10)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn10 / locals.var_tratio))) };
        locals.var_t2_dn11 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn11)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn11 / locals.var_tratio))) };
        locals.var_t2_dn14 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn14)) } } else { (assign99770_e151951 * (p.p547 * (locals.var_tratio_dn14 / locals.var_tratio))) };

        let assign99780_e151954: f64 = (p.p544 * locals.var_t2);
        locals.var_tau_hl = assign99780_e151954;
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
        let assign99790_e151957: f64 = (locals.var_tau_hl * locals.var_da);
        let assign99790_e151958: f64 = (assign99790_e151957).sqrt();
        locals.var_la = assign99790_e151958;
        locals.var_la_dn0 = (((locals.var_tau_hl_dn0 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn0)) / (2.0 * assign99790_e151958));
        locals.var_la_dn2 = (((locals.var_tau_hl_dn2 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn2)) / (2.0 * assign99790_e151958));
        locals.var_la_dn4 = (((locals.var_tau_hl_dn4 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn4)) / (2.0 * assign99790_e151958));
        locals.var_la_dn5 = (((locals.var_tau_hl_dn5 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn5)) / (2.0 * assign99790_e151958));
        locals.var_la_dn6 = (((locals.var_tau_hl_dn6 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn6)) / (2.0 * assign99790_e151958));
        locals.var_la_dn7 = (((locals.var_tau_hl_dn7 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn7)) / (2.0 * assign99790_e151958));
        locals.var_la_dn8 = (((locals.var_tau_hl_dn8 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn8)) / (2.0 * assign99790_e151958));
        locals.var_la_dn9 = (((locals.var_tau_hl_dn9 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn9)) / (2.0 * assign99790_e151958));
        locals.var_la_dn10 = (((locals.var_tau_hl_dn10 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn10)) / (2.0 * assign99790_e151958));
        locals.var_la_dn11 = (((locals.var_tau_hl_dn11 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn11)) / (2.0 * assign99790_e151958));
        locals.var_la_dn14 = (((locals.var_tau_hl_dn14 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn14)) / (2.0 * assign99790_e151958));

        let assign99800_e151961: f64 = (locals.var_njl * locals.var_beta_inv);
        let assign99800_e151964: f64 = (locals.var_ndi_i / locals.var_pn0);
        let assign99800_e151965: f64 = (assign99800_e151964).ln();
        let assign99800_e151966: f64 = (assign99800_e151961 * assign99800_e151965);
        locals.var_v_ha = assign99800_e151966;
        locals.var_v_ha_dn0 = (((locals.var_njl * locals.var_beta_inv_dn0) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn0) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));
        locals.var_v_ha_dn2 = (((locals.var_njl * locals.var_beta_inv_dn2) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn2) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));
        locals.var_v_ha_dn4 = (((locals.var_njl * locals.var_beta_inv_dn4) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn4) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));
        locals.var_v_ha_dn5 = (((locals.var_njl * locals.var_beta_inv_dn5) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn5) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));
        locals.var_v_ha_dn6 = (((locals.var_njl * locals.var_beta_inv_dn6) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn6) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));
        locals.var_v_ha_dn7 = (((locals.var_njl * locals.var_beta_inv_dn7) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn7) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));
        locals.var_v_ha_dn8 = (((locals.var_njl * locals.var_beta_inv_dn8) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn8) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));
        locals.var_v_ha_dn9 = (((locals.var_njl * locals.var_beta_inv_dn9) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn9) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));
        locals.var_v_ha_dn10 = (((locals.var_njl * locals.var_beta_inv_dn10) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn10) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));
        locals.var_v_ha_dn11 = (((locals.var_njl * locals.var_beta_inv_dn11) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn11) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));
        locals.var_v_ha_dn14 = (((locals.var_njl * locals.var_beta_inv_dn14) * assign99800_e151965) + (assign99800_e151961 * ((-((locals.var_ndi_i * locals.var_pn0_dn14) / (locals.var_pn0 * locals.var_pn0))) / assign99800_e151964)));

        let assign99810_e151969: f64 = (locals.var_njl * locals.var_beta_inv);
        let assign99810_e151972: f64 = (locals.var_ndi_i / locals.var_pn0);
        let assign99810_e151973: f64 = (assign99810_e151972).ln();
        let assign99810_e151976: f64 = (p.p545 / locals.var_la);
        let assign99810_e151977: f64 = (assign99810_e151973 + assign99810_e151976);
        let assign99810_e151978: f64 = (assign99810_e151969 * assign99810_e151977);
        locals.var_v_hk = assign99810_e151978;
        locals.var_v_hk_dn0 = (((locals.var_njl * locals.var_beta_inv_dn0) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn0) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn0) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn2 = (((locals.var_njl * locals.var_beta_inv_dn2) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn2) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn2) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn4 = (((locals.var_njl * locals.var_beta_inv_dn4) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn4) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn4) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn5 = (((locals.var_njl * locals.var_beta_inv_dn5) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn5) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn5) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn6 = (((locals.var_njl * locals.var_beta_inv_dn6) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn6) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn6) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn7 = (((locals.var_njl * locals.var_beta_inv_dn7) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn7) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn7) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn8 = (((locals.var_njl * locals.var_beta_inv_dn8) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn8) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn8) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn9 = (((locals.var_njl * locals.var_beta_inv_dn9) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn9) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn9) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn10 = (((locals.var_njl * locals.var_beta_inv_dn10) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn10) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn10) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn11 = (((locals.var_njl * locals.var_beta_inv_dn11) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn11) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn11) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn14 = (((locals.var_njl * locals.var_beta_inv_dn14) * assign99810_e151977) + (assign99810_e151969 * (((-((locals.var_ndi_i * locals.var_pn0_dn14) / (locals.var_pn0 * locals.var_pn0))) / assign99810_e151972) + (-((p.p545 * locals.var_la_dn14) / (locals.var_la * locals.var_la))))));

        let assign99820_e151981: f64 = if p.p539 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2305 = assign99820_e151981;

        let (assign99830_e151985,) = {
    if (locals.var_guard2305 != 0.0) {
        (locals.var_uc_njd,)
    } else {
        (locals.var_nj_k,)
    }
};
        locals.var_nj_k = assign99830_e151985;

        let (assign99840_e151992, assign99840_e151992_d_n0, assign99840_e151992_d_n2, assign99840_e151992_d_n4, assign99840_e151992_d_n5, assign99840_e151992_d_n6, assign99840_e151992_d_n7, assign99840_e151992_d_n8, assign99840_e151992_d_n9, assign99840_e151992_d_n10, assign99840_e151992_d_n11, assign99840_e151992_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign99840_e151989: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        let assign99840_e151990: f64 = (assign99840_e151989).exp();
        (assign99840_e151990, (assign99840_e151990 * ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0))), (assign99840_e151990 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2)), (assign99840_e151990 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4)), (assign99840_e151990 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5)), (assign99840_e151990 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6)), (assign99840_e151990 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7)), (assign99840_e151990 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8)), (assign99840_e151990 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9)), (assign99840_e151990 * ((locals.var_vbd_jct_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10))), (assign99840_e151990 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn11)), (assign99840_e151990 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn14)),)
    } else {
        (locals.var_exp_a, locals.var_exp_a_dn0, locals.var_exp_a_dn2, locals.var_exp_a_dn4, locals.var_exp_a_dn5, locals.var_exp_a_dn6, locals.var_exp_a_dn7, locals.var_exp_a_dn8, locals.var_exp_a_dn9, locals.var_exp_a_dn10, locals.var_exp_a_dn11, locals.var_exp_a_dn14,)
    }
};
        locals.var_exp_a = assign99840_e151992;
        locals.var_exp_a_dn0 = assign99840_e151992_d_n0;
        locals.var_exp_a_dn2 = assign99840_e151992_d_n2;
        locals.var_exp_a_dn4 = assign99840_e151992_d_n4;
        locals.var_exp_a_dn5 = assign99840_e151992_d_n5;
        locals.var_exp_a_dn6 = assign99840_e151992_d_n6;
        locals.var_exp_a_dn7 = assign99840_e151992_d_n7;
        locals.var_exp_a_dn8 = assign99840_e151992_d_n8;
        locals.var_exp_a_dn9 = assign99840_e151992_d_n9;
        locals.var_exp_a_dn10 = assign99840_e151992_d_n10;
        locals.var_exp_a_dn11 = assign99840_e151992_d_n11;
        locals.var_exp_a_dn14 = assign99840_e151992_d_n14;

        let assign99850_e151996: f64 = (locals.var_v_hk - locals.var_v_ha);
        let assign99850_e151997: f64 = (locals.var_vbd_jct - assign99850_e151996);
        let assign99850_e151999: f64 = if assign99850_e151997 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2306 = assign99850_e151999;

        let (assign99860_e152016, assign99860_e152016_d_n0, assign99860_e152016_d_n2, assign99860_e152016_d_n4, assign99860_e152016_d_n5, assign99860_e152016_d_n6, assign99860_e152016_d_n7, assign99860_e152016_d_n8, assign99860_e152016_d_n9, assign99860_e152016_d_n10, assign99860_e152016_d_n11, assign99860_e152016_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign99860_e152006: f64 = (locals.var_vbd_jct / locals.var_nj_k);
        let assign99860_e152009: f64 = (locals.var_v_hk - locals.var_v_ha);
        let assign99860_e152011: f64 = (assign99860_e152009 / locals.var_nj_k);
        let assign99860_e152012: f64 = (assign99860_e152006 - assign99860_e152011);
        let assign99860_e152013: f64 = (locals.var_beta * assign99860_e152012);
        let assign99860_e152014: f64 = (assign99860_e152013).exp();
        (assign99860_e152014, (assign99860_e152014 * ((locals.var_beta_dn0 * assign99860_e152012) + (locals.var_beta * ((locals.var_vbd_jct_dn0 / locals.var_nj_k) - ((locals.var_v_hk_dn0 - locals.var_v_ha_dn0) / locals.var_nj_k))))), (assign99860_e152014 * ((locals.var_beta_dn2 * assign99860_e152012) + (locals.var_beta * (-((locals.var_v_hk_dn2 - locals.var_v_ha_dn2) / locals.var_nj_k))))), (assign99860_e152014 * ((locals.var_beta_dn4 * assign99860_e152012) + (locals.var_beta * (-((locals.var_v_hk_dn4 - locals.var_v_ha_dn4) / locals.var_nj_k))))), (assign99860_e152014 * ((locals.var_beta_dn5 * assign99860_e152012) + (locals.var_beta * (-((locals.var_v_hk_dn5 - locals.var_v_ha_dn5) / locals.var_nj_k))))), (assign99860_e152014 * ((locals.var_beta_dn6 * assign99860_e152012) + (locals.var_beta * (-((locals.var_v_hk_dn6 - locals.var_v_ha_dn6) / locals.var_nj_k))))), (assign99860_e152014 * ((locals.var_beta_dn7 * assign99860_e152012) + (locals.var_beta * (-((locals.var_v_hk_dn7 - locals.var_v_ha_dn7) / locals.var_nj_k))))), (assign99860_e152014 * ((locals.var_beta_dn8 * assign99860_e152012) + (locals.var_beta * (-((locals.var_v_hk_dn8 - locals.var_v_ha_dn8) / locals.var_nj_k))))), (assign99860_e152014 * ((locals.var_beta_dn9 * assign99860_e152012) + (locals.var_beta * (-((locals.var_v_hk_dn9 - locals.var_v_ha_dn9) / locals.var_nj_k))))), (assign99860_e152014 * ((locals.var_beta_dn10 * assign99860_e152012) + (locals.var_beta * ((locals.var_vbd_jct_dn10 / locals.var_nj_k) - ((locals.var_v_hk_dn10 - locals.var_v_ha_dn10) / locals.var_nj_k))))), (assign99860_e152014 * ((locals.var_beta_dn11 * assign99860_e152012) + (locals.var_beta * (-((locals.var_v_hk_dn11 - locals.var_v_ha_dn11) / locals.var_nj_k))))), (assign99860_e152014 * ((locals.var_beta_dn14 * assign99860_e152012) + (locals.var_beta * (-((locals.var_v_hk_dn14 - locals.var_v_ha_dn14) / locals.var_nj_k))))),)
    } else {
        (locals.var_exp_k, locals.var_exp_k_dn0, locals.var_exp_k_dn2, locals.var_exp_k_dn4, locals.var_exp_k_dn5, locals.var_exp_k_dn6, locals.var_exp_k_dn7, locals.var_exp_k_dn8, locals.var_exp_k_dn9, locals.var_exp_k_dn10, locals.var_exp_k_dn11, locals.var_exp_k_dn14,)
    }
};
        locals.var_exp_k = assign99860_e152016;
        locals.var_exp_k_dn0 = assign99860_e152016_d_n0;
        locals.var_exp_k_dn2 = assign99860_e152016_d_n2;
        locals.var_exp_k_dn4 = assign99860_e152016_d_n4;
        locals.var_exp_k_dn5 = assign99860_e152016_d_n5;
        locals.var_exp_k_dn6 = assign99860_e152016_d_n6;
        locals.var_exp_k_dn7 = assign99860_e152016_d_n7;
        locals.var_exp_k_dn8 = assign99860_e152016_d_n8;
        locals.var_exp_k_dn9 = assign99860_e152016_d_n9;
        locals.var_exp_k_dn10 = assign99860_e152016_d_n10;
        locals.var_exp_k_dn11 = assign99860_e152016_d_n11;
        locals.var_exp_k_dn14 = assign99860_e152016_d_n14;

        let (assign99870_e152023, assign99870_e152023_d_n0, assign99870_e152023_d_n2, assign99870_e152023_d_n4, assign99870_e152023_d_n5, assign99870_e152023_d_n6, assign99870_e152023_d_n7, assign99870_e152023_d_n8, assign99870_e152023_d_n9, assign99870_e152023_d_n10, assign99870_e152023_d_n11, assign99870_e152023_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2306 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_k, locals.var_exp_k_dn0, locals.var_exp_k_dn2, locals.var_exp_k_dn4, locals.var_exp_k_dn5, locals.var_exp_k_dn6, locals.var_exp_k_dn7, locals.var_exp_k_dn8, locals.var_exp_k_dn9, locals.var_exp_k_dn10, locals.var_exp_k_dn11, locals.var_exp_k_dn14,)
    }
};
        locals.var_exp_k = assign99870_e152023;
        locals.var_exp_k_dn0 = assign99870_e152023_d_n0;
        locals.var_exp_k_dn2 = assign99870_e152023_d_n2;
        locals.var_exp_k_dn4 = assign99870_e152023_d_n4;
        locals.var_exp_k_dn5 = assign99870_e152023_d_n5;
        locals.var_exp_k_dn6 = assign99870_e152023_d_n6;
        locals.var_exp_k_dn7 = assign99870_e152023_d_n7;
        locals.var_exp_k_dn8 = assign99870_e152023_d_n8;
        locals.var_exp_k_dn9 = assign99870_e152023_d_n9;
        locals.var_exp_k_dn10 = assign99870_e152023_d_n10;
        locals.var_exp_k_dn11 = assign99870_e152023_d_n11;
        locals.var_exp_k_dn14 = assign99870_e152023_d_n14;

        let assign99880_e152030: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_ha)) { 1.0 } else { 0.0 };
        locals.var_guard2307 = assign99880_e152030;

        let (assign99890_e152038, assign99890_e152038_d_n0, assign99890_e152038_d_n2, assign99890_e152038_d_n4, assign99890_e152038_d_n5, assign99890_e152038_d_n6, assign99890_e152038_d_n7, assign99890_e152038_d_n8, assign99890_e152038_d_n9, assign99890_e152038_d_n10, assign99890_e152038_d_n11, assign99890_e152038_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2307 != 0.0)) {
        let assign99890_e152036: f64 = (locals.var_exp_a * p.p541);
        (assign99890_e152036, (locals.var_exp_a_dn0 * p.p541), (locals.var_exp_a_dn2 * p.p541), (locals.var_exp_a_dn4 * p.p541), (locals.var_exp_a_dn5 * p.p541), (locals.var_exp_a_dn6 * p.p541), (locals.var_exp_a_dn7 * p.p541), (locals.var_exp_a_dn8 * p.p541), (locals.var_exp_a_dn9 * p.p541), (locals.var_exp_a_dn10 * p.p541), (locals.var_exp_a_dn11 * p.p541), (locals.var_exp_a_dn14 * p.p541),)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn11, locals.var_exp_a2_dn14,)
    }
};
        locals.var_exp_a2 = assign99890_e152038;
        locals.var_exp_a2_dn0 = assign99890_e152038_d_n0;
        locals.var_exp_a2_dn2 = assign99890_e152038_d_n2;
        locals.var_exp_a2_dn4 = assign99890_e152038_d_n4;
        locals.var_exp_a2_dn5 = assign99890_e152038_d_n5;
        locals.var_exp_a2_dn6 = assign99890_e152038_d_n6;
        locals.var_exp_a2_dn7 = assign99890_e152038_d_n7;
        locals.var_exp_a2_dn8 = assign99890_e152038_d_n8;
        locals.var_exp_a2_dn9 = assign99890_e152038_d_n9;
        locals.var_exp_a2_dn10 = assign99890_e152038_d_n10;
        locals.var_exp_a2_dn11 = assign99890_e152038_d_n11;
        locals.var_exp_a2_dn14 = assign99890_e152038_d_n14;

        let (assign99900_e152067, assign99900_e152067_d_n0, assign99900_e152067_d_n2, assign99900_e152067_d_n4, assign99900_e152067_d_n5, assign99900_e152067_d_n6, assign99900_e152067_d_n7, assign99900_e152067_d_n8, assign99900_e152067_d_n9, assign99900_e152067_d_n10, assign99900_e152067_d_n11, assign99900_e152067_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2307 == 0.0)) {
        let assign99900_e152045: f64 = (locals.var_exp_a * p.p541);
        let assign99900_e152047: f64 = (-p.p542);
        let assign99900_e152050: f64 = (locals.var_vbd_jct - locals.var_v_ha);
        let assign99900_e152051: f64 = (assign99900_e152047 * assign99900_e152050);
        let assign99900_e152054: f64 = (locals.var_vbd_jct - locals.var_v_ha);
        let assign99900_e152055: f64 = (assign99900_e152051 * assign99900_e152054);
        let assign99900_e152059: f64 = (1.0 / locals.var_tratio);
        let assign99900_e152060: f64 = (assign99900_e152059).ln();
        let assign99900_e152061: f64 = (p.p548 * assign99900_e152060);
        let assign99900_e152062: f64 = (assign99900_e152061).exp();
        let assign99900_e152063: f64 = (assign99900_e152055 * assign99900_e152062);
        let assign99900_e152064: f64 = (assign99900_e152063).exp();
        let assign99900_e152065: f64 = (assign99900_e152045 * assign99900_e152064);
        (assign99900_e152065, (((locals.var_exp_a_dn0 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (locals.var_vbd_jct_dn0 - locals.var_v_ha_dn0)) * assign99900_e152054) + (assign99900_e152051 * (locals.var_vbd_jct_dn0 - locals.var_v_ha_dn0))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn0 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))), (((locals.var_exp_a_dn2 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (-locals.var_v_ha_dn2)) * assign99900_e152054) + (assign99900_e152051 * (-locals.var_v_ha_dn2))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn2 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))), (((locals.var_exp_a_dn4 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (-locals.var_v_ha_dn4)) * assign99900_e152054) + (assign99900_e152051 * (-locals.var_v_ha_dn4))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn4 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))), (((locals.var_exp_a_dn5 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (-locals.var_v_ha_dn5)) * assign99900_e152054) + (assign99900_e152051 * (-locals.var_v_ha_dn5))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn5 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))), (((locals.var_exp_a_dn6 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (-locals.var_v_ha_dn6)) * assign99900_e152054) + (assign99900_e152051 * (-locals.var_v_ha_dn6))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn6 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))), (((locals.var_exp_a_dn7 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (-locals.var_v_ha_dn7)) * assign99900_e152054) + (assign99900_e152051 * (-locals.var_v_ha_dn7))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn7 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))), (((locals.var_exp_a_dn8 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (-locals.var_v_ha_dn8)) * assign99900_e152054) + (assign99900_e152051 * (-locals.var_v_ha_dn8))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn8 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))), (((locals.var_exp_a_dn9 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (-locals.var_v_ha_dn9)) * assign99900_e152054) + (assign99900_e152051 * (-locals.var_v_ha_dn9))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn9 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))), (((locals.var_exp_a_dn10 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (locals.var_vbd_jct_dn10 - locals.var_v_ha_dn10)) * assign99900_e152054) + (assign99900_e152051 * (locals.var_vbd_jct_dn10 - locals.var_v_ha_dn10))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn10 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))), (((locals.var_exp_a_dn11 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (-locals.var_v_ha_dn11)) * assign99900_e152054) + (assign99900_e152051 * (-locals.var_v_ha_dn11))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn11 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))), (((locals.var_exp_a_dn14 * p.p541) * assign99900_e152064) + (assign99900_e152045 * (assign99900_e152064 * (((((assign99900_e152047 * (-locals.var_v_ha_dn14)) * assign99900_e152054) + (assign99900_e152051 * (-locals.var_v_ha_dn14))) * assign99900_e152062) + (assign99900_e152055 * (assign99900_e152062 * (p.p548 * ((-(locals.var_tratio_dn14 / (locals.var_tratio * locals.var_tratio))) / assign99900_e152059)))))))),)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn11, locals.var_exp_a2_dn14,)
    }
};
        locals.var_exp_a2 = assign99900_e152067;
        locals.var_exp_a2_dn0 = assign99900_e152067_d_n0;
        locals.var_exp_a2_dn2 = assign99900_e152067_d_n2;
        locals.var_exp_a2_dn4 = assign99900_e152067_d_n4;
        locals.var_exp_a2_dn5 = assign99900_e152067_d_n5;
        locals.var_exp_a2_dn6 = assign99900_e152067_d_n6;
        locals.var_exp_a2_dn7 = assign99900_e152067_d_n7;
        locals.var_exp_a2_dn8 = assign99900_e152067_d_n8;
        locals.var_exp_a2_dn9 = assign99900_e152067_d_n9;
        locals.var_exp_a2_dn10 = assign99900_e152067_d_n10;
        locals.var_exp_a2_dn11 = assign99900_e152067_d_n11;
        locals.var_exp_a2_dn14 = assign99900_e152067_d_n14;

        let (assign99910_e152076, assign99910_e152076_d_n0, assign99910_e152076_d_n2, assign99910_e152076_d_n4, assign99910_e152076_d_n5, assign99910_e152076_d_n6, assign99910_e152076_d_n7, assign99910_e152076_d_n8, assign99910_e152076_d_n9, assign99910_e152076_d_n10, assign99910_e152076_d_n11, assign99910_e152076_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let (assign99910_e152074, assign99910_e152074_d_n0, assign99910_e152074_d_n2, assign99910_e152074_d_n4, assign99910_e152074_d_n5, assign99910_e152074_d_n6, assign99910_e152074_d_n7, assign99910_e152074_d_n8, assign99910_e152074_d_n9, assign99910_e152074_d_n10, assign99910_e152074_d_n11, assign99910_e152074_d_n14,) = {
            if (locals.var_exp_a2 > 1e20) {
                (1e20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn11, locals.var_exp_a2_dn14,)
            }
        };
        (assign99910_e152074, assign99910_e152074_d_n0, assign99910_e152074_d_n2, assign99910_e152074_d_n4, assign99910_e152074_d_n5, assign99910_e152074_d_n6, assign99910_e152074_d_n7, assign99910_e152074_d_n8, assign99910_e152074_d_n9, assign99910_e152074_d_n10, assign99910_e152074_d_n11, assign99910_e152074_d_n14,)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn11, locals.var_exp_a2_dn14,)
    }
};
        locals.var_exp_a2 = assign99910_e152076;
        locals.var_exp_a2_dn0 = assign99910_e152076_d_n0;
        locals.var_exp_a2_dn2 = assign99910_e152076_d_n2;
        locals.var_exp_a2_dn4 = assign99910_e152076_d_n4;
        locals.var_exp_a2_dn5 = assign99910_e152076_d_n5;
        locals.var_exp_a2_dn6 = assign99910_e152076_d_n6;
        locals.var_exp_a2_dn7 = assign99910_e152076_d_n7;
        locals.var_exp_a2_dn8 = assign99910_e152076_d_n8;
        locals.var_exp_a2_dn9 = assign99910_e152076_d_n9;
        locals.var_exp_a2_dn10 = assign99910_e152076_d_n10;
        locals.var_exp_a2_dn11 = assign99910_e152076_d_n11;
        locals.var_exp_a2_dn14 = assign99910_e152076_d_n14;

        let (assign99920_e152082, assign99920_e152082_d_n0, assign99920_e152082_d_n2, assign99920_e152082_d_n4, assign99920_e152082_d_n5, assign99920_e152082_d_n6, assign99920_e152082_d_n7, assign99920_e152082_d_n8, assign99920_e152082_d_n9, assign99920_e152082_d_n10, assign99920_e152082_d_n11, assign99920_e152082_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign99920_e152080: f64 = (locals.var_pn0 * locals.var_exp_a2);
        (assign99920_e152080, ((locals.var_pn0_dn0 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn0)), ((locals.var_pn0_dn2 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn2)), ((locals.var_pn0_dn4 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn4)), ((locals.var_pn0_dn5 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn5)), ((locals.var_pn0_dn6 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn6)), ((locals.var_pn0_dn7 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn7)), ((locals.var_pn0_dn8 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn8)), ((locals.var_pn0_dn9 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn9)), ((locals.var_pn0_dn10 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn10)), ((locals.var_pn0_dn11 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn11)), ((locals.var_pn0_dn14 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn14)),)
    } else {
        (locals.var_p_na, locals.var_p_na_dn0, locals.var_p_na_dn2, locals.var_p_na_dn4, locals.var_p_na_dn5, locals.var_p_na_dn6, locals.var_p_na_dn7, locals.var_p_na_dn8, locals.var_p_na_dn9, locals.var_p_na_dn10, locals.var_p_na_dn11, locals.var_p_na_dn14,)
    }
};
        locals.var_p_na = assign99920_e152082;
        locals.var_p_na_dn0 = assign99920_e152082_d_n0;
        locals.var_p_na_dn2 = assign99920_e152082_d_n2;
        locals.var_p_na_dn4 = assign99920_e152082_d_n4;
        locals.var_p_na_dn5 = assign99920_e152082_d_n5;
        locals.var_p_na_dn6 = assign99920_e152082_d_n6;
        locals.var_p_na_dn7 = assign99920_e152082_d_n7;
        locals.var_p_na_dn8 = assign99920_e152082_d_n8;
        locals.var_p_na_dn9 = assign99920_e152082_d_n9;
        locals.var_p_na_dn10 = assign99920_e152082_d_n10;
        locals.var_p_na_dn11 = assign99920_e152082_d_n11;
        locals.var_p_na_dn14 = assign99920_e152082_d_n14;

        let (assign99930_e152092, assign99930_e152092_d_n0, assign99930_e152092_d_n2, assign99930_e152092_d_n4, assign99930_e152092_d_n5, assign99930_e152092_d_n6, assign99930_e152092_d_n7, assign99930_e152092_d_n8, assign99930_e152092_d_n9, assign99930_e152092_d_n10, assign99930_e152092_d_n11, assign99930_e152092_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign99930_e152086: f64 = (1.6021918e-19 * p.p13);
        let assign99930_e152089: f64 = (locals.var_p_na - locals.var_pn0);
        let assign99930_e152090: f64 = (assign99930_e152086 * assign99930_e152089);
        (assign99930_e152090, (assign99930_e152086 * (locals.var_p_na_dn0 - locals.var_pn0_dn0)), (assign99930_e152086 * (locals.var_p_na_dn2 - locals.var_pn0_dn2)), (assign99930_e152086 * (locals.var_p_na_dn4 - locals.var_pn0_dn4)), (assign99930_e152086 * (locals.var_p_na_dn5 - locals.var_pn0_dn5)), (assign99930_e152086 * (locals.var_p_na_dn6 - locals.var_pn0_dn6)), (assign99930_e152086 * (locals.var_p_na_dn7 - locals.var_pn0_dn7)), (assign99930_e152086 * (locals.var_p_na_dn8 - locals.var_pn0_dn8)), (assign99930_e152086 * (locals.var_p_na_dn9 - locals.var_pn0_dn9)), (assign99930_e152086 * (locals.var_p_na_dn10 - locals.var_pn0_dn10)), (assign99930_e152086 * (locals.var_p_na_dn11 - locals.var_pn0_dn11)), (assign99930_e152086 * (locals.var_p_na_dn14 - locals.var_pn0_dn14)),)
    } else {
        (locals.var_q_pexa, locals.var_q_pexa_dn0, locals.var_q_pexa_dn2, locals.var_q_pexa_dn4, locals.var_q_pexa_dn5, locals.var_q_pexa_dn6, locals.var_q_pexa_dn7, locals.var_q_pexa_dn8, locals.var_q_pexa_dn9, locals.var_q_pexa_dn10, locals.var_q_pexa_dn11, locals.var_q_pexa_dn14,)
    }
};
        locals.var_q_pexa = assign99930_e152092;
        locals.var_q_pexa_dn0 = assign99930_e152092_d_n0;
        locals.var_q_pexa_dn2 = assign99930_e152092_d_n2;
        locals.var_q_pexa_dn4 = assign99930_e152092_d_n4;
        locals.var_q_pexa_dn5 = assign99930_e152092_d_n5;
        locals.var_q_pexa_dn6 = assign99930_e152092_d_n6;
        locals.var_q_pexa_dn7 = assign99930_e152092_d_n7;
        locals.var_q_pexa_dn8 = assign99930_e152092_d_n8;
        locals.var_q_pexa_dn9 = assign99930_e152092_d_n9;
        locals.var_q_pexa_dn10 = assign99930_e152092_d_n10;
        locals.var_q_pexa_dn11 = assign99930_e152092_d_n11;
        locals.var_q_pexa_dn14 = assign99930_e152092_d_n14;

        let assign99940_e152095: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2308 = assign99940_e152095;

        let (assign99950_e152103, assign99950_e152103_d_n0, assign99950_e152103_d_n2, assign99950_e152103_d_n4, assign99950_e152103_d_n5, assign99950_e152103_d_n6, assign99950_e152103_d_n7, assign99950_e152103_d_n8, assign99950_e152103_d_n9, assign99950_e152103_d_n10, assign99950_e152103_d_n11, assign99950_e152103_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign99950_e152101: f64 = (locals.var_q_pexa * p.p543);
        (assign99950_e152101, (locals.var_q_pexa_dn0 * p.p543), (locals.var_q_pexa_dn2 * p.p543), (locals.var_q_pexa_dn4 * p.p543), (locals.var_q_pexa_dn5 * p.p543), (locals.var_q_pexa_dn6 * p.p543), (locals.var_q_pexa_dn7 * p.p543), (locals.var_q_pexa_dn8 * p.p543), (locals.var_q_pexa_dn9 * p.p543), (locals.var_q_pexa_dn10 * p.p543), (locals.var_q_pexa_dn11 * p.p543), (locals.var_q_pexa_dn14 * p.p543),)
    } else {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn11, locals.var_q_qs_a_dn14,)
    }
};
        locals.var_q_qs_a = assign99950_e152103;
        locals.var_q_qs_a_dn0 = assign99950_e152103_d_n0;
        locals.var_q_qs_a_dn2 = assign99950_e152103_d_n2;
        locals.var_q_qs_a_dn4 = assign99950_e152103_d_n4;
        locals.var_q_qs_a_dn5 = assign99950_e152103_d_n5;
        locals.var_q_qs_a_dn6 = assign99950_e152103_d_n6;
        locals.var_q_qs_a_dn7 = assign99950_e152103_d_n7;
        locals.var_q_qs_a_dn8 = assign99950_e152103_d_n8;
        locals.var_q_qs_a_dn9 = assign99950_e152103_d_n9;
        locals.var_q_qs_a_dn10 = assign99950_e152103_d_n10;
        locals.var_q_qs_a_dn11 = assign99950_e152103_d_n11;
        locals.var_q_qs_a_dn14 = assign99950_e152103_d_n14;

        let (assign99960_e152111, assign99960_e152111_d_n16,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign99960_e152109: f64 = (p.p543 * (nv16 - 0.0));
        (assign99960_e152109, p.p543,)
    } else {
        (locals.var_q_nqs_a, locals.var_q_nqs_a_dn16,)
    }
};
        locals.var_q_nqs_a = assign99960_e152111;
        locals.var_q_nqs_a_dn16 = assign99960_e152111_d_n16;

        let (assign99970_e152121, assign99970_e152121_d_n0, assign99970_e152121_d_n2, assign99970_e152121_d_n4, assign99970_e152121_d_n5, assign99970_e152121_d_n6, assign99970_e152121_d_n7, assign99970_e152121_d_n8, assign99970_e152121_d_n9, assign99970_e152121_d_n10, assign99970_e152121_d_n11, assign99970_e152121_d_n14, assign99970_e152121_d_n16,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign99970_e152117: f64 = (locals.var_q_nqs_a - locals.var_q_qs_a);
        let assign99970_e152119: f64 = (assign99970_e152117 / p.p543);
        (assign99970_e152119, ((-locals.var_q_qs_a_dn0) / p.p543), ((-locals.var_q_qs_a_dn2) / p.p543), ((-locals.var_q_qs_a_dn4) / p.p543), ((-locals.var_q_qs_a_dn5) / p.p543), ((-locals.var_q_qs_a_dn6) / p.p543), ((-locals.var_q_qs_a_dn7) / p.p543), ((-locals.var_q_qs_a_dn8) / p.p543), ((-locals.var_q_qs_a_dn9) / p.p543), ((-locals.var_q_qs_a_dn10) / p.p543), ((-locals.var_q_qs_a_dn11) / p.p543), ((-locals.var_q_qs_a_dn14) / p.p543), (locals.var_q_nqs_a_dn16 / p.p543),)
    } else {
        (locals.var_inqs0_a, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn11, locals.var_inqs0_a_dn14, locals.var_inqs0_a_dn16,)
    }
};
        locals.var_inqs0_a = assign99970_e152121;
        locals.var_inqs0_a_dn0 = assign99970_e152121_d_n0;
        locals.var_inqs0_a_dn2 = assign99970_e152121_d_n2;
        locals.var_inqs0_a_dn4 = assign99970_e152121_d_n4;
        locals.var_inqs0_a_dn5 = assign99970_e152121_d_n5;
        locals.var_inqs0_a_dn6 = assign99970_e152121_d_n6;
        locals.var_inqs0_a_dn7 = assign99970_e152121_d_n7;
        locals.var_inqs0_a_dn8 = assign99970_e152121_d_n8;
        locals.var_inqs0_a_dn9 = assign99970_e152121_d_n9;
        locals.var_inqs0_a_dn10 = assign99970_e152121_d_n10;
        locals.var_inqs0_a_dn11 = assign99970_e152121_d_n11;
        locals.var_inqs0_a_dn14 = assign99970_e152121_d_n14;
        locals.var_inqs0_a_dn16 = assign99970_e152121_d_n16;

        let (assign99980_e152129, assign99980_e152129_d_n0, assign99980_e152129_d_n2, assign99980_e152129_d_n4, assign99980_e152129_d_n5, assign99980_e152129_d_n6, assign99980_e152129_d_n7, assign99980_e152129_d_n8, assign99980_e152129_d_n9, assign99980_e152129_d_n10, assign99980_e152129_d_n11, assign99980_e152129_d_n14, assign99980_e152129_d_n16,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign99980_e152127: f64 = (locals.var_q_nqs_a / p.p543);
        (assign99980_e152127, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_q_nqs_a_dn16 / p.p543),)
    } else {
        (locals.var_q_pexa_nqs, locals.var_q_pexa_nqs_dn0, locals.var_q_pexa_nqs_dn2, locals.var_q_pexa_nqs_dn4, locals.var_q_pexa_nqs_dn5, locals.var_q_pexa_nqs_dn6, locals.var_q_pexa_nqs_dn7, locals.var_q_pexa_nqs_dn8, locals.var_q_pexa_nqs_dn9, locals.var_q_pexa_nqs_dn10, locals.var_q_pexa_nqs_dn11, locals.var_q_pexa_nqs_dn14, locals.var_q_pexa_nqs_dn16,)
    }
};
        locals.var_q_pexa_nqs = assign99980_e152129;
        locals.var_q_pexa_nqs_dn0 = assign99980_e152129_d_n0;
        locals.var_q_pexa_nqs_dn2 = assign99980_e152129_d_n2;
        locals.var_q_pexa_nqs_dn4 = assign99980_e152129_d_n4;
        locals.var_q_pexa_nqs_dn5 = assign99980_e152129_d_n5;
        locals.var_q_pexa_nqs_dn6 = assign99980_e152129_d_n6;
        locals.var_q_pexa_nqs_dn7 = assign99980_e152129_d_n7;
        locals.var_q_pexa_nqs_dn8 = assign99980_e152129_d_n8;
        locals.var_q_pexa_nqs_dn9 = assign99980_e152129_d_n9;
        locals.var_q_pexa_nqs_dn10 = assign99980_e152129_d_n10;
        locals.var_q_pexa_nqs_dn11 = assign99980_e152129_d_n11;
        locals.var_q_pexa_nqs_dn14 = assign99980_e152129_d_n14;
        locals.var_q_pexa_nqs_dn16 = assign99980_e152129_d_n16;

        let (assign99990_e152136, assign99990_e152136_d_n0, assign99990_e152136_d_n2, assign99990_e152136_d_n4, assign99990_e152136_d_n5, assign99990_e152136_d_n6, assign99990_e152136_d_n7, assign99990_e152136_d_n8, assign99990_e152136_d_n9, assign99990_e152136_d_n10, assign99990_e152136_d_n11, assign99990_e152136_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2308 == 0.0)) {
        (locals.var_q_pexa, locals.var_q_pexa_dn0, locals.var_q_pexa_dn2, locals.var_q_pexa_dn4, locals.var_q_pexa_dn5, locals.var_q_pexa_dn6, locals.var_q_pexa_dn7, locals.var_q_pexa_dn8, locals.var_q_pexa_dn9, locals.var_q_pexa_dn10, locals.var_q_pexa_dn11, locals.var_q_pexa_dn14,)
    } else {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn11, locals.var_q_qs_a_dn14,)
    }
};
        locals.var_q_qs_a = assign99990_e152136;
        locals.var_q_qs_a_dn0 = assign99990_e152136_d_n0;
        locals.var_q_qs_a_dn2 = assign99990_e152136_d_n2;
        locals.var_q_qs_a_dn4 = assign99990_e152136_d_n4;
        locals.var_q_qs_a_dn5 = assign99990_e152136_d_n5;
        locals.var_q_qs_a_dn6 = assign99990_e152136_d_n6;
        locals.var_q_qs_a_dn7 = assign99990_e152136_d_n7;
        locals.var_q_qs_a_dn8 = assign99990_e152136_d_n8;
        locals.var_q_qs_a_dn9 = assign99990_e152136_d_n9;
        locals.var_q_qs_a_dn10 = assign99990_e152136_d_n10;
        locals.var_q_qs_a_dn11 = assign99990_e152136_d_n11;
        locals.var_q_qs_a_dn14 = assign99990_e152136_d_n14;

        let (assign100000_e152143, assign100000_e152143_d_n0, assign100000_e152143_d_n2, assign100000_e152143_d_n4, assign100000_e152143_d_n5, assign100000_e152143_d_n6, assign100000_e152143_d_n7, assign100000_e152143_d_n8, assign100000_e152143_d_n9, assign100000_e152143_d_n10, assign100000_e152143_d_n11, assign100000_e152143_d_n14, assign100000_e152143_d_n16,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2308 == 0.0)) {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn11, locals.var_q_qs_a_dn14, 0.0,)
    } else {
        (locals.var_q_pexa_nqs, locals.var_q_pexa_nqs_dn0, locals.var_q_pexa_nqs_dn2, locals.var_q_pexa_nqs_dn4, locals.var_q_pexa_nqs_dn5, locals.var_q_pexa_nqs_dn6, locals.var_q_pexa_nqs_dn7, locals.var_q_pexa_nqs_dn8, locals.var_q_pexa_nqs_dn9, locals.var_q_pexa_nqs_dn10, locals.var_q_pexa_nqs_dn11, locals.var_q_pexa_nqs_dn14, locals.var_q_pexa_nqs_dn16,)
    }
};
        locals.var_q_pexa_nqs = assign100000_e152143;
        locals.var_q_pexa_nqs_dn0 = assign100000_e152143_d_n0;
        locals.var_q_pexa_nqs_dn2 = assign100000_e152143_d_n2;
        locals.var_q_pexa_nqs_dn4 = assign100000_e152143_d_n4;
        locals.var_q_pexa_nqs_dn5 = assign100000_e152143_d_n5;
        locals.var_q_pexa_nqs_dn6 = assign100000_e152143_d_n6;
        locals.var_q_pexa_nqs_dn7 = assign100000_e152143_d_n7;
        locals.var_q_pexa_nqs_dn8 = assign100000_e152143_d_n8;
        locals.var_q_pexa_nqs_dn9 = assign100000_e152143_d_n9;
        locals.var_q_pexa_nqs_dn10 = assign100000_e152143_d_n10;
        locals.var_q_pexa_nqs_dn11 = assign100000_e152143_d_n11;
        locals.var_q_pexa_nqs_dn14 = assign100000_e152143_d_n14;
        locals.var_q_pexa_nqs_dn16 = assign100000_e152143_d_n16;

        let assign100010_e152150: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_hk)) { 1.0 } else { 0.0 };
        locals.var_guard2309 = assign100010_e152150;

        let (assign100020_e152158, assign100020_e152158_d_n0, assign100020_e152158_d_n2, assign100020_e152158_d_n4, assign100020_e152158_d_n5, assign100020_e152158_d_n6, assign100020_e152158_d_n7, assign100020_e152158_d_n8, assign100020_e152158_d_n9, assign100020_e152158_d_n10, assign100020_e152158_d_n11, assign100020_e152158_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2309 != 0.0)) {
        let assign100020_e152156: f64 = (locals.var_exp_k * p.p541);
        (assign100020_e152156, (locals.var_exp_k_dn0 * p.p541), (locals.var_exp_k_dn2 * p.p541), (locals.var_exp_k_dn4 * p.p541), (locals.var_exp_k_dn5 * p.p541), (locals.var_exp_k_dn6 * p.p541), (locals.var_exp_k_dn7 * p.p541), (locals.var_exp_k_dn8 * p.p541), (locals.var_exp_k_dn9 * p.p541), (locals.var_exp_k_dn10 * p.p541), (locals.var_exp_k_dn11 * p.p541), (locals.var_exp_k_dn14 * p.p541),)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
    }
};
        locals.var_exp_k2 = assign100020_e152158;
        locals.var_exp_k2_dn0 = assign100020_e152158_d_n0;
        locals.var_exp_k2_dn2 = assign100020_e152158_d_n2;
        locals.var_exp_k2_dn4 = assign100020_e152158_d_n4;
        locals.var_exp_k2_dn5 = assign100020_e152158_d_n5;
        locals.var_exp_k2_dn6 = assign100020_e152158_d_n6;
        locals.var_exp_k2_dn7 = assign100020_e152158_d_n7;
        locals.var_exp_k2_dn8 = assign100020_e152158_d_n8;
        locals.var_exp_k2_dn9 = assign100020_e152158_d_n9;
        locals.var_exp_k2_dn10 = assign100020_e152158_d_n10;
        locals.var_exp_k2_dn11 = assign100020_e152158_d_n11;
        locals.var_exp_k2_dn14 = assign100020_e152158_d_n14;

        let (assign100030_e152187, assign100030_e152187_d_n0, assign100030_e152187_d_n2, assign100030_e152187_d_n4, assign100030_e152187_d_n5, assign100030_e152187_d_n6, assign100030_e152187_d_n7, assign100030_e152187_d_n8, assign100030_e152187_d_n9, assign100030_e152187_d_n10, assign100030_e152187_d_n11, assign100030_e152187_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2309 == 0.0)) {
        let assign100030_e152165: f64 = (locals.var_exp_k * p.p541);
        let assign100030_e152167: f64 = (-p.p542);
        let assign100030_e152170: f64 = (locals.var_vbd_jct - locals.var_v_hk);
        let assign100030_e152171: f64 = (assign100030_e152167 * assign100030_e152170);
        let assign100030_e152174: f64 = (locals.var_vbd_jct - locals.var_v_hk);
        let assign100030_e152175: f64 = (assign100030_e152171 * assign100030_e152174);
        let assign100030_e152179: f64 = (1.0 / locals.var_tratio);
        let assign100030_e152180: f64 = (assign100030_e152179).ln();
        let assign100030_e152181: f64 = (p.p548 * assign100030_e152180);
        let assign100030_e152182: f64 = (assign100030_e152181).exp();
        let assign100030_e152183: f64 = (assign100030_e152175 * assign100030_e152182);
        let assign100030_e152184: f64 = (assign100030_e152183).exp();
        let assign100030_e152185: f64 = (assign100030_e152165 * assign100030_e152184);
        (assign100030_e152185, (((locals.var_exp_k_dn0 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (locals.var_vbd_jct_dn0 - locals.var_v_hk_dn0)) * assign100030_e152174) + (assign100030_e152171 * (locals.var_vbd_jct_dn0 - locals.var_v_hk_dn0))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn0 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn2 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn2)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn2))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn2 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn4 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn4)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn4))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn4 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn5 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn5)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn5))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn5 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn6 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn6)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn6))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn6 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn7 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn7)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn7))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn7 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn8 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn8)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn8))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn8 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn9 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn9)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn9))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn9 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn10 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (locals.var_vbd_jct_dn10 - locals.var_v_hk_dn10)) * assign100030_e152174) + (assign100030_e152171 * (locals.var_vbd_jct_dn10 - locals.var_v_hk_dn10))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn10 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn11 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn11)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn11))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn11 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn14 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn14)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn14))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn14 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))),)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
    }
};
        locals.var_exp_k2 = assign100030_e152187;
        locals.var_exp_k2_dn0 = assign100030_e152187_d_n0;
        locals.var_exp_k2_dn2 = assign100030_e152187_d_n2;
        locals.var_exp_k2_dn4 = assign100030_e152187_d_n4;
        locals.var_exp_k2_dn5 = assign100030_e152187_d_n5;
        locals.var_exp_k2_dn6 = assign100030_e152187_d_n6;
        locals.var_exp_k2_dn7 = assign100030_e152187_d_n7;
        locals.var_exp_k2_dn8 = assign100030_e152187_d_n8;
        locals.var_exp_k2_dn9 = assign100030_e152187_d_n9;
        locals.var_exp_k2_dn10 = assign100030_e152187_d_n10;
        locals.var_exp_k2_dn11 = assign100030_e152187_d_n11;
        locals.var_exp_k2_dn14 = assign100030_e152187_d_n14;

        let (assign100040_e152196, assign100040_e152196_d_n0, assign100040_e152196_d_n2, assign100040_e152196_d_n4, assign100040_e152196_d_n5, assign100040_e152196_d_n6, assign100040_e152196_d_n7, assign100040_e152196_d_n8, assign100040_e152196_d_n9, assign100040_e152196_d_n10, assign100040_e152196_d_n11, assign100040_e152196_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let (assign100040_e152194, assign100040_e152194_d_n0, assign100040_e152194_d_n2, assign100040_e152194_d_n4, assign100040_e152194_d_n5, assign100040_e152194_d_n6, assign100040_e152194_d_n7, assign100040_e152194_d_n8, assign100040_e152194_d_n9, assign100040_e152194_d_n10, assign100040_e152194_d_n11, assign100040_e152194_d_n14,) = {
            if (locals.var_exp_k2 > 1e20) {
                (1e20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
            }
        };
        (assign100040_e152194, assign100040_e152194_d_n0, assign100040_e152194_d_n2, assign100040_e152194_d_n4, assign100040_e152194_d_n5, assign100040_e152194_d_n6, assign100040_e152194_d_n7, assign100040_e152194_d_n8, assign100040_e152194_d_n9, assign100040_e152194_d_n10, assign100040_e152194_d_n11, assign100040_e152194_d_n14,)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
    }
};
        locals.var_exp_k2 = assign100040_e152196;
        locals.var_exp_k2_dn0 = assign100040_e152196_d_n0;
        locals.var_exp_k2_dn2 = assign100040_e152196_d_n2;
        locals.var_exp_k2_dn4 = assign100040_e152196_d_n4;
        locals.var_exp_k2_dn5 = assign100040_e152196_d_n5;
        locals.var_exp_k2_dn6 = assign100040_e152196_d_n6;
        locals.var_exp_k2_dn7 = assign100040_e152196_d_n7;
        locals.var_exp_k2_dn8 = assign100040_e152196_d_n8;
        locals.var_exp_k2_dn9 = assign100040_e152196_d_n9;
        locals.var_exp_k2_dn10 = assign100040_e152196_d_n10;
        locals.var_exp_k2_dn11 = assign100040_e152196_d_n11;
        locals.var_exp_k2_dn14 = assign100040_e152196_d_n14;

        let (assign100050_e152202, assign100050_e152202_d_n0, assign100050_e152202_d_n2, assign100050_e152202_d_n4, assign100050_e152202_d_n5, assign100050_e152202_d_n6, assign100050_e152202_d_n7, assign100050_e152202_d_n8, assign100050_e152202_d_n9, assign100050_e152202_d_n10, assign100050_e152202_d_n11, assign100050_e152202_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100050_e152200: f64 = (locals.var_pn0 * locals.var_exp_k2);
        (assign100050_e152200, ((locals.var_pn0_dn0 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn0)), ((locals.var_pn0_dn2 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn2)), ((locals.var_pn0_dn4 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn4)), ((locals.var_pn0_dn5 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn5)), ((locals.var_pn0_dn6 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn6)), ((locals.var_pn0_dn7 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn7)), ((locals.var_pn0_dn8 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn8)), ((locals.var_pn0_dn9 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn9)), ((locals.var_pn0_dn10 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn10)), ((locals.var_pn0_dn11 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn11)), ((locals.var_pn0_dn14 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn14)),)
    } else {
        (locals.var_p_nk, locals.var_p_nk_dn0, locals.var_p_nk_dn2, locals.var_p_nk_dn4, locals.var_p_nk_dn5, locals.var_p_nk_dn6, locals.var_p_nk_dn7, locals.var_p_nk_dn8, locals.var_p_nk_dn9, locals.var_p_nk_dn10, locals.var_p_nk_dn11, locals.var_p_nk_dn14,)
    }
};
        locals.var_p_nk = assign100050_e152202;
        locals.var_p_nk_dn0 = assign100050_e152202_d_n0;
        locals.var_p_nk_dn2 = assign100050_e152202_d_n2;
        locals.var_p_nk_dn4 = assign100050_e152202_d_n4;
        locals.var_p_nk_dn5 = assign100050_e152202_d_n5;
        locals.var_p_nk_dn6 = assign100050_e152202_d_n6;
        locals.var_p_nk_dn7 = assign100050_e152202_d_n7;
        locals.var_p_nk_dn8 = assign100050_e152202_d_n8;
        locals.var_p_nk_dn9 = assign100050_e152202_d_n9;
        locals.var_p_nk_dn10 = assign100050_e152202_d_n10;
        locals.var_p_nk_dn11 = assign100050_e152202_d_n11;
        locals.var_p_nk_dn14 = assign100050_e152202_d_n14;

    }

    pub(super) fn stamp_transient_block_367(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (assign100060_e152212, assign100060_e152212_d_n0, assign100060_e152212_d_n2, assign100060_e152212_d_n4, assign100060_e152212_d_n5, assign100060_e152212_d_n6, assign100060_e152212_d_n7, assign100060_e152212_d_n8, assign100060_e152212_d_n9, assign100060_e152212_d_n10, assign100060_e152212_d_n11, assign100060_e152212_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100060_e152206: f64 = (1.6021918e-19 * p.p13);
        let assign100060_e152209: f64 = (locals.var_p_nk - locals.var_pn0);
        let assign100060_e152210: f64 = (assign100060_e152206 * assign100060_e152209);
        (assign100060_e152210, (assign100060_e152206 * (locals.var_p_nk_dn0 - locals.var_pn0_dn0)), (assign100060_e152206 * (locals.var_p_nk_dn2 - locals.var_pn0_dn2)), (assign100060_e152206 * (locals.var_p_nk_dn4 - locals.var_pn0_dn4)), (assign100060_e152206 * (locals.var_p_nk_dn5 - locals.var_pn0_dn5)), (assign100060_e152206 * (locals.var_p_nk_dn6 - locals.var_pn0_dn6)), (assign100060_e152206 * (locals.var_p_nk_dn7 - locals.var_pn0_dn7)), (assign100060_e152206 * (locals.var_p_nk_dn8 - locals.var_pn0_dn8)), (assign100060_e152206 * (locals.var_p_nk_dn9 - locals.var_pn0_dn9)), (assign100060_e152206 * (locals.var_p_nk_dn10 - locals.var_pn0_dn10)), (assign100060_e152206 * (locals.var_p_nk_dn11 - locals.var_pn0_dn11)), (assign100060_e152206 * (locals.var_p_nk_dn14 - locals.var_pn0_dn14)),)
    } else {
        (locals.var_q_pexk, locals.var_q_pexk_dn0, locals.var_q_pexk_dn2, locals.var_q_pexk_dn4, locals.var_q_pexk_dn5, locals.var_q_pexk_dn6, locals.var_q_pexk_dn7, locals.var_q_pexk_dn8, locals.var_q_pexk_dn9, locals.var_q_pexk_dn10, locals.var_q_pexk_dn11, locals.var_q_pexk_dn14,)
    }
};
        locals.var_q_pexk = assign100060_e152212;
        locals.var_q_pexk_dn0 = assign100060_e152212_d_n0;
        locals.var_q_pexk_dn2 = assign100060_e152212_d_n2;
        locals.var_q_pexk_dn4 = assign100060_e152212_d_n4;
        locals.var_q_pexk_dn5 = assign100060_e152212_d_n5;
        locals.var_q_pexk_dn6 = assign100060_e152212_d_n6;
        locals.var_q_pexk_dn7 = assign100060_e152212_d_n7;
        locals.var_q_pexk_dn8 = assign100060_e152212_d_n8;
        locals.var_q_pexk_dn9 = assign100060_e152212_d_n9;
        locals.var_q_pexk_dn10 = assign100060_e152212_d_n10;
        locals.var_q_pexk_dn11 = assign100060_e152212_d_n11;
        locals.var_q_pexk_dn14 = assign100060_e152212_d_n14;

        let assign100070_e152215: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2310 = assign100070_e152215;

        let (assign100080_e152223, assign100080_e152223_d_n0, assign100080_e152223_d_n2, assign100080_e152223_d_n4, assign100080_e152223_d_n5, assign100080_e152223_d_n6, assign100080_e152223_d_n7, assign100080_e152223_d_n8, assign100080_e152223_d_n9, assign100080_e152223_d_n10, assign100080_e152223_d_n11, assign100080_e152223_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100080_e152221: f64 = (locals.var_q_pexk * p.p543);
        (assign100080_e152221, (locals.var_q_pexk_dn0 * p.p543), (locals.var_q_pexk_dn2 * p.p543), (locals.var_q_pexk_dn4 * p.p543), (locals.var_q_pexk_dn5 * p.p543), (locals.var_q_pexk_dn6 * p.p543), (locals.var_q_pexk_dn7 * p.p543), (locals.var_q_pexk_dn8 * p.p543), (locals.var_q_pexk_dn9 * p.p543), (locals.var_q_pexk_dn10 * p.p543), (locals.var_q_pexk_dn11 * p.p543), (locals.var_q_pexk_dn14 * p.p543),)
    } else {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn11, locals.var_q_qs_k_dn14,)
    }
};
        locals.var_q_qs_k = assign100080_e152223;
        locals.var_q_qs_k_dn0 = assign100080_e152223_d_n0;
        locals.var_q_qs_k_dn2 = assign100080_e152223_d_n2;
        locals.var_q_qs_k_dn4 = assign100080_e152223_d_n4;
        locals.var_q_qs_k_dn5 = assign100080_e152223_d_n5;
        locals.var_q_qs_k_dn6 = assign100080_e152223_d_n6;
        locals.var_q_qs_k_dn7 = assign100080_e152223_d_n7;
        locals.var_q_qs_k_dn8 = assign100080_e152223_d_n8;
        locals.var_q_qs_k_dn9 = assign100080_e152223_d_n9;
        locals.var_q_qs_k_dn10 = assign100080_e152223_d_n10;
        locals.var_q_qs_k_dn11 = assign100080_e152223_d_n11;
        locals.var_q_qs_k_dn14 = assign100080_e152223_d_n14;

        let (assign100090_e152231, assign100090_e152231_d_n17,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100090_e152229: f64 = (p.p543 * (nv17 - 0.0));
        (assign100090_e152229, p.p543,)
    } else {
        (locals.var_q_nqs_k, locals.var_q_nqs_k_dn17,)
    }
};
        locals.var_q_nqs_k = assign100090_e152231;
        locals.var_q_nqs_k_dn17 = assign100090_e152231_d_n17;

        let (assign100100_e152241, assign100100_e152241_d_n0, assign100100_e152241_d_n2, assign100100_e152241_d_n4, assign100100_e152241_d_n5, assign100100_e152241_d_n6, assign100100_e152241_d_n7, assign100100_e152241_d_n8, assign100100_e152241_d_n9, assign100100_e152241_d_n10, assign100100_e152241_d_n11, assign100100_e152241_d_n14, assign100100_e152241_d_n17,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100100_e152237: f64 = (locals.var_q_nqs_k - locals.var_q_qs_k);
        let assign100100_e152239: f64 = (assign100100_e152237 / p.p543);
        (assign100100_e152239, ((-locals.var_q_qs_k_dn0) / p.p543), ((-locals.var_q_qs_k_dn2) / p.p543), ((-locals.var_q_qs_k_dn4) / p.p543), ((-locals.var_q_qs_k_dn5) / p.p543), ((-locals.var_q_qs_k_dn6) / p.p543), ((-locals.var_q_qs_k_dn7) / p.p543), ((-locals.var_q_qs_k_dn8) / p.p543), ((-locals.var_q_qs_k_dn9) / p.p543), ((-locals.var_q_qs_k_dn10) / p.p543), ((-locals.var_q_qs_k_dn11) / p.p543), ((-locals.var_q_qs_k_dn14) / p.p543), (locals.var_q_nqs_k_dn17 / p.p543),)
    } else {
        (locals.var_inqs0_k, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn11, locals.var_inqs0_k_dn14, locals.var_inqs0_k_dn17,)
    }
};
        locals.var_inqs0_k = assign100100_e152241;
        locals.var_inqs0_k_dn0 = assign100100_e152241_d_n0;
        locals.var_inqs0_k_dn2 = assign100100_e152241_d_n2;
        locals.var_inqs0_k_dn4 = assign100100_e152241_d_n4;
        locals.var_inqs0_k_dn5 = assign100100_e152241_d_n5;
        locals.var_inqs0_k_dn6 = assign100100_e152241_d_n6;
        locals.var_inqs0_k_dn7 = assign100100_e152241_d_n7;
        locals.var_inqs0_k_dn8 = assign100100_e152241_d_n8;
        locals.var_inqs0_k_dn9 = assign100100_e152241_d_n9;
        locals.var_inqs0_k_dn10 = assign100100_e152241_d_n10;
        locals.var_inqs0_k_dn11 = assign100100_e152241_d_n11;
        locals.var_inqs0_k_dn14 = assign100100_e152241_d_n14;
        locals.var_inqs0_k_dn17 = assign100100_e152241_d_n17;

        let (assign100110_e152249, assign100110_e152249_d_n0, assign100110_e152249_d_n2, assign100110_e152249_d_n4, assign100110_e152249_d_n5, assign100110_e152249_d_n6, assign100110_e152249_d_n7, assign100110_e152249_d_n8, assign100110_e152249_d_n9, assign100110_e152249_d_n10, assign100110_e152249_d_n11, assign100110_e152249_d_n14, assign100110_e152249_d_n17,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100110_e152247: f64 = (locals.var_q_nqs_k / p.p543);
        (assign100110_e152247, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_q_nqs_k_dn17 / p.p543),)
    } else {
        (locals.var_q_pexk_nqs, locals.var_q_pexk_nqs_dn0, locals.var_q_pexk_nqs_dn2, locals.var_q_pexk_nqs_dn4, locals.var_q_pexk_nqs_dn5, locals.var_q_pexk_nqs_dn6, locals.var_q_pexk_nqs_dn7, locals.var_q_pexk_nqs_dn8, locals.var_q_pexk_nqs_dn9, locals.var_q_pexk_nqs_dn10, locals.var_q_pexk_nqs_dn11, locals.var_q_pexk_nqs_dn14, locals.var_q_pexk_nqs_dn17,)
    }
};
        locals.var_q_pexk_nqs = assign100110_e152249;
        locals.var_q_pexk_nqs_dn0 = assign100110_e152249_d_n0;
        locals.var_q_pexk_nqs_dn2 = assign100110_e152249_d_n2;
        locals.var_q_pexk_nqs_dn4 = assign100110_e152249_d_n4;
        locals.var_q_pexk_nqs_dn5 = assign100110_e152249_d_n5;
        locals.var_q_pexk_nqs_dn6 = assign100110_e152249_d_n6;
        locals.var_q_pexk_nqs_dn7 = assign100110_e152249_d_n7;
        locals.var_q_pexk_nqs_dn8 = assign100110_e152249_d_n8;
        locals.var_q_pexk_nqs_dn9 = assign100110_e152249_d_n9;
        locals.var_q_pexk_nqs_dn10 = assign100110_e152249_d_n10;
        locals.var_q_pexk_nqs_dn11 = assign100110_e152249_d_n11;
        locals.var_q_pexk_nqs_dn14 = assign100110_e152249_d_n14;
        locals.var_q_pexk_nqs_dn17 = assign100110_e152249_d_n17;

        let (assign100120_e152256, assign100120_e152256_d_n0, assign100120_e152256_d_n2, assign100120_e152256_d_n4, assign100120_e152256_d_n5, assign100120_e152256_d_n6, assign100120_e152256_d_n7, assign100120_e152256_d_n8, assign100120_e152256_d_n9, assign100120_e152256_d_n10, assign100120_e152256_d_n11, assign100120_e152256_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 == 0.0)) {
        (locals.var_q_pexk, locals.var_q_pexk_dn0, locals.var_q_pexk_dn2, locals.var_q_pexk_dn4, locals.var_q_pexk_dn5, locals.var_q_pexk_dn6, locals.var_q_pexk_dn7, locals.var_q_pexk_dn8, locals.var_q_pexk_dn9, locals.var_q_pexk_dn10, locals.var_q_pexk_dn11, locals.var_q_pexk_dn14,)
    } else {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn11, locals.var_q_qs_k_dn14,)
    }
};
        locals.var_q_qs_k = assign100120_e152256;
        locals.var_q_qs_k_dn0 = assign100120_e152256_d_n0;
        locals.var_q_qs_k_dn2 = assign100120_e152256_d_n2;
        locals.var_q_qs_k_dn4 = assign100120_e152256_d_n4;
        locals.var_q_qs_k_dn5 = assign100120_e152256_d_n5;
        locals.var_q_qs_k_dn6 = assign100120_e152256_d_n6;
        locals.var_q_qs_k_dn7 = assign100120_e152256_d_n7;
        locals.var_q_qs_k_dn8 = assign100120_e152256_d_n8;
        locals.var_q_qs_k_dn9 = assign100120_e152256_d_n9;
        locals.var_q_qs_k_dn10 = assign100120_e152256_d_n10;
        locals.var_q_qs_k_dn11 = assign100120_e152256_d_n11;
        locals.var_q_qs_k_dn14 = assign100120_e152256_d_n14;

        let (assign100130_e152263, assign100130_e152263_d_n0, assign100130_e152263_d_n2, assign100130_e152263_d_n4, assign100130_e152263_d_n5, assign100130_e152263_d_n6, assign100130_e152263_d_n7, assign100130_e152263_d_n8, assign100130_e152263_d_n9, assign100130_e152263_d_n10, assign100130_e152263_d_n11, assign100130_e152263_d_n14, assign100130_e152263_d_n17,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 == 0.0)) {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn11, locals.var_q_qs_k_dn14, 0.0,)
    } else {
        (locals.var_q_pexk_nqs, locals.var_q_pexk_nqs_dn0, locals.var_q_pexk_nqs_dn2, locals.var_q_pexk_nqs_dn4, locals.var_q_pexk_nqs_dn5, locals.var_q_pexk_nqs_dn6, locals.var_q_pexk_nqs_dn7, locals.var_q_pexk_nqs_dn8, locals.var_q_pexk_nqs_dn9, locals.var_q_pexk_nqs_dn10, locals.var_q_pexk_nqs_dn11, locals.var_q_pexk_nqs_dn14, locals.var_q_pexk_nqs_dn17,)
    }
};
        locals.var_q_pexk_nqs = assign100130_e152263;
        locals.var_q_pexk_nqs_dn0 = assign100130_e152263_d_n0;
        locals.var_q_pexk_nqs_dn2 = assign100130_e152263_d_n2;
        locals.var_q_pexk_nqs_dn4 = assign100130_e152263_d_n4;
        locals.var_q_pexk_nqs_dn5 = assign100130_e152263_d_n5;
        locals.var_q_pexk_nqs_dn6 = assign100130_e152263_d_n6;
        locals.var_q_pexk_nqs_dn7 = assign100130_e152263_d_n7;
        locals.var_q_pexk_nqs_dn8 = assign100130_e152263_d_n8;
        locals.var_q_pexk_nqs_dn9 = assign100130_e152263_d_n9;
        locals.var_q_pexk_nqs_dn10 = assign100130_e152263_d_n10;
        locals.var_q_pexk_nqs_dn11 = assign100130_e152263_d_n11;
        locals.var_q_pexk_nqs_dn14 = assign100130_e152263_d_n14;
        locals.var_q_pexk_nqs_dn17 = assign100130_e152263_d_n17;

        let (assign100140_e152269, assign100140_e152269_d_n0, assign100140_e152269_d_n2, assign100140_e152269_d_n4, assign100140_e152269_d_n5, assign100140_e152269_d_n6, assign100140_e152269_d_n7, assign100140_e152269_d_n8, assign100140_e152269_d_n9, assign100140_e152269_d_n10, assign100140_e152269_d_n11, assign100140_e152269_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100140_e152267: f64 = (p.p506 - locals.var_vbd_jct);
        (assign100140_e152267, (-locals.var_vbd_jct_dn0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbd_jct_dn10), 0.0, 0.0,)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn11, locals.var_vjunc_a_dn14,)
    }
};
        locals.var_vjunc_a = assign100140_e152269;
        locals.var_vjunc_a_dn0 = assign100140_e152269_d_n0;
        locals.var_vjunc_a_dn2 = assign100140_e152269_d_n2;
        locals.var_vjunc_a_dn4 = assign100140_e152269_d_n4;
        locals.var_vjunc_a_dn5 = assign100140_e152269_d_n5;
        locals.var_vjunc_a_dn6 = assign100140_e152269_d_n6;
        locals.var_vjunc_a_dn7 = assign100140_e152269_d_n7;
        locals.var_vjunc_a_dn8 = assign100140_e152269_d_n8;
        locals.var_vjunc_a_dn9 = assign100140_e152269_d_n9;
        locals.var_vjunc_a_dn10 = assign100140_e152269_d_n10;
        locals.var_vjunc_a_dn11 = assign100140_e152269_d_n11;
        locals.var_vjunc_a_dn14 = assign100140_e152269_d_n14;

        let (assign100150_e152282, assign100150_e152282_d_n0, assign100150_e152282_d_n2, assign100150_e152282_d_n4, assign100150_e152282_d_n5, assign100150_e152282_d_n6, assign100150_e152282_d_n7, assign100150_e152282_d_n8, assign100150_e152282_d_n9, assign100150_e152282_d_n10, assign100150_e152282_d_n11, assign100150_e152282_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100150_e152273: f64 = (locals.var_vjunc_a * locals.var_vjunc_a);
        let assign100150_e152276: f64 = (4.0 * locals.var_juncdlt);
        let assign100150_e152278: f64 = (assign100150_e152276 * locals.var_juncdlt);
        let assign100150_e152279: f64 = (assign100150_e152273 + assign100150_e152278);
        let assign100150_e152280: f64 = (assign100150_e152279).sqrt();
        (assign100150_e152280, (((locals.var_vjunc_a_dn0 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn0)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn2 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn2)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn4 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn4)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn5 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn5)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn6 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn6)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn7 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn7)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn8 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn8)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn9 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn9)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn10 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn10)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn11 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn11)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn14 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn14)) / (2.0 * assign100150_e152280)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100150_e152282;
        locals.var_tmf2_dn0 = assign100150_e152282_d_n0;
        locals.var_tmf2_dn2 = assign100150_e152282_d_n2;
        locals.var_tmf2_dn4 = assign100150_e152282_d_n4;
        locals.var_tmf2_dn5 = assign100150_e152282_d_n5;
        locals.var_tmf2_dn6 = assign100150_e152282_d_n6;
        locals.var_tmf2_dn7 = assign100150_e152282_d_n7;
        locals.var_tmf2_dn8 = assign100150_e152282_d_n8;
        locals.var_tmf2_dn9 = assign100150_e152282_d_n9;
        locals.var_tmf2_dn10 = assign100150_e152282_d_n10;
        locals.var_tmf2_dn11 = assign100150_e152282_d_n11;
        locals.var_tmf2_dn14 = assign100150_e152282_d_n14;

        let (assign100160_e152292, assign100160_e152292_d_n0, assign100160_e152292_d_n2, assign100160_e152292_d_n4, assign100160_e152292_d_n5, assign100160_e152292_d_n6, assign100160_e152292_d_n7, assign100160_e152292_d_n8, assign100160_e152292_d_n9, assign100160_e152292_d_n10, assign100160_e152292_d_n11, assign100160_e152292_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100160_e152288: f64 = (locals.var_vjunc_a / locals.var_tmf2);
        let assign100160_e152289: f64 = (1.0 + assign100160_e152288);
        let assign100160_e152290: f64 = (0.5 * assign100160_e152289);
        (assign100160_e152290, (0.5 * (((locals.var_vjunc_a_dn0 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn2 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn4 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn5 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn6 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn7 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn8 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn9 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn10 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn11 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn14 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100160_e152292;
        locals.var_t0_dn0 = assign100160_e152292_d_n0;
        locals.var_t0_dn2 = assign100160_e152292_d_n2;
        locals.var_t0_dn4 = assign100160_e152292_d_n4;
        locals.var_t0_dn5 = assign100160_e152292_d_n5;
        locals.var_t0_dn6 = assign100160_e152292_d_n6;
        locals.var_t0_dn7 = assign100160_e152292_d_n7;
        locals.var_t0_dn8 = assign100160_e152292_d_n8;
        locals.var_t0_dn9 = assign100160_e152292_d_n9;
        locals.var_t0_dn10 = assign100160_e152292_d_n10;
        locals.var_t0_dn11 = assign100160_e152292_d_n11;
        locals.var_t0_dn14 = assign100160_e152292_d_n14;

        let (assign100170_e152300, assign100170_e152300_d_n0, assign100170_e152300_d_n2, assign100170_e152300_d_n4, assign100170_e152300_d_n5, assign100170_e152300_d_n6, assign100170_e152300_d_n7, assign100170_e152300_d_n8, assign100170_e152300_d_n9, assign100170_e152300_d_n10, assign100170_e152300_d_n11, assign100170_e152300_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100170_e152297: f64 = (locals.var_vjunc_a + locals.var_tmf2);
        let assign100170_e152298: f64 = (0.5 * assign100170_e152297);
        (assign100170_e152298, (0.5 * (locals.var_vjunc_a_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vjunc_a_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vjunc_a_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vjunc_a_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vjunc_a_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vjunc_a_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vjunc_a_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vjunc_a_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vjunc_a_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vjunc_a_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vjunc_a_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn11, locals.var_vjunc_a_dn14,)
    }
};
        locals.var_vjunc_a = assign100170_e152300;
        locals.var_vjunc_a_dn0 = assign100170_e152300_d_n0;
        locals.var_vjunc_a_dn2 = assign100170_e152300_d_n2;
        locals.var_vjunc_a_dn4 = assign100170_e152300_d_n4;
        locals.var_vjunc_a_dn5 = assign100170_e152300_d_n5;
        locals.var_vjunc_a_dn6 = assign100170_e152300_d_n6;
        locals.var_vjunc_a_dn7 = assign100170_e152300_d_n7;
        locals.var_vjunc_a_dn8 = assign100170_e152300_d_n8;
        locals.var_vjunc_a_dn9 = assign100170_e152300_d_n9;
        locals.var_vjunc_a_dn10 = assign100170_e152300_d_n10;
        locals.var_vjunc_a_dn11 = assign100170_e152300_d_n11;
        locals.var_vjunc_a_dn14 = assign100170_e152300_d_n14;

        let assign100180_e152303: f64 = if locals.var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2311 = assign100180_e152303;

        let (assign100190_e152309, assign100190_e152309_d_n0, assign100190_e152309_d_n2, assign100190_e152309_d_n4, assign100190_e152309_d_n5, assign100190_e152309_d_n6, assign100190_e152309_d_n7, assign100190_e152309_d_n8, assign100190_e152309_d_n9, assign100190_e152309_d_n10, assign100190_e152309_d_n11, assign100190_e152309_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2311 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn11, locals.var_vjunc_a_dn14,)
    }
};
        locals.var_vjunc_a = assign100190_e152309;
        locals.var_vjunc_a_dn0 = assign100190_e152309_d_n0;
        locals.var_vjunc_a_dn2 = assign100190_e152309_d_n2;
        locals.var_vjunc_a_dn4 = assign100190_e152309_d_n4;
        locals.var_vjunc_a_dn5 = assign100190_e152309_d_n5;
        locals.var_vjunc_a_dn6 = assign100190_e152309_d_n6;
        locals.var_vjunc_a_dn7 = assign100190_e152309_d_n7;
        locals.var_vjunc_a_dn8 = assign100190_e152309_d_n8;
        locals.var_vjunc_a_dn9 = assign100190_e152309_d_n9;
        locals.var_vjunc_a_dn10 = assign100190_e152309_d_n10;
        locals.var_vjunc_a_dn11 = assign100190_e152309_d_n11;
        locals.var_vjunc_a_dn14 = assign100190_e152309_d_n14;

        let (assign100200_e152315, assign100200_e152315_d_n0, assign100200_e152315_d_n2, assign100200_e152315_d_n4, assign100200_e152315_d_n5, assign100200_e152315_d_n6, assign100200_e152315_d_n7, assign100200_e152315_d_n8, assign100200_e152315_d_n9, assign100200_e152315_d_n10, assign100200_e152315_d_n11, assign100200_e152315_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2311 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100200_e152315;
        locals.var_t0_dn0 = assign100200_e152315_d_n0;
        locals.var_t0_dn2 = assign100200_e152315_d_n2;
        locals.var_t0_dn4 = assign100200_e152315_d_n4;
        locals.var_t0_dn5 = assign100200_e152315_d_n5;
        locals.var_t0_dn6 = assign100200_e152315_d_n6;
        locals.var_t0_dn7 = assign100200_e152315_d_n7;
        locals.var_t0_dn8 = assign100200_e152315_d_n8;
        locals.var_t0_dn9 = assign100200_e152315_d_n9;
        locals.var_t0_dn10 = assign100200_e152315_d_n10;
        locals.var_t0_dn11 = assign100200_e152315_d_n11;
        locals.var_t0_dn14 = assign100200_e152315_d_n14;

        let (assign100210_e152328, assign100210_e152328_d_n0, assign100210_e152328_d_n2, assign100210_e152328_d_n4, assign100210_e152328_d_n5, assign100210_e152328_d_n6, assign100210_e152328_d_n7, assign100210_e152328_d_n8, assign100210_e152328_d_n9, assign100210_e152328_d_n10, assign100210_e152328_d_n11, assign100210_e152328_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100210_e152319: f64 = (2.0 * 1.034943e-10);
        let assign100210_e152321: f64 = (assign100210_e152319 * locals.var_vjunc_a);
        let assign100210_e152324: f64 = (1.6021918e-19 * locals.var_ndi_i);
        let assign100210_e152325: f64 = (assign100210_e152321 / assign100210_e152324);
        let assign100210_e152326: f64 = (assign100210_e152325).sqrt();
        (assign100210_e152326, (((assign100210_e152319 * locals.var_vjunc_a_dn0) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn2) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn4) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn5) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn6) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn7) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn8) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn9) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn10) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn11) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn14) / assign100210_e152324) / (2.0 * assign100210_e152326)),)
    } else {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn11, locals.var_w_depa_dn14,)
    }
};
        locals.var_w_depa = assign100210_e152328;
        locals.var_w_depa_dn0 = assign100210_e152328_d_n0;
        locals.var_w_depa_dn2 = assign100210_e152328_d_n2;
        locals.var_w_depa_dn4 = assign100210_e152328_d_n4;
        locals.var_w_depa_dn5 = assign100210_e152328_d_n5;
        locals.var_w_depa_dn6 = assign100210_e152328_d_n6;
        locals.var_w_depa_dn7 = assign100210_e152328_d_n7;
        locals.var_w_depa_dn8 = assign100210_e152328_d_n8;
        locals.var_w_depa_dn9 = assign100210_e152328_d_n9;
        locals.var_w_depa_dn10 = assign100210_e152328_d_n10;
        locals.var_w_depa_dn11 = assign100210_e152328_d_n11;
        locals.var_w_depa_dn14 = assign100210_e152328_d_n14;

        let (assign100220_e152336, assign100220_e152336_d_n0, assign100220_e152336_d_n2, assign100220_e152336_d_n4, assign100220_e152336_d_n5, assign100220_e152336_d_n6, assign100220_e152336_d_n7, assign100220_e152336_d_n8, assign100220_e152336_d_n9, assign100220_e152336_d_n10, assign100220_e152336_d_n11, assign100220_e152336_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100220_e152332: f64 = (p.p545 - locals.var_w_depa);
        let assign100220_e152334: f64 = (assign100220_e152332 - 1e-7);
        (assign100220_e152334, (-locals.var_w_depa_dn0), (-locals.var_w_depa_dn2), (-locals.var_w_depa_dn4), (-locals.var_w_depa_dn5), (-locals.var_w_depa_dn6), (-locals.var_w_depa_dn7), (-locals.var_w_depa_dn8), (-locals.var_w_depa_dn9), (-locals.var_w_depa_dn10), (-locals.var_w_depa_dn11), (-locals.var_w_depa_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign100220_e152336;
        locals.var_tmf1_dn0 = assign100220_e152336_d_n0;
        locals.var_tmf1_dn2 = assign100220_e152336_d_n2;
        locals.var_tmf1_dn4 = assign100220_e152336_d_n4;
        locals.var_tmf1_dn5 = assign100220_e152336_d_n5;
        locals.var_tmf1_dn6 = assign100220_e152336_d_n6;
        locals.var_tmf1_dn7 = assign100220_e152336_d_n7;
        locals.var_tmf1_dn8 = assign100220_e152336_d_n8;
        locals.var_tmf1_dn9 = assign100220_e152336_d_n9;
        locals.var_tmf1_dn10 = assign100220_e152336_d_n10;
        locals.var_tmf1_dn11 = assign100220_e152336_d_n11;
        locals.var_tmf1_dn14 = assign100220_e152336_d_n14;

        let (assign100230_e152344, assign100230_e152344_d_n0, assign100230_e152344_d_n2, assign100230_e152344_d_n4, assign100230_e152344_d_n5, assign100230_e152344_d_n6, assign100230_e152344_d_n7, assign100230_e152344_d_n8, assign100230_e152344_d_n9, assign100230_e152344_d_n10, assign100230_e152344_d_n11, assign100230_e152344_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100230_e152340: f64 = (4.0 * p.p545);
        let assign100230_e152342: f64 = (assign100230_e152340 * 1e-7);
        (assign100230_e152342, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100230_e152344;
        locals.var_tmf2_dn0 = assign100230_e152344_d_n0;
        locals.var_tmf2_dn2 = assign100230_e152344_d_n2;
        locals.var_tmf2_dn4 = assign100230_e152344_d_n4;
        locals.var_tmf2_dn5 = assign100230_e152344_d_n5;
        locals.var_tmf2_dn6 = assign100230_e152344_d_n6;
        locals.var_tmf2_dn7 = assign100230_e152344_d_n7;
        locals.var_tmf2_dn8 = assign100230_e152344_d_n8;
        locals.var_tmf2_dn9 = assign100230_e152344_d_n9;
        locals.var_tmf2_dn10 = assign100230_e152344_d_n10;
        locals.var_tmf2_dn11 = assign100230_e152344_d_n11;
        locals.var_tmf2_dn14 = assign100230_e152344_d_n14;

        let (assign100240_e152354, assign100240_e152354_d_n0, assign100240_e152354_d_n2, assign100240_e152354_d_n4, assign100240_e152354_d_n5, assign100240_e152354_d_n6, assign100240_e152354_d_n7, assign100240_e152354_d_n8, assign100240_e152354_d_n9, assign100240_e152354_d_n10, assign100240_e152354_d_n11, assign100240_e152354_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let (assign100240_e152352, assign100240_e152352_d_n0, assign100240_e152352_d_n2, assign100240_e152352_d_n4, assign100240_e152352_d_n5, assign100240_e152352_d_n6, assign100240_e152352_d_n7, assign100240_e152352_d_n8, assign100240_e152352_d_n9, assign100240_e152352_d_n10, assign100240_e152352_d_n11, assign100240_e152352_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign100240_e152351: f64 = (-locals.var_tmf2);
                (assign100240_e152351, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign100240_e152352, assign100240_e152352_d_n0, assign100240_e152352_d_n2, assign100240_e152352_d_n4, assign100240_e152352_d_n5, assign100240_e152352_d_n6, assign100240_e152352_d_n7, assign100240_e152352_d_n8, assign100240_e152352_d_n9, assign100240_e152352_d_n10, assign100240_e152352_d_n11, assign100240_e152352_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100240_e152354;
        locals.var_tmf2_dn0 = assign100240_e152354_d_n0;
        locals.var_tmf2_dn2 = assign100240_e152354_d_n2;
        locals.var_tmf2_dn4 = assign100240_e152354_d_n4;
        locals.var_tmf2_dn5 = assign100240_e152354_d_n5;
        locals.var_tmf2_dn6 = assign100240_e152354_d_n6;
        locals.var_tmf2_dn7 = assign100240_e152354_d_n7;
        locals.var_tmf2_dn8 = assign100240_e152354_d_n8;
        locals.var_tmf2_dn9 = assign100240_e152354_d_n9;
        locals.var_tmf2_dn10 = assign100240_e152354_d_n10;
        locals.var_tmf2_dn11 = assign100240_e152354_d_n11;
        locals.var_tmf2_dn14 = assign100240_e152354_d_n14;

        let (assign100250_e152363, assign100250_e152363_d_n0, assign100250_e152363_d_n2, assign100250_e152363_d_n4, assign100250_e152363_d_n5, assign100250_e152363_d_n6, assign100250_e152363_d_n7, assign100250_e152363_d_n8, assign100250_e152363_d_n9, assign100250_e152363_d_n10, assign100250_e152363_d_n11, assign100250_e152363_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100250_e152358: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign100250_e152360: f64 = (assign100250_e152358 + locals.var_tmf2);
        let assign100250_e152361: f64 = (assign100250_e152360).sqrt();
        (assign100250_e152361, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign100250_e152361)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100250_e152363;
        locals.var_tmf2_dn0 = assign100250_e152363_d_n0;
        locals.var_tmf2_dn2 = assign100250_e152363_d_n2;
        locals.var_tmf2_dn4 = assign100250_e152363_d_n4;
        locals.var_tmf2_dn5 = assign100250_e152363_d_n5;
        locals.var_tmf2_dn6 = assign100250_e152363_d_n6;
        locals.var_tmf2_dn7 = assign100250_e152363_d_n7;
        locals.var_tmf2_dn8 = assign100250_e152363_d_n8;
        locals.var_tmf2_dn9 = assign100250_e152363_d_n9;
        locals.var_tmf2_dn10 = assign100250_e152363_d_n10;
        locals.var_tmf2_dn11 = assign100250_e152363_d_n11;
        locals.var_tmf2_dn14 = assign100250_e152363_d_n14;

        let (assign100260_e152373, assign100260_e152373_d_n0, assign100260_e152373_d_n2, assign100260_e152373_d_n4, assign100260_e152373_d_n5, assign100260_e152373_d_n6, assign100260_e152373_d_n7, assign100260_e152373_d_n8, assign100260_e152373_d_n9, assign100260_e152373_d_n10, assign100260_e152373_d_n11, assign100260_e152373_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100260_e152369: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign100260_e152370: f64 = (1.0 + assign100260_e152369);
        let assign100260_e152371: f64 = (0.5 * assign100260_e152370);
        (assign100260_e152371, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100260_e152373;
        locals.var_t0_dn0 = assign100260_e152373_d_n0;
        locals.var_t0_dn2 = assign100260_e152373_d_n2;
        locals.var_t0_dn4 = assign100260_e152373_d_n4;
        locals.var_t0_dn5 = assign100260_e152373_d_n5;
        locals.var_t0_dn6 = assign100260_e152373_d_n6;
        locals.var_t0_dn7 = assign100260_e152373_d_n7;
        locals.var_t0_dn8 = assign100260_e152373_d_n8;
        locals.var_t0_dn9 = assign100260_e152373_d_n9;
        locals.var_t0_dn10 = assign100260_e152373_d_n10;
        locals.var_t0_dn11 = assign100260_e152373_d_n11;
        locals.var_t0_dn14 = assign100260_e152373_d_n14;

        let (assign100270_e152383, assign100270_e152383_d_n0, assign100270_e152383_d_n2, assign100270_e152383_d_n4, assign100270_e152383_d_n5, assign100270_e152383_d_n6, assign100270_e152383_d_n7, assign100270_e152383_d_n8, assign100270_e152383_d_n9, assign100270_e152383_d_n10, assign100270_e152383_d_n11, assign100270_e152383_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100270_e152379: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign100270_e152380: f64 = (0.5 * assign100270_e152379);
        let assign100270_e152381: f64 = (p.p545 - assign100270_e152380);
        (assign100270_e152381, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn11, locals.var_w_depa_dn14,)
    }
};
        locals.var_w_depa = assign100270_e152383;
        locals.var_w_depa_dn0 = assign100270_e152383_d_n0;
        locals.var_w_depa_dn2 = assign100270_e152383_d_n2;
        locals.var_w_depa_dn4 = assign100270_e152383_d_n4;
        locals.var_w_depa_dn5 = assign100270_e152383_d_n5;
        locals.var_w_depa_dn6 = assign100270_e152383_d_n6;
        locals.var_w_depa_dn7 = assign100270_e152383_d_n7;
        locals.var_w_depa_dn8 = assign100270_e152383_d_n8;
        locals.var_w_depa_dn9 = assign100270_e152383_d_n9;
        locals.var_w_depa_dn10 = assign100270_e152383_d_n10;
        locals.var_w_depa_dn11 = assign100270_e152383_d_n11;
        locals.var_w_depa_dn14 = assign100270_e152383_d_n14;

        let assign100280_e152386: f64 = if p.p546 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2312 = assign100280_e152386;

        let (assign100290_e152394, assign100290_e152394_d_n0, assign100290_e152394_d_n2, assign100290_e152394_d_n4, assign100290_e152394_d_n5, assign100290_e152394_d_n6, assign100290_e152394_d_n7, assign100290_e152394_d_n8, assign100290_e152394_d_n9, assign100290_e152394_d_n10, assign100290_e152394_d_n11, assign100290_e152394_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100290_e152392: f64 = (locals.var_w_depa * p.p546);
        (assign100290_e152392, (locals.var_w_depa_dn0 * p.p546), (locals.var_w_depa_dn2 * p.p546), (locals.var_w_depa_dn4 * p.p546), (locals.var_w_depa_dn5 * p.p546), (locals.var_w_depa_dn6 * p.p546), (locals.var_w_depa_dn7 * p.p546), (locals.var_w_depa_dn8 * p.p546), (locals.var_w_depa_dn9 * p.p546), (locals.var_w_depa_dn10 * p.p546), (locals.var_w_depa_dn11 * p.p546), (locals.var_w_depa_dn14 * p.p546),)
    } else {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn11, locals.var_w_qs_a_dn14,)
    }
};
        locals.var_w_qs_a = assign100290_e152394;
        locals.var_w_qs_a_dn0 = assign100290_e152394_d_n0;
        locals.var_w_qs_a_dn2 = assign100290_e152394_d_n2;
        locals.var_w_qs_a_dn4 = assign100290_e152394_d_n4;
        locals.var_w_qs_a_dn5 = assign100290_e152394_d_n5;
        locals.var_w_qs_a_dn6 = assign100290_e152394_d_n6;
        locals.var_w_qs_a_dn7 = assign100290_e152394_d_n7;
        locals.var_w_qs_a_dn8 = assign100290_e152394_d_n8;
        locals.var_w_qs_a_dn9 = assign100290_e152394_d_n9;
        locals.var_w_qs_a_dn10 = assign100290_e152394_d_n10;
        locals.var_w_qs_a_dn11 = assign100290_e152394_d_n11;
        locals.var_w_qs_a_dn14 = assign100290_e152394_d_n14;

        let (assign100300_e152402, assign100300_e152402_d_n18,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100300_e152400: f64 = (p.p546 * (nv18 - 0.0));
        (assign100300_e152400, p.p546,)
    } else {
        (locals.var_w_nqs_a, locals.var_w_nqs_a_dn18,)
    }
};
        locals.var_w_nqs_a = assign100300_e152402;
        locals.var_w_nqs_a_dn18 = assign100300_e152402_d_n18;

        let (assign100310_e152412, assign100310_e152412_d_n0, assign100310_e152412_d_n2, assign100310_e152412_d_n4, assign100310_e152412_d_n5, assign100310_e152412_d_n6, assign100310_e152412_d_n7, assign100310_e152412_d_n8, assign100310_e152412_d_n9, assign100310_e152412_d_n10, assign100310_e152412_d_n11, assign100310_e152412_d_n14, assign100310_e152412_d_n18,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100310_e152408: f64 = (locals.var_w_nqs_a - locals.var_w_qs_a);
        let assign100310_e152410: f64 = (assign100310_e152408 / p.p546);
        (assign100310_e152410, ((-locals.var_w_qs_a_dn0) / p.p546), ((-locals.var_w_qs_a_dn2) / p.p546), ((-locals.var_w_qs_a_dn4) / p.p546), ((-locals.var_w_qs_a_dn5) / p.p546), ((-locals.var_w_qs_a_dn6) / p.p546), ((-locals.var_w_qs_a_dn7) / p.p546), ((-locals.var_w_qs_a_dn8) / p.p546), ((-locals.var_w_qs_a_dn9) / p.p546), ((-locals.var_w_qs_a_dn10) / p.p546), ((-locals.var_w_qs_a_dn11) / p.p546), ((-locals.var_w_qs_a_dn14) / p.p546), (locals.var_w_nqs_a_dn18 / p.p546),)
    } else {
        (locals.var_iwnqs0_a, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn11, locals.var_iwnqs0_a_dn14, locals.var_iwnqs0_a_dn18,)
    }
};
        locals.var_iwnqs0_a = assign100310_e152412;
        locals.var_iwnqs0_a_dn0 = assign100310_e152412_d_n0;
        locals.var_iwnqs0_a_dn2 = assign100310_e152412_d_n2;
        locals.var_iwnqs0_a_dn4 = assign100310_e152412_d_n4;
        locals.var_iwnqs0_a_dn5 = assign100310_e152412_d_n5;
        locals.var_iwnqs0_a_dn6 = assign100310_e152412_d_n6;
        locals.var_iwnqs0_a_dn7 = assign100310_e152412_d_n7;
        locals.var_iwnqs0_a_dn8 = assign100310_e152412_d_n8;
        locals.var_iwnqs0_a_dn9 = assign100310_e152412_d_n9;
        locals.var_iwnqs0_a_dn10 = assign100310_e152412_d_n10;
        locals.var_iwnqs0_a_dn11 = assign100310_e152412_d_n11;
        locals.var_iwnqs0_a_dn14 = assign100310_e152412_d_n14;
        locals.var_iwnqs0_a_dn18 = assign100310_e152412_d_n18;

    }
}
