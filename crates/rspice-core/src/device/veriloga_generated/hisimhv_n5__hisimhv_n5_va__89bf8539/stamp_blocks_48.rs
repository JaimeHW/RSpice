#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_377(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

        let assign97240_e149693: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2257 = assign97240_e149693;
        locals.var_guard2257_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign97260_e149704: f64 = (-3.0);
        let assign97260_e149706: f64 = (assign97260_e149704 * 34.0);
        let assign97260_e149707: f64 = if locals.var_tx < assign97260_e149706 { 1.0 } else { 0.0 };
        locals.var_guard2258 = assign97260_e149707;
        locals.var_guard2258_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

        let assign97360_e149810: f64 = if locals.var_isbd_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2259 = assign97360_e149810;
        locals.var_guard2259_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

        let assign97410_e149835: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2260 = assign97410_e149835;
        locals.var_guard2260_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign97430_e149846: f64 = (-3.0);
        let assign97430_e149848: f64 = (assign97430_e149846 * 34.0);
        let assign97430_e149849: f64 = if locals.var_tx < assign97430_e149848 { 1.0 } else { 0.0 };
        locals.var_guard2261 = assign97430_e149849;
        locals.var_guard2261_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

        let assign97530_e149952: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2262 = assign97530_e149952;
        locals.var_guard2262_rv = 0.0;

        let assign97540_e149955: f64 = if locals.var_isbd_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2263 = assign97540_e149955;
        locals.var_guard2263_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_378(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

        let assign97590_e149988: f64 = if locals.var_vbdi_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2264 = assign97590_e149988;
        locals.var_guard2264_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign97610_e150001: f64 = (-3.0);
        let assign97610_e150003: f64 = (assign97610_e150001 * 34.0);
        let assign97610_e150004: f64 = if locals.var_tx < assign97610_e150003 { 1.0 } else { 0.0 };
        locals.var_guard2265 = assign97610_e150004;
        locals.var_guard2265_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

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
        locals.var_t10_rv = 0.0;

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
        locals.var_t9_rv = 0.0;

        let assign97740_e150138: f64 = if locals.var_isbs_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2266 = assign97740_e150138;
        locals.var_guard2266_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

        let assign97790_e150163: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2267 = assign97790_e150163;
        locals.var_guard2267_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign97810_e150174: f64 = (-3.0);
        let assign97810_e150176: f64 = (assign97810_e150174 * 34.0);
        let assign97810_e150177: f64 = if locals.var_tx < assign97810_e150176 { 1.0 } else { 0.0 };
        locals.var_guard2268 = assign97810_e150177;
        locals.var_guard2268_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

        let assign97910_e150280: f64 = if locals.var_isbs_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2269 = assign97910_e150280;
        locals.var_guard2269_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_379(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t3_rv = 0.0;

        let assign97960_e150305: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2270 = assign97960_e150305;
        locals.var_guard2270_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign97980_e150316: f64 = (-3.0);
        let assign97980_e150318: f64 = (assign97980_e150316 * 34.0);
        let assign97980_e150319: f64 = if locals.var_tx < assign97980_e150318 { 1.0 } else { 0.0 };
        locals.var_guard2271 = assign97980_e150319;
        locals.var_guard2271_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

        let assign98080_e150422: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2272 = assign98080_e150422;
        locals.var_guard2272_rv = 0.0;

        let assign98090_e150425: f64 = if locals.var_isbs_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2273 = assign98090_e150425;
        locals.var_guard2273_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

        let assign98140_e150458: f64 = if locals.var_vbsi_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2274 = assign98140_e150458;
        locals.var_guard2274_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign98160_e150471: f64 = (-3.0);
        let assign98160_e150473: f64 = (assign98160_e150471 * 34.0);
        let assign98160_e150474: f64 = if locals.var_tx < assign98160_e150473 { 1.0 } else { 0.0 };
        locals.var_guard2275 = assign98160_e150474;
        locals.var_guard2275_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

        let assign98270_e150602: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2276 = assign98270_e150602;
        locals.var_guard2276_rv = 0.0;

        let assign98280_e150605: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2277 = assign98280_e150605;
        locals.var_guard2277_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign98300_e150618: f64 = if p.p503 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2278 = assign98300_e150618;
        locals.var_guard2278_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbd_btm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_380(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbd_btm_rv = 0.0;

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
        locals.var_qbd_btm_rv = 0.0;

        let assign98410_e150729: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2279 = assign98410_e150729;
        locals.var_guard2279_rv = 0.0;

        let assign98420_e150732: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2280 = assign98420_e150732;
        locals.var_guard2280_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign98440_e150745: f64 = if p.p504 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2281 = assign98440_e150745;
        locals.var_guard2281_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbd_sws_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbd_sws_rv = 0.0;

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
        locals.var_qbd_sws_rv = 0.0;

        let assign98550_e150856: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2282 = assign98550_e150856;
        locals.var_guard2282_rv = 0.0;

        let assign98560_e150859: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2283 = assign98560_e150859;
        locals.var_guard2283_rv = 0.0;

        let assign98570_e150862: f64 = if locals.var_vbdi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2284 = assign98570_e150862;
        locals.var_guard2284_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign98590_e150877: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2285 = assign98590_e150877;
        locals.var_guard2285_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

        let assign98700_e151008: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2286 = assign98700_e151008;
        locals.var_guard2286_rv = 0.0;

        let assign98710_e151011: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2287 = assign98710_e151011;
        locals.var_guard2287_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_381(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_arg_rv = 0.0;

        let assign98730_e151027: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2288 = assign98730_e151027;
        locals.var_guard2288_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

        let assign98840_e151168: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2289 = assign98840_e151168;
        locals.var_guard2289_rv = 0.0;

        let assign98850_e151171: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2290 = assign98850_e151171;
        locals.var_guard2290_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign98870_e151184: f64 = if p.p526 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2291 = assign98870_e151184;
        locals.var_guard2291_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbs_btm_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbs_btm_rv = 0.0;

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
        locals.var_qbs_btm_rv = 0.0;

        let assign98980_e151295: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2292 = assign98980_e151295;
        locals.var_guard2292_rv = 0.0;

        let assign98990_e151298: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2293 = assign98990_e151298;
        locals.var_guard2293_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign99010_e151311: f64 = if p.p527 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2294 = assign99010_e151311;
        locals.var_guard2294_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_382(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_qbs_sws_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbs_sws_rv = 0.0;

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
        locals.var_qbs_sws_rv = 0.0;

        let assign99120_e151422: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2295 = assign99120_e151422;
        locals.var_guard2295_rv = 0.0;

        let assign99130_e151425: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2296 = assign99130_e151425;
        locals.var_guard2296_rv = 0.0;

        let assign99140_e151428: f64 = if locals.var_vbsi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2297 = assign99140_e151428;
        locals.var_guard2297_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign99160_e151443: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2298 = assign99160_e151443;
        locals.var_guard2298_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbs_swg_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbs_swg_rv = 0.0;

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
        locals.var_qbs_swg_rv = 0.0;

        let assign99270_e151574: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2299 = assign99270_e151574;
        locals.var_guard2299_rv = 0.0;

        let assign99280_e151577: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2300 = assign99280_e151577;
        locals.var_guard2300_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign99300_e151593: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2301 = assign99300_e151593;
        locals.var_guard2301_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbs_swg_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbs_swg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_383(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_qbs_swg_rv = 0.0;

        let assign99430_e151744: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2302 = assign99430_e151744;
        locals.var_guard2302_rv = 0.0;

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
        locals.var_qbs_rv = 0.0;

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
        locals.var_qbd_rv = 0.0;

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
        locals.var_qbsi_rv = 0.0;

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
        locals.var_qbdi_rv = 0.0;

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
        locals.var_qbs_rv = 0.0;

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
        locals.var_qbd_rv = 0.0;

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
        locals.var_qbsi_rv = 0.0;

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
        locals.var_qbdi_rv = 0.0;

        let assign99640_e151889: f64 = (p.p540 / 1e-6);
        locals.var_ndi_i = assign99640_e151889;
        locals.var_ndi_i_rv = 0.0;

        locals.var_njl = locals.var_uc_njd;
        locals.var_njl_rv = 0.0;

        let assign99660_e151893: f64 = (1450.0 / 10000.0);
        locals.var_muen_i = assign99660_e151893;
        locals.var_muen_i_rv = 0.0;

        let assign99670_e151896: f64 = (500.0 / 10000.0);
        locals.var_muep_i = assign99670_e151896;
        locals.var_muep_i_rv = 0.0;

        locals.var_juncdlt = 0.001;
        locals.var_juncdlt_rv = 0.0;

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
        locals.var_nin_dio_rv = 0.0;

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
        locals.var_pn0_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_dn_rv = 0.0;

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
        locals.var_dp_rv = 0.0;

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
        locals.var_da_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_tau_hl_rv = 0.0;

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
        locals.var_la_rv = 0.0;

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
        locals.var_v_ha_rv = 0.0;

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
        locals.var_v_hk_rv = 0.0;

        let assign99800_e151974: f64 = if p.p539 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2303 = assign99800_e151974;
        locals.var_guard2303_rv = 0.0;

        let (assign99810_e151978,) = {
    if (locals.var_guard2303 != 0.0) {
        (locals.var_uc_njd,)
    } else {
        (locals.var_nj_k,)
    }
};
        locals.var_nj_k = assign99810_e151978;
        locals.var_nj_k_rv = 0.0;

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
        locals.var_exp_a_rv = 0.0;

        let assign99830_e151989: f64 = (locals.var_v_hk - locals.var_v_ha);
        let assign99830_e151990: f64 = (locals.var_vbd_jct - assign99830_e151989);
        let assign99830_e151992: f64 = if assign99830_e151990 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2304 = assign99830_e151992;
        locals.var_guard2304_rv = 0.0;

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
        locals.var_exp_k_rv = 0.0;

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
        locals.var_exp_k_rv = 0.0;

        let assign99860_e152023: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_ha)) { 1.0 } else { 0.0 };
        locals.var_guard2305 = assign99860_e152023;
        locals.var_guard2305_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_384(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
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
        locals.var_exp_a2_rv = 0.0;

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
        locals.var_exp_a2_rv = 0.0;

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
        locals.var_exp_a2_rv = 0.0;

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
        locals.var_p_na_rv = 0.0;

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
        locals.var_q_pexa_rv = 0.0;

        let assign99920_e152088: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2306 = assign99920_e152088;
        locals.var_guard2306_rv = 0.0;

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
        locals.var_q_qs_a_rv = 0.0;

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
        locals.var_q_nqs_a_rv = 0.0;

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
        locals.var_inqs0_a_rv = 0.0;

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
        locals.var_q_pexa_nqs_rv = 0.0;

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
        locals.var_q_qs_a_rv = 0.0;

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
        locals.var_q_pexa_nqs_rv = 0.0;

        let assign99990_e152143: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_hk)) { 1.0 } else { 0.0 };
        locals.var_guard2307 = assign99990_e152143;
        locals.var_guard2307_rv = 0.0;

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
        locals.var_exp_k2_rv = 0.0;

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
        locals.var_exp_k2_rv = 0.0;

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
        locals.var_exp_k2_rv = 0.0;

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
        locals.var_p_nk_rv = 0.0;

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
        locals.var_q_pexk_rv = 0.0;

        let assign100050_e152208: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2308 = assign100050_e152208;
        locals.var_guard2308_rv = 0.0;

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
        locals.var_q_qs_k_rv = 0.0;

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
        locals.var_q_nqs_k_rv = 0.0;

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
        locals.var_inqs0_k_rv = 0.0;

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
        locals.var_q_pexk_nqs_rv = 0.0;

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
        locals.var_q_qs_k_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_385(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv18 = ctx.node_voltage(nodes[18]);
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
        locals.var_q_pexk_nqs_rv = 0.0;

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
        locals.var_vjunc_a_rv = 0.0;

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
        locals.var_tmf2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_vjunc_a_rv = 0.0;

        let assign100160_e152296: f64 = if locals.var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2309 = assign100160_e152296;
        locals.var_guard2309_rv = 0.0;

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
        locals.var_vjunc_a_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_w_depa_rv = 0.0;

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
        locals.var_tmf1_rv = 0.0;

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
        locals.var_tmf2_rv = 0.0;

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
        locals.var_tmf2_rv = 0.0;

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
        locals.var_tmf2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_w_depa_rv = 0.0;

        let assign100260_e152379: f64 = if p.p546 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2310 = assign100260_e152379;
        locals.var_guard2310_rv = 0.0;

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
        locals.var_w_qs_a_rv = 0.0;

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
        locals.var_w_nqs_a_rv = 0.0;

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
        locals.var_iwnqs0_a_rv = 0.0;

        let (assign100300_e152413, assign100300_e152413_d_n0, assign100300_e152413_d_n2, assign100300_e152413_d_n4, assign100300_e152413_d_n5, assign100300_e152413_d_n6, assign100300_e152413_d_n7, assign100300_e152413_d_n8, assign100300_e152413_d_n9, assign100300_e152413_d_n10, assign100300_e152413_d_n11, assign100300_e152413_d_n14, assign100300_e152413_d_n18,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100300_e152411: f64 = (locals.var_w_nqs_a / p.p546);
        (assign100300_e152411, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_w_nqs_a_dn18 / p.p546),)
    } else {
        (locals.var_w_depa_nqs, locals.var_w_depa_nqs_dn0, locals.var_w_depa_nqs_dn2, locals.var_w_depa_nqs_dn4, locals.var_w_depa_nqs_dn5, locals.var_w_depa_nqs_dn6, locals.var_w_depa_nqs_dn7, locals.var_w_depa_nqs_dn8, locals.var_w_depa_nqs_dn9, locals.var_w_depa_nqs_dn10, locals.var_w_depa_nqs_dn11, locals.var_w_depa_nqs_dn14, locals.var_w_depa_nqs_dn18,)
    }
};
        locals.var_w_depa_nqs = assign100300_e152413;
        locals.var_w_depa_nqs_dn0 = assign100300_e152413_d_n0;
        locals.var_w_depa_nqs_dn2 = assign100300_e152413_d_n2;
        locals.var_w_depa_nqs_dn4 = assign100300_e152413_d_n4;
        locals.var_w_depa_nqs_dn5 = assign100300_e152413_d_n5;
        locals.var_w_depa_nqs_dn6 = assign100300_e152413_d_n6;
        locals.var_w_depa_nqs_dn7 = assign100300_e152413_d_n7;
        locals.var_w_depa_nqs_dn8 = assign100300_e152413_d_n8;
        locals.var_w_depa_nqs_dn9 = assign100300_e152413_d_n9;
        locals.var_w_depa_nqs_dn10 = assign100300_e152413_d_n10;
        locals.var_w_depa_nqs_dn11 = assign100300_e152413_d_n11;
        locals.var_w_depa_nqs_dn14 = assign100300_e152413_d_n14;
        locals.var_w_depa_nqs_dn18 = assign100300_e152413_d_n18;
        locals.var_w_depa_nqs_rv = 0.0;

        let (assign100310_e152420, assign100310_e152420_d_n0, assign100310_e152420_d_n2, assign100310_e152420_d_n4, assign100310_e152420_d_n5, assign100310_e152420_d_n6, assign100310_e152420_d_n7, assign100310_e152420_d_n8, assign100310_e152420_d_n9, assign100310_e152420_d_n10, assign100310_e152420_d_n11, assign100310_e152420_d_n14,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2310 == 0.0)) {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn11, locals.var_w_depa_dn14,)
    } else {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn11, locals.var_w_qs_a_dn14,)
    }
};
        locals.var_w_qs_a = assign100310_e152420;
        locals.var_w_qs_a_dn0 = assign100310_e152420_d_n0;
        locals.var_w_qs_a_dn2 = assign100310_e152420_d_n2;
        locals.var_w_qs_a_dn4 = assign100310_e152420_d_n4;
        locals.var_w_qs_a_dn5 = assign100310_e152420_d_n5;
        locals.var_w_qs_a_dn6 = assign100310_e152420_d_n6;
        locals.var_w_qs_a_dn7 = assign100310_e152420_d_n7;
        locals.var_w_qs_a_dn8 = assign100310_e152420_d_n8;
        locals.var_w_qs_a_dn9 = assign100310_e152420_d_n9;
        locals.var_w_qs_a_dn10 = assign100310_e152420_d_n10;
        locals.var_w_qs_a_dn11 = assign100310_e152420_d_n11;
        locals.var_w_qs_a_dn14 = assign100310_e152420_d_n14;
        locals.var_w_qs_a_rv = 0.0;

        let (assign100320_e152427, assign100320_e152427_d_n0, assign100320_e152427_d_n2, assign100320_e152427_d_n4, assign100320_e152427_d_n5, assign100320_e152427_d_n6, assign100320_e152427_d_n7, assign100320_e152427_d_n8, assign100320_e152427_d_n9, assign100320_e152427_d_n10, assign100320_e152427_d_n11, assign100320_e152427_d_n14, assign100320_e152427_d_n18,) = {
    if ((locals.var_guard2303 != 0.0) && (locals.var_guard2310 == 0.0)) {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn11, locals.var_w_qs_a_dn14, 0.0,)
    } else {
        (locals.var_w_depa_nqs, locals.var_w_depa_nqs_dn0, locals.var_w_depa_nqs_dn2, locals.var_w_depa_nqs_dn4, locals.var_w_depa_nqs_dn5, locals.var_w_depa_nqs_dn6, locals.var_w_depa_nqs_dn7, locals.var_w_depa_nqs_dn8, locals.var_w_depa_nqs_dn9, locals.var_w_depa_nqs_dn10, locals.var_w_depa_nqs_dn11, locals.var_w_depa_nqs_dn14, locals.var_w_depa_nqs_dn18,)
    }
};
        locals.var_w_depa_nqs = assign100320_e152427;
        locals.var_w_depa_nqs_dn0 = assign100320_e152427_d_n0;
        locals.var_w_depa_nqs_dn2 = assign100320_e152427_d_n2;
        locals.var_w_depa_nqs_dn4 = assign100320_e152427_d_n4;
        locals.var_w_depa_nqs_dn5 = assign100320_e152427_d_n5;
        locals.var_w_depa_nqs_dn6 = assign100320_e152427_d_n6;
        locals.var_w_depa_nqs_dn7 = assign100320_e152427_d_n7;
        locals.var_w_depa_nqs_dn8 = assign100320_e152427_d_n8;
        locals.var_w_depa_nqs_dn9 = assign100320_e152427_d_n9;
        locals.var_w_depa_nqs_dn10 = assign100320_e152427_d_n10;
        locals.var_w_depa_nqs_dn11 = assign100320_e152427_d_n11;
        locals.var_w_depa_nqs_dn14 = assign100320_e152427_d_n14;
        locals.var_w_depa_nqs_dn18 = assign100320_e152427_d_n18;
        locals.var_w_depa_nqs_rv = 0.0;

        let (assign100330_e152438,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100330_e152431: f64 = (locals.var_ndi_i * p.p13);
        let assign100330_e152433: f64 = (assign100330_e152431 * 1.6021918e-19);
        let assign100330_e152434: f64 = (-assign100330_e152433);
        let assign100330_e152436: f64 = (assign100330_e152434 * p.p545);
        (assign100330_e152436,)
    } else {
        (locals.var_q_n0,)
    }
};
        locals.var_q_n0 = assign100330_e152438;
        locals.var_q_n0_rv = 0.0;

        let (assign100340_e152456, assign100340_e152456_d_n0, assign100340_e152456_d_n2, assign100340_e152456_d_n4, assign100340_e152456_d_n5, assign100340_e152456_d_n6, assign100340_e152456_d_n7, assign100340_e152456_d_n8, assign100340_e152456_d_n9, assign100340_e152456_d_n10, assign100340_e152456_d_n11, assign100340_e152456_d_n14, assign100340_e152456_d_n16, assign100340_e152456_d_n18,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100340_e152442: f64 = (locals.var_la * locals.var_q_pexa_nqs);
        let assign100340_e152444: f64 = (-p.p545);
        let assign100340_e152446: f64 = (assign100340_e152444 / locals.var_la);
        let assign100340_e152447: f64 = (assign100340_e152446).exp();
        let assign100340_e152449: f64 = (-locals.var_w_depa_nqs);
        let assign100340_e152451: f64 = (assign100340_e152449 / locals.var_la);
        let assign100340_e152452: f64 = (assign100340_e152451).exp();
        let assign100340_e152453: f64 = (assign100340_e152447 - assign100340_e152452);
        let assign100340_e152454: f64 = (assign100340_e152442 * assign100340_e152453);
        (assign100340_e152454, ((((locals.var_la_dn0 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn0)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn0) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn0) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn0)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn2 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn2)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn2) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn2) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn2)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn4 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn4)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn4) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn4) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn4)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn5 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn5)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn5) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn5) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn5)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn6 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn6)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn6) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn6) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn6)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn7 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn7)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn7) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn7) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn7)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn8 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn8)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn8) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn8) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn8)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn9 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn9)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn9) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn9) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn9)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn10 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn10)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn10) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn10) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn10)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn11 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn11)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn11) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn11) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn11)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn14 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn14)) * assign100340_e152453) + (assign100340_e152442 * ((assign100340_e152447 * (-((assign100340_e152444 * locals.var_la_dn14) / (locals.var_la * locals.var_la)))) - (assign100340_e152452 * ((((-locals.var_w_depa_nqs_dn14) * locals.var_la) - (assign100340_e152449 * locals.var_la_dn14)) / (locals.var_la * locals.var_la)))))), ((locals.var_la * locals.var_q_pexa_nqs_dn16) * assign100340_e152453), (assign100340_e152442 * (-(assign100340_e152452 * ((-locals.var_w_depa_nqs_dn18) / locals.var_la)))),)
    } else {
        (locals.var_q_nexa_nqs, locals.var_q_nexa_nqs_dn0, locals.var_q_nexa_nqs_dn2, locals.var_q_nexa_nqs_dn4, locals.var_q_nexa_nqs_dn5, locals.var_q_nexa_nqs_dn6, locals.var_q_nexa_nqs_dn7, locals.var_q_nexa_nqs_dn8, locals.var_q_nexa_nqs_dn9, locals.var_q_nexa_nqs_dn10, locals.var_q_nexa_nqs_dn11, locals.var_q_nexa_nqs_dn14, locals.var_q_nexa_nqs_dn16, locals.var_q_nexa_nqs_dn18,)
    }
};
        locals.var_q_nexa_nqs = assign100340_e152456;
        locals.var_q_nexa_nqs_dn0 = assign100340_e152456_d_n0;
        locals.var_q_nexa_nqs_dn2 = assign100340_e152456_d_n2;
        locals.var_q_nexa_nqs_dn4 = assign100340_e152456_d_n4;
        locals.var_q_nexa_nqs_dn5 = assign100340_e152456_d_n5;
        locals.var_q_nexa_nqs_dn6 = assign100340_e152456_d_n6;
        locals.var_q_nexa_nqs_dn7 = assign100340_e152456_d_n7;
        locals.var_q_nexa_nqs_dn8 = assign100340_e152456_d_n8;
        locals.var_q_nexa_nqs_dn9 = assign100340_e152456_d_n9;
        locals.var_q_nexa_nqs_dn10 = assign100340_e152456_d_n10;
        locals.var_q_nexa_nqs_dn11 = assign100340_e152456_d_n11;
        locals.var_q_nexa_nqs_dn14 = assign100340_e152456_d_n14;
        locals.var_q_nexa_nqs_dn16 = assign100340_e152456_d_n16;
        locals.var_q_nexa_nqs_dn18 = assign100340_e152456_d_n18;
        locals.var_q_nexa_nqs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_386(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign100350_e152472, assign100350_e152472_d_n0, assign100350_e152472_d_n2, assign100350_e152472_d_n4, assign100350_e152472_d_n5, assign100350_e152472_d_n6, assign100350_e152472_d_n7, assign100350_e152472_d_n8, assign100350_e152472_d_n9, assign100350_e152472_d_n10, assign100350_e152472_d_n11, assign100350_e152472_d_n14, assign100350_e152472_d_n17, assign100350_e152472_d_n18,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100350_e152460: f64 = (locals.var_la * locals.var_q_pexk_nqs);
        let assign100350_e152463: f64 = (p.p545 - locals.var_w_depa_nqs);
        let assign100350_e152464: f64 = (-assign100350_e152463);
        let assign100350_e152466: f64 = (assign100350_e152464 / locals.var_la);
        let assign100350_e152467: f64 = (assign100350_e152466).exp();
        let assign100350_e152469: f64 = (assign100350_e152467 - 1.0);
        let assign100350_e152470: f64 = (assign100350_e152460 * assign100350_e152469);
        (assign100350_e152470, ((((locals.var_la_dn0 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn0)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn0)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn0)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn2 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn2)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn2)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn2)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn4 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn4)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn4)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn4)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn5 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn5)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn5)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn5)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn6 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn6)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn6)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn6)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn7 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn7)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn7)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn7)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn8 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn8)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn8)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn8)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn9 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn9)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn9)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn9)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn10 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn10)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn10)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn10)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn11 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn11)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn11)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn11)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn14 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn14)) * assign100350_e152469) + (assign100350_e152460 * (assign100350_e152467 * ((((-(-locals.var_w_depa_nqs_dn14)) * locals.var_la) - (assign100350_e152464 * locals.var_la_dn14)) / (locals.var_la * locals.var_la))))), ((locals.var_la * locals.var_q_pexk_nqs_dn17) * assign100350_e152469), (assign100350_e152460 * (assign100350_e152467 * ((-(-locals.var_w_depa_nqs_dn18)) / locals.var_la))),)
    } else {
        (locals.var_q_nexk_nqs, locals.var_q_nexk_nqs_dn0, locals.var_q_nexk_nqs_dn2, locals.var_q_nexk_nqs_dn4, locals.var_q_nexk_nqs_dn5, locals.var_q_nexk_nqs_dn6, locals.var_q_nexk_nqs_dn7, locals.var_q_nexk_nqs_dn8, locals.var_q_nexk_nqs_dn9, locals.var_q_nexk_nqs_dn10, locals.var_q_nexk_nqs_dn11, locals.var_q_nexk_nqs_dn14, locals.var_q_nexk_nqs_dn17, locals.var_q_nexk_nqs_dn18,)
    }
};
        locals.var_q_nexk_nqs = assign100350_e152472;
        locals.var_q_nexk_nqs_dn0 = assign100350_e152472_d_n0;
        locals.var_q_nexk_nqs_dn2 = assign100350_e152472_d_n2;
        locals.var_q_nexk_nqs_dn4 = assign100350_e152472_d_n4;
        locals.var_q_nexk_nqs_dn5 = assign100350_e152472_d_n5;
        locals.var_q_nexk_nqs_dn6 = assign100350_e152472_d_n6;
        locals.var_q_nexk_nqs_dn7 = assign100350_e152472_d_n7;
        locals.var_q_nexk_nqs_dn8 = assign100350_e152472_d_n8;
        locals.var_q_nexk_nqs_dn9 = assign100350_e152472_d_n9;
        locals.var_q_nexk_nqs_dn10 = assign100350_e152472_d_n10;
        locals.var_q_nexk_nqs_dn11 = assign100350_e152472_d_n11;
        locals.var_q_nexk_nqs_dn14 = assign100350_e152472_d_n14;
        locals.var_q_nexk_nqs_dn17 = assign100350_e152472_d_n17;
        locals.var_q_nexk_nqs_dn18 = assign100350_e152472_d_n18;
        locals.var_q_nexk_nqs_rv = 0.0;

        let (assign100360_e152481, assign100360_e152481_d_n0, assign100360_e152481_d_n2, assign100360_e152481_d_n4, assign100360_e152481_d_n5, assign100360_e152481_d_n6, assign100360_e152481_d_n7, assign100360_e152481_d_n8, assign100360_e152481_d_n9, assign100360_e152481_d_n10, assign100360_e152481_d_n11, assign100360_e152481_d_n14, assign100360_e152481_d_n16, assign100360_e152481_d_n17, assign100360_e152481_d_n18,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100360_e152476: f64 = (locals.var_q_n0 + locals.var_q_nexa_nqs);
        let assign100360_e152478: f64 = (assign100360_e152476 + locals.var_q_nexk_nqs);
        let assign100360_e152479: f64 = (-assign100360_e152478);
        (assign100360_e152479, (-(locals.var_q_nexa_nqs_dn0 + locals.var_q_nexk_nqs_dn0)), (-(locals.var_q_nexa_nqs_dn2 + locals.var_q_nexk_nqs_dn2)), (-(locals.var_q_nexa_nqs_dn4 + locals.var_q_nexk_nqs_dn4)), (-(locals.var_q_nexa_nqs_dn5 + locals.var_q_nexk_nqs_dn5)), (-(locals.var_q_nexa_nqs_dn6 + locals.var_q_nexk_nqs_dn6)), (-(locals.var_q_nexa_nqs_dn7 + locals.var_q_nexk_nqs_dn7)), (-(locals.var_q_nexa_nqs_dn8 + locals.var_q_nexk_nqs_dn8)), (-(locals.var_q_nexa_nqs_dn9 + locals.var_q_nexk_nqs_dn9)), (-(locals.var_q_nexa_nqs_dn10 + locals.var_q_nexk_nqs_dn10)), (-(locals.var_q_nexa_nqs_dn11 + locals.var_q_nexk_nqs_dn11)), (-(locals.var_q_nexa_nqs_dn14 + locals.var_q_nexk_nqs_dn14)), (-locals.var_q_nexa_nqs_dn16), (-locals.var_q_nexk_nqs_dn17), (-(locals.var_q_nexa_nqs_dn18 + locals.var_q_nexk_nqs_dn18)),)
    } else {
        (locals.var_qrr, locals.var_qrr_dn0, locals.var_qrr_dn2, locals.var_qrr_dn4, locals.var_qrr_dn5, locals.var_qrr_dn6, locals.var_qrr_dn7, locals.var_qrr_dn8, locals.var_qrr_dn9, locals.var_qrr_dn10, locals.var_qrr_dn11, locals.var_qrr_dn14, locals.var_qrr_dn16, locals.var_qrr_dn17, locals.var_qrr_dn18,)
    }
};
        locals.var_qrr = assign100360_e152481;
        locals.var_qrr_dn0 = assign100360_e152481_d_n0;
        locals.var_qrr_dn2 = assign100360_e152481_d_n2;
        locals.var_qrr_dn4 = assign100360_e152481_d_n4;
        locals.var_qrr_dn5 = assign100360_e152481_d_n5;
        locals.var_qrr_dn6 = assign100360_e152481_d_n6;
        locals.var_qrr_dn7 = assign100360_e152481_d_n7;
        locals.var_qrr_dn8 = assign100360_e152481_d_n8;
        locals.var_qrr_dn9 = assign100360_e152481_d_n9;
        locals.var_qrr_dn10 = assign100360_e152481_d_n10;
        locals.var_qrr_dn11 = assign100360_e152481_d_n11;
        locals.var_qrr_dn14 = assign100360_e152481_d_n14;
        locals.var_qrr_dn16 = assign100360_e152481_d_n16;
        locals.var_qrr_dn17 = assign100360_e152481_d_n17;
        locals.var_qrr_dn18 = assign100360_e152481_d_n18;
        locals.var_qrr_rv = 0.0;

        let (assign100370_e152489, assign100370_e152489_d_n0, assign100370_e152489_d_n2, assign100370_e152489_d_n4, assign100370_e152489_d_n5, assign100370_e152489_d_n6, assign100370_e152489_d_n7, assign100370_e152489_d_n8, assign100370_e152489_d_n9, assign100370_e152489_d_n10, assign100370_e152489_d_n11, assign100370_e152489_d_n14, assign100370_e152489_d_n16, assign100370_e152489_d_n17, assign100370_e152489_d_n18,) = {
    if (locals.var_guard2303 != 0.0) {
        let assign100370_e152486: f64 = (locals.var_mfactor * locals.var_qrr);
        let assign100370_e152487: f64 = (locals.var_qbd + assign100370_e152486);
        (assign100370_e152487, (locals.var_qbd_dn0 + (locals.var_mfactor * locals.var_qrr_dn0)), (locals.var_qbd_dn2 + (locals.var_mfactor * locals.var_qrr_dn2)), (locals.var_qbd_dn4 + (locals.var_mfactor * locals.var_qrr_dn4)), (locals.var_qbd_dn5 + (locals.var_mfactor * locals.var_qrr_dn5)), (locals.var_qbd_dn6 + (locals.var_mfactor * locals.var_qrr_dn6)), (locals.var_qbd_dn7 + (locals.var_mfactor * locals.var_qrr_dn7)), (locals.var_qbd_dn8 + (locals.var_mfactor * locals.var_qrr_dn8)), (locals.var_qbd_dn9 + (locals.var_mfactor * locals.var_qrr_dn9)), (locals.var_qbd_dn10 + (locals.var_mfactor * locals.var_qrr_dn10)), (locals.var_qbd_dn11 + (locals.var_mfactor * locals.var_qrr_dn11)), (locals.var_qbd_dn14 + (locals.var_mfactor * locals.var_qrr_dn14)), (locals.var_qbd_dn16 + (locals.var_mfactor * locals.var_qrr_dn16)), (locals.var_qbd_dn17 + (locals.var_mfactor * locals.var_qrr_dn17)), (locals.var_qbd_dn18 + (locals.var_mfactor * locals.var_qrr_dn18)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn14, locals.var_qbd_dn16, locals.var_qbd_dn17, locals.var_qbd_dn18,)
    }
};
        locals.var_qbd = assign100370_e152489;
        locals.var_qbd_dn0 = assign100370_e152489_d_n0;
        locals.var_qbd_dn2 = assign100370_e152489_d_n2;
        locals.var_qbd_dn4 = assign100370_e152489_d_n4;
        locals.var_qbd_dn5 = assign100370_e152489_d_n5;
        locals.var_qbd_dn6 = assign100370_e152489_d_n6;
        locals.var_qbd_dn7 = assign100370_e152489_d_n7;
        locals.var_qbd_dn8 = assign100370_e152489_d_n8;
        locals.var_qbd_dn9 = assign100370_e152489_d_n9;
        locals.var_qbd_dn10 = assign100370_e152489_d_n10;
        locals.var_qbd_dn11 = assign100370_e152489_d_n11;
        locals.var_qbd_dn14 = assign100370_e152489_d_n14;
        locals.var_qbd_dn16 = assign100370_e152489_d_n16;
        locals.var_qbd_dn17 = assign100370_e152489_d_n17;
        locals.var_qbd_dn18 = assign100370_e152489_d_n18;
        locals.var_qbd_rv = 0.0;

        let assign100380_e152496: f64 = if ((p.p539 > 0.0) && (p.p543 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2311 = assign100380_e152496;
        locals.var_guard2311_rv = 0.0;

        let assign100390_e152503: f64 = if ((p.p539 > 0.0) && (p.p546 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2312 = assign100390_e152503;
        locals.var_guard2312_rv = 0.0;

        let assign100400_e152506: f64 = if p.p46 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2313 = assign100400_e152506;
        locals.var_guard2313_rv = 0.0;

        let assign100410_e152513: f64 = if ((locals.var_uc_sub1snp > 0.0) && (locals.var_uc_vmax > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2314 = assign100410_e152513;
        locals.var_guard2314_rv = 0.0;

        let (assign100420_e152521, assign100420_e152521_d_n0, assign100420_e152521_d_n2, assign100420_e152521_d_n4, assign100420_e152521_d_n5, assign100420_e152521_d_n6, assign100420_e152521_d_n7, assign100420_e152521_d_n8, assign100420_e152521_d_n9, assign100420_e152521_d_n10, assign100420_e152521_d_n11, assign100420_e152521_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100420_e152519: f64 = (locals.var_vg2const_1 * locals.var_vgp);
        (assign100420_e152519, ((locals.var_vg2const_1_dn0 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn0)), ((locals.var_vg2const_1_dn2 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn2)), ((locals.var_vg2const_1_dn4 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn4)), ((locals.var_vg2const_1_dn5 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn5)), ((locals.var_vg2const_1_dn6 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn6)), ((locals.var_vg2const_1_dn7 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn7)), ((locals.var_vg2const_1_dn8 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn8)), ((locals.var_vg2const_1_dn9 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn9)), ((locals.var_vg2const_1_dn10 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn10)), ((locals.var_vg2const_1_dn11 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn11)), ((locals.var_vg2const_1_dn14 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign100420_e152521;
        locals.var_t1_dn0 = assign100420_e152521_d_n0;
        locals.var_t1_dn2 = assign100420_e152521_d_n2;
        locals.var_t1_dn4 = assign100420_e152521_d_n4;
        locals.var_t1_dn5 = assign100420_e152521_d_n5;
        locals.var_t1_dn6 = assign100420_e152521_d_n6;
        locals.var_t1_dn7 = assign100420_e152521_d_n7;
        locals.var_t1_dn8 = assign100420_e152521_d_n8;
        locals.var_t1_dn9 = assign100420_e152521_d_n9;
        locals.var_t1_dn10 = assign100420_e152521_d_n10;
        locals.var_t1_dn11 = assign100420_e152521_d_n11;
        locals.var_t1_dn14 = assign100420_e152521_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign100430_e152531, assign100430_e152531_d_n0, assign100430_e152531_d_n2, assign100430_e152531_d_n4, assign100430_e152531_d_n5, assign100430_e152531_d_n6, assign100430_e152531_d_n7, assign100430_e152531_d_n8, assign100430_e152531_d_n9, assign100430_e152531_d_n10, assign100430_e152531_d_n11, assign100430_e152531_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100430_e152528: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100430_e152529: f64 = (locals.var_qnsub_esi / assign100430_e152528);
        (assign100430_e152529, (locals.var_qnsub_esi_dn0 / assign100430_e152528), (locals.var_qnsub_esi_dn2 / assign100430_e152528), (locals.var_qnsub_esi_dn4 / assign100430_e152528), (locals.var_qnsub_esi_dn5 / assign100430_e152528), (locals.var_qnsub_esi_dn6 / assign100430_e152528), (locals.var_qnsub_esi_dn7 / assign100430_e152528), (locals.var_qnsub_esi_dn8 / assign100430_e152528), (locals.var_qnsub_esi_dn9 / assign100430_e152528), (locals.var_qnsub_esi_dn10 / assign100430_e152528), (locals.var_qnsub_esi_dn11 / assign100430_e152528), (locals.var_qnsub_esi_dn14 / assign100430_e152528),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign100430_e152531;
        locals.var_t3_dn0 = assign100430_e152531_d_n0;
        locals.var_t3_dn2 = assign100430_e152531_d_n2;
        locals.var_t3_dn4 = assign100430_e152531_d_n4;
        locals.var_t3_dn5 = assign100430_e152531_d_n5;
        locals.var_t3_dn6 = assign100430_e152531_d_n6;
        locals.var_t3_dn7 = assign100430_e152531_d_n7;
        locals.var_t3_dn8 = assign100430_e152531_d_n8;
        locals.var_t3_dn9 = assign100430_e152531_d_n9;
        locals.var_t3_dn10 = assign100430_e152531_d_n10;
        locals.var_t3_dn11 = assign100430_e152531_d_n11;
        locals.var_t3_dn14 = assign100430_e152531_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign100440_e152543, assign100440_e152543_d_n0, assign100440_e152543_d_n2, assign100440_e152543_d_n4, assign100440_e152543_d_n5, assign100440_e152543_d_n6, assign100440_e152543_d_n7, assign100440_e152543_d_n8, assign100440_e152543_d_n9, assign100440_e152543_d_n10, assign100440_e152543_d_n11, assign100440_e152543_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100440_e152537: f64 = (2.0 / locals.var_qnsub_esi);
        let assign100440_e152540: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100440_e152541: f64 = (assign100440_e152537 * assign100440_e152540);
        (assign100440_e152541, ((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn4) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn5) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn8) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn9) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn11) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540), ((-((2.0 * locals.var_qnsub_esi_dn14) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100440_e152540),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign100440_e152543;
        locals.var_t4_dn0 = assign100440_e152543_d_n0;
        locals.var_t4_dn2 = assign100440_e152543_d_n2;
        locals.var_t4_dn4 = assign100440_e152543_d_n4;
        locals.var_t4_dn5 = assign100440_e152543_d_n5;
        locals.var_t4_dn6 = assign100440_e152543_d_n6;
        locals.var_t4_dn7 = assign100440_e152543_d_n7;
        locals.var_t4_dn8 = assign100440_e152543_d_n8;
        locals.var_t4_dn9 = assign100440_e152543_d_n9;
        locals.var_t4_dn10 = assign100440_e152543_d_n10;
        locals.var_t4_dn11 = assign100440_e152543_d_n11;
        locals.var_t4_dn14 = assign100440_e152543_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign100450_e152555, assign100450_e152555_d_n0, assign100450_e152555_d_n2, assign100450_e152555_d_n4, assign100450_e152555_d_n5, assign100450_e152555_d_n6, assign100450_e152555_d_n7, assign100450_e152555_d_n8, assign100450_e152555_d_n9, assign100450_e152555_d_n10, assign100450_e152555_d_n11, assign100450_e152555_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100450_e152549: f64 = (locals.var_t1 - locals.var_beta_inv);
        let assign100450_e152552: f64 = (locals.var_xvbs_1 * locals.var_vbsz__blk440);
        let assign100450_e152553: f64 = (assign100450_e152549 - assign100450_e152552);
        (assign100450_e152553, ((locals.var_t1_dn0 - locals.var_beta_inv_dn0) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn0)), ((locals.var_t1_dn2 - locals.var_beta_inv_dn2) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn2)), ((locals.var_t1_dn4 - locals.var_beta_inv_dn4) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn4)), ((locals.var_t1_dn5 - locals.var_beta_inv_dn5) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn5)), ((locals.var_t1_dn6 - locals.var_beta_inv_dn6) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn6)), ((locals.var_t1_dn7 - locals.var_beta_inv_dn7) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn7)), ((locals.var_t1_dn8 - locals.var_beta_inv_dn8) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn8)), ((locals.var_t1_dn9 - locals.var_beta_inv_dn9) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn9)), ((locals.var_t1_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn10)), ((locals.var_t1_dn11 - locals.var_beta_inv_dn11) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn11)), ((locals.var_t1_dn14 - locals.var_beta_inv_dn14) - (locals.var_xvbs_1 * locals.var_vbsz__blk440_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign100450_e152555;
        locals.var_t5_dn0 = assign100450_e152555_d_n0;
        locals.var_t5_dn2 = assign100450_e152555_d_n2;
        locals.var_t5_dn4 = assign100450_e152555_d_n4;
        locals.var_t5_dn5 = assign100450_e152555_d_n5;
        locals.var_t5_dn6 = assign100450_e152555_d_n6;
        locals.var_t5_dn7 = assign100450_e152555_d_n7;
        locals.var_t5_dn8 = assign100450_e152555_d_n8;
        locals.var_t5_dn9 = assign100450_e152555_d_n9;
        locals.var_t5_dn10 = assign100450_e152555_d_n10;
        locals.var_t5_dn11 = assign100450_e152555_d_n11;
        locals.var_t5_dn14 = assign100450_e152555_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign100460_e152565, assign100460_e152565_d_n0, assign100460_e152565_d_n2, assign100460_e152565_d_n4, assign100460_e152565_d_n5, assign100460_e152565_d_n6, assign100460_e152565_d_n7, assign100460_e152565_d_n8, assign100460_e152565_d_n9, assign100460_e152565_d_n10, assign100460_e152565_d_n11, assign100460_e152565_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100460_e152562: f64 = (locals.var_t4 * locals.var_t5);
        let assign100460_e152563: f64 = (1.0 + assign100460_e152562);
        (assign100460_e152563, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn14 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100460_e152565;
        locals.var_t6_dn0 = assign100460_e152565_d_n0;
        locals.var_t6_dn2 = assign100460_e152565_d_n2;
        locals.var_t6_dn4 = assign100460_e152565_d_n4;
        locals.var_t6_dn5 = assign100460_e152565_d_n5;
        locals.var_t6_dn6 = assign100460_e152565_d_n6;
        locals.var_t6_dn7 = assign100460_e152565_d_n7;
        locals.var_t6_dn8 = assign100460_e152565_d_n8;
        locals.var_t6_dn9 = assign100460_e152565_d_n9;
        locals.var_t6_dn10 = assign100460_e152565_d_n10;
        locals.var_t6_dn11 = assign100460_e152565_d_n11;
        locals.var_t6_dn14 = assign100460_e152565_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign100470_e152575, assign100470_e152575_d_n0, assign100470_e152575_d_n2, assign100470_e152575_d_n4, assign100470_e152575_d_n5, assign100470_e152575_d_n6, assign100470_e152575_d_n7, assign100470_e152575_d_n8, assign100470_e152575_d_n9, assign100470_e152575_d_n10, assign100470_e152575_d_n11, assign100470_e152575_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100470_e152572: f64 = (1.0 + locals.var_t4);
        let assign100470_e152573: f64 = (2.0 * assign100470_e152572);
        (assign100470_e152573, (2.0 * locals.var_t4_dn0), (2.0 * locals.var_t4_dn2), (2.0 * locals.var_t4_dn4), (2.0 * locals.var_t4_dn5), (2.0 * locals.var_t4_dn6), (2.0 * locals.var_t4_dn7), (2.0 * locals.var_t4_dn8), (2.0 * locals.var_t4_dn9), (2.0 * locals.var_t4_dn10), (2.0 * locals.var_t4_dn11), (2.0 * locals.var_t4_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign100470_e152575;
        locals.var_t7_dn0 = assign100470_e152575_d_n0;
        locals.var_t7_dn2 = assign100470_e152575_d_n2;
        locals.var_t7_dn4 = assign100470_e152575_d_n4;
        locals.var_t7_dn5 = assign100470_e152575_d_n5;
        locals.var_t7_dn6 = assign100470_e152575_d_n6;
        locals.var_t7_dn7 = assign100470_e152575_d_n7;
        locals.var_t7_dn8 = assign100470_e152575_d_n8;
        locals.var_t7_dn9 = assign100470_e152575_d_n9;
        locals.var_t7_dn10 = assign100470_e152575_d_n10;
        locals.var_t7_dn11 = assign100470_e152575_d_n11;
        locals.var_t7_dn14 = assign100470_e152575_d_n14;
        locals.var_t7_rv = 0.0;

        let assign100480_e152579: f64 = locals.var_t7;
        let assign100480_e152584: f64 = if ((locals.var_t6 < assign100480_e152579) && (locals.var_t7 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2315 = assign100480_e152584;
        locals.var_guard2315_rv = 0.0;

        let (assign100490_e152596, assign100490_e152596_d_n0, assign100490_e152596_d_n2, assign100490_e152596_d_n4, assign100490_e152596_d_n5, assign100490_e152596_d_n6, assign100490_e152596_d_n7, assign100490_e152596_d_n8, assign100490_e152596_d_n9, assign100490_e152596_d_n10, assign100490_e152596_d_n11, assign100490_e152596_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100490_e152592: f64 = locals.var_t7;
        let assign100490_e152594: f64 = (assign100490_e152592 - locals.var_t6);
        (assign100490_e152594, (locals.var_t7_dn0 - locals.var_t6_dn0), (locals.var_t7_dn2 - locals.var_t6_dn2), (locals.var_t7_dn4 - locals.var_t6_dn4), (locals.var_t7_dn5 - locals.var_t6_dn5), (locals.var_t7_dn6 - locals.var_t6_dn6), (locals.var_t7_dn7 - locals.var_t6_dn7), (locals.var_t7_dn8 - locals.var_t6_dn8), (locals.var_t7_dn9 - locals.var_t6_dn9), (locals.var_t7_dn10 - locals.var_t6_dn10), (locals.var_t7_dn11 - locals.var_t6_dn11), (locals.var_t7_dn14 - locals.var_t6_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign100490_e152596;
        locals.var_tmf1_dn0 = assign100490_e152596_d_n0;
        locals.var_tmf1_dn2 = assign100490_e152596_d_n2;
        locals.var_tmf1_dn4 = assign100490_e152596_d_n4;
        locals.var_tmf1_dn5 = assign100490_e152596_d_n5;
        locals.var_tmf1_dn6 = assign100490_e152596_d_n6;
        locals.var_tmf1_dn7 = assign100490_e152596_d_n7;
        locals.var_tmf1_dn8 = assign100490_e152596_d_n8;
        locals.var_tmf1_dn9 = assign100490_e152596_d_n9;
        locals.var_tmf1_dn10 = assign100490_e152596_d_n10;
        locals.var_tmf1_dn11 = assign100490_e152596_d_n11;
        locals.var_tmf1_dn14 = assign100490_e152596_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign100500_e152606, assign100500_e152606_d_n0, assign100500_e152606_d_n2, assign100500_e152606_d_n4, assign100500_e152606_d_n5, assign100500_e152606_d_n6, assign100500_e152606_d_n7, assign100500_e152606_d_n8, assign100500_e152606_d_n9, assign100500_e152606_d_n10, assign100500_e152606_d_n11, assign100500_e152606_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100500_e152604: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign100500_e152604, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign100500_e152606;
        locals.var_x2_dn0 = assign100500_e152606_d_n0;
        locals.var_x2_dn2 = assign100500_e152606_d_n2;
        locals.var_x2_dn4 = assign100500_e152606_d_n4;
        locals.var_x2_dn5 = assign100500_e152606_d_n5;
        locals.var_x2_dn6 = assign100500_e152606_d_n6;
        locals.var_x2_dn7 = assign100500_e152606_d_n7;
        locals.var_x2_dn8 = assign100500_e152606_d_n8;
        locals.var_x2_dn9 = assign100500_e152606_d_n9;
        locals.var_x2_dn10 = assign100500_e152606_d_n10;
        locals.var_x2_dn11 = assign100500_e152606_d_n11;
        locals.var_x2_dn14 = assign100500_e152606_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign100510_e152616, assign100510_e152616_d_n0, assign100510_e152616_d_n2, assign100510_e152616_d_n4, assign100510_e152616_d_n5, assign100510_e152616_d_n6, assign100510_e152616_d_n7, assign100510_e152616_d_n8, assign100510_e152616_d_n9, assign100510_e152616_d_n10, assign100510_e152616_d_n11, assign100510_e152616_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100510_e152614: f64 = (locals.var_t7 * locals.var_t7);
        (assign100510_e152614, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)), ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign100510_e152616;
        locals.var_xmax2_dn0 = assign100510_e152616_d_n0;
        locals.var_xmax2_dn2 = assign100510_e152616_d_n2;
        locals.var_xmax2_dn4 = assign100510_e152616_d_n4;
        locals.var_xmax2_dn5 = assign100510_e152616_d_n5;
        locals.var_xmax2_dn6 = assign100510_e152616_d_n6;
        locals.var_xmax2_dn7 = assign100510_e152616_d_n7;
        locals.var_xmax2_dn8 = assign100510_e152616_d_n8;
        locals.var_xmax2_dn9 = assign100510_e152616_d_n9;
        locals.var_xmax2_dn10 = assign100510_e152616_d_n10;
        locals.var_xmax2_dn11 = assign100510_e152616_d_n11;
        locals.var_xmax2_dn14 = assign100510_e152616_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign100520_e152624, assign100520_e152624_d_n0, assign100520_e152624_d_n2, assign100520_e152624_d_n4, assign100520_e152624_d_n5, assign100520_e152624_d_n6, assign100520_e152624_d_n7, assign100520_e152624_d_n8, assign100520_e152624_d_n9, assign100520_e152624_d_n10, assign100520_e152624_d_n11, assign100520_e152624_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100520_e152624;
        locals.var_xp_dn0 = assign100520_e152624_d_n0;
        locals.var_xp_dn2 = assign100520_e152624_d_n2;
        locals.var_xp_dn4 = assign100520_e152624_d_n4;
        locals.var_xp_dn5 = assign100520_e152624_d_n5;
        locals.var_xp_dn6 = assign100520_e152624_d_n6;
        locals.var_xp_dn7 = assign100520_e152624_d_n7;
        locals.var_xp_dn8 = assign100520_e152624_d_n8;
        locals.var_xp_dn9 = assign100520_e152624_d_n9;
        locals.var_xp_dn10 = assign100520_e152624_d_n10;
        locals.var_xp_dn11 = assign100520_e152624_d_n11;
        locals.var_xp_dn14 = assign100520_e152624_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign100530_e152632, assign100530_e152632_d_n0, assign100530_e152632_d_n2, assign100530_e152632_d_n4, assign100530_e152632_d_n5, assign100530_e152632_d_n6, assign100530_e152632_d_n7, assign100530_e152632_d_n8, assign100530_e152632_d_n9, assign100530_e152632_d_n10, assign100530_e152632_d_n11, assign100530_e152632_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100530_e152632;
        locals.var_xmp_dn0 = assign100530_e152632_d_n0;
        locals.var_xmp_dn2 = assign100530_e152632_d_n2;
        locals.var_xmp_dn4 = assign100530_e152632_d_n4;
        locals.var_xmp_dn5 = assign100530_e152632_d_n5;
        locals.var_xmp_dn6 = assign100530_e152632_d_n6;
        locals.var_xmp_dn7 = assign100530_e152632_d_n7;
        locals.var_xmp_dn8 = assign100530_e152632_d_n8;
        locals.var_xmp_dn9 = assign100530_e152632_d_n9;
        locals.var_xmp_dn10 = assign100530_e152632_d_n10;
        locals.var_xmp_dn11 = assign100530_e152632_d_n11;
        locals.var_xmp_dn14 = assign100530_e152632_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign100540_e152640,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100540_e152640;
        locals.var_m0_rv = 0.0;

        let (assign100550_e152648,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100550_e152648;
        locals.var_mm_rv = 0.0;

        let (assign100560_e152656, assign100560_e152656_d_n0, assign100560_e152656_d_n2, assign100560_e152656_d_n4, assign100560_e152656_d_n5, assign100560_e152656_d_n6, assign100560_e152656_d_n7, assign100560_e152656_d_n8, assign100560_e152656_d_n9, assign100560_e152656_d_n10, assign100560_e152656_d_n11, assign100560_e152656_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign100560_e152656;
        locals.var_arg_dn0 = assign100560_e152656_d_n0;
        locals.var_arg_dn2 = assign100560_e152656_d_n2;
        locals.var_arg_dn4 = assign100560_e152656_d_n4;
        locals.var_arg_dn5 = assign100560_e152656_d_n5;
        locals.var_arg_dn6 = assign100560_e152656_d_n6;
        locals.var_arg_dn7 = assign100560_e152656_d_n7;
        locals.var_arg_dn8 = assign100560_e152656_d_n8;
        locals.var_arg_dn9 = assign100560_e152656_d_n9;
        locals.var_arg_dn10 = assign100560_e152656_d_n10;
        locals.var_arg_dn11 = assign100560_e152656_d_n11;
        locals.var_arg_dn14 = assign100560_e152656_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign100570_e152664, assign100570_e152664_d_n0, assign100570_e152664_d_n2, assign100570_e152664_d_n4, assign100570_e152664_d_n5, assign100570_e152664_d_n6, assign100570_e152664_d_n7, assign100570_e152664_d_n8, assign100570_e152664_d_n9, assign100570_e152664_d_n10, assign100570_e152664_d_n11, assign100570_e152664_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100570_e152664;
        locals.var_dnm_dn0 = assign100570_e152664_d_n0;
        locals.var_dnm_dn2 = assign100570_e152664_d_n2;
        locals.var_dnm_dn4 = assign100570_e152664_d_n4;
        locals.var_dnm_dn5 = assign100570_e152664_d_n5;
        locals.var_dnm_dn6 = assign100570_e152664_d_n6;
        locals.var_dnm_dn7 = assign100570_e152664_d_n7;
        locals.var_dnm_dn8 = assign100570_e152664_d_n8;
        locals.var_dnm_dn9 = assign100570_e152664_d_n9;
        locals.var_dnm_dn10 = assign100570_e152664_d_n10;
        locals.var_dnm_dn11 = assign100570_e152664_d_n11;
        locals.var_dnm_dn14 = assign100570_e152664_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign100580_e152674, assign100580_e152674_d_n0, assign100580_e152674_d_n2, assign100580_e152674_d_n4, assign100580_e152674_d_n5, assign100580_e152674_d_n6, assign100580_e152674_d_n7, assign100580_e152674_d_n8, assign100580_e152674_d_n9, assign100580_e152674_d_n10, assign100580_e152674_d_n11, assign100580_e152674_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100580_e152672: f64 = (locals.var_xp * locals.var_x2);
        (assign100580_e152672, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100580_e152674;
        locals.var_xp_dn0 = assign100580_e152674_d_n0;
        locals.var_xp_dn2 = assign100580_e152674_d_n2;
        locals.var_xp_dn4 = assign100580_e152674_d_n4;
        locals.var_xp_dn5 = assign100580_e152674_d_n5;
        locals.var_xp_dn6 = assign100580_e152674_d_n6;
        locals.var_xp_dn7 = assign100580_e152674_d_n7;
        locals.var_xp_dn8 = assign100580_e152674_d_n8;
        locals.var_xp_dn9 = assign100580_e152674_d_n9;
        locals.var_xp_dn10 = assign100580_e152674_d_n10;
        locals.var_xp_dn11 = assign100580_e152674_d_n11;
        locals.var_xp_dn14 = assign100580_e152674_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign100590_e152684, assign100590_e152684_d_n0, assign100590_e152684_d_n2, assign100590_e152684_d_n4, assign100590_e152684_d_n5, assign100590_e152684_d_n6, assign100590_e152684_d_n7, assign100590_e152684_d_n8, assign100590_e152684_d_n9, assign100590_e152684_d_n10, assign100590_e152684_d_n11, assign100590_e152684_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100590_e152682: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100590_e152682, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100590_e152684;
        locals.var_xmp_dn0 = assign100590_e152684_d_n0;
        locals.var_xmp_dn2 = assign100590_e152684_d_n2;
        locals.var_xmp_dn4 = assign100590_e152684_d_n4;
        locals.var_xmp_dn5 = assign100590_e152684_d_n5;
        locals.var_xmp_dn6 = assign100590_e152684_d_n6;
        locals.var_xmp_dn7 = assign100590_e152684_d_n7;
        locals.var_xmp_dn8 = assign100590_e152684_d_n8;
        locals.var_xmp_dn9 = assign100590_e152684_d_n9;
        locals.var_xmp_dn10 = assign100590_e152684_d_n10;
        locals.var_xmp_dn11 = assign100590_e152684_d_n11;
        locals.var_xmp_dn14 = assign100590_e152684_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign100600_e152694, assign100600_e152694_d_n0, assign100600_e152694_d_n2, assign100600_e152694_d_n4, assign100600_e152694_d_n5, assign100600_e152694_d_n6, assign100600_e152694_d_n7, assign100600_e152694_d_n8, assign100600_e152694_d_n9, assign100600_e152694_d_n10, assign100600_e152694_d_n11, assign100600_e152694_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100600_e152692: f64 = (locals.var_xp * locals.var_x2);
        (assign100600_e152692, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100600_e152694;
        locals.var_xp_dn0 = assign100600_e152694_d_n0;
        locals.var_xp_dn2 = assign100600_e152694_d_n2;
        locals.var_xp_dn4 = assign100600_e152694_d_n4;
        locals.var_xp_dn5 = assign100600_e152694_d_n5;
        locals.var_xp_dn6 = assign100600_e152694_d_n6;
        locals.var_xp_dn7 = assign100600_e152694_d_n7;
        locals.var_xp_dn8 = assign100600_e152694_d_n8;
        locals.var_xp_dn9 = assign100600_e152694_d_n9;
        locals.var_xp_dn10 = assign100600_e152694_d_n10;
        locals.var_xp_dn11 = assign100600_e152694_d_n11;
        locals.var_xp_dn14 = assign100600_e152694_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign100610_e152704, assign100610_e152704_d_n0, assign100610_e152704_d_n2, assign100610_e152704_d_n4, assign100610_e152704_d_n5, assign100610_e152704_d_n6, assign100610_e152704_d_n7, assign100610_e152704_d_n8, assign100610_e152704_d_n9, assign100610_e152704_d_n10, assign100610_e152704_d_n11, assign100610_e152704_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100610_e152702: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100610_e152702, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100610_e152704;
        locals.var_xmp_dn0 = assign100610_e152704_d_n0;
        locals.var_xmp_dn2 = assign100610_e152704_d_n2;
        locals.var_xmp_dn4 = assign100610_e152704_d_n4;
        locals.var_xmp_dn5 = assign100610_e152704_d_n5;
        locals.var_xmp_dn6 = assign100610_e152704_d_n6;
        locals.var_xmp_dn7 = assign100610_e152704_d_n7;
        locals.var_xmp_dn8 = assign100610_e152704_d_n8;
        locals.var_xmp_dn9 = assign100610_e152704_d_n9;
        locals.var_xmp_dn10 = assign100610_e152704_d_n10;
        locals.var_xmp_dn11 = assign100610_e152704_d_n11;
        locals.var_xmp_dn14 = assign100610_e152704_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_387(
        locals: &mut StampLocals,
    ) {
        let (assign100620_e152714, assign100620_e152714_d_n0, assign100620_e152714_d_n2, assign100620_e152714_d_n4, assign100620_e152714_d_n5, assign100620_e152714_d_n6, assign100620_e152714_d_n7, assign100620_e152714_d_n8, assign100620_e152714_d_n9, assign100620_e152714_d_n10, assign100620_e152714_d_n11, assign100620_e152714_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100620_e152712: f64 = (locals.var_xp * locals.var_x2);
        (assign100620_e152712, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100620_e152714;
        locals.var_xp_dn0 = assign100620_e152714_d_n0;
        locals.var_xp_dn2 = assign100620_e152714_d_n2;
        locals.var_xp_dn4 = assign100620_e152714_d_n4;
        locals.var_xp_dn5 = assign100620_e152714_d_n5;
        locals.var_xp_dn6 = assign100620_e152714_d_n6;
        locals.var_xp_dn7 = assign100620_e152714_d_n7;
        locals.var_xp_dn8 = assign100620_e152714_d_n8;
        locals.var_xp_dn9 = assign100620_e152714_d_n9;
        locals.var_xp_dn10 = assign100620_e152714_d_n10;
        locals.var_xp_dn11 = assign100620_e152714_d_n11;
        locals.var_xp_dn14 = assign100620_e152714_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign100630_e152724, assign100630_e152724_d_n0, assign100630_e152724_d_n2, assign100630_e152724_d_n4, assign100630_e152724_d_n5, assign100630_e152724_d_n6, assign100630_e152724_d_n7, assign100630_e152724_d_n8, assign100630_e152724_d_n9, assign100630_e152724_d_n10, assign100630_e152724_d_n11, assign100630_e152724_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100630_e152722: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100630_e152722, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100630_e152724;
        locals.var_xmp_dn0 = assign100630_e152724_d_n0;
        locals.var_xmp_dn2 = assign100630_e152724_d_n2;
        locals.var_xmp_dn4 = assign100630_e152724_d_n4;
        locals.var_xmp_dn5 = assign100630_e152724_d_n5;
        locals.var_xmp_dn6 = assign100630_e152724_d_n6;
        locals.var_xmp_dn7 = assign100630_e152724_d_n7;
        locals.var_xmp_dn8 = assign100630_e152724_d_n8;
        locals.var_xmp_dn9 = assign100630_e152724_d_n9;
        locals.var_xmp_dn10 = assign100630_e152724_d_n10;
        locals.var_xmp_dn11 = assign100630_e152724_d_n11;
        locals.var_xmp_dn14 = assign100630_e152724_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign100640_e152734, assign100640_e152734_d_n0, assign100640_e152734_d_n2, assign100640_e152734_d_n4, assign100640_e152734_d_n5, assign100640_e152734_d_n6, assign100640_e152734_d_n7, assign100640_e152734_d_n8, assign100640_e152734_d_n9, assign100640_e152734_d_n10, assign100640_e152734_d_n11, assign100640_e152734_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100640_e152732: f64 = (locals.var_xp * locals.var_x2);
        (assign100640_e152732, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100640_e152734;
        locals.var_xp_dn0 = assign100640_e152734_d_n0;
        locals.var_xp_dn2 = assign100640_e152734_d_n2;
        locals.var_xp_dn4 = assign100640_e152734_d_n4;
        locals.var_xp_dn5 = assign100640_e152734_d_n5;
        locals.var_xp_dn6 = assign100640_e152734_d_n6;
        locals.var_xp_dn7 = assign100640_e152734_d_n7;
        locals.var_xp_dn8 = assign100640_e152734_d_n8;
        locals.var_xp_dn9 = assign100640_e152734_d_n9;
        locals.var_xp_dn10 = assign100640_e152734_d_n10;
        locals.var_xp_dn11 = assign100640_e152734_d_n11;
        locals.var_xp_dn14 = assign100640_e152734_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign100650_e152744, assign100650_e152744_d_n0, assign100650_e152744_d_n2, assign100650_e152744_d_n4, assign100650_e152744_d_n5, assign100650_e152744_d_n6, assign100650_e152744_d_n7, assign100650_e152744_d_n8, assign100650_e152744_d_n9, assign100650_e152744_d_n10, assign100650_e152744_d_n11, assign100650_e152744_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100650_e152742: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100650_e152742, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100650_e152744;
        locals.var_xmp_dn0 = assign100650_e152744_d_n0;
        locals.var_xmp_dn2 = assign100650_e152744_d_n2;
        locals.var_xmp_dn4 = assign100650_e152744_d_n4;
        locals.var_xmp_dn5 = assign100650_e152744_d_n5;
        locals.var_xmp_dn6 = assign100650_e152744_d_n6;
        locals.var_xmp_dn7 = assign100650_e152744_d_n7;
        locals.var_xmp_dn8 = assign100650_e152744_d_n8;
        locals.var_xmp_dn9 = assign100650_e152744_d_n9;
        locals.var_xmp_dn10 = assign100650_e152744_d_n10;
        locals.var_xmp_dn11 = assign100650_e152744_d_n11;
        locals.var_xmp_dn14 = assign100650_e152744_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign100660_e152754, assign100660_e152754_d_n0, assign100660_e152754_d_n2, assign100660_e152754_d_n4, assign100660_e152754_d_n5, assign100660_e152754_d_n6, assign100660_e152754_d_n7, assign100660_e152754_d_n8, assign100660_e152754_d_n9, assign100660_e152754_d_n10, assign100660_e152754_d_n11, assign100660_e152754_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100660_e152752: f64 = (locals.var_xp + locals.var_xmp);
        (assign100660_e152752, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign100660_e152754;
        locals.var_arg_dn0 = assign100660_e152754_d_n0;
        locals.var_arg_dn2 = assign100660_e152754_d_n2;
        locals.var_arg_dn4 = assign100660_e152754_d_n4;
        locals.var_arg_dn5 = assign100660_e152754_d_n5;
        locals.var_arg_dn6 = assign100660_e152754_d_n6;
        locals.var_arg_dn7 = assign100660_e152754_d_n7;
        locals.var_arg_dn8 = assign100660_e152754_d_n8;
        locals.var_arg_dn9 = assign100660_e152754_d_n9;
        locals.var_arg_dn10 = assign100660_e152754_d_n10;
        locals.var_arg_dn11 = assign100660_e152754_d_n11;
        locals.var_arg_dn14 = assign100660_e152754_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign100670_e152762, assign100670_e152762_d_n0, assign100670_e152762_d_n2, assign100670_e152762_d_n4, assign100670_e152762_d_n5, assign100670_e152762_d_n6, assign100670_e152762_d_n7, assign100670_e152762_d_n8, assign100670_e152762_d_n9, assign100670_e152762_d_n10, assign100670_e152762_d_n11, assign100670_e152762_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100670_e152762;
        locals.var_dnm_dn0 = assign100670_e152762_d_n0;
        locals.var_dnm_dn2 = assign100670_e152762_d_n2;
        locals.var_dnm_dn4 = assign100670_e152762_d_n4;
        locals.var_dnm_dn5 = assign100670_e152762_d_n5;
        locals.var_dnm_dn6 = assign100670_e152762_d_n6;
        locals.var_dnm_dn7 = assign100670_e152762_d_n7;
        locals.var_dnm_dn8 = assign100670_e152762_d_n8;
        locals.var_dnm_dn9 = assign100670_e152762_d_n9;
        locals.var_dnm_dn10 = assign100670_e152762_d_n10;
        locals.var_dnm_dn11 = assign100670_e152762_d_n11;
        locals.var_dnm_dn14 = assign100670_e152762_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign100680_e152777: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2316 = assign100680_e152777;
        locals.var_guard2316_rv = 0.0;

        let assign100690_e152780: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2317 = assign100690_e152780;
        locals.var_guard2317_rv = 0.0;

        let (assign100700_e152792,) = {
    if (((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100700_e152792;
        locals.var_mm_rv = 0.0;

        let assign100710_e152795: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2318 = assign100710_e152795;
        locals.var_guard2318_rv = 0.0;

        let (assign100720_e152810,) = {
    if ((((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 == 0.0)) && (locals.var_guard2318 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100720_e152810;
        locals.var_mm_rv = 0.0;

        let assign100730_e152813: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2319 = assign100730_e152813;
        locals.var_guard2319_rv = 0.0;

        let (assign100740_e152831,) = {
    if (((((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 == 0.0)) && (locals.var_guard2318 == 0.0)) && (locals.var_guard2319 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100740_e152831;
        locals.var_mm_rv = 0.0;

        let assign100750_e152834: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2320 = assign100750_e152834;
        locals.var_guard2320_rv = 0.0;

        let (assign100760_e152855,) = {
    if ((((((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 == 0.0)) && (locals.var_guard2318 == 0.0)) && (locals.var_guard2319 == 0.0)) && (locals.var_guard2320 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100760_e152855;
        locals.var_mm_rv = 0.0;

        let (assign100770_e152865,) = {
    if ((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100770_e152865;
        locals.var_m0_rv = 0.0;

        let mut assign100780_loop_guard: usize = 0;
        while {
            let assign100780_cond_e152876: f64 = if (((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign100780_cond_e152876 != 0.0
        } {
            assign100780_loop_guard += 1;
            assert!(assign100780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign100780_body0_e152887, assign100780_body0_e152887_d_n0, assign100780_body0_e152887_d_n2, assign100780_body0_e152887_d_n4, assign100780_body0_e152887_d_n5, assign100780_body0_e152887_d_n6, assign100780_body0_e152887_d_n7, assign100780_body0_e152887_d_n8, assign100780_body0_e152887_d_n9, assign100780_body0_e152887_d_n10, assign100780_body0_e152887_d_n11, assign100780_body0_e152887_d_n14,) = {
    if ((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) {
        let assign100780_body0_e152885: f64 = (locals.var_dnm).sqrt();
        (assign100780_body0_e152885, (locals.var_dnm_dn0 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn2 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn4 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn5 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn6 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn7 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn8 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn9 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn10 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn11 / (2.0 * assign100780_body0_e152885)), (locals.var_dnm_dn14 / (2.0 * assign100780_body0_e152885)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign100780_body0_e152887;
            locals.var_dnm_dn0 = assign100780_body0_e152887_d_n0;
            locals.var_dnm_dn2 = assign100780_body0_e152887_d_n2;
            locals.var_dnm_dn4 = assign100780_body0_e152887_d_n4;
            locals.var_dnm_dn5 = assign100780_body0_e152887_d_n5;
            locals.var_dnm_dn6 = assign100780_body0_e152887_d_n6;
            locals.var_dnm_dn7 = assign100780_body0_e152887_d_n7;
            locals.var_dnm_dn8 = assign100780_body0_e152887_d_n8;
            locals.var_dnm_dn9 = assign100780_body0_e152887_d_n9;
            locals.var_dnm_dn10 = assign100780_body0_e152887_d_n10;
            locals.var_dnm_dn11 = assign100780_body0_e152887_d_n11;
            locals.var_dnm_dn14 = assign100780_body0_e152887_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign100780_body1_e152899,) = {
    if ((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 != 0.0)) {
        let assign100780_body1_e152897: f64 = (locals.var_m0 + 1.0);
        (assign100780_body1_e152897,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign100780_body1_e152899;
            locals.var_m0_rv = 0.0;
        }

        let (assign100790_e152921, assign100790_e152921_d_n0, assign100790_e152921_d_n2, assign100790_e152921_d_n4, assign100790_e152921_d_n5, assign100790_e152921_d_n6, assign100790_e152921_d_n7, assign100790_e152921_d_n8, assign100790_e152921_d_n9, assign100790_e152921_d_n10, assign100790_e152921_d_n11, assign100790_e152921_d_n14,) = {
    if ((((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) && (locals.var_guard2316 == 0.0)) {
        let (assign100790_e152919, assign100790_e152919_d_n0, assign100790_e152919_d_n2, assign100790_e152919_d_n4, assign100790_e152919_d_n5, assign100790_e152919_d_n6, assign100790_e152919_d_n7, assign100790_e152919_d_n8, assign100790_e152919_d_n9, assign100790_e152919_d_n10, assign100790_e152919_d_n11, assign100790_e152919_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign100790_e152916: f64 = (2.0 * 4.0);
                let assign100790_e152917: f64 = (1.0 / assign100790_e152916);
                let assign100790_e152918: f64 = (locals.var_dnm).powf(assign100790_e152917);
                (assign100790_e152918, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn0)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn2)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn4)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn5)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn6)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn7)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn8)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn9)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn10)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn11)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100790_e152917) as f64).is_finite() && ((assign100790_e152917) as f64).fract() == 0.0 { if assign100790_e152917 == 0.0 { 0.0 } else { (assign100790_e152917 * ((locals.var_dnm).powf(assign100790_e152917 - 1.0) * locals.var_dnm_dn14)) } } else { (assign100790_e152918 * (assign100790_e152917 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign100790_e152919, assign100790_e152919_d_n0, assign100790_e152919_d_n2, assign100790_e152919_d_n4, assign100790_e152919_d_n5, assign100790_e152919_d_n6, assign100790_e152919_d_n7, assign100790_e152919_d_n8, assign100790_e152919_d_n9, assign100790_e152919_d_n10, assign100790_e152919_d_n11, assign100790_e152919_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100790_e152921;
        locals.var_dnm_dn0 = assign100790_e152921_d_n0;
        locals.var_dnm_dn2 = assign100790_e152921_d_n2;
        locals.var_dnm_dn4 = assign100790_e152921_d_n4;
        locals.var_dnm_dn5 = assign100790_e152921_d_n5;
        locals.var_dnm_dn6 = assign100790_e152921_d_n6;
        locals.var_dnm_dn7 = assign100790_e152921_d_n7;
        locals.var_dnm_dn8 = assign100790_e152921_d_n8;
        locals.var_dnm_dn9 = assign100790_e152921_d_n9;
        locals.var_dnm_dn10 = assign100790_e152921_d_n10;
        locals.var_dnm_dn11 = assign100790_e152921_d_n11;
        locals.var_dnm_dn14 = assign100790_e152921_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign100800_e152931, assign100800_e152931_d_n0, assign100800_e152931_d_n2, assign100800_e152931_d_n4, assign100800_e152931_d_n5, assign100800_e152931_d_n6, assign100800_e152931_d_n7, assign100800_e152931_d_n8, assign100800_e152931_d_n9, assign100800_e152931_d_n10, assign100800_e152931_d_n11, assign100800_e152931_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100800_e152929: f64 = (1.0 / locals.var_dnm);
        (assign100800_e152929, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100800_e152931;
        locals.var_dnm_dn0 = assign100800_e152931_d_n0;
        locals.var_dnm_dn2 = assign100800_e152931_d_n2;
        locals.var_dnm_dn4 = assign100800_e152931_d_n4;
        locals.var_dnm_dn5 = assign100800_e152931_d_n5;
        locals.var_dnm_dn6 = assign100800_e152931_d_n6;
        locals.var_dnm_dn7 = assign100800_e152931_d_n7;
        locals.var_dnm_dn8 = assign100800_e152931_d_n8;
        locals.var_dnm_dn9 = assign100800_e152931_d_n9;
        locals.var_dnm_dn10 = assign100800_e152931_d_n10;
        locals.var_dnm_dn11 = assign100800_e152931_d_n11;
        locals.var_dnm_dn14 = assign100800_e152931_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign100810_e152943, assign100810_e152943_d_n0, assign100810_e152943_d_n2, assign100810_e152943_d_n4, assign100810_e152943_d_n5, assign100810_e152943_d_n6, assign100810_e152943_d_n7, assign100810_e152943_d_n8, assign100810_e152943_d_n9, assign100810_e152943_d_n10, assign100810_e152943_d_n11, assign100810_e152943_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100810_e152939: f64 = (locals.var_tmf1 * locals.var_t7);
        let assign100810_e152941: f64 = (assign100810_e152939 * locals.var_dnm);
        (assign100810_e152941, ((((locals.var_tmf1_dn0 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn0)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn2)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn4)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn5)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn6)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn7)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn8)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn9)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn10)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn11)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn14)) * locals.var_dnm) + (assign100810_e152939 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign100810_e152943;
        locals.var_tmf0_dn0 = assign100810_e152943_d_n0;
        locals.var_tmf0_dn2 = assign100810_e152943_d_n2;
        locals.var_tmf0_dn4 = assign100810_e152943_d_n4;
        locals.var_tmf0_dn5 = assign100810_e152943_d_n5;
        locals.var_tmf0_dn6 = assign100810_e152943_d_n6;
        locals.var_tmf0_dn7 = assign100810_e152943_d_n7;
        locals.var_tmf0_dn8 = assign100810_e152943_d_n8;
        locals.var_tmf0_dn9 = assign100810_e152943_d_n9;
        locals.var_tmf0_dn10 = assign100810_e152943_d_n10;
        locals.var_tmf0_dn11 = assign100810_e152943_d_n11;
        locals.var_tmf0_dn14 = assign100810_e152943_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign100820_e152957, assign100820_e152957_d_n0, assign100820_e152957_d_n2, assign100820_e152957_d_n4, assign100820_e152957_d_n5, assign100820_e152957_d_n6, assign100820_e152957_d_n7, assign100820_e152957_d_n8, assign100820_e152957_d_n9, assign100820_e152957_d_n10, assign100820_e152957_d_n11, assign100820_e152957_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100820_e152951: f64 = (locals.var_t7 * locals.var_xmp);
        let assign100820_e152953: f64 = (assign100820_e152951 * locals.var_dnm);
        let assign100820_e152955: f64 = (assign100820_e152953 / locals.var_arg);
        (assign100820_e152955, (((((((locals.var_t7_dn0 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn0)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn2 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn2)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn4 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn4)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn5 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn5)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn6 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn6)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn7 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn7)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn8 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn8)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn9 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn9)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn10 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn10)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn11 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn11)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn14 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign100820_e152951 * locals.var_dnm_dn14)) * locals.var_arg) - (assign100820_e152953 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100820_e152957;
        locals.var_t0_dn0 = assign100820_e152957_d_n0;
        locals.var_t0_dn2 = assign100820_e152957_d_n2;
        locals.var_t0_dn4 = assign100820_e152957_d_n4;
        locals.var_t0_dn5 = assign100820_e152957_d_n5;
        locals.var_t0_dn6 = assign100820_e152957_d_n6;
        locals.var_t0_dn7 = assign100820_e152957_d_n7;
        locals.var_t0_dn8 = assign100820_e152957_d_n8;
        locals.var_t0_dn9 = assign100820_e152957_d_n9;
        locals.var_t0_dn10 = assign100820_e152957_d_n10;
        locals.var_t0_dn11 = assign100820_e152957_d_n11;
        locals.var_t0_dn14 = assign100820_e152957_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign100830_e152969, assign100830_e152969_d_n0, assign100830_e152969_d_n2, assign100830_e152969_d_n4, assign100830_e152969_d_n5, assign100830_e152969_d_n6, assign100830_e152969_d_n7, assign100830_e152969_d_n8, assign100830_e152969_d_n9, assign100830_e152969_d_n10, assign100830_e152969_d_n11, assign100830_e152969_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        let assign100830_e152965: f64 = locals.var_t7;
        let assign100830_e152967: f64 = (assign100830_e152965 - locals.var_tmf0);
        (assign100830_e152967, (locals.var_t7_dn0 - locals.var_tmf0_dn0), (locals.var_t7_dn2 - locals.var_tmf0_dn2), (locals.var_t7_dn4 - locals.var_tmf0_dn4), (locals.var_t7_dn5 - locals.var_tmf0_dn5), (locals.var_t7_dn6 - locals.var_tmf0_dn6), (locals.var_t7_dn7 - locals.var_tmf0_dn7), (locals.var_t7_dn8 - locals.var_tmf0_dn8), (locals.var_t7_dn9 - locals.var_tmf0_dn9), (locals.var_t7_dn10 - locals.var_tmf0_dn10), (locals.var_t7_dn11 - locals.var_tmf0_dn11), (locals.var_t7_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100830_e152969;
        locals.var_t6_dn0 = assign100830_e152969_d_n0;
        locals.var_t6_dn2 = assign100830_e152969_d_n2;
        locals.var_t6_dn4 = assign100830_e152969_d_n4;
        locals.var_t6_dn5 = assign100830_e152969_d_n5;
        locals.var_t6_dn6 = assign100830_e152969_d_n6;
        locals.var_t6_dn7 = assign100830_e152969_d_n7;
        locals.var_t6_dn8 = assign100830_e152969_d_n8;
        locals.var_t6_dn9 = assign100830_e152969_d_n9;
        locals.var_t6_dn10 = assign100830_e152969_d_n10;
        locals.var_t6_dn11 = assign100830_e152969_d_n11;
        locals.var_t6_dn14 = assign100830_e152969_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign100840_e152977, assign100840_e152977_d_n0, assign100840_e152977_d_n2, assign100840_e152977_d_n4, assign100840_e152977_d_n5, assign100840_e152977_d_n6, assign100840_e152977_d_n7, assign100840_e152977_d_n8, assign100840_e152977_d_n9, assign100840_e152977_d_n10, assign100840_e152977_d_n11, assign100840_e152977_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100840_e152977;
        locals.var_t0_dn0 = assign100840_e152977_d_n0;
        locals.var_t0_dn2 = assign100840_e152977_d_n2;
        locals.var_t0_dn4 = assign100840_e152977_d_n4;
        locals.var_t0_dn5 = assign100840_e152977_d_n5;
        locals.var_t0_dn6 = assign100840_e152977_d_n6;
        locals.var_t0_dn7 = assign100840_e152977_d_n7;
        locals.var_t0_dn8 = assign100840_e152977_d_n8;
        locals.var_t0_dn9 = assign100840_e152977_d_n9;
        locals.var_t0_dn10 = assign100840_e152977_d_n10;
        locals.var_t0_dn11 = assign100840_e152977_d_n11;
        locals.var_t0_dn14 = assign100840_e152977_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign100850_e152986, assign100850_e152986_d_n0, assign100850_e152986_d_n2, assign100850_e152986_d_n4, assign100850_e152986_d_n5, assign100850_e152986_d_n6, assign100850_e152986_d_n7, assign100850_e152986_d_n8, assign100850_e152986_d_n9, assign100850_e152986_d_n10, assign100850_e152986_d_n11, assign100850_e152986_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100850_e152986;
        locals.var_t6_dn0 = assign100850_e152986_d_n0;
        locals.var_t6_dn2 = assign100850_e152986_d_n2;
        locals.var_t6_dn4 = assign100850_e152986_d_n4;
        locals.var_t6_dn5 = assign100850_e152986_d_n5;
        locals.var_t6_dn6 = assign100850_e152986_d_n6;
        locals.var_t6_dn7 = assign100850_e152986_d_n7;
        locals.var_t6_dn8 = assign100850_e152986_d_n8;
        locals.var_t6_dn9 = assign100850_e152986_d_n9;
        locals.var_t6_dn10 = assign100850_e152986_d_n10;
        locals.var_t6_dn11 = assign100850_e152986_d_n11;
        locals.var_t6_dn14 = assign100850_e152986_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign100860_e152995, assign100860_e152995_d_n0, assign100860_e152995_d_n2, assign100860_e152995_d_n4, assign100860_e152995_d_n5, assign100860_e152995_d_n6, assign100860_e152995_d_n7, assign100860_e152995_d_n8, assign100860_e152995_d_n9, assign100860_e152995_d_n10, assign100860_e152995_d_n11, assign100860_e152995_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100860_e152995;
        locals.var_t0_dn0 = assign100860_e152995_d_n0;
        locals.var_t0_dn2 = assign100860_e152995_d_n2;
        locals.var_t0_dn4 = assign100860_e152995_d_n4;
        locals.var_t0_dn5 = assign100860_e152995_d_n5;
        locals.var_t0_dn6 = assign100860_e152995_d_n6;
        locals.var_t0_dn7 = assign100860_e152995_d_n7;
        locals.var_t0_dn8 = assign100860_e152995_d_n8;
        locals.var_t0_dn9 = assign100860_e152995_d_n9;
        locals.var_t0_dn10 = assign100860_e152995_d_n10;
        locals.var_t0_dn11 = assign100860_e152995_d_n11;
        locals.var_t0_dn14 = assign100860_e152995_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign100870_e153002, assign100870_e153002_d_n0, assign100870_e153002_d_n2, assign100870_e153002_d_n4, assign100870_e153002_d_n5, assign100870_e153002_d_n6, assign100870_e153002_d_n7, assign100870_e153002_d_n8, assign100870_e153002_d_n9, assign100870_e153002_d_n10, assign100870_e153002_d_n11, assign100870_e153002_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100870_e153000: f64 = (locals.var_t6).sqrt();
        (assign100870_e153000, (locals.var_t6_dn0 / (2.0 * assign100870_e153000)), (locals.var_t6_dn2 / (2.0 * assign100870_e153000)), (locals.var_t6_dn4 / (2.0 * assign100870_e153000)), (locals.var_t6_dn5 / (2.0 * assign100870_e153000)), (locals.var_t6_dn6 / (2.0 * assign100870_e153000)), (locals.var_t6_dn7 / (2.0 * assign100870_e153000)), (locals.var_t6_dn8 / (2.0 * assign100870_e153000)), (locals.var_t6_dn9 / (2.0 * assign100870_e153000)), (locals.var_t6_dn10 / (2.0 * assign100870_e153000)), (locals.var_t6_dn11 / (2.0 * assign100870_e153000)), (locals.var_t6_dn14 / (2.0 * assign100870_e153000)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100870_e153002;
        locals.var_t6_dn0 = assign100870_e153002_d_n0;
        locals.var_t6_dn2 = assign100870_e153002_d_n2;
        locals.var_t6_dn4 = assign100870_e153002_d_n4;
        locals.var_t6_dn5 = assign100870_e153002_d_n5;
        locals.var_t6_dn6 = assign100870_e153002_d_n6;
        locals.var_t6_dn7 = assign100870_e153002_d_n7;
        locals.var_t6_dn8 = assign100870_e153002_d_n8;
        locals.var_t6_dn9 = assign100870_e153002_d_n9;
        locals.var_t6_dn10 = assign100870_e153002_d_n10;
        locals.var_t6_dn11 = assign100870_e153002_d_n11;
        locals.var_t6_dn14 = assign100870_e153002_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign100880_e153014, assign100880_e153014_d_n0, assign100880_e153014_d_n2, assign100880_e153014_d_n4, assign100880_e153014_d_n5, assign100880_e153014_d_n6, assign100880_e153014_d_n7, assign100880_e153014_d_n8, assign100880_e153014_d_n9, assign100880_e153014_d_n10, assign100880_e153014_d_n11, assign100880_e153014_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100880_e153010: f64 = (1.0 - locals.var_t6);
        let assign100880_e153011: f64 = (locals.var_t3 * assign100880_e153010);
        let assign100880_e153012: f64 = (locals.var_t1 + assign100880_e153011);
        (assign100880_e153012, (locals.var_t1_dn0 + ((locals.var_t3_dn0 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn0)))), (locals.var_t1_dn2 + ((locals.var_t3_dn2 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn2)))), (locals.var_t1_dn4 + ((locals.var_t3_dn4 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn4)))), (locals.var_t1_dn5 + ((locals.var_t3_dn5 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn5)))), (locals.var_t1_dn6 + ((locals.var_t3_dn6 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn6)))), (locals.var_t1_dn7 + ((locals.var_t3_dn7 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn7)))), (locals.var_t1_dn8 + ((locals.var_t3_dn8 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn8)))), (locals.var_t1_dn9 + ((locals.var_t3_dn9 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn9)))), (locals.var_t1_dn10 + ((locals.var_t3_dn10 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn10)))), (locals.var_t1_dn11 + ((locals.var_t3_dn11 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn11)))), (locals.var_t1_dn14 + ((locals.var_t3_dn14 * assign100880_e153010) + (locals.var_t3 * (-locals.var_t6_dn14)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn4, locals.var_psislsat_dn5, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn8, locals.var_psislsat_dn9, locals.var_psislsat_dn10, locals.var_psislsat_dn11, locals.var_psislsat_dn14,)
    }
};
        locals.var_psislsat = assign100880_e153014;
        locals.var_psislsat_dn0 = assign100880_e153014_d_n0;
        locals.var_psislsat_dn2 = assign100880_e153014_d_n2;
        locals.var_psislsat_dn4 = assign100880_e153014_d_n4;
        locals.var_psislsat_dn5 = assign100880_e153014_d_n5;
        locals.var_psislsat_dn6 = assign100880_e153014_d_n6;
        locals.var_psislsat_dn7 = assign100880_e153014_d_n7;
        locals.var_psislsat_dn8 = assign100880_e153014_d_n8;
        locals.var_psislsat_dn9 = assign100880_e153014_d_n9;
        locals.var_psislsat_dn10 = assign100880_e153014_d_n10;
        locals.var_psislsat_dn11 = assign100880_e153014_d_n11;
        locals.var_psislsat_dn14 = assign100880_e153014_d_n14;
        locals.var_psislsat_rv = 0.0;

        let (assign100890_e153024, assign100890_e153024_d_n0, assign100890_e153024_d_n2, assign100890_e153024_d_n4, assign100890_e153024_d_n5, assign100890_e153024_d_n6, assign100890_e153024_d_n7, assign100890_e153024_d_n8, assign100890_e153024_d_n9, assign100890_e153024_d_n10, assign100890_e153024_d_n11, assign100890_e153024_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100890_e153021: f64 = (locals.var_xgate_1 + locals.var_lgate);
        let assign100890_e153022: f64 = (locals.var_lgate / assign100890_e153021);
        (assign100890_e153022, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign100890_e153024;
        locals.var_t2_dn0 = assign100890_e153024_d_n0;
        locals.var_t2_dn2 = assign100890_e153024_d_n2;
        locals.var_t2_dn4 = assign100890_e153024_d_n4;
        locals.var_t2_dn5 = assign100890_e153024_d_n5;
        locals.var_t2_dn6 = assign100890_e153024_d_n6;
        locals.var_t2_dn7 = assign100890_e153024_d_n7;
        locals.var_t2_dn8 = assign100890_e153024_d_n8;
        locals.var_t2_dn9 = assign100890_e153024_d_n9;
        locals.var_t2_dn10 = assign100890_e153024_d_n10;
        locals.var_t2_dn11 = assign100890_e153024_d_n11;
        locals.var_t2_dn14 = assign100890_e153024_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_388(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign100900_e153038, assign100900_e153038_d_n0, assign100900_e153038_d_n2, assign100900_e153038_d_n4, assign100900_e153038_d_n5, assign100900_e153038_d_n6, assign100900_e153038_d_n7, assign100900_e153038_d_n8, assign100900_e153038_d_n9, assign100900_e153038_d_n10, assign100900_e153038_d_n11, assign100900_e153038_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100900_e153030: f64 = (locals.var_uc_svdssnp * locals.var_vdsz__blk441);
        let assign100900_e153032: f64 = (assign100900_e153030 + locals.var_ps0z);
        let assign100900_e153035: f64 = (locals.var_t2 * locals.var_psislsat);
        let assign100900_e153036: f64 = (assign100900_e153032 - assign100900_e153035);
        (assign100900_e153036, (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2_dn0 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn0))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2_dn2 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn2))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn4) + locals.var_ps0z_dn4) - ((locals.var_t2_dn4 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn4))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn5) + locals.var_ps0z_dn5) - ((locals.var_t2_dn5 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn5))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2_dn6 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn6))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2_dn7 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn7))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn8) + locals.var_ps0z_dn8) - ((locals.var_t2_dn8 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn8))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn9) + locals.var_ps0z_dn9) - ((locals.var_t2_dn9 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn9))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2_dn10 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn10))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn11) + locals.var_ps0z_dn11) - ((locals.var_t2_dn11 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn11))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk441_dn14) + locals.var_ps0z_dn14) - ((locals.var_t2_dn14 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn14))),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100900_e153038;
        locals.var_psisubsat_dn0 = assign100900_e153038_d_n0;
        locals.var_psisubsat_dn2 = assign100900_e153038_d_n2;
        locals.var_psisubsat_dn4 = assign100900_e153038_d_n4;
        locals.var_psisubsat_dn5 = assign100900_e153038_d_n5;
        locals.var_psisubsat_dn6 = assign100900_e153038_d_n6;
        locals.var_psisubsat_dn7 = assign100900_e153038_d_n7;
        locals.var_psisubsat_dn8 = assign100900_e153038_d_n8;
        locals.var_psisubsat_dn9 = assign100900_e153038_d_n9;
        locals.var_psisubsat_dn10 = assign100900_e153038_d_n10;
        locals.var_psisubsat_dn11 = assign100900_e153038_d_n11;
        locals.var_psisubsat_dn14 = assign100900_e153038_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let (assign100910_e153053, assign100910_e153053_d_n0, assign100910_e153053_d_n2, assign100910_e153053_d_n4, assign100910_e153053_d_n5, assign100910_e153053_d_n6, assign100910_e153053_d_n7, assign100910_e153053_d_n8, assign100910_e153053_d_n9, assign100910_e153053_d_n10, assign100910_e153053_d_n11, assign100910_e153053_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100910_e153044: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign100910_e153047: f64 = (4.0 * 0.001);
        let assign100910_e153049: f64 = (assign100910_e153047 * 0.001);
        let assign100910_e153050: f64 = (assign100910_e153044 + assign100910_e153049);
        let assign100910_e153051: f64 = (assign100910_e153050).sqrt();
        (assign100910_e153051, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn4 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn4)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn5 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn5)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn8 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn8)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn9 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn9)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn11 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn11)) / (2.0 * assign100910_e153051)), (((locals.var_psisubsat_dn14 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn14)) / (2.0 * assign100910_e153051)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100910_e153053;
        locals.var_tmf2_dn0 = assign100910_e153053_d_n0;
        locals.var_tmf2_dn2 = assign100910_e153053_d_n2;
        locals.var_tmf2_dn4 = assign100910_e153053_d_n4;
        locals.var_tmf2_dn5 = assign100910_e153053_d_n5;
        locals.var_tmf2_dn6 = assign100910_e153053_d_n6;
        locals.var_tmf2_dn7 = assign100910_e153053_d_n7;
        locals.var_tmf2_dn8 = assign100910_e153053_d_n8;
        locals.var_tmf2_dn9 = assign100910_e153053_d_n9;
        locals.var_tmf2_dn10 = assign100910_e153053_d_n10;
        locals.var_tmf2_dn11 = assign100910_e153053_d_n11;
        locals.var_tmf2_dn14 = assign100910_e153053_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign100920_e153065, assign100920_e153065_d_n0, assign100920_e153065_d_n2, assign100920_e153065_d_n4, assign100920_e153065_d_n5, assign100920_e153065_d_n6, assign100920_e153065_d_n7, assign100920_e153065_d_n8, assign100920_e153065_d_n9, assign100920_e153065_d_n10, assign100920_e153065_d_n11, assign100920_e153065_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100920_e153061: f64 = (locals.var_psisubsat / locals.var_tmf2);
        let assign100920_e153062: f64 = (1.0 + assign100920_e153061);
        let assign100920_e153063: f64 = (0.5 * assign100920_e153062);
        (assign100920_e153063, (0.5 * (((locals.var_psisubsat_dn0 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn2 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn4 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn5 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn6 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn7 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn8 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn9 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn10 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn11 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn14 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign100920_e153065;
        locals.var_t9_dn0 = assign100920_e153065_d_n0;
        locals.var_t9_dn2 = assign100920_e153065_d_n2;
        locals.var_t9_dn4 = assign100920_e153065_d_n4;
        locals.var_t9_dn5 = assign100920_e153065_d_n5;
        locals.var_t9_dn6 = assign100920_e153065_d_n6;
        locals.var_t9_dn7 = assign100920_e153065_d_n7;
        locals.var_t9_dn8 = assign100920_e153065_d_n8;
        locals.var_t9_dn9 = assign100920_e153065_d_n9;
        locals.var_t9_dn10 = assign100920_e153065_d_n10;
        locals.var_t9_dn11 = assign100920_e153065_d_n11;
        locals.var_t9_dn14 = assign100920_e153065_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign100930_e153075, assign100930_e153075_d_n0, assign100930_e153075_d_n2, assign100930_e153075_d_n4, assign100930_e153075_d_n5, assign100930_e153075_d_n6, assign100930_e153075_d_n7, assign100930_e153075_d_n8, assign100930_e153075_d_n9, assign100930_e153075_d_n10, assign100930_e153075_d_n11, assign100930_e153075_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100930_e153072: f64 = (locals.var_psisubsat + locals.var_tmf2);
        let assign100930_e153073: f64 = (0.5 * assign100930_e153072);
        (assign100930_e153073, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_psisubsat_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_psisubsat_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_psisubsat_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_psisubsat_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_psisubsat_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_psisubsat_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100930_e153075;
        locals.var_psisubsat_dn0 = assign100930_e153075_d_n0;
        locals.var_psisubsat_dn2 = assign100930_e153075_d_n2;
        locals.var_psisubsat_dn4 = assign100930_e153075_d_n4;
        locals.var_psisubsat_dn5 = assign100930_e153075_d_n5;
        locals.var_psisubsat_dn6 = assign100930_e153075_d_n6;
        locals.var_psisubsat_dn7 = assign100930_e153075_d_n7;
        locals.var_psisubsat_dn8 = assign100930_e153075_d_n8;
        locals.var_psisubsat_dn9 = assign100930_e153075_d_n9;
        locals.var_psisubsat_dn10 = assign100930_e153075_d_n10;
        locals.var_psisubsat_dn11 = assign100930_e153075_d_n11;
        locals.var_psisubsat_dn14 = assign100930_e153075_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let assign100940_e153078: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2321 = assign100940_e153078;
        locals.var_guard2321_rv = 0.0;

        let (assign100950_e153086, assign100950_e153086_d_n0, assign100950_e153086_d_n2, assign100950_e153086_d_n4, assign100950_e153086_d_n5, assign100950_e153086_d_n6, assign100950_e153086_d_n7, assign100950_e153086_d_n8, assign100950_e153086_d_n9, assign100950_e153086_d_n10, assign100950_e153086_d_n11, assign100950_e153086_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2321 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100950_e153086;
        locals.var_psisubsat_dn0 = assign100950_e153086_d_n0;
        locals.var_psisubsat_dn2 = assign100950_e153086_d_n2;
        locals.var_psisubsat_dn4 = assign100950_e153086_d_n4;
        locals.var_psisubsat_dn5 = assign100950_e153086_d_n5;
        locals.var_psisubsat_dn6 = assign100950_e153086_d_n6;
        locals.var_psisubsat_dn7 = assign100950_e153086_d_n7;
        locals.var_psisubsat_dn8 = assign100950_e153086_d_n8;
        locals.var_psisubsat_dn9 = assign100950_e153086_d_n9;
        locals.var_psisubsat_dn10 = assign100950_e153086_d_n10;
        locals.var_psisubsat_dn11 = assign100950_e153086_d_n11;
        locals.var_psisubsat_dn14 = assign100950_e153086_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let (assign100960_e153094, assign100960_e153094_d_n0, assign100960_e153094_d_n2, assign100960_e153094_d_n4, assign100960_e153094_d_n5, assign100960_e153094_d_n6, assign100960_e153094_d_n7, assign100960_e153094_d_n8, assign100960_e153094_d_n9, assign100960_e153094_d_n10, assign100960_e153094_d_n11, assign100960_e153094_d_n14,) = {
    if (((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2321 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign100960_e153094;
        locals.var_t9_dn0 = assign100960_e153094_d_n0;
        locals.var_t9_dn2 = assign100960_e153094_d_n2;
        locals.var_t9_dn4 = assign100960_e153094_d_n4;
        locals.var_t9_dn5 = assign100960_e153094_d_n5;
        locals.var_t9_dn6 = assign100960_e153094_d_n6;
        locals.var_t9_dn7 = assign100960_e153094_d_n7;
        locals.var_t9_dn8 = assign100960_e153094_d_n8;
        locals.var_t9_dn9 = assign100960_e153094_d_n9;
        locals.var_t9_dn10 = assign100960_e153094_d_n10;
        locals.var_t9_dn11 = assign100960_e153094_d_n11;
        locals.var_t9_dn14 = assign100960_e153094_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign100970_e153102, assign100970_e153102_d_n0, assign100970_e153102_d_n2, assign100970_e153102_d_n4, assign100970_e153102_d_n5, assign100970_e153102_d_n6, assign100970_e153102_d_n7, assign100970_e153102_d_n8, assign100970_e153102_d_n9, assign100970_e153102_d_n10, assign100970_e153102_d_n11, assign100970_e153102_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100970_e153100: f64 = (locals.var_psisubsat + 1e-25);
        (assign100970_e153100, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100970_e153102;
        locals.var_psisubsat_dn0 = assign100970_e153102_d_n0;
        locals.var_psisubsat_dn2 = assign100970_e153102_d_n2;
        locals.var_psisubsat_dn4 = assign100970_e153102_d_n4;
        locals.var_psisubsat_dn5 = assign100970_e153102_d_n5;
        locals.var_psisubsat_dn6 = assign100970_e153102_d_n6;
        locals.var_psisubsat_dn7 = assign100970_e153102_d_n7;
        locals.var_psisubsat_dn8 = assign100970_e153102_d_n8;
        locals.var_psisubsat_dn9 = assign100970_e153102_d_n9;
        locals.var_psisubsat_dn10 = assign100970_e153102_d_n10;
        locals.var_psisubsat_dn11 = assign100970_e153102_d_n11;
        locals.var_psisubsat_dn14 = assign100970_e153102_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let (assign100980_e153114, assign100980_e153114_d_n0, assign100980_e153114_d_n2, assign100980_e153114_d_n4, assign100980_e153114_d_n5, assign100980_e153114_d_n6, assign100980_e153114_d_n7, assign100980_e153114_d_n8, assign100980_e153114_d_n9, assign100980_e153114_d_n10, assign100980_e153114_d_n11, assign100980_e153114_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign100980_e153110: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign100980_e153111: f64 = (locals.var_uc_subtmp * assign100980_e153110);
        let assign100980_e153112: f64 = (1.0 + assign100980_e153111);
        (assign100980_e153112, (locals.var_uc_subtmp * locals.var_ttemp_dn0), (locals.var_uc_subtmp * locals.var_ttemp_dn2), (locals.var_uc_subtmp * locals.var_ttemp_dn4), (locals.var_uc_subtmp * locals.var_ttemp_dn5), (locals.var_uc_subtmp * locals.var_ttemp_dn6), (locals.var_uc_subtmp * locals.var_ttemp_dn7), (locals.var_uc_subtmp * locals.var_ttemp_dn8), (locals.var_uc_subtmp * locals.var_ttemp_dn9), (locals.var_uc_subtmp * locals.var_ttemp_dn10), (locals.var_uc_subtmp * locals.var_ttemp_dn11), (locals.var_uc_subtmp * locals.var_ttemp_dn14),)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
    }
};
        locals.var_xsubtmp = assign100980_e153114;
        locals.var_xsubtmp_dn0 = assign100980_e153114_d_n0;
        locals.var_xsubtmp_dn2 = assign100980_e153114_d_n2;
        locals.var_xsubtmp_dn4 = assign100980_e153114_d_n4;
        locals.var_xsubtmp_dn5 = assign100980_e153114_d_n5;
        locals.var_xsubtmp_dn6 = assign100980_e153114_d_n6;
        locals.var_xsubtmp_dn7 = assign100980_e153114_d_n7;
        locals.var_xsubtmp_dn8 = assign100980_e153114_d_n8;
        locals.var_xsubtmp_dn9 = assign100980_e153114_d_n9;
        locals.var_xsubtmp_dn10 = assign100980_e153114_d_n10;
        locals.var_xsubtmp_dn11 = assign100980_e153114_d_n11;
        locals.var_xsubtmp_dn14 = assign100980_e153114_d_n14;
        locals.var_xsubtmp_rv = 0.0;

        let (assign100990_e153125, assign100990_e153125_d_n0, assign100990_e153125_d_n2, assign100990_e153125_d_n4, assign100990_e153125_d_n5, assign100990_e153125_d_n6, assign100990_e153125_d_n7, assign100990_e153125_d_n8, assign100990_e153125_d_n9, assign100990_e153125_d_n10, assign100990_e153125_d_n11, assign100990_e153125_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let (assign100990_e153123, assign100990_e153123_d_n0, assign100990_e153123_d_n2, assign100990_e153123_d_n4, assign100990_e153123_d_n5, assign100990_e153123_d_n6, assign100990_e153123_d_n7, assign100990_e153123_d_n8, assign100990_e153123_d_n9, assign100990_e153123_d_n10, assign100990_e153123_d_n11, assign100990_e153123_d_n14,) = {
            if (locals.var_xsubtmp <= 0.001) {
                (0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
            }
        };
        (assign100990_e153123, assign100990_e153123_d_n0, assign100990_e153123_d_n2, assign100990_e153123_d_n4, assign100990_e153123_d_n5, assign100990_e153123_d_n6, assign100990_e153123_d_n7, assign100990_e153123_d_n8, assign100990_e153123_d_n9, assign100990_e153123_d_n10, assign100990_e153123_d_n11, assign100990_e153123_d_n14,)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
    }
};
        locals.var_xsubtmp = assign100990_e153125;
        locals.var_xsubtmp_dn0 = assign100990_e153125_d_n0;
        locals.var_xsubtmp_dn2 = assign100990_e153125_d_n2;
        locals.var_xsubtmp_dn4 = assign100990_e153125_d_n4;
        locals.var_xsubtmp_dn5 = assign100990_e153125_d_n5;
        locals.var_xsubtmp_dn6 = assign100990_e153125_d_n6;
        locals.var_xsubtmp_dn7 = assign100990_e153125_d_n7;
        locals.var_xsubtmp_dn8 = assign100990_e153125_d_n8;
        locals.var_xsubtmp_dn9 = assign100990_e153125_d_n9;
        locals.var_xsubtmp_dn10 = assign100990_e153125_d_n10;
        locals.var_xsubtmp_dn11 = assign100990_e153125_d_n11;
        locals.var_xsubtmp_dn14 = assign100990_e153125_d_n14;
        locals.var_xsubtmp_rv = 0.0;

        let (assign101000_e153133, assign101000_e153133_d_n0, assign101000_e153133_d_n2, assign101000_e153133_d_n4, assign101000_e153133_d_n5, assign101000_e153133_d_n6, assign101000_e153133_d_n7, assign101000_e153133_d_n8, assign101000_e153133_d_n9, assign101000_e153133_d_n10, assign101000_e153133_d_n11, assign101000_e153133_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign101000_e153131: f64 = (locals.var_xsub1_1 / locals.var_xsubtmp);
        (assign101000_e153131, (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn0) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn2) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn4) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn5) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn6) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn7) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn8) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn9) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn10) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn11) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn14) / (locals.var_xsubtmp * locals.var_xsubtmp))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101000_e153133;
        locals.var_t5_dn0 = assign101000_e153133_d_n0;
        locals.var_t5_dn2 = assign101000_e153133_d_n2;
        locals.var_t5_dn4 = assign101000_e153133_d_n4;
        locals.var_t5_dn5 = assign101000_e153133_d_n5;
        locals.var_t5_dn6 = assign101000_e153133_d_n6;
        locals.var_t5_dn7 = assign101000_e153133_d_n7;
        locals.var_t5_dn8 = assign101000_e153133_d_n8;
        locals.var_t5_dn9 = assign101000_e153133_d_n9;
        locals.var_t5_dn10 = assign101000_e153133_d_n10;
        locals.var_t5_dn11 = assign101000_e153133_d_n11;
        locals.var_t5_dn14 = assign101000_e153133_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign101010_e153141, assign101010_e153141_d_n0, assign101010_e153141_d_n2, assign101010_e153141_d_n4, assign101010_e153141_d_n5, assign101010_e153141_d_n6, assign101010_e153141_d_n7, assign101010_e153141_d_n8, assign101010_e153141_d_n9, assign101010_e153141_d_n10, assign101010_e153141_d_n11, assign101010_e153141_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign101010_e153139: f64 = (locals.var_xsub2_1 * locals.var_xsubtmp);
        (assign101010_e153139, (locals.var_xsub2_1 * locals.var_xsubtmp_dn0), (locals.var_xsub2_1 * locals.var_xsubtmp_dn2), (locals.var_xsub2_1 * locals.var_xsubtmp_dn4), (locals.var_xsub2_1 * locals.var_xsubtmp_dn5), (locals.var_xsub2_1 * locals.var_xsubtmp_dn6), (locals.var_xsub2_1 * locals.var_xsubtmp_dn7), (locals.var_xsub2_1 * locals.var_xsubtmp_dn8), (locals.var_xsub2_1 * locals.var_xsubtmp_dn9), (locals.var_xsub2_1 * locals.var_xsubtmp_dn10), (locals.var_xsub2_1 * locals.var_xsubtmp_dn11), (locals.var_xsub2_1 * locals.var_xsubtmp_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign101010_e153141;
        locals.var_t6_dn0 = assign101010_e153141_d_n0;
        locals.var_t6_dn2 = assign101010_e153141_d_n2;
        locals.var_t6_dn4 = assign101010_e153141_d_n4;
        locals.var_t6_dn5 = assign101010_e153141_d_n5;
        locals.var_t6_dn6 = assign101010_e153141_d_n6;
        locals.var_t6_dn7 = assign101010_e153141_d_n7;
        locals.var_t6_dn8 = assign101010_e153141_d_n8;
        locals.var_t6_dn9 = assign101010_e153141_d_n9;
        locals.var_t6_dn10 = assign101010_e153141_d_n10;
        locals.var_t6_dn11 = assign101010_e153141_d_n11;
        locals.var_t6_dn14 = assign101010_e153141_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign101020_e153151, assign101020_e153151_d_n0, assign101020_e153151_d_n2, assign101020_e153151_d_n4, assign101020_e153151_d_n5, assign101020_e153151_d_n6, assign101020_e153151_d_n7, assign101020_e153151_d_n8, assign101020_e153151_d_n9, assign101020_e153151_d_n10, assign101020_e153151_d_n11, assign101020_e153151_d_n14,) = {
    if ((locals.var_guard2313 != 0.0) && (locals.var_guard2314 != 0.0)) {
        let assign101020_e153146: f64 = (-locals.var_t6);
        let assign101020_e153148: f64 = (assign101020_e153146 / locals.var_psisubsat);
        let assign101020_e153149: f64 = (assign101020_e153148).exp();
        (assign101020_e153149, (assign101020_e153149 * ((((-locals.var_t6_dn0) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn0)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn2) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn2)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn4) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn4)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn5) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn5)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn6) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn6)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn7) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn7)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn8) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn8)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn9) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn9)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn10) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn10)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn11) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn11)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101020_e153149 * ((((-locals.var_t6_dn14) * locals.var_psisubsat) - (assign101020_e153146 * locals.var_psisubsat_dn14)) / (locals.var_psisubsat * locals.var_psisubsat))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101020_e153151;
        locals.var_t2_dn0 = assign101020_e153151_d_n0;
        locals.var_t2_dn2 = assign101020_e153151_d_n2;
        locals.var_t2_dn4 = assign101020_e153151_d_n4;
        locals.var_t2_dn5 = assign101020_e153151_d_n5;
        locals.var_t2_dn6 = assign101020_e153151_d_n6;
        locals.var_t2_dn7 = assign101020_e153151_d_n7;
        locals.var_t2_dn8 = assign101020_e153151_d_n8;
        locals.var_t2_dn9 = assign101020_e153151_d_n9;
        locals.var_t2_dn10 = assign101020_e153151_d_n10;
        locals.var_t2_dn11 = assign101020_e153151_d_n11;
        locals.var_t2_dn14 = assign101020_e153151_d_n14;
        locals.var_t2_rv = 0.0;

        let assign101070_e153192: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2323 = assign101070_e153192;
        locals.var_guard2323_rv = 0.0;

        let (assign101080_e153198, assign101080_e153198_d_n0, assign101080_e153198_d_n2, assign101080_e153198_d_n4, assign101080_e153198_d_n5, assign101080_e153198_d_n6, assign101080_e153198_d_n7, assign101080_e153198_d_n8, assign101080_e153198_d_n9, assign101080_e153198_d_n10, assign101080_e153198_d_n11, assign101080_e153198_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2323 != 0.0)) {
        (p.p270, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign101080_e153198;
        locals.var_t12_dn0 = assign101080_e153198_d_n0;
        locals.var_t12_dn2 = assign101080_e153198_d_n2;
        locals.var_t12_dn4 = assign101080_e153198_d_n4;
        locals.var_t12_dn5 = assign101080_e153198_d_n5;
        locals.var_t12_dn6 = assign101080_e153198_d_n6;
        locals.var_t12_dn7 = assign101080_e153198_d_n7;
        locals.var_t12_dn8 = assign101080_e153198_d_n8;
        locals.var_t12_dn9 = assign101080_e153198_d_n9;
        locals.var_t12_dn10 = assign101080_e153198_d_n10;
        locals.var_t12_dn11 = assign101080_e153198_d_n11;
        locals.var_t12_dn14 = assign101080_e153198_d_n14;
        locals.var_t12_rv = 0.0;

        let (assign101090_e153204, assign101090_e153204_d_n0, assign101090_e153204_d_n2, assign101090_e153204_d_n4, assign101090_e153204_d_n5, assign101090_e153204_d_n6, assign101090_e153204_d_n7, assign101090_e153204_d_n8, assign101090_e153204_d_n9, assign101090_e153204_d_n10, assign101090_e153204_d_n11, assign101090_e153204_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2323 != 0.0)) {
        (p.p271, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101090_e153204;
        locals.var_t10_dn0 = assign101090_e153204_d_n0;
        locals.var_t10_dn2 = assign101090_e153204_d_n2;
        locals.var_t10_dn4 = assign101090_e153204_d_n4;
        locals.var_t10_dn5 = assign101090_e153204_d_n5;
        locals.var_t10_dn6 = assign101090_e153204_d_n6;
        locals.var_t10_dn7 = assign101090_e153204_d_n7;
        locals.var_t10_dn8 = assign101090_e153204_d_n8;
        locals.var_t10_dn9 = assign101090_e153204_d_n9;
        locals.var_t10_dn10 = assign101090_e153204_d_n10;
        locals.var_t10_dn11 = assign101090_e153204_d_n11;
        locals.var_t10_dn14 = assign101090_e153204_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign101100_e153210, assign101100_e153210_d_n0, assign101100_e153210_d_n2, assign101100_e153210_d_n4, assign101100_e153210_d_n5, assign101100_e153210_d_n6, assign101100_e153210_d_n7, assign101100_e153210_d_n8, assign101100_e153210_d_n9, assign101100_e153210_d_n10, assign101100_e153210_d_n11, assign101100_e153210_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2323 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101100_e153210;
        locals.var_t3_dn0 = assign101100_e153210_d_n0;
        locals.var_t3_dn2 = assign101100_e153210_d_n2;
        locals.var_t3_dn4 = assign101100_e153210_d_n4;
        locals.var_t3_dn5 = assign101100_e153210_d_n5;
        locals.var_t3_dn6 = assign101100_e153210_d_n6;
        locals.var_t3_dn7 = assign101100_e153210_d_n7;
        locals.var_t3_dn8 = assign101100_e153210_d_n8;
        locals.var_t3_dn9 = assign101100_e153210_d_n9;
        locals.var_t3_dn10 = assign101100_e153210_d_n10;
        locals.var_t3_dn11 = assign101100_e153210_d_n11;
        locals.var_t3_dn14 = assign101100_e153210_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign101110_e153222, assign101110_e153222_d_n0, assign101110_e153222_d_n2, assign101110_e153222_d_n4, assign101110_e153222_d_n5, assign101110_e153222_d_n6, assign101110_e153222_d_n7, assign101110_e153222_d_n8, assign101110_e153222_d_n9, assign101110_e153222_d_n10, assign101110_e153222_d_n11, assign101110_e153222_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2323 != 0.0)) {
        let assign101110_e153216: f64 = (locals.var_t12 * locals.var_t10);
        let assign101110_e153218: f64 = (assign101110_e153216 * locals.var_t3);
        let assign101110_e153220: f64 = (assign101110_e153218 * locals.var_t3);
        (assign101110_e153220, ((((((locals.var_t12_dn0 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn0)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn0)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn0)), ((((((locals.var_t12_dn2 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn2)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn2)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn2)), ((((((locals.var_t12_dn4 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn4)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn4)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn4)), ((((((locals.var_t12_dn5 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn5)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn5)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn5)), ((((((locals.var_t12_dn6 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn6)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn6)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn6)), ((((((locals.var_t12_dn7 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn7)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn7)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn7)), ((((((locals.var_t12_dn8 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn8)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn8)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn8)), ((((((locals.var_t12_dn9 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn9)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn9)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn9)), ((((((locals.var_t12_dn10 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn10)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn10)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn10)), ((((((locals.var_t12_dn11 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn11)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn11)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn11)), ((((((locals.var_t12_dn14 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn14)) * locals.var_t3) + (assign101110_e153216 * locals.var_t3_dn14)) * locals.var_t3) + (assign101110_e153218 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign101110_e153222;
        locals.var_t1_dn0 = assign101110_e153222_d_n0;
        locals.var_t1_dn2 = assign101110_e153222_d_n2;
        locals.var_t1_dn4 = assign101110_e153222_d_n4;
        locals.var_t1_dn5 = assign101110_e153222_d_n5;
        locals.var_t1_dn6 = assign101110_e153222_d_n6;
        locals.var_t1_dn7 = assign101110_e153222_d_n7;
        locals.var_t1_dn8 = assign101110_e153222_d_n8;
        locals.var_t1_dn9 = assign101110_e153222_d_n9;
        locals.var_t1_dn10 = assign101110_e153222_d_n10;
        locals.var_t1_dn11 = assign101110_e153222_d_n11;
        locals.var_t1_dn14 = assign101110_e153222_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign101120_e153240, assign101120_e153240_d_n0, assign101120_e153240_d_n2, assign101120_e153240_d_n4, assign101120_e153240_d_n5, assign101120_e153240_d_n6, assign101120_e153240_d_n7, assign101120_e153240_d_n8, assign101120_e153240_d_n9, assign101120_e153240_d_n10, assign101120_e153240_d_n11, assign101120_e153240_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2323 != 0.0)) {
        let assign101120_e153228: f64 = (locals.var_mu * locals.var_vgvt);
        let assign101120_e153230: f64 = (assign101120_e153228 * locals.var_t12);
        let assign101120_e153233: f64 = (locals.var_t10 * locals.var_t3);
        let assign101120_e153235: f64 = (assign101120_e153233 * locals.var_t3);
        let assign101120_e153236: f64 = (assign101120_e153230 + assign101120_e153235);
        let assign101120_e153238: f64 = (assign101120_e153236 + 1e-25);
        (assign101120_e153238, (((((locals.var_mu_dn0 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn0)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn0)) + ((((locals.var_t10_dn0 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn0)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn0))), (((((locals.var_mu_dn2 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn2)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn2)) + ((((locals.var_t10_dn2 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn2)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn2))), (((((locals.var_mu_dn4 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn4)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn4)) + ((((locals.var_t10_dn4 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn4)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn4))), (((((locals.var_mu_dn5 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn5)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn5)) + ((((locals.var_t10_dn5 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn5)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn5))), (((((locals.var_mu_dn6 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn6)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn6)) + ((((locals.var_t10_dn6 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn6)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn6))), (((((locals.var_mu_dn7 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn7)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn7)) + ((((locals.var_t10_dn7 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn7)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn7))), (((((locals.var_mu_dn8 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn8)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn8)) + ((((locals.var_t10_dn8 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn8)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn8))), (((((locals.var_mu_dn9 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn9)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn9)) + ((((locals.var_t10_dn9 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn9)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn9))), (((((locals.var_mu_dn10 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn10)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn10)) + ((((locals.var_t10_dn10 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn10)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn10))), (((((locals.var_mu_dn11 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn11)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn11)) + ((((locals.var_t10_dn11 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn11)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn11))), (((((locals.var_mu_dn14 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn14)) * locals.var_t12) + (assign101120_e153228 * locals.var_t12_dn14)) + ((((locals.var_t10_dn14 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn14)) * locals.var_t3) + (assign101120_e153233 * locals.var_t3_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101120_e153240;
        locals.var_t2_dn0 = assign101120_e153240_d_n0;
        locals.var_t2_dn2 = assign101120_e153240_d_n2;
        locals.var_t2_dn4 = assign101120_e153240_d_n4;
        locals.var_t2_dn5 = assign101120_e153240_d_n5;
        locals.var_t2_dn6 = assign101120_e153240_d_n6;
        locals.var_t2_dn7 = assign101120_e153240_d_n7;
        locals.var_t2_dn8 = assign101120_e153240_d_n8;
        locals.var_t2_dn9 = assign101120_e153240_d_n9;
        locals.var_t2_dn10 = assign101120_e153240_d_n10;
        locals.var_t2_dn11 = assign101120_e153240_d_n11;
        locals.var_t2_dn14 = assign101120_e153240_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign101150_e153259, assign101150_e153259_d_n0, assign101150_e153259_d_n2, assign101150_e153259_d_n4, assign101150_e153259_d_n5, assign101150_e153259_d_n6, assign101150_e153259_d_n7, assign101150_e153259_d_n8, assign101150_e153259_d_n9, assign101150_e153259_d_n10, assign101150_e153259_d_n11, assign101150_e153259_d_n14,) = {
    if (locals.var_flg_nqs != 0.0) {
        (locals.var_mks_dly3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101150_e153259;
        locals.var_t2_dn0 = assign101150_e153259_d_n0;
        locals.var_t2_dn2 = assign101150_e153259_d_n2;
        locals.var_t2_dn4 = assign101150_e153259_d_n4;
        locals.var_t2_dn5 = assign101150_e153259_d_n5;
        locals.var_t2_dn6 = assign101150_e153259_d_n6;
        locals.var_t2_dn7 = assign101150_e153259_d_n7;
        locals.var_t2_dn8 = assign101150_e153259_d_n8;
        locals.var_t2_dn9 = assign101150_e153259_d_n9;
        locals.var_t2_dn10 = assign101150_e153259_d_n10;
        locals.var_t2_dn11 = assign101150_e153259_d_n11;
        locals.var_t2_dn14 = assign101150_e153259_d_n14;
        locals.var_t2_rv = 0.0;

        let assign101170_e153271: f64 = if ((p.p26 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2324 = assign101170_e153271;
        locals.var_guard2324_rv = 0.0;

        let (assign101180_e153275,) = {
    if (locals.var_guard2324 != 0.0) {
        (locals.var_uc_nfalp,)
    } else {
        (locals.var_nfalpe,)
    }
};
        locals.var_nfalpe = assign101180_e153275;
        locals.var_nfalpe_rv = 0.0;

        let (assign101200_e153283,) = {
    if (locals.var_guard2324 != 0.0) {
        (locals.var_mks_cit,)
    } else {
        (locals.var_cite,)
    }
};
        locals.var_cite = assign101200_e153283;
        locals.var_cite_rv = 0.0;

        let (assign101210_e153289, assign101210_e153289_d_n0, assign101210_e153289_d_n2, assign101210_e153289_d_n4, assign101210_e153289_d_n5, assign101210_e153289_d_n6, assign101210_e153289_d_n7, assign101210_e153289_d_n8, assign101210_e153289_d_n9, assign101210_e153289_d_n10, assign101210_e153289_d_n11, assign101210_e153289_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101210_e153287: f64 = (locals.var_qn0 / 1.6021918e-19);
        (assign101210_e153287, (locals.var_qn0_dn0 / 1.6021918e-19), (locals.var_qn0_dn2 / 1.6021918e-19), (locals.var_qn0_dn4 / 1.6021918e-19), (locals.var_qn0_dn5 / 1.6021918e-19), (locals.var_qn0_dn6 / 1.6021918e-19), (locals.var_qn0_dn7 / 1.6021918e-19), (locals.var_qn0_dn8 / 1.6021918e-19), (locals.var_qn0_dn9 / 1.6021918e-19), (locals.var_qn0_dn10 / 1.6021918e-19), (locals.var_qn0_dn11 / 1.6021918e-19), (locals.var_qn0_dn14 / 1.6021918e-19),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign101210_e153289;
        locals.var_t1_dn0 = assign101210_e153289_d_n0;
        locals.var_t1_dn2 = assign101210_e153289_d_n2;
        locals.var_t1_dn4 = assign101210_e153289_d_n4;
        locals.var_t1_dn5 = assign101210_e153289_d_n5;
        locals.var_t1_dn6 = assign101210_e153289_d_n6;
        locals.var_t1_dn7 = assign101210_e153289_d_n7;
        locals.var_t1_dn8 = assign101210_e153289_d_n8;
        locals.var_t1_dn9 = assign101210_e153289_d_n9;
        locals.var_t1_dn10 = assign101210_e153289_d_n10;
        locals.var_t1_dn11 = assign101210_e153289_d_n11;
        locals.var_t1_dn14 = assign101210_e153289_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign101220_e153306, assign101220_e153306_d_n0, assign101220_e153306_d_n2, assign101220_e153306_d_n4, assign101220_e153306_d_n5, assign101220_e153306_d_n6, assign101220_e153306_d_n7, assign101220_e153306_d_n8, assign101220_e153306_d_n9, assign101220_e153306_d_n10, assign101220_e153306_d_n11, assign101220_e153306_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101220_e153293: f64 = (locals.var_ps0 - locals.var_vbscl__blk437);
        let assign101220_e153296: f64 = (locals.var_ps0 - locals.var_vbscl__blk437);
        let assign101220_e153297: f64 = (assign101220_e153293 * assign101220_e153296);
        let assign101220_e153300: f64 = (4.0 * 0.001);
        let assign101220_e153302: f64 = (assign101220_e153300 * 0.001);
        let assign101220_e153303: f64 = (assign101220_e153297 + assign101220_e153302);
        let assign101220_e153304: f64 = (assign101220_e153303).sqrt();
        (assign101220_e153304, ((((locals.var_ps0_dn0 - locals.var_vbscl__blk437_dn0) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn0 - locals.var_vbscl__blk437_dn0))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn2 - locals.var_vbscl__blk437_dn2) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn2 - locals.var_vbscl__blk437_dn2))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn4 - locals.var_vbscl__blk437_dn4) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn4 - locals.var_vbscl__blk437_dn4))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn5 - locals.var_vbscl__blk437_dn5) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn5 - locals.var_vbscl__blk437_dn5))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn6 - locals.var_vbscl__blk437_dn6) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn6 - locals.var_vbscl__blk437_dn6))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn7 - locals.var_vbscl__blk437_dn7) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn7 - locals.var_vbscl__blk437_dn7))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn8 - locals.var_vbscl__blk437_dn8) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn8 - locals.var_vbscl__blk437_dn8))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn9 - locals.var_vbscl__blk437_dn9) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn9 - locals.var_vbscl__blk437_dn9))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn10 - locals.var_vbscl__blk437_dn10) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn10 - locals.var_vbscl__blk437_dn10))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn11 - locals.var_vbscl__blk437_dn11) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn11 - locals.var_vbscl__blk437_dn11))) / (2.0 * assign101220_e153304)), ((((locals.var_ps0_dn14 - locals.var_vbscl__blk437_dn14) * assign101220_e153296) + (assign101220_e153293 * (locals.var_ps0_dn14 - locals.var_vbscl__blk437_dn14))) / (2.0 * assign101220_e153304)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign101220_e153306;
        locals.var_tmf2_dn0 = assign101220_e153306_d_n0;
        locals.var_tmf2_dn2 = assign101220_e153306_d_n2;
        locals.var_tmf2_dn4 = assign101220_e153306_d_n4;
        locals.var_tmf2_dn5 = assign101220_e153306_d_n5;
        locals.var_tmf2_dn6 = assign101220_e153306_d_n6;
        locals.var_tmf2_dn7 = assign101220_e153306_d_n7;
        locals.var_tmf2_dn8 = assign101220_e153306_d_n8;
        locals.var_tmf2_dn9 = assign101220_e153306_d_n9;
        locals.var_tmf2_dn10 = assign101220_e153306_d_n10;
        locals.var_tmf2_dn11 = assign101220_e153306_d_n11;
        locals.var_tmf2_dn14 = assign101220_e153306_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_389(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101230_e153318, assign101230_e153318_d_n0, assign101230_e153318_d_n2, assign101230_e153318_d_n4, assign101230_e153318_d_n5, assign101230_e153318_d_n6, assign101230_e153318_d_n7, assign101230_e153318_d_n8, assign101230_e153318_d_n9, assign101230_e153318_d_n10, assign101230_e153318_d_n11, assign101230_e153318_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101230_e153312: f64 = (locals.var_ps0 - locals.var_vbscl__blk437);
        let assign101230_e153314: f64 = (assign101230_e153312 / locals.var_tmf2);
        let assign101230_e153315: f64 = (1.0 + assign101230_e153314);
        let assign101230_e153316: f64 = (0.5 * assign101230_e153315);
        (assign101230_e153316, (0.5 * ((((locals.var_ps0_dn0 - locals.var_vbscl__blk437_dn0) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn2 - locals.var_vbscl__blk437_dn2) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn4 - locals.var_vbscl__blk437_dn4) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn5 - locals.var_vbscl__blk437_dn5) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn6 - locals.var_vbscl__blk437_dn6) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn7 - locals.var_vbscl__blk437_dn7) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn8 - locals.var_vbscl__blk437_dn8) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn9 - locals.var_vbscl__blk437_dn9) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn10 - locals.var_vbscl__blk437_dn10) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn11 - locals.var_vbscl__blk437_dn11) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn14 - locals.var_vbscl__blk437_dn14) * locals.var_tmf2) - (assign101230_e153312 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101230_e153318;
        locals.var_t0_dn0 = assign101230_e153318_d_n0;
        locals.var_t0_dn2 = assign101230_e153318_d_n2;
        locals.var_t0_dn4 = assign101230_e153318_d_n4;
        locals.var_t0_dn5 = assign101230_e153318_d_n5;
        locals.var_t0_dn6 = assign101230_e153318_d_n6;
        locals.var_t0_dn7 = assign101230_e153318_d_n7;
        locals.var_t0_dn8 = assign101230_e153318_d_n8;
        locals.var_t0_dn9 = assign101230_e153318_d_n9;
        locals.var_t0_dn10 = assign101230_e153318_d_n10;
        locals.var_t0_dn11 = assign101230_e153318_d_n11;
        locals.var_t0_dn14 = assign101230_e153318_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign101240_e153328, assign101240_e153328_d_n0, assign101240_e153328_d_n2, assign101240_e153328_d_n4, assign101240_e153328_d_n5, assign101240_e153328_d_n6, assign101240_e153328_d_n7, assign101240_e153328_d_n8, assign101240_e153328_d_n9, assign101240_e153328_d_n10, assign101240_e153328_d_n11, assign101240_e153328_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101240_e153323: f64 = (locals.var_ps0 - locals.var_vbscl__blk437);
        let assign101240_e153325: f64 = (assign101240_e153323 + locals.var_tmf2);
        let assign101240_e153326: f64 = (0.5 * assign101240_e153325);
        (assign101240_e153326, (0.5 * ((locals.var_ps0_dn0 - locals.var_vbscl__blk437_dn0) + locals.var_tmf2_dn0)), (0.5 * ((locals.var_ps0_dn2 - locals.var_vbscl__blk437_dn2) + locals.var_tmf2_dn2)), (0.5 * ((locals.var_ps0_dn4 - locals.var_vbscl__blk437_dn4) + locals.var_tmf2_dn4)), (0.5 * ((locals.var_ps0_dn5 - locals.var_vbscl__blk437_dn5) + locals.var_tmf2_dn5)), (0.5 * ((locals.var_ps0_dn6 - locals.var_vbscl__blk437_dn6) + locals.var_tmf2_dn6)), (0.5 * ((locals.var_ps0_dn7 - locals.var_vbscl__blk437_dn7) + locals.var_tmf2_dn7)), (0.5 * ((locals.var_ps0_dn8 - locals.var_vbscl__blk437_dn8) + locals.var_tmf2_dn8)), (0.5 * ((locals.var_ps0_dn9 - locals.var_vbscl__blk437_dn9) + locals.var_tmf2_dn9)), (0.5 * ((locals.var_ps0_dn10 - locals.var_vbscl__blk437_dn10) + locals.var_tmf2_dn10)), (0.5 * ((locals.var_ps0_dn11 - locals.var_vbscl__blk437_dn11) + locals.var_tmf2_dn11)), (0.5 * ((locals.var_ps0_dn14 - locals.var_vbscl__blk437_dn14) + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101240_e153328;
        locals.var_t5_dn0 = assign101240_e153328_d_n0;
        locals.var_t5_dn2 = assign101240_e153328_d_n2;
        locals.var_t5_dn4 = assign101240_e153328_d_n4;
        locals.var_t5_dn5 = assign101240_e153328_d_n5;
        locals.var_t5_dn6 = assign101240_e153328_d_n6;
        locals.var_t5_dn7 = assign101240_e153328_d_n7;
        locals.var_t5_dn8 = assign101240_e153328_d_n8;
        locals.var_t5_dn9 = assign101240_e153328_d_n9;
        locals.var_t5_dn10 = assign101240_e153328_d_n10;
        locals.var_t5_dn11 = assign101240_e153328_d_n11;
        locals.var_t5_dn14 = assign101240_e153328_d_n14;
        locals.var_t5_rv = 0.0;

        let assign101250_e153331: f64 = if locals.var_t5 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2325 = assign101250_e153331;
        locals.var_guard2325_rv = 0.0;

        let (assign101260_e153337, assign101260_e153337_d_n0, assign101260_e153337_d_n2, assign101260_e153337_d_n4, assign101260_e153337_d_n5, assign101260_e153337_d_n6, assign101260_e153337_d_n7, assign101260_e153337_d_n8, assign101260_e153337_d_n9, assign101260_e153337_d_n10, assign101260_e153337_d_n11, assign101260_e153337_d_n14,) = {
    if ((locals.var_guard2324 != 0.0) && (locals.var_guard2325 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101260_e153337;
        locals.var_t5_dn0 = assign101260_e153337_d_n0;
        locals.var_t5_dn2 = assign101260_e153337_d_n2;
        locals.var_t5_dn4 = assign101260_e153337_d_n4;
        locals.var_t5_dn5 = assign101260_e153337_d_n5;
        locals.var_t5_dn6 = assign101260_e153337_d_n6;
        locals.var_t5_dn7 = assign101260_e153337_d_n7;
        locals.var_t5_dn8 = assign101260_e153337_d_n8;
        locals.var_t5_dn9 = assign101260_e153337_d_n9;
        locals.var_t5_dn10 = assign101260_e153337_d_n10;
        locals.var_t5_dn11 = assign101260_e153337_d_n11;
        locals.var_t5_dn14 = assign101260_e153337_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign101270_e153343, assign101270_e153343_d_n0, assign101270_e153343_d_n2, assign101270_e153343_d_n4, assign101270_e153343_d_n5, assign101270_e153343_d_n6, assign101270_e153343_d_n7, assign101270_e153343_d_n8, assign101270_e153343_d_n9, assign101270_e153343_d_n10, assign101270_e153343_d_n11, assign101270_e153343_d_n14,) = {
    if ((locals.var_guard2324 != 0.0) && (locals.var_guard2325 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101270_e153343;
        locals.var_t0_dn0 = assign101270_e153343_d_n0;
        locals.var_t0_dn2 = assign101270_e153343_d_n2;
        locals.var_t0_dn4 = assign101270_e153343_d_n4;
        locals.var_t0_dn5 = assign101270_e153343_d_n5;
        locals.var_t0_dn6 = assign101270_e153343_d_n6;
        locals.var_t0_dn7 = assign101270_e153343_d_n7;
        locals.var_t0_dn8 = assign101270_e153343_d_n8;
        locals.var_t0_dn9 = assign101270_e153343_d_n9;
        locals.var_t0_dn10 = assign101270_e153343_d_n10;
        locals.var_t0_dn11 = assign101270_e153343_d_n11;
        locals.var_t0_dn14 = assign101270_e153343_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign101280_e153357, assign101280_e153357_d_n0, assign101280_e153357_d_n2, assign101280_e153357_d_n4, assign101280_e153357_d_n5, assign101280_e153357_d_n6, assign101280_e153357_d_n7, assign101280_e153357_d_n8, assign101280_e153357_d_n9, assign101280_e153357_d_n10, assign101280_e153357_d_n11, assign101280_e153357_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101280_e153348: f64 = (locals.var_qn0 / locals.var_t5);
        let assign101280_e153349: f64 = (locals.var_cox + assign101280_e153348);
        let assign101280_e153351: f64 = (assign101280_e153349 + locals.var_cite);
        let assign101280_e153353: f64 = (assign101280_e153351 * locals.var_beta_inv);
        let assign101280_e153355: f64 = (assign101280_e153353 / 1.6021918e-19);
        (assign101280_e153355, ((((locals.var_cox_dn0 + (((locals.var_qn0_dn0 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn0)) / 1.6021918e-19), ((((locals.var_cox_dn2 + (((locals.var_qn0_dn2 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn2)) / 1.6021918e-19), ((((locals.var_cox_dn4 + (((locals.var_qn0_dn4 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn4)) / 1.6021918e-19), ((((locals.var_cox_dn5 + (((locals.var_qn0_dn5 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn5)) / 1.6021918e-19), ((((locals.var_cox_dn6 + (((locals.var_qn0_dn6 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn6)) / 1.6021918e-19), ((((locals.var_cox_dn7 + (((locals.var_qn0_dn7 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn7)) / 1.6021918e-19), ((((locals.var_cox_dn8 + (((locals.var_qn0_dn8 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn8)) / 1.6021918e-19), ((((locals.var_cox_dn9 + (((locals.var_qn0_dn9 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn9)) / 1.6021918e-19), ((((locals.var_cox_dn10 + (((locals.var_qn0_dn10 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn10)) / 1.6021918e-19), ((((locals.var_cox_dn11 + (((locals.var_qn0_dn11 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn11)) / 1.6021918e-19), ((((locals.var_cox_dn14 + (((locals.var_qn0_dn14 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101280_e153351 * locals.var_beta_inv_dn14)) / 1.6021918e-19),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101280_e153357;
        locals.var_t2_dn0 = assign101280_e153357_d_n0;
        locals.var_t2_dn2 = assign101280_e153357_d_n2;
        locals.var_t2_dn4 = assign101280_e153357_d_n4;
        locals.var_t2_dn5 = assign101280_e153357_d_n5;
        locals.var_t2_dn6 = assign101280_e153357_d_n6;
        locals.var_t2_dn7 = assign101280_e153357_d_n7;
        locals.var_t2_dn8 = assign101280_e153357_d_n8;
        locals.var_t2_dn9 = assign101280_e153357_d_n9;
        locals.var_t2_dn10 = assign101280_e153357_d_n10;
        locals.var_t2_dn11 = assign101280_e153357_d_n11;
        locals.var_t2_dn14 = assign101280_e153357_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign101290_e153372, assign101290_e153372_d_n0, assign101290_e153372_d_n2, assign101290_e153372_d_n4, assign101290_e153372_d_n5, assign101290_e153372_d_n6, assign101290_e153372_d_n7, assign101290_e153372_d_n8, assign101290_e153372_d_n9, assign101290_e153372_d_n10, assign101290_e153372_d_n11, assign101290_e153372_d_n14,) = {
    if (locals.var_guard2324 != 0.0) {
        let assign101290_e153360: f64 = (-2.0);
        let assign101290_e153362: f64 = (assign101290_e153360 * locals.var_qi_noi);
        let assign101290_e153364: f64 = (assign101290_e153362 / 1.6021918e-19);
        let assign101290_e153366: f64 = (assign101290_e153364 / locals.var_lch);
        let assign101290_e153368: f64 = (assign101290_e153366 / locals.var_weffcv_nf);
        let assign101290_e153370: f64 = (assign101290_e153368 - locals.var_t1);
        (assign101290_e153370, (((((((assign101290_e153360 * locals.var_qi_noi_dn0) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn0), (((((((assign101290_e153360 * locals.var_qi_noi_dn2) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn2), (((((((assign101290_e153360 * locals.var_qi_noi_dn4) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn4), (((((((assign101290_e153360 * locals.var_qi_noi_dn5) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn5), (((((((assign101290_e153360 * locals.var_qi_noi_dn6) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn6), (((((((assign101290_e153360 * locals.var_qi_noi_dn7) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn7), (((((((assign101290_e153360 * locals.var_qi_noi_dn8) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn8), (((((((assign101290_e153360 * locals.var_qi_noi_dn9) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn9), (((((((assign101290_e153360 * locals.var_qi_noi_dn10) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn10), (((((((assign101290_e153360 * locals.var_qi_noi_dn11) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn11), (((((((assign101290_e153360 * locals.var_qi_noi_dn14) / 1.6021918e-19) * locals.var_lch) - (assign101290_e153364 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101290_e153372;
        locals.var_t3_dn0 = assign101290_e153372_d_n0;
        locals.var_t3_dn2 = assign101290_e153372_d_n2;
        locals.var_t3_dn4 = assign101290_e153372_d_n4;
        locals.var_t3_dn5 = assign101290_e153372_d_n5;
        locals.var_t3_dn6 = assign101290_e153372_d_n6;
        locals.var_t3_dn7 = assign101290_e153372_d_n7;
        locals.var_t3_dn8 = assign101290_e153372_d_n8;
        locals.var_t3_dn9 = assign101290_e153372_d_n9;
        locals.var_t3_dn10 = assign101290_e153372_d_n10;
        locals.var_t3_dn11 = assign101290_e153372_d_n11;
        locals.var_t3_dn14 = assign101290_e153372_d_n14;
        locals.var_t3_rv = 0.0;

        let assign101300_e153375: f64 = (locals.var_t3 - locals.var_t1);
        let assign101300_e153376: f64 = (assign101300_e153375).abs();
        let assign101300_e153379: f64 = (10.0 * 2.220446049250313e-16);
        let assign101300_e153380: f64 = if assign101300_e153376 > assign101300_e153379 { 1.0 } else { 0.0 };
        locals.var_guard2326 = assign101300_e153380;
        locals.var_guard2326_rv = 0.0;

        let (assign101310_e153427, assign101310_e153427_d_n0, assign101310_e153427_d_n2, assign101310_e153427_d_n4, assign101310_e153427_d_n5, assign101310_e153427_d_n6, assign101310_e153427_d_n7, assign101310_e153427_d_n8, assign101310_e153427_d_n9, assign101310_e153427_d_n10, assign101310_e153427_d_n11, assign101310_e153427_d_n14,) = {
    if ((locals.var_guard2324 != 0.0) && (locals.var_guard2326 != 0.0)) {
        let assign101310_e153387: f64 = (locals.var_t1 + locals.var_t2);
        let assign101310_e153388: f64 = (1.0 / assign101310_e153387);
        let assign101310_e153391: f64 = (locals.var_t3 + locals.var_t2);
        let assign101310_e153392: f64 = (assign101310_e153388 / assign101310_e153391);
        let assign101310_e153395: f64 = (2.0 * locals.var_nfalpe);
        let assign101310_e153397: f64 = (assign101310_e153395 * locals.var_ey);
        let assign101310_e153399: f64 = (assign101310_e153397 * locals.var_mu);
        let assign101310_e153402: f64 = (locals.var_t3 - locals.var_t1);
        let assign101310_e153403: f64 = (assign101310_e153399 / assign101310_e153402);
        let assign101310_e153406: f64 = (locals.var_t3 + locals.var_t2);
        let assign101310_e153409: f64 = (locals.var_t1 + locals.var_t2);
        let assign101310_e153410: f64 = (assign101310_e153406 / assign101310_e153409);
        let assign101310_e153411: f64 = (assign101310_e153410).ln();
        let assign101310_e153412: f64 = (assign101310_e153403 * assign101310_e153411);
        let assign101310_e153413: f64 = (assign101310_e153392 + assign101310_e153412);
        let assign101310_e153416: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101310_e153418: f64 = (assign101310_e153416 * locals.var_mu);
        let assign101310_e153420: f64 = (assign101310_e153418 * locals.var_nfalpe);
        let assign101310_e153422: f64 = (assign101310_e153420 * locals.var_ey);
        let assign101310_e153424: f64 = (assign101310_e153422 * locals.var_mu);
        let assign101310_e153425: f64 = (assign101310_e153413 + assign101310_e153424);
        (assign101310_e153425, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn0) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn0)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn0 - locals.var_t1_dn0))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn0 + locals.var_t2_dn0) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn0)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn2) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn2)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn2 - locals.var_t1_dn2))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn2 + locals.var_t2_dn2) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn2)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn4) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn4)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn4 - locals.var_t1_dn4))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn4 + locals.var_t2_dn4) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn4)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn5) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn5)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn5 - locals.var_t1_dn5))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn5 + locals.var_t2_dn5) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn5)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn6) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn6)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn6 - locals.var_t1_dn6))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn6 + locals.var_t2_dn6) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn6)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn7) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn7)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn7 - locals.var_t1_dn7))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn7 + locals.var_t2_dn7) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn7)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn8) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn8)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn8 - locals.var_t1_dn8))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn8 + locals.var_t2_dn8) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn8)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn9) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn9)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn9 - locals.var_t1_dn9))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn9 + locals.var_t2_dn9) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn9)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn10) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn10)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn10 - locals.var_t1_dn10))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn10 + locals.var_t2_dn10) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn10)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn11) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn11)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn11 - locals.var_t1_dn11))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn11 + locals.var_t2_dn11) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn11)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn14 + locals.var_t2_dn14) / (assign101310_e153387 * assign101310_e153387))) * assign101310_e153391) - (assign101310_e153388 * (locals.var_t3_dn14 + locals.var_t2_dn14))) / (assign101310_e153391 * assign101310_e153391)) + ((((((((assign101310_e153395 * locals.var_ey_dn14) * locals.var_mu) + (assign101310_e153397 * locals.var_mu_dn14)) * assign101310_e153402) - (assign101310_e153399 * (locals.var_t3_dn14 - locals.var_t1_dn14))) / (assign101310_e153402 * assign101310_e153402)) * assign101310_e153411) + (assign101310_e153403 * (((((locals.var_t3_dn14 + locals.var_t2_dn14) * assign101310_e153409) - (assign101310_e153406 * (locals.var_t1_dn14 + locals.var_t2_dn14))) / (assign101310_e153409 * assign101310_e153409)) / assign101310_e153410)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn14) * locals.var_mu) + (assign101310_e153416 * locals.var_mu_dn14)) * locals.var_nfalpe) * locals.var_ey) + (assign101310_e153420 * locals.var_ey_dn14)) * locals.var_mu) + (assign101310_e153422 * locals.var_mu_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101310_e153427;
        locals.var_t4_dn0 = assign101310_e153427_d_n0;
        locals.var_t4_dn2 = assign101310_e153427_d_n2;
        locals.var_t4_dn4 = assign101310_e153427_d_n4;
        locals.var_t4_dn5 = assign101310_e153427_d_n5;
        locals.var_t4_dn6 = assign101310_e153427_d_n6;
        locals.var_t4_dn7 = assign101310_e153427_d_n7;
        locals.var_t4_dn8 = assign101310_e153427_d_n8;
        locals.var_t4_dn9 = assign101310_e153427_d_n9;
        locals.var_t4_dn10 = assign101310_e153427_d_n10;
        locals.var_t4_dn11 = assign101310_e153427_d_n11;
        locals.var_t4_dn14 = assign101310_e153427_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign101320_e153466, assign101320_e153466_d_n0, assign101320_e153466_d_n2, assign101320_e153466_d_n4, assign101320_e153466_d_n5, assign101320_e153466_d_n6, assign101320_e153466_d_n7, assign101320_e153466_d_n8, assign101320_e153466_d_n9, assign101320_e153466_d_n10, assign101320_e153466_d_n11, assign101320_e153466_d_n14,) = {
    if ((locals.var_guard2324 != 0.0) && (locals.var_guard2326 == 0.0)) {
        let assign101320_e153435: f64 = (locals.var_t1 + locals.var_t2);
        let assign101320_e153436: f64 = (1.0 / assign101320_e153435);
        let assign101320_e153439: f64 = (locals.var_t3 + locals.var_t2);
        let assign101320_e153440: f64 = (assign101320_e153436 / assign101320_e153439);
        let assign101320_e153443: f64 = (2.0 * locals.var_nfalpe);
        let assign101320_e153445: f64 = (assign101320_e153443 * locals.var_ey);
        let assign101320_e153447: f64 = (assign101320_e153445 * locals.var_mu);
        let assign101320_e153450: f64 = (locals.var_t1 + locals.var_t2);
        let assign101320_e153451: f64 = (assign101320_e153447 / assign101320_e153450);
        let assign101320_e153452: f64 = (assign101320_e153440 + assign101320_e153451);
        let assign101320_e153455: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101320_e153457: f64 = (assign101320_e153455 * locals.var_mu);
        let assign101320_e153459: f64 = (assign101320_e153457 * locals.var_nfalpe);
        let assign101320_e153461: f64 = (assign101320_e153459 * locals.var_ey);
        let assign101320_e153463: f64 = (assign101320_e153461 * locals.var_mu);
        let assign101320_e153464: f64 = (assign101320_e153452 + assign101320_e153463);
        (assign101320_e153464, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn0) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn0)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn0)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn2) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn2)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn2)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn4) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn4)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn4)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn5) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn5)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn5)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn6) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn6)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn6)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn7) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn7)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn7)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn8) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn8)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn8)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn9) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn9)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn9)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn10) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn10)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn10)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn11) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn11)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn11)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn14 + locals.var_t2_dn14) / (assign101320_e153435 * assign101320_e153435))) * assign101320_e153439) - (assign101320_e153436 * (locals.var_t3_dn14 + locals.var_t2_dn14))) / (assign101320_e153439 * assign101320_e153439)) + ((((((assign101320_e153443 * locals.var_ey_dn14) * locals.var_mu) + (assign101320_e153445 * locals.var_mu_dn14)) * assign101320_e153450) - (assign101320_e153447 * (locals.var_t1_dn14 + locals.var_t2_dn14))) / (assign101320_e153450 * assign101320_e153450))) + ((((((((locals.var_nfalpe * locals.var_ey_dn14) * locals.var_mu) + (assign101320_e153455 * locals.var_mu_dn14)) * locals.var_nfalpe) * locals.var_ey) + (assign101320_e153459 * locals.var_ey_dn14)) * locals.var_mu) + (assign101320_e153461 * locals.var_mu_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101320_e153466;
        locals.var_t4_dn0 = assign101320_e153466_d_n0;
        locals.var_t4_dn2 = assign101320_e153466_d_n2;
        locals.var_t4_dn4 = assign101320_e153466_d_n4;
        locals.var_t4_dn5 = assign101320_e153466_d_n5;
        locals.var_t4_dn6 = assign101320_e153466_d_n6;
        locals.var_t4_dn7 = assign101320_e153466_d_n7;
        locals.var_t4_dn8 = assign101320_e153466_d_n8;
        locals.var_t4_dn9 = assign101320_e153466_d_n9;
        locals.var_t4_dn10 = assign101320_e153466_d_n10;
        locals.var_t4_dn11 = assign101320_e153466_d_n11;
        locals.var_t4_dn14 = assign101320_e153466_d_n14;
        locals.var_t4_rv = 0.0;

        let assign101350_e153497: f64 = if (((p.p30 != 0.0) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2327 = assign101350_e153497;
        locals.var_guard2327_rv = 0.0;

        let (assign101360_e153509, assign101360_e153509_d_n0, assign101360_e153509_d_n2, assign101360_e153509_d_n4, assign101360_e153509_d_n5, assign101360_e153509_d_n6, assign101360_e153509_d_n7, assign101360_e153509_d_n8, assign101360_e153509_d_n9, assign101360_e153509_d_n10, assign101360_e153509_d_n11, assign101360_e153509_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101360_e153501: f64 = (locals.var_psdl - locals.var_ps0);
        let assign101360_e153504: f64 = (10.0 * 2.220446049250313e-16);
        let assign101360_e153505: f64 = (assign101360_e153501 + assign101360_e153504);
        let assign101360_e153507: f64 = (assign101360_e153505 / locals.var_lch);
        (assign101360_e153507, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn4 - locals.var_ps0_dn4) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn5 - locals.var_ps0_dn5) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn8 - locals.var_ps0_dn8) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn9 - locals.var_ps0_dn9) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn14 - locals.var_ps0_dn14) * locals.var_lch) - (assign101360_e153505 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
    }
};
        locals.var_eyd = assign101360_e153509;
        locals.var_eyd_dn0 = assign101360_e153509_d_n0;
        locals.var_eyd_dn2 = assign101360_e153509_d_n2;
        locals.var_eyd_dn4 = assign101360_e153509_d_n4;
        locals.var_eyd_dn5 = assign101360_e153509_d_n5;
        locals.var_eyd_dn6 = assign101360_e153509_d_n6;
        locals.var_eyd_dn7 = assign101360_e153509_d_n7;
        locals.var_eyd_dn8 = assign101360_e153509_d_n8;
        locals.var_eyd_dn9 = assign101360_e153509_d_n9;
        locals.var_eyd_dn10 = assign101360_e153509_d_n10;
        locals.var_eyd_dn11 = assign101360_e153509_d_n11;
        locals.var_eyd_dn14 = assign101360_e153509_d_n14;
        locals.var_eyd_rv = 0.0;

        let (assign101370_e153518, assign101370_e153518_d_n0, assign101370_e153518_d_n2, assign101370_e153518_d_n4, assign101370_e153518_d_n5, assign101370_e153518_d_n6, assign101370_e153518_d_n7, assign101370_e153518_d_n8, assign101370_e153518_d_n9, assign101370_e153518_d_n10, assign101370_e153518_d_n11, assign101370_e153518_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let (assign101370_e153516, assign101370_e153516_d_n0, assign101370_e153516_d_n2, assign101370_e153516_d_n4, assign101370_e153516_d_n5, assign101370_e153516_d_n6, assign101370_e153516_d_n7, assign101370_e153516_d_n8, assign101370_e153516_d_n9, assign101370_e153516_d_n10, assign101370_e153516_d_n11, assign101370_e153516_d_n14,) = {
            if (locals.var_eyd >= 0.0) {
                (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign101370_e153516, assign101370_e153516_d_n0, assign101370_e153516_d_n2, assign101370_e153516_d_n4, assign101370_e153516_d_n5, assign101370_e153516_d_n6, assign101370_e153516_d_n7, assign101370_e153516_d_n8, assign101370_e153516_d_n9, assign101370_e153516_d_n10, assign101370_e153516_d_n11, assign101370_e153516_d_n14,)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
    }
};
        locals.var_eyd = assign101370_e153518;
        locals.var_eyd_dn0 = assign101370_e153518_d_n0;
        locals.var_eyd_dn2 = assign101370_e153518_d_n2;
        locals.var_eyd_dn4 = assign101370_e153518_d_n4;
        locals.var_eyd_dn5 = assign101370_e153518_d_n5;
        locals.var_eyd_dn6 = assign101370_e153518_d_n6;
        locals.var_eyd_dn7 = assign101370_e153518_d_n7;
        locals.var_eyd_dn8 = assign101370_e153518_d_n8;
        locals.var_eyd_dn9 = assign101370_e153518_d_n9;
        locals.var_eyd_dn10 = assign101370_e153518_d_n10;
        locals.var_eyd_dn11 = assign101370_e153518_d_n11;
        locals.var_eyd_dn14 = assign101370_e153518_d_n14;
        locals.var_eyd_rv = 0.0;

        let (assign101380_e153526, assign101380_e153526_d_n0, assign101380_e153526_d_n2, assign101380_e153526_d_n4, assign101380_e153526_d_n5, assign101380_e153526_d_n6, assign101380_e153526_d_n7, assign101380_e153526_d_n8, assign101380_e153526_d_n9, assign101380_e153526_d_n10, assign101380_e153526_d_n11, assign101380_e153526_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101380_e153522: f64 = (locals.var_muun * locals.var_eyd);
        let assign101380_e153524: f64 = (assign101380_e153522 / 10000000.0);
        (assign101380_e153524, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 10000000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 10000000.0), (((locals.var_muun_dn4 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn4)) / 10000000.0), (((locals.var_muun_dn5 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn5)) / 10000000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 10000000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 10000000.0), (((locals.var_muun_dn8 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn8)) / 10000000.0), (((locals.var_muun_dn9 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn9)) / 10000000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 10000000.0), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / 10000000.0), (((locals.var_muun_dn14 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn14)) / 10000000.0),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign101380_e153526;
        locals.var_t12_dn0 = assign101380_e153526_d_n0;
        locals.var_t12_dn2 = assign101380_e153526_d_n2;
        locals.var_t12_dn4 = assign101380_e153526_d_n4;
        locals.var_t12_dn5 = assign101380_e153526_d_n5;
        locals.var_t12_dn6 = assign101380_e153526_d_n6;
        locals.var_t12_dn7 = assign101380_e153526_d_n7;
        locals.var_t12_dn8 = assign101380_e153526_d_n8;
        locals.var_t12_dn9 = assign101380_e153526_d_n9;
        locals.var_t12_dn10 = assign101380_e153526_d_n10;
        locals.var_t12_dn11 = assign101380_e153526_d_n11;
        locals.var_t12_dn14 = assign101380_e153526_d_n14;
        locals.var_t12_rv = 0.0;

        let assign101390_e153530: f64 = (10.0 * 2.220446049250313e-16);
        let assign101390_e153531: f64 = (1.0 - assign101390_e153530);
        let assign101390_e153538: f64 = (10.0 * 2.220446049250313e-16);
        let assign101390_e153539: f64 = (1.0 + assign101390_e153538);
        let assign101390_e153541: f64 = if ((assign101390_e153531 <= p.p178) && (p.p178 <= assign101390_e153539)) { 1.0 } else { 0.0 };
        locals.var_guard2328 = assign101390_e153541;
        locals.var_guard2328_rv = 0.0;

        let (assign101400_e153547, assign101400_e153547_d_n0, assign101400_e153547_d_n2, assign101400_e153547_d_n4, assign101400_e153547_d_n5, assign101400_e153547_d_n6, assign101400_e153547_d_n7, assign101400_e153547_d_n8, assign101400_e153547_d_n9, assign101400_e153547_d_n10, assign101400_e153547_d_n11, assign101400_e153547_d_n14,) = {
    if ((locals.var_guard2327 != 0.0) && (locals.var_guard2328 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101400_e153547;
        locals.var_t7_dn0 = assign101400_e153547_d_n0;
        locals.var_t7_dn2 = assign101400_e153547_d_n2;
        locals.var_t7_dn4 = assign101400_e153547_d_n4;
        locals.var_t7_dn5 = assign101400_e153547_d_n5;
        locals.var_t7_dn6 = assign101400_e153547_d_n6;
        locals.var_t7_dn7 = assign101400_e153547_d_n7;
        locals.var_t7_dn8 = assign101400_e153547_d_n8;
        locals.var_t7_dn9 = assign101400_e153547_d_n9;
        locals.var_t7_dn10 = assign101400_e153547_d_n10;
        locals.var_t7_dn11 = assign101400_e153547_d_n11;
        locals.var_t7_dn14 = assign101400_e153547_d_n14;
        locals.var_t7_rv = 0.0;

        let assign101410_e153551: f64 = (10.0 * 2.220446049250313e-16);
        let assign101410_e153552: f64 = (2.0 - assign101410_e153551);
        let assign101410_e153559: f64 = (10.0 * 2.220446049250313e-16);
        let assign101410_e153560: f64 = (2.0 + assign101410_e153559);
        let assign101410_e153562: f64 = if ((assign101410_e153552 <= p.p178) && (p.p178 <= assign101410_e153560)) { 1.0 } else { 0.0 };
        locals.var_guard2329 = assign101410_e153562;
        locals.var_guard2329_rv = 0.0;

        let (assign101420_e153571, assign101420_e153571_d_n0, assign101420_e153571_d_n2, assign101420_e153571_d_n4, assign101420_e153571_d_n5, assign101420_e153571_d_n6, assign101420_e153571_d_n7, assign101420_e153571_d_n8, assign101420_e153571_d_n9, assign101420_e153571_d_n10, assign101420_e153571_d_n11, assign101420_e153571_d_n14,) = {
    if (((locals.var_guard2327 != 0.0) && (locals.var_guard2328 == 0.0)) && (locals.var_guard2329 != 0.0)) {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101420_e153571;
        locals.var_t7_dn0 = assign101420_e153571_d_n0;
        locals.var_t7_dn2 = assign101420_e153571_d_n2;
        locals.var_t7_dn4 = assign101420_e153571_d_n4;
        locals.var_t7_dn5 = assign101420_e153571_d_n5;
        locals.var_t7_dn6 = assign101420_e153571_d_n6;
        locals.var_t7_dn7 = assign101420_e153571_d_n7;
        locals.var_t7_dn8 = assign101420_e153571_d_n8;
        locals.var_t7_dn9 = assign101420_e153571_d_n9;
        locals.var_t7_dn10 = assign101420_e153571_d_n10;
        locals.var_t7_dn11 = assign101420_e153571_d_n11;
        locals.var_t7_dn14 = assign101420_e153571_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign101430_e153590, assign101430_e153590_d_n0, assign101430_e153590_d_n2, assign101430_e153590_d_n4, assign101430_e153590_d_n5, assign101430_e153590_d_n6, assign101430_e153590_d_n7, assign101430_e153590_d_n8, assign101430_e153590_d_n9, assign101430_e153590_d_n10, assign101430_e153590_d_n11, assign101430_e153590_d_n14,) = {
    if (((locals.var_guard2327 != 0.0) && (locals.var_guard2328 == 0.0)) && (locals.var_guard2329 == 0.0)) {
        let (assign101430_e153588, assign101430_e153588_d_n0, assign101430_e153588_d_n2, assign101430_e153588_d_n4, assign101430_e153588_d_n5, assign101430_e153588_d_n6, assign101430_e153588_d_n7, assign101430_e153588_d_n8, assign101430_e153588_d_n9, assign101430_e153588_d_n10, assign101430_e153588_d_n11, assign101430_e153588_d_n14,) = {
            if (locals.var_eyd == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101430_e153586: f64 = (p.p178 - 1.0);
                let assign101430_e153587: f64 = (locals.var_eyd).powf(assign101430_e153586);
                (assign101430_e153587, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn0)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn0 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn2)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn2 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn4)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn4 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn5)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn5 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn6)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn6 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn7)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn7 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn8)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn8 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn9)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn9 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn10)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn10 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn11)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn11 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101430_e153586) as f64).is_finite() && ((assign101430_e153586) as f64).fract() == 0.0 { if assign101430_e153586 == 0.0 { 0.0 } else { (assign101430_e153586 * ((locals.var_eyd).powf(assign101430_e153586 - 1.0) * locals.var_eyd_dn14)) } } else { (assign101430_e153587 * (assign101430_e153586 * (locals.var_eyd_dn14 / locals.var_eyd))) },)
            }
        };
        (assign101430_e153588, assign101430_e153588_d_n0, assign101430_e153588_d_n2, assign101430_e153588_d_n4, assign101430_e153588_d_n5, assign101430_e153588_d_n6, assign101430_e153588_d_n7, assign101430_e153588_d_n8, assign101430_e153588_d_n9, assign101430_e153588_d_n10, assign101430_e153588_d_n11, assign101430_e153588_d_n14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101430_e153590;
        locals.var_t7_dn0 = assign101430_e153590_d_n0;
        locals.var_t7_dn2 = assign101430_e153590_d_n2;
        locals.var_t7_dn4 = assign101430_e153590_d_n4;
        locals.var_t7_dn5 = assign101430_e153590_d_n5;
        locals.var_t7_dn6 = assign101430_e153590_d_n6;
        locals.var_t7_dn7 = assign101430_e153590_d_n7;
        locals.var_t7_dn8 = assign101430_e153590_d_n8;
        locals.var_t7_dn9 = assign101430_e153590_d_n9;
        locals.var_t7_dn10 = assign101430_e153590_d_n10;
        locals.var_t7_dn11 = assign101430_e153590_d_n11;
        locals.var_t7_dn14 = assign101430_e153590_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign101440_e153596, assign101440_e153596_d_n0, assign101440_e153596_d_n2, assign101440_e153596_d_n4, assign101440_e153596_d_n5, assign101440_e153596_d_n6, assign101440_e153596_d_n7, assign101440_e153596_d_n8, assign101440_e153596_d_n9, assign101440_e153596_d_n10, assign101440_e153596_d_n11, assign101440_e153596_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101440_e153594: f64 = (locals.var_t12 * locals.var_t7);
        (assign101440_e153594, ((locals.var_t12_dn0 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn0)), ((locals.var_t12_dn2 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn2)), ((locals.var_t12_dn4 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn4)), ((locals.var_t12_dn5 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn5)), ((locals.var_t12_dn6 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn6)), ((locals.var_t12_dn7 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn7)), ((locals.var_t12_dn8 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn8)), ((locals.var_t12_dn9 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn9)), ((locals.var_t12_dn10 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn10)), ((locals.var_t12_dn11 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn11)), ((locals.var_t12_dn14 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign101440_e153596;
        locals.var_t8_dn0 = assign101440_e153596_d_n0;
        locals.var_t8_dn2 = assign101440_e153596_d_n2;
        locals.var_t8_dn4 = assign101440_e153596_d_n4;
        locals.var_t8_dn5 = assign101440_e153596_d_n5;
        locals.var_t8_dn6 = assign101440_e153596_d_n6;
        locals.var_t8_dn7 = assign101440_e153596_d_n7;
        locals.var_t8_dn8 = assign101440_e153596_d_n8;
        locals.var_t8_dn9 = assign101440_e153596_d_n9;
        locals.var_t8_dn10 = assign101440_e153596_d_n10;
        locals.var_t8_dn11 = assign101440_e153596_d_n11;
        locals.var_t8_dn14 = assign101440_e153596_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign101450_e153602, assign101450_e153602_d_n0, assign101450_e153602_d_n2, assign101450_e153602_d_n4, assign101450_e153602_d_n5, assign101450_e153602_d_n6, assign101450_e153602_d_n7, assign101450_e153602_d_n8, assign101450_e153602_d_n9, assign101450_e153602_d_n10, assign101450_e153602_d_n11, assign101450_e153602_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101450_e153600: f64 = (1.0 + locals.var_t8);
        (assign101450_e153600, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign101450_e153602;
        locals.var_t9_dn0 = assign101450_e153602_d_n0;
        locals.var_t9_dn2 = assign101450_e153602_d_n2;
        locals.var_t9_dn4 = assign101450_e153602_d_n4;
        locals.var_t9_dn5 = assign101450_e153602_d_n5;
        locals.var_t9_dn6 = assign101450_e153602_d_n6;
        locals.var_t9_dn7 = assign101450_e153602_d_n7;
        locals.var_t9_dn8 = assign101450_e153602_d_n8;
        locals.var_t9_dn9 = assign101450_e153602_d_n9;
        locals.var_t9_dn10 = assign101450_e153602_d_n10;
        locals.var_t9_dn11 = assign101450_e153602_d_n11;
        locals.var_t9_dn14 = assign101450_e153602_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign101460_e153618, assign101460_e153618_d_n0, assign101460_e153618_d_n2, assign101460_e153618_d_n4, assign101460_e153618_d_n5, assign101460_e153618_d_n6, assign101460_e153618_d_n7, assign101460_e153618_d_n8, assign101460_e153618_d_n9, assign101460_e153618_d_n10, assign101460_e153618_d_n11, assign101460_e153618_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let (assign101460_e153616, assign101460_e153616_d_n0, assign101460_e153616_d_n2, assign101460_e153616_d_n4, assign101460_e153616_d_n5, assign101460_e153616_d_n6, assign101460_e153616_d_n7, assign101460_e153616_d_n8, assign101460_e153616_d_n9, assign101460_e153616_d_n10, assign101460_e153616_d_n11, assign101460_e153616_d_n14,) = {
            if (locals.var_t9 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101460_e153610: f64 = (-1.0);
                let assign101460_e153612: f64 = (assign101460_e153610 / p.p178);
                let assign101460_e153614: f64 = (assign101460_e153612 - 1.0);
                let assign101460_e153615: f64 = (locals.var_t9).powf(assign101460_e153614);
                (assign101460_e153615, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn0)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn0 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn2)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn2 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn4)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn4 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn5)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn5 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn6)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn6 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn7)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn7 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn8)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn8 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn9)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn9 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn10)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn10 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn11)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn11 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101460_e153614) as f64).is_finite() && ((assign101460_e153614) as f64).fract() == 0.0 { if assign101460_e153614 == 0.0 { 0.0 } else { (assign101460_e153614 * ((locals.var_t9).powf(assign101460_e153614 - 1.0) * locals.var_t9_dn14)) } } else { (assign101460_e153615 * (assign101460_e153614 * (locals.var_t9_dn14 / locals.var_t9))) },)
            }
        };
        (assign101460_e153616, assign101460_e153616_d_n0, assign101460_e153616_d_n2, assign101460_e153616_d_n4, assign101460_e153616_d_n5, assign101460_e153616_d_n6, assign101460_e153616_d_n7, assign101460_e153616_d_n8, assign101460_e153616_d_n9, assign101460_e153616_d_n10, assign101460_e153616_d_n11, assign101460_e153616_d_n14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101460_e153618;
        locals.var_t10_dn0 = assign101460_e153618_d_n0;
        locals.var_t10_dn2 = assign101460_e153618_d_n2;
        locals.var_t10_dn4 = assign101460_e153618_d_n4;
        locals.var_t10_dn5 = assign101460_e153618_d_n5;
        locals.var_t10_dn6 = assign101460_e153618_d_n6;
        locals.var_t10_dn7 = assign101460_e153618_d_n7;
        locals.var_t10_dn8 = assign101460_e153618_d_n8;
        locals.var_t10_dn9 = assign101460_e153618_d_n9;
        locals.var_t10_dn10 = assign101460_e153618_d_n10;
        locals.var_t10_dn11 = assign101460_e153618_d_n11;
        locals.var_t10_dn14 = assign101460_e153618_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign101470_e153624, assign101470_e153624_d_n0, assign101470_e153624_d_n2, assign101470_e153624_d_n4, assign101470_e153624_d_n5, assign101470_e153624_d_n6, assign101470_e153624_d_n7, assign101470_e153624_d_n8, assign101470_e153624_d_n9, assign101470_e153624_d_n10, assign101470_e153624_d_n11, assign101470_e153624_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101470_e153622: f64 = (locals.var_t9 * locals.var_t10);
        (assign101470_e153622, ((locals.var_t9_dn0 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn0)), ((locals.var_t9_dn2 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn2)), ((locals.var_t9_dn4 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn4)), ((locals.var_t9_dn5 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn5)), ((locals.var_t9_dn6 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn6)), ((locals.var_t9_dn7 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn7)), ((locals.var_t9_dn8 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn8)), ((locals.var_t9_dn9 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn9)), ((locals.var_t9_dn10 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn10)), ((locals.var_t9_dn11 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn11)), ((locals.var_t9_dn14 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign101470_e153624;
        locals.var_t11_dn0 = assign101470_e153624_d_n0;
        locals.var_t11_dn2 = assign101470_e153624_d_n2;
        locals.var_t11_dn4 = assign101470_e153624_d_n4;
        locals.var_t11_dn5 = assign101470_e153624_d_n5;
        locals.var_t11_dn6 = assign101470_e153624_d_n6;
        locals.var_t11_dn7 = assign101470_e153624_d_n7;
        locals.var_t11_dn8 = assign101470_e153624_d_n8;
        locals.var_t11_dn9 = assign101470_e153624_d_n9;
        locals.var_t11_dn10 = assign101470_e153624_d_n10;
        locals.var_t11_dn11 = assign101470_e153624_d_n11;
        locals.var_t11_dn14 = assign101470_e153624_d_n14;
        locals.var_t11_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_390(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101480_e153630, assign101480_e153630_d_n0, assign101480_e153630_d_n2, assign101480_e153630_d_n4, assign101480_e153630_d_n5, assign101480_e153630_d_n6, assign101480_e153630_d_n7, assign101480_e153630_d_n8, assign101480_e153630_d_n9, assign101480_e153630_d_n10, assign101480_e153630_d_n11, assign101480_e153630_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101480_e153628: f64 = (locals.var_muun * locals.var_t11);
        (assign101480_e153628, ((locals.var_muun_dn0 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn0)), ((locals.var_muun_dn2 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn2)), ((locals.var_muun_dn4 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn4)), ((locals.var_muun_dn5 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn5)), ((locals.var_muun_dn6 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn6)), ((locals.var_muun_dn7 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn7)), ((locals.var_muun_dn8 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn8)), ((locals.var_muun_dn9 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn9)), ((locals.var_muun_dn10 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn10)), ((locals.var_muun_dn11 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn11)), ((locals.var_muun_dn14 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn14)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn4, locals.var_mud_hoso_dn5, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn8, locals.var_mud_hoso_dn9, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn14,)
    }
};
        locals.var_mud_hoso = assign101480_e153630;
        locals.var_mud_hoso_dn0 = assign101480_e153630_d_n0;
        locals.var_mud_hoso_dn2 = assign101480_e153630_d_n2;
        locals.var_mud_hoso_dn4 = assign101480_e153630_d_n4;
        locals.var_mud_hoso_dn5 = assign101480_e153630_d_n5;
        locals.var_mud_hoso_dn6 = assign101480_e153630_d_n6;
        locals.var_mud_hoso_dn7 = assign101480_e153630_d_n7;
        locals.var_mud_hoso_dn8 = assign101480_e153630_d_n8;
        locals.var_mud_hoso_dn9 = assign101480_e153630_d_n9;
        locals.var_mud_hoso_dn10 = assign101480_e153630_d_n10;
        locals.var_mud_hoso_dn11 = assign101480_e153630_d_n11;
        locals.var_mud_hoso_dn14 = assign101480_e153630_d_n14;
        locals.var_mud_hoso_rv = 0.0;

        let (assign101490_e153638, assign101490_e153638_d_n0, assign101490_e153638_d_n2, assign101490_e153638_d_n4, assign101490_e153638_d_n5, assign101490_e153638_d_n6, assign101490_e153638_d_n7, assign101490_e153638_d_n8, assign101490_e153638_d_n9, assign101490_e153638_d_n10, assign101490_e153638_d_n11, assign101490_e153638_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101490_e153634: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign101490_e153636: f64 = (assign101490_e153634 / 2.0);
        (assign101490_e153636, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn4 + locals.var_mud_hoso_dn4) / 2.0), ((locals.var_mu_dn5 + locals.var_mud_hoso_dn5) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn8 + locals.var_mud_hoso_dn8) / 2.0), ((locals.var_mu_dn9 + locals.var_mud_hoso_dn9) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn14 + locals.var_mud_hoso_dn14) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn4, locals.var_mu_ave_dn5, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn8, locals.var_mu_ave_dn9, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn14,)
    }
};
        locals.var_mu_ave = assign101490_e153638;
        locals.var_mu_ave_dn0 = assign101490_e153638_d_n0;
        locals.var_mu_ave_dn2 = assign101490_e153638_d_n2;
        locals.var_mu_ave_dn4 = assign101490_e153638_d_n4;
        locals.var_mu_ave_dn5 = assign101490_e153638_d_n5;
        locals.var_mu_ave_dn6 = assign101490_e153638_d_n6;
        locals.var_mu_ave_dn7 = assign101490_e153638_d_n7;
        locals.var_mu_ave_dn8 = assign101490_e153638_d_n8;
        locals.var_mu_ave_dn9 = assign101490_e153638_d_n9;
        locals.var_mu_ave_dn10 = assign101490_e153638_d_n10;
        locals.var_mu_ave_dn11 = assign101490_e153638_d_n11;
        locals.var_mu_ave_dn14 = assign101490_e153638_d_n14;
        locals.var_mu_ave_rv = 0.0;

        let (assign101500_e153644, assign101500_e153644_d_n0, assign101500_e153644_d_n2, assign101500_e153644_d_n4, assign101500_e153644_d_n5, assign101500_e153644_d_n6, assign101500_e153644_d_n7, assign101500_e153644_d_n8, assign101500_e153644_d_n9, assign101500_e153644_d_n10, assign101500_e153644_d_n11, assign101500_e153644_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101500_e153642: f64 = (locals.var_alpha * locals.var_alpha);
        (assign101500_e153642, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn4 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn4)), ((locals.var_alpha_dn5 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn5)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn8 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn8)), ((locals.var_alpha_dn9 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn9)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn14 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101500_e153644;
        locals.var_t0_dn0 = assign101500_e153644_d_n0;
        locals.var_t0_dn2 = assign101500_e153644_d_n2;
        locals.var_t0_dn4 = assign101500_e153644_d_n4;
        locals.var_t0_dn5 = assign101500_e153644_d_n5;
        locals.var_t0_dn6 = assign101500_e153644_d_n6;
        locals.var_t0_dn7 = assign101500_e153644_d_n7;
        locals.var_t0_dn8 = assign101500_e153644_d_n8;
        locals.var_t0_dn9 = assign101500_e153644_d_n9;
        locals.var_t0_dn10 = assign101500_e153644_d_n10;
        locals.var_t0_dn11 = assign101500_e153644_d_n11;
        locals.var_t0_dn14 = assign101500_e153644_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign101510_e153706, assign101510_e153706_d_n0, assign101510_e153706_d_n2, assign101510_e153706_d_n4, assign101510_e153706_d_n5, assign101510_e153706_d_n6, assign101510_e153706_d_n7, assign101510_e153706_d_n8, assign101510_e153706_d_n9, assign101510_e153706_d_n10, assign101510_e153706_d_n11, assign101510_e153706_d_n14,) = {
    if (locals.var_guard2327 != 0.0) {
        let assign101510_e153648: f64 = (locals.var_weff_nf * locals.var_cox);
        let assign101510_e153650: f64 = (assign101510_e153648 * locals.var_vgvt);
        let assign101510_e153652: f64 = (assign101510_e153650 * locals.var_mu);
        let assign101510_e153656: f64 = (3.0 * locals.var_alpha);
        let assign101510_e153657: f64 = (1.0 + assign101510_e153656);
        let assign101510_e153660: f64 = (6.0 * locals.var_t0);
        let assign101510_e153661: f64 = (assign101510_e153657 + assign101510_e153660);
        let assign101510_e153663: f64 = (assign101510_e153661 * locals.var_mud_hoso);
        let assign101510_e153665: f64 = (assign101510_e153663 * locals.var_mud_hoso);
        let assign101510_e153669: f64 = (4.0 * locals.var_alpha);
        let assign101510_e153670: f64 = (3.0 + assign101510_e153669);
        let assign101510_e153673: f64 = (3.0 * locals.var_t0);
        let assign101510_e153674: f64 = (assign101510_e153670 + assign101510_e153673);
        let assign101510_e153676: f64 = (assign101510_e153674 * locals.var_mud_hoso);
        let assign101510_e153678: f64 = (assign101510_e153676 * locals.var_mu);
        let assign101510_e153679: f64 = (assign101510_e153665 + assign101510_e153678);
        let assign101510_e153683: f64 = (3.0 * locals.var_alpha);
        let assign101510_e153684: f64 = (6.0 + assign101510_e153683);
        let assign101510_e153686: f64 = (assign101510_e153684 + locals.var_t0);
        let assign101510_e153688: f64 = (assign101510_e153686 * locals.var_mu);
        let assign101510_e153690: f64 = (assign101510_e153688 * locals.var_mu);
        let assign101510_e153691: f64 = (assign101510_e153679 + assign101510_e153690);
        let assign101510_e153692: f64 = (assign101510_e153652 * assign101510_e153691);
        let assign101510_e153695: f64 = (15.0 * locals.var_lch);
        let assign101510_e153698: f64 = (1.0 + locals.var_alpha);
        let assign101510_e153699: f64 = (assign101510_e153695 * assign101510_e153698);
        let assign101510_e153701: f64 = (assign101510_e153699 * locals.var_mu_ave);
        let assign101510_e153703: f64 = (assign101510_e153701 * locals.var_mu_ave);
        let assign101510_e153704: f64 = (assign101510_e153692 / assign101510_e153703);
        (assign101510_e153704, ((((((((((locals.var_weff_nf * locals.var_cox_dn0) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn0)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0_dn0) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn0)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn0))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn0) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn0)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn2) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn2)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0_dn2) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn2)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn2))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn2) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn2)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn4) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn4)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn4)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn4) + (6.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn4)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn4)) + ((((((4.0 * locals.var_alpha_dn4) + (3.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn4)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn4))) + ((((((3.0 * locals.var_alpha_dn4) + locals.var_t0_dn4) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn4)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn4))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn4) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn4)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn4)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn4)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn5) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn5)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn5)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn5) + (6.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn5)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn5)) + ((((((4.0 * locals.var_alpha_dn5) + (3.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn5)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn5))) + ((((((3.0 * locals.var_alpha_dn5) + locals.var_t0_dn5) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn5)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn5))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn5) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn5)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn5)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn5)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn6) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn6)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0_dn6) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn6)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn6))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn6) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn6)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn7) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn7)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0_dn7) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn7)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn7))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn7) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn7)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn8) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn8)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn8)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn8) + (6.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn8)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn8)) + ((((((4.0 * locals.var_alpha_dn8) + (3.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn8)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn8))) + ((((((3.0 * locals.var_alpha_dn8) + locals.var_t0_dn8) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn8)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn8))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn8) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn8)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn8)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn8)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn9) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn9)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn9)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn9) + (6.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn9)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn9)) + ((((((4.0 * locals.var_alpha_dn9) + (3.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn9)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn9))) + ((((((3.0 * locals.var_alpha_dn9) + locals.var_t0_dn9) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn9)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn9))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn9) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn9)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn9)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn9)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn10) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn10)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0_dn10) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn10)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn10))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn10) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn10)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn11) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn11)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0_dn11) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn11)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn11))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn11) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn11)))) / (assign101510_e153703 * assign101510_e153703)), ((((((((((locals.var_weff_nf * locals.var_cox_dn14) * locals.var_vgvt) + (assign101510_e153648 * locals.var_vgvt_dn14)) * locals.var_mu) + (assign101510_e153650 * locals.var_mu_dn14)) * assign101510_e153691) + (assign101510_e153652 * ((((((((3.0 * locals.var_alpha_dn14) + (6.0 * locals.var_t0_dn14)) * locals.var_mud_hoso) + (assign101510_e153661 * locals.var_mud_hoso_dn14)) * locals.var_mud_hoso) + (assign101510_e153663 * locals.var_mud_hoso_dn14)) + ((((((4.0 * locals.var_alpha_dn14) + (3.0 * locals.var_t0_dn14)) * locals.var_mud_hoso) + (assign101510_e153674 * locals.var_mud_hoso_dn14)) * locals.var_mu) + (assign101510_e153676 * locals.var_mu_dn14))) + ((((((3.0 * locals.var_alpha_dn14) + locals.var_t0_dn14) * locals.var_mu) + (assign101510_e153686 * locals.var_mu_dn14)) * locals.var_mu) + (assign101510_e153688 * locals.var_mu_dn14))))) * assign101510_e153703) - (assign101510_e153692 * (((((((15.0 * locals.var_lch_dn14) * assign101510_e153698) + (assign101510_e153695 * locals.var_alpha_dn14)) * locals.var_mu_ave) + (assign101510_e153699 * locals.var_mu_ave_dn14)) * locals.var_mu_ave) + (assign101510_e153701 * locals.var_mu_ave_dn14)))) / (assign101510_e153703 * assign101510_e153703)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn14,)
    }
};
        locals.var_nthrml = assign101510_e153706;
        locals.var_nthrml_dn0 = assign101510_e153706_d_n0;
        locals.var_nthrml_dn2 = assign101510_e153706_d_n2;
        locals.var_nthrml_dn4 = assign101510_e153706_d_n4;
        locals.var_nthrml_dn5 = assign101510_e153706_d_n5;
        locals.var_nthrml_dn6 = assign101510_e153706_d_n6;
        locals.var_nthrml_dn7 = assign101510_e153706_d_n7;
        locals.var_nthrml_dn8 = assign101510_e153706_d_n8;
        locals.var_nthrml_dn9 = assign101510_e153706_d_n9;
        locals.var_nthrml_dn10 = assign101510_e153706_d_n10;
        locals.var_nthrml_dn11 = assign101510_e153706_d_n11;
        locals.var_nthrml_dn14 = assign101510_e153706_d_n14;
        locals.var_nthrml_rv = 0.0;

        let (assign101520_e153711, assign101520_e153711_d_n0, assign101520_e153711_d_n2, assign101520_e153711_d_n4, assign101520_e153711_d_n5, assign101520_e153711_d_n6, assign101520_e153711_d_n7, assign101520_e153711_d_n8, assign101520_e153711_d_n9, assign101520_e153711_d_n10, assign101520_e153711_d_n11, assign101520_e153711_d_n14,) = {
    if (locals.var_guard2327 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn14,)
    }
};
        locals.var_nthrml = assign101520_e153711;
        locals.var_nthrml_dn0 = assign101520_e153711_d_n0;
        locals.var_nthrml_dn2 = assign101520_e153711_d_n2;
        locals.var_nthrml_dn4 = assign101520_e153711_d_n4;
        locals.var_nthrml_dn5 = assign101520_e153711_d_n5;
        locals.var_nthrml_dn6 = assign101520_e153711_d_n6;
        locals.var_nthrml_dn7 = assign101520_e153711_d_n7;
        locals.var_nthrml_dn8 = assign101520_e153711_d_n8;
        locals.var_nthrml_dn9 = assign101520_e153711_d_n9;
        locals.var_nthrml_dn10 = assign101520_e153711_d_n10;
        locals.var_nthrml_dn11 = assign101520_e153711_d_n11;
        locals.var_nthrml_dn14 = assign101520_e153711_d_n14;
        locals.var_nthrml_rv = 0.0;

        let assign101530_e153729: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2330 = assign101530_e153729;
        locals.var_guard2330_rv = 0.0;

        let (assign101540_e153734, assign101540_e153734_d_n0, assign101540_e153734_d_n2, assign101540_e153734_d_n4, assign101540_e153734_d_n5, assign101540_e153734_d_n6, assign101540_e153734_d_n7, assign101540_e153734_d_n8, assign101540_e153734_d_n9, assign101540_e153734_d_n10, assign101540_e153734_d_n11, assign101540_e153734_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101540_e153732: f64 = (locals.var_kusail).sqrt();
        (assign101540_e153732, (locals.var_kusail_dn0 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn2 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn4 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn5 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn6 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn7 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn8 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn9 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn10 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn11 / (2.0 * assign101540_e153732)), (locals.var_kusail_dn14 / (2.0 * assign101540_e153732)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn4, locals.var_sqrtkusail_dn5, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn8, locals.var_sqrtkusail_dn9, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn14,)
    }
};
        locals.var_sqrtkusail = assign101540_e153734;
        locals.var_sqrtkusail_dn0 = assign101540_e153734_d_n0;
        locals.var_sqrtkusail_dn2 = assign101540_e153734_d_n2;
        locals.var_sqrtkusail_dn4 = assign101540_e153734_d_n4;
        locals.var_sqrtkusail_dn5 = assign101540_e153734_d_n5;
        locals.var_sqrtkusail_dn6 = assign101540_e153734_d_n6;
        locals.var_sqrtkusail_dn7 = assign101540_e153734_d_n7;
        locals.var_sqrtkusail_dn8 = assign101540_e153734_d_n8;
        locals.var_sqrtkusail_dn9 = assign101540_e153734_d_n9;
        locals.var_sqrtkusail_dn10 = assign101540_e153734_d_n10;
        locals.var_sqrtkusail_dn11 = assign101540_e153734_d_n11;
        locals.var_sqrtkusail_dn14 = assign101540_e153734_d_n14;
        locals.var_sqrtkusail_rv = 0.0;

        let (assign101550_e153740, assign101550_e153740_d_n0, assign101550_e153740_d_n2, assign101550_e153740_d_n4, assign101550_e153740_d_n5, assign101550_e153740_d_n6, assign101550_e153740_d_n7, assign101550_e153740_d_n8, assign101550_e153740_d_n9, assign101550_e153740_d_n10, assign101550_e153740_d_n11, assign101550_e153740_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101550_e153738: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign101550_e153738, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4), (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8), (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn14 + locals.var_sqrtkusail_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101550_e153740;
        locals.var_t2_dn0 = assign101550_e153740_d_n0;
        locals.var_t2_dn2 = assign101550_e153740_d_n2;
        locals.var_t2_dn4 = assign101550_e153740_d_n4;
        locals.var_t2_dn5 = assign101550_e153740_d_n5;
        locals.var_t2_dn6 = assign101550_e153740_d_n6;
        locals.var_t2_dn7 = assign101550_e153740_d_n7;
        locals.var_t2_dn8 = assign101550_e153740_d_n8;
        locals.var_t2_dn9 = assign101550_e153740_d_n9;
        locals.var_t2_dn10 = assign101550_e153740_d_n10;
        locals.var_t2_dn11 = assign101550_e153740_d_n11;
        locals.var_t2_dn14 = assign101550_e153740_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign101560_e153746, assign101560_e153746_d_n0, assign101560_e153746_d_n2, assign101560_e153746_d_n4, assign101560_e153746_d_n5, assign101560_e153746_d_n6, assign101560_e153746_d_n7, assign101560_e153746_d_n8, assign101560_e153746_d_n9, assign101560_e153746_d_n10, assign101560_e153746_d_n11, assign101560_e153746_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101560_e153744: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign101560_e153744, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)), ((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)), ((locals.var_kusai00_dn9 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn9)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn14 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101560_e153746;
        locals.var_t3_dn0 = assign101560_e153746_d_n0;
        locals.var_t3_dn2 = assign101560_e153746_d_n2;
        locals.var_t3_dn4 = assign101560_e153746_d_n4;
        locals.var_t3_dn5 = assign101560_e153746_d_n5;
        locals.var_t3_dn6 = assign101560_e153746_d_n6;
        locals.var_t3_dn7 = assign101560_e153746_d_n7;
        locals.var_t3_dn8 = assign101560_e153746_d_n8;
        locals.var_t3_dn9 = assign101560_e153746_d_n9;
        locals.var_t3_dn10 = assign101560_e153746_d_n10;
        locals.var_t3_dn11 = assign101560_e153746_d_n11;
        locals.var_t3_dn14 = assign101560_e153746_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign101570_e153752, assign101570_e153752_d_n0, assign101570_e153752_d_n2, assign101570_e153752_d_n4, assign101570_e153752_d_n5, assign101570_e153752_d_n6, assign101570_e153752_d_n7, assign101570_e153752_d_n8, assign101570_e153752_d_n9, assign101570_e153752_d_n10, assign101570_e153752_d_n11, assign101570_e153752_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101570_e153750: f64 = (locals.var_kusail * locals.var_kusail);
        (assign101570_e153750, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)), ((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)), ((locals.var_kusail_dn9 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn9)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn14 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101570_e153752;
        locals.var_t4_dn0 = assign101570_e153752_d_n0;
        locals.var_t4_dn2 = assign101570_e153752_d_n2;
        locals.var_t4_dn4 = assign101570_e153752_d_n4;
        locals.var_t4_dn5 = assign101570_e153752_d_n5;
        locals.var_t4_dn6 = assign101570_e153752_d_n6;
        locals.var_t4_dn7 = assign101570_e153752_d_n7;
        locals.var_t4_dn8 = assign101570_e153752_d_n8;
        locals.var_t4_dn9 = assign101570_e153752_d_n9;
        locals.var_t4_dn10 = assign101570_e153752_d_n10;
        locals.var_t4_dn11 = assign101570_e153752_d_n11;
        locals.var_t4_dn14 = assign101570_e153752_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign101580_e153760, assign101580_e153760_d_n0, assign101580_e153760_d_n2, assign101580_e153760_d_n4, assign101580_e153760_d_n5, assign101580_e153760_d_n6, assign101580_e153760_d_n7, assign101580_e153760_d_n8, assign101580_e153760_d_n9, assign101580_e153760_d_n10, assign101580_e153760_d_n11, assign101580_e153760_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101580_e153756: f64 = (42.0 * locals.var_kusai00);
        let assign101580_e153758: f64 = (assign101580_e153756 * locals.var_kusail);
        (assign101580_e153758, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn4) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn4)), (((42.0 * locals.var_kusai00_dn5) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn5)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn8) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn8)), (((42.0 * locals.var_kusai00_dn9) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn9)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn14) * locals.var_kusail) + (assign101580_e153756 * locals.var_kusail_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101580_e153760;
        locals.var_t5_dn0 = assign101580_e153760_d_n0;
        locals.var_t5_dn2 = assign101580_e153760_d_n2;
        locals.var_t5_dn4 = assign101580_e153760_d_n4;
        locals.var_t5_dn5 = assign101580_e153760_d_n5;
        locals.var_t5_dn6 = assign101580_e153760_d_n6;
        locals.var_t5_dn7 = assign101580_e153760_d_n7;
        locals.var_t5_dn8 = assign101580_e153760_d_n8;
        locals.var_t5_dn9 = assign101580_e153760_d_n9;
        locals.var_t5_dn10 = assign101580_e153760_d_n10;
        locals.var_t5_dn11 = assign101580_e153760_d_n11;
        locals.var_t5_dn14 = assign101580_e153760_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign101590_e153770, assign101590_e153770_d_n0, assign101590_e153770_d_n2, assign101590_e153770_d_n4, assign101590_e153770_d_n5, assign101590_e153770_d_n6, assign101590_e153770_d_n7, assign101590_e153770_d_n8, assign101590_e153770_d_n9, assign101590_e153770_d_n10, assign101590_e153770_d_n11, assign101590_e153770_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101590_e153766: f64 = (locals.var_t3 + locals.var_t4);
        let assign101590_e153767: f64 = (4.0 * assign101590_e153766);
        let assign101590_e153768: f64 = (locals.var_t5 + assign101590_e153767);
        (assign101590_e153768, (locals.var_t5_dn0 + (4.0 * (locals.var_t3_dn0 + locals.var_t4_dn0))), (locals.var_t5_dn2 + (4.0 * (locals.var_t3_dn2 + locals.var_t4_dn2))), (locals.var_t5_dn4 + (4.0 * (locals.var_t3_dn4 + locals.var_t4_dn4))), (locals.var_t5_dn5 + (4.0 * (locals.var_t3_dn5 + locals.var_t4_dn5))), (locals.var_t5_dn6 + (4.0 * (locals.var_t3_dn6 + locals.var_t4_dn6))), (locals.var_t5_dn7 + (4.0 * (locals.var_t3_dn7 + locals.var_t4_dn7))), (locals.var_t5_dn8 + (4.0 * (locals.var_t3_dn8 + locals.var_t4_dn8))), (locals.var_t5_dn9 + (4.0 * (locals.var_t3_dn9 + locals.var_t4_dn9))), (locals.var_t5_dn10 + (4.0 * (locals.var_t3_dn10 + locals.var_t4_dn10))), (locals.var_t5_dn11 + (4.0 * (locals.var_t3_dn11 + locals.var_t4_dn11))), (locals.var_t5_dn14 + (4.0 * (locals.var_t3_dn14 + locals.var_t4_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101590_e153770;
        locals.var_t5_dn0 = assign101590_e153770_d_n0;
        locals.var_t5_dn2 = assign101590_e153770_d_n2;
        locals.var_t5_dn4 = assign101590_e153770_d_n4;
        locals.var_t5_dn5 = assign101590_e153770_d_n5;
        locals.var_t5_dn6 = assign101590_e153770_d_n6;
        locals.var_t5_dn7 = assign101590_e153770_d_n7;
        locals.var_t5_dn8 = assign101590_e153770_d_n8;
        locals.var_t5_dn9 = assign101590_e153770_d_n9;
        locals.var_t5_dn10 = assign101590_e153770_d_n10;
        locals.var_t5_dn11 = assign101590_e153770_d_n11;
        locals.var_t5_dn14 = assign101590_e153770_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign101600_e153784, assign101600_e153784_d_n0, assign101600_e153784_d_n2, assign101600_e153784_d_n4, assign101600_e153784_d_n5, assign101600_e153784_d_n6, assign101600_e153784_d_n7, assign101600_e153784_d_n8, assign101600_e153784_d_n9, assign101600_e153784_d_n10, assign101600_e153784_d_n11, assign101600_e153784_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101600_e153775: f64 = (20.0 * locals.var_sqrtkusail);
        let assign101600_e153777: f64 = (assign101600_e153775 * locals.var_vgvt);
        let assign101600_e153780: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign101600_e153781: f64 = (assign101600_e153777 * assign101600_e153780);
        let assign101600_e153782: f64 = (locals.var_t5 + assign101600_e153781);
        (assign101600_e153782, (locals.var_t5_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn0)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn2)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5_dn4 + (((((20.0 * locals.var_sqrtkusail_dn4) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn4)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn4 + locals.var_kusail_dn4)))), (locals.var_t5_dn5 + (((((20.0 * locals.var_sqrtkusail_dn5) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn5)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn5 + locals.var_kusail_dn5)))), (locals.var_t5_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn6)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn7)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5_dn8 + (((((20.0 * locals.var_sqrtkusail_dn8) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn8)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn8 + locals.var_kusail_dn8)))), (locals.var_t5_dn9 + (((((20.0 * locals.var_sqrtkusail_dn9) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn9)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn9 + locals.var_kusail_dn9)))), (locals.var_t5_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn10)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn11)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5_dn14 + (((((20.0 * locals.var_sqrtkusail_dn14) * locals.var_vgvt) + (assign101600_e153775 * locals.var_vgvt_dn14)) * assign101600_e153780) + (assign101600_e153777 * (locals.var_kusai00_dn14 + locals.var_kusail_dn14)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101600_e153784;
        locals.var_t5_dn0 = assign101600_e153784_d_n0;
        locals.var_t5_dn2 = assign101600_e153784_d_n2;
        locals.var_t5_dn4 = assign101600_e153784_d_n4;
        locals.var_t5_dn5 = assign101600_e153784_d_n5;
        locals.var_t5_dn6 = assign101600_e153784_d_n6;
        locals.var_t5_dn7 = assign101600_e153784_d_n7;
        locals.var_t5_dn8 = assign101600_e153784_d_n8;
        locals.var_t5_dn9 = assign101600_e153784_d_n9;
        locals.var_t5_dn10 = assign101600_e153784_d_n10;
        locals.var_t5_dn11 = assign101600_e153784_d_n11;
        locals.var_t5_dn14 = assign101600_e153784_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign101610_e153790, assign101610_e153790_d_n0, assign101610_e153790_d_n2, assign101610_e153790_d_n4, assign101610_e153790_d_n5, assign101610_e153790_d_n6, assign101610_e153790_d_n7, assign101610_e153790_d_n8, assign101610_e153790_d_n9, assign101610_e153790_d_n10, assign101610_e153790_d_n11, assign101610_e153790_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101610_e153788: f64 = (locals.var_t2 * locals.var_t2);
        (assign101610_e153788, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101610_e153790;
        locals.var_t10_dn0 = assign101610_e153790_d_n0;
        locals.var_t10_dn2 = assign101610_e153790_d_n2;
        locals.var_t10_dn4 = assign101610_e153790_d_n4;
        locals.var_t10_dn5 = assign101610_e153790_d_n5;
        locals.var_t10_dn6 = assign101610_e153790_d_n6;
        locals.var_t10_dn7 = assign101610_e153790_d_n7;
        locals.var_t10_dn8 = assign101610_e153790_d_n8;
        locals.var_t10_dn9 = assign101610_e153790_d_n9;
        locals.var_t10_dn10 = assign101610_e153790_d_n10;
        locals.var_t10_dn11 = assign101610_e153790_d_n11;
        locals.var_t10_dn14 = assign101610_e153790_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign101620_e153796, assign101620_e153796_d_n0, assign101620_e153796_d_n2, assign101620_e153796_d_n4, assign101620_e153796_d_n5, assign101620_e153796_d_n6, assign101620_e153796_d_n7, assign101620_e153796_d_n8, assign101620_e153796_d_n9, assign101620_e153796_d_n10, assign101620_e153796_d_n11, assign101620_e153796_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101620_e153794: f64 = (locals.var_t10 * locals.var_t10);
        (assign101620_e153794, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)), ((locals.var_t10_dn14 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101620_e153796;
        locals.var_t10_dn0 = assign101620_e153796_d_n0;
        locals.var_t10_dn2 = assign101620_e153796_d_n2;
        locals.var_t10_dn4 = assign101620_e153796_d_n4;
        locals.var_t10_dn5 = assign101620_e153796_d_n5;
        locals.var_t10_dn6 = assign101620_e153796_d_n6;
        locals.var_t10_dn7 = assign101620_e153796_d_n7;
        locals.var_t10_dn8 = assign101620_e153796_d_n8;
        locals.var_t10_dn9 = assign101620_e153796_d_n9;
        locals.var_t10_dn10 = assign101620_e153796_d_n10;
        locals.var_t10_dn11 = assign101620_e153796_d_n11;
        locals.var_t10_dn14 = assign101620_e153796_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign101630_e153804, assign101630_e153804_d_n0, assign101630_e153804_d_n2, assign101630_e153804_d_n4, assign101630_e153804_d_n5, assign101630_e153804_d_n6, assign101630_e153804_d_n7, assign101630_e153804_d_n8, assign101630_e153804_d_n9, assign101630_e153804_d_n10, assign101630_e153804_d_n11, assign101630_e153804_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101630_e153801: f64 = (locals.var_t10 * locals.var_t2);
        let assign101630_e153802: f64 = (locals.var_t5 / assign101630_e153801);
        (assign101630_e153802, (((locals.var_t5_dn0 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn0 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn0)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn2 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn2 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn2)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn4 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn4 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn4)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn5 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn5 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn5)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn6 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn6 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn6)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn7 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn7 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn7)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn8 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn8 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn8)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn9 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn9 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn9)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn10 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn10 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn10)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn11 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn11 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn11)))) / (assign101630_e153801 * assign101630_e153801)), (((locals.var_t5_dn14 * assign101630_e153801) - (locals.var_t5 * ((locals.var_t10_dn14 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn14)))) / (assign101630_e153801 * assign101630_e153801)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn4, locals.var_kusai_ig_dn5, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn8, locals.var_kusai_ig_dn9, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn14,)
    }
};
        locals.var_kusai_ig = assign101630_e153804;
        locals.var_kusai_ig_dn0 = assign101630_e153804_d_n0;
        locals.var_kusai_ig_dn2 = assign101630_e153804_d_n2;
        locals.var_kusai_ig_dn4 = assign101630_e153804_d_n4;
        locals.var_kusai_ig_dn5 = assign101630_e153804_d_n5;
        locals.var_kusai_ig_dn6 = assign101630_e153804_d_n6;
        locals.var_kusai_ig_dn7 = assign101630_e153804_d_n7;
        locals.var_kusai_ig_dn8 = assign101630_e153804_d_n8;
        locals.var_kusai_ig_dn9 = assign101630_e153804_d_n9;
        locals.var_kusai_ig_dn10 = assign101630_e153804_d_n10;
        locals.var_kusai_ig_dn11 = assign101630_e153804_d_n11;
        locals.var_kusai_ig_dn14 = assign101630_e153804_d_n14;
        locals.var_kusai_ig_rv = 0.0;

        let (assign101640_e153814, assign101640_e153814_d_n0, assign101640_e153814_d_n2, assign101640_e153814_d_n4, assign101640_e153814_d_n5, assign101640_e153814_d_n6, assign101640_e153814_d_n7, assign101640_e153814_d_n8, assign101640_e153814_d_n9, assign101640_e153814_d_n10, assign101640_e153814_d_n11, assign101640_e153814_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101640_e153808: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign101640_e153810: f64 = (assign101640_e153808 * locals.var_mu);
        let assign101640_e153812: f64 = (assign101640_e153810 * locals.var_cox);
        (assign101640_e153812, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn0)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn2)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn4) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn4)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn4)), (((((-((locals.var_weff_nf * locals.var_lch_dn5) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn5)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn5)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn6)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn7)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn8) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn8)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn8)), (((((-((locals.var_weff_nf * locals.var_lch_dn9) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn9)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn9)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn10)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn11) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn11)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn11)), (((((-((locals.var_weff_nf * locals.var_lch_dn14) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101640_e153808 * locals.var_mu_dn14)) * locals.var_cox) + (assign101640_e153810 * locals.var_cox_dn14)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn4, locals.var_gds0_ign_dn5, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn8, locals.var_gds0_ign_dn9, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn14,)
    }
};
        locals.var_gds0_ign = assign101640_e153814;
        locals.var_gds0_ign_dn0 = assign101640_e153814_d_n0;
        locals.var_gds0_ign_dn2 = assign101640_e153814_d_n2;
        locals.var_gds0_ign_dn4 = assign101640_e153814_d_n4;
        locals.var_gds0_ign_dn5 = assign101640_e153814_d_n5;
        locals.var_gds0_ign_dn6 = assign101640_e153814_d_n6;
        locals.var_gds0_ign_dn7 = assign101640_e153814_d_n7;
        locals.var_gds0_ign_dn8 = assign101640_e153814_d_n8;
        locals.var_gds0_ign_dn9 = assign101640_e153814_d_n9;
        locals.var_gds0_ign_dn10 = assign101640_e153814_d_n10;
        locals.var_gds0_ign_dn11 = assign101640_e153814_d_n11;
        locals.var_gds0_ign_dn14 = assign101640_e153814_d_n14;
        locals.var_gds0_ign_rv = 0.0;

        let (assign101670_e153838, assign101670_e153838_d_n0, assign101670_e153838_d_n2, assign101670_e153838_d_n4, assign101670_e153838_d_n5, assign101670_e153838_d_n6, assign101670_e153838_d_n7, assign101670_e153838_d_n8, assign101670_e153838_d_n9, assign101670_e153838_d_n10, assign101670_e153838_d_n11, assign101670_e153838_d_n14,) = {
    if (locals.var_guard2330 != 0.0) {
        let assign101670_e153831: f64 = (4.0 * locals.var_vgvt);
        let assign101670_e153833: f64 = (assign101670_e153831 * locals.var_sqrtkusail);
        let assign101670_e153834: f64 = (locals.var_kusai00 + assign101670_e153833);
        let assign101670_e153836: f64 = (assign101670_e153834 + locals.var_kusail);
        (assign101670_e153836, ((locals.var_kusai00_dn0 + (((4.0 * locals.var_vgvt_dn0) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0), ((locals.var_kusai00_dn2 + (((4.0 * locals.var_vgvt_dn2) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2), ((locals.var_kusai00_dn4 + (((4.0 * locals.var_vgvt_dn4) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4), ((locals.var_kusai00_dn5 + (((4.0 * locals.var_vgvt_dn5) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5), ((locals.var_kusai00_dn6 + (((4.0 * locals.var_vgvt_dn6) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6), ((locals.var_kusai00_dn7 + (((4.0 * locals.var_vgvt_dn7) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7), ((locals.var_kusai00_dn8 + (((4.0 * locals.var_vgvt_dn8) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8), ((locals.var_kusai00_dn9 + (((4.0 * locals.var_vgvt_dn9) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9), ((locals.var_kusai00_dn10 + (((4.0 * locals.var_vgvt_dn10) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10), ((locals.var_kusai00_dn11 + (((4.0 * locals.var_vgvt_dn11) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11), ((locals.var_kusai00_dn14 + (((4.0 * locals.var_vgvt_dn14) * locals.var_sqrtkusail) + (assign101670_e153831 * locals.var_sqrtkusail_dn14))) + locals.var_kusail_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101670_e153838;
        locals.var_t7_dn0 = assign101670_e153838_d_n0;
        locals.var_t7_dn2 = assign101670_e153838_d_n2;
        locals.var_t7_dn4 = assign101670_e153838_d_n4;
        locals.var_t7_dn5 = assign101670_e153838_d_n5;
        locals.var_t7_dn6 = assign101670_e153838_d_n6;
        locals.var_t7_dn7 = assign101670_e153838_d_n7;
        locals.var_t7_dn8 = assign101670_e153838_d_n8;
        locals.var_t7_dn9 = assign101670_e153838_d_n9;
        locals.var_t7_dn10 = assign101670_e153838_d_n10;
        locals.var_t7_dn11 = assign101670_e153838_d_n11;
        locals.var_t7_dn14 = assign101670_e153838_d_n14;
        locals.var_t7_rv = 0.0;

        let assign101690_e153862: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign101690_e153862;
        locals.var_idse_dn0 = (locals.var_mfactor * locals.var_ids_dn0);
        locals.var_idse_dn2 = (locals.var_mfactor * locals.var_ids_dn2);
        locals.var_idse_dn4 = (locals.var_mfactor * locals.var_ids_dn4);
        locals.var_idse_dn5 = (locals.var_mfactor * locals.var_ids_dn5);
        locals.var_idse_dn6 = (locals.var_mfactor * locals.var_ids_dn6);
        locals.var_idse_dn7 = (locals.var_mfactor * locals.var_ids_dn7);
        locals.var_idse_dn8 = (locals.var_mfactor * locals.var_ids_dn8);
        locals.var_idse_dn9 = (locals.var_mfactor * locals.var_ids_dn9);
        locals.var_idse_dn10 = (locals.var_mfactor * locals.var_ids_dn10);
        locals.var_idse_dn11 = (locals.var_mfactor * locals.var_ids_dn11);
        locals.var_idse_dn14 = (locals.var_mfactor * locals.var_ids_dn14);
        locals.var_idse_rv = 0.0;

        let assign101730_e153874: f64 = (locals.var_mfactor * locals.var_idsibpc);
        locals.var_idsibpce = assign101730_e153874;
        locals.var_idsibpce_dn0 = (locals.var_mfactor * locals.var_idsibpc_dn0);
        locals.var_idsibpce_dn2 = (locals.var_mfactor * locals.var_idsibpc_dn2);
        locals.var_idsibpce_dn4 = (locals.var_mfactor * locals.var_idsibpc_dn4);
        locals.var_idsibpce_dn5 = (locals.var_mfactor * locals.var_idsibpc_dn5);
        locals.var_idsibpce_dn6 = (locals.var_mfactor * locals.var_idsibpc_dn6);
        locals.var_idsibpce_dn7 = (locals.var_mfactor * locals.var_idsibpc_dn7);
        locals.var_idsibpce_dn8 = (locals.var_mfactor * locals.var_idsibpc_dn8);
        locals.var_idsibpce_dn9 = (locals.var_mfactor * locals.var_idsibpc_dn9);
        locals.var_idsibpce_dn10 = (locals.var_mfactor * locals.var_idsibpc_dn10);
        locals.var_idsibpce_dn11 = (locals.var_mfactor * locals.var_idsibpc_dn11);
        locals.var_idsibpce_dn14 = (locals.var_mfactor * locals.var_idsibpc_dn14);
        locals.var_idsibpce_rv = 0.0;

        locals.var_qgexte = 0.0;
        locals.var_qgexte_dn0 = 0.0;
        locals.var_qgexte_dn2 = 0.0;
        locals.var_qgexte_dn4 = 0.0;
        locals.var_qgexte_dn5 = 0.0;
        locals.var_qgexte_dn6 = 0.0;
        locals.var_qgexte_dn7 = 0.0;
        locals.var_qgexte_dn8 = 0.0;
        locals.var_qgexte_dn9 = 0.0;
        locals.var_qgexte_dn10 = 0.0;
        locals.var_qgexte_dn11 = 0.0;
        locals.var_qgexte_dn14 = 0.0;
        locals.var_qgexte_rv = 0.0;

        locals.var_qdexte = 0.0;
        locals.var_qdexte_dn0 = 0.0;
        locals.var_qdexte_dn2 = 0.0;
        locals.var_qdexte_dn4 = 0.0;
        locals.var_qdexte_dn5 = 0.0;
        locals.var_qdexte_dn6 = 0.0;
        locals.var_qdexte_dn7 = 0.0;
        locals.var_qdexte_dn8 = 0.0;
        locals.var_qdexte_dn9 = 0.0;
        locals.var_qdexte_dn10 = 0.0;
        locals.var_qdexte_dn11 = 0.0;
        locals.var_qdexte_dn14 = 0.0;
        locals.var_qdexte_rv = 0.0;

        locals.var_qsexte = 0.0;
        locals.var_qsexte_dn0 = 0.0;
        locals.var_qsexte_dn2 = 0.0;
        locals.var_qsexte_dn4 = 0.0;
        locals.var_qsexte_dn5 = 0.0;
        locals.var_qsexte_dn6 = 0.0;
        locals.var_qsexte_dn7 = 0.0;
        locals.var_qsexte_dn8 = 0.0;
        locals.var_qsexte_dn9 = 0.0;
        locals.var_qsexte_dn10 = 0.0;
        locals.var_qsexte_dn11 = 0.0;
        locals.var_qsexte_dn14 = 0.0;
        locals.var_qsexte_rv = 0.0;

        locals.var_qgov = 0.0;
        locals.var_qgov_dn0 = 0.0;
        locals.var_qgov_dn2 = 0.0;
        locals.var_qgov_dn4 = 0.0;
        locals.var_qgov_dn5 = 0.0;
        locals.var_qgov_dn6 = 0.0;
        locals.var_qgov_dn7 = 0.0;
        locals.var_qgov_dn8 = 0.0;
        locals.var_qgov_dn9 = 0.0;
        locals.var_qgov_dn10 = 0.0;
        locals.var_qgov_dn11 = 0.0;
        locals.var_qgov_dn14 = 0.0;
        locals.var_qgov_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_391(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);
        locals.var_qdov = 0.0;
        locals.var_qdov_dn0 = 0.0;
        locals.var_qdov_dn2 = 0.0;
        locals.var_qdov_dn4 = 0.0;
        locals.var_qdov_dn5 = 0.0;
        locals.var_qdov_dn6 = 0.0;
        locals.var_qdov_dn7 = 0.0;
        locals.var_qdov_dn8 = 0.0;
        locals.var_qdov_dn9 = 0.0;
        locals.var_qdov_dn10 = 0.0;
        locals.var_qdov_dn11 = 0.0;
        locals.var_qdov_dn14 = 0.0;
        locals.var_qdov_rv = 0.0;

        locals.var_qsov = 0.0;
        locals.var_qsov_dn0 = 0.0;
        locals.var_qsov_dn2 = 0.0;
        locals.var_qsov_dn4 = 0.0;
        locals.var_qsov_dn5 = 0.0;
        locals.var_qsov_dn6 = 0.0;
        locals.var_qsov_dn7 = 0.0;
        locals.var_qsov_dn8 = 0.0;
        locals.var_qsov_dn9 = 0.0;
        locals.var_qsov_dn10 = 0.0;
        locals.var_qsov_dn11 = 0.0;
        locals.var_qsov_dn14 = 0.0;
        locals.var_qsov_rv = 0.0;

        locals.var_qdp = 0.0;
        locals.var_qdp_dn0 = 0.0;
        locals.var_qdp_dn2 = 0.0;
        locals.var_qdp_dn7 = 0.0;
        locals.var_qdp_rv = 0.0;

        locals.var_qsp = 0.0;
        locals.var_qsp_dn2 = 0.0;
        locals.var_qsp_dn7 = 0.0;
        locals.var_qsp_rv = 0.0;

        let assign101830_e153888: f64 = if ((locals.var_flg_nqs != 0.0) || (p.p22 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard2331 = assign101830_e153888;
        locals.var_guard2331_rv = 0.0;

        let (assign101840_e153892, assign101840_e153892_d_n0, assign101840_e153892_d_n2, assign101840_e153892_d_n4, assign101840_e153892_d_n5, assign101840_e153892_d_n6, assign101840_e153892_d_n7, assign101840_e153892_d_n8, assign101840_e153892_d_n9, assign101840_e153892_d_n10, assign101840_e153892_d_n11, assign101840_e153892_d_n14,) = {
    if (locals.var_guard2331 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign101840_e153892;
        locals.var_qge_dn0 = assign101840_e153892_d_n0;
        locals.var_qge_dn2 = assign101840_e153892_d_n2;
        locals.var_qge_dn4 = assign101840_e153892_d_n4;
        locals.var_qge_dn5 = assign101840_e153892_d_n5;
        locals.var_qge_dn6 = assign101840_e153892_d_n6;
        locals.var_qge_dn7 = assign101840_e153892_d_n7;
        locals.var_qge_dn8 = assign101840_e153892_d_n8;
        locals.var_qge_dn9 = assign101840_e153892_d_n9;
        locals.var_qge_dn10 = assign101840_e153892_d_n10;
        locals.var_qge_dn11 = assign101840_e153892_d_n11;
        locals.var_qge_dn14 = assign101840_e153892_d_n14;
        locals.var_qge_rv = 0.0;

        let (assign101850_e153896, assign101850_e153896_d_n0, assign101850_e153896_d_n2, assign101850_e153896_d_n4, assign101850_e153896_d_n5, assign101850_e153896_d_n6, assign101850_e153896_d_n7, assign101850_e153896_d_n8, assign101850_e153896_d_n9, assign101850_e153896_d_n10, assign101850_e153896_d_n11, assign101850_e153896_d_n14,) = {
    if (locals.var_guard2331 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign101850_e153896;
        locals.var_qde_dn0 = assign101850_e153896_d_n0;
        locals.var_qde_dn2 = assign101850_e153896_d_n2;
        locals.var_qde_dn4 = assign101850_e153896_d_n4;
        locals.var_qde_dn5 = assign101850_e153896_d_n5;
        locals.var_qde_dn6 = assign101850_e153896_d_n6;
        locals.var_qde_dn7 = assign101850_e153896_d_n7;
        locals.var_qde_dn8 = assign101850_e153896_d_n8;
        locals.var_qde_dn9 = assign101850_e153896_d_n9;
        locals.var_qde_dn10 = assign101850_e153896_d_n10;
        locals.var_qde_dn11 = assign101850_e153896_d_n11;
        locals.var_qde_dn14 = assign101850_e153896_d_n14;
        locals.var_qde_rv = 0.0;

        let (assign101860_e153900, assign101860_e153900_d_n0, assign101860_e153900_d_n2, assign101860_e153900_d_n4, assign101860_e153900_d_n5, assign101860_e153900_d_n6, assign101860_e153900_d_n7, assign101860_e153900_d_n8, assign101860_e153900_d_n9, assign101860_e153900_d_n10, assign101860_e153900_d_n11, assign101860_e153900_d_n14,) = {
    if (locals.var_guard2331 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign101860_e153900;
        locals.var_qse_dn0 = assign101860_e153900_d_n0;
        locals.var_qse_dn2 = assign101860_e153900_d_n2;
        locals.var_qse_dn4 = assign101860_e153900_d_n4;
        locals.var_qse_dn5 = assign101860_e153900_d_n5;
        locals.var_qse_dn6 = assign101860_e153900_d_n6;
        locals.var_qse_dn7 = assign101860_e153900_d_n7;
        locals.var_qse_dn8 = assign101860_e153900_d_n8;
        locals.var_qse_dn9 = assign101860_e153900_d_n9;
        locals.var_qse_dn10 = assign101860_e153900_d_n10;
        locals.var_qse_dn11 = assign101860_e153900_d_n11;
        locals.var_qse_dn14 = assign101860_e153900_d_n14;
        locals.var_qse_rv = 0.0;

        let (assign101870_e153904, assign101870_e153904_d_n0, assign101870_e153904_d_n2, assign101870_e153904_d_n4, assign101870_e153904_d_n5, assign101870_e153904_d_n6, assign101870_e153904_d_n7, assign101870_e153904_d_n8, assign101870_e153904_d_n9, assign101870_e153904_d_n10, assign101870_e153904_d_n11, assign101870_e153904_d_n14,) = {
    if (locals.var_guard2331 != 0.0) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn14,)
    }
};
        locals.var_xd = assign101870_e153904;
        locals.var_xd_dn0 = assign101870_e153904_d_n0;
        locals.var_xd_dn2 = assign101870_e153904_d_n2;
        locals.var_xd_dn4 = assign101870_e153904_d_n4;
        locals.var_xd_dn5 = assign101870_e153904_d_n5;
        locals.var_xd_dn6 = assign101870_e153904_d_n6;
        locals.var_xd_dn7 = assign101870_e153904_d_n7;
        locals.var_xd_dn8 = assign101870_e153904_d_n8;
        locals.var_xd_dn9 = assign101870_e153904_d_n9;
        locals.var_xd_dn10 = assign101870_e153904_d_n10;
        locals.var_xd_dn11 = assign101870_e153904_d_n11;
        locals.var_xd_dn14 = assign101870_e153904_d_n14;
        locals.var_xd_rv = 0.0;

        let (assign101890_e153916, assign101890_e153916_d_n0, assign101890_e153916_d_n2, assign101890_e153916_d_n4, assign101890_e153916_d_n5, assign101890_e153916_d_n6, assign101890_e153916_d_n7, assign101890_e153916_d_n8, assign101890_e153916_d_n9, assign101890_e153916_d_n10, assign101890_e153916_d_n11, assign101890_e153916_d_n14,) = {
    if (locals.var_guard2331 != 0.0) {
        let assign101890_e153914: f64 = (locals.var_mfactor * locals.var_qi);
        (assign101890_e153914, (locals.var_mfactor * locals.var_qi_dn0), (locals.var_mfactor * locals.var_qi_dn2), (locals.var_mfactor * locals.var_qi_dn4), (locals.var_mfactor * locals.var_qi_dn5), (locals.var_mfactor * locals.var_qi_dn6), (locals.var_mfactor * locals.var_qi_dn7), (locals.var_mfactor * locals.var_qi_dn8), (locals.var_mfactor * locals.var_qi_dn9), (locals.var_mfactor * locals.var_qi_dn10), (locals.var_mfactor * locals.var_qi_dn11), (locals.var_mfactor * locals.var_qi_dn14),)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn4, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, locals.var_qi_dn9, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn14,)
    }
};
        locals.var_qi = assign101890_e153916;
        locals.var_qi_dn0 = assign101890_e153916_d_n0;
        locals.var_qi_dn2 = assign101890_e153916_d_n2;
        locals.var_qi_dn4 = assign101890_e153916_d_n4;
        locals.var_qi_dn5 = assign101890_e153916_d_n5;
        locals.var_qi_dn6 = assign101890_e153916_d_n6;
        locals.var_qi_dn7 = assign101890_e153916_d_n7;
        locals.var_qi_dn8 = assign101890_e153916_d_n8;
        locals.var_qi_dn9 = assign101890_e153916_d_n9;
        locals.var_qi_dn10 = assign101890_e153916_d_n10;
        locals.var_qi_dn11 = assign101890_e153916_d_n11;
        locals.var_qi_dn14 = assign101890_e153916_d_n14;
        locals.var_qi_rv = 0.0;

        let (assign101900_e153926, assign101900_e153926_d_n0, assign101900_e153926_d_n2, assign101900_e153926_d_n4, assign101900_e153926_d_n5, assign101900_e153926_d_n6, assign101900_e153926_d_n7, assign101900_e153926_d_n8, assign101900_e153926_d_n9, assign101900_e153926_d_n10, assign101900_e153926_d_n11, assign101900_e153926_d_n14,) = {
    if (locals.var_guard2331 == 0.0) {
        let assign101900_e153922: f64 = (locals.var_qb + locals.var_qi);
        let assign101900_e153923: f64 = (-assign101900_e153922);
        let assign101900_e153924: f64 = (locals.var_mfactor * assign101900_e153923);
        (assign101900_e153924, (locals.var_mfactor * (-(locals.var_qb_dn0 + locals.var_qi_dn0))), (locals.var_mfactor * (-(locals.var_qb_dn2 + locals.var_qi_dn2))), (locals.var_mfactor * (-(locals.var_qb_dn4 + locals.var_qi_dn4))), (locals.var_mfactor * (-(locals.var_qb_dn5 + locals.var_qi_dn5))), (locals.var_mfactor * (-(locals.var_qb_dn6 + locals.var_qi_dn6))), (locals.var_mfactor * (-(locals.var_qb_dn7 + locals.var_qi_dn7))), (locals.var_mfactor * (-(locals.var_qb_dn8 + locals.var_qi_dn8))), (locals.var_mfactor * (-(locals.var_qb_dn9 + locals.var_qi_dn9))), (locals.var_mfactor * (-(locals.var_qb_dn10 + locals.var_qi_dn10))), (locals.var_mfactor * (-(locals.var_qb_dn11 + locals.var_qi_dn11))), (locals.var_mfactor * (-(locals.var_qb_dn14 + locals.var_qi_dn14))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign101900_e153926;
        locals.var_qge_dn0 = assign101900_e153926_d_n0;
        locals.var_qge_dn2 = assign101900_e153926_d_n2;
        locals.var_qge_dn4 = assign101900_e153926_d_n4;
        locals.var_qge_dn5 = assign101900_e153926_d_n5;
        locals.var_qge_dn6 = assign101900_e153926_d_n6;
        locals.var_qge_dn7 = assign101900_e153926_d_n7;
        locals.var_qge_dn8 = assign101900_e153926_d_n8;
        locals.var_qge_dn9 = assign101900_e153926_d_n9;
        locals.var_qge_dn10 = assign101900_e153926_d_n10;
        locals.var_qge_dn11 = assign101900_e153926_d_n11;
        locals.var_qge_dn14 = assign101900_e153926_d_n14;
        locals.var_qge_rv = 0.0;

        let (assign101910_e153933, assign101910_e153933_d_n0, assign101910_e153933_d_n2, assign101910_e153933_d_n4, assign101910_e153933_d_n5, assign101910_e153933_d_n6, assign101910_e153933_d_n7, assign101910_e153933_d_n8, assign101910_e153933_d_n9, assign101910_e153933_d_n10, assign101910_e153933_d_n11, assign101910_e153933_d_n14,) = {
    if (locals.var_guard2331 == 0.0) {
        let assign101910_e153931: f64 = (locals.var_mfactor * locals.var_qd);
        (assign101910_e153931, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn4), (locals.var_mfactor * locals.var_qd_dn5), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn8), (locals.var_mfactor * locals.var_qd_dn9), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn14),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign101910_e153933;
        locals.var_qde_dn0 = assign101910_e153933_d_n0;
        locals.var_qde_dn2 = assign101910_e153933_d_n2;
        locals.var_qde_dn4 = assign101910_e153933_d_n4;
        locals.var_qde_dn5 = assign101910_e153933_d_n5;
        locals.var_qde_dn6 = assign101910_e153933_d_n6;
        locals.var_qde_dn7 = assign101910_e153933_d_n7;
        locals.var_qde_dn8 = assign101910_e153933_d_n8;
        locals.var_qde_dn9 = assign101910_e153933_d_n9;
        locals.var_qde_dn10 = assign101910_e153933_d_n10;
        locals.var_qde_dn11 = assign101910_e153933_d_n11;
        locals.var_qde_dn14 = assign101910_e153933_d_n14;
        locals.var_qde_rv = 0.0;

        let (assign101920_e153942, assign101920_e153942_d_n0, assign101920_e153942_d_n2, assign101920_e153942_d_n4, assign101920_e153942_d_n5, assign101920_e153942_d_n6, assign101920_e153942_d_n7, assign101920_e153942_d_n8, assign101920_e153942_d_n9, assign101920_e153942_d_n10, assign101920_e153942_d_n11, assign101920_e153942_d_n14,) = {
    if (locals.var_guard2331 == 0.0) {
        let assign101920_e153939: f64 = (locals.var_qi - locals.var_qd);
        let assign101920_e153940: f64 = (locals.var_mfactor * assign101920_e153939);
        (assign101920_e153940, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn4 - locals.var_qd_dn4)), (locals.var_mfactor * (locals.var_qi_dn5 - locals.var_qd_dn5)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn8 - locals.var_qd_dn8)), (locals.var_mfactor * (locals.var_qi_dn9 - locals.var_qd_dn9)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn14 - locals.var_qd_dn14)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign101920_e153942;
        locals.var_qse_dn0 = assign101920_e153942_d_n0;
        locals.var_qse_dn2 = assign101920_e153942_d_n2;
        locals.var_qse_dn4 = assign101920_e153942_d_n4;
        locals.var_qse_dn5 = assign101920_e153942_d_n5;
        locals.var_qse_dn6 = assign101920_e153942_d_n6;
        locals.var_qse_dn7 = assign101920_e153942_d_n7;
        locals.var_qse_dn8 = assign101920_e153942_d_n8;
        locals.var_qse_dn9 = assign101920_e153942_d_n9;
        locals.var_qse_dn10 = assign101920_e153942_d_n10;
        locals.var_qse_dn11 = assign101920_e153942_d_n11;
        locals.var_qse_dn14 = assign101920_e153942_d_n14;
        locals.var_qse_rv = 0.0;

        let (assign101930_e153948, assign101930_e153948_d_n0, assign101930_e153948_d_n2, assign101930_e153948_d_n4, assign101930_e153948_d_n5, assign101930_e153948_d_n6, assign101930_e153948_d_n7, assign101930_e153948_d_n8, assign101930_e153948_d_n9, assign101930_e153948_d_n10, assign101930_e153948_d_n11, assign101930_e153948_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101930_e153946: f64 = (locals.var_mks_dlyov * locals.var_psl);
        (assign101930_e153946, ((locals.var_mks_dlyov_dn0 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn0)), ((locals.var_mks_dlyov_dn2 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn2)), ((locals.var_mks_dlyov_dn4 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn4)), ((locals.var_mks_dlyov_dn5 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn5)), ((locals.var_mks_dlyov_dn6 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn6)), ((locals.var_mks_dlyov_dn7 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn7)), ((locals.var_mks_dlyov_dn8 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn8)), ((locals.var_mks_dlyov_dn9 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn9)), ((locals.var_mks_dlyov_dn10 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn10)), ((locals.var_mks_dlyov_dn11 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn11)), ((locals.var_mks_dlyov_dn14 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn14)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign101930_e153948;
        locals.var_mks_dlyov_dn0 = assign101930_e153948_d_n0;
        locals.var_mks_dlyov_dn2 = assign101930_e153948_d_n2;
        locals.var_mks_dlyov_dn4 = assign101930_e153948_d_n4;
        locals.var_mks_dlyov_dn5 = assign101930_e153948_d_n5;
        locals.var_mks_dlyov_dn6 = assign101930_e153948_d_n6;
        locals.var_mks_dlyov_dn7 = assign101930_e153948_d_n7;
        locals.var_mks_dlyov_dn8 = assign101930_e153948_d_n8;
        locals.var_mks_dlyov_dn9 = assign101930_e153948_d_n9;
        locals.var_mks_dlyov_dn10 = assign101930_e153948_d_n10;
        locals.var_mks_dlyov_dn11 = assign101930_e153948_d_n11;
        locals.var_mks_dlyov_dn14 = assign101930_e153948_d_n14;
        locals.var_mks_dlyov_rv = 0.0;

        let (assign101940_e153961, assign101940_e153961_d_n0, assign101940_e153961_d_n2, assign101940_e153961_d_n4, assign101940_e153961_d_n5, assign101940_e153961_d_n6, assign101940_e153961_d_n7, assign101940_e153961_d_n8, assign101940_e153961_d_n9, assign101940_e153961_d_n10, assign101940_e153961_d_n11, assign101940_e153961_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101940_e153952: f64 = (locals.var_mks_dlyov * locals.var_mks_dlyov);
        let assign101940_e153955: f64 = (4.0 * 1e-12);
        let assign101940_e153957: f64 = (assign101940_e153955 * 1e-12);
        let assign101940_e153958: f64 = (assign101940_e153952 + assign101940_e153957);
        let assign101940_e153959: f64 = (assign101940_e153958).sqrt();
        (assign101940_e153959, (((locals.var_mks_dlyov_dn0 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn0)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn2 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn2)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn4 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn4)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn5 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn5)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn6 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn6)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn7 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn7)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn8 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn8)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn9 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn9)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn10 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn10)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn11 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn11)) / (2.0 * assign101940_e153959)), (((locals.var_mks_dlyov_dn14 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn14)) / (2.0 * assign101940_e153959)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign101940_e153961;
        locals.var_tmf2_dn0 = assign101940_e153961_d_n0;
        locals.var_tmf2_dn2 = assign101940_e153961_d_n2;
        locals.var_tmf2_dn4 = assign101940_e153961_d_n4;
        locals.var_tmf2_dn5 = assign101940_e153961_d_n5;
        locals.var_tmf2_dn6 = assign101940_e153961_d_n6;
        locals.var_tmf2_dn7 = assign101940_e153961_d_n7;
        locals.var_tmf2_dn8 = assign101940_e153961_d_n8;
        locals.var_tmf2_dn9 = assign101940_e153961_d_n9;
        locals.var_tmf2_dn10 = assign101940_e153961_d_n10;
        locals.var_tmf2_dn11 = assign101940_e153961_d_n11;
        locals.var_tmf2_dn14 = assign101940_e153961_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign101950_e153971, assign101950_e153971_d_n0, assign101950_e153971_d_n2, assign101950_e153971_d_n4, assign101950_e153971_d_n5, assign101950_e153971_d_n6, assign101950_e153971_d_n7, assign101950_e153971_d_n8, assign101950_e153971_d_n9, assign101950_e153971_d_n10, assign101950_e153971_d_n11, assign101950_e153971_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101950_e153967: f64 = (locals.var_mks_dlyov / locals.var_tmf2);
        let assign101950_e153968: f64 = (1.0 + assign101950_e153967);
        let assign101950_e153969: f64 = (0.5 * assign101950_e153968);
        (assign101950_e153969, (0.5 * (((locals.var_mks_dlyov_dn0 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn2 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn4 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn5 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn6 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn7 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn8 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn9 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn10 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn11 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn14 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101950_e153971;
        locals.var_t0_dn0 = assign101950_e153971_d_n0;
        locals.var_t0_dn2 = assign101950_e153971_d_n2;
        locals.var_t0_dn4 = assign101950_e153971_d_n4;
        locals.var_t0_dn5 = assign101950_e153971_d_n5;
        locals.var_t0_dn6 = assign101950_e153971_d_n6;
        locals.var_t0_dn7 = assign101950_e153971_d_n7;
        locals.var_t0_dn8 = assign101950_e153971_d_n8;
        locals.var_t0_dn9 = assign101950_e153971_d_n9;
        locals.var_t0_dn10 = assign101950_e153971_d_n10;
        locals.var_t0_dn11 = assign101950_e153971_d_n11;
        locals.var_t0_dn14 = assign101950_e153971_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign101960_e153979, assign101960_e153979_d_n0, assign101960_e153979_d_n2, assign101960_e153979_d_n4, assign101960_e153979_d_n5, assign101960_e153979_d_n6, assign101960_e153979_d_n7, assign101960_e153979_d_n8, assign101960_e153979_d_n9, assign101960_e153979_d_n10, assign101960_e153979_d_n11, assign101960_e153979_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101960_e153976: f64 = (locals.var_mks_dlyov + locals.var_tmf2);
        let assign101960_e153977: f64 = (0.5 * assign101960_e153976);
        (assign101960_e153977, (0.5 * (locals.var_mks_dlyov_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_mks_dlyov_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_mks_dlyov_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_mks_dlyov_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_mks_dlyov_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_mks_dlyov_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_mks_dlyov_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_mks_dlyov_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_mks_dlyov_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_mks_dlyov_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_mks_dlyov_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign101960_e153979;
        locals.var_mks_dlyov_dn0 = assign101960_e153979_d_n0;
        locals.var_mks_dlyov_dn2 = assign101960_e153979_d_n2;
        locals.var_mks_dlyov_dn4 = assign101960_e153979_d_n4;
        locals.var_mks_dlyov_dn5 = assign101960_e153979_d_n5;
        locals.var_mks_dlyov_dn6 = assign101960_e153979_d_n6;
        locals.var_mks_dlyov_dn7 = assign101960_e153979_d_n7;
        locals.var_mks_dlyov_dn8 = assign101960_e153979_d_n8;
        locals.var_mks_dlyov_dn9 = assign101960_e153979_d_n9;
        locals.var_mks_dlyov_dn10 = assign101960_e153979_d_n10;
        locals.var_mks_dlyov_dn11 = assign101960_e153979_d_n11;
        locals.var_mks_dlyov_dn14 = assign101960_e153979_d_n14;
        locals.var_mks_dlyov_rv = 0.0;

        let assign101970_e153982: f64 = if locals.var_mks_dlyov < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2332 = assign101970_e153982;
        locals.var_guard2332_rv = 0.0;

        let (assign101980_e153988, assign101980_e153988_d_n0, assign101980_e153988_d_n2, assign101980_e153988_d_n4, assign101980_e153988_d_n5, assign101980_e153988_d_n6, assign101980_e153988_d_n7, assign101980_e153988_d_n8, assign101980_e153988_d_n9, assign101980_e153988_d_n10, assign101980_e153988_d_n11, assign101980_e153988_d_n14,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2332 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign101980_e153988;
        locals.var_mks_dlyov_dn0 = assign101980_e153988_d_n0;
        locals.var_mks_dlyov_dn2 = assign101980_e153988_d_n2;
        locals.var_mks_dlyov_dn4 = assign101980_e153988_d_n4;
        locals.var_mks_dlyov_dn5 = assign101980_e153988_d_n5;
        locals.var_mks_dlyov_dn6 = assign101980_e153988_d_n6;
        locals.var_mks_dlyov_dn7 = assign101980_e153988_d_n7;
        locals.var_mks_dlyov_dn8 = assign101980_e153988_d_n8;
        locals.var_mks_dlyov_dn9 = assign101980_e153988_d_n9;
        locals.var_mks_dlyov_dn10 = assign101980_e153988_d_n10;
        locals.var_mks_dlyov_dn11 = assign101980_e153988_d_n11;
        locals.var_mks_dlyov_dn14 = assign101980_e153988_d_n14;
        locals.var_mks_dlyov_rv = 0.0;

        let (assign101990_e153994, assign101990_e153994_d_n0, assign101990_e153994_d_n2, assign101990_e153994_d_n4, assign101990_e153994_d_n5, assign101990_e153994_d_n6, assign101990_e153994_d_n7, assign101990_e153994_d_n8, assign101990_e153994_d_n9, assign101990_e153994_d_n10, assign101990_e153994_d_n11, assign101990_e153994_d_n14,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2332 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101990_e153994;
        locals.var_t0_dn0 = assign101990_e153994_d_n0;
        locals.var_t0_dn2 = assign101990_e153994_d_n2;
        locals.var_t0_dn4 = assign101990_e153994_d_n4;
        locals.var_t0_dn5 = assign101990_e153994_d_n5;
        locals.var_t0_dn6 = assign101990_e153994_d_n6;
        locals.var_t0_dn7 = assign101990_e153994_d_n7;
        locals.var_t0_dn8 = assign101990_e153994_d_n8;
        locals.var_t0_dn9 = assign101990_e153994_d_n9;
        locals.var_t0_dn10 = assign101990_e153994_d_n10;
        locals.var_t0_dn11 = assign101990_e153994_d_n11;
        locals.var_t0_dn14 = assign101990_e153994_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign102010_e154004, assign102010_e154004_d_n0, assign102010_e154004_d_n2, assign102010_e154004_d_n4, assign102010_e154004_d_n5, assign102010_e154004_d_n6, assign102010_e154004_d_n7, assign102010_e154004_d_n8, assign102010_e154004_d_n9, assign102010_e154004_d_n10, assign102010_e154004_d_n11, assign102010_e154004_d_n14,) = {
    if (p.p29 != 0.0) {
        ((nv14 - 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    }
};
        locals.var_qbd_nqs = assign102010_e154004;
        locals.var_qbd_nqs_dn0 = assign102010_e154004_d_n0;
        locals.var_qbd_nqs_dn2 = assign102010_e154004_d_n2;
        locals.var_qbd_nqs_dn4 = assign102010_e154004_d_n4;
        locals.var_qbd_nqs_dn5 = assign102010_e154004_d_n5;
        locals.var_qbd_nqs_dn6 = assign102010_e154004_d_n6;
        locals.var_qbd_nqs_dn7 = assign102010_e154004_d_n7;
        locals.var_qbd_nqs_dn8 = assign102010_e154004_d_n8;
        locals.var_qbd_nqs_dn9 = assign102010_e154004_d_n9;
        locals.var_qbd_nqs_dn10 = assign102010_e154004_d_n10;
        locals.var_qbd_nqs_dn11 = assign102010_e154004_d_n11;
        locals.var_qbd_nqs_dn14 = assign102010_e154004_d_n14;
        locals.var_qbd_nqs_rv = 0.0;

        let (assign102030_e154020, assign102030_e154020_d_n0, assign102030_e154020_d_n2, assign102030_e154020_d_n4, assign102030_e154020_d_n5, assign102030_e154020_d_n6, assign102030_e154020_d_n7, assign102030_e154020_d_n8, assign102030_e154020_d_n9, assign102030_e154020_d_n10, assign102030_e154020_d_n11, assign102030_e154020_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign102030_e154017: f64 = (locals.var_qbd_qs - locals.var_qbd_nqs);
        let assign102030_e154018: f64 = (locals.var_qovd - assign102030_e154017);
        (assign102030_e154018, (locals.var_qovd_dn0 - (locals.var_qbd_qs_dn0 - locals.var_qbd_nqs_dn0)), (locals.var_qovd_dn2 - (locals.var_qbd_qs_dn2 - locals.var_qbd_nqs_dn2)), (locals.var_qovd_dn4 - (locals.var_qbd_qs_dn4 - locals.var_qbd_nqs_dn4)), (locals.var_qovd_dn5 - (locals.var_qbd_qs_dn5 - locals.var_qbd_nqs_dn5)), (locals.var_qovd_dn6 - (locals.var_qbd_qs_dn6 - locals.var_qbd_nqs_dn6)), (locals.var_qovd_dn7 - (locals.var_qbd_qs_dn7 - locals.var_qbd_nqs_dn7)), (locals.var_qovd_dn8 - (locals.var_qbd_qs_dn8 - locals.var_qbd_nqs_dn8)), (locals.var_qovd_dn9 - (locals.var_qbd_qs_dn9 - locals.var_qbd_nqs_dn9)), (locals.var_qovd_dn10 - (locals.var_qbd_qs_dn10 - locals.var_qbd_nqs_dn10)), (locals.var_qovd_dn11 - (locals.var_qbd_qs_dn11 - locals.var_qbd_nqs_dn11)), (locals.var_qovd_dn14 - (locals.var_qbd_qs_dn14 - locals.var_qbd_nqs_dn14)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign102030_e154020;
        locals.var_qovd_dn0 = assign102030_e154020_d_n0;
        locals.var_qovd_dn2 = assign102030_e154020_d_n2;
        locals.var_qovd_dn4 = assign102030_e154020_d_n4;
        locals.var_qovd_dn5 = assign102030_e154020_d_n5;
        locals.var_qovd_dn6 = assign102030_e154020_d_n6;
        locals.var_qovd_dn7 = assign102030_e154020_d_n7;
        locals.var_qovd_dn8 = assign102030_e154020_d_n8;
        locals.var_qovd_dn9 = assign102030_e154020_d_n9;
        locals.var_qovd_dn10 = assign102030_e154020_d_n10;
        locals.var_qovd_dn11 = assign102030_e154020_d_n11;
        locals.var_qovd_dn14 = assign102030_e154020_d_n14;
        locals.var_qovd_rv = 0.0;

        let (assign102040_e154024, assign102040_e154024_d_n0, assign102040_e154024_d_n2, assign102040_e154024_d_n4, assign102040_e154024_d_n5, assign102040_e154024_d_n6, assign102040_e154024_d_n7, assign102040_e154024_d_n8, assign102040_e154024_d_n9, assign102040_e154024_d_n10, assign102040_e154024_d_n11, assign102040_e154024_d_n14,) = {
    if (p.p29 != 0.0) {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    }
};
        locals.var_qbdld = assign102040_e154024;
        locals.var_qbdld_dn0 = assign102040_e154024_d_n0;
        locals.var_qbdld_dn2 = assign102040_e154024_d_n2;
        locals.var_qbdld_dn4 = assign102040_e154024_d_n4;
        locals.var_qbdld_dn5 = assign102040_e154024_d_n5;
        locals.var_qbdld_dn6 = assign102040_e154024_d_n6;
        locals.var_qbdld_dn7 = assign102040_e154024_d_n7;
        locals.var_qbdld_dn8 = assign102040_e154024_d_n8;
        locals.var_qbdld_dn9 = assign102040_e154024_d_n9;
        locals.var_qbdld_dn10 = assign102040_e154024_d_n10;
        locals.var_qbdld_dn11 = assign102040_e154024_d_n11;
        locals.var_qbdld_dn14 = assign102040_e154024_d_n14;
        locals.var_qbdld_rv = 0.0;

        let (assign102050_e154029, assign102050_e154029_d_n0, assign102050_e154029_d_n2, assign102050_e154029_d_n4, assign102050_e154029_d_n5, assign102050_e154029_d_n6, assign102050_e154029_d_n7, assign102050_e154029_d_n8, assign102050_e154029_d_n9, assign102050_e154029_d_n10, assign102050_e154029_d_n11, assign102050_e154029_d_n14,) = {
    if (p.p29 == 0.0) {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn11, locals.var_qbd_qs_dn14,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    }
};
        locals.var_qbd_nqs = assign102050_e154029;
        locals.var_qbd_nqs_dn0 = assign102050_e154029_d_n0;
        locals.var_qbd_nqs_dn2 = assign102050_e154029_d_n2;
        locals.var_qbd_nqs_dn4 = assign102050_e154029_d_n4;
        locals.var_qbd_nqs_dn5 = assign102050_e154029_d_n5;
        locals.var_qbd_nqs_dn6 = assign102050_e154029_d_n6;
        locals.var_qbd_nqs_dn7 = assign102050_e154029_d_n7;
        locals.var_qbd_nqs_dn8 = assign102050_e154029_d_n8;
        locals.var_qbd_nqs_dn9 = assign102050_e154029_d_n9;
        locals.var_qbd_nqs_dn10 = assign102050_e154029_d_n10;
        locals.var_qbd_nqs_dn11 = assign102050_e154029_d_n11;
        locals.var_qbd_nqs_dn14 = assign102050_e154029_d_n14;
        locals.var_qbd_nqs_rv = 0.0;

        let assign102060_e154032: f64 = if p.p22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2333 = assign102060_e154032;
        locals.var_guard2333_rv = 0.0;

        let (assign102070_e154046, assign102070_e154046_d_n0, assign102070_e154046_d_n2, assign102070_e154046_d_n4, assign102070_e154046_d_n5, assign102070_e154046_d_n6, assign102070_e154046_d_n7, assign102070_e154046_d_n8, assign102070_e154046_d_n9, assign102070_e154046_d_n10, assign102070_e154046_d_n11, assign102070_e154046_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102070_e154037: f64 = (locals.var_qgbo - locals.var_qovd);
        let assign102070_e154039: f64 = (assign102070_e154037 - locals.var_qovs);
        let assign102070_e154041: f64 = (assign102070_e154039 + locals.var_qgos);
        let assign102070_e154043: f64 = (assign102070_e154041 + locals.var_qgod);
        let assign102070_e154044: f64 = (locals.var_mfactor * assign102070_e154043);
        (assign102070_e154044, (locals.var_mfactor * ((((-locals.var_qovd_dn0) - locals.var_qovs_dn0) + locals.var_qgos_dn0) + locals.var_qgod_dn0)), (locals.var_mfactor * ((((-locals.var_qovd_dn2) - locals.var_qovs_dn2) + locals.var_qgos_dn2) + locals.var_qgod_dn2)), (locals.var_mfactor * ((((-locals.var_qovd_dn4) - locals.var_qovs_dn4) + locals.var_qgos_dn4) + locals.var_qgod_dn4)), (locals.var_mfactor * ((((-locals.var_qovd_dn5) - locals.var_qovs_dn5) + locals.var_qgos_dn5) + locals.var_qgod_dn5)), (locals.var_mfactor * ((((-locals.var_qovd_dn6) - locals.var_qovs_dn6) + locals.var_qgos_dn6) + locals.var_qgod_dn6)), (locals.var_mfactor * ((((locals.var_qgbo_dn7 - locals.var_qovd_dn7) - locals.var_qovs_dn7) + locals.var_qgos_dn7) + locals.var_qgod_dn7)), (locals.var_mfactor * ((((locals.var_qgbo_dn8 - locals.var_qovd_dn8) - locals.var_qovs_dn8) + locals.var_qgos_dn8) + locals.var_qgod_dn8)), (locals.var_mfactor * ((((locals.var_qgbo_dn9 - locals.var_qovd_dn9) - locals.var_qovs_dn9) + locals.var_qgos_dn9) + locals.var_qgod_dn9)), (locals.var_mfactor * ((((-locals.var_qovd_dn10) - locals.var_qovs_dn10) + locals.var_qgos_dn10) + locals.var_qgod_dn10)), (locals.var_mfactor * ((((-locals.var_qovd_dn11) - locals.var_qovs_dn11) + locals.var_qgos_dn11) + locals.var_qgod_dn11)), (locals.var_mfactor * ((((-locals.var_qovd_dn14) - locals.var_qovs_dn14) + locals.var_qgos_dn14) + locals.var_qgod_dn14)),)
    } else {
        (locals.var_qgov, locals.var_qgov_dn0, locals.var_qgov_dn2, locals.var_qgov_dn4, locals.var_qgov_dn5, locals.var_qgov_dn6, locals.var_qgov_dn7, locals.var_qgov_dn8, locals.var_qgov_dn9, locals.var_qgov_dn10, locals.var_qgov_dn11, locals.var_qgov_dn14,)
    }
};
        locals.var_qgov = assign102070_e154046;
        locals.var_qgov_dn0 = assign102070_e154046_d_n0;
        locals.var_qgov_dn2 = assign102070_e154046_d_n2;
        locals.var_qgov_dn4 = assign102070_e154046_d_n4;
        locals.var_qgov_dn5 = assign102070_e154046_d_n5;
        locals.var_qgov_dn6 = assign102070_e154046_d_n6;
        locals.var_qgov_dn7 = assign102070_e154046_d_n7;
        locals.var_qgov_dn8 = assign102070_e154046_d_n8;
        locals.var_qgov_dn9 = assign102070_e154046_d_n9;
        locals.var_qgov_dn10 = assign102070_e154046_d_n10;
        locals.var_qgov_dn11 = assign102070_e154046_d_n11;
        locals.var_qgov_dn14 = assign102070_e154046_d_n14;
        locals.var_qgov_rv = 0.0;

        let (assign102080_e154055, assign102080_e154055_d_n0, assign102080_e154055_d_n2, assign102080_e154055_d_n4, assign102080_e154055_d_n5, assign102080_e154055_d_n6, assign102080_e154055_d_n7, assign102080_e154055_d_n8, assign102080_e154055_d_n9, assign102080_e154055_d_n10, assign102080_e154055_d_n11, assign102080_e154055_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102080_e154050: f64 = locals.var_qbdld;
        let assign102080_e154052: f64 = (assign102080_e154050 - locals.var_qgod);
        let assign102080_e154053: f64 = (locals.var_mfactor * assign102080_e154052);
        (assign102080_e154053, (locals.var_mfactor * (locals.var_qbdld_dn0 - locals.var_qgod_dn0)), (locals.var_mfactor * (locals.var_qbdld_dn2 - locals.var_qgod_dn2)), (locals.var_mfactor * (locals.var_qbdld_dn4 - locals.var_qgod_dn4)), (locals.var_mfactor * (locals.var_qbdld_dn5 - locals.var_qgod_dn5)), (locals.var_mfactor * (locals.var_qbdld_dn6 - locals.var_qgod_dn6)), (locals.var_mfactor * (locals.var_qbdld_dn7 - locals.var_qgod_dn7)), (locals.var_mfactor * (locals.var_qbdld_dn8 - locals.var_qgod_dn8)), (locals.var_mfactor * (locals.var_qbdld_dn9 - locals.var_qgod_dn9)), (locals.var_mfactor * (locals.var_qbdld_dn10 - locals.var_qgod_dn10)), (locals.var_mfactor * (locals.var_qbdld_dn11 - locals.var_qgod_dn11)), (locals.var_mfactor * (locals.var_qbdld_dn14 - locals.var_qgod_dn14)),)
    } else {
        (locals.var_qdov, locals.var_qdov_dn0, locals.var_qdov_dn2, locals.var_qdov_dn4, locals.var_qdov_dn5, locals.var_qdov_dn6, locals.var_qdov_dn7, locals.var_qdov_dn8, locals.var_qdov_dn9, locals.var_qdov_dn10, locals.var_qdov_dn11, locals.var_qdov_dn14,)
    }
};
        locals.var_qdov = assign102080_e154055;
        locals.var_qdov_dn0 = assign102080_e154055_d_n0;
        locals.var_qdov_dn2 = assign102080_e154055_d_n2;
        locals.var_qdov_dn4 = assign102080_e154055_d_n4;
        locals.var_qdov_dn5 = assign102080_e154055_d_n5;
        locals.var_qdov_dn6 = assign102080_e154055_d_n6;
        locals.var_qdov_dn7 = assign102080_e154055_d_n7;
        locals.var_qdov_dn8 = assign102080_e154055_d_n8;
        locals.var_qdov_dn9 = assign102080_e154055_d_n9;
        locals.var_qdov_dn10 = assign102080_e154055_d_n10;
        locals.var_qdov_dn11 = assign102080_e154055_d_n11;
        locals.var_qdov_dn14 = assign102080_e154055_d_n14;
        locals.var_qdov_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_392(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign102090_e154064, assign102090_e154064_d_n0, assign102090_e154064_d_n2, assign102090_e154064_d_n4, assign102090_e154064_d_n5, assign102090_e154064_d_n6, assign102090_e154064_d_n7, assign102090_e154064_d_n8, assign102090_e154064_d_n9, assign102090_e154064_d_n10, assign102090_e154064_d_n11, assign102090_e154064_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102090_e154059: f64 = locals.var_qbsld;
        let assign102090_e154061: f64 = (assign102090_e154059 - locals.var_qgos);
        let assign102090_e154062: f64 = (locals.var_mfactor * assign102090_e154061);
        (assign102090_e154062, (locals.var_mfactor * (locals.var_qbsld_dn0 - locals.var_qgos_dn0)), (locals.var_mfactor * (locals.var_qbsld_dn2 - locals.var_qgos_dn2)), (locals.var_mfactor * (locals.var_qbsld_dn4 - locals.var_qgos_dn4)), (locals.var_mfactor * (locals.var_qbsld_dn5 - locals.var_qgos_dn5)), (locals.var_mfactor * (locals.var_qbsld_dn6 - locals.var_qgos_dn6)), (locals.var_mfactor * (locals.var_qbsld_dn7 - locals.var_qgos_dn7)), (locals.var_mfactor * (locals.var_qbsld_dn8 - locals.var_qgos_dn8)), (locals.var_mfactor * (locals.var_qbsld_dn9 - locals.var_qgos_dn9)), (locals.var_mfactor * (locals.var_qbsld_dn10 - locals.var_qgos_dn10)), (locals.var_mfactor * (locals.var_qbsld_dn11 - locals.var_qgos_dn11)), (locals.var_mfactor * (locals.var_qbsld_dn14 - locals.var_qgos_dn14)),)
    } else {
        (locals.var_qsov, locals.var_qsov_dn0, locals.var_qsov_dn2, locals.var_qsov_dn4, locals.var_qsov_dn5, locals.var_qsov_dn6, locals.var_qsov_dn7, locals.var_qsov_dn8, locals.var_qsov_dn9, locals.var_qsov_dn10, locals.var_qsov_dn11, locals.var_qsov_dn14,)
    }
};
        locals.var_qsov = assign102090_e154064;
        locals.var_qsov_dn0 = assign102090_e154064_d_n0;
        locals.var_qsov_dn2 = assign102090_e154064_d_n2;
        locals.var_qsov_dn4 = assign102090_e154064_d_n4;
        locals.var_qsov_dn5 = assign102090_e154064_d_n5;
        locals.var_qsov_dn6 = assign102090_e154064_d_n6;
        locals.var_qsov_dn7 = assign102090_e154064_d_n7;
        locals.var_qsov_dn8 = assign102090_e154064_d_n8;
        locals.var_qsov_dn9 = assign102090_e154064_d_n9;
        locals.var_qsov_dn10 = assign102090_e154064_d_n10;
        locals.var_qsov_dn11 = assign102090_e154064_d_n11;
        locals.var_qsov_dn14 = assign102090_e154064_d_n14;
        locals.var_qsov_rv = 0.0;

        let (assign102100_e154077, assign102100_e154077_d_n0, assign102100_e154077_d_n2, assign102100_e154077_d_n4, assign102100_e154077_d_n5, assign102100_e154077_d_n6, assign102100_e154077_d_n7, assign102100_e154077_d_n8, assign102100_e154077_d_n9, assign102100_e154077_d_n10, assign102100_e154077_d_n11, assign102100_e154077_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102100_e154069: f64 = locals.var_qy;
        let assign102100_e154071: f64 = (assign102100_e154069 - locals.var_qovd_add);
        let assign102100_e154073: f64 = (assign102100_e154071 - locals.var_qovs_add);
        let assign102100_e154074: f64 = (locals.var_mfactor * assign102100_e154073);
        let assign102100_e154075: f64 = (locals.var_qge + assign102100_e154074);
        (assign102100_e154075, (locals.var_qge_dn0 + (locals.var_mfactor * ((locals.var_qy_dn0 - locals.var_qovd_add_dn0) - locals.var_qovs_add_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((locals.var_qy_dn2 - locals.var_qovd_add_dn2) - locals.var_qovs_add_dn2))), (locals.var_qge_dn4 + (locals.var_mfactor * ((locals.var_qy_dn4 - locals.var_qovd_add_dn4) - locals.var_qovs_add_dn4))), (locals.var_qge_dn5 + (locals.var_mfactor * ((locals.var_qy_dn5 - locals.var_qovd_add_dn5) - locals.var_qovs_add_dn5))), (locals.var_qge_dn6 + (locals.var_mfactor * ((locals.var_qy_dn6 - locals.var_qovd_add_dn6) - locals.var_qovs_add_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((locals.var_qy_dn7 - locals.var_qovd_add_dn7) - locals.var_qovs_add_dn7))), (locals.var_qge_dn8 + (locals.var_mfactor * ((locals.var_qy_dn8 - locals.var_qovd_add_dn8) - locals.var_qovs_add_dn8))), (locals.var_qge_dn9 + (locals.var_mfactor * ((locals.var_qy_dn9 - locals.var_qovd_add_dn9) - locals.var_qovs_add_dn9))), (locals.var_qge_dn10 + (locals.var_mfactor * ((locals.var_qy_dn10 - locals.var_qovd_add_dn10) - locals.var_qovs_add_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((locals.var_qy_dn11 - locals.var_qovd_add_dn11) - locals.var_qovs_add_dn11))), (locals.var_qge_dn14 + (locals.var_mfactor * ((locals.var_qy_dn14 - locals.var_qovd_add_dn14) - locals.var_qovs_add_dn14))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign102100_e154077;
        locals.var_qge_dn0 = assign102100_e154077_d_n0;
        locals.var_qge_dn2 = assign102100_e154077_d_n2;
        locals.var_qge_dn4 = assign102100_e154077_d_n4;
        locals.var_qge_dn5 = assign102100_e154077_d_n5;
        locals.var_qge_dn6 = assign102100_e154077_d_n6;
        locals.var_qge_dn7 = assign102100_e154077_d_n7;
        locals.var_qge_dn8 = assign102100_e154077_d_n8;
        locals.var_qge_dn9 = assign102100_e154077_d_n9;
        locals.var_qge_dn10 = assign102100_e154077_d_n10;
        locals.var_qge_dn11 = assign102100_e154077_d_n11;
        locals.var_qge_dn14 = assign102100_e154077_d_n14;
        locals.var_qge_rv = 0.0;

        let (assign102110_e154088, assign102110_e154088_d_n0, assign102110_e154088_d_n2, assign102110_e154088_d_n4, assign102110_e154088_d_n5, assign102110_e154088_d_n6, assign102110_e154088_d_n7, assign102110_e154088_d_n8, assign102110_e154088_d_n9, assign102110_e154088_d_n10, assign102110_e154088_d_n11, assign102110_e154088_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102110_e154082: f64 = (-locals.var_qy);
        let assign102110_e154084: f64 = (assign102110_e154082 + locals.var_qbdld_add);
        let assign102110_e154085: f64 = (locals.var_mfactor * assign102110_e154084);
        let assign102110_e154086: f64 = (locals.var_qde + assign102110_e154085);
        (assign102110_e154086, (locals.var_qde_dn0 + (locals.var_mfactor * ((-locals.var_qy_dn0) + locals.var_qbdld_add_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((-locals.var_qy_dn2) + locals.var_qbdld_add_dn2))), (locals.var_qde_dn4 + (locals.var_mfactor * ((-locals.var_qy_dn4) + locals.var_qbdld_add_dn4))), (locals.var_qde_dn5 + (locals.var_mfactor * ((-locals.var_qy_dn5) + locals.var_qbdld_add_dn5))), (locals.var_qde_dn6 + (locals.var_mfactor * ((-locals.var_qy_dn6) + locals.var_qbdld_add_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((-locals.var_qy_dn7) + locals.var_qbdld_add_dn7))), (locals.var_qde_dn8 + (locals.var_mfactor * ((-locals.var_qy_dn8) + locals.var_qbdld_add_dn8))), (locals.var_qde_dn9 + (locals.var_mfactor * ((-locals.var_qy_dn9) + locals.var_qbdld_add_dn9))), (locals.var_qde_dn10 + (locals.var_mfactor * ((-locals.var_qy_dn10) + locals.var_qbdld_add_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((-locals.var_qy_dn11) + locals.var_qbdld_add_dn11))), (locals.var_qde_dn14 + (locals.var_mfactor * ((-locals.var_qy_dn14) + locals.var_qbdld_add_dn14))),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign102110_e154088;
        locals.var_qde_dn0 = assign102110_e154088_d_n0;
        locals.var_qde_dn2 = assign102110_e154088_d_n2;
        locals.var_qde_dn4 = assign102110_e154088_d_n4;
        locals.var_qde_dn5 = assign102110_e154088_d_n5;
        locals.var_qde_dn6 = assign102110_e154088_d_n6;
        locals.var_qde_dn7 = assign102110_e154088_d_n7;
        locals.var_qde_dn8 = assign102110_e154088_d_n8;
        locals.var_qde_dn9 = assign102110_e154088_d_n9;
        locals.var_qde_dn10 = assign102110_e154088_d_n10;
        locals.var_qde_dn11 = assign102110_e154088_d_n11;
        locals.var_qde_dn14 = assign102110_e154088_d_n14;
        locals.var_qde_rv = 0.0;

        let (assign102120_e154097, assign102120_e154097_d_n0, assign102120_e154097_d_n2, assign102120_e154097_d_n4, assign102120_e154097_d_n5, assign102120_e154097_d_n6, assign102120_e154097_d_n7, assign102120_e154097_d_n8, assign102120_e154097_d_n9, assign102120_e154097_d_n10, assign102120_e154097_d_n11, assign102120_e154097_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102120_e154093: f64 = locals.var_qbsld_add;
        let assign102120_e154094: f64 = (locals.var_mfactor * assign102120_e154093);
        let assign102120_e154095: f64 = (locals.var_qse + assign102120_e154094);
        (assign102120_e154095, (locals.var_qse_dn0 + (locals.var_mfactor * locals.var_qbsld_add_dn0)), (locals.var_qse_dn2 + (locals.var_mfactor * locals.var_qbsld_add_dn2)), (locals.var_qse_dn4 + (locals.var_mfactor * locals.var_qbsld_add_dn4)), (locals.var_qse_dn5 + (locals.var_mfactor * locals.var_qbsld_add_dn5)), (locals.var_qse_dn6 + (locals.var_mfactor * locals.var_qbsld_add_dn6)), (locals.var_qse_dn7 + (locals.var_mfactor * locals.var_qbsld_add_dn7)), (locals.var_qse_dn8 + (locals.var_mfactor * locals.var_qbsld_add_dn8)), (locals.var_qse_dn9 + (locals.var_mfactor * locals.var_qbsld_add_dn9)), (locals.var_qse_dn10 + (locals.var_mfactor * locals.var_qbsld_add_dn10)), (locals.var_qse_dn11 + (locals.var_mfactor * locals.var_qbsld_add_dn11)), (locals.var_qse_dn14 + (locals.var_mfactor * locals.var_qbsld_add_dn14)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign102120_e154097;
        locals.var_qse_dn0 = assign102120_e154097_d_n0;
        locals.var_qse_dn2 = assign102120_e154097_d_n2;
        locals.var_qse_dn4 = assign102120_e154097_d_n4;
        locals.var_qse_dn5 = assign102120_e154097_d_n5;
        locals.var_qse_dn6 = assign102120_e154097_d_n6;
        locals.var_qse_dn7 = assign102120_e154097_d_n7;
        locals.var_qse_dn8 = assign102120_e154097_d_n8;
        locals.var_qse_dn9 = assign102120_e154097_d_n9;
        locals.var_qse_dn10 = assign102120_e154097_d_n10;
        locals.var_qse_dn11 = assign102120_e154097_d_n11;
        locals.var_qse_dn14 = assign102120_e154097_d_n14;
        locals.var_qse_rv = 0.0;

        let (assign102130_e154106, assign102130_e154106_d_n0, assign102130_e154106_d_n2, assign102130_e154106_d_n4, assign102130_e154106_d_n5, assign102130_e154106_d_n6, assign102130_e154106_d_n7, assign102130_e154106_d_n8, assign102130_e154106_d_n9, assign102130_e154106_d_n10, assign102130_e154106_d_n11, assign102130_e154106_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102130_e154101: f64 = (-locals.var_qovdext);
        let assign102130_e154103: f64 = (assign102130_e154101 - locals.var_qovsext);
        let assign102130_e154104: f64 = (locals.var_mfactor * assign102130_e154103);
        (assign102130_e154104, (locals.var_mfactor * ((-locals.var_qovdext_dn0) - locals.var_qovsext_dn0)), (locals.var_mfactor * ((-locals.var_qovdext_dn2) - locals.var_qovsext_dn2)), (locals.var_mfactor * ((-locals.var_qovdext_dn4) - locals.var_qovsext_dn4)), (locals.var_mfactor * ((-locals.var_qovdext_dn5) - locals.var_qovsext_dn5)), (locals.var_mfactor * ((-locals.var_qovdext_dn6) - locals.var_qovsext_dn6)), (locals.var_mfactor * ((-locals.var_qovdext_dn7) - locals.var_qovsext_dn7)), (locals.var_mfactor * ((-locals.var_qovdext_dn8) - locals.var_qovsext_dn8)), (locals.var_mfactor * ((-locals.var_qovdext_dn9) - locals.var_qovsext_dn9)), (locals.var_mfactor * ((-locals.var_qovdext_dn10) - locals.var_qovsext_dn10)), (locals.var_mfactor * ((-locals.var_qovdext_dn11) - locals.var_qovsext_dn11)), (locals.var_mfactor * ((-locals.var_qovdext_dn14) - locals.var_qovsext_dn14)),)
    } else {
        (locals.var_qgexte, locals.var_qgexte_dn0, locals.var_qgexte_dn2, locals.var_qgexte_dn4, locals.var_qgexte_dn5, locals.var_qgexte_dn6, locals.var_qgexte_dn7, locals.var_qgexte_dn8, locals.var_qgexte_dn9, locals.var_qgexte_dn10, locals.var_qgexte_dn11, locals.var_qgexte_dn14,)
    }
};
        locals.var_qgexte = assign102130_e154106;
        locals.var_qgexte_dn0 = assign102130_e154106_d_n0;
        locals.var_qgexte_dn2 = assign102130_e154106_d_n2;
        locals.var_qgexte_dn4 = assign102130_e154106_d_n4;
        locals.var_qgexte_dn5 = assign102130_e154106_d_n5;
        locals.var_qgexte_dn6 = assign102130_e154106_d_n6;
        locals.var_qgexte_dn7 = assign102130_e154106_d_n7;
        locals.var_qgexte_dn8 = assign102130_e154106_d_n8;
        locals.var_qgexte_dn9 = assign102130_e154106_d_n9;
        locals.var_qgexte_dn10 = assign102130_e154106_d_n10;
        locals.var_qgexte_dn11 = assign102130_e154106_d_n11;
        locals.var_qgexte_dn14 = assign102130_e154106_d_n14;
        locals.var_qgexte_rv = 0.0;

        let (assign102140_e154112, assign102140_e154112_d_n0, assign102140_e154112_d_n2, assign102140_e154112_d_n4, assign102140_e154112_d_n5, assign102140_e154112_d_n6, assign102140_e154112_d_n7, assign102140_e154112_d_n8, assign102140_e154112_d_n9, assign102140_e154112_d_n10, assign102140_e154112_d_n11, assign102140_e154112_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102140_e154110: f64 = (locals.var_mfactor * locals.var_qbdldext);
        (assign102140_e154110, (locals.var_mfactor * locals.var_qbdldext_dn0), (locals.var_mfactor * locals.var_qbdldext_dn2), (locals.var_mfactor * locals.var_qbdldext_dn4), (locals.var_mfactor * locals.var_qbdldext_dn5), (locals.var_mfactor * locals.var_qbdldext_dn6), (locals.var_mfactor * locals.var_qbdldext_dn7), (locals.var_mfactor * locals.var_qbdldext_dn8), (locals.var_mfactor * locals.var_qbdldext_dn9), (locals.var_mfactor * locals.var_qbdldext_dn10), (locals.var_mfactor * locals.var_qbdldext_dn11), (locals.var_mfactor * locals.var_qbdldext_dn14),)
    } else {
        (locals.var_qdexte, locals.var_qdexte_dn0, locals.var_qdexte_dn2, locals.var_qdexte_dn4, locals.var_qdexte_dn5, locals.var_qdexte_dn6, locals.var_qdexte_dn7, locals.var_qdexte_dn8, locals.var_qdexte_dn9, locals.var_qdexte_dn10, locals.var_qdexte_dn11, locals.var_qdexte_dn14,)
    }
};
        locals.var_qdexte = assign102140_e154112;
        locals.var_qdexte_dn0 = assign102140_e154112_d_n0;
        locals.var_qdexte_dn2 = assign102140_e154112_d_n2;
        locals.var_qdexte_dn4 = assign102140_e154112_d_n4;
        locals.var_qdexte_dn5 = assign102140_e154112_d_n5;
        locals.var_qdexte_dn6 = assign102140_e154112_d_n6;
        locals.var_qdexte_dn7 = assign102140_e154112_d_n7;
        locals.var_qdexte_dn8 = assign102140_e154112_d_n8;
        locals.var_qdexte_dn9 = assign102140_e154112_d_n9;
        locals.var_qdexte_dn10 = assign102140_e154112_d_n10;
        locals.var_qdexte_dn11 = assign102140_e154112_d_n11;
        locals.var_qdexte_dn14 = assign102140_e154112_d_n14;
        locals.var_qdexte_rv = 0.0;

        let (assign102150_e154118, assign102150_e154118_d_n0, assign102150_e154118_d_n2, assign102150_e154118_d_n4, assign102150_e154118_d_n5, assign102150_e154118_d_n6, assign102150_e154118_d_n7, assign102150_e154118_d_n8, assign102150_e154118_d_n9, assign102150_e154118_d_n10, assign102150_e154118_d_n11, assign102150_e154118_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102150_e154116: f64 = (locals.var_mfactor * locals.var_qbsldext);
        (assign102150_e154116, (locals.var_mfactor * locals.var_qbsldext_dn0), (locals.var_mfactor * locals.var_qbsldext_dn2), (locals.var_mfactor * locals.var_qbsldext_dn4), (locals.var_mfactor * locals.var_qbsldext_dn5), (locals.var_mfactor * locals.var_qbsldext_dn6), (locals.var_mfactor * locals.var_qbsldext_dn7), (locals.var_mfactor * locals.var_qbsldext_dn8), (locals.var_mfactor * locals.var_qbsldext_dn9), (locals.var_mfactor * locals.var_qbsldext_dn10), (locals.var_mfactor * locals.var_qbsldext_dn11), (locals.var_mfactor * locals.var_qbsldext_dn14),)
    } else {
        (locals.var_qsexte, locals.var_qsexte_dn0, locals.var_qsexte_dn2, locals.var_qsexte_dn4, locals.var_qsexte_dn5, locals.var_qsexte_dn6, locals.var_qsexte_dn7, locals.var_qsexte_dn8, locals.var_qsexte_dn9, locals.var_qsexte_dn10, locals.var_qsexte_dn11, locals.var_qsexte_dn14,)
    }
};
        locals.var_qsexte = assign102150_e154118;
        locals.var_qsexte_dn0 = assign102150_e154118_d_n0;
        locals.var_qsexte_dn2 = assign102150_e154118_d_n2;
        locals.var_qsexte_dn4 = assign102150_e154118_d_n4;
        locals.var_qsexte_dn5 = assign102150_e154118_d_n5;
        locals.var_qsexte_dn6 = assign102150_e154118_d_n6;
        locals.var_qsexte_dn7 = assign102150_e154118_d_n7;
        locals.var_qsexte_dn8 = assign102150_e154118_d_n8;
        locals.var_qsexte_dn9 = assign102150_e154118_d_n9;
        locals.var_qsexte_dn10 = assign102150_e154118_d_n10;
        locals.var_qsexte_dn11 = assign102150_e154118_d_n11;
        locals.var_qsexte_dn14 = assign102150_e154118_d_n14;
        locals.var_qsexte_rv = 0.0;

        let (assign102160_e154129, assign102160_e154129_d_n0, assign102160_e154129_d_n2, assign102160_e154129_d_n7,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102160_e154123: f64 = (-locals.var_qfd);
        let assign102160_e154125: f64 = (assign102160_e154123 - locals.var_qgdo);
        let assign102160_e154126: f64 = (locals.var_mfactor * assign102160_e154125);
        let assign102160_e154127: f64 = (locals.var_qdp + assign102160_e154126);
        (assign102160_e154127, (locals.var_qdp_dn0 + (locals.var_mfactor * ((-locals.var_qfd_dn0) - locals.var_qgdo_dn0))), (locals.var_qdp_dn2 + (locals.var_mfactor * ((-locals.var_qfd_dn2) - locals.var_qgdo_dn2))), (locals.var_qdp_dn7 + (locals.var_mfactor * ((-locals.var_qfd_dn7) - locals.var_qgdo_dn7))),)
    } else {
        (locals.var_qdp, locals.var_qdp_dn0, locals.var_qdp_dn2, locals.var_qdp_dn7,)
    }
};
        locals.var_qdp = assign102160_e154129;
        locals.var_qdp_dn0 = assign102160_e154129_d_n0;
        locals.var_qdp_dn2 = assign102160_e154129_d_n2;
        locals.var_qdp_dn7 = assign102160_e154129_d_n7;
        locals.var_qdp_rv = 0.0;

        let (assign102170_e154140, assign102170_e154140_d_n2, assign102170_e154140_d_n7,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign102170_e154134: f64 = (-locals.var_qfs);
        let assign102170_e154136: f64 = (assign102170_e154134 - locals.var_qgso);
        let assign102170_e154137: f64 = (locals.var_mfactor * assign102170_e154136);
        let assign102170_e154138: f64 = (locals.var_qsp + assign102170_e154137);
        (assign102170_e154138, (locals.var_qsp_dn2 + (locals.var_mfactor * ((-locals.var_qfs_dn2) - locals.var_qgso_dn2))), (locals.var_qsp_dn7 + (locals.var_mfactor * ((-locals.var_qfs_dn7) - locals.var_qgso_dn7))),)
    } else {
        (locals.var_qsp, locals.var_qsp_dn2, locals.var_qsp_dn7,)
    }
};
        locals.var_qsp = assign102170_e154140;
        locals.var_qsp_dn2 = assign102170_e154140_d_n2;
        locals.var_qsp_dn7 = assign102170_e154140_d_n7;
        locals.var_qsp_rv = 0.0;

        let assign102180_e154144: f64 = (locals.var_isub + locals.var_isubibpc);
        let assign102180_e154145: f64 = (locals.var_mfactor * assign102180_e154144);
        locals.var_isube = assign102180_e154145;
        locals.var_isube_dn0 = (locals.var_mfactor * (locals.var_isub_dn0 + locals.var_isubibpc_dn0));
        locals.var_isube_dn2 = (locals.var_mfactor * (locals.var_isub_dn2 + locals.var_isubibpc_dn2));
        locals.var_isube_dn4 = (locals.var_mfactor * (locals.var_isub_dn4 + locals.var_isubibpc_dn4));
        locals.var_isube_dn5 = (locals.var_mfactor * (locals.var_isub_dn5 + locals.var_isubibpc_dn5));
        locals.var_isube_dn6 = (locals.var_mfactor * (locals.var_isub_dn6 + locals.var_isubibpc_dn6));
        locals.var_isube_dn7 = (locals.var_mfactor * (locals.var_isub_dn7 + locals.var_isubibpc_dn7));
        locals.var_isube_dn8 = (locals.var_mfactor * (locals.var_isub_dn8 + locals.var_isubibpc_dn8));
        locals.var_isube_dn9 = (locals.var_mfactor * (locals.var_isub_dn9 + locals.var_isubibpc_dn9));
        locals.var_isube_dn10 = (locals.var_mfactor * (locals.var_isub_dn10 + locals.var_isubibpc_dn10));
        locals.var_isube_dn11 = (locals.var_mfactor * (locals.var_isub_dn11 + locals.var_isubibpc_dn11));
        locals.var_isube_dn14 = (locals.var_mfactor * (locals.var_isub_dn14 + locals.var_isubibpc_dn14));
        locals.var_isube_rv = 0.0;

        let assign102190_e154148: f64 = (locals.var_mfactor * locals.var_isubld);
        locals.var_isublde = assign102190_e154148;
        locals.var_isublde_dn0 = (locals.var_mfactor * locals.var_isubld_dn0);
        locals.var_isublde_dn2 = (locals.var_mfactor * locals.var_isubld_dn2);
        locals.var_isublde_dn4 = (locals.var_mfactor * locals.var_isubld_dn4);
        locals.var_isublde_dn5 = (locals.var_mfactor * locals.var_isubld_dn5);
        locals.var_isublde_dn6 = (locals.var_mfactor * locals.var_isubld_dn6);
        locals.var_isublde_dn7 = (locals.var_mfactor * locals.var_isubld_dn7);
        locals.var_isublde_dn8 = (locals.var_mfactor * locals.var_isubld_dn8);
        locals.var_isublde_dn9 = (locals.var_mfactor * locals.var_isubld_dn9);
        locals.var_isublde_dn10 = (locals.var_mfactor * locals.var_isubld_dn10);
        locals.var_isublde_dn11 = (locals.var_mfactor * locals.var_isubld_dn11);
        locals.var_isublde_dn14 = (locals.var_mfactor * locals.var_isubld_dn14);
        locals.var_isublde_rv = 0.0;

        let assign102310_e154215: f64 = (4.0 * 1.3806226e-23);
        let assign102310_e154217: f64 = (assign102310_e154215 * locals.var_ttemp);
        let assign102310_e154219: f64 = assign102310_e154217;
        locals.var_whi_noise = assign102310_e154219;
        locals.var_whi_noise_dn0 = (assign102310_e154215 * locals.var_ttemp_dn0);
        locals.var_whi_noise_dn2 = (assign102310_e154215 * locals.var_ttemp_dn2);
        locals.var_whi_noise_dn4 = (assign102310_e154215 * locals.var_ttemp_dn4);
        locals.var_whi_noise_dn5 = (assign102310_e154215 * locals.var_ttemp_dn5);
        locals.var_whi_noise_dn6 = (assign102310_e154215 * locals.var_ttemp_dn6);
        locals.var_whi_noise_dn7 = (assign102310_e154215 * locals.var_ttemp_dn7);
        locals.var_whi_noise_dn8 = (assign102310_e154215 * locals.var_ttemp_dn8);
        locals.var_whi_noise_dn9 = (assign102310_e154215 * locals.var_ttemp_dn9);
        locals.var_whi_noise_dn10 = (assign102310_e154215 * locals.var_ttemp_dn10);
        locals.var_whi_noise_dn11 = (assign102310_e154215 * locals.var_ttemp_dn11);
        locals.var_whi_noise_dn14 = (assign102310_e154215 * locals.var_ttemp_dn14);
        locals.var_whi_noise_rv = 0.0;

        let assign102330_e154225: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign102330_e154225;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn4 = (locals.var_mfactor * locals.var_nthrml_dn4);
        locals.var_noithrml_dn5 = (locals.var_mfactor * locals.var_nthrml_dn5);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn7 = (locals.var_mfactor * locals.var_nthrml_dn7);
        locals.var_noithrml_dn8 = (locals.var_mfactor * locals.var_nthrml_dn8);
        locals.var_noithrml_dn9 = (locals.var_mfactor * locals.var_nthrml_dn9);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn11 = (locals.var_mfactor * locals.var_nthrml_dn11);
        locals.var_noithrml_dn14 = (locals.var_mfactor * locals.var_nthrml_dn14);
        locals.var_noithrml_rv = 0.0;

        let assign102340_e154228: f64 = locals.var_qge_dn6;
        locals.var_cgdbd = assign102340_e154228;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn9 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn14 = 0.0;
        locals.var_cgdbd_rv = 0.0;

        let assign102350_e154231: f64 = (p.p87 * locals.var_cgdbd);
        locals.var_cgdbd = assign102350_e154231;
        locals.var_cgdbd_dn0 = (p.p87 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p87 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p87 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p87 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p87 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p87 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn8 = (p.p87 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn9 = (p.p87 * locals.var_cgdbd_dn9);
        locals.var_cgdbd_dn10 = (p.p87 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p87 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn14 = (p.p87 * locals.var_cgdbd_dn14);
        locals.var_cgdbd_rv = 0.0;

        let assign102360_e154234: f64 = locals.var_qge_dn8;
        locals.var_cgsbd = assign102360_e154234;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn9 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn14 = 0.0;
        locals.var_cgsbd_rv = 0.0;

        let assign102370_e154237: f64 = (p.p87 * locals.var_cgsbd);
        locals.var_cgsbd = assign102370_e154237;
        locals.var_cgsbd_dn0 = (p.p87 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p87 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p87 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p87 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p87 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p87 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn8 = (p.p87 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn9 = (p.p87 * locals.var_cgsbd_dn9);
        locals.var_cgsbd_dn10 = (p.p87 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p87 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn14 = (p.p87 * locals.var_cgsbd_dn14);
        locals.var_cgsbd_rv = 0.0;

        let (assign102380_e154243, assign102380_e154243_d_n0, assign102380_e154243_d_n2, assign102380_e154243_d_n4, assign102380_e154243_d_n5, assign102380_e154243_d_n6, assign102380_e154243_d_n7, assign102380_e154243_d_n8, assign102380_e154243_d_n9, assign102380_e154243_d_n10, assign102380_e154243_d_n11, assign102380_e154243_d_n14,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14,)
    }
};
        locals.var_cgsb = assign102380_e154243;
        locals.var_cgsb_dn0 = assign102380_e154243_d_n0;
        locals.var_cgsb_dn2 = assign102380_e154243_d_n2;
        locals.var_cgsb_dn4 = assign102380_e154243_d_n4;
        locals.var_cgsb_dn5 = assign102380_e154243_d_n5;
        locals.var_cgsb_dn6 = assign102380_e154243_d_n6;
        locals.var_cgsb_dn7 = assign102380_e154243_d_n7;
        locals.var_cgsb_dn8 = assign102380_e154243_d_n8;
        locals.var_cgsb_dn9 = assign102380_e154243_d_n9;
        locals.var_cgsb_dn10 = assign102380_e154243_d_n10;
        locals.var_cgsb_dn11 = assign102380_e154243_d_n11;
        locals.var_cgsb_dn14 = assign102380_e154243_d_n14;
        locals.var_cgsb_rv = 0.0;

        locals.var_noiigate = 0.0;
        locals.var_noiigate_dn0 = 0.0;
        locals.var_noiigate_dn2 = 0.0;
        locals.var_noiigate_dn4 = 0.0;
        locals.var_noiigate_dn5 = 0.0;
        locals.var_noiigate_dn6 = 0.0;
        locals.var_noiigate_dn7 = 0.0;
        locals.var_noiigate_dn8 = 0.0;
        locals.var_noiigate_dn9 = 0.0;
        locals.var_noiigate_dn10 = 0.0;
        locals.var_noiigate_dn11 = 0.0;
        locals.var_noiigate_dn14 = 0.0;
        locals.var_noiigate_rv = 0.0;

        let assign102410_e154263: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2336 = assign102410_e154263;
        locals.var_guard2336_rv = 0.0;

        let (assign102420_e154273, assign102420_e154273_d_n0, assign102420_e154273_d_n2, assign102420_e154273_d_n4, assign102420_e154273_d_n5, assign102420_e154273_d_n6, assign102420_e154273_d_n7, assign102420_e154273_d_n8, assign102420_e154273_d_n9, assign102420_e154273_d_n10, assign102420_e154273_d_n11, assign102420_e154273_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let assign102420_e154267: f64 = (1e-6 * locals.var_cox);
        let assign102420_e154269: f64 = (assign102420_e154267 * locals.var_weffcv_nf);
        let assign102420_e154271: f64 = (assign102420_e154269 * locals.var_leff);
        (assign102420_e154271, (((1e-6 * locals.var_cox_dn0) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn2) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn4) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn5) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn6) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn7) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn8) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn9) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn10) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn11) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn14) * locals.var_weffcv_nf) * locals.var_leff),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign102420_e154273;
        locals.var_t0_dn0 = assign102420_e154273_d_n0;
        locals.var_t0_dn2 = assign102420_e154273_d_n2;
        locals.var_t0_dn4 = assign102420_e154273_d_n4;
        locals.var_t0_dn5 = assign102420_e154273_d_n5;
        locals.var_t0_dn6 = assign102420_e154273_d_n6;
        locals.var_t0_dn7 = assign102420_e154273_d_n7;
        locals.var_t0_dn8 = assign102420_e154273_d_n8;
        locals.var_t0_dn9 = assign102420_e154273_d_n9;
        locals.var_t0_dn10 = assign102420_e154273_d_n10;
        locals.var_t0_dn11 = assign102420_e154273_d_n11;
        locals.var_t0_dn14 = assign102420_e154273_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign102430_e154279, assign102430_e154279_d_n0, assign102430_e154279_d_n2, assign102430_e154279_d_n4, assign102430_e154279_d_n5, assign102430_e154279_d_n6, assign102430_e154279_d_n7, assign102430_e154279_d_n8, assign102430_e154279_d_n9, assign102430_e154279_d_n10, assign102430_e154279_d_n11, assign102430_e154279_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let assign102430_e154277: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign102430_e154277, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn4 / locals.var_mfactor), (locals.var_cgsb_dn5 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn8 / locals.var_mfactor), (locals.var_cgsb_dn9 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign102430_e154279;
        locals.var_t10_dn0 = assign102430_e154279_d_n0;
        locals.var_t10_dn2 = assign102430_e154279_d_n2;
        locals.var_t10_dn4 = assign102430_e154279_d_n4;
        locals.var_t10_dn5 = assign102430_e154279_d_n5;
        locals.var_t10_dn6 = assign102430_e154279_d_n6;
        locals.var_t10_dn7 = assign102430_e154279_d_n7;
        locals.var_t10_dn8 = assign102430_e154279_d_n8;
        locals.var_t10_dn9 = assign102430_e154279_d_n9;
        locals.var_t10_dn10 = assign102430_e154279_d_n10;
        locals.var_t10_dn11 = assign102430_e154279_d_n11;
        locals.var_t10_dn14 = assign102430_e154279_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign102440_e154293, assign102440_e154293_d_n0, assign102440_e154293_d_n2, assign102440_e154293_d_n4, assign102440_e154293_d_n5, assign102440_e154293_d_n6, assign102440_e154293_d_n7, assign102440_e154293_d_n8, assign102440_e154293_d_n9, assign102440_e154293_d_n10, assign102440_e154293_d_n11, assign102440_e154293_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let assign102440_e154283: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign102440_e154285: f64 = (assign102440_e154283 * locals.var_beta_inv);
        let assign102440_e154287: f64 = (assign102440_e154285 * locals.var_t10);
        let assign102440_e154289: f64 = (assign102440_e154287 * locals.var_t10);
        let assign102440_e154291: f64 = (assign102440_e154289 / locals.var_gds0_ign);
        (assign102440_e154291, ((((((((assign102440_e154283 * locals.var_beta_inv_dn0) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn0)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn0)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn2) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn2)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn2)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn4) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn4)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn4)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn4)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn5) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn5)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn5)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn5)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn6) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn6)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn6)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn7) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn7)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn7)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn8) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn8)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn8)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn8)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn9) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn9)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn9)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn9)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn10) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn10)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn10)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn11) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn11)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn11)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102440_e154283 * locals.var_beta_inv_dn14) * locals.var_t10) + (assign102440_e154285 * locals.var_t10_dn14)) * locals.var_t10) + (assign102440_e154287 * locals.var_t10_dn14)) * locals.var_gds0_ign) - (assign102440_e154289 * locals.var_gds0_ign_dn14)) / (locals.var_gds0_ign * locals.var_gds0_ign)),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn4, locals.var_nign0_dn5, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn8, locals.var_nign0_dn9, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn14,)
    }
};
        locals.var_nign0 = assign102440_e154293;
        locals.var_nign0_dn0 = assign102440_e154293_d_n0;
        locals.var_nign0_dn2 = assign102440_e154293_d_n2;
        locals.var_nign0_dn4 = assign102440_e154293_d_n4;
        locals.var_nign0_dn5 = assign102440_e154293_d_n5;
        locals.var_nign0_dn6 = assign102440_e154293_d_n6;
        locals.var_nign0_dn7 = assign102440_e154293_d_n7;
        locals.var_nign0_dn8 = assign102440_e154293_d_n8;
        locals.var_nign0_dn9 = assign102440_e154293_d_n9;
        locals.var_nign0_dn10 = assign102440_e154293_d_n10;
        locals.var_nign0_dn11 = assign102440_e154293_d_n11;
        locals.var_nign0_dn14 = assign102440_e154293_d_n14;
        locals.var_nign0_rv = 0.0;

        let assign102450_e154297: f64 = (10.0 * 2.220446049250313e-16);
        let assign102450_e154302: f64 = (10.0 * 2.220446049250313e-16);
        let assign102450_e154304: f64 = if ((locals.var_kusai00l > assign102450_e154297) && (locals.var_vds > assign102450_e154302)) { 1.0 } else { 0.0 };
        locals.var_guard2337 = assign102450_e154304;
        locals.var_guard2337_rv = 0.0;

        let (assign102460_e154312, assign102460_e154312_d_n0, assign102460_e154312_d_n2, assign102460_e154312_d_n4, assign102460_e154312_d_n5, assign102460_e154312_d_n6, assign102460_e154312_d_n7, assign102460_e154312_d_n8, assign102460_e154312_d_n9, assign102460_e154312_d_n10, assign102460_e154312_d_n11, assign102460_e154312_d_n14,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 != 0.0)) {
        let assign102460_e154310: f64 = (locals.var_muun / locals.var_mu);
        (assign102460_e154310, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn4 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn4)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn5 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn5)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn8 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn8)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn9 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn9)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn14 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn14)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn4, locals.var_mumoda_dn5, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn8, locals.var_mumoda_dn9, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn14,)
    }
};
        locals.var_mumoda = assign102460_e154312;
        locals.var_mumoda_dn0 = assign102460_e154312_d_n0;
        locals.var_mumoda_dn2 = assign102460_e154312_d_n2;
        locals.var_mumoda_dn4 = assign102460_e154312_d_n4;
        locals.var_mumoda_dn5 = assign102460_e154312_d_n5;
        locals.var_mumoda_dn6 = assign102460_e154312_d_n6;
        locals.var_mumoda_dn7 = assign102460_e154312_d_n7;
        locals.var_mumoda_dn8 = assign102460_e154312_d_n8;
        locals.var_mumoda_dn9 = assign102460_e154312_d_n9;
        locals.var_mumoda_dn10 = assign102460_e154312_d_n10;
        locals.var_mumoda_dn11 = assign102460_e154312_d_n11;
        locals.var_mumoda_dn14 = assign102460_e154312_d_n14;
        locals.var_mumoda_rv = 0.0;

        let (assign102470_e154324, assign102470_e154324_d_n0, assign102470_e154324_d_n2, assign102470_e154324_d_n4, assign102470_e154324_d_n5, assign102470_e154324_d_n6, assign102470_e154324_d_n7, assign102470_e154324_d_n8, assign102470_e154324_d_n9, assign102470_e154324_d_n10, assign102470_e154324_d_n11, assign102470_e154324_d_n14,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2337 != 0.0)) {
        let assign102470_e154318: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign102470_e154320: f64 = (assign102470_e154318 - locals.var_mumoda);
        let assign102470_e154322: f64 = (assign102470_e154320 / locals.var_vds);
        (assign102470_e154322, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn4) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn4)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn5) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn5)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn8) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn8)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn9) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn9)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn14 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn14)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn14) * locals.var_vds) - (assign102470_e154320 * locals.var_vds_dn14)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn4, locals.var_mumodb_dn5, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn8, locals.var_mumodb_dn9, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn14,)
    }
};
        locals.var_mumodb = assign102470_e154324;
        locals.var_mumodb_dn0 = assign102470_e154324_d_n0;
        locals.var_mumodb_dn2 = assign102470_e154324_d_n2;
        locals.var_mumodb_dn4 = assign102470_e154324_d_n4;
        locals.var_mumodb_dn5 = assign102470_e154324_d_n5;
        locals.var_mumodb_dn6 = assign102470_e154324_d_n6;
        locals.var_mumodb_dn7 = assign102470_e154324_d_n7;
        locals.var_mumodb_dn8 = assign102470_e154324_d_n8;
        locals.var_mumodb_dn9 = assign102470_e154324_d_n9;
        locals.var_mumodb_dn10 = assign102470_e154324_d_n10;
        locals.var_mumodb_dn11 = assign102470_e154324_d_n11;
        locals.var_mumodb_dn14 = assign102470_e154324_d_n14;
        locals.var_mumodb_rv = 0.0;

    }
}
