#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti_d: f64,
        var_cbbtstid_i: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_ctatstid_i: f64,
        var_erfcpos: f64,
        var_erfcpos_dn5: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_fbbtsti_d: f64,
        var_fstopsti_d: f64,
        var_ftdgat_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard610: f64,
        var_guard614: f64,
        var_guard618: f64,
        var_idmult: f64,
        var_idsatgat_d: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lgdrain_i: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_pstid_i: f64,
        var_slopesti_d: f64,
        var_two_psistar: f64,
        var_v2: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbigat_d: f64,
        var_vbirgatinv_d: f64,
        var_vbirstid_i: f64,
        var_vbirstiinv_d: f64,
        var_vbrinvsti_d: f64,
        var_vbrstid_i: f64,
        var_vjsrh: f64,
        var_wdepnulrgat_d: f64,
        var_wdepnulrinvsti_d: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard620_slot: &mut f64,
        var_guard621_slot: &mut f64,
        var_guard622_slot: &mut f64,
        var_guard623_slot: &mut f64,
        var_guard624_slot: &mut f64,
        var_guard625_slot: &mut f64,
        var_guard626_slot: &mut f64,
        var_guard627_slot: &mut f64,
        var_guard628_slot: &mut f64,
        var_guard629_slot: &mut f64,
        var_guard630_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard620: f64 = *var_guard620_slot;
        let mut var_guard621: f64 = *var_guard621_slot;
        let mut var_guard622: f64 = *var_guard622_slot;
        let mut var_guard623: f64 = *var_guard623_slot;
        let mut var_guard624: f64 = *var_guard624_slot;
        let mut var_guard625: f64 = *var_guard625_slot;
        let mut var_guard626: f64 = *var_guard626_slot;
        let mut var_guard627: f64 = *var_guard627_slot;
        let mut var_guard628: f64 = *var_guard628_slot;
        let mut var_guard629: f64 = *var_guard629_slot;
        let mut var_guard630: f64 = *var_guard630_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign31690_e39413, assign31690_e39413_d_n5, assign31690_e39413_d_n6, assign31690_e39413_d_n7, assign31690_e39413_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) && (var_guard618 == 0.0)) {
        let assign31690_e39409: f64 = (2.0 * var_tmp);
        let assign31690_e39411: f64 = (assign31690_e39409 - var_erfcpos);
        (assign31690_e39411, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign31690_e39413;
        var_erfctimesexpmtat_dn5 = assign31690_e39413_d_n5;
        var_erfctimesexpmtat_dn6 = assign31690_e39413_d_n6;
        var_erfctimesexpmtat_dn7 = assign31690_e39413_d_n7;
        var_erfctimesexpmtat_dn8 = assign31690_e39413_d_n8;

        let (assign31700_e39433, assign31700_e39433_d_n5, assign31700_e39433_d_n6, assign31700_e39433_d_n7, assign31700_e39433_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31700_e39425: f64 = (1.772453850905516 * 0.5);
        let assign31700_e39428: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign31700_e39430: f64 = (assign31700_e39428 / var_ktat);
        let assign31700_e39431: f64 = (assign31700_e39425 * assign31700_e39430);
        (assign31700_e39431, (assign31700_e39425 * ((((var_atatsti_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign31700_e39428 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign31700_e39425 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign31700_e39428 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign31700_e39425 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign31700_e39428 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign31700_e39425 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign31700_e39428 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign31700_e39433;
        var_gammamax_dn5 = assign31700_e39433_d_n5;
        var_gammamax_dn6 = assign31700_e39433_d_n6;
        var_gammamax_dn7 = assign31700_e39433_d_n7;
        var_gammamax_dn8 = assign31700_e39433_d_n8;

        let (assign31710_e39451, assign31710_e39451_d_n5, assign31710_e39451_d_n6, assign31710_e39451_d_n7, assign31710_e39451_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31710_e39446: f64 = (var_asrh * var_gammamax);
        let assign31710_e39448: f64 = (assign31710_e39446 * var_wtat);
        let assign31710_e39449: f64 = (var_ctatstid_i * assign31710_e39448);
        (assign31710_e39449, (var_ctatstid_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign31710_e39446 * var_wtat_dn5))), (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign31710_e39446 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign31710_e39446 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign31710_e39446 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign31710_e39451;
        var_itat_dn5 = assign31710_e39451_d_n5;
        var_itat_dn6 = assign31710_e39451_d_n6;
        var_itat_dn7 = assign31710_e39451_d_n7;
        var_itat_dn8 = assign31710_e39451_d_n8;

        let assign31720_e39454: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard620 = assign31720_e39454;

        let (assign31730_e39465, assign31730_e39465_d_n5, assign31730_e39465_d_n6, assign31730_e39465_d_n7, assign31730_e39465_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard620 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign31730_e39465;
        var_ibbt_dn5 = assign31730_e39465_d_n5;
        var_ibbt_dn6 = assign31730_e39465_d_n6;
        var_ibbt_dn7 = assign31730_e39465_d_n7;
        var_ibbt_dn8 = assign31730_e39465_d_n8;

        let assign31740_e39468: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard621 = assign31740_e39468;

        let (assign31750_e39487, assign31750_e39487_d_n5, assign31750_e39487_d_n6, assign31750_e39487_d_n7, assign31750_e39487_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard620 == 0.0)) && (var_guard621 != 0.0)) {
        let assign31750_e39482: f64 = (var_vbirstid_i - var_vbbt);
        let assign31750_e39484: f64 = (assign31750_e39482 * var_vbirstiinv_d);
        let assign31750_e39485: f64 = (assign31750_e39484).sqrt();
        (assign31750_e39485, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31750_e39487;
        var_tmp_dn5 = assign31750_e39487_d_n5;
        var_tmp_dn6 = assign31750_e39487_d_n6;
        var_tmp_dn7 = assign31750_e39487_d_n7;
        var_tmp_dn8 = assign31750_e39487_d_n8;

        let (assign31760_e39508, assign31760_e39508_d_n5, assign31760_e39508_d_n6, assign31760_e39508_d_n7, assign31760_e39508_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard620 == 0.0)) && (var_guard621 == 0.0)) {
        let assign31760_e39502: f64 = (var_vbirstid_i - var_vbbt);
        let assign31760_e39504: f64 = (assign31760_e39502 * var_vbirstiinv_d);
        let assign31760_e39506: f64 = (assign31760_e39504).powf(var_pstid_i);
        (assign31760_e39506, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31760_e39508;
        var_tmp_dn5 = assign31760_e39508_d_n5;
        var_tmp_dn6 = assign31760_e39508_d_n6;
        var_tmp_dn7 = assign31760_e39508_d_n7;
        var_tmp_dn8 = assign31760_e39508_d_n8;

        let (assign31770_e39528, assign31770_e39528_d_n5, assign31770_e39528_d_n6, assign31770_e39528_d_n7, assign31770_e39528_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31770_e39521: f64 = (var_vbirstid_i - var_vbbt);
        let assign31770_e39523: f64 = (assign31770_e39521 * var_wdepnulrinvsti_d);
        let assign31770_e39525: f64 = (assign31770_e39523 / var_tmp);
        let assign31770_e39526: f64 = (var_one_over_one_minus_psti_d * assign31770_e39525);
        (assign31770_e39526, (var_one_over_one_minus_psti_d * (-((assign31770_e39523 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign31770_e39523 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign31770_e39523 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign31770_e39523 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign31770_e39528;
        var_fmaxr_dn5 = assign31770_e39528_d_n5;
        var_fmaxr_dn6 = assign31770_e39528_d_n6;
        var_fmaxr_dn7 = assign31770_e39528_d_n7;
        var_fmaxr_dn8 = assign31770_e39528_d_n8;

        let assign31780_e39530: f64 = (-var_fbbtsti_d);
        let assign31780_e39532: f64 = (assign31780_e39530 / var_fmaxr);
        let assign31780_e39533: f64 = (assign31780_e39532).abs();
        let assign31780_e39535: f64 = if assign31780_e39533 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard622 = assign31780_e39535;

        let (assign31790_e39553, assign31790_e39553_d_n5, assign31790_e39553_d_n6, assign31790_e39553_d_n7, assign31790_e39553_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard620 == 0.0)) && (var_guard622 != 0.0)) {
        let assign31790_e39548: f64 = (-var_fbbtsti_d);
        let assign31790_e39550: f64 = (assign31790_e39548 / var_fmaxr);
        let assign31790_e39551: f64 = (assign31790_e39550).exp();
        (assign31790_e39551, (assign31790_e39551 * (-((assign31790_e39548 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign31790_e39551 * (-((assign31790_e39548 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign31790_e39551 * (-((assign31790_e39548 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign31790_e39551 * (-((assign31790_e39548 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31790_e39553;
        var_tmp_dn5 = assign31790_e39553_d_n5;
        var_tmp_dn6 = assign31790_e39553_d_n6;
        var_tmp_dn7 = assign31790_e39553_d_n7;
        var_tmp_dn8 = assign31790_e39553_d_n8;

        let assign31800_e39555: f64 = (-var_fbbtsti_d);
        let assign31800_e39557: f64 = (assign31800_e39555 / var_fmaxr);
        let assign31800_e39559: f64 = if assign31800_e39557 < 0.0 { 1.0 } else { 0.0 };
        var_guard623 = assign31800_e39559;

        let (assign31810_e39610, assign31810_e39610_d_n5, assign31810_e39610_d_n6, assign31810_e39610_d_n7, assign31810_e39610_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard620 == 0.0)) && (var_guard622 == 0.0)) && (var_guard623 != 0.0)) {
        let assign31810_e39577: f64 = (-230.25850929940458);
        let assign31810_e39579: f64 = (-var_fbbtsti_d);
        let assign31810_e39581: f64 = (assign31810_e39579 / var_fmaxr);
        let assign31810_e39582: f64 = (assign31810_e39577 - assign31810_e39581);
        let assign31810_e39586: f64 = (-230.25850929940458);
        let assign31810_e39588: f64 = (-var_fbbtsti_d);
        let assign31810_e39590: f64 = (assign31810_e39588 / var_fmaxr);
        let assign31810_e39591: f64 = (assign31810_e39586 - assign31810_e39590);
        let assign31810_e39594: f64 = (-230.25850929940458);
        let assign31810_e39596: f64 = (-var_fbbtsti_d);
        let assign31810_e39598: f64 = (assign31810_e39596 / var_fmaxr);
        let assign31810_e39599: f64 = (assign31810_e39594 - assign31810_e39598);
        let assign31810_e39601: f64 = (assign31810_e39599 * 0.3333333333333333);
        let assign31810_e39602: f64 = (1.0 + assign31810_e39601);
        let assign31810_e39603: f64 = (assign31810_e39591 * assign31810_e39602);
        let assign31810_e39604: f64 = (0.5 * assign31810_e39603);
        let assign31810_e39605: f64 = (1.0 + assign31810_e39604);
        let assign31810_e39606: f64 = (assign31810_e39582 * assign31810_e39605);
        let assign31810_e39607: f64 = (1.0 + assign31810_e39606);
        let assign31810_e39608: f64 = (1e-100 / assign31810_e39607);
        (assign31810_e39608, (-((1e-100 * (((-(-((assign31810_e39579 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign31810_e39605) + (assign31810_e39582 * (0.5 * (((-(-((assign31810_e39588 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign31810_e39602) + (assign31810_e39591 * ((-(-((assign31810_e39596 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31810_e39607 * assign31810_e39607))), (-((1e-100 * (((-(-((assign31810_e39579 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign31810_e39605) + (assign31810_e39582 * (0.5 * (((-(-((assign31810_e39588 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign31810_e39602) + (assign31810_e39591 * ((-(-((assign31810_e39596 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31810_e39607 * assign31810_e39607))), (-((1e-100 * (((-(-((assign31810_e39579 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign31810_e39605) + (assign31810_e39582 * (0.5 * (((-(-((assign31810_e39588 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign31810_e39602) + (assign31810_e39591 * ((-(-((assign31810_e39596 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31810_e39607 * assign31810_e39607))), (-((1e-100 * (((-(-((assign31810_e39579 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign31810_e39605) + (assign31810_e39582 * (0.5 * (((-(-((assign31810_e39588 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign31810_e39602) + (assign31810_e39591 * ((-(-((assign31810_e39596 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31810_e39607 * assign31810_e39607))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31810_e39610;
        var_tmp_dn5 = assign31810_e39610_d_n5;
        var_tmp_dn6 = assign31810_e39610_d_n6;
        var_tmp_dn7 = assign31810_e39610_d_n7;
        var_tmp_dn8 = assign31810_e39610_d_n8;

        let (assign31820_e39659, assign31820_e39659_d_n5, assign31820_e39659_d_n6, assign31820_e39659_d_n7, assign31820_e39659_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard620 == 0.0)) && (var_guard622 == 0.0)) && (var_guard623 == 0.0)) {
        let assign31820_e39629: f64 = (-var_fbbtsti_d);
        let assign31820_e39631: f64 = (assign31820_e39629 / var_fmaxr);
        let assign31820_e39633: f64 = (assign31820_e39631 - 230.25850929940458);
        let assign31820_e39637: f64 = (-var_fbbtsti_d);
        let assign31820_e39639: f64 = (assign31820_e39637 / var_fmaxr);
        let assign31820_e39641: f64 = (assign31820_e39639 - 230.25850929940458);
        let assign31820_e39644: f64 = (-var_fbbtsti_d);
        let assign31820_e39646: f64 = (assign31820_e39644 / var_fmaxr);
        let assign31820_e39648: f64 = (assign31820_e39646 - 230.25850929940458);
        let assign31820_e39650: f64 = (assign31820_e39648 * 0.3333333333333333);
        let assign31820_e39651: f64 = (1.0 + assign31820_e39650);
        let assign31820_e39652: f64 = (assign31820_e39641 * assign31820_e39651);
        let assign31820_e39653: f64 = (0.5 * assign31820_e39652);
        let assign31820_e39654: f64 = (1.0 + assign31820_e39653);
        let assign31820_e39655: f64 = (assign31820_e39633 * assign31820_e39654);
        let assign31820_e39656: f64 = (1.0 + assign31820_e39655);
        let assign31820_e39657: f64 = (1e100 * assign31820_e39656);
        (assign31820_e39657, (1e100 * (((-((assign31820_e39629 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign31820_e39654) + (assign31820_e39633 * (0.5 * (((-((assign31820_e39637 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign31820_e39651) + (assign31820_e39641 * ((-((assign31820_e39644 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31820_e39629 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign31820_e39654) + (assign31820_e39633 * (0.5 * (((-((assign31820_e39637 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign31820_e39651) + (assign31820_e39641 * ((-((assign31820_e39644 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31820_e39629 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign31820_e39654) + (assign31820_e39633 * (0.5 * (((-((assign31820_e39637 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign31820_e39651) + (assign31820_e39641 * ((-((assign31820_e39644 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31820_e39629 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign31820_e39654) + (assign31820_e39633 * (0.5 * (((-((assign31820_e39637 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign31820_e39651) + (assign31820_e39641 * ((-((assign31820_e39644 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31820_e39659;
        var_tmp_dn5 = assign31820_e39659_d_n5;
        var_tmp_dn6 = assign31820_e39659_d_n6;
        var_tmp_dn7 = assign31820_e39659_d_n7;
        var_tmp_dn8 = assign31820_e39659_d_n8;

        let (assign31830_e39679, assign31830_e39679_d_n5, assign31830_e39679_d_n6, assign31830_e39679_d_n7, assign31830_e39679_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31830_e39672: f64 = (var_v2 * var_fmaxr);
        let assign31830_e39674: f64 = (assign31830_e39672 * var_fmaxr);
        let assign31830_e39676: f64 = (assign31830_e39674 * var_tmp);
        let assign31830_e39677: f64 = (var_cbbtstid_i * assign31830_e39676);
        (assign31830_e39677, (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign31830_e39672 * var_fmaxr_dn5)) * var_tmp) + (assign31830_e39674 * var_tmp_dn5))), (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign31830_e39672 * var_fmaxr_dn6)) * var_tmp) + (assign31830_e39674 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign31830_e39672 * var_fmaxr_dn7)) * var_tmp) + (assign31830_e39674 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign31830_e39672 * var_fmaxr_dn8)) * var_tmp) + (assign31830_e39674 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign31830_e39679;
        var_ibbt_dn5 = assign31830_e39679_d_n5;
        var_ibbt_dn6 = assign31830_e39679_d_n6;
        var_ibbt_dn7 = assign31830_e39679_d_n7;
        var_ibbt_dn8 = assign31830_e39679_d_n8;

        let assign31840_e39682: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard624 = assign31840_e39682;

        let (assign31850_e39693, assign31850_e39693_d_n5, assign31850_e39693_d_n6, assign31850_e39693_d_n7, assign31850_e39693_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard624 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign31850_e39693;
        var_fbreakdown_dn5 = assign31850_e39693_d_n5;
        var_fbreakdown_dn6 = assign31850_e39693_d_n6;
        var_fbreakdown_dn7 = assign31850_e39693_d_n7;
        var_fbreakdown_dn8 = assign31850_e39693_d_n8;

        let assign31860_e39696: f64 = (-var_alphaav);
        let assign31860_e39698: f64 = (assign31860_e39696 * var_vbrstid_i);
        let assign31860_e39699: f64 = if var_vav > assign31860_e39698 { 1.0 } else { 0.0 };
        var_guard625 = assign31860_e39699;

        let assign31870_e39702: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard626 = assign31870_e39702;

        let (assign31880_e39732, assign31880_e39732_d_n5, assign31880_e39732_d_n6, assign31880_e39732_d_n7, assign31880_e39732_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard624 == 0.0)) && (var_guard625 != 0.0)) && (var_guard626 != 0.0)) {
        let assign31880_e39718: f64 = (var_vav * var_vbrinvsti_d);
        let assign31880_e39721: f64 = (var_vav * var_vbrinvsti_d);
        let assign31880_e39722: f64 = (assign31880_e39718 * assign31880_e39721);
        let assign31880_e39725: f64 = (var_vav * var_vbrinvsti_d);
        let assign31880_e39726: f64 = (assign31880_e39722 * assign31880_e39725);
        let assign31880_e39729: f64 = (var_vav * var_vbrinvsti_d);
        let assign31880_e39730: f64 = (assign31880_e39726 * assign31880_e39729);
        (assign31880_e39730, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31880_e39732;
        var_tmp_dn5 = assign31880_e39732_d_n5;
        var_tmp_dn6 = assign31880_e39732_d_n6;
        var_tmp_dn7 = assign31880_e39732_d_n7;
        var_tmp_dn8 = assign31880_e39732_d_n8;

        let (assign31890_e39754, assign31890_e39754_d_n5, assign31890_e39754_d_n6, assign31890_e39754_d_n7, assign31890_e39754_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard624 == 0.0)) && (var_guard625 != 0.0)) && (var_guard626 == 0.0)) {
        let assign31890_e39749: f64 = (var_vav * var_vbrinvsti_d);
        let assign31890_e39750: f64 = (assign31890_e39749).abs();
        let assign31890_e39752: f64 = (assign31890_e39750).powf(var_pbrstid_i);
        (assign31890_e39752, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31890_e39754;
        var_tmp_dn5 = assign31890_e39754_d_n5;
        var_tmp_dn6 = assign31890_e39754_d_n6;
        var_tmp_dn7 = assign31890_e39754_d_n7;
        var_tmp_dn8 = assign31890_e39754_d_n8;

        let (assign31900_e39772, assign31900_e39772_d_n5, assign31900_e39772_d_n6, assign31900_e39772_d_n7, assign31900_e39772_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard624 == 0.0)) && (var_guard625 != 0.0)) {
        let assign31900_e39769: f64 = (1.0 - var_tmp);
        let assign31900_e39770: f64 = (1.0 / assign31900_e39769);
        (assign31900_e39770, (-((-var_tmp_dn5) / (assign31900_e39769 * assign31900_e39769))), (-((-var_tmp_dn6) / (assign31900_e39769 * assign31900_e39769))), (-((-var_tmp_dn7) / (assign31900_e39769 * assign31900_e39769))), (-((-var_tmp_dn8) / (assign31900_e39769 * assign31900_e39769))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign31900_e39772;
        var_fbreakdown_dn5 = assign31900_e39772_d_n5;
        var_fbreakdown_dn6 = assign31900_e39772_d_n6;
        var_fbreakdown_dn7 = assign31900_e39772_d_n7;
        var_fbreakdown_dn8 = assign31900_e39772_d_n8;

        let (assign31910_e39795, assign31910_e39795_d_n5, assign31910_e39795_d_n6, assign31910_e39795_d_n7, assign31910_e39795_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard624 == 0.0)) && (var_guard625 == 0.0)) {
        let assign31910_e39789: f64 = (var_alphaav * var_vbrstid_i);
        let assign31910_e39790: f64 = (var_vav + assign31910_e39789);
        let assign31910_e39792: f64 = (assign31910_e39790 * var_slopesti_d);
        let assign31910_e39793: f64 = (var_fstopsti_d + assign31910_e39792);
        (assign31910_e39793, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign31910_e39795;
        var_fbreakdown_dn5 = assign31910_e39795_d_n5;
        var_fbreakdown_dn6 = assign31910_e39795_d_n6;
        var_fbreakdown_dn7 = assign31910_e39795_d_n7;
        var_fbreakdown_dn8 = assign31910_e39795_d_n8;

        let (assign31920_e39814, assign31920_e39814_d_n5, assign31920_e39814_d_n6, assign31920_e39814_d_n7, assign31920_e39814_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) {
        let assign31920_e39805: f64 = (var_id__blk213 + var_isrh);
        let assign31920_e39807: f64 = (assign31920_e39805 + var_itat);
        let assign31920_e39809: f64 = (assign31920_e39807 + var_ibbt);
        let assign31920_e39810: f64 = (p.p29 * assign31920_e39809);
        let assign31920_e39812: f64 = (assign31920_e39810 * var_fbreakdown);
        (assign31920_e39812, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign31920_e39810 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign31920_e39810 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign31920_e39810 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign31920_e39810 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign31920_e39814;
        var_ijunsti_dn5 = assign31920_e39814_d_n5;
        var_ijunsti_dn6 = assign31920_e39814_d_n6;
        var_ijunsti_dn7 = assign31920_e39814_d_n7;
        var_ijunsti_dn8 = assign31920_e39814_d_n8;

        let assign31930_e39817: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard627 = assign31930_e39817;

        let (assign31940_e39825, assign31940_e39825_d_n5, assign31940_e39825_d_n6, assign31940_e39825_d_n7, assign31940_e39825_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign31940_e39825;
        var_ijungat_dn5 = assign31940_e39825_d_n5;
        var_ijungat_dn6 = assign31940_e39825_d_n6;
        var_ijungat_dn7 = assign31940_e39825_d_n7;
        var_ijungat_dn8 = assign31940_e39825_d_n8;

        let (assign31950_e39836,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) {
        let assign31950_e39834: f64 = (var_idsatgat_d * var_idmult);
        (assign31950_e39834,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign31950_e39836;

        let assign31960_e39843: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard628 = assign31960_e39843;

        let (assign31970_e39854, assign31970_e39854_d_n5, assign31970_e39854_d_n6, assign31970_e39854_d_n7, assign31970_e39854_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign31970_e39854;
        var_isrh_dn5 = assign31970_e39854_d_n5;
        var_isrh_dn6 = assign31970_e39854_d_n6;
        var_isrh_dn7 = assign31970_e39854_d_n7;
        var_isrh_dn8 = assign31970_e39854_d_n8;

        let (assign31980_e39868,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) {
        let assign31980_e39866: f64 = (var_vbigat_d - var_vjsrh);
        (assign31980_e39866,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign31980_e39868;

        let (assign31990_e39887,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) {
        let assign31990_e39882: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign31990_e39883: f64 = (1.0 - assign31990_e39882);
        let assign31990_e39884: f64 = (assign31990_e39883).sqrt();
        let assign31990_e39885: f64 = (1.0 - assign31990_e39884);
        (assign31990_e39885,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign31990_e39887;

        let assign32000_e39890: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard629 = assign32000_e39890;

        let (assign32010_e39904,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) && (var_guard629 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign32010_e39904;

        let (assign32020_e39936,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) && (var_guard629 == 0.0)) {
        let assign32020_e39919: f64 = (var_wsrhstep * var_wsrhstep);
        let assign32020_e39921: f64 = (var_wsrhstep).ln();
        let assign32020_e39922: f64 = (assign32020_e39919 * assign32020_e39921);
        let assign32020_e39925: f64 = (1.0 - var_wsrhstep);
        let assign32020_e39926: f64 = (assign32020_e39922 / assign32020_e39925);
        let assign32020_e39928: f64 = (assign32020_e39926 + var_wsrhstep);
        let assign32020_e39932: f64 = (2.0 * var_pgatd_i);
        let assign32020_e39933: f64 = (1.0 - assign32020_e39932);
        let assign32020_e39934: f64 = (assign32020_e39928 * assign32020_e39933);
        (assign32020_e39934,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign32020_e39936;

        let (assign32030_e39950,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) {
        let assign32030_e39948: f64 = (var_wsrhstep + var_dwsrh);
        (assign32030_e39948,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign32030_e39950;

        let assign32040_e39953: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard630 = assign32040_e39953;

        let (assign32050_e39970, assign32050_e39970_d_n5, assign32050_e39970_d_n6, assign32050_e39970_d_n7, assign32050_e39970_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) && (var_guard630 != 0.0)) {
        let assign32050_e39967: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign32050_e39968: f64 = (assign32050_e39967).sqrt();
        (assign32050_e39968, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32050_e39970;
        var_tmp_dn5 = assign32050_e39970_d_n5;
        var_tmp_dn6 = assign32050_e39970_d_n6;
        var_tmp_dn7 = assign32050_e39970_d_n7;
        var_tmp_dn8 = assign32050_e39970_d_n8;

        let (assign32060_e39989, assign32060_e39989_d_n5, assign32060_e39989_d_n6, assign32060_e39989_d_n7, assign32060_e39989_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) && (var_guard630 == 0.0)) {
        let assign32060_e39985: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign32060_e39987: f64 = (assign32060_e39985).powf(var_pgatd_i);
        (assign32060_e39987, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32060_e39989;
        var_tmp_dn5 = assign32060_e39989_d_n5;
        var_tmp_dn6 = assign32060_e39989_d_n6;
        var_tmp_dn7 = assign32060_e39989_d_n7;
        var_tmp_dn8 = assign32060_e39989_d_n8;

        let (assign32070_e40003, assign32070_e40003_d_n5, assign32070_e40003_d_n6, assign32070_e40003_d_n7, assign32070_e40003_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) {
        let assign32070_e40001: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign32070_e40001, (var_wdepnulrgat_d * var_tmp_dn5), (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign32070_e40003;
        var_wdep_dn5 = assign32070_e40003_d_n5;
        var_wdep_dn6 = assign32070_e40003_d_n6;
        var_wdep_dn7 = assign32070_e40003_d_n7;
        var_wdep_dn8 = assign32070_e40003_d_n8;

        let (assign32080_e40021, assign32080_e40021_d_n5, assign32080_e40021_d_n6, assign32080_e40021_d_n7, assign32080_e40021_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) {
        let assign32080_e40016: f64 = (var_zinv - 1.0);
        let assign32080_e40018: f64 = (assign32080_e40016 * var_wdep);
        let assign32080_e40019: f64 = (var_ftdgat_d * assign32080_e40018);
        (assign32080_e40019, (var_ftdgat_d * (assign32080_e40016 * var_wdep_dn5)), (var_ftdgat_d * (assign32080_e40016 * var_wdep_dn6)), (var_ftdgat_d * (assign32080_e40016 * var_wdep_dn7)), (var_ftdgat_d * (assign32080_e40016 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign32080_e40021;
        var_asrh_dn5 = assign32080_e40021_d_n5;
        var_asrh_dn6 = assign32080_e40021_d_n6;
        var_asrh_dn7 = assign32080_e40021_d_n7;
        var_asrh_dn8 = assign32080_e40021_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard620_slot = var_guard620;
        *var_guard621_slot = var_guard621;
        *var_guard622_slot = var_guard622;
        *var_guard623_slot = var_guard623;
        *var_guard624_slot = var_guard624;
        *var_guard625_slot = var_guard625;
        *var_guard626_slot = var_guard626;
        *var_guard627_slot = var_guard627;
        *var_guard628_slot = var_guard628;
        *var_guard629_slot = var_guard629;
        *var_guard630_slot = var_guard630;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_65(
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btatpartgat_d: f64,
        var_cbbtgatd_i: f64,
        var_cerfc: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard627: f64,
        var_guard628: f64,
        var_one_minus_pgat_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
        var_vbi_minus_vjsrh: f64,
        var_wdep: f64,
        var_wdep_dn5: f64,
        var_wdep_dn6: f64,
        var_wdep_dn7: f64,
        var_wdep_dn8: f64,
        var_wsrh: f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard631_slot: &mut f64,
        var_guard632_slot: &mut f64,
        var_guard633_slot: &mut f64,
        var_guard634_slot: &mut f64,
        var_guard635_slot: &mut f64,
        var_guard636_slot: &mut f64,
        var_guard637_slot: &mut f64,
        var_guard638_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard631: f64 = *var_guard631_slot;
        let mut var_guard632: f64 = *var_guard632_slot;
        let mut var_guard633: f64 = *var_guard633_slot;
        let mut var_guard634: f64 = *var_guard634_slot;
        let mut var_guard635: f64 = *var_guard635_slot;
        let mut var_guard636: f64 = *var_guard636_slot;
        let mut var_guard637: f64 = *var_guard637_slot;
        let mut var_guard638: f64 = *var_guard638_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign32090_e40037, assign32090_e40037_d_n5, assign32090_e40037_d_n6, assign32090_e40037_d_n7, assign32090_e40037_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) {
        let assign32090_e40034: f64 = (var_asrh * var_wsrh);
        let assign32090_e40035: f64 = (var_csrhgatd_i * assign32090_e40034);
        (assign32090_e40035, (var_csrhgatd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign32090_e40037;
        var_isrh_dn5 = assign32090_e40037_d_n5;
        var_isrh_dn6 = assign32090_e40037_d_n6;
        var_isrh_dn7 = assign32090_e40037_d_n7;
        var_isrh_dn8 = assign32090_e40037_d_n8;

        let assign32100_e40040: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard631 = assign32100_e40040;

        let (assign32110_e40051, assign32110_e40051_d_n5, assign32110_e40051_d_n6, assign32110_e40051_d_n7, assign32110_e40051_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign32110_e40051;
        var_itat_dn5 = assign32110_e40051_d_n5;
        var_itat_dn6 = assign32110_e40051_d_n6;
        var_itat_dn7 = assign32110_e40051_d_n7;
        var_itat_dn8 = assign32110_e40051_d_n8;

        let (assign32120_e40069, assign32120_e40069_d_n5, assign32120_e40069_d_n6, assign32120_e40069_d_n7, assign32120_e40069_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32120_e40064: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign32120_e40066: f64 = (assign32120_e40064 / var_vbi_minus_vjsrh);
        let assign32120_e40067: f64 = (var_btatpartgat_d * assign32120_e40066);
        (assign32120_e40067, (var_btatpartgat_d * ((var_wdep_dn5 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign32120_e40069;
        var_btat_dn5 = assign32120_e40069_d_n5;
        var_btat_dn6 = assign32120_e40069_d_n6;
        var_btat_dn7 = assign32120_e40069_d_n7;
        var_btat_dn8 = assign32120_e40069_d_n8;

        let (assign32130_e40085, assign32130_e40085_d_n5, assign32130_e40085_d_n6, assign32130_e40085_d_n7, assign32130_e40085_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32130_e40081: f64 = (0.666666666666667 * var_atatgat_d);
        let assign32130_e40083: f64 = (assign32130_e40081 / var_btat);
        (assign32130_e40083, (-((assign32130_e40081 * var_btat_dn5) / (var_btat * var_btat))), (-((assign32130_e40081 * var_btat_dn6) / (var_btat * var_btat))), (-((assign32130_e40081 * var_btat_dn7) / (var_btat * var_btat))), (-((assign32130_e40081 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign32130_e40085;
        var_twoatatoverthreebtat_dn5 = assign32130_e40085_d_n5;
        var_twoatatoverthreebtat_dn6 = assign32130_e40085_d_n6;
        var_twoatatoverthreebtat_dn7 = assign32130_e40085_d_n7;
        var_twoatatoverthreebtat_dn8 = assign32130_e40085_d_n8;

        let (assign32140_e40099, assign32140_e40099_d_n5, assign32140_e40099_d_n6, assign32140_e40099_d_n7, assign32140_e40099_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32140_e40097: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign32140_e40097, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign32140_e40099;
        var_umaxbeforelimiting_dn5 = assign32140_e40099_d_n5;
        var_umaxbeforelimiting_dn6 = assign32140_e40099_d_n6;
        var_umaxbeforelimiting_dn7 = assign32140_e40099_d_n7;
        var_umaxbeforelimiting_dn8 = assign32140_e40099_d_n8;

        let (assign32150_e40120, assign32150_e40120_d_n5, assign32150_e40120_d_n6, assign32150_e40120_d_n7, assign32150_e40120_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32150_e40111: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign32150_e40114: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign32150_e40116: f64 = (assign32150_e40114 + 1.0);
        let assign32150_e40117: f64 = (assign32150_e40111 / assign32150_e40116);
        let assign32150_e40118: f64 = (assign32150_e40117).sqrt();
        (assign32150_e40118, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign32150_e40116) - (assign32150_e40111 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign32150_e40116 * assign32150_e40116)) / (2.0 * assign32150_e40118)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign32150_e40116) - (assign32150_e40111 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign32150_e40116 * assign32150_e40116)) / (2.0 * assign32150_e40118)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign32150_e40116) - (assign32150_e40111 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign32150_e40116 * assign32150_e40116)) / (2.0 * assign32150_e40118)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign32150_e40116) - (assign32150_e40111 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign32150_e40116 * assign32150_e40116)) / (2.0 * assign32150_e40118)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign32150_e40120;
        var_umax_dn5 = assign32150_e40120_d_n5;
        var_umax_dn6 = assign32150_e40120_d_n6;
        var_umax_dn7 = assign32150_e40120_d_n7;
        var_umax_dn8 = assign32150_e40120_d_n8;

        let (assign32160_e40133, assign32160_e40133_d_n5, assign32160_e40133_d_n6, assign32160_e40133_d_n7, assign32160_e40133_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32160_e40131: f64 = (var_umax).sqrt();
        (assign32160_e40131, (var_umax_dn5 / (2.0 * assign32160_e40131)), (var_umax_dn6 / (2.0 * assign32160_e40131)), (var_umax_dn7 / (2.0 * assign32160_e40131)), (var_umax_dn8 / (2.0 * assign32160_e40131)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign32160_e40133;
        var_sqrtumax_dn5 = assign32160_e40133_d_n5;
        var_sqrtumax_dn6 = assign32160_e40133_d_n6;
        var_sqrtumax_dn7 = assign32160_e40133_d_n7;
        var_sqrtumax_dn8 = assign32160_e40133_d_n8;

        let (assign32170_e40147, assign32170_e40147_d_n5, assign32170_e40147_d_n6, assign32170_e40147_d_n7, assign32170_e40147_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32170_e40145: f64 = (var_umax * var_sqrtumax);
        (assign32170_e40145, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign32170_e40147;
        var_umaxpoweronepointfive_dn5 = assign32170_e40147_d_n5;
        var_umaxpoweronepointfive_dn6 = assign32170_e40147_d_n6;
        var_umaxpoweronepointfive_dn7 = assign32170_e40147_d_n7;
        var_umaxpoweronepointfive_dn8 = assign32170_e40147_d_n8;

        let assign32180_e40149: f64 = (-var_pgatd_i);
        let assign32180_e40151: f64 = (assign32180_e40149 * var_one_over_one_minus_pgat_d);
        let assign32180_e40153: f64 = (-1.0);
        let assign32180_e40154: f64 = if assign32180_e40151 == assign32180_e40153 { 1.0 } else { 0.0 };
        var_guard632 = assign32180_e40154;

        let (assign32190_e40174, assign32190_e40174_d_n5, assign32190_e40174_d_n6, assign32190_e40174_d_n7, assign32190_e40174_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) && (var_guard632 != 0.0)) {
        let assign32190_e40170: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign32190_e40171: f64 = (1.0 + assign32190_e40170);
        let assign32190_e40172: f64 = (1.0 / assign32190_e40171);
        (assign32190_e40172, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign32190_e40171 * assign32190_e40171))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign32190_e40171 * assign32190_e40171))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign32190_e40171 * assign32190_e40171))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign32190_e40171 * assign32190_e40171))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign32190_e40174;
        var_wgamma_dn5 = assign32190_e40174_d_n5;
        var_wgamma_dn6 = assign32190_e40174_d_n6;
        var_wgamma_dn7 = assign32190_e40174_d_n7;
        var_wgamma_dn8 = assign32190_e40174_d_n8;

        let (assign32200_e40198, assign32200_e40198_d_n5, assign32200_e40198_d_n6, assign32200_e40198_d_n7, assign32200_e40198_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) && (var_guard632 == 0.0)) {
        let assign32200_e40190: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign32200_e40191: f64 = (1.0 + assign32200_e40190);
        let assign32200_e40193: f64 = (-var_pgatd_i);
        let assign32200_e40195: f64 = (assign32200_e40193 * var_one_over_one_minus_pgat_d);
        let assign32200_e40196: f64 = (assign32200_e40191).powf(assign32200_e40195);
        (assign32200_e40196, if 0.0 == 0.0 && ((assign32200_e40195) as f64).is_finite() && ((assign32200_e40195) as f64).fract() == 0.0 { if assign32200_e40195 == 0.0 { 0.0 } else { (assign32200_e40195 * ((assign32200_e40191).powf(assign32200_e40195 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign32200_e40196 * (assign32200_e40195 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign32200_e40191))) }, if 0.0 == 0.0 && ((assign32200_e40195) as f64).is_finite() && ((assign32200_e40195) as f64).fract() == 0.0 { if assign32200_e40195 == 0.0 { 0.0 } else { (assign32200_e40195 * ((assign32200_e40191).powf(assign32200_e40195 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign32200_e40196 * (assign32200_e40195 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign32200_e40191))) }, if 0.0 == 0.0 && ((assign32200_e40195) as f64).is_finite() && ((assign32200_e40195) as f64).fract() == 0.0 { if assign32200_e40195 == 0.0 { 0.0 } else { (assign32200_e40195 * ((assign32200_e40191).powf(assign32200_e40195 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign32200_e40196 * (assign32200_e40195 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign32200_e40191))) }, if 0.0 == 0.0 && ((assign32200_e40195) as f64).is_finite() && ((assign32200_e40195) as f64).fract() == 0.0 { if assign32200_e40195 == 0.0 { 0.0 } else { (assign32200_e40195 * ((assign32200_e40191).powf(assign32200_e40195 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign32200_e40196 * (assign32200_e40195 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign32200_e40191))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign32200_e40198;
        var_wgamma_dn5 = assign32200_e40198_d_n5;
        var_wgamma_dn6 = assign32200_e40198_d_n6;
        var_wgamma_dn7 = assign32200_e40198_d_n7;
        var_wgamma_dn8 = assign32200_e40198_d_n8;

        let (assign32210_e40216, assign32210_e40216_d_n5, assign32210_e40216_d_n6, assign32210_e40216_d_n7, assign32210_e40216_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32210_e40210: f64 = (var_wsrh * var_wgamma);
        let assign32210_e40213: f64 = (var_wsrh + var_wgamma);
        let assign32210_e40214: f64 = (assign32210_e40210 / assign32210_e40213);
        (assign32210_e40214, ((((var_wsrh * var_wgamma_dn5) * assign32210_e40213) - (assign32210_e40210 * var_wgamma_dn5)) / (assign32210_e40213 * assign32210_e40213)), ((((var_wsrh * var_wgamma_dn6) * assign32210_e40213) - (assign32210_e40210 * var_wgamma_dn6)) / (assign32210_e40213 * assign32210_e40213)), ((((var_wsrh * var_wgamma_dn7) * assign32210_e40213) - (assign32210_e40210 * var_wgamma_dn7)) / (assign32210_e40213 * assign32210_e40213)), ((((var_wsrh * var_wgamma_dn8) * assign32210_e40213) - (assign32210_e40210 * var_wgamma_dn8)) / (assign32210_e40213 * assign32210_e40213)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign32210_e40216;
        var_wtat_dn5 = assign32210_e40216_d_n5;
        var_wtat_dn6 = assign32210_e40216_d_n6;
        var_wtat_dn7 = assign32210_e40216_d_n7;
        var_wtat_dn8 = assign32210_e40216_d_n8;

        let (assign32220_e40233, assign32220_e40233_d_n5, assign32220_e40233_d_n6, assign32220_e40233_d_n7, assign32220_e40233_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32220_e40229: f64 = (var_btat / var_sqrtumax);
        let assign32220_e40230: f64 = (0.375 * assign32220_e40229);
        let assign32220_e40231: f64 = (assign32220_e40230).sqrt();
        (assign32220_e40231, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32220_e40231)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32220_e40231)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32220_e40231)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32220_e40231)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign32220_e40233;
        var_ktat_dn5 = assign32220_e40233_d_n5;
        var_ktat_dn6 = assign32220_e40233_d_n6;
        var_ktat_dn7 = assign32220_e40233_d_n7;
        var_ktat_dn8 = assign32220_e40233_d_n8;

        let (assign32230_e40251, assign32230_e40251_d_n5, assign32230_e40251_d_n6, assign32230_e40251_d_n7, assign32230_e40251_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32230_e40246: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign32230_e40247: f64 = (2.0 * assign32230_e40246);
        let assign32230_e40249: f64 = (assign32230_e40247 - var_umax);
        (assign32230_e40249, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign32230_e40251;
        var_ltat_dn5 = assign32230_e40251_d_n5;
        var_ltat_dn6 = assign32230_e40251_d_n6;
        var_ltat_dn7 = assign32230_e40251_d_n7;
        var_ltat_dn8 = assign32230_e40251_d_n8;

        let (assign32240_e40277, assign32240_e40277_d_n5, assign32240_e40277_d_n6, assign32240_e40277_d_n7, assign32240_e40277_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32240_e40263: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign32240_e40265: f64 = (assign32240_e40263 * var_sqrtumax);
        let assign32240_e40268: f64 = (var_atatgat_d * var_umax);
        let assign32240_e40269: f64 = (assign32240_e40265 - assign32240_e40268);
        let assign32240_e40273: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign32240_e40274: f64 = (0.5 * assign32240_e40273);
        let assign32240_e40275: f64 = (assign32240_e40269 + assign32240_e40274);
        (assign32240_e40275, (((((var_atatgat_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign32240_e40263 * var_sqrtumax_dn5)) - (var_atatgat_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign32240_e40263 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign32240_e40263 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign32240_e40263 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign32240_e40277;
        var_mtat_dn5 = assign32240_e40277_d_n5;
        var_mtat_dn6 = assign32240_e40277_d_n6;
        var_mtat_dn7 = assign32240_e40277_d_n7;
        var_mtat_dn8 = assign32240_e40277_d_n8;

        let (assign32250_e40293, assign32250_e40293_d_n5, assign32250_e40293_d_n6, assign32250_e40293_d_n7, assign32250_e40293_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32250_e40289: f64 = (var_ltat - 1.0);
        let assign32250_e40291: f64 = (assign32250_e40289 * var_ktat);
        (assign32250_e40291, ((var_ltat_dn5 * var_ktat) + (assign32250_e40289 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign32250_e40289 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign32250_e40289 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign32250_e40289 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign32250_e40293;
        var_xerfc_dn5 = assign32250_e40293_d_n5;
        var_xerfc_dn6 = assign32250_e40293_d_n6;
        var_xerfc_dn7 = assign32250_e40293_d_n7;
        var_xerfc_dn8 = assign32250_e40293_d_n8;

        let (assign32260_e40307, assign32260_e40307_d_n5, assign32260_e40307_d_n6, assign32260_e40307_d_n7, assign32260_e40307_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32260_e40305: f64 = (var_xerfc * var_xerfc);
        (assign32260_e40305, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign32260_e40307;
        var_ysq_dn5 = assign32260_e40307_d_n5;
        var_ysq_dn6 = assign32260_e40307_d_n6;
        var_ysq_dn7 = assign32260_e40307_d_n7;
        var_ysq_dn8 = assign32260_e40307_d_n8;

        let assign32270_e40310: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard633 = assign32270_e40310;

        let (assign32280_e40330, assign32280_e40330_d_n5, assign32280_e40330_d_n6, assign32280_e40330_d_n7, assign32280_e40330_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) && (var_guard633 != 0.0)) {
        let assign32280_e40326: f64 = (var_perfc * var_xerfc);
        let assign32280_e40327: f64 = (1.0 + assign32280_e40326);
        let assign32280_e40328: f64 = (1.0 / assign32280_e40327);
        (assign32280_e40328, (-((var_perfc * var_xerfc_dn5) / (assign32280_e40327 * assign32280_e40327))), (-((var_perfc * var_xerfc_dn6) / (assign32280_e40327 * assign32280_e40327))), (-((var_perfc * var_xerfc_dn7) / (assign32280_e40327 * assign32280_e40327))), (-((var_perfc * var_xerfc_dn8) / (assign32280_e40327 * assign32280_e40327))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign32280_e40330;
        var_terfc_dn5 = assign32280_e40330_d_n5;
        var_terfc_dn6 = assign32280_e40330_d_n6;
        var_terfc_dn7 = assign32280_e40330_d_n7;
        var_terfc_dn8 = assign32280_e40330_d_n8;

        let (assign32290_e40351, assign32290_e40351_d_n5, assign32290_e40351_d_n6, assign32290_e40351_d_n7, assign32290_e40351_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) && (var_guard633 == 0.0)) {
        let assign32290_e40347: f64 = (var_perfc * var_xerfc);
        let assign32290_e40348: f64 = (1.0 - assign32290_e40347);
        let assign32290_e40349: f64 = (1.0 / assign32290_e40348);
        (assign32290_e40349, (-((-(var_perfc * var_xerfc_dn5)) / (assign32290_e40348 * assign32290_e40348))), (-((-(var_perfc * var_xerfc_dn6)) / (assign32290_e40348 * assign32290_e40348))), (-((-(var_perfc * var_xerfc_dn7)) / (assign32290_e40348 * assign32290_e40348))), (-((-(var_perfc * var_xerfc_dn8)) / (assign32290_e40348 * assign32290_e40348))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign32290_e40351;
        var_terfc_dn5 = assign32290_e40351_d_n5;
        var_terfc_dn6 = assign32290_e40351_d_n6;
        var_terfc_dn7 = assign32290_e40351_d_n7;
        var_terfc_dn8 = assign32290_e40351_d_n8;

        let assign32300_e40353: f64 = (-var_ysq);
        let assign32300_e40355: f64 = (assign32300_e40353 + var_mtat);
        let assign32300_e40357: f64 = (-230.25850929940458);
        let assign32300_e40358: f64 = if assign32300_e40355 > assign32300_e40357 { 1.0 } else { 0.0 };
        var_guard634 = assign32300_e40358;

        let (assign32310_e40376, assign32310_e40376_d_n5, assign32310_e40376_d_n6, assign32310_e40376_d_n7, assign32310_e40376_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) && (var_guard634 != 0.0)) {
        let assign32310_e40371: f64 = (-var_ysq);
        let assign32310_e40373: f64 = (assign32310_e40371 + var_mtat);
        let assign32310_e40374: f64 = (assign32310_e40373).exp();
        (assign32310_e40374, (assign32310_e40374 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign32310_e40374 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign32310_e40374 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign32310_e40374 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32310_e40376;
        var_tmp_dn5 = assign32310_e40376_d_n5;
        var_tmp_dn6 = assign32310_e40376_d_n6;
        var_tmp_dn7 = assign32310_e40376_d_n7;
        var_tmp_dn8 = assign32310_e40376_d_n8;

        let (assign32320_e40425, assign32320_e40425_d_n5, assign32320_e40425_d_n6, assign32320_e40425_d_n7, assign32320_e40425_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) && (var_guard634 == 0.0)) {
        let assign32320_e40392: f64 = (-230.25850929940458);
        let assign32320_e40394: f64 = (-var_ysq);
        let assign32320_e40396: f64 = (assign32320_e40394 + var_mtat);
        let assign32320_e40397: f64 = (assign32320_e40392 - assign32320_e40396);
        let assign32320_e40401: f64 = (-230.25850929940458);
        let assign32320_e40403: f64 = (-var_ysq);
        let assign32320_e40405: f64 = (assign32320_e40403 + var_mtat);
        let assign32320_e40406: f64 = (assign32320_e40401 - assign32320_e40405);
        let assign32320_e40409: f64 = (-230.25850929940458);
        let assign32320_e40411: f64 = (-var_ysq);
        let assign32320_e40413: f64 = (assign32320_e40411 + var_mtat);
        let assign32320_e40414: f64 = (assign32320_e40409 - assign32320_e40413);
        let assign32320_e40416: f64 = (assign32320_e40414 * 0.3333333333333333);
        let assign32320_e40417: f64 = (1.0 + assign32320_e40416);
        let assign32320_e40418: f64 = (assign32320_e40406 * assign32320_e40417);
        let assign32320_e40419: f64 = (0.5 * assign32320_e40418);
        let assign32320_e40420: f64 = (1.0 + assign32320_e40419);
        let assign32320_e40421: f64 = (assign32320_e40397 * assign32320_e40420);
        let assign32320_e40422: f64 = (1.0 + assign32320_e40421);
        let assign32320_e40423: f64 = (1e-100 / assign32320_e40422);
        (assign32320_e40423, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign32320_e40420) + (assign32320_e40397 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign32320_e40417) + (assign32320_e40406 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign32320_e40422 * assign32320_e40422))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign32320_e40420) + (assign32320_e40397 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign32320_e40417) + (assign32320_e40406 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign32320_e40422 * assign32320_e40422))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign32320_e40420) + (assign32320_e40397 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign32320_e40417) + (assign32320_e40406 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign32320_e40422 * assign32320_e40422))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign32320_e40420) + (assign32320_e40397 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign32320_e40417) + (assign32320_e40406 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign32320_e40422 * assign32320_e40422))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32320_e40425;
        var_tmp_dn5 = assign32320_e40425_d_n5;
        var_tmp_dn6 = assign32320_e40425_d_n6;
        var_tmp_dn7 = assign32320_e40425_d_n7;
        var_tmp_dn8 = assign32320_e40425_d_n8;

        let (assign32330_e40455, assign32330_e40455_d_n5, assign32330_e40455_d_n6, assign32330_e40455_d_n7, assign32330_e40455_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32330_e40437: f64 = (0.29214664 * var_terfc);
        let assign32330_e40441: f64 = (var_terfc * var_terfc);
        let assign32330_e40442: f64 = (var_berfc * assign32330_e40441);
        let assign32330_e40443: f64 = (assign32330_e40437 + assign32330_e40442);
        let assign32330_e40447: f64 = (var_terfc * var_terfc);
        let assign32330_e40449: f64 = (assign32330_e40447 * var_terfc);
        let assign32330_e40450: f64 = (var_cerfc * assign32330_e40449);
        let assign32330_e40451: f64 = (assign32330_e40443 + assign32330_e40450);
        let assign32330_e40453: f64 = (assign32330_e40451 * var_tmp);
        (assign32330_e40453, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign32330_e40447 * var_terfc_dn5)))) * var_tmp) + (assign32330_e40451 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign32330_e40447 * var_terfc_dn6)))) * var_tmp) + (assign32330_e40451 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign32330_e40447 * var_terfc_dn7)))) * var_tmp) + (assign32330_e40451 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign32330_e40447 * var_terfc_dn8)))) * var_tmp) + (assign32330_e40451 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign32330_e40455;
        var_erfcpos_dn5 = assign32330_e40455_d_n5;
        var_erfcpos_dn6 = assign32330_e40455_d_n6;
        var_erfcpos_dn7 = assign32330_e40455_d_n7;
        var_erfcpos_dn8 = assign32330_e40455_d_n8;

        let assign32340_e40458: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard635 = assign32340_e40458;

        let (assign32350_e40472, assign32350_e40472_d_n5, assign32350_e40472_d_n6, assign32350_e40472_d_n7, assign32350_e40472_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) && (var_guard635 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign32350_e40472;
        var_erfctimesexpmtat_dn5 = assign32350_e40472_d_n5;
        var_erfctimesexpmtat_dn6 = assign32350_e40472_d_n6;
        var_erfctimesexpmtat_dn7 = assign32350_e40472_d_n7;
        var_erfctimesexpmtat_dn8 = assign32350_e40472_d_n8;

        let assign32360_e40475: f64 = (-230.25850929940458);
        let assign32360_e40476: f64 = if var_mtat > assign32360_e40475 { 1.0 } else { 0.0 };
        var_guard636 = assign32360_e40476;

        let (assign32370_e40494, assign32370_e40494_d_n5, assign32370_e40494_d_n6, assign32370_e40494_d_n7, assign32370_e40494_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) && (var_guard635 == 0.0)) && (var_guard636 != 0.0)) {
        let assign32370_e40492: f64 = (var_mtat).exp();
        (assign32370_e40492, (assign32370_e40492 * var_mtat_dn5), (assign32370_e40492 * var_mtat_dn6), (assign32370_e40492 * var_mtat_dn7), (assign32370_e40492 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32370_e40494;
        var_tmp_dn5 = assign32370_e40494_d_n5;
        var_tmp_dn6 = assign32370_e40494_d_n6;
        var_tmp_dn7 = assign32370_e40494_d_n7;
        var_tmp_dn8 = assign32370_e40494_d_n8;

        let (assign32380_e40537, assign32380_e40537_d_n5, assign32380_e40537_d_n6, assign32380_e40537_d_n7, assign32380_e40537_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) && (var_guard635 == 0.0)) && (var_guard636 == 0.0)) {
        let assign32380_e40513: f64 = (-230.25850929940458);
        let assign32380_e40515: f64 = (assign32380_e40513 - var_mtat);
        let assign32380_e40519: f64 = (-230.25850929940458);
        let assign32380_e40521: f64 = (assign32380_e40519 - var_mtat);
        let assign32380_e40524: f64 = (-230.25850929940458);
        let assign32380_e40526: f64 = (assign32380_e40524 - var_mtat);
        let assign32380_e40528: f64 = (assign32380_e40526 * 0.3333333333333333);
        let assign32380_e40529: f64 = (1.0 + assign32380_e40528);
        let assign32380_e40530: f64 = (assign32380_e40521 * assign32380_e40529);
        let assign32380_e40531: f64 = (0.5 * assign32380_e40530);
        let assign32380_e40532: f64 = (1.0 + assign32380_e40531);
        let assign32380_e40533: f64 = (assign32380_e40515 * assign32380_e40532);
        let assign32380_e40534: f64 = (1.0 + assign32380_e40533);
        let assign32380_e40535: f64 = (1e-100 / assign32380_e40534);
        (assign32380_e40535, (-((1e-100 * (((-var_mtat_dn5) * assign32380_e40532) + (assign32380_e40515 * (0.5 * (((-var_mtat_dn5) * assign32380_e40529) + (assign32380_e40521 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign32380_e40534 * assign32380_e40534))), (-((1e-100 * (((-var_mtat_dn6) * assign32380_e40532) + (assign32380_e40515 * (0.5 * (((-var_mtat_dn6) * assign32380_e40529) + (assign32380_e40521 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign32380_e40534 * assign32380_e40534))), (-((1e-100 * (((-var_mtat_dn7) * assign32380_e40532) + (assign32380_e40515 * (0.5 * (((-var_mtat_dn7) * assign32380_e40529) + (assign32380_e40521 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign32380_e40534 * assign32380_e40534))), (-((1e-100 * (((-var_mtat_dn8) * assign32380_e40532) + (assign32380_e40515 * (0.5 * (((-var_mtat_dn8) * assign32380_e40529) + (assign32380_e40521 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign32380_e40534 * assign32380_e40534))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32380_e40537;
        var_tmp_dn5 = assign32380_e40537_d_n5;
        var_tmp_dn6 = assign32380_e40537_d_n6;
        var_tmp_dn7 = assign32380_e40537_d_n7;
        var_tmp_dn8 = assign32380_e40537_d_n8;

        let (assign32390_e40556, assign32390_e40556_d_n5, assign32390_e40556_d_n6, assign32390_e40556_d_n7, assign32390_e40556_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) && (var_guard635 == 0.0)) {
        let assign32390_e40552: f64 = (2.0 * var_tmp);
        let assign32390_e40554: f64 = (assign32390_e40552 - var_erfcpos);
        (assign32390_e40554, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign32390_e40556;
        var_erfctimesexpmtat_dn5 = assign32390_e40556_d_n5;
        var_erfctimesexpmtat_dn6 = assign32390_e40556_d_n6;
        var_erfctimesexpmtat_dn7 = assign32390_e40556_d_n7;
        var_erfctimesexpmtat_dn8 = assign32390_e40556_d_n8;

        let (assign32400_e40576, assign32400_e40576_d_n5, assign32400_e40576_d_n6, assign32400_e40576_d_n7, assign32400_e40576_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32400_e40568: f64 = (1.772453850905516 * 0.5);
        let assign32400_e40571: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign32400_e40573: f64 = (assign32400_e40571 / var_ktat);
        let assign32400_e40574: f64 = (assign32400_e40568 * assign32400_e40573);
        (assign32400_e40574, (assign32400_e40568 * ((((var_atatgat_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign32400_e40571 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign32400_e40568 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign32400_e40571 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign32400_e40568 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign32400_e40571 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign32400_e40568 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign32400_e40571 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign32400_e40576;
        var_gammamax_dn5 = assign32400_e40576_d_n5;
        var_gammamax_dn6 = assign32400_e40576_d_n6;
        var_gammamax_dn7 = assign32400_e40576_d_n7;
        var_gammamax_dn8 = assign32400_e40576_d_n8;

        let (assign32410_e40594, assign32410_e40594_d_n5, assign32410_e40594_d_n6, assign32410_e40594_d_n7, assign32410_e40594_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32410_e40589: f64 = (var_asrh * var_gammamax);
        let assign32410_e40591: f64 = (assign32410_e40589 * var_wtat);
        let assign32410_e40592: f64 = (var_ctatgatd_i * assign32410_e40591);
        (assign32410_e40592, (var_ctatgatd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign32410_e40589 * var_wtat_dn5))), (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign32410_e40589 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign32410_e40589 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign32410_e40589 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign32410_e40594;
        var_itat_dn5 = assign32410_e40594_d_n5;
        var_itat_dn6 = assign32410_e40594_d_n6;
        var_itat_dn7 = assign32410_e40594_d_n7;
        var_itat_dn8 = assign32410_e40594_d_n8;

        let assign32420_e40597: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard637 = assign32420_e40597;

        let (assign32430_e40608, assign32430_e40608_d_n5, assign32430_e40608_d_n6, assign32430_e40608_d_n7, assign32430_e40608_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard637 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign32430_e40608;
        var_ibbt_dn5 = assign32430_e40608_d_n5;
        var_ibbt_dn6 = assign32430_e40608_d_n6;
        var_ibbt_dn7 = assign32430_e40608_d_n7;
        var_ibbt_dn8 = assign32430_e40608_d_n8;

        let assign32440_e40611: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard638 = assign32440_e40611;

        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard631_slot = var_guard631;
        *var_guard632_slot = var_guard632;
        *var_guard633_slot = var_guard633;
        *var_guard634_slot = var_guard634;
        *var_guard635_slot = var_guard635;
        *var_guard636_slot = var_guard636;
        *var_guard637_slot = var_guard637;
        *var_guard638_slot = var_guard638;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_66(
        p: &Parameters,
        var_abdrain_i: f64,
        var_alphaav: f64,
        var_cbbtgatd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn5: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fstopgat_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard627: f64,
        var_guard637: f64,
        var_guard638: f64,
        var_id__blk213: f64,
        var_ijunbot: f64,
        var_ijunbot_dn5: f64,
        var_ijunbot_dn6: f64,
        var_ijunbot_dn7: f64,
        var_ijunbot_dn8: f64,
        var_ijunsti: f64,
        var_ijunsti_dn5: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_isrh: f64,
        var_isrh_dn5: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_itat: f64,
        var_itat_dn5: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
        var_pgatd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn5: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_v2: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbimin_d: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_vbrgatd_i: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn5: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_vmax_d: f64,
        var_wdepnulrinvgat_d: f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_guard639_slot: &mut f64,
        var_guard640_slot: &mut f64,
        var_guard641_slot: &mut f64,
        var_guard642_slot: &mut f64,
        var_guard643_slot: &mut f64,
        var_guard644_slot: &mut f64,
        var_guard645_slot: &mut f64,
        var_guard646_slot: &mut f64,
        var_guard647_slot: &mut f64,
        var_guard648_slot: &mut f64,
        var_i2_slot: &mut f64,
        var_i2_dn5_slot: &mut f64,
        var_i2_dn6_slot: &mut f64,
        var_i2_dn7_slot: &mut f64,
        var_i2_dn8_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_guard639: f64 = *var_guard639_slot;
        let mut var_guard640: f64 = *var_guard640_slot;
        let mut var_guard641: f64 = *var_guard641_slot;
        let mut var_guard642: f64 = *var_guard642_slot;
        let mut var_guard643: f64 = *var_guard643_slot;
        let mut var_guard644: f64 = *var_guard644_slot;
        let mut var_guard645: f64 = *var_guard645_slot;
        let mut var_guard646: f64 = *var_guard646_slot;
        let mut var_guard647: f64 = *var_guard647_slot;
        let mut var_guard648: f64 = *var_guard648_slot;
        let mut var_i2: f64 = *var_i2_slot;
        let mut var_i2_dn5: f64 = *var_i2_dn5_slot;
        let mut var_i2_dn6: f64 = *var_i2_dn6_slot;
        let mut var_i2_dn7: f64 = *var_i2_dn7_slot;
        let mut var_i2_dn8: f64 = *var_i2_dn8_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign32450_e40630, assign32450_e40630_d_n5, assign32450_e40630_d_n6, assign32450_e40630_d_n7, assign32450_e40630_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard637 == 0.0)) && (var_guard638 != 0.0)) {
        let assign32450_e40625: f64 = (var_vbirgatd_i - var_vbbt);
        let assign32450_e40627: f64 = (assign32450_e40625 * var_vbirgatinv_d);
        let assign32450_e40628: f64 = (assign32450_e40627).sqrt();
        (assign32450_e40628, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32450_e40630;
        var_tmp_dn5 = assign32450_e40630_d_n5;
        var_tmp_dn6 = assign32450_e40630_d_n6;
        var_tmp_dn7 = assign32450_e40630_d_n7;
        var_tmp_dn8 = assign32450_e40630_d_n8;

        let (assign32460_e40651, assign32460_e40651_d_n5, assign32460_e40651_d_n6, assign32460_e40651_d_n7, assign32460_e40651_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard637 == 0.0)) && (var_guard638 == 0.0)) {
        let assign32460_e40645: f64 = (var_vbirgatd_i - var_vbbt);
        let assign32460_e40647: f64 = (assign32460_e40645 * var_vbirgatinv_d);
        let assign32460_e40649: f64 = (assign32460_e40647).powf(var_pgatd_i);
        (assign32460_e40649, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32460_e40651;
        var_tmp_dn5 = assign32460_e40651_d_n5;
        var_tmp_dn6 = assign32460_e40651_d_n6;
        var_tmp_dn7 = assign32460_e40651_d_n7;
        var_tmp_dn8 = assign32460_e40651_d_n8;

        let (assign32470_e40671, assign32470_e40671_d_n5, assign32470_e40671_d_n6, assign32470_e40671_d_n7, assign32470_e40671_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32470_e40664: f64 = (var_vbirgatd_i - var_vbbt);
        let assign32470_e40666: f64 = (assign32470_e40664 * var_wdepnulrinvgat_d);
        let assign32470_e40668: f64 = (assign32470_e40666 / var_tmp);
        let assign32470_e40669: f64 = (var_one_over_one_minus_pgat_d * assign32470_e40668);
        (assign32470_e40669, (var_one_over_one_minus_pgat_d * (-((assign32470_e40666 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign32470_e40666 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign32470_e40666 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign32470_e40666 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign32470_e40671;
        var_fmaxr_dn5 = assign32470_e40671_d_n5;
        var_fmaxr_dn6 = assign32470_e40671_d_n6;
        var_fmaxr_dn7 = assign32470_e40671_d_n7;
        var_fmaxr_dn8 = assign32470_e40671_d_n8;

        let assign32480_e40673: f64 = (-var_fbbtgat_d);
        let assign32480_e40675: f64 = (assign32480_e40673 / var_fmaxr);
        let assign32480_e40676: f64 = (assign32480_e40675).abs();
        let assign32480_e40678: f64 = if assign32480_e40676 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard639 = assign32480_e40678;

        let (assign32490_e40696, assign32490_e40696_d_n5, assign32490_e40696_d_n6, assign32490_e40696_d_n7, assign32490_e40696_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard637 == 0.0)) && (var_guard639 != 0.0)) {
        let assign32490_e40691: f64 = (-var_fbbtgat_d);
        let assign32490_e40693: f64 = (assign32490_e40691 / var_fmaxr);
        let assign32490_e40694: f64 = (assign32490_e40693).exp();
        (assign32490_e40694, (assign32490_e40694 * ((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32490_e40691 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign32490_e40694 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32490_e40691 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign32490_e40694 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32490_e40691 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign32490_e40694 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32490_e40691 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32490_e40696;
        var_tmp_dn5 = assign32490_e40696_d_n5;
        var_tmp_dn6 = assign32490_e40696_d_n6;
        var_tmp_dn7 = assign32490_e40696_d_n7;
        var_tmp_dn8 = assign32490_e40696_d_n8;

        let assign32500_e40698: f64 = (-var_fbbtgat_d);
        let assign32500_e40700: f64 = (assign32500_e40698 / var_fmaxr);
        let assign32500_e40702: f64 = if assign32500_e40700 < 0.0 { 1.0 } else { 0.0 };
        var_guard640 = assign32500_e40702;

        let (assign32510_e40753, assign32510_e40753_d_n5, assign32510_e40753_d_n6, assign32510_e40753_d_n7, assign32510_e40753_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard637 == 0.0)) && (var_guard639 == 0.0)) && (var_guard640 != 0.0)) {
        let assign32510_e40720: f64 = (-230.25850929940458);
        let assign32510_e40722: f64 = (-var_fbbtgat_d);
        let assign32510_e40724: f64 = (assign32510_e40722 / var_fmaxr);
        let assign32510_e40725: f64 = (assign32510_e40720 - assign32510_e40724);
        let assign32510_e40729: f64 = (-230.25850929940458);
        let assign32510_e40731: f64 = (-var_fbbtgat_d);
        let assign32510_e40733: f64 = (assign32510_e40731 / var_fmaxr);
        let assign32510_e40734: f64 = (assign32510_e40729 - assign32510_e40733);
        let assign32510_e40737: f64 = (-230.25850929940458);
        let assign32510_e40739: f64 = (-var_fbbtgat_d);
        let assign32510_e40741: f64 = (assign32510_e40739 / var_fmaxr);
        let assign32510_e40742: f64 = (assign32510_e40737 - assign32510_e40741);
        let assign32510_e40744: f64 = (assign32510_e40742 * 0.3333333333333333);
        let assign32510_e40745: f64 = (1.0 + assign32510_e40744);
        let assign32510_e40746: f64 = (assign32510_e40734 * assign32510_e40745);
        let assign32510_e40747: f64 = (0.5 * assign32510_e40746);
        let assign32510_e40748: f64 = (1.0 + assign32510_e40747);
        let assign32510_e40749: f64 = (assign32510_e40725 * assign32510_e40748);
        let assign32510_e40750: f64 = (1.0 + assign32510_e40749);
        let assign32510_e40751: f64 = (1e-100 / assign32510_e40750);
        (assign32510_e40751, (-((1e-100 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32510_e40722 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign32510_e40748) + (assign32510_e40725 * (0.5 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32510_e40731 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign32510_e40745) + (assign32510_e40734 * ((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32510_e40739 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign32510_e40750 * assign32510_e40750))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32510_e40722 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign32510_e40748) + (assign32510_e40725 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32510_e40731 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign32510_e40745) + (assign32510_e40734 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32510_e40739 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign32510_e40750 * assign32510_e40750))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32510_e40722 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign32510_e40748) + (assign32510_e40725 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32510_e40731 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign32510_e40745) + (assign32510_e40734 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32510_e40739 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign32510_e40750 * assign32510_e40750))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32510_e40722 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign32510_e40748) + (assign32510_e40725 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32510_e40731 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign32510_e40745) + (assign32510_e40734 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32510_e40739 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign32510_e40750 * assign32510_e40750))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32510_e40753;
        var_tmp_dn5 = assign32510_e40753_d_n5;
        var_tmp_dn6 = assign32510_e40753_d_n6;
        var_tmp_dn7 = assign32510_e40753_d_n7;
        var_tmp_dn8 = assign32510_e40753_d_n8;

        let (assign32520_e40802, assign32520_e40802_d_n5, assign32520_e40802_d_n6, assign32520_e40802_d_n7, assign32520_e40802_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard637 == 0.0)) && (var_guard639 == 0.0)) && (var_guard640 == 0.0)) {
        let assign32520_e40772: f64 = (-var_fbbtgat_d);
        let assign32520_e40774: f64 = (assign32520_e40772 / var_fmaxr);
        let assign32520_e40776: f64 = (assign32520_e40774 - 230.25850929940458);
        let assign32520_e40780: f64 = (-var_fbbtgat_d);
        let assign32520_e40782: f64 = (assign32520_e40780 / var_fmaxr);
        let assign32520_e40784: f64 = (assign32520_e40782 - 230.25850929940458);
        let assign32520_e40787: f64 = (-var_fbbtgat_d);
        let assign32520_e40789: f64 = (assign32520_e40787 / var_fmaxr);
        let assign32520_e40791: f64 = (assign32520_e40789 - 230.25850929940458);
        let assign32520_e40793: f64 = (assign32520_e40791 * 0.3333333333333333);
        let assign32520_e40794: f64 = (1.0 + assign32520_e40793);
        let assign32520_e40795: f64 = (assign32520_e40784 * assign32520_e40794);
        let assign32520_e40796: f64 = (0.5 * assign32520_e40795);
        let assign32520_e40797: f64 = (1.0 + assign32520_e40796);
        let assign32520_e40798: f64 = (assign32520_e40776 * assign32520_e40797);
        let assign32520_e40799: f64 = (1.0 + assign32520_e40798);
        let assign32520_e40800: f64 = (1e100 * assign32520_e40799);
        (assign32520_e40800, (1e100 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32520_e40772 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign32520_e40797) + (assign32520_e40776 * (0.5 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32520_e40780 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign32520_e40794) + (assign32520_e40784 * (((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32520_e40787 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32520_e40772 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign32520_e40797) + (assign32520_e40776 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32520_e40780 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign32520_e40794) + (assign32520_e40784 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32520_e40787 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32520_e40772 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign32520_e40797) + (assign32520_e40776 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32520_e40780 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign32520_e40794) + (assign32520_e40784 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32520_e40787 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32520_e40772 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign32520_e40797) + (assign32520_e40776 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32520_e40780 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign32520_e40794) + (assign32520_e40784 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32520_e40787 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32520_e40802;
        var_tmp_dn5 = assign32520_e40802_d_n5;
        var_tmp_dn6 = assign32520_e40802_d_n6;
        var_tmp_dn7 = assign32520_e40802_d_n7;
        var_tmp_dn8 = assign32520_e40802_d_n8;

        let (assign32530_e40822, assign32530_e40822_d_n5, assign32530_e40822_d_n6, assign32530_e40822_d_n7, assign32530_e40822_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32530_e40815: f64 = (var_v2 * var_fmaxr);
        let assign32530_e40817: f64 = (assign32530_e40815 * var_fmaxr);
        let assign32530_e40819: f64 = (assign32530_e40817 * var_tmp);
        let assign32530_e40820: f64 = (var_cbbtgatd_i * assign32530_e40819);
        (assign32530_e40820, (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign32530_e40815 * var_fmaxr_dn5)) * var_tmp) + (assign32530_e40817 * var_tmp_dn5))), (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign32530_e40815 * var_fmaxr_dn6)) * var_tmp) + (assign32530_e40817 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign32530_e40815 * var_fmaxr_dn7)) * var_tmp) + (assign32530_e40817 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign32530_e40815 * var_fmaxr_dn8)) * var_tmp) + (assign32530_e40817 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign32530_e40822;
        var_ibbt_dn5 = assign32530_e40822_d_n5;
        var_ibbt_dn6 = assign32530_e40822_d_n6;
        var_ibbt_dn7 = assign32530_e40822_d_n7;
        var_ibbt_dn8 = assign32530_e40822_d_n8;

        let assign32540_e40825: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard641 = assign32540_e40825;

        let (assign32550_e40836, assign32550_e40836_d_n5, assign32550_e40836_d_n6, assign32550_e40836_d_n7, assign32550_e40836_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard641 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign32550_e40836;
        var_fbreakdown_dn5 = assign32550_e40836_d_n5;
        var_fbreakdown_dn6 = assign32550_e40836_d_n6;
        var_fbreakdown_dn7 = assign32550_e40836_d_n7;
        var_fbreakdown_dn8 = assign32550_e40836_d_n8;

        let assign32560_e40839: f64 = (-var_alphaav);
        let assign32560_e40841: f64 = (assign32560_e40839 * var_vbrgatd_i);
        let assign32560_e40842: f64 = if var_vav > assign32560_e40841 { 1.0 } else { 0.0 };
        var_guard642 = assign32560_e40842;

        let assign32570_e40845: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard643 = assign32570_e40845;

        let (assign32580_e40875, assign32580_e40875_d_n5, assign32580_e40875_d_n6, assign32580_e40875_d_n7, assign32580_e40875_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard641 == 0.0)) && (var_guard642 != 0.0)) && (var_guard643 != 0.0)) {
        let assign32580_e40861: f64 = (var_vav * var_vbrinvgat_d);
        let assign32580_e40864: f64 = (var_vav * var_vbrinvgat_d);
        let assign32580_e40865: f64 = (assign32580_e40861 * assign32580_e40864);
        let assign32580_e40868: f64 = (var_vav * var_vbrinvgat_d);
        let assign32580_e40869: f64 = (assign32580_e40865 * assign32580_e40868);
        let assign32580_e40872: f64 = (var_vav * var_vbrinvgat_d);
        let assign32580_e40873: f64 = (assign32580_e40869 * assign32580_e40872);
        (assign32580_e40873, (((((((var_vav * var_vbrinvgat_d_dn5) * assign32580_e40864) + (assign32580_e40861 * (var_vav * var_vbrinvgat_d_dn5))) * assign32580_e40868) + (assign32580_e40865 * (var_vav * var_vbrinvgat_d_dn5))) * assign32580_e40872) + (assign32580_e40869 * (var_vav * var_vbrinvgat_d_dn5))), (((((((var_vav * var_vbrinvgat_d_dn6) * assign32580_e40864) + (assign32580_e40861 * (var_vav * var_vbrinvgat_d_dn6))) * assign32580_e40868) + (assign32580_e40865 * (var_vav * var_vbrinvgat_d_dn6))) * assign32580_e40872) + (assign32580_e40869 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign32580_e40864) + (assign32580_e40861 * (var_vav * var_vbrinvgat_d_dn7))) * assign32580_e40868) + (assign32580_e40865 * (var_vav * var_vbrinvgat_d_dn7))) * assign32580_e40872) + (assign32580_e40869 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign32580_e40864) + (assign32580_e40861 * (var_vav * var_vbrinvgat_d_dn8))) * assign32580_e40868) + (assign32580_e40865 * (var_vav * var_vbrinvgat_d_dn8))) * assign32580_e40872) + (assign32580_e40869 * (var_vav * var_vbrinvgat_d_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32580_e40875;
        var_tmp_dn5 = assign32580_e40875_d_n5;
        var_tmp_dn6 = assign32580_e40875_d_n6;
        var_tmp_dn7 = assign32580_e40875_d_n7;
        var_tmp_dn8 = assign32580_e40875_d_n8;

        let (assign32590_e40897, assign32590_e40897_d_n5, assign32590_e40897_d_n6, assign32590_e40897_d_n7, assign32590_e40897_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard641 == 0.0)) && (var_guard642 != 0.0)) && (var_guard643 == 0.0)) {
        let assign32590_e40892: f64 = (var_vav * var_vbrinvgat_d);
        let assign32590_e40893: f64 = (assign32590_e40892).abs();
        let assign32590_e40895: f64 = (assign32590_e40893).powf(var_pbrgatd_i);
        (assign32590_e40895, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign32590_e40893).powf(var_pbrgatd_i - 1.0) * if assign32590_e40892 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) })) } } else { (assign32590_e40895 * (var_pbrgatd_i * (if assign32590_e40892 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) } / assign32590_e40893))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign32590_e40893).powf(var_pbrgatd_i - 1.0) * if assign32590_e40892 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign32590_e40895 * (var_pbrgatd_i * (if assign32590_e40892 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign32590_e40893))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign32590_e40893).powf(var_pbrgatd_i - 1.0) * if assign32590_e40892 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign32590_e40895 * (var_pbrgatd_i * (if assign32590_e40892 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign32590_e40893))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign32590_e40893).powf(var_pbrgatd_i - 1.0) * if assign32590_e40892 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign32590_e40895 * (var_pbrgatd_i * (if assign32590_e40892 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign32590_e40893))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32590_e40897;
        var_tmp_dn5 = assign32590_e40897_d_n5;
        var_tmp_dn6 = assign32590_e40897_d_n6;
        var_tmp_dn7 = assign32590_e40897_d_n7;
        var_tmp_dn8 = assign32590_e40897_d_n8;

        let (assign32600_e40915, assign32600_e40915_d_n5, assign32600_e40915_d_n6, assign32600_e40915_d_n7, assign32600_e40915_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard641 == 0.0)) && (var_guard642 != 0.0)) {
        let assign32600_e40912: f64 = (1.0 - var_tmp);
        let assign32600_e40913: f64 = (1.0 / assign32600_e40912);
        (assign32600_e40913, (-((-var_tmp_dn5) / (assign32600_e40912 * assign32600_e40912))), (-((-var_tmp_dn6) / (assign32600_e40912 * assign32600_e40912))), (-((-var_tmp_dn7) / (assign32600_e40912 * assign32600_e40912))), (-((-var_tmp_dn8) / (assign32600_e40912 * assign32600_e40912))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign32600_e40915;
        var_fbreakdown_dn5 = assign32600_e40915_d_n5;
        var_fbreakdown_dn6 = assign32600_e40915_d_n6;
        var_fbreakdown_dn7 = assign32600_e40915_d_n7;
        var_fbreakdown_dn8 = assign32600_e40915_d_n8;

        let (assign32610_e40938, assign32610_e40938_d_n5, assign32610_e40938_d_n6, assign32610_e40938_d_n7, assign32610_e40938_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) && (var_guard641 == 0.0)) && (var_guard642 == 0.0)) {
        let assign32610_e40932: f64 = (var_alphaav * var_vbrgatd_i);
        let assign32610_e40933: f64 = (var_vav + assign32610_e40932);
        let assign32610_e40935: f64 = (assign32610_e40933 * var_slopegat_d);
        let assign32610_e40936: f64 = (var_fstopgat_d + assign32610_e40935);
        (assign32610_e40936, (assign32610_e40933 * var_slopegat_d_dn5), (assign32610_e40933 * var_slopegat_d_dn6), (assign32610_e40933 * var_slopegat_d_dn7), (assign32610_e40933 * var_slopegat_d_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign32610_e40938;
        var_fbreakdown_dn5 = assign32610_e40938_d_n5;
        var_fbreakdown_dn6 = assign32610_e40938_d_n6;
        var_fbreakdown_dn7 = assign32610_e40938_d_n7;
        var_fbreakdown_dn8 = assign32610_e40938_d_n8;

        let (assign32620_e40957, assign32620_e40957_d_n5, assign32620_e40957_d_n6, assign32620_e40957_d_n7, assign32620_e40957_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard627 == 0.0)) {
        let assign32620_e40948: f64 = (var_id__blk213 + var_isrh);
        let assign32620_e40950: f64 = (assign32620_e40948 + var_itat);
        let assign32620_e40952: f64 = (assign32620_e40950 + var_ibbt);
        let assign32620_e40953: f64 = (p.p29 * assign32620_e40952);
        let assign32620_e40955: f64 = (assign32620_e40953 * var_fbreakdown);
        (assign32620_e40955, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign32620_e40953 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign32620_e40953 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign32620_e40953 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign32620_e40953 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign32620_e40957;
        var_ijungat_dn5 = assign32620_e40957_d_n5;
        var_ijungat_dn6 = assign32620_e40957_d_n6;
        var_ijungat_dn7 = assign32620_e40957_d_n7;
        var_ijungat_dn8 = assign32620_e40957_d_n8;

        let (assign32630_e40973, assign32630_e40973_d_n5, assign32630_e40973_d_n6, assign32630_e40973_d_n7, assign32630_e40973_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign32630_e40963: f64 = (var_abdrain_i * var_ijunbot);
        let assign32630_e40966: f64 = (var_lsdrain_i * var_ijunsti);
        let assign32630_e40967: f64 = (assign32630_e40963 + assign32630_e40966);
        let assign32630_e40970: f64 = (var_lgdrain_i * var_ijungat);
        let assign32630_e40971: f64 = (assign32630_e40967 + assign32630_e40970);
        (assign32630_e40971, (((var_abdrain_i * var_ijunbot_dn5) + (var_lsdrain_i * var_ijunsti_dn5)) + (var_lgdrain_i * var_ijungat_dn5)), (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)),)
    } else {
        (var_i2, var_i2_dn5, var_i2_dn6, var_i2_dn7, var_i2_dn8,)
    }
};
        var_i2 = assign32630_e40973;
        var_i2_dn5 = assign32630_e40973_d_n5;
        var_i2_dn6 = assign32630_e40973_d_n6;
        var_i2_dn7 = assign32630_e40973_d_n7;
        var_i2_dn8 = assign32630_e40973_d_n8;

        let (assign32640_e40979,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign32640_e40979;

        let (assign32650_e40985,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign32650_e40985;

        let assign32660_e40997: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard644 = assign32660_e40997;

        let assign32740_e41083: f64 = if var_v3 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard645 = assign32740_e41083;

        let assign32750_e41085: f64 = (-0.5);
        let assign32750_e41088: f64 = (var_v3 * var_phitdinv);
        let assign32750_e41089: f64 = (assign32750_e41085 * assign32750_e41088);
        let assign32750_e41090: f64 = (assign32750_e41089).abs();
        let assign32750_e41092: f64 = if assign32750_e41090 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard646 = assign32750_e41092;

        let (assign32760_e41110,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 != 0.0)) && (var_guard646 != 0.0)) {
        let assign32760_e41103: f64 = (-0.5);
        let assign32760_e41106: f64 = (var_v3 * var_phitdinv);
        let assign32760_e41107: f64 = (assign32760_e41103 * assign32760_e41106);
        let assign32760_e41108: f64 = (assign32760_e41107).exp();
        (assign32760_e41108,)
    } else {
        (var_z,)
    }
};
        var_z = assign32760_e41110;

        let assign32770_e41112: f64 = (-0.5);
        let assign32770_e41115: f64 = (var_v3 * var_phitdinv);
        let assign32770_e41116: f64 = (assign32770_e41112 * assign32770_e41115);
        let assign32770_e41118: f64 = if assign32770_e41116 < 0.0 { 1.0 } else { 0.0 };
        var_guard647 = assign32770_e41118;

        let (assign32780_e41173,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 != 0.0)) && (var_guard646 == 0.0)) && (var_guard647 != 0.0)) {
        let assign32780_e41134: f64 = (-230.25850929940458);
        let assign32780_e41136: f64 = (-0.5);
        let assign32780_e41139: f64 = (var_v3 * var_phitdinv);
        let assign32780_e41140: f64 = (assign32780_e41136 * assign32780_e41139);
        let assign32780_e41141: f64 = (assign32780_e41134 - assign32780_e41140);
        let assign32780_e41145: f64 = (-230.25850929940458);
        let assign32780_e41147: f64 = (-0.5);
        let assign32780_e41150: f64 = (var_v3 * var_phitdinv);
        let assign32780_e41151: f64 = (assign32780_e41147 * assign32780_e41150);
        let assign32780_e41152: f64 = (assign32780_e41145 - assign32780_e41151);
        let assign32780_e41155: f64 = (-230.25850929940458);
        let assign32780_e41157: f64 = (-0.5);
        let assign32780_e41160: f64 = (var_v3 * var_phitdinv);
        let assign32780_e41161: f64 = (assign32780_e41157 * assign32780_e41160);
        let assign32780_e41162: f64 = (assign32780_e41155 - assign32780_e41161);
        let assign32780_e41164: f64 = (assign32780_e41162 * 0.3333333333333333);
        let assign32780_e41165: f64 = (1.0 + assign32780_e41164);
        let assign32780_e41166: f64 = (assign32780_e41152 * assign32780_e41165);
        let assign32780_e41167: f64 = (0.5 * assign32780_e41166);
        let assign32780_e41168: f64 = (1.0 + assign32780_e41167);
        let assign32780_e41169: f64 = (assign32780_e41141 * assign32780_e41168);
        let assign32780_e41170: f64 = (1.0 + assign32780_e41169);
        let assign32780_e41171: f64 = (1e-100 / assign32780_e41170);
        (assign32780_e41171,)
    } else {
        (var_z,)
    }
};
        var_z = assign32780_e41173;

        let (assign32790_e41226,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 != 0.0)) && (var_guard646 == 0.0)) && (var_guard647 == 0.0)) {
        let assign32790_e41190: f64 = (-0.5);
        let assign32790_e41193: f64 = (var_v3 * var_phitdinv);
        let assign32790_e41194: f64 = (assign32790_e41190 * assign32790_e41193);
        let assign32790_e41196: f64 = (assign32790_e41194 - 230.25850929940458);
        let assign32790_e41200: f64 = (-0.5);
        let assign32790_e41203: f64 = (var_v3 * var_phitdinv);
        let assign32790_e41204: f64 = (assign32790_e41200 * assign32790_e41203);
        let assign32790_e41206: f64 = (assign32790_e41204 - 230.25850929940458);
        let assign32790_e41209: f64 = (-0.5);
        let assign32790_e41212: f64 = (var_v3 * var_phitdinv);
        let assign32790_e41213: f64 = (assign32790_e41209 * assign32790_e41212);
        let assign32790_e41215: f64 = (assign32790_e41213 - 230.25850929940458);
        let assign32790_e41217: f64 = (assign32790_e41215 * 0.3333333333333333);
        let assign32790_e41218: f64 = (1.0 + assign32790_e41217);
        let assign32790_e41219: f64 = (assign32790_e41206 * assign32790_e41218);
        let assign32790_e41220: f64 = (0.5 * assign32790_e41219);
        let assign32790_e41221: f64 = (1.0 + assign32790_e41220);
        let assign32790_e41222: f64 = (assign32790_e41196 * assign32790_e41221);
        let assign32790_e41223: f64 = (1.0 + assign32790_e41222);
        let assign32790_e41224: f64 = (1e100 * assign32790_e41223);
        (assign32790_e41224,)
    } else {
        (var_z,)
    }
};
        var_z = assign32790_e41226;

        let (assign32800_e41238,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 != 0.0)) {
        let assign32800_e41236: f64 = (1.0 / var_z);
        (assign32800_e41236,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign32800_e41238;

        let (assign32810_e41250,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 != 0.0)) {
        let assign32810_e41248: f64 = (var_zinv * var_zinv);
        (assign32810_e41248,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign32810_e41250;

        let (assign32820_e41269,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 == 0.0)) {
        let assign32820_e41262: f64 = (var_v3 - var_vmax_d);
        let assign32820_e41264: f64 = (assign32820_e41262 * var_phitdinv);
        let assign32820_e41265: f64 = (1.0 + assign32820_e41264);
        let assign32820_e41267: f64 = (assign32820_e41265 * var_exp_vmax_over_phitd_d);
        (assign32820_e41267,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign32820_e41269;

        let (assign32830_e41281,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 == 0.0)) {
        let assign32830_e41279: f64 = (var_idmult).sqrt();
        (assign32830_e41279,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign32830_e41281;

        let (assign32840_e41294,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 == 0.0)) {
        let assign32840_e41292: f64 = (1.0 / var_zinv);
        (assign32840_e41292,)
    } else {
        (var_z,)
    }
};
        var_z = assign32840_e41294;

        let (assign32850_e41304,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) {
        let assign32850_e41302: f64 = (var_idmult - 1.0);
        (assign32850_e41302,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign32850_e41304;

        let assign32860_e41307: f64 = if var_v3 > 0.0 { 1.0 } else { 0.0 };
        var_guard648 = assign32860_e41307;

        let (assign32870_e41333,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) && (var_guard648 != 0.0)) {
        let assign32870_e41319: f64 = (2.0 + var_z);
        let assign32870_e41322: f64 = (var_z + 1.0);
        let assign32870_e41325: f64 = (var_z + 3.0);
        let assign32870_e41326: f64 = (assign32870_e41322 * assign32870_e41325);
        let assign32870_e41327: f64 = (assign32870_e41326).sqrt();
        let assign32870_e41328: f64 = (assign32870_e41319 + assign32870_e41327);
        let assign32870_e41329: f64 = (assign32870_e41328).ln();
        let assign32870_e41330: f64 = (var_phitd * assign32870_e41329);
        let assign32870_e41331: f64 = (2.0 * assign32870_e41330);
        (assign32870_e41331,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign32870_e41333;

        let (assign32880_e41367,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) && (var_guard648 == 0.0)) {
        let assign32880_e41343: f64 = (-var_v3);
        let assign32880_e41348: f64 = (2.0 * var_zinv);
        let assign32880_e41350: f64 = (assign32880_e41348 + 1.0);
        let assign32880_e41353: f64 = (1.0 + var_zinv);
        let assign32880_e41357: f64 = (3.0 * var_zinv);
        let assign32880_e41358: f64 = (1.0 + assign32880_e41357);
        let assign32880_e41359: f64 = (assign32880_e41353 * assign32880_e41358);
        let assign32880_e41360: f64 = (assign32880_e41359).sqrt();
        let assign32880_e41361: f64 = (assign32880_e41350 + assign32880_e41360);
        let assign32880_e41362: f64 = (assign32880_e41361).ln();
        let assign32880_e41363: f64 = (var_phitd * assign32880_e41362);
        let assign32880_e41364: f64 = (2.0 * assign32880_e41363);
        let assign32880_e41365: f64 = (assign32880_e41343 + assign32880_e41364);
        (assign32880_e41365,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign32880_e41367;

        let (assign32890_e41377,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) {
        let assign32890_e41375: f64 = (var_vbimin_d - var_two_psistar);
        (assign32890_e41375,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign32890_e41377;

        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_guard639_slot = var_guard639;
        *var_guard640_slot = var_guard640;
        *var_guard641_slot = var_guard641;
        *var_guard642_slot = var_guard642;
        *var_guard643_slot = var_guard643;
        *var_guard644_slot = var_guard644;
        *var_guard645_slot = var_guard645;
        *var_guard646_slot = var_guard646;
        *var_guard647_slot = var_guard647;
        *var_guard648_slot = var_guard648;
        *var_i2_slot = var_i2;
        *var_i2_dn5_slot = var_i2_dn5;
        *var_i2_dn6_slot = var_i2_dn6;
        *var_i2_dn7_slot = var_i2_dn7;
        *var_i2_dn8_slot = var_i2_dn8;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_idmult_slot = var_idmult;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_vbbt_slot = var_vbbt;
        *var_vjlim_slot = var_vjlim;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_67(
        var_abdrain_i: f64,
        var_atatbot_d: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_ftdbot_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard644: f64,
        var_idmult: f64,
        var_idsatbot_d: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_perfc: f64,
        var_phitd: f64,
        var_phitr: f64,
        var_two_psistar: f64,
        var_v3: f64,
        var_vbbtlim_d: f64,
        var_vbibot_d: f64,
        var_vbirbotinv_d: f64,
        var_vjlim: f64,
        var_wdepnulrbot_d: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard649_slot: &mut f64,
        var_guard650_slot: &mut f64,
        var_guard651_slot: &mut f64,
        var_guard652_slot: &mut f64,
        var_guard653_slot: &mut f64,
        var_guard654_slot: &mut f64,
        var_guard655_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard649: f64 = *var_guard649_slot;
        let mut var_guard650: f64 = *var_guard650_slot;
        let mut var_guard651: f64 = *var_guard651_slot;
        let mut var_guard652: f64 = *var_guard652_slot;
        let mut var_guard653: f64 = *var_guard653_slot;
        let mut var_guard654: f64 = *var_guard654_slot;
        let mut var_guard655: f64 = *var_guard655_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign32900_e41404,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) {
        let assign32900_e41386: f64 = (var_v3 + var_vjlim);
        let assign32900_e41389: f64 = (var_v3 - var_vjlim);
        let assign32900_e41392: f64 = (var_v3 - var_vjlim);
        let assign32900_e41393: f64 = (assign32900_e41389 * assign32900_e41392);
        let assign32900_e41396: f64 = (4.0 * var_phitd);
        let assign32900_e41398: f64 = (assign32900_e41396 * var_phitd);
        let assign32900_e41399: f64 = (assign32900_e41393 + assign32900_e41398);
        let assign32900_e41400: f64 = (assign32900_e41399).sqrt();
        let assign32900_e41401: f64 = (assign32900_e41386 - assign32900_e41400);
        let assign32900_e41402: f64 = (0.5 * assign32900_e41401);
        (assign32900_e41402,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign32900_e41404;

        let (assign32910_e41431,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) {
        let assign32910_e41413: f64 = (var_v3 + var_vbbtlim_d);
        let assign32910_e41416: f64 = (var_v3 - var_vbbtlim_d);
        let assign32910_e41419: f64 = (var_v3 - var_vbbtlim_d);
        let assign32910_e41420: f64 = (assign32910_e41416 * assign32910_e41419);
        let assign32910_e41423: f64 = (4.0 * var_phitr);
        let assign32910_e41425: f64 = (assign32910_e41423 * var_phitr);
        let assign32910_e41426: f64 = (assign32910_e41420 + assign32910_e41425);
        let assign32910_e41427: f64 = (assign32910_e41426).sqrt();
        let assign32910_e41428: f64 = (assign32910_e41413 - assign32910_e41427);
        let assign32910_e41429: f64 = (0.5 * assign32910_e41428);
        (assign32910_e41429,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign32910_e41431;

        let (assign32920_e41458,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard644 != 0.0)) {
        let assign32920_e41440: f64 = var_v3;
        let assign32920_e41443: f64 = var_v3;
        let assign32920_e41446: f64 = var_v3;
        let assign32920_e41447: f64 = (assign32920_e41443 * assign32920_e41446);
        let assign32920_e41450: f64 = (4.0 * 1e-6);
        let assign32920_e41452: f64 = (assign32920_e41450 * 1e-6);
        let assign32920_e41453: f64 = (assign32920_e41447 + assign32920_e41452);
        let assign32920_e41454: f64 = (assign32920_e41453).sqrt();
        let assign32920_e41455: f64 = (assign32920_e41440 - assign32920_e41454);
        let assign32920_e41456: f64 = (0.5 * assign32920_e41455);
        (assign32920_e41456,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign32920_e41458;

        let assign32930_e41461: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard649 = assign32930_e41461;

        let (assign32940_e41469, assign32940_e41469_d_n5, assign32940_e41469_d_n6, assign32940_e41469_d_n7, assign32940_e41469_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign32940_e41469;
        var_ijunbot_dn5 = assign32940_e41469_d_n5;
        var_ijunbot_dn6 = assign32940_e41469_d_n6;
        var_ijunbot_dn7 = assign32940_e41469_d_n7;
        var_ijunbot_dn8 = assign32940_e41469_d_n8;

        let (assign32950_e41480,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) {
        let assign32950_e41478: f64 = (var_idsatbot_d * var_idmult);
        (assign32950_e41478,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign32950_e41480;

        let assign32960_e41487: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard650 = assign32960_e41487;

        let (assign32970_e41498, assign32970_e41498_d_n5, assign32970_e41498_d_n6, assign32970_e41498_d_n7, assign32970_e41498_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign32970_e41498;
        var_isrh_dn5 = assign32970_e41498_d_n5;
        var_isrh_dn6 = assign32970_e41498_d_n6;
        var_isrh_dn7 = assign32970_e41498_d_n7;
        var_isrh_dn8 = assign32970_e41498_d_n8;

        let (assign32980_e41512,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) {
        let assign32980_e41510: f64 = (var_vbibot_d - var_vjsrh);
        (assign32980_e41510,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign32980_e41512;

        let (assign32990_e41531,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) {
        let assign32990_e41526: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign32990_e41527: f64 = (1.0 - assign32990_e41526);
        let assign32990_e41528: f64 = (assign32990_e41527).sqrt();
        let assign32990_e41529: f64 = (1.0 - assign32990_e41528);
        (assign32990_e41529,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign32990_e41531;

        let assign33000_e41534: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard651 = assign33000_e41534;

        let (assign33010_e41548,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) && (var_guard651 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign33010_e41548;

        let (assign33020_e41580,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) && (var_guard651 == 0.0)) {
        let assign33020_e41563: f64 = (var_wsrhstep * var_wsrhstep);
        let assign33020_e41565: f64 = (var_wsrhstep).ln();
        let assign33020_e41566: f64 = (assign33020_e41563 * assign33020_e41565);
        let assign33020_e41569: f64 = (1.0 - var_wsrhstep);
        let assign33020_e41570: f64 = (assign33020_e41566 / assign33020_e41569);
        let assign33020_e41572: f64 = (assign33020_e41570 + var_wsrhstep);
        let assign33020_e41576: f64 = (2.0 * var_pbotd_i);
        let assign33020_e41577: f64 = (1.0 - assign33020_e41576);
        let assign33020_e41578: f64 = (assign33020_e41572 * assign33020_e41577);
        (assign33020_e41578,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign33020_e41580;

        let (assign33030_e41594,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) {
        let assign33030_e41592: f64 = (var_wsrhstep + var_dwsrh);
        (assign33030_e41592,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign33030_e41594;

        let assign33040_e41597: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard652 = assign33040_e41597;

        let (assign33050_e41614, assign33050_e41614_d_n5, assign33050_e41614_d_n6, assign33050_e41614_d_n7, assign33050_e41614_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) && (var_guard652 != 0.0)) {
        let assign33050_e41611: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign33050_e41612: f64 = (assign33050_e41611).sqrt();
        (assign33050_e41612, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33050_e41614;
        var_tmp_dn5 = assign33050_e41614_d_n5;
        var_tmp_dn6 = assign33050_e41614_d_n6;
        var_tmp_dn7 = assign33050_e41614_d_n7;
        var_tmp_dn8 = assign33050_e41614_d_n8;

        let (assign33060_e41633, assign33060_e41633_d_n5, assign33060_e41633_d_n6, assign33060_e41633_d_n7, assign33060_e41633_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) && (var_guard652 == 0.0)) {
        let assign33060_e41629: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign33060_e41631: f64 = (assign33060_e41629).powf(var_pbotd_i);
        (assign33060_e41631, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33060_e41633;
        var_tmp_dn5 = assign33060_e41633_d_n5;
        var_tmp_dn6 = assign33060_e41633_d_n6;
        var_tmp_dn7 = assign33060_e41633_d_n7;
        var_tmp_dn8 = assign33060_e41633_d_n8;

        let (assign33070_e41647, assign33070_e41647_d_n5, assign33070_e41647_d_n6, assign33070_e41647_d_n7, assign33070_e41647_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) {
        let assign33070_e41645: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign33070_e41645, (var_wdepnulrbot_d * var_tmp_dn5), (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign33070_e41647;
        var_wdep_dn5 = assign33070_e41647_d_n5;
        var_wdep_dn6 = assign33070_e41647_d_n6;
        var_wdep_dn7 = assign33070_e41647_d_n7;
        var_wdep_dn8 = assign33070_e41647_d_n8;

        let (assign33080_e41665, assign33080_e41665_d_n5, assign33080_e41665_d_n6, assign33080_e41665_d_n7, assign33080_e41665_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) {
        let assign33080_e41660: f64 = (var_zinv - 1.0);
        let assign33080_e41662: f64 = (assign33080_e41660 * var_wdep);
        let assign33080_e41663: f64 = (var_ftdbot_d * assign33080_e41662);
        (assign33080_e41663, (var_ftdbot_d * (assign33080_e41660 * var_wdep_dn5)), (var_ftdbot_d * (assign33080_e41660 * var_wdep_dn6)), (var_ftdbot_d * (assign33080_e41660 * var_wdep_dn7)), (var_ftdbot_d * (assign33080_e41660 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign33080_e41665;
        var_asrh_dn5 = assign33080_e41665_d_n5;
        var_asrh_dn6 = assign33080_e41665_d_n6;
        var_asrh_dn7 = assign33080_e41665_d_n7;
        var_asrh_dn8 = assign33080_e41665_d_n8;

        let (assign33090_e41681, assign33090_e41681_d_n5, assign33090_e41681_d_n6, assign33090_e41681_d_n7, assign33090_e41681_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) {
        let assign33090_e41678: f64 = (var_asrh * var_wsrh);
        let assign33090_e41679: f64 = (var_csrhbotd_i * assign33090_e41678);
        (assign33090_e41679, (var_csrhbotd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign33090_e41681;
        var_isrh_dn5 = assign33090_e41681_d_n5;
        var_isrh_dn6 = assign33090_e41681_d_n6;
        var_isrh_dn7 = assign33090_e41681_d_n7;
        var_isrh_dn8 = assign33090_e41681_d_n8;

        let assign33100_e41684: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard653 = assign33100_e41684;

        let (assign33110_e41695, assign33110_e41695_d_n5, assign33110_e41695_d_n6, assign33110_e41695_d_n7, assign33110_e41695_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign33110_e41695;
        var_itat_dn5 = assign33110_e41695_d_n5;
        var_itat_dn6 = assign33110_e41695_d_n6;
        var_itat_dn7 = assign33110_e41695_d_n7;
        var_itat_dn8 = assign33110_e41695_d_n8;

        let (assign33120_e41713, assign33120_e41713_d_n5, assign33120_e41713_d_n6, assign33120_e41713_d_n7, assign33120_e41713_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33120_e41708: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign33120_e41710: f64 = (assign33120_e41708 / var_vbi_minus_vjsrh);
        let assign33120_e41711: f64 = (var_btatpartbot_d * assign33120_e41710);
        (assign33120_e41711, (var_btatpartbot_d * ((var_wdep_dn5 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign33120_e41713;
        var_btat_dn5 = assign33120_e41713_d_n5;
        var_btat_dn6 = assign33120_e41713_d_n6;
        var_btat_dn7 = assign33120_e41713_d_n7;
        var_btat_dn8 = assign33120_e41713_d_n8;

        let (assign33130_e41729, assign33130_e41729_d_n5, assign33130_e41729_d_n6, assign33130_e41729_d_n7, assign33130_e41729_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33130_e41725: f64 = (0.666666666666667 * var_atatbot_d);
        let assign33130_e41727: f64 = (assign33130_e41725 / var_btat);
        (assign33130_e41727, (-((assign33130_e41725 * var_btat_dn5) / (var_btat * var_btat))), (-((assign33130_e41725 * var_btat_dn6) / (var_btat * var_btat))), (-((assign33130_e41725 * var_btat_dn7) / (var_btat * var_btat))), (-((assign33130_e41725 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign33130_e41729;
        var_twoatatoverthreebtat_dn5 = assign33130_e41729_d_n5;
        var_twoatatoverthreebtat_dn6 = assign33130_e41729_d_n6;
        var_twoatatoverthreebtat_dn7 = assign33130_e41729_d_n7;
        var_twoatatoverthreebtat_dn8 = assign33130_e41729_d_n8;

        let (assign33140_e41743, assign33140_e41743_d_n5, assign33140_e41743_d_n6, assign33140_e41743_d_n7, assign33140_e41743_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33140_e41741: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign33140_e41741, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign33140_e41743;
        var_umaxbeforelimiting_dn5 = assign33140_e41743_d_n5;
        var_umaxbeforelimiting_dn6 = assign33140_e41743_d_n6;
        var_umaxbeforelimiting_dn7 = assign33140_e41743_d_n7;
        var_umaxbeforelimiting_dn8 = assign33140_e41743_d_n8;

        let (assign33150_e41764, assign33150_e41764_d_n5, assign33150_e41764_d_n6, assign33150_e41764_d_n7, assign33150_e41764_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33150_e41755: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign33150_e41758: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign33150_e41760: f64 = (assign33150_e41758 + 1.0);
        let assign33150_e41761: f64 = (assign33150_e41755 / assign33150_e41760);
        let assign33150_e41762: f64 = (assign33150_e41761).sqrt();
        (assign33150_e41762, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign33150_e41760) - (assign33150_e41755 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign33150_e41760 * assign33150_e41760)) / (2.0 * assign33150_e41762)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign33150_e41760) - (assign33150_e41755 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign33150_e41760 * assign33150_e41760)) / (2.0 * assign33150_e41762)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign33150_e41760) - (assign33150_e41755 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign33150_e41760 * assign33150_e41760)) / (2.0 * assign33150_e41762)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign33150_e41760) - (assign33150_e41755 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign33150_e41760 * assign33150_e41760)) / (2.0 * assign33150_e41762)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign33150_e41764;
        var_umax_dn5 = assign33150_e41764_d_n5;
        var_umax_dn6 = assign33150_e41764_d_n6;
        var_umax_dn7 = assign33150_e41764_d_n7;
        var_umax_dn8 = assign33150_e41764_d_n8;

        let (assign33160_e41777, assign33160_e41777_d_n5, assign33160_e41777_d_n6, assign33160_e41777_d_n7, assign33160_e41777_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33160_e41775: f64 = (var_umax).sqrt();
        (assign33160_e41775, (var_umax_dn5 / (2.0 * assign33160_e41775)), (var_umax_dn6 / (2.0 * assign33160_e41775)), (var_umax_dn7 / (2.0 * assign33160_e41775)), (var_umax_dn8 / (2.0 * assign33160_e41775)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign33160_e41777;
        var_sqrtumax_dn5 = assign33160_e41777_d_n5;
        var_sqrtumax_dn6 = assign33160_e41777_d_n6;
        var_sqrtumax_dn7 = assign33160_e41777_d_n7;
        var_sqrtumax_dn8 = assign33160_e41777_d_n8;

        let (assign33170_e41791, assign33170_e41791_d_n5, assign33170_e41791_d_n6, assign33170_e41791_d_n7, assign33170_e41791_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33170_e41789: f64 = (var_umax * var_sqrtumax);
        (assign33170_e41789, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign33170_e41791;
        var_umaxpoweronepointfive_dn5 = assign33170_e41791_d_n5;
        var_umaxpoweronepointfive_dn6 = assign33170_e41791_d_n6;
        var_umaxpoweronepointfive_dn7 = assign33170_e41791_d_n7;
        var_umaxpoweronepointfive_dn8 = assign33170_e41791_d_n8;

        let assign33180_e41793: f64 = (-var_pbotd_i);
        let assign33180_e41795: f64 = (assign33180_e41793 * var_one_over_one_minus_pbot_d);
        let assign33180_e41797: f64 = (-1.0);
        let assign33180_e41798: f64 = if assign33180_e41795 == assign33180_e41797 { 1.0 } else { 0.0 };
        var_guard654 = assign33180_e41798;

        let (assign33190_e41818, assign33190_e41818_d_n5, assign33190_e41818_d_n6, assign33190_e41818_d_n7, assign33190_e41818_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) && (var_guard654 != 0.0)) {
        let assign33190_e41814: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33190_e41815: f64 = (1.0 + assign33190_e41814);
        let assign33190_e41816: f64 = (1.0 / assign33190_e41815);
        (assign33190_e41816, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign33190_e41815 * assign33190_e41815))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign33190_e41815 * assign33190_e41815))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign33190_e41815 * assign33190_e41815))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign33190_e41815 * assign33190_e41815))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign33190_e41818;
        var_wgamma_dn5 = assign33190_e41818_d_n5;
        var_wgamma_dn6 = assign33190_e41818_d_n6;
        var_wgamma_dn7 = assign33190_e41818_d_n7;
        var_wgamma_dn8 = assign33190_e41818_d_n8;

        let (assign33200_e41842, assign33200_e41842_d_n5, assign33200_e41842_d_n6, assign33200_e41842_d_n7, assign33200_e41842_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) && (var_guard654 == 0.0)) {
        let assign33200_e41834: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33200_e41835: f64 = (1.0 + assign33200_e41834);
        let assign33200_e41837: f64 = (-var_pbotd_i);
        let assign33200_e41839: f64 = (assign33200_e41837 * var_one_over_one_minus_pbot_d);
        let assign33200_e41840: f64 = (assign33200_e41835).powf(assign33200_e41839);
        (assign33200_e41840, if 0.0 == 0.0 && ((assign33200_e41839) as f64).is_finite() && ((assign33200_e41839) as f64).fract() == 0.0 { if assign33200_e41839 == 0.0 { 0.0 } else { (assign33200_e41839 * ((assign33200_e41835).powf(assign33200_e41839 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign33200_e41840 * (assign33200_e41839 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign33200_e41835))) }, if 0.0 == 0.0 && ((assign33200_e41839) as f64).is_finite() && ((assign33200_e41839) as f64).fract() == 0.0 { if assign33200_e41839 == 0.0 { 0.0 } else { (assign33200_e41839 * ((assign33200_e41835).powf(assign33200_e41839 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign33200_e41840 * (assign33200_e41839 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign33200_e41835))) }, if 0.0 == 0.0 && ((assign33200_e41839) as f64).is_finite() && ((assign33200_e41839) as f64).fract() == 0.0 { if assign33200_e41839 == 0.0 { 0.0 } else { (assign33200_e41839 * ((assign33200_e41835).powf(assign33200_e41839 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign33200_e41840 * (assign33200_e41839 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign33200_e41835))) }, if 0.0 == 0.0 && ((assign33200_e41839) as f64).is_finite() && ((assign33200_e41839) as f64).fract() == 0.0 { if assign33200_e41839 == 0.0 { 0.0 } else { (assign33200_e41839 * ((assign33200_e41835).powf(assign33200_e41839 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign33200_e41840 * (assign33200_e41839 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign33200_e41835))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign33200_e41842;
        var_wgamma_dn5 = assign33200_e41842_d_n5;
        var_wgamma_dn6 = assign33200_e41842_d_n6;
        var_wgamma_dn7 = assign33200_e41842_d_n7;
        var_wgamma_dn8 = assign33200_e41842_d_n8;

        let (assign33210_e41860, assign33210_e41860_d_n5, assign33210_e41860_d_n6, assign33210_e41860_d_n7, assign33210_e41860_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33210_e41854: f64 = (var_wsrh * var_wgamma);
        let assign33210_e41857: f64 = (var_wsrh + var_wgamma);
        let assign33210_e41858: f64 = (assign33210_e41854 / assign33210_e41857);
        (assign33210_e41858, ((((var_wsrh * var_wgamma_dn5) * assign33210_e41857) - (assign33210_e41854 * var_wgamma_dn5)) / (assign33210_e41857 * assign33210_e41857)), ((((var_wsrh * var_wgamma_dn6) * assign33210_e41857) - (assign33210_e41854 * var_wgamma_dn6)) / (assign33210_e41857 * assign33210_e41857)), ((((var_wsrh * var_wgamma_dn7) * assign33210_e41857) - (assign33210_e41854 * var_wgamma_dn7)) / (assign33210_e41857 * assign33210_e41857)), ((((var_wsrh * var_wgamma_dn8) * assign33210_e41857) - (assign33210_e41854 * var_wgamma_dn8)) / (assign33210_e41857 * assign33210_e41857)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign33210_e41860;
        var_wtat_dn5 = assign33210_e41860_d_n5;
        var_wtat_dn6 = assign33210_e41860_d_n6;
        var_wtat_dn7 = assign33210_e41860_d_n7;
        var_wtat_dn8 = assign33210_e41860_d_n8;

        let (assign33220_e41877, assign33220_e41877_d_n5, assign33220_e41877_d_n6, assign33220_e41877_d_n7, assign33220_e41877_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33220_e41873: f64 = (var_btat / var_sqrtumax);
        let assign33220_e41874: f64 = (0.375 * assign33220_e41873);
        let assign33220_e41875: f64 = (assign33220_e41874).sqrt();
        (assign33220_e41875, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33220_e41875)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33220_e41875)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33220_e41875)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33220_e41875)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign33220_e41877;
        var_ktat_dn5 = assign33220_e41877_d_n5;
        var_ktat_dn6 = assign33220_e41877_d_n6;
        var_ktat_dn7 = assign33220_e41877_d_n7;
        var_ktat_dn8 = assign33220_e41877_d_n8;

        let (assign33230_e41895, assign33230_e41895_d_n5, assign33230_e41895_d_n6, assign33230_e41895_d_n7, assign33230_e41895_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33230_e41890: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign33230_e41891: f64 = (2.0 * assign33230_e41890);
        let assign33230_e41893: f64 = (assign33230_e41891 - var_umax);
        (assign33230_e41893, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign33230_e41895;
        var_ltat_dn5 = assign33230_e41895_d_n5;
        var_ltat_dn6 = assign33230_e41895_d_n6;
        var_ltat_dn7 = assign33230_e41895_d_n7;
        var_ltat_dn8 = assign33230_e41895_d_n8;

        let (assign33240_e41921, assign33240_e41921_d_n5, assign33240_e41921_d_n6, assign33240_e41921_d_n7, assign33240_e41921_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33240_e41907: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign33240_e41909: f64 = (assign33240_e41907 * var_sqrtumax);
        let assign33240_e41912: f64 = (var_atatbot_d * var_umax);
        let assign33240_e41913: f64 = (assign33240_e41909 - assign33240_e41912);
        let assign33240_e41917: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33240_e41918: f64 = (0.5 * assign33240_e41917);
        let assign33240_e41919: f64 = (assign33240_e41913 + assign33240_e41918);
        (assign33240_e41919, (((((var_atatbot_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign33240_e41907 * var_sqrtumax_dn5)) - (var_atatbot_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign33240_e41907 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign33240_e41907 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign33240_e41907 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign33240_e41921;
        var_mtat_dn5 = assign33240_e41921_d_n5;
        var_mtat_dn6 = assign33240_e41921_d_n6;
        var_mtat_dn7 = assign33240_e41921_d_n7;
        var_mtat_dn8 = assign33240_e41921_d_n8;

        let (assign33250_e41937, assign33250_e41937_d_n5, assign33250_e41937_d_n6, assign33250_e41937_d_n7, assign33250_e41937_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33250_e41933: f64 = (var_ltat - 1.0);
        let assign33250_e41935: f64 = (assign33250_e41933 * var_ktat);
        (assign33250_e41935, ((var_ltat_dn5 * var_ktat) + (assign33250_e41933 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign33250_e41933 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign33250_e41933 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign33250_e41933 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign33250_e41937;
        var_xerfc_dn5 = assign33250_e41937_d_n5;
        var_xerfc_dn6 = assign33250_e41937_d_n6;
        var_xerfc_dn7 = assign33250_e41937_d_n7;
        var_xerfc_dn8 = assign33250_e41937_d_n8;

        let (assign33260_e41951, assign33260_e41951_d_n5, assign33260_e41951_d_n6, assign33260_e41951_d_n7, assign33260_e41951_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33260_e41949: f64 = (var_xerfc * var_xerfc);
        (assign33260_e41949, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign33260_e41951;
        var_ysq_dn5 = assign33260_e41951_d_n5;
        var_ysq_dn6 = assign33260_e41951_d_n6;
        var_ysq_dn7 = assign33260_e41951_d_n7;
        var_ysq_dn8 = assign33260_e41951_d_n8;

        let assign33270_e41954: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard655 = assign33270_e41954;

        let (assign33280_e41974, assign33280_e41974_d_n5, assign33280_e41974_d_n6, assign33280_e41974_d_n7, assign33280_e41974_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) && (var_guard655 != 0.0)) {
        let assign33280_e41970: f64 = (var_perfc * var_xerfc);
        let assign33280_e41971: f64 = (1.0 + assign33280_e41970);
        let assign33280_e41972: f64 = (1.0 / assign33280_e41971);
        (assign33280_e41972, (-((var_perfc * var_xerfc_dn5) / (assign33280_e41971 * assign33280_e41971))), (-((var_perfc * var_xerfc_dn6) / (assign33280_e41971 * assign33280_e41971))), (-((var_perfc * var_xerfc_dn7) / (assign33280_e41971 * assign33280_e41971))), (-((var_perfc * var_xerfc_dn8) / (assign33280_e41971 * assign33280_e41971))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign33280_e41974;
        var_terfc_dn5 = assign33280_e41974_d_n5;
        var_terfc_dn6 = assign33280_e41974_d_n6;
        var_terfc_dn7 = assign33280_e41974_d_n7;
        var_terfc_dn8 = assign33280_e41974_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard649_slot = var_guard649;
        *var_guard650_slot = var_guard650;
        *var_guard651_slot = var_guard651;
        *var_guard652_slot = var_guard652;
        *var_guard653_slot = var_guard653;
        *var_guard654_slot = var_guard654;
        *var_guard655_slot = var_guard655;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_68(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatbot_d: f64,
        var_berfc: f64,
        var_cbbtbotd_i: f64,
        var_cerfc: f64,
        var_ctatbotd_i: f64,
        var_fbbtbot_d: f64,
        var_fstopbot_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard649: f64,
        var_guard653: f64,
        var_guard655: f64,
        var_id__blk213: f64,
        var_isrh: f64,
        var_isrh_dn5: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lsdrain_i: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_pbrbotd_i: f64,
        var_perfc: f64,
        var_slopebot_d: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotd_i: f64,
        var_vbirbotinv_d: f64,
        var_vbrbotd_i: f64,
        var_vbrinvbot_d: f64,
        var_wdepnulrinvbot_d: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_xerfc: f64,
        var_xerfc_dn5: f64,
        var_xerfc_dn6: f64,
        var_xerfc_dn7: f64,
        var_xerfc_dn8: f64,
        var_ysq: f64,
        var_ysq_dn5: f64,
        var_ysq_dn6: f64,
        var_ysq_dn7: f64,
        var_ysq_dn8: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard656_slot: &mut f64,
        var_guard657_slot: &mut f64,
        var_guard658_slot: &mut f64,
        var_guard659_slot: &mut f64,
        var_guard660_slot: &mut f64,
        var_guard661_slot: &mut f64,
        var_guard662_slot: &mut f64,
        var_guard663_slot: &mut f64,
        var_guard664_slot: &mut f64,
        var_guard665_slot: &mut f64,
        var_guard666_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard656: f64 = *var_guard656_slot;
        let mut var_guard657: f64 = *var_guard657_slot;
        let mut var_guard658: f64 = *var_guard658_slot;
        let mut var_guard659: f64 = *var_guard659_slot;
        let mut var_guard660: f64 = *var_guard660_slot;
        let mut var_guard661: f64 = *var_guard661_slot;
        let mut var_guard662: f64 = *var_guard662_slot;
        let mut var_guard663: f64 = *var_guard663_slot;
        let mut var_guard664: f64 = *var_guard664_slot;
        let mut var_guard665: f64 = *var_guard665_slot;
        let mut var_guard666: f64 = *var_guard666_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;

        let (assign33290_e41995, assign33290_e41995_d_n5, assign33290_e41995_d_n6, assign33290_e41995_d_n7, assign33290_e41995_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) && (var_guard655 == 0.0)) {
        let assign33290_e41991: f64 = (var_perfc * var_xerfc);
        let assign33290_e41992: f64 = (1.0 - assign33290_e41991);
        let assign33290_e41993: f64 = (1.0 / assign33290_e41992);
        (assign33290_e41993, (-((-(var_perfc * var_xerfc_dn5)) / (assign33290_e41992 * assign33290_e41992))), (-((-(var_perfc * var_xerfc_dn6)) / (assign33290_e41992 * assign33290_e41992))), (-((-(var_perfc * var_xerfc_dn7)) / (assign33290_e41992 * assign33290_e41992))), (-((-(var_perfc * var_xerfc_dn8)) / (assign33290_e41992 * assign33290_e41992))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign33290_e41995;
        var_terfc_dn5 = assign33290_e41995_d_n5;
        var_terfc_dn6 = assign33290_e41995_d_n6;
        var_terfc_dn7 = assign33290_e41995_d_n7;
        var_terfc_dn8 = assign33290_e41995_d_n8;

        let assign33300_e41997: f64 = (-var_ysq);
        let assign33300_e41999: f64 = (assign33300_e41997 + var_mtat);
        let assign33300_e42001: f64 = (-230.25850929940458);
        let assign33300_e42002: f64 = if assign33300_e41999 > assign33300_e42001 { 1.0 } else { 0.0 };
        var_guard656 = assign33300_e42002;

        let (assign33310_e42020, assign33310_e42020_d_n5, assign33310_e42020_d_n6, assign33310_e42020_d_n7, assign33310_e42020_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) && (var_guard656 != 0.0)) {
        let assign33310_e42015: f64 = (-var_ysq);
        let assign33310_e42017: f64 = (assign33310_e42015 + var_mtat);
        let assign33310_e42018: f64 = (assign33310_e42017).exp();
        (assign33310_e42018, (assign33310_e42018 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign33310_e42018 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign33310_e42018 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign33310_e42018 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33310_e42020;
        var_tmp_dn5 = assign33310_e42020_d_n5;
        var_tmp_dn6 = assign33310_e42020_d_n6;
        var_tmp_dn7 = assign33310_e42020_d_n7;
        var_tmp_dn8 = assign33310_e42020_d_n8;

        let (assign33320_e42069, assign33320_e42069_d_n5, assign33320_e42069_d_n6, assign33320_e42069_d_n7, assign33320_e42069_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) && (var_guard656 == 0.0)) {
        let assign33320_e42036: f64 = (-230.25850929940458);
        let assign33320_e42038: f64 = (-var_ysq);
        let assign33320_e42040: f64 = (assign33320_e42038 + var_mtat);
        let assign33320_e42041: f64 = (assign33320_e42036 - assign33320_e42040);
        let assign33320_e42045: f64 = (-230.25850929940458);
        let assign33320_e42047: f64 = (-var_ysq);
        let assign33320_e42049: f64 = (assign33320_e42047 + var_mtat);
        let assign33320_e42050: f64 = (assign33320_e42045 - assign33320_e42049);
        let assign33320_e42053: f64 = (-230.25850929940458);
        let assign33320_e42055: f64 = (-var_ysq);
        let assign33320_e42057: f64 = (assign33320_e42055 + var_mtat);
        let assign33320_e42058: f64 = (assign33320_e42053 - assign33320_e42057);
        let assign33320_e42060: f64 = (assign33320_e42058 * 0.3333333333333333);
        let assign33320_e42061: f64 = (1.0 + assign33320_e42060);
        let assign33320_e42062: f64 = (assign33320_e42050 * assign33320_e42061);
        let assign33320_e42063: f64 = (0.5 * assign33320_e42062);
        let assign33320_e42064: f64 = (1.0 + assign33320_e42063);
        let assign33320_e42065: f64 = (assign33320_e42041 * assign33320_e42064);
        let assign33320_e42066: f64 = (1.0 + assign33320_e42065);
        let assign33320_e42067: f64 = (1e-100 / assign33320_e42066);
        (assign33320_e42067, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign33320_e42064) + (assign33320_e42041 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign33320_e42061) + (assign33320_e42050 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign33320_e42066 * assign33320_e42066))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign33320_e42064) + (assign33320_e42041 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign33320_e42061) + (assign33320_e42050 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign33320_e42066 * assign33320_e42066))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign33320_e42064) + (assign33320_e42041 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign33320_e42061) + (assign33320_e42050 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign33320_e42066 * assign33320_e42066))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign33320_e42064) + (assign33320_e42041 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign33320_e42061) + (assign33320_e42050 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign33320_e42066 * assign33320_e42066))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33320_e42069;
        var_tmp_dn5 = assign33320_e42069_d_n5;
        var_tmp_dn6 = assign33320_e42069_d_n6;
        var_tmp_dn7 = assign33320_e42069_d_n7;
        var_tmp_dn8 = assign33320_e42069_d_n8;

        let (assign33330_e42099, assign33330_e42099_d_n5, assign33330_e42099_d_n6, assign33330_e42099_d_n7, assign33330_e42099_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33330_e42081: f64 = (0.29214664 * var_terfc);
        let assign33330_e42085: f64 = (var_terfc * var_terfc);
        let assign33330_e42086: f64 = (var_berfc * assign33330_e42085);
        let assign33330_e42087: f64 = (assign33330_e42081 + assign33330_e42086);
        let assign33330_e42091: f64 = (var_terfc * var_terfc);
        let assign33330_e42093: f64 = (assign33330_e42091 * var_terfc);
        let assign33330_e42094: f64 = (var_cerfc * assign33330_e42093);
        let assign33330_e42095: f64 = (assign33330_e42087 + assign33330_e42094);
        let assign33330_e42097: f64 = (assign33330_e42095 * var_tmp);
        (assign33330_e42097, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign33330_e42091 * var_terfc_dn5)))) * var_tmp) + (assign33330_e42095 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign33330_e42091 * var_terfc_dn6)))) * var_tmp) + (assign33330_e42095 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign33330_e42091 * var_terfc_dn7)))) * var_tmp) + (assign33330_e42095 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign33330_e42091 * var_terfc_dn8)))) * var_tmp) + (assign33330_e42095 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign33330_e42099;
        var_erfcpos_dn5 = assign33330_e42099_d_n5;
        var_erfcpos_dn6 = assign33330_e42099_d_n6;
        var_erfcpos_dn7 = assign33330_e42099_d_n7;
        var_erfcpos_dn8 = assign33330_e42099_d_n8;

        let assign33340_e42102: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard657 = assign33340_e42102;

        let (assign33350_e42116, assign33350_e42116_d_n5, assign33350_e42116_d_n6, assign33350_e42116_d_n7, assign33350_e42116_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) && (var_guard657 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign33350_e42116;
        var_erfctimesexpmtat_dn5 = assign33350_e42116_d_n5;
        var_erfctimesexpmtat_dn6 = assign33350_e42116_d_n6;
        var_erfctimesexpmtat_dn7 = assign33350_e42116_d_n7;
        var_erfctimesexpmtat_dn8 = assign33350_e42116_d_n8;

        let assign33360_e42119: f64 = (-230.25850929940458);
        let assign33360_e42120: f64 = if var_mtat > assign33360_e42119 { 1.0 } else { 0.0 };
        var_guard658 = assign33360_e42120;

        let (assign33370_e42138, assign33370_e42138_d_n5, assign33370_e42138_d_n6, assign33370_e42138_d_n7, assign33370_e42138_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) && (var_guard657 == 0.0)) && (var_guard658 != 0.0)) {
        let assign33370_e42136: f64 = (var_mtat).exp();
        (assign33370_e42136, (assign33370_e42136 * var_mtat_dn5), (assign33370_e42136 * var_mtat_dn6), (assign33370_e42136 * var_mtat_dn7), (assign33370_e42136 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33370_e42138;
        var_tmp_dn5 = assign33370_e42138_d_n5;
        var_tmp_dn6 = assign33370_e42138_d_n6;
        var_tmp_dn7 = assign33370_e42138_d_n7;
        var_tmp_dn8 = assign33370_e42138_d_n8;

        let (assign33380_e42181, assign33380_e42181_d_n5, assign33380_e42181_d_n6, assign33380_e42181_d_n7, assign33380_e42181_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) && (var_guard657 == 0.0)) && (var_guard658 == 0.0)) {
        let assign33380_e42157: f64 = (-230.25850929940458);
        let assign33380_e42159: f64 = (assign33380_e42157 - var_mtat);
        let assign33380_e42163: f64 = (-230.25850929940458);
        let assign33380_e42165: f64 = (assign33380_e42163 - var_mtat);
        let assign33380_e42168: f64 = (-230.25850929940458);
        let assign33380_e42170: f64 = (assign33380_e42168 - var_mtat);
        let assign33380_e42172: f64 = (assign33380_e42170 * 0.3333333333333333);
        let assign33380_e42173: f64 = (1.0 + assign33380_e42172);
        let assign33380_e42174: f64 = (assign33380_e42165 * assign33380_e42173);
        let assign33380_e42175: f64 = (0.5 * assign33380_e42174);
        let assign33380_e42176: f64 = (1.0 + assign33380_e42175);
        let assign33380_e42177: f64 = (assign33380_e42159 * assign33380_e42176);
        let assign33380_e42178: f64 = (1.0 + assign33380_e42177);
        let assign33380_e42179: f64 = (1e-100 / assign33380_e42178);
        (assign33380_e42179, (-((1e-100 * (((-var_mtat_dn5) * assign33380_e42176) + (assign33380_e42159 * (0.5 * (((-var_mtat_dn5) * assign33380_e42173) + (assign33380_e42165 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign33380_e42178 * assign33380_e42178))), (-((1e-100 * (((-var_mtat_dn6) * assign33380_e42176) + (assign33380_e42159 * (0.5 * (((-var_mtat_dn6) * assign33380_e42173) + (assign33380_e42165 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign33380_e42178 * assign33380_e42178))), (-((1e-100 * (((-var_mtat_dn7) * assign33380_e42176) + (assign33380_e42159 * (0.5 * (((-var_mtat_dn7) * assign33380_e42173) + (assign33380_e42165 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign33380_e42178 * assign33380_e42178))), (-((1e-100 * (((-var_mtat_dn8) * assign33380_e42176) + (assign33380_e42159 * (0.5 * (((-var_mtat_dn8) * assign33380_e42173) + (assign33380_e42165 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign33380_e42178 * assign33380_e42178))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33380_e42181;
        var_tmp_dn5 = assign33380_e42181_d_n5;
        var_tmp_dn6 = assign33380_e42181_d_n6;
        var_tmp_dn7 = assign33380_e42181_d_n7;
        var_tmp_dn8 = assign33380_e42181_d_n8;

        let (assign33390_e42200, assign33390_e42200_d_n5, assign33390_e42200_d_n6, assign33390_e42200_d_n7, assign33390_e42200_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) && (var_guard657 == 0.0)) {
        let assign33390_e42196: f64 = (2.0 * var_tmp);
        let assign33390_e42198: f64 = (assign33390_e42196 - var_erfcpos);
        (assign33390_e42198, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign33390_e42200;
        var_erfctimesexpmtat_dn5 = assign33390_e42200_d_n5;
        var_erfctimesexpmtat_dn6 = assign33390_e42200_d_n6;
        var_erfctimesexpmtat_dn7 = assign33390_e42200_d_n7;
        var_erfctimesexpmtat_dn8 = assign33390_e42200_d_n8;

        let (assign33400_e42220, assign33400_e42220_d_n5, assign33400_e42220_d_n6, assign33400_e42220_d_n7, assign33400_e42220_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33400_e42212: f64 = (1.772453850905516 * 0.5);
        let assign33400_e42215: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign33400_e42217: f64 = (assign33400_e42215 / var_ktat);
        let assign33400_e42218: f64 = (assign33400_e42212 * assign33400_e42217);
        (assign33400_e42218, (assign33400_e42212 * ((((var_atatbot_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign33400_e42215 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign33400_e42212 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign33400_e42215 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign33400_e42212 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign33400_e42215 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign33400_e42212 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign33400_e42215 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign33400_e42220;
        var_gammamax_dn5 = assign33400_e42220_d_n5;
        var_gammamax_dn6 = assign33400_e42220_d_n6;
        var_gammamax_dn7 = assign33400_e42220_d_n7;
        var_gammamax_dn8 = assign33400_e42220_d_n8;

        let (assign33410_e42238, assign33410_e42238_d_n5, assign33410_e42238_d_n6, assign33410_e42238_d_n7, assign33410_e42238_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard653 == 0.0)) {
        let assign33410_e42233: f64 = (var_asrh * var_gammamax);
        let assign33410_e42235: f64 = (assign33410_e42233 * var_wtat);
        let assign33410_e42236: f64 = (var_ctatbotd_i * assign33410_e42235);
        (assign33410_e42236, (var_ctatbotd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign33410_e42233 * var_wtat_dn5))), (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign33410_e42233 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign33410_e42233 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign33410_e42233 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign33410_e42238;
        var_itat_dn5 = assign33410_e42238_d_n5;
        var_itat_dn6 = assign33410_e42238_d_n6;
        var_itat_dn7 = assign33410_e42238_d_n7;
        var_itat_dn8 = assign33410_e42238_d_n8;

        let assign33420_e42241: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard659 = assign33420_e42241;

        let (assign33430_e42252, assign33430_e42252_d_n5, assign33430_e42252_d_n6, assign33430_e42252_d_n7, assign33430_e42252_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard659 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign33430_e42252;
        var_ibbt_dn5 = assign33430_e42252_d_n5;
        var_ibbt_dn6 = assign33430_e42252_d_n6;
        var_ibbt_dn7 = assign33430_e42252_d_n7;
        var_ibbt_dn8 = assign33430_e42252_d_n8;

        let assign33440_e42255: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard660 = assign33440_e42255;

        let (assign33450_e42274, assign33450_e42274_d_n5, assign33450_e42274_d_n6, assign33450_e42274_d_n7, assign33450_e42274_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard659 == 0.0)) && (var_guard660 != 0.0)) {
        let assign33450_e42269: f64 = (var_vbirbotd_i - var_vbbt);
        let assign33450_e42271: f64 = (assign33450_e42269 * var_vbirbotinv_d);
        let assign33450_e42272: f64 = (assign33450_e42271).sqrt();
        (assign33450_e42272, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33450_e42274;
        var_tmp_dn5 = assign33450_e42274_d_n5;
        var_tmp_dn6 = assign33450_e42274_d_n6;
        var_tmp_dn7 = assign33450_e42274_d_n7;
        var_tmp_dn8 = assign33450_e42274_d_n8;

        let (assign33460_e42295, assign33460_e42295_d_n5, assign33460_e42295_d_n6, assign33460_e42295_d_n7, assign33460_e42295_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard659 == 0.0)) && (var_guard660 == 0.0)) {
        let assign33460_e42289: f64 = (var_vbirbotd_i - var_vbbt);
        let assign33460_e42291: f64 = (assign33460_e42289 * var_vbirbotinv_d);
        let assign33460_e42293: f64 = (assign33460_e42291).powf(var_pbotd_i);
        (assign33460_e42293, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33460_e42295;
        var_tmp_dn5 = assign33460_e42295_d_n5;
        var_tmp_dn6 = assign33460_e42295_d_n6;
        var_tmp_dn7 = assign33460_e42295_d_n7;
        var_tmp_dn8 = assign33460_e42295_d_n8;

        let (assign33470_e42315, assign33470_e42315_d_n5, assign33470_e42315_d_n6, assign33470_e42315_d_n7, assign33470_e42315_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33470_e42308: f64 = (var_vbirbotd_i - var_vbbt);
        let assign33470_e42310: f64 = (assign33470_e42308 * var_wdepnulrinvbot_d);
        let assign33470_e42312: f64 = (assign33470_e42310 / var_tmp);
        let assign33470_e42313: f64 = (var_one_over_one_minus_pbot_d * assign33470_e42312);
        (assign33470_e42313, (var_one_over_one_minus_pbot_d * (-((assign33470_e42310 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign33470_e42310 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign33470_e42310 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign33470_e42310 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign33470_e42315;
        var_fmaxr_dn5 = assign33470_e42315_d_n5;
        var_fmaxr_dn6 = assign33470_e42315_d_n6;
        var_fmaxr_dn7 = assign33470_e42315_d_n7;
        var_fmaxr_dn8 = assign33470_e42315_d_n8;

        let assign33480_e42317: f64 = (-var_fbbtbot_d);
        let assign33480_e42319: f64 = (assign33480_e42317 / var_fmaxr);
        let assign33480_e42320: f64 = (assign33480_e42319).abs();
        let assign33480_e42322: f64 = if assign33480_e42320 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard661 = assign33480_e42322;

        let (assign33490_e42340, assign33490_e42340_d_n5, assign33490_e42340_d_n6, assign33490_e42340_d_n7, assign33490_e42340_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard659 == 0.0)) && (var_guard661 != 0.0)) {
        let assign33490_e42335: f64 = (-var_fbbtbot_d);
        let assign33490_e42337: f64 = (assign33490_e42335 / var_fmaxr);
        let assign33490_e42338: f64 = (assign33490_e42337).exp();
        (assign33490_e42338, (assign33490_e42338 * (-((assign33490_e42335 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign33490_e42338 * (-((assign33490_e42335 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign33490_e42338 * (-((assign33490_e42335 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign33490_e42338 * (-((assign33490_e42335 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33490_e42340;
        var_tmp_dn5 = assign33490_e42340_d_n5;
        var_tmp_dn6 = assign33490_e42340_d_n6;
        var_tmp_dn7 = assign33490_e42340_d_n7;
        var_tmp_dn8 = assign33490_e42340_d_n8;

        let assign33500_e42342: f64 = (-var_fbbtbot_d);
        let assign33500_e42344: f64 = (assign33500_e42342 / var_fmaxr);
        let assign33500_e42346: f64 = if assign33500_e42344 < 0.0 { 1.0 } else { 0.0 };
        var_guard662 = assign33500_e42346;

        let (assign33510_e42397, assign33510_e42397_d_n5, assign33510_e42397_d_n6, assign33510_e42397_d_n7, assign33510_e42397_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard659 == 0.0)) && (var_guard661 == 0.0)) && (var_guard662 != 0.0)) {
        let assign33510_e42364: f64 = (-230.25850929940458);
        let assign33510_e42366: f64 = (-var_fbbtbot_d);
        let assign33510_e42368: f64 = (assign33510_e42366 / var_fmaxr);
        let assign33510_e42369: f64 = (assign33510_e42364 - assign33510_e42368);
        let assign33510_e42373: f64 = (-230.25850929940458);
        let assign33510_e42375: f64 = (-var_fbbtbot_d);
        let assign33510_e42377: f64 = (assign33510_e42375 / var_fmaxr);
        let assign33510_e42378: f64 = (assign33510_e42373 - assign33510_e42377);
        let assign33510_e42381: f64 = (-230.25850929940458);
        let assign33510_e42383: f64 = (-var_fbbtbot_d);
        let assign33510_e42385: f64 = (assign33510_e42383 / var_fmaxr);
        let assign33510_e42386: f64 = (assign33510_e42381 - assign33510_e42385);
        let assign33510_e42388: f64 = (assign33510_e42386 * 0.3333333333333333);
        let assign33510_e42389: f64 = (1.0 + assign33510_e42388);
        let assign33510_e42390: f64 = (assign33510_e42378 * assign33510_e42389);
        let assign33510_e42391: f64 = (0.5 * assign33510_e42390);
        let assign33510_e42392: f64 = (1.0 + assign33510_e42391);
        let assign33510_e42393: f64 = (assign33510_e42369 * assign33510_e42392);
        let assign33510_e42394: f64 = (1.0 + assign33510_e42393);
        let assign33510_e42395: f64 = (1e-100 / assign33510_e42394);
        (assign33510_e42395, (-((1e-100 * (((-(-((assign33510_e42366 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign33510_e42392) + (assign33510_e42369 * (0.5 * (((-(-((assign33510_e42375 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign33510_e42389) + (assign33510_e42378 * ((-(-((assign33510_e42383 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33510_e42394 * assign33510_e42394))), (-((1e-100 * (((-(-((assign33510_e42366 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign33510_e42392) + (assign33510_e42369 * (0.5 * (((-(-((assign33510_e42375 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign33510_e42389) + (assign33510_e42378 * ((-(-((assign33510_e42383 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33510_e42394 * assign33510_e42394))), (-((1e-100 * (((-(-((assign33510_e42366 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign33510_e42392) + (assign33510_e42369 * (0.5 * (((-(-((assign33510_e42375 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign33510_e42389) + (assign33510_e42378 * ((-(-((assign33510_e42383 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33510_e42394 * assign33510_e42394))), (-((1e-100 * (((-(-((assign33510_e42366 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign33510_e42392) + (assign33510_e42369 * (0.5 * (((-(-((assign33510_e42375 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign33510_e42389) + (assign33510_e42378 * ((-(-((assign33510_e42383 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33510_e42394 * assign33510_e42394))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33510_e42397;
        var_tmp_dn5 = assign33510_e42397_d_n5;
        var_tmp_dn6 = assign33510_e42397_d_n6;
        var_tmp_dn7 = assign33510_e42397_d_n7;
        var_tmp_dn8 = assign33510_e42397_d_n8;

        let (assign33520_e42446, assign33520_e42446_d_n5, assign33520_e42446_d_n6, assign33520_e42446_d_n7, assign33520_e42446_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard659 == 0.0)) && (var_guard661 == 0.0)) && (var_guard662 == 0.0)) {
        let assign33520_e42416: f64 = (-var_fbbtbot_d);
        let assign33520_e42418: f64 = (assign33520_e42416 / var_fmaxr);
        let assign33520_e42420: f64 = (assign33520_e42418 - 230.25850929940458);
        let assign33520_e42424: f64 = (-var_fbbtbot_d);
        let assign33520_e42426: f64 = (assign33520_e42424 / var_fmaxr);
        let assign33520_e42428: f64 = (assign33520_e42426 - 230.25850929940458);
        let assign33520_e42431: f64 = (-var_fbbtbot_d);
        let assign33520_e42433: f64 = (assign33520_e42431 / var_fmaxr);
        let assign33520_e42435: f64 = (assign33520_e42433 - 230.25850929940458);
        let assign33520_e42437: f64 = (assign33520_e42435 * 0.3333333333333333);
        let assign33520_e42438: f64 = (1.0 + assign33520_e42437);
        let assign33520_e42439: f64 = (assign33520_e42428 * assign33520_e42438);
        let assign33520_e42440: f64 = (0.5 * assign33520_e42439);
        let assign33520_e42441: f64 = (1.0 + assign33520_e42440);
        let assign33520_e42442: f64 = (assign33520_e42420 * assign33520_e42441);
        let assign33520_e42443: f64 = (1.0 + assign33520_e42442);
        let assign33520_e42444: f64 = (1e100 * assign33520_e42443);
        (assign33520_e42444, (1e100 * (((-((assign33520_e42416 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign33520_e42441) + (assign33520_e42420 * (0.5 * (((-((assign33520_e42424 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign33520_e42438) + (assign33520_e42428 * ((-((assign33520_e42431 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33520_e42416 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign33520_e42441) + (assign33520_e42420 * (0.5 * (((-((assign33520_e42424 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign33520_e42438) + (assign33520_e42428 * ((-((assign33520_e42431 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33520_e42416 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign33520_e42441) + (assign33520_e42420 * (0.5 * (((-((assign33520_e42424 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign33520_e42438) + (assign33520_e42428 * ((-((assign33520_e42431 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33520_e42416 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign33520_e42441) + (assign33520_e42420 * (0.5 * (((-((assign33520_e42424 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign33520_e42438) + (assign33520_e42428 * ((-((assign33520_e42431 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33520_e42446;
        var_tmp_dn5 = assign33520_e42446_d_n5;
        var_tmp_dn6 = assign33520_e42446_d_n6;
        var_tmp_dn7 = assign33520_e42446_d_n7;
        var_tmp_dn8 = assign33520_e42446_d_n8;

        let (assign33530_e42466, assign33530_e42466_d_n5, assign33530_e42466_d_n6, assign33530_e42466_d_n7, assign33530_e42466_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33530_e42459: f64 = (var_v3 * var_fmaxr);
        let assign33530_e42461: f64 = (assign33530_e42459 * var_fmaxr);
        let assign33530_e42463: f64 = (assign33530_e42461 * var_tmp);
        let assign33530_e42464: f64 = (var_cbbtbotd_i * assign33530_e42463);
        (assign33530_e42464, (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign33530_e42459 * var_fmaxr_dn5)) * var_tmp) + (assign33530_e42461 * var_tmp_dn5))), (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign33530_e42459 * var_fmaxr_dn6)) * var_tmp) + (assign33530_e42461 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign33530_e42459 * var_fmaxr_dn7)) * var_tmp) + (assign33530_e42461 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign33530_e42459 * var_fmaxr_dn8)) * var_tmp) + (assign33530_e42461 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign33530_e42466;
        var_ibbt_dn5 = assign33530_e42466_d_n5;
        var_ibbt_dn6 = assign33530_e42466_d_n6;
        var_ibbt_dn7 = assign33530_e42466_d_n7;
        var_ibbt_dn8 = assign33530_e42466_d_n8;

        let assign33540_e42469: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard663 = assign33540_e42469;

        let (assign33550_e42480, assign33550_e42480_d_n5, assign33550_e42480_d_n6, assign33550_e42480_d_n7, assign33550_e42480_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard663 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign33550_e42480;
        var_fbreakdown_dn5 = assign33550_e42480_d_n5;
        var_fbreakdown_dn6 = assign33550_e42480_d_n6;
        var_fbreakdown_dn7 = assign33550_e42480_d_n7;
        var_fbreakdown_dn8 = assign33550_e42480_d_n8;

        let assign33560_e42483: f64 = (-var_alphaav);
        let assign33560_e42485: f64 = (assign33560_e42483 * var_vbrbotd_i);
        let assign33560_e42486: f64 = if var_vav > assign33560_e42485 { 1.0 } else { 0.0 };
        var_guard664 = assign33560_e42486;

        let assign33570_e42489: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard665 = assign33570_e42489;

        let (assign33580_e42519, assign33580_e42519_d_n5, assign33580_e42519_d_n6, assign33580_e42519_d_n7, assign33580_e42519_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard663 == 0.0)) && (var_guard664 != 0.0)) && (var_guard665 != 0.0)) {
        let assign33580_e42505: f64 = (var_vav * var_vbrinvbot_d);
        let assign33580_e42508: f64 = (var_vav * var_vbrinvbot_d);
        let assign33580_e42509: f64 = (assign33580_e42505 * assign33580_e42508);
        let assign33580_e42512: f64 = (var_vav * var_vbrinvbot_d);
        let assign33580_e42513: f64 = (assign33580_e42509 * assign33580_e42512);
        let assign33580_e42516: f64 = (var_vav * var_vbrinvbot_d);
        let assign33580_e42517: f64 = (assign33580_e42513 * assign33580_e42516);
        (assign33580_e42517, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33580_e42519;
        var_tmp_dn5 = assign33580_e42519_d_n5;
        var_tmp_dn6 = assign33580_e42519_d_n6;
        var_tmp_dn7 = assign33580_e42519_d_n7;
        var_tmp_dn8 = assign33580_e42519_d_n8;

        let (assign33590_e42541, assign33590_e42541_d_n5, assign33590_e42541_d_n6, assign33590_e42541_d_n7, assign33590_e42541_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard663 == 0.0)) && (var_guard664 != 0.0)) && (var_guard665 == 0.0)) {
        let assign33590_e42536: f64 = (var_vav * var_vbrinvbot_d);
        let assign33590_e42537: f64 = (assign33590_e42536).abs();
        let assign33590_e42539: f64 = (assign33590_e42537).powf(var_pbrbotd_i);
        (assign33590_e42539, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33590_e42541;
        var_tmp_dn5 = assign33590_e42541_d_n5;
        var_tmp_dn6 = assign33590_e42541_d_n6;
        var_tmp_dn7 = assign33590_e42541_d_n7;
        var_tmp_dn8 = assign33590_e42541_d_n8;

        let (assign33600_e42559, assign33600_e42559_d_n5, assign33600_e42559_d_n6, assign33600_e42559_d_n7, assign33600_e42559_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard663 == 0.0)) && (var_guard664 != 0.0)) {
        let assign33600_e42556: f64 = (1.0 - var_tmp);
        let assign33600_e42557: f64 = (1.0 / assign33600_e42556);
        (assign33600_e42557, (-((-var_tmp_dn5) / (assign33600_e42556 * assign33600_e42556))), (-((-var_tmp_dn6) / (assign33600_e42556 * assign33600_e42556))), (-((-var_tmp_dn7) / (assign33600_e42556 * assign33600_e42556))), (-((-var_tmp_dn8) / (assign33600_e42556 * assign33600_e42556))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign33600_e42559;
        var_fbreakdown_dn5 = assign33600_e42559_d_n5;
        var_fbreakdown_dn6 = assign33600_e42559_d_n6;
        var_fbreakdown_dn7 = assign33600_e42559_d_n7;
        var_fbreakdown_dn8 = assign33600_e42559_d_n8;

        let (assign33610_e42582, assign33610_e42582_d_n5, assign33610_e42582_d_n6, assign33610_e42582_d_n7, assign33610_e42582_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) && (var_guard663 == 0.0)) && (var_guard664 == 0.0)) {
        let assign33610_e42576: f64 = (var_alphaav * var_vbrbotd_i);
        let assign33610_e42577: f64 = (var_vav + assign33610_e42576);
        let assign33610_e42579: f64 = (assign33610_e42577 * var_slopebot_d);
        let assign33610_e42580: f64 = (var_fstopbot_d + assign33610_e42579);
        (assign33610_e42580, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign33610_e42582;
        var_fbreakdown_dn5 = assign33610_e42582_d_n5;
        var_fbreakdown_dn6 = assign33610_e42582_d_n6;
        var_fbreakdown_dn7 = assign33610_e42582_d_n7;
        var_fbreakdown_dn8 = assign33610_e42582_d_n8;

        let (assign33620_e42601, assign33620_e42601_d_n5, assign33620_e42601_d_n6, assign33620_e42601_d_n7, assign33620_e42601_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard649 == 0.0)) {
        let assign33620_e42592: f64 = (var_id__blk213 + var_isrh);
        let assign33620_e42594: f64 = (assign33620_e42592 + var_itat);
        let assign33620_e42596: f64 = (assign33620_e42594 + var_ibbt);
        let assign33620_e42597: f64 = (p.p29 * assign33620_e42596);
        let assign33620_e42599: f64 = (assign33620_e42597 * var_fbreakdown);
        (assign33620_e42599, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign33620_e42597 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign33620_e42597 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign33620_e42597 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign33620_e42597 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign33620_e42601;
        var_ijunbot_dn5 = assign33620_e42601_d_n5;
        var_ijunbot_dn6 = assign33620_e42601_d_n6;
        var_ijunbot_dn7 = assign33620_e42601_d_n7;
        var_ijunbot_dn8 = assign33620_e42601_d_n8;

        let assign33630_e42604: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard666 = assign33630_e42604;

        let (assign33640_e42612, assign33640_e42612_d_n5, assign33640_e42612_d_n6, assign33640_e42612_d_n7, assign33640_e42612_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign33640_e42612;
        var_ijunsti_dn5 = assign33640_e42612_d_n5;
        var_ijunsti_dn6 = assign33640_e42612_d_n6;
        var_ijunsti_dn7 = assign33640_e42612_d_n7;
        var_ijunsti_dn8 = assign33640_e42612_d_n8;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard656_slot = var_guard656;
        *var_guard657_slot = var_guard657;
        *var_guard658_slot = var_guard658;
        *var_guard659_slot = var_guard659;
        *var_guard660_slot = var_guard660;
        *var_guard661_slot = var_guard661;
        *var_guard662_slot = var_guard662;
        *var_guard663_slot = var_guard663;
        *var_guard664_slot = var_guard664;
        *var_guard665_slot = var_guard665;
        *var_guard666_slot = var_guard666;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
    }

    pub(super) fn stamp_transient_block_69(
        var_atatsti_d: f64,
        var_btatpartsti_d: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_ftdsti_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard666: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_perfc: f64,
        var_pstid_i: f64,
        var_two_psistar: f64,
        var_vbirstiinv_d: f64,
        var_vbisti_d: f64,
        var_vjsrh: f64,
        var_wdepnulrsti_d: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard667_slot: &mut f64,
        var_guard668_slot: &mut f64,
        var_guard669_slot: &mut f64,
        var_guard670_slot: &mut f64,
        var_guard671_slot: &mut f64,
        var_guard672_slot: &mut f64,
        var_guard673_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard667: f64 = *var_guard667_slot;
        let mut var_guard668: f64 = *var_guard668_slot;
        let mut var_guard669: f64 = *var_guard669_slot;
        let mut var_guard670: f64 = *var_guard670_slot;
        let mut var_guard671: f64 = *var_guard671_slot;
        let mut var_guard672: f64 = *var_guard672_slot;
        let mut var_guard673: f64 = *var_guard673_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign33650_e42623,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) {
        let assign33650_e42621: f64 = (var_idsatsti_d * var_idmult);
        (assign33650_e42621,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign33650_e42623;

        let assign33660_e42630: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard667 = assign33660_e42630;

        let (assign33670_e42641, assign33670_e42641_d_n5, assign33670_e42641_d_n6, assign33670_e42641_d_n7, assign33670_e42641_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign33670_e42641;
        var_isrh_dn5 = assign33670_e42641_d_n5;
        var_isrh_dn6 = assign33670_e42641_d_n6;
        var_isrh_dn7 = assign33670_e42641_d_n7;
        var_isrh_dn8 = assign33670_e42641_d_n8;

        let (assign33680_e42655,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) {
        let assign33680_e42653: f64 = (var_vbisti_d - var_vjsrh);
        (assign33680_e42653,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign33680_e42655;

        let (assign33690_e42674,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) {
        let assign33690_e42669: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign33690_e42670: f64 = (1.0 - assign33690_e42669);
        let assign33690_e42671: f64 = (assign33690_e42670).sqrt();
        let assign33690_e42672: f64 = (1.0 - assign33690_e42671);
        (assign33690_e42672,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign33690_e42674;

        let assign33700_e42677: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard668 = assign33700_e42677;

        let (assign33710_e42691,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) && (var_guard668 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign33710_e42691;

        let (assign33720_e42723,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) && (var_guard668 == 0.0)) {
        let assign33720_e42706: f64 = (var_wsrhstep * var_wsrhstep);
        let assign33720_e42708: f64 = (var_wsrhstep).ln();
        let assign33720_e42709: f64 = (assign33720_e42706 * assign33720_e42708);
        let assign33720_e42712: f64 = (1.0 - var_wsrhstep);
        let assign33720_e42713: f64 = (assign33720_e42709 / assign33720_e42712);
        let assign33720_e42715: f64 = (assign33720_e42713 + var_wsrhstep);
        let assign33720_e42719: f64 = (2.0 * var_pstid_i);
        let assign33720_e42720: f64 = (1.0 - assign33720_e42719);
        let assign33720_e42721: f64 = (assign33720_e42715 * assign33720_e42720);
        (assign33720_e42721,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign33720_e42723;

        let (assign33730_e42737,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) {
        let assign33730_e42735: f64 = (var_wsrhstep + var_dwsrh);
        (assign33730_e42735,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign33730_e42737;

        let assign33740_e42740: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard669 = assign33740_e42740;

        let (assign33750_e42757, assign33750_e42757_d_n5, assign33750_e42757_d_n6, assign33750_e42757_d_n7, assign33750_e42757_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) && (var_guard669 != 0.0)) {
        let assign33750_e42754: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign33750_e42755: f64 = (assign33750_e42754).sqrt();
        (assign33750_e42755, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33750_e42757;
        var_tmp_dn5 = assign33750_e42757_d_n5;
        var_tmp_dn6 = assign33750_e42757_d_n6;
        var_tmp_dn7 = assign33750_e42757_d_n7;
        var_tmp_dn8 = assign33750_e42757_d_n8;

        let (assign33760_e42776, assign33760_e42776_d_n5, assign33760_e42776_d_n6, assign33760_e42776_d_n7, assign33760_e42776_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) && (var_guard669 == 0.0)) {
        let assign33760_e42772: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign33760_e42774: f64 = (assign33760_e42772).powf(var_pstid_i);
        (assign33760_e42774, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33760_e42776;
        var_tmp_dn5 = assign33760_e42776_d_n5;
        var_tmp_dn6 = assign33760_e42776_d_n6;
        var_tmp_dn7 = assign33760_e42776_d_n7;
        var_tmp_dn8 = assign33760_e42776_d_n8;

        let (assign33770_e42790, assign33770_e42790_d_n5, assign33770_e42790_d_n6, assign33770_e42790_d_n7, assign33770_e42790_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) {
        let assign33770_e42788: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign33770_e42788, (var_wdepnulrsti_d * var_tmp_dn5), (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign33770_e42790;
        var_wdep_dn5 = assign33770_e42790_d_n5;
        var_wdep_dn6 = assign33770_e42790_d_n6;
        var_wdep_dn7 = assign33770_e42790_d_n7;
        var_wdep_dn8 = assign33770_e42790_d_n8;

        let (assign33780_e42808, assign33780_e42808_d_n5, assign33780_e42808_d_n6, assign33780_e42808_d_n7, assign33780_e42808_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) {
        let assign33780_e42803: f64 = (var_zinv - 1.0);
        let assign33780_e42805: f64 = (assign33780_e42803 * var_wdep);
        let assign33780_e42806: f64 = (var_ftdsti_d * assign33780_e42805);
        (assign33780_e42806, (var_ftdsti_d * (assign33780_e42803 * var_wdep_dn5)), (var_ftdsti_d * (assign33780_e42803 * var_wdep_dn6)), (var_ftdsti_d * (assign33780_e42803 * var_wdep_dn7)), (var_ftdsti_d * (assign33780_e42803 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign33780_e42808;
        var_asrh_dn5 = assign33780_e42808_d_n5;
        var_asrh_dn6 = assign33780_e42808_d_n6;
        var_asrh_dn7 = assign33780_e42808_d_n7;
        var_asrh_dn8 = assign33780_e42808_d_n8;

        let (assign33790_e42824, assign33790_e42824_d_n5, assign33790_e42824_d_n6, assign33790_e42824_d_n7, assign33790_e42824_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) {
        let assign33790_e42821: f64 = (var_asrh * var_wsrh);
        let assign33790_e42822: f64 = (var_csrhstid_i * assign33790_e42821);
        (assign33790_e42822, (var_csrhstid_i * (var_asrh_dn5 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign33790_e42824;
        var_isrh_dn5 = assign33790_e42824_d_n5;
        var_isrh_dn6 = assign33790_e42824_d_n6;
        var_isrh_dn7 = assign33790_e42824_d_n7;
        var_isrh_dn8 = assign33790_e42824_d_n8;

        let assign33800_e42827: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard670 = assign33800_e42827;

        let (assign33810_e42838, assign33810_e42838_d_n5, assign33810_e42838_d_n6, assign33810_e42838_d_n7, assign33810_e42838_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign33810_e42838;
        var_itat_dn5 = assign33810_e42838_d_n5;
        var_itat_dn6 = assign33810_e42838_d_n6;
        var_itat_dn7 = assign33810_e42838_d_n7;
        var_itat_dn8 = assign33810_e42838_d_n8;

        let (assign33820_e42856, assign33820_e42856_d_n5, assign33820_e42856_d_n6, assign33820_e42856_d_n7, assign33820_e42856_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33820_e42851: f64 = (var_wdep * var_one_minus_psti_d);
        let assign33820_e42853: f64 = (assign33820_e42851 / var_vbi_minus_vjsrh);
        let assign33820_e42854: f64 = (var_btatpartsti_d * assign33820_e42853);
        (assign33820_e42854, (var_btatpartsti_d * ((var_wdep_dn5 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign33820_e42856;
        var_btat_dn5 = assign33820_e42856_d_n5;
        var_btat_dn6 = assign33820_e42856_d_n6;
        var_btat_dn7 = assign33820_e42856_d_n7;
        var_btat_dn8 = assign33820_e42856_d_n8;

        let (assign33830_e42872, assign33830_e42872_d_n5, assign33830_e42872_d_n6, assign33830_e42872_d_n7, assign33830_e42872_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33830_e42868: f64 = (0.666666666666667 * var_atatsti_d);
        let assign33830_e42870: f64 = (assign33830_e42868 / var_btat);
        (assign33830_e42870, (-((assign33830_e42868 * var_btat_dn5) / (var_btat * var_btat))), (-((assign33830_e42868 * var_btat_dn6) / (var_btat * var_btat))), (-((assign33830_e42868 * var_btat_dn7) / (var_btat * var_btat))), (-((assign33830_e42868 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign33830_e42872;
        var_twoatatoverthreebtat_dn5 = assign33830_e42872_d_n5;
        var_twoatatoverthreebtat_dn6 = assign33830_e42872_d_n6;
        var_twoatatoverthreebtat_dn7 = assign33830_e42872_d_n7;
        var_twoatatoverthreebtat_dn8 = assign33830_e42872_d_n8;

        let (assign33840_e42886, assign33840_e42886_d_n5, assign33840_e42886_d_n6, assign33840_e42886_d_n7, assign33840_e42886_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33840_e42884: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign33840_e42884, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign33840_e42886;
        var_umaxbeforelimiting_dn5 = assign33840_e42886_d_n5;
        var_umaxbeforelimiting_dn6 = assign33840_e42886_d_n6;
        var_umaxbeforelimiting_dn7 = assign33840_e42886_d_n7;
        var_umaxbeforelimiting_dn8 = assign33840_e42886_d_n8;

        let (assign33850_e42907, assign33850_e42907_d_n5, assign33850_e42907_d_n6, assign33850_e42907_d_n7, assign33850_e42907_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33850_e42898: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign33850_e42901: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign33850_e42903: f64 = (assign33850_e42901 + 1.0);
        let assign33850_e42904: f64 = (assign33850_e42898 / assign33850_e42903);
        let assign33850_e42905: f64 = (assign33850_e42904).sqrt();
        (assign33850_e42905, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign33850_e42903) - (assign33850_e42898 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign33850_e42903 * assign33850_e42903)) / (2.0 * assign33850_e42905)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign33850_e42903) - (assign33850_e42898 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign33850_e42903 * assign33850_e42903)) / (2.0 * assign33850_e42905)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign33850_e42903) - (assign33850_e42898 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign33850_e42903 * assign33850_e42903)) / (2.0 * assign33850_e42905)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign33850_e42903) - (assign33850_e42898 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign33850_e42903 * assign33850_e42903)) / (2.0 * assign33850_e42905)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign33850_e42907;
        var_umax_dn5 = assign33850_e42907_d_n5;
        var_umax_dn6 = assign33850_e42907_d_n6;
        var_umax_dn7 = assign33850_e42907_d_n7;
        var_umax_dn8 = assign33850_e42907_d_n8;

        let (assign33860_e42920, assign33860_e42920_d_n5, assign33860_e42920_d_n6, assign33860_e42920_d_n7, assign33860_e42920_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33860_e42918: f64 = (var_umax).sqrt();
        (assign33860_e42918, (var_umax_dn5 / (2.0 * assign33860_e42918)), (var_umax_dn6 / (2.0 * assign33860_e42918)), (var_umax_dn7 / (2.0 * assign33860_e42918)), (var_umax_dn8 / (2.0 * assign33860_e42918)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign33860_e42920;
        var_sqrtumax_dn5 = assign33860_e42920_d_n5;
        var_sqrtumax_dn6 = assign33860_e42920_d_n6;
        var_sqrtumax_dn7 = assign33860_e42920_d_n7;
        var_sqrtumax_dn8 = assign33860_e42920_d_n8;

        let (assign33870_e42934, assign33870_e42934_d_n5, assign33870_e42934_d_n6, assign33870_e42934_d_n7, assign33870_e42934_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33870_e42932: f64 = (var_umax * var_sqrtumax);
        (assign33870_e42932, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign33870_e42934;
        var_umaxpoweronepointfive_dn5 = assign33870_e42934_d_n5;
        var_umaxpoweronepointfive_dn6 = assign33870_e42934_d_n6;
        var_umaxpoweronepointfive_dn7 = assign33870_e42934_d_n7;
        var_umaxpoweronepointfive_dn8 = assign33870_e42934_d_n8;

        let assign33880_e42936: f64 = (-var_pstid_i);
        let assign33880_e42938: f64 = (assign33880_e42936 * var_one_over_one_minus_psti_d);
        let assign33880_e42940: f64 = (-1.0);
        let assign33880_e42941: f64 = if assign33880_e42938 == assign33880_e42940 { 1.0 } else { 0.0 };
        var_guard671 = assign33880_e42941;

        let (assign33890_e42961, assign33890_e42961_d_n5, assign33890_e42961_d_n6, assign33890_e42961_d_n7, assign33890_e42961_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) && (var_guard671 != 0.0)) {
        let assign33890_e42957: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33890_e42958: f64 = (1.0 + assign33890_e42957);
        let assign33890_e42959: f64 = (1.0 / assign33890_e42958);
        (assign33890_e42959, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign33890_e42958 * assign33890_e42958))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign33890_e42958 * assign33890_e42958))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign33890_e42958 * assign33890_e42958))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign33890_e42958 * assign33890_e42958))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign33890_e42961;
        var_wgamma_dn5 = assign33890_e42961_d_n5;
        var_wgamma_dn6 = assign33890_e42961_d_n6;
        var_wgamma_dn7 = assign33890_e42961_d_n7;
        var_wgamma_dn8 = assign33890_e42961_d_n8;

        let (assign33900_e42985, assign33900_e42985_d_n5, assign33900_e42985_d_n6, assign33900_e42985_d_n7, assign33900_e42985_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) && (var_guard671 == 0.0)) {
        let assign33900_e42977: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33900_e42978: f64 = (1.0 + assign33900_e42977);
        let assign33900_e42980: f64 = (-var_pstid_i);
        let assign33900_e42982: f64 = (assign33900_e42980 * var_one_over_one_minus_psti_d);
        let assign33900_e42983: f64 = (assign33900_e42978).powf(assign33900_e42982);
        (assign33900_e42983, if 0.0 == 0.0 && ((assign33900_e42982) as f64).is_finite() && ((assign33900_e42982) as f64).fract() == 0.0 { if assign33900_e42982 == 0.0 { 0.0 } else { (assign33900_e42982 * ((assign33900_e42978).powf(assign33900_e42982 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign33900_e42983 * (assign33900_e42982 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign33900_e42978))) }, if 0.0 == 0.0 && ((assign33900_e42982) as f64).is_finite() && ((assign33900_e42982) as f64).fract() == 0.0 { if assign33900_e42982 == 0.0 { 0.0 } else { (assign33900_e42982 * ((assign33900_e42978).powf(assign33900_e42982 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign33900_e42983 * (assign33900_e42982 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign33900_e42978))) }, if 0.0 == 0.0 && ((assign33900_e42982) as f64).is_finite() && ((assign33900_e42982) as f64).fract() == 0.0 { if assign33900_e42982 == 0.0 { 0.0 } else { (assign33900_e42982 * ((assign33900_e42978).powf(assign33900_e42982 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign33900_e42983 * (assign33900_e42982 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign33900_e42978))) }, if 0.0 == 0.0 && ((assign33900_e42982) as f64).is_finite() && ((assign33900_e42982) as f64).fract() == 0.0 { if assign33900_e42982 == 0.0 { 0.0 } else { (assign33900_e42982 * ((assign33900_e42978).powf(assign33900_e42982 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign33900_e42983 * (assign33900_e42982 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign33900_e42978))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign33900_e42985;
        var_wgamma_dn5 = assign33900_e42985_d_n5;
        var_wgamma_dn6 = assign33900_e42985_d_n6;
        var_wgamma_dn7 = assign33900_e42985_d_n7;
        var_wgamma_dn8 = assign33900_e42985_d_n8;

        let (assign33910_e43003, assign33910_e43003_d_n5, assign33910_e43003_d_n6, assign33910_e43003_d_n7, assign33910_e43003_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33910_e42997: f64 = (var_wsrh * var_wgamma);
        let assign33910_e43000: f64 = (var_wsrh + var_wgamma);
        let assign33910_e43001: f64 = (assign33910_e42997 / assign33910_e43000);
        (assign33910_e43001, ((((var_wsrh * var_wgamma_dn5) * assign33910_e43000) - (assign33910_e42997 * var_wgamma_dn5)) / (assign33910_e43000 * assign33910_e43000)), ((((var_wsrh * var_wgamma_dn6) * assign33910_e43000) - (assign33910_e42997 * var_wgamma_dn6)) / (assign33910_e43000 * assign33910_e43000)), ((((var_wsrh * var_wgamma_dn7) * assign33910_e43000) - (assign33910_e42997 * var_wgamma_dn7)) / (assign33910_e43000 * assign33910_e43000)), ((((var_wsrh * var_wgamma_dn8) * assign33910_e43000) - (assign33910_e42997 * var_wgamma_dn8)) / (assign33910_e43000 * assign33910_e43000)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign33910_e43003;
        var_wtat_dn5 = assign33910_e43003_d_n5;
        var_wtat_dn6 = assign33910_e43003_d_n6;
        var_wtat_dn7 = assign33910_e43003_d_n7;
        var_wtat_dn8 = assign33910_e43003_d_n8;

        let (assign33920_e43020, assign33920_e43020_d_n5, assign33920_e43020_d_n6, assign33920_e43020_d_n7, assign33920_e43020_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33920_e43016: f64 = (var_btat / var_sqrtumax);
        let assign33920_e43017: f64 = (0.375 * assign33920_e43016);
        let assign33920_e43018: f64 = (assign33920_e43017).sqrt();
        (assign33920_e43018, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33920_e43018)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33920_e43018)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33920_e43018)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33920_e43018)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign33920_e43020;
        var_ktat_dn5 = assign33920_e43020_d_n5;
        var_ktat_dn6 = assign33920_e43020_d_n6;
        var_ktat_dn7 = assign33920_e43020_d_n7;
        var_ktat_dn8 = assign33920_e43020_d_n8;

        let (assign33930_e43038, assign33930_e43038_d_n5, assign33930_e43038_d_n6, assign33930_e43038_d_n7, assign33930_e43038_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33930_e43033: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign33930_e43034: f64 = (2.0 * assign33930_e43033);
        let assign33930_e43036: f64 = (assign33930_e43034 - var_umax);
        (assign33930_e43036, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign33930_e43038;
        var_ltat_dn5 = assign33930_e43038_d_n5;
        var_ltat_dn6 = assign33930_e43038_d_n6;
        var_ltat_dn7 = assign33930_e43038_d_n7;
        var_ltat_dn8 = assign33930_e43038_d_n8;

        let (assign33940_e43064, assign33940_e43064_d_n5, assign33940_e43064_d_n6, assign33940_e43064_d_n7, assign33940_e43064_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33940_e43050: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign33940_e43052: f64 = (assign33940_e43050 * var_sqrtumax);
        let assign33940_e43055: f64 = (var_atatsti_d * var_umax);
        let assign33940_e43056: f64 = (assign33940_e43052 - assign33940_e43055);
        let assign33940_e43060: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33940_e43061: f64 = (0.5 * assign33940_e43060);
        let assign33940_e43062: f64 = (assign33940_e43056 + assign33940_e43061);
        (assign33940_e43062, (((((var_atatsti_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign33940_e43050 * var_sqrtumax_dn5)) - (var_atatsti_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign33940_e43050 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign33940_e43050 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign33940_e43050 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign33940_e43064;
        var_mtat_dn5 = assign33940_e43064_d_n5;
        var_mtat_dn6 = assign33940_e43064_d_n6;
        var_mtat_dn7 = assign33940_e43064_d_n7;
        var_mtat_dn8 = assign33940_e43064_d_n8;

        let (assign33950_e43080, assign33950_e43080_d_n5, assign33950_e43080_d_n6, assign33950_e43080_d_n7, assign33950_e43080_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33950_e43076: f64 = (var_ltat - 1.0);
        let assign33950_e43078: f64 = (assign33950_e43076 * var_ktat);
        (assign33950_e43078, ((var_ltat_dn5 * var_ktat) + (assign33950_e43076 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign33950_e43076 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign33950_e43076 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign33950_e43076 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign33950_e43080;
        var_xerfc_dn5 = assign33950_e43080_d_n5;
        var_xerfc_dn6 = assign33950_e43080_d_n6;
        var_xerfc_dn7 = assign33950_e43080_d_n7;
        var_xerfc_dn8 = assign33950_e43080_d_n8;

        let (assign33960_e43094, assign33960_e43094_d_n5, assign33960_e43094_d_n6, assign33960_e43094_d_n7, assign33960_e43094_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33960_e43092: f64 = (var_xerfc * var_xerfc);
        (assign33960_e43092, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign33960_e43094;
        var_ysq_dn5 = assign33960_e43094_d_n5;
        var_ysq_dn6 = assign33960_e43094_d_n6;
        var_ysq_dn7 = assign33960_e43094_d_n7;
        var_ysq_dn8 = assign33960_e43094_d_n8;

        let assign33970_e43097: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard672 = assign33970_e43097;

        let (assign33980_e43117, assign33980_e43117_d_n5, assign33980_e43117_d_n6, assign33980_e43117_d_n7, assign33980_e43117_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) && (var_guard672 != 0.0)) {
        let assign33980_e43113: f64 = (var_perfc * var_xerfc);
        let assign33980_e43114: f64 = (1.0 + assign33980_e43113);
        let assign33980_e43115: f64 = (1.0 / assign33980_e43114);
        (assign33980_e43115, (-((var_perfc * var_xerfc_dn5) / (assign33980_e43114 * assign33980_e43114))), (-((var_perfc * var_xerfc_dn6) / (assign33980_e43114 * assign33980_e43114))), (-((var_perfc * var_xerfc_dn7) / (assign33980_e43114 * assign33980_e43114))), (-((var_perfc * var_xerfc_dn8) / (assign33980_e43114 * assign33980_e43114))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign33980_e43117;
        var_terfc_dn5 = assign33980_e43117_d_n5;
        var_terfc_dn6 = assign33980_e43117_d_n6;
        var_terfc_dn7 = assign33980_e43117_d_n7;
        var_terfc_dn8 = assign33980_e43117_d_n8;

        let (assign33990_e43138, assign33990_e43138_d_n5, assign33990_e43138_d_n6, assign33990_e43138_d_n7, assign33990_e43138_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) && (var_guard672 == 0.0)) {
        let assign33990_e43134: f64 = (var_perfc * var_xerfc);
        let assign33990_e43135: f64 = (1.0 - assign33990_e43134);
        let assign33990_e43136: f64 = (1.0 / assign33990_e43135);
        (assign33990_e43136, (-((-(var_perfc * var_xerfc_dn5)) / (assign33990_e43135 * assign33990_e43135))), (-((-(var_perfc * var_xerfc_dn6)) / (assign33990_e43135 * assign33990_e43135))), (-((-(var_perfc * var_xerfc_dn7)) / (assign33990_e43135 * assign33990_e43135))), (-((-(var_perfc * var_xerfc_dn8)) / (assign33990_e43135 * assign33990_e43135))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign33990_e43138;
        var_terfc_dn5 = assign33990_e43138_d_n5;
        var_terfc_dn6 = assign33990_e43138_d_n6;
        var_terfc_dn7 = assign33990_e43138_d_n7;
        var_terfc_dn8 = assign33990_e43138_d_n8;

        let assign34000_e43140: f64 = (-var_ysq);
        let assign34000_e43142: f64 = (assign34000_e43140 + var_mtat);
        let assign34000_e43144: f64 = (-230.25850929940458);
        let assign34000_e43145: f64 = if assign34000_e43142 > assign34000_e43144 { 1.0 } else { 0.0 };
        var_guard673 = assign34000_e43145;

        let (assign34010_e43163, assign34010_e43163_d_n5, assign34010_e43163_d_n6, assign34010_e43163_d_n7, assign34010_e43163_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) && (var_guard673 != 0.0)) {
        let assign34010_e43158: f64 = (-var_ysq);
        let assign34010_e43160: f64 = (assign34010_e43158 + var_mtat);
        let assign34010_e43161: f64 = (assign34010_e43160).exp();
        (assign34010_e43161, (assign34010_e43161 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign34010_e43161 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign34010_e43161 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign34010_e43161 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34010_e43163;
        var_tmp_dn5 = assign34010_e43163_d_n5;
        var_tmp_dn6 = assign34010_e43163_d_n6;
        var_tmp_dn7 = assign34010_e43163_d_n7;
        var_tmp_dn8 = assign34010_e43163_d_n8;

        let (assign34020_e43212, assign34020_e43212_d_n5, assign34020_e43212_d_n6, assign34020_e43212_d_n7, assign34020_e43212_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) && (var_guard673 == 0.0)) {
        let assign34020_e43179: f64 = (-230.25850929940458);
        let assign34020_e43181: f64 = (-var_ysq);
        let assign34020_e43183: f64 = (assign34020_e43181 + var_mtat);
        let assign34020_e43184: f64 = (assign34020_e43179 - assign34020_e43183);
        let assign34020_e43188: f64 = (-230.25850929940458);
        let assign34020_e43190: f64 = (-var_ysq);
        let assign34020_e43192: f64 = (assign34020_e43190 + var_mtat);
        let assign34020_e43193: f64 = (assign34020_e43188 - assign34020_e43192);
        let assign34020_e43196: f64 = (-230.25850929940458);
        let assign34020_e43198: f64 = (-var_ysq);
        let assign34020_e43200: f64 = (assign34020_e43198 + var_mtat);
        let assign34020_e43201: f64 = (assign34020_e43196 - assign34020_e43200);
        let assign34020_e43203: f64 = (assign34020_e43201 * 0.3333333333333333);
        let assign34020_e43204: f64 = (1.0 + assign34020_e43203);
        let assign34020_e43205: f64 = (assign34020_e43193 * assign34020_e43204);
        let assign34020_e43206: f64 = (0.5 * assign34020_e43205);
        let assign34020_e43207: f64 = (1.0 + assign34020_e43206);
        let assign34020_e43208: f64 = (assign34020_e43184 * assign34020_e43207);
        let assign34020_e43209: f64 = (1.0 + assign34020_e43208);
        let assign34020_e43210: f64 = (1e-100 / assign34020_e43209);
        (assign34020_e43210, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign34020_e43207) + (assign34020_e43184 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign34020_e43204) + (assign34020_e43193 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign34020_e43209 * assign34020_e43209))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign34020_e43207) + (assign34020_e43184 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign34020_e43204) + (assign34020_e43193 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign34020_e43209 * assign34020_e43209))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign34020_e43207) + (assign34020_e43184 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign34020_e43204) + (assign34020_e43193 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign34020_e43209 * assign34020_e43209))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign34020_e43207) + (assign34020_e43184 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign34020_e43204) + (assign34020_e43193 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign34020_e43209 * assign34020_e43209))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34020_e43212;
        var_tmp_dn5 = assign34020_e43212_d_n5;
        var_tmp_dn6 = assign34020_e43212_d_n6;
        var_tmp_dn7 = assign34020_e43212_d_n7;
        var_tmp_dn8 = assign34020_e43212_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard667_slot = var_guard667;
        *var_guard668_slot = var_guard668;
        *var_guard669_slot = var_guard669;
        *var_guard670_slot = var_guard670;
        *var_guard671_slot = var_guard671;
        *var_guard672_slot = var_guard672;
        *var_guard673_slot = var_guard673;
        *var_id__blk213_slot = var_id__blk213;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_70(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti_d: f64,
        var_berfc: f64,
        var_cbbtstid_i: f64,
        var_cerfc: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_ctatstid_i: f64,
        var_fbbtsti_d: f64,
        var_fstopsti_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard666: f64,
        var_guard670: f64,
        var_idmult: f64,
        var_idsatgat_d: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lgdrain_i: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_pstid_i: f64,
        var_slopesti_d: f64,
        var_terfc: f64,
        var_terfc_dn5: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_two_psistar: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbigat_d: f64,
        var_vbirstid_i: f64,
        var_vbirstiinv_d: f64,
        var_vbrinvsti_d: f64,
        var_vbrstid_i: f64,
        var_vjsrh: f64,
        var_wdepnulrinvsti_d: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_xerfc: f64,
        var_dwsrh_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard674_slot: &mut f64,
        var_guard675_slot: &mut f64,
        var_guard676_slot: &mut f64,
        var_guard677_slot: &mut f64,
        var_guard678_slot: &mut f64,
        var_guard679_slot: &mut f64,
        var_guard680_slot: &mut f64,
        var_guard681_slot: &mut f64,
        var_guard682_slot: &mut f64,
        var_guard683_slot: &mut f64,
        var_guard684_slot: &mut f64,
        var_guard685_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard674: f64 = *var_guard674_slot;
        let mut var_guard675: f64 = *var_guard675_slot;
        let mut var_guard676: f64 = *var_guard676_slot;
        let mut var_guard677: f64 = *var_guard677_slot;
        let mut var_guard678: f64 = *var_guard678_slot;
        let mut var_guard679: f64 = *var_guard679_slot;
        let mut var_guard680: f64 = *var_guard680_slot;
        let mut var_guard681: f64 = *var_guard681_slot;
        let mut var_guard682: f64 = *var_guard682_slot;
        let mut var_guard683: f64 = *var_guard683_slot;
        let mut var_guard684: f64 = *var_guard684_slot;
        let mut var_guard685: f64 = *var_guard685_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign34030_e43242, assign34030_e43242_d_n5, assign34030_e43242_d_n6, assign34030_e43242_d_n7, assign34030_e43242_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign34030_e43224: f64 = (0.29214664 * var_terfc);
        let assign34030_e43228: f64 = (var_terfc * var_terfc);
        let assign34030_e43229: f64 = (var_berfc * assign34030_e43228);
        let assign34030_e43230: f64 = (assign34030_e43224 + assign34030_e43229);
        let assign34030_e43234: f64 = (var_terfc * var_terfc);
        let assign34030_e43236: f64 = (assign34030_e43234 * var_terfc);
        let assign34030_e43237: f64 = (var_cerfc * assign34030_e43236);
        let assign34030_e43238: f64 = (assign34030_e43230 + assign34030_e43237);
        let assign34030_e43240: f64 = (assign34030_e43238 * var_tmp);
        (assign34030_e43240, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign34030_e43234 * var_terfc_dn5)))) * var_tmp) + (assign34030_e43238 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign34030_e43234 * var_terfc_dn6)))) * var_tmp) + (assign34030_e43238 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign34030_e43234 * var_terfc_dn7)))) * var_tmp) + (assign34030_e43238 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign34030_e43234 * var_terfc_dn8)))) * var_tmp) + (assign34030_e43238 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign34030_e43242;
        var_erfcpos_dn5 = assign34030_e43242_d_n5;
        var_erfcpos_dn6 = assign34030_e43242_d_n6;
        var_erfcpos_dn7 = assign34030_e43242_d_n7;
        var_erfcpos_dn8 = assign34030_e43242_d_n8;

        let assign34040_e43245: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard674 = assign34040_e43245;

        let (assign34050_e43259, assign34050_e43259_d_n5, assign34050_e43259_d_n6, assign34050_e43259_d_n7, assign34050_e43259_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) && (var_guard674 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign34050_e43259;
        var_erfctimesexpmtat_dn5 = assign34050_e43259_d_n5;
        var_erfctimesexpmtat_dn6 = assign34050_e43259_d_n6;
        var_erfctimesexpmtat_dn7 = assign34050_e43259_d_n7;
        var_erfctimesexpmtat_dn8 = assign34050_e43259_d_n8;

        let assign34060_e43262: f64 = (-230.25850929940458);
        let assign34060_e43263: f64 = if var_mtat > assign34060_e43262 { 1.0 } else { 0.0 };
        var_guard675 = assign34060_e43263;

        let (assign34070_e43281, assign34070_e43281_d_n5, assign34070_e43281_d_n6, assign34070_e43281_d_n7, assign34070_e43281_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) && (var_guard674 == 0.0)) && (var_guard675 != 0.0)) {
        let assign34070_e43279: f64 = (var_mtat).exp();
        (assign34070_e43279, (assign34070_e43279 * var_mtat_dn5), (assign34070_e43279 * var_mtat_dn6), (assign34070_e43279 * var_mtat_dn7), (assign34070_e43279 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34070_e43281;
        var_tmp_dn5 = assign34070_e43281_d_n5;
        var_tmp_dn6 = assign34070_e43281_d_n6;
        var_tmp_dn7 = assign34070_e43281_d_n7;
        var_tmp_dn8 = assign34070_e43281_d_n8;

        let (assign34080_e43324, assign34080_e43324_d_n5, assign34080_e43324_d_n6, assign34080_e43324_d_n7, assign34080_e43324_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) && (var_guard674 == 0.0)) && (var_guard675 == 0.0)) {
        let assign34080_e43300: f64 = (-230.25850929940458);
        let assign34080_e43302: f64 = (assign34080_e43300 - var_mtat);
        let assign34080_e43306: f64 = (-230.25850929940458);
        let assign34080_e43308: f64 = (assign34080_e43306 - var_mtat);
        let assign34080_e43311: f64 = (-230.25850929940458);
        let assign34080_e43313: f64 = (assign34080_e43311 - var_mtat);
        let assign34080_e43315: f64 = (assign34080_e43313 * 0.3333333333333333);
        let assign34080_e43316: f64 = (1.0 + assign34080_e43315);
        let assign34080_e43317: f64 = (assign34080_e43308 * assign34080_e43316);
        let assign34080_e43318: f64 = (0.5 * assign34080_e43317);
        let assign34080_e43319: f64 = (1.0 + assign34080_e43318);
        let assign34080_e43320: f64 = (assign34080_e43302 * assign34080_e43319);
        let assign34080_e43321: f64 = (1.0 + assign34080_e43320);
        let assign34080_e43322: f64 = (1e-100 / assign34080_e43321);
        (assign34080_e43322, (-((1e-100 * (((-var_mtat_dn5) * assign34080_e43319) + (assign34080_e43302 * (0.5 * (((-var_mtat_dn5) * assign34080_e43316) + (assign34080_e43308 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign34080_e43321 * assign34080_e43321))), (-((1e-100 * (((-var_mtat_dn6) * assign34080_e43319) + (assign34080_e43302 * (0.5 * (((-var_mtat_dn6) * assign34080_e43316) + (assign34080_e43308 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign34080_e43321 * assign34080_e43321))), (-((1e-100 * (((-var_mtat_dn7) * assign34080_e43319) + (assign34080_e43302 * (0.5 * (((-var_mtat_dn7) * assign34080_e43316) + (assign34080_e43308 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign34080_e43321 * assign34080_e43321))), (-((1e-100 * (((-var_mtat_dn8) * assign34080_e43319) + (assign34080_e43302 * (0.5 * (((-var_mtat_dn8) * assign34080_e43316) + (assign34080_e43308 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign34080_e43321 * assign34080_e43321))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34080_e43324;
        var_tmp_dn5 = assign34080_e43324_d_n5;
        var_tmp_dn6 = assign34080_e43324_d_n6;
        var_tmp_dn7 = assign34080_e43324_d_n7;
        var_tmp_dn8 = assign34080_e43324_d_n8;

        let (assign34090_e43343, assign34090_e43343_d_n5, assign34090_e43343_d_n6, assign34090_e43343_d_n7, assign34090_e43343_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) && (var_guard674 == 0.0)) {
        let assign34090_e43339: f64 = (2.0 * var_tmp);
        let assign34090_e43341: f64 = (assign34090_e43339 - var_erfcpos);
        (assign34090_e43341, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign34090_e43343;
        var_erfctimesexpmtat_dn5 = assign34090_e43343_d_n5;
        var_erfctimesexpmtat_dn6 = assign34090_e43343_d_n6;
        var_erfctimesexpmtat_dn7 = assign34090_e43343_d_n7;
        var_erfctimesexpmtat_dn8 = assign34090_e43343_d_n8;

        let (assign34100_e43363, assign34100_e43363_d_n5, assign34100_e43363_d_n6, assign34100_e43363_d_n7, assign34100_e43363_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign34100_e43355: f64 = (1.772453850905516 * 0.5);
        let assign34100_e43358: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign34100_e43360: f64 = (assign34100_e43358 / var_ktat);
        let assign34100_e43361: f64 = (assign34100_e43355 * assign34100_e43360);
        (assign34100_e43361, (assign34100_e43355 * ((((var_atatsti_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign34100_e43358 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign34100_e43355 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign34100_e43358 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign34100_e43355 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign34100_e43358 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign34100_e43355 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign34100_e43358 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign34100_e43363;
        var_gammamax_dn5 = assign34100_e43363_d_n5;
        var_gammamax_dn6 = assign34100_e43363_d_n6;
        var_gammamax_dn7 = assign34100_e43363_d_n7;
        var_gammamax_dn8 = assign34100_e43363_d_n8;

        let (assign34110_e43381, assign34110_e43381_d_n5, assign34110_e43381_d_n6, assign34110_e43381_d_n7, assign34110_e43381_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard670 == 0.0)) {
        let assign34110_e43376: f64 = (var_asrh * var_gammamax);
        let assign34110_e43378: f64 = (assign34110_e43376 * var_wtat);
        let assign34110_e43379: f64 = (var_ctatstid_i * assign34110_e43378);
        (assign34110_e43379, (var_ctatstid_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign34110_e43376 * var_wtat_dn5))), (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign34110_e43376 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign34110_e43376 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign34110_e43376 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign34110_e43381;
        var_itat_dn5 = assign34110_e43381_d_n5;
        var_itat_dn6 = assign34110_e43381_d_n6;
        var_itat_dn7 = assign34110_e43381_d_n7;
        var_itat_dn8 = assign34110_e43381_d_n8;

        let assign34120_e43384: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard676 = assign34120_e43384;

        let (assign34130_e43395, assign34130_e43395_d_n5, assign34130_e43395_d_n6, assign34130_e43395_d_n7, assign34130_e43395_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard676 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign34130_e43395;
        var_ibbt_dn5 = assign34130_e43395_d_n5;
        var_ibbt_dn6 = assign34130_e43395_d_n6;
        var_ibbt_dn7 = assign34130_e43395_d_n7;
        var_ibbt_dn8 = assign34130_e43395_d_n8;

        let assign34140_e43398: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard677 = assign34140_e43398;

        let (assign34150_e43417, assign34150_e43417_d_n5, assign34150_e43417_d_n6, assign34150_e43417_d_n7, assign34150_e43417_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard676 == 0.0)) && (var_guard677 != 0.0)) {
        let assign34150_e43412: f64 = (var_vbirstid_i - var_vbbt);
        let assign34150_e43414: f64 = (assign34150_e43412 * var_vbirstiinv_d);
        let assign34150_e43415: f64 = (assign34150_e43414).sqrt();
        (assign34150_e43415, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34150_e43417;
        var_tmp_dn5 = assign34150_e43417_d_n5;
        var_tmp_dn6 = assign34150_e43417_d_n6;
        var_tmp_dn7 = assign34150_e43417_d_n7;
        var_tmp_dn8 = assign34150_e43417_d_n8;

        let (assign34160_e43438, assign34160_e43438_d_n5, assign34160_e43438_d_n6, assign34160_e43438_d_n7, assign34160_e43438_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard676 == 0.0)) && (var_guard677 == 0.0)) {
        let assign34160_e43432: f64 = (var_vbirstid_i - var_vbbt);
        let assign34160_e43434: f64 = (assign34160_e43432 * var_vbirstiinv_d);
        let assign34160_e43436: f64 = (assign34160_e43434).powf(var_pstid_i);
        (assign34160_e43436, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34160_e43438;
        var_tmp_dn5 = assign34160_e43438_d_n5;
        var_tmp_dn6 = assign34160_e43438_d_n6;
        var_tmp_dn7 = assign34160_e43438_d_n7;
        var_tmp_dn8 = assign34160_e43438_d_n8;

        let (assign34170_e43458, assign34170_e43458_d_n5, assign34170_e43458_d_n6, assign34170_e43458_d_n7, assign34170_e43458_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34170_e43451: f64 = (var_vbirstid_i - var_vbbt);
        let assign34170_e43453: f64 = (assign34170_e43451 * var_wdepnulrinvsti_d);
        let assign34170_e43455: f64 = (assign34170_e43453 / var_tmp);
        let assign34170_e43456: f64 = (var_one_over_one_minus_psti_d * assign34170_e43455);
        (assign34170_e43456, (var_one_over_one_minus_psti_d * (-((assign34170_e43453 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign34170_e43453 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign34170_e43453 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign34170_e43453 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign34170_e43458;
        var_fmaxr_dn5 = assign34170_e43458_d_n5;
        var_fmaxr_dn6 = assign34170_e43458_d_n6;
        var_fmaxr_dn7 = assign34170_e43458_d_n7;
        var_fmaxr_dn8 = assign34170_e43458_d_n8;

        let assign34180_e43460: f64 = (-var_fbbtsti_d);
        let assign34180_e43462: f64 = (assign34180_e43460 / var_fmaxr);
        let assign34180_e43463: f64 = (assign34180_e43462).abs();
        let assign34180_e43465: f64 = if assign34180_e43463 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard678 = assign34180_e43465;

        let (assign34190_e43483, assign34190_e43483_d_n5, assign34190_e43483_d_n6, assign34190_e43483_d_n7, assign34190_e43483_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard676 == 0.0)) && (var_guard678 != 0.0)) {
        let assign34190_e43478: f64 = (-var_fbbtsti_d);
        let assign34190_e43480: f64 = (assign34190_e43478 / var_fmaxr);
        let assign34190_e43481: f64 = (assign34190_e43480).exp();
        (assign34190_e43481, (assign34190_e43481 * (-((assign34190_e43478 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign34190_e43481 * (-((assign34190_e43478 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign34190_e43481 * (-((assign34190_e43478 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign34190_e43481 * (-((assign34190_e43478 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34190_e43483;
        var_tmp_dn5 = assign34190_e43483_d_n5;
        var_tmp_dn6 = assign34190_e43483_d_n6;
        var_tmp_dn7 = assign34190_e43483_d_n7;
        var_tmp_dn8 = assign34190_e43483_d_n8;

        let assign34200_e43485: f64 = (-var_fbbtsti_d);
        let assign34200_e43487: f64 = (assign34200_e43485 / var_fmaxr);
        let assign34200_e43489: f64 = if assign34200_e43487 < 0.0 { 1.0 } else { 0.0 };
        var_guard679 = assign34200_e43489;

        let (assign34210_e43540, assign34210_e43540_d_n5, assign34210_e43540_d_n6, assign34210_e43540_d_n7, assign34210_e43540_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard676 == 0.0)) && (var_guard678 == 0.0)) && (var_guard679 != 0.0)) {
        let assign34210_e43507: f64 = (-230.25850929940458);
        let assign34210_e43509: f64 = (-var_fbbtsti_d);
        let assign34210_e43511: f64 = (assign34210_e43509 / var_fmaxr);
        let assign34210_e43512: f64 = (assign34210_e43507 - assign34210_e43511);
        let assign34210_e43516: f64 = (-230.25850929940458);
        let assign34210_e43518: f64 = (-var_fbbtsti_d);
        let assign34210_e43520: f64 = (assign34210_e43518 / var_fmaxr);
        let assign34210_e43521: f64 = (assign34210_e43516 - assign34210_e43520);
        let assign34210_e43524: f64 = (-230.25850929940458);
        let assign34210_e43526: f64 = (-var_fbbtsti_d);
        let assign34210_e43528: f64 = (assign34210_e43526 / var_fmaxr);
        let assign34210_e43529: f64 = (assign34210_e43524 - assign34210_e43528);
        let assign34210_e43531: f64 = (assign34210_e43529 * 0.3333333333333333);
        let assign34210_e43532: f64 = (1.0 + assign34210_e43531);
        let assign34210_e43533: f64 = (assign34210_e43521 * assign34210_e43532);
        let assign34210_e43534: f64 = (0.5 * assign34210_e43533);
        let assign34210_e43535: f64 = (1.0 + assign34210_e43534);
        let assign34210_e43536: f64 = (assign34210_e43512 * assign34210_e43535);
        let assign34210_e43537: f64 = (1.0 + assign34210_e43536);
        let assign34210_e43538: f64 = (1e-100 / assign34210_e43537);
        (assign34210_e43538, (-((1e-100 * (((-(-((assign34210_e43509 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign34210_e43535) + (assign34210_e43512 * (0.5 * (((-(-((assign34210_e43518 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign34210_e43532) + (assign34210_e43521 * ((-(-((assign34210_e43526 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign34210_e43537 * assign34210_e43537))), (-((1e-100 * (((-(-((assign34210_e43509 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign34210_e43535) + (assign34210_e43512 * (0.5 * (((-(-((assign34210_e43518 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign34210_e43532) + (assign34210_e43521 * ((-(-((assign34210_e43526 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign34210_e43537 * assign34210_e43537))), (-((1e-100 * (((-(-((assign34210_e43509 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign34210_e43535) + (assign34210_e43512 * (0.5 * (((-(-((assign34210_e43518 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign34210_e43532) + (assign34210_e43521 * ((-(-((assign34210_e43526 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign34210_e43537 * assign34210_e43537))), (-((1e-100 * (((-(-((assign34210_e43509 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign34210_e43535) + (assign34210_e43512 * (0.5 * (((-(-((assign34210_e43518 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign34210_e43532) + (assign34210_e43521 * ((-(-((assign34210_e43526 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign34210_e43537 * assign34210_e43537))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34210_e43540;
        var_tmp_dn5 = assign34210_e43540_d_n5;
        var_tmp_dn6 = assign34210_e43540_d_n6;
        var_tmp_dn7 = assign34210_e43540_d_n7;
        var_tmp_dn8 = assign34210_e43540_d_n8;

        let (assign34220_e43589, assign34220_e43589_d_n5, assign34220_e43589_d_n6, assign34220_e43589_d_n7, assign34220_e43589_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard676 == 0.0)) && (var_guard678 == 0.0)) && (var_guard679 == 0.0)) {
        let assign34220_e43559: f64 = (-var_fbbtsti_d);
        let assign34220_e43561: f64 = (assign34220_e43559 / var_fmaxr);
        let assign34220_e43563: f64 = (assign34220_e43561 - 230.25850929940458);
        let assign34220_e43567: f64 = (-var_fbbtsti_d);
        let assign34220_e43569: f64 = (assign34220_e43567 / var_fmaxr);
        let assign34220_e43571: f64 = (assign34220_e43569 - 230.25850929940458);
        let assign34220_e43574: f64 = (-var_fbbtsti_d);
        let assign34220_e43576: f64 = (assign34220_e43574 / var_fmaxr);
        let assign34220_e43578: f64 = (assign34220_e43576 - 230.25850929940458);
        let assign34220_e43580: f64 = (assign34220_e43578 * 0.3333333333333333);
        let assign34220_e43581: f64 = (1.0 + assign34220_e43580);
        let assign34220_e43582: f64 = (assign34220_e43571 * assign34220_e43581);
        let assign34220_e43583: f64 = (0.5 * assign34220_e43582);
        let assign34220_e43584: f64 = (1.0 + assign34220_e43583);
        let assign34220_e43585: f64 = (assign34220_e43563 * assign34220_e43584);
        let assign34220_e43586: f64 = (1.0 + assign34220_e43585);
        let assign34220_e43587: f64 = (1e100 * assign34220_e43586);
        (assign34220_e43587, (1e100 * (((-((assign34220_e43559 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign34220_e43584) + (assign34220_e43563 * (0.5 * (((-((assign34220_e43567 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign34220_e43581) + (assign34220_e43571 * ((-((assign34220_e43574 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign34220_e43559 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign34220_e43584) + (assign34220_e43563 * (0.5 * (((-((assign34220_e43567 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign34220_e43581) + (assign34220_e43571 * ((-((assign34220_e43574 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign34220_e43559 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign34220_e43584) + (assign34220_e43563 * (0.5 * (((-((assign34220_e43567 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign34220_e43581) + (assign34220_e43571 * ((-((assign34220_e43574 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign34220_e43559 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign34220_e43584) + (assign34220_e43563 * (0.5 * (((-((assign34220_e43567 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign34220_e43581) + (assign34220_e43571 * ((-((assign34220_e43574 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34220_e43589;
        var_tmp_dn5 = assign34220_e43589_d_n5;
        var_tmp_dn6 = assign34220_e43589_d_n6;
        var_tmp_dn7 = assign34220_e43589_d_n7;
        var_tmp_dn8 = assign34220_e43589_d_n8;

        let (assign34230_e43609, assign34230_e43609_d_n5, assign34230_e43609_d_n6, assign34230_e43609_d_n7, assign34230_e43609_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34230_e43602: f64 = (var_v3 * var_fmaxr);
        let assign34230_e43604: f64 = (assign34230_e43602 * var_fmaxr);
        let assign34230_e43606: f64 = (assign34230_e43604 * var_tmp);
        let assign34230_e43607: f64 = (var_cbbtstid_i * assign34230_e43606);
        (assign34230_e43607, (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign34230_e43602 * var_fmaxr_dn5)) * var_tmp) + (assign34230_e43604 * var_tmp_dn5))), (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign34230_e43602 * var_fmaxr_dn6)) * var_tmp) + (assign34230_e43604 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign34230_e43602 * var_fmaxr_dn7)) * var_tmp) + (assign34230_e43604 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign34230_e43602 * var_fmaxr_dn8)) * var_tmp) + (assign34230_e43604 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign34230_e43609;
        var_ibbt_dn5 = assign34230_e43609_d_n5;
        var_ibbt_dn6 = assign34230_e43609_d_n6;
        var_ibbt_dn7 = assign34230_e43609_d_n7;
        var_ibbt_dn8 = assign34230_e43609_d_n8;

        let assign34240_e43612: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard680 = assign34240_e43612;

        let (assign34250_e43623, assign34250_e43623_d_n5, assign34250_e43623_d_n6, assign34250_e43623_d_n7, assign34250_e43623_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard680 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign34250_e43623;
        var_fbreakdown_dn5 = assign34250_e43623_d_n5;
        var_fbreakdown_dn6 = assign34250_e43623_d_n6;
        var_fbreakdown_dn7 = assign34250_e43623_d_n7;
        var_fbreakdown_dn8 = assign34250_e43623_d_n8;

        let assign34260_e43626: f64 = (-var_alphaav);
        let assign34260_e43628: f64 = (assign34260_e43626 * var_vbrstid_i);
        let assign34260_e43629: f64 = if var_vav > assign34260_e43628 { 1.0 } else { 0.0 };
        var_guard681 = assign34260_e43629;

        let assign34270_e43632: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard682 = assign34270_e43632;

        let (assign34280_e43662, assign34280_e43662_d_n5, assign34280_e43662_d_n6, assign34280_e43662_d_n7, assign34280_e43662_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard680 == 0.0)) && (var_guard681 != 0.0)) && (var_guard682 != 0.0)) {
        let assign34280_e43648: f64 = (var_vav * var_vbrinvsti_d);
        let assign34280_e43651: f64 = (var_vav * var_vbrinvsti_d);
        let assign34280_e43652: f64 = (assign34280_e43648 * assign34280_e43651);
        let assign34280_e43655: f64 = (var_vav * var_vbrinvsti_d);
        let assign34280_e43656: f64 = (assign34280_e43652 * assign34280_e43655);
        let assign34280_e43659: f64 = (var_vav * var_vbrinvsti_d);
        let assign34280_e43660: f64 = (assign34280_e43656 * assign34280_e43659);
        (assign34280_e43660, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34280_e43662;
        var_tmp_dn5 = assign34280_e43662_d_n5;
        var_tmp_dn6 = assign34280_e43662_d_n6;
        var_tmp_dn7 = assign34280_e43662_d_n7;
        var_tmp_dn8 = assign34280_e43662_d_n8;

        let (assign34290_e43684, assign34290_e43684_d_n5, assign34290_e43684_d_n6, assign34290_e43684_d_n7, assign34290_e43684_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard680 == 0.0)) && (var_guard681 != 0.0)) && (var_guard682 == 0.0)) {
        let assign34290_e43679: f64 = (var_vav * var_vbrinvsti_d);
        let assign34290_e43680: f64 = (assign34290_e43679).abs();
        let assign34290_e43682: f64 = (assign34290_e43680).powf(var_pbrstid_i);
        (assign34290_e43682, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34290_e43684;
        var_tmp_dn5 = assign34290_e43684_d_n5;
        var_tmp_dn6 = assign34290_e43684_d_n6;
        var_tmp_dn7 = assign34290_e43684_d_n7;
        var_tmp_dn8 = assign34290_e43684_d_n8;

        let (assign34300_e43702, assign34300_e43702_d_n5, assign34300_e43702_d_n6, assign34300_e43702_d_n7, assign34300_e43702_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard680 == 0.0)) && (var_guard681 != 0.0)) {
        let assign34300_e43699: f64 = (1.0 - var_tmp);
        let assign34300_e43700: f64 = (1.0 / assign34300_e43699);
        (assign34300_e43700, (-((-var_tmp_dn5) / (assign34300_e43699 * assign34300_e43699))), (-((-var_tmp_dn6) / (assign34300_e43699 * assign34300_e43699))), (-((-var_tmp_dn7) / (assign34300_e43699 * assign34300_e43699))), (-((-var_tmp_dn8) / (assign34300_e43699 * assign34300_e43699))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign34300_e43702;
        var_fbreakdown_dn5 = assign34300_e43702_d_n5;
        var_fbreakdown_dn6 = assign34300_e43702_d_n6;
        var_fbreakdown_dn7 = assign34300_e43702_d_n7;
        var_fbreakdown_dn8 = assign34300_e43702_d_n8;

        let (assign34310_e43725, assign34310_e43725_d_n5, assign34310_e43725_d_n6, assign34310_e43725_d_n7, assign34310_e43725_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) && (var_guard680 == 0.0)) && (var_guard681 == 0.0)) {
        let assign34310_e43719: f64 = (var_alphaav * var_vbrstid_i);
        let assign34310_e43720: f64 = (var_vav + assign34310_e43719);
        let assign34310_e43722: f64 = (assign34310_e43720 * var_slopesti_d);
        let assign34310_e43723: f64 = (var_fstopsti_d + assign34310_e43722);
        (assign34310_e43723, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign34310_e43725;
        var_fbreakdown_dn5 = assign34310_e43725_d_n5;
        var_fbreakdown_dn6 = assign34310_e43725_d_n6;
        var_fbreakdown_dn7 = assign34310_e43725_d_n7;
        var_fbreakdown_dn8 = assign34310_e43725_d_n8;

        let (assign34320_e43744, assign34320_e43744_d_n5, assign34320_e43744_d_n6, assign34320_e43744_d_n7, assign34320_e43744_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard666 == 0.0)) {
        let assign34320_e43735: f64 = (var_id__blk213 + var_isrh);
        let assign34320_e43737: f64 = (assign34320_e43735 + var_itat);
        let assign34320_e43739: f64 = (assign34320_e43737 + var_ibbt);
        let assign34320_e43740: f64 = (p.p29 * assign34320_e43739);
        let assign34320_e43742: f64 = (assign34320_e43740 * var_fbreakdown);
        (assign34320_e43742, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign34320_e43740 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign34320_e43740 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign34320_e43740 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign34320_e43740 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign34320_e43744;
        var_ijunsti_dn5 = assign34320_e43744_d_n5;
        var_ijunsti_dn6 = assign34320_e43744_d_n6;
        var_ijunsti_dn7 = assign34320_e43744_d_n7;
        var_ijunsti_dn8 = assign34320_e43744_d_n8;

        let assign34330_e43747: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard683 = assign34330_e43747;

        let (assign34340_e43755, assign34340_e43755_d_n5, assign34340_e43755_d_n6, assign34340_e43755_d_n7, assign34340_e43755_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign34340_e43755;
        var_ijungat_dn5 = assign34340_e43755_d_n5;
        var_ijungat_dn6 = assign34340_e43755_d_n6;
        var_ijungat_dn7 = assign34340_e43755_d_n7;
        var_ijungat_dn8 = assign34340_e43755_d_n8;

        let (assign34350_e43766,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) {
        let assign34350_e43764: f64 = (var_idsatgat_d * var_idmult);
        (assign34350_e43764,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign34350_e43766;

        let assign34360_e43773: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard684 = assign34360_e43773;

        let (assign34370_e43784, assign34370_e43784_d_n5, assign34370_e43784_d_n6, assign34370_e43784_d_n7, assign34370_e43784_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign34370_e43784;
        var_isrh_dn5 = assign34370_e43784_d_n5;
        var_isrh_dn6 = assign34370_e43784_d_n6;
        var_isrh_dn7 = assign34370_e43784_d_n7;
        var_isrh_dn8 = assign34370_e43784_d_n8;

        let (assign34380_e43798,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) {
        let assign34380_e43796: f64 = (var_vbigat_d - var_vjsrh);
        (assign34380_e43796,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign34380_e43798;

        let (assign34390_e43817,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) {
        let assign34390_e43812: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign34390_e43813: f64 = (1.0 - assign34390_e43812);
        let assign34390_e43814: f64 = (assign34390_e43813).sqrt();
        let assign34390_e43815: f64 = (1.0 - assign34390_e43814);
        (assign34390_e43815,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign34390_e43817;

        let assign34400_e43820: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard685 = assign34400_e43820;

        let (assign34410_e43834,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) && (var_guard685 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign34410_e43834;

        let (assign34420_e43866,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) && (var_guard685 == 0.0)) {
        let assign34420_e43849: f64 = (var_wsrhstep * var_wsrhstep);
        let assign34420_e43851: f64 = (var_wsrhstep).ln();
        let assign34420_e43852: f64 = (assign34420_e43849 * assign34420_e43851);
        let assign34420_e43855: f64 = (1.0 - var_wsrhstep);
        let assign34420_e43856: f64 = (assign34420_e43852 / assign34420_e43855);
        let assign34420_e43858: f64 = (assign34420_e43856 + var_wsrhstep);
        let assign34420_e43862: f64 = (2.0 * var_pgatd_i);
        let assign34420_e43863: f64 = (1.0 - assign34420_e43862);
        let assign34420_e43864: f64 = (assign34420_e43858 * assign34420_e43863);
        (assign34420_e43864,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign34420_e43866;

        *var_dwsrh_slot = var_dwsrh;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard674_slot = var_guard674;
        *var_guard675_slot = var_guard675;
        *var_guard676_slot = var_guard676;
        *var_guard677_slot = var_guard677;
        *var_guard678_slot = var_guard678;
        *var_guard679_slot = var_guard679;
        *var_guard680_slot = var_guard680;
        *var_guard681_slot = var_guard681;
        *var_guard682_slot = var_guard682;
        *var_guard683_slot = var_guard683;
        *var_guard684_slot = var_guard684;
        *var_guard685_slot = var_guard685;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_71(
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btatpartgat_d: f64,
        var_cerfc: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_dwsrh: f64,
        var_ftdgat_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard683: f64,
        var_guard684: f64,
        var_one_minus_pgat_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirgatinv_d: f64,
        var_wdepnulrgat_d: f64,
        var_wsrhstep: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_guard686_slot: &mut f64,
        var_guard687_slot: &mut f64,
        var_guard688_slot: &mut f64,
        var_guard689_slot: &mut f64,
        var_guard690_slot: &mut f64,
        var_guard691_slot: &mut f64,
        var_guard692_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_guard686: f64 = *var_guard686_slot;
        let mut var_guard687: f64 = *var_guard687_slot;
        let mut var_guard688: f64 = *var_guard688_slot;
        let mut var_guard689: f64 = *var_guard689_slot;
        let mut var_guard690: f64 = *var_guard690_slot;
        let mut var_guard691: f64 = *var_guard691_slot;
        let mut var_guard692: f64 = *var_guard692_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign34430_e43880,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) {
        let assign34430_e43878: f64 = (var_wsrhstep + var_dwsrh);
        (assign34430_e43878,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign34430_e43880;

        let assign34440_e43883: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard686 = assign34440_e43883;

        let (assign34450_e43900, assign34450_e43900_d_n5, assign34450_e43900_d_n6, assign34450_e43900_d_n7, assign34450_e43900_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) && (var_guard686 != 0.0)) {
        let assign34450_e43897: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign34450_e43898: f64 = (assign34450_e43897).sqrt();
        (assign34450_e43898, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34450_e43900;
        var_tmp_dn5 = assign34450_e43900_d_n5;
        var_tmp_dn6 = assign34450_e43900_d_n6;
        var_tmp_dn7 = assign34450_e43900_d_n7;
        var_tmp_dn8 = assign34450_e43900_d_n8;

        let (assign34460_e43919, assign34460_e43919_d_n5, assign34460_e43919_d_n6, assign34460_e43919_d_n7, assign34460_e43919_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) && (var_guard686 == 0.0)) {
        let assign34460_e43915: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign34460_e43917: f64 = (assign34460_e43915).powf(var_pgatd_i);
        (assign34460_e43917, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34460_e43919;
        var_tmp_dn5 = assign34460_e43919_d_n5;
        var_tmp_dn6 = assign34460_e43919_d_n6;
        var_tmp_dn7 = assign34460_e43919_d_n7;
        var_tmp_dn8 = assign34460_e43919_d_n8;

        let (assign34470_e43933, assign34470_e43933_d_n5, assign34470_e43933_d_n6, assign34470_e43933_d_n7, assign34470_e43933_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) {
        let assign34470_e43931: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign34470_e43931, (var_wdepnulrgat_d * var_tmp_dn5), (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign34470_e43933;
        var_wdep_dn5 = assign34470_e43933_d_n5;
        var_wdep_dn6 = assign34470_e43933_d_n6;
        var_wdep_dn7 = assign34470_e43933_d_n7;
        var_wdep_dn8 = assign34470_e43933_d_n8;

        let (assign34480_e43951, assign34480_e43951_d_n5, assign34480_e43951_d_n6, assign34480_e43951_d_n7, assign34480_e43951_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) {
        let assign34480_e43946: f64 = (var_zinv - 1.0);
        let assign34480_e43948: f64 = (assign34480_e43946 * var_wdep);
        let assign34480_e43949: f64 = (var_ftdgat_d * assign34480_e43948);
        (assign34480_e43949, (var_ftdgat_d * (assign34480_e43946 * var_wdep_dn5)), (var_ftdgat_d * (assign34480_e43946 * var_wdep_dn6)), (var_ftdgat_d * (assign34480_e43946 * var_wdep_dn7)), (var_ftdgat_d * (assign34480_e43946 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign34480_e43951;
        var_asrh_dn5 = assign34480_e43951_d_n5;
        var_asrh_dn6 = assign34480_e43951_d_n6;
        var_asrh_dn7 = assign34480_e43951_d_n7;
        var_asrh_dn8 = assign34480_e43951_d_n8;

        let (assign34490_e43967, assign34490_e43967_d_n5, assign34490_e43967_d_n6, assign34490_e43967_d_n7, assign34490_e43967_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) {
        let assign34490_e43964: f64 = (var_asrh * var_wsrh);
        let assign34490_e43965: f64 = (var_csrhgatd_i * assign34490_e43964);
        (assign34490_e43965, (var_csrhgatd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign34490_e43967;
        var_isrh_dn5 = assign34490_e43967_d_n5;
        var_isrh_dn6 = assign34490_e43967_d_n6;
        var_isrh_dn7 = assign34490_e43967_d_n7;
        var_isrh_dn8 = assign34490_e43967_d_n8;

        let assign34500_e43970: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard687 = assign34500_e43970;

        let (assign34510_e43981, assign34510_e43981_d_n5, assign34510_e43981_d_n6, assign34510_e43981_d_n7, assign34510_e43981_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign34510_e43981;
        var_itat_dn5 = assign34510_e43981_d_n5;
        var_itat_dn6 = assign34510_e43981_d_n6;
        var_itat_dn7 = assign34510_e43981_d_n7;
        var_itat_dn8 = assign34510_e43981_d_n8;

        let (assign34520_e43999, assign34520_e43999_d_n5, assign34520_e43999_d_n6, assign34520_e43999_d_n7, assign34520_e43999_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34520_e43994: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign34520_e43996: f64 = (assign34520_e43994 / var_vbi_minus_vjsrh);
        let assign34520_e43997: f64 = (var_btatpartgat_d * assign34520_e43996);
        (assign34520_e43997, (var_btatpartgat_d * ((var_wdep_dn5 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign34520_e43999;
        var_btat_dn5 = assign34520_e43999_d_n5;
        var_btat_dn6 = assign34520_e43999_d_n6;
        var_btat_dn7 = assign34520_e43999_d_n7;
        var_btat_dn8 = assign34520_e43999_d_n8;

        let (assign34530_e44015, assign34530_e44015_d_n5, assign34530_e44015_d_n6, assign34530_e44015_d_n7, assign34530_e44015_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34530_e44011: f64 = (0.666666666666667 * var_atatgat_d);
        let assign34530_e44013: f64 = (assign34530_e44011 / var_btat);
        (assign34530_e44013, (-((assign34530_e44011 * var_btat_dn5) / (var_btat * var_btat))), (-((assign34530_e44011 * var_btat_dn6) / (var_btat * var_btat))), (-((assign34530_e44011 * var_btat_dn7) / (var_btat * var_btat))), (-((assign34530_e44011 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign34530_e44015;
        var_twoatatoverthreebtat_dn5 = assign34530_e44015_d_n5;
        var_twoatatoverthreebtat_dn6 = assign34530_e44015_d_n6;
        var_twoatatoverthreebtat_dn7 = assign34530_e44015_d_n7;
        var_twoatatoverthreebtat_dn8 = assign34530_e44015_d_n8;

        let (assign34540_e44029, assign34540_e44029_d_n5, assign34540_e44029_d_n6, assign34540_e44029_d_n7, assign34540_e44029_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34540_e44027: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign34540_e44027, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign34540_e44029;
        var_umaxbeforelimiting_dn5 = assign34540_e44029_d_n5;
        var_umaxbeforelimiting_dn6 = assign34540_e44029_d_n6;
        var_umaxbeforelimiting_dn7 = assign34540_e44029_d_n7;
        var_umaxbeforelimiting_dn8 = assign34540_e44029_d_n8;

        let (assign34550_e44050, assign34550_e44050_d_n5, assign34550_e44050_d_n6, assign34550_e44050_d_n7, assign34550_e44050_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34550_e44041: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign34550_e44044: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign34550_e44046: f64 = (assign34550_e44044 + 1.0);
        let assign34550_e44047: f64 = (assign34550_e44041 / assign34550_e44046);
        let assign34550_e44048: f64 = (assign34550_e44047).sqrt();
        (assign34550_e44048, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign34550_e44046) - (assign34550_e44041 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign34550_e44046 * assign34550_e44046)) / (2.0 * assign34550_e44048)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign34550_e44046) - (assign34550_e44041 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign34550_e44046 * assign34550_e44046)) / (2.0 * assign34550_e44048)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign34550_e44046) - (assign34550_e44041 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign34550_e44046 * assign34550_e44046)) / (2.0 * assign34550_e44048)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign34550_e44046) - (assign34550_e44041 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign34550_e44046 * assign34550_e44046)) / (2.0 * assign34550_e44048)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign34550_e44050;
        var_umax_dn5 = assign34550_e44050_d_n5;
        var_umax_dn6 = assign34550_e44050_d_n6;
        var_umax_dn7 = assign34550_e44050_d_n7;
        var_umax_dn8 = assign34550_e44050_d_n8;

        let (assign34560_e44063, assign34560_e44063_d_n5, assign34560_e44063_d_n6, assign34560_e44063_d_n7, assign34560_e44063_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34560_e44061: f64 = (var_umax).sqrt();
        (assign34560_e44061, (var_umax_dn5 / (2.0 * assign34560_e44061)), (var_umax_dn6 / (2.0 * assign34560_e44061)), (var_umax_dn7 / (2.0 * assign34560_e44061)), (var_umax_dn8 / (2.0 * assign34560_e44061)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign34560_e44063;
        var_sqrtumax_dn5 = assign34560_e44063_d_n5;
        var_sqrtumax_dn6 = assign34560_e44063_d_n6;
        var_sqrtumax_dn7 = assign34560_e44063_d_n7;
        var_sqrtumax_dn8 = assign34560_e44063_d_n8;

        let (assign34570_e44077, assign34570_e44077_d_n5, assign34570_e44077_d_n6, assign34570_e44077_d_n7, assign34570_e44077_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34570_e44075: f64 = (var_umax * var_sqrtumax);
        (assign34570_e44075, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign34570_e44077;
        var_umaxpoweronepointfive_dn5 = assign34570_e44077_d_n5;
        var_umaxpoweronepointfive_dn6 = assign34570_e44077_d_n6;
        var_umaxpoweronepointfive_dn7 = assign34570_e44077_d_n7;
        var_umaxpoweronepointfive_dn8 = assign34570_e44077_d_n8;

        let assign34580_e44079: f64 = (-var_pgatd_i);
        let assign34580_e44081: f64 = (assign34580_e44079 * var_one_over_one_minus_pgat_d);
        let assign34580_e44083: f64 = (-1.0);
        let assign34580_e44084: f64 = if assign34580_e44081 == assign34580_e44083 { 1.0 } else { 0.0 };
        var_guard688 = assign34580_e44084;

        let (assign34590_e44104, assign34590_e44104_d_n5, assign34590_e44104_d_n6, assign34590_e44104_d_n7, assign34590_e44104_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) && (var_guard688 != 0.0)) {
        let assign34590_e44100: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34590_e44101: f64 = (1.0 + assign34590_e44100);
        let assign34590_e44102: f64 = (1.0 / assign34590_e44101);
        (assign34590_e44102, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign34590_e44101 * assign34590_e44101))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign34590_e44101 * assign34590_e44101))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign34590_e44101 * assign34590_e44101))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign34590_e44101 * assign34590_e44101))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign34590_e44104;
        var_wgamma_dn5 = assign34590_e44104_d_n5;
        var_wgamma_dn6 = assign34590_e44104_d_n6;
        var_wgamma_dn7 = assign34590_e44104_d_n7;
        var_wgamma_dn8 = assign34590_e44104_d_n8;

        let (assign34600_e44128, assign34600_e44128_d_n5, assign34600_e44128_d_n6, assign34600_e44128_d_n7, assign34600_e44128_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) && (var_guard688 == 0.0)) {
        let assign34600_e44120: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34600_e44121: f64 = (1.0 + assign34600_e44120);
        let assign34600_e44123: f64 = (-var_pgatd_i);
        let assign34600_e44125: f64 = (assign34600_e44123 * var_one_over_one_minus_pgat_d);
        let assign34600_e44126: f64 = (assign34600_e44121).powf(assign34600_e44125);
        (assign34600_e44126, if 0.0 == 0.0 && ((assign34600_e44125) as f64).is_finite() && ((assign34600_e44125) as f64).fract() == 0.0 { if assign34600_e44125 == 0.0 { 0.0 } else { (assign34600_e44125 * ((assign34600_e44121).powf(assign34600_e44125 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign34600_e44126 * (assign34600_e44125 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign34600_e44121))) }, if 0.0 == 0.0 && ((assign34600_e44125) as f64).is_finite() && ((assign34600_e44125) as f64).fract() == 0.0 { if assign34600_e44125 == 0.0 { 0.0 } else { (assign34600_e44125 * ((assign34600_e44121).powf(assign34600_e44125 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign34600_e44126 * (assign34600_e44125 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign34600_e44121))) }, if 0.0 == 0.0 && ((assign34600_e44125) as f64).is_finite() && ((assign34600_e44125) as f64).fract() == 0.0 { if assign34600_e44125 == 0.0 { 0.0 } else { (assign34600_e44125 * ((assign34600_e44121).powf(assign34600_e44125 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign34600_e44126 * (assign34600_e44125 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign34600_e44121))) }, if 0.0 == 0.0 && ((assign34600_e44125) as f64).is_finite() && ((assign34600_e44125) as f64).fract() == 0.0 { if assign34600_e44125 == 0.0 { 0.0 } else { (assign34600_e44125 * ((assign34600_e44121).powf(assign34600_e44125 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign34600_e44126 * (assign34600_e44125 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign34600_e44121))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign34600_e44128;
        var_wgamma_dn5 = assign34600_e44128_d_n5;
        var_wgamma_dn6 = assign34600_e44128_d_n6;
        var_wgamma_dn7 = assign34600_e44128_d_n7;
        var_wgamma_dn8 = assign34600_e44128_d_n8;

        let (assign34610_e44146, assign34610_e44146_d_n5, assign34610_e44146_d_n6, assign34610_e44146_d_n7, assign34610_e44146_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34610_e44140: f64 = (var_wsrh * var_wgamma);
        let assign34610_e44143: f64 = (var_wsrh + var_wgamma);
        let assign34610_e44144: f64 = (assign34610_e44140 / assign34610_e44143);
        (assign34610_e44144, ((((var_wsrh * var_wgamma_dn5) * assign34610_e44143) - (assign34610_e44140 * var_wgamma_dn5)) / (assign34610_e44143 * assign34610_e44143)), ((((var_wsrh * var_wgamma_dn6) * assign34610_e44143) - (assign34610_e44140 * var_wgamma_dn6)) / (assign34610_e44143 * assign34610_e44143)), ((((var_wsrh * var_wgamma_dn7) * assign34610_e44143) - (assign34610_e44140 * var_wgamma_dn7)) / (assign34610_e44143 * assign34610_e44143)), ((((var_wsrh * var_wgamma_dn8) * assign34610_e44143) - (assign34610_e44140 * var_wgamma_dn8)) / (assign34610_e44143 * assign34610_e44143)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign34610_e44146;
        var_wtat_dn5 = assign34610_e44146_d_n5;
        var_wtat_dn6 = assign34610_e44146_d_n6;
        var_wtat_dn7 = assign34610_e44146_d_n7;
        var_wtat_dn8 = assign34610_e44146_d_n8;

        let (assign34620_e44163, assign34620_e44163_d_n5, assign34620_e44163_d_n6, assign34620_e44163_d_n7, assign34620_e44163_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34620_e44159: f64 = (var_btat / var_sqrtumax);
        let assign34620_e44160: f64 = (0.375 * assign34620_e44159);
        let assign34620_e44161: f64 = (assign34620_e44160).sqrt();
        (assign34620_e44161, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34620_e44161)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34620_e44161)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34620_e44161)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34620_e44161)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign34620_e44163;
        var_ktat_dn5 = assign34620_e44163_d_n5;
        var_ktat_dn6 = assign34620_e44163_d_n6;
        var_ktat_dn7 = assign34620_e44163_d_n7;
        var_ktat_dn8 = assign34620_e44163_d_n8;

        let (assign34630_e44181, assign34630_e44181_d_n5, assign34630_e44181_d_n6, assign34630_e44181_d_n7, assign34630_e44181_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34630_e44176: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign34630_e44177: f64 = (2.0 * assign34630_e44176);
        let assign34630_e44179: f64 = (assign34630_e44177 - var_umax);
        (assign34630_e44179, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign34630_e44181;
        var_ltat_dn5 = assign34630_e44181_d_n5;
        var_ltat_dn6 = assign34630_e44181_d_n6;
        var_ltat_dn7 = assign34630_e44181_d_n7;
        var_ltat_dn8 = assign34630_e44181_d_n8;

        let (assign34640_e44207, assign34640_e44207_d_n5, assign34640_e44207_d_n6, assign34640_e44207_d_n7, assign34640_e44207_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34640_e44193: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign34640_e44195: f64 = (assign34640_e44193 * var_sqrtumax);
        let assign34640_e44198: f64 = (var_atatgat_d * var_umax);
        let assign34640_e44199: f64 = (assign34640_e44195 - assign34640_e44198);
        let assign34640_e44203: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34640_e44204: f64 = (0.5 * assign34640_e44203);
        let assign34640_e44205: f64 = (assign34640_e44199 + assign34640_e44204);
        (assign34640_e44205, (((((var_atatgat_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign34640_e44193 * var_sqrtumax_dn5)) - (var_atatgat_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign34640_e44193 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign34640_e44193 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign34640_e44193 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign34640_e44207;
        var_mtat_dn5 = assign34640_e44207_d_n5;
        var_mtat_dn6 = assign34640_e44207_d_n6;
        var_mtat_dn7 = assign34640_e44207_d_n7;
        var_mtat_dn8 = assign34640_e44207_d_n8;

        let (assign34650_e44223, assign34650_e44223_d_n5, assign34650_e44223_d_n6, assign34650_e44223_d_n7, assign34650_e44223_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34650_e44219: f64 = (var_ltat - 1.0);
        let assign34650_e44221: f64 = (assign34650_e44219 * var_ktat);
        (assign34650_e44221, ((var_ltat_dn5 * var_ktat) + (assign34650_e44219 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign34650_e44219 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign34650_e44219 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign34650_e44219 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign34650_e44223;
        var_xerfc_dn5 = assign34650_e44223_d_n5;
        var_xerfc_dn6 = assign34650_e44223_d_n6;
        var_xerfc_dn7 = assign34650_e44223_d_n7;
        var_xerfc_dn8 = assign34650_e44223_d_n8;

        let (assign34660_e44237, assign34660_e44237_d_n5, assign34660_e44237_d_n6, assign34660_e44237_d_n7, assign34660_e44237_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34660_e44235: f64 = (var_xerfc * var_xerfc);
        (assign34660_e44235, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign34660_e44237;
        var_ysq_dn5 = assign34660_e44237_d_n5;
        var_ysq_dn6 = assign34660_e44237_d_n6;
        var_ysq_dn7 = assign34660_e44237_d_n7;
        var_ysq_dn8 = assign34660_e44237_d_n8;

        let assign34670_e44240: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard689 = assign34670_e44240;

        let (assign34680_e44260, assign34680_e44260_d_n5, assign34680_e44260_d_n6, assign34680_e44260_d_n7, assign34680_e44260_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) && (var_guard689 != 0.0)) {
        let assign34680_e44256: f64 = (var_perfc * var_xerfc);
        let assign34680_e44257: f64 = (1.0 + assign34680_e44256);
        let assign34680_e44258: f64 = (1.0 / assign34680_e44257);
        (assign34680_e44258, (-((var_perfc * var_xerfc_dn5) / (assign34680_e44257 * assign34680_e44257))), (-((var_perfc * var_xerfc_dn6) / (assign34680_e44257 * assign34680_e44257))), (-((var_perfc * var_xerfc_dn7) / (assign34680_e44257 * assign34680_e44257))), (-((var_perfc * var_xerfc_dn8) / (assign34680_e44257 * assign34680_e44257))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign34680_e44260;
        var_terfc_dn5 = assign34680_e44260_d_n5;
        var_terfc_dn6 = assign34680_e44260_d_n6;
        var_terfc_dn7 = assign34680_e44260_d_n7;
        var_terfc_dn8 = assign34680_e44260_d_n8;

        let (assign34690_e44281, assign34690_e44281_d_n5, assign34690_e44281_d_n6, assign34690_e44281_d_n7, assign34690_e44281_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) && (var_guard689 == 0.0)) {
        let assign34690_e44277: f64 = (var_perfc * var_xerfc);
        let assign34690_e44278: f64 = (1.0 - assign34690_e44277);
        let assign34690_e44279: f64 = (1.0 / assign34690_e44278);
        (assign34690_e44279, (-((-(var_perfc * var_xerfc_dn5)) / (assign34690_e44278 * assign34690_e44278))), (-((-(var_perfc * var_xerfc_dn6)) / (assign34690_e44278 * assign34690_e44278))), (-((-(var_perfc * var_xerfc_dn7)) / (assign34690_e44278 * assign34690_e44278))), (-((-(var_perfc * var_xerfc_dn8)) / (assign34690_e44278 * assign34690_e44278))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign34690_e44281;
        var_terfc_dn5 = assign34690_e44281_d_n5;
        var_terfc_dn6 = assign34690_e44281_d_n6;
        var_terfc_dn7 = assign34690_e44281_d_n7;
        var_terfc_dn8 = assign34690_e44281_d_n8;

        let assign34700_e44283: f64 = (-var_ysq);
        let assign34700_e44285: f64 = (assign34700_e44283 + var_mtat);
        let assign34700_e44287: f64 = (-230.25850929940458);
        let assign34700_e44288: f64 = if assign34700_e44285 > assign34700_e44287 { 1.0 } else { 0.0 };
        var_guard690 = assign34700_e44288;

        let (assign34710_e44306, assign34710_e44306_d_n5, assign34710_e44306_d_n6, assign34710_e44306_d_n7, assign34710_e44306_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) && (var_guard690 != 0.0)) {
        let assign34710_e44301: f64 = (-var_ysq);
        let assign34710_e44303: f64 = (assign34710_e44301 + var_mtat);
        let assign34710_e44304: f64 = (assign34710_e44303).exp();
        (assign34710_e44304, (assign34710_e44304 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign34710_e44304 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign34710_e44304 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign34710_e44304 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34710_e44306;
        var_tmp_dn5 = assign34710_e44306_d_n5;
        var_tmp_dn6 = assign34710_e44306_d_n6;
        var_tmp_dn7 = assign34710_e44306_d_n7;
        var_tmp_dn8 = assign34710_e44306_d_n8;

        let (assign34720_e44355, assign34720_e44355_d_n5, assign34720_e44355_d_n6, assign34720_e44355_d_n7, assign34720_e44355_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) && (var_guard690 == 0.0)) {
        let assign34720_e44322: f64 = (-230.25850929940458);
        let assign34720_e44324: f64 = (-var_ysq);
        let assign34720_e44326: f64 = (assign34720_e44324 + var_mtat);
        let assign34720_e44327: f64 = (assign34720_e44322 - assign34720_e44326);
        let assign34720_e44331: f64 = (-230.25850929940458);
        let assign34720_e44333: f64 = (-var_ysq);
        let assign34720_e44335: f64 = (assign34720_e44333 + var_mtat);
        let assign34720_e44336: f64 = (assign34720_e44331 - assign34720_e44335);
        let assign34720_e44339: f64 = (-230.25850929940458);
        let assign34720_e44341: f64 = (-var_ysq);
        let assign34720_e44343: f64 = (assign34720_e44341 + var_mtat);
        let assign34720_e44344: f64 = (assign34720_e44339 - assign34720_e44343);
        let assign34720_e44346: f64 = (assign34720_e44344 * 0.3333333333333333);
        let assign34720_e44347: f64 = (1.0 + assign34720_e44346);
        let assign34720_e44348: f64 = (assign34720_e44336 * assign34720_e44347);
        let assign34720_e44349: f64 = (0.5 * assign34720_e44348);
        let assign34720_e44350: f64 = (1.0 + assign34720_e44349);
        let assign34720_e44351: f64 = (assign34720_e44327 * assign34720_e44350);
        let assign34720_e44352: f64 = (1.0 + assign34720_e44351);
        let assign34720_e44353: f64 = (1e-100 / assign34720_e44352);
        (assign34720_e44353, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign34720_e44350) + (assign34720_e44327 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign34720_e44347) + (assign34720_e44336 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign34720_e44352 * assign34720_e44352))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign34720_e44350) + (assign34720_e44327 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign34720_e44347) + (assign34720_e44336 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign34720_e44352 * assign34720_e44352))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign34720_e44350) + (assign34720_e44327 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign34720_e44347) + (assign34720_e44336 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign34720_e44352 * assign34720_e44352))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign34720_e44350) + (assign34720_e44327 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign34720_e44347) + (assign34720_e44336 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign34720_e44352 * assign34720_e44352))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34720_e44355;
        var_tmp_dn5 = assign34720_e44355_d_n5;
        var_tmp_dn6 = assign34720_e44355_d_n6;
        var_tmp_dn7 = assign34720_e44355_d_n7;
        var_tmp_dn8 = assign34720_e44355_d_n8;

        let (assign34730_e44385, assign34730_e44385_d_n5, assign34730_e44385_d_n6, assign34730_e44385_d_n7, assign34730_e44385_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34730_e44367: f64 = (0.29214664 * var_terfc);
        let assign34730_e44371: f64 = (var_terfc * var_terfc);
        let assign34730_e44372: f64 = (var_berfc * assign34730_e44371);
        let assign34730_e44373: f64 = (assign34730_e44367 + assign34730_e44372);
        let assign34730_e44377: f64 = (var_terfc * var_terfc);
        let assign34730_e44379: f64 = (assign34730_e44377 * var_terfc);
        let assign34730_e44380: f64 = (var_cerfc * assign34730_e44379);
        let assign34730_e44381: f64 = (assign34730_e44373 + assign34730_e44380);
        let assign34730_e44383: f64 = (assign34730_e44381 * var_tmp);
        (assign34730_e44383, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign34730_e44377 * var_terfc_dn5)))) * var_tmp) + (assign34730_e44381 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign34730_e44377 * var_terfc_dn6)))) * var_tmp) + (assign34730_e44381 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign34730_e44377 * var_terfc_dn7)))) * var_tmp) + (assign34730_e44381 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign34730_e44377 * var_terfc_dn8)))) * var_tmp) + (assign34730_e44381 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign34730_e44385;
        var_erfcpos_dn5 = assign34730_e44385_d_n5;
        var_erfcpos_dn6 = assign34730_e44385_d_n6;
        var_erfcpos_dn7 = assign34730_e44385_d_n7;
        var_erfcpos_dn8 = assign34730_e44385_d_n8;

        let assign34740_e44388: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard691 = assign34740_e44388;

        let (assign34750_e44402, assign34750_e44402_d_n5, assign34750_e44402_d_n6, assign34750_e44402_d_n7, assign34750_e44402_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) && (var_guard691 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign34750_e44402;
        var_erfctimesexpmtat_dn5 = assign34750_e44402_d_n5;
        var_erfctimesexpmtat_dn6 = assign34750_e44402_d_n6;
        var_erfctimesexpmtat_dn7 = assign34750_e44402_d_n7;
        var_erfctimesexpmtat_dn8 = assign34750_e44402_d_n8;

        let assign34760_e44405: f64 = (-230.25850929940458);
        let assign34760_e44406: f64 = if var_mtat > assign34760_e44405 { 1.0 } else { 0.0 };
        var_guard692 = assign34760_e44406;

        let (assign34770_e44424, assign34770_e44424_d_n5, assign34770_e44424_d_n6, assign34770_e44424_d_n7, assign34770_e44424_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) && (var_guard691 == 0.0)) && (var_guard692 != 0.0)) {
        let assign34770_e44422: f64 = (var_mtat).exp();
        (assign34770_e44422, (assign34770_e44422 * var_mtat_dn5), (assign34770_e44422 * var_mtat_dn6), (assign34770_e44422 * var_mtat_dn7), (assign34770_e44422 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34770_e44424;
        var_tmp_dn5 = assign34770_e44424_d_n5;
        var_tmp_dn6 = assign34770_e44424_d_n6;
        var_tmp_dn7 = assign34770_e44424_d_n7;
        var_tmp_dn8 = assign34770_e44424_d_n8;

        let (assign34780_e44467, assign34780_e44467_d_n5, assign34780_e44467_d_n6, assign34780_e44467_d_n7, assign34780_e44467_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) && (var_guard691 == 0.0)) && (var_guard692 == 0.0)) {
        let assign34780_e44443: f64 = (-230.25850929940458);
        let assign34780_e44445: f64 = (assign34780_e44443 - var_mtat);
        let assign34780_e44449: f64 = (-230.25850929940458);
        let assign34780_e44451: f64 = (assign34780_e44449 - var_mtat);
        let assign34780_e44454: f64 = (-230.25850929940458);
        let assign34780_e44456: f64 = (assign34780_e44454 - var_mtat);
        let assign34780_e44458: f64 = (assign34780_e44456 * 0.3333333333333333);
        let assign34780_e44459: f64 = (1.0 + assign34780_e44458);
        let assign34780_e44460: f64 = (assign34780_e44451 * assign34780_e44459);
        let assign34780_e44461: f64 = (0.5 * assign34780_e44460);
        let assign34780_e44462: f64 = (1.0 + assign34780_e44461);
        let assign34780_e44463: f64 = (assign34780_e44445 * assign34780_e44462);
        let assign34780_e44464: f64 = (1.0 + assign34780_e44463);
        let assign34780_e44465: f64 = (1e-100 / assign34780_e44464);
        (assign34780_e44465, (-((1e-100 * (((-var_mtat_dn5) * assign34780_e44462) + (assign34780_e44445 * (0.5 * (((-var_mtat_dn5) * assign34780_e44459) + (assign34780_e44451 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign34780_e44464 * assign34780_e44464))), (-((1e-100 * (((-var_mtat_dn6) * assign34780_e44462) + (assign34780_e44445 * (0.5 * (((-var_mtat_dn6) * assign34780_e44459) + (assign34780_e44451 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign34780_e44464 * assign34780_e44464))), (-((1e-100 * (((-var_mtat_dn7) * assign34780_e44462) + (assign34780_e44445 * (0.5 * (((-var_mtat_dn7) * assign34780_e44459) + (assign34780_e44451 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign34780_e44464 * assign34780_e44464))), (-((1e-100 * (((-var_mtat_dn8) * assign34780_e44462) + (assign34780_e44445 * (0.5 * (((-var_mtat_dn8) * assign34780_e44459) + (assign34780_e44451 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign34780_e44464 * assign34780_e44464))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34780_e44467;
        var_tmp_dn5 = assign34780_e44467_d_n5;
        var_tmp_dn6 = assign34780_e44467_d_n6;
        var_tmp_dn7 = assign34780_e44467_d_n7;
        var_tmp_dn8 = assign34780_e44467_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_guard686_slot = var_guard686;
        *var_guard687_slot = var_guard687;
        *var_guard688_slot = var_guard688;
        *var_guard689_slot = var_guard689;
        *var_guard690_slot = var_guard690;
        *var_guard691_slot = var_guard691;
        *var_guard692_slot = var_guard692;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_72(
        p: &Parameters,
        var_abdrain_i: f64,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat_d: f64,
        var_cbbtgatd_i: f64,
        var_ctatgatd_i: f64,
        var_erfcpos: f64,
        var_erfcpos_dn5: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn5: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fstopgat_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard683: f64,
        var_guard687: f64,
        var_guard691: f64,
        var_id__blk213: f64,
        var_ijunbot: f64,
        var_ijunbot_dn5: f64,
        var_ijunbot_dn6: f64,
        var_ijunbot_dn7: f64,
        var_ijunbot_dn8: f64,
        var_ijunsti: f64,
        var_ijunsti_dn5: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_isrh: f64,
        var_isrh_dn5: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
        var_pgatd_i: f64,
        var_phitdinv: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn5: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_v3: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_vbrgatd_i: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn5: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_vmax_d: f64,
        var_wdepnulrinvgat_d: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard693_slot: &mut f64,
        var_guard694_slot: &mut f64,
        var_guard695_slot: &mut f64,
        var_guard696_slot: &mut f64,
        var_guard697_slot: &mut f64,
        var_guard698_slot: &mut f64,
        var_guard699_slot: &mut f64,
        var_guard700_slot: &mut f64,
        var_guard701_slot: &mut f64,
        var_guard702_slot: &mut f64,
        var_guard703_slot: &mut f64,
        var_i3_slot: &mut f64,
        var_i3_dn5_slot: &mut f64,
        var_i3_dn6_slot: &mut f64,
        var_i3_dn7_slot: &mut f64,
        var_i3_dn8_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard693: f64 = *var_guard693_slot;
        let mut var_guard694: f64 = *var_guard694_slot;
        let mut var_guard695: f64 = *var_guard695_slot;
        let mut var_guard696: f64 = *var_guard696_slot;
        let mut var_guard697: f64 = *var_guard697_slot;
        let mut var_guard698: f64 = *var_guard698_slot;
        let mut var_guard699: f64 = *var_guard699_slot;
        let mut var_guard700: f64 = *var_guard700_slot;
        let mut var_guard701: f64 = *var_guard701_slot;
        let mut var_guard702: f64 = *var_guard702_slot;
        let mut var_guard703: f64 = *var_guard703_slot;
        let mut var_i3: f64 = *var_i3_slot;
        let mut var_i3_dn5: f64 = *var_i3_dn5_slot;
        let mut var_i3_dn6: f64 = *var_i3_dn6_slot;
        let mut var_i3_dn7: f64 = *var_i3_dn7_slot;
        let mut var_i3_dn8: f64 = *var_i3_dn8_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign34790_e44486, assign34790_e44486_d_n5, assign34790_e44486_d_n6, assign34790_e44486_d_n7, assign34790_e44486_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) && (var_guard691 == 0.0)) {
        let assign34790_e44482: f64 = (2.0 * var_tmp);
        let assign34790_e44484: f64 = (assign34790_e44482 - var_erfcpos);
        (assign34790_e44484, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign34790_e44486;
        var_erfctimesexpmtat_dn5 = assign34790_e44486_d_n5;
        var_erfctimesexpmtat_dn6 = assign34790_e44486_d_n6;
        var_erfctimesexpmtat_dn7 = assign34790_e44486_d_n7;
        var_erfctimesexpmtat_dn8 = assign34790_e44486_d_n8;

        let (assign34800_e44506, assign34800_e44506_d_n5, assign34800_e44506_d_n6, assign34800_e44506_d_n7, assign34800_e44506_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34800_e44498: f64 = (1.772453850905516 * 0.5);
        let assign34800_e44501: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign34800_e44503: f64 = (assign34800_e44501 / var_ktat);
        let assign34800_e44504: f64 = (assign34800_e44498 * assign34800_e44503);
        (assign34800_e44504, (assign34800_e44498 * ((((var_atatgat_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign34800_e44501 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign34800_e44498 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign34800_e44501 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign34800_e44498 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign34800_e44501 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign34800_e44498 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign34800_e44501 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign34800_e44506;
        var_gammamax_dn5 = assign34800_e44506_d_n5;
        var_gammamax_dn6 = assign34800_e44506_d_n6;
        var_gammamax_dn7 = assign34800_e44506_d_n7;
        var_gammamax_dn8 = assign34800_e44506_d_n8;

        let (assign34810_e44524, assign34810_e44524_d_n5, assign34810_e44524_d_n6, assign34810_e44524_d_n7, assign34810_e44524_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34810_e44519: f64 = (var_asrh * var_gammamax);
        let assign34810_e44521: f64 = (assign34810_e44519 * var_wtat);
        let assign34810_e44522: f64 = (var_ctatgatd_i * assign34810_e44521);
        (assign34810_e44522, (var_ctatgatd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign34810_e44519 * var_wtat_dn5))), (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign34810_e44519 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign34810_e44519 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign34810_e44519 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign34810_e44524;
        var_itat_dn5 = assign34810_e44524_d_n5;
        var_itat_dn6 = assign34810_e44524_d_n6;
        var_itat_dn7 = assign34810_e44524_d_n7;
        var_itat_dn8 = assign34810_e44524_d_n8;

        let assign34820_e44527: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard693 = assign34820_e44527;

        let (assign34830_e44538, assign34830_e44538_d_n5, assign34830_e44538_d_n6, assign34830_e44538_d_n7, assign34830_e44538_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard693 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign34830_e44538;
        var_ibbt_dn5 = assign34830_e44538_d_n5;
        var_ibbt_dn6 = assign34830_e44538_d_n6;
        var_ibbt_dn7 = assign34830_e44538_d_n7;
        var_ibbt_dn8 = assign34830_e44538_d_n8;

        let assign34840_e44541: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard694 = assign34840_e44541;

        let (assign34850_e44560, assign34850_e44560_d_n5, assign34850_e44560_d_n6, assign34850_e44560_d_n7, assign34850_e44560_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard693 == 0.0)) && (var_guard694 != 0.0)) {
        let assign34850_e44555: f64 = (var_vbirgatd_i - var_vbbt);
        let assign34850_e44557: f64 = (assign34850_e44555 * var_vbirgatinv_d);
        let assign34850_e44558: f64 = (assign34850_e44557).sqrt();
        (assign34850_e44558, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34850_e44560;
        var_tmp_dn5 = assign34850_e44560_d_n5;
        var_tmp_dn6 = assign34850_e44560_d_n6;
        var_tmp_dn7 = assign34850_e44560_d_n7;
        var_tmp_dn8 = assign34850_e44560_d_n8;

        let (assign34860_e44581, assign34860_e44581_d_n5, assign34860_e44581_d_n6, assign34860_e44581_d_n7, assign34860_e44581_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard693 == 0.0)) && (var_guard694 == 0.0)) {
        let assign34860_e44575: f64 = (var_vbirgatd_i - var_vbbt);
        let assign34860_e44577: f64 = (assign34860_e44575 * var_vbirgatinv_d);
        let assign34860_e44579: f64 = (assign34860_e44577).powf(var_pgatd_i);
        (assign34860_e44579, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34860_e44581;
        var_tmp_dn5 = assign34860_e44581_d_n5;
        var_tmp_dn6 = assign34860_e44581_d_n6;
        var_tmp_dn7 = assign34860_e44581_d_n7;
        var_tmp_dn8 = assign34860_e44581_d_n8;

        let (assign34870_e44601, assign34870_e44601_d_n5, assign34870_e44601_d_n6, assign34870_e44601_d_n7, assign34870_e44601_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34870_e44594: f64 = (var_vbirgatd_i - var_vbbt);
        let assign34870_e44596: f64 = (assign34870_e44594 * var_wdepnulrinvgat_d);
        let assign34870_e44598: f64 = (assign34870_e44596 / var_tmp);
        let assign34870_e44599: f64 = (var_one_over_one_minus_pgat_d * assign34870_e44598);
        (assign34870_e44599, (var_one_over_one_minus_pgat_d * (-((assign34870_e44596 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign34870_e44596 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign34870_e44596 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign34870_e44596 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign34870_e44601;
        var_fmaxr_dn5 = assign34870_e44601_d_n5;
        var_fmaxr_dn6 = assign34870_e44601_d_n6;
        var_fmaxr_dn7 = assign34870_e44601_d_n7;
        var_fmaxr_dn8 = assign34870_e44601_d_n8;

        let assign34880_e44603: f64 = (-var_fbbtgat_d);
        let assign34880_e44605: f64 = (assign34880_e44603 / var_fmaxr);
        let assign34880_e44606: f64 = (assign34880_e44605).abs();
        let assign34880_e44608: f64 = if assign34880_e44606 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard695 = assign34880_e44608;

        let (assign34890_e44626, assign34890_e44626_d_n5, assign34890_e44626_d_n6, assign34890_e44626_d_n7, assign34890_e44626_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard693 == 0.0)) && (var_guard695 != 0.0)) {
        let assign34890_e44621: f64 = (-var_fbbtgat_d);
        let assign34890_e44623: f64 = (assign34890_e44621 / var_fmaxr);
        let assign34890_e44624: f64 = (assign34890_e44623).exp();
        (assign34890_e44624, (assign34890_e44624 * ((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign34890_e44621 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign34890_e44624 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34890_e44621 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign34890_e44624 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34890_e44621 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign34890_e44624 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34890_e44621 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34890_e44626;
        var_tmp_dn5 = assign34890_e44626_d_n5;
        var_tmp_dn6 = assign34890_e44626_d_n6;
        var_tmp_dn7 = assign34890_e44626_d_n7;
        var_tmp_dn8 = assign34890_e44626_d_n8;

        let assign34900_e44628: f64 = (-var_fbbtgat_d);
        let assign34900_e44630: f64 = (assign34900_e44628 / var_fmaxr);
        let assign34900_e44632: f64 = if assign34900_e44630 < 0.0 { 1.0 } else { 0.0 };
        var_guard696 = assign34900_e44632;

        let (assign34910_e44683, assign34910_e44683_d_n5, assign34910_e44683_d_n6, assign34910_e44683_d_n7, assign34910_e44683_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard693 == 0.0)) && (var_guard695 == 0.0)) && (var_guard696 != 0.0)) {
        let assign34910_e44650: f64 = (-230.25850929940458);
        let assign34910_e44652: f64 = (-var_fbbtgat_d);
        let assign34910_e44654: f64 = (assign34910_e44652 / var_fmaxr);
        let assign34910_e44655: f64 = (assign34910_e44650 - assign34910_e44654);
        let assign34910_e44659: f64 = (-230.25850929940458);
        let assign34910_e44661: f64 = (-var_fbbtgat_d);
        let assign34910_e44663: f64 = (assign34910_e44661 / var_fmaxr);
        let assign34910_e44664: f64 = (assign34910_e44659 - assign34910_e44663);
        let assign34910_e44667: f64 = (-230.25850929940458);
        let assign34910_e44669: f64 = (-var_fbbtgat_d);
        let assign34910_e44671: f64 = (assign34910_e44669 / var_fmaxr);
        let assign34910_e44672: f64 = (assign34910_e44667 - assign34910_e44671);
        let assign34910_e44674: f64 = (assign34910_e44672 * 0.3333333333333333);
        let assign34910_e44675: f64 = (1.0 + assign34910_e44674);
        let assign34910_e44676: f64 = (assign34910_e44664 * assign34910_e44675);
        let assign34910_e44677: f64 = (0.5 * assign34910_e44676);
        let assign34910_e44678: f64 = (1.0 + assign34910_e44677);
        let assign34910_e44679: f64 = (assign34910_e44655 * assign34910_e44678);
        let assign34910_e44680: f64 = (1.0 + assign34910_e44679);
        let assign34910_e44681: f64 = (1e-100 / assign34910_e44680);
        (assign34910_e44681, (-((1e-100 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign34910_e44652 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign34910_e44678) + (assign34910_e44655 * (0.5 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign34910_e44661 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign34910_e44675) + (assign34910_e44664 * ((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign34910_e44669 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign34910_e44680 * assign34910_e44680))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34910_e44652 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign34910_e44678) + (assign34910_e44655 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34910_e44661 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign34910_e44675) + (assign34910_e44664 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34910_e44669 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign34910_e44680 * assign34910_e44680))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34910_e44652 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign34910_e44678) + (assign34910_e44655 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34910_e44661 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign34910_e44675) + (assign34910_e44664 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34910_e44669 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign34910_e44680 * assign34910_e44680))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34910_e44652 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign34910_e44678) + (assign34910_e44655 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34910_e44661 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign34910_e44675) + (assign34910_e44664 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34910_e44669 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign34910_e44680 * assign34910_e44680))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34910_e44683;
        var_tmp_dn5 = assign34910_e44683_d_n5;
        var_tmp_dn6 = assign34910_e44683_d_n6;
        var_tmp_dn7 = assign34910_e44683_d_n7;
        var_tmp_dn8 = assign34910_e44683_d_n8;

        let (assign34920_e44732, assign34920_e44732_d_n5, assign34920_e44732_d_n6, assign34920_e44732_d_n7, assign34920_e44732_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard693 == 0.0)) && (var_guard695 == 0.0)) && (var_guard696 == 0.0)) {
        let assign34920_e44702: f64 = (-var_fbbtgat_d);
        let assign34920_e44704: f64 = (assign34920_e44702 / var_fmaxr);
        let assign34920_e44706: f64 = (assign34920_e44704 - 230.25850929940458);
        let assign34920_e44710: f64 = (-var_fbbtgat_d);
        let assign34920_e44712: f64 = (assign34920_e44710 / var_fmaxr);
        let assign34920_e44714: f64 = (assign34920_e44712 - 230.25850929940458);
        let assign34920_e44717: f64 = (-var_fbbtgat_d);
        let assign34920_e44719: f64 = (assign34920_e44717 / var_fmaxr);
        let assign34920_e44721: f64 = (assign34920_e44719 - 230.25850929940458);
        let assign34920_e44723: f64 = (assign34920_e44721 * 0.3333333333333333);
        let assign34920_e44724: f64 = (1.0 + assign34920_e44723);
        let assign34920_e44725: f64 = (assign34920_e44714 * assign34920_e44724);
        let assign34920_e44726: f64 = (0.5 * assign34920_e44725);
        let assign34920_e44727: f64 = (1.0 + assign34920_e44726);
        let assign34920_e44728: f64 = (assign34920_e44706 * assign34920_e44727);
        let assign34920_e44729: f64 = (1.0 + assign34920_e44728);
        let assign34920_e44730: f64 = (1e100 * assign34920_e44729);
        (assign34920_e44730, (1e100 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign34920_e44702 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign34920_e44727) + (assign34920_e44706 * (0.5 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign34920_e44710 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign34920_e44724) + (assign34920_e44714 * (((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign34920_e44717 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34920_e44702 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign34920_e44727) + (assign34920_e44706 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34920_e44710 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign34920_e44724) + (assign34920_e44714 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34920_e44717 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34920_e44702 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign34920_e44727) + (assign34920_e44706 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34920_e44710 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign34920_e44724) + (assign34920_e44714 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34920_e44717 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34920_e44702 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign34920_e44727) + (assign34920_e44706 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34920_e44710 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign34920_e44724) + (assign34920_e44714 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34920_e44717 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34920_e44732;
        var_tmp_dn5 = assign34920_e44732_d_n5;
        var_tmp_dn6 = assign34920_e44732_d_n6;
        var_tmp_dn7 = assign34920_e44732_d_n7;
        var_tmp_dn8 = assign34920_e44732_d_n8;

        let (assign34930_e44752, assign34930_e44752_d_n5, assign34930_e44752_d_n6, assign34930_e44752_d_n7, assign34930_e44752_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34930_e44745: f64 = (var_v3 * var_fmaxr);
        let assign34930_e44747: f64 = (assign34930_e44745 * var_fmaxr);
        let assign34930_e44749: f64 = (assign34930_e44747 * var_tmp);
        let assign34930_e44750: f64 = (var_cbbtgatd_i * assign34930_e44749);
        (assign34930_e44750, (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign34930_e44745 * var_fmaxr_dn5)) * var_tmp) + (assign34930_e44747 * var_tmp_dn5))), (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign34930_e44745 * var_fmaxr_dn6)) * var_tmp) + (assign34930_e44747 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign34930_e44745 * var_fmaxr_dn7)) * var_tmp) + (assign34930_e44747 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign34930_e44745 * var_fmaxr_dn8)) * var_tmp) + (assign34930_e44747 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign34930_e44752;
        var_ibbt_dn5 = assign34930_e44752_d_n5;
        var_ibbt_dn6 = assign34930_e44752_d_n6;
        var_ibbt_dn7 = assign34930_e44752_d_n7;
        var_ibbt_dn8 = assign34930_e44752_d_n8;

        let assign34940_e44755: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard697 = assign34940_e44755;

        let (assign34950_e44766, assign34950_e44766_d_n5, assign34950_e44766_d_n6, assign34950_e44766_d_n7, assign34950_e44766_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard697 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign34950_e44766;
        var_fbreakdown_dn5 = assign34950_e44766_d_n5;
        var_fbreakdown_dn6 = assign34950_e44766_d_n6;
        var_fbreakdown_dn7 = assign34950_e44766_d_n7;
        var_fbreakdown_dn8 = assign34950_e44766_d_n8;

        let assign34960_e44769: f64 = (-var_alphaav);
        let assign34960_e44771: f64 = (assign34960_e44769 * var_vbrgatd_i);
        let assign34960_e44772: f64 = if var_vav > assign34960_e44771 { 1.0 } else { 0.0 };
        var_guard698 = assign34960_e44772;

        let assign34970_e44775: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard699 = assign34970_e44775;

        let (assign34980_e44805, assign34980_e44805_d_n5, assign34980_e44805_d_n6, assign34980_e44805_d_n7, assign34980_e44805_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard697 == 0.0)) && (var_guard698 != 0.0)) && (var_guard699 != 0.0)) {
        let assign34980_e44791: f64 = (var_vav * var_vbrinvgat_d);
        let assign34980_e44794: f64 = (var_vav * var_vbrinvgat_d);
        let assign34980_e44795: f64 = (assign34980_e44791 * assign34980_e44794);
        let assign34980_e44798: f64 = (var_vav * var_vbrinvgat_d);
        let assign34980_e44799: f64 = (assign34980_e44795 * assign34980_e44798);
        let assign34980_e44802: f64 = (var_vav * var_vbrinvgat_d);
        let assign34980_e44803: f64 = (assign34980_e44799 * assign34980_e44802);
        (assign34980_e44803, (((((((var_vav * var_vbrinvgat_d_dn5) * assign34980_e44794) + (assign34980_e44791 * (var_vav * var_vbrinvgat_d_dn5))) * assign34980_e44798) + (assign34980_e44795 * (var_vav * var_vbrinvgat_d_dn5))) * assign34980_e44802) + (assign34980_e44799 * (var_vav * var_vbrinvgat_d_dn5))), (((((((var_vav * var_vbrinvgat_d_dn6) * assign34980_e44794) + (assign34980_e44791 * (var_vav * var_vbrinvgat_d_dn6))) * assign34980_e44798) + (assign34980_e44795 * (var_vav * var_vbrinvgat_d_dn6))) * assign34980_e44802) + (assign34980_e44799 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign34980_e44794) + (assign34980_e44791 * (var_vav * var_vbrinvgat_d_dn7))) * assign34980_e44798) + (assign34980_e44795 * (var_vav * var_vbrinvgat_d_dn7))) * assign34980_e44802) + (assign34980_e44799 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign34980_e44794) + (assign34980_e44791 * (var_vav * var_vbrinvgat_d_dn8))) * assign34980_e44798) + (assign34980_e44795 * (var_vav * var_vbrinvgat_d_dn8))) * assign34980_e44802) + (assign34980_e44799 * (var_vav * var_vbrinvgat_d_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34980_e44805;
        var_tmp_dn5 = assign34980_e44805_d_n5;
        var_tmp_dn6 = assign34980_e44805_d_n6;
        var_tmp_dn7 = assign34980_e44805_d_n7;
        var_tmp_dn8 = assign34980_e44805_d_n8;

        let (assign34990_e44827, assign34990_e44827_d_n5, assign34990_e44827_d_n6, assign34990_e44827_d_n7, assign34990_e44827_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard697 == 0.0)) && (var_guard698 != 0.0)) && (var_guard699 == 0.0)) {
        let assign34990_e44822: f64 = (var_vav * var_vbrinvgat_d);
        let assign34990_e44823: f64 = (assign34990_e44822).abs();
        let assign34990_e44825: f64 = (assign34990_e44823).powf(var_pbrgatd_i);
        (assign34990_e44825, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign34990_e44823).powf(var_pbrgatd_i - 1.0) * if assign34990_e44822 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) })) } } else { (assign34990_e44825 * (var_pbrgatd_i * (if assign34990_e44822 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) } / assign34990_e44823))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign34990_e44823).powf(var_pbrgatd_i - 1.0) * if assign34990_e44822 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign34990_e44825 * (var_pbrgatd_i * (if assign34990_e44822 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign34990_e44823))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign34990_e44823).powf(var_pbrgatd_i - 1.0) * if assign34990_e44822 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign34990_e44825 * (var_pbrgatd_i * (if assign34990_e44822 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign34990_e44823))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign34990_e44823).powf(var_pbrgatd_i - 1.0) * if assign34990_e44822 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign34990_e44825 * (var_pbrgatd_i * (if assign34990_e44822 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign34990_e44823))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34990_e44827;
        var_tmp_dn5 = assign34990_e44827_d_n5;
        var_tmp_dn6 = assign34990_e44827_d_n6;
        var_tmp_dn7 = assign34990_e44827_d_n7;
        var_tmp_dn8 = assign34990_e44827_d_n8;

        let (assign35000_e44845, assign35000_e44845_d_n5, assign35000_e44845_d_n6, assign35000_e44845_d_n7, assign35000_e44845_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard697 == 0.0)) && (var_guard698 != 0.0)) {
        let assign35000_e44842: f64 = (1.0 - var_tmp);
        let assign35000_e44843: f64 = (1.0 / assign35000_e44842);
        (assign35000_e44843, (-((-var_tmp_dn5) / (assign35000_e44842 * assign35000_e44842))), (-((-var_tmp_dn6) / (assign35000_e44842 * assign35000_e44842))), (-((-var_tmp_dn7) / (assign35000_e44842 * assign35000_e44842))), (-((-var_tmp_dn8) / (assign35000_e44842 * assign35000_e44842))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign35000_e44845;
        var_fbreakdown_dn5 = assign35000_e44845_d_n5;
        var_fbreakdown_dn6 = assign35000_e44845_d_n6;
        var_fbreakdown_dn7 = assign35000_e44845_d_n7;
        var_fbreakdown_dn8 = assign35000_e44845_d_n8;

        let (assign35010_e44868, assign35010_e44868_d_n5, assign35010_e44868_d_n6, assign35010_e44868_d_n7, assign35010_e44868_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) && (var_guard697 == 0.0)) && (var_guard698 == 0.0)) {
        let assign35010_e44862: f64 = (var_alphaav * var_vbrgatd_i);
        let assign35010_e44863: f64 = (var_vav + assign35010_e44862);
        let assign35010_e44865: f64 = (assign35010_e44863 * var_slopegat_d);
        let assign35010_e44866: f64 = (var_fstopgat_d + assign35010_e44865);
        (assign35010_e44866, (assign35010_e44863 * var_slopegat_d_dn5), (assign35010_e44863 * var_slopegat_d_dn6), (assign35010_e44863 * var_slopegat_d_dn7), (assign35010_e44863 * var_slopegat_d_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign35010_e44868;
        var_fbreakdown_dn5 = assign35010_e44868_d_n5;
        var_fbreakdown_dn6 = assign35010_e44868_d_n6;
        var_fbreakdown_dn7 = assign35010_e44868_d_n7;
        var_fbreakdown_dn8 = assign35010_e44868_d_n8;

        let (assign35020_e44887, assign35020_e44887_d_n5, assign35020_e44887_d_n6, assign35020_e44887_d_n7, assign35020_e44887_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard683 == 0.0)) {
        let assign35020_e44878: f64 = (var_id__blk213 + var_isrh);
        let assign35020_e44880: f64 = (assign35020_e44878 + var_itat);
        let assign35020_e44882: f64 = (assign35020_e44880 + var_ibbt);
        let assign35020_e44883: f64 = (p.p29 * assign35020_e44882);
        let assign35020_e44885: f64 = (assign35020_e44883 * var_fbreakdown);
        (assign35020_e44885, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign35020_e44883 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign35020_e44883 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign35020_e44883 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign35020_e44883 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign35020_e44887;
        var_ijungat_dn5 = assign35020_e44887_d_n5;
        var_ijungat_dn6 = assign35020_e44887_d_n6;
        var_ijungat_dn7 = assign35020_e44887_d_n7;
        var_ijungat_dn8 = assign35020_e44887_d_n8;

        let (assign35030_e44903, assign35030_e44903_d_n5, assign35030_e44903_d_n6, assign35030_e44903_d_n7, assign35030_e44903_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign35030_e44893: f64 = (var_abdrain_i * var_ijunbot);
        let assign35030_e44896: f64 = (var_lsdrain_i * var_ijunsti);
        let assign35030_e44897: f64 = (assign35030_e44893 + assign35030_e44896);
        let assign35030_e44900: f64 = (var_lgdrain_i * var_ijungat);
        let assign35030_e44901: f64 = (assign35030_e44897 + assign35030_e44900);
        (assign35030_e44901, (((var_abdrain_i * var_ijunbot_dn5) + (var_lsdrain_i * var_ijunsti_dn5)) + (var_lgdrain_i * var_ijungat_dn5)), (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)),)
    } else {
        (var_i3, var_i3_dn5, var_i3_dn6, var_i3_dn7, var_i3_dn8,)
    }
};
        var_i3 = assign35030_e44903;
        var_i3_dn5 = assign35030_e44903_d_n5;
        var_i3_dn6 = assign35030_e44903_d_n6;
        var_i3_dn7 = assign35030_e44903_d_n7;
        var_i3_dn8 = assign35030_e44903_d_n8;

        let (assign35040_e44909,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign35040_e44909;

        let (assign35050_e44915,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign35050_e44915;

        let assign35060_e44927: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard700 = assign35060_e44927;

        let assign35140_e45013: f64 = if var_v4 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard701 = assign35140_e45013;

        let assign35150_e45015: f64 = (-0.5);
        let assign35150_e45018: f64 = (var_v4 * var_phitdinv);
        let assign35150_e45019: f64 = (assign35150_e45015 * assign35150_e45018);
        let assign35150_e45020: f64 = (assign35150_e45019).abs();
        let assign35150_e45022: f64 = if assign35150_e45020 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard702 = assign35150_e45022;

        let (assign35160_e45040,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 != 0.0)) && (var_guard702 != 0.0)) {
        let assign35160_e45033: f64 = (-0.5);
        let assign35160_e45036: f64 = (var_v4 * var_phitdinv);
        let assign35160_e45037: f64 = (assign35160_e45033 * assign35160_e45036);
        let assign35160_e45038: f64 = (assign35160_e45037).exp();
        (assign35160_e45038,)
    } else {
        (var_z,)
    }
};
        var_z = assign35160_e45040;

        let assign35170_e45042: f64 = (-0.5);
        let assign35170_e45045: f64 = (var_v4 * var_phitdinv);
        let assign35170_e45046: f64 = (assign35170_e45042 * assign35170_e45045);
        let assign35170_e45048: f64 = if assign35170_e45046 < 0.0 { 1.0 } else { 0.0 };
        var_guard703 = assign35170_e45048;

        let (assign35180_e45103,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 != 0.0)) && (var_guard702 == 0.0)) && (var_guard703 != 0.0)) {
        let assign35180_e45064: f64 = (-230.25850929940458);
        let assign35180_e45066: f64 = (-0.5);
        let assign35180_e45069: f64 = (var_v4 * var_phitdinv);
        let assign35180_e45070: f64 = (assign35180_e45066 * assign35180_e45069);
        let assign35180_e45071: f64 = (assign35180_e45064 - assign35180_e45070);
        let assign35180_e45075: f64 = (-230.25850929940458);
        let assign35180_e45077: f64 = (-0.5);
        let assign35180_e45080: f64 = (var_v4 * var_phitdinv);
        let assign35180_e45081: f64 = (assign35180_e45077 * assign35180_e45080);
        let assign35180_e45082: f64 = (assign35180_e45075 - assign35180_e45081);
        let assign35180_e45085: f64 = (-230.25850929940458);
        let assign35180_e45087: f64 = (-0.5);
        let assign35180_e45090: f64 = (var_v4 * var_phitdinv);
        let assign35180_e45091: f64 = (assign35180_e45087 * assign35180_e45090);
        let assign35180_e45092: f64 = (assign35180_e45085 - assign35180_e45091);
        let assign35180_e45094: f64 = (assign35180_e45092 * 0.3333333333333333);
        let assign35180_e45095: f64 = (1.0 + assign35180_e45094);
        let assign35180_e45096: f64 = (assign35180_e45082 * assign35180_e45095);
        let assign35180_e45097: f64 = (0.5 * assign35180_e45096);
        let assign35180_e45098: f64 = (1.0 + assign35180_e45097);
        let assign35180_e45099: f64 = (assign35180_e45071 * assign35180_e45098);
        let assign35180_e45100: f64 = (1.0 + assign35180_e45099);
        let assign35180_e45101: f64 = (1e-100 / assign35180_e45100);
        (assign35180_e45101,)
    } else {
        (var_z,)
    }
};
        var_z = assign35180_e45103;

        let (assign35190_e45156,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 != 0.0)) && (var_guard702 == 0.0)) && (var_guard703 == 0.0)) {
        let assign35190_e45120: f64 = (-0.5);
        let assign35190_e45123: f64 = (var_v4 * var_phitdinv);
        let assign35190_e45124: f64 = (assign35190_e45120 * assign35190_e45123);
        let assign35190_e45126: f64 = (assign35190_e45124 - 230.25850929940458);
        let assign35190_e45130: f64 = (-0.5);
        let assign35190_e45133: f64 = (var_v4 * var_phitdinv);
        let assign35190_e45134: f64 = (assign35190_e45130 * assign35190_e45133);
        let assign35190_e45136: f64 = (assign35190_e45134 - 230.25850929940458);
        let assign35190_e45139: f64 = (-0.5);
        let assign35190_e45142: f64 = (var_v4 * var_phitdinv);
        let assign35190_e45143: f64 = (assign35190_e45139 * assign35190_e45142);
        let assign35190_e45145: f64 = (assign35190_e45143 - 230.25850929940458);
        let assign35190_e45147: f64 = (assign35190_e45145 * 0.3333333333333333);
        let assign35190_e45148: f64 = (1.0 + assign35190_e45147);
        let assign35190_e45149: f64 = (assign35190_e45136 * assign35190_e45148);
        let assign35190_e45150: f64 = (0.5 * assign35190_e45149);
        let assign35190_e45151: f64 = (1.0 + assign35190_e45150);
        let assign35190_e45152: f64 = (assign35190_e45126 * assign35190_e45151);
        let assign35190_e45153: f64 = (1.0 + assign35190_e45152);
        let assign35190_e45154: f64 = (1e100 * assign35190_e45153);
        (assign35190_e45154,)
    } else {
        (var_z,)
    }
};
        var_z = assign35190_e45156;

        let (assign35200_e45168,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 != 0.0)) {
        let assign35200_e45166: f64 = (1.0 / var_z);
        (assign35200_e45166,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign35200_e45168;

        let (assign35210_e45180,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 != 0.0)) {
        let assign35210_e45178: f64 = (var_zinv * var_zinv);
        (assign35210_e45178,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign35210_e45180;

        let (assign35220_e45199,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 == 0.0)) {
        let assign35220_e45192: f64 = (var_v4 - var_vmax_d);
        let assign35220_e45194: f64 = (assign35220_e45192 * var_phitdinv);
        let assign35220_e45195: f64 = (1.0 + assign35220_e45194);
        let assign35220_e45197: f64 = (assign35220_e45195 * var_exp_vmax_over_phitd_d);
        (assign35220_e45197,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign35220_e45199;

        let (assign35230_e45211,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 == 0.0)) {
        let assign35230_e45209: f64 = (var_idmult).sqrt();
        (assign35230_e45209,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign35230_e45211;

        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard693_slot = var_guard693;
        *var_guard694_slot = var_guard694;
        *var_guard695_slot = var_guard695;
        *var_guard696_slot = var_guard696;
        *var_guard697_slot = var_guard697;
        *var_guard698_slot = var_guard698;
        *var_guard699_slot = var_guard699;
        *var_guard700_slot = var_guard700;
        *var_guard701_slot = var_guard701;
        *var_guard702_slot = var_guard702;
        *var_guard703_slot = var_guard703;
        *var_i3_slot = var_i3;
        *var_i3_dn5_slot = var_i3_dn5;
        *var_i3_dn6_slot = var_i3_dn6;
        *var_i3_dn7_slot = var_i3_dn7;
        *var_i3_dn8_slot = var_i3_dn8;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_idmult_slot = var_idmult;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_vbbt_slot = var_vbbt;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_73(
        var_abdrain_i: f64,
        var_atatbot_d: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_ftdbot_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard700: f64,
        var_guard701: f64,
        var_idsatbot_d: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_phitd: f64,
        var_phitr: f64,
        var_v4: f64,
        var_vbbtlim_d: f64,
        var_vbibot_d: f64,
        var_vbimin_d: f64,
        var_vbirbotinv_d: f64,
        var_wdepnulrbot_d: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard704_slot: &mut f64,
        var_guard705_slot: &mut f64,
        var_guard706_slot: &mut f64,
        var_guard707_slot: &mut f64,
        var_guard708_slot: &mut f64,
        var_guard709_slot: &mut f64,
        var_guard710_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_z_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard704: f64 = *var_guard704_slot;
        let mut var_guard705: f64 = *var_guard705_slot;
        let mut var_guard706: f64 = *var_guard706_slot;
        let mut var_guard707: f64 = *var_guard707_slot;
        let mut var_guard708: f64 = *var_guard708_slot;
        let mut var_guard709: f64 = *var_guard709_slot;
        let mut var_guard710: f64 = *var_guard710_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_z: f64 = *var_z_slot;

        let (assign35240_e45224,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 == 0.0)) {
        let assign35240_e45222: f64 = (1.0 / var_zinv);
        (assign35240_e45222,)
    } else {
        (var_z,)
    }
};
        var_z = assign35240_e45224;

        let (assign35250_e45234,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) {
        let assign35250_e45232: f64 = (var_idmult - 1.0);
        (assign35250_e45232,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign35250_e45234;

        let assign35260_e45237: f64 = if var_v4 > 0.0 { 1.0 } else { 0.0 };
        var_guard704 = assign35260_e45237;

        let (assign35270_e45263,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) && (var_guard704 != 0.0)) {
        let assign35270_e45249: f64 = (2.0 + var_z);
        let assign35270_e45252: f64 = (var_z + 1.0);
        let assign35270_e45255: f64 = (var_z + 3.0);
        let assign35270_e45256: f64 = (assign35270_e45252 * assign35270_e45255);
        let assign35270_e45257: f64 = (assign35270_e45256).sqrt();
        let assign35270_e45258: f64 = (assign35270_e45249 + assign35270_e45257);
        let assign35270_e45259: f64 = (assign35270_e45258).ln();
        let assign35270_e45260: f64 = (var_phitd * assign35270_e45259);
        let assign35270_e45261: f64 = (2.0 * assign35270_e45260);
        (assign35270_e45261,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign35270_e45263;

        let (assign35280_e45297,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) && (var_guard704 == 0.0)) {
        let assign35280_e45273: f64 = (-var_v4);
        let assign35280_e45278: f64 = (2.0 * var_zinv);
        let assign35280_e45280: f64 = (assign35280_e45278 + 1.0);
        let assign35280_e45283: f64 = (1.0 + var_zinv);
        let assign35280_e45287: f64 = (3.0 * var_zinv);
        let assign35280_e45288: f64 = (1.0 + assign35280_e45287);
        let assign35280_e45289: f64 = (assign35280_e45283 * assign35280_e45288);
        let assign35280_e45290: f64 = (assign35280_e45289).sqrt();
        let assign35280_e45291: f64 = (assign35280_e45280 + assign35280_e45290);
        let assign35280_e45292: f64 = (assign35280_e45291).ln();
        let assign35280_e45293: f64 = (var_phitd * assign35280_e45292);
        let assign35280_e45294: f64 = (2.0 * assign35280_e45293);
        let assign35280_e45295: f64 = (assign35280_e45273 + assign35280_e45294);
        (assign35280_e45295,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign35280_e45297;

        let (assign35290_e45307,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) {
        let assign35290_e45305: f64 = (var_vbimin_d - var_two_psistar);
        (assign35290_e45305,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign35290_e45307;

        let (assign35300_e45334,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) {
        let assign35300_e45316: f64 = (var_v4 + var_vjlim);
        let assign35300_e45319: f64 = (var_v4 - var_vjlim);
        let assign35300_e45322: f64 = (var_v4 - var_vjlim);
        let assign35300_e45323: f64 = (assign35300_e45319 * assign35300_e45322);
        let assign35300_e45326: f64 = (4.0 * var_phitd);
        let assign35300_e45328: f64 = (assign35300_e45326 * var_phitd);
        let assign35300_e45329: f64 = (assign35300_e45323 + assign35300_e45328);
        let assign35300_e45330: f64 = (assign35300_e45329).sqrt();
        let assign35300_e45331: f64 = (assign35300_e45316 - assign35300_e45330);
        let assign35300_e45332: f64 = (0.5 * assign35300_e45331);
        (assign35300_e45332,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign35300_e45334;

        let (assign35310_e45361,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) {
        let assign35310_e45343: f64 = (var_v4 + var_vbbtlim_d);
        let assign35310_e45346: f64 = (var_v4 - var_vbbtlim_d);
        let assign35310_e45349: f64 = (var_v4 - var_vbbtlim_d);
        let assign35310_e45350: f64 = (assign35310_e45346 * assign35310_e45349);
        let assign35310_e45353: f64 = (4.0 * var_phitr);
        let assign35310_e45355: f64 = (assign35310_e45353 * var_phitr);
        let assign35310_e45356: f64 = (assign35310_e45350 + assign35310_e45355);
        let assign35310_e45357: f64 = (assign35310_e45356).sqrt();
        let assign35310_e45358: f64 = (assign35310_e45343 - assign35310_e45357);
        let assign35310_e45359: f64 = (0.5 * assign35310_e45358);
        (assign35310_e45359,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign35310_e45361;

        let (assign35320_e45388,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard700 != 0.0)) {
        let assign35320_e45370: f64 = var_v4;
        let assign35320_e45373: f64 = var_v4;
        let assign35320_e45376: f64 = var_v4;
        let assign35320_e45377: f64 = (assign35320_e45373 * assign35320_e45376);
        let assign35320_e45380: f64 = (4.0 * 1e-6);
        let assign35320_e45382: f64 = (assign35320_e45380 * 1e-6);
        let assign35320_e45383: f64 = (assign35320_e45377 + assign35320_e45382);
        let assign35320_e45384: f64 = (assign35320_e45383).sqrt();
        let assign35320_e45385: f64 = (assign35320_e45370 - assign35320_e45384);
        let assign35320_e45386: f64 = (0.5 * assign35320_e45385);
        (assign35320_e45386,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign35320_e45388;

        let assign35330_e45391: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard705 = assign35330_e45391;

        let (assign35340_e45399, assign35340_e45399_d_n5, assign35340_e45399_d_n6, assign35340_e45399_d_n7, assign35340_e45399_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign35340_e45399;
        var_ijunbot_dn5 = assign35340_e45399_d_n5;
        var_ijunbot_dn6 = assign35340_e45399_d_n6;
        var_ijunbot_dn7 = assign35340_e45399_d_n7;
        var_ijunbot_dn8 = assign35340_e45399_d_n8;

        let (assign35350_e45410,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) {
        let assign35350_e45408: f64 = (var_idsatbot_d * var_idmult);
        (assign35350_e45408,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign35350_e45410;

        let assign35360_e45417: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard706 = assign35360_e45417;

        let (assign35370_e45428, assign35370_e45428_d_n5, assign35370_e45428_d_n6, assign35370_e45428_d_n7, assign35370_e45428_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign35370_e45428;
        var_isrh_dn5 = assign35370_e45428_d_n5;
        var_isrh_dn6 = assign35370_e45428_d_n6;
        var_isrh_dn7 = assign35370_e45428_d_n7;
        var_isrh_dn8 = assign35370_e45428_d_n8;

        let (assign35380_e45442,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) {
        let assign35380_e45440: f64 = (var_vbibot_d - var_vjsrh);
        (assign35380_e45440,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign35380_e45442;

        let (assign35390_e45461,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) {
        let assign35390_e45456: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign35390_e45457: f64 = (1.0 - assign35390_e45456);
        let assign35390_e45458: f64 = (assign35390_e45457).sqrt();
        let assign35390_e45459: f64 = (1.0 - assign35390_e45458);
        (assign35390_e45459,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign35390_e45461;

        let assign35400_e45464: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard707 = assign35400_e45464;

        let (assign35410_e45478,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) && (var_guard707 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign35410_e45478;

        let (assign35420_e45510,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) && (var_guard707 == 0.0)) {
        let assign35420_e45493: f64 = (var_wsrhstep * var_wsrhstep);
        let assign35420_e45495: f64 = (var_wsrhstep).ln();
        let assign35420_e45496: f64 = (assign35420_e45493 * assign35420_e45495);
        let assign35420_e45499: f64 = (1.0 - var_wsrhstep);
        let assign35420_e45500: f64 = (assign35420_e45496 / assign35420_e45499);
        let assign35420_e45502: f64 = (assign35420_e45500 + var_wsrhstep);
        let assign35420_e45506: f64 = (2.0 * var_pbotd_i);
        let assign35420_e45507: f64 = (1.0 - assign35420_e45506);
        let assign35420_e45508: f64 = (assign35420_e45502 * assign35420_e45507);
        (assign35420_e45508,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign35420_e45510;

        let (assign35430_e45524,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) {
        let assign35430_e45522: f64 = (var_wsrhstep + var_dwsrh);
        (assign35430_e45522,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign35430_e45524;

        let assign35440_e45527: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard708 = assign35440_e45527;

        let (assign35450_e45544, assign35450_e45544_d_n5, assign35450_e45544_d_n6, assign35450_e45544_d_n7, assign35450_e45544_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) && (var_guard708 != 0.0)) {
        let assign35450_e45541: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign35450_e45542: f64 = (assign35450_e45541).sqrt();
        (assign35450_e45542, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35450_e45544;
        var_tmp_dn5 = assign35450_e45544_d_n5;
        var_tmp_dn6 = assign35450_e45544_d_n6;
        var_tmp_dn7 = assign35450_e45544_d_n7;
        var_tmp_dn8 = assign35450_e45544_d_n8;

        let (assign35460_e45563, assign35460_e45563_d_n5, assign35460_e45563_d_n6, assign35460_e45563_d_n7, assign35460_e45563_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) && (var_guard708 == 0.0)) {
        let assign35460_e45559: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign35460_e45561: f64 = (assign35460_e45559).powf(var_pbotd_i);
        (assign35460_e45561, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35460_e45563;
        var_tmp_dn5 = assign35460_e45563_d_n5;
        var_tmp_dn6 = assign35460_e45563_d_n6;
        var_tmp_dn7 = assign35460_e45563_d_n7;
        var_tmp_dn8 = assign35460_e45563_d_n8;

        let (assign35470_e45577, assign35470_e45577_d_n5, assign35470_e45577_d_n6, assign35470_e45577_d_n7, assign35470_e45577_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) {
        let assign35470_e45575: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign35470_e45575, (var_wdepnulrbot_d * var_tmp_dn5), (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign35470_e45577;
        var_wdep_dn5 = assign35470_e45577_d_n5;
        var_wdep_dn6 = assign35470_e45577_d_n6;
        var_wdep_dn7 = assign35470_e45577_d_n7;
        var_wdep_dn8 = assign35470_e45577_d_n8;

        let (assign35480_e45595, assign35480_e45595_d_n5, assign35480_e45595_d_n6, assign35480_e45595_d_n7, assign35480_e45595_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) {
        let assign35480_e45590: f64 = (var_zinv - 1.0);
        let assign35480_e45592: f64 = (assign35480_e45590 * var_wdep);
        let assign35480_e45593: f64 = (var_ftdbot_d * assign35480_e45592);
        (assign35480_e45593, (var_ftdbot_d * (assign35480_e45590 * var_wdep_dn5)), (var_ftdbot_d * (assign35480_e45590 * var_wdep_dn6)), (var_ftdbot_d * (assign35480_e45590 * var_wdep_dn7)), (var_ftdbot_d * (assign35480_e45590 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign35480_e45595;
        var_asrh_dn5 = assign35480_e45595_d_n5;
        var_asrh_dn6 = assign35480_e45595_d_n6;
        var_asrh_dn7 = assign35480_e45595_d_n7;
        var_asrh_dn8 = assign35480_e45595_d_n8;

        let (assign35490_e45611, assign35490_e45611_d_n5, assign35490_e45611_d_n6, assign35490_e45611_d_n7, assign35490_e45611_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) {
        let assign35490_e45608: f64 = (var_asrh * var_wsrh);
        let assign35490_e45609: f64 = (var_csrhbotd_i * assign35490_e45608);
        (assign35490_e45609, (var_csrhbotd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign35490_e45611;
        var_isrh_dn5 = assign35490_e45611_d_n5;
        var_isrh_dn6 = assign35490_e45611_d_n6;
        var_isrh_dn7 = assign35490_e45611_d_n7;
        var_isrh_dn8 = assign35490_e45611_d_n8;

        let assign35500_e45614: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard709 = assign35500_e45614;

        let (assign35510_e45625, assign35510_e45625_d_n5, assign35510_e45625_d_n6, assign35510_e45625_d_n7, assign35510_e45625_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign35510_e45625;
        var_itat_dn5 = assign35510_e45625_d_n5;
        var_itat_dn6 = assign35510_e45625_d_n6;
        var_itat_dn7 = assign35510_e45625_d_n7;
        var_itat_dn8 = assign35510_e45625_d_n8;

        let (assign35520_e45643, assign35520_e45643_d_n5, assign35520_e45643_d_n6, assign35520_e45643_d_n7, assign35520_e45643_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35520_e45638: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign35520_e45640: f64 = (assign35520_e45638 / var_vbi_minus_vjsrh);
        let assign35520_e45641: f64 = (var_btatpartbot_d * assign35520_e45640);
        (assign35520_e45641, (var_btatpartbot_d * ((var_wdep_dn5 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign35520_e45643;
        var_btat_dn5 = assign35520_e45643_d_n5;
        var_btat_dn6 = assign35520_e45643_d_n6;
        var_btat_dn7 = assign35520_e45643_d_n7;
        var_btat_dn8 = assign35520_e45643_d_n8;

        let (assign35530_e45659, assign35530_e45659_d_n5, assign35530_e45659_d_n6, assign35530_e45659_d_n7, assign35530_e45659_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35530_e45655: f64 = (0.666666666666667 * var_atatbot_d);
        let assign35530_e45657: f64 = (assign35530_e45655 / var_btat);
        (assign35530_e45657, (-((assign35530_e45655 * var_btat_dn5) / (var_btat * var_btat))), (-((assign35530_e45655 * var_btat_dn6) / (var_btat * var_btat))), (-((assign35530_e45655 * var_btat_dn7) / (var_btat * var_btat))), (-((assign35530_e45655 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign35530_e45659;
        var_twoatatoverthreebtat_dn5 = assign35530_e45659_d_n5;
        var_twoatatoverthreebtat_dn6 = assign35530_e45659_d_n6;
        var_twoatatoverthreebtat_dn7 = assign35530_e45659_d_n7;
        var_twoatatoverthreebtat_dn8 = assign35530_e45659_d_n8;

        let (assign35540_e45673, assign35540_e45673_d_n5, assign35540_e45673_d_n6, assign35540_e45673_d_n7, assign35540_e45673_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35540_e45671: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign35540_e45671, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign35540_e45673;
        var_umaxbeforelimiting_dn5 = assign35540_e45673_d_n5;
        var_umaxbeforelimiting_dn6 = assign35540_e45673_d_n6;
        var_umaxbeforelimiting_dn7 = assign35540_e45673_d_n7;
        var_umaxbeforelimiting_dn8 = assign35540_e45673_d_n8;

        let (assign35550_e45694, assign35550_e45694_d_n5, assign35550_e45694_d_n6, assign35550_e45694_d_n7, assign35550_e45694_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35550_e45685: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign35550_e45688: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign35550_e45690: f64 = (assign35550_e45688 + 1.0);
        let assign35550_e45691: f64 = (assign35550_e45685 / assign35550_e45690);
        let assign35550_e45692: f64 = (assign35550_e45691).sqrt();
        (assign35550_e45692, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign35550_e45690) - (assign35550_e45685 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign35550_e45690 * assign35550_e45690)) / (2.0 * assign35550_e45692)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign35550_e45690) - (assign35550_e45685 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign35550_e45690 * assign35550_e45690)) / (2.0 * assign35550_e45692)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign35550_e45690) - (assign35550_e45685 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign35550_e45690 * assign35550_e45690)) / (2.0 * assign35550_e45692)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign35550_e45690) - (assign35550_e45685 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign35550_e45690 * assign35550_e45690)) / (2.0 * assign35550_e45692)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign35550_e45694;
        var_umax_dn5 = assign35550_e45694_d_n5;
        var_umax_dn6 = assign35550_e45694_d_n6;
        var_umax_dn7 = assign35550_e45694_d_n7;
        var_umax_dn8 = assign35550_e45694_d_n8;

        let (assign35560_e45707, assign35560_e45707_d_n5, assign35560_e45707_d_n6, assign35560_e45707_d_n7, assign35560_e45707_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35560_e45705: f64 = (var_umax).sqrt();
        (assign35560_e45705, (var_umax_dn5 / (2.0 * assign35560_e45705)), (var_umax_dn6 / (2.0 * assign35560_e45705)), (var_umax_dn7 / (2.0 * assign35560_e45705)), (var_umax_dn8 / (2.0 * assign35560_e45705)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign35560_e45707;
        var_sqrtumax_dn5 = assign35560_e45707_d_n5;
        var_sqrtumax_dn6 = assign35560_e45707_d_n6;
        var_sqrtumax_dn7 = assign35560_e45707_d_n7;
        var_sqrtumax_dn8 = assign35560_e45707_d_n8;

        let (assign35570_e45721, assign35570_e45721_d_n5, assign35570_e45721_d_n6, assign35570_e45721_d_n7, assign35570_e45721_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35570_e45719: f64 = (var_umax * var_sqrtumax);
        (assign35570_e45719, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign35570_e45721;
        var_umaxpoweronepointfive_dn5 = assign35570_e45721_d_n5;
        var_umaxpoweronepointfive_dn6 = assign35570_e45721_d_n6;
        var_umaxpoweronepointfive_dn7 = assign35570_e45721_d_n7;
        var_umaxpoweronepointfive_dn8 = assign35570_e45721_d_n8;

        let assign35580_e45723: f64 = (-var_pbotd_i);
        let assign35580_e45725: f64 = (assign35580_e45723 * var_one_over_one_minus_pbot_d);
        let assign35580_e45727: f64 = (-1.0);
        let assign35580_e45728: f64 = if assign35580_e45725 == assign35580_e45727 { 1.0 } else { 0.0 };
        var_guard710 = assign35580_e45728;

        let (assign35590_e45748, assign35590_e45748_d_n5, assign35590_e45748_d_n6, assign35590_e45748_d_n7, assign35590_e45748_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) && (var_guard710 != 0.0)) {
        let assign35590_e45744: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign35590_e45745: f64 = (1.0 + assign35590_e45744);
        let assign35590_e45746: f64 = (1.0 / assign35590_e45745);
        (assign35590_e45746, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign35590_e45745 * assign35590_e45745))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign35590_e45745 * assign35590_e45745))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign35590_e45745 * assign35590_e45745))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign35590_e45745 * assign35590_e45745))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign35590_e45748;
        var_wgamma_dn5 = assign35590_e45748_d_n5;
        var_wgamma_dn6 = assign35590_e45748_d_n6;
        var_wgamma_dn7 = assign35590_e45748_d_n7;
        var_wgamma_dn8 = assign35590_e45748_d_n8;

        let (assign35600_e45772, assign35600_e45772_d_n5, assign35600_e45772_d_n6, assign35600_e45772_d_n7, assign35600_e45772_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) && (var_guard710 == 0.0)) {
        let assign35600_e45764: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign35600_e45765: f64 = (1.0 + assign35600_e45764);
        let assign35600_e45767: f64 = (-var_pbotd_i);
        let assign35600_e45769: f64 = (assign35600_e45767 * var_one_over_one_minus_pbot_d);
        let assign35600_e45770: f64 = (assign35600_e45765).powf(assign35600_e45769);
        (assign35600_e45770, if 0.0 == 0.0 && ((assign35600_e45769) as f64).is_finite() && ((assign35600_e45769) as f64).fract() == 0.0 { if assign35600_e45769 == 0.0 { 0.0 } else { (assign35600_e45769 * ((assign35600_e45765).powf(assign35600_e45769 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign35600_e45770 * (assign35600_e45769 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign35600_e45765))) }, if 0.0 == 0.0 && ((assign35600_e45769) as f64).is_finite() && ((assign35600_e45769) as f64).fract() == 0.0 { if assign35600_e45769 == 0.0 { 0.0 } else { (assign35600_e45769 * ((assign35600_e45765).powf(assign35600_e45769 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign35600_e45770 * (assign35600_e45769 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign35600_e45765))) }, if 0.0 == 0.0 && ((assign35600_e45769) as f64).is_finite() && ((assign35600_e45769) as f64).fract() == 0.0 { if assign35600_e45769 == 0.0 { 0.0 } else { (assign35600_e45769 * ((assign35600_e45765).powf(assign35600_e45769 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign35600_e45770 * (assign35600_e45769 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign35600_e45765))) }, if 0.0 == 0.0 && ((assign35600_e45769) as f64).is_finite() && ((assign35600_e45769) as f64).fract() == 0.0 { if assign35600_e45769 == 0.0 { 0.0 } else { (assign35600_e45769 * ((assign35600_e45765).powf(assign35600_e45769 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign35600_e45770 * (assign35600_e45769 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign35600_e45765))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign35600_e45772;
        var_wgamma_dn5 = assign35600_e45772_d_n5;
        var_wgamma_dn6 = assign35600_e45772_d_n6;
        var_wgamma_dn7 = assign35600_e45772_d_n7;
        var_wgamma_dn8 = assign35600_e45772_d_n8;

        let (assign35610_e45790, assign35610_e45790_d_n5, assign35610_e45790_d_n6, assign35610_e45790_d_n7, assign35610_e45790_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35610_e45784: f64 = (var_wsrh * var_wgamma);
        let assign35610_e45787: f64 = (var_wsrh + var_wgamma);
        let assign35610_e45788: f64 = (assign35610_e45784 / assign35610_e45787);
        (assign35610_e45788, ((((var_wsrh * var_wgamma_dn5) * assign35610_e45787) - (assign35610_e45784 * var_wgamma_dn5)) / (assign35610_e45787 * assign35610_e45787)), ((((var_wsrh * var_wgamma_dn6) * assign35610_e45787) - (assign35610_e45784 * var_wgamma_dn6)) / (assign35610_e45787 * assign35610_e45787)), ((((var_wsrh * var_wgamma_dn7) * assign35610_e45787) - (assign35610_e45784 * var_wgamma_dn7)) / (assign35610_e45787 * assign35610_e45787)), ((((var_wsrh * var_wgamma_dn8) * assign35610_e45787) - (assign35610_e45784 * var_wgamma_dn8)) / (assign35610_e45787 * assign35610_e45787)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign35610_e45790;
        var_wtat_dn5 = assign35610_e45790_d_n5;
        var_wtat_dn6 = assign35610_e45790_d_n6;
        var_wtat_dn7 = assign35610_e45790_d_n7;
        var_wtat_dn8 = assign35610_e45790_d_n8;

        let (assign35620_e45807, assign35620_e45807_d_n5, assign35620_e45807_d_n6, assign35620_e45807_d_n7, assign35620_e45807_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35620_e45803: f64 = (var_btat / var_sqrtumax);
        let assign35620_e45804: f64 = (0.375 * assign35620_e45803);
        let assign35620_e45805: f64 = (assign35620_e45804).sqrt();
        (assign35620_e45805, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35620_e45805)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35620_e45805)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35620_e45805)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35620_e45805)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign35620_e45807;
        var_ktat_dn5 = assign35620_e45807_d_n5;
        var_ktat_dn6 = assign35620_e45807_d_n6;
        var_ktat_dn7 = assign35620_e45807_d_n7;
        var_ktat_dn8 = assign35620_e45807_d_n8;

        let (assign35630_e45825, assign35630_e45825_d_n5, assign35630_e45825_d_n6, assign35630_e45825_d_n7, assign35630_e45825_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35630_e45820: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign35630_e45821: f64 = (2.0 * assign35630_e45820);
        let assign35630_e45823: f64 = (assign35630_e45821 - var_umax);
        (assign35630_e45823, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign35630_e45825;
        var_ltat_dn5 = assign35630_e45825_d_n5;
        var_ltat_dn6 = assign35630_e45825_d_n6;
        var_ltat_dn7 = assign35630_e45825_d_n7;
        var_ltat_dn8 = assign35630_e45825_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard704_slot = var_guard704;
        *var_guard705_slot = var_guard705;
        *var_guard706_slot = var_guard706;
        *var_guard707_slot = var_guard707;
        *var_guard708_slot = var_guard708;
        *var_guard709_slot = var_guard709;
        *var_guard710_slot = var_guard710;
        *var_id__blk213_slot = var_id__blk213;
        *var_idmult_slot = var_idmult;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_z_slot = var_z;
    }

    pub(super) fn stamp_transient_block_74(
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatbot_d: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cbbtbotd_i: f64,
        var_cerfc: f64,
        var_ctatbotd_i: f64,
        var_fbbtbot_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard705: f64,
        var_guard709: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ltat: f64,
        var_ltat_dn5: f64,
        var_ltat_dn6: f64,
        var_ltat_dn7: f64,
        var_ltat_dn8: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_pbrbotd_i: f64,
        var_perfc: f64,
        var_sqrtumax: f64,
        var_sqrtumax_dn5: f64,
        var_sqrtumax_dn6: f64,
        var_sqrtumax_dn7: f64,
        var_sqrtumax_dn8: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn5: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_umax: f64,
        var_umax_dn5: f64,
        var_umax_dn6: f64,
        var_umax_dn7: f64,
        var_umax_dn8: f64,
        var_umaxpoweronepointfive: f64,
        var_umaxpoweronepointfive_dn5: f64,
        var_umaxpoweronepointfive_dn6: f64,
        var_umaxpoweronepointfive_dn7: f64,
        var_umaxpoweronepointfive_dn8: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotd_i: f64,
        var_vbirbotinv_d: f64,
        var_vbrbotd_i: f64,
        var_vbrinvbot_d: f64,
        var_wdepnulrinvbot_d: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard711_slot: &mut f64,
        var_guard712_slot: &mut f64,
        var_guard713_slot: &mut f64,
        var_guard714_slot: &mut f64,
        var_guard715_slot: &mut f64,
        var_guard716_slot: &mut f64,
        var_guard717_slot: &mut f64,
        var_guard718_slot: &mut f64,
        var_guard719_slot: &mut f64,
        var_guard720_slot: &mut f64,
        var_guard721_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard711: f64 = *var_guard711_slot;
        let mut var_guard712: f64 = *var_guard712_slot;
        let mut var_guard713: f64 = *var_guard713_slot;
        let mut var_guard714: f64 = *var_guard714_slot;
        let mut var_guard715: f64 = *var_guard715_slot;
        let mut var_guard716: f64 = *var_guard716_slot;
        let mut var_guard717: f64 = *var_guard717_slot;
        let mut var_guard718: f64 = *var_guard718_slot;
        let mut var_guard719: f64 = *var_guard719_slot;
        let mut var_guard720: f64 = *var_guard720_slot;
        let mut var_guard721: f64 = *var_guard721_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign35640_e45851, assign35640_e45851_d_n5, assign35640_e45851_d_n6, assign35640_e45851_d_n7, assign35640_e45851_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35640_e45837: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign35640_e45839: f64 = (assign35640_e45837 * var_sqrtumax);
        let assign35640_e45842: f64 = (var_atatbot_d * var_umax);
        let assign35640_e45843: f64 = (assign35640_e45839 - assign35640_e45842);
        let assign35640_e45847: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign35640_e45848: f64 = (0.5 * assign35640_e45847);
        let assign35640_e45849: f64 = (assign35640_e45843 + assign35640_e45848);
        (assign35640_e45849, (((((var_atatbot_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign35640_e45837 * var_sqrtumax_dn5)) - (var_atatbot_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign35640_e45837 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign35640_e45837 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign35640_e45837 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign35640_e45851;
        var_mtat_dn5 = assign35640_e45851_d_n5;
        var_mtat_dn6 = assign35640_e45851_d_n6;
        var_mtat_dn7 = assign35640_e45851_d_n7;
        var_mtat_dn8 = assign35640_e45851_d_n8;

        let (assign35650_e45867, assign35650_e45867_d_n5, assign35650_e45867_d_n6, assign35650_e45867_d_n7, assign35650_e45867_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35650_e45863: f64 = (var_ltat - 1.0);
        let assign35650_e45865: f64 = (assign35650_e45863 * var_ktat);
        (assign35650_e45865, ((var_ltat_dn5 * var_ktat) + (assign35650_e45863 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign35650_e45863 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign35650_e45863 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign35650_e45863 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign35650_e45867;
        var_xerfc_dn5 = assign35650_e45867_d_n5;
        var_xerfc_dn6 = assign35650_e45867_d_n6;
        var_xerfc_dn7 = assign35650_e45867_d_n7;
        var_xerfc_dn8 = assign35650_e45867_d_n8;

        let (assign35660_e45881, assign35660_e45881_d_n5, assign35660_e45881_d_n6, assign35660_e45881_d_n7, assign35660_e45881_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35660_e45879: f64 = (var_xerfc * var_xerfc);
        (assign35660_e45879, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign35660_e45881;
        var_ysq_dn5 = assign35660_e45881_d_n5;
        var_ysq_dn6 = assign35660_e45881_d_n6;
        var_ysq_dn7 = assign35660_e45881_d_n7;
        var_ysq_dn8 = assign35660_e45881_d_n8;

        let assign35670_e45884: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard711 = assign35670_e45884;

        let (assign35680_e45904, assign35680_e45904_d_n5, assign35680_e45904_d_n6, assign35680_e45904_d_n7, assign35680_e45904_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) && (var_guard711 != 0.0)) {
        let assign35680_e45900: f64 = (var_perfc * var_xerfc);
        let assign35680_e45901: f64 = (1.0 + assign35680_e45900);
        let assign35680_e45902: f64 = (1.0 / assign35680_e45901);
        (assign35680_e45902, (-((var_perfc * var_xerfc_dn5) / (assign35680_e45901 * assign35680_e45901))), (-((var_perfc * var_xerfc_dn6) / (assign35680_e45901 * assign35680_e45901))), (-((var_perfc * var_xerfc_dn7) / (assign35680_e45901 * assign35680_e45901))), (-((var_perfc * var_xerfc_dn8) / (assign35680_e45901 * assign35680_e45901))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign35680_e45904;
        var_terfc_dn5 = assign35680_e45904_d_n5;
        var_terfc_dn6 = assign35680_e45904_d_n6;
        var_terfc_dn7 = assign35680_e45904_d_n7;
        var_terfc_dn8 = assign35680_e45904_d_n8;

        let (assign35690_e45925, assign35690_e45925_d_n5, assign35690_e45925_d_n6, assign35690_e45925_d_n7, assign35690_e45925_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) && (var_guard711 == 0.0)) {
        let assign35690_e45921: f64 = (var_perfc * var_xerfc);
        let assign35690_e45922: f64 = (1.0 - assign35690_e45921);
        let assign35690_e45923: f64 = (1.0 / assign35690_e45922);
        (assign35690_e45923, (-((-(var_perfc * var_xerfc_dn5)) / (assign35690_e45922 * assign35690_e45922))), (-((-(var_perfc * var_xerfc_dn6)) / (assign35690_e45922 * assign35690_e45922))), (-((-(var_perfc * var_xerfc_dn7)) / (assign35690_e45922 * assign35690_e45922))), (-((-(var_perfc * var_xerfc_dn8)) / (assign35690_e45922 * assign35690_e45922))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign35690_e45925;
        var_terfc_dn5 = assign35690_e45925_d_n5;
        var_terfc_dn6 = assign35690_e45925_d_n6;
        var_terfc_dn7 = assign35690_e45925_d_n7;
        var_terfc_dn8 = assign35690_e45925_d_n8;

        let assign35700_e45927: f64 = (-var_ysq);
        let assign35700_e45929: f64 = (assign35700_e45927 + var_mtat);
        let assign35700_e45931: f64 = (-230.25850929940458);
        let assign35700_e45932: f64 = if assign35700_e45929 > assign35700_e45931 { 1.0 } else { 0.0 };
        var_guard712 = assign35700_e45932;

        let (assign35710_e45950, assign35710_e45950_d_n5, assign35710_e45950_d_n6, assign35710_e45950_d_n7, assign35710_e45950_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) && (var_guard712 != 0.0)) {
        let assign35710_e45945: f64 = (-var_ysq);
        let assign35710_e45947: f64 = (assign35710_e45945 + var_mtat);
        let assign35710_e45948: f64 = (assign35710_e45947).exp();
        (assign35710_e45948, (assign35710_e45948 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign35710_e45948 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign35710_e45948 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign35710_e45948 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35710_e45950;
        var_tmp_dn5 = assign35710_e45950_d_n5;
        var_tmp_dn6 = assign35710_e45950_d_n6;
        var_tmp_dn7 = assign35710_e45950_d_n7;
        var_tmp_dn8 = assign35710_e45950_d_n8;

        let (assign35720_e45999, assign35720_e45999_d_n5, assign35720_e45999_d_n6, assign35720_e45999_d_n7, assign35720_e45999_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) && (var_guard712 == 0.0)) {
        let assign35720_e45966: f64 = (-230.25850929940458);
        let assign35720_e45968: f64 = (-var_ysq);
        let assign35720_e45970: f64 = (assign35720_e45968 + var_mtat);
        let assign35720_e45971: f64 = (assign35720_e45966 - assign35720_e45970);
        let assign35720_e45975: f64 = (-230.25850929940458);
        let assign35720_e45977: f64 = (-var_ysq);
        let assign35720_e45979: f64 = (assign35720_e45977 + var_mtat);
        let assign35720_e45980: f64 = (assign35720_e45975 - assign35720_e45979);
        let assign35720_e45983: f64 = (-230.25850929940458);
        let assign35720_e45985: f64 = (-var_ysq);
        let assign35720_e45987: f64 = (assign35720_e45985 + var_mtat);
        let assign35720_e45988: f64 = (assign35720_e45983 - assign35720_e45987);
        let assign35720_e45990: f64 = (assign35720_e45988 * 0.3333333333333333);
        let assign35720_e45991: f64 = (1.0 + assign35720_e45990);
        let assign35720_e45992: f64 = (assign35720_e45980 * assign35720_e45991);
        let assign35720_e45993: f64 = (0.5 * assign35720_e45992);
        let assign35720_e45994: f64 = (1.0 + assign35720_e45993);
        let assign35720_e45995: f64 = (assign35720_e45971 * assign35720_e45994);
        let assign35720_e45996: f64 = (1.0 + assign35720_e45995);
        let assign35720_e45997: f64 = (1e-100 / assign35720_e45996);
        (assign35720_e45997, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign35720_e45994) + (assign35720_e45971 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign35720_e45991) + (assign35720_e45980 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign35720_e45996 * assign35720_e45996))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign35720_e45994) + (assign35720_e45971 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign35720_e45991) + (assign35720_e45980 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign35720_e45996 * assign35720_e45996))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign35720_e45994) + (assign35720_e45971 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign35720_e45991) + (assign35720_e45980 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign35720_e45996 * assign35720_e45996))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign35720_e45994) + (assign35720_e45971 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign35720_e45991) + (assign35720_e45980 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign35720_e45996 * assign35720_e45996))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35720_e45999;
        var_tmp_dn5 = assign35720_e45999_d_n5;
        var_tmp_dn6 = assign35720_e45999_d_n6;
        var_tmp_dn7 = assign35720_e45999_d_n7;
        var_tmp_dn8 = assign35720_e45999_d_n8;

        let (assign35730_e46029, assign35730_e46029_d_n5, assign35730_e46029_d_n6, assign35730_e46029_d_n7, assign35730_e46029_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35730_e46011: f64 = (0.29214664 * var_terfc);
        let assign35730_e46015: f64 = (var_terfc * var_terfc);
        let assign35730_e46016: f64 = (var_berfc * assign35730_e46015);
        let assign35730_e46017: f64 = (assign35730_e46011 + assign35730_e46016);
        let assign35730_e46021: f64 = (var_terfc * var_terfc);
        let assign35730_e46023: f64 = (assign35730_e46021 * var_terfc);
        let assign35730_e46024: f64 = (var_cerfc * assign35730_e46023);
        let assign35730_e46025: f64 = (assign35730_e46017 + assign35730_e46024);
        let assign35730_e46027: f64 = (assign35730_e46025 * var_tmp);
        (assign35730_e46027, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign35730_e46021 * var_terfc_dn5)))) * var_tmp) + (assign35730_e46025 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign35730_e46021 * var_terfc_dn6)))) * var_tmp) + (assign35730_e46025 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign35730_e46021 * var_terfc_dn7)))) * var_tmp) + (assign35730_e46025 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign35730_e46021 * var_terfc_dn8)))) * var_tmp) + (assign35730_e46025 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign35730_e46029;
        var_erfcpos_dn5 = assign35730_e46029_d_n5;
        var_erfcpos_dn6 = assign35730_e46029_d_n6;
        var_erfcpos_dn7 = assign35730_e46029_d_n7;
        var_erfcpos_dn8 = assign35730_e46029_d_n8;

        let assign35740_e46032: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard713 = assign35740_e46032;

        let (assign35750_e46046, assign35750_e46046_d_n5, assign35750_e46046_d_n6, assign35750_e46046_d_n7, assign35750_e46046_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) && (var_guard713 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign35750_e46046;
        var_erfctimesexpmtat_dn5 = assign35750_e46046_d_n5;
        var_erfctimesexpmtat_dn6 = assign35750_e46046_d_n6;
        var_erfctimesexpmtat_dn7 = assign35750_e46046_d_n7;
        var_erfctimesexpmtat_dn8 = assign35750_e46046_d_n8;

        let assign35760_e46049: f64 = (-230.25850929940458);
        let assign35760_e46050: f64 = if var_mtat > assign35760_e46049 { 1.0 } else { 0.0 };
        var_guard714 = assign35760_e46050;

        let (assign35770_e46068, assign35770_e46068_d_n5, assign35770_e46068_d_n6, assign35770_e46068_d_n7, assign35770_e46068_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) && (var_guard713 == 0.0)) && (var_guard714 != 0.0)) {
        let assign35770_e46066: f64 = (var_mtat).exp();
        (assign35770_e46066, (assign35770_e46066 * var_mtat_dn5), (assign35770_e46066 * var_mtat_dn6), (assign35770_e46066 * var_mtat_dn7), (assign35770_e46066 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35770_e46068;
        var_tmp_dn5 = assign35770_e46068_d_n5;
        var_tmp_dn6 = assign35770_e46068_d_n6;
        var_tmp_dn7 = assign35770_e46068_d_n7;
        var_tmp_dn8 = assign35770_e46068_d_n8;

        let (assign35780_e46111, assign35780_e46111_d_n5, assign35780_e46111_d_n6, assign35780_e46111_d_n7, assign35780_e46111_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) && (var_guard713 == 0.0)) && (var_guard714 == 0.0)) {
        let assign35780_e46087: f64 = (-230.25850929940458);
        let assign35780_e46089: f64 = (assign35780_e46087 - var_mtat);
        let assign35780_e46093: f64 = (-230.25850929940458);
        let assign35780_e46095: f64 = (assign35780_e46093 - var_mtat);
        let assign35780_e46098: f64 = (-230.25850929940458);
        let assign35780_e46100: f64 = (assign35780_e46098 - var_mtat);
        let assign35780_e46102: f64 = (assign35780_e46100 * 0.3333333333333333);
        let assign35780_e46103: f64 = (1.0 + assign35780_e46102);
        let assign35780_e46104: f64 = (assign35780_e46095 * assign35780_e46103);
        let assign35780_e46105: f64 = (0.5 * assign35780_e46104);
        let assign35780_e46106: f64 = (1.0 + assign35780_e46105);
        let assign35780_e46107: f64 = (assign35780_e46089 * assign35780_e46106);
        let assign35780_e46108: f64 = (1.0 + assign35780_e46107);
        let assign35780_e46109: f64 = (1e-100 / assign35780_e46108);
        (assign35780_e46109, (-((1e-100 * (((-var_mtat_dn5) * assign35780_e46106) + (assign35780_e46089 * (0.5 * (((-var_mtat_dn5) * assign35780_e46103) + (assign35780_e46095 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign35780_e46108 * assign35780_e46108))), (-((1e-100 * (((-var_mtat_dn6) * assign35780_e46106) + (assign35780_e46089 * (0.5 * (((-var_mtat_dn6) * assign35780_e46103) + (assign35780_e46095 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign35780_e46108 * assign35780_e46108))), (-((1e-100 * (((-var_mtat_dn7) * assign35780_e46106) + (assign35780_e46089 * (0.5 * (((-var_mtat_dn7) * assign35780_e46103) + (assign35780_e46095 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign35780_e46108 * assign35780_e46108))), (-((1e-100 * (((-var_mtat_dn8) * assign35780_e46106) + (assign35780_e46089 * (0.5 * (((-var_mtat_dn8) * assign35780_e46103) + (assign35780_e46095 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign35780_e46108 * assign35780_e46108))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35780_e46111;
        var_tmp_dn5 = assign35780_e46111_d_n5;
        var_tmp_dn6 = assign35780_e46111_d_n6;
        var_tmp_dn7 = assign35780_e46111_d_n7;
        var_tmp_dn8 = assign35780_e46111_d_n8;

        let (assign35790_e46130, assign35790_e46130_d_n5, assign35790_e46130_d_n6, assign35790_e46130_d_n7, assign35790_e46130_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) && (var_guard713 == 0.0)) {
        let assign35790_e46126: f64 = (2.0 * var_tmp);
        let assign35790_e46128: f64 = (assign35790_e46126 - var_erfcpos);
        (assign35790_e46128, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign35790_e46130;
        var_erfctimesexpmtat_dn5 = assign35790_e46130_d_n5;
        var_erfctimesexpmtat_dn6 = assign35790_e46130_d_n6;
        var_erfctimesexpmtat_dn7 = assign35790_e46130_d_n7;
        var_erfctimesexpmtat_dn8 = assign35790_e46130_d_n8;

        let (assign35800_e46150, assign35800_e46150_d_n5, assign35800_e46150_d_n6, assign35800_e46150_d_n7, assign35800_e46150_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35800_e46142: f64 = (1.772453850905516 * 0.5);
        let assign35800_e46145: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign35800_e46147: f64 = (assign35800_e46145 / var_ktat);
        let assign35800_e46148: f64 = (assign35800_e46142 * assign35800_e46147);
        (assign35800_e46148, (assign35800_e46142 * ((((var_atatbot_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign35800_e46145 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign35800_e46142 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign35800_e46145 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign35800_e46142 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign35800_e46145 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign35800_e46142 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign35800_e46145 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign35800_e46150;
        var_gammamax_dn5 = assign35800_e46150_d_n5;
        var_gammamax_dn6 = assign35800_e46150_d_n6;
        var_gammamax_dn7 = assign35800_e46150_d_n7;
        var_gammamax_dn8 = assign35800_e46150_d_n8;

        let (assign35810_e46168, assign35810_e46168_d_n5, assign35810_e46168_d_n6, assign35810_e46168_d_n7, assign35810_e46168_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35810_e46163: f64 = (var_asrh * var_gammamax);
        let assign35810_e46165: f64 = (assign35810_e46163 * var_wtat);
        let assign35810_e46166: f64 = (var_ctatbotd_i * assign35810_e46165);
        (assign35810_e46166, (var_ctatbotd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign35810_e46163 * var_wtat_dn5))), (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign35810_e46163 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign35810_e46163 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign35810_e46163 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign35810_e46168;
        var_itat_dn5 = assign35810_e46168_d_n5;
        var_itat_dn6 = assign35810_e46168_d_n6;
        var_itat_dn7 = assign35810_e46168_d_n7;
        var_itat_dn8 = assign35810_e46168_d_n8;

        let assign35820_e46171: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard715 = assign35820_e46171;

        let (assign35830_e46182, assign35830_e46182_d_n5, assign35830_e46182_d_n6, assign35830_e46182_d_n7, assign35830_e46182_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard715 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign35830_e46182;
        var_ibbt_dn5 = assign35830_e46182_d_n5;
        var_ibbt_dn6 = assign35830_e46182_d_n6;
        var_ibbt_dn7 = assign35830_e46182_d_n7;
        var_ibbt_dn8 = assign35830_e46182_d_n8;

        let assign35840_e46185: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard716 = assign35840_e46185;

        let (assign35850_e46204, assign35850_e46204_d_n5, assign35850_e46204_d_n6, assign35850_e46204_d_n7, assign35850_e46204_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard715 == 0.0)) && (var_guard716 != 0.0)) {
        let assign35850_e46199: f64 = (var_vbirbotd_i - var_vbbt);
        let assign35850_e46201: f64 = (assign35850_e46199 * var_vbirbotinv_d);
        let assign35850_e46202: f64 = (assign35850_e46201).sqrt();
        (assign35850_e46202, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35850_e46204;
        var_tmp_dn5 = assign35850_e46204_d_n5;
        var_tmp_dn6 = assign35850_e46204_d_n6;
        var_tmp_dn7 = assign35850_e46204_d_n7;
        var_tmp_dn8 = assign35850_e46204_d_n8;

        let (assign35860_e46225, assign35860_e46225_d_n5, assign35860_e46225_d_n6, assign35860_e46225_d_n7, assign35860_e46225_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard715 == 0.0)) && (var_guard716 == 0.0)) {
        let assign35860_e46219: f64 = (var_vbirbotd_i - var_vbbt);
        let assign35860_e46221: f64 = (assign35860_e46219 * var_vbirbotinv_d);
        let assign35860_e46223: f64 = (assign35860_e46221).powf(var_pbotd_i);
        (assign35860_e46223, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35860_e46225;
        var_tmp_dn5 = assign35860_e46225_d_n5;
        var_tmp_dn6 = assign35860_e46225_d_n6;
        var_tmp_dn7 = assign35860_e46225_d_n7;
        var_tmp_dn8 = assign35860_e46225_d_n8;

        let (assign35870_e46245, assign35870_e46245_d_n5, assign35870_e46245_d_n6, assign35870_e46245_d_n7, assign35870_e46245_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35870_e46238: f64 = (var_vbirbotd_i - var_vbbt);
        let assign35870_e46240: f64 = (assign35870_e46238 * var_wdepnulrinvbot_d);
        let assign35870_e46242: f64 = (assign35870_e46240 / var_tmp);
        let assign35870_e46243: f64 = (var_one_over_one_minus_pbot_d * assign35870_e46242);
        (assign35870_e46243, (var_one_over_one_minus_pbot_d * (-((assign35870_e46240 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign35870_e46240 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign35870_e46240 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign35870_e46240 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign35870_e46245;
        var_fmaxr_dn5 = assign35870_e46245_d_n5;
        var_fmaxr_dn6 = assign35870_e46245_d_n6;
        var_fmaxr_dn7 = assign35870_e46245_d_n7;
        var_fmaxr_dn8 = assign35870_e46245_d_n8;

        let assign35880_e46247: f64 = (-var_fbbtbot_d);
        let assign35880_e46249: f64 = (assign35880_e46247 / var_fmaxr);
        let assign35880_e46250: f64 = (assign35880_e46249).abs();
        let assign35880_e46252: f64 = if assign35880_e46250 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard717 = assign35880_e46252;

        let (assign35890_e46270, assign35890_e46270_d_n5, assign35890_e46270_d_n6, assign35890_e46270_d_n7, assign35890_e46270_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard715 == 0.0)) && (var_guard717 != 0.0)) {
        let assign35890_e46265: f64 = (-var_fbbtbot_d);
        let assign35890_e46267: f64 = (assign35890_e46265 / var_fmaxr);
        let assign35890_e46268: f64 = (assign35890_e46267).exp();
        (assign35890_e46268, (assign35890_e46268 * (-((assign35890_e46265 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign35890_e46268 * (-((assign35890_e46265 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign35890_e46268 * (-((assign35890_e46265 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign35890_e46268 * (-((assign35890_e46265 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35890_e46270;
        var_tmp_dn5 = assign35890_e46270_d_n5;
        var_tmp_dn6 = assign35890_e46270_d_n6;
        var_tmp_dn7 = assign35890_e46270_d_n7;
        var_tmp_dn8 = assign35890_e46270_d_n8;

        let assign35900_e46272: f64 = (-var_fbbtbot_d);
        let assign35900_e46274: f64 = (assign35900_e46272 / var_fmaxr);
        let assign35900_e46276: f64 = if assign35900_e46274 < 0.0 { 1.0 } else { 0.0 };
        var_guard718 = assign35900_e46276;

        let (assign35910_e46327, assign35910_e46327_d_n5, assign35910_e46327_d_n6, assign35910_e46327_d_n7, assign35910_e46327_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard715 == 0.0)) && (var_guard717 == 0.0)) && (var_guard718 != 0.0)) {
        let assign35910_e46294: f64 = (-230.25850929940458);
        let assign35910_e46296: f64 = (-var_fbbtbot_d);
        let assign35910_e46298: f64 = (assign35910_e46296 / var_fmaxr);
        let assign35910_e46299: f64 = (assign35910_e46294 - assign35910_e46298);
        let assign35910_e46303: f64 = (-230.25850929940458);
        let assign35910_e46305: f64 = (-var_fbbtbot_d);
        let assign35910_e46307: f64 = (assign35910_e46305 / var_fmaxr);
        let assign35910_e46308: f64 = (assign35910_e46303 - assign35910_e46307);
        let assign35910_e46311: f64 = (-230.25850929940458);
        let assign35910_e46313: f64 = (-var_fbbtbot_d);
        let assign35910_e46315: f64 = (assign35910_e46313 / var_fmaxr);
        let assign35910_e46316: f64 = (assign35910_e46311 - assign35910_e46315);
        let assign35910_e46318: f64 = (assign35910_e46316 * 0.3333333333333333);
        let assign35910_e46319: f64 = (1.0 + assign35910_e46318);
        let assign35910_e46320: f64 = (assign35910_e46308 * assign35910_e46319);
        let assign35910_e46321: f64 = (0.5 * assign35910_e46320);
        let assign35910_e46322: f64 = (1.0 + assign35910_e46321);
        let assign35910_e46323: f64 = (assign35910_e46299 * assign35910_e46322);
        let assign35910_e46324: f64 = (1.0 + assign35910_e46323);
        let assign35910_e46325: f64 = (1e-100 / assign35910_e46324);
        (assign35910_e46325, (-((1e-100 * (((-(-((assign35910_e46296 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign35910_e46322) + (assign35910_e46299 * (0.5 * (((-(-((assign35910_e46305 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign35910_e46319) + (assign35910_e46308 * ((-(-((assign35910_e46313 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35910_e46324 * assign35910_e46324))), (-((1e-100 * (((-(-((assign35910_e46296 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign35910_e46322) + (assign35910_e46299 * (0.5 * (((-(-((assign35910_e46305 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign35910_e46319) + (assign35910_e46308 * ((-(-((assign35910_e46313 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35910_e46324 * assign35910_e46324))), (-((1e-100 * (((-(-((assign35910_e46296 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign35910_e46322) + (assign35910_e46299 * (0.5 * (((-(-((assign35910_e46305 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign35910_e46319) + (assign35910_e46308 * ((-(-((assign35910_e46313 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35910_e46324 * assign35910_e46324))), (-((1e-100 * (((-(-((assign35910_e46296 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign35910_e46322) + (assign35910_e46299 * (0.5 * (((-(-((assign35910_e46305 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign35910_e46319) + (assign35910_e46308 * ((-(-((assign35910_e46313 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35910_e46324 * assign35910_e46324))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35910_e46327;
        var_tmp_dn5 = assign35910_e46327_d_n5;
        var_tmp_dn6 = assign35910_e46327_d_n6;
        var_tmp_dn7 = assign35910_e46327_d_n7;
        var_tmp_dn8 = assign35910_e46327_d_n8;

        let (assign35920_e46376, assign35920_e46376_d_n5, assign35920_e46376_d_n6, assign35920_e46376_d_n7, assign35920_e46376_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard715 == 0.0)) && (var_guard717 == 0.0)) && (var_guard718 == 0.0)) {
        let assign35920_e46346: f64 = (-var_fbbtbot_d);
        let assign35920_e46348: f64 = (assign35920_e46346 / var_fmaxr);
        let assign35920_e46350: f64 = (assign35920_e46348 - 230.25850929940458);
        let assign35920_e46354: f64 = (-var_fbbtbot_d);
        let assign35920_e46356: f64 = (assign35920_e46354 / var_fmaxr);
        let assign35920_e46358: f64 = (assign35920_e46356 - 230.25850929940458);
        let assign35920_e46361: f64 = (-var_fbbtbot_d);
        let assign35920_e46363: f64 = (assign35920_e46361 / var_fmaxr);
        let assign35920_e46365: f64 = (assign35920_e46363 - 230.25850929940458);
        let assign35920_e46367: f64 = (assign35920_e46365 * 0.3333333333333333);
        let assign35920_e46368: f64 = (1.0 + assign35920_e46367);
        let assign35920_e46369: f64 = (assign35920_e46358 * assign35920_e46368);
        let assign35920_e46370: f64 = (0.5 * assign35920_e46369);
        let assign35920_e46371: f64 = (1.0 + assign35920_e46370);
        let assign35920_e46372: f64 = (assign35920_e46350 * assign35920_e46371);
        let assign35920_e46373: f64 = (1.0 + assign35920_e46372);
        let assign35920_e46374: f64 = (1e100 * assign35920_e46373);
        (assign35920_e46374, (1e100 * (((-((assign35920_e46346 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign35920_e46371) + (assign35920_e46350 * (0.5 * (((-((assign35920_e46354 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign35920_e46368) + (assign35920_e46358 * ((-((assign35920_e46361 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign35920_e46346 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign35920_e46371) + (assign35920_e46350 * (0.5 * (((-((assign35920_e46354 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign35920_e46368) + (assign35920_e46358 * ((-((assign35920_e46361 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign35920_e46346 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign35920_e46371) + (assign35920_e46350 * (0.5 * (((-((assign35920_e46354 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign35920_e46368) + (assign35920_e46358 * ((-((assign35920_e46361 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign35920_e46346 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign35920_e46371) + (assign35920_e46350 * (0.5 * (((-((assign35920_e46354 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign35920_e46368) + (assign35920_e46358 * ((-((assign35920_e46361 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35920_e46376;
        var_tmp_dn5 = assign35920_e46376_d_n5;
        var_tmp_dn6 = assign35920_e46376_d_n6;
        var_tmp_dn7 = assign35920_e46376_d_n7;
        var_tmp_dn8 = assign35920_e46376_d_n8;

        let (assign35930_e46396, assign35930_e46396_d_n5, assign35930_e46396_d_n6, assign35930_e46396_d_n7, assign35930_e46396_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35930_e46389: f64 = (var_v4 * var_fmaxr);
        let assign35930_e46391: f64 = (assign35930_e46389 * var_fmaxr);
        let assign35930_e46393: f64 = (assign35930_e46391 * var_tmp);
        let assign35930_e46394: f64 = (var_cbbtbotd_i * assign35930_e46393);
        (assign35930_e46394, (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign35930_e46389 * var_fmaxr_dn5)) * var_tmp) + (assign35930_e46391 * var_tmp_dn5))), (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign35930_e46389 * var_fmaxr_dn6)) * var_tmp) + (assign35930_e46391 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign35930_e46389 * var_fmaxr_dn7)) * var_tmp) + (assign35930_e46391 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign35930_e46389 * var_fmaxr_dn8)) * var_tmp) + (assign35930_e46391 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign35930_e46396;
        var_ibbt_dn5 = assign35930_e46396_d_n5;
        var_ibbt_dn6 = assign35930_e46396_d_n6;
        var_ibbt_dn7 = assign35930_e46396_d_n7;
        var_ibbt_dn8 = assign35930_e46396_d_n8;

        let assign35940_e46399: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard719 = assign35940_e46399;

        let (assign35950_e46410, assign35950_e46410_d_n5, assign35950_e46410_d_n6, assign35950_e46410_d_n7, assign35950_e46410_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard719 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign35950_e46410;
        var_fbreakdown_dn5 = assign35950_e46410_d_n5;
        var_fbreakdown_dn6 = assign35950_e46410_d_n6;
        var_fbreakdown_dn7 = assign35950_e46410_d_n7;
        var_fbreakdown_dn8 = assign35950_e46410_d_n8;

        let assign35960_e46413: f64 = (-var_alphaav);
        let assign35960_e46415: f64 = (assign35960_e46413 * var_vbrbotd_i);
        let assign35960_e46416: f64 = if var_vav > assign35960_e46415 { 1.0 } else { 0.0 };
        var_guard720 = assign35960_e46416;

        let assign35970_e46419: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard721 = assign35970_e46419;

        let (assign35980_e46449, assign35980_e46449_d_n5, assign35980_e46449_d_n6, assign35980_e46449_d_n7, assign35980_e46449_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard719 == 0.0)) && (var_guard720 != 0.0)) && (var_guard721 != 0.0)) {
        let assign35980_e46435: f64 = (var_vav * var_vbrinvbot_d);
        let assign35980_e46438: f64 = (var_vav * var_vbrinvbot_d);
        let assign35980_e46439: f64 = (assign35980_e46435 * assign35980_e46438);
        let assign35980_e46442: f64 = (var_vav * var_vbrinvbot_d);
        let assign35980_e46443: f64 = (assign35980_e46439 * assign35980_e46442);
        let assign35980_e46446: f64 = (var_vav * var_vbrinvbot_d);
        let assign35980_e46447: f64 = (assign35980_e46443 * assign35980_e46446);
        (assign35980_e46447, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35980_e46449;
        var_tmp_dn5 = assign35980_e46449_d_n5;
        var_tmp_dn6 = assign35980_e46449_d_n6;
        var_tmp_dn7 = assign35980_e46449_d_n7;
        var_tmp_dn8 = assign35980_e46449_d_n8;

        let (assign35990_e46471, assign35990_e46471_d_n5, assign35990_e46471_d_n6, assign35990_e46471_d_n7, assign35990_e46471_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard719 == 0.0)) && (var_guard720 != 0.0)) && (var_guard721 == 0.0)) {
        let assign35990_e46466: f64 = (var_vav * var_vbrinvbot_d);
        let assign35990_e46467: f64 = (assign35990_e46466).abs();
        let assign35990_e46469: f64 = (assign35990_e46467).powf(var_pbrbotd_i);
        (assign35990_e46469, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35990_e46471;
        var_tmp_dn5 = assign35990_e46471_d_n5;
        var_tmp_dn6 = assign35990_e46471_d_n6;
        var_tmp_dn7 = assign35990_e46471_d_n7;
        var_tmp_dn8 = assign35990_e46471_d_n8;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard711_slot = var_guard711;
        *var_guard712_slot = var_guard712;
        *var_guard713_slot = var_guard713;
        *var_guard714_slot = var_guard714;
        *var_guard715_slot = var_guard715;
        *var_guard716_slot = var_guard716;
        *var_guard717_slot = var_guard717;
        *var_guard718_slot = var_guard718;
        *var_guard719_slot = var_guard719;
        *var_guard720_slot = var_guard720;
        *var_guard721_slot = var_guard721;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_75(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti_d: f64,
        var_btatpartsti_d: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_fstopbot_d: f64,
        var_ftdsti_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard705: f64,
        var_guard719: f64,
        var_guard720: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
        var_lsdrain_i: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_perfc: f64,
        var_pstid_i: f64,
        var_slopebot_d: f64,
        var_two_psistar: f64,
        var_vav: f64,
        var_vbirstiinv_d: f64,
        var_vbisti_d: f64,
        var_vbrbotd_i: f64,
        var_vjsrh: f64,
        var_wdepnulrsti_d: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard722_slot: &mut f64,
        var_guard723_slot: &mut f64,
        var_guard724_slot: &mut f64,
        var_guard725_slot: &mut f64,
        var_guard726_slot: &mut f64,
        var_guard727_slot: &mut f64,
        var_guard728_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_guard722: f64 = *var_guard722_slot;
        let mut var_guard723: f64 = *var_guard723_slot;
        let mut var_guard724: f64 = *var_guard724_slot;
        let mut var_guard725: f64 = *var_guard725_slot;
        let mut var_guard726: f64 = *var_guard726_slot;
        let mut var_guard727: f64 = *var_guard727_slot;
        let mut var_guard728: f64 = *var_guard728_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign36000_e46489, assign36000_e46489_d_n5, assign36000_e46489_d_n6, assign36000_e46489_d_n7, assign36000_e46489_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard719 == 0.0)) && (var_guard720 != 0.0)) {
        let assign36000_e46486: f64 = (1.0 - var_tmp);
        let assign36000_e46487: f64 = (1.0 / assign36000_e46486);
        (assign36000_e46487, (-((-var_tmp_dn5) / (assign36000_e46486 * assign36000_e46486))), (-((-var_tmp_dn6) / (assign36000_e46486 * assign36000_e46486))), (-((-var_tmp_dn7) / (assign36000_e46486 * assign36000_e46486))), (-((-var_tmp_dn8) / (assign36000_e46486 * assign36000_e46486))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36000_e46489;
        var_fbreakdown_dn5 = assign36000_e46489_d_n5;
        var_fbreakdown_dn6 = assign36000_e46489_d_n6;
        var_fbreakdown_dn7 = assign36000_e46489_d_n7;
        var_fbreakdown_dn8 = assign36000_e46489_d_n8;

        let (assign36010_e46512, assign36010_e46512_d_n5, assign36010_e46512_d_n6, assign36010_e46512_d_n7, assign36010_e46512_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) && (var_guard719 == 0.0)) && (var_guard720 == 0.0)) {
        let assign36010_e46506: f64 = (var_alphaav * var_vbrbotd_i);
        let assign36010_e46507: f64 = (var_vav + assign36010_e46506);
        let assign36010_e46509: f64 = (assign36010_e46507 * var_slopebot_d);
        let assign36010_e46510: f64 = (var_fstopbot_d + assign36010_e46509);
        (assign36010_e46510, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36010_e46512;
        var_fbreakdown_dn5 = assign36010_e46512_d_n5;
        var_fbreakdown_dn6 = assign36010_e46512_d_n6;
        var_fbreakdown_dn7 = assign36010_e46512_d_n7;
        var_fbreakdown_dn8 = assign36010_e46512_d_n8;

        let (assign36020_e46531, assign36020_e46531_d_n5, assign36020_e46531_d_n6, assign36020_e46531_d_n7, assign36020_e46531_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard705 == 0.0)) {
        let assign36020_e46522: f64 = (var_id__blk213 + var_isrh);
        let assign36020_e46524: f64 = (assign36020_e46522 + var_itat);
        let assign36020_e46526: f64 = (assign36020_e46524 + var_ibbt);
        let assign36020_e46527: f64 = (p.p29 * assign36020_e46526);
        let assign36020_e46529: f64 = (assign36020_e46527 * var_fbreakdown);
        (assign36020_e46529, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign36020_e46527 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign36020_e46527 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign36020_e46527 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign36020_e46527 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign36020_e46531;
        var_ijunbot_dn5 = assign36020_e46531_d_n5;
        var_ijunbot_dn6 = assign36020_e46531_d_n6;
        var_ijunbot_dn7 = assign36020_e46531_d_n7;
        var_ijunbot_dn8 = assign36020_e46531_d_n8;

        let assign36030_e46534: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard722 = assign36030_e46534;

        let (assign36040_e46542, assign36040_e46542_d_n5, assign36040_e46542_d_n6, assign36040_e46542_d_n7, assign36040_e46542_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign36040_e46542;
        var_ijunsti_dn5 = assign36040_e46542_d_n5;
        var_ijunsti_dn6 = assign36040_e46542_d_n6;
        var_ijunsti_dn7 = assign36040_e46542_d_n7;
        var_ijunsti_dn8 = assign36040_e46542_d_n8;

        let (assign36050_e46553,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) {
        let assign36050_e46551: f64 = (var_idsatsti_d * var_idmult);
        (assign36050_e46551,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign36050_e46553;

        let assign36060_e46560: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard723 = assign36060_e46560;

        let (assign36070_e46571, assign36070_e46571_d_n5, assign36070_e46571_d_n6, assign36070_e46571_d_n7, assign36070_e46571_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign36070_e46571;
        var_isrh_dn5 = assign36070_e46571_d_n5;
        var_isrh_dn6 = assign36070_e46571_d_n6;
        var_isrh_dn7 = assign36070_e46571_d_n7;
        var_isrh_dn8 = assign36070_e46571_d_n8;

        let (assign36080_e46585,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) {
        let assign36080_e46583: f64 = (var_vbisti_d - var_vjsrh);
        (assign36080_e46583,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign36080_e46585;

        let (assign36090_e46604,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) {
        let assign36090_e46599: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign36090_e46600: f64 = (1.0 - assign36090_e46599);
        let assign36090_e46601: f64 = (assign36090_e46600).sqrt();
        let assign36090_e46602: f64 = (1.0 - assign36090_e46601);
        (assign36090_e46602,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign36090_e46604;

        let assign36100_e46607: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard724 = assign36100_e46607;

        let (assign36110_e46621,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) && (var_guard724 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign36110_e46621;

        let (assign36120_e46653,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) && (var_guard724 == 0.0)) {
        let assign36120_e46636: f64 = (var_wsrhstep * var_wsrhstep);
        let assign36120_e46638: f64 = (var_wsrhstep).ln();
        let assign36120_e46639: f64 = (assign36120_e46636 * assign36120_e46638);
        let assign36120_e46642: f64 = (1.0 - var_wsrhstep);
        let assign36120_e46643: f64 = (assign36120_e46639 / assign36120_e46642);
        let assign36120_e46645: f64 = (assign36120_e46643 + var_wsrhstep);
        let assign36120_e46649: f64 = (2.0 * var_pstid_i);
        let assign36120_e46650: f64 = (1.0 - assign36120_e46649);
        let assign36120_e46651: f64 = (assign36120_e46645 * assign36120_e46650);
        (assign36120_e46651,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign36120_e46653;

        let (assign36130_e46667,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) {
        let assign36130_e46665: f64 = (var_wsrhstep + var_dwsrh);
        (assign36130_e46665,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign36130_e46667;

        let assign36140_e46670: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard725 = assign36140_e46670;

        let (assign36150_e46687, assign36150_e46687_d_n5, assign36150_e46687_d_n6, assign36150_e46687_d_n7, assign36150_e46687_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) && (var_guard725 != 0.0)) {
        let assign36150_e46684: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign36150_e46685: f64 = (assign36150_e46684).sqrt();
        (assign36150_e46685, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36150_e46687;
        var_tmp_dn5 = assign36150_e46687_d_n5;
        var_tmp_dn6 = assign36150_e46687_d_n6;
        var_tmp_dn7 = assign36150_e46687_d_n7;
        var_tmp_dn8 = assign36150_e46687_d_n8;

        let (assign36160_e46706, assign36160_e46706_d_n5, assign36160_e46706_d_n6, assign36160_e46706_d_n7, assign36160_e46706_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) && (var_guard725 == 0.0)) {
        let assign36160_e46702: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign36160_e46704: f64 = (assign36160_e46702).powf(var_pstid_i);
        (assign36160_e46704, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36160_e46706;
        var_tmp_dn5 = assign36160_e46706_d_n5;
        var_tmp_dn6 = assign36160_e46706_d_n6;
        var_tmp_dn7 = assign36160_e46706_d_n7;
        var_tmp_dn8 = assign36160_e46706_d_n8;

        let (assign36170_e46720, assign36170_e46720_d_n5, assign36170_e46720_d_n6, assign36170_e46720_d_n7, assign36170_e46720_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) {
        let assign36170_e46718: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign36170_e46718, (var_wdepnulrsti_d * var_tmp_dn5), (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign36170_e46720;
        var_wdep_dn5 = assign36170_e46720_d_n5;
        var_wdep_dn6 = assign36170_e46720_d_n6;
        var_wdep_dn7 = assign36170_e46720_d_n7;
        var_wdep_dn8 = assign36170_e46720_d_n8;

        let (assign36180_e46738, assign36180_e46738_d_n5, assign36180_e46738_d_n6, assign36180_e46738_d_n7, assign36180_e46738_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) {
        let assign36180_e46733: f64 = (var_zinv - 1.0);
        let assign36180_e46735: f64 = (assign36180_e46733 * var_wdep);
        let assign36180_e46736: f64 = (var_ftdsti_d * assign36180_e46735);
        (assign36180_e46736, (var_ftdsti_d * (assign36180_e46733 * var_wdep_dn5)), (var_ftdsti_d * (assign36180_e46733 * var_wdep_dn6)), (var_ftdsti_d * (assign36180_e46733 * var_wdep_dn7)), (var_ftdsti_d * (assign36180_e46733 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign36180_e46738;
        var_asrh_dn5 = assign36180_e46738_d_n5;
        var_asrh_dn6 = assign36180_e46738_d_n6;
        var_asrh_dn7 = assign36180_e46738_d_n7;
        var_asrh_dn8 = assign36180_e46738_d_n8;

        let (assign36190_e46754, assign36190_e46754_d_n5, assign36190_e46754_d_n6, assign36190_e46754_d_n7, assign36190_e46754_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) {
        let assign36190_e46751: f64 = (var_asrh * var_wsrh);
        let assign36190_e46752: f64 = (var_csrhstid_i * assign36190_e46751);
        (assign36190_e46752, (var_csrhstid_i * (var_asrh_dn5 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign36190_e46754;
        var_isrh_dn5 = assign36190_e46754_d_n5;
        var_isrh_dn6 = assign36190_e46754_d_n6;
        var_isrh_dn7 = assign36190_e46754_d_n7;
        var_isrh_dn8 = assign36190_e46754_d_n8;

        let assign36200_e46757: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard726 = assign36200_e46757;

        let (assign36210_e46768, assign36210_e46768_d_n5, assign36210_e46768_d_n6, assign36210_e46768_d_n7, assign36210_e46768_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign36210_e46768;
        var_itat_dn5 = assign36210_e46768_d_n5;
        var_itat_dn6 = assign36210_e46768_d_n6;
        var_itat_dn7 = assign36210_e46768_d_n7;
        var_itat_dn8 = assign36210_e46768_d_n8;

        let (assign36220_e46786, assign36220_e46786_d_n5, assign36220_e46786_d_n6, assign36220_e46786_d_n7, assign36220_e46786_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36220_e46781: f64 = (var_wdep * var_one_minus_psti_d);
        let assign36220_e46783: f64 = (assign36220_e46781 / var_vbi_minus_vjsrh);
        let assign36220_e46784: f64 = (var_btatpartsti_d * assign36220_e46783);
        (assign36220_e46784, (var_btatpartsti_d * ((var_wdep_dn5 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign36220_e46786;
        var_btat_dn5 = assign36220_e46786_d_n5;
        var_btat_dn6 = assign36220_e46786_d_n6;
        var_btat_dn7 = assign36220_e46786_d_n7;
        var_btat_dn8 = assign36220_e46786_d_n8;

        let (assign36230_e46802, assign36230_e46802_d_n5, assign36230_e46802_d_n6, assign36230_e46802_d_n7, assign36230_e46802_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36230_e46798: f64 = (0.666666666666667 * var_atatsti_d);
        let assign36230_e46800: f64 = (assign36230_e46798 / var_btat);
        (assign36230_e46800, (-((assign36230_e46798 * var_btat_dn5) / (var_btat * var_btat))), (-((assign36230_e46798 * var_btat_dn6) / (var_btat * var_btat))), (-((assign36230_e46798 * var_btat_dn7) / (var_btat * var_btat))), (-((assign36230_e46798 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign36230_e46802;
        var_twoatatoverthreebtat_dn5 = assign36230_e46802_d_n5;
        var_twoatatoverthreebtat_dn6 = assign36230_e46802_d_n6;
        var_twoatatoverthreebtat_dn7 = assign36230_e46802_d_n7;
        var_twoatatoverthreebtat_dn8 = assign36230_e46802_d_n8;

        let (assign36240_e46816, assign36240_e46816_d_n5, assign36240_e46816_d_n6, assign36240_e46816_d_n7, assign36240_e46816_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36240_e46814: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign36240_e46814, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign36240_e46816;
        var_umaxbeforelimiting_dn5 = assign36240_e46816_d_n5;
        var_umaxbeforelimiting_dn6 = assign36240_e46816_d_n6;
        var_umaxbeforelimiting_dn7 = assign36240_e46816_d_n7;
        var_umaxbeforelimiting_dn8 = assign36240_e46816_d_n8;

        let (assign36250_e46837, assign36250_e46837_d_n5, assign36250_e46837_d_n6, assign36250_e46837_d_n7, assign36250_e46837_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36250_e46828: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign36250_e46831: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign36250_e46833: f64 = (assign36250_e46831 + 1.0);
        let assign36250_e46834: f64 = (assign36250_e46828 / assign36250_e46833);
        let assign36250_e46835: f64 = (assign36250_e46834).sqrt();
        (assign36250_e46835, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign36250_e46833) - (assign36250_e46828 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign36250_e46833 * assign36250_e46833)) / (2.0 * assign36250_e46835)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign36250_e46833) - (assign36250_e46828 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign36250_e46833 * assign36250_e46833)) / (2.0 * assign36250_e46835)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign36250_e46833) - (assign36250_e46828 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign36250_e46833 * assign36250_e46833)) / (2.0 * assign36250_e46835)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign36250_e46833) - (assign36250_e46828 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign36250_e46833 * assign36250_e46833)) / (2.0 * assign36250_e46835)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign36250_e46837;
        var_umax_dn5 = assign36250_e46837_d_n5;
        var_umax_dn6 = assign36250_e46837_d_n6;
        var_umax_dn7 = assign36250_e46837_d_n7;
        var_umax_dn8 = assign36250_e46837_d_n8;

        let (assign36260_e46850, assign36260_e46850_d_n5, assign36260_e46850_d_n6, assign36260_e46850_d_n7, assign36260_e46850_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36260_e46848: f64 = (var_umax).sqrt();
        (assign36260_e46848, (var_umax_dn5 / (2.0 * assign36260_e46848)), (var_umax_dn6 / (2.0 * assign36260_e46848)), (var_umax_dn7 / (2.0 * assign36260_e46848)), (var_umax_dn8 / (2.0 * assign36260_e46848)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign36260_e46850;
        var_sqrtumax_dn5 = assign36260_e46850_d_n5;
        var_sqrtumax_dn6 = assign36260_e46850_d_n6;
        var_sqrtumax_dn7 = assign36260_e46850_d_n7;
        var_sqrtumax_dn8 = assign36260_e46850_d_n8;

        let (assign36270_e46864, assign36270_e46864_d_n5, assign36270_e46864_d_n6, assign36270_e46864_d_n7, assign36270_e46864_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36270_e46862: f64 = (var_umax * var_sqrtumax);
        (assign36270_e46862, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign36270_e46864;
        var_umaxpoweronepointfive_dn5 = assign36270_e46864_d_n5;
        var_umaxpoweronepointfive_dn6 = assign36270_e46864_d_n6;
        var_umaxpoweronepointfive_dn7 = assign36270_e46864_d_n7;
        var_umaxpoweronepointfive_dn8 = assign36270_e46864_d_n8;

        let assign36280_e46866: f64 = (-var_pstid_i);
        let assign36280_e46868: f64 = (assign36280_e46866 * var_one_over_one_minus_psti_d);
        let assign36280_e46870: f64 = (-1.0);
        let assign36280_e46871: f64 = if assign36280_e46868 == assign36280_e46870 { 1.0 } else { 0.0 };
        var_guard727 = assign36280_e46871;

        let (assign36290_e46891, assign36290_e46891_d_n5, assign36290_e46891_d_n6, assign36290_e46891_d_n7, assign36290_e46891_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) && (var_guard727 != 0.0)) {
        let assign36290_e46887: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign36290_e46888: f64 = (1.0 + assign36290_e46887);
        let assign36290_e46889: f64 = (1.0 / assign36290_e46888);
        (assign36290_e46889, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign36290_e46888 * assign36290_e46888))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign36290_e46888 * assign36290_e46888))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign36290_e46888 * assign36290_e46888))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign36290_e46888 * assign36290_e46888))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign36290_e46891;
        var_wgamma_dn5 = assign36290_e46891_d_n5;
        var_wgamma_dn6 = assign36290_e46891_d_n6;
        var_wgamma_dn7 = assign36290_e46891_d_n7;
        var_wgamma_dn8 = assign36290_e46891_d_n8;

        let (assign36300_e46915, assign36300_e46915_d_n5, assign36300_e46915_d_n6, assign36300_e46915_d_n7, assign36300_e46915_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) && (var_guard727 == 0.0)) {
        let assign36300_e46907: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign36300_e46908: f64 = (1.0 + assign36300_e46907);
        let assign36300_e46910: f64 = (-var_pstid_i);
        let assign36300_e46912: f64 = (assign36300_e46910 * var_one_over_one_minus_psti_d);
        let assign36300_e46913: f64 = (assign36300_e46908).powf(assign36300_e46912);
        (assign36300_e46913, if 0.0 == 0.0 && ((assign36300_e46912) as f64).is_finite() && ((assign36300_e46912) as f64).fract() == 0.0 { if assign36300_e46912 == 0.0 { 0.0 } else { (assign36300_e46912 * ((assign36300_e46908).powf(assign36300_e46912 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign36300_e46913 * (assign36300_e46912 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign36300_e46908))) }, if 0.0 == 0.0 && ((assign36300_e46912) as f64).is_finite() && ((assign36300_e46912) as f64).fract() == 0.0 { if assign36300_e46912 == 0.0 { 0.0 } else { (assign36300_e46912 * ((assign36300_e46908).powf(assign36300_e46912 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign36300_e46913 * (assign36300_e46912 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign36300_e46908))) }, if 0.0 == 0.0 && ((assign36300_e46912) as f64).is_finite() && ((assign36300_e46912) as f64).fract() == 0.0 { if assign36300_e46912 == 0.0 { 0.0 } else { (assign36300_e46912 * ((assign36300_e46908).powf(assign36300_e46912 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign36300_e46913 * (assign36300_e46912 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign36300_e46908))) }, if 0.0 == 0.0 && ((assign36300_e46912) as f64).is_finite() && ((assign36300_e46912) as f64).fract() == 0.0 { if assign36300_e46912 == 0.0 { 0.0 } else { (assign36300_e46912 * ((assign36300_e46908).powf(assign36300_e46912 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign36300_e46913 * (assign36300_e46912 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign36300_e46908))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign36300_e46915;
        var_wgamma_dn5 = assign36300_e46915_d_n5;
        var_wgamma_dn6 = assign36300_e46915_d_n6;
        var_wgamma_dn7 = assign36300_e46915_d_n7;
        var_wgamma_dn8 = assign36300_e46915_d_n8;

        let (assign36310_e46933, assign36310_e46933_d_n5, assign36310_e46933_d_n6, assign36310_e46933_d_n7, assign36310_e46933_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36310_e46927: f64 = (var_wsrh * var_wgamma);
        let assign36310_e46930: f64 = (var_wsrh + var_wgamma);
        let assign36310_e46931: f64 = (assign36310_e46927 / assign36310_e46930);
        (assign36310_e46931, ((((var_wsrh * var_wgamma_dn5) * assign36310_e46930) - (assign36310_e46927 * var_wgamma_dn5)) / (assign36310_e46930 * assign36310_e46930)), ((((var_wsrh * var_wgamma_dn6) * assign36310_e46930) - (assign36310_e46927 * var_wgamma_dn6)) / (assign36310_e46930 * assign36310_e46930)), ((((var_wsrh * var_wgamma_dn7) * assign36310_e46930) - (assign36310_e46927 * var_wgamma_dn7)) / (assign36310_e46930 * assign36310_e46930)), ((((var_wsrh * var_wgamma_dn8) * assign36310_e46930) - (assign36310_e46927 * var_wgamma_dn8)) / (assign36310_e46930 * assign36310_e46930)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign36310_e46933;
        var_wtat_dn5 = assign36310_e46933_d_n5;
        var_wtat_dn6 = assign36310_e46933_d_n6;
        var_wtat_dn7 = assign36310_e46933_d_n7;
        var_wtat_dn8 = assign36310_e46933_d_n8;

        let (assign36320_e46950, assign36320_e46950_d_n5, assign36320_e46950_d_n6, assign36320_e46950_d_n7, assign36320_e46950_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36320_e46946: f64 = (var_btat / var_sqrtumax);
        let assign36320_e46947: f64 = (0.375 * assign36320_e46946);
        let assign36320_e46948: f64 = (assign36320_e46947).sqrt();
        (assign36320_e46948, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36320_e46948)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36320_e46948)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36320_e46948)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36320_e46948)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign36320_e46950;
        var_ktat_dn5 = assign36320_e46950_d_n5;
        var_ktat_dn6 = assign36320_e46950_d_n6;
        var_ktat_dn7 = assign36320_e46950_d_n7;
        var_ktat_dn8 = assign36320_e46950_d_n8;

        let (assign36330_e46968, assign36330_e46968_d_n5, assign36330_e46968_d_n6, assign36330_e46968_d_n7, assign36330_e46968_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36330_e46963: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign36330_e46964: f64 = (2.0 * assign36330_e46963);
        let assign36330_e46966: f64 = (assign36330_e46964 - var_umax);
        (assign36330_e46966, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign36330_e46968;
        var_ltat_dn5 = assign36330_e46968_d_n5;
        var_ltat_dn6 = assign36330_e46968_d_n6;
        var_ltat_dn7 = assign36330_e46968_d_n7;
        var_ltat_dn8 = assign36330_e46968_d_n8;

        let (assign36340_e46994, assign36340_e46994_d_n5, assign36340_e46994_d_n6, assign36340_e46994_d_n7, assign36340_e46994_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36340_e46980: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign36340_e46982: f64 = (assign36340_e46980 * var_sqrtumax);
        let assign36340_e46985: f64 = (var_atatsti_d * var_umax);
        let assign36340_e46986: f64 = (assign36340_e46982 - assign36340_e46985);
        let assign36340_e46990: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign36340_e46991: f64 = (0.5 * assign36340_e46990);
        let assign36340_e46992: f64 = (assign36340_e46986 + assign36340_e46991);
        (assign36340_e46992, (((((var_atatsti_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign36340_e46980 * var_sqrtumax_dn5)) - (var_atatsti_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign36340_e46980 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign36340_e46980 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign36340_e46980 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign36340_e46994;
        var_mtat_dn5 = assign36340_e46994_d_n5;
        var_mtat_dn6 = assign36340_e46994_d_n6;
        var_mtat_dn7 = assign36340_e46994_d_n7;
        var_mtat_dn8 = assign36340_e46994_d_n8;

        let (assign36350_e47010, assign36350_e47010_d_n5, assign36350_e47010_d_n6, assign36350_e47010_d_n7, assign36350_e47010_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36350_e47006: f64 = (var_ltat - 1.0);
        let assign36350_e47008: f64 = (assign36350_e47006 * var_ktat);
        (assign36350_e47008, ((var_ltat_dn5 * var_ktat) + (assign36350_e47006 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign36350_e47006 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign36350_e47006 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign36350_e47006 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign36350_e47010;
        var_xerfc_dn5 = assign36350_e47010_d_n5;
        var_xerfc_dn6 = assign36350_e47010_d_n6;
        var_xerfc_dn7 = assign36350_e47010_d_n7;
        var_xerfc_dn8 = assign36350_e47010_d_n8;

        let (assign36360_e47024, assign36360_e47024_d_n5, assign36360_e47024_d_n6, assign36360_e47024_d_n7, assign36360_e47024_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36360_e47022: f64 = (var_xerfc * var_xerfc);
        (assign36360_e47022, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign36360_e47024;
        var_ysq_dn5 = assign36360_e47024_d_n5;
        var_ysq_dn6 = assign36360_e47024_d_n6;
        var_ysq_dn7 = assign36360_e47024_d_n7;
        var_ysq_dn8 = assign36360_e47024_d_n8;

        let assign36370_e47027: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard728 = assign36370_e47027;

        let (assign36380_e47047, assign36380_e47047_d_n5, assign36380_e47047_d_n6, assign36380_e47047_d_n7, assign36380_e47047_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) && (var_guard728 != 0.0)) {
        let assign36380_e47043: f64 = (var_perfc * var_xerfc);
        let assign36380_e47044: f64 = (1.0 + assign36380_e47043);
        let assign36380_e47045: f64 = (1.0 / assign36380_e47044);
        (assign36380_e47045, (-((var_perfc * var_xerfc_dn5) / (assign36380_e47044 * assign36380_e47044))), (-((var_perfc * var_xerfc_dn6) / (assign36380_e47044 * assign36380_e47044))), (-((var_perfc * var_xerfc_dn7) / (assign36380_e47044 * assign36380_e47044))), (-((var_perfc * var_xerfc_dn8) / (assign36380_e47044 * assign36380_e47044))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign36380_e47047;
        var_terfc_dn5 = assign36380_e47047_d_n5;
        var_terfc_dn6 = assign36380_e47047_d_n6;
        var_terfc_dn7 = assign36380_e47047_d_n7;
        var_terfc_dn8 = assign36380_e47047_d_n8;

        let (assign36390_e47068, assign36390_e47068_d_n5, assign36390_e47068_d_n6, assign36390_e47068_d_n7, assign36390_e47068_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) && (var_guard728 == 0.0)) {
        let assign36390_e47064: f64 = (var_perfc * var_xerfc);
        let assign36390_e47065: f64 = (1.0 - assign36390_e47064);
        let assign36390_e47066: f64 = (1.0 / assign36390_e47065);
        (assign36390_e47066, (-((-(var_perfc * var_xerfc_dn5)) / (assign36390_e47065 * assign36390_e47065))), (-((-(var_perfc * var_xerfc_dn6)) / (assign36390_e47065 * assign36390_e47065))), (-((-(var_perfc * var_xerfc_dn7)) / (assign36390_e47065 * assign36390_e47065))), (-((-(var_perfc * var_xerfc_dn8)) / (assign36390_e47065 * assign36390_e47065))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign36390_e47068;
        var_terfc_dn5 = assign36390_e47068_d_n5;
        var_terfc_dn6 = assign36390_e47068_d_n6;
        var_terfc_dn7 = assign36390_e47068_d_n7;
        var_terfc_dn8 = assign36390_e47068_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_guard722_slot = var_guard722;
        *var_guard723_slot = var_guard723;
        *var_guard724_slot = var_guard724;
        *var_guard725_slot = var_guard725;
        *var_guard726_slot = var_guard726;
        *var_guard727_slot = var_guard727;
        *var_guard728_slot = var_guard728;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_76(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti_d: f64,
        var_berfc: f64,
        var_cbbtstid_i: f64,
        var_cerfc: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_ctatstid_i: f64,
        var_fbbtsti_d: f64,
        var_fstopsti_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard722: f64,
        var_guard726: f64,
        var_idmult: f64,
        var_idsatgat_d: f64,
        var_isrh: f64,
        var_isrh_dn5: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lgdrain_i: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrstid_i: f64,
        var_pstid_i: f64,
        var_slopesti_d: f64,
        var_terfc: f64,
        var_terfc_dn5: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirstid_i: f64,
        var_vbirstiinv_d: f64,
        var_vbrinvsti_d: f64,
        var_vbrstid_i: f64,
        var_wdepnulrinvsti_d: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_xerfc: f64,
        var_ysq: f64,
        var_ysq_dn5: f64,
        var_ysq_dn6: f64,
        var_ysq_dn7: f64,
        var_ysq_dn8: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard729_slot: &mut f64,
        var_guard730_slot: &mut f64,
        var_guard731_slot: &mut f64,
        var_guard732_slot: &mut f64,
        var_guard733_slot: &mut f64,
        var_guard734_slot: &mut f64,
        var_guard735_slot: &mut f64,
        var_guard736_slot: &mut f64,
        var_guard737_slot: &mut f64,
        var_guard738_slot: &mut f64,
        var_guard739_slot: &mut f64,
        var_guard740_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard729: f64 = *var_guard729_slot;
        let mut var_guard730: f64 = *var_guard730_slot;
        let mut var_guard731: f64 = *var_guard731_slot;
        let mut var_guard732: f64 = *var_guard732_slot;
        let mut var_guard733: f64 = *var_guard733_slot;
        let mut var_guard734: f64 = *var_guard734_slot;
        let mut var_guard735: f64 = *var_guard735_slot;
        let mut var_guard736: f64 = *var_guard736_slot;
        let mut var_guard737: f64 = *var_guard737_slot;
        let mut var_guard738: f64 = *var_guard738_slot;
        let mut var_guard739: f64 = *var_guard739_slot;
        let mut var_guard740: f64 = *var_guard740_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;

        let assign36400_e47070: f64 = (-var_ysq);
        let assign36400_e47072: f64 = (assign36400_e47070 + var_mtat);
        let assign36400_e47074: f64 = (-230.25850929940458);
        let assign36400_e47075: f64 = if assign36400_e47072 > assign36400_e47074 { 1.0 } else { 0.0 };
        var_guard729 = assign36400_e47075;

        let (assign36410_e47093, assign36410_e47093_d_n5, assign36410_e47093_d_n6, assign36410_e47093_d_n7, assign36410_e47093_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) && (var_guard729 != 0.0)) {
        let assign36410_e47088: f64 = (-var_ysq);
        let assign36410_e47090: f64 = (assign36410_e47088 + var_mtat);
        let assign36410_e47091: f64 = (assign36410_e47090).exp();
        (assign36410_e47091, (assign36410_e47091 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign36410_e47091 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign36410_e47091 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign36410_e47091 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36410_e47093;
        var_tmp_dn5 = assign36410_e47093_d_n5;
        var_tmp_dn6 = assign36410_e47093_d_n6;
        var_tmp_dn7 = assign36410_e47093_d_n7;
        var_tmp_dn8 = assign36410_e47093_d_n8;

        let (assign36420_e47142, assign36420_e47142_d_n5, assign36420_e47142_d_n6, assign36420_e47142_d_n7, assign36420_e47142_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) && (var_guard729 == 0.0)) {
        let assign36420_e47109: f64 = (-230.25850929940458);
        let assign36420_e47111: f64 = (-var_ysq);
        let assign36420_e47113: f64 = (assign36420_e47111 + var_mtat);
        let assign36420_e47114: f64 = (assign36420_e47109 - assign36420_e47113);
        let assign36420_e47118: f64 = (-230.25850929940458);
        let assign36420_e47120: f64 = (-var_ysq);
        let assign36420_e47122: f64 = (assign36420_e47120 + var_mtat);
        let assign36420_e47123: f64 = (assign36420_e47118 - assign36420_e47122);
        let assign36420_e47126: f64 = (-230.25850929940458);
        let assign36420_e47128: f64 = (-var_ysq);
        let assign36420_e47130: f64 = (assign36420_e47128 + var_mtat);
        let assign36420_e47131: f64 = (assign36420_e47126 - assign36420_e47130);
        let assign36420_e47133: f64 = (assign36420_e47131 * 0.3333333333333333);
        let assign36420_e47134: f64 = (1.0 + assign36420_e47133);
        let assign36420_e47135: f64 = (assign36420_e47123 * assign36420_e47134);
        let assign36420_e47136: f64 = (0.5 * assign36420_e47135);
        let assign36420_e47137: f64 = (1.0 + assign36420_e47136);
        let assign36420_e47138: f64 = (assign36420_e47114 * assign36420_e47137);
        let assign36420_e47139: f64 = (1.0 + assign36420_e47138);
        let assign36420_e47140: f64 = (1e-100 / assign36420_e47139);
        (assign36420_e47140, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign36420_e47137) + (assign36420_e47114 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign36420_e47134) + (assign36420_e47123 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign36420_e47139 * assign36420_e47139))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign36420_e47137) + (assign36420_e47114 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign36420_e47134) + (assign36420_e47123 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign36420_e47139 * assign36420_e47139))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign36420_e47137) + (assign36420_e47114 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign36420_e47134) + (assign36420_e47123 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign36420_e47139 * assign36420_e47139))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign36420_e47137) + (assign36420_e47114 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign36420_e47134) + (assign36420_e47123 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign36420_e47139 * assign36420_e47139))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36420_e47142;
        var_tmp_dn5 = assign36420_e47142_d_n5;
        var_tmp_dn6 = assign36420_e47142_d_n6;
        var_tmp_dn7 = assign36420_e47142_d_n7;
        var_tmp_dn8 = assign36420_e47142_d_n8;

        let (assign36430_e47172, assign36430_e47172_d_n5, assign36430_e47172_d_n6, assign36430_e47172_d_n7, assign36430_e47172_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36430_e47154: f64 = (0.29214664 * var_terfc);
        let assign36430_e47158: f64 = (var_terfc * var_terfc);
        let assign36430_e47159: f64 = (var_berfc * assign36430_e47158);
        let assign36430_e47160: f64 = (assign36430_e47154 + assign36430_e47159);
        let assign36430_e47164: f64 = (var_terfc * var_terfc);
        let assign36430_e47166: f64 = (assign36430_e47164 * var_terfc);
        let assign36430_e47167: f64 = (var_cerfc * assign36430_e47166);
        let assign36430_e47168: f64 = (assign36430_e47160 + assign36430_e47167);
        let assign36430_e47170: f64 = (assign36430_e47168 * var_tmp);
        (assign36430_e47170, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign36430_e47164 * var_terfc_dn5)))) * var_tmp) + (assign36430_e47168 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign36430_e47164 * var_terfc_dn6)))) * var_tmp) + (assign36430_e47168 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign36430_e47164 * var_terfc_dn7)))) * var_tmp) + (assign36430_e47168 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign36430_e47164 * var_terfc_dn8)))) * var_tmp) + (assign36430_e47168 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign36430_e47172;
        var_erfcpos_dn5 = assign36430_e47172_d_n5;
        var_erfcpos_dn6 = assign36430_e47172_d_n6;
        var_erfcpos_dn7 = assign36430_e47172_d_n7;
        var_erfcpos_dn8 = assign36430_e47172_d_n8;

        let assign36440_e47175: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard730 = assign36440_e47175;

        let (assign36450_e47189, assign36450_e47189_d_n5, assign36450_e47189_d_n6, assign36450_e47189_d_n7, assign36450_e47189_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) && (var_guard730 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign36450_e47189;
        var_erfctimesexpmtat_dn5 = assign36450_e47189_d_n5;
        var_erfctimesexpmtat_dn6 = assign36450_e47189_d_n6;
        var_erfctimesexpmtat_dn7 = assign36450_e47189_d_n7;
        var_erfctimesexpmtat_dn8 = assign36450_e47189_d_n8;

        let assign36460_e47192: f64 = (-230.25850929940458);
        let assign36460_e47193: f64 = if var_mtat > assign36460_e47192 { 1.0 } else { 0.0 };
        var_guard731 = assign36460_e47193;

        let (assign36470_e47211, assign36470_e47211_d_n5, assign36470_e47211_d_n6, assign36470_e47211_d_n7, assign36470_e47211_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) && (var_guard730 == 0.0)) && (var_guard731 != 0.0)) {
        let assign36470_e47209: f64 = (var_mtat).exp();
        (assign36470_e47209, (assign36470_e47209 * var_mtat_dn5), (assign36470_e47209 * var_mtat_dn6), (assign36470_e47209 * var_mtat_dn7), (assign36470_e47209 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36470_e47211;
        var_tmp_dn5 = assign36470_e47211_d_n5;
        var_tmp_dn6 = assign36470_e47211_d_n6;
        var_tmp_dn7 = assign36470_e47211_d_n7;
        var_tmp_dn8 = assign36470_e47211_d_n8;

        let (assign36480_e47254, assign36480_e47254_d_n5, assign36480_e47254_d_n6, assign36480_e47254_d_n7, assign36480_e47254_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) && (var_guard730 == 0.0)) && (var_guard731 == 0.0)) {
        let assign36480_e47230: f64 = (-230.25850929940458);
        let assign36480_e47232: f64 = (assign36480_e47230 - var_mtat);
        let assign36480_e47236: f64 = (-230.25850929940458);
        let assign36480_e47238: f64 = (assign36480_e47236 - var_mtat);
        let assign36480_e47241: f64 = (-230.25850929940458);
        let assign36480_e47243: f64 = (assign36480_e47241 - var_mtat);
        let assign36480_e47245: f64 = (assign36480_e47243 * 0.3333333333333333);
        let assign36480_e47246: f64 = (1.0 + assign36480_e47245);
        let assign36480_e47247: f64 = (assign36480_e47238 * assign36480_e47246);
        let assign36480_e47248: f64 = (0.5 * assign36480_e47247);
        let assign36480_e47249: f64 = (1.0 + assign36480_e47248);
        let assign36480_e47250: f64 = (assign36480_e47232 * assign36480_e47249);
        let assign36480_e47251: f64 = (1.0 + assign36480_e47250);
        let assign36480_e47252: f64 = (1e-100 / assign36480_e47251);
        (assign36480_e47252, (-((1e-100 * (((-var_mtat_dn5) * assign36480_e47249) + (assign36480_e47232 * (0.5 * (((-var_mtat_dn5) * assign36480_e47246) + (assign36480_e47238 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign36480_e47251 * assign36480_e47251))), (-((1e-100 * (((-var_mtat_dn6) * assign36480_e47249) + (assign36480_e47232 * (0.5 * (((-var_mtat_dn6) * assign36480_e47246) + (assign36480_e47238 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign36480_e47251 * assign36480_e47251))), (-((1e-100 * (((-var_mtat_dn7) * assign36480_e47249) + (assign36480_e47232 * (0.5 * (((-var_mtat_dn7) * assign36480_e47246) + (assign36480_e47238 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign36480_e47251 * assign36480_e47251))), (-((1e-100 * (((-var_mtat_dn8) * assign36480_e47249) + (assign36480_e47232 * (0.5 * (((-var_mtat_dn8) * assign36480_e47246) + (assign36480_e47238 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign36480_e47251 * assign36480_e47251))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36480_e47254;
        var_tmp_dn5 = assign36480_e47254_d_n5;
        var_tmp_dn6 = assign36480_e47254_d_n6;
        var_tmp_dn7 = assign36480_e47254_d_n7;
        var_tmp_dn8 = assign36480_e47254_d_n8;

        let (assign36490_e47273, assign36490_e47273_d_n5, assign36490_e47273_d_n6, assign36490_e47273_d_n7, assign36490_e47273_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) && (var_guard730 == 0.0)) {
        let assign36490_e47269: f64 = (2.0 * var_tmp);
        let assign36490_e47271: f64 = (assign36490_e47269 - var_erfcpos);
        (assign36490_e47271, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign36490_e47273;
        var_erfctimesexpmtat_dn5 = assign36490_e47273_d_n5;
        var_erfctimesexpmtat_dn6 = assign36490_e47273_d_n6;
        var_erfctimesexpmtat_dn7 = assign36490_e47273_d_n7;
        var_erfctimesexpmtat_dn8 = assign36490_e47273_d_n8;

        let (assign36500_e47293, assign36500_e47293_d_n5, assign36500_e47293_d_n6, assign36500_e47293_d_n7, assign36500_e47293_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36500_e47285: f64 = (1.772453850905516 * 0.5);
        let assign36500_e47288: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign36500_e47290: f64 = (assign36500_e47288 / var_ktat);
        let assign36500_e47291: f64 = (assign36500_e47285 * assign36500_e47290);
        (assign36500_e47291, (assign36500_e47285 * ((((var_atatsti_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign36500_e47288 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign36500_e47285 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign36500_e47288 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign36500_e47285 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign36500_e47288 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign36500_e47285 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign36500_e47288 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign36500_e47293;
        var_gammamax_dn5 = assign36500_e47293_d_n5;
        var_gammamax_dn6 = assign36500_e47293_d_n6;
        var_gammamax_dn7 = assign36500_e47293_d_n7;
        var_gammamax_dn8 = assign36500_e47293_d_n8;

        let (assign36510_e47311, assign36510_e47311_d_n5, assign36510_e47311_d_n6, assign36510_e47311_d_n7, assign36510_e47311_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36510_e47306: f64 = (var_asrh * var_gammamax);
        let assign36510_e47308: f64 = (assign36510_e47306 * var_wtat);
        let assign36510_e47309: f64 = (var_ctatstid_i * assign36510_e47308);
        (assign36510_e47309, (var_ctatstid_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign36510_e47306 * var_wtat_dn5))), (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign36510_e47306 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign36510_e47306 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign36510_e47306 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign36510_e47311;
        var_itat_dn5 = assign36510_e47311_d_n5;
        var_itat_dn6 = assign36510_e47311_d_n6;
        var_itat_dn7 = assign36510_e47311_d_n7;
        var_itat_dn8 = assign36510_e47311_d_n8;

        let assign36520_e47314: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard732 = assign36520_e47314;

        let (assign36530_e47325, assign36530_e47325_d_n5, assign36530_e47325_d_n6, assign36530_e47325_d_n7, assign36530_e47325_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard732 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign36530_e47325;
        var_ibbt_dn5 = assign36530_e47325_d_n5;
        var_ibbt_dn6 = assign36530_e47325_d_n6;
        var_ibbt_dn7 = assign36530_e47325_d_n7;
        var_ibbt_dn8 = assign36530_e47325_d_n8;

        let assign36540_e47328: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard733 = assign36540_e47328;

        let (assign36550_e47347, assign36550_e47347_d_n5, assign36550_e47347_d_n6, assign36550_e47347_d_n7, assign36550_e47347_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard732 == 0.0)) && (var_guard733 != 0.0)) {
        let assign36550_e47342: f64 = (var_vbirstid_i - var_vbbt);
        let assign36550_e47344: f64 = (assign36550_e47342 * var_vbirstiinv_d);
        let assign36550_e47345: f64 = (assign36550_e47344).sqrt();
        (assign36550_e47345, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36550_e47347;
        var_tmp_dn5 = assign36550_e47347_d_n5;
        var_tmp_dn6 = assign36550_e47347_d_n6;
        var_tmp_dn7 = assign36550_e47347_d_n7;
        var_tmp_dn8 = assign36550_e47347_d_n8;

        let (assign36560_e47368, assign36560_e47368_d_n5, assign36560_e47368_d_n6, assign36560_e47368_d_n7, assign36560_e47368_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard732 == 0.0)) && (var_guard733 == 0.0)) {
        let assign36560_e47362: f64 = (var_vbirstid_i - var_vbbt);
        let assign36560_e47364: f64 = (assign36560_e47362 * var_vbirstiinv_d);
        let assign36560_e47366: f64 = (assign36560_e47364).powf(var_pstid_i);
        (assign36560_e47366, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36560_e47368;
        var_tmp_dn5 = assign36560_e47368_d_n5;
        var_tmp_dn6 = assign36560_e47368_d_n6;
        var_tmp_dn7 = assign36560_e47368_d_n7;
        var_tmp_dn8 = assign36560_e47368_d_n8;

        let (assign36570_e47388, assign36570_e47388_d_n5, assign36570_e47388_d_n6, assign36570_e47388_d_n7, assign36570_e47388_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36570_e47381: f64 = (var_vbirstid_i - var_vbbt);
        let assign36570_e47383: f64 = (assign36570_e47381 * var_wdepnulrinvsti_d);
        let assign36570_e47385: f64 = (assign36570_e47383 / var_tmp);
        let assign36570_e47386: f64 = (var_one_over_one_minus_psti_d * assign36570_e47385);
        (assign36570_e47386, (var_one_over_one_minus_psti_d * (-((assign36570_e47383 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign36570_e47383 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign36570_e47383 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign36570_e47383 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign36570_e47388;
        var_fmaxr_dn5 = assign36570_e47388_d_n5;
        var_fmaxr_dn6 = assign36570_e47388_d_n6;
        var_fmaxr_dn7 = assign36570_e47388_d_n7;
        var_fmaxr_dn8 = assign36570_e47388_d_n8;

        let assign36580_e47390: f64 = (-var_fbbtsti_d);
        let assign36580_e47392: f64 = (assign36580_e47390 / var_fmaxr);
        let assign36580_e47393: f64 = (assign36580_e47392).abs();
        let assign36580_e47395: f64 = if assign36580_e47393 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard734 = assign36580_e47395;

        let (assign36590_e47413, assign36590_e47413_d_n5, assign36590_e47413_d_n6, assign36590_e47413_d_n7, assign36590_e47413_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard732 == 0.0)) && (var_guard734 != 0.0)) {
        let assign36590_e47408: f64 = (-var_fbbtsti_d);
        let assign36590_e47410: f64 = (assign36590_e47408 / var_fmaxr);
        let assign36590_e47411: f64 = (assign36590_e47410).exp();
        (assign36590_e47411, (assign36590_e47411 * (-((assign36590_e47408 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign36590_e47411 * (-((assign36590_e47408 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign36590_e47411 * (-((assign36590_e47408 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign36590_e47411 * (-((assign36590_e47408 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36590_e47413;
        var_tmp_dn5 = assign36590_e47413_d_n5;
        var_tmp_dn6 = assign36590_e47413_d_n6;
        var_tmp_dn7 = assign36590_e47413_d_n7;
        var_tmp_dn8 = assign36590_e47413_d_n8;

        let assign36600_e47415: f64 = (-var_fbbtsti_d);
        let assign36600_e47417: f64 = (assign36600_e47415 / var_fmaxr);
        let assign36600_e47419: f64 = if assign36600_e47417 < 0.0 { 1.0 } else { 0.0 };
        var_guard735 = assign36600_e47419;

        let (assign36610_e47470, assign36610_e47470_d_n5, assign36610_e47470_d_n6, assign36610_e47470_d_n7, assign36610_e47470_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard732 == 0.0)) && (var_guard734 == 0.0)) && (var_guard735 != 0.0)) {
        let assign36610_e47437: f64 = (-230.25850929940458);
        let assign36610_e47439: f64 = (-var_fbbtsti_d);
        let assign36610_e47441: f64 = (assign36610_e47439 / var_fmaxr);
        let assign36610_e47442: f64 = (assign36610_e47437 - assign36610_e47441);
        let assign36610_e47446: f64 = (-230.25850929940458);
        let assign36610_e47448: f64 = (-var_fbbtsti_d);
        let assign36610_e47450: f64 = (assign36610_e47448 / var_fmaxr);
        let assign36610_e47451: f64 = (assign36610_e47446 - assign36610_e47450);
        let assign36610_e47454: f64 = (-230.25850929940458);
        let assign36610_e47456: f64 = (-var_fbbtsti_d);
        let assign36610_e47458: f64 = (assign36610_e47456 / var_fmaxr);
        let assign36610_e47459: f64 = (assign36610_e47454 - assign36610_e47458);
        let assign36610_e47461: f64 = (assign36610_e47459 * 0.3333333333333333);
        let assign36610_e47462: f64 = (1.0 + assign36610_e47461);
        let assign36610_e47463: f64 = (assign36610_e47451 * assign36610_e47462);
        let assign36610_e47464: f64 = (0.5 * assign36610_e47463);
        let assign36610_e47465: f64 = (1.0 + assign36610_e47464);
        let assign36610_e47466: f64 = (assign36610_e47442 * assign36610_e47465);
        let assign36610_e47467: f64 = (1.0 + assign36610_e47466);
        let assign36610_e47468: f64 = (1e-100 / assign36610_e47467);
        (assign36610_e47468, (-((1e-100 * (((-(-((assign36610_e47439 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign36610_e47465) + (assign36610_e47442 * (0.5 * (((-(-((assign36610_e47448 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign36610_e47462) + (assign36610_e47451 * ((-(-((assign36610_e47456 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36610_e47467 * assign36610_e47467))), (-((1e-100 * (((-(-((assign36610_e47439 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign36610_e47465) + (assign36610_e47442 * (0.5 * (((-(-((assign36610_e47448 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign36610_e47462) + (assign36610_e47451 * ((-(-((assign36610_e47456 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36610_e47467 * assign36610_e47467))), (-((1e-100 * (((-(-((assign36610_e47439 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign36610_e47465) + (assign36610_e47442 * (0.5 * (((-(-((assign36610_e47448 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign36610_e47462) + (assign36610_e47451 * ((-(-((assign36610_e47456 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36610_e47467 * assign36610_e47467))), (-((1e-100 * (((-(-((assign36610_e47439 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign36610_e47465) + (assign36610_e47442 * (0.5 * (((-(-((assign36610_e47448 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign36610_e47462) + (assign36610_e47451 * ((-(-((assign36610_e47456 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36610_e47467 * assign36610_e47467))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36610_e47470;
        var_tmp_dn5 = assign36610_e47470_d_n5;
        var_tmp_dn6 = assign36610_e47470_d_n6;
        var_tmp_dn7 = assign36610_e47470_d_n7;
        var_tmp_dn8 = assign36610_e47470_d_n8;

        let (assign36620_e47519, assign36620_e47519_d_n5, assign36620_e47519_d_n6, assign36620_e47519_d_n7, assign36620_e47519_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard732 == 0.0)) && (var_guard734 == 0.0)) && (var_guard735 == 0.0)) {
        let assign36620_e47489: f64 = (-var_fbbtsti_d);
        let assign36620_e47491: f64 = (assign36620_e47489 / var_fmaxr);
        let assign36620_e47493: f64 = (assign36620_e47491 - 230.25850929940458);
        let assign36620_e47497: f64 = (-var_fbbtsti_d);
        let assign36620_e47499: f64 = (assign36620_e47497 / var_fmaxr);
        let assign36620_e47501: f64 = (assign36620_e47499 - 230.25850929940458);
        let assign36620_e47504: f64 = (-var_fbbtsti_d);
        let assign36620_e47506: f64 = (assign36620_e47504 / var_fmaxr);
        let assign36620_e47508: f64 = (assign36620_e47506 - 230.25850929940458);
        let assign36620_e47510: f64 = (assign36620_e47508 * 0.3333333333333333);
        let assign36620_e47511: f64 = (1.0 + assign36620_e47510);
        let assign36620_e47512: f64 = (assign36620_e47501 * assign36620_e47511);
        let assign36620_e47513: f64 = (0.5 * assign36620_e47512);
        let assign36620_e47514: f64 = (1.0 + assign36620_e47513);
        let assign36620_e47515: f64 = (assign36620_e47493 * assign36620_e47514);
        let assign36620_e47516: f64 = (1.0 + assign36620_e47515);
        let assign36620_e47517: f64 = (1e100 * assign36620_e47516);
        (assign36620_e47517, (1e100 * (((-((assign36620_e47489 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign36620_e47514) + (assign36620_e47493 * (0.5 * (((-((assign36620_e47497 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign36620_e47511) + (assign36620_e47501 * ((-((assign36620_e47504 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36620_e47489 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign36620_e47514) + (assign36620_e47493 * (0.5 * (((-((assign36620_e47497 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign36620_e47511) + (assign36620_e47501 * ((-((assign36620_e47504 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36620_e47489 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign36620_e47514) + (assign36620_e47493 * (0.5 * (((-((assign36620_e47497 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign36620_e47511) + (assign36620_e47501 * ((-((assign36620_e47504 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36620_e47489 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign36620_e47514) + (assign36620_e47493 * (0.5 * (((-((assign36620_e47497 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign36620_e47511) + (assign36620_e47501 * ((-((assign36620_e47504 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36620_e47519;
        var_tmp_dn5 = assign36620_e47519_d_n5;
        var_tmp_dn6 = assign36620_e47519_d_n6;
        var_tmp_dn7 = assign36620_e47519_d_n7;
        var_tmp_dn8 = assign36620_e47519_d_n8;

        let (assign36630_e47539, assign36630_e47539_d_n5, assign36630_e47539_d_n6, assign36630_e47539_d_n7, assign36630_e47539_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36630_e47532: f64 = (var_v4 * var_fmaxr);
        let assign36630_e47534: f64 = (assign36630_e47532 * var_fmaxr);
        let assign36630_e47536: f64 = (assign36630_e47534 * var_tmp);
        let assign36630_e47537: f64 = (var_cbbtstid_i * assign36630_e47536);
        (assign36630_e47537, (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign36630_e47532 * var_fmaxr_dn5)) * var_tmp) + (assign36630_e47534 * var_tmp_dn5))), (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign36630_e47532 * var_fmaxr_dn6)) * var_tmp) + (assign36630_e47534 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign36630_e47532 * var_fmaxr_dn7)) * var_tmp) + (assign36630_e47534 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign36630_e47532 * var_fmaxr_dn8)) * var_tmp) + (assign36630_e47534 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign36630_e47539;
        var_ibbt_dn5 = assign36630_e47539_d_n5;
        var_ibbt_dn6 = assign36630_e47539_d_n6;
        var_ibbt_dn7 = assign36630_e47539_d_n7;
        var_ibbt_dn8 = assign36630_e47539_d_n8;

        let assign36640_e47542: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard736 = assign36640_e47542;

        let (assign36650_e47553, assign36650_e47553_d_n5, assign36650_e47553_d_n6, assign36650_e47553_d_n7, assign36650_e47553_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard736 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36650_e47553;
        var_fbreakdown_dn5 = assign36650_e47553_d_n5;
        var_fbreakdown_dn6 = assign36650_e47553_d_n6;
        var_fbreakdown_dn7 = assign36650_e47553_d_n7;
        var_fbreakdown_dn8 = assign36650_e47553_d_n8;

        let assign36660_e47556: f64 = (-var_alphaav);
        let assign36660_e47558: f64 = (assign36660_e47556 * var_vbrstid_i);
        let assign36660_e47559: f64 = if var_vav > assign36660_e47558 { 1.0 } else { 0.0 };
        var_guard737 = assign36660_e47559;

        let assign36670_e47562: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard738 = assign36670_e47562;

        let (assign36680_e47592, assign36680_e47592_d_n5, assign36680_e47592_d_n6, assign36680_e47592_d_n7, assign36680_e47592_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard736 == 0.0)) && (var_guard737 != 0.0)) && (var_guard738 != 0.0)) {
        let assign36680_e47578: f64 = (var_vav * var_vbrinvsti_d);
        let assign36680_e47581: f64 = (var_vav * var_vbrinvsti_d);
        let assign36680_e47582: f64 = (assign36680_e47578 * assign36680_e47581);
        let assign36680_e47585: f64 = (var_vav * var_vbrinvsti_d);
        let assign36680_e47586: f64 = (assign36680_e47582 * assign36680_e47585);
        let assign36680_e47589: f64 = (var_vav * var_vbrinvsti_d);
        let assign36680_e47590: f64 = (assign36680_e47586 * assign36680_e47589);
        (assign36680_e47590, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36680_e47592;
        var_tmp_dn5 = assign36680_e47592_d_n5;
        var_tmp_dn6 = assign36680_e47592_d_n6;
        var_tmp_dn7 = assign36680_e47592_d_n7;
        var_tmp_dn8 = assign36680_e47592_d_n8;

        let (assign36690_e47614, assign36690_e47614_d_n5, assign36690_e47614_d_n6, assign36690_e47614_d_n7, assign36690_e47614_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard736 == 0.0)) && (var_guard737 != 0.0)) && (var_guard738 == 0.0)) {
        let assign36690_e47609: f64 = (var_vav * var_vbrinvsti_d);
        let assign36690_e47610: f64 = (assign36690_e47609).abs();
        let assign36690_e47612: f64 = (assign36690_e47610).powf(var_pbrstid_i);
        (assign36690_e47612, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36690_e47614;
        var_tmp_dn5 = assign36690_e47614_d_n5;
        var_tmp_dn6 = assign36690_e47614_d_n6;
        var_tmp_dn7 = assign36690_e47614_d_n7;
        var_tmp_dn8 = assign36690_e47614_d_n8;

        let (assign36700_e47632, assign36700_e47632_d_n5, assign36700_e47632_d_n6, assign36700_e47632_d_n7, assign36700_e47632_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard736 == 0.0)) && (var_guard737 != 0.0)) {
        let assign36700_e47629: f64 = (1.0 - var_tmp);
        let assign36700_e47630: f64 = (1.0 / assign36700_e47629);
        (assign36700_e47630, (-((-var_tmp_dn5) / (assign36700_e47629 * assign36700_e47629))), (-((-var_tmp_dn6) / (assign36700_e47629 * assign36700_e47629))), (-((-var_tmp_dn7) / (assign36700_e47629 * assign36700_e47629))), (-((-var_tmp_dn8) / (assign36700_e47629 * assign36700_e47629))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36700_e47632;
        var_fbreakdown_dn5 = assign36700_e47632_d_n5;
        var_fbreakdown_dn6 = assign36700_e47632_d_n6;
        var_fbreakdown_dn7 = assign36700_e47632_d_n7;
        var_fbreakdown_dn8 = assign36700_e47632_d_n8;

        let (assign36710_e47655, assign36710_e47655_d_n5, assign36710_e47655_d_n6, assign36710_e47655_d_n7, assign36710_e47655_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) && (var_guard736 == 0.0)) && (var_guard737 == 0.0)) {
        let assign36710_e47649: f64 = (var_alphaav * var_vbrstid_i);
        let assign36710_e47650: f64 = (var_vav + assign36710_e47649);
        let assign36710_e47652: f64 = (assign36710_e47650 * var_slopesti_d);
        let assign36710_e47653: f64 = (var_fstopsti_d + assign36710_e47652);
        (assign36710_e47653, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36710_e47655;
        var_fbreakdown_dn5 = assign36710_e47655_d_n5;
        var_fbreakdown_dn6 = assign36710_e47655_d_n6;
        var_fbreakdown_dn7 = assign36710_e47655_d_n7;
        var_fbreakdown_dn8 = assign36710_e47655_d_n8;

        let (assign36720_e47674, assign36720_e47674_d_n5, assign36720_e47674_d_n6, assign36720_e47674_d_n7, assign36720_e47674_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard722 == 0.0)) {
        let assign36720_e47665: f64 = (var_id__blk213 + var_isrh);
        let assign36720_e47667: f64 = (assign36720_e47665 + var_itat);
        let assign36720_e47669: f64 = (assign36720_e47667 + var_ibbt);
        let assign36720_e47670: f64 = (p.p29 * assign36720_e47669);
        let assign36720_e47672: f64 = (assign36720_e47670 * var_fbreakdown);
        (assign36720_e47672, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign36720_e47670 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign36720_e47670 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign36720_e47670 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign36720_e47670 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign36720_e47674;
        var_ijunsti_dn5 = assign36720_e47674_d_n5;
        var_ijunsti_dn6 = assign36720_e47674_d_n6;
        var_ijunsti_dn7 = assign36720_e47674_d_n7;
        var_ijunsti_dn8 = assign36720_e47674_d_n8;

        let assign36730_e47677: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard739 = assign36730_e47677;

        let (assign36740_e47685, assign36740_e47685_d_n5, assign36740_e47685_d_n6, assign36740_e47685_d_n7, assign36740_e47685_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign36740_e47685;
        var_ijungat_dn5 = assign36740_e47685_d_n5;
        var_ijungat_dn6 = assign36740_e47685_d_n6;
        var_ijungat_dn7 = assign36740_e47685_d_n7;
        var_ijungat_dn8 = assign36740_e47685_d_n8;

        let (assign36750_e47696,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) {
        let assign36750_e47694: f64 = (var_idsatgat_d * var_idmult);
        (assign36750_e47694,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign36750_e47696;

        let assign36760_e47703: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard740 = assign36760_e47703;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard729_slot = var_guard729;
        *var_guard730_slot = var_guard730;
        *var_guard731_slot = var_guard731;
        *var_guard732_slot = var_guard732;
        *var_guard733_slot = var_guard733;
        *var_guard734_slot = var_guard734;
        *var_guard735_slot = var_guard735;
        *var_guard736_slot = var_guard736;
        *var_guard737_slot = var_guard737;
        *var_guard738_slot = var_guard738;
        *var_guard739_slot = var_guard739;
        *var_guard740_slot = var_guard740;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
    }

    pub(super) fn stamp_transient_block_77(
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btatpartgat_d: f64,
        var_cerfc: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_ftdgat_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard739: f64,
        var_guard740: f64,
        var_one_minus_pgat_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
        var_two_psistar: f64,
        var_vbigat_d: f64,
        var_vbirgatinv_d: f64,
        var_vjsrh: f64,
        var_wdepnulrgat_d: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_guard741_slot: &mut f64,
        var_guard742_slot: &mut f64,
        var_guard743_slot: &mut f64,
        var_guard744_slot: &mut f64,
        var_guard745_slot: &mut f64,
        var_guard746_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_guard741: f64 = *var_guard741_slot;
        let mut var_guard742: f64 = *var_guard742_slot;
        let mut var_guard743: f64 = *var_guard743_slot;
        let mut var_guard744: f64 = *var_guard744_slot;
        let mut var_guard745: f64 = *var_guard745_slot;
        let mut var_guard746: f64 = *var_guard746_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign36770_e47714, assign36770_e47714_d_n5, assign36770_e47714_d_n6, assign36770_e47714_d_n7, assign36770_e47714_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign36770_e47714;
        var_isrh_dn5 = assign36770_e47714_d_n5;
        var_isrh_dn6 = assign36770_e47714_d_n6;
        var_isrh_dn7 = assign36770_e47714_d_n7;
        var_isrh_dn8 = assign36770_e47714_d_n8;

        let (assign36780_e47728,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) {
        let assign36780_e47726: f64 = (var_vbigat_d - var_vjsrh);
        (assign36780_e47726,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign36780_e47728;

        let (assign36790_e47747,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) {
        let assign36790_e47742: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign36790_e47743: f64 = (1.0 - assign36790_e47742);
        let assign36790_e47744: f64 = (assign36790_e47743).sqrt();
        let assign36790_e47745: f64 = (1.0 - assign36790_e47744);
        (assign36790_e47745,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign36790_e47747;

        let assign36800_e47750: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard741 = assign36800_e47750;

        let (assign36810_e47764,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) && (var_guard741 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign36810_e47764;

        let (assign36820_e47796,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) && (var_guard741 == 0.0)) {
        let assign36820_e47779: f64 = (var_wsrhstep * var_wsrhstep);
        let assign36820_e47781: f64 = (var_wsrhstep).ln();
        let assign36820_e47782: f64 = (assign36820_e47779 * assign36820_e47781);
        let assign36820_e47785: f64 = (1.0 - var_wsrhstep);
        let assign36820_e47786: f64 = (assign36820_e47782 / assign36820_e47785);
        let assign36820_e47788: f64 = (assign36820_e47786 + var_wsrhstep);
        let assign36820_e47792: f64 = (2.0 * var_pgatd_i);
        let assign36820_e47793: f64 = (1.0 - assign36820_e47792);
        let assign36820_e47794: f64 = (assign36820_e47788 * assign36820_e47793);
        (assign36820_e47794,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign36820_e47796;

        let (assign36830_e47810,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) {
        let assign36830_e47808: f64 = (var_wsrhstep + var_dwsrh);
        (assign36830_e47808,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign36830_e47810;

        let assign36840_e47813: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard742 = assign36840_e47813;

        let (assign36850_e47830, assign36850_e47830_d_n5, assign36850_e47830_d_n6, assign36850_e47830_d_n7, assign36850_e47830_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) && (var_guard742 != 0.0)) {
        let assign36850_e47827: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign36850_e47828: f64 = (assign36850_e47827).sqrt();
        (assign36850_e47828, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36850_e47830;
        var_tmp_dn5 = assign36850_e47830_d_n5;
        var_tmp_dn6 = assign36850_e47830_d_n6;
        var_tmp_dn7 = assign36850_e47830_d_n7;
        var_tmp_dn8 = assign36850_e47830_d_n8;

        let (assign36860_e47849, assign36860_e47849_d_n5, assign36860_e47849_d_n6, assign36860_e47849_d_n7, assign36860_e47849_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36860_e47845: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign36860_e47847: f64 = (assign36860_e47845).powf(var_pgatd_i);
        (assign36860_e47847, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36860_e47849;
        var_tmp_dn5 = assign36860_e47849_d_n5;
        var_tmp_dn6 = assign36860_e47849_d_n6;
        var_tmp_dn7 = assign36860_e47849_d_n7;
        var_tmp_dn8 = assign36860_e47849_d_n8;

        let (assign36870_e47863, assign36870_e47863_d_n5, assign36870_e47863_d_n6, assign36870_e47863_d_n7, assign36870_e47863_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) {
        let assign36870_e47861: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign36870_e47861, (var_wdepnulrgat_d * var_tmp_dn5), (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign36870_e47863;
        var_wdep_dn5 = assign36870_e47863_d_n5;
        var_wdep_dn6 = assign36870_e47863_d_n6;
        var_wdep_dn7 = assign36870_e47863_d_n7;
        var_wdep_dn8 = assign36870_e47863_d_n8;

        let (assign36880_e47881, assign36880_e47881_d_n5, assign36880_e47881_d_n6, assign36880_e47881_d_n7, assign36880_e47881_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) {
        let assign36880_e47876: f64 = (var_zinv - 1.0);
        let assign36880_e47878: f64 = (assign36880_e47876 * var_wdep);
        let assign36880_e47879: f64 = (var_ftdgat_d * assign36880_e47878);
        (assign36880_e47879, (var_ftdgat_d * (assign36880_e47876 * var_wdep_dn5)), (var_ftdgat_d * (assign36880_e47876 * var_wdep_dn6)), (var_ftdgat_d * (assign36880_e47876 * var_wdep_dn7)), (var_ftdgat_d * (assign36880_e47876 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign36880_e47881;
        var_asrh_dn5 = assign36880_e47881_d_n5;
        var_asrh_dn6 = assign36880_e47881_d_n6;
        var_asrh_dn7 = assign36880_e47881_d_n7;
        var_asrh_dn8 = assign36880_e47881_d_n8;

        let (assign36890_e47897, assign36890_e47897_d_n5, assign36890_e47897_d_n6, assign36890_e47897_d_n7, assign36890_e47897_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) {
        let assign36890_e47894: f64 = (var_asrh * var_wsrh);
        let assign36890_e47895: f64 = (var_csrhgatd_i * assign36890_e47894);
        (assign36890_e47895, (var_csrhgatd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign36890_e47897;
        var_isrh_dn5 = assign36890_e47897_d_n5;
        var_isrh_dn6 = assign36890_e47897_d_n6;
        var_isrh_dn7 = assign36890_e47897_d_n7;
        var_isrh_dn8 = assign36890_e47897_d_n8;

        let assign36900_e47900: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard743 = assign36900_e47900;

        let (assign36910_e47911, assign36910_e47911_d_n5, assign36910_e47911_d_n6, assign36910_e47911_d_n7, assign36910_e47911_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign36910_e47911;
        var_itat_dn5 = assign36910_e47911_d_n5;
        var_itat_dn6 = assign36910_e47911_d_n6;
        var_itat_dn7 = assign36910_e47911_d_n7;
        var_itat_dn8 = assign36910_e47911_d_n8;

        let (assign36920_e47929, assign36920_e47929_d_n5, assign36920_e47929_d_n6, assign36920_e47929_d_n7, assign36920_e47929_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign36920_e47924: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign36920_e47926: f64 = (assign36920_e47924 / var_vbi_minus_vjsrh);
        let assign36920_e47927: f64 = (var_btatpartgat_d * assign36920_e47926);
        (assign36920_e47927, (var_btatpartgat_d * ((var_wdep_dn5 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign36920_e47929;
        var_btat_dn5 = assign36920_e47929_d_n5;
        var_btat_dn6 = assign36920_e47929_d_n6;
        var_btat_dn7 = assign36920_e47929_d_n7;
        var_btat_dn8 = assign36920_e47929_d_n8;

        let (assign36930_e47945, assign36930_e47945_d_n5, assign36930_e47945_d_n6, assign36930_e47945_d_n7, assign36930_e47945_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign36930_e47941: f64 = (0.666666666666667 * var_atatgat_d);
        let assign36930_e47943: f64 = (assign36930_e47941 / var_btat);
        (assign36930_e47943, (-((assign36930_e47941 * var_btat_dn5) / (var_btat * var_btat))), (-((assign36930_e47941 * var_btat_dn6) / (var_btat * var_btat))), (-((assign36930_e47941 * var_btat_dn7) / (var_btat * var_btat))), (-((assign36930_e47941 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign36930_e47945;
        var_twoatatoverthreebtat_dn5 = assign36930_e47945_d_n5;
        var_twoatatoverthreebtat_dn6 = assign36930_e47945_d_n6;
        var_twoatatoverthreebtat_dn7 = assign36930_e47945_d_n7;
        var_twoatatoverthreebtat_dn8 = assign36930_e47945_d_n8;

        let (assign36940_e47959, assign36940_e47959_d_n5, assign36940_e47959_d_n6, assign36940_e47959_d_n7, assign36940_e47959_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign36940_e47957: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign36940_e47957, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign36940_e47959;
        var_umaxbeforelimiting_dn5 = assign36940_e47959_d_n5;
        var_umaxbeforelimiting_dn6 = assign36940_e47959_d_n6;
        var_umaxbeforelimiting_dn7 = assign36940_e47959_d_n7;
        var_umaxbeforelimiting_dn8 = assign36940_e47959_d_n8;

        let (assign36950_e47980, assign36950_e47980_d_n5, assign36950_e47980_d_n6, assign36950_e47980_d_n7, assign36950_e47980_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign36950_e47971: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign36950_e47974: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign36950_e47976: f64 = (assign36950_e47974 + 1.0);
        let assign36950_e47977: f64 = (assign36950_e47971 / assign36950_e47976);
        let assign36950_e47978: f64 = (assign36950_e47977).sqrt();
        (assign36950_e47978, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign36950_e47976) - (assign36950_e47971 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign36950_e47976 * assign36950_e47976)) / (2.0 * assign36950_e47978)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign36950_e47976) - (assign36950_e47971 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign36950_e47976 * assign36950_e47976)) / (2.0 * assign36950_e47978)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign36950_e47976) - (assign36950_e47971 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign36950_e47976 * assign36950_e47976)) / (2.0 * assign36950_e47978)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign36950_e47976) - (assign36950_e47971 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign36950_e47976 * assign36950_e47976)) / (2.0 * assign36950_e47978)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign36950_e47980;
        var_umax_dn5 = assign36950_e47980_d_n5;
        var_umax_dn6 = assign36950_e47980_d_n6;
        var_umax_dn7 = assign36950_e47980_d_n7;
        var_umax_dn8 = assign36950_e47980_d_n8;

        let (assign36960_e47993, assign36960_e47993_d_n5, assign36960_e47993_d_n6, assign36960_e47993_d_n7, assign36960_e47993_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign36960_e47991: f64 = (var_umax).sqrt();
        (assign36960_e47991, (var_umax_dn5 / (2.0 * assign36960_e47991)), (var_umax_dn6 / (2.0 * assign36960_e47991)), (var_umax_dn7 / (2.0 * assign36960_e47991)), (var_umax_dn8 / (2.0 * assign36960_e47991)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign36960_e47993;
        var_sqrtumax_dn5 = assign36960_e47993_d_n5;
        var_sqrtumax_dn6 = assign36960_e47993_d_n6;
        var_sqrtumax_dn7 = assign36960_e47993_d_n7;
        var_sqrtumax_dn8 = assign36960_e47993_d_n8;

        let (assign36970_e48007, assign36970_e48007_d_n5, assign36970_e48007_d_n6, assign36970_e48007_d_n7, assign36970_e48007_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign36970_e48005: f64 = (var_umax * var_sqrtumax);
        (assign36970_e48005, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign36970_e48007;
        var_umaxpoweronepointfive_dn5 = assign36970_e48007_d_n5;
        var_umaxpoweronepointfive_dn6 = assign36970_e48007_d_n6;
        var_umaxpoweronepointfive_dn7 = assign36970_e48007_d_n7;
        var_umaxpoweronepointfive_dn8 = assign36970_e48007_d_n8;

        let assign36980_e48009: f64 = (-var_pgatd_i);
        let assign36980_e48011: f64 = (assign36980_e48009 * var_one_over_one_minus_pgat_d);
        let assign36980_e48013: f64 = (-1.0);
        let assign36980_e48014: f64 = if assign36980_e48011 == assign36980_e48013 { 1.0 } else { 0.0 };
        var_guard744 = assign36980_e48014;

        let (assign36990_e48034, assign36990_e48034_d_n5, assign36990_e48034_d_n6, assign36990_e48034_d_n7, assign36990_e48034_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) && (var_guard744 != 0.0)) {
        let assign36990_e48030: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign36990_e48031: f64 = (1.0 + assign36990_e48030);
        let assign36990_e48032: f64 = (1.0 / assign36990_e48031);
        (assign36990_e48032, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign36990_e48031 * assign36990_e48031))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign36990_e48031 * assign36990_e48031))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign36990_e48031 * assign36990_e48031))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign36990_e48031 * assign36990_e48031))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign36990_e48034;
        var_wgamma_dn5 = assign36990_e48034_d_n5;
        var_wgamma_dn6 = assign36990_e48034_d_n6;
        var_wgamma_dn7 = assign36990_e48034_d_n7;
        var_wgamma_dn8 = assign36990_e48034_d_n8;

        let (assign37000_e48058, assign37000_e48058_d_n5, assign37000_e48058_d_n6, assign37000_e48058_d_n7, assign37000_e48058_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) && (var_guard744 == 0.0)) {
        let assign37000_e48050: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37000_e48051: f64 = (1.0 + assign37000_e48050);
        let assign37000_e48053: f64 = (-var_pgatd_i);
        let assign37000_e48055: f64 = (assign37000_e48053 * var_one_over_one_minus_pgat_d);
        let assign37000_e48056: f64 = (assign37000_e48051).powf(assign37000_e48055);
        (assign37000_e48056, if 0.0 == 0.0 && ((assign37000_e48055) as f64).is_finite() && ((assign37000_e48055) as f64).fract() == 0.0 { if assign37000_e48055 == 0.0 { 0.0 } else { (assign37000_e48055 * ((assign37000_e48051).powf(assign37000_e48055 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign37000_e48056 * (assign37000_e48055 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign37000_e48051))) }, if 0.0 == 0.0 && ((assign37000_e48055) as f64).is_finite() && ((assign37000_e48055) as f64).fract() == 0.0 { if assign37000_e48055 == 0.0 { 0.0 } else { (assign37000_e48055 * ((assign37000_e48051).powf(assign37000_e48055 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign37000_e48056 * (assign37000_e48055 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign37000_e48051))) }, if 0.0 == 0.0 && ((assign37000_e48055) as f64).is_finite() && ((assign37000_e48055) as f64).fract() == 0.0 { if assign37000_e48055 == 0.0 { 0.0 } else { (assign37000_e48055 * ((assign37000_e48051).powf(assign37000_e48055 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign37000_e48056 * (assign37000_e48055 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign37000_e48051))) }, if 0.0 == 0.0 && ((assign37000_e48055) as f64).is_finite() && ((assign37000_e48055) as f64).fract() == 0.0 { if assign37000_e48055 == 0.0 { 0.0 } else { (assign37000_e48055 * ((assign37000_e48051).powf(assign37000_e48055 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign37000_e48056 * (assign37000_e48055 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign37000_e48051))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign37000_e48058;
        var_wgamma_dn5 = assign37000_e48058_d_n5;
        var_wgamma_dn6 = assign37000_e48058_d_n6;
        var_wgamma_dn7 = assign37000_e48058_d_n7;
        var_wgamma_dn8 = assign37000_e48058_d_n8;

        let (assign37010_e48076, assign37010_e48076_d_n5, assign37010_e48076_d_n6, assign37010_e48076_d_n7, assign37010_e48076_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign37010_e48070: f64 = (var_wsrh * var_wgamma);
        let assign37010_e48073: f64 = (var_wsrh + var_wgamma);
        let assign37010_e48074: f64 = (assign37010_e48070 / assign37010_e48073);
        (assign37010_e48074, ((((var_wsrh * var_wgamma_dn5) * assign37010_e48073) - (assign37010_e48070 * var_wgamma_dn5)) / (assign37010_e48073 * assign37010_e48073)), ((((var_wsrh * var_wgamma_dn6) * assign37010_e48073) - (assign37010_e48070 * var_wgamma_dn6)) / (assign37010_e48073 * assign37010_e48073)), ((((var_wsrh * var_wgamma_dn7) * assign37010_e48073) - (assign37010_e48070 * var_wgamma_dn7)) / (assign37010_e48073 * assign37010_e48073)), ((((var_wsrh * var_wgamma_dn8) * assign37010_e48073) - (assign37010_e48070 * var_wgamma_dn8)) / (assign37010_e48073 * assign37010_e48073)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign37010_e48076;
        var_wtat_dn5 = assign37010_e48076_d_n5;
        var_wtat_dn6 = assign37010_e48076_d_n6;
        var_wtat_dn7 = assign37010_e48076_d_n7;
        var_wtat_dn8 = assign37010_e48076_d_n8;

        let (assign37020_e48093, assign37020_e48093_d_n5, assign37020_e48093_d_n6, assign37020_e48093_d_n7, assign37020_e48093_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign37020_e48089: f64 = (var_btat / var_sqrtumax);
        let assign37020_e48090: f64 = (0.375 * assign37020_e48089);
        let assign37020_e48091: f64 = (assign37020_e48090).sqrt();
        (assign37020_e48091, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37020_e48091)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37020_e48091)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37020_e48091)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37020_e48091)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign37020_e48093;
        var_ktat_dn5 = assign37020_e48093_d_n5;
        var_ktat_dn6 = assign37020_e48093_d_n6;
        var_ktat_dn7 = assign37020_e48093_d_n7;
        var_ktat_dn8 = assign37020_e48093_d_n8;

        let (assign37030_e48111, assign37030_e48111_d_n5, assign37030_e48111_d_n6, assign37030_e48111_d_n7, assign37030_e48111_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign37030_e48106: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign37030_e48107: f64 = (2.0 * assign37030_e48106);
        let assign37030_e48109: f64 = (assign37030_e48107 - var_umax);
        (assign37030_e48109, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign37030_e48111;
        var_ltat_dn5 = assign37030_e48111_d_n5;
        var_ltat_dn6 = assign37030_e48111_d_n6;
        var_ltat_dn7 = assign37030_e48111_d_n7;
        var_ltat_dn8 = assign37030_e48111_d_n8;

        let (assign37040_e48137, assign37040_e48137_d_n5, assign37040_e48137_d_n6, assign37040_e48137_d_n7, assign37040_e48137_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign37040_e48123: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign37040_e48125: f64 = (assign37040_e48123 * var_sqrtumax);
        let assign37040_e48128: f64 = (var_atatgat_d * var_umax);
        let assign37040_e48129: f64 = (assign37040_e48125 - assign37040_e48128);
        let assign37040_e48133: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37040_e48134: f64 = (0.5 * assign37040_e48133);
        let assign37040_e48135: f64 = (assign37040_e48129 + assign37040_e48134);
        (assign37040_e48135, (((((var_atatgat_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign37040_e48123 * var_sqrtumax_dn5)) - (var_atatgat_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign37040_e48123 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign37040_e48123 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign37040_e48123 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign37040_e48137;
        var_mtat_dn5 = assign37040_e48137_d_n5;
        var_mtat_dn6 = assign37040_e48137_d_n6;
        var_mtat_dn7 = assign37040_e48137_d_n7;
        var_mtat_dn8 = assign37040_e48137_d_n8;

        let (assign37050_e48153, assign37050_e48153_d_n5, assign37050_e48153_d_n6, assign37050_e48153_d_n7, assign37050_e48153_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign37050_e48149: f64 = (var_ltat - 1.0);
        let assign37050_e48151: f64 = (assign37050_e48149 * var_ktat);
        (assign37050_e48151, ((var_ltat_dn5 * var_ktat) + (assign37050_e48149 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign37050_e48149 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign37050_e48149 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign37050_e48149 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign37050_e48153;
        var_xerfc_dn5 = assign37050_e48153_d_n5;
        var_xerfc_dn6 = assign37050_e48153_d_n6;
        var_xerfc_dn7 = assign37050_e48153_d_n7;
        var_xerfc_dn8 = assign37050_e48153_d_n8;

        let (assign37060_e48167, assign37060_e48167_d_n5, assign37060_e48167_d_n6, assign37060_e48167_d_n7, assign37060_e48167_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign37060_e48165: f64 = (var_xerfc * var_xerfc);
        (assign37060_e48165, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign37060_e48167;
        var_ysq_dn5 = assign37060_e48167_d_n5;
        var_ysq_dn6 = assign37060_e48167_d_n6;
        var_ysq_dn7 = assign37060_e48167_d_n7;
        var_ysq_dn8 = assign37060_e48167_d_n8;

        let assign37070_e48170: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard745 = assign37070_e48170;

        let (assign37080_e48190, assign37080_e48190_d_n5, assign37080_e48190_d_n6, assign37080_e48190_d_n7, assign37080_e48190_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) && (var_guard745 != 0.0)) {
        let assign37080_e48186: f64 = (var_perfc * var_xerfc);
        let assign37080_e48187: f64 = (1.0 + assign37080_e48186);
        let assign37080_e48188: f64 = (1.0 / assign37080_e48187);
        (assign37080_e48188, (-((var_perfc * var_xerfc_dn5) / (assign37080_e48187 * assign37080_e48187))), (-((var_perfc * var_xerfc_dn6) / (assign37080_e48187 * assign37080_e48187))), (-((var_perfc * var_xerfc_dn7) / (assign37080_e48187 * assign37080_e48187))), (-((var_perfc * var_xerfc_dn8) / (assign37080_e48187 * assign37080_e48187))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign37080_e48190;
        var_terfc_dn5 = assign37080_e48190_d_n5;
        var_terfc_dn6 = assign37080_e48190_d_n6;
        var_terfc_dn7 = assign37080_e48190_d_n7;
        var_terfc_dn8 = assign37080_e48190_d_n8;

        let (assign37090_e48211, assign37090_e48211_d_n5, assign37090_e48211_d_n6, assign37090_e48211_d_n7, assign37090_e48211_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) && (var_guard745 == 0.0)) {
        let assign37090_e48207: f64 = (var_perfc * var_xerfc);
        let assign37090_e48208: f64 = (1.0 - assign37090_e48207);
        let assign37090_e48209: f64 = (1.0 / assign37090_e48208);
        (assign37090_e48209, (-((-(var_perfc * var_xerfc_dn5)) / (assign37090_e48208 * assign37090_e48208))), (-((-(var_perfc * var_xerfc_dn6)) / (assign37090_e48208 * assign37090_e48208))), (-((-(var_perfc * var_xerfc_dn7)) / (assign37090_e48208 * assign37090_e48208))), (-((-(var_perfc * var_xerfc_dn8)) / (assign37090_e48208 * assign37090_e48208))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign37090_e48211;
        var_terfc_dn5 = assign37090_e48211_d_n5;
        var_terfc_dn6 = assign37090_e48211_d_n6;
        var_terfc_dn7 = assign37090_e48211_d_n7;
        var_terfc_dn8 = assign37090_e48211_d_n8;

        let assign37100_e48213: f64 = (-var_ysq);
        let assign37100_e48215: f64 = (assign37100_e48213 + var_mtat);
        let assign37100_e48217: f64 = (-230.25850929940458);
        let assign37100_e48218: f64 = if assign37100_e48215 > assign37100_e48217 { 1.0 } else { 0.0 };
        var_guard746 = assign37100_e48218;

        let (assign37110_e48236, assign37110_e48236_d_n5, assign37110_e48236_d_n6, assign37110_e48236_d_n7, assign37110_e48236_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) && (var_guard746 != 0.0)) {
        let assign37110_e48231: f64 = (-var_ysq);
        let assign37110_e48233: f64 = (assign37110_e48231 + var_mtat);
        let assign37110_e48234: f64 = (assign37110_e48233).exp();
        (assign37110_e48234, (assign37110_e48234 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign37110_e48234 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign37110_e48234 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign37110_e48234 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37110_e48236;
        var_tmp_dn5 = assign37110_e48236_d_n5;
        var_tmp_dn6 = assign37110_e48236_d_n6;
        var_tmp_dn7 = assign37110_e48236_d_n7;
        var_tmp_dn8 = assign37110_e48236_d_n8;

        let (assign37120_e48285, assign37120_e48285_d_n5, assign37120_e48285_d_n6, assign37120_e48285_d_n7, assign37120_e48285_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) && (var_guard746 == 0.0)) {
        let assign37120_e48252: f64 = (-230.25850929940458);
        let assign37120_e48254: f64 = (-var_ysq);
        let assign37120_e48256: f64 = (assign37120_e48254 + var_mtat);
        let assign37120_e48257: f64 = (assign37120_e48252 - assign37120_e48256);
        let assign37120_e48261: f64 = (-230.25850929940458);
        let assign37120_e48263: f64 = (-var_ysq);
        let assign37120_e48265: f64 = (assign37120_e48263 + var_mtat);
        let assign37120_e48266: f64 = (assign37120_e48261 - assign37120_e48265);
        let assign37120_e48269: f64 = (-230.25850929940458);
        let assign37120_e48271: f64 = (-var_ysq);
        let assign37120_e48273: f64 = (assign37120_e48271 + var_mtat);
        let assign37120_e48274: f64 = (assign37120_e48269 - assign37120_e48273);
        let assign37120_e48276: f64 = (assign37120_e48274 * 0.3333333333333333);
        let assign37120_e48277: f64 = (1.0 + assign37120_e48276);
        let assign37120_e48278: f64 = (assign37120_e48266 * assign37120_e48277);
        let assign37120_e48279: f64 = (0.5 * assign37120_e48278);
        let assign37120_e48280: f64 = (1.0 + assign37120_e48279);
        let assign37120_e48281: f64 = (assign37120_e48257 * assign37120_e48280);
        let assign37120_e48282: f64 = (1.0 + assign37120_e48281);
        let assign37120_e48283: f64 = (1e-100 / assign37120_e48282);
        (assign37120_e48283, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign37120_e48280) + (assign37120_e48257 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign37120_e48277) + (assign37120_e48266 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign37120_e48282 * assign37120_e48282))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign37120_e48280) + (assign37120_e48257 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign37120_e48277) + (assign37120_e48266 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign37120_e48282 * assign37120_e48282))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign37120_e48280) + (assign37120_e48257 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign37120_e48277) + (assign37120_e48266 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign37120_e48282 * assign37120_e48282))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign37120_e48280) + (assign37120_e48257 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign37120_e48277) + (assign37120_e48266 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign37120_e48282 * assign37120_e48282))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37120_e48285;
        var_tmp_dn5 = assign37120_e48285_d_n5;
        var_tmp_dn6 = assign37120_e48285_d_n6;
        var_tmp_dn7 = assign37120_e48285_d_n7;
        var_tmp_dn8 = assign37120_e48285_d_n8;

        let (assign37130_e48315, assign37130_e48315_d_n5, assign37130_e48315_d_n6, assign37130_e48315_d_n7, assign37130_e48315_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign37130_e48297: f64 = (0.29214664 * var_terfc);
        let assign37130_e48301: f64 = (var_terfc * var_terfc);
        let assign37130_e48302: f64 = (var_berfc * assign37130_e48301);
        let assign37130_e48303: f64 = (assign37130_e48297 + assign37130_e48302);
        let assign37130_e48307: f64 = (var_terfc * var_terfc);
        let assign37130_e48309: f64 = (assign37130_e48307 * var_terfc);
        let assign37130_e48310: f64 = (var_cerfc * assign37130_e48309);
        let assign37130_e48311: f64 = (assign37130_e48303 + assign37130_e48310);
        let assign37130_e48313: f64 = (assign37130_e48311 * var_tmp);
        (assign37130_e48313, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign37130_e48307 * var_terfc_dn5)))) * var_tmp) + (assign37130_e48311 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign37130_e48307 * var_terfc_dn6)))) * var_tmp) + (assign37130_e48311 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign37130_e48307 * var_terfc_dn7)))) * var_tmp) + (assign37130_e48311 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign37130_e48307 * var_terfc_dn8)))) * var_tmp) + (assign37130_e48311 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign37130_e48315;
        var_erfcpos_dn5 = assign37130_e48315_d_n5;
        var_erfcpos_dn6 = assign37130_e48315_d_n6;
        var_erfcpos_dn7 = assign37130_e48315_d_n7;
        var_erfcpos_dn8 = assign37130_e48315_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_guard741_slot = var_guard741;
        *var_guard742_slot = var_guard742;
        *var_guard743_slot = var_guard743;
        *var_guard744_slot = var_guard744;
        *var_guard745_slot = var_guard745;
        *var_guard746_slot = var_guard746;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_78(
        p: &Parameters,
        var_abdrain_i: f64,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat_d: f64,
        var_cbbtgatd_i: f64,
        var_ctatgatd_i: f64,
        var_erfcpos: f64,
        var_erfcpos_dn5: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn5: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fstopgat_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard739: f64,
        var_guard743: f64,
        var_id__blk213: f64,
        var_ijunbot: f64,
        var_ijunbot_dn5: f64,
        var_ijunbot_dn6: f64,
        var_ijunbot_dn7: f64,
        var_ijunbot_dn8: f64,
        var_ijunsti: f64,
        var_ijunsti_dn5: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_isrh: f64,
        var_isrh_dn5: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
        var_pgatd_i: f64,
        var_phitdinv: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn5: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_v4: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_vbrgatd_i: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn5: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_vmax_d: f64,
        var_wdepnulrinvgat_d: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_xerfc: f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard747_slot: &mut f64,
        var_guard748_slot: &mut f64,
        var_guard749_slot: &mut f64,
        var_guard750_slot: &mut f64,
        var_guard751_slot: &mut f64,
        var_guard752_slot: &mut f64,
        var_guard753_slot: &mut f64,
        var_guard754_slot: &mut f64,
        var_guard755_slot: &mut f64,
        var_guard756_slot: &mut f64,
        var_guard757_slot: &mut f64,
        var_guard758_slot: &mut f64,
        var_guard759_slot: &mut f64,
        var_i4_slot: &mut f64,
        var_i4_dn5_slot: &mut f64,
        var_i4_dn6_slot: &mut f64,
        var_i4_dn7_slot: &mut f64,
        var_i4_dn8_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_z_slot: &mut f64,
    ) {
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard747: f64 = *var_guard747_slot;
        let mut var_guard748: f64 = *var_guard748_slot;
        let mut var_guard749: f64 = *var_guard749_slot;
        let mut var_guard750: f64 = *var_guard750_slot;
        let mut var_guard751: f64 = *var_guard751_slot;
        let mut var_guard752: f64 = *var_guard752_slot;
        let mut var_guard753: f64 = *var_guard753_slot;
        let mut var_guard754: f64 = *var_guard754_slot;
        let mut var_guard755: f64 = *var_guard755_slot;
        let mut var_guard756: f64 = *var_guard756_slot;
        let mut var_guard757: f64 = *var_guard757_slot;
        let mut var_guard758: f64 = *var_guard758_slot;
        let mut var_guard759: f64 = *var_guard759_slot;
        let mut var_i4: f64 = *var_i4_slot;
        let mut var_i4_dn5: f64 = *var_i4_dn5_slot;
        let mut var_i4_dn6: f64 = *var_i4_dn6_slot;
        let mut var_i4_dn7: f64 = *var_i4_dn7_slot;
        let mut var_i4_dn8: f64 = *var_i4_dn8_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_z: f64 = *var_z_slot;

        let assign37140_e48318: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard747 = assign37140_e48318;

        let (assign37150_e48332, assign37150_e48332_d_n5, assign37150_e48332_d_n6, assign37150_e48332_d_n7, assign37150_e48332_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) && (var_guard747 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign37150_e48332;
        var_erfctimesexpmtat_dn5 = assign37150_e48332_d_n5;
        var_erfctimesexpmtat_dn6 = assign37150_e48332_d_n6;
        var_erfctimesexpmtat_dn7 = assign37150_e48332_d_n7;
        var_erfctimesexpmtat_dn8 = assign37150_e48332_d_n8;

        let assign37160_e48335: f64 = (-230.25850929940458);
        let assign37160_e48336: f64 = if var_mtat > assign37160_e48335 { 1.0 } else { 0.0 };
        var_guard748 = assign37160_e48336;

        let (assign37170_e48354, assign37170_e48354_d_n5, assign37170_e48354_d_n6, assign37170_e48354_d_n7, assign37170_e48354_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) && (var_guard747 == 0.0)) && (var_guard748 != 0.0)) {
        let assign37170_e48352: f64 = (var_mtat).exp();
        (assign37170_e48352, (assign37170_e48352 * var_mtat_dn5), (assign37170_e48352 * var_mtat_dn6), (assign37170_e48352 * var_mtat_dn7), (assign37170_e48352 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37170_e48354;
        var_tmp_dn5 = assign37170_e48354_d_n5;
        var_tmp_dn6 = assign37170_e48354_d_n6;
        var_tmp_dn7 = assign37170_e48354_d_n7;
        var_tmp_dn8 = assign37170_e48354_d_n8;

        let (assign37180_e48397, assign37180_e48397_d_n5, assign37180_e48397_d_n6, assign37180_e48397_d_n7, assign37180_e48397_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) && (var_guard747 == 0.0)) && (var_guard748 == 0.0)) {
        let assign37180_e48373: f64 = (-230.25850929940458);
        let assign37180_e48375: f64 = (assign37180_e48373 - var_mtat);
        let assign37180_e48379: f64 = (-230.25850929940458);
        let assign37180_e48381: f64 = (assign37180_e48379 - var_mtat);
        let assign37180_e48384: f64 = (-230.25850929940458);
        let assign37180_e48386: f64 = (assign37180_e48384 - var_mtat);
        let assign37180_e48388: f64 = (assign37180_e48386 * 0.3333333333333333);
        let assign37180_e48389: f64 = (1.0 + assign37180_e48388);
        let assign37180_e48390: f64 = (assign37180_e48381 * assign37180_e48389);
        let assign37180_e48391: f64 = (0.5 * assign37180_e48390);
        let assign37180_e48392: f64 = (1.0 + assign37180_e48391);
        let assign37180_e48393: f64 = (assign37180_e48375 * assign37180_e48392);
        let assign37180_e48394: f64 = (1.0 + assign37180_e48393);
        let assign37180_e48395: f64 = (1e-100 / assign37180_e48394);
        (assign37180_e48395, (-((1e-100 * (((-var_mtat_dn5) * assign37180_e48392) + (assign37180_e48375 * (0.5 * (((-var_mtat_dn5) * assign37180_e48389) + (assign37180_e48381 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign37180_e48394 * assign37180_e48394))), (-((1e-100 * (((-var_mtat_dn6) * assign37180_e48392) + (assign37180_e48375 * (0.5 * (((-var_mtat_dn6) * assign37180_e48389) + (assign37180_e48381 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign37180_e48394 * assign37180_e48394))), (-((1e-100 * (((-var_mtat_dn7) * assign37180_e48392) + (assign37180_e48375 * (0.5 * (((-var_mtat_dn7) * assign37180_e48389) + (assign37180_e48381 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign37180_e48394 * assign37180_e48394))), (-((1e-100 * (((-var_mtat_dn8) * assign37180_e48392) + (assign37180_e48375 * (0.5 * (((-var_mtat_dn8) * assign37180_e48389) + (assign37180_e48381 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign37180_e48394 * assign37180_e48394))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37180_e48397;
        var_tmp_dn5 = assign37180_e48397_d_n5;
        var_tmp_dn6 = assign37180_e48397_d_n6;
        var_tmp_dn7 = assign37180_e48397_d_n7;
        var_tmp_dn8 = assign37180_e48397_d_n8;

        let (assign37190_e48416, assign37190_e48416_d_n5, assign37190_e48416_d_n6, assign37190_e48416_d_n7, assign37190_e48416_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) && (var_guard747 == 0.0)) {
        let assign37190_e48412: f64 = (2.0 * var_tmp);
        let assign37190_e48414: f64 = (assign37190_e48412 - var_erfcpos);
        (assign37190_e48414, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign37190_e48416;
        var_erfctimesexpmtat_dn5 = assign37190_e48416_d_n5;
        var_erfctimesexpmtat_dn6 = assign37190_e48416_d_n6;
        var_erfctimesexpmtat_dn7 = assign37190_e48416_d_n7;
        var_erfctimesexpmtat_dn8 = assign37190_e48416_d_n8;

        let (assign37200_e48436, assign37200_e48436_d_n5, assign37200_e48436_d_n6, assign37200_e48436_d_n7, assign37200_e48436_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign37200_e48428: f64 = (1.772453850905516 * 0.5);
        let assign37200_e48431: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign37200_e48433: f64 = (assign37200_e48431 / var_ktat);
        let assign37200_e48434: f64 = (assign37200_e48428 * assign37200_e48433);
        (assign37200_e48434, (assign37200_e48428 * ((((var_atatgat_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign37200_e48431 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign37200_e48428 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign37200_e48431 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign37200_e48428 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign37200_e48431 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign37200_e48428 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign37200_e48431 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign37200_e48436;
        var_gammamax_dn5 = assign37200_e48436_d_n5;
        var_gammamax_dn6 = assign37200_e48436_d_n6;
        var_gammamax_dn7 = assign37200_e48436_d_n7;
        var_gammamax_dn8 = assign37200_e48436_d_n8;

        let (assign37210_e48454, assign37210_e48454_d_n5, assign37210_e48454_d_n6, assign37210_e48454_d_n7, assign37210_e48454_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard743 == 0.0)) {
        let assign37210_e48449: f64 = (var_asrh * var_gammamax);
        let assign37210_e48451: f64 = (assign37210_e48449 * var_wtat);
        let assign37210_e48452: f64 = (var_ctatgatd_i * assign37210_e48451);
        (assign37210_e48452, (var_ctatgatd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign37210_e48449 * var_wtat_dn5))), (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign37210_e48449 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign37210_e48449 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign37210_e48449 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign37210_e48454;
        var_itat_dn5 = assign37210_e48454_d_n5;
        var_itat_dn6 = assign37210_e48454_d_n6;
        var_itat_dn7 = assign37210_e48454_d_n7;
        var_itat_dn8 = assign37210_e48454_d_n8;

        let assign37220_e48457: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard749 = assign37220_e48457;

        let (assign37230_e48468, assign37230_e48468_d_n5, assign37230_e48468_d_n6, assign37230_e48468_d_n7, assign37230_e48468_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard749 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign37230_e48468;
        var_ibbt_dn5 = assign37230_e48468_d_n5;
        var_ibbt_dn6 = assign37230_e48468_d_n6;
        var_ibbt_dn7 = assign37230_e48468_d_n7;
        var_ibbt_dn8 = assign37230_e48468_d_n8;

        let assign37240_e48471: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard750 = assign37240_e48471;

        let (assign37250_e48490, assign37250_e48490_d_n5, assign37250_e48490_d_n6, assign37250_e48490_d_n7, assign37250_e48490_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard749 == 0.0)) && (var_guard750 != 0.0)) {
        let assign37250_e48485: f64 = (var_vbirgatd_i - var_vbbt);
        let assign37250_e48487: f64 = (assign37250_e48485 * var_vbirgatinv_d);
        let assign37250_e48488: f64 = (assign37250_e48487).sqrt();
        (assign37250_e48488, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37250_e48490;
        var_tmp_dn5 = assign37250_e48490_d_n5;
        var_tmp_dn6 = assign37250_e48490_d_n6;
        var_tmp_dn7 = assign37250_e48490_d_n7;
        var_tmp_dn8 = assign37250_e48490_d_n8;

        let (assign37260_e48511, assign37260_e48511_d_n5, assign37260_e48511_d_n6, assign37260_e48511_d_n7, assign37260_e48511_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard749 == 0.0)) && (var_guard750 == 0.0)) {
        let assign37260_e48505: f64 = (var_vbirgatd_i - var_vbbt);
        let assign37260_e48507: f64 = (assign37260_e48505 * var_vbirgatinv_d);
        let assign37260_e48509: f64 = (assign37260_e48507).powf(var_pgatd_i);
        (assign37260_e48509, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37260_e48511;
        var_tmp_dn5 = assign37260_e48511_d_n5;
        var_tmp_dn6 = assign37260_e48511_d_n6;
        var_tmp_dn7 = assign37260_e48511_d_n7;
        var_tmp_dn8 = assign37260_e48511_d_n8;

        let (assign37270_e48531, assign37270_e48531_d_n5, assign37270_e48531_d_n6, assign37270_e48531_d_n7, assign37270_e48531_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37270_e48524: f64 = (var_vbirgatd_i - var_vbbt);
        let assign37270_e48526: f64 = (assign37270_e48524 * var_wdepnulrinvgat_d);
        let assign37270_e48528: f64 = (assign37270_e48526 / var_tmp);
        let assign37270_e48529: f64 = (var_one_over_one_minus_pgat_d * assign37270_e48528);
        (assign37270_e48529, (var_one_over_one_minus_pgat_d * (-((assign37270_e48526 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign37270_e48526 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign37270_e48526 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign37270_e48526 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign37270_e48531;
        var_fmaxr_dn5 = assign37270_e48531_d_n5;
        var_fmaxr_dn6 = assign37270_e48531_d_n6;
        var_fmaxr_dn7 = assign37270_e48531_d_n7;
        var_fmaxr_dn8 = assign37270_e48531_d_n8;

        let assign37280_e48533: f64 = (-var_fbbtgat_d);
        let assign37280_e48535: f64 = (assign37280_e48533 / var_fmaxr);
        let assign37280_e48536: f64 = (assign37280_e48535).abs();
        let assign37280_e48538: f64 = if assign37280_e48536 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard751 = assign37280_e48538;

        let (assign37290_e48556, assign37290_e48556_d_n5, assign37290_e48556_d_n6, assign37290_e48556_d_n7, assign37290_e48556_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard749 == 0.0)) && (var_guard751 != 0.0)) {
        let assign37290_e48551: f64 = (-var_fbbtgat_d);
        let assign37290_e48553: f64 = (assign37290_e48551 / var_fmaxr);
        let assign37290_e48554: f64 = (assign37290_e48553).exp();
        (assign37290_e48554, (assign37290_e48554 * ((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37290_e48551 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign37290_e48554 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37290_e48551 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign37290_e48554 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37290_e48551 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign37290_e48554 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37290_e48551 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37290_e48556;
        var_tmp_dn5 = assign37290_e48556_d_n5;
        var_tmp_dn6 = assign37290_e48556_d_n6;
        var_tmp_dn7 = assign37290_e48556_d_n7;
        var_tmp_dn8 = assign37290_e48556_d_n8;

        let assign37300_e48558: f64 = (-var_fbbtgat_d);
        let assign37300_e48560: f64 = (assign37300_e48558 / var_fmaxr);
        let assign37300_e48562: f64 = if assign37300_e48560 < 0.0 { 1.0 } else { 0.0 };
        var_guard752 = assign37300_e48562;

        let (assign37310_e48613, assign37310_e48613_d_n5, assign37310_e48613_d_n6, assign37310_e48613_d_n7, assign37310_e48613_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard749 == 0.0)) && (var_guard751 == 0.0)) && (var_guard752 != 0.0)) {
        let assign37310_e48580: f64 = (-230.25850929940458);
        let assign37310_e48582: f64 = (-var_fbbtgat_d);
        let assign37310_e48584: f64 = (assign37310_e48582 / var_fmaxr);
        let assign37310_e48585: f64 = (assign37310_e48580 - assign37310_e48584);
        let assign37310_e48589: f64 = (-230.25850929940458);
        let assign37310_e48591: f64 = (-var_fbbtgat_d);
        let assign37310_e48593: f64 = (assign37310_e48591 / var_fmaxr);
        let assign37310_e48594: f64 = (assign37310_e48589 - assign37310_e48593);
        let assign37310_e48597: f64 = (-230.25850929940458);
        let assign37310_e48599: f64 = (-var_fbbtgat_d);
        let assign37310_e48601: f64 = (assign37310_e48599 / var_fmaxr);
        let assign37310_e48602: f64 = (assign37310_e48597 - assign37310_e48601);
        let assign37310_e48604: f64 = (assign37310_e48602 * 0.3333333333333333);
        let assign37310_e48605: f64 = (1.0 + assign37310_e48604);
        let assign37310_e48606: f64 = (assign37310_e48594 * assign37310_e48605);
        let assign37310_e48607: f64 = (0.5 * assign37310_e48606);
        let assign37310_e48608: f64 = (1.0 + assign37310_e48607);
        let assign37310_e48609: f64 = (assign37310_e48585 * assign37310_e48608);
        let assign37310_e48610: f64 = (1.0 + assign37310_e48609);
        let assign37310_e48611: f64 = (1e-100 / assign37310_e48610);
        (assign37310_e48611, (-((1e-100 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37310_e48582 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign37310_e48608) + (assign37310_e48585 * (0.5 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37310_e48591 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign37310_e48605) + (assign37310_e48594 * ((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37310_e48599 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign37310_e48610 * assign37310_e48610))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37310_e48582 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign37310_e48608) + (assign37310_e48585 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37310_e48591 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign37310_e48605) + (assign37310_e48594 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37310_e48599 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign37310_e48610 * assign37310_e48610))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37310_e48582 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign37310_e48608) + (assign37310_e48585 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37310_e48591 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign37310_e48605) + (assign37310_e48594 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37310_e48599 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign37310_e48610 * assign37310_e48610))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37310_e48582 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign37310_e48608) + (assign37310_e48585 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37310_e48591 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign37310_e48605) + (assign37310_e48594 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37310_e48599 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign37310_e48610 * assign37310_e48610))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37310_e48613;
        var_tmp_dn5 = assign37310_e48613_d_n5;
        var_tmp_dn6 = assign37310_e48613_d_n6;
        var_tmp_dn7 = assign37310_e48613_d_n7;
        var_tmp_dn8 = assign37310_e48613_d_n8;

        let (assign37320_e48662, assign37320_e48662_d_n5, assign37320_e48662_d_n6, assign37320_e48662_d_n7, assign37320_e48662_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard749 == 0.0)) && (var_guard751 == 0.0)) && (var_guard752 == 0.0)) {
        let assign37320_e48632: f64 = (-var_fbbtgat_d);
        let assign37320_e48634: f64 = (assign37320_e48632 / var_fmaxr);
        let assign37320_e48636: f64 = (assign37320_e48634 - 230.25850929940458);
        let assign37320_e48640: f64 = (-var_fbbtgat_d);
        let assign37320_e48642: f64 = (assign37320_e48640 / var_fmaxr);
        let assign37320_e48644: f64 = (assign37320_e48642 - 230.25850929940458);
        let assign37320_e48647: f64 = (-var_fbbtgat_d);
        let assign37320_e48649: f64 = (assign37320_e48647 / var_fmaxr);
        let assign37320_e48651: f64 = (assign37320_e48649 - 230.25850929940458);
        let assign37320_e48653: f64 = (assign37320_e48651 * 0.3333333333333333);
        let assign37320_e48654: f64 = (1.0 + assign37320_e48653);
        let assign37320_e48655: f64 = (assign37320_e48644 * assign37320_e48654);
        let assign37320_e48656: f64 = (0.5 * assign37320_e48655);
        let assign37320_e48657: f64 = (1.0 + assign37320_e48656);
        let assign37320_e48658: f64 = (assign37320_e48636 * assign37320_e48657);
        let assign37320_e48659: f64 = (1.0 + assign37320_e48658);
        let assign37320_e48660: f64 = (1e100 * assign37320_e48659);
        (assign37320_e48660, (1e100 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37320_e48632 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign37320_e48657) + (assign37320_e48636 * (0.5 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37320_e48640 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign37320_e48654) + (assign37320_e48644 * (((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37320_e48647 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37320_e48632 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign37320_e48657) + (assign37320_e48636 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37320_e48640 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign37320_e48654) + (assign37320_e48644 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37320_e48647 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37320_e48632 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign37320_e48657) + (assign37320_e48636 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37320_e48640 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign37320_e48654) + (assign37320_e48644 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37320_e48647 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37320_e48632 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign37320_e48657) + (assign37320_e48636 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37320_e48640 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign37320_e48654) + (assign37320_e48644 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37320_e48647 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37320_e48662;
        var_tmp_dn5 = assign37320_e48662_d_n5;
        var_tmp_dn6 = assign37320_e48662_d_n6;
        var_tmp_dn7 = assign37320_e48662_d_n7;
        var_tmp_dn8 = assign37320_e48662_d_n8;

        let (assign37330_e48682, assign37330_e48682_d_n5, assign37330_e48682_d_n6, assign37330_e48682_d_n7, assign37330_e48682_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37330_e48675: f64 = (var_v4 * var_fmaxr);
        let assign37330_e48677: f64 = (assign37330_e48675 * var_fmaxr);
        let assign37330_e48679: f64 = (assign37330_e48677 * var_tmp);
        let assign37330_e48680: f64 = (var_cbbtgatd_i * assign37330_e48679);
        (assign37330_e48680, (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign37330_e48675 * var_fmaxr_dn5)) * var_tmp) + (assign37330_e48677 * var_tmp_dn5))), (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign37330_e48675 * var_fmaxr_dn6)) * var_tmp) + (assign37330_e48677 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign37330_e48675 * var_fmaxr_dn7)) * var_tmp) + (assign37330_e48677 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign37330_e48675 * var_fmaxr_dn8)) * var_tmp) + (assign37330_e48677 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign37330_e48682;
        var_ibbt_dn5 = assign37330_e48682_d_n5;
        var_ibbt_dn6 = assign37330_e48682_d_n6;
        var_ibbt_dn7 = assign37330_e48682_d_n7;
        var_ibbt_dn8 = assign37330_e48682_d_n8;

        let assign37340_e48685: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard753 = assign37340_e48685;

        let (assign37350_e48696, assign37350_e48696_d_n5, assign37350_e48696_d_n6, assign37350_e48696_d_n7, assign37350_e48696_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard753 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign37350_e48696;
        var_fbreakdown_dn5 = assign37350_e48696_d_n5;
        var_fbreakdown_dn6 = assign37350_e48696_d_n6;
        var_fbreakdown_dn7 = assign37350_e48696_d_n7;
        var_fbreakdown_dn8 = assign37350_e48696_d_n8;

        let assign37360_e48699: f64 = (-var_alphaav);
        let assign37360_e48701: f64 = (assign37360_e48699 * var_vbrgatd_i);
        let assign37360_e48702: f64 = if var_vav > assign37360_e48701 { 1.0 } else { 0.0 };
        var_guard754 = assign37360_e48702;

        let assign37370_e48705: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard755 = assign37370_e48705;

        let (assign37380_e48735, assign37380_e48735_d_n5, assign37380_e48735_d_n6, assign37380_e48735_d_n7, assign37380_e48735_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard753 == 0.0)) && (var_guard754 != 0.0)) && (var_guard755 != 0.0)) {
        let assign37380_e48721: f64 = (var_vav * var_vbrinvgat_d);
        let assign37380_e48724: f64 = (var_vav * var_vbrinvgat_d);
        let assign37380_e48725: f64 = (assign37380_e48721 * assign37380_e48724);
        let assign37380_e48728: f64 = (var_vav * var_vbrinvgat_d);
        let assign37380_e48729: f64 = (assign37380_e48725 * assign37380_e48728);
        let assign37380_e48732: f64 = (var_vav * var_vbrinvgat_d);
        let assign37380_e48733: f64 = (assign37380_e48729 * assign37380_e48732);
        (assign37380_e48733, (((((((var_vav * var_vbrinvgat_d_dn5) * assign37380_e48724) + (assign37380_e48721 * (var_vav * var_vbrinvgat_d_dn5))) * assign37380_e48728) + (assign37380_e48725 * (var_vav * var_vbrinvgat_d_dn5))) * assign37380_e48732) + (assign37380_e48729 * (var_vav * var_vbrinvgat_d_dn5))), (((((((var_vav * var_vbrinvgat_d_dn6) * assign37380_e48724) + (assign37380_e48721 * (var_vav * var_vbrinvgat_d_dn6))) * assign37380_e48728) + (assign37380_e48725 * (var_vav * var_vbrinvgat_d_dn6))) * assign37380_e48732) + (assign37380_e48729 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign37380_e48724) + (assign37380_e48721 * (var_vav * var_vbrinvgat_d_dn7))) * assign37380_e48728) + (assign37380_e48725 * (var_vav * var_vbrinvgat_d_dn7))) * assign37380_e48732) + (assign37380_e48729 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign37380_e48724) + (assign37380_e48721 * (var_vav * var_vbrinvgat_d_dn8))) * assign37380_e48728) + (assign37380_e48725 * (var_vav * var_vbrinvgat_d_dn8))) * assign37380_e48732) + (assign37380_e48729 * (var_vav * var_vbrinvgat_d_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37380_e48735;
        var_tmp_dn5 = assign37380_e48735_d_n5;
        var_tmp_dn6 = assign37380_e48735_d_n6;
        var_tmp_dn7 = assign37380_e48735_d_n7;
        var_tmp_dn8 = assign37380_e48735_d_n8;

        let (assign37390_e48757, assign37390_e48757_d_n5, assign37390_e48757_d_n6, assign37390_e48757_d_n7, assign37390_e48757_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard753 == 0.0)) && (var_guard754 != 0.0)) && (var_guard755 == 0.0)) {
        let assign37390_e48752: f64 = (var_vav * var_vbrinvgat_d);
        let assign37390_e48753: f64 = (assign37390_e48752).abs();
        let assign37390_e48755: f64 = (assign37390_e48753).powf(var_pbrgatd_i);
        (assign37390_e48755, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign37390_e48753).powf(var_pbrgatd_i - 1.0) * if assign37390_e48752 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) })) } } else { (assign37390_e48755 * (var_pbrgatd_i * (if assign37390_e48752 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) } / assign37390_e48753))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign37390_e48753).powf(var_pbrgatd_i - 1.0) * if assign37390_e48752 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign37390_e48755 * (var_pbrgatd_i * (if assign37390_e48752 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign37390_e48753))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign37390_e48753).powf(var_pbrgatd_i - 1.0) * if assign37390_e48752 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign37390_e48755 * (var_pbrgatd_i * (if assign37390_e48752 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign37390_e48753))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign37390_e48753).powf(var_pbrgatd_i - 1.0) * if assign37390_e48752 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign37390_e48755 * (var_pbrgatd_i * (if assign37390_e48752 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign37390_e48753))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37390_e48757;
        var_tmp_dn5 = assign37390_e48757_d_n5;
        var_tmp_dn6 = assign37390_e48757_d_n6;
        var_tmp_dn7 = assign37390_e48757_d_n7;
        var_tmp_dn8 = assign37390_e48757_d_n8;

        let (assign37400_e48775, assign37400_e48775_d_n5, assign37400_e48775_d_n6, assign37400_e48775_d_n7, assign37400_e48775_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard753 == 0.0)) && (var_guard754 != 0.0)) {
        let assign37400_e48772: f64 = (1.0 - var_tmp);
        let assign37400_e48773: f64 = (1.0 / assign37400_e48772);
        (assign37400_e48773, (-((-var_tmp_dn5) / (assign37400_e48772 * assign37400_e48772))), (-((-var_tmp_dn6) / (assign37400_e48772 * assign37400_e48772))), (-((-var_tmp_dn7) / (assign37400_e48772 * assign37400_e48772))), (-((-var_tmp_dn8) / (assign37400_e48772 * assign37400_e48772))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign37400_e48775;
        var_fbreakdown_dn5 = assign37400_e48775_d_n5;
        var_fbreakdown_dn6 = assign37400_e48775_d_n6;
        var_fbreakdown_dn7 = assign37400_e48775_d_n7;
        var_fbreakdown_dn8 = assign37400_e48775_d_n8;

        let (assign37410_e48798, assign37410_e48798_d_n5, assign37410_e48798_d_n6, assign37410_e48798_d_n7, assign37410_e48798_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) && (var_guard753 == 0.0)) && (var_guard754 == 0.0)) {
        let assign37410_e48792: f64 = (var_alphaav * var_vbrgatd_i);
        let assign37410_e48793: f64 = (var_vav + assign37410_e48792);
        let assign37410_e48795: f64 = (assign37410_e48793 * var_slopegat_d);
        let assign37410_e48796: f64 = (var_fstopgat_d + assign37410_e48795);
        (assign37410_e48796, (assign37410_e48793 * var_slopegat_d_dn5), (assign37410_e48793 * var_slopegat_d_dn6), (assign37410_e48793 * var_slopegat_d_dn7), (assign37410_e48793 * var_slopegat_d_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign37410_e48798;
        var_fbreakdown_dn5 = assign37410_e48798_d_n5;
        var_fbreakdown_dn6 = assign37410_e48798_d_n6;
        var_fbreakdown_dn7 = assign37410_e48798_d_n7;
        var_fbreakdown_dn8 = assign37410_e48798_d_n8;

        let (assign37420_e48817, assign37420_e48817_d_n5, assign37420_e48817_d_n6, assign37420_e48817_d_n7, assign37420_e48817_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard739 == 0.0)) {
        let assign37420_e48808: f64 = (var_id__blk213 + var_isrh);
        let assign37420_e48810: f64 = (assign37420_e48808 + var_itat);
        let assign37420_e48812: f64 = (assign37420_e48810 + var_ibbt);
        let assign37420_e48813: f64 = (p.p29 * assign37420_e48812);
        let assign37420_e48815: f64 = (assign37420_e48813 * var_fbreakdown);
        (assign37420_e48815, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign37420_e48813 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign37420_e48813 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign37420_e48813 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign37420_e48813 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign37420_e48817;
        var_ijungat_dn5 = assign37420_e48817_d_n5;
        var_ijungat_dn6 = assign37420_e48817_d_n6;
        var_ijungat_dn7 = assign37420_e48817_d_n7;
        var_ijungat_dn8 = assign37420_e48817_d_n8;

        let (assign37430_e48833, assign37430_e48833_d_n5, assign37430_e48833_d_n6, assign37430_e48833_d_n7, assign37430_e48833_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign37430_e48823: f64 = (var_abdrain_i * var_ijunbot);
        let assign37430_e48826: f64 = (var_lsdrain_i * var_ijunsti);
        let assign37430_e48827: f64 = (assign37430_e48823 + assign37430_e48826);
        let assign37430_e48830: f64 = (var_lgdrain_i * var_ijungat);
        let assign37430_e48831: f64 = (assign37430_e48827 + assign37430_e48830);
        (assign37430_e48831, (((var_abdrain_i * var_ijunbot_dn5) + (var_lsdrain_i * var_ijunsti_dn5)) + (var_lgdrain_i * var_ijungat_dn5)), (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)),)
    } else {
        (var_i4, var_i4_dn5, var_i4_dn6, var_i4_dn7, var_i4_dn8,)
    }
};
        var_i4 = assign37430_e48833;
        var_i4_dn5 = assign37430_e48833_d_n5;
        var_i4_dn6 = assign37430_e48833_d_n6;
        var_i4_dn7 = assign37430_e48833_d_n7;
        var_i4_dn8 = assign37430_e48833_d_n8;

        let (assign37440_e48839,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign37440_e48839;

        let (assign37450_e48845,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign37450_e48845;

        let assign37460_e48857: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard756 = assign37460_e48857;

        let assign37540_e48943: f64 = if var_v5 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard757 = assign37540_e48943;

        let assign37550_e48945: f64 = (-0.5);
        let assign37550_e48948: f64 = (var_v5 * var_phitdinv);
        let assign37550_e48949: f64 = (assign37550_e48945 * assign37550_e48948);
        let assign37550_e48950: f64 = (assign37550_e48949).abs();
        let assign37550_e48952: f64 = if assign37550_e48950 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard758 = assign37550_e48952;

        let (assign37560_e48970,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 != 0.0)) && (var_guard758 != 0.0)) {
        let assign37560_e48963: f64 = (-0.5);
        let assign37560_e48966: f64 = (var_v5 * var_phitdinv);
        let assign37560_e48967: f64 = (assign37560_e48963 * assign37560_e48966);
        let assign37560_e48968: f64 = (assign37560_e48967).exp();
        (assign37560_e48968,)
    } else {
        (var_z,)
    }
};
        var_z = assign37560_e48970;

        let assign37570_e48972: f64 = (-0.5);
        let assign37570_e48975: f64 = (var_v5 * var_phitdinv);
        let assign37570_e48976: f64 = (assign37570_e48972 * assign37570_e48975);
        let assign37570_e48978: f64 = if assign37570_e48976 < 0.0 { 1.0 } else { 0.0 };
        var_guard759 = assign37570_e48978;

        let (assign37580_e49033,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 != 0.0)) && (var_guard758 == 0.0)) && (var_guard759 != 0.0)) {
        let assign37580_e48994: f64 = (-230.25850929940458);
        let assign37580_e48996: f64 = (-0.5);
        let assign37580_e48999: f64 = (var_v5 * var_phitdinv);
        let assign37580_e49000: f64 = (assign37580_e48996 * assign37580_e48999);
        let assign37580_e49001: f64 = (assign37580_e48994 - assign37580_e49000);
        let assign37580_e49005: f64 = (-230.25850929940458);
        let assign37580_e49007: f64 = (-0.5);
        let assign37580_e49010: f64 = (var_v5 * var_phitdinv);
        let assign37580_e49011: f64 = (assign37580_e49007 * assign37580_e49010);
        let assign37580_e49012: f64 = (assign37580_e49005 - assign37580_e49011);
        let assign37580_e49015: f64 = (-230.25850929940458);
        let assign37580_e49017: f64 = (-0.5);
        let assign37580_e49020: f64 = (var_v5 * var_phitdinv);
        let assign37580_e49021: f64 = (assign37580_e49017 * assign37580_e49020);
        let assign37580_e49022: f64 = (assign37580_e49015 - assign37580_e49021);
        let assign37580_e49024: f64 = (assign37580_e49022 * 0.3333333333333333);
        let assign37580_e49025: f64 = (1.0 + assign37580_e49024);
        let assign37580_e49026: f64 = (assign37580_e49012 * assign37580_e49025);
        let assign37580_e49027: f64 = (0.5 * assign37580_e49026);
        let assign37580_e49028: f64 = (1.0 + assign37580_e49027);
        let assign37580_e49029: f64 = (assign37580_e49001 * assign37580_e49028);
        let assign37580_e49030: f64 = (1.0 + assign37580_e49029);
        let assign37580_e49031: f64 = (1e-100 / assign37580_e49030);
        (assign37580_e49031,)
    } else {
        (var_z,)
    }
};
        var_z = assign37580_e49033;

        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard747_slot = var_guard747;
        *var_guard748_slot = var_guard748;
        *var_guard749_slot = var_guard749;
        *var_guard750_slot = var_guard750;
        *var_guard751_slot = var_guard751;
        *var_guard752_slot = var_guard752;
        *var_guard753_slot = var_guard753;
        *var_guard754_slot = var_guard754;
        *var_guard755_slot = var_guard755;
        *var_guard756_slot = var_guard756;
        *var_guard757_slot = var_guard757;
        *var_guard758_slot = var_guard758;
        *var_guard759_slot = var_guard759;
        *var_i4_slot = var_i4;
        *var_i4_dn5_slot = var_i4_dn5;
        *var_i4_dn6_slot = var_i4_dn6;
        *var_i4_dn7_slot = var_i4_dn7;
        *var_i4_dn8_slot = var_i4_dn8;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_vbbt_slot = var_vbbt;
        *var_z_slot = var_z;
    }

    pub(super) fn stamp_transient_block_79(
        var_abdrain_i: f64,
        var_atatbot_d: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_ftdbot_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard756: f64,
        var_guard757: f64,
        var_guard758: f64,
        var_guard759: f64,
        var_idsatbot_d: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_v5: f64,
        var_vbbtlim_d: f64,
        var_vbibot_d: f64,
        var_vbimin_d: f64,
        var_vbirbotinv_d: f64,
        var_vmax_d: f64,
        var_wdepnulrbot_d: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard760_slot: &mut f64,
        var_guard761_slot: &mut f64,
        var_guard762_slot: &mut f64,
        var_guard763_slot: &mut f64,
        var_guard764_slot: &mut f64,
        var_guard765_slot: &mut f64,
        var_guard766_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard760: f64 = *var_guard760_slot;
        let mut var_guard761: f64 = *var_guard761_slot;
        let mut var_guard762: f64 = *var_guard762_slot;
        let mut var_guard763: f64 = *var_guard763_slot;
        let mut var_guard764: f64 = *var_guard764_slot;
        let mut var_guard765: f64 = *var_guard765_slot;
        let mut var_guard766: f64 = *var_guard766_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign37590_e49086,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 != 0.0)) && (var_guard758 == 0.0)) && (var_guard759 == 0.0)) {
        let assign37590_e49050: f64 = (-0.5);
        let assign37590_e49053: f64 = (var_v5 * var_phitdinv);
        let assign37590_e49054: f64 = (assign37590_e49050 * assign37590_e49053);
        let assign37590_e49056: f64 = (assign37590_e49054 - 230.25850929940458);
        let assign37590_e49060: f64 = (-0.5);
        let assign37590_e49063: f64 = (var_v5 * var_phitdinv);
        let assign37590_e49064: f64 = (assign37590_e49060 * assign37590_e49063);
        let assign37590_e49066: f64 = (assign37590_e49064 - 230.25850929940458);
        let assign37590_e49069: f64 = (-0.5);
        let assign37590_e49072: f64 = (var_v5 * var_phitdinv);
        let assign37590_e49073: f64 = (assign37590_e49069 * assign37590_e49072);
        let assign37590_e49075: f64 = (assign37590_e49073 - 230.25850929940458);
        let assign37590_e49077: f64 = (assign37590_e49075 * 0.3333333333333333);
        let assign37590_e49078: f64 = (1.0 + assign37590_e49077);
        let assign37590_e49079: f64 = (assign37590_e49066 * assign37590_e49078);
        let assign37590_e49080: f64 = (0.5 * assign37590_e49079);
        let assign37590_e49081: f64 = (1.0 + assign37590_e49080);
        let assign37590_e49082: f64 = (assign37590_e49056 * assign37590_e49081);
        let assign37590_e49083: f64 = (1.0 + assign37590_e49082);
        let assign37590_e49084: f64 = (1e100 * assign37590_e49083);
        (assign37590_e49084,)
    } else {
        (var_z,)
    }
};
        var_z = assign37590_e49086;

        let (assign37600_e49098,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 != 0.0)) {
        let assign37600_e49096: f64 = (1.0 / var_z);
        (assign37600_e49096,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign37600_e49098;

        let (assign37610_e49110,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 != 0.0)) {
        let assign37610_e49108: f64 = (var_zinv * var_zinv);
        (assign37610_e49108,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign37610_e49110;

        let (assign37620_e49129,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 == 0.0)) {
        let assign37620_e49122: f64 = (var_v5 - var_vmax_d);
        let assign37620_e49124: f64 = (assign37620_e49122 * var_phitdinv);
        let assign37620_e49125: f64 = (1.0 + assign37620_e49124);
        let assign37620_e49127: f64 = (assign37620_e49125 * var_exp_vmax_over_phitd_d);
        (assign37620_e49127,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign37620_e49129;

        let (assign37630_e49141,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 == 0.0)) {
        let assign37630_e49139: f64 = (var_idmult).sqrt();
        (assign37630_e49139,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign37630_e49141;

        let (assign37640_e49154,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 == 0.0)) {
        let assign37640_e49152: f64 = (1.0 / var_zinv);
        (assign37640_e49152,)
    } else {
        (var_z,)
    }
};
        var_z = assign37640_e49154;

        let (assign37650_e49164,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) {
        let assign37650_e49162: f64 = (var_idmult - 1.0);
        (assign37650_e49162,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign37650_e49164;

        let assign37660_e49167: f64 = if var_v5 > 0.0 { 1.0 } else { 0.0 };
        var_guard760 = assign37660_e49167;

        let (assign37670_e49193,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) && (var_guard760 != 0.0)) {
        let assign37670_e49179: f64 = (2.0 + var_z);
        let assign37670_e49182: f64 = (var_z + 1.0);
        let assign37670_e49185: f64 = (var_z + 3.0);
        let assign37670_e49186: f64 = (assign37670_e49182 * assign37670_e49185);
        let assign37670_e49187: f64 = (assign37670_e49186).sqrt();
        let assign37670_e49188: f64 = (assign37670_e49179 + assign37670_e49187);
        let assign37670_e49189: f64 = (assign37670_e49188).ln();
        let assign37670_e49190: f64 = (var_phitd * assign37670_e49189);
        let assign37670_e49191: f64 = (2.0 * assign37670_e49190);
        (assign37670_e49191,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign37670_e49193;

        let (assign37680_e49227,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) && (var_guard760 == 0.0)) {
        let assign37680_e49203: f64 = (-var_v5);
        let assign37680_e49208: f64 = (2.0 * var_zinv);
        let assign37680_e49210: f64 = (assign37680_e49208 + 1.0);
        let assign37680_e49213: f64 = (1.0 + var_zinv);
        let assign37680_e49217: f64 = (3.0 * var_zinv);
        let assign37680_e49218: f64 = (1.0 + assign37680_e49217);
        let assign37680_e49219: f64 = (assign37680_e49213 * assign37680_e49218);
        let assign37680_e49220: f64 = (assign37680_e49219).sqrt();
        let assign37680_e49221: f64 = (assign37680_e49210 + assign37680_e49220);
        let assign37680_e49222: f64 = (assign37680_e49221).ln();
        let assign37680_e49223: f64 = (var_phitd * assign37680_e49222);
        let assign37680_e49224: f64 = (2.0 * assign37680_e49223);
        let assign37680_e49225: f64 = (assign37680_e49203 + assign37680_e49224);
        (assign37680_e49225,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign37680_e49227;

        let (assign37690_e49237,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) {
        let assign37690_e49235: f64 = (var_vbimin_d - var_two_psistar);
        (assign37690_e49235,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign37690_e49237;

        let (assign37700_e49264,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) {
        let assign37700_e49246: f64 = (var_v5 + var_vjlim);
        let assign37700_e49249: f64 = (var_v5 - var_vjlim);
        let assign37700_e49252: f64 = (var_v5 - var_vjlim);
        let assign37700_e49253: f64 = (assign37700_e49249 * assign37700_e49252);
        let assign37700_e49256: f64 = (4.0 * var_phitd);
        let assign37700_e49258: f64 = (assign37700_e49256 * var_phitd);
        let assign37700_e49259: f64 = (assign37700_e49253 + assign37700_e49258);
        let assign37700_e49260: f64 = (assign37700_e49259).sqrt();
        let assign37700_e49261: f64 = (assign37700_e49246 - assign37700_e49260);
        let assign37700_e49262: f64 = (0.5 * assign37700_e49261);
        (assign37700_e49262,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign37700_e49264;

        let (assign37710_e49291,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) {
        let assign37710_e49273: f64 = (var_v5 + var_vbbtlim_d);
        let assign37710_e49276: f64 = (var_v5 - var_vbbtlim_d);
        let assign37710_e49279: f64 = (var_v5 - var_vbbtlim_d);
        let assign37710_e49280: f64 = (assign37710_e49276 * assign37710_e49279);
        let assign37710_e49283: f64 = (4.0 * var_phitr);
        let assign37710_e49285: f64 = (assign37710_e49283 * var_phitr);
        let assign37710_e49286: f64 = (assign37710_e49280 + assign37710_e49285);
        let assign37710_e49287: f64 = (assign37710_e49286).sqrt();
        let assign37710_e49288: f64 = (assign37710_e49273 - assign37710_e49287);
        let assign37710_e49289: f64 = (0.5 * assign37710_e49288);
        (assign37710_e49289,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign37710_e49291;

        let (assign37720_e49318,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard756 != 0.0)) {
        let assign37720_e49300: f64 = var_v5;
        let assign37720_e49303: f64 = var_v5;
        let assign37720_e49306: f64 = var_v5;
        let assign37720_e49307: f64 = (assign37720_e49303 * assign37720_e49306);
        let assign37720_e49310: f64 = (4.0 * 1e-6);
        let assign37720_e49312: f64 = (assign37720_e49310 * 1e-6);
        let assign37720_e49313: f64 = (assign37720_e49307 + assign37720_e49312);
        let assign37720_e49314: f64 = (assign37720_e49313).sqrt();
        let assign37720_e49315: f64 = (assign37720_e49300 - assign37720_e49314);
        let assign37720_e49316: f64 = (0.5 * assign37720_e49315);
        (assign37720_e49316,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign37720_e49318;

        let assign37730_e49321: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard761 = assign37730_e49321;

        let (assign37740_e49329, assign37740_e49329_d_n5, assign37740_e49329_d_n6, assign37740_e49329_d_n7, assign37740_e49329_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign37740_e49329;
        var_ijunbot_dn5 = assign37740_e49329_d_n5;
        var_ijunbot_dn6 = assign37740_e49329_d_n6;
        var_ijunbot_dn7 = assign37740_e49329_d_n7;
        var_ijunbot_dn8 = assign37740_e49329_d_n8;

        let (assign37750_e49340,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) {
        let assign37750_e49338: f64 = (var_idsatbot_d * var_idmult);
        (assign37750_e49338,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign37750_e49340;

        let assign37760_e49347: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard762 = assign37760_e49347;

        let (assign37770_e49358, assign37770_e49358_d_n5, assign37770_e49358_d_n6, assign37770_e49358_d_n7, assign37770_e49358_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign37770_e49358;
        var_isrh_dn5 = assign37770_e49358_d_n5;
        var_isrh_dn6 = assign37770_e49358_d_n6;
        var_isrh_dn7 = assign37770_e49358_d_n7;
        var_isrh_dn8 = assign37770_e49358_d_n8;

        let (assign37780_e49372,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) {
        let assign37780_e49370: f64 = (var_vbibot_d - var_vjsrh);
        (assign37780_e49370,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign37780_e49372;

        let (assign37790_e49391,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) {
        let assign37790_e49386: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign37790_e49387: f64 = (1.0 - assign37790_e49386);
        let assign37790_e49388: f64 = (assign37790_e49387).sqrt();
        let assign37790_e49389: f64 = (1.0 - assign37790_e49388);
        (assign37790_e49389,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign37790_e49391;

        let assign37800_e49394: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard763 = assign37800_e49394;

        let (assign37810_e49408,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) && (var_guard763 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign37810_e49408;

        let (assign37820_e49440,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) && (var_guard763 == 0.0)) {
        let assign37820_e49423: f64 = (var_wsrhstep * var_wsrhstep);
        let assign37820_e49425: f64 = (var_wsrhstep).ln();
        let assign37820_e49426: f64 = (assign37820_e49423 * assign37820_e49425);
        let assign37820_e49429: f64 = (1.0 - var_wsrhstep);
        let assign37820_e49430: f64 = (assign37820_e49426 / assign37820_e49429);
        let assign37820_e49432: f64 = (assign37820_e49430 + var_wsrhstep);
        let assign37820_e49436: f64 = (2.0 * var_pbotd_i);
        let assign37820_e49437: f64 = (1.0 - assign37820_e49436);
        let assign37820_e49438: f64 = (assign37820_e49432 * assign37820_e49437);
        (assign37820_e49438,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign37820_e49440;

        let (assign37830_e49454,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) {
        let assign37830_e49452: f64 = (var_wsrhstep + var_dwsrh);
        (assign37830_e49452,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign37830_e49454;

        let assign37840_e49457: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard764 = assign37840_e49457;

        let (assign37850_e49474, assign37850_e49474_d_n5, assign37850_e49474_d_n6, assign37850_e49474_d_n7, assign37850_e49474_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) && (var_guard764 != 0.0)) {
        let assign37850_e49471: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign37850_e49472: f64 = (assign37850_e49471).sqrt();
        (assign37850_e49472, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37850_e49474;
        var_tmp_dn5 = assign37850_e49474_d_n5;
        var_tmp_dn6 = assign37850_e49474_d_n6;
        var_tmp_dn7 = assign37850_e49474_d_n7;
        var_tmp_dn8 = assign37850_e49474_d_n8;

        let (assign37860_e49493, assign37860_e49493_d_n5, assign37860_e49493_d_n6, assign37860_e49493_d_n7, assign37860_e49493_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37860_e49489: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign37860_e49491: f64 = (assign37860_e49489).powf(var_pbotd_i);
        (assign37860_e49491, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37860_e49493;
        var_tmp_dn5 = assign37860_e49493_d_n5;
        var_tmp_dn6 = assign37860_e49493_d_n6;
        var_tmp_dn7 = assign37860_e49493_d_n7;
        var_tmp_dn8 = assign37860_e49493_d_n8;

        let (assign37870_e49507, assign37870_e49507_d_n5, assign37870_e49507_d_n6, assign37870_e49507_d_n7, assign37870_e49507_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) {
        let assign37870_e49505: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign37870_e49505, (var_wdepnulrbot_d * var_tmp_dn5), (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign37870_e49507;
        var_wdep_dn5 = assign37870_e49507_d_n5;
        var_wdep_dn6 = assign37870_e49507_d_n6;
        var_wdep_dn7 = assign37870_e49507_d_n7;
        var_wdep_dn8 = assign37870_e49507_d_n8;

        let (assign37880_e49525, assign37880_e49525_d_n5, assign37880_e49525_d_n6, assign37880_e49525_d_n7, assign37880_e49525_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) {
        let assign37880_e49520: f64 = (var_zinv - 1.0);
        let assign37880_e49522: f64 = (assign37880_e49520 * var_wdep);
        let assign37880_e49523: f64 = (var_ftdbot_d * assign37880_e49522);
        (assign37880_e49523, (var_ftdbot_d * (assign37880_e49520 * var_wdep_dn5)), (var_ftdbot_d * (assign37880_e49520 * var_wdep_dn6)), (var_ftdbot_d * (assign37880_e49520 * var_wdep_dn7)), (var_ftdbot_d * (assign37880_e49520 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign37880_e49525;
        var_asrh_dn5 = assign37880_e49525_d_n5;
        var_asrh_dn6 = assign37880_e49525_d_n6;
        var_asrh_dn7 = assign37880_e49525_d_n7;
        var_asrh_dn8 = assign37880_e49525_d_n8;

        let (assign37890_e49541, assign37890_e49541_d_n5, assign37890_e49541_d_n6, assign37890_e49541_d_n7, assign37890_e49541_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) {
        let assign37890_e49538: f64 = (var_asrh * var_wsrh);
        let assign37890_e49539: f64 = (var_csrhbotd_i * assign37890_e49538);
        (assign37890_e49539, (var_csrhbotd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign37890_e49541;
        var_isrh_dn5 = assign37890_e49541_d_n5;
        var_isrh_dn6 = assign37890_e49541_d_n6;
        var_isrh_dn7 = assign37890_e49541_d_n7;
        var_isrh_dn8 = assign37890_e49541_d_n8;

        let assign37900_e49544: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard765 = assign37900_e49544;

        let (assign37910_e49555, assign37910_e49555_d_n5, assign37910_e49555_d_n6, assign37910_e49555_d_n7, assign37910_e49555_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign37910_e49555;
        var_itat_dn5 = assign37910_e49555_d_n5;
        var_itat_dn6 = assign37910_e49555_d_n6;
        var_itat_dn7 = assign37910_e49555_d_n7;
        var_itat_dn8 = assign37910_e49555_d_n8;

        let (assign37920_e49573, assign37920_e49573_d_n5, assign37920_e49573_d_n6, assign37920_e49573_d_n7, assign37920_e49573_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign37920_e49568: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign37920_e49570: f64 = (assign37920_e49568 / var_vbi_minus_vjsrh);
        let assign37920_e49571: f64 = (var_btatpartbot_d * assign37920_e49570);
        (assign37920_e49571, (var_btatpartbot_d * ((var_wdep_dn5 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign37920_e49573;
        var_btat_dn5 = assign37920_e49573_d_n5;
        var_btat_dn6 = assign37920_e49573_d_n6;
        var_btat_dn7 = assign37920_e49573_d_n7;
        var_btat_dn8 = assign37920_e49573_d_n8;

        let (assign37930_e49589, assign37930_e49589_d_n5, assign37930_e49589_d_n6, assign37930_e49589_d_n7, assign37930_e49589_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign37930_e49585: f64 = (0.666666666666667 * var_atatbot_d);
        let assign37930_e49587: f64 = (assign37930_e49585 / var_btat);
        (assign37930_e49587, (-((assign37930_e49585 * var_btat_dn5) / (var_btat * var_btat))), (-((assign37930_e49585 * var_btat_dn6) / (var_btat * var_btat))), (-((assign37930_e49585 * var_btat_dn7) / (var_btat * var_btat))), (-((assign37930_e49585 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign37930_e49589;
        var_twoatatoverthreebtat_dn5 = assign37930_e49589_d_n5;
        var_twoatatoverthreebtat_dn6 = assign37930_e49589_d_n6;
        var_twoatatoverthreebtat_dn7 = assign37930_e49589_d_n7;
        var_twoatatoverthreebtat_dn8 = assign37930_e49589_d_n8;

        let (assign37940_e49603, assign37940_e49603_d_n5, assign37940_e49603_d_n6, assign37940_e49603_d_n7, assign37940_e49603_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign37940_e49601: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign37940_e49601, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign37940_e49603;
        var_umaxbeforelimiting_dn5 = assign37940_e49603_d_n5;
        var_umaxbeforelimiting_dn6 = assign37940_e49603_d_n6;
        var_umaxbeforelimiting_dn7 = assign37940_e49603_d_n7;
        var_umaxbeforelimiting_dn8 = assign37940_e49603_d_n8;

        let (assign37950_e49624, assign37950_e49624_d_n5, assign37950_e49624_d_n6, assign37950_e49624_d_n7, assign37950_e49624_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign37950_e49615: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign37950_e49618: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign37950_e49620: f64 = (assign37950_e49618 + 1.0);
        let assign37950_e49621: f64 = (assign37950_e49615 / assign37950_e49620);
        let assign37950_e49622: f64 = (assign37950_e49621).sqrt();
        (assign37950_e49622, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign37950_e49620) - (assign37950_e49615 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign37950_e49620 * assign37950_e49620)) / (2.0 * assign37950_e49622)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign37950_e49620) - (assign37950_e49615 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign37950_e49620 * assign37950_e49620)) / (2.0 * assign37950_e49622)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign37950_e49620) - (assign37950_e49615 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign37950_e49620 * assign37950_e49620)) / (2.0 * assign37950_e49622)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign37950_e49620) - (assign37950_e49615 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign37950_e49620 * assign37950_e49620)) / (2.0 * assign37950_e49622)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign37950_e49624;
        var_umax_dn5 = assign37950_e49624_d_n5;
        var_umax_dn6 = assign37950_e49624_d_n6;
        var_umax_dn7 = assign37950_e49624_d_n7;
        var_umax_dn8 = assign37950_e49624_d_n8;

        let (assign37960_e49637, assign37960_e49637_d_n5, assign37960_e49637_d_n6, assign37960_e49637_d_n7, assign37960_e49637_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign37960_e49635: f64 = (var_umax).sqrt();
        (assign37960_e49635, (var_umax_dn5 / (2.0 * assign37960_e49635)), (var_umax_dn6 / (2.0 * assign37960_e49635)), (var_umax_dn7 / (2.0 * assign37960_e49635)), (var_umax_dn8 / (2.0 * assign37960_e49635)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign37960_e49637;
        var_sqrtumax_dn5 = assign37960_e49637_d_n5;
        var_sqrtumax_dn6 = assign37960_e49637_d_n6;
        var_sqrtumax_dn7 = assign37960_e49637_d_n7;
        var_sqrtumax_dn8 = assign37960_e49637_d_n8;

        let (assign37970_e49651, assign37970_e49651_d_n5, assign37970_e49651_d_n6, assign37970_e49651_d_n7, assign37970_e49651_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign37970_e49649: f64 = (var_umax * var_sqrtumax);
        (assign37970_e49649, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign37970_e49651;
        var_umaxpoweronepointfive_dn5 = assign37970_e49651_d_n5;
        var_umaxpoweronepointfive_dn6 = assign37970_e49651_d_n6;
        var_umaxpoweronepointfive_dn7 = assign37970_e49651_d_n7;
        var_umaxpoweronepointfive_dn8 = assign37970_e49651_d_n8;

        let assign37980_e49653: f64 = (-var_pbotd_i);
        let assign37980_e49655: f64 = (assign37980_e49653 * var_one_over_one_minus_pbot_d);
        let assign37980_e49657: f64 = (-1.0);
        let assign37980_e49658: f64 = if assign37980_e49655 == assign37980_e49657 { 1.0 } else { 0.0 };
        var_guard766 = assign37980_e49658;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard760_slot = var_guard760;
        *var_guard761_slot = var_guard761;
        *var_guard762_slot = var_guard762;
        *var_guard763_slot = var_guard763;
        *var_guard764_slot = var_guard764;
        *var_guard765_slot = var_guard765;
        *var_guard766_slot = var_guard766;
        *var_id__blk213_slot = var_id__blk213;
        *var_idmult_slot = var_idmult;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }
}
