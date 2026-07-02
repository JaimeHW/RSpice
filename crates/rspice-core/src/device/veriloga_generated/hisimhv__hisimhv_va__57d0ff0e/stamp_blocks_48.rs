#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_377(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

        let assign97260_e149700: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2259 = assign97260_e149700;
        locals.var_guard2259_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign97280_e149711: f64 = (-3.0);
        let assign97280_e149713: f64 = (assign97280_e149711 * 34.0);
        let assign97280_e149714: f64 = if locals.var_tx < assign97280_e149713 { 1.0 } else { 0.0 };
        locals.var_guard2260 = assign97280_e149714;
        locals.var_guard2260_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

        let assign97380_e149817: f64 = if locals.var_isbd_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2261 = assign97380_e149817;
        locals.var_guard2261_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

        let assign97430_e149842: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2262 = assign97430_e149842;
        locals.var_guard2262_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign97450_e149853: f64 = (-3.0);
        let assign97450_e149855: f64 = (assign97450_e149853 * 34.0);
        let assign97450_e149856: f64 = if locals.var_tx < assign97450_e149855 { 1.0 } else { 0.0 };
        locals.var_guard2263 = assign97450_e149856;
        locals.var_guard2263_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

        let assign97550_e149959: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2264 = assign97550_e149959;
        locals.var_guard2264_rv = 0.0;

        let assign97560_e149962: f64 = if locals.var_isbd_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2265 = assign97560_e149962;
        locals.var_guard2265_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_378(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

        let assign97610_e149995: f64 = if locals.var_vbdi_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2266 = assign97610_e149995;
        locals.var_guard2266_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign97630_e150008: f64 = (-3.0);
        let assign97630_e150010: f64 = (assign97630_e150008 * 34.0);
        let assign97630_e150011: f64 = if locals.var_tx < assign97630_e150010 { 1.0 } else { 0.0 };
        locals.var_guard2267 = assign97630_e150011;
        locals.var_guard2267_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

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
        locals.var_t10_rv = 0.0;

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
        locals.var_t9_rv = 0.0;

        let assign97760_e150145: f64 = if locals.var_isbs_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2268 = assign97760_e150145;
        locals.var_guard2268_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

        let assign97810_e150170: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2269 = assign97810_e150170;
        locals.var_guard2269_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign97830_e150181: f64 = (-3.0);
        let assign97830_e150183: f64 = (assign97830_e150181 * 34.0);
        let assign97830_e150184: f64 = if locals.var_tx < assign97830_e150183 { 1.0 } else { 0.0 };
        locals.var_guard2270 = assign97830_e150184;
        locals.var_guard2270_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

        let assign97930_e150287: f64 = if locals.var_isbs_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2271 = assign97930_e150287;
        locals.var_guard2271_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_379(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t3_rv = 0.0;

        let assign97980_e150312: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2272 = assign97980_e150312;
        locals.var_guard2272_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign98000_e150323: f64 = (-3.0);
        let assign98000_e150325: f64 = (assign98000_e150323 * 34.0);
        let assign98000_e150326: f64 = if locals.var_tx < assign98000_e150325 { 1.0 } else { 0.0 };
        locals.var_guard2273 = assign98000_e150326;
        locals.var_guard2273_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

        let assign98100_e150429: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2274 = assign98100_e150429;
        locals.var_guard2274_rv = 0.0;

        let assign98110_e150432: f64 = if locals.var_isbs_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2275 = assign98110_e150432;
        locals.var_guard2275_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_t3_rv = 0.0;

        let assign98160_e150465: f64 = if locals.var_vbsi_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2276 = assign98160_e150465;
        locals.var_guard2276_rv = 0.0;

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
        locals.var_tx_rv = 0.0;

        let assign98180_e150478: f64 = (-3.0);
        let assign98180_e150480: f64 = (assign98180_e150478 * 34.0);
        let assign98180_e150481: f64 = if locals.var_tx < assign98180_e150480 { 1.0 } else { 0.0 };
        locals.var_guard2277 = assign98180_e150481;
        locals.var_guard2277_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t4_rv = 0.0;

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
        locals.var_t12_rv = 0.0;

        let assign98290_e150609: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2278 = assign98290_e150609;
        locals.var_guard2278_rv = 0.0;

        let assign98300_e150612: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2279 = assign98300_e150612;
        locals.var_guard2279_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign98320_e150625: f64 = if p.p503 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2280 = assign98320_e150625;
        locals.var_guard2280_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbd_btm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_380(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbd_btm_rv = 0.0;

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
        locals.var_qbd_btm_rv = 0.0;

        let assign98430_e150736: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2281 = assign98430_e150736;
        locals.var_guard2281_rv = 0.0;

        let assign98440_e150739: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2282 = assign98440_e150739;
        locals.var_guard2282_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign98460_e150752: f64 = if p.p504 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2283 = assign98460_e150752;
        locals.var_guard2283_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbd_sws_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbd_sws_rv = 0.0;

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
        locals.var_qbd_sws_rv = 0.0;

        let assign98570_e150863: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2284 = assign98570_e150863;
        locals.var_guard2284_rv = 0.0;

        let assign98580_e150866: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2285 = assign98580_e150866;
        locals.var_guard2285_rv = 0.0;

        let assign98590_e150869: f64 = if locals.var_vbdi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2286 = assign98590_e150869;
        locals.var_guard2286_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign98610_e150884: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2287 = assign98610_e150884;
        locals.var_guard2287_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

        let assign98720_e151015: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2288 = assign98720_e151015;
        locals.var_guard2288_rv = 0.0;

        let assign98730_e151018: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2289 = assign98730_e151018;
        locals.var_guard2289_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_381(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_arg_rv = 0.0;

        let assign98750_e151034: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2290 = assign98750_e151034;
        locals.var_guard2290_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

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
        locals.var_qbd_swg_rv = 0.0;

        let assign98860_e151175: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2291 = assign98860_e151175;
        locals.var_guard2291_rv = 0.0;

        let assign98870_e151178: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2292 = assign98870_e151178;
        locals.var_guard2292_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign98890_e151191: f64 = if p.p526 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2293 = assign98890_e151191;
        locals.var_guard2293_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbs_btm_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbs_btm_rv = 0.0;

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
        locals.var_qbs_btm_rv = 0.0;

        let assign99000_e151302: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2294 = assign99000_e151302;
        locals.var_guard2294_rv = 0.0;

        let assign99010_e151305: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2295 = assign99010_e151305;
        locals.var_guard2295_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign99030_e151318: f64 = if p.p527 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2296 = assign99030_e151318;
        locals.var_guard2296_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_382(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_qbs_sws_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbs_sws_rv = 0.0;

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
        locals.var_qbs_sws_rv = 0.0;

        let assign99140_e151429: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2297 = assign99140_e151429;
        locals.var_guard2297_rv = 0.0;

        let assign99150_e151432: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2298 = assign99150_e151432;
        locals.var_guard2298_rv = 0.0;

        let assign99160_e151435: f64 = if locals.var_vbsi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2299 = assign99160_e151435;
        locals.var_guard2299_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign99180_e151450: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2300 = assign99180_e151450;
        locals.var_guard2300_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbs_swg_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbs_swg_rv = 0.0;

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
        locals.var_qbs_swg_rv = 0.0;

        let assign99290_e151581: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2301 = assign99290_e151581;
        locals.var_guard2301_rv = 0.0;

        let assign99300_e151584: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2302 = assign99300_e151584;
        locals.var_guard2302_rv = 0.0;

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
        locals.var_arg_rv = 0.0;

        let assign99320_e151600: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2303 = assign99320_e151600;
        locals.var_guard2303_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_sarg_rv = 0.0;

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
        locals.var_qbs_swg_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_qbs_swg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_383(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_qbs_swg_rv = 0.0;

        let assign99450_e151751: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2304 = assign99450_e151751;
        locals.var_guard2304_rv = 0.0;

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
        locals.var_qbs_rv = 0.0;

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
        locals.var_qbd_rv = 0.0;

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
        locals.var_qbsi_rv = 0.0;

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
        locals.var_qbdi_rv = 0.0;

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
        locals.var_qbs_rv = 0.0;

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
        locals.var_qbd_rv = 0.0;

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
        locals.var_qbsi_rv = 0.0;

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
        locals.var_qbdi_rv = 0.0;

        let assign99660_e151896: f64 = (p.p540 / 1e-6);
        locals.var_ndi_i = assign99660_e151896;
        locals.var_ndi_i_rv = 0.0;

        locals.var_njl = locals.var_uc_njd;
        locals.var_njl_rv = 0.0;

        let assign99680_e151900: f64 = (1450.0 / 10000.0);
        locals.var_muen_i = assign99680_e151900;
        locals.var_muen_i_rv = 0.0;

        let assign99690_e151903: f64 = (500.0 / 10000.0);
        locals.var_muep_i = assign99690_e151903;
        locals.var_muep_i_rv = 0.0;

        locals.var_juncdlt = 0.001;
        locals.var_juncdlt_rv = 0.0;

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
        locals.var_nin_dio_rv = 0.0;

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
        locals.var_pn0_rv = 0.0;

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
        locals.var_t1_rv = 0.0;

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
        locals.var_dn_rv = 0.0;

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
        locals.var_dp_rv = 0.0;

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
        locals.var_da_rv = 0.0;

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
        locals.var_t2_rv = 0.0;

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
        locals.var_tau_hl_rv = 0.0;

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
        locals.var_la_rv = 0.0;

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
        locals.var_v_ha_rv = 0.0;

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
        locals.var_v_hk_rv = 0.0;

        let assign99820_e151981: f64 = if p.p539 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2305 = assign99820_e151981;
        locals.var_guard2305_rv = 0.0;

        let (assign99830_e151985,) = {
    if (locals.var_guard2305 != 0.0) {
        (locals.var_uc_njd,)
    } else {
        (locals.var_nj_k,)
    }
};
        locals.var_nj_k = assign99830_e151985;
        locals.var_nj_k_rv = 0.0;

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
        locals.var_exp_a_rv = 0.0;

        let assign99850_e151996: f64 = (locals.var_v_hk - locals.var_v_ha);
        let assign99850_e151997: f64 = (locals.var_vbd_jct - assign99850_e151996);
        let assign99850_e151999: f64 = if assign99850_e151997 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2306 = assign99850_e151999;
        locals.var_guard2306_rv = 0.0;

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
        locals.var_exp_k_rv = 0.0;

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
        locals.var_exp_k_rv = 0.0;

        let assign99880_e152030: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_ha)) { 1.0 } else { 0.0 };
        locals.var_guard2307 = assign99880_e152030;
        locals.var_guard2307_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_384(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
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
        locals.var_exp_a2_rv = 0.0;

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
        locals.var_exp_a2_rv = 0.0;

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
        locals.var_exp_a2_rv = 0.0;

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
        locals.var_p_na_rv = 0.0;

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
        locals.var_q_pexa_rv = 0.0;

        let assign99940_e152095: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2308 = assign99940_e152095;
        locals.var_guard2308_rv = 0.0;

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
        locals.var_q_qs_a_rv = 0.0;

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
        locals.var_q_nqs_a_rv = 0.0;

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
        locals.var_inqs0_a_rv = 0.0;

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
        locals.var_q_pexa_nqs_rv = 0.0;

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
        locals.var_q_qs_a_rv = 0.0;

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
        locals.var_q_pexa_nqs_rv = 0.0;

        let assign100010_e152150: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_hk)) { 1.0 } else { 0.0 };
        locals.var_guard2309 = assign100010_e152150;
        locals.var_guard2309_rv = 0.0;

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
        locals.var_exp_k2_rv = 0.0;

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
        locals.var_exp_k2_rv = 0.0;

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
        locals.var_exp_k2_rv = 0.0;

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
        locals.var_p_nk_rv = 0.0;

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
        locals.var_q_pexk_rv = 0.0;

        let assign100070_e152215: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2310 = assign100070_e152215;
        locals.var_guard2310_rv = 0.0;

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
        locals.var_q_qs_k_rv = 0.0;

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
        locals.var_q_nqs_k_rv = 0.0;

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
        locals.var_inqs0_k_rv = 0.0;

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
        locals.var_q_pexk_nqs_rv = 0.0;

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
        locals.var_q_qs_k_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_385(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv18 = ctx.node_voltage(nodes[18]);
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
        locals.var_q_pexk_nqs_rv = 0.0;

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
        locals.var_vjunc_a_rv = 0.0;

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
        locals.var_tmf2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_vjunc_a_rv = 0.0;

        let assign100180_e152303: f64 = if locals.var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2311 = assign100180_e152303;
        locals.var_guard2311_rv = 0.0;

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
        locals.var_vjunc_a_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_w_depa_rv = 0.0;

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
        locals.var_tmf1_rv = 0.0;

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
        locals.var_tmf2_rv = 0.0;

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
        locals.var_tmf2_rv = 0.0;

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
        locals.var_tmf2_rv = 0.0;

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
        locals.var_t0_rv = 0.0;

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
        locals.var_w_depa_rv = 0.0;

        let assign100280_e152386: f64 = if p.p546 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2312 = assign100280_e152386;
        locals.var_guard2312_rv = 0.0;

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
        locals.var_w_qs_a_rv = 0.0;

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
        locals.var_w_nqs_a_rv = 0.0;

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
        locals.var_iwnqs0_a_rv = 0.0;

        let (assign100320_e152420, assign100320_e152420_d_n0, assign100320_e152420_d_n2, assign100320_e152420_d_n4, assign100320_e152420_d_n5, assign100320_e152420_d_n6, assign100320_e152420_d_n7, assign100320_e152420_d_n8, assign100320_e152420_d_n9, assign100320_e152420_d_n10, assign100320_e152420_d_n11, assign100320_e152420_d_n14, assign100320_e152420_d_n18,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100320_e152418: f64 = (locals.var_w_nqs_a / p.p546);
        (assign100320_e152418, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_w_nqs_a_dn18 / p.p546),)
    } else {
        (locals.var_w_depa_nqs, locals.var_w_depa_nqs_dn0, locals.var_w_depa_nqs_dn2, locals.var_w_depa_nqs_dn4, locals.var_w_depa_nqs_dn5, locals.var_w_depa_nqs_dn6, locals.var_w_depa_nqs_dn7, locals.var_w_depa_nqs_dn8, locals.var_w_depa_nqs_dn9, locals.var_w_depa_nqs_dn10, locals.var_w_depa_nqs_dn11, locals.var_w_depa_nqs_dn14, locals.var_w_depa_nqs_dn18,)
    }
};
        locals.var_w_depa_nqs = assign100320_e152420;
        locals.var_w_depa_nqs_dn0 = assign100320_e152420_d_n0;
        locals.var_w_depa_nqs_dn2 = assign100320_e152420_d_n2;
        locals.var_w_depa_nqs_dn4 = assign100320_e152420_d_n4;
        locals.var_w_depa_nqs_dn5 = assign100320_e152420_d_n5;
        locals.var_w_depa_nqs_dn6 = assign100320_e152420_d_n6;
        locals.var_w_depa_nqs_dn7 = assign100320_e152420_d_n7;
        locals.var_w_depa_nqs_dn8 = assign100320_e152420_d_n8;
        locals.var_w_depa_nqs_dn9 = assign100320_e152420_d_n9;
        locals.var_w_depa_nqs_dn10 = assign100320_e152420_d_n10;
        locals.var_w_depa_nqs_dn11 = assign100320_e152420_d_n11;
        locals.var_w_depa_nqs_dn14 = assign100320_e152420_d_n14;
        locals.var_w_depa_nqs_dn18 = assign100320_e152420_d_n18;
        locals.var_w_depa_nqs_rv = 0.0;

        let (assign100330_e152427, assign100330_e152427_d_n0, assign100330_e152427_d_n2, assign100330_e152427_d_n4, assign100330_e152427_d_n5, assign100330_e152427_d_n6, assign100330_e152427_d_n7, assign100330_e152427_d_n8, assign100330_e152427_d_n9, assign100330_e152427_d_n10, assign100330_e152427_d_n11, assign100330_e152427_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 == 0.0)) {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn11, locals.var_w_depa_dn14,)
    } else {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn11, locals.var_w_qs_a_dn14,)
    }
};
        locals.var_w_qs_a = assign100330_e152427;
        locals.var_w_qs_a_dn0 = assign100330_e152427_d_n0;
        locals.var_w_qs_a_dn2 = assign100330_e152427_d_n2;
        locals.var_w_qs_a_dn4 = assign100330_e152427_d_n4;
        locals.var_w_qs_a_dn5 = assign100330_e152427_d_n5;
        locals.var_w_qs_a_dn6 = assign100330_e152427_d_n6;
        locals.var_w_qs_a_dn7 = assign100330_e152427_d_n7;
        locals.var_w_qs_a_dn8 = assign100330_e152427_d_n8;
        locals.var_w_qs_a_dn9 = assign100330_e152427_d_n9;
        locals.var_w_qs_a_dn10 = assign100330_e152427_d_n10;
        locals.var_w_qs_a_dn11 = assign100330_e152427_d_n11;
        locals.var_w_qs_a_dn14 = assign100330_e152427_d_n14;
        locals.var_w_qs_a_rv = 0.0;

        let (assign100340_e152434, assign100340_e152434_d_n0, assign100340_e152434_d_n2, assign100340_e152434_d_n4, assign100340_e152434_d_n5, assign100340_e152434_d_n6, assign100340_e152434_d_n7, assign100340_e152434_d_n8, assign100340_e152434_d_n9, assign100340_e152434_d_n10, assign100340_e152434_d_n11, assign100340_e152434_d_n14, assign100340_e152434_d_n18,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 == 0.0)) {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn11, locals.var_w_qs_a_dn14, 0.0,)
    } else {
        (locals.var_w_depa_nqs, locals.var_w_depa_nqs_dn0, locals.var_w_depa_nqs_dn2, locals.var_w_depa_nqs_dn4, locals.var_w_depa_nqs_dn5, locals.var_w_depa_nqs_dn6, locals.var_w_depa_nqs_dn7, locals.var_w_depa_nqs_dn8, locals.var_w_depa_nqs_dn9, locals.var_w_depa_nqs_dn10, locals.var_w_depa_nqs_dn11, locals.var_w_depa_nqs_dn14, locals.var_w_depa_nqs_dn18,)
    }
};
        locals.var_w_depa_nqs = assign100340_e152434;
        locals.var_w_depa_nqs_dn0 = assign100340_e152434_d_n0;
        locals.var_w_depa_nqs_dn2 = assign100340_e152434_d_n2;
        locals.var_w_depa_nqs_dn4 = assign100340_e152434_d_n4;
        locals.var_w_depa_nqs_dn5 = assign100340_e152434_d_n5;
        locals.var_w_depa_nqs_dn6 = assign100340_e152434_d_n6;
        locals.var_w_depa_nqs_dn7 = assign100340_e152434_d_n7;
        locals.var_w_depa_nqs_dn8 = assign100340_e152434_d_n8;
        locals.var_w_depa_nqs_dn9 = assign100340_e152434_d_n9;
        locals.var_w_depa_nqs_dn10 = assign100340_e152434_d_n10;
        locals.var_w_depa_nqs_dn11 = assign100340_e152434_d_n11;
        locals.var_w_depa_nqs_dn14 = assign100340_e152434_d_n14;
        locals.var_w_depa_nqs_dn18 = assign100340_e152434_d_n18;
        locals.var_w_depa_nqs_rv = 0.0;

        let (assign100350_e152445,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100350_e152438: f64 = (locals.var_ndi_i * p.p13);
        let assign100350_e152440: f64 = (assign100350_e152438 * 1.6021918e-19);
        let assign100350_e152441: f64 = (-assign100350_e152440);
        let assign100350_e152443: f64 = (assign100350_e152441 * p.p545);
        (assign100350_e152443,)
    } else {
        (locals.var_q_n0,)
    }
};
        locals.var_q_n0 = assign100350_e152445;
        locals.var_q_n0_rv = 0.0;

        let (assign100360_e152463, assign100360_e152463_d_n0, assign100360_e152463_d_n2, assign100360_e152463_d_n4, assign100360_e152463_d_n5, assign100360_e152463_d_n6, assign100360_e152463_d_n7, assign100360_e152463_d_n8, assign100360_e152463_d_n9, assign100360_e152463_d_n10, assign100360_e152463_d_n11, assign100360_e152463_d_n14, assign100360_e152463_d_n16, assign100360_e152463_d_n18,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100360_e152449: f64 = (locals.var_la * locals.var_q_pexa_nqs);
        let assign100360_e152451: f64 = (-p.p545);
        let assign100360_e152453: f64 = (assign100360_e152451 / locals.var_la);
        let assign100360_e152454: f64 = (assign100360_e152453).exp();
        let assign100360_e152456: f64 = (-locals.var_w_depa_nqs);
        let assign100360_e152458: f64 = (assign100360_e152456 / locals.var_la);
        let assign100360_e152459: f64 = (assign100360_e152458).exp();
        let assign100360_e152460: f64 = (assign100360_e152454 - assign100360_e152459);
        let assign100360_e152461: f64 = (assign100360_e152449 * assign100360_e152460);
        (assign100360_e152461, ((((locals.var_la_dn0 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn0)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn0) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn0) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn0)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn2 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn2)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn2) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn2) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn2)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn4 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn4)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn4) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn4) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn4)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn5 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn5)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn5) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn5) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn5)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn6 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn6)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn6) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn6) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn6)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn7 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn7)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn7) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn7) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn7)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn8 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn8)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn8) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn8) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn8)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn9 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn9)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn9) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn9) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn9)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn10 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn10)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn10) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn10) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn10)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn11 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn11)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn11) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn11) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn11)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn14 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn14)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn14) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn14) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn14)) / (locals.var_la * locals.var_la)))))), ((locals.var_la * locals.var_q_pexa_nqs_dn16) * assign100360_e152460), (assign100360_e152449 * (-(assign100360_e152459 * ((-locals.var_w_depa_nqs_dn18) / locals.var_la)))),)
    } else {
        (locals.var_q_nexa_nqs, locals.var_q_nexa_nqs_dn0, locals.var_q_nexa_nqs_dn2, locals.var_q_nexa_nqs_dn4, locals.var_q_nexa_nqs_dn5, locals.var_q_nexa_nqs_dn6, locals.var_q_nexa_nqs_dn7, locals.var_q_nexa_nqs_dn8, locals.var_q_nexa_nqs_dn9, locals.var_q_nexa_nqs_dn10, locals.var_q_nexa_nqs_dn11, locals.var_q_nexa_nqs_dn14, locals.var_q_nexa_nqs_dn16, locals.var_q_nexa_nqs_dn18,)
    }
};
        locals.var_q_nexa_nqs = assign100360_e152463;
        locals.var_q_nexa_nqs_dn0 = assign100360_e152463_d_n0;
        locals.var_q_nexa_nqs_dn2 = assign100360_e152463_d_n2;
        locals.var_q_nexa_nqs_dn4 = assign100360_e152463_d_n4;
        locals.var_q_nexa_nqs_dn5 = assign100360_e152463_d_n5;
        locals.var_q_nexa_nqs_dn6 = assign100360_e152463_d_n6;
        locals.var_q_nexa_nqs_dn7 = assign100360_e152463_d_n7;
        locals.var_q_nexa_nqs_dn8 = assign100360_e152463_d_n8;
        locals.var_q_nexa_nqs_dn9 = assign100360_e152463_d_n9;
        locals.var_q_nexa_nqs_dn10 = assign100360_e152463_d_n10;
        locals.var_q_nexa_nqs_dn11 = assign100360_e152463_d_n11;
        locals.var_q_nexa_nqs_dn14 = assign100360_e152463_d_n14;
        locals.var_q_nexa_nqs_dn16 = assign100360_e152463_d_n16;
        locals.var_q_nexa_nqs_dn18 = assign100360_e152463_d_n18;
        locals.var_q_nexa_nqs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_386(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign100370_e152479, assign100370_e152479_d_n0, assign100370_e152479_d_n2, assign100370_e152479_d_n4, assign100370_e152479_d_n5, assign100370_e152479_d_n6, assign100370_e152479_d_n7, assign100370_e152479_d_n8, assign100370_e152479_d_n9, assign100370_e152479_d_n10, assign100370_e152479_d_n11, assign100370_e152479_d_n14, assign100370_e152479_d_n17, assign100370_e152479_d_n18,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100370_e152467: f64 = (locals.var_la * locals.var_q_pexk_nqs);
        let assign100370_e152470: f64 = (p.p545 - locals.var_w_depa_nqs);
        let assign100370_e152471: f64 = (-assign100370_e152470);
        let assign100370_e152473: f64 = (assign100370_e152471 / locals.var_la);
        let assign100370_e152474: f64 = (assign100370_e152473).exp();
        let assign100370_e152476: f64 = (assign100370_e152474 - 1.0);
        let assign100370_e152477: f64 = (assign100370_e152467 * assign100370_e152476);
        (assign100370_e152477, ((((locals.var_la_dn0 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn0)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn0)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn0)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn2 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn2)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn2)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn2)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn4 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn4)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn4)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn4)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn5 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn5)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn5)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn5)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn6 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn6)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn6)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn6)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn7 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn7)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn7)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn7)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn8 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn8)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn8)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn8)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn9 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn9)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn9)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn9)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn10 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn10)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn10)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn10)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn11 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn11)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn11)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn11)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn14 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn14)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn14)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn14)) / (locals.var_la * locals.var_la))))), ((locals.var_la * locals.var_q_pexk_nqs_dn17) * assign100370_e152476), (assign100370_e152467 * (assign100370_e152474 * ((-(-locals.var_w_depa_nqs_dn18)) / locals.var_la))),)
    } else {
        (locals.var_q_nexk_nqs, locals.var_q_nexk_nqs_dn0, locals.var_q_nexk_nqs_dn2, locals.var_q_nexk_nqs_dn4, locals.var_q_nexk_nqs_dn5, locals.var_q_nexk_nqs_dn6, locals.var_q_nexk_nqs_dn7, locals.var_q_nexk_nqs_dn8, locals.var_q_nexk_nqs_dn9, locals.var_q_nexk_nqs_dn10, locals.var_q_nexk_nqs_dn11, locals.var_q_nexk_nqs_dn14, locals.var_q_nexk_nqs_dn17, locals.var_q_nexk_nqs_dn18,)
    }
};
        locals.var_q_nexk_nqs = assign100370_e152479;
        locals.var_q_nexk_nqs_dn0 = assign100370_e152479_d_n0;
        locals.var_q_nexk_nqs_dn2 = assign100370_e152479_d_n2;
        locals.var_q_nexk_nqs_dn4 = assign100370_e152479_d_n4;
        locals.var_q_nexk_nqs_dn5 = assign100370_e152479_d_n5;
        locals.var_q_nexk_nqs_dn6 = assign100370_e152479_d_n6;
        locals.var_q_nexk_nqs_dn7 = assign100370_e152479_d_n7;
        locals.var_q_nexk_nqs_dn8 = assign100370_e152479_d_n8;
        locals.var_q_nexk_nqs_dn9 = assign100370_e152479_d_n9;
        locals.var_q_nexk_nqs_dn10 = assign100370_e152479_d_n10;
        locals.var_q_nexk_nqs_dn11 = assign100370_e152479_d_n11;
        locals.var_q_nexk_nqs_dn14 = assign100370_e152479_d_n14;
        locals.var_q_nexk_nqs_dn17 = assign100370_e152479_d_n17;
        locals.var_q_nexk_nqs_dn18 = assign100370_e152479_d_n18;
        locals.var_q_nexk_nqs_rv = 0.0;

        let (assign100380_e152488, assign100380_e152488_d_n0, assign100380_e152488_d_n2, assign100380_e152488_d_n4, assign100380_e152488_d_n5, assign100380_e152488_d_n6, assign100380_e152488_d_n7, assign100380_e152488_d_n8, assign100380_e152488_d_n9, assign100380_e152488_d_n10, assign100380_e152488_d_n11, assign100380_e152488_d_n14, assign100380_e152488_d_n16, assign100380_e152488_d_n17, assign100380_e152488_d_n18,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100380_e152483: f64 = (locals.var_q_n0 + locals.var_q_nexa_nqs);
        let assign100380_e152485: f64 = (assign100380_e152483 + locals.var_q_nexk_nqs);
        let assign100380_e152486: f64 = (-assign100380_e152485);
        (assign100380_e152486, (-(locals.var_q_nexa_nqs_dn0 + locals.var_q_nexk_nqs_dn0)), (-(locals.var_q_nexa_nqs_dn2 + locals.var_q_nexk_nqs_dn2)), (-(locals.var_q_nexa_nqs_dn4 + locals.var_q_nexk_nqs_dn4)), (-(locals.var_q_nexa_nqs_dn5 + locals.var_q_nexk_nqs_dn5)), (-(locals.var_q_nexa_nqs_dn6 + locals.var_q_nexk_nqs_dn6)), (-(locals.var_q_nexa_nqs_dn7 + locals.var_q_nexk_nqs_dn7)), (-(locals.var_q_nexa_nqs_dn8 + locals.var_q_nexk_nqs_dn8)), (-(locals.var_q_nexa_nqs_dn9 + locals.var_q_nexk_nqs_dn9)), (-(locals.var_q_nexa_nqs_dn10 + locals.var_q_nexk_nqs_dn10)), (-(locals.var_q_nexa_nqs_dn11 + locals.var_q_nexk_nqs_dn11)), (-(locals.var_q_nexa_nqs_dn14 + locals.var_q_nexk_nqs_dn14)), (-locals.var_q_nexa_nqs_dn16), (-locals.var_q_nexk_nqs_dn17), (-(locals.var_q_nexa_nqs_dn18 + locals.var_q_nexk_nqs_dn18)),)
    } else {
        (locals.var_qrr, locals.var_qrr_dn0, locals.var_qrr_dn2, locals.var_qrr_dn4, locals.var_qrr_dn5, locals.var_qrr_dn6, locals.var_qrr_dn7, locals.var_qrr_dn8, locals.var_qrr_dn9, locals.var_qrr_dn10, locals.var_qrr_dn11, locals.var_qrr_dn14, locals.var_qrr_dn16, locals.var_qrr_dn17, locals.var_qrr_dn18,)
    }
};
        locals.var_qrr = assign100380_e152488;
        locals.var_qrr_dn0 = assign100380_e152488_d_n0;
        locals.var_qrr_dn2 = assign100380_e152488_d_n2;
        locals.var_qrr_dn4 = assign100380_e152488_d_n4;
        locals.var_qrr_dn5 = assign100380_e152488_d_n5;
        locals.var_qrr_dn6 = assign100380_e152488_d_n6;
        locals.var_qrr_dn7 = assign100380_e152488_d_n7;
        locals.var_qrr_dn8 = assign100380_e152488_d_n8;
        locals.var_qrr_dn9 = assign100380_e152488_d_n9;
        locals.var_qrr_dn10 = assign100380_e152488_d_n10;
        locals.var_qrr_dn11 = assign100380_e152488_d_n11;
        locals.var_qrr_dn14 = assign100380_e152488_d_n14;
        locals.var_qrr_dn16 = assign100380_e152488_d_n16;
        locals.var_qrr_dn17 = assign100380_e152488_d_n17;
        locals.var_qrr_dn18 = assign100380_e152488_d_n18;
        locals.var_qrr_rv = 0.0;

        let (assign100390_e152496, assign100390_e152496_d_n0, assign100390_e152496_d_n2, assign100390_e152496_d_n4, assign100390_e152496_d_n5, assign100390_e152496_d_n6, assign100390_e152496_d_n7, assign100390_e152496_d_n8, assign100390_e152496_d_n9, assign100390_e152496_d_n10, assign100390_e152496_d_n11, assign100390_e152496_d_n14, assign100390_e152496_d_n16, assign100390_e152496_d_n17, assign100390_e152496_d_n18,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100390_e152493: f64 = (locals.var_mfactor * locals.var_qrr);
        let assign100390_e152494: f64 = (locals.var_qbd + assign100390_e152493);
        (assign100390_e152494, (locals.var_qbd_dn0 + (locals.var_mfactor * locals.var_qrr_dn0)), (locals.var_qbd_dn2 + (locals.var_mfactor * locals.var_qrr_dn2)), (locals.var_qbd_dn4 + (locals.var_mfactor * locals.var_qrr_dn4)), (locals.var_qbd_dn5 + (locals.var_mfactor * locals.var_qrr_dn5)), (locals.var_qbd_dn6 + (locals.var_mfactor * locals.var_qrr_dn6)), (locals.var_qbd_dn7 + (locals.var_mfactor * locals.var_qrr_dn7)), (locals.var_qbd_dn8 + (locals.var_mfactor * locals.var_qrr_dn8)), (locals.var_qbd_dn9 + (locals.var_mfactor * locals.var_qrr_dn9)), (locals.var_qbd_dn10 + (locals.var_mfactor * locals.var_qrr_dn10)), (locals.var_qbd_dn11 + (locals.var_mfactor * locals.var_qrr_dn11)), (locals.var_qbd_dn14 + (locals.var_mfactor * locals.var_qrr_dn14)), (locals.var_qbd_dn16 + (locals.var_mfactor * locals.var_qrr_dn16)), (locals.var_qbd_dn17 + (locals.var_mfactor * locals.var_qrr_dn17)), (locals.var_qbd_dn18 + (locals.var_mfactor * locals.var_qrr_dn18)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn14, locals.var_qbd_dn16, locals.var_qbd_dn17, locals.var_qbd_dn18,)
    }
};
        locals.var_qbd = assign100390_e152496;
        locals.var_qbd_dn0 = assign100390_e152496_d_n0;
        locals.var_qbd_dn2 = assign100390_e152496_d_n2;
        locals.var_qbd_dn4 = assign100390_e152496_d_n4;
        locals.var_qbd_dn5 = assign100390_e152496_d_n5;
        locals.var_qbd_dn6 = assign100390_e152496_d_n6;
        locals.var_qbd_dn7 = assign100390_e152496_d_n7;
        locals.var_qbd_dn8 = assign100390_e152496_d_n8;
        locals.var_qbd_dn9 = assign100390_e152496_d_n9;
        locals.var_qbd_dn10 = assign100390_e152496_d_n10;
        locals.var_qbd_dn11 = assign100390_e152496_d_n11;
        locals.var_qbd_dn14 = assign100390_e152496_d_n14;
        locals.var_qbd_dn16 = assign100390_e152496_d_n16;
        locals.var_qbd_dn17 = assign100390_e152496_d_n17;
        locals.var_qbd_dn18 = assign100390_e152496_d_n18;
        locals.var_qbd_rv = 0.0;

        let assign100400_e152503: f64 = if ((p.p539 > 0.0) && (p.p543 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2313 = assign100400_e152503;
        locals.var_guard2313_rv = 0.0;

        let assign100410_e152510: f64 = if ((p.p539 > 0.0) && (p.p546 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2314 = assign100410_e152510;
        locals.var_guard2314_rv = 0.0;

        let assign100420_e152513: f64 = if p.p46 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2315 = assign100420_e152513;
        locals.var_guard2315_rv = 0.0;

        let assign100430_e152520: f64 = if ((locals.var_uc_sub1snp > 0.0) && (locals.var_uc_vmax > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2316 = assign100430_e152520;
        locals.var_guard2316_rv = 0.0;

        let (assign100440_e152528, assign100440_e152528_d_n0, assign100440_e152528_d_n2, assign100440_e152528_d_n4, assign100440_e152528_d_n5, assign100440_e152528_d_n6, assign100440_e152528_d_n7, assign100440_e152528_d_n8, assign100440_e152528_d_n9, assign100440_e152528_d_n10, assign100440_e152528_d_n11, assign100440_e152528_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100440_e152526: f64 = (locals.var_vg2const_1 * locals.var_vgp);
        (assign100440_e152526, ((locals.var_vg2const_1_dn0 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn0)), ((locals.var_vg2const_1_dn2 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn2)), ((locals.var_vg2const_1_dn4 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn4)), ((locals.var_vg2const_1_dn5 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn5)), ((locals.var_vg2const_1_dn6 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn6)), ((locals.var_vg2const_1_dn7 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn7)), ((locals.var_vg2const_1_dn8 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn8)), ((locals.var_vg2const_1_dn9 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn9)), ((locals.var_vg2const_1_dn10 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn10)), ((locals.var_vg2const_1_dn11 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn11)), ((locals.var_vg2const_1_dn14 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign100440_e152528;
        locals.var_t1_dn0 = assign100440_e152528_d_n0;
        locals.var_t1_dn2 = assign100440_e152528_d_n2;
        locals.var_t1_dn4 = assign100440_e152528_d_n4;
        locals.var_t1_dn5 = assign100440_e152528_d_n5;
        locals.var_t1_dn6 = assign100440_e152528_d_n6;
        locals.var_t1_dn7 = assign100440_e152528_d_n7;
        locals.var_t1_dn8 = assign100440_e152528_d_n8;
        locals.var_t1_dn9 = assign100440_e152528_d_n9;
        locals.var_t1_dn10 = assign100440_e152528_d_n10;
        locals.var_t1_dn11 = assign100440_e152528_d_n11;
        locals.var_t1_dn14 = assign100440_e152528_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign100450_e152538, assign100450_e152538_d_n0, assign100450_e152538_d_n2, assign100450_e152538_d_n4, assign100450_e152538_d_n5, assign100450_e152538_d_n6, assign100450_e152538_d_n7, assign100450_e152538_d_n8, assign100450_e152538_d_n9, assign100450_e152538_d_n10, assign100450_e152538_d_n11, assign100450_e152538_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100450_e152535: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100450_e152536: f64 = (locals.var_qnsub_esi / assign100450_e152535);
        (assign100450_e152536, (locals.var_qnsub_esi_dn0 / assign100450_e152535), (locals.var_qnsub_esi_dn2 / assign100450_e152535), (locals.var_qnsub_esi_dn4 / assign100450_e152535), (locals.var_qnsub_esi_dn5 / assign100450_e152535), (locals.var_qnsub_esi_dn6 / assign100450_e152535), (locals.var_qnsub_esi_dn7 / assign100450_e152535), (locals.var_qnsub_esi_dn8 / assign100450_e152535), (locals.var_qnsub_esi_dn9 / assign100450_e152535), (locals.var_qnsub_esi_dn10 / assign100450_e152535), (locals.var_qnsub_esi_dn11 / assign100450_e152535), (locals.var_qnsub_esi_dn14 / assign100450_e152535),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign100450_e152538;
        locals.var_t3_dn0 = assign100450_e152538_d_n0;
        locals.var_t3_dn2 = assign100450_e152538_d_n2;
        locals.var_t3_dn4 = assign100450_e152538_d_n4;
        locals.var_t3_dn5 = assign100450_e152538_d_n5;
        locals.var_t3_dn6 = assign100450_e152538_d_n6;
        locals.var_t3_dn7 = assign100450_e152538_d_n7;
        locals.var_t3_dn8 = assign100450_e152538_d_n8;
        locals.var_t3_dn9 = assign100450_e152538_d_n9;
        locals.var_t3_dn10 = assign100450_e152538_d_n10;
        locals.var_t3_dn11 = assign100450_e152538_d_n11;
        locals.var_t3_dn14 = assign100450_e152538_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign100460_e152550, assign100460_e152550_d_n0, assign100460_e152550_d_n2, assign100460_e152550_d_n4, assign100460_e152550_d_n5, assign100460_e152550_d_n6, assign100460_e152550_d_n7, assign100460_e152550_d_n8, assign100460_e152550_d_n9, assign100460_e152550_d_n10, assign100460_e152550_d_n11, assign100460_e152550_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100460_e152544: f64 = (2.0 / locals.var_qnsub_esi);
        let assign100460_e152547: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100460_e152548: f64 = (assign100460_e152544 * assign100460_e152547);
        (assign100460_e152548, ((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn4) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn5) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn8) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn9) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn11) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn14) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign100460_e152550;
        locals.var_t4_dn0 = assign100460_e152550_d_n0;
        locals.var_t4_dn2 = assign100460_e152550_d_n2;
        locals.var_t4_dn4 = assign100460_e152550_d_n4;
        locals.var_t4_dn5 = assign100460_e152550_d_n5;
        locals.var_t4_dn6 = assign100460_e152550_d_n6;
        locals.var_t4_dn7 = assign100460_e152550_d_n7;
        locals.var_t4_dn8 = assign100460_e152550_d_n8;
        locals.var_t4_dn9 = assign100460_e152550_d_n9;
        locals.var_t4_dn10 = assign100460_e152550_d_n10;
        locals.var_t4_dn11 = assign100460_e152550_d_n11;
        locals.var_t4_dn14 = assign100460_e152550_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign100470_e152562, assign100470_e152562_d_n0, assign100470_e152562_d_n2, assign100470_e152562_d_n4, assign100470_e152562_d_n5, assign100470_e152562_d_n6, assign100470_e152562_d_n7, assign100470_e152562_d_n8, assign100470_e152562_d_n9, assign100470_e152562_d_n10, assign100470_e152562_d_n11, assign100470_e152562_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100470_e152556: f64 = (locals.var_t1 - locals.var_beta_inv);
        let assign100470_e152559: f64 = (locals.var_xvbs_1 * locals.var_vbsz__blk442);
        let assign100470_e152560: f64 = (assign100470_e152556 - assign100470_e152559);
        (assign100470_e152560, ((locals.var_t1_dn0 - locals.var_beta_inv_dn0) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn0)), ((locals.var_t1_dn2 - locals.var_beta_inv_dn2) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn2)), ((locals.var_t1_dn4 - locals.var_beta_inv_dn4) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn4)), ((locals.var_t1_dn5 - locals.var_beta_inv_dn5) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn5)), ((locals.var_t1_dn6 - locals.var_beta_inv_dn6) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn6)), ((locals.var_t1_dn7 - locals.var_beta_inv_dn7) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn7)), ((locals.var_t1_dn8 - locals.var_beta_inv_dn8) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn8)), ((locals.var_t1_dn9 - locals.var_beta_inv_dn9) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn9)), ((locals.var_t1_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn10)), ((locals.var_t1_dn11 - locals.var_beta_inv_dn11) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn11)), ((locals.var_t1_dn14 - locals.var_beta_inv_dn14) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign100470_e152562;
        locals.var_t5_dn0 = assign100470_e152562_d_n0;
        locals.var_t5_dn2 = assign100470_e152562_d_n2;
        locals.var_t5_dn4 = assign100470_e152562_d_n4;
        locals.var_t5_dn5 = assign100470_e152562_d_n5;
        locals.var_t5_dn6 = assign100470_e152562_d_n6;
        locals.var_t5_dn7 = assign100470_e152562_d_n7;
        locals.var_t5_dn8 = assign100470_e152562_d_n8;
        locals.var_t5_dn9 = assign100470_e152562_d_n9;
        locals.var_t5_dn10 = assign100470_e152562_d_n10;
        locals.var_t5_dn11 = assign100470_e152562_d_n11;
        locals.var_t5_dn14 = assign100470_e152562_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign100480_e152572, assign100480_e152572_d_n0, assign100480_e152572_d_n2, assign100480_e152572_d_n4, assign100480_e152572_d_n5, assign100480_e152572_d_n6, assign100480_e152572_d_n7, assign100480_e152572_d_n8, assign100480_e152572_d_n9, assign100480_e152572_d_n10, assign100480_e152572_d_n11, assign100480_e152572_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100480_e152569: f64 = (locals.var_t4 * locals.var_t5);
        let assign100480_e152570: f64 = (1.0 + assign100480_e152569);
        (assign100480_e152570, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn14 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100480_e152572;
        locals.var_t6_dn0 = assign100480_e152572_d_n0;
        locals.var_t6_dn2 = assign100480_e152572_d_n2;
        locals.var_t6_dn4 = assign100480_e152572_d_n4;
        locals.var_t6_dn5 = assign100480_e152572_d_n5;
        locals.var_t6_dn6 = assign100480_e152572_d_n6;
        locals.var_t6_dn7 = assign100480_e152572_d_n7;
        locals.var_t6_dn8 = assign100480_e152572_d_n8;
        locals.var_t6_dn9 = assign100480_e152572_d_n9;
        locals.var_t6_dn10 = assign100480_e152572_d_n10;
        locals.var_t6_dn11 = assign100480_e152572_d_n11;
        locals.var_t6_dn14 = assign100480_e152572_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign100490_e152582, assign100490_e152582_d_n0, assign100490_e152582_d_n2, assign100490_e152582_d_n4, assign100490_e152582_d_n5, assign100490_e152582_d_n6, assign100490_e152582_d_n7, assign100490_e152582_d_n8, assign100490_e152582_d_n9, assign100490_e152582_d_n10, assign100490_e152582_d_n11, assign100490_e152582_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100490_e152579: f64 = (1.0 + locals.var_t4);
        let assign100490_e152580: f64 = (2.0 * assign100490_e152579);
        (assign100490_e152580, (2.0 * locals.var_t4_dn0), (2.0 * locals.var_t4_dn2), (2.0 * locals.var_t4_dn4), (2.0 * locals.var_t4_dn5), (2.0 * locals.var_t4_dn6), (2.0 * locals.var_t4_dn7), (2.0 * locals.var_t4_dn8), (2.0 * locals.var_t4_dn9), (2.0 * locals.var_t4_dn10), (2.0 * locals.var_t4_dn11), (2.0 * locals.var_t4_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign100490_e152582;
        locals.var_t7_dn0 = assign100490_e152582_d_n0;
        locals.var_t7_dn2 = assign100490_e152582_d_n2;
        locals.var_t7_dn4 = assign100490_e152582_d_n4;
        locals.var_t7_dn5 = assign100490_e152582_d_n5;
        locals.var_t7_dn6 = assign100490_e152582_d_n6;
        locals.var_t7_dn7 = assign100490_e152582_d_n7;
        locals.var_t7_dn8 = assign100490_e152582_d_n8;
        locals.var_t7_dn9 = assign100490_e152582_d_n9;
        locals.var_t7_dn10 = assign100490_e152582_d_n10;
        locals.var_t7_dn11 = assign100490_e152582_d_n11;
        locals.var_t7_dn14 = assign100490_e152582_d_n14;
        locals.var_t7_rv = 0.0;

        let assign100500_e152586: f64 = locals.var_t7;
        let assign100500_e152591: f64 = if ((locals.var_t6 < assign100500_e152586) && (locals.var_t7 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2317 = assign100500_e152591;
        locals.var_guard2317_rv = 0.0;

        let (assign100510_e152603, assign100510_e152603_d_n0, assign100510_e152603_d_n2, assign100510_e152603_d_n4, assign100510_e152603_d_n5, assign100510_e152603_d_n6, assign100510_e152603_d_n7, assign100510_e152603_d_n8, assign100510_e152603_d_n9, assign100510_e152603_d_n10, assign100510_e152603_d_n11, assign100510_e152603_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100510_e152599: f64 = locals.var_t7;
        let assign100510_e152601: f64 = (assign100510_e152599 - locals.var_t6);
        (assign100510_e152601, (locals.var_t7_dn0 - locals.var_t6_dn0), (locals.var_t7_dn2 - locals.var_t6_dn2), (locals.var_t7_dn4 - locals.var_t6_dn4), (locals.var_t7_dn5 - locals.var_t6_dn5), (locals.var_t7_dn6 - locals.var_t6_dn6), (locals.var_t7_dn7 - locals.var_t6_dn7), (locals.var_t7_dn8 - locals.var_t6_dn8), (locals.var_t7_dn9 - locals.var_t6_dn9), (locals.var_t7_dn10 - locals.var_t6_dn10), (locals.var_t7_dn11 - locals.var_t6_dn11), (locals.var_t7_dn14 - locals.var_t6_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign100510_e152603;
        locals.var_tmf1_dn0 = assign100510_e152603_d_n0;
        locals.var_tmf1_dn2 = assign100510_e152603_d_n2;
        locals.var_tmf1_dn4 = assign100510_e152603_d_n4;
        locals.var_tmf1_dn5 = assign100510_e152603_d_n5;
        locals.var_tmf1_dn6 = assign100510_e152603_d_n6;
        locals.var_tmf1_dn7 = assign100510_e152603_d_n7;
        locals.var_tmf1_dn8 = assign100510_e152603_d_n8;
        locals.var_tmf1_dn9 = assign100510_e152603_d_n9;
        locals.var_tmf1_dn10 = assign100510_e152603_d_n10;
        locals.var_tmf1_dn11 = assign100510_e152603_d_n11;
        locals.var_tmf1_dn14 = assign100510_e152603_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign100520_e152613, assign100520_e152613_d_n0, assign100520_e152613_d_n2, assign100520_e152613_d_n4, assign100520_e152613_d_n5, assign100520_e152613_d_n6, assign100520_e152613_d_n7, assign100520_e152613_d_n8, assign100520_e152613_d_n9, assign100520_e152613_d_n10, assign100520_e152613_d_n11, assign100520_e152613_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100520_e152611: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign100520_e152611, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign100520_e152613;
        locals.var_x2_dn0 = assign100520_e152613_d_n0;
        locals.var_x2_dn2 = assign100520_e152613_d_n2;
        locals.var_x2_dn4 = assign100520_e152613_d_n4;
        locals.var_x2_dn5 = assign100520_e152613_d_n5;
        locals.var_x2_dn6 = assign100520_e152613_d_n6;
        locals.var_x2_dn7 = assign100520_e152613_d_n7;
        locals.var_x2_dn8 = assign100520_e152613_d_n8;
        locals.var_x2_dn9 = assign100520_e152613_d_n9;
        locals.var_x2_dn10 = assign100520_e152613_d_n10;
        locals.var_x2_dn11 = assign100520_e152613_d_n11;
        locals.var_x2_dn14 = assign100520_e152613_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign100530_e152623, assign100530_e152623_d_n0, assign100530_e152623_d_n2, assign100530_e152623_d_n4, assign100530_e152623_d_n5, assign100530_e152623_d_n6, assign100530_e152623_d_n7, assign100530_e152623_d_n8, assign100530_e152623_d_n9, assign100530_e152623_d_n10, assign100530_e152623_d_n11, assign100530_e152623_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100530_e152621: f64 = (locals.var_t7 * locals.var_t7);
        (assign100530_e152621, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)), ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign100530_e152623;
        locals.var_xmax2_dn0 = assign100530_e152623_d_n0;
        locals.var_xmax2_dn2 = assign100530_e152623_d_n2;
        locals.var_xmax2_dn4 = assign100530_e152623_d_n4;
        locals.var_xmax2_dn5 = assign100530_e152623_d_n5;
        locals.var_xmax2_dn6 = assign100530_e152623_d_n6;
        locals.var_xmax2_dn7 = assign100530_e152623_d_n7;
        locals.var_xmax2_dn8 = assign100530_e152623_d_n8;
        locals.var_xmax2_dn9 = assign100530_e152623_d_n9;
        locals.var_xmax2_dn10 = assign100530_e152623_d_n10;
        locals.var_xmax2_dn11 = assign100530_e152623_d_n11;
        locals.var_xmax2_dn14 = assign100530_e152623_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign100540_e152631, assign100540_e152631_d_n0, assign100540_e152631_d_n2, assign100540_e152631_d_n4, assign100540_e152631_d_n5, assign100540_e152631_d_n6, assign100540_e152631_d_n7, assign100540_e152631_d_n8, assign100540_e152631_d_n9, assign100540_e152631_d_n10, assign100540_e152631_d_n11, assign100540_e152631_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100540_e152631;
        locals.var_xp_dn0 = assign100540_e152631_d_n0;
        locals.var_xp_dn2 = assign100540_e152631_d_n2;
        locals.var_xp_dn4 = assign100540_e152631_d_n4;
        locals.var_xp_dn5 = assign100540_e152631_d_n5;
        locals.var_xp_dn6 = assign100540_e152631_d_n6;
        locals.var_xp_dn7 = assign100540_e152631_d_n7;
        locals.var_xp_dn8 = assign100540_e152631_d_n8;
        locals.var_xp_dn9 = assign100540_e152631_d_n9;
        locals.var_xp_dn10 = assign100540_e152631_d_n10;
        locals.var_xp_dn11 = assign100540_e152631_d_n11;
        locals.var_xp_dn14 = assign100540_e152631_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign100550_e152639, assign100550_e152639_d_n0, assign100550_e152639_d_n2, assign100550_e152639_d_n4, assign100550_e152639_d_n5, assign100550_e152639_d_n6, assign100550_e152639_d_n7, assign100550_e152639_d_n8, assign100550_e152639_d_n9, assign100550_e152639_d_n10, assign100550_e152639_d_n11, assign100550_e152639_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100550_e152639;
        locals.var_xmp_dn0 = assign100550_e152639_d_n0;
        locals.var_xmp_dn2 = assign100550_e152639_d_n2;
        locals.var_xmp_dn4 = assign100550_e152639_d_n4;
        locals.var_xmp_dn5 = assign100550_e152639_d_n5;
        locals.var_xmp_dn6 = assign100550_e152639_d_n6;
        locals.var_xmp_dn7 = assign100550_e152639_d_n7;
        locals.var_xmp_dn8 = assign100550_e152639_d_n8;
        locals.var_xmp_dn9 = assign100550_e152639_d_n9;
        locals.var_xmp_dn10 = assign100550_e152639_d_n10;
        locals.var_xmp_dn11 = assign100550_e152639_d_n11;
        locals.var_xmp_dn14 = assign100550_e152639_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign100560_e152647,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100560_e152647;
        locals.var_m0_rv = 0.0;

        let (assign100570_e152655,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100570_e152655;
        locals.var_mm_rv = 0.0;

        let (assign100580_e152663, assign100580_e152663_d_n0, assign100580_e152663_d_n2, assign100580_e152663_d_n4, assign100580_e152663_d_n5, assign100580_e152663_d_n6, assign100580_e152663_d_n7, assign100580_e152663_d_n8, assign100580_e152663_d_n9, assign100580_e152663_d_n10, assign100580_e152663_d_n11, assign100580_e152663_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign100580_e152663;
        locals.var_arg_dn0 = assign100580_e152663_d_n0;
        locals.var_arg_dn2 = assign100580_e152663_d_n2;
        locals.var_arg_dn4 = assign100580_e152663_d_n4;
        locals.var_arg_dn5 = assign100580_e152663_d_n5;
        locals.var_arg_dn6 = assign100580_e152663_d_n6;
        locals.var_arg_dn7 = assign100580_e152663_d_n7;
        locals.var_arg_dn8 = assign100580_e152663_d_n8;
        locals.var_arg_dn9 = assign100580_e152663_d_n9;
        locals.var_arg_dn10 = assign100580_e152663_d_n10;
        locals.var_arg_dn11 = assign100580_e152663_d_n11;
        locals.var_arg_dn14 = assign100580_e152663_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign100590_e152671, assign100590_e152671_d_n0, assign100590_e152671_d_n2, assign100590_e152671_d_n4, assign100590_e152671_d_n5, assign100590_e152671_d_n6, assign100590_e152671_d_n7, assign100590_e152671_d_n8, assign100590_e152671_d_n9, assign100590_e152671_d_n10, assign100590_e152671_d_n11, assign100590_e152671_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100590_e152671;
        locals.var_dnm_dn0 = assign100590_e152671_d_n0;
        locals.var_dnm_dn2 = assign100590_e152671_d_n2;
        locals.var_dnm_dn4 = assign100590_e152671_d_n4;
        locals.var_dnm_dn5 = assign100590_e152671_d_n5;
        locals.var_dnm_dn6 = assign100590_e152671_d_n6;
        locals.var_dnm_dn7 = assign100590_e152671_d_n7;
        locals.var_dnm_dn8 = assign100590_e152671_d_n8;
        locals.var_dnm_dn9 = assign100590_e152671_d_n9;
        locals.var_dnm_dn10 = assign100590_e152671_d_n10;
        locals.var_dnm_dn11 = assign100590_e152671_d_n11;
        locals.var_dnm_dn14 = assign100590_e152671_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign100600_e152681, assign100600_e152681_d_n0, assign100600_e152681_d_n2, assign100600_e152681_d_n4, assign100600_e152681_d_n5, assign100600_e152681_d_n6, assign100600_e152681_d_n7, assign100600_e152681_d_n8, assign100600_e152681_d_n9, assign100600_e152681_d_n10, assign100600_e152681_d_n11, assign100600_e152681_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100600_e152679: f64 = (locals.var_xp * locals.var_x2);
        (assign100600_e152679, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100600_e152681;
        locals.var_xp_dn0 = assign100600_e152681_d_n0;
        locals.var_xp_dn2 = assign100600_e152681_d_n2;
        locals.var_xp_dn4 = assign100600_e152681_d_n4;
        locals.var_xp_dn5 = assign100600_e152681_d_n5;
        locals.var_xp_dn6 = assign100600_e152681_d_n6;
        locals.var_xp_dn7 = assign100600_e152681_d_n7;
        locals.var_xp_dn8 = assign100600_e152681_d_n8;
        locals.var_xp_dn9 = assign100600_e152681_d_n9;
        locals.var_xp_dn10 = assign100600_e152681_d_n10;
        locals.var_xp_dn11 = assign100600_e152681_d_n11;
        locals.var_xp_dn14 = assign100600_e152681_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign100610_e152691, assign100610_e152691_d_n0, assign100610_e152691_d_n2, assign100610_e152691_d_n4, assign100610_e152691_d_n5, assign100610_e152691_d_n6, assign100610_e152691_d_n7, assign100610_e152691_d_n8, assign100610_e152691_d_n9, assign100610_e152691_d_n10, assign100610_e152691_d_n11, assign100610_e152691_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100610_e152689: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100610_e152689, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100610_e152691;
        locals.var_xmp_dn0 = assign100610_e152691_d_n0;
        locals.var_xmp_dn2 = assign100610_e152691_d_n2;
        locals.var_xmp_dn4 = assign100610_e152691_d_n4;
        locals.var_xmp_dn5 = assign100610_e152691_d_n5;
        locals.var_xmp_dn6 = assign100610_e152691_d_n6;
        locals.var_xmp_dn7 = assign100610_e152691_d_n7;
        locals.var_xmp_dn8 = assign100610_e152691_d_n8;
        locals.var_xmp_dn9 = assign100610_e152691_d_n9;
        locals.var_xmp_dn10 = assign100610_e152691_d_n10;
        locals.var_xmp_dn11 = assign100610_e152691_d_n11;
        locals.var_xmp_dn14 = assign100610_e152691_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign100620_e152701, assign100620_e152701_d_n0, assign100620_e152701_d_n2, assign100620_e152701_d_n4, assign100620_e152701_d_n5, assign100620_e152701_d_n6, assign100620_e152701_d_n7, assign100620_e152701_d_n8, assign100620_e152701_d_n9, assign100620_e152701_d_n10, assign100620_e152701_d_n11, assign100620_e152701_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100620_e152699: f64 = (locals.var_xp * locals.var_x2);
        (assign100620_e152699, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100620_e152701;
        locals.var_xp_dn0 = assign100620_e152701_d_n0;
        locals.var_xp_dn2 = assign100620_e152701_d_n2;
        locals.var_xp_dn4 = assign100620_e152701_d_n4;
        locals.var_xp_dn5 = assign100620_e152701_d_n5;
        locals.var_xp_dn6 = assign100620_e152701_d_n6;
        locals.var_xp_dn7 = assign100620_e152701_d_n7;
        locals.var_xp_dn8 = assign100620_e152701_d_n8;
        locals.var_xp_dn9 = assign100620_e152701_d_n9;
        locals.var_xp_dn10 = assign100620_e152701_d_n10;
        locals.var_xp_dn11 = assign100620_e152701_d_n11;
        locals.var_xp_dn14 = assign100620_e152701_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign100630_e152711, assign100630_e152711_d_n0, assign100630_e152711_d_n2, assign100630_e152711_d_n4, assign100630_e152711_d_n5, assign100630_e152711_d_n6, assign100630_e152711_d_n7, assign100630_e152711_d_n8, assign100630_e152711_d_n9, assign100630_e152711_d_n10, assign100630_e152711_d_n11, assign100630_e152711_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100630_e152709: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100630_e152709, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100630_e152711;
        locals.var_xmp_dn0 = assign100630_e152711_d_n0;
        locals.var_xmp_dn2 = assign100630_e152711_d_n2;
        locals.var_xmp_dn4 = assign100630_e152711_d_n4;
        locals.var_xmp_dn5 = assign100630_e152711_d_n5;
        locals.var_xmp_dn6 = assign100630_e152711_d_n6;
        locals.var_xmp_dn7 = assign100630_e152711_d_n7;
        locals.var_xmp_dn8 = assign100630_e152711_d_n8;
        locals.var_xmp_dn9 = assign100630_e152711_d_n9;
        locals.var_xmp_dn10 = assign100630_e152711_d_n10;
        locals.var_xmp_dn11 = assign100630_e152711_d_n11;
        locals.var_xmp_dn14 = assign100630_e152711_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_387(
        locals: &mut StampLocals,
    ) {
        let (assign100640_e152721, assign100640_e152721_d_n0, assign100640_e152721_d_n2, assign100640_e152721_d_n4, assign100640_e152721_d_n5, assign100640_e152721_d_n6, assign100640_e152721_d_n7, assign100640_e152721_d_n8, assign100640_e152721_d_n9, assign100640_e152721_d_n10, assign100640_e152721_d_n11, assign100640_e152721_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100640_e152719: f64 = (locals.var_xp * locals.var_x2);
        (assign100640_e152719, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100640_e152721;
        locals.var_xp_dn0 = assign100640_e152721_d_n0;
        locals.var_xp_dn2 = assign100640_e152721_d_n2;
        locals.var_xp_dn4 = assign100640_e152721_d_n4;
        locals.var_xp_dn5 = assign100640_e152721_d_n5;
        locals.var_xp_dn6 = assign100640_e152721_d_n6;
        locals.var_xp_dn7 = assign100640_e152721_d_n7;
        locals.var_xp_dn8 = assign100640_e152721_d_n8;
        locals.var_xp_dn9 = assign100640_e152721_d_n9;
        locals.var_xp_dn10 = assign100640_e152721_d_n10;
        locals.var_xp_dn11 = assign100640_e152721_d_n11;
        locals.var_xp_dn14 = assign100640_e152721_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign100650_e152731, assign100650_e152731_d_n0, assign100650_e152731_d_n2, assign100650_e152731_d_n4, assign100650_e152731_d_n5, assign100650_e152731_d_n6, assign100650_e152731_d_n7, assign100650_e152731_d_n8, assign100650_e152731_d_n9, assign100650_e152731_d_n10, assign100650_e152731_d_n11, assign100650_e152731_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100650_e152729: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100650_e152729, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100650_e152731;
        locals.var_xmp_dn0 = assign100650_e152731_d_n0;
        locals.var_xmp_dn2 = assign100650_e152731_d_n2;
        locals.var_xmp_dn4 = assign100650_e152731_d_n4;
        locals.var_xmp_dn5 = assign100650_e152731_d_n5;
        locals.var_xmp_dn6 = assign100650_e152731_d_n6;
        locals.var_xmp_dn7 = assign100650_e152731_d_n7;
        locals.var_xmp_dn8 = assign100650_e152731_d_n8;
        locals.var_xmp_dn9 = assign100650_e152731_d_n9;
        locals.var_xmp_dn10 = assign100650_e152731_d_n10;
        locals.var_xmp_dn11 = assign100650_e152731_d_n11;
        locals.var_xmp_dn14 = assign100650_e152731_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign100660_e152741, assign100660_e152741_d_n0, assign100660_e152741_d_n2, assign100660_e152741_d_n4, assign100660_e152741_d_n5, assign100660_e152741_d_n6, assign100660_e152741_d_n7, assign100660_e152741_d_n8, assign100660_e152741_d_n9, assign100660_e152741_d_n10, assign100660_e152741_d_n11, assign100660_e152741_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100660_e152739: f64 = (locals.var_xp * locals.var_x2);
        (assign100660_e152739, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100660_e152741;
        locals.var_xp_dn0 = assign100660_e152741_d_n0;
        locals.var_xp_dn2 = assign100660_e152741_d_n2;
        locals.var_xp_dn4 = assign100660_e152741_d_n4;
        locals.var_xp_dn5 = assign100660_e152741_d_n5;
        locals.var_xp_dn6 = assign100660_e152741_d_n6;
        locals.var_xp_dn7 = assign100660_e152741_d_n7;
        locals.var_xp_dn8 = assign100660_e152741_d_n8;
        locals.var_xp_dn9 = assign100660_e152741_d_n9;
        locals.var_xp_dn10 = assign100660_e152741_d_n10;
        locals.var_xp_dn11 = assign100660_e152741_d_n11;
        locals.var_xp_dn14 = assign100660_e152741_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign100670_e152751, assign100670_e152751_d_n0, assign100670_e152751_d_n2, assign100670_e152751_d_n4, assign100670_e152751_d_n5, assign100670_e152751_d_n6, assign100670_e152751_d_n7, assign100670_e152751_d_n8, assign100670_e152751_d_n9, assign100670_e152751_d_n10, assign100670_e152751_d_n11, assign100670_e152751_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100670_e152749: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100670_e152749, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100670_e152751;
        locals.var_xmp_dn0 = assign100670_e152751_d_n0;
        locals.var_xmp_dn2 = assign100670_e152751_d_n2;
        locals.var_xmp_dn4 = assign100670_e152751_d_n4;
        locals.var_xmp_dn5 = assign100670_e152751_d_n5;
        locals.var_xmp_dn6 = assign100670_e152751_d_n6;
        locals.var_xmp_dn7 = assign100670_e152751_d_n7;
        locals.var_xmp_dn8 = assign100670_e152751_d_n8;
        locals.var_xmp_dn9 = assign100670_e152751_d_n9;
        locals.var_xmp_dn10 = assign100670_e152751_d_n10;
        locals.var_xmp_dn11 = assign100670_e152751_d_n11;
        locals.var_xmp_dn14 = assign100670_e152751_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign100680_e152761, assign100680_e152761_d_n0, assign100680_e152761_d_n2, assign100680_e152761_d_n4, assign100680_e152761_d_n5, assign100680_e152761_d_n6, assign100680_e152761_d_n7, assign100680_e152761_d_n8, assign100680_e152761_d_n9, assign100680_e152761_d_n10, assign100680_e152761_d_n11, assign100680_e152761_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100680_e152759: f64 = (locals.var_xp + locals.var_xmp);
        (assign100680_e152759, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign100680_e152761;
        locals.var_arg_dn0 = assign100680_e152761_d_n0;
        locals.var_arg_dn2 = assign100680_e152761_d_n2;
        locals.var_arg_dn4 = assign100680_e152761_d_n4;
        locals.var_arg_dn5 = assign100680_e152761_d_n5;
        locals.var_arg_dn6 = assign100680_e152761_d_n6;
        locals.var_arg_dn7 = assign100680_e152761_d_n7;
        locals.var_arg_dn8 = assign100680_e152761_d_n8;
        locals.var_arg_dn9 = assign100680_e152761_d_n9;
        locals.var_arg_dn10 = assign100680_e152761_d_n10;
        locals.var_arg_dn11 = assign100680_e152761_d_n11;
        locals.var_arg_dn14 = assign100680_e152761_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign100690_e152769, assign100690_e152769_d_n0, assign100690_e152769_d_n2, assign100690_e152769_d_n4, assign100690_e152769_d_n5, assign100690_e152769_d_n6, assign100690_e152769_d_n7, assign100690_e152769_d_n8, assign100690_e152769_d_n9, assign100690_e152769_d_n10, assign100690_e152769_d_n11, assign100690_e152769_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100690_e152769;
        locals.var_dnm_dn0 = assign100690_e152769_d_n0;
        locals.var_dnm_dn2 = assign100690_e152769_d_n2;
        locals.var_dnm_dn4 = assign100690_e152769_d_n4;
        locals.var_dnm_dn5 = assign100690_e152769_d_n5;
        locals.var_dnm_dn6 = assign100690_e152769_d_n6;
        locals.var_dnm_dn7 = assign100690_e152769_d_n7;
        locals.var_dnm_dn8 = assign100690_e152769_d_n8;
        locals.var_dnm_dn9 = assign100690_e152769_d_n9;
        locals.var_dnm_dn10 = assign100690_e152769_d_n10;
        locals.var_dnm_dn11 = assign100690_e152769_d_n11;
        locals.var_dnm_dn14 = assign100690_e152769_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign100700_e152784: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2318 = assign100700_e152784;
        locals.var_guard2318_rv = 0.0;

        let assign100710_e152787: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2319 = assign100710_e152787;
        locals.var_guard2319_rv = 0.0;

        let (assign100720_e152799,) = {
    if (((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) && (locals.var_guard2319 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100720_e152799;
        locals.var_mm_rv = 0.0;

        let assign100730_e152802: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2320 = assign100730_e152802;
        locals.var_guard2320_rv = 0.0;

        let (assign100740_e152817,) = {
    if ((((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) && (locals.var_guard2319 == 0.0)) && (locals.var_guard2320 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100740_e152817;
        locals.var_mm_rv = 0.0;

        let assign100750_e152820: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2321 = assign100750_e152820;
        locals.var_guard2321_rv = 0.0;

        let (assign100760_e152838,) = {
    if (((((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) && (locals.var_guard2319 == 0.0)) && (locals.var_guard2320 == 0.0)) && (locals.var_guard2321 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100760_e152838;
        locals.var_mm_rv = 0.0;

        let assign100770_e152841: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2322 = assign100770_e152841;
        locals.var_guard2322_rv = 0.0;

        let (assign100780_e152862,) = {
    if ((((((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) && (locals.var_guard2319 == 0.0)) && (locals.var_guard2320 == 0.0)) && (locals.var_guard2321 == 0.0)) && (locals.var_guard2322 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100780_e152862;
        locals.var_mm_rv = 0.0;

        let (assign100790_e152872,) = {
    if ((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100790_e152872;
        locals.var_m0_rv = 0.0;

        let mut assign100800_loop_guard: usize = 0;
        while {
            let assign100800_cond_e152883: f64 = if (((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign100800_cond_e152883 != 0.0
        } {
            assign100800_loop_guard += 1;
            assert!(assign100800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign100800_body0_e152894, assign100800_body0_e152894_d_n0, assign100800_body0_e152894_d_n2, assign100800_body0_e152894_d_n4, assign100800_body0_e152894_d_n5, assign100800_body0_e152894_d_n6, assign100800_body0_e152894_d_n7, assign100800_body0_e152894_d_n8, assign100800_body0_e152894_d_n9, assign100800_body0_e152894_d_n10, assign100800_body0_e152894_d_n11, assign100800_body0_e152894_d_n14,) = {
    if ((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) {
        let assign100800_body0_e152892: f64 = (locals.var_dnm).sqrt();
        (assign100800_body0_e152892, (locals.var_dnm_dn0 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn2 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn4 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn5 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn6 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn7 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn8 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn9 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn10 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn11 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn14 / (2.0 * assign100800_body0_e152892)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign100800_body0_e152894;
            locals.var_dnm_dn0 = assign100800_body0_e152894_d_n0;
            locals.var_dnm_dn2 = assign100800_body0_e152894_d_n2;
            locals.var_dnm_dn4 = assign100800_body0_e152894_d_n4;
            locals.var_dnm_dn5 = assign100800_body0_e152894_d_n5;
            locals.var_dnm_dn6 = assign100800_body0_e152894_d_n6;
            locals.var_dnm_dn7 = assign100800_body0_e152894_d_n7;
            locals.var_dnm_dn8 = assign100800_body0_e152894_d_n8;
            locals.var_dnm_dn9 = assign100800_body0_e152894_d_n9;
            locals.var_dnm_dn10 = assign100800_body0_e152894_d_n10;
            locals.var_dnm_dn11 = assign100800_body0_e152894_d_n11;
            locals.var_dnm_dn14 = assign100800_body0_e152894_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign100800_body1_e152906,) = {
    if ((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) {
        let assign100800_body1_e152904: f64 = (locals.var_m0 + 1.0);
        (assign100800_body1_e152904,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign100800_body1_e152906;
            locals.var_m0_rv = 0.0;
        }

        let (assign100810_e152928, assign100810_e152928_d_n0, assign100810_e152928_d_n2, assign100810_e152928_d_n4, assign100810_e152928_d_n5, assign100810_e152928_d_n6, assign100810_e152928_d_n7, assign100810_e152928_d_n8, assign100810_e152928_d_n9, assign100810_e152928_d_n10, assign100810_e152928_d_n11, assign100810_e152928_d_n14,) = {
    if ((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 == 0.0)) {
        let (assign100810_e152926, assign100810_e152926_d_n0, assign100810_e152926_d_n2, assign100810_e152926_d_n4, assign100810_e152926_d_n5, assign100810_e152926_d_n6, assign100810_e152926_d_n7, assign100810_e152926_d_n8, assign100810_e152926_d_n9, assign100810_e152926_d_n10, assign100810_e152926_d_n11, assign100810_e152926_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign100810_e152923: f64 = (2.0 * 4.0);
                let assign100810_e152924: f64 = (1.0 / assign100810_e152923);
                let assign100810_e152925: f64 = (locals.var_dnm).powf(assign100810_e152924);
                (assign100810_e152925, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn0)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn2)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn4)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn5)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn6)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn7)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn8)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn9)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn10)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn11)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn14)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign100810_e152926, assign100810_e152926_d_n0, assign100810_e152926_d_n2, assign100810_e152926_d_n4, assign100810_e152926_d_n5, assign100810_e152926_d_n6, assign100810_e152926_d_n7, assign100810_e152926_d_n8, assign100810_e152926_d_n9, assign100810_e152926_d_n10, assign100810_e152926_d_n11, assign100810_e152926_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100810_e152928;
        locals.var_dnm_dn0 = assign100810_e152928_d_n0;
        locals.var_dnm_dn2 = assign100810_e152928_d_n2;
        locals.var_dnm_dn4 = assign100810_e152928_d_n4;
        locals.var_dnm_dn5 = assign100810_e152928_d_n5;
        locals.var_dnm_dn6 = assign100810_e152928_d_n6;
        locals.var_dnm_dn7 = assign100810_e152928_d_n7;
        locals.var_dnm_dn8 = assign100810_e152928_d_n8;
        locals.var_dnm_dn9 = assign100810_e152928_d_n9;
        locals.var_dnm_dn10 = assign100810_e152928_d_n10;
        locals.var_dnm_dn11 = assign100810_e152928_d_n11;
        locals.var_dnm_dn14 = assign100810_e152928_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign100820_e152938, assign100820_e152938_d_n0, assign100820_e152938_d_n2, assign100820_e152938_d_n4, assign100820_e152938_d_n5, assign100820_e152938_d_n6, assign100820_e152938_d_n7, assign100820_e152938_d_n8, assign100820_e152938_d_n9, assign100820_e152938_d_n10, assign100820_e152938_d_n11, assign100820_e152938_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100820_e152936: f64 = (1.0 / locals.var_dnm);
        (assign100820_e152936, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100820_e152938;
        locals.var_dnm_dn0 = assign100820_e152938_d_n0;
        locals.var_dnm_dn2 = assign100820_e152938_d_n2;
        locals.var_dnm_dn4 = assign100820_e152938_d_n4;
        locals.var_dnm_dn5 = assign100820_e152938_d_n5;
        locals.var_dnm_dn6 = assign100820_e152938_d_n6;
        locals.var_dnm_dn7 = assign100820_e152938_d_n7;
        locals.var_dnm_dn8 = assign100820_e152938_d_n8;
        locals.var_dnm_dn9 = assign100820_e152938_d_n9;
        locals.var_dnm_dn10 = assign100820_e152938_d_n10;
        locals.var_dnm_dn11 = assign100820_e152938_d_n11;
        locals.var_dnm_dn14 = assign100820_e152938_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign100830_e152950, assign100830_e152950_d_n0, assign100830_e152950_d_n2, assign100830_e152950_d_n4, assign100830_e152950_d_n5, assign100830_e152950_d_n6, assign100830_e152950_d_n7, assign100830_e152950_d_n8, assign100830_e152950_d_n9, assign100830_e152950_d_n10, assign100830_e152950_d_n11, assign100830_e152950_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100830_e152946: f64 = (locals.var_tmf1 * locals.var_t7);
        let assign100830_e152948: f64 = (assign100830_e152946 * locals.var_dnm);
        (assign100830_e152948, ((((locals.var_tmf1_dn0 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn0)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn2)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn4)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn5)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn6)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn7)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn8)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn9)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn10)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn11)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn14)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign100830_e152950;
        locals.var_tmf0_dn0 = assign100830_e152950_d_n0;
        locals.var_tmf0_dn2 = assign100830_e152950_d_n2;
        locals.var_tmf0_dn4 = assign100830_e152950_d_n4;
        locals.var_tmf0_dn5 = assign100830_e152950_d_n5;
        locals.var_tmf0_dn6 = assign100830_e152950_d_n6;
        locals.var_tmf0_dn7 = assign100830_e152950_d_n7;
        locals.var_tmf0_dn8 = assign100830_e152950_d_n8;
        locals.var_tmf0_dn9 = assign100830_e152950_d_n9;
        locals.var_tmf0_dn10 = assign100830_e152950_d_n10;
        locals.var_tmf0_dn11 = assign100830_e152950_d_n11;
        locals.var_tmf0_dn14 = assign100830_e152950_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign100840_e152964, assign100840_e152964_d_n0, assign100840_e152964_d_n2, assign100840_e152964_d_n4, assign100840_e152964_d_n5, assign100840_e152964_d_n6, assign100840_e152964_d_n7, assign100840_e152964_d_n8, assign100840_e152964_d_n9, assign100840_e152964_d_n10, assign100840_e152964_d_n11, assign100840_e152964_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100840_e152958: f64 = (locals.var_t7 * locals.var_xmp);
        let assign100840_e152960: f64 = (assign100840_e152958 * locals.var_dnm);
        let assign100840_e152962: f64 = (assign100840_e152960 / locals.var_arg);
        (assign100840_e152962, (((((((locals.var_t7_dn0 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn0)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn2 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn2)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn4 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn4)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn5 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn5)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn6 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn6)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn7 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn7)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn8 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn8)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn9 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn9)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn10 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn10)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn11 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn11)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn14 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn14)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100840_e152964;
        locals.var_t0_dn0 = assign100840_e152964_d_n0;
        locals.var_t0_dn2 = assign100840_e152964_d_n2;
        locals.var_t0_dn4 = assign100840_e152964_d_n4;
        locals.var_t0_dn5 = assign100840_e152964_d_n5;
        locals.var_t0_dn6 = assign100840_e152964_d_n6;
        locals.var_t0_dn7 = assign100840_e152964_d_n7;
        locals.var_t0_dn8 = assign100840_e152964_d_n8;
        locals.var_t0_dn9 = assign100840_e152964_d_n9;
        locals.var_t0_dn10 = assign100840_e152964_d_n10;
        locals.var_t0_dn11 = assign100840_e152964_d_n11;
        locals.var_t0_dn14 = assign100840_e152964_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign100850_e152976, assign100850_e152976_d_n0, assign100850_e152976_d_n2, assign100850_e152976_d_n4, assign100850_e152976_d_n5, assign100850_e152976_d_n6, assign100850_e152976_d_n7, assign100850_e152976_d_n8, assign100850_e152976_d_n9, assign100850_e152976_d_n10, assign100850_e152976_d_n11, assign100850_e152976_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100850_e152972: f64 = locals.var_t7;
        let assign100850_e152974: f64 = (assign100850_e152972 - locals.var_tmf0);
        (assign100850_e152974, (locals.var_t7_dn0 - locals.var_tmf0_dn0), (locals.var_t7_dn2 - locals.var_tmf0_dn2), (locals.var_t7_dn4 - locals.var_tmf0_dn4), (locals.var_t7_dn5 - locals.var_tmf0_dn5), (locals.var_t7_dn6 - locals.var_tmf0_dn6), (locals.var_t7_dn7 - locals.var_tmf0_dn7), (locals.var_t7_dn8 - locals.var_tmf0_dn8), (locals.var_t7_dn9 - locals.var_tmf0_dn9), (locals.var_t7_dn10 - locals.var_tmf0_dn10), (locals.var_t7_dn11 - locals.var_tmf0_dn11), (locals.var_t7_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100850_e152976;
        locals.var_t6_dn0 = assign100850_e152976_d_n0;
        locals.var_t6_dn2 = assign100850_e152976_d_n2;
        locals.var_t6_dn4 = assign100850_e152976_d_n4;
        locals.var_t6_dn5 = assign100850_e152976_d_n5;
        locals.var_t6_dn6 = assign100850_e152976_d_n6;
        locals.var_t6_dn7 = assign100850_e152976_d_n7;
        locals.var_t6_dn8 = assign100850_e152976_d_n8;
        locals.var_t6_dn9 = assign100850_e152976_d_n9;
        locals.var_t6_dn10 = assign100850_e152976_d_n10;
        locals.var_t6_dn11 = assign100850_e152976_d_n11;
        locals.var_t6_dn14 = assign100850_e152976_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign100860_e152984, assign100860_e152984_d_n0, assign100860_e152984_d_n2, assign100860_e152984_d_n4, assign100860_e152984_d_n5, assign100860_e152984_d_n6, assign100860_e152984_d_n7, assign100860_e152984_d_n8, assign100860_e152984_d_n9, assign100860_e152984_d_n10, assign100860_e152984_d_n11, assign100860_e152984_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100860_e152984;
        locals.var_t0_dn0 = assign100860_e152984_d_n0;
        locals.var_t0_dn2 = assign100860_e152984_d_n2;
        locals.var_t0_dn4 = assign100860_e152984_d_n4;
        locals.var_t0_dn5 = assign100860_e152984_d_n5;
        locals.var_t0_dn6 = assign100860_e152984_d_n6;
        locals.var_t0_dn7 = assign100860_e152984_d_n7;
        locals.var_t0_dn8 = assign100860_e152984_d_n8;
        locals.var_t0_dn9 = assign100860_e152984_d_n9;
        locals.var_t0_dn10 = assign100860_e152984_d_n10;
        locals.var_t0_dn11 = assign100860_e152984_d_n11;
        locals.var_t0_dn14 = assign100860_e152984_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign100870_e152993, assign100870_e152993_d_n0, assign100870_e152993_d_n2, assign100870_e152993_d_n4, assign100870_e152993_d_n5, assign100870_e152993_d_n6, assign100870_e152993_d_n7, assign100870_e152993_d_n8, assign100870_e152993_d_n9, assign100870_e152993_d_n10, assign100870_e152993_d_n11, assign100870_e152993_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100870_e152993;
        locals.var_t6_dn0 = assign100870_e152993_d_n0;
        locals.var_t6_dn2 = assign100870_e152993_d_n2;
        locals.var_t6_dn4 = assign100870_e152993_d_n4;
        locals.var_t6_dn5 = assign100870_e152993_d_n5;
        locals.var_t6_dn6 = assign100870_e152993_d_n6;
        locals.var_t6_dn7 = assign100870_e152993_d_n7;
        locals.var_t6_dn8 = assign100870_e152993_d_n8;
        locals.var_t6_dn9 = assign100870_e152993_d_n9;
        locals.var_t6_dn10 = assign100870_e152993_d_n10;
        locals.var_t6_dn11 = assign100870_e152993_d_n11;
        locals.var_t6_dn14 = assign100870_e152993_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign100880_e153002, assign100880_e153002_d_n0, assign100880_e153002_d_n2, assign100880_e153002_d_n4, assign100880_e153002_d_n5, assign100880_e153002_d_n6, assign100880_e153002_d_n7, assign100880_e153002_d_n8, assign100880_e153002_d_n9, assign100880_e153002_d_n10, assign100880_e153002_d_n11, assign100880_e153002_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100880_e153002;
        locals.var_t0_dn0 = assign100880_e153002_d_n0;
        locals.var_t0_dn2 = assign100880_e153002_d_n2;
        locals.var_t0_dn4 = assign100880_e153002_d_n4;
        locals.var_t0_dn5 = assign100880_e153002_d_n5;
        locals.var_t0_dn6 = assign100880_e153002_d_n6;
        locals.var_t0_dn7 = assign100880_e153002_d_n7;
        locals.var_t0_dn8 = assign100880_e153002_d_n8;
        locals.var_t0_dn9 = assign100880_e153002_d_n9;
        locals.var_t0_dn10 = assign100880_e153002_d_n10;
        locals.var_t0_dn11 = assign100880_e153002_d_n11;
        locals.var_t0_dn14 = assign100880_e153002_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign100890_e153009, assign100890_e153009_d_n0, assign100890_e153009_d_n2, assign100890_e153009_d_n4, assign100890_e153009_d_n5, assign100890_e153009_d_n6, assign100890_e153009_d_n7, assign100890_e153009_d_n8, assign100890_e153009_d_n9, assign100890_e153009_d_n10, assign100890_e153009_d_n11, assign100890_e153009_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100890_e153007: f64 = (locals.var_t6).sqrt();
        (assign100890_e153007, (locals.var_t6_dn0 / (2.0 * assign100890_e153007)), (locals.var_t6_dn2 / (2.0 * assign100890_e153007)), (locals.var_t6_dn4 / (2.0 * assign100890_e153007)), (locals.var_t6_dn5 / (2.0 * assign100890_e153007)), (locals.var_t6_dn6 / (2.0 * assign100890_e153007)), (locals.var_t6_dn7 / (2.0 * assign100890_e153007)), (locals.var_t6_dn8 / (2.0 * assign100890_e153007)), (locals.var_t6_dn9 / (2.0 * assign100890_e153007)), (locals.var_t6_dn10 / (2.0 * assign100890_e153007)), (locals.var_t6_dn11 / (2.0 * assign100890_e153007)), (locals.var_t6_dn14 / (2.0 * assign100890_e153007)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100890_e153009;
        locals.var_t6_dn0 = assign100890_e153009_d_n0;
        locals.var_t6_dn2 = assign100890_e153009_d_n2;
        locals.var_t6_dn4 = assign100890_e153009_d_n4;
        locals.var_t6_dn5 = assign100890_e153009_d_n5;
        locals.var_t6_dn6 = assign100890_e153009_d_n6;
        locals.var_t6_dn7 = assign100890_e153009_d_n7;
        locals.var_t6_dn8 = assign100890_e153009_d_n8;
        locals.var_t6_dn9 = assign100890_e153009_d_n9;
        locals.var_t6_dn10 = assign100890_e153009_d_n10;
        locals.var_t6_dn11 = assign100890_e153009_d_n11;
        locals.var_t6_dn14 = assign100890_e153009_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign100900_e153021, assign100900_e153021_d_n0, assign100900_e153021_d_n2, assign100900_e153021_d_n4, assign100900_e153021_d_n5, assign100900_e153021_d_n6, assign100900_e153021_d_n7, assign100900_e153021_d_n8, assign100900_e153021_d_n9, assign100900_e153021_d_n10, assign100900_e153021_d_n11, assign100900_e153021_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100900_e153017: f64 = (1.0 - locals.var_t6);
        let assign100900_e153018: f64 = (locals.var_t3 * assign100900_e153017);
        let assign100900_e153019: f64 = (locals.var_t1 + assign100900_e153018);
        (assign100900_e153019, (locals.var_t1_dn0 + ((locals.var_t3_dn0 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn0)))), (locals.var_t1_dn2 + ((locals.var_t3_dn2 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn2)))), (locals.var_t1_dn4 + ((locals.var_t3_dn4 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn4)))), (locals.var_t1_dn5 + ((locals.var_t3_dn5 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn5)))), (locals.var_t1_dn6 + ((locals.var_t3_dn6 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn6)))), (locals.var_t1_dn7 + ((locals.var_t3_dn7 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn7)))), (locals.var_t1_dn8 + ((locals.var_t3_dn8 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn8)))), (locals.var_t1_dn9 + ((locals.var_t3_dn9 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn9)))), (locals.var_t1_dn10 + ((locals.var_t3_dn10 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn10)))), (locals.var_t1_dn11 + ((locals.var_t3_dn11 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn11)))), (locals.var_t1_dn14 + ((locals.var_t3_dn14 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn14)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn4, locals.var_psislsat_dn5, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn8, locals.var_psislsat_dn9, locals.var_psislsat_dn10, locals.var_psislsat_dn11, locals.var_psislsat_dn14,)
    }
};
        locals.var_psislsat = assign100900_e153021;
        locals.var_psislsat_dn0 = assign100900_e153021_d_n0;
        locals.var_psislsat_dn2 = assign100900_e153021_d_n2;
        locals.var_psislsat_dn4 = assign100900_e153021_d_n4;
        locals.var_psislsat_dn5 = assign100900_e153021_d_n5;
        locals.var_psislsat_dn6 = assign100900_e153021_d_n6;
        locals.var_psislsat_dn7 = assign100900_e153021_d_n7;
        locals.var_psislsat_dn8 = assign100900_e153021_d_n8;
        locals.var_psislsat_dn9 = assign100900_e153021_d_n9;
        locals.var_psislsat_dn10 = assign100900_e153021_d_n10;
        locals.var_psislsat_dn11 = assign100900_e153021_d_n11;
        locals.var_psislsat_dn14 = assign100900_e153021_d_n14;
        locals.var_psislsat_rv = 0.0;

        let (assign100910_e153031, assign100910_e153031_d_n0, assign100910_e153031_d_n2, assign100910_e153031_d_n4, assign100910_e153031_d_n5, assign100910_e153031_d_n6, assign100910_e153031_d_n7, assign100910_e153031_d_n8, assign100910_e153031_d_n9, assign100910_e153031_d_n10, assign100910_e153031_d_n11, assign100910_e153031_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100910_e153028: f64 = (locals.var_xgate_1 + locals.var_lgate);
        let assign100910_e153029: f64 = (locals.var_lgate / assign100910_e153028);
        (assign100910_e153029, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign100910_e153031;
        locals.var_t2_dn0 = assign100910_e153031_d_n0;
        locals.var_t2_dn2 = assign100910_e153031_d_n2;
        locals.var_t2_dn4 = assign100910_e153031_d_n4;
        locals.var_t2_dn5 = assign100910_e153031_d_n5;
        locals.var_t2_dn6 = assign100910_e153031_d_n6;
        locals.var_t2_dn7 = assign100910_e153031_d_n7;
        locals.var_t2_dn8 = assign100910_e153031_d_n8;
        locals.var_t2_dn9 = assign100910_e153031_d_n9;
        locals.var_t2_dn10 = assign100910_e153031_d_n10;
        locals.var_t2_dn11 = assign100910_e153031_d_n11;
        locals.var_t2_dn14 = assign100910_e153031_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_388(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign100920_e153045, assign100920_e153045_d_n0, assign100920_e153045_d_n2, assign100920_e153045_d_n4, assign100920_e153045_d_n5, assign100920_e153045_d_n6, assign100920_e153045_d_n7, assign100920_e153045_d_n8, assign100920_e153045_d_n9, assign100920_e153045_d_n10, assign100920_e153045_d_n11, assign100920_e153045_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100920_e153037: f64 = (locals.var_uc_svdssnp * locals.var_vdsz__blk443);
        let assign100920_e153039: f64 = (assign100920_e153037 + locals.var_ps0z);
        let assign100920_e153042: f64 = (locals.var_t2 * locals.var_psislsat);
        let assign100920_e153043: f64 = (assign100920_e153039 - assign100920_e153042);
        (assign100920_e153043, (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2_dn0 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn0))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2_dn2 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn2))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn4) + locals.var_ps0z_dn4) - ((locals.var_t2_dn4 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn4))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn5) + locals.var_ps0z_dn5) - ((locals.var_t2_dn5 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn5))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2_dn6 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn6))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2_dn7 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn7))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn8) + locals.var_ps0z_dn8) - ((locals.var_t2_dn8 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn8))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn9) + locals.var_ps0z_dn9) - ((locals.var_t2_dn9 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn9))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2_dn10 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn10))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn11) + locals.var_ps0z_dn11) - ((locals.var_t2_dn11 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn11))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn14) + locals.var_ps0z_dn14) - ((locals.var_t2_dn14 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn14))),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100920_e153045;
        locals.var_psisubsat_dn0 = assign100920_e153045_d_n0;
        locals.var_psisubsat_dn2 = assign100920_e153045_d_n2;
        locals.var_psisubsat_dn4 = assign100920_e153045_d_n4;
        locals.var_psisubsat_dn5 = assign100920_e153045_d_n5;
        locals.var_psisubsat_dn6 = assign100920_e153045_d_n6;
        locals.var_psisubsat_dn7 = assign100920_e153045_d_n7;
        locals.var_psisubsat_dn8 = assign100920_e153045_d_n8;
        locals.var_psisubsat_dn9 = assign100920_e153045_d_n9;
        locals.var_psisubsat_dn10 = assign100920_e153045_d_n10;
        locals.var_psisubsat_dn11 = assign100920_e153045_d_n11;
        locals.var_psisubsat_dn14 = assign100920_e153045_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let (assign100930_e153060, assign100930_e153060_d_n0, assign100930_e153060_d_n2, assign100930_e153060_d_n4, assign100930_e153060_d_n5, assign100930_e153060_d_n6, assign100930_e153060_d_n7, assign100930_e153060_d_n8, assign100930_e153060_d_n9, assign100930_e153060_d_n10, assign100930_e153060_d_n11, assign100930_e153060_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100930_e153051: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign100930_e153054: f64 = (4.0 * 0.001);
        let assign100930_e153056: f64 = (assign100930_e153054 * 0.001);
        let assign100930_e153057: f64 = (assign100930_e153051 + assign100930_e153056);
        let assign100930_e153058: f64 = (assign100930_e153057).sqrt();
        (assign100930_e153058, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn4 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn4)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn5 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn5)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn8 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn8)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn9 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn9)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn11 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn11)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn14 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn14)) / (2.0 * assign100930_e153058)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100930_e153060;
        locals.var_tmf2_dn0 = assign100930_e153060_d_n0;
        locals.var_tmf2_dn2 = assign100930_e153060_d_n2;
        locals.var_tmf2_dn4 = assign100930_e153060_d_n4;
        locals.var_tmf2_dn5 = assign100930_e153060_d_n5;
        locals.var_tmf2_dn6 = assign100930_e153060_d_n6;
        locals.var_tmf2_dn7 = assign100930_e153060_d_n7;
        locals.var_tmf2_dn8 = assign100930_e153060_d_n8;
        locals.var_tmf2_dn9 = assign100930_e153060_d_n9;
        locals.var_tmf2_dn10 = assign100930_e153060_d_n10;
        locals.var_tmf2_dn11 = assign100930_e153060_d_n11;
        locals.var_tmf2_dn14 = assign100930_e153060_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign100940_e153072, assign100940_e153072_d_n0, assign100940_e153072_d_n2, assign100940_e153072_d_n4, assign100940_e153072_d_n5, assign100940_e153072_d_n6, assign100940_e153072_d_n7, assign100940_e153072_d_n8, assign100940_e153072_d_n9, assign100940_e153072_d_n10, assign100940_e153072_d_n11, assign100940_e153072_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100940_e153068: f64 = (locals.var_psisubsat / locals.var_tmf2);
        let assign100940_e153069: f64 = (1.0 + assign100940_e153068);
        let assign100940_e153070: f64 = (0.5 * assign100940_e153069);
        (assign100940_e153070, (0.5 * (((locals.var_psisubsat_dn0 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn2 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn4 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn5 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn6 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn7 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn8 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn9 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn10 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn11 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn14 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign100940_e153072;
        locals.var_t9_dn0 = assign100940_e153072_d_n0;
        locals.var_t9_dn2 = assign100940_e153072_d_n2;
        locals.var_t9_dn4 = assign100940_e153072_d_n4;
        locals.var_t9_dn5 = assign100940_e153072_d_n5;
        locals.var_t9_dn6 = assign100940_e153072_d_n6;
        locals.var_t9_dn7 = assign100940_e153072_d_n7;
        locals.var_t9_dn8 = assign100940_e153072_d_n8;
        locals.var_t9_dn9 = assign100940_e153072_d_n9;
        locals.var_t9_dn10 = assign100940_e153072_d_n10;
        locals.var_t9_dn11 = assign100940_e153072_d_n11;
        locals.var_t9_dn14 = assign100940_e153072_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign100950_e153082, assign100950_e153082_d_n0, assign100950_e153082_d_n2, assign100950_e153082_d_n4, assign100950_e153082_d_n5, assign100950_e153082_d_n6, assign100950_e153082_d_n7, assign100950_e153082_d_n8, assign100950_e153082_d_n9, assign100950_e153082_d_n10, assign100950_e153082_d_n11, assign100950_e153082_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100950_e153079: f64 = (locals.var_psisubsat + locals.var_tmf2);
        let assign100950_e153080: f64 = (0.5 * assign100950_e153079);
        (assign100950_e153080, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_psisubsat_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_psisubsat_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_psisubsat_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_psisubsat_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_psisubsat_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_psisubsat_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100950_e153082;
        locals.var_psisubsat_dn0 = assign100950_e153082_d_n0;
        locals.var_psisubsat_dn2 = assign100950_e153082_d_n2;
        locals.var_psisubsat_dn4 = assign100950_e153082_d_n4;
        locals.var_psisubsat_dn5 = assign100950_e153082_d_n5;
        locals.var_psisubsat_dn6 = assign100950_e153082_d_n6;
        locals.var_psisubsat_dn7 = assign100950_e153082_d_n7;
        locals.var_psisubsat_dn8 = assign100950_e153082_d_n8;
        locals.var_psisubsat_dn9 = assign100950_e153082_d_n9;
        locals.var_psisubsat_dn10 = assign100950_e153082_d_n10;
        locals.var_psisubsat_dn11 = assign100950_e153082_d_n11;
        locals.var_psisubsat_dn14 = assign100950_e153082_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let assign100960_e153085: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2323 = assign100960_e153085;
        locals.var_guard2323_rv = 0.0;

        let (assign100970_e153093, assign100970_e153093_d_n0, assign100970_e153093_d_n2, assign100970_e153093_d_n4, assign100970_e153093_d_n5, assign100970_e153093_d_n6, assign100970_e153093_d_n7, assign100970_e153093_d_n8, assign100970_e153093_d_n9, assign100970_e153093_d_n10, assign100970_e153093_d_n11, assign100970_e153093_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100970_e153093;
        locals.var_psisubsat_dn0 = assign100970_e153093_d_n0;
        locals.var_psisubsat_dn2 = assign100970_e153093_d_n2;
        locals.var_psisubsat_dn4 = assign100970_e153093_d_n4;
        locals.var_psisubsat_dn5 = assign100970_e153093_d_n5;
        locals.var_psisubsat_dn6 = assign100970_e153093_d_n6;
        locals.var_psisubsat_dn7 = assign100970_e153093_d_n7;
        locals.var_psisubsat_dn8 = assign100970_e153093_d_n8;
        locals.var_psisubsat_dn9 = assign100970_e153093_d_n9;
        locals.var_psisubsat_dn10 = assign100970_e153093_d_n10;
        locals.var_psisubsat_dn11 = assign100970_e153093_d_n11;
        locals.var_psisubsat_dn14 = assign100970_e153093_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let (assign100980_e153101, assign100980_e153101_d_n0, assign100980_e153101_d_n2, assign100980_e153101_d_n4, assign100980_e153101_d_n5, assign100980_e153101_d_n6, assign100980_e153101_d_n7, assign100980_e153101_d_n8, assign100980_e153101_d_n9, assign100980_e153101_d_n10, assign100980_e153101_d_n11, assign100980_e153101_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign100980_e153101;
        locals.var_t9_dn0 = assign100980_e153101_d_n0;
        locals.var_t9_dn2 = assign100980_e153101_d_n2;
        locals.var_t9_dn4 = assign100980_e153101_d_n4;
        locals.var_t9_dn5 = assign100980_e153101_d_n5;
        locals.var_t9_dn6 = assign100980_e153101_d_n6;
        locals.var_t9_dn7 = assign100980_e153101_d_n7;
        locals.var_t9_dn8 = assign100980_e153101_d_n8;
        locals.var_t9_dn9 = assign100980_e153101_d_n9;
        locals.var_t9_dn10 = assign100980_e153101_d_n10;
        locals.var_t9_dn11 = assign100980_e153101_d_n11;
        locals.var_t9_dn14 = assign100980_e153101_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign100990_e153109, assign100990_e153109_d_n0, assign100990_e153109_d_n2, assign100990_e153109_d_n4, assign100990_e153109_d_n5, assign100990_e153109_d_n6, assign100990_e153109_d_n7, assign100990_e153109_d_n8, assign100990_e153109_d_n9, assign100990_e153109_d_n10, assign100990_e153109_d_n11, assign100990_e153109_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100990_e153107: f64 = (locals.var_psisubsat + 1e-25);
        (assign100990_e153107, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100990_e153109;
        locals.var_psisubsat_dn0 = assign100990_e153109_d_n0;
        locals.var_psisubsat_dn2 = assign100990_e153109_d_n2;
        locals.var_psisubsat_dn4 = assign100990_e153109_d_n4;
        locals.var_psisubsat_dn5 = assign100990_e153109_d_n5;
        locals.var_psisubsat_dn6 = assign100990_e153109_d_n6;
        locals.var_psisubsat_dn7 = assign100990_e153109_d_n7;
        locals.var_psisubsat_dn8 = assign100990_e153109_d_n8;
        locals.var_psisubsat_dn9 = assign100990_e153109_d_n9;
        locals.var_psisubsat_dn10 = assign100990_e153109_d_n10;
        locals.var_psisubsat_dn11 = assign100990_e153109_d_n11;
        locals.var_psisubsat_dn14 = assign100990_e153109_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let (assign101000_e153121, assign101000_e153121_d_n0, assign101000_e153121_d_n2, assign101000_e153121_d_n4, assign101000_e153121_d_n5, assign101000_e153121_d_n6, assign101000_e153121_d_n7, assign101000_e153121_d_n8, assign101000_e153121_d_n9, assign101000_e153121_d_n10, assign101000_e153121_d_n11, assign101000_e153121_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign101000_e153117: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign101000_e153118: f64 = (locals.var_uc_subtmp * assign101000_e153117);
        let assign101000_e153119: f64 = (1.0 + assign101000_e153118);
        (assign101000_e153119, (locals.var_uc_subtmp * locals.var_ttemp_dn0), (locals.var_uc_subtmp * locals.var_ttemp_dn2), (locals.var_uc_subtmp * locals.var_ttemp_dn4), (locals.var_uc_subtmp * locals.var_ttemp_dn5), (locals.var_uc_subtmp * locals.var_ttemp_dn6), (locals.var_uc_subtmp * locals.var_ttemp_dn7), (locals.var_uc_subtmp * locals.var_ttemp_dn8), (locals.var_uc_subtmp * locals.var_ttemp_dn9), (locals.var_uc_subtmp * locals.var_ttemp_dn10), (locals.var_uc_subtmp * locals.var_ttemp_dn11), (locals.var_uc_subtmp * locals.var_ttemp_dn14),)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
    }
};
        locals.var_xsubtmp = assign101000_e153121;
        locals.var_xsubtmp_dn0 = assign101000_e153121_d_n0;
        locals.var_xsubtmp_dn2 = assign101000_e153121_d_n2;
        locals.var_xsubtmp_dn4 = assign101000_e153121_d_n4;
        locals.var_xsubtmp_dn5 = assign101000_e153121_d_n5;
        locals.var_xsubtmp_dn6 = assign101000_e153121_d_n6;
        locals.var_xsubtmp_dn7 = assign101000_e153121_d_n7;
        locals.var_xsubtmp_dn8 = assign101000_e153121_d_n8;
        locals.var_xsubtmp_dn9 = assign101000_e153121_d_n9;
        locals.var_xsubtmp_dn10 = assign101000_e153121_d_n10;
        locals.var_xsubtmp_dn11 = assign101000_e153121_d_n11;
        locals.var_xsubtmp_dn14 = assign101000_e153121_d_n14;
        locals.var_xsubtmp_rv = 0.0;

        let (assign101010_e153132, assign101010_e153132_d_n0, assign101010_e153132_d_n2, assign101010_e153132_d_n4, assign101010_e153132_d_n5, assign101010_e153132_d_n6, assign101010_e153132_d_n7, assign101010_e153132_d_n8, assign101010_e153132_d_n9, assign101010_e153132_d_n10, assign101010_e153132_d_n11, assign101010_e153132_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let (assign101010_e153130, assign101010_e153130_d_n0, assign101010_e153130_d_n2, assign101010_e153130_d_n4, assign101010_e153130_d_n5, assign101010_e153130_d_n6, assign101010_e153130_d_n7, assign101010_e153130_d_n8, assign101010_e153130_d_n9, assign101010_e153130_d_n10, assign101010_e153130_d_n11, assign101010_e153130_d_n14,) = {
            if (locals.var_xsubtmp <= 0.001) {
                (0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
            }
        };
        (assign101010_e153130, assign101010_e153130_d_n0, assign101010_e153130_d_n2, assign101010_e153130_d_n4, assign101010_e153130_d_n5, assign101010_e153130_d_n6, assign101010_e153130_d_n7, assign101010_e153130_d_n8, assign101010_e153130_d_n9, assign101010_e153130_d_n10, assign101010_e153130_d_n11, assign101010_e153130_d_n14,)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
    }
};
        locals.var_xsubtmp = assign101010_e153132;
        locals.var_xsubtmp_dn0 = assign101010_e153132_d_n0;
        locals.var_xsubtmp_dn2 = assign101010_e153132_d_n2;
        locals.var_xsubtmp_dn4 = assign101010_e153132_d_n4;
        locals.var_xsubtmp_dn5 = assign101010_e153132_d_n5;
        locals.var_xsubtmp_dn6 = assign101010_e153132_d_n6;
        locals.var_xsubtmp_dn7 = assign101010_e153132_d_n7;
        locals.var_xsubtmp_dn8 = assign101010_e153132_d_n8;
        locals.var_xsubtmp_dn9 = assign101010_e153132_d_n9;
        locals.var_xsubtmp_dn10 = assign101010_e153132_d_n10;
        locals.var_xsubtmp_dn11 = assign101010_e153132_d_n11;
        locals.var_xsubtmp_dn14 = assign101010_e153132_d_n14;
        locals.var_xsubtmp_rv = 0.0;

        let (assign101020_e153140, assign101020_e153140_d_n0, assign101020_e153140_d_n2, assign101020_e153140_d_n4, assign101020_e153140_d_n5, assign101020_e153140_d_n6, assign101020_e153140_d_n7, assign101020_e153140_d_n8, assign101020_e153140_d_n9, assign101020_e153140_d_n10, assign101020_e153140_d_n11, assign101020_e153140_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign101020_e153138: f64 = (locals.var_xsub1_1 / locals.var_xsubtmp);
        (assign101020_e153138, (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn0) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn2) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn4) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn5) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn6) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn7) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn8) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn9) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn10) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn11) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn14) / (locals.var_xsubtmp * locals.var_xsubtmp))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101020_e153140;
        locals.var_t5_dn0 = assign101020_e153140_d_n0;
        locals.var_t5_dn2 = assign101020_e153140_d_n2;
        locals.var_t5_dn4 = assign101020_e153140_d_n4;
        locals.var_t5_dn5 = assign101020_e153140_d_n5;
        locals.var_t5_dn6 = assign101020_e153140_d_n6;
        locals.var_t5_dn7 = assign101020_e153140_d_n7;
        locals.var_t5_dn8 = assign101020_e153140_d_n8;
        locals.var_t5_dn9 = assign101020_e153140_d_n9;
        locals.var_t5_dn10 = assign101020_e153140_d_n10;
        locals.var_t5_dn11 = assign101020_e153140_d_n11;
        locals.var_t5_dn14 = assign101020_e153140_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign101030_e153148, assign101030_e153148_d_n0, assign101030_e153148_d_n2, assign101030_e153148_d_n4, assign101030_e153148_d_n5, assign101030_e153148_d_n6, assign101030_e153148_d_n7, assign101030_e153148_d_n8, assign101030_e153148_d_n9, assign101030_e153148_d_n10, assign101030_e153148_d_n11, assign101030_e153148_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign101030_e153146: f64 = (locals.var_xsub2_1 * locals.var_xsubtmp);
        (assign101030_e153146, (locals.var_xsub2_1 * locals.var_xsubtmp_dn0), (locals.var_xsub2_1 * locals.var_xsubtmp_dn2), (locals.var_xsub2_1 * locals.var_xsubtmp_dn4), (locals.var_xsub2_1 * locals.var_xsubtmp_dn5), (locals.var_xsub2_1 * locals.var_xsubtmp_dn6), (locals.var_xsub2_1 * locals.var_xsubtmp_dn7), (locals.var_xsub2_1 * locals.var_xsubtmp_dn8), (locals.var_xsub2_1 * locals.var_xsubtmp_dn9), (locals.var_xsub2_1 * locals.var_xsubtmp_dn10), (locals.var_xsub2_1 * locals.var_xsubtmp_dn11), (locals.var_xsub2_1 * locals.var_xsubtmp_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign101030_e153148;
        locals.var_t6_dn0 = assign101030_e153148_d_n0;
        locals.var_t6_dn2 = assign101030_e153148_d_n2;
        locals.var_t6_dn4 = assign101030_e153148_d_n4;
        locals.var_t6_dn5 = assign101030_e153148_d_n5;
        locals.var_t6_dn6 = assign101030_e153148_d_n6;
        locals.var_t6_dn7 = assign101030_e153148_d_n7;
        locals.var_t6_dn8 = assign101030_e153148_d_n8;
        locals.var_t6_dn9 = assign101030_e153148_d_n9;
        locals.var_t6_dn10 = assign101030_e153148_d_n10;
        locals.var_t6_dn11 = assign101030_e153148_d_n11;
        locals.var_t6_dn14 = assign101030_e153148_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign101040_e153158, assign101040_e153158_d_n0, assign101040_e153158_d_n2, assign101040_e153158_d_n4, assign101040_e153158_d_n5, assign101040_e153158_d_n6, assign101040_e153158_d_n7, assign101040_e153158_d_n8, assign101040_e153158_d_n9, assign101040_e153158_d_n10, assign101040_e153158_d_n11, assign101040_e153158_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign101040_e153153: f64 = (-locals.var_t6);
        let assign101040_e153155: f64 = (assign101040_e153153 / locals.var_psisubsat);
        let assign101040_e153156: f64 = (assign101040_e153155).exp();
        (assign101040_e153156, (assign101040_e153156 * ((((-locals.var_t6_dn0) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn0)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn2) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn2)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn4) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn4)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn5) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn5)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn6) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn6)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn7) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn7)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn8) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn8)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn9) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn9)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn10) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn10)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn11) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn11)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn14) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn14)) / (locals.var_psisubsat * locals.var_psisubsat))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101040_e153158;
        locals.var_t2_dn0 = assign101040_e153158_d_n0;
        locals.var_t2_dn2 = assign101040_e153158_d_n2;
        locals.var_t2_dn4 = assign101040_e153158_d_n4;
        locals.var_t2_dn5 = assign101040_e153158_d_n5;
        locals.var_t2_dn6 = assign101040_e153158_d_n6;
        locals.var_t2_dn7 = assign101040_e153158_d_n7;
        locals.var_t2_dn8 = assign101040_e153158_d_n8;
        locals.var_t2_dn9 = assign101040_e153158_d_n9;
        locals.var_t2_dn10 = assign101040_e153158_d_n10;
        locals.var_t2_dn11 = assign101040_e153158_d_n11;
        locals.var_t2_dn14 = assign101040_e153158_d_n14;
        locals.var_t2_rv = 0.0;

        let assign101090_e153199: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2325 = assign101090_e153199;
        locals.var_guard2325_rv = 0.0;

        let (assign101100_e153205, assign101100_e153205_d_n0, assign101100_e153205_d_n2, assign101100_e153205_d_n4, assign101100_e153205_d_n5, assign101100_e153205_d_n6, assign101100_e153205_d_n7, assign101100_e153205_d_n8, assign101100_e153205_d_n9, assign101100_e153205_d_n10, assign101100_e153205_d_n11, assign101100_e153205_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        (p.p270, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign101100_e153205;
        locals.var_t12_dn0 = assign101100_e153205_d_n0;
        locals.var_t12_dn2 = assign101100_e153205_d_n2;
        locals.var_t12_dn4 = assign101100_e153205_d_n4;
        locals.var_t12_dn5 = assign101100_e153205_d_n5;
        locals.var_t12_dn6 = assign101100_e153205_d_n6;
        locals.var_t12_dn7 = assign101100_e153205_d_n7;
        locals.var_t12_dn8 = assign101100_e153205_d_n8;
        locals.var_t12_dn9 = assign101100_e153205_d_n9;
        locals.var_t12_dn10 = assign101100_e153205_d_n10;
        locals.var_t12_dn11 = assign101100_e153205_d_n11;
        locals.var_t12_dn14 = assign101100_e153205_d_n14;
        locals.var_t12_rv = 0.0;

        let (assign101110_e153211, assign101110_e153211_d_n0, assign101110_e153211_d_n2, assign101110_e153211_d_n4, assign101110_e153211_d_n5, assign101110_e153211_d_n6, assign101110_e153211_d_n7, assign101110_e153211_d_n8, assign101110_e153211_d_n9, assign101110_e153211_d_n10, assign101110_e153211_d_n11, assign101110_e153211_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        (p.p271, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101110_e153211;
        locals.var_t10_dn0 = assign101110_e153211_d_n0;
        locals.var_t10_dn2 = assign101110_e153211_d_n2;
        locals.var_t10_dn4 = assign101110_e153211_d_n4;
        locals.var_t10_dn5 = assign101110_e153211_d_n5;
        locals.var_t10_dn6 = assign101110_e153211_d_n6;
        locals.var_t10_dn7 = assign101110_e153211_d_n7;
        locals.var_t10_dn8 = assign101110_e153211_d_n8;
        locals.var_t10_dn9 = assign101110_e153211_d_n9;
        locals.var_t10_dn10 = assign101110_e153211_d_n10;
        locals.var_t10_dn11 = assign101110_e153211_d_n11;
        locals.var_t10_dn14 = assign101110_e153211_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign101120_e153217, assign101120_e153217_d_n0, assign101120_e153217_d_n2, assign101120_e153217_d_n4, assign101120_e153217_d_n5, assign101120_e153217_d_n6, assign101120_e153217_d_n7, assign101120_e153217_d_n8, assign101120_e153217_d_n9, assign101120_e153217_d_n10, assign101120_e153217_d_n11, assign101120_e153217_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101120_e153217;
        locals.var_t3_dn0 = assign101120_e153217_d_n0;
        locals.var_t3_dn2 = assign101120_e153217_d_n2;
        locals.var_t3_dn4 = assign101120_e153217_d_n4;
        locals.var_t3_dn5 = assign101120_e153217_d_n5;
        locals.var_t3_dn6 = assign101120_e153217_d_n6;
        locals.var_t3_dn7 = assign101120_e153217_d_n7;
        locals.var_t3_dn8 = assign101120_e153217_d_n8;
        locals.var_t3_dn9 = assign101120_e153217_d_n9;
        locals.var_t3_dn10 = assign101120_e153217_d_n10;
        locals.var_t3_dn11 = assign101120_e153217_d_n11;
        locals.var_t3_dn14 = assign101120_e153217_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign101130_e153229, assign101130_e153229_d_n0, assign101130_e153229_d_n2, assign101130_e153229_d_n4, assign101130_e153229_d_n5, assign101130_e153229_d_n6, assign101130_e153229_d_n7, assign101130_e153229_d_n8, assign101130_e153229_d_n9, assign101130_e153229_d_n10, assign101130_e153229_d_n11, assign101130_e153229_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        let assign101130_e153223: f64 = (locals.var_t12 * locals.var_t10);
        let assign101130_e153225: f64 = (assign101130_e153223 * locals.var_t3);
        let assign101130_e153227: f64 = (assign101130_e153225 * locals.var_t3);
        (assign101130_e153227, ((((((locals.var_t12_dn0 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn0)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn0)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn0)), ((((((locals.var_t12_dn2 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn2)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn2)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn2)), ((((((locals.var_t12_dn4 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn4)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn4)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn4)), ((((((locals.var_t12_dn5 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn5)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn5)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn5)), ((((((locals.var_t12_dn6 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn6)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn6)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn6)), ((((((locals.var_t12_dn7 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn7)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn7)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn7)), ((((((locals.var_t12_dn8 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn8)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn8)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn8)), ((((((locals.var_t12_dn9 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn9)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn9)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn9)), ((((((locals.var_t12_dn10 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn10)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn10)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn10)), ((((((locals.var_t12_dn11 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn11)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn11)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn11)), ((((((locals.var_t12_dn14 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn14)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn14)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign101130_e153229;
        locals.var_t1_dn0 = assign101130_e153229_d_n0;
        locals.var_t1_dn2 = assign101130_e153229_d_n2;
        locals.var_t1_dn4 = assign101130_e153229_d_n4;
        locals.var_t1_dn5 = assign101130_e153229_d_n5;
        locals.var_t1_dn6 = assign101130_e153229_d_n6;
        locals.var_t1_dn7 = assign101130_e153229_d_n7;
        locals.var_t1_dn8 = assign101130_e153229_d_n8;
        locals.var_t1_dn9 = assign101130_e153229_d_n9;
        locals.var_t1_dn10 = assign101130_e153229_d_n10;
        locals.var_t1_dn11 = assign101130_e153229_d_n11;
        locals.var_t1_dn14 = assign101130_e153229_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign101140_e153247, assign101140_e153247_d_n0, assign101140_e153247_d_n2, assign101140_e153247_d_n4, assign101140_e153247_d_n5, assign101140_e153247_d_n6, assign101140_e153247_d_n7, assign101140_e153247_d_n8, assign101140_e153247_d_n9, assign101140_e153247_d_n10, assign101140_e153247_d_n11, assign101140_e153247_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        let assign101140_e153235: f64 = (locals.var_mu * locals.var_vgvt);
        let assign101140_e153237: f64 = (assign101140_e153235 * locals.var_t12);
        let assign101140_e153240: f64 = (locals.var_t10 * locals.var_t3);
        let assign101140_e153242: f64 = (assign101140_e153240 * locals.var_t3);
        let assign101140_e153243: f64 = (assign101140_e153237 + assign101140_e153242);
        let assign101140_e153245: f64 = (assign101140_e153243 + 1e-25);
        (assign101140_e153245, (((((locals.var_mu_dn0 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn0)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn0)) + ((((locals.var_t10_dn0 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn0)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn0))), (((((locals.var_mu_dn2 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn2)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn2)) + ((((locals.var_t10_dn2 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn2)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn2))), (((((locals.var_mu_dn4 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn4)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn4)) + ((((locals.var_t10_dn4 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn4)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn4))), (((((locals.var_mu_dn5 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn5)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn5)) + ((((locals.var_t10_dn5 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn5)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn5))), (((((locals.var_mu_dn6 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn6)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn6)) + ((((locals.var_t10_dn6 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn6)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn6))), (((((locals.var_mu_dn7 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn7)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn7)) + ((((locals.var_t10_dn7 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn7)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn7))), (((((locals.var_mu_dn8 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn8)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn8)) + ((((locals.var_t10_dn8 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn8)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn8))), (((((locals.var_mu_dn9 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn9)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn9)) + ((((locals.var_t10_dn9 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn9)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn9))), (((((locals.var_mu_dn10 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn10)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn10)) + ((((locals.var_t10_dn10 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn10)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn10))), (((((locals.var_mu_dn11 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn11)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn11)) + ((((locals.var_t10_dn11 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn11)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn11))), (((((locals.var_mu_dn14 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn14)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn14)) + ((((locals.var_t10_dn14 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn14)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101140_e153247;
        locals.var_t2_dn0 = assign101140_e153247_d_n0;
        locals.var_t2_dn2 = assign101140_e153247_d_n2;
        locals.var_t2_dn4 = assign101140_e153247_d_n4;
        locals.var_t2_dn5 = assign101140_e153247_d_n5;
        locals.var_t2_dn6 = assign101140_e153247_d_n6;
        locals.var_t2_dn7 = assign101140_e153247_d_n7;
        locals.var_t2_dn8 = assign101140_e153247_d_n8;
        locals.var_t2_dn9 = assign101140_e153247_d_n9;
        locals.var_t2_dn10 = assign101140_e153247_d_n10;
        locals.var_t2_dn11 = assign101140_e153247_d_n11;
        locals.var_t2_dn14 = assign101140_e153247_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign101170_e153266, assign101170_e153266_d_n0, assign101170_e153266_d_n2, assign101170_e153266_d_n4, assign101170_e153266_d_n5, assign101170_e153266_d_n6, assign101170_e153266_d_n7, assign101170_e153266_d_n8, assign101170_e153266_d_n9, assign101170_e153266_d_n10, assign101170_e153266_d_n11, assign101170_e153266_d_n14,) = {
    if (locals.var_flg_nqs != 0.0) {
        (locals.var_mks_dly3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101170_e153266;
        locals.var_t2_dn0 = assign101170_e153266_d_n0;
        locals.var_t2_dn2 = assign101170_e153266_d_n2;
        locals.var_t2_dn4 = assign101170_e153266_d_n4;
        locals.var_t2_dn5 = assign101170_e153266_d_n5;
        locals.var_t2_dn6 = assign101170_e153266_d_n6;
        locals.var_t2_dn7 = assign101170_e153266_d_n7;
        locals.var_t2_dn8 = assign101170_e153266_d_n8;
        locals.var_t2_dn9 = assign101170_e153266_d_n9;
        locals.var_t2_dn10 = assign101170_e153266_d_n10;
        locals.var_t2_dn11 = assign101170_e153266_d_n11;
        locals.var_t2_dn14 = assign101170_e153266_d_n14;
        locals.var_t2_rv = 0.0;

        let assign101190_e153278: f64 = if ((p.p26 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2326 = assign101190_e153278;
        locals.var_guard2326_rv = 0.0;

        let (assign101200_e153282,) = {
    if (locals.var_guard2326 != 0.0) {
        (locals.var_uc_nfalp,)
    } else {
        (locals.var_nfalpe,)
    }
};
        locals.var_nfalpe = assign101200_e153282;
        locals.var_nfalpe_rv = 0.0;

        let (assign101220_e153290,) = {
    if (locals.var_guard2326 != 0.0) {
        (locals.var_mks_cit,)
    } else {
        (locals.var_cite,)
    }
};
        locals.var_cite = assign101220_e153290;
        locals.var_cite_rv = 0.0;

        let (assign101230_e153296, assign101230_e153296_d_n0, assign101230_e153296_d_n2, assign101230_e153296_d_n4, assign101230_e153296_d_n5, assign101230_e153296_d_n6, assign101230_e153296_d_n7, assign101230_e153296_d_n8, assign101230_e153296_d_n9, assign101230_e153296_d_n10, assign101230_e153296_d_n11, assign101230_e153296_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101230_e153294: f64 = (locals.var_qn0 / 1.6021918e-19);
        (assign101230_e153294, (locals.var_qn0_dn0 / 1.6021918e-19), (locals.var_qn0_dn2 / 1.6021918e-19), (locals.var_qn0_dn4 / 1.6021918e-19), (locals.var_qn0_dn5 / 1.6021918e-19), (locals.var_qn0_dn6 / 1.6021918e-19), (locals.var_qn0_dn7 / 1.6021918e-19), (locals.var_qn0_dn8 / 1.6021918e-19), (locals.var_qn0_dn9 / 1.6021918e-19), (locals.var_qn0_dn10 / 1.6021918e-19), (locals.var_qn0_dn11 / 1.6021918e-19), (locals.var_qn0_dn14 / 1.6021918e-19),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign101230_e153296;
        locals.var_t1_dn0 = assign101230_e153296_d_n0;
        locals.var_t1_dn2 = assign101230_e153296_d_n2;
        locals.var_t1_dn4 = assign101230_e153296_d_n4;
        locals.var_t1_dn5 = assign101230_e153296_d_n5;
        locals.var_t1_dn6 = assign101230_e153296_d_n6;
        locals.var_t1_dn7 = assign101230_e153296_d_n7;
        locals.var_t1_dn8 = assign101230_e153296_d_n8;
        locals.var_t1_dn9 = assign101230_e153296_d_n9;
        locals.var_t1_dn10 = assign101230_e153296_d_n10;
        locals.var_t1_dn11 = assign101230_e153296_d_n11;
        locals.var_t1_dn14 = assign101230_e153296_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign101240_e153313, assign101240_e153313_d_n0, assign101240_e153313_d_n2, assign101240_e153313_d_n4, assign101240_e153313_d_n5, assign101240_e153313_d_n6, assign101240_e153313_d_n7, assign101240_e153313_d_n8, assign101240_e153313_d_n9, assign101240_e153313_d_n10, assign101240_e153313_d_n11, assign101240_e153313_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101240_e153300: f64 = (locals.var_ps0 - locals.var_vbscl__blk439);
        let assign101240_e153303: f64 = (locals.var_ps0 - locals.var_vbscl__blk439);
        let assign101240_e153304: f64 = (assign101240_e153300 * assign101240_e153303);
        let assign101240_e153307: f64 = (4.0 * 0.001);
        let assign101240_e153309: f64 = (assign101240_e153307 * 0.001);
        let assign101240_e153310: f64 = (assign101240_e153304 + assign101240_e153309);
        let assign101240_e153311: f64 = (assign101240_e153310).sqrt();
        (assign101240_e153311, ((((locals.var_ps0_dn0 - locals.var_vbscl__blk439_dn0) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn0 - locals.var_vbscl__blk439_dn0))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn2 - locals.var_vbscl__blk439_dn2) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn2 - locals.var_vbscl__blk439_dn2))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn4 - locals.var_vbscl__blk439_dn4) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn4 - locals.var_vbscl__blk439_dn4))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn5 - locals.var_vbscl__blk439_dn5) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn5 - locals.var_vbscl__blk439_dn5))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn6 - locals.var_vbscl__blk439_dn6) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn6 - locals.var_vbscl__blk439_dn6))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn7 - locals.var_vbscl__blk439_dn7) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn7 - locals.var_vbscl__blk439_dn7))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn8 - locals.var_vbscl__blk439_dn8) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn8 - locals.var_vbscl__blk439_dn8))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn9 - locals.var_vbscl__blk439_dn9) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn9 - locals.var_vbscl__blk439_dn9))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn10 - locals.var_vbscl__blk439_dn10) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn10 - locals.var_vbscl__blk439_dn10))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn11 - locals.var_vbscl__blk439_dn11) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn11 - locals.var_vbscl__blk439_dn11))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn14 - locals.var_vbscl__blk439_dn14) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn14 - locals.var_vbscl__blk439_dn14))) / (2.0 * assign101240_e153311)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign101240_e153313;
        locals.var_tmf2_dn0 = assign101240_e153313_d_n0;
        locals.var_tmf2_dn2 = assign101240_e153313_d_n2;
        locals.var_tmf2_dn4 = assign101240_e153313_d_n4;
        locals.var_tmf2_dn5 = assign101240_e153313_d_n5;
        locals.var_tmf2_dn6 = assign101240_e153313_d_n6;
        locals.var_tmf2_dn7 = assign101240_e153313_d_n7;
        locals.var_tmf2_dn8 = assign101240_e153313_d_n8;
        locals.var_tmf2_dn9 = assign101240_e153313_d_n9;
        locals.var_tmf2_dn10 = assign101240_e153313_d_n10;
        locals.var_tmf2_dn11 = assign101240_e153313_d_n11;
        locals.var_tmf2_dn14 = assign101240_e153313_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_389(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101250_e153325, assign101250_e153325_d_n0, assign101250_e153325_d_n2, assign101250_e153325_d_n4, assign101250_e153325_d_n5, assign101250_e153325_d_n6, assign101250_e153325_d_n7, assign101250_e153325_d_n8, assign101250_e153325_d_n9, assign101250_e153325_d_n10, assign101250_e153325_d_n11, assign101250_e153325_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101250_e153319: f64 = (locals.var_ps0 - locals.var_vbscl__blk439);
        let assign101250_e153321: f64 = (assign101250_e153319 / locals.var_tmf2);
        let assign101250_e153322: f64 = (1.0 + assign101250_e153321);
        let assign101250_e153323: f64 = (0.5 * assign101250_e153322);
        (assign101250_e153323, (0.5 * ((((locals.var_ps0_dn0 - locals.var_vbscl__blk439_dn0) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn2 - locals.var_vbscl__blk439_dn2) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn4 - locals.var_vbscl__blk439_dn4) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn5 - locals.var_vbscl__blk439_dn5) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn6 - locals.var_vbscl__blk439_dn6) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn7 - locals.var_vbscl__blk439_dn7) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn8 - locals.var_vbscl__blk439_dn8) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn9 - locals.var_vbscl__blk439_dn9) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn10 - locals.var_vbscl__blk439_dn10) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn11 - locals.var_vbscl__blk439_dn11) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn14 - locals.var_vbscl__blk439_dn14) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101250_e153325;
        locals.var_t0_dn0 = assign101250_e153325_d_n0;
        locals.var_t0_dn2 = assign101250_e153325_d_n2;
        locals.var_t0_dn4 = assign101250_e153325_d_n4;
        locals.var_t0_dn5 = assign101250_e153325_d_n5;
        locals.var_t0_dn6 = assign101250_e153325_d_n6;
        locals.var_t0_dn7 = assign101250_e153325_d_n7;
        locals.var_t0_dn8 = assign101250_e153325_d_n8;
        locals.var_t0_dn9 = assign101250_e153325_d_n9;
        locals.var_t0_dn10 = assign101250_e153325_d_n10;
        locals.var_t0_dn11 = assign101250_e153325_d_n11;
        locals.var_t0_dn14 = assign101250_e153325_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign101260_e153335, assign101260_e153335_d_n0, assign101260_e153335_d_n2, assign101260_e153335_d_n4, assign101260_e153335_d_n5, assign101260_e153335_d_n6, assign101260_e153335_d_n7, assign101260_e153335_d_n8, assign101260_e153335_d_n9, assign101260_e153335_d_n10, assign101260_e153335_d_n11, assign101260_e153335_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101260_e153330: f64 = (locals.var_ps0 - locals.var_vbscl__blk439);
        let assign101260_e153332: f64 = (assign101260_e153330 + locals.var_tmf2);
        let assign101260_e153333: f64 = (0.5 * assign101260_e153332);
        (assign101260_e153333, (0.5 * ((locals.var_ps0_dn0 - locals.var_vbscl__blk439_dn0) + locals.var_tmf2_dn0)), (0.5 * ((locals.var_ps0_dn2 - locals.var_vbscl__blk439_dn2) + locals.var_tmf2_dn2)), (0.5 * ((locals.var_ps0_dn4 - locals.var_vbscl__blk439_dn4) + locals.var_tmf2_dn4)), (0.5 * ((locals.var_ps0_dn5 - locals.var_vbscl__blk439_dn5) + locals.var_tmf2_dn5)), (0.5 * ((locals.var_ps0_dn6 - locals.var_vbscl__blk439_dn6) + locals.var_tmf2_dn6)), (0.5 * ((locals.var_ps0_dn7 - locals.var_vbscl__blk439_dn7) + locals.var_tmf2_dn7)), (0.5 * ((locals.var_ps0_dn8 - locals.var_vbscl__blk439_dn8) + locals.var_tmf2_dn8)), (0.5 * ((locals.var_ps0_dn9 - locals.var_vbscl__blk439_dn9) + locals.var_tmf2_dn9)), (0.5 * ((locals.var_ps0_dn10 - locals.var_vbscl__blk439_dn10) + locals.var_tmf2_dn10)), (0.5 * ((locals.var_ps0_dn11 - locals.var_vbscl__blk439_dn11) + locals.var_tmf2_dn11)), (0.5 * ((locals.var_ps0_dn14 - locals.var_vbscl__blk439_dn14) + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101260_e153335;
        locals.var_t5_dn0 = assign101260_e153335_d_n0;
        locals.var_t5_dn2 = assign101260_e153335_d_n2;
        locals.var_t5_dn4 = assign101260_e153335_d_n4;
        locals.var_t5_dn5 = assign101260_e153335_d_n5;
        locals.var_t5_dn6 = assign101260_e153335_d_n6;
        locals.var_t5_dn7 = assign101260_e153335_d_n7;
        locals.var_t5_dn8 = assign101260_e153335_d_n8;
        locals.var_t5_dn9 = assign101260_e153335_d_n9;
        locals.var_t5_dn10 = assign101260_e153335_d_n10;
        locals.var_t5_dn11 = assign101260_e153335_d_n11;
        locals.var_t5_dn14 = assign101260_e153335_d_n14;
        locals.var_t5_rv = 0.0;

        let assign101270_e153338: f64 = if locals.var_t5 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2327 = assign101270_e153338;
        locals.var_guard2327_rv = 0.0;

        let (assign101280_e153344, assign101280_e153344_d_n0, assign101280_e153344_d_n2, assign101280_e153344_d_n4, assign101280_e153344_d_n5, assign101280_e153344_d_n6, assign101280_e153344_d_n7, assign101280_e153344_d_n8, assign101280_e153344_d_n9, assign101280_e153344_d_n10, assign101280_e153344_d_n11, assign101280_e153344_d_n14,) = {
    if ((locals.var_guard2326 != 0.0) && (locals.var_guard2327 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101280_e153344;
        locals.var_t5_dn0 = assign101280_e153344_d_n0;
        locals.var_t5_dn2 = assign101280_e153344_d_n2;
        locals.var_t5_dn4 = assign101280_e153344_d_n4;
        locals.var_t5_dn5 = assign101280_e153344_d_n5;
        locals.var_t5_dn6 = assign101280_e153344_d_n6;
        locals.var_t5_dn7 = assign101280_e153344_d_n7;
        locals.var_t5_dn8 = assign101280_e153344_d_n8;
        locals.var_t5_dn9 = assign101280_e153344_d_n9;
        locals.var_t5_dn10 = assign101280_e153344_d_n10;
        locals.var_t5_dn11 = assign101280_e153344_d_n11;
        locals.var_t5_dn14 = assign101280_e153344_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign101290_e153350, assign101290_e153350_d_n0, assign101290_e153350_d_n2, assign101290_e153350_d_n4, assign101290_e153350_d_n5, assign101290_e153350_d_n6, assign101290_e153350_d_n7, assign101290_e153350_d_n8, assign101290_e153350_d_n9, assign101290_e153350_d_n10, assign101290_e153350_d_n11, assign101290_e153350_d_n14,) = {
    if ((locals.var_guard2326 != 0.0) && (locals.var_guard2327 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101290_e153350;
        locals.var_t0_dn0 = assign101290_e153350_d_n0;
        locals.var_t0_dn2 = assign101290_e153350_d_n2;
        locals.var_t0_dn4 = assign101290_e153350_d_n4;
        locals.var_t0_dn5 = assign101290_e153350_d_n5;
        locals.var_t0_dn6 = assign101290_e153350_d_n6;
        locals.var_t0_dn7 = assign101290_e153350_d_n7;
        locals.var_t0_dn8 = assign101290_e153350_d_n8;
        locals.var_t0_dn9 = assign101290_e153350_d_n9;
        locals.var_t0_dn10 = assign101290_e153350_d_n10;
        locals.var_t0_dn11 = assign101290_e153350_d_n11;
        locals.var_t0_dn14 = assign101290_e153350_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign101300_e153364, assign101300_e153364_d_n0, assign101300_e153364_d_n2, assign101300_e153364_d_n4, assign101300_e153364_d_n5, assign101300_e153364_d_n6, assign101300_e153364_d_n7, assign101300_e153364_d_n8, assign101300_e153364_d_n9, assign101300_e153364_d_n10, assign101300_e153364_d_n11, assign101300_e153364_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101300_e153355: f64 = (locals.var_qn0 / locals.var_t5);
        let assign101300_e153356: f64 = (locals.var_cox + assign101300_e153355);
        let assign101300_e153358: f64 = (assign101300_e153356 + locals.var_cite);
        let assign101300_e153360: f64 = (assign101300_e153358 * locals.var_beta_inv);
        let assign101300_e153362: f64 = (assign101300_e153360 / 1.6021918e-19);
        (assign101300_e153362, ((((locals.var_cox_dn0 + (((locals.var_qn0_dn0 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn0)) / 1.6021918e-19), ((((locals.var_cox_dn2 + (((locals.var_qn0_dn2 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn2)) / 1.6021918e-19), ((((locals.var_cox_dn4 + (((locals.var_qn0_dn4 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn4)) / 1.6021918e-19), ((((locals.var_cox_dn5 + (((locals.var_qn0_dn5 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn5)) / 1.6021918e-19), ((((locals.var_cox_dn6 + (((locals.var_qn0_dn6 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn6)) / 1.6021918e-19), ((((locals.var_cox_dn7 + (((locals.var_qn0_dn7 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn7)) / 1.6021918e-19), ((((locals.var_cox_dn8 + (((locals.var_qn0_dn8 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn8)) / 1.6021918e-19), ((((locals.var_cox_dn9 + (((locals.var_qn0_dn9 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn9)) / 1.6021918e-19), ((((locals.var_cox_dn10 + (((locals.var_qn0_dn10 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn10)) / 1.6021918e-19), ((((locals.var_cox_dn11 + (((locals.var_qn0_dn11 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn11)) / 1.6021918e-19), ((((locals.var_cox_dn14 + (((locals.var_qn0_dn14 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn14)) / 1.6021918e-19),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101300_e153364;
        locals.var_t2_dn0 = assign101300_e153364_d_n0;
        locals.var_t2_dn2 = assign101300_e153364_d_n2;
        locals.var_t2_dn4 = assign101300_e153364_d_n4;
        locals.var_t2_dn5 = assign101300_e153364_d_n5;
        locals.var_t2_dn6 = assign101300_e153364_d_n6;
        locals.var_t2_dn7 = assign101300_e153364_d_n7;
        locals.var_t2_dn8 = assign101300_e153364_d_n8;
        locals.var_t2_dn9 = assign101300_e153364_d_n9;
        locals.var_t2_dn10 = assign101300_e153364_d_n10;
        locals.var_t2_dn11 = assign101300_e153364_d_n11;
        locals.var_t2_dn14 = assign101300_e153364_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign101310_e153379, assign101310_e153379_d_n0, assign101310_e153379_d_n2, assign101310_e153379_d_n4, assign101310_e153379_d_n5, assign101310_e153379_d_n6, assign101310_e153379_d_n7, assign101310_e153379_d_n8, assign101310_e153379_d_n9, assign101310_e153379_d_n10, assign101310_e153379_d_n11, assign101310_e153379_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101310_e153367: f64 = (-2.0);
        let assign101310_e153369: f64 = (assign101310_e153367 * locals.var_qi_noi);
        let assign101310_e153371: f64 = (assign101310_e153369 / 1.6021918e-19);
        let assign101310_e153373: f64 = (assign101310_e153371 / locals.var_lch);
        let assign101310_e153375: f64 = (assign101310_e153373 / locals.var_weffcv_nf);
        let assign101310_e153377: f64 = (assign101310_e153375 - locals.var_t1);
        (assign101310_e153377, (((((((assign101310_e153367 * locals.var_qi_noi_dn0) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn0), (((((((assign101310_e153367 * locals.var_qi_noi_dn2) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn2), (((((((assign101310_e153367 * locals.var_qi_noi_dn4) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn4), (((((((assign101310_e153367 * locals.var_qi_noi_dn5) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn5), (((((((assign101310_e153367 * locals.var_qi_noi_dn6) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn6), (((((((assign101310_e153367 * locals.var_qi_noi_dn7) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn7), (((((((assign101310_e153367 * locals.var_qi_noi_dn8) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn8), (((((((assign101310_e153367 * locals.var_qi_noi_dn9) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn9), (((((((assign101310_e153367 * locals.var_qi_noi_dn10) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn10), (((((((assign101310_e153367 * locals.var_qi_noi_dn11) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn11), (((((((assign101310_e153367 * locals.var_qi_noi_dn14) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101310_e153379;
        locals.var_t3_dn0 = assign101310_e153379_d_n0;
        locals.var_t3_dn2 = assign101310_e153379_d_n2;
        locals.var_t3_dn4 = assign101310_e153379_d_n4;
        locals.var_t3_dn5 = assign101310_e153379_d_n5;
        locals.var_t3_dn6 = assign101310_e153379_d_n6;
        locals.var_t3_dn7 = assign101310_e153379_d_n7;
        locals.var_t3_dn8 = assign101310_e153379_d_n8;
        locals.var_t3_dn9 = assign101310_e153379_d_n9;
        locals.var_t3_dn10 = assign101310_e153379_d_n10;
        locals.var_t3_dn11 = assign101310_e153379_d_n11;
        locals.var_t3_dn14 = assign101310_e153379_d_n14;
        locals.var_t3_rv = 0.0;

        let assign101320_e153382: f64 = (locals.var_t3 - locals.var_t1);
        let assign101320_e153383: f64 = (assign101320_e153382).abs();
        let assign101320_e153386: f64 = (10.0 * 2.220446049250313e-16);
        let assign101320_e153387: f64 = if assign101320_e153383 > assign101320_e153386 { 1.0 } else { 0.0 };
        locals.var_guard2328 = assign101320_e153387;
        locals.var_guard2328_rv = 0.0;

        let (assign101330_e153434, assign101330_e153434_d_n0, assign101330_e153434_d_n2, assign101330_e153434_d_n4, assign101330_e153434_d_n5, assign101330_e153434_d_n6, assign101330_e153434_d_n7, assign101330_e153434_d_n8, assign101330_e153434_d_n9, assign101330_e153434_d_n10, assign101330_e153434_d_n11, assign101330_e153434_d_n14,) = {
    if ((locals.var_guard2326 != 0.0) && (locals.var_guard2328 != 0.0)) {
        let assign101330_e153394: f64 = (locals.var_t1 + locals.var_t2);
        let assign101330_e153395: f64 = (1.0 / assign101330_e153394);
        let assign101330_e153398: f64 = (locals.var_t3 + locals.var_t2);
        let assign101330_e153399: f64 = (assign101330_e153395 / assign101330_e153398);
        let assign101330_e153402: f64 = (2.0 * locals.var_nfalpe);
        let assign101330_e153404: f64 = (assign101330_e153402 * locals.var_ey);
        let assign101330_e153406: f64 = (assign101330_e153404 * locals.var_mu);
        let assign101330_e153409: f64 = (locals.var_t3 - locals.var_t1);
        let assign101330_e153410: f64 = (assign101330_e153406 / assign101330_e153409);
        let assign101330_e153413: f64 = (locals.var_t3 + locals.var_t2);
        let assign101330_e153416: f64 = (locals.var_t1 + locals.var_t2);
        let assign101330_e153417: f64 = (assign101330_e153413 / assign101330_e153416);
        let assign101330_e153418: f64 = (assign101330_e153417).ln();
        let assign101330_e153419: f64 = (assign101330_e153410 * assign101330_e153418);
        let assign101330_e153420: f64 = (assign101330_e153399 + assign101330_e153419);
        let assign101330_e153423: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101330_e153425: f64 = (assign101330_e153423 * locals.var_mu);
        let assign101330_e153427: f64 = (assign101330_e153425 * locals.var_nfalpe);
        let assign101330_e153429: f64 = (assign101330_e153427 * locals.var_ey);
        let assign101330_e153431: f64 = (assign101330_e153429 * locals.var_mu);
        let assign101330_e153432: f64 = (assign101330_e153420 + assign101330_e153431);
        (assign101330_e153432, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn0) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn0)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn0 - locals.var_t1_dn0))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn0 + locals.var_t2_dn0) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn0)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn2) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn2)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn2 - locals.var_t1_dn2))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn2 + locals.var_t2_dn2) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn2)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn4) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn4)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn4 - locals.var_t1_dn4))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn4 + locals.var_t2_dn4) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn4)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn5) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn5)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn5 - locals.var_t1_dn5))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn5 + locals.var_t2_dn5) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn5)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn6) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn6)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn6 - locals.var_t1_dn6))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn6 + locals.var_t2_dn6) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn6)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn7) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn7)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn7 - locals.var_t1_dn7))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn7 + locals.var_t2_dn7) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn7)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn8) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn8)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn8 - locals.var_t1_dn8))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn8 + locals.var_t2_dn8) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn8)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn9) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn9)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn9 - locals.var_t1_dn9))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn9 + locals.var_t2_dn9) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn9)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn10) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn10)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn10 - locals.var_t1_dn10))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn10 + locals.var_t2_dn10) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn10)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn11) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn11)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn11 - locals.var_t1_dn11))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn11 + locals.var_t2_dn11) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn11)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn14 + locals.var_t2_dn14) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn14 + locals.var_t2_dn14))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn14) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn14)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn14 - locals.var_t1_dn14))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn14 + locals.var_t2_dn14) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn14 + locals.var_t2_dn14))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn14) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn14)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn14)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101330_e153434;
        locals.var_t4_dn0 = assign101330_e153434_d_n0;
        locals.var_t4_dn2 = assign101330_e153434_d_n2;
        locals.var_t4_dn4 = assign101330_e153434_d_n4;
        locals.var_t4_dn5 = assign101330_e153434_d_n5;
        locals.var_t4_dn6 = assign101330_e153434_d_n6;
        locals.var_t4_dn7 = assign101330_e153434_d_n7;
        locals.var_t4_dn8 = assign101330_e153434_d_n8;
        locals.var_t4_dn9 = assign101330_e153434_d_n9;
        locals.var_t4_dn10 = assign101330_e153434_d_n10;
        locals.var_t4_dn11 = assign101330_e153434_d_n11;
        locals.var_t4_dn14 = assign101330_e153434_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign101340_e153473, assign101340_e153473_d_n0, assign101340_e153473_d_n2, assign101340_e153473_d_n4, assign101340_e153473_d_n5, assign101340_e153473_d_n6, assign101340_e153473_d_n7, assign101340_e153473_d_n8, assign101340_e153473_d_n9, assign101340_e153473_d_n10, assign101340_e153473_d_n11, assign101340_e153473_d_n14,) = {
    if ((locals.var_guard2326 != 0.0) && (locals.var_guard2328 == 0.0)) {
        let assign101340_e153442: f64 = (locals.var_t1 + locals.var_t2);
        let assign101340_e153443: f64 = (1.0 / assign101340_e153442);
        let assign101340_e153446: f64 = (locals.var_t3 + locals.var_t2);
        let assign101340_e153447: f64 = (assign101340_e153443 / assign101340_e153446);
        let assign101340_e153450: f64 = (2.0 * locals.var_nfalpe);
        let assign101340_e153452: f64 = (assign101340_e153450 * locals.var_ey);
        let assign101340_e153454: f64 = (assign101340_e153452 * locals.var_mu);
        let assign101340_e153457: f64 = (locals.var_t1 + locals.var_t2);
        let assign101340_e153458: f64 = (assign101340_e153454 / assign101340_e153457);
        let assign101340_e153459: f64 = (assign101340_e153447 + assign101340_e153458);
        let assign101340_e153462: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101340_e153464: f64 = (assign101340_e153462 * locals.var_mu);
        let assign101340_e153466: f64 = (assign101340_e153464 * locals.var_nfalpe);
        let assign101340_e153468: f64 = (assign101340_e153466 * locals.var_ey);
        let assign101340_e153470: f64 = (assign101340_e153468 * locals.var_mu);
        let assign101340_e153471: f64 = (assign101340_e153459 + assign101340_e153470);
        (assign101340_e153471, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn0) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn0)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn0)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn2) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn2)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn2)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn4) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn4)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn4)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn5) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn5)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn5)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn6) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn6)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn6)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn7) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn7)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn7)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn8) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn8)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn8)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn9) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn9)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn9)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn10) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn10)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn10)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn11) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn11)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn11)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn14 + locals.var_t2_dn14) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn14 + locals.var_t2_dn14))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn14) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn14)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn14 + locals.var_t2_dn14))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn14) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn14)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn14)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101340_e153473;
        locals.var_t4_dn0 = assign101340_e153473_d_n0;
        locals.var_t4_dn2 = assign101340_e153473_d_n2;
        locals.var_t4_dn4 = assign101340_e153473_d_n4;
        locals.var_t4_dn5 = assign101340_e153473_d_n5;
        locals.var_t4_dn6 = assign101340_e153473_d_n6;
        locals.var_t4_dn7 = assign101340_e153473_d_n7;
        locals.var_t4_dn8 = assign101340_e153473_d_n8;
        locals.var_t4_dn9 = assign101340_e153473_d_n9;
        locals.var_t4_dn10 = assign101340_e153473_d_n10;
        locals.var_t4_dn11 = assign101340_e153473_d_n11;
        locals.var_t4_dn14 = assign101340_e153473_d_n14;
        locals.var_t4_rv = 0.0;

        let assign101370_e153504: f64 = if (((p.p30 != 0.0) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2329 = assign101370_e153504;
        locals.var_guard2329_rv = 0.0;

        let (assign101380_e153516, assign101380_e153516_d_n0, assign101380_e153516_d_n2, assign101380_e153516_d_n4, assign101380_e153516_d_n5, assign101380_e153516_d_n6, assign101380_e153516_d_n7, assign101380_e153516_d_n8, assign101380_e153516_d_n9, assign101380_e153516_d_n10, assign101380_e153516_d_n11, assign101380_e153516_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101380_e153508: f64 = (locals.var_psdl - locals.var_ps0);
        let assign101380_e153511: f64 = (10.0 * 2.220446049250313e-16);
        let assign101380_e153512: f64 = (assign101380_e153508 + assign101380_e153511);
        let assign101380_e153514: f64 = (assign101380_e153512 / locals.var_lch);
        (assign101380_e153514, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn4 - locals.var_ps0_dn4) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn5 - locals.var_ps0_dn5) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn8 - locals.var_ps0_dn8) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn9 - locals.var_ps0_dn9) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn14 - locals.var_ps0_dn14) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
    }
};
        locals.var_eyd = assign101380_e153516;
        locals.var_eyd_dn0 = assign101380_e153516_d_n0;
        locals.var_eyd_dn2 = assign101380_e153516_d_n2;
        locals.var_eyd_dn4 = assign101380_e153516_d_n4;
        locals.var_eyd_dn5 = assign101380_e153516_d_n5;
        locals.var_eyd_dn6 = assign101380_e153516_d_n6;
        locals.var_eyd_dn7 = assign101380_e153516_d_n7;
        locals.var_eyd_dn8 = assign101380_e153516_d_n8;
        locals.var_eyd_dn9 = assign101380_e153516_d_n9;
        locals.var_eyd_dn10 = assign101380_e153516_d_n10;
        locals.var_eyd_dn11 = assign101380_e153516_d_n11;
        locals.var_eyd_dn14 = assign101380_e153516_d_n14;
        locals.var_eyd_rv = 0.0;

        let (assign101390_e153525, assign101390_e153525_d_n0, assign101390_e153525_d_n2, assign101390_e153525_d_n4, assign101390_e153525_d_n5, assign101390_e153525_d_n6, assign101390_e153525_d_n7, assign101390_e153525_d_n8, assign101390_e153525_d_n9, assign101390_e153525_d_n10, assign101390_e153525_d_n11, assign101390_e153525_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let (assign101390_e153523, assign101390_e153523_d_n0, assign101390_e153523_d_n2, assign101390_e153523_d_n4, assign101390_e153523_d_n5, assign101390_e153523_d_n6, assign101390_e153523_d_n7, assign101390_e153523_d_n8, assign101390_e153523_d_n9, assign101390_e153523_d_n10, assign101390_e153523_d_n11, assign101390_e153523_d_n14,) = {
            if (locals.var_eyd >= 0.0) {
                (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign101390_e153523, assign101390_e153523_d_n0, assign101390_e153523_d_n2, assign101390_e153523_d_n4, assign101390_e153523_d_n5, assign101390_e153523_d_n6, assign101390_e153523_d_n7, assign101390_e153523_d_n8, assign101390_e153523_d_n9, assign101390_e153523_d_n10, assign101390_e153523_d_n11, assign101390_e153523_d_n14,)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
    }
};
        locals.var_eyd = assign101390_e153525;
        locals.var_eyd_dn0 = assign101390_e153525_d_n0;
        locals.var_eyd_dn2 = assign101390_e153525_d_n2;
        locals.var_eyd_dn4 = assign101390_e153525_d_n4;
        locals.var_eyd_dn5 = assign101390_e153525_d_n5;
        locals.var_eyd_dn6 = assign101390_e153525_d_n6;
        locals.var_eyd_dn7 = assign101390_e153525_d_n7;
        locals.var_eyd_dn8 = assign101390_e153525_d_n8;
        locals.var_eyd_dn9 = assign101390_e153525_d_n9;
        locals.var_eyd_dn10 = assign101390_e153525_d_n10;
        locals.var_eyd_dn11 = assign101390_e153525_d_n11;
        locals.var_eyd_dn14 = assign101390_e153525_d_n14;
        locals.var_eyd_rv = 0.0;

        let (assign101400_e153533, assign101400_e153533_d_n0, assign101400_e153533_d_n2, assign101400_e153533_d_n4, assign101400_e153533_d_n5, assign101400_e153533_d_n6, assign101400_e153533_d_n7, assign101400_e153533_d_n8, assign101400_e153533_d_n9, assign101400_e153533_d_n10, assign101400_e153533_d_n11, assign101400_e153533_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101400_e153529: f64 = (locals.var_muun * locals.var_eyd);
        let assign101400_e153531: f64 = (assign101400_e153529 / 10000000.0);
        (assign101400_e153531, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 10000000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 10000000.0), (((locals.var_muun_dn4 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn4)) / 10000000.0), (((locals.var_muun_dn5 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn5)) / 10000000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 10000000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 10000000.0), (((locals.var_muun_dn8 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn8)) / 10000000.0), (((locals.var_muun_dn9 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn9)) / 10000000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 10000000.0), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / 10000000.0), (((locals.var_muun_dn14 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn14)) / 10000000.0),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign101400_e153533;
        locals.var_t12_dn0 = assign101400_e153533_d_n0;
        locals.var_t12_dn2 = assign101400_e153533_d_n2;
        locals.var_t12_dn4 = assign101400_e153533_d_n4;
        locals.var_t12_dn5 = assign101400_e153533_d_n5;
        locals.var_t12_dn6 = assign101400_e153533_d_n6;
        locals.var_t12_dn7 = assign101400_e153533_d_n7;
        locals.var_t12_dn8 = assign101400_e153533_d_n8;
        locals.var_t12_dn9 = assign101400_e153533_d_n9;
        locals.var_t12_dn10 = assign101400_e153533_d_n10;
        locals.var_t12_dn11 = assign101400_e153533_d_n11;
        locals.var_t12_dn14 = assign101400_e153533_d_n14;
        locals.var_t12_rv = 0.0;

        let assign101410_e153537: f64 = (10.0 * 2.220446049250313e-16);
        let assign101410_e153538: f64 = (1.0 - assign101410_e153537);
        let assign101410_e153545: f64 = (10.0 * 2.220446049250313e-16);
        let assign101410_e153546: f64 = (1.0 + assign101410_e153545);
        let assign101410_e153548: f64 = if ((assign101410_e153538 <= p.p178) && (p.p178 <= assign101410_e153546)) { 1.0 } else { 0.0 };
        locals.var_guard2330 = assign101410_e153548;
        locals.var_guard2330_rv = 0.0;

        let (assign101420_e153554, assign101420_e153554_d_n0, assign101420_e153554_d_n2, assign101420_e153554_d_n4, assign101420_e153554_d_n5, assign101420_e153554_d_n6, assign101420_e153554_d_n7, assign101420_e153554_d_n8, assign101420_e153554_d_n9, assign101420_e153554_d_n10, assign101420_e153554_d_n11, assign101420_e153554_d_n14,) = {
    if ((locals.var_guard2329 != 0.0) && (locals.var_guard2330 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101420_e153554;
        locals.var_t7_dn0 = assign101420_e153554_d_n0;
        locals.var_t7_dn2 = assign101420_e153554_d_n2;
        locals.var_t7_dn4 = assign101420_e153554_d_n4;
        locals.var_t7_dn5 = assign101420_e153554_d_n5;
        locals.var_t7_dn6 = assign101420_e153554_d_n6;
        locals.var_t7_dn7 = assign101420_e153554_d_n7;
        locals.var_t7_dn8 = assign101420_e153554_d_n8;
        locals.var_t7_dn9 = assign101420_e153554_d_n9;
        locals.var_t7_dn10 = assign101420_e153554_d_n10;
        locals.var_t7_dn11 = assign101420_e153554_d_n11;
        locals.var_t7_dn14 = assign101420_e153554_d_n14;
        locals.var_t7_rv = 0.0;

        let assign101430_e153558: f64 = (10.0 * 2.220446049250313e-16);
        let assign101430_e153559: f64 = (2.0 - assign101430_e153558);
        let assign101430_e153566: f64 = (10.0 * 2.220446049250313e-16);
        let assign101430_e153567: f64 = (2.0 + assign101430_e153566);
        let assign101430_e153569: f64 = if ((assign101430_e153559 <= p.p178) && (p.p178 <= assign101430_e153567)) { 1.0 } else { 0.0 };
        locals.var_guard2331 = assign101430_e153569;
        locals.var_guard2331_rv = 0.0;

        let (assign101440_e153578, assign101440_e153578_d_n0, assign101440_e153578_d_n2, assign101440_e153578_d_n4, assign101440_e153578_d_n5, assign101440_e153578_d_n6, assign101440_e153578_d_n7, assign101440_e153578_d_n8, assign101440_e153578_d_n9, assign101440_e153578_d_n10, assign101440_e153578_d_n11, assign101440_e153578_d_n14,) = {
    if (((locals.var_guard2329 != 0.0) && (locals.var_guard2330 == 0.0)) && (locals.var_guard2331 != 0.0)) {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101440_e153578;
        locals.var_t7_dn0 = assign101440_e153578_d_n0;
        locals.var_t7_dn2 = assign101440_e153578_d_n2;
        locals.var_t7_dn4 = assign101440_e153578_d_n4;
        locals.var_t7_dn5 = assign101440_e153578_d_n5;
        locals.var_t7_dn6 = assign101440_e153578_d_n6;
        locals.var_t7_dn7 = assign101440_e153578_d_n7;
        locals.var_t7_dn8 = assign101440_e153578_d_n8;
        locals.var_t7_dn9 = assign101440_e153578_d_n9;
        locals.var_t7_dn10 = assign101440_e153578_d_n10;
        locals.var_t7_dn11 = assign101440_e153578_d_n11;
        locals.var_t7_dn14 = assign101440_e153578_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign101450_e153597, assign101450_e153597_d_n0, assign101450_e153597_d_n2, assign101450_e153597_d_n4, assign101450_e153597_d_n5, assign101450_e153597_d_n6, assign101450_e153597_d_n7, assign101450_e153597_d_n8, assign101450_e153597_d_n9, assign101450_e153597_d_n10, assign101450_e153597_d_n11, assign101450_e153597_d_n14,) = {
    if (((locals.var_guard2329 != 0.0) && (locals.var_guard2330 == 0.0)) && (locals.var_guard2331 == 0.0)) {
        let (assign101450_e153595, assign101450_e153595_d_n0, assign101450_e153595_d_n2, assign101450_e153595_d_n4, assign101450_e153595_d_n5, assign101450_e153595_d_n6, assign101450_e153595_d_n7, assign101450_e153595_d_n8, assign101450_e153595_d_n9, assign101450_e153595_d_n10, assign101450_e153595_d_n11, assign101450_e153595_d_n14,) = {
            if (locals.var_eyd == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101450_e153593: f64 = (p.p178 - 1.0);
                let assign101450_e153594: f64 = (locals.var_eyd).powf(assign101450_e153593);
                (assign101450_e153594, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn0)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn0 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn2)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn2 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn4)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn4 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn5)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn5 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn6)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn6 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn7)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn7 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn8)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn8 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn9)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn9 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn10)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn10 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn11)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn11 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn14)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn14 / locals.var_eyd))) },)
            }
        };
        (assign101450_e153595, assign101450_e153595_d_n0, assign101450_e153595_d_n2, assign101450_e153595_d_n4, assign101450_e153595_d_n5, assign101450_e153595_d_n6, assign101450_e153595_d_n7, assign101450_e153595_d_n8, assign101450_e153595_d_n9, assign101450_e153595_d_n10, assign101450_e153595_d_n11, assign101450_e153595_d_n14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101450_e153597;
        locals.var_t7_dn0 = assign101450_e153597_d_n0;
        locals.var_t7_dn2 = assign101450_e153597_d_n2;
        locals.var_t7_dn4 = assign101450_e153597_d_n4;
        locals.var_t7_dn5 = assign101450_e153597_d_n5;
        locals.var_t7_dn6 = assign101450_e153597_d_n6;
        locals.var_t7_dn7 = assign101450_e153597_d_n7;
        locals.var_t7_dn8 = assign101450_e153597_d_n8;
        locals.var_t7_dn9 = assign101450_e153597_d_n9;
        locals.var_t7_dn10 = assign101450_e153597_d_n10;
        locals.var_t7_dn11 = assign101450_e153597_d_n11;
        locals.var_t7_dn14 = assign101450_e153597_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign101460_e153603, assign101460_e153603_d_n0, assign101460_e153603_d_n2, assign101460_e153603_d_n4, assign101460_e153603_d_n5, assign101460_e153603_d_n6, assign101460_e153603_d_n7, assign101460_e153603_d_n8, assign101460_e153603_d_n9, assign101460_e153603_d_n10, assign101460_e153603_d_n11, assign101460_e153603_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101460_e153601: f64 = (locals.var_t12 * locals.var_t7);
        (assign101460_e153601, ((locals.var_t12_dn0 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn0)), ((locals.var_t12_dn2 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn2)), ((locals.var_t12_dn4 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn4)), ((locals.var_t12_dn5 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn5)), ((locals.var_t12_dn6 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn6)), ((locals.var_t12_dn7 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn7)), ((locals.var_t12_dn8 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn8)), ((locals.var_t12_dn9 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn9)), ((locals.var_t12_dn10 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn10)), ((locals.var_t12_dn11 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn11)), ((locals.var_t12_dn14 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign101460_e153603;
        locals.var_t8_dn0 = assign101460_e153603_d_n0;
        locals.var_t8_dn2 = assign101460_e153603_d_n2;
        locals.var_t8_dn4 = assign101460_e153603_d_n4;
        locals.var_t8_dn5 = assign101460_e153603_d_n5;
        locals.var_t8_dn6 = assign101460_e153603_d_n6;
        locals.var_t8_dn7 = assign101460_e153603_d_n7;
        locals.var_t8_dn8 = assign101460_e153603_d_n8;
        locals.var_t8_dn9 = assign101460_e153603_d_n9;
        locals.var_t8_dn10 = assign101460_e153603_d_n10;
        locals.var_t8_dn11 = assign101460_e153603_d_n11;
        locals.var_t8_dn14 = assign101460_e153603_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign101470_e153609, assign101470_e153609_d_n0, assign101470_e153609_d_n2, assign101470_e153609_d_n4, assign101470_e153609_d_n5, assign101470_e153609_d_n6, assign101470_e153609_d_n7, assign101470_e153609_d_n8, assign101470_e153609_d_n9, assign101470_e153609_d_n10, assign101470_e153609_d_n11, assign101470_e153609_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101470_e153607: f64 = (1.0 + locals.var_t8);
        (assign101470_e153607, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign101470_e153609;
        locals.var_t9_dn0 = assign101470_e153609_d_n0;
        locals.var_t9_dn2 = assign101470_e153609_d_n2;
        locals.var_t9_dn4 = assign101470_e153609_d_n4;
        locals.var_t9_dn5 = assign101470_e153609_d_n5;
        locals.var_t9_dn6 = assign101470_e153609_d_n6;
        locals.var_t9_dn7 = assign101470_e153609_d_n7;
        locals.var_t9_dn8 = assign101470_e153609_d_n8;
        locals.var_t9_dn9 = assign101470_e153609_d_n9;
        locals.var_t9_dn10 = assign101470_e153609_d_n10;
        locals.var_t9_dn11 = assign101470_e153609_d_n11;
        locals.var_t9_dn14 = assign101470_e153609_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign101480_e153625, assign101480_e153625_d_n0, assign101480_e153625_d_n2, assign101480_e153625_d_n4, assign101480_e153625_d_n5, assign101480_e153625_d_n6, assign101480_e153625_d_n7, assign101480_e153625_d_n8, assign101480_e153625_d_n9, assign101480_e153625_d_n10, assign101480_e153625_d_n11, assign101480_e153625_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let (assign101480_e153623, assign101480_e153623_d_n0, assign101480_e153623_d_n2, assign101480_e153623_d_n4, assign101480_e153623_d_n5, assign101480_e153623_d_n6, assign101480_e153623_d_n7, assign101480_e153623_d_n8, assign101480_e153623_d_n9, assign101480_e153623_d_n10, assign101480_e153623_d_n11, assign101480_e153623_d_n14,) = {
            if (locals.var_t9 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101480_e153617: f64 = (-1.0);
                let assign101480_e153619: f64 = (assign101480_e153617 / p.p178);
                let assign101480_e153621: f64 = (assign101480_e153619 - 1.0);
                let assign101480_e153622: f64 = (locals.var_t9).powf(assign101480_e153621);
                (assign101480_e153622, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn0)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn0 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn2)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn2 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn4)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn4 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn5)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn5 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn6)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn6 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn7)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn7 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn8)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn8 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn9)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn9 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn10)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn10 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn11)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn11 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn14)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn14 / locals.var_t9))) },)
            }
        };
        (assign101480_e153623, assign101480_e153623_d_n0, assign101480_e153623_d_n2, assign101480_e153623_d_n4, assign101480_e153623_d_n5, assign101480_e153623_d_n6, assign101480_e153623_d_n7, assign101480_e153623_d_n8, assign101480_e153623_d_n9, assign101480_e153623_d_n10, assign101480_e153623_d_n11, assign101480_e153623_d_n14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101480_e153625;
        locals.var_t10_dn0 = assign101480_e153625_d_n0;
        locals.var_t10_dn2 = assign101480_e153625_d_n2;
        locals.var_t10_dn4 = assign101480_e153625_d_n4;
        locals.var_t10_dn5 = assign101480_e153625_d_n5;
        locals.var_t10_dn6 = assign101480_e153625_d_n6;
        locals.var_t10_dn7 = assign101480_e153625_d_n7;
        locals.var_t10_dn8 = assign101480_e153625_d_n8;
        locals.var_t10_dn9 = assign101480_e153625_d_n9;
        locals.var_t10_dn10 = assign101480_e153625_d_n10;
        locals.var_t10_dn11 = assign101480_e153625_d_n11;
        locals.var_t10_dn14 = assign101480_e153625_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign101490_e153631, assign101490_e153631_d_n0, assign101490_e153631_d_n2, assign101490_e153631_d_n4, assign101490_e153631_d_n5, assign101490_e153631_d_n6, assign101490_e153631_d_n7, assign101490_e153631_d_n8, assign101490_e153631_d_n9, assign101490_e153631_d_n10, assign101490_e153631_d_n11, assign101490_e153631_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101490_e153629: f64 = (locals.var_t9 * locals.var_t10);
        (assign101490_e153629, ((locals.var_t9_dn0 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn0)), ((locals.var_t9_dn2 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn2)), ((locals.var_t9_dn4 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn4)), ((locals.var_t9_dn5 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn5)), ((locals.var_t9_dn6 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn6)), ((locals.var_t9_dn7 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn7)), ((locals.var_t9_dn8 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn8)), ((locals.var_t9_dn9 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn9)), ((locals.var_t9_dn10 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn10)), ((locals.var_t9_dn11 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn11)), ((locals.var_t9_dn14 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign101490_e153631;
        locals.var_t11_dn0 = assign101490_e153631_d_n0;
        locals.var_t11_dn2 = assign101490_e153631_d_n2;
        locals.var_t11_dn4 = assign101490_e153631_d_n4;
        locals.var_t11_dn5 = assign101490_e153631_d_n5;
        locals.var_t11_dn6 = assign101490_e153631_d_n6;
        locals.var_t11_dn7 = assign101490_e153631_d_n7;
        locals.var_t11_dn8 = assign101490_e153631_d_n8;
        locals.var_t11_dn9 = assign101490_e153631_d_n9;
        locals.var_t11_dn10 = assign101490_e153631_d_n10;
        locals.var_t11_dn11 = assign101490_e153631_d_n11;
        locals.var_t11_dn14 = assign101490_e153631_d_n14;
        locals.var_t11_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_390(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101500_e153637, assign101500_e153637_d_n0, assign101500_e153637_d_n2, assign101500_e153637_d_n4, assign101500_e153637_d_n5, assign101500_e153637_d_n6, assign101500_e153637_d_n7, assign101500_e153637_d_n8, assign101500_e153637_d_n9, assign101500_e153637_d_n10, assign101500_e153637_d_n11, assign101500_e153637_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101500_e153635: f64 = (locals.var_muun * locals.var_t11);
        (assign101500_e153635, ((locals.var_muun_dn0 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn0)), ((locals.var_muun_dn2 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn2)), ((locals.var_muun_dn4 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn4)), ((locals.var_muun_dn5 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn5)), ((locals.var_muun_dn6 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn6)), ((locals.var_muun_dn7 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn7)), ((locals.var_muun_dn8 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn8)), ((locals.var_muun_dn9 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn9)), ((locals.var_muun_dn10 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn10)), ((locals.var_muun_dn11 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn11)), ((locals.var_muun_dn14 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn14)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn4, locals.var_mud_hoso_dn5, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn8, locals.var_mud_hoso_dn9, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn14,)
    }
};
        locals.var_mud_hoso = assign101500_e153637;
        locals.var_mud_hoso_dn0 = assign101500_e153637_d_n0;
        locals.var_mud_hoso_dn2 = assign101500_e153637_d_n2;
        locals.var_mud_hoso_dn4 = assign101500_e153637_d_n4;
        locals.var_mud_hoso_dn5 = assign101500_e153637_d_n5;
        locals.var_mud_hoso_dn6 = assign101500_e153637_d_n6;
        locals.var_mud_hoso_dn7 = assign101500_e153637_d_n7;
        locals.var_mud_hoso_dn8 = assign101500_e153637_d_n8;
        locals.var_mud_hoso_dn9 = assign101500_e153637_d_n9;
        locals.var_mud_hoso_dn10 = assign101500_e153637_d_n10;
        locals.var_mud_hoso_dn11 = assign101500_e153637_d_n11;
        locals.var_mud_hoso_dn14 = assign101500_e153637_d_n14;
        locals.var_mud_hoso_rv = 0.0;

        let (assign101510_e153645, assign101510_e153645_d_n0, assign101510_e153645_d_n2, assign101510_e153645_d_n4, assign101510_e153645_d_n5, assign101510_e153645_d_n6, assign101510_e153645_d_n7, assign101510_e153645_d_n8, assign101510_e153645_d_n9, assign101510_e153645_d_n10, assign101510_e153645_d_n11, assign101510_e153645_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101510_e153641: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign101510_e153643: f64 = (assign101510_e153641 / 2.0);
        (assign101510_e153643, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn4 + locals.var_mud_hoso_dn4) / 2.0), ((locals.var_mu_dn5 + locals.var_mud_hoso_dn5) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn8 + locals.var_mud_hoso_dn8) / 2.0), ((locals.var_mu_dn9 + locals.var_mud_hoso_dn9) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn14 + locals.var_mud_hoso_dn14) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn4, locals.var_mu_ave_dn5, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn8, locals.var_mu_ave_dn9, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn14,)
    }
};
        locals.var_mu_ave = assign101510_e153645;
        locals.var_mu_ave_dn0 = assign101510_e153645_d_n0;
        locals.var_mu_ave_dn2 = assign101510_e153645_d_n2;
        locals.var_mu_ave_dn4 = assign101510_e153645_d_n4;
        locals.var_mu_ave_dn5 = assign101510_e153645_d_n5;
        locals.var_mu_ave_dn6 = assign101510_e153645_d_n6;
        locals.var_mu_ave_dn7 = assign101510_e153645_d_n7;
        locals.var_mu_ave_dn8 = assign101510_e153645_d_n8;
        locals.var_mu_ave_dn9 = assign101510_e153645_d_n9;
        locals.var_mu_ave_dn10 = assign101510_e153645_d_n10;
        locals.var_mu_ave_dn11 = assign101510_e153645_d_n11;
        locals.var_mu_ave_dn14 = assign101510_e153645_d_n14;
        locals.var_mu_ave_rv = 0.0;

        let (assign101520_e153651, assign101520_e153651_d_n0, assign101520_e153651_d_n2, assign101520_e153651_d_n4, assign101520_e153651_d_n5, assign101520_e153651_d_n6, assign101520_e153651_d_n7, assign101520_e153651_d_n8, assign101520_e153651_d_n9, assign101520_e153651_d_n10, assign101520_e153651_d_n11, assign101520_e153651_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101520_e153649: f64 = (locals.var_alpha * locals.var_alpha);
        (assign101520_e153649, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn4 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn4)), ((locals.var_alpha_dn5 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn5)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn8 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn8)), ((locals.var_alpha_dn9 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn9)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn14 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101520_e153651;
        locals.var_t0_dn0 = assign101520_e153651_d_n0;
        locals.var_t0_dn2 = assign101520_e153651_d_n2;
        locals.var_t0_dn4 = assign101520_e153651_d_n4;
        locals.var_t0_dn5 = assign101520_e153651_d_n5;
        locals.var_t0_dn6 = assign101520_e153651_d_n6;
        locals.var_t0_dn7 = assign101520_e153651_d_n7;
        locals.var_t0_dn8 = assign101520_e153651_d_n8;
        locals.var_t0_dn9 = assign101520_e153651_d_n9;
        locals.var_t0_dn10 = assign101520_e153651_d_n10;
        locals.var_t0_dn11 = assign101520_e153651_d_n11;
        locals.var_t0_dn14 = assign101520_e153651_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign101530_e153713, assign101530_e153713_d_n0, assign101530_e153713_d_n2, assign101530_e153713_d_n4, assign101530_e153713_d_n5, assign101530_e153713_d_n6, assign101530_e153713_d_n7, assign101530_e153713_d_n8, assign101530_e153713_d_n9, assign101530_e153713_d_n10, assign101530_e153713_d_n11, assign101530_e153713_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101530_e153655: f64 = (locals.var_weff_nf * locals.var_cox);
        let assign101530_e153657: f64 = (assign101530_e153655 * locals.var_vgvt);
        let assign101530_e153659: f64 = (assign101530_e153657 * locals.var_mu);
        let assign101530_e153663: f64 = (3.0 * locals.var_alpha);
        let assign101530_e153664: f64 = (1.0 + assign101530_e153663);
        let assign101530_e153667: f64 = (6.0 * locals.var_t0);
        let assign101530_e153668: f64 = (assign101530_e153664 + assign101530_e153667);
        let assign101530_e153670: f64 = (assign101530_e153668 * locals.var_mud_hoso);
        let assign101530_e153672: f64 = (assign101530_e153670 * locals.var_mud_hoso);
        let assign101530_e153676: f64 = (4.0 * locals.var_alpha);
        let assign101530_e153677: f64 = (3.0 + assign101530_e153676);
        let assign101530_e153680: f64 = (3.0 * locals.var_t0);
        let assign101530_e153681: f64 = (assign101530_e153677 + assign101530_e153680);
        let assign101530_e153683: f64 = (assign101530_e153681 * locals.var_mud_hoso);
        let assign101530_e153685: f64 = (assign101530_e153683 * locals.var_mu);
        let assign101530_e153686: f64 = (assign101530_e153672 + assign101530_e153685);
        let assign101530_e153690: f64 = (3.0 * locals.var_alpha);
        let assign101530_e153691: f64 = (6.0 + assign101530_e153690);
        let assign101530_e153693: f64 = (assign101530_e153691 + locals.var_t0);
        let assign101530_e153695: f64 = (assign101530_e153693 * locals.var_mu);
        let assign101530_e153697: f64 = (assign101530_e153695 * locals.var_mu);
        let assign101530_e153698: f64 = (assign101530_e153686 + assign101530_e153697);
        let assign101530_e153699: f64 = (assign101530_e153659 * assign101530_e153698);
        let assign101530_e153702: f64 = (15.0 * locals.var_lch);
        let assign101530_e153705: f64 = (1.0 + locals.var_alpha);
        let assign101530_e153706: f64 = (assign101530_e153702 * assign101530_e153705);
        let assign101530_e153708: f64 = (assign101530_e153706 * locals.var_mu_ave);
        let assign101530_e153710: f64 = (assign101530_e153708 * locals.var_mu_ave);
        let assign101530_e153711: f64 = (assign101530_e153699 / assign101530_e153710);
        (assign101530_e153711, ((((((((((locals.var_weff_nf * locals.var_cox_dn0) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn0)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0_dn0) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn0)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn0))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn0) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn0)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn2) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn2)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0_dn2) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn2)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn2))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn2) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn2)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn4) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn4)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn4)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn4) + (6.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn4)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn4)) + ((((((4.0 * locals.var_alpha_dn4) + (3.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn4)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn4))) + ((((((3.0 * locals.var_alpha_dn4) + locals.var_t0_dn4) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn4)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn4))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn4) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn4)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn4)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn4)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn5) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn5)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn5)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn5) + (6.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn5)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn5)) + ((((((4.0 * locals.var_alpha_dn5) + (3.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn5)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn5))) + ((((((3.0 * locals.var_alpha_dn5) + locals.var_t0_dn5) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn5)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn5))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn5) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn5)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn5)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn5)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn6) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn6)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0_dn6) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn6)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn6))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn6) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn6)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn7) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn7)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0_dn7) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn7)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn7))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn7) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn7)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn8) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn8)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn8)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn8) + (6.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn8)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn8)) + ((((((4.0 * locals.var_alpha_dn8) + (3.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn8)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn8))) + ((((((3.0 * locals.var_alpha_dn8) + locals.var_t0_dn8) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn8)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn8))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn8) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn8)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn8)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn8)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn9) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn9)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn9)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn9) + (6.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn9)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn9)) + ((((((4.0 * locals.var_alpha_dn9) + (3.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn9)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn9))) + ((((((3.0 * locals.var_alpha_dn9) + locals.var_t0_dn9) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn9)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn9))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn9) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn9)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn9)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn9)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn10) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn10)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0_dn10) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn10)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn10))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn10) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn10)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn11) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn11)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0_dn11) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn11)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn11))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn11) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn11)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn14) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn14)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn14)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn14) + (6.0 * locals.var_t0_dn14)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn14)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn14)) + ((((((4.0 * locals.var_alpha_dn14) + (3.0 * locals.var_t0_dn14)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn14)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn14))) + ((((((3.0 * locals.var_alpha_dn14) + locals.var_t0_dn14) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn14)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn14))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn14) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn14)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn14)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn14)))) / (assign101530_e153710 * assign101530_e153710)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn14,)
    }
};
        locals.var_nthrml = assign101530_e153713;
        locals.var_nthrml_dn0 = assign101530_e153713_d_n0;
        locals.var_nthrml_dn2 = assign101530_e153713_d_n2;
        locals.var_nthrml_dn4 = assign101530_e153713_d_n4;
        locals.var_nthrml_dn5 = assign101530_e153713_d_n5;
        locals.var_nthrml_dn6 = assign101530_e153713_d_n6;
        locals.var_nthrml_dn7 = assign101530_e153713_d_n7;
        locals.var_nthrml_dn8 = assign101530_e153713_d_n8;
        locals.var_nthrml_dn9 = assign101530_e153713_d_n9;
        locals.var_nthrml_dn10 = assign101530_e153713_d_n10;
        locals.var_nthrml_dn11 = assign101530_e153713_d_n11;
        locals.var_nthrml_dn14 = assign101530_e153713_d_n14;
        locals.var_nthrml_rv = 0.0;

        let (assign101540_e153718, assign101540_e153718_d_n0, assign101540_e153718_d_n2, assign101540_e153718_d_n4, assign101540_e153718_d_n5, assign101540_e153718_d_n6, assign101540_e153718_d_n7, assign101540_e153718_d_n8, assign101540_e153718_d_n9, assign101540_e153718_d_n10, assign101540_e153718_d_n11, assign101540_e153718_d_n14,) = {
    if (locals.var_guard2329 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn14,)
    }
};
        locals.var_nthrml = assign101540_e153718;
        locals.var_nthrml_dn0 = assign101540_e153718_d_n0;
        locals.var_nthrml_dn2 = assign101540_e153718_d_n2;
        locals.var_nthrml_dn4 = assign101540_e153718_d_n4;
        locals.var_nthrml_dn5 = assign101540_e153718_d_n5;
        locals.var_nthrml_dn6 = assign101540_e153718_d_n6;
        locals.var_nthrml_dn7 = assign101540_e153718_d_n7;
        locals.var_nthrml_dn8 = assign101540_e153718_d_n8;
        locals.var_nthrml_dn9 = assign101540_e153718_d_n9;
        locals.var_nthrml_dn10 = assign101540_e153718_d_n10;
        locals.var_nthrml_dn11 = assign101540_e153718_d_n11;
        locals.var_nthrml_dn14 = assign101540_e153718_d_n14;
        locals.var_nthrml_rv = 0.0;

        let assign101550_e153736: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2332 = assign101550_e153736;
        locals.var_guard2332_rv = 0.0;

        let (assign101560_e153741, assign101560_e153741_d_n0, assign101560_e153741_d_n2, assign101560_e153741_d_n4, assign101560_e153741_d_n5, assign101560_e153741_d_n6, assign101560_e153741_d_n7, assign101560_e153741_d_n8, assign101560_e153741_d_n9, assign101560_e153741_d_n10, assign101560_e153741_d_n11, assign101560_e153741_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101560_e153739: f64 = (locals.var_kusail).sqrt();
        (assign101560_e153739, (locals.var_kusail_dn0 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn2 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn4 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn5 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn6 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn7 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn8 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn9 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn10 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn11 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn14 / (2.0 * assign101560_e153739)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn4, locals.var_sqrtkusail_dn5, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn8, locals.var_sqrtkusail_dn9, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn14,)
    }
};
        locals.var_sqrtkusail = assign101560_e153741;
        locals.var_sqrtkusail_dn0 = assign101560_e153741_d_n0;
        locals.var_sqrtkusail_dn2 = assign101560_e153741_d_n2;
        locals.var_sqrtkusail_dn4 = assign101560_e153741_d_n4;
        locals.var_sqrtkusail_dn5 = assign101560_e153741_d_n5;
        locals.var_sqrtkusail_dn6 = assign101560_e153741_d_n6;
        locals.var_sqrtkusail_dn7 = assign101560_e153741_d_n7;
        locals.var_sqrtkusail_dn8 = assign101560_e153741_d_n8;
        locals.var_sqrtkusail_dn9 = assign101560_e153741_d_n9;
        locals.var_sqrtkusail_dn10 = assign101560_e153741_d_n10;
        locals.var_sqrtkusail_dn11 = assign101560_e153741_d_n11;
        locals.var_sqrtkusail_dn14 = assign101560_e153741_d_n14;
        locals.var_sqrtkusail_rv = 0.0;

        let (assign101570_e153747, assign101570_e153747_d_n0, assign101570_e153747_d_n2, assign101570_e153747_d_n4, assign101570_e153747_d_n5, assign101570_e153747_d_n6, assign101570_e153747_d_n7, assign101570_e153747_d_n8, assign101570_e153747_d_n9, assign101570_e153747_d_n10, assign101570_e153747_d_n11, assign101570_e153747_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101570_e153745: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign101570_e153745, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4), (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8), (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn14 + locals.var_sqrtkusail_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101570_e153747;
        locals.var_t2_dn0 = assign101570_e153747_d_n0;
        locals.var_t2_dn2 = assign101570_e153747_d_n2;
        locals.var_t2_dn4 = assign101570_e153747_d_n4;
        locals.var_t2_dn5 = assign101570_e153747_d_n5;
        locals.var_t2_dn6 = assign101570_e153747_d_n6;
        locals.var_t2_dn7 = assign101570_e153747_d_n7;
        locals.var_t2_dn8 = assign101570_e153747_d_n8;
        locals.var_t2_dn9 = assign101570_e153747_d_n9;
        locals.var_t2_dn10 = assign101570_e153747_d_n10;
        locals.var_t2_dn11 = assign101570_e153747_d_n11;
        locals.var_t2_dn14 = assign101570_e153747_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign101580_e153753, assign101580_e153753_d_n0, assign101580_e153753_d_n2, assign101580_e153753_d_n4, assign101580_e153753_d_n5, assign101580_e153753_d_n6, assign101580_e153753_d_n7, assign101580_e153753_d_n8, assign101580_e153753_d_n9, assign101580_e153753_d_n10, assign101580_e153753_d_n11, assign101580_e153753_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101580_e153751: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign101580_e153751, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)), ((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)), ((locals.var_kusai00_dn9 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn9)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn14 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101580_e153753;
        locals.var_t3_dn0 = assign101580_e153753_d_n0;
        locals.var_t3_dn2 = assign101580_e153753_d_n2;
        locals.var_t3_dn4 = assign101580_e153753_d_n4;
        locals.var_t3_dn5 = assign101580_e153753_d_n5;
        locals.var_t3_dn6 = assign101580_e153753_d_n6;
        locals.var_t3_dn7 = assign101580_e153753_d_n7;
        locals.var_t3_dn8 = assign101580_e153753_d_n8;
        locals.var_t3_dn9 = assign101580_e153753_d_n9;
        locals.var_t3_dn10 = assign101580_e153753_d_n10;
        locals.var_t3_dn11 = assign101580_e153753_d_n11;
        locals.var_t3_dn14 = assign101580_e153753_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign101590_e153759, assign101590_e153759_d_n0, assign101590_e153759_d_n2, assign101590_e153759_d_n4, assign101590_e153759_d_n5, assign101590_e153759_d_n6, assign101590_e153759_d_n7, assign101590_e153759_d_n8, assign101590_e153759_d_n9, assign101590_e153759_d_n10, assign101590_e153759_d_n11, assign101590_e153759_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101590_e153757: f64 = (locals.var_kusail * locals.var_kusail);
        (assign101590_e153757, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)), ((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)), ((locals.var_kusail_dn9 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn9)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn14 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101590_e153759;
        locals.var_t4_dn0 = assign101590_e153759_d_n0;
        locals.var_t4_dn2 = assign101590_e153759_d_n2;
        locals.var_t4_dn4 = assign101590_e153759_d_n4;
        locals.var_t4_dn5 = assign101590_e153759_d_n5;
        locals.var_t4_dn6 = assign101590_e153759_d_n6;
        locals.var_t4_dn7 = assign101590_e153759_d_n7;
        locals.var_t4_dn8 = assign101590_e153759_d_n8;
        locals.var_t4_dn9 = assign101590_e153759_d_n9;
        locals.var_t4_dn10 = assign101590_e153759_d_n10;
        locals.var_t4_dn11 = assign101590_e153759_d_n11;
        locals.var_t4_dn14 = assign101590_e153759_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign101600_e153767, assign101600_e153767_d_n0, assign101600_e153767_d_n2, assign101600_e153767_d_n4, assign101600_e153767_d_n5, assign101600_e153767_d_n6, assign101600_e153767_d_n7, assign101600_e153767_d_n8, assign101600_e153767_d_n9, assign101600_e153767_d_n10, assign101600_e153767_d_n11, assign101600_e153767_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101600_e153763: f64 = (42.0 * locals.var_kusai00);
        let assign101600_e153765: f64 = (assign101600_e153763 * locals.var_kusail);
        (assign101600_e153765, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn4) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn4)), (((42.0 * locals.var_kusai00_dn5) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn5)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn8) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn8)), (((42.0 * locals.var_kusai00_dn9) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn9)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn14) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101600_e153767;
        locals.var_t5_dn0 = assign101600_e153767_d_n0;
        locals.var_t5_dn2 = assign101600_e153767_d_n2;
        locals.var_t5_dn4 = assign101600_e153767_d_n4;
        locals.var_t5_dn5 = assign101600_e153767_d_n5;
        locals.var_t5_dn6 = assign101600_e153767_d_n6;
        locals.var_t5_dn7 = assign101600_e153767_d_n7;
        locals.var_t5_dn8 = assign101600_e153767_d_n8;
        locals.var_t5_dn9 = assign101600_e153767_d_n9;
        locals.var_t5_dn10 = assign101600_e153767_d_n10;
        locals.var_t5_dn11 = assign101600_e153767_d_n11;
        locals.var_t5_dn14 = assign101600_e153767_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign101610_e153777, assign101610_e153777_d_n0, assign101610_e153777_d_n2, assign101610_e153777_d_n4, assign101610_e153777_d_n5, assign101610_e153777_d_n6, assign101610_e153777_d_n7, assign101610_e153777_d_n8, assign101610_e153777_d_n9, assign101610_e153777_d_n10, assign101610_e153777_d_n11, assign101610_e153777_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101610_e153773: f64 = (locals.var_t3 + locals.var_t4);
        let assign101610_e153774: f64 = (4.0 * assign101610_e153773);
        let assign101610_e153775: f64 = (locals.var_t5 + assign101610_e153774);
        (assign101610_e153775, (locals.var_t5_dn0 + (4.0 * (locals.var_t3_dn0 + locals.var_t4_dn0))), (locals.var_t5_dn2 + (4.0 * (locals.var_t3_dn2 + locals.var_t4_dn2))), (locals.var_t5_dn4 + (4.0 * (locals.var_t3_dn4 + locals.var_t4_dn4))), (locals.var_t5_dn5 + (4.0 * (locals.var_t3_dn5 + locals.var_t4_dn5))), (locals.var_t5_dn6 + (4.0 * (locals.var_t3_dn6 + locals.var_t4_dn6))), (locals.var_t5_dn7 + (4.0 * (locals.var_t3_dn7 + locals.var_t4_dn7))), (locals.var_t5_dn8 + (4.0 * (locals.var_t3_dn8 + locals.var_t4_dn8))), (locals.var_t5_dn9 + (4.0 * (locals.var_t3_dn9 + locals.var_t4_dn9))), (locals.var_t5_dn10 + (4.0 * (locals.var_t3_dn10 + locals.var_t4_dn10))), (locals.var_t5_dn11 + (4.0 * (locals.var_t3_dn11 + locals.var_t4_dn11))), (locals.var_t5_dn14 + (4.0 * (locals.var_t3_dn14 + locals.var_t4_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101610_e153777;
        locals.var_t5_dn0 = assign101610_e153777_d_n0;
        locals.var_t5_dn2 = assign101610_e153777_d_n2;
        locals.var_t5_dn4 = assign101610_e153777_d_n4;
        locals.var_t5_dn5 = assign101610_e153777_d_n5;
        locals.var_t5_dn6 = assign101610_e153777_d_n6;
        locals.var_t5_dn7 = assign101610_e153777_d_n7;
        locals.var_t5_dn8 = assign101610_e153777_d_n8;
        locals.var_t5_dn9 = assign101610_e153777_d_n9;
        locals.var_t5_dn10 = assign101610_e153777_d_n10;
        locals.var_t5_dn11 = assign101610_e153777_d_n11;
        locals.var_t5_dn14 = assign101610_e153777_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign101620_e153791, assign101620_e153791_d_n0, assign101620_e153791_d_n2, assign101620_e153791_d_n4, assign101620_e153791_d_n5, assign101620_e153791_d_n6, assign101620_e153791_d_n7, assign101620_e153791_d_n8, assign101620_e153791_d_n9, assign101620_e153791_d_n10, assign101620_e153791_d_n11, assign101620_e153791_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101620_e153782: f64 = (20.0 * locals.var_sqrtkusail);
        let assign101620_e153784: f64 = (assign101620_e153782 * locals.var_vgvt);
        let assign101620_e153787: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign101620_e153788: f64 = (assign101620_e153784 * assign101620_e153787);
        let assign101620_e153789: f64 = (locals.var_t5 + assign101620_e153788);
        (assign101620_e153789, (locals.var_t5_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn0)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn2)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5_dn4 + (((((20.0 * locals.var_sqrtkusail_dn4) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn4)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn4 + locals.var_kusail_dn4)))), (locals.var_t5_dn5 + (((((20.0 * locals.var_sqrtkusail_dn5) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn5)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn5 + locals.var_kusail_dn5)))), (locals.var_t5_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn6)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn7)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5_dn8 + (((((20.0 * locals.var_sqrtkusail_dn8) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn8)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn8 + locals.var_kusail_dn8)))), (locals.var_t5_dn9 + (((((20.0 * locals.var_sqrtkusail_dn9) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn9)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn9 + locals.var_kusail_dn9)))), (locals.var_t5_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn10)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn11)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5_dn14 + (((((20.0 * locals.var_sqrtkusail_dn14) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn14)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn14 + locals.var_kusail_dn14)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101620_e153791;
        locals.var_t5_dn0 = assign101620_e153791_d_n0;
        locals.var_t5_dn2 = assign101620_e153791_d_n2;
        locals.var_t5_dn4 = assign101620_e153791_d_n4;
        locals.var_t5_dn5 = assign101620_e153791_d_n5;
        locals.var_t5_dn6 = assign101620_e153791_d_n6;
        locals.var_t5_dn7 = assign101620_e153791_d_n7;
        locals.var_t5_dn8 = assign101620_e153791_d_n8;
        locals.var_t5_dn9 = assign101620_e153791_d_n9;
        locals.var_t5_dn10 = assign101620_e153791_d_n10;
        locals.var_t5_dn11 = assign101620_e153791_d_n11;
        locals.var_t5_dn14 = assign101620_e153791_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign101630_e153797, assign101630_e153797_d_n0, assign101630_e153797_d_n2, assign101630_e153797_d_n4, assign101630_e153797_d_n5, assign101630_e153797_d_n6, assign101630_e153797_d_n7, assign101630_e153797_d_n8, assign101630_e153797_d_n9, assign101630_e153797_d_n10, assign101630_e153797_d_n11, assign101630_e153797_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101630_e153795: f64 = (locals.var_t2 * locals.var_t2);
        (assign101630_e153795, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101630_e153797;
        locals.var_t10_dn0 = assign101630_e153797_d_n0;
        locals.var_t10_dn2 = assign101630_e153797_d_n2;
        locals.var_t10_dn4 = assign101630_e153797_d_n4;
        locals.var_t10_dn5 = assign101630_e153797_d_n5;
        locals.var_t10_dn6 = assign101630_e153797_d_n6;
        locals.var_t10_dn7 = assign101630_e153797_d_n7;
        locals.var_t10_dn8 = assign101630_e153797_d_n8;
        locals.var_t10_dn9 = assign101630_e153797_d_n9;
        locals.var_t10_dn10 = assign101630_e153797_d_n10;
        locals.var_t10_dn11 = assign101630_e153797_d_n11;
        locals.var_t10_dn14 = assign101630_e153797_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign101640_e153803, assign101640_e153803_d_n0, assign101640_e153803_d_n2, assign101640_e153803_d_n4, assign101640_e153803_d_n5, assign101640_e153803_d_n6, assign101640_e153803_d_n7, assign101640_e153803_d_n8, assign101640_e153803_d_n9, assign101640_e153803_d_n10, assign101640_e153803_d_n11, assign101640_e153803_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101640_e153801: f64 = (locals.var_t10 * locals.var_t10);
        (assign101640_e153801, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)), ((locals.var_t10_dn14 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101640_e153803;
        locals.var_t10_dn0 = assign101640_e153803_d_n0;
        locals.var_t10_dn2 = assign101640_e153803_d_n2;
        locals.var_t10_dn4 = assign101640_e153803_d_n4;
        locals.var_t10_dn5 = assign101640_e153803_d_n5;
        locals.var_t10_dn6 = assign101640_e153803_d_n6;
        locals.var_t10_dn7 = assign101640_e153803_d_n7;
        locals.var_t10_dn8 = assign101640_e153803_d_n8;
        locals.var_t10_dn9 = assign101640_e153803_d_n9;
        locals.var_t10_dn10 = assign101640_e153803_d_n10;
        locals.var_t10_dn11 = assign101640_e153803_d_n11;
        locals.var_t10_dn14 = assign101640_e153803_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign101650_e153811, assign101650_e153811_d_n0, assign101650_e153811_d_n2, assign101650_e153811_d_n4, assign101650_e153811_d_n5, assign101650_e153811_d_n6, assign101650_e153811_d_n7, assign101650_e153811_d_n8, assign101650_e153811_d_n9, assign101650_e153811_d_n10, assign101650_e153811_d_n11, assign101650_e153811_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101650_e153808: f64 = (locals.var_t10 * locals.var_t2);
        let assign101650_e153809: f64 = (locals.var_t5 / assign101650_e153808);
        (assign101650_e153809, (((locals.var_t5_dn0 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn0 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn0)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn2 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn2 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn2)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn4 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn4 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn4)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn5 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn5 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn5)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn6 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn6 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn6)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn7 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn7 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn7)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn8 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn8 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn8)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn9 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn9 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn9)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn10 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn10 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn10)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn11 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn11 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn11)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn14 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn14 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn14)))) / (assign101650_e153808 * assign101650_e153808)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn4, locals.var_kusai_ig_dn5, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn8, locals.var_kusai_ig_dn9, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn14,)
    }
};
        locals.var_kusai_ig = assign101650_e153811;
        locals.var_kusai_ig_dn0 = assign101650_e153811_d_n0;
        locals.var_kusai_ig_dn2 = assign101650_e153811_d_n2;
        locals.var_kusai_ig_dn4 = assign101650_e153811_d_n4;
        locals.var_kusai_ig_dn5 = assign101650_e153811_d_n5;
        locals.var_kusai_ig_dn6 = assign101650_e153811_d_n6;
        locals.var_kusai_ig_dn7 = assign101650_e153811_d_n7;
        locals.var_kusai_ig_dn8 = assign101650_e153811_d_n8;
        locals.var_kusai_ig_dn9 = assign101650_e153811_d_n9;
        locals.var_kusai_ig_dn10 = assign101650_e153811_d_n10;
        locals.var_kusai_ig_dn11 = assign101650_e153811_d_n11;
        locals.var_kusai_ig_dn14 = assign101650_e153811_d_n14;
        locals.var_kusai_ig_rv = 0.0;

        let (assign101660_e153821, assign101660_e153821_d_n0, assign101660_e153821_d_n2, assign101660_e153821_d_n4, assign101660_e153821_d_n5, assign101660_e153821_d_n6, assign101660_e153821_d_n7, assign101660_e153821_d_n8, assign101660_e153821_d_n9, assign101660_e153821_d_n10, assign101660_e153821_d_n11, assign101660_e153821_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101660_e153815: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign101660_e153817: f64 = (assign101660_e153815 * locals.var_mu);
        let assign101660_e153819: f64 = (assign101660_e153817 * locals.var_cox);
        (assign101660_e153819, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn0)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn2)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn4) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn4)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn4)), (((((-((locals.var_weff_nf * locals.var_lch_dn5) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn5)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn5)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn6)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn7)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn8) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn8)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn8)), (((((-((locals.var_weff_nf * locals.var_lch_dn9) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn9)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn9)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn10)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn11) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn11)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn11)), (((((-((locals.var_weff_nf * locals.var_lch_dn14) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn14)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn14)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn4, locals.var_gds0_ign_dn5, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn8, locals.var_gds0_ign_dn9, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn14,)
    }
};
        locals.var_gds0_ign = assign101660_e153821;
        locals.var_gds0_ign_dn0 = assign101660_e153821_d_n0;
        locals.var_gds0_ign_dn2 = assign101660_e153821_d_n2;
        locals.var_gds0_ign_dn4 = assign101660_e153821_d_n4;
        locals.var_gds0_ign_dn5 = assign101660_e153821_d_n5;
        locals.var_gds0_ign_dn6 = assign101660_e153821_d_n6;
        locals.var_gds0_ign_dn7 = assign101660_e153821_d_n7;
        locals.var_gds0_ign_dn8 = assign101660_e153821_d_n8;
        locals.var_gds0_ign_dn9 = assign101660_e153821_d_n9;
        locals.var_gds0_ign_dn10 = assign101660_e153821_d_n10;
        locals.var_gds0_ign_dn11 = assign101660_e153821_d_n11;
        locals.var_gds0_ign_dn14 = assign101660_e153821_d_n14;
        locals.var_gds0_ign_rv = 0.0;

        let (assign101690_e153845, assign101690_e153845_d_n0, assign101690_e153845_d_n2, assign101690_e153845_d_n4, assign101690_e153845_d_n5, assign101690_e153845_d_n6, assign101690_e153845_d_n7, assign101690_e153845_d_n8, assign101690_e153845_d_n9, assign101690_e153845_d_n10, assign101690_e153845_d_n11, assign101690_e153845_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101690_e153838: f64 = (4.0 * locals.var_vgvt);
        let assign101690_e153840: f64 = (assign101690_e153838 * locals.var_sqrtkusail);
        let assign101690_e153841: f64 = (locals.var_kusai00 + assign101690_e153840);
        let assign101690_e153843: f64 = (assign101690_e153841 + locals.var_kusail);
        (assign101690_e153843, ((locals.var_kusai00_dn0 + (((4.0 * locals.var_vgvt_dn0) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0), ((locals.var_kusai00_dn2 + (((4.0 * locals.var_vgvt_dn2) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2), ((locals.var_kusai00_dn4 + (((4.0 * locals.var_vgvt_dn4) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4), ((locals.var_kusai00_dn5 + (((4.0 * locals.var_vgvt_dn5) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5), ((locals.var_kusai00_dn6 + (((4.0 * locals.var_vgvt_dn6) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6), ((locals.var_kusai00_dn7 + (((4.0 * locals.var_vgvt_dn7) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7), ((locals.var_kusai00_dn8 + (((4.0 * locals.var_vgvt_dn8) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8), ((locals.var_kusai00_dn9 + (((4.0 * locals.var_vgvt_dn9) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9), ((locals.var_kusai00_dn10 + (((4.0 * locals.var_vgvt_dn10) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10), ((locals.var_kusai00_dn11 + (((4.0 * locals.var_vgvt_dn11) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11), ((locals.var_kusai00_dn14 + (((4.0 * locals.var_vgvt_dn14) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn14))) + locals.var_kusail_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101690_e153845;
        locals.var_t7_dn0 = assign101690_e153845_d_n0;
        locals.var_t7_dn2 = assign101690_e153845_d_n2;
        locals.var_t7_dn4 = assign101690_e153845_d_n4;
        locals.var_t7_dn5 = assign101690_e153845_d_n5;
        locals.var_t7_dn6 = assign101690_e153845_d_n6;
        locals.var_t7_dn7 = assign101690_e153845_d_n7;
        locals.var_t7_dn8 = assign101690_e153845_d_n8;
        locals.var_t7_dn9 = assign101690_e153845_d_n9;
        locals.var_t7_dn10 = assign101690_e153845_d_n10;
        locals.var_t7_dn11 = assign101690_e153845_d_n11;
        locals.var_t7_dn14 = assign101690_e153845_d_n14;
        locals.var_t7_rv = 0.0;

        let assign101710_e153869: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign101710_e153869;
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

        let assign101750_e153881: f64 = (locals.var_mfactor * locals.var_idsibpc);
        locals.var_idsibpce = assign101750_e153881;
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

        let assign101850_e153895: f64 = if ((locals.var_flg_nqs != 0.0) || (p.p22 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard2333 = assign101850_e153895;
        locals.var_guard2333_rv = 0.0;

        let (assign101860_e153899, assign101860_e153899_d_n0, assign101860_e153899_d_n2, assign101860_e153899_d_n4, assign101860_e153899_d_n5, assign101860_e153899_d_n6, assign101860_e153899_d_n7, assign101860_e153899_d_n8, assign101860_e153899_d_n9, assign101860_e153899_d_n10, assign101860_e153899_d_n11, assign101860_e153899_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign101860_e153899;
        locals.var_qge_dn0 = assign101860_e153899_d_n0;
        locals.var_qge_dn2 = assign101860_e153899_d_n2;
        locals.var_qge_dn4 = assign101860_e153899_d_n4;
        locals.var_qge_dn5 = assign101860_e153899_d_n5;
        locals.var_qge_dn6 = assign101860_e153899_d_n6;
        locals.var_qge_dn7 = assign101860_e153899_d_n7;
        locals.var_qge_dn8 = assign101860_e153899_d_n8;
        locals.var_qge_dn9 = assign101860_e153899_d_n9;
        locals.var_qge_dn10 = assign101860_e153899_d_n10;
        locals.var_qge_dn11 = assign101860_e153899_d_n11;
        locals.var_qge_dn14 = assign101860_e153899_d_n14;
        locals.var_qge_rv = 0.0;

        let (assign101870_e153903, assign101870_e153903_d_n0, assign101870_e153903_d_n2, assign101870_e153903_d_n4, assign101870_e153903_d_n5, assign101870_e153903_d_n6, assign101870_e153903_d_n7, assign101870_e153903_d_n8, assign101870_e153903_d_n9, assign101870_e153903_d_n10, assign101870_e153903_d_n11, assign101870_e153903_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign101870_e153903;
        locals.var_qde_dn0 = assign101870_e153903_d_n0;
        locals.var_qde_dn2 = assign101870_e153903_d_n2;
        locals.var_qde_dn4 = assign101870_e153903_d_n4;
        locals.var_qde_dn5 = assign101870_e153903_d_n5;
        locals.var_qde_dn6 = assign101870_e153903_d_n6;
        locals.var_qde_dn7 = assign101870_e153903_d_n7;
        locals.var_qde_dn8 = assign101870_e153903_d_n8;
        locals.var_qde_dn9 = assign101870_e153903_d_n9;
        locals.var_qde_dn10 = assign101870_e153903_d_n10;
        locals.var_qde_dn11 = assign101870_e153903_d_n11;
        locals.var_qde_dn14 = assign101870_e153903_d_n14;
        locals.var_qde_rv = 0.0;

        let (assign101880_e153907, assign101880_e153907_d_n0, assign101880_e153907_d_n2, assign101880_e153907_d_n4, assign101880_e153907_d_n5, assign101880_e153907_d_n6, assign101880_e153907_d_n7, assign101880_e153907_d_n8, assign101880_e153907_d_n9, assign101880_e153907_d_n10, assign101880_e153907_d_n11, assign101880_e153907_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign101880_e153907;
        locals.var_qse_dn0 = assign101880_e153907_d_n0;
        locals.var_qse_dn2 = assign101880_e153907_d_n2;
        locals.var_qse_dn4 = assign101880_e153907_d_n4;
        locals.var_qse_dn5 = assign101880_e153907_d_n5;
        locals.var_qse_dn6 = assign101880_e153907_d_n6;
        locals.var_qse_dn7 = assign101880_e153907_d_n7;
        locals.var_qse_dn8 = assign101880_e153907_d_n8;
        locals.var_qse_dn9 = assign101880_e153907_d_n9;
        locals.var_qse_dn10 = assign101880_e153907_d_n10;
        locals.var_qse_dn11 = assign101880_e153907_d_n11;
        locals.var_qse_dn14 = assign101880_e153907_d_n14;
        locals.var_qse_rv = 0.0;

        let (assign101890_e153911, assign101890_e153911_d_n0, assign101890_e153911_d_n2, assign101890_e153911_d_n4, assign101890_e153911_d_n5, assign101890_e153911_d_n6, assign101890_e153911_d_n7, assign101890_e153911_d_n8, assign101890_e153911_d_n9, assign101890_e153911_d_n10, assign101890_e153911_d_n11, assign101890_e153911_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn14,)
    }
};
        locals.var_xd = assign101890_e153911;
        locals.var_xd_dn0 = assign101890_e153911_d_n0;
        locals.var_xd_dn2 = assign101890_e153911_d_n2;
        locals.var_xd_dn4 = assign101890_e153911_d_n4;
        locals.var_xd_dn5 = assign101890_e153911_d_n5;
        locals.var_xd_dn6 = assign101890_e153911_d_n6;
        locals.var_xd_dn7 = assign101890_e153911_d_n7;
        locals.var_xd_dn8 = assign101890_e153911_d_n8;
        locals.var_xd_dn9 = assign101890_e153911_d_n9;
        locals.var_xd_dn10 = assign101890_e153911_d_n10;
        locals.var_xd_dn11 = assign101890_e153911_d_n11;
        locals.var_xd_dn14 = assign101890_e153911_d_n14;
        locals.var_xd_rv = 0.0;

        let (assign101910_e153923, assign101910_e153923_d_n0, assign101910_e153923_d_n2, assign101910_e153923_d_n4, assign101910_e153923_d_n5, assign101910_e153923_d_n6, assign101910_e153923_d_n7, assign101910_e153923_d_n8, assign101910_e153923_d_n9, assign101910_e153923_d_n10, assign101910_e153923_d_n11, assign101910_e153923_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign101910_e153921: f64 = (locals.var_mfactor * locals.var_qi);
        (assign101910_e153921, (locals.var_mfactor * locals.var_qi_dn0), (locals.var_mfactor * locals.var_qi_dn2), (locals.var_mfactor * locals.var_qi_dn4), (locals.var_mfactor * locals.var_qi_dn5), (locals.var_mfactor * locals.var_qi_dn6), (locals.var_mfactor * locals.var_qi_dn7), (locals.var_mfactor * locals.var_qi_dn8), (locals.var_mfactor * locals.var_qi_dn9), (locals.var_mfactor * locals.var_qi_dn10), (locals.var_mfactor * locals.var_qi_dn11), (locals.var_mfactor * locals.var_qi_dn14),)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn4, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, locals.var_qi_dn9, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn14,)
    }
};
        locals.var_qi = assign101910_e153923;
        locals.var_qi_dn0 = assign101910_e153923_d_n0;
        locals.var_qi_dn2 = assign101910_e153923_d_n2;
        locals.var_qi_dn4 = assign101910_e153923_d_n4;
        locals.var_qi_dn5 = assign101910_e153923_d_n5;
        locals.var_qi_dn6 = assign101910_e153923_d_n6;
        locals.var_qi_dn7 = assign101910_e153923_d_n7;
        locals.var_qi_dn8 = assign101910_e153923_d_n8;
        locals.var_qi_dn9 = assign101910_e153923_d_n9;
        locals.var_qi_dn10 = assign101910_e153923_d_n10;
        locals.var_qi_dn11 = assign101910_e153923_d_n11;
        locals.var_qi_dn14 = assign101910_e153923_d_n14;
        locals.var_qi_rv = 0.0;

        let (assign101920_e153933, assign101920_e153933_d_n0, assign101920_e153933_d_n2, assign101920_e153933_d_n4, assign101920_e153933_d_n5, assign101920_e153933_d_n6, assign101920_e153933_d_n7, assign101920_e153933_d_n8, assign101920_e153933_d_n9, assign101920_e153933_d_n10, assign101920_e153933_d_n11, assign101920_e153933_d_n14,) = {
    if (locals.var_guard2333 == 0.0) {
        let assign101920_e153929: f64 = (locals.var_qb + locals.var_qi);
        let assign101920_e153930: f64 = (-assign101920_e153929);
        let assign101920_e153931: f64 = (locals.var_mfactor * assign101920_e153930);
        (assign101920_e153931, (locals.var_mfactor * (-(locals.var_qb_dn0 + locals.var_qi_dn0))), (locals.var_mfactor * (-(locals.var_qb_dn2 + locals.var_qi_dn2))), (locals.var_mfactor * (-(locals.var_qb_dn4 + locals.var_qi_dn4))), (locals.var_mfactor * (-(locals.var_qb_dn5 + locals.var_qi_dn5))), (locals.var_mfactor * (-(locals.var_qb_dn6 + locals.var_qi_dn6))), (locals.var_mfactor * (-(locals.var_qb_dn7 + locals.var_qi_dn7))), (locals.var_mfactor * (-(locals.var_qb_dn8 + locals.var_qi_dn8))), (locals.var_mfactor * (-(locals.var_qb_dn9 + locals.var_qi_dn9))), (locals.var_mfactor * (-(locals.var_qb_dn10 + locals.var_qi_dn10))), (locals.var_mfactor * (-(locals.var_qb_dn11 + locals.var_qi_dn11))), (locals.var_mfactor * (-(locals.var_qb_dn14 + locals.var_qi_dn14))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign101920_e153933;
        locals.var_qge_dn0 = assign101920_e153933_d_n0;
        locals.var_qge_dn2 = assign101920_e153933_d_n2;
        locals.var_qge_dn4 = assign101920_e153933_d_n4;
        locals.var_qge_dn5 = assign101920_e153933_d_n5;
        locals.var_qge_dn6 = assign101920_e153933_d_n6;
        locals.var_qge_dn7 = assign101920_e153933_d_n7;
        locals.var_qge_dn8 = assign101920_e153933_d_n8;
        locals.var_qge_dn9 = assign101920_e153933_d_n9;
        locals.var_qge_dn10 = assign101920_e153933_d_n10;
        locals.var_qge_dn11 = assign101920_e153933_d_n11;
        locals.var_qge_dn14 = assign101920_e153933_d_n14;
        locals.var_qge_rv = 0.0;

        let (assign101930_e153940, assign101930_e153940_d_n0, assign101930_e153940_d_n2, assign101930_e153940_d_n4, assign101930_e153940_d_n5, assign101930_e153940_d_n6, assign101930_e153940_d_n7, assign101930_e153940_d_n8, assign101930_e153940_d_n9, assign101930_e153940_d_n10, assign101930_e153940_d_n11, assign101930_e153940_d_n14,) = {
    if (locals.var_guard2333 == 0.0) {
        let assign101930_e153938: f64 = (locals.var_mfactor * locals.var_qd);
        (assign101930_e153938, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn4), (locals.var_mfactor * locals.var_qd_dn5), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn8), (locals.var_mfactor * locals.var_qd_dn9), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn14),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign101930_e153940;
        locals.var_qde_dn0 = assign101930_e153940_d_n0;
        locals.var_qde_dn2 = assign101930_e153940_d_n2;
        locals.var_qde_dn4 = assign101930_e153940_d_n4;
        locals.var_qde_dn5 = assign101930_e153940_d_n5;
        locals.var_qde_dn6 = assign101930_e153940_d_n6;
        locals.var_qde_dn7 = assign101930_e153940_d_n7;
        locals.var_qde_dn8 = assign101930_e153940_d_n8;
        locals.var_qde_dn9 = assign101930_e153940_d_n9;
        locals.var_qde_dn10 = assign101930_e153940_d_n10;
        locals.var_qde_dn11 = assign101930_e153940_d_n11;
        locals.var_qde_dn14 = assign101930_e153940_d_n14;
        locals.var_qde_rv = 0.0;

        let (assign101940_e153949, assign101940_e153949_d_n0, assign101940_e153949_d_n2, assign101940_e153949_d_n4, assign101940_e153949_d_n5, assign101940_e153949_d_n6, assign101940_e153949_d_n7, assign101940_e153949_d_n8, assign101940_e153949_d_n9, assign101940_e153949_d_n10, assign101940_e153949_d_n11, assign101940_e153949_d_n14,) = {
    if (locals.var_guard2333 == 0.0) {
        let assign101940_e153946: f64 = (locals.var_qi - locals.var_qd);
        let assign101940_e153947: f64 = (locals.var_mfactor * assign101940_e153946);
        (assign101940_e153947, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn4 - locals.var_qd_dn4)), (locals.var_mfactor * (locals.var_qi_dn5 - locals.var_qd_dn5)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn8 - locals.var_qd_dn8)), (locals.var_mfactor * (locals.var_qi_dn9 - locals.var_qd_dn9)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn14 - locals.var_qd_dn14)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign101940_e153949;
        locals.var_qse_dn0 = assign101940_e153949_d_n0;
        locals.var_qse_dn2 = assign101940_e153949_d_n2;
        locals.var_qse_dn4 = assign101940_e153949_d_n4;
        locals.var_qse_dn5 = assign101940_e153949_d_n5;
        locals.var_qse_dn6 = assign101940_e153949_d_n6;
        locals.var_qse_dn7 = assign101940_e153949_d_n7;
        locals.var_qse_dn8 = assign101940_e153949_d_n8;
        locals.var_qse_dn9 = assign101940_e153949_d_n9;
        locals.var_qse_dn10 = assign101940_e153949_d_n10;
        locals.var_qse_dn11 = assign101940_e153949_d_n11;
        locals.var_qse_dn14 = assign101940_e153949_d_n14;
        locals.var_qse_rv = 0.0;

        let (assign101950_e153955, assign101950_e153955_d_n0, assign101950_e153955_d_n2, assign101950_e153955_d_n4, assign101950_e153955_d_n5, assign101950_e153955_d_n6, assign101950_e153955_d_n7, assign101950_e153955_d_n8, assign101950_e153955_d_n9, assign101950_e153955_d_n10, assign101950_e153955_d_n11, assign101950_e153955_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101950_e153953: f64 = (locals.var_mks_dlyov * locals.var_psl);
        (assign101950_e153953, ((locals.var_mks_dlyov_dn0 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn0)), ((locals.var_mks_dlyov_dn2 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn2)), ((locals.var_mks_dlyov_dn4 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn4)), ((locals.var_mks_dlyov_dn5 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn5)), ((locals.var_mks_dlyov_dn6 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn6)), ((locals.var_mks_dlyov_dn7 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn7)), ((locals.var_mks_dlyov_dn8 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn8)), ((locals.var_mks_dlyov_dn9 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn9)), ((locals.var_mks_dlyov_dn10 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn10)), ((locals.var_mks_dlyov_dn11 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn11)), ((locals.var_mks_dlyov_dn14 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn14)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign101950_e153955;
        locals.var_mks_dlyov_dn0 = assign101950_e153955_d_n0;
        locals.var_mks_dlyov_dn2 = assign101950_e153955_d_n2;
        locals.var_mks_dlyov_dn4 = assign101950_e153955_d_n4;
        locals.var_mks_dlyov_dn5 = assign101950_e153955_d_n5;
        locals.var_mks_dlyov_dn6 = assign101950_e153955_d_n6;
        locals.var_mks_dlyov_dn7 = assign101950_e153955_d_n7;
        locals.var_mks_dlyov_dn8 = assign101950_e153955_d_n8;
        locals.var_mks_dlyov_dn9 = assign101950_e153955_d_n9;
        locals.var_mks_dlyov_dn10 = assign101950_e153955_d_n10;
        locals.var_mks_dlyov_dn11 = assign101950_e153955_d_n11;
        locals.var_mks_dlyov_dn14 = assign101950_e153955_d_n14;
        locals.var_mks_dlyov_rv = 0.0;

        let (assign101960_e153968, assign101960_e153968_d_n0, assign101960_e153968_d_n2, assign101960_e153968_d_n4, assign101960_e153968_d_n5, assign101960_e153968_d_n6, assign101960_e153968_d_n7, assign101960_e153968_d_n8, assign101960_e153968_d_n9, assign101960_e153968_d_n10, assign101960_e153968_d_n11, assign101960_e153968_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101960_e153959: f64 = (locals.var_mks_dlyov * locals.var_mks_dlyov);
        let assign101960_e153962: f64 = (4.0 * 1e-12);
        let assign101960_e153964: f64 = (assign101960_e153962 * 1e-12);
        let assign101960_e153965: f64 = (assign101960_e153959 + assign101960_e153964);
        let assign101960_e153966: f64 = (assign101960_e153965).sqrt();
        (assign101960_e153966, (((locals.var_mks_dlyov_dn0 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn0)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn2 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn2)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn4 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn4)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn5 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn5)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn6 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn6)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn7 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn7)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn8 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn8)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn9 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn9)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn10 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn10)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn11 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn11)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn14 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn14)) / (2.0 * assign101960_e153966)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign101960_e153968;
        locals.var_tmf2_dn0 = assign101960_e153968_d_n0;
        locals.var_tmf2_dn2 = assign101960_e153968_d_n2;
        locals.var_tmf2_dn4 = assign101960_e153968_d_n4;
        locals.var_tmf2_dn5 = assign101960_e153968_d_n5;
        locals.var_tmf2_dn6 = assign101960_e153968_d_n6;
        locals.var_tmf2_dn7 = assign101960_e153968_d_n7;
        locals.var_tmf2_dn8 = assign101960_e153968_d_n8;
        locals.var_tmf2_dn9 = assign101960_e153968_d_n9;
        locals.var_tmf2_dn10 = assign101960_e153968_d_n10;
        locals.var_tmf2_dn11 = assign101960_e153968_d_n11;
        locals.var_tmf2_dn14 = assign101960_e153968_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign101970_e153978, assign101970_e153978_d_n0, assign101970_e153978_d_n2, assign101970_e153978_d_n4, assign101970_e153978_d_n5, assign101970_e153978_d_n6, assign101970_e153978_d_n7, assign101970_e153978_d_n8, assign101970_e153978_d_n9, assign101970_e153978_d_n10, assign101970_e153978_d_n11, assign101970_e153978_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101970_e153974: f64 = (locals.var_mks_dlyov / locals.var_tmf2);
        let assign101970_e153975: f64 = (1.0 + assign101970_e153974);
        let assign101970_e153976: f64 = (0.5 * assign101970_e153975);
        (assign101970_e153976, (0.5 * (((locals.var_mks_dlyov_dn0 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn2 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn4 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn5 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn6 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn7 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn8 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn9 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn10 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn11 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn14 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101970_e153978;
        locals.var_t0_dn0 = assign101970_e153978_d_n0;
        locals.var_t0_dn2 = assign101970_e153978_d_n2;
        locals.var_t0_dn4 = assign101970_e153978_d_n4;
        locals.var_t0_dn5 = assign101970_e153978_d_n5;
        locals.var_t0_dn6 = assign101970_e153978_d_n6;
        locals.var_t0_dn7 = assign101970_e153978_d_n7;
        locals.var_t0_dn8 = assign101970_e153978_d_n8;
        locals.var_t0_dn9 = assign101970_e153978_d_n9;
        locals.var_t0_dn10 = assign101970_e153978_d_n10;
        locals.var_t0_dn11 = assign101970_e153978_d_n11;
        locals.var_t0_dn14 = assign101970_e153978_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign101980_e153986, assign101980_e153986_d_n0, assign101980_e153986_d_n2, assign101980_e153986_d_n4, assign101980_e153986_d_n5, assign101980_e153986_d_n6, assign101980_e153986_d_n7, assign101980_e153986_d_n8, assign101980_e153986_d_n9, assign101980_e153986_d_n10, assign101980_e153986_d_n11, assign101980_e153986_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101980_e153983: f64 = (locals.var_mks_dlyov + locals.var_tmf2);
        let assign101980_e153984: f64 = (0.5 * assign101980_e153983);
        (assign101980_e153984, (0.5 * (locals.var_mks_dlyov_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_mks_dlyov_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_mks_dlyov_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_mks_dlyov_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_mks_dlyov_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_mks_dlyov_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_mks_dlyov_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_mks_dlyov_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_mks_dlyov_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_mks_dlyov_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_mks_dlyov_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign101980_e153986;
        locals.var_mks_dlyov_dn0 = assign101980_e153986_d_n0;
        locals.var_mks_dlyov_dn2 = assign101980_e153986_d_n2;
        locals.var_mks_dlyov_dn4 = assign101980_e153986_d_n4;
        locals.var_mks_dlyov_dn5 = assign101980_e153986_d_n5;
        locals.var_mks_dlyov_dn6 = assign101980_e153986_d_n6;
        locals.var_mks_dlyov_dn7 = assign101980_e153986_d_n7;
        locals.var_mks_dlyov_dn8 = assign101980_e153986_d_n8;
        locals.var_mks_dlyov_dn9 = assign101980_e153986_d_n9;
        locals.var_mks_dlyov_dn10 = assign101980_e153986_d_n10;
        locals.var_mks_dlyov_dn11 = assign101980_e153986_d_n11;
        locals.var_mks_dlyov_dn14 = assign101980_e153986_d_n14;
        locals.var_mks_dlyov_rv = 0.0;

        let assign101990_e153989: f64 = if locals.var_mks_dlyov < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2334 = assign101990_e153989;
        locals.var_guard2334_rv = 0.0;

        let (assign102000_e153995, assign102000_e153995_d_n0, assign102000_e153995_d_n2, assign102000_e153995_d_n4, assign102000_e153995_d_n5, assign102000_e153995_d_n6, assign102000_e153995_d_n7, assign102000_e153995_d_n8, assign102000_e153995_d_n9, assign102000_e153995_d_n10, assign102000_e153995_d_n11, assign102000_e153995_d_n14,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2334 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign102000_e153995;
        locals.var_mks_dlyov_dn0 = assign102000_e153995_d_n0;
        locals.var_mks_dlyov_dn2 = assign102000_e153995_d_n2;
        locals.var_mks_dlyov_dn4 = assign102000_e153995_d_n4;
        locals.var_mks_dlyov_dn5 = assign102000_e153995_d_n5;
        locals.var_mks_dlyov_dn6 = assign102000_e153995_d_n6;
        locals.var_mks_dlyov_dn7 = assign102000_e153995_d_n7;
        locals.var_mks_dlyov_dn8 = assign102000_e153995_d_n8;
        locals.var_mks_dlyov_dn9 = assign102000_e153995_d_n9;
        locals.var_mks_dlyov_dn10 = assign102000_e153995_d_n10;
        locals.var_mks_dlyov_dn11 = assign102000_e153995_d_n11;
        locals.var_mks_dlyov_dn14 = assign102000_e153995_d_n14;
        locals.var_mks_dlyov_rv = 0.0;

        let (assign102010_e154001, assign102010_e154001_d_n0, assign102010_e154001_d_n2, assign102010_e154001_d_n4, assign102010_e154001_d_n5, assign102010_e154001_d_n6, assign102010_e154001_d_n7, assign102010_e154001_d_n8, assign102010_e154001_d_n9, assign102010_e154001_d_n10, assign102010_e154001_d_n11, assign102010_e154001_d_n14,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2334 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign102010_e154001;
        locals.var_t0_dn0 = assign102010_e154001_d_n0;
        locals.var_t0_dn2 = assign102010_e154001_d_n2;
        locals.var_t0_dn4 = assign102010_e154001_d_n4;
        locals.var_t0_dn5 = assign102010_e154001_d_n5;
        locals.var_t0_dn6 = assign102010_e154001_d_n6;
        locals.var_t0_dn7 = assign102010_e154001_d_n7;
        locals.var_t0_dn8 = assign102010_e154001_d_n8;
        locals.var_t0_dn9 = assign102010_e154001_d_n9;
        locals.var_t0_dn10 = assign102010_e154001_d_n10;
        locals.var_t0_dn11 = assign102010_e154001_d_n11;
        locals.var_t0_dn14 = assign102010_e154001_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign102030_e154011, assign102030_e154011_d_n0, assign102030_e154011_d_n2, assign102030_e154011_d_n4, assign102030_e154011_d_n5, assign102030_e154011_d_n6, assign102030_e154011_d_n7, assign102030_e154011_d_n8, assign102030_e154011_d_n9, assign102030_e154011_d_n10, assign102030_e154011_d_n11, assign102030_e154011_d_n14,) = {
    if (p.p29 != 0.0) {
        ((nv14 - 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    }
};
        locals.var_qbd_nqs = assign102030_e154011;
        locals.var_qbd_nqs_dn0 = assign102030_e154011_d_n0;
        locals.var_qbd_nqs_dn2 = assign102030_e154011_d_n2;
        locals.var_qbd_nqs_dn4 = assign102030_e154011_d_n4;
        locals.var_qbd_nqs_dn5 = assign102030_e154011_d_n5;
        locals.var_qbd_nqs_dn6 = assign102030_e154011_d_n6;
        locals.var_qbd_nqs_dn7 = assign102030_e154011_d_n7;
        locals.var_qbd_nqs_dn8 = assign102030_e154011_d_n8;
        locals.var_qbd_nqs_dn9 = assign102030_e154011_d_n9;
        locals.var_qbd_nqs_dn10 = assign102030_e154011_d_n10;
        locals.var_qbd_nqs_dn11 = assign102030_e154011_d_n11;
        locals.var_qbd_nqs_dn14 = assign102030_e154011_d_n14;
        locals.var_qbd_nqs_rv = 0.0;

        let (assign102050_e154027, assign102050_e154027_d_n0, assign102050_e154027_d_n2, assign102050_e154027_d_n4, assign102050_e154027_d_n5, assign102050_e154027_d_n6, assign102050_e154027_d_n7, assign102050_e154027_d_n8, assign102050_e154027_d_n9, assign102050_e154027_d_n10, assign102050_e154027_d_n11, assign102050_e154027_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign102050_e154024: f64 = (locals.var_qbd_qs - locals.var_qbd_nqs);
        let assign102050_e154025: f64 = (locals.var_qovd - assign102050_e154024);
        (assign102050_e154025, (locals.var_qovd_dn0 - (locals.var_qbd_qs_dn0 - locals.var_qbd_nqs_dn0)), (locals.var_qovd_dn2 - (locals.var_qbd_qs_dn2 - locals.var_qbd_nqs_dn2)), (locals.var_qovd_dn4 - (locals.var_qbd_qs_dn4 - locals.var_qbd_nqs_dn4)), (locals.var_qovd_dn5 - (locals.var_qbd_qs_dn5 - locals.var_qbd_nqs_dn5)), (locals.var_qovd_dn6 - (locals.var_qbd_qs_dn6 - locals.var_qbd_nqs_dn6)), (locals.var_qovd_dn7 - (locals.var_qbd_qs_dn7 - locals.var_qbd_nqs_dn7)), (locals.var_qovd_dn8 - (locals.var_qbd_qs_dn8 - locals.var_qbd_nqs_dn8)), (locals.var_qovd_dn9 - (locals.var_qbd_qs_dn9 - locals.var_qbd_nqs_dn9)), (locals.var_qovd_dn10 - (locals.var_qbd_qs_dn10 - locals.var_qbd_nqs_dn10)), (locals.var_qovd_dn11 - (locals.var_qbd_qs_dn11 - locals.var_qbd_nqs_dn11)), (locals.var_qovd_dn14 - (locals.var_qbd_qs_dn14 - locals.var_qbd_nqs_dn14)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign102050_e154027;
        locals.var_qovd_dn0 = assign102050_e154027_d_n0;
        locals.var_qovd_dn2 = assign102050_e154027_d_n2;
        locals.var_qovd_dn4 = assign102050_e154027_d_n4;
        locals.var_qovd_dn5 = assign102050_e154027_d_n5;
        locals.var_qovd_dn6 = assign102050_e154027_d_n6;
        locals.var_qovd_dn7 = assign102050_e154027_d_n7;
        locals.var_qovd_dn8 = assign102050_e154027_d_n8;
        locals.var_qovd_dn9 = assign102050_e154027_d_n9;
        locals.var_qovd_dn10 = assign102050_e154027_d_n10;
        locals.var_qovd_dn11 = assign102050_e154027_d_n11;
        locals.var_qovd_dn14 = assign102050_e154027_d_n14;
        locals.var_qovd_rv = 0.0;

        let (assign102060_e154031, assign102060_e154031_d_n0, assign102060_e154031_d_n2, assign102060_e154031_d_n4, assign102060_e154031_d_n5, assign102060_e154031_d_n6, assign102060_e154031_d_n7, assign102060_e154031_d_n8, assign102060_e154031_d_n9, assign102060_e154031_d_n10, assign102060_e154031_d_n11, assign102060_e154031_d_n14,) = {
    if (p.p29 != 0.0) {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    }
};
        locals.var_qbdld = assign102060_e154031;
        locals.var_qbdld_dn0 = assign102060_e154031_d_n0;
        locals.var_qbdld_dn2 = assign102060_e154031_d_n2;
        locals.var_qbdld_dn4 = assign102060_e154031_d_n4;
        locals.var_qbdld_dn5 = assign102060_e154031_d_n5;
        locals.var_qbdld_dn6 = assign102060_e154031_d_n6;
        locals.var_qbdld_dn7 = assign102060_e154031_d_n7;
        locals.var_qbdld_dn8 = assign102060_e154031_d_n8;
        locals.var_qbdld_dn9 = assign102060_e154031_d_n9;
        locals.var_qbdld_dn10 = assign102060_e154031_d_n10;
        locals.var_qbdld_dn11 = assign102060_e154031_d_n11;
        locals.var_qbdld_dn14 = assign102060_e154031_d_n14;
        locals.var_qbdld_rv = 0.0;

        let (assign102070_e154036, assign102070_e154036_d_n0, assign102070_e154036_d_n2, assign102070_e154036_d_n4, assign102070_e154036_d_n5, assign102070_e154036_d_n6, assign102070_e154036_d_n7, assign102070_e154036_d_n8, assign102070_e154036_d_n9, assign102070_e154036_d_n10, assign102070_e154036_d_n11, assign102070_e154036_d_n14,) = {
    if (p.p29 == 0.0) {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn11, locals.var_qbd_qs_dn14,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    }
};
        locals.var_qbd_nqs = assign102070_e154036;
        locals.var_qbd_nqs_dn0 = assign102070_e154036_d_n0;
        locals.var_qbd_nqs_dn2 = assign102070_e154036_d_n2;
        locals.var_qbd_nqs_dn4 = assign102070_e154036_d_n4;
        locals.var_qbd_nqs_dn5 = assign102070_e154036_d_n5;
        locals.var_qbd_nqs_dn6 = assign102070_e154036_d_n6;
        locals.var_qbd_nqs_dn7 = assign102070_e154036_d_n7;
        locals.var_qbd_nqs_dn8 = assign102070_e154036_d_n8;
        locals.var_qbd_nqs_dn9 = assign102070_e154036_d_n9;
        locals.var_qbd_nqs_dn10 = assign102070_e154036_d_n10;
        locals.var_qbd_nqs_dn11 = assign102070_e154036_d_n11;
        locals.var_qbd_nqs_dn14 = assign102070_e154036_d_n14;
        locals.var_qbd_nqs_rv = 0.0;

        let assign102080_e154039: f64 = if p.p22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2335 = assign102080_e154039;
        locals.var_guard2335_rv = 0.0;

        let (assign102090_e154053, assign102090_e154053_d_n0, assign102090_e154053_d_n2, assign102090_e154053_d_n4, assign102090_e154053_d_n5, assign102090_e154053_d_n6, assign102090_e154053_d_n7, assign102090_e154053_d_n8, assign102090_e154053_d_n9, assign102090_e154053_d_n10, assign102090_e154053_d_n11, assign102090_e154053_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102090_e154044: f64 = (locals.var_qgbo - locals.var_qovd);
        let assign102090_e154046: f64 = (assign102090_e154044 - locals.var_qovs);
        let assign102090_e154048: f64 = (assign102090_e154046 + locals.var_qgos);
        let assign102090_e154050: f64 = (assign102090_e154048 + locals.var_qgod);
        let assign102090_e154051: f64 = (locals.var_mfactor * assign102090_e154050);
        (assign102090_e154051, (locals.var_mfactor * ((((-locals.var_qovd_dn0) - locals.var_qovs_dn0) + locals.var_qgos_dn0) + locals.var_qgod_dn0)), (locals.var_mfactor * ((((-locals.var_qovd_dn2) - locals.var_qovs_dn2) + locals.var_qgos_dn2) + locals.var_qgod_dn2)), (locals.var_mfactor * ((((-locals.var_qovd_dn4) - locals.var_qovs_dn4) + locals.var_qgos_dn4) + locals.var_qgod_dn4)), (locals.var_mfactor * ((((-locals.var_qovd_dn5) - locals.var_qovs_dn5) + locals.var_qgos_dn5) + locals.var_qgod_dn5)), (locals.var_mfactor * ((((-locals.var_qovd_dn6) - locals.var_qovs_dn6) + locals.var_qgos_dn6) + locals.var_qgod_dn6)), (locals.var_mfactor * ((((locals.var_qgbo_dn7 - locals.var_qovd_dn7) - locals.var_qovs_dn7) + locals.var_qgos_dn7) + locals.var_qgod_dn7)), (locals.var_mfactor * ((((locals.var_qgbo_dn8 - locals.var_qovd_dn8) - locals.var_qovs_dn8) + locals.var_qgos_dn8) + locals.var_qgod_dn8)), (locals.var_mfactor * ((((locals.var_qgbo_dn9 - locals.var_qovd_dn9) - locals.var_qovs_dn9) + locals.var_qgos_dn9) + locals.var_qgod_dn9)), (locals.var_mfactor * ((((-locals.var_qovd_dn10) - locals.var_qovs_dn10) + locals.var_qgos_dn10) + locals.var_qgod_dn10)), (locals.var_mfactor * ((((-locals.var_qovd_dn11) - locals.var_qovs_dn11) + locals.var_qgos_dn11) + locals.var_qgod_dn11)), (locals.var_mfactor * ((((-locals.var_qovd_dn14) - locals.var_qovs_dn14) + locals.var_qgos_dn14) + locals.var_qgod_dn14)),)
    } else {
        (locals.var_qgov, locals.var_qgov_dn0, locals.var_qgov_dn2, locals.var_qgov_dn4, locals.var_qgov_dn5, locals.var_qgov_dn6, locals.var_qgov_dn7, locals.var_qgov_dn8, locals.var_qgov_dn9, locals.var_qgov_dn10, locals.var_qgov_dn11, locals.var_qgov_dn14,)
    }
};
        locals.var_qgov = assign102090_e154053;
        locals.var_qgov_dn0 = assign102090_e154053_d_n0;
        locals.var_qgov_dn2 = assign102090_e154053_d_n2;
        locals.var_qgov_dn4 = assign102090_e154053_d_n4;
        locals.var_qgov_dn5 = assign102090_e154053_d_n5;
        locals.var_qgov_dn6 = assign102090_e154053_d_n6;
        locals.var_qgov_dn7 = assign102090_e154053_d_n7;
        locals.var_qgov_dn8 = assign102090_e154053_d_n8;
        locals.var_qgov_dn9 = assign102090_e154053_d_n9;
        locals.var_qgov_dn10 = assign102090_e154053_d_n10;
        locals.var_qgov_dn11 = assign102090_e154053_d_n11;
        locals.var_qgov_dn14 = assign102090_e154053_d_n14;
        locals.var_qgov_rv = 0.0;

        let (assign102100_e154062, assign102100_e154062_d_n0, assign102100_e154062_d_n2, assign102100_e154062_d_n4, assign102100_e154062_d_n5, assign102100_e154062_d_n6, assign102100_e154062_d_n7, assign102100_e154062_d_n8, assign102100_e154062_d_n9, assign102100_e154062_d_n10, assign102100_e154062_d_n11, assign102100_e154062_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102100_e154057: f64 = locals.var_qbdld;
        let assign102100_e154059: f64 = (assign102100_e154057 - locals.var_qgod);
        let assign102100_e154060: f64 = (locals.var_mfactor * assign102100_e154059);
        (assign102100_e154060, (locals.var_mfactor * (locals.var_qbdld_dn0 - locals.var_qgod_dn0)), (locals.var_mfactor * (locals.var_qbdld_dn2 - locals.var_qgod_dn2)), (locals.var_mfactor * (locals.var_qbdld_dn4 - locals.var_qgod_dn4)), (locals.var_mfactor * (locals.var_qbdld_dn5 - locals.var_qgod_dn5)), (locals.var_mfactor * (locals.var_qbdld_dn6 - locals.var_qgod_dn6)), (locals.var_mfactor * (locals.var_qbdld_dn7 - locals.var_qgod_dn7)), (locals.var_mfactor * (locals.var_qbdld_dn8 - locals.var_qgod_dn8)), (locals.var_mfactor * (locals.var_qbdld_dn9 - locals.var_qgod_dn9)), (locals.var_mfactor * (locals.var_qbdld_dn10 - locals.var_qgod_dn10)), (locals.var_mfactor * (locals.var_qbdld_dn11 - locals.var_qgod_dn11)), (locals.var_mfactor * (locals.var_qbdld_dn14 - locals.var_qgod_dn14)),)
    } else {
        (locals.var_qdov, locals.var_qdov_dn0, locals.var_qdov_dn2, locals.var_qdov_dn4, locals.var_qdov_dn5, locals.var_qdov_dn6, locals.var_qdov_dn7, locals.var_qdov_dn8, locals.var_qdov_dn9, locals.var_qdov_dn10, locals.var_qdov_dn11, locals.var_qdov_dn14,)
    }
};
        locals.var_qdov = assign102100_e154062;
        locals.var_qdov_dn0 = assign102100_e154062_d_n0;
        locals.var_qdov_dn2 = assign102100_e154062_d_n2;
        locals.var_qdov_dn4 = assign102100_e154062_d_n4;
        locals.var_qdov_dn5 = assign102100_e154062_d_n5;
        locals.var_qdov_dn6 = assign102100_e154062_d_n6;
        locals.var_qdov_dn7 = assign102100_e154062_d_n7;
        locals.var_qdov_dn8 = assign102100_e154062_d_n8;
        locals.var_qdov_dn9 = assign102100_e154062_d_n9;
        locals.var_qdov_dn10 = assign102100_e154062_d_n10;
        locals.var_qdov_dn11 = assign102100_e154062_d_n11;
        locals.var_qdov_dn14 = assign102100_e154062_d_n14;
        locals.var_qdov_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_392(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign102110_e154071, assign102110_e154071_d_n0, assign102110_e154071_d_n2, assign102110_e154071_d_n4, assign102110_e154071_d_n5, assign102110_e154071_d_n6, assign102110_e154071_d_n7, assign102110_e154071_d_n8, assign102110_e154071_d_n9, assign102110_e154071_d_n10, assign102110_e154071_d_n11, assign102110_e154071_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102110_e154066: f64 = locals.var_qbsld;
        let assign102110_e154068: f64 = (assign102110_e154066 - locals.var_qgos);
        let assign102110_e154069: f64 = (locals.var_mfactor * assign102110_e154068);
        (assign102110_e154069, (locals.var_mfactor * (locals.var_qbsld_dn0 - locals.var_qgos_dn0)), (locals.var_mfactor * (locals.var_qbsld_dn2 - locals.var_qgos_dn2)), (locals.var_mfactor * (locals.var_qbsld_dn4 - locals.var_qgos_dn4)), (locals.var_mfactor * (locals.var_qbsld_dn5 - locals.var_qgos_dn5)), (locals.var_mfactor * (locals.var_qbsld_dn6 - locals.var_qgos_dn6)), (locals.var_mfactor * (locals.var_qbsld_dn7 - locals.var_qgos_dn7)), (locals.var_mfactor * (locals.var_qbsld_dn8 - locals.var_qgos_dn8)), (locals.var_mfactor * (locals.var_qbsld_dn9 - locals.var_qgos_dn9)), (locals.var_mfactor * (locals.var_qbsld_dn10 - locals.var_qgos_dn10)), (locals.var_mfactor * (locals.var_qbsld_dn11 - locals.var_qgos_dn11)), (locals.var_mfactor * (locals.var_qbsld_dn14 - locals.var_qgos_dn14)),)
    } else {
        (locals.var_qsov, locals.var_qsov_dn0, locals.var_qsov_dn2, locals.var_qsov_dn4, locals.var_qsov_dn5, locals.var_qsov_dn6, locals.var_qsov_dn7, locals.var_qsov_dn8, locals.var_qsov_dn9, locals.var_qsov_dn10, locals.var_qsov_dn11, locals.var_qsov_dn14,)
    }
};
        locals.var_qsov = assign102110_e154071;
        locals.var_qsov_dn0 = assign102110_e154071_d_n0;
        locals.var_qsov_dn2 = assign102110_e154071_d_n2;
        locals.var_qsov_dn4 = assign102110_e154071_d_n4;
        locals.var_qsov_dn5 = assign102110_e154071_d_n5;
        locals.var_qsov_dn6 = assign102110_e154071_d_n6;
        locals.var_qsov_dn7 = assign102110_e154071_d_n7;
        locals.var_qsov_dn8 = assign102110_e154071_d_n8;
        locals.var_qsov_dn9 = assign102110_e154071_d_n9;
        locals.var_qsov_dn10 = assign102110_e154071_d_n10;
        locals.var_qsov_dn11 = assign102110_e154071_d_n11;
        locals.var_qsov_dn14 = assign102110_e154071_d_n14;
        locals.var_qsov_rv = 0.0;

        let (assign102120_e154084, assign102120_e154084_d_n0, assign102120_e154084_d_n2, assign102120_e154084_d_n4, assign102120_e154084_d_n5, assign102120_e154084_d_n6, assign102120_e154084_d_n7, assign102120_e154084_d_n8, assign102120_e154084_d_n9, assign102120_e154084_d_n10, assign102120_e154084_d_n11, assign102120_e154084_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102120_e154076: f64 = locals.var_qy;
        let assign102120_e154078: f64 = (assign102120_e154076 - locals.var_qovd_add);
        let assign102120_e154080: f64 = (assign102120_e154078 - locals.var_qovs_add);
        let assign102120_e154081: f64 = (locals.var_mfactor * assign102120_e154080);
        let assign102120_e154082: f64 = (locals.var_qge + assign102120_e154081);
        (assign102120_e154082, (locals.var_qge_dn0 + (locals.var_mfactor * ((locals.var_qy_dn0 - locals.var_qovd_add_dn0) - locals.var_qovs_add_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((locals.var_qy_dn2 - locals.var_qovd_add_dn2) - locals.var_qovs_add_dn2))), (locals.var_qge_dn4 + (locals.var_mfactor * ((locals.var_qy_dn4 - locals.var_qovd_add_dn4) - locals.var_qovs_add_dn4))), (locals.var_qge_dn5 + (locals.var_mfactor * ((locals.var_qy_dn5 - locals.var_qovd_add_dn5) - locals.var_qovs_add_dn5))), (locals.var_qge_dn6 + (locals.var_mfactor * ((locals.var_qy_dn6 - locals.var_qovd_add_dn6) - locals.var_qovs_add_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((locals.var_qy_dn7 - locals.var_qovd_add_dn7) - locals.var_qovs_add_dn7))), (locals.var_qge_dn8 + (locals.var_mfactor * ((locals.var_qy_dn8 - locals.var_qovd_add_dn8) - locals.var_qovs_add_dn8))), (locals.var_qge_dn9 + (locals.var_mfactor * ((locals.var_qy_dn9 - locals.var_qovd_add_dn9) - locals.var_qovs_add_dn9))), (locals.var_qge_dn10 + (locals.var_mfactor * ((locals.var_qy_dn10 - locals.var_qovd_add_dn10) - locals.var_qovs_add_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((locals.var_qy_dn11 - locals.var_qovd_add_dn11) - locals.var_qovs_add_dn11))), (locals.var_qge_dn14 + (locals.var_mfactor * ((locals.var_qy_dn14 - locals.var_qovd_add_dn14) - locals.var_qovs_add_dn14))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign102120_e154084;
        locals.var_qge_dn0 = assign102120_e154084_d_n0;
        locals.var_qge_dn2 = assign102120_e154084_d_n2;
        locals.var_qge_dn4 = assign102120_e154084_d_n4;
        locals.var_qge_dn5 = assign102120_e154084_d_n5;
        locals.var_qge_dn6 = assign102120_e154084_d_n6;
        locals.var_qge_dn7 = assign102120_e154084_d_n7;
        locals.var_qge_dn8 = assign102120_e154084_d_n8;
        locals.var_qge_dn9 = assign102120_e154084_d_n9;
        locals.var_qge_dn10 = assign102120_e154084_d_n10;
        locals.var_qge_dn11 = assign102120_e154084_d_n11;
        locals.var_qge_dn14 = assign102120_e154084_d_n14;
        locals.var_qge_rv = 0.0;

        let (assign102130_e154095, assign102130_e154095_d_n0, assign102130_e154095_d_n2, assign102130_e154095_d_n4, assign102130_e154095_d_n5, assign102130_e154095_d_n6, assign102130_e154095_d_n7, assign102130_e154095_d_n8, assign102130_e154095_d_n9, assign102130_e154095_d_n10, assign102130_e154095_d_n11, assign102130_e154095_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102130_e154089: f64 = (-locals.var_qy);
        let assign102130_e154091: f64 = (assign102130_e154089 + locals.var_qbdld_add);
        let assign102130_e154092: f64 = (locals.var_mfactor * assign102130_e154091);
        let assign102130_e154093: f64 = (locals.var_qde + assign102130_e154092);
        (assign102130_e154093, (locals.var_qde_dn0 + (locals.var_mfactor * ((-locals.var_qy_dn0) + locals.var_qbdld_add_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((-locals.var_qy_dn2) + locals.var_qbdld_add_dn2))), (locals.var_qde_dn4 + (locals.var_mfactor * ((-locals.var_qy_dn4) + locals.var_qbdld_add_dn4))), (locals.var_qde_dn5 + (locals.var_mfactor * ((-locals.var_qy_dn5) + locals.var_qbdld_add_dn5))), (locals.var_qde_dn6 + (locals.var_mfactor * ((-locals.var_qy_dn6) + locals.var_qbdld_add_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((-locals.var_qy_dn7) + locals.var_qbdld_add_dn7))), (locals.var_qde_dn8 + (locals.var_mfactor * ((-locals.var_qy_dn8) + locals.var_qbdld_add_dn8))), (locals.var_qde_dn9 + (locals.var_mfactor * ((-locals.var_qy_dn9) + locals.var_qbdld_add_dn9))), (locals.var_qde_dn10 + (locals.var_mfactor * ((-locals.var_qy_dn10) + locals.var_qbdld_add_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((-locals.var_qy_dn11) + locals.var_qbdld_add_dn11))), (locals.var_qde_dn14 + (locals.var_mfactor * ((-locals.var_qy_dn14) + locals.var_qbdld_add_dn14))),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign102130_e154095;
        locals.var_qde_dn0 = assign102130_e154095_d_n0;
        locals.var_qde_dn2 = assign102130_e154095_d_n2;
        locals.var_qde_dn4 = assign102130_e154095_d_n4;
        locals.var_qde_dn5 = assign102130_e154095_d_n5;
        locals.var_qde_dn6 = assign102130_e154095_d_n6;
        locals.var_qde_dn7 = assign102130_e154095_d_n7;
        locals.var_qde_dn8 = assign102130_e154095_d_n8;
        locals.var_qde_dn9 = assign102130_e154095_d_n9;
        locals.var_qde_dn10 = assign102130_e154095_d_n10;
        locals.var_qde_dn11 = assign102130_e154095_d_n11;
        locals.var_qde_dn14 = assign102130_e154095_d_n14;
        locals.var_qde_rv = 0.0;

        let (assign102140_e154104, assign102140_e154104_d_n0, assign102140_e154104_d_n2, assign102140_e154104_d_n4, assign102140_e154104_d_n5, assign102140_e154104_d_n6, assign102140_e154104_d_n7, assign102140_e154104_d_n8, assign102140_e154104_d_n9, assign102140_e154104_d_n10, assign102140_e154104_d_n11, assign102140_e154104_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102140_e154100: f64 = locals.var_qbsld_add;
        let assign102140_e154101: f64 = (locals.var_mfactor * assign102140_e154100);
        let assign102140_e154102: f64 = (locals.var_qse + assign102140_e154101);
        (assign102140_e154102, (locals.var_qse_dn0 + (locals.var_mfactor * locals.var_qbsld_add_dn0)), (locals.var_qse_dn2 + (locals.var_mfactor * locals.var_qbsld_add_dn2)), (locals.var_qse_dn4 + (locals.var_mfactor * locals.var_qbsld_add_dn4)), (locals.var_qse_dn5 + (locals.var_mfactor * locals.var_qbsld_add_dn5)), (locals.var_qse_dn6 + (locals.var_mfactor * locals.var_qbsld_add_dn6)), (locals.var_qse_dn7 + (locals.var_mfactor * locals.var_qbsld_add_dn7)), (locals.var_qse_dn8 + (locals.var_mfactor * locals.var_qbsld_add_dn8)), (locals.var_qse_dn9 + (locals.var_mfactor * locals.var_qbsld_add_dn9)), (locals.var_qse_dn10 + (locals.var_mfactor * locals.var_qbsld_add_dn10)), (locals.var_qse_dn11 + (locals.var_mfactor * locals.var_qbsld_add_dn11)), (locals.var_qse_dn14 + (locals.var_mfactor * locals.var_qbsld_add_dn14)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign102140_e154104;
        locals.var_qse_dn0 = assign102140_e154104_d_n0;
        locals.var_qse_dn2 = assign102140_e154104_d_n2;
        locals.var_qse_dn4 = assign102140_e154104_d_n4;
        locals.var_qse_dn5 = assign102140_e154104_d_n5;
        locals.var_qse_dn6 = assign102140_e154104_d_n6;
        locals.var_qse_dn7 = assign102140_e154104_d_n7;
        locals.var_qse_dn8 = assign102140_e154104_d_n8;
        locals.var_qse_dn9 = assign102140_e154104_d_n9;
        locals.var_qse_dn10 = assign102140_e154104_d_n10;
        locals.var_qse_dn11 = assign102140_e154104_d_n11;
        locals.var_qse_dn14 = assign102140_e154104_d_n14;
        locals.var_qse_rv = 0.0;

        let (assign102150_e154113, assign102150_e154113_d_n0, assign102150_e154113_d_n2, assign102150_e154113_d_n4, assign102150_e154113_d_n5, assign102150_e154113_d_n6, assign102150_e154113_d_n7, assign102150_e154113_d_n8, assign102150_e154113_d_n9, assign102150_e154113_d_n10, assign102150_e154113_d_n11, assign102150_e154113_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102150_e154108: f64 = (-locals.var_qovdext);
        let assign102150_e154110: f64 = (assign102150_e154108 - locals.var_qovsext);
        let assign102150_e154111: f64 = (locals.var_mfactor * assign102150_e154110);
        (assign102150_e154111, (locals.var_mfactor * ((-locals.var_qovdext_dn0) - locals.var_qovsext_dn0)), (locals.var_mfactor * ((-locals.var_qovdext_dn2) - locals.var_qovsext_dn2)), (locals.var_mfactor * ((-locals.var_qovdext_dn4) - locals.var_qovsext_dn4)), (locals.var_mfactor * ((-locals.var_qovdext_dn5) - locals.var_qovsext_dn5)), (locals.var_mfactor * ((-locals.var_qovdext_dn6) - locals.var_qovsext_dn6)), (locals.var_mfactor * ((-locals.var_qovdext_dn7) - locals.var_qovsext_dn7)), (locals.var_mfactor * ((-locals.var_qovdext_dn8) - locals.var_qovsext_dn8)), (locals.var_mfactor * ((-locals.var_qovdext_dn9) - locals.var_qovsext_dn9)), (locals.var_mfactor * ((-locals.var_qovdext_dn10) - locals.var_qovsext_dn10)), (locals.var_mfactor * ((-locals.var_qovdext_dn11) - locals.var_qovsext_dn11)), (locals.var_mfactor * ((-locals.var_qovdext_dn14) - locals.var_qovsext_dn14)),)
    } else {
        (locals.var_qgexte, locals.var_qgexte_dn0, locals.var_qgexte_dn2, locals.var_qgexte_dn4, locals.var_qgexte_dn5, locals.var_qgexte_dn6, locals.var_qgexte_dn7, locals.var_qgexte_dn8, locals.var_qgexte_dn9, locals.var_qgexte_dn10, locals.var_qgexte_dn11, locals.var_qgexte_dn14,)
    }
};
        locals.var_qgexte = assign102150_e154113;
        locals.var_qgexte_dn0 = assign102150_e154113_d_n0;
        locals.var_qgexte_dn2 = assign102150_e154113_d_n2;
        locals.var_qgexte_dn4 = assign102150_e154113_d_n4;
        locals.var_qgexte_dn5 = assign102150_e154113_d_n5;
        locals.var_qgexte_dn6 = assign102150_e154113_d_n6;
        locals.var_qgexte_dn7 = assign102150_e154113_d_n7;
        locals.var_qgexte_dn8 = assign102150_e154113_d_n8;
        locals.var_qgexte_dn9 = assign102150_e154113_d_n9;
        locals.var_qgexte_dn10 = assign102150_e154113_d_n10;
        locals.var_qgexte_dn11 = assign102150_e154113_d_n11;
        locals.var_qgexte_dn14 = assign102150_e154113_d_n14;
        locals.var_qgexte_rv = 0.0;

        let (assign102160_e154119, assign102160_e154119_d_n0, assign102160_e154119_d_n2, assign102160_e154119_d_n4, assign102160_e154119_d_n5, assign102160_e154119_d_n6, assign102160_e154119_d_n7, assign102160_e154119_d_n8, assign102160_e154119_d_n9, assign102160_e154119_d_n10, assign102160_e154119_d_n11, assign102160_e154119_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102160_e154117: f64 = (locals.var_mfactor * locals.var_qbdldext);
        (assign102160_e154117, (locals.var_mfactor * locals.var_qbdldext_dn0), (locals.var_mfactor * locals.var_qbdldext_dn2), (locals.var_mfactor * locals.var_qbdldext_dn4), (locals.var_mfactor * locals.var_qbdldext_dn5), (locals.var_mfactor * locals.var_qbdldext_dn6), (locals.var_mfactor * locals.var_qbdldext_dn7), (locals.var_mfactor * locals.var_qbdldext_dn8), (locals.var_mfactor * locals.var_qbdldext_dn9), (locals.var_mfactor * locals.var_qbdldext_dn10), (locals.var_mfactor * locals.var_qbdldext_dn11), (locals.var_mfactor * locals.var_qbdldext_dn14),)
    } else {
        (locals.var_qdexte, locals.var_qdexte_dn0, locals.var_qdexte_dn2, locals.var_qdexte_dn4, locals.var_qdexte_dn5, locals.var_qdexte_dn6, locals.var_qdexte_dn7, locals.var_qdexte_dn8, locals.var_qdexte_dn9, locals.var_qdexte_dn10, locals.var_qdexte_dn11, locals.var_qdexte_dn14,)
    }
};
        locals.var_qdexte = assign102160_e154119;
        locals.var_qdexte_dn0 = assign102160_e154119_d_n0;
        locals.var_qdexte_dn2 = assign102160_e154119_d_n2;
        locals.var_qdexte_dn4 = assign102160_e154119_d_n4;
        locals.var_qdexte_dn5 = assign102160_e154119_d_n5;
        locals.var_qdexte_dn6 = assign102160_e154119_d_n6;
        locals.var_qdexte_dn7 = assign102160_e154119_d_n7;
        locals.var_qdexte_dn8 = assign102160_e154119_d_n8;
        locals.var_qdexte_dn9 = assign102160_e154119_d_n9;
        locals.var_qdexte_dn10 = assign102160_e154119_d_n10;
        locals.var_qdexte_dn11 = assign102160_e154119_d_n11;
        locals.var_qdexte_dn14 = assign102160_e154119_d_n14;
        locals.var_qdexte_rv = 0.0;

        let (assign102170_e154125, assign102170_e154125_d_n0, assign102170_e154125_d_n2, assign102170_e154125_d_n4, assign102170_e154125_d_n5, assign102170_e154125_d_n6, assign102170_e154125_d_n7, assign102170_e154125_d_n8, assign102170_e154125_d_n9, assign102170_e154125_d_n10, assign102170_e154125_d_n11, assign102170_e154125_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102170_e154123: f64 = (locals.var_mfactor * locals.var_qbsldext);
        (assign102170_e154123, (locals.var_mfactor * locals.var_qbsldext_dn0), (locals.var_mfactor * locals.var_qbsldext_dn2), (locals.var_mfactor * locals.var_qbsldext_dn4), (locals.var_mfactor * locals.var_qbsldext_dn5), (locals.var_mfactor * locals.var_qbsldext_dn6), (locals.var_mfactor * locals.var_qbsldext_dn7), (locals.var_mfactor * locals.var_qbsldext_dn8), (locals.var_mfactor * locals.var_qbsldext_dn9), (locals.var_mfactor * locals.var_qbsldext_dn10), (locals.var_mfactor * locals.var_qbsldext_dn11), (locals.var_mfactor * locals.var_qbsldext_dn14),)
    } else {
        (locals.var_qsexte, locals.var_qsexte_dn0, locals.var_qsexte_dn2, locals.var_qsexte_dn4, locals.var_qsexte_dn5, locals.var_qsexte_dn6, locals.var_qsexte_dn7, locals.var_qsexte_dn8, locals.var_qsexte_dn9, locals.var_qsexte_dn10, locals.var_qsexte_dn11, locals.var_qsexte_dn14,)
    }
};
        locals.var_qsexte = assign102170_e154125;
        locals.var_qsexte_dn0 = assign102170_e154125_d_n0;
        locals.var_qsexte_dn2 = assign102170_e154125_d_n2;
        locals.var_qsexte_dn4 = assign102170_e154125_d_n4;
        locals.var_qsexte_dn5 = assign102170_e154125_d_n5;
        locals.var_qsexte_dn6 = assign102170_e154125_d_n6;
        locals.var_qsexte_dn7 = assign102170_e154125_d_n7;
        locals.var_qsexte_dn8 = assign102170_e154125_d_n8;
        locals.var_qsexte_dn9 = assign102170_e154125_d_n9;
        locals.var_qsexte_dn10 = assign102170_e154125_d_n10;
        locals.var_qsexte_dn11 = assign102170_e154125_d_n11;
        locals.var_qsexte_dn14 = assign102170_e154125_d_n14;
        locals.var_qsexte_rv = 0.0;

        let (assign102180_e154136, assign102180_e154136_d_n0, assign102180_e154136_d_n2, assign102180_e154136_d_n7,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102180_e154130: f64 = (-locals.var_qfd);
        let assign102180_e154132: f64 = (assign102180_e154130 - locals.var_qgdo);
        let assign102180_e154133: f64 = (locals.var_mfactor * assign102180_e154132);
        let assign102180_e154134: f64 = (locals.var_qdp + assign102180_e154133);
        (assign102180_e154134, (locals.var_qdp_dn0 + (locals.var_mfactor * ((-locals.var_qfd_dn0) - locals.var_qgdo_dn0))), (locals.var_qdp_dn2 + (locals.var_mfactor * ((-locals.var_qfd_dn2) - locals.var_qgdo_dn2))), (locals.var_qdp_dn7 + (locals.var_mfactor * ((-locals.var_qfd_dn7) - locals.var_qgdo_dn7))),)
    } else {
        (locals.var_qdp, locals.var_qdp_dn0, locals.var_qdp_dn2, locals.var_qdp_dn7,)
    }
};
        locals.var_qdp = assign102180_e154136;
        locals.var_qdp_dn0 = assign102180_e154136_d_n0;
        locals.var_qdp_dn2 = assign102180_e154136_d_n2;
        locals.var_qdp_dn7 = assign102180_e154136_d_n7;
        locals.var_qdp_rv = 0.0;

        let (assign102190_e154147, assign102190_e154147_d_n2, assign102190_e154147_d_n7,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102190_e154141: f64 = (-locals.var_qfs);
        let assign102190_e154143: f64 = (assign102190_e154141 - locals.var_qgso);
        let assign102190_e154144: f64 = (locals.var_mfactor * assign102190_e154143);
        let assign102190_e154145: f64 = (locals.var_qsp + assign102190_e154144);
        (assign102190_e154145, (locals.var_qsp_dn2 + (locals.var_mfactor * ((-locals.var_qfs_dn2) - locals.var_qgso_dn2))), (locals.var_qsp_dn7 + (locals.var_mfactor * ((-locals.var_qfs_dn7) - locals.var_qgso_dn7))),)
    } else {
        (locals.var_qsp, locals.var_qsp_dn2, locals.var_qsp_dn7,)
    }
};
        locals.var_qsp = assign102190_e154147;
        locals.var_qsp_dn2 = assign102190_e154147_d_n2;
        locals.var_qsp_dn7 = assign102190_e154147_d_n7;
        locals.var_qsp_rv = 0.0;

        let assign102200_e154151: f64 = (locals.var_isub + locals.var_isubibpc);
        let assign102200_e154152: f64 = (locals.var_mfactor * assign102200_e154151);
        locals.var_isube = assign102200_e154152;
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

        let assign102210_e154155: f64 = (locals.var_mfactor * locals.var_isubld);
        locals.var_isublde = assign102210_e154155;
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

        let assign102330_e154222: f64 = (4.0 * 1.3806226e-23);
        let assign102330_e154224: f64 = (assign102330_e154222 * locals.var_ttemp);
        let assign102330_e154226: f64 = assign102330_e154224;
        locals.var_whi_noise = assign102330_e154226;
        locals.var_whi_noise_dn0 = (assign102330_e154222 * locals.var_ttemp_dn0);
        locals.var_whi_noise_dn2 = (assign102330_e154222 * locals.var_ttemp_dn2);
        locals.var_whi_noise_dn4 = (assign102330_e154222 * locals.var_ttemp_dn4);
        locals.var_whi_noise_dn5 = (assign102330_e154222 * locals.var_ttemp_dn5);
        locals.var_whi_noise_dn6 = (assign102330_e154222 * locals.var_ttemp_dn6);
        locals.var_whi_noise_dn7 = (assign102330_e154222 * locals.var_ttemp_dn7);
        locals.var_whi_noise_dn8 = (assign102330_e154222 * locals.var_ttemp_dn8);
        locals.var_whi_noise_dn9 = (assign102330_e154222 * locals.var_ttemp_dn9);
        locals.var_whi_noise_dn10 = (assign102330_e154222 * locals.var_ttemp_dn10);
        locals.var_whi_noise_dn11 = (assign102330_e154222 * locals.var_ttemp_dn11);
        locals.var_whi_noise_dn14 = (assign102330_e154222 * locals.var_ttemp_dn14);
        locals.var_whi_noise_rv = 0.0;

        let assign102350_e154232: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign102350_e154232;
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

        let assign102360_e154235: f64 = locals.var_qge_dn6;
        locals.var_cgdbd = assign102360_e154235;
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

        let assign102370_e154238: f64 = (p.p87 * locals.var_cgdbd);
        locals.var_cgdbd = assign102370_e154238;
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

        let assign102380_e154241: f64 = locals.var_qge_dn8;
        locals.var_cgsbd = assign102380_e154241;
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

        let assign102390_e154244: f64 = (p.p87 * locals.var_cgsbd);
        locals.var_cgsbd = assign102390_e154244;
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

        let (assign102400_e154250, assign102400_e154250_d_n0, assign102400_e154250_d_n2, assign102400_e154250_d_n4, assign102400_e154250_d_n5, assign102400_e154250_d_n6, assign102400_e154250_d_n7, assign102400_e154250_d_n8, assign102400_e154250_d_n9, assign102400_e154250_d_n10, assign102400_e154250_d_n11, assign102400_e154250_d_n14,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14,)
    }
};
        locals.var_cgsb = assign102400_e154250;
        locals.var_cgsb_dn0 = assign102400_e154250_d_n0;
        locals.var_cgsb_dn2 = assign102400_e154250_d_n2;
        locals.var_cgsb_dn4 = assign102400_e154250_d_n4;
        locals.var_cgsb_dn5 = assign102400_e154250_d_n5;
        locals.var_cgsb_dn6 = assign102400_e154250_d_n6;
        locals.var_cgsb_dn7 = assign102400_e154250_d_n7;
        locals.var_cgsb_dn8 = assign102400_e154250_d_n8;
        locals.var_cgsb_dn9 = assign102400_e154250_d_n9;
        locals.var_cgsb_dn10 = assign102400_e154250_d_n10;
        locals.var_cgsb_dn11 = assign102400_e154250_d_n11;
        locals.var_cgsb_dn14 = assign102400_e154250_d_n14;
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

        let assign102430_e154270: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2338 = assign102430_e154270;
        locals.var_guard2338_rv = 0.0;

        let (assign102440_e154280, assign102440_e154280_d_n0, assign102440_e154280_d_n2, assign102440_e154280_d_n4, assign102440_e154280_d_n5, assign102440_e154280_d_n6, assign102440_e154280_d_n7, assign102440_e154280_d_n8, assign102440_e154280_d_n9, assign102440_e154280_d_n10, assign102440_e154280_d_n11, assign102440_e154280_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102440_e154274: f64 = (1e-6 * locals.var_cox);
        let assign102440_e154276: f64 = (assign102440_e154274 * locals.var_weffcv_nf);
        let assign102440_e154278: f64 = (assign102440_e154276 * locals.var_leff);
        (assign102440_e154278, (((1e-6 * locals.var_cox_dn0) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn2) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn4) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn5) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn6) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn7) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn8) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn9) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn10) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn11) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn14) * locals.var_weffcv_nf) * locals.var_leff),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign102440_e154280;
        locals.var_t0_dn0 = assign102440_e154280_d_n0;
        locals.var_t0_dn2 = assign102440_e154280_d_n2;
        locals.var_t0_dn4 = assign102440_e154280_d_n4;
        locals.var_t0_dn5 = assign102440_e154280_d_n5;
        locals.var_t0_dn6 = assign102440_e154280_d_n6;
        locals.var_t0_dn7 = assign102440_e154280_d_n7;
        locals.var_t0_dn8 = assign102440_e154280_d_n8;
        locals.var_t0_dn9 = assign102440_e154280_d_n9;
        locals.var_t0_dn10 = assign102440_e154280_d_n10;
        locals.var_t0_dn11 = assign102440_e154280_d_n11;
        locals.var_t0_dn14 = assign102440_e154280_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign102450_e154286, assign102450_e154286_d_n0, assign102450_e154286_d_n2, assign102450_e154286_d_n4, assign102450_e154286_d_n5, assign102450_e154286_d_n6, assign102450_e154286_d_n7, assign102450_e154286_d_n8, assign102450_e154286_d_n9, assign102450_e154286_d_n10, assign102450_e154286_d_n11, assign102450_e154286_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102450_e154284: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign102450_e154284, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn4 / locals.var_mfactor), (locals.var_cgsb_dn5 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn8 / locals.var_mfactor), (locals.var_cgsb_dn9 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign102450_e154286;
        locals.var_t10_dn0 = assign102450_e154286_d_n0;
        locals.var_t10_dn2 = assign102450_e154286_d_n2;
        locals.var_t10_dn4 = assign102450_e154286_d_n4;
        locals.var_t10_dn5 = assign102450_e154286_d_n5;
        locals.var_t10_dn6 = assign102450_e154286_d_n6;
        locals.var_t10_dn7 = assign102450_e154286_d_n7;
        locals.var_t10_dn8 = assign102450_e154286_d_n8;
        locals.var_t10_dn9 = assign102450_e154286_d_n9;
        locals.var_t10_dn10 = assign102450_e154286_d_n10;
        locals.var_t10_dn11 = assign102450_e154286_d_n11;
        locals.var_t10_dn14 = assign102450_e154286_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign102460_e154300, assign102460_e154300_d_n0, assign102460_e154300_d_n2, assign102460_e154300_d_n4, assign102460_e154300_d_n5, assign102460_e154300_d_n6, assign102460_e154300_d_n7, assign102460_e154300_d_n8, assign102460_e154300_d_n9, assign102460_e154300_d_n10, assign102460_e154300_d_n11, assign102460_e154300_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102460_e154290: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign102460_e154292: f64 = (assign102460_e154290 * locals.var_beta_inv);
        let assign102460_e154294: f64 = (assign102460_e154292 * locals.var_t10);
        let assign102460_e154296: f64 = (assign102460_e154294 * locals.var_t10);
        let assign102460_e154298: f64 = (assign102460_e154296 / locals.var_gds0_ign);
        (assign102460_e154298, ((((((((assign102460_e154290 * locals.var_beta_inv_dn0) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn0)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn0)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn2) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn2)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn2)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn4) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn4)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn4)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn4)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn5) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn5)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn5)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn5)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn6) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn6)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn6)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn7) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn7)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn7)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn8) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn8)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn8)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn8)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn9) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn9)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn9)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn9)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn10) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn10)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn10)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn11) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn11)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn11)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn14) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn14)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn14)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn14)) / (locals.var_gds0_ign * locals.var_gds0_ign)),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn4, locals.var_nign0_dn5, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn8, locals.var_nign0_dn9, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn14,)
    }
};
        locals.var_nign0 = assign102460_e154300;
        locals.var_nign0_dn0 = assign102460_e154300_d_n0;
        locals.var_nign0_dn2 = assign102460_e154300_d_n2;
        locals.var_nign0_dn4 = assign102460_e154300_d_n4;
        locals.var_nign0_dn5 = assign102460_e154300_d_n5;
        locals.var_nign0_dn6 = assign102460_e154300_d_n6;
        locals.var_nign0_dn7 = assign102460_e154300_d_n7;
        locals.var_nign0_dn8 = assign102460_e154300_d_n8;
        locals.var_nign0_dn9 = assign102460_e154300_d_n9;
        locals.var_nign0_dn10 = assign102460_e154300_d_n10;
        locals.var_nign0_dn11 = assign102460_e154300_d_n11;
        locals.var_nign0_dn14 = assign102460_e154300_d_n14;
        locals.var_nign0_rv = 0.0;

        let assign102470_e154304: f64 = (10.0 * 2.220446049250313e-16);
        let assign102470_e154309: f64 = (10.0 * 2.220446049250313e-16);
        let assign102470_e154311: f64 = if ((locals.var_kusai00l > assign102470_e154304) && (locals.var_vds > assign102470_e154309)) { 1.0 } else { 0.0 };
        locals.var_guard2339 = assign102470_e154311;
        locals.var_guard2339_rv = 0.0;

        let (assign102480_e154319, assign102480_e154319_d_n0, assign102480_e154319_d_n2, assign102480_e154319_d_n4, assign102480_e154319_d_n5, assign102480_e154319_d_n6, assign102480_e154319_d_n7, assign102480_e154319_d_n8, assign102480_e154319_d_n9, assign102480_e154319_d_n10, assign102480_e154319_d_n11, assign102480_e154319_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 != 0.0)) {
        let assign102480_e154317: f64 = (locals.var_muun / locals.var_mu);
        (assign102480_e154317, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn4 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn4)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn5 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn5)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn8 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn8)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn9 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn9)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn14 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn14)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn4, locals.var_mumoda_dn5, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn8, locals.var_mumoda_dn9, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn14,)
    }
};
        locals.var_mumoda = assign102480_e154319;
        locals.var_mumoda_dn0 = assign102480_e154319_d_n0;
        locals.var_mumoda_dn2 = assign102480_e154319_d_n2;
        locals.var_mumoda_dn4 = assign102480_e154319_d_n4;
        locals.var_mumoda_dn5 = assign102480_e154319_d_n5;
        locals.var_mumoda_dn6 = assign102480_e154319_d_n6;
        locals.var_mumoda_dn7 = assign102480_e154319_d_n7;
        locals.var_mumoda_dn8 = assign102480_e154319_d_n8;
        locals.var_mumoda_dn9 = assign102480_e154319_d_n9;
        locals.var_mumoda_dn10 = assign102480_e154319_d_n10;
        locals.var_mumoda_dn11 = assign102480_e154319_d_n11;
        locals.var_mumoda_dn14 = assign102480_e154319_d_n14;
        locals.var_mumoda_rv = 0.0;

        let (assign102490_e154331, assign102490_e154331_d_n0, assign102490_e154331_d_n2, assign102490_e154331_d_n4, assign102490_e154331_d_n5, assign102490_e154331_d_n6, assign102490_e154331_d_n7, assign102490_e154331_d_n8, assign102490_e154331_d_n9, assign102490_e154331_d_n10, assign102490_e154331_d_n11, assign102490_e154331_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 != 0.0)) {
        let assign102490_e154325: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign102490_e154327: f64 = (assign102490_e154325 - locals.var_mumoda);
        let assign102490_e154329: f64 = (assign102490_e154327 / locals.var_vds);
        (assign102490_e154329, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn4) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn4)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn5) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn5)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn8) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn8)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn9) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn9)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn14 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn14)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn14) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn14)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn4, locals.var_mumodb_dn5, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn8, locals.var_mumodb_dn9, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn14,)
    }
};
        locals.var_mumodb = assign102490_e154331;
        locals.var_mumodb_dn0 = assign102490_e154331_d_n0;
        locals.var_mumodb_dn2 = assign102490_e154331_d_n2;
        locals.var_mumodb_dn4 = assign102490_e154331_d_n4;
        locals.var_mumodb_dn5 = assign102490_e154331_d_n5;
        locals.var_mumodb_dn6 = assign102490_e154331_d_n6;
        locals.var_mumodb_dn7 = assign102490_e154331_d_n7;
        locals.var_mumodb_dn8 = assign102490_e154331_d_n8;
        locals.var_mumodb_dn9 = assign102490_e154331_d_n9;
        locals.var_mumodb_dn10 = assign102490_e154331_d_n10;
        locals.var_mumodb_dn11 = assign102490_e154331_d_n11;
        locals.var_mumodb_dn14 = assign102490_e154331_d_n14;
        locals.var_mumodb_rv = 0.0;

    }
}
