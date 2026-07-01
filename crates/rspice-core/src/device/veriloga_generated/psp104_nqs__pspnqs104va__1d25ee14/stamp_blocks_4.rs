#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
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
        var_guard182: f64,
        var_guard199: f64,
        var_guard616: f64,
        var_guard620: f64,
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
        var_v2: f64,
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
        var_guard623_slot: &mut f64,
        var_guard624_slot: &mut f64,
        var_guard625_slot: &mut f64,
        var_guard626_slot: &mut f64,
        var_guard627_slot: &mut f64,
        var_guard628_slot: &mut f64,
        var_guard629_slot: &mut f64,
        var_guard630_slot: &mut f64,
        var_guard631_slot: &mut f64,
        var_guard632_slot: &mut f64,
        var_guard633_slot: &mut f64,
        var_guard634_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard623: f64 = *var_guard623_slot;
        let mut var_guard624: f64 = *var_guard624_slot;
        let mut var_guard625: f64 = *var_guard625_slot;
        let mut var_guard626: f64 = *var_guard626_slot;
        let mut var_guard627: f64 = *var_guard627_slot;
        let mut var_guard628: f64 = *var_guard628_slot;
        let mut var_guard629: f64 = *var_guard629_slot;
        let mut var_guard630: f64 = *var_guard630_slot;
        let mut var_guard631: f64 = *var_guard631_slot;
        let mut var_guard632: f64 = *var_guard632_slot;
        let mut var_guard633: f64 = *var_guard633_slot;
        let mut var_guard634: f64 = *var_guard634_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let assign31790_e39469: f64 = (-var_ysq);
        let assign31790_e39471: f64 = (assign31790_e39469 + var_mtat);
        let assign31790_e39473: f64 = (-230.25850929940458);
        let assign31790_e39474: f64 = if assign31790_e39471 > assign31790_e39473 { 1.0 } else { 0.0 };
        var_guard623 = assign31790_e39474;

        let (assign31800_e39492, assign31800_e39492_d_n5, assign31800_e39492_d_n6, assign31800_e39492_d_n7, assign31800_e39492_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) && (var_guard623 != 0.0)) {
        let assign31800_e39487: f64 = (-var_ysq);
        let assign31800_e39489: f64 = (assign31800_e39487 + var_mtat);
        let assign31800_e39490: f64 = (assign31800_e39489).exp();
        (assign31800_e39490, (assign31800_e39490 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign31800_e39490 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign31800_e39490 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign31800_e39490 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31800_e39492;
        var_tmp_dn5 = assign31800_e39492_d_n5;
        var_tmp_dn6 = assign31800_e39492_d_n6;
        var_tmp_dn7 = assign31800_e39492_d_n7;
        var_tmp_dn8 = assign31800_e39492_d_n8;

        let (assign31810_e39541, assign31810_e39541_d_n5, assign31810_e39541_d_n6, assign31810_e39541_d_n7, assign31810_e39541_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) && (var_guard623 == 0.0)) {
        let assign31810_e39508: f64 = (-230.25850929940458);
        let assign31810_e39510: f64 = (-var_ysq);
        let assign31810_e39512: f64 = (assign31810_e39510 + var_mtat);
        let assign31810_e39513: f64 = (assign31810_e39508 - assign31810_e39512);
        let assign31810_e39517: f64 = (-230.25850929940458);
        let assign31810_e39519: f64 = (-var_ysq);
        let assign31810_e39521: f64 = (assign31810_e39519 + var_mtat);
        let assign31810_e39522: f64 = (assign31810_e39517 - assign31810_e39521);
        let assign31810_e39525: f64 = (-230.25850929940458);
        let assign31810_e39527: f64 = (-var_ysq);
        let assign31810_e39529: f64 = (assign31810_e39527 + var_mtat);
        let assign31810_e39530: f64 = (assign31810_e39525 - assign31810_e39529);
        let assign31810_e39532: f64 = (assign31810_e39530 * 0.3333333333333333);
        let assign31810_e39533: f64 = (1.0 + assign31810_e39532);
        let assign31810_e39534: f64 = (assign31810_e39522 * assign31810_e39533);
        let assign31810_e39535: f64 = (0.5 * assign31810_e39534);
        let assign31810_e39536: f64 = (1.0 + assign31810_e39535);
        let assign31810_e39537: f64 = (assign31810_e39513 * assign31810_e39536);
        let assign31810_e39538: f64 = (1.0 + assign31810_e39537);
        let assign31810_e39539: f64 = (1e-100 / assign31810_e39538);
        (assign31810_e39539, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign31810_e39536) + (assign31810_e39513 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign31810_e39533) + (assign31810_e39522 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign31810_e39538 * assign31810_e39538))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign31810_e39536) + (assign31810_e39513 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign31810_e39533) + (assign31810_e39522 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign31810_e39538 * assign31810_e39538))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign31810_e39536) + (assign31810_e39513 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign31810_e39533) + (assign31810_e39522 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign31810_e39538 * assign31810_e39538))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign31810_e39536) + (assign31810_e39513 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign31810_e39533) + (assign31810_e39522 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign31810_e39538 * assign31810_e39538))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31810_e39541;
        var_tmp_dn5 = assign31810_e39541_d_n5;
        var_tmp_dn6 = assign31810_e39541_d_n6;
        var_tmp_dn7 = assign31810_e39541_d_n7;
        var_tmp_dn8 = assign31810_e39541_d_n8;

        let (assign31820_e39571, assign31820_e39571_d_n5, assign31820_e39571_d_n6, assign31820_e39571_d_n7, assign31820_e39571_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31820_e39553: f64 = (0.29214664 * var_terfc);
        let assign31820_e39557: f64 = (var_terfc * var_terfc);
        let assign31820_e39558: f64 = (var_berfc * assign31820_e39557);
        let assign31820_e39559: f64 = (assign31820_e39553 + assign31820_e39558);
        let assign31820_e39563: f64 = (var_terfc * var_terfc);
        let assign31820_e39565: f64 = (assign31820_e39563 * var_terfc);
        let assign31820_e39566: f64 = (var_cerfc * assign31820_e39565);
        let assign31820_e39567: f64 = (assign31820_e39559 + assign31820_e39566);
        let assign31820_e39569: f64 = (assign31820_e39567 * var_tmp);
        (assign31820_e39569, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign31820_e39563 * var_terfc_dn5)))) * var_tmp) + (assign31820_e39567 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign31820_e39563 * var_terfc_dn6)))) * var_tmp) + (assign31820_e39567 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign31820_e39563 * var_terfc_dn7)))) * var_tmp) + (assign31820_e39567 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign31820_e39563 * var_terfc_dn8)))) * var_tmp) + (assign31820_e39567 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign31820_e39571;
        var_erfcpos_dn5 = assign31820_e39571_d_n5;
        var_erfcpos_dn6 = assign31820_e39571_d_n6;
        var_erfcpos_dn7 = assign31820_e39571_d_n7;
        var_erfcpos_dn8 = assign31820_e39571_d_n8;

        let assign31830_e39574: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard624 = assign31830_e39574;

        let (assign31840_e39588, assign31840_e39588_d_n5, assign31840_e39588_d_n6, assign31840_e39588_d_n7, assign31840_e39588_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) && (var_guard624 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign31840_e39588;
        var_erfctimesexpmtat_dn5 = assign31840_e39588_d_n5;
        var_erfctimesexpmtat_dn6 = assign31840_e39588_d_n6;
        var_erfctimesexpmtat_dn7 = assign31840_e39588_d_n7;
        var_erfctimesexpmtat_dn8 = assign31840_e39588_d_n8;

        let assign31850_e39591: f64 = (-230.25850929940458);
        let assign31850_e39592: f64 = if var_mtat > assign31850_e39591 { 1.0 } else { 0.0 };
        var_guard625 = assign31850_e39592;

        let (assign31860_e39610, assign31860_e39610_d_n5, assign31860_e39610_d_n6, assign31860_e39610_d_n7, assign31860_e39610_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) && (var_guard624 == 0.0)) && (var_guard625 != 0.0)) {
        let assign31860_e39608: f64 = (var_mtat).exp();
        (assign31860_e39608, (assign31860_e39608 * var_mtat_dn5), (assign31860_e39608 * var_mtat_dn6), (assign31860_e39608 * var_mtat_dn7), (assign31860_e39608 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31860_e39610;
        var_tmp_dn5 = assign31860_e39610_d_n5;
        var_tmp_dn6 = assign31860_e39610_d_n6;
        var_tmp_dn7 = assign31860_e39610_d_n7;
        var_tmp_dn8 = assign31860_e39610_d_n8;

        let (assign31870_e39653, assign31870_e39653_d_n5, assign31870_e39653_d_n6, assign31870_e39653_d_n7, assign31870_e39653_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) && (var_guard624 == 0.0)) && (var_guard625 == 0.0)) {
        let assign31870_e39629: f64 = (-230.25850929940458);
        let assign31870_e39631: f64 = (assign31870_e39629 - var_mtat);
        let assign31870_e39635: f64 = (-230.25850929940458);
        let assign31870_e39637: f64 = (assign31870_e39635 - var_mtat);
        let assign31870_e39640: f64 = (-230.25850929940458);
        let assign31870_e39642: f64 = (assign31870_e39640 - var_mtat);
        let assign31870_e39644: f64 = (assign31870_e39642 * 0.3333333333333333);
        let assign31870_e39645: f64 = (1.0 + assign31870_e39644);
        let assign31870_e39646: f64 = (assign31870_e39637 * assign31870_e39645);
        let assign31870_e39647: f64 = (0.5 * assign31870_e39646);
        let assign31870_e39648: f64 = (1.0 + assign31870_e39647);
        let assign31870_e39649: f64 = (assign31870_e39631 * assign31870_e39648);
        let assign31870_e39650: f64 = (1.0 + assign31870_e39649);
        let assign31870_e39651: f64 = (1e-100 / assign31870_e39650);
        (assign31870_e39651, (-((1e-100 * (((-var_mtat_dn5) * assign31870_e39648) + (assign31870_e39631 * (0.5 * (((-var_mtat_dn5) * assign31870_e39645) + (assign31870_e39637 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign31870_e39650 * assign31870_e39650))), (-((1e-100 * (((-var_mtat_dn6) * assign31870_e39648) + (assign31870_e39631 * (0.5 * (((-var_mtat_dn6) * assign31870_e39645) + (assign31870_e39637 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign31870_e39650 * assign31870_e39650))), (-((1e-100 * (((-var_mtat_dn7) * assign31870_e39648) + (assign31870_e39631 * (0.5 * (((-var_mtat_dn7) * assign31870_e39645) + (assign31870_e39637 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign31870_e39650 * assign31870_e39650))), (-((1e-100 * (((-var_mtat_dn8) * assign31870_e39648) + (assign31870_e39631 * (0.5 * (((-var_mtat_dn8) * assign31870_e39645) + (assign31870_e39637 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign31870_e39650 * assign31870_e39650))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31870_e39653;
        var_tmp_dn5 = assign31870_e39653_d_n5;
        var_tmp_dn6 = assign31870_e39653_d_n6;
        var_tmp_dn7 = assign31870_e39653_d_n7;
        var_tmp_dn8 = assign31870_e39653_d_n8;

        let (assign31880_e39672, assign31880_e39672_d_n5, assign31880_e39672_d_n6, assign31880_e39672_d_n7, assign31880_e39672_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) && (var_guard624 == 0.0)) {
        let assign31880_e39668: f64 = (2.0 * var_tmp);
        let assign31880_e39670: f64 = (assign31880_e39668 - var_erfcpos);
        (assign31880_e39670, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign31880_e39672;
        var_erfctimesexpmtat_dn5 = assign31880_e39672_d_n5;
        var_erfctimesexpmtat_dn6 = assign31880_e39672_d_n6;
        var_erfctimesexpmtat_dn7 = assign31880_e39672_d_n7;
        var_erfctimesexpmtat_dn8 = assign31880_e39672_d_n8;

        let (assign31890_e39692, assign31890_e39692_d_n5, assign31890_e39692_d_n6, assign31890_e39692_d_n7, assign31890_e39692_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31890_e39684: f64 = (1.772453850905516 * 0.5);
        let assign31890_e39687: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign31890_e39689: f64 = (assign31890_e39687 / var_ktat);
        let assign31890_e39690: f64 = (assign31890_e39684 * assign31890_e39689);
        (assign31890_e39690, (assign31890_e39684 * ((((var_atatsti_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign31890_e39687 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign31890_e39684 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign31890_e39687 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign31890_e39684 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign31890_e39687 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign31890_e39684 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign31890_e39687 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign31890_e39692;
        var_gammamax_dn5 = assign31890_e39692_d_n5;
        var_gammamax_dn6 = assign31890_e39692_d_n6;
        var_gammamax_dn7 = assign31890_e39692_d_n7;
        var_gammamax_dn8 = assign31890_e39692_d_n8;

        let (assign31900_e39710, assign31900_e39710_d_n5, assign31900_e39710_d_n6, assign31900_e39710_d_n7, assign31900_e39710_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31900_e39705: f64 = (var_asrh * var_gammamax);
        let assign31900_e39707: f64 = (assign31900_e39705 * var_wtat);
        let assign31900_e39708: f64 = (var_ctatstid_i * assign31900_e39707);
        (assign31900_e39708, (var_ctatstid_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign31900_e39705 * var_wtat_dn5))), (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign31900_e39705 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign31900_e39705 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign31900_e39705 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign31900_e39710;
        var_itat_dn5 = assign31900_e39710_d_n5;
        var_itat_dn6 = assign31900_e39710_d_n6;
        var_itat_dn7 = assign31900_e39710_d_n7;
        var_itat_dn8 = assign31900_e39710_d_n8;

        let assign31910_e39713: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard626 = assign31910_e39713;

        let (assign31920_e39724, assign31920_e39724_d_n5, assign31920_e39724_d_n6, assign31920_e39724_d_n7, assign31920_e39724_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard626 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign31920_e39724;
        var_ibbt_dn5 = assign31920_e39724_d_n5;
        var_ibbt_dn6 = assign31920_e39724_d_n6;
        var_ibbt_dn7 = assign31920_e39724_d_n7;
        var_ibbt_dn8 = assign31920_e39724_d_n8;

        let assign31930_e39727: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard627 = assign31930_e39727;

        let (assign31940_e39746, assign31940_e39746_d_n5, assign31940_e39746_d_n6, assign31940_e39746_d_n7, assign31940_e39746_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard626 == 0.0)) && (var_guard627 != 0.0)) {
        let assign31940_e39741: f64 = (var_vbirstid_i - var_vbbt);
        let assign31940_e39743: f64 = (assign31940_e39741 * var_vbirstiinv_d);
        let assign31940_e39744: f64 = (assign31940_e39743).sqrt();
        (assign31940_e39744, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31940_e39746;
        var_tmp_dn5 = assign31940_e39746_d_n5;
        var_tmp_dn6 = assign31940_e39746_d_n6;
        var_tmp_dn7 = assign31940_e39746_d_n7;
        var_tmp_dn8 = assign31940_e39746_d_n8;

        let (assign31950_e39767, assign31950_e39767_d_n5, assign31950_e39767_d_n6, assign31950_e39767_d_n7, assign31950_e39767_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) {
        let assign31950_e39761: f64 = (var_vbirstid_i - var_vbbt);
        let assign31950_e39763: f64 = (assign31950_e39761 * var_vbirstiinv_d);
        let assign31950_e39765: f64 = (assign31950_e39763).powf(var_pstid_i);
        (assign31950_e39765, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31950_e39767;
        var_tmp_dn5 = assign31950_e39767_d_n5;
        var_tmp_dn6 = assign31950_e39767_d_n6;
        var_tmp_dn7 = assign31950_e39767_d_n7;
        var_tmp_dn8 = assign31950_e39767_d_n8;

        let (assign31960_e39787, assign31960_e39787_d_n5, assign31960_e39787_d_n6, assign31960_e39787_d_n7, assign31960_e39787_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard626 == 0.0)) {
        let assign31960_e39780: f64 = (var_vbirstid_i - var_vbbt);
        let assign31960_e39782: f64 = (assign31960_e39780 * var_wdepnulrinvsti_d);
        let assign31960_e39784: f64 = (assign31960_e39782 / var_tmp);
        let assign31960_e39785: f64 = (var_one_over_one_minus_psti_d * assign31960_e39784);
        (assign31960_e39785, (var_one_over_one_minus_psti_d * (-((assign31960_e39782 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign31960_e39782 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign31960_e39782 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign31960_e39782 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign31960_e39787;
        var_fmaxr_dn5 = assign31960_e39787_d_n5;
        var_fmaxr_dn6 = assign31960_e39787_d_n6;
        var_fmaxr_dn7 = assign31960_e39787_d_n7;
        var_fmaxr_dn8 = assign31960_e39787_d_n8;

        let assign31970_e39789: f64 = (-var_fbbtsti_d);
        let assign31970_e39791: f64 = (assign31970_e39789 / var_fmaxr);
        let assign31970_e39792: f64 = (assign31970_e39791).abs();
        let assign31970_e39794: f64 = if assign31970_e39792 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard628 = assign31970_e39794;

        let (assign31980_e39812, assign31980_e39812_d_n5, assign31980_e39812_d_n6, assign31980_e39812_d_n7, assign31980_e39812_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard626 == 0.0)) && (var_guard628 != 0.0)) {
        let assign31980_e39807: f64 = (-var_fbbtsti_d);
        let assign31980_e39809: f64 = (assign31980_e39807 / var_fmaxr);
        let assign31980_e39810: f64 = (assign31980_e39809).exp();
        (assign31980_e39810, (assign31980_e39810 * (-((assign31980_e39807 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign31980_e39810 * (-((assign31980_e39807 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign31980_e39810 * (-((assign31980_e39807 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign31980_e39810 * (-((assign31980_e39807 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31980_e39812;
        var_tmp_dn5 = assign31980_e39812_d_n5;
        var_tmp_dn6 = assign31980_e39812_d_n6;
        var_tmp_dn7 = assign31980_e39812_d_n7;
        var_tmp_dn8 = assign31980_e39812_d_n8;

        let assign31990_e39814: f64 = (-var_fbbtsti_d);
        let assign31990_e39816: f64 = (assign31990_e39814 / var_fmaxr);
        let assign31990_e39818: f64 = if assign31990_e39816 < 0.0 { 1.0 } else { 0.0 };
        var_guard629 = assign31990_e39818;

        let (assign32000_e39869, assign32000_e39869_d_n5, assign32000_e39869_d_n6, assign32000_e39869_d_n7, assign32000_e39869_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard626 == 0.0)) && (var_guard628 == 0.0)) && (var_guard629 != 0.0)) {
        let assign32000_e39836: f64 = (-230.25850929940458);
        let assign32000_e39838: f64 = (-var_fbbtsti_d);
        let assign32000_e39840: f64 = (assign32000_e39838 / var_fmaxr);
        let assign32000_e39841: f64 = (assign32000_e39836 - assign32000_e39840);
        let assign32000_e39845: f64 = (-230.25850929940458);
        let assign32000_e39847: f64 = (-var_fbbtsti_d);
        let assign32000_e39849: f64 = (assign32000_e39847 / var_fmaxr);
        let assign32000_e39850: f64 = (assign32000_e39845 - assign32000_e39849);
        let assign32000_e39853: f64 = (-230.25850929940458);
        let assign32000_e39855: f64 = (-var_fbbtsti_d);
        let assign32000_e39857: f64 = (assign32000_e39855 / var_fmaxr);
        let assign32000_e39858: f64 = (assign32000_e39853 - assign32000_e39857);
        let assign32000_e39860: f64 = (assign32000_e39858 * 0.3333333333333333);
        let assign32000_e39861: f64 = (1.0 + assign32000_e39860);
        let assign32000_e39862: f64 = (assign32000_e39850 * assign32000_e39861);
        let assign32000_e39863: f64 = (0.5 * assign32000_e39862);
        let assign32000_e39864: f64 = (1.0 + assign32000_e39863);
        let assign32000_e39865: f64 = (assign32000_e39841 * assign32000_e39864);
        let assign32000_e39866: f64 = (1.0 + assign32000_e39865);
        let assign32000_e39867: f64 = (1e-100 / assign32000_e39866);
        (assign32000_e39867, (-((1e-100 * (((-(-((assign32000_e39838 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign32000_e39864) + (assign32000_e39841 * (0.5 * (((-(-((assign32000_e39847 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign32000_e39861) + (assign32000_e39850 * ((-(-((assign32000_e39855 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign32000_e39866 * assign32000_e39866))), (-((1e-100 * (((-(-((assign32000_e39838 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign32000_e39864) + (assign32000_e39841 * (0.5 * (((-(-((assign32000_e39847 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign32000_e39861) + (assign32000_e39850 * ((-(-((assign32000_e39855 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign32000_e39866 * assign32000_e39866))), (-((1e-100 * (((-(-((assign32000_e39838 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign32000_e39864) + (assign32000_e39841 * (0.5 * (((-(-((assign32000_e39847 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign32000_e39861) + (assign32000_e39850 * ((-(-((assign32000_e39855 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign32000_e39866 * assign32000_e39866))), (-((1e-100 * (((-(-((assign32000_e39838 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign32000_e39864) + (assign32000_e39841 * (0.5 * (((-(-((assign32000_e39847 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign32000_e39861) + (assign32000_e39850 * ((-(-((assign32000_e39855 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign32000_e39866 * assign32000_e39866))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32000_e39869;
        var_tmp_dn5 = assign32000_e39869_d_n5;
        var_tmp_dn6 = assign32000_e39869_d_n6;
        var_tmp_dn7 = assign32000_e39869_d_n7;
        var_tmp_dn8 = assign32000_e39869_d_n8;

        let (assign32010_e39918, assign32010_e39918_d_n5, assign32010_e39918_d_n6, assign32010_e39918_d_n7, assign32010_e39918_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard626 == 0.0)) && (var_guard628 == 0.0)) && (var_guard629 == 0.0)) {
        let assign32010_e39888: f64 = (-var_fbbtsti_d);
        let assign32010_e39890: f64 = (assign32010_e39888 / var_fmaxr);
        let assign32010_e39892: f64 = (assign32010_e39890 - 230.25850929940458);
        let assign32010_e39896: f64 = (-var_fbbtsti_d);
        let assign32010_e39898: f64 = (assign32010_e39896 / var_fmaxr);
        let assign32010_e39900: f64 = (assign32010_e39898 - 230.25850929940458);
        let assign32010_e39903: f64 = (-var_fbbtsti_d);
        let assign32010_e39905: f64 = (assign32010_e39903 / var_fmaxr);
        let assign32010_e39907: f64 = (assign32010_e39905 - 230.25850929940458);
        let assign32010_e39909: f64 = (assign32010_e39907 * 0.3333333333333333);
        let assign32010_e39910: f64 = (1.0 + assign32010_e39909);
        let assign32010_e39911: f64 = (assign32010_e39900 * assign32010_e39910);
        let assign32010_e39912: f64 = (0.5 * assign32010_e39911);
        let assign32010_e39913: f64 = (1.0 + assign32010_e39912);
        let assign32010_e39914: f64 = (assign32010_e39892 * assign32010_e39913);
        let assign32010_e39915: f64 = (1.0 + assign32010_e39914);
        let assign32010_e39916: f64 = (1e100 * assign32010_e39915);
        (assign32010_e39916, (1e100 * (((-((assign32010_e39888 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign32010_e39913) + (assign32010_e39892 * (0.5 * (((-((assign32010_e39896 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign32010_e39910) + (assign32010_e39900 * ((-((assign32010_e39903 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign32010_e39888 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign32010_e39913) + (assign32010_e39892 * (0.5 * (((-((assign32010_e39896 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign32010_e39910) + (assign32010_e39900 * ((-((assign32010_e39903 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign32010_e39888 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign32010_e39913) + (assign32010_e39892 * (0.5 * (((-((assign32010_e39896 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign32010_e39910) + (assign32010_e39900 * ((-((assign32010_e39903 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign32010_e39888 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign32010_e39913) + (assign32010_e39892 * (0.5 * (((-((assign32010_e39896 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign32010_e39910) + (assign32010_e39900 * ((-((assign32010_e39903 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32010_e39918;
        var_tmp_dn5 = assign32010_e39918_d_n5;
        var_tmp_dn6 = assign32010_e39918_d_n6;
        var_tmp_dn7 = assign32010_e39918_d_n7;
        var_tmp_dn8 = assign32010_e39918_d_n8;

        let (assign32020_e39938, assign32020_e39938_d_n5, assign32020_e39938_d_n6, assign32020_e39938_d_n7, assign32020_e39938_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard626 == 0.0)) {
        let assign32020_e39931: f64 = (var_v2 * var_fmaxr);
        let assign32020_e39933: f64 = (assign32020_e39931 * var_fmaxr);
        let assign32020_e39935: f64 = (assign32020_e39933 * var_tmp);
        let assign32020_e39936: f64 = (var_cbbtstid_i * assign32020_e39935);
        (assign32020_e39936, (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign32020_e39931 * var_fmaxr_dn5)) * var_tmp) + (assign32020_e39933 * var_tmp_dn5))), (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign32020_e39931 * var_fmaxr_dn6)) * var_tmp) + (assign32020_e39933 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign32020_e39931 * var_fmaxr_dn7)) * var_tmp) + (assign32020_e39933 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign32020_e39931 * var_fmaxr_dn8)) * var_tmp) + (assign32020_e39933 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign32020_e39938;
        var_ibbt_dn5 = assign32020_e39938_d_n5;
        var_ibbt_dn6 = assign32020_e39938_d_n6;
        var_ibbt_dn7 = assign32020_e39938_d_n7;
        var_ibbt_dn8 = assign32020_e39938_d_n8;

        let assign32030_e39941: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard630 = assign32030_e39941;

        let (assign32040_e39952, assign32040_e39952_d_n5, assign32040_e39952_d_n6, assign32040_e39952_d_n7, assign32040_e39952_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard630 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign32040_e39952;
        var_fbreakdown_dn5 = assign32040_e39952_d_n5;
        var_fbreakdown_dn6 = assign32040_e39952_d_n6;
        var_fbreakdown_dn7 = assign32040_e39952_d_n7;
        var_fbreakdown_dn8 = assign32040_e39952_d_n8;

        let assign32050_e39955: f64 = (-var_alphaav);
        let assign32050_e39957: f64 = (assign32050_e39955 * var_vbrstid_i);
        let assign32050_e39958: f64 = if var_vav > assign32050_e39957 { 1.0 } else { 0.0 };
        var_guard631 = assign32050_e39958;

        let assign32060_e39961: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard632 = assign32060_e39961;

        let (assign32070_e39991, assign32070_e39991_d_n5, assign32070_e39991_d_n6, assign32070_e39991_d_n7, assign32070_e39991_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard630 == 0.0)) && (var_guard631 != 0.0)) && (var_guard632 != 0.0)) {
        let assign32070_e39977: f64 = (var_vav * var_vbrinvsti_d);
        let assign32070_e39980: f64 = (var_vav * var_vbrinvsti_d);
        let assign32070_e39981: f64 = (assign32070_e39977 * assign32070_e39980);
        let assign32070_e39984: f64 = (var_vav * var_vbrinvsti_d);
        let assign32070_e39985: f64 = (assign32070_e39981 * assign32070_e39984);
        let assign32070_e39988: f64 = (var_vav * var_vbrinvsti_d);
        let assign32070_e39989: f64 = (assign32070_e39985 * assign32070_e39988);
        (assign32070_e39989, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32070_e39991;
        var_tmp_dn5 = assign32070_e39991_d_n5;
        var_tmp_dn6 = assign32070_e39991_d_n6;
        var_tmp_dn7 = assign32070_e39991_d_n7;
        var_tmp_dn8 = assign32070_e39991_d_n8;

        let (assign32080_e40013, assign32080_e40013_d_n5, assign32080_e40013_d_n6, assign32080_e40013_d_n7, assign32080_e40013_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard630 == 0.0)) && (var_guard631 != 0.0)) && (var_guard632 == 0.0)) {
        let assign32080_e40008: f64 = (var_vav * var_vbrinvsti_d);
        let assign32080_e40009: f64 = (assign32080_e40008).abs();
        let assign32080_e40011: f64 = (assign32080_e40009).powf(var_pbrstid_i);
        (assign32080_e40011, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32080_e40013;
        var_tmp_dn5 = assign32080_e40013_d_n5;
        var_tmp_dn6 = assign32080_e40013_d_n6;
        var_tmp_dn7 = assign32080_e40013_d_n7;
        var_tmp_dn8 = assign32080_e40013_d_n8;

        let (assign32090_e40031, assign32090_e40031_d_n5, assign32090_e40031_d_n6, assign32090_e40031_d_n7, assign32090_e40031_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard630 == 0.0)) && (var_guard631 != 0.0)) {
        let assign32090_e40028: f64 = (1.0 - var_tmp);
        let assign32090_e40029: f64 = (1.0 / assign32090_e40028);
        (assign32090_e40029, (-((-var_tmp_dn5) / (assign32090_e40028 * assign32090_e40028))), (-((-var_tmp_dn6) / (assign32090_e40028 * assign32090_e40028))), (-((-var_tmp_dn7) / (assign32090_e40028 * assign32090_e40028))), (-((-var_tmp_dn8) / (assign32090_e40028 * assign32090_e40028))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign32090_e40031;
        var_fbreakdown_dn5 = assign32090_e40031_d_n5;
        var_fbreakdown_dn6 = assign32090_e40031_d_n6;
        var_fbreakdown_dn7 = assign32090_e40031_d_n7;
        var_fbreakdown_dn8 = assign32090_e40031_d_n8;

        let (assign32100_e40054, assign32100_e40054_d_n5, assign32100_e40054_d_n6, assign32100_e40054_d_n7, assign32100_e40054_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard630 == 0.0)) && (var_guard631 == 0.0)) {
        let assign32100_e40048: f64 = (var_alphaav * var_vbrstid_i);
        let assign32100_e40049: f64 = (var_vav + assign32100_e40048);
        let assign32100_e40051: f64 = (assign32100_e40049 * var_slopesti_d);
        let assign32100_e40052: f64 = (var_fstopsti_d + assign32100_e40051);
        (assign32100_e40052, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign32100_e40054;
        var_fbreakdown_dn5 = assign32100_e40054_d_n5;
        var_fbreakdown_dn6 = assign32100_e40054_d_n6;
        var_fbreakdown_dn7 = assign32100_e40054_d_n7;
        var_fbreakdown_dn8 = assign32100_e40054_d_n8;

        let (assign32110_e40073, assign32110_e40073_d_n5, assign32110_e40073_d_n6, assign32110_e40073_d_n7, assign32110_e40073_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) {
        let assign32110_e40064: f64 = (var_id__blk219 + var_isrh);
        let assign32110_e40066: f64 = (assign32110_e40064 + var_itat);
        let assign32110_e40068: f64 = (assign32110_e40066 + var_ibbt);
        let assign32110_e40069: f64 = (p.p29 * assign32110_e40068);
        let assign32110_e40071: f64 = (assign32110_e40069 * var_fbreakdown);
        (assign32110_e40071, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign32110_e40069 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign32110_e40069 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign32110_e40069 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign32110_e40069 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign32110_e40073;
        var_ijunsti_dn5 = assign32110_e40073_d_n5;
        var_ijunsti_dn6 = assign32110_e40073_d_n6;
        var_ijunsti_dn7 = assign32110_e40073_d_n7;
        var_ijunsti_dn8 = assign32110_e40073_d_n8;

        let assign32120_e40076: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard633 = assign32120_e40076;

        let (assign32130_e40084, assign32130_e40084_d_n5, assign32130_e40084_d_n6, assign32130_e40084_d_n7, assign32130_e40084_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign32130_e40084;
        var_ijungat_dn5 = assign32130_e40084_d_n5;
        var_ijungat_dn6 = assign32130_e40084_d_n6;
        var_ijungat_dn7 = assign32130_e40084_d_n7;
        var_ijungat_dn8 = assign32130_e40084_d_n8;

        let (assign32140_e40095,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) {
        let assign32140_e40093: f64 = (var_idsatgat_d * var_idmult);
        (assign32140_e40093,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign32140_e40095;

        let assign32150_e40102: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard634 = assign32150_e40102;

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
        *var_guard623_slot = var_guard623;
        *var_guard624_slot = var_guard624;
        *var_guard625_slot = var_guard625;
        *var_guard626_slot = var_guard626;
        *var_guard627_slot = var_guard627;
        *var_guard628_slot = var_guard628;
        *var_guard629_slot = var_guard629;
        *var_guard630_slot = var_guard630;
        *var_guard631_slot = var_guard631;
        *var_guard632_slot = var_guard632;
        *var_guard633_slot = var_guard633;
        *var_guard634_slot = var_guard634;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk219_slot = var_id__blk219;
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

    pub(super) fn stamp_transient_block_65(
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btatpartgat_d: f64,
        var_cerfc: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_ftdgat_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard633: f64,
        var_guard634: f64,
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
        var_guard635_slot: &mut f64,
        var_guard636_slot: &mut f64,
        var_guard637_slot: &mut f64,
        var_guard638_slot: &mut f64,
        var_guard639_slot: &mut f64,
        var_guard640_slot: &mut f64,
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
        let mut var_guard635: f64 = *var_guard635_slot;
        let mut var_guard636: f64 = *var_guard636_slot;
        let mut var_guard637: f64 = *var_guard637_slot;
        let mut var_guard638: f64 = *var_guard638_slot;
        let mut var_guard639: f64 = *var_guard639_slot;
        let mut var_guard640: f64 = *var_guard640_slot;
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

        let (assign32160_e40113, assign32160_e40113_d_n5, assign32160_e40113_d_n6, assign32160_e40113_d_n7, assign32160_e40113_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign32160_e40113;
        var_isrh_dn5 = assign32160_e40113_d_n5;
        var_isrh_dn6 = assign32160_e40113_d_n6;
        var_isrh_dn7 = assign32160_e40113_d_n7;
        var_isrh_dn8 = assign32160_e40113_d_n8;

        let (assign32170_e40127,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 == 0.0)) {
        let assign32170_e40125: f64 = (var_vbigat_d - var_vjsrh);
        (assign32170_e40125,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign32170_e40127;

        let (assign32180_e40146,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 == 0.0)) {
        let assign32180_e40141: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign32180_e40142: f64 = (1.0 - assign32180_e40141);
        let assign32180_e40143: f64 = (assign32180_e40142).sqrt();
        let assign32180_e40144: f64 = (1.0 - assign32180_e40143);
        (assign32180_e40144,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign32180_e40146;

        let assign32190_e40149: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard635 = assign32190_e40149;

        let (assign32200_e40163,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 == 0.0)) && (var_guard635 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign32200_e40163;

        let (assign32210_e40195,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 == 0.0)) && (var_guard635 == 0.0)) {
        let assign32210_e40178: f64 = (var_wsrhstep * var_wsrhstep);
        let assign32210_e40180: f64 = (var_wsrhstep).ln();
        let assign32210_e40181: f64 = (assign32210_e40178 * assign32210_e40180);
        let assign32210_e40184: f64 = (1.0 - var_wsrhstep);
        let assign32210_e40185: f64 = (assign32210_e40181 / assign32210_e40184);
        let assign32210_e40187: f64 = (assign32210_e40185 + var_wsrhstep);
        let assign32210_e40191: f64 = (2.0 * var_pgatd_i);
        let assign32210_e40192: f64 = (1.0 - assign32210_e40191);
        let assign32210_e40193: f64 = (assign32210_e40187 * assign32210_e40192);
        (assign32210_e40193,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign32210_e40195;

        let (assign32220_e40209,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 == 0.0)) {
        let assign32220_e40207: f64 = (var_wsrhstep + var_dwsrh);
        (assign32220_e40207,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign32220_e40209;

        let assign32230_e40212: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard636 = assign32230_e40212;

        let (assign32240_e40229, assign32240_e40229_d_n5, assign32240_e40229_d_n6, assign32240_e40229_d_n7, assign32240_e40229_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 == 0.0)) && (var_guard636 != 0.0)) {
        let assign32240_e40226: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign32240_e40227: f64 = (assign32240_e40226).sqrt();
        (assign32240_e40227, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32240_e40229;
        var_tmp_dn5 = assign32240_e40229_d_n5;
        var_tmp_dn6 = assign32240_e40229_d_n6;
        var_tmp_dn7 = assign32240_e40229_d_n7;
        var_tmp_dn8 = assign32240_e40229_d_n8;

        let (assign32250_e40248, assign32250_e40248_d_n5, assign32250_e40248_d_n6, assign32250_e40248_d_n7, assign32250_e40248_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 == 0.0)) && (var_guard636 == 0.0)) {
        let assign32250_e40244: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign32250_e40246: f64 = (assign32250_e40244).powf(var_pgatd_i);
        (assign32250_e40246, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32250_e40248;
        var_tmp_dn5 = assign32250_e40248_d_n5;
        var_tmp_dn6 = assign32250_e40248_d_n6;
        var_tmp_dn7 = assign32250_e40248_d_n7;
        var_tmp_dn8 = assign32250_e40248_d_n8;

        let (assign32260_e40262, assign32260_e40262_d_n5, assign32260_e40262_d_n6, assign32260_e40262_d_n7, assign32260_e40262_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 == 0.0)) {
        let assign32260_e40260: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign32260_e40260, (var_wdepnulrgat_d * var_tmp_dn5), (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign32260_e40262;
        var_wdep_dn5 = assign32260_e40262_d_n5;
        var_wdep_dn6 = assign32260_e40262_d_n6;
        var_wdep_dn7 = assign32260_e40262_d_n7;
        var_wdep_dn8 = assign32260_e40262_d_n8;

        let (assign32270_e40280, assign32270_e40280_d_n5, assign32270_e40280_d_n6, assign32270_e40280_d_n7, assign32270_e40280_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 == 0.0)) {
        let assign32270_e40275: f64 = (var_zinv - 1.0);
        let assign32270_e40277: f64 = (assign32270_e40275 * var_wdep);
        let assign32270_e40278: f64 = (var_ftdgat_d * assign32270_e40277);
        (assign32270_e40278, (var_ftdgat_d * (assign32270_e40275 * var_wdep_dn5)), (var_ftdgat_d * (assign32270_e40275 * var_wdep_dn6)), (var_ftdgat_d * (assign32270_e40275 * var_wdep_dn7)), (var_ftdgat_d * (assign32270_e40275 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign32270_e40280;
        var_asrh_dn5 = assign32270_e40280_d_n5;
        var_asrh_dn6 = assign32270_e40280_d_n6;
        var_asrh_dn7 = assign32270_e40280_d_n7;
        var_asrh_dn8 = assign32270_e40280_d_n8;

        let (assign32280_e40296, assign32280_e40296_d_n5, assign32280_e40296_d_n6, assign32280_e40296_d_n7, assign32280_e40296_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard634 == 0.0)) {
        let assign32280_e40293: f64 = (var_asrh * var_wsrh);
        let assign32280_e40294: f64 = (var_csrhgatd_i * assign32280_e40293);
        (assign32280_e40294, (var_csrhgatd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign32280_e40296;
        var_isrh_dn5 = assign32280_e40296_d_n5;
        var_isrh_dn6 = assign32280_e40296_d_n6;
        var_isrh_dn7 = assign32280_e40296_d_n7;
        var_isrh_dn8 = assign32280_e40296_d_n8;

        let assign32290_e40299: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard637 = assign32290_e40299;

        let (assign32300_e40310, assign32300_e40310_d_n5, assign32300_e40310_d_n6, assign32300_e40310_d_n7, assign32300_e40310_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign32300_e40310;
        var_itat_dn5 = assign32300_e40310_d_n5;
        var_itat_dn6 = assign32300_e40310_d_n6;
        var_itat_dn7 = assign32300_e40310_d_n7;
        var_itat_dn8 = assign32300_e40310_d_n8;

        let (assign32310_e40328, assign32310_e40328_d_n5, assign32310_e40328_d_n6, assign32310_e40328_d_n7, assign32310_e40328_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32310_e40323: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign32310_e40325: f64 = (assign32310_e40323 / var_vbi_minus_vjsrh);
        let assign32310_e40326: f64 = (var_btatpartgat_d * assign32310_e40325);
        (assign32310_e40326, (var_btatpartgat_d * ((var_wdep_dn5 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign32310_e40328;
        var_btat_dn5 = assign32310_e40328_d_n5;
        var_btat_dn6 = assign32310_e40328_d_n6;
        var_btat_dn7 = assign32310_e40328_d_n7;
        var_btat_dn8 = assign32310_e40328_d_n8;

        let (assign32320_e40344, assign32320_e40344_d_n5, assign32320_e40344_d_n6, assign32320_e40344_d_n7, assign32320_e40344_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32320_e40340: f64 = (0.666666666666667 * var_atatgat_d);
        let assign32320_e40342: f64 = (assign32320_e40340 / var_btat);
        (assign32320_e40342, (-((assign32320_e40340 * var_btat_dn5) / (var_btat * var_btat))), (-((assign32320_e40340 * var_btat_dn6) / (var_btat * var_btat))), (-((assign32320_e40340 * var_btat_dn7) / (var_btat * var_btat))), (-((assign32320_e40340 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign32320_e40344;
        var_twoatatoverthreebtat_dn5 = assign32320_e40344_d_n5;
        var_twoatatoverthreebtat_dn6 = assign32320_e40344_d_n6;
        var_twoatatoverthreebtat_dn7 = assign32320_e40344_d_n7;
        var_twoatatoverthreebtat_dn8 = assign32320_e40344_d_n8;

        let (assign32330_e40358, assign32330_e40358_d_n5, assign32330_e40358_d_n6, assign32330_e40358_d_n7, assign32330_e40358_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32330_e40356: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign32330_e40356, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign32330_e40358;
        var_umaxbeforelimiting_dn5 = assign32330_e40358_d_n5;
        var_umaxbeforelimiting_dn6 = assign32330_e40358_d_n6;
        var_umaxbeforelimiting_dn7 = assign32330_e40358_d_n7;
        var_umaxbeforelimiting_dn8 = assign32330_e40358_d_n8;

        let (assign32340_e40379, assign32340_e40379_d_n5, assign32340_e40379_d_n6, assign32340_e40379_d_n7, assign32340_e40379_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32340_e40370: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign32340_e40373: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign32340_e40375: f64 = (assign32340_e40373 + 1.0);
        let assign32340_e40376: f64 = (assign32340_e40370 / assign32340_e40375);
        let assign32340_e40377: f64 = (assign32340_e40376).sqrt();
        (assign32340_e40377, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign32340_e40375) - (assign32340_e40370 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign32340_e40375 * assign32340_e40375)) / (2.0 * assign32340_e40377)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign32340_e40375) - (assign32340_e40370 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign32340_e40375 * assign32340_e40375)) / (2.0 * assign32340_e40377)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign32340_e40375) - (assign32340_e40370 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign32340_e40375 * assign32340_e40375)) / (2.0 * assign32340_e40377)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign32340_e40375) - (assign32340_e40370 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign32340_e40375 * assign32340_e40375)) / (2.0 * assign32340_e40377)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign32340_e40379;
        var_umax_dn5 = assign32340_e40379_d_n5;
        var_umax_dn6 = assign32340_e40379_d_n6;
        var_umax_dn7 = assign32340_e40379_d_n7;
        var_umax_dn8 = assign32340_e40379_d_n8;

        let (assign32350_e40392, assign32350_e40392_d_n5, assign32350_e40392_d_n6, assign32350_e40392_d_n7, assign32350_e40392_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32350_e40390: f64 = (var_umax).sqrt();
        (assign32350_e40390, (var_umax_dn5 / (2.0 * assign32350_e40390)), (var_umax_dn6 / (2.0 * assign32350_e40390)), (var_umax_dn7 / (2.0 * assign32350_e40390)), (var_umax_dn8 / (2.0 * assign32350_e40390)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign32350_e40392;
        var_sqrtumax_dn5 = assign32350_e40392_d_n5;
        var_sqrtumax_dn6 = assign32350_e40392_d_n6;
        var_sqrtumax_dn7 = assign32350_e40392_d_n7;
        var_sqrtumax_dn8 = assign32350_e40392_d_n8;

        let (assign32360_e40406, assign32360_e40406_d_n5, assign32360_e40406_d_n6, assign32360_e40406_d_n7, assign32360_e40406_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32360_e40404: f64 = (var_umax * var_sqrtumax);
        (assign32360_e40404, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign32360_e40406;
        var_umaxpoweronepointfive_dn5 = assign32360_e40406_d_n5;
        var_umaxpoweronepointfive_dn6 = assign32360_e40406_d_n6;
        var_umaxpoweronepointfive_dn7 = assign32360_e40406_d_n7;
        var_umaxpoweronepointfive_dn8 = assign32360_e40406_d_n8;

        let assign32370_e40408: f64 = (-var_pgatd_i);
        let assign32370_e40410: f64 = (assign32370_e40408 * var_one_over_one_minus_pgat_d);
        let assign32370_e40412: f64 = (-1.0);
        let assign32370_e40413: f64 = if assign32370_e40410 == assign32370_e40412 { 1.0 } else { 0.0 };
        var_guard638 = assign32370_e40413;

        let (assign32380_e40433, assign32380_e40433_d_n5, assign32380_e40433_d_n6, assign32380_e40433_d_n7, assign32380_e40433_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) && (var_guard638 != 0.0)) {
        let assign32380_e40429: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign32380_e40430: f64 = (1.0 + assign32380_e40429);
        let assign32380_e40431: f64 = (1.0 / assign32380_e40430);
        (assign32380_e40431, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign32380_e40430 * assign32380_e40430))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign32380_e40430 * assign32380_e40430))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign32380_e40430 * assign32380_e40430))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign32380_e40430 * assign32380_e40430))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign32380_e40433;
        var_wgamma_dn5 = assign32380_e40433_d_n5;
        var_wgamma_dn6 = assign32380_e40433_d_n6;
        var_wgamma_dn7 = assign32380_e40433_d_n7;
        var_wgamma_dn8 = assign32380_e40433_d_n8;

        let (assign32390_e40457, assign32390_e40457_d_n5, assign32390_e40457_d_n6, assign32390_e40457_d_n7, assign32390_e40457_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) && (var_guard638 == 0.0)) {
        let assign32390_e40449: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign32390_e40450: f64 = (1.0 + assign32390_e40449);
        let assign32390_e40452: f64 = (-var_pgatd_i);
        let assign32390_e40454: f64 = (assign32390_e40452 * var_one_over_one_minus_pgat_d);
        let assign32390_e40455: f64 = (assign32390_e40450).powf(assign32390_e40454);
        (assign32390_e40455, if 0.0 == 0.0 && ((assign32390_e40454) as f64).is_finite() && ((assign32390_e40454) as f64).fract() == 0.0 { if assign32390_e40454 == 0.0 { 0.0 } else { (assign32390_e40454 * ((assign32390_e40450).powf(assign32390_e40454 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign32390_e40455 * (assign32390_e40454 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign32390_e40450))) }, if 0.0 == 0.0 && ((assign32390_e40454) as f64).is_finite() && ((assign32390_e40454) as f64).fract() == 0.0 { if assign32390_e40454 == 0.0 { 0.0 } else { (assign32390_e40454 * ((assign32390_e40450).powf(assign32390_e40454 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign32390_e40455 * (assign32390_e40454 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign32390_e40450))) }, if 0.0 == 0.0 && ((assign32390_e40454) as f64).is_finite() && ((assign32390_e40454) as f64).fract() == 0.0 { if assign32390_e40454 == 0.0 { 0.0 } else { (assign32390_e40454 * ((assign32390_e40450).powf(assign32390_e40454 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign32390_e40455 * (assign32390_e40454 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign32390_e40450))) }, if 0.0 == 0.0 && ((assign32390_e40454) as f64).is_finite() && ((assign32390_e40454) as f64).fract() == 0.0 { if assign32390_e40454 == 0.0 { 0.0 } else { (assign32390_e40454 * ((assign32390_e40450).powf(assign32390_e40454 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign32390_e40455 * (assign32390_e40454 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign32390_e40450))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign32390_e40457;
        var_wgamma_dn5 = assign32390_e40457_d_n5;
        var_wgamma_dn6 = assign32390_e40457_d_n6;
        var_wgamma_dn7 = assign32390_e40457_d_n7;
        var_wgamma_dn8 = assign32390_e40457_d_n8;

        let (assign32400_e40475, assign32400_e40475_d_n5, assign32400_e40475_d_n6, assign32400_e40475_d_n7, assign32400_e40475_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32400_e40469: f64 = (var_wsrh * var_wgamma);
        let assign32400_e40472: f64 = (var_wsrh + var_wgamma);
        let assign32400_e40473: f64 = (assign32400_e40469 / assign32400_e40472);
        (assign32400_e40473, ((((var_wsrh * var_wgamma_dn5) * assign32400_e40472) - (assign32400_e40469 * var_wgamma_dn5)) / (assign32400_e40472 * assign32400_e40472)), ((((var_wsrh * var_wgamma_dn6) * assign32400_e40472) - (assign32400_e40469 * var_wgamma_dn6)) / (assign32400_e40472 * assign32400_e40472)), ((((var_wsrh * var_wgamma_dn7) * assign32400_e40472) - (assign32400_e40469 * var_wgamma_dn7)) / (assign32400_e40472 * assign32400_e40472)), ((((var_wsrh * var_wgamma_dn8) * assign32400_e40472) - (assign32400_e40469 * var_wgamma_dn8)) / (assign32400_e40472 * assign32400_e40472)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign32400_e40475;
        var_wtat_dn5 = assign32400_e40475_d_n5;
        var_wtat_dn6 = assign32400_e40475_d_n6;
        var_wtat_dn7 = assign32400_e40475_d_n7;
        var_wtat_dn8 = assign32400_e40475_d_n8;

        let (assign32410_e40492, assign32410_e40492_d_n5, assign32410_e40492_d_n6, assign32410_e40492_d_n7, assign32410_e40492_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32410_e40488: f64 = (var_btat / var_sqrtumax);
        let assign32410_e40489: f64 = (0.375 * assign32410_e40488);
        let assign32410_e40490: f64 = (assign32410_e40489).sqrt();
        (assign32410_e40490, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32410_e40490)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32410_e40490)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32410_e40490)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32410_e40490)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign32410_e40492;
        var_ktat_dn5 = assign32410_e40492_d_n5;
        var_ktat_dn6 = assign32410_e40492_d_n6;
        var_ktat_dn7 = assign32410_e40492_d_n7;
        var_ktat_dn8 = assign32410_e40492_d_n8;

        let (assign32420_e40510, assign32420_e40510_d_n5, assign32420_e40510_d_n6, assign32420_e40510_d_n7, assign32420_e40510_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32420_e40505: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign32420_e40506: f64 = (2.0 * assign32420_e40505);
        let assign32420_e40508: f64 = (assign32420_e40506 - var_umax);
        (assign32420_e40508, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign32420_e40510;
        var_ltat_dn5 = assign32420_e40510_d_n5;
        var_ltat_dn6 = assign32420_e40510_d_n6;
        var_ltat_dn7 = assign32420_e40510_d_n7;
        var_ltat_dn8 = assign32420_e40510_d_n8;

        let (assign32430_e40536, assign32430_e40536_d_n5, assign32430_e40536_d_n6, assign32430_e40536_d_n7, assign32430_e40536_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32430_e40522: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign32430_e40524: f64 = (assign32430_e40522 * var_sqrtumax);
        let assign32430_e40527: f64 = (var_atatgat_d * var_umax);
        let assign32430_e40528: f64 = (assign32430_e40524 - assign32430_e40527);
        let assign32430_e40532: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign32430_e40533: f64 = (0.5 * assign32430_e40532);
        let assign32430_e40534: f64 = (assign32430_e40528 + assign32430_e40533);
        (assign32430_e40534, (((((var_atatgat_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign32430_e40522 * var_sqrtumax_dn5)) - (var_atatgat_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign32430_e40522 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign32430_e40522 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign32430_e40522 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign32430_e40536;
        var_mtat_dn5 = assign32430_e40536_d_n5;
        var_mtat_dn6 = assign32430_e40536_d_n6;
        var_mtat_dn7 = assign32430_e40536_d_n7;
        var_mtat_dn8 = assign32430_e40536_d_n8;

        let (assign32440_e40552, assign32440_e40552_d_n5, assign32440_e40552_d_n6, assign32440_e40552_d_n7, assign32440_e40552_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32440_e40548: f64 = (var_ltat - 1.0);
        let assign32440_e40550: f64 = (assign32440_e40548 * var_ktat);
        (assign32440_e40550, ((var_ltat_dn5 * var_ktat) + (assign32440_e40548 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign32440_e40548 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign32440_e40548 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign32440_e40548 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign32440_e40552;
        var_xerfc_dn5 = assign32440_e40552_d_n5;
        var_xerfc_dn6 = assign32440_e40552_d_n6;
        var_xerfc_dn7 = assign32440_e40552_d_n7;
        var_xerfc_dn8 = assign32440_e40552_d_n8;

        let (assign32450_e40566, assign32450_e40566_d_n5, assign32450_e40566_d_n6, assign32450_e40566_d_n7, assign32450_e40566_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32450_e40564: f64 = (var_xerfc * var_xerfc);
        (assign32450_e40564, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign32450_e40566;
        var_ysq_dn5 = assign32450_e40566_d_n5;
        var_ysq_dn6 = assign32450_e40566_d_n6;
        var_ysq_dn7 = assign32450_e40566_d_n7;
        var_ysq_dn8 = assign32450_e40566_d_n8;

        let assign32460_e40569: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard639 = assign32460_e40569;

        let (assign32470_e40589, assign32470_e40589_d_n5, assign32470_e40589_d_n6, assign32470_e40589_d_n7, assign32470_e40589_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) && (var_guard639 != 0.0)) {
        let assign32470_e40585: f64 = (var_perfc * var_xerfc);
        let assign32470_e40586: f64 = (1.0 + assign32470_e40585);
        let assign32470_e40587: f64 = (1.0 / assign32470_e40586);
        (assign32470_e40587, (-((var_perfc * var_xerfc_dn5) / (assign32470_e40586 * assign32470_e40586))), (-((var_perfc * var_xerfc_dn6) / (assign32470_e40586 * assign32470_e40586))), (-((var_perfc * var_xerfc_dn7) / (assign32470_e40586 * assign32470_e40586))), (-((var_perfc * var_xerfc_dn8) / (assign32470_e40586 * assign32470_e40586))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign32470_e40589;
        var_terfc_dn5 = assign32470_e40589_d_n5;
        var_terfc_dn6 = assign32470_e40589_d_n6;
        var_terfc_dn7 = assign32470_e40589_d_n7;
        var_terfc_dn8 = assign32470_e40589_d_n8;

        let (assign32480_e40610, assign32480_e40610_d_n5, assign32480_e40610_d_n6, assign32480_e40610_d_n7, assign32480_e40610_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) && (var_guard639 == 0.0)) {
        let assign32480_e40606: f64 = (var_perfc * var_xerfc);
        let assign32480_e40607: f64 = (1.0 - assign32480_e40606);
        let assign32480_e40608: f64 = (1.0 / assign32480_e40607);
        (assign32480_e40608, (-((-(var_perfc * var_xerfc_dn5)) / (assign32480_e40607 * assign32480_e40607))), (-((-(var_perfc * var_xerfc_dn6)) / (assign32480_e40607 * assign32480_e40607))), (-((-(var_perfc * var_xerfc_dn7)) / (assign32480_e40607 * assign32480_e40607))), (-((-(var_perfc * var_xerfc_dn8)) / (assign32480_e40607 * assign32480_e40607))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign32480_e40610;
        var_terfc_dn5 = assign32480_e40610_d_n5;
        var_terfc_dn6 = assign32480_e40610_d_n6;
        var_terfc_dn7 = assign32480_e40610_d_n7;
        var_terfc_dn8 = assign32480_e40610_d_n8;

        let assign32490_e40612: f64 = (-var_ysq);
        let assign32490_e40614: f64 = (assign32490_e40612 + var_mtat);
        let assign32490_e40616: f64 = (-230.25850929940458);
        let assign32490_e40617: f64 = if assign32490_e40614 > assign32490_e40616 { 1.0 } else { 0.0 };
        var_guard640 = assign32490_e40617;

        let (assign32500_e40635, assign32500_e40635_d_n5, assign32500_e40635_d_n6, assign32500_e40635_d_n7, assign32500_e40635_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) && (var_guard640 != 0.0)) {
        let assign32500_e40630: f64 = (-var_ysq);
        let assign32500_e40632: f64 = (assign32500_e40630 + var_mtat);
        let assign32500_e40633: f64 = (assign32500_e40632).exp();
        (assign32500_e40633, (assign32500_e40633 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign32500_e40633 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign32500_e40633 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign32500_e40633 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32500_e40635;
        var_tmp_dn5 = assign32500_e40635_d_n5;
        var_tmp_dn6 = assign32500_e40635_d_n6;
        var_tmp_dn7 = assign32500_e40635_d_n7;
        var_tmp_dn8 = assign32500_e40635_d_n8;

        let (assign32510_e40684, assign32510_e40684_d_n5, assign32510_e40684_d_n6, assign32510_e40684_d_n7, assign32510_e40684_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) && (var_guard640 == 0.0)) {
        let assign32510_e40651: f64 = (-230.25850929940458);
        let assign32510_e40653: f64 = (-var_ysq);
        let assign32510_e40655: f64 = (assign32510_e40653 + var_mtat);
        let assign32510_e40656: f64 = (assign32510_e40651 - assign32510_e40655);
        let assign32510_e40660: f64 = (-230.25850929940458);
        let assign32510_e40662: f64 = (-var_ysq);
        let assign32510_e40664: f64 = (assign32510_e40662 + var_mtat);
        let assign32510_e40665: f64 = (assign32510_e40660 - assign32510_e40664);
        let assign32510_e40668: f64 = (-230.25850929940458);
        let assign32510_e40670: f64 = (-var_ysq);
        let assign32510_e40672: f64 = (assign32510_e40670 + var_mtat);
        let assign32510_e40673: f64 = (assign32510_e40668 - assign32510_e40672);
        let assign32510_e40675: f64 = (assign32510_e40673 * 0.3333333333333333);
        let assign32510_e40676: f64 = (1.0 + assign32510_e40675);
        let assign32510_e40677: f64 = (assign32510_e40665 * assign32510_e40676);
        let assign32510_e40678: f64 = (0.5 * assign32510_e40677);
        let assign32510_e40679: f64 = (1.0 + assign32510_e40678);
        let assign32510_e40680: f64 = (assign32510_e40656 * assign32510_e40679);
        let assign32510_e40681: f64 = (1.0 + assign32510_e40680);
        let assign32510_e40682: f64 = (1e-100 / assign32510_e40681);
        (assign32510_e40682, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign32510_e40679) + (assign32510_e40656 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign32510_e40676) + (assign32510_e40665 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign32510_e40681 * assign32510_e40681))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign32510_e40679) + (assign32510_e40656 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign32510_e40676) + (assign32510_e40665 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign32510_e40681 * assign32510_e40681))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign32510_e40679) + (assign32510_e40656 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign32510_e40676) + (assign32510_e40665 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign32510_e40681 * assign32510_e40681))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign32510_e40679) + (assign32510_e40656 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign32510_e40676) + (assign32510_e40665 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign32510_e40681 * assign32510_e40681))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32510_e40684;
        var_tmp_dn5 = assign32510_e40684_d_n5;
        var_tmp_dn6 = assign32510_e40684_d_n6;
        var_tmp_dn7 = assign32510_e40684_d_n7;
        var_tmp_dn8 = assign32510_e40684_d_n8;

        let (assign32520_e40714, assign32520_e40714_d_n5, assign32520_e40714_d_n6, assign32520_e40714_d_n7, assign32520_e40714_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32520_e40696: f64 = (0.29214664 * var_terfc);
        let assign32520_e40700: f64 = (var_terfc * var_terfc);
        let assign32520_e40701: f64 = (var_berfc * assign32520_e40700);
        let assign32520_e40702: f64 = (assign32520_e40696 + assign32520_e40701);
        let assign32520_e40706: f64 = (var_terfc * var_terfc);
        let assign32520_e40708: f64 = (assign32520_e40706 * var_terfc);
        let assign32520_e40709: f64 = (var_cerfc * assign32520_e40708);
        let assign32520_e40710: f64 = (assign32520_e40702 + assign32520_e40709);
        let assign32520_e40712: f64 = (assign32520_e40710 * var_tmp);
        (assign32520_e40712, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign32520_e40706 * var_terfc_dn5)))) * var_tmp) + (assign32520_e40710 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign32520_e40706 * var_terfc_dn6)))) * var_tmp) + (assign32520_e40710 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign32520_e40706 * var_terfc_dn7)))) * var_tmp) + (assign32520_e40710 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign32520_e40706 * var_terfc_dn8)))) * var_tmp) + (assign32520_e40710 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign32520_e40714;
        var_erfcpos_dn5 = assign32520_e40714_d_n5;
        var_erfcpos_dn6 = assign32520_e40714_d_n6;
        var_erfcpos_dn7 = assign32520_e40714_d_n7;
        var_erfcpos_dn8 = assign32520_e40714_d_n8;

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
        *var_guard635_slot = var_guard635;
        *var_guard636_slot = var_guard636;
        *var_guard637_slot = var_guard637;
        *var_guard638_slot = var_guard638;
        *var_guard639_slot = var_guard639;
        *var_guard640_slot = var_guard640;
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

    pub(super) fn stamp_transient_block_66(
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
        var_guard182: f64,
        var_guard199: f64,
        var_guard633: f64,
        var_guard637: f64,
        var_id__blk219: f64,
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
        var_v2: f64,
        var_v3: f64,
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
        var_guard641_slot: &mut f64,
        var_guard642_slot: &mut f64,
        var_guard643_slot: &mut f64,
        var_guard644_slot: &mut f64,
        var_guard645_slot: &mut f64,
        var_guard646_slot: &mut f64,
        var_guard647_slot: &mut f64,
        var_guard648_slot: &mut f64,
        var_guard649_slot: &mut f64,
        var_guard650_slot: &mut f64,
        var_guard651_slot: &mut f64,
        var_guard652_slot: &mut f64,
        var_guard653_slot: &mut f64,
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
        let mut var_guard641: f64 = *var_guard641_slot;
        let mut var_guard642: f64 = *var_guard642_slot;
        let mut var_guard643: f64 = *var_guard643_slot;
        let mut var_guard644: f64 = *var_guard644_slot;
        let mut var_guard645: f64 = *var_guard645_slot;
        let mut var_guard646: f64 = *var_guard646_slot;
        let mut var_guard647: f64 = *var_guard647_slot;
        let mut var_guard648: f64 = *var_guard648_slot;
        let mut var_guard649: f64 = *var_guard649_slot;
        let mut var_guard650: f64 = *var_guard650_slot;
        let mut var_guard651: f64 = *var_guard651_slot;
        let mut var_guard652: f64 = *var_guard652_slot;
        let mut var_guard653: f64 = *var_guard653_slot;
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

        let assign32530_e40717: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard641 = assign32530_e40717;

        let (assign32540_e40731, assign32540_e40731_d_n5, assign32540_e40731_d_n6, assign32540_e40731_d_n7, assign32540_e40731_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) && (var_guard641 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign32540_e40731;
        var_erfctimesexpmtat_dn5 = assign32540_e40731_d_n5;
        var_erfctimesexpmtat_dn6 = assign32540_e40731_d_n6;
        var_erfctimesexpmtat_dn7 = assign32540_e40731_d_n7;
        var_erfctimesexpmtat_dn8 = assign32540_e40731_d_n8;

        let assign32550_e40734: f64 = (-230.25850929940458);
        let assign32550_e40735: f64 = if var_mtat > assign32550_e40734 { 1.0 } else { 0.0 };
        var_guard642 = assign32550_e40735;

        let (assign32560_e40753, assign32560_e40753_d_n5, assign32560_e40753_d_n6, assign32560_e40753_d_n7, assign32560_e40753_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) && (var_guard641 == 0.0)) && (var_guard642 != 0.0)) {
        let assign32560_e40751: f64 = (var_mtat).exp();
        (assign32560_e40751, (assign32560_e40751 * var_mtat_dn5), (assign32560_e40751 * var_mtat_dn6), (assign32560_e40751 * var_mtat_dn7), (assign32560_e40751 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32560_e40753;
        var_tmp_dn5 = assign32560_e40753_d_n5;
        var_tmp_dn6 = assign32560_e40753_d_n6;
        var_tmp_dn7 = assign32560_e40753_d_n7;
        var_tmp_dn8 = assign32560_e40753_d_n8;

        let (assign32570_e40796, assign32570_e40796_d_n5, assign32570_e40796_d_n6, assign32570_e40796_d_n7, assign32570_e40796_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) && (var_guard641 == 0.0)) && (var_guard642 == 0.0)) {
        let assign32570_e40772: f64 = (-230.25850929940458);
        let assign32570_e40774: f64 = (assign32570_e40772 - var_mtat);
        let assign32570_e40778: f64 = (-230.25850929940458);
        let assign32570_e40780: f64 = (assign32570_e40778 - var_mtat);
        let assign32570_e40783: f64 = (-230.25850929940458);
        let assign32570_e40785: f64 = (assign32570_e40783 - var_mtat);
        let assign32570_e40787: f64 = (assign32570_e40785 * 0.3333333333333333);
        let assign32570_e40788: f64 = (1.0 + assign32570_e40787);
        let assign32570_e40789: f64 = (assign32570_e40780 * assign32570_e40788);
        let assign32570_e40790: f64 = (0.5 * assign32570_e40789);
        let assign32570_e40791: f64 = (1.0 + assign32570_e40790);
        let assign32570_e40792: f64 = (assign32570_e40774 * assign32570_e40791);
        let assign32570_e40793: f64 = (1.0 + assign32570_e40792);
        let assign32570_e40794: f64 = (1e-100 / assign32570_e40793);
        (assign32570_e40794, (-((1e-100 * (((-var_mtat_dn5) * assign32570_e40791) + (assign32570_e40774 * (0.5 * (((-var_mtat_dn5) * assign32570_e40788) + (assign32570_e40780 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign32570_e40793 * assign32570_e40793))), (-((1e-100 * (((-var_mtat_dn6) * assign32570_e40791) + (assign32570_e40774 * (0.5 * (((-var_mtat_dn6) * assign32570_e40788) + (assign32570_e40780 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign32570_e40793 * assign32570_e40793))), (-((1e-100 * (((-var_mtat_dn7) * assign32570_e40791) + (assign32570_e40774 * (0.5 * (((-var_mtat_dn7) * assign32570_e40788) + (assign32570_e40780 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign32570_e40793 * assign32570_e40793))), (-((1e-100 * (((-var_mtat_dn8) * assign32570_e40791) + (assign32570_e40774 * (0.5 * (((-var_mtat_dn8) * assign32570_e40788) + (assign32570_e40780 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign32570_e40793 * assign32570_e40793))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32570_e40796;
        var_tmp_dn5 = assign32570_e40796_d_n5;
        var_tmp_dn6 = assign32570_e40796_d_n6;
        var_tmp_dn7 = assign32570_e40796_d_n7;
        var_tmp_dn8 = assign32570_e40796_d_n8;

        let (assign32580_e40815, assign32580_e40815_d_n5, assign32580_e40815_d_n6, assign32580_e40815_d_n7, assign32580_e40815_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) && (var_guard641 == 0.0)) {
        let assign32580_e40811: f64 = (2.0 * var_tmp);
        let assign32580_e40813: f64 = (assign32580_e40811 - var_erfcpos);
        (assign32580_e40813, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign32580_e40815;
        var_erfctimesexpmtat_dn5 = assign32580_e40815_d_n5;
        var_erfctimesexpmtat_dn6 = assign32580_e40815_d_n6;
        var_erfctimesexpmtat_dn7 = assign32580_e40815_d_n7;
        var_erfctimesexpmtat_dn8 = assign32580_e40815_d_n8;

        let (assign32590_e40835, assign32590_e40835_d_n5, assign32590_e40835_d_n6, assign32590_e40835_d_n7, assign32590_e40835_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32590_e40827: f64 = (1.772453850905516 * 0.5);
        let assign32590_e40830: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign32590_e40832: f64 = (assign32590_e40830 / var_ktat);
        let assign32590_e40833: f64 = (assign32590_e40827 * assign32590_e40832);
        (assign32590_e40833, (assign32590_e40827 * ((((var_atatgat_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign32590_e40830 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign32590_e40827 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign32590_e40830 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign32590_e40827 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign32590_e40830 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign32590_e40827 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign32590_e40830 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign32590_e40835;
        var_gammamax_dn5 = assign32590_e40835_d_n5;
        var_gammamax_dn6 = assign32590_e40835_d_n6;
        var_gammamax_dn7 = assign32590_e40835_d_n7;
        var_gammamax_dn8 = assign32590_e40835_d_n8;

        let (assign32600_e40853, assign32600_e40853_d_n5, assign32600_e40853_d_n6, assign32600_e40853_d_n7, assign32600_e40853_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard637 == 0.0)) {
        let assign32600_e40848: f64 = (var_asrh * var_gammamax);
        let assign32600_e40850: f64 = (assign32600_e40848 * var_wtat);
        let assign32600_e40851: f64 = (var_ctatgatd_i * assign32600_e40850);
        (assign32600_e40851, (var_ctatgatd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign32600_e40848 * var_wtat_dn5))), (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign32600_e40848 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign32600_e40848 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign32600_e40848 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign32600_e40853;
        var_itat_dn5 = assign32600_e40853_d_n5;
        var_itat_dn6 = assign32600_e40853_d_n6;
        var_itat_dn7 = assign32600_e40853_d_n7;
        var_itat_dn8 = assign32600_e40853_d_n8;

        let assign32610_e40856: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard643 = assign32610_e40856;

        let (assign32620_e40867, assign32620_e40867_d_n5, assign32620_e40867_d_n6, assign32620_e40867_d_n7, assign32620_e40867_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard643 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign32620_e40867;
        var_ibbt_dn5 = assign32620_e40867_d_n5;
        var_ibbt_dn6 = assign32620_e40867_d_n6;
        var_ibbt_dn7 = assign32620_e40867_d_n7;
        var_ibbt_dn8 = assign32620_e40867_d_n8;

        let assign32630_e40870: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard644 = assign32630_e40870;

        let (assign32640_e40889, assign32640_e40889_d_n5, assign32640_e40889_d_n6, assign32640_e40889_d_n7, assign32640_e40889_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard643 == 0.0)) && (var_guard644 != 0.0)) {
        let assign32640_e40884: f64 = (var_vbirgatd_i - var_vbbt);
        let assign32640_e40886: f64 = (assign32640_e40884 * var_vbirgatinv_d);
        let assign32640_e40887: f64 = (assign32640_e40886).sqrt();
        (assign32640_e40887, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32640_e40889;
        var_tmp_dn5 = assign32640_e40889_d_n5;
        var_tmp_dn6 = assign32640_e40889_d_n6;
        var_tmp_dn7 = assign32640_e40889_d_n7;
        var_tmp_dn8 = assign32640_e40889_d_n8;

        let (assign32650_e40910, assign32650_e40910_d_n5, assign32650_e40910_d_n6, assign32650_e40910_d_n7, assign32650_e40910_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard643 == 0.0)) && (var_guard644 == 0.0)) {
        let assign32650_e40904: f64 = (var_vbirgatd_i - var_vbbt);
        let assign32650_e40906: f64 = (assign32650_e40904 * var_vbirgatinv_d);
        let assign32650_e40908: f64 = (assign32650_e40906).powf(var_pgatd_i);
        (assign32650_e40908, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32650_e40910;
        var_tmp_dn5 = assign32650_e40910_d_n5;
        var_tmp_dn6 = assign32650_e40910_d_n6;
        var_tmp_dn7 = assign32650_e40910_d_n7;
        var_tmp_dn8 = assign32650_e40910_d_n8;

        let (assign32660_e40930, assign32660_e40930_d_n5, assign32660_e40930_d_n6, assign32660_e40930_d_n7, assign32660_e40930_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard643 == 0.0)) {
        let assign32660_e40923: f64 = (var_vbirgatd_i - var_vbbt);
        let assign32660_e40925: f64 = (assign32660_e40923 * var_wdepnulrinvgat_d);
        let assign32660_e40927: f64 = (assign32660_e40925 / var_tmp);
        let assign32660_e40928: f64 = (var_one_over_one_minus_pgat_d * assign32660_e40927);
        (assign32660_e40928, (var_one_over_one_minus_pgat_d * (-((assign32660_e40925 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign32660_e40925 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign32660_e40925 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign32660_e40925 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign32660_e40930;
        var_fmaxr_dn5 = assign32660_e40930_d_n5;
        var_fmaxr_dn6 = assign32660_e40930_d_n6;
        var_fmaxr_dn7 = assign32660_e40930_d_n7;
        var_fmaxr_dn8 = assign32660_e40930_d_n8;

        let assign32670_e40932: f64 = (-var_fbbtgat_d);
        let assign32670_e40934: f64 = (assign32670_e40932 / var_fmaxr);
        let assign32670_e40935: f64 = (assign32670_e40934).abs();
        let assign32670_e40937: f64 = if assign32670_e40935 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard645 = assign32670_e40937;

        let (assign32680_e40955, assign32680_e40955_d_n5, assign32680_e40955_d_n6, assign32680_e40955_d_n7, assign32680_e40955_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard643 == 0.0)) && (var_guard645 != 0.0)) {
        let assign32680_e40950: f64 = (-var_fbbtgat_d);
        let assign32680_e40952: f64 = (assign32680_e40950 / var_fmaxr);
        let assign32680_e40953: f64 = (assign32680_e40952).exp();
        (assign32680_e40953, (assign32680_e40953 * ((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32680_e40950 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign32680_e40953 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32680_e40950 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign32680_e40953 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32680_e40950 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign32680_e40953 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32680_e40950 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32680_e40955;
        var_tmp_dn5 = assign32680_e40955_d_n5;
        var_tmp_dn6 = assign32680_e40955_d_n6;
        var_tmp_dn7 = assign32680_e40955_d_n7;
        var_tmp_dn8 = assign32680_e40955_d_n8;

        let assign32690_e40957: f64 = (-var_fbbtgat_d);
        let assign32690_e40959: f64 = (assign32690_e40957 / var_fmaxr);
        let assign32690_e40961: f64 = if assign32690_e40959 < 0.0 { 1.0 } else { 0.0 };
        var_guard646 = assign32690_e40961;

        let (assign32700_e41012, assign32700_e41012_d_n5, assign32700_e41012_d_n6, assign32700_e41012_d_n7, assign32700_e41012_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard643 == 0.0)) && (var_guard645 == 0.0)) && (var_guard646 != 0.0)) {
        let assign32700_e40979: f64 = (-230.25850929940458);
        let assign32700_e40981: f64 = (-var_fbbtgat_d);
        let assign32700_e40983: f64 = (assign32700_e40981 / var_fmaxr);
        let assign32700_e40984: f64 = (assign32700_e40979 - assign32700_e40983);
        let assign32700_e40988: f64 = (-230.25850929940458);
        let assign32700_e40990: f64 = (-var_fbbtgat_d);
        let assign32700_e40992: f64 = (assign32700_e40990 / var_fmaxr);
        let assign32700_e40993: f64 = (assign32700_e40988 - assign32700_e40992);
        let assign32700_e40996: f64 = (-230.25850929940458);
        let assign32700_e40998: f64 = (-var_fbbtgat_d);
        let assign32700_e41000: f64 = (assign32700_e40998 / var_fmaxr);
        let assign32700_e41001: f64 = (assign32700_e40996 - assign32700_e41000);
        let assign32700_e41003: f64 = (assign32700_e41001 * 0.3333333333333333);
        let assign32700_e41004: f64 = (1.0 + assign32700_e41003);
        let assign32700_e41005: f64 = (assign32700_e40993 * assign32700_e41004);
        let assign32700_e41006: f64 = (0.5 * assign32700_e41005);
        let assign32700_e41007: f64 = (1.0 + assign32700_e41006);
        let assign32700_e41008: f64 = (assign32700_e40984 * assign32700_e41007);
        let assign32700_e41009: f64 = (1.0 + assign32700_e41008);
        let assign32700_e41010: f64 = (1e-100 / assign32700_e41009);
        (assign32700_e41010, (-((1e-100 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32700_e40981 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign32700_e41007) + (assign32700_e40984 * (0.5 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32700_e40990 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign32700_e41004) + (assign32700_e40993 * ((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32700_e40998 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign32700_e41009 * assign32700_e41009))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32700_e40981 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign32700_e41007) + (assign32700_e40984 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32700_e40990 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign32700_e41004) + (assign32700_e40993 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32700_e40998 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign32700_e41009 * assign32700_e41009))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32700_e40981 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign32700_e41007) + (assign32700_e40984 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32700_e40990 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign32700_e41004) + (assign32700_e40993 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32700_e40998 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign32700_e41009 * assign32700_e41009))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32700_e40981 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign32700_e41007) + (assign32700_e40984 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32700_e40990 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign32700_e41004) + (assign32700_e40993 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32700_e40998 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign32700_e41009 * assign32700_e41009))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32700_e41012;
        var_tmp_dn5 = assign32700_e41012_d_n5;
        var_tmp_dn6 = assign32700_e41012_d_n6;
        var_tmp_dn7 = assign32700_e41012_d_n7;
        var_tmp_dn8 = assign32700_e41012_d_n8;

        let (assign32710_e41061, assign32710_e41061_d_n5, assign32710_e41061_d_n6, assign32710_e41061_d_n7, assign32710_e41061_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard643 == 0.0)) && (var_guard645 == 0.0)) && (var_guard646 == 0.0)) {
        let assign32710_e41031: f64 = (-var_fbbtgat_d);
        let assign32710_e41033: f64 = (assign32710_e41031 / var_fmaxr);
        let assign32710_e41035: f64 = (assign32710_e41033 - 230.25850929940458);
        let assign32710_e41039: f64 = (-var_fbbtgat_d);
        let assign32710_e41041: f64 = (assign32710_e41039 / var_fmaxr);
        let assign32710_e41043: f64 = (assign32710_e41041 - 230.25850929940458);
        let assign32710_e41046: f64 = (-var_fbbtgat_d);
        let assign32710_e41048: f64 = (assign32710_e41046 / var_fmaxr);
        let assign32710_e41050: f64 = (assign32710_e41048 - 230.25850929940458);
        let assign32710_e41052: f64 = (assign32710_e41050 * 0.3333333333333333);
        let assign32710_e41053: f64 = (1.0 + assign32710_e41052);
        let assign32710_e41054: f64 = (assign32710_e41043 * assign32710_e41053);
        let assign32710_e41055: f64 = (0.5 * assign32710_e41054);
        let assign32710_e41056: f64 = (1.0 + assign32710_e41055);
        let assign32710_e41057: f64 = (assign32710_e41035 * assign32710_e41056);
        let assign32710_e41058: f64 = (1.0 + assign32710_e41057);
        let assign32710_e41059: f64 = (1e100 * assign32710_e41058);
        (assign32710_e41059, (1e100 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32710_e41031 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign32710_e41056) + (assign32710_e41035 * (0.5 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32710_e41039 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign32710_e41053) + (assign32710_e41043 * (((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign32710_e41046 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32710_e41031 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign32710_e41056) + (assign32710_e41035 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32710_e41039 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign32710_e41053) + (assign32710_e41043 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign32710_e41046 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32710_e41031 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign32710_e41056) + (assign32710_e41035 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32710_e41039 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign32710_e41053) + (assign32710_e41043 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign32710_e41046 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32710_e41031 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign32710_e41056) + (assign32710_e41035 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32710_e41039 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign32710_e41053) + (assign32710_e41043 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign32710_e41046 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32710_e41061;
        var_tmp_dn5 = assign32710_e41061_d_n5;
        var_tmp_dn6 = assign32710_e41061_d_n6;
        var_tmp_dn7 = assign32710_e41061_d_n7;
        var_tmp_dn8 = assign32710_e41061_d_n8;

        let (assign32720_e41081, assign32720_e41081_d_n5, assign32720_e41081_d_n6, assign32720_e41081_d_n7, assign32720_e41081_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard643 == 0.0)) {
        let assign32720_e41074: f64 = (var_v2 * var_fmaxr);
        let assign32720_e41076: f64 = (assign32720_e41074 * var_fmaxr);
        let assign32720_e41078: f64 = (assign32720_e41076 * var_tmp);
        let assign32720_e41079: f64 = (var_cbbtgatd_i * assign32720_e41078);
        (assign32720_e41079, (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign32720_e41074 * var_fmaxr_dn5)) * var_tmp) + (assign32720_e41076 * var_tmp_dn5))), (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign32720_e41074 * var_fmaxr_dn6)) * var_tmp) + (assign32720_e41076 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign32720_e41074 * var_fmaxr_dn7)) * var_tmp) + (assign32720_e41076 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign32720_e41074 * var_fmaxr_dn8)) * var_tmp) + (assign32720_e41076 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign32720_e41081;
        var_ibbt_dn5 = assign32720_e41081_d_n5;
        var_ibbt_dn6 = assign32720_e41081_d_n6;
        var_ibbt_dn7 = assign32720_e41081_d_n7;
        var_ibbt_dn8 = assign32720_e41081_d_n8;

        let assign32730_e41084: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard647 = assign32730_e41084;

        let (assign32740_e41095, assign32740_e41095_d_n5, assign32740_e41095_d_n6, assign32740_e41095_d_n7, assign32740_e41095_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard647 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign32740_e41095;
        var_fbreakdown_dn5 = assign32740_e41095_d_n5;
        var_fbreakdown_dn6 = assign32740_e41095_d_n6;
        var_fbreakdown_dn7 = assign32740_e41095_d_n7;
        var_fbreakdown_dn8 = assign32740_e41095_d_n8;

        let assign32750_e41098: f64 = (-var_alphaav);
        let assign32750_e41100: f64 = (assign32750_e41098 * var_vbrgatd_i);
        let assign32750_e41101: f64 = if var_vav > assign32750_e41100 { 1.0 } else { 0.0 };
        var_guard648 = assign32750_e41101;

        let assign32760_e41104: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard649 = assign32760_e41104;

        let (assign32770_e41134, assign32770_e41134_d_n5, assign32770_e41134_d_n6, assign32770_e41134_d_n7, assign32770_e41134_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard647 == 0.0)) && (var_guard648 != 0.0)) && (var_guard649 != 0.0)) {
        let assign32770_e41120: f64 = (var_vav * var_vbrinvgat_d);
        let assign32770_e41123: f64 = (var_vav * var_vbrinvgat_d);
        let assign32770_e41124: f64 = (assign32770_e41120 * assign32770_e41123);
        let assign32770_e41127: f64 = (var_vav * var_vbrinvgat_d);
        let assign32770_e41128: f64 = (assign32770_e41124 * assign32770_e41127);
        let assign32770_e41131: f64 = (var_vav * var_vbrinvgat_d);
        let assign32770_e41132: f64 = (assign32770_e41128 * assign32770_e41131);
        (assign32770_e41132, (((((((var_vav * var_vbrinvgat_d_dn5) * assign32770_e41123) + (assign32770_e41120 * (var_vav * var_vbrinvgat_d_dn5))) * assign32770_e41127) + (assign32770_e41124 * (var_vav * var_vbrinvgat_d_dn5))) * assign32770_e41131) + (assign32770_e41128 * (var_vav * var_vbrinvgat_d_dn5))), (((((((var_vav * var_vbrinvgat_d_dn6) * assign32770_e41123) + (assign32770_e41120 * (var_vav * var_vbrinvgat_d_dn6))) * assign32770_e41127) + (assign32770_e41124 * (var_vav * var_vbrinvgat_d_dn6))) * assign32770_e41131) + (assign32770_e41128 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign32770_e41123) + (assign32770_e41120 * (var_vav * var_vbrinvgat_d_dn7))) * assign32770_e41127) + (assign32770_e41124 * (var_vav * var_vbrinvgat_d_dn7))) * assign32770_e41131) + (assign32770_e41128 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign32770_e41123) + (assign32770_e41120 * (var_vav * var_vbrinvgat_d_dn8))) * assign32770_e41127) + (assign32770_e41124 * (var_vav * var_vbrinvgat_d_dn8))) * assign32770_e41131) + (assign32770_e41128 * (var_vav * var_vbrinvgat_d_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32770_e41134;
        var_tmp_dn5 = assign32770_e41134_d_n5;
        var_tmp_dn6 = assign32770_e41134_d_n6;
        var_tmp_dn7 = assign32770_e41134_d_n7;
        var_tmp_dn8 = assign32770_e41134_d_n8;

        let (assign32780_e41156, assign32780_e41156_d_n5, assign32780_e41156_d_n6, assign32780_e41156_d_n7, assign32780_e41156_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard647 == 0.0)) && (var_guard648 != 0.0)) && (var_guard649 == 0.0)) {
        let assign32780_e41151: f64 = (var_vav * var_vbrinvgat_d);
        let assign32780_e41152: f64 = (assign32780_e41151).abs();
        let assign32780_e41154: f64 = (assign32780_e41152).powf(var_pbrgatd_i);
        (assign32780_e41154, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign32780_e41152).powf(var_pbrgatd_i - 1.0) * if assign32780_e41151 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) })) } } else { (assign32780_e41154 * (var_pbrgatd_i * (if assign32780_e41151 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) } / assign32780_e41152))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign32780_e41152).powf(var_pbrgatd_i - 1.0) * if assign32780_e41151 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign32780_e41154 * (var_pbrgatd_i * (if assign32780_e41151 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign32780_e41152))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign32780_e41152).powf(var_pbrgatd_i - 1.0) * if assign32780_e41151 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign32780_e41154 * (var_pbrgatd_i * (if assign32780_e41151 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign32780_e41152))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign32780_e41152).powf(var_pbrgatd_i - 1.0) * if assign32780_e41151 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign32780_e41154 * (var_pbrgatd_i * (if assign32780_e41151 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign32780_e41152))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign32780_e41156;
        var_tmp_dn5 = assign32780_e41156_d_n5;
        var_tmp_dn6 = assign32780_e41156_d_n6;
        var_tmp_dn7 = assign32780_e41156_d_n7;
        var_tmp_dn8 = assign32780_e41156_d_n8;

        let (assign32790_e41174, assign32790_e41174_d_n5, assign32790_e41174_d_n6, assign32790_e41174_d_n7, assign32790_e41174_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard647 == 0.0)) && (var_guard648 != 0.0)) {
        let assign32790_e41171: f64 = (1.0 - var_tmp);
        let assign32790_e41172: f64 = (1.0 / assign32790_e41171);
        (assign32790_e41172, (-((-var_tmp_dn5) / (assign32790_e41171 * assign32790_e41171))), (-((-var_tmp_dn6) / (assign32790_e41171 * assign32790_e41171))), (-((-var_tmp_dn7) / (assign32790_e41171 * assign32790_e41171))), (-((-var_tmp_dn8) / (assign32790_e41171 * assign32790_e41171))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign32790_e41174;
        var_fbreakdown_dn5 = assign32790_e41174_d_n5;
        var_fbreakdown_dn6 = assign32790_e41174_d_n6;
        var_fbreakdown_dn7 = assign32790_e41174_d_n7;
        var_fbreakdown_dn8 = assign32790_e41174_d_n8;

        let (assign32800_e41197, assign32800_e41197_d_n5, assign32800_e41197_d_n6, assign32800_e41197_d_n7, assign32800_e41197_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) && (var_guard647 == 0.0)) && (var_guard648 == 0.0)) {
        let assign32800_e41191: f64 = (var_alphaav * var_vbrgatd_i);
        let assign32800_e41192: f64 = (var_vav + assign32800_e41191);
        let assign32800_e41194: f64 = (assign32800_e41192 * var_slopegat_d);
        let assign32800_e41195: f64 = (var_fstopgat_d + assign32800_e41194);
        (assign32800_e41195, (assign32800_e41192 * var_slopegat_d_dn5), (assign32800_e41192 * var_slopegat_d_dn6), (assign32800_e41192 * var_slopegat_d_dn7), (assign32800_e41192 * var_slopegat_d_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign32800_e41197;
        var_fbreakdown_dn5 = assign32800_e41197_d_n5;
        var_fbreakdown_dn6 = assign32800_e41197_d_n6;
        var_fbreakdown_dn7 = assign32800_e41197_d_n7;
        var_fbreakdown_dn8 = assign32800_e41197_d_n8;

        let (assign32810_e41216, assign32810_e41216_d_n5, assign32810_e41216_d_n6, assign32810_e41216_d_n7, assign32810_e41216_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard633 == 0.0)) {
        let assign32810_e41207: f64 = (var_id__blk219 + var_isrh);
        let assign32810_e41209: f64 = (assign32810_e41207 + var_itat);
        let assign32810_e41211: f64 = (assign32810_e41209 + var_ibbt);
        let assign32810_e41212: f64 = (p.p29 * assign32810_e41211);
        let assign32810_e41214: f64 = (assign32810_e41212 * var_fbreakdown);
        (assign32810_e41214, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign32810_e41212 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign32810_e41212 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign32810_e41212 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign32810_e41212 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign32810_e41216;
        var_ijungat_dn5 = assign32810_e41216_d_n5;
        var_ijungat_dn6 = assign32810_e41216_d_n6;
        var_ijungat_dn7 = assign32810_e41216_d_n7;
        var_ijungat_dn8 = assign32810_e41216_d_n8;

        let (assign32820_e41232, assign32820_e41232_d_n5, assign32820_e41232_d_n6, assign32820_e41232_d_n7, assign32820_e41232_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign32820_e41222: f64 = (var_abdrain_i * var_ijunbot);
        let assign32820_e41225: f64 = (var_lsdrain_i * var_ijunsti);
        let assign32820_e41226: f64 = (assign32820_e41222 + assign32820_e41225);
        let assign32820_e41229: f64 = (var_lgdrain_i * var_ijungat);
        let assign32820_e41230: f64 = (assign32820_e41226 + assign32820_e41229);
        (assign32820_e41230, (((var_abdrain_i * var_ijunbot_dn5) + (var_lsdrain_i * var_ijunsti_dn5)) + (var_lgdrain_i * var_ijungat_dn5)), (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)),)
    } else {
        (var_i2, var_i2_dn5, var_i2_dn6, var_i2_dn7, var_i2_dn8,)
    }
};
        var_i2 = assign32820_e41232;
        var_i2_dn5 = assign32820_e41232_d_n5;
        var_i2_dn6 = assign32820_e41232_d_n6;
        var_i2_dn7 = assign32820_e41232_d_n7;
        var_i2_dn8 = assign32820_e41232_d_n8;

        let (assign32830_e41238,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign32830_e41238;

        let (assign32840_e41244,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign32840_e41244;

        let assign32850_e41256: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard650 = assign32850_e41256;

        let assign32930_e41342: f64 = if var_v3 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard651 = assign32930_e41342;

        let assign32940_e41344: f64 = (-0.5);
        let assign32940_e41347: f64 = (var_v3 * var_phitdinv);
        let assign32940_e41348: f64 = (assign32940_e41344 * assign32940_e41347);
        let assign32940_e41349: f64 = (assign32940_e41348).abs();
        let assign32940_e41351: f64 = if assign32940_e41349 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard652 = assign32940_e41351;

        let (assign32950_e41369,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) && (var_guard651 != 0.0)) && (var_guard652 != 0.0)) {
        let assign32950_e41362: f64 = (-0.5);
        let assign32950_e41365: f64 = (var_v3 * var_phitdinv);
        let assign32950_e41366: f64 = (assign32950_e41362 * assign32950_e41365);
        let assign32950_e41367: f64 = (assign32950_e41366).exp();
        (assign32950_e41367,)
    } else {
        (var_z,)
    }
};
        var_z = assign32950_e41369;

        let assign32960_e41371: f64 = (-0.5);
        let assign32960_e41374: f64 = (var_v3 * var_phitdinv);
        let assign32960_e41375: f64 = (assign32960_e41371 * assign32960_e41374);
        let assign32960_e41377: f64 = if assign32960_e41375 < 0.0 { 1.0 } else { 0.0 };
        var_guard653 = assign32960_e41377;

        let (assign32970_e41432,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) && (var_guard651 != 0.0)) && (var_guard652 == 0.0)) && (var_guard653 != 0.0)) {
        let assign32970_e41393: f64 = (-230.25850929940458);
        let assign32970_e41395: f64 = (-0.5);
        let assign32970_e41398: f64 = (var_v3 * var_phitdinv);
        let assign32970_e41399: f64 = (assign32970_e41395 * assign32970_e41398);
        let assign32970_e41400: f64 = (assign32970_e41393 - assign32970_e41399);
        let assign32970_e41404: f64 = (-230.25850929940458);
        let assign32970_e41406: f64 = (-0.5);
        let assign32970_e41409: f64 = (var_v3 * var_phitdinv);
        let assign32970_e41410: f64 = (assign32970_e41406 * assign32970_e41409);
        let assign32970_e41411: f64 = (assign32970_e41404 - assign32970_e41410);
        let assign32970_e41414: f64 = (-230.25850929940458);
        let assign32970_e41416: f64 = (-0.5);
        let assign32970_e41419: f64 = (var_v3 * var_phitdinv);
        let assign32970_e41420: f64 = (assign32970_e41416 * assign32970_e41419);
        let assign32970_e41421: f64 = (assign32970_e41414 - assign32970_e41420);
        let assign32970_e41423: f64 = (assign32970_e41421 * 0.3333333333333333);
        let assign32970_e41424: f64 = (1.0 + assign32970_e41423);
        let assign32970_e41425: f64 = (assign32970_e41411 * assign32970_e41424);
        let assign32970_e41426: f64 = (0.5 * assign32970_e41425);
        let assign32970_e41427: f64 = (1.0 + assign32970_e41426);
        let assign32970_e41428: f64 = (assign32970_e41400 * assign32970_e41427);
        let assign32970_e41429: f64 = (1.0 + assign32970_e41428);
        let assign32970_e41430: f64 = (1e-100 / assign32970_e41429);
        (assign32970_e41430,)
    } else {
        (var_z,)
    }
};
        var_z = assign32970_e41432;

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
        *var_guard641_slot = var_guard641;
        *var_guard642_slot = var_guard642;
        *var_guard643_slot = var_guard643;
        *var_guard644_slot = var_guard644;
        *var_guard645_slot = var_guard645;
        *var_guard646_slot = var_guard646;
        *var_guard647_slot = var_guard647;
        *var_guard648_slot = var_guard648;
        *var_guard649_slot = var_guard649;
        *var_guard650_slot = var_guard650;
        *var_guard651_slot = var_guard651;
        *var_guard652_slot = var_guard652;
        *var_guard653_slot = var_guard653;
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

    pub(super) fn stamp_transient_block_67(
        var_abdrain_i: f64,
        var_atatbot_d: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_ftdbot_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard650: f64,
        var_guard651: f64,
        var_guard652: f64,
        var_guard653: f64,
        var_idsatbot_d: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_v3: f64,
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
        var_guard654_slot: &mut f64,
        var_guard655_slot: &mut f64,
        var_guard656_slot: &mut f64,
        var_guard657_slot: &mut f64,
        var_guard658_slot: &mut f64,
        var_guard659_slot: &mut f64,
        var_guard660_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard654: f64 = *var_guard654_slot;
        let mut var_guard655: f64 = *var_guard655_slot;
        let mut var_guard656: f64 = *var_guard656_slot;
        let mut var_guard657: f64 = *var_guard657_slot;
        let mut var_guard658: f64 = *var_guard658_slot;
        let mut var_guard659: f64 = *var_guard659_slot;
        let mut var_guard660: f64 = *var_guard660_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let (assign32980_e41485,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) && (var_guard651 != 0.0)) && (var_guard652 == 0.0)) && (var_guard653 == 0.0)) {
        let assign32980_e41449: f64 = (-0.5);
        let assign32980_e41452: f64 = (var_v3 * var_phitdinv);
        let assign32980_e41453: f64 = (assign32980_e41449 * assign32980_e41452);
        let assign32980_e41455: f64 = (assign32980_e41453 - 230.25850929940458);
        let assign32980_e41459: f64 = (-0.5);
        let assign32980_e41462: f64 = (var_v3 * var_phitdinv);
        let assign32980_e41463: f64 = (assign32980_e41459 * assign32980_e41462);
        let assign32980_e41465: f64 = (assign32980_e41463 - 230.25850929940458);
        let assign32980_e41468: f64 = (-0.5);
        let assign32980_e41471: f64 = (var_v3 * var_phitdinv);
        let assign32980_e41472: f64 = (assign32980_e41468 * assign32980_e41471);
        let assign32980_e41474: f64 = (assign32980_e41472 - 230.25850929940458);
        let assign32980_e41476: f64 = (assign32980_e41474 * 0.3333333333333333);
        let assign32980_e41477: f64 = (1.0 + assign32980_e41476);
        let assign32980_e41478: f64 = (assign32980_e41465 * assign32980_e41477);
        let assign32980_e41479: f64 = (0.5 * assign32980_e41478);
        let assign32980_e41480: f64 = (1.0 + assign32980_e41479);
        let assign32980_e41481: f64 = (assign32980_e41455 * assign32980_e41480);
        let assign32980_e41482: f64 = (1.0 + assign32980_e41481);
        let assign32980_e41483: f64 = (1e100 * assign32980_e41482);
        (assign32980_e41483,)
    } else {
        (var_z,)
    }
};
        var_z = assign32980_e41485;

        let (assign32990_e41497,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) && (var_guard651 != 0.0)) {
        let assign32990_e41495: f64 = (1.0 / var_z);
        (assign32990_e41495,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign32990_e41497;

        let (assign33000_e41509,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) && (var_guard651 != 0.0)) {
        let assign33000_e41507: f64 = (var_zinv * var_zinv);
        (assign33000_e41507,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign33000_e41509;

        let (assign33010_e41528,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) && (var_guard651 == 0.0)) {
        let assign33010_e41521: f64 = (var_v3 - var_vmax_d);
        let assign33010_e41523: f64 = (assign33010_e41521 * var_phitdinv);
        let assign33010_e41524: f64 = (1.0 + assign33010_e41523);
        let assign33010_e41526: f64 = (assign33010_e41524 * var_exp_vmax_over_phitd_d);
        (assign33010_e41526,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign33010_e41528;

        let (assign33020_e41540,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) && (var_guard651 == 0.0)) {
        let assign33020_e41538: f64 = (var_idmult).sqrt();
        (assign33020_e41538,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign33020_e41540;

        let (assign33030_e41553,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) && (var_guard651 == 0.0)) {
        let assign33030_e41551: f64 = (1.0 / var_zinv);
        (assign33030_e41551,)
    } else {
        (var_z,)
    }
};
        var_z = assign33030_e41553;

        let (assign33040_e41563,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) {
        let assign33040_e41561: f64 = (var_idmult - 1.0);
        (assign33040_e41561,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign33040_e41563;

        let assign33050_e41566: f64 = if var_v3 > 0.0 { 1.0 } else { 0.0 };
        var_guard654 = assign33050_e41566;

        let (assign33060_e41592,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) && (var_guard654 != 0.0)) {
        let assign33060_e41578: f64 = (2.0 + var_z);
        let assign33060_e41581: f64 = (var_z + 1.0);
        let assign33060_e41584: f64 = (var_z + 3.0);
        let assign33060_e41585: f64 = (assign33060_e41581 * assign33060_e41584);
        let assign33060_e41586: f64 = (assign33060_e41585).sqrt();
        let assign33060_e41587: f64 = (assign33060_e41578 + assign33060_e41586);
        let assign33060_e41588: f64 = (assign33060_e41587).ln();
        let assign33060_e41589: f64 = (var_phitd * assign33060_e41588);
        let assign33060_e41590: f64 = (2.0 * assign33060_e41589);
        (assign33060_e41590,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign33060_e41592;

        let (assign33070_e41626,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) && (var_guard654 == 0.0)) {
        let assign33070_e41602: f64 = (-var_v3);
        let assign33070_e41607: f64 = (2.0 * var_zinv);
        let assign33070_e41609: f64 = (assign33070_e41607 + 1.0);
        let assign33070_e41612: f64 = (1.0 + var_zinv);
        let assign33070_e41616: f64 = (3.0 * var_zinv);
        let assign33070_e41617: f64 = (1.0 + assign33070_e41616);
        let assign33070_e41618: f64 = (assign33070_e41612 * assign33070_e41617);
        let assign33070_e41619: f64 = (assign33070_e41618).sqrt();
        let assign33070_e41620: f64 = (assign33070_e41609 + assign33070_e41619);
        let assign33070_e41621: f64 = (assign33070_e41620).ln();
        let assign33070_e41622: f64 = (var_phitd * assign33070_e41621);
        let assign33070_e41623: f64 = (2.0 * assign33070_e41622);
        let assign33070_e41624: f64 = (assign33070_e41602 + assign33070_e41623);
        (assign33070_e41624,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign33070_e41626;

        let (assign33080_e41636,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) {
        let assign33080_e41634: f64 = (var_vbimin_d - var_two_psistar);
        (assign33080_e41634,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign33080_e41636;

        let (assign33090_e41663,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) {
        let assign33090_e41645: f64 = (var_v3 + var_vjlim);
        let assign33090_e41648: f64 = (var_v3 - var_vjlim);
        let assign33090_e41651: f64 = (var_v3 - var_vjlim);
        let assign33090_e41652: f64 = (assign33090_e41648 * assign33090_e41651);
        let assign33090_e41655: f64 = (4.0 * var_phitd);
        let assign33090_e41657: f64 = (assign33090_e41655 * var_phitd);
        let assign33090_e41658: f64 = (assign33090_e41652 + assign33090_e41657);
        let assign33090_e41659: f64 = (assign33090_e41658).sqrt();
        let assign33090_e41660: f64 = (assign33090_e41645 - assign33090_e41659);
        let assign33090_e41661: f64 = (0.5 * assign33090_e41660);
        (assign33090_e41661,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign33090_e41663;

        let (assign33100_e41690,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) {
        let assign33100_e41672: f64 = (var_v3 + var_vbbtlim_d);
        let assign33100_e41675: f64 = (var_v3 - var_vbbtlim_d);
        let assign33100_e41678: f64 = (var_v3 - var_vbbtlim_d);
        let assign33100_e41679: f64 = (assign33100_e41675 * assign33100_e41678);
        let assign33100_e41682: f64 = (4.0 * var_phitr);
        let assign33100_e41684: f64 = (assign33100_e41682 * var_phitr);
        let assign33100_e41685: f64 = (assign33100_e41679 + assign33100_e41684);
        let assign33100_e41686: f64 = (assign33100_e41685).sqrt();
        let assign33100_e41687: f64 = (assign33100_e41672 - assign33100_e41686);
        let assign33100_e41688: f64 = (0.5 * assign33100_e41687);
        (assign33100_e41688,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign33100_e41690;

        let (assign33110_e41717,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard650 != 0.0)) {
        let assign33110_e41699: f64 = var_v3;
        let assign33110_e41702: f64 = var_v3;
        let assign33110_e41705: f64 = var_v3;
        let assign33110_e41706: f64 = (assign33110_e41702 * assign33110_e41705);
        let assign33110_e41709: f64 = (4.0 * 1e-6);
        let assign33110_e41711: f64 = (assign33110_e41709 * 1e-6);
        let assign33110_e41712: f64 = (assign33110_e41706 + assign33110_e41711);
        let assign33110_e41713: f64 = (assign33110_e41712).sqrt();
        let assign33110_e41714: f64 = (assign33110_e41699 - assign33110_e41713);
        let assign33110_e41715: f64 = (0.5 * assign33110_e41714);
        (assign33110_e41715,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign33110_e41717;

        let assign33120_e41720: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard655 = assign33120_e41720;

        let (assign33130_e41728, assign33130_e41728_d_n5, assign33130_e41728_d_n6, assign33130_e41728_d_n7, assign33130_e41728_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign33130_e41728;
        var_ijunbot_dn5 = assign33130_e41728_d_n5;
        var_ijunbot_dn6 = assign33130_e41728_d_n6;
        var_ijunbot_dn7 = assign33130_e41728_d_n7;
        var_ijunbot_dn8 = assign33130_e41728_d_n8;

        let (assign33140_e41739,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) {
        let assign33140_e41737: f64 = (var_idsatbot_d * var_idmult);
        (assign33140_e41737,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign33140_e41739;

        let assign33150_e41746: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard656 = assign33150_e41746;

        let (assign33160_e41757, assign33160_e41757_d_n5, assign33160_e41757_d_n6, assign33160_e41757_d_n7, assign33160_e41757_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign33160_e41757;
        var_isrh_dn5 = assign33160_e41757_d_n5;
        var_isrh_dn6 = assign33160_e41757_d_n6;
        var_isrh_dn7 = assign33160_e41757_d_n7;
        var_isrh_dn8 = assign33160_e41757_d_n8;

        let (assign33170_e41771,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 == 0.0)) {
        let assign33170_e41769: f64 = (var_vbibot_d - var_vjsrh);
        (assign33170_e41769,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign33170_e41771;

        let (assign33180_e41790,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 == 0.0)) {
        let assign33180_e41785: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign33180_e41786: f64 = (1.0 - assign33180_e41785);
        let assign33180_e41787: f64 = (assign33180_e41786).sqrt();
        let assign33180_e41788: f64 = (1.0 - assign33180_e41787);
        (assign33180_e41788,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign33180_e41790;

        let assign33190_e41793: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard657 = assign33190_e41793;

        let (assign33200_e41807,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 == 0.0)) && (var_guard657 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign33200_e41807;

        let (assign33210_e41839,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 == 0.0)) && (var_guard657 == 0.0)) {
        let assign33210_e41822: f64 = (var_wsrhstep * var_wsrhstep);
        let assign33210_e41824: f64 = (var_wsrhstep).ln();
        let assign33210_e41825: f64 = (assign33210_e41822 * assign33210_e41824);
        let assign33210_e41828: f64 = (1.0 - var_wsrhstep);
        let assign33210_e41829: f64 = (assign33210_e41825 / assign33210_e41828);
        let assign33210_e41831: f64 = (assign33210_e41829 + var_wsrhstep);
        let assign33210_e41835: f64 = (2.0 * var_pbotd_i);
        let assign33210_e41836: f64 = (1.0 - assign33210_e41835);
        let assign33210_e41837: f64 = (assign33210_e41831 * assign33210_e41836);
        (assign33210_e41837,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign33210_e41839;

        let (assign33220_e41853,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 == 0.0)) {
        let assign33220_e41851: f64 = (var_wsrhstep + var_dwsrh);
        (assign33220_e41851,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign33220_e41853;

        let assign33230_e41856: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard658 = assign33230_e41856;

        let (assign33240_e41873, assign33240_e41873_d_n5, assign33240_e41873_d_n6, assign33240_e41873_d_n7, assign33240_e41873_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 == 0.0)) && (var_guard658 != 0.0)) {
        let assign33240_e41870: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign33240_e41871: f64 = (assign33240_e41870).sqrt();
        (assign33240_e41871, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33240_e41873;
        var_tmp_dn5 = assign33240_e41873_d_n5;
        var_tmp_dn6 = assign33240_e41873_d_n6;
        var_tmp_dn7 = assign33240_e41873_d_n7;
        var_tmp_dn8 = assign33240_e41873_d_n8;

        let (assign33250_e41892, assign33250_e41892_d_n5, assign33250_e41892_d_n6, assign33250_e41892_d_n7, assign33250_e41892_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 == 0.0)) && (var_guard658 == 0.0)) {
        let assign33250_e41888: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign33250_e41890: f64 = (assign33250_e41888).powf(var_pbotd_i);
        (assign33250_e41890, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33250_e41892;
        var_tmp_dn5 = assign33250_e41892_d_n5;
        var_tmp_dn6 = assign33250_e41892_d_n6;
        var_tmp_dn7 = assign33250_e41892_d_n7;
        var_tmp_dn8 = assign33250_e41892_d_n8;

        let (assign33260_e41906, assign33260_e41906_d_n5, assign33260_e41906_d_n6, assign33260_e41906_d_n7, assign33260_e41906_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 == 0.0)) {
        let assign33260_e41904: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign33260_e41904, (var_wdepnulrbot_d * var_tmp_dn5), (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign33260_e41906;
        var_wdep_dn5 = assign33260_e41906_d_n5;
        var_wdep_dn6 = assign33260_e41906_d_n6;
        var_wdep_dn7 = assign33260_e41906_d_n7;
        var_wdep_dn8 = assign33260_e41906_d_n8;

        let (assign33270_e41924, assign33270_e41924_d_n5, assign33270_e41924_d_n6, assign33270_e41924_d_n7, assign33270_e41924_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 == 0.0)) {
        let assign33270_e41919: f64 = (var_zinv - 1.0);
        let assign33270_e41921: f64 = (assign33270_e41919 * var_wdep);
        let assign33270_e41922: f64 = (var_ftdbot_d * assign33270_e41921);
        (assign33270_e41922, (var_ftdbot_d * (assign33270_e41919 * var_wdep_dn5)), (var_ftdbot_d * (assign33270_e41919 * var_wdep_dn6)), (var_ftdbot_d * (assign33270_e41919 * var_wdep_dn7)), (var_ftdbot_d * (assign33270_e41919 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign33270_e41924;
        var_asrh_dn5 = assign33270_e41924_d_n5;
        var_asrh_dn6 = assign33270_e41924_d_n6;
        var_asrh_dn7 = assign33270_e41924_d_n7;
        var_asrh_dn8 = assign33270_e41924_d_n8;

        let (assign33280_e41940, assign33280_e41940_d_n5, assign33280_e41940_d_n6, assign33280_e41940_d_n7, assign33280_e41940_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard656 == 0.0)) {
        let assign33280_e41937: f64 = (var_asrh * var_wsrh);
        let assign33280_e41938: f64 = (var_csrhbotd_i * assign33280_e41937);
        (assign33280_e41938, (var_csrhbotd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign33280_e41940;
        var_isrh_dn5 = assign33280_e41940_d_n5;
        var_isrh_dn6 = assign33280_e41940_d_n6;
        var_isrh_dn7 = assign33280_e41940_d_n7;
        var_isrh_dn8 = assign33280_e41940_d_n8;

        let assign33290_e41943: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard659 = assign33290_e41943;

        let (assign33300_e41954, assign33300_e41954_d_n5, assign33300_e41954_d_n6, assign33300_e41954_d_n7, assign33300_e41954_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign33300_e41954;
        var_itat_dn5 = assign33300_e41954_d_n5;
        var_itat_dn6 = assign33300_e41954_d_n6;
        var_itat_dn7 = assign33300_e41954_d_n7;
        var_itat_dn8 = assign33300_e41954_d_n8;

        let (assign33310_e41972, assign33310_e41972_d_n5, assign33310_e41972_d_n6, assign33310_e41972_d_n7, assign33310_e41972_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33310_e41967: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign33310_e41969: f64 = (assign33310_e41967 / var_vbi_minus_vjsrh);
        let assign33310_e41970: f64 = (var_btatpartbot_d * assign33310_e41969);
        (assign33310_e41970, (var_btatpartbot_d * ((var_wdep_dn5 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign33310_e41972;
        var_btat_dn5 = assign33310_e41972_d_n5;
        var_btat_dn6 = assign33310_e41972_d_n6;
        var_btat_dn7 = assign33310_e41972_d_n7;
        var_btat_dn8 = assign33310_e41972_d_n8;

        let (assign33320_e41988, assign33320_e41988_d_n5, assign33320_e41988_d_n6, assign33320_e41988_d_n7, assign33320_e41988_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33320_e41984: f64 = (0.666666666666667 * var_atatbot_d);
        let assign33320_e41986: f64 = (assign33320_e41984 / var_btat);
        (assign33320_e41986, (-((assign33320_e41984 * var_btat_dn5) / (var_btat * var_btat))), (-((assign33320_e41984 * var_btat_dn6) / (var_btat * var_btat))), (-((assign33320_e41984 * var_btat_dn7) / (var_btat * var_btat))), (-((assign33320_e41984 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign33320_e41988;
        var_twoatatoverthreebtat_dn5 = assign33320_e41988_d_n5;
        var_twoatatoverthreebtat_dn6 = assign33320_e41988_d_n6;
        var_twoatatoverthreebtat_dn7 = assign33320_e41988_d_n7;
        var_twoatatoverthreebtat_dn8 = assign33320_e41988_d_n8;

        let (assign33330_e42002, assign33330_e42002_d_n5, assign33330_e42002_d_n6, assign33330_e42002_d_n7, assign33330_e42002_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33330_e42000: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign33330_e42000, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign33330_e42002;
        var_umaxbeforelimiting_dn5 = assign33330_e42002_d_n5;
        var_umaxbeforelimiting_dn6 = assign33330_e42002_d_n6;
        var_umaxbeforelimiting_dn7 = assign33330_e42002_d_n7;
        var_umaxbeforelimiting_dn8 = assign33330_e42002_d_n8;

        let (assign33340_e42023, assign33340_e42023_d_n5, assign33340_e42023_d_n6, assign33340_e42023_d_n7, assign33340_e42023_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33340_e42014: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign33340_e42017: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign33340_e42019: f64 = (assign33340_e42017 + 1.0);
        let assign33340_e42020: f64 = (assign33340_e42014 / assign33340_e42019);
        let assign33340_e42021: f64 = (assign33340_e42020).sqrt();
        (assign33340_e42021, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign33340_e42019) - (assign33340_e42014 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign33340_e42019 * assign33340_e42019)) / (2.0 * assign33340_e42021)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign33340_e42019) - (assign33340_e42014 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign33340_e42019 * assign33340_e42019)) / (2.0 * assign33340_e42021)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign33340_e42019) - (assign33340_e42014 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign33340_e42019 * assign33340_e42019)) / (2.0 * assign33340_e42021)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign33340_e42019) - (assign33340_e42014 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign33340_e42019 * assign33340_e42019)) / (2.0 * assign33340_e42021)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign33340_e42023;
        var_umax_dn5 = assign33340_e42023_d_n5;
        var_umax_dn6 = assign33340_e42023_d_n6;
        var_umax_dn7 = assign33340_e42023_d_n7;
        var_umax_dn8 = assign33340_e42023_d_n8;

        let (assign33350_e42036, assign33350_e42036_d_n5, assign33350_e42036_d_n6, assign33350_e42036_d_n7, assign33350_e42036_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33350_e42034: f64 = (var_umax).sqrt();
        (assign33350_e42034, (var_umax_dn5 / (2.0 * assign33350_e42034)), (var_umax_dn6 / (2.0 * assign33350_e42034)), (var_umax_dn7 / (2.0 * assign33350_e42034)), (var_umax_dn8 / (2.0 * assign33350_e42034)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign33350_e42036;
        var_sqrtumax_dn5 = assign33350_e42036_d_n5;
        var_sqrtumax_dn6 = assign33350_e42036_d_n6;
        var_sqrtumax_dn7 = assign33350_e42036_d_n7;
        var_sqrtumax_dn8 = assign33350_e42036_d_n8;

        let (assign33360_e42050, assign33360_e42050_d_n5, assign33360_e42050_d_n6, assign33360_e42050_d_n7, assign33360_e42050_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33360_e42048: f64 = (var_umax * var_sqrtumax);
        (assign33360_e42048, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign33360_e42050;
        var_umaxpoweronepointfive_dn5 = assign33360_e42050_d_n5;
        var_umaxpoweronepointfive_dn6 = assign33360_e42050_d_n6;
        var_umaxpoweronepointfive_dn7 = assign33360_e42050_d_n7;
        var_umaxpoweronepointfive_dn8 = assign33360_e42050_d_n8;

        let assign33370_e42052: f64 = (-var_pbotd_i);
        let assign33370_e42054: f64 = (assign33370_e42052 * var_one_over_one_minus_pbot_d);
        let assign33370_e42056: f64 = (-1.0);
        let assign33370_e42057: f64 = if assign33370_e42054 == assign33370_e42056 { 1.0 } else { 0.0 };
        var_guard660 = assign33370_e42057;

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
        *var_guard654_slot = var_guard654;
        *var_guard655_slot = var_guard655;
        *var_guard656_slot = var_guard656;
        *var_guard657_slot = var_guard657;
        *var_guard658_slot = var_guard658;
        *var_guard659_slot = var_guard659;
        *var_guard660_slot = var_guard660;
        *var_id__blk219_slot = var_id__blk219;
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

    pub(super) fn stamp_transient_block_68(
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
        var_guard182: f64,
        var_guard199: f64,
        var_guard655: f64,
        var_guard659: f64,
        var_guard660: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
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
        var_vbbt: f64,
        var_vbirbotd_i: f64,
        var_vbirbotinv_d: f64,
        var_wdepnulrinvbot_d: f64,
        var_wsrh: f64,
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
        var_guard661_slot: &mut f64,
        var_guard662_slot: &mut f64,
        var_guard663_slot: &mut f64,
        var_guard664_slot: &mut f64,
        var_guard665_slot: &mut f64,
        var_guard666_slot: &mut f64,
        var_guard667_slot: &mut f64,
        var_guard668_slot: &mut f64,
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
        let mut var_guard661: f64 = *var_guard661_slot;
        let mut var_guard662: f64 = *var_guard662_slot;
        let mut var_guard663: f64 = *var_guard663_slot;
        let mut var_guard664: f64 = *var_guard664_slot;
        let mut var_guard665: f64 = *var_guard665_slot;
        let mut var_guard666: f64 = *var_guard666_slot;
        let mut var_guard667: f64 = *var_guard667_slot;
        let mut var_guard668: f64 = *var_guard668_slot;
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

        let (assign33380_e42077, assign33380_e42077_d_n5, assign33380_e42077_d_n6, assign33380_e42077_d_n7, assign33380_e42077_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) && (var_guard660 != 0.0)) {
        let assign33380_e42073: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33380_e42074: f64 = (1.0 + assign33380_e42073);
        let assign33380_e42075: f64 = (1.0 / assign33380_e42074);
        (assign33380_e42075, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign33380_e42074 * assign33380_e42074))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign33380_e42074 * assign33380_e42074))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign33380_e42074 * assign33380_e42074))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign33380_e42074 * assign33380_e42074))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign33380_e42077;
        var_wgamma_dn5 = assign33380_e42077_d_n5;
        var_wgamma_dn6 = assign33380_e42077_d_n6;
        var_wgamma_dn7 = assign33380_e42077_d_n7;
        var_wgamma_dn8 = assign33380_e42077_d_n8;

        let (assign33390_e42101, assign33390_e42101_d_n5, assign33390_e42101_d_n6, assign33390_e42101_d_n7, assign33390_e42101_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) && (var_guard660 == 0.0)) {
        let assign33390_e42093: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33390_e42094: f64 = (1.0 + assign33390_e42093);
        let assign33390_e42096: f64 = (-var_pbotd_i);
        let assign33390_e42098: f64 = (assign33390_e42096 * var_one_over_one_minus_pbot_d);
        let assign33390_e42099: f64 = (assign33390_e42094).powf(assign33390_e42098);
        (assign33390_e42099, if 0.0 == 0.0 && ((assign33390_e42098) as f64).is_finite() && ((assign33390_e42098) as f64).fract() == 0.0 { if assign33390_e42098 == 0.0 { 0.0 } else { (assign33390_e42098 * ((assign33390_e42094).powf(assign33390_e42098 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign33390_e42099 * (assign33390_e42098 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign33390_e42094))) }, if 0.0 == 0.0 && ((assign33390_e42098) as f64).is_finite() && ((assign33390_e42098) as f64).fract() == 0.0 { if assign33390_e42098 == 0.0 { 0.0 } else { (assign33390_e42098 * ((assign33390_e42094).powf(assign33390_e42098 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign33390_e42099 * (assign33390_e42098 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign33390_e42094))) }, if 0.0 == 0.0 && ((assign33390_e42098) as f64).is_finite() && ((assign33390_e42098) as f64).fract() == 0.0 { if assign33390_e42098 == 0.0 { 0.0 } else { (assign33390_e42098 * ((assign33390_e42094).powf(assign33390_e42098 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign33390_e42099 * (assign33390_e42098 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign33390_e42094))) }, if 0.0 == 0.0 && ((assign33390_e42098) as f64).is_finite() && ((assign33390_e42098) as f64).fract() == 0.0 { if assign33390_e42098 == 0.0 { 0.0 } else { (assign33390_e42098 * ((assign33390_e42094).powf(assign33390_e42098 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign33390_e42099 * (assign33390_e42098 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign33390_e42094))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign33390_e42101;
        var_wgamma_dn5 = assign33390_e42101_d_n5;
        var_wgamma_dn6 = assign33390_e42101_d_n6;
        var_wgamma_dn7 = assign33390_e42101_d_n7;
        var_wgamma_dn8 = assign33390_e42101_d_n8;

        let (assign33400_e42119, assign33400_e42119_d_n5, assign33400_e42119_d_n6, assign33400_e42119_d_n7, assign33400_e42119_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33400_e42113: f64 = (var_wsrh * var_wgamma);
        let assign33400_e42116: f64 = (var_wsrh + var_wgamma);
        let assign33400_e42117: f64 = (assign33400_e42113 / assign33400_e42116);
        (assign33400_e42117, ((((var_wsrh * var_wgamma_dn5) * assign33400_e42116) - (assign33400_e42113 * var_wgamma_dn5)) / (assign33400_e42116 * assign33400_e42116)), ((((var_wsrh * var_wgamma_dn6) * assign33400_e42116) - (assign33400_e42113 * var_wgamma_dn6)) / (assign33400_e42116 * assign33400_e42116)), ((((var_wsrh * var_wgamma_dn7) * assign33400_e42116) - (assign33400_e42113 * var_wgamma_dn7)) / (assign33400_e42116 * assign33400_e42116)), ((((var_wsrh * var_wgamma_dn8) * assign33400_e42116) - (assign33400_e42113 * var_wgamma_dn8)) / (assign33400_e42116 * assign33400_e42116)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign33400_e42119;
        var_wtat_dn5 = assign33400_e42119_d_n5;
        var_wtat_dn6 = assign33400_e42119_d_n6;
        var_wtat_dn7 = assign33400_e42119_d_n7;
        var_wtat_dn8 = assign33400_e42119_d_n8;

        let (assign33410_e42136, assign33410_e42136_d_n5, assign33410_e42136_d_n6, assign33410_e42136_d_n7, assign33410_e42136_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33410_e42132: f64 = (var_btat / var_sqrtumax);
        let assign33410_e42133: f64 = (0.375 * assign33410_e42132);
        let assign33410_e42134: f64 = (assign33410_e42133).sqrt();
        (assign33410_e42134, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33410_e42134)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33410_e42134)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33410_e42134)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33410_e42134)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign33410_e42136;
        var_ktat_dn5 = assign33410_e42136_d_n5;
        var_ktat_dn6 = assign33410_e42136_d_n6;
        var_ktat_dn7 = assign33410_e42136_d_n7;
        var_ktat_dn8 = assign33410_e42136_d_n8;

        let (assign33420_e42154, assign33420_e42154_d_n5, assign33420_e42154_d_n6, assign33420_e42154_d_n7, assign33420_e42154_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33420_e42149: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign33420_e42150: f64 = (2.0 * assign33420_e42149);
        let assign33420_e42152: f64 = (assign33420_e42150 - var_umax);
        (assign33420_e42152, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign33420_e42154;
        var_ltat_dn5 = assign33420_e42154_d_n5;
        var_ltat_dn6 = assign33420_e42154_d_n6;
        var_ltat_dn7 = assign33420_e42154_d_n7;
        var_ltat_dn8 = assign33420_e42154_d_n8;

        let (assign33430_e42180, assign33430_e42180_d_n5, assign33430_e42180_d_n6, assign33430_e42180_d_n7, assign33430_e42180_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33430_e42166: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign33430_e42168: f64 = (assign33430_e42166 * var_sqrtumax);
        let assign33430_e42171: f64 = (var_atatbot_d * var_umax);
        let assign33430_e42172: f64 = (assign33430_e42168 - assign33430_e42171);
        let assign33430_e42176: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33430_e42177: f64 = (0.5 * assign33430_e42176);
        let assign33430_e42178: f64 = (assign33430_e42172 + assign33430_e42177);
        (assign33430_e42178, (((((var_atatbot_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign33430_e42166 * var_sqrtumax_dn5)) - (var_atatbot_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign33430_e42166 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign33430_e42166 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign33430_e42166 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign33430_e42180;
        var_mtat_dn5 = assign33430_e42180_d_n5;
        var_mtat_dn6 = assign33430_e42180_d_n6;
        var_mtat_dn7 = assign33430_e42180_d_n7;
        var_mtat_dn8 = assign33430_e42180_d_n8;

        let (assign33440_e42196, assign33440_e42196_d_n5, assign33440_e42196_d_n6, assign33440_e42196_d_n7, assign33440_e42196_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33440_e42192: f64 = (var_ltat - 1.0);
        let assign33440_e42194: f64 = (assign33440_e42192 * var_ktat);
        (assign33440_e42194, ((var_ltat_dn5 * var_ktat) + (assign33440_e42192 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign33440_e42192 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign33440_e42192 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign33440_e42192 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign33440_e42196;
        var_xerfc_dn5 = assign33440_e42196_d_n5;
        var_xerfc_dn6 = assign33440_e42196_d_n6;
        var_xerfc_dn7 = assign33440_e42196_d_n7;
        var_xerfc_dn8 = assign33440_e42196_d_n8;

        let (assign33450_e42210, assign33450_e42210_d_n5, assign33450_e42210_d_n6, assign33450_e42210_d_n7, assign33450_e42210_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33450_e42208: f64 = (var_xerfc * var_xerfc);
        (assign33450_e42208, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign33450_e42210;
        var_ysq_dn5 = assign33450_e42210_d_n5;
        var_ysq_dn6 = assign33450_e42210_d_n6;
        var_ysq_dn7 = assign33450_e42210_d_n7;
        var_ysq_dn8 = assign33450_e42210_d_n8;

        let assign33460_e42213: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard661 = assign33460_e42213;

        let (assign33470_e42233, assign33470_e42233_d_n5, assign33470_e42233_d_n6, assign33470_e42233_d_n7, assign33470_e42233_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) && (var_guard661 != 0.0)) {
        let assign33470_e42229: f64 = (var_perfc * var_xerfc);
        let assign33470_e42230: f64 = (1.0 + assign33470_e42229);
        let assign33470_e42231: f64 = (1.0 / assign33470_e42230);
        (assign33470_e42231, (-((var_perfc * var_xerfc_dn5) / (assign33470_e42230 * assign33470_e42230))), (-((var_perfc * var_xerfc_dn6) / (assign33470_e42230 * assign33470_e42230))), (-((var_perfc * var_xerfc_dn7) / (assign33470_e42230 * assign33470_e42230))), (-((var_perfc * var_xerfc_dn8) / (assign33470_e42230 * assign33470_e42230))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign33470_e42233;
        var_terfc_dn5 = assign33470_e42233_d_n5;
        var_terfc_dn6 = assign33470_e42233_d_n6;
        var_terfc_dn7 = assign33470_e42233_d_n7;
        var_terfc_dn8 = assign33470_e42233_d_n8;

        let (assign33480_e42254, assign33480_e42254_d_n5, assign33480_e42254_d_n6, assign33480_e42254_d_n7, assign33480_e42254_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) && (var_guard661 == 0.0)) {
        let assign33480_e42250: f64 = (var_perfc * var_xerfc);
        let assign33480_e42251: f64 = (1.0 - assign33480_e42250);
        let assign33480_e42252: f64 = (1.0 / assign33480_e42251);
        (assign33480_e42252, (-((-(var_perfc * var_xerfc_dn5)) / (assign33480_e42251 * assign33480_e42251))), (-((-(var_perfc * var_xerfc_dn6)) / (assign33480_e42251 * assign33480_e42251))), (-((-(var_perfc * var_xerfc_dn7)) / (assign33480_e42251 * assign33480_e42251))), (-((-(var_perfc * var_xerfc_dn8)) / (assign33480_e42251 * assign33480_e42251))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign33480_e42254;
        var_terfc_dn5 = assign33480_e42254_d_n5;
        var_terfc_dn6 = assign33480_e42254_d_n6;
        var_terfc_dn7 = assign33480_e42254_d_n7;
        var_terfc_dn8 = assign33480_e42254_d_n8;

        let assign33490_e42256: f64 = (-var_ysq);
        let assign33490_e42258: f64 = (assign33490_e42256 + var_mtat);
        let assign33490_e42260: f64 = (-230.25850929940458);
        let assign33490_e42261: f64 = if assign33490_e42258 > assign33490_e42260 { 1.0 } else { 0.0 };
        var_guard662 = assign33490_e42261;

        let (assign33500_e42279, assign33500_e42279_d_n5, assign33500_e42279_d_n6, assign33500_e42279_d_n7, assign33500_e42279_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) && (var_guard662 != 0.0)) {
        let assign33500_e42274: f64 = (-var_ysq);
        let assign33500_e42276: f64 = (assign33500_e42274 + var_mtat);
        let assign33500_e42277: f64 = (assign33500_e42276).exp();
        (assign33500_e42277, (assign33500_e42277 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign33500_e42277 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign33500_e42277 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign33500_e42277 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33500_e42279;
        var_tmp_dn5 = assign33500_e42279_d_n5;
        var_tmp_dn6 = assign33500_e42279_d_n6;
        var_tmp_dn7 = assign33500_e42279_d_n7;
        var_tmp_dn8 = assign33500_e42279_d_n8;

        let (assign33510_e42328, assign33510_e42328_d_n5, assign33510_e42328_d_n6, assign33510_e42328_d_n7, assign33510_e42328_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) && (var_guard662 == 0.0)) {
        let assign33510_e42295: f64 = (-230.25850929940458);
        let assign33510_e42297: f64 = (-var_ysq);
        let assign33510_e42299: f64 = (assign33510_e42297 + var_mtat);
        let assign33510_e42300: f64 = (assign33510_e42295 - assign33510_e42299);
        let assign33510_e42304: f64 = (-230.25850929940458);
        let assign33510_e42306: f64 = (-var_ysq);
        let assign33510_e42308: f64 = (assign33510_e42306 + var_mtat);
        let assign33510_e42309: f64 = (assign33510_e42304 - assign33510_e42308);
        let assign33510_e42312: f64 = (-230.25850929940458);
        let assign33510_e42314: f64 = (-var_ysq);
        let assign33510_e42316: f64 = (assign33510_e42314 + var_mtat);
        let assign33510_e42317: f64 = (assign33510_e42312 - assign33510_e42316);
        let assign33510_e42319: f64 = (assign33510_e42317 * 0.3333333333333333);
        let assign33510_e42320: f64 = (1.0 + assign33510_e42319);
        let assign33510_e42321: f64 = (assign33510_e42309 * assign33510_e42320);
        let assign33510_e42322: f64 = (0.5 * assign33510_e42321);
        let assign33510_e42323: f64 = (1.0 + assign33510_e42322);
        let assign33510_e42324: f64 = (assign33510_e42300 * assign33510_e42323);
        let assign33510_e42325: f64 = (1.0 + assign33510_e42324);
        let assign33510_e42326: f64 = (1e-100 / assign33510_e42325);
        (assign33510_e42326, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign33510_e42323) + (assign33510_e42300 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign33510_e42320) + (assign33510_e42309 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign33510_e42325 * assign33510_e42325))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign33510_e42323) + (assign33510_e42300 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign33510_e42320) + (assign33510_e42309 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign33510_e42325 * assign33510_e42325))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign33510_e42323) + (assign33510_e42300 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign33510_e42320) + (assign33510_e42309 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign33510_e42325 * assign33510_e42325))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign33510_e42323) + (assign33510_e42300 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign33510_e42320) + (assign33510_e42309 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign33510_e42325 * assign33510_e42325))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33510_e42328;
        var_tmp_dn5 = assign33510_e42328_d_n5;
        var_tmp_dn6 = assign33510_e42328_d_n6;
        var_tmp_dn7 = assign33510_e42328_d_n7;
        var_tmp_dn8 = assign33510_e42328_d_n8;

        let (assign33520_e42358, assign33520_e42358_d_n5, assign33520_e42358_d_n6, assign33520_e42358_d_n7, assign33520_e42358_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33520_e42340: f64 = (0.29214664 * var_terfc);
        let assign33520_e42344: f64 = (var_terfc * var_terfc);
        let assign33520_e42345: f64 = (var_berfc * assign33520_e42344);
        let assign33520_e42346: f64 = (assign33520_e42340 + assign33520_e42345);
        let assign33520_e42350: f64 = (var_terfc * var_terfc);
        let assign33520_e42352: f64 = (assign33520_e42350 * var_terfc);
        let assign33520_e42353: f64 = (var_cerfc * assign33520_e42352);
        let assign33520_e42354: f64 = (assign33520_e42346 + assign33520_e42353);
        let assign33520_e42356: f64 = (assign33520_e42354 * var_tmp);
        (assign33520_e42356, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign33520_e42350 * var_terfc_dn5)))) * var_tmp) + (assign33520_e42354 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign33520_e42350 * var_terfc_dn6)))) * var_tmp) + (assign33520_e42354 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign33520_e42350 * var_terfc_dn7)))) * var_tmp) + (assign33520_e42354 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign33520_e42350 * var_terfc_dn8)))) * var_tmp) + (assign33520_e42354 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign33520_e42358;
        var_erfcpos_dn5 = assign33520_e42358_d_n5;
        var_erfcpos_dn6 = assign33520_e42358_d_n6;
        var_erfcpos_dn7 = assign33520_e42358_d_n7;
        var_erfcpos_dn8 = assign33520_e42358_d_n8;

        let assign33530_e42361: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard663 = assign33530_e42361;

        let (assign33540_e42375, assign33540_e42375_d_n5, assign33540_e42375_d_n6, assign33540_e42375_d_n7, assign33540_e42375_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) && (var_guard663 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign33540_e42375;
        var_erfctimesexpmtat_dn5 = assign33540_e42375_d_n5;
        var_erfctimesexpmtat_dn6 = assign33540_e42375_d_n6;
        var_erfctimesexpmtat_dn7 = assign33540_e42375_d_n7;
        var_erfctimesexpmtat_dn8 = assign33540_e42375_d_n8;

        let assign33550_e42378: f64 = (-230.25850929940458);
        let assign33550_e42379: f64 = if var_mtat > assign33550_e42378 { 1.0 } else { 0.0 };
        var_guard664 = assign33550_e42379;

        let (assign33560_e42397, assign33560_e42397_d_n5, assign33560_e42397_d_n6, assign33560_e42397_d_n7, assign33560_e42397_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) && (var_guard663 == 0.0)) && (var_guard664 != 0.0)) {
        let assign33560_e42395: f64 = (var_mtat).exp();
        (assign33560_e42395, (assign33560_e42395 * var_mtat_dn5), (assign33560_e42395 * var_mtat_dn6), (assign33560_e42395 * var_mtat_dn7), (assign33560_e42395 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33560_e42397;
        var_tmp_dn5 = assign33560_e42397_d_n5;
        var_tmp_dn6 = assign33560_e42397_d_n6;
        var_tmp_dn7 = assign33560_e42397_d_n7;
        var_tmp_dn8 = assign33560_e42397_d_n8;

        let (assign33570_e42440, assign33570_e42440_d_n5, assign33570_e42440_d_n6, assign33570_e42440_d_n7, assign33570_e42440_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) && (var_guard663 == 0.0)) && (var_guard664 == 0.0)) {
        let assign33570_e42416: f64 = (-230.25850929940458);
        let assign33570_e42418: f64 = (assign33570_e42416 - var_mtat);
        let assign33570_e42422: f64 = (-230.25850929940458);
        let assign33570_e42424: f64 = (assign33570_e42422 - var_mtat);
        let assign33570_e42427: f64 = (-230.25850929940458);
        let assign33570_e42429: f64 = (assign33570_e42427 - var_mtat);
        let assign33570_e42431: f64 = (assign33570_e42429 * 0.3333333333333333);
        let assign33570_e42432: f64 = (1.0 + assign33570_e42431);
        let assign33570_e42433: f64 = (assign33570_e42424 * assign33570_e42432);
        let assign33570_e42434: f64 = (0.5 * assign33570_e42433);
        let assign33570_e42435: f64 = (1.0 + assign33570_e42434);
        let assign33570_e42436: f64 = (assign33570_e42418 * assign33570_e42435);
        let assign33570_e42437: f64 = (1.0 + assign33570_e42436);
        let assign33570_e42438: f64 = (1e-100 / assign33570_e42437);
        (assign33570_e42438, (-((1e-100 * (((-var_mtat_dn5) * assign33570_e42435) + (assign33570_e42418 * (0.5 * (((-var_mtat_dn5) * assign33570_e42432) + (assign33570_e42424 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign33570_e42437 * assign33570_e42437))), (-((1e-100 * (((-var_mtat_dn6) * assign33570_e42435) + (assign33570_e42418 * (0.5 * (((-var_mtat_dn6) * assign33570_e42432) + (assign33570_e42424 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign33570_e42437 * assign33570_e42437))), (-((1e-100 * (((-var_mtat_dn7) * assign33570_e42435) + (assign33570_e42418 * (0.5 * (((-var_mtat_dn7) * assign33570_e42432) + (assign33570_e42424 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign33570_e42437 * assign33570_e42437))), (-((1e-100 * (((-var_mtat_dn8) * assign33570_e42435) + (assign33570_e42418 * (0.5 * (((-var_mtat_dn8) * assign33570_e42432) + (assign33570_e42424 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign33570_e42437 * assign33570_e42437))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33570_e42440;
        var_tmp_dn5 = assign33570_e42440_d_n5;
        var_tmp_dn6 = assign33570_e42440_d_n6;
        var_tmp_dn7 = assign33570_e42440_d_n7;
        var_tmp_dn8 = assign33570_e42440_d_n8;

        let (assign33580_e42459, assign33580_e42459_d_n5, assign33580_e42459_d_n6, assign33580_e42459_d_n7, assign33580_e42459_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) && (var_guard663 == 0.0)) {
        let assign33580_e42455: f64 = (2.0 * var_tmp);
        let assign33580_e42457: f64 = (assign33580_e42455 - var_erfcpos);
        (assign33580_e42457, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign33580_e42459;
        var_erfctimesexpmtat_dn5 = assign33580_e42459_d_n5;
        var_erfctimesexpmtat_dn6 = assign33580_e42459_d_n6;
        var_erfctimesexpmtat_dn7 = assign33580_e42459_d_n7;
        var_erfctimesexpmtat_dn8 = assign33580_e42459_d_n8;

        let (assign33590_e42479, assign33590_e42479_d_n5, assign33590_e42479_d_n6, assign33590_e42479_d_n7, assign33590_e42479_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33590_e42471: f64 = (1.772453850905516 * 0.5);
        let assign33590_e42474: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign33590_e42476: f64 = (assign33590_e42474 / var_ktat);
        let assign33590_e42477: f64 = (assign33590_e42471 * assign33590_e42476);
        (assign33590_e42477, (assign33590_e42471 * ((((var_atatbot_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign33590_e42474 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign33590_e42471 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign33590_e42474 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign33590_e42471 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign33590_e42474 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign33590_e42471 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign33590_e42474 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign33590_e42479;
        var_gammamax_dn5 = assign33590_e42479_d_n5;
        var_gammamax_dn6 = assign33590_e42479_d_n6;
        var_gammamax_dn7 = assign33590_e42479_d_n7;
        var_gammamax_dn8 = assign33590_e42479_d_n8;

        let (assign33600_e42497, assign33600_e42497_d_n5, assign33600_e42497_d_n6, assign33600_e42497_d_n7, assign33600_e42497_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard659 == 0.0)) {
        let assign33600_e42492: f64 = (var_asrh * var_gammamax);
        let assign33600_e42494: f64 = (assign33600_e42492 * var_wtat);
        let assign33600_e42495: f64 = (var_ctatbotd_i * assign33600_e42494);
        (assign33600_e42495, (var_ctatbotd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign33600_e42492 * var_wtat_dn5))), (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign33600_e42492 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign33600_e42492 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign33600_e42492 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign33600_e42497;
        var_itat_dn5 = assign33600_e42497_d_n5;
        var_itat_dn6 = assign33600_e42497_d_n6;
        var_itat_dn7 = assign33600_e42497_d_n7;
        var_itat_dn8 = assign33600_e42497_d_n8;

        let assign33610_e42500: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard665 = assign33610_e42500;

        let (assign33620_e42511, assign33620_e42511_d_n5, assign33620_e42511_d_n6, assign33620_e42511_d_n7, assign33620_e42511_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard665 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign33620_e42511;
        var_ibbt_dn5 = assign33620_e42511_d_n5;
        var_ibbt_dn6 = assign33620_e42511_d_n6;
        var_ibbt_dn7 = assign33620_e42511_d_n7;
        var_ibbt_dn8 = assign33620_e42511_d_n8;

        let assign33630_e42514: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard666 = assign33630_e42514;

        let (assign33640_e42533, assign33640_e42533_d_n5, assign33640_e42533_d_n6, assign33640_e42533_d_n7, assign33640_e42533_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard665 == 0.0)) && (var_guard666 != 0.0)) {
        let assign33640_e42528: f64 = (var_vbirbotd_i - var_vbbt);
        let assign33640_e42530: f64 = (assign33640_e42528 * var_vbirbotinv_d);
        let assign33640_e42531: f64 = (assign33640_e42530).sqrt();
        (assign33640_e42531, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33640_e42533;
        var_tmp_dn5 = assign33640_e42533_d_n5;
        var_tmp_dn6 = assign33640_e42533_d_n6;
        var_tmp_dn7 = assign33640_e42533_d_n7;
        var_tmp_dn8 = assign33640_e42533_d_n8;

        let (assign33650_e42554, assign33650_e42554_d_n5, assign33650_e42554_d_n6, assign33650_e42554_d_n7, assign33650_e42554_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) {
        let assign33650_e42548: f64 = (var_vbirbotd_i - var_vbbt);
        let assign33650_e42550: f64 = (assign33650_e42548 * var_vbirbotinv_d);
        let assign33650_e42552: f64 = (assign33650_e42550).powf(var_pbotd_i);
        (assign33650_e42552, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33650_e42554;
        var_tmp_dn5 = assign33650_e42554_d_n5;
        var_tmp_dn6 = assign33650_e42554_d_n6;
        var_tmp_dn7 = assign33650_e42554_d_n7;
        var_tmp_dn8 = assign33650_e42554_d_n8;

        let (assign33660_e42574, assign33660_e42574_d_n5, assign33660_e42574_d_n6, assign33660_e42574_d_n7, assign33660_e42574_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard665 == 0.0)) {
        let assign33660_e42567: f64 = (var_vbirbotd_i - var_vbbt);
        let assign33660_e42569: f64 = (assign33660_e42567 * var_wdepnulrinvbot_d);
        let assign33660_e42571: f64 = (assign33660_e42569 / var_tmp);
        let assign33660_e42572: f64 = (var_one_over_one_minus_pbot_d * assign33660_e42571);
        (assign33660_e42572, (var_one_over_one_minus_pbot_d * (-((assign33660_e42569 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign33660_e42569 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign33660_e42569 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign33660_e42569 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign33660_e42574;
        var_fmaxr_dn5 = assign33660_e42574_d_n5;
        var_fmaxr_dn6 = assign33660_e42574_d_n6;
        var_fmaxr_dn7 = assign33660_e42574_d_n7;
        var_fmaxr_dn8 = assign33660_e42574_d_n8;

        let assign33670_e42576: f64 = (-var_fbbtbot_d);
        let assign33670_e42578: f64 = (assign33670_e42576 / var_fmaxr);
        let assign33670_e42579: f64 = (assign33670_e42578).abs();
        let assign33670_e42581: f64 = if assign33670_e42579 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard667 = assign33670_e42581;

        let (assign33680_e42599, assign33680_e42599_d_n5, assign33680_e42599_d_n6, assign33680_e42599_d_n7, assign33680_e42599_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard665 == 0.0)) && (var_guard667 != 0.0)) {
        let assign33680_e42594: f64 = (-var_fbbtbot_d);
        let assign33680_e42596: f64 = (assign33680_e42594 / var_fmaxr);
        let assign33680_e42597: f64 = (assign33680_e42596).exp();
        (assign33680_e42597, (assign33680_e42597 * (-((assign33680_e42594 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign33680_e42597 * (-((assign33680_e42594 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign33680_e42597 * (-((assign33680_e42594 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign33680_e42597 * (-((assign33680_e42594 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33680_e42599;
        var_tmp_dn5 = assign33680_e42599_d_n5;
        var_tmp_dn6 = assign33680_e42599_d_n6;
        var_tmp_dn7 = assign33680_e42599_d_n7;
        var_tmp_dn8 = assign33680_e42599_d_n8;

        let assign33690_e42601: f64 = (-var_fbbtbot_d);
        let assign33690_e42603: f64 = (assign33690_e42601 / var_fmaxr);
        let assign33690_e42605: f64 = if assign33690_e42603 < 0.0 { 1.0 } else { 0.0 };
        var_guard668 = assign33690_e42605;

        let (assign33700_e42656, assign33700_e42656_d_n5, assign33700_e42656_d_n6, assign33700_e42656_d_n7, assign33700_e42656_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard665 == 0.0)) && (var_guard667 == 0.0)) && (var_guard668 != 0.0)) {
        let assign33700_e42623: f64 = (-230.25850929940458);
        let assign33700_e42625: f64 = (-var_fbbtbot_d);
        let assign33700_e42627: f64 = (assign33700_e42625 / var_fmaxr);
        let assign33700_e42628: f64 = (assign33700_e42623 - assign33700_e42627);
        let assign33700_e42632: f64 = (-230.25850929940458);
        let assign33700_e42634: f64 = (-var_fbbtbot_d);
        let assign33700_e42636: f64 = (assign33700_e42634 / var_fmaxr);
        let assign33700_e42637: f64 = (assign33700_e42632 - assign33700_e42636);
        let assign33700_e42640: f64 = (-230.25850929940458);
        let assign33700_e42642: f64 = (-var_fbbtbot_d);
        let assign33700_e42644: f64 = (assign33700_e42642 / var_fmaxr);
        let assign33700_e42645: f64 = (assign33700_e42640 - assign33700_e42644);
        let assign33700_e42647: f64 = (assign33700_e42645 * 0.3333333333333333);
        let assign33700_e42648: f64 = (1.0 + assign33700_e42647);
        let assign33700_e42649: f64 = (assign33700_e42637 * assign33700_e42648);
        let assign33700_e42650: f64 = (0.5 * assign33700_e42649);
        let assign33700_e42651: f64 = (1.0 + assign33700_e42650);
        let assign33700_e42652: f64 = (assign33700_e42628 * assign33700_e42651);
        let assign33700_e42653: f64 = (1.0 + assign33700_e42652);
        let assign33700_e42654: f64 = (1e-100 / assign33700_e42653);
        (assign33700_e42654, (-((1e-100 * (((-(-((assign33700_e42625 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign33700_e42651) + (assign33700_e42628 * (0.5 * (((-(-((assign33700_e42634 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign33700_e42648) + (assign33700_e42637 * ((-(-((assign33700_e42642 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33700_e42653 * assign33700_e42653))), (-((1e-100 * (((-(-((assign33700_e42625 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign33700_e42651) + (assign33700_e42628 * (0.5 * (((-(-((assign33700_e42634 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign33700_e42648) + (assign33700_e42637 * ((-(-((assign33700_e42642 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33700_e42653 * assign33700_e42653))), (-((1e-100 * (((-(-((assign33700_e42625 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign33700_e42651) + (assign33700_e42628 * (0.5 * (((-(-((assign33700_e42634 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign33700_e42648) + (assign33700_e42637 * ((-(-((assign33700_e42642 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33700_e42653 * assign33700_e42653))), (-((1e-100 * (((-(-((assign33700_e42625 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign33700_e42651) + (assign33700_e42628 * (0.5 * (((-(-((assign33700_e42634 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign33700_e42648) + (assign33700_e42637 * ((-(-((assign33700_e42642 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33700_e42653 * assign33700_e42653))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33700_e42656;
        var_tmp_dn5 = assign33700_e42656_d_n5;
        var_tmp_dn6 = assign33700_e42656_d_n6;
        var_tmp_dn7 = assign33700_e42656_d_n7;
        var_tmp_dn8 = assign33700_e42656_d_n8;

        let (assign33710_e42705, assign33710_e42705_d_n5, assign33710_e42705_d_n6, assign33710_e42705_d_n7, assign33710_e42705_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard665 == 0.0)) && (var_guard667 == 0.0)) && (var_guard668 == 0.0)) {
        let assign33710_e42675: f64 = (-var_fbbtbot_d);
        let assign33710_e42677: f64 = (assign33710_e42675 / var_fmaxr);
        let assign33710_e42679: f64 = (assign33710_e42677 - 230.25850929940458);
        let assign33710_e42683: f64 = (-var_fbbtbot_d);
        let assign33710_e42685: f64 = (assign33710_e42683 / var_fmaxr);
        let assign33710_e42687: f64 = (assign33710_e42685 - 230.25850929940458);
        let assign33710_e42690: f64 = (-var_fbbtbot_d);
        let assign33710_e42692: f64 = (assign33710_e42690 / var_fmaxr);
        let assign33710_e42694: f64 = (assign33710_e42692 - 230.25850929940458);
        let assign33710_e42696: f64 = (assign33710_e42694 * 0.3333333333333333);
        let assign33710_e42697: f64 = (1.0 + assign33710_e42696);
        let assign33710_e42698: f64 = (assign33710_e42687 * assign33710_e42697);
        let assign33710_e42699: f64 = (0.5 * assign33710_e42698);
        let assign33710_e42700: f64 = (1.0 + assign33710_e42699);
        let assign33710_e42701: f64 = (assign33710_e42679 * assign33710_e42700);
        let assign33710_e42702: f64 = (1.0 + assign33710_e42701);
        let assign33710_e42703: f64 = (1e100 * assign33710_e42702);
        (assign33710_e42703, (1e100 * (((-((assign33710_e42675 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign33710_e42700) + (assign33710_e42679 * (0.5 * (((-((assign33710_e42683 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign33710_e42697) + (assign33710_e42687 * ((-((assign33710_e42690 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33710_e42675 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign33710_e42700) + (assign33710_e42679 * (0.5 * (((-((assign33710_e42683 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign33710_e42697) + (assign33710_e42687 * ((-((assign33710_e42690 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33710_e42675 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign33710_e42700) + (assign33710_e42679 * (0.5 * (((-((assign33710_e42683 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign33710_e42697) + (assign33710_e42687 * ((-((assign33710_e42690 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33710_e42675 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign33710_e42700) + (assign33710_e42679 * (0.5 * (((-((assign33710_e42683 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign33710_e42697) + (assign33710_e42687 * ((-((assign33710_e42690 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33710_e42705;
        var_tmp_dn5 = assign33710_e42705_d_n5;
        var_tmp_dn6 = assign33710_e42705_d_n6;
        var_tmp_dn7 = assign33710_e42705_d_n7;
        var_tmp_dn8 = assign33710_e42705_d_n8;

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
        *var_guard661_slot = var_guard661;
        *var_guard662_slot = var_guard662;
        *var_guard663_slot = var_guard663;
        *var_guard664_slot = var_guard664;
        *var_guard665_slot = var_guard665;
        *var_guard666_slot = var_guard666;
        *var_guard667_slot = var_guard667;
        *var_guard668_slot = var_guard668;
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

    pub(super) fn stamp_transient_block_69(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti_d: f64,
        var_btatpartsti_d: f64,
        var_cbbtbotd_i: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_fmaxr: f64,
        var_fmaxr_dn5: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fstopbot_d: f64,
        var_ftdsti_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard655: f64,
        var_guard665: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
        var_lsdrain_i: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrbotd_i: f64,
        var_pstid_i: f64,
        var_slopebot_d: f64,
        var_two_psistar: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbirstiinv_d: f64,
        var_vbisti_d: f64,
        var_vbrbotd_i: f64,
        var_vbrinvbot_d: f64,
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
        var_guard669_slot: &mut f64,
        var_guard670_slot: &mut f64,
        var_guard671_slot: &mut f64,
        var_guard672_slot: &mut f64,
        var_guard673_slot: &mut f64,
        var_guard674_slot: &mut f64,
        var_guard675_slot: &mut f64,
        var_guard676_slot: &mut f64,
        var_guard677_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard669: f64 = *var_guard669_slot;
        let mut var_guard670: f64 = *var_guard670_slot;
        let mut var_guard671: f64 = *var_guard671_slot;
        let mut var_guard672: f64 = *var_guard672_slot;
        let mut var_guard673: f64 = *var_guard673_slot;
        let mut var_guard674: f64 = *var_guard674_slot;
        let mut var_guard675: f64 = *var_guard675_slot;
        let mut var_guard676: f64 = *var_guard676_slot;
        let mut var_guard677: f64 = *var_guard677_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let (assign33720_e42725, assign33720_e42725_d_n5, assign33720_e42725_d_n6, assign33720_e42725_d_n7, assign33720_e42725_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard665 == 0.0)) {
        let assign33720_e42718: f64 = (var_v3 * var_fmaxr);
        let assign33720_e42720: f64 = (assign33720_e42718 * var_fmaxr);
        let assign33720_e42722: f64 = (assign33720_e42720 * var_tmp);
        let assign33720_e42723: f64 = (var_cbbtbotd_i * assign33720_e42722);
        (assign33720_e42723, (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign33720_e42718 * var_fmaxr_dn5)) * var_tmp) + (assign33720_e42720 * var_tmp_dn5))), (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign33720_e42718 * var_fmaxr_dn6)) * var_tmp) + (assign33720_e42720 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign33720_e42718 * var_fmaxr_dn7)) * var_tmp) + (assign33720_e42720 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign33720_e42718 * var_fmaxr_dn8)) * var_tmp) + (assign33720_e42720 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign33720_e42725;
        var_ibbt_dn5 = assign33720_e42725_d_n5;
        var_ibbt_dn6 = assign33720_e42725_d_n6;
        var_ibbt_dn7 = assign33720_e42725_d_n7;
        var_ibbt_dn8 = assign33720_e42725_d_n8;

        let assign33730_e42728: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard669 = assign33730_e42728;

        let (assign33740_e42739, assign33740_e42739_d_n5, assign33740_e42739_d_n6, assign33740_e42739_d_n7, assign33740_e42739_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard669 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign33740_e42739;
        var_fbreakdown_dn5 = assign33740_e42739_d_n5;
        var_fbreakdown_dn6 = assign33740_e42739_d_n6;
        var_fbreakdown_dn7 = assign33740_e42739_d_n7;
        var_fbreakdown_dn8 = assign33740_e42739_d_n8;

        let assign33750_e42742: f64 = (-var_alphaav);
        let assign33750_e42744: f64 = (assign33750_e42742 * var_vbrbotd_i);
        let assign33750_e42745: f64 = if var_vav > assign33750_e42744 { 1.0 } else { 0.0 };
        var_guard670 = assign33750_e42745;

        let assign33760_e42748: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard671 = assign33760_e42748;

        let (assign33770_e42778, assign33770_e42778_d_n5, assign33770_e42778_d_n6, assign33770_e42778_d_n7, assign33770_e42778_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard669 == 0.0)) && (var_guard670 != 0.0)) && (var_guard671 != 0.0)) {
        let assign33770_e42764: f64 = (var_vav * var_vbrinvbot_d);
        let assign33770_e42767: f64 = (var_vav * var_vbrinvbot_d);
        let assign33770_e42768: f64 = (assign33770_e42764 * assign33770_e42767);
        let assign33770_e42771: f64 = (var_vav * var_vbrinvbot_d);
        let assign33770_e42772: f64 = (assign33770_e42768 * assign33770_e42771);
        let assign33770_e42775: f64 = (var_vav * var_vbrinvbot_d);
        let assign33770_e42776: f64 = (assign33770_e42772 * assign33770_e42775);
        (assign33770_e42776, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33770_e42778;
        var_tmp_dn5 = assign33770_e42778_d_n5;
        var_tmp_dn6 = assign33770_e42778_d_n6;
        var_tmp_dn7 = assign33770_e42778_d_n7;
        var_tmp_dn8 = assign33770_e42778_d_n8;

        let (assign33780_e42800, assign33780_e42800_d_n5, assign33780_e42800_d_n6, assign33780_e42800_d_n7, assign33780_e42800_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard669 == 0.0)) && (var_guard670 != 0.0)) && (var_guard671 == 0.0)) {
        let assign33780_e42795: f64 = (var_vav * var_vbrinvbot_d);
        let assign33780_e42796: f64 = (assign33780_e42795).abs();
        let assign33780_e42798: f64 = (assign33780_e42796).powf(var_pbrbotd_i);
        (assign33780_e42798, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33780_e42800;
        var_tmp_dn5 = assign33780_e42800_d_n5;
        var_tmp_dn6 = assign33780_e42800_d_n6;
        var_tmp_dn7 = assign33780_e42800_d_n7;
        var_tmp_dn8 = assign33780_e42800_d_n8;

        let (assign33790_e42818, assign33790_e42818_d_n5, assign33790_e42818_d_n6, assign33790_e42818_d_n7, assign33790_e42818_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard669 == 0.0)) && (var_guard670 != 0.0)) {
        let assign33790_e42815: f64 = (1.0 - var_tmp);
        let assign33790_e42816: f64 = (1.0 / assign33790_e42815);
        (assign33790_e42816, (-((-var_tmp_dn5) / (assign33790_e42815 * assign33790_e42815))), (-((-var_tmp_dn6) / (assign33790_e42815 * assign33790_e42815))), (-((-var_tmp_dn7) / (assign33790_e42815 * assign33790_e42815))), (-((-var_tmp_dn8) / (assign33790_e42815 * assign33790_e42815))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign33790_e42818;
        var_fbreakdown_dn5 = assign33790_e42818_d_n5;
        var_fbreakdown_dn6 = assign33790_e42818_d_n6;
        var_fbreakdown_dn7 = assign33790_e42818_d_n7;
        var_fbreakdown_dn8 = assign33790_e42818_d_n8;

        let (assign33800_e42841, assign33800_e42841_d_n5, assign33800_e42841_d_n6, assign33800_e42841_d_n7, assign33800_e42841_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) && (var_guard669 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33800_e42835: f64 = (var_alphaav * var_vbrbotd_i);
        let assign33800_e42836: f64 = (var_vav + assign33800_e42835);
        let assign33800_e42838: f64 = (assign33800_e42836 * var_slopebot_d);
        let assign33800_e42839: f64 = (var_fstopbot_d + assign33800_e42838);
        (assign33800_e42839, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign33800_e42841;
        var_fbreakdown_dn5 = assign33800_e42841_d_n5;
        var_fbreakdown_dn6 = assign33800_e42841_d_n6;
        var_fbreakdown_dn7 = assign33800_e42841_d_n7;
        var_fbreakdown_dn8 = assign33800_e42841_d_n8;

        let (assign33810_e42860, assign33810_e42860_d_n5, assign33810_e42860_d_n6, assign33810_e42860_d_n7, assign33810_e42860_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard655 == 0.0)) {
        let assign33810_e42851: f64 = (var_id__blk219 + var_isrh);
        let assign33810_e42853: f64 = (assign33810_e42851 + var_itat);
        let assign33810_e42855: f64 = (assign33810_e42853 + var_ibbt);
        let assign33810_e42856: f64 = (p.p29 * assign33810_e42855);
        let assign33810_e42858: f64 = (assign33810_e42856 * var_fbreakdown);
        (assign33810_e42858, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign33810_e42856 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign33810_e42856 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign33810_e42856 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign33810_e42856 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign33810_e42860;
        var_ijunbot_dn5 = assign33810_e42860_d_n5;
        var_ijunbot_dn6 = assign33810_e42860_d_n6;
        var_ijunbot_dn7 = assign33810_e42860_d_n7;
        var_ijunbot_dn8 = assign33810_e42860_d_n8;

        let assign33820_e42863: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard672 = assign33820_e42863;

        let (assign33830_e42871, assign33830_e42871_d_n5, assign33830_e42871_d_n6, assign33830_e42871_d_n7, assign33830_e42871_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign33830_e42871;
        var_ijunsti_dn5 = assign33830_e42871_d_n5;
        var_ijunsti_dn6 = assign33830_e42871_d_n6;
        var_ijunsti_dn7 = assign33830_e42871_d_n7;
        var_ijunsti_dn8 = assign33830_e42871_d_n8;

        let (assign33840_e42882,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) {
        let assign33840_e42880: f64 = (var_idsatsti_d * var_idmult);
        (assign33840_e42880,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign33840_e42882;

        let assign33850_e42889: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard673 = assign33850_e42889;

        let (assign33860_e42900, assign33860_e42900_d_n5, assign33860_e42900_d_n6, assign33860_e42900_d_n7, assign33860_e42900_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign33860_e42900;
        var_isrh_dn5 = assign33860_e42900_d_n5;
        var_isrh_dn6 = assign33860_e42900_d_n6;
        var_isrh_dn7 = assign33860_e42900_d_n7;
        var_isrh_dn8 = assign33860_e42900_d_n8;

        let (assign33870_e42914,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 == 0.0)) {
        let assign33870_e42912: f64 = (var_vbisti_d - var_vjsrh);
        (assign33870_e42912,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign33870_e42914;

        let (assign33880_e42933,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 == 0.0)) {
        let assign33880_e42928: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign33880_e42929: f64 = (1.0 - assign33880_e42928);
        let assign33880_e42930: f64 = (assign33880_e42929).sqrt();
        let assign33880_e42931: f64 = (1.0 - assign33880_e42930);
        (assign33880_e42931,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign33880_e42933;

        let assign33890_e42936: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard674 = assign33890_e42936;

        let (assign33900_e42950,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 == 0.0)) && (var_guard674 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign33900_e42950;

        let (assign33910_e42982,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 == 0.0)) && (var_guard674 == 0.0)) {
        let assign33910_e42965: f64 = (var_wsrhstep * var_wsrhstep);
        let assign33910_e42967: f64 = (var_wsrhstep).ln();
        let assign33910_e42968: f64 = (assign33910_e42965 * assign33910_e42967);
        let assign33910_e42971: f64 = (1.0 - var_wsrhstep);
        let assign33910_e42972: f64 = (assign33910_e42968 / assign33910_e42971);
        let assign33910_e42974: f64 = (assign33910_e42972 + var_wsrhstep);
        let assign33910_e42978: f64 = (2.0 * var_pstid_i);
        let assign33910_e42979: f64 = (1.0 - assign33910_e42978);
        let assign33910_e42980: f64 = (assign33910_e42974 * assign33910_e42979);
        (assign33910_e42980,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign33910_e42982;

        let (assign33920_e42996,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 == 0.0)) {
        let assign33920_e42994: f64 = (var_wsrhstep + var_dwsrh);
        (assign33920_e42994,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign33920_e42996;

        let assign33930_e42999: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard675 = assign33930_e42999;

        let (assign33940_e43016, assign33940_e43016_d_n5, assign33940_e43016_d_n6, assign33940_e43016_d_n7, assign33940_e43016_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 == 0.0)) && (var_guard675 != 0.0)) {
        let assign33940_e43013: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign33940_e43014: f64 = (assign33940_e43013).sqrt();
        (assign33940_e43014, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33940_e43016;
        var_tmp_dn5 = assign33940_e43016_d_n5;
        var_tmp_dn6 = assign33940_e43016_d_n6;
        var_tmp_dn7 = assign33940_e43016_d_n7;
        var_tmp_dn8 = assign33940_e43016_d_n8;

        let (assign33950_e43035, assign33950_e43035_d_n5, assign33950_e43035_d_n6, assign33950_e43035_d_n7, assign33950_e43035_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 == 0.0)) && (var_guard675 == 0.0)) {
        let assign33950_e43031: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign33950_e43033: f64 = (assign33950_e43031).powf(var_pstid_i);
        (assign33950_e43033, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign33950_e43035;
        var_tmp_dn5 = assign33950_e43035_d_n5;
        var_tmp_dn6 = assign33950_e43035_d_n6;
        var_tmp_dn7 = assign33950_e43035_d_n7;
        var_tmp_dn8 = assign33950_e43035_d_n8;

        let (assign33960_e43049, assign33960_e43049_d_n5, assign33960_e43049_d_n6, assign33960_e43049_d_n7, assign33960_e43049_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 == 0.0)) {
        let assign33960_e43047: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign33960_e43047, (var_wdepnulrsti_d * var_tmp_dn5), (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign33960_e43049;
        var_wdep_dn5 = assign33960_e43049_d_n5;
        var_wdep_dn6 = assign33960_e43049_d_n6;
        var_wdep_dn7 = assign33960_e43049_d_n7;
        var_wdep_dn8 = assign33960_e43049_d_n8;

        let (assign33970_e43067, assign33970_e43067_d_n5, assign33970_e43067_d_n6, assign33970_e43067_d_n7, assign33970_e43067_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 == 0.0)) {
        let assign33970_e43062: f64 = (var_zinv - 1.0);
        let assign33970_e43064: f64 = (assign33970_e43062 * var_wdep);
        let assign33970_e43065: f64 = (var_ftdsti_d * assign33970_e43064);
        (assign33970_e43065, (var_ftdsti_d * (assign33970_e43062 * var_wdep_dn5)), (var_ftdsti_d * (assign33970_e43062 * var_wdep_dn6)), (var_ftdsti_d * (assign33970_e43062 * var_wdep_dn7)), (var_ftdsti_d * (assign33970_e43062 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign33970_e43067;
        var_asrh_dn5 = assign33970_e43067_d_n5;
        var_asrh_dn6 = assign33970_e43067_d_n6;
        var_asrh_dn7 = assign33970_e43067_d_n7;
        var_asrh_dn8 = assign33970_e43067_d_n8;

        let (assign33980_e43083, assign33980_e43083_d_n5, assign33980_e43083_d_n6, assign33980_e43083_d_n7, assign33980_e43083_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard673 == 0.0)) {
        let assign33980_e43080: f64 = (var_asrh * var_wsrh);
        let assign33980_e43081: f64 = (var_csrhstid_i * assign33980_e43080);
        (assign33980_e43081, (var_csrhstid_i * (var_asrh_dn5 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign33980_e43083;
        var_isrh_dn5 = assign33980_e43083_d_n5;
        var_isrh_dn6 = assign33980_e43083_d_n6;
        var_isrh_dn7 = assign33980_e43083_d_n7;
        var_isrh_dn8 = assign33980_e43083_d_n8;

        let assign33990_e43086: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard676 = assign33990_e43086;

        let (assign34000_e43097, assign34000_e43097_d_n5, assign34000_e43097_d_n6, assign34000_e43097_d_n7, assign34000_e43097_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign34000_e43097;
        var_itat_dn5 = assign34000_e43097_d_n5;
        var_itat_dn6 = assign34000_e43097_d_n6;
        var_itat_dn7 = assign34000_e43097_d_n7;
        var_itat_dn8 = assign34000_e43097_d_n8;

        let (assign34010_e43115, assign34010_e43115_d_n5, assign34010_e43115_d_n6, assign34010_e43115_d_n7, assign34010_e43115_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34010_e43110: f64 = (var_wdep * var_one_minus_psti_d);
        let assign34010_e43112: f64 = (assign34010_e43110 / var_vbi_minus_vjsrh);
        let assign34010_e43113: f64 = (var_btatpartsti_d * assign34010_e43112);
        (assign34010_e43113, (var_btatpartsti_d * ((var_wdep_dn5 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign34010_e43115;
        var_btat_dn5 = assign34010_e43115_d_n5;
        var_btat_dn6 = assign34010_e43115_d_n6;
        var_btat_dn7 = assign34010_e43115_d_n7;
        var_btat_dn8 = assign34010_e43115_d_n8;

        let (assign34020_e43131, assign34020_e43131_d_n5, assign34020_e43131_d_n6, assign34020_e43131_d_n7, assign34020_e43131_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34020_e43127: f64 = (0.666666666666667 * var_atatsti_d);
        let assign34020_e43129: f64 = (assign34020_e43127 / var_btat);
        (assign34020_e43129, (-((assign34020_e43127 * var_btat_dn5) / (var_btat * var_btat))), (-((assign34020_e43127 * var_btat_dn6) / (var_btat * var_btat))), (-((assign34020_e43127 * var_btat_dn7) / (var_btat * var_btat))), (-((assign34020_e43127 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign34020_e43131;
        var_twoatatoverthreebtat_dn5 = assign34020_e43131_d_n5;
        var_twoatatoverthreebtat_dn6 = assign34020_e43131_d_n6;
        var_twoatatoverthreebtat_dn7 = assign34020_e43131_d_n7;
        var_twoatatoverthreebtat_dn8 = assign34020_e43131_d_n8;

        let (assign34030_e43145, assign34030_e43145_d_n5, assign34030_e43145_d_n6, assign34030_e43145_d_n7, assign34030_e43145_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34030_e43143: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign34030_e43143, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign34030_e43145;
        var_umaxbeforelimiting_dn5 = assign34030_e43145_d_n5;
        var_umaxbeforelimiting_dn6 = assign34030_e43145_d_n6;
        var_umaxbeforelimiting_dn7 = assign34030_e43145_d_n7;
        var_umaxbeforelimiting_dn8 = assign34030_e43145_d_n8;

        let (assign34040_e43166, assign34040_e43166_d_n5, assign34040_e43166_d_n6, assign34040_e43166_d_n7, assign34040_e43166_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34040_e43157: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign34040_e43160: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign34040_e43162: f64 = (assign34040_e43160 + 1.0);
        let assign34040_e43163: f64 = (assign34040_e43157 / assign34040_e43162);
        let assign34040_e43164: f64 = (assign34040_e43163).sqrt();
        (assign34040_e43164, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign34040_e43162) - (assign34040_e43157 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign34040_e43162 * assign34040_e43162)) / (2.0 * assign34040_e43164)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign34040_e43162) - (assign34040_e43157 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign34040_e43162 * assign34040_e43162)) / (2.0 * assign34040_e43164)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign34040_e43162) - (assign34040_e43157 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign34040_e43162 * assign34040_e43162)) / (2.0 * assign34040_e43164)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign34040_e43162) - (assign34040_e43157 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign34040_e43162 * assign34040_e43162)) / (2.0 * assign34040_e43164)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign34040_e43166;
        var_umax_dn5 = assign34040_e43166_d_n5;
        var_umax_dn6 = assign34040_e43166_d_n6;
        var_umax_dn7 = assign34040_e43166_d_n7;
        var_umax_dn8 = assign34040_e43166_d_n8;

        let (assign34050_e43179, assign34050_e43179_d_n5, assign34050_e43179_d_n6, assign34050_e43179_d_n7, assign34050_e43179_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34050_e43177: f64 = (var_umax).sqrt();
        (assign34050_e43177, (var_umax_dn5 / (2.0 * assign34050_e43177)), (var_umax_dn6 / (2.0 * assign34050_e43177)), (var_umax_dn7 / (2.0 * assign34050_e43177)), (var_umax_dn8 / (2.0 * assign34050_e43177)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign34050_e43179;
        var_sqrtumax_dn5 = assign34050_e43179_d_n5;
        var_sqrtumax_dn6 = assign34050_e43179_d_n6;
        var_sqrtumax_dn7 = assign34050_e43179_d_n7;
        var_sqrtumax_dn8 = assign34050_e43179_d_n8;

        let (assign34060_e43193, assign34060_e43193_d_n5, assign34060_e43193_d_n6, assign34060_e43193_d_n7, assign34060_e43193_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34060_e43191: f64 = (var_umax * var_sqrtumax);
        (assign34060_e43191, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign34060_e43193;
        var_umaxpoweronepointfive_dn5 = assign34060_e43193_d_n5;
        var_umaxpoweronepointfive_dn6 = assign34060_e43193_d_n6;
        var_umaxpoweronepointfive_dn7 = assign34060_e43193_d_n7;
        var_umaxpoweronepointfive_dn8 = assign34060_e43193_d_n8;

        let assign34070_e43195: f64 = (-var_pstid_i);
        let assign34070_e43197: f64 = (assign34070_e43195 * var_one_over_one_minus_psti_d);
        let assign34070_e43199: f64 = (-1.0);
        let assign34070_e43200: f64 = if assign34070_e43197 == assign34070_e43199 { 1.0 } else { 0.0 };
        var_guard677 = assign34070_e43200;

        let (assign34080_e43220, assign34080_e43220_d_n5, assign34080_e43220_d_n6, assign34080_e43220_d_n7, assign34080_e43220_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) && (var_guard677 != 0.0)) {
        let assign34080_e43216: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34080_e43217: f64 = (1.0 + assign34080_e43216);
        let assign34080_e43218: f64 = (1.0 / assign34080_e43217);
        (assign34080_e43218, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign34080_e43217 * assign34080_e43217))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign34080_e43217 * assign34080_e43217))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign34080_e43217 * assign34080_e43217))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign34080_e43217 * assign34080_e43217))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign34080_e43220;
        var_wgamma_dn5 = assign34080_e43220_d_n5;
        var_wgamma_dn6 = assign34080_e43220_d_n6;
        var_wgamma_dn7 = assign34080_e43220_d_n7;
        var_wgamma_dn8 = assign34080_e43220_d_n8;

        let (assign34090_e43244, assign34090_e43244_d_n5, assign34090_e43244_d_n6, assign34090_e43244_d_n7, assign34090_e43244_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) && (var_guard677 == 0.0)) {
        let assign34090_e43236: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34090_e43237: f64 = (1.0 + assign34090_e43236);
        let assign34090_e43239: f64 = (-var_pstid_i);
        let assign34090_e43241: f64 = (assign34090_e43239 * var_one_over_one_minus_psti_d);
        let assign34090_e43242: f64 = (assign34090_e43237).powf(assign34090_e43241);
        (assign34090_e43242, if 0.0 == 0.0 && ((assign34090_e43241) as f64).is_finite() && ((assign34090_e43241) as f64).fract() == 0.0 { if assign34090_e43241 == 0.0 { 0.0 } else { (assign34090_e43241 * ((assign34090_e43237).powf(assign34090_e43241 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign34090_e43242 * (assign34090_e43241 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign34090_e43237))) }, if 0.0 == 0.0 && ((assign34090_e43241) as f64).is_finite() && ((assign34090_e43241) as f64).fract() == 0.0 { if assign34090_e43241 == 0.0 { 0.0 } else { (assign34090_e43241 * ((assign34090_e43237).powf(assign34090_e43241 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign34090_e43242 * (assign34090_e43241 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign34090_e43237))) }, if 0.0 == 0.0 && ((assign34090_e43241) as f64).is_finite() && ((assign34090_e43241) as f64).fract() == 0.0 { if assign34090_e43241 == 0.0 { 0.0 } else { (assign34090_e43241 * ((assign34090_e43237).powf(assign34090_e43241 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign34090_e43242 * (assign34090_e43241 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign34090_e43237))) }, if 0.0 == 0.0 && ((assign34090_e43241) as f64).is_finite() && ((assign34090_e43241) as f64).fract() == 0.0 { if assign34090_e43241 == 0.0 { 0.0 } else { (assign34090_e43241 * ((assign34090_e43237).powf(assign34090_e43241 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign34090_e43242 * (assign34090_e43241 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign34090_e43237))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign34090_e43244;
        var_wgamma_dn5 = assign34090_e43244_d_n5;
        var_wgamma_dn6 = assign34090_e43244_d_n6;
        var_wgamma_dn7 = assign34090_e43244_d_n7;
        var_wgamma_dn8 = assign34090_e43244_d_n8;

        let (assign34100_e43262, assign34100_e43262_d_n5, assign34100_e43262_d_n6, assign34100_e43262_d_n7, assign34100_e43262_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34100_e43256: f64 = (var_wsrh * var_wgamma);
        let assign34100_e43259: f64 = (var_wsrh + var_wgamma);
        let assign34100_e43260: f64 = (assign34100_e43256 / assign34100_e43259);
        (assign34100_e43260, ((((var_wsrh * var_wgamma_dn5) * assign34100_e43259) - (assign34100_e43256 * var_wgamma_dn5)) / (assign34100_e43259 * assign34100_e43259)), ((((var_wsrh * var_wgamma_dn6) * assign34100_e43259) - (assign34100_e43256 * var_wgamma_dn6)) / (assign34100_e43259 * assign34100_e43259)), ((((var_wsrh * var_wgamma_dn7) * assign34100_e43259) - (assign34100_e43256 * var_wgamma_dn7)) / (assign34100_e43259 * assign34100_e43259)), ((((var_wsrh * var_wgamma_dn8) * assign34100_e43259) - (assign34100_e43256 * var_wgamma_dn8)) / (assign34100_e43259 * assign34100_e43259)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign34100_e43262;
        var_wtat_dn5 = assign34100_e43262_d_n5;
        var_wtat_dn6 = assign34100_e43262_d_n6;
        var_wtat_dn7 = assign34100_e43262_d_n7;
        var_wtat_dn8 = assign34100_e43262_d_n8;

        let (assign34110_e43279, assign34110_e43279_d_n5, assign34110_e43279_d_n6, assign34110_e43279_d_n7, assign34110_e43279_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34110_e43275: f64 = (var_btat / var_sqrtumax);
        let assign34110_e43276: f64 = (0.375 * assign34110_e43275);
        let assign34110_e43277: f64 = (assign34110_e43276).sqrt();
        (assign34110_e43277, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34110_e43277)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34110_e43277)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34110_e43277)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34110_e43277)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign34110_e43279;
        var_ktat_dn5 = assign34110_e43279_d_n5;
        var_ktat_dn6 = assign34110_e43279_d_n6;
        var_ktat_dn7 = assign34110_e43279_d_n7;
        var_ktat_dn8 = assign34110_e43279_d_n8;

        let (assign34120_e43297, assign34120_e43297_d_n5, assign34120_e43297_d_n6, assign34120_e43297_d_n7, assign34120_e43297_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34120_e43292: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign34120_e43293: f64 = (2.0 * assign34120_e43292);
        let assign34120_e43295: f64 = (assign34120_e43293 - var_umax);
        (assign34120_e43295, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign34120_e43297;
        var_ltat_dn5 = assign34120_e43297_d_n5;
        var_ltat_dn6 = assign34120_e43297_d_n6;
        var_ltat_dn7 = assign34120_e43297_d_n7;
        var_ltat_dn8 = assign34120_e43297_d_n8;

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
        *var_guard669_slot = var_guard669;
        *var_guard670_slot = var_guard670;
        *var_guard671_slot = var_guard671;
        *var_guard672_slot = var_guard672;
        *var_guard673_slot = var_guard673;
        *var_guard674_slot = var_guard674;
        *var_guard675_slot = var_guard675;
        *var_guard676_slot = var_guard676;
        *var_guard677_slot = var_guard677;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk219_slot = var_id__blk219;
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
    }

    pub(super) fn stamp_transient_block_70(
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti_d: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cbbtstid_i: f64,
        var_cerfc: f64,
        var_ctatstid_i: f64,
        var_fbbtsti_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard672: f64,
        var_guard676: f64,
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
        var_one_over_one_minus_psti_d: f64,
        var_pbrstid_i: f64,
        var_perfc: f64,
        var_pstid_i: f64,
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
        var_v3: f64,
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
        var_guard678_slot: &mut f64,
        var_guard679_slot: &mut f64,
        var_guard680_slot: &mut f64,
        var_guard681_slot: &mut f64,
        var_guard682_slot: &mut f64,
        var_guard683_slot: &mut f64,
        var_guard684_slot: &mut f64,
        var_guard685_slot: &mut f64,
        var_guard686_slot: &mut f64,
        var_guard687_slot: &mut f64,
        var_guard688_slot: &mut f64,
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
        let mut var_guard678: f64 = *var_guard678_slot;
        let mut var_guard679: f64 = *var_guard679_slot;
        let mut var_guard680: f64 = *var_guard680_slot;
        let mut var_guard681: f64 = *var_guard681_slot;
        let mut var_guard682: f64 = *var_guard682_slot;
        let mut var_guard683: f64 = *var_guard683_slot;
        let mut var_guard684: f64 = *var_guard684_slot;
        let mut var_guard685: f64 = *var_guard685_slot;
        let mut var_guard686: f64 = *var_guard686_slot;
        let mut var_guard687: f64 = *var_guard687_slot;
        let mut var_guard688: f64 = *var_guard688_slot;
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

        let (assign34130_e43323, assign34130_e43323_d_n5, assign34130_e43323_d_n6, assign34130_e43323_d_n7, assign34130_e43323_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34130_e43309: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign34130_e43311: f64 = (assign34130_e43309 * var_sqrtumax);
        let assign34130_e43314: f64 = (var_atatsti_d * var_umax);
        let assign34130_e43315: f64 = (assign34130_e43311 - assign34130_e43314);
        let assign34130_e43319: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34130_e43320: f64 = (0.5 * assign34130_e43319);
        let assign34130_e43321: f64 = (assign34130_e43315 + assign34130_e43320);
        (assign34130_e43321, (((((var_atatsti_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign34130_e43309 * var_sqrtumax_dn5)) - (var_atatsti_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign34130_e43309 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign34130_e43309 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign34130_e43309 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign34130_e43323;
        var_mtat_dn5 = assign34130_e43323_d_n5;
        var_mtat_dn6 = assign34130_e43323_d_n6;
        var_mtat_dn7 = assign34130_e43323_d_n7;
        var_mtat_dn8 = assign34130_e43323_d_n8;

        let (assign34140_e43339, assign34140_e43339_d_n5, assign34140_e43339_d_n6, assign34140_e43339_d_n7, assign34140_e43339_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34140_e43335: f64 = (var_ltat - 1.0);
        let assign34140_e43337: f64 = (assign34140_e43335 * var_ktat);
        (assign34140_e43337, ((var_ltat_dn5 * var_ktat) + (assign34140_e43335 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign34140_e43335 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign34140_e43335 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign34140_e43335 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign34140_e43339;
        var_xerfc_dn5 = assign34140_e43339_d_n5;
        var_xerfc_dn6 = assign34140_e43339_d_n6;
        var_xerfc_dn7 = assign34140_e43339_d_n7;
        var_xerfc_dn8 = assign34140_e43339_d_n8;

        let (assign34150_e43353, assign34150_e43353_d_n5, assign34150_e43353_d_n6, assign34150_e43353_d_n7, assign34150_e43353_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34150_e43351: f64 = (var_xerfc * var_xerfc);
        (assign34150_e43351, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign34150_e43353;
        var_ysq_dn5 = assign34150_e43353_d_n5;
        var_ysq_dn6 = assign34150_e43353_d_n6;
        var_ysq_dn7 = assign34150_e43353_d_n7;
        var_ysq_dn8 = assign34150_e43353_d_n8;

        let assign34160_e43356: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard678 = assign34160_e43356;

        let (assign34170_e43376, assign34170_e43376_d_n5, assign34170_e43376_d_n6, assign34170_e43376_d_n7, assign34170_e43376_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) && (var_guard678 != 0.0)) {
        let assign34170_e43372: f64 = (var_perfc * var_xerfc);
        let assign34170_e43373: f64 = (1.0 + assign34170_e43372);
        let assign34170_e43374: f64 = (1.0 / assign34170_e43373);
        (assign34170_e43374, (-((var_perfc * var_xerfc_dn5) / (assign34170_e43373 * assign34170_e43373))), (-((var_perfc * var_xerfc_dn6) / (assign34170_e43373 * assign34170_e43373))), (-((var_perfc * var_xerfc_dn7) / (assign34170_e43373 * assign34170_e43373))), (-((var_perfc * var_xerfc_dn8) / (assign34170_e43373 * assign34170_e43373))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign34170_e43376;
        var_terfc_dn5 = assign34170_e43376_d_n5;
        var_terfc_dn6 = assign34170_e43376_d_n6;
        var_terfc_dn7 = assign34170_e43376_d_n7;
        var_terfc_dn8 = assign34170_e43376_d_n8;

        let (assign34180_e43397, assign34180_e43397_d_n5, assign34180_e43397_d_n6, assign34180_e43397_d_n7, assign34180_e43397_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) && (var_guard678 == 0.0)) {
        let assign34180_e43393: f64 = (var_perfc * var_xerfc);
        let assign34180_e43394: f64 = (1.0 - assign34180_e43393);
        let assign34180_e43395: f64 = (1.0 / assign34180_e43394);
        (assign34180_e43395, (-((-(var_perfc * var_xerfc_dn5)) / (assign34180_e43394 * assign34180_e43394))), (-((-(var_perfc * var_xerfc_dn6)) / (assign34180_e43394 * assign34180_e43394))), (-((-(var_perfc * var_xerfc_dn7)) / (assign34180_e43394 * assign34180_e43394))), (-((-(var_perfc * var_xerfc_dn8)) / (assign34180_e43394 * assign34180_e43394))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign34180_e43397;
        var_terfc_dn5 = assign34180_e43397_d_n5;
        var_terfc_dn6 = assign34180_e43397_d_n6;
        var_terfc_dn7 = assign34180_e43397_d_n7;
        var_terfc_dn8 = assign34180_e43397_d_n8;

        let assign34190_e43399: f64 = (-var_ysq);
        let assign34190_e43401: f64 = (assign34190_e43399 + var_mtat);
        let assign34190_e43403: f64 = (-230.25850929940458);
        let assign34190_e43404: f64 = if assign34190_e43401 > assign34190_e43403 { 1.0 } else { 0.0 };
        var_guard679 = assign34190_e43404;

        let (assign34200_e43422, assign34200_e43422_d_n5, assign34200_e43422_d_n6, assign34200_e43422_d_n7, assign34200_e43422_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) && (var_guard679 != 0.0)) {
        let assign34200_e43417: f64 = (-var_ysq);
        let assign34200_e43419: f64 = (assign34200_e43417 + var_mtat);
        let assign34200_e43420: f64 = (assign34200_e43419).exp();
        (assign34200_e43420, (assign34200_e43420 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign34200_e43420 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign34200_e43420 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign34200_e43420 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34200_e43422;
        var_tmp_dn5 = assign34200_e43422_d_n5;
        var_tmp_dn6 = assign34200_e43422_d_n6;
        var_tmp_dn7 = assign34200_e43422_d_n7;
        var_tmp_dn8 = assign34200_e43422_d_n8;

        let (assign34210_e43471, assign34210_e43471_d_n5, assign34210_e43471_d_n6, assign34210_e43471_d_n7, assign34210_e43471_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) && (var_guard679 == 0.0)) {
        let assign34210_e43438: f64 = (-230.25850929940458);
        let assign34210_e43440: f64 = (-var_ysq);
        let assign34210_e43442: f64 = (assign34210_e43440 + var_mtat);
        let assign34210_e43443: f64 = (assign34210_e43438 - assign34210_e43442);
        let assign34210_e43447: f64 = (-230.25850929940458);
        let assign34210_e43449: f64 = (-var_ysq);
        let assign34210_e43451: f64 = (assign34210_e43449 + var_mtat);
        let assign34210_e43452: f64 = (assign34210_e43447 - assign34210_e43451);
        let assign34210_e43455: f64 = (-230.25850929940458);
        let assign34210_e43457: f64 = (-var_ysq);
        let assign34210_e43459: f64 = (assign34210_e43457 + var_mtat);
        let assign34210_e43460: f64 = (assign34210_e43455 - assign34210_e43459);
        let assign34210_e43462: f64 = (assign34210_e43460 * 0.3333333333333333);
        let assign34210_e43463: f64 = (1.0 + assign34210_e43462);
        let assign34210_e43464: f64 = (assign34210_e43452 * assign34210_e43463);
        let assign34210_e43465: f64 = (0.5 * assign34210_e43464);
        let assign34210_e43466: f64 = (1.0 + assign34210_e43465);
        let assign34210_e43467: f64 = (assign34210_e43443 * assign34210_e43466);
        let assign34210_e43468: f64 = (1.0 + assign34210_e43467);
        let assign34210_e43469: f64 = (1e-100 / assign34210_e43468);
        (assign34210_e43469, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign34210_e43466) + (assign34210_e43443 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign34210_e43463) + (assign34210_e43452 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign34210_e43468 * assign34210_e43468))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign34210_e43466) + (assign34210_e43443 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign34210_e43463) + (assign34210_e43452 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign34210_e43468 * assign34210_e43468))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign34210_e43466) + (assign34210_e43443 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign34210_e43463) + (assign34210_e43452 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign34210_e43468 * assign34210_e43468))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign34210_e43466) + (assign34210_e43443 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign34210_e43463) + (assign34210_e43452 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign34210_e43468 * assign34210_e43468))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34210_e43471;
        var_tmp_dn5 = assign34210_e43471_d_n5;
        var_tmp_dn6 = assign34210_e43471_d_n6;
        var_tmp_dn7 = assign34210_e43471_d_n7;
        var_tmp_dn8 = assign34210_e43471_d_n8;

        let (assign34220_e43501, assign34220_e43501_d_n5, assign34220_e43501_d_n6, assign34220_e43501_d_n7, assign34220_e43501_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34220_e43483: f64 = (0.29214664 * var_terfc);
        let assign34220_e43487: f64 = (var_terfc * var_terfc);
        let assign34220_e43488: f64 = (var_berfc * assign34220_e43487);
        let assign34220_e43489: f64 = (assign34220_e43483 + assign34220_e43488);
        let assign34220_e43493: f64 = (var_terfc * var_terfc);
        let assign34220_e43495: f64 = (assign34220_e43493 * var_terfc);
        let assign34220_e43496: f64 = (var_cerfc * assign34220_e43495);
        let assign34220_e43497: f64 = (assign34220_e43489 + assign34220_e43496);
        let assign34220_e43499: f64 = (assign34220_e43497 * var_tmp);
        (assign34220_e43499, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign34220_e43493 * var_terfc_dn5)))) * var_tmp) + (assign34220_e43497 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign34220_e43493 * var_terfc_dn6)))) * var_tmp) + (assign34220_e43497 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign34220_e43493 * var_terfc_dn7)))) * var_tmp) + (assign34220_e43497 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign34220_e43493 * var_terfc_dn8)))) * var_tmp) + (assign34220_e43497 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign34220_e43501;
        var_erfcpos_dn5 = assign34220_e43501_d_n5;
        var_erfcpos_dn6 = assign34220_e43501_d_n6;
        var_erfcpos_dn7 = assign34220_e43501_d_n7;
        var_erfcpos_dn8 = assign34220_e43501_d_n8;

        let assign34230_e43504: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard680 = assign34230_e43504;

        let (assign34240_e43518, assign34240_e43518_d_n5, assign34240_e43518_d_n6, assign34240_e43518_d_n7, assign34240_e43518_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) && (var_guard680 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign34240_e43518;
        var_erfctimesexpmtat_dn5 = assign34240_e43518_d_n5;
        var_erfctimesexpmtat_dn6 = assign34240_e43518_d_n6;
        var_erfctimesexpmtat_dn7 = assign34240_e43518_d_n7;
        var_erfctimesexpmtat_dn8 = assign34240_e43518_d_n8;

        let assign34250_e43521: f64 = (-230.25850929940458);
        let assign34250_e43522: f64 = if var_mtat > assign34250_e43521 { 1.0 } else { 0.0 };
        var_guard681 = assign34250_e43522;

        let (assign34260_e43540, assign34260_e43540_d_n5, assign34260_e43540_d_n6, assign34260_e43540_d_n7, assign34260_e43540_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) && (var_guard680 == 0.0)) && (var_guard681 != 0.0)) {
        let assign34260_e43538: f64 = (var_mtat).exp();
        (assign34260_e43538, (assign34260_e43538 * var_mtat_dn5), (assign34260_e43538 * var_mtat_dn6), (assign34260_e43538 * var_mtat_dn7), (assign34260_e43538 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34260_e43540;
        var_tmp_dn5 = assign34260_e43540_d_n5;
        var_tmp_dn6 = assign34260_e43540_d_n6;
        var_tmp_dn7 = assign34260_e43540_d_n7;
        var_tmp_dn8 = assign34260_e43540_d_n8;

        let (assign34270_e43583, assign34270_e43583_d_n5, assign34270_e43583_d_n6, assign34270_e43583_d_n7, assign34270_e43583_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) && (var_guard680 == 0.0)) && (var_guard681 == 0.0)) {
        let assign34270_e43559: f64 = (-230.25850929940458);
        let assign34270_e43561: f64 = (assign34270_e43559 - var_mtat);
        let assign34270_e43565: f64 = (-230.25850929940458);
        let assign34270_e43567: f64 = (assign34270_e43565 - var_mtat);
        let assign34270_e43570: f64 = (-230.25850929940458);
        let assign34270_e43572: f64 = (assign34270_e43570 - var_mtat);
        let assign34270_e43574: f64 = (assign34270_e43572 * 0.3333333333333333);
        let assign34270_e43575: f64 = (1.0 + assign34270_e43574);
        let assign34270_e43576: f64 = (assign34270_e43567 * assign34270_e43575);
        let assign34270_e43577: f64 = (0.5 * assign34270_e43576);
        let assign34270_e43578: f64 = (1.0 + assign34270_e43577);
        let assign34270_e43579: f64 = (assign34270_e43561 * assign34270_e43578);
        let assign34270_e43580: f64 = (1.0 + assign34270_e43579);
        let assign34270_e43581: f64 = (1e-100 / assign34270_e43580);
        (assign34270_e43581, (-((1e-100 * (((-var_mtat_dn5) * assign34270_e43578) + (assign34270_e43561 * (0.5 * (((-var_mtat_dn5) * assign34270_e43575) + (assign34270_e43567 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign34270_e43580 * assign34270_e43580))), (-((1e-100 * (((-var_mtat_dn6) * assign34270_e43578) + (assign34270_e43561 * (0.5 * (((-var_mtat_dn6) * assign34270_e43575) + (assign34270_e43567 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign34270_e43580 * assign34270_e43580))), (-((1e-100 * (((-var_mtat_dn7) * assign34270_e43578) + (assign34270_e43561 * (0.5 * (((-var_mtat_dn7) * assign34270_e43575) + (assign34270_e43567 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign34270_e43580 * assign34270_e43580))), (-((1e-100 * (((-var_mtat_dn8) * assign34270_e43578) + (assign34270_e43561 * (0.5 * (((-var_mtat_dn8) * assign34270_e43575) + (assign34270_e43567 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign34270_e43580 * assign34270_e43580))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34270_e43583;
        var_tmp_dn5 = assign34270_e43583_d_n5;
        var_tmp_dn6 = assign34270_e43583_d_n6;
        var_tmp_dn7 = assign34270_e43583_d_n7;
        var_tmp_dn8 = assign34270_e43583_d_n8;

        let (assign34280_e43602, assign34280_e43602_d_n5, assign34280_e43602_d_n6, assign34280_e43602_d_n7, assign34280_e43602_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) && (var_guard680 == 0.0)) {
        let assign34280_e43598: f64 = (2.0 * var_tmp);
        let assign34280_e43600: f64 = (assign34280_e43598 - var_erfcpos);
        (assign34280_e43600, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign34280_e43602;
        var_erfctimesexpmtat_dn5 = assign34280_e43602_d_n5;
        var_erfctimesexpmtat_dn6 = assign34280_e43602_d_n6;
        var_erfctimesexpmtat_dn7 = assign34280_e43602_d_n7;
        var_erfctimesexpmtat_dn8 = assign34280_e43602_d_n8;

        let (assign34290_e43622, assign34290_e43622_d_n5, assign34290_e43622_d_n6, assign34290_e43622_d_n7, assign34290_e43622_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34290_e43614: f64 = (1.772453850905516 * 0.5);
        let assign34290_e43617: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign34290_e43619: f64 = (assign34290_e43617 / var_ktat);
        let assign34290_e43620: f64 = (assign34290_e43614 * assign34290_e43619);
        (assign34290_e43620, (assign34290_e43614 * ((((var_atatsti_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign34290_e43617 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign34290_e43614 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign34290_e43617 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign34290_e43614 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign34290_e43617 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign34290_e43614 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign34290_e43617 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign34290_e43622;
        var_gammamax_dn5 = assign34290_e43622_d_n5;
        var_gammamax_dn6 = assign34290_e43622_d_n6;
        var_gammamax_dn7 = assign34290_e43622_d_n7;
        var_gammamax_dn8 = assign34290_e43622_d_n8;

        let (assign34300_e43640, assign34300_e43640_d_n5, assign34300_e43640_d_n6, assign34300_e43640_d_n7, assign34300_e43640_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard676 == 0.0)) {
        let assign34300_e43635: f64 = (var_asrh * var_gammamax);
        let assign34300_e43637: f64 = (assign34300_e43635 * var_wtat);
        let assign34300_e43638: f64 = (var_ctatstid_i * assign34300_e43637);
        (assign34300_e43638, (var_ctatstid_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign34300_e43635 * var_wtat_dn5))), (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign34300_e43635 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign34300_e43635 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign34300_e43635 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign34300_e43640;
        var_itat_dn5 = assign34300_e43640_d_n5;
        var_itat_dn6 = assign34300_e43640_d_n6;
        var_itat_dn7 = assign34300_e43640_d_n7;
        var_itat_dn8 = assign34300_e43640_d_n8;

        let assign34310_e43643: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard682 = assign34310_e43643;

        let (assign34320_e43654, assign34320_e43654_d_n5, assign34320_e43654_d_n6, assign34320_e43654_d_n7, assign34320_e43654_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard682 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign34320_e43654;
        var_ibbt_dn5 = assign34320_e43654_d_n5;
        var_ibbt_dn6 = assign34320_e43654_d_n6;
        var_ibbt_dn7 = assign34320_e43654_d_n7;
        var_ibbt_dn8 = assign34320_e43654_d_n8;

        let assign34330_e43657: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard683 = assign34330_e43657;

        let (assign34340_e43676, assign34340_e43676_d_n5, assign34340_e43676_d_n6, assign34340_e43676_d_n7, assign34340_e43676_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard682 == 0.0)) && (var_guard683 != 0.0)) {
        let assign34340_e43671: f64 = (var_vbirstid_i - var_vbbt);
        let assign34340_e43673: f64 = (assign34340_e43671 * var_vbirstiinv_d);
        let assign34340_e43674: f64 = (assign34340_e43673).sqrt();
        (assign34340_e43674, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34340_e43676;
        var_tmp_dn5 = assign34340_e43676_d_n5;
        var_tmp_dn6 = assign34340_e43676_d_n6;
        var_tmp_dn7 = assign34340_e43676_d_n7;
        var_tmp_dn8 = assign34340_e43676_d_n8;

        let (assign34350_e43697, assign34350_e43697_d_n5, assign34350_e43697_d_n6, assign34350_e43697_d_n7, assign34350_e43697_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) {
        let assign34350_e43691: f64 = (var_vbirstid_i - var_vbbt);
        let assign34350_e43693: f64 = (assign34350_e43691 * var_vbirstiinv_d);
        let assign34350_e43695: f64 = (assign34350_e43693).powf(var_pstid_i);
        (assign34350_e43695, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34350_e43697;
        var_tmp_dn5 = assign34350_e43697_d_n5;
        var_tmp_dn6 = assign34350_e43697_d_n6;
        var_tmp_dn7 = assign34350_e43697_d_n7;
        var_tmp_dn8 = assign34350_e43697_d_n8;

        let (assign34360_e43717, assign34360_e43717_d_n5, assign34360_e43717_d_n6, assign34360_e43717_d_n7, assign34360_e43717_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard682 == 0.0)) {
        let assign34360_e43710: f64 = (var_vbirstid_i - var_vbbt);
        let assign34360_e43712: f64 = (assign34360_e43710 * var_wdepnulrinvsti_d);
        let assign34360_e43714: f64 = (assign34360_e43712 / var_tmp);
        let assign34360_e43715: f64 = (var_one_over_one_minus_psti_d * assign34360_e43714);
        (assign34360_e43715, (var_one_over_one_minus_psti_d * (-((assign34360_e43712 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign34360_e43712 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign34360_e43712 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign34360_e43712 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign34360_e43717;
        var_fmaxr_dn5 = assign34360_e43717_d_n5;
        var_fmaxr_dn6 = assign34360_e43717_d_n6;
        var_fmaxr_dn7 = assign34360_e43717_d_n7;
        var_fmaxr_dn8 = assign34360_e43717_d_n8;

        let assign34370_e43719: f64 = (-var_fbbtsti_d);
        let assign34370_e43721: f64 = (assign34370_e43719 / var_fmaxr);
        let assign34370_e43722: f64 = (assign34370_e43721).abs();
        let assign34370_e43724: f64 = if assign34370_e43722 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard684 = assign34370_e43724;

        let (assign34380_e43742, assign34380_e43742_d_n5, assign34380_e43742_d_n6, assign34380_e43742_d_n7, assign34380_e43742_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard682 == 0.0)) && (var_guard684 != 0.0)) {
        let assign34380_e43737: f64 = (-var_fbbtsti_d);
        let assign34380_e43739: f64 = (assign34380_e43737 / var_fmaxr);
        let assign34380_e43740: f64 = (assign34380_e43739).exp();
        (assign34380_e43740, (assign34380_e43740 * (-((assign34380_e43737 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign34380_e43740 * (-((assign34380_e43737 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign34380_e43740 * (-((assign34380_e43737 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign34380_e43740 * (-((assign34380_e43737 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34380_e43742;
        var_tmp_dn5 = assign34380_e43742_d_n5;
        var_tmp_dn6 = assign34380_e43742_d_n6;
        var_tmp_dn7 = assign34380_e43742_d_n7;
        var_tmp_dn8 = assign34380_e43742_d_n8;

        let assign34390_e43744: f64 = (-var_fbbtsti_d);
        let assign34390_e43746: f64 = (assign34390_e43744 / var_fmaxr);
        let assign34390_e43748: f64 = if assign34390_e43746 < 0.0 { 1.0 } else { 0.0 };
        var_guard685 = assign34390_e43748;

        let (assign34400_e43799, assign34400_e43799_d_n5, assign34400_e43799_d_n6, assign34400_e43799_d_n7, assign34400_e43799_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard682 == 0.0)) && (var_guard684 == 0.0)) && (var_guard685 != 0.0)) {
        let assign34400_e43766: f64 = (-230.25850929940458);
        let assign34400_e43768: f64 = (-var_fbbtsti_d);
        let assign34400_e43770: f64 = (assign34400_e43768 / var_fmaxr);
        let assign34400_e43771: f64 = (assign34400_e43766 - assign34400_e43770);
        let assign34400_e43775: f64 = (-230.25850929940458);
        let assign34400_e43777: f64 = (-var_fbbtsti_d);
        let assign34400_e43779: f64 = (assign34400_e43777 / var_fmaxr);
        let assign34400_e43780: f64 = (assign34400_e43775 - assign34400_e43779);
        let assign34400_e43783: f64 = (-230.25850929940458);
        let assign34400_e43785: f64 = (-var_fbbtsti_d);
        let assign34400_e43787: f64 = (assign34400_e43785 / var_fmaxr);
        let assign34400_e43788: f64 = (assign34400_e43783 - assign34400_e43787);
        let assign34400_e43790: f64 = (assign34400_e43788 * 0.3333333333333333);
        let assign34400_e43791: f64 = (1.0 + assign34400_e43790);
        let assign34400_e43792: f64 = (assign34400_e43780 * assign34400_e43791);
        let assign34400_e43793: f64 = (0.5 * assign34400_e43792);
        let assign34400_e43794: f64 = (1.0 + assign34400_e43793);
        let assign34400_e43795: f64 = (assign34400_e43771 * assign34400_e43794);
        let assign34400_e43796: f64 = (1.0 + assign34400_e43795);
        let assign34400_e43797: f64 = (1e-100 / assign34400_e43796);
        (assign34400_e43797, (-((1e-100 * (((-(-((assign34400_e43768 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign34400_e43794) + (assign34400_e43771 * (0.5 * (((-(-((assign34400_e43777 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign34400_e43791) + (assign34400_e43780 * ((-(-((assign34400_e43785 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign34400_e43796 * assign34400_e43796))), (-((1e-100 * (((-(-((assign34400_e43768 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign34400_e43794) + (assign34400_e43771 * (0.5 * (((-(-((assign34400_e43777 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign34400_e43791) + (assign34400_e43780 * ((-(-((assign34400_e43785 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign34400_e43796 * assign34400_e43796))), (-((1e-100 * (((-(-((assign34400_e43768 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign34400_e43794) + (assign34400_e43771 * (0.5 * (((-(-((assign34400_e43777 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign34400_e43791) + (assign34400_e43780 * ((-(-((assign34400_e43785 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign34400_e43796 * assign34400_e43796))), (-((1e-100 * (((-(-((assign34400_e43768 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign34400_e43794) + (assign34400_e43771 * (0.5 * (((-(-((assign34400_e43777 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign34400_e43791) + (assign34400_e43780 * ((-(-((assign34400_e43785 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign34400_e43796 * assign34400_e43796))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34400_e43799;
        var_tmp_dn5 = assign34400_e43799_d_n5;
        var_tmp_dn6 = assign34400_e43799_d_n6;
        var_tmp_dn7 = assign34400_e43799_d_n7;
        var_tmp_dn8 = assign34400_e43799_d_n8;

        let (assign34410_e43848, assign34410_e43848_d_n5, assign34410_e43848_d_n6, assign34410_e43848_d_n7, assign34410_e43848_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard682 == 0.0)) && (var_guard684 == 0.0)) && (var_guard685 == 0.0)) {
        let assign34410_e43818: f64 = (-var_fbbtsti_d);
        let assign34410_e43820: f64 = (assign34410_e43818 / var_fmaxr);
        let assign34410_e43822: f64 = (assign34410_e43820 - 230.25850929940458);
        let assign34410_e43826: f64 = (-var_fbbtsti_d);
        let assign34410_e43828: f64 = (assign34410_e43826 / var_fmaxr);
        let assign34410_e43830: f64 = (assign34410_e43828 - 230.25850929940458);
        let assign34410_e43833: f64 = (-var_fbbtsti_d);
        let assign34410_e43835: f64 = (assign34410_e43833 / var_fmaxr);
        let assign34410_e43837: f64 = (assign34410_e43835 - 230.25850929940458);
        let assign34410_e43839: f64 = (assign34410_e43837 * 0.3333333333333333);
        let assign34410_e43840: f64 = (1.0 + assign34410_e43839);
        let assign34410_e43841: f64 = (assign34410_e43830 * assign34410_e43840);
        let assign34410_e43842: f64 = (0.5 * assign34410_e43841);
        let assign34410_e43843: f64 = (1.0 + assign34410_e43842);
        let assign34410_e43844: f64 = (assign34410_e43822 * assign34410_e43843);
        let assign34410_e43845: f64 = (1.0 + assign34410_e43844);
        let assign34410_e43846: f64 = (1e100 * assign34410_e43845);
        (assign34410_e43846, (1e100 * (((-((assign34410_e43818 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign34410_e43843) + (assign34410_e43822 * (0.5 * (((-((assign34410_e43826 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign34410_e43840) + (assign34410_e43830 * ((-((assign34410_e43833 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign34410_e43818 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign34410_e43843) + (assign34410_e43822 * (0.5 * (((-((assign34410_e43826 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign34410_e43840) + (assign34410_e43830 * ((-((assign34410_e43833 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign34410_e43818 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign34410_e43843) + (assign34410_e43822 * (0.5 * (((-((assign34410_e43826 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign34410_e43840) + (assign34410_e43830 * ((-((assign34410_e43833 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign34410_e43818 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign34410_e43843) + (assign34410_e43822 * (0.5 * (((-((assign34410_e43826 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign34410_e43840) + (assign34410_e43830 * ((-((assign34410_e43833 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34410_e43848;
        var_tmp_dn5 = assign34410_e43848_d_n5;
        var_tmp_dn6 = assign34410_e43848_d_n6;
        var_tmp_dn7 = assign34410_e43848_d_n7;
        var_tmp_dn8 = assign34410_e43848_d_n8;

        let (assign34420_e43868, assign34420_e43868_d_n5, assign34420_e43868_d_n6, assign34420_e43868_d_n7, assign34420_e43868_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard682 == 0.0)) {
        let assign34420_e43861: f64 = (var_v3 * var_fmaxr);
        let assign34420_e43863: f64 = (assign34420_e43861 * var_fmaxr);
        let assign34420_e43865: f64 = (assign34420_e43863 * var_tmp);
        let assign34420_e43866: f64 = (var_cbbtstid_i * assign34420_e43865);
        (assign34420_e43866, (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign34420_e43861 * var_fmaxr_dn5)) * var_tmp) + (assign34420_e43863 * var_tmp_dn5))), (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign34420_e43861 * var_fmaxr_dn6)) * var_tmp) + (assign34420_e43863 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign34420_e43861 * var_fmaxr_dn7)) * var_tmp) + (assign34420_e43863 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign34420_e43861 * var_fmaxr_dn8)) * var_tmp) + (assign34420_e43863 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign34420_e43868;
        var_ibbt_dn5 = assign34420_e43868_d_n5;
        var_ibbt_dn6 = assign34420_e43868_d_n6;
        var_ibbt_dn7 = assign34420_e43868_d_n7;
        var_ibbt_dn8 = assign34420_e43868_d_n8;

        let assign34430_e43871: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard686 = assign34430_e43871;

        let (assign34440_e43882, assign34440_e43882_d_n5, assign34440_e43882_d_n6, assign34440_e43882_d_n7, assign34440_e43882_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard686 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign34440_e43882;
        var_fbreakdown_dn5 = assign34440_e43882_d_n5;
        var_fbreakdown_dn6 = assign34440_e43882_d_n6;
        var_fbreakdown_dn7 = assign34440_e43882_d_n7;
        var_fbreakdown_dn8 = assign34440_e43882_d_n8;

        let assign34450_e43885: f64 = (-var_alphaav);
        let assign34450_e43887: f64 = (assign34450_e43885 * var_vbrstid_i);
        let assign34450_e43888: f64 = if var_vav > assign34450_e43887 { 1.0 } else { 0.0 };
        var_guard687 = assign34450_e43888;

        let assign34460_e43891: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard688 = assign34460_e43891;

        let (assign34470_e43921, assign34470_e43921_d_n5, assign34470_e43921_d_n6, assign34470_e43921_d_n7, assign34470_e43921_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard686 == 0.0)) && (var_guard687 != 0.0)) && (var_guard688 != 0.0)) {
        let assign34470_e43907: f64 = (var_vav * var_vbrinvsti_d);
        let assign34470_e43910: f64 = (var_vav * var_vbrinvsti_d);
        let assign34470_e43911: f64 = (assign34470_e43907 * assign34470_e43910);
        let assign34470_e43914: f64 = (var_vav * var_vbrinvsti_d);
        let assign34470_e43915: f64 = (assign34470_e43911 * assign34470_e43914);
        let assign34470_e43918: f64 = (var_vav * var_vbrinvsti_d);
        let assign34470_e43919: f64 = (assign34470_e43915 * assign34470_e43918);
        (assign34470_e43919, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34470_e43921;
        var_tmp_dn5 = assign34470_e43921_d_n5;
        var_tmp_dn6 = assign34470_e43921_d_n6;
        var_tmp_dn7 = assign34470_e43921_d_n7;
        var_tmp_dn8 = assign34470_e43921_d_n8;

        let (assign34480_e43943, assign34480_e43943_d_n5, assign34480_e43943_d_n6, assign34480_e43943_d_n7, assign34480_e43943_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard686 == 0.0)) && (var_guard687 != 0.0)) && (var_guard688 == 0.0)) {
        let assign34480_e43938: f64 = (var_vav * var_vbrinvsti_d);
        let assign34480_e43939: f64 = (assign34480_e43938).abs();
        let assign34480_e43941: f64 = (assign34480_e43939).powf(var_pbrstid_i);
        (assign34480_e43941, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34480_e43943;
        var_tmp_dn5 = assign34480_e43943_d_n5;
        var_tmp_dn6 = assign34480_e43943_d_n6;
        var_tmp_dn7 = assign34480_e43943_d_n7;
        var_tmp_dn8 = assign34480_e43943_d_n8;

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
        *var_guard678_slot = var_guard678;
        *var_guard679_slot = var_guard679;
        *var_guard680_slot = var_guard680;
        *var_guard681_slot = var_guard681;
        *var_guard682_slot = var_guard682;
        *var_guard683_slot = var_guard683;
        *var_guard684_slot = var_guard684;
        *var_guard685_slot = var_guard685;
        *var_guard686_slot = var_guard686;
        *var_guard687_slot = var_guard687;
        *var_guard688_slot = var_guard688;
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

    pub(super) fn stamp_transient_block_71(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat_d: f64,
        var_btatpartgat_d: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_fstopsti_d: f64,
        var_ftdgat_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard672: f64,
        var_guard686: f64,
        var_guard687: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idmult: f64,
        var_idsatgat_d: f64,
        var_lgdrain_i: f64,
        var_one_minus_pgat_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
        var_slopesti_d: f64,
        var_two_psistar: f64,
        var_vav: f64,
        var_vbigat_d: f64,
        var_vbirgatinv_d: f64,
        var_vbrstid_i: f64,
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
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard689_slot: &mut f64,
        var_guard690_slot: &mut f64,
        var_guard691_slot: &mut f64,
        var_guard692_slot: &mut f64,
        var_guard693_slot: &mut f64,
        var_guard694_slot: &mut f64,
        var_guard695_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard689: f64 = *var_guard689_slot;
        let mut var_guard690: f64 = *var_guard690_slot;
        let mut var_guard691: f64 = *var_guard691_slot;
        let mut var_guard692: f64 = *var_guard692_slot;
        let mut var_guard693: f64 = *var_guard693_slot;
        let mut var_guard694: f64 = *var_guard694_slot;
        let mut var_guard695: f64 = *var_guard695_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let (assign34490_e43961, assign34490_e43961_d_n5, assign34490_e43961_d_n6, assign34490_e43961_d_n7, assign34490_e43961_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard686 == 0.0)) && (var_guard687 != 0.0)) {
        let assign34490_e43958: f64 = (1.0 - var_tmp);
        let assign34490_e43959: f64 = (1.0 / assign34490_e43958);
        (assign34490_e43959, (-((-var_tmp_dn5) / (assign34490_e43958 * assign34490_e43958))), (-((-var_tmp_dn6) / (assign34490_e43958 * assign34490_e43958))), (-((-var_tmp_dn7) / (assign34490_e43958 * assign34490_e43958))), (-((-var_tmp_dn8) / (assign34490_e43958 * assign34490_e43958))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign34490_e43961;
        var_fbreakdown_dn5 = assign34490_e43961_d_n5;
        var_fbreakdown_dn6 = assign34490_e43961_d_n6;
        var_fbreakdown_dn7 = assign34490_e43961_d_n7;
        var_fbreakdown_dn8 = assign34490_e43961_d_n8;

        let (assign34500_e43984, assign34500_e43984_d_n5, assign34500_e43984_d_n6, assign34500_e43984_d_n7, assign34500_e43984_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) && (var_guard686 == 0.0)) && (var_guard687 == 0.0)) {
        let assign34500_e43978: f64 = (var_alphaav * var_vbrstid_i);
        let assign34500_e43979: f64 = (var_vav + assign34500_e43978);
        let assign34500_e43981: f64 = (assign34500_e43979 * var_slopesti_d);
        let assign34500_e43982: f64 = (var_fstopsti_d + assign34500_e43981);
        (assign34500_e43982, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign34500_e43984;
        var_fbreakdown_dn5 = assign34500_e43984_d_n5;
        var_fbreakdown_dn6 = assign34500_e43984_d_n6;
        var_fbreakdown_dn7 = assign34500_e43984_d_n7;
        var_fbreakdown_dn8 = assign34500_e43984_d_n8;

        let (assign34510_e44003, assign34510_e44003_d_n5, assign34510_e44003_d_n6, assign34510_e44003_d_n7, assign34510_e44003_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard672 == 0.0)) {
        let assign34510_e43994: f64 = (var_id__blk219 + var_isrh);
        let assign34510_e43996: f64 = (assign34510_e43994 + var_itat);
        let assign34510_e43998: f64 = (assign34510_e43996 + var_ibbt);
        let assign34510_e43999: f64 = (p.p29 * assign34510_e43998);
        let assign34510_e44001: f64 = (assign34510_e43999 * var_fbreakdown);
        (assign34510_e44001, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign34510_e43999 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign34510_e43999 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign34510_e43999 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign34510_e43999 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign34510_e44003;
        var_ijunsti_dn5 = assign34510_e44003_d_n5;
        var_ijunsti_dn6 = assign34510_e44003_d_n6;
        var_ijunsti_dn7 = assign34510_e44003_d_n7;
        var_ijunsti_dn8 = assign34510_e44003_d_n8;

        let assign34520_e44006: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard689 = assign34520_e44006;

        let (assign34530_e44014, assign34530_e44014_d_n5, assign34530_e44014_d_n6, assign34530_e44014_d_n7, assign34530_e44014_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign34530_e44014;
        var_ijungat_dn5 = assign34530_e44014_d_n5;
        var_ijungat_dn6 = assign34530_e44014_d_n6;
        var_ijungat_dn7 = assign34530_e44014_d_n7;
        var_ijungat_dn8 = assign34530_e44014_d_n8;

        let (assign34540_e44025,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) {
        let assign34540_e44023: f64 = (var_idsatgat_d * var_idmult);
        (assign34540_e44023,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign34540_e44025;

        let assign34550_e44032: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard690 = assign34550_e44032;

        let (assign34560_e44043, assign34560_e44043_d_n5, assign34560_e44043_d_n6, assign34560_e44043_d_n7, assign34560_e44043_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign34560_e44043;
        var_isrh_dn5 = assign34560_e44043_d_n5;
        var_isrh_dn6 = assign34560_e44043_d_n6;
        var_isrh_dn7 = assign34560_e44043_d_n7;
        var_isrh_dn8 = assign34560_e44043_d_n8;

        let (assign34570_e44057,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 == 0.0)) {
        let assign34570_e44055: f64 = (var_vbigat_d - var_vjsrh);
        (assign34570_e44055,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign34570_e44057;

        let (assign34580_e44076,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 == 0.0)) {
        let assign34580_e44071: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign34580_e44072: f64 = (1.0 - assign34580_e44071);
        let assign34580_e44073: f64 = (assign34580_e44072).sqrt();
        let assign34580_e44074: f64 = (1.0 - assign34580_e44073);
        (assign34580_e44074,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign34580_e44076;

        let assign34590_e44079: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard691 = assign34590_e44079;

        let (assign34600_e44093,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 == 0.0)) && (var_guard691 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign34600_e44093;

        let (assign34610_e44125,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 == 0.0)) && (var_guard691 == 0.0)) {
        let assign34610_e44108: f64 = (var_wsrhstep * var_wsrhstep);
        let assign34610_e44110: f64 = (var_wsrhstep).ln();
        let assign34610_e44111: f64 = (assign34610_e44108 * assign34610_e44110);
        let assign34610_e44114: f64 = (1.0 - var_wsrhstep);
        let assign34610_e44115: f64 = (assign34610_e44111 / assign34610_e44114);
        let assign34610_e44117: f64 = (assign34610_e44115 + var_wsrhstep);
        let assign34610_e44121: f64 = (2.0 * var_pgatd_i);
        let assign34610_e44122: f64 = (1.0 - assign34610_e44121);
        let assign34610_e44123: f64 = (assign34610_e44117 * assign34610_e44122);
        (assign34610_e44123,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign34610_e44125;

        let (assign34620_e44139,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 == 0.0)) {
        let assign34620_e44137: f64 = (var_wsrhstep + var_dwsrh);
        (assign34620_e44137,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign34620_e44139;

        let assign34630_e44142: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard692 = assign34630_e44142;

        let (assign34640_e44159, assign34640_e44159_d_n5, assign34640_e44159_d_n6, assign34640_e44159_d_n7, assign34640_e44159_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 == 0.0)) && (var_guard692 != 0.0)) {
        let assign34640_e44156: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign34640_e44157: f64 = (assign34640_e44156).sqrt();
        (assign34640_e44157, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34640_e44159;
        var_tmp_dn5 = assign34640_e44159_d_n5;
        var_tmp_dn6 = assign34640_e44159_d_n6;
        var_tmp_dn7 = assign34640_e44159_d_n7;
        var_tmp_dn8 = assign34640_e44159_d_n8;

        let (assign34650_e44178, assign34650_e44178_d_n5, assign34650_e44178_d_n6, assign34650_e44178_d_n7, assign34650_e44178_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 == 0.0)) && (var_guard692 == 0.0)) {
        let assign34650_e44174: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign34650_e44176: f64 = (assign34650_e44174).powf(var_pgatd_i);
        (assign34650_e44176, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34650_e44178;
        var_tmp_dn5 = assign34650_e44178_d_n5;
        var_tmp_dn6 = assign34650_e44178_d_n6;
        var_tmp_dn7 = assign34650_e44178_d_n7;
        var_tmp_dn8 = assign34650_e44178_d_n8;

        let (assign34660_e44192, assign34660_e44192_d_n5, assign34660_e44192_d_n6, assign34660_e44192_d_n7, assign34660_e44192_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 == 0.0)) {
        let assign34660_e44190: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign34660_e44190, (var_wdepnulrgat_d * var_tmp_dn5), (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign34660_e44192;
        var_wdep_dn5 = assign34660_e44192_d_n5;
        var_wdep_dn6 = assign34660_e44192_d_n6;
        var_wdep_dn7 = assign34660_e44192_d_n7;
        var_wdep_dn8 = assign34660_e44192_d_n8;

        let (assign34670_e44210, assign34670_e44210_d_n5, assign34670_e44210_d_n6, assign34670_e44210_d_n7, assign34670_e44210_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 == 0.0)) {
        let assign34670_e44205: f64 = (var_zinv - 1.0);
        let assign34670_e44207: f64 = (assign34670_e44205 * var_wdep);
        let assign34670_e44208: f64 = (var_ftdgat_d * assign34670_e44207);
        (assign34670_e44208, (var_ftdgat_d * (assign34670_e44205 * var_wdep_dn5)), (var_ftdgat_d * (assign34670_e44205 * var_wdep_dn6)), (var_ftdgat_d * (assign34670_e44205 * var_wdep_dn7)), (var_ftdgat_d * (assign34670_e44205 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign34670_e44210;
        var_asrh_dn5 = assign34670_e44210_d_n5;
        var_asrh_dn6 = assign34670_e44210_d_n6;
        var_asrh_dn7 = assign34670_e44210_d_n7;
        var_asrh_dn8 = assign34670_e44210_d_n8;

        let (assign34680_e44226, assign34680_e44226_d_n5, assign34680_e44226_d_n6, assign34680_e44226_d_n7, assign34680_e44226_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard690 == 0.0)) {
        let assign34680_e44223: f64 = (var_asrh * var_wsrh);
        let assign34680_e44224: f64 = (var_csrhgatd_i * assign34680_e44223);
        (assign34680_e44224, (var_csrhgatd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign34680_e44226;
        var_isrh_dn5 = assign34680_e44226_d_n5;
        var_isrh_dn6 = assign34680_e44226_d_n6;
        var_isrh_dn7 = assign34680_e44226_d_n7;
        var_isrh_dn8 = assign34680_e44226_d_n8;

        let assign34690_e44229: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard693 = assign34690_e44229;

        let (assign34700_e44240, assign34700_e44240_d_n5, assign34700_e44240_d_n6, assign34700_e44240_d_n7, assign34700_e44240_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign34700_e44240;
        var_itat_dn5 = assign34700_e44240_d_n5;
        var_itat_dn6 = assign34700_e44240_d_n6;
        var_itat_dn7 = assign34700_e44240_d_n7;
        var_itat_dn8 = assign34700_e44240_d_n8;

        let (assign34710_e44258, assign34710_e44258_d_n5, assign34710_e44258_d_n6, assign34710_e44258_d_n7, assign34710_e44258_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34710_e44253: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign34710_e44255: f64 = (assign34710_e44253 / var_vbi_minus_vjsrh);
        let assign34710_e44256: f64 = (var_btatpartgat_d * assign34710_e44255);
        (assign34710_e44256, (var_btatpartgat_d * ((var_wdep_dn5 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign34710_e44258;
        var_btat_dn5 = assign34710_e44258_d_n5;
        var_btat_dn6 = assign34710_e44258_d_n6;
        var_btat_dn7 = assign34710_e44258_d_n7;
        var_btat_dn8 = assign34710_e44258_d_n8;

        let (assign34720_e44274, assign34720_e44274_d_n5, assign34720_e44274_d_n6, assign34720_e44274_d_n7, assign34720_e44274_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34720_e44270: f64 = (0.666666666666667 * var_atatgat_d);
        let assign34720_e44272: f64 = (assign34720_e44270 / var_btat);
        (assign34720_e44272, (-((assign34720_e44270 * var_btat_dn5) / (var_btat * var_btat))), (-((assign34720_e44270 * var_btat_dn6) / (var_btat * var_btat))), (-((assign34720_e44270 * var_btat_dn7) / (var_btat * var_btat))), (-((assign34720_e44270 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign34720_e44274;
        var_twoatatoverthreebtat_dn5 = assign34720_e44274_d_n5;
        var_twoatatoverthreebtat_dn6 = assign34720_e44274_d_n6;
        var_twoatatoverthreebtat_dn7 = assign34720_e44274_d_n7;
        var_twoatatoverthreebtat_dn8 = assign34720_e44274_d_n8;

        let (assign34730_e44288, assign34730_e44288_d_n5, assign34730_e44288_d_n6, assign34730_e44288_d_n7, assign34730_e44288_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34730_e44286: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign34730_e44286, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign34730_e44288;
        var_umaxbeforelimiting_dn5 = assign34730_e44288_d_n5;
        var_umaxbeforelimiting_dn6 = assign34730_e44288_d_n6;
        var_umaxbeforelimiting_dn7 = assign34730_e44288_d_n7;
        var_umaxbeforelimiting_dn8 = assign34730_e44288_d_n8;

        let (assign34740_e44309, assign34740_e44309_d_n5, assign34740_e44309_d_n6, assign34740_e44309_d_n7, assign34740_e44309_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34740_e44300: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign34740_e44303: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign34740_e44305: f64 = (assign34740_e44303 + 1.0);
        let assign34740_e44306: f64 = (assign34740_e44300 / assign34740_e44305);
        let assign34740_e44307: f64 = (assign34740_e44306).sqrt();
        (assign34740_e44307, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign34740_e44305) - (assign34740_e44300 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign34740_e44305 * assign34740_e44305)) / (2.0 * assign34740_e44307)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign34740_e44305) - (assign34740_e44300 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign34740_e44305 * assign34740_e44305)) / (2.0 * assign34740_e44307)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign34740_e44305) - (assign34740_e44300 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign34740_e44305 * assign34740_e44305)) / (2.0 * assign34740_e44307)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign34740_e44305) - (assign34740_e44300 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign34740_e44305 * assign34740_e44305)) / (2.0 * assign34740_e44307)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign34740_e44309;
        var_umax_dn5 = assign34740_e44309_d_n5;
        var_umax_dn6 = assign34740_e44309_d_n6;
        var_umax_dn7 = assign34740_e44309_d_n7;
        var_umax_dn8 = assign34740_e44309_d_n8;

        let (assign34750_e44322, assign34750_e44322_d_n5, assign34750_e44322_d_n6, assign34750_e44322_d_n7, assign34750_e44322_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34750_e44320: f64 = (var_umax).sqrt();
        (assign34750_e44320, (var_umax_dn5 / (2.0 * assign34750_e44320)), (var_umax_dn6 / (2.0 * assign34750_e44320)), (var_umax_dn7 / (2.0 * assign34750_e44320)), (var_umax_dn8 / (2.0 * assign34750_e44320)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign34750_e44322;
        var_sqrtumax_dn5 = assign34750_e44322_d_n5;
        var_sqrtumax_dn6 = assign34750_e44322_d_n6;
        var_sqrtumax_dn7 = assign34750_e44322_d_n7;
        var_sqrtumax_dn8 = assign34750_e44322_d_n8;

        let (assign34760_e44336, assign34760_e44336_d_n5, assign34760_e44336_d_n6, assign34760_e44336_d_n7, assign34760_e44336_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34760_e44334: f64 = (var_umax * var_sqrtumax);
        (assign34760_e44334, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign34760_e44336;
        var_umaxpoweronepointfive_dn5 = assign34760_e44336_d_n5;
        var_umaxpoweronepointfive_dn6 = assign34760_e44336_d_n6;
        var_umaxpoweronepointfive_dn7 = assign34760_e44336_d_n7;
        var_umaxpoweronepointfive_dn8 = assign34760_e44336_d_n8;

        let assign34770_e44338: f64 = (-var_pgatd_i);
        let assign34770_e44340: f64 = (assign34770_e44338 * var_one_over_one_minus_pgat_d);
        let assign34770_e44342: f64 = (-1.0);
        let assign34770_e44343: f64 = if assign34770_e44340 == assign34770_e44342 { 1.0 } else { 0.0 };
        var_guard694 = assign34770_e44343;

        let (assign34780_e44363, assign34780_e44363_d_n5, assign34780_e44363_d_n6, assign34780_e44363_d_n7, assign34780_e44363_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) && (var_guard694 != 0.0)) {
        let assign34780_e44359: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34780_e44360: f64 = (1.0 + assign34780_e44359);
        let assign34780_e44361: f64 = (1.0 / assign34780_e44360);
        (assign34780_e44361, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign34780_e44360 * assign34780_e44360))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign34780_e44360 * assign34780_e44360))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign34780_e44360 * assign34780_e44360))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign34780_e44360 * assign34780_e44360))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign34780_e44363;
        var_wgamma_dn5 = assign34780_e44363_d_n5;
        var_wgamma_dn6 = assign34780_e44363_d_n6;
        var_wgamma_dn7 = assign34780_e44363_d_n7;
        var_wgamma_dn8 = assign34780_e44363_d_n8;

        let (assign34790_e44387, assign34790_e44387_d_n5, assign34790_e44387_d_n6, assign34790_e44387_d_n7, assign34790_e44387_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) && (var_guard694 == 0.0)) {
        let assign34790_e44379: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34790_e44380: f64 = (1.0 + assign34790_e44379);
        let assign34790_e44382: f64 = (-var_pgatd_i);
        let assign34790_e44384: f64 = (assign34790_e44382 * var_one_over_one_minus_pgat_d);
        let assign34790_e44385: f64 = (assign34790_e44380).powf(assign34790_e44384);
        (assign34790_e44385, if 0.0 == 0.0 && ((assign34790_e44384) as f64).is_finite() && ((assign34790_e44384) as f64).fract() == 0.0 { if assign34790_e44384 == 0.0 { 0.0 } else { (assign34790_e44384 * ((assign34790_e44380).powf(assign34790_e44384 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign34790_e44385 * (assign34790_e44384 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign34790_e44380))) }, if 0.0 == 0.0 && ((assign34790_e44384) as f64).is_finite() && ((assign34790_e44384) as f64).fract() == 0.0 { if assign34790_e44384 == 0.0 { 0.0 } else { (assign34790_e44384 * ((assign34790_e44380).powf(assign34790_e44384 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign34790_e44385 * (assign34790_e44384 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign34790_e44380))) }, if 0.0 == 0.0 && ((assign34790_e44384) as f64).is_finite() && ((assign34790_e44384) as f64).fract() == 0.0 { if assign34790_e44384 == 0.0 { 0.0 } else { (assign34790_e44384 * ((assign34790_e44380).powf(assign34790_e44384 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign34790_e44385 * (assign34790_e44384 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign34790_e44380))) }, if 0.0 == 0.0 && ((assign34790_e44384) as f64).is_finite() && ((assign34790_e44384) as f64).fract() == 0.0 { if assign34790_e44384 == 0.0 { 0.0 } else { (assign34790_e44384 * ((assign34790_e44380).powf(assign34790_e44384 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign34790_e44385 * (assign34790_e44384 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign34790_e44380))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign34790_e44387;
        var_wgamma_dn5 = assign34790_e44387_d_n5;
        var_wgamma_dn6 = assign34790_e44387_d_n6;
        var_wgamma_dn7 = assign34790_e44387_d_n7;
        var_wgamma_dn8 = assign34790_e44387_d_n8;

        let (assign34800_e44405, assign34800_e44405_d_n5, assign34800_e44405_d_n6, assign34800_e44405_d_n7, assign34800_e44405_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34800_e44399: f64 = (var_wsrh * var_wgamma);
        let assign34800_e44402: f64 = (var_wsrh + var_wgamma);
        let assign34800_e44403: f64 = (assign34800_e44399 / assign34800_e44402);
        (assign34800_e44403, ((((var_wsrh * var_wgamma_dn5) * assign34800_e44402) - (assign34800_e44399 * var_wgamma_dn5)) / (assign34800_e44402 * assign34800_e44402)), ((((var_wsrh * var_wgamma_dn6) * assign34800_e44402) - (assign34800_e44399 * var_wgamma_dn6)) / (assign34800_e44402 * assign34800_e44402)), ((((var_wsrh * var_wgamma_dn7) * assign34800_e44402) - (assign34800_e44399 * var_wgamma_dn7)) / (assign34800_e44402 * assign34800_e44402)), ((((var_wsrh * var_wgamma_dn8) * assign34800_e44402) - (assign34800_e44399 * var_wgamma_dn8)) / (assign34800_e44402 * assign34800_e44402)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign34800_e44405;
        var_wtat_dn5 = assign34800_e44405_d_n5;
        var_wtat_dn6 = assign34800_e44405_d_n6;
        var_wtat_dn7 = assign34800_e44405_d_n7;
        var_wtat_dn8 = assign34800_e44405_d_n8;

        let (assign34810_e44422, assign34810_e44422_d_n5, assign34810_e44422_d_n6, assign34810_e44422_d_n7, assign34810_e44422_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34810_e44418: f64 = (var_btat / var_sqrtumax);
        let assign34810_e44419: f64 = (0.375 * assign34810_e44418);
        let assign34810_e44420: f64 = (assign34810_e44419).sqrt();
        (assign34810_e44420, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34810_e44420)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34810_e44420)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34810_e44420)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34810_e44420)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign34810_e44422;
        var_ktat_dn5 = assign34810_e44422_d_n5;
        var_ktat_dn6 = assign34810_e44422_d_n6;
        var_ktat_dn7 = assign34810_e44422_d_n7;
        var_ktat_dn8 = assign34810_e44422_d_n8;

        let (assign34820_e44440, assign34820_e44440_d_n5, assign34820_e44440_d_n6, assign34820_e44440_d_n7, assign34820_e44440_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34820_e44435: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign34820_e44436: f64 = (2.0 * assign34820_e44435);
        let assign34820_e44438: f64 = (assign34820_e44436 - var_umax);
        (assign34820_e44438, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign34820_e44440;
        var_ltat_dn5 = assign34820_e44440_d_n5;
        var_ltat_dn6 = assign34820_e44440_d_n6;
        var_ltat_dn7 = assign34820_e44440_d_n7;
        var_ltat_dn8 = assign34820_e44440_d_n8;

        let (assign34830_e44466, assign34830_e44466_d_n5, assign34830_e44466_d_n6, assign34830_e44466_d_n7, assign34830_e44466_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34830_e44452: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign34830_e44454: f64 = (assign34830_e44452 * var_sqrtumax);
        let assign34830_e44457: f64 = (var_atatgat_d * var_umax);
        let assign34830_e44458: f64 = (assign34830_e44454 - assign34830_e44457);
        let assign34830_e44462: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34830_e44463: f64 = (0.5 * assign34830_e44462);
        let assign34830_e44464: f64 = (assign34830_e44458 + assign34830_e44463);
        (assign34830_e44464, (((((var_atatgat_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign34830_e44452 * var_sqrtumax_dn5)) - (var_atatgat_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign34830_e44452 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign34830_e44452 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign34830_e44452 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign34830_e44466;
        var_mtat_dn5 = assign34830_e44466_d_n5;
        var_mtat_dn6 = assign34830_e44466_d_n6;
        var_mtat_dn7 = assign34830_e44466_d_n7;
        var_mtat_dn8 = assign34830_e44466_d_n8;

        let (assign34840_e44482, assign34840_e44482_d_n5, assign34840_e44482_d_n6, assign34840_e44482_d_n7, assign34840_e44482_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34840_e44478: f64 = (var_ltat - 1.0);
        let assign34840_e44480: f64 = (assign34840_e44478 * var_ktat);
        (assign34840_e44480, ((var_ltat_dn5 * var_ktat) + (assign34840_e44478 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign34840_e44478 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign34840_e44478 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign34840_e44478 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign34840_e44482;
        var_xerfc_dn5 = assign34840_e44482_d_n5;
        var_xerfc_dn6 = assign34840_e44482_d_n6;
        var_xerfc_dn7 = assign34840_e44482_d_n7;
        var_xerfc_dn8 = assign34840_e44482_d_n8;

        let (assign34850_e44496, assign34850_e44496_d_n5, assign34850_e44496_d_n6, assign34850_e44496_d_n7, assign34850_e44496_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34850_e44494: f64 = (var_xerfc * var_xerfc);
        (assign34850_e44494, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign34850_e44496;
        var_ysq_dn5 = assign34850_e44496_d_n5;
        var_ysq_dn6 = assign34850_e44496_d_n6;
        var_ysq_dn7 = assign34850_e44496_d_n7;
        var_ysq_dn8 = assign34850_e44496_d_n8;

        let assign34860_e44499: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard695 = assign34860_e44499;

        let (assign34870_e44519, assign34870_e44519_d_n5, assign34870_e44519_d_n6, assign34870_e44519_d_n7, assign34870_e44519_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) && (var_guard695 != 0.0)) {
        let assign34870_e44515: f64 = (var_perfc * var_xerfc);
        let assign34870_e44516: f64 = (1.0 + assign34870_e44515);
        let assign34870_e44517: f64 = (1.0 / assign34870_e44516);
        (assign34870_e44517, (-((var_perfc * var_xerfc_dn5) / (assign34870_e44516 * assign34870_e44516))), (-((var_perfc * var_xerfc_dn6) / (assign34870_e44516 * assign34870_e44516))), (-((var_perfc * var_xerfc_dn7) / (assign34870_e44516 * assign34870_e44516))), (-((var_perfc * var_xerfc_dn8) / (assign34870_e44516 * assign34870_e44516))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign34870_e44519;
        var_terfc_dn5 = assign34870_e44519_d_n5;
        var_terfc_dn6 = assign34870_e44519_d_n6;
        var_terfc_dn7 = assign34870_e44519_d_n7;
        var_terfc_dn8 = assign34870_e44519_d_n8;

        let (assign34880_e44540, assign34880_e44540_d_n5, assign34880_e44540_d_n6, assign34880_e44540_d_n7, assign34880_e44540_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) && (var_guard695 == 0.0)) {
        let assign34880_e44536: f64 = (var_perfc * var_xerfc);
        let assign34880_e44537: f64 = (1.0 - assign34880_e44536);
        let assign34880_e44538: f64 = (1.0 / assign34880_e44537);
        (assign34880_e44538, (-((-(var_perfc * var_xerfc_dn5)) / (assign34880_e44537 * assign34880_e44537))), (-((-(var_perfc * var_xerfc_dn6)) / (assign34880_e44537 * assign34880_e44537))), (-((-(var_perfc * var_xerfc_dn7)) / (assign34880_e44537 * assign34880_e44537))), (-((-(var_perfc * var_xerfc_dn8)) / (assign34880_e44537 * assign34880_e44537))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign34880_e44540;
        var_terfc_dn5 = assign34880_e44540_d_n5;
        var_terfc_dn6 = assign34880_e44540_d_n6;
        var_terfc_dn7 = assign34880_e44540_d_n7;
        var_terfc_dn8 = assign34880_e44540_d_n8;

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
        *var_guard689_slot = var_guard689;
        *var_guard690_slot = var_guard690;
        *var_guard691_slot = var_guard691;
        *var_guard692_slot = var_guard692;
        *var_guard693_slot = var_guard693;
        *var_guard694_slot = var_guard694;
        *var_guard695_slot = var_guard695;
        *var_id__blk219_slot = var_id__blk219;
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
        var_berfc: f64,
        var_cbbtgatd_i: f64,
        var_cerfc: f64,
        var_ctatgatd_i: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn5: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fstopgat_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard689: f64,
        var_guard693: f64,
        var_id__blk219: f64,
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
        var_slopegat_d: f64,
        var_slopegat_d_dn5: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_terfc: f64,
        var_terfc_dn5: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_vbrgatd_i: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn5: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_wdepnulrinvgat_d: f64,
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
        var_guard696_slot: &mut f64,
        var_guard697_slot: &mut f64,
        var_guard698_slot: &mut f64,
        var_guard699_slot: &mut f64,
        var_guard700_slot: &mut f64,
        var_guard701_slot: &mut f64,
        var_guard702_slot: &mut f64,
        var_guard703_slot: &mut f64,
        var_guard704_slot: &mut f64,
        var_guard705_slot: &mut f64,
        var_guard706_slot: &mut f64,
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
        let mut var_guard696: f64 = *var_guard696_slot;
        let mut var_guard697: f64 = *var_guard697_slot;
        let mut var_guard698: f64 = *var_guard698_slot;
        let mut var_guard699: f64 = *var_guard699_slot;
        let mut var_guard700: f64 = *var_guard700_slot;
        let mut var_guard701: f64 = *var_guard701_slot;
        let mut var_guard702: f64 = *var_guard702_slot;
        let mut var_guard703: f64 = *var_guard703_slot;
        let mut var_guard704: f64 = *var_guard704_slot;
        let mut var_guard705: f64 = *var_guard705_slot;
        let mut var_guard706: f64 = *var_guard706_slot;
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

        let assign34890_e44542: f64 = (-var_ysq);
        let assign34890_e44544: f64 = (assign34890_e44542 + var_mtat);
        let assign34890_e44546: f64 = (-230.25850929940458);
        let assign34890_e44547: f64 = if assign34890_e44544 > assign34890_e44546 { 1.0 } else { 0.0 };
        var_guard696 = assign34890_e44547;

        let (assign34900_e44565, assign34900_e44565_d_n5, assign34900_e44565_d_n6, assign34900_e44565_d_n7, assign34900_e44565_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) && (var_guard696 != 0.0)) {
        let assign34900_e44560: f64 = (-var_ysq);
        let assign34900_e44562: f64 = (assign34900_e44560 + var_mtat);
        let assign34900_e44563: f64 = (assign34900_e44562).exp();
        (assign34900_e44563, (assign34900_e44563 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign34900_e44563 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign34900_e44563 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign34900_e44563 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34900_e44565;
        var_tmp_dn5 = assign34900_e44565_d_n5;
        var_tmp_dn6 = assign34900_e44565_d_n6;
        var_tmp_dn7 = assign34900_e44565_d_n7;
        var_tmp_dn8 = assign34900_e44565_d_n8;

        let (assign34910_e44614, assign34910_e44614_d_n5, assign34910_e44614_d_n6, assign34910_e44614_d_n7, assign34910_e44614_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) && (var_guard696 == 0.0)) {
        let assign34910_e44581: f64 = (-230.25850929940458);
        let assign34910_e44583: f64 = (-var_ysq);
        let assign34910_e44585: f64 = (assign34910_e44583 + var_mtat);
        let assign34910_e44586: f64 = (assign34910_e44581 - assign34910_e44585);
        let assign34910_e44590: f64 = (-230.25850929940458);
        let assign34910_e44592: f64 = (-var_ysq);
        let assign34910_e44594: f64 = (assign34910_e44592 + var_mtat);
        let assign34910_e44595: f64 = (assign34910_e44590 - assign34910_e44594);
        let assign34910_e44598: f64 = (-230.25850929940458);
        let assign34910_e44600: f64 = (-var_ysq);
        let assign34910_e44602: f64 = (assign34910_e44600 + var_mtat);
        let assign34910_e44603: f64 = (assign34910_e44598 - assign34910_e44602);
        let assign34910_e44605: f64 = (assign34910_e44603 * 0.3333333333333333);
        let assign34910_e44606: f64 = (1.0 + assign34910_e44605);
        let assign34910_e44607: f64 = (assign34910_e44595 * assign34910_e44606);
        let assign34910_e44608: f64 = (0.5 * assign34910_e44607);
        let assign34910_e44609: f64 = (1.0 + assign34910_e44608);
        let assign34910_e44610: f64 = (assign34910_e44586 * assign34910_e44609);
        let assign34910_e44611: f64 = (1.0 + assign34910_e44610);
        let assign34910_e44612: f64 = (1e-100 / assign34910_e44611);
        (assign34910_e44612, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign34910_e44609) + (assign34910_e44586 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign34910_e44606) + (assign34910_e44595 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign34910_e44611 * assign34910_e44611))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign34910_e44609) + (assign34910_e44586 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign34910_e44606) + (assign34910_e44595 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign34910_e44611 * assign34910_e44611))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign34910_e44609) + (assign34910_e44586 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign34910_e44606) + (assign34910_e44595 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign34910_e44611 * assign34910_e44611))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign34910_e44609) + (assign34910_e44586 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign34910_e44606) + (assign34910_e44595 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign34910_e44611 * assign34910_e44611))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34910_e44614;
        var_tmp_dn5 = assign34910_e44614_d_n5;
        var_tmp_dn6 = assign34910_e44614_d_n6;
        var_tmp_dn7 = assign34910_e44614_d_n7;
        var_tmp_dn8 = assign34910_e44614_d_n8;

        let (assign34920_e44644, assign34920_e44644_d_n5, assign34920_e44644_d_n6, assign34920_e44644_d_n7, assign34920_e44644_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34920_e44626: f64 = (0.29214664 * var_terfc);
        let assign34920_e44630: f64 = (var_terfc * var_terfc);
        let assign34920_e44631: f64 = (var_berfc * assign34920_e44630);
        let assign34920_e44632: f64 = (assign34920_e44626 + assign34920_e44631);
        let assign34920_e44636: f64 = (var_terfc * var_terfc);
        let assign34920_e44638: f64 = (assign34920_e44636 * var_terfc);
        let assign34920_e44639: f64 = (var_cerfc * assign34920_e44638);
        let assign34920_e44640: f64 = (assign34920_e44632 + assign34920_e44639);
        let assign34920_e44642: f64 = (assign34920_e44640 * var_tmp);
        (assign34920_e44642, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign34920_e44636 * var_terfc_dn5)))) * var_tmp) + (assign34920_e44640 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign34920_e44636 * var_terfc_dn6)))) * var_tmp) + (assign34920_e44640 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign34920_e44636 * var_terfc_dn7)))) * var_tmp) + (assign34920_e44640 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign34920_e44636 * var_terfc_dn8)))) * var_tmp) + (assign34920_e44640 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign34920_e44644;
        var_erfcpos_dn5 = assign34920_e44644_d_n5;
        var_erfcpos_dn6 = assign34920_e44644_d_n6;
        var_erfcpos_dn7 = assign34920_e44644_d_n7;
        var_erfcpos_dn8 = assign34920_e44644_d_n8;

        let assign34930_e44647: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard697 = assign34930_e44647;

        let (assign34940_e44661, assign34940_e44661_d_n5, assign34940_e44661_d_n6, assign34940_e44661_d_n7, assign34940_e44661_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) && (var_guard697 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign34940_e44661;
        var_erfctimesexpmtat_dn5 = assign34940_e44661_d_n5;
        var_erfctimesexpmtat_dn6 = assign34940_e44661_d_n6;
        var_erfctimesexpmtat_dn7 = assign34940_e44661_d_n7;
        var_erfctimesexpmtat_dn8 = assign34940_e44661_d_n8;

        let assign34950_e44664: f64 = (-230.25850929940458);
        let assign34950_e44665: f64 = if var_mtat > assign34950_e44664 { 1.0 } else { 0.0 };
        var_guard698 = assign34950_e44665;

        let (assign34960_e44683, assign34960_e44683_d_n5, assign34960_e44683_d_n6, assign34960_e44683_d_n7, assign34960_e44683_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) && (var_guard697 == 0.0)) && (var_guard698 != 0.0)) {
        let assign34960_e44681: f64 = (var_mtat).exp();
        (assign34960_e44681, (assign34960_e44681 * var_mtat_dn5), (assign34960_e44681 * var_mtat_dn6), (assign34960_e44681 * var_mtat_dn7), (assign34960_e44681 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34960_e44683;
        var_tmp_dn5 = assign34960_e44683_d_n5;
        var_tmp_dn6 = assign34960_e44683_d_n6;
        var_tmp_dn7 = assign34960_e44683_d_n7;
        var_tmp_dn8 = assign34960_e44683_d_n8;

        let (assign34970_e44726, assign34970_e44726_d_n5, assign34970_e44726_d_n6, assign34970_e44726_d_n7, assign34970_e44726_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) && (var_guard697 == 0.0)) && (var_guard698 == 0.0)) {
        let assign34970_e44702: f64 = (-230.25850929940458);
        let assign34970_e44704: f64 = (assign34970_e44702 - var_mtat);
        let assign34970_e44708: f64 = (-230.25850929940458);
        let assign34970_e44710: f64 = (assign34970_e44708 - var_mtat);
        let assign34970_e44713: f64 = (-230.25850929940458);
        let assign34970_e44715: f64 = (assign34970_e44713 - var_mtat);
        let assign34970_e44717: f64 = (assign34970_e44715 * 0.3333333333333333);
        let assign34970_e44718: f64 = (1.0 + assign34970_e44717);
        let assign34970_e44719: f64 = (assign34970_e44710 * assign34970_e44718);
        let assign34970_e44720: f64 = (0.5 * assign34970_e44719);
        let assign34970_e44721: f64 = (1.0 + assign34970_e44720);
        let assign34970_e44722: f64 = (assign34970_e44704 * assign34970_e44721);
        let assign34970_e44723: f64 = (1.0 + assign34970_e44722);
        let assign34970_e44724: f64 = (1e-100 / assign34970_e44723);
        (assign34970_e44724, (-((1e-100 * (((-var_mtat_dn5) * assign34970_e44721) + (assign34970_e44704 * (0.5 * (((-var_mtat_dn5) * assign34970_e44718) + (assign34970_e44710 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign34970_e44723 * assign34970_e44723))), (-((1e-100 * (((-var_mtat_dn6) * assign34970_e44721) + (assign34970_e44704 * (0.5 * (((-var_mtat_dn6) * assign34970_e44718) + (assign34970_e44710 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign34970_e44723 * assign34970_e44723))), (-((1e-100 * (((-var_mtat_dn7) * assign34970_e44721) + (assign34970_e44704 * (0.5 * (((-var_mtat_dn7) * assign34970_e44718) + (assign34970_e44710 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign34970_e44723 * assign34970_e44723))), (-((1e-100 * (((-var_mtat_dn8) * assign34970_e44721) + (assign34970_e44704 * (0.5 * (((-var_mtat_dn8) * assign34970_e44718) + (assign34970_e44710 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign34970_e44723 * assign34970_e44723))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign34970_e44726;
        var_tmp_dn5 = assign34970_e44726_d_n5;
        var_tmp_dn6 = assign34970_e44726_d_n6;
        var_tmp_dn7 = assign34970_e44726_d_n7;
        var_tmp_dn8 = assign34970_e44726_d_n8;

        let (assign34980_e44745, assign34980_e44745_d_n5, assign34980_e44745_d_n6, assign34980_e44745_d_n7, assign34980_e44745_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) && (var_guard697 == 0.0)) {
        let assign34980_e44741: f64 = (2.0 * var_tmp);
        let assign34980_e44743: f64 = (assign34980_e44741 - var_erfcpos);
        (assign34980_e44743, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign34980_e44745;
        var_erfctimesexpmtat_dn5 = assign34980_e44745_d_n5;
        var_erfctimesexpmtat_dn6 = assign34980_e44745_d_n6;
        var_erfctimesexpmtat_dn7 = assign34980_e44745_d_n7;
        var_erfctimesexpmtat_dn8 = assign34980_e44745_d_n8;

        let (assign34990_e44765, assign34990_e44765_d_n5, assign34990_e44765_d_n6, assign34990_e44765_d_n7, assign34990_e44765_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign34990_e44757: f64 = (1.772453850905516 * 0.5);
        let assign34990_e44760: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign34990_e44762: f64 = (assign34990_e44760 / var_ktat);
        let assign34990_e44763: f64 = (assign34990_e44757 * assign34990_e44762);
        (assign34990_e44763, (assign34990_e44757 * ((((var_atatgat_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign34990_e44760 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign34990_e44757 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign34990_e44760 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign34990_e44757 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign34990_e44760 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign34990_e44757 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign34990_e44760 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign34990_e44765;
        var_gammamax_dn5 = assign34990_e44765_d_n5;
        var_gammamax_dn6 = assign34990_e44765_d_n6;
        var_gammamax_dn7 = assign34990_e44765_d_n7;
        var_gammamax_dn8 = assign34990_e44765_d_n8;

        let (assign35000_e44783, assign35000_e44783_d_n5, assign35000_e44783_d_n6, assign35000_e44783_d_n7, assign35000_e44783_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard693 == 0.0)) {
        let assign35000_e44778: f64 = (var_asrh * var_gammamax);
        let assign35000_e44780: f64 = (assign35000_e44778 * var_wtat);
        let assign35000_e44781: f64 = (var_ctatgatd_i * assign35000_e44780);
        (assign35000_e44781, (var_ctatgatd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign35000_e44778 * var_wtat_dn5))), (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign35000_e44778 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign35000_e44778 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign35000_e44778 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign35000_e44783;
        var_itat_dn5 = assign35000_e44783_d_n5;
        var_itat_dn6 = assign35000_e44783_d_n6;
        var_itat_dn7 = assign35000_e44783_d_n7;
        var_itat_dn8 = assign35000_e44783_d_n8;

        let assign35010_e44786: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard699 = assign35010_e44786;

        let (assign35020_e44797, assign35020_e44797_d_n5, assign35020_e44797_d_n6, assign35020_e44797_d_n7, assign35020_e44797_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard699 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign35020_e44797;
        var_ibbt_dn5 = assign35020_e44797_d_n5;
        var_ibbt_dn6 = assign35020_e44797_d_n6;
        var_ibbt_dn7 = assign35020_e44797_d_n7;
        var_ibbt_dn8 = assign35020_e44797_d_n8;

        let assign35030_e44800: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard700 = assign35030_e44800;

        let (assign35040_e44819, assign35040_e44819_d_n5, assign35040_e44819_d_n6, assign35040_e44819_d_n7, assign35040_e44819_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard699 == 0.0)) && (var_guard700 != 0.0)) {
        let assign35040_e44814: f64 = (var_vbirgatd_i - var_vbbt);
        let assign35040_e44816: f64 = (assign35040_e44814 * var_vbirgatinv_d);
        let assign35040_e44817: f64 = (assign35040_e44816).sqrt();
        (assign35040_e44817, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35040_e44819;
        var_tmp_dn5 = assign35040_e44819_d_n5;
        var_tmp_dn6 = assign35040_e44819_d_n6;
        var_tmp_dn7 = assign35040_e44819_d_n7;
        var_tmp_dn8 = assign35040_e44819_d_n8;

        let (assign35050_e44840, assign35050_e44840_d_n5, assign35050_e44840_d_n6, assign35050_e44840_d_n7, assign35050_e44840_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard699 == 0.0)) && (var_guard700 == 0.0)) {
        let assign35050_e44834: f64 = (var_vbirgatd_i - var_vbbt);
        let assign35050_e44836: f64 = (assign35050_e44834 * var_vbirgatinv_d);
        let assign35050_e44838: f64 = (assign35050_e44836).powf(var_pgatd_i);
        (assign35050_e44838, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35050_e44840;
        var_tmp_dn5 = assign35050_e44840_d_n5;
        var_tmp_dn6 = assign35050_e44840_d_n6;
        var_tmp_dn7 = assign35050_e44840_d_n7;
        var_tmp_dn8 = assign35050_e44840_d_n8;

        let (assign35060_e44860, assign35060_e44860_d_n5, assign35060_e44860_d_n6, assign35060_e44860_d_n7, assign35060_e44860_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard699 == 0.0)) {
        let assign35060_e44853: f64 = (var_vbirgatd_i - var_vbbt);
        let assign35060_e44855: f64 = (assign35060_e44853 * var_wdepnulrinvgat_d);
        let assign35060_e44857: f64 = (assign35060_e44855 / var_tmp);
        let assign35060_e44858: f64 = (var_one_over_one_minus_pgat_d * assign35060_e44857);
        (assign35060_e44858, (var_one_over_one_minus_pgat_d * (-((assign35060_e44855 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign35060_e44855 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign35060_e44855 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign35060_e44855 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign35060_e44860;
        var_fmaxr_dn5 = assign35060_e44860_d_n5;
        var_fmaxr_dn6 = assign35060_e44860_d_n6;
        var_fmaxr_dn7 = assign35060_e44860_d_n7;
        var_fmaxr_dn8 = assign35060_e44860_d_n8;

        let assign35070_e44862: f64 = (-var_fbbtgat_d);
        let assign35070_e44864: f64 = (assign35070_e44862 / var_fmaxr);
        let assign35070_e44865: f64 = (assign35070_e44864).abs();
        let assign35070_e44867: f64 = if assign35070_e44865 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard701 = assign35070_e44867;

        let (assign35080_e44885, assign35080_e44885_d_n5, assign35080_e44885_d_n6, assign35080_e44885_d_n7, assign35080_e44885_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard699 == 0.0)) && (var_guard701 != 0.0)) {
        let assign35080_e44880: f64 = (-var_fbbtgat_d);
        let assign35080_e44882: f64 = (assign35080_e44880 / var_fmaxr);
        let assign35080_e44883: f64 = (assign35080_e44882).exp();
        (assign35080_e44883, (assign35080_e44883 * ((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign35080_e44880 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign35080_e44883 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign35080_e44880 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign35080_e44883 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign35080_e44880 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign35080_e44883 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign35080_e44880 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35080_e44885;
        var_tmp_dn5 = assign35080_e44885_d_n5;
        var_tmp_dn6 = assign35080_e44885_d_n6;
        var_tmp_dn7 = assign35080_e44885_d_n7;
        var_tmp_dn8 = assign35080_e44885_d_n8;

        let assign35090_e44887: f64 = (-var_fbbtgat_d);
        let assign35090_e44889: f64 = (assign35090_e44887 / var_fmaxr);
        let assign35090_e44891: f64 = if assign35090_e44889 < 0.0 { 1.0 } else { 0.0 };
        var_guard702 = assign35090_e44891;

        let (assign35100_e44942, assign35100_e44942_d_n5, assign35100_e44942_d_n6, assign35100_e44942_d_n7, assign35100_e44942_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard699 == 0.0)) && (var_guard701 == 0.0)) && (var_guard702 != 0.0)) {
        let assign35100_e44909: f64 = (-230.25850929940458);
        let assign35100_e44911: f64 = (-var_fbbtgat_d);
        let assign35100_e44913: f64 = (assign35100_e44911 / var_fmaxr);
        let assign35100_e44914: f64 = (assign35100_e44909 - assign35100_e44913);
        let assign35100_e44918: f64 = (-230.25850929940458);
        let assign35100_e44920: f64 = (-var_fbbtgat_d);
        let assign35100_e44922: f64 = (assign35100_e44920 / var_fmaxr);
        let assign35100_e44923: f64 = (assign35100_e44918 - assign35100_e44922);
        let assign35100_e44926: f64 = (-230.25850929940458);
        let assign35100_e44928: f64 = (-var_fbbtgat_d);
        let assign35100_e44930: f64 = (assign35100_e44928 / var_fmaxr);
        let assign35100_e44931: f64 = (assign35100_e44926 - assign35100_e44930);
        let assign35100_e44933: f64 = (assign35100_e44931 * 0.3333333333333333);
        let assign35100_e44934: f64 = (1.0 + assign35100_e44933);
        let assign35100_e44935: f64 = (assign35100_e44923 * assign35100_e44934);
        let assign35100_e44936: f64 = (0.5 * assign35100_e44935);
        let assign35100_e44937: f64 = (1.0 + assign35100_e44936);
        let assign35100_e44938: f64 = (assign35100_e44914 * assign35100_e44937);
        let assign35100_e44939: f64 = (1.0 + assign35100_e44938);
        let assign35100_e44940: f64 = (1e-100 / assign35100_e44939);
        (assign35100_e44940, (-((1e-100 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign35100_e44911 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign35100_e44937) + (assign35100_e44914 * (0.5 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign35100_e44920 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign35100_e44934) + (assign35100_e44923 * ((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign35100_e44928 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign35100_e44939 * assign35100_e44939))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign35100_e44911 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign35100_e44937) + (assign35100_e44914 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign35100_e44920 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign35100_e44934) + (assign35100_e44923 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign35100_e44928 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign35100_e44939 * assign35100_e44939))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign35100_e44911 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign35100_e44937) + (assign35100_e44914 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign35100_e44920 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign35100_e44934) + (assign35100_e44923 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign35100_e44928 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign35100_e44939 * assign35100_e44939))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign35100_e44911 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign35100_e44937) + (assign35100_e44914 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign35100_e44920 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign35100_e44934) + (assign35100_e44923 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign35100_e44928 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign35100_e44939 * assign35100_e44939))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35100_e44942;
        var_tmp_dn5 = assign35100_e44942_d_n5;
        var_tmp_dn6 = assign35100_e44942_d_n6;
        var_tmp_dn7 = assign35100_e44942_d_n7;
        var_tmp_dn8 = assign35100_e44942_d_n8;

        let (assign35110_e44991, assign35110_e44991_d_n5, assign35110_e44991_d_n6, assign35110_e44991_d_n7, assign35110_e44991_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard699 == 0.0)) && (var_guard701 == 0.0)) && (var_guard702 == 0.0)) {
        let assign35110_e44961: f64 = (-var_fbbtgat_d);
        let assign35110_e44963: f64 = (assign35110_e44961 / var_fmaxr);
        let assign35110_e44965: f64 = (assign35110_e44963 - 230.25850929940458);
        let assign35110_e44969: f64 = (-var_fbbtgat_d);
        let assign35110_e44971: f64 = (assign35110_e44969 / var_fmaxr);
        let assign35110_e44973: f64 = (assign35110_e44971 - 230.25850929940458);
        let assign35110_e44976: f64 = (-var_fbbtgat_d);
        let assign35110_e44978: f64 = (assign35110_e44976 / var_fmaxr);
        let assign35110_e44980: f64 = (assign35110_e44978 - 230.25850929940458);
        let assign35110_e44982: f64 = (assign35110_e44980 * 0.3333333333333333);
        let assign35110_e44983: f64 = (1.0 + assign35110_e44982);
        let assign35110_e44984: f64 = (assign35110_e44973 * assign35110_e44983);
        let assign35110_e44985: f64 = (0.5 * assign35110_e44984);
        let assign35110_e44986: f64 = (1.0 + assign35110_e44985);
        let assign35110_e44987: f64 = (assign35110_e44965 * assign35110_e44986);
        let assign35110_e44988: f64 = (1.0 + assign35110_e44987);
        let assign35110_e44989: f64 = (1e100 * assign35110_e44988);
        (assign35110_e44989, (1e100 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign35110_e44961 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign35110_e44986) + (assign35110_e44965 * (0.5 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign35110_e44969 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign35110_e44983) + (assign35110_e44973 * (((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign35110_e44976 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign35110_e44961 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign35110_e44986) + (assign35110_e44965 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign35110_e44969 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign35110_e44983) + (assign35110_e44973 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign35110_e44976 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign35110_e44961 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign35110_e44986) + (assign35110_e44965 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign35110_e44969 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign35110_e44983) + (assign35110_e44973 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign35110_e44976 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign35110_e44961 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign35110_e44986) + (assign35110_e44965 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign35110_e44969 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign35110_e44983) + (assign35110_e44973 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign35110_e44976 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35110_e44991;
        var_tmp_dn5 = assign35110_e44991_d_n5;
        var_tmp_dn6 = assign35110_e44991_d_n6;
        var_tmp_dn7 = assign35110_e44991_d_n7;
        var_tmp_dn8 = assign35110_e44991_d_n8;

        let (assign35120_e45011, assign35120_e45011_d_n5, assign35120_e45011_d_n6, assign35120_e45011_d_n7, assign35120_e45011_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard699 == 0.0)) {
        let assign35120_e45004: f64 = (var_v3 * var_fmaxr);
        let assign35120_e45006: f64 = (assign35120_e45004 * var_fmaxr);
        let assign35120_e45008: f64 = (assign35120_e45006 * var_tmp);
        let assign35120_e45009: f64 = (var_cbbtgatd_i * assign35120_e45008);
        (assign35120_e45009, (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign35120_e45004 * var_fmaxr_dn5)) * var_tmp) + (assign35120_e45006 * var_tmp_dn5))), (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign35120_e45004 * var_fmaxr_dn6)) * var_tmp) + (assign35120_e45006 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign35120_e45004 * var_fmaxr_dn7)) * var_tmp) + (assign35120_e45006 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign35120_e45004 * var_fmaxr_dn8)) * var_tmp) + (assign35120_e45006 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign35120_e45011;
        var_ibbt_dn5 = assign35120_e45011_d_n5;
        var_ibbt_dn6 = assign35120_e45011_d_n6;
        var_ibbt_dn7 = assign35120_e45011_d_n7;
        var_ibbt_dn8 = assign35120_e45011_d_n8;

        let assign35130_e45014: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard703 = assign35130_e45014;

        let (assign35140_e45025, assign35140_e45025_d_n5, assign35140_e45025_d_n6, assign35140_e45025_d_n7, assign35140_e45025_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard703 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign35140_e45025;
        var_fbreakdown_dn5 = assign35140_e45025_d_n5;
        var_fbreakdown_dn6 = assign35140_e45025_d_n6;
        var_fbreakdown_dn7 = assign35140_e45025_d_n7;
        var_fbreakdown_dn8 = assign35140_e45025_d_n8;

        let assign35150_e45028: f64 = (-var_alphaav);
        let assign35150_e45030: f64 = (assign35150_e45028 * var_vbrgatd_i);
        let assign35150_e45031: f64 = if var_vav > assign35150_e45030 { 1.0 } else { 0.0 };
        var_guard704 = assign35150_e45031;

        let assign35160_e45034: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard705 = assign35160_e45034;

        let (assign35170_e45064, assign35170_e45064_d_n5, assign35170_e45064_d_n6, assign35170_e45064_d_n7, assign35170_e45064_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard703 == 0.0)) && (var_guard704 != 0.0)) && (var_guard705 != 0.0)) {
        let assign35170_e45050: f64 = (var_vav * var_vbrinvgat_d);
        let assign35170_e45053: f64 = (var_vav * var_vbrinvgat_d);
        let assign35170_e45054: f64 = (assign35170_e45050 * assign35170_e45053);
        let assign35170_e45057: f64 = (var_vav * var_vbrinvgat_d);
        let assign35170_e45058: f64 = (assign35170_e45054 * assign35170_e45057);
        let assign35170_e45061: f64 = (var_vav * var_vbrinvgat_d);
        let assign35170_e45062: f64 = (assign35170_e45058 * assign35170_e45061);
        (assign35170_e45062, (((((((var_vav * var_vbrinvgat_d_dn5) * assign35170_e45053) + (assign35170_e45050 * (var_vav * var_vbrinvgat_d_dn5))) * assign35170_e45057) + (assign35170_e45054 * (var_vav * var_vbrinvgat_d_dn5))) * assign35170_e45061) + (assign35170_e45058 * (var_vav * var_vbrinvgat_d_dn5))), (((((((var_vav * var_vbrinvgat_d_dn6) * assign35170_e45053) + (assign35170_e45050 * (var_vav * var_vbrinvgat_d_dn6))) * assign35170_e45057) + (assign35170_e45054 * (var_vav * var_vbrinvgat_d_dn6))) * assign35170_e45061) + (assign35170_e45058 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign35170_e45053) + (assign35170_e45050 * (var_vav * var_vbrinvgat_d_dn7))) * assign35170_e45057) + (assign35170_e45054 * (var_vav * var_vbrinvgat_d_dn7))) * assign35170_e45061) + (assign35170_e45058 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign35170_e45053) + (assign35170_e45050 * (var_vav * var_vbrinvgat_d_dn8))) * assign35170_e45057) + (assign35170_e45054 * (var_vav * var_vbrinvgat_d_dn8))) * assign35170_e45061) + (assign35170_e45058 * (var_vav * var_vbrinvgat_d_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35170_e45064;
        var_tmp_dn5 = assign35170_e45064_d_n5;
        var_tmp_dn6 = assign35170_e45064_d_n6;
        var_tmp_dn7 = assign35170_e45064_d_n7;
        var_tmp_dn8 = assign35170_e45064_d_n8;

        let (assign35180_e45086, assign35180_e45086_d_n5, assign35180_e45086_d_n6, assign35180_e45086_d_n7, assign35180_e45086_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard703 == 0.0)) && (var_guard704 != 0.0)) && (var_guard705 == 0.0)) {
        let assign35180_e45081: f64 = (var_vav * var_vbrinvgat_d);
        let assign35180_e45082: f64 = (assign35180_e45081).abs();
        let assign35180_e45084: f64 = (assign35180_e45082).powf(var_pbrgatd_i);
        (assign35180_e45084, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign35180_e45082).powf(var_pbrgatd_i - 1.0) * if assign35180_e45081 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) })) } } else { (assign35180_e45084 * (var_pbrgatd_i * (if assign35180_e45081 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) } / assign35180_e45082))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign35180_e45082).powf(var_pbrgatd_i - 1.0) * if assign35180_e45081 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign35180_e45084 * (var_pbrgatd_i * (if assign35180_e45081 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign35180_e45082))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign35180_e45082).powf(var_pbrgatd_i - 1.0) * if assign35180_e45081 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign35180_e45084 * (var_pbrgatd_i * (if assign35180_e45081 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign35180_e45082))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign35180_e45082).powf(var_pbrgatd_i - 1.0) * if assign35180_e45081 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign35180_e45084 * (var_pbrgatd_i * (if assign35180_e45081 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign35180_e45082))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35180_e45086;
        var_tmp_dn5 = assign35180_e45086_d_n5;
        var_tmp_dn6 = assign35180_e45086_d_n6;
        var_tmp_dn7 = assign35180_e45086_d_n7;
        var_tmp_dn8 = assign35180_e45086_d_n8;

        let (assign35190_e45104, assign35190_e45104_d_n5, assign35190_e45104_d_n6, assign35190_e45104_d_n7, assign35190_e45104_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard703 == 0.0)) && (var_guard704 != 0.0)) {
        let assign35190_e45101: f64 = (1.0 - var_tmp);
        let assign35190_e45102: f64 = (1.0 / assign35190_e45101);
        (assign35190_e45102, (-((-var_tmp_dn5) / (assign35190_e45101 * assign35190_e45101))), (-((-var_tmp_dn6) / (assign35190_e45101 * assign35190_e45101))), (-((-var_tmp_dn7) / (assign35190_e45101 * assign35190_e45101))), (-((-var_tmp_dn8) / (assign35190_e45101 * assign35190_e45101))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign35190_e45104;
        var_fbreakdown_dn5 = assign35190_e45104_d_n5;
        var_fbreakdown_dn6 = assign35190_e45104_d_n6;
        var_fbreakdown_dn7 = assign35190_e45104_d_n7;
        var_fbreakdown_dn8 = assign35190_e45104_d_n8;

        let (assign35200_e45127, assign35200_e45127_d_n5, assign35200_e45127_d_n6, assign35200_e45127_d_n7, assign35200_e45127_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) && (var_guard703 == 0.0)) && (var_guard704 == 0.0)) {
        let assign35200_e45121: f64 = (var_alphaav * var_vbrgatd_i);
        let assign35200_e45122: f64 = (var_vav + assign35200_e45121);
        let assign35200_e45124: f64 = (assign35200_e45122 * var_slopegat_d);
        let assign35200_e45125: f64 = (var_fstopgat_d + assign35200_e45124);
        (assign35200_e45125, (assign35200_e45122 * var_slopegat_d_dn5), (assign35200_e45122 * var_slopegat_d_dn6), (assign35200_e45122 * var_slopegat_d_dn7), (assign35200_e45122 * var_slopegat_d_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign35200_e45127;
        var_fbreakdown_dn5 = assign35200_e45127_d_n5;
        var_fbreakdown_dn6 = assign35200_e45127_d_n6;
        var_fbreakdown_dn7 = assign35200_e45127_d_n7;
        var_fbreakdown_dn8 = assign35200_e45127_d_n8;

        let (assign35210_e45146, assign35210_e45146_d_n5, assign35210_e45146_d_n6, assign35210_e45146_d_n7, assign35210_e45146_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard689 == 0.0)) {
        let assign35210_e45137: f64 = (var_id__blk219 + var_isrh);
        let assign35210_e45139: f64 = (assign35210_e45137 + var_itat);
        let assign35210_e45141: f64 = (assign35210_e45139 + var_ibbt);
        let assign35210_e45142: f64 = (p.p29 * assign35210_e45141);
        let assign35210_e45144: f64 = (assign35210_e45142 * var_fbreakdown);
        (assign35210_e45144, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign35210_e45142 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign35210_e45142 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign35210_e45142 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign35210_e45142 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign35210_e45146;
        var_ijungat_dn5 = assign35210_e45146_d_n5;
        var_ijungat_dn6 = assign35210_e45146_d_n6;
        var_ijungat_dn7 = assign35210_e45146_d_n7;
        var_ijungat_dn8 = assign35210_e45146_d_n8;

        let (assign35220_e45162, assign35220_e45162_d_n5, assign35220_e45162_d_n6, assign35220_e45162_d_n7, assign35220_e45162_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign35220_e45152: f64 = (var_abdrain_i * var_ijunbot);
        let assign35220_e45155: f64 = (var_lsdrain_i * var_ijunsti);
        let assign35220_e45156: f64 = (assign35220_e45152 + assign35220_e45155);
        let assign35220_e45159: f64 = (var_lgdrain_i * var_ijungat);
        let assign35220_e45160: f64 = (assign35220_e45156 + assign35220_e45159);
        (assign35220_e45160, (((var_abdrain_i * var_ijunbot_dn5) + (var_lsdrain_i * var_ijunsti_dn5)) + (var_lgdrain_i * var_ijungat_dn5)), (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)),)
    } else {
        (var_i3, var_i3_dn5, var_i3_dn6, var_i3_dn7, var_i3_dn8,)
    }
};
        var_i3 = assign35220_e45162;
        var_i3_dn5 = assign35220_e45162_d_n5;
        var_i3_dn6 = assign35220_e45162_d_n6;
        var_i3_dn7 = assign35220_e45162_d_n7;
        var_i3_dn8 = assign35220_e45162_d_n8;

        let (assign35230_e45168,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign35230_e45168;

        let (assign35240_e45174,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign35240_e45174;

        let assign35250_e45186: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard706 = assign35250_e45186;

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
        *var_guard696_slot = var_guard696;
        *var_guard697_slot = var_guard697;
        *var_guard698_slot = var_guard698;
        *var_guard699_slot = var_guard699;
        *var_guard700_slot = var_guard700;
        *var_guard701_slot = var_guard701;
        *var_guard702_slot = var_guard702;
        *var_guard703_slot = var_guard703;
        *var_guard704_slot = var_guard704;
        *var_guard705_slot = var_guard705;
        *var_guard706_slot = var_guard706;
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
    }

    pub(super) fn stamp_transient_block_73(
        var_abdrain_i: f64,
        var_atatbot_d: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_ftdbot_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard706: f64,
        var_idsatbot_d: f64,
        var_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_v4: f64,
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
        var_guard707_slot: &mut f64,
        var_guard708_slot: &mut f64,
        var_guard709_slot: &mut f64,
        var_guard710_slot: &mut f64,
        var_guard711_slot: &mut f64,
        var_guard712_slot: &mut f64,
        var_guard713_slot: &mut f64,
        var_guard714_slot: &mut f64,
        var_guard715_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
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
        let mut var_guard707: f64 = *var_guard707_slot;
        let mut var_guard708: f64 = *var_guard708_slot;
        let mut var_guard709: f64 = *var_guard709_slot;
        let mut var_guard710: f64 = *var_guard710_slot;
        let mut var_guard711: f64 = *var_guard711_slot;
        let mut var_guard712: f64 = *var_guard712_slot;
        let mut var_guard713: f64 = *var_guard713_slot;
        let mut var_guard714: f64 = *var_guard714_slot;
        let mut var_guard715: f64 = *var_guard715_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
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

        let assign35330_e45272: f64 = if var_v4 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard707 = assign35330_e45272;

        let assign35340_e45274: f64 = (-0.5);
        let assign35340_e45277: f64 = (var_v4 * var_phitdinv);
        let assign35340_e45278: f64 = (assign35340_e45274 * assign35340_e45277);
        let assign35340_e45279: f64 = (assign35340_e45278).abs();
        let assign35340_e45281: f64 = if assign35340_e45279 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard708 = assign35340_e45281;

        let (assign35350_e45299,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) && (var_guard707 != 0.0)) && (var_guard708 != 0.0)) {
        let assign35350_e45292: f64 = (-0.5);
        let assign35350_e45295: f64 = (var_v4 * var_phitdinv);
        let assign35350_e45296: f64 = (assign35350_e45292 * assign35350_e45295);
        let assign35350_e45297: f64 = (assign35350_e45296).exp();
        (assign35350_e45297,)
    } else {
        (var_z,)
    }
};
        var_z = assign35350_e45299;

        let assign35360_e45301: f64 = (-0.5);
        let assign35360_e45304: f64 = (var_v4 * var_phitdinv);
        let assign35360_e45305: f64 = (assign35360_e45301 * assign35360_e45304);
        let assign35360_e45307: f64 = if assign35360_e45305 < 0.0 { 1.0 } else { 0.0 };
        var_guard709 = assign35360_e45307;

        let (assign35370_e45362,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) && (var_guard707 != 0.0)) && (var_guard708 == 0.0)) && (var_guard709 != 0.0)) {
        let assign35370_e45323: f64 = (-230.25850929940458);
        let assign35370_e45325: f64 = (-0.5);
        let assign35370_e45328: f64 = (var_v4 * var_phitdinv);
        let assign35370_e45329: f64 = (assign35370_e45325 * assign35370_e45328);
        let assign35370_e45330: f64 = (assign35370_e45323 - assign35370_e45329);
        let assign35370_e45334: f64 = (-230.25850929940458);
        let assign35370_e45336: f64 = (-0.5);
        let assign35370_e45339: f64 = (var_v4 * var_phitdinv);
        let assign35370_e45340: f64 = (assign35370_e45336 * assign35370_e45339);
        let assign35370_e45341: f64 = (assign35370_e45334 - assign35370_e45340);
        let assign35370_e45344: f64 = (-230.25850929940458);
        let assign35370_e45346: f64 = (-0.5);
        let assign35370_e45349: f64 = (var_v4 * var_phitdinv);
        let assign35370_e45350: f64 = (assign35370_e45346 * assign35370_e45349);
        let assign35370_e45351: f64 = (assign35370_e45344 - assign35370_e45350);
        let assign35370_e45353: f64 = (assign35370_e45351 * 0.3333333333333333);
        let assign35370_e45354: f64 = (1.0 + assign35370_e45353);
        let assign35370_e45355: f64 = (assign35370_e45341 * assign35370_e45354);
        let assign35370_e45356: f64 = (0.5 * assign35370_e45355);
        let assign35370_e45357: f64 = (1.0 + assign35370_e45356);
        let assign35370_e45358: f64 = (assign35370_e45330 * assign35370_e45357);
        let assign35370_e45359: f64 = (1.0 + assign35370_e45358);
        let assign35370_e45360: f64 = (1e-100 / assign35370_e45359);
        (assign35370_e45360,)
    } else {
        (var_z,)
    }
};
        var_z = assign35370_e45362;

        let (assign35380_e45415,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) && (var_guard707 != 0.0)) && (var_guard708 == 0.0)) && (var_guard709 == 0.0)) {
        let assign35380_e45379: f64 = (-0.5);
        let assign35380_e45382: f64 = (var_v4 * var_phitdinv);
        let assign35380_e45383: f64 = (assign35380_e45379 * assign35380_e45382);
        let assign35380_e45385: f64 = (assign35380_e45383 - 230.25850929940458);
        let assign35380_e45389: f64 = (-0.5);
        let assign35380_e45392: f64 = (var_v4 * var_phitdinv);
        let assign35380_e45393: f64 = (assign35380_e45389 * assign35380_e45392);
        let assign35380_e45395: f64 = (assign35380_e45393 - 230.25850929940458);
        let assign35380_e45398: f64 = (-0.5);
        let assign35380_e45401: f64 = (var_v4 * var_phitdinv);
        let assign35380_e45402: f64 = (assign35380_e45398 * assign35380_e45401);
        let assign35380_e45404: f64 = (assign35380_e45402 - 230.25850929940458);
        let assign35380_e45406: f64 = (assign35380_e45404 * 0.3333333333333333);
        let assign35380_e45407: f64 = (1.0 + assign35380_e45406);
        let assign35380_e45408: f64 = (assign35380_e45395 * assign35380_e45407);
        let assign35380_e45409: f64 = (0.5 * assign35380_e45408);
        let assign35380_e45410: f64 = (1.0 + assign35380_e45409);
        let assign35380_e45411: f64 = (assign35380_e45385 * assign35380_e45410);
        let assign35380_e45412: f64 = (1.0 + assign35380_e45411);
        let assign35380_e45413: f64 = (1e100 * assign35380_e45412);
        (assign35380_e45413,)
    } else {
        (var_z,)
    }
};
        var_z = assign35380_e45415;

        let (assign35390_e45427,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) && (var_guard707 != 0.0)) {
        let assign35390_e45425: f64 = (1.0 / var_z);
        (assign35390_e45425,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign35390_e45427;

        let (assign35400_e45439,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) && (var_guard707 != 0.0)) {
        let assign35400_e45437: f64 = (var_zinv * var_zinv);
        (assign35400_e45437,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign35400_e45439;

        let (assign35410_e45458,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) && (var_guard707 == 0.0)) {
        let assign35410_e45451: f64 = (var_v4 - var_vmax_d);
        let assign35410_e45453: f64 = (assign35410_e45451 * var_phitdinv);
        let assign35410_e45454: f64 = (1.0 + assign35410_e45453);
        let assign35410_e45456: f64 = (assign35410_e45454 * var_exp_vmax_over_phitd_d);
        (assign35410_e45456,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign35410_e45458;

        let (assign35420_e45470,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) && (var_guard707 == 0.0)) {
        let assign35420_e45468: f64 = (var_idmult).sqrt();
        (assign35420_e45468,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign35420_e45470;

        let (assign35430_e45483,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) && (var_guard707 == 0.0)) {
        let assign35430_e45481: f64 = (1.0 / var_zinv);
        (assign35430_e45481,)
    } else {
        (var_z,)
    }
};
        var_z = assign35430_e45483;

        let (assign35440_e45493,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) {
        let assign35440_e45491: f64 = (var_idmult - 1.0);
        (assign35440_e45491,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign35440_e45493;

        let assign35450_e45496: f64 = if var_v4 > 0.0 { 1.0 } else { 0.0 };
        var_guard710 = assign35450_e45496;

        let (assign35460_e45522,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) && (var_guard710 != 0.0)) {
        let assign35460_e45508: f64 = (2.0 + var_z);
        let assign35460_e45511: f64 = (var_z + 1.0);
        let assign35460_e45514: f64 = (var_z + 3.0);
        let assign35460_e45515: f64 = (assign35460_e45511 * assign35460_e45514);
        let assign35460_e45516: f64 = (assign35460_e45515).sqrt();
        let assign35460_e45517: f64 = (assign35460_e45508 + assign35460_e45516);
        let assign35460_e45518: f64 = (assign35460_e45517).ln();
        let assign35460_e45519: f64 = (var_phitd * assign35460_e45518);
        let assign35460_e45520: f64 = (2.0 * assign35460_e45519);
        (assign35460_e45520,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign35460_e45522;

        let (assign35470_e45556,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) && (var_guard710 == 0.0)) {
        let assign35470_e45532: f64 = (-var_v4);
        let assign35470_e45537: f64 = (2.0 * var_zinv);
        let assign35470_e45539: f64 = (assign35470_e45537 + 1.0);
        let assign35470_e45542: f64 = (1.0 + var_zinv);
        let assign35470_e45546: f64 = (3.0 * var_zinv);
        let assign35470_e45547: f64 = (1.0 + assign35470_e45546);
        let assign35470_e45548: f64 = (assign35470_e45542 * assign35470_e45547);
        let assign35470_e45549: f64 = (assign35470_e45548).sqrt();
        let assign35470_e45550: f64 = (assign35470_e45539 + assign35470_e45549);
        let assign35470_e45551: f64 = (assign35470_e45550).ln();
        let assign35470_e45552: f64 = (var_phitd * assign35470_e45551);
        let assign35470_e45553: f64 = (2.0 * assign35470_e45552);
        let assign35470_e45554: f64 = (assign35470_e45532 + assign35470_e45553);
        (assign35470_e45554,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign35470_e45556;

        let (assign35480_e45566,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) {
        let assign35480_e45564: f64 = (var_vbimin_d - var_two_psistar);
        (assign35480_e45564,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign35480_e45566;

        let (assign35490_e45593,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) {
        let assign35490_e45575: f64 = (var_v4 + var_vjlim);
        let assign35490_e45578: f64 = (var_v4 - var_vjlim);
        let assign35490_e45581: f64 = (var_v4 - var_vjlim);
        let assign35490_e45582: f64 = (assign35490_e45578 * assign35490_e45581);
        let assign35490_e45585: f64 = (4.0 * var_phitd);
        let assign35490_e45587: f64 = (assign35490_e45585 * var_phitd);
        let assign35490_e45588: f64 = (assign35490_e45582 + assign35490_e45587);
        let assign35490_e45589: f64 = (assign35490_e45588).sqrt();
        let assign35490_e45590: f64 = (assign35490_e45575 - assign35490_e45589);
        let assign35490_e45591: f64 = (0.5 * assign35490_e45590);
        (assign35490_e45591,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign35490_e45593;

        let (assign35500_e45620,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) {
        let assign35500_e45602: f64 = (var_v4 + var_vbbtlim_d);
        let assign35500_e45605: f64 = (var_v4 - var_vbbtlim_d);
        let assign35500_e45608: f64 = (var_v4 - var_vbbtlim_d);
        let assign35500_e45609: f64 = (assign35500_e45605 * assign35500_e45608);
        let assign35500_e45612: f64 = (4.0 * var_phitr);
        let assign35500_e45614: f64 = (assign35500_e45612 * var_phitr);
        let assign35500_e45615: f64 = (assign35500_e45609 + assign35500_e45614);
        let assign35500_e45616: f64 = (assign35500_e45615).sqrt();
        let assign35500_e45617: f64 = (assign35500_e45602 - assign35500_e45616);
        let assign35500_e45618: f64 = (0.5 * assign35500_e45617);
        (assign35500_e45618,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign35500_e45620;

        let (assign35510_e45647,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard706 != 0.0)) {
        let assign35510_e45629: f64 = var_v4;
        let assign35510_e45632: f64 = var_v4;
        let assign35510_e45635: f64 = var_v4;
        let assign35510_e45636: f64 = (assign35510_e45632 * assign35510_e45635);
        let assign35510_e45639: f64 = (4.0 * 1e-6);
        let assign35510_e45641: f64 = (assign35510_e45639 * 1e-6);
        let assign35510_e45642: f64 = (assign35510_e45636 + assign35510_e45641);
        let assign35510_e45643: f64 = (assign35510_e45642).sqrt();
        let assign35510_e45644: f64 = (assign35510_e45629 - assign35510_e45643);
        let assign35510_e45645: f64 = (0.5 * assign35510_e45644);
        (assign35510_e45645,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign35510_e45647;

        let assign35520_e45650: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard711 = assign35520_e45650;

        let (assign35530_e45658, assign35530_e45658_d_n5, assign35530_e45658_d_n6, assign35530_e45658_d_n7, assign35530_e45658_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign35530_e45658;
        var_ijunbot_dn5 = assign35530_e45658_d_n5;
        var_ijunbot_dn6 = assign35530_e45658_d_n6;
        var_ijunbot_dn7 = assign35530_e45658_d_n7;
        var_ijunbot_dn8 = assign35530_e45658_d_n8;

        let (assign35540_e45669,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) {
        let assign35540_e45667: f64 = (var_idsatbot_d * var_idmult);
        (assign35540_e45667,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign35540_e45669;

        let assign35550_e45676: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard712 = assign35550_e45676;

        let (assign35560_e45687, assign35560_e45687_d_n5, assign35560_e45687_d_n6, assign35560_e45687_d_n7, assign35560_e45687_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign35560_e45687;
        var_isrh_dn5 = assign35560_e45687_d_n5;
        var_isrh_dn6 = assign35560_e45687_d_n6;
        var_isrh_dn7 = assign35560_e45687_d_n7;
        var_isrh_dn8 = assign35560_e45687_d_n8;

        let (assign35570_e45701,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 == 0.0)) {
        let assign35570_e45699: f64 = (var_vbibot_d - var_vjsrh);
        (assign35570_e45699,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign35570_e45701;

        let (assign35580_e45720,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 == 0.0)) {
        let assign35580_e45715: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign35580_e45716: f64 = (1.0 - assign35580_e45715);
        let assign35580_e45717: f64 = (assign35580_e45716).sqrt();
        let assign35580_e45718: f64 = (1.0 - assign35580_e45717);
        (assign35580_e45718,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign35580_e45720;

        let assign35590_e45723: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard713 = assign35590_e45723;

        let (assign35600_e45737,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 == 0.0)) && (var_guard713 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign35600_e45737;

        let (assign35610_e45769,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 == 0.0)) && (var_guard713 == 0.0)) {
        let assign35610_e45752: f64 = (var_wsrhstep * var_wsrhstep);
        let assign35610_e45754: f64 = (var_wsrhstep).ln();
        let assign35610_e45755: f64 = (assign35610_e45752 * assign35610_e45754);
        let assign35610_e45758: f64 = (1.0 - var_wsrhstep);
        let assign35610_e45759: f64 = (assign35610_e45755 / assign35610_e45758);
        let assign35610_e45761: f64 = (assign35610_e45759 + var_wsrhstep);
        let assign35610_e45765: f64 = (2.0 * var_pbotd_i);
        let assign35610_e45766: f64 = (1.0 - assign35610_e45765);
        let assign35610_e45767: f64 = (assign35610_e45761 * assign35610_e45766);
        (assign35610_e45767,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign35610_e45769;

        let (assign35620_e45783,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 == 0.0)) {
        let assign35620_e45781: f64 = (var_wsrhstep + var_dwsrh);
        (assign35620_e45781,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign35620_e45783;

        let assign35630_e45786: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard714 = assign35630_e45786;

        let (assign35640_e45803, assign35640_e45803_d_n5, assign35640_e45803_d_n6, assign35640_e45803_d_n7, assign35640_e45803_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 == 0.0)) && (var_guard714 != 0.0)) {
        let assign35640_e45800: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign35640_e45801: f64 = (assign35640_e45800).sqrt();
        (assign35640_e45801, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35640_e45803;
        var_tmp_dn5 = assign35640_e45803_d_n5;
        var_tmp_dn6 = assign35640_e45803_d_n6;
        var_tmp_dn7 = assign35640_e45803_d_n7;
        var_tmp_dn8 = assign35640_e45803_d_n8;

        let (assign35650_e45822, assign35650_e45822_d_n5, assign35650_e45822_d_n6, assign35650_e45822_d_n7, assign35650_e45822_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 == 0.0)) && (var_guard714 == 0.0)) {
        let assign35650_e45818: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign35650_e45820: f64 = (assign35650_e45818).powf(var_pbotd_i);
        (assign35650_e45820, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35650_e45822;
        var_tmp_dn5 = assign35650_e45822_d_n5;
        var_tmp_dn6 = assign35650_e45822_d_n6;
        var_tmp_dn7 = assign35650_e45822_d_n7;
        var_tmp_dn8 = assign35650_e45822_d_n8;

        let (assign35660_e45836, assign35660_e45836_d_n5, assign35660_e45836_d_n6, assign35660_e45836_d_n7, assign35660_e45836_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 == 0.0)) {
        let assign35660_e45834: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign35660_e45834, (var_wdepnulrbot_d * var_tmp_dn5), (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign35660_e45836;
        var_wdep_dn5 = assign35660_e45836_d_n5;
        var_wdep_dn6 = assign35660_e45836_d_n6;
        var_wdep_dn7 = assign35660_e45836_d_n7;
        var_wdep_dn8 = assign35660_e45836_d_n8;

        let (assign35670_e45854, assign35670_e45854_d_n5, assign35670_e45854_d_n6, assign35670_e45854_d_n7, assign35670_e45854_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 == 0.0)) {
        let assign35670_e45849: f64 = (var_zinv - 1.0);
        let assign35670_e45851: f64 = (assign35670_e45849 * var_wdep);
        let assign35670_e45852: f64 = (var_ftdbot_d * assign35670_e45851);
        (assign35670_e45852, (var_ftdbot_d * (assign35670_e45849 * var_wdep_dn5)), (var_ftdbot_d * (assign35670_e45849 * var_wdep_dn6)), (var_ftdbot_d * (assign35670_e45849 * var_wdep_dn7)), (var_ftdbot_d * (assign35670_e45849 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign35670_e45854;
        var_asrh_dn5 = assign35670_e45854_d_n5;
        var_asrh_dn6 = assign35670_e45854_d_n6;
        var_asrh_dn7 = assign35670_e45854_d_n7;
        var_asrh_dn8 = assign35670_e45854_d_n8;

        let (assign35680_e45870, assign35680_e45870_d_n5, assign35680_e45870_d_n6, assign35680_e45870_d_n7, assign35680_e45870_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard712 == 0.0)) {
        let assign35680_e45867: f64 = (var_asrh * var_wsrh);
        let assign35680_e45868: f64 = (var_csrhbotd_i * assign35680_e45867);
        (assign35680_e45868, (var_csrhbotd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign35680_e45870;
        var_isrh_dn5 = assign35680_e45870_d_n5;
        var_isrh_dn6 = assign35680_e45870_d_n6;
        var_isrh_dn7 = assign35680_e45870_d_n7;
        var_isrh_dn8 = assign35680_e45870_d_n8;

        let assign35690_e45873: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard715 = assign35690_e45873;

        let (assign35700_e45884, assign35700_e45884_d_n5, assign35700_e45884_d_n6, assign35700_e45884_d_n7, assign35700_e45884_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign35700_e45884;
        var_itat_dn5 = assign35700_e45884_d_n5;
        var_itat_dn6 = assign35700_e45884_d_n6;
        var_itat_dn7 = assign35700_e45884_d_n7;
        var_itat_dn8 = assign35700_e45884_d_n8;

        let (assign35710_e45902, assign35710_e45902_d_n5, assign35710_e45902_d_n6, assign35710_e45902_d_n7, assign35710_e45902_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35710_e45897: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign35710_e45899: f64 = (assign35710_e45897 / var_vbi_minus_vjsrh);
        let assign35710_e45900: f64 = (var_btatpartbot_d * assign35710_e45899);
        (assign35710_e45900, (var_btatpartbot_d * ((var_wdep_dn5 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign35710_e45902;
        var_btat_dn5 = assign35710_e45902_d_n5;
        var_btat_dn6 = assign35710_e45902_d_n6;
        var_btat_dn7 = assign35710_e45902_d_n7;
        var_btat_dn8 = assign35710_e45902_d_n8;

        let (assign35720_e45918, assign35720_e45918_d_n5, assign35720_e45918_d_n6, assign35720_e45918_d_n7, assign35720_e45918_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35720_e45914: f64 = (0.666666666666667 * var_atatbot_d);
        let assign35720_e45916: f64 = (assign35720_e45914 / var_btat);
        (assign35720_e45916, (-((assign35720_e45914 * var_btat_dn5) / (var_btat * var_btat))), (-((assign35720_e45914 * var_btat_dn6) / (var_btat * var_btat))), (-((assign35720_e45914 * var_btat_dn7) / (var_btat * var_btat))), (-((assign35720_e45914 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign35720_e45918;
        var_twoatatoverthreebtat_dn5 = assign35720_e45918_d_n5;
        var_twoatatoverthreebtat_dn6 = assign35720_e45918_d_n6;
        var_twoatatoverthreebtat_dn7 = assign35720_e45918_d_n7;
        var_twoatatoverthreebtat_dn8 = assign35720_e45918_d_n8;

        let (assign35730_e45932, assign35730_e45932_d_n5, assign35730_e45932_d_n6, assign35730_e45932_d_n7, assign35730_e45932_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35730_e45930: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign35730_e45930, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign35730_e45932;
        var_umaxbeforelimiting_dn5 = assign35730_e45932_d_n5;
        var_umaxbeforelimiting_dn6 = assign35730_e45932_d_n6;
        var_umaxbeforelimiting_dn7 = assign35730_e45932_d_n7;
        var_umaxbeforelimiting_dn8 = assign35730_e45932_d_n8;

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
        *var_guard707_slot = var_guard707;
        *var_guard708_slot = var_guard708;
        *var_guard709_slot = var_guard709;
        *var_guard710_slot = var_guard710;
        *var_guard711_slot = var_guard711;
        *var_guard712_slot = var_guard712;
        *var_guard713_slot = var_guard713;
        *var_guard714_slot = var_guard714;
        *var_guard715_slot = var_guard715;
        *var_id__blk219_slot = var_id__blk219;
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
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
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

    pub(super) fn stamp_transient_block_74(
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
        var_guard182: f64,
        var_guard199: f64,
        var_guard711: f64,
        var_guard715: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_perfc: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn5: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_umaxbeforelimiting: f64,
        var_umaxbeforelimiting_dn5: f64,
        var_umaxbeforelimiting_dn6: f64,
        var_umaxbeforelimiting_dn7: f64,
        var_umaxbeforelimiting_dn8: f64,
        var_vbbt: f64,
        var_vbirbotd_i: f64,
        var_vbirbotinv_d: f64,
        var_wdepnulrinvbot_d: f64,
        var_wsrh: f64,
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
        var_guard716_slot: &mut f64,
        var_guard717_slot: &mut f64,
        var_guard718_slot: &mut f64,
        var_guard719_slot: &mut f64,
        var_guard720_slot: &mut f64,
        var_guard721_slot: &mut f64,
        var_guard722_slot: &mut f64,
        var_guard723_slot: &mut f64,
        var_guard724_slot: &mut f64,
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
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
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
        let mut var_guard716: f64 = *var_guard716_slot;
        let mut var_guard717: f64 = *var_guard717_slot;
        let mut var_guard718: f64 = *var_guard718_slot;
        let mut var_guard719: f64 = *var_guard719_slot;
        let mut var_guard720: f64 = *var_guard720_slot;
        let mut var_guard721: f64 = *var_guard721_slot;
        let mut var_guard722: f64 = *var_guard722_slot;
        let mut var_guard723: f64 = *var_guard723_slot;
        let mut var_guard724: f64 = *var_guard724_slot;
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
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
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

        let (assign35740_e45953, assign35740_e45953_d_n5, assign35740_e45953_d_n6, assign35740_e45953_d_n7, assign35740_e45953_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35740_e45944: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign35740_e45947: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign35740_e45949: f64 = (assign35740_e45947 + 1.0);
        let assign35740_e45950: f64 = (assign35740_e45944 / assign35740_e45949);
        let assign35740_e45951: f64 = (assign35740_e45950).sqrt();
        (assign35740_e45951, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign35740_e45949) - (assign35740_e45944 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign35740_e45949 * assign35740_e45949)) / (2.0 * assign35740_e45951)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign35740_e45949) - (assign35740_e45944 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign35740_e45949 * assign35740_e45949)) / (2.0 * assign35740_e45951)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign35740_e45949) - (assign35740_e45944 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign35740_e45949 * assign35740_e45949)) / (2.0 * assign35740_e45951)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign35740_e45949) - (assign35740_e45944 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign35740_e45949 * assign35740_e45949)) / (2.0 * assign35740_e45951)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign35740_e45953;
        var_umax_dn5 = assign35740_e45953_d_n5;
        var_umax_dn6 = assign35740_e45953_d_n6;
        var_umax_dn7 = assign35740_e45953_d_n7;
        var_umax_dn8 = assign35740_e45953_d_n8;

        let (assign35750_e45966, assign35750_e45966_d_n5, assign35750_e45966_d_n6, assign35750_e45966_d_n7, assign35750_e45966_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35750_e45964: f64 = (var_umax).sqrt();
        (assign35750_e45964, (var_umax_dn5 / (2.0 * assign35750_e45964)), (var_umax_dn6 / (2.0 * assign35750_e45964)), (var_umax_dn7 / (2.0 * assign35750_e45964)), (var_umax_dn8 / (2.0 * assign35750_e45964)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign35750_e45966;
        var_sqrtumax_dn5 = assign35750_e45966_d_n5;
        var_sqrtumax_dn6 = assign35750_e45966_d_n6;
        var_sqrtumax_dn7 = assign35750_e45966_d_n7;
        var_sqrtumax_dn8 = assign35750_e45966_d_n8;

        let (assign35760_e45980, assign35760_e45980_d_n5, assign35760_e45980_d_n6, assign35760_e45980_d_n7, assign35760_e45980_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35760_e45978: f64 = (var_umax * var_sqrtumax);
        (assign35760_e45978, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign35760_e45980;
        var_umaxpoweronepointfive_dn5 = assign35760_e45980_d_n5;
        var_umaxpoweronepointfive_dn6 = assign35760_e45980_d_n6;
        var_umaxpoweronepointfive_dn7 = assign35760_e45980_d_n7;
        var_umaxpoweronepointfive_dn8 = assign35760_e45980_d_n8;

        let assign35770_e45982: f64 = (-var_pbotd_i);
        let assign35770_e45984: f64 = (assign35770_e45982 * var_one_over_one_minus_pbot_d);
        let assign35770_e45986: f64 = (-1.0);
        let assign35770_e45987: f64 = if assign35770_e45984 == assign35770_e45986 { 1.0 } else { 0.0 };
        var_guard716 = assign35770_e45987;

        let (assign35780_e46007, assign35780_e46007_d_n5, assign35780_e46007_d_n6, assign35780_e46007_d_n7, assign35780_e46007_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) && (var_guard716 != 0.0)) {
        let assign35780_e46003: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign35780_e46004: f64 = (1.0 + assign35780_e46003);
        let assign35780_e46005: f64 = (1.0 / assign35780_e46004);
        (assign35780_e46005, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign35780_e46004 * assign35780_e46004))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign35780_e46004 * assign35780_e46004))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign35780_e46004 * assign35780_e46004))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign35780_e46004 * assign35780_e46004))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign35780_e46007;
        var_wgamma_dn5 = assign35780_e46007_d_n5;
        var_wgamma_dn6 = assign35780_e46007_d_n6;
        var_wgamma_dn7 = assign35780_e46007_d_n7;
        var_wgamma_dn8 = assign35780_e46007_d_n8;

        let (assign35790_e46031, assign35790_e46031_d_n5, assign35790_e46031_d_n6, assign35790_e46031_d_n7, assign35790_e46031_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) && (var_guard716 == 0.0)) {
        let assign35790_e46023: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign35790_e46024: f64 = (1.0 + assign35790_e46023);
        let assign35790_e46026: f64 = (-var_pbotd_i);
        let assign35790_e46028: f64 = (assign35790_e46026 * var_one_over_one_minus_pbot_d);
        let assign35790_e46029: f64 = (assign35790_e46024).powf(assign35790_e46028);
        (assign35790_e46029, if 0.0 == 0.0 && ((assign35790_e46028) as f64).is_finite() && ((assign35790_e46028) as f64).fract() == 0.0 { if assign35790_e46028 == 0.0 { 0.0 } else { (assign35790_e46028 * ((assign35790_e46024).powf(assign35790_e46028 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign35790_e46029 * (assign35790_e46028 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign35790_e46024))) }, if 0.0 == 0.0 && ((assign35790_e46028) as f64).is_finite() && ((assign35790_e46028) as f64).fract() == 0.0 { if assign35790_e46028 == 0.0 { 0.0 } else { (assign35790_e46028 * ((assign35790_e46024).powf(assign35790_e46028 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign35790_e46029 * (assign35790_e46028 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign35790_e46024))) }, if 0.0 == 0.0 && ((assign35790_e46028) as f64).is_finite() && ((assign35790_e46028) as f64).fract() == 0.0 { if assign35790_e46028 == 0.0 { 0.0 } else { (assign35790_e46028 * ((assign35790_e46024).powf(assign35790_e46028 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign35790_e46029 * (assign35790_e46028 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign35790_e46024))) }, if 0.0 == 0.0 && ((assign35790_e46028) as f64).is_finite() && ((assign35790_e46028) as f64).fract() == 0.0 { if assign35790_e46028 == 0.0 { 0.0 } else { (assign35790_e46028 * ((assign35790_e46024).powf(assign35790_e46028 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign35790_e46029 * (assign35790_e46028 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign35790_e46024))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign35790_e46031;
        var_wgamma_dn5 = assign35790_e46031_d_n5;
        var_wgamma_dn6 = assign35790_e46031_d_n6;
        var_wgamma_dn7 = assign35790_e46031_d_n7;
        var_wgamma_dn8 = assign35790_e46031_d_n8;

        let (assign35800_e46049, assign35800_e46049_d_n5, assign35800_e46049_d_n6, assign35800_e46049_d_n7, assign35800_e46049_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35800_e46043: f64 = (var_wsrh * var_wgamma);
        let assign35800_e46046: f64 = (var_wsrh + var_wgamma);
        let assign35800_e46047: f64 = (assign35800_e46043 / assign35800_e46046);
        (assign35800_e46047, ((((var_wsrh * var_wgamma_dn5) * assign35800_e46046) - (assign35800_e46043 * var_wgamma_dn5)) / (assign35800_e46046 * assign35800_e46046)), ((((var_wsrh * var_wgamma_dn6) * assign35800_e46046) - (assign35800_e46043 * var_wgamma_dn6)) / (assign35800_e46046 * assign35800_e46046)), ((((var_wsrh * var_wgamma_dn7) * assign35800_e46046) - (assign35800_e46043 * var_wgamma_dn7)) / (assign35800_e46046 * assign35800_e46046)), ((((var_wsrh * var_wgamma_dn8) * assign35800_e46046) - (assign35800_e46043 * var_wgamma_dn8)) / (assign35800_e46046 * assign35800_e46046)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign35800_e46049;
        var_wtat_dn5 = assign35800_e46049_d_n5;
        var_wtat_dn6 = assign35800_e46049_d_n6;
        var_wtat_dn7 = assign35800_e46049_d_n7;
        var_wtat_dn8 = assign35800_e46049_d_n8;

        let (assign35810_e46066, assign35810_e46066_d_n5, assign35810_e46066_d_n6, assign35810_e46066_d_n7, assign35810_e46066_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35810_e46062: f64 = (var_btat / var_sqrtumax);
        let assign35810_e46063: f64 = (0.375 * assign35810_e46062);
        let assign35810_e46064: f64 = (assign35810_e46063).sqrt();
        (assign35810_e46064, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35810_e46064)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35810_e46064)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35810_e46064)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35810_e46064)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign35810_e46066;
        var_ktat_dn5 = assign35810_e46066_d_n5;
        var_ktat_dn6 = assign35810_e46066_d_n6;
        var_ktat_dn7 = assign35810_e46066_d_n7;
        var_ktat_dn8 = assign35810_e46066_d_n8;

        let (assign35820_e46084, assign35820_e46084_d_n5, assign35820_e46084_d_n6, assign35820_e46084_d_n7, assign35820_e46084_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35820_e46079: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign35820_e46080: f64 = (2.0 * assign35820_e46079);
        let assign35820_e46082: f64 = (assign35820_e46080 - var_umax);
        (assign35820_e46082, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign35820_e46084;
        var_ltat_dn5 = assign35820_e46084_d_n5;
        var_ltat_dn6 = assign35820_e46084_d_n6;
        var_ltat_dn7 = assign35820_e46084_d_n7;
        var_ltat_dn8 = assign35820_e46084_d_n8;

        let (assign35830_e46110, assign35830_e46110_d_n5, assign35830_e46110_d_n6, assign35830_e46110_d_n7, assign35830_e46110_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35830_e46096: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign35830_e46098: f64 = (assign35830_e46096 * var_sqrtumax);
        let assign35830_e46101: f64 = (var_atatbot_d * var_umax);
        let assign35830_e46102: f64 = (assign35830_e46098 - assign35830_e46101);
        let assign35830_e46106: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign35830_e46107: f64 = (0.5 * assign35830_e46106);
        let assign35830_e46108: f64 = (assign35830_e46102 + assign35830_e46107);
        (assign35830_e46108, (((((var_atatbot_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign35830_e46096 * var_sqrtumax_dn5)) - (var_atatbot_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign35830_e46096 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign35830_e46096 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign35830_e46096 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign35830_e46110;
        var_mtat_dn5 = assign35830_e46110_d_n5;
        var_mtat_dn6 = assign35830_e46110_d_n6;
        var_mtat_dn7 = assign35830_e46110_d_n7;
        var_mtat_dn8 = assign35830_e46110_d_n8;

        let (assign35840_e46126, assign35840_e46126_d_n5, assign35840_e46126_d_n6, assign35840_e46126_d_n7, assign35840_e46126_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35840_e46122: f64 = (var_ltat - 1.0);
        let assign35840_e46124: f64 = (assign35840_e46122 * var_ktat);
        (assign35840_e46124, ((var_ltat_dn5 * var_ktat) + (assign35840_e46122 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign35840_e46122 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign35840_e46122 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign35840_e46122 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign35840_e46126;
        var_xerfc_dn5 = assign35840_e46126_d_n5;
        var_xerfc_dn6 = assign35840_e46126_d_n6;
        var_xerfc_dn7 = assign35840_e46126_d_n7;
        var_xerfc_dn8 = assign35840_e46126_d_n8;

        let (assign35850_e46140, assign35850_e46140_d_n5, assign35850_e46140_d_n6, assign35850_e46140_d_n7, assign35850_e46140_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35850_e46138: f64 = (var_xerfc * var_xerfc);
        (assign35850_e46138, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign35850_e46140;
        var_ysq_dn5 = assign35850_e46140_d_n5;
        var_ysq_dn6 = assign35850_e46140_d_n6;
        var_ysq_dn7 = assign35850_e46140_d_n7;
        var_ysq_dn8 = assign35850_e46140_d_n8;

        let assign35860_e46143: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard717 = assign35860_e46143;

        let (assign35870_e46163, assign35870_e46163_d_n5, assign35870_e46163_d_n6, assign35870_e46163_d_n7, assign35870_e46163_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) && (var_guard717 != 0.0)) {
        let assign35870_e46159: f64 = (var_perfc * var_xerfc);
        let assign35870_e46160: f64 = (1.0 + assign35870_e46159);
        let assign35870_e46161: f64 = (1.0 / assign35870_e46160);
        (assign35870_e46161, (-((var_perfc * var_xerfc_dn5) / (assign35870_e46160 * assign35870_e46160))), (-((var_perfc * var_xerfc_dn6) / (assign35870_e46160 * assign35870_e46160))), (-((var_perfc * var_xerfc_dn7) / (assign35870_e46160 * assign35870_e46160))), (-((var_perfc * var_xerfc_dn8) / (assign35870_e46160 * assign35870_e46160))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign35870_e46163;
        var_terfc_dn5 = assign35870_e46163_d_n5;
        var_terfc_dn6 = assign35870_e46163_d_n6;
        var_terfc_dn7 = assign35870_e46163_d_n7;
        var_terfc_dn8 = assign35870_e46163_d_n8;

        let (assign35880_e46184, assign35880_e46184_d_n5, assign35880_e46184_d_n6, assign35880_e46184_d_n7, assign35880_e46184_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) && (var_guard717 == 0.0)) {
        let assign35880_e46180: f64 = (var_perfc * var_xerfc);
        let assign35880_e46181: f64 = (1.0 - assign35880_e46180);
        let assign35880_e46182: f64 = (1.0 / assign35880_e46181);
        (assign35880_e46182, (-((-(var_perfc * var_xerfc_dn5)) / (assign35880_e46181 * assign35880_e46181))), (-((-(var_perfc * var_xerfc_dn6)) / (assign35880_e46181 * assign35880_e46181))), (-((-(var_perfc * var_xerfc_dn7)) / (assign35880_e46181 * assign35880_e46181))), (-((-(var_perfc * var_xerfc_dn8)) / (assign35880_e46181 * assign35880_e46181))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign35880_e46184;
        var_terfc_dn5 = assign35880_e46184_d_n5;
        var_terfc_dn6 = assign35880_e46184_d_n6;
        var_terfc_dn7 = assign35880_e46184_d_n7;
        var_terfc_dn8 = assign35880_e46184_d_n8;

        let assign35890_e46186: f64 = (-var_ysq);
        let assign35890_e46188: f64 = (assign35890_e46186 + var_mtat);
        let assign35890_e46190: f64 = (-230.25850929940458);
        let assign35890_e46191: f64 = if assign35890_e46188 > assign35890_e46190 { 1.0 } else { 0.0 };
        var_guard718 = assign35890_e46191;

        let (assign35900_e46209, assign35900_e46209_d_n5, assign35900_e46209_d_n6, assign35900_e46209_d_n7, assign35900_e46209_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) && (var_guard718 != 0.0)) {
        let assign35900_e46204: f64 = (-var_ysq);
        let assign35900_e46206: f64 = (assign35900_e46204 + var_mtat);
        let assign35900_e46207: f64 = (assign35900_e46206).exp();
        (assign35900_e46207, (assign35900_e46207 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign35900_e46207 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign35900_e46207 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign35900_e46207 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35900_e46209;
        var_tmp_dn5 = assign35900_e46209_d_n5;
        var_tmp_dn6 = assign35900_e46209_d_n6;
        var_tmp_dn7 = assign35900_e46209_d_n7;
        var_tmp_dn8 = assign35900_e46209_d_n8;

        let (assign35910_e46258, assign35910_e46258_d_n5, assign35910_e46258_d_n6, assign35910_e46258_d_n7, assign35910_e46258_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) && (var_guard718 == 0.0)) {
        let assign35910_e46225: f64 = (-230.25850929940458);
        let assign35910_e46227: f64 = (-var_ysq);
        let assign35910_e46229: f64 = (assign35910_e46227 + var_mtat);
        let assign35910_e46230: f64 = (assign35910_e46225 - assign35910_e46229);
        let assign35910_e46234: f64 = (-230.25850929940458);
        let assign35910_e46236: f64 = (-var_ysq);
        let assign35910_e46238: f64 = (assign35910_e46236 + var_mtat);
        let assign35910_e46239: f64 = (assign35910_e46234 - assign35910_e46238);
        let assign35910_e46242: f64 = (-230.25850929940458);
        let assign35910_e46244: f64 = (-var_ysq);
        let assign35910_e46246: f64 = (assign35910_e46244 + var_mtat);
        let assign35910_e46247: f64 = (assign35910_e46242 - assign35910_e46246);
        let assign35910_e46249: f64 = (assign35910_e46247 * 0.3333333333333333);
        let assign35910_e46250: f64 = (1.0 + assign35910_e46249);
        let assign35910_e46251: f64 = (assign35910_e46239 * assign35910_e46250);
        let assign35910_e46252: f64 = (0.5 * assign35910_e46251);
        let assign35910_e46253: f64 = (1.0 + assign35910_e46252);
        let assign35910_e46254: f64 = (assign35910_e46230 * assign35910_e46253);
        let assign35910_e46255: f64 = (1.0 + assign35910_e46254);
        let assign35910_e46256: f64 = (1e-100 / assign35910_e46255);
        (assign35910_e46256, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign35910_e46253) + (assign35910_e46230 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign35910_e46250) + (assign35910_e46239 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign35910_e46255 * assign35910_e46255))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign35910_e46253) + (assign35910_e46230 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign35910_e46250) + (assign35910_e46239 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign35910_e46255 * assign35910_e46255))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign35910_e46253) + (assign35910_e46230 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign35910_e46250) + (assign35910_e46239 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign35910_e46255 * assign35910_e46255))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign35910_e46253) + (assign35910_e46230 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign35910_e46250) + (assign35910_e46239 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign35910_e46255 * assign35910_e46255))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35910_e46258;
        var_tmp_dn5 = assign35910_e46258_d_n5;
        var_tmp_dn6 = assign35910_e46258_d_n6;
        var_tmp_dn7 = assign35910_e46258_d_n7;
        var_tmp_dn8 = assign35910_e46258_d_n8;

        let (assign35920_e46288, assign35920_e46288_d_n5, assign35920_e46288_d_n6, assign35920_e46288_d_n7, assign35920_e46288_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35920_e46270: f64 = (0.29214664 * var_terfc);
        let assign35920_e46274: f64 = (var_terfc * var_terfc);
        let assign35920_e46275: f64 = (var_berfc * assign35920_e46274);
        let assign35920_e46276: f64 = (assign35920_e46270 + assign35920_e46275);
        let assign35920_e46280: f64 = (var_terfc * var_terfc);
        let assign35920_e46282: f64 = (assign35920_e46280 * var_terfc);
        let assign35920_e46283: f64 = (var_cerfc * assign35920_e46282);
        let assign35920_e46284: f64 = (assign35920_e46276 + assign35920_e46283);
        let assign35920_e46286: f64 = (assign35920_e46284 * var_tmp);
        (assign35920_e46286, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign35920_e46280 * var_terfc_dn5)))) * var_tmp) + (assign35920_e46284 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign35920_e46280 * var_terfc_dn6)))) * var_tmp) + (assign35920_e46284 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign35920_e46280 * var_terfc_dn7)))) * var_tmp) + (assign35920_e46284 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign35920_e46280 * var_terfc_dn8)))) * var_tmp) + (assign35920_e46284 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign35920_e46288;
        var_erfcpos_dn5 = assign35920_e46288_d_n5;
        var_erfcpos_dn6 = assign35920_e46288_d_n6;
        var_erfcpos_dn7 = assign35920_e46288_d_n7;
        var_erfcpos_dn8 = assign35920_e46288_d_n8;

        let assign35930_e46291: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard719 = assign35930_e46291;

        let (assign35940_e46305, assign35940_e46305_d_n5, assign35940_e46305_d_n6, assign35940_e46305_d_n7, assign35940_e46305_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) && (var_guard719 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign35940_e46305;
        var_erfctimesexpmtat_dn5 = assign35940_e46305_d_n5;
        var_erfctimesexpmtat_dn6 = assign35940_e46305_d_n6;
        var_erfctimesexpmtat_dn7 = assign35940_e46305_d_n7;
        var_erfctimesexpmtat_dn8 = assign35940_e46305_d_n8;

        let assign35950_e46308: f64 = (-230.25850929940458);
        let assign35950_e46309: f64 = if var_mtat > assign35950_e46308 { 1.0 } else { 0.0 };
        var_guard720 = assign35950_e46309;

        let (assign35960_e46327, assign35960_e46327_d_n5, assign35960_e46327_d_n6, assign35960_e46327_d_n7, assign35960_e46327_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) && (var_guard719 == 0.0)) && (var_guard720 != 0.0)) {
        let assign35960_e46325: f64 = (var_mtat).exp();
        (assign35960_e46325, (assign35960_e46325 * var_mtat_dn5), (assign35960_e46325 * var_mtat_dn6), (assign35960_e46325 * var_mtat_dn7), (assign35960_e46325 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35960_e46327;
        var_tmp_dn5 = assign35960_e46327_d_n5;
        var_tmp_dn6 = assign35960_e46327_d_n6;
        var_tmp_dn7 = assign35960_e46327_d_n7;
        var_tmp_dn8 = assign35960_e46327_d_n8;

        let (assign35970_e46370, assign35970_e46370_d_n5, assign35970_e46370_d_n6, assign35970_e46370_d_n7, assign35970_e46370_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) && (var_guard719 == 0.0)) && (var_guard720 == 0.0)) {
        let assign35970_e46346: f64 = (-230.25850929940458);
        let assign35970_e46348: f64 = (assign35970_e46346 - var_mtat);
        let assign35970_e46352: f64 = (-230.25850929940458);
        let assign35970_e46354: f64 = (assign35970_e46352 - var_mtat);
        let assign35970_e46357: f64 = (-230.25850929940458);
        let assign35970_e46359: f64 = (assign35970_e46357 - var_mtat);
        let assign35970_e46361: f64 = (assign35970_e46359 * 0.3333333333333333);
        let assign35970_e46362: f64 = (1.0 + assign35970_e46361);
        let assign35970_e46363: f64 = (assign35970_e46354 * assign35970_e46362);
        let assign35970_e46364: f64 = (0.5 * assign35970_e46363);
        let assign35970_e46365: f64 = (1.0 + assign35970_e46364);
        let assign35970_e46366: f64 = (assign35970_e46348 * assign35970_e46365);
        let assign35970_e46367: f64 = (1.0 + assign35970_e46366);
        let assign35970_e46368: f64 = (1e-100 / assign35970_e46367);
        (assign35970_e46368, (-((1e-100 * (((-var_mtat_dn5) * assign35970_e46365) + (assign35970_e46348 * (0.5 * (((-var_mtat_dn5) * assign35970_e46362) + (assign35970_e46354 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign35970_e46367 * assign35970_e46367))), (-((1e-100 * (((-var_mtat_dn6) * assign35970_e46365) + (assign35970_e46348 * (0.5 * (((-var_mtat_dn6) * assign35970_e46362) + (assign35970_e46354 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign35970_e46367 * assign35970_e46367))), (-((1e-100 * (((-var_mtat_dn7) * assign35970_e46365) + (assign35970_e46348 * (0.5 * (((-var_mtat_dn7) * assign35970_e46362) + (assign35970_e46354 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign35970_e46367 * assign35970_e46367))), (-((1e-100 * (((-var_mtat_dn8) * assign35970_e46365) + (assign35970_e46348 * (0.5 * (((-var_mtat_dn8) * assign35970_e46362) + (assign35970_e46354 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign35970_e46367 * assign35970_e46367))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign35970_e46370;
        var_tmp_dn5 = assign35970_e46370_d_n5;
        var_tmp_dn6 = assign35970_e46370_d_n6;
        var_tmp_dn7 = assign35970_e46370_d_n7;
        var_tmp_dn8 = assign35970_e46370_d_n8;

        let (assign35980_e46389, assign35980_e46389_d_n5, assign35980_e46389_d_n6, assign35980_e46389_d_n7, assign35980_e46389_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) && (var_guard719 == 0.0)) {
        let assign35980_e46385: f64 = (2.0 * var_tmp);
        let assign35980_e46387: f64 = (assign35980_e46385 - var_erfcpos);
        (assign35980_e46387, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign35980_e46389;
        var_erfctimesexpmtat_dn5 = assign35980_e46389_d_n5;
        var_erfctimesexpmtat_dn6 = assign35980_e46389_d_n6;
        var_erfctimesexpmtat_dn7 = assign35980_e46389_d_n7;
        var_erfctimesexpmtat_dn8 = assign35980_e46389_d_n8;

        let (assign35990_e46409, assign35990_e46409_d_n5, assign35990_e46409_d_n6, assign35990_e46409_d_n7, assign35990_e46409_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign35990_e46401: f64 = (1.772453850905516 * 0.5);
        let assign35990_e46404: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign35990_e46406: f64 = (assign35990_e46404 / var_ktat);
        let assign35990_e46407: f64 = (assign35990_e46401 * assign35990_e46406);
        (assign35990_e46407, (assign35990_e46401 * ((((var_atatbot_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign35990_e46404 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign35990_e46401 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign35990_e46404 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign35990_e46401 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign35990_e46404 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign35990_e46401 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign35990_e46404 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign35990_e46409;
        var_gammamax_dn5 = assign35990_e46409_d_n5;
        var_gammamax_dn6 = assign35990_e46409_d_n6;
        var_gammamax_dn7 = assign35990_e46409_d_n7;
        var_gammamax_dn8 = assign35990_e46409_d_n8;

        let (assign36000_e46427, assign36000_e46427_d_n5, assign36000_e46427_d_n6, assign36000_e46427_d_n7, assign36000_e46427_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard715 == 0.0)) {
        let assign36000_e46422: f64 = (var_asrh * var_gammamax);
        let assign36000_e46424: f64 = (assign36000_e46422 * var_wtat);
        let assign36000_e46425: f64 = (var_ctatbotd_i * assign36000_e46424);
        (assign36000_e46425, (var_ctatbotd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign36000_e46422 * var_wtat_dn5))), (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign36000_e46422 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign36000_e46422 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign36000_e46422 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign36000_e46427;
        var_itat_dn5 = assign36000_e46427_d_n5;
        var_itat_dn6 = assign36000_e46427_d_n6;
        var_itat_dn7 = assign36000_e46427_d_n7;
        var_itat_dn8 = assign36000_e46427_d_n8;

        let assign36010_e46430: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard721 = assign36010_e46430;

        let (assign36020_e46441, assign36020_e46441_d_n5, assign36020_e46441_d_n6, assign36020_e46441_d_n7, assign36020_e46441_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard721 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign36020_e46441;
        var_ibbt_dn5 = assign36020_e46441_d_n5;
        var_ibbt_dn6 = assign36020_e46441_d_n6;
        var_ibbt_dn7 = assign36020_e46441_d_n7;
        var_ibbt_dn8 = assign36020_e46441_d_n8;

        let assign36030_e46444: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard722 = assign36030_e46444;

        let (assign36040_e46463, assign36040_e46463_d_n5, assign36040_e46463_d_n6, assign36040_e46463_d_n7, assign36040_e46463_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard721 == 0.0)) && (var_guard722 != 0.0)) {
        let assign36040_e46458: f64 = (var_vbirbotd_i - var_vbbt);
        let assign36040_e46460: f64 = (assign36040_e46458 * var_vbirbotinv_d);
        let assign36040_e46461: f64 = (assign36040_e46460).sqrt();
        (assign36040_e46461, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36040_e46463;
        var_tmp_dn5 = assign36040_e46463_d_n5;
        var_tmp_dn6 = assign36040_e46463_d_n6;
        var_tmp_dn7 = assign36040_e46463_d_n7;
        var_tmp_dn8 = assign36040_e46463_d_n8;

        let (assign36050_e46484, assign36050_e46484_d_n5, assign36050_e46484_d_n6, assign36050_e46484_d_n7, assign36050_e46484_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) {
        let assign36050_e46478: f64 = (var_vbirbotd_i - var_vbbt);
        let assign36050_e46480: f64 = (assign36050_e46478 * var_vbirbotinv_d);
        let assign36050_e46482: f64 = (assign36050_e46480).powf(var_pbotd_i);
        (assign36050_e46482, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36050_e46484;
        var_tmp_dn5 = assign36050_e46484_d_n5;
        var_tmp_dn6 = assign36050_e46484_d_n6;
        var_tmp_dn7 = assign36050_e46484_d_n7;
        var_tmp_dn8 = assign36050_e46484_d_n8;

        let (assign36060_e46504, assign36060_e46504_d_n5, assign36060_e46504_d_n6, assign36060_e46504_d_n7, assign36060_e46504_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard721 == 0.0)) {
        let assign36060_e46497: f64 = (var_vbirbotd_i - var_vbbt);
        let assign36060_e46499: f64 = (assign36060_e46497 * var_wdepnulrinvbot_d);
        let assign36060_e46501: f64 = (assign36060_e46499 / var_tmp);
        let assign36060_e46502: f64 = (var_one_over_one_minus_pbot_d * assign36060_e46501);
        (assign36060_e46502, (var_one_over_one_minus_pbot_d * (-((assign36060_e46499 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign36060_e46499 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign36060_e46499 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign36060_e46499 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign36060_e46504;
        var_fmaxr_dn5 = assign36060_e46504_d_n5;
        var_fmaxr_dn6 = assign36060_e46504_d_n6;
        var_fmaxr_dn7 = assign36060_e46504_d_n7;
        var_fmaxr_dn8 = assign36060_e46504_d_n8;

        let assign36070_e46506: f64 = (-var_fbbtbot_d);
        let assign36070_e46508: f64 = (assign36070_e46506 / var_fmaxr);
        let assign36070_e46509: f64 = (assign36070_e46508).abs();
        let assign36070_e46511: f64 = if assign36070_e46509 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard723 = assign36070_e46511;

        let (assign36080_e46529, assign36080_e46529_d_n5, assign36080_e46529_d_n6, assign36080_e46529_d_n7, assign36080_e46529_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard721 == 0.0)) && (var_guard723 != 0.0)) {
        let assign36080_e46524: f64 = (-var_fbbtbot_d);
        let assign36080_e46526: f64 = (assign36080_e46524 / var_fmaxr);
        let assign36080_e46527: f64 = (assign36080_e46526).exp();
        (assign36080_e46527, (assign36080_e46527 * (-((assign36080_e46524 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign36080_e46527 * (-((assign36080_e46524 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign36080_e46527 * (-((assign36080_e46524 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign36080_e46527 * (-((assign36080_e46524 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36080_e46529;
        var_tmp_dn5 = assign36080_e46529_d_n5;
        var_tmp_dn6 = assign36080_e46529_d_n6;
        var_tmp_dn7 = assign36080_e46529_d_n7;
        var_tmp_dn8 = assign36080_e46529_d_n8;

        let assign36090_e46531: f64 = (-var_fbbtbot_d);
        let assign36090_e46533: f64 = (assign36090_e46531 / var_fmaxr);
        let assign36090_e46535: f64 = if assign36090_e46533 < 0.0 { 1.0 } else { 0.0 };
        var_guard724 = assign36090_e46535;

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
        *var_guard716_slot = var_guard716;
        *var_guard717_slot = var_guard717;
        *var_guard718_slot = var_guard718;
        *var_guard719_slot = var_guard719;
        *var_guard720_slot = var_guard720;
        *var_guard721_slot = var_guard721;
        *var_guard722_slot = var_guard722;
        *var_guard723_slot = var_guard723;
        *var_guard724_slot = var_guard724;
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
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
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

    pub(super) fn stamp_transient_block_75(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti_d: f64,
        var_btatpartsti_d: f64,
        var_cbbtbotd_i: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_fbbtbot_d: f64,
        var_fmaxr: f64,
        var_fmaxr_dn5: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fstopbot_d: f64,
        var_ftdsti_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard711: f64,
        var_guard721: f64,
        var_guard723: f64,
        var_guard724: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
        var_lsdrain_i: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrbotd_i: f64,
        var_pstid_i: f64,
        var_slopebot_d: f64,
        var_two_psistar: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbirstiinv_d: f64,
        var_vbisti_d: f64,
        var_vbrbotd_i: f64,
        var_vbrinvbot_d: f64,
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
        var_guard725_slot: &mut f64,
        var_guard726_slot: &mut f64,
        var_guard727_slot: &mut f64,
        var_guard728_slot: &mut f64,
        var_guard729_slot: &mut f64,
        var_guard730_slot: &mut f64,
        var_guard731_slot: &mut f64,
        var_guard732_slot: &mut f64,
        var_guard733_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard725: f64 = *var_guard725_slot;
        let mut var_guard726: f64 = *var_guard726_slot;
        let mut var_guard727: f64 = *var_guard727_slot;
        let mut var_guard728: f64 = *var_guard728_slot;
        let mut var_guard729: f64 = *var_guard729_slot;
        let mut var_guard730: f64 = *var_guard730_slot;
        let mut var_guard731: f64 = *var_guard731_slot;
        let mut var_guard732: f64 = *var_guard732_slot;
        let mut var_guard733: f64 = *var_guard733_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let (assign36100_e46586, assign36100_e46586_d_n5, assign36100_e46586_d_n6, assign36100_e46586_d_n7, assign36100_e46586_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard721 == 0.0)) && (var_guard723 == 0.0)) && (var_guard724 != 0.0)) {
        let assign36100_e46553: f64 = (-230.25850929940458);
        let assign36100_e46555: f64 = (-var_fbbtbot_d);
        let assign36100_e46557: f64 = (assign36100_e46555 / var_fmaxr);
        let assign36100_e46558: f64 = (assign36100_e46553 - assign36100_e46557);
        let assign36100_e46562: f64 = (-230.25850929940458);
        let assign36100_e46564: f64 = (-var_fbbtbot_d);
        let assign36100_e46566: f64 = (assign36100_e46564 / var_fmaxr);
        let assign36100_e46567: f64 = (assign36100_e46562 - assign36100_e46566);
        let assign36100_e46570: f64 = (-230.25850929940458);
        let assign36100_e46572: f64 = (-var_fbbtbot_d);
        let assign36100_e46574: f64 = (assign36100_e46572 / var_fmaxr);
        let assign36100_e46575: f64 = (assign36100_e46570 - assign36100_e46574);
        let assign36100_e46577: f64 = (assign36100_e46575 * 0.3333333333333333);
        let assign36100_e46578: f64 = (1.0 + assign36100_e46577);
        let assign36100_e46579: f64 = (assign36100_e46567 * assign36100_e46578);
        let assign36100_e46580: f64 = (0.5 * assign36100_e46579);
        let assign36100_e46581: f64 = (1.0 + assign36100_e46580);
        let assign36100_e46582: f64 = (assign36100_e46558 * assign36100_e46581);
        let assign36100_e46583: f64 = (1.0 + assign36100_e46582);
        let assign36100_e46584: f64 = (1e-100 / assign36100_e46583);
        (assign36100_e46584, (-((1e-100 * (((-(-((assign36100_e46555 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign36100_e46581) + (assign36100_e46558 * (0.5 * (((-(-((assign36100_e46564 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign36100_e46578) + (assign36100_e46567 * ((-(-((assign36100_e46572 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36100_e46583 * assign36100_e46583))), (-((1e-100 * (((-(-((assign36100_e46555 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign36100_e46581) + (assign36100_e46558 * (0.5 * (((-(-((assign36100_e46564 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign36100_e46578) + (assign36100_e46567 * ((-(-((assign36100_e46572 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36100_e46583 * assign36100_e46583))), (-((1e-100 * (((-(-((assign36100_e46555 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign36100_e46581) + (assign36100_e46558 * (0.5 * (((-(-((assign36100_e46564 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign36100_e46578) + (assign36100_e46567 * ((-(-((assign36100_e46572 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36100_e46583 * assign36100_e46583))), (-((1e-100 * (((-(-((assign36100_e46555 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign36100_e46581) + (assign36100_e46558 * (0.5 * (((-(-((assign36100_e46564 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign36100_e46578) + (assign36100_e46567 * ((-(-((assign36100_e46572 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36100_e46583 * assign36100_e46583))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36100_e46586;
        var_tmp_dn5 = assign36100_e46586_d_n5;
        var_tmp_dn6 = assign36100_e46586_d_n6;
        var_tmp_dn7 = assign36100_e46586_d_n7;
        var_tmp_dn8 = assign36100_e46586_d_n8;

        let (assign36110_e46635, assign36110_e46635_d_n5, assign36110_e46635_d_n6, assign36110_e46635_d_n7, assign36110_e46635_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard721 == 0.0)) && (var_guard723 == 0.0)) && (var_guard724 == 0.0)) {
        let assign36110_e46605: f64 = (-var_fbbtbot_d);
        let assign36110_e46607: f64 = (assign36110_e46605 / var_fmaxr);
        let assign36110_e46609: f64 = (assign36110_e46607 - 230.25850929940458);
        let assign36110_e46613: f64 = (-var_fbbtbot_d);
        let assign36110_e46615: f64 = (assign36110_e46613 / var_fmaxr);
        let assign36110_e46617: f64 = (assign36110_e46615 - 230.25850929940458);
        let assign36110_e46620: f64 = (-var_fbbtbot_d);
        let assign36110_e46622: f64 = (assign36110_e46620 / var_fmaxr);
        let assign36110_e46624: f64 = (assign36110_e46622 - 230.25850929940458);
        let assign36110_e46626: f64 = (assign36110_e46624 * 0.3333333333333333);
        let assign36110_e46627: f64 = (1.0 + assign36110_e46626);
        let assign36110_e46628: f64 = (assign36110_e46617 * assign36110_e46627);
        let assign36110_e46629: f64 = (0.5 * assign36110_e46628);
        let assign36110_e46630: f64 = (1.0 + assign36110_e46629);
        let assign36110_e46631: f64 = (assign36110_e46609 * assign36110_e46630);
        let assign36110_e46632: f64 = (1.0 + assign36110_e46631);
        let assign36110_e46633: f64 = (1e100 * assign36110_e46632);
        (assign36110_e46633, (1e100 * (((-((assign36110_e46605 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign36110_e46630) + (assign36110_e46609 * (0.5 * (((-((assign36110_e46613 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign36110_e46627) + (assign36110_e46617 * ((-((assign36110_e46620 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36110_e46605 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign36110_e46630) + (assign36110_e46609 * (0.5 * (((-((assign36110_e46613 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign36110_e46627) + (assign36110_e46617 * ((-((assign36110_e46620 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36110_e46605 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign36110_e46630) + (assign36110_e46609 * (0.5 * (((-((assign36110_e46613 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign36110_e46627) + (assign36110_e46617 * ((-((assign36110_e46620 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36110_e46605 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign36110_e46630) + (assign36110_e46609 * (0.5 * (((-((assign36110_e46613 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign36110_e46627) + (assign36110_e46617 * ((-((assign36110_e46620 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36110_e46635;
        var_tmp_dn5 = assign36110_e46635_d_n5;
        var_tmp_dn6 = assign36110_e46635_d_n6;
        var_tmp_dn7 = assign36110_e46635_d_n7;
        var_tmp_dn8 = assign36110_e46635_d_n8;

        let (assign36120_e46655, assign36120_e46655_d_n5, assign36120_e46655_d_n6, assign36120_e46655_d_n7, assign36120_e46655_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard721 == 0.0)) {
        let assign36120_e46648: f64 = (var_v4 * var_fmaxr);
        let assign36120_e46650: f64 = (assign36120_e46648 * var_fmaxr);
        let assign36120_e46652: f64 = (assign36120_e46650 * var_tmp);
        let assign36120_e46653: f64 = (var_cbbtbotd_i * assign36120_e46652);
        (assign36120_e46653, (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign36120_e46648 * var_fmaxr_dn5)) * var_tmp) + (assign36120_e46650 * var_tmp_dn5))), (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign36120_e46648 * var_fmaxr_dn6)) * var_tmp) + (assign36120_e46650 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign36120_e46648 * var_fmaxr_dn7)) * var_tmp) + (assign36120_e46650 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign36120_e46648 * var_fmaxr_dn8)) * var_tmp) + (assign36120_e46650 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign36120_e46655;
        var_ibbt_dn5 = assign36120_e46655_d_n5;
        var_ibbt_dn6 = assign36120_e46655_d_n6;
        var_ibbt_dn7 = assign36120_e46655_d_n7;
        var_ibbt_dn8 = assign36120_e46655_d_n8;

        let assign36130_e46658: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard725 = assign36130_e46658;

        let (assign36140_e46669, assign36140_e46669_d_n5, assign36140_e46669_d_n6, assign36140_e46669_d_n7, assign36140_e46669_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard725 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36140_e46669;
        var_fbreakdown_dn5 = assign36140_e46669_d_n5;
        var_fbreakdown_dn6 = assign36140_e46669_d_n6;
        var_fbreakdown_dn7 = assign36140_e46669_d_n7;
        var_fbreakdown_dn8 = assign36140_e46669_d_n8;

        let assign36150_e46672: f64 = (-var_alphaav);
        let assign36150_e46674: f64 = (assign36150_e46672 * var_vbrbotd_i);
        let assign36150_e46675: f64 = if var_vav > assign36150_e46674 { 1.0 } else { 0.0 };
        var_guard726 = assign36150_e46675;

        let assign36160_e46678: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard727 = assign36160_e46678;

        let (assign36170_e46708, assign36170_e46708_d_n5, assign36170_e46708_d_n6, assign36170_e46708_d_n7, assign36170_e46708_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard725 == 0.0)) && (var_guard726 != 0.0)) && (var_guard727 != 0.0)) {
        let assign36170_e46694: f64 = (var_vav * var_vbrinvbot_d);
        let assign36170_e46697: f64 = (var_vav * var_vbrinvbot_d);
        let assign36170_e46698: f64 = (assign36170_e46694 * assign36170_e46697);
        let assign36170_e46701: f64 = (var_vav * var_vbrinvbot_d);
        let assign36170_e46702: f64 = (assign36170_e46698 * assign36170_e46701);
        let assign36170_e46705: f64 = (var_vav * var_vbrinvbot_d);
        let assign36170_e46706: f64 = (assign36170_e46702 * assign36170_e46705);
        (assign36170_e46706, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36170_e46708;
        var_tmp_dn5 = assign36170_e46708_d_n5;
        var_tmp_dn6 = assign36170_e46708_d_n6;
        var_tmp_dn7 = assign36170_e46708_d_n7;
        var_tmp_dn8 = assign36170_e46708_d_n8;

        let (assign36180_e46730, assign36180_e46730_d_n5, assign36180_e46730_d_n6, assign36180_e46730_d_n7, assign36180_e46730_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard725 == 0.0)) && (var_guard726 != 0.0)) && (var_guard727 == 0.0)) {
        let assign36180_e46725: f64 = (var_vav * var_vbrinvbot_d);
        let assign36180_e46726: f64 = (assign36180_e46725).abs();
        let assign36180_e46728: f64 = (assign36180_e46726).powf(var_pbrbotd_i);
        (assign36180_e46728, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36180_e46730;
        var_tmp_dn5 = assign36180_e46730_d_n5;
        var_tmp_dn6 = assign36180_e46730_d_n6;
        var_tmp_dn7 = assign36180_e46730_d_n7;
        var_tmp_dn8 = assign36180_e46730_d_n8;

        let (assign36190_e46748, assign36190_e46748_d_n5, assign36190_e46748_d_n6, assign36190_e46748_d_n7, assign36190_e46748_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard725 == 0.0)) && (var_guard726 != 0.0)) {
        let assign36190_e46745: f64 = (1.0 - var_tmp);
        let assign36190_e46746: f64 = (1.0 / assign36190_e46745);
        (assign36190_e46746, (-((-var_tmp_dn5) / (assign36190_e46745 * assign36190_e46745))), (-((-var_tmp_dn6) / (assign36190_e46745 * assign36190_e46745))), (-((-var_tmp_dn7) / (assign36190_e46745 * assign36190_e46745))), (-((-var_tmp_dn8) / (assign36190_e46745 * assign36190_e46745))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36190_e46748;
        var_fbreakdown_dn5 = assign36190_e46748_d_n5;
        var_fbreakdown_dn6 = assign36190_e46748_d_n6;
        var_fbreakdown_dn7 = assign36190_e46748_d_n7;
        var_fbreakdown_dn8 = assign36190_e46748_d_n8;

        let (assign36200_e46771, assign36200_e46771_d_n5, assign36200_e46771_d_n6, assign36200_e46771_d_n7, assign36200_e46771_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) && (var_guard725 == 0.0)) && (var_guard726 == 0.0)) {
        let assign36200_e46765: f64 = (var_alphaav * var_vbrbotd_i);
        let assign36200_e46766: f64 = (var_vav + assign36200_e46765);
        let assign36200_e46768: f64 = (assign36200_e46766 * var_slopebot_d);
        let assign36200_e46769: f64 = (var_fstopbot_d + assign36200_e46768);
        (assign36200_e46769, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36200_e46771;
        var_fbreakdown_dn5 = assign36200_e46771_d_n5;
        var_fbreakdown_dn6 = assign36200_e46771_d_n6;
        var_fbreakdown_dn7 = assign36200_e46771_d_n7;
        var_fbreakdown_dn8 = assign36200_e46771_d_n8;

        let (assign36210_e46790, assign36210_e46790_d_n5, assign36210_e46790_d_n6, assign36210_e46790_d_n7, assign36210_e46790_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard711 == 0.0)) {
        let assign36210_e46781: f64 = (var_id__blk219 + var_isrh);
        let assign36210_e46783: f64 = (assign36210_e46781 + var_itat);
        let assign36210_e46785: f64 = (assign36210_e46783 + var_ibbt);
        let assign36210_e46786: f64 = (p.p29 * assign36210_e46785);
        let assign36210_e46788: f64 = (assign36210_e46786 * var_fbreakdown);
        (assign36210_e46788, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign36210_e46786 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign36210_e46786 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign36210_e46786 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign36210_e46786 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign36210_e46790;
        var_ijunbot_dn5 = assign36210_e46790_d_n5;
        var_ijunbot_dn6 = assign36210_e46790_d_n6;
        var_ijunbot_dn7 = assign36210_e46790_d_n7;
        var_ijunbot_dn8 = assign36210_e46790_d_n8;

        let assign36220_e46793: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard728 = assign36220_e46793;

        let (assign36230_e46801, assign36230_e46801_d_n5, assign36230_e46801_d_n6, assign36230_e46801_d_n7, assign36230_e46801_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign36230_e46801;
        var_ijunsti_dn5 = assign36230_e46801_d_n5;
        var_ijunsti_dn6 = assign36230_e46801_d_n6;
        var_ijunsti_dn7 = assign36230_e46801_d_n7;
        var_ijunsti_dn8 = assign36230_e46801_d_n8;

        let (assign36240_e46812,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) {
        let assign36240_e46810: f64 = (var_idsatsti_d * var_idmult);
        (assign36240_e46810,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign36240_e46812;

        let assign36250_e46819: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard729 = assign36250_e46819;

        let (assign36260_e46830, assign36260_e46830_d_n5, assign36260_e46830_d_n6, assign36260_e46830_d_n7, assign36260_e46830_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign36260_e46830;
        var_isrh_dn5 = assign36260_e46830_d_n5;
        var_isrh_dn6 = assign36260_e46830_d_n6;
        var_isrh_dn7 = assign36260_e46830_d_n7;
        var_isrh_dn8 = assign36260_e46830_d_n8;

        let (assign36270_e46844,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 == 0.0)) {
        let assign36270_e46842: f64 = (var_vbisti_d - var_vjsrh);
        (assign36270_e46842,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign36270_e46844;

        let (assign36280_e46863,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 == 0.0)) {
        let assign36280_e46858: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign36280_e46859: f64 = (1.0 - assign36280_e46858);
        let assign36280_e46860: f64 = (assign36280_e46859).sqrt();
        let assign36280_e46861: f64 = (1.0 - assign36280_e46860);
        (assign36280_e46861,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign36280_e46863;

        let assign36290_e46866: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard730 = assign36290_e46866;

        let (assign36300_e46880,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 == 0.0)) && (var_guard730 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign36300_e46880;

        let (assign36310_e46912,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 == 0.0)) && (var_guard730 == 0.0)) {
        let assign36310_e46895: f64 = (var_wsrhstep * var_wsrhstep);
        let assign36310_e46897: f64 = (var_wsrhstep).ln();
        let assign36310_e46898: f64 = (assign36310_e46895 * assign36310_e46897);
        let assign36310_e46901: f64 = (1.0 - var_wsrhstep);
        let assign36310_e46902: f64 = (assign36310_e46898 / assign36310_e46901);
        let assign36310_e46904: f64 = (assign36310_e46902 + var_wsrhstep);
        let assign36310_e46908: f64 = (2.0 * var_pstid_i);
        let assign36310_e46909: f64 = (1.0 - assign36310_e46908);
        let assign36310_e46910: f64 = (assign36310_e46904 * assign36310_e46909);
        (assign36310_e46910,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign36310_e46912;

        let (assign36320_e46926,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 == 0.0)) {
        let assign36320_e46924: f64 = (var_wsrhstep + var_dwsrh);
        (assign36320_e46924,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign36320_e46926;

        let assign36330_e46929: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard731 = assign36330_e46929;

        let (assign36340_e46946, assign36340_e46946_d_n5, assign36340_e46946_d_n6, assign36340_e46946_d_n7, assign36340_e46946_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 == 0.0)) && (var_guard731 != 0.0)) {
        let assign36340_e46943: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign36340_e46944: f64 = (assign36340_e46943).sqrt();
        (assign36340_e46944, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36340_e46946;
        var_tmp_dn5 = assign36340_e46946_d_n5;
        var_tmp_dn6 = assign36340_e46946_d_n6;
        var_tmp_dn7 = assign36340_e46946_d_n7;
        var_tmp_dn8 = assign36340_e46946_d_n8;

        let (assign36350_e46965, assign36350_e46965_d_n5, assign36350_e46965_d_n6, assign36350_e46965_d_n7, assign36350_e46965_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 == 0.0)) && (var_guard731 == 0.0)) {
        let assign36350_e46961: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign36350_e46963: f64 = (assign36350_e46961).powf(var_pstid_i);
        (assign36350_e46963, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36350_e46965;
        var_tmp_dn5 = assign36350_e46965_d_n5;
        var_tmp_dn6 = assign36350_e46965_d_n6;
        var_tmp_dn7 = assign36350_e46965_d_n7;
        var_tmp_dn8 = assign36350_e46965_d_n8;

        let (assign36360_e46979, assign36360_e46979_d_n5, assign36360_e46979_d_n6, assign36360_e46979_d_n7, assign36360_e46979_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 == 0.0)) {
        let assign36360_e46977: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign36360_e46977, (var_wdepnulrsti_d * var_tmp_dn5), (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign36360_e46979;
        var_wdep_dn5 = assign36360_e46979_d_n5;
        var_wdep_dn6 = assign36360_e46979_d_n6;
        var_wdep_dn7 = assign36360_e46979_d_n7;
        var_wdep_dn8 = assign36360_e46979_d_n8;

        let (assign36370_e46997, assign36370_e46997_d_n5, assign36370_e46997_d_n6, assign36370_e46997_d_n7, assign36370_e46997_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 == 0.0)) {
        let assign36370_e46992: f64 = (var_zinv - 1.0);
        let assign36370_e46994: f64 = (assign36370_e46992 * var_wdep);
        let assign36370_e46995: f64 = (var_ftdsti_d * assign36370_e46994);
        (assign36370_e46995, (var_ftdsti_d * (assign36370_e46992 * var_wdep_dn5)), (var_ftdsti_d * (assign36370_e46992 * var_wdep_dn6)), (var_ftdsti_d * (assign36370_e46992 * var_wdep_dn7)), (var_ftdsti_d * (assign36370_e46992 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign36370_e46997;
        var_asrh_dn5 = assign36370_e46997_d_n5;
        var_asrh_dn6 = assign36370_e46997_d_n6;
        var_asrh_dn7 = assign36370_e46997_d_n7;
        var_asrh_dn8 = assign36370_e46997_d_n8;

        let (assign36380_e47013, assign36380_e47013_d_n5, assign36380_e47013_d_n6, assign36380_e47013_d_n7, assign36380_e47013_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard729 == 0.0)) {
        let assign36380_e47010: f64 = (var_asrh * var_wsrh);
        let assign36380_e47011: f64 = (var_csrhstid_i * assign36380_e47010);
        (assign36380_e47011, (var_csrhstid_i * (var_asrh_dn5 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign36380_e47013;
        var_isrh_dn5 = assign36380_e47013_d_n5;
        var_isrh_dn6 = assign36380_e47013_d_n6;
        var_isrh_dn7 = assign36380_e47013_d_n7;
        var_isrh_dn8 = assign36380_e47013_d_n8;

        let assign36390_e47016: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard732 = assign36390_e47016;

        let (assign36400_e47027, assign36400_e47027_d_n5, assign36400_e47027_d_n6, assign36400_e47027_d_n7, assign36400_e47027_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign36400_e47027;
        var_itat_dn5 = assign36400_e47027_d_n5;
        var_itat_dn6 = assign36400_e47027_d_n6;
        var_itat_dn7 = assign36400_e47027_d_n7;
        var_itat_dn8 = assign36400_e47027_d_n8;

        let (assign36410_e47045, assign36410_e47045_d_n5, assign36410_e47045_d_n6, assign36410_e47045_d_n7, assign36410_e47045_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36410_e47040: f64 = (var_wdep * var_one_minus_psti_d);
        let assign36410_e47042: f64 = (assign36410_e47040 / var_vbi_minus_vjsrh);
        let assign36410_e47043: f64 = (var_btatpartsti_d * assign36410_e47042);
        (assign36410_e47043, (var_btatpartsti_d * ((var_wdep_dn5 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign36410_e47045;
        var_btat_dn5 = assign36410_e47045_d_n5;
        var_btat_dn6 = assign36410_e47045_d_n6;
        var_btat_dn7 = assign36410_e47045_d_n7;
        var_btat_dn8 = assign36410_e47045_d_n8;

        let (assign36420_e47061, assign36420_e47061_d_n5, assign36420_e47061_d_n6, assign36420_e47061_d_n7, assign36420_e47061_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36420_e47057: f64 = (0.666666666666667 * var_atatsti_d);
        let assign36420_e47059: f64 = (assign36420_e47057 / var_btat);
        (assign36420_e47059, (-((assign36420_e47057 * var_btat_dn5) / (var_btat * var_btat))), (-((assign36420_e47057 * var_btat_dn6) / (var_btat * var_btat))), (-((assign36420_e47057 * var_btat_dn7) / (var_btat * var_btat))), (-((assign36420_e47057 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign36420_e47061;
        var_twoatatoverthreebtat_dn5 = assign36420_e47061_d_n5;
        var_twoatatoverthreebtat_dn6 = assign36420_e47061_d_n6;
        var_twoatatoverthreebtat_dn7 = assign36420_e47061_d_n7;
        var_twoatatoverthreebtat_dn8 = assign36420_e47061_d_n8;

        let (assign36430_e47075, assign36430_e47075_d_n5, assign36430_e47075_d_n6, assign36430_e47075_d_n7, assign36430_e47075_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36430_e47073: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign36430_e47073, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign36430_e47075;
        var_umaxbeforelimiting_dn5 = assign36430_e47075_d_n5;
        var_umaxbeforelimiting_dn6 = assign36430_e47075_d_n6;
        var_umaxbeforelimiting_dn7 = assign36430_e47075_d_n7;
        var_umaxbeforelimiting_dn8 = assign36430_e47075_d_n8;

        let (assign36440_e47096, assign36440_e47096_d_n5, assign36440_e47096_d_n6, assign36440_e47096_d_n7, assign36440_e47096_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36440_e47087: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign36440_e47090: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign36440_e47092: f64 = (assign36440_e47090 + 1.0);
        let assign36440_e47093: f64 = (assign36440_e47087 / assign36440_e47092);
        let assign36440_e47094: f64 = (assign36440_e47093).sqrt();
        (assign36440_e47094, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign36440_e47092) - (assign36440_e47087 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign36440_e47092 * assign36440_e47092)) / (2.0 * assign36440_e47094)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign36440_e47092) - (assign36440_e47087 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign36440_e47092 * assign36440_e47092)) / (2.0 * assign36440_e47094)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign36440_e47092) - (assign36440_e47087 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign36440_e47092 * assign36440_e47092)) / (2.0 * assign36440_e47094)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign36440_e47092) - (assign36440_e47087 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign36440_e47092 * assign36440_e47092)) / (2.0 * assign36440_e47094)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign36440_e47096;
        var_umax_dn5 = assign36440_e47096_d_n5;
        var_umax_dn6 = assign36440_e47096_d_n6;
        var_umax_dn7 = assign36440_e47096_d_n7;
        var_umax_dn8 = assign36440_e47096_d_n8;

        let (assign36450_e47109, assign36450_e47109_d_n5, assign36450_e47109_d_n6, assign36450_e47109_d_n7, assign36450_e47109_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36450_e47107: f64 = (var_umax).sqrt();
        (assign36450_e47107, (var_umax_dn5 / (2.0 * assign36450_e47107)), (var_umax_dn6 / (2.0 * assign36450_e47107)), (var_umax_dn7 / (2.0 * assign36450_e47107)), (var_umax_dn8 / (2.0 * assign36450_e47107)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign36450_e47109;
        var_sqrtumax_dn5 = assign36450_e47109_d_n5;
        var_sqrtumax_dn6 = assign36450_e47109_d_n6;
        var_sqrtumax_dn7 = assign36450_e47109_d_n7;
        var_sqrtumax_dn8 = assign36450_e47109_d_n8;

        let (assign36460_e47123, assign36460_e47123_d_n5, assign36460_e47123_d_n6, assign36460_e47123_d_n7, assign36460_e47123_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36460_e47121: f64 = (var_umax * var_sqrtumax);
        (assign36460_e47121, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign36460_e47123;
        var_umaxpoweronepointfive_dn5 = assign36460_e47123_d_n5;
        var_umaxpoweronepointfive_dn6 = assign36460_e47123_d_n6;
        var_umaxpoweronepointfive_dn7 = assign36460_e47123_d_n7;
        var_umaxpoweronepointfive_dn8 = assign36460_e47123_d_n8;

        let assign36470_e47125: f64 = (-var_pstid_i);
        let assign36470_e47127: f64 = (assign36470_e47125 * var_one_over_one_minus_psti_d);
        let assign36470_e47129: f64 = (-1.0);
        let assign36470_e47130: f64 = if assign36470_e47127 == assign36470_e47129 { 1.0 } else { 0.0 };
        var_guard733 = assign36470_e47130;

        let (assign36480_e47150, assign36480_e47150_d_n5, assign36480_e47150_d_n6, assign36480_e47150_d_n7, assign36480_e47150_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) && (var_guard733 != 0.0)) {
        let assign36480_e47146: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign36480_e47147: f64 = (1.0 + assign36480_e47146);
        let assign36480_e47148: f64 = (1.0 / assign36480_e47147);
        (assign36480_e47148, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign36480_e47147 * assign36480_e47147))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign36480_e47147 * assign36480_e47147))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign36480_e47147 * assign36480_e47147))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign36480_e47147 * assign36480_e47147))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign36480_e47150;
        var_wgamma_dn5 = assign36480_e47150_d_n5;
        var_wgamma_dn6 = assign36480_e47150_d_n6;
        var_wgamma_dn7 = assign36480_e47150_d_n7;
        var_wgamma_dn8 = assign36480_e47150_d_n8;

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
        *var_guard725_slot = var_guard725;
        *var_guard726_slot = var_guard726;
        *var_guard727_slot = var_guard727;
        *var_guard728_slot = var_guard728;
        *var_guard729_slot = var_guard729;
        *var_guard730_slot = var_guard730;
        *var_guard731_slot = var_guard731;
        *var_guard732_slot = var_guard732;
        *var_guard733_slot = var_guard733;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk219_slot = var_id__blk219;
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
    }

    pub(super) fn stamp_transient_block_76(
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti_d: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cbbtstid_i: f64,
        var_cerfc: f64,
        var_ctatstid_i: f64,
        var_fbbtsti_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard728: f64,
        var_guard732: f64,
        var_guard733: f64,
        var_one_over_one_minus_psti_d: f64,
        var_perfc: f64,
        var_pstid_i: f64,
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
        var_vbbt: f64,
        var_vbirstid_i: f64,
        var_vbirstiinv_d: f64,
        var_wdepnulrinvsti_d: f64,
        var_wsrh: f64,
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
        var_guard734_slot: &mut f64,
        var_guard735_slot: &mut f64,
        var_guard736_slot: &mut f64,
        var_guard737_slot: &mut f64,
        var_guard738_slot: &mut f64,
        var_guard739_slot: &mut f64,
        var_guard740_slot: &mut f64,
        var_guard741_slot: &mut f64,
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
        let mut var_guard734: f64 = *var_guard734_slot;
        let mut var_guard735: f64 = *var_guard735_slot;
        let mut var_guard736: f64 = *var_guard736_slot;
        let mut var_guard737: f64 = *var_guard737_slot;
        let mut var_guard738: f64 = *var_guard738_slot;
        let mut var_guard739: f64 = *var_guard739_slot;
        let mut var_guard740: f64 = *var_guard740_slot;
        let mut var_guard741: f64 = *var_guard741_slot;
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

        let (assign36490_e47174, assign36490_e47174_d_n5, assign36490_e47174_d_n6, assign36490_e47174_d_n7, assign36490_e47174_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) && (var_guard733 == 0.0)) {
        let assign36490_e47166: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign36490_e47167: f64 = (1.0 + assign36490_e47166);
        let assign36490_e47169: f64 = (-var_pstid_i);
        let assign36490_e47171: f64 = (assign36490_e47169 * var_one_over_one_minus_psti_d);
        let assign36490_e47172: f64 = (assign36490_e47167).powf(assign36490_e47171);
        (assign36490_e47172, if 0.0 == 0.0 && ((assign36490_e47171) as f64).is_finite() && ((assign36490_e47171) as f64).fract() == 0.0 { if assign36490_e47171 == 0.0 { 0.0 } else { (assign36490_e47171 * ((assign36490_e47167).powf(assign36490_e47171 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign36490_e47172 * (assign36490_e47171 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign36490_e47167))) }, if 0.0 == 0.0 && ((assign36490_e47171) as f64).is_finite() && ((assign36490_e47171) as f64).fract() == 0.0 { if assign36490_e47171 == 0.0 { 0.0 } else { (assign36490_e47171 * ((assign36490_e47167).powf(assign36490_e47171 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign36490_e47172 * (assign36490_e47171 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign36490_e47167))) }, if 0.0 == 0.0 && ((assign36490_e47171) as f64).is_finite() && ((assign36490_e47171) as f64).fract() == 0.0 { if assign36490_e47171 == 0.0 { 0.0 } else { (assign36490_e47171 * ((assign36490_e47167).powf(assign36490_e47171 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign36490_e47172 * (assign36490_e47171 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign36490_e47167))) }, if 0.0 == 0.0 && ((assign36490_e47171) as f64).is_finite() && ((assign36490_e47171) as f64).fract() == 0.0 { if assign36490_e47171 == 0.0 { 0.0 } else { (assign36490_e47171 * ((assign36490_e47167).powf(assign36490_e47171 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign36490_e47172 * (assign36490_e47171 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign36490_e47167))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign36490_e47174;
        var_wgamma_dn5 = assign36490_e47174_d_n5;
        var_wgamma_dn6 = assign36490_e47174_d_n6;
        var_wgamma_dn7 = assign36490_e47174_d_n7;
        var_wgamma_dn8 = assign36490_e47174_d_n8;

        let (assign36500_e47192, assign36500_e47192_d_n5, assign36500_e47192_d_n6, assign36500_e47192_d_n7, assign36500_e47192_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36500_e47186: f64 = (var_wsrh * var_wgamma);
        let assign36500_e47189: f64 = (var_wsrh + var_wgamma);
        let assign36500_e47190: f64 = (assign36500_e47186 / assign36500_e47189);
        (assign36500_e47190, ((((var_wsrh * var_wgamma_dn5) * assign36500_e47189) - (assign36500_e47186 * var_wgamma_dn5)) / (assign36500_e47189 * assign36500_e47189)), ((((var_wsrh * var_wgamma_dn6) * assign36500_e47189) - (assign36500_e47186 * var_wgamma_dn6)) / (assign36500_e47189 * assign36500_e47189)), ((((var_wsrh * var_wgamma_dn7) * assign36500_e47189) - (assign36500_e47186 * var_wgamma_dn7)) / (assign36500_e47189 * assign36500_e47189)), ((((var_wsrh * var_wgamma_dn8) * assign36500_e47189) - (assign36500_e47186 * var_wgamma_dn8)) / (assign36500_e47189 * assign36500_e47189)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign36500_e47192;
        var_wtat_dn5 = assign36500_e47192_d_n5;
        var_wtat_dn6 = assign36500_e47192_d_n6;
        var_wtat_dn7 = assign36500_e47192_d_n7;
        var_wtat_dn8 = assign36500_e47192_d_n8;

        let (assign36510_e47209, assign36510_e47209_d_n5, assign36510_e47209_d_n6, assign36510_e47209_d_n7, assign36510_e47209_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36510_e47205: f64 = (var_btat / var_sqrtumax);
        let assign36510_e47206: f64 = (0.375 * assign36510_e47205);
        let assign36510_e47207: f64 = (assign36510_e47206).sqrt();
        (assign36510_e47207, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36510_e47207)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36510_e47207)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36510_e47207)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36510_e47207)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign36510_e47209;
        var_ktat_dn5 = assign36510_e47209_d_n5;
        var_ktat_dn6 = assign36510_e47209_d_n6;
        var_ktat_dn7 = assign36510_e47209_d_n7;
        var_ktat_dn8 = assign36510_e47209_d_n8;

        let (assign36520_e47227, assign36520_e47227_d_n5, assign36520_e47227_d_n6, assign36520_e47227_d_n7, assign36520_e47227_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36520_e47222: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign36520_e47223: f64 = (2.0 * assign36520_e47222);
        let assign36520_e47225: f64 = (assign36520_e47223 - var_umax);
        (assign36520_e47225, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign36520_e47227;
        var_ltat_dn5 = assign36520_e47227_d_n5;
        var_ltat_dn6 = assign36520_e47227_d_n6;
        var_ltat_dn7 = assign36520_e47227_d_n7;
        var_ltat_dn8 = assign36520_e47227_d_n8;

        let (assign36530_e47253, assign36530_e47253_d_n5, assign36530_e47253_d_n6, assign36530_e47253_d_n7, assign36530_e47253_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36530_e47239: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign36530_e47241: f64 = (assign36530_e47239 * var_sqrtumax);
        let assign36530_e47244: f64 = (var_atatsti_d * var_umax);
        let assign36530_e47245: f64 = (assign36530_e47241 - assign36530_e47244);
        let assign36530_e47249: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign36530_e47250: f64 = (0.5 * assign36530_e47249);
        let assign36530_e47251: f64 = (assign36530_e47245 + assign36530_e47250);
        (assign36530_e47251, (((((var_atatsti_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign36530_e47239 * var_sqrtumax_dn5)) - (var_atatsti_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign36530_e47239 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign36530_e47239 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign36530_e47239 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign36530_e47253;
        var_mtat_dn5 = assign36530_e47253_d_n5;
        var_mtat_dn6 = assign36530_e47253_d_n6;
        var_mtat_dn7 = assign36530_e47253_d_n7;
        var_mtat_dn8 = assign36530_e47253_d_n8;

        let (assign36540_e47269, assign36540_e47269_d_n5, assign36540_e47269_d_n6, assign36540_e47269_d_n7, assign36540_e47269_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36540_e47265: f64 = (var_ltat - 1.0);
        let assign36540_e47267: f64 = (assign36540_e47265 * var_ktat);
        (assign36540_e47267, ((var_ltat_dn5 * var_ktat) + (assign36540_e47265 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign36540_e47265 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign36540_e47265 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign36540_e47265 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign36540_e47269;
        var_xerfc_dn5 = assign36540_e47269_d_n5;
        var_xerfc_dn6 = assign36540_e47269_d_n6;
        var_xerfc_dn7 = assign36540_e47269_d_n7;
        var_xerfc_dn8 = assign36540_e47269_d_n8;

        let (assign36550_e47283, assign36550_e47283_d_n5, assign36550_e47283_d_n6, assign36550_e47283_d_n7, assign36550_e47283_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36550_e47281: f64 = (var_xerfc * var_xerfc);
        (assign36550_e47281, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign36550_e47283;
        var_ysq_dn5 = assign36550_e47283_d_n5;
        var_ysq_dn6 = assign36550_e47283_d_n6;
        var_ysq_dn7 = assign36550_e47283_d_n7;
        var_ysq_dn8 = assign36550_e47283_d_n8;

        let assign36560_e47286: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard734 = assign36560_e47286;

        let (assign36570_e47306, assign36570_e47306_d_n5, assign36570_e47306_d_n6, assign36570_e47306_d_n7, assign36570_e47306_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) && (var_guard734 != 0.0)) {
        let assign36570_e47302: f64 = (var_perfc * var_xerfc);
        let assign36570_e47303: f64 = (1.0 + assign36570_e47302);
        let assign36570_e47304: f64 = (1.0 / assign36570_e47303);
        (assign36570_e47304, (-((var_perfc * var_xerfc_dn5) / (assign36570_e47303 * assign36570_e47303))), (-((var_perfc * var_xerfc_dn6) / (assign36570_e47303 * assign36570_e47303))), (-((var_perfc * var_xerfc_dn7) / (assign36570_e47303 * assign36570_e47303))), (-((var_perfc * var_xerfc_dn8) / (assign36570_e47303 * assign36570_e47303))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign36570_e47306;
        var_terfc_dn5 = assign36570_e47306_d_n5;
        var_terfc_dn6 = assign36570_e47306_d_n6;
        var_terfc_dn7 = assign36570_e47306_d_n7;
        var_terfc_dn8 = assign36570_e47306_d_n8;

        let (assign36580_e47327, assign36580_e47327_d_n5, assign36580_e47327_d_n6, assign36580_e47327_d_n7, assign36580_e47327_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) && (var_guard734 == 0.0)) {
        let assign36580_e47323: f64 = (var_perfc * var_xerfc);
        let assign36580_e47324: f64 = (1.0 - assign36580_e47323);
        let assign36580_e47325: f64 = (1.0 / assign36580_e47324);
        (assign36580_e47325, (-((-(var_perfc * var_xerfc_dn5)) / (assign36580_e47324 * assign36580_e47324))), (-((-(var_perfc * var_xerfc_dn6)) / (assign36580_e47324 * assign36580_e47324))), (-((-(var_perfc * var_xerfc_dn7)) / (assign36580_e47324 * assign36580_e47324))), (-((-(var_perfc * var_xerfc_dn8)) / (assign36580_e47324 * assign36580_e47324))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign36580_e47327;
        var_terfc_dn5 = assign36580_e47327_d_n5;
        var_terfc_dn6 = assign36580_e47327_d_n6;
        var_terfc_dn7 = assign36580_e47327_d_n7;
        var_terfc_dn8 = assign36580_e47327_d_n8;

        let assign36590_e47329: f64 = (-var_ysq);
        let assign36590_e47331: f64 = (assign36590_e47329 + var_mtat);
        let assign36590_e47333: f64 = (-230.25850929940458);
        let assign36590_e47334: f64 = if assign36590_e47331 > assign36590_e47333 { 1.0 } else { 0.0 };
        var_guard735 = assign36590_e47334;

        let (assign36600_e47352, assign36600_e47352_d_n5, assign36600_e47352_d_n6, assign36600_e47352_d_n7, assign36600_e47352_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) && (var_guard735 != 0.0)) {
        let assign36600_e47347: f64 = (-var_ysq);
        let assign36600_e47349: f64 = (assign36600_e47347 + var_mtat);
        let assign36600_e47350: f64 = (assign36600_e47349).exp();
        (assign36600_e47350, (assign36600_e47350 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign36600_e47350 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign36600_e47350 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign36600_e47350 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36600_e47352;
        var_tmp_dn5 = assign36600_e47352_d_n5;
        var_tmp_dn6 = assign36600_e47352_d_n6;
        var_tmp_dn7 = assign36600_e47352_d_n7;
        var_tmp_dn8 = assign36600_e47352_d_n8;

        let (assign36610_e47401, assign36610_e47401_d_n5, assign36610_e47401_d_n6, assign36610_e47401_d_n7, assign36610_e47401_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) && (var_guard735 == 0.0)) {
        let assign36610_e47368: f64 = (-230.25850929940458);
        let assign36610_e47370: f64 = (-var_ysq);
        let assign36610_e47372: f64 = (assign36610_e47370 + var_mtat);
        let assign36610_e47373: f64 = (assign36610_e47368 - assign36610_e47372);
        let assign36610_e47377: f64 = (-230.25850929940458);
        let assign36610_e47379: f64 = (-var_ysq);
        let assign36610_e47381: f64 = (assign36610_e47379 + var_mtat);
        let assign36610_e47382: f64 = (assign36610_e47377 - assign36610_e47381);
        let assign36610_e47385: f64 = (-230.25850929940458);
        let assign36610_e47387: f64 = (-var_ysq);
        let assign36610_e47389: f64 = (assign36610_e47387 + var_mtat);
        let assign36610_e47390: f64 = (assign36610_e47385 - assign36610_e47389);
        let assign36610_e47392: f64 = (assign36610_e47390 * 0.3333333333333333);
        let assign36610_e47393: f64 = (1.0 + assign36610_e47392);
        let assign36610_e47394: f64 = (assign36610_e47382 * assign36610_e47393);
        let assign36610_e47395: f64 = (0.5 * assign36610_e47394);
        let assign36610_e47396: f64 = (1.0 + assign36610_e47395);
        let assign36610_e47397: f64 = (assign36610_e47373 * assign36610_e47396);
        let assign36610_e47398: f64 = (1.0 + assign36610_e47397);
        let assign36610_e47399: f64 = (1e-100 / assign36610_e47398);
        (assign36610_e47399, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign36610_e47396) + (assign36610_e47373 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign36610_e47393) + (assign36610_e47382 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign36610_e47398 * assign36610_e47398))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign36610_e47396) + (assign36610_e47373 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign36610_e47393) + (assign36610_e47382 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign36610_e47398 * assign36610_e47398))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign36610_e47396) + (assign36610_e47373 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign36610_e47393) + (assign36610_e47382 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign36610_e47398 * assign36610_e47398))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign36610_e47396) + (assign36610_e47373 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign36610_e47393) + (assign36610_e47382 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign36610_e47398 * assign36610_e47398))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36610_e47401;
        var_tmp_dn5 = assign36610_e47401_d_n5;
        var_tmp_dn6 = assign36610_e47401_d_n6;
        var_tmp_dn7 = assign36610_e47401_d_n7;
        var_tmp_dn8 = assign36610_e47401_d_n8;

        let (assign36620_e47431, assign36620_e47431_d_n5, assign36620_e47431_d_n6, assign36620_e47431_d_n7, assign36620_e47431_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36620_e47413: f64 = (0.29214664 * var_terfc);
        let assign36620_e47417: f64 = (var_terfc * var_terfc);
        let assign36620_e47418: f64 = (var_berfc * assign36620_e47417);
        let assign36620_e47419: f64 = (assign36620_e47413 + assign36620_e47418);
        let assign36620_e47423: f64 = (var_terfc * var_terfc);
        let assign36620_e47425: f64 = (assign36620_e47423 * var_terfc);
        let assign36620_e47426: f64 = (var_cerfc * assign36620_e47425);
        let assign36620_e47427: f64 = (assign36620_e47419 + assign36620_e47426);
        let assign36620_e47429: f64 = (assign36620_e47427 * var_tmp);
        (assign36620_e47429, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign36620_e47423 * var_terfc_dn5)))) * var_tmp) + (assign36620_e47427 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign36620_e47423 * var_terfc_dn6)))) * var_tmp) + (assign36620_e47427 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign36620_e47423 * var_terfc_dn7)))) * var_tmp) + (assign36620_e47427 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign36620_e47423 * var_terfc_dn8)))) * var_tmp) + (assign36620_e47427 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign36620_e47431;
        var_erfcpos_dn5 = assign36620_e47431_d_n5;
        var_erfcpos_dn6 = assign36620_e47431_d_n6;
        var_erfcpos_dn7 = assign36620_e47431_d_n7;
        var_erfcpos_dn8 = assign36620_e47431_d_n8;

        let assign36630_e47434: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard736 = assign36630_e47434;

        let (assign36640_e47448, assign36640_e47448_d_n5, assign36640_e47448_d_n6, assign36640_e47448_d_n7, assign36640_e47448_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) && (var_guard736 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign36640_e47448;
        var_erfctimesexpmtat_dn5 = assign36640_e47448_d_n5;
        var_erfctimesexpmtat_dn6 = assign36640_e47448_d_n6;
        var_erfctimesexpmtat_dn7 = assign36640_e47448_d_n7;
        var_erfctimesexpmtat_dn8 = assign36640_e47448_d_n8;

        let assign36650_e47451: f64 = (-230.25850929940458);
        let assign36650_e47452: f64 = if var_mtat > assign36650_e47451 { 1.0 } else { 0.0 };
        var_guard737 = assign36650_e47452;

        let (assign36660_e47470, assign36660_e47470_d_n5, assign36660_e47470_d_n6, assign36660_e47470_d_n7, assign36660_e47470_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) && (var_guard736 == 0.0)) && (var_guard737 != 0.0)) {
        let assign36660_e47468: f64 = (var_mtat).exp();
        (assign36660_e47468, (assign36660_e47468 * var_mtat_dn5), (assign36660_e47468 * var_mtat_dn6), (assign36660_e47468 * var_mtat_dn7), (assign36660_e47468 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36660_e47470;
        var_tmp_dn5 = assign36660_e47470_d_n5;
        var_tmp_dn6 = assign36660_e47470_d_n6;
        var_tmp_dn7 = assign36660_e47470_d_n7;
        var_tmp_dn8 = assign36660_e47470_d_n8;

        let (assign36670_e47513, assign36670_e47513_d_n5, assign36670_e47513_d_n6, assign36670_e47513_d_n7, assign36670_e47513_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) && (var_guard736 == 0.0)) && (var_guard737 == 0.0)) {
        let assign36670_e47489: f64 = (-230.25850929940458);
        let assign36670_e47491: f64 = (assign36670_e47489 - var_mtat);
        let assign36670_e47495: f64 = (-230.25850929940458);
        let assign36670_e47497: f64 = (assign36670_e47495 - var_mtat);
        let assign36670_e47500: f64 = (-230.25850929940458);
        let assign36670_e47502: f64 = (assign36670_e47500 - var_mtat);
        let assign36670_e47504: f64 = (assign36670_e47502 * 0.3333333333333333);
        let assign36670_e47505: f64 = (1.0 + assign36670_e47504);
        let assign36670_e47506: f64 = (assign36670_e47497 * assign36670_e47505);
        let assign36670_e47507: f64 = (0.5 * assign36670_e47506);
        let assign36670_e47508: f64 = (1.0 + assign36670_e47507);
        let assign36670_e47509: f64 = (assign36670_e47491 * assign36670_e47508);
        let assign36670_e47510: f64 = (1.0 + assign36670_e47509);
        let assign36670_e47511: f64 = (1e-100 / assign36670_e47510);
        (assign36670_e47511, (-((1e-100 * (((-var_mtat_dn5) * assign36670_e47508) + (assign36670_e47491 * (0.5 * (((-var_mtat_dn5) * assign36670_e47505) + (assign36670_e47497 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign36670_e47510 * assign36670_e47510))), (-((1e-100 * (((-var_mtat_dn6) * assign36670_e47508) + (assign36670_e47491 * (0.5 * (((-var_mtat_dn6) * assign36670_e47505) + (assign36670_e47497 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign36670_e47510 * assign36670_e47510))), (-((1e-100 * (((-var_mtat_dn7) * assign36670_e47508) + (assign36670_e47491 * (0.5 * (((-var_mtat_dn7) * assign36670_e47505) + (assign36670_e47497 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign36670_e47510 * assign36670_e47510))), (-((1e-100 * (((-var_mtat_dn8) * assign36670_e47508) + (assign36670_e47491 * (0.5 * (((-var_mtat_dn8) * assign36670_e47505) + (assign36670_e47497 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign36670_e47510 * assign36670_e47510))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36670_e47513;
        var_tmp_dn5 = assign36670_e47513_d_n5;
        var_tmp_dn6 = assign36670_e47513_d_n6;
        var_tmp_dn7 = assign36670_e47513_d_n7;
        var_tmp_dn8 = assign36670_e47513_d_n8;

        let (assign36680_e47532, assign36680_e47532_d_n5, assign36680_e47532_d_n6, assign36680_e47532_d_n7, assign36680_e47532_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) && (var_guard736 == 0.0)) {
        let assign36680_e47528: f64 = (2.0 * var_tmp);
        let assign36680_e47530: f64 = (assign36680_e47528 - var_erfcpos);
        (assign36680_e47530, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign36680_e47532;
        var_erfctimesexpmtat_dn5 = assign36680_e47532_d_n5;
        var_erfctimesexpmtat_dn6 = assign36680_e47532_d_n6;
        var_erfctimesexpmtat_dn7 = assign36680_e47532_d_n7;
        var_erfctimesexpmtat_dn8 = assign36680_e47532_d_n8;

        let (assign36690_e47552, assign36690_e47552_d_n5, assign36690_e47552_d_n6, assign36690_e47552_d_n7, assign36690_e47552_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36690_e47544: f64 = (1.772453850905516 * 0.5);
        let assign36690_e47547: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign36690_e47549: f64 = (assign36690_e47547 / var_ktat);
        let assign36690_e47550: f64 = (assign36690_e47544 * assign36690_e47549);
        (assign36690_e47550, (assign36690_e47544 * ((((var_atatsti_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign36690_e47547 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign36690_e47544 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign36690_e47547 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign36690_e47544 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign36690_e47547 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign36690_e47544 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign36690_e47547 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign36690_e47552;
        var_gammamax_dn5 = assign36690_e47552_d_n5;
        var_gammamax_dn6 = assign36690_e47552_d_n6;
        var_gammamax_dn7 = assign36690_e47552_d_n7;
        var_gammamax_dn8 = assign36690_e47552_d_n8;

        let (assign36700_e47570, assign36700_e47570_d_n5, assign36700_e47570_d_n6, assign36700_e47570_d_n7, assign36700_e47570_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard732 == 0.0)) {
        let assign36700_e47565: f64 = (var_asrh * var_gammamax);
        let assign36700_e47567: f64 = (assign36700_e47565 * var_wtat);
        let assign36700_e47568: f64 = (var_ctatstid_i * assign36700_e47567);
        (assign36700_e47568, (var_ctatstid_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign36700_e47565 * var_wtat_dn5))), (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign36700_e47565 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign36700_e47565 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign36700_e47565 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign36700_e47570;
        var_itat_dn5 = assign36700_e47570_d_n5;
        var_itat_dn6 = assign36700_e47570_d_n6;
        var_itat_dn7 = assign36700_e47570_d_n7;
        var_itat_dn8 = assign36700_e47570_d_n8;

        let assign36710_e47573: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard738 = assign36710_e47573;

        let (assign36720_e47584, assign36720_e47584_d_n5, assign36720_e47584_d_n6, assign36720_e47584_d_n7, assign36720_e47584_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard738 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign36720_e47584;
        var_ibbt_dn5 = assign36720_e47584_d_n5;
        var_ibbt_dn6 = assign36720_e47584_d_n6;
        var_ibbt_dn7 = assign36720_e47584_d_n7;
        var_ibbt_dn8 = assign36720_e47584_d_n8;

        let assign36730_e47587: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard739 = assign36730_e47587;

        let (assign36740_e47606, assign36740_e47606_d_n5, assign36740_e47606_d_n6, assign36740_e47606_d_n7, assign36740_e47606_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard738 == 0.0)) && (var_guard739 != 0.0)) {
        let assign36740_e47601: f64 = (var_vbirstid_i - var_vbbt);
        let assign36740_e47603: f64 = (assign36740_e47601 * var_vbirstiinv_d);
        let assign36740_e47604: f64 = (assign36740_e47603).sqrt();
        (assign36740_e47604, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36740_e47606;
        var_tmp_dn5 = assign36740_e47606_d_n5;
        var_tmp_dn6 = assign36740_e47606_d_n6;
        var_tmp_dn7 = assign36740_e47606_d_n7;
        var_tmp_dn8 = assign36740_e47606_d_n8;

        let (assign36750_e47627, assign36750_e47627_d_n5, assign36750_e47627_d_n6, assign36750_e47627_d_n7, assign36750_e47627_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) {
        let assign36750_e47621: f64 = (var_vbirstid_i - var_vbbt);
        let assign36750_e47623: f64 = (assign36750_e47621 * var_vbirstiinv_d);
        let assign36750_e47625: f64 = (assign36750_e47623).powf(var_pstid_i);
        (assign36750_e47625, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36750_e47627;
        var_tmp_dn5 = assign36750_e47627_d_n5;
        var_tmp_dn6 = assign36750_e47627_d_n6;
        var_tmp_dn7 = assign36750_e47627_d_n7;
        var_tmp_dn8 = assign36750_e47627_d_n8;

        let (assign36760_e47647, assign36760_e47647_d_n5, assign36760_e47647_d_n6, assign36760_e47647_d_n7, assign36760_e47647_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard738 == 0.0)) {
        let assign36760_e47640: f64 = (var_vbirstid_i - var_vbbt);
        let assign36760_e47642: f64 = (assign36760_e47640 * var_wdepnulrinvsti_d);
        let assign36760_e47644: f64 = (assign36760_e47642 / var_tmp);
        let assign36760_e47645: f64 = (var_one_over_one_minus_psti_d * assign36760_e47644);
        (assign36760_e47645, (var_one_over_one_minus_psti_d * (-((assign36760_e47642 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign36760_e47642 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign36760_e47642 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign36760_e47642 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign36760_e47647;
        var_fmaxr_dn5 = assign36760_e47647_d_n5;
        var_fmaxr_dn6 = assign36760_e47647_d_n6;
        var_fmaxr_dn7 = assign36760_e47647_d_n7;
        var_fmaxr_dn8 = assign36760_e47647_d_n8;

        let assign36770_e47649: f64 = (-var_fbbtsti_d);
        let assign36770_e47651: f64 = (assign36770_e47649 / var_fmaxr);
        let assign36770_e47652: f64 = (assign36770_e47651).abs();
        let assign36770_e47654: f64 = if assign36770_e47652 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard740 = assign36770_e47654;

        let (assign36780_e47672, assign36780_e47672_d_n5, assign36780_e47672_d_n6, assign36780_e47672_d_n7, assign36780_e47672_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard738 == 0.0)) && (var_guard740 != 0.0)) {
        let assign36780_e47667: f64 = (-var_fbbtsti_d);
        let assign36780_e47669: f64 = (assign36780_e47667 / var_fmaxr);
        let assign36780_e47670: f64 = (assign36780_e47669).exp();
        (assign36780_e47670, (assign36780_e47670 * (-((assign36780_e47667 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign36780_e47670 * (-((assign36780_e47667 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign36780_e47670 * (-((assign36780_e47667 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign36780_e47670 * (-((assign36780_e47667 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36780_e47672;
        var_tmp_dn5 = assign36780_e47672_d_n5;
        var_tmp_dn6 = assign36780_e47672_d_n6;
        var_tmp_dn7 = assign36780_e47672_d_n7;
        var_tmp_dn8 = assign36780_e47672_d_n8;

        let assign36790_e47674: f64 = (-var_fbbtsti_d);
        let assign36790_e47676: f64 = (assign36790_e47674 / var_fmaxr);
        let assign36790_e47678: f64 = if assign36790_e47676 < 0.0 { 1.0 } else { 0.0 };
        var_guard741 = assign36790_e47678;

        let (assign36800_e47729, assign36800_e47729_d_n5, assign36800_e47729_d_n6, assign36800_e47729_d_n7, assign36800_e47729_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard738 == 0.0)) && (var_guard740 == 0.0)) && (var_guard741 != 0.0)) {
        let assign36800_e47696: f64 = (-230.25850929940458);
        let assign36800_e47698: f64 = (-var_fbbtsti_d);
        let assign36800_e47700: f64 = (assign36800_e47698 / var_fmaxr);
        let assign36800_e47701: f64 = (assign36800_e47696 - assign36800_e47700);
        let assign36800_e47705: f64 = (-230.25850929940458);
        let assign36800_e47707: f64 = (-var_fbbtsti_d);
        let assign36800_e47709: f64 = (assign36800_e47707 / var_fmaxr);
        let assign36800_e47710: f64 = (assign36800_e47705 - assign36800_e47709);
        let assign36800_e47713: f64 = (-230.25850929940458);
        let assign36800_e47715: f64 = (-var_fbbtsti_d);
        let assign36800_e47717: f64 = (assign36800_e47715 / var_fmaxr);
        let assign36800_e47718: f64 = (assign36800_e47713 - assign36800_e47717);
        let assign36800_e47720: f64 = (assign36800_e47718 * 0.3333333333333333);
        let assign36800_e47721: f64 = (1.0 + assign36800_e47720);
        let assign36800_e47722: f64 = (assign36800_e47710 * assign36800_e47721);
        let assign36800_e47723: f64 = (0.5 * assign36800_e47722);
        let assign36800_e47724: f64 = (1.0 + assign36800_e47723);
        let assign36800_e47725: f64 = (assign36800_e47701 * assign36800_e47724);
        let assign36800_e47726: f64 = (1.0 + assign36800_e47725);
        let assign36800_e47727: f64 = (1e-100 / assign36800_e47726);
        (assign36800_e47727, (-((1e-100 * (((-(-((assign36800_e47698 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign36800_e47724) + (assign36800_e47701 * (0.5 * (((-(-((assign36800_e47707 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign36800_e47721) + (assign36800_e47710 * ((-(-((assign36800_e47715 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36800_e47726 * assign36800_e47726))), (-((1e-100 * (((-(-((assign36800_e47698 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign36800_e47724) + (assign36800_e47701 * (0.5 * (((-(-((assign36800_e47707 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign36800_e47721) + (assign36800_e47710 * ((-(-((assign36800_e47715 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36800_e47726 * assign36800_e47726))), (-((1e-100 * (((-(-((assign36800_e47698 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign36800_e47724) + (assign36800_e47701 * (0.5 * (((-(-((assign36800_e47707 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign36800_e47721) + (assign36800_e47710 * ((-(-((assign36800_e47715 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36800_e47726 * assign36800_e47726))), (-((1e-100 * (((-(-((assign36800_e47698 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign36800_e47724) + (assign36800_e47701 * (0.5 * (((-(-((assign36800_e47707 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign36800_e47721) + (assign36800_e47710 * ((-(-((assign36800_e47715 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign36800_e47726 * assign36800_e47726))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36800_e47729;
        var_tmp_dn5 = assign36800_e47729_d_n5;
        var_tmp_dn6 = assign36800_e47729_d_n6;
        var_tmp_dn7 = assign36800_e47729_d_n7;
        var_tmp_dn8 = assign36800_e47729_d_n8;

        let (assign36810_e47778, assign36810_e47778_d_n5, assign36810_e47778_d_n6, assign36810_e47778_d_n7, assign36810_e47778_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard738 == 0.0)) && (var_guard740 == 0.0)) && (var_guard741 == 0.0)) {
        let assign36810_e47748: f64 = (-var_fbbtsti_d);
        let assign36810_e47750: f64 = (assign36810_e47748 / var_fmaxr);
        let assign36810_e47752: f64 = (assign36810_e47750 - 230.25850929940458);
        let assign36810_e47756: f64 = (-var_fbbtsti_d);
        let assign36810_e47758: f64 = (assign36810_e47756 / var_fmaxr);
        let assign36810_e47760: f64 = (assign36810_e47758 - 230.25850929940458);
        let assign36810_e47763: f64 = (-var_fbbtsti_d);
        let assign36810_e47765: f64 = (assign36810_e47763 / var_fmaxr);
        let assign36810_e47767: f64 = (assign36810_e47765 - 230.25850929940458);
        let assign36810_e47769: f64 = (assign36810_e47767 * 0.3333333333333333);
        let assign36810_e47770: f64 = (1.0 + assign36810_e47769);
        let assign36810_e47771: f64 = (assign36810_e47760 * assign36810_e47770);
        let assign36810_e47772: f64 = (0.5 * assign36810_e47771);
        let assign36810_e47773: f64 = (1.0 + assign36810_e47772);
        let assign36810_e47774: f64 = (assign36810_e47752 * assign36810_e47773);
        let assign36810_e47775: f64 = (1.0 + assign36810_e47774);
        let assign36810_e47776: f64 = (1e100 * assign36810_e47775);
        (assign36810_e47776, (1e100 * (((-((assign36810_e47748 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign36810_e47773) + (assign36810_e47752 * (0.5 * (((-((assign36810_e47756 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign36810_e47770) + (assign36810_e47760 * ((-((assign36810_e47763 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36810_e47748 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign36810_e47773) + (assign36810_e47752 * (0.5 * (((-((assign36810_e47756 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign36810_e47770) + (assign36810_e47760 * ((-((assign36810_e47763 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36810_e47748 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign36810_e47773) + (assign36810_e47752 * (0.5 * (((-((assign36810_e47756 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign36810_e47770) + (assign36810_e47760 * ((-((assign36810_e47763 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign36810_e47748 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign36810_e47773) + (assign36810_e47752 * (0.5 * (((-((assign36810_e47756 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign36810_e47770) + (assign36810_e47760 * ((-((assign36810_e47763 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36810_e47778;
        var_tmp_dn5 = assign36810_e47778_d_n5;
        var_tmp_dn6 = assign36810_e47778_d_n6;
        var_tmp_dn7 = assign36810_e47778_d_n7;
        var_tmp_dn8 = assign36810_e47778_d_n8;

        let (assign36820_e47798, assign36820_e47798_d_n5, assign36820_e47798_d_n6, assign36820_e47798_d_n7, assign36820_e47798_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard738 == 0.0)) {
        let assign36820_e47791: f64 = (var_v4 * var_fmaxr);
        let assign36820_e47793: f64 = (assign36820_e47791 * var_fmaxr);
        let assign36820_e47795: f64 = (assign36820_e47793 * var_tmp);
        let assign36820_e47796: f64 = (var_cbbtstid_i * assign36820_e47795);
        (assign36820_e47796, (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign36820_e47791 * var_fmaxr_dn5)) * var_tmp) + (assign36820_e47793 * var_tmp_dn5))), (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign36820_e47791 * var_fmaxr_dn6)) * var_tmp) + (assign36820_e47793 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign36820_e47791 * var_fmaxr_dn7)) * var_tmp) + (assign36820_e47793 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign36820_e47791 * var_fmaxr_dn8)) * var_tmp) + (assign36820_e47793 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign36820_e47798;
        var_ibbt_dn5 = assign36820_e47798_d_n5;
        var_ibbt_dn6 = assign36820_e47798_d_n6;
        var_ibbt_dn7 = assign36820_e47798_d_n7;
        var_ibbt_dn8 = assign36820_e47798_d_n8;

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
        *var_guard734_slot = var_guard734;
        *var_guard735_slot = var_guard735;
        *var_guard736_slot = var_guard736;
        *var_guard737_slot = var_guard737;
        *var_guard738_slot = var_guard738;
        *var_guard739_slot = var_guard739;
        *var_guard740_slot = var_guard740;
        *var_guard741_slot = var_guard741;
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

    pub(super) fn stamp_transient_block_77(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat_d: f64,
        var_btatpartgat_d: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_fstopsti_d: f64,
        var_ftdgat_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard728: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idmult: f64,
        var_idsatgat_d: f64,
        var_lgdrain_i: f64,
        var_one_minus_pgat_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_slopesti_d: f64,
        var_two_psistar: f64,
        var_vav: f64,
        var_vbigat_d: f64,
        var_vbirgatinv_d: f64,
        var_vbrinvsti_d: f64,
        var_vbrstid_i: f64,
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
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard742_slot: &mut f64,
        var_guard743_slot: &mut f64,
        var_guard744_slot: &mut f64,
        var_guard745_slot: &mut f64,
        var_guard746_slot: &mut f64,
        var_guard747_slot: &mut f64,
        var_guard748_slot: &mut f64,
        var_guard749_slot: &mut f64,
        var_guard750_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard742: f64 = *var_guard742_slot;
        let mut var_guard743: f64 = *var_guard743_slot;
        let mut var_guard744: f64 = *var_guard744_slot;
        let mut var_guard745: f64 = *var_guard745_slot;
        let mut var_guard746: f64 = *var_guard746_slot;
        let mut var_guard747: f64 = *var_guard747_slot;
        let mut var_guard748: f64 = *var_guard748_slot;
        let mut var_guard749: f64 = *var_guard749_slot;
        let mut var_guard750: f64 = *var_guard750_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let assign36830_e47801: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard742 = assign36830_e47801;

        let (assign36840_e47812, assign36840_e47812_d_n5, assign36840_e47812_d_n6, assign36840_e47812_d_n7, assign36840_e47812_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard742 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36840_e47812;
        var_fbreakdown_dn5 = assign36840_e47812_d_n5;
        var_fbreakdown_dn6 = assign36840_e47812_d_n6;
        var_fbreakdown_dn7 = assign36840_e47812_d_n7;
        var_fbreakdown_dn8 = assign36840_e47812_d_n8;

        let assign36850_e47815: f64 = (-var_alphaav);
        let assign36850_e47817: f64 = (assign36850_e47815 * var_vbrstid_i);
        let assign36850_e47818: f64 = if var_vav > assign36850_e47817 { 1.0 } else { 0.0 };
        var_guard743 = assign36850_e47818;

        let assign36860_e47821: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard744 = assign36860_e47821;

        let (assign36870_e47851, assign36870_e47851_d_n5, assign36870_e47851_d_n6, assign36870_e47851_d_n7, assign36870_e47851_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard742 == 0.0)) && (var_guard743 != 0.0)) && (var_guard744 != 0.0)) {
        let assign36870_e47837: f64 = (var_vav * var_vbrinvsti_d);
        let assign36870_e47840: f64 = (var_vav * var_vbrinvsti_d);
        let assign36870_e47841: f64 = (assign36870_e47837 * assign36870_e47840);
        let assign36870_e47844: f64 = (var_vav * var_vbrinvsti_d);
        let assign36870_e47845: f64 = (assign36870_e47841 * assign36870_e47844);
        let assign36870_e47848: f64 = (var_vav * var_vbrinvsti_d);
        let assign36870_e47849: f64 = (assign36870_e47845 * assign36870_e47848);
        (assign36870_e47849, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36870_e47851;
        var_tmp_dn5 = assign36870_e47851_d_n5;
        var_tmp_dn6 = assign36870_e47851_d_n6;
        var_tmp_dn7 = assign36870_e47851_d_n7;
        var_tmp_dn8 = assign36870_e47851_d_n8;

        let (assign36880_e47873, assign36880_e47873_d_n5, assign36880_e47873_d_n6, assign36880_e47873_d_n7, assign36880_e47873_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard742 == 0.0)) && (var_guard743 != 0.0)) && (var_guard744 == 0.0)) {
        let assign36880_e47868: f64 = (var_vav * var_vbrinvsti_d);
        let assign36880_e47869: f64 = (assign36880_e47868).abs();
        let assign36880_e47871: f64 = (assign36880_e47869).powf(var_pbrstid_i);
        (assign36880_e47871, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign36880_e47873;
        var_tmp_dn5 = assign36880_e47873_d_n5;
        var_tmp_dn6 = assign36880_e47873_d_n6;
        var_tmp_dn7 = assign36880_e47873_d_n7;
        var_tmp_dn8 = assign36880_e47873_d_n8;

        let (assign36890_e47891, assign36890_e47891_d_n5, assign36890_e47891_d_n6, assign36890_e47891_d_n7, assign36890_e47891_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard742 == 0.0)) && (var_guard743 != 0.0)) {
        let assign36890_e47888: f64 = (1.0 - var_tmp);
        let assign36890_e47889: f64 = (1.0 / assign36890_e47888);
        (assign36890_e47889, (-((-var_tmp_dn5) / (assign36890_e47888 * assign36890_e47888))), (-((-var_tmp_dn6) / (assign36890_e47888 * assign36890_e47888))), (-((-var_tmp_dn7) / (assign36890_e47888 * assign36890_e47888))), (-((-var_tmp_dn8) / (assign36890_e47888 * assign36890_e47888))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36890_e47891;
        var_fbreakdown_dn5 = assign36890_e47891_d_n5;
        var_fbreakdown_dn6 = assign36890_e47891_d_n6;
        var_fbreakdown_dn7 = assign36890_e47891_d_n7;
        var_fbreakdown_dn8 = assign36890_e47891_d_n8;

        let (assign36900_e47914, assign36900_e47914_d_n5, assign36900_e47914_d_n6, assign36900_e47914_d_n7, assign36900_e47914_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) && (var_guard742 == 0.0)) && (var_guard743 == 0.0)) {
        let assign36900_e47908: f64 = (var_alphaav * var_vbrstid_i);
        let assign36900_e47909: f64 = (var_vav + assign36900_e47908);
        let assign36900_e47911: f64 = (assign36900_e47909 * var_slopesti_d);
        let assign36900_e47912: f64 = (var_fstopsti_d + assign36900_e47911);
        (assign36900_e47912, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign36900_e47914;
        var_fbreakdown_dn5 = assign36900_e47914_d_n5;
        var_fbreakdown_dn6 = assign36900_e47914_d_n6;
        var_fbreakdown_dn7 = assign36900_e47914_d_n7;
        var_fbreakdown_dn8 = assign36900_e47914_d_n8;

        let (assign36910_e47933, assign36910_e47933_d_n5, assign36910_e47933_d_n6, assign36910_e47933_d_n7, assign36910_e47933_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard728 == 0.0)) {
        let assign36910_e47924: f64 = (var_id__blk219 + var_isrh);
        let assign36910_e47926: f64 = (assign36910_e47924 + var_itat);
        let assign36910_e47928: f64 = (assign36910_e47926 + var_ibbt);
        let assign36910_e47929: f64 = (p.p29 * assign36910_e47928);
        let assign36910_e47931: f64 = (assign36910_e47929 * var_fbreakdown);
        (assign36910_e47931, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign36910_e47929 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign36910_e47929 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign36910_e47929 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign36910_e47929 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign36910_e47933;
        var_ijunsti_dn5 = assign36910_e47933_d_n5;
        var_ijunsti_dn6 = assign36910_e47933_d_n6;
        var_ijunsti_dn7 = assign36910_e47933_d_n7;
        var_ijunsti_dn8 = assign36910_e47933_d_n8;

        let assign36920_e47936: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard745 = assign36920_e47936;

        let (assign36930_e47944, assign36930_e47944_d_n5, assign36930_e47944_d_n6, assign36930_e47944_d_n7, assign36930_e47944_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign36930_e47944;
        var_ijungat_dn5 = assign36930_e47944_d_n5;
        var_ijungat_dn6 = assign36930_e47944_d_n6;
        var_ijungat_dn7 = assign36930_e47944_d_n7;
        var_ijungat_dn8 = assign36930_e47944_d_n8;

        let (assign36940_e47955,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) {
        let assign36940_e47953: f64 = (var_idsatgat_d * var_idmult);
        (assign36940_e47953,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign36940_e47955;

        let assign36950_e47962: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard746 = assign36950_e47962;

        let (assign36960_e47973, assign36960_e47973_d_n5, assign36960_e47973_d_n6, assign36960_e47973_d_n7, assign36960_e47973_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign36960_e47973;
        var_isrh_dn5 = assign36960_e47973_d_n5;
        var_isrh_dn6 = assign36960_e47973_d_n6;
        var_isrh_dn7 = assign36960_e47973_d_n7;
        var_isrh_dn8 = assign36960_e47973_d_n8;

        let (assign36970_e47987,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 == 0.0)) {
        let assign36970_e47985: f64 = (var_vbigat_d - var_vjsrh);
        (assign36970_e47985,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign36970_e47987;

        let (assign36980_e48006,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 == 0.0)) {
        let assign36980_e48001: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign36980_e48002: f64 = (1.0 - assign36980_e48001);
        let assign36980_e48003: f64 = (assign36980_e48002).sqrt();
        let assign36980_e48004: f64 = (1.0 - assign36980_e48003);
        (assign36980_e48004,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign36980_e48006;

        let assign36990_e48009: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard747 = assign36990_e48009;

        let (assign37000_e48023,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 == 0.0)) && (var_guard747 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign37000_e48023;

        let (assign37010_e48055,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 == 0.0)) && (var_guard747 == 0.0)) {
        let assign37010_e48038: f64 = (var_wsrhstep * var_wsrhstep);
        let assign37010_e48040: f64 = (var_wsrhstep).ln();
        let assign37010_e48041: f64 = (assign37010_e48038 * assign37010_e48040);
        let assign37010_e48044: f64 = (1.0 - var_wsrhstep);
        let assign37010_e48045: f64 = (assign37010_e48041 / assign37010_e48044);
        let assign37010_e48047: f64 = (assign37010_e48045 + var_wsrhstep);
        let assign37010_e48051: f64 = (2.0 * var_pgatd_i);
        let assign37010_e48052: f64 = (1.0 - assign37010_e48051);
        let assign37010_e48053: f64 = (assign37010_e48047 * assign37010_e48052);
        (assign37010_e48053,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign37010_e48055;

        let (assign37020_e48069,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 == 0.0)) {
        let assign37020_e48067: f64 = (var_wsrhstep + var_dwsrh);
        (assign37020_e48067,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign37020_e48069;

        let assign37030_e48072: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard748 = assign37030_e48072;

        let (assign37040_e48089, assign37040_e48089_d_n5, assign37040_e48089_d_n6, assign37040_e48089_d_n7, assign37040_e48089_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 == 0.0)) && (var_guard748 != 0.0)) {
        let assign37040_e48086: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign37040_e48087: f64 = (assign37040_e48086).sqrt();
        (assign37040_e48087, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37040_e48089;
        var_tmp_dn5 = assign37040_e48089_d_n5;
        var_tmp_dn6 = assign37040_e48089_d_n6;
        var_tmp_dn7 = assign37040_e48089_d_n7;
        var_tmp_dn8 = assign37040_e48089_d_n8;

        let (assign37050_e48108, assign37050_e48108_d_n5, assign37050_e48108_d_n6, assign37050_e48108_d_n7, assign37050_e48108_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 == 0.0)) && (var_guard748 == 0.0)) {
        let assign37050_e48104: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign37050_e48106: f64 = (assign37050_e48104).powf(var_pgatd_i);
        (assign37050_e48106, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37050_e48108;
        var_tmp_dn5 = assign37050_e48108_d_n5;
        var_tmp_dn6 = assign37050_e48108_d_n6;
        var_tmp_dn7 = assign37050_e48108_d_n7;
        var_tmp_dn8 = assign37050_e48108_d_n8;

        let (assign37060_e48122, assign37060_e48122_d_n5, assign37060_e48122_d_n6, assign37060_e48122_d_n7, assign37060_e48122_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 == 0.0)) {
        let assign37060_e48120: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign37060_e48120, (var_wdepnulrgat_d * var_tmp_dn5), (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign37060_e48122;
        var_wdep_dn5 = assign37060_e48122_d_n5;
        var_wdep_dn6 = assign37060_e48122_d_n6;
        var_wdep_dn7 = assign37060_e48122_d_n7;
        var_wdep_dn8 = assign37060_e48122_d_n8;

        let (assign37070_e48140, assign37070_e48140_d_n5, assign37070_e48140_d_n6, assign37070_e48140_d_n7, assign37070_e48140_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 == 0.0)) {
        let assign37070_e48135: f64 = (var_zinv - 1.0);
        let assign37070_e48137: f64 = (assign37070_e48135 * var_wdep);
        let assign37070_e48138: f64 = (var_ftdgat_d * assign37070_e48137);
        (assign37070_e48138, (var_ftdgat_d * (assign37070_e48135 * var_wdep_dn5)), (var_ftdgat_d * (assign37070_e48135 * var_wdep_dn6)), (var_ftdgat_d * (assign37070_e48135 * var_wdep_dn7)), (var_ftdgat_d * (assign37070_e48135 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign37070_e48140;
        var_asrh_dn5 = assign37070_e48140_d_n5;
        var_asrh_dn6 = assign37070_e48140_d_n6;
        var_asrh_dn7 = assign37070_e48140_d_n7;
        var_asrh_dn8 = assign37070_e48140_d_n8;

        let (assign37080_e48156, assign37080_e48156_d_n5, assign37080_e48156_d_n6, assign37080_e48156_d_n7, assign37080_e48156_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard746 == 0.0)) {
        let assign37080_e48153: f64 = (var_asrh * var_wsrh);
        let assign37080_e48154: f64 = (var_csrhgatd_i * assign37080_e48153);
        (assign37080_e48154, (var_csrhgatd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign37080_e48156;
        var_isrh_dn5 = assign37080_e48156_d_n5;
        var_isrh_dn6 = assign37080_e48156_d_n6;
        var_isrh_dn7 = assign37080_e48156_d_n7;
        var_isrh_dn8 = assign37080_e48156_d_n8;

        let assign37090_e48159: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard749 = assign37090_e48159;

        let (assign37100_e48170, assign37100_e48170_d_n5, assign37100_e48170_d_n6, assign37100_e48170_d_n7, assign37100_e48170_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign37100_e48170;
        var_itat_dn5 = assign37100_e48170_d_n5;
        var_itat_dn6 = assign37100_e48170_d_n6;
        var_itat_dn7 = assign37100_e48170_d_n7;
        var_itat_dn8 = assign37100_e48170_d_n8;

        let (assign37110_e48188, assign37110_e48188_d_n5, assign37110_e48188_d_n6, assign37110_e48188_d_n7, assign37110_e48188_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37110_e48183: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign37110_e48185: f64 = (assign37110_e48183 / var_vbi_minus_vjsrh);
        let assign37110_e48186: f64 = (var_btatpartgat_d * assign37110_e48185);
        (assign37110_e48186, (var_btatpartgat_d * ((var_wdep_dn5 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign37110_e48188;
        var_btat_dn5 = assign37110_e48188_d_n5;
        var_btat_dn6 = assign37110_e48188_d_n6;
        var_btat_dn7 = assign37110_e48188_d_n7;
        var_btat_dn8 = assign37110_e48188_d_n8;

        let (assign37120_e48204, assign37120_e48204_d_n5, assign37120_e48204_d_n6, assign37120_e48204_d_n7, assign37120_e48204_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37120_e48200: f64 = (0.666666666666667 * var_atatgat_d);
        let assign37120_e48202: f64 = (assign37120_e48200 / var_btat);
        (assign37120_e48202, (-((assign37120_e48200 * var_btat_dn5) / (var_btat * var_btat))), (-((assign37120_e48200 * var_btat_dn6) / (var_btat * var_btat))), (-((assign37120_e48200 * var_btat_dn7) / (var_btat * var_btat))), (-((assign37120_e48200 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign37120_e48204;
        var_twoatatoverthreebtat_dn5 = assign37120_e48204_d_n5;
        var_twoatatoverthreebtat_dn6 = assign37120_e48204_d_n6;
        var_twoatatoverthreebtat_dn7 = assign37120_e48204_d_n7;
        var_twoatatoverthreebtat_dn8 = assign37120_e48204_d_n8;

        let (assign37130_e48218, assign37130_e48218_d_n5, assign37130_e48218_d_n6, assign37130_e48218_d_n7, assign37130_e48218_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37130_e48216: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign37130_e48216, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign37130_e48218;
        var_umaxbeforelimiting_dn5 = assign37130_e48218_d_n5;
        var_umaxbeforelimiting_dn6 = assign37130_e48218_d_n6;
        var_umaxbeforelimiting_dn7 = assign37130_e48218_d_n7;
        var_umaxbeforelimiting_dn8 = assign37130_e48218_d_n8;

        let (assign37140_e48239, assign37140_e48239_d_n5, assign37140_e48239_d_n6, assign37140_e48239_d_n7, assign37140_e48239_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37140_e48230: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign37140_e48233: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign37140_e48235: f64 = (assign37140_e48233 + 1.0);
        let assign37140_e48236: f64 = (assign37140_e48230 / assign37140_e48235);
        let assign37140_e48237: f64 = (assign37140_e48236).sqrt();
        (assign37140_e48237, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign37140_e48235) - (assign37140_e48230 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign37140_e48235 * assign37140_e48235)) / (2.0 * assign37140_e48237)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign37140_e48235) - (assign37140_e48230 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign37140_e48235 * assign37140_e48235)) / (2.0 * assign37140_e48237)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign37140_e48235) - (assign37140_e48230 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign37140_e48235 * assign37140_e48235)) / (2.0 * assign37140_e48237)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign37140_e48235) - (assign37140_e48230 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign37140_e48235 * assign37140_e48235)) / (2.0 * assign37140_e48237)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign37140_e48239;
        var_umax_dn5 = assign37140_e48239_d_n5;
        var_umax_dn6 = assign37140_e48239_d_n6;
        var_umax_dn7 = assign37140_e48239_d_n7;
        var_umax_dn8 = assign37140_e48239_d_n8;

        let (assign37150_e48252, assign37150_e48252_d_n5, assign37150_e48252_d_n6, assign37150_e48252_d_n7, assign37150_e48252_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37150_e48250: f64 = (var_umax).sqrt();
        (assign37150_e48250, (var_umax_dn5 / (2.0 * assign37150_e48250)), (var_umax_dn6 / (2.0 * assign37150_e48250)), (var_umax_dn7 / (2.0 * assign37150_e48250)), (var_umax_dn8 / (2.0 * assign37150_e48250)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign37150_e48252;
        var_sqrtumax_dn5 = assign37150_e48252_d_n5;
        var_sqrtumax_dn6 = assign37150_e48252_d_n6;
        var_sqrtumax_dn7 = assign37150_e48252_d_n7;
        var_sqrtumax_dn8 = assign37150_e48252_d_n8;

        let (assign37160_e48266, assign37160_e48266_d_n5, assign37160_e48266_d_n6, assign37160_e48266_d_n7, assign37160_e48266_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37160_e48264: f64 = (var_umax * var_sqrtumax);
        (assign37160_e48264, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign37160_e48266;
        var_umaxpoweronepointfive_dn5 = assign37160_e48266_d_n5;
        var_umaxpoweronepointfive_dn6 = assign37160_e48266_d_n6;
        var_umaxpoweronepointfive_dn7 = assign37160_e48266_d_n7;
        var_umaxpoweronepointfive_dn8 = assign37160_e48266_d_n8;

        let assign37170_e48268: f64 = (-var_pgatd_i);
        let assign37170_e48270: f64 = (assign37170_e48268 * var_one_over_one_minus_pgat_d);
        let assign37170_e48272: f64 = (-1.0);
        let assign37170_e48273: f64 = if assign37170_e48270 == assign37170_e48272 { 1.0 } else { 0.0 };
        var_guard750 = assign37170_e48273;

        let (assign37180_e48293, assign37180_e48293_d_n5, assign37180_e48293_d_n6, assign37180_e48293_d_n7, assign37180_e48293_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) && (var_guard750 != 0.0)) {
        let assign37180_e48289: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37180_e48290: f64 = (1.0 + assign37180_e48289);
        let assign37180_e48291: f64 = (1.0 / assign37180_e48290);
        (assign37180_e48291, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign37180_e48290 * assign37180_e48290))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign37180_e48290 * assign37180_e48290))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign37180_e48290 * assign37180_e48290))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign37180_e48290 * assign37180_e48290))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign37180_e48293;
        var_wgamma_dn5 = assign37180_e48293_d_n5;
        var_wgamma_dn6 = assign37180_e48293_d_n6;
        var_wgamma_dn7 = assign37180_e48293_d_n7;
        var_wgamma_dn8 = assign37180_e48293_d_n8;

        let (assign37190_e48317, assign37190_e48317_d_n5, assign37190_e48317_d_n6, assign37190_e48317_d_n7, assign37190_e48317_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) && (var_guard750 == 0.0)) {
        let assign37190_e48309: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37190_e48310: f64 = (1.0 + assign37190_e48309);
        let assign37190_e48312: f64 = (-var_pgatd_i);
        let assign37190_e48314: f64 = (assign37190_e48312 * var_one_over_one_minus_pgat_d);
        let assign37190_e48315: f64 = (assign37190_e48310).powf(assign37190_e48314);
        (assign37190_e48315, if 0.0 == 0.0 && ((assign37190_e48314) as f64).is_finite() && ((assign37190_e48314) as f64).fract() == 0.0 { if assign37190_e48314 == 0.0 { 0.0 } else { (assign37190_e48314 * ((assign37190_e48310).powf(assign37190_e48314 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign37190_e48315 * (assign37190_e48314 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign37190_e48310))) }, if 0.0 == 0.0 && ((assign37190_e48314) as f64).is_finite() && ((assign37190_e48314) as f64).fract() == 0.0 { if assign37190_e48314 == 0.0 { 0.0 } else { (assign37190_e48314 * ((assign37190_e48310).powf(assign37190_e48314 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign37190_e48315 * (assign37190_e48314 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign37190_e48310))) }, if 0.0 == 0.0 && ((assign37190_e48314) as f64).is_finite() && ((assign37190_e48314) as f64).fract() == 0.0 { if assign37190_e48314 == 0.0 { 0.0 } else { (assign37190_e48314 * ((assign37190_e48310).powf(assign37190_e48314 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign37190_e48315 * (assign37190_e48314 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign37190_e48310))) }, if 0.0 == 0.0 && ((assign37190_e48314) as f64).is_finite() && ((assign37190_e48314) as f64).fract() == 0.0 { if assign37190_e48314 == 0.0 { 0.0 } else { (assign37190_e48314 * ((assign37190_e48310).powf(assign37190_e48314 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign37190_e48315 * (assign37190_e48314 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign37190_e48310))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign37190_e48317;
        var_wgamma_dn5 = assign37190_e48317_d_n5;
        var_wgamma_dn6 = assign37190_e48317_d_n6;
        var_wgamma_dn7 = assign37190_e48317_d_n7;
        var_wgamma_dn8 = assign37190_e48317_d_n8;

        let (assign37200_e48335, assign37200_e48335_d_n5, assign37200_e48335_d_n6, assign37200_e48335_d_n7, assign37200_e48335_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37200_e48329: f64 = (var_wsrh * var_wgamma);
        let assign37200_e48332: f64 = (var_wsrh + var_wgamma);
        let assign37200_e48333: f64 = (assign37200_e48329 / assign37200_e48332);
        (assign37200_e48333, ((((var_wsrh * var_wgamma_dn5) * assign37200_e48332) - (assign37200_e48329 * var_wgamma_dn5)) / (assign37200_e48332 * assign37200_e48332)), ((((var_wsrh * var_wgamma_dn6) * assign37200_e48332) - (assign37200_e48329 * var_wgamma_dn6)) / (assign37200_e48332 * assign37200_e48332)), ((((var_wsrh * var_wgamma_dn7) * assign37200_e48332) - (assign37200_e48329 * var_wgamma_dn7)) / (assign37200_e48332 * assign37200_e48332)), ((((var_wsrh * var_wgamma_dn8) * assign37200_e48332) - (assign37200_e48329 * var_wgamma_dn8)) / (assign37200_e48332 * assign37200_e48332)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign37200_e48335;
        var_wtat_dn5 = assign37200_e48335_d_n5;
        var_wtat_dn6 = assign37200_e48335_d_n6;
        var_wtat_dn7 = assign37200_e48335_d_n7;
        var_wtat_dn8 = assign37200_e48335_d_n8;

        let (assign37210_e48352, assign37210_e48352_d_n5, assign37210_e48352_d_n6, assign37210_e48352_d_n7, assign37210_e48352_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37210_e48348: f64 = (var_btat / var_sqrtumax);
        let assign37210_e48349: f64 = (0.375 * assign37210_e48348);
        let assign37210_e48350: f64 = (assign37210_e48349).sqrt();
        (assign37210_e48350, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37210_e48350)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37210_e48350)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37210_e48350)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37210_e48350)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign37210_e48352;
        var_ktat_dn5 = assign37210_e48352_d_n5;
        var_ktat_dn6 = assign37210_e48352_d_n6;
        var_ktat_dn7 = assign37210_e48352_d_n7;
        var_ktat_dn8 = assign37210_e48352_d_n8;

        let (assign37220_e48370, assign37220_e48370_d_n5, assign37220_e48370_d_n6, assign37220_e48370_d_n7, assign37220_e48370_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37220_e48365: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign37220_e48366: f64 = (2.0 * assign37220_e48365);
        let assign37220_e48368: f64 = (assign37220_e48366 - var_umax);
        (assign37220_e48368, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign37220_e48370;
        var_ltat_dn5 = assign37220_e48370_d_n5;
        var_ltat_dn6 = assign37220_e48370_d_n6;
        var_ltat_dn7 = assign37220_e48370_d_n7;
        var_ltat_dn8 = assign37220_e48370_d_n8;

        let (assign37230_e48396, assign37230_e48396_d_n5, assign37230_e48396_d_n6, assign37230_e48396_d_n7, assign37230_e48396_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37230_e48382: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign37230_e48384: f64 = (assign37230_e48382 * var_sqrtumax);
        let assign37230_e48387: f64 = (var_atatgat_d * var_umax);
        let assign37230_e48388: f64 = (assign37230_e48384 - assign37230_e48387);
        let assign37230_e48392: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37230_e48393: f64 = (0.5 * assign37230_e48392);
        let assign37230_e48394: f64 = (assign37230_e48388 + assign37230_e48393);
        (assign37230_e48394, (((((var_atatgat_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign37230_e48382 * var_sqrtumax_dn5)) - (var_atatgat_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign37230_e48382 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign37230_e48382 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign37230_e48382 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign37230_e48396;
        var_mtat_dn5 = assign37230_e48396_d_n5;
        var_mtat_dn6 = assign37230_e48396_d_n6;
        var_mtat_dn7 = assign37230_e48396_d_n7;
        var_mtat_dn8 = assign37230_e48396_d_n8;

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
        *var_guard742_slot = var_guard742;
        *var_guard743_slot = var_guard743;
        *var_guard744_slot = var_guard744;
        *var_guard745_slot = var_guard745;
        *var_guard746_slot = var_guard746;
        *var_guard747_slot = var_guard747;
        *var_guard748_slot = var_guard748;
        *var_guard749_slot = var_guard749;
        *var_guard750_slot = var_guard750;
        *var_id__blk219_slot = var_id__blk219;
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
    }

    pub(super) fn stamp_transient_block_78(
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat_d: f64,
        var_berfc: f64,
        var_cbbtgatd_i: f64,
        var_cerfc: f64,
        var_ctatgatd_i: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn5: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard745: f64,
        var_guard749: f64,
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
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_vbrgatd_i: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn5: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_wdepnulrinvgat_d: f64,
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
        var_guard751_slot: &mut f64,
        var_guard752_slot: &mut f64,
        var_guard753_slot: &mut f64,
        var_guard754_slot: &mut f64,
        var_guard755_slot: &mut f64,
        var_guard756_slot: &mut f64,
        var_guard757_slot: &mut f64,
        var_guard758_slot: &mut f64,
        var_guard759_slot: &mut f64,
        var_guard760_slot: &mut f64,
        var_guard761_slot: &mut f64,
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
        let mut var_guard751: f64 = *var_guard751_slot;
        let mut var_guard752: f64 = *var_guard752_slot;
        let mut var_guard753: f64 = *var_guard753_slot;
        let mut var_guard754: f64 = *var_guard754_slot;
        let mut var_guard755: f64 = *var_guard755_slot;
        let mut var_guard756: f64 = *var_guard756_slot;
        let mut var_guard757: f64 = *var_guard757_slot;
        let mut var_guard758: f64 = *var_guard758_slot;
        let mut var_guard759: f64 = *var_guard759_slot;
        let mut var_guard760: f64 = *var_guard760_slot;
        let mut var_guard761: f64 = *var_guard761_slot;
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

        let (assign37240_e48412, assign37240_e48412_d_n5, assign37240_e48412_d_n6, assign37240_e48412_d_n7, assign37240_e48412_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37240_e48408: f64 = (var_ltat - 1.0);
        let assign37240_e48410: f64 = (assign37240_e48408 * var_ktat);
        (assign37240_e48410, ((var_ltat_dn5 * var_ktat) + (assign37240_e48408 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign37240_e48408 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign37240_e48408 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign37240_e48408 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign37240_e48412;
        var_xerfc_dn5 = assign37240_e48412_d_n5;
        var_xerfc_dn6 = assign37240_e48412_d_n6;
        var_xerfc_dn7 = assign37240_e48412_d_n7;
        var_xerfc_dn8 = assign37240_e48412_d_n8;

        let (assign37250_e48426, assign37250_e48426_d_n5, assign37250_e48426_d_n6, assign37250_e48426_d_n7, assign37250_e48426_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37250_e48424: f64 = (var_xerfc * var_xerfc);
        (assign37250_e48424, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign37250_e48426;
        var_ysq_dn5 = assign37250_e48426_d_n5;
        var_ysq_dn6 = assign37250_e48426_d_n6;
        var_ysq_dn7 = assign37250_e48426_d_n7;
        var_ysq_dn8 = assign37250_e48426_d_n8;

        let assign37260_e48429: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard751 = assign37260_e48429;

        let (assign37270_e48449, assign37270_e48449_d_n5, assign37270_e48449_d_n6, assign37270_e48449_d_n7, assign37270_e48449_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) && (var_guard751 != 0.0)) {
        let assign37270_e48445: f64 = (var_perfc * var_xerfc);
        let assign37270_e48446: f64 = (1.0 + assign37270_e48445);
        let assign37270_e48447: f64 = (1.0 / assign37270_e48446);
        (assign37270_e48447, (-((var_perfc * var_xerfc_dn5) / (assign37270_e48446 * assign37270_e48446))), (-((var_perfc * var_xerfc_dn6) / (assign37270_e48446 * assign37270_e48446))), (-((var_perfc * var_xerfc_dn7) / (assign37270_e48446 * assign37270_e48446))), (-((var_perfc * var_xerfc_dn8) / (assign37270_e48446 * assign37270_e48446))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign37270_e48449;
        var_terfc_dn5 = assign37270_e48449_d_n5;
        var_terfc_dn6 = assign37270_e48449_d_n6;
        var_terfc_dn7 = assign37270_e48449_d_n7;
        var_terfc_dn8 = assign37270_e48449_d_n8;

        let (assign37280_e48470, assign37280_e48470_d_n5, assign37280_e48470_d_n6, assign37280_e48470_d_n7, assign37280_e48470_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) && (var_guard751 == 0.0)) {
        let assign37280_e48466: f64 = (var_perfc * var_xerfc);
        let assign37280_e48467: f64 = (1.0 - assign37280_e48466);
        let assign37280_e48468: f64 = (1.0 / assign37280_e48467);
        (assign37280_e48468, (-((-(var_perfc * var_xerfc_dn5)) / (assign37280_e48467 * assign37280_e48467))), (-((-(var_perfc * var_xerfc_dn6)) / (assign37280_e48467 * assign37280_e48467))), (-((-(var_perfc * var_xerfc_dn7)) / (assign37280_e48467 * assign37280_e48467))), (-((-(var_perfc * var_xerfc_dn8)) / (assign37280_e48467 * assign37280_e48467))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign37280_e48470;
        var_terfc_dn5 = assign37280_e48470_d_n5;
        var_terfc_dn6 = assign37280_e48470_d_n6;
        var_terfc_dn7 = assign37280_e48470_d_n7;
        var_terfc_dn8 = assign37280_e48470_d_n8;

        let assign37290_e48472: f64 = (-var_ysq);
        let assign37290_e48474: f64 = (assign37290_e48472 + var_mtat);
        let assign37290_e48476: f64 = (-230.25850929940458);
        let assign37290_e48477: f64 = if assign37290_e48474 > assign37290_e48476 { 1.0 } else { 0.0 };
        var_guard752 = assign37290_e48477;

        let (assign37300_e48495, assign37300_e48495_d_n5, assign37300_e48495_d_n6, assign37300_e48495_d_n7, assign37300_e48495_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) && (var_guard752 != 0.0)) {
        let assign37300_e48490: f64 = (-var_ysq);
        let assign37300_e48492: f64 = (assign37300_e48490 + var_mtat);
        let assign37300_e48493: f64 = (assign37300_e48492).exp();
        (assign37300_e48493, (assign37300_e48493 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign37300_e48493 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign37300_e48493 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign37300_e48493 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37300_e48495;
        var_tmp_dn5 = assign37300_e48495_d_n5;
        var_tmp_dn6 = assign37300_e48495_d_n6;
        var_tmp_dn7 = assign37300_e48495_d_n7;
        var_tmp_dn8 = assign37300_e48495_d_n8;

        let (assign37310_e48544, assign37310_e48544_d_n5, assign37310_e48544_d_n6, assign37310_e48544_d_n7, assign37310_e48544_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) && (var_guard752 == 0.0)) {
        let assign37310_e48511: f64 = (-230.25850929940458);
        let assign37310_e48513: f64 = (-var_ysq);
        let assign37310_e48515: f64 = (assign37310_e48513 + var_mtat);
        let assign37310_e48516: f64 = (assign37310_e48511 - assign37310_e48515);
        let assign37310_e48520: f64 = (-230.25850929940458);
        let assign37310_e48522: f64 = (-var_ysq);
        let assign37310_e48524: f64 = (assign37310_e48522 + var_mtat);
        let assign37310_e48525: f64 = (assign37310_e48520 - assign37310_e48524);
        let assign37310_e48528: f64 = (-230.25850929940458);
        let assign37310_e48530: f64 = (-var_ysq);
        let assign37310_e48532: f64 = (assign37310_e48530 + var_mtat);
        let assign37310_e48533: f64 = (assign37310_e48528 - assign37310_e48532);
        let assign37310_e48535: f64 = (assign37310_e48533 * 0.3333333333333333);
        let assign37310_e48536: f64 = (1.0 + assign37310_e48535);
        let assign37310_e48537: f64 = (assign37310_e48525 * assign37310_e48536);
        let assign37310_e48538: f64 = (0.5 * assign37310_e48537);
        let assign37310_e48539: f64 = (1.0 + assign37310_e48538);
        let assign37310_e48540: f64 = (assign37310_e48516 * assign37310_e48539);
        let assign37310_e48541: f64 = (1.0 + assign37310_e48540);
        let assign37310_e48542: f64 = (1e-100 / assign37310_e48541);
        (assign37310_e48542, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign37310_e48539) + (assign37310_e48516 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign37310_e48536) + (assign37310_e48525 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign37310_e48541 * assign37310_e48541))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign37310_e48539) + (assign37310_e48516 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign37310_e48536) + (assign37310_e48525 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign37310_e48541 * assign37310_e48541))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign37310_e48539) + (assign37310_e48516 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign37310_e48536) + (assign37310_e48525 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign37310_e48541 * assign37310_e48541))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign37310_e48539) + (assign37310_e48516 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign37310_e48536) + (assign37310_e48525 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign37310_e48541 * assign37310_e48541))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37310_e48544;
        var_tmp_dn5 = assign37310_e48544_d_n5;
        var_tmp_dn6 = assign37310_e48544_d_n6;
        var_tmp_dn7 = assign37310_e48544_d_n7;
        var_tmp_dn8 = assign37310_e48544_d_n8;

        let (assign37320_e48574, assign37320_e48574_d_n5, assign37320_e48574_d_n6, assign37320_e48574_d_n7, assign37320_e48574_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37320_e48556: f64 = (0.29214664 * var_terfc);
        let assign37320_e48560: f64 = (var_terfc * var_terfc);
        let assign37320_e48561: f64 = (var_berfc * assign37320_e48560);
        let assign37320_e48562: f64 = (assign37320_e48556 + assign37320_e48561);
        let assign37320_e48566: f64 = (var_terfc * var_terfc);
        let assign37320_e48568: f64 = (assign37320_e48566 * var_terfc);
        let assign37320_e48569: f64 = (var_cerfc * assign37320_e48568);
        let assign37320_e48570: f64 = (assign37320_e48562 + assign37320_e48569);
        let assign37320_e48572: f64 = (assign37320_e48570 * var_tmp);
        (assign37320_e48572, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign37320_e48566 * var_terfc_dn5)))) * var_tmp) + (assign37320_e48570 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign37320_e48566 * var_terfc_dn6)))) * var_tmp) + (assign37320_e48570 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign37320_e48566 * var_terfc_dn7)))) * var_tmp) + (assign37320_e48570 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign37320_e48566 * var_terfc_dn8)))) * var_tmp) + (assign37320_e48570 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign37320_e48574;
        var_erfcpos_dn5 = assign37320_e48574_d_n5;
        var_erfcpos_dn6 = assign37320_e48574_d_n6;
        var_erfcpos_dn7 = assign37320_e48574_d_n7;
        var_erfcpos_dn8 = assign37320_e48574_d_n8;

        let assign37330_e48577: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard753 = assign37330_e48577;

        let (assign37340_e48591, assign37340_e48591_d_n5, assign37340_e48591_d_n6, assign37340_e48591_d_n7, assign37340_e48591_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) && (var_guard753 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign37340_e48591;
        var_erfctimesexpmtat_dn5 = assign37340_e48591_d_n5;
        var_erfctimesexpmtat_dn6 = assign37340_e48591_d_n6;
        var_erfctimesexpmtat_dn7 = assign37340_e48591_d_n7;
        var_erfctimesexpmtat_dn8 = assign37340_e48591_d_n8;

        let assign37350_e48594: f64 = (-230.25850929940458);
        let assign37350_e48595: f64 = if var_mtat > assign37350_e48594 { 1.0 } else { 0.0 };
        var_guard754 = assign37350_e48595;

        let (assign37360_e48613, assign37360_e48613_d_n5, assign37360_e48613_d_n6, assign37360_e48613_d_n7, assign37360_e48613_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) && (var_guard753 == 0.0)) && (var_guard754 != 0.0)) {
        let assign37360_e48611: f64 = (var_mtat).exp();
        (assign37360_e48611, (assign37360_e48611 * var_mtat_dn5), (assign37360_e48611 * var_mtat_dn6), (assign37360_e48611 * var_mtat_dn7), (assign37360_e48611 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37360_e48613;
        var_tmp_dn5 = assign37360_e48613_d_n5;
        var_tmp_dn6 = assign37360_e48613_d_n6;
        var_tmp_dn7 = assign37360_e48613_d_n7;
        var_tmp_dn8 = assign37360_e48613_d_n8;

        let (assign37370_e48656, assign37370_e48656_d_n5, assign37370_e48656_d_n6, assign37370_e48656_d_n7, assign37370_e48656_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) && (var_guard753 == 0.0)) && (var_guard754 == 0.0)) {
        let assign37370_e48632: f64 = (-230.25850929940458);
        let assign37370_e48634: f64 = (assign37370_e48632 - var_mtat);
        let assign37370_e48638: f64 = (-230.25850929940458);
        let assign37370_e48640: f64 = (assign37370_e48638 - var_mtat);
        let assign37370_e48643: f64 = (-230.25850929940458);
        let assign37370_e48645: f64 = (assign37370_e48643 - var_mtat);
        let assign37370_e48647: f64 = (assign37370_e48645 * 0.3333333333333333);
        let assign37370_e48648: f64 = (1.0 + assign37370_e48647);
        let assign37370_e48649: f64 = (assign37370_e48640 * assign37370_e48648);
        let assign37370_e48650: f64 = (0.5 * assign37370_e48649);
        let assign37370_e48651: f64 = (1.0 + assign37370_e48650);
        let assign37370_e48652: f64 = (assign37370_e48634 * assign37370_e48651);
        let assign37370_e48653: f64 = (1.0 + assign37370_e48652);
        let assign37370_e48654: f64 = (1e-100 / assign37370_e48653);
        (assign37370_e48654, (-((1e-100 * (((-var_mtat_dn5) * assign37370_e48651) + (assign37370_e48634 * (0.5 * (((-var_mtat_dn5) * assign37370_e48648) + (assign37370_e48640 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign37370_e48653 * assign37370_e48653))), (-((1e-100 * (((-var_mtat_dn6) * assign37370_e48651) + (assign37370_e48634 * (0.5 * (((-var_mtat_dn6) * assign37370_e48648) + (assign37370_e48640 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign37370_e48653 * assign37370_e48653))), (-((1e-100 * (((-var_mtat_dn7) * assign37370_e48651) + (assign37370_e48634 * (0.5 * (((-var_mtat_dn7) * assign37370_e48648) + (assign37370_e48640 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign37370_e48653 * assign37370_e48653))), (-((1e-100 * (((-var_mtat_dn8) * assign37370_e48651) + (assign37370_e48634 * (0.5 * (((-var_mtat_dn8) * assign37370_e48648) + (assign37370_e48640 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign37370_e48653 * assign37370_e48653))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37370_e48656;
        var_tmp_dn5 = assign37370_e48656_d_n5;
        var_tmp_dn6 = assign37370_e48656_d_n6;
        var_tmp_dn7 = assign37370_e48656_d_n7;
        var_tmp_dn8 = assign37370_e48656_d_n8;

        let (assign37380_e48675, assign37380_e48675_d_n5, assign37380_e48675_d_n6, assign37380_e48675_d_n7, assign37380_e48675_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) && (var_guard753 == 0.0)) {
        let assign37380_e48671: f64 = (2.0 * var_tmp);
        let assign37380_e48673: f64 = (assign37380_e48671 - var_erfcpos);
        (assign37380_e48673, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign37380_e48675;
        var_erfctimesexpmtat_dn5 = assign37380_e48675_d_n5;
        var_erfctimesexpmtat_dn6 = assign37380_e48675_d_n6;
        var_erfctimesexpmtat_dn7 = assign37380_e48675_d_n7;
        var_erfctimesexpmtat_dn8 = assign37380_e48675_d_n8;

        let (assign37390_e48695, assign37390_e48695_d_n5, assign37390_e48695_d_n6, assign37390_e48695_d_n7, assign37390_e48695_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37390_e48687: f64 = (1.772453850905516 * 0.5);
        let assign37390_e48690: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign37390_e48692: f64 = (assign37390_e48690 / var_ktat);
        let assign37390_e48693: f64 = (assign37390_e48687 * assign37390_e48692);
        (assign37390_e48693, (assign37390_e48687 * ((((var_atatgat_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign37390_e48690 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign37390_e48687 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign37390_e48690 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign37390_e48687 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign37390_e48690 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign37390_e48687 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign37390_e48690 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign37390_e48695;
        var_gammamax_dn5 = assign37390_e48695_d_n5;
        var_gammamax_dn6 = assign37390_e48695_d_n6;
        var_gammamax_dn7 = assign37390_e48695_d_n7;
        var_gammamax_dn8 = assign37390_e48695_d_n8;

        let (assign37400_e48713, assign37400_e48713_d_n5, assign37400_e48713_d_n6, assign37400_e48713_d_n7, assign37400_e48713_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard749 == 0.0)) {
        let assign37400_e48708: f64 = (var_asrh * var_gammamax);
        let assign37400_e48710: f64 = (assign37400_e48708 * var_wtat);
        let assign37400_e48711: f64 = (var_ctatgatd_i * assign37400_e48710);
        (assign37400_e48711, (var_ctatgatd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign37400_e48708 * var_wtat_dn5))), (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign37400_e48708 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign37400_e48708 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign37400_e48708 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign37400_e48713;
        var_itat_dn5 = assign37400_e48713_d_n5;
        var_itat_dn6 = assign37400_e48713_d_n6;
        var_itat_dn7 = assign37400_e48713_d_n7;
        var_itat_dn8 = assign37400_e48713_d_n8;

        let assign37410_e48716: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard755 = assign37410_e48716;

        let (assign37420_e48727, assign37420_e48727_d_n5, assign37420_e48727_d_n6, assign37420_e48727_d_n7, assign37420_e48727_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard755 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign37420_e48727;
        var_ibbt_dn5 = assign37420_e48727_d_n5;
        var_ibbt_dn6 = assign37420_e48727_d_n6;
        var_ibbt_dn7 = assign37420_e48727_d_n7;
        var_ibbt_dn8 = assign37420_e48727_d_n8;

        let assign37430_e48730: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard756 = assign37430_e48730;

        let (assign37440_e48749, assign37440_e48749_d_n5, assign37440_e48749_d_n6, assign37440_e48749_d_n7, assign37440_e48749_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard755 == 0.0)) && (var_guard756 != 0.0)) {
        let assign37440_e48744: f64 = (var_vbirgatd_i - var_vbbt);
        let assign37440_e48746: f64 = (assign37440_e48744 * var_vbirgatinv_d);
        let assign37440_e48747: f64 = (assign37440_e48746).sqrt();
        (assign37440_e48747, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37440_e48749;
        var_tmp_dn5 = assign37440_e48749_d_n5;
        var_tmp_dn6 = assign37440_e48749_d_n6;
        var_tmp_dn7 = assign37440_e48749_d_n7;
        var_tmp_dn8 = assign37440_e48749_d_n8;

        let (assign37450_e48770, assign37450_e48770_d_n5, assign37450_e48770_d_n6, assign37450_e48770_d_n7, assign37450_e48770_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard755 == 0.0)) && (var_guard756 == 0.0)) {
        let assign37450_e48764: f64 = (var_vbirgatd_i - var_vbbt);
        let assign37450_e48766: f64 = (assign37450_e48764 * var_vbirgatinv_d);
        let assign37450_e48768: f64 = (assign37450_e48766).powf(var_pgatd_i);
        (assign37450_e48768, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37450_e48770;
        var_tmp_dn5 = assign37450_e48770_d_n5;
        var_tmp_dn6 = assign37450_e48770_d_n6;
        var_tmp_dn7 = assign37450_e48770_d_n7;
        var_tmp_dn8 = assign37450_e48770_d_n8;

        let (assign37460_e48790, assign37460_e48790_d_n5, assign37460_e48790_d_n6, assign37460_e48790_d_n7, assign37460_e48790_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard755 == 0.0)) {
        let assign37460_e48783: f64 = (var_vbirgatd_i - var_vbbt);
        let assign37460_e48785: f64 = (assign37460_e48783 * var_wdepnulrinvgat_d);
        let assign37460_e48787: f64 = (assign37460_e48785 / var_tmp);
        let assign37460_e48788: f64 = (var_one_over_one_minus_pgat_d * assign37460_e48787);
        (assign37460_e48788, (var_one_over_one_minus_pgat_d * (-((assign37460_e48785 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign37460_e48785 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign37460_e48785 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign37460_e48785 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign37460_e48790;
        var_fmaxr_dn5 = assign37460_e48790_d_n5;
        var_fmaxr_dn6 = assign37460_e48790_d_n6;
        var_fmaxr_dn7 = assign37460_e48790_d_n7;
        var_fmaxr_dn8 = assign37460_e48790_d_n8;

        let assign37470_e48792: f64 = (-var_fbbtgat_d);
        let assign37470_e48794: f64 = (assign37470_e48792 / var_fmaxr);
        let assign37470_e48795: f64 = (assign37470_e48794).abs();
        let assign37470_e48797: f64 = if assign37470_e48795 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard757 = assign37470_e48797;

        let (assign37480_e48815, assign37480_e48815_d_n5, assign37480_e48815_d_n6, assign37480_e48815_d_n7, assign37480_e48815_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard755 == 0.0)) && (var_guard757 != 0.0)) {
        let assign37480_e48810: f64 = (-var_fbbtgat_d);
        let assign37480_e48812: f64 = (assign37480_e48810 / var_fmaxr);
        let assign37480_e48813: f64 = (assign37480_e48812).exp();
        (assign37480_e48813, (assign37480_e48813 * ((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37480_e48810 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign37480_e48813 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37480_e48810 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign37480_e48813 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37480_e48810 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign37480_e48813 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37480_e48810 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37480_e48815;
        var_tmp_dn5 = assign37480_e48815_d_n5;
        var_tmp_dn6 = assign37480_e48815_d_n6;
        var_tmp_dn7 = assign37480_e48815_d_n7;
        var_tmp_dn8 = assign37480_e48815_d_n8;

        let assign37490_e48817: f64 = (-var_fbbtgat_d);
        let assign37490_e48819: f64 = (assign37490_e48817 / var_fmaxr);
        let assign37490_e48821: f64 = if assign37490_e48819 < 0.0 { 1.0 } else { 0.0 };
        var_guard758 = assign37490_e48821;

        let (assign37500_e48872, assign37500_e48872_d_n5, assign37500_e48872_d_n6, assign37500_e48872_d_n7, assign37500_e48872_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard755 == 0.0)) && (var_guard757 == 0.0)) && (var_guard758 != 0.0)) {
        let assign37500_e48839: f64 = (-230.25850929940458);
        let assign37500_e48841: f64 = (-var_fbbtgat_d);
        let assign37500_e48843: f64 = (assign37500_e48841 / var_fmaxr);
        let assign37500_e48844: f64 = (assign37500_e48839 - assign37500_e48843);
        let assign37500_e48848: f64 = (-230.25850929940458);
        let assign37500_e48850: f64 = (-var_fbbtgat_d);
        let assign37500_e48852: f64 = (assign37500_e48850 / var_fmaxr);
        let assign37500_e48853: f64 = (assign37500_e48848 - assign37500_e48852);
        let assign37500_e48856: f64 = (-230.25850929940458);
        let assign37500_e48858: f64 = (-var_fbbtgat_d);
        let assign37500_e48860: f64 = (assign37500_e48858 / var_fmaxr);
        let assign37500_e48861: f64 = (assign37500_e48856 - assign37500_e48860);
        let assign37500_e48863: f64 = (assign37500_e48861 * 0.3333333333333333);
        let assign37500_e48864: f64 = (1.0 + assign37500_e48863);
        let assign37500_e48865: f64 = (assign37500_e48853 * assign37500_e48864);
        let assign37500_e48866: f64 = (0.5 * assign37500_e48865);
        let assign37500_e48867: f64 = (1.0 + assign37500_e48866);
        let assign37500_e48868: f64 = (assign37500_e48844 * assign37500_e48867);
        let assign37500_e48869: f64 = (1.0 + assign37500_e48868);
        let assign37500_e48870: f64 = (1e-100 / assign37500_e48869);
        (assign37500_e48870, (-((1e-100 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37500_e48841 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign37500_e48867) + (assign37500_e48844 * (0.5 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37500_e48850 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign37500_e48864) + (assign37500_e48853 * ((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37500_e48858 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign37500_e48869 * assign37500_e48869))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37500_e48841 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign37500_e48867) + (assign37500_e48844 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37500_e48850 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign37500_e48864) + (assign37500_e48853 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37500_e48858 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign37500_e48869 * assign37500_e48869))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37500_e48841 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign37500_e48867) + (assign37500_e48844 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37500_e48850 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign37500_e48864) + (assign37500_e48853 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37500_e48858 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign37500_e48869 * assign37500_e48869))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37500_e48841 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign37500_e48867) + (assign37500_e48844 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37500_e48850 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign37500_e48864) + (assign37500_e48853 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37500_e48858 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign37500_e48869 * assign37500_e48869))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37500_e48872;
        var_tmp_dn5 = assign37500_e48872_d_n5;
        var_tmp_dn6 = assign37500_e48872_d_n6;
        var_tmp_dn7 = assign37500_e48872_d_n7;
        var_tmp_dn8 = assign37500_e48872_d_n8;

        let (assign37510_e48921, assign37510_e48921_d_n5, assign37510_e48921_d_n6, assign37510_e48921_d_n7, assign37510_e48921_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard755 == 0.0)) && (var_guard757 == 0.0)) && (var_guard758 == 0.0)) {
        let assign37510_e48891: f64 = (-var_fbbtgat_d);
        let assign37510_e48893: f64 = (assign37510_e48891 / var_fmaxr);
        let assign37510_e48895: f64 = (assign37510_e48893 - 230.25850929940458);
        let assign37510_e48899: f64 = (-var_fbbtgat_d);
        let assign37510_e48901: f64 = (assign37510_e48899 / var_fmaxr);
        let assign37510_e48903: f64 = (assign37510_e48901 - 230.25850929940458);
        let assign37510_e48906: f64 = (-var_fbbtgat_d);
        let assign37510_e48908: f64 = (assign37510_e48906 / var_fmaxr);
        let assign37510_e48910: f64 = (assign37510_e48908 - 230.25850929940458);
        let assign37510_e48912: f64 = (assign37510_e48910 * 0.3333333333333333);
        let assign37510_e48913: f64 = (1.0 + assign37510_e48912);
        let assign37510_e48914: f64 = (assign37510_e48903 * assign37510_e48913);
        let assign37510_e48915: f64 = (0.5 * assign37510_e48914);
        let assign37510_e48916: f64 = (1.0 + assign37510_e48915);
        let assign37510_e48917: f64 = (assign37510_e48895 * assign37510_e48916);
        let assign37510_e48918: f64 = (1.0 + assign37510_e48917);
        let assign37510_e48919: f64 = (1e100 * assign37510_e48918);
        (assign37510_e48919, (1e100 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37510_e48891 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign37510_e48916) + (assign37510_e48895 * (0.5 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37510_e48899 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign37510_e48913) + (assign37510_e48903 * (((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign37510_e48906 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37510_e48891 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign37510_e48916) + (assign37510_e48895 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37510_e48899 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign37510_e48913) + (assign37510_e48903 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign37510_e48906 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37510_e48891 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign37510_e48916) + (assign37510_e48895 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37510_e48899 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign37510_e48913) + (assign37510_e48903 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign37510_e48906 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37510_e48891 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign37510_e48916) + (assign37510_e48895 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37510_e48899 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign37510_e48913) + (assign37510_e48903 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign37510_e48906 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37510_e48921;
        var_tmp_dn5 = assign37510_e48921_d_n5;
        var_tmp_dn6 = assign37510_e48921_d_n6;
        var_tmp_dn7 = assign37510_e48921_d_n7;
        var_tmp_dn8 = assign37510_e48921_d_n8;

        let (assign37520_e48941, assign37520_e48941_d_n5, assign37520_e48941_d_n6, assign37520_e48941_d_n7, assign37520_e48941_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard755 == 0.0)) {
        let assign37520_e48934: f64 = (var_v4 * var_fmaxr);
        let assign37520_e48936: f64 = (assign37520_e48934 * var_fmaxr);
        let assign37520_e48938: f64 = (assign37520_e48936 * var_tmp);
        let assign37520_e48939: f64 = (var_cbbtgatd_i * assign37520_e48938);
        (assign37520_e48939, (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign37520_e48934 * var_fmaxr_dn5)) * var_tmp) + (assign37520_e48936 * var_tmp_dn5))), (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign37520_e48934 * var_fmaxr_dn6)) * var_tmp) + (assign37520_e48936 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign37520_e48934 * var_fmaxr_dn7)) * var_tmp) + (assign37520_e48936 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign37520_e48934 * var_fmaxr_dn8)) * var_tmp) + (assign37520_e48936 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign37520_e48941;
        var_ibbt_dn5 = assign37520_e48941_d_n5;
        var_ibbt_dn6 = assign37520_e48941_d_n6;
        var_ibbt_dn7 = assign37520_e48941_d_n7;
        var_ibbt_dn8 = assign37520_e48941_d_n8;

        let assign37530_e48944: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard759 = assign37530_e48944;

        let (assign37540_e48955, assign37540_e48955_d_n5, assign37540_e48955_d_n6, assign37540_e48955_d_n7, assign37540_e48955_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard759 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign37540_e48955;
        var_fbreakdown_dn5 = assign37540_e48955_d_n5;
        var_fbreakdown_dn6 = assign37540_e48955_d_n6;
        var_fbreakdown_dn7 = assign37540_e48955_d_n7;
        var_fbreakdown_dn8 = assign37540_e48955_d_n8;

        let assign37550_e48958: f64 = (-var_alphaav);
        let assign37550_e48960: f64 = (assign37550_e48958 * var_vbrgatd_i);
        let assign37550_e48961: f64 = if var_vav > assign37550_e48960 { 1.0 } else { 0.0 };
        var_guard760 = assign37550_e48961;

        let assign37560_e48964: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard761 = assign37560_e48964;

        let (assign37570_e48994, assign37570_e48994_d_n5, assign37570_e48994_d_n6, assign37570_e48994_d_n7, assign37570_e48994_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard759 == 0.0)) && (var_guard760 != 0.0)) && (var_guard761 != 0.0)) {
        let assign37570_e48980: f64 = (var_vav * var_vbrinvgat_d);
        let assign37570_e48983: f64 = (var_vav * var_vbrinvgat_d);
        let assign37570_e48984: f64 = (assign37570_e48980 * assign37570_e48983);
        let assign37570_e48987: f64 = (var_vav * var_vbrinvgat_d);
        let assign37570_e48988: f64 = (assign37570_e48984 * assign37570_e48987);
        let assign37570_e48991: f64 = (var_vav * var_vbrinvgat_d);
        let assign37570_e48992: f64 = (assign37570_e48988 * assign37570_e48991);
        (assign37570_e48992, (((((((var_vav * var_vbrinvgat_d_dn5) * assign37570_e48983) + (assign37570_e48980 * (var_vav * var_vbrinvgat_d_dn5))) * assign37570_e48987) + (assign37570_e48984 * (var_vav * var_vbrinvgat_d_dn5))) * assign37570_e48991) + (assign37570_e48988 * (var_vav * var_vbrinvgat_d_dn5))), (((((((var_vav * var_vbrinvgat_d_dn6) * assign37570_e48983) + (assign37570_e48980 * (var_vav * var_vbrinvgat_d_dn6))) * assign37570_e48987) + (assign37570_e48984 * (var_vav * var_vbrinvgat_d_dn6))) * assign37570_e48991) + (assign37570_e48988 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign37570_e48983) + (assign37570_e48980 * (var_vav * var_vbrinvgat_d_dn7))) * assign37570_e48987) + (assign37570_e48984 * (var_vav * var_vbrinvgat_d_dn7))) * assign37570_e48991) + (assign37570_e48988 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign37570_e48983) + (assign37570_e48980 * (var_vav * var_vbrinvgat_d_dn8))) * assign37570_e48987) + (assign37570_e48984 * (var_vav * var_vbrinvgat_d_dn8))) * assign37570_e48991) + (assign37570_e48988 * (var_vav * var_vbrinvgat_d_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37570_e48994;
        var_tmp_dn5 = assign37570_e48994_d_n5;
        var_tmp_dn6 = assign37570_e48994_d_n6;
        var_tmp_dn7 = assign37570_e48994_d_n7;
        var_tmp_dn8 = assign37570_e48994_d_n8;

        let (assign37580_e49016, assign37580_e49016_d_n5, assign37580_e49016_d_n6, assign37580_e49016_d_n7, assign37580_e49016_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard759 == 0.0)) && (var_guard760 != 0.0)) && (var_guard761 == 0.0)) {
        let assign37580_e49011: f64 = (var_vav * var_vbrinvgat_d);
        let assign37580_e49012: f64 = (assign37580_e49011).abs();
        let assign37580_e49014: f64 = (assign37580_e49012).powf(var_pbrgatd_i);
        (assign37580_e49014, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign37580_e49012).powf(var_pbrgatd_i - 1.0) * if assign37580_e49011 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) })) } } else { (assign37580_e49014 * (var_pbrgatd_i * (if assign37580_e49011 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) } / assign37580_e49012))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign37580_e49012).powf(var_pbrgatd_i - 1.0) * if assign37580_e49011 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign37580_e49014 * (var_pbrgatd_i * (if assign37580_e49011 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign37580_e49012))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign37580_e49012).powf(var_pbrgatd_i - 1.0) * if assign37580_e49011 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign37580_e49014 * (var_pbrgatd_i * (if assign37580_e49011 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign37580_e49012))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign37580_e49012).powf(var_pbrgatd_i - 1.0) * if assign37580_e49011 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign37580_e49014 * (var_pbrgatd_i * (if assign37580_e49011 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign37580_e49012))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign37580_e49016;
        var_tmp_dn5 = assign37580_e49016_d_n5;
        var_tmp_dn6 = assign37580_e49016_d_n6;
        var_tmp_dn7 = assign37580_e49016_d_n7;
        var_tmp_dn8 = assign37580_e49016_d_n8;

        let (assign37590_e49034, assign37590_e49034_d_n5, assign37590_e49034_d_n6, assign37590_e49034_d_n7, assign37590_e49034_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard759 == 0.0)) && (var_guard760 != 0.0)) {
        let assign37590_e49031: f64 = (1.0 - var_tmp);
        let assign37590_e49032: f64 = (1.0 / assign37590_e49031);
        (assign37590_e49032, (-((-var_tmp_dn5) / (assign37590_e49031 * assign37590_e49031))), (-((-var_tmp_dn6) / (assign37590_e49031 * assign37590_e49031))), (-((-var_tmp_dn7) / (assign37590_e49031 * assign37590_e49031))), (-((-var_tmp_dn8) / (assign37590_e49031 * assign37590_e49031))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign37590_e49034;
        var_fbreakdown_dn5 = assign37590_e49034_d_n5;
        var_fbreakdown_dn6 = assign37590_e49034_d_n6;
        var_fbreakdown_dn7 = assign37590_e49034_d_n7;
        var_fbreakdown_dn8 = assign37590_e49034_d_n8;

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
        *var_guard751_slot = var_guard751;
        *var_guard752_slot = var_guard752;
        *var_guard753_slot = var_guard753;
        *var_guard754_slot = var_guard754;
        *var_guard755_slot = var_guard755;
        *var_guard756_slot = var_guard756;
        *var_guard757_slot = var_guard757;
        *var_guard758_slot = var_guard758;
        *var_guard759_slot = var_guard759;
        *var_guard760_slot = var_guard760;
        *var_guard761_slot = var_guard761;
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

    pub(super) fn stamp_transient_block_79(
        p: &Parameters,
        var_abdrain_i: f64,
        var_alphaav: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_fstopgat_d: f64,
        var_ftdbot_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard745: f64,
        var_guard759: f64,
        var_guard760: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idsatbot_d: f64,
        var_ijunsti: f64,
        var_ijunsti_dn5: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_itat: f64,
        var_itat_dn5: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_pbotd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn5: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_v5: f64,
        var_vbbtlim_d: f64,
        var_vbibot_d: f64,
        var_vbimin_d: f64,
        var_vbirbotinv_d: f64,
        var_vbrgatd_i: f64,
        var_vmax_d: f64,
        var_wdepnulrbot_d: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard762_slot: &mut f64,
        var_guard763_slot: &mut f64,
        var_guard764_slot: &mut f64,
        var_guard765_slot: &mut f64,
        var_guard766_slot: &mut f64,
        var_guard767_slot: &mut f64,
        var_guard768_slot: &mut f64,
        var_guard769_slot: &mut f64,
        var_guard770_slot: &mut f64,
        var_i4_slot: &mut f64,
        var_i4_dn5_slot: &mut f64,
        var_i4_dn6_slot: &mut f64,
        var_i4_dn7_slot: &mut f64,
        var_i4_dn8_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
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
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_guard762: f64 = *var_guard762_slot;
        let mut var_guard763: f64 = *var_guard763_slot;
        let mut var_guard764: f64 = *var_guard764_slot;
        let mut var_guard765: f64 = *var_guard765_slot;
        let mut var_guard766: f64 = *var_guard766_slot;
        let mut var_guard767: f64 = *var_guard767_slot;
        let mut var_guard768: f64 = *var_guard768_slot;
        let mut var_guard769: f64 = *var_guard769_slot;
        let mut var_guard770: f64 = *var_guard770_slot;
        let mut var_i4: f64 = *var_i4_slot;
        let mut var_i4_dn5: f64 = *var_i4_dn5_slot;
        let mut var_i4_dn6: f64 = *var_i4_dn6_slot;
        let mut var_i4_dn7: f64 = *var_i4_dn7_slot;
        let mut var_i4_dn8: f64 = *var_i4_dn8_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
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

        let (assign37600_e49057, assign37600_e49057_d_n5, assign37600_e49057_d_n6, assign37600_e49057_d_n7, assign37600_e49057_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) && (var_guard759 == 0.0)) && (var_guard760 == 0.0)) {
        let assign37600_e49051: f64 = (var_alphaav * var_vbrgatd_i);
        let assign37600_e49052: f64 = (var_vav + assign37600_e49051);
        let assign37600_e49054: f64 = (assign37600_e49052 * var_slopegat_d);
        let assign37600_e49055: f64 = (var_fstopgat_d + assign37600_e49054);
        (assign37600_e49055, (assign37600_e49052 * var_slopegat_d_dn5), (assign37600_e49052 * var_slopegat_d_dn6), (assign37600_e49052 * var_slopegat_d_dn7), (assign37600_e49052 * var_slopegat_d_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign37600_e49057;
        var_fbreakdown_dn5 = assign37600_e49057_d_n5;
        var_fbreakdown_dn6 = assign37600_e49057_d_n6;
        var_fbreakdown_dn7 = assign37600_e49057_d_n7;
        var_fbreakdown_dn8 = assign37600_e49057_d_n8;

        let (assign37610_e49076, assign37610_e49076_d_n5, assign37610_e49076_d_n6, assign37610_e49076_d_n7, assign37610_e49076_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard745 == 0.0)) {
        let assign37610_e49067: f64 = (var_id__blk219 + var_isrh);
        let assign37610_e49069: f64 = (assign37610_e49067 + var_itat);
        let assign37610_e49071: f64 = (assign37610_e49069 + var_ibbt);
        let assign37610_e49072: f64 = (p.p29 * assign37610_e49071);
        let assign37610_e49074: f64 = (assign37610_e49072 * var_fbreakdown);
        (assign37610_e49074, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign37610_e49072 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign37610_e49072 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign37610_e49072 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign37610_e49072 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign37610_e49076;
        var_ijungat_dn5 = assign37610_e49076_d_n5;
        var_ijungat_dn6 = assign37610_e49076_d_n6;
        var_ijungat_dn7 = assign37610_e49076_d_n7;
        var_ijungat_dn8 = assign37610_e49076_d_n8;

        let (assign37620_e49092, assign37620_e49092_d_n5, assign37620_e49092_d_n6, assign37620_e49092_d_n7, assign37620_e49092_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign37620_e49082: f64 = (var_abdrain_i * var_ijunbot);
        let assign37620_e49085: f64 = (var_lsdrain_i * var_ijunsti);
        let assign37620_e49086: f64 = (assign37620_e49082 + assign37620_e49085);
        let assign37620_e49089: f64 = (var_lgdrain_i * var_ijungat);
        let assign37620_e49090: f64 = (assign37620_e49086 + assign37620_e49089);
        (assign37620_e49090, (((var_abdrain_i * var_ijunbot_dn5) + (var_lsdrain_i * var_ijunsti_dn5)) + (var_lgdrain_i * var_ijungat_dn5)), (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)),)
    } else {
        (var_i4, var_i4_dn5, var_i4_dn6, var_i4_dn7, var_i4_dn8,)
    }
};
        var_i4 = assign37620_e49092;
        var_i4_dn5 = assign37620_e49092_d_n5;
        var_i4_dn6 = assign37620_e49092_d_n6;
        var_i4_dn7 = assign37620_e49092_d_n7;
        var_i4_dn8 = assign37620_e49092_d_n8;

        let (assign37630_e49098,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign37630_e49098;

        let (assign37640_e49104,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign37640_e49104;

        let assign37650_e49116: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard762 = assign37650_e49116;

        let assign37730_e49202: f64 = if var_v5 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard763 = assign37730_e49202;

        let assign37740_e49204: f64 = (-0.5);
        let assign37740_e49207: f64 = (var_v5 * var_phitdinv);
        let assign37740_e49208: f64 = (assign37740_e49204 * assign37740_e49207);
        let assign37740_e49209: f64 = (assign37740_e49208).abs();
        let assign37740_e49211: f64 = if assign37740_e49209 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard764 = assign37740_e49211;

        let (assign37750_e49229,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) && (var_guard763 != 0.0)) && (var_guard764 != 0.0)) {
        let assign37750_e49222: f64 = (-0.5);
        let assign37750_e49225: f64 = (var_v5 * var_phitdinv);
        let assign37750_e49226: f64 = (assign37750_e49222 * assign37750_e49225);
        let assign37750_e49227: f64 = (assign37750_e49226).exp();
        (assign37750_e49227,)
    } else {
        (var_z,)
    }
};
        var_z = assign37750_e49229;

        let assign37760_e49231: f64 = (-0.5);
        let assign37760_e49234: f64 = (var_v5 * var_phitdinv);
        let assign37760_e49235: f64 = (assign37760_e49231 * assign37760_e49234);
        let assign37760_e49237: f64 = if assign37760_e49235 < 0.0 { 1.0 } else { 0.0 };
        var_guard765 = assign37760_e49237;

        let (assign37770_e49292,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) && (var_guard763 != 0.0)) && (var_guard764 == 0.0)) && (var_guard765 != 0.0)) {
        let assign37770_e49253: f64 = (-230.25850929940458);
        let assign37770_e49255: f64 = (-0.5);
        let assign37770_e49258: f64 = (var_v5 * var_phitdinv);
        let assign37770_e49259: f64 = (assign37770_e49255 * assign37770_e49258);
        let assign37770_e49260: f64 = (assign37770_e49253 - assign37770_e49259);
        let assign37770_e49264: f64 = (-230.25850929940458);
        let assign37770_e49266: f64 = (-0.5);
        let assign37770_e49269: f64 = (var_v5 * var_phitdinv);
        let assign37770_e49270: f64 = (assign37770_e49266 * assign37770_e49269);
        let assign37770_e49271: f64 = (assign37770_e49264 - assign37770_e49270);
        let assign37770_e49274: f64 = (-230.25850929940458);
        let assign37770_e49276: f64 = (-0.5);
        let assign37770_e49279: f64 = (var_v5 * var_phitdinv);
        let assign37770_e49280: f64 = (assign37770_e49276 * assign37770_e49279);
        let assign37770_e49281: f64 = (assign37770_e49274 - assign37770_e49280);
        let assign37770_e49283: f64 = (assign37770_e49281 * 0.3333333333333333);
        let assign37770_e49284: f64 = (1.0 + assign37770_e49283);
        let assign37770_e49285: f64 = (assign37770_e49271 * assign37770_e49284);
        let assign37770_e49286: f64 = (0.5 * assign37770_e49285);
        let assign37770_e49287: f64 = (1.0 + assign37770_e49286);
        let assign37770_e49288: f64 = (assign37770_e49260 * assign37770_e49287);
        let assign37770_e49289: f64 = (1.0 + assign37770_e49288);
        let assign37770_e49290: f64 = (1e-100 / assign37770_e49289);
        (assign37770_e49290,)
    } else {
        (var_z,)
    }
};
        var_z = assign37770_e49292;

        let (assign37780_e49345,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) && (var_guard763 != 0.0)) && (var_guard764 == 0.0)) && (var_guard765 == 0.0)) {
        let assign37780_e49309: f64 = (-0.5);
        let assign37780_e49312: f64 = (var_v5 * var_phitdinv);
        let assign37780_e49313: f64 = (assign37780_e49309 * assign37780_e49312);
        let assign37780_e49315: f64 = (assign37780_e49313 - 230.25850929940458);
        let assign37780_e49319: f64 = (-0.5);
        let assign37780_e49322: f64 = (var_v5 * var_phitdinv);
        let assign37780_e49323: f64 = (assign37780_e49319 * assign37780_e49322);
        let assign37780_e49325: f64 = (assign37780_e49323 - 230.25850929940458);
        let assign37780_e49328: f64 = (-0.5);
        let assign37780_e49331: f64 = (var_v5 * var_phitdinv);
        let assign37780_e49332: f64 = (assign37780_e49328 * assign37780_e49331);
        let assign37780_e49334: f64 = (assign37780_e49332 - 230.25850929940458);
        let assign37780_e49336: f64 = (assign37780_e49334 * 0.3333333333333333);
        let assign37780_e49337: f64 = (1.0 + assign37780_e49336);
        let assign37780_e49338: f64 = (assign37780_e49325 * assign37780_e49337);
        let assign37780_e49339: f64 = (0.5 * assign37780_e49338);
        let assign37780_e49340: f64 = (1.0 + assign37780_e49339);
        let assign37780_e49341: f64 = (assign37780_e49315 * assign37780_e49340);
        let assign37780_e49342: f64 = (1.0 + assign37780_e49341);
        let assign37780_e49343: f64 = (1e100 * assign37780_e49342);
        (assign37780_e49343,)
    } else {
        (var_z,)
    }
};
        var_z = assign37780_e49345;

        let (assign37790_e49357,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) && (var_guard763 != 0.0)) {
        let assign37790_e49355: f64 = (1.0 / var_z);
        (assign37790_e49355,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign37790_e49357;

        let (assign37800_e49369,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) && (var_guard763 != 0.0)) {
        let assign37800_e49367: f64 = (var_zinv * var_zinv);
        (assign37800_e49367,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign37800_e49369;

        let (assign37810_e49388,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) && (var_guard763 == 0.0)) {
        let assign37810_e49381: f64 = (var_v5 - var_vmax_d);
        let assign37810_e49383: f64 = (assign37810_e49381 * var_phitdinv);
        let assign37810_e49384: f64 = (1.0 + assign37810_e49383);
        let assign37810_e49386: f64 = (assign37810_e49384 * var_exp_vmax_over_phitd_d);
        (assign37810_e49386,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign37810_e49388;

        let (assign37820_e49400,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) && (var_guard763 == 0.0)) {
        let assign37820_e49398: f64 = (var_idmult).sqrt();
        (assign37820_e49398,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign37820_e49400;

        let (assign37830_e49413,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) && (var_guard763 == 0.0)) {
        let assign37830_e49411: f64 = (1.0 / var_zinv);
        (assign37830_e49411,)
    } else {
        (var_z,)
    }
};
        var_z = assign37830_e49413;

        let (assign37840_e49423,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) {
        let assign37840_e49421: f64 = (var_idmult - 1.0);
        (assign37840_e49421,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign37840_e49423;

        let assign37850_e49426: f64 = if var_v5 > 0.0 { 1.0 } else { 0.0 };
        var_guard766 = assign37850_e49426;

        let (assign37860_e49452,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) && (var_guard766 != 0.0)) {
        let assign37860_e49438: f64 = (2.0 + var_z);
        let assign37860_e49441: f64 = (var_z + 1.0);
        let assign37860_e49444: f64 = (var_z + 3.0);
        let assign37860_e49445: f64 = (assign37860_e49441 * assign37860_e49444);
        let assign37860_e49446: f64 = (assign37860_e49445).sqrt();
        let assign37860_e49447: f64 = (assign37860_e49438 + assign37860_e49446);
        let assign37860_e49448: f64 = (assign37860_e49447).ln();
        let assign37860_e49449: f64 = (var_phitd * assign37860_e49448);
        let assign37860_e49450: f64 = (2.0 * assign37860_e49449);
        (assign37860_e49450,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign37860_e49452;

        let (assign37870_e49486,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) && (var_guard766 == 0.0)) {
        let assign37870_e49462: f64 = (-var_v5);
        let assign37870_e49467: f64 = (2.0 * var_zinv);
        let assign37870_e49469: f64 = (assign37870_e49467 + 1.0);
        let assign37870_e49472: f64 = (1.0 + var_zinv);
        let assign37870_e49476: f64 = (3.0 * var_zinv);
        let assign37870_e49477: f64 = (1.0 + assign37870_e49476);
        let assign37870_e49478: f64 = (assign37870_e49472 * assign37870_e49477);
        let assign37870_e49479: f64 = (assign37870_e49478).sqrt();
        let assign37870_e49480: f64 = (assign37870_e49469 + assign37870_e49479);
        let assign37870_e49481: f64 = (assign37870_e49480).ln();
        let assign37870_e49482: f64 = (var_phitd * assign37870_e49481);
        let assign37870_e49483: f64 = (2.0 * assign37870_e49482);
        let assign37870_e49484: f64 = (assign37870_e49462 + assign37870_e49483);
        (assign37870_e49484,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign37870_e49486;

        let (assign37880_e49496,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) {
        let assign37880_e49494: f64 = (var_vbimin_d - var_two_psistar);
        (assign37880_e49494,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign37880_e49496;

        let (assign37890_e49523,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) {
        let assign37890_e49505: f64 = (var_v5 + var_vjlim);
        let assign37890_e49508: f64 = (var_v5 - var_vjlim);
        let assign37890_e49511: f64 = (var_v5 - var_vjlim);
        let assign37890_e49512: f64 = (assign37890_e49508 * assign37890_e49511);
        let assign37890_e49515: f64 = (4.0 * var_phitd);
        let assign37890_e49517: f64 = (assign37890_e49515 * var_phitd);
        let assign37890_e49518: f64 = (assign37890_e49512 + assign37890_e49517);
        let assign37890_e49519: f64 = (assign37890_e49518).sqrt();
        let assign37890_e49520: f64 = (assign37890_e49505 - assign37890_e49519);
        let assign37890_e49521: f64 = (0.5 * assign37890_e49520);
        (assign37890_e49521,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign37890_e49523;

        let (assign37900_e49550,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) {
        let assign37900_e49532: f64 = (var_v5 + var_vbbtlim_d);
        let assign37900_e49535: f64 = (var_v5 - var_vbbtlim_d);
        let assign37900_e49538: f64 = (var_v5 - var_vbbtlim_d);
        let assign37900_e49539: f64 = (assign37900_e49535 * assign37900_e49538);
        let assign37900_e49542: f64 = (4.0 * var_phitr);
        let assign37900_e49544: f64 = (assign37900_e49542 * var_phitr);
        let assign37900_e49545: f64 = (assign37900_e49539 + assign37900_e49544);
        let assign37900_e49546: f64 = (assign37900_e49545).sqrt();
        let assign37900_e49547: f64 = (assign37900_e49532 - assign37900_e49546);
        let assign37900_e49548: f64 = (0.5 * assign37900_e49547);
        (assign37900_e49548,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign37900_e49550;

        let (assign37910_e49577,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard762 != 0.0)) {
        let assign37910_e49559: f64 = var_v5;
        let assign37910_e49562: f64 = var_v5;
        let assign37910_e49565: f64 = var_v5;
        let assign37910_e49566: f64 = (assign37910_e49562 * assign37910_e49565);
        let assign37910_e49569: f64 = (4.0 * 1e-6);
        let assign37910_e49571: f64 = (assign37910_e49569 * 1e-6);
        let assign37910_e49572: f64 = (assign37910_e49566 + assign37910_e49571);
        let assign37910_e49573: f64 = (assign37910_e49572).sqrt();
        let assign37910_e49574: f64 = (assign37910_e49559 - assign37910_e49573);
        let assign37910_e49575: f64 = (0.5 * assign37910_e49574);
        (assign37910_e49575,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign37910_e49577;

        let assign37920_e49580: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard767 = assign37920_e49580;

        let (assign37930_e49588, assign37930_e49588_d_n5, assign37930_e49588_d_n6, assign37930_e49588_d_n7, assign37930_e49588_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign37930_e49588;
        var_ijunbot_dn5 = assign37930_e49588_d_n5;
        var_ijunbot_dn6 = assign37930_e49588_d_n6;
        var_ijunbot_dn7 = assign37930_e49588_d_n7;
        var_ijunbot_dn8 = assign37930_e49588_d_n8;

        let (assign37940_e49599,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) {
        let assign37940_e49597: f64 = (var_idsatbot_d * var_idmult);
        (assign37940_e49597,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign37940_e49599;

        let assign37950_e49606: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard768 = assign37950_e49606;

        let (assign37960_e49617, assign37960_e49617_d_n5, assign37960_e49617_d_n6, assign37960_e49617_d_n7, assign37960_e49617_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) && (var_guard768 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign37960_e49617;
        var_isrh_dn5 = assign37960_e49617_d_n5;
        var_isrh_dn6 = assign37960_e49617_d_n6;
        var_isrh_dn7 = assign37960_e49617_d_n7;
        var_isrh_dn8 = assign37960_e49617_d_n8;

        let (assign37970_e49631,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) && (var_guard768 == 0.0)) {
        let assign37970_e49629: f64 = (var_vbibot_d - var_vjsrh);
        (assign37970_e49629,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign37970_e49631;

        let (assign37980_e49650,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) && (var_guard768 == 0.0)) {
        let assign37980_e49645: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign37980_e49646: f64 = (1.0 - assign37980_e49645);
        let assign37980_e49647: f64 = (assign37980_e49646).sqrt();
        let assign37980_e49648: f64 = (1.0 - assign37980_e49647);
        (assign37980_e49648,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign37980_e49650;

        let assign37990_e49653: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard769 = assign37990_e49653;

        let (assign38000_e49667,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) && (var_guard768 == 0.0)) && (var_guard769 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign38000_e49667;

        let (assign38010_e49699,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) && (var_guard768 == 0.0)) && (var_guard769 == 0.0)) {
        let assign38010_e49682: f64 = (var_wsrhstep * var_wsrhstep);
        let assign38010_e49684: f64 = (var_wsrhstep).ln();
        let assign38010_e49685: f64 = (assign38010_e49682 * assign38010_e49684);
        let assign38010_e49688: f64 = (1.0 - var_wsrhstep);
        let assign38010_e49689: f64 = (assign38010_e49685 / assign38010_e49688);
        let assign38010_e49691: f64 = (assign38010_e49689 + var_wsrhstep);
        let assign38010_e49695: f64 = (2.0 * var_pbotd_i);
        let assign38010_e49696: f64 = (1.0 - assign38010_e49695);
        let assign38010_e49697: f64 = (assign38010_e49691 * assign38010_e49696);
        (assign38010_e49697,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign38010_e49699;

        let (assign38020_e49713,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) && (var_guard768 == 0.0)) {
        let assign38020_e49711: f64 = (var_wsrhstep + var_dwsrh);
        (assign38020_e49711,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign38020_e49713;

        let assign38030_e49716: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard770 = assign38030_e49716;

        let (assign38040_e49733, assign38040_e49733_d_n5, assign38040_e49733_d_n6, assign38040_e49733_d_n7, assign38040_e49733_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) && (var_guard768 == 0.0)) && (var_guard770 != 0.0)) {
        let assign38040_e49730: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign38040_e49731: f64 = (assign38040_e49730).sqrt();
        (assign38040_e49731, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38040_e49733;
        var_tmp_dn5 = assign38040_e49733_d_n5;
        var_tmp_dn6 = assign38040_e49733_d_n6;
        var_tmp_dn7 = assign38040_e49733_d_n7;
        var_tmp_dn8 = assign38040_e49733_d_n8;

        let (assign38050_e49752, assign38050_e49752_d_n5, assign38050_e49752_d_n6, assign38050_e49752_d_n7, assign38050_e49752_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) && (var_guard768 == 0.0)) && (var_guard770 == 0.0)) {
        let assign38050_e49748: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign38050_e49750: f64 = (assign38050_e49748).powf(var_pbotd_i);
        (assign38050_e49750, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38050_e49752;
        var_tmp_dn5 = assign38050_e49752_d_n5;
        var_tmp_dn6 = assign38050_e49752_d_n6;
        var_tmp_dn7 = assign38050_e49752_d_n7;
        var_tmp_dn8 = assign38050_e49752_d_n8;

        let (assign38060_e49766, assign38060_e49766_d_n5, assign38060_e49766_d_n6, assign38060_e49766_d_n7, assign38060_e49766_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) && (var_guard768 == 0.0)) {
        let assign38060_e49764: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign38060_e49764, (var_wdepnulrbot_d * var_tmp_dn5), (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign38060_e49766;
        var_wdep_dn5 = assign38060_e49766_d_n5;
        var_wdep_dn6 = assign38060_e49766_d_n6;
        var_wdep_dn7 = assign38060_e49766_d_n7;
        var_wdep_dn8 = assign38060_e49766_d_n8;

        let (assign38070_e49784, assign38070_e49784_d_n5, assign38070_e49784_d_n6, assign38070_e49784_d_n7, assign38070_e49784_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard767 == 0.0)) && (var_guard768 == 0.0)) {
        let assign38070_e49779: f64 = (var_zinv - 1.0);
        let assign38070_e49781: f64 = (assign38070_e49779 * var_wdep);
        let assign38070_e49782: f64 = (var_ftdbot_d * assign38070_e49781);
        (assign38070_e49782, (var_ftdbot_d * (assign38070_e49779 * var_wdep_dn5)), (var_ftdbot_d * (assign38070_e49779 * var_wdep_dn6)), (var_ftdbot_d * (assign38070_e49779 * var_wdep_dn7)), (var_ftdbot_d * (assign38070_e49779 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign38070_e49784;
        var_asrh_dn5 = assign38070_e49784_d_n5;
        var_asrh_dn6 = assign38070_e49784_d_n6;
        var_asrh_dn7 = assign38070_e49784_d_n7;
        var_asrh_dn8 = assign38070_e49784_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_guard762_slot = var_guard762;
        *var_guard763_slot = var_guard763;
        *var_guard764_slot = var_guard764;
        *var_guard765_slot = var_guard765;
        *var_guard766_slot = var_guard766;
        *var_guard767_slot = var_guard767;
        *var_guard768_slot = var_guard768;
        *var_guard769_slot = var_guard769;
        *var_guard770_slot = var_guard770;
        *var_i4_slot = var_i4;
        *var_i4_dn5_slot = var_i4_dn5;
        *var_i4_dn6_slot = var_i4_dn6;
        *var_i4_dn7_slot = var_i4_dn7;
        *var_i4_dn8_slot = var_i4_dn8;
        *var_id__blk219_slot = var_id__blk219;
        *var_idmult_slot = var_idmult;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
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
