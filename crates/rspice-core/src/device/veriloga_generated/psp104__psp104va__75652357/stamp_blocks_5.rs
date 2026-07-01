#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
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
        var_guard761: f64,
        var_guard765: f64,
        var_guard766: f64,
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
        var_guard767_slot: &mut f64,
        var_guard768_slot: &mut f64,
        var_guard769_slot: &mut f64,
        var_guard770_slot: &mut f64,
        var_guard771_slot: &mut f64,
        var_guard772_slot: &mut f64,
        var_guard773_slot: &mut f64,
        var_guard774_slot: &mut f64,
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
        let mut var_guard767: f64 = *var_guard767_slot;
        let mut var_guard768: f64 = *var_guard768_slot;
        let mut var_guard769: f64 = *var_guard769_slot;
        let mut var_guard770: f64 = *var_guard770_slot;
        let mut var_guard771: f64 = *var_guard771_slot;
        let mut var_guard772: f64 = *var_guard772_slot;
        let mut var_guard773: f64 = *var_guard773_slot;
        let mut var_guard774: f64 = *var_guard774_slot;
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

        let (assign37990_e49678, assign37990_e49678_d_n5, assign37990_e49678_d_n6, assign37990_e49678_d_n7, assign37990_e49678_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) && (var_guard766 != 0.0)) {
        let assign37990_e49674: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37990_e49675: f64 = (1.0 + assign37990_e49674);
        let assign37990_e49676: f64 = (1.0 / assign37990_e49675);
        (assign37990_e49676, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign37990_e49675 * assign37990_e49675))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign37990_e49675 * assign37990_e49675))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign37990_e49675 * assign37990_e49675))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign37990_e49675 * assign37990_e49675))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign37990_e49678;
        var_wgamma_dn5 = assign37990_e49678_d_n5;
        var_wgamma_dn6 = assign37990_e49678_d_n6;
        var_wgamma_dn7 = assign37990_e49678_d_n7;
        var_wgamma_dn8 = assign37990_e49678_d_n8;

        let (assign38000_e49702, assign38000_e49702_d_n5, assign38000_e49702_d_n6, assign38000_e49702_d_n7, assign38000_e49702_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) && (var_guard766 == 0.0)) {
        let assign38000_e49694: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign38000_e49695: f64 = (1.0 + assign38000_e49694);
        let assign38000_e49697: f64 = (-var_pbotd_i);
        let assign38000_e49699: f64 = (assign38000_e49697 * var_one_over_one_minus_pbot_d);
        let assign38000_e49700: f64 = (assign38000_e49695).powf(assign38000_e49699);
        (assign38000_e49700, if 0.0 == 0.0 && ((assign38000_e49699) as f64).is_finite() && ((assign38000_e49699) as f64).fract() == 0.0 { if assign38000_e49699 == 0.0 { 0.0 } else { (assign38000_e49699 * ((assign38000_e49695).powf(assign38000_e49699 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign38000_e49700 * (assign38000_e49699 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign38000_e49695))) }, if 0.0 == 0.0 && ((assign38000_e49699) as f64).is_finite() && ((assign38000_e49699) as f64).fract() == 0.0 { if assign38000_e49699 == 0.0 { 0.0 } else { (assign38000_e49699 * ((assign38000_e49695).powf(assign38000_e49699 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign38000_e49700 * (assign38000_e49699 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign38000_e49695))) }, if 0.0 == 0.0 && ((assign38000_e49699) as f64).is_finite() && ((assign38000_e49699) as f64).fract() == 0.0 { if assign38000_e49699 == 0.0 { 0.0 } else { (assign38000_e49699 * ((assign38000_e49695).powf(assign38000_e49699 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign38000_e49700 * (assign38000_e49699 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign38000_e49695))) }, if 0.0 == 0.0 && ((assign38000_e49699) as f64).is_finite() && ((assign38000_e49699) as f64).fract() == 0.0 { if assign38000_e49699 == 0.0 { 0.0 } else { (assign38000_e49699 * ((assign38000_e49695).powf(assign38000_e49699 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign38000_e49700 * (assign38000_e49699 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign38000_e49695))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign38000_e49702;
        var_wgamma_dn5 = assign38000_e49702_d_n5;
        var_wgamma_dn6 = assign38000_e49702_d_n6;
        var_wgamma_dn7 = assign38000_e49702_d_n7;
        var_wgamma_dn8 = assign38000_e49702_d_n8;

        let (assign38010_e49720, assign38010_e49720_d_n5, assign38010_e49720_d_n6, assign38010_e49720_d_n7, assign38010_e49720_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign38010_e49714: f64 = (var_wsrh * var_wgamma);
        let assign38010_e49717: f64 = (var_wsrh + var_wgamma);
        let assign38010_e49718: f64 = (assign38010_e49714 / assign38010_e49717);
        (assign38010_e49718, ((((var_wsrh * var_wgamma_dn5) * assign38010_e49717) - (assign38010_e49714 * var_wgamma_dn5)) / (assign38010_e49717 * assign38010_e49717)), ((((var_wsrh * var_wgamma_dn6) * assign38010_e49717) - (assign38010_e49714 * var_wgamma_dn6)) / (assign38010_e49717 * assign38010_e49717)), ((((var_wsrh * var_wgamma_dn7) * assign38010_e49717) - (assign38010_e49714 * var_wgamma_dn7)) / (assign38010_e49717 * assign38010_e49717)), ((((var_wsrh * var_wgamma_dn8) * assign38010_e49717) - (assign38010_e49714 * var_wgamma_dn8)) / (assign38010_e49717 * assign38010_e49717)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign38010_e49720;
        var_wtat_dn5 = assign38010_e49720_d_n5;
        var_wtat_dn6 = assign38010_e49720_d_n6;
        var_wtat_dn7 = assign38010_e49720_d_n7;
        var_wtat_dn8 = assign38010_e49720_d_n8;

        let (assign38020_e49737, assign38020_e49737_d_n5, assign38020_e49737_d_n6, assign38020_e49737_d_n7, assign38020_e49737_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign38020_e49733: f64 = (var_btat / var_sqrtumax);
        let assign38020_e49734: f64 = (0.375 * assign38020_e49733);
        let assign38020_e49735: f64 = (assign38020_e49734).sqrt();
        (assign38020_e49735, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38020_e49735)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38020_e49735)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38020_e49735)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38020_e49735)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign38020_e49737;
        var_ktat_dn5 = assign38020_e49737_d_n5;
        var_ktat_dn6 = assign38020_e49737_d_n6;
        var_ktat_dn7 = assign38020_e49737_d_n7;
        var_ktat_dn8 = assign38020_e49737_d_n8;

        let (assign38030_e49755, assign38030_e49755_d_n5, assign38030_e49755_d_n6, assign38030_e49755_d_n7, assign38030_e49755_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign38030_e49750: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign38030_e49751: f64 = (2.0 * assign38030_e49750);
        let assign38030_e49753: f64 = (assign38030_e49751 - var_umax);
        (assign38030_e49753, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign38030_e49755;
        var_ltat_dn5 = assign38030_e49755_d_n5;
        var_ltat_dn6 = assign38030_e49755_d_n6;
        var_ltat_dn7 = assign38030_e49755_d_n7;
        var_ltat_dn8 = assign38030_e49755_d_n8;

        let (assign38040_e49781, assign38040_e49781_d_n5, assign38040_e49781_d_n6, assign38040_e49781_d_n7, assign38040_e49781_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign38040_e49767: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign38040_e49769: f64 = (assign38040_e49767 * var_sqrtumax);
        let assign38040_e49772: f64 = (var_atatbot_d * var_umax);
        let assign38040_e49773: f64 = (assign38040_e49769 - assign38040_e49772);
        let assign38040_e49777: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign38040_e49778: f64 = (0.5 * assign38040_e49777);
        let assign38040_e49779: f64 = (assign38040_e49773 + assign38040_e49778);
        (assign38040_e49779, (((((var_atatbot_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign38040_e49767 * var_sqrtumax_dn5)) - (var_atatbot_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign38040_e49767 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign38040_e49767 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign38040_e49767 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign38040_e49781;
        var_mtat_dn5 = assign38040_e49781_d_n5;
        var_mtat_dn6 = assign38040_e49781_d_n6;
        var_mtat_dn7 = assign38040_e49781_d_n7;
        var_mtat_dn8 = assign38040_e49781_d_n8;

        let (assign38050_e49797, assign38050_e49797_d_n5, assign38050_e49797_d_n6, assign38050_e49797_d_n7, assign38050_e49797_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign38050_e49793: f64 = (var_ltat - 1.0);
        let assign38050_e49795: f64 = (assign38050_e49793 * var_ktat);
        (assign38050_e49795, ((var_ltat_dn5 * var_ktat) + (assign38050_e49793 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign38050_e49793 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign38050_e49793 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign38050_e49793 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign38050_e49797;
        var_xerfc_dn5 = assign38050_e49797_d_n5;
        var_xerfc_dn6 = assign38050_e49797_d_n6;
        var_xerfc_dn7 = assign38050_e49797_d_n7;
        var_xerfc_dn8 = assign38050_e49797_d_n8;

        let (assign38060_e49811, assign38060_e49811_d_n5, assign38060_e49811_d_n6, assign38060_e49811_d_n7, assign38060_e49811_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign38060_e49809: f64 = (var_xerfc * var_xerfc);
        (assign38060_e49809, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign38060_e49811;
        var_ysq_dn5 = assign38060_e49811_d_n5;
        var_ysq_dn6 = assign38060_e49811_d_n6;
        var_ysq_dn7 = assign38060_e49811_d_n7;
        var_ysq_dn8 = assign38060_e49811_d_n8;

        let assign38070_e49814: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard767 = assign38070_e49814;

        let (assign38080_e49834, assign38080_e49834_d_n5, assign38080_e49834_d_n6, assign38080_e49834_d_n7, assign38080_e49834_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) && (var_guard767 != 0.0)) {
        let assign38080_e49830: f64 = (var_perfc * var_xerfc);
        let assign38080_e49831: f64 = (1.0 + assign38080_e49830);
        let assign38080_e49832: f64 = (1.0 / assign38080_e49831);
        (assign38080_e49832, (-((var_perfc * var_xerfc_dn5) / (assign38080_e49831 * assign38080_e49831))), (-((var_perfc * var_xerfc_dn6) / (assign38080_e49831 * assign38080_e49831))), (-((var_perfc * var_xerfc_dn7) / (assign38080_e49831 * assign38080_e49831))), (-((var_perfc * var_xerfc_dn8) / (assign38080_e49831 * assign38080_e49831))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign38080_e49834;
        var_terfc_dn5 = assign38080_e49834_d_n5;
        var_terfc_dn6 = assign38080_e49834_d_n6;
        var_terfc_dn7 = assign38080_e49834_d_n7;
        var_terfc_dn8 = assign38080_e49834_d_n8;

        let (assign38090_e49855, assign38090_e49855_d_n5, assign38090_e49855_d_n6, assign38090_e49855_d_n7, assign38090_e49855_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) && (var_guard767 == 0.0)) {
        let assign38090_e49851: f64 = (var_perfc * var_xerfc);
        let assign38090_e49852: f64 = (1.0 - assign38090_e49851);
        let assign38090_e49853: f64 = (1.0 / assign38090_e49852);
        (assign38090_e49853, (-((-(var_perfc * var_xerfc_dn5)) / (assign38090_e49852 * assign38090_e49852))), (-((-(var_perfc * var_xerfc_dn6)) / (assign38090_e49852 * assign38090_e49852))), (-((-(var_perfc * var_xerfc_dn7)) / (assign38090_e49852 * assign38090_e49852))), (-((-(var_perfc * var_xerfc_dn8)) / (assign38090_e49852 * assign38090_e49852))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign38090_e49855;
        var_terfc_dn5 = assign38090_e49855_d_n5;
        var_terfc_dn6 = assign38090_e49855_d_n6;
        var_terfc_dn7 = assign38090_e49855_d_n7;
        var_terfc_dn8 = assign38090_e49855_d_n8;

        let assign38100_e49857: f64 = (-var_ysq);
        let assign38100_e49859: f64 = (assign38100_e49857 + var_mtat);
        let assign38100_e49861: f64 = (-230.25850929940458);
        let assign38100_e49862: f64 = if assign38100_e49859 > assign38100_e49861 { 1.0 } else { 0.0 };
        var_guard768 = assign38100_e49862;

        let (assign38110_e49880, assign38110_e49880_d_n5, assign38110_e49880_d_n6, assign38110_e49880_d_n7, assign38110_e49880_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) && (var_guard768 != 0.0)) {
        let assign38110_e49875: f64 = (-var_ysq);
        let assign38110_e49877: f64 = (assign38110_e49875 + var_mtat);
        let assign38110_e49878: f64 = (assign38110_e49877).exp();
        (assign38110_e49878, (assign38110_e49878 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign38110_e49878 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign38110_e49878 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign38110_e49878 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38110_e49880;
        var_tmp_dn5 = assign38110_e49880_d_n5;
        var_tmp_dn6 = assign38110_e49880_d_n6;
        var_tmp_dn7 = assign38110_e49880_d_n7;
        var_tmp_dn8 = assign38110_e49880_d_n8;

        let (assign38120_e49929, assign38120_e49929_d_n5, assign38120_e49929_d_n6, assign38120_e49929_d_n7, assign38120_e49929_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) && (var_guard768 == 0.0)) {
        let assign38120_e49896: f64 = (-230.25850929940458);
        let assign38120_e49898: f64 = (-var_ysq);
        let assign38120_e49900: f64 = (assign38120_e49898 + var_mtat);
        let assign38120_e49901: f64 = (assign38120_e49896 - assign38120_e49900);
        let assign38120_e49905: f64 = (-230.25850929940458);
        let assign38120_e49907: f64 = (-var_ysq);
        let assign38120_e49909: f64 = (assign38120_e49907 + var_mtat);
        let assign38120_e49910: f64 = (assign38120_e49905 - assign38120_e49909);
        let assign38120_e49913: f64 = (-230.25850929940458);
        let assign38120_e49915: f64 = (-var_ysq);
        let assign38120_e49917: f64 = (assign38120_e49915 + var_mtat);
        let assign38120_e49918: f64 = (assign38120_e49913 - assign38120_e49917);
        let assign38120_e49920: f64 = (assign38120_e49918 * 0.3333333333333333);
        let assign38120_e49921: f64 = (1.0 + assign38120_e49920);
        let assign38120_e49922: f64 = (assign38120_e49910 * assign38120_e49921);
        let assign38120_e49923: f64 = (0.5 * assign38120_e49922);
        let assign38120_e49924: f64 = (1.0 + assign38120_e49923);
        let assign38120_e49925: f64 = (assign38120_e49901 * assign38120_e49924);
        let assign38120_e49926: f64 = (1.0 + assign38120_e49925);
        let assign38120_e49927: f64 = (1e-100 / assign38120_e49926);
        (assign38120_e49927, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign38120_e49924) + (assign38120_e49901 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign38120_e49921) + (assign38120_e49910 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign38120_e49926 * assign38120_e49926))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign38120_e49924) + (assign38120_e49901 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign38120_e49921) + (assign38120_e49910 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign38120_e49926 * assign38120_e49926))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign38120_e49924) + (assign38120_e49901 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign38120_e49921) + (assign38120_e49910 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign38120_e49926 * assign38120_e49926))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign38120_e49924) + (assign38120_e49901 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign38120_e49921) + (assign38120_e49910 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign38120_e49926 * assign38120_e49926))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38120_e49929;
        var_tmp_dn5 = assign38120_e49929_d_n5;
        var_tmp_dn6 = assign38120_e49929_d_n6;
        var_tmp_dn7 = assign38120_e49929_d_n7;
        var_tmp_dn8 = assign38120_e49929_d_n8;

        let (assign38130_e49959, assign38130_e49959_d_n5, assign38130_e49959_d_n6, assign38130_e49959_d_n7, assign38130_e49959_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign38130_e49941: f64 = (0.29214664 * var_terfc);
        let assign38130_e49945: f64 = (var_terfc * var_terfc);
        let assign38130_e49946: f64 = (var_berfc * assign38130_e49945);
        let assign38130_e49947: f64 = (assign38130_e49941 + assign38130_e49946);
        let assign38130_e49951: f64 = (var_terfc * var_terfc);
        let assign38130_e49953: f64 = (assign38130_e49951 * var_terfc);
        let assign38130_e49954: f64 = (var_cerfc * assign38130_e49953);
        let assign38130_e49955: f64 = (assign38130_e49947 + assign38130_e49954);
        let assign38130_e49957: f64 = (assign38130_e49955 * var_tmp);
        (assign38130_e49957, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign38130_e49951 * var_terfc_dn5)))) * var_tmp) + (assign38130_e49955 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign38130_e49951 * var_terfc_dn6)))) * var_tmp) + (assign38130_e49955 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign38130_e49951 * var_terfc_dn7)))) * var_tmp) + (assign38130_e49955 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign38130_e49951 * var_terfc_dn8)))) * var_tmp) + (assign38130_e49955 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign38130_e49959;
        var_erfcpos_dn5 = assign38130_e49959_d_n5;
        var_erfcpos_dn6 = assign38130_e49959_d_n6;
        var_erfcpos_dn7 = assign38130_e49959_d_n7;
        var_erfcpos_dn8 = assign38130_e49959_d_n8;

        let assign38140_e49962: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard769 = assign38140_e49962;

        let (assign38150_e49976, assign38150_e49976_d_n5, assign38150_e49976_d_n6, assign38150_e49976_d_n7, assign38150_e49976_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) && (var_guard769 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign38150_e49976;
        var_erfctimesexpmtat_dn5 = assign38150_e49976_d_n5;
        var_erfctimesexpmtat_dn6 = assign38150_e49976_d_n6;
        var_erfctimesexpmtat_dn7 = assign38150_e49976_d_n7;
        var_erfctimesexpmtat_dn8 = assign38150_e49976_d_n8;

        let assign38160_e49979: f64 = (-230.25850929940458);
        let assign38160_e49980: f64 = if var_mtat > assign38160_e49979 { 1.0 } else { 0.0 };
        var_guard770 = assign38160_e49980;

        let (assign38170_e49998, assign38170_e49998_d_n5, assign38170_e49998_d_n6, assign38170_e49998_d_n7, assign38170_e49998_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) && (var_guard769 == 0.0)) && (var_guard770 != 0.0)) {
        let assign38170_e49996: f64 = (var_mtat).exp();
        (assign38170_e49996, (assign38170_e49996 * var_mtat_dn5), (assign38170_e49996 * var_mtat_dn6), (assign38170_e49996 * var_mtat_dn7), (assign38170_e49996 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38170_e49998;
        var_tmp_dn5 = assign38170_e49998_d_n5;
        var_tmp_dn6 = assign38170_e49998_d_n6;
        var_tmp_dn7 = assign38170_e49998_d_n7;
        var_tmp_dn8 = assign38170_e49998_d_n8;

        let (assign38180_e50041, assign38180_e50041_d_n5, assign38180_e50041_d_n6, assign38180_e50041_d_n7, assign38180_e50041_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) && (var_guard769 == 0.0)) && (var_guard770 == 0.0)) {
        let assign38180_e50017: f64 = (-230.25850929940458);
        let assign38180_e50019: f64 = (assign38180_e50017 - var_mtat);
        let assign38180_e50023: f64 = (-230.25850929940458);
        let assign38180_e50025: f64 = (assign38180_e50023 - var_mtat);
        let assign38180_e50028: f64 = (-230.25850929940458);
        let assign38180_e50030: f64 = (assign38180_e50028 - var_mtat);
        let assign38180_e50032: f64 = (assign38180_e50030 * 0.3333333333333333);
        let assign38180_e50033: f64 = (1.0 + assign38180_e50032);
        let assign38180_e50034: f64 = (assign38180_e50025 * assign38180_e50033);
        let assign38180_e50035: f64 = (0.5 * assign38180_e50034);
        let assign38180_e50036: f64 = (1.0 + assign38180_e50035);
        let assign38180_e50037: f64 = (assign38180_e50019 * assign38180_e50036);
        let assign38180_e50038: f64 = (1.0 + assign38180_e50037);
        let assign38180_e50039: f64 = (1e-100 / assign38180_e50038);
        (assign38180_e50039, (-((1e-100 * (((-var_mtat_dn5) * assign38180_e50036) + (assign38180_e50019 * (0.5 * (((-var_mtat_dn5) * assign38180_e50033) + (assign38180_e50025 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign38180_e50038 * assign38180_e50038))), (-((1e-100 * (((-var_mtat_dn6) * assign38180_e50036) + (assign38180_e50019 * (0.5 * (((-var_mtat_dn6) * assign38180_e50033) + (assign38180_e50025 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign38180_e50038 * assign38180_e50038))), (-((1e-100 * (((-var_mtat_dn7) * assign38180_e50036) + (assign38180_e50019 * (0.5 * (((-var_mtat_dn7) * assign38180_e50033) + (assign38180_e50025 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign38180_e50038 * assign38180_e50038))), (-((1e-100 * (((-var_mtat_dn8) * assign38180_e50036) + (assign38180_e50019 * (0.5 * (((-var_mtat_dn8) * assign38180_e50033) + (assign38180_e50025 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign38180_e50038 * assign38180_e50038))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38180_e50041;
        var_tmp_dn5 = assign38180_e50041_d_n5;
        var_tmp_dn6 = assign38180_e50041_d_n6;
        var_tmp_dn7 = assign38180_e50041_d_n7;
        var_tmp_dn8 = assign38180_e50041_d_n8;

        let (assign38190_e50060, assign38190_e50060_d_n5, assign38190_e50060_d_n6, assign38190_e50060_d_n7, assign38190_e50060_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) && (var_guard769 == 0.0)) {
        let assign38190_e50056: f64 = (2.0 * var_tmp);
        let assign38190_e50058: f64 = (assign38190_e50056 - var_erfcpos);
        (assign38190_e50058, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign38190_e50060;
        var_erfctimesexpmtat_dn5 = assign38190_e50060_d_n5;
        var_erfctimesexpmtat_dn6 = assign38190_e50060_d_n6;
        var_erfctimesexpmtat_dn7 = assign38190_e50060_d_n7;
        var_erfctimesexpmtat_dn8 = assign38190_e50060_d_n8;

        let (assign38200_e50080, assign38200_e50080_d_n5, assign38200_e50080_d_n6, assign38200_e50080_d_n7, assign38200_e50080_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign38200_e50072: f64 = (1.772453850905516 * 0.5);
        let assign38200_e50075: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign38200_e50077: f64 = (assign38200_e50075 / var_ktat);
        let assign38200_e50078: f64 = (assign38200_e50072 * assign38200_e50077);
        (assign38200_e50078, (assign38200_e50072 * ((((var_atatbot_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign38200_e50075 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign38200_e50072 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign38200_e50075 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign38200_e50072 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign38200_e50075 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign38200_e50072 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign38200_e50075 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign38200_e50080;
        var_gammamax_dn5 = assign38200_e50080_d_n5;
        var_gammamax_dn6 = assign38200_e50080_d_n6;
        var_gammamax_dn7 = assign38200_e50080_d_n7;
        var_gammamax_dn8 = assign38200_e50080_d_n8;

        let (assign38210_e50098, assign38210_e50098_d_n5, assign38210_e50098_d_n6, assign38210_e50098_d_n7, assign38210_e50098_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard765 == 0.0)) {
        let assign38210_e50093: f64 = (var_asrh * var_gammamax);
        let assign38210_e50095: f64 = (assign38210_e50093 * var_wtat);
        let assign38210_e50096: f64 = (var_ctatbotd_i * assign38210_e50095);
        (assign38210_e50096, (var_ctatbotd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign38210_e50093 * var_wtat_dn5))), (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign38210_e50093 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign38210_e50093 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign38210_e50093 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign38210_e50098;
        var_itat_dn5 = assign38210_e50098_d_n5;
        var_itat_dn6 = assign38210_e50098_d_n6;
        var_itat_dn7 = assign38210_e50098_d_n7;
        var_itat_dn8 = assign38210_e50098_d_n8;

        let assign38220_e50101: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard771 = assign38220_e50101;

        let (assign38230_e50112, assign38230_e50112_d_n5, assign38230_e50112_d_n6, assign38230_e50112_d_n7, assign38230_e50112_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard771 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign38230_e50112;
        var_ibbt_dn5 = assign38230_e50112_d_n5;
        var_ibbt_dn6 = assign38230_e50112_d_n6;
        var_ibbt_dn7 = assign38230_e50112_d_n7;
        var_ibbt_dn8 = assign38230_e50112_d_n8;

        let assign38240_e50115: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard772 = assign38240_e50115;

        let (assign38250_e50134, assign38250_e50134_d_n5, assign38250_e50134_d_n6, assign38250_e50134_d_n7, assign38250_e50134_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard771 == 0.0)) && (var_guard772 != 0.0)) {
        let assign38250_e50129: f64 = (var_vbirbotd_i - var_vbbt);
        let assign38250_e50131: f64 = (assign38250_e50129 * var_vbirbotinv_d);
        let assign38250_e50132: f64 = (assign38250_e50131).sqrt();
        (assign38250_e50132, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38250_e50134;
        var_tmp_dn5 = assign38250_e50134_d_n5;
        var_tmp_dn6 = assign38250_e50134_d_n6;
        var_tmp_dn7 = assign38250_e50134_d_n7;
        var_tmp_dn8 = assign38250_e50134_d_n8;

        let (assign38260_e50155, assign38260_e50155_d_n5, assign38260_e50155_d_n6, assign38260_e50155_d_n7, assign38260_e50155_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard771 == 0.0)) && (var_guard772 == 0.0)) {
        let assign38260_e50149: f64 = (var_vbirbotd_i - var_vbbt);
        let assign38260_e50151: f64 = (assign38260_e50149 * var_vbirbotinv_d);
        let assign38260_e50153: f64 = (assign38260_e50151).powf(var_pbotd_i);
        (assign38260_e50153, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38260_e50155;
        var_tmp_dn5 = assign38260_e50155_d_n5;
        var_tmp_dn6 = assign38260_e50155_d_n6;
        var_tmp_dn7 = assign38260_e50155_d_n7;
        var_tmp_dn8 = assign38260_e50155_d_n8;

        let (assign38270_e50175, assign38270_e50175_d_n5, assign38270_e50175_d_n6, assign38270_e50175_d_n7, assign38270_e50175_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard771 == 0.0)) {
        let assign38270_e50168: f64 = (var_vbirbotd_i - var_vbbt);
        let assign38270_e50170: f64 = (assign38270_e50168 * var_wdepnulrinvbot_d);
        let assign38270_e50172: f64 = (assign38270_e50170 / var_tmp);
        let assign38270_e50173: f64 = (var_one_over_one_minus_pbot_d * assign38270_e50172);
        (assign38270_e50173, (var_one_over_one_minus_pbot_d * (-((assign38270_e50170 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign38270_e50170 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign38270_e50170 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign38270_e50170 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign38270_e50175;
        var_fmaxr_dn5 = assign38270_e50175_d_n5;
        var_fmaxr_dn6 = assign38270_e50175_d_n6;
        var_fmaxr_dn7 = assign38270_e50175_d_n7;
        var_fmaxr_dn8 = assign38270_e50175_d_n8;

        let assign38280_e50177: f64 = (-var_fbbtbot_d);
        let assign38280_e50179: f64 = (assign38280_e50177 / var_fmaxr);
        let assign38280_e50180: f64 = (assign38280_e50179).abs();
        let assign38280_e50182: f64 = if assign38280_e50180 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard773 = assign38280_e50182;

        let (assign38290_e50200, assign38290_e50200_d_n5, assign38290_e50200_d_n6, assign38290_e50200_d_n7, assign38290_e50200_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard771 == 0.0)) && (var_guard773 != 0.0)) {
        let assign38290_e50195: f64 = (-var_fbbtbot_d);
        let assign38290_e50197: f64 = (assign38290_e50195 / var_fmaxr);
        let assign38290_e50198: f64 = (assign38290_e50197).exp();
        (assign38290_e50198, (assign38290_e50198 * (-((assign38290_e50195 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign38290_e50198 * (-((assign38290_e50195 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign38290_e50198 * (-((assign38290_e50195 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign38290_e50198 * (-((assign38290_e50195 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38290_e50200;
        var_tmp_dn5 = assign38290_e50200_d_n5;
        var_tmp_dn6 = assign38290_e50200_d_n6;
        var_tmp_dn7 = assign38290_e50200_d_n7;
        var_tmp_dn8 = assign38290_e50200_d_n8;

        let assign38300_e50202: f64 = (-var_fbbtbot_d);
        let assign38300_e50204: f64 = (assign38300_e50202 / var_fmaxr);
        let assign38300_e50206: f64 = if assign38300_e50204 < 0.0 { 1.0 } else { 0.0 };
        var_guard774 = assign38300_e50206;

        let (assign38310_e50257, assign38310_e50257_d_n5, assign38310_e50257_d_n6, assign38310_e50257_d_n7, assign38310_e50257_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard771 == 0.0)) && (var_guard773 == 0.0)) && (var_guard774 != 0.0)) {
        let assign38310_e50224: f64 = (-230.25850929940458);
        let assign38310_e50226: f64 = (-var_fbbtbot_d);
        let assign38310_e50228: f64 = (assign38310_e50226 / var_fmaxr);
        let assign38310_e50229: f64 = (assign38310_e50224 - assign38310_e50228);
        let assign38310_e50233: f64 = (-230.25850929940458);
        let assign38310_e50235: f64 = (-var_fbbtbot_d);
        let assign38310_e50237: f64 = (assign38310_e50235 / var_fmaxr);
        let assign38310_e50238: f64 = (assign38310_e50233 - assign38310_e50237);
        let assign38310_e50241: f64 = (-230.25850929940458);
        let assign38310_e50243: f64 = (-var_fbbtbot_d);
        let assign38310_e50245: f64 = (assign38310_e50243 / var_fmaxr);
        let assign38310_e50246: f64 = (assign38310_e50241 - assign38310_e50245);
        let assign38310_e50248: f64 = (assign38310_e50246 * 0.3333333333333333);
        let assign38310_e50249: f64 = (1.0 + assign38310_e50248);
        let assign38310_e50250: f64 = (assign38310_e50238 * assign38310_e50249);
        let assign38310_e50251: f64 = (0.5 * assign38310_e50250);
        let assign38310_e50252: f64 = (1.0 + assign38310_e50251);
        let assign38310_e50253: f64 = (assign38310_e50229 * assign38310_e50252);
        let assign38310_e50254: f64 = (1.0 + assign38310_e50253);
        let assign38310_e50255: f64 = (1e-100 / assign38310_e50254);
        (assign38310_e50255, (-((1e-100 * (((-(-((assign38310_e50226 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign38310_e50252) + (assign38310_e50229 * (0.5 * (((-(-((assign38310_e50235 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign38310_e50249) + (assign38310_e50238 * ((-(-((assign38310_e50243 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign38310_e50254 * assign38310_e50254))), (-((1e-100 * (((-(-((assign38310_e50226 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign38310_e50252) + (assign38310_e50229 * (0.5 * (((-(-((assign38310_e50235 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign38310_e50249) + (assign38310_e50238 * ((-(-((assign38310_e50243 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign38310_e50254 * assign38310_e50254))), (-((1e-100 * (((-(-((assign38310_e50226 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign38310_e50252) + (assign38310_e50229 * (0.5 * (((-(-((assign38310_e50235 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign38310_e50249) + (assign38310_e50238 * ((-(-((assign38310_e50243 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign38310_e50254 * assign38310_e50254))), (-((1e-100 * (((-(-((assign38310_e50226 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign38310_e50252) + (assign38310_e50229 * (0.5 * (((-(-((assign38310_e50235 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign38310_e50249) + (assign38310_e50238 * ((-(-((assign38310_e50243 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign38310_e50254 * assign38310_e50254))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38310_e50257;
        var_tmp_dn5 = assign38310_e50257_d_n5;
        var_tmp_dn6 = assign38310_e50257_d_n6;
        var_tmp_dn7 = assign38310_e50257_d_n7;
        var_tmp_dn8 = assign38310_e50257_d_n8;

        let (assign38320_e50306, assign38320_e50306_d_n5, assign38320_e50306_d_n6, assign38320_e50306_d_n7, assign38320_e50306_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard771 == 0.0)) && (var_guard773 == 0.0)) && (var_guard774 == 0.0)) {
        let assign38320_e50276: f64 = (-var_fbbtbot_d);
        let assign38320_e50278: f64 = (assign38320_e50276 / var_fmaxr);
        let assign38320_e50280: f64 = (assign38320_e50278 - 230.25850929940458);
        let assign38320_e50284: f64 = (-var_fbbtbot_d);
        let assign38320_e50286: f64 = (assign38320_e50284 / var_fmaxr);
        let assign38320_e50288: f64 = (assign38320_e50286 - 230.25850929940458);
        let assign38320_e50291: f64 = (-var_fbbtbot_d);
        let assign38320_e50293: f64 = (assign38320_e50291 / var_fmaxr);
        let assign38320_e50295: f64 = (assign38320_e50293 - 230.25850929940458);
        let assign38320_e50297: f64 = (assign38320_e50295 * 0.3333333333333333);
        let assign38320_e50298: f64 = (1.0 + assign38320_e50297);
        let assign38320_e50299: f64 = (assign38320_e50288 * assign38320_e50298);
        let assign38320_e50300: f64 = (0.5 * assign38320_e50299);
        let assign38320_e50301: f64 = (1.0 + assign38320_e50300);
        let assign38320_e50302: f64 = (assign38320_e50280 * assign38320_e50301);
        let assign38320_e50303: f64 = (1.0 + assign38320_e50302);
        let assign38320_e50304: f64 = (1e100 * assign38320_e50303);
        (assign38320_e50304, (1e100 * (((-((assign38320_e50276 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign38320_e50301) + (assign38320_e50280 * (0.5 * (((-((assign38320_e50284 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign38320_e50298) + (assign38320_e50288 * ((-((assign38320_e50291 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign38320_e50276 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign38320_e50301) + (assign38320_e50280 * (0.5 * (((-((assign38320_e50284 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign38320_e50298) + (assign38320_e50288 * ((-((assign38320_e50291 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign38320_e50276 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign38320_e50301) + (assign38320_e50280 * (0.5 * (((-((assign38320_e50284 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign38320_e50298) + (assign38320_e50288 * ((-((assign38320_e50291 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign38320_e50276 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign38320_e50301) + (assign38320_e50280 * (0.5 * (((-((assign38320_e50284 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign38320_e50298) + (assign38320_e50288 * ((-((assign38320_e50291 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38320_e50306;
        var_tmp_dn5 = assign38320_e50306_d_n5;
        var_tmp_dn6 = assign38320_e50306_d_n6;
        var_tmp_dn7 = assign38320_e50306_d_n7;
        var_tmp_dn8 = assign38320_e50306_d_n8;

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
        *var_guard767_slot = var_guard767;
        *var_guard768_slot = var_guard768;
        *var_guard769_slot = var_guard769;
        *var_guard770_slot = var_guard770;
        *var_guard771_slot = var_guard771;
        *var_guard772_slot = var_guard772;
        *var_guard773_slot = var_guard773;
        *var_guard774_slot = var_guard774;
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

    pub(super) fn stamp_transient_block_81(
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
        var_guard176: f64,
        var_guard193: f64,
        var_guard761: f64,
        var_guard771: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
        var_lsdrain_i: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrbotd_i: f64,
        var_pstid_i: f64,
        var_slopebot_d: f64,
        var_two_psistar: f64,
        var_v5: f64,
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
        var_guard775_slot: &mut f64,
        var_guard776_slot: &mut f64,
        var_guard777_slot: &mut f64,
        var_guard778_slot: &mut f64,
        var_guard779_slot: &mut f64,
        var_guard780_slot: &mut f64,
        var_guard781_slot: &mut f64,
        var_guard782_slot: &mut f64,
        var_guard783_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
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
        let mut var_guard775: f64 = *var_guard775_slot;
        let mut var_guard776: f64 = *var_guard776_slot;
        let mut var_guard777: f64 = *var_guard777_slot;
        let mut var_guard778: f64 = *var_guard778_slot;
        let mut var_guard779: f64 = *var_guard779_slot;
        let mut var_guard780: f64 = *var_guard780_slot;
        let mut var_guard781: f64 = *var_guard781_slot;
        let mut var_guard782: f64 = *var_guard782_slot;
        let mut var_guard783: f64 = *var_guard783_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
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

        let (assign38330_e50326, assign38330_e50326_d_n5, assign38330_e50326_d_n6, assign38330_e50326_d_n7, assign38330_e50326_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard771 == 0.0)) {
        let assign38330_e50319: f64 = (var_v5 * var_fmaxr);
        let assign38330_e50321: f64 = (assign38330_e50319 * var_fmaxr);
        let assign38330_e50323: f64 = (assign38330_e50321 * var_tmp);
        let assign38330_e50324: f64 = (var_cbbtbotd_i * assign38330_e50323);
        (assign38330_e50324, (var_cbbtbotd_i * (((((var_v5 * var_fmaxr_dn5) * var_fmaxr) + (assign38330_e50319 * var_fmaxr_dn5)) * var_tmp) + (assign38330_e50321 * var_tmp_dn5))), (var_cbbtbotd_i * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign38330_e50319 * var_fmaxr_dn6)) * var_tmp) + (assign38330_e50321 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign38330_e50319 * var_fmaxr_dn7)) * var_tmp) + (assign38330_e50321 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign38330_e50319 * var_fmaxr_dn8)) * var_tmp) + (assign38330_e50321 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign38330_e50326;
        var_ibbt_dn5 = assign38330_e50326_d_n5;
        var_ibbt_dn6 = assign38330_e50326_d_n6;
        var_ibbt_dn7 = assign38330_e50326_d_n7;
        var_ibbt_dn8 = assign38330_e50326_d_n8;

        let assign38340_e50329: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard775 = assign38340_e50329;

        let (assign38350_e50340, assign38350_e50340_d_n5, assign38350_e50340_d_n6, assign38350_e50340_d_n7, assign38350_e50340_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard775 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign38350_e50340;
        var_fbreakdown_dn5 = assign38350_e50340_d_n5;
        var_fbreakdown_dn6 = assign38350_e50340_d_n6;
        var_fbreakdown_dn7 = assign38350_e50340_d_n7;
        var_fbreakdown_dn8 = assign38350_e50340_d_n8;

        let assign38360_e50343: f64 = (-var_alphaav);
        let assign38360_e50345: f64 = (assign38360_e50343 * var_vbrbotd_i);
        let assign38360_e50346: f64 = if var_vav > assign38360_e50345 { 1.0 } else { 0.0 };
        var_guard776 = assign38360_e50346;

        let assign38370_e50349: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard777 = assign38370_e50349;

        let (assign38380_e50379, assign38380_e50379_d_n5, assign38380_e50379_d_n6, assign38380_e50379_d_n7, assign38380_e50379_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard775 == 0.0)) && (var_guard776 != 0.0)) && (var_guard777 != 0.0)) {
        let assign38380_e50365: f64 = (var_vav * var_vbrinvbot_d);
        let assign38380_e50368: f64 = (var_vav * var_vbrinvbot_d);
        let assign38380_e50369: f64 = (assign38380_e50365 * assign38380_e50368);
        let assign38380_e50372: f64 = (var_vav * var_vbrinvbot_d);
        let assign38380_e50373: f64 = (assign38380_e50369 * assign38380_e50372);
        let assign38380_e50376: f64 = (var_vav * var_vbrinvbot_d);
        let assign38380_e50377: f64 = (assign38380_e50373 * assign38380_e50376);
        (assign38380_e50377, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38380_e50379;
        var_tmp_dn5 = assign38380_e50379_d_n5;
        var_tmp_dn6 = assign38380_e50379_d_n6;
        var_tmp_dn7 = assign38380_e50379_d_n7;
        var_tmp_dn8 = assign38380_e50379_d_n8;

        let (assign38390_e50401, assign38390_e50401_d_n5, assign38390_e50401_d_n6, assign38390_e50401_d_n7, assign38390_e50401_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard775 == 0.0)) && (var_guard776 != 0.0)) && (var_guard777 == 0.0)) {
        let assign38390_e50396: f64 = (var_vav * var_vbrinvbot_d);
        let assign38390_e50397: f64 = (assign38390_e50396).abs();
        let assign38390_e50399: f64 = (assign38390_e50397).powf(var_pbrbotd_i);
        (assign38390_e50399, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38390_e50401;
        var_tmp_dn5 = assign38390_e50401_d_n5;
        var_tmp_dn6 = assign38390_e50401_d_n6;
        var_tmp_dn7 = assign38390_e50401_d_n7;
        var_tmp_dn8 = assign38390_e50401_d_n8;

        let (assign38400_e50419, assign38400_e50419_d_n5, assign38400_e50419_d_n6, assign38400_e50419_d_n7, assign38400_e50419_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard775 == 0.0)) && (var_guard776 != 0.0)) {
        let assign38400_e50416: f64 = (1.0 - var_tmp);
        let assign38400_e50417: f64 = (1.0 / assign38400_e50416);
        (assign38400_e50417, (-((-var_tmp_dn5) / (assign38400_e50416 * assign38400_e50416))), (-((-var_tmp_dn6) / (assign38400_e50416 * assign38400_e50416))), (-((-var_tmp_dn7) / (assign38400_e50416 * assign38400_e50416))), (-((-var_tmp_dn8) / (assign38400_e50416 * assign38400_e50416))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign38400_e50419;
        var_fbreakdown_dn5 = assign38400_e50419_d_n5;
        var_fbreakdown_dn6 = assign38400_e50419_d_n6;
        var_fbreakdown_dn7 = assign38400_e50419_d_n7;
        var_fbreakdown_dn8 = assign38400_e50419_d_n8;

        let (assign38410_e50442, assign38410_e50442_d_n5, assign38410_e50442_d_n6, assign38410_e50442_d_n7, assign38410_e50442_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) && (var_guard775 == 0.0)) && (var_guard776 == 0.0)) {
        let assign38410_e50436: f64 = (var_alphaav * var_vbrbotd_i);
        let assign38410_e50437: f64 = (var_vav + assign38410_e50436);
        let assign38410_e50439: f64 = (assign38410_e50437 * var_slopebot_d);
        let assign38410_e50440: f64 = (var_fstopbot_d + assign38410_e50439);
        (assign38410_e50440, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign38410_e50442;
        var_fbreakdown_dn5 = assign38410_e50442_d_n5;
        var_fbreakdown_dn6 = assign38410_e50442_d_n6;
        var_fbreakdown_dn7 = assign38410_e50442_d_n7;
        var_fbreakdown_dn8 = assign38410_e50442_d_n8;

        let (assign38420_e50461, assign38420_e50461_d_n5, assign38420_e50461_d_n6, assign38420_e50461_d_n7, assign38420_e50461_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard761 == 0.0)) {
        let assign38420_e50452: f64 = (var_id__blk213 + var_isrh);
        let assign38420_e50454: f64 = (assign38420_e50452 + var_itat);
        let assign38420_e50456: f64 = (assign38420_e50454 + var_ibbt);
        let assign38420_e50457: f64 = (p.p29 * assign38420_e50456);
        let assign38420_e50459: f64 = (assign38420_e50457 * var_fbreakdown);
        (assign38420_e50459, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign38420_e50457 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign38420_e50457 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign38420_e50457 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign38420_e50457 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign38420_e50461;
        var_ijunbot_dn5 = assign38420_e50461_d_n5;
        var_ijunbot_dn6 = assign38420_e50461_d_n6;
        var_ijunbot_dn7 = assign38420_e50461_d_n7;
        var_ijunbot_dn8 = assign38420_e50461_d_n8;

        let assign38430_e50464: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard778 = assign38430_e50464;

        let (assign38440_e50472, assign38440_e50472_d_n5, assign38440_e50472_d_n6, assign38440_e50472_d_n7, assign38440_e50472_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign38440_e50472;
        var_ijunsti_dn5 = assign38440_e50472_d_n5;
        var_ijunsti_dn6 = assign38440_e50472_d_n6;
        var_ijunsti_dn7 = assign38440_e50472_d_n7;
        var_ijunsti_dn8 = assign38440_e50472_d_n8;

        let (assign38450_e50483,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) {
        let assign38450_e50481: f64 = (var_idsatsti_d * var_idmult);
        (assign38450_e50481,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign38450_e50483;

        let assign38460_e50490: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard779 = assign38460_e50490;

        let (assign38470_e50501, assign38470_e50501_d_n5, assign38470_e50501_d_n6, assign38470_e50501_d_n7, assign38470_e50501_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign38470_e50501;
        var_isrh_dn5 = assign38470_e50501_d_n5;
        var_isrh_dn6 = assign38470_e50501_d_n6;
        var_isrh_dn7 = assign38470_e50501_d_n7;
        var_isrh_dn8 = assign38470_e50501_d_n8;

        let (assign38480_e50515,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) {
        let assign38480_e50513: f64 = (var_vbisti_d - var_vjsrh);
        (assign38480_e50513,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign38480_e50515;

        let (assign38490_e50534,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) {
        let assign38490_e50529: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign38490_e50530: f64 = (1.0 - assign38490_e50529);
        let assign38490_e50531: f64 = (assign38490_e50530).sqrt();
        let assign38490_e50532: f64 = (1.0 - assign38490_e50531);
        (assign38490_e50532,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign38490_e50534;

        let assign38500_e50537: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard780 = assign38500_e50537;

        let (assign38510_e50551,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) && (var_guard780 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign38510_e50551;

        let (assign38520_e50583,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) && (var_guard780 == 0.0)) {
        let assign38520_e50566: f64 = (var_wsrhstep * var_wsrhstep);
        let assign38520_e50568: f64 = (var_wsrhstep).ln();
        let assign38520_e50569: f64 = (assign38520_e50566 * assign38520_e50568);
        let assign38520_e50572: f64 = (1.0 - var_wsrhstep);
        let assign38520_e50573: f64 = (assign38520_e50569 / assign38520_e50572);
        let assign38520_e50575: f64 = (assign38520_e50573 + var_wsrhstep);
        let assign38520_e50579: f64 = (2.0 * var_pstid_i);
        let assign38520_e50580: f64 = (1.0 - assign38520_e50579);
        let assign38520_e50581: f64 = (assign38520_e50575 * assign38520_e50580);
        (assign38520_e50581,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign38520_e50583;

        let (assign38530_e50597,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) {
        let assign38530_e50595: f64 = (var_wsrhstep + var_dwsrh);
        (assign38530_e50595,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign38530_e50597;

        let assign38540_e50600: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard781 = assign38540_e50600;

        let (assign38550_e50617, assign38550_e50617_d_n5, assign38550_e50617_d_n6, assign38550_e50617_d_n7, assign38550_e50617_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) && (var_guard781 != 0.0)) {
        let assign38550_e50614: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign38550_e50615: f64 = (assign38550_e50614).sqrt();
        (assign38550_e50615, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38550_e50617;
        var_tmp_dn5 = assign38550_e50617_d_n5;
        var_tmp_dn6 = assign38550_e50617_d_n6;
        var_tmp_dn7 = assign38550_e50617_d_n7;
        var_tmp_dn8 = assign38550_e50617_d_n8;

        let (assign38560_e50636, assign38560_e50636_d_n5, assign38560_e50636_d_n6, assign38560_e50636_d_n7, assign38560_e50636_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) && (var_guard781 == 0.0)) {
        let assign38560_e50632: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign38560_e50634: f64 = (assign38560_e50632).powf(var_pstid_i);
        (assign38560_e50634, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38560_e50636;
        var_tmp_dn5 = assign38560_e50636_d_n5;
        var_tmp_dn6 = assign38560_e50636_d_n6;
        var_tmp_dn7 = assign38560_e50636_d_n7;
        var_tmp_dn8 = assign38560_e50636_d_n8;

        let (assign38570_e50650, assign38570_e50650_d_n5, assign38570_e50650_d_n6, assign38570_e50650_d_n7, assign38570_e50650_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) {
        let assign38570_e50648: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign38570_e50648, (var_wdepnulrsti_d * var_tmp_dn5), (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign38570_e50650;
        var_wdep_dn5 = assign38570_e50650_d_n5;
        var_wdep_dn6 = assign38570_e50650_d_n6;
        var_wdep_dn7 = assign38570_e50650_d_n7;
        var_wdep_dn8 = assign38570_e50650_d_n8;

        let (assign38580_e50668, assign38580_e50668_d_n5, assign38580_e50668_d_n6, assign38580_e50668_d_n7, assign38580_e50668_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) {
        let assign38580_e50663: f64 = (var_zinv - 1.0);
        let assign38580_e50665: f64 = (assign38580_e50663 * var_wdep);
        let assign38580_e50666: f64 = (var_ftdsti_d * assign38580_e50665);
        (assign38580_e50666, (var_ftdsti_d * (assign38580_e50663 * var_wdep_dn5)), (var_ftdsti_d * (assign38580_e50663 * var_wdep_dn6)), (var_ftdsti_d * (assign38580_e50663 * var_wdep_dn7)), (var_ftdsti_d * (assign38580_e50663 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign38580_e50668;
        var_asrh_dn5 = assign38580_e50668_d_n5;
        var_asrh_dn6 = assign38580_e50668_d_n6;
        var_asrh_dn7 = assign38580_e50668_d_n7;
        var_asrh_dn8 = assign38580_e50668_d_n8;

        let (assign38590_e50684, assign38590_e50684_d_n5, assign38590_e50684_d_n6, assign38590_e50684_d_n7, assign38590_e50684_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) {
        let assign38590_e50681: f64 = (var_asrh * var_wsrh);
        let assign38590_e50682: f64 = (var_csrhstid_i * assign38590_e50681);
        (assign38590_e50682, (var_csrhstid_i * (var_asrh_dn5 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign38590_e50684;
        var_isrh_dn5 = assign38590_e50684_d_n5;
        var_isrh_dn6 = assign38590_e50684_d_n6;
        var_isrh_dn7 = assign38590_e50684_d_n7;
        var_isrh_dn8 = assign38590_e50684_d_n8;

        let assign38600_e50687: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard782 = assign38600_e50687;

        let (assign38610_e50698, assign38610_e50698_d_n5, assign38610_e50698_d_n6, assign38610_e50698_d_n7, assign38610_e50698_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign38610_e50698;
        var_itat_dn5 = assign38610_e50698_d_n5;
        var_itat_dn6 = assign38610_e50698_d_n6;
        var_itat_dn7 = assign38610_e50698_d_n7;
        var_itat_dn8 = assign38610_e50698_d_n8;

        let (assign38620_e50716, assign38620_e50716_d_n5, assign38620_e50716_d_n6, assign38620_e50716_d_n7, assign38620_e50716_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38620_e50711: f64 = (var_wdep * var_one_minus_psti_d);
        let assign38620_e50713: f64 = (assign38620_e50711 / var_vbi_minus_vjsrh);
        let assign38620_e50714: f64 = (var_btatpartsti_d * assign38620_e50713);
        (assign38620_e50714, (var_btatpartsti_d * ((var_wdep_dn5 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign38620_e50716;
        var_btat_dn5 = assign38620_e50716_d_n5;
        var_btat_dn6 = assign38620_e50716_d_n6;
        var_btat_dn7 = assign38620_e50716_d_n7;
        var_btat_dn8 = assign38620_e50716_d_n8;

        let (assign38630_e50732, assign38630_e50732_d_n5, assign38630_e50732_d_n6, assign38630_e50732_d_n7, assign38630_e50732_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38630_e50728: f64 = (0.666666666666667 * var_atatsti_d);
        let assign38630_e50730: f64 = (assign38630_e50728 / var_btat);
        (assign38630_e50730, (-((assign38630_e50728 * var_btat_dn5) / (var_btat * var_btat))), (-((assign38630_e50728 * var_btat_dn6) / (var_btat * var_btat))), (-((assign38630_e50728 * var_btat_dn7) / (var_btat * var_btat))), (-((assign38630_e50728 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign38630_e50732;
        var_twoatatoverthreebtat_dn5 = assign38630_e50732_d_n5;
        var_twoatatoverthreebtat_dn6 = assign38630_e50732_d_n6;
        var_twoatatoverthreebtat_dn7 = assign38630_e50732_d_n7;
        var_twoatatoverthreebtat_dn8 = assign38630_e50732_d_n8;

        let (assign38640_e50746, assign38640_e50746_d_n5, assign38640_e50746_d_n6, assign38640_e50746_d_n7, assign38640_e50746_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38640_e50744: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign38640_e50744, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign38640_e50746;
        var_umaxbeforelimiting_dn5 = assign38640_e50746_d_n5;
        var_umaxbeforelimiting_dn6 = assign38640_e50746_d_n6;
        var_umaxbeforelimiting_dn7 = assign38640_e50746_d_n7;
        var_umaxbeforelimiting_dn8 = assign38640_e50746_d_n8;

        let (assign38650_e50767, assign38650_e50767_d_n5, assign38650_e50767_d_n6, assign38650_e50767_d_n7, assign38650_e50767_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38650_e50758: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign38650_e50761: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign38650_e50763: f64 = (assign38650_e50761 + 1.0);
        let assign38650_e50764: f64 = (assign38650_e50758 / assign38650_e50763);
        let assign38650_e50765: f64 = (assign38650_e50764).sqrt();
        (assign38650_e50765, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign38650_e50763) - (assign38650_e50758 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign38650_e50763 * assign38650_e50763)) / (2.0 * assign38650_e50765)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign38650_e50763) - (assign38650_e50758 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign38650_e50763 * assign38650_e50763)) / (2.0 * assign38650_e50765)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign38650_e50763) - (assign38650_e50758 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign38650_e50763 * assign38650_e50763)) / (2.0 * assign38650_e50765)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign38650_e50763) - (assign38650_e50758 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign38650_e50763 * assign38650_e50763)) / (2.0 * assign38650_e50765)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign38650_e50767;
        var_umax_dn5 = assign38650_e50767_d_n5;
        var_umax_dn6 = assign38650_e50767_d_n6;
        var_umax_dn7 = assign38650_e50767_d_n7;
        var_umax_dn8 = assign38650_e50767_d_n8;

        let (assign38660_e50780, assign38660_e50780_d_n5, assign38660_e50780_d_n6, assign38660_e50780_d_n7, assign38660_e50780_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38660_e50778: f64 = (var_umax).sqrt();
        (assign38660_e50778, (var_umax_dn5 / (2.0 * assign38660_e50778)), (var_umax_dn6 / (2.0 * assign38660_e50778)), (var_umax_dn7 / (2.0 * assign38660_e50778)), (var_umax_dn8 / (2.0 * assign38660_e50778)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign38660_e50780;
        var_sqrtumax_dn5 = assign38660_e50780_d_n5;
        var_sqrtumax_dn6 = assign38660_e50780_d_n6;
        var_sqrtumax_dn7 = assign38660_e50780_d_n7;
        var_sqrtumax_dn8 = assign38660_e50780_d_n8;

        let (assign38670_e50794, assign38670_e50794_d_n5, assign38670_e50794_d_n6, assign38670_e50794_d_n7, assign38670_e50794_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38670_e50792: f64 = (var_umax * var_sqrtumax);
        (assign38670_e50792, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign38670_e50794;
        var_umaxpoweronepointfive_dn5 = assign38670_e50794_d_n5;
        var_umaxpoweronepointfive_dn6 = assign38670_e50794_d_n6;
        var_umaxpoweronepointfive_dn7 = assign38670_e50794_d_n7;
        var_umaxpoweronepointfive_dn8 = assign38670_e50794_d_n8;

        let assign38680_e50796: f64 = (-var_pstid_i);
        let assign38680_e50798: f64 = (assign38680_e50796 * var_one_over_one_minus_psti_d);
        let assign38680_e50800: f64 = (-1.0);
        let assign38680_e50801: f64 = if assign38680_e50798 == assign38680_e50800 { 1.0 } else { 0.0 };
        var_guard783 = assign38680_e50801;

        let (assign38690_e50821, assign38690_e50821_d_n5, assign38690_e50821_d_n6, assign38690_e50821_d_n7, assign38690_e50821_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) && (var_guard783 != 0.0)) {
        let assign38690_e50817: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign38690_e50818: f64 = (1.0 + assign38690_e50817);
        let assign38690_e50819: f64 = (1.0 / assign38690_e50818);
        (assign38690_e50819, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign38690_e50818 * assign38690_e50818))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign38690_e50818 * assign38690_e50818))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign38690_e50818 * assign38690_e50818))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign38690_e50818 * assign38690_e50818))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign38690_e50821;
        var_wgamma_dn5 = assign38690_e50821_d_n5;
        var_wgamma_dn6 = assign38690_e50821_d_n6;
        var_wgamma_dn7 = assign38690_e50821_d_n7;
        var_wgamma_dn8 = assign38690_e50821_d_n8;

        let (assign38700_e50845, assign38700_e50845_d_n5, assign38700_e50845_d_n6, assign38700_e50845_d_n7, assign38700_e50845_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) && (var_guard783 == 0.0)) {
        let assign38700_e50837: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign38700_e50838: f64 = (1.0 + assign38700_e50837);
        let assign38700_e50840: f64 = (-var_pstid_i);
        let assign38700_e50842: f64 = (assign38700_e50840 * var_one_over_one_minus_psti_d);
        let assign38700_e50843: f64 = (assign38700_e50838).powf(assign38700_e50842);
        (assign38700_e50843, if 0.0 == 0.0 && ((assign38700_e50842) as f64).is_finite() && ((assign38700_e50842) as f64).fract() == 0.0 { if assign38700_e50842 == 0.0 { 0.0 } else { (assign38700_e50842 * ((assign38700_e50838).powf(assign38700_e50842 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign38700_e50843 * (assign38700_e50842 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign38700_e50838))) }, if 0.0 == 0.0 && ((assign38700_e50842) as f64).is_finite() && ((assign38700_e50842) as f64).fract() == 0.0 { if assign38700_e50842 == 0.0 { 0.0 } else { (assign38700_e50842 * ((assign38700_e50838).powf(assign38700_e50842 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign38700_e50843 * (assign38700_e50842 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign38700_e50838))) }, if 0.0 == 0.0 && ((assign38700_e50842) as f64).is_finite() && ((assign38700_e50842) as f64).fract() == 0.0 { if assign38700_e50842 == 0.0 { 0.0 } else { (assign38700_e50842 * ((assign38700_e50838).powf(assign38700_e50842 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign38700_e50843 * (assign38700_e50842 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign38700_e50838))) }, if 0.0 == 0.0 && ((assign38700_e50842) as f64).is_finite() && ((assign38700_e50842) as f64).fract() == 0.0 { if assign38700_e50842 == 0.0 { 0.0 } else { (assign38700_e50842 * ((assign38700_e50838).powf(assign38700_e50842 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign38700_e50843 * (assign38700_e50842 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign38700_e50838))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign38700_e50845;
        var_wgamma_dn5 = assign38700_e50845_d_n5;
        var_wgamma_dn6 = assign38700_e50845_d_n6;
        var_wgamma_dn7 = assign38700_e50845_d_n7;
        var_wgamma_dn8 = assign38700_e50845_d_n8;

        let (assign38710_e50863, assign38710_e50863_d_n5, assign38710_e50863_d_n6, assign38710_e50863_d_n7, assign38710_e50863_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38710_e50857: f64 = (var_wsrh * var_wgamma);
        let assign38710_e50860: f64 = (var_wsrh + var_wgamma);
        let assign38710_e50861: f64 = (assign38710_e50857 / assign38710_e50860);
        (assign38710_e50861, ((((var_wsrh * var_wgamma_dn5) * assign38710_e50860) - (assign38710_e50857 * var_wgamma_dn5)) / (assign38710_e50860 * assign38710_e50860)), ((((var_wsrh * var_wgamma_dn6) * assign38710_e50860) - (assign38710_e50857 * var_wgamma_dn6)) / (assign38710_e50860 * assign38710_e50860)), ((((var_wsrh * var_wgamma_dn7) * assign38710_e50860) - (assign38710_e50857 * var_wgamma_dn7)) / (assign38710_e50860 * assign38710_e50860)), ((((var_wsrh * var_wgamma_dn8) * assign38710_e50860) - (assign38710_e50857 * var_wgamma_dn8)) / (assign38710_e50860 * assign38710_e50860)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign38710_e50863;
        var_wtat_dn5 = assign38710_e50863_d_n5;
        var_wtat_dn6 = assign38710_e50863_d_n6;
        var_wtat_dn7 = assign38710_e50863_d_n7;
        var_wtat_dn8 = assign38710_e50863_d_n8;

        let (assign38720_e50880, assign38720_e50880_d_n5, assign38720_e50880_d_n6, assign38720_e50880_d_n7, assign38720_e50880_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38720_e50876: f64 = (var_btat / var_sqrtumax);
        let assign38720_e50877: f64 = (0.375 * assign38720_e50876);
        let assign38720_e50878: f64 = (assign38720_e50877).sqrt();
        (assign38720_e50878, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38720_e50878)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38720_e50878)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38720_e50878)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38720_e50878)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign38720_e50880;
        var_ktat_dn5 = assign38720_e50880_d_n5;
        var_ktat_dn6 = assign38720_e50880_d_n6;
        var_ktat_dn7 = assign38720_e50880_d_n7;
        var_ktat_dn8 = assign38720_e50880_d_n8;

        let (assign38730_e50898, assign38730_e50898_d_n5, assign38730_e50898_d_n6, assign38730_e50898_d_n7, assign38730_e50898_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38730_e50893: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign38730_e50894: f64 = (2.0 * assign38730_e50893);
        let assign38730_e50896: f64 = (assign38730_e50894 - var_umax);
        (assign38730_e50896, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign38730_e50898;
        var_ltat_dn5 = assign38730_e50898_d_n5;
        var_ltat_dn6 = assign38730_e50898_d_n6;
        var_ltat_dn7 = assign38730_e50898_d_n7;
        var_ltat_dn8 = assign38730_e50898_d_n8;

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
        *var_guard775_slot = var_guard775;
        *var_guard776_slot = var_guard776;
        *var_guard777_slot = var_guard777;
        *var_guard778_slot = var_guard778;
        *var_guard779_slot = var_guard779;
        *var_guard780_slot = var_guard780;
        *var_guard781_slot = var_guard781;
        *var_guard782_slot = var_guard782;
        *var_guard783_slot = var_guard783;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
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

    pub(super) fn stamp_transient_block_82(
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
        var_guard176: f64,
        var_guard193: f64,
        var_guard778: f64,
        var_guard782: f64,
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
        var_v5: f64,
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
        var_guard784_slot: &mut f64,
        var_guard785_slot: &mut f64,
        var_guard786_slot: &mut f64,
        var_guard787_slot: &mut f64,
        var_guard788_slot: &mut f64,
        var_guard789_slot: &mut f64,
        var_guard790_slot: &mut f64,
        var_guard791_slot: &mut f64,
        var_guard792_slot: &mut f64,
        var_guard793_slot: &mut f64,
        var_guard794_slot: &mut f64,
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
        let mut var_guard784: f64 = *var_guard784_slot;
        let mut var_guard785: f64 = *var_guard785_slot;
        let mut var_guard786: f64 = *var_guard786_slot;
        let mut var_guard787: f64 = *var_guard787_slot;
        let mut var_guard788: f64 = *var_guard788_slot;
        let mut var_guard789: f64 = *var_guard789_slot;
        let mut var_guard790: f64 = *var_guard790_slot;
        let mut var_guard791: f64 = *var_guard791_slot;
        let mut var_guard792: f64 = *var_guard792_slot;
        let mut var_guard793: f64 = *var_guard793_slot;
        let mut var_guard794: f64 = *var_guard794_slot;
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

        let (assign38740_e50924, assign38740_e50924_d_n5, assign38740_e50924_d_n6, assign38740_e50924_d_n7, assign38740_e50924_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38740_e50910: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign38740_e50912: f64 = (assign38740_e50910 * var_sqrtumax);
        let assign38740_e50915: f64 = (var_atatsti_d * var_umax);
        let assign38740_e50916: f64 = (assign38740_e50912 - assign38740_e50915);
        let assign38740_e50920: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign38740_e50921: f64 = (0.5 * assign38740_e50920);
        let assign38740_e50922: f64 = (assign38740_e50916 + assign38740_e50921);
        (assign38740_e50922, (((((var_atatsti_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign38740_e50910 * var_sqrtumax_dn5)) - (var_atatsti_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign38740_e50910 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign38740_e50910 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign38740_e50910 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign38740_e50924;
        var_mtat_dn5 = assign38740_e50924_d_n5;
        var_mtat_dn6 = assign38740_e50924_d_n6;
        var_mtat_dn7 = assign38740_e50924_d_n7;
        var_mtat_dn8 = assign38740_e50924_d_n8;

        let (assign38750_e50940, assign38750_e50940_d_n5, assign38750_e50940_d_n6, assign38750_e50940_d_n7, assign38750_e50940_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38750_e50936: f64 = (var_ltat - 1.0);
        let assign38750_e50938: f64 = (assign38750_e50936 * var_ktat);
        (assign38750_e50938, ((var_ltat_dn5 * var_ktat) + (assign38750_e50936 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign38750_e50936 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign38750_e50936 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign38750_e50936 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign38750_e50940;
        var_xerfc_dn5 = assign38750_e50940_d_n5;
        var_xerfc_dn6 = assign38750_e50940_d_n6;
        var_xerfc_dn7 = assign38750_e50940_d_n7;
        var_xerfc_dn8 = assign38750_e50940_d_n8;

        let (assign38760_e50954, assign38760_e50954_d_n5, assign38760_e50954_d_n6, assign38760_e50954_d_n7, assign38760_e50954_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38760_e50952: f64 = (var_xerfc * var_xerfc);
        (assign38760_e50952, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign38760_e50954;
        var_ysq_dn5 = assign38760_e50954_d_n5;
        var_ysq_dn6 = assign38760_e50954_d_n6;
        var_ysq_dn7 = assign38760_e50954_d_n7;
        var_ysq_dn8 = assign38760_e50954_d_n8;

        let assign38770_e50957: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard784 = assign38770_e50957;

        let (assign38780_e50977, assign38780_e50977_d_n5, assign38780_e50977_d_n6, assign38780_e50977_d_n7, assign38780_e50977_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) && (var_guard784 != 0.0)) {
        let assign38780_e50973: f64 = (var_perfc * var_xerfc);
        let assign38780_e50974: f64 = (1.0 + assign38780_e50973);
        let assign38780_e50975: f64 = (1.0 / assign38780_e50974);
        (assign38780_e50975, (-((var_perfc * var_xerfc_dn5) / (assign38780_e50974 * assign38780_e50974))), (-((var_perfc * var_xerfc_dn6) / (assign38780_e50974 * assign38780_e50974))), (-((var_perfc * var_xerfc_dn7) / (assign38780_e50974 * assign38780_e50974))), (-((var_perfc * var_xerfc_dn8) / (assign38780_e50974 * assign38780_e50974))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign38780_e50977;
        var_terfc_dn5 = assign38780_e50977_d_n5;
        var_terfc_dn6 = assign38780_e50977_d_n6;
        var_terfc_dn7 = assign38780_e50977_d_n7;
        var_terfc_dn8 = assign38780_e50977_d_n8;

        let (assign38790_e50998, assign38790_e50998_d_n5, assign38790_e50998_d_n6, assign38790_e50998_d_n7, assign38790_e50998_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) && (var_guard784 == 0.0)) {
        let assign38790_e50994: f64 = (var_perfc * var_xerfc);
        let assign38790_e50995: f64 = (1.0 - assign38790_e50994);
        let assign38790_e50996: f64 = (1.0 / assign38790_e50995);
        (assign38790_e50996, (-((-(var_perfc * var_xerfc_dn5)) / (assign38790_e50995 * assign38790_e50995))), (-((-(var_perfc * var_xerfc_dn6)) / (assign38790_e50995 * assign38790_e50995))), (-((-(var_perfc * var_xerfc_dn7)) / (assign38790_e50995 * assign38790_e50995))), (-((-(var_perfc * var_xerfc_dn8)) / (assign38790_e50995 * assign38790_e50995))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign38790_e50998;
        var_terfc_dn5 = assign38790_e50998_d_n5;
        var_terfc_dn6 = assign38790_e50998_d_n6;
        var_terfc_dn7 = assign38790_e50998_d_n7;
        var_terfc_dn8 = assign38790_e50998_d_n8;

        let assign38800_e51000: f64 = (-var_ysq);
        let assign38800_e51002: f64 = (assign38800_e51000 + var_mtat);
        let assign38800_e51004: f64 = (-230.25850929940458);
        let assign38800_e51005: f64 = if assign38800_e51002 > assign38800_e51004 { 1.0 } else { 0.0 };
        var_guard785 = assign38800_e51005;

        let (assign38810_e51023, assign38810_e51023_d_n5, assign38810_e51023_d_n6, assign38810_e51023_d_n7, assign38810_e51023_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) && (var_guard785 != 0.0)) {
        let assign38810_e51018: f64 = (-var_ysq);
        let assign38810_e51020: f64 = (assign38810_e51018 + var_mtat);
        let assign38810_e51021: f64 = (assign38810_e51020).exp();
        (assign38810_e51021, (assign38810_e51021 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign38810_e51021 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign38810_e51021 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign38810_e51021 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38810_e51023;
        var_tmp_dn5 = assign38810_e51023_d_n5;
        var_tmp_dn6 = assign38810_e51023_d_n6;
        var_tmp_dn7 = assign38810_e51023_d_n7;
        var_tmp_dn8 = assign38810_e51023_d_n8;

        let (assign38820_e51072, assign38820_e51072_d_n5, assign38820_e51072_d_n6, assign38820_e51072_d_n7, assign38820_e51072_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) && (var_guard785 == 0.0)) {
        let assign38820_e51039: f64 = (-230.25850929940458);
        let assign38820_e51041: f64 = (-var_ysq);
        let assign38820_e51043: f64 = (assign38820_e51041 + var_mtat);
        let assign38820_e51044: f64 = (assign38820_e51039 - assign38820_e51043);
        let assign38820_e51048: f64 = (-230.25850929940458);
        let assign38820_e51050: f64 = (-var_ysq);
        let assign38820_e51052: f64 = (assign38820_e51050 + var_mtat);
        let assign38820_e51053: f64 = (assign38820_e51048 - assign38820_e51052);
        let assign38820_e51056: f64 = (-230.25850929940458);
        let assign38820_e51058: f64 = (-var_ysq);
        let assign38820_e51060: f64 = (assign38820_e51058 + var_mtat);
        let assign38820_e51061: f64 = (assign38820_e51056 - assign38820_e51060);
        let assign38820_e51063: f64 = (assign38820_e51061 * 0.3333333333333333);
        let assign38820_e51064: f64 = (1.0 + assign38820_e51063);
        let assign38820_e51065: f64 = (assign38820_e51053 * assign38820_e51064);
        let assign38820_e51066: f64 = (0.5 * assign38820_e51065);
        let assign38820_e51067: f64 = (1.0 + assign38820_e51066);
        let assign38820_e51068: f64 = (assign38820_e51044 * assign38820_e51067);
        let assign38820_e51069: f64 = (1.0 + assign38820_e51068);
        let assign38820_e51070: f64 = (1e-100 / assign38820_e51069);
        (assign38820_e51070, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign38820_e51067) + (assign38820_e51044 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign38820_e51064) + (assign38820_e51053 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign38820_e51069 * assign38820_e51069))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign38820_e51067) + (assign38820_e51044 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign38820_e51064) + (assign38820_e51053 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign38820_e51069 * assign38820_e51069))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign38820_e51067) + (assign38820_e51044 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign38820_e51064) + (assign38820_e51053 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign38820_e51069 * assign38820_e51069))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign38820_e51067) + (assign38820_e51044 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign38820_e51064) + (assign38820_e51053 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign38820_e51069 * assign38820_e51069))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38820_e51072;
        var_tmp_dn5 = assign38820_e51072_d_n5;
        var_tmp_dn6 = assign38820_e51072_d_n6;
        var_tmp_dn7 = assign38820_e51072_d_n7;
        var_tmp_dn8 = assign38820_e51072_d_n8;

        let (assign38830_e51102, assign38830_e51102_d_n5, assign38830_e51102_d_n6, assign38830_e51102_d_n7, assign38830_e51102_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38830_e51084: f64 = (0.29214664 * var_terfc);
        let assign38830_e51088: f64 = (var_terfc * var_terfc);
        let assign38830_e51089: f64 = (var_berfc * assign38830_e51088);
        let assign38830_e51090: f64 = (assign38830_e51084 + assign38830_e51089);
        let assign38830_e51094: f64 = (var_terfc * var_terfc);
        let assign38830_e51096: f64 = (assign38830_e51094 * var_terfc);
        let assign38830_e51097: f64 = (var_cerfc * assign38830_e51096);
        let assign38830_e51098: f64 = (assign38830_e51090 + assign38830_e51097);
        let assign38830_e51100: f64 = (assign38830_e51098 * var_tmp);
        (assign38830_e51100, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign38830_e51094 * var_terfc_dn5)))) * var_tmp) + (assign38830_e51098 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign38830_e51094 * var_terfc_dn6)))) * var_tmp) + (assign38830_e51098 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign38830_e51094 * var_terfc_dn7)))) * var_tmp) + (assign38830_e51098 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign38830_e51094 * var_terfc_dn8)))) * var_tmp) + (assign38830_e51098 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign38830_e51102;
        var_erfcpos_dn5 = assign38830_e51102_d_n5;
        var_erfcpos_dn6 = assign38830_e51102_d_n6;
        var_erfcpos_dn7 = assign38830_e51102_d_n7;
        var_erfcpos_dn8 = assign38830_e51102_d_n8;

        let assign38840_e51105: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard786 = assign38840_e51105;

        let (assign38850_e51119, assign38850_e51119_d_n5, assign38850_e51119_d_n6, assign38850_e51119_d_n7, assign38850_e51119_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) && (var_guard786 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign38850_e51119;
        var_erfctimesexpmtat_dn5 = assign38850_e51119_d_n5;
        var_erfctimesexpmtat_dn6 = assign38850_e51119_d_n6;
        var_erfctimesexpmtat_dn7 = assign38850_e51119_d_n7;
        var_erfctimesexpmtat_dn8 = assign38850_e51119_d_n8;

        let assign38860_e51122: f64 = (-230.25850929940458);
        let assign38860_e51123: f64 = if var_mtat > assign38860_e51122 { 1.0 } else { 0.0 };
        var_guard787 = assign38860_e51123;

        let (assign38870_e51141, assign38870_e51141_d_n5, assign38870_e51141_d_n6, assign38870_e51141_d_n7, assign38870_e51141_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) && (var_guard786 == 0.0)) && (var_guard787 != 0.0)) {
        let assign38870_e51139: f64 = (var_mtat).exp();
        (assign38870_e51139, (assign38870_e51139 * var_mtat_dn5), (assign38870_e51139 * var_mtat_dn6), (assign38870_e51139 * var_mtat_dn7), (assign38870_e51139 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38870_e51141;
        var_tmp_dn5 = assign38870_e51141_d_n5;
        var_tmp_dn6 = assign38870_e51141_d_n6;
        var_tmp_dn7 = assign38870_e51141_d_n7;
        var_tmp_dn8 = assign38870_e51141_d_n8;

        let (assign38880_e51184, assign38880_e51184_d_n5, assign38880_e51184_d_n6, assign38880_e51184_d_n7, assign38880_e51184_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) && (var_guard786 == 0.0)) && (var_guard787 == 0.0)) {
        let assign38880_e51160: f64 = (-230.25850929940458);
        let assign38880_e51162: f64 = (assign38880_e51160 - var_mtat);
        let assign38880_e51166: f64 = (-230.25850929940458);
        let assign38880_e51168: f64 = (assign38880_e51166 - var_mtat);
        let assign38880_e51171: f64 = (-230.25850929940458);
        let assign38880_e51173: f64 = (assign38880_e51171 - var_mtat);
        let assign38880_e51175: f64 = (assign38880_e51173 * 0.3333333333333333);
        let assign38880_e51176: f64 = (1.0 + assign38880_e51175);
        let assign38880_e51177: f64 = (assign38880_e51168 * assign38880_e51176);
        let assign38880_e51178: f64 = (0.5 * assign38880_e51177);
        let assign38880_e51179: f64 = (1.0 + assign38880_e51178);
        let assign38880_e51180: f64 = (assign38880_e51162 * assign38880_e51179);
        let assign38880_e51181: f64 = (1.0 + assign38880_e51180);
        let assign38880_e51182: f64 = (1e-100 / assign38880_e51181);
        (assign38880_e51182, (-((1e-100 * (((-var_mtat_dn5) * assign38880_e51179) + (assign38880_e51162 * (0.5 * (((-var_mtat_dn5) * assign38880_e51176) + (assign38880_e51168 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign38880_e51181 * assign38880_e51181))), (-((1e-100 * (((-var_mtat_dn6) * assign38880_e51179) + (assign38880_e51162 * (0.5 * (((-var_mtat_dn6) * assign38880_e51176) + (assign38880_e51168 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign38880_e51181 * assign38880_e51181))), (-((1e-100 * (((-var_mtat_dn7) * assign38880_e51179) + (assign38880_e51162 * (0.5 * (((-var_mtat_dn7) * assign38880_e51176) + (assign38880_e51168 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign38880_e51181 * assign38880_e51181))), (-((1e-100 * (((-var_mtat_dn8) * assign38880_e51179) + (assign38880_e51162 * (0.5 * (((-var_mtat_dn8) * assign38880_e51176) + (assign38880_e51168 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign38880_e51181 * assign38880_e51181))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38880_e51184;
        var_tmp_dn5 = assign38880_e51184_d_n5;
        var_tmp_dn6 = assign38880_e51184_d_n6;
        var_tmp_dn7 = assign38880_e51184_d_n7;
        var_tmp_dn8 = assign38880_e51184_d_n8;

        let (assign38890_e51203, assign38890_e51203_d_n5, assign38890_e51203_d_n6, assign38890_e51203_d_n7, assign38890_e51203_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) && (var_guard786 == 0.0)) {
        let assign38890_e51199: f64 = (2.0 * var_tmp);
        let assign38890_e51201: f64 = (assign38890_e51199 - var_erfcpos);
        (assign38890_e51201, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign38890_e51203;
        var_erfctimesexpmtat_dn5 = assign38890_e51203_d_n5;
        var_erfctimesexpmtat_dn6 = assign38890_e51203_d_n6;
        var_erfctimesexpmtat_dn7 = assign38890_e51203_d_n7;
        var_erfctimesexpmtat_dn8 = assign38890_e51203_d_n8;

        let (assign38900_e51223, assign38900_e51223_d_n5, assign38900_e51223_d_n6, assign38900_e51223_d_n7, assign38900_e51223_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38900_e51215: f64 = (1.772453850905516 * 0.5);
        let assign38900_e51218: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign38900_e51220: f64 = (assign38900_e51218 / var_ktat);
        let assign38900_e51221: f64 = (assign38900_e51215 * assign38900_e51220);
        (assign38900_e51221, (assign38900_e51215 * ((((var_atatsti_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign38900_e51218 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign38900_e51215 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign38900_e51218 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign38900_e51215 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign38900_e51218 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign38900_e51215 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign38900_e51218 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign38900_e51223;
        var_gammamax_dn5 = assign38900_e51223_d_n5;
        var_gammamax_dn6 = assign38900_e51223_d_n6;
        var_gammamax_dn7 = assign38900_e51223_d_n7;
        var_gammamax_dn8 = assign38900_e51223_d_n8;

        let (assign38910_e51241, assign38910_e51241_d_n5, assign38910_e51241_d_n6, assign38910_e51241_d_n7, assign38910_e51241_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard782 == 0.0)) {
        let assign38910_e51236: f64 = (var_asrh * var_gammamax);
        let assign38910_e51238: f64 = (assign38910_e51236 * var_wtat);
        let assign38910_e51239: f64 = (var_ctatstid_i * assign38910_e51238);
        (assign38910_e51239, (var_ctatstid_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign38910_e51236 * var_wtat_dn5))), (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign38910_e51236 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign38910_e51236 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign38910_e51236 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign38910_e51241;
        var_itat_dn5 = assign38910_e51241_d_n5;
        var_itat_dn6 = assign38910_e51241_d_n6;
        var_itat_dn7 = assign38910_e51241_d_n7;
        var_itat_dn8 = assign38910_e51241_d_n8;

        let assign38920_e51244: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard788 = assign38920_e51244;

        let (assign38930_e51255, assign38930_e51255_d_n5, assign38930_e51255_d_n6, assign38930_e51255_d_n7, assign38930_e51255_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard788 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign38930_e51255;
        var_ibbt_dn5 = assign38930_e51255_d_n5;
        var_ibbt_dn6 = assign38930_e51255_d_n6;
        var_ibbt_dn7 = assign38930_e51255_d_n7;
        var_ibbt_dn8 = assign38930_e51255_d_n8;

        let assign38940_e51258: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard789 = assign38940_e51258;

        let (assign38950_e51277, assign38950_e51277_d_n5, assign38950_e51277_d_n6, assign38950_e51277_d_n7, assign38950_e51277_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard788 == 0.0)) && (var_guard789 != 0.0)) {
        let assign38950_e51272: f64 = (var_vbirstid_i - var_vbbt);
        let assign38950_e51274: f64 = (assign38950_e51272 * var_vbirstiinv_d);
        let assign38950_e51275: f64 = (assign38950_e51274).sqrt();
        (assign38950_e51275, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38950_e51277;
        var_tmp_dn5 = assign38950_e51277_d_n5;
        var_tmp_dn6 = assign38950_e51277_d_n6;
        var_tmp_dn7 = assign38950_e51277_d_n7;
        var_tmp_dn8 = assign38950_e51277_d_n8;

        let (assign38960_e51298, assign38960_e51298_d_n5, assign38960_e51298_d_n6, assign38960_e51298_d_n7, assign38960_e51298_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard788 == 0.0)) && (var_guard789 == 0.0)) {
        let assign38960_e51292: f64 = (var_vbirstid_i - var_vbbt);
        let assign38960_e51294: f64 = (assign38960_e51292 * var_vbirstiinv_d);
        let assign38960_e51296: f64 = (assign38960_e51294).powf(var_pstid_i);
        (assign38960_e51296, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38960_e51298;
        var_tmp_dn5 = assign38960_e51298_d_n5;
        var_tmp_dn6 = assign38960_e51298_d_n6;
        var_tmp_dn7 = assign38960_e51298_d_n7;
        var_tmp_dn8 = assign38960_e51298_d_n8;

        let (assign38970_e51318, assign38970_e51318_d_n5, assign38970_e51318_d_n6, assign38970_e51318_d_n7, assign38970_e51318_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard788 == 0.0)) {
        let assign38970_e51311: f64 = (var_vbirstid_i - var_vbbt);
        let assign38970_e51313: f64 = (assign38970_e51311 * var_wdepnulrinvsti_d);
        let assign38970_e51315: f64 = (assign38970_e51313 / var_tmp);
        let assign38970_e51316: f64 = (var_one_over_one_minus_psti_d * assign38970_e51315);
        (assign38970_e51316, (var_one_over_one_minus_psti_d * (-((assign38970_e51313 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign38970_e51313 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign38970_e51313 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign38970_e51313 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign38970_e51318;
        var_fmaxr_dn5 = assign38970_e51318_d_n5;
        var_fmaxr_dn6 = assign38970_e51318_d_n6;
        var_fmaxr_dn7 = assign38970_e51318_d_n7;
        var_fmaxr_dn8 = assign38970_e51318_d_n8;

        let assign38980_e51320: f64 = (-var_fbbtsti_d);
        let assign38980_e51322: f64 = (assign38980_e51320 / var_fmaxr);
        let assign38980_e51323: f64 = (assign38980_e51322).abs();
        let assign38980_e51325: f64 = if assign38980_e51323 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard790 = assign38980_e51325;

        let (assign38990_e51343, assign38990_e51343_d_n5, assign38990_e51343_d_n6, assign38990_e51343_d_n7, assign38990_e51343_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard788 == 0.0)) && (var_guard790 != 0.0)) {
        let assign38990_e51338: f64 = (-var_fbbtsti_d);
        let assign38990_e51340: f64 = (assign38990_e51338 / var_fmaxr);
        let assign38990_e51341: f64 = (assign38990_e51340).exp();
        (assign38990_e51341, (assign38990_e51341 * (-((assign38990_e51338 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign38990_e51341 * (-((assign38990_e51338 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign38990_e51341 * (-((assign38990_e51338 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign38990_e51341 * (-((assign38990_e51338 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign38990_e51343;
        var_tmp_dn5 = assign38990_e51343_d_n5;
        var_tmp_dn6 = assign38990_e51343_d_n6;
        var_tmp_dn7 = assign38990_e51343_d_n7;
        var_tmp_dn8 = assign38990_e51343_d_n8;

        let assign39000_e51345: f64 = (-var_fbbtsti_d);
        let assign39000_e51347: f64 = (assign39000_e51345 / var_fmaxr);
        let assign39000_e51349: f64 = if assign39000_e51347 < 0.0 { 1.0 } else { 0.0 };
        var_guard791 = assign39000_e51349;

        let (assign39010_e51400, assign39010_e51400_d_n5, assign39010_e51400_d_n6, assign39010_e51400_d_n7, assign39010_e51400_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard788 == 0.0)) && (var_guard790 == 0.0)) && (var_guard791 != 0.0)) {
        let assign39010_e51367: f64 = (-230.25850929940458);
        let assign39010_e51369: f64 = (-var_fbbtsti_d);
        let assign39010_e51371: f64 = (assign39010_e51369 / var_fmaxr);
        let assign39010_e51372: f64 = (assign39010_e51367 - assign39010_e51371);
        let assign39010_e51376: f64 = (-230.25850929940458);
        let assign39010_e51378: f64 = (-var_fbbtsti_d);
        let assign39010_e51380: f64 = (assign39010_e51378 / var_fmaxr);
        let assign39010_e51381: f64 = (assign39010_e51376 - assign39010_e51380);
        let assign39010_e51384: f64 = (-230.25850929940458);
        let assign39010_e51386: f64 = (-var_fbbtsti_d);
        let assign39010_e51388: f64 = (assign39010_e51386 / var_fmaxr);
        let assign39010_e51389: f64 = (assign39010_e51384 - assign39010_e51388);
        let assign39010_e51391: f64 = (assign39010_e51389 * 0.3333333333333333);
        let assign39010_e51392: f64 = (1.0 + assign39010_e51391);
        let assign39010_e51393: f64 = (assign39010_e51381 * assign39010_e51392);
        let assign39010_e51394: f64 = (0.5 * assign39010_e51393);
        let assign39010_e51395: f64 = (1.0 + assign39010_e51394);
        let assign39010_e51396: f64 = (assign39010_e51372 * assign39010_e51395);
        let assign39010_e51397: f64 = (1.0 + assign39010_e51396);
        let assign39010_e51398: f64 = (1e-100 / assign39010_e51397);
        (assign39010_e51398, (-((1e-100 * (((-(-((assign39010_e51369 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign39010_e51395) + (assign39010_e51372 * (0.5 * (((-(-((assign39010_e51378 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign39010_e51392) + (assign39010_e51381 * ((-(-((assign39010_e51386 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign39010_e51397 * assign39010_e51397))), (-((1e-100 * (((-(-((assign39010_e51369 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign39010_e51395) + (assign39010_e51372 * (0.5 * (((-(-((assign39010_e51378 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign39010_e51392) + (assign39010_e51381 * ((-(-((assign39010_e51386 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign39010_e51397 * assign39010_e51397))), (-((1e-100 * (((-(-((assign39010_e51369 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign39010_e51395) + (assign39010_e51372 * (0.5 * (((-(-((assign39010_e51378 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign39010_e51392) + (assign39010_e51381 * ((-(-((assign39010_e51386 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign39010_e51397 * assign39010_e51397))), (-((1e-100 * (((-(-((assign39010_e51369 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign39010_e51395) + (assign39010_e51372 * (0.5 * (((-(-((assign39010_e51378 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign39010_e51392) + (assign39010_e51381 * ((-(-((assign39010_e51386 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign39010_e51397 * assign39010_e51397))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39010_e51400;
        var_tmp_dn5 = assign39010_e51400_d_n5;
        var_tmp_dn6 = assign39010_e51400_d_n6;
        var_tmp_dn7 = assign39010_e51400_d_n7;
        var_tmp_dn8 = assign39010_e51400_d_n8;

        let (assign39020_e51449, assign39020_e51449_d_n5, assign39020_e51449_d_n6, assign39020_e51449_d_n7, assign39020_e51449_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard788 == 0.0)) && (var_guard790 == 0.0)) && (var_guard791 == 0.0)) {
        let assign39020_e51419: f64 = (-var_fbbtsti_d);
        let assign39020_e51421: f64 = (assign39020_e51419 / var_fmaxr);
        let assign39020_e51423: f64 = (assign39020_e51421 - 230.25850929940458);
        let assign39020_e51427: f64 = (-var_fbbtsti_d);
        let assign39020_e51429: f64 = (assign39020_e51427 / var_fmaxr);
        let assign39020_e51431: f64 = (assign39020_e51429 - 230.25850929940458);
        let assign39020_e51434: f64 = (-var_fbbtsti_d);
        let assign39020_e51436: f64 = (assign39020_e51434 / var_fmaxr);
        let assign39020_e51438: f64 = (assign39020_e51436 - 230.25850929940458);
        let assign39020_e51440: f64 = (assign39020_e51438 * 0.3333333333333333);
        let assign39020_e51441: f64 = (1.0 + assign39020_e51440);
        let assign39020_e51442: f64 = (assign39020_e51431 * assign39020_e51441);
        let assign39020_e51443: f64 = (0.5 * assign39020_e51442);
        let assign39020_e51444: f64 = (1.0 + assign39020_e51443);
        let assign39020_e51445: f64 = (assign39020_e51423 * assign39020_e51444);
        let assign39020_e51446: f64 = (1.0 + assign39020_e51445);
        let assign39020_e51447: f64 = (1e100 * assign39020_e51446);
        (assign39020_e51447, (1e100 * (((-((assign39020_e51419 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign39020_e51444) + (assign39020_e51423 * (0.5 * (((-((assign39020_e51427 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign39020_e51441) + (assign39020_e51431 * ((-((assign39020_e51434 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign39020_e51419 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign39020_e51444) + (assign39020_e51423 * (0.5 * (((-((assign39020_e51427 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign39020_e51441) + (assign39020_e51431 * ((-((assign39020_e51434 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign39020_e51419 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign39020_e51444) + (assign39020_e51423 * (0.5 * (((-((assign39020_e51427 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign39020_e51441) + (assign39020_e51431 * ((-((assign39020_e51434 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign39020_e51419 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign39020_e51444) + (assign39020_e51423 * (0.5 * (((-((assign39020_e51427 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign39020_e51441) + (assign39020_e51431 * ((-((assign39020_e51434 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39020_e51449;
        var_tmp_dn5 = assign39020_e51449_d_n5;
        var_tmp_dn6 = assign39020_e51449_d_n6;
        var_tmp_dn7 = assign39020_e51449_d_n7;
        var_tmp_dn8 = assign39020_e51449_d_n8;

        let (assign39030_e51469, assign39030_e51469_d_n5, assign39030_e51469_d_n6, assign39030_e51469_d_n7, assign39030_e51469_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard788 == 0.0)) {
        let assign39030_e51462: f64 = (var_v5 * var_fmaxr);
        let assign39030_e51464: f64 = (assign39030_e51462 * var_fmaxr);
        let assign39030_e51466: f64 = (assign39030_e51464 * var_tmp);
        let assign39030_e51467: f64 = (var_cbbtstid_i * assign39030_e51466);
        (assign39030_e51467, (var_cbbtstid_i * (((((var_v5 * var_fmaxr_dn5) * var_fmaxr) + (assign39030_e51462 * var_fmaxr_dn5)) * var_tmp) + (assign39030_e51464 * var_tmp_dn5))), (var_cbbtstid_i * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign39030_e51462 * var_fmaxr_dn6)) * var_tmp) + (assign39030_e51464 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign39030_e51462 * var_fmaxr_dn7)) * var_tmp) + (assign39030_e51464 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign39030_e51462 * var_fmaxr_dn8)) * var_tmp) + (assign39030_e51464 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign39030_e51469;
        var_ibbt_dn5 = assign39030_e51469_d_n5;
        var_ibbt_dn6 = assign39030_e51469_d_n6;
        var_ibbt_dn7 = assign39030_e51469_d_n7;
        var_ibbt_dn8 = assign39030_e51469_d_n8;

        let assign39040_e51472: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard792 = assign39040_e51472;

        let (assign39050_e51483, assign39050_e51483_d_n5, assign39050_e51483_d_n6, assign39050_e51483_d_n7, assign39050_e51483_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard792 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign39050_e51483;
        var_fbreakdown_dn5 = assign39050_e51483_d_n5;
        var_fbreakdown_dn6 = assign39050_e51483_d_n6;
        var_fbreakdown_dn7 = assign39050_e51483_d_n7;
        var_fbreakdown_dn8 = assign39050_e51483_d_n8;

        let assign39060_e51486: f64 = (-var_alphaav);
        let assign39060_e51488: f64 = (assign39060_e51486 * var_vbrstid_i);
        let assign39060_e51489: f64 = if var_vav > assign39060_e51488 { 1.0 } else { 0.0 };
        var_guard793 = assign39060_e51489;

        let assign39070_e51492: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard794 = assign39070_e51492;

        let (assign39080_e51522, assign39080_e51522_d_n5, assign39080_e51522_d_n6, assign39080_e51522_d_n7, assign39080_e51522_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard792 == 0.0)) && (var_guard793 != 0.0)) && (var_guard794 != 0.0)) {
        let assign39080_e51508: f64 = (var_vav * var_vbrinvsti_d);
        let assign39080_e51511: f64 = (var_vav * var_vbrinvsti_d);
        let assign39080_e51512: f64 = (assign39080_e51508 * assign39080_e51511);
        let assign39080_e51515: f64 = (var_vav * var_vbrinvsti_d);
        let assign39080_e51516: f64 = (assign39080_e51512 * assign39080_e51515);
        let assign39080_e51519: f64 = (var_vav * var_vbrinvsti_d);
        let assign39080_e51520: f64 = (assign39080_e51516 * assign39080_e51519);
        (assign39080_e51520, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39080_e51522;
        var_tmp_dn5 = assign39080_e51522_d_n5;
        var_tmp_dn6 = assign39080_e51522_d_n6;
        var_tmp_dn7 = assign39080_e51522_d_n7;
        var_tmp_dn8 = assign39080_e51522_d_n8;

        let (assign39090_e51544, assign39090_e51544_d_n5, assign39090_e51544_d_n6, assign39090_e51544_d_n7, assign39090_e51544_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard792 == 0.0)) && (var_guard793 != 0.0)) && (var_guard794 == 0.0)) {
        let assign39090_e51539: f64 = (var_vav * var_vbrinvsti_d);
        let assign39090_e51540: f64 = (assign39090_e51539).abs();
        let assign39090_e51542: f64 = (assign39090_e51540).powf(var_pbrstid_i);
        (assign39090_e51542, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39090_e51544;
        var_tmp_dn5 = assign39090_e51544_d_n5;
        var_tmp_dn6 = assign39090_e51544_d_n6;
        var_tmp_dn7 = assign39090_e51544_d_n7;
        var_tmp_dn8 = assign39090_e51544_d_n8;

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
        *var_guard784_slot = var_guard784;
        *var_guard785_slot = var_guard785;
        *var_guard786_slot = var_guard786;
        *var_guard787_slot = var_guard787;
        *var_guard788_slot = var_guard788;
        *var_guard789_slot = var_guard789;
        *var_guard790_slot = var_guard790;
        *var_guard791_slot = var_guard791;
        *var_guard792_slot = var_guard792;
        *var_guard793_slot = var_guard793;
        *var_guard794_slot = var_guard794;
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

    pub(super) fn stamp_transient_block_83(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat_d: f64,
        var_btatpartgat_d: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_fstopsti_d: f64,
        var_ftdgat_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard778: f64,
        var_guard792: f64,
        var_guard793: f64,
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
        var_guard795_slot: &mut f64,
        var_guard796_slot: &mut f64,
        var_guard797_slot: &mut f64,
        var_guard798_slot: &mut f64,
        var_guard799_slot: &mut f64,
        var_guard800_slot: &mut f64,
        var_guard801_slot: &mut f64,
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
        let mut var_guard795: f64 = *var_guard795_slot;
        let mut var_guard796: f64 = *var_guard796_slot;
        let mut var_guard797: f64 = *var_guard797_slot;
        let mut var_guard798: f64 = *var_guard798_slot;
        let mut var_guard799: f64 = *var_guard799_slot;
        let mut var_guard800: f64 = *var_guard800_slot;
        let mut var_guard801: f64 = *var_guard801_slot;
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

        let (assign39100_e51562, assign39100_e51562_d_n5, assign39100_e51562_d_n6, assign39100_e51562_d_n7, assign39100_e51562_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard792 == 0.0)) && (var_guard793 != 0.0)) {
        let assign39100_e51559: f64 = (1.0 - var_tmp);
        let assign39100_e51560: f64 = (1.0 / assign39100_e51559);
        (assign39100_e51560, (-((-var_tmp_dn5) / (assign39100_e51559 * assign39100_e51559))), (-((-var_tmp_dn6) / (assign39100_e51559 * assign39100_e51559))), (-((-var_tmp_dn7) / (assign39100_e51559 * assign39100_e51559))), (-((-var_tmp_dn8) / (assign39100_e51559 * assign39100_e51559))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign39100_e51562;
        var_fbreakdown_dn5 = assign39100_e51562_d_n5;
        var_fbreakdown_dn6 = assign39100_e51562_d_n6;
        var_fbreakdown_dn7 = assign39100_e51562_d_n7;
        var_fbreakdown_dn8 = assign39100_e51562_d_n8;

        let (assign39110_e51585, assign39110_e51585_d_n5, assign39110_e51585_d_n6, assign39110_e51585_d_n7, assign39110_e51585_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) && (var_guard792 == 0.0)) && (var_guard793 == 0.0)) {
        let assign39110_e51579: f64 = (var_alphaav * var_vbrstid_i);
        let assign39110_e51580: f64 = (var_vav + assign39110_e51579);
        let assign39110_e51582: f64 = (assign39110_e51580 * var_slopesti_d);
        let assign39110_e51583: f64 = (var_fstopsti_d + assign39110_e51582);
        (assign39110_e51583, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign39110_e51585;
        var_fbreakdown_dn5 = assign39110_e51585_d_n5;
        var_fbreakdown_dn6 = assign39110_e51585_d_n6;
        var_fbreakdown_dn7 = assign39110_e51585_d_n7;
        var_fbreakdown_dn8 = assign39110_e51585_d_n8;

        let (assign39120_e51604, assign39120_e51604_d_n5, assign39120_e51604_d_n6, assign39120_e51604_d_n7, assign39120_e51604_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard778 == 0.0)) {
        let assign39120_e51595: f64 = (var_id__blk213 + var_isrh);
        let assign39120_e51597: f64 = (assign39120_e51595 + var_itat);
        let assign39120_e51599: f64 = (assign39120_e51597 + var_ibbt);
        let assign39120_e51600: f64 = (p.p29 * assign39120_e51599);
        let assign39120_e51602: f64 = (assign39120_e51600 * var_fbreakdown);
        (assign39120_e51602, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign39120_e51600 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign39120_e51600 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign39120_e51600 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign39120_e51600 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign39120_e51604;
        var_ijunsti_dn5 = assign39120_e51604_d_n5;
        var_ijunsti_dn6 = assign39120_e51604_d_n6;
        var_ijunsti_dn7 = assign39120_e51604_d_n7;
        var_ijunsti_dn8 = assign39120_e51604_d_n8;

        let assign39130_e51607: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard795 = assign39130_e51607;

        let (assign39140_e51615, assign39140_e51615_d_n5, assign39140_e51615_d_n6, assign39140_e51615_d_n7, assign39140_e51615_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign39140_e51615;
        var_ijungat_dn5 = assign39140_e51615_d_n5;
        var_ijungat_dn6 = assign39140_e51615_d_n6;
        var_ijungat_dn7 = assign39140_e51615_d_n7;
        var_ijungat_dn8 = assign39140_e51615_d_n8;

        let (assign39150_e51626,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) {
        let assign39150_e51624: f64 = (var_idsatgat_d * var_idmult);
        (assign39150_e51624,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign39150_e51626;

        let assign39160_e51633: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard796 = assign39160_e51633;

        let (assign39170_e51644, assign39170_e51644_d_n5, assign39170_e51644_d_n6, assign39170_e51644_d_n7, assign39170_e51644_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign39170_e51644;
        var_isrh_dn5 = assign39170_e51644_d_n5;
        var_isrh_dn6 = assign39170_e51644_d_n6;
        var_isrh_dn7 = assign39170_e51644_d_n7;
        var_isrh_dn8 = assign39170_e51644_d_n8;

        let (assign39180_e51658,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) {
        let assign39180_e51656: f64 = (var_vbigat_d - var_vjsrh);
        (assign39180_e51656,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign39180_e51658;

        let (assign39190_e51677,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) {
        let assign39190_e51672: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign39190_e51673: f64 = (1.0 - assign39190_e51672);
        let assign39190_e51674: f64 = (assign39190_e51673).sqrt();
        let assign39190_e51675: f64 = (1.0 - assign39190_e51674);
        (assign39190_e51675,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign39190_e51677;

        let assign39200_e51680: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard797 = assign39200_e51680;

        let (assign39210_e51694,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) && (var_guard797 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign39210_e51694;

        let (assign39220_e51726,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) && (var_guard797 == 0.0)) {
        let assign39220_e51709: f64 = (var_wsrhstep * var_wsrhstep);
        let assign39220_e51711: f64 = (var_wsrhstep).ln();
        let assign39220_e51712: f64 = (assign39220_e51709 * assign39220_e51711);
        let assign39220_e51715: f64 = (1.0 - var_wsrhstep);
        let assign39220_e51716: f64 = (assign39220_e51712 / assign39220_e51715);
        let assign39220_e51718: f64 = (assign39220_e51716 + var_wsrhstep);
        let assign39220_e51722: f64 = (2.0 * var_pgatd_i);
        let assign39220_e51723: f64 = (1.0 - assign39220_e51722);
        let assign39220_e51724: f64 = (assign39220_e51718 * assign39220_e51723);
        (assign39220_e51724,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign39220_e51726;

        let (assign39230_e51740,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) {
        let assign39230_e51738: f64 = (var_wsrhstep + var_dwsrh);
        (assign39230_e51738,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign39230_e51740;

        let assign39240_e51743: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard798 = assign39240_e51743;

        let (assign39250_e51760, assign39250_e51760_d_n5, assign39250_e51760_d_n6, assign39250_e51760_d_n7, assign39250_e51760_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) && (var_guard798 != 0.0)) {
        let assign39250_e51757: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign39250_e51758: f64 = (assign39250_e51757).sqrt();
        (assign39250_e51758, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39250_e51760;
        var_tmp_dn5 = assign39250_e51760_d_n5;
        var_tmp_dn6 = assign39250_e51760_d_n6;
        var_tmp_dn7 = assign39250_e51760_d_n7;
        var_tmp_dn8 = assign39250_e51760_d_n8;

        let (assign39260_e51779, assign39260_e51779_d_n5, assign39260_e51779_d_n6, assign39260_e51779_d_n7, assign39260_e51779_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) && (var_guard798 == 0.0)) {
        let assign39260_e51775: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign39260_e51777: f64 = (assign39260_e51775).powf(var_pgatd_i);
        (assign39260_e51777, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39260_e51779;
        var_tmp_dn5 = assign39260_e51779_d_n5;
        var_tmp_dn6 = assign39260_e51779_d_n6;
        var_tmp_dn7 = assign39260_e51779_d_n7;
        var_tmp_dn8 = assign39260_e51779_d_n8;

        let (assign39270_e51793, assign39270_e51793_d_n5, assign39270_e51793_d_n6, assign39270_e51793_d_n7, assign39270_e51793_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) {
        let assign39270_e51791: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign39270_e51791, (var_wdepnulrgat_d * var_tmp_dn5), (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign39270_e51793;
        var_wdep_dn5 = assign39270_e51793_d_n5;
        var_wdep_dn6 = assign39270_e51793_d_n6;
        var_wdep_dn7 = assign39270_e51793_d_n7;
        var_wdep_dn8 = assign39270_e51793_d_n8;

        let (assign39280_e51811, assign39280_e51811_d_n5, assign39280_e51811_d_n6, assign39280_e51811_d_n7, assign39280_e51811_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) {
        let assign39280_e51806: f64 = (var_zinv - 1.0);
        let assign39280_e51808: f64 = (assign39280_e51806 * var_wdep);
        let assign39280_e51809: f64 = (var_ftdgat_d * assign39280_e51808);
        (assign39280_e51809, (var_ftdgat_d * (assign39280_e51806 * var_wdep_dn5)), (var_ftdgat_d * (assign39280_e51806 * var_wdep_dn6)), (var_ftdgat_d * (assign39280_e51806 * var_wdep_dn7)), (var_ftdgat_d * (assign39280_e51806 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign39280_e51811;
        var_asrh_dn5 = assign39280_e51811_d_n5;
        var_asrh_dn6 = assign39280_e51811_d_n6;
        var_asrh_dn7 = assign39280_e51811_d_n7;
        var_asrh_dn8 = assign39280_e51811_d_n8;

        let (assign39290_e51827, assign39290_e51827_d_n5, assign39290_e51827_d_n6, assign39290_e51827_d_n7, assign39290_e51827_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) {
        let assign39290_e51824: f64 = (var_asrh * var_wsrh);
        let assign39290_e51825: f64 = (var_csrhgatd_i * assign39290_e51824);
        (assign39290_e51825, (var_csrhgatd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign39290_e51827;
        var_isrh_dn5 = assign39290_e51827_d_n5;
        var_isrh_dn6 = assign39290_e51827_d_n6;
        var_isrh_dn7 = assign39290_e51827_d_n7;
        var_isrh_dn8 = assign39290_e51827_d_n8;

        let assign39300_e51830: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard799 = assign39300_e51830;

        let (assign39310_e51841, assign39310_e51841_d_n5, assign39310_e51841_d_n6, assign39310_e51841_d_n7, assign39310_e51841_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign39310_e51841;
        var_itat_dn5 = assign39310_e51841_d_n5;
        var_itat_dn6 = assign39310_e51841_d_n6;
        var_itat_dn7 = assign39310_e51841_d_n7;
        var_itat_dn8 = assign39310_e51841_d_n8;

        let (assign39320_e51859, assign39320_e51859_d_n5, assign39320_e51859_d_n6, assign39320_e51859_d_n7, assign39320_e51859_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39320_e51854: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign39320_e51856: f64 = (assign39320_e51854 / var_vbi_minus_vjsrh);
        let assign39320_e51857: f64 = (var_btatpartgat_d * assign39320_e51856);
        (assign39320_e51857, (var_btatpartgat_d * ((var_wdep_dn5 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign39320_e51859;
        var_btat_dn5 = assign39320_e51859_d_n5;
        var_btat_dn6 = assign39320_e51859_d_n6;
        var_btat_dn7 = assign39320_e51859_d_n7;
        var_btat_dn8 = assign39320_e51859_d_n8;

        let (assign39330_e51875, assign39330_e51875_d_n5, assign39330_e51875_d_n6, assign39330_e51875_d_n7, assign39330_e51875_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39330_e51871: f64 = (0.666666666666667 * var_atatgat_d);
        let assign39330_e51873: f64 = (assign39330_e51871 / var_btat);
        (assign39330_e51873, (-((assign39330_e51871 * var_btat_dn5) / (var_btat * var_btat))), (-((assign39330_e51871 * var_btat_dn6) / (var_btat * var_btat))), (-((assign39330_e51871 * var_btat_dn7) / (var_btat * var_btat))), (-((assign39330_e51871 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign39330_e51875;
        var_twoatatoverthreebtat_dn5 = assign39330_e51875_d_n5;
        var_twoatatoverthreebtat_dn6 = assign39330_e51875_d_n6;
        var_twoatatoverthreebtat_dn7 = assign39330_e51875_d_n7;
        var_twoatatoverthreebtat_dn8 = assign39330_e51875_d_n8;

        let (assign39340_e51889, assign39340_e51889_d_n5, assign39340_e51889_d_n6, assign39340_e51889_d_n7, assign39340_e51889_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39340_e51887: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign39340_e51887, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign39340_e51889;
        var_umaxbeforelimiting_dn5 = assign39340_e51889_d_n5;
        var_umaxbeforelimiting_dn6 = assign39340_e51889_d_n6;
        var_umaxbeforelimiting_dn7 = assign39340_e51889_d_n7;
        var_umaxbeforelimiting_dn8 = assign39340_e51889_d_n8;

        let (assign39350_e51910, assign39350_e51910_d_n5, assign39350_e51910_d_n6, assign39350_e51910_d_n7, assign39350_e51910_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39350_e51901: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign39350_e51904: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign39350_e51906: f64 = (assign39350_e51904 + 1.0);
        let assign39350_e51907: f64 = (assign39350_e51901 / assign39350_e51906);
        let assign39350_e51908: f64 = (assign39350_e51907).sqrt();
        (assign39350_e51908, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign39350_e51906) - (assign39350_e51901 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign39350_e51906 * assign39350_e51906)) / (2.0 * assign39350_e51908)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign39350_e51906) - (assign39350_e51901 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign39350_e51906 * assign39350_e51906)) / (2.0 * assign39350_e51908)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign39350_e51906) - (assign39350_e51901 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign39350_e51906 * assign39350_e51906)) / (2.0 * assign39350_e51908)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign39350_e51906) - (assign39350_e51901 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign39350_e51906 * assign39350_e51906)) / (2.0 * assign39350_e51908)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign39350_e51910;
        var_umax_dn5 = assign39350_e51910_d_n5;
        var_umax_dn6 = assign39350_e51910_d_n6;
        var_umax_dn7 = assign39350_e51910_d_n7;
        var_umax_dn8 = assign39350_e51910_d_n8;

        let (assign39360_e51923, assign39360_e51923_d_n5, assign39360_e51923_d_n6, assign39360_e51923_d_n7, assign39360_e51923_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39360_e51921: f64 = (var_umax).sqrt();
        (assign39360_e51921, (var_umax_dn5 / (2.0 * assign39360_e51921)), (var_umax_dn6 / (2.0 * assign39360_e51921)), (var_umax_dn7 / (2.0 * assign39360_e51921)), (var_umax_dn8 / (2.0 * assign39360_e51921)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign39360_e51923;
        var_sqrtumax_dn5 = assign39360_e51923_d_n5;
        var_sqrtumax_dn6 = assign39360_e51923_d_n6;
        var_sqrtumax_dn7 = assign39360_e51923_d_n7;
        var_sqrtumax_dn8 = assign39360_e51923_d_n8;

        let (assign39370_e51937, assign39370_e51937_d_n5, assign39370_e51937_d_n6, assign39370_e51937_d_n7, assign39370_e51937_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39370_e51935: f64 = (var_umax * var_sqrtumax);
        (assign39370_e51935, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign39370_e51937;
        var_umaxpoweronepointfive_dn5 = assign39370_e51937_d_n5;
        var_umaxpoweronepointfive_dn6 = assign39370_e51937_d_n6;
        var_umaxpoweronepointfive_dn7 = assign39370_e51937_d_n7;
        var_umaxpoweronepointfive_dn8 = assign39370_e51937_d_n8;

        let assign39380_e51939: f64 = (-var_pgatd_i);
        let assign39380_e51941: f64 = (assign39380_e51939 * var_one_over_one_minus_pgat_d);
        let assign39380_e51943: f64 = (-1.0);
        let assign39380_e51944: f64 = if assign39380_e51941 == assign39380_e51943 { 1.0 } else { 0.0 };
        var_guard800 = assign39380_e51944;

        let (assign39390_e51964, assign39390_e51964_d_n5, assign39390_e51964_d_n6, assign39390_e51964_d_n7, assign39390_e51964_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) && (var_guard800 != 0.0)) {
        let assign39390_e51960: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign39390_e51961: f64 = (1.0 + assign39390_e51960);
        let assign39390_e51962: f64 = (1.0 / assign39390_e51961);
        (assign39390_e51962, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign39390_e51961 * assign39390_e51961))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign39390_e51961 * assign39390_e51961))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign39390_e51961 * assign39390_e51961))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign39390_e51961 * assign39390_e51961))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign39390_e51964;
        var_wgamma_dn5 = assign39390_e51964_d_n5;
        var_wgamma_dn6 = assign39390_e51964_d_n6;
        var_wgamma_dn7 = assign39390_e51964_d_n7;
        var_wgamma_dn8 = assign39390_e51964_d_n8;

        let (assign39400_e51988, assign39400_e51988_d_n5, assign39400_e51988_d_n6, assign39400_e51988_d_n7, assign39400_e51988_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) && (var_guard800 == 0.0)) {
        let assign39400_e51980: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign39400_e51981: f64 = (1.0 + assign39400_e51980);
        let assign39400_e51983: f64 = (-var_pgatd_i);
        let assign39400_e51985: f64 = (assign39400_e51983 * var_one_over_one_minus_pgat_d);
        let assign39400_e51986: f64 = (assign39400_e51981).powf(assign39400_e51985);
        (assign39400_e51986, if 0.0 == 0.0 && ((assign39400_e51985) as f64).is_finite() && ((assign39400_e51985) as f64).fract() == 0.0 { if assign39400_e51985 == 0.0 { 0.0 } else { (assign39400_e51985 * ((assign39400_e51981).powf(assign39400_e51985 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign39400_e51986 * (assign39400_e51985 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign39400_e51981))) }, if 0.0 == 0.0 && ((assign39400_e51985) as f64).is_finite() && ((assign39400_e51985) as f64).fract() == 0.0 { if assign39400_e51985 == 0.0 { 0.0 } else { (assign39400_e51985 * ((assign39400_e51981).powf(assign39400_e51985 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign39400_e51986 * (assign39400_e51985 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign39400_e51981))) }, if 0.0 == 0.0 && ((assign39400_e51985) as f64).is_finite() && ((assign39400_e51985) as f64).fract() == 0.0 { if assign39400_e51985 == 0.0 { 0.0 } else { (assign39400_e51985 * ((assign39400_e51981).powf(assign39400_e51985 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign39400_e51986 * (assign39400_e51985 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign39400_e51981))) }, if 0.0 == 0.0 && ((assign39400_e51985) as f64).is_finite() && ((assign39400_e51985) as f64).fract() == 0.0 { if assign39400_e51985 == 0.0 { 0.0 } else { (assign39400_e51985 * ((assign39400_e51981).powf(assign39400_e51985 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign39400_e51986 * (assign39400_e51985 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign39400_e51981))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign39400_e51988;
        var_wgamma_dn5 = assign39400_e51988_d_n5;
        var_wgamma_dn6 = assign39400_e51988_d_n6;
        var_wgamma_dn7 = assign39400_e51988_d_n7;
        var_wgamma_dn8 = assign39400_e51988_d_n8;

        let (assign39410_e52006, assign39410_e52006_d_n5, assign39410_e52006_d_n6, assign39410_e52006_d_n7, assign39410_e52006_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39410_e52000: f64 = (var_wsrh * var_wgamma);
        let assign39410_e52003: f64 = (var_wsrh + var_wgamma);
        let assign39410_e52004: f64 = (assign39410_e52000 / assign39410_e52003);
        (assign39410_e52004, ((((var_wsrh * var_wgamma_dn5) * assign39410_e52003) - (assign39410_e52000 * var_wgamma_dn5)) / (assign39410_e52003 * assign39410_e52003)), ((((var_wsrh * var_wgamma_dn6) * assign39410_e52003) - (assign39410_e52000 * var_wgamma_dn6)) / (assign39410_e52003 * assign39410_e52003)), ((((var_wsrh * var_wgamma_dn7) * assign39410_e52003) - (assign39410_e52000 * var_wgamma_dn7)) / (assign39410_e52003 * assign39410_e52003)), ((((var_wsrh * var_wgamma_dn8) * assign39410_e52003) - (assign39410_e52000 * var_wgamma_dn8)) / (assign39410_e52003 * assign39410_e52003)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign39410_e52006;
        var_wtat_dn5 = assign39410_e52006_d_n5;
        var_wtat_dn6 = assign39410_e52006_d_n6;
        var_wtat_dn7 = assign39410_e52006_d_n7;
        var_wtat_dn8 = assign39410_e52006_d_n8;

        let (assign39420_e52023, assign39420_e52023_d_n5, assign39420_e52023_d_n6, assign39420_e52023_d_n7, assign39420_e52023_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39420_e52019: f64 = (var_btat / var_sqrtumax);
        let assign39420_e52020: f64 = (0.375 * assign39420_e52019);
        let assign39420_e52021: f64 = (assign39420_e52020).sqrt();
        (assign39420_e52021, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign39420_e52021)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign39420_e52021)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign39420_e52021)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign39420_e52021)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign39420_e52023;
        var_ktat_dn5 = assign39420_e52023_d_n5;
        var_ktat_dn6 = assign39420_e52023_d_n6;
        var_ktat_dn7 = assign39420_e52023_d_n7;
        var_ktat_dn8 = assign39420_e52023_d_n8;

        let (assign39430_e52041, assign39430_e52041_d_n5, assign39430_e52041_d_n6, assign39430_e52041_d_n7, assign39430_e52041_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39430_e52036: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign39430_e52037: f64 = (2.0 * assign39430_e52036);
        let assign39430_e52039: f64 = (assign39430_e52037 - var_umax);
        (assign39430_e52039, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign39430_e52041;
        var_ltat_dn5 = assign39430_e52041_d_n5;
        var_ltat_dn6 = assign39430_e52041_d_n6;
        var_ltat_dn7 = assign39430_e52041_d_n7;
        var_ltat_dn8 = assign39430_e52041_d_n8;

        let (assign39440_e52067, assign39440_e52067_d_n5, assign39440_e52067_d_n6, assign39440_e52067_d_n7, assign39440_e52067_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39440_e52053: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign39440_e52055: f64 = (assign39440_e52053 * var_sqrtumax);
        let assign39440_e52058: f64 = (var_atatgat_d * var_umax);
        let assign39440_e52059: f64 = (assign39440_e52055 - assign39440_e52058);
        let assign39440_e52063: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign39440_e52064: f64 = (0.5 * assign39440_e52063);
        let assign39440_e52065: f64 = (assign39440_e52059 + assign39440_e52064);
        (assign39440_e52065, (((((var_atatgat_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign39440_e52053 * var_sqrtumax_dn5)) - (var_atatgat_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign39440_e52053 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign39440_e52053 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign39440_e52053 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign39440_e52067;
        var_mtat_dn5 = assign39440_e52067_d_n5;
        var_mtat_dn6 = assign39440_e52067_d_n6;
        var_mtat_dn7 = assign39440_e52067_d_n7;
        var_mtat_dn8 = assign39440_e52067_d_n8;

        let (assign39450_e52083, assign39450_e52083_d_n5, assign39450_e52083_d_n6, assign39450_e52083_d_n7, assign39450_e52083_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39450_e52079: f64 = (var_ltat - 1.0);
        let assign39450_e52081: f64 = (assign39450_e52079 * var_ktat);
        (assign39450_e52081, ((var_ltat_dn5 * var_ktat) + (assign39450_e52079 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign39450_e52079 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign39450_e52079 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign39450_e52079 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign39450_e52083;
        var_xerfc_dn5 = assign39450_e52083_d_n5;
        var_xerfc_dn6 = assign39450_e52083_d_n6;
        var_xerfc_dn7 = assign39450_e52083_d_n7;
        var_xerfc_dn8 = assign39450_e52083_d_n8;

        let (assign39460_e52097, assign39460_e52097_d_n5, assign39460_e52097_d_n6, assign39460_e52097_d_n7, assign39460_e52097_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39460_e52095: f64 = (var_xerfc * var_xerfc);
        (assign39460_e52095, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign39460_e52097;
        var_ysq_dn5 = assign39460_e52097_d_n5;
        var_ysq_dn6 = assign39460_e52097_d_n6;
        var_ysq_dn7 = assign39460_e52097_d_n7;
        var_ysq_dn8 = assign39460_e52097_d_n8;

        let assign39470_e52100: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard801 = assign39470_e52100;

        let (assign39480_e52120, assign39480_e52120_d_n5, assign39480_e52120_d_n6, assign39480_e52120_d_n7, assign39480_e52120_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) && (var_guard801 != 0.0)) {
        let assign39480_e52116: f64 = (var_perfc * var_xerfc);
        let assign39480_e52117: f64 = (1.0 + assign39480_e52116);
        let assign39480_e52118: f64 = (1.0 / assign39480_e52117);
        (assign39480_e52118, (-((var_perfc * var_xerfc_dn5) / (assign39480_e52117 * assign39480_e52117))), (-((var_perfc * var_xerfc_dn6) / (assign39480_e52117 * assign39480_e52117))), (-((var_perfc * var_xerfc_dn7) / (assign39480_e52117 * assign39480_e52117))), (-((var_perfc * var_xerfc_dn8) / (assign39480_e52117 * assign39480_e52117))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign39480_e52120;
        var_terfc_dn5 = assign39480_e52120_d_n5;
        var_terfc_dn6 = assign39480_e52120_d_n6;
        var_terfc_dn7 = assign39480_e52120_d_n7;
        var_terfc_dn8 = assign39480_e52120_d_n8;

        let (assign39490_e52141, assign39490_e52141_d_n5, assign39490_e52141_d_n6, assign39490_e52141_d_n7, assign39490_e52141_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) && (var_guard801 == 0.0)) {
        let assign39490_e52137: f64 = (var_perfc * var_xerfc);
        let assign39490_e52138: f64 = (1.0 - assign39490_e52137);
        let assign39490_e52139: f64 = (1.0 / assign39490_e52138);
        (assign39490_e52139, (-((-(var_perfc * var_xerfc_dn5)) / (assign39490_e52138 * assign39490_e52138))), (-((-(var_perfc * var_xerfc_dn6)) / (assign39490_e52138 * assign39490_e52138))), (-((-(var_perfc * var_xerfc_dn7)) / (assign39490_e52138 * assign39490_e52138))), (-((-(var_perfc * var_xerfc_dn8)) / (assign39490_e52138 * assign39490_e52138))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign39490_e52141;
        var_terfc_dn5 = assign39490_e52141_d_n5;
        var_terfc_dn6 = assign39490_e52141_d_n6;
        var_terfc_dn7 = assign39490_e52141_d_n7;
        var_terfc_dn8 = assign39490_e52141_d_n8;

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
        *var_guard795_slot = var_guard795;
        *var_guard796_slot = var_guard796;
        *var_guard797_slot = var_guard797;
        *var_guard798_slot = var_guard798;
        *var_guard799_slot = var_guard799;
        *var_guard800_slot = var_guard800;
        *var_guard801_slot = var_guard801;
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

    pub(super) fn stamp_transient_block_84(
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
        var_guard176: f64,
        var_guard193: f64,
        var_guard795: f64,
        var_guard799: f64,
        var_id__blk213: f64,
        var_idsatbot_d: f64,
        var_idsatgat_d: f64,
        var_idsatsti_d: f64,
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
        var_v5: f64,
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
        var_guard802_slot: &mut f64,
        var_guard803_slot: &mut f64,
        var_guard804_slot: &mut f64,
        var_guard805_slot: &mut f64,
        var_guard806_slot: &mut f64,
        var_guard807_slot: &mut f64,
        var_guard808_slot: &mut f64,
        var_guard809_slot: &mut f64,
        var_guard810_slot: &mut f64,
        var_guard811_slot: &mut f64,
        var_i5_slot: &mut f64,
        var_i5_dn5_slot: &mut f64,
        var_i5_dn6_slot: &mut f64,
        var_i5_dn7_slot: &mut f64,
        var_i5_dn8_slot: &mut f64,
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
        var_isatfor1_d_slot: &mut f64,
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
        let mut var_guard802: f64 = *var_guard802_slot;
        let mut var_guard803: f64 = *var_guard803_slot;
        let mut var_guard804: f64 = *var_guard804_slot;
        let mut var_guard805: f64 = *var_guard805_slot;
        let mut var_guard806: f64 = *var_guard806_slot;
        let mut var_guard807: f64 = *var_guard807_slot;
        let mut var_guard808: f64 = *var_guard808_slot;
        let mut var_guard809: f64 = *var_guard809_slot;
        let mut var_guard810: f64 = *var_guard810_slot;
        let mut var_guard811: f64 = *var_guard811_slot;
        let mut var_i5: f64 = *var_i5_slot;
        let mut var_i5_dn5: f64 = *var_i5_dn5_slot;
        let mut var_i5_dn6: f64 = *var_i5_dn6_slot;
        let mut var_i5_dn7: f64 = *var_i5_dn7_slot;
        let mut var_i5_dn8: f64 = *var_i5_dn8_slot;
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
        let mut var_isatfor1_d: f64 = *var_isatfor1_d_slot;
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

        let assign39500_e52143: f64 = (-var_ysq);
        let assign39500_e52145: f64 = (assign39500_e52143 + var_mtat);
        let assign39500_e52147: f64 = (-230.25850929940458);
        let assign39500_e52148: f64 = if assign39500_e52145 > assign39500_e52147 { 1.0 } else { 0.0 };
        var_guard802 = assign39500_e52148;

        let (assign39510_e52166, assign39510_e52166_d_n5, assign39510_e52166_d_n6, assign39510_e52166_d_n7, assign39510_e52166_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) && (var_guard802 != 0.0)) {
        let assign39510_e52161: f64 = (-var_ysq);
        let assign39510_e52163: f64 = (assign39510_e52161 + var_mtat);
        let assign39510_e52164: f64 = (assign39510_e52163).exp();
        (assign39510_e52164, (assign39510_e52164 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign39510_e52164 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign39510_e52164 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign39510_e52164 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39510_e52166;
        var_tmp_dn5 = assign39510_e52166_d_n5;
        var_tmp_dn6 = assign39510_e52166_d_n6;
        var_tmp_dn7 = assign39510_e52166_d_n7;
        var_tmp_dn8 = assign39510_e52166_d_n8;

        let (assign39520_e52215, assign39520_e52215_d_n5, assign39520_e52215_d_n6, assign39520_e52215_d_n7, assign39520_e52215_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) && (var_guard802 == 0.0)) {
        let assign39520_e52182: f64 = (-230.25850929940458);
        let assign39520_e52184: f64 = (-var_ysq);
        let assign39520_e52186: f64 = (assign39520_e52184 + var_mtat);
        let assign39520_e52187: f64 = (assign39520_e52182 - assign39520_e52186);
        let assign39520_e52191: f64 = (-230.25850929940458);
        let assign39520_e52193: f64 = (-var_ysq);
        let assign39520_e52195: f64 = (assign39520_e52193 + var_mtat);
        let assign39520_e52196: f64 = (assign39520_e52191 - assign39520_e52195);
        let assign39520_e52199: f64 = (-230.25850929940458);
        let assign39520_e52201: f64 = (-var_ysq);
        let assign39520_e52203: f64 = (assign39520_e52201 + var_mtat);
        let assign39520_e52204: f64 = (assign39520_e52199 - assign39520_e52203);
        let assign39520_e52206: f64 = (assign39520_e52204 * 0.3333333333333333);
        let assign39520_e52207: f64 = (1.0 + assign39520_e52206);
        let assign39520_e52208: f64 = (assign39520_e52196 * assign39520_e52207);
        let assign39520_e52209: f64 = (0.5 * assign39520_e52208);
        let assign39520_e52210: f64 = (1.0 + assign39520_e52209);
        let assign39520_e52211: f64 = (assign39520_e52187 * assign39520_e52210);
        let assign39520_e52212: f64 = (1.0 + assign39520_e52211);
        let assign39520_e52213: f64 = (1e-100 / assign39520_e52212);
        (assign39520_e52213, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign39520_e52210) + (assign39520_e52187 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign39520_e52207) + (assign39520_e52196 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign39520_e52212 * assign39520_e52212))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign39520_e52210) + (assign39520_e52187 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign39520_e52207) + (assign39520_e52196 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign39520_e52212 * assign39520_e52212))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign39520_e52210) + (assign39520_e52187 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign39520_e52207) + (assign39520_e52196 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign39520_e52212 * assign39520_e52212))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign39520_e52210) + (assign39520_e52187 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign39520_e52207) + (assign39520_e52196 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign39520_e52212 * assign39520_e52212))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39520_e52215;
        var_tmp_dn5 = assign39520_e52215_d_n5;
        var_tmp_dn6 = assign39520_e52215_d_n6;
        var_tmp_dn7 = assign39520_e52215_d_n7;
        var_tmp_dn8 = assign39520_e52215_d_n8;

        let (assign39530_e52245, assign39530_e52245_d_n5, assign39530_e52245_d_n6, assign39530_e52245_d_n7, assign39530_e52245_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39530_e52227: f64 = (0.29214664 * var_terfc);
        let assign39530_e52231: f64 = (var_terfc * var_terfc);
        let assign39530_e52232: f64 = (var_berfc * assign39530_e52231);
        let assign39530_e52233: f64 = (assign39530_e52227 + assign39530_e52232);
        let assign39530_e52237: f64 = (var_terfc * var_terfc);
        let assign39530_e52239: f64 = (assign39530_e52237 * var_terfc);
        let assign39530_e52240: f64 = (var_cerfc * assign39530_e52239);
        let assign39530_e52241: f64 = (assign39530_e52233 + assign39530_e52240);
        let assign39530_e52243: f64 = (assign39530_e52241 * var_tmp);
        (assign39530_e52243, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign39530_e52237 * var_terfc_dn5)))) * var_tmp) + (assign39530_e52241 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign39530_e52237 * var_terfc_dn6)))) * var_tmp) + (assign39530_e52241 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign39530_e52237 * var_terfc_dn7)))) * var_tmp) + (assign39530_e52241 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign39530_e52237 * var_terfc_dn8)))) * var_tmp) + (assign39530_e52241 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign39530_e52245;
        var_erfcpos_dn5 = assign39530_e52245_d_n5;
        var_erfcpos_dn6 = assign39530_e52245_d_n6;
        var_erfcpos_dn7 = assign39530_e52245_d_n7;
        var_erfcpos_dn8 = assign39530_e52245_d_n8;

        let assign39540_e52248: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard803 = assign39540_e52248;

        let (assign39550_e52262, assign39550_e52262_d_n5, assign39550_e52262_d_n6, assign39550_e52262_d_n7, assign39550_e52262_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) && (var_guard803 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign39550_e52262;
        var_erfctimesexpmtat_dn5 = assign39550_e52262_d_n5;
        var_erfctimesexpmtat_dn6 = assign39550_e52262_d_n6;
        var_erfctimesexpmtat_dn7 = assign39550_e52262_d_n7;
        var_erfctimesexpmtat_dn8 = assign39550_e52262_d_n8;

        let assign39560_e52265: f64 = (-230.25850929940458);
        let assign39560_e52266: f64 = if var_mtat > assign39560_e52265 { 1.0 } else { 0.0 };
        var_guard804 = assign39560_e52266;

        let (assign39570_e52284, assign39570_e52284_d_n5, assign39570_e52284_d_n6, assign39570_e52284_d_n7, assign39570_e52284_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) && (var_guard803 == 0.0)) && (var_guard804 != 0.0)) {
        let assign39570_e52282: f64 = (var_mtat).exp();
        (assign39570_e52282, (assign39570_e52282 * var_mtat_dn5), (assign39570_e52282 * var_mtat_dn6), (assign39570_e52282 * var_mtat_dn7), (assign39570_e52282 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39570_e52284;
        var_tmp_dn5 = assign39570_e52284_d_n5;
        var_tmp_dn6 = assign39570_e52284_d_n6;
        var_tmp_dn7 = assign39570_e52284_d_n7;
        var_tmp_dn8 = assign39570_e52284_d_n8;

        let (assign39580_e52327, assign39580_e52327_d_n5, assign39580_e52327_d_n6, assign39580_e52327_d_n7, assign39580_e52327_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) && (var_guard803 == 0.0)) && (var_guard804 == 0.0)) {
        let assign39580_e52303: f64 = (-230.25850929940458);
        let assign39580_e52305: f64 = (assign39580_e52303 - var_mtat);
        let assign39580_e52309: f64 = (-230.25850929940458);
        let assign39580_e52311: f64 = (assign39580_e52309 - var_mtat);
        let assign39580_e52314: f64 = (-230.25850929940458);
        let assign39580_e52316: f64 = (assign39580_e52314 - var_mtat);
        let assign39580_e52318: f64 = (assign39580_e52316 * 0.3333333333333333);
        let assign39580_e52319: f64 = (1.0 + assign39580_e52318);
        let assign39580_e52320: f64 = (assign39580_e52311 * assign39580_e52319);
        let assign39580_e52321: f64 = (0.5 * assign39580_e52320);
        let assign39580_e52322: f64 = (1.0 + assign39580_e52321);
        let assign39580_e52323: f64 = (assign39580_e52305 * assign39580_e52322);
        let assign39580_e52324: f64 = (1.0 + assign39580_e52323);
        let assign39580_e52325: f64 = (1e-100 / assign39580_e52324);
        (assign39580_e52325, (-((1e-100 * (((-var_mtat_dn5) * assign39580_e52322) + (assign39580_e52305 * (0.5 * (((-var_mtat_dn5) * assign39580_e52319) + (assign39580_e52311 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign39580_e52324 * assign39580_e52324))), (-((1e-100 * (((-var_mtat_dn6) * assign39580_e52322) + (assign39580_e52305 * (0.5 * (((-var_mtat_dn6) * assign39580_e52319) + (assign39580_e52311 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign39580_e52324 * assign39580_e52324))), (-((1e-100 * (((-var_mtat_dn7) * assign39580_e52322) + (assign39580_e52305 * (0.5 * (((-var_mtat_dn7) * assign39580_e52319) + (assign39580_e52311 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign39580_e52324 * assign39580_e52324))), (-((1e-100 * (((-var_mtat_dn8) * assign39580_e52322) + (assign39580_e52305 * (0.5 * (((-var_mtat_dn8) * assign39580_e52319) + (assign39580_e52311 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign39580_e52324 * assign39580_e52324))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39580_e52327;
        var_tmp_dn5 = assign39580_e52327_d_n5;
        var_tmp_dn6 = assign39580_e52327_d_n6;
        var_tmp_dn7 = assign39580_e52327_d_n7;
        var_tmp_dn8 = assign39580_e52327_d_n8;

        let (assign39590_e52346, assign39590_e52346_d_n5, assign39590_e52346_d_n6, assign39590_e52346_d_n7, assign39590_e52346_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) && (var_guard803 == 0.0)) {
        let assign39590_e52342: f64 = (2.0 * var_tmp);
        let assign39590_e52344: f64 = (assign39590_e52342 - var_erfcpos);
        (assign39590_e52344, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign39590_e52346;
        var_erfctimesexpmtat_dn5 = assign39590_e52346_d_n5;
        var_erfctimesexpmtat_dn6 = assign39590_e52346_d_n6;
        var_erfctimesexpmtat_dn7 = assign39590_e52346_d_n7;
        var_erfctimesexpmtat_dn8 = assign39590_e52346_d_n8;

        let (assign39600_e52366, assign39600_e52366_d_n5, assign39600_e52366_d_n6, assign39600_e52366_d_n7, assign39600_e52366_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39600_e52358: f64 = (1.772453850905516 * 0.5);
        let assign39600_e52361: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign39600_e52363: f64 = (assign39600_e52361 / var_ktat);
        let assign39600_e52364: f64 = (assign39600_e52358 * assign39600_e52363);
        (assign39600_e52364, (assign39600_e52358 * ((((var_atatgat_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign39600_e52361 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign39600_e52358 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign39600_e52361 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign39600_e52358 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign39600_e52361 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign39600_e52358 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign39600_e52361 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign39600_e52366;
        var_gammamax_dn5 = assign39600_e52366_d_n5;
        var_gammamax_dn6 = assign39600_e52366_d_n6;
        var_gammamax_dn7 = assign39600_e52366_d_n7;
        var_gammamax_dn8 = assign39600_e52366_d_n8;

        let (assign39610_e52384, assign39610_e52384_d_n5, assign39610_e52384_d_n6, assign39610_e52384_d_n7, assign39610_e52384_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard799 == 0.0)) {
        let assign39610_e52379: f64 = (var_asrh * var_gammamax);
        let assign39610_e52381: f64 = (assign39610_e52379 * var_wtat);
        let assign39610_e52382: f64 = (var_ctatgatd_i * assign39610_e52381);
        (assign39610_e52382, (var_ctatgatd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign39610_e52379 * var_wtat_dn5))), (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign39610_e52379 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign39610_e52379 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign39610_e52379 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign39610_e52384;
        var_itat_dn5 = assign39610_e52384_d_n5;
        var_itat_dn6 = assign39610_e52384_d_n6;
        var_itat_dn7 = assign39610_e52384_d_n7;
        var_itat_dn8 = assign39610_e52384_d_n8;

        let assign39620_e52387: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard805 = assign39620_e52387;

        let (assign39630_e52398, assign39630_e52398_d_n5, assign39630_e52398_d_n6, assign39630_e52398_d_n7, assign39630_e52398_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard805 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign39630_e52398;
        var_ibbt_dn5 = assign39630_e52398_d_n5;
        var_ibbt_dn6 = assign39630_e52398_d_n6;
        var_ibbt_dn7 = assign39630_e52398_d_n7;
        var_ibbt_dn8 = assign39630_e52398_d_n8;

        let assign39640_e52401: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard806 = assign39640_e52401;

        let (assign39650_e52420, assign39650_e52420_d_n5, assign39650_e52420_d_n6, assign39650_e52420_d_n7, assign39650_e52420_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard805 == 0.0)) && (var_guard806 != 0.0)) {
        let assign39650_e52415: f64 = (var_vbirgatd_i - var_vbbt);
        let assign39650_e52417: f64 = (assign39650_e52415 * var_vbirgatinv_d);
        let assign39650_e52418: f64 = (assign39650_e52417).sqrt();
        (assign39650_e52418, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39650_e52420;
        var_tmp_dn5 = assign39650_e52420_d_n5;
        var_tmp_dn6 = assign39650_e52420_d_n6;
        var_tmp_dn7 = assign39650_e52420_d_n7;
        var_tmp_dn8 = assign39650_e52420_d_n8;

        let (assign39660_e52441, assign39660_e52441_d_n5, assign39660_e52441_d_n6, assign39660_e52441_d_n7, assign39660_e52441_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard805 == 0.0)) && (var_guard806 == 0.0)) {
        let assign39660_e52435: f64 = (var_vbirgatd_i - var_vbbt);
        let assign39660_e52437: f64 = (assign39660_e52435 * var_vbirgatinv_d);
        let assign39660_e52439: f64 = (assign39660_e52437).powf(var_pgatd_i);
        (assign39660_e52439, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39660_e52441;
        var_tmp_dn5 = assign39660_e52441_d_n5;
        var_tmp_dn6 = assign39660_e52441_d_n6;
        var_tmp_dn7 = assign39660_e52441_d_n7;
        var_tmp_dn8 = assign39660_e52441_d_n8;

        let (assign39670_e52461, assign39670_e52461_d_n5, assign39670_e52461_d_n6, assign39670_e52461_d_n7, assign39670_e52461_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard805 == 0.0)) {
        let assign39670_e52454: f64 = (var_vbirgatd_i - var_vbbt);
        let assign39670_e52456: f64 = (assign39670_e52454 * var_wdepnulrinvgat_d);
        let assign39670_e52458: f64 = (assign39670_e52456 / var_tmp);
        let assign39670_e52459: f64 = (var_one_over_one_minus_pgat_d * assign39670_e52458);
        (assign39670_e52459, (var_one_over_one_minus_pgat_d * (-((assign39670_e52456 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign39670_e52456 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign39670_e52456 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign39670_e52456 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign39670_e52461;
        var_fmaxr_dn5 = assign39670_e52461_d_n5;
        var_fmaxr_dn6 = assign39670_e52461_d_n6;
        var_fmaxr_dn7 = assign39670_e52461_d_n7;
        var_fmaxr_dn8 = assign39670_e52461_d_n8;

        let assign39680_e52463: f64 = (-var_fbbtgat_d);
        let assign39680_e52465: f64 = (assign39680_e52463 / var_fmaxr);
        let assign39680_e52466: f64 = (assign39680_e52465).abs();
        let assign39680_e52468: f64 = if assign39680_e52466 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard807 = assign39680_e52468;

        let (assign39690_e52486, assign39690_e52486_d_n5, assign39690_e52486_d_n6, assign39690_e52486_d_n7, assign39690_e52486_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard805 == 0.0)) && (var_guard807 != 0.0)) {
        let assign39690_e52481: f64 = (-var_fbbtgat_d);
        let assign39690_e52483: f64 = (assign39690_e52481 / var_fmaxr);
        let assign39690_e52484: f64 = (assign39690_e52483).exp();
        (assign39690_e52484, (assign39690_e52484 * ((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign39690_e52481 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign39690_e52484 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign39690_e52481 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign39690_e52484 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign39690_e52481 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign39690_e52484 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign39690_e52481 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39690_e52486;
        var_tmp_dn5 = assign39690_e52486_d_n5;
        var_tmp_dn6 = assign39690_e52486_d_n6;
        var_tmp_dn7 = assign39690_e52486_d_n7;
        var_tmp_dn8 = assign39690_e52486_d_n8;

        let assign39700_e52488: f64 = (-var_fbbtgat_d);
        let assign39700_e52490: f64 = (assign39700_e52488 / var_fmaxr);
        let assign39700_e52492: f64 = if assign39700_e52490 < 0.0 { 1.0 } else { 0.0 };
        var_guard808 = assign39700_e52492;

        let (assign39710_e52543, assign39710_e52543_d_n5, assign39710_e52543_d_n6, assign39710_e52543_d_n7, assign39710_e52543_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard805 == 0.0)) && (var_guard807 == 0.0)) && (var_guard808 != 0.0)) {
        let assign39710_e52510: f64 = (-230.25850929940458);
        let assign39710_e52512: f64 = (-var_fbbtgat_d);
        let assign39710_e52514: f64 = (assign39710_e52512 / var_fmaxr);
        let assign39710_e52515: f64 = (assign39710_e52510 - assign39710_e52514);
        let assign39710_e52519: f64 = (-230.25850929940458);
        let assign39710_e52521: f64 = (-var_fbbtgat_d);
        let assign39710_e52523: f64 = (assign39710_e52521 / var_fmaxr);
        let assign39710_e52524: f64 = (assign39710_e52519 - assign39710_e52523);
        let assign39710_e52527: f64 = (-230.25850929940458);
        let assign39710_e52529: f64 = (-var_fbbtgat_d);
        let assign39710_e52531: f64 = (assign39710_e52529 / var_fmaxr);
        let assign39710_e52532: f64 = (assign39710_e52527 - assign39710_e52531);
        let assign39710_e52534: f64 = (assign39710_e52532 * 0.3333333333333333);
        let assign39710_e52535: f64 = (1.0 + assign39710_e52534);
        let assign39710_e52536: f64 = (assign39710_e52524 * assign39710_e52535);
        let assign39710_e52537: f64 = (0.5 * assign39710_e52536);
        let assign39710_e52538: f64 = (1.0 + assign39710_e52537);
        let assign39710_e52539: f64 = (assign39710_e52515 * assign39710_e52538);
        let assign39710_e52540: f64 = (1.0 + assign39710_e52539);
        let assign39710_e52541: f64 = (1e-100 / assign39710_e52540);
        (assign39710_e52541, (-((1e-100 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign39710_e52512 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign39710_e52538) + (assign39710_e52515 * (0.5 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign39710_e52521 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign39710_e52535) + (assign39710_e52524 * ((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign39710_e52529 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign39710_e52540 * assign39710_e52540))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign39710_e52512 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign39710_e52538) + (assign39710_e52515 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign39710_e52521 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign39710_e52535) + (assign39710_e52524 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign39710_e52529 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign39710_e52540 * assign39710_e52540))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign39710_e52512 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign39710_e52538) + (assign39710_e52515 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign39710_e52521 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign39710_e52535) + (assign39710_e52524 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign39710_e52529 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign39710_e52540 * assign39710_e52540))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign39710_e52512 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign39710_e52538) + (assign39710_e52515 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign39710_e52521 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign39710_e52535) + (assign39710_e52524 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign39710_e52529 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign39710_e52540 * assign39710_e52540))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39710_e52543;
        var_tmp_dn5 = assign39710_e52543_d_n5;
        var_tmp_dn6 = assign39710_e52543_d_n6;
        var_tmp_dn7 = assign39710_e52543_d_n7;
        var_tmp_dn8 = assign39710_e52543_d_n8;

        let (assign39720_e52592, assign39720_e52592_d_n5, assign39720_e52592_d_n6, assign39720_e52592_d_n7, assign39720_e52592_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard805 == 0.0)) && (var_guard807 == 0.0)) && (var_guard808 == 0.0)) {
        let assign39720_e52562: f64 = (-var_fbbtgat_d);
        let assign39720_e52564: f64 = (assign39720_e52562 / var_fmaxr);
        let assign39720_e52566: f64 = (assign39720_e52564 - 230.25850929940458);
        let assign39720_e52570: f64 = (-var_fbbtgat_d);
        let assign39720_e52572: f64 = (assign39720_e52570 / var_fmaxr);
        let assign39720_e52574: f64 = (assign39720_e52572 - 230.25850929940458);
        let assign39720_e52577: f64 = (-var_fbbtgat_d);
        let assign39720_e52579: f64 = (assign39720_e52577 / var_fmaxr);
        let assign39720_e52581: f64 = (assign39720_e52579 - 230.25850929940458);
        let assign39720_e52583: f64 = (assign39720_e52581 * 0.3333333333333333);
        let assign39720_e52584: f64 = (1.0 + assign39720_e52583);
        let assign39720_e52585: f64 = (assign39720_e52574 * assign39720_e52584);
        let assign39720_e52586: f64 = (0.5 * assign39720_e52585);
        let assign39720_e52587: f64 = (1.0 + assign39720_e52586);
        let assign39720_e52588: f64 = (assign39720_e52566 * assign39720_e52587);
        let assign39720_e52589: f64 = (1.0 + assign39720_e52588);
        let assign39720_e52590: f64 = (1e100 * assign39720_e52589);
        (assign39720_e52590, (1e100 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign39720_e52562 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign39720_e52587) + (assign39720_e52566 * (0.5 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign39720_e52570 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign39720_e52584) + (assign39720_e52574 * (((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign39720_e52577 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign39720_e52562 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign39720_e52587) + (assign39720_e52566 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign39720_e52570 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign39720_e52584) + (assign39720_e52574 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign39720_e52577 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign39720_e52562 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign39720_e52587) + (assign39720_e52566 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign39720_e52570 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign39720_e52584) + (assign39720_e52574 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign39720_e52577 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign39720_e52562 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign39720_e52587) + (assign39720_e52566 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign39720_e52570 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign39720_e52584) + (assign39720_e52574 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign39720_e52577 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39720_e52592;
        var_tmp_dn5 = assign39720_e52592_d_n5;
        var_tmp_dn6 = assign39720_e52592_d_n6;
        var_tmp_dn7 = assign39720_e52592_d_n7;
        var_tmp_dn8 = assign39720_e52592_d_n8;

        let (assign39730_e52612, assign39730_e52612_d_n5, assign39730_e52612_d_n6, assign39730_e52612_d_n7, assign39730_e52612_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard805 == 0.0)) {
        let assign39730_e52605: f64 = (var_v5 * var_fmaxr);
        let assign39730_e52607: f64 = (assign39730_e52605 * var_fmaxr);
        let assign39730_e52609: f64 = (assign39730_e52607 * var_tmp);
        let assign39730_e52610: f64 = (var_cbbtgatd_i * assign39730_e52609);
        (assign39730_e52610, (var_cbbtgatd_i * (((((var_v5 * var_fmaxr_dn5) * var_fmaxr) + (assign39730_e52605 * var_fmaxr_dn5)) * var_tmp) + (assign39730_e52607 * var_tmp_dn5))), (var_cbbtgatd_i * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign39730_e52605 * var_fmaxr_dn6)) * var_tmp) + (assign39730_e52607 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign39730_e52605 * var_fmaxr_dn7)) * var_tmp) + (assign39730_e52607 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign39730_e52605 * var_fmaxr_dn8)) * var_tmp) + (assign39730_e52607 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign39730_e52612;
        var_ibbt_dn5 = assign39730_e52612_d_n5;
        var_ibbt_dn6 = assign39730_e52612_d_n6;
        var_ibbt_dn7 = assign39730_e52612_d_n7;
        var_ibbt_dn8 = assign39730_e52612_d_n8;

        let assign39740_e52615: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard809 = assign39740_e52615;

        let (assign39750_e52626, assign39750_e52626_d_n5, assign39750_e52626_d_n6, assign39750_e52626_d_n7, assign39750_e52626_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard809 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign39750_e52626;
        var_fbreakdown_dn5 = assign39750_e52626_d_n5;
        var_fbreakdown_dn6 = assign39750_e52626_d_n6;
        var_fbreakdown_dn7 = assign39750_e52626_d_n7;
        var_fbreakdown_dn8 = assign39750_e52626_d_n8;

        let assign39760_e52629: f64 = (-var_alphaav);
        let assign39760_e52631: f64 = (assign39760_e52629 * var_vbrgatd_i);
        let assign39760_e52632: f64 = if var_vav > assign39760_e52631 { 1.0 } else { 0.0 };
        var_guard810 = assign39760_e52632;

        let assign39770_e52635: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard811 = assign39770_e52635;

        let (assign39780_e52665, assign39780_e52665_d_n5, assign39780_e52665_d_n6, assign39780_e52665_d_n7, assign39780_e52665_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard809 == 0.0)) && (var_guard810 != 0.0)) && (var_guard811 != 0.0)) {
        let assign39780_e52651: f64 = (var_vav * var_vbrinvgat_d);
        let assign39780_e52654: f64 = (var_vav * var_vbrinvgat_d);
        let assign39780_e52655: f64 = (assign39780_e52651 * assign39780_e52654);
        let assign39780_e52658: f64 = (var_vav * var_vbrinvgat_d);
        let assign39780_e52659: f64 = (assign39780_e52655 * assign39780_e52658);
        let assign39780_e52662: f64 = (var_vav * var_vbrinvgat_d);
        let assign39780_e52663: f64 = (assign39780_e52659 * assign39780_e52662);
        (assign39780_e52663, (((((((var_vav * var_vbrinvgat_d_dn5) * assign39780_e52654) + (assign39780_e52651 * (var_vav * var_vbrinvgat_d_dn5))) * assign39780_e52658) + (assign39780_e52655 * (var_vav * var_vbrinvgat_d_dn5))) * assign39780_e52662) + (assign39780_e52659 * (var_vav * var_vbrinvgat_d_dn5))), (((((((var_vav * var_vbrinvgat_d_dn6) * assign39780_e52654) + (assign39780_e52651 * (var_vav * var_vbrinvgat_d_dn6))) * assign39780_e52658) + (assign39780_e52655 * (var_vav * var_vbrinvgat_d_dn6))) * assign39780_e52662) + (assign39780_e52659 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign39780_e52654) + (assign39780_e52651 * (var_vav * var_vbrinvgat_d_dn7))) * assign39780_e52658) + (assign39780_e52655 * (var_vav * var_vbrinvgat_d_dn7))) * assign39780_e52662) + (assign39780_e52659 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign39780_e52654) + (assign39780_e52651 * (var_vav * var_vbrinvgat_d_dn8))) * assign39780_e52658) + (assign39780_e52655 * (var_vav * var_vbrinvgat_d_dn8))) * assign39780_e52662) + (assign39780_e52659 * (var_vav * var_vbrinvgat_d_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39780_e52665;
        var_tmp_dn5 = assign39780_e52665_d_n5;
        var_tmp_dn6 = assign39780_e52665_d_n6;
        var_tmp_dn7 = assign39780_e52665_d_n7;
        var_tmp_dn8 = assign39780_e52665_d_n8;

        let (assign39790_e52687, assign39790_e52687_d_n5, assign39790_e52687_d_n6, assign39790_e52687_d_n7, assign39790_e52687_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard809 == 0.0)) && (var_guard810 != 0.0)) && (var_guard811 == 0.0)) {
        let assign39790_e52682: f64 = (var_vav * var_vbrinvgat_d);
        let assign39790_e52683: f64 = (assign39790_e52682).abs();
        let assign39790_e52685: f64 = (assign39790_e52683).powf(var_pbrgatd_i);
        (assign39790_e52685, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign39790_e52683).powf(var_pbrgatd_i - 1.0) * if assign39790_e52682 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) })) } } else { (assign39790_e52685 * (var_pbrgatd_i * (if assign39790_e52682 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) } / assign39790_e52683))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign39790_e52683).powf(var_pbrgatd_i - 1.0) * if assign39790_e52682 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign39790_e52685 * (var_pbrgatd_i * (if assign39790_e52682 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign39790_e52683))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign39790_e52683).powf(var_pbrgatd_i - 1.0) * if assign39790_e52682 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign39790_e52685 * (var_pbrgatd_i * (if assign39790_e52682 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign39790_e52683))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign39790_e52683).powf(var_pbrgatd_i - 1.0) * if assign39790_e52682 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign39790_e52685 * (var_pbrgatd_i * (if assign39790_e52682 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign39790_e52683))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign39790_e52687;
        var_tmp_dn5 = assign39790_e52687_d_n5;
        var_tmp_dn6 = assign39790_e52687_d_n6;
        var_tmp_dn7 = assign39790_e52687_d_n7;
        var_tmp_dn8 = assign39790_e52687_d_n8;

        let (assign39800_e52705, assign39800_e52705_d_n5, assign39800_e52705_d_n6, assign39800_e52705_d_n7, assign39800_e52705_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard809 == 0.0)) && (var_guard810 != 0.0)) {
        let assign39800_e52702: f64 = (1.0 - var_tmp);
        let assign39800_e52703: f64 = (1.0 / assign39800_e52702);
        (assign39800_e52703, (-((-var_tmp_dn5) / (assign39800_e52702 * assign39800_e52702))), (-((-var_tmp_dn6) / (assign39800_e52702 * assign39800_e52702))), (-((-var_tmp_dn7) / (assign39800_e52702 * assign39800_e52702))), (-((-var_tmp_dn8) / (assign39800_e52702 * assign39800_e52702))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign39800_e52705;
        var_fbreakdown_dn5 = assign39800_e52705_d_n5;
        var_fbreakdown_dn6 = assign39800_e52705_d_n6;
        var_fbreakdown_dn7 = assign39800_e52705_d_n7;
        var_fbreakdown_dn8 = assign39800_e52705_d_n8;

        let (assign39810_e52728, assign39810_e52728_d_n5, assign39810_e52728_d_n6, assign39810_e52728_d_n7, assign39810_e52728_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) && (var_guard809 == 0.0)) && (var_guard810 == 0.0)) {
        let assign39810_e52722: f64 = (var_alphaav * var_vbrgatd_i);
        let assign39810_e52723: f64 = (var_vav + assign39810_e52722);
        let assign39810_e52725: f64 = (assign39810_e52723 * var_slopegat_d);
        let assign39810_e52726: f64 = (var_fstopgat_d + assign39810_e52725);
        (assign39810_e52726, (assign39810_e52723 * var_slopegat_d_dn5), (assign39810_e52723 * var_slopegat_d_dn6), (assign39810_e52723 * var_slopegat_d_dn7), (assign39810_e52723 * var_slopegat_d_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign39810_e52728;
        var_fbreakdown_dn5 = assign39810_e52728_d_n5;
        var_fbreakdown_dn6 = assign39810_e52728_d_n6;
        var_fbreakdown_dn7 = assign39810_e52728_d_n7;
        var_fbreakdown_dn8 = assign39810_e52728_d_n8;

        let (assign39820_e52747, assign39820_e52747_d_n5, assign39820_e52747_d_n6, assign39820_e52747_d_n7, assign39820_e52747_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard795 == 0.0)) {
        let assign39820_e52738: f64 = (var_id__blk213 + var_isrh);
        let assign39820_e52740: f64 = (assign39820_e52738 + var_itat);
        let assign39820_e52742: f64 = (assign39820_e52740 + var_ibbt);
        let assign39820_e52743: f64 = (p.p29 * assign39820_e52742);
        let assign39820_e52745: f64 = (assign39820_e52743 * var_fbreakdown);
        (assign39820_e52745, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign39820_e52743 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign39820_e52743 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign39820_e52743 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign39820_e52743 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign39820_e52747;
        var_ijungat_dn5 = assign39820_e52747_d_n5;
        var_ijungat_dn6 = assign39820_e52747_d_n6;
        var_ijungat_dn7 = assign39820_e52747_d_n7;
        var_ijungat_dn8 = assign39820_e52747_d_n8;

        let (assign39830_e52763, assign39830_e52763_d_n5, assign39830_e52763_d_n6, assign39830_e52763_d_n7, assign39830_e52763_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign39830_e52753: f64 = (var_abdrain_i * var_ijunbot);
        let assign39830_e52756: f64 = (var_lsdrain_i * var_ijunsti);
        let assign39830_e52757: f64 = (assign39830_e52753 + assign39830_e52756);
        let assign39830_e52760: f64 = (var_lgdrain_i * var_ijungat);
        let assign39830_e52761: f64 = (assign39830_e52757 + assign39830_e52760);
        (assign39830_e52761, (((var_abdrain_i * var_ijunbot_dn5) + (var_lsdrain_i * var_ijunsti_dn5)) + (var_lgdrain_i * var_ijungat_dn5)), (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)),)
    } else {
        (var_i5, var_i5_dn5, var_i5_dn6, var_i5_dn7, var_i5_dn8,)
    }
};
        var_i5 = assign39830_e52763;
        var_i5_dn5 = assign39830_e52763_d_n5;
        var_i5_dn6 = assign39830_e52763_d_n6;
        var_i5_dn7 = assign39830_e52763_d_n7;
        var_i5_dn8 = assign39830_e52763_d_n8;

        let (assign39840_e52779,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign39840_e52769: f64 = (var_abdrain_i * var_idsatbot_d);
        let assign39840_e52772: f64 = (var_lsdrain_i * var_idsatsti_d);
        let assign39840_e52773: f64 = (assign39840_e52769 + assign39840_e52772);
        let assign39840_e52776: f64 = (var_lgdrain_i * var_idsatgat_d);
        let assign39840_e52777: f64 = (assign39840_e52773 + assign39840_e52776);
        (assign39840_e52777,)
    } else {
        (var_isatfor1_d,)
    }
};
        var_isatfor1_d = assign39840_e52779;

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
        *var_guard802_slot = var_guard802;
        *var_guard803_slot = var_guard803;
        *var_guard804_slot = var_guard804;
        *var_guard805_slot = var_guard805;
        *var_guard806_slot = var_guard806;
        *var_guard807_slot = var_guard807;
        *var_guard808_slot = var_guard808;
        *var_guard809_slot = var_guard809;
        *var_guard810_slot = var_guard810;
        *var_guard811_slot = var_guard811;
        *var_i5_slot = var_i5;
        *var_i5_dn5_slot = var_i5_dn5;
        *var_i5_dn6_slot = var_i5_dn6;
        *var_i5_dn7_slot = var_i5_dn7;
        *var_i5_dn8_slot = var_i5_dn8;
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
        *var_isatfor1_d_slot = var_isatfor1_d;
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

    pub(super) fn stamp_transient_block_85(
        p: &Parameters,
        var_abdrain_i: f64,
        var_cjobot_d: f64,
        var_cjogat_d: f64,
        var_cjosti_d: f64,
        var_fjunqd_i: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_i1: f64,
        var_i1_dn5: f64,
        var_i1_dn6: f64,
        var_i1_dn7: f64,
        var_i1_dn8: f64,
        var_i2: f64,
        var_i2_dn5: f64,
        var_i2_dn6: f64,
        var_i2_dn7: f64,
        var_i2_dn8: f64,
        var_i3: f64,
        var_i3_dn5: f64,
        var_i3_dn6: f64,
        var_i3_dn7: f64,
        var_i3_dn8: f64,
        var_i4: f64,
        var_i4_dn5: f64,
        var_i4_dn6: f64,
        var_i4_dn7: f64,
        var_i4_dn8: f64,
        var_i5: f64,
        var_i5_dn5: f64,
        var_i5_dn6: f64,
        var_i5_dn7: f64,
        var_i5_dn8: f64,
        var_isatfor1_d: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_mfor1_d: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_v1: f64,
        var_v2: f64,
        var_v3: f64,
        var_v4: f64,
        var_v5: f64,
        var_alphaje_slot: &mut f64,
        var_alphaje_dn5_slot: &mut f64,
        var_alphaje_dn6_slot: &mut f64,
        var_alphaje_dn7_slot: &mut f64,
        var_alphaje_dn8_slot: &mut f64,
        var_expxhf1_d_slot: &mut f64,
        var_guard812_slot: &mut f64,
        var_guard813_slot: &mut f64,
        var_guard814_slot: &mut f64,
        var_guard815_slot: &mut f64,
        var_guard816_slot: &mut f64,
        var_guard817_slot: &mut f64,
        var_guard818_slot: &mut f64,
        var_guard819_slot: &mut f64,
        var_guard820_slot: &mut f64,
        var_guard821_slot: &mut f64,
        var_i1_cor_slot: &mut f64,
        var_i1_cor_dn5_slot: &mut f64,
        var_i1_cor_dn6_slot: &mut f64,
        var_i1_cor_dn7_slot: &mut f64,
        var_i1_cor_dn8_slot: &mut f64,
        var_i2_cor_slot: &mut f64,
        var_i2_cor_dn5_slot: &mut f64,
        var_i2_cor_dn6_slot: &mut f64,
        var_i2_cor_dn7_slot: &mut f64,
        var_i2_cor_dn8_slot: &mut f64,
        var_i3_cor_slot: &mut f64,
        var_i3_cor_dn5_slot: &mut f64,
        var_i3_cor_dn6_slot: &mut f64,
        var_i3_cor_dn7_slot: &mut f64,
        var_i3_cor_dn8_slot: &mut f64,
        var_i4_cor_slot: &mut f64,
        var_i4_cor_dn5_slot: &mut f64,
        var_i4_cor_dn6_slot: &mut f64,
        var_i4_cor_dn7_slot: &mut f64,
        var_i4_cor_dn8_slot: &mut f64,
        var_i5_cor_slot: &mut f64,
        var_i5_cor_dn5_slot: &mut f64,
        var_i5_cor_dn6_slot: &mut f64,
        var_i5_cor_dn7_slot: &mut f64,
        var_i5_cor_dn8_slot: &mut f64,
        var_isatfor2_d_slot: &mut f64,
        var_isatfor2_d_dn5_slot: &mut f64,
        var_isatfor2_d_dn6_slot: &mut f64,
        var_isatfor2_d_dn7_slot: &mut f64,
        var_isatfor2_d_dn8_slot: &mut f64,
        var_isatrev_d_slot: &mut f64,
        var_isatrev_d_dn5_slot: &mut f64,
        var_isatrev_d_dn6_slot: &mut f64,
        var_isatrev_d_dn7_slot: &mut f64,
        var_isatrev_d_dn8_slot: &mut f64,
        var_m0_rev_slot: &mut f64,
        var_m0_rev_dn5_slot: &mut f64,
        var_m0_rev_dn6_slot: &mut f64,
        var_m0_rev_dn7_slot: &mut f64,
        var_m0_rev_dn8_slot: &mut f64,
        var_m0flag_d_slot: &mut f64,
        var_mcor_rev_slot: &mut f64,
        var_mcor_rev_dn5_slot: &mut f64,
        var_mcor_rev_dn6_slot: &mut f64,
        var_mcor_rev_dn7_slot: &mut f64,
        var_mcor_rev_dn8_slot: &mut f64,
        var_mfor2_d_slot: &mut f64,
        var_mfor2_d_dn5_slot: &mut f64,
        var_mfor2_d_dn6_slot: &mut f64,
        var_mfor2_d_dn7_slot: &mut f64,
        var_mfor2_d_dn8_slot: &mut f64,
        var_mrev_d_slot: &mut f64,
        var_mrev_d_dn5_slot: &mut f64,
        var_mrev_d_dn6_slot: &mut f64,
        var_mrev_d_dn7_slot: &mut f64,
        var_mrev_d_dn8_slot: &mut f64,
        var_tt0_slot: &mut f64,
        var_tt1_slot: &mut f64,
        var_tt1_dn5_slot: &mut f64,
        var_tt1_dn6_slot: &mut f64,
        var_tt1_dn7_slot: &mut f64,
        var_tt1_dn8_slot: &mut f64,
        var_tt2_slot: &mut f64,
        var_tt2_dn5_slot: &mut f64,
        var_tt2_dn6_slot: &mut f64,
        var_tt2_dn7_slot: &mut f64,
        var_tt2_dn8_slot: &mut f64,
        var_xhighf1_d_slot: &mut f64,
        var_xhighf2_d_slot: &mut f64,
        var_xhighf2_d_dn5_slot: &mut f64,
        var_xhighf2_d_dn6_slot: &mut f64,
        var_xhighf2_d_dn7_slot: &mut f64,
        var_xhighf2_d_dn8_slot: &mut f64,
        var_xhighr_d_slot: &mut f64,
        var_xhighr_d_dn5_slot: &mut f64,
        var_xhighr_d_dn6_slot: &mut f64,
        var_xhighr_d_dn7_slot: &mut f64,
        var_xhighr_d_dn8_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zfrac_slot: &mut f64,
    ) {
        let mut var_alphaje: f64 = *var_alphaje_slot;
        let mut var_alphaje_dn5: f64 = *var_alphaje_dn5_slot;
        let mut var_alphaje_dn6: f64 = *var_alphaje_dn6_slot;
        let mut var_alphaje_dn7: f64 = *var_alphaje_dn7_slot;
        let mut var_alphaje_dn8: f64 = *var_alphaje_dn8_slot;
        let mut var_expxhf1_d: f64 = *var_expxhf1_d_slot;
        let mut var_guard812: f64 = *var_guard812_slot;
        let mut var_guard813: f64 = *var_guard813_slot;
        let mut var_guard814: f64 = *var_guard814_slot;
        let mut var_guard815: f64 = *var_guard815_slot;
        let mut var_guard816: f64 = *var_guard816_slot;
        let mut var_guard817: f64 = *var_guard817_slot;
        let mut var_guard818: f64 = *var_guard818_slot;
        let mut var_guard819: f64 = *var_guard819_slot;
        let mut var_guard820: f64 = *var_guard820_slot;
        let mut var_guard821: f64 = *var_guard821_slot;
        let mut var_i1_cor: f64 = *var_i1_cor_slot;
        let mut var_i1_cor_dn5: f64 = *var_i1_cor_dn5_slot;
        let mut var_i1_cor_dn6: f64 = *var_i1_cor_dn6_slot;
        let mut var_i1_cor_dn7: f64 = *var_i1_cor_dn7_slot;
        let mut var_i1_cor_dn8: f64 = *var_i1_cor_dn8_slot;
        let mut var_i2_cor: f64 = *var_i2_cor_slot;
        let mut var_i2_cor_dn5: f64 = *var_i2_cor_dn5_slot;
        let mut var_i2_cor_dn6: f64 = *var_i2_cor_dn6_slot;
        let mut var_i2_cor_dn7: f64 = *var_i2_cor_dn7_slot;
        let mut var_i2_cor_dn8: f64 = *var_i2_cor_dn8_slot;
        let mut var_i3_cor: f64 = *var_i3_cor_slot;
        let mut var_i3_cor_dn5: f64 = *var_i3_cor_dn5_slot;
        let mut var_i3_cor_dn6: f64 = *var_i3_cor_dn6_slot;
        let mut var_i3_cor_dn7: f64 = *var_i3_cor_dn7_slot;
        let mut var_i3_cor_dn8: f64 = *var_i3_cor_dn8_slot;
        let mut var_i4_cor: f64 = *var_i4_cor_slot;
        let mut var_i4_cor_dn5: f64 = *var_i4_cor_dn5_slot;
        let mut var_i4_cor_dn6: f64 = *var_i4_cor_dn6_slot;
        let mut var_i4_cor_dn7: f64 = *var_i4_cor_dn7_slot;
        let mut var_i4_cor_dn8: f64 = *var_i4_cor_dn8_slot;
        let mut var_i5_cor: f64 = *var_i5_cor_slot;
        let mut var_i5_cor_dn5: f64 = *var_i5_cor_dn5_slot;
        let mut var_i5_cor_dn6: f64 = *var_i5_cor_dn6_slot;
        let mut var_i5_cor_dn7: f64 = *var_i5_cor_dn7_slot;
        let mut var_i5_cor_dn8: f64 = *var_i5_cor_dn8_slot;
        let mut var_isatfor2_d: f64 = *var_isatfor2_d_slot;
        let mut var_isatfor2_d_dn5: f64 = *var_isatfor2_d_dn5_slot;
        let mut var_isatfor2_d_dn6: f64 = *var_isatfor2_d_dn6_slot;
        let mut var_isatfor2_d_dn7: f64 = *var_isatfor2_d_dn7_slot;
        let mut var_isatfor2_d_dn8: f64 = *var_isatfor2_d_dn8_slot;
        let mut var_isatrev_d: f64 = *var_isatrev_d_slot;
        let mut var_isatrev_d_dn5: f64 = *var_isatrev_d_dn5_slot;
        let mut var_isatrev_d_dn6: f64 = *var_isatrev_d_dn6_slot;
        let mut var_isatrev_d_dn7: f64 = *var_isatrev_d_dn7_slot;
        let mut var_isatrev_d_dn8: f64 = *var_isatrev_d_dn8_slot;
        let mut var_m0_rev: f64 = *var_m0_rev_slot;
        let mut var_m0_rev_dn5: f64 = *var_m0_rev_dn5_slot;
        let mut var_m0_rev_dn6: f64 = *var_m0_rev_dn6_slot;
        let mut var_m0_rev_dn7: f64 = *var_m0_rev_dn7_slot;
        let mut var_m0_rev_dn8: f64 = *var_m0_rev_dn8_slot;
        let mut var_m0flag_d: f64 = *var_m0flag_d_slot;
        let mut var_mcor_rev: f64 = *var_mcor_rev_slot;
        let mut var_mcor_rev_dn5: f64 = *var_mcor_rev_dn5_slot;
        let mut var_mcor_rev_dn6: f64 = *var_mcor_rev_dn6_slot;
        let mut var_mcor_rev_dn7: f64 = *var_mcor_rev_dn7_slot;
        let mut var_mcor_rev_dn8: f64 = *var_mcor_rev_dn8_slot;
        let mut var_mfor2_d: f64 = *var_mfor2_d_slot;
        let mut var_mfor2_d_dn5: f64 = *var_mfor2_d_dn5_slot;
        let mut var_mfor2_d_dn6: f64 = *var_mfor2_d_dn6_slot;
        let mut var_mfor2_d_dn7: f64 = *var_mfor2_d_dn7_slot;
        let mut var_mfor2_d_dn8: f64 = *var_mfor2_d_dn8_slot;
        let mut var_mrev_d: f64 = *var_mrev_d_slot;
        let mut var_mrev_d_dn5: f64 = *var_mrev_d_dn5_slot;
        let mut var_mrev_d_dn6: f64 = *var_mrev_d_dn6_slot;
        let mut var_mrev_d_dn7: f64 = *var_mrev_d_dn7_slot;
        let mut var_mrev_d_dn8: f64 = *var_mrev_d_dn8_slot;
        let mut var_tt0: f64 = *var_tt0_slot;
        let mut var_tt1: f64 = *var_tt1_slot;
        let mut var_tt1_dn5: f64 = *var_tt1_dn5_slot;
        let mut var_tt1_dn6: f64 = *var_tt1_dn6_slot;
        let mut var_tt1_dn7: f64 = *var_tt1_dn7_slot;
        let mut var_tt1_dn8: f64 = *var_tt1_dn8_slot;
        let mut var_tt2: f64 = *var_tt2_slot;
        let mut var_tt2_dn5: f64 = *var_tt2_dn5_slot;
        let mut var_tt2_dn6: f64 = *var_tt2_dn6_slot;
        let mut var_tt2_dn7: f64 = *var_tt2_dn7_slot;
        let mut var_tt2_dn8: f64 = *var_tt2_dn8_slot;
        let mut var_xhighf1_d: f64 = *var_xhighf1_d_slot;
        let mut var_xhighf2_d: f64 = *var_xhighf2_d_slot;
        let mut var_xhighf2_d_dn5: f64 = *var_xhighf2_d_dn5_slot;
        let mut var_xhighf2_d_dn6: f64 = *var_xhighf2_d_dn6_slot;
        let mut var_xhighf2_d_dn7: f64 = *var_xhighf2_d_dn7_slot;
        let mut var_xhighf2_d_dn8: f64 = *var_xhighf2_d_dn8_slot;
        let mut var_xhighr_d: f64 = *var_xhighr_d_slot;
        let mut var_xhighr_d_dn5: f64 = *var_xhighr_d_dn5_slot;
        let mut var_xhighr_d_dn6: f64 = *var_xhighr_d_dn6_slot;
        let mut var_xhighr_d_dn7: f64 = *var_xhighr_d_dn7_slot;
        let mut var_xhighr_d_dn8: f64 = *var_xhighr_d_dn8_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;

        let (assign39850_e52796, assign39850_e52796_d_n5, assign39850_e52796_d_n6, assign39850_e52796_d_n7, assign39850_e52796_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign39850_e52787: f64 = (var_v4 * var_phitdinv);
        let assign39850_e52789: f64 = (assign39850_e52787 * var_mfor1_d);
        let assign39850_e52790: f64 = (assign39850_e52789).exp();
        let assign39850_e52792: f64 = (assign39850_e52790 - 1.0);
        let assign39850_e52793: f64 = (var_isatfor1_d * assign39850_e52792);
        let assign39850_e52794: f64 = (var_i4 - assign39850_e52793);
        (assign39850_e52794, var_i4_dn5, var_i4_dn6, var_i4_dn7, var_i4_dn8,)
    } else {
        (var_i4_cor, var_i4_cor_dn5, var_i4_cor_dn6, var_i4_cor_dn7, var_i4_cor_dn8,)
    }
};
        var_i4_cor = assign39850_e52796;
        var_i4_cor_dn5 = assign39850_e52796_d_n5;
        var_i4_cor_dn6 = assign39850_e52796_d_n6;
        var_i4_cor_dn7 = assign39850_e52796_d_n7;
        var_i4_cor_dn8 = assign39850_e52796_d_n8;

        let (assign39860_e52813, assign39860_e52813_d_n5, assign39860_e52813_d_n6, assign39860_e52813_d_n7, assign39860_e52813_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign39860_e52804: f64 = (var_v5 * var_phitdinv);
        let assign39860_e52806: f64 = (assign39860_e52804 * var_mfor1_d);
        let assign39860_e52807: f64 = (assign39860_e52806).exp();
        let assign39860_e52809: f64 = (assign39860_e52807 - 1.0);
        let assign39860_e52810: f64 = (var_isatfor1_d * assign39860_e52809);
        let assign39860_e52811: f64 = (var_i5 - assign39860_e52810);
        (assign39860_e52811, var_i5_dn5, var_i5_dn6, var_i5_dn7, var_i5_dn8,)
    } else {
        (var_i5_cor, var_i5_cor_dn5, var_i5_cor_dn6, var_i5_cor_dn7, var_i5_cor_dn8,)
    }
};
        var_i5_cor = assign39860_e52813;
        var_i5_cor_dn5 = assign39860_e52813_d_n5;
        var_i5_cor_dn6 = assign39860_e52813_d_n6;
        var_i5_cor_dn7 = assign39860_e52813_d_n7;
        var_i5_cor_dn8 = assign39860_e52813_d_n8;

        let assign39870_e52825: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard812 = assign39870_e52825;

        let assign39880_e52832: f64 = if ((var_i4 > 0.0) && (var_i5 > 0.0)) { 1.0 } else { 0.0 };
        var_guard813 = assign39880_e52832;

        let assign39890_e52835: f64 = (var_i4_cor / var_i4);
        let assign39890_e52840: f64 = (var_i5_cor / var_i5);
        let assign39890_e52855: f64 = if (((((assign39890_e52835 > 0.001) || (assign39890_e52840 > 0.001)) && (var_i4_cor > 0.0)) && (var_i5_cor > 0.0)) && (var_i5_cor > var_i4_cor)) { 1.0 } else { 0.0 };
        var_guard814 = assign39890_e52855;

        let (assign39900_e52869, assign39900_e52869_d_n5, assign39900_e52869_d_n6, assign39900_e52869_d_n7, assign39900_e52869_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard813 != 0.0)) && (var_guard814 != 0.0)) {
        let assign39900_e52867: f64 = (var_i4_cor / var_i5_cor);
        (assign39900_e52867, (((var_i4_cor_dn5 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn5)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn6 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn6)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn7 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn7)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn8 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn8)) / (var_i5_cor * var_i5_cor)),)
    } else {
        (var_alphaje, var_alphaje_dn5, var_alphaje_dn6, var_alphaje_dn7, var_alphaje_dn8,)
    }
};
        var_alphaje = assign39900_e52869;
        var_alphaje_dn5 = assign39900_e52869_d_n5;
        var_alphaje_dn6 = assign39900_e52869_d_n6;
        var_alphaje_dn7 = assign39900_e52869_d_n7;
        var_alphaje_dn8 = assign39900_e52869_d_n8;

        let (assign39910_e52888, assign39910_e52888_d_n5, assign39910_e52888_d_n6, assign39910_e52888_d_n7, assign39910_e52888_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard813 != 0.0)) && (var_guard814 != 0.0)) {
        let assign39910_e52881: f64 = (var_alphaje).ln();
        let assign39910_e52882: f64 = (var_phitd * assign39910_e52881);
        let assign39910_e52885: f64 = (var_v4 - var_v5);
        let assign39910_e52886: f64 = (assign39910_e52882 / assign39910_e52885);
        (assign39910_e52886, ((var_phitd * (var_alphaje_dn5 / var_alphaje)) / assign39910_e52885), ((var_phitd * (var_alphaje_dn6 / var_alphaje)) / assign39910_e52885), ((var_phitd * (var_alphaje_dn7 / var_alphaje)) / assign39910_e52885), ((var_phitd * (var_alphaje_dn8 / var_alphaje)) / assign39910_e52885),)
    } else {
        (var_mfor2_d, var_mfor2_d_dn5, var_mfor2_d_dn6, var_mfor2_d_dn7, var_mfor2_d_dn8,)
    }
};
        var_mfor2_d = assign39910_e52888;
        var_mfor2_d_dn5 = assign39910_e52888_d_n5;
        var_mfor2_d_dn6 = assign39910_e52888_d_n6;
        var_mfor2_d_dn7 = assign39910_e52888_d_n7;
        var_mfor2_d_dn8 = assign39910_e52888_d_n8;

        let (assign39920_e52909, assign39920_e52909_d_n5, assign39920_e52909_d_n6, assign39920_e52909_d_n7, assign39920_e52909_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard813 != 0.0)) && (var_guard814 != 0.0)) {
        let assign39920_e52901: f64 = (var_v4 * var_phitdinv);
        let assign39920_e52903: f64 = (assign39920_e52901 * var_mfor2_d);
        let assign39920_e52904: f64 = (assign39920_e52903).exp();
        let assign39920_e52906: f64 = (assign39920_e52904 - 1.0);
        let assign39920_e52907: f64 = (var_i4_cor / assign39920_e52906);
        (assign39920_e52907, (((var_i4_cor_dn5 * assign39920_e52906) - (var_i4_cor * (assign39920_e52904 * (assign39920_e52901 * var_mfor2_d_dn5)))) / (assign39920_e52906 * assign39920_e52906)), (((var_i4_cor_dn6 * assign39920_e52906) - (var_i4_cor * (assign39920_e52904 * (assign39920_e52901 * var_mfor2_d_dn6)))) / (assign39920_e52906 * assign39920_e52906)), (((var_i4_cor_dn7 * assign39920_e52906) - (var_i4_cor * (assign39920_e52904 * (assign39920_e52901 * var_mfor2_d_dn7)))) / (assign39920_e52906 * assign39920_e52906)), (((var_i4_cor_dn8 * assign39920_e52906) - (var_i4_cor * (assign39920_e52904 * (assign39920_e52901 * var_mfor2_d_dn8)))) / (assign39920_e52906 * assign39920_e52906)),)
    } else {
        (var_isatfor2_d, var_isatfor2_d_dn5, var_isatfor2_d_dn6, var_isatfor2_d_dn7, var_isatfor2_d_dn8,)
    }
};
        var_isatfor2_d = assign39920_e52909;
        var_isatfor2_d_dn5 = assign39920_e52909_d_n5;
        var_isatfor2_d_dn6 = assign39920_e52909_d_n6;
        var_isatfor2_d_dn7 = assign39920_e52909_d_n7;
        var_isatfor2_d_dn8 = assign39920_e52909_d_n8;

        let (assign39930_e52939, assign39930_e52939_d_n5, assign39930_e52939_d_n6, assign39930_e52939_d_n7, assign39930_e52939_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) {
        let assign39930_e52919: f64 = (var_v1 * var_phitdinv);
        let assign39930_e52921: f64 = (assign39930_e52919 * var_mfor1_d);
        let assign39930_e52922: f64 = (assign39930_e52921).exp();
        let assign39930_e52924: f64 = (assign39930_e52922 - 1.0);
        let assign39930_e52925: f64 = (var_isatfor1_d * assign39930_e52924);
        let assign39930_e52926: f64 = (var_i1 - assign39930_e52925);
        let assign39930_e52930: f64 = (var_v1 * var_phitdinv);
        let assign39930_e52932: f64 = (assign39930_e52930 * var_mfor2_d);
        let assign39930_e52933: f64 = (assign39930_e52932).exp();
        let assign39930_e52935: f64 = (assign39930_e52933 - 1.0);
        let assign39930_e52936: f64 = (var_isatfor2_d * assign39930_e52935);
        let assign39930_e52937: f64 = (assign39930_e52926 - assign39930_e52936);
        (assign39930_e52937, (var_i1_dn5 - ((var_isatfor2_d_dn5 * assign39930_e52935) + (var_isatfor2_d * (assign39930_e52933 * (assign39930_e52930 * var_mfor2_d_dn5))))), (var_i1_dn6 - ((var_isatfor2_d_dn6 * assign39930_e52935) + (var_isatfor2_d * (assign39930_e52933 * (assign39930_e52930 * var_mfor2_d_dn6))))), (var_i1_dn7 - ((var_isatfor2_d_dn7 * assign39930_e52935) + (var_isatfor2_d * (assign39930_e52933 * (assign39930_e52930 * var_mfor2_d_dn7))))), (var_i1_dn8 - ((var_isatfor2_d_dn8 * assign39930_e52935) + (var_isatfor2_d * (assign39930_e52933 * (assign39930_e52930 * var_mfor2_d_dn8))))),)
    } else {
        (var_i1_cor, var_i1_cor_dn5, var_i1_cor_dn6, var_i1_cor_dn7, var_i1_cor_dn8,)
    }
};
        var_i1_cor = assign39930_e52939;
        var_i1_cor_dn5 = assign39930_e52939_d_n5;
        var_i1_cor_dn6 = assign39930_e52939_d_n6;
        var_i1_cor_dn7 = assign39930_e52939_d_n7;
        var_i1_cor_dn8 = assign39930_e52939_d_n8;

        let (assign39940_e52969, assign39940_e52969_d_n5, assign39940_e52969_d_n6, assign39940_e52969_d_n7, assign39940_e52969_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) {
        let assign39940_e52949: f64 = (var_v2 * var_phitdinv);
        let assign39940_e52951: f64 = (assign39940_e52949 * var_mfor1_d);
        let assign39940_e52952: f64 = (assign39940_e52951).exp();
        let assign39940_e52954: f64 = (assign39940_e52952 - 1.0);
        let assign39940_e52955: f64 = (var_isatfor1_d * assign39940_e52954);
        let assign39940_e52956: f64 = (var_i2 - assign39940_e52955);
        let assign39940_e52960: f64 = (var_v2 * var_phitdinv);
        let assign39940_e52962: f64 = (assign39940_e52960 * var_mfor2_d);
        let assign39940_e52963: f64 = (assign39940_e52962).exp();
        let assign39940_e52965: f64 = (assign39940_e52963 - 1.0);
        let assign39940_e52966: f64 = (var_isatfor2_d * assign39940_e52965);
        let assign39940_e52967: f64 = (assign39940_e52956 - assign39940_e52966);
        (assign39940_e52967, (var_i2_dn5 - ((var_isatfor2_d_dn5 * assign39940_e52965) + (var_isatfor2_d * (assign39940_e52963 * (assign39940_e52960 * var_mfor2_d_dn5))))), (var_i2_dn6 - ((var_isatfor2_d_dn6 * assign39940_e52965) + (var_isatfor2_d * (assign39940_e52963 * (assign39940_e52960 * var_mfor2_d_dn6))))), (var_i2_dn7 - ((var_isatfor2_d_dn7 * assign39940_e52965) + (var_isatfor2_d * (assign39940_e52963 * (assign39940_e52960 * var_mfor2_d_dn7))))), (var_i2_dn8 - ((var_isatfor2_d_dn8 * assign39940_e52965) + (var_isatfor2_d * (assign39940_e52963 * (assign39940_e52960 * var_mfor2_d_dn8))))),)
    } else {
        (var_i2_cor, var_i2_cor_dn5, var_i2_cor_dn6, var_i2_cor_dn7, var_i2_cor_dn8,)
    }
};
        var_i2_cor = assign39940_e52969;
        var_i2_cor_dn5 = assign39940_e52969_d_n5;
        var_i2_cor_dn6 = assign39940_e52969_d_n6;
        var_i2_cor_dn7 = assign39940_e52969_d_n7;
        var_i2_cor_dn8 = assign39940_e52969_d_n8;

        let (assign39950_e52999, assign39950_e52999_d_n5, assign39950_e52999_d_n6, assign39950_e52999_d_n7, assign39950_e52999_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) {
        let assign39950_e52979: f64 = (var_v3 * var_phitdinv);
        let assign39950_e52981: f64 = (assign39950_e52979 * var_mfor1_d);
        let assign39950_e52982: f64 = (assign39950_e52981).exp();
        let assign39950_e52984: f64 = (assign39950_e52982 - 1.0);
        let assign39950_e52985: f64 = (var_isatfor1_d * assign39950_e52984);
        let assign39950_e52986: f64 = (var_i3 - assign39950_e52985);
        let assign39950_e52990: f64 = (var_v3 * var_phitdinv);
        let assign39950_e52992: f64 = (assign39950_e52990 * var_mfor2_d);
        let assign39950_e52993: f64 = (assign39950_e52992).exp();
        let assign39950_e52995: f64 = (assign39950_e52993 - 1.0);
        let assign39950_e52996: f64 = (var_isatfor2_d * assign39950_e52995);
        let assign39950_e52997: f64 = (assign39950_e52986 - assign39950_e52996);
        (assign39950_e52997, (var_i3_dn5 - ((var_isatfor2_d_dn5 * assign39950_e52995) + (var_isatfor2_d * (assign39950_e52993 * (assign39950_e52990 * var_mfor2_d_dn5))))), (var_i3_dn6 - ((var_isatfor2_d_dn6 * assign39950_e52995) + (var_isatfor2_d * (assign39950_e52993 * (assign39950_e52990 * var_mfor2_d_dn6))))), (var_i3_dn7 - ((var_isatfor2_d_dn7 * assign39950_e52995) + (var_isatfor2_d * (assign39950_e52993 * (assign39950_e52990 * var_mfor2_d_dn7))))), (var_i3_dn8 - ((var_isatfor2_d_dn8 * assign39950_e52995) + (var_isatfor2_d * (assign39950_e52993 * (assign39950_e52990 * var_mfor2_d_dn8))))),)
    } else {
        (var_i3_cor, var_i3_cor_dn5, var_i3_cor_dn6, var_i3_cor_dn7, var_i3_cor_dn8,)
    }
};
        var_i3_cor = assign39950_e52999;
        var_i3_cor_dn5 = assign39950_e52999_d_n5;
        var_i3_cor_dn6 = assign39950_e52999_d_n6;
        var_i3_cor_dn7 = assign39950_e52999_d_n7;
        var_i3_cor_dn8 = assign39950_e52999_d_n8;

        let assign39960_e53010: f64 = if (((var_i1 < 0.0) && (var_i2 < 0.0)) && (var_i3 < 0.0)) { 1.0 } else { 0.0 };
        var_guard815 = assign39960_e53010;

        let assign39970_e53013: f64 = (var_i1_cor / var_i1);
        let assign39970_e53018: f64 = (var_i2_cor / var_i2);
        let assign39970_e53024: f64 = (var_i3_cor / var_i3);
        let assign39970_e53039: f64 = if ((((((assign39970_e53013 > 0.001) || (assign39970_e53018 > 0.001)) || (assign39970_e53024 > 0.001)) && (var_i1_cor < 0.0)) && (var_i2_cor < 0.0)) && (var_i3_cor < 0.0)) { 1.0 } else { 0.0 };
        var_guard816 = assign39970_e53039;

        let (assign39980_e53053, assign39980_e53053_d_n5, assign39980_e53053_d_n6, assign39980_e53053_d_n7, assign39980_e53053_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        let assign39980_e53051: f64 = (var_i1_cor / var_i2_cor);
        (assign39980_e53051, (((var_i1_cor_dn5 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn5)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn6 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn6)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn7 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn7)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn8 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn8)) / (var_i2_cor * var_i2_cor)),)
    } else {
        (var_alphaje, var_alphaje_dn5, var_alphaje_dn6, var_alphaje_dn7, var_alphaje_dn8,)
    }
};
        var_alphaje = assign39980_e53053;
        var_alphaje_dn5 = assign39980_e53053_d_n5;
        var_alphaje_dn6 = assign39980_e53053_d_n6;
        var_alphaje_dn7 = assign39980_e53053_d_n7;
        var_alphaje_dn8 = assign39980_e53053_d_n8;

        let (assign39990_e53073, assign39990_e53073_d_n5, assign39990_e53073_d_n6, assign39990_e53073_d_n7, assign39990_e53073_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        let assign39990_e53064: f64 = (-var_phitd);
        let assign39990_e53066: f64 = (var_alphaje).ln();
        let assign39990_e53067: f64 = (assign39990_e53064 * assign39990_e53066);
        let assign39990_e53070: f64 = (var_v1 - var_v2);
        let assign39990_e53071: f64 = (assign39990_e53067 / assign39990_e53070);
        (assign39990_e53071, ((assign39990_e53064 * (var_alphaje_dn5 / var_alphaje)) / assign39990_e53070), ((assign39990_e53064 * (var_alphaje_dn6 / var_alphaje)) / assign39990_e53070), ((assign39990_e53064 * (var_alphaje_dn7 / var_alphaje)) / assign39990_e53070), ((assign39990_e53064 * (var_alphaje_dn8 / var_alphaje)) / assign39990_e53070),)
    } else {
        (var_m0_rev, var_m0_rev_dn5, var_m0_rev_dn6, var_m0_rev_dn7, var_m0_rev_dn8,)
    }
};
        var_m0_rev = assign39990_e53073;
        var_m0_rev_dn5 = assign39990_e53073_d_n5;
        var_m0_rev_dn6 = assign39990_e53073_d_n6;
        var_m0_rev_dn7 = assign39990_e53073_d_n7;
        var_m0_rev_dn8 = assign39990_e53073_d_n8;

        let (assign40000_e53089,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        let assign40000_e53086: f64 = (var_v2 - var_v1);
        let assign40000_e53087: f64 = (var_v2 / assign40000_e53086);
        (assign40000_e53087,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign40000_e53089;

        let (assign40010_e53111, assign40010_e53111_d_n5, assign40010_e53111_d_n6, assign40010_e53111_d_n7, assign40010_e53111_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        let assign40010_e53102: f64 = (var_alphaje - 1.0);
        let assign40010_e53103: f64 = (var_phitd * assign40010_e53102);
        let assign40010_e53106: f64 = (var_alphaje).powf(var_tt0);
        let assign40010_e53108: f64 = (assign40010_e53106 - 1.0);
        let assign40010_e53109: f64 = (assign40010_e53103 * assign40010_e53108);
        (assign40010_e53109, (((var_phitd * var_alphaje_dn5) * assign40010_e53108) + (assign40010_e53103 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn5)) } } else { (assign40010_e53106 * (var_tt0 * (var_alphaje_dn5 / var_alphaje))) })), (((var_phitd * var_alphaje_dn6) * assign40010_e53108) + (assign40010_e53103 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn6)) } } else { (assign40010_e53106 * (var_tt0 * (var_alphaje_dn6 / var_alphaje))) })), (((var_phitd * var_alphaje_dn7) * assign40010_e53108) + (assign40010_e53103 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn7)) } } else { (assign40010_e53106 * (var_tt0 * (var_alphaje_dn7 / var_alphaje))) })), (((var_phitd * var_alphaje_dn8) * assign40010_e53108) + (assign40010_e53103 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn8)) } } else { (assign40010_e53106 * (var_tt0 * (var_alphaje_dn8 / var_alphaje))) })),)
    } else {
        (var_tt1, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8,)
    }
};
        var_tt1 = assign40010_e53111;
        var_tt1_dn5 = assign40010_e53111_d_n5;
        var_tt1_dn6 = assign40010_e53111_d_n6;
        var_tt1_dn7 = assign40010_e53111_d_n7;
        var_tt1_dn8 = assign40010_e53111_d_n8;

        let (assign40020_e53127,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        let assign40020_e53124: f64 = (var_v1 - var_v2);
        let assign40020_e53125: f64 = (var_v1 / assign40020_e53124);
        (assign40020_e53125,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign40020_e53127;

        let (assign40030_e53151, assign40030_e53151_d_n5, assign40030_e53151_d_n6, assign40030_e53151_d_n7, assign40030_e53151_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        let assign40030_e53139: f64 = (var_alphaje).powf(var_tt0);
        let assign40030_e53142: f64 = (var_v2 - var_v1);
        let assign40030_e53143: f64 = (assign40030_e53139 * assign40030_e53142);
        let assign40030_e53146: f64 = (var_alphaje * var_v1);
        let assign40030_e53147: f64 = (assign40030_e53143 + assign40030_e53146);
        let assign40030_e53149: f64 = (assign40030_e53147 - var_v2);
        (assign40030_e53149, ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn5)) } } else { (assign40030_e53139 * (var_tt0 * (var_alphaje_dn5 / var_alphaje))) } * assign40030_e53142) + (var_alphaje_dn5 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn6)) } } else { (assign40030_e53139 * (var_tt0 * (var_alphaje_dn6 / var_alphaje))) } * assign40030_e53142) + (var_alphaje_dn6 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn7)) } } else { (assign40030_e53139 * (var_tt0 * (var_alphaje_dn7 / var_alphaje))) } * assign40030_e53142) + (var_alphaje_dn7 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn8)) } } else { (assign40030_e53139 * (var_tt0 * (var_alphaje_dn8 / var_alphaje))) } * assign40030_e53142) + (var_alphaje_dn8 * var_v1)),)
    } else {
        (var_tt2, var_tt2_dn5, var_tt2_dn6, var_tt2_dn7, var_tt2_dn8,)
    }
};
        var_tt2 = assign40030_e53151;
        var_tt2_dn5 = assign40030_e53151_d_n5;
        var_tt2_dn6 = assign40030_e53151_d_n6;
        var_tt2_dn7 = assign40030_e53151_d_n7;
        var_tt2_dn8 = assign40030_e53151_d_n8;

        let (assign40040_e53165, assign40040_e53165_d_n5, assign40040_e53165_d_n6, assign40040_e53165_d_n7, assign40040_e53165_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        let assign40040_e53163: f64 = (var_tt1 / var_tt2);
        (assign40040_e53163, (((var_tt1_dn5 * var_tt2) - (var_tt1 * var_tt2_dn5)) / (var_tt2 * var_tt2)), (((var_tt1_dn6 * var_tt2) - (var_tt1 * var_tt2_dn6)) / (var_tt2 * var_tt2)), (((var_tt1_dn7 * var_tt2) - (var_tt1 * var_tt2_dn7)) / (var_tt2 * var_tt2)), (((var_tt1_dn8 * var_tt2) - (var_tt1 * var_tt2_dn8)) / (var_tt2 * var_tt2)),)
    } else {
        (var_mcor_rev, var_mcor_rev_dn5, var_mcor_rev_dn6, var_mcor_rev_dn7, var_mcor_rev_dn8,)
    }
};
        var_mcor_rev = assign40040_e53165;
        var_mcor_rev_dn5 = assign40040_e53165_d_n5;
        var_mcor_rev_dn6 = assign40040_e53165_d_n6;
        var_mcor_rev_dn7 = assign40040_e53165_d_n7;
        var_mcor_rev_dn8 = assign40040_e53165_d_n8;

        let (assign40050_e53179, assign40050_e53179_d_n5, assign40050_e53179_d_n6, assign40050_e53179_d_n7, assign40050_e53179_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        let assign40050_e53177: f64 = (var_m0_rev + var_mcor_rev);
        (assign40050_e53177, (var_m0_rev_dn5 + var_mcor_rev_dn5), (var_m0_rev_dn6 + var_mcor_rev_dn6), (var_m0_rev_dn7 + var_mcor_rev_dn7), (var_m0_rev_dn8 + var_mcor_rev_dn8),)
    } else {
        (var_mrev_d, var_mrev_d_dn5, var_mrev_d_dn6, var_mrev_d_dn7, var_mrev_d_dn8,)
    }
};
        var_mrev_d = assign40050_e53179;
        var_mrev_d_dn5 = assign40050_e53179_d_n5;
        var_mrev_d_dn6 = assign40050_e53179_d_n6;
        var_mrev_d_dn7 = assign40050_e53179_d_n7;
        var_mrev_d_dn8 = assign40050_e53179_d_n8;

        let assign40060_e53182: f64 = (var_v3 * var_phitdinv);
        let assign40060_e53184: f64 = (assign40060_e53182 * var_mrev_d);
        let assign40060_e53185: f64 = (assign40060_e53184).abs();
        let assign40060_e53187: f64 = if assign40060_e53185 < 1e-6 { 1.0 } else { 0.0 };
        var_guard817 = assign40060_e53187;

        let (assign40070_e53201,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) && (var_guard817 != 0.0)) {
        (1.0,)
    } else {
        (var_m0flag_d,)
    }
};
        var_m0flag_d = assign40070_e53201;

        let (assign40080_e53225, assign40080_e53225_d_n5, assign40080_e53225_d_n6, assign40080_e53225_d_n7, assign40080_e53225_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) && (var_guard817 != 0.0)) {
        let assign40080_e53216: f64 = (1.0 / var_v3);
        let assign40080_e53219: f64 = (0.5 * var_phitdinv);
        let assign40080_e53221: f64 = (assign40080_e53219 * var_mrev_d);
        let assign40080_e53222: f64 = (assign40080_e53216 + assign40080_e53221);
        let assign40080_e53223: f64 = (var_i3_cor * assign40080_e53222);
        (assign40080_e53223, ((var_i3_cor_dn5 * assign40080_e53222) + (var_i3_cor * (assign40080_e53219 * var_mrev_d_dn5))), ((var_i3_cor_dn6 * assign40080_e53222) + (var_i3_cor * (assign40080_e53219 * var_mrev_d_dn6))), ((var_i3_cor_dn7 * assign40080_e53222) + (var_i3_cor * (assign40080_e53219 * var_mrev_d_dn7))), ((var_i3_cor_dn8 * assign40080_e53222) + (var_i3_cor * (assign40080_e53219 * var_mrev_d_dn8))),)
    } else {
        (var_isatrev_d, var_isatrev_d_dn5, var_isatrev_d_dn6, var_isatrev_d_dn7, var_isatrev_d_dn8,)
    }
};
        var_isatrev_d = assign40080_e53225;
        var_isatrev_d_dn5 = assign40080_e53225_d_n5;
        var_isatrev_d_dn6 = assign40080_e53225_d_n6;
        var_isatrev_d_dn7 = assign40080_e53225_d_n7;
        var_isatrev_d_dn8 = assign40080_e53225_d_n8;

        let (assign40090_e53248, assign40090_e53248_d_n5, assign40090_e53248_d_n6, assign40090_e53248_d_n7, assign40090_e53248_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) && (var_guard817 != 0.0)) {
        let assign40090_e53238: f64 = (-0.5);
        let assign40090_e53240: f64 = (assign40090_e53238 * var_i3_cor);
        let assign40090_e53242: f64 = (assign40090_e53240 * var_mrev_d);
        let assign40090_e53244: f64 = (assign40090_e53242 * var_phitdinv);
        let assign40090_e53246: f64 = (assign40090_e53244 / var_v3);
        (assign40090_e53246, (((((assign40090_e53238 * var_i3_cor_dn5) * var_mrev_d) + (assign40090_e53240 * var_mrev_d_dn5)) * var_phitdinv) / var_v3), (((((assign40090_e53238 * var_i3_cor_dn6) * var_mrev_d) + (assign40090_e53240 * var_mrev_d_dn6)) * var_phitdinv) / var_v3), (((((assign40090_e53238 * var_i3_cor_dn7) * var_mrev_d) + (assign40090_e53240 * var_mrev_d_dn7)) * var_phitdinv) / var_v3), (((((assign40090_e53238 * var_i3_cor_dn8) * var_mrev_d) + (assign40090_e53240 * var_mrev_d_dn8)) * var_phitdinv) / var_v3),)
    } else {
        (var_mrev_d, var_mrev_d_dn5, var_mrev_d_dn6, var_mrev_d_dn7, var_mrev_d_dn8,)
    }
};
        var_mrev_d = assign40090_e53248;
        var_mrev_d_dn5 = assign40090_e53248_d_n5;
        var_mrev_d_dn6 = assign40090_e53248_d_n6;
        var_mrev_d_dn7 = assign40090_e53248_d_n7;
        var_mrev_d_dn8 = assign40090_e53248_d_n8;

        let (assign40100_e53263,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) && (var_guard817 == 0.0)) {
        (0.0,)
    } else {
        (var_m0flag_d,)
    }
};
        var_m0flag_d = assign40100_e53263;

        let (assign40110_e53289, assign40110_e53289_d_n5, assign40110_e53289_d_n6, assign40110_e53289_d_n7, assign40110_e53289_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard812 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) && (var_guard817 == 0.0)) {
        let assign40110_e53277: f64 = (-var_i3_cor);
        let assign40110_e53279: f64 = (-var_v3);
        let assign40110_e53281: f64 = (assign40110_e53279 * var_phitdinv);
        let assign40110_e53283: f64 = (assign40110_e53281 * var_mrev_d);
        let assign40110_e53284: f64 = (assign40110_e53283).exp();
        let assign40110_e53286: f64 = (assign40110_e53284 - 1.0);
        let assign40110_e53287: f64 = (assign40110_e53277 / assign40110_e53286);
        (assign40110_e53287, ((((-var_i3_cor_dn5) * assign40110_e53286) - (assign40110_e53277 * (assign40110_e53284 * (assign40110_e53281 * var_mrev_d_dn5)))) / (assign40110_e53286 * assign40110_e53286)), ((((-var_i3_cor_dn6) * assign40110_e53286) - (assign40110_e53277 * (assign40110_e53284 * (assign40110_e53281 * var_mrev_d_dn6)))) / (assign40110_e53286 * assign40110_e53286)), ((((-var_i3_cor_dn7) * assign40110_e53286) - (assign40110_e53277 * (assign40110_e53284 * (assign40110_e53281 * var_mrev_d_dn7)))) / (assign40110_e53286 * assign40110_e53286)), ((((-var_i3_cor_dn8) * assign40110_e53286) - (assign40110_e53277 * (assign40110_e53284 * (assign40110_e53281 * var_mrev_d_dn8)))) / (assign40110_e53286 * assign40110_e53286)),)
    } else {
        (var_isatrev_d, var_isatrev_d_dn5, var_isatrev_d_dn6, var_isatrev_d_dn7, var_isatrev_d_dn8,)
    }
};
        var_isatrev_d = assign40110_e53289;
        var_isatrev_d_dn5 = assign40110_e53289_d_n5;
        var_isatrev_d_dn6 = assign40110_e53289_d_n6;
        var_isatrev_d_dn7 = assign40110_e53289_d_n7;
        var_isatrev_d_dn8 = assign40110_e53289_d_n8;

        let (assign40120_e53307,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign40120_e53296: f64 = (var_abdrain_i * var_cjobot_d);
        let assign40120_e53299: f64 = (var_lsdrain_i * var_cjosti_d);
        let assign40120_e53300: f64 = (assign40120_e53296 + assign40120_e53299);
        let assign40120_e53303: f64 = (var_lgdrain_i * var_cjogat_d);
        let assign40120_e53304: f64 = (assign40120_e53300 + assign40120_e53303);
        let assign40120_e53305: f64 = (var_fjunqd_i * assign40120_e53304);
        (assign40120_e53305,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign40120_e53307;

        let assign40130_e53310: f64 = (var_abdrain_i * var_cjobot_d);
        let assign40130_e53312: f64 = if assign40130_e53310 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard818 = assign40130_e53312;

        let (assign40140_e53320,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard818 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_d,)
    }
};
        var_zflagbot_d = assign40140_e53320;

        let assign40150_e53323: f64 = (var_lsdrain_i * var_cjosti_d);
        let assign40150_e53325: f64 = if assign40150_e53323 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard819 = assign40150_e53325;

        let (assign40160_e53333,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard819 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_d,)
    }
};
        var_zflagsti_d = assign40160_e53333;

        let assign40170_e53336: f64 = (var_lgdrain_i * var_cjogat_d);
        let assign40170_e53338: f64 = if assign40170_e53336 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard820 = assign40170_e53338;

        let (assign40180_e53346,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard820 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_d,)
    }
};
        var_zflaggat_d = assign40180_e53346;

        let assign40190_e53358: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard821 = assign40190_e53358;

        let (assign40200_e53373,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard821 != 0.0)) {
        let assign40200_e53366: f64 = (0.5 * p.p815);
        let assign40200_e53369: f64 = (var_isatfor1_d + 1e-21);
        let assign40200_e53370: f64 = (assign40200_e53366 / assign40200_e53369);
        let assign40200_e53371: f64 = (assign40200_e53370).ln();
        (assign40200_e53371,)
    } else {
        (var_xhighf1_d,)
    }
};
        var_xhighf1_d = assign40200_e53373;

        let (assign40210_e53388, assign40210_e53388_d_n5, assign40210_e53388_d_n6, assign40210_e53388_d_n7, assign40210_e53388_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard821 != 0.0)) {
        let assign40210_e53381: f64 = (0.5 * p.p815);
        let assign40210_e53384: f64 = (var_isatfor2_d + 1e-21);
        let assign40210_e53385: f64 = (assign40210_e53381 / assign40210_e53384);
        let assign40210_e53386: f64 = (assign40210_e53385).ln();
        (assign40210_e53386, ((-((assign40210_e53381 * var_isatfor2_d_dn5) / (assign40210_e53384 * assign40210_e53384))) / assign40210_e53385), ((-((assign40210_e53381 * var_isatfor2_d_dn6) / (assign40210_e53384 * assign40210_e53384))) / assign40210_e53385), ((-((assign40210_e53381 * var_isatfor2_d_dn7) / (assign40210_e53384 * assign40210_e53384))) / assign40210_e53385), ((-((assign40210_e53381 * var_isatfor2_d_dn8) / (assign40210_e53384 * assign40210_e53384))) / assign40210_e53385),)
    } else {
        (var_xhighf2_d, var_xhighf2_d_dn5, var_xhighf2_d_dn6, var_xhighf2_d_dn7, var_xhighf2_d_dn8,)
    }
};
        var_xhighf2_d = assign40210_e53388;
        var_xhighf2_d_dn5 = assign40210_e53388_d_n5;
        var_xhighf2_d_dn6 = assign40210_e53388_d_n6;
        var_xhighf2_d_dn7 = assign40210_e53388_d_n7;
        var_xhighf2_d_dn8 = assign40210_e53388_d_n8;

        let (assign40220_e53404, assign40220_e53404_d_n5, assign40220_e53404_d_n6, assign40220_e53404_d_n7, assign40220_e53404_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard821 != 0.0)) {
        let assign40220_e53396: f64 = (0.5 * p.p815);
        let assign40220_e53398: f64 = (var_isatrev_d).abs();
        let assign40220_e53400: f64 = (assign40220_e53398 + 1e-21);
        let assign40220_e53401: f64 = (assign40220_e53396 / assign40220_e53400);
        let assign40220_e53402: f64 = (assign40220_e53401).ln();
        (assign40220_e53402, ((-((assign40220_e53396 * if var_isatrev_d >= 0.0 { var_isatrev_d_dn5 } else { (-var_isatrev_d_dn5) }) / (assign40220_e53400 * assign40220_e53400))) / assign40220_e53401), ((-((assign40220_e53396 * if var_isatrev_d >= 0.0 { var_isatrev_d_dn6 } else { (-var_isatrev_d_dn6) }) / (assign40220_e53400 * assign40220_e53400))) / assign40220_e53401), ((-((assign40220_e53396 * if var_isatrev_d >= 0.0 { var_isatrev_d_dn7 } else { (-var_isatrev_d_dn7) }) / (assign40220_e53400 * assign40220_e53400))) / assign40220_e53401), ((-((assign40220_e53396 * if var_isatrev_d >= 0.0 { var_isatrev_d_dn8 } else { (-var_isatrev_d_dn8) }) / (assign40220_e53400 * assign40220_e53400))) / assign40220_e53401),)
    } else {
        (var_xhighr_d, var_xhighr_d_dn5, var_xhighr_d_dn6, var_xhighr_d_dn7, var_xhighr_d_dn8,)
    }
};
        var_xhighr_d = assign40220_e53404;
        var_xhighr_d_dn5 = assign40220_e53404_d_n5;
        var_xhighr_d_dn6 = assign40220_e53404_d_n6;
        var_xhighr_d_dn7 = assign40220_e53404_d_n7;
        var_xhighr_d_dn8 = assign40220_e53404_d_n8;

        let (assign40230_e53412,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign40230_e53410: f64 = (var_xhighf1_d).min(230.25850929940458);
        (assign40230_e53410,)
    } else {
        (var_xhighf1_d,)
    }
};
        var_xhighf1_d = assign40230_e53412;

        let (assign40240_e53419,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign40240_e53417: f64 = (var_xhighf1_d).exp();
        (assign40240_e53417,)
    } else {
        (var_expxhf1_d,)
    }
};
        var_expxhf1_d = assign40240_e53419;

        *var_alphaje_slot = var_alphaje;
        *var_alphaje_dn5_slot = var_alphaje_dn5;
        *var_alphaje_dn6_slot = var_alphaje_dn6;
        *var_alphaje_dn7_slot = var_alphaje_dn7;
        *var_alphaje_dn8_slot = var_alphaje_dn8;
        *var_expxhf1_d_slot = var_expxhf1_d;
        *var_guard812_slot = var_guard812;
        *var_guard813_slot = var_guard813;
        *var_guard814_slot = var_guard814;
        *var_guard815_slot = var_guard815;
        *var_guard816_slot = var_guard816;
        *var_guard817_slot = var_guard817;
        *var_guard818_slot = var_guard818;
        *var_guard819_slot = var_guard819;
        *var_guard820_slot = var_guard820;
        *var_guard821_slot = var_guard821;
        *var_i1_cor_slot = var_i1_cor;
        *var_i1_cor_dn5_slot = var_i1_cor_dn5;
        *var_i1_cor_dn6_slot = var_i1_cor_dn6;
        *var_i1_cor_dn7_slot = var_i1_cor_dn7;
        *var_i1_cor_dn8_slot = var_i1_cor_dn8;
        *var_i2_cor_slot = var_i2_cor;
        *var_i2_cor_dn5_slot = var_i2_cor_dn5;
        *var_i2_cor_dn6_slot = var_i2_cor_dn6;
        *var_i2_cor_dn7_slot = var_i2_cor_dn7;
        *var_i2_cor_dn8_slot = var_i2_cor_dn8;
        *var_i3_cor_slot = var_i3_cor;
        *var_i3_cor_dn5_slot = var_i3_cor_dn5;
        *var_i3_cor_dn6_slot = var_i3_cor_dn6;
        *var_i3_cor_dn7_slot = var_i3_cor_dn7;
        *var_i3_cor_dn8_slot = var_i3_cor_dn8;
        *var_i4_cor_slot = var_i4_cor;
        *var_i4_cor_dn5_slot = var_i4_cor_dn5;
        *var_i4_cor_dn6_slot = var_i4_cor_dn6;
        *var_i4_cor_dn7_slot = var_i4_cor_dn7;
        *var_i4_cor_dn8_slot = var_i4_cor_dn8;
        *var_i5_cor_slot = var_i5_cor;
        *var_i5_cor_dn5_slot = var_i5_cor_dn5;
        *var_i5_cor_dn6_slot = var_i5_cor_dn6;
        *var_i5_cor_dn7_slot = var_i5_cor_dn7;
        *var_i5_cor_dn8_slot = var_i5_cor_dn8;
        *var_isatfor2_d_slot = var_isatfor2_d;
        *var_isatfor2_d_dn5_slot = var_isatfor2_d_dn5;
        *var_isatfor2_d_dn6_slot = var_isatfor2_d_dn6;
        *var_isatfor2_d_dn7_slot = var_isatfor2_d_dn7;
        *var_isatfor2_d_dn8_slot = var_isatfor2_d_dn8;
        *var_isatrev_d_slot = var_isatrev_d;
        *var_isatrev_d_dn5_slot = var_isatrev_d_dn5;
        *var_isatrev_d_dn6_slot = var_isatrev_d_dn6;
        *var_isatrev_d_dn7_slot = var_isatrev_d_dn7;
        *var_isatrev_d_dn8_slot = var_isatrev_d_dn8;
        *var_m0_rev_slot = var_m0_rev;
        *var_m0_rev_dn5_slot = var_m0_rev_dn5;
        *var_m0_rev_dn6_slot = var_m0_rev_dn6;
        *var_m0_rev_dn7_slot = var_m0_rev_dn7;
        *var_m0_rev_dn8_slot = var_m0_rev_dn8;
        *var_m0flag_d_slot = var_m0flag_d;
        *var_mcor_rev_slot = var_mcor_rev;
        *var_mcor_rev_dn5_slot = var_mcor_rev_dn5;
        *var_mcor_rev_dn6_slot = var_mcor_rev_dn6;
        *var_mcor_rev_dn7_slot = var_mcor_rev_dn7;
        *var_mcor_rev_dn8_slot = var_mcor_rev_dn8;
        *var_mfor2_d_slot = var_mfor2_d;
        *var_mfor2_d_dn5_slot = var_mfor2_d_dn5;
        *var_mfor2_d_dn6_slot = var_mfor2_d_dn6;
        *var_mfor2_d_dn7_slot = var_mfor2_d_dn7;
        *var_mfor2_d_dn8_slot = var_mfor2_d_dn8;
        *var_mrev_d_slot = var_mrev_d;
        *var_mrev_d_dn5_slot = var_mrev_d_dn5;
        *var_mrev_d_dn6_slot = var_mrev_d_dn6;
        *var_mrev_d_dn7_slot = var_mrev_d_dn7;
        *var_mrev_d_dn8_slot = var_mrev_d_dn8;
        *var_tt0_slot = var_tt0;
        *var_tt1_slot = var_tt1;
        *var_tt1_dn5_slot = var_tt1_dn5;
        *var_tt1_dn6_slot = var_tt1_dn6;
        *var_tt1_dn7_slot = var_tt1_dn7;
        *var_tt1_dn8_slot = var_tt1_dn8;
        *var_tt2_slot = var_tt2;
        *var_tt2_dn5_slot = var_tt2_dn5;
        *var_tt2_dn6_slot = var_tt2_dn6;
        *var_tt2_dn7_slot = var_tt2_dn7;
        *var_tt2_dn8_slot = var_tt2_dn8;
        *var_xhighf1_d_slot = var_xhighf1_d;
        *var_xhighf2_d_slot = var_xhighf2_d;
        *var_xhighf2_d_dn5_slot = var_xhighf2_d_dn5;
        *var_xhighf2_d_dn6_slot = var_xhighf2_d_dn6;
        *var_xhighf2_d_dn7_slot = var_xhighf2_d_dn7;
        *var_xhighf2_d_dn8_slot = var_xhighf2_d_dn8;
        *var_xhighr_d_slot = var_xhighr_d;
        *var_xhighr_d_dn5_slot = var_xhighr_d_dn5;
        *var_xhighr_d_dn6_slot = var_xhighr_d_dn6;
        *var_xhighr_d_dn7_slot = var_xhighr_d_dn7;
        *var_xhighr_d_dn8_slot = var_xhighr_d_dn8;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zfrac_slot = var_zfrac;
    }

    pub(super) fn stamp_transient_block_86(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_aphi_dc: f64,
        var_ar: f64,
        var_bphi_dc: f64,
        var_chnl_type: f64,
        var_g_0_dc: f64,
        var_gfacnud_i: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_inv_phita: f64,
        var_phib_dc: f64,
        var_phix1_dc: f64,
        var_phix_dc: f64,
        var_sqrt_phib_dc: f64,
        var_thesat_t: f64,
        var_us1: f64,
        var_us21: f64,
        var_vfb_t: f64,
        var_aphi_slot: &mut f64,
        var_arloc_slot: &mut f64,
        var_dvbstar_slot: &mut f64,
        var_dvbstar_dc_slot: &mut f64,
        var_dvbstar_dc_dn5_slot: &mut f64,
        var_dvbstar_dc_dn6_slot: &mut f64,
        var_dvbstar_dc_dn7_slot: &mut f64,
        var_dvbstar_dc_dn8_slot: &mut f64,
        var_dvbstar_dn5_slot: &mut f64,
        var_dvbstar_dn6_slot: &mut f64,
        var_dvbstar_dn7_slot: &mut f64,
        var_dvbstar_dn8_slot: &mut f64,
        var_expxhf2_d_slot: &mut f64,
        var_expxhf2_d_dn5_slot: &mut f64,
        var_expxhf2_d_dn6_slot: &mut f64,
        var_expxhf2_d_dn7_slot: &mut f64,
        var_expxhf2_d_dn8_slot: &mut f64,
        var_expxhr_d_slot: &mut f64,
        var_expxhr_d_dn5_slot: &mut f64,
        var_expxhr_d_dn6_slot: &mut f64,
        var_expxhr_d_dn7_slot: &mut f64,
        var_expxhr_d_dn8_slot: &mut f64,
        var_g_0_slot: &mut f64,
        var_guard1011_slot: &mut f64,
        var_guard1012_slot: &mut f64,
        var_guard1172_slot: &mut f64,
        var_phib_slot: &mut f64,
        var_sigvds_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_thesatloc_slot: &mut f64,
        var_us_slot: &mut f64,
        var_us_dn5_slot: &mut f64,
        var_us_dn6_slot: &mut f64,
        var_us_dn7_slot: &mut f64,
        var_us_dn8_slot: &mut f64,
        var_usnew_slot: &mut f64,
        var_usnew_dn5_slot: &mut f64,
        var_usnew_dn6_slot: &mut f64,
        var_usnew_dn7_slot: &mut f64,
        var_usnew_dn8_slot: &mut f64,
        var_v_db_slot: &mut f64,
        var_v_db_dn6_slot: &mut f64,
        var_v_db_dn7_slot: &mut f64,
        var_v_db_dn8_slot: &mut f64,
        var_v_ds_slot: &mut f64,
        var_v_ds_dn6_slot: &mut f64,
        var_v_ds_dn7_slot: &mut f64,
        var_v_gs_slot: &mut f64,
        var_v_gs_dn5_slot: &mut f64,
        var_v_gs_dn6_slot: &mut f64,
        var_v_gs_dn7_slot: &mut f64,
        var_v_sb_slot: &mut f64,
        var_v_sb_dn6_slot: &mut f64,
        var_v_sb_dn7_slot: &mut f64,
        var_v_sb_dn8_slot: &mut f64,
        var_v_xb_slot: &mut f64,
        var_v_xb_dc_tmp_slot: &mut f64,
        var_v_xb_dc_tmp_dn6_slot: &mut f64,
        var_v_xb_dc_tmp_dn7_slot: &mut f64,
        var_v_xb_dc_tmp_dn8_slot: &mut f64,
        var_v_xb_dn6_slot: &mut f64,
        var_v_xb_dn7_slot: &mut f64,
        var_v_xb_dn8_slot: &mut f64,
        var_vdbprime_slot: &mut f64,
        var_vdbprime_dn6_slot: &mut f64,
        var_vdbprime_dn7_slot: &mut f64,
        var_vdbprime_dn8_slot: &mut f64,
        var_vdsx_slot: &mut f64,
        var_vdsx_dn6_slot: &mut f64,
        var_vdsx_dn7_slot: &mut f64,
        var_vgb_slot: &mut f64,
        var_vgb_dn5_slot: &mut f64,
        var_vgb_dn6_slot: &mut f64,
        var_vgb_dn7_slot: &mut f64,
        var_vgb_dn8_slot: &mut f64,
        var_vgdprime_slot: &mut f64,
        var_vgdprime_dn5_slot: &mut f64,
        var_vgdprime_dn6_slot: &mut f64,
        var_vgdprime_dn7_slot: &mut f64,
        var_vgsprime_slot: &mut f64,
        var_vgsprime_dn5_slot: &mut f64,
        var_vgsprime_dn6_slot: &mut f64,
        var_vgsprime_dn7_slot: &mut f64,
        var_vjun_d_slot: &mut f64,
        var_vjun_d_dn11_slot: &mut f64,
        var_vjun_d_dn7_slot: &mut f64,
        var_vjun_s_slot: &mut f64,
        var_vjun_s_dn10_slot: &mut f64,
        var_vjun_s_dn6_slot: &mut f64,
        var_vmb_slot: &mut f64,
        var_vmb_dn5_slot: &mut f64,
        var_vmb_dn6_slot: &mut f64,
        var_vmb_dn7_slot: &mut f64,
        var_vmb_dn8_slot: &mut f64,
        var_vmbnew_slot: &mut f64,
        var_vmbnew_dn5_slot: &mut f64,
        var_vmbnew_dn6_slot: &mut f64,
        var_vmbnew_dn7_slot: &mut f64,
        var_vmbnew_dn8_slot: &mut f64,
        var_vsbprime_slot: &mut f64,
        var_vsbprime_dn6_slot: &mut f64,
        var_vsbprime_dn7_slot: &mut f64,
        var_vsbprime_dn8_slot: &mut f64,
        var_vsbstar_slot: &mut f64,
        var_vsbstar_dc_slot: &mut f64,
        var_vsbstar_dc_dn5_slot: &mut f64,
        var_vsbstar_dc_dn6_slot: &mut f64,
        var_vsbstar_dc_dn7_slot: &mut f64,
        var_vsbstar_dc_dn8_slot: &mut f64,
        var_vsbstar_dc_tmp_slot: &mut f64,
        var_vsbstar_dc_tmp_dn5_slot: &mut f64,
        var_vsbstar_dc_tmp_dn6_slot: &mut f64,
        var_vsbstar_dc_tmp_dn7_slot: &mut f64,
        var_vsbstar_dc_tmp_dn8_slot: &mut f64,
        var_vsbstar_dn5_slot: &mut f64,
        var_vsbstar_dn6_slot: &mut f64,
        var_vsbstar_dn7_slot: &mut f64,
        var_vsbstar_dn8_slot: &mut f64,
        var_xgb_ov_slot: &mut f64,
        var_xgb_ov_dn5_slot: &mut f64,
        var_xgb_ov_dn6_slot: &mut f64,
        var_xgb_ov_dn7_slot: &mut f64,
        var_xgb_ov_dn8_slot: &mut f64,
        var_xgd_ov_slot: &mut f64,
        var_xgd_ov_dn5_slot: &mut f64,
        var_xgd_ov_dn6_slot: &mut f64,
        var_xgd_ov_dn7_slot: &mut f64,
        var_xgs_ov_slot: &mut f64,
        var_xgs_ov_dn5_slot: &mut f64,
        var_xgs_ov_dn6_slot: &mut f64,
        var_xgs_ov_dn7_slot: &mut f64,
        var_xhighf2_d_slot: &mut f64,
        var_xhighf2_d_dn5_slot: &mut f64,
        var_xhighf2_d_dn6_slot: &mut f64,
        var_xhighf2_d_dn7_slot: &mut f64,
        var_xhighf2_d_dn8_slot: &mut f64,
        var_xhighr_d_slot: &mut f64,
        var_xhighr_d_dn5_slot: &mut f64,
        var_xhighr_d_dn6_slot: &mut f64,
        var_xhighr_d_dn7_slot: &mut f64,
        var_xhighr_d_dn8_slot: &mut f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let mut var_aphi: f64 = *var_aphi_slot;
        let mut var_arloc: f64 = *var_arloc_slot;
        let mut var_dvbstar: f64 = *var_dvbstar_slot;
        let mut var_dvbstar_dc: f64 = *var_dvbstar_dc_slot;
        let mut var_dvbstar_dc_dn5: f64 = *var_dvbstar_dc_dn5_slot;
        let mut var_dvbstar_dc_dn6: f64 = *var_dvbstar_dc_dn6_slot;
        let mut var_dvbstar_dc_dn7: f64 = *var_dvbstar_dc_dn7_slot;
        let mut var_dvbstar_dc_dn8: f64 = *var_dvbstar_dc_dn8_slot;
        let mut var_dvbstar_dn5: f64 = *var_dvbstar_dn5_slot;
        let mut var_dvbstar_dn6: f64 = *var_dvbstar_dn6_slot;
        let mut var_dvbstar_dn7: f64 = *var_dvbstar_dn7_slot;
        let mut var_dvbstar_dn8: f64 = *var_dvbstar_dn8_slot;
        let mut var_expxhf2_d: f64 = *var_expxhf2_d_slot;
        let mut var_expxhf2_d_dn5: f64 = *var_expxhf2_d_dn5_slot;
        let mut var_expxhf2_d_dn6: f64 = *var_expxhf2_d_dn6_slot;
        let mut var_expxhf2_d_dn7: f64 = *var_expxhf2_d_dn7_slot;
        let mut var_expxhf2_d_dn8: f64 = *var_expxhf2_d_dn8_slot;
        let mut var_expxhr_d: f64 = *var_expxhr_d_slot;
        let mut var_expxhr_d_dn5: f64 = *var_expxhr_d_dn5_slot;
        let mut var_expxhr_d_dn6: f64 = *var_expxhr_d_dn6_slot;
        let mut var_expxhr_d_dn7: f64 = *var_expxhr_d_dn7_slot;
        let mut var_expxhr_d_dn8: f64 = *var_expxhr_d_dn8_slot;
        let mut var_g_0: f64 = *var_g_0_slot;
        let mut var_guard1011: f64 = *var_guard1011_slot;
        let mut var_guard1012: f64 = *var_guard1012_slot;
        let mut var_guard1172: f64 = *var_guard1172_slot;
        let mut var_phib: f64 = *var_phib_slot;
        let mut var_sigvds: f64 = *var_sigvds_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_thesatloc: f64 = *var_thesatloc_slot;
        let mut var_us: f64 = *var_us_slot;
        let mut var_us_dn5: f64 = *var_us_dn5_slot;
        let mut var_us_dn6: f64 = *var_us_dn6_slot;
        let mut var_us_dn7: f64 = *var_us_dn7_slot;
        let mut var_us_dn8: f64 = *var_us_dn8_slot;
        let mut var_usnew: f64 = *var_usnew_slot;
        let mut var_usnew_dn5: f64 = *var_usnew_dn5_slot;
        let mut var_usnew_dn6: f64 = *var_usnew_dn6_slot;
        let mut var_usnew_dn7: f64 = *var_usnew_dn7_slot;
        let mut var_usnew_dn8: f64 = *var_usnew_dn8_slot;
        let mut var_v_db: f64 = *var_v_db_slot;
        let mut var_v_db_dn6: f64 = *var_v_db_dn6_slot;
        let mut var_v_db_dn7: f64 = *var_v_db_dn7_slot;
        let mut var_v_db_dn8: f64 = *var_v_db_dn8_slot;
        let mut var_v_ds: f64 = *var_v_ds_slot;
        let mut var_v_ds_dn6: f64 = *var_v_ds_dn6_slot;
        let mut var_v_ds_dn7: f64 = *var_v_ds_dn7_slot;
        let mut var_v_gs: f64 = *var_v_gs_slot;
        let mut var_v_gs_dn5: f64 = *var_v_gs_dn5_slot;
        let mut var_v_gs_dn6: f64 = *var_v_gs_dn6_slot;
        let mut var_v_gs_dn7: f64 = *var_v_gs_dn7_slot;
        let mut var_v_sb: f64 = *var_v_sb_slot;
        let mut var_v_sb_dn6: f64 = *var_v_sb_dn6_slot;
        let mut var_v_sb_dn7: f64 = *var_v_sb_dn7_slot;
        let mut var_v_sb_dn8: f64 = *var_v_sb_dn8_slot;
        let mut var_v_xb: f64 = *var_v_xb_slot;
        let mut var_v_xb_dc_tmp: f64 = *var_v_xb_dc_tmp_slot;
        let mut var_v_xb_dc_tmp_dn6: f64 = *var_v_xb_dc_tmp_dn6_slot;
        let mut var_v_xb_dc_tmp_dn7: f64 = *var_v_xb_dc_tmp_dn7_slot;
        let mut var_v_xb_dc_tmp_dn8: f64 = *var_v_xb_dc_tmp_dn8_slot;
        let mut var_v_xb_dn6: f64 = *var_v_xb_dn6_slot;
        let mut var_v_xb_dn7: f64 = *var_v_xb_dn7_slot;
        let mut var_v_xb_dn8: f64 = *var_v_xb_dn8_slot;
        let mut var_vdbprime: f64 = *var_vdbprime_slot;
        let mut var_vdbprime_dn6: f64 = *var_vdbprime_dn6_slot;
        let mut var_vdbprime_dn7: f64 = *var_vdbprime_dn7_slot;
        let mut var_vdbprime_dn8: f64 = *var_vdbprime_dn8_slot;
        let mut var_vdsx: f64 = *var_vdsx_slot;
        let mut var_vdsx_dn6: f64 = *var_vdsx_dn6_slot;
        let mut var_vdsx_dn7: f64 = *var_vdsx_dn7_slot;
        let mut var_vgb: f64 = *var_vgb_slot;
        let mut var_vgb_dn5: f64 = *var_vgb_dn5_slot;
        let mut var_vgb_dn6: f64 = *var_vgb_dn6_slot;
        let mut var_vgb_dn7: f64 = *var_vgb_dn7_slot;
        let mut var_vgb_dn8: f64 = *var_vgb_dn8_slot;
        let mut var_vgdprime: f64 = *var_vgdprime_slot;
        let mut var_vgdprime_dn5: f64 = *var_vgdprime_dn5_slot;
        let mut var_vgdprime_dn6: f64 = *var_vgdprime_dn6_slot;
        let mut var_vgdprime_dn7: f64 = *var_vgdprime_dn7_slot;
        let mut var_vgsprime: f64 = *var_vgsprime_slot;
        let mut var_vgsprime_dn5: f64 = *var_vgsprime_dn5_slot;
        let mut var_vgsprime_dn6: f64 = *var_vgsprime_dn6_slot;
        let mut var_vgsprime_dn7: f64 = *var_vgsprime_dn7_slot;
        let mut var_vjun_d: f64 = *var_vjun_d_slot;
        let mut var_vjun_d_dn11: f64 = *var_vjun_d_dn11_slot;
        let mut var_vjun_d_dn7: f64 = *var_vjun_d_dn7_slot;
        let mut var_vjun_s: f64 = *var_vjun_s_slot;
        let mut var_vjun_s_dn10: f64 = *var_vjun_s_dn10_slot;
        let mut var_vjun_s_dn6: f64 = *var_vjun_s_dn6_slot;
        let mut var_vmb: f64 = *var_vmb_slot;
        let mut var_vmb_dn5: f64 = *var_vmb_dn5_slot;
        let mut var_vmb_dn6: f64 = *var_vmb_dn6_slot;
        let mut var_vmb_dn7: f64 = *var_vmb_dn7_slot;
        let mut var_vmb_dn8: f64 = *var_vmb_dn8_slot;
        let mut var_vmbnew: f64 = *var_vmbnew_slot;
        let mut var_vmbnew_dn5: f64 = *var_vmbnew_dn5_slot;
        let mut var_vmbnew_dn6: f64 = *var_vmbnew_dn6_slot;
        let mut var_vmbnew_dn7: f64 = *var_vmbnew_dn7_slot;
        let mut var_vmbnew_dn8: f64 = *var_vmbnew_dn8_slot;
        let mut var_vsbprime: f64 = *var_vsbprime_slot;
        let mut var_vsbprime_dn6: f64 = *var_vsbprime_dn6_slot;
        let mut var_vsbprime_dn7: f64 = *var_vsbprime_dn7_slot;
        let mut var_vsbprime_dn8: f64 = *var_vsbprime_dn8_slot;
        let mut var_vsbstar: f64 = *var_vsbstar_slot;
        let mut var_vsbstar_dc: f64 = *var_vsbstar_dc_slot;
        let mut var_vsbstar_dc_dn5: f64 = *var_vsbstar_dc_dn5_slot;
        let mut var_vsbstar_dc_dn6: f64 = *var_vsbstar_dc_dn6_slot;
        let mut var_vsbstar_dc_dn7: f64 = *var_vsbstar_dc_dn7_slot;
        let mut var_vsbstar_dc_dn8: f64 = *var_vsbstar_dc_dn8_slot;
        let mut var_vsbstar_dc_tmp: f64 = *var_vsbstar_dc_tmp_slot;
        let mut var_vsbstar_dc_tmp_dn5: f64 = *var_vsbstar_dc_tmp_dn5_slot;
        let mut var_vsbstar_dc_tmp_dn6: f64 = *var_vsbstar_dc_tmp_dn6_slot;
        let mut var_vsbstar_dc_tmp_dn7: f64 = *var_vsbstar_dc_tmp_dn7_slot;
        let mut var_vsbstar_dc_tmp_dn8: f64 = *var_vsbstar_dc_tmp_dn8_slot;
        let mut var_vsbstar_dn5: f64 = *var_vsbstar_dn5_slot;
        let mut var_vsbstar_dn6: f64 = *var_vsbstar_dn6_slot;
        let mut var_vsbstar_dn7: f64 = *var_vsbstar_dn7_slot;
        let mut var_vsbstar_dn8: f64 = *var_vsbstar_dn8_slot;
        let mut var_xgb_ov: f64 = *var_xgb_ov_slot;
        let mut var_xgb_ov_dn5: f64 = *var_xgb_ov_dn5_slot;
        let mut var_xgb_ov_dn6: f64 = *var_xgb_ov_dn6_slot;
        let mut var_xgb_ov_dn7: f64 = *var_xgb_ov_dn7_slot;
        let mut var_xgb_ov_dn8: f64 = *var_xgb_ov_dn8_slot;
        let mut var_xgd_ov: f64 = *var_xgd_ov_slot;
        let mut var_xgd_ov_dn5: f64 = *var_xgd_ov_dn5_slot;
        let mut var_xgd_ov_dn6: f64 = *var_xgd_ov_dn6_slot;
        let mut var_xgd_ov_dn7: f64 = *var_xgd_ov_dn7_slot;
        let mut var_xgs_ov: f64 = *var_xgs_ov_slot;
        let mut var_xgs_ov_dn5: f64 = *var_xgs_ov_dn5_slot;
        let mut var_xgs_ov_dn6: f64 = *var_xgs_ov_dn6_slot;
        let mut var_xgs_ov_dn7: f64 = *var_xgs_ov_dn7_slot;
        let mut var_xhighf2_d: f64 = *var_xhighf2_d_slot;
        let mut var_xhighf2_d_dn5: f64 = *var_xhighf2_d_dn5_slot;
        let mut var_xhighf2_d_dn6: f64 = *var_xhighf2_d_dn6_slot;
        let mut var_xhighf2_d_dn7: f64 = *var_xhighf2_d_dn7_slot;
        let mut var_xhighf2_d_dn8: f64 = *var_xhighf2_d_dn8_slot;
        let mut var_xhighr_d: f64 = *var_xhighr_d_slot;
        let mut var_xhighr_d_dn5: f64 = *var_xhighr_d_dn5_slot;
        let mut var_xhighr_d_dn6: f64 = *var_xhighr_d_dn6_slot;
        let mut var_xhighr_d_dn7: f64 = *var_xhighr_d_dn7_slot;
        let mut var_xhighr_d_dn8: f64 = *var_xhighr_d_dn8_slot;

        let (assign40250_e53427, assign40250_e53427_d_n5, assign40250_e53427_d_n6, assign40250_e53427_d_n7, assign40250_e53427_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign40250_e53425: f64 = (var_xhighf2_d).min(230.25850929940458);
        (assign40250_e53425, if var_xhighf2_d <= 230.25850929940458 { var_xhighf2_d_dn5 } else { 0.0 }, if var_xhighf2_d <= 230.25850929940458 { var_xhighf2_d_dn6 } else { 0.0 }, if var_xhighf2_d <= 230.25850929940458 { var_xhighf2_d_dn7 } else { 0.0 }, if var_xhighf2_d <= 230.25850929940458 { var_xhighf2_d_dn8 } else { 0.0 },)
    } else {
        (var_xhighf2_d, var_xhighf2_d_dn5, var_xhighf2_d_dn6, var_xhighf2_d_dn7, var_xhighf2_d_dn8,)
    }
};
        var_xhighf2_d = assign40250_e53427;
        var_xhighf2_d_dn5 = assign40250_e53427_d_n5;
        var_xhighf2_d_dn6 = assign40250_e53427_d_n6;
        var_xhighf2_d_dn7 = assign40250_e53427_d_n7;
        var_xhighf2_d_dn8 = assign40250_e53427_d_n8;

        let (assign40260_e53434, assign40260_e53434_d_n5, assign40260_e53434_d_n6, assign40260_e53434_d_n7, assign40260_e53434_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign40260_e53432: f64 = (var_xhighf2_d).exp();
        (assign40260_e53432, (assign40260_e53432 * var_xhighf2_d_dn5), (assign40260_e53432 * var_xhighf2_d_dn6), (assign40260_e53432 * var_xhighf2_d_dn7), (assign40260_e53432 * var_xhighf2_d_dn8),)
    } else {
        (var_expxhf2_d, var_expxhf2_d_dn5, var_expxhf2_d_dn6, var_expxhf2_d_dn7, var_expxhf2_d_dn8,)
    }
};
        var_expxhf2_d = assign40260_e53434;
        var_expxhf2_d_dn5 = assign40260_e53434_d_n5;
        var_expxhf2_d_dn6 = assign40260_e53434_d_n6;
        var_expxhf2_d_dn7 = assign40260_e53434_d_n7;
        var_expxhf2_d_dn8 = assign40260_e53434_d_n8;

        let (assign40270_e53442, assign40270_e53442_d_n5, assign40270_e53442_d_n6, assign40270_e53442_d_n7, assign40270_e53442_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign40270_e53440: f64 = (var_xhighr_d).min(230.25850929940458);
        (assign40270_e53440, if var_xhighr_d <= 230.25850929940458 { var_xhighr_d_dn5 } else { 0.0 }, if var_xhighr_d <= 230.25850929940458 { var_xhighr_d_dn6 } else { 0.0 }, if var_xhighr_d <= 230.25850929940458 { var_xhighr_d_dn7 } else { 0.0 }, if var_xhighr_d <= 230.25850929940458 { var_xhighr_d_dn8 } else { 0.0 },)
    } else {
        (var_xhighr_d, var_xhighr_d_dn5, var_xhighr_d_dn6, var_xhighr_d_dn7, var_xhighr_d_dn8,)
    }
};
        var_xhighr_d = assign40270_e53442;
        var_xhighr_d_dn5 = assign40270_e53442_d_n5;
        var_xhighr_d_dn6 = assign40270_e53442_d_n6;
        var_xhighr_d_dn7 = assign40270_e53442_d_n7;
        var_xhighr_d_dn8 = assign40270_e53442_d_n8;

        let (assign40280_e53449, assign40280_e53449_d_n5, assign40280_e53449_d_n6, assign40280_e53449_d_n7, assign40280_e53449_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign40280_e53447: f64 = (var_xhighr_d).exp();
        (assign40280_e53447, (assign40280_e53447 * var_xhighr_d_dn5), (assign40280_e53447 * var_xhighr_d_dn6), (assign40280_e53447 * var_xhighr_d_dn7), (assign40280_e53447 * var_xhighr_d_dn8),)
    } else {
        (var_expxhr_d, var_expxhr_d_dn5, var_expxhr_d_dn6, var_expxhr_d_dn7, var_expxhr_d_dn8,)
    }
};
        var_expxhr_d = assign40280_e53449;
        var_expxhr_d_dn5 = assign40280_e53449_d_n5;
        var_expxhr_d_dn6 = assign40280_e53449_d_n6;
        var_expxhr_d_dn7 = assign40280_e53449_d_n7;
        var_expxhr_d_dn8 = assign40280_e53449_d_n8;

        var_temp__blk936 = 0.0;
        var_temp__blk936_dn5 = 0.0;
        var_temp__blk936_dn6 = 0.0;
        var_temp__blk936_dn7 = 0.0;
        var_temp__blk936_dn8 = 0.0;

        var_temp1 = 0.0;
        var_temp1_dn5 = 0.0;
        var_temp1_dn6 = 0.0;
        var_temp1_dn7 = 0.0;
        var_temp1_dn8 = 0.0;

        var_temp2 = 0.0;
        var_temp2_dn5 = 0.0;
        var_temp2_dn6 = 0.0;
        var_temp2_dn7 = 0.0;
        var_temp2_dn8 = 0.0;

        let assign40320_e53455: f64 = 1.0;
        let assign40320_e53456: f64 = if var_chnl_type == assign40320_e53455 { 1.0 } else { 0.0 };
        var_guard1011 = assign40320_e53456;

        let (assign40330_e53460, assign40330_e53460_d_n5, assign40330_e53460_d_n6, assign40330_e53460_d_n7,) = {
    if (var_guard1011 != 0.0) {
        ((nv5 - nv6), 1.0, -1.0, 0.0,)
    } else {
        (var_v_gs, var_v_gs_dn5, var_v_gs_dn6, var_v_gs_dn7,)
    }
};
        var_v_gs = assign40330_e53460;
        var_v_gs_dn5 = assign40330_e53460_d_n5;
        var_v_gs_dn6 = assign40330_e53460_d_n6;
        var_v_gs_dn7 = assign40330_e53460_d_n7;

        let (assign40340_e53464, assign40340_e53464_d_n6, assign40340_e53464_d_n7,) = {
    if (var_guard1011 != 0.0) {
        ((nv7 - nv6), -1.0, 1.0,)
    } else {
        (var_v_ds, var_v_ds_dn6, var_v_ds_dn7,)
    }
};
        var_v_ds = assign40340_e53464;
        var_v_ds_dn6 = assign40340_e53464_d_n6;
        var_v_ds_dn7 = assign40340_e53464_d_n7;

        let (assign40350_e53468, assign40350_e53468_d_n6, assign40350_e53468_d_n7, assign40350_e53468_d_n8,) = {
    if (var_guard1011 != 0.0) {
        ((nv6 - nv8), 1.0, 0.0, -1.0,)
    } else {
        (var_v_sb, var_v_sb_dn6, var_v_sb_dn7, var_v_sb_dn8,)
    }
};
        var_v_sb = assign40350_e53468;
        var_v_sb_dn6 = assign40350_e53468_d_n6;
        var_v_sb_dn7 = assign40350_e53468_d_n7;
        var_v_sb_dn8 = assign40350_e53468_d_n8;

        let (assign40360_e53473, assign40360_e53473_d_n6, assign40360_e53473_d_n10,) = {
    if (var_guard1011 != 0.0) {
        let assign40360_e53471: f64 = (-(nv6 - nv10));
        (assign40360_e53471, (-1.0), 1.0,)
    } else {
        (var_vjun_s, var_vjun_s_dn6, var_vjun_s_dn10,)
    }
};
        var_vjun_s = assign40360_e53473;
        var_vjun_s_dn6 = assign40360_e53473_d_n6;
        var_vjun_s_dn10 = assign40360_e53473_d_n10;

        let (assign40370_e53478, assign40370_e53478_d_n7, assign40370_e53478_d_n11,) = {
    if (var_guard1011 != 0.0) {
        let assign40370_e53476: f64 = (-(nv7 - nv11));
        (assign40370_e53476, (-1.0), 1.0,)
    } else {
        (var_vjun_d, var_vjun_d_dn7, var_vjun_d_dn11,)
    }
};
        var_vjun_d = assign40370_e53478;
        var_vjun_d_dn7 = assign40370_e53478_d_n7;
        var_vjun_d_dn11 = assign40370_e53478_d_n11;

        let (assign40380_e53484, assign40380_e53484_d_n5, assign40380_e53484_d_n6, assign40380_e53484_d_n7,) = {
    if (var_guard1011 == 0.0) {
        let assign40380_e53482: f64 = (-(nv5 - nv6));
        (assign40380_e53482, (-1.0), 1.0, 0.0,)
    } else {
        (var_v_gs, var_v_gs_dn5, var_v_gs_dn6, var_v_gs_dn7,)
    }
};
        var_v_gs = assign40380_e53484;
        var_v_gs_dn5 = assign40380_e53484_d_n5;
        var_v_gs_dn6 = assign40380_e53484_d_n6;
        var_v_gs_dn7 = assign40380_e53484_d_n7;

        let (assign40390_e53490, assign40390_e53490_d_n6, assign40390_e53490_d_n7,) = {
    if (var_guard1011 == 0.0) {
        let assign40390_e53488: f64 = (-(nv7 - nv6));
        (assign40390_e53488, 1.0, (-1.0),)
    } else {
        (var_v_ds, var_v_ds_dn6, var_v_ds_dn7,)
    }
};
        var_v_ds = assign40390_e53490;
        var_v_ds_dn6 = assign40390_e53490_d_n6;
        var_v_ds_dn7 = assign40390_e53490_d_n7;

        let (assign40400_e53496, assign40400_e53496_d_n6, assign40400_e53496_d_n7, assign40400_e53496_d_n8,) = {
    if (var_guard1011 == 0.0) {
        let assign40400_e53494: f64 = (-(nv6 - nv8));
        (assign40400_e53494, (-1.0), 0.0, 1.0,)
    } else {
        (var_v_sb, var_v_sb_dn6, var_v_sb_dn7, var_v_sb_dn8,)
    }
};
        var_v_sb = assign40400_e53496;
        var_v_sb_dn6 = assign40400_e53496_d_n6;
        var_v_sb_dn7 = assign40400_e53496_d_n7;
        var_v_sb_dn8 = assign40400_e53496_d_n8;

        let (assign40410_e53501, assign40410_e53501_d_n6, assign40410_e53501_d_n10,) = {
    if (var_guard1011 == 0.0) {
        ((nv6 - nv10), 1.0, -1.0,)
    } else {
        (var_vjun_s, var_vjun_s_dn6, var_vjun_s_dn10,)
    }
};
        var_vjun_s = assign40410_e53501;
        var_vjun_s_dn6 = assign40410_e53501_d_n6;
        var_vjun_s_dn10 = assign40410_e53501_d_n10;

        let (assign40420_e53506, assign40420_e53506_d_n7, assign40420_e53506_d_n11,) = {
    if (var_guard1011 == 0.0) {
        ((nv7 - nv11), 1.0, -1.0,)
    } else {
        (var_vjun_d, var_vjun_d_dn7, var_vjun_d_dn11,)
    }
};
        var_vjun_d = assign40420_e53506;
        var_vjun_d_dn7 = assign40420_e53506_d_n7;
        var_vjun_d_dn11 = assign40420_e53506_d_n11;

        let assign40430_e53509: f64 = (var_v_gs + var_v_sb);
        var_vgb = assign40430_e53509;
        var_vgb_dn5 = var_v_gs_dn5;
        var_vgb_dn6 = (var_v_gs_dn6 + var_v_sb_dn6);
        var_vgb_dn7 = (var_v_gs_dn7 + var_v_sb_dn7);
        var_vgb_dn8 = var_v_sb_dn8;

        var_vgsprime = var_v_gs;
        var_vgsprime_dn5 = var_v_gs_dn5;
        var_vgsprime_dn6 = var_v_gs_dn6;
        var_vgsprime_dn7 = var_v_gs_dn7;

        var_vsbprime = var_v_sb;
        var_vsbprime_dn6 = var_v_sb_dn6;
        var_vsbprime_dn7 = var_v_sb_dn7;
        var_vsbprime_dn8 = var_v_sb_dn8;

        let assign40460_e53514: f64 = (var_v_ds + var_v_sb);
        var_vdbprime = assign40460_e53514;
        var_vdbprime_dn6 = (var_v_ds_dn6 + var_v_sb_dn6);
        var_vdbprime_dn7 = (var_v_ds_dn7 + var_v_sb_dn7);
        var_vdbprime_dn8 = var_v_sb_dn8;

        let assign40470_e53517: f64 = (var_v_gs - var_v_ds);
        var_vgdprime = assign40470_e53517;
        var_vgdprime_dn5 = var_v_gs_dn5;
        var_vgdprime_dn6 = (var_v_gs_dn6 - var_v_ds_dn6);
        var_vgdprime_dn7 = (var_v_gs_dn7 - var_v_ds_dn7);

        let assign40480_e53519: f64 = (-var_vgsprime);
        let assign40480_e53521: f64 = (assign40480_e53519 * var_inv_phita);
        var_xgs_ov = assign40480_e53521;
        var_xgs_ov_dn5 = ((-var_vgsprime_dn5) * var_inv_phita);
        var_xgs_ov_dn6 = ((-var_vgsprime_dn6) * var_inv_phita);
        var_xgs_ov_dn7 = ((-var_vgsprime_dn7) * var_inv_phita);

        let assign40490_e53523: f64 = (-var_vgdprime);
        let assign40490_e53525: f64 = (assign40490_e53523 * var_inv_phita);
        var_xgd_ov = assign40490_e53525;
        var_xgd_ov_dn5 = ((-var_vgdprime_dn5) * var_inv_phita);
        var_xgd_ov_dn6 = ((-var_vgdprime_dn6) * var_inv_phita);
        var_xgd_ov_dn7 = ((-var_vgdprime_dn7) * var_inv_phita);

        let assign40500_e53528: f64 = (var_vgb - var_vfb_t);
        let assign40500_e53529: f64 = (-assign40500_e53528);
        let assign40500_e53531: f64 = (assign40500_e53529 * var_inv_phita);
        var_xgb_ov = assign40500_e53531;
        var_xgb_ov_dn5 = ((-var_vgb_dn5) * var_inv_phita);
        var_xgb_ov_dn6 = ((-var_vgb_dn6) * var_inv_phita);
        var_xgb_ov_dn7 = ((-var_vgb_dn7) * var_inv_phita);
        var_xgb_ov_dn8 = ((-var_vgb_dn8) * var_inv_phita);

        var_sigvds = 1.0;

        let assign40520_e53535: f64 = if var_v_ds < 0.0 { 1.0 } else { 0.0 };
        var_guard1012 = assign40520_e53535;

        let (assign40530_e53540,) = {
    if (var_guard1012 != 0.0) {
        let assign40530_e53538: f64 = (-1.0);
        (assign40530_e53538,)
    } else {
        (var_sigvds,)
    }
};
        var_sigvds = assign40530_e53540;

        let (assign40540_e53546, assign40540_e53546_d_n5, assign40540_e53546_d_n6, assign40540_e53546_d_n7,) = {
    if (var_guard1012 != 0.0) {
        let assign40540_e53544: f64 = (var_v_gs - var_v_ds);
        (assign40540_e53544, var_v_gs_dn5, (var_v_gs_dn6 - var_v_ds_dn6), (var_v_gs_dn7 - var_v_ds_dn7),)
    } else {
        (var_v_gs, var_v_gs_dn5, var_v_gs_dn6, var_v_gs_dn7,)
    }
};
        var_v_gs = assign40540_e53546;
        var_v_gs_dn5 = assign40540_e53546_d_n5;
        var_v_gs_dn6 = assign40540_e53546_d_n6;
        var_v_gs_dn7 = assign40540_e53546_d_n7;

        let (assign40550_e53552, assign40550_e53552_d_n6, assign40550_e53552_d_n7, assign40550_e53552_d_n8,) = {
    if (var_guard1012 != 0.0) {
        let assign40550_e53550: f64 = (var_v_sb + var_v_ds);
        (assign40550_e53550, (var_v_sb_dn6 + var_v_ds_dn6), (var_v_sb_dn7 + var_v_ds_dn7), var_v_sb_dn8,)
    } else {
        (var_v_sb, var_v_sb_dn6, var_v_sb_dn7, var_v_sb_dn8,)
    }
};
        var_v_sb = assign40550_e53552;
        var_v_sb_dn6 = assign40550_e53552_d_n6;
        var_v_sb_dn7 = assign40550_e53552_d_n7;
        var_v_sb_dn8 = assign40550_e53552_d_n8;

        let (assign40560_e53557, assign40560_e53557_d_n6, assign40560_e53557_d_n7,) = {
    if (var_guard1012 != 0.0) {
        let assign40560_e53555: f64 = (-var_v_ds);
        (assign40560_e53555, (-var_v_ds_dn6), (-var_v_ds_dn7),)
    } else {
        (var_v_ds, var_v_ds_dn6, var_v_ds_dn7,)
    }
};
        var_v_ds = assign40560_e53557;
        var_v_ds_dn6 = assign40560_e53557_d_n6;
        var_v_ds_dn7 = assign40560_e53557_d_n7;

        let assign40570_e53560: f64 = (var_v_ds + var_v_sb);
        var_v_db = assign40570_e53560;
        var_v_db_dn6 = (var_v_ds_dn6 + var_v_sb_dn6);
        var_v_db_dn7 = (var_v_ds_dn7 + var_v_sb_dn7);
        var_v_db_dn8 = var_v_sb_dn8;

        let assign40580_e53563: f64 = (var_v_ds * var_v_ds);
        let assign40580_e53566: f64 = (var_v_ds * var_v_ds);
        let assign40580_e53568: f64 = (assign40580_e53566 + 0.01);
        let assign40580_e53569: f64 = (assign40580_e53568).sqrt();
        let assign40580_e53571: f64 = (assign40580_e53569 + 0.1);
        let assign40580_e53572: f64 = (assign40580_e53563 / assign40580_e53571);
        var_vdsx = assign40580_e53572;
        var_vdsx_dn6 = (((((var_v_ds_dn6 * var_v_ds) + (var_v_ds * var_v_ds_dn6)) * assign40580_e53571) - (assign40580_e53563 * (((var_v_ds_dn6 * var_v_ds) + (var_v_ds * var_v_ds_dn6)) / (2.0 * assign40580_e53569)))) / (assign40580_e53571 * assign40580_e53571));
        var_vdsx_dn7 = (((((var_v_ds_dn7 * var_v_ds) + (var_v_ds * var_v_ds_dn7)) * assign40580_e53571) - (assign40580_e53563 * (((var_v_ds_dn7 * var_v_ds) + (var_v_ds * var_v_ds_dn7)) / (2.0 * assign40580_e53569)))) / (assign40580_e53571 * assign40580_e53571));

        let assign40590_e53576: f64 = (var_v_db + var_v_sb);
        let assign40590_e53579: f64 = (var_v_db - var_v_sb);
        let assign40590_e53582: f64 = (var_v_db - var_v_sb);
        let assign40590_e53583: f64 = (assign40590_e53579 * assign40590_e53582);
        let assign40590_e53585: f64 = (assign40590_e53583 + var_bphi_dc);
        let assign40590_e53586: f64 = (assign40590_e53585).sqrt();
        let assign40590_e53587: f64 = (assign40590_e53576 - assign40590_e53586);
        let assign40590_e53588: f64 = (0.5 * assign40590_e53587);
        let assign40590_e53590: f64 = (assign40590_e53588 + var_phix_dc);
        var_v_xb = assign40590_e53590;
        var_v_xb_dn6 = (0.5 * ((var_v_db_dn6 + var_v_sb_dn6) - ((((var_v_db_dn6 - var_v_sb_dn6) * assign40590_e53582) + (assign40590_e53579 * (var_v_db_dn6 - var_v_sb_dn6))) / (2.0 * assign40590_e53586))));
        var_v_xb_dn7 = (0.5 * ((var_v_db_dn7 + var_v_sb_dn7) - ((((var_v_db_dn7 - var_v_sb_dn7) * assign40590_e53582) + (assign40590_e53579 * (var_v_db_dn7 - var_v_sb_dn7))) / (2.0 * assign40590_e53586))));
        var_v_xb_dn8 = (0.5 * ((var_v_db_dn8 + var_v_sb_dn8) - ((((var_v_db_dn8 - var_v_sb_dn8) * assign40590_e53582) + (assign40590_e53579 * (var_v_db_dn8 - var_v_sb_dn8))) / (2.0 * assign40590_e53586))));

        var_v_xb_dc_tmp = var_v_xb;
        var_v_xb_dc_tmp_dn6 = var_v_xb_dn6;
        var_v_xb_dc_tmp_dn7 = var_v_xb_dn7;
        var_v_xb_dc_tmp_dn8 = var_v_xb_dn8;

        let assign40610_e53596: f64 = var_v_xb;
        let assign40610_e53599: f64 = var_v_xb;
        let assign40610_e53602: f64 = var_v_xb;
        let assign40610_e53603: f64 = (assign40610_e53599 * assign40610_e53602);
        let assign40610_e53605: f64 = (assign40610_e53603 + var_aphi_dc);
        let assign40610_e53606: f64 = (assign40610_e53605).sqrt();
        let assign40610_e53607: f64 = (assign40610_e53596 - assign40610_e53606);
        let assign40610_e53608: f64 = (0.5 * assign40610_e53607);
        let assign40610_e53609: f64 = (var_v_sb - assign40610_e53608);
        let assign40610_e53611: f64 = (assign40610_e53609 + var_phix1_dc);
        var_vsbstar_dc = assign40610_e53611;
        var_vsbstar_dc_dn5 = 0.0;
        var_vsbstar_dc_dn6 = (var_v_sb_dn6 - (0.5 * (var_v_xb_dn6 - (((var_v_xb_dn6 * assign40610_e53602) + (assign40610_e53599 * var_v_xb_dn6)) / (2.0 * assign40610_e53606)))));
        var_vsbstar_dc_dn7 = (var_v_sb_dn7 - (0.5 * (var_v_xb_dn7 - (((var_v_xb_dn7 * assign40610_e53602) + (assign40610_e53599 * var_v_xb_dn7)) / (2.0 * assign40610_e53606)))));
        var_vsbstar_dc_dn8 = (var_v_sb_dn8 - (0.5 * (var_v_xb_dn8 - (((var_v_xb_dn8 * assign40610_e53602) + (assign40610_e53599 * var_v_xb_dn8)) / (2.0 * assign40610_e53606)))));

        var_vsbstar_dc_tmp = var_vsbstar_dc;
        var_vsbstar_dc_tmp_dn5 = var_vsbstar_dc_dn5;
        var_vsbstar_dc_tmp_dn6 = var_vsbstar_dc_dn6;
        var_vsbstar_dc_tmp_dn7 = var_vsbstar_dc_dn7;
        var_vsbstar_dc_tmp_dn8 = var_vsbstar_dc_dn8;

        var_dvbstar_dc = 0.0;
        var_dvbstar_dc_dn5 = 0.0;
        var_dvbstar_dc_dn6 = 0.0;
        var_dvbstar_dc_dn7 = 0.0;
        var_dvbstar_dc_dn8 = 0.0;

        let assign40640_e53620: f64 = if ((p.p45 != 0.0) && (var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard1172 = assign40640_e53620;

        let (assign40650_e53630, assign40650_e53630_d_n5, assign40650_e53630_d_n6, assign40650_e53630_d_n7, assign40650_e53630_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40650_e53626: f64 = (var_v_ds - var_vdsx);
        let assign40650_e53627: f64 = (0.5 * assign40650_e53626);
        let assign40650_e53628: f64 = (var_vsbstar_dc + assign40650_e53627);
        (assign40650_e53628, var_vsbstar_dc_dn5, (var_vsbstar_dc_dn6 + (0.5 * (var_v_ds_dn6 - var_vdsx_dn6))), (var_vsbstar_dc_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), var_vsbstar_dc_dn8,)
    } else {
        (var_vmb, var_vmb_dn5, var_vmb_dn6, var_vmb_dn7, var_vmb_dn8,)
    }
};
        var_vmb = assign40650_e53630;
        var_vmb_dn5 = assign40650_e53630_d_n5;
        var_vmb_dn6 = assign40650_e53630_d_n6;
        var_vmb_dn7 = assign40650_e53630_d_n7;
        var_vmb_dn8 = assign40650_e53630_d_n8;

        let (assign40660_e53639, assign40660_e53639_d_n5, assign40660_e53639_d_n6, assign40660_e53639_d_n7, assign40660_e53639_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40660_e53634: f64 = (var_vmb + var_phib_dc);
        let assign40660_e53635: f64 = (assign40660_e53634).sqrt();
        let assign40660_e53637: f64 = (assign40660_e53635 - var_sqrt_phib_dc);
        (assign40660_e53637, (var_vmb_dn5 / (2.0 * assign40660_e53635)), (var_vmb_dn6 / (2.0 * assign40660_e53635)), (var_vmb_dn7 / (2.0 * assign40660_e53635)), (var_vmb_dn8 / (2.0 * assign40660_e53635)),)
    } else {
        (var_us, var_us_dn5, var_us_dn6, var_us_dn7, var_us_dn8,)
    }
};
        var_us = assign40660_e53639;
        var_us_dn5 = assign40660_e53639_d_n5;
        var_us_dn6 = assign40660_e53639_d_n6;
        var_us_dn7 = assign40660_e53639_d_n7;
        var_us_dn8 = assign40660_e53639_d_n8;

        let (assign40670_e53651, assign40670_e53651_d_n5, assign40670_e53651_d_n6, assign40670_e53651_d_n7, assign40670_e53651_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40670_e53644: f64 = (var_us - var_us1);
        let assign40670_e53645: f64 = (2.0 * assign40670_e53644);
        let assign40670_e53647: f64 = (assign40670_e53645 / var_us21);
        let assign40670_e53649: f64 = (assign40670_e53647 - 1.0);
        (assign40670_e53649, ((2.0 * var_us_dn5) / var_us21), ((2.0 * var_us_dn6) / var_us21), ((2.0 * var_us_dn7) / var_us21), ((2.0 * var_us_dn8) / var_us21),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign40670_e53651;
        var_temp__blk936_dn5 = assign40670_e53651_d_n5;
        var_temp__blk936_dn6 = assign40670_e53651_d_n6;
        var_temp__blk936_dn7 = assign40670_e53651_d_n7;
        var_temp__blk936_dn8 = assign40670_e53651_d_n8;

        let (assign40680_e53672, assign40680_e53672_d_n5, assign40680_e53672_d_n6, assign40680_e53672_d_n7, assign40680_e53672_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40680_e53657: f64 = (1.0 - var_gfacnud_i);
        let assign40680_e53658: f64 = (0.25 * assign40680_e53657);
        let assign40680_e53660: f64 = (assign40680_e53658 * var_us21);
        let assign40680_e53664: f64 = (var_temp__blk936 * var_temp__blk936);
        let assign40680_e53666: f64 = (assign40680_e53664 + 0.4804530139182);
        let assign40680_e53667: f64 = (assign40680_e53666).sqrt();
        let assign40680_e53668: f64 = (var_temp__blk936 + assign40680_e53667);
        let assign40680_e53669: f64 = (assign40680_e53660 * assign40680_e53668);
        let assign40680_e53670: f64 = (var_us - assign40680_e53669);
        (assign40680_e53670, (var_us_dn5 - (assign40680_e53660 * (var_temp__blk936_dn5 + (((var_temp__blk936_dn5 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn5)) / (2.0 * assign40680_e53667))))), (var_us_dn6 - (assign40680_e53660 * (var_temp__blk936_dn6 + (((var_temp__blk936_dn6 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn6)) / (2.0 * assign40680_e53667))))), (var_us_dn7 - (assign40680_e53660 * (var_temp__blk936_dn7 + (((var_temp__blk936_dn7 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn7)) / (2.0 * assign40680_e53667))))), (var_us_dn8 - (assign40680_e53660 * (var_temp__blk936_dn8 + (((var_temp__blk936_dn8 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn8)) / (2.0 * assign40680_e53667))))),)
    } else {
        (var_usnew, var_usnew_dn5, var_usnew_dn6, var_usnew_dn7, var_usnew_dn8,)
    }
};
        var_usnew = assign40680_e53672;
        var_usnew_dn5 = assign40680_e53672_d_n5;
        var_usnew_dn6 = assign40680_e53672_d_n6;
        var_usnew_dn7 = assign40680_e53672_d_n7;
        var_usnew_dn8 = assign40680_e53672_d_n8;

        let (assign40690_e53684, assign40690_e53684_d_n5, assign40690_e53684_d_n6, assign40690_e53684_d_n7, assign40690_e53684_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40690_e53676: f64 = (var_usnew * var_usnew);
        let assign40690_e53679: f64 = (2.0 * var_sqrt_phib_dc);
        let assign40690_e53681: f64 = (assign40690_e53679 * var_usnew);
        let assign40690_e53682: f64 = (assign40690_e53676 + assign40690_e53681);
        (assign40690_e53682, (((var_usnew_dn5 * var_usnew) + (var_usnew * var_usnew_dn5)) + (assign40690_e53679 * var_usnew_dn5)), (((var_usnew_dn6 * var_usnew) + (var_usnew * var_usnew_dn6)) + (assign40690_e53679 * var_usnew_dn6)), (((var_usnew_dn7 * var_usnew) + (var_usnew * var_usnew_dn7)) + (assign40690_e53679 * var_usnew_dn7)), (((var_usnew_dn8 * var_usnew) + (var_usnew * var_usnew_dn8)) + (assign40690_e53679 * var_usnew_dn8)),)
    } else {
        (var_vmbnew, var_vmbnew_dn5, var_vmbnew_dn6, var_vmbnew_dn7, var_vmbnew_dn8,)
    }
};
        var_vmbnew = assign40690_e53684;
        var_vmbnew_dn5 = assign40690_e53684_d_n5;
        var_vmbnew_dn6 = assign40690_e53684_d_n6;
        var_vmbnew_dn7 = assign40690_e53684_d_n7;
        var_vmbnew_dn8 = assign40690_e53684_d_n8;

        let (assign40700_e53694, assign40700_e53694_d_n5, assign40700_e53694_d_n6, assign40700_e53694_d_n7, assign40700_e53694_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40700_e53690: f64 = (var_v_ds - var_vdsx);
        let assign40700_e53691: f64 = (0.5 * assign40700_e53690);
        let assign40700_e53692: f64 = (var_vmbnew - assign40700_e53691);
        (assign40700_e53692, var_vmbnew_dn5, (var_vmbnew_dn6 - (0.5 * (var_v_ds_dn6 - var_vdsx_dn6))), (var_vmbnew_dn7 - (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), var_vmbnew_dn8,)
    } else {
        (var_vsbstar_dc, var_vsbstar_dc_dn5, var_vsbstar_dc_dn6, var_vsbstar_dc_dn7, var_vsbstar_dc_dn8,)
    }
};
        var_vsbstar_dc = assign40700_e53694;
        var_vsbstar_dc_dn5 = assign40700_e53694_d_n5;
        var_vsbstar_dc_dn6 = assign40700_e53694_d_n6;
        var_vsbstar_dc_dn7 = assign40700_e53694_d_n7;
        var_vsbstar_dc_dn8 = assign40700_e53694_d_n8;

        let (assign40710_e53700, assign40710_e53700_d_n5, assign40710_e53700_d_n6, assign40710_e53700_d_n7, assign40710_e53700_d_n8,) = {
    if (var_guard1172 != 0.0) {
        let assign40710_e53698: f64 = (var_vsbstar_dc_tmp - var_vsbstar_dc);
        (assign40710_e53698, (var_vsbstar_dc_tmp_dn5 - var_vsbstar_dc_dn5), (var_vsbstar_dc_tmp_dn6 - var_vsbstar_dc_dn6), (var_vsbstar_dc_tmp_dn7 - var_vsbstar_dc_dn7), (var_vsbstar_dc_tmp_dn8 - var_vsbstar_dc_dn8),)
    } else {
        (var_dvbstar_dc, var_dvbstar_dc_dn5, var_dvbstar_dc_dn6, var_dvbstar_dc_dn7, var_dvbstar_dc_dn8,)
    }
};
        var_dvbstar_dc = assign40710_e53700;
        var_dvbstar_dc_dn5 = assign40710_e53700_d_n5;
        var_dvbstar_dc_dn6 = assign40710_e53700_d_n6;
        var_dvbstar_dc_dn7 = assign40710_e53700_d_n7;
        var_dvbstar_dc_dn8 = assign40710_e53700_d_n8;

        var_phib = var_phib_dc;

        var_aphi = var_aphi_dc;

        var_g_0 = var_g_0_dc;

        var_vsbstar = var_vsbstar_dc;
        var_vsbstar_dn5 = var_vsbstar_dc_dn5;
        var_vsbstar_dn6 = var_vsbstar_dc_dn6;
        var_vsbstar_dn7 = var_vsbstar_dc_dn7;
        var_vsbstar_dn8 = var_vsbstar_dc_dn8;

        var_dvbstar = var_dvbstar_dc;
        var_dvbstar_dn5 = var_dvbstar_dc_dn5;
        var_dvbstar_dn6 = var_dvbstar_dc_dn6;
        var_dvbstar_dn7 = var_dvbstar_dc_dn7;
        var_dvbstar_dn8 = var_dvbstar_dc_dn8;

        var_thesatloc = var_thesat_t;

        var_arloc = var_ar;

        *var_aphi_slot = var_aphi;
        *var_arloc_slot = var_arloc;
        *var_dvbstar_slot = var_dvbstar;
        *var_dvbstar_dc_slot = var_dvbstar_dc;
        *var_dvbstar_dc_dn5_slot = var_dvbstar_dc_dn5;
        *var_dvbstar_dc_dn6_slot = var_dvbstar_dc_dn6;
        *var_dvbstar_dc_dn7_slot = var_dvbstar_dc_dn7;
        *var_dvbstar_dc_dn8_slot = var_dvbstar_dc_dn8;
        *var_dvbstar_dn5_slot = var_dvbstar_dn5;
        *var_dvbstar_dn6_slot = var_dvbstar_dn6;
        *var_dvbstar_dn7_slot = var_dvbstar_dn7;
        *var_dvbstar_dn8_slot = var_dvbstar_dn8;
        *var_expxhf2_d_slot = var_expxhf2_d;
        *var_expxhf2_d_dn5_slot = var_expxhf2_d_dn5;
        *var_expxhf2_d_dn6_slot = var_expxhf2_d_dn6;
        *var_expxhf2_d_dn7_slot = var_expxhf2_d_dn7;
        *var_expxhf2_d_dn8_slot = var_expxhf2_d_dn8;
        *var_expxhr_d_slot = var_expxhr_d;
        *var_expxhr_d_dn5_slot = var_expxhr_d_dn5;
        *var_expxhr_d_dn6_slot = var_expxhr_d_dn6;
        *var_expxhr_d_dn7_slot = var_expxhr_d_dn7;
        *var_expxhr_d_dn8_slot = var_expxhr_d_dn8;
        *var_g_0_slot = var_g_0;
        *var_guard1011_slot = var_guard1011;
        *var_guard1012_slot = var_guard1012;
        *var_guard1172_slot = var_guard1172;
        *var_phib_slot = var_phib;
        *var_sigvds_slot = var_sigvds;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_thesatloc_slot = var_thesatloc;
        *var_us_slot = var_us;
        *var_us_dn5_slot = var_us_dn5;
        *var_us_dn6_slot = var_us_dn6;
        *var_us_dn7_slot = var_us_dn7;
        *var_us_dn8_slot = var_us_dn8;
        *var_usnew_slot = var_usnew;
        *var_usnew_dn5_slot = var_usnew_dn5;
        *var_usnew_dn6_slot = var_usnew_dn6;
        *var_usnew_dn7_slot = var_usnew_dn7;
        *var_usnew_dn8_slot = var_usnew_dn8;
        *var_v_db_slot = var_v_db;
        *var_v_db_dn6_slot = var_v_db_dn6;
        *var_v_db_dn7_slot = var_v_db_dn7;
        *var_v_db_dn8_slot = var_v_db_dn8;
        *var_v_ds_slot = var_v_ds;
        *var_v_ds_dn6_slot = var_v_ds_dn6;
        *var_v_ds_dn7_slot = var_v_ds_dn7;
        *var_v_gs_slot = var_v_gs;
        *var_v_gs_dn5_slot = var_v_gs_dn5;
        *var_v_gs_dn6_slot = var_v_gs_dn6;
        *var_v_gs_dn7_slot = var_v_gs_dn7;
        *var_v_sb_slot = var_v_sb;
        *var_v_sb_dn6_slot = var_v_sb_dn6;
        *var_v_sb_dn7_slot = var_v_sb_dn7;
        *var_v_sb_dn8_slot = var_v_sb_dn8;
        *var_v_xb_slot = var_v_xb;
        *var_v_xb_dc_tmp_slot = var_v_xb_dc_tmp;
        *var_v_xb_dc_tmp_dn6_slot = var_v_xb_dc_tmp_dn6;
        *var_v_xb_dc_tmp_dn7_slot = var_v_xb_dc_tmp_dn7;
        *var_v_xb_dc_tmp_dn8_slot = var_v_xb_dc_tmp_dn8;
        *var_v_xb_dn6_slot = var_v_xb_dn6;
        *var_v_xb_dn7_slot = var_v_xb_dn7;
        *var_v_xb_dn8_slot = var_v_xb_dn8;
        *var_vdbprime_slot = var_vdbprime;
        *var_vdbprime_dn6_slot = var_vdbprime_dn6;
        *var_vdbprime_dn7_slot = var_vdbprime_dn7;
        *var_vdbprime_dn8_slot = var_vdbprime_dn8;
        *var_vdsx_slot = var_vdsx;
        *var_vdsx_dn6_slot = var_vdsx_dn6;
        *var_vdsx_dn7_slot = var_vdsx_dn7;
        *var_vgb_slot = var_vgb;
        *var_vgb_dn5_slot = var_vgb_dn5;
        *var_vgb_dn6_slot = var_vgb_dn6;
        *var_vgb_dn7_slot = var_vgb_dn7;
        *var_vgb_dn8_slot = var_vgb_dn8;
        *var_vgdprime_slot = var_vgdprime;
        *var_vgdprime_dn5_slot = var_vgdprime_dn5;
        *var_vgdprime_dn6_slot = var_vgdprime_dn6;
        *var_vgdprime_dn7_slot = var_vgdprime_dn7;
        *var_vgsprime_slot = var_vgsprime;
        *var_vgsprime_dn5_slot = var_vgsprime_dn5;
        *var_vgsprime_dn6_slot = var_vgsprime_dn6;
        *var_vgsprime_dn7_slot = var_vgsprime_dn7;
        *var_vjun_d_slot = var_vjun_d;
        *var_vjun_d_dn11_slot = var_vjun_d_dn11;
        *var_vjun_d_dn7_slot = var_vjun_d_dn7;
        *var_vjun_s_slot = var_vjun_s;
        *var_vjun_s_dn10_slot = var_vjun_s_dn10;
        *var_vjun_s_dn6_slot = var_vjun_s_dn6;
        *var_vmb_slot = var_vmb;
        *var_vmb_dn5_slot = var_vmb_dn5;
        *var_vmb_dn6_slot = var_vmb_dn6;
        *var_vmb_dn7_slot = var_vmb_dn7;
        *var_vmb_dn8_slot = var_vmb_dn8;
        *var_vmbnew_slot = var_vmbnew;
        *var_vmbnew_dn5_slot = var_vmbnew_dn5;
        *var_vmbnew_dn6_slot = var_vmbnew_dn6;
        *var_vmbnew_dn7_slot = var_vmbnew_dn7;
        *var_vmbnew_dn8_slot = var_vmbnew_dn8;
        *var_vsbprime_slot = var_vsbprime;
        *var_vsbprime_dn6_slot = var_vsbprime_dn6;
        *var_vsbprime_dn7_slot = var_vsbprime_dn7;
        *var_vsbprime_dn8_slot = var_vsbprime_dn8;
        *var_vsbstar_slot = var_vsbstar;
        *var_vsbstar_dc_slot = var_vsbstar_dc;
        *var_vsbstar_dc_dn5_slot = var_vsbstar_dc_dn5;
        *var_vsbstar_dc_dn6_slot = var_vsbstar_dc_dn6;
        *var_vsbstar_dc_dn7_slot = var_vsbstar_dc_dn7;
        *var_vsbstar_dc_dn8_slot = var_vsbstar_dc_dn8;
        *var_vsbstar_dc_tmp_slot = var_vsbstar_dc_tmp;
        *var_vsbstar_dc_tmp_dn5_slot = var_vsbstar_dc_tmp_dn5;
        *var_vsbstar_dc_tmp_dn6_slot = var_vsbstar_dc_tmp_dn6;
        *var_vsbstar_dc_tmp_dn7_slot = var_vsbstar_dc_tmp_dn7;
        *var_vsbstar_dc_tmp_dn8_slot = var_vsbstar_dc_tmp_dn8;
        *var_vsbstar_dn5_slot = var_vsbstar_dn5;
        *var_vsbstar_dn6_slot = var_vsbstar_dn6;
        *var_vsbstar_dn7_slot = var_vsbstar_dn7;
        *var_vsbstar_dn8_slot = var_vsbstar_dn8;
        *var_xgb_ov_slot = var_xgb_ov;
        *var_xgb_ov_dn5_slot = var_xgb_ov_dn5;
        *var_xgb_ov_dn6_slot = var_xgb_ov_dn6;
        *var_xgb_ov_dn7_slot = var_xgb_ov_dn7;
        *var_xgb_ov_dn8_slot = var_xgb_ov_dn8;
        *var_xgd_ov_slot = var_xgd_ov;
        *var_xgd_ov_dn5_slot = var_xgd_ov_dn5;
        *var_xgd_ov_dn6_slot = var_xgd_ov_dn6;
        *var_xgd_ov_dn7_slot = var_xgd_ov_dn7;
        *var_xgs_ov_slot = var_xgs_ov;
        *var_xgs_ov_dn5_slot = var_xgs_ov_dn5;
        *var_xgs_ov_dn6_slot = var_xgs_ov_dn6;
        *var_xgs_ov_dn7_slot = var_xgs_ov_dn7;
        *var_xhighf2_d_slot = var_xhighf2_d;
        *var_xhighf2_d_dn5_slot = var_xhighf2_d_dn5;
        *var_xhighf2_d_dn6_slot = var_xhighf2_d_dn6;
        *var_xhighf2_d_dn7_slot = var_xhighf2_d_dn7;
        *var_xhighf2_d_dn8_slot = var_xhighf2_d_dn8;
        *var_xhighr_d_slot = var_xhighr_d;
        *var_xhighr_d_dn5_slot = var_xhighr_d_dn5;
        *var_xhighr_d_dn6_slot = var_xhighr_d_dn6;
        *var_xhighr_d_dn7_slot = var_xhighr_d_dn7;
        *var_xhighr_d_dn8_slot = var_xhighr_d_dn8;
    }

    pub(super) fn stamp_transient_block_87(
        p: &Parameters,
        var_aphi: f64,
        var_cf_i: f64,
        var_cfb_i: f64,
        var_cfd_i: f64,
        var_ct_t: f64,
        var_ctb_i: f64,
        var_ctg_i: f64,
        var_ctg_t: f64,
        var_dvbstar: f64,
        var_dvbstar_dn5: f64,
        var_dvbstar_dn6: f64,
        var_dvbstar_dn7: f64,
        var_dvbstar_dn8: f64,
        var_g_0: f64,
        var_inv_phit: f64,
        var_phib: f64,
        var_phit: f64,
        var_psce_i: f64,
        var_psceb_i: f64,
        var_psced_i: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_v_xb: f64,
        var_v_xb_dn6: f64,
        var_v_xb_dn7: f64,
        var_v_xb_dn8: f64,
        var_vdsx: f64,
        var_vdsx_dn6: f64,
        var_vdsx_dn7: f64,
        var_vfb_t: f64,
        var_vgb: f64,
        var_vgb_dn5: f64,
        var_vgb_dn6: f64,
        var_vgb_dn7: f64,
        var_vgb_dn8: f64,
        var_vsbstar: f64,
        var_vsbstar_dn5: f64,
        var_vsbstar_dn6: f64,
        var_vsbstar_dn7: f64,
        var_vsbstar_dn8: f64,
        var_ct_fact_slot: &mut f64,
        var_ct_fact_dn5_slot: &mut f64,
        var_ct_fact_dn6_slot: &mut f64,
        var_ct_fact_dn7_slot: &mut f64,
        var_ct_fact_dn8_slot: &mut f64,
        var_dctg_slot: &mut f64,
        var_dctg_dn5_slot: &mut f64,
        var_dctg_dn6_slot: &mut f64,
        var_dctg_dn7_slot: &mut f64,
        var_dctg_dn8_slot: &mut f64,
        var_delphib_slot: &mut f64,
        var_delphib_dn5_slot: &mut f64,
        var_delphib_dn6_slot: &mut f64,
        var_delphib_dn7_slot: &mut f64,
        var_delphib_dn8_slot: &mut f64,
        var_delxb_slot: &mut f64,
        var_delxb_dn5_slot: &mut f64,
        var_delxb_dn6_slot: &mut f64,
        var_delxb_dn7_slot: &mut f64,
        var_delxb_dn8_slot: &mut f64,
        var_dphit1_slot: &mut f64,
        var_dphit1_dn5_slot: &mut f64,
        var_dphit1_dn6_slot: &mut f64,
        var_dphit1_dn7_slot: &mut f64,
        var_dphit1_dn8_slot: &mut f64,
        var_gf_slot: &mut f64,
        var_gf2_slot: &mut f64,
        var_gf2_dn5_slot: &mut f64,
        var_gf2_dn6_slot: &mut f64,
        var_gf2_dn7_slot: &mut f64,
        var_gf2_dn8_slot: &mut f64,
        var_gf_dn5_slot: &mut f64,
        var_gf_dn6_slot: &mut f64,
        var_gf_dn7_slot: &mut f64,
        var_gf_dn8_slot: &mut f64,
        var_guard1173_slot: &mut f64,
        var_guard1174_slot: &mut f64,
        var_guard1175_slot: &mut f64,
        var_guard1176_slot: &mut f64,
        var_inv_gf2_slot: &mut f64,
        var_inv_gf2_dn5_slot: &mut f64,
        var_inv_gf2_dn6_slot: &mut f64,
        var_inv_gf2_dn7_slot: &mut f64,
        var_inv_gf2_dn8_slot: &mut f64,
        var_inv_phit1_slot: &mut f64,
        var_inv_phit1_dn5_slot: &mut f64,
        var_inv_phit1_dn6_slot: &mut f64,
        var_inv_phit1_dn7_slot: &mut f64,
        var_inv_phit1_dn8_slot: &mut f64,
        var_phit1_slot: &mut f64,
        var_phit1_dn5_slot: &mut f64,
        var_phit1_dn6_slot: &mut f64,
        var_phit1_dn7_slot: &mut f64,
        var_phit1_dn8_slot: &mut f64,
        var_phitct_slot: &mut f64,
        var_phitct_dn5_slot: &mut f64,
        var_phitct_dn6_slot: &mut f64,
        var_phitct_dn7_slot: &mut f64,
        var_phitct_dn8_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_ux_slot: &mut f64,
        var_ux_dn5_slot: &mut f64,
        var_ux_dn6_slot: &mut f64,
        var_ux_dn7_slot: &mut f64,
        var_ux_dn8_slot: &mut f64,
        var_vdsp_slot: &mut f64,
        var_vdsp_dn6_slot: &mut f64,
        var_vdsp_dn7_slot: &mut f64,
        var_vgb1_slot: &mut f64,
        var_vgb1_dn5_slot: &mut f64,
        var_vgb1_dn6_slot: &mut f64,
        var_vgb1_dn7_slot: &mut f64,
        var_vgb1_dn8_slot: &mut f64,
        var_vsbx_slot: &mut f64,
        var_vsbx_dn5_slot: &mut f64,
        var_vsbx_dn6_slot: &mut f64,
        var_vsbx_dn7_slot: &mut f64,
        var_vsbx_dn8_slot: &mut f64,
        var_xb_slot: &mut f64,
        var_xb_dn5_slot: &mut f64,
        var_xb_dn6_slot: &mut f64,
        var_xb_dn7_slot: &mut f64,
        var_xb_dn8_slot: &mut f64,
        var_xbct_slot: &mut f64,
        var_xct_slot: &mut f64,
        var_xct_dn5_slot: &mut f64,
        var_xct_dn6_slot: &mut f64,
        var_xct_dn7_slot: &mut f64,
        var_xct_dn8_slot: &mut f64,
        var_xctmax_slot: &mut f64,
        var_xg_slot: &mut f64,
        var_xg_dn5_slot: &mut f64,
        var_xg_dn6_slot: &mut f64,
        var_xg_dn7_slot: &mut f64,
        var_xg_dn8_slot: &mut f64,
        var_xgct_slot: &mut f64,
        var_xgct_dn5_slot: &mut f64,
        var_xgct_dn6_slot: &mut f64,
        var_xgct_dn7_slot: &mut f64,
        var_xgct_dn8_slot: &mut f64,
        var_xmict_slot: &mut f64,
        var_xmict_dn5_slot: &mut f64,
        var_xmict_dn6_slot: &mut f64,
        var_xmict_dn7_slot: &mut f64,
        var_xmict_dn8_slot: &mut f64,
        var_xn_s_slot: &mut f64,
        var_xn_s_dn5_slot: &mut f64,
        var_xn_s_dn6_slot: &mut f64,
        var_xn_s_dn7_slot: &mut f64,
        var_xn_s_dn8_slot: &mut f64,
        var_xnct_slot: &mut f64,
        var_xnct_dn5_slot: &mut f64,
        var_xnct_dn6_slot: &mut f64,
        var_xnct_dn7_slot: &mut f64,
        var_xnct_dn8_slot: &mut f64,
        var_xno_s_slot: &mut f64,
        var_xno_s_dn5_slot: &mut f64,
        var_xno_s_dn6_slot: &mut f64,
        var_xno_s_dn7_slot: &mut f64,
        var_xno_s_dn8_slot: &mut f64,
        var_xsbstar_slot: &mut f64,
        var_xsbstar_dn5_slot: &mut f64,
        var_xsbstar_dn6_slot: &mut f64,
        var_xsbstar_dn7_slot: &mut f64,
        var_xsbstar_dn8_slot: &mut f64,
        var_xsubct_slot: &mut f64,
        var_xsubct_dn5_slot: &mut f64,
        var_xsubct_dn6_slot: &mut f64,
        var_xsubct_dn7_slot: &mut f64,
        var_xsubct_dn8_slot: &mut f64,
        var_xwict_slot: &mut f64,
        var_xwict_dn5_slot: &mut f64,
        var_xwict_dn6_slot: &mut f64,
        var_xwict_dn7_slot: &mut f64,
        var_xwict_dn8_slot: &mut f64,
    ) {
        let mut var_ct_fact: f64 = *var_ct_fact_slot;
        let mut var_ct_fact_dn5: f64 = *var_ct_fact_dn5_slot;
        let mut var_ct_fact_dn6: f64 = *var_ct_fact_dn6_slot;
        let mut var_ct_fact_dn7: f64 = *var_ct_fact_dn7_slot;
        let mut var_ct_fact_dn8: f64 = *var_ct_fact_dn8_slot;
        let mut var_dctg: f64 = *var_dctg_slot;
        let mut var_dctg_dn5: f64 = *var_dctg_dn5_slot;
        let mut var_dctg_dn6: f64 = *var_dctg_dn6_slot;
        let mut var_dctg_dn7: f64 = *var_dctg_dn7_slot;
        let mut var_dctg_dn8: f64 = *var_dctg_dn8_slot;
        let mut var_delphib: f64 = *var_delphib_slot;
        let mut var_delphib_dn5: f64 = *var_delphib_dn5_slot;
        let mut var_delphib_dn6: f64 = *var_delphib_dn6_slot;
        let mut var_delphib_dn7: f64 = *var_delphib_dn7_slot;
        let mut var_delphib_dn8: f64 = *var_delphib_dn8_slot;
        let mut var_delxb: f64 = *var_delxb_slot;
        let mut var_delxb_dn5: f64 = *var_delxb_dn5_slot;
        let mut var_delxb_dn6: f64 = *var_delxb_dn6_slot;
        let mut var_delxb_dn7: f64 = *var_delxb_dn7_slot;
        let mut var_delxb_dn8: f64 = *var_delxb_dn8_slot;
        let mut var_dphit1: f64 = *var_dphit1_slot;
        let mut var_dphit1_dn5: f64 = *var_dphit1_dn5_slot;
        let mut var_dphit1_dn6: f64 = *var_dphit1_dn6_slot;
        let mut var_dphit1_dn7: f64 = *var_dphit1_dn7_slot;
        let mut var_dphit1_dn8: f64 = *var_dphit1_dn8_slot;
        let mut var_gf: f64 = *var_gf_slot;
        let mut var_gf2: f64 = *var_gf2_slot;
        let mut var_gf2_dn5: f64 = *var_gf2_dn5_slot;
        let mut var_gf2_dn6: f64 = *var_gf2_dn6_slot;
        let mut var_gf2_dn7: f64 = *var_gf2_dn7_slot;
        let mut var_gf2_dn8: f64 = *var_gf2_dn8_slot;
        let mut var_gf_dn5: f64 = *var_gf_dn5_slot;
        let mut var_gf_dn6: f64 = *var_gf_dn6_slot;
        let mut var_gf_dn7: f64 = *var_gf_dn7_slot;
        let mut var_gf_dn8: f64 = *var_gf_dn8_slot;
        let mut var_guard1173: f64 = *var_guard1173_slot;
        let mut var_guard1174: f64 = *var_guard1174_slot;
        let mut var_guard1175: f64 = *var_guard1175_slot;
        let mut var_guard1176: f64 = *var_guard1176_slot;
        let mut var_inv_gf2: f64 = *var_inv_gf2_slot;
        let mut var_inv_gf2_dn5: f64 = *var_inv_gf2_dn5_slot;
        let mut var_inv_gf2_dn6: f64 = *var_inv_gf2_dn6_slot;
        let mut var_inv_gf2_dn7: f64 = *var_inv_gf2_dn7_slot;
        let mut var_inv_gf2_dn8: f64 = *var_inv_gf2_dn8_slot;
        let mut var_inv_phit1: f64 = *var_inv_phit1_slot;
        let mut var_inv_phit1_dn5: f64 = *var_inv_phit1_dn5_slot;
        let mut var_inv_phit1_dn6: f64 = *var_inv_phit1_dn6_slot;
        let mut var_inv_phit1_dn7: f64 = *var_inv_phit1_dn7_slot;
        let mut var_inv_phit1_dn8: f64 = *var_inv_phit1_dn8_slot;
        let mut var_phit1: f64 = *var_phit1_slot;
        let mut var_phit1_dn5: f64 = *var_phit1_dn5_slot;
        let mut var_phit1_dn6: f64 = *var_phit1_dn6_slot;
        let mut var_phit1_dn7: f64 = *var_phit1_dn7_slot;
        let mut var_phit1_dn8: f64 = *var_phit1_dn8_slot;
        let mut var_phitct: f64 = *var_phitct_slot;
        let mut var_phitct_dn5: f64 = *var_phitct_dn5_slot;
        let mut var_phitct_dn6: f64 = *var_phitct_dn6_slot;
        let mut var_phitct_dn7: f64 = *var_phitct_dn7_slot;
        let mut var_phitct_dn8: f64 = *var_phitct_dn8_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_ux: f64 = *var_ux_slot;
        let mut var_ux_dn5: f64 = *var_ux_dn5_slot;
        let mut var_ux_dn6: f64 = *var_ux_dn6_slot;
        let mut var_ux_dn7: f64 = *var_ux_dn7_slot;
        let mut var_ux_dn8: f64 = *var_ux_dn8_slot;
        let mut var_vdsp: f64 = *var_vdsp_slot;
        let mut var_vdsp_dn6: f64 = *var_vdsp_dn6_slot;
        let mut var_vdsp_dn7: f64 = *var_vdsp_dn7_slot;
        let mut var_vgb1: f64 = *var_vgb1_slot;
        let mut var_vgb1_dn5: f64 = *var_vgb1_dn5_slot;
        let mut var_vgb1_dn6: f64 = *var_vgb1_dn6_slot;
        let mut var_vgb1_dn7: f64 = *var_vgb1_dn7_slot;
        let mut var_vgb1_dn8: f64 = *var_vgb1_dn8_slot;
        let mut var_vsbx: f64 = *var_vsbx_slot;
        let mut var_vsbx_dn5: f64 = *var_vsbx_dn5_slot;
        let mut var_vsbx_dn6: f64 = *var_vsbx_dn6_slot;
        let mut var_vsbx_dn7: f64 = *var_vsbx_dn7_slot;
        let mut var_vsbx_dn8: f64 = *var_vsbx_dn8_slot;
        let mut var_xb: f64 = *var_xb_slot;
        let mut var_xb_dn5: f64 = *var_xb_dn5_slot;
        let mut var_xb_dn6: f64 = *var_xb_dn6_slot;
        let mut var_xb_dn7: f64 = *var_xb_dn7_slot;
        let mut var_xb_dn8: f64 = *var_xb_dn8_slot;
        let mut var_xbct: f64 = *var_xbct_slot;
        let mut var_xct: f64 = *var_xct_slot;
        let mut var_xct_dn5: f64 = *var_xct_dn5_slot;
        let mut var_xct_dn6: f64 = *var_xct_dn6_slot;
        let mut var_xct_dn7: f64 = *var_xct_dn7_slot;
        let mut var_xct_dn8: f64 = *var_xct_dn8_slot;
        let mut var_xctmax: f64 = *var_xctmax_slot;
        let mut var_xg: f64 = *var_xg_slot;
        let mut var_xg_dn5: f64 = *var_xg_dn5_slot;
        let mut var_xg_dn6: f64 = *var_xg_dn6_slot;
        let mut var_xg_dn7: f64 = *var_xg_dn7_slot;
        let mut var_xg_dn8: f64 = *var_xg_dn8_slot;
        let mut var_xgct: f64 = *var_xgct_slot;
        let mut var_xgct_dn5: f64 = *var_xgct_dn5_slot;
        let mut var_xgct_dn6: f64 = *var_xgct_dn6_slot;
        let mut var_xgct_dn7: f64 = *var_xgct_dn7_slot;
        let mut var_xgct_dn8: f64 = *var_xgct_dn8_slot;
        let mut var_xmict: f64 = *var_xmict_slot;
        let mut var_xmict_dn5: f64 = *var_xmict_dn5_slot;
        let mut var_xmict_dn6: f64 = *var_xmict_dn6_slot;
        let mut var_xmict_dn7: f64 = *var_xmict_dn7_slot;
        let mut var_xmict_dn8: f64 = *var_xmict_dn8_slot;
        let mut var_xn_s: f64 = *var_xn_s_slot;
        let mut var_xn_s_dn5: f64 = *var_xn_s_dn5_slot;
        let mut var_xn_s_dn6: f64 = *var_xn_s_dn6_slot;
        let mut var_xn_s_dn7: f64 = *var_xn_s_dn7_slot;
        let mut var_xn_s_dn8: f64 = *var_xn_s_dn8_slot;
        let mut var_xnct: f64 = *var_xnct_slot;
        let mut var_xnct_dn5: f64 = *var_xnct_dn5_slot;
        let mut var_xnct_dn6: f64 = *var_xnct_dn6_slot;
        let mut var_xnct_dn7: f64 = *var_xnct_dn7_slot;
        let mut var_xnct_dn8: f64 = *var_xnct_dn8_slot;
        let mut var_xno_s: f64 = *var_xno_s_slot;
        let mut var_xno_s_dn5: f64 = *var_xno_s_dn5_slot;
        let mut var_xno_s_dn6: f64 = *var_xno_s_dn6_slot;
        let mut var_xno_s_dn7: f64 = *var_xno_s_dn7_slot;
        let mut var_xno_s_dn8: f64 = *var_xno_s_dn8_slot;
        let mut var_xsbstar: f64 = *var_xsbstar_slot;
        let mut var_xsbstar_dn5: f64 = *var_xsbstar_dn5_slot;
        let mut var_xsbstar_dn6: f64 = *var_xsbstar_dn6_slot;
        let mut var_xsbstar_dn7: f64 = *var_xsbstar_dn7_slot;
        let mut var_xsbstar_dn8: f64 = *var_xsbstar_dn8_slot;
        let mut var_xsubct: f64 = *var_xsubct_slot;
        let mut var_xsubct_dn5: f64 = *var_xsubct_dn5_slot;
        let mut var_xsubct_dn6: f64 = *var_xsubct_dn6_slot;
        let mut var_xsubct_dn7: f64 = *var_xsubct_dn7_slot;
        let mut var_xsubct_dn8: f64 = *var_xsubct_dn8_slot;
        let mut var_xwict: f64 = *var_xwict_slot;
        let mut var_xwict_dn5: f64 = *var_xwict_dn5_slot;
        let mut var_xwict_dn6: f64 = *var_xwict_dn6_slot;
        let mut var_xwict_dn7: f64 = *var_xwict_dn7_slot;
        let mut var_xwict_dn8: f64 = *var_xwict_dn8_slot;

        let assign40790_e53710: f64 = (var_vgb - var_dvbstar);
        let assign40790_e53712: f64 = (assign40790_e53710 - var_vfb_t);
        var_vgb1 = assign40790_e53712;
        var_vgb1_dn5 = (var_vgb_dn5 - var_dvbstar_dn5);
        var_vgb1_dn6 = (var_vgb_dn6 - var_dvbstar_dn6);
        var_vgb1_dn7 = (var_vgb_dn7 - var_dvbstar_dn7);
        var_vgb1_dn8 = (var_vgb_dn8 - var_dvbstar_dn8);

        let assign40800_e53717: f64 = (var_v_ds - var_vdsx);
        let assign40800_e53718: f64 = (0.5 * assign40800_e53717);
        let assign40800_e53719: f64 = (var_vsbstar + assign40800_e53718);
        var_vsbx = assign40800_e53719;
        var_vsbx_dn5 = var_vsbstar_dn5;
        var_vsbx_dn6 = (var_vsbstar_dn6 + (0.5 * (var_v_ds_dn6 - var_vdsx_dn6)));
        var_vsbx_dn7 = (var_vsbstar_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7)));
        var_vsbx_dn8 = var_vsbstar_dn8;

        var_dctg = 1.0;
        var_dctg_dn5 = 0.0;
        var_dctg_dn6 = 0.0;
        var_dctg_dn7 = 0.0;
        var_dctg_dn8 = 0.0;

        let assign40820_e53723: f64 = if var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1173 = assign40820_e53723;

        let (assign40830_e53729,) = {
    if (var_guard1173 != 0.0) {
        let assign40830_e53727: f64 = (var_phib * var_inv_phit);
        (assign40830_e53727,)
    } else {
        (var_xbct,)
    }
};
        var_xbct = assign40830_e53729;

        let (assign40840_e53735, assign40840_e53735_d_n5, assign40840_e53735_d_n6, assign40840_e53735_d_n7, assign40840_e53735_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40840_e53733: f64 = (var_vsbx * var_inv_phit);
        (assign40840_e53733, (var_vsbx_dn5 * var_inv_phit), (var_vsbx_dn6 * var_inv_phit), (var_vsbx_dn7 * var_inv_phit), (var_vsbx_dn8 * var_inv_phit),)
    } else {
        (var_xsbstar, var_xsbstar_dn5, var_xsbstar_dn6, var_xsbstar_dn7, var_xsbstar_dn8,)
    }
};
        var_xsbstar = assign40840_e53735;
        var_xsbstar_dn5 = assign40840_e53735_d_n5;
        var_xsbstar_dn6 = assign40840_e53735_d_n6;
        var_xsbstar_dn7 = assign40840_e53735_d_n7;
        var_xsbstar_dn8 = assign40840_e53735_d_n8;

        let (assign40850_e53741, assign40850_e53741_d_n5, assign40850_e53741_d_n6, assign40850_e53741_d_n7, assign40850_e53741_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40850_e53739: f64 = (var_vgb1 * var_inv_phit);
        (assign40850_e53739, (var_vgb1_dn5 * var_inv_phit), (var_vgb1_dn6 * var_inv_phit), (var_vgb1_dn7 * var_inv_phit), (var_vgb1_dn8 * var_inv_phit),)
    } else {
        (var_xgct, var_xgct_dn5, var_xgct_dn6, var_xgct_dn7, var_xgct_dn8,)
    }
};
        var_xgct = assign40850_e53741;
        var_xgct_dn5 = assign40850_e53741_d_n5;
        var_xgct_dn6 = assign40850_e53741_d_n6;
        var_xgct_dn7 = assign40850_e53741_d_n7;
        var_xgct_dn8 = assign40850_e53741_d_n8;

        let (assign40860_e53752, assign40860_e53752_d_n5, assign40860_e53752_d_n6, assign40860_e53752_d_n7, assign40860_e53752_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40860_e53746: f64 = (0.5 * var_g_0);
        let assign40860_e53748: f64 = (var_xbct).sqrt();
        let assign40860_e53749: f64 = (assign40860_e53746 / assign40860_e53748);
        let assign40860_e53750: f64 = (1.0 + assign40860_e53749);
        (assign40860_e53750, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign40860_e53752;
        var_temp1_dn5 = assign40860_e53752_d_n5;
        var_temp1_dn6 = assign40860_e53752_d_n6;
        var_temp1_dn7 = assign40860_e53752_d_n7;
        var_temp1_dn8 = assign40860_e53752_d_n8;

        let (assign40870_e53761, assign40870_e53761_d_n5, assign40870_e53761_d_n6, assign40870_e53761_d_n7, assign40870_e53761_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40870_e53757: f64 = (var_xbct).sqrt();
        let assign40870_e53758: f64 = (var_g_0 * assign40870_e53757);
        let assign40870_e53759: f64 = (var_xbct + assign40870_e53758);
        (assign40870_e53759, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign40870_e53761;
        var_temp2_dn5 = assign40870_e53761_d_n5;
        var_temp2_dn6 = assign40870_e53761_d_n6;
        var_temp2_dn7 = assign40870_e53761_d_n7;
        var_temp2_dn8 = assign40870_e53761_d_n8;

        let (assign40880_e53779, assign40880_e53779_d_n5, assign40880_e53779_d_n6, assign40880_e53779_d_n7, assign40880_e53779_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40880_e53765: f64 = (var_xgct - var_temp2);
        let assign40880_e53767: f64 = (assign40880_e53765 / var_temp1);
        let assign40880_e53770: f64 = (0.5 * var_xbct);
        let assign40880_e53771: f64 = (assign40880_e53767 + assign40880_e53770);
        let assign40880_e53774: f64 = (1.0 + var_ctb_i);
        let assign40880_e53776: f64 = (assign40880_e53774 * var_xsbstar);
        let assign40880_e53777: f64 = (assign40880_e53771 - assign40880_e53776);
        (assign40880_e53777, (((((var_xgct_dn5 - var_temp2_dn5) * var_temp1) - (assign40880_e53765 * var_temp1_dn5)) / (var_temp1 * var_temp1)) - (assign40880_e53774 * var_xsbstar_dn5)), (((((var_xgct_dn6 - var_temp2_dn6) * var_temp1) - (assign40880_e53765 * var_temp1_dn6)) / (var_temp1 * var_temp1)) - (assign40880_e53774 * var_xsbstar_dn6)), (((((var_xgct_dn7 - var_temp2_dn7) * var_temp1) - (assign40880_e53765 * var_temp1_dn7)) / (var_temp1 * var_temp1)) - (assign40880_e53774 * var_xsbstar_dn7)), (((((var_xgct_dn8 - var_temp2_dn8) * var_temp1) - (assign40880_e53765 * var_temp1_dn8)) / (var_temp1 * var_temp1)) - (assign40880_e53774 * var_xsbstar_dn8)),)
    } else {
        (var_xwict, var_xwict_dn5, var_xwict_dn6, var_xwict_dn7, var_xwict_dn8,)
    }
};
        var_xwict = assign40880_e53779;
        var_xwict_dn5 = assign40880_e53779_d_n5;
        var_xwict_dn6 = assign40880_e53779_d_n6;
        var_xwict_dn7 = assign40880_e53779_d_n7;
        var_xwict_dn8 = assign40880_e53779_d_n8;

        let (assign40890_e53787,) = {
    if (var_guard1173 != 0.0) {
        let assign40890_e53783: f64 = (0.5 * var_xbct);
        let assign40890_e53785: f64 = (assign40890_e53783 + 2.0);
        (assign40890_e53785,)
    } else {
        (var_xctmax,)
    }
};
        var_xctmax = assign40890_e53787;

        let (assign40900_e53793, assign40900_e53793_d_n5, assign40900_e53793_d_n6, assign40900_e53793_d_n7, assign40900_e53793_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40900_e53791: f64 = (var_xbct + var_xsbstar);
        (assign40900_e53791, var_xsbstar_dn5, var_xsbstar_dn6, var_xsbstar_dn7, var_xsbstar_dn8,)
    } else {
        (var_xnct, var_xnct_dn5, var_xnct_dn6, var_xnct_dn7, var_xnct_dn8,)
    }
};
        var_xnct = assign40900_e53793;
        var_xnct_dn5 = assign40900_e53793_d_n5;
        var_xnct_dn6 = assign40900_e53793_d_n6;
        var_xnct_dn7 = assign40900_e53793_d_n7;
        var_xnct_dn8 = assign40900_e53793_d_n8;

        let (assign40910_e53814, assign40910_e53814_d_n5, assign40910_e53814_d_n6, assign40910_e53814_d_n7, assign40910_e53814_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40910_e53797: f64 = (var_xgct - var_xnct);
        let assign40910_e53800: f64 = (var_xnct).sqrt();
        let assign40910_e53801: f64 = (var_g_0 * assign40910_e53800);
        let assign40910_e53802: f64 = (assign40910_e53797 - assign40910_e53801);
        let assign40910_e53806: f64 = (var_xbct / var_g_0);
        let assign40910_e53808: f64 = (var_xbct).sqrt();
        let assign40910_e53809: f64 = (assign40910_e53806 + assign40910_e53808);
        let assign40910_e53810: f64 = (assign40910_e53809).ln();
        let assign40910_e53811: f64 = (2.0 * assign40910_e53810);
        let assign40910_e53812: f64 = (assign40910_e53802 - assign40910_e53811);
        (assign40910_e53812, ((var_xgct_dn5 - var_xnct_dn5) - (var_g_0 * (var_xnct_dn5 / (2.0 * assign40910_e53800)))), ((var_xgct_dn6 - var_xnct_dn6) - (var_g_0 * (var_xnct_dn6 / (2.0 * assign40910_e53800)))), ((var_xgct_dn7 - var_xnct_dn7) - (var_g_0 * (var_xnct_dn7 / (2.0 * assign40910_e53800)))), ((var_xgct_dn8 - var_xnct_dn8) - (var_g_0 * (var_xnct_dn8 / (2.0 * assign40910_e53800)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign40910_e53814;
        var_temp1_dn5 = assign40910_e53814_d_n5;
        var_temp1_dn6 = assign40910_e53814_d_n6;
        var_temp1_dn7 = assign40910_e53814_d_n7;
        var_temp1_dn8 = assign40910_e53814_d_n8;

        let (assign40920_e53822, assign40920_e53822_d_n5, assign40920_e53822_d_n6, assign40920_e53822_d_n7, assign40920_e53822_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40920_e53818: f64 = (2.0 * var_temp1);
        let assign40920_e53820: f64 = (assign40920_e53818 + var_xctmax);
        (assign40920_e53820, (2.0 * var_temp1_dn5), (2.0 * var_temp1_dn6), (2.0 * var_temp1_dn7), (2.0 * var_temp1_dn8),)
    } else {
        (var_xmict, var_xmict_dn5, var_xmict_dn6, var_xmict_dn7, var_xmict_dn8,)
    }
};
        var_xmict = assign40920_e53822;
        var_xmict_dn5 = assign40920_e53822_d_n5;
        var_xmict_dn6 = assign40920_e53822_d_n6;
        var_xmict_dn7 = assign40920_e53822_d_n7;
        var_xmict_dn8 = assign40920_e53822_d_n8;

        let (assign40930_e53841, assign40930_e53841_d_n5, assign40930_e53841_d_n6, assign40930_e53841_d_n7, assign40930_e53841_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40930_e53827: f64 = (var_xwict + var_xmict);
        let assign40930_e53830: f64 = (var_xwict - var_xmict);
        let assign40930_e53833: f64 = (var_xwict - var_xmict);
        let assign40930_e53834: f64 = (assign40930_e53830 * assign40930_e53833);
        let assign40930_e53836: f64 = (assign40930_e53834 + 20.0);
        let assign40930_e53837: f64 = (assign40930_e53836).sqrt();
        let assign40930_e53838: f64 = (assign40930_e53827 + assign40930_e53837);
        let assign40930_e53839: f64 = (0.5 * assign40930_e53838);
        (assign40930_e53839, (0.5 * ((var_xwict_dn5 + var_xmict_dn5) + ((((var_xwict_dn5 - var_xmict_dn5) * assign40930_e53833) + (assign40930_e53830 * (var_xwict_dn5 - var_xmict_dn5))) / (2.0 * assign40930_e53837)))), (0.5 * ((var_xwict_dn6 + var_xmict_dn6) + ((((var_xwict_dn6 - var_xmict_dn6) * assign40930_e53833) + (assign40930_e53830 * (var_xwict_dn6 - var_xmict_dn6))) / (2.0 * assign40930_e53837)))), (0.5 * ((var_xwict_dn7 + var_xmict_dn7) + ((((var_xwict_dn7 - var_xmict_dn7) * assign40930_e53833) + (assign40930_e53830 * (var_xwict_dn7 - var_xmict_dn7))) / (2.0 * assign40930_e53837)))), (0.5 * ((var_xwict_dn8 + var_xmict_dn8) + ((((var_xwict_dn8 - var_xmict_dn8) * assign40930_e53833) + (assign40930_e53830 * (var_xwict_dn8 - var_xmict_dn8))) / (2.0 * assign40930_e53837)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign40930_e53841;
        var_temp1_dn5 = assign40930_e53841_d_n5;
        var_temp1_dn6 = assign40930_e53841_d_n6;
        var_temp1_dn7 = assign40930_e53841_d_n7;
        var_temp1_dn8 = assign40930_e53841_d_n8;

        let (assign40940_e53851, assign40940_e53851_d_n5, assign40940_e53851_d_n6, assign40940_e53851_d_n7, assign40940_e53851_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40940_e53846: f64 = (var_xgct - var_xsbstar);
        let assign40940_e53847: f64 = (2.0 * assign40940_e53846);
        let assign40940_e53849: f64 = (assign40940_e53847 - var_xctmax);
        (assign40940_e53849, (2.0 * (var_xgct_dn5 - var_xsbstar_dn5)), (2.0 * (var_xgct_dn6 - var_xsbstar_dn6)), (2.0 * (var_xgct_dn7 - var_xsbstar_dn7)), (2.0 * (var_xgct_dn8 - var_xsbstar_dn8)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign40940_e53851;
        var_temp2_dn5 = assign40940_e53851_d_n5;
        var_temp2_dn6 = assign40940_e53851_d_n6;
        var_temp2_dn7 = assign40940_e53851_d_n7;
        var_temp2_dn8 = assign40940_e53851_d_n8;

        let (assign40950_e53870, assign40950_e53870_d_n5, assign40950_e53870_d_n6, assign40950_e53870_d_n7, assign40950_e53870_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40950_e53856: f64 = (var_temp1 + var_temp2);
        let assign40950_e53859: f64 = (var_temp1 - var_temp2);
        let assign40950_e53862: f64 = (var_temp1 - var_temp2);
        let assign40950_e53863: f64 = (assign40950_e53859 * assign40950_e53862);
        let assign40950_e53865: f64 = (assign40950_e53863 + 20.0);
        let assign40950_e53866: f64 = (assign40950_e53865).sqrt();
        let assign40950_e53867: f64 = (assign40950_e53856 - assign40950_e53866);
        let assign40950_e53868: f64 = (0.5 * assign40950_e53867);
        (assign40950_e53868, (0.5 * ((var_temp1_dn5 + var_temp2_dn5) - ((((var_temp1_dn5 - var_temp2_dn5) * assign40950_e53862) + (assign40950_e53859 * (var_temp1_dn5 - var_temp2_dn5))) / (2.0 * assign40950_e53866)))), (0.5 * ((var_temp1_dn6 + var_temp2_dn6) - ((((var_temp1_dn6 - var_temp2_dn6) * assign40950_e53862) + (assign40950_e53859 * (var_temp1_dn6 - var_temp2_dn6))) / (2.0 * assign40950_e53866)))), (0.5 * ((var_temp1_dn7 + var_temp2_dn7) - ((((var_temp1_dn7 - var_temp2_dn7) * assign40950_e53862) + (assign40950_e53859 * (var_temp1_dn7 - var_temp2_dn7))) / (2.0 * assign40950_e53866)))), (0.5 * ((var_temp1_dn8 + var_temp2_dn8) - ((((var_temp1_dn8 - var_temp2_dn8) * assign40950_e53862) + (assign40950_e53859 * (var_temp1_dn8 - var_temp2_dn8))) / (2.0 * assign40950_e53866)))),)
    } else {
        (var_xsubct, var_xsubct_dn5, var_xsubct_dn6, var_xsubct_dn7, var_xsubct_dn8,)
    }
};
        var_xsubct = assign40950_e53870;
        var_xsubct_dn5 = assign40950_e53870_d_n5;
        var_xsubct_dn6 = assign40950_e53870_d_n6;
        var_xsubct_dn7 = assign40950_e53870_d_n7;
        var_xsubct_dn8 = assign40950_e53870_d_n8;

        let (assign40960_e53889, assign40960_e53889_d_n5, assign40960_e53889_d_n6, assign40960_e53889_d_n7, assign40960_e53889_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40960_e53875: f64 = (var_xsubct + var_xctmax);
        let assign40960_e53878: f64 = (var_xsubct - var_xctmax);
        let assign40960_e53881: f64 = (var_xsubct - var_xctmax);
        let assign40960_e53882: f64 = (assign40960_e53878 * assign40960_e53881);
        let assign40960_e53884: f64 = (assign40960_e53882 + 5.0);
        let assign40960_e53885: f64 = (assign40960_e53884).sqrt();
        let assign40960_e53886: f64 = (assign40960_e53875 - assign40960_e53885);
        let assign40960_e53887: f64 = (0.5 * assign40960_e53886);
        (assign40960_e53887, (0.5 * (var_xsubct_dn5 - (((var_xsubct_dn5 * assign40960_e53881) + (assign40960_e53878 * var_xsubct_dn5)) / (2.0 * assign40960_e53885)))), (0.5 * (var_xsubct_dn6 - (((var_xsubct_dn6 * assign40960_e53881) + (assign40960_e53878 * var_xsubct_dn6)) / (2.0 * assign40960_e53885)))), (0.5 * (var_xsubct_dn7 - (((var_xsubct_dn7 * assign40960_e53881) + (assign40960_e53878 * var_xsubct_dn7)) / (2.0 * assign40960_e53885)))), (0.5 * (var_xsubct_dn8 - (((var_xsubct_dn8 * assign40960_e53881) + (assign40960_e53878 * var_xsubct_dn8)) / (2.0 * assign40960_e53885)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign40960_e53889;
        var_temp1_dn5 = assign40960_e53889_d_n5;
        var_temp1_dn6 = assign40960_e53889_d_n6;
        var_temp1_dn7 = assign40960_e53889_d_n7;
        var_temp1_dn8 = assign40960_e53889_d_n8;

        let (assign40970_e53911, assign40970_e53911_d_n5, assign40970_e53911_d_n6, assign40970_e53911_d_n7, assign40970_e53911_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40970_e53894: f64 = (-var_xctmax);
        let assign40970_e53895: f64 = (var_temp1 + assign40970_e53894);
        let assign40970_e53898: f64 = (-var_xctmax);
        let assign40970_e53899: f64 = (var_temp1 - assign40970_e53898);
        let assign40970_e53902: f64 = (-var_xctmax);
        let assign40970_e53903: f64 = (var_temp1 - assign40970_e53902);
        let assign40970_e53904: f64 = (assign40970_e53899 * assign40970_e53903);
        let assign40970_e53906: f64 = (assign40970_e53904 + 20.0);
        let assign40970_e53907: f64 = (assign40970_e53906).sqrt();
        let assign40970_e53908: f64 = (assign40970_e53895 + assign40970_e53907);
        let assign40970_e53909: f64 = (0.5 * assign40970_e53908);
        (assign40970_e53909, (0.5 * (var_temp1_dn5 + (((var_temp1_dn5 * assign40970_e53903) + (assign40970_e53899 * var_temp1_dn5)) / (2.0 * assign40970_e53907)))), (0.5 * (var_temp1_dn6 + (((var_temp1_dn6 * assign40970_e53903) + (assign40970_e53899 * var_temp1_dn6)) / (2.0 * assign40970_e53907)))), (0.5 * (var_temp1_dn7 + (((var_temp1_dn7 * assign40970_e53903) + (assign40970_e53899 * var_temp1_dn7)) / (2.0 * assign40970_e53907)))), (0.5 * (var_temp1_dn8 + (((var_temp1_dn8 * assign40970_e53903) + (assign40970_e53899 * var_temp1_dn8)) / (2.0 * assign40970_e53907)))),)
    } else {
        (var_xct, var_xct_dn5, var_xct_dn6, var_xct_dn7, var_xct_dn8,)
    }
};
        var_xct = assign40970_e53911;
        var_xct_dn5 = assign40970_e53911_d_n5;
        var_xct_dn6 = assign40970_e53911_d_n6;
        var_xct_dn7 = assign40970_e53911_d_n7;
        var_xct_dn8 = assign40970_e53911_d_n8;

        let (assign40980_e53921, assign40980_e53921_d_n5, assign40980_e53921_d_n6, assign40980_e53921_d_n7, assign40980_e53921_d_n8,) = {
    if (var_guard1173 != 0.0) {
        let assign40980_e53916: f64 = (var_xct / var_xctmax);
        let assign40980_e53918: f64 = (assign40980_e53916 + 1.0);
        let assign40980_e53919: f64 = (var_ctg_t * assign40980_e53918);
        (assign40980_e53919, (var_ctg_t * (var_xct_dn5 / var_xctmax)), (var_ctg_t * (var_xct_dn6 / var_xctmax)), (var_ctg_t * (var_xct_dn7 / var_xctmax)), (var_ctg_t * (var_xct_dn8 / var_xctmax)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign40980_e53921;
        var_temp2_dn5 = assign40980_e53921_d_n5;
        var_temp2_dn6 = assign40980_e53921_d_n6;
        var_temp2_dn7 = assign40980_e53921_d_n7;
        var_temp2_dn8 = assign40980_e53921_d_n8;

        let assign40990_e53924: f64 = (-230.25850929940458);
        let assign40990_e53925: f64 = if var_temp2 > assign40990_e53924 { 1.0 } else { 0.0 };
        var_guard1174 = assign40990_e53925;

        let (assign41000_e53932, assign41000_e53932_d_n5, assign41000_e53932_d_n6, assign41000_e53932_d_n7, assign41000_e53932_d_n8,) = {
    if ((var_guard1173 != 0.0) && (var_guard1174 != 0.0)) {
        let assign41000_e53930: f64 = (var_temp2).exp();
        (assign41000_e53930, (assign41000_e53930 * var_temp2_dn5), (assign41000_e53930 * var_temp2_dn6), (assign41000_e53930 * var_temp2_dn7), (assign41000_e53930 * var_temp2_dn8),)
    } else {
        (var_dctg, var_dctg_dn5, var_dctg_dn6, var_dctg_dn7, var_dctg_dn8,)
    }
};
        var_dctg = assign41000_e53932;
        var_dctg_dn5 = assign41000_e53932_d_n5;
        var_dctg_dn6 = assign41000_e53932_d_n6;
        var_dctg_dn7 = assign41000_e53932_d_n7;
        var_dctg_dn8 = assign41000_e53932_d_n8;

        let (assign41010_e53964, assign41010_e53964_d_n5, assign41010_e53964_d_n6, assign41010_e53964_d_n7, assign41010_e53964_d_n8,) = {
    if ((var_guard1173 != 0.0) && (var_guard1174 == 0.0)) {
        let assign41010_e53940: f64 = (-230.25850929940458);
        let assign41010_e53942: f64 = (assign41010_e53940 - var_temp2);
        let assign41010_e53946: f64 = (-230.25850929940458);
        let assign41010_e53948: f64 = (assign41010_e53946 - var_temp2);
        let assign41010_e53951: f64 = (-230.25850929940458);
        let assign41010_e53953: f64 = (assign41010_e53951 - var_temp2);
        let assign41010_e53955: f64 = (assign41010_e53953 * 0.3333333333333333);
        let assign41010_e53956: f64 = (1.0 + assign41010_e53955);
        let assign41010_e53957: f64 = (assign41010_e53948 * assign41010_e53956);
        let assign41010_e53958: f64 = (0.5 * assign41010_e53957);
        let assign41010_e53959: f64 = (1.0 + assign41010_e53958);
        let assign41010_e53960: f64 = (assign41010_e53942 * assign41010_e53959);
        let assign41010_e53961: f64 = (1.0 + assign41010_e53960);
        let assign41010_e53962: f64 = (1e-100 / assign41010_e53961);
        (assign41010_e53962, (-((1e-100 * (((-var_temp2_dn5) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-var_temp2_dn5) * assign41010_e53956) + (assign41010_e53948 * ((-var_temp2_dn5) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-var_temp2_dn6) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-var_temp2_dn6) * assign41010_e53956) + (assign41010_e53948 * ((-var_temp2_dn6) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-var_temp2_dn7) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-var_temp2_dn7) * assign41010_e53956) + (assign41010_e53948 * ((-var_temp2_dn7) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-var_temp2_dn8) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-var_temp2_dn8) * assign41010_e53956) + (assign41010_e53948 * ((-var_temp2_dn8) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))),)
    } else {
        (var_dctg, var_dctg_dn5, var_dctg_dn6, var_dctg_dn7, var_dctg_dn8,)
    }
};
        var_dctg = assign41010_e53964;
        var_dctg_dn5 = assign41010_e53964_d_n5;
        var_dctg_dn6 = assign41010_e53964_d_n6;
        var_dctg_dn7 = assign41010_e53964_d_n7;
        var_dctg_dn8 = assign41010_e53964_d_n8;

        let assign41020_e53968: f64 = (var_ct_t * var_dctg);
        let assign41020_e53969: f64 = (1.0 + assign41020_e53968);
        var_ct_fact = assign41020_e53969;
        var_ct_fact_dn5 = (var_ct_t * var_dctg_dn5);
        var_ct_fact_dn6 = (var_ct_t * var_dctg_dn6);
        var_ct_fact_dn7 = (var_ct_t * var_dctg_dn7);
        var_ct_fact_dn8 = (var_ct_t * var_dctg_dn8);

        let assign41030_e53972: f64 = (var_phit * var_ct_fact);
        var_phitct = assign41030_e53972;
        var_phitct_dn5 = (var_phit * var_ct_fact_dn5);
        var_phitct_dn6 = (var_phit * var_ct_fact_dn6);
        var_phitct_dn7 = (var_phit * var_ct_fact_dn7);
        var_phitct_dn8 = (var_phit * var_ct_fact_dn8);

        let assign41040_e53977: f64 = (var_psced_i * var_vdsx);
        let assign41040_e53978: f64 = (1.0 + assign41040_e53977);
        let assign41040_e53979: f64 = (var_psce_i * assign41040_e53978);
        let assign41040_e53983: f64 = (var_psceb_i * var_vsbx);
        let assign41040_e53984: f64 = (1.0 + assign41040_e53983);
        let assign41040_e53985: f64 = (assign41040_e53979 * assign41040_e53984);
        var_dphit1 = assign41040_e53985;
        var_dphit1_dn5 = (assign41040_e53979 * (var_psceb_i * var_vsbx_dn5));
        var_dphit1_dn6 = (((var_psce_i * (var_psced_i * var_vdsx_dn6)) * assign41040_e53984) + (assign41040_e53979 * (var_psceb_i * var_vsbx_dn6)));
        var_dphit1_dn7 = (((var_psce_i * (var_psced_i * var_vdsx_dn7)) * assign41040_e53984) + (assign41040_e53979 * (var_psceb_i * var_vsbx_dn7)));
        var_dphit1_dn8 = (assign41040_e53979 * (var_psceb_i * var_vsbx_dn8));

        let assign41050_e53989: f64 = (1.0 + var_dphit1);
        let assign41050_e53990: f64 = (var_phitct * assign41050_e53989);
        var_phit1 = assign41050_e53990;
        var_phit1_dn5 = ((var_phitct_dn5 * assign41050_e53989) + (var_phitct * var_dphit1_dn5));
        var_phit1_dn6 = ((var_phitct_dn6 * assign41050_e53989) + (var_phitct * var_dphit1_dn6));
        var_phit1_dn7 = ((var_phitct_dn7 * assign41050_e53989) + (var_phitct * var_dphit1_dn7));
        var_phit1_dn8 = ((var_phitct_dn8 * assign41050_e53989) + (var_phitct * var_dphit1_dn8));

        let assign41060_e53993: f64 = (1.0 / var_phit1);
        var_inv_phit1 = assign41060_e53993;
        var_inv_phit1_dn5 = (-(var_phit1_dn5 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn6 = (-(var_phit1_dn6 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn7 = (-(var_phit1_dn7 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn8 = (-(var_phit1_dn8 / (var_phit1 * var_phit1)));

        let assign41070_e53997: f64 = (var_phit * var_inv_phit1);
        let assign41070_e53998: f64 = (assign41070_e53997).sqrt();
        let assign41070_e53999: f64 = (var_g_0 * assign41070_e53998);
        var_gf = assign41070_e53999;
        var_gf_dn5 = (var_g_0 * ((var_phit * var_inv_phit1_dn5) / (2.0 * assign41070_e53998)));
        var_gf_dn6 = (var_g_0 * ((var_phit * var_inv_phit1_dn6) / (2.0 * assign41070_e53998)));
        var_gf_dn7 = (var_g_0 * ((var_phit * var_inv_phit1_dn7) / (2.0 * assign41070_e53998)));
        var_gf_dn8 = (var_g_0 * ((var_phit * var_inv_phit1_dn8) / (2.0 * assign41070_e53998)));

        let assign41080_e54002: f64 = (var_gf * var_gf);
        var_gf2 = assign41080_e54002;
        var_gf2_dn5 = ((var_gf_dn5 * var_gf) + (var_gf * var_gf_dn5));
        var_gf2_dn6 = ((var_gf_dn6 * var_gf) + (var_gf * var_gf_dn6));
        var_gf2_dn7 = ((var_gf_dn7 * var_gf) + (var_gf * var_gf_dn7));
        var_gf2_dn8 = ((var_gf_dn8 * var_gf) + (var_gf * var_gf_dn8));

        let assign41090_e54005: f64 = (1.0 / var_gf2);
        var_inv_gf2 = assign41090_e54005;
        var_inv_gf2_dn5 = (-(var_gf2_dn5 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn6 = (-(var_gf2_dn6 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn7 = (-(var_gf2_dn7 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn8 = (-(var_gf2_dn8 / (var_gf2 * var_gf2)));

        let assign41100_e54008: f64 = (var_vsbstar * var_inv_phit1);
        var_ux = assign41100_e54008;
        var_ux_dn5 = ((var_vsbstar_dn5 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn5));
        var_ux_dn6 = ((var_vsbstar_dn6 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn6));
        var_ux_dn7 = ((var_vsbstar_dn7 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn7));
        var_ux_dn8 = ((var_vsbstar_dn8 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn8));

        let assign41110_e54011: f64 = (var_vgb1 * var_inv_phit1);
        var_xg = assign41110_e54011;
        var_xg_dn5 = ((var_vgb1_dn5 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn5));
        var_xg_dn6 = ((var_vgb1_dn6 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn6));
        var_xg_dn7 = ((var_vgb1_dn7 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn7));
        var_xg_dn8 = ((var_vgb1_dn8 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn8));

        let assign41120_e54014: f64 = (2.0 * var_vdsx);
        let assign41120_e54019: f64 = (var_cfd_i * var_vdsx);
        let assign41120_e54020: f64 = (1.0 + assign41120_e54019);
        let assign41120_e54021: f64 = (assign41120_e54020).sqrt();
        let assign41120_e54022: f64 = (1.0 + assign41120_e54021);
        let assign41120_e54023: f64 = (assign41120_e54014 / assign41120_e54022);
        var_vdsp = assign41120_e54023;
        var_vdsp_dn6 = ((((2.0 * var_vdsx_dn6) * assign41120_e54022) - (assign41120_e54014 * ((var_cfd_i * var_vdsx_dn6) / (2.0 * assign41120_e54021)))) / (assign41120_e54022 * assign41120_e54022));
        var_vdsp_dn7 = ((((2.0 * var_vdsx_dn7) * assign41120_e54022) - (assign41120_e54014 * ((var_cfd_i * var_vdsx_dn7) / (2.0 * assign41120_e54021)))) / (assign41120_e54022 * assign41120_e54022));

        let assign41130_e54026: f64 = (var_cf_i * var_vdsp);
        let assign41130_e54030: f64 = (var_cfb_i * var_vsbx);
        let assign41130_e54031: f64 = (1.0 + assign41130_e54030);
        let assign41130_e54032: f64 = (assign41130_e54026 * assign41130_e54031);
        var_delphib = assign41130_e54032;
        var_delphib_dn5 = (assign41130_e54026 * (var_cfb_i * var_vsbx_dn5));
        var_delphib_dn6 = (((var_cf_i * var_vdsp_dn6) * assign41130_e54031) + (assign41130_e54026 * (var_cfb_i * var_vsbx_dn6)));
        var_delphib_dn7 = (((var_cf_i * var_vdsp_dn7) * assign41130_e54031) + (assign41130_e54026 * (var_cfb_i * var_vsbx_dn7)));
        var_delphib_dn8 = (assign41130_e54026 * (var_cfb_i * var_vsbx_dn8));

        let assign41140_e54035: f64 = (var_phib * var_inv_phit1);
        var_xb = assign41140_e54035;
        var_xb_dn5 = (var_phib * var_inv_phit1_dn5);
        var_xb_dn6 = (var_phib * var_inv_phit1_dn6);
        var_xb_dn7 = (var_phib * var_inv_phit1_dn7);
        var_xb_dn8 = (var_phib * var_inv_phit1_dn8);

        let assign41150_e54038: f64 = (var_v_xb * var_v_xb);
        let assign41150_e54040: f64 = (assign41150_e54038 + var_aphi);
        let assign41150_e54041: f64 = (assign41150_e54040).sqrt();
        var_temp1 = assign41150_e54041;
        var_temp1_dn5 = 0.0;
        var_temp1_dn6 = (((var_v_xb_dn6 * var_v_xb) + (var_v_xb * var_v_xb_dn6)) / (2.0 * assign41150_e54041));
        var_temp1_dn7 = (((var_v_xb_dn7 * var_v_xb) + (var_v_xb * var_v_xb_dn7)) / (2.0 * assign41150_e54041));
        var_temp1_dn8 = (((var_v_xb_dn8 * var_v_xb) + (var_v_xb * var_v_xb_dn8)) / (2.0 * assign41150_e54041));

        let assign41160_e54044: f64 = (var_v_xb - var_delphib);
        let assign41160_e54047: f64 = (var_v_xb - var_delphib);
        let assign41160_e54048: f64 = (assign41160_e54044 * assign41160_e54047);
        let assign41160_e54050: f64 = (assign41160_e54048 + var_aphi);
        let assign41160_e54051: f64 = (assign41160_e54050).sqrt();
        var_temp2 = assign41160_e54051;
        var_temp2_dn5 = ((((-var_delphib_dn5) * assign41160_e54047) + (assign41160_e54044 * (-var_delphib_dn5))) / (2.0 * assign41160_e54051));
        var_temp2_dn6 = ((((var_v_xb_dn6 - var_delphib_dn6) * assign41160_e54047) + (assign41160_e54044 * (var_v_xb_dn6 - var_delphib_dn6))) / (2.0 * assign41160_e54051));
        var_temp2_dn7 = ((((var_v_xb_dn7 - var_delphib_dn7) * assign41160_e54047) + (assign41160_e54044 * (var_v_xb_dn7 - var_delphib_dn7))) / (2.0 * assign41160_e54051));
        var_temp2_dn8 = ((((var_v_xb_dn8 - var_delphib_dn8) * assign41160_e54047) + (assign41160_e54044 * (var_v_xb_dn8 - var_delphib_dn8))) / (2.0 * assign41160_e54051));

        let assign41170_e54054: f64 = (0.5 * var_inv_phit1);
        let assign41170_e54057: f64 = (var_delphib + var_temp1);
        let assign41170_e54059: f64 = (assign41170_e54057 - var_temp2);
        let assign41170_e54060: f64 = (assign41170_e54054 * assign41170_e54059);
        var_delxb = assign41170_e54060;
        var_delxb_dn5 = (((0.5 * var_inv_phit1_dn5) * assign41170_e54059) + (assign41170_e54054 * ((var_delphib_dn5 + var_temp1_dn5) - var_temp2_dn5)));
        var_delxb_dn6 = (((0.5 * var_inv_phit1_dn6) * assign41170_e54059) + (assign41170_e54054 * ((var_delphib_dn6 + var_temp1_dn6) - var_temp2_dn6)));
        var_delxb_dn7 = (((0.5 * var_inv_phit1_dn7) * assign41170_e54059) + (assign41170_e54054 * ((var_delphib_dn7 + var_temp1_dn7) - var_temp2_dn7)));
        var_delxb_dn8 = (((0.5 * var_inv_phit1_dn8) * assign41170_e54059) + (assign41170_e54054 * ((var_delphib_dn8 + var_temp1_dn8) - var_temp2_dn8)));

        let assign41180_e54063: f64 = (var_xb + var_ux);
        var_xno_s = assign41180_e54063;
        var_xno_s_dn5 = (var_xb_dn5 + var_ux_dn5);
        var_xno_s_dn6 = (var_xb_dn6 + var_ux_dn6);
        var_xno_s_dn7 = (var_xb_dn7 + var_ux_dn7);
        var_xno_s_dn8 = (var_xb_dn8 + var_ux_dn8);

        let assign41190_e54066: f64 = (var_xno_s - var_delxb);
        var_xn_s = assign41190_e54066;
        var_xn_s_dn5 = (var_xno_s_dn5 - var_delxb_dn5);
        var_xn_s_dn6 = (var_xno_s_dn6 - var_delxb_dn6);
        var_xn_s_dn7 = (var_xno_s_dn7 - var_delxb_dn7);
        var_xn_s_dn8 = (var_xno_s_dn8 - var_delxb_dn8);

        let assign41200_e54069: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        var_guard1175 = assign41200_e54069;

        let assign41210_e54071: f64 = (var_xn_s).abs();
        let assign41210_e54073: f64 = if assign41210_e54071 < 1e-5 { 1.0 } else { 0.0 };
        var_guard1176 = assign41210_e54073;

        *var_ct_fact_slot = var_ct_fact;
        *var_ct_fact_dn5_slot = var_ct_fact_dn5;
        *var_ct_fact_dn6_slot = var_ct_fact_dn6;
        *var_ct_fact_dn7_slot = var_ct_fact_dn7;
        *var_ct_fact_dn8_slot = var_ct_fact_dn8;
        *var_dctg_slot = var_dctg;
        *var_dctg_dn5_slot = var_dctg_dn5;
        *var_dctg_dn6_slot = var_dctg_dn6;
        *var_dctg_dn7_slot = var_dctg_dn7;
        *var_dctg_dn8_slot = var_dctg_dn8;
        *var_delphib_slot = var_delphib;
        *var_delphib_dn5_slot = var_delphib_dn5;
        *var_delphib_dn6_slot = var_delphib_dn6;
        *var_delphib_dn7_slot = var_delphib_dn7;
        *var_delphib_dn8_slot = var_delphib_dn8;
        *var_delxb_slot = var_delxb;
        *var_delxb_dn5_slot = var_delxb_dn5;
        *var_delxb_dn6_slot = var_delxb_dn6;
        *var_delxb_dn7_slot = var_delxb_dn7;
        *var_delxb_dn8_slot = var_delxb_dn8;
        *var_dphit1_slot = var_dphit1;
        *var_dphit1_dn5_slot = var_dphit1_dn5;
        *var_dphit1_dn6_slot = var_dphit1_dn6;
        *var_dphit1_dn7_slot = var_dphit1_dn7;
        *var_dphit1_dn8_slot = var_dphit1_dn8;
        *var_gf_slot = var_gf;
        *var_gf2_slot = var_gf2;
        *var_gf2_dn5_slot = var_gf2_dn5;
        *var_gf2_dn6_slot = var_gf2_dn6;
        *var_gf2_dn7_slot = var_gf2_dn7;
        *var_gf2_dn8_slot = var_gf2_dn8;
        *var_gf_dn5_slot = var_gf_dn5;
        *var_gf_dn6_slot = var_gf_dn6;
        *var_gf_dn7_slot = var_gf_dn7;
        *var_gf_dn8_slot = var_gf_dn8;
        *var_guard1173_slot = var_guard1173;
        *var_guard1174_slot = var_guard1174;
        *var_guard1175_slot = var_guard1175;
        *var_guard1176_slot = var_guard1176;
        *var_inv_gf2_slot = var_inv_gf2;
        *var_inv_gf2_dn5_slot = var_inv_gf2_dn5;
        *var_inv_gf2_dn6_slot = var_inv_gf2_dn6;
        *var_inv_gf2_dn7_slot = var_inv_gf2_dn7;
        *var_inv_gf2_dn8_slot = var_inv_gf2_dn8;
        *var_inv_phit1_slot = var_inv_phit1;
        *var_inv_phit1_dn5_slot = var_inv_phit1_dn5;
        *var_inv_phit1_dn6_slot = var_inv_phit1_dn6;
        *var_inv_phit1_dn7_slot = var_inv_phit1_dn7;
        *var_inv_phit1_dn8_slot = var_inv_phit1_dn8;
        *var_phit1_slot = var_phit1;
        *var_phit1_dn5_slot = var_phit1_dn5;
        *var_phit1_dn6_slot = var_phit1_dn6;
        *var_phit1_dn7_slot = var_phit1_dn7;
        *var_phit1_dn8_slot = var_phit1_dn8;
        *var_phitct_slot = var_phitct;
        *var_phitct_dn5_slot = var_phitct_dn5;
        *var_phitct_dn6_slot = var_phitct_dn6;
        *var_phitct_dn7_slot = var_phitct_dn7;
        *var_phitct_dn8_slot = var_phitct_dn8;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_ux_slot = var_ux;
        *var_ux_dn5_slot = var_ux_dn5;
        *var_ux_dn6_slot = var_ux_dn6;
        *var_ux_dn7_slot = var_ux_dn7;
        *var_ux_dn8_slot = var_ux_dn8;
        *var_vdsp_slot = var_vdsp;
        *var_vdsp_dn6_slot = var_vdsp_dn6;
        *var_vdsp_dn7_slot = var_vdsp_dn7;
        *var_vgb1_slot = var_vgb1;
        *var_vgb1_dn5_slot = var_vgb1_dn5;
        *var_vgb1_dn6_slot = var_vgb1_dn6;
        *var_vgb1_dn7_slot = var_vgb1_dn7;
        *var_vgb1_dn8_slot = var_vgb1_dn8;
        *var_vsbx_slot = var_vsbx;
        *var_vsbx_dn5_slot = var_vsbx_dn5;
        *var_vsbx_dn6_slot = var_vsbx_dn6;
        *var_vsbx_dn7_slot = var_vsbx_dn7;
        *var_vsbx_dn8_slot = var_vsbx_dn8;
        *var_xb_slot = var_xb;
        *var_xb_dn5_slot = var_xb_dn5;
        *var_xb_dn6_slot = var_xb_dn6;
        *var_xb_dn7_slot = var_xb_dn7;
        *var_xb_dn8_slot = var_xb_dn8;
        *var_xbct_slot = var_xbct;
        *var_xct_slot = var_xct;
        *var_xct_dn5_slot = var_xct_dn5;
        *var_xct_dn6_slot = var_xct_dn6;
        *var_xct_dn7_slot = var_xct_dn7;
        *var_xct_dn8_slot = var_xct_dn8;
        *var_xctmax_slot = var_xctmax;
        *var_xg_slot = var_xg;
        *var_xg_dn5_slot = var_xg_dn5;
        *var_xg_dn6_slot = var_xg_dn6;
        *var_xg_dn7_slot = var_xg_dn7;
        *var_xg_dn8_slot = var_xg_dn8;
        *var_xgct_slot = var_xgct;
        *var_xgct_dn5_slot = var_xgct_dn5;
        *var_xgct_dn6_slot = var_xgct_dn6;
        *var_xgct_dn7_slot = var_xgct_dn7;
        *var_xgct_dn8_slot = var_xgct_dn8;
        *var_xmict_slot = var_xmict;
        *var_xmict_dn5_slot = var_xmict_dn5;
        *var_xmict_dn6_slot = var_xmict_dn6;
        *var_xmict_dn7_slot = var_xmict_dn7;
        *var_xmict_dn8_slot = var_xmict_dn8;
        *var_xn_s_slot = var_xn_s;
        *var_xn_s_dn5_slot = var_xn_s_dn5;
        *var_xn_s_dn6_slot = var_xn_s_dn6;
        *var_xn_s_dn7_slot = var_xn_s_dn7;
        *var_xn_s_dn8_slot = var_xn_s_dn8;
        *var_xnct_slot = var_xnct;
        *var_xnct_dn5_slot = var_xnct_dn5;
        *var_xnct_dn6_slot = var_xnct_dn6;
        *var_xnct_dn7_slot = var_xnct_dn7;
        *var_xnct_dn8_slot = var_xnct_dn8;
        *var_xno_s_slot = var_xno_s;
        *var_xno_s_dn5_slot = var_xno_s_dn5;
        *var_xno_s_dn6_slot = var_xno_s_dn6;
        *var_xno_s_dn7_slot = var_xno_s_dn7;
        *var_xno_s_dn8_slot = var_xno_s_dn8;
        *var_xsbstar_slot = var_xsbstar;
        *var_xsbstar_dn5_slot = var_xsbstar_dn5;
        *var_xsbstar_dn6_slot = var_xsbstar_dn6;
        *var_xsbstar_dn7_slot = var_xsbstar_dn7;
        *var_xsbstar_dn8_slot = var_xsbstar_dn8;
        *var_xsubct_slot = var_xsubct;
        *var_xsubct_dn5_slot = var_xsubct_dn5;
        *var_xsubct_dn6_slot = var_xsubct_dn6;
        *var_xsubct_dn7_slot = var_xsubct_dn7;
        *var_xsubct_dn8_slot = var_xsubct_dn8;
        *var_xwict_slot = var_xwict;
        *var_xwict_dn5_slot = var_xwict_dn5;
        *var_xwict_dn6_slot = var_xwict_dn6;
        *var_xwict_dn7_slot = var_xwict_dn7;
        *var_xwict_dn8_slot = var_xwict_dn8;
    }

    pub(super) fn stamp_transient_block_88(
        var_delxb: f64,
        var_delxb_dn5: f64,
        var_delxb_dn6: f64,
        var_delxb_dn7: f64,
        var_delxb_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1175: f64,
        var_guard1176: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xno_s: f64,
        var_xno_s_dn5: f64,
        var_xno_s_dn6: f64,
        var_xno_s_dn7: f64,
        var_xno_s_dn8: f64,
        var_delta_ns_slot: &mut f64,
        var_delta_ns_dn5_slot: &mut f64,
        var_delta_ns_dn6_slot: &mut f64,
        var_delta_ns_dn7_slot: &mut f64,
        var_delta_ns_dn8_slot: &mut f64,
        var_dscr0_slot: &mut f64,
        var_dscr0_dn5_slot: &mut f64,
        var_dscr0_dn6_slot: &mut f64,
        var_dscr0_dn7_slot: &mut f64,
        var_dscr0_dn8_slot: &mut f64,
        var_fscr_slot: &mut f64,
        var_fscr_dn5_slot: &mut f64,
        var_fscr_dn6_slot: &mut f64,
        var_fscr_dn7_slot: &mut f64,
        var_fscr_dn8_slot: &mut f64,
        var_guard1177_slot: &mut f64,
        var_guard1178_slot: &mut f64,
        var_guard1179_slot: &mut f64,
        var_guard1180_slot: &mut f64,
        var_guard1181_slot: &mut f64,
        var_guard1182_slot: &mut f64,
        var_inv_xi_slot: &mut f64,
        var_inv_xi_dn5_slot: &mut f64,
        var_inv_xi_dn6_slot: &mut f64,
        var_inv_xi_dn7_slot: &mut f64,
        var_inv_xi_dn8_slot: &mut f64,
        var_margin_slot: &mut f64,
        var_nscr_slot: &mut f64,
        var_nscr_dn5_slot: &mut f64,
        var_nscr_dn6_slot: &mut f64,
        var_nscr_dn7_slot: &mut f64,
        var_nscr_dn8_slot: &mut f64,
        var_qbscr_slot: &mut f64,
        var_qbscr_dn5_slot: &mut f64,
        var_qbscr_dn6_slot: &mut f64,
        var_qbscr_dn7_slot: &mut f64,
        var_qbscr_dn8_slot: &mut f64,
        var_qiscr_slot: &mut f64,
        var_qiscr0_slot: &mut f64,
        var_qiscr0_dn5_slot: &mut f64,
        var_qiscr0_dn6_slot: &mut f64,
        var_qiscr0_dn7_slot: &mut f64,
        var_qiscr0_dn8_slot: &mut f64,
        var_qiscr0si_slot: &mut f64,
        var_qiscr0si_dn5_slot: &mut f64,
        var_qiscr0si_dn6_slot: &mut f64,
        var_qiscr0si_dn7_slot: &mut f64,
        var_qiscr0si_dn8_slot: &mut f64,
        var_qiscr_dn5_slot: &mut f64,
        var_qiscr_dn6_slot: &mut f64,
        var_qiscr_dn7_slot: &mut f64,
        var_qiscr_dn8_slot: &mut f64,
        var_sp_s_x1_slot: &mut f64,
        var_sp_s_x1_dn5_slot: &mut f64,
        var_sp_s_x1_dn6_slot: &mut f64,
        var_sp_s_x1_dn7_slot: &mut f64,
        var_sp_s_x1_dn8_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_x_s_slot: &mut f64,
        var_x_s_dn5_slot: &mut f64,
        var_x_s_dn6_slot: &mut f64,
        var_x_s_dn7_slot: &mut f64,
        var_x_s_dn8_slot: &mut f64,
        var_xgtscr_slot: &mut f64,
        var_xgtscr0_slot: &mut f64,
        var_xgtscr0_dn5_slot: &mut f64,
        var_xgtscr0_dn6_slot: &mut f64,
        var_xgtscr0_dn7_slot: &mut f64,
        var_xgtscr0_dn8_slot: &mut f64,
        var_xgtscr_dn5_slot: &mut f64,
        var_xgtscr_dn6_slot: &mut f64,
        var_xgtscr_dn7_slot: &mut f64,
        var_xgtscr_dn8_slot: &mut f64,
        var_xi_slot: &mut f64,
        var_xi_dn5_slot: &mut f64,
        var_xi_dn6_slot: &mut f64,
        var_xi_dn7_slot: &mut f64,
        var_xi_dn8_slot: &mut f64,
        var_xn_s_slot: &mut f64,
        var_xn_s_dn5_slot: &mut f64,
        var_xn_s_dn6_slot: &mut f64,
        var_xn_s_dn7_slot: &mut f64,
        var_xn_s_dn8_slot: &mut f64,
        var_xthscr_slot: &mut f64,
        var_xthscr_dn5_slot: &mut f64,
        var_xthscr_dn6_slot: &mut f64,
        var_xthscr_dn7_slot: &mut f64,
        var_xthscr_dn8_slot: &mut f64,
    ) {
        let mut var_delta_ns: f64 = *var_delta_ns_slot;
        let mut var_delta_ns_dn5: f64 = *var_delta_ns_dn5_slot;
        let mut var_delta_ns_dn6: f64 = *var_delta_ns_dn6_slot;
        let mut var_delta_ns_dn7: f64 = *var_delta_ns_dn7_slot;
        let mut var_delta_ns_dn8: f64 = *var_delta_ns_dn8_slot;
        let mut var_dscr0: f64 = *var_dscr0_slot;
        let mut var_dscr0_dn5: f64 = *var_dscr0_dn5_slot;
        let mut var_dscr0_dn6: f64 = *var_dscr0_dn6_slot;
        let mut var_dscr0_dn7: f64 = *var_dscr0_dn7_slot;
        let mut var_dscr0_dn8: f64 = *var_dscr0_dn8_slot;
        let mut var_fscr: f64 = *var_fscr_slot;
        let mut var_fscr_dn5: f64 = *var_fscr_dn5_slot;
        let mut var_fscr_dn6: f64 = *var_fscr_dn6_slot;
        let mut var_fscr_dn7: f64 = *var_fscr_dn7_slot;
        let mut var_fscr_dn8: f64 = *var_fscr_dn8_slot;
        let mut var_guard1177: f64 = *var_guard1177_slot;
        let mut var_guard1178: f64 = *var_guard1178_slot;
        let mut var_guard1179: f64 = *var_guard1179_slot;
        let mut var_guard1180: f64 = *var_guard1180_slot;
        let mut var_guard1181: f64 = *var_guard1181_slot;
        let mut var_guard1182: f64 = *var_guard1182_slot;
        let mut var_inv_xi: f64 = *var_inv_xi_slot;
        let mut var_inv_xi_dn5: f64 = *var_inv_xi_dn5_slot;
        let mut var_inv_xi_dn6: f64 = *var_inv_xi_dn6_slot;
        let mut var_inv_xi_dn7: f64 = *var_inv_xi_dn7_slot;
        let mut var_inv_xi_dn8: f64 = *var_inv_xi_dn8_slot;
        let mut var_margin: f64 = *var_margin_slot;
        let mut var_nscr: f64 = *var_nscr_slot;
        let mut var_nscr_dn5: f64 = *var_nscr_dn5_slot;
        let mut var_nscr_dn6: f64 = *var_nscr_dn6_slot;
        let mut var_nscr_dn7: f64 = *var_nscr_dn7_slot;
        let mut var_nscr_dn8: f64 = *var_nscr_dn8_slot;
        let mut var_qbscr: f64 = *var_qbscr_slot;
        let mut var_qbscr_dn5: f64 = *var_qbscr_dn5_slot;
        let mut var_qbscr_dn6: f64 = *var_qbscr_dn6_slot;
        let mut var_qbscr_dn7: f64 = *var_qbscr_dn7_slot;
        let mut var_qbscr_dn8: f64 = *var_qbscr_dn8_slot;
        let mut var_qiscr: f64 = *var_qiscr_slot;
        let mut var_qiscr0: f64 = *var_qiscr0_slot;
        let mut var_qiscr0_dn5: f64 = *var_qiscr0_dn5_slot;
        let mut var_qiscr0_dn6: f64 = *var_qiscr0_dn6_slot;
        let mut var_qiscr0_dn7: f64 = *var_qiscr0_dn7_slot;
        let mut var_qiscr0_dn8: f64 = *var_qiscr0_dn8_slot;
        let mut var_qiscr0si: f64 = *var_qiscr0si_slot;
        let mut var_qiscr0si_dn5: f64 = *var_qiscr0si_dn5_slot;
        let mut var_qiscr0si_dn6: f64 = *var_qiscr0si_dn6_slot;
        let mut var_qiscr0si_dn7: f64 = *var_qiscr0si_dn7_slot;
        let mut var_qiscr0si_dn8: f64 = *var_qiscr0si_dn8_slot;
        let mut var_qiscr_dn5: f64 = *var_qiscr_dn5_slot;
        let mut var_qiscr_dn6: f64 = *var_qiscr_dn6_slot;
        let mut var_qiscr_dn7: f64 = *var_qiscr_dn7_slot;
        let mut var_qiscr_dn8: f64 = *var_qiscr_dn8_slot;
        let mut var_sp_s_x1: f64 = *var_sp_s_x1_slot;
        let mut var_sp_s_x1_dn5: f64 = *var_sp_s_x1_dn5_slot;
        let mut var_sp_s_x1_dn6: f64 = *var_sp_s_x1_dn6_slot;
        let mut var_sp_s_x1_dn7: f64 = *var_sp_s_x1_dn7_slot;
        let mut var_sp_s_x1_dn8: f64 = *var_sp_s_x1_dn8_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_x_s: f64 = *var_x_s_slot;
        let mut var_x_s_dn5: f64 = *var_x_s_dn5_slot;
        let mut var_x_s_dn6: f64 = *var_x_s_dn6_slot;
        let mut var_x_s_dn7: f64 = *var_x_s_dn7_slot;
        let mut var_x_s_dn8: f64 = *var_x_s_dn8_slot;
        let mut var_xgtscr: f64 = *var_xgtscr_slot;
        let mut var_xgtscr0: f64 = *var_xgtscr0_slot;
        let mut var_xgtscr0_dn5: f64 = *var_xgtscr0_dn5_slot;
        let mut var_xgtscr0_dn6: f64 = *var_xgtscr0_dn6_slot;
        let mut var_xgtscr0_dn7: f64 = *var_xgtscr0_dn7_slot;
        let mut var_xgtscr0_dn8: f64 = *var_xgtscr0_dn8_slot;
        let mut var_xgtscr_dn5: f64 = *var_xgtscr_dn5_slot;
        let mut var_xgtscr_dn6: f64 = *var_xgtscr_dn6_slot;
        let mut var_xgtscr_dn7: f64 = *var_xgtscr_dn7_slot;
        let mut var_xgtscr_dn8: f64 = *var_xgtscr_dn8_slot;
        let mut var_xi: f64 = *var_xi_slot;
        let mut var_xi_dn5: f64 = *var_xi_dn5_slot;
        let mut var_xi_dn6: f64 = *var_xi_dn6_slot;
        let mut var_xi_dn7: f64 = *var_xi_dn7_slot;
        let mut var_xi_dn8: f64 = *var_xi_dn8_slot;
        let mut var_xn_s: f64 = *var_xn_s_slot;
        let mut var_xn_s_dn5: f64 = *var_xn_s_dn5_slot;
        let mut var_xn_s_dn6: f64 = *var_xn_s_dn6_slot;
        let mut var_xn_s_dn7: f64 = *var_xn_s_dn7_slot;
        let mut var_xn_s_dn8: f64 = *var_xn_s_dn8_slot;
        let mut var_xthscr: f64 = *var_xthscr_slot;
        let mut var_xthscr_dn5: f64 = *var_xthscr_dn5_slot;
        let mut var_xthscr_dn6: f64 = *var_xthscr_dn6_slot;
        let mut var_xthscr_dn7: f64 = *var_xthscr_dn7_slot;
        let mut var_xthscr_dn8: f64 = *var_xthscr_dn8_slot;

        let (assign41220_e54093, assign41220_e54093_d_n5, assign41220_e54093_d_n6, assign41220_e54093_d_n7, assign41220_e54093_d_n8,) = {
    if ((var_guard1175 != 0.0) && (var_guard1176 != 0.0)) {
        let assign41220_e54082: f64 = (0.5 * var_xn_s);
        let assign41220_e54086: f64 = (0.3125 * var_xn_s);
        let assign41220_e54087: f64 = (1.0 - assign41220_e54086);
        let assign41220_e54088: f64 = (assign41220_e54082 * assign41220_e54087);
        let assign41220_e54089: f64 = (1.0 - assign41220_e54088);
        let assign41220_e54090: f64 = (var_gf * assign41220_e54089);
        let assign41220_e54091: f64 = (1.0 + assign41220_e54090);
        (assign41220_e54091, ((var_gf_dn5 * assign41220_e54089) + (var_gf * (-(((0.5 * var_xn_s_dn5) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * var_xn_s_dn5))))))), ((var_gf_dn6 * assign41220_e54089) + (var_gf * (-(((0.5 * var_xn_s_dn6) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * var_xn_s_dn6))))))), ((var_gf_dn7 * assign41220_e54089) + (var_gf * (-(((0.5 * var_xn_s_dn7) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * var_xn_s_dn7))))))), ((var_gf_dn8 * assign41220_e54089) + (var_gf * (-(((0.5 * var_xn_s_dn8) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * var_xn_s_dn8))))))),)
    } else {
        (var_nscr, var_nscr_dn5, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8,)
    }
};
        var_nscr = assign41220_e54093;
        var_nscr_dn5 = assign41220_e54093_d_n5;
        var_nscr_dn6 = assign41220_e54093_d_n6;
        var_nscr_dn7 = assign41220_e54093_d_n7;
        var_nscr_dn8 = assign41220_e54093_d_n8;

        let assign41230_e54096: f64 = if var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1177 = assign41230_e54096;

        let (assign41240_e54107, assign41240_e54107_d_n5, assign41240_e54107_d_n6, assign41240_e54107_d_n7, assign41240_e54107_d_n8,) = {
    if (((var_guard1175 != 0.0) && (var_guard1176 == 0.0)) && (var_guard1177 != 0.0)) {
        let assign41240_e54104: f64 = (-var_xn_s);
        let assign41240_e54105: f64 = (assign41240_e54104).exp();
        (assign41240_e54105, (assign41240_e54105 * (-var_xn_s_dn5)), (assign41240_e54105 * (-var_xn_s_dn6)), (assign41240_e54105 * (-var_xn_s_dn7)), (assign41240_e54105 * (-var_xn_s_dn8)),)
    } else {
        (var_delta_ns, var_delta_ns_dn5, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8,)
    }
};
        var_delta_ns = assign41240_e54107;
        var_delta_ns_dn5 = assign41240_e54107_d_n5;
        var_delta_ns_dn6 = assign41240_e54107_d_n6;
        var_delta_ns_dn7 = assign41240_e54107_d_n7;
        var_delta_ns_dn8 = assign41240_e54107_d_n8;

        let (assign41250_e54139, assign41250_e54139_d_n5, assign41250_e54139_d_n6, assign41250_e54139_d_n7, assign41250_e54139_d_n8,) = {
    if (((var_guard1175 != 0.0) && (var_guard1176 == 0.0)) && (var_guard1177 == 0.0)) {
        let assign41250_e54119: f64 = (var_xn_s - 460.51701859880916);
        let assign41250_e54124: f64 = (var_xn_s - 460.51701859880916);
        let assign41250_e54128: f64 = (var_xn_s - 460.51701859880916);
        let assign41250_e54130: f64 = (assign41250_e54128 * 0.3333333333333333);
        let assign41250_e54131: f64 = (1.0 + assign41250_e54130);
        let assign41250_e54132: f64 = (assign41250_e54124 * assign41250_e54131);
        let assign41250_e54133: f64 = (0.5 * assign41250_e54132);
        let assign41250_e54134: f64 = (1.0 + assign41250_e54133);
        let assign41250_e54135: f64 = (assign41250_e54119 * assign41250_e54134);
        let assign41250_e54136: f64 = (1.0 + assign41250_e54135);
        let assign41250_e54137: f64 = (1e-200 / assign41250_e54136);
        (assign41250_e54137, (-((1e-200 * ((var_xn_s_dn5 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((var_xn_s_dn5 * assign41250_e54131) + (assign41250_e54124 * (var_xn_s_dn5 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((var_xn_s_dn6 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((var_xn_s_dn6 * assign41250_e54131) + (assign41250_e54124 * (var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((var_xn_s_dn7 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((var_xn_s_dn7 * assign41250_e54131) + (assign41250_e54124 * (var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((var_xn_s_dn8 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((var_xn_s_dn8 * assign41250_e54131) + (assign41250_e54124 * (var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))),)
    } else {
        (var_delta_ns, var_delta_ns_dn5, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8,)
    }
};
        var_delta_ns = assign41250_e54139;
        var_delta_ns_dn5 = assign41250_e54139_d_n5;
        var_delta_ns_dn6 = assign41250_e54139_d_n6;
        var_delta_ns_dn7 = assign41250_e54139_d_n7;
        var_delta_ns_dn8 = assign41250_e54139_d_n8;

        let (assign41260_e54152, assign41260_e54152_d_n5, assign41260_e54152_d_n6, assign41260_e54152_d_n7, assign41260_e54152_d_n8,) = {
    if ((var_guard1175 != 0.0) && (var_guard1176 == 0.0)) {
        let (assign41260_e54150,) = {
            if (var_xn_s > 0.0) {
                (1.0,)
            } else {
                let assign41260_e54149: f64 = (-1.0);
                (assign41260_e54149,)
            }
        };
        (assign41260_e54150, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41260_e54152;
        var_temp__blk936_dn5 = assign41260_e54152_d_n5;
        var_temp__blk936_dn6 = assign41260_e54152_d_n6;
        var_temp__blk936_dn7 = assign41260_e54152_d_n7;
        var_temp__blk936_dn8 = assign41260_e54152_d_n8;

        let (assign41270_e54180, assign41270_e54180_d_n5, assign41270_e54180_d_n6, assign41270_e54180_d_n7, assign41270_e54180_d_n8,) = {
    if ((var_guard1175 != 0.0) && (var_guard1176 == 0.0)) {
        let assign41270_e54160: f64 = (var_temp__blk936 * var_gf);
        let assign41270_e54165: f64 = (1.0 - var_xn_s);
        let assign41270_e54166: f64 = (var_delta_ns * assign41270_e54165);
        let assign41270_e54167: f64 = (1.0 - assign41270_e54166);
        let assign41270_e54168: f64 = (assign41270_e54160 * assign41270_e54167);
        let assign41270_e54173: f64 = (1.0 - var_delta_ns);
        let assign41270_e54174: f64 = (var_xn_s * assign41270_e54173);
        let assign41270_e54175: f64 = (assign41270_e54174).sqrt();
        let assign41270_e54176: f64 = (2.0 * assign41270_e54175);
        let assign41270_e54177: f64 = (assign41270_e54168 / assign41270_e54176);
        let assign41270_e54178: f64 = (1.0 + assign41270_e54177);
        (assign41270_e54178, (((((((var_temp__blk936_dn5 * var_gf) + (var_temp__blk936 * var_gf_dn5)) * assign41270_e54167) + (assign41270_e54160 * (-((var_delta_ns_dn5 * assign41270_e54165) + (var_delta_ns * (-var_xn_s_dn5)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((var_xn_s_dn5 * assign41270_e54173) + (var_xn_s * (-var_delta_ns_dn5))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((var_temp__blk936_dn6 * var_gf) + (var_temp__blk936 * var_gf_dn6)) * assign41270_e54167) + (assign41270_e54160 * (-((var_delta_ns_dn6 * assign41270_e54165) + (var_delta_ns * (-var_xn_s_dn6)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((var_xn_s_dn6 * assign41270_e54173) + (var_xn_s * (-var_delta_ns_dn6))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((var_temp__blk936_dn7 * var_gf) + (var_temp__blk936 * var_gf_dn7)) * assign41270_e54167) + (assign41270_e54160 * (-((var_delta_ns_dn7 * assign41270_e54165) + (var_delta_ns * (-var_xn_s_dn7)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((var_xn_s_dn7 * assign41270_e54173) + (var_xn_s * (-var_delta_ns_dn7))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((var_temp__blk936_dn8 * var_gf) + (var_temp__blk936 * var_gf_dn8)) * assign41270_e54167) + (assign41270_e54160 * (-((var_delta_ns_dn8 * assign41270_e54165) + (var_delta_ns * (-var_xn_s_dn8)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((var_xn_s_dn8 * assign41270_e54173) + (var_xn_s * (-var_delta_ns_dn8))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)),)
    } else {
        (var_nscr, var_nscr_dn5, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8,)
    }
};
        var_nscr = assign41270_e54180;
        var_nscr_dn5 = assign41270_e54180_d_n5;
        var_nscr_dn6 = assign41270_e54180_d_n6;
        var_nscr_dn7 = assign41270_e54180_d_n7;
        var_nscr_dn8 = assign41270_e54180_d_n8;

        let (assign41280_e54192, assign41280_e54192_d_n5, assign41280_e54192_d_n6, assign41280_e54192_d_n7, assign41280_e54192_d_n8,) = {
    if (var_guard1175 == 0.0) {
        let assign41280_e54186: f64 = (0.5 * var_gf);
        let assign41280_e54188: f64 = (var_xn_s).sqrt();
        let assign41280_e54189: f64 = (assign41280_e54186 / assign41280_e54188);
        let assign41280_e54190: f64 = (1.0 + assign41280_e54189);
        (assign41280_e54190, ((((0.5 * var_gf_dn5) * assign41280_e54188) - (assign41280_e54186 * (var_xn_s_dn5 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * var_gf_dn6) * assign41280_e54188) - (assign41280_e54186 * (var_xn_s_dn6 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * var_gf_dn7) * assign41280_e54188) - (assign41280_e54186 * (var_xn_s_dn7 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * var_gf_dn8) * assign41280_e54188) - (assign41280_e54186 * (var_xn_s_dn8 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)),)
    } else {
        (var_nscr, var_nscr_dn5, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8,)
    }
};
        var_nscr = assign41280_e54192;
        var_nscr_dn5 = assign41280_e54192_d_n5;
        var_nscr_dn6 = assign41280_e54192_d_n6;
        var_nscr_dn7 = assign41280_e54192_d_n7;
        var_nscr_dn8 = assign41280_e54192_d_n8;

        let assign41290_e54196: f64 = (var_xn_s).sqrt();
        let assign41290_e54197: f64 = (var_gf * assign41290_e54196);
        let assign41290_e54198: f64 = (var_xn_s + assign41290_e54197);
        let assign41290_e54202: f64 = (var_nscr - 1.0);
        let assign41290_e54203: f64 = (assign41290_e54202).ln();
        let assign41290_e54204: f64 = (var_nscr * assign41290_e54203);
        let assign41290_e54205: f64 = (assign41290_e54198 - assign41290_e54204);
        var_xthscr = assign41290_e54205;
        var_xthscr_dn5 = ((var_xn_s_dn5 + ((var_gf_dn5 * assign41290_e54196) + (var_gf * (var_xn_s_dn5 / (2.0 * assign41290_e54196))))) - ((var_nscr_dn5 * assign41290_e54203) + (var_nscr * (var_nscr_dn5 / assign41290_e54202))));
        var_xthscr_dn6 = ((var_xn_s_dn6 + ((var_gf_dn6 * assign41290_e54196) + (var_gf * (var_xn_s_dn6 / (2.0 * assign41290_e54196))))) - ((var_nscr_dn6 * assign41290_e54203) + (var_nscr * (var_nscr_dn6 / assign41290_e54202))));
        var_xthscr_dn7 = ((var_xn_s_dn7 + ((var_gf_dn7 * assign41290_e54196) + (var_gf * (var_xn_s_dn7 / (2.0 * assign41290_e54196))))) - ((var_nscr_dn7 * assign41290_e54203) + (var_nscr * (var_nscr_dn7 / assign41290_e54202))));
        var_xthscr_dn8 = ((var_xn_s_dn8 + ((var_gf_dn8 * assign41290_e54196) + (var_gf * (var_xn_s_dn8 / (2.0 * assign41290_e54196))))) - ((var_nscr_dn8 * assign41290_e54203) + (var_nscr * (var_nscr_dn8 / assign41290_e54202))));

        let assign41300_e54208: f64 = (var_xg - var_xthscr);
        let assign41300_e54210: f64 = (assign41300_e54208 / var_nscr);
        var_xgtscr = assign41300_e54210;
        var_xgtscr_dn5 = ((((var_xg_dn5 - var_xthscr_dn5) * var_nscr) - (assign41300_e54208 * var_nscr_dn5)) / (var_nscr * var_nscr));
        var_xgtscr_dn6 = ((((var_xg_dn6 - var_xthscr_dn6) * var_nscr) - (assign41300_e54208 * var_nscr_dn6)) / (var_nscr * var_nscr));
        var_xgtscr_dn7 = ((((var_xg_dn7 - var_xthscr_dn7) * var_nscr) - (assign41300_e54208 * var_nscr_dn7)) / (var_nscr * var_nscr));
        var_xgtscr_dn8 = ((((var_xg_dn8 - var_xthscr_dn8) * var_nscr) - (assign41300_e54208 * var_nscr_dn8)) / (var_nscr * var_nscr));

        let assign41310_e54213: f64 = (0.5 * var_gf2);
        let assign41310_e54217: f64 = (8.0 / var_gf2);
        let assign41310_e54218: f64 = (1.0 + assign41310_e54217);
        let assign41310_e54219: f64 = (assign41310_e54218).sqrt();
        let assign41310_e54221: f64 = (assign41310_e54219 - 1.0);
        let assign41310_e54222: f64 = (assign41310_e54213 * assign41310_e54221);
        var_qbscr = assign41310_e54222;
        var_qbscr_dn5 = (((0.5 * var_gf2_dn5) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * var_gf2_dn5) / (var_gf2 * var_gf2))) / (2.0 * assign41310_e54219))));
        var_qbscr_dn6 = (((0.5 * var_gf2_dn6) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * var_gf2_dn6) / (var_gf2 * var_gf2))) / (2.0 * assign41310_e54219))));
        var_qbscr_dn7 = (((0.5 * var_gf2_dn7) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * var_gf2_dn7) / (var_gf2 * var_gf2))) / (2.0 * assign41310_e54219))));
        var_qbscr_dn8 = (((0.5 * var_gf2_dn8) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * var_gf2_dn8) / (var_gf2 * var_gf2))) / (2.0 * assign41310_e54219))));

        var_qiscr = 0.0;
        var_qiscr_dn5 = 0.0;
        var_qiscr_dn6 = 0.0;
        var_qiscr_dn7 = 0.0;
        var_qiscr_dn8 = 0.0;

        var_fscr = 1.0;
        var_fscr_dn5 = 0.0;
        var_fscr_dn6 = 0.0;
        var_fscr_dn7 = 0.0;
        var_fscr_dn8 = 0.0;

        let assign41340_e54227: f64 = (-30.0);
        let assign41340_e54228: f64 = if var_xgtscr > assign41340_e54227 { 1.0 } else { 0.0 };
        var_guard1178 = assign41340_e54228;

        let (assign41350_e54236, assign41350_e54236_d_n5, assign41350_e54236_d_n6, assign41350_e54236_d_n7, assign41350_e54236_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41350_e54232: f64 = (var_nscr * var_xgtscr);
        let assign41350_e54234: f64 = (assign41350_e54232 - 1.0);
        (assign41350_e54234, ((var_nscr_dn5 * var_xgtscr) + (var_nscr * var_xgtscr_dn5)), ((var_nscr_dn6 * var_xgtscr) + (var_nscr * var_xgtscr_dn6)), ((var_nscr_dn7 * var_xgtscr) + (var_nscr * var_xgtscr_dn7)), ((var_nscr_dn8 * var_xgtscr) + (var_nscr * var_xgtscr_dn8)),)
    } else {
        (var_xgtscr0, var_xgtscr0_dn5, var_xgtscr0_dn6, var_xgtscr0_dn7, var_xgtscr0_dn8,)
    }
};
        var_xgtscr0 = assign41350_e54236;
        var_xgtscr0_dn5 = assign41350_e54236_d_n5;
        var_xgtscr0_dn6 = assign41350_e54236_d_n6;
        var_xgtscr0_dn7 = assign41350_e54236_d_n7;
        var_xgtscr0_dn8 = assign41350_e54236_d_n8;

        let (assign41360_e54249, assign41360_e54249_d_n5, assign41360_e54249_d_n6, assign41360_e54249_d_n7, assign41360_e54249_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41360_e54242: f64 = (var_xgtscr0 * var_xgtscr0);
        let assign41360_e54244: f64 = (assign41360_e54242 + 10.0);
        let assign41360_e54245: f64 = (assign41360_e54244).sqrt();
        let assign41360_e54246: f64 = (var_xgtscr0 + assign41360_e54245);
        let assign41360_e54247: f64 = (0.5 * assign41360_e54246);
        (assign41360_e54247, (0.5 * (var_xgtscr0_dn5 + (((var_xgtscr0_dn5 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn5)) / (2.0 * assign41360_e54245)))), (0.5 * (var_xgtscr0_dn6 + (((var_xgtscr0_dn6 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn6)) / (2.0 * assign41360_e54245)))), (0.5 * (var_xgtscr0_dn7 + (((var_xgtscr0_dn7 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn7)) / (2.0 * assign41360_e54245)))), (0.5 * (var_xgtscr0_dn8 + (((var_xgtscr0_dn8 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn8)) / (2.0 * assign41360_e54245)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41360_e54249;
        var_temp__blk936_dn5 = assign41360_e54249_d_n5;
        var_temp__blk936_dn6 = assign41360_e54249_d_n6;
        var_temp__blk936_dn7 = assign41360_e54249_d_n7;
        var_temp__blk936_dn8 = assign41360_e54249_d_n8;

        let (assign41370_e54256, assign41370_e54256_d_n5, assign41370_e54256_d_n6, assign41370_e54256_d_n7, assign41370_e54256_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41370_e54253: f64 = (var_temp__blk936).ln();
        let assign41370_e54254: f64 = (var_xgtscr - assign41370_e54253);
        (assign41370_e54254, (var_xgtscr_dn5 - (var_temp__blk936_dn5 / var_temp__blk936)), (var_xgtscr_dn6 - (var_temp__blk936_dn6 / var_temp__blk936)), (var_xgtscr_dn7 - (var_temp__blk936_dn7 / var_temp__blk936)), (var_xgtscr_dn8 - (var_temp__blk936_dn8 / var_temp__blk936)),)
    } else {
        (var_qiscr0si, var_qiscr0si_dn5, var_qiscr0si_dn6, var_qiscr0si_dn7, var_qiscr0si_dn8,)
    }
};
        var_qiscr0si = assign41370_e54256;
        var_qiscr0si_dn5 = assign41370_e54256_d_n5;
        var_qiscr0si_dn6 = assign41370_e54256_d_n6;
        var_qiscr0si_dn7 = assign41370_e54256_d_n7;
        var_qiscr0si_dn8 = assign41370_e54256_d_n8;

        let (assign41380_e54269, assign41380_e54269_d_n5, assign41380_e54269_d_n6, assign41380_e54269_d_n7, assign41380_e54269_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41380_e54262: f64 = (var_qiscr0si * var_qiscr0si);
        let assign41380_e54264: f64 = (assign41380_e54262 + 2.0);
        let assign41380_e54265: f64 = (assign41380_e54264).sqrt();
        let assign41380_e54266: f64 = (var_qiscr0si + assign41380_e54265);
        let assign41380_e54267: f64 = (0.5 * assign41380_e54266);
        (assign41380_e54267, (0.5 * (var_qiscr0si_dn5 + (((var_qiscr0si_dn5 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn5)) / (2.0 * assign41380_e54265)))), (0.5 * (var_qiscr0si_dn6 + (((var_qiscr0si_dn6 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn6)) / (2.0 * assign41380_e54265)))), (0.5 * (var_qiscr0si_dn7 + (((var_qiscr0si_dn7 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn7)) / (2.0 * assign41380_e54265)))), (0.5 * (var_qiscr0si_dn8 + (((var_qiscr0si_dn8 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn8)) / (2.0 * assign41380_e54265)))),)
    } else {
        (var_qiscr0, var_qiscr0_dn5, var_qiscr0_dn6, var_qiscr0_dn7, var_qiscr0_dn8,)
    }
};
        var_qiscr0 = assign41380_e54269;
        var_qiscr0_dn5 = assign41380_e54269_d_n5;
        var_qiscr0_dn6 = assign41380_e54269_d_n6;
        var_qiscr0_dn7 = assign41380_e54269_d_n7;
        var_qiscr0_dn8 = assign41380_e54269_d_n8;

        let assign41390_e54272: f64 = (var_xgtscr - var_qiscr0);
        let assign41390_e54274: f64 = if assign41390_e54272 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1179 = assign41390_e54274;

        let (assign41400_e54283, assign41400_e54283_d_n5, assign41400_e54283_d_n6, assign41400_e54283_d_n7, assign41400_e54283_d_n8,) = {
    if ((var_guard1178 != 0.0) && (var_guard1179 != 0.0)) {
        let assign41400_e54280: f64 = (var_xgtscr - var_qiscr0);
        let assign41400_e54281: f64 = (assign41400_e54280).exp();
        (assign41400_e54281, (assign41400_e54281 * (var_xgtscr_dn5 - var_qiscr0_dn5)), (assign41400_e54281 * (var_xgtscr_dn6 - var_qiscr0_dn6)), (assign41400_e54281 * (var_xgtscr_dn7 - var_qiscr0_dn7)), (assign41400_e54281 * (var_xgtscr_dn8 - var_qiscr0_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41400_e54283;
        var_temp__blk936_dn5 = assign41400_e54283_d_n5;
        var_temp__blk936_dn6 = assign41400_e54283_d_n6;
        var_temp__blk936_dn7 = assign41400_e54283_d_n7;
        var_temp__blk936_dn8 = assign41400_e54283_d_n8;

        let (assign41410_e54318, assign41410_e54318_d_n5, assign41410_e54318_d_n6, assign41410_e54318_d_n7, assign41410_e54318_d_n8,) = {
    if ((var_guard1178 != 0.0) && (var_guard1179 == 0.0)) {
        let assign41410_e54292: f64 = (var_xgtscr - var_qiscr0);
        let assign41410_e54294: f64 = (assign41410_e54292 - 230.25850929940458);
        let assign41410_e54299: f64 = (var_xgtscr - var_qiscr0);
        let assign41410_e54301: f64 = (assign41410_e54299 - 230.25850929940458);
        let assign41410_e54305: f64 = (var_xgtscr - var_qiscr0);
        let assign41410_e54307: f64 = (assign41410_e54305 - 230.25850929940458);
        let assign41410_e54309: f64 = (assign41410_e54307 * 0.3333333333333333);
        let assign41410_e54310: f64 = (1.0 + assign41410_e54309);
        let assign41410_e54311: f64 = (assign41410_e54301 * assign41410_e54310);
        let assign41410_e54312: f64 = (0.5 * assign41410_e54311);
        let assign41410_e54313: f64 = (1.0 + assign41410_e54312);
        let assign41410_e54314: f64 = (assign41410_e54294 * assign41410_e54313);
        let assign41410_e54315: f64 = (1.0 + assign41410_e54314);
        let assign41410_e54316: f64 = (1e100 * assign41410_e54315);
        (assign41410_e54316, (1e100 * (((var_xgtscr_dn5 - var_qiscr0_dn5) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((var_xgtscr_dn5 - var_qiscr0_dn5) * assign41410_e54310) + (assign41410_e54301 * ((var_xgtscr_dn5 - var_qiscr0_dn5) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn6 - var_qiscr0_dn6) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((var_xgtscr_dn6 - var_qiscr0_dn6) * assign41410_e54310) + (assign41410_e54301 * ((var_xgtscr_dn6 - var_qiscr0_dn6) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn7 - var_qiscr0_dn7) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((var_xgtscr_dn7 - var_qiscr0_dn7) * assign41410_e54310) + (assign41410_e54301 * ((var_xgtscr_dn7 - var_qiscr0_dn7) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn8 - var_qiscr0_dn8) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((var_xgtscr_dn8 - var_qiscr0_dn8) * assign41410_e54310) + (assign41410_e54301 * ((var_xgtscr_dn8 - var_qiscr0_dn8) * 0.3333333333333333))))))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41410_e54318;
        var_temp__blk936_dn5 = assign41410_e54318_d_n5;
        var_temp__blk936_dn6 = assign41410_e54318_d_n6;
        var_temp__blk936_dn7 = assign41410_e54318_d_n7;
        var_temp__blk936_dn8 = assign41410_e54318_d_n8;

        let (assign41420_e54324, assign41420_e54324_d_n5, assign41420_e54324_d_n6, assign41420_e54324_d_n7, assign41420_e54324_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41420_e54322: f64 = (var_temp__blk936 / var_nscr);
        (assign41420_e54322, (((var_temp__blk936_dn5 * var_nscr) - (var_temp__blk936 * var_nscr_dn5)) / (var_nscr * var_nscr)), (((var_temp__blk936_dn6 * var_nscr) - (var_temp__blk936 * var_nscr_dn6)) / (var_nscr * var_nscr)), (((var_temp__blk936_dn7 * var_nscr) - (var_temp__blk936 * var_nscr_dn7)) / (var_nscr * var_nscr)), (((var_temp__blk936_dn8 * var_nscr) - (var_temp__blk936 * var_nscr_dn8)) / (var_nscr * var_nscr)),)
    } else {
        (var_dscr0, var_dscr0_dn5, var_dscr0_dn6, var_dscr0_dn7, var_dscr0_dn8,)
    }
};
        var_dscr0 = assign41420_e54324;
        var_dscr0_dn5 = assign41420_e54324_d_n5;
        var_dscr0_dn6 = assign41420_e54324_d_n6;
        var_dscr0_dn7 = assign41420_e54324_d_n7;
        var_dscr0_dn8 = assign41420_e54324_d_n8;

        let (assign41430_e54334, assign41430_e54334_d_n5, assign41430_e54334_d_n6, assign41430_e54334_d_n7, assign41430_e54334_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41430_e54329: f64 = (var_qiscr0 + 1.0);
        let assign41430_e54330: f64 = (2.0 * assign41430_e54329);
        let assign41430_e54332: f64 = (assign41430_e54330 - var_dscr0);
        (assign41430_e54332, ((2.0 * var_qiscr0_dn5) - var_dscr0_dn5), ((2.0 * var_qiscr0_dn6) - var_dscr0_dn6), ((2.0 * var_qiscr0_dn7) - var_dscr0_dn7), ((2.0 * var_qiscr0_dn8) - var_dscr0_dn8),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41430_e54334;
        var_temp__blk936_dn5 = assign41430_e54334_d_n5;
        var_temp__blk936_dn6 = assign41430_e54334_d_n6;
        var_temp__blk936_dn7 = assign41430_e54334_d_n7;
        var_temp__blk936_dn8 = assign41430_e54334_d_n8;

        let assign41440_e54337: f64 = if var_dscr0 > 1e-6 { 1.0 } else { 0.0 };
        var_guard1180 = assign41440_e54337;

        let (assign41450_e54358, assign41450_e54358_d_n5, assign41450_e54358_d_n6, assign41450_e54358_d_n7, assign41450_e54358_d_n8,) = {
    if ((var_guard1178 != 0.0) && (var_guard1180 != 0.0)) {
        let assign41450_e54346: f64 = (var_dscr0 * var_temp__blk936);
        let assign41450_e54347: f64 = (1.0 + assign41450_e54346);
        let assign41450_e54348: f64 = (assign41450_e54347).sqrt();
        let assign41450_e54350: f64 = (assign41450_e54348 - 1.0);
        let assign41450_e54352: f64 = (assign41450_e54350 / var_dscr0);
        let assign41450_e54353: f64 = (var_qiscr0 - assign41450_e54352);
        let assign41450_e54355: f64 = (assign41450_e54353 + 1.0);
        let assign41450_e54356: f64 = (var_nscr * assign41450_e54355);
        (assign41450_e54356, ((var_nscr_dn5 * assign41450_e54355) + (var_nscr * (var_qiscr0_dn5 - ((((((var_dscr0_dn5 * var_temp__blk936) + (var_dscr0 * var_temp__blk936_dn5)) / (2.0 * assign41450_e54348)) * var_dscr0) - (assign41450_e54350 * var_dscr0_dn5)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn6 * assign41450_e54355) + (var_nscr * (var_qiscr0_dn6 - ((((((var_dscr0_dn6 * var_temp__blk936) + (var_dscr0 * var_temp__blk936_dn6)) / (2.0 * assign41450_e54348)) * var_dscr0) - (assign41450_e54350 * var_dscr0_dn6)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn7 * assign41450_e54355) + (var_nscr * (var_qiscr0_dn7 - ((((((var_dscr0_dn7 * var_temp__blk936) + (var_dscr0 * var_temp__blk936_dn7)) / (2.0 * assign41450_e54348)) * var_dscr0) - (assign41450_e54350 * var_dscr0_dn7)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn8 * assign41450_e54355) + (var_nscr * (var_qiscr0_dn8 - ((((((var_dscr0_dn8 * var_temp__blk936) + (var_dscr0 * var_temp__blk936_dn8)) / (2.0 * assign41450_e54348)) * var_dscr0) - (assign41450_e54350 * var_dscr0_dn8)) / (var_dscr0 * var_dscr0))))),)
    } else {
        (var_qiscr, var_qiscr_dn5, var_qiscr_dn6, var_qiscr_dn7, var_qiscr_dn8,)
    }
};
        var_qiscr = assign41450_e54358;
        var_qiscr_dn5 = assign41450_e54358_d_n5;
        var_qiscr_dn6 = assign41450_e54358_d_n6;
        var_qiscr_dn7 = assign41450_e54358_d_n7;
        var_qiscr_dn8 = assign41450_e54358_d_n8;

        let (assign41460_e54377, assign41460_e54377_d_n5, assign41460_e54377_d_n6, assign41460_e54377_d_n7, assign41460_e54377_d_n8,) = {
    if ((var_guard1178 != 0.0) && (var_guard1180 == 0.0)) {
        let assign41460_e54365: f64 = (var_nscr * 0.5);
        let assign41460_e54367: f64 = (assign41460_e54365 * var_dscr0);
        let assign41460_e54371: f64 = (0.25 * var_temp__blk936);
        let assign41460_e54373: f64 = (assign41460_e54371 * var_temp__blk936);
        let assign41460_e54374: f64 = (1.0 + assign41460_e54373);
        let assign41460_e54375: f64 = (assign41460_e54367 * assign41460_e54374);
        (assign41460_e54375, (((((var_nscr_dn5 * 0.5) * var_dscr0) + (assign41460_e54365 * var_dscr0_dn5)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * var_temp__blk936_dn5) * var_temp__blk936) + (assign41460_e54371 * var_temp__blk936_dn5)))), (((((var_nscr_dn6 * 0.5) * var_dscr0) + (assign41460_e54365 * var_dscr0_dn6)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * var_temp__blk936_dn6) * var_temp__blk936) + (assign41460_e54371 * var_temp__blk936_dn6)))), (((((var_nscr_dn7 * 0.5) * var_dscr0) + (assign41460_e54365 * var_dscr0_dn7)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * var_temp__blk936_dn7) * var_temp__blk936) + (assign41460_e54371 * var_temp__blk936_dn7)))), (((((var_nscr_dn8 * 0.5) * var_dscr0) + (assign41460_e54365 * var_dscr0_dn8)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * var_temp__blk936_dn8) * var_temp__blk936) + (assign41460_e54371 * var_temp__blk936_dn8)))),)
    } else {
        (var_qiscr, var_qiscr_dn5, var_qiscr_dn6, var_qiscr_dn7, var_qiscr_dn8,)
    }
};
        var_qiscr = assign41460_e54377;
        var_qiscr_dn5 = assign41460_e54377_d_n5;
        var_qiscr_dn6 = assign41460_e54377_d_n6;
        var_qiscr_dn7 = assign41460_e54377_d_n7;
        var_qiscr_dn8 = assign41460_e54377_d_n8;

        let (assign41470_e54402, assign41470_e54402_d_n5, assign41470_e54402_d_n6, assign41470_e54402_d_n7, assign41470_e54402_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41470_e54382: f64 = (var_xg - var_qiscr);
        let assign41470_e54384: f64 = (assign41470_e54382 + 2.0);
        let assign41470_e54387: f64 = (var_xg - var_qiscr);
        let assign41470_e54389: f64 = (assign41470_e54387 - 2.0);
        let assign41470_e54392: f64 = (var_xg - var_qiscr);
        let assign41470_e54394: f64 = (assign41470_e54392 - 2.0);
        let assign41470_e54395: f64 = (assign41470_e54389 * assign41470_e54394);
        let assign41470_e54397: f64 = (assign41470_e54395 + 1.0);
        let assign41470_e54398: f64 = (assign41470_e54397).sqrt();
        let assign41470_e54399: f64 = (assign41470_e54384 + assign41470_e54398);
        let assign41470_e54400: f64 = (0.5 * assign41470_e54399);
        (assign41470_e54400, (0.5 * ((var_xg_dn5 - var_qiscr_dn5) + ((((var_xg_dn5 - var_qiscr_dn5) * assign41470_e54394) + (assign41470_e54389 * (var_xg_dn5 - var_qiscr_dn5))) / (2.0 * assign41470_e54398)))), (0.5 * ((var_xg_dn6 - var_qiscr_dn6) + ((((var_xg_dn6 - var_qiscr_dn6) * assign41470_e54394) + (assign41470_e54389 * (var_xg_dn6 - var_qiscr_dn6))) / (2.0 * assign41470_e54398)))), (0.5 * ((var_xg_dn7 - var_qiscr_dn7) + ((((var_xg_dn7 - var_qiscr_dn7) * assign41470_e54394) + (assign41470_e54389 * (var_xg_dn7 - var_qiscr_dn7))) / (2.0 * assign41470_e54398)))), (0.5 * ((var_xg_dn8 - var_qiscr_dn8) + ((((var_xg_dn8 - var_qiscr_dn8) * assign41470_e54394) + (assign41470_e54389 * (var_xg_dn8 - var_qiscr_dn8))) / (2.0 * assign41470_e54398)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign41470_e54402;
        var_temp__blk936_dn5 = assign41470_e54402_d_n5;
        var_temp__blk936_dn6 = assign41470_e54402_d_n6;
        var_temp__blk936_dn7 = assign41470_e54402_d_n7;
        var_temp__blk936_dn8 = assign41470_e54402_d_n8;

        let (assign41480_e54419, assign41480_e54419_d_n5, assign41480_e54419_d_n6, assign41480_e54419_d_n7, assign41480_e54419_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41480_e54406: f64 = (0.5 * var_gf2);
        let assign41480_e54410: f64 = (4.0 / var_gf2);
        let assign41480_e54412: f64 = (assign41480_e54410 * var_temp__blk936);
        let assign41480_e54413: f64 = (1.0 + assign41480_e54412);
        let assign41480_e54414: f64 = (assign41480_e54413).sqrt();
        let assign41480_e54416: f64 = (assign41480_e54414 - 1.0);
        let assign41480_e54417: f64 = (assign41480_e54406 * assign41480_e54416);
        (assign41480_e54417, (((0.5 * var_gf2_dn5) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * var_gf2_dn5) / (var_gf2 * var_gf2))) * var_temp__blk936) + (assign41480_e54410 * var_temp__blk936_dn5)) / (2.0 * assign41480_e54414)))), (((0.5 * var_gf2_dn6) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * var_gf2_dn6) / (var_gf2 * var_gf2))) * var_temp__blk936) + (assign41480_e54410 * var_temp__blk936_dn6)) / (2.0 * assign41480_e54414)))), (((0.5 * var_gf2_dn7) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * var_gf2_dn7) / (var_gf2 * var_gf2))) * var_temp__blk936) + (assign41480_e54410 * var_temp__blk936_dn7)) / (2.0 * assign41480_e54414)))), (((0.5 * var_gf2_dn8) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * var_gf2_dn8) / (var_gf2 * var_gf2))) * var_temp__blk936) + (assign41480_e54410 * var_temp__blk936_dn8)) / (2.0 * assign41480_e54414)))),)
    } else {
        (var_qbscr, var_qbscr_dn5, var_qbscr_dn6, var_qbscr_dn7, var_qbscr_dn8,)
    }
};
        var_qbscr = assign41480_e54419;
        var_qbscr_dn5 = assign41480_e54419_d_n5;
        var_qbscr_dn6 = assign41480_e54419_d_n6;
        var_qbscr_dn7 = assign41480_e54419_d_n7;
        var_qbscr_dn8 = assign41480_e54419_d_n8;

        let (assign41490_e54427, assign41490_e54427_d_n5, assign41490_e54427_d_n6, assign41490_e54427_d_n7, assign41490_e54427_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41490_e54424: f64 = (var_qbscr + var_qiscr);
        let assign41490_e54425: f64 = (var_qbscr / assign41490_e54424);
        (assign41490_e54425, (((var_qbscr_dn5 * assign41490_e54424) - (var_qbscr * (var_qbscr_dn5 + var_qiscr_dn5))) / (assign41490_e54424 * assign41490_e54424)), (((var_qbscr_dn6 * assign41490_e54424) - (var_qbscr * (var_qbscr_dn6 + var_qiscr_dn6))) / (assign41490_e54424 * assign41490_e54424)), (((var_qbscr_dn7 * assign41490_e54424) - (var_qbscr * (var_qbscr_dn7 + var_qiscr_dn7))) / (assign41490_e54424 * assign41490_e54424)), (((var_qbscr_dn8 * assign41490_e54424) - (var_qbscr * (var_qbscr_dn8 + var_qiscr_dn8))) / (assign41490_e54424 * assign41490_e54424)),)
    } else {
        (var_fscr, var_fscr_dn5, var_fscr_dn6, var_fscr_dn7, var_fscr_dn8,)
    }
};
        var_fscr = assign41490_e54427;
        var_fscr_dn5 = assign41490_e54427_d_n5;
        var_fscr_dn6 = assign41490_e54427_d_n6;
        var_fscr_dn7 = assign41490_e54427_d_n7;
        var_fscr_dn8 = assign41490_e54427_d_n8;

        let (assign41500_e54435, assign41500_e54435_d_n5, assign41500_e54435_d_n6, assign41500_e54435_d_n7, assign41500_e54435_d_n8,) = {
    if (var_guard1178 != 0.0) {
        let assign41500_e54432: f64 = (var_fscr * var_delxb);
        let assign41500_e54433: f64 = (var_xno_s - assign41500_e54432);
        (assign41500_e54433, (var_xno_s_dn5 - ((var_fscr_dn5 * var_delxb) + (var_fscr * var_delxb_dn5))), (var_xno_s_dn6 - ((var_fscr_dn6 * var_delxb) + (var_fscr * var_delxb_dn6))), (var_xno_s_dn7 - ((var_fscr_dn7 * var_delxb) + (var_fscr * var_delxb_dn7))), (var_xno_s_dn8 - ((var_fscr_dn8 * var_delxb) + (var_fscr * var_delxb_dn8))),)
    } else {
        (var_xn_s, var_xn_s_dn5, var_xn_s_dn6, var_xn_s_dn7, var_xn_s_dn8,)
    }
};
        var_xn_s = assign41500_e54435;
        var_xn_s_dn5 = assign41500_e54435_d_n5;
        var_xn_s_dn6 = assign41500_e54435_d_n6;
        var_xn_s_dn7 = assign41500_e54435_d_n7;
        var_xn_s_dn8 = assign41500_e54435_d_n8;

        let assign41510_e54439: f64 = (var_gf * 0.7071067811865475);
        let assign41510_e54440: f64 = (1.0 + assign41510_e54439);
        var_xi = assign41510_e54440;
        var_xi_dn5 = (var_gf_dn5 * 0.7071067811865475);
        var_xi_dn6 = (var_gf_dn6 * 0.7071067811865475);
        var_xi_dn7 = (var_gf_dn7 * 0.7071067811865475);
        var_xi_dn8 = (var_gf_dn8 * 0.7071067811865475);

        let assign41520_e54443: f64 = (1e-5 * var_xi);
        var_margin = assign41520_e54443;

        let assign41530_e54446: f64 = (1.0 / var_xi);
        var_inv_xi = assign41530_e54446;
        var_inv_xi_dn5 = (-(var_xi_dn5 / (var_xi * var_xi)));
        var_inv_xi_dn6 = (-(var_xi_dn6 / (var_xi * var_xi)));
        var_inv_xi_dn7 = (-(var_xi_dn7 / (var_xi * var_xi)));
        var_inv_xi_dn8 = (-(var_xi_dn8 / (var_xi * var_xi)));

        var_sp_s_x1 = 0.0;
        var_sp_s_x1_dn5 = 0.0;
        var_sp_s_x1_dn6 = 0.0;
        var_sp_s_x1_dn7 = 0.0;
        var_sp_s_x1_dn8 = 0.0;

        var_x_s = 0.0;
        var_x_s_dn5 = 0.0;
        var_x_s_dn6 = 0.0;
        var_x_s_dn7 = 0.0;
        var_x_s_dn8 = 0.0;

        let assign41560_e54451: f64 = if var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1181 = assign41560_e54451;

        let (assign41570_e54457, assign41570_e54457_d_n5, assign41570_e54457_d_n6, assign41570_e54457_d_n7, assign41570_e54457_d_n8,) = {
    if (var_guard1181 != 0.0) {
        let assign41570_e54454: f64 = (-var_xn_s);
        let assign41570_e54455: f64 = (assign41570_e54454).exp();
        (assign41570_e54455, (assign41570_e54455 * (-var_xn_s_dn5)), (assign41570_e54455 * (-var_xn_s_dn6)), (assign41570_e54455 * (-var_xn_s_dn7)), (assign41570_e54455 * (-var_xn_s_dn8)),)
    } else {
        (var_delta_ns, var_delta_ns_dn5, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8,)
    }
};
        var_delta_ns = assign41570_e54457;
        var_delta_ns_dn5 = assign41570_e54457_d_n5;
        var_delta_ns_dn6 = assign41570_e54457_d_n6;
        var_delta_ns_dn7 = assign41570_e54457_d_n7;
        var_delta_ns_dn8 = assign41570_e54457_d_n8;

        let (assign41580_e54484, assign41580_e54484_d_n5, assign41580_e54484_d_n6, assign41580_e54484_d_n7, assign41580_e54484_d_n8,) = {
    if (var_guard1181 == 0.0) {
        let assign41580_e54464: f64 = (var_xn_s - 460.51701859880916);
        let assign41580_e54469: f64 = (var_xn_s - 460.51701859880916);
        let assign41580_e54473: f64 = (var_xn_s - 460.51701859880916);
        let assign41580_e54475: f64 = (assign41580_e54473 * 0.3333333333333333);
        let assign41580_e54476: f64 = (1.0 + assign41580_e54475);
        let assign41580_e54477: f64 = (assign41580_e54469 * assign41580_e54476);
        let assign41580_e54478: f64 = (0.5 * assign41580_e54477);
        let assign41580_e54479: f64 = (1.0 + assign41580_e54478);
        let assign41580_e54480: f64 = (assign41580_e54464 * assign41580_e54479);
        let assign41580_e54481: f64 = (1.0 + assign41580_e54480);
        let assign41580_e54482: f64 = (1e-200 / assign41580_e54481);
        (assign41580_e54482, (-((1e-200 * ((var_xn_s_dn5 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((var_xn_s_dn5 * assign41580_e54476) + (assign41580_e54469 * (var_xn_s_dn5 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((var_xn_s_dn6 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((var_xn_s_dn6 * assign41580_e54476) + (assign41580_e54469 * (var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((var_xn_s_dn7 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((var_xn_s_dn7 * assign41580_e54476) + (assign41580_e54469 * (var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((var_xn_s_dn8 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((var_xn_s_dn8 * assign41580_e54476) + (assign41580_e54469 * (var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))),)
    } else {
        (var_delta_ns, var_delta_ns_dn5, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8,)
    }
};
        var_delta_ns = assign41580_e54484;
        var_delta_ns_dn5 = assign41580_e54484_d_n5;
        var_delta_ns_dn6 = assign41580_e54484_d_n6;
        var_delta_ns_dn7 = assign41580_e54484_d_n7;
        var_delta_ns_dn8 = assign41580_e54484_d_n8;

        let assign41590_e54486: f64 = (var_xg).abs();
        let assign41590_e54488: f64 = if assign41590_e54486 <= var_margin { 1.0 } else { 0.0 };
        var_guard1182 = assign41590_e54488;

        *var_delta_ns_slot = var_delta_ns;
        *var_delta_ns_dn5_slot = var_delta_ns_dn5;
        *var_delta_ns_dn6_slot = var_delta_ns_dn6;
        *var_delta_ns_dn7_slot = var_delta_ns_dn7;
        *var_delta_ns_dn8_slot = var_delta_ns_dn8;
        *var_dscr0_slot = var_dscr0;
        *var_dscr0_dn5_slot = var_dscr0_dn5;
        *var_dscr0_dn6_slot = var_dscr0_dn6;
        *var_dscr0_dn7_slot = var_dscr0_dn7;
        *var_dscr0_dn8_slot = var_dscr0_dn8;
        *var_fscr_slot = var_fscr;
        *var_fscr_dn5_slot = var_fscr_dn5;
        *var_fscr_dn6_slot = var_fscr_dn6;
        *var_fscr_dn7_slot = var_fscr_dn7;
        *var_fscr_dn8_slot = var_fscr_dn8;
        *var_guard1177_slot = var_guard1177;
        *var_guard1178_slot = var_guard1178;
        *var_guard1179_slot = var_guard1179;
        *var_guard1180_slot = var_guard1180;
        *var_guard1181_slot = var_guard1181;
        *var_guard1182_slot = var_guard1182;
        *var_inv_xi_slot = var_inv_xi;
        *var_inv_xi_dn5_slot = var_inv_xi_dn5;
        *var_inv_xi_dn6_slot = var_inv_xi_dn6;
        *var_inv_xi_dn7_slot = var_inv_xi_dn7;
        *var_inv_xi_dn8_slot = var_inv_xi_dn8;
        *var_margin_slot = var_margin;
        *var_nscr_slot = var_nscr;
        *var_nscr_dn5_slot = var_nscr_dn5;
        *var_nscr_dn6_slot = var_nscr_dn6;
        *var_nscr_dn7_slot = var_nscr_dn7;
        *var_nscr_dn8_slot = var_nscr_dn8;
        *var_qbscr_slot = var_qbscr;
        *var_qbscr_dn5_slot = var_qbscr_dn5;
        *var_qbscr_dn6_slot = var_qbscr_dn6;
        *var_qbscr_dn7_slot = var_qbscr_dn7;
        *var_qbscr_dn8_slot = var_qbscr_dn8;
        *var_qiscr_slot = var_qiscr;
        *var_qiscr0_slot = var_qiscr0;
        *var_qiscr0_dn5_slot = var_qiscr0_dn5;
        *var_qiscr0_dn6_slot = var_qiscr0_dn6;
        *var_qiscr0_dn7_slot = var_qiscr0_dn7;
        *var_qiscr0_dn8_slot = var_qiscr0_dn8;
        *var_qiscr0si_slot = var_qiscr0si;
        *var_qiscr0si_dn5_slot = var_qiscr0si_dn5;
        *var_qiscr0si_dn6_slot = var_qiscr0si_dn6;
        *var_qiscr0si_dn7_slot = var_qiscr0si_dn7;
        *var_qiscr0si_dn8_slot = var_qiscr0si_dn8;
        *var_qiscr_dn5_slot = var_qiscr_dn5;
        *var_qiscr_dn6_slot = var_qiscr_dn6;
        *var_qiscr_dn7_slot = var_qiscr_dn7;
        *var_qiscr_dn8_slot = var_qiscr_dn8;
        *var_sp_s_x1_slot = var_sp_s_x1;
        *var_sp_s_x1_dn5_slot = var_sp_s_x1_dn5;
        *var_sp_s_x1_dn6_slot = var_sp_s_x1_dn6;
        *var_sp_s_x1_dn7_slot = var_sp_s_x1_dn7;
        *var_sp_s_x1_dn8_slot = var_sp_s_x1_dn8;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_x_s_slot = var_x_s;
        *var_x_s_dn5_slot = var_x_s_dn5;
        *var_x_s_dn6_slot = var_x_s_dn6;
        *var_x_s_dn7_slot = var_x_s_dn7;
        *var_x_s_dn8_slot = var_x_s_dn8;
        *var_xgtscr_slot = var_xgtscr;
        *var_xgtscr0_slot = var_xgtscr0;
        *var_xgtscr0_dn5_slot = var_xgtscr0_dn5;
        *var_xgtscr0_dn6_slot = var_xgtscr0_dn6;
        *var_xgtscr0_dn7_slot = var_xgtscr0_dn7;
        *var_xgtscr0_dn8_slot = var_xgtscr0_dn8;
        *var_xgtscr_dn5_slot = var_xgtscr_dn5;
        *var_xgtscr_dn6_slot = var_xgtscr_dn6;
        *var_xgtscr_dn7_slot = var_xgtscr_dn7;
        *var_xgtscr_dn8_slot = var_xgtscr_dn8;
        *var_xi_slot = var_xi;
        *var_xi_dn5_slot = var_xi_dn5;
        *var_xi_dn6_slot = var_xi_dn6;
        *var_xi_dn7_slot = var_xi_dn7;
        *var_xi_dn8_slot = var_xi_dn8;
        *var_xn_s_slot = var_xn_s;
        *var_xn_s_dn5_slot = var_xn_s_dn5;
        *var_xn_s_dn6_slot = var_xn_s_dn6;
        *var_xn_s_dn7_slot = var_xn_s_dn7;
        *var_xn_s_dn8_slot = var_xn_s_dn8;
        *var_xthscr_slot = var_xthscr;
        *var_xthscr_dn5_slot = var_xthscr_dn5;
        *var_xthscr_dn6_slot = var_xthscr_dn6;
        *var_xthscr_dn7_slot = var_xthscr_dn7;
        *var_xthscr_dn8_slot = var_xthscr_dn8;
    }

    pub(super) fn stamp_transient_block_89(
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1182: f64,
        var_inv_gf2: f64,
        var_inv_gf2_dn5: f64,
        var_inv_gf2_dn6: f64,
        var_inv_gf2_dn7: f64,
        var_inv_gf2_dn8: f64,
        var_inv_xi: f64,
        var_inv_xi_dn5: f64,
        var_inv_xi_dn6: f64,
        var_inv_xi_dn7: f64,
        var_inv_xi_dn8: f64,
        var_margin: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xi: f64,
        var_xi_dn5: f64,
        var_xi_dn6: f64,
        var_xi_dn7: f64,
        var_xi_dn8: f64,
        var_guard1183_slot: &mut f64,
        var_guard1184_slot: &mut f64,
        var_mutau_slot: &mut f64,
        var_mutau_dn5_slot: &mut f64,
        var_mutau_dn6_slot: &mut f64,
        var_mutau_dn7_slot: &mut f64,
        var_mutau_dn8_slot: &mut f64,
        var_nu_slot: &mut f64,
        var_nu_dn5_slot: &mut f64,
        var_nu_dn6_slot: &mut f64,
        var_nu_dn7_slot: &mut f64,
        var_nu_dn8_slot: &mut f64,
        var_sp_s_a_slot: &mut f64,
        var_sp_s_a_dn5_slot: &mut f64,
        var_sp_s_a_dn6_slot: &mut f64,
        var_sp_s_a_dn7_slot: &mut f64,
        var_sp_s_a_dn8_slot: &mut f64,
        var_sp_s_a_fac_slot: &mut f64,
        var_sp_s_a_fac_dn5_slot: &mut f64,
        var_sp_s_a_fac_dn6_slot: &mut f64,
        var_sp_s_a_fac_dn7_slot: &mut f64,
        var_sp_s_a_fac_dn8_slot: &mut f64,
        var_sp_s_c_slot: &mut f64,
        var_sp_s_c_dn5_slot: &mut f64,
        var_sp_s_c_dn6_slot: &mut f64,
        var_sp_s_c_dn7_slot: &mut f64,
        var_sp_s_c_dn8_slot: &mut f64,
        var_sp_s_delta0_slot: &mut f64,
        var_sp_s_delta0_dn5_slot: &mut f64,
        var_sp_s_delta0_dn6_slot: &mut f64,
        var_sp_s_delta0_dn7_slot: &mut f64,
        var_sp_s_delta0_dn8_slot: &mut f64,
        var_sp_s_delta1_slot: &mut f64,
        var_sp_s_delta1_dn5_slot: &mut f64,
        var_sp_s_delta1_dn6_slot: &mut f64,
        var_sp_s_delta1_dn7_slot: &mut f64,
        var_sp_s_delta1_dn8_slot: &mut f64,
        var_sp_s_eta_slot: &mut f64,
        var_sp_s_eta_dn5_slot: &mut f64,
        var_sp_s_eta_dn6_slot: &mut f64,
        var_sp_s_eta_dn7_slot: &mut f64,
        var_sp_s_eta_dn8_slot: &mut f64,
        var_sp_s_pc_slot: &mut f64,
        var_sp_s_pc_dn5_slot: &mut f64,
        var_sp_s_pc_dn6_slot: &mut f64,
        var_sp_s_pc_dn7_slot: &mut f64,
        var_sp_s_pc_dn8_slot: &mut f64,
        var_sp_s_qc_slot: &mut f64,
        var_sp_s_qc_dn5_slot: &mut f64,
        var_sp_s_qc_dn6_slot: &mut f64,
        var_sp_s_qc_dn7_slot: &mut f64,
        var_sp_s_qc_dn8_slot: &mut f64,
        var_sp_s_tau_slot: &mut f64,
        var_sp_s_tau_dn5_slot: &mut f64,
        var_sp_s_tau_dn6_slot: &mut f64,
        var_sp_s_tau_dn7_slot: &mut f64,
        var_sp_s_tau_dn8_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp1_slot: &mut f64,
        var_sp_s_temp1_dn5_slot: &mut f64,
        var_sp_s_temp1_dn6_slot: &mut f64,
        var_sp_s_temp1_dn7_slot: &mut f64,
        var_sp_s_temp1_dn8_slot: &mut f64,
        var_sp_s_temp_dn5_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_xbar_slot: &mut f64,
        var_sp_s_xbar_dn5_slot: &mut f64,
        var_sp_s_xbar_dn6_slot: &mut f64,
        var_sp_s_xbar_dn7_slot: &mut f64,
        var_sp_s_xbar_dn8_slot: &mut f64,
        var_sp_s_xi0_slot: &mut f64,
        var_sp_s_xi0_dn5_slot: &mut f64,
        var_sp_s_xi0_dn6_slot: &mut f64,
        var_sp_s_xi0_dn7_slot: &mut f64,
        var_sp_s_xi0_dn8_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn5_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn5_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
        var_sp_s_y0_slot: &mut f64,
        var_sp_s_y0_dn5_slot: &mut f64,
        var_sp_s_y0_dn6_slot: &mut f64,
        var_sp_s_y0_dn7_slot: &mut f64,
        var_sp_s_y0_dn8_slot: &mut f64,
        var_sp_s_yg_slot: &mut f64,
        var_sp_s_yg_dn5_slot: &mut f64,
        var_sp_s_yg_dn6_slot: &mut f64,
        var_sp_s_yg_dn7_slot: &mut f64,
        var_sp_s_yg_dn8_slot: &mut f64,
        var_sp_s_ysub_slot: &mut f64,
        var_sp_s_ysub_dn5_slot: &mut f64,
        var_sp_s_ysub_dn6_slot: &mut f64,
        var_sp_s_ysub_dn7_slot: &mut f64,
        var_sp_s_ysub_dn8_slot: &mut f64,
        var_sp_xg1_slot: &mut f64,
        var_sp_xg1_dn5_slot: &mut f64,
        var_sp_xg1_dn6_slot: &mut f64,
        var_sp_xg1_dn7_slot: &mut f64,
        var_sp_xg1_dn8_slot: &mut f64,
        var_x_s_slot: &mut f64,
        var_x_s_dn5_slot: &mut f64,
        var_x_s_dn6_slot: &mut f64,
        var_x_s_dn7_slot: &mut f64,
        var_x_s_dn8_slot: &mut f64,
    ) {
        let mut var_guard1183: f64 = *var_guard1183_slot;
        let mut var_guard1184: f64 = *var_guard1184_slot;
        let mut var_mutau: f64 = *var_mutau_slot;
        let mut var_mutau_dn5: f64 = *var_mutau_dn5_slot;
        let mut var_mutau_dn6: f64 = *var_mutau_dn6_slot;
        let mut var_mutau_dn7: f64 = *var_mutau_dn7_slot;
        let mut var_mutau_dn8: f64 = *var_mutau_dn8_slot;
        let mut var_nu: f64 = *var_nu_slot;
        let mut var_nu_dn5: f64 = *var_nu_dn5_slot;
        let mut var_nu_dn6: f64 = *var_nu_dn6_slot;
        let mut var_nu_dn7: f64 = *var_nu_dn7_slot;
        let mut var_nu_dn8: f64 = *var_nu_dn8_slot;
        let mut var_sp_s_a: f64 = *var_sp_s_a_slot;
        let mut var_sp_s_a_dn5: f64 = *var_sp_s_a_dn5_slot;
        let mut var_sp_s_a_dn6: f64 = *var_sp_s_a_dn6_slot;
        let mut var_sp_s_a_dn7: f64 = *var_sp_s_a_dn7_slot;
        let mut var_sp_s_a_dn8: f64 = *var_sp_s_a_dn8_slot;
        let mut var_sp_s_a_fac: f64 = *var_sp_s_a_fac_slot;
        let mut var_sp_s_a_fac_dn5: f64 = *var_sp_s_a_fac_dn5_slot;
        let mut var_sp_s_a_fac_dn6: f64 = *var_sp_s_a_fac_dn6_slot;
        let mut var_sp_s_a_fac_dn7: f64 = *var_sp_s_a_fac_dn7_slot;
        let mut var_sp_s_a_fac_dn8: f64 = *var_sp_s_a_fac_dn8_slot;
        let mut var_sp_s_c: f64 = *var_sp_s_c_slot;
        let mut var_sp_s_c_dn5: f64 = *var_sp_s_c_dn5_slot;
        let mut var_sp_s_c_dn6: f64 = *var_sp_s_c_dn6_slot;
        let mut var_sp_s_c_dn7: f64 = *var_sp_s_c_dn7_slot;
        let mut var_sp_s_c_dn8: f64 = *var_sp_s_c_dn8_slot;
        let mut var_sp_s_delta0: f64 = *var_sp_s_delta0_slot;
        let mut var_sp_s_delta0_dn5: f64 = *var_sp_s_delta0_dn5_slot;
        let mut var_sp_s_delta0_dn6: f64 = *var_sp_s_delta0_dn6_slot;
        let mut var_sp_s_delta0_dn7: f64 = *var_sp_s_delta0_dn7_slot;
        let mut var_sp_s_delta0_dn8: f64 = *var_sp_s_delta0_dn8_slot;
        let mut var_sp_s_delta1: f64 = *var_sp_s_delta1_slot;
        let mut var_sp_s_delta1_dn5: f64 = *var_sp_s_delta1_dn5_slot;
        let mut var_sp_s_delta1_dn6: f64 = *var_sp_s_delta1_dn6_slot;
        let mut var_sp_s_delta1_dn7: f64 = *var_sp_s_delta1_dn7_slot;
        let mut var_sp_s_delta1_dn8: f64 = *var_sp_s_delta1_dn8_slot;
        let mut var_sp_s_eta: f64 = *var_sp_s_eta_slot;
        let mut var_sp_s_eta_dn5: f64 = *var_sp_s_eta_dn5_slot;
        let mut var_sp_s_eta_dn6: f64 = *var_sp_s_eta_dn6_slot;
        let mut var_sp_s_eta_dn7: f64 = *var_sp_s_eta_dn7_slot;
        let mut var_sp_s_eta_dn8: f64 = *var_sp_s_eta_dn8_slot;
        let mut var_sp_s_pc: f64 = *var_sp_s_pc_slot;
        let mut var_sp_s_pc_dn5: f64 = *var_sp_s_pc_dn5_slot;
        let mut var_sp_s_pc_dn6: f64 = *var_sp_s_pc_dn6_slot;
        let mut var_sp_s_pc_dn7: f64 = *var_sp_s_pc_dn7_slot;
        let mut var_sp_s_pc_dn8: f64 = *var_sp_s_pc_dn8_slot;
        let mut var_sp_s_qc: f64 = *var_sp_s_qc_slot;
        let mut var_sp_s_qc_dn5: f64 = *var_sp_s_qc_dn5_slot;
        let mut var_sp_s_qc_dn6: f64 = *var_sp_s_qc_dn6_slot;
        let mut var_sp_s_qc_dn7: f64 = *var_sp_s_qc_dn7_slot;
        let mut var_sp_s_qc_dn8: f64 = *var_sp_s_qc_dn8_slot;
        let mut var_sp_s_tau: f64 = *var_sp_s_tau_slot;
        let mut var_sp_s_tau_dn5: f64 = *var_sp_s_tau_dn5_slot;
        let mut var_sp_s_tau_dn6: f64 = *var_sp_s_tau_dn6_slot;
        let mut var_sp_s_tau_dn7: f64 = *var_sp_s_tau_dn7_slot;
        let mut var_sp_s_tau_dn8: f64 = *var_sp_s_tau_dn8_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp1: f64 = *var_sp_s_temp1_slot;
        let mut var_sp_s_temp1_dn5: f64 = *var_sp_s_temp1_dn5_slot;
        let mut var_sp_s_temp1_dn6: f64 = *var_sp_s_temp1_dn6_slot;
        let mut var_sp_s_temp1_dn7: f64 = *var_sp_s_temp1_dn7_slot;
        let mut var_sp_s_temp1_dn8: f64 = *var_sp_s_temp1_dn8_slot;
        let mut var_sp_s_temp_dn5: f64 = *var_sp_s_temp_dn5_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_xbar: f64 = *var_sp_s_xbar_slot;
        let mut var_sp_s_xbar_dn5: f64 = *var_sp_s_xbar_dn5_slot;
        let mut var_sp_s_xbar_dn6: f64 = *var_sp_s_xbar_dn6_slot;
        let mut var_sp_s_xbar_dn7: f64 = *var_sp_s_xbar_dn7_slot;
        let mut var_sp_s_xbar_dn8: f64 = *var_sp_s_xbar_dn8_slot;
        let mut var_sp_s_xi0: f64 = *var_sp_s_xi0_slot;
        let mut var_sp_s_xi0_dn5: f64 = *var_sp_s_xi0_dn5_slot;
        let mut var_sp_s_xi0_dn6: f64 = *var_sp_s_xi0_dn6_slot;
        let mut var_sp_s_xi0_dn7: f64 = *var_sp_s_xi0_dn7_slot;
        let mut var_sp_s_xi0_dn8: f64 = *var_sp_s_xi0_dn8_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn5: f64 = *var_sp_s_xi1_dn5_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn5: f64 = *var_sp_s_xi2_dn5_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;
        let mut var_sp_s_y0: f64 = *var_sp_s_y0_slot;
        let mut var_sp_s_y0_dn5: f64 = *var_sp_s_y0_dn5_slot;
        let mut var_sp_s_y0_dn6: f64 = *var_sp_s_y0_dn6_slot;
        let mut var_sp_s_y0_dn7: f64 = *var_sp_s_y0_dn7_slot;
        let mut var_sp_s_y0_dn8: f64 = *var_sp_s_y0_dn8_slot;
        let mut var_sp_s_yg: f64 = *var_sp_s_yg_slot;
        let mut var_sp_s_yg_dn5: f64 = *var_sp_s_yg_dn5_slot;
        let mut var_sp_s_yg_dn6: f64 = *var_sp_s_yg_dn6_slot;
        let mut var_sp_s_yg_dn7: f64 = *var_sp_s_yg_dn7_slot;
        let mut var_sp_s_yg_dn8: f64 = *var_sp_s_yg_dn8_slot;
        let mut var_sp_s_ysub: f64 = *var_sp_s_ysub_slot;
        let mut var_sp_s_ysub_dn5: f64 = *var_sp_s_ysub_dn5_slot;
        let mut var_sp_s_ysub_dn6: f64 = *var_sp_s_ysub_dn6_slot;
        let mut var_sp_s_ysub_dn7: f64 = *var_sp_s_ysub_dn7_slot;
        let mut var_sp_s_ysub_dn8: f64 = *var_sp_s_ysub_dn8_slot;
        let mut var_sp_xg1: f64 = *var_sp_xg1_slot;
        let mut var_sp_xg1_dn5: f64 = *var_sp_xg1_dn5_slot;
        let mut var_sp_xg1_dn6: f64 = *var_sp_xg1_dn6_slot;
        let mut var_sp_xg1_dn7: f64 = *var_sp_xg1_dn7_slot;
        let mut var_sp_xg1_dn8: f64 = *var_sp_xg1_dn8_slot;
        let mut var_x_s: f64 = *var_x_s_slot;
        let mut var_x_s_dn5: f64 = *var_x_s_dn5_slot;
        let mut var_x_s_dn6: f64 = *var_x_s_dn6_slot;
        let mut var_x_s_dn7: f64 = *var_x_s_dn7_slot;
        let mut var_x_s_dn8: f64 = *var_x_s_dn8_slot;

        let (assign41600_e54498, assign41600_e54498_d_n5, assign41600_e54498_d_n6, assign41600_e54498_d_n7, assign41600_e54498_d_n8,) = {
    if (var_guard1182 != 0.0) {
        let assign41600_e54492: f64 = (var_inv_xi * var_inv_xi);
        let assign41600_e54494: f64 = (assign41600_e54492 * 0.16666666666666666);
        let assign41600_e54496: f64 = (assign41600_e54494 * 0.7071067811865475);
        (assign41600_e54496, ((((var_inv_xi_dn5 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn6 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn7 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn8 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn5, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8,)
    }
};
        var_sp_s_temp1 = assign41600_e54498;
        var_sp_s_temp1_dn5 = assign41600_e54498_d_n5;
        var_sp_s_temp1_dn6 = assign41600_e54498_d_n6;
        var_sp_s_temp1_dn7 = assign41600_e54498_d_n7;
        var_sp_s_temp1_dn8 = assign41600_e54498_d_n8;

        let (assign41610_e54516, assign41610_e54516_d_n5, assign41610_e54516_d_n6, assign41610_e54516_d_n7, assign41610_e54516_d_n8,) = {
    if (var_guard1182 != 0.0) {
        let assign41610_e54502: f64 = (var_xg * var_inv_xi);
        let assign41610_e54507: f64 = (1.0 - var_delta_ns);
        let assign41610_e54508: f64 = (var_xg * assign41610_e54507);
        let assign41610_e54510: f64 = (assign41610_e54508 * var_gf);
        let assign41610_e54512: f64 = (assign41610_e54510 * var_sp_s_temp1);
        let assign41610_e54513: f64 = (1.0 + assign41610_e54512);
        let assign41610_e54514: f64 = (assign41610_e54502 * assign41610_e54513);
        (assign41610_e54514, ((((var_xg_dn5 * var_inv_xi) + (var_xg * var_inv_xi_dn5)) * assign41610_e54513) + (assign41610_e54502 * ((((((var_xg_dn5 * assign41610_e54507) + (var_xg * (-var_delta_ns_dn5))) * var_gf) + (assign41610_e54508 * var_gf_dn5)) * var_sp_s_temp1) + (assign41610_e54510 * var_sp_s_temp1_dn5)))), ((((var_xg_dn6 * var_inv_xi) + (var_xg * var_inv_xi_dn6)) * assign41610_e54513) + (assign41610_e54502 * ((((((var_xg_dn6 * assign41610_e54507) + (var_xg * (-var_delta_ns_dn6))) * var_gf) + (assign41610_e54508 * var_gf_dn6)) * var_sp_s_temp1) + (assign41610_e54510 * var_sp_s_temp1_dn6)))), ((((var_xg_dn7 * var_inv_xi) + (var_xg * var_inv_xi_dn7)) * assign41610_e54513) + (assign41610_e54502 * ((((((var_xg_dn7 * assign41610_e54507) + (var_xg * (-var_delta_ns_dn7))) * var_gf) + (assign41610_e54508 * var_gf_dn7)) * var_sp_s_temp1) + (assign41610_e54510 * var_sp_s_temp1_dn7)))), ((((var_xg_dn8 * var_inv_xi) + (var_xg * var_inv_xi_dn8)) * assign41610_e54513) + (assign41610_e54502 * ((((((var_xg_dn8 * assign41610_e54507) + (var_xg * (-var_delta_ns_dn8))) * var_gf) + (assign41610_e54508 * var_gf_dn8)) * var_sp_s_temp1) + (assign41610_e54510 * var_sp_s_temp1_dn8)))),)
    } else {
        (var_x_s, var_x_s_dn5, var_x_s_dn6, var_x_s_dn7, var_x_s_dn8,)
    }
};
        var_x_s = assign41610_e54516;
        var_x_s_dn5 = assign41610_e54516_d_n5;
        var_x_s_dn6 = assign41610_e54516_d_n6;
        var_x_s_dn7 = assign41610_e54516_d_n7;
        var_x_s_dn8 = assign41610_e54516_d_n8;

        let assign41620_e54519: f64 = (-var_margin);
        let assign41620_e54520: f64 = if var_xg < assign41620_e54519 { 1.0 } else { 0.0 };
        var_guard1183 = assign41620_e54520;

        let (assign41630_e54528, assign41630_e54528_d_n5, assign41630_e54528_d_n6, assign41630_e54528_d_n7, assign41630_e54528_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41630_e54526: f64 = (-var_xg);
        (assign41630_e54526, (-var_xg_dn5), (-var_xg_dn6), (-var_xg_dn7), (-var_xg_dn8),)
    } else {
        (var_sp_s_yg, var_sp_s_yg_dn5, var_sp_s_yg_dn6, var_sp_s_yg_dn7, var_sp_s_yg_dn8,)
    }
};
        var_sp_s_yg = assign41630_e54528;
        var_sp_s_yg_dn5 = assign41630_e54528_d_n5;
        var_sp_s_yg_dn6 = assign41630_e54528_d_n6;
        var_sp_s_yg_dn7 = assign41630_e54528_d_n7;
        var_sp_s_yg_dn8 = assign41630_e54528_d_n8;

        let (assign41640_e54539, assign41640_e54539_d_n5, assign41640_e54539_d_n6, assign41640_e54539_d_n7, assign41640_e54539_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41640_e54536: f64 = (var_sp_s_yg * var_inv_xi);
        let assign41640_e54537: f64 = (1.25 * assign41640_e54536);
        (assign41640_e54537, (1.25 * ((var_sp_s_yg_dn5 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn5))), (1.25 * ((var_sp_s_yg_dn6 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn6))), (1.25 * ((var_sp_s_yg_dn7 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn7))), (1.25 * ((var_sp_s_yg_dn8 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn8))),)
    } else {
        (var_sp_s_ysub, var_sp_s_ysub_dn5, var_sp_s_ysub_dn6, var_sp_s_ysub_dn7, var_sp_s_ysub_dn8,)
    }
};
        var_sp_s_ysub = assign41640_e54539;
        var_sp_s_ysub_dn5 = assign41640_e54539_d_n5;
        var_sp_s_ysub_dn6 = assign41640_e54539_d_n6;
        var_sp_s_ysub_dn7 = assign41640_e54539_d_n7;
        var_sp_s_ysub_dn8 = assign41640_e54539_d_n8;

        let (assign41650_e54561, assign41650_e54561_d_n5, assign41650_e54561_d_n6, assign41650_e54561_d_n7, assign41650_e54561_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41650_e54547: f64 = (var_sp_s_ysub + 10.0);
        let assign41650_e54550: f64 = (var_sp_s_ysub - 6.0);
        let assign41650_e54553: f64 = (var_sp_s_ysub - 6.0);
        let assign41650_e54554: f64 = (assign41650_e54550 * assign41650_e54553);
        let assign41650_e54556: f64 = (assign41650_e54554 + 64.0);
        let assign41650_e54557: f64 = (assign41650_e54556).sqrt();
        let assign41650_e54558: f64 = (assign41650_e54547 - assign41650_e54557);
        let assign41650_e54559: f64 = (0.5 * assign41650_e54558);
        (assign41650_e54559, (0.5 * (var_sp_s_ysub_dn5 - (((var_sp_s_ysub_dn5 * assign41650_e54553) + (assign41650_e54550 * var_sp_s_ysub_dn5)) / (2.0 * assign41650_e54557)))), (0.5 * (var_sp_s_ysub_dn6 - (((var_sp_s_ysub_dn6 * assign41650_e54553) + (assign41650_e54550 * var_sp_s_ysub_dn6)) / (2.0 * assign41650_e54557)))), (0.5 * (var_sp_s_ysub_dn7 - (((var_sp_s_ysub_dn7 * assign41650_e54553) + (assign41650_e54550 * var_sp_s_ysub_dn7)) / (2.0 * assign41650_e54557)))), (0.5 * (var_sp_s_ysub_dn8 - (((var_sp_s_ysub_dn8 * assign41650_e54553) + (assign41650_e54550 * var_sp_s_ysub_dn8)) / (2.0 * assign41650_e54557)))),)
    } else {
        (var_sp_s_eta, var_sp_s_eta_dn5, var_sp_s_eta_dn6, var_sp_s_eta_dn7, var_sp_s_eta_dn8,)
    }
};
        var_sp_s_eta = assign41650_e54561;
        var_sp_s_eta_dn5 = assign41650_e54561_d_n5;
        var_sp_s_eta_dn6 = assign41650_e54561_d_n6;
        var_sp_s_eta_dn7 = assign41650_e54561_d_n7;
        var_sp_s_eta_dn8 = assign41650_e54561_d_n8;

        let (assign41660_e54570, assign41660_e54570_d_n5, assign41660_e54570_d_n6, assign41660_e54570_d_n7, assign41660_e54570_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41660_e54568: f64 = (var_sp_s_yg - var_sp_s_eta);
        (assign41660_e54568, (var_sp_s_yg_dn5 - var_sp_s_eta_dn5), (var_sp_s_yg_dn6 - var_sp_s_eta_dn6), (var_sp_s_yg_dn7 - var_sp_s_eta_dn7), (var_sp_s_yg_dn8 - var_sp_s_eta_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41660_e54570;
        var_sp_s_temp_dn5 = assign41660_e54570_d_n5;
        var_sp_s_temp_dn6 = assign41660_e54570_d_n6;
        var_sp_s_temp_dn7 = assign41660_e54570_d_n7;
        var_sp_s_temp_dn8 = assign41660_e54570_d_n8;

        let (assign41670_e54585, assign41670_e54585_d_n5, assign41670_e54585_d_n6, assign41670_e54585_d_n7, assign41670_e54585_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41670_e54577: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign41670_e54581: f64 = (var_sp_s_eta + 1.0);
        let assign41670_e54582: f64 = (var_gf2 * assign41670_e54581);
        let assign41670_e54583: f64 = (assign41670_e54577 + assign41670_e54582);
        (assign41670_e54583, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) + ((var_gf2_dn5 * assign41670_e54581) + (var_gf2 * var_sp_s_eta_dn5))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) + ((var_gf2_dn6 * assign41670_e54581) + (var_gf2 * var_sp_s_eta_dn6))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) + ((var_gf2_dn7 * assign41670_e54581) + (var_gf2 * var_sp_s_eta_dn7))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) + ((var_gf2_dn8 * assign41670_e54581) + (var_gf2 * var_sp_s_eta_dn8))),)
    } else {
        (var_sp_s_a, var_sp_s_a_dn5, var_sp_s_a_dn6, var_sp_s_a_dn7, var_sp_s_a_dn8,)
    }
};
        var_sp_s_a = assign41670_e54585;
        var_sp_s_a_dn5 = assign41670_e54585_d_n5;
        var_sp_s_a_dn6 = assign41670_e54585_d_n6;
        var_sp_s_a_dn7 = assign41670_e54585_d_n7;
        var_sp_s_a_dn8 = assign41670_e54585_d_n8;

        let (assign41680_e54596, assign41680_e54596_d_n5, assign41680_e54596_d_n6, assign41680_e54596_d_n7, assign41680_e54596_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41680_e54592: f64 = (2.0 * var_sp_s_temp);
        let assign41680_e54594: f64 = (assign41680_e54592 - var_gf2);
        (assign41680_e54594, ((2.0 * var_sp_s_temp_dn5) - var_gf2_dn5), ((2.0 * var_sp_s_temp_dn6) - var_gf2_dn6), ((2.0 * var_sp_s_temp_dn7) - var_gf2_dn7), ((2.0 * var_sp_s_temp_dn8) - var_gf2_dn8),)
    } else {
        (var_sp_s_c, var_sp_s_c_dn5, var_sp_s_c_dn6, var_sp_s_c_dn7, var_sp_s_c_dn8,)
    }
};
        var_sp_s_c = assign41680_e54596;
        var_sp_s_c_dn5 = assign41680_e54596_d_n5;
        var_sp_s_c_dn6 = assign41680_e54596_d_n6;
        var_sp_s_c_dn7 = assign41680_e54596_d_n7;
        var_sp_s_c_dn8 = assign41680_e54596_d_n8;

        let (assign41690_e54609, assign41690_e54609_d_n5, assign41690_e54609_d_n6, assign41690_e54609_d_n7, assign41690_e54609_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41690_e54602: f64 = (-var_sp_s_eta);
        let assign41690_e54605: f64 = (var_sp_s_a * var_inv_gf2);
        let assign41690_e54606: f64 = (assign41690_e54605).ln();
        let assign41690_e54607: f64 = (assign41690_e54602 + assign41690_e54606);
        (assign41690_e54607, ((-var_sp_s_eta_dn5) + (((var_sp_s_a_dn5 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn5)) / assign41690_e54605)), ((-var_sp_s_eta_dn6) + (((var_sp_s_a_dn6 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn6)) / assign41690_e54605)), ((-var_sp_s_eta_dn7) + (((var_sp_s_a_dn7 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn7)) / assign41690_e54605)), ((-var_sp_s_eta_dn8) + (((var_sp_s_a_dn8 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn8)) / assign41690_e54605)),)
    } else {
        (var_sp_s_tau, var_sp_s_tau_dn5, var_sp_s_tau_dn6, var_sp_s_tau_dn7, var_sp_s_tau_dn8,)
    }
};
        var_sp_s_tau = assign41690_e54609;
        var_sp_s_tau_dn5 = assign41690_e54609_d_n5;
        var_sp_s_tau_dn6 = assign41690_e54609_d_n6;
        var_sp_s_tau_dn7 = assign41690_e54609_d_n7;
        var_sp_s_tau_dn8 = assign41690_e54609_d_n8;

        let (assign41700_e54618, assign41700_e54618_d_n5, assign41700_e54618_d_n6, assign41700_e54618_d_n7, assign41700_e54618_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41700_e54616: f64 = (var_sp_s_a + var_sp_s_c);
        (assign41700_e54616, (var_sp_s_a_dn5 + var_sp_s_c_dn5), (var_sp_s_a_dn6 + var_sp_s_c_dn6), (var_sp_s_a_dn7 + var_sp_s_c_dn7), (var_sp_s_a_dn8 + var_sp_s_c_dn8),)
    } else {
        (var_nu, var_nu_dn5, var_nu_dn6, var_nu_dn7, var_nu_dn8,)
    }
};
        var_nu = assign41700_e54618;
        var_nu_dn5 = assign41700_e54618_d_n5;
        var_nu_dn6 = assign41700_e54618_d_n6;
        var_nu_dn7 = assign41700_e54618_d_n7;
        var_nu_dn8 = assign41700_e54618_d_n8;

        let (assign41710_e54637, assign41710_e54637_d_n5, assign41710_e54637_d_n6, assign41710_e54637_d_n7, assign41710_e54637_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41710_e54625: f64 = (var_nu * var_nu);
        let assign41710_e54630: f64 = (var_sp_s_c * var_sp_s_c);
        let assign41710_e54631: f64 = (0.5 * assign41710_e54630);
        let assign41710_e54633: f64 = (assign41710_e54631 - var_sp_s_a);
        let assign41710_e54634: f64 = (var_sp_s_tau * assign41710_e54633);
        let assign41710_e54635: f64 = (assign41710_e54625 + assign41710_e54634);
        (assign41710_e54635, (((var_nu_dn5 * var_nu) + (var_nu * var_nu_dn5)) + ((var_sp_s_tau_dn5 * assign41710_e54633) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5))) - var_sp_s_a_dn5)))), (((var_nu_dn6 * var_nu) + (var_nu * var_nu_dn6)) + ((var_sp_s_tau_dn6 * assign41710_e54633) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6))) - var_sp_s_a_dn6)))), (((var_nu_dn7 * var_nu) + (var_nu * var_nu_dn7)) + ((var_sp_s_tau_dn7 * assign41710_e54633) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7))) - var_sp_s_a_dn7)))), (((var_nu_dn8 * var_nu) + (var_nu * var_nu_dn8)) + ((var_sp_s_tau_dn8 * assign41710_e54633) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8))) - var_sp_s_a_dn8)))),)
    } else {
        (var_mutau, var_mutau_dn5, var_mutau_dn6, var_mutau_dn7, var_mutau_dn8,)
    }
};
        var_mutau = assign41710_e54637;
        var_mutau_dn5 = assign41710_e54637_d_n5;
        var_mutau_dn6 = assign41710_e54637_d_n6;
        var_mutau_dn7 = assign41710_e54637_d_n7;
        var_mutau_dn8 = assign41710_e54637_d_n8;

        let (assign41720_e54670, assign41720_e54670_d_n5, assign41720_e54670_d_n6, assign41720_e54670_d_n7, assign41720_e54670_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41720_e54645: f64 = (var_sp_s_a * var_nu);
        let assign41720_e54647: f64 = (assign41720_e54645 * var_sp_s_tau);
        let assign41720_e54651: f64 = (var_nu / var_mutau);
        let assign41720_e54653: f64 = (assign41720_e54651 * var_sp_s_tau);
        let assign41720_e54655: f64 = (assign41720_e54653 * var_sp_s_tau);
        let assign41720_e54657: f64 = (assign41720_e54655 * var_sp_s_c);
        let assign41720_e54660: f64 = (var_sp_s_c * var_sp_s_c);
        let assign41720_e54662: f64 = (assign41720_e54660 * 0.3333333333333333);
        let assign41720_e54664: f64 = (assign41720_e54662 - var_sp_s_a);
        let assign41720_e54665: f64 = (assign41720_e54657 * assign41720_e54664);
        let assign41720_e54666: f64 = (var_mutau + assign41720_e54665);
        let assign41720_e54667: f64 = (assign41720_e54647 / assign41720_e54666);
        let assign41720_e54668: f64 = (var_sp_s_eta + assign41720_e54667);
        (assign41720_e54668, (var_sp_s_eta_dn5 + (((((((var_sp_s_a_dn5 * var_nu) + (var_sp_s_a * var_nu_dn5)) * var_sp_s_tau) + (assign41720_e54645 * var_sp_s_tau_dn5)) * assign41720_e54666) - (assign41720_e54647 * (var_mutau_dn5 + (((((((((((var_nu_dn5 * var_mutau) - (var_nu * var_mutau_dn5)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41720_e54651 * var_sp_s_tau_dn5)) * var_sp_s_tau) + (assign41720_e54653 * var_sp_s_tau_dn5)) * var_sp_s_c) + (assign41720_e54655 * var_sp_s_c_dn5)) * assign41720_e54664) + (assign41720_e54657 * ((((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5)) * 0.3333333333333333) - var_sp_s_a_dn5)))))) / (assign41720_e54666 * assign41720_e54666))), (var_sp_s_eta_dn6 + (((((((var_sp_s_a_dn6 * var_nu) + (var_sp_s_a * var_nu_dn6)) * var_sp_s_tau) + (assign41720_e54645 * var_sp_s_tau_dn6)) * assign41720_e54666) - (assign41720_e54647 * (var_mutau_dn6 + (((((((((((var_nu_dn6 * var_mutau) - (var_nu * var_mutau_dn6)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41720_e54651 * var_sp_s_tau_dn6)) * var_sp_s_tau) + (assign41720_e54653 * var_sp_s_tau_dn6)) * var_sp_s_c) + (assign41720_e54655 * var_sp_s_c_dn6)) * assign41720_e54664) + (assign41720_e54657 * ((((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6)) * 0.3333333333333333) - var_sp_s_a_dn6)))))) / (assign41720_e54666 * assign41720_e54666))), (var_sp_s_eta_dn7 + (((((((var_sp_s_a_dn7 * var_nu) + (var_sp_s_a * var_nu_dn7)) * var_sp_s_tau) + (assign41720_e54645 * var_sp_s_tau_dn7)) * assign41720_e54666) - (assign41720_e54647 * (var_mutau_dn7 + (((((((((((var_nu_dn7 * var_mutau) - (var_nu * var_mutau_dn7)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41720_e54651 * var_sp_s_tau_dn7)) * var_sp_s_tau) + (assign41720_e54653 * var_sp_s_tau_dn7)) * var_sp_s_c) + (assign41720_e54655 * var_sp_s_c_dn7)) * assign41720_e54664) + (assign41720_e54657 * ((((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7)) * 0.3333333333333333) - var_sp_s_a_dn7)))))) / (assign41720_e54666 * assign41720_e54666))), (var_sp_s_eta_dn8 + (((((((var_sp_s_a_dn8 * var_nu) + (var_sp_s_a * var_nu_dn8)) * var_sp_s_tau) + (assign41720_e54645 * var_sp_s_tau_dn8)) * assign41720_e54666) - (assign41720_e54647 * (var_mutau_dn8 + (((((((((((var_nu_dn8 * var_mutau) - (var_nu * var_mutau_dn8)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41720_e54651 * var_sp_s_tau_dn8)) * var_sp_s_tau) + (assign41720_e54653 * var_sp_s_tau_dn8)) * var_sp_s_c) + (assign41720_e54655 * var_sp_s_c_dn8)) * assign41720_e54664) + (assign41720_e54657 * ((((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8)) * 0.3333333333333333) - var_sp_s_a_dn8)))))) / (assign41720_e54666 * assign41720_e54666))),)
    } else {
        (var_sp_s_y0, var_sp_s_y0_dn5, var_sp_s_y0_dn6, var_sp_s_y0_dn7, var_sp_s_y0_dn8,)
    }
};
        var_sp_s_y0 = assign41720_e54670;
        var_sp_s_y0_dn5 = assign41720_e54670_d_n5;
        var_sp_s_y0_dn6 = assign41720_e54670_d_n6;
        var_sp_s_y0_dn7 = assign41720_e54670_d_n7;
        var_sp_s_y0_dn8 = assign41720_e54670_d_n8;

        let assign41730_e54673: f64 = if var_sp_s_y0 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1184 = assign41730_e54673;

        let (assign41740_e54683, assign41740_e54683_d_n5, assign41740_e54683_d_n6, assign41740_e54683_d_n7, assign41740_e54683_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) && (var_guard1184 != 0.0)) {
        let assign41740_e54681: f64 = (var_sp_s_y0).exp();
        (assign41740_e54681, (assign41740_e54681 * var_sp_s_y0_dn5), (assign41740_e54681 * var_sp_s_y0_dn6), (assign41740_e54681 * var_sp_s_y0_dn7), (assign41740_e54681 * var_sp_s_y0_dn8),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign41740_e54683;
        var_sp_s_delta0_dn5 = assign41740_e54683_d_n5;
        var_sp_s_delta0_dn6 = assign41740_e54683_d_n6;
        var_sp_s_delta0_dn7 = assign41740_e54683_d_n7;
        var_sp_s_delta0_dn8 = assign41740_e54683_d_n8;

        let (assign41750_e54715, assign41750_e54715_d_n5, assign41750_e54715_d_n6, assign41750_e54715_d_n7, assign41750_e54715_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) && (var_guard1184 == 0.0)) {
        let assign41750_e54695: f64 = (var_sp_s_y0 - 230.25850929940458);
        let assign41750_e54700: f64 = (var_sp_s_y0 - 230.25850929940458);
        let assign41750_e54704: f64 = (var_sp_s_y0 - 230.25850929940458);
        let assign41750_e54706: f64 = (assign41750_e54704 * 0.3333333333333333);
        let assign41750_e54707: f64 = (1.0 + assign41750_e54706);
        let assign41750_e54708: f64 = (assign41750_e54700 * assign41750_e54707);
        let assign41750_e54709: f64 = (0.5 * assign41750_e54708);
        let assign41750_e54710: f64 = (1.0 + assign41750_e54709);
        let assign41750_e54711: f64 = (assign41750_e54695 * assign41750_e54710);
        let assign41750_e54712: f64 = (1.0 + assign41750_e54711);
        let assign41750_e54713: f64 = (1e100 * assign41750_e54712);
        (assign41750_e54713, (1e100 * ((var_sp_s_y0_dn5 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((var_sp_s_y0_dn5 * assign41750_e54707) + (assign41750_e54700 * (var_sp_s_y0_dn5 * 0.3333333333333333))))))), (1e100 * ((var_sp_s_y0_dn6 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((var_sp_s_y0_dn6 * assign41750_e54707) + (assign41750_e54700 * (var_sp_s_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((var_sp_s_y0_dn7 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((var_sp_s_y0_dn7 * assign41750_e54707) + (assign41750_e54700 * (var_sp_s_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((var_sp_s_y0_dn8 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((var_sp_s_y0_dn8 * assign41750_e54707) + (assign41750_e54700 * (var_sp_s_y0_dn8 * 0.3333333333333333))))))),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign41750_e54715;
        var_sp_s_delta0_dn5 = assign41750_e54715_d_n5;
        var_sp_s_delta0_dn6 = assign41750_e54715_d_n6;
        var_sp_s_delta0_dn7 = assign41750_e54715_d_n7;
        var_sp_s_delta0_dn8 = assign41750_e54715_d_n8;

        let (assign41760_e54724, assign41760_e54724_d_n5, assign41760_e54724_d_n6, assign41760_e54724_d_n7, assign41760_e54724_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41760_e54722: f64 = (1.0 / var_sp_s_delta0);
        (assign41760_e54722, (-(var_sp_s_delta0_dn5 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn6 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn7 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn8 / (var_sp_s_delta0 * var_sp_s_delta0))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign41760_e54724;
        var_sp_s_delta1_dn5 = assign41760_e54724_d_n5;
        var_sp_s_delta1_dn6 = assign41760_e54724_d_n6;
        var_sp_s_delta1_dn7 = assign41760_e54724_d_n7;
        var_sp_s_delta1_dn8 = assign41760_e54724_d_n8;

        let (assign41770_e54737, assign41770_e54737_d_n5, assign41770_e54737_d_n6, assign41770_e54737_d_n7, assign41770_e54737_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41770_e54733: f64 = (var_sp_s_y0 * var_sp_s_y0);
        let assign41770_e54734: f64 = (2.0 + assign41770_e54733);
        let assign41770_e54735: f64 = (1.0 / assign41770_e54734);
        (assign41770_e54735, (-(((var_sp_s_y0_dn5 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn5)) / (assign41770_e54734 * assign41770_e54734))), (-(((var_sp_s_y0_dn6 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn6)) / (assign41770_e54734 * assign41770_e54734))), (-(((var_sp_s_y0_dn7 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn7)) / (assign41770_e54734 * assign41770_e54734))), (-(((var_sp_s_y0_dn8 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn8)) / (assign41770_e54734 * assign41770_e54734))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41770_e54737;
        var_sp_s_temp_dn5 = assign41770_e54737_d_n5;
        var_sp_s_temp_dn6 = assign41770_e54737_d_n6;
        var_sp_s_temp_dn7 = assign41770_e54737_d_n7;
        var_sp_s_temp_dn8 = assign41770_e54737_d_n8;

        let (assign41780_e54748, assign41780_e54748_d_n5, assign41780_e54748_d_n6, assign41780_e54748_d_n7, assign41780_e54748_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41780_e54744: f64 = (var_sp_s_y0 * var_sp_s_y0);
        let assign41780_e54746: f64 = (assign41780_e54744 * var_sp_s_temp);
        (assign41780_e54746, ((((var_sp_s_y0_dn5 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn5)) * var_sp_s_temp) + (assign41780_e54744 * var_sp_s_temp_dn5)), ((((var_sp_s_y0_dn6 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn6)) * var_sp_s_temp) + (assign41780_e54744 * var_sp_s_temp_dn6)), ((((var_sp_s_y0_dn7 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn7)) * var_sp_s_temp) + (assign41780_e54744 * var_sp_s_temp_dn7)), ((((var_sp_s_y0_dn8 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn8)) * var_sp_s_temp) + (assign41780_e54744 * var_sp_s_temp_dn8)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn5, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8,)
    }
};
        var_sp_s_xi0 = assign41780_e54748;
        var_sp_s_xi0_dn5 = assign41780_e54748_d_n5;
        var_sp_s_xi0_dn6 = assign41780_e54748_d_n6;
        var_sp_s_xi0_dn7 = assign41780_e54748_d_n7;
        var_sp_s_xi0_dn8 = assign41780_e54748_d_n8;

        let (assign41790_e54761, assign41790_e54761_d_n5, assign41790_e54761_d_n6, assign41790_e54761_d_n7, assign41790_e54761_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41790_e54756: f64 = (var_sp_s_y0 * var_sp_s_temp);
        let assign41790_e54758: f64 = (assign41790_e54756 * var_sp_s_temp);
        let assign41790_e54759: f64 = (4.0 * assign41790_e54758);
        (assign41790_e54759, (4.0 * ((((var_sp_s_y0_dn5 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn5)) * var_sp_s_temp) + (assign41790_e54756 * var_sp_s_temp_dn5))), (4.0 * ((((var_sp_s_y0_dn6 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign41790_e54756 * var_sp_s_temp_dn6))), (4.0 * ((((var_sp_s_y0_dn7 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign41790_e54756 * var_sp_s_temp_dn7))), (4.0 * ((((var_sp_s_y0_dn8 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign41790_e54756 * var_sp_s_temp_dn8))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn5, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8,)
    }
};
        var_sp_s_xi1 = assign41790_e54761;
        var_sp_s_xi1_dn5 = assign41790_e54761_d_n5;
        var_sp_s_xi1_dn6 = assign41790_e54761_d_n6;
        var_sp_s_xi1_dn7 = assign41790_e54761_d_n7;
        var_sp_s_xi1_dn8 = assign41790_e54761_d_n8;

        let (assign41800_e54778, assign41800_e54778_d_n5, assign41800_e54778_d_n6, assign41800_e54778_d_n7, assign41800_e54778_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41800_e54768: f64 = (8.0 * var_sp_s_temp);
        let assign41800_e54771: f64 = (12.0 * var_sp_s_xi0);
        let assign41800_e54772: f64 = (assign41800_e54768 - assign41800_e54771);
        let assign41800_e54774: f64 = (assign41800_e54772 * var_sp_s_temp);
        let assign41800_e54776: f64 = (assign41800_e54774 * var_sp_s_temp);
        (assign41800_e54776, ((((((8.0 * var_sp_s_temp_dn5) - (12.0 * var_sp_s_xi0_dn5)) * var_sp_s_temp) + (assign41800_e54772 * var_sp_s_temp_dn5)) * var_sp_s_temp) + (assign41800_e54774 * var_sp_s_temp_dn5)), ((((((8.0 * var_sp_s_temp_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp) + (assign41800_e54772 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign41800_e54774 * var_sp_s_temp_dn6)), ((((((8.0 * var_sp_s_temp_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp) + (assign41800_e54772 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign41800_e54774 * var_sp_s_temp_dn7)), ((((((8.0 * var_sp_s_temp_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp) + (assign41800_e54772 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign41800_e54774 * var_sp_s_temp_dn8)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn5, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8,)
    }
};
        var_sp_s_xi2 = assign41800_e54778;
        var_sp_s_xi2_dn5 = assign41800_e54778_d_n5;
        var_sp_s_xi2_dn6 = assign41800_e54778_d_n6;
        var_sp_s_xi2_dn7 = assign41800_e54778_d_n7;
        var_sp_s_xi2_dn8 = assign41800_e54778_d_n8;

        let (assign41810_e54787, assign41810_e54787_d_n5, assign41810_e54787_d_n6, assign41810_e54787_d_n7, assign41810_e54787_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41810_e54785: f64 = (var_sp_s_yg - var_sp_s_y0);
        (assign41810_e54785, (var_sp_s_yg_dn5 - var_sp_s_y0_dn5), (var_sp_s_yg_dn6 - var_sp_s_y0_dn6), (var_sp_s_yg_dn7 - var_sp_s_y0_dn7), (var_sp_s_yg_dn8 - var_sp_s_y0_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41810_e54787;
        var_sp_s_temp_dn5 = assign41810_e54787_d_n5;
        var_sp_s_temp_dn6 = assign41810_e54787_d_n6;
        var_sp_s_temp_dn7 = assign41810_e54787_d_n7;
        var_sp_s_temp_dn8 = assign41810_e54787_d_n8;

        let (assign41820_e54796, assign41820_e54796_d_n5, assign41820_e54796_d_n6, assign41820_e54796_d_n7, assign41820_e54796_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41820_e54794: f64 = (var_delta_ns * var_sp_s_delta1);
        (assign41820_e54794, ((var_delta_ns_dn5 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn5)), ((var_delta_ns_dn6 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn6)), ((var_delta_ns_dn7 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn7)), ((var_delta_ns_dn8 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn8)),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn5, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8,)
    }
};
        var_sp_s_temp1 = assign41820_e54796;
        var_sp_s_temp1_dn5 = assign41820_e54796_d_n5;
        var_sp_s_temp1_dn6 = assign41820_e54796_d_n6;
        var_sp_s_temp1_dn7 = assign41820_e54796_d_n7;
        var_sp_s_temp1_dn8 = assign41820_e54796_d_n8;

        let (assign41830_e54819, assign41830_e54819_d_n5, assign41830_e54819_d_n6, assign41830_e54819_d_n7, assign41830_e54819_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41830_e54803: f64 = (2.0 * var_sp_s_temp);
        let assign41830_e54807: f64 = (var_sp_s_delta0 - 1.0);
        let assign41830_e54809: f64 = (assign41830_e54807 - var_sp_s_temp1);
        let assign41830_e54813: f64 = (1.0 - var_sp_s_xi1);
        let assign41830_e54814: f64 = (var_delta_ns * assign41830_e54813);
        let assign41830_e54815: f64 = (assign41830_e54809 + assign41830_e54814);
        let assign41830_e54816: f64 = (var_gf2 * assign41830_e54815);
        let assign41830_e54817: f64 = (assign41830_e54803 + assign41830_e54816);
        (assign41830_e54817, ((2.0 * var_sp_s_temp_dn5) + ((var_gf2_dn5 * assign41830_e54815) + (var_gf2 * ((var_sp_s_delta0_dn5 - var_sp_s_temp1_dn5) + ((var_delta_ns_dn5 * assign41830_e54813) + (var_delta_ns * (-var_sp_s_xi1_dn5))))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign41830_e54815) + (var_gf2 * ((var_sp_s_delta0_dn6 - var_sp_s_temp1_dn6) + ((var_delta_ns_dn6 * assign41830_e54813) + (var_delta_ns * (-var_sp_s_xi1_dn6))))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign41830_e54815) + (var_gf2 * ((var_sp_s_delta0_dn7 - var_sp_s_temp1_dn7) + ((var_delta_ns_dn7 * assign41830_e54813) + (var_delta_ns * (-var_sp_s_xi1_dn7))))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign41830_e54815) + (var_gf2 * ((var_sp_s_delta0_dn8 - var_sp_s_temp1_dn8) + ((var_delta_ns_dn8 * assign41830_e54813) + (var_delta_ns * (-var_sp_s_xi1_dn8))))))),)
    } else {
        (var_sp_s_pc, var_sp_s_pc_dn5, var_sp_s_pc_dn6, var_sp_s_pc_dn7, var_sp_s_pc_dn8,)
    }
};
        var_sp_s_pc = assign41830_e54819;
        var_sp_s_pc_dn5 = assign41830_e54819_d_n5;
        var_sp_s_pc_dn6 = assign41830_e54819_d_n6;
        var_sp_s_pc_dn7 = assign41830_e54819_d_n7;
        var_sp_s_pc_dn8 = assign41830_e54819_d_n8;

        let (assign41840_e54846, assign41840_e54846_d_n5, assign41840_e54846_d_n6, assign41840_e54846_d_n7, assign41840_e54846_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41840_e54826: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign41840_e54830: f64 = (var_sp_s_delta0 - var_sp_s_y0);
        let assign41840_e54832: f64 = (assign41840_e54830 - 1.0);
        let assign41840_e54834: f64 = (assign41840_e54832 + var_sp_s_temp1);
        let assign41840_e54838: f64 = (var_sp_s_y0 - 1.0);
        let assign41840_e54840: f64 = (assign41840_e54838 - var_sp_s_xi0);
        let assign41840_e54841: f64 = (var_delta_ns * assign41840_e54840);
        let assign41840_e54842: f64 = (assign41840_e54834 + assign41840_e54841);
        let assign41840_e54843: f64 = (var_gf2 * assign41840_e54842);
        let assign41840_e54844: f64 = (assign41840_e54826 - assign41840_e54843);
        (assign41840_e54844, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) - ((var_gf2_dn5 * assign41840_e54842) + (var_gf2 * (((var_sp_s_delta0_dn5 - var_sp_s_y0_dn5) + var_sp_s_temp1_dn5) + ((var_delta_ns_dn5 * assign41840_e54840) + (var_delta_ns * (var_sp_s_y0_dn5 - var_sp_s_xi0_dn5))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign41840_e54842) + (var_gf2 * (((var_sp_s_delta0_dn6 - var_sp_s_y0_dn6) + var_sp_s_temp1_dn6) + ((var_delta_ns_dn6 * assign41840_e54840) + (var_delta_ns * (var_sp_s_y0_dn6 - var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign41840_e54842) + (var_gf2 * (((var_sp_s_delta0_dn7 - var_sp_s_y0_dn7) + var_sp_s_temp1_dn7) + ((var_delta_ns_dn7 * assign41840_e54840) + (var_delta_ns * (var_sp_s_y0_dn7 - var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign41840_e54842) + (var_gf2 * (((var_sp_s_delta0_dn8 - var_sp_s_y0_dn8) + var_sp_s_temp1_dn8) + ((var_delta_ns_dn8 * assign41840_e54840) + (var_delta_ns * (var_sp_s_y0_dn8 - var_sp_s_xi0_dn8))))))),)
    } else {
        (var_sp_s_qc, var_sp_s_qc_dn5, var_sp_s_qc_dn6, var_sp_s_qc_dn7, var_sp_s_qc_dn8,)
    }
};
        var_sp_s_qc = assign41840_e54846;
        var_sp_s_qc_dn5 = assign41840_e54846_d_n5;
        var_sp_s_qc_dn6 = assign41840_e54846_d_n6;
        var_sp_s_qc_dn7 = assign41840_e54846_d_n7;
        var_sp_s_qc_dn8 = assign41840_e54846_d_n8;

        let (assign41850_e54863, assign41850_e54863_d_n5, assign41850_e54863_d_n6, assign41850_e54863_d_n7, assign41850_e54863_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41850_e54855: f64 = (var_sp_s_delta0 + var_sp_s_temp1);
        let assign41850_e54858: f64 = (var_delta_ns * var_sp_s_xi2);
        let assign41850_e54859: f64 = (assign41850_e54855 - assign41850_e54858);
        let assign41850_e54860: f64 = (var_gf2 * assign41850_e54859);
        let assign41850_e54861: f64 = (2.0 - assign41850_e54860);
        (assign41850_e54861, (-((var_gf2_dn5 * assign41850_e54859) + (var_gf2 * ((var_sp_s_delta0_dn5 + var_sp_s_temp1_dn5) - ((var_delta_ns_dn5 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn5)))))), (-((var_gf2_dn6 * assign41850_e54859) + (var_gf2 * ((var_sp_s_delta0_dn6 + var_sp_s_temp1_dn6) - ((var_delta_ns_dn6 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn6)))))), (-((var_gf2_dn7 * assign41850_e54859) + (var_gf2 * ((var_sp_s_delta0_dn7 + var_sp_s_temp1_dn7) - ((var_delta_ns_dn7 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn7)))))), (-((var_gf2_dn8 * assign41850_e54859) + (var_gf2 * ((var_sp_s_delta0_dn8 + var_sp_s_temp1_dn8) - ((var_delta_ns_dn8 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn8)))))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41850_e54863;
        var_sp_s_temp_dn5 = assign41850_e54863_d_n5;
        var_sp_s_temp_dn6 = assign41850_e54863_d_n6;
        var_sp_s_temp_dn7 = assign41850_e54863_d_n7;
        var_sp_s_temp_dn8 = assign41850_e54863_d_n8;

        let (assign41860_e54878, assign41860_e54878_d_n5, assign41860_e54878_d_n6, assign41860_e54878_d_n7, assign41860_e54878_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41860_e54870: f64 = (var_sp_s_pc * var_sp_s_pc);
        let assign41860_e54874: f64 = (var_sp_s_qc * var_sp_s_temp);
        let assign41860_e54875: f64 = (2.0 * assign41860_e54874);
        let assign41860_e54876: f64 = (assign41860_e54870 - assign41860_e54875);
        (assign41860_e54876, (((var_sp_s_pc_dn5 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn5)) - (2.0 * ((var_sp_s_qc_dn5 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn5)))), (((var_sp_s_pc_dn6 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn6)) - (2.0 * ((var_sp_s_qc_dn6 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn6)))), (((var_sp_s_pc_dn7 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn7)) - (2.0 * ((var_sp_s_qc_dn7 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn7)))), (((var_sp_s_pc_dn8 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn8)) - (2.0 * ((var_sp_s_qc_dn8 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn8)))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41860_e54878;
        var_sp_s_temp_dn5 = assign41860_e54878_d_n5;
        var_sp_s_temp_dn6 = assign41860_e54878_d_n6;
        var_sp_s_temp_dn7 = assign41860_e54878_d_n7;
        var_sp_s_temp_dn8 = assign41860_e54878_d_n8;

        let (assign41870_e54895, assign41870_e54895_d_n5, assign41870_e54895_d_n6, assign41870_e54895_d_n7, assign41870_e54895_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 != 0.0)) {
        let assign41870_e54884: f64 = (-var_sp_s_y0);
        let assign41870_e54889: f64 = (var_sp_s_temp).sqrt();
        let assign41870_e54890: f64 = (var_sp_s_pc + assign41870_e54889);
        let assign41870_e54891: f64 = (var_sp_s_qc / assign41870_e54890);
        let assign41870_e54892: f64 = (2.0 * assign41870_e54891);
        let assign41870_e54893: f64 = (assign41870_e54884 - assign41870_e54892);
        (assign41870_e54893, ((-var_sp_s_y0_dn5) - (2.0 * (((var_sp_s_qc_dn5 * assign41870_e54890) - (var_sp_s_qc * (var_sp_s_pc_dn5 + (var_sp_s_temp_dn5 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-var_sp_s_y0_dn6) - (2.0 * (((var_sp_s_qc_dn6 * assign41870_e54890) - (var_sp_s_qc * (var_sp_s_pc_dn6 + (var_sp_s_temp_dn6 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-var_sp_s_y0_dn7) - (2.0 * (((var_sp_s_qc_dn7 * assign41870_e54890) - (var_sp_s_qc * (var_sp_s_pc_dn7 + (var_sp_s_temp_dn7 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-var_sp_s_y0_dn8) - (2.0 * (((var_sp_s_qc_dn8 * assign41870_e54890) - (var_sp_s_qc * (var_sp_s_pc_dn8 + (var_sp_s_temp_dn8 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))),)
    } else {
        (var_x_s, var_x_s_dn5, var_x_s_dn6, var_x_s_dn7, var_x_s_dn8,)
    }
};
        var_x_s = assign41870_e54895;
        var_x_s_dn5 = assign41870_e54895_d_n5;
        var_x_s_dn6 = assign41870_e54895_d_n6;
        var_x_s_dn7 = assign41870_e54895_d_n7;
        var_x_s_dn8 = assign41870_e54895_d_n8;

        let (assign41880_e54909, assign41880_e54909_d_n5, assign41880_e54909_d_n6, assign41880_e54909_d_n7, assign41880_e54909_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41880_e54905: f64 = (var_gf * 0.7324648775608221);
        let assign41880_e54906: f64 = (1.25 + assign41880_e54905);
        let assign41880_e54907: f64 = (1.0 / assign41880_e54906);
        (assign41880_e54907, (-((var_gf_dn5 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((var_gf_dn6 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((var_gf_dn7 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((var_gf_dn8 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))),)
    } else {
        (var_sp_xg1, var_sp_xg1_dn5, var_sp_xg1_dn6, var_sp_xg1_dn7, var_sp_xg1_dn8,)
    }
};
        var_sp_xg1 = assign41880_e54909;
        var_sp_xg1_dn5 = assign41880_e54909_d_n5;
        var_sp_xg1_dn6 = assign41880_e54909_d_n6;
        var_sp_xg1_dn7 = assign41880_e54909_d_n7;
        var_sp_xg1_dn8 = assign41880_e54909_d_n8;

        let (assign41890_e54925, assign41890_e54925_d_n5, assign41890_e54925_d_n6, assign41890_e54925_d_n7, assign41890_e54925_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41890_e54917: f64 = (var_xi * 1.25);
        let assign41890_e54919: f64 = (assign41890_e54917 * var_sp_xg1);
        let assign41890_e54921: f64 = (assign41890_e54919 - 1.0);
        let assign41890_e54923: f64 = (assign41890_e54921 * var_sp_xg1);
        (assign41890_e54923, (((((var_xi_dn5 * 1.25) * var_sp_xg1) + (assign41890_e54917 * var_sp_xg1_dn5)) * var_sp_xg1) + (assign41890_e54921 * var_sp_xg1_dn5)), (((((var_xi_dn6 * 1.25) * var_sp_xg1) + (assign41890_e54917 * var_sp_xg1_dn6)) * var_sp_xg1) + (assign41890_e54921 * var_sp_xg1_dn6)), (((((var_xi_dn7 * 1.25) * var_sp_xg1) + (assign41890_e54917 * var_sp_xg1_dn7)) * var_sp_xg1) + (assign41890_e54921 * var_sp_xg1_dn7)), (((((var_xi_dn8 * 1.25) * var_sp_xg1) + (assign41890_e54917 * var_sp_xg1_dn8)) * var_sp_xg1) + (assign41890_e54921 * var_sp_xg1_dn8)),)
    } else {
        (var_sp_s_a_fac, var_sp_s_a_fac_dn5, var_sp_s_a_fac_dn6, var_sp_s_a_fac_dn7, var_sp_s_a_fac_dn8,)
    }
};
        var_sp_s_a_fac = assign41890_e54925;
        var_sp_s_a_fac_dn5 = assign41890_e54925_d_n5;
        var_sp_s_a_fac_dn6 = assign41890_e54925_d_n6;
        var_sp_s_a_fac_dn7 = assign41890_e54925_d_n7;
        var_sp_s_a_fac_dn8 = assign41890_e54925_d_n8;

        let (assign41900_e54941, assign41900_e54941_d_n5, assign41900_e54941_d_n6, assign41900_e54941_d_n7, assign41900_e54941_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41900_e54933: f64 = (var_xg * var_inv_xi);
        let assign41900_e54937: f64 = (var_sp_s_a_fac * var_xg);
        let assign41900_e54938: f64 = (1.0 + assign41900_e54937);
        let assign41900_e54939: f64 = (assign41900_e54933 * assign41900_e54938);
        (assign41900_e54939, ((((var_xg_dn5 * var_inv_xi) + (var_xg * var_inv_xi_dn5)) * assign41900_e54938) + (assign41900_e54933 * ((var_sp_s_a_fac_dn5 * var_xg) + (var_sp_s_a_fac * var_xg_dn5)))), ((((var_xg_dn6 * var_inv_xi) + (var_xg * var_inv_xi_dn6)) * assign41900_e54938) + (assign41900_e54933 * ((var_sp_s_a_fac_dn6 * var_xg) + (var_sp_s_a_fac * var_xg_dn6)))), ((((var_xg_dn7 * var_inv_xi) + (var_xg * var_inv_xi_dn7)) * assign41900_e54938) + (assign41900_e54933 * ((var_sp_s_a_fac_dn7 * var_xg) + (var_sp_s_a_fac * var_xg_dn7)))), ((((var_xg_dn8 * var_inv_xi) + (var_xg * var_inv_xi_dn8)) * assign41900_e54938) + (assign41900_e54933 * ((var_sp_s_a_fac_dn8 * var_xg) + (var_sp_s_a_fac * var_xg_dn8)))),)
    } else {
        (var_sp_s_xbar, var_sp_s_xbar_dn5, var_sp_s_xbar_dn6, var_sp_s_xbar_dn7, var_sp_s_xbar_dn8,)
    }
};
        var_sp_s_xbar = assign41900_e54941;
        var_sp_s_xbar_dn5 = assign41900_e54941_d_n5;
        var_sp_s_xbar_dn6 = assign41900_e54941_d_n6;
        var_sp_s_xbar_dn7 = assign41900_e54941_d_n7;
        var_sp_s_xbar_dn8 = assign41900_e54941_d_n8;

        *var_guard1183_slot = var_guard1183;
        *var_guard1184_slot = var_guard1184;
        *var_mutau_slot = var_mutau;
        *var_mutau_dn5_slot = var_mutau_dn5;
        *var_mutau_dn6_slot = var_mutau_dn6;
        *var_mutau_dn7_slot = var_mutau_dn7;
        *var_mutau_dn8_slot = var_mutau_dn8;
        *var_nu_slot = var_nu;
        *var_nu_dn5_slot = var_nu_dn5;
        *var_nu_dn6_slot = var_nu_dn6;
        *var_nu_dn7_slot = var_nu_dn7;
        *var_nu_dn8_slot = var_nu_dn8;
        *var_sp_s_a_slot = var_sp_s_a;
        *var_sp_s_a_dn5_slot = var_sp_s_a_dn5;
        *var_sp_s_a_dn6_slot = var_sp_s_a_dn6;
        *var_sp_s_a_dn7_slot = var_sp_s_a_dn7;
        *var_sp_s_a_dn8_slot = var_sp_s_a_dn8;
        *var_sp_s_a_fac_slot = var_sp_s_a_fac;
        *var_sp_s_a_fac_dn5_slot = var_sp_s_a_fac_dn5;
        *var_sp_s_a_fac_dn6_slot = var_sp_s_a_fac_dn6;
        *var_sp_s_a_fac_dn7_slot = var_sp_s_a_fac_dn7;
        *var_sp_s_a_fac_dn8_slot = var_sp_s_a_fac_dn8;
        *var_sp_s_c_slot = var_sp_s_c;
        *var_sp_s_c_dn5_slot = var_sp_s_c_dn5;
        *var_sp_s_c_dn6_slot = var_sp_s_c_dn6;
        *var_sp_s_c_dn7_slot = var_sp_s_c_dn7;
        *var_sp_s_c_dn8_slot = var_sp_s_c_dn8;
        *var_sp_s_delta0_slot = var_sp_s_delta0;
        *var_sp_s_delta0_dn5_slot = var_sp_s_delta0_dn5;
        *var_sp_s_delta0_dn6_slot = var_sp_s_delta0_dn6;
        *var_sp_s_delta0_dn7_slot = var_sp_s_delta0_dn7;
        *var_sp_s_delta0_dn8_slot = var_sp_s_delta0_dn8;
        *var_sp_s_delta1_slot = var_sp_s_delta1;
        *var_sp_s_delta1_dn5_slot = var_sp_s_delta1_dn5;
        *var_sp_s_delta1_dn6_slot = var_sp_s_delta1_dn6;
        *var_sp_s_delta1_dn7_slot = var_sp_s_delta1_dn7;
        *var_sp_s_delta1_dn8_slot = var_sp_s_delta1_dn8;
        *var_sp_s_eta_slot = var_sp_s_eta;
        *var_sp_s_eta_dn5_slot = var_sp_s_eta_dn5;
        *var_sp_s_eta_dn6_slot = var_sp_s_eta_dn6;
        *var_sp_s_eta_dn7_slot = var_sp_s_eta_dn7;
        *var_sp_s_eta_dn8_slot = var_sp_s_eta_dn8;
        *var_sp_s_pc_slot = var_sp_s_pc;
        *var_sp_s_pc_dn5_slot = var_sp_s_pc_dn5;
        *var_sp_s_pc_dn6_slot = var_sp_s_pc_dn6;
        *var_sp_s_pc_dn7_slot = var_sp_s_pc_dn7;
        *var_sp_s_pc_dn8_slot = var_sp_s_pc_dn8;
        *var_sp_s_qc_slot = var_sp_s_qc;
        *var_sp_s_qc_dn5_slot = var_sp_s_qc_dn5;
        *var_sp_s_qc_dn6_slot = var_sp_s_qc_dn6;
        *var_sp_s_qc_dn7_slot = var_sp_s_qc_dn7;
        *var_sp_s_qc_dn8_slot = var_sp_s_qc_dn8;
        *var_sp_s_tau_slot = var_sp_s_tau;
        *var_sp_s_tau_dn5_slot = var_sp_s_tau_dn5;
        *var_sp_s_tau_dn6_slot = var_sp_s_tau_dn6;
        *var_sp_s_tau_dn7_slot = var_sp_s_tau_dn7;
        *var_sp_s_tau_dn8_slot = var_sp_s_tau_dn8;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp1_slot = var_sp_s_temp1;
        *var_sp_s_temp1_dn5_slot = var_sp_s_temp1_dn5;
        *var_sp_s_temp1_dn6_slot = var_sp_s_temp1_dn6;
        *var_sp_s_temp1_dn7_slot = var_sp_s_temp1_dn7;
        *var_sp_s_temp1_dn8_slot = var_sp_s_temp1_dn8;
        *var_sp_s_temp_dn5_slot = var_sp_s_temp_dn5;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_xbar_slot = var_sp_s_xbar;
        *var_sp_s_xbar_dn5_slot = var_sp_s_xbar_dn5;
        *var_sp_s_xbar_dn6_slot = var_sp_s_xbar_dn6;
        *var_sp_s_xbar_dn7_slot = var_sp_s_xbar_dn7;
        *var_sp_s_xbar_dn8_slot = var_sp_s_xbar_dn8;
        *var_sp_s_xi0_slot = var_sp_s_xi0;
        *var_sp_s_xi0_dn5_slot = var_sp_s_xi0_dn5;
        *var_sp_s_xi0_dn6_slot = var_sp_s_xi0_dn6;
        *var_sp_s_xi0_dn7_slot = var_sp_s_xi0_dn7;
        *var_sp_s_xi0_dn8_slot = var_sp_s_xi0_dn8;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn5_slot = var_sp_s_xi1_dn5;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn5_slot = var_sp_s_xi2_dn5;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
        *var_sp_s_y0_slot = var_sp_s_y0;
        *var_sp_s_y0_dn5_slot = var_sp_s_y0_dn5;
        *var_sp_s_y0_dn6_slot = var_sp_s_y0_dn6;
        *var_sp_s_y0_dn7_slot = var_sp_s_y0_dn7;
        *var_sp_s_y0_dn8_slot = var_sp_s_y0_dn8;
        *var_sp_s_yg_slot = var_sp_s_yg;
        *var_sp_s_yg_dn5_slot = var_sp_s_yg_dn5;
        *var_sp_s_yg_dn6_slot = var_sp_s_yg_dn6;
        *var_sp_s_yg_dn7_slot = var_sp_s_yg_dn7;
        *var_sp_s_yg_dn8_slot = var_sp_s_yg_dn8;
        *var_sp_s_ysub_slot = var_sp_s_ysub;
        *var_sp_s_ysub_dn5_slot = var_sp_s_ysub_dn5;
        *var_sp_s_ysub_dn6_slot = var_sp_s_ysub_dn6;
        *var_sp_s_ysub_dn7_slot = var_sp_s_ysub_dn7;
        *var_sp_s_ysub_dn8_slot = var_sp_s_ysub_dn8;
        *var_sp_xg1_slot = var_sp_xg1;
        *var_sp_xg1_dn5_slot = var_sp_xg1_dn5;
        *var_sp_xg1_dn6_slot = var_sp_xg1_dn6;
        *var_sp_xg1_dn7_slot = var_sp_xg1_dn7;
        *var_sp_xg1_dn8_slot = var_sp_xg1_dn8;
        *var_x_s_slot = var_x_s;
        *var_x_s_dn5_slot = var_x_s_dn5;
        *var_x_s_dn6_slot = var_x_s_dn6;
        *var_x_s_dn7_slot = var_x_s_dn7;
        *var_x_s_dn8_slot = var_x_s_dn8;
    }

    pub(super) fn stamp_transient_block_90(
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1182: f64,
        var_guard1183: f64,
        var_sp_s_xbar: f64,
        var_sp_s_xbar_dn5: f64,
        var_sp_s_xbar_dn6: f64,
        var_sp_s_xbar_dn7: f64,
        var_sp_s_xbar_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xn_s: f64,
        var_xn_s_dn5: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_guard1185_slot: &mut f64,
        var_guard1186_slot: &mut f64,
        var_guard1187_slot: &mut f64,
        var_mutau_slot: &mut f64,
        var_mutau_dn5_slot: &mut f64,
        var_mutau_dn6_slot: &mut f64,
        var_mutau_dn7_slot: &mut f64,
        var_mutau_dn8_slot: &mut f64,
        var_nu_slot: &mut f64,
        var_nu_dn5_slot: &mut f64,
        var_nu_dn6_slot: &mut f64,
        var_nu_dn7_slot: &mut f64,
        var_nu_dn8_slot: &mut f64,
        var_sp_s_a_slot: &mut f64,
        var_sp_s_a_dn5_slot: &mut f64,
        var_sp_s_a_dn6_slot: &mut f64,
        var_sp_s_a_dn7_slot: &mut f64,
        var_sp_s_a_dn8_slot: &mut f64,
        var_sp_s_b_slot: &mut f64,
        var_sp_s_b_dn5_slot: &mut f64,
        var_sp_s_b_dn6_slot: &mut f64,
        var_sp_s_b_dn7_slot: &mut f64,
        var_sp_s_b_dn8_slot: &mut f64,
        var_sp_s_bx_slot: &mut f64,
        var_sp_s_bx_dn5_slot: &mut f64,
        var_sp_s_bx_dn6_slot: &mut f64,
        var_sp_s_bx_dn7_slot: &mut f64,
        var_sp_s_bx_dn8_slot: &mut f64,
        var_sp_s_c_slot: &mut f64,
        var_sp_s_c_dn5_slot: &mut f64,
        var_sp_s_c_dn6_slot: &mut f64,
        var_sp_s_c_dn7_slot: &mut f64,
        var_sp_s_c_dn8_slot: &mut f64,
        var_sp_s_delta0_slot: &mut f64,
        var_sp_s_delta0_dn5_slot: &mut f64,
        var_sp_s_delta0_dn6_slot: &mut f64,
        var_sp_s_delta0_dn7_slot: &mut f64,
        var_sp_s_delta0_dn8_slot: &mut f64,
        var_sp_s_delta1_slot: &mut f64,
        var_sp_s_delta1_dn5_slot: &mut f64,
        var_sp_s_delta1_dn6_slot: &mut f64,
        var_sp_s_delta1_dn7_slot: &mut f64,
        var_sp_s_delta1_dn8_slot: &mut f64,
        var_sp_s_eta_slot: &mut f64,
        var_sp_s_eta_dn5_slot: &mut f64,
        var_sp_s_eta_dn6_slot: &mut f64,
        var_sp_s_eta_dn7_slot: &mut f64,
        var_sp_s_eta_dn8_slot: &mut f64,
        var_sp_s_tau_slot: &mut f64,
        var_sp_s_tau_dn5_slot: &mut f64,
        var_sp_s_tau_dn6_slot: &mut f64,
        var_sp_s_tau_dn7_slot: &mut f64,
        var_sp_s_tau_dn8_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp1_slot: &mut f64,
        var_sp_s_temp1_dn5_slot: &mut f64,
        var_sp_s_temp1_dn6_slot: &mut f64,
        var_sp_s_temp1_dn7_slot: &mut f64,
        var_sp_s_temp1_dn8_slot: &mut f64,
        var_sp_s_temp2_slot: &mut f64,
        var_sp_s_temp2_dn5_slot: &mut f64,
        var_sp_s_temp2_dn6_slot: &mut f64,
        var_sp_s_temp2_dn7_slot: &mut f64,
        var_sp_s_temp2_dn8_slot: &mut f64,
        var_sp_s_temp_dn5_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_w_slot: &mut f64,
        var_sp_s_w_dn5_slot: &mut f64,
        var_sp_s_w_dn6_slot: &mut f64,
        var_sp_s_w_dn7_slot: &mut f64,
        var_sp_s_w_dn8_slot: &mut f64,
        var_sp_s_x0_slot: &mut f64,
        var_sp_s_x0_dn5_slot: &mut f64,
        var_sp_s_x0_dn6_slot: &mut f64,
        var_sp_s_x0_dn7_slot: &mut f64,
        var_sp_s_x0_dn8_slot: &mut f64,
        var_sp_s_x1_slot: &mut f64,
        var_sp_s_x1_dn5_slot: &mut f64,
        var_sp_s_x1_dn6_slot: &mut f64,
        var_sp_s_x1_dn7_slot: &mut f64,
        var_sp_s_x1_dn8_slot: &mut f64,
        var_sp_s_xi0_slot: &mut f64,
        var_sp_s_xi0_dn5_slot: &mut f64,
        var_sp_s_xi0_dn6_slot: &mut f64,
        var_sp_s_xi0_dn7_slot: &mut f64,
        var_sp_s_xi0_dn8_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn5_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn5_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
    ) {
        let mut var_guard1185: f64 = *var_guard1185_slot;
        let mut var_guard1186: f64 = *var_guard1186_slot;
        let mut var_guard1187: f64 = *var_guard1187_slot;
        let mut var_mutau: f64 = *var_mutau_slot;
        let mut var_mutau_dn5: f64 = *var_mutau_dn5_slot;
        let mut var_mutau_dn6: f64 = *var_mutau_dn6_slot;
        let mut var_mutau_dn7: f64 = *var_mutau_dn7_slot;
        let mut var_mutau_dn8: f64 = *var_mutau_dn8_slot;
        let mut var_nu: f64 = *var_nu_slot;
        let mut var_nu_dn5: f64 = *var_nu_dn5_slot;
        let mut var_nu_dn6: f64 = *var_nu_dn6_slot;
        let mut var_nu_dn7: f64 = *var_nu_dn7_slot;
        let mut var_nu_dn8: f64 = *var_nu_dn8_slot;
        let mut var_sp_s_a: f64 = *var_sp_s_a_slot;
        let mut var_sp_s_a_dn5: f64 = *var_sp_s_a_dn5_slot;
        let mut var_sp_s_a_dn6: f64 = *var_sp_s_a_dn6_slot;
        let mut var_sp_s_a_dn7: f64 = *var_sp_s_a_dn7_slot;
        let mut var_sp_s_a_dn8: f64 = *var_sp_s_a_dn8_slot;
        let mut var_sp_s_b: f64 = *var_sp_s_b_slot;
        let mut var_sp_s_b_dn5: f64 = *var_sp_s_b_dn5_slot;
        let mut var_sp_s_b_dn6: f64 = *var_sp_s_b_dn6_slot;
        let mut var_sp_s_b_dn7: f64 = *var_sp_s_b_dn7_slot;
        let mut var_sp_s_b_dn8: f64 = *var_sp_s_b_dn8_slot;
        let mut var_sp_s_bx: f64 = *var_sp_s_bx_slot;
        let mut var_sp_s_bx_dn5: f64 = *var_sp_s_bx_dn5_slot;
        let mut var_sp_s_bx_dn6: f64 = *var_sp_s_bx_dn6_slot;
        let mut var_sp_s_bx_dn7: f64 = *var_sp_s_bx_dn7_slot;
        let mut var_sp_s_bx_dn8: f64 = *var_sp_s_bx_dn8_slot;
        let mut var_sp_s_c: f64 = *var_sp_s_c_slot;
        let mut var_sp_s_c_dn5: f64 = *var_sp_s_c_dn5_slot;
        let mut var_sp_s_c_dn6: f64 = *var_sp_s_c_dn6_slot;
        let mut var_sp_s_c_dn7: f64 = *var_sp_s_c_dn7_slot;
        let mut var_sp_s_c_dn8: f64 = *var_sp_s_c_dn8_slot;
        let mut var_sp_s_delta0: f64 = *var_sp_s_delta0_slot;
        let mut var_sp_s_delta0_dn5: f64 = *var_sp_s_delta0_dn5_slot;
        let mut var_sp_s_delta0_dn6: f64 = *var_sp_s_delta0_dn6_slot;
        let mut var_sp_s_delta0_dn7: f64 = *var_sp_s_delta0_dn7_slot;
        let mut var_sp_s_delta0_dn8: f64 = *var_sp_s_delta0_dn8_slot;
        let mut var_sp_s_delta1: f64 = *var_sp_s_delta1_slot;
        let mut var_sp_s_delta1_dn5: f64 = *var_sp_s_delta1_dn5_slot;
        let mut var_sp_s_delta1_dn6: f64 = *var_sp_s_delta1_dn6_slot;
        let mut var_sp_s_delta1_dn7: f64 = *var_sp_s_delta1_dn7_slot;
        let mut var_sp_s_delta1_dn8: f64 = *var_sp_s_delta1_dn8_slot;
        let mut var_sp_s_eta: f64 = *var_sp_s_eta_slot;
        let mut var_sp_s_eta_dn5: f64 = *var_sp_s_eta_dn5_slot;
        let mut var_sp_s_eta_dn6: f64 = *var_sp_s_eta_dn6_slot;
        let mut var_sp_s_eta_dn7: f64 = *var_sp_s_eta_dn7_slot;
        let mut var_sp_s_eta_dn8: f64 = *var_sp_s_eta_dn8_slot;
        let mut var_sp_s_tau: f64 = *var_sp_s_tau_slot;
        let mut var_sp_s_tau_dn5: f64 = *var_sp_s_tau_dn5_slot;
        let mut var_sp_s_tau_dn6: f64 = *var_sp_s_tau_dn6_slot;
        let mut var_sp_s_tau_dn7: f64 = *var_sp_s_tau_dn7_slot;
        let mut var_sp_s_tau_dn8: f64 = *var_sp_s_tau_dn8_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp1: f64 = *var_sp_s_temp1_slot;
        let mut var_sp_s_temp1_dn5: f64 = *var_sp_s_temp1_dn5_slot;
        let mut var_sp_s_temp1_dn6: f64 = *var_sp_s_temp1_dn6_slot;
        let mut var_sp_s_temp1_dn7: f64 = *var_sp_s_temp1_dn7_slot;
        let mut var_sp_s_temp1_dn8: f64 = *var_sp_s_temp1_dn8_slot;
        let mut var_sp_s_temp2: f64 = *var_sp_s_temp2_slot;
        let mut var_sp_s_temp2_dn5: f64 = *var_sp_s_temp2_dn5_slot;
        let mut var_sp_s_temp2_dn6: f64 = *var_sp_s_temp2_dn6_slot;
        let mut var_sp_s_temp2_dn7: f64 = *var_sp_s_temp2_dn7_slot;
        let mut var_sp_s_temp2_dn8: f64 = *var_sp_s_temp2_dn8_slot;
        let mut var_sp_s_temp_dn5: f64 = *var_sp_s_temp_dn5_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_w: f64 = *var_sp_s_w_slot;
        let mut var_sp_s_w_dn5: f64 = *var_sp_s_w_dn5_slot;
        let mut var_sp_s_w_dn6: f64 = *var_sp_s_w_dn6_slot;
        let mut var_sp_s_w_dn7: f64 = *var_sp_s_w_dn7_slot;
        let mut var_sp_s_w_dn8: f64 = *var_sp_s_w_dn8_slot;
        let mut var_sp_s_x0: f64 = *var_sp_s_x0_slot;
        let mut var_sp_s_x0_dn5: f64 = *var_sp_s_x0_dn5_slot;
        let mut var_sp_s_x0_dn6: f64 = *var_sp_s_x0_dn6_slot;
        let mut var_sp_s_x0_dn7: f64 = *var_sp_s_x0_dn7_slot;
        let mut var_sp_s_x0_dn8: f64 = *var_sp_s_x0_dn8_slot;
        let mut var_sp_s_x1: f64 = *var_sp_s_x1_slot;
        let mut var_sp_s_x1_dn5: f64 = *var_sp_s_x1_dn5_slot;
        let mut var_sp_s_x1_dn6: f64 = *var_sp_s_x1_dn6_slot;
        let mut var_sp_s_x1_dn7: f64 = *var_sp_s_x1_dn7_slot;
        let mut var_sp_s_x1_dn8: f64 = *var_sp_s_x1_dn8_slot;
        let mut var_sp_s_xi0: f64 = *var_sp_s_xi0_slot;
        let mut var_sp_s_xi0_dn5: f64 = *var_sp_s_xi0_dn5_slot;
        let mut var_sp_s_xi0_dn6: f64 = *var_sp_s_xi0_dn6_slot;
        let mut var_sp_s_xi0_dn7: f64 = *var_sp_s_xi0_dn7_slot;
        let mut var_sp_s_xi0_dn8: f64 = *var_sp_s_xi0_dn8_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn5: f64 = *var_sp_s_xi1_dn5_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn5: f64 = *var_sp_s_xi2_dn5_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;

        let assign41910_e54943: f64 = (-var_sp_s_xbar);
        let assign41910_e54945: f64 = (-230.25850929940458);
        let assign41910_e54946: f64 = if assign41910_e54943 > assign41910_e54945 { 1.0 } else { 0.0 };
        var_guard1185 = assign41910_e54946;

        let (assign41920_e54958, assign41920_e54958_d_n5, assign41920_e54958_d_n6, assign41920_e54958_d_n7, assign41920_e54958_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1185 != 0.0)) {
        let assign41920_e54955: f64 = (-var_sp_s_xbar);
        let assign41920_e54956: f64 = (assign41920_e54955).exp();
        (assign41920_e54956, (assign41920_e54956 * (-var_sp_s_xbar_dn5)), (assign41920_e54956 * (-var_sp_s_xbar_dn6)), (assign41920_e54956 * (-var_sp_s_xbar_dn7)), (assign41920_e54956 * (-var_sp_s_xbar_dn8)),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41920_e54958;
        var_sp_s_temp_dn5 = assign41920_e54958_d_n5;
        var_sp_s_temp_dn6 = assign41920_e54958_d_n6;
        var_sp_s_temp_dn7 = assign41920_e54958_d_n7;
        var_sp_s_temp_dn8 = assign41920_e54958_d_n8;

        let (assign41930_e54997, assign41930_e54997_d_n5, assign41930_e54997_d_n6, assign41930_e54997_d_n7, assign41930_e54997_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1185 == 0.0)) {
        let assign41930_e54970: f64 = (-230.25850929940458);
        let assign41930_e54972: f64 = (-var_sp_s_xbar);
        let assign41930_e54973: f64 = (assign41930_e54970 - assign41930_e54972);
        let assign41930_e54977: f64 = (-230.25850929940458);
        let assign41930_e54979: f64 = (-var_sp_s_xbar);
        let assign41930_e54980: f64 = (assign41930_e54977 - assign41930_e54979);
        let assign41930_e54983: f64 = (-230.25850929940458);
        let assign41930_e54985: f64 = (-var_sp_s_xbar);
        let assign41930_e54986: f64 = (assign41930_e54983 - assign41930_e54985);
        let assign41930_e54988: f64 = (assign41930_e54986 * 0.3333333333333333);
        let assign41930_e54989: f64 = (1.0 + assign41930_e54988);
        let assign41930_e54990: f64 = (assign41930_e54980 * assign41930_e54989);
        let assign41930_e54991: f64 = (0.5 * assign41930_e54990);
        let assign41930_e54992: f64 = (1.0 + assign41930_e54991);
        let assign41930_e54993: f64 = (assign41930_e54973 * assign41930_e54992);
        let assign41930_e54994: f64 = (1.0 + assign41930_e54993);
        let assign41930_e54995: f64 = (1e-100 / assign41930_e54994);
        (assign41930_e54995, (-((1e-100 * (((-(-var_sp_s_xbar_dn5)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-var_sp_s_xbar_dn5)) * assign41930_e54989) + (assign41930_e54980 * ((-(-var_sp_s_xbar_dn5)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-var_sp_s_xbar_dn6)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-var_sp_s_xbar_dn6)) * assign41930_e54989) + (assign41930_e54980 * ((-(-var_sp_s_xbar_dn6)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-var_sp_s_xbar_dn7)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-var_sp_s_xbar_dn7)) * assign41930_e54989) + (assign41930_e54980 * ((-(-var_sp_s_xbar_dn7)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-var_sp_s_xbar_dn8)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-var_sp_s_xbar_dn8)) * assign41930_e54989) + (assign41930_e54980 * ((-(-var_sp_s_xbar_dn8)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41930_e54997;
        var_sp_s_temp_dn5 = assign41930_e54997_d_n5;
        var_sp_s_temp_dn6 = assign41930_e54997_d_n6;
        var_sp_s_temp_dn7 = assign41930_e54997_d_n7;
        var_sp_s_temp_dn8 = assign41930_e54997_d_n8;

        let (assign41940_e55007, assign41940_e55007_d_n5, assign41940_e55007_d_n6, assign41940_e55007_d_n7, assign41940_e55007_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41940_e55005: f64 = (1.0 - var_sp_s_temp);
        (assign41940_e55005, (-var_sp_s_temp_dn5), (-var_sp_s_temp_dn6), (-var_sp_s_temp_dn7), (-var_sp_s_temp_dn8),)
    } else {
        (var_sp_s_w, var_sp_s_w_dn5, var_sp_s_w_dn6, var_sp_s_w_dn7, var_sp_s_w_dn8,)
    }
};
        var_sp_s_w = assign41940_e55007;
        var_sp_s_w_dn5 = assign41940_e55007_d_n5;
        var_sp_s_w_dn6 = assign41940_e55007_d_n6;
        var_sp_s_w_dn7 = assign41940_e55007_d_n7;
        var_sp_s_w_dn8 = assign41940_e55007_d_n8;

        let (assign41950_e55030, assign41950_e55030_d_n5, assign41950_e55030_d_n6, assign41950_e55030_d_n7, assign41950_e55030_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41950_e55016: f64 = (var_gf2 * 0.5);
        let assign41950_e55017: f64 = (var_xg + assign41950_e55016);
        let assign41950_e55022: f64 = (var_gf2 * 0.25);
        let assign41950_e55023: f64 = (var_xg + assign41950_e55022);
        let assign41950_e55025: f64 = (assign41950_e55023 - var_sp_s_w);
        let assign41950_e55026: f64 = (assign41950_e55025).sqrt();
        let assign41950_e55027: f64 = (var_gf * assign41950_e55026);
        let assign41950_e55028: f64 = (assign41950_e55017 - assign41950_e55027);
        (assign41950_e55028, ((var_xg_dn5 + (var_gf2_dn5 * 0.5)) - ((var_gf_dn5 * assign41950_e55026) + (var_gf * (((var_xg_dn5 + (var_gf2_dn5 * 0.25)) - var_sp_s_w_dn5) / (2.0 * assign41950_e55026))))), ((var_xg_dn6 + (var_gf2_dn6 * 0.5)) - ((var_gf_dn6 * assign41950_e55026) + (var_gf * (((var_xg_dn6 + (var_gf2_dn6 * 0.25)) - var_sp_s_w_dn6) / (2.0 * assign41950_e55026))))), ((var_xg_dn7 + (var_gf2_dn7 * 0.5)) - ((var_gf_dn7 * assign41950_e55026) + (var_gf * (((var_xg_dn7 + (var_gf2_dn7 * 0.25)) - var_sp_s_w_dn7) / (2.0 * assign41950_e55026))))), ((var_xg_dn8 + (var_gf2_dn8 * 0.5)) - ((var_gf_dn8 * assign41950_e55026) + (var_gf * (((var_xg_dn8 + (var_gf2_dn8 * 0.25)) - var_sp_s_w_dn8) / (2.0 * assign41950_e55026))))),)
    } else {
        (var_sp_s_x1, var_sp_s_x1_dn5, var_sp_s_x1_dn6, var_sp_s_x1_dn7, var_sp_s_x1_dn8,)
    }
};
        var_sp_s_x1 = assign41950_e55030;
        var_sp_s_x1_dn5 = assign41950_e55030_d_n5;
        var_sp_s_x1_dn6 = assign41950_e55030_d_n6;
        var_sp_s_x1_dn7 = assign41950_e55030_d_n7;
        var_sp_s_x1_dn8 = assign41950_e55030_d_n8;

        let (assign41960_e55040, assign41960_e55040_d_n5, assign41960_e55040_d_n6, assign41960_e55040_d_n7, assign41960_e55040_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41960_e55038: f64 = (var_xn_s + 3.0);
        (assign41960_e55038, var_xn_s_dn5, var_xn_s_dn6, var_xn_s_dn7, var_xn_s_dn8,)
    } else {
        (var_sp_s_bx, var_sp_s_bx_dn5, var_sp_s_bx_dn6, var_sp_s_bx_dn7, var_sp_s_bx_dn8,)
    }
};
        var_sp_s_bx = assign41960_e55040;
        var_sp_s_bx_dn5 = assign41960_e55040_d_n5;
        var_sp_s_bx_dn6 = assign41960_e55040_d_n6;
        var_sp_s_bx_dn7 = assign41960_e55040_d_n7;
        var_sp_s_bx_dn8 = assign41960_e55040_d_n8;

        let (assign41970_e55074, assign41970_e55074_d_n5, assign41970_e55074_d_n6, assign41970_e55074_d_n7, assign41970_e55074_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41970_e55049: f64 = (var_sp_s_x1 + var_sp_s_bx);
        let assign41970_e55052: f64 = (var_sp_s_x1 - var_sp_s_bx);
        let assign41970_e55055: f64 = (var_sp_s_x1 - var_sp_s_bx);
        let assign41970_e55056: f64 = (assign41970_e55052 * assign41970_e55055);
        let assign41970_e55058: f64 = (assign41970_e55056 + 5.0);
        let assign41970_e55059: f64 = (assign41970_e55058).sqrt();
        let assign41970_e55060: f64 = (assign41970_e55049 - assign41970_e55059);
        let assign41970_e55061: f64 = (0.5 * assign41970_e55060);
        let assign41970_e55066: f64 = (var_sp_s_bx * var_sp_s_bx);
        let assign41970_e55068: f64 = (assign41970_e55066 + 5.0);
        let assign41970_e55069: f64 = (assign41970_e55068).sqrt();
        let assign41970_e55070: f64 = (var_sp_s_bx - assign41970_e55069);
        let assign41970_e55071: f64 = (0.5 * assign41970_e55070);
        let assign41970_e55072: f64 = (assign41970_e55061 - assign41970_e55071);
        (assign41970_e55072, ((0.5 * ((var_sp_s_x1_dn5 + var_sp_s_bx_dn5) - ((((var_sp_s_x1_dn5 - var_sp_s_bx_dn5) * assign41970_e55055) + (assign41970_e55052 * (var_sp_s_x1_dn5 - var_sp_s_bx_dn5))) / (2.0 * assign41970_e55059)))) - (0.5 * (var_sp_s_bx_dn5 - (((var_sp_s_bx_dn5 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn5)) / (2.0 * assign41970_e55069))))), ((0.5 * ((var_sp_s_x1_dn6 + var_sp_s_bx_dn6) - ((((var_sp_s_x1_dn6 - var_sp_s_bx_dn6) * assign41970_e55055) + (assign41970_e55052 * (var_sp_s_x1_dn6 - var_sp_s_bx_dn6))) / (2.0 * assign41970_e55059)))) - (0.5 * (var_sp_s_bx_dn6 - (((var_sp_s_bx_dn6 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn6)) / (2.0 * assign41970_e55069))))), ((0.5 * ((var_sp_s_x1_dn7 + var_sp_s_bx_dn7) - ((((var_sp_s_x1_dn7 - var_sp_s_bx_dn7) * assign41970_e55055) + (assign41970_e55052 * (var_sp_s_x1_dn7 - var_sp_s_bx_dn7))) / (2.0 * assign41970_e55059)))) - (0.5 * (var_sp_s_bx_dn7 - (((var_sp_s_bx_dn7 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn7)) / (2.0 * assign41970_e55069))))), ((0.5 * ((var_sp_s_x1_dn8 + var_sp_s_bx_dn8) - ((((var_sp_s_x1_dn8 - var_sp_s_bx_dn8) * assign41970_e55055) + (assign41970_e55052 * (var_sp_s_x1_dn8 - var_sp_s_bx_dn8))) / (2.0 * assign41970_e55059)))) - (0.5 * (var_sp_s_bx_dn8 - (((var_sp_s_bx_dn8 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn8)) / (2.0 * assign41970_e55069))))),)
    } else {
        (var_sp_s_eta, var_sp_s_eta_dn5, var_sp_s_eta_dn6, var_sp_s_eta_dn7, var_sp_s_eta_dn8,)
    }
};
        var_sp_s_eta = assign41970_e55074;
        var_sp_s_eta_dn5 = assign41970_e55074_d_n5;
        var_sp_s_eta_dn6 = assign41970_e55074_d_n6;
        var_sp_s_eta_dn7 = assign41970_e55074_d_n7;
        var_sp_s_eta_dn8 = assign41970_e55074_d_n8;

        let (assign41980_e55084, assign41980_e55084_d_n5, assign41980_e55084_d_n6, assign41980_e55084_d_n7, assign41980_e55084_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41980_e55082: f64 = (var_xg - var_sp_s_eta);
        (assign41980_e55082, (var_xg_dn5 - var_sp_s_eta_dn5), (var_xg_dn6 - var_sp_s_eta_dn6), (var_xg_dn7 - var_sp_s_eta_dn7), (var_xg_dn8 - var_sp_s_eta_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign41980_e55084;
        var_sp_s_temp_dn5 = assign41980_e55084_d_n5;
        var_sp_s_temp_dn6 = assign41980_e55084_d_n6;
        var_sp_s_temp_dn7 = assign41980_e55084_d_n7;
        var_sp_s_temp_dn8 = assign41980_e55084_d_n8;

        let (assign41990_e55094, assign41990_e55094_d_n5, assign41990_e55094_d_n6, assign41990_e55094_d_n7, assign41990_e55094_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign41990_e55091: f64 = (-var_sp_s_eta);
        let assign41990_e55092: f64 = (assign41990_e55091).exp();
        (assign41990_e55092, (assign41990_e55092 * (-var_sp_s_eta_dn5)), (assign41990_e55092 * (-var_sp_s_eta_dn6)), (assign41990_e55092 * (-var_sp_s_eta_dn7)), (assign41990_e55092 * (-var_sp_s_eta_dn8)),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn5, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8,)
    }
};
        var_sp_s_temp1 = assign41990_e55094;
        var_sp_s_temp1_dn5 = assign41990_e55094_d_n5;
        var_sp_s_temp1_dn6 = assign41990_e55094_d_n6;
        var_sp_s_temp1_dn7 = assign41990_e55094_d_n7;
        var_sp_s_temp1_dn8 = assign41990_e55094_d_n8;

        let (assign42000_e55108, assign42000_e55108_d_n5, assign42000_e55108_d_n6, assign42000_e55108_d_n7, assign42000_e55108_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42000_e55104: f64 = (var_sp_s_eta * var_sp_s_eta);
        let assign42000_e55105: f64 = (2.0 + assign42000_e55104);
        let assign42000_e55106: f64 = (1.0 / assign42000_e55105);
        (assign42000_e55106, (-(((var_sp_s_eta_dn5 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn5)) / (assign42000_e55105 * assign42000_e55105))), (-(((var_sp_s_eta_dn6 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn6)) / (assign42000_e55105 * assign42000_e55105))), (-(((var_sp_s_eta_dn7 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn7)) / (assign42000_e55105 * assign42000_e55105))), (-(((var_sp_s_eta_dn8 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn8)) / (assign42000_e55105 * assign42000_e55105))),)
    } else {
        (var_sp_s_temp2, var_sp_s_temp2_dn5, var_sp_s_temp2_dn6, var_sp_s_temp2_dn7, var_sp_s_temp2_dn8,)
    }
};
        var_sp_s_temp2 = assign42000_e55108;
        var_sp_s_temp2_dn5 = assign42000_e55108_d_n5;
        var_sp_s_temp2_dn6 = assign42000_e55108_d_n6;
        var_sp_s_temp2_dn7 = assign42000_e55108_d_n7;
        var_sp_s_temp2_dn8 = assign42000_e55108_d_n8;

        let (assign42010_e55120, assign42010_e55120_d_n5, assign42010_e55120_d_n6, assign42010_e55120_d_n7, assign42010_e55120_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42010_e55116: f64 = (var_sp_s_eta * var_sp_s_eta);
        let assign42010_e55118: f64 = (assign42010_e55116 * var_sp_s_temp2);
        (assign42010_e55118, ((((var_sp_s_eta_dn5 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn5)) * var_sp_s_temp2) + (assign42010_e55116 * var_sp_s_temp2_dn5)), ((((var_sp_s_eta_dn6 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn6)) * var_sp_s_temp2) + (assign42010_e55116 * var_sp_s_temp2_dn6)), ((((var_sp_s_eta_dn7 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn7)) * var_sp_s_temp2) + (assign42010_e55116 * var_sp_s_temp2_dn7)), ((((var_sp_s_eta_dn8 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn8)) * var_sp_s_temp2) + (assign42010_e55116 * var_sp_s_temp2_dn8)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn5, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8,)
    }
};
        var_sp_s_xi0 = assign42010_e55120;
        var_sp_s_xi0_dn5 = assign42010_e55120_d_n5;
        var_sp_s_xi0_dn6 = assign42010_e55120_d_n6;
        var_sp_s_xi0_dn7 = assign42010_e55120_d_n7;
        var_sp_s_xi0_dn8 = assign42010_e55120_d_n8;

        let (assign42020_e55134, assign42020_e55134_d_n5, assign42020_e55134_d_n6, assign42020_e55134_d_n7, assign42020_e55134_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42020_e55129: f64 = (var_sp_s_eta * var_sp_s_temp2);
        let assign42020_e55131: f64 = (assign42020_e55129 * var_sp_s_temp2);
        let assign42020_e55132: f64 = (4.0 * assign42020_e55131);
        (assign42020_e55132, (4.0 * ((((var_sp_s_eta_dn5 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn5)) * var_sp_s_temp2) + (assign42020_e55129 * var_sp_s_temp2_dn5))), (4.0 * ((((var_sp_s_eta_dn6 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn6)) * var_sp_s_temp2) + (assign42020_e55129 * var_sp_s_temp2_dn6))), (4.0 * ((((var_sp_s_eta_dn7 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn7)) * var_sp_s_temp2) + (assign42020_e55129 * var_sp_s_temp2_dn7))), (4.0 * ((((var_sp_s_eta_dn8 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn8)) * var_sp_s_temp2) + (assign42020_e55129 * var_sp_s_temp2_dn8))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn5, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8,)
    }
};
        var_sp_s_xi1 = assign42020_e55134;
        var_sp_s_xi1_dn5 = assign42020_e55134_d_n5;
        var_sp_s_xi1_dn6 = assign42020_e55134_d_n6;
        var_sp_s_xi1_dn7 = assign42020_e55134_d_n7;
        var_sp_s_xi1_dn8 = assign42020_e55134_d_n8;

        let (assign42030_e55152, assign42030_e55152_d_n5, assign42030_e55152_d_n6, assign42030_e55152_d_n7, assign42030_e55152_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42030_e55142: f64 = (8.0 * var_sp_s_temp2);
        let assign42030_e55145: f64 = (12.0 * var_sp_s_xi0);
        let assign42030_e55146: f64 = (assign42030_e55142 - assign42030_e55145);
        let assign42030_e55148: f64 = (assign42030_e55146 * var_sp_s_temp2);
        let assign42030_e55150: f64 = (assign42030_e55148 * var_sp_s_temp2);
        (assign42030_e55150, ((((((8.0 * var_sp_s_temp2_dn5) - (12.0 * var_sp_s_xi0_dn5)) * var_sp_s_temp2) + (assign42030_e55146 * var_sp_s_temp2_dn5)) * var_sp_s_temp2) + (assign42030_e55148 * var_sp_s_temp2_dn5)), ((((((8.0 * var_sp_s_temp2_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp2) + (assign42030_e55146 * var_sp_s_temp2_dn6)) * var_sp_s_temp2) + (assign42030_e55148 * var_sp_s_temp2_dn6)), ((((((8.0 * var_sp_s_temp2_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp2) + (assign42030_e55146 * var_sp_s_temp2_dn7)) * var_sp_s_temp2) + (assign42030_e55148 * var_sp_s_temp2_dn7)), ((((((8.0 * var_sp_s_temp2_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp2) + (assign42030_e55146 * var_sp_s_temp2_dn8)) * var_sp_s_temp2) + (assign42030_e55148 * var_sp_s_temp2_dn8)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn5, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8,)
    }
};
        var_sp_s_xi2 = assign42030_e55152;
        var_sp_s_xi2_dn5 = assign42030_e55152_d_n5;
        var_sp_s_xi2_dn6 = assign42030_e55152_d_n6;
        var_sp_s_xi2_dn7 = assign42030_e55152_d_n7;
        var_sp_s_xi2_dn8 = assign42030_e55152_d_n8;

        let (assign42040_e55201, assign42040_e55201_d_n5, assign42040_e55201_d_n6, assign42040_e55201_d_n7, assign42040_e55201_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42040_e55161: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign42040_e55165: f64 = (var_sp_s_temp1 + var_sp_s_eta);
        let assign42040_e55167: f64 = (assign42040_e55165 - 1.0);
        let assign42040_e55171: f64 = (var_sp_s_eta + 1.0);
        let assign42040_e55173: f64 = (assign42040_e55171 + var_sp_s_xi0);
        let assign42040_e55174: f64 = (var_delta_ns * assign42040_e55173);
        let assign42040_e55175: f64 = (assign42040_e55167 - assign42040_e55174);
        let assign42040_e55176: f64 = (var_gf2 * assign42040_e55175);
        let assign42040_e55177: f64 = (assign42040_e55161 - assign42040_e55176);
        let (assign42040_e55199, assign42040_e55199_d_n5, assign42040_e55199_d_n6, assign42040_e55199_d_n7, assign42040_e55199_d_n8,) = {
            if (1e-40 > assign42040_e55177) {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42040_e55182: f64 = (var_sp_s_temp * var_sp_s_temp);
                let assign42040_e55186: f64 = (var_sp_s_temp1 + var_sp_s_eta);
                let assign42040_e55188: f64 = (assign42040_e55186 - 1.0);
                let assign42040_e55192: f64 = (var_sp_s_eta + 1.0);
                let assign42040_e55194: f64 = (assign42040_e55192 + var_sp_s_xi0);
                let assign42040_e55195: f64 = (var_delta_ns * assign42040_e55194);
                let assign42040_e55196: f64 = (assign42040_e55188 - assign42040_e55195);
                let assign42040_e55197: f64 = (var_gf2 * assign42040_e55196);
                let assign42040_e55198: f64 = (assign42040_e55182 - assign42040_e55197);
                (assign42040_e55198, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) - ((var_gf2_dn5 * assign42040_e55196) + (var_gf2 * ((var_sp_s_temp1_dn5 + var_sp_s_eta_dn5) - ((var_delta_ns_dn5 * assign42040_e55194) + (var_delta_ns * (var_sp_s_eta_dn5 + var_sp_s_xi0_dn5))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign42040_e55196) + (var_gf2 * ((var_sp_s_temp1_dn6 + var_sp_s_eta_dn6) - ((var_delta_ns_dn6 * assign42040_e55194) + (var_delta_ns * (var_sp_s_eta_dn6 + var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign42040_e55196) + (var_gf2 * ((var_sp_s_temp1_dn7 + var_sp_s_eta_dn7) - ((var_delta_ns_dn7 * assign42040_e55194) + (var_delta_ns * (var_sp_s_eta_dn7 + var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign42040_e55196) + (var_gf2 * ((var_sp_s_temp1_dn8 + var_sp_s_eta_dn8) - ((var_delta_ns_dn8 * assign42040_e55194) + (var_delta_ns * (var_sp_s_eta_dn8 + var_sp_s_xi0_dn8))))))),)
            }
        };
        (assign42040_e55199, assign42040_e55199_d_n5, assign42040_e55199_d_n6, assign42040_e55199_d_n7, assign42040_e55199_d_n8,)
    } else {
        (var_sp_s_a, var_sp_s_a_dn5, var_sp_s_a_dn6, var_sp_s_a_dn7, var_sp_s_a_dn8,)
    }
};
        var_sp_s_a = assign42040_e55201;
        var_sp_s_a_dn5 = assign42040_e55201_d_n5;
        var_sp_s_a_dn6 = assign42040_e55201_d_n6;
        var_sp_s_a_dn7 = assign42040_e55201_d_n7;
        var_sp_s_a_dn8 = assign42040_e55201_d_n8;

        let (assign42050_e55219, assign42050_e55219_d_n5, assign42050_e55219_d_n6, assign42050_e55219_d_n7, assign42050_e55219_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42050_e55213: f64 = (var_delta_ns * var_sp_s_xi2);
        let assign42050_e55214: f64 = (var_sp_s_temp1 - assign42050_e55213);
        let assign42050_e55215: f64 = (var_gf2 * assign42050_e55214);
        let assign42050_e55216: f64 = (0.5 * assign42050_e55215);
        let assign42050_e55217: f64 = (1.0 - assign42050_e55216);
        (assign42050_e55217, (-(0.5 * ((var_gf2_dn5 * assign42050_e55214) + (var_gf2 * (var_sp_s_temp1_dn5 - ((var_delta_ns_dn5 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn5))))))), (-(0.5 * ((var_gf2_dn6 * assign42050_e55214) + (var_gf2 * (var_sp_s_temp1_dn6 - ((var_delta_ns_dn6 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn6))))))), (-(0.5 * ((var_gf2_dn7 * assign42050_e55214) + (var_gf2 * (var_sp_s_temp1_dn7 - ((var_delta_ns_dn7 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn7))))))), (-(0.5 * ((var_gf2_dn8 * assign42050_e55214) + (var_gf2 * (var_sp_s_temp1_dn8 - ((var_delta_ns_dn8 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn8))))))),)
    } else {
        (var_sp_s_b, var_sp_s_b_dn5, var_sp_s_b_dn6, var_sp_s_b_dn7, var_sp_s_b_dn8,)
    }
};
        var_sp_s_b = assign42050_e55219;
        var_sp_s_b_dn5 = assign42050_e55219_d_n5;
        var_sp_s_b_dn6 = assign42050_e55219_d_n6;
        var_sp_s_b_dn7 = assign42050_e55219_d_n7;
        var_sp_s_b_dn8 = assign42050_e55219_d_n8;

        let (assign42060_e55241, assign42060_e55241_d_n5, assign42060_e55241_d_n6, assign42060_e55241_d_n7, assign42060_e55241_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42060_e55227: f64 = (2.0 * var_sp_s_temp);
        let assign42060_e55231: f64 = (1.0 - var_sp_s_temp1);
        let assign42060_e55235: f64 = (1.0 + var_sp_s_xi1);
        let assign42060_e55236: f64 = (var_delta_ns * assign42060_e55235);
        let assign42060_e55237: f64 = (assign42060_e55231 - assign42060_e55236);
        let assign42060_e55238: f64 = (var_gf2 * assign42060_e55237);
        let assign42060_e55239: f64 = (assign42060_e55227 + assign42060_e55238);
        (assign42060_e55239, ((2.0 * var_sp_s_temp_dn5) + ((var_gf2_dn5 * assign42060_e55237) + (var_gf2 * ((-var_sp_s_temp1_dn5) - ((var_delta_ns_dn5 * assign42060_e55235) + (var_delta_ns * var_sp_s_xi1_dn5)))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign42060_e55237) + (var_gf2 * ((-var_sp_s_temp1_dn6) - ((var_delta_ns_dn6 * assign42060_e55235) + (var_delta_ns * var_sp_s_xi1_dn6)))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign42060_e55237) + (var_gf2 * ((-var_sp_s_temp1_dn7) - ((var_delta_ns_dn7 * assign42060_e55235) + (var_delta_ns * var_sp_s_xi1_dn7)))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign42060_e55237) + (var_gf2 * ((-var_sp_s_temp1_dn8) - ((var_delta_ns_dn8 * assign42060_e55235) + (var_delta_ns * var_sp_s_xi1_dn8)))))),)
    } else {
        (var_sp_s_c, var_sp_s_c_dn5, var_sp_s_c_dn6, var_sp_s_c_dn7, var_sp_s_c_dn8,)
    }
};
        var_sp_s_c = assign42060_e55241;
        var_sp_s_c_dn5 = assign42060_e55241_d_n5;
        var_sp_s_c_dn6 = assign42060_e55241_d_n6;
        var_sp_s_c_dn7 = assign42060_e55241_d_n7;
        var_sp_s_c_dn8 = assign42060_e55241_d_n8;

        let (assign42070_e55256, assign42070_e55256_d_n5, assign42070_e55256_d_n6, assign42070_e55256_d_n7, assign42070_e55256_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42070_e55249: f64 = (var_xn_s - var_sp_s_eta);
        let assign42070_e55252: f64 = (var_sp_s_a / var_gf2);
        let assign42070_e55253: f64 = (assign42070_e55252).ln();
        let assign42070_e55254: f64 = (assign42070_e55249 + assign42070_e55253);
        (assign42070_e55254, ((var_xn_s_dn5 - var_sp_s_eta_dn5) + ((((var_sp_s_a_dn5 * var_gf2) - (var_sp_s_a * var_gf2_dn5)) / (var_gf2 * var_gf2)) / assign42070_e55252)), ((var_xn_s_dn6 - var_sp_s_eta_dn6) + ((((var_sp_s_a_dn6 * var_gf2) - (var_sp_s_a * var_gf2_dn6)) / (var_gf2 * var_gf2)) / assign42070_e55252)), ((var_xn_s_dn7 - var_sp_s_eta_dn7) + ((((var_sp_s_a_dn7 * var_gf2) - (var_sp_s_a * var_gf2_dn7)) / (var_gf2 * var_gf2)) / assign42070_e55252)), ((var_xn_s_dn8 - var_sp_s_eta_dn8) + ((((var_sp_s_a_dn8 * var_gf2) - (var_sp_s_a * var_gf2_dn8)) / (var_gf2 * var_gf2)) / assign42070_e55252)),)
    } else {
        (var_sp_s_tau, var_sp_s_tau_dn5, var_sp_s_tau_dn6, var_sp_s_tau_dn7, var_sp_s_tau_dn8,)
    }
};
        var_sp_s_tau = assign42070_e55256;
        var_sp_s_tau_dn5 = assign42070_e55256_d_n5;
        var_sp_s_tau_dn6 = assign42070_e55256_d_n6;
        var_sp_s_tau_dn7 = assign42070_e55256_d_n7;
        var_sp_s_tau_dn8 = assign42070_e55256_d_n8;

        let (assign42080_e55266, assign42080_e55266_d_n5, assign42080_e55266_d_n6, assign42080_e55266_d_n7, assign42080_e55266_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42080_e55264: f64 = (var_sp_s_a + var_sp_s_c);
        (assign42080_e55264, (var_sp_s_a_dn5 + var_sp_s_c_dn5), (var_sp_s_a_dn6 + var_sp_s_c_dn6), (var_sp_s_a_dn7 + var_sp_s_c_dn7), (var_sp_s_a_dn8 + var_sp_s_c_dn8),)
    } else {
        (var_nu, var_nu_dn5, var_nu_dn6, var_nu_dn7, var_nu_dn8,)
    }
};
        var_nu = assign42080_e55266;
        var_nu_dn5 = assign42080_e55266_d_n5;
        var_nu_dn6 = assign42080_e55266_d_n6;
        var_nu_dn7 = assign42080_e55266_d_n7;
        var_nu_dn8 = assign42080_e55266_d_n8;

        let (assign42090_e55288, assign42090_e55288_d_n5, assign42090_e55288_d_n6, assign42090_e55288_d_n7, assign42090_e55288_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42090_e55274: f64 = (var_nu * var_nu);
        let assign42090_e55279: f64 = (var_sp_s_c * var_sp_s_c);
        let assign42090_e55280: f64 = (0.5 * assign42090_e55279);
        let assign42090_e55283: f64 = (var_sp_s_a * var_sp_s_b);
        let assign42090_e55284: f64 = (assign42090_e55280 - assign42090_e55283);
        let assign42090_e55285: f64 = (var_sp_s_tau * assign42090_e55284);
        let assign42090_e55286: f64 = (assign42090_e55274 + assign42090_e55285);
        (assign42090_e55286, (((var_nu_dn5 * var_nu) + (var_nu * var_nu_dn5)) + ((var_sp_s_tau_dn5 * assign42090_e55284) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5))) - ((var_sp_s_a_dn5 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn5)))))), (((var_nu_dn6 * var_nu) + (var_nu * var_nu_dn6)) + ((var_sp_s_tau_dn6 * assign42090_e55284) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6))) - ((var_sp_s_a_dn6 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn6)))))), (((var_nu_dn7 * var_nu) + (var_nu * var_nu_dn7)) + ((var_sp_s_tau_dn7 * assign42090_e55284) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7))) - ((var_sp_s_a_dn7 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn7)))))), (((var_nu_dn8 * var_nu) + (var_nu * var_nu_dn8)) + ((var_sp_s_tau_dn8 * assign42090_e55284) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8))) - ((var_sp_s_a_dn8 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn8)))))),)
    } else {
        (var_mutau, var_mutau_dn5, var_mutau_dn6, var_mutau_dn7, var_mutau_dn8,)
    }
};
        var_mutau = assign42090_e55288;
        var_mutau_dn5 = assign42090_e55288_d_n5;
        var_mutau_dn6 = assign42090_e55288_d_n6;
        var_mutau_dn7 = assign42090_e55288_d_n7;
        var_mutau_dn8 = assign42090_e55288_d_n8;

        let (assign42100_e55324, assign42100_e55324_d_n5, assign42100_e55324_d_n6, assign42100_e55324_d_n7, assign42100_e55324_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42100_e55297: f64 = (var_sp_s_a * var_nu);
        let assign42100_e55299: f64 = (assign42100_e55297 * var_sp_s_tau);
        let assign42100_e55303: f64 = (var_nu / var_mutau);
        let assign42100_e55305: f64 = (assign42100_e55303 * var_sp_s_tau);
        let assign42100_e55307: f64 = (assign42100_e55305 * var_sp_s_tau);
        let assign42100_e55309: f64 = (assign42100_e55307 * var_sp_s_c);
        let assign42100_e55312: f64 = (var_sp_s_c * var_sp_s_c);
        let assign42100_e55314: f64 = (assign42100_e55312 * 0.3333333333333333);
        let assign42100_e55317: f64 = (var_sp_s_a * var_sp_s_b);
        let assign42100_e55318: f64 = (assign42100_e55314 - assign42100_e55317);
        let assign42100_e55319: f64 = (assign42100_e55309 * assign42100_e55318);
        let assign42100_e55320: f64 = (var_mutau + assign42100_e55319);
        let assign42100_e55321: f64 = (assign42100_e55299 / assign42100_e55320);
        let assign42100_e55322: f64 = (var_sp_s_eta + assign42100_e55321);
        (assign42100_e55322, (var_sp_s_eta_dn5 + (((((((var_sp_s_a_dn5 * var_nu) + (var_sp_s_a * var_nu_dn5)) * var_sp_s_tau) + (assign42100_e55297 * var_sp_s_tau_dn5)) * assign42100_e55320) - (assign42100_e55299 * (var_mutau_dn5 + (((((((((((var_nu_dn5 * var_mutau) - (var_nu * var_mutau_dn5)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42100_e55303 * var_sp_s_tau_dn5)) * var_sp_s_tau) + (assign42100_e55305 * var_sp_s_tau_dn5)) * var_sp_s_c) + (assign42100_e55307 * var_sp_s_c_dn5)) * assign42100_e55318) + (assign42100_e55309 * ((((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5)) * 0.3333333333333333) - ((var_sp_s_a_dn5 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn5)))))))) / (assign42100_e55320 * assign42100_e55320))), (var_sp_s_eta_dn6 + (((((((var_sp_s_a_dn6 * var_nu) + (var_sp_s_a * var_nu_dn6)) * var_sp_s_tau) + (assign42100_e55297 * var_sp_s_tau_dn6)) * assign42100_e55320) - (assign42100_e55299 * (var_mutau_dn6 + (((((((((((var_nu_dn6 * var_mutau) - (var_nu * var_mutau_dn6)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42100_e55303 * var_sp_s_tau_dn6)) * var_sp_s_tau) + (assign42100_e55305 * var_sp_s_tau_dn6)) * var_sp_s_c) + (assign42100_e55307 * var_sp_s_c_dn6)) * assign42100_e55318) + (assign42100_e55309 * ((((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6)) * 0.3333333333333333) - ((var_sp_s_a_dn6 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn6)))))))) / (assign42100_e55320 * assign42100_e55320))), (var_sp_s_eta_dn7 + (((((((var_sp_s_a_dn7 * var_nu) + (var_sp_s_a * var_nu_dn7)) * var_sp_s_tau) + (assign42100_e55297 * var_sp_s_tau_dn7)) * assign42100_e55320) - (assign42100_e55299 * (var_mutau_dn7 + (((((((((((var_nu_dn7 * var_mutau) - (var_nu * var_mutau_dn7)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42100_e55303 * var_sp_s_tau_dn7)) * var_sp_s_tau) + (assign42100_e55305 * var_sp_s_tau_dn7)) * var_sp_s_c) + (assign42100_e55307 * var_sp_s_c_dn7)) * assign42100_e55318) + (assign42100_e55309 * ((((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7)) * 0.3333333333333333) - ((var_sp_s_a_dn7 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn7)))))))) / (assign42100_e55320 * assign42100_e55320))), (var_sp_s_eta_dn8 + (((((((var_sp_s_a_dn8 * var_nu) + (var_sp_s_a * var_nu_dn8)) * var_sp_s_tau) + (assign42100_e55297 * var_sp_s_tau_dn8)) * assign42100_e55320) - (assign42100_e55299 * (var_mutau_dn8 + (((((((((((var_nu_dn8 * var_mutau) - (var_nu * var_mutau_dn8)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42100_e55303 * var_sp_s_tau_dn8)) * var_sp_s_tau) + (assign42100_e55305 * var_sp_s_tau_dn8)) * var_sp_s_c) + (assign42100_e55307 * var_sp_s_c_dn8)) * assign42100_e55318) + (assign42100_e55309 * ((((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8)) * 0.3333333333333333) - ((var_sp_s_a_dn8 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn8)))))))) / (assign42100_e55320 * assign42100_e55320))),)
    } else {
        (var_sp_s_x0, var_sp_s_x0_dn5, var_sp_s_x0_dn6, var_sp_s_x0_dn7, var_sp_s_x0_dn8,)
    }
};
        var_sp_s_x0 = assign42100_e55324;
        var_sp_s_x0_dn5 = assign42100_e55324_d_n5;
        var_sp_s_x0_dn6 = assign42100_e55324_d_n6;
        var_sp_s_x0_dn7 = assign42100_e55324_d_n7;
        var_sp_s_x0_dn8 = assign42100_e55324_d_n8;

        let assign42110_e55327: f64 = if var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1186 = assign42110_e55327;

        let (assign42120_e55338, assign42120_e55338_d_n5, assign42120_e55338_d_n6, assign42120_e55338_d_n7, assign42120_e55338_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 != 0.0)) {
        let assign42120_e55336: f64 = (var_sp_s_x0).exp();
        (assign42120_e55336, (assign42120_e55336 * var_sp_s_x0_dn5), (assign42120_e55336 * var_sp_s_x0_dn6), (assign42120_e55336 * var_sp_s_x0_dn7), (assign42120_e55336 * var_sp_s_x0_dn8),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign42120_e55338;
        var_sp_s_delta0_dn5 = assign42120_e55338_d_n5;
        var_sp_s_delta0_dn6 = assign42120_e55338_d_n6;
        var_sp_s_delta0_dn7 = assign42120_e55338_d_n7;
        var_sp_s_delta0_dn8 = assign42120_e55338_d_n8;

        let (assign42130_e55350, assign42130_e55350_d_n5, assign42130_e55350_d_n6, assign42130_e55350_d_n7, assign42130_e55350_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 != 0.0)) {
        let assign42130_e55348: f64 = (1.0 / var_sp_s_delta0);
        (assign42130_e55348, (-(var_sp_s_delta0_dn5 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn6 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn7 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn8 / (var_sp_s_delta0 * var_sp_s_delta0))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign42130_e55350;
        var_sp_s_delta1_dn5 = assign42130_e55350_d_n5;
        var_sp_s_delta1_dn6 = assign42130_e55350_d_n6;
        var_sp_s_delta1_dn7 = assign42130_e55350_d_n7;
        var_sp_s_delta1_dn8 = assign42130_e55350_d_n8;

        let (assign42140_e55362, assign42140_e55362_d_n5, assign42140_e55362_d_n6, assign42140_e55362_d_n7, assign42140_e55362_d_n8,) = {
    if (((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 != 0.0)) {
        let assign42140_e55360: f64 = (var_delta_ns * var_sp_s_delta0);
        (assign42140_e55360, ((var_delta_ns_dn5 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn5)), ((var_delta_ns_dn6 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn6)), ((var_delta_ns_dn7 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn7)), ((var_delta_ns_dn8 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn8)),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign42140_e55362;
        var_sp_s_delta0_dn5 = assign42140_e55362_d_n5;
        var_sp_s_delta0_dn6 = assign42140_e55362_d_n6;
        var_sp_s_delta0_dn7 = assign42140_e55362_d_n7;
        var_sp_s_delta0_dn8 = assign42140_e55362_d_n8;

        let assign42150_e55366: f64 = (var_xn_s - 230.25850929940458);
        let assign42150_e55367: f64 = if var_sp_s_x0 > assign42150_e55366 { 1.0 } else { 0.0 };
        var_guard1187 = assign42150_e55367;

        let (assign42160_e55383, assign42160_e55383_d_n5, assign42160_e55383_d_n6, assign42160_e55383_d_n7, assign42160_e55383_d_n8,) = {
    if ((((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 == 0.0)) && (var_guard1187 != 0.0)) {
        let assign42160_e55380: f64 = (var_sp_s_x0 - var_xn_s);
        let assign42160_e55381: f64 = (assign42160_e55380).exp();
        (assign42160_e55381, (assign42160_e55381 * (var_sp_s_x0_dn5 - var_xn_s_dn5)), (assign42160_e55381 * (var_sp_s_x0_dn6 - var_xn_s_dn6)), (assign42160_e55381 * (var_sp_s_x0_dn7 - var_xn_s_dn7)), (assign42160_e55381 * (var_sp_s_x0_dn8 - var_xn_s_dn8)),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign42160_e55383;
        var_sp_s_delta0_dn5 = assign42160_e55383_d_n5;
        var_sp_s_delta0_dn6 = assign42160_e55383_d_n6;
        var_sp_s_delta0_dn7 = assign42160_e55383_d_n7;
        var_sp_s_delta0_dn8 = assign42160_e55383_d_n8;

        let (assign42170_e55398, assign42170_e55398_d_n5, assign42170_e55398_d_n6, assign42170_e55398_d_n7, assign42170_e55398_d_n8,) = {
    if ((((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 == 0.0)) && (var_guard1187 != 0.0)) {
        let assign42170_e55396: f64 = (var_delta_ns / var_sp_s_delta0);
        (assign42170_e55396, (((var_delta_ns_dn5 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn5)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_ns_dn6 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn6)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_ns_dn7 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn7)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_ns_dn8 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn8)) / (var_sp_s_delta0 * var_sp_s_delta0)),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign42170_e55398;
        var_sp_s_delta1_dn5 = assign42170_e55398_d_n5;
        var_sp_s_delta1_dn6 = assign42170_e55398_d_n6;
        var_sp_s_delta1_dn7 = assign42170_e55398_d_n7;
        var_sp_s_delta1_dn8 = assign42170_e55398_d_n8;

        let (assign42180_e55440, assign42180_e55440_d_n5, assign42180_e55440_d_n6, assign42180_e55440_d_n7, assign42180_e55440_d_n8,) = {
    if ((((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 == 0.0)) && (var_guard1187 == 0.0)) {
        let assign42180_e55414: f64 = (var_xn_s - var_sp_s_x0);
        let assign42180_e55416: f64 = (assign42180_e55414 - 230.25850929940458);
        let assign42180_e55421: f64 = (var_xn_s - var_sp_s_x0);
        let assign42180_e55423: f64 = (assign42180_e55421 - 230.25850929940458);
        let assign42180_e55427: f64 = (var_xn_s - var_sp_s_x0);
        let assign42180_e55429: f64 = (assign42180_e55427 - 230.25850929940458);
        let assign42180_e55431: f64 = (assign42180_e55429 * 0.3333333333333333);
        let assign42180_e55432: f64 = (1.0 + assign42180_e55431);
        let assign42180_e55433: f64 = (assign42180_e55423 * assign42180_e55432);
        let assign42180_e55434: f64 = (0.5 * assign42180_e55433);
        let assign42180_e55435: f64 = (1.0 + assign42180_e55434);
        let assign42180_e55436: f64 = (assign42180_e55416 * assign42180_e55435);
        let assign42180_e55437: f64 = (1.0 + assign42180_e55436);
        let assign42180_e55438: f64 = (1e-100 / assign42180_e55437);
        (assign42180_e55438, (-((1e-100 * (((var_xn_s_dn5 - var_sp_s_x0_dn5) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((var_xn_s_dn5 - var_sp_s_x0_dn5) * assign42180_e55432) + (assign42180_e55423 * ((var_xn_s_dn5 - var_sp_s_x0_dn5) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((var_xn_s_dn6 - var_sp_s_x0_dn6) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((var_xn_s_dn6 - var_sp_s_x0_dn6) * assign42180_e55432) + (assign42180_e55423 * ((var_xn_s_dn6 - var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((var_xn_s_dn7 - var_sp_s_x0_dn7) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((var_xn_s_dn7 - var_sp_s_x0_dn7) * assign42180_e55432) + (assign42180_e55423 * ((var_xn_s_dn7 - var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((var_xn_s_dn8 - var_sp_s_x0_dn8) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((var_xn_s_dn8 - var_sp_s_x0_dn8) * assign42180_e55432) + (assign42180_e55423 * ((var_xn_s_dn8 - var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign42180_e55440;
        var_sp_s_delta0_dn5 = assign42180_e55440_d_n5;
        var_sp_s_delta0_dn6 = assign42180_e55440_d_n6;
        var_sp_s_delta0_dn7 = assign42180_e55440_d_n7;
        var_sp_s_delta0_dn8 = assign42180_e55440_d_n8;

        let (assign42190_e55476, assign42190_e55476_d_n5, assign42190_e55476_d_n6, assign42190_e55476_d_n7, assign42190_e55476_d_n8,) = {
    if ((((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) && (var_guard1186 == 0.0)) && (var_guard1187 == 0.0)) {
        let assign42190_e55456: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign42190_e55461: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign42190_e55465: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign42190_e55467: f64 = (assign42190_e55465 * 0.3333333333333333);
        let assign42190_e55468: f64 = (1.0 + assign42190_e55467);
        let assign42190_e55469: f64 = (assign42190_e55461 * assign42190_e55468);
        let assign42190_e55470: f64 = (0.5 * assign42190_e55469);
        let assign42190_e55471: f64 = (1.0 + assign42190_e55470);
        let assign42190_e55472: f64 = (assign42190_e55456 * assign42190_e55471);
        let assign42190_e55473: f64 = (1.0 + assign42190_e55472);
        let assign42190_e55474: f64 = (1e-100 / assign42190_e55473);
        (assign42190_e55474, (-((1e-100 * ((var_sp_s_x0_dn5 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((var_sp_s_x0_dn5 * assign42190_e55468) + (assign42190_e55461 * (var_sp_s_x0_dn5 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((var_sp_s_x0_dn6 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((var_sp_s_x0_dn6 * assign42190_e55468) + (assign42190_e55461 * (var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((var_sp_s_x0_dn7 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((var_sp_s_x0_dn7 * assign42190_e55468) + (assign42190_e55461 * (var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((var_sp_s_x0_dn8 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((var_sp_s_x0_dn8 * assign42190_e55468) + (assign42190_e55461 * (var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign42190_e55476;
        var_sp_s_delta1_dn5 = assign42190_e55476_d_n5;
        var_sp_s_delta1_dn6 = assign42190_e55476_d_n6;
        var_sp_s_delta1_dn7 = assign42190_e55476_d_n7;
        var_sp_s_delta1_dn8 = assign42190_e55476_d_n8;

        *var_guard1185_slot = var_guard1185;
        *var_guard1186_slot = var_guard1186;
        *var_guard1187_slot = var_guard1187;
        *var_mutau_slot = var_mutau;
        *var_mutau_dn5_slot = var_mutau_dn5;
        *var_mutau_dn6_slot = var_mutau_dn6;
        *var_mutau_dn7_slot = var_mutau_dn7;
        *var_mutau_dn8_slot = var_mutau_dn8;
        *var_nu_slot = var_nu;
        *var_nu_dn5_slot = var_nu_dn5;
        *var_nu_dn6_slot = var_nu_dn6;
        *var_nu_dn7_slot = var_nu_dn7;
        *var_nu_dn8_slot = var_nu_dn8;
        *var_sp_s_a_slot = var_sp_s_a;
        *var_sp_s_a_dn5_slot = var_sp_s_a_dn5;
        *var_sp_s_a_dn6_slot = var_sp_s_a_dn6;
        *var_sp_s_a_dn7_slot = var_sp_s_a_dn7;
        *var_sp_s_a_dn8_slot = var_sp_s_a_dn8;
        *var_sp_s_b_slot = var_sp_s_b;
        *var_sp_s_b_dn5_slot = var_sp_s_b_dn5;
        *var_sp_s_b_dn6_slot = var_sp_s_b_dn6;
        *var_sp_s_b_dn7_slot = var_sp_s_b_dn7;
        *var_sp_s_b_dn8_slot = var_sp_s_b_dn8;
        *var_sp_s_bx_slot = var_sp_s_bx;
        *var_sp_s_bx_dn5_slot = var_sp_s_bx_dn5;
        *var_sp_s_bx_dn6_slot = var_sp_s_bx_dn6;
        *var_sp_s_bx_dn7_slot = var_sp_s_bx_dn7;
        *var_sp_s_bx_dn8_slot = var_sp_s_bx_dn8;
        *var_sp_s_c_slot = var_sp_s_c;
        *var_sp_s_c_dn5_slot = var_sp_s_c_dn5;
        *var_sp_s_c_dn6_slot = var_sp_s_c_dn6;
        *var_sp_s_c_dn7_slot = var_sp_s_c_dn7;
        *var_sp_s_c_dn8_slot = var_sp_s_c_dn8;
        *var_sp_s_delta0_slot = var_sp_s_delta0;
        *var_sp_s_delta0_dn5_slot = var_sp_s_delta0_dn5;
        *var_sp_s_delta0_dn6_slot = var_sp_s_delta0_dn6;
        *var_sp_s_delta0_dn7_slot = var_sp_s_delta0_dn7;
        *var_sp_s_delta0_dn8_slot = var_sp_s_delta0_dn8;
        *var_sp_s_delta1_slot = var_sp_s_delta1;
        *var_sp_s_delta1_dn5_slot = var_sp_s_delta1_dn5;
        *var_sp_s_delta1_dn6_slot = var_sp_s_delta1_dn6;
        *var_sp_s_delta1_dn7_slot = var_sp_s_delta1_dn7;
        *var_sp_s_delta1_dn8_slot = var_sp_s_delta1_dn8;
        *var_sp_s_eta_slot = var_sp_s_eta;
        *var_sp_s_eta_dn5_slot = var_sp_s_eta_dn5;
        *var_sp_s_eta_dn6_slot = var_sp_s_eta_dn6;
        *var_sp_s_eta_dn7_slot = var_sp_s_eta_dn7;
        *var_sp_s_eta_dn8_slot = var_sp_s_eta_dn8;
        *var_sp_s_tau_slot = var_sp_s_tau;
        *var_sp_s_tau_dn5_slot = var_sp_s_tau_dn5;
        *var_sp_s_tau_dn6_slot = var_sp_s_tau_dn6;
        *var_sp_s_tau_dn7_slot = var_sp_s_tau_dn7;
        *var_sp_s_tau_dn8_slot = var_sp_s_tau_dn8;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp1_slot = var_sp_s_temp1;
        *var_sp_s_temp1_dn5_slot = var_sp_s_temp1_dn5;
        *var_sp_s_temp1_dn6_slot = var_sp_s_temp1_dn6;
        *var_sp_s_temp1_dn7_slot = var_sp_s_temp1_dn7;
        *var_sp_s_temp1_dn8_slot = var_sp_s_temp1_dn8;
        *var_sp_s_temp2_slot = var_sp_s_temp2;
        *var_sp_s_temp2_dn5_slot = var_sp_s_temp2_dn5;
        *var_sp_s_temp2_dn6_slot = var_sp_s_temp2_dn6;
        *var_sp_s_temp2_dn7_slot = var_sp_s_temp2_dn7;
        *var_sp_s_temp2_dn8_slot = var_sp_s_temp2_dn8;
        *var_sp_s_temp_dn5_slot = var_sp_s_temp_dn5;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_w_slot = var_sp_s_w;
        *var_sp_s_w_dn5_slot = var_sp_s_w_dn5;
        *var_sp_s_w_dn6_slot = var_sp_s_w_dn6;
        *var_sp_s_w_dn7_slot = var_sp_s_w_dn7;
        *var_sp_s_w_dn8_slot = var_sp_s_w_dn8;
        *var_sp_s_x0_slot = var_sp_s_x0;
        *var_sp_s_x0_dn5_slot = var_sp_s_x0_dn5;
        *var_sp_s_x0_dn6_slot = var_sp_s_x0_dn6;
        *var_sp_s_x0_dn7_slot = var_sp_s_x0_dn7;
        *var_sp_s_x0_dn8_slot = var_sp_s_x0_dn8;
        *var_sp_s_x1_slot = var_sp_s_x1;
        *var_sp_s_x1_dn5_slot = var_sp_s_x1_dn5;
        *var_sp_s_x1_dn6_slot = var_sp_s_x1_dn6;
        *var_sp_s_x1_dn7_slot = var_sp_s_x1_dn7;
        *var_sp_s_x1_dn8_slot = var_sp_s_x1_dn8;
        *var_sp_s_xi0_slot = var_sp_s_xi0;
        *var_sp_s_xi0_dn5_slot = var_sp_s_xi0_dn5;
        *var_sp_s_xi0_dn6_slot = var_sp_s_xi0_dn6;
        *var_sp_s_xi0_dn7_slot = var_sp_s_xi0_dn7;
        *var_sp_s_xi0_dn8_slot = var_sp_s_xi0_dn8;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn5_slot = var_sp_s_xi1_dn5;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn5_slot = var_sp_s_xi2_dn5;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
    }

    pub(super) fn stamp_transient_block_91(
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_guard1182: f64,
        var_guard1183: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_sp_s_delta0: f64,
        var_sp_s_delta0_dn5: f64,
        var_sp_s_delta0_dn6: f64,
        var_sp_s_delta0_dn7: f64,
        var_sp_s_delta0_dn8: f64,
        var_sp_s_delta1: f64,
        var_sp_s_delta1_dn5: f64,
        var_sp_s_delta1_dn6: f64,
        var_sp_s_delta1_dn7: f64,
        var_sp_s_delta1_dn8: f64,
        var_sp_s_x0: f64,
        var_sp_s_x0_dn5: f64,
        var_sp_s_x0_dn6: f64,
        var_sp_s_x0_dn7: f64,
        var_sp_s_x0_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xn_s: f64,
        var_xn_s_dn5: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_alphas_slot: &mut f64,
        var_alphas_dn5_slot: &mut f64,
        var_alphas_dn6_slot: &mut f64,
        var_alphas_dn7_slot: &mut f64,
        var_alphas_dn8_slot: &mut f64,
        var_delta_1s_slot: &mut f64,
        var_delta_1s_dn5_slot: &mut f64,
        var_delta_1s_dn6_slot: &mut f64,
        var_delta_1s_dn7_slot: &mut f64,
        var_delta_1s_dn8_slot: &mut f64,
        var_ds_slot: &mut f64,
        var_ds_dn5_slot: &mut f64,
        var_ds_dn6_slot: &mut f64,
        var_ds_dn7_slot: &mut f64,
        var_ds_dn8_slot: &mut f64,
        var_es_slot: &mut f64,
        var_es_dn5_slot: &mut f64,
        var_es_dn6_slot: &mut f64,
        var_es_dn7_slot: &mut f64,
        var_es_dn8_slot: &mut f64,
        var_factheta_slot: &mut f64,
        var_factheta_dn5_slot: &mut f64,
        var_factheta_dn6_slot: &mut f64,
        var_factheta_dn7_slot: &mut f64,
        var_factheta_dn8_slot: &mut f64,
        var_gmobs_slot: &mut f64,
        var_gmobs_dn5_slot: &mut f64,
        var_gmobs_dn6_slot: &mut f64,
        var_gmobs_dn7_slot: &mut f64,
        var_gmobs_dn8_slot: &mut f64,
        var_guard1188_slot: &mut f64,
        var_guard1189_slot: &mut f64,
        var_guard1190_slot: &mut f64,
        var_guard1191_slot: &mut f64,
        var_ps_slot: &mut f64,
        var_ps_dn5_slot: &mut f64,
        var_ps_dn6_slot: &mut f64,
        var_ps_dn7_slot: &mut f64,
        var_ps_dn8_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn5_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbs_dn8_slot: &mut f64,
        var_qis_slot: &mut f64,
        var_qis_dn5_slot: &mut f64,
        var_qis_dn6_slot: &mut f64,
        var_qis_dn7_slot: &mut f64,
        var_qis_dn8_slot: &mut f64,
        var_rhob_slot: &mut f64,
        var_rhob_dn5_slot: &mut f64,
        var_rhob_dn6_slot: &mut f64,
        var_rhob_dn7_slot: &mut f64,
        var_rhob_dn8_slot: &mut f64,
        var_rhog_slot: &mut f64,
        var_rhog_dn5_slot: &mut f64,
        var_rhog_dn6_slot: &mut f64,
        var_rhog_dn7_slot: &mut f64,
        var_rhog_dn8_slot: &mut f64,
        var_rxcor_slot: &mut f64,
        var_rxcor_dn5_slot: &mut f64,
        var_rxcor_dn6_slot: &mut f64,
        var_rxcor_dn7_slot: &mut f64,
        var_rxcor_dn8_slot: &mut f64,
        var_sp_s_pc_slot: &mut f64,
        var_sp_s_pc_dn5_slot: &mut f64,
        var_sp_s_pc_dn6_slot: &mut f64,
        var_sp_s_pc_dn7_slot: &mut f64,
        var_sp_s_pc_dn8_slot: &mut f64,
        var_sp_s_qc_slot: &mut f64,
        var_sp_s_qc_dn5_slot: &mut f64,
        var_sp_s_qc_dn6_slot: &mut f64,
        var_sp_s_qc_dn7_slot: &mut f64,
        var_sp_s_qc_dn8_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp_dn5_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_xi0_slot: &mut f64,
        var_sp_s_xi0_dn5_slot: &mut f64,
        var_sp_s_xi0_dn6_slot: &mut f64,
        var_sp_s_xi0_dn7_slot: &mut f64,
        var_sp_s_xi0_dn8_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn5_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn5_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
        var_sqs_slot: &mut f64,
        var_sqs_dn5_slot: &mut f64,
        var_sqs_dn6_slot: &mut f64,
        var_sqs_dn7_slot: &mut f64,
        var_sqs_dn8_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_x_s_slot: &mut f64,
        var_x_s_dn5_slot: &mut f64,
        var_x_s_dn6_slot: &mut f64,
        var_x_s_dn7_slot: &mut f64,
        var_x_s_dn8_slot: &mut f64,
        var_xgs_slot: &mut f64,
        var_xgs_dn5_slot: &mut f64,
        var_xgs_dn6_slot: &mut f64,
        var_xgs_dn7_slot: &mut f64,
        var_xgs_dn8_slot: &mut f64,
        var_xi0s_slot: &mut f64,
        var_xi0s_dn5_slot: &mut f64,
        var_xi0s_dn6_slot: &mut f64,
        var_xi0s_dn7_slot: &mut f64,
        var_xi0s_dn8_slot: &mut f64,
        var_xi1s_slot: &mut f64,
        var_xi1s_dn5_slot: &mut f64,
        var_xi1s_dn6_slot: &mut f64,
        var_xi1s_dn7_slot: &mut f64,
        var_xi1s_dn8_slot: &mut f64,
        var_xi2s_slot: &mut f64,
        var_xi2s_dn5_slot: &mut f64,
        var_xi2s_dn6_slot: &mut f64,
        var_xi2s_dn7_slot: &mut f64,
        var_xi2s_dn8_slot: &mut f64,
        var_xitsb_slot: &mut f64,
        var_xitsb_dn5_slot: &mut f64,
        var_xitsb_dn6_slot: &mut f64,
        var_xitsb_dn7_slot: &mut f64,
        var_xitsb_dn8_slot: &mut f64,
    ) {
        let mut var_alphas: f64 = *var_alphas_slot;
        let mut var_alphas_dn5: f64 = *var_alphas_dn5_slot;
        let mut var_alphas_dn6: f64 = *var_alphas_dn6_slot;
        let mut var_alphas_dn7: f64 = *var_alphas_dn7_slot;
        let mut var_alphas_dn8: f64 = *var_alphas_dn8_slot;
        let mut var_delta_1s: f64 = *var_delta_1s_slot;
        let mut var_delta_1s_dn5: f64 = *var_delta_1s_dn5_slot;
        let mut var_delta_1s_dn6: f64 = *var_delta_1s_dn6_slot;
        let mut var_delta_1s_dn7: f64 = *var_delta_1s_dn7_slot;
        let mut var_delta_1s_dn8: f64 = *var_delta_1s_dn8_slot;
        let mut var_ds: f64 = *var_ds_slot;
        let mut var_ds_dn5: f64 = *var_ds_dn5_slot;
        let mut var_ds_dn6: f64 = *var_ds_dn6_slot;
        let mut var_ds_dn7: f64 = *var_ds_dn7_slot;
        let mut var_ds_dn8: f64 = *var_ds_dn8_slot;
        let mut var_es: f64 = *var_es_slot;
        let mut var_es_dn5: f64 = *var_es_dn5_slot;
        let mut var_es_dn6: f64 = *var_es_dn6_slot;
        let mut var_es_dn7: f64 = *var_es_dn7_slot;
        let mut var_es_dn8: f64 = *var_es_dn8_slot;
        let mut var_factheta: f64 = *var_factheta_slot;
        let mut var_factheta_dn5: f64 = *var_factheta_dn5_slot;
        let mut var_factheta_dn6: f64 = *var_factheta_dn6_slot;
        let mut var_factheta_dn7: f64 = *var_factheta_dn7_slot;
        let mut var_factheta_dn8: f64 = *var_factheta_dn8_slot;
        let mut var_gmobs: f64 = *var_gmobs_slot;
        let mut var_gmobs_dn5: f64 = *var_gmobs_dn5_slot;
        let mut var_gmobs_dn6: f64 = *var_gmobs_dn6_slot;
        let mut var_gmobs_dn7: f64 = *var_gmobs_dn7_slot;
        let mut var_gmobs_dn8: f64 = *var_gmobs_dn8_slot;
        let mut var_guard1188: f64 = *var_guard1188_slot;
        let mut var_guard1189: f64 = *var_guard1189_slot;
        let mut var_guard1190: f64 = *var_guard1190_slot;
        let mut var_guard1191: f64 = *var_guard1191_slot;
        let mut var_ps: f64 = *var_ps_slot;
        let mut var_ps_dn5: f64 = *var_ps_dn5_slot;
        let mut var_ps_dn6: f64 = *var_ps_dn6_slot;
        let mut var_ps_dn7: f64 = *var_ps_dn7_slot;
        let mut var_ps_dn8: f64 = *var_ps_dn8_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn5: f64 = *var_qbs_dn5_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbs_dn8: f64 = *var_qbs_dn8_slot;
        let mut var_qis: f64 = *var_qis_slot;
        let mut var_qis_dn5: f64 = *var_qis_dn5_slot;
        let mut var_qis_dn6: f64 = *var_qis_dn6_slot;
        let mut var_qis_dn7: f64 = *var_qis_dn7_slot;
        let mut var_qis_dn8: f64 = *var_qis_dn8_slot;
        let mut var_rhob: f64 = *var_rhob_slot;
        let mut var_rhob_dn5: f64 = *var_rhob_dn5_slot;
        let mut var_rhob_dn6: f64 = *var_rhob_dn6_slot;
        let mut var_rhob_dn7: f64 = *var_rhob_dn7_slot;
        let mut var_rhob_dn8: f64 = *var_rhob_dn8_slot;
        let mut var_rhog: f64 = *var_rhog_slot;
        let mut var_rhog_dn5: f64 = *var_rhog_dn5_slot;
        let mut var_rhog_dn6: f64 = *var_rhog_dn6_slot;
        let mut var_rhog_dn7: f64 = *var_rhog_dn7_slot;
        let mut var_rhog_dn8: f64 = *var_rhog_dn8_slot;
        let mut var_rxcor: f64 = *var_rxcor_slot;
        let mut var_rxcor_dn5: f64 = *var_rxcor_dn5_slot;
        let mut var_rxcor_dn6: f64 = *var_rxcor_dn6_slot;
        let mut var_rxcor_dn7: f64 = *var_rxcor_dn7_slot;
        let mut var_rxcor_dn8: f64 = *var_rxcor_dn8_slot;
        let mut var_sp_s_pc: f64 = *var_sp_s_pc_slot;
        let mut var_sp_s_pc_dn5: f64 = *var_sp_s_pc_dn5_slot;
        let mut var_sp_s_pc_dn6: f64 = *var_sp_s_pc_dn6_slot;
        let mut var_sp_s_pc_dn7: f64 = *var_sp_s_pc_dn7_slot;
        let mut var_sp_s_pc_dn8: f64 = *var_sp_s_pc_dn8_slot;
        let mut var_sp_s_qc: f64 = *var_sp_s_qc_slot;
        let mut var_sp_s_qc_dn5: f64 = *var_sp_s_qc_dn5_slot;
        let mut var_sp_s_qc_dn6: f64 = *var_sp_s_qc_dn6_slot;
        let mut var_sp_s_qc_dn7: f64 = *var_sp_s_qc_dn7_slot;
        let mut var_sp_s_qc_dn8: f64 = *var_sp_s_qc_dn8_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp_dn5: f64 = *var_sp_s_temp_dn5_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_xi0: f64 = *var_sp_s_xi0_slot;
        let mut var_sp_s_xi0_dn5: f64 = *var_sp_s_xi0_dn5_slot;
        let mut var_sp_s_xi0_dn6: f64 = *var_sp_s_xi0_dn6_slot;
        let mut var_sp_s_xi0_dn7: f64 = *var_sp_s_xi0_dn7_slot;
        let mut var_sp_s_xi0_dn8: f64 = *var_sp_s_xi0_dn8_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn5: f64 = *var_sp_s_xi1_dn5_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn5: f64 = *var_sp_s_xi2_dn5_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;
        let mut var_sqs: f64 = *var_sqs_slot;
        let mut var_sqs_dn5: f64 = *var_sqs_dn5_slot;
        let mut var_sqs_dn6: f64 = *var_sqs_dn6_slot;
        let mut var_sqs_dn7: f64 = *var_sqs_dn7_slot;
        let mut var_sqs_dn8: f64 = *var_sqs_dn8_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_x_s: f64 = *var_x_s_slot;
        let mut var_x_s_dn5: f64 = *var_x_s_dn5_slot;
        let mut var_x_s_dn6: f64 = *var_x_s_dn6_slot;
        let mut var_x_s_dn7: f64 = *var_x_s_dn7_slot;
        let mut var_x_s_dn8: f64 = *var_x_s_dn8_slot;
        let mut var_xgs: f64 = *var_xgs_slot;
        let mut var_xgs_dn5: f64 = *var_xgs_dn5_slot;
        let mut var_xgs_dn6: f64 = *var_xgs_dn6_slot;
        let mut var_xgs_dn7: f64 = *var_xgs_dn7_slot;
        let mut var_xgs_dn8: f64 = *var_xgs_dn8_slot;
        let mut var_xi0s: f64 = *var_xi0s_slot;
        let mut var_xi0s_dn5: f64 = *var_xi0s_dn5_slot;
        let mut var_xi0s_dn6: f64 = *var_xi0s_dn6_slot;
        let mut var_xi0s_dn7: f64 = *var_xi0s_dn7_slot;
        let mut var_xi0s_dn8: f64 = *var_xi0s_dn8_slot;
        let mut var_xi1s: f64 = *var_xi1s_slot;
        let mut var_xi1s_dn5: f64 = *var_xi1s_dn5_slot;
        let mut var_xi1s_dn6: f64 = *var_xi1s_dn6_slot;
        let mut var_xi1s_dn7: f64 = *var_xi1s_dn7_slot;
        let mut var_xi1s_dn8: f64 = *var_xi1s_dn8_slot;
        let mut var_xi2s: f64 = *var_xi2s_slot;
        let mut var_xi2s_dn5: f64 = *var_xi2s_dn5_slot;
        let mut var_xi2s_dn6: f64 = *var_xi2s_dn6_slot;
        let mut var_xi2s_dn7: f64 = *var_xi2s_dn7_slot;
        let mut var_xi2s_dn8: f64 = *var_xi2s_dn8_slot;
        let mut var_xitsb: f64 = *var_xitsb_slot;
        let mut var_xitsb_dn5: f64 = *var_xitsb_dn5_slot;
        let mut var_xitsb_dn6: f64 = *var_xitsb_dn6_slot;
        let mut var_xitsb_dn7: f64 = *var_xitsb_dn7_slot;
        let mut var_xitsb_dn8: f64 = *var_xitsb_dn8_slot;

        let (assign42200_e55490, assign42200_e55490_d_n5, assign42200_e55490_d_n6, assign42200_e55490_d_n7, assign42200_e55490_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42200_e55486: f64 = (var_sp_s_x0 * var_sp_s_x0);
        let assign42200_e55487: f64 = (2.0 + assign42200_e55486);
        let assign42200_e55488: f64 = (1.0 / assign42200_e55487);
        (assign42200_e55488, (-(((var_sp_s_x0_dn5 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn5)) / (assign42200_e55487 * assign42200_e55487))), (-(((var_sp_s_x0_dn6 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn6)) / (assign42200_e55487 * assign42200_e55487))), (-(((var_sp_s_x0_dn7 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn7)) / (assign42200_e55487 * assign42200_e55487))), (-(((var_sp_s_x0_dn8 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn8)) / (assign42200_e55487 * assign42200_e55487))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign42200_e55490;
        var_sp_s_temp_dn5 = assign42200_e55490_d_n5;
        var_sp_s_temp_dn6 = assign42200_e55490_d_n6;
        var_sp_s_temp_dn7 = assign42200_e55490_d_n7;
        var_sp_s_temp_dn8 = assign42200_e55490_d_n8;

        let (assign42210_e55502, assign42210_e55502_d_n5, assign42210_e55502_d_n6, assign42210_e55502_d_n7, assign42210_e55502_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42210_e55498: f64 = (var_sp_s_x0 * var_sp_s_x0);
        let assign42210_e55500: f64 = (assign42210_e55498 * var_sp_s_temp);
        (assign42210_e55500, ((((var_sp_s_x0_dn5 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn5)) * var_sp_s_temp) + (assign42210_e55498 * var_sp_s_temp_dn5)), ((((var_sp_s_x0_dn6 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn6)) * var_sp_s_temp) + (assign42210_e55498 * var_sp_s_temp_dn6)), ((((var_sp_s_x0_dn7 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn7)) * var_sp_s_temp) + (assign42210_e55498 * var_sp_s_temp_dn7)), ((((var_sp_s_x0_dn8 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn8)) * var_sp_s_temp) + (assign42210_e55498 * var_sp_s_temp_dn8)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn5, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8,)
    }
};
        var_sp_s_xi0 = assign42210_e55502;
        var_sp_s_xi0_dn5 = assign42210_e55502_d_n5;
        var_sp_s_xi0_dn6 = assign42210_e55502_d_n6;
        var_sp_s_xi0_dn7 = assign42210_e55502_d_n7;
        var_sp_s_xi0_dn8 = assign42210_e55502_d_n8;

        let (assign42220_e55516, assign42220_e55516_d_n5, assign42220_e55516_d_n6, assign42220_e55516_d_n7, assign42220_e55516_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42220_e55511: f64 = (var_sp_s_x0 * var_sp_s_temp);
        let assign42220_e55513: f64 = (assign42220_e55511 * var_sp_s_temp);
        let assign42220_e55514: f64 = (4.0 * assign42220_e55513);
        (assign42220_e55514, (4.0 * ((((var_sp_s_x0_dn5 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn5)) * var_sp_s_temp) + (assign42220_e55511 * var_sp_s_temp_dn5))), (4.0 * ((((var_sp_s_x0_dn6 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign42220_e55511 * var_sp_s_temp_dn6))), (4.0 * ((((var_sp_s_x0_dn7 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign42220_e55511 * var_sp_s_temp_dn7))), (4.0 * ((((var_sp_s_x0_dn8 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign42220_e55511 * var_sp_s_temp_dn8))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn5, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8,)
    }
};
        var_sp_s_xi1 = assign42220_e55516;
        var_sp_s_xi1_dn5 = assign42220_e55516_d_n5;
        var_sp_s_xi1_dn6 = assign42220_e55516_d_n6;
        var_sp_s_xi1_dn7 = assign42220_e55516_d_n7;
        var_sp_s_xi1_dn8 = assign42220_e55516_d_n8;

        let (assign42230_e55534, assign42230_e55534_d_n5, assign42230_e55534_d_n6, assign42230_e55534_d_n7, assign42230_e55534_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42230_e55524: f64 = (8.0 * var_sp_s_temp);
        let assign42230_e55527: f64 = (12.0 * var_sp_s_xi0);
        let assign42230_e55528: f64 = (assign42230_e55524 - assign42230_e55527);
        let assign42230_e55530: f64 = (assign42230_e55528 * var_sp_s_temp);
        let assign42230_e55532: f64 = (assign42230_e55530 * var_sp_s_temp);
        (assign42230_e55532, ((((((8.0 * var_sp_s_temp_dn5) - (12.0 * var_sp_s_xi0_dn5)) * var_sp_s_temp) + (assign42230_e55528 * var_sp_s_temp_dn5)) * var_sp_s_temp) + (assign42230_e55530 * var_sp_s_temp_dn5)), ((((((8.0 * var_sp_s_temp_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp) + (assign42230_e55528 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign42230_e55530 * var_sp_s_temp_dn6)), ((((((8.0 * var_sp_s_temp_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp) + (assign42230_e55528 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign42230_e55530 * var_sp_s_temp_dn7)), ((((((8.0 * var_sp_s_temp_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp) + (assign42230_e55528 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign42230_e55530 * var_sp_s_temp_dn8)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn5, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8,)
    }
};
        var_sp_s_xi2 = assign42230_e55534;
        var_sp_s_xi2_dn5 = assign42230_e55534_d_n5;
        var_sp_s_xi2_dn6 = assign42230_e55534_d_n6;
        var_sp_s_xi2_dn7 = assign42230_e55534_d_n7;
        var_sp_s_xi2_dn8 = assign42230_e55534_d_n8;

        let (assign42240_e55544, assign42240_e55544_d_n5, assign42240_e55544_d_n6, assign42240_e55544_d_n7, assign42240_e55544_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42240_e55542: f64 = (var_xg - var_sp_s_x0);
        (assign42240_e55542, (var_xg_dn5 - var_sp_s_x0_dn5), (var_xg_dn6 - var_sp_s_x0_dn6), (var_xg_dn7 - var_sp_s_x0_dn7), (var_xg_dn8 - var_sp_s_x0_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign42240_e55544;
        var_sp_s_temp_dn5 = assign42240_e55544_d_n5;
        var_sp_s_temp_dn6 = assign42240_e55544_d_n6;
        var_sp_s_temp_dn7 = assign42240_e55544_d_n7;
        var_sp_s_temp_dn8 = assign42240_e55544_d_n8;

        let (assign42250_e55568, assign42250_e55568_d_n5, assign42250_e55568_d_n6, assign42250_e55568_d_n7, assign42250_e55568_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42250_e55552: f64 = (2.0 * var_sp_s_temp);
        let assign42250_e55556: f64 = (1.0 - var_sp_s_delta1);
        let assign42250_e55558: f64 = (assign42250_e55556 + var_sp_s_delta0);
        let assign42250_e55562: f64 = (1.0 + var_sp_s_xi1);
        let assign42250_e55563: f64 = (var_delta_ns * assign42250_e55562);
        let assign42250_e55564: f64 = (assign42250_e55558 - assign42250_e55563);
        let assign42250_e55565: f64 = (var_gf2 * assign42250_e55564);
        let assign42250_e55566: f64 = (assign42250_e55552 + assign42250_e55565);
        (assign42250_e55566, ((2.0 * var_sp_s_temp_dn5) + ((var_gf2_dn5 * assign42250_e55564) + (var_gf2 * (((-var_sp_s_delta1_dn5) + var_sp_s_delta0_dn5) - ((var_delta_ns_dn5 * assign42250_e55562) + (var_delta_ns * var_sp_s_xi1_dn5)))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign42250_e55564) + (var_gf2 * (((-var_sp_s_delta1_dn6) + var_sp_s_delta0_dn6) - ((var_delta_ns_dn6 * assign42250_e55562) + (var_delta_ns * var_sp_s_xi1_dn6)))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign42250_e55564) + (var_gf2 * (((-var_sp_s_delta1_dn7) + var_sp_s_delta0_dn7) - ((var_delta_ns_dn7 * assign42250_e55562) + (var_delta_ns * var_sp_s_xi1_dn7)))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign42250_e55564) + (var_gf2 * (((-var_sp_s_delta1_dn8) + var_sp_s_delta0_dn8) - ((var_delta_ns_dn8 * assign42250_e55562) + (var_delta_ns * var_sp_s_xi1_dn8)))))),)
    } else {
        (var_sp_s_pc, var_sp_s_pc_dn5, var_sp_s_pc_dn6, var_sp_s_pc_dn7, var_sp_s_pc_dn8,)
    }
};
        var_sp_s_pc = assign42250_e55568;
        var_sp_s_pc_dn5 = assign42250_e55568_d_n5;
        var_sp_s_pc_dn6 = assign42250_e55568_d_n6;
        var_sp_s_pc_dn7 = assign42250_e55568_d_n7;
        var_sp_s_pc_dn8 = assign42250_e55568_d_n8;

        let (assign42260_e55596, assign42260_e55596_d_n5, assign42260_e55596_d_n6, assign42260_e55596_d_n7, assign42260_e55596_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42260_e55576: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign42260_e55580: f64 = (var_sp_s_delta1 + var_sp_s_x0);
        let assign42260_e55582: f64 = (assign42260_e55580 - 1.0);
        let assign42260_e55584: f64 = (assign42260_e55582 + var_sp_s_delta0);
        let assign42260_e55588: f64 = (var_sp_s_x0 + 1.0);
        let assign42260_e55590: f64 = (assign42260_e55588 + var_sp_s_xi0);
        let assign42260_e55591: f64 = (var_delta_ns * assign42260_e55590);
        let assign42260_e55592: f64 = (assign42260_e55584 - assign42260_e55591);
        let assign42260_e55593: f64 = (var_gf2 * assign42260_e55592);
        let assign42260_e55594: f64 = (assign42260_e55576 - assign42260_e55593);
        (assign42260_e55594, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) - ((var_gf2_dn5 * assign42260_e55592) + (var_gf2 * (((var_sp_s_delta1_dn5 + var_sp_s_x0_dn5) + var_sp_s_delta0_dn5) - ((var_delta_ns_dn5 * assign42260_e55590) + (var_delta_ns * (var_sp_s_x0_dn5 + var_sp_s_xi0_dn5))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign42260_e55592) + (var_gf2 * (((var_sp_s_delta1_dn6 + var_sp_s_x0_dn6) + var_sp_s_delta0_dn6) - ((var_delta_ns_dn6 * assign42260_e55590) + (var_delta_ns * (var_sp_s_x0_dn6 + var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign42260_e55592) + (var_gf2 * (((var_sp_s_delta1_dn7 + var_sp_s_x0_dn7) + var_sp_s_delta0_dn7) - ((var_delta_ns_dn7 * assign42260_e55590) + (var_delta_ns * (var_sp_s_x0_dn7 + var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign42260_e55592) + (var_gf2 * (((var_sp_s_delta1_dn8 + var_sp_s_x0_dn8) + var_sp_s_delta0_dn8) - ((var_delta_ns_dn8 * assign42260_e55590) + (var_delta_ns * (var_sp_s_x0_dn8 + var_sp_s_xi0_dn8))))))),)
    } else {
        (var_sp_s_qc, var_sp_s_qc_dn5, var_sp_s_qc_dn6, var_sp_s_qc_dn7, var_sp_s_qc_dn8,)
    }
};
        var_sp_s_qc = assign42260_e55596;
        var_sp_s_qc_dn5 = assign42260_e55596_d_n5;
        var_sp_s_qc_dn6 = assign42260_e55596_d_n6;
        var_sp_s_qc_dn7 = assign42260_e55596_d_n7;
        var_sp_s_qc_dn8 = assign42260_e55596_d_n8;

        let (assign42270_e55614, assign42270_e55614_d_n5, assign42270_e55614_d_n6, assign42270_e55614_d_n7, assign42270_e55614_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42270_e55606: f64 = (var_sp_s_delta1 + var_sp_s_delta0);
        let assign42270_e55609: f64 = (var_delta_ns * var_sp_s_xi2);
        let assign42270_e55610: f64 = (assign42270_e55606 - assign42270_e55609);
        let assign42270_e55611: f64 = (var_gf2 * assign42270_e55610);
        let assign42270_e55612: f64 = (2.0 - assign42270_e55611);
        (assign42270_e55612, (-((var_gf2_dn5 * assign42270_e55610) + (var_gf2 * ((var_sp_s_delta1_dn5 + var_sp_s_delta0_dn5) - ((var_delta_ns_dn5 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn5)))))), (-((var_gf2_dn6 * assign42270_e55610) + (var_gf2 * ((var_sp_s_delta1_dn6 + var_sp_s_delta0_dn6) - ((var_delta_ns_dn6 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn6)))))), (-((var_gf2_dn7 * assign42270_e55610) + (var_gf2 * ((var_sp_s_delta1_dn7 + var_sp_s_delta0_dn7) - ((var_delta_ns_dn7 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn7)))))), (-((var_gf2_dn8 * assign42270_e55610) + (var_gf2 * ((var_sp_s_delta1_dn8 + var_sp_s_delta0_dn8) - ((var_delta_ns_dn8 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn8)))))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign42270_e55614;
        var_sp_s_temp_dn5 = assign42270_e55614_d_n5;
        var_sp_s_temp_dn6 = assign42270_e55614_d_n6;
        var_sp_s_temp_dn7 = assign42270_e55614_d_n7;
        var_sp_s_temp_dn8 = assign42270_e55614_d_n8;

        let (assign42280_e55630, assign42280_e55630_d_n5, assign42280_e55630_d_n6, assign42280_e55630_d_n7, assign42280_e55630_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42280_e55622: f64 = (var_sp_s_pc * var_sp_s_pc);
        let assign42280_e55626: f64 = (var_sp_s_qc * var_sp_s_temp);
        let assign42280_e55627: f64 = (2.0 * assign42280_e55626);
        let assign42280_e55628: f64 = (assign42280_e55622 - assign42280_e55627);
        (assign42280_e55628, (((var_sp_s_pc_dn5 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn5)) - (2.0 * ((var_sp_s_qc_dn5 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn5)))), (((var_sp_s_pc_dn6 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn6)) - (2.0 * ((var_sp_s_qc_dn6 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn6)))), (((var_sp_s_pc_dn7 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn7)) - (2.0 * ((var_sp_s_qc_dn7 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn7)))), (((var_sp_s_pc_dn8 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn8)) - (2.0 * ((var_sp_s_qc_dn8 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn8)))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign42280_e55630;
        var_sp_s_temp_dn5 = assign42280_e55630_d_n5;
        var_sp_s_temp_dn6 = assign42280_e55630_d_n6;
        var_sp_s_temp_dn7 = assign42280_e55630_d_n7;
        var_sp_s_temp_dn8 = assign42280_e55630_d_n8;

        let (assign42290_e55647, assign42290_e55647_d_n5, assign42290_e55647_d_n6, assign42290_e55647_d_n7, assign42290_e55647_d_n8,) = {
    if ((var_guard1182 == 0.0) && (var_guard1183 == 0.0)) {
        let assign42290_e55641: f64 = (var_sp_s_temp).sqrt();
        let assign42290_e55642: f64 = (var_sp_s_pc + assign42290_e55641);
        let assign42290_e55643: f64 = (var_sp_s_qc / assign42290_e55642);
        let assign42290_e55644: f64 = (2.0 * assign42290_e55643);
        let assign42290_e55645: f64 = (var_sp_s_x0 + assign42290_e55644);
        (assign42290_e55645, (var_sp_s_x0_dn5 + (2.0 * (((var_sp_s_qc_dn5 * assign42290_e55642) - (var_sp_s_qc * (var_sp_s_pc_dn5 + (var_sp_s_temp_dn5 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (var_sp_s_x0_dn6 + (2.0 * (((var_sp_s_qc_dn6 * assign42290_e55642) - (var_sp_s_qc * (var_sp_s_pc_dn6 + (var_sp_s_temp_dn6 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (var_sp_s_x0_dn7 + (2.0 * (((var_sp_s_qc_dn7 * assign42290_e55642) - (var_sp_s_qc * (var_sp_s_pc_dn7 + (var_sp_s_temp_dn7 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (var_sp_s_x0_dn8 + (2.0 * (((var_sp_s_qc_dn8 * assign42290_e55642) - (var_sp_s_qc * (var_sp_s_pc_dn8 + (var_sp_s_temp_dn8 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))),)
    } else {
        (var_x_s, var_x_s_dn5, var_x_s_dn6, var_x_s_dn7, var_x_s_dn8,)
    }
};
        var_x_s = assign42290_e55647;
        var_x_s_dn5 = assign42290_e55647_d_n5;
        var_x_s_dn6 = assign42290_e55647_d_n6;
        var_x_s_dn7 = assign42290_e55647_d_n7;
        var_x_s_dn8 = assign42290_e55647_d_n8;

        var_xi1s = 0.0;
        var_xi1s_dn5 = 0.0;
        var_xi1s_dn6 = 0.0;
        var_xi1s_dn7 = 0.0;
        var_xi1s_dn8 = 0.0;

        var_xi2s = 0.0;
        var_xi2s_dn5 = 0.0;
        var_xi2s_dn6 = 0.0;
        var_xi2s_dn7 = 0.0;
        var_xi2s_dn8 = 0.0;

        var_delta_1s = 0.0;
        var_delta_1s_dn5 = 0.0;
        var_delta_1s_dn6 = 0.0;
        var_delta_1s_dn7 = 0.0;
        var_delta_1s_dn8 = 0.0;

        var_es = 0.0;
        var_es_dn5 = 0.0;
        var_es_dn6 = 0.0;
        var_es_dn7 = 0.0;
        var_es_dn8 = 0.0;

        var_ds = 0.0;
        var_ds_dn5 = 0.0;
        var_ds_dn6 = 0.0;
        var_ds_dn7 = 0.0;
        var_ds_dn8 = 0.0;

        var_ps = 0.0;
        var_ps_dn5 = 0.0;
        var_ps_dn6 = 0.0;
        var_ps_dn7 = 0.0;
        var_ps_dn8 = 0.0;

        var_sqs = 0.0;
        var_sqs_dn5 = 0.0;
        var_sqs_dn6 = 0.0;
        var_sqs_dn7 = 0.0;
        var_sqs_dn8 = 0.0;

        var_alphas = 1.0;
        var_alphas_dn5 = 0.0;
        var_alphas_dn6 = 0.0;
        var_alphas_dn7 = 0.0;
        var_alphas_dn8 = 0.0;

        var_rxcor = 1.0;
        var_rxcor_dn5 = 0.0;
        var_rxcor_dn6 = 0.0;
        var_rxcor_dn7 = 0.0;
        var_rxcor_dn8 = 0.0;

        let assign42390_e55659: f64 = (var_xg - var_x_s);
        var_xgs = assign42390_e55659;
        var_xgs_dn5 = (var_xg_dn5 - var_x_s_dn5);
        var_xgs_dn6 = (var_xg_dn6 - var_x_s_dn6);
        var_xgs_dn7 = (var_xg_dn7 - var_x_s_dn7);
        var_xgs_dn8 = (var_xg_dn8 - var_x_s_dn8);

        var_qis = 0.0;
        var_qis_dn5 = 0.0;
        var_qis_dn6 = 0.0;
        var_qis_dn7 = 0.0;
        var_qis_dn8 = 0.0;

        let assign42410_e55663: f64 = (var_phit1 * var_xgs);
        var_qbs = assign42410_e55663;
        var_qbs_dn5 = ((var_phit1_dn5 * var_xgs) + (var_phit1 * var_xgs_dn5));
        var_qbs_dn6 = ((var_phit1_dn6 * var_xgs) + (var_phit1 * var_xgs_dn6));
        var_qbs_dn7 = ((var_phit1_dn7 * var_xgs) + (var_phit1 * var_xgs_dn7));
        var_qbs_dn8 = ((var_phit1_dn8 * var_xgs) + (var_phit1 * var_xgs_dn8));

        var_rhob = 1.0;
        var_rhob_dn5 = 0.0;
        var_rhob_dn6 = 0.0;
        var_rhob_dn7 = 0.0;
        var_rhob_dn8 = 0.0;

        var_rhog = 1.0;
        var_rhog_dn5 = 0.0;
        var_rhog_dn6 = 0.0;
        var_rhog_dn7 = 0.0;
        var_rhog_dn8 = 0.0;

        var_gmobs = 1.0;
        var_gmobs_dn5 = 0.0;
        var_gmobs_dn6 = 0.0;
        var_gmobs_dn7 = 0.0;
        var_gmobs_dn8 = 0.0;

        var_xitsb = 1.0;
        var_xitsb_dn5 = 0.0;
        var_xitsb_dn6 = 0.0;
        var_xitsb_dn7 = 0.0;
        var_xitsb_dn8 = 0.0;

        var_factheta = 1.0;
        var_factheta_dn5 = 0.0;
        var_factheta_dn6 = 0.0;
        var_factheta_dn7 = 0.0;
        var_factheta_dn8 = 0.0;

        let assign42470_e55671: f64 = if var_xg > 0.0 { 1.0 } else { 0.0 };
        var_guard1188 = assign42470_e55671;

        let (assign42480_e55681, assign42480_e55681_d_n5, assign42480_e55681_d_n6, assign42480_e55681_d_n7, assign42480_e55681_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42480_e55677: f64 = (var_x_s * var_x_s);
        let assign42480_e55678: f64 = (2.0 + assign42480_e55677);
        let assign42480_e55679: f64 = (1.0 / assign42480_e55678);
        (assign42480_e55679, (-(((var_x_s_dn5 * var_x_s) + (var_x_s * var_x_s_dn5)) / (assign42480_e55678 * assign42480_e55678))), (-(((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)) / (assign42480_e55678 * assign42480_e55678))), (-(((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)) / (assign42480_e55678 * assign42480_e55678))), (-(((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)) / (assign42480_e55678 * assign42480_e55678))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign42480_e55681;
        var_temp__blk936_dn5 = assign42480_e55681_d_n5;
        var_temp__blk936_dn6 = assign42480_e55681_d_n6;
        var_temp__blk936_dn7 = assign42480_e55681_d_n7;
        var_temp__blk936_dn8 = assign42480_e55681_d_n8;

        let (assign42490_e55689, assign42490_e55689_d_n5, assign42490_e55689_d_n6, assign42490_e55689_d_n7, assign42490_e55689_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42490_e55685: f64 = (var_x_s * var_x_s);
        let assign42490_e55687: f64 = (assign42490_e55685 * var_temp__blk936);
        (assign42490_e55687, ((((var_x_s_dn5 * var_x_s) + (var_x_s * var_x_s_dn5)) * var_temp__blk936) + (assign42490_e55685 * var_temp__blk936_dn5)), ((((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)) * var_temp__blk936) + (assign42490_e55685 * var_temp__blk936_dn6)), ((((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)) * var_temp__blk936) + (assign42490_e55685 * var_temp__blk936_dn7)), ((((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)) * var_temp__blk936) + (assign42490_e55685 * var_temp__blk936_dn8)),)
    } else {
        (var_xi0s, var_xi0s_dn5, var_xi0s_dn6, var_xi0s_dn7, var_xi0s_dn8,)
    }
};
        var_xi0s = assign42490_e55689;
        var_xi0s_dn5 = assign42490_e55689_d_n5;
        var_xi0s_dn6 = assign42490_e55689_d_n6;
        var_xi0s_dn7 = assign42490_e55689_d_n7;
        var_xi0s_dn8 = assign42490_e55689_d_n8;

        let (assign42500_e55699, assign42500_e55699_d_n5, assign42500_e55699_d_n6, assign42500_e55699_d_n7, assign42500_e55699_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42500_e55694: f64 = (var_x_s * var_temp__blk936);
        let assign42500_e55696: f64 = (assign42500_e55694 * var_temp__blk936);
        let assign42500_e55697: f64 = (4.0 * assign42500_e55696);
        (assign42500_e55697, (4.0 * ((((var_x_s_dn5 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn5)) * var_temp__blk936) + (assign42500_e55694 * var_temp__blk936_dn5))), (4.0 * ((((var_x_s_dn6 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn6)) * var_temp__blk936) + (assign42500_e55694 * var_temp__blk936_dn6))), (4.0 * ((((var_x_s_dn7 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn7)) * var_temp__blk936) + (assign42500_e55694 * var_temp__blk936_dn7))), (4.0 * ((((var_x_s_dn8 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn8)) * var_temp__blk936) + (assign42500_e55694 * var_temp__blk936_dn8))),)
    } else {
        (var_xi1s, var_xi1s_dn5, var_xi1s_dn6, var_xi1s_dn7, var_xi1s_dn8,)
    }
};
        var_xi1s = assign42500_e55699;
        var_xi1s_dn5 = assign42500_e55699_d_n5;
        var_xi1s_dn6 = assign42500_e55699_d_n6;
        var_xi1s_dn7 = assign42500_e55699_d_n7;
        var_xi1s_dn8 = assign42500_e55699_d_n8;

        let (assign42510_e55713, assign42510_e55713_d_n5, assign42510_e55713_d_n6, assign42510_e55713_d_n7, assign42510_e55713_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42510_e55703: f64 = (8.0 * var_temp__blk936);
        let assign42510_e55706: f64 = (12.0 * var_xi0s);
        let assign42510_e55707: f64 = (assign42510_e55703 - assign42510_e55706);
        let assign42510_e55709: f64 = (assign42510_e55707 * var_temp__blk936);
        let assign42510_e55711: f64 = (assign42510_e55709 * var_temp__blk936);
        (assign42510_e55711, ((((((8.0 * var_temp__blk936_dn5) - (12.0 * var_xi0s_dn5)) * var_temp__blk936) + (assign42510_e55707 * var_temp__blk936_dn5)) * var_temp__blk936) + (assign42510_e55709 * var_temp__blk936_dn5)), ((((((8.0 * var_temp__blk936_dn6) - (12.0 * var_xi0s_dn6)) * var_temp__blk936) + (assign42510_e55707 * var_temp__blk936_dn6)) * var_temp__blk936) + (assign42510_e55709 * var_temp__blk936_dn6)), ((((((8.0 * var_temp__blk936_dn7) - (12.0 * var_xi0s_dn7)) * var_temp__blk936) + (assign42510_e55707 * var_temp__blk936_dn7)) * var_temp__blk936) + (assign42510_e55709 * var_temp__blk936_dn7)), ((((((8.0 * var_temp__blk936_dn8) - (12.0 * var_xi0s_dn8)) * var_temp__blk936) + (assign42510_e55707 * var_temp__blk936_dn8)) * var_temp__blk936) + (assign42510_e55709 * var_temp__blk936_dn8)),)
    } else {
        (var_xi2s, var_xi2s_dn5, var_xi2s_dn6, var_xi2s_dn7, var_xi2s_dn8,)
    }
};
        var_xi2s = assign42510_e55713;
        var_xi2s_dn5 = assign42510_e55713_d_n5;
        var_xi2s_dn6 = assign42510_e55713_d_n6;
        var_xi2s_dn7 = assign42510_e55713_d_n7;
        var_xi2s_dn8 = assign42510_e55713_d_n8;

        let (assign42520_e55717, assign42520_e55717_d_n5, assign42520_e55717_d_n6, assign42520_e55717_d_n7, assign42520_e55717_d_n8,) = {
    if (var_guard1188 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_delta_1s, var_delta_1s_dn5, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8,)
    }
};
        var_delta_1s = assign42520_e55717;
        var_delta_1s_dn5 = assign42520_e55717_d_n5;
        var_delta_1s_dn6 = assign42520_e55717_d_n6;
        var_delta_1s_dn7 = assign42520_e55717_d_n7;
        var_delta_1s_dn8 = assign42520_e55717_d_n8;

        let assign42530_e55720: f64 = if var_x_s < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1189 = assign42530_e55720;

        let (assign42540_e55727, assign42540_e55727_d_n5, assign42540_e55727_d_n6, assign42540_e55727_d_n7, assign42540_e55727_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1189 != 0.0)) {
        let assign42540_e55725: f64 = (var_x_s).exp();
        (assign42540_e55725, (assign42540_e55725 * var_x_s_dn5), (assign42540_e55725 * var_x_s_dn6), (assign42540_e55725 * var_x_s_dn7), (assign42540_e55725 * var_x_s_dn8),)
    } else {
        (var_delta_1s, var_delta_1s_dn5, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8,)
    }
};
        var_delta_1s = assign42540_e55727;
        var_delta_1s_dn5 = assign42540_e55727_d_n5;
        var_delta_1s_dn6 = assign42540_e55727_d_n6;
        var_delta_1s_dn7 = assign42540_e55727_d_n7;
        var_delta_1s_dn8 = assign42540_e55727_d_n8;

        let (assign42550_e55735, assign42550_e55735_d_n5, assign42550_e55735_d_n6, assign42550_e55735_d_n7, assign42550_e55735_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1189 != 0.0)) {
        let assign42550_e55733: f64 = (1.0 / var_delta_1s);
        (assign42550_e55733, (-(var_delta_1s_dn5 / (var_delta_1s * var_delta_1s))), (-(var_delta_1s_dn6 / (var_delta_1s * var_delta_1s))), (-(var_delta_1s_dn7 / (var_delta_1s * var_delta_1s))), (-(var_delta_1s_dn8 / (var_delta_1s * var_delta_1s))),)
    } else {
        (var_es, var_es_dn5, var_es_dn6, var_es_dn7, var_es_dn8,)
    }
};
        var_es = assign42550_e55735;
        var_es_dn5 = assign42550_e55735_d_n5;
        var_es_dn6 = assign42550_e55735_d_n6;
        var_es_dn7 = assign42550_e55735_d_n7;
        var_es_dn8 = assign42550_e55735_d_n8;

        let (assign42560_e55743, assign42560_e55743_d_n5, assign42560_e55743_d_n6, assign42560_e55743_d_n7, assign42560_e55743_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1189 != 0.0)) {
        let assign42560_e55741: f64 = (var_delta_ns * var_delta_1s);
        (assign42560_e55741, ((var_delta_ns_dn5 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn5)), ((var_delta_ns_dn6 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn6)), ((var_delta_ns_dn7 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn7)), ((var_delta_ns_dn8 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn8)),)
    } else {
        (var_delta_1s, var_delta_1s_dn5, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8,)
    }
};
        var_delta_1s = assign42560_e55743;
        var_delta_1s_dn5 = assign42560_e55743_d_n5;
        var_delta_1s_dn6 = assign42560_e55743_d_n6;
        var_delta_1s_dn7 = assign42560_e55743_d_n7;
        var_delta_1s_dn8 = assign42560_e55743_d_n8;

        let assign42570_e55747: f64 = (var_xn_s - 230.25850929940458);
        let assign42570_e55748: f64 = if var_x_s > assign42570_e55747 { 1.0 } else { 0.0 };
        var_guard1190 = assign42570_e55748;

        let (assign42580_e55760, assign42580_e55760_d_n5, assign42580_e55760_d_n6, assign42580_e55760_d_n7, assign42580_e55760_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1189 == 0.0)) && (var_guard1190 != 0.0)) {
        let assign42580_e55757: f64 = (var_x_s - var_xn_s);
        let assign42580_e55758: f64 = (assign42580_e55757).exp();
        (assign42580_e55758, (assign42580_e55758 * (var_x_s_dn5 - var_xn_s_dn5)), (assign42580_e55758 * (var_x_s_dn6 - var_xn_s_dn6)), (assign42580_e55758 * (var_x_s_dn7 - var_xn_s_dn7)), (assign42580_e55758 * (var_x_s_dn8 - var_xn_s_dn8)),)
    } else {
        (var_delta_1s, var_delta_1s_dn5, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8,)
    }
};
        var_delta_1s = assign42580_e55760;
        var_delta_1s_dn5 = assign42580_e55760_d_n5;
        var_delta_1s_dn6 = assign42580_e55760_d_n6;
        var_delta_1s_dn7 = assign42580_e55760_d_n7;
        var_delta_1s_dn8 = assign42580_e55760_d_n8;

        let (assign42590_e55771, assign42590_e55771_d_n5, assign42590_e55771_d_n6, assign42590_e55771_d_n7, assign42590_e55771_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1189 == 0.0)) && (var_guard1190 != 0.0)) {
        let assign42590_e55769: f64 = (var_delta_ns / var_delta_1s);
        (assign42590_e55769, (((var_delta_ns_dn5 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn5)) / (var_delta_1s * var_delta_1s)), (((var_delta_ns_dn6 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn6)) / (var_delta_1s * var_delta_1s)), (((var_delta_ns_dn7 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn7)) / (var_delta_1s * var_delta_1s)), (((var_delta_ns_dn8 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn8)) / (var_delta_1s * var_delta_1s)),)
    } else {
        (var_es, var_es_dn5, var_es_dn6, var_es_dn7, var_es_dn8,)
    }
};
        var_es = assign42590_e55771;
        var_es_dn5 = assign42590_e55771_d_n5;
        var_es_dn6 = assign42590_e55771_d_n6;
        var_es_dn7 = assign42590_e55771_d_n7;
        var_es_dn8 = assign42590_e55771_d_n8;

        let (assign42600_e55809, assign42600_e55809_d_n5, assign42600_e55809_d_n6, assign42600_e55809_d_n7, assign42600_e55809_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1189 == 0.0)) && (var_guard1190 == 0.0)) {
        let assign42600_e55783: f64 = (var_xn_s - var_x_s);
        let assign42600_e55785: f64 = (assign42600_e55783 - 230.25850929940458);
        let assign42600_e55790: f64 = (var_xn_s - var_x_s);
        let assign42600_e55792: f64 = (assign42600_e55790 - 230.25850929940458);
        let assign42600_e55796: f64 = (var_xn_s - var_x_s);
        let assign42600_e55798: f64 = (assign42600_e55796 - 230.25850929940458);
        let assign42600_e55800: f64 = (assign42600_e55798 * 0.3333333333333333);
        let assign42600_e55801: f64 = (1.0 + assign42600_e55800);
        let assign42600_e55802: f64 = (assign42600_e55792 * assign42600_e55801);
        let assign42600_e55803: f64 = (0.5 * assign42600_e55802);
        let assign42600_e55804: f64 = (1.0 + assign42600_e55803);
        let assign42600_e55805: f64 = (assign42600_e55785 * assign42600_e55804);
        let assign42600_e55806: f64 = (1.0 + assign42600_e55805);
        let assign42600_e55807: f64 = (1e-100 / assign42600_e55806);
        (assign42600_e55807, (-((1e-100 * (((var_xn_s_dn5 - var_x_s_dn5) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((var_xn_s_dn5 - var_x_s_dn5) * assign42600_e55801) + (assign42600_e55792 * ((var_xn_s_dn5 - var_x_s_dn5) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((var_xn_s_dn6 - var_x_s_dn6) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((var_xn_s_dn6 - var_x_s_dn6) * assign42600_e55801) + (assign42600_e55792 * ((var_xn_s_dn6 - var_x_s_dn6) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((var_xn_s_dn7 - var_x_s_dn7) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((var_xn_s_dn7 - var_x_s_dn7) * assign42600_e55801) + (assign42600_e55792 * ((var_xn_s_dn7 - var_x_s_dn7) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((var_xn_s_dn8 - var_x_s_dn8) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((var_xn_s_dn8 - var_x_s_dn8) * assign42600_e55801) + (assign42600_e55792 * ((var_xn_s_dn8 - var_x_s_dn8) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))),)
    } else {
        (var_delta_1s, var_delta_1s_dn5, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8,)
    }
};
        var_delta_1s = assign42600_e55809;
        var_delta_1s_dn5 = assign42600_e55809_d_n5;
        var_delta_1s_dn6 = assign42600_e55809_d_n6;
        var_delta_1s_dn7 = assign42600_e55809_d_n7;
        var_delta_1s_dn8 = assign42600_e55809_d_n8;

        let (assign42610_e55841, assign42610_e55841_d_n5, assign42610_e55841_d_n6, assign42610_e55841_d_n7, assign42610_e55841_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1189 == 0.0)) && (var_guard1190 == 0.0)) {
        let assign42610_e55821: f64 = (var_x_s - 230.25850929940458);
        let assign42610_e55826: f64 = (var_x_s - 230.25850929940458);
        let assign42610_e55830: f64 = (var_x_s - 230.25850929940458);
        let assign42610_e55832: f64 = (assign42610_e55830 * 0.3333333333333333);
        let assign42610_e55833: f64 = (1.0 + assign42610_e55832);
        let assign42610_e55834: f64 = (assign42610_e55826 * assign42610_e55833);
        let assign42610_e55835: f64 = (0.5 * assign42610_e55834);
        let assign42610_e55836: f64 = (1.0 + assign42610_e55835);
        let assign42610_e55837: f64 = (assign42610_e55821 * assign42610_e55836);
        let assign42610_e55838: f64 = (1.0 + assign42610_e55837);
        let assign42610_e55839: f64 = (1e-100 / assign42610_e55838);
        (assign42610_e55839, (-((1e-100 * ((var_x_s_dn5 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((var_x_s_dn5 * assign42610_e55833) + (assign42610_e55826 * (var_x_s_dn5 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((var_x_s_dn6 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((var_x_s_dn6 * assign42610_e55833) + (assign42610_e55826 * (var_x_s_dn6 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((var_x_s_dn7 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((var_x_s_dn7 * assign42610_e55833) + (assign42610_e55826 * (var_x_s_dn7 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((var_x_s_dn8 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((var_x_s_dn8 * assign42610_e55833) + (assign42610_e55826 * (var_x_s_dn8 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))),)
    } else {
        (var_es, var_es_dn5, var_es_dn6, var_es_dn7, var_es_dn8,)
    }
};
        var_es = assign42610_e55841;
        var_es_dn5 = assign42610_e55841_d_n5;
        var_es_dn6 = assign42610_e55841_d_n6;
        var_es_dn7 = assign42610_e55841_d_n7;
        var_es_dn8 = assign42610_e55841_d_n8;

        let (assign42620_e55853, assign42620_e55853_d_n5, assign42620_e55853_d_n6, assign42620_e55853_d_n7, assign42620_e55853_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42620_e55847: f64 = (var_x_s + 1.0);
        let assign42620_e55849: f64 = (assign42620_e55847 + var_xi0s);
        let assign42620_e55850: f64 = (var_delta_ns * assign42620_e55849);
        let assign42620_e55851: f64 = (var_delta_1s - assign42620_e55850);
        (assign42620_e55851, (var_delta_1s_dn5 - ((var_delta_ns_dn5 * assign42620_e55849) + (var_delta_ns * (var_x_s_dn5 + var_xi0s_dn5)))), (var_delta_1s_dn6 - ((var_delta_ns_dn6 * assign42620_e55849) + (var_delta_ns * (var_x_s_dn6 + var_xi0s_dn6)))), (var_delta_1s_dn7 - ((var_delta_ns_dn7 * assign42620_e55849) + (var_delta_ns * (var_x_s_dn7 + var_xi0s_dn7)))), (var_delta_1s_dn8 - ((var_delta_ns_dn8 * assign42620_e55849) + (var_delta_ns * (var_x_s_dn8 + var_xi0s_dn8)))),)
    } else {
        (var_ds, var_ds_dn5, var_ds_dn6, var_ds_dn7, var_ds_dn8,)
    }
};
        var_ds = assign42620_e55853;
        var_ds_dn5 = assign42620_e55853_d_n5;
        var_ds_dn6 = assign42620_e55853_d_n6;
        var_ds_dn7 = assign42620_e55853_d_n7;
        var_ds_dn8 = assign42620_e55853_d_n8;

        let assign42630_e55856: f64 = if var_x_s < 1e-5 { 1.0 } else { 0.0 };
        var_guard1191 = assign42630_e55856;

        *var_alphas_slot = var_alphas;
        *var_alphas_dn5_slot = var_alphas_dn5;
        *var_alphas_dn6_slot = var_alphas_dn6;
        *var_alphas_dn7_slot = var_alphas_dn7;
        *var_alphas_dn8_slot = var_alphas_dn8;
        *var_delta_1s_slot = var_delta_1s;
        *var_delta_1s_dn5_slot = var_delta_1s_dn5;
        *var_delta_1s_dn6_slot = var_delta_1s_dn6;
        *var_delta_1s_dn7_slot = var_delta_1s_dn7;
        *var_delta_1s_dn8_slot = var_delta_1s_dn8;
        *var_ds_slot = var_ds;
        *var_ds_dn5_slot = var_ds_dn5;
        *var_ds_dn6_slot = var_ds_dn6;
        *var_ds_dn7_slot = var_ds_dn7;
        *var_ds_dn8_slot = var_ds_dn8;
        *var_es_slot = var_es;
        *var_es_dn5_slot = var_es_dn5;
        *var_es_dn6_slot = var_es_dn6;
        *var_es_dn7_slot = var_es_dn7;
        *var_es_dn8_slot = var_es_dn8;
        *var_factheta_slot = var_factheta;
        *var_factheta_dn5_slot = var_factheta_dn5;
        *var_factheta_dn6_slot = var_factheta_dn6;
        *var_factheta_dn7_slot = var_factheta_dn7;
        *var_factheta_dn8_slot = var_factheta_dn8;
        *var_gmobs_slot = var_gmobs;
        *var_gmobs_dn5_slot = var_gmobs_dn5;
        *var_gmobs_dn6_slot = var_gmobs_dn6;
        *var_gmobs_dn7_slot = var_gmobs_dn7;
        *var_gmobs_dn8_slot = var_gmobs_dn8;
        *var_guard1188_slot = var_guard1188;
        *var_guard1189_slot = var_guard1189;
        *var_guard1190_slot = var_guard1190;
        *var_guard1191_slot = var_guard1191;
        *var_ps_slot = var_ps;
        *var_ps_dn5_slot = var_ps_dn5;
        *var_ps_dn6_slot = var_ps_dn6;
        *var_ps_dn7_slot = var_ps_dn7;
        *var_ps_dn8_slot = var_ps_dn8;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn5_slot = var_qbs_dn5;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbs_dn8_slot = var_qbs_dn8;
        *var_qis_slot = var_qis;
        *var_qis_dn5_slot = var_qis_dn5;
        *var_qis_dn6_slot = var_qis_dn6;
        *var_qis_dn7_slot = var_qis_dn7;
        *var_qis_dn8_slot = var_qis_dn8;
        *var_rhob_slot = var_rhob;
        *var_rhob_dn5_slot = var_rhob_dn5;
        *var_rhob_dn6_slot = var_rhob_dn6;
        *var_rhob_dn7_slot = var_rhob_dn7;
        *var_rhob_dn8_slot = var_rhob_dn8;
        *var_rhog_slot = var_rhog;
        *var_rhog_dn5_slot = var_rhog_dn5;
        *var_rhog_dn6_slot = var_rhog_dn6;
        *var_rhog_dn7_slot = var_rhog_dn7;
        *var_rhog_dn8_slot = var_rhog_dn8;
        *var_rxcor_slot = var_rxcor;
        *var_rxcor_dn5_slot = var_rxcor_dn5;
        *var_rxcor_dn6_slot = var_rxcor_dn6;
        *var_rxcor_dn7_slot = var_rxcor_dn7;
        *var_rxcor_dn8_slot = var_rxcor_dn8;
        *var_sp_s_pc_slot = var_sp_s_pc;
        *var_sp_s_pc_dn5_slot = var_sp_s_pc_dn5;
        *var_sp_s_pc_dn6_slot = var_sp_s_pc_dn6;
        *var_sp_s_pc_dn7_slot = var_sp_s_pc_dn7;
        *var_sp_s_pc_dn8_slot = var_sp_s_pc_dn8;
        *var_sp_s_qc_slot = var_sp_s_qc;
        *var_sp_s_qc_dn5_slot = var_sp_s_qc_dn5;
        *var_sp_s_qc_dn6_slot = var_sp_s_qc_dn6;
        *var_sp_s_qc_dn7_slot = var_sp_s_qc_dn7;
        *var_sp_s_qc_dn8_slot = var_sp_s_qc_dn8;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp_dn5_slot = var_sp_s_temp_dn5;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_xi0_slot = var_sp_s_xi0;
        *var_sp_s_xi0_dn5_slot = var_sp_s_xi0_dn5;
        *var_sp_s_xi0_dn6_slot = var_sp_s_xi0_dn6;
        *var_sp_s_xi0_dn7_slot = var_sp_s_xi0_dn7;
        *var_sp_s_xi0_dn8_slot = var_sp_s_xi0_dn8;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn5_slot = var_sp_s_xi1_dn5;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn5_slot = var_sp_s_xi2_dn5;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
        *var_sqs_slot = var_sqs;
        *var_sqs_dn5_slot = var_sqs_dn5;
        *var_sqs_dn6_slot = var_sqs_dn6;
        *var_sqs_dn7_slot = var_sqs_dn7;
        *var_sqs_dn8_slot = var_sqs_dn8;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_x_s_slot = var_x_s;
        *var_x_s_dn5_slot = var_x_s_dn5;
        *var_x_s_dn6_slot = var_x_s_dn6;
        *var_x_s_dn7_slot = var_x_s_dn7;
        *var_x_s_dn8_slot = var_x_s_dn8;
        *var_xgs_slot = var_xgs;
        *var_xgs_dn5_slot = var_xgs_dn5;
        *var_xgs_dn6_slot = var_xgs_dn6;
        *var_xgs_dn7_slot = var_xgs_dn7;
        *var_xgs_dn8_slot = var_xgs_dn8;
        *var_xi0s_slot = var_xi0s;
        *var_xi0s_dn5_slot = var_xi0s_dn5;
        *var_xi0s_dn6_slot = var_xi0s_dn6;
        *var_xi0s_dn7_slot = var_xi0s_dn7;
        *var_xi0s_dn8_slot = var_xi0s_dn8;
        *var_xi1s_slot = var_xi1s;
        *var_xi1s_dn5_slot = var_xi1s_dn5;
        *var_xi1s_dn6_slot = var_xi1s_dn6;
        *var_xi1s_dn7_slot = var_xi1s_dn7;
        *var_xi1s_dn8_slot = var_xi1s_dn8;
        *var_xi2s_slot = var_xi2s;
        *var_xi2s_dn5_slot = var_xi2s_dn5;
        *var_xi2s_dn6_slot = var_xi2s_dn6;
        *var_xi2s_dn7_slot = var_xi2s_dn7;
        *var_xi2s_dn8_slot = var_xi2s_dn8;
        *var_xitsb_slot = var_xitsb;
        *var_xitsb_dn5_slot = var_xitsb_dn5;
        *var_xitsb_dn6_slot = var_xitsb_dn6;
        *var_xitsb_dn7_slot = var_xitsb_dn7;
        *var_xitsb_dn8_slot = var_xitsb_dn8;
    }

    pub(super) fn stamp_transient_block_92(
        var_cs_t: f64,
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_e_eff0: f64,
        var_es: f64,
        var_es_dn5: f64,
        var_es_dn6: f64,
        var_es_dn7: f64,
        var_es_dn8: f64,
        var_eta_mu: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1188: f64,
        var_guard1191: f64,
        var_inv_gf2: f64,
        var_inv_gf2_dn5: f64,
        var_inv_gf2_dn6: f64,
        var_inv_gf2_dn7: f64,
        var_inv_gf2_dn8: f64,
        var_inv_phit1: f64,
        var_inv_phit1_dn5: f64,
        var_inv_phit1_dn6: f64,
        var_inv_phit1_dn7: f64,
        var_inv_phit1_dn8: f64,
        var_mue_t: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_rsb_i: f64,
        var_rsg_i: f64,
        var_thecs_t: f64,
        var_themu_t: f64,
        var_ther_i: f64,
        var_thesatb_i: f64,
        var_thesatg_i: f64,
        var_thesatt_i: f64,
        var_vgb1: f64,
        var_vgb1_dn5: f64,
        var_vgb1_dn6: f64,
        var_vgb1_dn7: f64,
        var_vgb1_dn8: f64,
        var_vsbx: f64,
        var_vsbx_dn5: f64,
        var_vsbx_dn6: f64,
        var_vsbx_dn7: f64,
        var_vsbx_dn8: f64,
        var_x_s: f64,
        var_x_s_dn5: f64,
        var_x_s_dn6: f64,
        var_x_s_dn7: f64,
        var_x_s_dn8: f64,
        var_xcor_t: f64,
        var_alphas_slot: &mut f64,
        var_alphas_dn5_slot: &mut f64,
        var_alphas_dn6_slot: &mut f64,
        var_alphas_dn7_slot: &mut f64,
        var_alphas_dn8_slot: &mut f64,
        var_ds_slot: &mut f64,
        var_ds_dn5_slot: &mut f64,
        var_ds_dn6_slot: &mut f64,
        var_ds_dn7_slot: &mut f64,
        var_ds_dn8_slot: &mut f64,
        var_eeffs_slot: &mut f64,
        var_eeffs_dn5_slot: &mut f64,
        var_eeffs_dn6_slot: &mut f64,
        var_eeffs_dn7_slot: &mut f64,
        var_eeffs_dn8_slot: &mut f64,
        var_factheta_slot: &mut f64,
        var_factheta_dn5_slot: &mut f64,
        var_factheta_dn6_slot: &mut f64,
        var_factheta_dn7_slot: &mut f64,
        var_factheta_dn8_slot: &mut f64,
        var_gf2_dc_slot: &mut f64,
        var_gf2_dc_dn5_slot: &mut f64,
        var_gf2_dc_dn6_slot: &mut f64,
        var_gf2_dc_dn7_slot: &mut f64,
        var_gf2_dc_dn8_slot: &mut f64,
        var_gf_dc_slot: &mut f64,
        var_gf_dc_dn5_slot: &mut f64,
        var_gf_dc_dn6_slot: &mut f64,
        var_gf_dc_dn7_slot: &mut f64,
        var_gf_dc_dn8_slot: &mut f64,
        var_gmobs_slot: &mut f64,
        var_gmobs_dn5_slot: &mut f64,
        var_gmobs_dn6_slot: &mut f64,
        var_gmobs_dn7_slot: &mut f64,
        var_gmobs_dn8_slot: &mut f64,
        var_gr_slot: &mut f64,
        var_gr_dn5_slot: &mut f64,
        var_gr_dn6_slot: &mut f64,
        var_gr_dn7_slot: &mut f64,
        var_gr_dn8_slot: &mut f64,
        var_guard1192_slot: &mut f64,
        var_guard1193_slot: &mut f64,
        var_guard1194_slot: &mut f64,
        var_guard1195_slot: &mut f64,
        var_guard1196_slot: &mut f64,
        var_inv_gf2_dc_slot: &mut f64,
        var_inv_gf2_dc_dn5_slot: &mut f64,
        var_inv_gf2_dc_dn6_slot: &mut f64,
        var_inv_gf2_dc_dn7_slot: &mut f64,
        var_inv_gf2_dc_dn8_slot: &mut f64,
        var_inv_phit1_dc_slot: &mut f64,
        var_inv_phit1_dc_dn5_slot: &mut f64,
        var_inv_phit1_dc_dn6_slot: &mut f64,
        var_inv_phit1_dc_dn7_slot: &mut f64,
        var_inv_phit1_dc_dn8_slot: &mut f64,
        var_mutmp_slot: &mut f64,
        var_mutmp_dn5_slot: &mut f64,
        var_mutmp_dn6_slot: &mut f64,
        var_mutmp_dn7_slot: &mut f64,
        var_mutmp_dn8_slot: &mut f64,
        var_phit1_dc_slot: &mut f64,
        var_phit1_dc_dn5_slot: &mut f64,
        var_phit1_dc_dn6_slot: &mut f64,
        var_phit1_dc_dn7_slot: &mut f64,
        var_phit1_dc_dn8_slot: &mut f64,
        var_ps_slot: &mut f64,
        var_ps_dn5_slot: &mut f64,
        var_ps_dn6_slot: &mut f64,
        var_ps_dn7_slot: &mut f64,
        var_ps_dn8_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn5_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbs_dn8_slot: &mut f64,
        var_qis_slot: &mut f64,
        var_qis_dn5_slot: &mut f64,
        var_qis_dn6_slot: &mut f64,
        var_qis_dn7_slot: &mut f64,
        var_qis_dn8_slot: &mut f64,
        var_rhob_slot: &mut f64,
        var_rhob_dn5_slot: &mut f64,
        var_rhob_dn6_slot: &mut f64,
        var_rhob_dn7_slot: &mut f64,
        var_rhob_dn8_slot: &mut f64,
        var_rhog_slot: &mut f64,
        var_rhog_dn5_slot: &mut f64,
        var_rhog_dn6_slot: &mut f64,
        var_rhog_dn7_slot: &mut f64,
        var_rhog_dn8_slot: &mut f64,
        var_rxcor_slot: &mut f64,
        var_rxcor_dn5_slot: &mut f64,
        var_rxcor_dn6_slot: &mut f64,
        var_rxcor_dn7_slot: &mut f64,
        var_rxcor_dn8_slot: &mut f64,
        var_sqs_slot: &mut f64,
        var_sqs_dn5_slot: &mut f64,
        var_sqs_dn6_slot: &mut f64,
        var_sqs_dn7_slot: &mut f64,
        var_sqs_dn8_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_vgb1_dc_slot: &mut f64,
        var_vgb1_dc_dn5_slot: &mut f64,
        var_vgb1_dc_dn6_slot: &mut f64,
        var_vgb1_dc_dn7_slot: &mut f64,
        var_vgb1_dc_dn8_slot: &mut f64,
        var_vsbx_dc_slot: &mut f64,
        var_vsbx_dc_dn5_slot: &mut f64,
        var_vsbx_dc_dn6_slot: &mut f64,
        var_vsbx_dc_dn7_slot: &mut f64,
        var_vsbx_dc_dn8_slot: &mut f64,
        var_wsat_slot: &mut f64,
        var_wsat_dn5_slot: &mut f64,
        var_wsat_dn6_slot: &mut f64,
        var_wsat_dn7_slot: &mut f64,
        var_wsat_dn8_slot: &mut f64,
        var_xgs_slot: &mut f64,
        var_xgs_dn5_slot: &mut f64,
        var_xgs_dn6_slot: &mut f64,
        var_xgs_dn7_slot: &mut f64,
        var_xgs_dn8_slot: &mut f64,
        var_xitsb_slot: &mut f64,
        var_xitsb_dn5_slot: &mut f64,
        var_xitsb_dn6_slot: &mut f64,
        var_xitsb_dn7_slot: &mut f64,
        var_xitsb_dn8_slot: &mut f64,
    ) {
        let mut var_alphas: f64 = *var_alphas_slot;
        let mut var_alphas_dn5: f64 = *var_alphas_dn5_slot;
        let mut var_alphas_dn6: f64 = *var_alphas_dn6_slot;
        let mut var_alphas_dn7: f64 = *var_alphas_dn7_slot;
        let mut var_alphas_dn8: f64 = *var_alphas_dn8_slot;
        let mut var_ds: f64 = *var_ds_slot;
        let mut var_ds_dn5: f64 = *var_ds_dn5_slot;
        let mut var_ds_dn6: f64 = *var_ds_dn6_slot;
        let mut var_ds_dn7: f64 = *var_ds_dn7_slot;
        let mut var_ds_dn8: f64 = *var_ds_dn8_slot;
        let mut var_eeffs: f64 = *var_eeffs_slot;
        let mut var_eeffs_dn5: f64 = *var_eeffs_dn5_slot;
        let mut var_eeffs_dn6: f64 = *var_eeffs_dn6_slot;
        let mut var_eeffs_dn7: f64 = *var_eeffs_dn7_slot;
        let mut var_eeffs_dn8: f64 = *var_eeffs_dn8_slot;
        let mut var_factheta: f64 = *var_factheta_slot;
        let mut var_factheta_dn5: f64 = *var_factheta_dn5_slot;
        let mut var_factheta_dn6: f64 = *var_factheta_dn6_slot;
        let mut var_factheta_dn7: f64 = *var_factheta_dn7_slot;
        let mut var_factheta_dn8: f64 = *var_factheta_dn8_slot;
        let mut var_gf2_dc: f64 = *var_gf2_dc_slot;
        let mut var_gf2_dc_dn5: f64 = *var_gf2_dc_dn5_slot;
        let mut var_gf2_dc_dn6: f64 = *var_gf2_dc_dn6_slot;
        let mut var_gf2_dc_dn7: f64 = *var_gf2_dc_dn7_slot;
        let mut var_gf2_dc_dn8: f64 = *var_gf2_dc_dn8_slot;
        let mut var_gf_dc: f64 = *var_gf_dc_slot;
        let mut var_gf_dc_dn5: f64 = *var_gf_dc_dn5_slot;
        let mut var_gf_dc_dn6: f64 = *var_gf_dc_dn6_slot;
        let mut var_gf_dc_dn7: f64 = *var_gf_dc_dn7_slot;
        let mut var_gf_dc_dn8: f64 = *var_gf_dc_dn8_slot;
        let mut var_gmobs: f64 = *var_gmobs_slot;
        let mut var_gmobs_dn5: f64 = *var_gmobs_dn5_slot;
        let mut var_gmobs_dn6: f64 = *var_gmobs_dn6_slot;
        let mut var_gmobs_dn7: f64 = *var_gmobs_dn7_slot;
        let mut var_gmobs_dn8: f64 = *var_gmobs_dn8_slot;
        let mut var_gr: f64 = *var_gr_slot;
        let mut var_gr_dn5: f64 = *var_gr_dn5_slot;
        let mut var_gr_dn6: f64 = *var_gr_dn6_slot;
        let mut var_gr_dn7: f64 = *var_gr_dn7_slot;
        let mut var_gr_dn8: f64 = *var_gr_dn8_slot;
        let mut var_guard1192: f64 = *var_guard1192_slot;
        let mut var_guard1193: f64 = *var_guard1193_slot;
        let mut var_guard1194: f64 = *var_guard1194_slot;
        let mut var_guard1195: f64 = *var_guard1195_slot;
        let mut var_guard1196: f64 = *var_guard1196_slot;
        let mut var_inv_gf2_dc: f64 = *var_inv_gf2_dc_slot;
        let mut var_inv_gf2_dc_dn5: f64 = *var_inv_gf2_dc_dn5_slot;
        let mut var_inv_gf2_dc_dn6: f64 = *var_inv_gf2_dc_dn6_slot;
        let mut var_inv_gf2_dc_dn7: f64 = *var_inv_gf2_dc_dn7_slot;
        let mut var_inv_gf2_dc_dn8: f64 = *var_inv_gf2_dc_dn8_slot;
        let mut var_inv_phit1_dc: f64 = *var_inv_phit1_dc_slot;
        let mut var_inv_phit1_dc_dn5: f64 = *var_inv_phit1_dc_dn5_slot;
        let mut var_inv_phit1_dc_dn6: f64 = *var_inv_phit1_dc_dn6_slot;
        let mut var_inv_phit1_dc_dn7: f64 = *var_inv_phit1_dc_dn7_slot;
        let mut var_inv_phit1_dc_dn8: f64 = *var_inv_phit1_dc_dn8_slot;
        let mut var_mutmp: f64 = *var_mutmp_slot;
        let mut var_mutmp_dn5: f64 = *var_mutmp_dn5_slot;
        let mut var_mutmp_dn6: f64 = *var_mutmp_dn6_slot;
        let mut var_mutmp_dn7: f64 = *var_mutmp_dn7_slot;
        let mut var_mutmp_dn8: f64 = *var_mutmp_dn8_slot;
        let mut var_phit1_dc: f64 = *var_phit1_dc_slot;
        let mut var_phit1_dc_dn5: f64 = *var_phit1_dc_dn5_slot;
        let mut var_phit1_dc_dn6: f64 = *var_phit1_dc_dn6_slot;
        let mut var_phit1_dc_dn7: f64 = *var_phit1_dc_dn7_slot;
        let mut var_phit1_dc_dn8: f64 = *var_phit1_dc_dn8_slot;
        let mut var_ps: f64 = *var_ps_slot;
        let mut var_ps_dn5: f64 = *var_ps_dn5_slot;
        let mut var_ps_dn6: f64 = *var_ps_dn6_slot;
        let mut var_ps_dn7: f64 = *var_ps_dn7_slot;
        let mut var_ps_dn8: f64 = *var_ps_dn8_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn5: f64 = *var_qbs_dn5_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbs_dn8: f64 = *var_qbs_dn8_slot;
        let mut var_qis: f64 = *var_qis_slot;
        let mut var_qis_dn5: f64 = *var_qis_dn5_slot;
        let mut var_qis_dn6: f64 = *var_qis_dn6_slot;
        let mut var_qis_dn7: f64 = *var_qis_dn7_slot;
        let mut var_qis_dn8: f64 = *var_qis_dn8_slot;
        let mut var_rhob: f64 = *var_rhob_slot;
        let mut var_rhob_dn5: f64 = *var_rhob_dn5_slot;
        let mut var_rhob_dn6: f64 = *var_rhob_dn6_slot;
        let mut var_rhob_dn7: f64 = *var_rhob_dn7_slot;
        let mut var_rhob_dn8: f64 = *var_rhob_dn8_slot;
        let mut var_rhog: f64 = *var_rhog_slot;
        let mut var_rhog_dn5: f64 = *var_rhog_dn5_slot;
        let mut var_rhog_dn6: f64 = *var_rhog_dn6_slot;
        let mut var_rhog_dn7: f64 = *var_rhog_dn7_slot;
        let mut var_rhog_dn8: f64 = *var_rhog_dn8_slot;
        let mut var_rxcor: f64 = *var_rxcor_slot;
        let mut var_rxcor_dn5: f64 = *var_rxcor_dn5_slot;
        let mut var_rxcor_dn6: f64 = *var_rxcor_dn6_slot;
        let mut var_rxcor_dn7: f64 = *var_rxcor_dn7_slot;
        let mut var_rxcor_dn8: f64 = *var_rxcor_dn8_slot;
        let mut var_sqs: f64 = *var_sqs_slot;
        let mut var_sqs_dn5: f64 = *var_sqs_dn5_slot;
        let mut var_sqs_dn6: f64 = *var_sqs_dn6_slot;
        let mut var_sqs_dn7: f64 = *var_sqs_dn7_slot;
        let mut var_sqs_dn8: f64 = *var_sqs_dn8_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_vgb1_dc: f64 = *var_vgb1_dc_slot;
        let mut var_vgb1_dc_dn5: f64 = *var_vgb1_dc_dn5_slot;
        let mut var_vgb1_dc_dn6: f64 = *var_vgb1_dc_dn6_slot;
        let mut var_vgb1_dc_dn7: f64 = *var_vgb1_dc_dn7_slot;
        let mut var_vgb1_dc_dn8: f64 = *var_vgb1_dc_dn8_slot;
        let mut var_vsbx_dc: f64 = *var_vsbx_dc_slot;
        let mut var_vsbx_dc_dn5: f64 = *var_vsbx_dc_dn5_slot;
        let mut var_vsbx_dc_dn6: f64 = *var_vsbx_dc_dn6_slot;
        let mut var_vsbx_dc_dn7: f64 = *var_vsbx_dc_dn7_slot;
        let mut var_vsbx_dc_dn8: f64 = *var_vsbx_dc_dn8_slot;
        let mut var_wsat: f64 = *var_wsat_slot;
        let mut var_wsat_dn5: f64 = *var_wsat_dn5_slot;
        let mut var_wsat_dn6: f64 = *var_wsat_dn6_slot;
        let mut var_wsat_dn7: f64 = *var_wsat_dn7_slot;
        let mut var_wsat_dn8: f64 = *var_wsat_dn8_slot;
        let mut var_xgs: f64 = *var_xgs_slot;
        let mut var_xgs_dn5: f64 = *var_xgs_dn5_slot;
        let mut var_xgs_dn6: f64 = *var_xgs_dn6_slot;
        let mut var_xgs_dn7: f64 = *var_xgs_dn7_slot;
        let mut var_xgs_dn8: f64 = *var_xgs_dn8_slot;
        let mut var_xitsb: f64 = *var_xitsb_slot;
        let mut var_xitsb_dn5: f64 = *var_xitsb_dn5_slot;
        let mut var_xitsb_dn6: f64 = *var_xitsb_dn6_slot;
        let mut var_xitsb_dn7: f64 = *var_xitsb_dn7_slot;
        let mut var_xitsb_dn8: f64 = *var_xitsb_dn8_slot;

        let (assign42640_e55878, assign42640_e55878_d_n5, assign42640_e55878_d_n6, assign42640_e55878_d_n7, assign42640_e55878_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 != 0.0)) {
        let assign42640_e55863: f64 = (var_x_s * var_x_s);
        let assign42640_e55870: f64 = (0.25 * var_x_s);
        let assign42640_e55871: f64 = (1.0 - assign42640_e55870);
        let assign42640_e55872: f64 = (var_x_s * assign42640_e55871);
        let assign42640_e55873: f64 = (0.3333333333333333 * assign42640_e55872);
        let assign42640_e55874: f64 = (1.0 - assign42640_e55873);
        let assign42640_e55875: f64 = (assign42640_e55863 * assign42640_e55874);
        let assign42640_e55876: f64 = (0.5 * assign42640_e55875);
        (assign42640_e55876, (0.5 * ((((var_x_s_dn5 * var_x_s) + (var_x_s * var_x_s_dn5)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((var_x_s_dn5 * assign42640_e55871) + (var_x_s * (-(0.25 * var_x_s_dn5))))))))), (0.5 * ((((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((var_x_s_dn6 * assign42640_e55871) + (var_x_s * (-(0.25 * var_x_s_dn6))))))))), (0.5 * ((((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((var_x_s_dn7 * assign42640_e55871) + (var_x_s * (-(0.25 * var_x_s_dn7))))))))), (0.5 * ((((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((var_x_s_dn8 * assign42640_e55871) + (var_x_s * (-(0.25 * var_x_s_dn8))))))))),)
    } else {
        (var_ps, var_ps_dn5, var_ps_dn6, var_ps_dn7, var_ps_dn8,)
    }
};
        var_ps = assign42640_e55878;
        var_ps_dn5 = assign42640_e55878_d_n5;
        var_ps_dn6 = assign42640_e55878_d_n6;
        var_ps_dn7 = assign42640_e55878_d_n7;
        var_ps_dn8 = assign42640_e55878_d_n8;

        let (assign42650_e55898, assign42650_e55898_d_n5, assign42650_e55898_d_n6, assign42650_e55898_d_n7, assign42650_e55898_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 != 0.0)) {
        let assign42650_e55885: f64 = (var_delta_ns * var_x_s);
        let assign42650_e55887: f64 = (assign42650_e55885 * var_x_s);
        let assign42650_e55889: f64 = (assign42650_e55887 * var_x_s);
        let assign42650_e55893: f64 = (1.75 * var_x_s);
        let assign42650_e55894: f64 = (1.0 + assign42650_e55893);
        let assign42650_e55895: f64 = (assign42650_e55889 * assign42650_e55894);
        let assign42650_e55896: f64 = (0.16666666666666666 * assign42650_e55895);
        (assign42650_e55896, (0.16666666666666666 * ((((((((var_delta_ns_dn5 * var_x_s) + (var_delta_ns * var_x_s_dn5)) * var_x_s) + (assign42650_e55885 * var_x_s_dn5)) * var_x_s) + (assign42650_e55887 * var_x_s_dn5)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * var_x_s_dn5)))), (0.16666666666666666 * ((((((((var_delta_ns_dn6 * var_x_s) + (var_delta_ns * var_x_s_dn6)) * var_x_s) + (assign42650_e55885 * var_x_s_dn6)) * var_x_s) + (assign42650_e55887 * var_x_s_dn6)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * var_x_s_dn6)))), (0.16666666666666666 * ((((((((var_delta_ns_dn7 * var_x_s) + (var_delta_ns * var_x_s_dn7)) * var_x_s) + (assign42650_e55885 * var_x_s_dn7)) * var_x_s) + (assign42650_e55887 * var_x_s_dn7)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * var_x_s_dn7)))), (0.16666666666666666 * ((((((((var_delta_ns_dn8 * var_x_s) + (var_delta_ns * var_x_s_dn8)) * var_x_s) + (assign42650_e55885 * var_x_s_dn8)) * var_x_s) + (assign42650_e55887 * var_x_s_dn8)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * var_x_s_dn8)))),)
    } else {
        (var_ds, var_ds_dn5, var_ds_dn6, var_ds_dn7, var_ds_dn8,)
    }
};
        var_ds = assign42650_e55898;
        var_ds_dn5 = assign42650_e55898_d_n5;
        var_ds_dn6 = assign42650_e55898_d_n6;
        var_ds_dn7 = assign42650_e55898_d_n7;
        var_ds_dn8 = assign42650_e55898_d_n8;

        let (assign42660_e55915, assign42660_e55915_d_n5, assign42660_e55915_d_n6, assign42660_e55915_d_n7, assign42660_e55915_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 != 0.0)) {
        let assign42660_e55908: f64 = (0.25 * var_x_s);
        let assign42660_e55909: f64 = (1.0 - assign42660_e55908);
        let assign42660_e55910: f64 = (var_x_s * assign42660_e55909);
        let assign42660_e55911: f64 = (0.3333333333333333 * assign42660_e55910);
        let assign42660_e55912: f64 = (1.0 - assign42660_e55911);
        let assign42660_e55913: f64 = (assign42660_e55912).sqrt();
        (assign42660_e55913, ((-(0.3333333333333333 * ((var_x_s_dn5 * assign42660_e55909) + (var_x_s * (-(0.25 * var_x_s_dn5)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((var_x_s_dn6 * assign42660_e55909) + (var_x_s * (-(0.25 * var_x_s_dn6)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((var_x_s_dn7 * assign42660_e55909) + (var_x_s * (-(0.25 * var_x_s_dn7)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((var_x_s_dn8 * assign42660_e55909) + (var_x_s * (-(0.25 * var_x_s_dn8)))))) / (2.0 * assign42660_e55913)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign42660_e55915;
        var_temp__blk936_dn5 = assign42660_e55915_d_n5;
        var_temp__blk936_dn6 = assign42660_e55915_d_n6;
        var_temp__blk936_dn7 = assign42660_e55915_d_n7;
        var_temp__blk936_dn8 = assign42660_e55915_d_n8;

        let (assign42670_e55925, assign42670_e55925_d_n5, assign42670_e55925_d_n6, assign42670_e55925_d_n7, assign42670_e55925_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 != 0.0)) {
        let assign42670_e55922: f64 = (var_x_s * var_temp__blk936);
        let assign42670_e55923: f64 = (0.7071067811865475 * assign42670_e55922);
        (assign42670_e55923, (0.7071067811865475 * ((var_x_s_dn5 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn5))), (0.7071067811865475 * ((var_x_s_dn6 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn6))), (0.7071067811865475 * ((var_x_s_dn7 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn7))), (0.7071067811865475 * ((var_x_s_dn8 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn8))),)
    } else {
        (var_sqs, var_sqs_dn5, var_sqs_dn6, var_sqs_dn7, var_sqs_dn8,)
    }
};
        var_sqs = assign42670_e55925;
        var_sqs_dn5 = assign42670_e55925_d_n5;
        var_sqs_dn6 = assign42670_e55925_d_n6;
        var_sqs_dn7 = assign42670_e55925_d_n7;
        var_sqs_dn8 = assign42670_e55925_d_n8;

        let (assign42680_e55949, assign42680_e55949_d_n5, assign42680_e55949_d_n6, assign42680_e55949_d_n7, assign42680_e55949_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 != 0.0)) {
        let assign42680_e55935: f64 = (0.5 * var_x_s);
        let assign42680_e55936: f64 = (1.0 - assign42680_e55935);
        let assign42680_e55940: f64 = (var_x_s * var_x_s);
        let assign42680_e55941: f64 = (0.16666666666666666 * assign42680_e55940);
        let assign42680_e55942: f64 = (assign42680_e55936 + assign42680_e55941);
        let assign42680_e55943: f64 = (var_gf * assign42680_e55942);
        let assign42680_e55945: f64 = (assign42680_e55943 / var_temp__blk936);
        let assign42680_e55946: f64 = (0.7071067811865475 * assign42680_e55945);
        let assign42680_e55947: f64 = (1.0 + assign42680_e55946);
        (assign42680_e55947, (0.7071067811865475 * (((((var_gf_dn5 * assign42680_e55942) + (var_gf * ((-(0.5 * var_x_s_dn5)) + (0.16666666666666666 * ((var_x_s_dn5 * var_x_s) + (var_x_s * var_x_s_dn5)))))) * var_temp__blk936) - (assign42680_e55943 * var_temp__blk936_dn5)) / (var_temp__blk936 * var_temp__blk936))), (0.7071067811865475 * (((((var_gf_dn6 * assign42680_e55942) + (var_gf * ((-(0.5 * var_x_s_dn6)) + (0.16666666666666666 * ((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)))))) * var_temp__blk936) - (assign42680_e55943 * var_temp__blk936_dn6)) / (var_temp__blk936 * var_temp__blk936))), (0.7071067811865475 * (((((var_gf_dn7 * assign42680_e55942) + (var_gf * ((-(0.5 * var_x_s_dn7)) + (0.16666666666666666 * ((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)))))) * var_temp__blk936) - (assign42680_e55943 * var_temp__blk936_dn7)) / (var_temp__blk936 * var_temp__blk936))), (0.7071067811865475 * (((((var_gf_dn8 * assign42680_e55942) + (var_gf * ((-(0.5 * var_x_s_dn8)) + (0.16666666666666666 * ((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)))))) * var_temp__blk936) - (assign42680_e55943 * var_temp__blk936_dn8)) / (var_temp__blk936 * var_temp__blk936))),)
    } else {
        (var_alphas, var_alphas_dn5, var_alphas_dn6, var_alphas_dn7, var_alphas_dn8,)
    }
};
        var_alphas = assign42680_e55949;
        var_alphas_dn5 = assign42680_e55949_d_n5;
        var_alphas_dn6 = assign42680_e55949_d_n6;
        var_alphas_dn7 = assign42680_e55949_d_n7;
        var_alphas_dn8 = assign42680_e55949_d_n8;

        let (assign42690_e55960, assign42690_e55960_d_n5, assign42690_e55960_d_n6, assign42690_e55960_d_n7, assign42690_e55960_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 == 0.0)) {
        let assign42690_e55956: f64 = (var_x_s - 1.0);
        let assign42690_e55958: f64 = (assign42690_e55956 + var_es);
        (assign42690_e55958, (var_x_s_dn5 + var_es_dn5), (var_x_s_dn6 + var_es_dn6), (var_x_s_dn7 + var_es_dn7), (var_x_s_dn8 + var_es_dn8),)
    } else {
        (var_ps, var_ps_dn5, var_ps_dn6, var_ps_dn7, var_ps_dn8,)
    }
};
        var_ps = assign42690_e55960;
        var_ps_dn5 = assign42690_e55960_d_n5;
        var_ps_dn6 = assign42690_e55960_d_n6;
        var_ps_dn7 = assign42690_e55960_d_n7;
        var_ps_dn8 = assign42690_e55960_d_n8;

        let (assign42700_e55968, assign42700_e55968_d_n5, assign42700_e55968_d_n6, assign42700_e55968_d_n7, assign42700_e55968_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 == 0.0)) {
        let assign42700_e55966: f64 = (var_ps).sqrt();
        (assign42700_e55966, (var_ps_dn5 / (2.0 * assign42700_e55966)), (var_ps_dn6 / (2.0 * assign42700_e55966)), (var_ps_dn7 / (2.0 * assign42700_e55966)), (var_ps_dn8 / (2.0 * assign42700_e55966)),)
    } else {
        (var_sqs, var_sqs_dn5, var_sqs_dn6, var_sqs_dn7, var_sqs_dn8,)
    }
};
        var_sqs = assign42700_e55968;
        var_sqs_dn5 = assign42700_e55968_d_n5;
        var_sqs_dn6 = assign42700_e55968_d_n6;
        var_sqs_dn7 = assign42700_e55968_d_n7;
        var_sqs_dn8 = assign42700_e55968_d_n8;

        let (assign42710_e55985, assign42710_e55985_d_n5, assign42710_e55985_d_n6, assign42710_e55985_d_n7, assign42710_e55985_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 == 0.0)) {
        let assign42710_e55978: f64 = (1.0 - var_es);
        let assign42710_e55979: f64 = (var_gf * assign42710_e55978);
        let assign42710_e55981: f64 = (assign42710_e55979 / var_sqs);
        let assign42710_e55982: f64 = (0.5 * assign42710_e55981);
        let assign42710_e55983: f64 = (1.0 + assign42710_e55982);
        (assign42710_e55983, (0.5 * (((((var_gf_dn5 * assign42710_e55978) + (var_gf * (-var_es_dn5))) * var_sqs) - (assign42710_e55979 * var_sqs_dn5)) / (var_sqs * var_sqs))), (0.5 * (((((var_gf_dn6 * assign42710_e55978) + (var_gf * (-var_es_dn6))) * var_sqs) - (assign42710_e55979 * var_sqs_dn6)) / (var_sqs * var_sqs))), (0.5 * (((((var_gf_dn7 * assign42710_e55978) + (var_gf * (-var_es_dn7))) * var_sqs) - (assign42710_e55979 * var_sqs_dn7)) / (var_sqs * var_sqs))), (0.5 * (((((var_gf_dn8 * assign42710_e55978) + (var_gf * (-var_es_dn8))) * var_sqs) - (assign42710_e55979 * var_sqs_dn8)) / (var_sqs * var_sqs))),)
    } else {
        (var_alphas, var_alphas_dn5, var_alphas_dn6, var_alphas_dn7, var_alphas_dn8,)
    }
};
        var_alphas = assign42710_e55985;
        var_alphas_dn5 = assign42710_e55985_d_n5;
        var_alphas_dn6 = assign42710_e55985_d_n6;
        var_alphas_dn7 = assign42710_e55985_d_n7;
        var_alphas_dn8 = assign42710_e55985_d_n8;

        let (assign42720_e56001, assign42720_e56001_d_n5, assign42720_e56001_d_n6, assign42720_e56001_d_n7, assign42720_e56001_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42720_e55990: f64 = (0.2 * var_xcor_t);
        let assign42720_e55992: f64 = (assign42720_e55990 * var_vsbx);
        let assign42720_e55993: f64 = (1.0 + assign42720_e55992);
        let assign42720_e55997: f64 = (var_xcor_t * var_vsbx);
        let assign42720_e55998: f64 = (1.0 + assign42720_e55997);
        let assign42720_e55999: f64 = (assign42720_e55993 / assign42720_e55998);
        (assign42720_e55999, ((((assign42720_e55990 * var_vsbx_dn5) * assign42720_e55998) - (assign42720_e55993 * (var_xcor_t * var_vsbx_dn5))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * var_vsbx_dn6) * assign42720_e55998) - (assign42720_e55993 * (var_xcor_t * var_vsbx_dn6))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * var_vsbx_dn7) * assign42720_e55998) - (assign42720_e55993 * (var_xcor_t * var_vsbx_dn7))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * var_vsbx_dn8) * assign42720_e55998) - (assign42720_e55993 * (var_xcor_t * var_vsbx_dn8))) / (assign42720_e55998 * assign42720_e55998)),)
    } else {
        (var_rxcor, var_rxcor_dn5, var_rxcor_dn6, var_rxcor_dn7, var_rxcor_dn8,)
    }
};
        var_rxcor = assign42720_e56001;
        var_rxcor_dn5 = assign42720_e56001_d_n5;
        var_rxcor_dn6 = assign42720_e56001_d_n6;
        var_rxcor_dn7 = assign42720_e56001_d_n7;
        var_rxcor_dn8 = assign42720_e56001_d_n8;

        let assign42730_e56004: f64 = if var_ds > 1e-100 { 1.0 } else { 0.0 };
        var_guard1192 = assign42730_e56004;

        let (assign42740_e56015, assign42740_e56015_d_n5, assign42740_e56015_d_n6, assign42740_e56015_d_n7, assign42740_e56015_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42740_e56011: f64 = (var_ps + var_ds);
        let assign42740_e56012: f64 = (assign42740_e56011).sqrt();
        let assign42740_e56013: f64 = (var_gf * assign42740_e56012);
        (assign42740_e56013, ((var_gf_dn5 * assign42740_e56012) + (var_gf * ((var_ps_dn5 + var_ds_dn5) / (2.0 * assign42740_e56012)))), ((var_gf_dn6 * assign42740_e56012) + (var_gf * ((var_ps_dn6 + var_ds_dn6) / (2.0 * assign42740_e56012)))), ((var_gf_dn7 * assign42740_e56012) + (var_gf * ((var_ps_dn7 + var_ds_dn7) / (2.0 * assign42740_e56012)))), ((var_gf_dn8 * assign42740_e56012) + (var_gf * ((var_ps_dn8 + var_ds_dn8) / (2.0 * assign42740_e56012)))),)
    } else {
        (var_xgs, var_xgs_dn5, var_xgs_dn6, var_xgs_dn7, var_xgs_dn8,)
    }
};
        var_xgs = assign42740_e56015;
        var_xgs_dn5 = assign42740_e56015_d_n5;
        var_xgs_dn6 = assign42740_e56015_d_n6;
        var_xgs_dn7 = assign42740_e56015_d_n7;
        var_xgs_dn8 = assign42740_e56015_d_n8;

        let (assign42750_e56031, assign42750_e56031_d_n5, assign42750_e56031_d_n6, assign42750_e56031_d_n7, assign42750_e56031_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42750_e56021: f64 = (var_gf2 * var_ds);
        let assign42750_e56023: f64 = (assign42750_e56021 * var_phit1);
        let assign42750_e56027: f64 = (var_gf * var_sqs);
        let assign42750_e56028: f64 = (var_xgs + assign42750_e56027);
        let assign42750_e56029: f64 = (assign42750_e56023 / assign42750_e56028);
        (assign42750_e56029, (((((((var_gf2_dn5 * var_ds) + (var_gf2 * var_ds_dn5)) * var_phit1) + (assign42750_e56021 * var_phit1_dn5)) * assign42750_e56028) - (assign42750_e56023 * (var_xgs_dn5 + ((var_gf_dn5 * var_sqs) + (var_gf * var_sqs_dn5))))) / (assign42750_e56028 * assign42750_e56028)), (((((((var_gf2_dn6 * var_ds) + (var_gf2 * var_ds_dn6)) * var_phit1) + (assign42750_e56021 * var_phit1_dn6)) * assign42750_e56028) - (assign42750_e56023 * (var_xgs_dn6 + ((var_gf_dn6 * var_sqs) + (var_gf * var_sqs_dn6))))) / (assign42750_e56028 * assign42750_e56028)), (((((((var_gf2_dn7 * var_ds) + (var_gf2 * var_ds_dn7)) * var_phit1) + (assign42750_e56021 * var_phit1_dn7)) * assign42750_e56028) - (assign42750_e56023 * (var_xgs_dn7 + ((var_gf_dn7 * var_sqs) + (var_gf * var_sqs_dn7))))) / (assign42750_e56028 * assign42750_e56028)), (((((((var_gf2_dn8 * var_ds) + (var_gf2 * var_ds_dn8)) * var_phit1) + (assign42750_e56021 * var_phit1_dn8)) * assign42750_e56028) - (assign42750_e56023 * (var_xgs_dn8 + ((var_gf_dn8 * var_sqs) + (var_gf * var_sqs_dn8))))) / (assign42750_e56028 * assign42750_e56028)),)
    } else {
        (var_qis, var_qis_dn5, var_qis_dn6, var_qis_dn7, var_qis_dn8,)
    }
};
        var_qis = assign42750_e56031;
        var_qis_dn5 = assign42750_e56031_d_n5;
        var_qis_dn6 = assign42750_e56031_d_n6;
        var_qis_dn7 = assign42750_e56031_d_n7;
        var_qis_dn8 = assign42750_e56031_d_n8;

        let (assign42760_e56041, assign42760_e56041_d_n5, assign42760_e56041_d_n6, assign42760_e56041_d_n7, assign42760_e56041_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42760_e56037: f64 = (var_sqs * var_gf);
        let assign42760_e56039: f64 = (assign42760_e56037 * var_phit1);
        (assign42760_e56039, ((((var_sqs_dn5 * var_gf) + (var_sqs * var_gf_dn5)) * var_phit1) + (assign42760_e56037 * var_phit1_dn5)), ((((var_sqs_dn6 * var_gf) + (var_sqs * var_gf_dn6)) * var_phit1) + (assign42760_e56037 * var_phit1_dn6)), ((((var_sqs_dn7 * var_gf) + (var_sqs * var_gf_dn7)) * var_phit1) + (assign42760_e56037 * var_phit1_dn7)), ((((var_sqs_dn8 * var_gf) + (var_sqs * var_gf_dn8)) * var_phit1) + (assign42760_e56037 * var_phit1_dn8)),)
    } else {
        (var_qbs, var_qbs_dn5, var_qbs_dn6, var_qbs_dn7, var_qbs_dn8,)
    }
};
        var_qbs = assign42760_e56041;
        var_qbs_dn5 = assign42760_e56041_d_n5;
        var_qbs_dn6 = assign42760_e56041_d_n6;
        var_qbs_dn7 = assign42760_e56041_d_n7;
        var_qbs_dn8 = assign42760_e56041_d_n8;

        let assign42770_e56044: f64 = if var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1193 = assign42770_e56044;

        let (assign42780_e56058, assign42780_e56058_d_n5, assign42780_e56058_d_n6, assign42780_e56058_d_n7, assign42780_e56058_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1193 != 0.0)) {
        let assign42780_e56054: f64 = (var_rsb_i * var_vsbx);
        let assign42780_e56055: f64 = (1.0 - assign42780_e56054);
        let assign42780_e56056: f64 = (1.0 / assign42780_e56055);
        (assign42780_e56056, (-((-(var_rsb_i * var_vsbx_dn5)) / (assign42780_e56055 * assign42780_e56055))), (-((-(var_rsb_i * var_vsbx_dn6)) / (assign42780_e56055 * assign42780_e56055))), (-((-(var_rsb_i * var_vsbx_dn7)) / (assign42780_e56055 * assign42780_e56055))), (-((-(var_rsb_i * var_vsbx_dn8)) / (assign42780_e56055 * assign42780_e56055))),)
    } else {
        (var_rhob, var_rhob_dn5, var_rhob_dn6, var_rhob_dn7, var_rhob_dn8,)
    }
};
        var_rhob = assign42780_e56058;
        var_rhob_dn5 = assign42780_e56058_d_n5;
        var_rhob_dn6 = assign42780_e56058_d_n6;
        var_rhob_dn7 = assign42780_e56058_d_n7;
        var_rhob_dn8 = assign42780_e56058_d_n8;

        let (assign42790_e56071, assign42790_e56071_d_n5, assign42790_e56071_d_n6, assign42790_e56071_d_n7, assign42790_e56071_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1193 == 0.0)) {
        let assign42790_e56068: f64 = (var_rsb_i * var_vsbx);
        let assign42790_e56069: f64 = (1.0 + assign42790_e56068);
        (assign42790_e56069, (var_rsb_i * var_vsbx_dn5), (var_rsb_i * var_vsbx_dn6), (var_rsb_i * var_vsbx_dn7), (var_rsb_i * var_vsbx_dn8),)
    } else {
        (var_rhob, var_rhob_dn5, var_rhob_dn6, var_rhob_dn7, var_rhob_dn8,)
    }
};
        var_rhob = assign42790_e56071;
        var_rhob_dn5 = assign42790_e56071_d_n5;
        var_rhob_dn6 = assign42790_e56071_d_n6;
        var_rhob_dn7 = assign42790_e56071_d_n7;
        var_rhob_dn8 = assign42790_e56071_d_n8;

        let assign42800_e56074: f64 = if var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1194 = assign42800_e56074;

        let (assign42810_e56086, assign42810_e56086_d_n5, assign42810_e56086_d_n6, assign42810_e56086_d_n7, assign42810_e56086_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1194 != 0.0)) {
        let assign42810_e56083: f64 = (var_rsg_i * var_qis);
        let assign42810_e56084: f64 = (1.0 - assign42810_e56083);
        (assign42810_e56084, (-(var_rsg_i * var_qis_dn5)), (-(var_rsg_i * var_qis_dn6)), (-(var_rsg_i * var_qis_dn7)), (-(var_rsg_i * var_qis_dn8)),)
    } else {
        (var_rhog, var_rhog_dn5, var_rhog_dn6, var_rhog_dn7, var_rhog_dn8,)
    }
};
        var_rhog = assign42810_e56086;
        var_rhog_dn5 = assign42810_e56086_d_n5;
        var_rhog_dn6 = assign42810_e56086_d_n6;
        var_rhog_dn7 = assign42810_e56086_d_n7;
        var_rhog_dn8 = assign42810_e56086_d_n8;

        let (assign42820_e56101, assign42820_e56101_d_n5, assign42820_e56101_d_n6, assign42820_e56101_d_n7, assign42820_e56101_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1194 == 0.0)) {
        let assign42820_e56097: f64 = (var_rsg_i * var_qis);
        let assign42820_e56098: f64 = (1.0 + assign42820_e56097);
        let assign42820_e56099: f64 = (1.0 / assign42820_e56098);
        (assign42820_e56099, (-((var_rsg_i * var_qis_dn5) / (assign42820_e56098 * assign42820_e56098))), (-((var_rsg_i * var_qis_dn6) / (assign42820_e56098 * assign42820_e56098))), (-((var_rsg_i * var_qis_dn7) / (assign42820_e56098 * assign42820_e56098))), (-((var_rsg_i * var_qis_dn8) / (assign42820_e56098 * assign42820_e56098))),)
    } else {
        (var_rhog, var_rhog_dn5, var_rhog_dn6, var_rhog_dn7, var_rhog_dn8,)
    }
};
        var_rhog = assign42820_e56101;
        var_rhog_dn5 = assign42820_e56101_d_n5;
        var_rhog_dn6 = assign42820_e56101_d_n6;
        var_rhog_dn7 = assign42820_e56101_d_n7;
        var_rhog_dn8 = assign42820_e56101_d_n8;

        let (assign42830_e56113, assign42830_e56113_d_n5, assign42830_e56113_d_n6, assign42830_e56113_d_n7, assign42830_e56113_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42830_e56107: f64 = (var_ther_i * var_rhob);
        let assign42830_e56109: f64 = (assign42830_e56107 * var_rhog);
        let assign42830_e56111: f64 = (assign42830_e56109 * var_qis);
        (assign42830_e56111, (((((var_ther_i * var_rhob_dn5) * var_rhog) + (assign42830_e56107 * var_rhog_dn5)) * var_qis) + (assign42830_e56109 * var_qis_dn5)), (((((var_ther_i * var_rhob_dn6) * var_rhog) + (assign42830_e56107 * var_rhog_dn6)) * var_qis) + (assign42830_e56109 * var_qis_dn6)), (((((var_ther_i * var_rhob_dn7) * var_rhog) + (assign42830_e56107 * var_rhog_dn7)) * var_qis) + (assign42830_e56109 * var_qis_dn7)), (((((var_ther_i * var_rhob_dn8) * var_rhog) + (assign42830_e56107 * var_rhog_dn8)) * var_qis) + (assign42830_e56109 * var_qis_dn8)),)
    } else {
        (var_gr, var_gr_dn5, var_gr_dn6, var_gr_dn7, var_gr_dn8,)
    }
};
        var_gr = assign42830_e56113;
        var_gr_dn5 = assign42830_e56113_d_n5;
        var_gr_dn6 = assign42830_e56113_d_n6;
        var_gr_dn7 = assign42830_e56113_d_n7;
        var_gr_dn8 = assign42830_e56113_d_n8;

        let (assign42840_e56125, assign42840_e56125_d_n5, assign42840_e56125_d_n6, assign42840_e56125_d_n7, assign42840_e56125_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42840_e56121: f64 = (var_eta_mu * var_qis);
        let assign42840_e56122: f64 = (var_qbs + assign42840_e56121);
        let assign42840_e56123: f64 = (var_e_eff0 * assign42840_e56122);
        (assign42840_e56123, (var_e_eff0 * (var_qbs_dn5 + (var_eta_mu * var_qis_dn5))), (var_e_eff0 * (var_qbs_dn6 + (var_eta_mu * var_qis_dn6))), (var_e_eff0 * (var_qbs_dn7 + (var_eta_mu * var_qis_dn7))), (var_e_eff0 * (var_qbs_dn8 + (var_eta_mu * var_qis_dn8))),)
    } else {
        (var_eeffs, var_eeffs_dn5, var_eeffs_dn6, var_eeffs_dn7, var_eeffs_dn8,)
    }
};
        var_eeffs = assign42840_e56125;
        var_eeffs_dn5 = assign42840_e56125_d_n5;
        var_eeffs_dn6 = assign42840_e56125_d_n6;
        var_eeffs_dn7 = assign42840_e56125_d_n7;
        var_eeffs_dn8 = assign42840_e56125_d_n8;

        let (assign42850_e56138, assign42850_e56138_d_n5, assign42850_e56138_d_n6, assign42850_e56138_d_n7, assign42850_e56138_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42850_e56132: f64 = (var_ps + var_ds);
        let assign42850_e56134: f64 = (assign42850_e56132 + 1e-14);
        let assign42850_e56135: f64 = (var_ps / assign42850_e56134);
        let assign42850_e56136: f64 = (assign42850_e56135).ln();
        (assign42850_e56136, ((((var_ps_dn5 * assign42850_e56134) - (var_ps * (var_ps_dn5 + var_ds_dn5))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((var_ps_dn6 * assign42850_e56134) - (var_ps * (var_ps_dn6 + var_ds_dn6))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((var_ps_dn7 * assign42850_e56134) - (var_ps * (var_ps_dn7 + var_ds_dn7))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((var_ps_dn8 * assign42850_e56134) - (var_ps * (var_ps_dn8 + var_ds_dn8))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign42850_e56138;
        var_temp1_dn5 = assign42850_e56138_d_n5;
        var_temp1_dn6 = assign42850_e56138_d_n6;
        var_temp1_dn7 = assign42850_e56138_d_n7;
        var_temp1_dn8 = assign42850_e56138_d_n8;

        let (assign42860_e56157, assign42860_e56157_d_n5, assign42860_e56157_d_n6, assign42860_e56157_d_n7, assign42860_e56157_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42860_e56144: f64 = (var_eeffs * var_mue_t);
        let assign42860_e56146: f64 = (assign42860_e56144).powf(var_themu_t);
        let assign42860_e56150: f64 = (0.5 * var_thecs_t);
        let assign42860_e56152: f64 = (assign42860_e56150 * var_temp1);
        let assign42860_e56153: f64 = (assign42860_e56152).exp();
        let assign42860_e56154: f64 = (var_cs_t * assign42860_e56153);
        let assign42860_e56155: f64 = (assign42860_e56146 + assign42860_e56154);
        (assign42860_e56155, (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign42860_e56144).powf(var_themu_t - 1.0) * (var_eeffs_dn5 * var_mue_t))) } } else { (assign42860_e56146 * (var_themu_t * ((var_eeffs_dn5 * var_mue_t) / assign42860_e56144))) } + (var_cs_t * (assign42860_e56153 * (assign42860_e56150 * var_temp1_dn5)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign42860_e56144).powf(var_themu_t - 1.0) * (var_eeffs_dn6 * var_mue_t))) } } else { (assign42860_e56146 * (var_themu_t * ((var_eeffs_dn6 * var_mue_t) / assign42860_e56144))) } + (var_cs_t * (assign42860_e56153 * (assign42860_e56150 * var_temp1_dn6)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign42860_e56144).powf(var_themu_t - 1.0) * (var_eeffs_dn7 * var_mue_t))) } } else { (assign42860_e56146 * (var_themu_t * ((var_eeffs_dn7 * var_mue_t) / assign42860_e56144))) } + (var_cs_t * (assign42860_e56153 * (assign42860_e56150 * var_temp1_dn7)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign42860_e56144).powf(var_themu_t - 1.0) * (var_eeffs_dn8 * var_mue_t))) } } else { (assign42860_e56146 * (var_themu_t * ((var_eeffs_dn8 * var_mue_t) / assign42860_e56144))) } + (var_cs_t * (assign42860_e56153 * (assign42860_e56150 * var_temp1_dn8)))),)
    } else {
        (var_mutmp, var_mutmp_dn5, var_mutmp_dn6, var_mutmp_dn7, var_mutmp_dn8,)
    }
};
        var_mutmp = assign42860_e56157;
        var_mutmp_dn5 = assign42860_e56157_d_n5;
        var_mutmp_dn6 = assign42860_e56157_d_n6;
        var_mutmp_dn7 = assign42860_e56157_d_n7;
        var_mutmp_dn8 = assign42860_e56157_d_n8;

        let (assign42870_e56169, assign42870_e56169_d_n5, assign42870_e56169_d_n6, assign42870_e56169_d_n7, assign42870_e56169_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42870_e56163: f64 = (1.0 + var_mutmp);
        let assign42870_e56165: f64 = (assign42870_e56163 + var_gr);
        let assign42870_e56167: f64 = (assign42870_e56165 * var_rxcor);
        (assign42870_e56167, (((var_mutmp_dn5 + var_gr_dn5) * var_rxcor) + (assign42870_e56165 * var_rxcor_dn5)), (((var_mutmp_dn6 + var_gr_dn6) * var_rxcor) + (assign42870_e56165 * var_rxcor_dn6)), (((var_mutmp_dn7 + var_gr_dn7) * var_rxcor) + (assign42870_e56165 * var_rxcor_dn7)), (((var_mutmp_dn8 + var_gr_dn8) * var_rxcor) + (assign42870_e56165 * var_rxcor_dn8)),)
    } else {
        (var_gmobs, var_gmobs_dn5, var_gmobs_dn6, var_gmobs_dn7, var_gmobs_dn8,)
    }
};
        var_gmobs = assign42870_e56169;
        var_gmobs_dn5 = assign42870_e56169_d_n5;
        var_gmobs_dn6 = assign42870_e56169_d_n6;
        var_gmobs_dn7 = assign42870_e56169_d_n7;
        var_gmobs_dn8 = assign42870_e56169_d_n8;

        let assign42880_e56172: f64 = if var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1195 = assign42880_e56172;

        let (assign42890_e56186, assign42890_e56186_d_n5, assign42890_e56186_d_n6, assign42890_e56186_d_n7, assign42890_e56186_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1195 != 0.0)) {
        let assign42890_e56182: f64 = (var_thesatb_i * var_vsbx);
        let assign42890_e56183: f64 = (1.0 - assign42890_e56182);
        let assign42890_e56184: f64 = (1.0 / assign42890_e56183);
        (assign42890_e56184, (-((-(var_thesatb_i * var_vsbx_dn5)) / (assign42890_e56183 * assign42890_e56183))), (-((-(var_thesatb_i * var_vsbx_dn6)) / (assign42890_e56183 * assign42890_e56183))), (-((-(var_thesatb_i * var_vsbx_dn7)) / (assign42890_e56183 * assign42890_e56183))), (-((-(var_thesatb_i * var_vsbx_dn8)) / (assign42890_e56183 * assign42890_e56183))),)
    } else {
        (var_xitsb, var_xitsb_dn5, var_xitsb_dn6, var_xitsb_dn7, var_xitsb_dn8,)
    }
};
        var_xitsb = assign42890_e56186;
        var_xitsb_dn5 = assign42890_e56186_d_n5;
        var_xitsb_dn6 = assign42890_e56186_d_n6;
        var_xitsb_dn7 = assign42890_e56186_d_n7;
        var_xitsb_dn8 = assign42890_e56186_d_n8;

        let (assign42900_e56199, assign42900_e56199_d_n5, assign42900_e56199_d_n6, assign42900_e56199_d_n7, assign42900_e56199_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1195 == 0.0)) {
        let assign42900_e56196: f64 = (var_thesatb_i * var_vsbx);
        let assign42900_e56197: f64 = (1.0 + assign42900_e56196);
        (assign42900_e56197, (var_thesatb_i * var_vsbx_dn5), (var_thesatb_i * var_vsbx_dn6), (var_thesatb_i * var_vsbx_dn7), (var_thesatb_i * var_vsbx_dn8),)
    } else {
        (var_xitsb, var_xitsb_dn5, var_xitsb_dn6, var_xitsb_dn7, var_xitsb_dn8,)
    }
};
        var_xitsb = assign42900_e56199;
        var_xitsb_dn5 = assign42900_e56199_d_n5;
        var_xitsb_dn6 = assign42900_e56199_d_n6;
        var_xitsb_dn7 = assign42900_e56199_d_n7;
        var_xitsb_dn8 = assign42900_e56199_d_n8;

        let (assign42910_e56207, assign42910_e56207_d_n5, assign42910_e56207_d_n6, assign42910_e56207_d_n7, assign42910_e56207_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42910_e56205: f64 = (var_qis * var_xitsb);
        (assign42910_e56205, ((var_qis_dn5 * var_xitsb) + (var_qis * var_xitsb_dn5)), ((var_qis_dn6 * var_xitsb) + (var_qis * var_xitsb_dn6)), ((var_qis_dn7 * var_xitsb) + (var_qis * var_xitsb_dn7)), ((var_qis_dn8 * var_xitsb) + (var_qis * var_xitsb_dn8)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign42910_e56207;
        var_temp2_dn5 = assign42910_e56207_d_n5;
        var_temp2_dn6 = assign42910_e56207_d_n6;
        var_temp2_dn7 = assign42910_e56207_d_n7;
        var_temp2_dn8 = assign42910_e56207_d_n8;

        let (assign42920_e56217, assign42920_e56217_d_n5, assign42920_e56217_d_n6, assign42920_e56217_d_n7, assign42920_e56217_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42920_e56214: f64 = (var_thesatt_i + var_temp2);
        let assign42920_e56215: f64 = (var_temp2 / assign42920_e56214);
        (assign42920_e56215, (((var_temp2_dn5 * assign42920_e56214) - (var_temp2 * var_temp2_dn5)) / (assign42920_e56214 * assign42920_e56214)), (((var_temp2_dn6 * assign42920_e56214) - (var_temp2 * var_temp2_dn6)) / (assign42920_e56214 * assign42920_e56214)), (((var_temp2_dn7 * assign42920_e56214) - (var_temp2 * var_temp2_dn7)) / (assign42920_e56214 * assign42920_e56214)), (((var_temp2_dn8 * assign42920_e56214) - (var_temp2 * var_temp2_dn8)) / (assign42920_e56214 * assign42920_e56214)),)
    } else {
        (var_wsat, var_wsat_dn5, var_wsat_dn6, var_wsat_dn7, var_wsat_dn8,)
    }
};
        var_wsat = assign42920_e56217;
        var_wsat_dn5 = assign42920_e56217_d_n5;
        var_wsat_dn6 = assign42920_e56217_d_n6;
        var_wsat_dn7 = assign42920_e56217_d_n7;
        var_wsat_dn8 = assign42920_e56217_d_n8;

        let assign42930_e56220: f64 = if var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1196 = assign42930_e56220;

        let (assign42940_e56234, assign42940_e56234_d_n5, assign42940_e56234_d_n6, assign42940_e56234_d_n7, assign42940_e56234_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1196 != 0.0)) {
        let assign42940_e56230: f64 = (var_thesatg_i * var_wsat);
        let assign42940_e56231: f64 = (1.0 - assign42940_e56230);
        let assign42940_e56232: f64 = (1.0 / assign42940_e56231);
        (assign42940_e56232, (-((-(var_thesatg_i * var_wsat_dn5)) / (assign42940_e56231 * assign42940_e56231))), (-((-(var_thesatg_i * var_wsat_dn6)) / (assign42940_e56231 * assign42940_e56231))), (-((-(var_thesatg_i * var_wsat_dn7)) / (assign42940_e56231 * assign42940_e56231))), (-((-(var_thesatg_i * var_wsat_dn8)) / (assign42940_e56231 * assign42940_e56231))),)
    } else {
        (var_factheta, var_factheta_dn5, var_factheta_dn6, var_factheta_dn7, var_factheta_dn8,)
    }
};
        var_factheta = assign42940_e56234;
        var_factheta_dn5 = assign42940_e56234_d_n5;
        var_factheta_dn6 = assign42940_e56234_d_n6;
        var_factheta_dn7 = assign42940_e56234_d_n7;
        var_factheta_dn8 = assign42940_e56234_d_n8;

        let (assign42950_e56247, assign42950_e56247_d_n5, assign42950_e56247_d_n6, assign42950_e56247_d_n7, assign42950_e56247_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1196 == 0.0)) {
        let assign42950_e56244: f64 = (var_thesatg_i * var_wsat);
        let assign42950_e56245: f64 = (1.0 + assign42950_e56244);
        (assign42950_e56245, (var_thesatg_i * var_wsat_dn5), (var_thesatg_i * var_wsat_dn6), (var_thesatg_i * var_wsat_dn7), (var_thesatg_i * var_wsat_dn8),)
    } else {
        (var_factheta, var_factheta_dn5, var_factheta_dn6, var_factheta_dn7, var_factheta_dn8,)
    }
};
        var_factheta = assign42950_e56247;
        var_factheta_dn5 = assign42950_e56247_d_n5;
        var_factheta_dn6 = assign42950_e56247_d_n6;
        var_factheta_dn7 = assign42950_e56247_d_n7;
        var_factheta_dn8 = assign42950_e56247_d_n8;

        var_vgb1_dc = var_vgb1;
        var_vgb1_dc_dn5 = var_vgb1_dn5;
        var_vgb1_dc_dn6 = var_vgb1_dn6;
        var_vgb1_dc_dn7 = var_vgb1_dn7;
        var_vgb1_dc_dn8 = var_vgb1_dn8;

        var_vsbx_dc = var_vsbx;
        var_vsbx_dc_dn5 = var_vsbx_dn5;
        var_vsbx_dc_dn6 = var_vsbx_dn6;
        var_vsbx_dc_dn7 = var_vsbx_dn7;
        var_vsbx_dc_dn8 = var_vsbx_dn8;

        var_phit1_dc = var_phit1;
        var_phit1_dc_dn5 = var_phit1_dn5;
        var_phit1_dc_dn6 = var_phit1_dn6;
        var_phit1_dc_dn7 = var_phit1_dn7;
        var_phit1_dc_dn8 = var_phit1_dn8;

        var_inv_phit1_dc = var_inv_phit1;
        var_inv_phit1_dc_dn5 = var_inv_phit1_dn5;
        var_inv_phit1_dc_dn6 = var_inv_phit1_dn6;
        var_inv_phit1_dc_dn7 = var_inv_phit1_dn7;
        var_inv_phit1_dc_dn8 = var_inv_phit1_dn8;

        var_gf_dc = var_gf;
        var_gf_dc_dn5 = var_gf_dn5;
        var_gf_dc_dn6 = var_gf_dn6;
        var_gf_dc_dn7 = var_gf_dn7;
        var_gf_dc_dn8 = var_gf_dn8;

        var_gf2_dc = var_gf2;
        var_gf2_dc_dn5 = var_gf2_dn5;
        var_gf2_dc_dn6 = var_gf2_dn6;
        var_gf2_dc_dn7 = var_gf2_dn7;
        var_gf2_dc_dn8 = var_gf2_dn8;

        var_inv_gf2_dc = var_inv_gf2;
        var_inv_gf2_dc_dn5 = var_inv_gf2_dn5;
        var_inv_gf2_dc_dn6 = var_inv_gf2_dn6;
        var_inv_gf2_dc_dn7 = var_inv_gf2_dn7;
        var_inv_gf2_dc_dn8 = var_inv_gf2_dn8;

        *var_alphas_slot = var_alphas;
        *var_alphas_dn5_slot = var_alphas_dn5;
        *var_alphas_dn6_slot = var_alphas_dn6;
        *var_alphas_dn7_slot = var_alphas_dn7;
        *var_alphas_dn8_slot = var_alphas_dn8;
        *var_ds_slot = var_ds;
        *var_ds_dn5_slot = var_ds_dn5;
        *var_ds_dn6_slot = var_ds_dn6;
        *var_ds_dn7_slot = var_ds_dn7;
        *var_ds_dn8_slot = var_ds_dn8;
        *var_eeffs_slot = var_eeffs;
        *var_eeffs_dn5_slot = var_eeffs_dn5;
        *var_eeffs_dn6_slot = var_eeffs_dn6;
        *var_eeffs_dn7_slot = var_eeffs_dn7;
        *var_eeffs_dn8_slot = var_eeffs_dn8;
        *var_factheta_slot = var_factheta;
        *var_factheta_dn5_slot = var_factheta_dn5;
        *var_factheta_dn6_slot = var_factheta_dn6;
        *var_factheta_dn7_slot = var_factheta_dn7;
        *var_factheta_dn8_slot = var_factheta_dn8;
        *var_gf2_dc_slot = var_gf2_dc;
        *var_gf2_dc_dn5_slot = var_gf2_dc_dn5;
        *var_gf2_dc_dn6_slot = var_gf2_dc_dn6;
        *var_gf2_dc_dn7_slot = var_gf2_dc_dn7;
        *var_gf2_dc_dn8_slot = var_gf2_dc_dn8;
        *var_gf_dc_slot = var_gf_dc;
        *var_gf_dc_dn5_slot = var_gf_dc_dn5;
        *var_gf_dc_dn6_slot = var_gf_dc_dn6;
        *var_gf_dc_dn7_slot = var_gf_dc_dn7;
        *var_gf_dc_dn8_slot = var_gf_dc_dn8;
        *var_gmobs_slot = var_gmobs;
        *var_gmobs_dn5_slot = var_gmobs_dn5;
        *var_gmobs_dn6_slot = var_gmobs_dn6;
        *var_gmobs_dn7_slot = var_gmobs_dn7;
        *var_gmobs_dn8_slot = var_gmobs_dn8;
        *var_gr_slot = var_gr;
        *var_gr_dn5_slot = var_gr_dn5;
        *var_gr_dn6_slot = var_gr_dn6;
        *var_gr_dn7_slot = var_gr_dn7;
        *var_gr_dn8_slot = var_gr_dn8;
        *var_guard1192_slot = var_guard1192;
        *var_guard1193_slot = var_guard1193;
        *var_guard1194_slot = var_guard1194;
        *var_guard1195_slot = var_guard1195;
        *var_guard1196_slot = var_guard1196;
        *var_inv_gf2_dc_slot = var_inv_gf2_dc;
        *var_inv_gf2_dc_dn5_slot = var_inv_gf2_dc_dn5;
        *var_inv_gf2_dc_dn6_slot = var_inv_gf2_dc_dn6;
        *var_inv_gf2_dc_dn7_slot = var_inv_gf2_dc_dn7;
        *var_inv_gf2_dc_dn8_slot = var_inv_gf2_dc_dn8;
        *var_inv_phit1_dc_slot = var_inv_phit1_dc;
        *var_inv_phit1_dc_dn5_slot = var_inv_phit1_dc_dn5;
        *var_inv_phit1_dc_dn6_slot = var_inv_phit1_dc_dn6;
        *var_inv_phit1_dc_dn7_slot = var_inv_phit1_dc_dn7;
        *var_inv_phit1_dc_dn8_slot = var_inv_phit1_dc_dn8;
        *var_mutmp_slot = var_mutmp;
        *var_mutmp_dn5_slot = var_mutmp_dn5;
        *var_mutmp_dn6_slot = var_mutmp_dn6;
        *var_mutmp_dn7_slot = var_mutmp_dn7;
        *var_mutmp_dn8_slot = var_mutmp_dn8;
        *var_phit1_dc_slot = var_phit1_dc;
        *var_phit1_dc_dn5_slot = var_phit1_dc_dn5;
        *var_phit1_dc_dn6_slot = var_phit1_dc_dn6;
        *var_phit1_dc_dn7_slot = var_phit1_dc_dn7;
        *var_phit1_dc_dn8_slot = var_phit1_dc_dn8;
        *var_ps_slot = var_ps;
        *var_ps_dn5_slot = var_ps_dn5;
        *var_ps_dn6_slot = var_ps_dn6;
        *var_ps_dn7_slot = var_ps_dn7;
        *var_ps_dn8_slot = var_ps_dn8;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn5_slot = var_qbs_dn5;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbs_dn8_slot = var_qbs_dn8;
        *var_qis_slot = var_qis;
        *var_qis_dn5_slot = var_qis_dn5;
        *var_qis_dn6_slot = var_qis_dn6;
        *var_qis_dn7_slot = var_qis_dn7;
        *var_qis_dn8_slot = var_qis_dn8;
        *var_rhob_slot = var_rhob;
        *var_rhob_dn5_slot = var_rhob_dn5;
        *var_rhob_dn6_slot = var_rhob_dn6;
        *var_rhob_dn7_slot = var_rhob_dn7;
        *var_rhob_dn8_slot = var_rhob_dn8;
        *var_rhog_slot = var_rhog;
        *var_rhog_dn5_slot = var_rhog_dn5;
        *var_rhog_dn6_slot = var_rhog_dn6;
        *var_rhog_dn7_slot = var_rhog_dn7;
        *var_rhog_dn8_slot = var_rhog_dn8;
        *var_rxcor_slot = var_rxcor;
        *var_rxcor_dn5_slot = var_rxcor_dn5;
        *var_rxcor_dn6_slot = var_rxcor_dn6;
        *var_rxcor_dn7_slot = var_rxcor_dn7;
        *var_rxcor_dn8_slot = var_rxcor_dn8;
        *var_sqs_slot = var_sqs;
        *var_sqs_dn5_slot = var_sqs_dn5;
        *var_sqs_dn6_slot = var_sqs_dn6;
        *var_sqs_dn7_slot = var_sqs_dn7;
        *var_sqs_dn8_slot = var_sqs_dn8;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_vgb1_dc_slot = var_vgb1_dc;
        *var_vgb1_dc_dn5_slot = var_vgb1_dc_dn5;
        *var_vgb1_dc_dn6_slot = var_vgb1_dc_dn6;
        *var_vgb1_dc_dn7_slot = var_vgb1_dc_dn7;
        *var_vgb1_dc_dn8_slot = var_vgb1_dc_dn8;
        *var_vsbx_dc_slot = var_vsbx_dc;
        *var_vsbx_dc_dn5_slot = var_vsbx_dc_dn5;
        *var_vsbx_dc_dn6_slot = var_vsbx_dc_dn6;
        *var_vsbx_dc_dn7_slot = var_vsbx_dc_dn7;
        *var_vsbx_dc_dn8_slot = var_vsbx_dc_dn8;
        *var_wsat_slot = var_wsat;
        *var_wsat_dn5_slot = var_wsat_dn5;
        *var_wsat_dn6_slot = var_wsat_dn6;
        *var_wsat_dn7_slot = var_wsat_dn7;
        *var_wsat_dn8_slot = var_wsat_dn8;
        *var_xgs_slot = var_xgs;
        *var_xgs_dn5_slot = var_xgs_dn5;
        *var_xgs_dn6_slot = var_xgs_dn6;
        *var_xgs_dn7_slot = var_xgs_dn7;
        *var_xgs_dn8_slot = var_xgs_dn8;
        *var_xitsb_slot = var_xitsb;
        *var_xitsb_dn5_slot = var_xitsb_dn5;
        *var_xitsb_dn6_slot = var_xitsb_dn6;
        *var_xitsb_dn7_slot = var_xitsb_dn7;
        *var_xitsb_dn8_slot = var_xitsb_dn8;
    }

    pub(super) fn stamp_transient_block_93(
        var_alphas: f64,
        var_alphas_dn5: f64,
        var_alphas_dn6: f64,
        var_alphas_dn7: f64,
        var_alphas_dn8: f64,
        var_cs_t: f64,
        var_delta_1s: f64,
        var_delta_1s_dn5: f64,
        var_delta_1s_dn6: f64,
        var_delta_1s_dn7: f64,
        var_delta_1s_dn8: f64,
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_ds: f64,
        var_ds_dn5: f64,
        var_ds_dn6: f64,
        var_ds_dn7: f64,
        var_ds_dn8: f64,
        var_es: f64,
        var_es_dn5: f64,
        var_es_dn6: f64,
        var_es_dn7: f64,
        var_es_dn8: f64,
        var_factheta: f64,
        var_factheta_dn5: f64,
        var_factheta_dn6: f64,
        var_factheta_dn7: f64,
        var_factheta_dn8: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gmobs: f64,
        var_gmobs_dn5: f64,
        var_gmobs_dn6: f64,
        var_gmobs_dn7: f64,
        var_gmobs_dn8: f64,
        var_inv_phit1: f64,
        var_inv_phit1_dn5: f64,
        var_inv_phit1_dn6: f64,
        var_inv_phit1_dn7: f64,
        var_inv_phit1_dn8: f64,
        var_inv_xi: f64,
        var_inv_xi_dn5: f64,
        var_inv_xi_dn6: f64,
        var_inv_xi_dn7: f64,
        var_inv_xi_dn8: f64,
        var_margin: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_ps: f64,
        var_ps_dn5: f64,
        var_ps_dn6: f64,
        var_ps_dn7: f64,
        var_ps_dn8: f64,
        var_qbs: f64,
        var_qbs_dn5: f64,
        var_qbs_dn6: f64,
        var_qbs_dn7: f64,
        var_qbs_dn8: f64,
        var_qis: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_rhob: f64,
        var_rhob_dn5: f64,
        var_rhob_dn6: f64,
        var_rhob_dn7: f64,
        var_rhob_dn8: f64,
        var_rhog: f64,
        var_rhog_dn5: f64,
        var_rhog_dn6: f64,
        var_rhog_dn7: f64,
        var_rhog_dn8: f64,
        var_rxcor: f64,
        var_rxcor_dn5: f64,
        var_rxcor_dn6: f64,
        var_rxcor_dn7: f64,
        var_rxcor_dn8: f64,
        var_sp_s_x1: f64,
        var_sp_s_x1_dn5: f64,
        var_sp_s_x1_dn6: f64,
        var_sp_s_x1_dn7: f64,
        var_sp_s_x1_dn8: f64,
        var_sqs: f64,
        var_sqs_dn5: f64,
        var_sqs_dn6: f64,
        var_sqs_dn7: f64,
        var_sqs_dn8: f64,
        var_thecs_t: f64,
        var_thesatloc: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_x_s: f64,
        var_x_s_dn5: f64,
        var_x_s_dn6: f64,
        var_x_s_dn7: f64,
        var_x_s_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xgs: f64,
        var_xgs_dn5: f64,
        var_xgs_dn6: f64,
        var_xgs_dn7: f64,
        var_xgs_dn8: f64,
        var_xi: f64,
        var_xi1s: f64,
        var_xi1s_dn5: f64,
        var_xi1s_dn6: f64,
        var_xi1s_dn7: f64,
        var_xi1s_dn8: f64,
        var_xi2s: f64,
        var_xi2s_dn5: f64,
        var_xi2s_dn6: f64,
        var_xi2s_dn7: f64,
        var_xi2s_dn8: f64,
        var_xi_dn5: f64,
        var_xi_dn6: f64,
        var_xi_dn7: f64,
        var_xi_dn8: f64,
        var_xitsb: f64,
        var_xitsb_dn5: f64,
        var_xitsb_dn6: f64,
        var_xitsb_dn7: f64,
        var_xitsb_dn8: f64,
        var_xn_s: f64,
        var_xn_s_dn5: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_xno_s: f64,
        var_xno_s_dn5: f64,
        var_xno_s_dn6: f64,
        var_xno_s_dn7: f64,
        var_xno_s_dn8: f64,
        var_alpha_slot: &mut f64,
        var_alpha_dn5_slot: &mut f64,
        var_alpha_dn6_slot: &mut f64,
        var_alpha_dn7_slot: &mut f64,
        var_alpha_dn8_slot: &mut f64,
        var_alphas_dc_slot: &mut f64,
        var_alphas_dc_dn5_slot: &mut f64,
        var_alphas_dc_dn6_slot: &mut f64,
        var_alphas_dc_dn7_slot: &mut f64,
        var_alphas_dc_dn8_slot: &mut f64,
        var_asat_slot: &mut f64,
        var_asat_dn5_slot: &mut f64,
        var_asat_dn6_slot: &mut f64,
        var_asat_dn7_slot: &mut f64,
        var_asat_dn8_slot: &mut f64,
        var_dd_slot: &mut f64,
        var_dd_dn5_slot: &mut f64,
        var_dd_dn6_slot: &mut f64,
        var_dd_dn7_slot: &mut f64,
        var_dd_dn8_slot: &mut f64,
        var_delta_1s_dc_slot: &mut f64,
        var_delta_1s_dc_dn5_slot: &mut f64,
        var_delta_1s_dc_dn6_slot: &mut f64,
        var_delta_1s_dc_dn7_slot: &mut f64,
        var_delta_1s_dc_dn8_slot: &mut f64,
        var_delta_ns_dc_slot: &mut f64,
        var_delta_ns_dc_dn5_slot: &mut f64,
        var_delta_ns_dc_dn6_slot: &mut f64,
        var_delta_ns_dc_dn7_slot: &mut f64,
        var_delta_ns_dc_dn8_slot: &mut f64,
        var_dm_slot: &mut f64,
        var_dm_dn5_slot: &mut f64,
        var_dm_dn6_slot: &mut f64,
        var_dm_dn7_slot: &mut f64,
        var_dm_dn8_slot: &mut f64,
        var_dps_slot: &mut f64,
        var_dps_dn5_slot: &mut f64,
        var_dps_dn6_slot: &mut f64,
        var_dps_dn7_slot: &mut f64,
        var_dps_dn8_slot: &mut f64,
        var_ds_dc_slot: &mut f64,
        var_ds_dc_dn5_slot: &mut f64,
        var_ds_dc_dn6_slot: &mut f64,
        var_ds_dc_dn7_slot: &mut f64,
        var_ds_dc_dn8_slot: &mut f64,
        var_ed_slot: &mut f64,
        var_ed_dn5_slot: &mut f64,
        var_ed_dn6_slot: &mut f64,
        var_ed_dn7_slot: &mut f64,
        var_ed_dn8_slot: &mut f64,
        var_em_slot: &mut f64,
        var_em_dn5_slot: &mut f64,
        var_em_dn6_slot: &mut f64,
        var_em_dn7_slot: &mut f64,
        var_em_dn8_slot: &mut f64,
        var_es_dc_slot: &mut f64,
        var_es_dc_dn5_slot: &mut f64,
        var_es_dc_dn6_slot: &mut f64,
        var_es_dc_dn7_slot: &mut f64,
        var_es_dc_dn8_slot: &mut f64,
        var_eta_p_slot: &mut f64,
        var_eta_p_dn5_slot: &mut f64,
        var_eta_p_dn6_slot: &mut f64,
        var_eta_p_dn7_slot: &mut f64,
        var_eta_p_dn8_slot: &mut f64,
        var_factheta_dc_slot: &mut f64,
        var_factheta_dc_dn5_slot: &mut f64,
        var_factheta_dc_dn6_slot: &mut f64,
        var_factheta_dc_dn7_slot: &mut f64,
        var_factheta_dc_dn8_slot: &mut f64,
        var_gmob_slot: &mut f64,
        var_gmob_dn5_slot: &mut f64,
        var_gmob_dn6_slot: &mut f64,
        var_gmob_dn7_slot: &mut f64,
        var_gmob_dn8_slot: &mut f64,
        var_gmobs_dc_slot: &mut f64,
        var_gmobs_dc_dn5_slot: &mut f64,
        var_gmobs_dc_dn6_slot: &mut f64,
        var_gmobs_dc_dn7_slot: &mut f64,
        var_gmobs_dc_dn8_slot: &mut f64,
        var_guard1197_slot: &mut f64,
        var_guard1198_slot: &mut f64,
        var_guard1199_slot: &mut f64,
        var_guard1200_slot: &mut f64,
        var_guard1201_slot: &mut f64,
        var_inv_xi_dc_slot: &mut f64,
        var_inv_xi_dc_dn5_slot: &mut f64,
        var_inv_xi_dc_dn6_slot: &mut f64,
        var_inv_xi_dc_dn7_slot: &mut f64,
        var_inv_xi_dc_dn8_slot: &mut f64,
        var_margin_dc_slot: &mut f64,
        var_midphi0_slot: &mut f64,
        var_midphi0_dn5_slot: &mut f64,
        var_midphi0_dn6_slot: &mut f64,
        var_midphi0_dn7_slot: &mut f64,
        var_midphi0_dn8_slot: &mut f64,
        var_pd_slot: &mut f64,
        var_pd_dn5_slot: &mut f64,
        var_pd_dn6_slot: &mut f64,
        var_pd_dn7_slot: &mut f64,
        var_pd_dn8_slot: &mut f64,
        var_pm_slot: &mut f64,
        var_pm_dn5_slot: &mut f64,
        var_pm_dn6_slot: &mut f64,
        var_pm_dn7_slot: &mut f64,
        var_pm_dn8_slot: &mut f64,
        var_ps_dc_slot: &mut f64,
        var_ps_dc_dn5_slot: &mut f64,
        var_ps_dc_dn6_slot: &mut f64,
        var_ps_dc_dn7_slot: &mut f64,
        var_ps_dc_dn8_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn5_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbd_dn8_slot: &mut f64,
        var_qbm_slot: &mut f64,
        var_qbm_dn5_slot: &mut f64,
        var_qbm_dn6_slot: &mut f64,
        var_qbm_dn7_slot: &mut f64,
        var_qbm_dn8_slot: &mut f64,
        var_qbs_dc_slot: &mut f64,
        var_qbs_dc_dn5_slot: &mut f64,
        var_qbs_dc_dn6_slot: &mut f64,
        var_qbs_dc_dn7_slot: &mut f64,
        var_qbs_dc_dn8_slot: &mut f64,
        var_qeff1_slot: &mut f64,
        var_qeff1_dn5_slot: &mut f64,
        var_qeff1_dn6_slot: &mut f64,
        var_qeff1_dn7_slot: &mut f64,
        var_qeff1_dn8_slot: &mut f64,
        var_qim_slot: &mut f64,
        var_qim1_slot: &mut f64,
        var_qim1_dn5_slot: &mut f64,
        var_qim1_dn6_slot: &mut f64,
        var_qim1_dn7_slot: &mut f64,
        var_qim1_dn8_slot: &mut f64,
        var_qim_dn5_slot: &mut f64,
        var_qim_dn6_slot: &mut f64,
        var_qim_dn7_slot: &mut f64,
        var_qim_dn8_slot: &mut f64,
        var_qis_dc_slot: &mut f64,
        var_qis_dc_dn5_slot: &mut f64,
        var_qis_dc_dn6_slot: &mut f64,
        var_qis_dc_dn7_slot: &mut f64,
        var_qis_dc_dn8_slot: &mut f64,
        var_rhob_dc_slot: &mut f64,
        var_rhob_dc_dn5_slot: &mut f64,
        var_rhob_dc_dn6_slot: &mut f64,
        var_rhob_dc_dn7_slot: &mut f64,
        var_rhob_dc_dn8_slot: &mut f64,
        var_rhog_dc_slot: &mut f64,
        var_rhog_dc_dn5_slot: &mut f64,
        var_rhog_dc_dn6_slot: &mut f64,
        var_rhog_dc_dn7_slot: &mut f64,
        var_rhog_dc_dn8_slot: &mut f64,
        var_rxcor_dc_slot: &mut f64,
        var_rxcor_dc_dn5_slot: &mut f64,
        var_rxcor_dc_dn6_slot: &mut f64,
        var_rxcor_dc_dn7_slot: &mut f64,
        var_rxcor_dc_dn8_slot: &mut f64,
        var_s1_slot: &mut f64,
        var_s1_dn5_slot: &mut f64,
        var_s1_dn6_slot: &mut f64,
        var_s1_dn7_slot: &mut f64,
        var_s1_dn8_slot: &mut f64,
        var_sp_s_x1_dc_slot: &mut f64,
        var_sp_s_x1_dc_dn5_slot: &mut f64,
        var_sp_s_x1_dc_dn6_slot: &mut f64,
        var_sp_s_x1_dc_dn7_slot: &mut f64,
        var_sp_s_x1_dc_dn8_slot: &mut f64,
        var_sqm_slot: &mut f64,
        var_sqm_dn5_slot: &mut f64,
        var_sqm_dn6_slot: &mut f64,
        var_sqm_dn7_slot: &mut f64,
        var_sqm_dn8_slot: &mut f64,
        var_sqs_dc_slot: &mut f64,
        var_sqs_dc_dn5_slot: &mut f64,
        var_sqs_dc_dn6_slot: &mut f64,
        var_sqs_dc_dn7_slot: &mut f64,
        var_sqs_dc_dn8_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_thesat1_slot: &mut f64,
        var_thesat1_dn5_slot: &mut f64,
        var_thesat1_dn6_slot: &mut f64,
        var_thesat1_dn7_slot: &mut f64,
        var_thesat1_dn8_slot: &mut f64,
        var_thesateff_slot: &mut f64,
        var_thesateff_dn5_slot: &mut f64,
        var_thesateff_dn6_slot: &mut f64,
        var_thesateff_dn7_slot: &mut f64,
        var_thesateff_dn8_slot: &mut f64,
        var_udse_slot: &mut f64,
        var_udse_dn5_slot: &mut f64,
        var_udse_dn6_slot: &mut f64,
        var_udse_dn7_slot: &mut f64,
        var_udse_dn8_slot: &mut f64,
        var_v_dsat_slot: &mut f64,
        var_v_dsat_dn5_slot: &mut f64,
        var_v_dsat_dn6_slot: &mut f64,
        var_v_dsat_dn7_slot: &mut f64,
        var_v_dsat_dn8_slot: &mut f64,
        var_vdsat_lim_slot: &mut f64,
        var_vdsat_lim_dn5_slot: &mut f64,
        var_vdsat_lim_dn6_slot: &mut f64,
        var_vdsat_lim_dn7_slot: &mut f64,
        var_vdsat_lim_dn8_slot: &mut f64,
        var_vdse_slot: &mut f64,
        var_vdse_dn5_slot: &mut f64,
        var_vdse_dn6_slot: &mut f64,
        var_vdse_dn7_slot: &mut f64,
        var_vdse_dn8_slot: &mut f64,
        var_voxm_slot: &mut f64,
        var_voxm_dn5_slot: &mut f64,
        var_voxm_dn6_slot: &mut f64,
        var_voxm_dn7_slot: &mut f64,
        var_voxm_dn8_slot: &mut f64,
        var_x_d_slot: &mut f64,
        var_x_d_dn5_slot: &mut f64,
        var_x_d_dn6_slot: &mut f64,
        var_x_d_dn7_slot: &mut f64,
        var_x_d_dn8_slot: &mut f64,
        var_x_ds_slot: &mut f64,
        var_x_ds_dn5_slot: &mut f64,
        var_x_ds_dn6_slot: &mut f64,
        var_x_ds_dn7_slot: &mut f64,
        var_x_ds_dn8_slot: &mut f64,
        var_x_inf0_slot: &mut f64,
        var_x_inf0_dn5_slot: &mut f64,
        var_x_inf0_dn6_slot: &mut f64,
        var_x_inf0_dn7_slot: &mut f64,
        var_x_inf0_dn8_slot: &mut f64,
        var_x_m_slot: &mut f64,
        var_x_m_dn5_slot: &mut f64,
        var_x_m_dn6_slot: &mut f64,
        var_x_m_dn7_slot: &mut f64,
        var_x_m_dn8_slot: &mut f64,
        var_x_s_dc_slot: &mut f64,
        var_x_s_dc_dn5_slot: &mut f64,
        var_x_s_dc_dn6_slot: &mut f64,
        var_x_s_dc_dn7_slot: &mut f64,
        var_x_s_dc_dn8_slot: &mut f64,
        var_xg_dc_slot: &mut f64,
        var_xg_dc_dn5_slot: &mut f64,
        var_xg_dc_dn6_slot: &mut f64,
        var_xg_dc_dn7_slot: &mut f64,
        var_xg_dc_dn8_slot: &mut f64,
        var_xgm_slot: &mut f64,
        var_xgm_dn5_slot: &mut f64,
        var_xgm_dn6_slot: &mut f64,
        var_xgm_dn7_slot: &mut f64,
        var_xgm_dn8_slot: &mut f64,
        var_xgs_dc_slot: &mut f64,
        var_xgs_dc_dn5_slot: &mut f64,
        var_xgs_dc_dn6_slot: &mut f64,
        var_xgs_dc_dn7_slot: &mut f64,
        var_xgs_dc_dn8_slot: &mut f64,
        var_xi1s_dc_slot: &mut f64,
        var_xi1s_dc_dn5_slot: &mut f64,
        var_xi1s_dc_dn6_slot: &mut f64,
        var_xi1s_dc_dn7_slot: &mut f64,
        var_xi1s_dc_dn8_slot: &mut f64,
        var_xi2s_dc_slot: &mut f64,
        var_xi2s_dc_dn5_slot: &mut f64,
        var_xi2s_dc_dn6_slot: &mut f64,
        var_xi2s_dc_dn7_slot: &mut f64,
        var_xi2s_dc_dn8_slot: &mut f64,
        var_xi_dc_slot: &mut f64,
        var_xi_dc_dn5_slot: &mut f64,
        var_xi_dc_dn6_slot: &mut f64,
        var_xi_dc_dn7_slot: &mut f64,
        var_xi_dc_dn8_slot: &mut f64,
        var_xitsb_dc_slot: &mut f64,
        var_xitsb_dc_dn5_slot: &mut f64,
        var_xitsb_dc_dn6_slot: &mut f64,
        var_xitsb_dc_dn7_slot: &mut f64,
        var_xitsb_dc_dn8_slot: &mut f64,
        var_xn_s_dc_slot: &mut f64,
        var_xn_s_dc_dn5_slot: &mut f64,
        var_xn_s_dc_dn6_slot: &mut f64,
        var_xn_s_dc_dn7_slot: &mut f64,
        var_xn_s_dc_dn8_slot: &mut f64,
        var_xno_s_dc_slot: &mut f64,
        var_xno_s_dc_dn5_slot: &mut f64,
        var_xno_s_dc_dn6_slot: &mut f64,
        var_xno_s_dc_dn7_slot: &mut f64,
        var_xno_s_dc_dn8_slot: &mut f64,
    ) {
        let mut var_alpha: f64 = *var_alpha_slot;
        let mut var_alpha_dn5: f64 = *var_alpha_dn5_slot;
        let mut var_alpha_dn6: f64 = *var_alpha_dn6_slot;
        let mut var_alpha_dn7: f64 = *var_alpha_dn7_slot;
        let mut var_alpha_dn8: f64 = *var_alpha_dn8_slot;
        let mut var_alphas_dc: f64 = *var_alphas_dc_slot;
        let mut var_alphas_dc_dn5: f64 = *var_alphas_dc_dn5_slot;
        let mut var_alphas_dc_dn6: f64 = *var_alphas_dc_dn6_slot;
        let mut var_alphas_dc_dn7: f64 = *var_alphas_dc_dn7_slot;
        let mut var_alphas_dc_dn8: f64 = *var_alphas_dc_dn8_slot;
        let mut var_asat: f64 = *var_asat_slot;
        let mut var_asat_dn5: f64 = *var_asat_dn5_slot;
        let mut var_asat_dn6: f64 = *var_asat_dn6_slot;
        let mut var_asat_dn7: f64 = *var_asat_dn7_slot;
        let mut var_asat_dn8: f64 = *var_asat_dn8_slot;
        let mut var_dd: f64 = *var_dd_slot;
        let mut var_dd_dn5: f64 = *var_dd_dn5_slot;
        let mut var_dd_dn6: f64 = *var_dd_dn6_slot;
        let mut var_dd_dn7: f64 = *var_dd_dn7_slot;
        let mut var_dd_dn8: f64 = *var_dd_dn8_slot;
        let mut var_delta_1s_dc: f64 = *var_delta_1s_dc_slot;
        let mut var_delta_1s_dc_dn5: f64 = *var_delta_1s_dc_dn5_slot;
        let mut var_delta_1s_dc_dn6: f64 = *var_delta_1s_dc_dn6_slot;
        let mut var_delta_1s_dc_dn7: f64 = *var_delta_1s_dc_dn7_slot;
        let mut var_delta_1s_dc_dn8: f64 = *var_delta_1s_dc_dn8_slot;
        let mut var_delta_ns_dc: f64 = *var_delta_ns_dc_slot;
        let mut var_delta_ns_dc_dn5: f64 = *var_delta_ns_dc_dn5_slot;
        let mut var_delta_ns_dc_dn6: f64 = *var_delta_ns_dc_dn6_slot;
        let mut var_delta_ns_dc_dn7: f64 = *var_delta_ns_dc_dn7_slot;
        let mut var_delta_ns_dc_dn8: f64 = *var_delta_ns_dc_dn8_slot;
        let mut var_dm: f64 = *var_dm_slot;
        let mut var_dm_dn5: f64 = *var_dm_dn5_slot;
        let mut var_dm_dn6: f64 = *var_dm_dn6_slot;
        let mut var_dm_dn7: f64 = *var_dm_dn7_slot;
        let mut var_dm_dn8: f64 = *var_dm_dn8_slot;
        let mut var_dps: f64 = *var_dps_slot;
        let mut var_dps_dn5: f64 = *var_dps_dn5_slot;
        let mut var_dps_dn6: f64 = *var_dps_dn6_slot;
        let mut var_dps_dn7: f64 = *var_dps_dn7_slot;
        let mut var_dps_dn8: f64 = *var_dps_dn8_slot;
        let mut var_ds_dc: f64 = *var_ds_dc_slot;
        let mut var_ds_dc_dn5: f64 = *var_ds_dc_dn5_slot;
        let mut var_ds_dc_dn6: f64 = *var_ds_dc_dn6_slot;
        let mut var_ds_dc_dn7: f64 = *var_ds_dc_dn7_slot;
        let mut var_ds_dc_dn8: f64 = *var_ds_dc_dn8_slot;
        let mut var_ed: f64 = *var_ed_slot;
        let mut var_ed_dn5: f64 = *var_ed_dn5_slot;
        let mut var_ed_dn6: f64 = *var_ed_dn6_slot;
        let mut var_ed_dn7: f64 = *var_ed_dn7_slot;
        let mut var_ed_dn8: f64 = *var_ed_dn8_slot;
        let mut var_em: f64 = *var_em_slot;
        let mut var_em_dn5: f64 = *var_em_dn5_slot;
        let mut var_em_dn6: f64 = *var_em_dn6_slot;
        let mut var_em_dn7: f64 = *var_em_dn7_slot;
        let mut var_em_dn8: f64 = *var_em_dn8_slot;
        let mut var_es_dc: f64 = *var_es_dc_slot;
        let mut var_es_dc_dn5: f64 = *var_es_dc_dn5_slot;
        let mut var_es_dc_dn6: f64 = *var_es_dc_dn6_slot;
        let mut var_es_dc_dn7: f64 = *var_es_dc_dn7_slot;
        let mut var_es_dc_dn8: f64 = *var_es_dc_dn8_slot;
        let mut var_eta_p: f64 = *var_eta_p_slot;
        let mut var_eta_p_dn5: f64 = *var_eta_p_dn5_slot;
        let mut var_eta_p_dn6: f64 = *var_eta_p_dn6_slot;
        let mut var_eta_p_dn7: f64 = *var_eta_p_dn7_slot;
        let mut var_eta_p_dn8: f64 = *var_eta_p_dn8_slot;
        let mut var_factheta_dc: f64 = *var_factheta_dc_slot;
        let mut var_factheta_dc_dn5: f64 = *var_factheta_dc_dn5_slot;
        let mut var_factheta_dc_dn6: f64 = *var_factheta_dc_dn6_slot;
        let mut var_factheta_dc_dn7: f64 = *var_factheta_dc_dn7_slot;
        let mut var_factheta_dc_dn8: f64 = *var_factheta_dc_dn8_slot;
        let mut var_gmob: f64 = *var_gmob_slot;
        let mut var_gmob_dn5: f64 = *var_gmob_dn5_slot;
        let mut var_gmob_dn6: f64 = *var_gmob_dn6_slot;
        let mut var_gmob_dn7: f64 = *var_gmob_dn7_slot;
        let mut var_gmob_dn8: f64 = *var_gmob_dn8_slot;
        let mut var_gmobs_dc: f64 = *var_gmobs_dc_slot;
        let mut var_gmobs_dc_dn5: f64 = *var_gmobs_dc_dn5_slot;
        let mut var_gmobs_dc_dn6: f64 = *var_gmobs_dc_dn6_slot;
        let mut var_gmobs_dc_dn7: f64 = *var_gmobs_dc_dn7_slot;
        let mut var_gmobs_dc_dn8: f64 = *var_gmobs_dc_dn8_slot;
        let mut var_guard1197: f64 = *var_guard1197_slot;
        let mut var_guard1198: f64 = *var_guard1198_slot;
        let mut var_guard1199: f64 = *var_guard1199_slot;
        let mut var_guard1200: f64 = *var_guard1200_slot;
        let mut var_guard1201: f64 = *var_guard1201_slot;
        let mut var_inv_xi_dc: f64 = *var_inv_xi_dc_slot;
        let mut var_inv_xi_dc_dn5: f64 = *var_inv_xi_dc_dn5_slot;
        let mut var_inv_xi_dc_dn6: f64 = *var_inv_xi_dc_dn6_slot;
        let mut var_inv_xi_dc_dn7: f64 = *var_inv_xi_dc_dn7_slot;
        let mut var_inv_xi_dc_dn8: f64 = *var_inv_xi_dc_dn8_slot;
        let mut var_margin_dc: f64 = *var_margin_dc_slot;
        let mut var_midphi0: f64 = *var_midphi0_slot;
        let mut var_midphi0_dn5: f64 = *var_midphi0_dn5_slot;
        let mut var_midphi0_dn6: f64 = *var_midphi0_dn6_slot;
        let mut var_midphi0_dn7: f64 = *var_midphi0_dn7_slot;
        let mut var_midphi0_dn8: f64 = *var_midphi0_dn8_slot;
        let mut var_pd: f64 = *var_pd_slot;
        let mut var_pd_dn5: f64 = *var_pd_dn5_slot;
        let mut var_pd_dn6: f64 = *var_pd_dn6_slot;
        let mut var_pd_dn7: f64 = *var_pd_dn7_slot;
        let mut var_pd_dn8: f64 = *var_pd_dn8_slot;
        let mut var_pm: f64 = *var_pm_slot;
        let mut var_pm_dn5: f64 = *var_pm_dn5_slot;
        let mut var_pm_dn6: f64 = *var_pm_dn6_slot;
        let mut var_pm_dn7: f64 = *var_pm_dn7_slot;
        let mut var_pm_dn8: f64 = *var_pm_dn8_slot;
        let mut var_ps_dc: f64 = *var_ps_dc_slot;
        let mut var_ps_dc_dn5: f64 = *var_ps_dc_dn5_slot;
        let mut var_ps_dc_dn6: f64 = *var_ps_dc_dn6_slot;
        let mut var_ps_dc_dn7: f64 = *var_ps_dc_dn7_slot;
        let mut var_ps_dc_dn8: f64 = *var_ps_dc_dn8_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn5: f64 = *var_qbd_dn5_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbd_dn8: f64 = *var_qbd_dn8_slot;
        let mut var_qbm: f64 = *var_qbm_slot;
        let mut var_qbm_dn5: f64 = *var_qbm_dn5_slot;
        let mut var_qbm_dn6: f64 = *var_qbm_dn6_slot;
        let mut var_qbm_dn7: f64 = *var_qbm_dn7_slot;
        let mut var_qbm_dn8: f64 = *var_qbm_dn8_slot;
        let mut var_qbs_dc: f64 = *var_qbs_dc_slot;
        let mut var_qbs_dc_dn5: f64 = *var_qbs_dc_dn5_slot;
        let mut var_qbs_dc_dn6: f64 = *var_qbs_dc_dn6_slot;
        let mut var_qbs_dc_dn7: f64 = *var_qbs_dc_dn7_slot;
        let mut var_qbs_dc_dn8: f64 = *var_qbs_dc_dn8_slot;
        let mut var_qeff1: f64 = *var_qeff1_slot;
        let mut var_qeff1_dn5: f64 = *var_qeff1_dn5_slot;
        let mut var_qeff1_dn6: f64 = *var_qeff1_dn6_slot;
        let mut var_qeff1_dn7: f64 = *var_qeff1_dn7_slot;
        let mut var_qeff1_dn8: f64 = *var_qeff1_dn8_slot;
        let mut var_qim: f64 = *var_qim_slot;
        let mut var_qim1: f64 = *var_qim1_slot;
        let mut var_qim1_dn5: f64 = *var_qim1_dn5_slot;
        let mut var_qim1_dn6: f64 = *var_qim1_dn6_slot;
        let mut var_qim1_dn7: f64 = *var_qim1_dn7_slot;
        let mut var_qim1_dn8: f64 = *var_qim1_dn8_slot;
        let mut var_qim_dn5: f64 = *var_qim_dn5_slot;
        let mut var_qim_dn6: f64 = *var_qim_dn6_slot;
        let mut var_qim_dn7: f64 = *var_qim_dn7_slot;
        let mut var_qim_dn8: f64 = *var_qim_dn8_slot;
        let mut var_qis_dc: f64 = *var_qis_dc_slot;
        let mut var_qis_dc_dn5: f64 = *var_qis_dc_dn5_slot;
        let mut var_qis_dc_dn6: f64 = *var_qis_dc_dn6_slot;
        let mut var_qis_dc_dn7: f64 = *var_qis_dc_dn7_slot;
        let mut var_qis_dc_dn8: f64 = *var_qis_dc_dn8_slot;
        let mut var_rhob_dc: f64 = *var_rhob_dc_slot;
        let mut var_rhob_dc_dn5: f64 = *var_rhob_dc_dn5_slot;
        let mut var_rhob_dc_dn6: f64 = *var_rhob_dc_dn6_slot;
        let mut var_rhob_dc_dn7: f64 = *var_rhob_dc_dn7_slot;
        let mut var_rhob_dc_dn8: f64 = *var_rhob_dc_dn8_slot;
        let mut var_rhog_dc: f64 = *var_rhog_dc_slot;
        let mut var_rhog_dc_dn5: f64 = *var_rhog_dc_dn5_slot;
        let mut var_rhog_dc_dn6: f64 = *var_rhog_dc_dn6_slot;
        let mut var_rhog_dc_dn7: f64 = *var_rhog_dc_dn7_slot;
        let mut var_rhog_dc_dn8: f64 = *var_rhog_dc_dn8_slot;
        let mut var_rxcor_dc: f64 = *var_rxcor_dc_slot;
        let mut var_rxcor_dc_dn5: f64 = *var_rxcor_dc_dn5_slot;
        let mut var_rxcor_dc_dn6: f64 = *var_rxcor_dc_dn6_slot;
        let mut var_rxcor_dc_dn7: f64 = *var_rxcor_dc_dn7_slot;
        let mut var_rxcor_dc_dn8: f64 = *var_rxcor_dc_dn8_slot;
        let mut var_s1: f64 = *var_s1_slot;
        let mut var_s1_dn5: f64 = *var_s1_dn5_slot;
        let mut var_s1_dn6: f64 = *var_s1_dn6_slot;
        let mut var_s1_dn7: f64 = *var_s1_dn7_slot;
        let mut var_s1_dn8: f64 = *var_s1_dn8_slot;
        let mut var_sp_s_x1_dc: f64 = *var_sp_s_x1_dc_slot;
        let mut var_sp_s_x1_dc_dn5: f64 = *var_sp_s_x1_dc_dn5_slot;
        let mut var_sp_s_x1_dc_dn6: f64 = *var_sp_s_x1_dc_dn6_slot;
        let mut var_sp_s_x1_dc_dn7: f64 = *var_sp_s_x1_dc_dn7_slot;
        let mut var_sp_s_x1_dc_dn8: f64 = *var_sp_s_x1_dc_dn8_slot;
        let mut var_sqm: f64 = *var_sqm_slot;
        let mut var_sqm_dn5: f64 = *var_sqm_dn5_slot;
        let mut var_sqm_dn6: f64 = *var_sqm_dn6_slot;
        let mut var_sqm_dn7: f64 = *var_sqm_dn7_slot;
        let mut var_sqm_dn8: f64 = *var_sqm_dn8_slot;
        let mut var_sqs_dc: f64 = *var_sqs_dc_slot;
        let mut var_sqs_dc_dn5: f64 = *var_sqs_dc_dn5_slot;
        let mut var_sqs_dc_dn6: f64 = *var_sqs_dc_dn6_slot;
        let mut var_sqs_dc_dn7: f64 = *var_sqs_dc_dn7_slot;
        let mut var_sqs_dc_dn8: f64 = *var_sqs_dc_dn8_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_thesat1: f64 = *var_thesat1_slot;
        let mut var_thesat1_dn5: f64 = *var_thesat1_dn5_slot;
        let mut var_thesat1_dn6: f64 = *var_thesat1_dn6_slot;
        let mut var_thesat1_dn7: f64 = *var_thesat1_dn7_slot;
        let mut var_thesat1_dn8: f64 = *var_thesat1_dn8_slot;
        let mut var_thesateff: f64 = *var_thesateff_slot;
        let mut var_thesateff_dn5: f64 = *var_thesateff_dn5_slot;
        let mut var_thesateff_dn6: f64 = *var_thesateff_dn6_slot;
        let mut var_thesateff_dn7: f64 = *var_thesateff_dn7_slot;
        let mut var_thesateff_dn8: f64 = *var_thesateff_dn8_slot;
        let mut var_udse: f64 = *var_udse_slot;
        let mut var_udse_dn5: f64 = *var_udse_dn5_slot;
        let mut var_udse_dn6: f64 = *var_udse_dn6_slot;
        let mut var_udse_dn7: f64 = *var_udse_dn7_slot;
        let mut var_udse_dn8: f64 = *var_udse_dn8_slot;
        let mut var_v_dsat: f64 = *var_v_dsat_slot;
        let mut var_v_dsat_dn5: f64 = *var_v_dsat_dn5_slot;
        let mut var_v_dsat_dn6: f64 = *var_v_dsat_dn6_slot;
        let mut var_v_dsat_dn7: f64 = *var_v_dsat_dn7_slot;
        let mut var_v_dsat_dn8: f64 = *var_v_dsat_dn8_slot;
        let mut var_vdsat_lim: f64 = *var_vdsat_lim_slot;
        let mut var_vdsat_lim_dn5: f64 = *var_vdsat_lim_dn5_slot;
        let mut var_vdsat_lim_dn6: f64 = *var_vdsat_lim_dn6_slot;
        let mut var_vdsat_lim_dn7: f64 = *var_vdsat_lim_dn7_slot;
        let mut var_vdsat_lim_dn8: f64 = *var_vdsat_lim_dn8_slot;
        let mut var_vdse: f64 = *var_vdse_slot;
        let mut var_vdse_dn5: f64 = *var_vdse_dn5_slot;
        let mut var_vdse_dn6: f64 = *var_vdse_dn6_slot;
        let mut var_vdse_dn7: f64 = *var_vdse_dn7_slot;
        let mut var_vdse_dn8: f64 = *var_vdse_dn8_slot;
        let mut var_voxm: f64 = *var_voxm_slot;
        let mut var_voxm_dn5: f64 = *var_voxm_dn5_slot;
        let mut var_voxm_dn6: f64 = *var_voxm_dn6_slot;
        let mut var_voxm_dn7: f64 = *var_voxm_dn7_slot;
        let mut var_voxm_dn8: f64 = *var_voxm_dn8_slot;
        let mut var_x_d: f64 = *var_x_d_slot;
        let mut var_x_d_dn5: f64 = *var_x_d_dn5_slot;
        let mut var_x_d_dn6: f64 = *var_x_d_dn6_slot;
        let mut var_x_d_dn7: f64 = *var_x_d_dn7_slot;
        let mut var_x_d_dn8: f64 = *var_x_d_dn8_slot;
        let mut var_x_ds: f64 = *var_x_ds_slot;
        let mut var_x_ds_dn5: f64 = *var_x_ds_dn5_slot;
        let mut var_x_ds_dn6: f64 = *var_x_ds_dn6_slot;
        let mut var_x_ds_dn7: f64 = *var_x_ds_dn7_slot;
        let mut var_x_ds_dn8: f64 = *var_x_ds_dn8_slot;
        let mut var_x_inf0: f64 = *var_x_inf0_slot;
        let mut var_x_inf0_dn5: f64 = *var_x_inf0_dn5_slot;
        let mut var_x_inf0_dn6: f64 = *var_x_inf0_dn6_slot;
        let mut var_x_inf0_dn7: f64 = *var_x_inf0_dn7_slot;
        let mut var_x_inf0_dn8: f64 = *var_x_inf0_dn8_slot;
        let mut var_x_m: f64 = *var_x_m_slot;
        let mut var_x_m_dn5: f64 = *var_x_m_dn5_slot;
        let mut var_x_m_dn6: f64 = *var_x_m_dn6_slot;
        let mut var_x_m_dn7: f64 = *var_x_m_dn7_slot;
        let mut var_x_m_dn8: f64 = *var_x_m_dn8_slot;
        let mut var_x_s_dc: f64 = *var_x_s_dc_slot;
        let mut var_x_s_dc_dn5: f64 = *var_x_s_dc_dn5_slot;
        let mut var_x_s_dc_dn6: f64 = *var_x_s_dc_dn6_slot;
        let mut var_x_s_dc_dn7: f64 = *var_x_s_dc_dn7_slot;
        let mut var_x_s_dc_dn8: f64 = *var_x_s_dc_dn8_slot;
        let mut var_xg_dc: f64 = *var_xg_dc_slot;
        let mut var_xg_dc_dn5: f64 = *var_xg_dc_dn5_slot;
        let mut var_xg_dc_dn6: f64 = *var_xg_dc_dn6_slot;
        let mut var_xg_dc_dn7: f64 = *var_xg_dc_dn7_slot;
        let mut var_xg_dc_dn8: f64 = *var_xg_dc_dn8_slot;
        let mut var_xgm: f64 = *var_xgm_slot;
        let mut var_xgm_dn5: f64 = *var_xgm_dn5_slot;
        let mut var_xgm_dn6: f64 = *var_xgm_dn6_slot;
        let mut var_xgm_dn7: f64 = *var_xgm_dn7_slot;
        let mut var_xgm_dn8: f64 = *var_xgm_dn8_slot;
        let mut var_xgs_dc: f64 = *var_xgs_dc_slot;
        let mut var_xgs_dc_dn5: f64 = *var_xgs_dc_dn5_slot;
        let mut var_xgs_dc_dn6: f64 = *var_xgs_dc_dn6_slot;
        let mut var_xgs_dc_dn7: f64 = *var_xgs_dc_dn7_slot;
        let mut var_xgs_dc_dn8: f64 = *var_xgs_dc_dn8_slot;
        let mut var_xi1s_dc: f64 = *var_xi1s_dc_slot;
        let mut var_xi1s_dc_dn5: f64 = *var_xi1s_dc_dn5_slot;
        let mut var_xi1s_dc_dn6: f64 = *var_xi1s_dc_dn6_slot;
        let mut var_xi1s_dc_dn7: f64 = *var_xi1s_dc_dn7_slot;
        let mut var_xi1s_dc_dn8: f64 = *var_xi1s_dc_dn8_slot;
        let mut var_xi2s_dc: f64 = *var_xi2s_dc_slot;
        let mut var_xi2s_dc_dn5: f64 = *var_xi2s_dc_dn5_slot;
        let mut var_xi2s_dc_dn6: f64 = *var_xi2s_dc_dn6_slot;
        let mut var_xi2s_dc_dn7: f64 = *var_xi2s_dc_dn7_slot;
        let mut var_xi2s_dc_dn8: f64 = *var_xi2s_dc_dn8_slot;
        let mut var_xi_dc: f64 = *var_xi_dc_slot;
        let mut var_xi_dc_dn5: f64 = *var_xi_dc_dn5_slot;
        let mut var_xi_dc_dn6: f64 = *var_xi_dc_dn6_slot;
        let mut var_xi_dc_dn7: f64 = *var_xi_dc_dn7_slot;
        let mut var_xi_dc_dn8: f64 = *var_xi_dc_dn8_slot;
        let mut var_xitsb_dc: f64 = *var_xitsb_dc_slot;
        let mut var_xitsb_dc_dn5: f64 = *var_xitsb_dc_dn5_slot;
        let mut var_xitsb_dc_dn6: f64 = *var_xitsb_dc_dn6_slot;
        let mut var_xitsb_dc_dn7: f64 = *var_xitsb_dc_dn7_slot;
        let mut var_xitsb_dc_dn8: f64 = *var_xitsb_dc_dn8_slot;
        let mut var_xn_s_dc: f64 = *var_xn_s_dc_slot;
        let mut var_xn_s_dc_dn5: f64 = *var_xn_s_dc_dn5_slot;
        let mut var_xn_s_dc_dn6: f64 = *var_xn_s_dc_dn6_slot;
        let mut var_xn_s_dc_dn7: f64 = *var_xn_s_dc_dn7_slot;
        let mut var_xn_s_dc_dn8: f64 = *var_xn_s_dc_dn8_slot;
        let mut var_xno_s_dc: f64 = *var_xno_s_dc_slot;
        let mut var_xno_s_dc_dn5: f64 = *var_xno_s_dc_dn5_slot;
        let mut var_xno_s_dc_dn6: f64 = *var_xno_s_dc_dn6_slot;
        let mut var_xno_s_dc_dn7: f64 = *var_xno_s_dc_dn7_slot;
        let mut var_xno_s_dc_dn8: f64 = *var_xno_s_dc_dn8_slot;

        var_xg_dc = var_xg;
        var_xg_dc_dn5 = var_xg_dn5;
        var_xg_dc_dn6 = var_xg_dn6;
        var_xg_dc_dn7 = var_xg_dn7;
        var_xg_dc_dn8 = var_xg_dn8;

        var_xno_s_dc = var_xno_s;
        var_xno_s_dc_dn5 = var_xno_s_dn5;
        var_xno_s_dc_dn6 = var_xno_s_dn6;
        var_xno_s_dc_dn7 = var_xno_s_dn7;
        var_xno_s_dc_dn8 = var_xno_s_dn8;

        var_xn_s_dc = var_xn_s;
        var_xn_s_dc_dn5 = var_xn_s_dn5;
        var_xn_s_dc_dn6 = var_xn_s_dn6;
        var_xn_s_dc_dn7 = var_xn_s_dn7;
        var_xn_s_dc_dn8 = var_xn_s_dn8;

        var_xi_dc = var_xi;
        var_xi_dc_dn5 = var_xi_dn5;
        var_xi_dc_dn6 = var_xi_dn6;
        var_xi_dc_dn7 = var_xi_dn7;
        var_xi_dc_dn8 = var_xi_dn8;

        var_margin_dc = var_margin;

        var_inv_xi_dc = var_inv_xi;
        var_inv_xi_dc_dn5 = var_inv_xi_dn5;
        var_inv_xi_dc_dn6 = var_inv_xi_dn6;
        var_inv_xi_dc_dn7 = var_inv_xi_dn7;
        var_inv_xi_dc_dn8 = var_inv_xi_dn8;

        var_sp_s_x1_dc = var_sp_s_x1;
        var_sp_s_x1_dc_dn5 = var_sp_s_x1_dn5;
        var_sp_s_x1_dc_dn6 = var_sp_s_x1_dn6;
        var_sp_s_x1_dc_dn7 = var_sp_s_x1_dn7;
        var_sp_s_x1_dc_dn8 = var_sp_s_x1_dn8;

        var_delta_ns_dc = var_delta_ns;
        var_delta_ns_dc_dn5 = var_delta_ns_dn5;
        var_delta_ns_dc_dn6 = var_delta_ns_dn6;
        var_delta_ns_dc_dn7 = var_delta_ns_dn7;
        var_delta_ns_dc_dn8 = var_delta_ns_dn8;

        var_x_s_dc = var_x_s;
        var_x_s_dc_dn5 = var_x_s_dn5;
        var_x_s_dc_dn6 = var_x_s_dn6;
        var_x_s_dc_dn7 = var_x_s_dn7;
        var_x_s_dc_dn8 = var_x_s_dn8;

        var_xi1s_dc = var_xi1s;
        var_xi1s_dc_dn5 = var_xi1s_dn5;
        var_xi1s_dc_dn6 = var_xi1s_dn6;
        var_xi1s_dc_dn7 = var_xi1s_dn7;
        var_xi1s_dc_dn8 = var_xi1s_dn8;

        var_xi2s_dc = var_xi2s;
        var_xi2s_dc_dn5 = var_xi2s_dn5;
        var_xi2s_dc_dn6 = var_xi2s_dn6;
        var_xi2s_dc_dn7 = var_xi2s_dn7;
        var_xi2s_dc_dn8 = var_xi2s_dn8;

        var_delta_1s_dc = var_delta_1s;
        var_delta_1s_dc_dn5 = var_delta_1s_dn5;
        var_delta_1s_dc_dn6 = var_delta_1s_dn6;
        var_delta_1s_dc_dn7 = var_delta_1s_dn7;
        var_delta_1s_dc_dn8 = var_delta_1s_dn8;

        var_es_dc = var_es;
        var_es_dc_dn5 = var_es_dn5;
        var_es_dc_dn6 = var_es_dn6;
        var_es_dc_dn7 = var_es_dn7;
        var_es_dc_dn8 = var_es_dn8;

        var_ps_dc = var_ps;
        var_ps_dc_dn5 = var_ps_dn5;
        var_ps_dc_dn6 = var_ps_dn6;
        var_ps_dc_dn7 = var_ps_dn7;
        var_ps_dc_dn8 = var_ps_dn8;

        var_ds_dc = var_ds;
        var_ds_dc_dn5 = var_ds_dn5;
        var_ds_dc_dn6 = var_ds_dn6;
        var_ds_dc_dn7 = var_ds_dn7;
        var_ds_dc_dn8 = var_ds_dn8;

        var_sqs_dc = var_sqs;
        var_sqs_dc_dn5 = var_sqs_dn5;
        var_sqs_dc_dn6 = var_sqs_dn6;
        var_sqs_dc_dn7 = var_sqs_dn7;
        var_sqs_dc_dn8 = var_sqs_dn8;

        var_alphas_dc = var_alphas;
        var_alphas_dc_dn5 = var_alphas_dn5;
        var_alphas_dc_dn6 = var_alphas_dn6;
        var_alphas_dc_dn7 = var_alphas_dn7;
        var_alphas_dc_dn8 = var_alphas_dn8;

        var_rxcor_dc = var_rxcor;
        var_rxcor_dc_dn5 = var_rxcor_dn5;
        var_rxcor_dc_dn6 = var_rxcor_dn6;
        var_rxcor_dc_dn7 = var_rxcor_dn7;
        var_rxcor_dc_dn8 = var_rxcor_dn8;

        var_xgs_dc = var_xgs;
        var_xgs_dc_dn5 = var_xgs_dn5;
        var_xgs_dc_dn6 = var_xgs_dn6;
        var_xgs_dc_dn7 = var_xgs_dn7;
        var_xgs_dc_dn8 = var_xgs_dn8;

        var_qis_dc = var_qis;
        var_qis_dc_dn5 = var_qis_dn5;
        var_qis_dc_dn6 = var_qis_dn6;
        var_qis_dc_dn7 = var_qis_dn7;
        var_qis_dc_dn8 = var_qis_dn8;

        var_qbs_dc = var_qbs;
        var_qbs_dc_dn5 = var_qbs_dn5;
        var_qbs_dc_dn6 = var_qbs_dn6;
        var_qbs_dc_dn7 = var_qbs_dn7;
        var_qbs_dc_dn8 = var_qbs_dn8;

        var_rhob_dc = var_rhob;
        var_rhob_dc_dn5 = var_rhob_dn5;
        var_rhob_dc_dn6 = var_rhob_dn6;
        var_rhob_dc_dn7 = var_rhob_dn7;
        var_rhob_dc_dn8 = var_rhob_dn8;

        var_rhog_dc = var_rhog;
        var_rhog_dc_dn5 = var_rhog_dn5;
        var_rhog_dc_dn6 = var_rhog_dn6;
        var_rhog_dc_dn7 = var_rhog_dn7;
        var_rhog_dc_dn8 = var_rhog_dn8;

        var_gmobs_dc = var_gmobs;
        var_gmobs_dc_dn5 = var_gmobs_dn5;
        var_gmobs_dc_dn6 = var_gmobs_dn6;
        var_gmobs_dc_dn7 = var_gmobs_dn7;
        var_gmobs_dc_dn8 = var_gmobs_dn8;

        var_xitsb_dc = var_xitsb;
        var_xitsb_dc_dn5 = var_xitsb_dn5;
        var_xitsb_dc_dn6 = var_xitsb_dn6;
        var_xitsb_dc_dn7 = var_xitsb_dn7;
        var_xitsb_dc_dn8 = var_xitsb_dn8;

        var_factheta_dc = var_factheta;
        var_factheta_dc_dn5 = var_factheta_dn5;
        var_factheta_dc_dn6 = var_factheta_dn6;
        var_factheta_dc_dn7 = var_factheta_dn7;
        var_factheta_dc_dn8 = var_factheta_dn8;

        var_thesat1 = 0.0;
        var_thesat1_dn5 = 0.0;
        var_thesat1_dn6 = 0.0;
        var_thesat1_dn7 = 0.0;
        var_thesat1_dn8 = 0.0;

        let assign43300_e56284: f64 = (var_phit1 * 4.60517018598809);
        var_vdsat_lim = assign43300_e56284;
        var_vdsat_lim_dn5 = (var_phit1_dn5 * 4.60517018598809);
        var_vdsat_lim_dn6 = (var_phit1_dn6 * 4.60517018598809);
        var_vdsat_lim_dn7 = (var_phit1_dn7 * 4.60517018598809);
        var_vdsat_lim_dn8 = (var_phit1_dn8 * 4.60517018598809);

        var_v_dsat = var_vdsat_lim;
        var_v_dsat_dn5 = var_vdsat_lim_dn5;
        var_v_dsat_dn6 = var_vdsat_lim_dn6;
        var_v_dsat_dn7 = var_vdsat_lim_dn7;
        var_v_dsat_dn8 = var_vdsat_lim_dn8;

        var_vdse = var_v_ds;
        var_vdse_dn5 = 0.0;
        var_vdse_dn6 = var_v_ds_dn6;
        var_vdse_dn7 = var_v_ds_dn7;
        var_vdse_dn8 = 0.0;

        let assign43330_e56289: f64 = (var_v_ds * var_inv_phit1);
        var_udse = assign43330_e56289;
        var_udse_dn5 = (var_v_ds * var_inv_phit1_dn5);
        var_udse_dn6 = ((var_v_ds_dn6 * var_inv_phit1) + (var_v_ds * var_inv_phit1_dn6));
        var_udse_dn7 = ((var_v_ds_dn7 * var_inv_phit1) + (var_v_ds * var_inv_phit1_dn7));
        var_udse_dn8 = (var_v_ds * var_inv_phit1_dn8);

        var_x_d = var_x_s;
        var_x_d_dn5 = var_x_s_dn5;
        var_x_d_dn6 = var_x_s_dn6;
        var_x_d_dn7 = var_x_s_dn7;
        var_x_d_dn8 = var_x_s_dn8;

        var_x_ds = 0.0;
        var_x_ds_dn5 = 0.0;
        var_x_ds_dn6 = 0.0;
        var_x_ds_dn7 = 0.0;
        var_x_ds_dn8 = 0.0;

        var_dps = 0.0;
        var_dps_dn5 = 0.0;
        var_dps_dn6 = 0.0;
        var_dps_dn7 = 0.0;
        var_dps_dn8 = 0.0;

        var_ed = var_es;
        var_ed_dn5 = var_es_dn5;
        var_ed_dn6 = var_es_dn6;
        var_ed_dn7 = var_es_dn7;
        var_ed_dn8 = var_es_dn8;

        var_pd = var_ps;
        var_pd_dn5 = var_ps_dn5;
        var_pd_dn6 = var_ps_dn6;
        var_pd_dn7 = var_ps_dn7;
        var_pd_dn8 = var_ps_dn8;

        var_dd = var_ds;
        var_dd_dn5 = var_ds_dn5;
        var_dd_dn6 = var_ds_dn6;
        var_dd_dn7 = var_ds_dn7;
        var_dd_dn8 = var_ds_dn8;

        var_qbd = var_qbs;
        var_qbd_dn5 = var_qbs_dn5;
        var_qbd_dn6 = var_qbs_dn6;
        var_qbd_dn7 = var_qbs_dn7;
        var_qbd_dn8 = var_qbs_dn8;

        var_x_m = var_x_s;
        var_x_m_dn5 = var_x_s_dn5;
        var_x_m_dn6 = var_x_s_dn6;
        var_x_m_dn7 = var_x_s_dn7;
        var_x_m_dn8 = var_x_s_dn8;

        var_em = var_es;
        var_em_dn5 = var_es_dn5;
        var_em_dn6 = var_es_dn6;
        var_em_dn7 = var_es_dn7;
        var_em_dn8 = var_es_dn8;

        var_dm = var_ds;
        var_dm_dn5 = var_ds_dn5;
        var_dm_dn6 = var_ds_dn6;
        var_dm_dn7 = var_ds_dn7;
        var_dm_dn8 = var_ds_dn8;

        var_pm = var_ps;
        var_pm_dn5 = var_ps_dn5;
        var_pm_dn6 = var_ps_dn6;
        var_pm_dn7 = var_ps_dn7;
        var_pm_dn8 = var_ps_dn8;

        let assign43450_e56303: f64 = (var_xg - var_x_s);
        var_xgm = assign43450_e56303;
        var_xgm_dn5 = (var_xg_dn5 - var_x_s_dn5);
        var_xgm_dn6 = (var_xg_dn6 - var_x_s_dn6);
        var_xgm_dn7 = (var_xg_dn7 - var_x_s_dn7);
        var_xgm_dn8 = (var_xg_dn8 - var_x_s_dn8);

        var_eta_p = 1.0;
        var_eta_p_dn5 = 0.0;
        var_eta_p_dn6 = 0.0;
        var_eta_p_dn7 = 0.0;
        var_eta_p_dn8 = 0.0;

        var_alpha = 1.0;
        var_alpha_dn5 = 0.0;
        var_alpha_dn6 = 0.0;
        var_alpha_dn7 = 0.0;
        var_alpha_dn8 = 0.0;

        var_sqm = 0.0;
        var_sqm_dn5 = 0.0;
        var_sqm_dn6 = 0.0;
        var_sqm_dn7 = 0.0;
        var_sqm_dn8 = 0.0;

        var_qim = var_qis;
        var_qim_dn5 = var_qis_dn5;
        var_qim_dn6 = var_qis_dn6;
        var_qim_dn7 = var_qis_dn7;
        var_qim_dn8 = var_qis_dn8;

        let assign43500_e56310: f64 = (var_xgm * var_phit1);
        var_qeff1 = assign43500_e56310;
        var_qeff1_dn5 = ((var_xgm_dn5 * var_phit1) + (var_xgm * var_phit1_dn5));
        var_qeff1_dn6 = ((var_xgm_dn6 * var_phit1) + (var_xgm * var_phit1_dn6));
        var_qeff1_dn7 = ((var_xgm_dn7 * var_phit1) + (var_xgm * var_phit1_dn7));
        var_qeff1_dn8 = ((var_xgm_dn8 * var_phit1) + (var_xgm * var_phit1_dn8));

        var_qim1 = 0.0;
        var_qim1_dn5 = 0.0;
        var_qim1_dn6 = 0.0;
        var_qim1_dn7 = 0.0;
        var_qim1_dn8 = 0.0;

        var_qbm = var_qbs;
        var_qbm_dn5 = var_qbs_dn5;
        var_qbm_dn6 = var_qbs_dn6;
        var_qbm_dn7 = var_qbs_dn7;
        var_qbm_dn8 = var_qbs_dn8;

        var_s1 = 0.0;
        var_s1_dn5 = 0.0;
        var_s1_dn6 = 0.0;
        var_s1_dn7 = 0.0;
        var_s1_dn8 = 0.0;

        var_gmob = 1.0;
        var_gmob_dn5 = 0.0;
        var_gmob_dn6 = 0.0;
        var_gmob_dn7 = 0.0;
        var_gmob_dn8 = 0.0;

        var_thesateff = var_thesatloc;
        var_thesateff_dn5 = 0.0;
        var_thesateff_dn6 = 0.0;
        var_thesateff_dn7 = 0.0;
        var_thesateff_dn8 = 0.0;

        var_voxm = var_qeff1;
        var_voxm_dn5 = var_qeff1_dn5;
        var_voxm_dn6 = var_qeff1_dn6;
        var_voxm_dn7 = var_qeff1_dn7;
        var_voxm_dn8 = var_qeff1_dn8;

        let assign43570_e56319: f64 = if var_xg > 0.0 { 1.0 } else { 0.0 };
        var_guard1197 = assign43570_e56319;

        let assign43580_e56322: f64 = if var_ds > 1e-100 { 1.0 } else { 0.0 };
        var_guard1198 = assign43580_e56322;

        let (assign43590_e56330, assign43590_e56330_d_n5, assign43590_e56330_d_n6, assign43590_e56330_d_n7, assign43590_e56330_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43590_e56328: f64 = (var_thesatloc * var_factheta);
        (assign43590_e56328, (var_thesatloc * var_factheta_dn5), (var_thesatloc * var_factheta_dn6), (var_thesatloc * var_factheta_dn7), (var_thesatloc * var_factheta_dn8),)
    } else {
        (var_thesateff, var_thesateff_dn5, var_thesateff_dn6, var_thesateff_dn7, var_thesateff_dn8,)
    }
};
        var_thesateff = assign43590_e56330;
        var_thesateff_dn5 = assign43590_e56330_d_n5;
        var_thesateff_dn6 = assign43590_e56330_d_n6;
        var_thesateff_dn7 = assign43590_e56330_d_n7;
        var_thesateff_dn8 = assign43590_e56330_d_n8;

        let (assign43600_e56338, assign43600_e56338_d_n5, assign43600_e56338_d_n6, assign43600_e56338_d_n7, assign43600_e56338_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43600_e56336: f64 = (var_thesateff / var_gmobs);
        (assign43600_e56336, (((var_thesateff_dn5 * var_gmobs) - (var_thesateff * var_gmobs_dn5)) / (var_gmobs * var_gmobs)), (((var_thesateff_dn6 * var_gmobs) - (var_thesateff * var_gmobs_dn6)) / (var_gmobs * var_gmobs)), (((var_thesateff_dn7 * var_gmobs) - (var_thesateff * var_gmobs_dn7)) / (var_gmobs * var_gmobs)), (((var_thesateff_dn8 * var_gmobs) - (var_thesateff * var_gmobs_dn8)) / (var_gmobs * var_gmobs)),)
    } else {
        (var_thesat1, var_thesat1_dn5, var_thesat1_dn6, var_thesat1_dn7, var_thesat1_dn8,)
    }
};
        var_thesat1 = assign43600_e56338;
        var_thesat1_dn5 = assign43600_e56338_d_n5;
        var_thesat1_dn6 = assign43600_e56338_d_n6;
        var_thesat1_dn7 = assign43600_e56338_d_n7;
        var_thesat1_dn8 = assign43600_e56338_d_n8;

        let (assign43610_e56348, assign43610_e56348_d_n5, assign43610_e56348_d_n6, assign43610_e56348_d_n7, assign43610_e56348_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43610_e56345: f64 = (0.5 * var_gf2);
        let assign43610_e56346: f64 = (var_xgs + assign43610_e56345);
        (assign43610_e56346, (var_xgs_dn5 + (0.5 * var_gf2_dn5)), (var_xgs_dn6 + (0.5 * var_gf2_dn6)), (var_xgs_dn7 + (0.5 * var_gf2_dn7)), (var_xgs_dn8 + (0.5 * var_gf2_dn8)),)
    } else {
        (var_asat, var_asat_dn5, var_asat_dn6, var_asat_dn7, var_asat_dn8,)
    }
};
        var_asat = assign43610_e56348;
        var_asat_dn5 = assign43610_e56348_d_n5;
        var_asat_dn6 = assign43610_e56348_d_n6;
        var_asat_dn7 = assign43610_e56348_d_n7;
        var_asat_dn8 = assign43610_e56348_d_n8;

        let (assign43620_e56360, assign43620_e56360_d_n5, assign43620_e56360_d_n6, assign43620_e56360_d_n7, assign43620_e56360_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43620_e56354: f64 = (var_gf2 * var_delta_1s);
        let __rspice_inv_cse_0: f64 = 1.0 / var_asat;
        let assign43620_e56356: f64 = (assign43620_e56354 * __rspice_inv_cse_0);
        let assign43620_e56358: f64 = (assign43620_e56356 * __rspice_inv_cse_0);
        (assign43620_e56358, ((((((((var_gf2_dn5 * var_delta_1s) + (var_gf2 * var_delta_1s_dn5)) * var_asat) - (assign43620_e56354 * var_asat_dn5)) / (var_asat * var_asat)) * var_asat) - (assign43620_e56356 * var_asat_dn5)) / (var_asat * var_asat)), ((((((((var_gf2_dn6 * var_delta_1s) + (var_gf2 * var_delta_1s_dn6)) * var_asat) - (assign43620_e56354 * var_asat_dn6)) / (var_asat * var_asat)) * var_asat) - (assign43620_e56356 * var_asat_dn6)) / (var_asat * var_asat)), ((((((((var_gf2_dn7 * var_delta_1s) + (var_gf2 * var_delta_1s_dn7)) * var_asat) - (assign43620_e56354 * var_asat_dn7)) / (var_asat * var_asat)) * var_asat) - (assign43620_e56356 * var_asat_dn7)) / (var_asat * var_asat)), ((((((((var_gf2_dn8 * var_delta_1s) + (var_gf2 * var_delta_1s_dn8)) * var_asat) - (assign43620_e56354 * var_asat_dn8)) / (var_asat * var_asat)) * var_asat) - (assign43620_e56356 * var_asat_dn8)) / (var_asat * var_asat)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43620_e56360;
        var_temp__blk936_dn5 = assign43620_e56360_d_n5;
        var_temp__blk936_dn6 = assign43620_e56360_d_n6;
        var_temp__blk936_dn7 = assign43620_e56360_d_n7;
        var_temp__blk936_dn8 = assign43620_e56360_d_n8;

        let assign43630_e56363: f64 = if var_temp__blk936 > 0.0001 { 1.0 } else { 0.0 };
        var_guard1199 = assign43630_e56363;

        let (assign43640_e56373, assign43640_e56373_d_n5, assign43640_e56373_d_n6, assign43640_e56373_d_n7, assign43640_e56373_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1199 != 0.0)) {
        let assign43640_e56371: f64 = (1.0 - var_temp__blk936);
        (assign43640_e56371, (-var_temp__blk936_dn5), (-var_temp__blk936_dn6), (-var_temp__blk936_dn7), (-var_temp__blk936_dn8),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign43640_e56373;
        var_temp1_dn5 = assign43640_e56373_d_n5;
        var_temp1_dn6 = assign43640_e56373_d_n6;
        var_temp1_dn7 = assign43640_e56373_d_n7;
        var_temp1_dn8 = assign43640_e56373_d_n8;

        let assign43650_e56376: f64 = if var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        var_guard1200 = assign43650_e56376;

        let (assign43660_e56386, assign43660_e56386_d_n5, assign43660_e56386_d_n6, assign43660_e56386_d_n7, assign43660_e56386_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1199 != 0.0)) && (var_guard1200 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign43660_e56386;
        var_temp2_dn5 = assign43660_e56386_d_n5;
        var_temp2_dn6 = assign43660_e56386_d_n6;
        var_temp2_dn7 = assign43660_e56386_d_n7;
        var_temp2_dn8 = assign43660_e56386_d_n8;

        let (assign43670_e56400, assign43670_e56400_d_n5, assign43670_e56400_d_n6, assign43670_e56400_d_n7, assign43670_e56400_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1199 != 0.0)) && (var_guard1200 == 0.0)) {
        let assign43670_e56397: f64 = (var_temp1).sqrt();
        let assign43670_e56398: f64 = (1.0 - assign43670_e56397);
        (assign43670_e56398, (-(var_temp1_dn5 / (2.0 * assign43670_e56397))), (-(var_temp1_dn6 / (2.0 * assign43670_e56397))), (-(var_temp1_dn7 / (2.0 * assign43670_e56397))), (-(var_temp1_dn8 / (2.0 * assign43670_e56397))),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign43670_e56400;
        var_temp2_dn5 = assign43670_e56400_d_n5;
        var_temp2_dn6 = assign43670_e56400_d_n6;
        var_temp2_dn7 = assign43670_e56400_d_n7;
        var_temp2_dn8 = assign43670_e56400_d_n8;

        let (assign43680_e56411, assign43680_e56411_d_n5, assign43680_e56411_d_n6, assign43680_e56411_d_n7, assign43680_e56411_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1199 == 0.0)) {
        let assign43680_e56409: f64 = (0.5 * var_temp__blk936);
        (assign43680_e56409, (0.5 * var_temp__blk936_dn5), (0.5 * var_temp__blk936_dn6), (0.5 * var_temp__blk936_dn7), (0.5 * var_temp__blk936_dn8),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign43680_e56411;
        var_temp2_dn5 = assign43680_e56411_d_n5;
        var_temp2_dn6 = assign43680_e56411_d_n6;
        var_temp2_dn7 = assign43680_e56411_d_n7;
        var_temp2_dn8 = assign43680_e56411_d_n8;

        let (assign43690_e56419, assign43690_e56419_d_n5, assign43690_e56419_d_n6, assign43690_e56419_d_n7, assign43690_e56419_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43690_e56417: f64 = (var_temp2 * var_asat);
        (assign43690_e56417, ((var_temp2_dn5 * var_asat) + (var_temp2 * var_asat_dn5)), ((var_temp2_dn6 * var_asat) + (var_temp2 * var_asat_dn6)), ((var_temp2_dn7 * var_asat) + (var_temp2 * var_asat_dn7)), ((var_temp2_dn8 * var_asat) + (var_temp2 * var_asat_dn8)),)
    } else {
        (var_x_inf0, var_x_inf0_dn5, var_x_inf0_dn6, var_x_inf0_dn7, var_x_inf0_dn8,)
    }
};
        var_x_inf0 = assign43690_e56419;
        var_x_inf0_dn5 = assign43690_e56419_d_n5;
        var_x_inf0_dn6 = assign43690_e56419_d_n6;
        var_x_inf0_dn7 = assign43690_e56419_d_n7;
        var_x_inf0_dn8 = assign43690_e56419_d_n8;

        let assign43700_e56426: f64 = if ((var_cs_t > 0.0) && (var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        var_guard1201 = assign43700_e56426;

        let (assign43710_e56438, assign43710_e56438_d_n5, assign43710_e56438_d_n6, assign43710_e56438_d_n7, assign43710_e56438_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43710_e56434: f64 = (0.475 * var_phit1);
        let assign43710_e56436: f64 = (assign43710_e56434 * var_x_inf0);
        (assign43710_e56436, (((0.475 * var_phit1_dn5) * var_x_inf0) + (assign43710_e56434 * var_x_inf0_dn5)), (((0.475 * var_phit1_dn6) * var_x_inf0) + (assign43710_e56434 * var_x_inf0_dn6)), (((0.475 * var_phit1_dn7) * var_x_inf0) + (assign43710_e56434 * var_x_inf0_dn7)), (((0.475 * var_phit1_dn8) * var_x_inf0) + (assign43710_e56434 * var_x_inf0_dn8)),)
    } else {
        (var_midphi0, var_midphi0_dn5, var_midphi0_dn6, var_midphi0_dn7, var_midphi0_dn8,)
    }
};
        var_midphi0 = assign43710_e56438;
        var_midphi0_dn5 = assign43710_e56438_d_n5;
        var_midphi0_dn6 = assign43710_e56438_d_n6;
        var_midphi0_dn7 = assign43710_e56438_d_n7;
        var_midphi0_dn8 = assign43710_e56438_d_n8;

        let (assign43720_e56450, assign43720_e56450_d_n5, assign43720_e56450_d_n6, assign43720_e56450_d_n7, assign43720_e56450_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43720_e56447: f64 = (var_alphas * var_midphi0);
        let assign43720_e56448: f64 = (var_qis - assign43720_e56447);
        (assign43720_e56448, (var_qis_dn5 - ((var_alphas_dn5 * var_midphi0) + (var_alphas * var_midphi0_dn5))), (var_qis_dn6 - ((var_alphas_dn6 * var_midphi0) + (var_alphas * var_midphi0_dn6))), (var_qis_dn7 - ((var_alphas_dn7 * var_midphi0) + (var_alphas * var_midphi0_dn7))), (var_qis_dn8 - ((var_alphas_dn8 * var_midphi0) + (var_alphas * var_midphi0_dn8))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43720_e56450;
        var_temp__blk936_dn5 = assign43720_e56450_d_n5;
        var_temp__blk936_dn6 = assign43720_e56450_d_n6;
        var_temp__blk936_dn7 = assign43720_e56450_d_n7;
        var_temp__blk936_dn8 = assign43720_e56450_d_n8;

        *var_alpha_slot = var_alpha;
        *var_alpha_dn5_slot = var_alpha_dn5;
        *var_alpha_dn6_slot = var_alpha_dn6;
        *var_alpha_dn7_slot = var_alpha_dn7;
        *var_alpha_dn8_slot = var_alpha_dn8;
        *var_alphas_dc_slot = var_alphas_dc;
        *var_alphas_dc_dn5_slot = var_alphas_dc_dn5;
        *var_alphas_dc_dn6_slot = var_alphas_dc_dn6;
        *var_alphas_dc_dn7_slot = var_alphas_dc_dn7;
        *var_alphas_dc_dn8_slot = var_alphas_dc_dn8;
        *var_asat_slot = var_asat;
        *var_asat_dn5_slot = var_asat_dn5;
        *var_asat_dn6_slot = var_asat_dn6;
        *var_asat_dn7_slot = var_asat_dn7;
        *var_asat_dn8_slot = var_asat_dn8;
        *var_dd_slot = var_dd;
        *var_dd_dn5_slot = var_dd_dn5;
        *var_dd_dn6_slot = var_dd_dn6;
        *var_dd_dn7_slot = var_dd_dn7;
        *var_dd_dn8_slot = var_dd_dn8;
        *var_delta_1s_dc_slot = var_delta_1s_dc;
        *var_delta_1s_dc_dn5_slot = var_delta_1s_dc_dn5;
        *var_delta_1s_dc_dn6_slot = var_delta_1s_dc_dn6;
        *var_delta_1s_dc_dn7_slot = var_delta_1s_dc_dn7;
        *var_delta_1s_dc_dn8_slot = var_delta_1s_dc_dn8;
        *var_delta_ns_dc_slot = var_delta_ns_dc;
        *var_delta_ns_dc_dn5_slot = var_delta_ns_dc_dn5;
        *var_delta_ns_dc_dn6_slot = var_delta_ns_dc_dn6;
        *var_delta_ns_dc_dn7_slot = var_delta_ns_dc_dn7;
        *var_delta_ns_dc_dn8_slot = var_delta_ns_dc_dn8;
        *var_dm_slot = var_dm;
        *var_dm_dn5_slot = var_dm_dn5;
        *var_dm_dn6_slot = var_dm_dn6;
        *var_dm_dn7_slot = var_dm_dn7;
        *var_dm_dn8_slot = var_dm_dn8;
        *var_dps_slot = var_dps;
        *var_dps_dn5_slot = var_dps_dn5;
        *var_dps_dn6_slot = var_dps_dn6;
        *var_dps_dn7_slot = var_dps_dn7;
        *var_dps_dn8_slot = var_dps_dn8;
        *var_ds_dc_slot = var_ds_dc;
        *var_ds_dc_dn5_slot = var_ds_dc_dn5;
        *var_ds_dc_dn6_slot = var_ds_dc_dn6;
        *var_ds_dc_dn7_slot = var_ds_dc_dn7;
        *var_ds_dc_dn8_slot = var_ds_dc_dn8;
        *var_ed_slot = var_ed;
        *var_ed_dn5_slot = var_ed_dn5;
        *var_ed_dn6_slot = var_ed_dn6;
        *var_ed_dn7_slot = var_ed_dn7;
        *var_ed_dn8_slot = var_ed_dn8;
        *var_em_slot = var_em;
        *var_em_dn5_slot = var_em_dn5;
        *var_em_dn6_slot = var_em_dn6;
        *var_em_dn7_slot = var_em_dn7;
        *var_em_dn8_slot = var_em_dn8;
        *var_es_dc_slot = var_es_dc;
        *var_es_dc_dn5_slot = var_es_dc_dn5;
        *var_es_dc_dn6_slot = var_es_dc_dn6;
        *var_es_dc_dn7_slot = var_es_dc_dn7;
        *var_es_dc_dn8_slot = var_es_dc_dn8;
        *var_eta_p_slot = var_eta_p;
        *var_eta_p_dn5_slot = var_eta_p_dn5;
        *var_eta_p_dn6_slot = var_eta_p_dn6;
        *var_eta_p_dn7_slot = var_eta_p_dn7;
        *var_eta_p_dn8_slot = var_eta_p_dn8;
        *var_factheta_dc_slot = var_factheta_dc;
        *var_factheta_dc_dn5_slot = var_factheta_dc_dn5;
        *var_factheta_dc_dn6_slot = var_factheta_dc_dn6;
        *var_factheta_dc_dn7_slot = var_factheta_dc_dn7;
        *var_factheta_dc_dn8_slot = var_factheta_dc_dn8;
        *var_gmob_slot = var_gmob;
        *var_gmob_dn5_slot = var_gmob_dn5;
        *var_gmob_dn6_slot = var_gmob_dn6;
        *var_gmob_dn7_slot = var_gmob_dn7;
        *var_gmob_dn8_slot = var_gmob_dn8;
        *var_gmobs_dc_slot = var_gmobs_dc;
        *var_gmobs_dc_dn5_slot = var_gmobs_dc_dn5;
        *var_gmobs_dc_dn6_slot = var_gmobs_dc_dn6;
        *var_gmobs_dc_dn7_slot = var_gmobs_dc_dn7;
        *var_gmobs_dc_dn8_slot = var_gmobs_dc_dn8;
        *var_guard1197_slot = var_guard1197;
        *var_guard1198_slot = var_guard1198;
        *var_guard1199_slot = var_guard1199;
        *var_guard1200_slot = var_guard1200;
        *var_guard1201_slot = var_guard1201;
        *var_inv_xi_dc_slot = var_inv_xi_dc;
        *var_inv_xi_dc_dn5_slot = var_inv_xi_dc_dn5;
        *var_inv_xi_dc_dn6_slot = var_inv_xi_dc_dn6;
        *var_inv_xi_dc_dn7_slot = var_inv_xi_dc_dn7;
        *var_inv_xi_dc_dn8_slot = var_inv_xi_dc_dn8;
        *var_margin_dc_slot = var_margin_dc;
        *var_midphi0_slot = var_midphi0;
        *var_midphi0_dn5_slot = var_midphi0_dn5;
        *var_midphi0_dn6_slot = var_midphi0_dn6;
        *var_midphi0_dn7_slot = var_midphi0_dn7;
        *var_midphi0_dn8_slot = var_midphi0_dn8;
        *var_pd_slot = var_pd;
        *var_pd_dn5_slot = var_pd_dn5;
        *var_pd_dn6_slot = var_pd_dn6;
        *var_pd_dn7_slot = var_pd_dn7;
        *var_pd_dn8_slot = var_pd_dn8;
        *var_pm_slot = var_pm;
        *var_pm_dn5_slot = var_pm_dn5;
        *var_pm_dn6_slot = var_pm_dn6;
        *var_pm_dn7_slot = var_pm_dn7;
        *var_pm_dn8_slot = var_pm_dn8;
        *var_ps_dc_slot = var_ps_dc;
        *var_ps_dc_dn5_slot = var_ps_dc_dn5;
        *var_ps_dc_dn6_slot = var_ps_dc_dn6;
        *var_ps_dc_dn7_slot = var_ps_dc_dn7;
        *var_ps_dc_dn8_slot = var_ps_dc_dn8;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn5_slot = var_qbd_dn5;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbd_dn8_slot = var_qbd_dn8;
        *var_qbm_slot = var_qbm;
        *var_qbm_dn5_slot = var_qbm_dn5;
        *var_qbm_dn6_slot = var_qbm_dn6;
        *var_qbm_dn7_slot = var_qbm_dn7;
        *var_qbm_dn8_slot = var_qbm_dn8;
        *var_qbs_dc_slot = var_qbs_dc;
        *var_qbs_dc_dn5_slot = var_qbs_dc_dn5;
        *var_qbs_dc_dn6_slot = var_qbs_dc_dn6;
        *var_qbs_dc_dn7_slot = var_qbs_dc_dn7;
        *var_qbs_dc_dn8_slot = var_qbs_dc_dn8;
        *var_qeff1_slot = var_qeff1;
        *var_qeff1_dn5_slot = var_qeff1_dn5;
        *var_qeff1_dn6_slot = var_qeff1_dn6;
        *var_qeff1_dn7_slot = var_qeff1_dn7;
        *var_qeff1_dn8_slot = var_qeff1_dn8;
        *var_qim_slot = var_qim;
        *var_qim1_slot = var_qim1;
        *var_qim1_dn5_slot = var_qim1_dn5;
        *var_qim1_dn6_slot = var_qim1_dn6;
        *var_qim1_dn7_slot = var_qim1_dn7;
        *var_qim1_dn8_slot = var_qim1_dn8;
        *var_qim_dn5_slot = var_qim_dn5;
        *var_qim_dn6_slot = var_qim_dn6;
        *var_qim_dn7_slot = var_qim_dn7;
        *var_qim_dn8_slot = var_qim_dn8;
        *var_qis_dc_slot = var_qis_dc;
        *var_qis_dc_dn5_slot = var_qis_dc_dn5;
        *var_qis_dc_dn6_slot = var_qis_dc_dn6;
        *var_qis_dc_dn7_slot = var_qis_dc_dn7;
        *var_qis_dc_dn8_slot = var_qis_dc_dn8;
        *var_rhob_dc_slot = var_rhob_dc;
        *var_rhob_dc_dn5_slot = var_rhob_dc_dn5;
        *var_rhob_dc_dn6_slot = var_rhob_dc_dn6;
        *var_rhob_dc_dn7_slot = var_rhob_dc_dn7;
        *var_rhob_dc_dn8_slot = var_rhob_dc_dn8;
        *var_rhog_dc_slot = var_rhog_dc;
        *var_rhog_dc_dn5_slot = var_rhog_dc_dn5;
        *var_rhog_dc_dn6_slot = var_rhog_dc_dn6;
        *var_rhog_dc_dn7_slot = var_rhog_dc_dn7;
        *var_rhog_dc_dn8_slot = var_rhog_dc_dn8;
        *var_rxcor_dc_slot = var_rxcor_dc;
        *var_rxcor_dc_dn5_slot = var_rxcor_dc_dn5;
        *var_rxcor_dc_dn6_slot = var_rxcor_dc_dn6;
        *var_rxcor_dc_dn7_slot = var_rxcor_dc_dn7;
        *var_rxcor_dc_dn8_slot = var_rxcor_dc_dn8;
        *var_s1_slot = var_s1;
        *var_s1_dn5_slot = var_s1_dn5;
        *var_s1_dn6_slot = var_s1_dn6;
        *var_s1_dn7_slot = var_s1_dn7;
        *var_s1_dn8_slot = var_s1_dn8;
        *var_sp_s_x1_dc_slot = var_sp_s_x1_dc;
        *var_sp_s_x1_dc_dn5_slot = var_sp_s_x1_dc_dn5;
        *var_sp_s_x1_dc_dn6_slot = var_sp_s_x1_dc_dn6;
        *var_sp_s_x1_dc_dn7_slot = var_sp_s_x1_dc_dn7;
        *var_sp_s_x1_dc_dn8_slot = var_sp_s_x1_dc_dn8;
        *var_sqm_slot = var_sqm;
        *var_sqm_dn5_slot = var_sqm_dn5;
        *var_sqm_dn6_slot = var_sqm_dn6;
        *var_sqm_dn7_slot = var_sqm_dn7;
        *var_sqm_dn8_slot = var_sqm_dn8;
        *var_sqs_dc_slot = var_sqs_dc;
        *var_sqs_dc_dn5_slot = var_sqs_dc_dn5;
        *var_sqs_dc_dn6_slot = var_sqs_dc_dn6;
        *var_sqs_dc_dn7_slot = var_sqs_dc_dn7;
        *var_sqs_dc_dn8_slot = var_sqs_dc_dn8;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_thesat1_slot = var_thesat1;
        *var_thesat1_dn5_slot = var_thesat1_dn5;
        *var_thesat1_dn6_slot = var_thesat1_dn6;
        *var_thesat1_dn7_slot = var_thesat1_dn7;
        *var_thesat1_dn8_slot = var_thesat1_dn8;
        *var_thesateff_slot = var_thesateff;
        *var_thesateff_dn5_slot = var_thesateff_dn5;
        *var_thesateff_dn6_slot = var_thesateff_dn6;
        *var_thesateff_dn7_slot = var_thesateff_dn7;
        *var_thesateff_dn8_slot = var_thesateff_dn8;
        *var_udse_slot = var_udse;
        *var_udse_dn5_slot = var_udse_dn5;
        *var_udse_dn6_slot = var_udse_dn6;
        *var_udse_dn7_slot = var_udse_dn7;
        *var_udse_dn8_slot = var_udse_dn8;
        *var_v_dsat_slot = var_v_dsat;
        *var_v_dsat_dn5_slot = var_v_dsat_dn5;
        *var_v_dsat_dn6_slot = var_v_dsat_dn6;
        *var_v_dsat_dn7_slot = var_v_dsat_dn7;
        *var_v_dsat_dn8_slot = var_v_dsat_dn8;
        *var_vdsat_lim_slot = var_vdsat_lim;
        *var_vdsat_lim_dn5_slot = var_vdsat_lim_dn5;
        *var_vdsat_lim_dn6_slot = var_vdsat_lim_dn6;
        *var_vdsat_lim_dn7_slot = var_vdsat_lim_dn7;
        *var_vdsat_lim_dn8_slot = var_vdsat_lim_dn8;
        *var_vdse_slot = var_vdse;
        *var_vdse_dn5_slot = var_vdse_dn5;
        *var_vdse_dn6_slot = var_vdse_dn6;
        *var_vdse_dn7_slot = var_vdse_dn7;
        *var_vdse_dn8_slot = var_vdse_dn8;
        *var_voxm_slot = var_voxm;
        *var_voxm_dn5_slot = var_voxm_dn5;
        *var_voxm_dn6_slot = var_voxm_dn6;
        *var_voxm_dn7_slot = var_voxm_dn7;
        *var_voxm_dn8_slot = var_voxm_dn8;
        *var_x_d_slot = var_x_d;
        *var_x_d_dn5_slot = var_x_d_dn5;
        *var_x_d_dn6_slot = var_x_d_dn6;
        *var_x_d_dn7_slot = var_x_d_dn7;
        *var_x_d_dn8_slot = var_x_d_dn8;
        *var_x_ds_slot = var_x_ds;
        *var_x_ds_dn5_slot = var_x_ds_dn5;
        *var_x_ds_dn6_slot = var_x_ds_dn6;
        *var_x_ds_dn7_slot = var_x_ds_dn7;
        *var_x_ds_dn8_slot = var_x_ds_dn8;
        *var_x_inf0_slot = var_x_inf0;
        *var_x_inf0_dn5_slot = var_x_inf0_dn5;
        *var_x_inf0_dn6_slot = var_x_inf0_dn6;
        *var_x_inf0_dn7_slot = var_x_inf0_dn7;
        *var_x_inf0_dn8_slot = var_x_inf0_dn8;
        *var_x_m_slot = var_x_m;
        *var_x_m_dn5_slot = var_x_m_dn5;
        *var_x_m_dn6_slot = var_x_m_dn6;
        *var_x_m_dn7_slot = var_x_m_dn7;
        *var_x_m_dn8_slot = var_x_m_dn8;
        *var_x_s_dc_slot = var_x_s_dc;
        *var_x_s_dc_dn5_slot = var_x_s_dc_dn5;
        *var_x_s_dc_dn6_slot = var_x_s_dc_dn6;
        *var_x_s_dc_dn7_slot = var_x_s_dc_dn7;
        *var_x_s_dc_dn8_slot = var_x_s_dc_dn8;
        *var_xg_dc_slot = var_xg_dc;
        *var_xg_dc_dn5_slot = var_xg_dc_dn5;
        *var_xg_dc_dn6_slot = var_xg_dc_dn6;
        *var_xg_dc_dn7_slot = var_xg_dc_dn7;
        *var_xg_dc_dn8_slot = var_xg_dc_dn8;
        *var_xgm_slot = var_xgm;
        *var_xgm_dn5_slot = var_xgm_dn5;
        *var_xgm_dn6_slot = var_xgm_dn6;
        *var_xgm_dn7_slot = var_xgm_dn7;
        *var_xgm_dn8_slot = var_xgm_dn8;
        *var_xgs_dc_slot = var_xgs_dc;
        *var_xgs_dc_dn5_slot = var_xgs_dc_dn5;
        *var_xgs_dc_dn6_slot = var_xgs_dc_dn6;
        *var_xgs_dc_dn7_slot = var_xgs_dc_dn7;
        *var_xgs_dc_dn8_slot = var_xgs_dc_dn8;
        *var_xi1s_dc_slot = var_xi1s_dc;
        *var_xi1s_dc_dn5_slot = var_xi1s_dc_dn5;
        *var_xi1s_dc_dn6_slot = var_xi1s_dc_dn6;
        *var_xi1s_dc_dn7_slot = var_xi1s_dc_dn7;
        *var_xi1s_dc_dn8_slot = var_xi1s_dc_dn8;
        *var_xi2s_dc_slot = var_xi2s_dc;
        *var_xi2s_dc_dn5_slot = var_xi2s_dc_dn5;
        *var_xi2s_dc_dn6_slot = var_xi2s_dc_dn6;
        *var_xi2s_dc_dn7_slot = var_xi2s_dc_dn7;
        *var_xi2s_dc_dn8_slot = var_xi2s_dc_dn8;
        *var_xi_dc_slot = var_xi_dc;
        *var_xi_dc_dn5_slot = var_xi_dc_dn5;
        *var_xi_dc_dn6_slot = var_xi_dc_dn6;
        *var_xi_dc_dn7_slot = var_xi_dc_dn7;
        *var_xi_dc_dn8_slot = var_xi_dc_dn8;
        *var_xitsb_dc_slot = var_xitsb_dc;
        *var_xitsb_dc_dn5_slot = var_xitsb_dc_dn5;
        *var_xitsb_dc_dn6_slot = var_xitsb_dc_dn6;
        *var_xitsb_dc_dn7_slot = var_xitsb_dc_dn7;
        *var_xitsb_dc_dn8_slot = var_xitsb_dc_dn8;
        *var_xn_s_dc_slot = var_xn_s_dc;
        *var_xn_s_dc_dn5_slot = var_xn_s_dc_dn5;
        *var_xn_s_dc_dn6_slot = var_xn_s_dc_dn6;
        *var_xn_s_dc_dn7_slot = var_xn_s_dc_dn7;
        *var_xn_s_dc_dn8_slot = var_xn_s_dc_dn8;
        *var_xno_s_dc_slot = var_xno_s_dc;
        *var_xno_s_dc_dn5_slot = var_xno_s_dc_dn5;
        *var_xno_s_dc_dn6_slot = var_xno_s_dc_dn6;
        *var_xno_s_dc_dn7_slot = var_xno_s_dc_dn7;
        *var_xno_s_dc_dn8_slot = var_xno_s_dc_dn8;
    }

    pub(super) fn stamp_transient_block_94(
        var_alphas: f64,
        var_alphas_dn5: f64,
        var_alphas_dn6: f64,
        var_alphas_dn7: f64,
        var_alphas_dn8: f64,
        var_arloc: f64,
        var_asat: f64,
        var_asat_dn5: f64,
        var_asat_dn6: f64,
        var_asat_dn7: f64,
        var_asat_dn8: f64,
        var_chnl_type: f64,
        var_cs_t: f64,
        var_ds: f64,
        var_ds_dn5: f64,
        var_ds_dn6: f64,
        var_ds_dn7: f64,
        var_ds_dn8: f64,
        var_e_eff0: f64,
        var_eta_mu: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_guard1197: f64,
        var_guard1198: f64,
        var_guard1201: f64,
        var_inv_gf2: f64,
        var_inv_gf2_dn5: f64,
        var_inv_gf2_dn6: f64,
        var_inv_gf2_dn7: f64,
        var_inv_gf2_dn8: f64,
        var_midphi0: f64,
        var_midphi0_dn5: f64,
        var_midphi0_dn6: f64,
        var_midphi0_dn7: f64,
        var_midphi0_dn8: f64,
        var_mue_t: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_qis: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_rhob: f64,
        var_rhob_dn5: f64,
        var_rhob_dn6: f64,
        var_rhob_dn7: f64,
        var_rhob_dn8: f64,
        var_rhog: f64,
        var_rhog_dn5: f64,
        var_rhog_dn6: f64,
        var_rhog_dn7: f64,
        var_rhog_dn8: f64,
        var_thecs_t: f64,
        var_themu_t: f64,
        var_ther_i: f64,
        var_thesat1: f64,
        var_thesat1_dn5: f64,
        var_thesat1_dn6: f64,
        var_thesat1_dn7: f64,
        var_thesat1_dn8: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_vdsat_lim: f64,
        var_vdsat_lim_dn5: f64,
        var_vdsat_lim_dn6: f64,
        var_vdsat_lim_dn7: f64,
        var_vdsat_lim_dn8: f64,
        var_x_inf0: f64,
        var_x_inf0_dn5: f64,
        var_x_inf0_dn6: f64,
        var_x_inf0_dn7: f64,
        var_x_inf0_dn8: f64,
        var_xgs: f64,
        var_xgs_dn5: f64,
        var_xgs_dn6: f64,
        var_xgs_dn7: f64,
        var_xgs_dn8: f64,
        var_alphasat_slot: &mut f64,
        var_alphasat_dn5_slot: &mut f64,
        var_alphasat_dn6_slot: &mut f64,
        var_alphasat_dn7_slot: &mut f64,
        var_alphasat_dn8_slot: &mut f64,
        var_delta_gmob_slot: &mut f64,
        var_delta_gmob_dn5_slot: &mut f64,
        var_delta_gmob_dn6_slot: &mut f64,
        var_delta_gmob_dn7_slot: &mut f64,
        var_delta_gmob_dn8_slot: &mut f64,
        var_gmobcssat_slot: &mut f64,
        var_gmobcssat_dn5_slot: &mut f64,
        var_gmobcssat_dn6_slot: &mut f64,
        var_gmobcssat_dn7_slot: &mut f64,
        var_gmobcssat_dn8_slot: &mut f64,
        var_gmobmusat_slot: &mut f64,
        var_gmobmusat_dn5_slot: &mut f64,
        var_gmobmusat_dn6_slot: &mut f64,
        var_gmobmusat_dn7_slot: &mut f64,
        var_gmobmusat_dn8_slot: &mut f64,
        var_grsat_slot: &mut f64,
        var_grsat_dn5_slot: &mut f64,
        var_grsat_dn6_slot: &mut f64,
        var_grsat_dn7_slot: &mut f64,
        var_grsat_dn8_slot: &mut f64,
        var_guard1202_slot: &mut f64,
        var_guard1203_slot: &mut f64,
        var_qbsat_slot: &mut f64,
        var_qbsat_dn5_slot: &mut f64,
        var_qbsat_dn6_slot: &mut f64,
        var_qbsat_dn7_slot: &mut f64,
        var_qbsat_dn8_slot: &mut f64,
        var_qisat_slot: &mut f64,
        var_qisat_dn5_slot: &mut f64,
        var_qisat_dn6_slot: &mut f64,
        var_qisat_dn7_slot: &mut f64,
        var_qisat_dn8_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_v_dsat_slot: &mut f64,
        var_v_dsat_dn5_slot: &mut f64,
        var_v_dsat_dn6_slot: &mut f64,
        var_v_dsat_dn7_slot: &mut f64,
        var_v_dsat_dn8_slot: &mut f64,
        var_x_0_slot: &mut f64,
        var_x_0_dn5_slot: &mut f64,
        var_x_0_dn6_slot: &mut f64,
        var_x_0_dn7_slot: &mut f64,
        var_x_0_dn8_slot: &mut f64,
        var_x_inf_slot: &mut f64,
        var_x_inf_dn5_slot: &mut f64,
        var_x_inf_dn6_slot: &mut f64,
        var_x_inf_dn7_slot: &mut f64,
        var_x_inf_dn8_slot: &mut f64,
        var_x_sat_slot: &mut f64,
        var_x_sat_dn5_slot: &mut f64,
        var_x_sat_dn6_slot: &mut f64,
        var_x_sat_dn7_slot: &mut f64,
        var_x_sat_dn8_slot: &mut f64,
        var_ysat_slot: &mut f64,
        var_ysat_dn5_slot: &mut f64,
        var_ysat_dn6_slot: &mut f64,
        var_ysat_dn7_slot: &mut f64,
        var_ysat_dn8_slot: &mut f64,
        var_za_slot: &mut f64,
        var_za_dn5_slot: &mut f64,
        var_za_dn6_slot: &mut f64,
        var_za_dn7_slot: &mut f64,
        var_za_dn8_slot: &mut f64,
    ) {
        let mut var_alphasat: f64 = *var_alphasat_slot;
        let mut var_alphasat_dn5: f64 = *var_alphasat_dn5_slot;
        let mut var_alphasat_dn6: f64 = *var_alphasat_dn6_slot;
        let mut var_alphasat_dn7: f64 = *var_alphasat_dn7_slot;
        let mut var_alphasat_dn8: f64 = *var_alphasat_dn8_slot;
        let mut var_delta_gmob: f64 = *var_delta_gmob_slot;
        let mut var_delta_gmob_dn5: f64 = *var_delta_gmob_dn5_slot;
        let mut var_delta_gmob_dn6: f64 = *var_delta_gmob_dn6_slot;
        let mut var_delta_gmob_dn7: f64 = *var_delta_gmob_dn7_slot;
        let mut var_delta_gmob_dn8: f64 = *var_delta_gmob_dn8_slot;
        let mut var_gmobcssat: f64 = *var_gmobcssat_slot;
        let mut var_gmobcssat_dn5: f64 = *var_gmobcssat_dn5_slot;
        let mut var_gmobcssat_dn6: f64 = *var_gmobcssat_dn6_slot;
        let mut var_gmobcssat_dn7: f64 = *var_gmobcssat_dn7_slot;
        let mut var_gmobcssat_dn8: f64 = *var_gmobcssat_dn8_slot;
        let mut var_gmobmusat: f64 = *var_gmobmusat_slot;
        let mut var_gmobmusat_dn5: f64 = *var_gmobmusat_dn5_slot;
        let mut var_gmobmusat_dn6: f64 = *var_gmobmusat_dn6_slot;
        let mut var_gmobmusat_dn7: f64 = *var_gmobmusat_dn7_slot;
        let mut var_gmobmusat_dn8: f64 = *var_gmobmusat_dn8_slot;
        let mut var_grsat: f64 = *var_grsat_slot;
        let mut var_grsat_dn5: f64 = *var_grsat_dn5_slot;
        let mut var_grsat_dn6: f64 = *var_grsat_dn6_slot;
        let mut var_grsat_dn7: f64 = *var_grsat_dn7_slot;
        let mut var_grsat_dn8: f64 = *var_grsat_dn8_slot;
        let mut var_guard1202: f64 = *var_guard1202_slot;
        let mut var_guard1203: f64 = *var_guard1203_slot;
        let mut var_qbsat: f64 = *var_qbsat_slot;
        let mut var_qbsat_dn5: f64 = *var_qbsat_dn5_slot;
        let mut var_qbsat_dn6: f64 = *var_qbsat_dn6_slot;
        let mut var_qbsat_dn7: f64 = *var_qbsat_dn7_slot;
        let mut var_qbsat_dn8: f64 = *var_qbsat_dn8_slot;
        let mut var_qisat: f64 = *var_qisat_slot;
        let mut var_qisat_dn5: f64 = *var_qisat_dn5_slot;
        let mut var_qisat_dn6: f64 = *var_qisat_dn6_slot;
        let mut var_qisat_dn7: f64 = *var_qisat_dn7_slot;
        let mut var_qisat_dn8: f64 = *var_qisat_dn8_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_v_dsat: f64 = *var_v_dsat_slot;
        let mut var_v_dsat_dn5: f64 = *var_v_dsat_dn5_slot;
        let mut var_v_dsat_dn6: f64 = *var_v_dsat_dn6_slot;
        let mut var_v_dsat_dn7: f64 = *var_v_dsat_dn7_slot;
        let mut var_v_dsat_dn8: f64 = *var_v_dsat_dn8_slot;
        let mut var_x_0: f64 = *var_x_0_slot;
        let mut var_x_0_dn5: f64 = *var_x_0_dn5_slot;
        let mut var_x_0_dn6: f64 = *var_x_0_dn6_slot;
        let mut var_x_0_dn7: f64 = *var_x_0_dn7_slot;
        let mut var_x_0_dn8: f64 = *var_x_0_dn8_slot;
        let mut var_x_inf: f64 = *var_x_inf_slot;
        let mut var_x_inf_dn5: f64 = *var_x_inf_dn5_slot;
        let mut var_x_inf_dn6: f64 = *var_x_inf_dn6_slot;
        let mut var_x_inf_dn7: f64 = *var_x_inf_dn7_slot;
        let mut var_x_inf_dn8: f64 = *var_x_inf_dn8_slot;
        let mut var_x_sat: f64 = *var_x_sat_slot;
        let mut var_x_sat_dn5: f64 = *var_x_sat_dn5_slot;
        let mut var_x_sat_dn6: f64 = *var_x_sat_dn6_slot;
        let mut var_x_sat_dn7: f64 = *var_x_sat_dn7_slot;
        let mut var_x_sat_dn8: f64 = *var_x_sat_dn8_slot;
        let mut var_ysat: f64 = *var_ysat_slot;
        let mut var_ysat_dn5: f64 = *var_ysat_dn5_slot;
        let mut var_ysat_dn6: f64 = *var_ysat_dn6_slot;
        let mut var_ysat_dn7: f64 = *var_ysat_dn7_slot;
        let mut var_ysat_dn8: f64 = *var_ysat_dn8_slot;
        let mut var_za: f64 = *var_za_slot;
        let mut var_za_dn5: f64 = *var_za_dn5_slot;
        let mut var_za_dn6: f64 = *var_za_dn6_slot;
        let mut var_za_dn7: f64 = *var_za_dn7_slot;
        let mut var_za_dn8: f64 = *var_za_dn8_slot;

        let (assign43730_e56467, assign43730_e56467_d_n5, assign43730_e56467_d_n6, assign43730_e56467_d_n7, assign43730_e56467_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43730_e56460: f64 = (var_temp__blk936 * var_temp__blk936);
        let assign43730_e56462: f64 = (assign43730_e56460 + 1e-12);
        let assign43730_e56463: f64 = (assign43730_e56462).sqrt();
        let assign43730_e56464: f64 = (var_temp__blk936 + assign43730_e56463);
        let assign43730_e56465: f64 = (0.5 * assign43730_e56464);
        (assign43730_e56465, (0.5 * (var_temp__blk936_dn5 + (((var_temp__blk936_dn5 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn5)) / (2.0 * assign43730_e56463)))), (0.5 * (var_temp__blk936_dn6 + (((var_temp__blk936_dn6 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn6)) / (2.0 * assign43730_e56463)))), (0.5 * (var_temp__blk936_dn7 + (((var_temp__blk936_dn7 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn7)) / (2.0 * assign43730_e56463)))), (0.5 * (var_temp__blk936_dn8 + (((var_temp__blk936_dn8 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn8)) / (2.0 * assign43730_e56463)))),)
    } else {
        (var_qisat, var_qisat_dn5, var_qisat_dn6, var_qisat_dn7, var_qisat_dn8,)
    }
};
        var_qisat = assign43730_e56467;
        var_qisat_dn5 = assign43730_e56467_d_n5;
        var_qisat_dn6 = assign43730_e56467_d_n6;
        var_qisat_dn7 = assign43730_e56467_d_n7;
        var_qisat_dn8 = assign43730_e56467_d_n8;

        let (assign43740_e56485, assign43740_e56485_d_n5, assign43740_e56485_d_n6, assign43740_e56485_d_n7, assign43740_e56485_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43740_e56475: f64 = (var_phit1 * var_xgs);
        let assign43740_e56477: f64 = (assign43740_e56475 - var_qis);
        let assign43740_e56480: f64 = (var_alphas - 1.0);
        let assign43740_e56482: f64 = (assign43740_e56480 * var_midphi0);
        let assign43740_e56483: f64 = (assign43740_e56477 + assign43740_e56482);
        (assign43740_e56483, ((((var_phit1_dn5 * var_xgs) + (var_phit1 * var_xgs_dn5)) - var_qis_dn5) + ((var_alphas_dn5 * var_midphi0) + (assign43740_e56480 * var_midphi0_dn5))), ((((var_phit1_dn6 * var_xgs) + (var_phit1 * var_xgs_dn6)) - var_qis_dn6) + ((var_alphas_dn6 * var_midphi0) + (assign43740_e56480 * var_midphi0_dn6))), ((((var_phit1_dn7 * var_xgs) + (var_phit1 * var_xgs_dn7)) - var_qis_dn7) + ((var_alphas_dn7 * var_midphi0) + (assign43740_e56480 * var_midphi0_dn7))), ((((var_phit1_dn8 * var_xgs) + (var_phit1 * var_xgs_dn8)) - var_qis_dn8) + ((var_alphas_dn8 * var_midphi0) + (assign43740_e56480 * var_midphi0_dn8))),)
    } else {
        (var_qbsat, var_qbsat_dn5, var_qbsat_dn6, var_qbsat_dn7, var_qbsat_dn8,)
    }
};
        var_qbsat = assign43740_e56485;
        var_qbsat_dn5 = assign43740_e56485_d_n5;
        var_qbsat_dn6 = assign43740_e56485_d_n6;
        var_qbsat_dn7 = assign43740_e56485_d_n7;
        var_qbsat_dn8 = assign43740_e56485_d_n8;

        let (assign43750_e56501, assign43750_e56501_d_n5, assign43750_e56501_d_n6, assign43750_e56501_d_n7, assign43750_e56501_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43750_e56494: f64 = (0.5 * var_gf2);
        let assign43750_e56496: f64 = (assign43750_e56494 * var_phit1);
        let assign43750_e56498: f64 = (assign43750_e56496 / var_qbsat);
        let assign43750_e56499: f64 = (1.0 + assign43750_e56498);
        (assign43750_e56499, ((((((0.5 * var_gf2_dn5) * var_phit1) + (assign43750_e56494 * var_phit1_dn5)) * var_qbsat) - (assign43750_e56496 * var_qbsat_dn5)) / (var_qbsat * var_qbsat)), ((((((0.5 * var_gf2_dn6) * var_phit1) + (assign43750_e56494 * var_phit1_dn6)) * var_qbsat) - (assign43750_e56496 * var_qbsat_dn6)) / (var_qbsat * var_qbsat)), ((((((0.5 * var_gf2_dn7) * var_phit1) + (assign43750_e56494 * var_phit1_dn7)) * var_qbsat) - (assign43750_e56496 * var_qbsat_dn7)) / (var_qbsat * var_qbsat)), ((((((0.5 * var_gf2_dn8) * var_phit1) + (assign43750_e56494 * var_phit1_dn8)) * var_qbsat) - (assign43750_e56496 * var_qbsat_dn8)) / (var_qbsat * var_qbsat)),)
    } else {
        (var_alphasat, var_alphasat_dn5, var_alphasat_dn6, var_alphasat_dn7, var_alphasat_dn8,)
    }
};
        var_alphasat = assign43750_e56501;
        var_alphasat_dn5 = assign43750_e56501_d_n5;
        var_alphasat_dn6 = assign43750_e56501_d_n6;
        var_alphasat_dn7 = assign43750_e56501_d_n7;
        var_alphasat_dn8 = assign43750_e56501_d_n8;

        let (assign43760_e56513, assign43760_e56513_d_n5, assign43760_e56513_d_n6, assign43760_e56513_d_n7, assign43760_e56513_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43760_e56510: f64 = (var_eta_mu * var_qisat);
        let assign43760_e56511: f64 = (var_qbsat + assign43760_e56510);
        (assign43760_e56511, (var_qbsat_dn5 + (var_eta_mu * var_qisat_dn5)), (var_qbsat_dn6 + (var_eta_mu * var_qisat_dn6)), (var_qbsat_dn7 + (var_eta_mu * var_qisat_dn7)), (var_qbsat_dn8 + (var_eta_mu * var_qisat_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43760_e56513;
        var_temp__blk936_dn5 = assign43760_e56513_d_n5;
        var_temp__blk936_dn6 = assign43760_e56513_d_n6;
        var_temp__blk936_dn7 = assign43760_e56513_d_n7;
        var_temp__blk936_dn8 = assign43760_e56513_d_n8;

        let (assign43770_e56527, assign43770_e56527_d_n5, assign43770_e56527_d_n6, assign43770_e56527_d_n7, assign43770_e56527_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43770_e56521: f64 = (var_e_eff0 * var_temp__blk936);
        let assign43770_e56523: f64 = (assign43770_e56521 * var_mue_t);
        let assign43770_e56525: f64 = (assign43770_e56523).powf(var_themu_t);
        (assign43770_e56525, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43770_e56523).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk936_dn5) * var_mue_t))) } } else { (assign43770_e56525 * (var_themu_t * (((var_e_eff0 * var_temp__blk936_dn5) * var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43770_e56523).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk936_dn6) * var_mue_t))) } } else { (assign43770_e56525 * (var_themu_t * (((var_e_eff0 * var_temp__blk936_dn6) * var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43770_e56523).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk936_dn7) * var_mue_t))) } } else { (assign43770_e56525 * (var_themu_t * (((var_e_eff0 * var_temp__blk936_dn7) * var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43770_e56523).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk936_dn8) * var_mue_t))) } } else { (assign43770_e56525 * (var_themu_t * (((var_e_eff0 * var_temp__blk936_dn8) * var_mue_t) / assign43770_e56523))) },)
    } else {
        (var_gmobmusat, var_gmobmusat_dn5, var_gmobmusat_dn6, var_gmobmusat_dn7, var_gmobmusat_dn8,)
    }
};
        var_gmobmusat = assign43770_e56527;
        var_gmobmusat_dn5 = assign43770_e56527_d_n5;
        var_gmobmusat_dn6 = assign43770_e56527_d_n6;
        var_gmobmusat_dn7 = assign43770_e56527_d_n7;
        var_gmobmusat_dn8 = assign43770_e56527_d_n8;

        let (assign43780_e56547, assign43780_e56547_d_n5, assign43780_e56547_d_n6, assign43780_e56547_d_n7, assign43780_e56547_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43780_e56537: f64 = (1.0 - var_eta_mu);
        let assign43780_e56538: f64 = (var_alphasat * assign43780_e56537);
        let assign43780_e56540: f64 = (assign43780_e56538 - 1.0);
        let assign43780_e56541: f64 = (var_themu_t * assign43780_e56540);
        let assign43780_e56543: f64 = (assign43780_e56541 / var_temp__blk936);
        let assign43780_e56545: f64 = (assign43780_e56543 * var_gmobmusat);
        (assign43780_e56545, ((((((var_themu_t * (var_alphasat_dn5 * assign43780_e56537)) * var_temp__blk936) - (assign43780_e56541 * var_temp__blk936_dn5)) / (var_temp__blk936 * var_temp__blk936)) * var_gmobmusat) + (assign43780_e56543 * var_gmobmusat_dn5)), ((((((var_themu_t * (var_alphasat_dn6 * assign43780_e56537)) * var_temp__blk936) - (assign43780_e56541 * var_temp__blk936_dn6)) / (var_temp__blk936 * var_temp__blk936)) * var_gmobmusat) + (assign43780_e56543 * var_gmobmusat_dn6)), ((((((var_themu_t * (var_alphasat_dn7 * assign43780_e56537)) * var_temp__blk936) - (assign43780_e56541 * var_temp__blk936_dn7)) / (var_temp__blk936 * var_temp__blk936)) * var_gmobmusat) + (assign43780_e56543 * var_gmobmusat_dn7)), ((((((var_themu_t * (var_alphasat_dn8 * assign43780_e56537)) * var_temp__blk936) - (assign43780_e56541 * var_temp__blk936_dn8)) / (var_temp__blk936 * var_temp__blk936)) * var_gmobmusat) + (assign43780_e56543 * var_gmobmusat_dn8)),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign43780_e56547;
        var_temp1_dn5 = assign43780_e56547_d_n5;
        var_temp1_dn6 = assign43780_e56547_d_n6;
        var_temp1_dn7 = assign43780_e56547_d_n7;
        var_temp1_dn8 = assign43780_e56547_d_n8;

        let (assign43790_e56557, assign43790_e56557_d_n5, assign43790_e56557_d_n6, assign43790_e56557_d_n7, assign43790_e56557_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43790_e56555: f64 = (var_qisat / var_qbsat);
        (assign43790_e56555, (((var_qisat_dn5 * var_qbsat) - (var_qisat * var_qbsat_dn5)) / (var_qbsat * var_qbsat)), (((var_qisat_dn6 * var_qbsat) - (var_qisat * var_qbsat_dn6)) / (var_qbsat * var_qbsat)), (((var_qisat_dn7 * var_qbsat) - (var_qisat * var_qbsat_dn7)) / (var_qbsat * var_qbsat)), (((var_qisat_dn8 * var_qbsat) - (var_qisat * var_qbsat_dn8)) / (var_qbsat * var_qbsat)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43790_e56557;
        var_temp__blk936_dn5 = assign43790_e56557_d_n5;
        var_temp__blk936_dn6 = assign43790_e56557_d_n6;
        var_temp__blk936_dn7 = assign43790_e56557_d_n7;
        var_temp__blk936_dn8 = assign43790_e56557_d_n8;

        let (assign43800_e56572, assign43800_e56572_d_n5, assign43800_e56572_d_n6, assign43800_e56572_d_n7, assign43800_e56572_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43800_e56566: f64 = (1.0 + var_temp__blk936);
        let assign43800_e56568: f64 = (-var_thecs_t);
        let assign43800_e56569: f64 = (assign43800_e56566).powf(assign43800_e56568);
        let assign43800_e56570: f64 = (var_cs_t * assign43800_e56569);
        (assign43800_e56570, (var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * var_temp__blk936_dn5)) } } else { (assign43800_e56569 * (assign43800_e56568 * (var_temp__blk936_dn5 / assign43800_e56566))) }), (var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * var_temp__blk936_dn6)) } } else { (assign43800_e56569 * (assign43800_e56568 * (var_temp__blk936_dn6 / assign43800_e56566))) }), (var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * var_temp__blk936_dn7)) } } else { (assign43800_e56569 * (assign43800_e56568 * (var_temp__blk936_dn7 / assign43800_e56566))) }), (var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * var_temp__blk936_dn8)) } } else { (assign43800_e56569 * (assign43800_e56568 * (var_temp__blk936_dn8 / assign43800_e56566))) }),)
    } else {
        (var_gmobcssat, var_gmobcssat_dn5, var_gmobcssat_dn6, var_gmobcssat_dn7, var_gmobcssat_dn8,)
    }
};
        var_gmobcssat = assign43800_e56572;
        var_gmobcssat_dn5 = assign43800_e56572_d_n5;
        var_gmobcssat_dn6 = assign43800_e56572_d_n6;
        var_gmobcssat_dn7 = assign43800_e56572_d_n7;
        var_gmobcssat_dn8 = assign43800_e56572_d_n8;

        let (assign43810_e56594, assign43810_e56594_d_n5, assign43810_e56594_d_n6, assign43810_e56594_d_n7, assign43810_e56594_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43810_e56581: f64 = (var_alphasat - 1.0);
        let assign43810_e56585: f64 = (var_temp__blk936 + 1.0);
        let assign43810_e56586: f64 = (1.0 / assign43810_e56585);
        let assign43810_e56587: f64 = (assign43810_e56581 + assign43810_e56586);
        let assign43810_e56588: f64 = (var_thecs_t * assign43810_e56587);
        let assign43810_e56590: f64 = (assign43810_e56588 / var_qbsat);
        let assign43810_e56592: f64 = (assign43810_e56590 * var_gmobcssat);
        (assign43810_e56592, ((((((var_thecs_t * (var_alphasat_dn5 + (-(var_temp__blk936_dn5 / (assign43810_e56585 * assign43810_e56585))))) * var_qbsat) - (assign43810_e56588 * var_qbsat_dn5)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43810_e56590 * var_gmobcssat_dn5)), ((((((var_thecs_t * (var_alphasat_dn6 + (-(var_temp__blk936_dn6 / (assign43810_e56585 * assign43810_e56585))))) * var_qbsat) - (assign43810_e56588 * var_qbsat_dn6)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43810_e56590 * var_gmobcssat_dn6)), ((((((var_thecs_t * (var_alphasat_dn7 + (-(var_temp__blk936_dn7 / (assign43810_e56585 * assign43810_e56585))))) * var_qbsat) - (assign43810_e56588 * var_qbsat_dn7)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43810_e56590 * var_gmobcssat_dn7)), ((((((var_thecs_t * (var_alphasat_dn8 + (-(var_temp__blk936_dn8 / (assign43810_e56585 * assign43810_e56585))))) * var_qbsat) - (assign43810_e56588 * var_qbsat_dn8)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43810_e56590 * var_gmobcssat_dn8)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign43810_e56594;
        var_temp2_dn5 = assign43810_e56594_d_n5;
        var_temp2_dn6 = assign43810_e56594_d_n6;
        var_temp2_dn7 = assign43810_e56594_d_n7;
        var_temp2_dn8 = assign43810_e56594_d_n8;

        let (assign43820_e56608, assign43820_e56608_d_n5, assign43820_e56608_d_n6, assign43820_e56608_d_n7, assign43820_e56608_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43820_e56602: f64 = (var_ther_i * var_rhob);
        let assign43820_e56604: f64 = (assign43820_e56602 * var_rhog);
        let assign43820_e56606: f64 = (assign43820_e56604 * var_qisat);
        (assign43820_e56606, (((((var_ther_i * var_rhob_dn5) * var_rhog) + (assign43820_e56602 * var_rhog_dn5)) * var_qisat) + (assign43820_e56604 * var_qisat_dn5)), (((((var_ther_i * var_rhob_dn6) * var_rhog) + (assign43820_e56602 * var_rhog_dn6)) * var_qisat) + (assign43820_e56604 * var_qisat_dn6)), (((((var_ther_i * var_rhob_dn7) * var_rhog) + (assign43820_e56602 * var_rhog_dn7)) * var_qisat) + (assign43820_e56604 * var_qisat_dn7)), (((((var_ther_i * var_rhob_dn8) * var_rhog) + (assign43820_e56602 * var_rhog_dn8)) * var_qisat) + (assign43820_e56604 * var_qisat_dn8)),)
    } else {
        (var_grsat, var_grsat_dn5, var_grsat_dn6, var_grsat_dn7, var_grsat_dn8,)
    }
};
        var_grsat = assign43820_e56608;
        var_grsat_dn5 = assign43820_e56608_d_n5;
        var_grsat_dn6 = assign43820_e56608_d_n6;
        var_grsat_dn7 = assign43820_e56608_d_n7;
        var_grsat_dn8 = assign43820_e56608_d_n8;

        let (assign43830_e56628, assign43830_e56628_d_n5, assign43830_e56628_d_n6, assign43830_e56628_d_n7, assign43830_e56628_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43830_e56618: f64 = (var_ther_i * var_rhob);
        let assign43830_e56620: f64 = (assign43830_e56618 * var_rhog);
        let assign43830_e56622: f64 = (assign43830_e56620 * var_alphasat);
        let assign43830_e56623: f64 = (var_temp1 - assign43830_e56622);
        let assign43830_e56625: f64 = (assign43830_e56623 / var_temp2);
        let assign43830_e56626: f64 = (1.0 + assign43830_e56625);
        (assign43830_e56626, ((((var_temp1_dn5 - (((((var_ther_i * var_rhob_dn5) * var_rhog) + (assign43830_e56618 * var_rhog_dn5)) * var_alphasat) + (assign43830_e56620 * var_alphasat_dn5))) * var_temp2) - (assign43830_e56623 * var_temp2_dn5)) / (var_temp2 * var_temp2)), ((((var_temp1_dn6 - (((((var_ther_i * var_rhob_dn6) * var_rhog) + (assign43830_e56618 * var_rhog_dn6)) * var_alphasat) + (assign43830_e56620 * var_alphasat_dn6))) * var_temp2) - (assign43830_e56623 * var_temp2_dn6)) / (var_temp2 * var_temp2)), ((((var_temp1_dn7 - (((((var_ther_i * var_rhob_dn7) * var_rhog) + (assign43830_e56618 * var_rhog_dn7)) * var_alphasat) + (assign43830_e56620 * var_alphasat_dn7))) * var_temp2) - (assign43830_e56623 * var_temp2_dn7)) / (var_temp2 * var_temp2)), ((((var_temp1_dn8 - (((((var_ther_i * var_rhob_dn8) * var_rhog) + (assign43830_e56618 * var_rhog_dn8)) * var_alphasat) + (assign43830_e56620 * var_alphasat_dn8))) * var_temp2) - (assign43830_e56623 * var_temp2_dn8)) / (var_temp2 * var_temp2)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43830_e56628;
        var_temp__blk936_dn5 = assign43830_e56628_d_n5;
        var_temp__blk936_dn6 = assign43830_e56628_d_n6;
        var_temp__blk936_dn7 = assign43830_e56628_d_n7;
        var_temp__blk936_dn8 = assign43830_e56628_d_n8;

        let assign43840_e56631: f64 = if var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1202 = assign43840_e56631;

        let (assign43850_e56649, assign43850_e56649_d_n5, assign43850_e56649_d_n6, assign43850_e56649_d_n7, assign43850_e56649_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) && (var_guard1202 != 0.0)) {
        let assign43850_e56643: f64 = (2.0 * var_temp__blk936);
        let assign43850_e56644: f64 = (assign43850_e56643).exp();
        let assign43850_e56645: f64 = (1.0 + assign43850_e56644);
        let assign43850_e56646: f64 = (assign43850_e56645).ln();
        let assign43850_e56647: f64 = (0.5 * assign43850_e56646);
        (assign43850_e56647, (0.5 * ((assign43850_e56644 * (2.0 * var_temp__blk936_dn5)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * var_temp__blk936_dn6)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * var_temp__blk936_dn7)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * var_temp__blk936_dn8)) / assign43850_e56645)),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign43850_e56649;
        var_temp1_dn5 = assign43850_e56649_d_n5;
        var_temp1_dn6 = assign43850_e56649_d_n6;
        var_temp1_dn7 = assign43850_e56649_d_n7;
        var_temp1_dn8 = assign43850_e56649_d_n8;

        let (assign43860_e56660, assign43860_e56660_d_n5, assign43860_e56660_d_n6, assign43860_e56660_d_n7, assign43860_e56660_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) && (var_guard1202 == 0.0)) {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign43860_e56660;
        var_temp1_dn5 = assign43860_e56660_d_n5;
        var_temp1_dn6 = assign43860_e56660_d_n6;
        var_temp1_dn7 = assign43860_e56660_d_n7;
        var_temp1_dn8 = assign43860_e56660_d_n8;

        let (assign43870_e56681, assign43870_e56681_d_n5, assign43870_e56681_d_n6, assign43870_e56681_d_n7, assign43870_e56681_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43870_e56667: f64 = (-var_midphi0);
        let assign43870_e56669: f64 = (assign43870_e56667 * var_temp2);
        let assign43870_e56671: f64 = (assign43870_e56669 * var_temp1);
        let assign43870_e56674: f64 = (1.0 + var_gmobmusat);
        let assign43870_e56676: f64 = (assign43870_e56674 + var_gmobcssat);
        let assign43870_e56678: f64 = (assign43870_e56676 + var_grsat);
        let assign43870_e56679: f64 = (assign43870_e56671 / assign43870_e56678);
        (assign43870_e56679, ((((((((-var_midphi0_dn5) * var_temp2) + (assign43870_e56667 * var_temp2_dn5)) * var_temp1) + (assign43870_e56669 * var_temp1_dn5)) * assign43870_e56678) - (assign43870_e56671 * ((var_gmobmusat_dn5 + var_gmobcssat_dn5) + var_grsat_dn5))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-var_midphi0_dn6) * var_temp2) + (assign43870_e56667 * var_temp2_dn6)) * var_temp1) + (assign43870_e56669 * var_temp1_dn6)) * assign43870_e56678) - (assign43870_e56671 * ((var_gmobmusat_dn6 + var_gmobcssat_dn6) + var_grsat_dn6))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-var_midphi0_dn7) * var_temp2) + (assign43870_e56667 * var_temp2_dn7)) * var_temp1) + (assign43870_e56669 * var_temp1_dn7)) * assign43870_e56678) - (assign43870_e56671 * ((var_gmobmusat_dn7 + var_gmobcssat_dn7) + var_grsat_dn7))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-var_midphi0_dn8) * var_temp2) + (assign43870_e56667 * var_temp2_dn8)) * var_temp1) + (assign43870_e56669 * var_temp1_dn8)) * assign43870_e56678) - (assign43870_e56671 * ((var_gmobmusat_dn8 + var_gmobcssat_dn8) + var_grsat_dn8))) / (assign43870_e56678 * assign43870_e56678)),)
    } else {
        (var_delta_gmob, var_delta_gmob_dn5, var_delta_gmob_dn6, var_delta_gmob_dn7, var_delta_gmob_dn8,)
    }
};
        var_delta_gmob = assign43870_e56681;
        var_delta_gmob_dn5 = assign43870_e56681_d_n5;
        var_delta_gmob_dn6 = assign43870_e56681_d_n6;
        var_delta_gmob_dn7 = assign43870_e56681_d_n7;
        var_delta_gmob_dn8 = assign43870_e56681_d_n8;

        let (assign43880_e56702, assign43880_e56702_d_n5, assign43880_e56702_d_n6, assign43880_e56702_d_n7, assign43880_e56702_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43880_e56694: f64 = (var_delta_gmob * var_delta_gmob);
        let assign43880_e56695: f64 = (1.0 + assign43880_e56694);
        let assign43880_e56696: f64 = (assign43880_e56695).sqrt();
        let assign43880_e56697: f64 = (1.0 + assign43880_e56696);
        let assign43880_e56698: f64 = (var_delta_gmob / assign43880_e56697);
        let assign43880_e56699: f64 = (1.0 + assign43880_e56698);
        let assign43880_e56700: f64 = (var_x_inf0 * assign43880_e56699);
        (assign43880_e56700, ((var_x_inf0_dn5 * assign43880_e56699) + (var_x_inf0 * (((var_delta_gmob_dn5 * assign43880_e56697) - (var_delta_gmob * (((var_delta_gmob_dn5 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn5)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((var_x_inf0_dn6 * assign43880_e56699) + (var_x_inf0 * (((var_delta_gmob_dn6 * assign43880_e56697) - (var_delta_gmob * (((var_delta_gmob_dn6 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn6)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((var_x_inf0_dn7 * assign43880_e56699) + (var_x_inf0 * (((var_delta_gmob_dn7 * assign43880_e56697) - (var_delta_gmob * (((var_delta_gmob_dn7 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn7)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((var_x_inf0_dn8 * assign43880_e56699) + (var_x_inf0 * (((var_delta_gmob_dn8 * assign43880_e56697) - (var_delta_gmob * (((var_delta_gmob_dn8 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn8)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))),)
    } else {
        (var_x_inf, var_x_inf_dn5, var_x_inf_dn6, var_x_inf_dn7, var_x_inf_dn8,)
    }
};
        var_x_inf = assign43880_e56702;
        var_x_inf_dn5 = assign43880_e56702_d_n5;
        var_x_inf_dn6 = assign43880_e56702_d_n6;
        var_x_inf_dn7 = assign43880_e56702_d_n7;
        var_x_inf_dn8 = assign43880_e56702_d_n8;

        let (assign43890_e56711, assign43890_e56711_d_n5, assign43890_e56711_d_n6, assign43890_e56711_d_n7, assign43890_e56711_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 == 0.0)) {
        (var_x_inf0, var_x_inf0_dn5, var_x_inf0_dn6, var_x_inf0_dn7, var_x_inf0_dn8,)
    } else {
        (var_x_inf, var_x_inf_dn5, var_x_inf_dn6, var_x_inf_dn7, var_x_inf_dn8,)
    }
};
        var_x_inf = assign43890_e56711;
        var_x_inf_dn5 = assign43890_e56711_d_n5;
        var_x_inf_dn6 = assign43890_e56711_d_n6;
        var_x_inf_dn7 = assign43890_e56711_d_n7;
        var_x_inf_dn8 = assign43890_e56711_d_n8;

        let (assign43900_e56723, assign43900_e56723_d_n5, assign43900_e56723_d_n6, assign43900_e56723_d_n7, assign43900_e56723_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43900_e56717: f64 = (var_phit1 * var_thesat1);
        let assign43900_e56719: f64 = (assign43900_e56717 * var_x_inf);
        let assign43900_e56721: f64 = (assign43900_e56719 * 0.7071067811865475);
        (assign43900_e56721, (((((var_phit1_dn5 * var_thesat1) + (var_phit1 * var_thesat1_dn5)) * var_x_inf) + (assign43900_e56717 * var_x_inf_dn5)) * 0.7071067811865475), (((((var_phit1_dn6 * var_thesat1) + (var_phit1 * var_thesat1_dn6)) * var_x_inf) + (assign43900_e56717 * var_x_inf_dn6)) * 0.7071067811865475), (((((var_phit1_dn7 * var_thesat1) + (var_phit1 * var_thesat1_dn7)) * var_x_inf) + (assign43900_e56717 * var_x_inf_dn7)) * 0.7071067811865475), (((((var_phit1_dn8 * var_thesat1) + (var_phit1 * var_thesat1_dn8)) * var_x_inf) + (assign43900_e56717 * var_x_inf_dn8)) * 0.7071067811865475),)
    } else {
        (var_ysat, var_ysat_dn5, var_ysat_dn6, var_ysat_dn7, var_ysat_dn8,)
    }
};
        var_ysat = assign43900_e56723;
        var_ysat_dn5 = assign43900_e56723_d_n5;
        var_ysat_dn6 = assign43900_e56723_d_n6;
        var_ysat_dn7 = assign43900_e56723_d_n7;
        var_ysat_dn8 = assign43900_e56723_d_n8;

        let assign43910_e56726: f64 = (-1.0);
        let assign43910_e56727: f64 = if var_chnl_type == assign43910_e56726 { 1.0 } else { 0.0 };
        var_guard1203 = assign43910_e56727;

        let (assign43920_e56740, assign43920_e56740_d_n5, assign43920_e56740_d_n6, assign43920_e56740_d_n7, assign43920_e56740_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1203 != 0.0)) {
        let assign43920_e56736: f64 = (1.0 + var_ysat);
        let assign43920_e56737: f64 = (assign43920_e56736).sqrt();
        let assign43920_e56738: f64 = (var_ysat / assign43920_e56737);
        (assign43920_e56738, (((var_ysat_dn5 * assign43920_e56737) - (var_ysat * (var_ysat_dn5 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((var_ysat_dn6 * assign43920_e56737) - (var_ysat * (var_ysat_dn6 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((var_ysat_dn7 * assign43920_e56737) - (var_ysat * (var_ysat_dn7 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((var_ysat_dn8 * assign43920_e56737) - (var_ysat * (var_ysat_dn8 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)),)
    } else {
        (var_ysat, var_ysat_dn5, var_ysat_dn6, var_ysat_dn7, var_ysat_dn8,)
    }
};
        var_ysat = assign43920_e56740;
        var_ysat_dn5 = assign43920_e56740_d_n5;
        var_ysat_dn6 = assign43920_e56740_d_n6;
        var_ysat_dn7 = assign43920_e56740_d_n7;
        var_ysat_dn8 = assign43920_e56740_d_n8;

        let (assign43930_e56755, assign43930_e56755_d_n5, assign43930_e56755_d_n6, assign43930_e56755_d_n7, assign43930_e56755_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43930_e56749: f64 = (4.0 * var_ysat);
        let assign43930_e56750: f64 = (1.0 + assign43930_e56749);
        let assign43930_e56751: f64 = (assign43930_e56750).sqrt();
        let assign43930_e56752: f64 = (1.0 + assign43930_e56751);
        let assign43930_e56753: f64 = (2.0 / assign43930_e56752);
        (assign43930_e56753, (-((2.0 * ((4.0 * var_ysat_dn5) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * var_ysat_dn6) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * var_ysat_dn7) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * var_ysat_dn8) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))),)
    } else {
        (var_za, var_za_dn5, var_za_dn6, var_za_dn7, var_za_dn8,)
    }
};
        var_za = assign43930_e56755;
        var_za_dn5 = assign43930_e56755_d_n5;
        var_za_dn6 = assign43930_e56755_d_n6;
        var_za_dn7 = assign43930_e56755_d_n7;
        var_za_dn8 = assign43930_e56755_d_n8;

        let (assign43940_e56763, assign43940_e56763_d_n5, assign43940_e56763_d_n6, assign43940_e56763_d_n7, assign43940_e56763_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43940_e56761: f64 = (var_za * var_ysat);
        (assign43940_e56761, ((var_za_dn5 * var_ysat) + (var_za * var_ysat_dn5)), ((var_za_dn6 * var_ysat) + (var_za * var_ysat_dn6)), ((var_za_dn7 * var_ysat) + (var_za * var_ysat_dn7)), ((var_za_dn8 * var_ysat) + (var_za * var_ysat_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43940_e56763;
        var_temp__blk936_dn5 = assign43940_e56763_d_n5;
        var_temp__blk936_dn6 = assign43940_e56763_d_n6;
        var_temp__blk936_dn7 = assign43940_e56763_d_n7;
        var_temp__blk936_dn8 = assign43940_e56763_d_n8;

        let (assign43950_e56793, assign43950_e56793_d_n5, assign43950_e56793_d_n6, assign43950_e56793_d_n7, assign43950_e56793_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43950_e56769: f64 = (var_x_inf * var_za);
        let assign43950_e56773: f64 = (0.86 * var_temp__blk936);
        let assign43950_e56777: f64 = (var_temp__blk936 * var_za);
        let assign43950_e56778: f64 = (1.0 - assign43950_e56777);
        let assign43950_e56779: f64 = (assign43950_e56773 * assign43950_e56778);
        let assign43950_e56783: f64 = (4.0 * var_temp__blk936);
        let assign43950_e56785: f64 = (assign43950_e56783 * var_temp__blk936);
        let assign43950_e56787: f64 = (assign43950_e56785 * var_za);
        let assign43950_e56788: f64 = (1.0 + assign43950_e56787);
        let assign43950_e56789: f64 = (assign43950_e56779 / assign43950_e56788);
        let assign43950_e56790: f64 = (1.0 + assign43950_e56789);
        let assign43950_e56791: f64 = (assign43950_e56769 * assign43950_e56790);
        (assign43950_e56791, ((((var_x_inf_dn5 * var_za) + (var_x_inf * var_za_dn5)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * var_temp__blk936_dn5) * assign43950_e56778) + (assign43950_e56773 * (-((var_temp__blk936_dn5 * var_za) + (var_temp__blk936 * var_za_dn5))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * var_temp__blk936_dn5) * var_temp__blk936) + (assign43950_e56783 * var_temp__blk936_dn5)) * var_za) + (assign43950_e56785 * var_za_dn5)))) / (assign43950_e56788 * assign43950_e56788)))), ((((var_x_inf_dn6 * var_za) + (var_x_inf * var_za_dn6)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * var_temp__blk936_dn6) * assign43950_e56778) + (assign43950_e56773 * (-((var_temp__blk936_dn6 * var_za) + (var_temp__blk936 * var_za_dn6))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * var_temp__blk936_dn6) * var_temp__blk936) + (assign43950_e56783 * var_temp__blk936_dn6)) * var_za) + (assign43950_e56785 * var_za_dn6)))) / (assign43950_e56788 * assign43950_e56788)))), ((((var_x_inf_dn7 * var_za) + (var_x_inf * var_za_dn7)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * var_temp__blk936_dn7) * assign43950_e56778) + (assign43950_e56773 * (-((var_temp__blk936_dn7 * var_za) + (var_temp__blk936 * var_za_dn7))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * var_temp__blk936_dn7) * var_temp__blk936) + (assign43950_e56783 * var_temp__blk936_dn7)) * var_za) + (assign43950_e56785 * var_za_dn7)))) / (assign43950_e56788 * assign43950_e56788)))), ((((var_x_inf_dn8 * var_za) + (var_x_inf * var_za_dn8)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * var_temp__blk936_dn8) * assign43950_e56778) + (assign43950_e56773 * (-((var_temp__blk936_dn8 * var_za) + (var_temp__blk936 * var_za_dn8))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * var_temp__blk936_dn8) * var_temp__blk936) + (assign43950_e56783 * var_temp__blk936_dn8)) * var_za) + (assign43950_e56785 * var_za_dn8)))) / (assign43950_e56788 * assign43950_e56788)))),)
    } else {
        (var_x_0, var_x_0_dn5, var_x_0_dn6, var_x_0_dn7, var_x_0_dn8,)
    }
};
        var_x_0 = assign43950_e56793;
        var_x_0_dn5 = assign43950_e56793_d_n5;
        var_x_0_dn6 = assign43950_e56793_d_n6;
        var_x_0_dn7 = assign43950_e56793_d_n7;
        var_x_0_dn8 = assign43950_e56793_d_n8;

        let (assign43960_e56801, assign43960_e56801_d_n5, assign43960_e56801_d_n6, assign43960_e56801_d_n7, assign43960_e56801_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43960_e56799: f64 = (0.99 * var_x_0);
        (assign43960_e56799, (0.99 * var_x_0_dn5), (0.99 * var_x_0_dn6), (0.99 * var_x_0_dn7), (0.99 * var_x_0_dn8),)
    } else {
        (var_x_sat, var_x_sat_dn5, var_x_sat_dn6, var_x_sat_dn7, var_x_sat_dn8,)
    }
};
        var_x_sat = assign43960_e56801;
        var_x_sat_dn5 = assign43960_e56801_d_n5;
        var_x_sat_dn6 = assign43960_e56801_d_n6;
        var_x_sat_dn7 = assign43960_e56801_d_n7;
        var_x_sat_dn8 = assign43960_e56801_d_n8;

        let (assign43970_e56817, assign43970_e56817_d_n5, assign43970_e56817_d_n6, assign43970_e56817_d_n7, assign43970_e56817_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43970_e56809: f64 = (2.0 * var_asat);
        let assign43970_e56810: f64 = (var_x_sat - assign43970_e56809);
        let assign43970_e56811: f64 = (var_x_sat * assign43970_e56810);
        let assign43970_e56813: f64 = (assign43970_e56811 * var_inv_gf2);
        let assign43970_e56815: f64 = (assign43970_e56813 / var_ds);
        (assign43970_e56815, (((((((var_x_sat_dn5 * assign43970_e56810) + (var_x_sat * (var_x_sat_dn5 - (2.0 * var_asat_dn5)))) * var_inv_gf2) + (assign43970_e56811 * var_inv_gf2_dn5)) * var_ds) - (assign43970_e56813 * var_ds_dn5)) / (var_ds * var_ds)), (((((((var_x_sat_dn6 * assign43970_e56810) + (var_x_sat * (var_x_sat_dn6 - (2.0 * var_asat_dn6)))) * var_inv_gf2) + (assign43970_e56811 * var_inv_gf2_dn6)) * var_ds) - (assign43970_e56813 * var_ds_dn6)) / (var_ds * var_ds)), (((((((var_x_sat_dn7 * assign43970_e56810) + (var_x_sat * (var_x_sat_dn7 - (2.0 * var_asat_dn7)))) * var_inv_gf2) + (assign43970_e56811 * var_inv_gf2_dn7)) * var_ds) - (assign43970_e56813 * var_ds_dn7)) / (var_ds * var_ds)), (((((((var_x_sat_dn8 * assign43970_e56810) + (var_x_sat * (var_x_sat_dn8 - (2.0 * var_asat_dn8)))) * var_inv_gf2) + (assign43970_e56811 * var_inv_gf2_dn8)) * var_ds) - (assign43970_e56813 * var_ds_dn8)) / (var_ds * var_ds)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43970_e56817;
        var_temp__blk936_dn5 = assign43970_e56817_d_n5;
        var_temp__blk936_dn6 = assign43970_e56817_d_n6;
        var_temp__blk936_dn7 = assign43970_e56817_d_n7;
        var_temp__blk936_dn8 = assign43970_e56817_d_n8;

        let (assign43980_e56837, assign43980_e56837_d_n5, assign43980_e56837_d_n6, assign43980_e56837_d_n7, assign43980_e56837_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43980_e56826: f64 = (-0.99);
        let (assign43980_e56831, assign43980_e56831_d_n5, assign43980_e56831_d_n6, assign43980_e56831_d_n7, assign43980_e56831_d_n8,) = {
            if (var_temp__blk936 > assign43980_e56826) {
                (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
            } else {
                let assign43980_e56830: f64 = (-0.99);
                (assign43980_e56830, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign43980_e56832: f64 = (1.0 + assign43980_e56831);
        let assign43980_e56833: f64 = (assign43980_e56832).ln();
        let assign43980_e56834: f64 = (var_x_sat - assign43980_e56833);
        let assign43980_e56835: f64 = (var_phit1 * assign43980_e56834);
        (assign43980_e56835, ((var_phit1_dn5 * assign43980_e56834) + (var_phit1 * (var_x_sat_dn5 - (assign43980_e56831_d_n5 / assign43980_e56832)))), ((var_phit1_dn6 * assign43980_e56834) + (var_phit1 * (var_x_sat_dn6 - (assign43980_e56831_d_n6 / assign43980_e56832)))), ((var_phit1_dn7 * assign43980_e56834) + (var_phit1 * (var_x_sat_dn7 - (assign43980_e56831_d_n7 / assign43980_e56832)))), ((var_phit1_dn8 * assign43980_e56834) + (var_phit1 * (var_x_sat_dn8 - (assign43980_e56831_d_n8 / assign43980_e56832)))),)
    } else {
        (var_v_dsat, var_v_dsat_dn5, var_v_dsat_dn6, var_v_dsat_dn7, var_v_dsat_dn8,)
    }
};
        var_v_dsat = assign43980_e56837;
        var_v_dsat_dn5 = assign43980_e56837_d_n5;
        var_v_dsat_dn6 = assign43980_e56837_d_n6;
        var_v_dsat_dn7 = assign43980_e56837_d_n7;
        var_v_dsat_dn8 = assign43980_e56837_d_n8;

        let (assign43990_e56844, assign43990_e56844_d_n5, assign43990_e56844_d_n6, assign43990_e56844_d_n7, assign43990_e56844_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 == 0.0)) {
        (var_vdsat_lim, var_vdsat_lim_dn5, var_vdsat_lim_dn6, var_vdsat_lim_dn7, var_vdsat_lim_dn8,)
    } else {
        (var_v_dsat, var_v_dsat_dn5, var_v_dsat_dn6, var_v_dsat_dn7, var_v_dsat_dn8,)
    }
};
        var_v_dsat = assign43990_e56844;
        var_v_dsat_dn5 = assign43990_e56844_d_n5;
        var_v_dsat_dn6 = assign43990_e56844_d_n6;
        var_v_dsat_dn7 = assign43990_e56844_d_n7;
        var_v_dsat_dn8 = assign43990_e56844_d_n8;

        let (assign44000_e56850, assign44000_e56850_d_n5, assign44000_e56850_d_n6, assign44000_e56850_d_n7, assign44000_e56850_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44000_e56848: f64 = (1.0 + var_arloc);
        (assign44000_e56848, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44000_e56850;
        var_temp__blk936_dn5 = assign44000_e56850_d_n5;
        var_temp__blk936_dn6 = assign44000_e56850_d_n6;
        var_temp__blk936_dn7 = assign44000_e56850_d_n7;
        var_temp__blk936_dn8 = assign44000_e56850_d_n8;

        let (assign44010_e56859, assign44010_e56859_d_n5, assign44010_e56859_d_n6, assign44010_e56859_d_n7, assign44010_e56859_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44010_e56853: f64 = (var_temp__blk936).sqrt();
        let assign44010_e56855: f64 = (assign44010_e56853 * var_v_ds);
        let assign44010_e56857: f64 = (assign44010_e56855 / var_v_dsat);
        (assign44010_e56857, (((((var_temp__blk936_dn5 / (2.0 * assign44010_e56853)) * var_v_ds) * var_v_dsat) - (assign44010_e56855 * var_v_dsat_dn5)) / (var_v_dsat * var_v_dsat)), ((((((var_temp__blk936_dn6 / (2.0 * assign44010_e56853)) * var_v_ds) + (assign44010_e56853 * var_v_ds_dn6)) * var_v_dsat) - (assign44010_e56855 * var_v_dsat_dn6)) / (var_v_dsat * var_v_dsat)), ((((((var_temp__blk936_dn7 / (2.0 * assign44010_e56853)) * var_v_ds) + (assign44010_e56853 * var_v_ds_dn7)) * var_v_dsat) - (assign44010_e56855 * var_v_dsat_dn7)) / (var_v_dsat * var_v_dsat)), (((((var_temp__blk936_dn8 / (2.0 * assign44010_e56853)) * var_v_ds) * var_v_dsat) - (assign44010_e56855 * var_v_dsat_dn8)) / (var_v_dsat * var_v_dsat)),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign44010_e56859;
        var_temp1_dn5 = assign44010_e56859_d_n5;
        var_temp1_dn6 = assign44010_e56859_d_n6;
        var_temp1_dn7 = assign44010_e56859_d_n7;
        var_temp1_dn8 = assign44010_e56859_d_n8;

        let (assign44020_e56867, assign44020_e56867_d_n5, assign44020_e56867_d_n6, assign44020_e56867_d_n7, assign44020_e56867_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44020_e56863: f64 = (var_temp1 * var_temp1);
        let assign44020_e56865: f64 = (assign44020_e56863 + var_temp__blk936);
        (assign44020_e56865, (((var_temp1_dn5 * var_temp1) + (var_temp1 * var_temp1_dn5)) + var_temp__blk936_dn5), (((var_temp1_dn6 * var_temp1) + (var_temp1 * var_temp1_dn6)) + var_temp__blk936_dn6), (((var_temp1_dn7 * var_temp1) + (var_temp1 * var_temp1_dn7)) + var_temp__blk936_dn7), (((var_temp1_dn8 * var_temp1) + (var_temp1 * var_temp1_dn8)) + var_temp__blk936_dn8),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign44020_e56867;
        var_temp2_dn5 = assign44020_e56867_d_n5;
        var_temp2_dn6 = assign44020_e56867_d_n6;
        var_temp2_dn7 = assign44020_e56867_d_n7;
        var_temp2_dn8 = assign44020_e56867_d_n8;

        let (assign44030_e56873, assign44030_e56873_d_n5, assign44030_e56873_d_n6, assign44030_e56873_d_n7, assign44030_e56873_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44030_e56871: f64 = (2.0 * var_temp1);
        (assign44030_e56871, (2.0 * var_temp1_dn5), (2.0 * var_temp1_dn6), (2.0 * var_temp1_dn7), (2.0 * var_temp1_dn8),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44030_e56873;
        var_temp__blk936_dn5 = assign44030_e56873_d_n5;
        var_temp__blk936_dn6 = assign44030_e56873_d_n6;
        var_temp__blk936_dn7 = assign44030_e56873_d_n7;
        var_temp__blk936_dn8 = assign44030_e56873_d_n8;

        *var_alphasat_slot = var_alphasat;
        *var_alphasat_dn5_slot = var_alphasat_dn5;
        *var_alphasat_dn6_slot = var_alphasat_dn6;
        *var_alphasat_dn7_slot = var_alphasat_dn7;
        *var_alphasat_dn8_slot = var_alphasat_dn8;
        *var_delta_gmob_slot = var_delta_gmob;
        *var_delta_gmob_dn5_slot = var_delta_gmob_dn5;
        *var_delta_gmob_dn6_slot = var_delta_gmob_dn6;
        *var_delta_gmob_dn7_slot = var_delta_gmob_dn7;
        *var_delta_gmob_dn8_slot = var_delta_gmob_dn8;
        *var_gmobcssat_slot = var_gmobcssat;
        *var_gmobcssat_dn5_slot = var_gmobcssat_dn5;
        *var_gmobcssat_dn6_slot = var_gmobcssat_dn6;
        *var_gmobcssat_dn7_slot = var_gmobcssat_dn7;
        *var_gmobcssat_dn8_slot = var_gmobcssat_dn8;
        *var_gmobmusat_slot = var_gmobmusat;
        *var_gmobmusat_dn5_slot = var_gmobmusat_dn5;
        *var_gmobmusat_dn6_slot = var_gmobmusat_dn6;
        *var_gmobmusat_dn7_slot = var_gmobmusat_dn7;
        *var_gmobmusat_dn8_slot = var_gmobmusat_dn8;
        *var_grsat_slot = var_grsat;
        *var_grsat_dn5_slot = var_grsat_dn5;
        *var_grsat_dn6_slot = var_grsat_dn6;
        *var_grsat_dn7_slot = var_grsat_dn7;
        *var_grsat_dn8_slot = var_grsat_dn8;
        *var_guard1202_slot = var_guard1202;
        *var_guard1203_slot = var_guard1203;
        *var_qbsat_slot = var_qbsat;
        *var_qbsat_dn5_slot = var_qbsat_dn5;
        *var_qbsat_dn6_slot = var_qbsat_dn6;
        *var_qbsat_dn7_slot = var_qbsat_dn7;
        *var_qbsat_dn8_slot = var_qbsat_dn8;
        *var_qisat_slot = var_qisat;
        *var_qisat_dn5_slot = var_qisat_dn5;
        *var_qisat_dn6_slot = var_qisat_dn6;
        *var_qisat_dn7_slot = var_qisat_dn7;
        *var_qisat_dn8_slot = var_qisat_dn8;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_v_dsat_slot = var_v_dsat;
        *var_v_dsat_dn5_slot = var_v_dsat_dn5;
        *var_v_dsat_dn6_slot = var_v_dsat_dn6;
        *var_v_dsat_dn7_slot = var_v_dsat_dn7;
        *var_v_dsat_dn8_slot = var_v_dsat_dn8;
        *var_x_0_slot = var_x_0;
        *var_x_0_dn5_slot = var_x_0_dn5;
        *var_x_0_dn6_slot = var_x_0_dn6;
        *var_x_0_dn7_slot = var_x_0_dn7;
        *var_x_0_dn8_slot = var_x_0_dn8;
        *var_x_inf_slot = var_x_inf;
        *var_x_inf_dn5_slot = var_x_inf_dn5;
        *var_x_inf_dn6_slot = var_x_inf_dn6;
        *var_x_inf_dn7_slot = var_x_inf_dn7;
        *var_x_inf_dn8_slot = var_x_inf_dn8;
        *var_x_sat_slot = var_x_sat;
        *var_x_sat_dn5_slot = var_x_sat_dn5;
        *var_x_sat_dn6_slot = var_x_sat_dn6;
        *var_x_sat_dn7_slot = var_x_sat_dn7;
        *var_x_sat_dn8_slot = var_x_sat_dn8;
        *var_ysat_slot = var_ysat;
        *var_ysat_dn5_slot = var_ysat_dn5;
        *var_ysat_dn6_slot = var_ysat_dn6;
        *var_ysat_dn7_slot = var_ysat_dn7;
        *var_ysat_dn8_slot = var_ysat_dn8;
        *var_za_slot = var_za;
        *var_za_dn5_slot = var_za_dn5;
        *var_za_dn6_slot = var_za_dn6;
        *var_za_dn7_slot = var_za_dn7;
        *var_za_dn8_slot = var_za_dn8;
    }

    pub(super) fn stamp_transient_block_95(
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1197: f64,
        var_inv_phit1: f64,
        var_inv_phit1_dn5: f64,
        var_inv_phit1_dn6: f64,
        var_inv_phit1_dn7: f64,
        var_inv_phit1_dn8: f64,
        var_inv_xi: f64,
        var_inv_xi_dn5: f64,
        var_inv_xi_dn6: f64,
        var_inv_xi_dn7: f64,
        var_inv_xi_dn8: f64,
        var_margin: f64,
        var_sp_s_x1: f64,
        var_sp_s_x1_dn5: f64,
        var_sp_s_x1_dn6: f64,
        var_sp_s_x1_dn7: f64,
        var_sp_s_x1_dn8: f64,
        var_temp2: f64,
        var_temp2_dn5: f64,
        var_temp2_dn6: f64,
        var_temp2_dn7: f64,
        var_temp2_dn8: f64,
        var_temp__blk936: f64,
        var_temp__blk936_dn5: f64,
        var_temp__blk936_dn6: f64,
        var_temp__blk936_dn7: f64,
        var_temp__blk936_dn8: f64,
        var_v_dsat: f64,
        var_v_dsat_dn5: f64,
        var_v_dsat_dn6: f64,
        var_v_dsat_dn7: f64,
        var_v_dsat_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xn_s: f64,
        var_xn_s_dn5: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_delta_nd_slot: &mut f64,
        var_delta_nd_dn5_slot: &mut f64,
        var_delta_nd_dn6_slot: &mut f64,
        var_delta_nd_dn7_slot: &mut f64,
        var_delta_nd_dn8_slot: &mut f64,
        var_guard1204_slot: &mut f64,
        var_guard1205_slot: &mut f64,
        var_guard1206_slot: &mut f64,
        var_guard1207_slot: &mut f64,
        var_k_ds_slot: &mut f64,
        var_k_ds_dn5_slot: &mut f64,
        var_k_ds_dn6_slot: &mut f64,
        var_k_ds_dn7_slot: &mut f64,
        var_k_ds_dn8_slot: &mut f64,
        var_mutau_slot: &mut f64,
        var_mutau_dn5_slot: &mut f64,
        var_mutau_dn6_slot: &mut f64,
        var_mutau_dn7_slot: &mut f64,
        var_mutau_dn8_slot: &mut f64,
        var_nu_slot: &mut f64,
        var_nu_dn5_slot: &mut f64,
        var_nu_dn6_slot: &mut f64,
        var_nu_dn7_slot: &mut f64,
        var_nu_dn8_slot: &mut f64,
        var_sp_s_a_slot: &mut f64,
        var_sp_s_a_dn5_slot: &mut f64,
        var_sp_s_a_dn6_slot: &mut f64,
        var_sp_s_a_dn7_slot: &mut f64,
        var_sp_s_a_dn8_slot: &mut f64,
        var_sp_s_b_slot: &mut f64,
        var_sp_s_b_dn5_slot: &mut f64,
        var_sp_s_b_dn6_slot: &mut f64,
        var_sp_s_b_dn7_slot: &mut f64,
        var_sp_s_b_dn8_slot: &mut f64,
        var_sp_s_bx_slot: &mut f64,
        var_sp_s_bx_dn5_slot: &mut f64,
        var_sp_s_bx_dn6_slot: &mut f64,
        var_sp_s_bx_dn7_slot: &mut f64,
        var_sp_s_bx_dn8_slot: &mut f64,
        var_sp_s_c_slot: &mut f64,
        var_sp_s_c_dn5_slot: &mut f64,
        var_sp_s_c_dn6_slot: &mut f64,
        var_sp_s_c_dn7_slot: &mut f64,
        var_sp_s_c_dn8_slot: &mut f64,
        var_sp_s_delta0_slot: &mut f64,
        var_sp_s_delta0_dn5_slot: &mut f64,
        var_sp_s_delta0_dn6_slot: &mut f64,
        var_sp_s_delta0_dn7_slot: &mut f64,
        var_sp_s_delta0_dn8_slot: &mut f64,
        var_sp_s_delta1_slot: &mut f64,
        var_sp_s_delta1_dn5_slot: &mut f64,
        var_sp_s_delta1_dn6_slot: &mut f64,
        var_sp_s_delta1_dn7_slot: &mut f64,
        var_sp_s_delta1_dn8_slot: &mut f64,
        var_sp_s_eta_slot: &mut f64,
        var_sp_s_eta_dn5_slot: &mut f64,
        var_sp_s_eta_dn6_slot: &mut f64,
        var_sp_s_eta_dn7_slot: &mut f64,
        var_sp_s_eta_dn8_slot: &mut f64,
        var_sp_s_tau_slot: &mut f64,
        var_sp_s_tau_dn5_slot: &mut f64,
        var_sp_s_tau_dn6_slot: &mut f64,
        var_sp_s_tau_dn7_slot: &mut f64,
        var_sp_s_tau_dn8_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp1_slot: &mut f64,
        var_sp_s_temp1_dn5_slot: &mut f64,
        var_sp_s_temp1_dn6_slot: &mut f64,
        var_sp_s_temp1_dn7_slot: &mut f64,
        var_sp_s_temp1_dn8_slot: &mut f64,
        var_sp_s_temp2_slot: &mut f64,
        var_sp_s_temp2_dn5_slot: &mut f64,
        var_sp_s_temp2_dn6_slot: &mut f64,
        var_sp_s_temp2_dn7_slot: &mut f64,
        var_sp_s_temp2_dn8_slot: &mut f64,
        var_sp_s_temp_dn5_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_x0_slot: &mut f64,
        var_sp_s_x0_dn5_slot: &mut f64,
        var_sp_s_x0_dn6_slot: &mut f64,
        var_sp_s_x0_dn7_slot: &mut f64,
        var_sp_s_x0_dn8_slot: &mut f64,
        var_sp_s_xi0_slot: &mut f64,
        var_sp_s_xi0_dn5_slot: &mut f64,
        var_sp_s_xi0_dn6_slot: &mut f64,
        var_sp_s_xi0_dn7_slot: &mut f64,
        var_sp_s_xi0_dn8_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn5_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn5_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
        var_udse_slot: &mut f64,
        var_udse_dn5_slot: &mut f64,
        var_udse_dn6_slot: &mut f64,
        var_udse_dn7_slot: &mut f64,
        var_udse_dn8_slot: &mut f64,
        var_vdse_slot: &mut f64,
        var_vdse_dn5_slot: &mut f64,
        var_vdse_dn6_slot: &mut f64,
        var_vdse_dn7_slot: &mut f64,
        var_vdse_dn8_slot: &mut f64,
        var_x_d_slot: &mut f64,
        var_x_d_dn5_slot: &mut f64,
        var_x_d_dn6_slot: &mut f64,
        var_x_d_dn7_slot: &mut f64,
        var_x_d_dn8_slot: &mut f64,
        var_xn_d_slot: &mut f64,
        var_xn_d_dn5_slot: &mut f64,
        var_xn_d_dn6_slot: &mut f64,
        var_xn_d_dn7_slot: &mut f64,
        var_xn_d_dn8_slot: &mut f64,
    ) {
        let mut var_delta_nd: f64 = *var_delta_nd_slot;
        let mut var_delta_nd_dn5: f64 = *var_delta_nd_dn5_slot;
        let mut var_delta_nd_dn6: f64 = *var_delta_nd_dn6_slot;
        let mut var_delta_nd_dn7: f64 = *var_delta_nd_dn7_slot;
        let mut var_delta_nd_dn8: f64 = *var_delta_nd_dn8_slot;
        let mut var_guard1204: f64 = *var_guard1204_slot;
        let mut var_guard1205: f64 = *var_guard1205_slot;
        let mut var_guard1206: f64 = *var_guard1206_slot;
        let mut var_guard1207: f64 = *var_guard1207_slot;
        let mut var_k_ds: f64 = *var_k_ds_slot;
        let mut var_k_ds_dn5: f64 = *var_k_ds_dn5_slot;
        let mut var_k_ds_dn6: f64 = *var_k_ds_dn6_slot;
        let mut var_k_ds_dn7: f64 = *var_k_ds_dn7_slot;
        let mut var_k_ds_dn8: f64 = *var_k_ds_dn8_slot;
        let mut var_mutau: f64 = *var_mutau_slot;
        let mut var_mutau_dn5: f64 = *var_mutau_dn5_slot;
        let mut var_mutau_dn6: f64 = *var_mutau_dn6_slot;
        let mut var_mutau_dn7: f64 = *var_mutau_dn7_slot;
        let mut var_mutau_dn8: f64 = *var_mutau_dn8_slot;
        let mut var_nu: f64 = *var_nu_slot;
        let mut var_nu_dn5: f64 = *var_nu_dn5_slot;
        let mut var_nu_dn6: f64 = *var_nu_dn6_slot;
        let mut var_nu_dn7: f64 = *var_nu_dn7_slot;
        let mut var_nu_dn8: f64 = *var_nu_dn8_slot;
        let mut var_sp_s_a: f64 = *var_sp_s_a_slot;
        let mut var_sp_s_a_dn5: f64 = *var_sp_s_a_dn5_slot;
        let mut var_sp_s_a_dn6: f64 = *var_sp_s_a_dn6_slot;
        let mut var_sp_s_a_dn7: f64 = *var_sp_s_a_dn7_slot;
        let mut var_sp_s_a_dn8: f64 = *var_sp_s_a_dn8_slot;
        let mut var_sp_s_b: f64 = *var_sp_s_b_slot;
        let mut var_sp_s_b_dn5: f64 = *var_sp_s_b_dn5_slot;
        let mut var_sp_s_b_dn6: f64 = *var_sp_s_b_dn6_slot;
        let mut var_sp_s_b_dn7: f64 = *var_sp_s_b_dn7_slot;
        let mut var_sp_s_b_dn8: f64 = *var_sp_s_b_dn8_slot;
        let mut var_sp_s_bx: f64 = *var_sp_s_bx_slot;
        let mut var_sp_s_bx_dn5: f64 = *var_sp_s_bx_dn5_slot;
        let mut var_sp_s_bx_dn6: f64 = *var_sp_s_bx_dn6_slot;
        let mut var_sp_s_bx_dn7: f64 = *var_sp_s_bx_dn7_slot;
        let mut var_sp_s_bx_dn8: f64 = *var_sp_s_bx_dn8_slot;
        let mut var_sp_s_c: f64 = *var_sp_s_c_slot;
        let mut var_sp_s_c_dn5: f64 = *var_sp_s_c_dn5_slot;
        let mut var_sp_s_c_dn6: f64 = *var_sp_s_c_dn6_slot;
        let mut var_sp_s_c_dn7: f64 = *var_sp_s_c_dn7_slot;
        let mut var_sp_s_c_dn8: f64 = *var_sp_s_c_dn8_slot;
        let mut var_sp_s_delta0: f64 = *var_sp_s_delta0_slot;
        let mut var_sp_s_delta0_dn5: f64 = *var_sp_s_delta0_dn5_slot;
        let mut var_sp_s_delta0_dn6: f64 = *var_sp_s_delta0_dn6_slot;
        let mut var_sp_s_delta0_dn7: f64 = *var_sp_s_delta0_dn7_slot;
        let mut var_sp_s_delta0_dn8: f64 = *var_sp_s_delta0_dn8_slot;
        let mut var_sp_s_delta1: f64 = *var_sp_s_delta1_slot;
        let mut var_sp_s_delta1_dn5: f64 = *var_sp_s_delta1_dn5_slot;
        let mut var_sp_s_delta1_dn6: f64 = *var_sp_s_delta1_dn6_slot;
        let mut var_sp_s_delta1_dn7: f64 = *var_sp_s_delta1_dn7_slot;
        let mut var_sp_s_delta1_dn8: f64 = *var_sp_s_delta1_dn8_slot;
        let mut var_sp_s_eta: f64 = *var_sp_s_eta_slot;
        let mut var_sp_s_eta_dn5: f64 = *var_sp_s_eta_dn5_slot;
        let mut var_sp_s_eta_dn6: f64 = *var_sp_s_eta_dn6_slot;
        let mut var_sp_s_eta_dn7: f64 = *var_sp_s_eta_dn7_slot;
        let mut var_sp_s_eta_dn8: f64 = *var_sp_s_eta_dn8_slot;
        let mut var_sp_s_tau: f64 = *var_sp_s_tau_slot;
        let mut var_sp_s_tau_dn5: f64 = *var_sp_s_tau_dn5_slot;
        let mut var_sp_s_tau_dn6: f64 = *var_sp_s_tau_dn6_slot;
        let mut var_sp_s_tau_dn7: f64 = *var_sp_s_tau_dn7_slot;
        let mut var_sp_s_tau_dn8: f64 = *var_sp_s_tau_dn8_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp1: f64 = *var_sp_s_temp1_slot;
        let mut var_sp_s_temp1_dn5: f64 = *var_sp_s_temp1_dn5_slot;
        let mut var_sp_s_temp1_dn6: f64 = *var_sp_s_temp1_dn6_slot;
        let mut var_sp_s_temp1_dn7: f64 = *var_sp_s_temp1_dn7_slot;
        let mut var_sp_s_temp1_dn8: f64 = *var_sp_s_temp1_dn8_slot;
        let mut var_sp_s_temp2: f64 = *var_sp_s_temp2_slot;
        let mut var_sp_s_temp2_dn5: f64 = *var_sp_s_temp2_dn5_slot;
        let mut var_sp_s_temp2_dn6: f64 = *var_sp_s_temp2_dn6_slot;
        let mut var_sp_s_temp2_dn7: f64 = *var_sp_s_temp2_dn7_slot;
        let mut var_sp_s_temp2_dn8: f64 = *var_sp_s_temp2_dn8_slot;
        let mut var_sp_s_temp_dn5: f64 = *var_sp_s_temp_dn5_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_x0: f64 = *var_sp_s_x0_slot;
        let mut var_sp_s_x0_dn5: f64 = *var_sp_s_x0_dn5_slot;
        let mut var_sp_s_x0_dn6: f64 = *var_sp_s_x0_dn6_slot;
        let mut var_sp_s_x0_dn7: f64 = *var_sp_s_x0_dn7_slot;
        let mut var_sp_s_x0_dn8: f64 = *var_sp_s_x0_dn8_slot;
        let mut var_sp_s_xi0: f64 = *var_sp_s_xi0_slot;
        let mut var_sp_s_xi0_dn5: f64 = *var_sp_s_xi0_dn5_slot;
        let mut var_sp_s_xi0_dn6: f64 = *var_sp_s_xi0_dn6_slot;
        let mut var_sp_s_xi0_dn7: f64 = *var_sp_s_xi0_dn7_slot;
        let mut var_sp_s_xi0_dn8: f64 = *var_sp_s_xi0_dn8_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn5: f64 = *var_sp_s_xi1_dn5_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn5: f64 = *var_sp_s_xi2_dn5_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;
        let mut var_udse: f64 = *var_udse_slot;
        let mut var_udse_dn5: f64 = *var_udse_dn5_slot;
        let mut var_udse_dn6: f64 = *var_udse_dn6_slot;
        let mut var_udse_dn7: f64 = *var_udse_dn7_slot;
        let mut var_udse_dn8: f64 = *var_udse_dn8_slot;
        let mut var_vdse: f64 = *var_vdse_slot;
        let mut var_vdse_dn5: f64 = *var_vdse_dn5_slot;
        let mut var_vdse_dn6: f64 = *var_vdse_dn6_slot;
        let mut var_vdse_dn7: f64 = *var_vdse_dn7_slot;
        let mut var_vdse_dn8: f64 = *var_vdse_dn8_slot;
        let mut var_x_d: f64 = *var_x_d_slot;
        let mut var_x_d_dn5: f64 = *var_x_d_dn5_slot;
        let mut var_x_d_dn6: f64 = *var_x_d_dn6_slot;
        let mut var_x_d_dn7: f64 = *var_x_d_dn7_slot;
        let mut var_x_d_dn8: f64 = *var_x_d_dn8_slot;
        let mut var_xn_d: f64 = *var_xn_d_slot;
        let mut var_xn_d_dn5: f64 = *var_xn_d_dn5_slot;
        let mut var_xn_d_dn6: f64 = *var_xn_d_dn6_slot;
        let mut var_xn_d_dn7: f64 = *var_xn_d_dn7_slot;
        let mut var_xn_d_dn8: f64 = *var_xn_d_dn8_slot;

        let (assign44040_e56889, assign44040_e56889_d_n5, assign44040_e56889_d_n6, assign44040_e56889_d_n7, assign44040_e56889_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44040_e56877: f64 = (var_v_dsat * var_temp__blk936);
        let assign44040_e56880: f64 = (var_temp2 - var_temp__blk936);
        let assign44040_e56881: f64 = (assign44040_e56880).sqrt();
        let assign44040_e56884: f64 = (var_temp2 + var_temp__blk936);
        let assign44040_e56885: f64 = (assign44040_e56884).sqrt();
        let assign44040_e56886: f64 = (assign44040_e56881 + assign44040_e56885);
        let assign44040_e56887: f64 = (assign44040_e56877 / assign44040_e56886);
        (assign44040_e56887, (((((var_v_dsat_dn5 * var_temp__blk936) + (var_v_dsat * var_temp__blk936_dn5)) * assign44040_e56886) - (assign44040_e56877 * (((var_temp2_dn5 - var_temp__blk936_dn5) / (2.0 * assign44040_e56881)) + ((var_temp2_dn5 + var_temp__blk936_dn5) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((var_v_dsat_dn6 * var_temp__blk936) + (var_v_dsat * var_temp__blk936_dn6)) * assign44040_e56886) - (assign44040_e56877 * (((var_temp2_dn6 - var_temp__blk936_dn6) / (2.0 * assign44040_e56881)) + ((var_temp2_dn6 + var_temp__blk936_dn6) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((var_v_dsat_dn7 * var_temp__blk936) + (var_v_dsat * var_temp__blk936_dn7)) * assign44040_e56886) - (assign44040_e56877 * (((var_temp2_dn7 - var_temp__blk936_dn7) / (2.0 * assign44040_e56881)) + ((var_temp2_dn7 + var_temp__blk936_dn7) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((var_v_dsat_dn8 * var_temp__blk936) + (var_v_dsat * var_temp__blk936_dn8)) * assign44040_e56886) - (assign44040_e56877 * (((var_temp2_dn8 - var_temp__blk936_dn8) / (2.0 * assign44040_e56881)) + ((var_temp2_dn8 + var_temp__blk936_dn8) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)),)
    } else {
        (var_vdse, var_vdse_dn5, var_vdse_dn6, var_vdse_dn7, var_vdse_dn8,)
    }
};
        var_vdse = assign44040_e56889;
        var_vdse_dn5 = assign44040_e56889_d_n5;
        var_vdse_dn6 = assign44040_e56889_d_n6;
        var_vdse_dn7 = assign44040_e56889_d_n7;
        var_vdse_dn8 = assign44040_e56889_d_n8;

        let (assign44050_e56895, assign44050_e56895_d_n5, assign44050_e56895_d_n6, assign44050_e56895_d_n7, assign44050_e56895_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44050_e56893: f64 = (var_vdse * var_inv_phit1);
        (assign44050_e56893, ((var_vdse_dn5 * var_inv_phit1) + (var_vdse * var_inv_phit1_dn5)), ((var_vdse_dn6 * var_inv_phit1) + (var_vdse * var_inv_phit1_dn6)), ((var_vdse_dn7 * var_inv_phit1) + (var_vdse * var_inv_phit1_dn7)), ((var_vdse_dn8 * var_inv_phit1) + (var_vdse * var_inv_phit1_dn8)),)
    } else {
        (var_udse, var_udse_dn5, var_udse_dn6, var_udse_dn7, var_udse_dn8,)
    }
};
        var_udse = assign44050_e56895;
        var_udse_dn5 = assign44050_e56895_d_n5;
        var_udse_dn6 = assign44050_e56895_d_n6;
        var_udse_dn7 = assign44050_e56895_d_n7;
        var_udse_dn8 = assign44050_e56895_d_n8;

        let (assign44060_e56901, assign44060_e56901_d_n5, assign44060_e56901_d_n6, assign44060_e56901_d_n7, assign44060_e56901_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44060_e56899: f64 = (var_xn_s + var_udse);
        (assign44060_e56899, (var_xn_s_dn5 + var_udse_dn5), (var_xn_s_dn6 + var_udse_dn6), (var_xn_s_dn7 + var_udse_dn7), (var_xn_s_dn8 + var_udse_dn8),)
    } else {
        (var_xn_d, var_xn_d_dn5, var_xn_d_dn6, var_xn_d_dn7, var_xn_d_dn8,)
    }
};
        var_xn_d = assign44060_e56901;
        var_xn_d_dn5 = assign44060_e56901_d_n5;
        var_xn_d_dn6 = assign44060_e56901_d_n6;
        var_xn_d_dn7 = assign44060_e56901_d_n7;
        var_xn_d_dn8 = assign44060_e56901_d_n8;

        let assign44070_e56904: f64 = if var_udse < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1204 = assign44070_e56904;

        let (assign44080_e56912, assign44080_e56912_d_n5, assign44080_e56912_d_n6, assign44080_e56912_d_n7, assign44080_e56912_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1204 != 0.0)) {
        let assign44080_e56909: f64 = (-var_udse);
        let assign44080_e56910: f64 = (assign44080_e56909).exp();
        (assign44080_e56910, (assign44080_e56910 * (-var_udse_dn5)), (assign44080_e56910 * (-var_udse_dn6)), (assign44080_e56910 * (-var_udse_dn7)), (assign44080_e56910 * (-var_udse_dn8)),)
    } else {
        (var_k_ds, var_k_ds_dn5, var_k_ds_dn6, var_k_ds_dn7, var_k_ds_dn8,)
    }
};
        var_k_ds = assign44080_e56912;
        var_k_ds_dn5 = assign44080_e56912_d_n5;
        var_k_ds_dn6 = assign44080_e56912_d_n6;
        var_k_ds_dn7 = assign44080_e56912_d_n7;
        var_k_ds_dn8 = assign44080_e56912_d_n8;

        let (assign44090_e56941, assign44090_e56941_d_n5, assign44090_e56941_d_n6, assign44090_e56941_d_n7, assign44090_e56941_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1204 == 0.0)) {
        let assign44090_e56921: f64 = (var_udse - 460.51701859880916);
        let assign44090_e56926: f64 = (var_udse - 460.51701859880916);
        let assign44090_e56930: f64 = (var_udse - 460.51701859880916);
        let assign44090_e56932: f64 = (assign44090_e56930 * 0.3333333333333333);
        let assign44090_e56933: f64 = (1.0 + assign44090_e56932);
        let assign44090_e56934: f64 = (assign44090_e56926 * assign44090_e56933);
        let assign44090_e56935: f64 = (0.5 * assign44090_e56934);
        let assign44090_e56936: f64 = (1.0 + assign44090_e56935);
        let assign44090_e56937: f64 = (assign44090_e56921 * assign44090_e56936);
        let assign44090_e56938: f64 = (1.0 + assign44090_e56937);
        let assign44090_e56939: f64 = (1e-200 / assign44090_e56938);
        (assign44090_e56939, (-((1e-200 * ((var_udse_dn5 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((var_udse_dn5 * assign44090_e56933) + (assign44090_e56926 * (var_udse_dn5 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((var_udse_dn6 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((var_udse_dn6 * assign44090_e56933) + (assign44090_e56926 * (var_udse_dn6 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((var_udse_dn7 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((var_udse_dn7 * assign44090_e56933) + (assign44090_e56926 * (var_udse_dn7 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((var_udse_dn8 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((var_udse_dn8 * assign44090_e56933) + (assign44090_e56926 * (var_udse_dn8 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))),)
    } else {
        (var_k_ds, var_k_ds_dn5, var_k_ds_dn6, var_k_ds_dn7, var_k_ds_dn8,)
    }
};
        var_k_ds = assign44090_e56941;
        var_k_ds_dn5 = assign44090_e56941_d_n5;
        var_k_ds_dn6 = assign44090_e56941_d_n6;
        var_k_ds_dn7 = assign44090_e56941_d_n7;
        var_k_ds_dn8 = assign44090_e56941_d_n8;

        let (assign44100_e56947, assign44100_e56947_d_n5, assign44100_e56947_d_n6, assign44100_e56947_d_n7, assign44100_e56947_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44100_e56945: f64 = (var_delta_ns * var_k_ds);
        (assign44100_e56945, ((var_delta_ns_dn5 * var_k_ds) + (var_delta_ns * var_k_ds_dn5)), ((var_delta_ns_dn6 * var_k_ds) + (var_delta_ns * var_k_ds_dn6)), ((var_delta_ns_dn7 * var_k_ds) + (var_delta_ns * var_k_ds_dn7)), ((var_delta_ns_dn8 * var_k_ds) + (var_delta_ns * var_k_ds_dn8)),)
    } else {
        (var_delta_nd, var_delta_nd_dn5, var_delta_nd_dn6, var_delta_nd_dn7, var_delta_nd_dn8,)
    }
};
        var_delta_nd = assign44100_e56947;
        var_delta_nd_dn5 = assign44100_e56947_d_n5;
        var_delta_nd_dn6 = assign44100_e56947_d_n6;
        var_delta_nd_dn7 = assign44100_e56947_d_n7;
        var_delta_nd_dn8 = assign44100_e56947_d_n8;

        let assign44110_e56949: f64 = (var_xg).abs();
        let assign44110_e56951: f64 = if assign44110_e56949 <= var_margin { 1.0 } else { 0.0 };
        var_guard1205 = assign44110_e56951;

        let (assign44120_e56963, assign44120_e56963_d_n5, assign44120_e56963_d_n6, assign44120_e56963_d_n7, assign44120_e56963_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 != 0.0)) {
        let assign44120_e56957: f64 = (var_inv_xi * var_inv_xi);
        let assign44120_e56959: f64 = (assign44120_e56957 * 0.16666666666666666);
        let assign44120_e56961: f64 = (assign44120_e56959 * 0.7071067811865475);
        (assign44120_e56961, ((((var_inv_xi_dn5 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn6 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn7 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn8 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn5, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8,)
    }
};
        var_sp_s_temp1 = assign44120_e56963;
        var_sp_s_temp1_dn5 = assign44120_e56963_d_n5;
        var_sp_s_temp1_dn6 = assign44120_e56963_d_n6;
        var_sp_s_temp1_dn7 = assign44120_e56963_d_n7;
        var_sp_s_temp1_dn8 = assign44120_e56963_d_n8;

        let (assign44130_e56983, assign44130_e56983_d_n5, assign44130_e56983_d_n6, assign44130_e56983_d_n7, assign44130_e56983_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 != 0.0)) {
        let assign44130_e56969: f64 = (var_xg * var_inv_xi);
        let assign44130_e56974: f64 = (1.0 - var_delta_nd);
        let assign44130_e56975: f64 = (var_xg * assign44130_e56974);
        let assign44130_e56977: f64 = (assign44130_e56975 * var_gf);
        let assign44130_e56979: f64 = (assign44130_e56977 * var_sp_s_temp1);
        let assign44130_e56980: f64 = (1.0 + assign44130_e56979);
        let assign44130_e56981: f64 = (assign44130_e56969 * assign44130_e56980);
        (assign44130_e56981, ((((var_xg_dn5 * var_inv_xi) + (var_xg * var_inv_xi_dn5)) * assign44130_e56980) + (assign44130_e56969 * ((((((var_xg_dn5 * assign44130_e56974) + (var_xg * (-var_delta_nd_dn5))) * var_gf) + (assign44130_e56975 * var_gf_dn5)) * var_sp_s_temp1) + (assign44130_e56977 * var_sp_s_temp1_dn5)))), ((((var_xg_dn6 * var_inv_xi) + (var_xg * var_inv_xi_dn6)) * assign44130_e56980) + (assign44130_e56969 * ((((((var_xg_dn6 * assign44130_e56974) + (var_xg * (-var_delta_nd_dn6))) * var_gf) + (assign44130_e56975 * var_gf_dn6)) * var_sp_s_temp1) + (assign44130_e56977 * var_sp_s_temp1_dn6)))), ((((var_xg_dn7 * var_inv_xi) + (var_xg * var_inv_xi_dn7)) * assign44130_e56980) + (assign44130_e56969 * ((((((var_xg_dn7 * assign44130_e56974) + (var_xg * (-var_delta_nd_dn7))) * var_gf) + (assign44130_e56975 * var_gf_dn7)) * var_sp_s_temp1) + (assign44130_e56977 * var_sp_s_temp1_dn7)))), ((((var_xg_dn8 * var_inv_xi) + (var_xg * var_inv_xi_dn8)) * assign44130_e56980) + (assign44130_e56969 * ((((((var_xg_dn8 * assign44130_e56974) + (var_xg * (-var_delta_nd_dn8))) * var_gf) + (assign44130_e56975 * var_gf_dn8)) * var_sp_s_temp1) + (assign44130_e56977 * var_sp_s_temp1_dn8)))),)
    } else {
        (var_x_d, var_x_d_dn5, var_x_d_dn6, var_x_d_dn7, var_x_d_dn8,)
    }
};
        var_x_d = assign44130_e56983;
        var_x_d_dn5 = assign44130_e56983_d_n5;
        var_x_d_dn6 = assign44130_e56983_d_n6;
        var_x_d_dn7 = assign44130_e56983_d_n7;
        var_x_d_dn8 = assign44130_e56983_d_n8;

        let (assign44140_e56992, assign44140_e56992_d_n5, assign44140_e56992_d_n6, assign44140_e56992_d_n7, assign44140_e56992_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44140_e56990: f64 = (var_xn_d + 3.0);
        (assign44140_e56990, var_xn_d_dn5, var_xn_d_dn6, var_xn_d_dn7, var_xn_d_dn8,)
    } else {
        (var_sp_s_bx, var_sp_s_bx_dn5, var_sp_s_bx_dn6, var_sp_s_bx_dn7, var_sp_s_bx_dn8,)
    }
};
        var_sp_s_bx = assign44140_e56992;
        var_sp_s_bx_dn5 = assign44140_e56992_d_n5;
        var_sp_s_bx_dn6 = assign44140_e56992_d_n6;
        var_sp_s_bx_dn7 = assign44140_e56992_d_n7;
        var_sp_s_bx_dn8 = assign44140_e56992_d_n8;

        let (assign44150_e57025, assign44150_e57025_d_n5, assign44150_e57025_d_n6, assign44150_e57025_d_n7, assign44150_e57025_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44150_e57000: f64 = (var_sp_s_x1 + var_sp_s_bx);
        let assign44150_e57003: f64 = (var_sp_s_x1 - var_sp_s_bx);
        let assign44150_e57006: f64 = (var_sp_s_x1 - var_sp_s_bx);
        let assign44150_e57007: f64 = (assign44150_e57003 * assign44150_e57006);
        let assign44150_e57009: f64 = (assign44150_e57007 + 5.0);
        let assign44150_e57010: f64 = (assign44150_e57009).sqrt();
        let assign44150_e57011: f64 = (assign44150_e57000 - assign44150_e57010);
        let assign44150_e57012: f64 = (0.5 * assign44150_e57011);
        let assign44150_e57017: f64 = (var_sp_s_bx * var_sp_s_bx);
        let assign44150_e57019: f64 = (assign44150_e57017 + 5.0);
        let assign44150_e57020: f64 = (assign44150_e57019).sqrt();
        let assign44150_e57021: f64 = (var_sp_s_bx - assign44150_e57020);
        let assign44150_e57022: f64 = (0.5 * assign44150_e57021);
        let assign44150_e57023: f64 = (assign44150_e57012 - assign44150_e57022);
        (assign44150_e57023, ((0.5 * ((var_sp_s_x1_dn5 + var_sp_s_bx_dn5) - ((((var_sp_s_x1_dn5 - var_sp_s_bx_dn5) * assign44150_e57006) + (assign44150_e57003 * (var_sp_s_x1_dn5 - var_sp_s_bx_dn5))) / (2.0 * assign44150_e57010)))) - (0.5 * (var_sp_s_bx_dn5 - (((var_sp_s_bx_dn5 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn5)) / (2.0 * assign44150_e57020))))), ((0.5 * ((var_sp_s_x1_dn6 + var_sp_s_bx_dn6) - ((((var_sp_s_x1_dn6 - var_sp_s_bx_dn6) * assign44150_e57006) + (assign44150_e57003 * (var_sp_s_x1_dn6 - var_sp_s_bx_dn6))) / (2.0 * assign44150_e57010)))) - (0.5 * (var_sp_s_bx_dn6 - (((var_sp_s_bx_dn6 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn6)) / (2.0 * assign44150_e57020))))), ((0.5 * ((var_sp_s_x1_dn7 + var_sp_s_bx_dn7) - ((((var_sp_s_x1_dn7 - var_sp_s_bx_dn7) * assign44150_e57006) + (assign44150_e57003 * (var_sp_s_x1_dn7 - var_sp_s_bx_dn7))) / (2.0 * assign44150_e57010)))) - (0.5 * (var_sp_s_bx_dn7 - (((var_sp_s_bx_dn7 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn7)) / (2.0 * assign44150_e57020))))), ((0.5 * ((var_sp_s_x1_dn8 + var_sp_s_bx_dn8) - ((((var_sp_s_x1_dn8 - var_sp_s_bx_dn8) * assign44150_e57006) + (assign44150_e57003 * (var_sp_s_x1_dn8 - var_sp_s_bx_dn8))) / (2.0 * assign44150_e57010)))) - (0.5 * (var_sp_s_bx_dn8 - (((var_sp_s_bx_dn8 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn8)) / (2.0 * assign44150_e57020))))),)
    } else {
        (var_sp_s_eta, var_sp_s_eta_dn5, var_sp_s_eta_dn6, var_sp_s_eta_dn7, var_sp_s_eta_dn8,)
    }
};
        var_sp_s_eta = assign44150_e57025;
        var_sp_s_eta_dn5 = assign44150_e57025_d_n5;
        var_sp_s_eta_dn6 = assign44150_e57025_d_n6;
        var_sp_s_eta_dn7 = assign44150_e57025_d_n7;
        var_sp_s_eta_dn8 = assign44150_e57025_d_n8;

        let (assign44160_e57034, assign44160_e57034_d_n5, assign44160_e57034_d_n6, assign44160_e57034_d_n7, assign44160_e57034_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44160_e57032: f64 = (var_xg - var_sp_s_eta);
        (assign44160_e57032, (var_xg_dn5 - var_sp_s_eta_dn5), (var_xg_dn6 - var_sp_s_eta_dn6), (var_xg_dn7 - var_sp_s_eta_dn7), (var_xg_dn8 - var_sp_s_eta_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign44160_e57034;
        var_sp_s_temp_dn5 = assign44160_e57034_d_n5;
        var_sp_s_temp_dn6 = assign44160_e57034_d_n6;
        var_sp_s_temp_dn7 = assign44160_e57034_d_n7;
        var_sp_s_temp_dn8 = assign44160_e57034_d_n8;

        let (assign44170_e57043, assign44170_e57043_d_n5, assign44170_e57043_d_n6, assign44170_e57043_d_n7, assign44170_e57043_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44170_e57040: f64 = (-var_sp_s_eta);
        let assign44170_e57041: f64 = (assign44170_e57040).exp();
        (assign44170_e57041, (assign44170_e57041 * (-var_sp_s_eta_dn5)), (assign44170_e57041 * (-var_sp_s_eta_dn6)), (assign44170_e57041 * (-var_sp_s_eta_dn7)), (assign44170_e57041 * (-var_sp_s_eta_dn8)),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn5, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8,)
    }
};
        var_sp_s_temp1 = assign44170_e57043;
        var_sp_s_temp1_dn5 = assign44170_e57043_d_n5;
        var_sp_s_temp1_dn6 = assign44170_e57043_d_n6;
        var_sp_s_temp1_dn7 = assign44170_e57043_d_n7;
        var_sp_s_temp1_dn8 = assign44170_e57043_d_n8;

        let (assign44180_e57056, assign44180_e57056_d_n5, assign44180_e57056_d_n6, assign44180_e57056_d_n7, assign44180_e57056_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44180_e57052: f64 = (var_sp_s_eta * var_sp_s_eta);
        let assign44180_e57053: f64 = (2.0 + assign44180_e57052);
        let assign44180_e57054: f64 = (1.0 / assign44180_e57053);
        (assign44180_e57054, (-(((var_sp_s_eta_dn5 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn5)) / (assign44180_e57053 * assign44180_e57053))), (-(((var_sp_s_eta_dn6 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn6)) / (assign44180_e57053 * assign44180_e57053))), (-(((var_sp_s_eta_dn7 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn7)) / (assign44180_e57053 * assign44180_e57053))), (-(((var_sp_s_eta_dn8 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn8)) / (assign44180_e57053 * assign44180_e57053))),)
    } else {
        (var_sp_s_temp2, var_sp_s_temp2_dn5, var_sp_s_temp2_dn6, var_sp_s_temp2_dn7, var_sp_s_temp2_dn8,)
    }
};
        var_sp_s_temp2 = assign44180_e57056;
        var_sp_s_temp2_dn5 = assign44180_e57056_d_n5;
        var_sp_s_temp2_dn6 = assign44180_e57056_d_n6;
        var_sp_s_temp2_dn7 = assign44180_e57056_d_n7;
        var_sp_s_temp2_dn8 = assign44180_e57056_d_n8;

        let (assign44190_e57067, assign44190_e57067_d_n5, assign44190_e57067_d_n6, assign44190_e57067_d_n7, assign44190_e57067_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44190_e57063: f64 = (var_sp_s_eta * var_sp_s_eta);
        let assign44190_e57065: f64 = (assign44190_e57063 * var_sp_s_temp2);
        (assign44190_e57065, ((((var_sp_s_eta_dn5 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn5)) * var_sp_s_temp2) + (assign44190_e57063 * var_sp_s_temp2_dn5)), ((((var_sp_s_eta_dn6 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn6)) * var_sp_s_temp2) + (assign44190_e57063 * var_sp_s_temp2_dn6)), ((((var_sp_s_eta_dn7 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn7)) * var_sp_s_temp2) + (assign44190_e57063 * var_sp_s_temp2_dn7)), ((((var_sp_s_eta_dn8 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn8)) * var_sp_s_temp2) + (assign44190_e57063 * var_sp_s_temp2_dn8)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn5, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8,)
    }
};
        var_sp_s_xi0 = assign44190_e57067;
        var_sp_s_xi0_dn5 = assign44190_e57067_d_n5;
        var_sp_s_xi0_dn6 = assign44190_e57067_d_n6;
        var_sp_s_xi0_dn7 = assign44190_e57067_d_n7;
        var_sp_s_xi0_dn8 = assign44190_e57067_d_n8;

        let (assign44200_e57080, assign44200_e57080_d_n5, assign44200_e57080_d_n6, assign44200_e57080_d_n7, assign44200_e57080_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44200_e57075: f64 = (var_sp_s_eta * var_sp_s_temp2);
        let assign44200_e57077: f64 = (assign44200_e57075 * var_sp_s_temp2);
        let assign44200_e57078: f64 = (4.0 * assign44200_e57077);
        (assign44200_e57078, (4.0 * ((((var_sp_s_eta_dn5 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn5)) * var_sp_s_temp2) + (assign44200_e57075 * var_sp_s_temp2_dn5))), (4.0 * ((((var_sp_s_eta_dn6 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn6)) * var_sp_s_temp2) + (assign44200_e57075 * var_sp_s_temp2_dn6))), (4.0 * ((((var_sp_s_eta_dn7 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn7)) * var_sp_s_temp2) + (assign44200_e57075 * var_sp_s_temp2_dn7))), (4.0 * ((((var_sp_s_eta_dn8 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn8)) * var_sp_s_temp2) + (assign44200_e57075 * var_sp_s_temp2_dn8))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn5, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8,)
    }
};
        var_sp_s_xi1 = assign44200_e57080;
        var_sp_s_xi1_dn5 = assign44200_e57080_d_n5;
        var_sp_s_xi1_dn6 = assign44200_e57080_d_n6;
        var_sp_s_xi1_dn7 = assign44200_e57080_d_n7;
        var_sp_s_xi1_dn8 = assign44200_e57080_d_n8;

        let (assign44210_e57097, assign44210_e57097_d_n5, assign44210_e57097_d_n6, assign44210_e57097_d_n7, assign44210_e57097_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44210_e57087: f64 = (8.0 * var_sp_s_temp2);
        let assign44210_e57090: f64 = (12.0 * var_sp_s_xi0);
        let assign44210_e57091: f64 = (assign44210_e57087 - assign44210_e57090);
        let assign44210_e57093: f64 = (assign44210_e57091 * var_sp_s_temp2);
        let assign44210_e57095: f64 = (assign44210_e57093 * var_sp_s_temp2);
        (assign44210_e57095, ((((((8.0 * var_sp_s_temp2_dn5) - (12.0 * var_sp_s_xi0_dn5)) * var_sp_s_temp2) + (assign44210_e57091 * var_sp_s_temp2_dn5)) * var_sp_s_temp2) + (assign44210_e57093 * var_sp_s_temp2_dn5)), ((((((8.0 * var_sp_s_temp2_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp2) + (assign44210_e57091 * var_sp_s_temp2_dn6)) * var_sp_s_temp2) + (assign44210_e57093 * var_sp_s_temp2_dn6)), ((((((8.0 * var_sp_s_temp2_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp2) + (assign44210_e57091 * var_sp_s_temp2_dn7)) * var_sp_s_temp2) + (assign44210_e57093 * var_sp_s_temp2_dn7)), ((((((8.0 * var_sp_s_temp2_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp2) + (assign44210_e57091 * var_sp_s_temp2_dn8)) * var_sp_s_temp2) + (assign44210_e57093 * var_sp_s_temp2_dn8)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn5, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8,)
    }
};
        var_sp_s_xi2 = assign44210_e57097;
        var_sp_s_xi2_dn5 = assign44210_e57097_d_n5;
        var_sp_s_xi2_dn6 = assign44210_e57097_d_n6;
        var_sp_s_xi2_dn7 = assign44210_e57097_d_n7;
        var_sp_s_xi2_dn8 = assign44210_e57097_d_n8;

        let (assign44220_e57145, assign44220_e57145_d_n5, assign44220_e57145_d_n6, assign44220_e57145_d_n7, assign44220_e57145_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44220_e57105: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign44220_e57109: f64 = (var_sp_s_temp1 + var_sp_s_eta);
        let assign44220_e57111: f64 = (assign44220_e57109 - 1.0);
        let assign44220_e57115: f64 = (var_sp_s_eta + 1.0);
        let assign44220_e57117: f64 = (assign44220_e57115 + var_sp_s_xi0);
        let assign44220_e57118: f64 = (var_delta_nd * assign44220_e57117);
        let assign44220_e57119: f64 = (assign44220_e57111 - assign44220_e57118);
        let assign44220_e57120: f64 = (var_gf2 * assign44220_e57119);
        let assign44220_e57121: f64 = (assign44220_e57105 - assign44220_e57120);
        let (assign44220_e57143, assign44220_e57143_d_n5, assign44220_e57143_d_n6, assign44220_e57143_d_n7, assign44220_e57143_d_n8,) = {
            if (1e-40 > assign44220_e57121) {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign44220_e57126: f64 = (var_sp_s_temp * var_sp_s_temp);
                let assign44220_e57130: f64 = (var_sp_s_temp1 + var_sp_s_eta);
                let assign44220_e57132: f64 = (assign44220_e57130 - 1.0);
                let assign44220_e57136: f64 = (var_sp_s_eta + 1.0);
                let assign44220_e57138: f64 = (assign44220_e57136 + var_sp_s_xi0);
                let assign44220_e57139: f64 = (var_delta_nd * assign44220_e57138);
                let assign44220_e57140: f64 = (assign44220_e57132 - assign44220_e57139);
                let assign44220_e57141: f64 = (var_gf2 * assign44220_e57140);
                let assign44220_e57142: f64 = (assign44220_e57126 - assign44220_e57141);
                (assign44220_e57142, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) - ((var_gf2_dn5 * assign44220_e57140) + (var_gf2 * ((var_sp_s_temp1_dn5 + var_sp_s_eta_dn5) - ((var_delta_nd_dn5 * assign44220_e57138) + (var_delta_nd * (var_sp_s_eta_dn5 + var_sp_s_xi0_dn5))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign44220_e57140) + (var_gf2 * ((var_sp_s_temp1_dn6 + var_sp_s_eta_dn6) - ((var_delta_nd_dn6 * assign44220_e57138) + (var_delta_nd * (var_sp_s_eta_dn6 + var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign44220_e57140) + (var_gf2 * ((var_sp_s_temp1_dn7 + var_sp_s_eta_dn7) - ((var_delta_nd_dn7 * assign44220_e57138) + (var_delta_nd * (var_sp_s_eta_dn7 + var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign44220_e57140) + (var_gf2 * ((var_sp_s_temp1_dn8 + var_sp_s_eta_dn8) - ((var_delta_nd_dn8 * assign44220_e57138) + (var_delta_nd * (var_sp_s_eta_dn8 + var_sp_s_xi0_dn8))))))),)
            }
        };
        (assign44220_e57143, assign44220_e57143_d_n5, assign44220_e57143_d_n6, assign44220_e57143_d_n7, assign44220_e57143_d_n8,)
    } else {
        (var_sp_s_a, var_sp_s_a_dn5, var_sp_s_a_dn6, var_sp_s_a_dn7, var_sp_s_a_dn8,)
    }
};
        var_sp_s_a = assign44220_e57145;
        var_sp_s_a_dn5 = assign44220_e57145_d_n5;
        var_sp_s_a_dn6 = assign44220_e57145_d_n6;
        var_sp_s_a_dn7 = assign44220_e57145_d_n7;
        var_sp_s_a_dn8 = assign44220_e57145_d_n8;

        let (assign44230_e57162, assign44230_e57162_d_n5, assign44230_e57162_d_n6, assign44230_e57162_d_n7, assign44230_e57162_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44230_e57156: f64 = (var_delta_nd * var_sp_s_xi2);
        let assign44230_e57157: f64 = (var_sp_s_temp1 - assign44230_e57156);
        let assign44230_e57158: f64 = (var_gf2 * assign44230_e57157);
        let assign44230_e57159: f64 = (0.5 * assign44230_e57158);
        let assign44230_e57160: f64 = (1.0 - assign44230_e57159);
        (assign44230_e57160, (-(0.5 * ((var_gf2_dn5 * assign44230_e57157) + (var_gf2 * (var_sp_s_temp1_dn5 - ((var_delta_nd_dn5 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn5))))))), (-(0.5 * ((var_gf2_dn6 * assign44230_e57157) + (var_gf2 * (var_sp_s_temp1_dn6 - ((var_delta_nd_dn6 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn6))))))), (-(0.5 * ((var_gf2_dn7 * assign44230_e57157) + (var_gf2 * (var_sp_s_temp1_dn7 - ((var_delta_nd_dn7 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn7))))))), (-(0.5 * ((var_gf2_dn8 * assign44230_e57157) + (var_gf2 * (var_sp_s_temp1_dn8 - ((var_delta_nd_dn8 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn8))))))),)
    } else {
        (var_sp_s_b, var_sp_s_b_dn5, var_sp_s_b_dn6, var_sp_s_b_dn7, var_sp_s_b_dn8,)
    }
};
        var_sp_s_b = assign44230_e57162;
        var_sp_s_b_dn5 = assign44230_e57162_d_n5;
        var_sp_s_b_dn6 = assign44230_e57162_d_n6;
        var_sp_s_b_dn7 = assign44230_e57162_d_n7;
        var_sp_s_b_dn8 = assign44230_e57162_d_n8;

        let (assign44240_e57183, assign44240_e57183_d_n5, assign44240_e57183_d_n6, assign44240_e57183_d_n7, assign44240_e57183_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44240_e57169: f64 = (2.0 * var_sp_s_temp);
        let assign44240_e57173: f64 = (1.0 - var_sp_s_temp1);
        let assign44240_e57177: f64 = (1.0 + var_sp_s_xi1);
        let assign44240_e57178: f64 = (var_delta_nd * assign44240_e57177);
        let assign44240_e57179: f64 = (assign44240_e57173 - assign44240_e57178);
        let assign44240_e57180: f64 = (var_gf2 * assign44240_e57179);
        let assign44240_e57181: f64 = (assign44240_e57169 + assign44240_e57180);
        (assign44240_e57181, ((2.0 * var_sp_s_temp_dn5) + ((var_gf2_dn5 * assign44240_e57179) + (var_gf2 * ((-var_sp_s_temp1_dn5) - ((var_delta_nd_dn5 * assign44240_e57177) + (var_delta_nd * var_sp_s_xi1_dn5)))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign44240_e57179) + (var_gf2 * ((-var_sp_s_temp1_dn6) - ((var_delta_nd_dn6 * assign44240_e57177) + (var_delta_nd * var_sp_s_xi1_dn6)))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign44240_e57179) + (var_gf2 * ((-var_sp_s_temp1_dn7) - ((var_delta_nd_dn7 * assign44240_e57177) + (var_delta_nd * var_sp_s_xi1_dn7)))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign44240_e57179) + (var_gf2 * ((-var_sp_s_temp1_dn8) - ((var_delta_nd_dn8 * assign44240_e57177) + (var_delta_nd * var_sp_s_xi1_dn8)))))),)
    } else {
        (var_sp_s_c, var_sp_s_c_dn5, var_sp_s_c_dn6, var_sp_s_c_dn7, var_sp_s_c_dn8,)
    }
};
        var_sp_s_c = assign44240_e57183;
        var_sp_s_c_dn5 = assign44240_e57183_d_n5;
        var_sp_s_c_dn6 = assign44240_e57183_d_n6;
        var_sp_s_c_dn7 = assign44240_e57183_d_n7;
        var_sp_s_c_dn8 = assign44240_e57183_d_n8;

        let (assign44250_e57197, assign44250_e57197_d_n5, assign44250_e57197_d_n6, assign44250_e57197_d_n7, assign44250_e57197_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44250_e57190: f64 = (var_xn_d - var_sp_s_eta);
        let assign44250_e57193: f64 = (var_sp_s_a / var_gf2);
        let assign44250_e57194: f64 = (assign44250_e57193).ln();
        let assign44250_e57195: f64 = (assign44250_e57190 + assign44250_e57194);
        (assign44250_e57195, ((var_xn_d_dn5 - var_sp_s_eta_dn5) + ((((var_sp_s_a_dn5 * var_gf2) - (var_sp_s_a * var_gf2_dn5)) / (var_gf2 * var_gf2)) / assign44250_e57193)), ((var_xn_d_dn6 - var_sp_s_eta_dn6) + ((((var_sp_s_a_dn6 * var_gf2) - (var_sp_s_a * var_gf2_dn6)) / (var_gf2 * var_gf2)) / assign44250_e57193)), ((var_xn_d_dn7 - var_sp_s_eta_dn7) + ((((var_sp_s_a_dn7 * var_gf2) - (var_sp_s_a * var_gf2_dn7)) / (var_gf2 * var_gf2)) / assign44250_e57193)), ((var_xn_d_dn8 - var_sp_s_eta_dn8) + ((((var_sp_s_a_dn8 * var_gf2) - (var_sp_s_a * var_gf2_dn8)) / (var_gf2 * var_gf2)) / assign44250_e57193)),)
    } else {
        (var_sp_s_tau, var_sp_s_tau_dn5, var_sp_s_tau_dn6, var_sp_s_tau_dn7, var_sp_s_tau_dn8,)
    }
};
        var_sp_s_tau = assign44250_e57197;
        var_sp_s_tau_dn5 = assign44250_e57197_d_n5;
        var_sp_s_tau_dn6 = assign44250_e57197_d_n6;
        var_sp_s_tau_dn7 = assign44250_e57197_d_n7;
        var_sp_s_tau_dn8 = assign44250_e57197_d_n8;

        let (assign44260_e57206, assign44260_e57206_d_n5, assign44260_e57206_d_n6, assign44260_e57206_d_n7, assign44260_e57206_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44260_e57204: f64 = (var_sp_s_a + var_sp_s_c);
        (assign44260_e57204, (var_sp_s_a_dn5 + var_sp_s_c_dn5), (var_sp_s_a_dn6 + var_sp_s_c_dn6), (var_sp_s_a_dn7 + var_sp_s_c_dn7), (var_sp_s_a_dn8 + var_sp_s_c_dn8),)
    } else {
        (var_nu, var_nu_dn5, var_nu_dn6, var_nu_dn7, var_nu_dn8,)
    }
};
        var_nu = assign44260_e57206;
        var_nu_dn5 = assign44260_e57206_d_n5;
        var_nu_dn6 = assign44260_e57206_d_n6;
        var_nu_dn7 = assign44260_e57206_d_n7;
        var_nu_dn8 = assign44260_e57206_d_n8;

        let (assign44270_e57227, assign44270_e57227_d_n5, assign44270_e57227_d_n6, assign44270_e57227_d_n7, assign44270_e57227_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44270_e57213: f64 = (var_nu * var_nu);
        let assign44270_e57218: f64 = (var_sp_s_c * var_sp_s_c);
        let assign44270_e57219: f64 = (0.5 * assign44270_e57218);
        let assign44270_e57222: f64 = (var_sp_s_a * var_sp_s_b);
        let assign44270_e57223: f64 = (assign44270_e57219 - assign44270_e57222);
        let assign44270_e57224: f64 = (var_sp_s_tau * assign44270_e57223);
        let assign44270_e57225: f64 = (assign44270_e57213 + assign44270_e57224);
        (assign44270_e57225, (((var_nu_dn5 * var_nu) + (var_nu * var_nu_dn5)) + ((var_sp_s_tau_dn5 * assign44270_e57223) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5))) - ((var_sp_s_a_dn5 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn5)))))), (((var_nu_dn6 * var_nu) + (var_nu * var_nu_dn6)) + ((var_sp_s_tau_dn6 * assign44270_e57223) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6))) - ((var_sp_s_a_dn6 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn6)))))), (((var_nu_dn7 * var_nu) + (var_nu * var_nu_dn7)) + ((var_sp_s_tau_dn7 * assign44270_e57223) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7))) - ((var_sp_s_a_dn7 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn7)))))), (((var_nu_dn8 * var_nu) + (var_nu * var_nu_dn8)) + ((var_sp_s_tau_dn8 * assign44270_e57223) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8))) - ((var_sp_s_a_dn8 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn8)))))),)
    } else {
        (var_mutau, var_mutau_dn5, var_mutau_dn6, var_mutau_dn7, var_mutau_dn8,)
    }
};
        var_mutau = assign44270_e57227;
        var_mutau_dn5 = assign44270_e57227_d_n5;
        var_mutau_dn6 = assign44270_e57227_d_n6;
        var_mutau_dn7 = assign44270_e57227_d_n7;
        var_mutau_dn8 = assign44270_e57227_d_n8;

        let (assign44280_e57262, assign44280_e57262_d_n5, assign44280_e57262_d_n6, assign44280_e57262_d_n7, assign44280_e57262_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44280_e57235: f64 = (var_sp_s_a * var_nu);
        let assign44280_e57237: f64 = (assign44280_e57235 * var_sp_s_tau);
        let assign44280_e57241: f64 = (var_nu / var_mutau);
        let assign44280_e57243: f64 = (assign44280_e57241 * var_sp_s_tau);
        let assign44280_e57245: f64 = (assign44280_e57243 * var_sp_s_tau);
        let assign44280_e57247: f64 = (assign44280_e57245 * var_sp_s_c);
        let assign44280_e57250: f64 = (var_sp_s_c * var_sp_s_c);
        let assign44280_e57252: f64 = (assign44280_e57250 * 0.3333333333333333);
        let assign44280_e57255: f64 = (var_sp_s_a * var_sp_s_b);
        let assign44280_e57256: f64 = (assign44280_e57252 - assign44280_e57255);
        let assign44280_e57257: f64 = (assign44280_e57247 * assign44280_e57256);
        let assign44280_e57258: f64 = (var_mutau + assign44280_e57257);
        let assign44280_e57259: f64 = (assign44280_e57237 / assign44280_e57258);
        let assign44280_e57260: f64 = (var_sp_s_eta + assign44280_e57259);
        (assign44280_e57260, (var_sp_s_eta_dn5 + (((((((var_sp_s_a_dn5 * var_nu) + (var_sp_s_a * var_nu_dn5)) * var_sp_s_tau) + (assign44280_e57235 * var_sp_s_tau_dn5)) * assign44280_e57258) - (assign44280_e57237 * (var_mutau_dn5 + (((((((((((var_nu_dn5 * var_mutau) - (var_nu * var_mutau_dn5)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign44280_e57241 * var_sp_s_tau_dn5)) * var_sp_s_tau) + (assign44280_e57243 * var_sp_s_tau_dn5)) * var_sp_s_c) + (assign44280_e57245 * var_sp_s_c_dn5)) * assign44280_e57256) + (assign44280_e57247 * ((((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5)) * 0.3333333333333333) - ((var_sp_s_a_dn5 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn5)))))))) / (assign44280_e57258 * assign44280_e57258))), (var_sp_s_eta_dn6 + (((((((var_sp_s_a_dn6 * var_nu) + (var_sp_s_a * var_nu_dn6)) * var_sp_s_tau) + (assign44280_e57235 * var_sp_s_tau_dn6)) * assign44280_e57258) - (assign44280_e57237 * (var_mutau_dn6 + (((((((((((var_nu_dn6 * var_mutau) - (var_nu * var_mutau_dn6)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign44280_e57241 * var_sp_s_tau_dn6)) * var_sp_s_tau) + (assign44280_e57243 * var_sp_s_tau_dn6)) * var_sp_s_c) + (assign44280_e57245 * var_sp_s_c_dn6)) * assign44280_e57256) + (assign44280_e57247 * ((((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6)) * 0.3333333333333333) - ((var_sp_s_a_dn6 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn6)))))))) / (assign44280_e57258 * assign44280_e57258))), (var_sp_s_eta_dn7 + (((((((var_sp_s_a_dn7 * var_nu) + (var_sp_s_a * var_nu_dn7)) * var_sp_s_tau) + (assign44280_e57235 * var_sp_s_tau_dn7)) * assign44280_e57258) - (assign44280_e57237 * (var_mutau_dn7 + (((((((((((var_nu_dn7 * var_mutau) - (var_nu * var_mutau_dn7)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign44280_e57241 * var_sp_s_tau_dn7)) * var_sp_s_tau) + (assign44280_e57243 * var_sp_s_tau_dn7)) * var_sp_s_c) + (assign44280_e57245 * var_sp_s_c_dn7)) * assign44280_e57256) + (assign44280_e57247 * ((((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7)) * 0.3333333333333333) - ((var_sp_s_a_dn7 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn7)))))))) / (assign44280_e57258 * assign44280_e57258))), (var_sp_s_eta_dn8 + (((((((var_sp_s_a_dn8 * var_nu) + (var_sp_s_a * var_nu_dn8)) * var_sp_s_tau) + (assign44280_e57235 * var_sp_s_tau_dn8)) * assign44280_e57258) - (assign44280_e57237 * (var_mutau_dn8 + (((((((((((var_nu_dn8 * var_mutau) - (var_nu * var_mutau_dn8)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign44280_e57241 * var_sp_s_tau_dn8)) * var_sp_s_tau) + (assign44280_e57243 * var_sp_s_tau_dn8)) * var_sp_s_c) + (assign44280_e57245 * var_sp_s_c_dn8)) * assign44280_e57256) + (assign44280_e57247 * ((((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8)) * 0.3333333333333333) - ((var_sp_s_a_dn8 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn8)))))))) / (assign44280_e57258 * assign44280_e57258))),)
    } else {
        (var_sp_s_x0, var_sp_s_x0_dn5, var_sp_s_x0_dn6, var_sp_s_x0_dn7, var_sp_s_x0_dn8,)
    }
};
        var_sp_s_x0 = assign44280_e57262;
        var_sp_s_x0_dn5 = assign44280_e57262_d_n5;
        var_sp_s_x0_dn6 = assign44280_e57262_d_n6;
        var_sp_s_x0_dn7 = assign44280_e57262_d_n7;
        var_sp_s_x0_dn8 = assign44280_e57262_d_n8;

        let assign44290_e57265: f64 = if var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1206 = assign44290_e57265;

        let (assign44300_e57275, assign44300_e57275_d_n5, assign44300_e57275_d_n6, assign44300_e57275_d_n7, assign44300_e57275_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 != 0.0)) {
        let assign44300_e57273: f64 = (var_sp_s_x0).exp();
        (assign44300_e57273, (assign44300_e57273 * var_sp_s_x0_dn5), (assign44300_e57273 * var_sp_s_x0_dn6), (assign44300_e57273 * var_sp_s_x0_dn7), (assign44300_e57273 * var_sp_s_x0_dn8),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign44300_e57275;
        var_sp_s_delta0_dn5 = assign44300_e57275_d_n5;
        var_sp_s_delta0_dn6 = assign44300_e57275_d_n6;
        var_sp_s_delta0_dn7 = assign44300_e57275_d_n7;
        var_sp_s_delta0_dn8 = assign44300_e57275_d_n8;

        let (assign44310_e57286, assign44310_e57286_d_n5, assign44310_e57286_d_n6, assign44310_e57286_d_n7, assign44310_e57286_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 != 0.0)) {
        let assign44310_e57284: f64 = (1.0 / var_sp_s_delta0);
        (assign44310_e57284, (-(var_sp_s_delta0_dn5 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn6 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn7 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn8 / (var_sp_s_delta0 * var_sp_s_delta0))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign44310_e57286;
        var_sp_s_delta1_dn5 = assign44310_e57286_d_n5;
        var_sp_s_delta1_dn6 = assign44310_e57286_d_n6;
        var_sp_s_delta1_dn7 = assign44310_e57286_d_n7;
        var_sp_s_delta1_dn8 = assign44310_e57286_d_n8;

        let (assign44320_e57297, assign44320_e57297_d_n5, assign44320_e57297_d_n6, assign44320_e57297_d_n7, assign44320_e57297_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 != 0.0)) {
        let assign44320_e57295: f64 = (var_delta_nd * var_sp_s_delta0);
        (assign44320_e57295, ((var_delta_nd_dn5 * var_sp_s_delta0) + (var_delta_nd * var_sp_s_delta0_dn5)), ((var_delta_nd_dn6 * var_sp_s_delta0) + (var_delta_nd * var_sp_s_delta0_dn6)), ((var_delta_nd_dn7 * var_sp_s_delta0) + (var_delta_nd * var_sp_s_delta0_dn7)), ((var_delta_nd_dn8 * var_sp_s_delta0) + (var_delta_nd * var_sp_s_delta0_dn8)),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign44320_e57297;
        var_sp_s_delta0_dn5 = assign44320_e57297_d_n5;
        var_sp_s_delta0_dn6 = assign44320_e57297_d_n6;
        var_sp_s_delta0_dn7 = assign44320_e57297_d_n7;
        var_sp_s_delta0_dn8 = assign44320_e57297_d_n8;

        let assign44330_e57301: f64 = (var_xn_d - 230.25850929940458);
        let assign44330_e57302: f64 = if var_sp_s_x0 > assign44330_e57301 { 1.0 } else { 0.0 };
        var_guard1207 = assign44330_e57302;

        let (assign44340_e57317, assign44340_e57317_d_n5, assign44340_e57317_d_n6, assign44340_e57317_d_n7, assign44340_e57317_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 == 0.0)) && (var_guard1207 != 0.0)) {
        let assign44340_e57314: f64 = (var_sp_s_x0 - var_xn_d);
        let assign44340_e57315: f64 = (assign44340_e57314).exp();
        (assign44340_e57315, (assign44340_e57315 * (var_sp_s_x0_dn5 - var_xn_d_dn5)), (assign44340_e57315 * (var_sp_s_x0_dn6 - var_xn_d_dn6)), (assign44340_e57315 * (var_sp_s_x0_dn7 - var_xn_d_dn7)), (assign44340_e57315 * (var_sp_s_x0_dn8 - var_xn_d_dn8)),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign44340_e57317;
        var_sp_s_delta0_dn5 = assign44340_e57317_d_n5;
        var_sp_s_delta0_dn6 = assign44340_e57317_d_n6;
        var_sp_s_delta0_dn7 = assign44340_e57317_d_n7;
        var_sp_s_delta0_dn8 = assign44340_e57317_d_n8;

        let (assign44350_e57331, assign44350_e57331_d_n5, assign44350_e57331_d_n6, assign44350_e57331_d_n7, assign44350_e57331_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 == 0.0)) && (var_guard1207 != 0.0)) {
        let assign44350_e57329: f64 = (var_delta_nd / var_sp_s_delta0);
        (assign44350_e57329, (((var_delta_nd_dn5 * var_sp_s_delta0) - (var_delta_nd * var_sp_s_delta0_dn5)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_nd_dn6 * var_sp_s_delta0) - (var_delta_nd * var_sp_s_delta0_dn6)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_nd_dn7 * var_sp_s_delta0) - (var_delta_nd * var_sp_s_delta0_dn7)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_nd_dn8 * var_sp_s_delta0) - (var_delta_nd * var_sp_s_delta0_dn8)) / (var_sp_s_delta0 * var_sp_s_delta0)),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign44350_e57331;
        var_sp_s_delta1_dn5 = assign44350_e57331_d_n5;
        var_sp_s_delta1_dn6 = assign44350_e57331_d_n6;
        var_sp_s_delta1_dn7 = assign44350_e57331_d_n7;
        var_sp_s_delta1_dn8 = assign44350_e57331_d_n8;

        *var_delta_nd_slot = var_delta_nd;
        *var_delta_nd_dn5_slot = var_delta_nd_dn5;
        *var_delta_nd_dn6_slot = var_delta_nd_dn6;
        *var_delta_nd_dn7_slot = var_delta_nd_dn7;
        *var_delta_nd_dn8_slot = var_delta_nd_dn8;
        *var_guard1204_slot = var_guard1204;
        *var_guard1205_slot = var_guard1205;
        *var_guard1206_slot = var_guard1206;
        *var_guard1207_slot = var_guard1207;
        *var_k_ds_slot = var_k_ds;
        *var_k_ds_dn5_slot = var_k_ds_dn5;
        *var_k_ds_dn6_slot = var_k_ds_dn6;
        *var_k_ds_dn7_slot = var_k_ds_dn7;
        *var_k_ds_dn8_slot = var_k_ds_dn8;
        *var_mutau_slot = var_mutau;
        *var_mutau_dn5_slot = var_mutau_dn5;
        *var_mutau_dn6_slot = var_mutau_dn6;
        *var_mutau_dn7_slot = var_mutau_dn7;
        *var_mutau_dn8_slot = var_mutau_dn8;
        *var_nu_slot = var_nu;
        *var_nu_dn5_slot = var_nu_dn5;
        *var_nu_dn6_slot = var_nu_dn6;
        *var_nu_dn7_slot = var_nu_dn7;
        *var_nu_dn8_slot = var_nu_dn8;
        *var_sp_s_a_slot = var_sp_s_a;
        *var_sp_s_a_dn5_slot = var_sp_s_a_dn5;
        *var_sp_s_a_dn6_slot = var_sp_s_a_dn6;
        *var_sp_s_a_dn7_slot = var_sp_s_a_dn7;
        *var_sp_s_a_dn8_slot = var_sp_s_a_dn8;
        *var_sp_s_b_slot = var_sp_s_b;
        *var_sp_s_b_dn5_slot = var_sp_s_b_dn5;
        *var_sp_s_b_dn6_slot = var_sp_s_b_dn6;
        *var_sp_s_b_dn7_slot = var_sp_s_b_dn7;
        *var_sp_s_b_dn8_slot = var_sp_s_b_dn8;
        *var_sp_s_bx_slot = var_sp_s_bx;
        *var_sp_s_bx_dn5_slot = var_sp_s_bx_dn5;
        *var_sp_s_bx_dn6_slot = var_sp_s_bx_dn6;
        *var_sp_s_bx_dn7_slot = var_sp_s_bx_dn7;
        *var_sp_s_bx_dn8_slot = var_sp_s_bx_dn8;
        *var_sp_s_c_slot = var_sp_s_c;
        *var_sp_s_c_dn5_slot = var_sp_s_c_dn5;
        *var_sp_s_c_dn6_slot = var_sp_s_c_dn6;
        *var_sp_s_c_dn7_slot = var_sp_s_c_dn7;
        *var_sp_s_c_dn8_slot = var_sp_s_c_dn8;
        *var_sp_s_delta0_slot = var_sp_s_delta0;
        *var_sp_s_delta0_dn5_slot = var_sp_s_delta0_dn5;
        *var_sp_s_delta0_dn6_slot = var_sp_s_delta0_dn6;
        *var_sp_s_delta0_dn7_slot = var_sp_s_delta0_dn7;
        *var_sp_s_delta0_dn8_slot = var_sp_s_delta0_dn8;
        *var_sp_s_delta1_slot = var_sp_s_delta1;
        *var_sp_s_delta1_dn5_slot = var_sp_s_delta1_dn5;
        *var_sp_s_delta1_dn6_slot = var_sp_s_delta1_dn6;
        *var_sp_s_delta1_dn7_slot = var_sp_s_delta1_dn7;
        *var_sp_s_delta1_dn8_slot = var_sp_s_delta1_dn8;
        *var_sp_s_eta_slot = var_sp_s_eta;
        *var_sp_s_eta_dn5_slot = var_sp_s_eta_dn5;
        *var_sp_s_eta_dn6_slot = var_sp_s_eta_dn6;
        *var_sp_s_eta_dn7_slot = var_sp_s_eta_dn7;
        *var_sp_s_eta_dn8_slot = var_sp_s_eta_dn8;
        *var_sp_s_tau_slot = var_sp_s_tau;
        *var_sp_s_tau_dn5_slot = var_sp_s_tau_dn5;
        *var_sp_s_tau_dn6_slot = var_sp_s_tau_dn6;
        *var_sp_s_tau_dn7_slot = var_sp_s_tau_dn7;
        *var_sp_s_tau_dn8_slot = var_sp_s_tau_dn8;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp1_slot = var_sp_s_temp1;
        *var_sp_s_temp1_dn5_slot = var_sp_s_temp1_dn5;
        *var_sp_s_temp1_dn6_slot = var_sp_s_temp1_dn6;
        *var_sp_s_temp1_dn7_slot = var_sp_s_temp1_dn7;
        *var_sp_s_temp1_dn8_slot = var_sp_s_temp1_dn8;
        *var_sp_s_temp2_slot = var_sp_s_temp2;
        *var_sp_s_temp2_dn5_slot = var_sp_s_temp2_dn5;
        *var_sp_s_temp2_dn6_slot = var_sp_s_temp2_dn6;
        *var_sp_s_temp2_dn7_slot = var_sp_s_temp2_dn7;
        *var_sp_s_temp2_dn8_slot = var_sp_s_temp2_dn8;
        *var_sp_s_temp_dn5_slot = var_sp_s_temp_dn5;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_x0_slot = var_sp_s_x0;
        *var_sp_s_x0_dn5_slot = var_sp_s_x0_dn5;
        *var_sp_s_x0_dn6_slot = var_sp_s_x0_dn6;
        *var_sp_s_x0_dn7_slot = var_sp_s_x0_dn7;
        *var_sp_s_x0_dn8_slot = var_sp_s_x0_dn8;
        *var_sp_s_xi0_slot = var_sp_s_xi0;
        *var_sp_s_xi0_dn5_slot = var_sp_s_xi0_dn5;
        *var_sp_s_xi0_dn6_slot = var_sp_s_xi0_dn6;
        *var_sp_s_xi0_dn7_slot = var_sp_s_xi0_dn7;
        *var_sp_s_xi0_dn8_slot = var_sp_s_xi0_dn8;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn5_slot = var_sp_s_xi1_dn5;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn5_slot = var_sp_s_xi2_dn5;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
        *var_udse_slot = var_udse;
        *var_udse_dn5_slot = var_udse_dn5;
        *var_udse_dn6_slot = var_udse_dn6;
        *var_udse_dn7_slot = var_udse_dn7;
        *var_udse_dn8_slot = var_udse_dn8;
        *var_vdse_slot = var_vdse;
        *var_vdse_dn5_slot = var_vdse_dn5;
        *var_vdse_dn6_slot = var_vdse_dn6;
        *var_vdse_dn7_slot = var_vdse_dn7;
        *var_vdse_dn8_slot = var_vdse_dn8;
        *var_x_d_slot = var_x_d;
        *var_x_d_dn5_slot = var_x_d_dn5;
        *var_x_d_dn6_slot = var_x_d_dn6;
        *var_x_d_dn7_slot = var_x_d_dn7;
        *var_x_d_dn8_slot = var_x_d_dn8;
        *var_xn_d_slot = var_xn_d;
        *var_xn_d_dn5_slot = var_xn_d_dn5;
        *var_xn_d_dn6_slot = var_xn_d_dn6;
        *var_xn_d_dn7_slot = var_xn_d_dn7;
        *var_xn_d_dn8_slot = var_xn_d_dn8;
    }
}
