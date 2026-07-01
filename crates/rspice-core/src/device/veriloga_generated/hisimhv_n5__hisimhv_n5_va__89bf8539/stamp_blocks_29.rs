#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_75(
        locals: &mut StampLocals,
    ) {
        let assign25290_e23575: f64 = if ((locals.var_w_bsub0 > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard618 = assign25290_e23575;
        locals.var_guard618_rv = 0.0;

        let assign25300_e23579: f64 = (locals.var_phi_s0_dep - 0.02);
        let assign25300_e23584: f64 = if ((locals.var_phi_b0_dep > assign25300_e23579) && (0.02 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard619 = assign25300_e23584;
        locals.var_guard619_rv = 0.0;

        let (assign25310_e23598, assign25310_e23598_d_n0, assign25310_e23598_d_n2, assign25310_e23598_d_n4, assign25310_e23598_d_n5, assign25310_e23598_d_n6, assign25310_e23598_d_n7, assign25310_e23598_d_n8, assign25310_e23598_d_n9, assign25310_e23598_d_n10, assign25310_e23598_d_n11, assign25310_e23598_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25310_e23594: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep);
        let assign25310_e23596: f64 = (assign25310_e23594 + 0.02);
        (assign25310_e23596, (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_dn10), (locals.var_phi_b0_dep_dn11 - locals.var_phi_s0_dep_dn11), (locals.var_phi_b0_dep_dn14 - locals.var_phi_s0_dep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign25310_e23598;
        locals.var_tmf1_dn0 = assign25310_e23598_d_n0;
        locals.var_tmf1_dn2 = assign25310_e23598_d_n2;
        locals.var_tmf1_dn4 = assign25310_e23598_d_n4;
        locals.var_tmf1_dn5 = assign25310_e23598_d_n5;
        locals.var_tmf1_dn6 = assign25310_e23598_d_n6;
        locals.var_tmf1_dn7 = assign25310_e23598_d_n7;
        locals.var_tmf1_dn8 = assign25310_e23598_d_n8;
        locals.var_tmf1_dn9 = assign25310_e23598_d_n9;
        locals.var_tmf1_dn10 = assign25310_e23598_d_n10;
        locals.var_tmf1_dn11 = assign25310_e23598_d_n11;
        locals.var_tmf1_dn14 = assign25310_e23598_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign25320_e23610, assign25320_e23610_d_n0, assign25320_e23610_d_n2, assign25320_e23610_d_n4, assign25320_e23610_d_n5, assign25320_e23610_d_n6, assign25320_e23610_d_n7, assign25320_e23610_d_n8, assign25320_e23610_d_n9, assign25320_e23610_d_n10, assign25320_e23610_d_n11, assign25320_e23610_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25320_e23608: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign25320_e23608, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign25320_e23610;
        locals.var_x2_dn0 = assign25320_e23610_d_n0;
        locals.var_x2_dn2 = assign25320_e23610_d_n2;
        locals.var_x2_dn4 = assign25320_e23610_d_n4;
        locals.var_x2_dn5 = assign25320_e23610_d_n5;
        locals.var_x2_dn6 = assign25320_e23610_d_n6;
        locals.var_x2_dn7 = assign25320_e23610_d_n7;
        locals.var_x2_dn8 = assign25320_e23610_d_n8;
        locals.var_x2_dn9 = assign25320_e23610_d_n9;
        locals.var_x2_dn10 = assign25320_e23610_d_n10;
        locals.var_x2_dn11 = assign25320_e23610_d_n11;
        locals.var_x2_dn14 = assign25320_e23610_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign25330_e23622, assign25330_e23622_d_n0, assign25330_e23622_d_n2, assign25330_e23622_d_n4, assign25330_e23622_d_n5, assign25330_e23622_d_n6, assign25330_e23622_d_n7, assign25330_e23622_d_n8, assign25330_e23622_d_n9, assign25330_e23622_d_n10, assign25330_e23622_d_n11, assign25330_e23622_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25330_e23620: f64 = (0.02 * 0.02);
        (assign25330_e23620, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign25330_e23622;
        locals.var_xmax2_dn0 = assign25330_e23622_d_n0;
        locals.var_xmax2_dn2 = assign25330_e23622_d_n2;
        locals.var_xmax2_dn4 = assign25330_e23622_d_n4;
        locals.var_xmax2_dn5 = assign25330_e23622_d_n5;
        locals.var_xmax2_dn6 = assign25330_e23622_d_n6;
        locals.var_xmax2_dn7 = assign25330_e23622_d_n7;
        locals.var_xmax2_dn8 = assign25330_e23622_d_n8;
        locals.var_xmax2_dn9 = assign25330_e23622_d_n9;
        locals.var_xmax2_dn10 = assign25330_e23622_d_n10;
        locals.var_xmax2_dn11 = assign25330_e23622_d_n11;
        locals.var_xmax2_dn14 = assign25330_e23622_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign25340_e23632, assign25340_e23632_d_n0, assign25340_e23632_d_n2, assign25340_e23632_d_n4, assign25340_e23632_d_n5, assign25340_e23632_d_n6, assign25340_e23632_d_n7, assign25340_e23632_d_n8, assign25340_e23632_d_n9, assign25340_e23632_d_n10, assign25340_e23632_d_n11, assign25340_e23632_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25340_e23632;
        locals.var_xp_dn0 = assign25340_e23632_d_n0;
        locals.var_xp_dn2 = assign25340_e23632_d_n2;
        locals.var_xp_dn4 = assign25340_e23632_d_n4;
        locals.var_xp_dn5 = assign25340_e23632_d_n5;
        locals.var_xp_dn6 = assign25340_e23632_d_n6;
        locals.var_xp_dn7 = assign25340_e23632_d_n7;
        locals.var_xp_dn8 = assign25340_e23632_d_n8;
        locals.var_xp_dn9 = assign25340_e23632_d_n9;
        locals.var_xp_dn10 = assign25340_e23632_d_n10;
        locals.var_xp_dn11 = assign25340_e23632_d_n11;
        locals.var_xp_dn14 = assign25340_e23632_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign25350_e23642, assign25350_e23642_d_n0, assign25350_e23642_d_n2, assign25350_e23642_d_n4, assign25350_e23642_d_n5, assign25350_e23642_d_n6, assign25350_e23642_d_n7, assign25350_e23642_d_n8, assign25350_e23642_d_n9, assign25350_e23642_d_n10, assign25350_e23642_d_n11, assign25350_e23642_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25350_e23642;
        locals.var_xmp_dn0 = assign25350_e23642_d_n0;
        locals.var_xmp_dn2 = assign25350_e23642_d_n2;
        locals.var_xmp_dn4 = assign25350_e23642_d_n4;
        locals.var_xmp_dn5 = assign25350_e23642_d_n5;
        locals.var_xmp_dn6 = assign25350_e23642_d_n6;
        locals.var_xmp_dn7 = assign25350_e23642_d_n7;
        locals.var_xmp_dn8 = assign25350_e23642_d_n8;
        locals.var_xmp_dn9 = assign25350_e23642_d_n9;
        locals.var_xmp_dn10 = assign25350_e23642_d_n10;
        locals.var_xmp_dn11 = assign25350_e23642_d_n11;
        locals.var_xmp_dn14 = assign25350_e23642_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25360_e23652,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign25360_e23652;
        locals.var_m0_rv = 0.0;

        let (assign25370_e23662,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25370_e23662;
        locals.var_mm_rv = 0.0;

        let (assign25380_e23672, assign25380_e23672_d_n0, assign25380_e23672_d_n2, assign25380_e23672_d_n4, assign25380_e23672_d_n5, assign25380_e23672_d_n6, assign25380_e23672_d_n7, assign25380_e23672_d_n8, assign25380_e23672_d_n9, assign25380_e23672_d_n10, assign25380_e23672_d_n11, assign25380_e23672_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign25380_e23672;
        locals.var_arg_dn0 = assign25380_e23672_d_n0;
        locals.var_arg_dn2 = assign25380_e23672_d_n2;
        locals.var_arg_dn4 = assign25380_e23672_d_n4;
        locals.var_arg_dn5 = assign25380_e23672_d_n5;
        locals.var_arg_dn6 = assign25380_e23672_d_n6;
        locals.var_arg_dn7 = assign25380_e23672_d_n7;
        locals.var_arg_dn8 = assign25380_e23672_d_n8;
        locals.var_arg_dn9 = assign25380_e23672_d_n9;
        locals.var_arg_dn10 = assign25380_e23672_d_n10;
        locals.var_arg_dn11 = assign25380_e23672_d_n11;
        locals.var_arg_dn14 = assign25380_e23672_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign25390_e23682, assign25390_e23682_d_n0, assign25390_e23682_d_n2, assign25390_e23682_d_n4, assign25390_e23682_d_n5, assign25390_e23682_d_n6, assign25390_e23682_d_n7, assign25390_e23682_d_n8, assign25390_e23682_d_n9, assign25390_e23682_d_n10, assign25390_e23682_d_n11, assign25390_e23682_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25390_e23682;
        locals.var_dnm_dn0 = assign25390_e23682_d_n0;
        locals.var_dnm_dn2 = assign25390_e23682_d_n2;
        locals.var_dnm_dn4 = assign25390_e23682_d_n4;
        locals.var_dnm_dn5 = assign25390_e23682_d_n5;
        locals.var_dnm_dn6 = assign25390_e23682_d_n6;
        locals.var_dnm_dn7 = assign25390_e23682_d_n7;
        locals.var_dnm_dn8 = assign25390_e23682_d_n8;
        locals.var_dnm_dn9 = assign25390_e23682_d_n9;
        locals.var_dnm_dn10 = assign25390_e23682_d_n10;
        locals.var_dnm_dn11 = assign25390_e23682_d_n11;
        locals.var_dnm_dn14 = assign25390_e23682_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign25400_e23694, assign25400_e23694_d_n0, assign25400_e23694_d_n2, assign25400_e23694_d_n4, assign25400_e23694_d_n5, assign25400_e23694_d_n6, assign25400_e23694_d_n7, assign25400_e23694_d_n8, assign25400_e23694_d_n9, assign25400_e23694_d_n10, assign25400_e23694_d_n11, assign25400_e23694_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25400_e23692: f64 = (locals.var_xp * locals.var_x2);
        (assign25400_e23692, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25400_e23694;
        locals.var_xp_dn0 = assign25400_e23694_d_n0;
        locals.var_xp_dn2 = assign25400_e23694_d_n2;
        locals.var_xp_dn4 = assign25400_e23694_d_n4;
        locals.var_xp_dn5 = assign25400_e23694_d_n5;
        locals.var_xp_dn6 = assign25400_e23694_d_n6;
        locals.var_xp_dn7 = assign25400_e23694_d_n7;
        locals.var_xp_dn8 = assign25400_e23694_d_n8;
        locals.var_xp_dn9 = assign25400_e23694_d_n9;
        locals.var_xp_dn10 = assign25400_e23694_d_n10;
        locals.var_xp_dn11 = assign25400_e23694_d_n11;
        locals.var_xp_dn14 = assign25400_e23694_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign25410_e23706, assign25410_e23706_d_n0, assign25410_e23706_d_n2, assign25410_e23706_d_n4, assign25410_e23706_d_n5, assign25410_e23706_d_n6, assign25410_e23706_d_n7, assign25410_e23706_d_n8, assign25410_e23706_d_n9, assign25410_e23706_d_n10, assign25410_e23706_d_n11, assign25410_e23706_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25410_e23704: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign25410_e23704, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25410_e23706;
        locals.var_xmp_dn0 = assign25410_e23706_d_n0;
        locals.var_xmp_dn2 = assign25410_e23706_d_n2;
        locals.var_xmp_dn4 = assign25410_e23706_d_n4;
        locals.var_xmp_dn5 = assign25410_e23706_d_n5;
        locals.var_xmp_dn6 = assign25410_e23706_d_n6;
        locals.var_xmp_dn7 = assign25410_e23706_d_n7;
        locals.var_xmp_dn8 = assign25410_e23706_d_n8;
        locals.var_xmp_dn9 = assign25410_e23706_d_n9;
        locals.var_xmp_dn10 = assign25410_e23706_d_n10;
        locals.var_xmp_dn11 = assign25410_e23706_d_n11;
        locals.var_xmp_dn14 = assign25410_e23706_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25420_e23718, assign25420_e23718_d_n0, assign25420_e23718_d_n2, assign25420_e23718_d_n4, assign25420_e23718_d_n5, assign25420_e23718_d_n6, assign25420_e23718_d_n7, assign25420_e23718_d_n8, assign25420_e23718_d_n9, assign25420_e23718_d_n10, assign25420_e23718_d_n11, assign25420_e23718_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25420_e23716: f64 = (locals.var_xp * locals.var_x2);
        (assign25420_e23716, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25420_e23718;
        locals.var_xp_dn0 = assign25420_e23718_d_n0;
        locals.var_xp_dn2 = assign25420_e23718_d_n2;
        locals.var_xp_dn4 = assign25420_e23718_d_n4;
        locals.var_xp_dn5 = assign25420_e23718_d_n5;
        locals.var_xp_dn6 = assign25420_e23718_d_n6;
        locals.var_xp_dn7 = assign25420_e23718_d_n7;
        locals.var_xp_dn8 = assign25420_e23718_d_n8;
        locals.var_xp_dn9 = assign25420_e23718_d_n9;
        locals.var_xp_dn10 = assign25420_e23718_d_n10;
        locals.var_xp_dn11 = assign25420_e23718_d_n11;
        locals.var_xp_dn14 = assign25420_e23718_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign25430_e23730, assign25430_e23730_d_n0, assign25430_e23730_d_n2, assign25430_e23730_d_n4, assign25430_e23730_d_n5, assign25430_e23730_d_n6, assign25430_e23730_d_n7, assign25430_e23730_d_n8, assign25430_e23730_d_n9, assign25430_e23730_d_n10, assign25430_e23730_d_n11, assign25430_e23730_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25430_e23728: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign25430_e23728, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25430_e23730;
        locals.var_xmp_dn0 = assign25430_e23730_d_n0;
        locals.var_xmp_dn2 = assign25430_e23730_d_n2;
        locals.var_xmp_dn4 = assign25430_e23730_d_n4;
        locals.var_xmp_dn5 = assign25430_e23730_d_n5;
        locals.var_xmp_dn6 = assign25430_e23730_d_n6;
        locals.var_xmp_dn7 = assign25430_e23730_d_n7;
        locals.var_xmp_dn8 = assign25430_e23730_d_n8;
        locals.var_xmp_dn9 = assign25430_e23730_d_n9;
        locals.var_xmp_dn10 = assign25430_e23730_d_n10;
        locals.var_xmp_dn11 = assign25430_e23730_d_n11;
        locals.var_xmp_dn14 = assign25430_e23730_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25440_e23742, assign25440_e23742_d_n0, assign25440_e23742_d_n2, assign25440_e23742_d_n4, assign25440_e23742_d_n5, assign25440_e23742_d_n6, assign25440_e23742_d_n7, assign25440_e23742_d_n8, assign25440_e23742_d_n9, assign25440_e23742_d_n10, assign25440_e23742_d_n11, assign25440_e23742_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25440_e23740: f64 = (locals.var_xp + locals.var_xmp);
        (assign25440_e23740, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign25440_e23742;
        locals.var_arg_dn0 = assign25440_e23742_d_n0;
        locals.var_arg_dn2 = assign25440_e23742_d_n2;
        locals.var_arg_dn4 = assign25440_e23742_d_n4;
        locals.var_arg_dn5 = assign25440_e23742_d_n5;
        locals.var_arg_dn6 = assign25440_e23742_d_n6;
        locals.var_arg_dn7 = assign25440_e23742_d_n7;
        locals.var_arg_dn8 = assign25440_e23742_d_n8;
        locals.var_arg_dn9 = assign25440_e23742_d_n9;
        locals.var_arg_dn10 = assign25440_e23742_d_n10;
        locals.var_arg_dn11 = assign25440_e23742_d_n11;
        locals.var_arg_dn14 = assign25440_e23742_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign25450_e23752, assign25450_e23752_d_n0, assign25450_e23752_d_n2, assign25450_e23752_d_n4, assign25450_e23752_d_n5, assign25450_e23752_d_n6, assign25450_e23752_d_n7, assign25450_e23752_d_n8, assign25450_e23752_d_n9, assign25450_e23752_d_n10, assign25450_e23752_d_n11, assign25450_e23752_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25450_e23752;
        locals.var_dnm_dn0 = assign25450_e23752_d_n0;
        locals.var_dnm_dn2 = assign25450_e23752_d_n2;
        locals.var_dnm_dn4 = assign25450_e23752_d_n4;
        locals.var_dnm_dn5 = assign25450_e23752_d_n5;
        locals.var_dnm_dn6 = assign25450_e23752_d_n6;
        locals.var_dnm_dn7 = assign25450_e23752_d_n7;
        locals.var_dnm_dn8 = assign25450_e23752_d_n8;
        locals.var_dnm_dn9 = assign25450_e23752_d_n9;
        locals.var_dnm_dn10 = assign25450_e23752_d_n10;
        locals.var_dnm_dn11 = assign25450_e23752_d_n11;
        locals.var_dnm_dn14 = assign25450_e23752_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign25460_e23767: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard620 = assign25460_e23767;
        locals.var_guard620_rv = 0.0;

        let assign25470_e23770: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard621 = assign25470_e23770;
        locals.var_guard621_rv = 0.0;

        let (assign25480_e23784,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25480_e23784;
        locals.var_mm_rv = 0.0;

        let assign25490_e23787: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard622 = assign25490_e23787;
        locals.var_guard622_rv = 0.0;

        let (assign25500_e23804,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 == 0.0)) && (locals.var_guard622 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25500_e23804;
        locals.var_mm_rv = 0.0;

        let assign25510_e23807: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard623 = assign25510_e23807;
        locals.var_guard623_rv = 0.0;

        let (assign25520_e23827,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 == 0.0)) && (locals.var_guard622 == 0.0)) && (locals.var_guard623 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25520_e23827;
        locals.var_mm_rv = 0.0;

        let assign25530_e23830: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard624 = assign25530_e23830;
        locals.var_guard624_rv = 0.0;

        let (assign25540_e23853,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_guard621 == 0.0)) && (locals.var_guard622 == 0.0)) && (locals.var_guard623 == 0.0)) && (locals.var_guard624 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25540_e23853;
        locals.var_mm_rv = 0.0;

        let (assign25550_e23865,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) && (locals.var_guard620 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign25550_e23865;
        locals.var_m0_rv = 0.0;

        let mut assign25560_loop_guard: usize = 0;
        while {
            let assign25560_cond_e23878: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) && (locals.var_guard620 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign25560_cond_e23878 != 0.0
        } {
            assign25560_loop_guard += 1;
            assert!(assign25560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign25560_body0_e23891, assign25560_body0_e23891_d_n0, assign25560_body0_e23891_d_n2, assign25560_body0_e23891_d_n4, assign25560_body0_e23891_d_n5, assign25560_body0_e23891_d_n6, assign25560_body0_e23891_d_n7, assign25560_body0_e23891_d_n8, assign25560_body0_e23891_d_n9, assign25560_body0_e23891_d_n10, assign25560_body0_e23891_d_n11, assign25560_body0_e23891_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign25560_body0_e23889: f64 = (locals.var_dnm).sqrt();
        (assign25560_body0_e23889, (locals.var_dnm_dn0 / (2.0 * assign25560_body0_e23889)), (locals.var_dnm_dn2 / (2.0 * assign25560_body0_e23889)), (locals.var_dnm_dn4 / (2.0 * assign25560_body0_e23889)), (locals.var_dnm_dn5 / (2.0 * assign25560_body0_e23889)), (locals.var_dnm_dn6 / (2.0 * assign25560_body0_e23889)), (locals.var_dnm_dn7 / (2.0 * assign25560_body0_e23889)), (locals.var_dnm_dn8 / (2.0 * assign25560_body0_e23889)), (locals.var_dnm_dn9 / (2.0 * assign25560_body0_e23889)), (locals.var_dnm_dn10 / (2.0 * assign25560_body0_e23889)), (locals.var_dnm_dn11 / (2.0 * assign25560_body0_e23889)), (locals.var_dnm_dn14 / (2.0 * assign25560_body0_e23889)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign25560_body0_e23891;
            locals.var_dnm_dn0 = assign25560_body0_e23891_d_n0;
            locals.var_dnm_dn2 = assign25560_body0_e23891_d_n2;
            locals.var_dnm_dn4 = assign25560_body0_e23891_d_n4;
            locals.var_dnm_dn5 = assign25560_body0_e23891_d_n5;
            locals.var_dnm_dn6 = assign25560_body0_e23891_d_n6;
            locals.var_dnm_dn7 = assign25560_body0_e23891_d_n7;
            locals.var_dnm_dn8 = assign25560_body0_e23891_d_n8;
            locals.var_dnm_dn9 = assign25560_body0_e23891_d_n9;
            locals.var_dnm_dn10 = assign25560_body0_e23891_d_n10;
            locals.var_dnm_dn11 = assign25560_body0_e23891_d_n11;
            locals.var_dnm_dn14 = assign25560_body0_e23891_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign25560_body1_e23905,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign25560_body1_e23903: f64 = (locals.var_m0 + 1.0);
        (assign25560_body1_e23903,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign25560_body1_e23905;
            locals.var_m0_rv = 0.0;
        }

        let (assign25570_e23929, assign25570_e23929_d_n0, assign25570_e23929_d_n2, assign25570_e23929_d_n4, assign25570_e23929_d_n5, assign25570_e23929_d_n6, assign25570_e23929_d_n7, assign25570_e23929_d_n8, assign25570_e23929_d_n9, assign25570_e23929_d_n10, assign25570_e23929_d_n11, assign25570_e23929_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) && (locals.var_guard620 == 0.0)) {
        let (assign25570_e23927, assign25570_e23927_d_n0, assign25570_e23927_d_n2, assign25570_e23927_d_n4, assign25570_e23927_d_n5, assign25570_e23927_d_n6, assign25570_e23927_d_n7, assign25570_e23927_d_n8, assign25570_e23927_d_n9, assign25570_e23927_d_n10, assign25570_e23927_d_n11, assign25570_e23927_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign25570_e23924: f64 = (2.0 * 2.0);
                let assign25570_e23925: f64 = (1.0 / assign25570_e23924);
                let assign25570_e23926: f64 = (locals.var_dnm).powf(assign25570_e23925);
                (assign25570_e23926, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn0)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn2)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn4)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn5)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn6)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn7)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn8)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn9)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn10)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn11)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign25570_e23925) as f64).is_finite() && ((assign25570_e23925) as f64).fract() == 0.0 { if assign25570_e23925 == 0.0 { 0.0 } else { (assign25570_e23925 * ((locals.var_dnm).powf(assign25570_e23925 - 1.0) * locals.var_dnm_dn14)) } } else { (assign25570_e23926 * (assign25570_e23925 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign25570_e23927, assign25570_e23927_d_n0, assign25570_e23927_d_n2, assign25570_e23927_d_n4, assign25570_e23927_d_n5, assign25570_e23927_d_n6, assign25570_e23927_d_n7, assign25570_e23927_d_n8, assign25570_e23927_d_n9, assign25570_e23927_d_n10, assign25570_e23927_d_n11, assign25570_e23927_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25570_e23929;
        locals.var_dnm_dn0 = assign25570_e23929_d_n0;
        locals.var_dnm_dn2 = assign25570_e23929_d_n2;
        locals.var_dnm_dn4 = assign25570_e23929_d_n4;
        locals.var_dnm_dn5 = assign25570_e23929_d_n5;
        locals.var_dnm_dn6 = assign25570_e23929_d_n6;
        locals.var_dnm_dn7 = assign25570_e23929_d_n7;
        locals.var_dnm_dn8 = assign25570_e23929_d_n8;
        locals.var_dnm_dn9 = assign25570_e23929_d_n9;
        locals.var_dnm_dn10 = assign25570_e23929_d_n10;
        locals.var_dnm_dn11 = assign25570_e23929_d_n11;
        locals.var_dnm_dn14 = assign25570_e23929_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign25580_e23941, assign25580_e23941_d_n0, assign25580_e23941_d_n2, assign25580_e23941_d_n4, assign25580_e23941_d_n5, assign25580_e23941_d_n6, assign25580_e23941_d_n7, assign25580_e23941_d_n8, assign25580_e23941_d_n9, assign25580_e23941_d_n10, assign25580_e23941_d_n11, assign25580_e23941_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25580_e23939: f64 = (1.0 / locals.var_dnm);
        (assign25580_e23939, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25580_e23941;
        locals.var_dnm_dn0 = assign25580_e23941_d_n0;
        locals.var_dnm_dn2 = assign25580_e23941_d_n2;
        locals.var_dnm_dn4 = assign25580_e23941_d_n4;
        locals.var_dnm_dn5 = assign25580_e23941_d_n5;
        locals.var_dnm_dn6 = assign25580_e23941_d_n6;
        locals.var_dnm_dn7 = assign25580_e23941_d_n7;
        locals.var_dnm_dn8 = assign25580_e23941_d_n8;
        locals.var_dnm_dn9 = assign25580_e23941_d_n9;
        locals.var_dnm_dn10 = assign25580_e23941_d_n10;
        locals.var_dnm_dn11 = assign25580_e23941_d_n11;
        locals.var_dnm_dn14 = assign25580_e23941_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign25590_e23955, assign25590_e23955_d_n0, assign25590_e23955_d_n2, assign25590_e23955_d_n4, assign25590_e23955_d_n5, assign25590_e23955_d_n6, assign25590_e23955_d_n7, assign25590_e23955_d_n8, assign25590_e23955_d_n9, assign25590_e23955_d_n10, assign25590_e23955_d_n11, assign25590_e23955_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25590_e23951: f64 = (locals.var_tmf1 * 0.02);
        let assign25590_e23953: f64 = (assign25590_e23951 * locals.var_dnm);
        (assign25590_e23953, (((locals.var_tmf1_dn0 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.02) * locals.var_dnm) + (assign25590_e23951 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign25590_e23955;
        locals.var_tmf0_dn0 = assign25590_e23955_d_n0;
        locals.var_tmf0_dn2 = assign25590_e23955_d_n2;
        locals.var_tmf0_dn4 = assign25590_e23955_d_n4;
        locals.var_tmf0_dn5 = assign25590_e23955_d_n5;
        locals.var_tmf0_dn6 = assign25590_e23955_d_n6;
        locals.var_tmf0_dn7 = assign25590_e23955_d_n7;
        locals.var_tmf0_dn8 = assign25590_e23955_d_n8;
        locals.var_tmf0_dn9 = assign25590_e23955_d_n9;
        locals.var_tmf0_dn10 = assign25590_e23955_d_n10;
        locals.var_tmf0_dn11 = assign25590_e23955_d_n11;
        locals.var_tmf0_dn14 = assign25590_e23955_d_n14;
        locals.var_tmf0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_76(
        locals: &mut StampLocals,
    ) {
        let (assign25600_e23971, assign25600_e23971_d_n0, assign25600_e23971_d_n2, assign25600_e23971_d_n4, assign25600_e23971_d_n5, assign25600_e23971_d_n6, assign25600_e23971_d_n7, assign25600_e23971_d_n8, assign25600_e23971_d_n9, assign25600_e23971_d_n10, assign25600_e23971_d_n11, assign25600_e23971_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25600_e23965: f64 = (0.02 * locals.var_xmp);
        let assign25600_e23967: f64 = (assign25600_e23965 * locals.var_dnm);
        let assign25600_e23969: f64 = (assign25600_e23967 / locals.var_arg);
        (assign25600_e23969, ((((((0.02 * locals.var_xmp_dn0) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn0)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn2) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn2)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn4) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn4)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn5) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn5)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn6) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn6)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn7) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn7)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn8) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn8)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn9) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn9)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn10) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn10)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn11) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn11)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn14) * locals.var_dnm) + (assign25600_e23965 * locals.var_dnm_dn14)) * locals.var_arg) - (assign25600_e23967 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25600_e23971;
        locals.var_t1_dn0 = assign25600_e23971_d_n0;
        locals.var_t1_dn2 = assign25600_e23971_d_n2;
        locals.var_t1_dn4 = assign25600_e23971_d_n4;
        locals.var_t1_dn5 = assign25600_e23971_d_n5;
        locals.var_t1_dn6 = assign25600_e23971_d_n6;
        locals.var_t1_dn7 = assign25600_e23971_d_n7;
        locals.var_t1_dn8 = assign25600_e23971_d_n8;
        locals.var_t1_dn9 = assign25600_e23971_d_n9;
        locals.var_t1_dn10 = assign25600_e23971_d_n10;
        locals.var_t1_dn11 = assign25600_e23971_d_n11;
        locals.var_t1_dn14 = assign25600_e23971_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25610_e23985, assign25610_e23985_d_n0, assign25610_e23985_d_n2, assign25610_e23985_d_n4, assign25610_e23985_d_n5, assign25610_e23985_d_n6, assign25610_e23985_d_n7, assign25610_e23985_d_n8, assign25610_e23985_d_n9, assign25610_e23985_d_n10, assign25610_e23985_d_n11, assign25610_e23985_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign25610_e23981: f64 = (locals.var_phi_s0_dep - 0.02);
        let assign25610_e23983: f64 = (assign25610_e23981 + locals.var_tmf0);
        (assign25610_e23983, (locals.var_phi_s0_dep_dn0 + locals.var_tmf0_dn0), (locals.var_phi_s0_dep_dn2 + locals.var_tmf0_dn2), (locals.var_phi_s0_dep_dn4 + locals.var_tmf0_dn4), (locals.var_phi_s0_dep_dn5 + locals.var_tmf0_dn5), (locals.var_phi_s0_dep_dn6 + locals.var_tmf0_dn6), (locals.var_phi_s0_dep_dn7 + locals.var_tmf0_dn7), (locals.var_phi_s0_dep_dn8 + locals.var_tmf0_dn8), (locals.var_phi_s0_dep_dn9 + locals.var_tmf0_dn9), (locals.var_phi_s0_dep_dn10 + locals.var_tmf0_dn10), (locals.var_phi_s0_dep_dn11 + locals.var_tmf0_dn11), (locals.var_phi_s0_dep_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign25610_e23985;
        locals.var_phi_b0_dep_dn0 = assign25610_e23985_d_n0;
        locals.var_phi_b0_dep_dn2 = assign25610_e23985_d_n2;
        locals.var_phi_b0_dep_dn4 = assign25610_e23985_d_n4;
        locals.var_phi_b0_dep_dn5 = assign25610_e23985_d_n5;
        locals.var_phi_b0_dep_dn6 = assign25610_e23985_d_n6;
        locals.var_phi_b0_dep_dn7 = assign25610_e23985_d_n7;
        locals.var_phi_b0_dep_dn8 = assign25610_e23985_d_n8;
        locals.var_phi_b0_dep_dn9 = assign25610_e23985_d_n9;
        locals.var_phi_b0_dep_dn10 = assign25610_e23985_d_n10;
        locals.var_phi_b0_dep_dn11 = assign25610_e23985_d_n11;
        locals.var_phi_b0_dep_dn14 = assign25610_e23985_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign25620_e23995, assign25620_e23995_d_n0, assign25620_e23995_d_n2, assign25620_e23995_d_n4, assign25620_e23995_d_n5, assign25620_e23995_d_n6, assign25620_e23995_d_n7, assign25620_e23995_d_n8, assign25620_e23995_d_n9, assign25620_e23995_d_n10, assign25620_e23995_d_n11, assign25620_e23995_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25620_e23995;
        locals.var_t1_dn0 = assign25620_e23995_d_n0;
        locals.var_t1_dn2 = assign25620_e23995_d_n2;
        locals.var_t1_dn4 = assign25620_e23995_d_n4;
        locals.var_t1_dn5 = assign25620_e23995_d_n5;
        locals.var_t1_dn6 = assign25620_e23995_d_n6;
        locals.var_t1_dn7 = assign25620_e23995_d_n7;
        locals.var_t1_dn8 = assign25620_e23995_d_n8;
        locals.var_t1_dn9 = assign25620_e23995_d_n9;
        locals.var_t1_dn10 = assign25620_e23995_d_n10;
        locals.var_t1_dn11 = assign25620_e23995_d_n11;
        locals.var_t1_dn14 = assign25620_e23995_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25630_e24006, assign25630_e24006_d_n0, assign25630_e24006_d_n2, assign25630_e24006_d_n4, assign25630_e24006_d_n5, assign25630_e24006_d_n6, assign25630_e24006_d_n7, assign25630_e24006_d_n8, assign25630_e24006_d_n9, assign25630_e24006_d_n10, assign25630_e24006_d_n11, assign25630_e24006_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn11, locals.var_phi_b0_dep_dn14,)
    }
};
        locals.var_phi_b0_dep = assign25630_e24006;
        locals.var_phi_b0_dep_dn0 = assign25630_e24006_d_n0;
        locals.var_phi_b0_dep_dn2 = assign25630_e24006_d_n2;
        locals.var_phi_b0_dep_dn4 = assign25630_e24006_d_n4;
        locals.var_phi_b0_dep_dn5 = assign25630_e24006_d_n5;
        locals.var_phi_b0_dep_dn6 = assign25630_e24006_d_n6;
        locals.var_phi_b0_dep_dn7 = assign25630_e24006_d_n7;
        locals.var_phi_b0_dep_dn8 = assign25630_e24006_d_n8;
        locals.var_phi_b0_dep_dn9 = assign25630_e24006_d_n9;
        locals.var_phi_b0_dep_dn10 = assign25630_e24006_d_n10;
        locals.var_phi_b0_dep_dn11 = assign25630_e24006_d_n11;
        locals.var_phi_b0_dep_dn14 = assign25630_e24006_d_n14;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign25640_e24017, assign25640_e24017_d_n0, assign25640_e24017_d_n2, assign25640_e24017_d_n4, assign25640_e24017_d_n5, assign25640_e24017_d_n6, assign25640_e24017_d_n7, assign25640_e24017_d_n8, assign25640_e24017_d_n9, assign25640_e24017_d_n10, assign25640_e24017_d_n11, assign25640_e24017_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25640_e24017;
        locals.var_t1_dn0 = assign25640_e24017_d_n0;
        locals.var_t1_dn2 = assign25640_e24017_d_n2;
        locals.var_t1_dn4 = assign25640_e24017_d_n4;
        locals.var_t1_dn5 = assign25640_e24017_d_n5;
        locals.var_t1_dn6 = assign25640_e24017_d_n6;
        locals.var_t1_dn7 = assign25640_e24017_d_n7;
        locals.var_t1_dn8 = assign25640_e24017_d_n8;
        locals.var_t1_dn9 = assign25640_e24017_d_n9;
        locals.var_t1_dn10 = assign25640_e24017_d_n10;
        locals.var_t1_dn11 = assign25640_e24017_d_n11;
        locals.var_t1_dn14 = assign25640_e24017_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25650_e24031, assign25650_e24031_d_n0, assign25650_e24031_d_n2, assign25650_e24031_d_n4, assign25650_e24031_d_n5, assign25650_e24031_d_n6, assign25650_e24031_d_n7, assign25650_e24031_d_n8, assign25650_e24031_d_n9, assign25650_e24031_d_n10, assign25650_e24031_d_n11, assign25650_e24031_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign25650_e24024: f64 = (locals.var_ndepmpnsub * locals.var_phi_b0_dep);
        let assign25650_e24026: f64 = (assign25650_e24024 + locals.var_vbscl__blk437);
        let assign25650_e24028: f64 = (assign25650_e24026 - locals.var_vbi_dep);
        let assign25650_e24029: f64 = (locals.var_ndepmpnsub_inv1 * assign25650_e24028);
        (assign25650_e24029, ((locals.var_ndepmpnsub_inv1_dn0 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn0 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn0)) + locals.var_vbscl__blk437_dn0) - locals.var_vbi_dep_dn0))), ((locals.var_ndepmpnsub_inv1_dn2 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn2 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn2)) + locals.var_vbscl__blk437_dn2) - locals.var_vbi_dep_dn2))), ((locals.var_ndepmpnsub_inv1_dn4 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn4 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn4)) + locals.var_vbscl__blk437_dn4) - locals.var_vbi_dep_dn4))), ((locals.var_ndepmpnsub_inv1_dn5 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn5 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn5)) + locals.var_vbscl__blk437_dn5) - locals.var_vbi_dep_dn5))), ((locals.var_ndepmpnsub_inv1_dn6 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn6 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn6)) + locals.var_vbscl__blk437_dn6) - locals.var_vbi_dep_dn6))), ((locals.var_ndepmpnsub_inv1_dn7 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn7 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn7)) + locals.var_vbscl__blk437_dn7) - locals.var_vbi_dep_dn7))), ((locals.var_ndepmpnsub_inv1_dn8 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn8 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn8)) + locals.var_vbscl__blk437_dn8) - locals.var_vbi_dep_dn8))), ((locals.var_ndepmpnsub_inv1_dn9 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn9 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn9)) + locals.var_vbscl__blk437_dn9) - locals.var_vbi_dep_dn9))), ((locals.var_ndepmpnsub_inv1_dn10 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn10 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn10)) + locals.var_vbscl__blk437_dn10) - locals.var_vbi_dep_dn10))), ((locals.var_ndepmpnsub_inv1_dn11 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn11 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn11)) + locals.var_vbscl__blk437_dn11) - locals.var_vbi_dep_dn11))), ((locals.var_ndepmpnsub_inv1_dn14 * assign25650_e24028) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn14 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn14)) + locals.var_vbscl__blk437_dn14) - locals.var_vbi_dep_dn14))),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn11, locals.var_phi_j0_dep_dn14,)
    }
};
        locals.var_phi_j0_dep = assign25650_e24031;
        locals.var_phi_j0_dep_dn0 = assign25650_e24031_d_n0;
        locals.var_phi_j0_dep_dn2 = assign25650_e24031_d_n2;
        locals.var_phi_j0_dep_dn4 = assign25650_e24031_d_n4;
        locals.var_phi_j0_dep_dn5 = assign25650_e24031_d_n5;
        locals.var_phi_j0_dep_dn6 = assign25650_e24031_d_n6;
        locals.var_phi_j0_dep_dn7 = assign25650_e24031_d_n7;
        locals.var_phi_j0_dep_dn8 = assign25650_e24031_d_n8;
        locals.var_phi_j0_dep_dn9 = assign25650_e24031_d_n9;
        locals.var_phi_j0_dep_dn10 = assign25650_e24031_d_n10;
        locals.var_phi_j0_dep_dn11 = assign25650_e24031_d_n11;
        locals.var_phi_j0_dep_dn14 = assign25650_e24031_d_n14;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign25660_e24041, assign25660_e24041_d_n0, assign25660_e24041_d_n2, assign25660_e24041_d_n4, assign25660_e24041_d_n5, assign25660_e24041_d_n6, assign25660_e24041_d_n7, assign25660_e24041_d_n8, assign25660_e24041_d_n9, assign25660_e24041_d_n10, assign25660_e24041_d_n11, assign25660_e24041_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign25660_e24038: f64 = (locals.var_phi_s0_dep - locals.var_phi_b0_dep);
        let assign25660_e24039: f64 = (locals.var_beta * assign25660_e24038);
        (assign25660_e24039, ((locals.var_beta_dn0 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn0 - locals.var_phi_b0_dep_dn0))), ((locals.var_beta_dn2 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn2 - locals.var_phi_b0_dep_dn2))), ((locals.var_beta_dn4 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn4 - locals.var_phi_b0_dep_dn4))), ((locals.var_beta_dn5 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn5 - locals.var_phi_b0_dep_dn5))), ((locals.var_beta_dn6 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn6 - locals.var_phi_b0_dep_dn6))), ((locals.var_beta_dn7 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn7 - locals.var_phi_b0_dep_dn7))), ((locals.var_beta_dn8 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn8 - locals.var_phi_b0_dep_dn8))), ((locals.var_beta_dn9 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn9 - locals.var_phi_b0_dep_dn9))), ((locals.var_beta_dn10 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn10 - locals.var_phi_b0_dep_dn10))), ((locals.var_beta_dn11 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn11 - locals.var_phi_b0_dep_dn11))), ((locals.var_beta_dn14 * assign25660_e24038) + (locals.var_beta * (locals.var_phi_s0_dep_dn14 - locals.var_phi_b0_dep_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25660_e24041;
        locals.var_t1_dn0 = assign25660_e24041_d_n0;
        locals.var_t1_dn2 = assign25660_e24041_d_n2;
        locals.var_t1_dn4 = assign25660_e24041_d_n4;
        locals.var_t1_dn5 = assign25660_e24041_d_n5;
        locals.var_t1_dn6 = assign25660_e24041_d_n6;
        locals.var_t1_dn7 = assign25660_e24041_d_n7;
        locals.var_t1_dn8 = assign25660_e24041_d_n8;
        locals.var_t1_dn9 = assign25660_e24041_d_n9;
        locals.var_t1_dn10 = assign25660_e24041_d_n10;
        locals.var_t1_dn11 = assign25660_e24041_d_n11;
        locals.var_t1_dn14 = assign25660_e24041_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25670_e24048, assign25670_e24048_d_n0, assign25670_e24048_d_n2, assign25670_e24048_d_n4, assign25670_e24048_d_n5, assign25670_e24048_d_n6, assign25670_e24048_d_n7, assign25670_e24048_d_n8, assign25670_e24048_d_n9, assign25670_e24048_d_n10, assign25670_e24048_d_n11, assign25670_e24048_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign25670_e24046: f64 = (locals.var_t1).exp();
        (assign25670_e24046, (assign25670_e24046 * locals.var_t1_dn0), (assign25670_e24046 * locals.var_t1_dn2), (assign25670_e24046 * locals.var_t1_dn4), (assign25670_e24046 * locals.var_t1_dn5), (assign25670_e24046 * locals.var_t1_dn6), (assign25670_e24046 * locals.var_t1_dn7), (assign25670_e24046 * locals.var_t1_dn8), (assign25670_e24046 * locals.var_t1_dn9), (assign25670_e24046 * locals.var_t1_dn10), (assign25670_e24046 * locals.var_t1_dn11), (assign25670_e24046 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign25670_e24048;
        locals.var_t2_dn0 = assign25670_e24048_d_n0;
        locals.var_t2_dn2 = assign25670_e24048_d_n2;
        locals.var_t2_dn4 = assign25670_e24048_d_n4;
        locals.var_t2_dn5 = assign25670_e24048_d_n5;
        locals.var_t2_dn6 = assign25670_e24048_d_n6;
        locals.var_t2_dn7 = assign25670_e24048_d_n7;
        locals.var_t2_dn8 = assign25670_e24048_d_n8;
        locals.var_t2_dn9 = assign25670_e24048_d_n9;
        locals.var_t2_dn10 = assign25670_e24048_d_n10;
        locals.var_t2_dn11 = assign25670_e24048_d_n11;
        locals.var_t2_dn14 = assign25670_e24048_d_n14;
        locals.var_t2_rv = 0.0;

        let assign25680_e24051: f64 = if locals.var_phi_s0_dep >= locals.var_phi_b0_dep { 1.0 } else { 0.0 };
        locals.var_guard625 = assign25680_e24051;
        locals.var_guard625_rv = 0.0;

        let (assign25690_e24069, assign25690_e24069_d_n0, assign25690_e24069_d_n2, assign25690_e24069_d_n4, assign25690_e24069_d_n5, assign25690_e24069_d_n6, assign25690_e24069_d_n7, assign25690_e24069_d_n8, assign25690_e24069_d_n9, assign25690_e24069_d_n10, assign25690_e24069_d_n11, assign25690_e24069_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) {
        let assign25690_e24058: f64 = (-locals.var_cnst0);
        let assign25690_e24061: f64 = (locals.var_t2 - 1.0);
        let assign25690_e24063: f64 = (assign25690_e24061 - locals.var_t1);
        let assign25690_e24065: f64 = (assign25690_e24063 + 1e-15);
        let assign25690_e24066: f64 = (assign25690_e24065).sqrt();
        let assign25690_e24067: f64 = (assign25690_e24058 * assign25690_e24066);
        (assign25690_e24067, (((-locals.var_cnst0_dn0) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign25690_e24066)))), (((-locals.var_cnst0_dn2) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign25690_e24066)))), (((-locals.var_cnst0_dn4) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign25690_e24066)))), (((-locals.var_cnst0_dn5) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign25690_e24066)))), (((-locals.var_cnst0_dn6) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign25690_e24066)))), (((-locals.var_cnst0_dn7) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign25690_e24066)))), (((-locals.var_cnst0_dn8) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign25690_e24066)))), (((-locals.var_cnst0_dn9) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign25690_e24066)))), (((-locals.var_cnst0_dn10) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign25690_e24066)))), (((-locals.var_cnst0_dn11) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign25690_e24066)))), (((-locals.var_cnst0_dn14) * assign25690_e24066) + (assign25690_e24058 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign25690_e24066)))),)
    } else {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn11, locals.var_q_s0_dn14,)
    }
};
        locals.var_q_s0 = assign25690_e24069;
        locals.var_q_s0_dn0 = assign25690_e24069_d_n0;
        locals.var_q_s0_dn2 = assign25690_e24069_d_n2;
        locals.var_q_s0_dn4 = assign25690_e24069_d_n4;
        locals.var_q_s0_dn5 = assign25690_e24069_d_n5;
        locals.var_q_s0_dn6 = assign25690_e24069_d_n6;
        locals.var_q_s0_dn7 = assign25690_e24069_d_n7;
        locals.var_q_s0_dn8 = assign25690_e24069_d_n8;
        locals.var_q_s0_dn9 = assign25690_e24069_d_n9;
        locals.var_q_s0_dn10 = assign25690_e24069_d_n10;
        locals.var_q_s0_dn11 = assign25690_e24069_d_n11;
        locals.var_q_s0_dn14 = assign25690_e24069_d_n14;
        locals.var_q_s0_rv = 0.0;

        let (assign25700_e24077, assign25700_e24077_d_n0, assign25700_e24077_d_n2, assign25700_e24077_d_n4, assign25700_e24077_d_n5, assign25700_e24077_d_n6, assign25700_e24077_d_n7, assign25700_e24077_d_n8, assign25700_e24077_d_n9, assign25700_e24077_d_n10, assign25700_e24077_d_n11, assign25700_e24077_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn11, locals.var_q_s0_dn14,)
    } else {
        (locals.var_q_n0__blk540, locals.var_q_n0__blk540_dn0, locals.var_q_n0__blk540_dn2, locals.var_q_n0__blk540_dn4, locals.var_q_n0__blk540_dn5, locals.var_q_n0__blk540_dn6, locals.var_q_n0__blk540_dn7, locals.var_q_n0__blk540_dn8, locals.var_q_n0__blk540_dn9, locals.var_q_n0__blk540_dn10, locals.var_q_n0__blk540_dn11, locals.var_q_n0__blk540_dn14,)
    }
};
        locals.var_q_n0__blk540 = assign25700_e24077;
        locals.var_q_n0__blk540_dn0 = assign25700_e24077_d_n0;
        locals.var_q_n0__blk540_dn2 = assign25700_e24077_d_n2;
        locals.var_q_n0__blk540_dn4 = assign25700_e24077_d_n4;
        locals.var_q_n0__blk540_dn5 = assign25700_e24077_d_n5;
        locals.var_q_n0__blk540_dn6 = assign25700_e24077_d_n6;
        locals.var_q_n0__blk540_dn7 = assign25700_e24077_d_n7;
        locals.var_q_n0__blk540_dn8 = assign25700_e24077_d_n8;
        locals.var_q_n0__blk540_dn9 = assign25700_e24077_d_n9;
        locals.var_q_n0__blk540_dn10 = assign25700_e24077_d_n10;
        locals.var_q_n0__blk540_dn11 = assign25700_e24077_d_n11;
        locals.var_q_n0__blk540_dn14 = assign25700_e24077_d_n14;
        locals.var_q_n0__blk540_rv = 0.0;

        let (assign25710_e24085, assign25710_e24085_d_n0, assign25710_e24085_d_n2, assign25710_e24085_d_n4, assign25710_e24085_d_n5, assign25710_e24085_d_n6, assign25710_e24085_d_n7, assign25710_e24085_d_n8, assign25710_e24085_d_n9, assign25710_e24085_d_n10, assign25710_e24085_d_n11, assign25710_e24085_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn11, locals.var_q_s0_dep_dn14,)
    }
};
        locals.var_q_s0_dep = assign25710_e24085;
        locals.var_q_s0_dep_dn0 = assign25710_e24085_d_n0;
        locals.var_q_s0_dep_dn2 = assign25710_e24085_d_n2;
        locals.var_q_s0_dep_dn4 = assign25710_e24085_d_n4;
        locals.var_q_s0_dep_dn5 = assign25710_e24085_d_n5;
        locals.var_q_s0_dep_dn6 = assign25710_e24085_d_n6;
        locals.var_q_s0_dep_dn7 = assign25710_e24085_d_n7;
        locals.var_q_s0_dep_dn8 = assign25710_e24085_d_n8;
        locals.var_q_s0_dep_dn9 = assign25710_e24085_d_n9;
        locals.var_q_s0_dep_dn10 = assign25710_e24085_d_n10;
        locals.var_q_s0_dep_dn11 = assign25710_e24085_d_n11;
        locals.var_q_s0_dep_dn14 = assign25710_e24085_d_n14;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign25720_e24093, assign25720_e24093_d_n0, assign25720_e24093_d_n2, assign25720_e24093_d_n4, assign25720_e24093_d_n5, assign25720_e24093_d_n6, assign25720_e24093_d_n7, assign25720_e24093_d_n8, assign25720_e24093_d_n9, assign25720_e24093_d_n10, assign25720_e24093_d_n11, assign25720_e24093_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn11, locals.var_q_sub0_dn14,)
    }
};
        locals.var_q_sub0 = assign25720_e24093;
        locals.var_q_sub0_dn0 = assign25720_e24093_d_n0;
        locals.var_q_sub0_dn2 = assign25720_e24093_d_n2;
        locals.var_q_sub0_dn4 = assign25720_e24093_d_n4;
        locals.var_q_sub0_dn5 = assign25720_e24093_d_n5;
        locals.var_q_sub0_dn6 = assign25720_e24093_d_n6;
        locals.var_q_sub0_dn7 = assign25720_e24093_d_n7;
        locals.var_q_sub0_dn8 = assign25720_e24093_d_n8;
        locals.var_q_sub0_dn9 = assign25720_e24093_d_n9;
        locals.var_q_sub0_dn10 = assign25720_e24093_d_n10;
        locals.var_q_sub0_dn11 = assign25720_e24093_d_n11;
        locals.var_q_sub0_dn14 = assign25720_e24093_d_n14;
        locals.var_q_sub0_rv = 0.0;

        let (assign25730_e24106, assign25730_e24106_d_n0, assign25730_e24106_d_n2, assign25730_e24106_d_n4, assign25730_e24106_d_n5, assign25730_e24106_d_n6, assign25730_e24106_d_n7, assign25730_e24106_d_n8, assign25730_e24106_d_n9, assign25730_e24106_d_n10, assign25730_e24106_d_n11, assign25730_e24106_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) {
        let assign25730_e24102: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign25730_e24103: f64 = (locals.var_c_2esipq_ndepm * assign25730_e24102);
        let assign25730_e24104: f64 = (assign25730_e24103).sqrt();
        (assign25730_e24104, (((locals.var_c_2esipq_ndepm_dn0 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign25730_e24104)), (((locals.var_c_2esipq_ndepm_dn2 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign25730_e24104)), (((locals.var_c_2esipq_ndepm_dn4 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign25730_e24104)), (((locals.var_c_2esipq_ndepm_dn5 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign25730_e24104)), (((locals.var_c_2esipq_ndepm_dn6 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign25730_e24104)), (((locals.var_c_2esipq_ndepm_dn7 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign25730_e24104)), (((locals.var_c_2esipq_ndepm_dn8 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign25730_e24104)), (((locals.var_c_2esipq_ndepm_dn9 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign25730_e24104)), (((locals.var_c_2esipq_ndepm_dn10 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign25730_e24104)), (((locals.var_c_2esipq_ndepm_dn11 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11))) / (2.0 * assign25730_e24104)), (((locals.var_c_2esipq_ndepm_dn14 * assign25730_e24102) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14))) / (2.0 * assign25730_e24104)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign25730_e24106;
        locals.var_w_b0_dn0 = assign25730_e24106_d_n0;
        locals.var_w_b0_dn2 = assign25730_e24106_d_n2;
        locals.var_w_b0_dn4 = assign25730_e24106_d_n4;
        locals.var_w_b0_dn5 = assign25730_e24106_d_n5;
        locals.var_w_b0_dn6 = assign25730_e24106_d_n6;
        locals.var_w_b0_dn7 = assign25730_e24106_d_n7;
        locals.var_w_b0_dn8 = assign25730_e24106_d_n8;
        locals.var_w_b0_dn9 = assign25730_e24106_d_n9;
        locals.var_w_b0_dn10 = assign25730_e24106_d_n10;
        locals.var_w_b0_dn11 = assign25730_e24106_d_n11;
        locals.var_w_b0_dn14 = assign25730_e24106_d_n14;
        locals.var_w_b0_rv = 0.0;

        let assign25740_e24110: f64 = (locals.var_uc_depthn - 1e-8);
        let assign25740_e24115: f64 = if ((locals.var_w_b0 > assign25740_e24110) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard626 = assign25740_e24115;
        locals.var_guard626_rv = 0.0;

        let (assign25750_e24129, assign25750_e24129_d_n0, assign25750_e24129_d_n2, assign25750_e24129_d_n4, assign25750_e24129_d_n5, assign25750_e24129_d_n6, assign25750_e24129_d_n7, assign25750_e24129_d_n8, assign25750_e24129_d_n9, assign25750_e24129_d_n10, assign25750_e24129_d_n11, assign25750_e24129_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign25750_e24125: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign25750_e24127: f64 = (assign25750_e24125 + 1e-8);
        (assign25750_e24127, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_b0_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign25750_e24129;
        locals.var_tmf1_dn0 = assign25750_e24129_d_n0;
        locals.var_tmf1_dn2 = assign25750_e24129_d_n2;
        locals.var_tmf1_dn4 = assign25750_e24129_d_n4;
        locals.var_tmf1_dn5 = assign25750_e24129_d_n5;
        locals.var_tmf1_dn6 = assign25750_e24129_d_n6;
        locals.var_tmf1_dn7 = assign25750_e24129_d_n7;
        locals.var_tmf1_dn8 = assign25750_e24129_d_n8;
        locals.var_tmf1_dn9 = assign25750_e24129_d_n9;
        locals.var_tmf1_dn10 = assign25750_e24129_d_n10;
        locals.var_tmf1_dn11 = assign25750_e24129_d_n11;
        locals.var_tmf1_dn14 = assign25750_e24129_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign25760_e24141, assign25760_e24141_d_n0, assign25760_e24141_d_n2, assign25760_e24141_d_n4, assign25760_e24141_d_n5, assign25760_e24141_d_n6, assign25760_e24141_d_n7, assign25760_e24141_d_n8, assign25760_e24141_d_n9, assign25760_e24141_d_n10, assign25760_e24141_d_n11, assign25760_e24141_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign25760_e24139: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign25760_e24139, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign25760_e24141;
        locals.var_x2_dn0 = assign25760_e24141_d_n0;
        locals.var_x2_dn2 = assign25760_e24141_d_n2;
        locals.var_x2_dn4 = assign25760_e24141_d_n4;
        locals.var_x2_dn5 = assign25760_e24141_d_n5;
        locals.var_x2_dn6 = assign25760_e24141_d_n6;
        locals.var_x2_dn7 = assign25760_e24141_d_n7;
        locals.var_x2_dn8 = assign25760_e24141_d_n8;
        locals.var_x2_dn9 = assign25760_e24141_d_n9;
        locals.var_x2_dn10 = assign25760_e24141_d_n10;
        locals.var_x2_dn11 = assign25760_e24141_d_n11;
        locals.var_x2_dn14 = assign25760_e24141_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign25770_e24153, assign25770_e24153_d_n0, assign25770_e24153_d_n2, assign25770_e24153_d_n4, assign25770_e24153_d_n5, assign25770_e24153_d_n6, assign25770_e24153_d_n7, assign25770_e24153_d_n8, assign25770_e24153_d_n9, assign25770_e24153_d_n10, assign25770_e24153_d_n11, assign25770_e24153_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign25770_e24151: f64 = (1e-8 * 1e-8);
        (assign25770_e24151, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign25770_e24153;
        locals.var_xmax2_dn0 = assign25770_e24153_d_n0;
        locals.var_xmax2_dn2 = assign25770_e24153_d_n2;
        locals.var_xmax2_dn4 = assign25770_e24153_d_n4;
        locals.var_xmax2_dn5 = assign25770_e24153_d_n5;
        locals.var_xmax2_dn6 = assign25770_e24153_d_n6;
        locals.var_xmax2_dn7 = assign25770_e24153_d_n7;
        locals.var_xmax2_dn8 = assign25770_e24153_d_n8;
        locals.var_xmax2_dn9 = assign25770_e24153_d_n9;
        locals.var_xmax2_dn10 = assign25770_e24153_d_n10;
        locals.var_xmax2_dn11 = assign25770_e24153_d_n11;
        locals.var_xmax2_dn14 = assign25770_e24153_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign25780_e24163, assign25780_e24163_d_n0, assign25780_e24163_d_n2, assign25780_e24163_d_n4, assign25780_e24163_d_n5, assign25780_e24163_d_n6, assign25780_e24163_d_n7, assign25780_e24163_d_n8, assign25780_e24163_d_n9, assign25780_e24163_d_n10, assign25780_e24163_d_n11, assign25780_e24163_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25780_e24163;
        locals.var_xp_dn0 = assign25780_e24163_d_n0;
        locals.var_xp_dn2 = assign25780_e24163_d_n2;
        locals.var_xp_dn4 = assign25780_e24163_d_n4;
        locals.var_xp_dn5 = assign25780_e24163_d_n5;
        locals.var_xp_dn6 = assign25780_e24163_d_n6;
        locals.var_xp_dn7 = assign25780_e24163_d_n7;
        locals.var_xp_dn8 = assign25780_e24163_d_n8;
        locals.var_xp_dn9 = assign25780_e24163_d_n9;
        locals.var_xp_dn10 = assign25780_e24163_d_n10;
        locals.var_xp_dn11 = assign25780_e24163_d_n11;
        locals.var_xp_dn14 = assign25780_e24163_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign25790_e24173, assign25790_e24173_d_n0, assign25790_e24173_d_n2, assign25790_e24173_d_n4, assign25790_e24173_d_n5, assign25790_e24173_d_n6, assign25790_e24173_d_n7, assign25790_e24173_d_n8, assign25790_e24173_d_n9, assign25790_e24173_d_n10, assign25790_e24173_d_n11, assign25790_e24173_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25790_e24173;
        locals.var_xmp_dn0 = assign25790_e24173_d_n0;
        locals.var_xmp_dn2 = assign25790_e24173_d_n2;
        locals.var_xmp_dn4 = assign25790_e24173_d_n4;
        locals.var_xmp_dn5 = assign25790_e24173_d_n5;
        locals.var_xmp_dn6 = assign25790_e24173_d_n6;
        locals.var_xmp_dn7 = assign25790_e24173_d_n7;
        locals.var_xmp_dn8 = assign25790_e24173_d_n8;
        locals.var_xmp_dn9 = assign25790_e24173_d_n9;
        locals.var_xmp_dn10 = assign25790_e24173_d_n10;
        locals.var_xmp_dn11 = assign25790_e24173_d_n11;
        locals.var_xmp_dn14 = assign25790_e24173_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25800_e24183,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign25800_e24183;
        locals.var_m0_rv = 0.0;

        let (assign25810_e24193,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25810_e24193;
        locals.var_mm_rv = 0.0;

        let (assign25820_e24203, assign25820_e24203_d_n0, assign25820_e24203_d_n2, assign25820_e24203_d_n4, assign25820_e24203_d_n5, assign25820_e24203_d_n6, assign25820_e24203_d_n7, assign25820_e24203_d_n8, assign25820_e24203_d_n9, assign25820_e24203_d_n10, assign25820_e24203_d_n11, assign25820_e24203_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign25820_e24203;
        locals.var_arg_dn0 = assign25820_e24203_d_n0;
        locals.var_arg_dn2 = assign25820_e24203_d_n2;
        locals.var_arg_dn4 = assign25820_e24203_d_n4;
        locals.var_arg_dn5 = assign25820_e24203_d_n5;
        locals.var_arg_dn6 = assign25820_e24203_d_n6;
        locals.var_arg_dn7 = assign25820_e24203_d_n7;
        locals.var_arg_dn8 = assign25820_e24203_d_n8;
        locals.var_arg_dn9 = assign25820_e24203_d_n9;
        locals.var_arg_dn10 = assign25820_e24203_d_n10;
        locals.var_arg_dn11 = assign25820_e24203_d_n11;
        locals.var_arg_dn14 = assign25820_e24203_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign25830_e24213, assign25830_e24213_d_n0, assign25830_e24213_d_n2, assign25830_e24213_d_n4, assign25830_e24213_d_n5, assign25830_e24213_d_n6, assign25830_e24213_d_n7, assign25830_e24213_d_n8, assign25830_e24213_d_n9, assign25830_e24213_d_n10, assign25830_e24213_d_n11, assign25830_e24213_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25830_e24213;
        locals.var_dnm_dn0 = assign25830_e24213_d_n0;
        locals.var_dnm_dn2 = assign25830_e24213_d_n2;
        locals.var_dnm_dn4 = assign25830_e24213_d_n4;
        locals.var_dnm_dn5 = assign25830_e24213_d_n5;
        locals.var_dnm_dn6 = assign25830_e24213_d_n6;
        locals.var_dnm_dn7 = assign25830_e24213_d_n7;
        locals.var_dnm_dn8 = assign25830_e24213_d_n8;
        locals.var_dnm_dn9 = assign25830_e24213_d_n9;
        locals.var_dnm_dn10 = assign25830_e24213_d_n10;
        locals.var_dnm_dn11 = assign25830_e24213_d_n11;
        locals.var_dnm_dn14 = assign25830_e24213_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign25840_e24225, assign25840_e24225_d_n0, assign25840_e24225_d_n2, assign25840_e24225_d_n4, assign25840_e24225_d_n5, assign25840_e24225_d_n6, assign25840_e24225_d_n7, assign25840_e24225_d_n8, assign25840_e24225_d_n9, assign25840_e24225_d_n10, assign25840_e24225_d_n11, assign25840_e24225_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign25840_e24223: f64 = (locals.var_xp * locals.var_x2);
        (assign25840_e24223, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25840_e24225;
        locals.var_xp_dn0 = assign25840_e24225_d_n0;
        locals.var_xp_dn2 = assign25840_e24225_d_n2;
        locals.var_xp_dn4 = assign25840_e24225_d_n4;
        locals.var_xp_dn5 = assign25840_e24225_d_n5;
        locals.var_xp_dn6 = assign25840_e24225_d_n6;
        locals.var_xp_dn7 = assign25840_e24225_d_n7;
        locals.var_xp_dn8 = assign25840_e24225_d_n8;
        locals.var_xp_dn9 = assign25840_e24225_d_n9;
        locals.var_xp_dn10 = assign25840_e24225_d_n10;
        locals.var_xp_dn11 = assign25840_e24225_d_n11;
        locals.var_xp_dn14 = assign25840_e24225_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_77(
        locals: &mut StampLocals,
    ) {
        let (assign25850_e24237, assign25850_e24237_d_n0, assign25850_e24237_d_n2, assign25850_e24237_d_n4, assign25850_e24237_d_n5, assign25850_e24237_d_n6, assign25850_e24237_d_n7, assign25850_e24237_d_n8, assign25850_e24237_d_n9, assign25850_e24237_d_n10, assign25850_e24237_d_n11, assign25850_e24237_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign25850_e24235: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign25850_e24235, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25850_e24237;
        locals.var_xmp_dn0 = assign25850_e24237_d_n0;
        locals.var_xmp_dn2 = assign25850_e24237_d_n2;
        locals.var_xmp_dn4 = assign25850_e24237_d_n4;
        locals.var_xmp_dn5 = assign25850_e24237_d_n5;
        locals.var_xmp_dn6 = assign25850_e24237_d_n6;
        locals.var_xmp_dn7 = assign25850_e24237_d_n7;
        locals.var_xmp_dn8 = assign25850_e24237_d_n8;
        locals.var_xmp_dn9 = assign25850_e24237_d_n9;
        locals.var_xmp_dn10 = assign25850_e24237_d_n10;
        locals.var_xmp_dn11 = assign25850_e24237_d_n11;
        locals.var_xmp_dn14 = assign25850_e24237_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25860_e24249, assign25860_e24249_d_n0, assign25860_e24249_d_n2, assign25860_e24249_d_n4, assign25860_e24249_d_n5, assign25860_e24249_d_n6, assign25860_e24249_d_n7, assign25860_e24249_d_n8, assign25860_e24249_d_n9, assign25860_e24249_d_n10, assign25860_e24249_d_n11, assign25860_e24249_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign25860_e24247: f64 = (locals.var_xp * locals.var_x2);
        (assign25860_e24247, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign25860_e24249;
        locals.var_xp_dn0 = assign25860_e24249_d_n0;
        locals.var_xp_dn2 = assign25860_e24249_d_n2;
        locals.var_xp_dn4 = assign25860_e24249_d_n4;
        locals.var_xp_dn5 = assign25860_e24249_d_n5;
        locals.var_xp_dn6 = assign25860_e24249_d_n6;
        locals.var_xp_dn7 = assign25860_e24249_d_n7;
        locals.var_xp_dn8 = assign25860_e24249_d_n8;
        locals.var_xp_dn9 = assign25860_e24249_d_n9;
        locals.var_xp_dn10 = assign25860_e24249_d_n10;
        locals.var_xp_dn11 = assign25860_e24249_d_n11;
        locals.var_xp_dn14 = assign25860_e24249_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign25870_e24261, assign25870_e24261_d_n0, assign25870_e24261_d_n2, assign25870_e24261_d_n4, assign25870_e24261_d_n5, assign25870_e24261_d_n6, assign25870_e24261_d_n7, assign25870_e24261_d_n8, assign25870_e24261_d_n9, assign25870_e24261_d_n10, assign25870_e24261_d_n11, assign25870_e24261_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign25870_e24259: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign25870_e24259, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign25870_e24261;
        locals.var_xmp_dn0 = assign25870_e24261_d_n0;
        locals.var_xmp_dn2 = assign25870_e24261_d_n2;
        locals.var_xmp_dn4 = assign25870_e24261_d_n4;
        locals.var_xmp_dn5 = assign25870_e24261_d_n5;
        locals.var_xmp_dn6 = assign25870_e24261_d_n6;
        locals.var_xmp_dn7 = assign25870_e24261_d_n7;
        locals.var_xmp_dn8 = assign25870_e24261_d_n8;
        locals.var_xmp_dn9 = assign25870_e24261_d_n9;
        locals.var_xmp_dn10 = assign25870_e24261_d_n10;
        locals.var_xmp_dn11 = assign25870_e24261_d_n11;
        locals.var_xmp_dn14 = assign25870_e24261_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign25880_e24273, assign25880_e24273_d_n0, assign25880_e24273_d_n2, assign25880_e24273_d_n4, assign25880_e24273_d_n5, assign25880_e24273_d_n6, assign25880_e24273_d_n7, assign25880_e24273_d_n8, assign25880_e24273_d_n9, assign25880_e24273_d_n10, assign25880_e24273_d_n11, assign25880_e24273_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign25880_e24271: f64 = (locals.var_xp + locals.var_xmp);
        (assign25880_e24271, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign25880_e24273;
        locals.var_arg_dn0 = assign25880_e24273_d_n0;
        locals.var_arg_dn2 = assign25880_e24273_d_n2;
        locals.var_arg_dn4 = assign25880_e24273_d_n4;
        locals.var_arg_dn5 = assign25880_e24273_d_n5;
        locals.var_arg_dn6 = assign25880_e24273_d_n6;
        locals.var_arg_dn7 = assign25880_e24273_d_n7;
        locals.var_arg_dn8 = assign25880_e24273_d_n8;
        locals.var_arg_dn9 = assign25880_e24273_d_n9;
        locals.var_arg_dn10 = assign25880_e24273_d_n10;
        locals.var_arg_dn11 = assign25880_e24273_d_n11;
        locals.var_arg_dn14 = assign25880_e24273_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign25890_e24283, assign25890_e24283_d_n0, assign25890_e24283_d_n2, assign25890_e24283_d_n4, assign25890_e24283_d_n5, assign25890_e24283_d_n6, assign25890_e24283_d_n7, assign25890_e24283_d_n8, assign25890_e24283_d_n9, assign25890_e24283_d_n10, assign25890_e24283_d_n11, assign25890_e24283_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign25890_e24283;
        locals.var_dnm_dn0 = assign25890_e24283_d_n0;
        locals.var_dnm_dn2 = assign25890_e24283_d_n2;
        locals.var_dnm_dn4 = assign25890_e24283_d_n4;
        locals.var_dnm_dn5 = assign25890_e24283_d_n5;
        locals.var_dnm_dn6 = assign25890_e24283_d_n6;
        locals.var_dnm_dn7 = assign25890_e24283_d_n7;
        locals.var_dnm_dn8 = assign25890_e24283_d_n8;
        locals.var_dnm_dn9 = assign25890_e24283_d_n9;
        locals.var_dnm_dn10 = assign25890_e24283_d_n10;
        locals.var_dnm_dn11 = assign25890_e24283_d_n11;
        locals.var_dnm_dn14 = assign25890_e24283_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign25900_e24298: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard627 = assign25900_e24298;
        locals.var_guard627_rv = 0.0;

        let assign25910_e24301: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard628 = assign25910_e24301;
        locals.var_guard628_rv = 0.0;

        let (assign25920_e24315,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25920_e24315;
        locals.var_mm_rv = 0.0;

        let assign25930_e24318: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard629 = assign25930_e24318;
        locals.var_guard629_rv = 0.0;

        let (assign25940_e24335,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 == 0.0)) && (locals.var_guard629 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25940_e24335;
        locals.var_mm_rv = 0.0;

        let assign25950_e24338: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign25950_e24338;
        locals.var_guard630_rv = 0.0;

        let (assign25960_e24358,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 == 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25960_e24358;
        locals.var_mm_rv = 0.0;

        let assign25970_e24361: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard631 = assign25970_e24361;
        locals.var_guard631_rv = 0.0;

        let (assign25980_e24384,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 == 0.0)) && (locals.var_guard629 == 0.0)) && (locals.var_guard630 == 0.0)) && (locals.var_guard631 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign25980_e24384;
        locals.var_mm_rv = 0.0;

        let (assign25990_e24396,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign25990_e24396;
        locals.var_m0_rv = 0.0;

        let mut assign26000_loop_guard: usize = 0;
        while {
            let assign26000_cond_e24409: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign26000_cond_e24409 != 0.0
        } {
            assign26000_loop_guard += 1;
            assert!(assign26000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26000_body0_e24422, assign26000_body0_e24422_d_n0, assign26000_body0_e24422_d_n2, assign26000_body0_e24422_d_n4, assign26000_body0_e24422_d_n5, assign26000_body0_e24422_d_n6, assign26000_body0_e24422_d_n7, assign26000_body0_e24422_d_n8, assign26000_body0_e24422_d_n9, assign26000_body0_e24422_d_n10, assign26000_body0_e24422_d_n11, assign26000_body0_e24422_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign26000_body0_e24420: f64 = (locals.var_dnm).sqrt();
        (assign26000_body0_e24420, (locals.var_dnm_dn0 / (2.0 * assign26000_body0_e24420)), (locals.var_dnm_dn2 / (2.0 * assign26000_body0_e24420)), (locals.var_dnm_dn4 / (2.0 * assign26000_body0_e24420)), (locals.var_dnm_dn5 / (2.0 * assign26000_body0_e24420)), (locals.var_dnm_dn6 / (2.0 * assign26000_body0_e24420)), (locals.var_dnm_dn7 / (2.0 * assign26000_body0_e24420)), (locals.var_dnm_dn8 / (2.0 * assign26000_body0_e24420)), (locals.var_dnm_dn9 / (2.0 * assign26000_body0_e24420)), (locals.var_dnm_dn10 / (2.0 * assign26000_body0_e24420)), (locals.var_dnm_dn11 / (2.0 * assign26000_body0_e24420)), (locals.var_dnm_dn14 / (2.0 * assign26000_body0_e24420)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign26000_body0_e24422;
            locals.var_dnm_dn0 = assign26000_body0_e24422_d_n0;
            locals.var_dnm_dn2 = assign26000_body0_e24422_d_n2;
            locals.var_dnm_dn4 = assign26000_body0_e24422_d_n4;
            locals.var_dnm_dn5 = assign26000_body0_e24422_d_n5;
            locals.var_dnm_dn6 = assign26000_body0_e24422_d_n6;
            locals.var_dnm_dn7 = assign26000_body0_e24422_d_n7;
            locals.var_dnm_dn8 = assign26000_body0_e24422_d_n8;
            locals.var_dnm_dn9 = assign26000_body0_e24422_d_n9;
            locals.var_dnm_dn10 = assign26000_body0_e24422_d_n10;
            locals.var_dnm_dn11 = assign26000_body0_e24422_d_n11;
            locals.var_dnm_dn14 = assign26000_body0_e24422_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign26000_body1_e24436,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign26000_body1_e24434: f64 = (locals.var_m0 + 1.0);
        (assign26000_body1_e24434,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign26000_body1_e24436;
            locals.var_m0_rv = 0.0;
        }

        let (assign26010_e24460, assign26010_e24460_d_n0, assign26010_e24460_d_n2, assign26010_e24460_d_n4, assign26010_e24460_d_n5, assign26010_e24460_d_n6, assign26010_e24460_d_n7, assign26010_e24460_d_n8, assign26010_e24460_d_n9, assign26010_e24460_d_n10, assign26010_e24460_d_n11, assign26010_e24460_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) {
        let (assign26010_e24458, assign26010_e24458_d_n0, assign26010_e24458_d_n2, assign26010_e24458_d_n4, assign26010_e24458_d_n5, assign26010_e24458_d_n6, assign26010_e24458_d_n7, assign26010_e24458_d_n8, assign26010_e24458_d_n9, assign26010_e24458_d_n10, assign26010_e24458_d_n11, assign26010_e24458_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26010_e24455: f64 = (2.0 * 2.0);
                let assign26010_e24456: f64 = (1.0 / assign26010_e24455);
                let assign26010_e24457: f64 = (locals.var_dnm).powf(assign26010_e24456);
                (assign26010_e24457, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn0)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn2)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn4)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn5)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn6)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn7)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn8)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn9)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn10)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn11)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26010_e24456) as f64).is_finite() && ((assign26010_e24456) as f64).fract() == 0.0 { if assign26010_e24456 == 0.0 { 0.0 } else { (assign26010_e24456 * ((locals.var_dnm).powf(assign26010_e24456 - 1.0) * locals.var_dnm_dn14)) } } else { (assign26010_e24457 * (assign26010_e24456 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign26010_e24458, assign26010_e24458_d_n0, assign26010_e24458_d_n2, assign26010_e24458_d_n4, assign26010_e24458_d_n5, assign26010_e24458_d_n6, assign26010_e24458_d_n7, assign26010_e24458_d_n8, assign26010_e24458_d_n9, assign26010_e24458_d_n10, assign26010_e24458_d_n11, assign26010_e24458_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26010_e24460;
        locals.var_dnm_dn0 = assign26010_e24460_d_n0;
        locals.var_dnm_dn2 = assign26010_e24460_d_n2;
        locals.var_dnm_dn4 = assign26010_e24460_d_n4;
        locals.var_dnm_dn5 = assign26010_e24460_d_n5;
        locals.var_dnm_dn6 = assign26010_e24460_d_n6;
        locals.var_dnm_dn7 = assign26010_e24460_d_n7;
        locals.var_dnm_dn8 = assign26010_e24460_d_n8;
        locals.var_dnm_dn9 = assign26010_e24460_d_n9;
        locals.var_dnm_dn10 = assign26010_e24460_d_n10;
        locals.var_dnm_dn11 = assign26010_e24460_d_n11;
        locals.var_dnm_dn14 = assign26010_e24460_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26020_e24472, assign26020_e24472_d_n0, assign26020_e24472_d_n2, assign26020_e24472_d_n4, assign26020_e24472_d_n5, assign26020_e24472_d_n6, assign26020_e24472_d_n7, assign26020_e24472_d_n8, assign26020_e24472_d_n9, assign26020_e24472_d_n10, assign26020_e24472_d_n11, assign26020_e24472_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign26020_e24470: f64 = (1.0 / locals.var_dnm);
        (assign26020_e24470, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26020_e24472;
        locals.var_dnm_dn0 = assign26020_e24472_d_n0;
        locals.var_dnm_dn2 = assign26020_e24472_d_n2;
        locals.var_dnm_dn4 = assign26020_e24472_d_n4;
        locals.var_dnm_dn5 = assign26020_e24472_d_n5;
        locals.var_dnm_dn6 = assign26020_e24472_d_n6;
        locals.var_dnm_dn7 = assign26020_e24472_d_n7;
        locals.var_dnm_dn8 = assign26020_e24472_d_n8;
        locals.var_dnm_dn9 = assign26020_e24472_d_n9;
        locals.var_dnm_dn10 = assign26020_e24472_d_n10;
        locals.var_dnm_dn11 = assign26020_e24472_d_n11;
        locals.var_dnm_dn14 = assign26020_e24472_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26030_e24486, assign26030_e24486_d_n0, assign26030_e24486_d_n2, assign26030_e24486_d_n4, assign26030_e24486_d_n5, assign26030_e24486_d_n6, assign26030_e24486_d_n7, assign26030_e24486_d_n8, assign26030_e24486_d_n9, assign26030_e24486_d_n10, assign26030_e24486_d_n11, assign26030_e24486_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign26030_e24482: f64 = (locals.var_tmf1 * 1e-8);
        let assign26030_e24484: f64 = (assign26030_e24482 * locals.var_dnm);
        (assign26030_e24484, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign26030_e24482 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign26030_e24486;
        locals.var_tmf0_dn0 = assign26030_e24486_d_n0;
        locals.var_tmf0_dn2 = assign26030_e24486_d_n2;
        locals.var_tmf0_dn4 = assign26030_e24486_d_n4;
        locals.var_tmf0_dn5 = assign26030_e24486_d_n5;
        locals.var_tmf0_dn6 = assign26030_e24486_d_n6;
        locals.var_tmf0_dn7 = assign26030_e24486_d_n7;
        locals.var_tmf0_dn8 = assign26030_e24486_d_n8;
        locals.var_tmf0_dn9 = assign26030_e24486_d_n9;
        locals.var_tmf0_dn10 = assign26030_e24486_d_n10;
        locals.var_tmf0_dn11 = assign26030_e24486_d_n11;
        locals.var_tmf0_dn14 = assign26030_e24486_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign26040_e24502, assign26040_e24502_d_n0, assign26040_e24502_d_n2, assign26040_e24502_d_n4, assign26040_e24502_d_n5, assign26040_e24502_d_n6, assign26040_e24502_d_n7, assign26040_e24502_d_n8, assign26040_e24502_d_n9, assign26040_e24502_d_n10, assign26040_e24502_d_n11, assign26040_e24502_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign26040_e24496: f64 = (1e-8 * locals.var_xmp);
        let assign26040_e24498: f64 = (assign26040_e24496 * locals.var_dnm);
        let assign26040_e24500: f64 = (assign26040_e24498 / locals.var_arg);
        (assign26040_e24500, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn0)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn2)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn4)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn5)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn6)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn7)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn8)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn9)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn10)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn11)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign26040_e24496 * locals.var_dnm_dn14)) * locals.var_arg) - (assign26040_e24498 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26040_e24502;
        locals.var_t3_dn0 = assign26040_e24502_d_n0;
        locals.var_t3_dn2 = assign26040_e24502_d_n2;
        locals.var_t3_dn4 = assign26040_e24502_d_n4;
        locals.var_t3_dn5 = assign26040_e24502_d_n5;
        locals.var_t3_dn6 = assign26040_e24502_d_n6;
        locals.var_t3_dn7 = assign26040_e24502_d_n7;
        locals.var_t3_dn8 = assign26040_e24502_d_n8;
        locals.var_t3_dn9 = assign26040_e24502_d_n9;
        locals.var_t3_dn10 = assign26040_e24502_d_n10;
        locals.var_t3_dn11 = assign26040_e24502_d_n11;
        locals.var_t3_dn14 = assign26040_e24502_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26050_e24516, assign26050_e24516_d_n0, assign26050_e24516_d_n2, assign26050_e24516_d_n4, assign26050_e24516_d_n5, assign26050_e24516_d_n6, assign26050_e24516_d_n7, assign26050_e24516_d_n8, assign26050_e24516_d_n9, assign26050_e24516_d_n10, assign26050_e24516_d_n11, assign26050_e24516_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        let assign26050_e24512: f64 = (locals.var_uc_depthn - 1e-8);
        let assign26050_e24514: f64 = (assign26050_e24512 + locals.var_tmf0);
        (assign26050_e24514, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign26050_e24516;
        locals.var_w_b0_dn0 = assign26050_e24516_d_n0;
        locals.var_w_b0_dn2 = assign26050_e24516_d_n2;
        locals.var_w_b0_dn4 = assign26050_e24516_d_n4;
        locals.var_w_b0_dn5 = assign26050_e24516_d_n5;
        locals.var_w_b0_dn6 = assign26050_e24516_d_n6;
        locals.var_w_b0_dn7 = assign26050_e24516_d_n7;
        locals.var_w_b0_dn8 = assign26050_e24516_d_n8;
        locals.var_w_b0_dn9 = assign26050_e24516_d_n9;
        locals.var_w_b0_dn10 = assign26050_e24516_d_n10;
        locals.var_w_b0_dn11 = assign26050_e24516_d_n11;
        locals.var_w_b0_dn14 = assign26050_e24516_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign26060_e24526, assign26060_e24526_d_n0, assign26060_e24526_d_n2, assign26060_e24526_d_n4, assign26060_e24526_d_n5, assign26060_e24526_d_n6, assign26060_e24526_d_n7, assign26060_e24526_d_n8, assign26060_e24526_d_n9, assign26060_e24526_d_n10, assign26060_e24526_d_n11, assign26060_e24526_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26060_e24526;
        locals.var_t3_dn0 = assign26060_e24526_d_n0;
        locals.var_t3_dn2 = assign26060_e24526_d_n2;
        locals.var_t3_dn4 = assign26060_e24526_d_n4;
        locals.var_t3_dn5 = assign26060_e24526_d_n5;
        locals.var_t3_dn6 = assign26060_e24526_d_n6;
        locals.var_t3_dn7 = assign26060_e24526_d_n7;
        locals.var_t3_dn8 = assign26060_e24526_d_n8;
        locals.var_t3_dn9 = assign26060_e24526_d_n9;
        locals.var_t3_dn10 = assign26060_e24526_d_n10;
        locals.var_t3_dn11 = assign26060_e24526_d_n11;
        locals.var_t3_dn14 = assign26060_e24526_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26070_e24537, assign26070_e24537_d_n0, assign26070_e24537_d_n2, assign26070_e24537_d_n4, assign26070_e24537_d_n5, assign26070_e24537_d_n6, assign26070_e24537_d_n7, assign26070_e24537_d_n8, assign26070_e24537_d_n9, assign26070_e24537_d_n10, assign26070_e24537_d_n11, assign26070_e24537_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign26070_e24537;
        locals.var_w_b0_dn0 = assign26070_e24537_d_n0;
        locals.var_w_b0_dn2 = assign26070_e24537_d_n2;
        locals.var_w_b0_dn4 = assign26070_e24537_d_n4;
        locals.var_w_b0_dn5 = assign26070_e24537_d_n5;
        locals.var_w_b0_dn6 = assign26070_e24537_d_n6;
        locals.var_w_b0_dn7 = assign26070_e24537_d_n7;
        locals.var_w_b0_dn8 = assign26070_e24537_d_n8;
        locals.var_w_b0_dn9 = assign26070_e24537_d_n9;
        locals.var_w_b0_dn10 = assign26070_e24537_d_n10;
        locals.var_w_b0_dn11 = assign26070_e24537_d_n11;
        locals.var_w_b0_dn14 = assign26070_e24537_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign26080_e24548, assign26080_e24548_d_n0, assign26080_e24548_d_n2, assign26080_e24548_d_n4, assign26080_e24548_d_n5, assign26080_e24548_d_n6, assign26080_e24548_d_n7, assign26080_e24548_d_n8, assign26080_e24548_d_n9, assign26080_e24548_d_n10, assign26080_e24548_d_n11, assign26080_e24548_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) && (locals.var_guard626 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26080_e24548;
        locals.var_t3_dn0 = assign26080_e24548_d_n0;
        locals.var_t3_dn2 = assign26080_e24548_d_n2;
        locals.var_t3_dn4 = assign26080_e24548_d_n4;
        locals.var_t3_dn5 = assign26080_e24548_d_n5;
        locals.var_t3_dn6 = assign26080_e24548_d_n6;
        locals.var_t3_dn7 = assign26080_e24548_d_n7;
        locals.var_t3_dn8 = assign26080_e24548_d_n8;
        locals.var_t3_dn9 = assign26080_e24548_d_n9;
        locals.var_t3_dn10 = assign26080_e24548_d_n10;
        locals.var_t3_dn11 = assign26080_e24548_d_n11;
        locals.var_t3_dn14 = assign26080_e24548_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26090_e24563, assign26090_e24563_d_n0, assign26090_e24563_d_n2, assign26090_e24563_d_n4, assign26090_e24563_d_n5, assign26090_e24563_d_n6, assign26090_e24563_d_n7, assign26090_e24563_d_n8, assign26090_e24563_d_n9, assign26090_e24563_d_n10, assign26090_e24563_d_n11, assign26090_e24563_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) {
        let assign26090_e24557: f64 = (locals.var_phi_j0_dep - locals.var_vbscl__blk437);
        let assign26090_e24559: f64 = (assign26090_e24557 + locals.var_vbi_dep);
        let assign26090_e24560: f64 = (locals.var_c_2esipq_nsub * assign26090_e24559);
        let assign26090_e24561: f64 = (assign26090_e24560).sqrt();
        (assign26090_e24561, (((locals.var_c_2esipq_nsub_dn0 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn0 - locals.var_vbscl__blk437_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign26090_e24561)), (((locals.var_c_2esipq_nsub_dn2 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn2 - locals.var_vbscl__blk437_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign26090_e24561)), (((locals.var_c_2esipq_nsub_dn4 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn4 - locals.var_vbscl__blk437_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign26090_e24561)), (((locals.var_c_2esipq_nsub_dn5 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn5 - locals.var_vbscl__blk437_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign26090_e24561)), (((locals.var_c_2esipq_nsub_dn6 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn6 - locals.var_vbscl__blk437_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign26090_e24561)), (((locals.var_c_2esipq_nsub_dn7 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn7 - locals.var_vbscl__blk437_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign26090_e24561)), (((locals.var_c_2esipq_nsub_dn8 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn8 - locals.var_vbscl__blk437_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign26090_e24561)), (((locals.var_c_2esipq_nsub_dn9 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn9 - locals.var_vbscl__blk437_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign26090_e24561)), (((locals.var_c_2esipq_nsub_dn10 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn10 - locals.var_vbscl__blk437_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign26090_e24561)), (((locals.var_c_2esipq_nsub_dn11 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn11 - locals.var_vbscl__blk437_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign26090_e24561)), (((locals.var_c_2esipq_nsub_dn14 * assign26090_e24559) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn14 - locals.var_vbscl__blk437_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign26090_e24561)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn11, locals.var_w_sub0_dn14,)
    }
};
        locals.var_w_sub0 = assign26090_e24563;
        locals.var_w_sub0_dn0 = assign26090_e24563_d_n0;
        locals.var_w_sub0_dn2 = assign26090_e24563_d_n2;
        locals.var_w_sub0_dn4 = assign26090_e24563_d_n4;
        locals.var_w_sub0_dn5 = assign26090_e24563_d_n5;
        locals.var_w_sub0_dn6 = assign26090_e24563_d_n6;
        locals.var_w_sub0_dn7 = assign26090_e24563_d_n7;
        locals.var_w_sub0_dn8 = assign26090_e24563_d_n8;
        locals.var_w_sub0_dn9 = assign26090_e24563_d_n9;
        locals.var_w_sub0_dn10 = assign26090_e24563_d_n10;
        locals.var_w_sub0_dn11 = assign26090_e24563_d_n11;
        locals.var_w_sub0_dn14 = assign26090_e24563_d_n14;
        locals.var_w_sub0_rv = 0.0;

        let (assign26100_e24573, assign26100_e24573_d_n0, assign26100_e24573_d_n2, assign26100_e24573_d_n4, assign26100_e24573_d_n5, assign26100_e24573_d_n6, assign26100_e24573_d_n7, assign26100_e24573_d_n8, assign26100_e24573_d_n9, assign26100_e24573_d_n10, assign26100_e24573_d_n11, assign26100_e24573_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) {
        let assign26100_e24571: f64 = (locals.var_w_b0 * locals.var_q_ndepm);
        (assign26100_e24571, ((locals.var_w_b0_dn0 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn0)), ((locals.var_w_b0_dn2 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn2)), ((locals.var_w_b0_dn4 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn4)), ((locals.var_w_b0_dn5 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn5)), ((locals.var_w_b0_dn6 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn6)), ((locals.var_w_b0_dn7 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn7)), ((locals.var_w_b0_dn8 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn8)), ((locals.var_w_b0_dn9 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn9)), ((locals.var_w_b0_dn10 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn10)), ((locals.var_w_b0_dn11 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn11)), ((locals.var_w_b0_dn14 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn14,)
    }
};
        locals.var_q_b0_dep = assign26100_e24573;
        locals.var_q_b0_dep_dn0 = assign26100_e24573_d_n0;
        locals.var_q_b0_dep_dn2 = assign26100_e24573_d_n2;
        locals.var_q_b0_dep_dn4 = assign26100_e24573_d_n4;
        locals.var_q_b0_dep_dn5 = assign26100_e24573_d_n5;
        locals.var_q_b0_dep_dn6 = assign26100_e24573_d_n6;
        locals.var_q_b0_dep_dn7 = assign26100_e24573_d_n7;
        locals.var_q_b0_dep_dn8 = assign26100_e24573_d_n8;
        locals.var_q_b0_dep_dn9 = assign26100_e24573_d_n9;
        locals.var_q_b0_dep_dn10 = assign26100_e24573_d_n10;
        locals.var_q_b0_dep_dn11 = assign26100_e24573_d_n11;
        locals.var_q_b0_dep_dn14 = assign26100_e24573_d_n14;
        locals.var_q_b0_dep_rv = 0.0;

        let (assign26110_e24584, assign26110_e24584_d_n0, assign26110_e24584_d_n2, assign26110_e24584_d_n4, assign26110_e24584_d_n5, assign26110_e24584_d_n6, assign26110_e24584_d_n7, assign26110_e24584_d_n8, assign26110_e24584_d_n9, assign26110_e24584_d_n10, assign26110_e24584_d_n11, assign26110_e24584_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 != 0.0)) {
        let assign26110_e24580: f64 = (-locals.var_w_sub0);
        let assign26110_e24582: f64 = (assign26110_e24580 * locals.var_q_nsub__blk546);
        (assign26110_e24582, (((-locals.var_w_sub0_dn0) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn0)), (((-locals.var_w_sub0_dn2) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn2)), (((-locals.var_w_sub0_dn4) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn4)), (((-locals.var_w_sub0_dn5) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn5)), (((-locals.var_w_sub0_dn6) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn6)), (((-locals.var_w_sub0_dn7) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn7)), (((-locals.var_w_sub0_dn8) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn8)), (((-locals.var_w_sub0_dn9) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn9)), (((-locals.var_w_sub0_dn10) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn10)), (((-locals.var_w_sub0_dn11) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn11)), (((-locals.var_w_sub0_dn14) * locals.var_q_nsub__blk546) + (assign26110_e24580 * locals.var_q_nsub__blk546_dn14)),)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn11, locals.var_q_sub0_dep_dn14,)
    }
};
        locals.var_q_sub0_dep = assign26110_e24584;
        locals.var_q_sub0_dep_dn0 = assign26110_e24584_d_n0;
        locals.var_q_sub0_dep_dn2 = assign26110_e24584_d_n2;
        locals.var_q_sub0_dep_dn4 = assign26110_e24584_d_n4;
        locals.var_q_sub0_dep_dn5 = assign26110_e24584_d_n5;
        locals.var_q_sub0_dep_dn6 = assign26110_e24584_d_n6;
        locals.var_q_sub0_dep_dn7 = assign26110_e24584_d_n7;
        locals.var_q_sub0_dep_dn8 = assign26110_e24584_d_n8;
        locals.var_q_sub0_dep_dn9 = assign26110_e24584_d_n9;
        locals.var_q_sub0_dep_dn10 = assign26110_e24584_d_n10;
        locals.var_q_sub0_dep_dn11 = assign26110_e24584_d_n11;
        locals.var_q_sub0_dep_dn14 = assign26110_e24584_d_n14;
        locals.var_q_sub0_dep_rv = 0.0;

        let (assign26120_e24599, assign26120_e24599_d_n0, assign26120_e24599_d_n2, assign26120_e24599_d_n4, assign26120_e24599_d_n5, assign26120_e24599_d_n6, assign26120_e24599_d_n7, assign26120_e24599_d_n8, assign26120_e24599_d_n9, assign26120_e24599_d_n10, assign26120_e24599_d_n11, assign26120_e24599_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) {
        let assign26120_e24592: f64 = (-locals.var_beta);
        let assign26120_e24595: f64 = (locals.var_phi_s0_dep - locals.var_vbscl__blk437);
        let assign26120_e24596: f64 = (assign26120_e24592 * assign26120_e24595);
        let assign26120_e24597: f64 = (assign26120_e24596).exp();
        (assign26120_e24597, (assign26120_e24597 * (((-locals.var_beta_dn0) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn0 - locals.var_vbscl__blk437_dn0)))), (assign26120_e24597 * (((-locals.var_beta_dn2) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn2 - locals.var_vbscl__blk437_dn2)))), (assign26120_e24597 * (((-locals.var_beta_dn4) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn4 - locals.var_vbscl__blk437_dn4)))), (assign26120_e24597 * (((-locals.var_beta_dn5) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn5 - locals.var_vbscl__blk437_dn5)))), (assign26120_e24597 * (((-locals.var_beta_dn6) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn6 - locals.var_vbscl__blk437_dn6)))), (assign26120_e24597 * (((-locals.var_beta_dn7) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn7 - locals.var_vbscl__blk437_dn7)))), (assign26120_e24597 * (((-locals.var_beta_dn8) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn8 - locals.var_vbscl__blk437_dn8)))), (assign26120_e24597 * (((-locals.var_beta_dn9) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn9 - locals.var_vbscl__blk437_dn9)))), (assign26120_e24597 * (((-locals.var_beta_dn10) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn10 - locals.var_vbscl__blk437_dn10)))), (assign26120_e24597 * (((-locals.var_beta_dn11) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn11 - locals.var_vbscl__blk437_dn11)))), (assign26120_e24597 * (((-locals.var_beta_dn14) * assign26120_e24595) + (assign26120_e24592 * (locals.var_phi_s0_dep_dn14 - locals.var_vbscl__blk437_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26120_e24599;
        locals.var_t3_dn0 = assign26120_e24599_d_n0;
        locals.var_t3_dn2 = assign26120_e24599_d_n2;
        locals.var_t3_dn4 = assign26120_e24599_d_n4;
        locals.var_t3_dn5 = assign26120_e24599_d_n5;
        locals.var_t3_dn6 = assign26120_e24599_d_n6;
        locals.var_t3_dn7 = assign26120_e24599_d_n7;
        locals.var_t3_dn8 = assign26120_e24599_d_n8;
        locals.var_t3_dn9 = assign26120_e24599_d_n9;
        locals.var_t3_dn10 = assign26120_e24599_d_n10;
        locals.var_t3_dn11 = assign26120_e24599_d_n11;
        locals.var_t3_dn14 = assign26120_e24599_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_78(
        locals: &mut StampLocals,
    ) {
        let (assign26130_e24614, assign26130_e24614_d_n0, assign26130_e24614_d_n2, assign26130_e24614_d_n4, assign26130_e24614_d_n5, assign26130_e24614_d_n6, assign26130_e24614_d_n7, assign26130_e24614_d_n8, assign26130_e24614_d_n9, assign26130_e24614_d_n10, assign26130_e24614_d_n11, assign26130_e24614_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) {
        let assign26130_e24607: f64 = (-locals.var_beta);
        let assign26130_e24610: f64 = (locals.var_phi_b0_dep - locals.var_vbscl__blk437);
        let assign26130_e24611: f64 = (assign26130_e24607 * assign26130_e24610);
        let assign26130_e24612: f64 = (assign26130_e24611).exp();
        (assign26130_e24612, (assign26130_e24612 * (((-locals.var_beta_dn0) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn0 - locals.var_vbscl__blk437_dn0)))), (assign26130_e24612 * (((-locals.var_beta_dn2) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn2 - locals.var_vbscl__blk437_dn2)))), (assign26130_e24612 * (((-locals.var_beta_dn4) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn4 - locals.var_vbscl__blk437_dn4)))), (assign26130_e24612 * (((-locals.var_beta_dn5) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn5 - locals.var_vbscl__blk437_dn5)))), (assign26130_e24612 * (((-locals.var_beta_dn6) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn6 - locals.var_vbscl__blk437_dn6)))), (assign26130_e24612 * (((-locals.var_beta_dn7) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn7 - locals.var_vbscl__blk437_dn7)))), (assign26130_e24612 * (((-locals.var_beta_dn8) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn8 - locals.var_vbscl__blk437_dn8)))), (assign26130_e24612 * (((-locals.var_beta_dn9) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn9 - locals.var_vbscl__blk437_dn9)))), (assign26130_e24612 * (((-locals.var_beta_dn10) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn10 - locals.var_vbscl__blk437_dn10)))), (assign26130_e24612 * (((-locals.var_beta_dn11) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn11 - locals.var_vbscl__blk437_dn11)))), (assign26130_e24612 * (((-locals.var_beta_dn14) * assign26130_e24610) + (assign26130_e24607 * (locals.var_phi_b0_dep_dn14 - locals.var_vbscl__blk437_dn14)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign26130_e24614;
        locals.var_t4_dn0 = assign26130_e24614_d_n0;
        locals.var_t4_dn2 = assign26130_e24614_d_n2;
        locals.var_t4_dn4 = assign26130_e24614_d_n4;
        locals.var_t4_dn5 = assign26130_e24614_d_n5;
        locals.var_t4_dn6 = assign26130_e24614_d_n6;
        locals.var_t4_dn7 = assign26130_e24614_d_n7;
        locals.var_t4_dn8 = assign26130_e24614_d_n8;
        locals.var_t4_dn9 = assign26130_e24614_d_n9;
        locals.var_t4_dn10 = assign26130_e24614_d_n10;
        locals.var_t4_dn11 = assign26130_e24614_d_n11;
        locals.var_t4_dn14 = assign26130_e24614_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign26140_e24638, assign26140_e24638_d_n0, assign26140_e24638_d_n2, assign26140_e24638_d_n4, assign26140_e24638_d_n5, assign26140_e24638_d_n6, assign26140_e24638_d_n7, assign26140_e24638_d_n8, assign26140_e24638_d_n9, assign26140_e24638_d_n10, assign26140_e24638_d_n11, assign26140_e24638_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) {
        let assign26140_e24624: f64 = (locals.var_t2 - 1.0);
        let assign26140_e24626: f64 = (assign26140_e24624 - locals.var_t1);
        let assign26140_e24630: f64 = (locals.var_t3 - locals.var_t4);
        let assign26140_e24631: f64 = (locals.var_cnst1 * assign26140_e24630);
        let assign26140_e24632: f64 = (assign26140_e24626 + assign26140_e24631);
        let assign26140_e24634: f64 = (assign26140_e24632 + 1e-15);
        let assign26140_e24635: f64 = (assign26140_e24634).sqrt();
        let assign26140_e24636: f64 = (locals.var_cnst0 * assign26140_e24635);
        (assign26140_e24636, ((locals.var_cnst0_dn0 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign26140_e24635)))), ((locals.var_cnst0_dn2 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign26140_e24635)))), ((locals.var_cnst0_dn4 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign26140_e24635)))), ((locals.var_cnst0_dn5 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign26140_e24635)))), ((locals.var_cnst0_dn6 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign26140_e24635)))), ((locals.var_cnst0_dn7 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign26140_e24635)))), ((locals.var_cnst0_dn8 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign26140_e24635)))), ((locals.var_cnst0_dn9 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign26140_e24635)))), ((locals.var_cnst0_dn10 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign26140_e24635)))), ((locals.var_cnst0_dn11 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn11 - locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn11 - locals.var_t4_dn11)))) / (2.0 * assign26140_e24635)))), ((locals.var_cnst0_dn14 * assign26140_e24635) + (locals.var_cnst0 * (((locals.var_t2_dn14 - locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign26140_e24630) + (locals.var_cnst1 * (locals.var_t3_dn14 - locals.var_t4_dn14)))) / (2.0 * assign26140_e24635)))),)
    } else {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn11, locals.var_q_s0_dn14,)
    }
};
        locals.var_q_s0 = assign26140_e24638;
        locals.var_q_s0_dn0 = assign26140_e24638_d_n0;
        locals.var_q_s0_dn2 = assign26140_e24638_d_n2;
        locals.var_q_s0_dn4 = assign26140_e24638_d_n4;
        locals.var_q_s0_dn5 = assign26140_e24638_d_n5;
        locals.var_q_s0_dn6 = assign26140_e24638_d_n6;
        locals.var_q_s0_dn7 = assign26140_e24638_d_n7;
        locals.var_q_s0_dn8 = assign26140_e24638_d_n8;
        locals.var_q_s0_dn9 = assign26140_e24638_d_n9;
        locals.var_q_s0_dn10 = assign26140_e24638_d_n10;
        locals.var_q_s0_dn11 = assign26140_e24638_d_n11;
        locals.var_q_s0_dn14 = assign26140_e24638_d_n14;
        locals.var_q_s0_rv = 0.0;

        let assign26150_e24645: f64 = if ((locals.var_w_bsub0 > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard632 = assign26150_e24645;
        locals.var_guard632_rv = 0.0;

        let (assign26160_e24656, assign26160_e24656_d_n0, assign26160_e24656_d_n2, assign26160_e24656_d_n4, assign26160_e24656_d_n5, assign26160_e24656_d_n6, assign26160_e24656_d_n7, assign26160_e24656_d_n8, assign26160_e24656_d_n9, assign26160_e24656_d_n10, assign26160_e24656_d_n11, assign26160_e24656_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard632 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn11, locals.var_q_sub0_dn14,)
    }
};
        locals.var_q_sub0 = assign26160_e24656;
        locals.var_q_sub0_dn0 = assign26160_e24656_d_n0;
        locals.var_q_sub0_dn2 = assign26160_e24656_d_n2;
        locals.var_q_sub0_dn4 = assign26160_e24656_d_n4;
        locals.var_q_sub0_dn5 = assign26160_e24656_d_n5;
        locals.var_q_sub0_dn6 = assign26160_e24656_d_n6;
        locals.var_q_sub0_dn7 = assign26160_e24656_d_n7;
        locals.var_q_sub0_dn8 = assign26160_e24656_d_n8;
        locals.var_q_sub0_dn9 = assign26160_e24656_d_n9;
        locals.var_q_sub0_dn10 = assign26160_e24656_d_n10;
        locals.var_q_sub0_dn11 = assign26160_e24656_d_n11;
        locals.var_q_sub0_dn14 = assign26160_e24656_d_n14;
        locals.var_q_sub0_rv = 0.0;

        let (assign26170_e24667, assign26170_e24667_d_n0, assign26170_e24667_d_n2, assign26170_e24667_d_n4, assign26170_e24667_d_n5, assign26170_e24667_d_n6, assign26170_e24667_d_n7, assign26170_e24667_d_n8, assign26170_e24667_d_n9, assign26170_e24667_d_n10, assign26170_e24667_d_n11, assign26170_e24667_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard632 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn11, locals.var_q_s0_dep_dn14,)
    }
};
        locals.var_q_s0_dep = assign26170_e24667;
        locals.var_q_s0_dep_dn0 = assign26170_e24667_d_n0;
        locals.var_q_s0_dep_dn2 = assign26170_e24667_d_n2;
        locals.var_q_s0_dep_dn4 = assign26170_e24667_d_n4;
        locals.var_q_s0_dep_dn5 = assign26170_e24667_d_n5;
        locals.var_q_s0_dep_dn6 = assign26170_e24667_d_n6;
        locals.var_q_s0_dep_dn7 = assign26170_e24667_d_n7;
        locals.var_q_s0_dep_dn8 = assign26170_e24667_d_n8;
        locals.var_q_s0_dep_dn9 = assign26170_e24667_d_n9;
        locals.var_q_s0_dep_dn10 = assign26170_e24667_d_n10;
        locals.var_q_s0_dep_dn11 = assign26170_e24667_d_n11;
        locals.var_q_s0_dep_dn14 = assign26170_e24667_d_n14;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign26180_e24701, assign26180_e24701_d_n0, assign26180_e24701_d_n2, assign26180_e24701_d_n4, assign26180_e24701_d_n5, assign26180_e24701_d_n6, assign26180_e24701_d_n7, assign26180_e24701_d_n8, assign26180_e24701_d_n9, assign26180_e24701_d_n10, assign26180_e24701_d_n11, assign26180_e24701_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign26180_e24679: f64 = (-locals.var_t1);
        let assign26180_e24682: f64 = (-locals.var_beta);
        let assign26180_e24685: f64 = (locals.var_phi_s0_dep - locals.var_vbscl__blk437);
        let assign26180_e24686: f64 = (assign26180_e24682 * assign26180_e24685);
        let assign26180_e24687: f64 = (assign26180_e24686).exp();
        let assign26180_e24689: f64 = (-locals.var_beta);
        let assign26180_e24692: f64 = (locals.var_phi_b0_dep - locals.var_vbscl__blk437);
        let assign26180_e24693: f64 = (assign26180_e24689 * assign26180_e24692);
        let assign26180_e24694: f64 = (assign26180_e24693).exp();
        let assign26180_e24695: f64 = (assign26180_e24687 - assign26180_e24694);
        let assign26180_e24696: f64 = (locals.var_cnst1 * assign26180_e24695);
        let assign26180_e24697: f64 = (assign26180_e24679 + assign26180_e24696);
        let assign26180_e24698: f64 = (assign26180_e24697).sqrt();
        let assign26180_e24699: f64 = (locals.var_cnst0 * assign26180_e24698);
        (assign26180_e24699, ((locals.var_cnst0_dn0 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn0) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn0 - locals.var_vbscl__blk437_dn0)))) - (assign26180_e24694 * (((-locals.var_beta_dn0) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn0 - locals.var_vbscl__blk437_dn0)))))))) / (2.0 * assign26180_e24698)))), ((locals.var_cnst0_dn2 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn2) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn2 - locals.var_vbscl__blk437_dn2)))) - (assign26180_e24694 * (((-locals.var_beta_dn2) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn2 - locals.var_vbscl__blk437_dn2)))))))) / (2.0 * assign26180_e24698)))), ((locals.var_cnst0_dn4 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn4) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn4 - locals.var_vbscl__blk437_dn4)))) - (assign26180_e24694 * (((-locals.var_beta_dn4) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn4 - locals.var_vbscl__blk437_dn4)))))))) / (2.0 * assign26180_e24698)))), ((locals.var_cnst0_dn5 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn5) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn5 - locals.var_vbscl__blk437_dn5)))) - (assign26180_e24694 * (((-locals.var_beta_dn5) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn5 - locals.var_vbscl__blk437_dn5)))))))) / (2.0 * assign26180_e24698)))), ((locals.var_cnst0_dn6 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn6) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn6 - locals.var_vbscl__blk437_dn6)))) - (assign26180_e24694 * (((-locals.var_beta_dn6) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn6 - locals.var_vbscl__blk437_dn6)))))))) / (2.0 * assign26180_e24698)))), ((locals.var_cnst0_dn7 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn7) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn7 - locals.var_vbscl__blk437_dn7)))) - (assign26180_e24694 * (((-locals.var_beta_dn7) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn7 - locals.var_vbscl__blk437_dn7)))))))) / (2.0 * assign26180_e24698)))), ((locals.var_cnst0_dn8 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn8) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn8 - locals.var_vbscl__blk437_dn8)))) - (assign26180_e24694 * (((-locals.var_beta_dn8) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn8 - locals.var_vbscl__blk437_dn8)))))))) / (2.0 * assign26180_e24698)))), ((locals.var_cnst0_dn9 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn9) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn9 - locals.var_vbscl__blk437_dn9)))) - (assign26180_e24694 * (((-locals.var_beta_dn9) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn9 - locals.var_vbscl__blk437_dn9)))))))) / (2.0 * assign26180_e24698)))), ((locals.var_cnst0_dn10 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn10) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn10 - locals.var_vbscl__blk437_dn10)))) - (assign26180_e24694 * (((-locals.var_beta_dn10) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn10 - locals.var_vbscl__blk437_dn10)))))))) / (2.0 * assign26180_e24698)))), ((locals.var_cnst0_dn11 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn11) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn11 - locals.var_vbscl__blk437_dn11)))) - (assign26180_e24694 * (((-locals.var_beta_dn11) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn11 - locals.var_vbscl__blk437_dn11)))))))) / (2.0 * assign26180_e24698)))), ((locals.var_cnst0_dn14 * assign26180_e24698) + (locals.var_cnst0 * (((-locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign26180_e24695) + (locals.var_cnst1 * ((assign26180_e24687 * (((-locals.var_beta_dn14) * assign26180_e24685) + (assign26180_e24682 * (locals.var_phi_s0_dep_dn14 - locals.var_vbscl__blk437_dn14)))) - (assign26180_e24694 * (((-locals.var_beta_dn14) * assign26180_e24692) + (assign26180_e24689 * (locals.var_phi_b0_dep_dn14 - locals.var_vbscl__blk437_dn14)))))))) / (2.0 * assign26180_e24698)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26180_e24701;
        locals.var_t3_dn0 = assign26180_e24701_d_n0;
        locals.var_t3_dn2 = assign26180_e24701_d_n2;
        locals.var_t3_dn4 = assign26180_e24701_d_n4;
        locals.var_t3_dn5 = assign26180_e24701_d_n5;
        locals.var_t3_dn6 = assign26180_e24701_d_n6;
        locals.var_t3_dn7 = assign26180_e24701_d_n7;
        locals.var_t3_dn8 = assign26180_e24701_d_n8;
        locals.var_t3_dn9 = assign26180_e24701_d_n9;
        locals.var_t3_dn10 = assign26180_e24701_d_n10;
        locals.var_t3_dn11 = assign26180_e24701_d_n11;
        locals.var_t3_dn14 = assign26180_e24701_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26190_e24719, assign26190_e24719_d_n0, assign26190_e24719_d_n2, assign26190_e24719_d_n4, assign26190_e24719_d_n5, assign26190_e24719_d_n6, assign26190_e24719_d_n7, assign26190_e24719_d_n8, assign26190_e24719_d_n9, assign26190_e24719_d_n10, assign26190_e24719_d_n11, assign26190_e24719_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign26190_e24714: f64 = (-locals.var_t1);
        let assign26190_e24715: f64 = (assign26190_e24714).sqrt();
        let assign26190_e24716: f64 = (locals.var_cnst0 * assign26190_e24715);
        let assign26190_e24717: f64 = (locals.var_t3 - assign26190_e24716);
        (assign26190_e24717, (locals.var_t3_dn0 - ((locals.var_cnst0_dn0 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn0) / (2.0 * assign26190_e24715))))), (locals.var_t3_dn2 - ((locals.var_cnst0_dn2 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn2) / (2.0 * assign26190_e24715))))), (locals.var_t3_dn4 - ((locals.var_cnst0_dn4 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn4) / (2.0 * assign26190_e24715))))), (locals.var_t3_dn5 - ((locals.var_cnst0_dn5 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn5) / (2.0 * assign26190_e24715))))), (locals.var_t3_dn6 - ((locals.var_cnst0_dn6 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn6) / (2.0 * assign26190_e24715))))), (locals.var_t3_dn7 - ((locals.var_cnst0_dn7 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn7) / (2.0 * assign26190_e24715))))), (locals.var_t3_dn8 - ((locals.var_cnst0_dn8 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn8) / (2.0 * assign26190_e24715))))), (locals.var_t3_dn9 - ((locals.var_cnst0_dn9 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn9) / (2.0 * assign26190_e24715))))), (locals.var_t3_dn10 - ((locals.var_cnst0_dn10 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn10) / (2.0 * assign26190_e24715))))), (locals.var_t3_dn11 - ((locals.var_cnst0_dn11 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn11) / (2.0 * assign26190_e24715))))), (locals.var_t3_dn14 - ((locals.var_cnst0_dn14 * assign26190_e24715) + (locals.var_cnst0 * ((-locals.var_t1_dn14) / (2.0 * assign26190_e24715))))),)
    } else {
        (locals.var_q_sub0, locals.var_q_sub0_dn0, locals.var_q_sub0_dn2, locals.var_q_sub0_dn4, locals.var_q_sub0_dn5, locals.var_q_sub0_dn6, locals.var_q_sub0_dn7, locals.var_q_sub0_dn8, locals.var_q_sub0_dn9, locals.var_q_sub0_dn10, locals.var_q_sub0_dn11, locals.var_q_sub0_dn14,)
    }
};
        locals.var_q_sub0 = assign26190_e24719;
        locals.var_q_sub0_dn0 = assign26190_e24719_d_n0;
        locals.var_q_sub0_dn2 = assign26190_e24719_d_n2;
        locals.var_q_sub0_dn4 = assign26190_e24719_d_n4;
        locals.var_q_sub0_dn5 = assign26190_e24719_d_n5;
        locals.var_q_sub0_dn6 = assign26190_e24719_d_n6;
        locals.var_q_sub0_dn7 = assign26190_e24719_d_n7;
        locals.var_q_sub0_dn8 = assign26190_e24719_d_n8;
        locals.var_q_sub0_dn9 = assign26190_e24719_d_n9;
        locals.var_q_sub0_dn10 = assign26190_e24719_d_n10;
        locals.var_q_sub0_dn11 = assign26190_e24719_d_n11;
        locals.var_q_sub0_dn14 = assign26190_e24719_d_n14;
        locals.var_q_sub0_rv = 0.0;

        let (assign26200_e24740, assign26200_e24740_d_n0, assign26200_e24740_d_n2, assign26200_e24740_d_n4, assign26200_e24740_d_n5, assign26200_e24740_d_n6, assign26200_e24740_d_n7, assign26200_e24740_d_n8, assign26200_e24740_d_n9, assign26200_e24740_d_n10, assign26200_e24740_d_n11, assign26200_e24740_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard632 == 0.0)) {
        let assign26200_e24732: f64 = (locals.var_t2 - 1.0);
        let assign26200_e24734: f64 = (assign26200_e24732 - locals.var_t1);
        let assign26200_e24736: f64 = (assign26200_e24734 + 1e-15);
        let assign26200_e24737: f64 = (assign26200_e24736).sqrt();
        let assign26200_e24738: f64 = (locals.var_cnst0 * assign26200_e24737);
        (assign26200_e24738, ((locals.var_cnst0_dn0 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign26200_e24737)))), ((locals.var_cnst0_dn2 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign26200_e24737)))), ((locals.var_cnst0_dn4 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign26200_e24737)))), ((locals.var_cnst0_dn5 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign26200_e24737)))), ((locals.var_cnst0_dn6 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign26200_e24737)))), ((locals.var_cnst0_dn7 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign26200_e24737)))), ((locals.var_cnst0_dn8 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign26200_e24737)))), ((locals.var_cnst0_dn9 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign26200_e24737)))), ((locals.var_cnst0_dn10 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign26200_e24737)))), ((locals.var_cnst0_dn11 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign26200_e24737)))), ((locals.var_cnst0_dn14 * assign26200_e24737) + (locals.var_cnst0 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign26200_e24737)))),)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn11, locals.var_q_s0_dep_dn14,)
    }
};
        locals.var_q_s0_dep = assign26200_e24740;
        locals.var_q_s0_dep_dn0 = assign26200_e24740_d_n0;
        locals.var_q_s0_dep_dn2 = assign26200_e24740_d_n2;
        locals.var_q_s0_dep_dn4 = assign26200_e24740_d_n4;
        locals.var_q_s0_dep_dn5 = assign26200_e24740_d_n5;
        locals.var_q_s0_dep_dn6 = assign26200_e24740_d_n6;
        locals.var_q_s0_dep_dn7 = assign26200_e24740_d_n7;
        locals.var_q_s0_dep_dn8 = assign26200_e24740_d_n8;
        locals.var_q_s0_dep_dn9 = assign26200_e24740_d_n9;
        locals.var_q_s0_dep_dn10 = assign26200_e24740_d_n10;
        locals.var_q_s0_dep_dn11 = assign26200_e24740_d_n11;
        locals.var_q_s0_dep_dn14 = assign26200_e24740_d_n14;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign26210_e24749, assign26210_e24749_d_n0, assign26210_e24749_d_n2, assign26210_e24749_d_n4, assign26210_e24749_d_n5, assign26210_e24749_d_n6, assign26210_e24749_d_n7, assign26210_e24749_d_n8, assign26210_e24749_d_n9, assign26210_e24749_d_n10, assign26210_e24749_d_n11, assign26210_e24749_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_n0__blk540, locals.var_q_n0__blk540_dn0, locals.var_q_n0__blk540_dn2, locals.var_q_n0__blk540_dn4, locals.var_q_n0__blk540_dn5, locals.var_q_n0__blk540_dn6, locals.var_q_n0__blk540_dn7, locals.var_q_n0__blk540_dn8, locals.var_q_n0__blk540_dn9, locals.var_q_n0__blk540_dn10, locals.var_q_n0__blk540_dn11, locals.var_q_n0__blk540_dn14,)
    }
};
        locals.var_q_n0__blk540 = assign26210_e24749;
        locals.var_q_n0__blk540_dn0 = assign26210_e24749_d_n0;
        locals.var_q_n0__blk540_dn2 = assign26210_e24749_d_n2;
        locals.var_q_n0__blk540_dn4 = assign26210_e24749_d_n4;
        locals.var_q_n0__blk540_dn5 = assign26210_e24749_d_n5;
        locals.var_q_n0__blk540_dn6 = assign26210_e24749_d_n6;
        locals.var_q_n0__blk540_dn7 = assign26210_e24749_d_n7;
        locals.var_q_n0__blk540_dn8 = assign26210_e24749_d_n8;
        locals.var_q_n0__blk540_dn9 = assign26210_e24749_d_n9;
        locals.var_q_n0__blk540_dn10 = assign26210_e24749_d_n10;
        locals.var_q_n0__blk540_dn11 = assign26210_e24749_d_n11;
        locals.var_q_n0__blk540_dn14 = assign26210_e24749_d_n14;
        locals.var_q_n0__blk540_rv = 0.0;

        let (assign26220_e24760, assign26220_e24760_d_n0, assign26220_e24760_d_n2, assign26220_e24760_d_n4, assign26220_e24760_d_n5, assign26220_e24760_d_n6, assign26220_e24760_d_n7, assign26220_e24760_d_n8, assign26220_e24760_d_n9, assign26220_e24760_d_n10, assign26220_e24760_d_n11, assign26220_e24760_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) {
        let assign26220_e24758: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        (assign26220_e24758, (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10), (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11), (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign26220_e24760;
        locals.var_t1_dn0 = assign26220_e24760_d_n0;
        locals.var_t1_dn2 = assign26220_e24760_d_n2;
        locals.var_t1_dn4 = assign26220_e24760_d_n4;
        locals.var_t1_dn5 = assign26220_e24760_d_n5;
        locals.var_t1_dn6 = assign26220_e24760_d_n6;
        locals.var_t1_dn7 = assign26220_e24760_d_n7;
        locals.var_t1_dn8 = assign26220_e24760_d_n8;
        locals.var_t1_dn9 = assign26220_e24760_d_n9;
        locals.var_t1_dn10 = assign26220_e24760_d_n10;
        locals.var_t1_dn11 = assign26220_e24760_d_n11;
        locals.var_t1_dn14 = assign26220_e24760_d_n14;
        locals.var_t1_rv = 0.0;

        let assign26230_e24764: f64 = 0.1;
        let assign26230_e24769: f64 = if ((locals.var_t1 < assign26230_e24764) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard633 = assign26230_e24769;
        locals.var_guard633_rv = 0.0;

        let (assign26240_e24784, assign26240_e24784_d_n0, assign26240_e24784_d_n2, assign26240_e24784_d_n4, assign26240_e24784_d_n5, assign26240_e24784_d_n6, assign26240_e24784_d_n7, assign26240_e24784_d_n8, assign26240_e24784_d_n9, assign26240_e24784_d_n10, assign26240_e24784_d_n11, assign26240_e24784_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26240_e24780: f64 = 0.1;
        let assign26240_e24782: f64 = (assign26240_e24780 - locals.var_t1);
        (assign26240_e24782, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign26240_e24784;
        locals.var_tmf1_dn0 = assign26240_e24784_d_n0;
        locals.var_tmf1_dn2 = assign26240_e24784_d_n2;
        locals.var_tmf1_dn4 = assign26240_e24784_d_n4;
        locals.var_tmf1_dn5 = assign26240_e24784_d_n5;
        locals.var_tmf1_dn6 = assign26240_e24784_d_n6;
        locals.var_tmf1_dn7 = assign26240_e24784_d_n7;
        locals.var_tmf1_dn8 = assign26240_e24784_d_n8;
        locals.var_tmf1_dn9 = assign26240_e24784_d_n9;
        locals.var_tmf1_dn10 = assign26240_e24784_d_n10;
        locals.var_tmf1_dn11 = assign26240_e24784_d_n11;
        locals.var_tmf1_dn14 = assign26240_e24784_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign26250_e24797, assign26250_e24797_d_n0, assign26250_e24797_d_n2, assign26250_e24797_d_n4, assign26250_e24797_d_n5, assign26250_e24797_d_n6, assign26250_e24797_d_n7, assign26250_e24797_d_n8, assign26250_e24797_d_n9, assign26250_e24797_d_n10, assign26250_e24797_d_n11, assign26250_e24797_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26250_e24795: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign26250_e24795, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign26250_e24797;
        locals.var_x2_dn0 = assign26250_e24797_d_n0;
        locals.var_x2_dn2 = assign26250_e24797_d_n2;
        locals.var_x2_dn4 = assign26250_e24797_d_n4;
        locals.var_x2_dn5 = assign26250_e24797_d_n5;
        locals.var_x2_dn6 = assign26250_e24797_d_n6;
        locals.var_x2_dn7 = assign26250_e24797_d_n7;
        locals.var_x2_dn8 = assign26250_e24797_d_n8;
        locals.var_x2_dn9 = assign26250_e24797_d_n9;
        locals.var_x2_dn10 = assign26250_e24797_d_n10;
        locals.var_x2_dn11 = assign26250_e24797_d_n11;
        locals.var_x2_dn14 = assign26250_e24797_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign26260_e24810, assign26260_e24810_d_n0, assign26260_e24810_d_n2, assign26260_e24810_d_n4, assign26260_e24810_d_n5, assign26260_e24810_d_n6, assign26260_e24810_d_n7, assign26260_e24810_d_n8, assign26260_e24810_d_n9, assign26260_e24810_d_n10, assign26260_e24810_d_n11, assign26260_e24810_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26260_e24808: f64 = (0.1 * 0.1);
        (assign26260_e24808, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign26260_e24810;
        locals.var_xmax2_dn0 = assign26260_e24810_d_n0;
        locals.var_xmax2_dn2 = assign26260_e24810_d_n2;
        locals.var_xmax2_dn4 = assign26260_e24810_d_n4;
        locals.var_xmax2_dn5 = assign26260_e24810_d_n5;
        locals.var_xmax2_dn6 = assign26260_e24810_d_n6;
        locals.var_xmax2_dn7 = assign26260_e24810_d_n7;
        locals.var_xmax2_dn8 = assign26260_e24810_d_n8;
        locals.var_xmax2_dn9 = assign26260_e24810_d_n9;
        locals.var_xmax2_dn10 = assign26260_e24810_d_n10;
        locals.var_xmax2_dn11 = assign26260_e24810_d_n11;
        locals.var_xmax2_dn14 = assign26260_e24810_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign26270_e24821, assign26270_e24821_d_n0, assign26270_e24821_d_n2, assign26270_e24821_d_n4, assign26270_e24821_d_n5, assign26270_e24821_d_n6, assign26270_e24821_d_n7, assign26270_e24821_d_n8, assign26270_e24821_d_n9, assign26270_e24821_d_n10, assign26270_e24821_d_n11, assign26270_e24821_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26270_e24821;
        locals.var_xp_dn0 = assign26270_e24821_d_n0;
        locals.var_xp_dn2 = assign26270_e24821_d_n2;
        locals.var_xp_dn4 = assign26270_e24821_d_n4;
        locals.var_xp_dn5 = assign26270_e24821_d_n5;
        locals.var_xp_dn6 = assign26270_e24821_d_n6;
        locals.var_xp_dn7 = assign26270_e24821_d_n7;
        locals.var_xp_dn8 = assign26270_e24821_d_n8;
        locals.var_xp_dn9 = assign26270_e24821_d_n9;
        locals.var_xp_dn10 = assign26270_e24821_d_n10;
        locals.var_xp_dn11 = assign26270_e24821_d_n11;
        locals.var_xp_dn14 = assign26270_e24821_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26280_e24832, assign26280_e24832_d_n0, assign26280_e24832_d_n2, assign26280_e24832_d_n4, assign26280_e24832_d_n5, assign26280_e24832_d_n6, assign26280_e24832_d_n7, assign26280_e24832_d_n8, assign26280_e24832_d_n9, assign26280_e24832_d_n10, assign26280_e24832_d_n11, assign26280_e24832_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26280_e24832;
        locals.var_xmp_dn0 = assign26280_e24832_d_n0;
        locals.var_xmp_dn2 = assign26280_e24832_d_n2;
        locals.var_xmp_dn4 = assign26280_e24832_d_n4;
        locals.var_xmp_dn5 = assign26280_e24832_d_n5;
        locals.var_xmp_dn6 = assign26280_e24832_d_n6;
        locals.var_xmp_dn7 = assign26280_e24832_d_n7;
        locals.var_xmp_dn8 = assign26280_e24832_d_n8;
        locals.var_xmp_dn9 = assign26280_e24832_d_n9;
        locals.var_xmp_dn10 = assign26280_e24832_d_n10;
        locals.var_xmp_dn11 = assign26280_e24832_d_n11;
        locals.var_xmp_dn14 = assign26280_e24832_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign26290_e24843,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26290_e24843;
        locals.var_m0_rv = 0.0;

        let (assign26300_e24854,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26300_e24854;
        locals.var_mm_rv = 0.0;

        let (assign26310_e24865, assign26310_e24865_d_n0, assign26310_e24865_d_n2, assign26310_e24865_d_n4, assign26310_e24865_d_n5, assign26310_e24865_d_n6, assign26310_e24865_d_n7, assign26310_e24865_d_n8, assign26310_e24865_d_n9, assign26310_e24865_d_n10, assign26310_e24865_d_n11, assign26310_e24865_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26310_e24865;
        locals.var_arg_dn0 = assign26310_e24865_d_n0;
        locals.var_arg_dn2 = assign26310_e24865_d_n2;
        locals.var_arg_dn4 = assign26310_e24865_d_n4;
        locals.var_arg_dn5 = assign26310_e24865_d_n5;
        locals.var_arg_dn6 = assign26310_e24865_d_n6;
        locals.var_arg_dn7 = assign26310_e24865_d_n7;
        locals.var_arg_dn8 = assign26310_e24865_d_n8;
        locals.var_arg_dn9 = assign26310_e24865_d_n9;
        locals.var_arg_dn10 = assign26310_e24865_d_n10;
        locals.var_arg_dn11 = assign26310_e24865_d_n11;
        locals.var_arg_dn14 = assign26310_e24865_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign26320_e24876, assign26320_e24876_d_n0, assign26320_e24876_d_n2, assign26320_e24876_d_n4, assign26320_e24876_d_n5, assign26320_e24876_d_n6, assign26320_e24876_d_n7, assign26320_e24876_d_n8, assign26320_e24876_d_n9, assign26320_e24876_d_n10, assign26320_e24876_d_n11, assign26320_e24876_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26320_e24876;
        locals.var_dnm_dn0 = assign26320_e24876_d_n0;
        locals.var_dnm_dn2 = assign26320_e24876_d_n2;
        locals.var_dnm_dn4 = assign26320_e24876_d_n4;
        locals.var_dnm_dn5 = assign26320_e24876_d_n5;
        locals.var_dnm_dn6 = assign26320_e24876_d_n6;
        locals.var_dnm_dn7 = assign26320_e24876_d_n7;
        locals.var_dnm_dn8 = assign26320_e24876_d_n8;
        locals.var_dnm_dn9 = assign26320_e24876_d_n9;
        locals.var_dnm_dn10 = assign26320_e24876_d_n10;
        locals.var_dnm_dn11 = assign26320_e24876_d_n11;
        locals.var_dnm_dn14 = assign26320_e24876_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26330_e24889, assign26330_e24889_d_n0, assign26330_e24889_d_n2, assign26330_e24889_d_n4, assign26330_e24889_d_n5, assign26330_e24889_d_n6, assign26330_e24889_d_n7, assign26330_e24889_d_n8, assign26330_e24889_d_n9, assign26330_e24889_d_n10, assign26330_e24889_d_n11, assign26330_e24889_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26330_e24887: f64 = (locals.var_xp * locals.var_x2);
        (assign26330_e24887, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26330_e24889;
        locals.var_xp_dn0 = assign26330_e24889_d_n0;
        locals.var_xp_dn2 = assign26330_e24889_d_n2;
        locals.var_xp_dn4 = assign26330_e24889_d_n4;
        locals.var_xp_dn5 = assign26330_e24889_d_n5;
        locals.var_xp_dn6 = assign26330_e24889_d_n6;
        locals.var_xp_dn7 = assign26330_e24889_d_n7;
        locals.var_xp_dn8 = assign26330_e24889_d_n8;
        locals.var_xp_dn9 = assign26330_e24889_d_n9;
        locals.var_xp_dn10 = assign26330_e24889_d_n10;
        locals.var_xp_dn11 = assign26330_e24889_d_n11;
        locals.var_xp_dn14 = assign26330_e24889_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26340_e24902, assign26340_e24902_d_n0, assign26340_e24902_d_n2, assign26340_e24902_d_n4, assign26340_e24902_d_n5, assign26340_e24902_d_n6, assign26340_e24902_d_n7, assign26340_e24902_d_n8, assign26340_e24902_d_n9, assign26340_e24902_d_n10, assign26340_e24902_d_n11, assign26340_e24902_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26340_e24900: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26340_e24900, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26340_e24902;
        locals.var_xmp_dn0 = assign26340_e24902_d_n0;
        locals.var_xmp_dn2 = assign26340_e24902_d_n2;
        locals.var_xmp_dn4 = assign26340_e24902_d_n4;
        locals.var_xmp_dn5 = assign26340_e24902_d_n5;
        locals.var_xmp_dn6 = assign26340_e24902_d_n6;
        locals.var_xmp_dn7 = assign26340_e24902_d_n7;
        locals.var_xmp_dn8 = assign26340_e24902_d_n8;
        locals.var_xmp_dn9 = assign26340_e24902_d_n9;
        locals.var_xmp_dn10 = assign26340_e24902_d_n10;
        locals.var_xmp_dn11 = assign26340_e24902_d_n11;
        locals.var_xmp_dn14 = assign26340_e24902_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign26350_e24915, assign26350_e24915_d_n0, assign26350_e24915_d_n2, assign26350_e24915_d_n4, assign26350_e24915_d_n5, assign26350_e24915_d_n6, assign26350_e24915_d_n7, assign26350_e24915_d_n8, assign26350_e24915_d_n9, assign26350_e24915_d_n10, assign26350_e24915_d_n11, assign26350_e24915_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26350_e24913: f64 = (locals.var_xp * locals.var_x2);
        (assign26350_e24913, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26350_e24915;
        locals.var_xp_dn0 = assign26350_e24915_d_n0;
        locals.var_xp_dn2 = assign26350_e24915_d_n2;
        locals.var_xp_dn4 = assign26350_e24915_d_n4;
        locals.var_xp_dn5 = assign26350_e24915_d_n5;
        locals.var_xp_dn6 = assign26350_e24915_d_n6;
        locals.var_xp_dn7 = assign26350_e24915_d_n7;
        locals.var_xp_dn8 = assign26350_e24915_d_n8;
        locals.var_xp_dn9 = assign26350_e24915_d_n9;
        locals.var_xp_dn10 = assign26350_e24915_d_n10;
        locals.var_xp_dn11 = assign26350_e24915_d_n11;
        locals.var_xp_dn14 = assign26350_e24915_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26360_e24928, assign26360_e24928_d_n0, assign26360_e24928_d_n2, assign26360_e24928_d_n4, assign26360_e24928_d_n5, assign26360_e24928_d_n6, assign26360_e24928_d_n7, assign26360_e24928_d_n8, assign26360_e24928_d_n9, assign26360_e24928_d_n10, assign26360_e24928_d_n11, assign26360_e24928_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26360_e24926: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26360_e24926, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26360_e24928;
        locals.var_xmp_dn0 = assign26360_e24928_d_n0;
        locals.var_xmp_dn2 = assign26360_e24928_d_n2;
        locals.var_xmp_dn4 = assign26360_e24928_d_n4;
        locals.var_xmp_dn5 = assign26360_e24928_d_n5;
        locals.var_xmp_dn6 = assign26360_e24928_d_n6;
        locals.var_xmp_dn7 = assign26360_e24928_d_n7;
        locals.var_xmp_dn8 = assign26360_e24928_d_n8;
        locals.var_xmp_dn9 = assign26360_e24928_d_n9;
        locals.var_xmp_dn10 = assign26360_e24928_d_n10;
        locals.var_xmp_dn11 = assign26360_e24928_d_n11;
        locals.var_xmp_dn14 = assign26360_e24928_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_79(
        locals: &mut StampLocals,
    ) {
        let (assign26370_e24941, assign26370_e24941_d_n0, assign26370_e24941_d_n2, assign26370_e24941_d_n4, assign26370_e24941_d_n5, assign26370_e24941_d_n6, assign26370_e24941_d_n7, assign26370_e24941_d_n8, assign26370_e24941_d_n9, assign26370_e24941_d_n10, assign26370_e24941_d_n11, assign26370_e24941_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26370_e24939: f64 = (locals.var_xp + locals.var_xmp);
        (assign26370_e24939, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26370_e24941;
        locals.var_arg_dn0 = assign26370_e24941_d_n0;
        locals.var_arg_dn2 = assign26370_e24941_d_n2;
        locals.var_arg_dn4 = assign26370_e24941_d_n4;
        locals.var_arg_dn5 = assign26370_e24941_d_n5;
        locals.var_arg_dn6 = assign26370_e24941_d_n6;
        locals.var_arg_dn7 = assign26370_e24941_d_n7;
        locals.var_arg_dn8 = assign26370_e24941_d_n8;
        locals.var_arg_dn9 = assign26370_e24941_d_n9;
        locals.var_arg_dn10 = assign26370_e24941_d_n10;
        locals.var_arg_dn11 = assign26370_e24941_d_n11;
        locals.var_arg_dn14 = assign26370_e24941_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign26380_e24952, assign26380_e24952_d_n0, assign26380_e24952_d_n2, assign26380_e24952_d_n4, assign26380_e24952_d_n5, assign26380_e24952_d_n6, assign26380_e24952_d_n7, assign26380_e24952_d_n8, assign26380_e24952_d_n9, assign26380_e24952_d_n10, assign26380_e24952_d_n11, assign26380_e24952_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26380_e24952;
        locals.var_dnm_dn0 = assign26380_e24952_d_n0;
        locals.var_dnm_dn2 = assign26380_e24952_d_n2;
        locals.var_dnm_dn4 = assign26380_e24952_d_n4;
        locals.var_dnm_dn5 = assign26380_e24952_d_n5;
        locals.var_dnm_dn6 = assign26380_e24952_d_n6;
        locals.var_dnm_dn7 = assign26380_e24952_d_n7;
        locals.var_dnm_dn8 = assign26380_e24952_d_n8;
        locals.var_dnm_dn9 = assign26380_e24952_d_n9;
        locals.var_dnm_dn10 = assign26380_e24952_d_n10;
        locals.var_dnm_dn11 = assign26380_e24952_d_n11;
        locals.var_dnm_dn14 = assign26380_e24952_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign26390_e24967: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard634 = assign26390_e24967;
        locals.var_guard634_rv = 0.0;

        let assign26400_e24970: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign26400_e24970;
        locals.var_guard635_rv = 0.0;

        let (assign26410_e24985,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26410_e24985;
        locals.var_mm_rv = 0.0;

        let assign26420_e24988: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign26420_e24988;
        locals.var_guard636_rv = 0.0;

        let (assign26430_e25006,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26430_e25006;
        locals.var_mm_rv = 0.0;

        let assign26440_e25009: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign26440_e25009;
        locals.var_guard637_rv = 0.0;

        let (assign26450_e25030,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 == 0.0)) && (locals.var_guard637 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26450_e25030;
        locals.var_mm_rv = 0.0;

        let assign26460_e25033: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign26460_e25033;
        locals.var_guard638_rv = 0.0;

        let (assign26470_e25057,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_guard635 == 0.0)) && (locals.var_guard636 == 0.0)) && (locals.var_guard637 == 0.0)) && (locals.var_guard638 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26470_e25057;
        locals.var_mm_rv = 0.0;

        let (assign26480_e25070,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26480_e25070;
        locals.var_m0_rv = 0.0;

        let mut assign26490_loop_guard: usize = 0;
        while {
            let assign26490_cond_e25084: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign26490_cond_e25084 != 0.0
        } {
            assign26490_loop_guard += 1;
            assert!(assign26490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26490_body0_e25098, assign26490_body0_e25098_d_n0, assign26490_body0_e25098_d_n2, assign26490_body0_e25098_d_n4, assign26490_body0_e25098_d_n5, assign26490_body0_e25098_d_n6, assign26490_body0_e25098_d_n7, assign26490_body0_e25098_d_n8, assign26490_body0_e25098_d_n9, assign26490_body0_e25098_d_n10, assign26490_body0_e25098_d_n11, assign26490_body0_e25098_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign26490_body0_e25096: f64 = (locals.var_dnm).sqrt();
        (assign26490_body0_e25096, (locals.var_dnm_dn0 / (2.0 * assign26490_body0_e25096)), (locals.var_dnm_dn2 / (2.0 * assign26490_body0_e25096)), (locals.var_dnm_dn4 / (2.0 * assign26490_body0_e25096)), (locals.var_dnm_dn5 / (2.0 * assign26490_body0_e25096)), (locals.var_dnm_dn6 / (2.0 * assign26490_body0_e25096)), (locals.var_dnm_dn7 / (2.0 * assign26490_body0_e25096)), (locals.var_dnm_dn8 / (2.0 * assign26490_body0_e25096)), (locals.var_dnm_dn9 / (2.0 * assign26490_body0_e25096)), (locals.var_dnm_dn10 / (2.0 * assign26490_body0_e25096)), (locals.var_dnm_dn11 / (2.0 * assign26490_body0_e25096)), (locals.var_dnm_dn14 / (2.0 * assign26490_body0_e25096)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign26490_body0_e25098;
            locals.var_dnm_dn0 = assign26490_body0_e25098_d_n0;
            locals.var_dnm_dn2 = assign26490_body0_e25098_d_n2;
            locals.var_dnm_dn4 = assign26490_body0_e25098_d_n4;
            locals.var_dnm_dn5 = assign26490_body0_e25098_d_n5;
            locals.var_dnm_dn6 = assign26490_body0_e25098_d_n6;
            locals.var_dnm_dn7 = assign26490_body0_e25098_d_n7;
            locals.var_dnm_dn8 = assign26490_body0_e25098_d_n8;
            locals.var_dnm_dn9 = assign26490_body0_e25098_d_n9;
            locals.var_dnm_dn10 = assign26490_body0_e25098_d_n10;
            locals.var_dnm_dn11 = assign26490_body0_e25098_d_n11;
            locals.var_dnm_dn14 = assign26490_body0_e25098_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign26490_body1_e25113,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign26490_body1_e25111: f64 = (locals.var_m0 + 1.0);
        (assign26490_body1_e25111,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign26490_body1_e25113;
            locals.var_m0_rv = 0.0;
        }

        let (assign26500_e25138, assign26500_e25138_d_n0, assign26500_e25138_d_n2, assign26500_e25138_d_n4, assign26500_e25138_d_n5, assign26500_e25138_d_n6, assign26500_e25138_d_n7, assign26500_e25138_d_n8, assign26500_e25138_d_n9, assign26500_e25138_d_n10, assign26500_e25138_d_n11, assign26500_e25138_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 == 0.0)) {
        let (assign26500_e25136, assign26500_e25136_d_n0, assign26500_e25136_d_n2, assign26500_e25136_d_n4, assign26500_e25136_d_n5, assign26500_e25136_d_n6, assign26500_e25136_d_n7, assign26500_e25136_d_n8, assign26500_e25136_d_n9, assign26500_e25136_d_n10, assign26500_e25136_d_n11, assign26500_e25136_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26500_e25133: f64 = (2.0 * 2.0);
                let assign26500_e25134: f64 = (1.0 / assign26500_e25133);
                let assign26500_e25135: f64 = (locals.var_dnm).powf(assign26500_e25134);
                (assign26500_e25135, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn0)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn2)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn4)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn5)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn6)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn7)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn8)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn9)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn10)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn11)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26500_e25134) as f64).is_finite() && ((assign26500_e25134) as f64).fract() == 0.0 { if assign26500_e25134 == 0.0 { 0.0 } else { (assign26500_e25134 * ((locals.var_dnm).powf(assign26500_e25134 - 1.0) * locals.var_dnm_dn14)) } } else { (assign26500_e25135 * (assign26500_e25134 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign26500_e25136, assign26500_e25136_d_n0, assign26500_e25136_d_n2, assign26500_e25136_d_n4, assign26500_e25136_d_n5, assign26500_e25136_d_n6, assign26500_e25136_d_n7, assign26500_e25136_d_n8, assign26500_e25136_d_n9, assign26500_e25136_d_n10, assign26500_e25136_d_n11, assign26500_e25136_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26500_e25138;
        locals.var_dnm_dn0 = assign26500_e25138_d_n0;
        locals.var_dnm_dn2 = assign26500_e25138_d_n2;
        locals.var_dnm_dn4 = assign26500_e25138_d_n4;
        locals.var_dnm_dn5 = assign26500_e25138_d_n5;
        locals.var_dnm_dn6 = assign26500_e25138_d_n6;
        locals.var_dnm_dn7 = assign26500_e25138_d_n7;
        locals.var_dnm_dn8 = assign26500_e25138_d_n8;
        locals.var_dnm_dn9 = assign26500_e25138_d_n9;
        locals.var_dnm_dn10 = assign26500_e25138_d_n10;
        locals.var_dnm_dn11 = assign26500_e25138_d_n11;
        locals.var_dnm_dn14 = assign26500_e25138_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26510_e25151, assign26510_e25151_d_n0, assign26510_e25151_d_n2, assign26510_e25151_d_n4, assign26510_e25151_d_n5, assign26510_e25151_d_n6, assign26510_e25151_d_n7, assign26510_e25151_d_n8, assign26510_e25151_d_n9, assign26510_e25151_d_n10, assign26510_e25151_d_n11, assign26510_e25151_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26510_e25149: f64 = (1.0 / locals.var_dnm);
        (assign26510_e25149, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26510_e25151;
        locals.var_dnm_dn0 = assign26510_e25151_d_n0;
        locals.var_dnm_dn2 = assign26510_e25151_d_n2;
        locals.var_dnm_dn4 = assign26510_e25151_d_n4;
        locals.var_dnm_dn5 = assign26510_e25151_d_n5;
        locals.var_dnm_dn6 = assign26510_e25151_d_n6;
        locals.var_dnm_dn7 = assign26510_e25151_d_n7;
        locals.var_dnm_dn8 = assign26510_e25151_d_n8;
        locals.var_dnm_dn9 = assign26510_e25151_d_n9;
        locals.var_dnm_dn10 = assign26510_e25151_d_n10;
        locals.var_dnm_dn11 = assign26510_e25151_d_n11;
        locals.var_dnm_dn14 = assign26510_e25151_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26520_e25166, assign26520_e25166_d_n0, assign26520_e25166_d_n2, assign26520_e25166_d_n4, assign26520_e25166_d_n5, assign26520_e25166_d_n6, assign26520_e25166_d_n7, assign26520_e25166_d_n8, assign26520_e25166_d_n9, assign26520_e25166_d_n10, assign26520_e25166_d_n11, assign26520_e25166_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26520_e25162: f64 = (locals.var_tmf1 * 0.1);
        let assign26520_e25164: f64 = (assign26520_e25162 * locals.var_dnm);
        (assign26520_e25164, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign26520_e25162 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign26520_e25166;
        locals.var_tmf0_dn0 = assign26520_e25166_d_n0;
        locals.var_tmf0_dn2 = assign26520_e25166_d_n2;
        locals.var_tmf0_dn4 = assign26520_e25166_d_n4;
        locals.var_tmf0_dn5 = assign26520_e25166_d_n5;
        locals.var_tmf0_dn6 = assign26520_e25166_d_n6;
        locals.var_tmf0_dn7 = assign26520_e25166_d_n7;
        locals.var_tmf0_dn8 = assign26520_e25166_d_n8;
        locals.var_tmf0_dn9 = assign26520_e25166_d_n9;
        locals.var_tmf0_dn10 = assign26520_e25166_d_n10;
        locals.var_tmf0_dn11 = assign26520_e25166_d_n11;
        locals.var_tmf0_dn14 = assign26520_e25166_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign26530_e25183, assign26530_e25183_d_n0, assign26530_e25183_d_n2, assign26530_e25183_d_n4, assign26530_e25183_d_n5, assign26530_e25183_d_n6, assign26530_e25183_d_n7, assign26530_e25183_d_n8, assign26530_e25183_d_n9, assign26530_e25183_d_n10, assign26530_e25183_d_n11, assign26530_e25183_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26530_e25177: f64 = (0.1 * locals.var_xmp);
        let assign26530_e25179: f64 = (assign26530_e25177 * locals.var_dnm);
        let assign26530_e25181: f64 = (assign26530_e25179 / locals.var_arg);
        (assign26530_e25181, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn0)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn2)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn4)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn5)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn6)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn7)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn8)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn9)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn10)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn11)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign26530_e25177 * locals.var_dnm_dn14)) * locals.var_arg) - (assign26530_e25179 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign26530_e25183;
        locals.var_t0_dn0 = assign26530_e25183_d_n0;
        locals.var_t0_dn2 = assign26530_e25183_d_n2;
        locals.var_t0_dn4 = assign26530_e25183_d_n4;
        locals.var_t0_dn5 = assign26530_e25183_d_n5;
        locals.var_t0_dn6 = assign26530_e25183_d_n6;
        locals.var_t0_dn7 = assign26530_e25183_d_n7;
        locals.var_t0_dn8 = assign26530_e25183_d_n8;
        locals.var_t0_dn9 = assign26530_e25183_d_n9;
        locals.var_t0_dn10 = assign26530_e25183_d_n10;
        locals.var_t0_dn11 = assign26530_e25183_d_n11;
        locals.var_t0_dn14 = assign26530_e25183_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign26540_e25198, assign26540_e25198_d_n0, assign26540_e25198_d_n2, assign26540_e25198_d_n4, assign26540_e25198_d_n5, assign26540_e25198_d_n6, assign26540_e25198_d_n7, assign26540_e25198_d_n8, assign26540_e25198_d_n9, assign26540_e25198_d_n10, assign26540_e25198_d_n11, assign26540_e25198_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        let assign26540_e25194: f64 = 0.1;
        let assign26540_e25196: f64 = (assign26540_e25194 - locals.var_tmf0);
        (assign26540_e25196, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign26540_e25198;
        locals.var_t2_dn0 = assign26540_e25198_d_n0;
        locals.var_t2_dn2 = assign26540_e25198_d_n2;
        locals.var_t2_dn4 = assign26540_e25198_d_n4;
        locals.var_t2_dn5 = assign26540_e25198_d_n5;
        locals.var_t2_dn6 = assign26540_e25198_d_n6;
        locals.var_t2_dn7 = assign26540_e25198_d_n7;
        locals.var_t2_dn8 = assign26540_e25198_d_n8;
        locals.var_t2_dn9 = assign26540_e25198_d_n9;
        locals.var_t2_dn10 = assign26540_e25198_d_n10;
        locals.var_t2_dn11 = assign26540_e25198_d_n11;
        locals.var_t2_dn14 = assign26540_e25198_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign26550_e25209, assign26550_e25209_d_n0, assign26550_e25209_d_n2, assign26550_e25209_d_n4, assign26550_e25209_d_n5, assign26550_e25209_d_n6, assign26550_e25209_d_n7, assign26550_e25209_d_n8, assign26550_e25209_d_n9, assign26550_e25209_d_n10, assign26550_e25209_d_n11, assign26550_e25209_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign26550_e25209;
        locals.var_t0_dn0 = assign26550_e25209_d_n0;
        locals.var_t0_dn2 = assign26550_e25209_d_n2;
        locals.var_t0_dn4 = assign26550_e25209_d_n4;
        locals.var_t0_dn5 = assign26550_e25209_d_n5;
        locals.var_t0_dn6 = assign26550_e25209_d_n6;
        locals.var_t0_dn7 = assign26550_e25209_d_n7;
        locals.var_t0_dn8 = assign26550_e25209_d_n8;
        locals.var_t0_dn9 = assign26550_e25209_d_n9;
        locals.var_t0_dn10 = assign26550_e25209_d_n10;
        locals.var_t0_dn11 = assign26550_e25209_d_n11;
        locals.var_t0_dn14 = assign26550_e25209_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign26560_e25221, assign26560_e25221_d_n0, assign26560_e25221_d_n2, assign26560_e25221_d_n4, assign26560_e25221_d_n5, assign26560_e25221_d_n6, assign26560_e25221_d_n7, assign26560_e25221_d_n8, assign26560_e25221_d_n9, assign26560_e25221_d_n10, assign26560_e25221_d_n11, assign26560_e25221_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign26560_e25221;
        locals.var_t2_dn0 = assign26560_e25221_d_n0;
        locals.var_t2_dn2 = assign26560_e25221_d_n2;
        locals.var_t2_dn4 = assign26560_e25221_d_n4;
        locals.var_t2_dn5 = assign26560_e25221_d_n5;
        locals.var_t2_dn6 = assign26560_e25221_d_n6;
        locals.var_t2_dn7 = assign26560_e25221_d_n7;
        locals.var_t2_dn8 = assign26560_e25221_d_n8;
        locals.var_t2_dn9 = assign26560_e25221_d_n9;
        locals.var_t2_dn10 = assign26560_e25221_d_n10;
        locals.var_t2_dn11 = assign26560_e25221_d_n11;
        locals.var_t2_dn14 = assign26560_e25221_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign26570_e25233, assign26570_e25233_d_n0, assign26570_e25233_d_n2, assign26570_e25233_d_n4, assign26570_e25233_d_n5, assign26570_e25233_d_n6, assign26570_e25233_d_n7, assign26570_e25233_d_n8, assign26570_e25233_d_n9, assign26570_e25233_d_n10, assign26570_e25233_d_n11, assign26570_e25233_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard633 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign26570_e25233;
        locals.var_t0_dn0 = assign26570_e25233_d_n0;
        locals.var_t0_dn2 = assign26570_e25233_d_n2;
        locals.var_t0_dn4 = assign26570_e25233_d_n4;
        locals.var_t0_dn5 = assign26570_e25233_d_n5;
        locals.var_t0_dn6 = assign26570_e25233_d_n6;
        locals.var_t0_dn7 = assign26570_e25233_d_n7;
        locals.var_t0_dn8 = assign26570_e25233_d_n8;
        locals.var_t0_dn9 = assign26570_e25233_d_n9;
        locals.var_t0_dn10 = assign26570_e25233_d_n10;
        locals.var_t0_dn11 = assign26570_e25233_d_n11;
        locals.var_t0_dn14 = assign26570_e25233_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign26580_e25245, assign26580_e25245_d_n0, assign26580_e25245_d_n2, assign26580_e25245_d_n4, assign26580_e25245_d_n5, assign26580_e25245_d_n6, assign26580_e25245_d_n7, assign26580_e25245_d_n8, assign26580_e25245_d_n9, assign26580_e25245_d_n10, assign26580_e25245_d_n11, assign26580_e25245_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) {
        let assign26580_e25242: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign26580_e25243: f64 = (assign26580_e25242).sqrt();
        (assign26580_e25243, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign26580_e25243)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign26580_e25243)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign26580_e25243)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign26580_e25243)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign26580_e25243)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign26580_e25243)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign26580_e25243)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign26580_e25243)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign26580_e25243)), (((locals.var_c_2esipq_ndepm_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn11)) / (2.0 * assign26580_e25243)), (((locals.var_c_2esipq_ndepm_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn14)) / (2.0 * assign26580_e25243)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign26580_e25245;
        locals.var_w_b0_dn0 = assign26580_e25245_d_n0;
        locals.var_w_b0_dn2 = assign26580_e25245_d_n2;
        locals.var_w_b0_dn4 = assign26580_e25245_d_n4;
        locals.var_w_b0_dn5 = assign26580_e25245_d_n5;
        locals.var_w_b0_dn6 = assign26580_e25245_d_n6;
        locals.var_w_b0_dn7 = assign26580_e25245_d_n7;
        locals.var_w_b0_dn8 = assign26580_e25245_d_n8;
        locals.var_w_b0_dn9 = assign26580_e25245_d_n9;
        locals.var_w_b0_dn10 = assign26580_e25245_d_n10;
        locals.var_w_b0_dn11 = assign26580_e25245_d_n11;
        locals.var_w_b0_dn14 = assign26580_e25245_d_n14;
        locals.var_w_b0_rv = 0.0;

        let assign26590_e25249: f64 = (locals.var_uc_depthn - 1e-8);
        let assign26590_e25254: f64 = if ((locals.var_w_b0 > assign26590_e25249) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard639 = assign26590_e25254;
        locals.var_guard639_rv = 0.0;

        let (assign26600_e25269, assign26600_e25269_d_n0, assign26600_e25269_d_n2, assign26600_e25269_d_n4, assign26600_e25269_d_n5, assign26600_e25269_d_n6, assign26600_e25269_d_n7, assign26600_e25269_d_n8, assign26600_e25269_d_n9, assign26600_e25269_d_n10, assign26600_e25269_d_n11, assign26600_e25269_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26600_e25265: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign26600_e25267: f64 = (assign26600_e25265 + 1e-8);
        (assign26600_e25267, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_b0_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign26600_e25269;
        locals.var_tmf1_dn0 = assign26600_e25269_d_n0;
        locals.var_tmf1_dn2 = assign26600_e25269_d_n2;
        locals.var_tmf1_dn4 = assign26600_e25269_d_n4;
        locals.var_tmf1_dn5 = assign26600_e25269_d_n5;
        locals.var_tmf1_dn6 = assign26600_e25269_d_n6;
        locals.var_tmf1_dn7 = assign26600_e25269_d_n7;
        locals.var_tmf1_dn8 = assign26600_e25269_d_n8;
        locals.var_tmf1_dn9 = assign26600_e25269_d_n9;
        locals.var_tmf1_dn10 = assign26600_e25269_d_n10;
        locals.var_tmf1_dn11 = assign26600_e25269_d_n11;
        locals.var_tmf1_dn14 = assign26600_e25269_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign26610_e25282, assign26610_e25282_d_n0, assign26610_e25282_d_n2, assign26610_e25282_d_n4, assign26610_e25282_d_n5, assign26610_e25282_d_n6, assign26610_e25282_d_n7, assign26610_e25282_d_n8, assign26610_e25282_d_n9, assign26610_e25282_d_n10, assign26610_e25282_d_n11, assign26610_e25282_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26610_e25280: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign26610_e25280, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign26610_e25282;
        locals.var_x2_dn0 = assign26610_e25282_d_n0;
        locals.var_x2_dn2 = assign26610_e25282_d_n2;
        locals.var_x2_dn4 = assign26610_e25282_d_n4;
        locals.var_x2_dn5 = assign26610_e25282_d_n5;
        locals.var_x2_dn6 = assign26610_e25282_d_n6;
        locals.var_x2_dn7 = assign26610_e25282_d_n7;
        locals.var_x2_dn8 = assign26610_e25282_d_n8;
        locals.var_x2_dn9 = assign26610_e25282_d_n9;
        locals.var_x2_dn10 = assign26610_e25282_d_n10;
        locals.var_x2_dn11 = assign26610_e25282_d_n11;
        locals.var_x2_dn14 = assign26610_e25282_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign26620_e25295, assign26620_e25295_d_n0, assign26620_e25295_d_n2, assign26620_e25295_d_n4, assign26620_e25295_d_n5, assign26620_e25295_d_n6, assign26620_e25295_d_n7, assign26620_e25295_d_n8, assign26620_e25295_d_n9, assign26620_e25295_d_n10, assign26620_e25295_d_n11, assign26620_e25295_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26620_e25293: f64 = (1e-8 * 1e-8);
        (assign26620_e25293, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign26620_e25295;
        locals.var_xmax2_dn0 = assign26620_e25295_d_n0;
        locals.var_xmax2_dn2 = assign26620_e25295_d_n2;
        locals.var_xmax2_dn4 = assign26620_e25295_d_n4;
        locals.var_xmax2_dn5 = assign26620_e25295_d_n5;
        locals.var_xmax2_dn6 = assign26620_e25295_d_n6;
        locals.var_xmax2_dn7 = assign26620_e25295_d_n7;
        locals.var_xmax2_dn8 = assign26620_e25295_d_n8;
        locals.var_xmax2_dn9 = assign26620_e25295_d_n9;
        locals.var_xmax2_dn10 = assign26620_e25295_d_n10;
        locals.var_xmax2_dn11 = assign26620_e25295_d_n11;
        locals.var_xmax2_dn14 = assign26620_e25295_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign26630_e25306, assign26630_e25306_d_n0, assign26630_e25306_d_n2, assign26630_e25306_d_n4, assign26630_e25306_d_n5, assign26630_e25306_d_n6, assign26630_e25306_d_n7, assign26630_e25306_d_n8, assign26630_e25306_d_n9, assign26630_e25306_d_n10, assign26630_e25306_d_n11, assign26630_e25306_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26630_e25306;
        locals.var_xp_dn0 = assign26630_e25306_d_n0;
        locals.var_xp_dn2 = assign26630_e25306_d_n2;
        locals.var_xp_dn4 = assign26630_e25306_d_n4;
        locals.var_xp_dn5 = assign26630_e25306_d_n5;
        locals.var_xp_dn6 = assign26630_e25306_d_n6;
        locals.var_xp_dn7 = assign26630_e25306_d_n7;
        locals.var_xp_dn8 = assign26630_e25306_d_n8;
        locals.var_xp_dn9 = assign26630_e25306_d_n9;
        locals.var_xp_dn10 = assign26630_e25306_d_n10;
        locals.var_xp_dn11 = assign26630_e25306_d_n11;
        locals.var_xp_dn14 = assign26630_e25306_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26640_e25317, assign26640_e25317_d_n0, assign26640_e25317_d_n2, assign26640_e25317_d_n4, assign26640_e25317_d_n5, assign26640_e25317_d_n6, assign26640_e25317_d_n7, assign26640_e25317_d_n8, assign26640_e25317_d_n9, assign26640_e25317_d_n10, assign26640_e25317_d_n11, assign26640_e25317_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26640_e25317;
        locals.var_xmp_dn0 = assign26640_e25317_d_n0;
        locals.var_xmp_dn2 = assign26640_e25317_d_n2;
        locals.var_xmp_dn4 = assign26640_e25317_d_n4;
        locals.var_xmp_dn5 = assign26640_e25317_d_n5;
        locals.var_xmp_dn6 = assign26640_e25317_d_n6;
        locals.var_xmp_dn7 = assign26640_e25317_d_n7;
        locals.var_xmp_dn8 = assign26640_e25317_d_n8;
        locals.var_xmp_dn9 = assign26640_e25317_d_n9;
        locals.var_xmp_dn10 = assign26640_e25317_d_n10;
        locals.var_xmp_dn11 = assign26640_e25317_d_n11;
        locals.var_xmp_dn14 = assign26640_e25317_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign26650_e25328,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26650_e25328;
        locals.var_m0_rv = 0.0;

        let (assign26660_e25339,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26660_e25339;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_80(
        locals: &mut StampLocals,
    ) {
        let (assign26670_e25350, assign26670_e25350_d_n0, assign26670_e25350_d_n2, assign26670_e25350_d_n4, assign26670_e25350_d_n5, assign26670_e25350_d_n6, assign26670_e25350_d_n7, assign26670_e25350_d_n8, assign26670_e25350_d_n9, assign26670_e25350_d_n10, assign26670_e25350_d_n11, assign26670_e25350_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26670_e25350;
        locals.var_arg_dn0 = assign26670_e25350_d_n0;
        locals.var_arg_dn2 = assign26670_e25350_d_n2;
        locals.var_arg_dn4 = assign26670_e25350_d_n4;
        locals.var_arg_dn5 = assign26670_e25350_d_n5;
        locals.var_arg_dn6 = assign26670_e25350_d_n6;
        locals.var_arg_dn7 = assign26670_e25350_d_n7;
        locals.var_arg_dn8 = assign26670_e25350_d_n8;
        locals.var_arg_dn9 = assign26670_e25350_d_n9;
        locals.var_arg_dn10 = assign26670_e25350_d_n10;
        locals.var_arg_dn11 = assign26670_e25350_d_n11;
        locals.var_arg_dn14 = assign26670_e25350_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign26680_e25361, assign26680_e25361_d_n0, assign26680_e25361_d_n2, assign26680_e25361_d_n4, assign26680_e25361_d_n5, assign26680_e25361_d_n6, assign26680_e25361_d_n7, assign26680_e25361_d_n8, assign26680_e25361_d_n9, assign26680_e25361_d_n10, assign26680_e25361_d_n11, assign26680_e25361_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26680_e25361;
        locals.var_dnm_dn0 = assign26680_e25361_d_n0;
        locals.var_dnm_dn2 = assign26680_e25361_d_n2;
        locals.var_dnm_dn4 = assign26680_e25361_d_n4;
        locals.var_dnm_dn5 = assign26680_e25361_d_n5;
        locals.var_dnm_dn6 = assign26680_e25361_d_n6;
        locals.var_dnm_dn7 = assign26680_e25361_d_n7;
        locals.var_dnm_dn8 = assign26680_e25361_d_n8;
        locals.var_dnm_dn9 = assign26680_e25361_d_n9;
        locals.var_dnm_dn10 = assign26680_e25361_d_n10;
        locals.var_dnm_dn11 = assign26680_e25361_d_n11;
        locals.var_dnm_dn14 = assign26680_e25361_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26690_e25374, assign26690_e25374_d_n0, assign26690_e25374_d_n2, assign26690_e25374_d_n4, assign26690_e25374_d_n5, assign26690_e25374_d_n6, assign26690_e25374_d_n7, assign26690_e25374_d_n8, assign26690_e25374_d_n9, assign26690_e25374_d_n10, assign26690_e25374_d_n11, assign26690_e25374_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26690_e25372: f64 = (locals.var_xp * locals.var_x2);
        (assign26690_e25372, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26690_e25374;
        locals.var_xp_dn0 = assign26690_e25374_d_n0;
        locals.var_xp_dn2 = assign26690_e25374_d_n2;
        locals.var_xp_dn4 = assign26690_e25374_d_n4;
        locals.var_xp_dn5 = assign26690_e25374_d_n5;
        locals.var_xp_dn6 = assign26690_e25374_d_n6;
        locals.var_xp_dn7 = assign26690_e25374_d_n7;
        locals.var_xp_dn8 = assign26690_e25374_d_n8;
        locals.var_xp_dn9 = assign26690_e25374_d_n9;
        locals.var_xp_dn10 = assign26690_e25374_d_n10;
        locals.var_xp_dn11 = assign26690_e25374_d_n11;
        locals.var_xp_dn14 = assign26690_e25374_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26700_e25387, assign26700_e25387_d_n0, assign26700_e25387_d_n2, assign26700_e25387_d_n4, assign26700_e25387_d_n5, assign26700_e25387_d_n6, assign26700_e25387_d_n7, assign26700_e25387_d_n8, assign26700_e25387_d_n9, assign26700_e25387_d_n10, assign26700_e25387_d_n11, assign26700_e25387_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26700_e25385: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26700_e25385, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26700_e25387;
        locals.var_xmp_dn0 = assign26700_e25387_d_n0;
        locals.var_xmp_dn2 = assign26700_e25387_d_n2;
        locals.var_xmp_dn4 = assign26700_e25387_d_n4;
        locals.var_xmp_dn5 = assign26700_e25387_d_n5;
        locals.var_xmp_dn6 = assign26700_e25387_d_n6;
        locals.var_xmp_dn7 = assign26700_e25387_d_n7;
        locals.var_xmp_dn8 = assign26700_e25387_d_n8;
        locals.var_xmp_dn9 = assign26700_e25387_d_n9;
        locals.var_xmp_dn10 = assign26700_e25387_d_n10;
        locals.var_xmp_dn11 = assign26700_e25387_d_n11;
        locals.var_xmp_dn14 = assign26700_e25387_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign26710_e25400, assign26710_e25400_d_n0, assign26710_e25400_d_n2, assign26710_e25400_d_n4, assign26710_e25400_d_n5, assign26710_e25400_d_n6, assign26710_e25400_d_n7, assign26710_e25400_d_n8, assign26710_e25400_d_n9, assign26710_e25400_d_n10, assign26710_e25400_d_n11, assign26710_e25400_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26710_e25398: f64 = (locals.var_xp * locals.var_x2);
        (assign26710_e25398, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign26710_e25400;
        locals.var_xp_dn0 = assign26710_e25400_d_n0;
        locals.var_xp_dn2 = assign26710_e25400_d_n2;
        locals.var_xp_dn4 = assign26710_e25400_d_n4;
        locals.var_xp_dn5 = assign26710_e25400_d_n5;
        locals.var_xp_dn6 = assign26710_e25400_d_n6;
        locals.var_xp_dn7 = assign26710_e25400_d_n7;
        locals.var_xp_dn8 = assign26710_e25400_d_n8;
        locals.var_xp_dn9 = assign26710_e25400_d_n9;
        locals.var_xp_dn10 = assign26710_e25400_d_n10;
        locals.var_xp_dn11 = assign26710_e25400_d_n11;
        locals.var_xp_dn14 = assign26710_e25400_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign26720_e25413, assign26720_e25413_d_n0, assign26720_e25413_d_n2, assign26720_e25413_d_n4, assign26720_e25413_d_n5, assign26720_e25413_d_n6, assign26720_e25413_d_n7, assign26720_e25413_d_n8, assign26720_e25413_d_n9, assign26720_e25413_d_n10, assign26720_e25413_d_n11, assign26720_e25413_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26720_e25411: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign26720_e25411, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign26720_e25413;
        locals.var_xmp_dn0 = assign26720_e25413_d_n0;
        locals.var_xmp_dn2 = assign26720_e25413_d_n2;
        locals.var_xmp_dn4 = assign26720_e25413_d_n4;
        locals.var_xmp_dn5 = assign26720_e25413_d_n5;
        locals.var_xmp_dn6 = assign26720_e25413_d_n6;
        locals.var_xmp_dn7 = assign26720_e25413_d_n7;
        locals.var_xmp_dn8 = assign26720_e25413_d_n8;
        locals.var_xmp_dn9 = assign26720_e25413_d_n9;
        locals.var_xmp_dn10 = assign26720_e25413_d_n10;
        locals.var_xmp_dn11 = assign26720_e25413_d_n11;
        locals.var_xmp_dn14 = assign26720_e25413_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign26730_e25426, assign26730_e25426_d_n0, assign26730_e25426_d_n2, assign26730_e25426_d_n4, assign26730_e25426_d_n5, assign26730_e25426_d_n6, assign26730_e25426_d_n7, assign26730_e25426_d_n8, assign26730_e25426_d_n9, assign26730_e25426_d_n10, assign26730_e25426_d_n11, assign26730_e25426_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26730_e25424: f64 = (locals.var_xp + locals.var_xmp);
        (assign26730_e25424, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26730_e25426;
        locals.var_arg_dn0 = assign26730_e25426_d_n0;
        locals.var_arg_dn2 = assign26730_e25426_d_n2;
        locals.var_arg_dn4 = assign26730_e25426_d_n4;
        locals.var_arg_dn5 = assign26730_e25426_d_n5;
        locals.var_arg_dn6 = assign26730_e25426_d_n6;
        locals.var_arg_dn7 = assign26730_e25426_d_n7;
        locals.var_arg_dn8 = assign26730_e25426_d_n8;
        locals.var_arg_dn9 = assign26730_e25426_d_n9;
        locals.var_arg_dn10 = assign26730_e25426_d_n10;
        locals.var_arg_dn11 = assign26730_e25426_d_n11;
        locals.var_arg_dn14 = assign26730_e25426_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign26740_e25437, assign26740_e25437_d_n0, assign26740_e25437_d_n2, assign26740_e25437_d_n4, assign26740_e25437_d_n5, assign26740_e25437_d_n6, assign26740_e25437_d_n7, assign26740_e25437_d_n8, assign26740_e25437_d_n9, assign26740_e25437_d_n10, assign26740_e25437_d_n11, assign26740_e25437_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26740_e25437;
        locals.var_dnm_dn0 = assign26740_e25437_d_n0;
        locals.var_dnm_dn2 = assign26740_e25437_d_n2;
        locals.var_dnm_dn4 = assign26740_e25437_d_n4;
        locals.var_dnm_dn5 = assign26740_e25437_d_n5;
        locals.var_dnm_dn6 = assign26740_e25437_d_n6;
        locals.var_dnm_dn7 = assign26740_e25437_d_n7;
        locals.var_dnm_dn8 = assign26740_e25437_d_n8;
        locals.var_dnm_dn9 = assign26740_e25437_d_n9;
        locals.var_dnm_dn10 = assign26740_e25437_d_n10;
        locals.var_dnm_dn11 = assign26740_e25437_d_n11;
        locals.var_dnm_dn14 = assign26740_e25437_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign26750_e25452: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard640 = assign26750_e25452;
        locals.var_guard640_rv = 0.0;

        let assign26760_e25455: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard641 = assign26760_e25455;
        locals.var_guard641_rv = 0.0;

        let (assign26770_e25470,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) && (locals.var_guard640 != 0.0)) && (locals.var_guard641 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26770_e25470;
        locals.var_mm_rv = 0.0;

        let assign26780_e25473: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign26780_e25473;
        locals.var_guard642_rv = 0.0;

        let (assign26790_e25491,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) && (locals.var_guard640 != 0.0)) && (locals.var_guard641 == 0.0)) && (locals.var_guard642 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26790_e25491;
        locals.var_mm_rv = 0.0;

        let assign26800_e25494: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard643 = assign26800_e25494;
        locals.var_guard643_rv = 0.0;

        let (assign26810_e25515,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) && (locals.var_guard640 != 0.0)) && (locals.var_guard641 == 0.0)) && (locals.var_guard642 == 0.0)) && (locals.var_guard643 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26810_e25515;
        locals.var_mm_rv = 0.0;

        let assign26820_e25518: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard644 = assign26820_e25518;
        locals.var_guard644_rv = 0.0;

        let (assign26830_e25542,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) && (locals.var_guard640 != 0.0)) && (locals.var_guard641 == 0.0)) && (locals.var_guard642 == 0.0)) && (locals.var_guard643 == 0.0)) && (locals.var_guard644 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign26830_e25542;
        locals.var_mm_rv = 0.0;

        let (assign26840_e25555,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) && (locals.var_guard640 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign26840_e25555;
        locals.var_m0_rv = 0.0;

        let mut assign26850_loop_guard: usize = 0;
        while {
            let assign26850_cond_e25569: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) && (locals.var_guard640 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign26850_cond_e25569 != 0.0
        } {
            assign26850_loop_guard += 1;
            assert!(assign26850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26850_body0_e25583, assign26850_body0_e25583_d_n0, assign26850_body0_e25583_d_n2, assign26850_body0_e25583_d_n4, assign26850_body0_e25583_d_n5, assign26850_body0_e25583_d_n6, assign26850_body0_e25583_d_n7, assign26850_body0_e25583_d_n8, assign26850_body0_e25583_d_n9, assign26850_body0_e25583_d_n10, assign26850_body0_e25583_d_n11, assign26850_body0_e25583_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) && (locals.var_guard640 != 0.0)) {
        let assign26850_body0_e25581: f64 = (locals.var_dnm).sqrt();
        (assign26850_body0_e25581, (locals.var_dnm_dn0 / (2.0 * assign26850_body0_e25581)), (locals.var_dnm_dn2 / (2.0 * assign26850_body0_e25581)), (locals.var_dnm_dn4 / (2.0 * assign26850_body0_e25581)), (locals.var_dnm_dn5 / (2.0 * assign26850_body0_e25581)), (locals.var_dnm_dn6 / (2.0 * assign26850_body0_e25581)), (locals.var_dnm_dn7 / (2.0 * assign26850_body0_e25581)), (locals.var_dnm_dn8 / (2.0 * assign26850_body0_e25581)), (locals.var_dnm_dn9 / (2.0 * assign26850_body0_e25581)), (locals.var_dnm_dn10 / (2.0 * assign26850_body0_e25581)), (locals.var_dnm_dn11 / (2.0 * assign26850_body0_e25581)), (locals.var_dnm_dn14 / (2.0 * assign26850_body0_e25581)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign26850_body0_e25583;
            locals.var_dnm_dn0 = assign26850_body0_e25583_d_n0;
            locals.var_dnm_dn2 = assign26850_body0_e25583_d_n2;
            locals.var_dnm_dn4 = assign26850_body0_e25583_d_n4;
            locals.var_dnm_dn5 = assign26850_body0_e25583_d_n5;
            locals.var_dnm_dn6 = assign26850_body0_e25583_d_n6;
            locals.var_dnm_dn7 = assign26850_body0_e25583_d_n7;
            locals.var_dnm_dn8 = assign26850_body0_e25583_d_n8;
            locals.var_dnm_dn9 = assign26850_body0_e25583_d_n9;
            locals.var_dnm_dn10 = assign26850_body0_e25583_d_n10;
            locals.var_dnm_dn11 = assign26850_body0_e25583_d_n11;
            locals.var_dnm_dn14 = assign26850_body0_e25583_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign26850_body1_e25598,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) && (locals.var_guard640 != 0.0)) {
        let assign26850_body1_e25596: f64 = (locals.var_m0 + 1.0);
        (assign26850_body1_e25596,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign26850_body1_e25598;
            locals.var_m0_rv = 0.0;
        }

        let (assign26860_e25623, assign26860_e25623_d_n0, assign26860_e25623_d_n2, assign26860_e25623_d_n4, assign26860_e25623_d_n5, assign26860_e25623_d_n6, assign26860_e25623_d_n7, assign26860_e25623_d_n8, assign26860_e25623_d_n9, assign26860_e25623_d_n10, assign26860_e25623_d_n11, assign26860_e25623_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) && (locals.var_guard640 == 0.0)) {
        let (assign26860_e25621, assign26860_e25621_d_n0, assign26860_e25621_d_n2, assign26860_e25621_d_n4, assign26860_e25621_d_n5, assign26860_e25621_d_n6, assign26860_e25621_d_n7, assign26860_e25621_d_n8, assign26860_e25621_d_n9, assign26860_e25621_d_n10, assign26860_e25621_d_n11, assign26860_e25621_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26860_e25618: f64 = (2.0 * 2.0);
                let assign26860_e25619: f64 = (1.0 / assign26860_e25618);
                let assign26860_e25620: f64 = (locals.var_dnm).powf(assign26860_e25619);
                (assign26860_e25620, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn0)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn2)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn4)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn5)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn6)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn7)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn8)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn9)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn10)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn11)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign26860_e25619) as f64).is_finite() && ((assign26860_e25619) as f64).fract() == 0.0 { if assign26860_e25619 == 0.0 { 0.0 } else { (assign26860_e25619 * ((locals.var_dnm).powf(assign26860_e25619 - 1.0) * locals.var_dnm_dn14)) } } else { (assign26860_e25620 * (assign26860_e25619 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign26860_e25621, assign26860_e25621_d_n0, assign26860_e25621_d_n2, assign26860_e25621_d_n4, assign26860_e25621_d_n5, assign26860_e25621_d_n6, assign26860_e25621_d_n7, assign26860_e25621_d_n8, assign26860_e25621_d_n9, assign26860_e25621_d_n10, assign26860_e25621_d_n11, assign26860_e25621_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26860_e25623;
        locals.var_dnm_dn0 = assign26860_e25623_d_n0;
        locals.var_dnm_dn2 = assign26860_e25623_d_n2;
        locals.var_dnm_dn4 = assign26860_e25623_d_n4;
        locals.var_dnm_dn5 = assign26860_e25623_d_n5;
        locals.var_dnm_dn6 = assign26860_e25623_d_n6;
        locals.var_dnm_dn7 = assign26860_e25623_d_n7;
        locals.var_dnm_dn8 = assign26860_e25623_d_n8;
        locals.var_dnm_dn9 = assign26860_e25623_d_n9;
        locals.var_dnm_dn10 = assign26860_e25623_d_n10;
        locals.var_dnm_dn11 = assign26860_e25623_d_n11;
        locals.var_dnm_dn14 = assign26860_e25623_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26870_e25636, assign26870_e25636_d_n0, assign26870_e25636_d_n2, assign26870_e25636_d_n4, assign26870_e25636_d_n5, assign26870_e25636_d_n6, assign26870_e25636_d_n7, assign26870_e25636_d_n8, assign26870_e25636_d_n9, assign26870_e25636_d_n10, assign26870_e25636_d_n11, assign26870_e25636_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26870_e25634: f64 = (1.0 / locals.var_dnm);
        (assign26870_e25634, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign26870_e25636;
        locals.var_dnm_dn0 = assign26870_e25636_d_n0;
        locals.var_dnm_dn2 = assign26870_e25636_d_n2;
        locals.var_dnm_dn4 = assign26870_e25636_d_n4;
        locals.var_dnm_dn5 = assign26870_e25636_d_n5;
        locals.var_dnm_dn6 = assign26870_e25636_d_n6;
        locals.var_dnm_dn7 = assign26870_e25636_d_n7;
        locals.var_dnm_dn8 = assign26870_e25636_d_n8;
        locals.var_dnm_dn9 = assign26870_e25636_d_n9;
        locals.var_dnm_dn10 = assign26870_e25636_d_n10;
        locals.var_dnm_dn11 = assign26870_e25636_d_n11;
        locals.var_dnm_dn14 = assign26870_e25636_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign26880_e25651, assign26880_e25651_d_n0, assign26880_e25651_d_n2, assign26880_e25651_d_n4, assign26880_e25651_d_n5, assign26880_e25651_d_n6, assign26880_e25651_d_n7, assign26880_e25651_d_n8, assign26880_e25651_d_n9, assign26880_e25651_d_n10, assign26880_e25651_d_n11, assign26880_e25651_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26880_e25647: f64 = (locals.var_tmf1 * 1e-8);
        let assign26880_e25649: f64 = (assign26880_e25647 * locals.var_dnm);
        (assign26880_e25649, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign26880_e25647 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign26880_e25651;
        locals.var_tmf0_dn0 = assign26880_e25651_d_n0;
        locals.var_tmf0_dn2 = assign26880_e25651_d_n2;
        locals.var_tmf0_dn4 = assign26880_e25651_d_n4;
        locals.var_tmf0_dn5 = assign26880_e25651_d_n5;
        locals.var_tmf0_dn6 = assign26880_e25651_d_n6;
        locals.var_tmf0_dn7 = assign26880_e25651_d_n7;
        locals.var_tmf0_dn8 = assign26880_e25651_d_n8;
        locals.var_tmf0_dn9 = assign26880_e25651_d_n9;
        locals.var_tmf0_dn10 = assign26880_e25651_d_n10;
        locals.var_tmf0_dn11 = assign26880_e25651_d_n11;
        locals.var_tmf0_dn14 = assign26880_e25651_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign26890_e25668, assign26890_e25668_d_n0, assign26890_e25668_d_n2, assign26890_e25668_d_n4, assign26890_e25668_d_n5, assign26890_e25668_d_n6, assign26890_e25668_d_n7, assign26890_e25668_d_n8, assign26890_e25668_d_n9, assign26890_e25668_d_n10, assign26890_e25668_d_n11, assign26890_e25668_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26890_e25662: f64 = (1e-8 * locals.var_xmp);
        let assign26890_e25664: f64 = (assign26890_e25662 * locals.var_dnm);
        let assign26890_e25666: f64 = (assign26890_e25664 / locals.var_arg);
        (assign26890_e25666, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn0)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn2)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn4)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn5)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn6)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn7)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn8)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn9)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn10)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn11)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign26890_e25662 * locals.var_dnm_dn14)) * locals.var_arg) - (assign26890_e25664 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26890_e25668;
        locals.var_t3_dn0 = assign26890_e25668_d_n0;
        locals.var_t3_dn2 = assign26890_e25668_d_n2;
        locals.var_t3_dn4 = assign26890_e25668_d_n4;
        locals.var_t3_dn5 = assign26890_e25668_d_n5;
        locals.var_t3_dn6 = assign26890_e25668_d_n6;
        locals.var_t3_dn7 = assign26890_e25668_d_n7;
        locals.var_t3_dn8 = assign26890_e25668_d_n8;
        locals.var_t3_dn9 = assign26890_e25668_d_n9;
        locals.var_t3_dn10 = assign26890_e25668_d_n10;
        locals.var_t3_dn11 = assign26890_e25668_d_n11;
        locals.var_t3_dn14 = assign26890_e25668_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26900_e25683, assign26900_e25683_d_n0, assign26900_e25683_d_n2, assign26900_e25683_d_n4, assign26900_e25683_d_n5, assign26900_e25683_d_n6, assign26900_e25683_d_n7, assign26900_e25683_d_n8, assign26900_e25683_d_n9, assign26900_e25683_d_n10, assign26900_e25683_d_n11, assign26900_e25683_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        let assign26900_e25679: f64 = (locals.var_uc_depthn - 1e-8);
        let assign26900_e25681: f64 = (assign26900_e25679 + locals.var_tmf0);
        (assign26900_e25681, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign26900_e25683;
        locals.var_w_b0_dn0 = assign26900_e25683_d_n0;
        locals.var_w_b0_dn2 = assign26900_e25683_d_n2;
        locals.var_w_b0_dn4 = assign26900_e25683_d_n4;
        locals.var_w_b0_dn5 = assign26900_e25683_d_n5;
        locals.var_w_b0_dn6 = assign26900_e25683_d_n6;
        locals.var_w_b0_dn7 = assign26900_e25683_d_n7;
        locals.var_w_b0_dn8 = assign26900_e25683_d_n8;
        locals.var_w_b0_dn9 = assign26900_e25683_d_n9;
        locals.var_w_b0_dn10 = assign26900_e25683_d_n10;
        locals.var_w_b0_dn11 = assign26900_e25683_d_n11;
        locals.var_w_b0_dn14 = assign26900_e25683_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign26910_e25694, assign26910_e25694_d_n0, assign26910_e25694_d_n2, assign26910_e25694_d_n4, assign26910_e25694_d_n5, assign26910_e25694_d_n6, assign26910_e25694_d_n7, assign26910_e25694_d_n8, assign26910_e25694_d_n9, assign26910_e25694_d_n10, assign26910_e25694_d_n11, assign26910_e25694_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26910_e25694;
        locals.var_t3_dn0 = assign26910_e25694_d_n0;
        locals.var_t3_dn2 = assign26910_e25694_d_n2;
        locals.var_t3_dn4 = assign26910_e25694_d_n4;
        locals.var_t3_dn5 = assign26910_e25694_d_n5;
        locals.var_t3_dn6 = assign26910_e25694_d_n6;
        locals.var_t3_dn7 = assign26910_e25694_d_n7;
        locals.var_t3_dn8 = assign26910_e25694_d_n8;
        locals.var_t3_dn9 = assign26910_e25694_d_n9;
        locals.var_t3_dn10 = assign26910_e25694_d_n10;
        locals.var_t3_dn11 = assign26910_e25694_d_n11;
        locals.var_t3_dn14 = assign26910_e25694_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26920_e25706, assign26920_e25706_d_n0, assign26920_e25706_d_n2, assign26920_e25706_d_n4, assign26920_e25706_d_n5, assign26920_e25706_d_n6, assign26920_e25706_d_n7, assign26920_e25706_d_n8, assign26920_e25706_d_n9, assign26920_e25706_d_n10, assign26920_e25706_d_n11, assign26920_e25706_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign26920_e25706;
        locals.var_w_b0_dn0 = assign26920_e25706_d_n0;
        locals.var_w_b0_dn2 = assign26920_e25706_d_n2;
        locals.var_w_b0_dn4 = assign26920_e25706_d_n4;
        locals.var_w_b0_dn5 = assign26920_e25706_d_n5;
        locals.var_w_b0_dn6 = assign26920_e25706_d_n6;
        locals.var_w_b0_dn7 = assign26920_e25706_d_n7;
        locals.var_w_b0_dn8 = assign26920_e25706_d_n8;
        locals.var_w_b0_dn9 = assign26920_e25706_d_n9;
        locals.var_w_b0_dn10 = assign26920_e25706_d_n10;
        locals.var_w_b0_dn11 = assign26920_e25706_d_n11;
        locals.var_w_b0_dn14 = assign26920_e25706_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign26930_e25718, assign26930_e25718_d_n0, assign26930_e25718_d_n2, assign26930_e25718_d_n4, assign26930_e25718_d_n5, assign26930_e25718_d_n6, assign26930_e25718_d_n7, assign26930_e25718_d_n8, assign26930_e25718_d_n9, assign26930_e25718_d_n10, assign26930_e25718_d_n11, assign26930_e25718_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) && (locals.var_guard639 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign26930_e25718;
        locals.var_t3_dn0 = assign26930_e25718_d_n0;
        locals.var_t3_dn2 = assign26930_e25718_d_n2;
        locals.var_t3_dn4 = assign26930_e25718_d_n4;
        locals.var_t3_dn5 = assign26930_e25718_d_n5;
        locals.var_t3_dn6 = assign26930_e25718_d_n6;
        locals.var_t3_dn7 = assign26930_e25718_d_n7;
        locals.var_t3_dn8 = assign26930_e25718_d_n8;
        locals.var_t3_dn9 = assign26930_e25718_d_n9;
        locals.var_t3_dn10 = assign26930_e25718_d_n10;
        locals.var_t3_dn11 = assign26930_e25718_d_n11;
        locals.var_t3_dn14 = assign26930_e25718_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign26940_e25734, assign26940_e25734_d_n0, assign26940_e25734_d_n2, assign26940_e25734_d_n4, assign26940_e25734_d_n5, assign26940_e25734_d_n6, assign26940_e25734_d_n7, assign26940_e25734_d_n8, assign26940_e25734_d_n9, assign26940_e25734_d_n10, assign26940_e25734_d_n11, assign26940_e25734_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) {
        let assign26940_e25728: f64 = (locals.var_phi_j0_dep - locals.var_vbscl__blk437);
        let assign26940_e25730: f64 = (assign26940_e25728 + locals.var_vbi_dep);
        let assign26940_e25731: f64 = (locals.var_c_2esipq_nsub * assign26940_e25730);
        let assign26940_e25732: f64 = (assign26940_e25731).sqrt();
        (assign26940_e25732, (((locals.var_c_2esipq_nsub_dn0 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn0 - locals.var_vbscl__blk437_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign26940_e25732)), (((locals.var_c_2esipq_nsub_dn2 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn2 - locals.var_vbscl__blk437_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign26940_e25732)), (((locals.var_c_2esipq_nsub_dn4 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn4 - locals.var_vbscl__blk437_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign26940_e25732)), (((locals.var_c_2esipq_nsub_dn5 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn5 - locals.var_vbscl__blk437_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign26940_e25732)), (((locals.var_c_2esipq_nsub_dn6 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn6 - locals.var_vbscl__blk437_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign26940_e25732)), (((locals.var_c_2esipq_nsub_dn7 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn7 - locals.var_vbscl__blk437_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign26940_e25732)), (((locals.var_c_2esipq_nsub_dn8 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn8 - locals.var_vbscl__blk437_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign26940_e25732)), (((locals.var_c_2esipq_nsub_dn9 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn9 - locals.var_vbscl__blk437_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign26940_e25732)), (((locals.var_c_2esipq_nsub_dn10 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn10 - locals.var_vbscl__blk437_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign26940_e25732)), (((locals.var_c_2esipq_nsub_dn11 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn11 - locals.var_vbscl__blk437_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign26940_e25732)), (((locals.var_c_2esipq_nsub_dn14 * assign26940_e25730) + (locals.var_c_2esipq_nsub * ((locals.var_phi_j0_dep_dn14 - locals.var_vbscl__blk437_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign26940_e25732)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn11, locals.var_w_sub0_dn14,)
    }
};
        locals.var_w_sub0 = assign26940_e25734;
        locals.var_w_sub0_dn0 = assign26940_e25734_d_n0;
        locals.var_w_sub0_dn2 = assign26940_e25734_d_n2;
        locals.var_w_sub0_dn4 = assign26940_e25734_d_n4;
        locals.var_w_sub0_dn5 = assign26940_e25734_d_n5;
        locals.var_w_sub0_dn6 = assign26940_e25734_d_n6;
        locals.var_w_sub0_dn7 = assign26940_e25734_d_n7;
        locals.var_w_sub0_dn8 = assign26940_e25734_d_n8;
        locals.var_w_sub0_dn9 = assign26940_e25734_d_n9;
        locals.var_w_sub0_dn10 = assign26940_e25734_d_n10;
        locals.var_w_sub0_dn11 = assign26940_e25734_d_n11;
        locals.var_w_sub0_dn14 = assign26940_e25734_d_n14;
        locals.var_w_sub0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_81(
        locals: &mut StampLocals,
    ) {
        let (assign26950_e25745, assign26950_e25745_d_n0, assign26950_e25745_d_n2, assign26950_e25745_d_n4, assign26950_e25745_d_n5, assign26950_e25745_d_n6, assign26950_e25745_d_n7, assign26950_e25745_d_n8, assign26950_e25745_d_n9, assign26950_e25745_d_n10, assign26950_e25745_d_n11, assign26950_e25745_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) {
        let assign26950_e25743: f64 = (locals.var_w_b0 * locals.var_q_ndepm);
        (assign26950_e25743, ((locals.var_w_b0_dn0 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn0)), ((locals.var_w_b0_dn2 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn2)), ((locals.var_w_b0_dn4 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn4)), ((locals.var_w_b0_dn5 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn5)), ((locals.var_w_b0_dn6 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn6)), ((locals.var_w_b0_dn7 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn7)), ((locals.var_w_b0_dn8 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn8)), ((locals.var_w_b0_dn9 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn9)), ((locals.var_w_b0_dn10 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn10)), ((locals.var_w_b0_dn11 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn11)), ((locals.var_w_b0_dn14 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn14,)
    }
};
        locals.var_q_b0_dep = assign26950_e25745;
        locals.var_q_b0_dep_dn0 = assign26950_e25745_d_n0;
        locals.var_q_b0_dep_dn2 = assign26950_e25745_d_n2;
        locals.var_q_b0_dep_dn4 = assign26950_e25745_d_n4;
        locals.var_q_b0_dep_dn5 = assign26950_e25745_d_n5;
        locals.var_q_b0_dep_dn6 = assign26950_e25745_d_n6;
        locals.var_q_b0_dep_dn7 = assign26950_e25745_d_n7;
        locals.var_q_b0_dep_dn8 = assign26950_e25745_d_n8;
        locals.var_q_b0_dep_dn9 = assign26950_e25745_d_n9;
        locals.var_q_b0_dep_dn10 = assign26950_e25745_d_n10;
        locals.var_q_b0_dep_dn11 = assign26950_e25745_d_n11;
        locals.var_q_b0_dep_dn14 = assign26950_e25745_d_n14;
        locals.var_q_b0_dep_rv = 0.0;

        let (assign26960_e25757, assign26960_e25757_d_n0, assign26960_e25757_d_n2, assign26960_e25757_d_n4, assign26960_e25757_d_n5, assign26960_e25757_d_n6, assign26960_e25757_d_n7, assign26960_e25757_d_n8, assign26960_e25757_d_n9, assign26960_e25757_d_n10, assign26960_e25757_d_n11, assign26960_e25757_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard625 == 0.0)) {
        let assign26960_e25753: f64 = (-locals.var_w_sub0);
        let assign26960_e25755: f64 = (assign26960_e25753 * locals.var_q_nsub__blk546);
        (assign26960_e25755, (((-locals.var_w_sub0_dn0) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn0)), (((-locals.var_w_sub0_dn2) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn2)), (((-locals.var_w_sub0_dn4) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn4)), (((-locals.var_w_sub0_dn5) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn5)), (((-locals.var_w_sub0_dn6) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn6)), (((-locals.var_w_sub0_dn7) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn7)), (((-locals.var_w_sub0_dn8) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn8)), (((-locals.var_w_sub0_dn9) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn9)), (((-locals.var_w_sub0_dn10) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn10)), (((-locals.var_w_sub0_dn11) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn11)), (((-locals.var_w_sub0_dn14) * locals.var_q_nsub__blk546) + (assign26960_e25753 * locals.var_q_nsub__blk546_dn14)),)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn11, locals.var_q_sub0_dep_dn14,)
    }
};
        locals.var_q_sub0_dep = assign26960_e25757;
        locals.var_q_sub0_dep_dn0 = assign26960_e25757_d_n0;
        locals.var_q_sub0_dep_dn2 = assign26960_e25757_d_n2;
        locals.var_q_sub0_dep_dn4 = assign26960_e25757_d_n4;
        locals.var_q_sub0_dep_dn5 = assign26960_e25757_d_n5;
        locals.var_q_sub0_dep_dn6 = assign26960_e25757_d_n6;
        locals.var_q_sub0_dep_dn7 = assign26960_e25757_d_n7;
        locals.var_q_sub0_dep_dn8 = assign26960_e25757_d_n8;
        locals.var_q_sub0_dep_dn9 = assign26960_e25757_d_n9;
        locals.var_q_sub0_dep_dn10 = assign26960_e25757_d_n10;
        locals.var_q_sub0_dep_dn11 = assign26960_e25757_d_n11;
        locals.var_q_sub0_dep_dn14 = assign26960_e25757_d_n14;
        locals.var_q_sub0_dep_rv = 0.0;

        let (assign26970_e25765, assign26970_e25765_d_n0, assign26970_e25765_d_n2, assign26970_e25765_d_n4, assign26970_e25765_d_n5, assign26970_e25765_d_n6, assign26970_e25765_d_n7, assign26970_e25765_d_n8, assign26970_e25765_d_n9, assign26970_e25765_d_n10, assign26970_e25765_d_n11, assign26970_e25765_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign26970_e25763: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        (assign26970_e25763, (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10), (locals.var_phi_b0_dep_dn11 - locals.var_phi_j0_dep_dn11), (locals.var_phi_b0_dep_dn14 - locals.var_phi_j0_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign26970_e25765;
        locals.var_t1_dn0 = assign26970_e25765_d_n0;
        locals.var_t1_dn2 = assign26970_e25765_d_n2;
        locals.var_t1_dn4 = assign26970_e25765_d_n4;
        locals.var_t1_dn5 = assign26970_e25765_d_n5;
        locals.var_t1_dn6 = assign26970_e25765_d_n6;
        locals.var_t1_dn7 = assign26970_e25765_d_n7;
        locals.var_t1_dn8 = assign26970_e25765_d_n8;
        locals.var_t1_dn9 = assign26970_e25765_d_n9;
        locals.var_t1_dn10 = assign26970_e25765_d_n10;
        locals.var_t1_dn11 = assign26970_e25765_d_n11;
        locals.var_t1_dn14 = assign26970_e25765_d_n14;
        locals.var_t1_rv = 0.0;

        let assign26980_e25769: f64 = 0.1;
        let assign26980_e25774: f64 = if ((locals.var_t1 < assign26980_e25769) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard645 = assign26980_e25774;
        locals.var_guard645_rv = 0.0;

        let (assign26990_e25786, assign26990_e25786_d_n0, assign26990_e25786_d_n2, assign26990_e25786_d_n4, assign26990_e25786_d_n5, assign26990_e25786_d_n6, assign26990_e25786_d_n7, assign26990_e25786_d_n8, assign26990_e25786_d_n9, assign26990_e25786_d_n10, assign26990_e25786_d_n11, assign26990_e25786_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign26990_e25782: f64 = 0.1;
        let assign26990_e25784: f64 = (assign26990_e25782 - locals.var_t1);
        (assign26990_e25784, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign26990_e25786;
        locals.var_tmf1_dn0 = assign26990_e25786_d_n0;
        locals.var_tmf1_dn2 = assign26990_e25786_d_n2;
        locals.var_tmf1_dn4 = assign26990_e25786_d_n4;
        locals.var_tmf1_dn5 = assign26990_e25786_d_n5;
        locals.var_tmf1_dn6 = assign26990_e25786_d_n6;
        locals.var_tmf1_dn7 = assign26990_e25786_d_n7;
        locals.var_tmf1_dn8 = assign26990_e25786_d_n8;
        locals.var_tmf1_dn9 = assign26990_e25786_d_n9;
        locals.var_tmf1_dn10 = assign26990_e25786_d_n10;
        locals.var_tmf1_dn11 = assign26990_e25786_d_n11;
        locals.var_tmf1_dn14 = assign26990_e25786_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign27000_e25796, assign27000_e25796_d_n0, assign27000_e25796_d_n2, assign27000_e25796_d_n4, assign27000_e25796_d_n5, assign27000_e25796_d_n6, assign27000_e25796_d_n7, assign27000_e25796_d_n8, assign27000_e25796_d_n9, assign27000_e25796_d_n10, assign27000_e25796_d_n11, assign27000_e25796_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27000_e25794: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign27000_e25794, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign27000_e25796;
        locals.var_x2_dn0 = assign27000_e25796_d_n0;
        locals.var_x2_dn2 = assign27000_e25796_d_n2;
        locals.var_x2_dn4 = assign27000_e25796_d_n4;
        locals.var_x2_dn5 = assign27000_e25796_d_n5;
        locals.var_x2_dn6 = assign27000_e25796_d_n6;
        locals.var_x2_dn7 = assign27000_e25796_d_n7;
        locals.var_x2_dn8 = assign27000_e25796_d_n8;
        locals.var_x2_dn9 = assign27000_e25796_d_n9;
        locals.var_x2_dn10 = assign27000_e25796_d_n10;
        locals.var_x2_dn11 = assign27000_e25796_d_n11;
        locals.var_x2_dn14 = assign27000_e25796_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign27010_e25806, assign27010_e25806_d_n0, assign27010_e25806_d_n2, assign27010_e25806_d_n4, assign27010_e25806_d_n5, assign27010_e25806_d_n6, assign27010_e25806_d_n7, assign27010_e25806_d_n8, assign27010_e25806_d_n9, assign27010_e25806_d_n10, assign27010_e25806_d_n11, assign27010_e25806_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27010_e25804: f64 = (0.1 * 0.1);
        (assign27010_e25804, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign27010_e25806;
        locals.var_xmax2_dn0 = assign27010_e25806_d_n0;
        locals.var_xmax2_dn2 = assign27010_e25806_d_n2;
        locals.var_xmax2_dn4 = assign27010_e25806_d_n4;
        locals.var_xmax2_dn5 = assign27010_e25806_d_n5;
        locals.var_xmax2_dn6 = assign27010_e25806_d_n6;
        locals.var_xmax2_dn7 = assign27010_e25806_d_n7;
        locals.var_xmax2_dn8 = assign27010_e25806_d_n8;
        locals.var_xmax2_dn9 = assign27010_e25806_d_n9;
        locals.var_xmax2_dn10 = assign27010_e25806_d_n10;
        locals.var_xmax2_dn11 = assign27010_e25806_d_n11;
        locals.var_xmax2_dn14 = assign27010_e25806_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign27020_e25814, assign27020_e25814_d_n0, assign27020_e25814_d_n2, assign27020_e25814_d_n4, assign27020_e25814_d_n5, assign27020_e25814_d_n6, assign27020_e25814_d_n7, assign27020_e25814_d_n8, assign27020_e25814_d_n9, assign27020_e25814_d_n10, assign27020_e25814_d_n11, assign27020_e25814_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27020_e25814;
        locals.var_xp_dn0 = assign27020_e25814_d_n0;
        locals.var_xp_dn2 = assign27020_e25814_d_n2;
        locals.var_xp_dn4 = assign27020_e25814_d_n4;
        locals.var_xp_dn5 = assign27020_e25814_d_n5;
        locals.var_xp_dn6 = assign27020_e25814_d_n6;
        locals.var_xp_dn7 = assign27020_e25814_d_n7;
        locals.var_xp_dn8 = assign27020_e25814_d_n8;
        locals.var_xp_dn9 = assign27020_e25814_d_n9;
        locals.var_xp_dn10 = assign27020_e25814_d_n10;
        locals.var_xp_dn11 = assign27020_e25814_d_n11;
        locals.var_xp_dn14 = assign27020_e25814_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27030_e25822, assign27030_e25822_d_n0, assign27030_e25822_d_n2, assign27030_e25822_d_n4, assign27030_e25822_d_n5, assign27030_e25822_d_n6, assign27030_e25822_d_n7, assign27030_e25822_d_n8, assign27030_e25822_d_n9, assign27030_e25822_d_n10, assign27030_e25822_d_n11, assign27030_e25822_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27030_e25822;
        locals.var_xmp_dn0 = assign27030_e25822_d_n0;
        locals.var_xmp_dn2 = assign27030_e25822_d_n2;
        locals.var_xmp_dn4 = assign27030_e25822_d_n4;
        locals.var_xmp_dn5 = assign27030_e25822_d_n5;
        locals.var_xmp_dn6 = assign27030_e25822_d_n6;
        locals.var_xmp_dn7 = assign27030_e25822_d_n7;
        locals.var_xmp_dn8 = assign27030_e25822_d_n8;
        locals.var_xmp_dn9 = assign27030_e25822_d_n9;
        locals.var_xmp_dn10 = assign27030_e25822_d_n10;
        locals.var_xmp_dn11 = assign27030_e25822_d_n11;
        locals.var_xmp_dn14 = assign27030_e25822_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27040_e25830,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27040_e25830;
        locals.var_m0_rv = 0.0;

        let (assign27050_e25838,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27050_e25838;
        locals.var_mm_rv = 0.0;

        let (assign27060_e25846, assign27060_e25846_d_n0, assign27060_e25846_d_n2, assign27060_e25846_d_n4, assign27060_e25846_d_n5, assign27060_e25846_d_n6, assign27060_e25846_d_n7, assign27060_e25846_d_n8, assign27060_e25846_d_n9, assign27060_e25846_d_n10, assign27060_e25846_d_n11, assign27060_e25846_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27060_e25846;
        locals.var_arg_dn0 = assign27060_e25846_d_n0;
        locals.var_arg_dn2 = assign27060_e25846_d_n2;
        locals.var_arg_dn4 = assign27060_e25846_d_n4;
        locals.var_arg_dn5 = assign27060_e25846_d_n5;
        locals.var_arg_dn6 = assign27060_e25846_d_n6;
        locals.var_arg_dn7 = assign27060_e25846_d_n7;
        locals.var_arg_dn8 = assign27060_e25846_d_n8;
        locals.var_arg_dn9 = assign27060_e25846_d_n9;
        locals.var_arg_dn10 = assign27060_e25846_d_n10;
        locals.var_arg_dn11 = assign27060_e25846_d_n11;
        locals.var_arg_dn14 = assign27060_e25846_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign27070_e25854, assign27070_e25854_d_n0, assign27070_e25854_d_n2, assign27070_e25854_d_n4, assign27070_e25854_d_n5, assign27070_e25854_d_n6, assign27070_e25854_d_n7, assign27070_e25854_d_n8, assign27070_e25854_d_n9, assign27070_e25854_d_n10, assign27070_e25854_d_n11, assign27070_e25854_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27070_e25854;
        locals.var_dnm_dn0 = assign27070_e25854_d_n0;
        locals.var_dnm_dn2 = assign27070_e25854_d_n2;
        locals.var_dnm_dn4 = assign27070_e25854_d_n4;
        locals.var_dnm_dn5 = assign27070_e25854_d_n5;
        locals.var_dnm_dn6 = assign27070_e25854_d_n6;
        locals.var_dnm_dn7 = assign27070_e25854_d_n7;
        locals.var_dnm_dn8 = assign27070_e25854_d_n8;
        locals.var_dnm_dn9 = assign27070_e25854_d_n9;
        locals.var_dnm_dn10 = assign27070_e25854_d_n10;
        locals.var_dnm_dn11 = assign27070_e25854_d_n11;
        locals.var_dnm_dn14 = assign27070_e25854_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27080_e25864, assign27080_e25864_d_n0, assign27080_e25864_d_n2, assign27080_e25864_d_n4, assign27080_e25864_d_n5, assign27080_e25864_d_n6, assign27080_e25864_d_n7, assign27080_e25864_d_n8, assign27080_e25864_d_n9, assign27080_e25864_d_n10, assign27080_e25864_d_n11, assign27080_e25864_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27080_e25862: f64 = (locals.var_xp * locals.var_x2);
        (assign27080_e25862, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27080_e25864;
        locals.var_xp_dn0 = assign27080_e25864_d_n0;
        locals.var_xp_dn2 = assign27080_e25864_d_n2;
        locals.var_xp_dn4 = assign27080_e25864_d_n4;
        locals.var_xp_dn5 = assign27080_e25864_d_n5;
        locals.var_xp_dn6 = assign27080_e25864_d_n6;
        locals.var_xp_dn7 = assign27080_e25864_d_n7;
        locals.var_xp_dn8 = assign27080_e25864_d_n8;
        locals.var_xp_dn9 = assign27080_e25864_d_n9;
        locals.var_xp_dn10 = assign27080_e25864_d_n10;
        locals.var_xp_dn11 = assign27080_e25864_d_n11;
        locals.var_xp_dn14 = assign27080_e25864_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27090_e25874, assign27090_e25874_d_n0, assign27090_e25874_d_n2, assign27090_e25874_d_n4, assign27090_e25874_d_n5, assign27090_e25874_d_n6, assign27090_e25874_d_n7, assign27090_e25874_d_n8, assign27090_e25874_d_n9, assign27090_e25874_d_n10, assign27090_e25874_d_n11, assign27090_e25874_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27090_e25872: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27090_e25872, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27090_e25874;
        locals.var_xmp_dn0 = assign27090_e25874_d_n0;
        locals.var_xmp_dn2 = assign27090_e25874_d_n2;
        locals.var_xmp_dn4 = assign27090_e25874_d_n4;
        locals.var_xmp_dn5 = assign27090_e25874_d_n5;
        locals.var_xmp_dn6 = assign27090_e25874_d_n6;
        locals.var_xmp_dn7 = assign27090_e25874_d_n7;
        locals.var_xmp_dn8 = assign27090_e25874_d_n8;
        locals.var_xmp_dn9 = assign27090_e25874_d_n9;
        locals.var_xmp_dn10 = assign27090_e25874_d_n10;
        locals.var_xmp_dn11 = assign27090_e25874_d_n11;
        locals.var_xmp_dn14 = assign27090_e25874_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27100_e25884, assign27100_e25884_d_n0, assign27100_e25884_d_n2, assign27100_e25884_d_n4, assign27100_e25884_d_n5, assign27100_e25884_d_n6, assign27100_e25884_d_n7, assign27100_e25884_d_n8, assign27100_e25884_d_n9, assign27100_e25884_d_n10, assign27100_e25884_d_n11, assign27100_e25884_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27100_e25882: f64 = (locals.var_xp * locals.var_x2);
        (assign27100_e25882, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27100_e25884;
        locals.var_xp_dn0 = assign27100_e25884_d_n0;
        locals.var_xp_dn2 = assign27100_e25884_d_n2;
        locals.var_xp_dn4 = assign27100_e25884_d_n4;
        locals.var_xp_dn5 = assign27100_e25884_d_n5;
        locals.var_xp_dn6 = assign27100_e25884_d_n6;
        locals.var_xp_dn7 = assign27100_e25884_d_n7;
        locals.var_xp_dn8 = assign27100_e25884_d_n8;
        locals.var_xp_dn9 = assign27100_e25884_d_n9;
        locals.var_xp_dn10 = assign27100_e25884_d_n10;
        locals.var_xp_dn11 = assign27100_e25884_d_n11;
        locals.var_xp_dn14 = assign27100_e25884_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27110_e25894, assign27110_e25894_d_n0, assign27110_e25894_d_n2, assign27110_e25894_d_n4, assign27110_e25894_d_n5, assign27110_e25894_d_n6, assign27110_e25894_d_n7, assign27110_e25894_d_n8, assign27110_e25894_d_n9, assign27110_e25894_d_n10, assign27110_e25894_d_n11, assign27110_e25894_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27110_e25892: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27110_e25892, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27110_e25894;
        locals.var_xmp_dn0 = assign27110_e25894_d_n0;
        locals.var_xmp_dn2 = assign27110_e25894_d_n2;
        locals.var_xmp_dn4 = assign27110_e25894_d_n4;
        locals.var_xmp_dn5 = assign27110_e25894_d_n5;
        locals.var_xmp_dn6 = assign27110_e25894_d_n6;
        locals.var_xmp_dn7 = assign27110_e25894_d_n7;
        locals.var_xmp_dn8 = assign27110_e25894_d_n8;
        locals.var_xmp_dn9 = assign27110_e25894_d_n9;
        locals.var_xmp_dn10 = assign27110_e25894_d_n10;
        locals.var_xmp_dn11 = assign27110_e25894_d_n11;
        locals.var_xmp_dn14 = assign27110_e25894_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27120_e25904, assign27120_e25904_d_n0, assign27120_e25904_d_n2, assign27120_e25904_d_n4, assign27120_e25904_d_n5, assign27120_e25904_d_n6, assign27120_e25904_d_n7, assign27120_e25904_d_n8, assign27120_e25904_d_n9, assign27120_e25904_d_n10, assign27120_e25904_d_n11, assign27120_e25904_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27120_e25902: f64 = (locals.var_xp + locals.var_xmp);
        (assign27120_e25902, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27120_e25904;
        locals.var_arg_dn0 = assign27120_e25904_d_n0;
        locals.var_arg_dn2 = assign27120_e25904_d_n2;
        locals.var_arg_dn4 = assign27120_e25904_d_n4;
        locals.var_arg_dn5 = assign27120_e25904_d_n5;
        locals.var_arg_dn6 = assign27120_e25904_d_n6;
        locals.var_arg_dn7 = assign27120_e25904_d_n7;
        locals.var_arg_dn8 = assign27120_e25904_d_n8;
        locals.var_arg_dn9 = assign27120_e25904_d_n9;
        locals.var_arg_dn10 = assign27120_e25904_d_n10;
        locals.var_arg_dn11 = assign27120_e25904_d_n11;
        locals.var_arg_dn14 = assign27120_e25904_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign27130_e25912, assign27130_e25912_d_n0, assign27130_e25912_d_n2, assign27130_e25912_d_n4, assign27130_e25912_d_n5, assign27130_e25912_d_n6, assign27130_e25912_d_n7, assign27130_e25912_d_n8, assign27130_e25912_d_n9, assign27130_e25912_d_n10, assign27130_e25912_d_n11, assign27130_e25912_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27130_e25912;
        locals.var_dnm_dn0 = assign27130_e25912_d_n0;
        locals.var_dnm_dn2 = assign27130_e25912_d_n2;
        locals.var_dnm_dn4 = assign27130_e25912_d_n4;
        locals.var_dnm_dn5 = assign27130_e25912_d_n5;
        locals.var_dnm_dn6 = assign27130_e25912_d_n6;
        locals.var_dnm_dn7 = assign27130_e25912_d_n7;
        locals.var_dnm_dn8 = assign27130_e25912_d_n8;
        locals.var_dnm_dn9 = assign27130_e25912_d_n9;
        locals.var_dnm_dn10 = assign27130_e25912_d_n10;
        locals.var_dnm_dn11 = assign27130_e25912_d_n11;
        locals.var_dnm_dn14 = assign27130_e25912_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign27140_e25927: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard646 = assign27140_e25927;
        locals.var_guard646_rv = 0.0;

        let assign27150_e25930: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard647 = assign27150_e25930;
        locals.var_guard647_rv = 0.0;

        let (assign27160_e25942,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) && (locals.var_guard647 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27160_e25942;
        locals.var_mm_rv = 0.0;

        let assign27170_e25945: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard648 = assign27170_e25945;
        locals.var_guard648_rv = 0.0;

        let (assign27180_e25960,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard648 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27180_e25960;
        locals.var_mm_rv = 0.0;

        let assign27190_e25963: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard649 = assign27190_e25963;
        locals.var_guard649_rv = 0.0;

        let (assign27200_e25981,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard649 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27200_e25981;
        locals.var_mm_rv = 0.0;

        let assign27210_e25984: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard650 = assign27210_e25984;
        locals.var_guard650_rv = 0.0;

        let (assign27220_e26005,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) && (locals.var_guard647 == 0.0)) && (locals.var_guard648 == 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27220_e26005;
        locals.var_mm_rv = 0.0;

        let (assign27230_e26015,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27230_e26015;
        locals.var_m0_rv = 0.0;

        let mut assign27240_loop_guard: usize = 0;
        while {
            let assign27240_cond_e26026: f64 = if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign27240_cond_e26026 != 0.0
        } {
            assign27240_loop_guard += 1;
            assert!(assign27240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27240_body0_e26037, assign27240_body0_e26037_d_n0, assign27240_body0_e26037_d_n2, assign27240_body0_e26037_d_n4, assign27240_body0_e26037_d_n5, assign27240_body0_e26037_d_n6, assign27240_body0_e26037_d_n7, assign27240_body0_e26037_d_n8, assign27240_body0_e26037_d_n9, assign27240_body0_e26037_d_n10, assign27240_body0_e26037_d_n11, assign27240_body0_e26037_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) {
        let assign27240_body0_e26035: f64 = (locals.var_dnm).sqrt();
        (assign27240_body0_e26035, (locals.var_dnm_dn0 / (2.0 * assign27240_body0_e26035)), (locals.var_dnm_dn2 / (2.0 * assign27240_body0_e26035)), (locals.var_dnm_dn4 / (2.0 * assign27240_body0_e26035)), (locals.var_dnm_dn5 / (2.0 * assign27240_body0_e26035)), (locals.var_dnm_dn6 / (2.0 * assign27240_body0_e26035)), (locals.var_dnm_dn7 / (2.0 * assign27240_body0_e26035)), (locals.var_dnm_dn8 / (2.0 * assign27240_body0_e26035)), (locals.var_dnm_dn9 / (2.0 * assign27240_body0_e26035)), (locals.var_dnm_dn10 / (2.0 * assign27240_body0_e26035)), (locals.var_dnm_dn11 / (2.0 * assign27240_body0_e26035)), (locals.var_dnm_dn14 / (2.0 * assign27240_body0_e26035)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign27240_body0_e26037;
            locals.var_dnm_dn0 = assign27240_body0_e26037_d_n0;
            locals.var_dnm_dn2 = assign27240_body0_e26037_d_n2;
            locals.var_dnm_dn4 = assign27240_body0_e26037_d_n4;
            locals.var_dnm_dn5 = assign27240_body0_e26037_d_n5;
            locals.var_dnm_dn6 = assign27240_body0_e26037_d_n6;
            locals.var_dnm_dn7 = assign27240_body0_e26037_d_n7;
            locals.var_dnm_dn8 = assign27240_body0_e26037_d_n8;
            locals.var_dnm_dn9 = assign27240_body0_e26037_d_n9;
            locals.var_dnm_dn10 = assign27240_body0_e26037_d_n10;
            locals.var_dnm_dn11 = assign27240_body0_e26037_d_n11;
            locals.var_dnm_dn14 = assign27240_body0_e26037_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign27240_body1_e26049,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 != 0.0)) {
        let assign27240_body1_e26047: f64 = (locals.var_m0 + 1.0);
        (assign27240_body1_e26047,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign27240_body1_e26049;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_82(
        locals: &mut StampLocals,
    ) {
        let (assign27250_e26071, assign27250_e26071_d_n0, assign27250_e26071_d_n2, assign27250_e26071_d_n4, assign27250_e26071_d_n5, assign27250_e26071_d_n6, assign27250_e26071_d_n7, assign27250_e26071_d_n8, assign27250_e26071_d_n9, assign27250_e26071_d_n10, assign27250_e26071_d_n11, assign27250_e26071_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) && (locals.var_guard646 == 0.0)) {
        let (assign27250_e26069, assign27250_e26069_d_n0, assign27250_e26069_d_n2, assign27250_e26069_d_n4, assign27250_e26069_d_n5, assign27250_e26069_d_n6, assign27250_e26069_d_n7, assign27250_e26069_d_n8, assign27250_e26069_d_n9, assign27250_e26069_d_n10, assign27250_e26069_d_n11, assign27250_e26069_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27250_e26066: f64 = (2.0 * 2.0);
                let assign27250_e26067: f64 = (1.0 / assign27250_e26066);
                let assign27250_e26068: f64 = (locals.var_dnm).powf(assign27250_e26067);
                (assign27250_e26068, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn0)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn2)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn4)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn5)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn6)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn7)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn8)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn9)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn10)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn11)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27250_e26067) as f64).is_finite() && ((assign27250_e26067) as f64).fract() == 0.0 { if assign27250_e26067 == 0.0 { 0.0 } else { (assign27250_e26067 * ((locals.var_dnm).powf(assign27250_e26067 - 1.0) * locals.var_dnm_dn14)) } } else { (assign27250_e26068 * (assign27250_e26067 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign27250_e26069, assign27250_e26069_d_n0, assign27250_e26069_d_n2, assign27250_e26069_d_n4, assign27250_e26069_d_n5, assign27250_e26069_d_n6, assign27250_e26069_d_n7, assign27250_e26069_d_n8, assign27250_e26069_d_n9, assign27250_e26069_d_n10, assign27250_e26069_d_n11, assign27250_e26069_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27250_e26071;
        locals.var_dnm_dn0 = assign27250_e26071_d_n0;
        locals.var_dnm_dn2 = assign27250_e26071_d_n2;
        locals.var_dnm_dn4 = assign27250_e26071_d_n4;
        locals.var_dnm_dn5 = assign27250_e26071_d_n5;
        locals.var_dnm_dn6 = assign27250_e26071_d_n6;
        locals.var_dnm_dn7 = assign27250_e26071_d_n7;
        locals.var_dnm_dn8 = assign27250_e26071_d_n8;
        locals.var_dnm_dn9 = assign27250_e26071_d_n9;
        locals.var_dnm_dn10 = assign27250_e26071_d_n10;
        locals.var_dnm_dn11 = assign27250_e26071_d_n11;
        locals.var_dnm_dn14 = assign27250_e26071_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27260_e26081, assign27260_e26081_d_n0, assign27260_e26081_d_n2, assign27260_e26081_d_n4, assign27260_e26081_d_n5, assign27260_e26081_d_n6, assign27260_e26081_d_n7, assign27260_e26081_d_n8, assign27260_e26081_d_n9, assign27260_e26081_d_n10, assign27260_e26081_d_n11, assign27260_e26081_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27260_e26079: f64 = (1.0 / locals.var_dnm);
        (assign27260_e26079, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27260_e26081;
        locals.var_dnm_dn0 = assign27260_e26081_d_n0;
        locals.var_dnm_dn2 = assign27260_e26081_d_n2;
        locals.var_dnm_dn4 = assign27260_e26081_d_n4;
        locals.var_dnm_dn5 = assign27260_e26081_d_n5;
        locals.var_dnm_dn6 = assign27260_e26081_d_n6;
        locals.var_dnm_dn7 = assign27260_e26081_d_n7;
        locals.var_dnm_dn8 = assign27260_e26081_d_n8;
        locals.var_dnm_dn9 = assign27260_e26081_d_n9;
        locals.var_dnm_dn10 = assign27260_e26081_d_n10;
        locals.var_dnm_dn11 = assign27260_e26081_d_n11;
        locals.var_dnm_dn14 = assign27260_e26081_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27270_e26093, assign27270_e26093_d_n0, assign27270_e26093_d_n2, assign27270_e26093_d_n4, assign27270_e26093_d_n5, assign27270_e26093_d_n6, assign27270_e26093_d_n7, assign27270_e26093_d_n8, assign27270_e26093_d_n9, assign27270_e26093_d_n10, assign27270_e26093_d_n11, assign27270_e26093_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27270_e26089: f64 = (locals.var_tmf1 * 0.1);
        let assign27270_e26091: f64 = (assign27270_e26089 * locals.var_dnm);
        (assign27270_e26091, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign27270_e26089 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign27270_e26093;
        locals.var_tmf0_dn0 = assign27270_e26093_d_n0;
        locals.var_tmf0_dn2 = assign27270_e26093_d_n2;
        locals.var_tmf0_dn4 = assign27270_e26093_d_n4;
        locals.var_tmf0_dn5 = assign27270_e26093_d_n5;
        locals.var_tmf0_dn6 = assign27270_e26093_d_n6;
        locals.var_tmf0_dn7 = assign27270_e26093_d_n7;
        locals.var_tmf0_dn8 = assign27270_e26093_d_n8;
        locals.var_tmf0_dn9 = assign27270_e26093_d_n9;
        locals.var_tmf0_dn10 = assign27270_e26093_d_n10;
        locals.var_tmf0_dn11 = assign27270_e26093_d_n11;
        locals.var_tmf0_dn14 = assign27270_e26093_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign27280_e26107, assign27280_e26107_d_n0, assign27280_e26107_d_n2, assign27280_e26107_d_n4, assign27280_e26107_d_n5, assign27280_e26107_d_n6, assign27280_e26107_d_n7, assign27280_e26107_d_n8, assign27280_e26107_d_n9, assign27280_e26107_d_n10, assign27280_e26107_d_n11, assign27280_e26107_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27280_e26101: f64 = (0.1 * locals.var_xmp);
        let assign27280_e26103: f64 = (assign27280_e26101 * locals.var_dnm);
        let assign27280_e26105: f64 = (assign27280_e26103 / locals.var_arg);
        (assign27280_e26105, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn0)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn2)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn4)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn5)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn6)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn7)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn8)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn9)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn10)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn11)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign27280_e26101 * locals.var_dnm_dn14)) * locals.var_arg) - (assign27280_e26103 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27280_e26107;
        locals.var_t0_dn0 = assign27280_e26107_d_n0;
        locals.var_t0_dn2 = assign27280_e26107_d_n2;
        locals.var_t0_dn4 = assign27280_e26107_d_n4;
        locals.var_t0_dn5 = assign27280_e26107_d_n5;
        locals.var_t0_dn6 = assign27280_e26107_d_n6;
        locals.var_t0_dn7 = assign27280_e26107_d_n7;
        locals.var_t0_dn8 = assign27280_e26107_d_n8;
        locals.var_t0_dn9 = assign27280_e26107_d_n9;
        locals.var_t0_dn10 = assign27280_e26107_d_n10;
        locals.var_t0_dn11 = assign27280_e26107_d_n11;
        locals.var_t0_dn14 = assign27280_e26107_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27290_e26119, assign27290_e26119_d_n0, assign27290_e26119_d_n2, assign27290_e26119_d_n4, assign27290_e26119_d_n5, assign27290_e26119_d_n6, assign27290_e26119_d_n7, assign27290_e26119_d_n8, assign27290_e26119_d_n9, assign27290_e26119_d_n10, assign27290_e26119_d_n11, assign27290_e26119_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        let assign27290_e26115: f64 = 0.1;
        let assign27290_e26117: f64 = (assign27290_e26115 - locals.var_tmf0);
        (assign27290_e26117, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27290_e26119;
        locals.var_t2_dn0 = assign27290_e26119_d_n0;
        locals.var_t2_dn2 = assign27290_e26119_d_n2;
        locals.var_t2_dn4 = assign27290_e26119_d_n4;
        locals.var_t2_dn5 = assign27290_e26119_d_n5;
        locals.var_t2_dn6 = assign27290_e26119_d_n6;
        locals.var_t2_dn7 = assign27290_e26119_d_n7;
        locals.var_t2_dn8 = assign27290_e26119_d_n8;
        locals.var_t2_dn9 = assign27290_e26119_d_n9;
        locals.var_t2_dn10 = assign27290_e26119_d_n10;
        locals.var_t2_dn11 = assign27290_e26119_d_n11;
        locals.var_t2_dn14 = assign27290_e26119_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27300_e26127, assign27300_e26127_d_n0, assign27300_e26127_d_n2, assign27300_e26127_d_n4, assign27300_e26127_d_n5, assign27300_e26127_d_n6, assign27300_e26127_d_n7, assign27300_e26127_d_n8, assign27300_e26127_d_n9, assign27300_e26127_d_n10, assign27300_e26127_d_n11, assign27300_e26127_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27300_e26127;
        locals.var_t0_dn0 = assign27300_e26127_d_n0;
        locals.var_t0_dn2 = assign27300_e26127_d_n2;
        locals.var_t0_dn4 = assign27300_e26127_d_n4;
        locals.var_t0_dn5 = assign27300_e26127_d_n5;
        locals.var_t0_dn6 = assign27300_e26127_d_n6;
        locals.var_t0_dn7 = assign27300_e26127_d_n7;
        locals.var_t0_dn8 = assign27300_e26127_d_n8;
        locals.var_t0_dn9 = assign27300_e26127_d_n9;
        locals.var_t0_dn10 = assign27300_e26127_d_n10;
        locals.var_t0_dn11 = assign27300_e26127_d_n11;
        locals.var_t0_dn14 = assign27300_e26127_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27310_e26136, assign27310_e26136_d_n0, assign27310_e26136_d_n2, assign27310_e26136_d_n4, assign27310_e26136_d_n5, assign27310_e26136_d_n6, assign27310_e26136_d_n7, assign27310_e26136_d_n8, assign27310_e26136_d_n9, assign27310_e26136_d_n10, assign27310_e26136_d_n11, assign27310_e26136_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27310_e26136;
        locals.var_t2_dn0 = assign27310_e26136_d_n0;
        locals.var_t2_dn2 = assign27310_e26136_d_n2;
        locals.var_t2_dn4 = assign27310_e26136_d_n4;
        locals.var_t2_dn5 = assign27310_e26136_d_n5;
        locals.var_t2_dn6 = assign27310_e26136_d_n6;
        locals.var_t2_dn7 = assign27310_e26136_d_n7;
        locals.var_t2_dn8 = assign27310_e26136_d_n8;
        locals.var_t2_dn9 = assign27310_e26136_d_n9;
        locals.var_t2_dn10 = assign27310_e26136_d_n10;
        locals.var_t2_dn11 = assign27310_e26136_d_n11;
        locals.var_t2_dn14 = assign27310_e26136_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27320_e26145, assign27320_e26145_d_n0, assign27320_e26145_d_n2, assign27320_e26145_d_n4, assign27320_e26145_d_n5, assign27320_e26145_d_n6, assign27320_e26145_d_n7, assign27320_e26145_d_n8, assign27320_e26145_d_n9, assign27320_e26145_d_n10, assign27320_e26145_d_n11, assign27320_e26145_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard645 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27320_e26145;
        locals.var_t0_dn0 = assign27320_e26145_d_n0;
        locals.var_t0_dn2 = assign27320_e26145_d_n2;
        locals.var_t0_dn4 = assign27320_e26145_d_n4;
        locals.var_t0_dn5 = assign27320_e26145_d_n5;
        locals.var_t0_dn6 = assign27320_e26145_d_n6;
        locals.var_t0_dn7 = assign27320_e26145_d_n7;
        locals.var_t0_dn8 = assign27320_e26145_d_n8;
        locals.var_t0_dn9 = assign27320_e26145_d_n9;
        locals.var_t0_dn10 = assign27320_e26145_d_n10;
        locals.var_t0_dn11 = assign27320_e26145_d_n11;
        locals.var_t0_dn14 = assign27320_e26145_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27330_e26154, assign27330_e26154_d_n0, assign27330_e26154_d_n2, assign27330_e26154_d_n4, assign27330_e26154_d_n5, assign27330_e26154_d_n6, assign27330_e26154_d_n7, assign27330_e26154_d_n8, assign27330_e26154_d_n9, assign27330_e26154_d_n10, assign27330_e26154_d_n11, assign27330_e26154_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign27330_e26151: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign27330_e26152: f64 = (assign27330_e26151).sqrt();
        (assign27330_e26152, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign27330_e26152)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign27330_e26152)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign27330_e26152)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign27330_e26152)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign27330_e26152)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign27330_e26152)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign27330_e26152)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign27330_e26152)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign27330_e26152)), (((locals.var_c_2esipq_ndepm_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn11)) / (2.0 * assign27330_e26152)), (((locals.var_c_2esipq_ndepm_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn14)) / (2.0 * assign27330_e26152)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign27330_e26154;
        locals.var_w_b0_dn0 = assign27330_e26154_d_n0;
        locals.var_w_b0_dn2 = assign27330_e26154_d_n2;
        locals.var_w_b0_dn4 = assign27330_e26154_d_n4;
        locals.var_w_b0_dn5 = assign27330_e26154_d_n5;
        locals.var_w_b0_dn6 = assign27330_e26154_d_n6;
        locals.var_w_b0_dn7 = assign27330_e26154_d_n7;
        locals.var_w_b0_dn8 = assign27330_e26154_d_n8;
        locals.var_w_b0_dn9 = assign27330_e26154_d_n9;
        locals.var_w_b0_dn10 = assign27330_e26154_d_n10;
        locals.var_w_b0_dn11 = assign27330_e26154_d_n11;
        locals.var_w_b0_dn14 = assign27330_e26154_d_n14;
        locals.var_w_b0_rv = 0.0;

        let assign27340_e26158: f64 = (locals.var_uc_depthn - 1e-8);
        let assign27340_e26163: f64 = if ((locals.var_w_b0 > assign27340_e26158) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard651 = assign27340_e26163;
        locals.var_guard651_rv = 0.0;

        let (assign27350_e26175, assign27350_e26175_d_n0, assign27350_e26175_d_n2, assign27350_e26175_d_n4, assign27350_e26175_d_n5, assign27350_e26175_d_n6, assign27350_e26175_d_n7, assign27350_e26175_d_n8, assign27350_e26175_d_n9, assign27350_e26175_d_n10, assign27350_e26175_d_n11, assign27350_e26175_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27350_e26171: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign27350_e26173: f64 = (assign27350_e26171 + 1e-8);
        (assign27350_e26173, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_b0_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign27350_e26175;
        locals.var_tmf1_dn0 = assign27350_e26175_d_n0;
        locals.var_tmf1_dn2 = assign27350_e26175_d_n2;
        locals.var_tmf1_dn4 = assign27350_e26175_d_n4;
        locals.var_tmf1_dn5 = assign27350_e26175_d_n5;
        locals.var_tmf1_dn6 = assign27350_e26175_d_n6;
        locals.var_tmf1_dn7 = assign27350_e26175_d_n7;
        locals.var_tmf1_dn8 = assign27350_e26175_d_n8;
        locals.var_tmf1_dn9 = assign27350_e26175_d_n9;
        locals.var_tmf1_dn10 = assign27350_e26175_d_n10;
        locals.var_tmf1_dn11 = assign27350_e26175_d_n11;
        locals.var_tmf1_dn14 = assign27350_e26175_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign27360_e26185, assign27360_e26185_d_n0, assign27360_e26185_d_n2, assign27360_e26185_d_n4, assign27360_e26185_d_n5, assign27360_e26185_d_n6, assign27360_e26185_d_n7, assign27360_e26185_d_n8, assign27360_e26185_d_n9, assign27360_e26185_d_n10, assign27360_e26185_d_n11, assign27360_e26185_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27360_e26183: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign27360_e26183, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign27360_e26185;
        locals.var_x2_dn0 = assign27360_e26185_d_n0;
        locals.var_x2_dn2 = assign27360_e26185_d_n2;
        locals.var_x2_dn4 = assign27360_e26185_d_n4;
        locals.var_x2_dn5 = assign27360_e26185_d_n5;
        locals.var_x2_dn6 = assign27360_e26185_d_n6;
        locals.var_x2_dn7 = assign27360_e26185_d_n7;
        locals.var_x2_dn8 = assign27360_e26185_d_n8;
        locals.var_x2_dn9 = assign27360_e26185_d_n9;
        locals.var_x2_dn10 = assign27360_e26185_d_n10;
        locals.var_x2_dn11 = assign27360_e26185_d_n11;
        locals.var_x2_dn14 = assign27360_e26185_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign27370_e26195, assign27370_e26195_d_n0, assign27370_e26195_d_n2, assign27370_e26195_d_n4, assign27370_e26195_d_n5, assign27370_e26195_d_n6, assign27370_e26195_d_n7, assign27370_e26195_d_n8, assign27370_e26195_d_n9, assign27370_e26195_d_n10, assign27370_e26195_d_n11, assign27370_e26195_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27370_e26193: f64 = (1e-8 * 1e-8);
        (assign27370_e26193, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign27370_e26195;
        locals.var_xmax2_dn0 = assign27370_e26195_d_n0;
        locals.var_xmax2_dn2 = assign27370_e26195_d_n2;
        locals.var_xmax2_dn4 = assign27370_e26195_d_n4;
        locals.var_xmax2_dn5 = assign27370_e26195_d_n5;
        locals.var_xmax2_dn6 = assign27370_e26195_d_n6;
        locals.var_xmax2_dn7 = assign27370_e26195_d_n7;
        locals.var_xmax2_dn8 = assign27370_e26195_d_n8;
        locals.var_xmax2_dn9 = assign27370_e26195_d_n9;
        locals.var_xmax2_dn10 = assign27370_e26195_d_n10;
        locals.var_xmax2_dn11 = assign27370_e26195_d_n11;
        locals.var_xmax2_dn14 = assign27370_e26195_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign27380_e26203, assign27380_e26203_d_n0, assign27380_e26203_d_n2, assign27380_e26203_d_n4, assign27380_e26203_d_n5, assign27380_e26203_d_n6, assign27380_e26203_d_n7, assign27380_e26203_d_n8, assign27380_e26203_d_n9, assign27380_e26203_d_n10, assign27380_e26203_d_n11, assign27380_e26203_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27380_e26203;
        locals.var_xp_dn0 = assign27380_e26203_d_n0;
        locals.var_xp_dn2 = assign27380_e26203_d_n2;
        locals.var_xp_dn4 = assign27380_e26203_d_n4;
        locals.var_xp_dn5 = assign27380_e26203_d_n5;
        locals.var_xp_dn6 = assign27380_e26203_d_n6;
        locals.var_xp_dn7 = assign27380_e26203_d_n7;
        locals.var_xp_dn8 = assign27380_e26203_d_n8;
        locals.var_xp_dn9 = assign27380_e26203_d_n9;
        locals.var_xp_dn10 = assign27380_e26203_d_n10;
        locals.var_xp_dn11 = assign27380_e26203_d_n11;
        locals.var_xp_dn14 = assign27380_e26203_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27390_e26211, assign27390_e26211_d_n0, assign27390_e26211_d_n2, assign27390_e26211_d_n4, assign27390_e26211_d_n5, assign27390_e26211_d_n6, assign27390_e26211_d_n7, assign27390_e26211_d_n8, assign27390_e26211_d_n9, assign27390_e26211_d_n10, assign27390_e26211_d_n11, assign27390_e26211_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27390_e26211;
        locals.var_xmp_dn0 = assign27390_e26211_d_n0;
        locals.var_xmp_dn2 = assign27390_e26211_d_n2;
        locals.var_xmp_dn4 = assign27390_e26211_d_n4;
        locals.var_xmp_dn5 = assign27390_e26211_d_n5;
        locals.var_xmp_dn6 = assign27390_e26211_d_n6;
        locals.var_xmp_dn7 = assign27390_e26211_d_n7;
        locals.var_xmp_dn8 = assign27390_e26211_d_n8;
        locals.var_xmp_dn9 = assign27390_e26211_d_n9;
        locals.var_xmp_dn10 = assign27390_e26211_d_n10;
        locals.var_xmp_dn11 = assign27390_e26211_d_n11;
        locals.var_xmp_dn14 = assign27390_e26211_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27400_e26219,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27400_e26219;
        locals.var_m0_rv = 0.0;

        let (assign27410_e26227,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27410_e26227;
        locals.var_mm_rv = 0.0;

        let (assign27420_e26235, assign27420_e26235_d_n0, assign27420_e26235_d_n2, assign27420_e26235_d_n4, assign27420_e26235_d_n5, assign27420_e26235_d_n6, assign27420_e26235_d_n7, assign27420_e26235_d_n8, assign27420_e26235_d_n9, assign27420_e26235_d_n10, assign27420_e26235_d_n11, assign27420_e26235_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27420_e26235;
        locals.var_arg_dn0 = assign27420_e26235_d_n0;
        locals.var_arg_dn2 = assign27420_e26235_d_n2;
        locals.var_arg_dn4 = assign27420_e26235_d_n4;
        locals.var_arg_dn5 = assign27420_e26235_d_n5;
        locals.var_arg_dn6 = assign27420_e26235_d_n6;
        locals.var_arg_dn7 = assign27420_e26235_d_n7;
        locals.var_arg_dn8 = assign27420_e26235_d_n8;
        locals.var_arg_dn9 = assign27420_e26235_d_n9;
        locals.var_arg_dn10 = assign27420_e26235_d_n10;
        locals.var_arg_dn11 = assign27420_e26235_d_n11;
        locals.var_arg_dn14 = assign27420_e26235_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign27430_e26243, assign27430_e26243_d_n0, assign27430_e26243_d_n2, assign27430_e26243_d_n4, assign27430_e26243_d_n5, assign27430_e26243_d_n6, assign27430_e26243_d_n7, assign27430_e26243_d_n8, assign27430_e26243_d_n9, assign27430_e26243_d_n10, assign27430_e26243_d_n11, assign27430_e26243_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27430_e26243;
        locals.var_dnm_dn0 = assign27430_e26243_d_n0;
        locals.var_dnm_dn2 = assign27430_e26243_d_n2;
        locals.var_dnm_dn4 = assign27430_e26243_d_n4;
        locals.var_dnm_dn5 = assign27430_e26243_d_n5;
        locals.var_dnm_dn6 = assign27430_e26243_d_n6;
        locals.var_dnm_dn7 = assign27430_e26243_d_n7;
        locals.var_dnm_dn8 = assign27430_e26243_d_n8;
        locals.var_dnm_dn9 = assign27430_e26243_d_n9;
        locals.var_dnm_dn10 = assign27430_e26243_d_n10;
        locals.var_dnm_dn11 = assign27430_e26243_d_n11;
        locals.var_dnm_dn14 = assign27430_e26243_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27440_e26253, assign27440_e26253_d_n0, assign27440_e26253_d_n2, assign27440_e26253_d_n4, assign27440_e26253_d_n5, assign27440_e26253_d_n6, assign27440_e26253_d_n7, assign27440_e26253_d_n8, assign27440_e26253_d_n9, assign27440_e26253_d_n10, assign27440_e26253_d_n11, assign27440_e26253_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27440_e26251: f64 = (locals.var_xp * locals.var_x2);
        (assign27440_e26251, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27440_e26253;
        locals.var_xp_dn0 = assign27440_e26253_d_n0;
        locals.var_xp_dn2 = assign27440_e26253_d_n2;
        locals.var_xp_dn4 = assign27440_e26253_d_n4;
        locals.var_xp_dn5 = assign27440_e26253_d_n5;
        locals.var_xp_dn6 = assign27440_e26253_d_n6;
        locals.var_xp_dn7 = assign27440_e26253_d_n7;
        locals.var_xp_dn8 = assign27440_e26253_d_n8;
        locals.var_xp_dn9 = assign27440_e26253_d_n9;
        locals.var_xp_dn10 = assign27440_e26253_d_n10;
        locals.var_xp_dn11 = assign27440_e26253_d_n11;
        locals.var_xp_dn14 = assign27440_e26253_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27450_e26263, assign27450_e26263_d_n0, assign27450_e26263_d_n2, assign27450_e26263_d_n4, assign27450_e26263_d_n5, assign27450_e26263_d_n6, assign27450_e26263_d_n7, assign27450_e26263_d_n8, assign27450_e26263_d_n9, assign27450_e26263_d_n10, assign27450_e26263_d_n11, assign27450_e26263_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27450_e26261: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27450_e26261, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27450_e26263;
        locals.var_xmp_dn0 = assign27450_e26263_d_n0;
        locals.var_xmp_dn2 = assign27450_e26263_d_n2;
        locals.var_xmp_dn4 = assign27450_e26263_d_n4;
        locals.var_xmp_dn5 = assign27450_e26263_d_n5;
        locals.var_xmp_dn6 = assign27450_e26263_d_n6;
        locals.var_xmp_dn7 = assign27450_e26263_d_n7;
        locals.var_xmp_dn8 = assign27450_e26263_d_n8;
        locals.var_xmp_dn9 = assign27450_e26263_d_n9;
        locals.var_xmp_dn10 = assign27450_e26263_d_n10;
        locals.var_xmp_dn11 = assign27450_e26263_d_n11;
        locals.var_xmp_dn14 = assign27450_e26263_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27460_e26273, assign27460_e26273_d_n0, assign27460_e26273_d_n2, assign27460_e26273_d_n4, assign27460_e26273_d_n5, assign27460_e26273_d_n6, assign27460_e26273_d_n7, assign27460_e26273_d_n8, assign27460_e26273_d_n9, assign27460_e26273_d_n10, assign27460_e26273_d_n11, assign27460_e26273_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27460_e26271: f64 = (locals.var_xp * locals.var_x2);
        (assign27460_e26271, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27460_e26273;
        locals.var_xp_dn0 = assign27460_e26273_d_n0;
        locals.var_xp_dn2 = assign27460_e26273_d_n2;
        locals.var_xp_dn4 = assign27460_e26273_d_n4;
        locals.var_xp_dn5 = assign27460_e26273_d_n5;
        locals.var_xp_dn6 = assign27460_e26273_d_n6;
        locals.var_xp_dn7 = assign27460_e26273_d_n7;
        locals.var_xp_dn8 = assign27460_e26273_d_n8;
        locals.var_xp_dn9 = assign27460_e26273_d_n9;
        locals.var_xp_dn10 = assign27460_e26273_d_n10;
        locals.var_xp_dn11 = assign27460_e26273_d_n11;
        locals.var_xp_dn14 = assign27460_e26273_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27470_e26283, assign27470_e26283_d_n0, assign27470_e26283_d_n2, assign27470_e26283_d_n4, assign27470_e26283_d_n5, assign27470_e26283_d_n6, assign27470_e26283_d_n7, assign27470_e26283_d_n8, assign27470_e26283_d_n9, assign27470_e26283_d_n10, assign27470_e26283_d_n11, assign27470_e26283_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27470_e26281: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27470_e26281, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27470_e26283;
        locals.var_xmp_dn0 = assign27470_e26283_d_n0;
        locals.var_xmp_dn2 = assign27470_e26283_d_n2;
        locals.var_xmp_dn4 = assign27470_e26283_d_n4;
        locals.var_xmp_dn5 = assign27470_e26283_d_n5;
        locals.var_xmp_dn6 = assign27470_e26283_d_n6;
        locals.var_xmp_dn7 = assign27470_e26283_d_n7;
        locals.var_xmp_dn8 = assign27470_e26283_d_n8;
        locals.var_xmp_dn9 = assign27470_e26283_d_n9;
        locals.var_xmp_dn10 = assign27470_e26283_d_n10;
        locals.var_xmp_dn11 = assign27470_e26283_d_n11;
        locals.var_xmp_dn14 = assign27470_e26283_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27480_e26293, assign27480_e26293_d_n0, assign27480_e26293_d_n2, assign27480_e26293_d_n4, assign27480_e26293_d_n5, assign27480_e26293_d_n6, assign27480_e26293_d_n7, assign27480_e26293_d_n8, assign27480_e26293_d_n9, assign27480_e26293_d_n10, assign27480_e26293_d_n11, assign27480_e26293_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27480_e26291: f64 = (locals.var_xp + locals.var_xmp);
        (assign27480_e26291, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27480_e26293;
        locals.var_arg_dn0 = assign27480_e26293_d_n0;
        locals.var_arg_dn2 = assign27480_e26293_d_n2;
        locals.var_arg_dn4 = assign27480_e26293_d_n4;
        locals.var_arg_dn5 = assign27480_e26293_d_n5;
        locals.var_arg_dn6 = assign27480_e26293_d_n6;
        locals.var_arg_dn7 = assign27480_e26293_d_n7;
        locals.var_arg_dn8 = assign27480_e26293_d_n8;
        locals.var_arg_dn9 = assign27480_e26293_d_n9;
        locals.var_arg_dn10 = assign27480_e26293_d_n10;
        locals.var_arg_dn11 = assign27480_e26293_d_n11;
        locals.var_arg_dn14 = assign27480_e26293_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_83(
        locals: &mut StampLocals,
    ) {
        let (assign27490_e26301, assign27490_e26301_d_n0, assign27490_e26301_d_n2, assign27490_e26301_d_n4, assign27490_e26301_d_n5, assign27490_e26301_d_n6, assign27490_e26301_d_n7, assign27490_e26301_d_n8, assign27490_e26301_d_n9, assign27490_e26301_d_n10, assign27490_e26301_d_n11, assign27490_e26301_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27490_e26301;
        locals.var_dnm_dn0 = assign27490_e26301_d_n0;
        locals.var_dnm_dn2 = assign27490_e26301_d_n2;
        locals.var_dnm_dn4 = assign27490_e26301_d_n4;
        locals.var_dnm_dn5 = assign27490_e26301_d_n5;
        locals.var_dnm_dn6 = assign27490_e26301_d_n6;
        locals.var_dnm_dn7 = assign27490_e26301_d_n7;
        locals.var_dnm_dn8 = assign27490_e26301_d_n8;
        locals.var_dnm_dn9 = assign27490_e26301_d_n9;
        locals.var_dnm_dn10 = assign27490_e26301_d_n10;
        locals.var_dnm_dn11 = assign27490_e26301_d_n11;
        locals.var_dnm_dn14 = assign27490_e26301_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign27500_e26316: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard652 = assign27500_e26316;
        locals.var_guard652_rv = 0.0;

        let assign27510_e26319: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard653 = assign27510_e26319;
        locals.var_guard653_rv = 0.0;

        let (assign27520_e26331,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27520_e26331;
        locals.var_mm_rv = 0.0;

        let assign27530_e26334: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard654 = assign27530_e26334;
        locals.var_guard654_rv = 0.0;

        let (assign27540_e26349,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 != 0.0)) && (locals.var_guard653 == 0.0)) && (locals.var_guard654 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27540_e26349;
        locals.var_mm_rv = 0.0;

        let assign27550_e26352: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard655 = assign27550_e26352;
        locals.var_guard655_rv = 0.0;

        let (assign27560_e26370,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 != 0.0)) && (locals.var_guard653 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27560_e26370;
        locals.var_mm_rv = 0.0;

        let assign27570_e26373: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard656 = assign27570_e26373;
        locals.var_guard656_rv = 0.0;

        let (assign27580_e26394,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 != 0.0)) && (locals.var_guard653 == 0.0)) && (locals.var_guard654 == 0.0)) && (locals.var_guard655 == 0.0)) && (locals.var_guard656 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27580_e26394;
        locals.var_mm_rv = 0.0;

        let (assign27590_e26404,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27590_e26404;
        locals.var_m0_rv = 0.0;

        let mut assign27600_loop_guard: usize = 0;
        while {
            let assign27600_cond_e26415: f64 = if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign27600_cond_e26415 != 0.0
        } {
            assign27600_loop_guard += 1;
            assert!(assign27600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27600_body0_e26426, assign27600_body0_e26426_d_n0, assign27600_body0_e26426_d_n2, assign27600_body0_e26426_d_n4, assign27600_body0_e26426_d_n5, assign27600_body0_e26426_d_n6, assign27600_body0_e26426_d_n7, assign27600_body0_e26426_d_n8, assign27600_body0_e26426_d_n9, assign27600_body0_e26426_d_n10, assign27600_body0_e26426_d_n11, assign27600_body0_e26426_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign27600_body0_e26424: f64 = (locals.var_dnm).sqrt();
        (assign27600_body0_e26424, (locals.var_dnm_dn0 / (2.0 * assign27600_body0_e26424)), (locals.var_dnm_dn2 / (2.0 * assign27600_body0_e26424)), (locals.var_dnm_dn4 / (2.0 * assign27600_body0_e26424)), (locals.var_dnm_dn5 / (2.0 * assign27600_body0_e26424)), (locals.var_dnm_dn6 / (2.0 * assign27600_body0_e26424)), (locals.var_dnm_dn7 / (2.0 * assign27600_body0_e26424)), (locals.var_dnm_dn8 / (2.0 * assign27600_body0_e26424)), (locals.var_dnm_dn9 / (2.0 * assign27600_body0_e26424)), (locals.var_dnm_dn10 / (2.0 * assign27600_body0_e26424)), (locals.var_dnm_dn11 / (2.0 * assign27600_body0_e26424)), (locals.var_dnm_dn14 / (2.0 * assign27600_body0_e26424)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign27600_body0_e26426;
            locals.var_dnm_dn0 = assign27600_body0_e26426_d_n0;
            locals.var_dnm_dn2 = assign27600_body0_e26426_d_n2;
            locals.var_dnm_dn4 = assign27600_body0_e26426_d_n4;
            locals.var_dnm_dn5 = assign27600_body0_e26426_d_n5;
            locals.var_dnm_dn6 = assign27600_body0_e26426_d_n6;
            locals.var_dnm_dn7 = assign27600_body0_e26426_d_n7;
            locals.var_dnm_dn8 = assign27600_body0_e26426_d_n8;
            locals.var_dnm_dn9 = assign27600_body0_e26426_d_n9;
            locals.var_dnm_dn10 = assign27600_body0_e26426_d_n10;
            locals.var_dnm_dn11 = assign27600_body0_e26426_d_n11;
            locals.var_dnm_dn14 = assign27600_body0_e26426_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign27600_body1_e26438,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign27600_body1_e26436: f64 = (locals.var_m0 + 1.0);
        (assign27600_body1_e26436,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign27600_body1_e26438;
            locals.var_m0_rv = 0.0;
        }

        let (assign27610_e26460, assign27610_e26460_d_n0, assign27610_e26460_d_n2, assign27610_e26460_d_n4, assign27610_e26460_d_n5, assign27610_e26460_d_n6, assign27610_e26460_d_n7, assign27610_e26460_d_n8, assign27610_e26460_d_n9, assign27610_e26460_d_n10, assign27610_e26460_d_n11, assign27610_e26460_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 == 0.0)) {
        let (assign27610_e26458, assign27610_e26458_d_n0, assign27610_e26458_d_n2, assign27610_e26458_d_n4, assign27610_e26458_d_n5, assign27610_e26458_d_n6, assign27610_e26458_d_n7, assign27610_e26458_d_n8, assign27610_e26458_d_n9, assign27610_e26458_d_n10, assign27610_e26458_d_n11, assign27610_e26458_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27610_e26455: f64 = (2.0 * 2.0);
                let assign27610_e26456: f64 = (1.0 / assign27610_e26455);
                let assign27610_e26457: f64 = (locals.var_dnm).powf(assign27610_e26456);
                (assign27610_e26457, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn0)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn2)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn4)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn5)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn6)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn7)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn8)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn9)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn10)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn11)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27610_e26456) as f64).is_finite() && ((assign27610_e26456) as f64).fract() == 0.0 { if assign27610_e26456 == 0.0 { 0.0 } else { (assign27610_e26456 * ((locals.var_dnm).powf(assign27610_e26456 - 1.0) * locals.var_dnm_dn14)) } } else { (assign27610_e26457 * (assign27610_e26456 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign27610_e26458, assign27610_e26458_d_n0, assign27610_e26458_d_n2, assign27610_e26458_d_n4, assign27610_e26458_d_n5, assign27610_e26458_d_n6, assign27610_e26458_d_n7, assign27610_e26458_d_n8, assign27610_e26458_d_n9, assign27610_e26458_d_n10, assign27610_e26458_d_n11, assign27610_e26458_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27610_e26460;
        locals.var_dnm_dn0 = assign27610_e26460_d_n0;
        locals.var_dnm_dn2 = assign27610_e26460_d_n2;
        locals.var_dnm_dn4 = assign27610_e26460_d_n4;
        locals.var_dnm_dn5 = assign27610_e26460_d_n5;
        locals.var_dnm_dn6 = assign27610_e26460_d_n6;
        locals.var_dnm_dn7 = assign27610_e26460_d_n7;
        locals.var_dnm_dn8 = assign27610_e26460_d_n8;
        locals.var_dnm_dn9 = assign27610_e26460_d_n9;
        locals.var_dnm_dn10 = assign27610_e26460_d_n10;
        locals.var_dnm_dn11 = assign27610_e26460_d_n11;
        locals.var_dnm_dn14 = assign27610_e26460_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27620_e26470, assign27620_e26470_d_n0, assign27620_e26470_d_n2, assign27620_e26470_d_n4, assign27620_e26470_d_n5, assign27620_e26470_d_n6, assign27620_e26470_d_n7, assign27620_e26470_d_n8, assign27620_e26470_d_n9, assign27620_e26470_d_n10, assign27620_e26470_d_n11, assign27620_e26470_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27620_e26468: f64 = (1.0 / locals.var_dnm);
        (assign27620_e26468, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27620_e26470;
        locals.var_dnm_dn0 = assign27620_e26470_d_n0;
        locals.var_dnm_dn2 = assign27620_e26470_d_n2;
        locals.var_dnm_dn4 = assign27620_e26470_d_n4;
        locals.var_dnm_dn5 = assign27620_e26470_d_n5;
        locals.var_dnm_dn6 = assign27620_e26470_d_n6;
        locals.var_dnm_dn7 = assign27620_e26470_d_n7;
        locals.var_dnm_dn8 = assign27620_e26470_d_n8;
        locals.var_dnm_dn9 = assign27620_e26470_d_n9;
        locals.var_dnm_dn10 = assign27620_e26470_d_n10;
        locals.var_dnm_dn11 = assign27620_e26470_d_n11;
        locals.var_dnm_dn14 = assign27620_e26470_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27630_e26482, assign27630_e26482_d_n0, assign27630_e26482_d_n2, assign27630_e26482_d_n4, assign27630_e26482_d_n5, assign27630_e26482_d_n6, assign27630_e26482_d_n7, assign27630_e26482_d_n8, assign27630_e26482_d_n9, assign27630_e26482_d_n10, assign27630_e26482_d_n11, assign27630_e26482_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27630_e26478: f64 = (locals.var_tmf1 * 1e-8);
        let assign27630_e26480: f64 = (assign27630_e26478 * locals.var_dnm);
        (assign27630_e26480, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign27630_e26478 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign27630_e26482;
        locals.var_tmf0_dn0 = assign27630_e26482_d_n0;
        locals.var_tmf0_dn2 = assign27630_e26482_d_n2;
        locals.var_tmf0_dn4 = assign27630_e26482_d_n4;
        locals.var_tmf0_dn5 = assign27630_e26482_d_n5;
        locals.var_tmf0_dn6 = assign27630_e26482_d_n6;
        locals.var_tmf0_dn7 = assign27630_e26482_d_n7;
        locals.var_tmf0_dn8 = assign27630_e26482_d_n8;
        locals.var_tmf0_dn9 = assign27630_e26482_d_n9;
        locals.var_tmf0_dn10 = assign27630_e26482_d_n10;
        locals.var_tmf0_dn11 = assign27630_e26482_d_n11;
        locals.var_tmf0_dn14 = assign27630_e26482_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign27640_e26496, assign27640_e26496_d_n0, assign27640_e26496_d_n2, assign27640_e26496_d_n4, assign27640_e26496_d_n5, assign27640_e26496_d_n6, assign27640_e26496_d_n7, assign27640_e26496_d_n8, assign27640_e26496_d_n9, assign27640_e26496_d_n10, assign27640_e26496_d_n11, assign27640_e26496_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27640_e26490: f64 = (1e-8 * locals.var_xmp);
        let assign27640_e26492: f64 = (assign27640_e26490 * locals.var_dnm);
        let assign27640_e26494: f64 = (assign27640_e26492 / locals.var_arg);
        (assign27640_e26494, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn0)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn2)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn4)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn5)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn6)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn7)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn8)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn9)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn10)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn11)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign27640_e26490 * locals.var_dnm_dn14)) * locals.var_arg) - (assign27640_e26492 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27640_e26496;
        locals.var_t3_dn0 = assign27640_e26496_d_n0;
        locals.var_t3_dn2 = assign27640_e26496_d_n2;
        locals.var_t3_dn4 = assign27640_e26496_d_n4;
        locals.var_t3_dn5 = assign27640_e26496_d_n5;
        locals.var_t3_dn6 = assign27640_e26496_d_n6;
        locals.var_t3_dn7 = assign27640_e26496_d_n7;
        locals.var_t3_dn8 = assign27640_e26496_d_n8;
        locals.var_t3_dn9 = assign27640_e26496_d_n9;
        locals.var_t3_dn10 = assign27640_e26496_d_n10;
        locals.var_t3_dn11 = assign27640_e26496_d_n11;
        locals.var_t3_dn14 = assign27640_e26496_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27650_e26508, assign27650_e26508_d_n0, assign27650_e26508_d_n2, assign27650_e26508_d_n4, assign27650_e26508_d_n5, assign27650_e26508_d_n6, assign27650_e26508_d_n7, assign27650_e26508_d_n8, assign27650_e26508_d_n9, assign27650_e26508_d_n10, assign27650_e26508_d_n11, assign27650_e26508_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign27650_e26504: f64 = (locals.var_uc_depthn - 1e-8);
        let assign27650_e26506: f64 = (assign27650_e26504 + locals.var_tmf0);
        (assign27650_e26506, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign27650_e26508;
        locals.var_w_b0_dn0 = assign27650_e26508_d_n0;
        locals.var_w_b0_dn2 = assign27650_e26508_d_n2;
        locals.var_w_b0_dn4 = assign27650_e26508_d_n4;
        locals.var_w_b0_dn5 = assign27650_e26508_d_n5;
        locals.var_w_b0_dn6 = assign27650_e26508_d_n6;
        locals.var_w_b0_dn7 = assign27650_e26508_d_n7;
        locals.var_w_b0_dn8 = assign27650_e26508_d_n8;
        locals.var_w_b0_dn9 = assign27650_e26508_d_n9;
        locals.var_w_b0_dn10 = assign27650_e26508_d_n10;
        locals.var_w_b0_dn11 = assign27650_e26508_d_n11;
        locals.var_w_b0_dn14 = assign27650_e26508_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign27660_e26516, assign27660_e26516_d_n0, assign27660_e26516_d_n2, assign27660_e26516_d_n4, assign27660_e26516_d_n5, assign27660_e26516_d_n6, assign27660_e26516_d_n7, assign27660_e26516_d_n8, assign27660_e26516_d_n9, assign27660_e26516_d_n10, assign27660_e26516_d_n11, assign27660_e26516_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27660_e26516;
        locals.var_t3_dn0 = assign27660_e26516_d_n0;
        locals.var_t3_dn2 = assign27660_e26516_d_n2;
        locals.var_t3_dn4 = assign27660_e26516_d_n4;
        locals.var_t3_dn5 = assign27660_e26516_d_n5;
        locals.var_t3_dn6 = assign27660_e26516_d_n6;
        locals.var_t3_dn7 = assign27660_e26516_d_n7;
        locals.var_t3_dn8 = assign27660_e26516_d_n8;
        locals.var_t3_dn9 = assign27660_e26516_d_n9;
        locals.var_t3_dn10 = assign27660_e26516_d_n10;
        locals.var_t3_dn11 = assign27660_e26516_d_n11;
        locals.var_t3_dn14 = assign27660_e26516_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27670_e26525, assign27670_e26525_d_n0, assign27670_e26525_d_n2, assign27670_e26525_d_n4, assign27670_e26525_d_n5, assign27670_e26525_d_n6, assign27670_e26525_d_n7, assign27670_e26525_d_n8, assign27670_e26525_d_n9, assign27670_e26525_d_n10, assign27670_e26525_d_n11, assign27670_e26525_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn11, locals.var_w_b0_dn14,)
    }
};
        locals.var_w_b0 = assign27670_e26525;
        locals.var_w_b0_dn0 = assign27670_e26525_d_n0;
        locals.var_w_b0_dn2 = assign27670_e26525_d_n2;
        locals.var_w_b0_dn4 = assign27670_e26525_d_n4;
        locals.var_w_b0_dn5 = assign27670_e26525_d_n5;
        locals.var_w_b0_dn6 = assign27670_e26525_d_n6;
        locals.var_w_b0_dn7 = assign27670_e26525_d_n7;
        locals.var_w_b0_dn8 = assign27670_e26525_d_n8;
        locals.var_w_b0_dn9 = assign27670_e26525_d_n9;
        locals.var_w_b0_dn10 = assign27670_e26525_d_n10;
        locals.var_w_b0_dn11 = assign27670_e26525_d_n11;
        locals.var_w_b0_dn14 = assign27670_e26525_d_n14;
        locals.var_w_b0_rv = 0.0;

        let (assign27680_e26534, assign27680_e26534_d_n0, assign27680_e26534_d_n2, assign27680_e26534_d_n4, assign27680_e26534_d_n5, assign27680_e26534_d_n6, assign27680_e26534_d_n7, assign27680_e26534_d_n8, assign27680_e26534_d_n9, assign27680_e26534_d_n10, assign27680_e26534_d_n11, assign27680_e26534_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard651 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27680_e26534;
        locals.var_t3_dn0 = assign27680_e26534_d_n0;
        locals.var_t3_dn2 = assign27680_e26534_d_n2;
        locals.var_t3_dn4 = assign27680_e26534_d_n4;
        locals.var_t3_dn5 = assign27680_e26534_d_n5;
        locals.var_t3_dn6 = assign27680_e26534_d_n6;
        locals.var_t3_dn7 = assign27680_e26534_d_n7;
        locals.var_t3_dn8 = assign27680_e26534_d_n8;
        locals.var_t3_dn9 = assign27680_e26534_d_n9;
        locals.var_t3_dn10 = assign27680_e26534_d_n10;
        locals.var_t3_dn11 = assign27680_e26534_d_n11;
        locals.var_t3_dn14 = assign27680_e26534_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27690_e26542, assign27690_e26542_d_n0, assign27690_e26542_d_n2, assign27690_e26542_d_n4, assign27690_e26542_d_n5, assign27690_e26542_d_n6, assign27690_e26542_d_n7, assign27690_e26542_d_n8, assign27690_e26542_d_n9, assign27690_e26542_d_n10, assign27690_e26542_d_n11, assign27690_e26542_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign27690_e26540: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep);
        (assign27690_e26540, (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_dn0), (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_dn2), (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_dn4), (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_dn5), (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_dn6), (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_dn7), (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_dn8), (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_dn9), (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_dn10), (locals.var_phi_b0_dep_dn11 - locals.var_phi_s0_dep_dn11), (locals.var_phi_b0_dep_dn14 - locals.var_phi_s0_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27690_e26542;
        locals.var_t1_dn0 = assign27690_e26542_d_n0;
        locals.var_t1_dn2 = assign27690_e26542_d_n2;
        locals.var_t1_dn4 = assign27690_e26542_d_n4;
        locals.var_t1_dn5 = assign27690_e26542_d_n5;
        locals.var_t1_dn6 = assign27690_e26542_d_n6;
        locals.var_t1_dn7 = assign27690_e26542_d_n7;
        locals.var_t1_dn8 = assign27690_e26542_d_n8;
        locals.var_t1_dn9 = assign27690_e26542_d_n9;
        locals.var_t1_dn10 = assign27690_e26542_d_n10;
        locals.var_t1_dn11 = assign27690_e26542_d_n11;
        locals.var_t1_dn14 = assign27690_e26542_d_n14;
        locals.var_t1_rv = 0.0;

        let assign27700_e26546: f64 = 0.05;
        let assign27700_e26551: f64 = if ((locals.var_t1 < assign27700_e26546) && (0.05 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard657 = assign27700_e26551;
        locals.var_guard657_rv = 0.0;

        let (assign27710_e26563, assign27710_e26563_d_n0, assign27710_e26563_d_n2, assign27710_e26563_d_n4, assign27710_e26563_d_n5, assign27710_e26563_d_n6, assign27710_e26563_d_n7, assign27710_e26563_d_n8, assign27710_e26563_d_n9, assign27710_e26563_d_n10, assign27710_e26563_d_n11, assign27710_e26563_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign27710_e26559: f64 = 0.05;
        let assign27710_e26561: f64 = (assign27710_e26559 - locals.var_t1);
        (assign27710_e26561, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign27710_e26563;
        locals.var_tmf1_dn0 = assign27710_e26563_d_n0;
        locals.var_tmf1_dn2 = assign27710_e26563_d_n2;
        locals.var_tmf1_dn4 = assign27710_e26563_d_n4;
        locals.var_tmf1_dn5 = assign27710_e26563_d_n5;
        locals.var_tmf1_dn6 = assign27710_e26563_d_n6;
        locals.var_tmf1_dn7 = assign27710_e26563_d_n7;
        locals.var_tmf1_dn8 = assign27710_e26563_d_n8;
        locals.var_tmf1_dn9 = assign27710_e26563_d_n9;
        locals.var_tmf1_dn10 = assign27710_e26563_d_n10;
        locals.var_tmf1_dn11 = assign27710_e26563_d_n11;
        locals.var_tmf1_dn14 = assign27710_e26563_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign27720_e26573, assign27720_e26573_d_n0, assign27720_e26573_d_n2, assign27720_e26573_d_n4, assign27720_e26573_d_n5, assign27720_e26573_d_n6, assign27720_e26573_d_n7, assign27720_e26573_d_n8, assign27720_e26573_d_n9, assign27720_e26573_d_n10, assign27720_e26573_d_n11, assign27720_e26573_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign27720_e26571: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign27720_e26571, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign27720_e26573;
        locals.var_x2_dn0 = assign27720_e26573_d_n0;
        locals.var_x2_dn2 = assign27720_e26573_d_n2;
        locals.var_x2_dn4 = assign27720_e26573_d_n4;
        locals.var_x2_dn5 = assign27720_e26573_d_n5;
        locals.var_x2_dn6 = assign27720_e26573_d_n6;
        locals.var_x2_dn7 = assign27720_e26573_d_n7;
        locals.var_x2_dn8 = assign27720_e26573_d_n8;
        locals.var_x2_dn9 = assign27720_e26573_d_n9;
        locals.var_x2_dn10 = assign27720_e26573_d_n10;
        locals.var_x2_dn11 = assign27720_e26573_d_n11;
        locals.var_x2_dn14 = assign27720_e26573_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign27730_e26583, assign27730_e26583_d_n0, assign27730_e26583_d_n2, assign27730_e26583_d_n4, assign27730_e26583_d_n5, assign27730_e26583_d_n6, assign27730_e26583_d_n7, assign27730_e26583_d_n8, assign27730_e26583_d_n9, assign27730_e26583_d_n10, assign27730_e26583_d_n11, assign27730_e26583_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign27730_e26581: f64 = (0.05 * 0.05);
        (assign27730_e26581, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign27730_e26583;
        locals.var_xmax2_dn0 = assign27730_e26583_d_n0;
        locals.var_xmax2_dn2 = assign27730_e26583_d_n2;
        locals.var_xmax2_dn4 = assign27730_e26583_d_n4;
        locals.var_xmax2_dn5 = assign27730_e26583_d_n5;
        locals.var_xmax2_dn6 = assign27730_e26583_d_n6;
        locals.var_xmax2_dn7 = assign27730_e26583_d_n7;
        locals.var_xmax2_dn8 = assign27730_e26583_d_n8;
        locals.var_xmax2_dn9 = assign27730_e26583_d_n9;
        locals.var_xmax2_dn10 = assign27730_e26583_d_n10;
        locals.var_xmax2_dn11 = assign27730_e26583_d_n11;
        locals.var_xmax2_dn14 = assign27730_e26583_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign27740_e26591, assign27740_e26591_d_n0, assign27740_e26591_d_n2, assign27740_e26591_d_n4, assign27740_e26591_d_n5, assign27740_e26591_d_n6, assign27740_e26591_d_n7, assign27740_e26591_d_n8, assign27740_e26591_d_n9, assign27740_e26591_d_n10, assign27740_e26591_d_n11, assign27740_e26591_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27740_e26591;
        locals.var_xp_dn0 = assign27740_e26591_d_n0;
        locals.var_xp_dn2 = assign27740_e26591_d_n2;
        locals.var_xp_dn4 = assign27740_e26591_d_n4;
        locals.var_xp_dn5 = assign27740_e26591_d_n5;
        locals.var_xp_dn6 = assign27740_e26591_d_n6;
        locals.var_xp_dn7 = assign27740_e26591_d_n7;
        locals.var_xp_dn8 = assign27740_e26591_d_n8;
        locals.var_xp_dn9 = assign27740_e26591_d_n9;
        locals.var_xp_dn10 = assign27740_e26591_d_n10;
        locals.var_xp_dn11 = assign27740_e26591_d_n11;
        locals.var_xp_dn14 = assign27740_e26591_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27750_e26599, assign27750_e26599_d_n0, assign27750_e26599_d_n2, assign27750_e26599_d_n4, assign27750_e26599_d_n5, assign27750_e26599_d_n6, assign27750_e26599_d_n7, assign27750_e26599_d_n8, assign27750_e26599_d_n9, assign27750_e26599_d_n10, assign27750_e26599_d_n11, assign27750_e26599_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27750_e26599;
        locals.var_xmp_dn0 = assign27750_e26599_d_n0;
        locals.var_xmp_dn2 = assign27750_e26599_d_n2;
        locals.var_xmp_dn4 = assign27750_e26599_d_n4;
        locals.var_xmp_dn5 = assign27750_e26599_d_n5;
        locals.var_xmp_dn6 = assign27750_e26599_d_n6;
        locals.var_xmp_dn7 = assign27750_e26599_d_n7;
        locals.var_xmp_dn8 = assign27750_e26599_d_n8;
        locals.var_xmp_dn9 = assign27750_e26599_d_n9;
        locals.var_xmp_dn10 = assign27750_e26599_d_n10;
        locals.var_xmp_dn11 = assign27750_e26599_d_n11;
        locals.var_xmp_dn14 = assign27750_e26599_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27760_e26607,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27760_e26607;
        locals.var_m0_rv = 0.0;

        let (assign27770_e26615,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27770_e26615;
        locals.var_mm_rv = 0.0;

        let (assign27780_e26623, assign27780_e26623_d_n0, assign27780_e26623_d_n2, assign27780_e26623_d_n4, assign27780_e26623_d_n5, assign27780_e26623_d_n6, assign27780_e26623_d_n7, assign27780_e26623_d_n8, assign27780_e26623_d_n9, assign27780_e26623_d_n10, assign27780_e26623_d_n11, assign27780_e26623_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27780_e26623;
        locals.var_arg_dn0 = assign27780_e26623_d_n0;
        locals.var_arg_dn2 = assign27780_e26623_d_n2;
        locals.var_arg_dn4 = assign27780_e26623_d_n4;
        locals.var_arg_dn5 = assign27780_e26623_d_n5;
        locals.var_arg_dn6 = assign27780_e26623_d_n6;
        locals.var_arg_dn7 = assign27780_e26623_d_n7;
        locals.var_arg_dn8 = assign27780_e26623_d_n8;
        locals.var_arg_dn9 = assign27780_e26623_d_n9;
        locals.var_arg_dn10 = assign27780_e26623_d_n10;
        locals.var_arg_dn11 = assign27780_e26623_d_n11;
        locals.var_arg_dn14 = assign27780_e26623_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_84(
        locals: &mut StampLocals,
    ) {
        let (assign27790_e26631, assign27790_e26631_d_n0, assign27790_e26631_d_n2, assign27790_e26631_d_n4, assign27790_e26631_d_n5, assign27790_e26631_d_n6, assign27790_e26631_d_n7, assign27790_e26631_d_n8, assign27790_e26631_d_n9, assign27790_e26631_d_n10, assign27790_e26631_d_n11, assign27790_e26631_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27790_e26631;
        locals.var_dnm_dn0 = assign27790_e26631_d_n0;
        locals.var_dnm_dn2 = assign27790_e26631_d_n2;
        locals.var_dnm_dn4 = assign27790_e26631_d_n4;
        locals.var_dnm_dn5 = assign27790_e26631_d_n5;
        locals.var_dnm_dn6 = assign27790_e26631_d_n6;
        locals.var_dnm_dn7 = assign27790_e26631_d_n7;
        locals.var_dnm_dn8 = assign27790_e26631_d_n8;
        locals.var_dnm_dn9 = assign27790_e26631_d_n9;
        locals.var_dnm_dn10 = assign27790_e26631_d_n10;
        locals.var_dnm_dn11 = assign27790_e26631_d_n11;
        locals.var_dnm_dn14 = assign27790_e26631_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27800_e26641, assign27800_e26641_d_n0, assign27800_e26641_d_n2, assign27800_e26641_d_n4, assign27800_e26641_d_n5, assign27800_e26641_d_n6, assign27800_e26641_d_n7, assign27800_e26641_d_n8, assign27800_e26641_d_n9, assign27800_e26641_d_n10, assign27800_e26641_d_n11, assign27800_e26641_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign27800_e26639: f64 = (locals.var_xp * locals.var_x2);
        (assign27800_e26639, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27800_e26641;
        locals.var_xp_dn0 = assign27800_e26641_d_n0;
        locals.var_xp_dn2 = assign27800_e26641_d_n2;
        locals.var_xp_dn4 = assign27800_e26641_d_n4;
        locals.var_xp_dn5 = assign27800_e26641_d_n5;
        locals.var_xp_dn6 = assign27800_e26641_d_n6;
        locals.var_xp_dn7 = assign27800_e26641_d_n7;
        locals.var_xp_dn8 = assign27800_e26641_d_n8;
        locals.var_xp_dn9 = assign27800_e26641_d_n9;
        locals.var_xp_dn10 = assign27800_e26641_d_n10;
        locals.var_xp_dn11 = assign27800_e26641_d_n11;
        locals.var_xp_dn14 = assign27800_e26641_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27810_e26651, assign27810_e26651_d_n0, assign27810_e26651_d_n2, assign27810_e26651_d_n4, assign27810_e26651_d_n5, assign27810_e26651_d_n6, assign27810_e26651_d_n7, assign27810_e26651_d_n8, assign27810_e26651_d_n9, assign27810_e26651_d_n10, assign27810_e26651_d_n11, assign27810_e26651_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign27810_e26649: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27810_e26649, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27810_e26651;
        locals.var_xmp_dn0 = assign27810_e26651_d_n0;
        locals.var_xmp_dn2 = assign27810_e26651_d_n2;
        locals.var_xmp_dn4 = assign27810_e26651_d_n4;
        locals.var_xmp_dn5 = assign27810_e26651_d_n5;
        locals.var_xmp_dn6 = assign27810_e26651_d_n6;
        locals.var_xmp_dn7 = assign27810_e26651_d_n7;
        locals.var_xmp_dn8 = assign27810_e26651_d_n8;
        locals.var_xmp_dn9 = assign27810_e26651_d_n9;
        locals.var_xmp_dn10 = assign27810_e26651_d_n10;
        locals.var_xmp_dn11 = assign27810_e26651_d_n11;
        locals.var_xmp_dn14 = assign27810_e26651_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27820_e26661, assign27820_e26661_d_n0, assign27820_e26661_d_n2, assign27820_e26661_d_n4, assign27820_e26661_d_n5, assign27820_e26661_d_n6, assign27820_e26661_d_n7, assign27820_e26661_d_n8, assign27820_e26661_d_n9, assign27820_e26661_d_n10, assign27820_e26661_d_n11, assign27820_e26661_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign27820_e26659: f64 = (locals.var_xp * locals.var_x2);
        (assign27820_e26659, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign27820_e26661;
        locals.var_xp_dn0 = assign27820_e26661_d_n0;
        locals.var_xp_dn2 = assign27820_e26661_d_n2;
        locals.var_xp_dn4 = assign27820_e26661_d_n4;
        locals.var_xp_dn5 = assign27820_e26661_d_n5;
        locals.var_xp_dn6 = assign27820_e26661_d_n6;
        locals.var_xp_dn7 = assign27820_e26661_d_n7;
        locals.var_xp_dn8 = assign27820_e26661_d_n8;
        locals.var_xp_dn9 = assign27820_e26661_d_n9;
        locals.var_xp_dn10 = assign27820_e26661_d_n10;
        locals.var_xp_dn11 = assign27820_e26661_d_n11;
        locals.var_xp_dn14 = assign27820_e26661_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign27830_e26671, assign27830_e26671_d_n0, assign27830_e26671_d_n2, assign27830_e26671_d_n4, assign27830_e26671_d_n5, assign27830_e26671_d_n6, assign27830_e26671_d_n7, assign27830_e26671_d_n8, assign27830_e26671_d_n9, assign27830_e26671_d_n10, assign27830_e26671_d_n11, assign27830_e26671_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign27830_e26669: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign27830_e26669, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign27830_e26671;
        locals.var_xmp_dn0 = assign27830_e26671_d_n0;
        locals.var_xmp_dn2 = assign27830_e26671_d_n2;
        locals.var_xmp_dn4 = assign27830_e26671_d_n4;
        locals.var_xmp_dn5 = assign27830_e26671_d_n5;
        locals.var_xmp_dn6 = assign27830_e26671_d_n6;
        locals.var_xmp_dn7 = assign27830_e26671_d_n7;
        locals.var_xmp_dn8 = assign27830_e26671_d_n8;
        locals.var_xmp_dn9 = assign27830_e26671_d_n9;
        locals.var_xmp_dn10 = assign27830_e26671_d_n10;
        locals.var_xmp_dn11 = assign27830_e26671_d_n11;
        locals.var_xmp_dn14 = assign27830_e26671_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign27840_e26681, assign27840_e26681_d_n0, assign27840_e26681_d_n2, assign27840_e26681_d_n4, assign27840_e26681_d_n5, assign27840_e26681_d_n6, assign27840_e26681_d_n7, assign27840_e26681_d_n8, assign27840_e26681_d_n9, assign27840_e26681_d_n10, assign27840_e26681_d_n11, assign27840_e26681_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign27840_e26679: f64 = (locals.var_xp + locals.var_xmp);
        (assign27840_e26679, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27840_e26681;
        locals.var_arg_dn0 = assign27840_e26681_d_n0;
        locals.var_arg_dn2 = assign27840_e26681_d_n2;
        locals.var_arg_dn4 = assign27840_e26681_d_n4;
        locals.var_arg_dn5 = assign27840_e26681_d_n5;
        locals.var_arg_dn6 = assign27840_e26681_d_n6;
        locals.var_arg_dn7 = assign27840_e26681_d_n7;
        locals.var_arg_dn8 = assign27840_e26681_d_n8;
        locals.var_arg_dn9 = assign27840_e26681_d_n9;
        locals.var_arg_dn10 = assign27840_e26681_d_n10;
        locals.var_arg_dn11 = assign27840_e26681_d_n11;
        locals.var_arg_dn14 = assign27840_e26681_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign27850_e26689, assign27850_e26689_d_n0, assign27850_e26689_d_n2, assign27850_e26689_d_n4, assign27850_e26689_d_n5, assign27850_e26689_d_n6, assign27850_e26689_d_n7, assign27850_e26689_d_n8, assign27850_e26689_d_n9, assign27850_e26689_d_n10, assign27850_e26689_d_n11, assign27850_e26689_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27850_e26689;
        locals.var_dnm_dn0 = assign27850_e26689_d_n0;
        locals.var_dnm_dn2 = assign27850_e26689_d_n2;
        locals.var_dnm_dn4 = assign27850_e26689_d_n4;
        locals.var_dnm_dn5 = assign27850_e26689_d_n5;
        locals.var_dnm_dn6 = assign27850_e26689_d_n6;
        locals.var_dnm_dn7 = assign27850_e26689_d_n7;
        locals.var_dnm_dn8 = assign27850_e26689_d_n8;
        locals.var_dnm_dn9 = assign27850_e26689_d_n9;
        locals.var_dnm_dn10 = assign27850_e26689_d_n10;
        locals.var_dnm_dn11 = assign27850_e26689_d_n11;
        locals.var_dnm_dn14 = assign27850_e26689_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign27860_e26704: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard658 = assign27860_e26704;
        locals.var_guard658_rv = 0.0;

        let assign27870_e26707: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign27870_e26707;
        locals.var_guard659_rv = 0.0;

        let (assign27880_e26719,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) && (locals.var_guard658 != 0.0)) && (locals.var_guard659 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27880_e26719;
        locals.var_mm_rv = 0.0;

        let assign27890_e26722: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign27890_e26722;
        locals.var_guard660_rv = 0.0;

        let (assign27900_e26737,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) && (locals.var_guard658 != 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard660 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27900_e26737;
        locals.var_mm_rv = 0.0;

        let assign27910_e26740: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign27910_e26740;
        locals.var_guard661_rv = 0.0;

        let (assign27920_e26758,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) && (locals.var_guard658 != 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard660 == 0.0)) && (locals.var_guard661 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27920_e26758;
        locals.var_mm_rv = 0.0;

        let assign27930_e26761: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign27930_e26761;
        locals.var_guard662_rv = 0.0;

        let (assign27940_e26782,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) && (locals.var_guard658 != 0.0)) && (locals.var_guard659 == 0.0)) && (locals.var_guard660 == 0.0)) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign27940_e26782;
        locals.var_mm_rv = 0.0;

        let (assign27950_e26792,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) && (locals.var_guard658 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign27950_e26792;
        locals.var_m0_rv = 0.0;

        let mut assign27960_loop_guard: usize = 0;
        while {
            let assign27960_cond_e26803: f64 = if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) && (locals.var_guard658 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign27960_cond_e26803 != 0.0
        } {
            assign27960_loop_guard += 1;
            assert!(assign27960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27960_body0_e26814, assign27960_body0_e26814_d_n0, assign27960_body0_e26814_d_n2, assign27960_body0_e26814_d_n4, assign27960_body0_e26814_d_n5, assign27960_body0_e26814_d_n6, assign27960_body0_e26814_d_n7, assign27960_body0_e26814_d_n8, assign27960_body0_e26814_d_n9, assign27960_body0_e26814_d_n10, assign27960_body0_e26814_d_n11, assign27960_body0_e26814_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) && (locals.var_guard658 != 0.0)) {
        let assign27960_body0_e26812: f64 = (locals.var_dnm).sqrt();
        (assign27960_body0_e26812, (locals.var_dnm_dn0 / (2.0 * assign27960_body0_e26812)), (locals.var_dnm_dn2 / (2.0 * assign27960_body0_e26812)), (locals.var_dnm_dn4 / (2.0 * assign27960_body0_e26812)), (locals.var_dnm_dn5 / (2.0 * assign27960_body0_e26812)), (locals.var_dnm_dn6 / (2.0 * assign27960_body0_e26812)), (locals.var_dnm_dn7 / (2.0 * assign27960_body0_e26812)), (locals.var_dnm_dn8 / (2.0 * assign27960_body0_e26812)), (locals.var_dnm_dn9 / (2.0 * assign27960_body0_e26812)), (locals.var_dnm_dn10 / (2.0 * assign27960_body0_e26812)), (locals.var_dnm_dn11 / (2.0 * assign27960_body0_e26812)), (locals.var_dnm_dn14 / (2.0 * assign27960_body0_e26812)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign27960_body0_e26814;
            locals.var_dnm_dn0 = assign27960_body0_e26814_d_n0;
            locals.var_dnm_dn2 = assign27960_body0_e26814_d_n2;
            locals.var_dnm_dn4 = assign27960_body0_e26814_d_n4;
            locals.var_dnm_dn5 = assign27960_body0_e26814_d_n5;
            locals.var_dnm_dn6 = assign27960_body0_e26814_d_n6;
            locals.var_dnm_dn7 = assign27960_body0_e26814_d_n7;
            locals.var_dnm_dn8 = assign27960_body0_e26814_d_n8;
            locals.var_dnm_dn9 = assign27960_body0_e26814_d_n9;
            locals.var_dnm_dn10 = assign27960_body0_e26814_d_n10;
            locals.var_dnm_dn11 = assign27960_body0_e26814_d_n11;
            locals.var_dnm_dn14 = assign27960_body0_e26814_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign27960_body1_e26826,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) && (locals.var_guard658 != 0.0)) {
        let assign27960_body1_e26824: f64 = (locals.var_m0 + 1.0);
        (assign27960_body1_e26824,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign27960_body1_e26826;
            locals.var_m0_rv = 0.0;
        }

        let (assign27970_e26848, assign27970_e26848_d_n0, assign27970_e26848_d_n2, assign27970_e26848_d_n4, assign27970_e26848_d_n5, assign27970_e26848_d_n6, assign27970_e26848_d_n7, assign27970_e26848_d_n8, assign27970_e26848_d_n9, assign27970_e26848_d_n10, assign27970_e26848_d_n11, assign27970_e26848_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) && (locals.var_guard658 == 0.0)) {
        let (assign27970_e26846, assign27970_e26846_d_n0, assign27970_e26846_d_n2, assign27970_e26846_d_n4, assign27970_e26846_d_n5, assign27970_e26846_d_n6, assign27970_e26846_d_n7, assign27970_e26846_d_n8, assign27970_e26846_d_n9, assign27970_e26846_d_n10, assign27970_e26846_d_n11, assign27970_e26846_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27970_e26843: f64 = (2.0 * 2.0);
                let assign27970_e26844: f64 = (1.0 / assign27970_e26843);
                let assign27970_e26845: f64 = (locals.var_dnm).powf(assign27970_e26844);
                (assign27970_e26845, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn0)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn2)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn4)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn5)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn6)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn7)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn8)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn9)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn10)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn11)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign27970_e26844) as f64).is_finite() && ((assign27970_e26844) as f64).fract() == 0.0 { if assign27970_e26844 == 0.0 { 0.0 } else { (assign27970_e26844 * ((locals.var_dnm).powf(assign27970_e26844 - 1.0) * locals.var_dnm_dn14)) } } else { (assign27970_e26845 * (assign27970_e26844 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign27970_e26846, assign27970_e26846_d_n0, assign27970_e26846_d_n2, assign27970_e26846_d_n4, assign27970_e26846_d_n5, assign27970_e26846_d_n6, assign27970_e26846_d_n7, assign27970_e26846_d_n8, assign27970_e26846_d_n9, assign27970_e26846_d_n10, assign27970_e26846_d_n11, assign27970_e26846_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27970_e26848;
        locals.var_dnm_dn0 = assign27970_e26848_d_n0;
        locals.var_dnm_dn2 = assign27970_e26848_d_n2;
        locals.var_dnm_dn4 = assign27970_e26848_d_n4;
        locals.var_dnm_dn5 = assign27970_e26848_d_n5;
        locals.var_dnm_dn6 = assign27970_e26848_d_n6;
        locals.var_dnm_dn7 = assign27970_e26848_d_n7;
        locals.var_dnm_dn8 = assign27970_e26848_d_n8;
        locals.var_dnm_dn9 = assign27970_e26848_d_n9;
        locals.var_dnm_dn10 = assign27970_e26848_d_n10;
        locals.var_dnm_dn11 = assign27970_e26848_d_n11;
        locals.var_dnm_dn14 = assign27970_e26848_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27980_e26858, assign27980_e26858_d_n0, assign27980_e26858_d_n2, assign27980_e26858_d_n4, assign27980_e26858_d_n5, assign27980_e26858_d_n6, assign27980_e26858_d_n7, assign27980_e26858_d_n8, assign27980_e26858_d_n9, assign27980_e26858_d_n10, assign27980_e26858_d_n11, assign27980_e26858_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign27980_e26856: f64 = (1.0 / locals.var_dnm);
        (assign27980_e26856, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign27980_e26858;
        locals.var_dnm_dn0 = assign27980_e26858_d_n0;
        locals.var_dnm_dn2 = assign27980_e26858_d_n2;
        locals.var_dnm_dn4 = assign27980_e26858_d_n4;
        locals.var_dnm_dn5 = assign27980_e26858_d_n5;
        locals.var_dnm_dn6 = assign27980_e26858_d_n6;
        locals.var_dnm_dn7 = assign27980_e26858_d_n7;
        locals.var_dnm_dn8 = assign27980_e26858_d_n8;
        locals.var_dnm_dn9 = assign27980_e26858_d_n9;
        locals.var_dnm_dn10 = assign27980_e26858_d_n10;
        locals.var_dnm_dn11 = assign27980_e26858_d_n11;
        locals.var_dnm_dn14 = assign27980_e26858_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign27990_e26870, assign27990_e26870_d_n0, assign27990_e26870_d_n2, assign27990_e26870_d_n4, assign27990_e26870_d_n5, assign27990_e26870_d_n6, assign27990_e26870_d_n7, assign27990_e26870_d_n8, assign27990_e26870_d_n9, assign27990_e26870_d_n10, assign27990_e26870_d_n11, assign27990_e26870_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard657 != 0.0)) {
        let assign27990_e26866: f64 = (locals.var_tmf1 * 0.05);
        let assign27990_e26868: f64 = (assign27990_e26866 * locals.var_dnm);
        (assign27990_e26868, (((locals.var_tmf1_dn0 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.05) * locals.var_dnm) + (assign27990_e26866 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign27990_e26870;
        locals.var_tmf0_dn0 = assign27990_e26870_d_n0;
        locals.var_tmf0_dn2 = assign27990_e26870_d_n2;
        locals.var_tmf0_dn4 = assign27990_e26870_d_n4;
        locals.var_tmf0_dn5 = assign27990_e26870_d_n5;
        locals.var_tmf0_dn6 = assign27990_e26870_d_n6;
        locals.var_tmf0_dn7 = assign27990_e26870_d_n7;
        locals.var_tmf0_dn8 = assign27990_e26870_d_n8;
        locals.var_tmf0_dn9 = assign27990_e26870_d_n9;
        locals.var_tmf0_dn10 = assign27990_e26870_d_n10;
        locals.var_tmf0_dn11 = assign27990_e26870_d_n11;
        locals.var_tmf0_dn14 = assign27990_e26870_d_n14;
        locals.var_tmf0_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_w_s0_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

        let assign28070_e26945: f64 = (1e-25 + 1e-18);
        let assign28070_e26950: f64 = if ((locals.var_t1 < assign28070_e26945) && (1e-18 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard663 = assign28070_e26950;
        locals.var_guard663_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_85(
        locals: &mut StampLocals,
    ) {
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
        locals.var_tmf1_rv = 0.0;

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
        locals.var_x2_rv = 0.0;

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
        locals.var_xmax2_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

        let (assign28130_e27006,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28130_e27006;
        locals.var_m0_rv = 0.0;

        let (assign28140_e27014,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28140_e27014;
        locals.var_mm_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

        let assign28230_e27103: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard664 = assign28230_e27103;
        locals.var_guard664_rv = 0.0;

        let assign28240_e27106: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard665 = assign28240_e27106;
        locals.var_guard665_rv = 0.0;

        let (assign28250_e27118,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28250_e27118;
        locals.var_mm_rv = 0.0;

        let assign28260_e27121: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard666 = assign28260_e27121;
        locals.var_guard666_rv = 0.0;

        let (assign28270_e27136,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28270_e27136;
        locals.var_mm_rv = 0.0;

        let assign28280_e27139: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard667 = assign28280_e27139;
        locals.var_guard667_rv = 0.0;

        let (assign28290_e27157,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 == 0.0)) && (locals.var_guard667 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28290_e27157;
        locals.var_mm_rv = 0.0;

        let assign28300_e27160: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard668 = assign28300_e27160;
        locals.var_guard668_rv = 0.0;

        let (assign28310_e27181,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 == 0.0)) && (locals.var_guard666 == 0.0)) && (locals.var_guard667 == 0.0)) && (locals.var_guard668 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28310_e27181;
        locals.var_mm_rv = 0.0;

        let (assign28320_e27191,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28320_e27191;
        locals.var_m0_rv = 0.0;

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
            locals.var_dnm_rv = 0.0;
            let (assign28330_body1_e27225,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        let assign28330_body1_e27223: f64 = (locals.var_m0 + 1.0);
        (assign28330_body1_e27223,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign28330_body1_e27225;
            locals.var_m0_rv = 0.0;
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
        locals.var_dnm_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

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
        locals.var_tmf0_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_86(
        locals: &mut StampLocals,
    ) {
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
        locals.var_w_res0_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_w_res0_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_qn_res0_rv = 0.0;

        let assign28430_e27337: f64 = if ((locals.var_w_bsub0 > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard669 = assign28430_e27337;
        locals.var_guard669_rv = 0.0;

        let assign28440_e27341: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28440_e27346: f64 = if ((locals.var_phi_s0_dep > assign28440_e27341) && (0.8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard670 = assign28440_e27346;
        locals.var_guard670_rv = 0.0;

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
        locals.var_tmf1_rv = 0.0;

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
        locals.var_x2_rv = 0.0;

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
        locals.var_xmax2_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

        let (assign28500_e27414,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28500_e27414;
        locals.var_m0_rv = 0.0;

        let (assign28510_e27424,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28510_e27424;
        locals.var_mm_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

        let assign28600_e27529: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard671 = assign28600_e27529;
        locals.var_guard671_rv = 0.0;

        let assign28610_e27532: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard672 = assign28610_e27532;
        locals.var_guard672_rv = 0.0;

        let (assign28620_e27546,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28620_e27546;
        locals.var_mm_rv = 0.0;

        let assign28630_e27549: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard673 = assign28630_e27549;
        locals.var_guard673_rv = 0.0;

        let (assign28640_e27566,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28640_e27566;
        locals.var_mm_rv = 0.0;

        let assign28650_e27569: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard674 = assign28650_e27569;
        locals.var_guard674_rv = 0.0;

        let (assign28660_e27589,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) && (locals.var_guard674 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28660_e27589;
        locals.var_mm_rv = 0.0;

        let assign28670_e27592: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard675 = assign28670_e27592;
        locals.var_guard675_rv = 0.0;

        let (assign28680_e27615,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) && (locals.var_guard672 == 0.0)) && (locals.var_guard673 == 0.0)) && (locals.var_guard674 == 0.0)) && (locals.var_guard675 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28680_e27615;
        locals.var_mm_rv = 0.0;

        let (assign28690_e27627,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28690_e27627;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_87(
        locals: &mut StampLocals,
    ) {
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
            locals.var_dnm_rv = 0.0;
            let (assign28700_body1_e27667,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 != 0.0)) && (locals.var_guard670 != 0.0)) && (locals.var_guard671 != 0.0)) {
        let assign28700_body1_e27665: f64 = (locals.var_m0 + 1.0);
        (assign28700_body1_e27665,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign28700_body1_e27667;
            locals.var_m0_rv = 0.0;
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
        locals.var_dnm_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

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
        locals.var_tmf0_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

        let assign28790_e27783: f64 = (locals.var_vds_maxb0 - 0.8);
        let assign28790_e27788: f64 = if ((locals.var_phib_ref > assign28790_e27783) && (0.8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard676 = assign28790_e27788;
        locals.var_guard676_rv = 0.0;

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
        locals.var_tmf1_rv = 0.0;

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
        locals.var_x2_rv = 0.0;

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
        locals.var_xmax2_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

        let (assign28850_e27862,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign28850_e27862;
        locals.var_m0_rv = 0.0;

        let (assign28860_e27873,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28860_e27873;
        locals.var_mm_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_88(
        locals: &mut StampLocals,
    ) {
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
        locals.var_dnm_rv = 0.0;

        let assign28950_e27986: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard677 = assign28950_e27986;
        locals.var_guard677_rv = 0.0;

        let assign28960_e27989: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard678 = assign28960_e27989;
        locals.var_guard678_rv = 0.0;

        let (assign28970_e28004,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28970_e28004;
        locals.var_mm_rv = 0.0;

        let assign28980_e28007: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard679 = assign28980_e28007;
        locals.var_guard679_rv = 0.0;

        let (assign28990_e28025,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 == 0.0)) && (locals.var_guard679 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign28990_e28025;
        locals.var_mm_rv = 0.0;

        let assign29000_e28028: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard680 = assign29000_e28028;
        locals.var_guard680_rv = 0.0;

        let (assign29010_e28049,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 == 0.0)) && (locals.var_guard679 == 0.0)) && (locals.var_guard680 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29010_e28049;
        locals.var_mm_rv = 0.0;

        let assign29020_e28052: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard681 = assign29020_e28052;
        locals.var_guard681_rv = 0.0;

        let (assign29030_e28076,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 == 0.0)) && (locals.var_guard679 == 0.0)) && (locals.var_guard680 == 0.0)) && (locals.var_guard681 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29030_e28076;
        locals.var_mm_rv = 0.0;

        let (assign29040_e28089,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29040_e28089;
        locals.var_m0_rv = 0.0;

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
            locals.var_dnm_rv = 0.0;
            let (assign29050_body1_e28132,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard669 == 0.0)) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) {
        let assign29050_body1_e28130: f64 = (locals.var_m0 + 1.0);
        (assign29050_body1_e28130,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29050_body1_e28132;
            locals.var_m0_rv = 0.0;
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
        locals.var_dnm_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

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
        locals.var_tmf0_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_qn_bac_rv = 0.0;

        let assign29150_e28273: f64 = (locals.var_phi_s0_dep - locals.var_vds_maxb0);
        let assign29150_e28276: f64 = 0.06;
        let assign29150_e28281: f64 = if ((assign29150_e28273 < assign29150_e28276) && (0.06 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard682 = assign29150_e28281;
        locals.var_guard682_rv = 0.0;

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
        locals.var_tmf1_rv = 0.0;

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
        locals.var_x2_rv = 0.0;

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
        locals.var_xmax2_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

        let (assign29210_e28339,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29210_e28339;
        locals.var_m0_rv = 0.0;

        let (assign29220_e28347,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29220_e28347;
        locals.var_mm_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_89(
        locals: &mut StampLocals,
    ) {
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
        locals.var_dnm_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

        let assign29310_e28436: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard683 = assign29310_e28436;
        locals.var_guard683_rv = 0.0;

        let assign29320_e28439: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard684 = assign29320_e28439;
        locals.var_guard684_rv = 0.0;

        let (assign29330_e28451,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) && (locals.var_guard684 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29330_e28451;
        locals.var_mm_rv = 0.0;

        let assign29340_e28454: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard685 = assign29340_e28454;
        locals.var_guard685_rv = 0.0;

        let (assign29350_e28469,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29350_e28469;
        locals.var_mm_rv = 0.0;

        let assign29360_e28472: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard686 = assign29360_e28472;
        locals.var_guard686_rv = 0.0;

        let (assign29370_e28490,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29370_e28490;
        locals.var_mm_rv = 0.0;

        let assign29380_e28493: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard687 = assign29380_e28493;
        locals.var_guard687_rv = 0.0;

        let (assign29390_e28514,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29390_e28514;
        locals.var_mm_rv = 0.0;

        let (assign29400_e28524,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29400_e28524;
        locals.var_m0_rv = 0.0;

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
            locals.var_dnm_rv = 0.0;
            let (assign29410_body1_e28558,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard682 != 0.0)) && (locals.var_guard683 != 0.0)) {
        let assign29410_body1_e28556: f64 = (locals.var_m0 + 1.0);
        (assign29410_body1_e28556,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign29410_body1_e28558;
            locals.var_m0_rv = 0.0;
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
        locals.var_dnm_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

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
        locals.var_tmf0_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_q_n0_cur_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_90(
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
        locals.var_t4_rv = 0.0;

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
        locals.var_qn_delta_rv = 0.0;

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
        locals.var_vdsorg_rv = 0.0;

        let assign29550_e28718: f64 = if locals.var_vds > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard688 = assign29550_e28718;
        locals.var_guard688_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

        let assign29590_e28762: f64 = 2.0;
        let assign29590_e28767: f64 = if ((locals.var_t4 < assign29590_e28762) && (2.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard689 = assign29590_e28767;
        locals.var_guard689_rv = 0.0;

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
        locals.var_tmf1_rv = 0.0;

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
        locals.var_x2_rv = 0.0;

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
        locals.var_xmax2_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

        let (assign29650_e28835,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign29650_e28835;
        locals.var_m0_rv = 0.0;

        let (assign29660_e28845,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29660_e28845;
        locals.var_mm_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

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
        locals.var_xp_rv = 0.0;

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
        locals.var_xmp_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

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
        locals.var_dnm_rv = 0.0;

        let assign29750_e28950: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard690 = assign29750_e28950;
        locals.var_guard690_rv = 0.0;

        let assign29760_e28953: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard691 = assign29760_e28953;
        locals.var_guard691_rv = 0.0;

        let (assign29770_e28967,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29770_e28967;
        locals.var_mm_rv = 0.0;

        let assign29780_e28970: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard692 = assign29780_e28970;
        locals.var_guard692_rv = 0.0;

        let (assign29790_e28987,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29790_e28987;
        locals.var_mm_rv = 0.0;

        let assign29800_e28990: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard693 = assign29800_e28990;
        locals.var_guard693_rv = 0.0;

        let (assign29810_e29010,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 == 0.0)) && (locals.var_guard693 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29810_e29010;
        locals.var_mm_rv = 0.0;

        let assign29820_e29013: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard694 = assign29820_e29013;
        locals.var_guard694_rv = 0.0;

        let (assign29830_e29036,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign29830_e29036;
        locals.var_mm_rv = 0.0;

    }
}
